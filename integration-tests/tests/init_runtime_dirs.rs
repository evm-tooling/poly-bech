//! Init runtime dirs test: verify poly-bench init correctly sets up .polybench/runtime-env/ for each runtime.
//! No build step; init-only. Run with: cargo test -p integration-tests init_runtime_dirs

use integration_tests::with_temp_project_init;
use miette::Result;
use poly_bench_dsl::Lang;
use poly_bench_project::runtime_env;

#[test]
fn init_runtime_dirs() -> Result<()> {
    let languages = vec![
        "go".to_string(),
        "ts".to_string(),
        "rust".to_string(),
        "python".to_string(),
        "zig".to_string(),
    ];

    with_temp_project_init(languages, "pb-project", |project_path| {
        // Assert .polybench/runtime-env/{lang}/ exists for each lang
        for lang in [Lang::Go, Lang::TypeScript, Lang::Rust, Lang::Python, Lang::Zig] {
            let env_path = runtime_env(project_path, lang);
            assert!(
                env_path.exists(),
                ".polybench/runtime-env/{} should exist",
                lang.as_str()
            );
        }

        // Lang-specific markers
        assert!(
            runtime_env(project_path, Lang::Go).join("go.mod").exists(),
            "go/go.mod should exist"
        );
        assert!(
            runtime_env(project_path, Lang::TypeScript)
                .join("package.json")
                .exists(),
            "ts/package.json should exist"
        );
        assert!(
            runtime_env(project_path, Lang::Rust)
                .join("Cargo.toml")
                .exists(),
            "rust/Cargo.toml should exist"
        );
        assert!(
            runtime_env(project_path, Lang::Python)
                .join("requirements.txt")
                .exists(),
            "python/requirements.txt should exist"
        );
        assert!(
            runtime_env(project_path, Lang::Zig)
                .join("build.zig")
                .exists(),
            "zig/build.zig should exist"
        );

        Ok(())
    })
}
