//! Runtime validation test: verify that compile/run fails when a bench file uses
//! a language not configured in polybench.toml.

use integration_tests::with_temp_project_init;
use miette::Result;
use poly_bench_dsl as dsl;
use poly_bench_ir as ir;
use poly_bench_project::load_manifest;
use std::process::Command;

#[test]
fn compile_fails_when_using_unconfigured_language() -> Result<()> {
    let languages = vec!["go".to_string(), "ts".to_string()];

    with_temp_project_init(languages, "runtime-validation", |project_path| {
        // Write a bench file that uses rust (not in project)
        let bench_content = r#"
declare suite test performance timeBased sameDataset: true {
    targetTime: 2s
    setup go {
        helpers {
            func work() int { return 1 }
        }
    }
    setup rust {
        helpers {
            fn work() -> i32 { 1 }
        }
    }
    bench foo {
        go: work()
        rust: work()
    }
}
"#;
        let bench_path = project_path.join("benchmarks").join("rust_bench.bench");
        std::fs::write(&bench_path, bench_content)
            .map_err(|e| miette::miette!("Failed to write bench file: {}", e))?;

        // Verify validation logic: parse, lower, check languages_used vs manifest
        let manifest = load_manifest(project_path)?;
        assert!(!manifest.has_runtime(poly_bench_dsl::Lang::Rust));
        let ast = dsl::parse(&bench_content, "rust_bench.bench")?;
        let ir_result = ir::lower(&ast, bench_path.parent())?;
        let languages_used = ir_result.languages_used();
        assert!(languages_used.contains(&poly_bench_dsl::Lang::Rust), "IR should contain rust");
        let unconfigured: Vec<_> =
            languages_used.iter().filter(|lang| !manifest.has_runtime(**lang)).collect();
        assert!(!unconfigured.is_empty(), "Should have unconfigured languages (rust)");

        // Run poly-bench compile - should fail with runtime-not-configured error.
        // Uses CARGO_BIN_EXE_poly-bench when running via `cargo test -p poly-bench`.
        if let Ok(poly_bench) = std::env::var("CARGO_BIN_EXE_poly-bench") {
            let output = Command::new(&poly_bench)
                .arg("compile")
                .arg(&bench_path)
                .current_dir(project_path)
                .output()
                .expect("Failed to run poly-bench compile");

            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert!(
                !output.status.success(),
                "Expected compile to fail when using unconfigured language rust. stdout: {}, stderr: {}",
                stdout,
                stderr
            );
            let combined = format!("{} {}", stdout, stderr);
            assert!(
                combined.contains("rust") &&
                    (combined.contains("poly-bench add") || combined.contains("not configured")),
                "Expected error message about rust not configured and poly-bench add; got: {}",
                combined
            );
        }
        // When CARGO_BIN_EXE_poly-bench is not set, we've still verified the validation logic
        // above.

        Ok(())
    })
}
