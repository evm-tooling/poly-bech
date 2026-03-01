//! Integration tests for C, C#, and Zig runtimes with external dependencies.
//! Verifies that benchmarks using OpenSSL (C), Keccak256 NuGet (C#), and stdlib Keccak256 (Zig)
//! compile and run when dependencies are configured in polybench.toml.
//!
//! Run with: cargo test -p integration-tests external_deps -- --ignored
//! Requires: clang, pkg-config, openssl, dotnet, zig

use integration_tests::with_temp_project_init;
use miette::Result;
use poly_bench_project::{load_manifest, save_manifest};
use std::{fs, path::Path, process::Command};

fn add_external_deps_to_manifest(project_path: &Path) -> Result<()> {
    let mut manifest = load_manifest(project_path)?;
    if let Some(ref mut c) = manifest.c {
        c.dependencies.insert("openssl".to_string(), "3.2".to_string());
    }
    if let Some(ref mut csharp) = manifest.csharp {
        csharp.dependencies.insert("Keccak256".to_string(), "1.0.2".to_string());
    }
    save_manifest(project_path, &manifest)?;
    Ok(())
}

#[test]
#[ignore = "requires clang, pkg-config, openssl, dotnet, zig, and network for NuGet"]
fn keccak_bench_with_external_deps_compiles_and_runs() -> Result<()> {
    let languages = vec!["c".to_string(), "csharp".to_string(), "zig".to_string()];

    with_temp_project_init(languages, "external-deps-keccak", |project_path| {
        add_external_deps_to_manifest(project_path)?;

        let poly_bench =
            std::env::var("CARGO_BIN_EXE_poly-bench").unwrap_or_else(|_| "poly-bench".to_string());

        let build_output = Command::new(&poly_bench)
            .arg("build")
            .current_dir(project_path)
            .output()
            .expect("Failed to run poly-bench build");
        if !build_output.status.success() {
            eprintln!("Build stderr: {}", String::from_utf8_lossy(&build_output.stderr));
            eprintln!("Build stdout: {}", String::from_utf8_lossy(&build_output.stdout));
        }
        assert!(
            build_output.status.success(),
            "poly-bench build should succeed with external deps"
        );

        let benchmarks_dir = project_path.join("benchmarks");
        fs::create_dir_all(&benchmarks_dir)
            .map_err(|e| miette::miette!("Failed to create benchmarks dir: {}", e))?;

        let bench_content = r#"declare suite keccak256 performance timeBased sameDataset: true {
    description: "Keccak256 hash with external deps"
    baseline: "zig"
    warmup: 10
    targetTime: 50ms
    count: 2

    setup zig {
        import {
            const Keccak256 = std.crypto.hash.sha3.Keccak256;
        }
        helpers {
            fn keccak256Zig(data: []const u8) [32]u8 {
                var hasher = Keccak256.init(.{});
                hasher.update(data);
                var digest: [32]u8 = undefined;
                hasher.final(&digest);
                return digest;
            }
            fn keccak256ZigBench(data: []const u8) void {
                _ = keccak256Zig(data);
            }
        }
    }

    setup c {
        import {
            #include <openssl/evp.h>
            #include <openssl/sha.h>
        }
        helpers {
            static void keccak256_c(const unsigned char* data, size_t len, unsigned char* out) {
                EVP_MD_CTX* ctx = EVP_MD_CTX_new();
                if (!ctx) return;
                EVP_DigestInit_ex(ctx, EVP_sha3_256(), NULL);
                EVP_DigestUpdate(ctx, data, len);
                unsigned int outlen = 32;
                EVP_DigestFinal_ex(ctx, out, &outlen);
                EVP_MD_CTX_free(ctx);
            }
        }
    }

    setup csharp {
        import {
            using System;
            using Epoche;
        }
        helpers {
            static byte[] Keccak256Csharp(byte[] data) {
                return Keccak256.ComputeHash(data);
            }
        }
    }

    fixture shortData {
        hex: "68656c6c6f20776f726c64"
    }

    bench keccak256Short {
        zig: keccak256ZigBench(&shortData);
        c: {
            unsigned char __out[32];
            keccak256_c(shortData, sizeof(shortData), __out);
        }
        csharp: Keccak256Csharp(shortData);
    }
}
"#;

        let bench_path = benchmarks_dir.join("keccak.bench");
        fs::write(&bench_path, bench_content)
            .map_err(|e| miette::miette!("Failed to write bench file: {}", e))?;

        let run_output = Command::new(&poly_bench)
            .args(["run", bench_path.to_str().unwrap()])
            .current_dir(project_path)
            .output()
            .expect("Failed to run poly-bench run");

        let stdout = String::from_utf8_lossy(&run_output.stdout);
        let stderr = String::from_utf8_lossy(&run_output.stderr);

        if !run_output.status.success() {
            eprintln!("Run stdout: {}", stdout);
            eprintln!("Run stderr: {}", stderr);
        }

        assert!(run_output.status.success(), "poly-bench run should succeed: {}", stderr);

        assert!(
            stdout.contains("keccak256Short") ||
                stdout.contains("keccak256") ||
                stdout.contains("iterations"),
            "Output should contain benchmark results"
        );

        Ok(())
    })
}
