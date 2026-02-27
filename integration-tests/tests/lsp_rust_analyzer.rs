//! Rust LSP integration test: build, start rust-analyzer, initialize, did_open, hover.
//! Run with: cargo test -p integration-tests lsp_rust_analyzer -- --ignored --nocapture

use integration_tests::{init_lsp_tracing, path_to_file_uri, with_temp_project_build};
use miette::Result;
use poly_bench_project::runtime_env;

#[test]
#[ignore = "requires network (cargo/rustup); run with: cargo test -p integration-tests lsp_rust_analyzer -- --ignored --nocapture"]
fn lsp_rust_analyzer() -> Result<()> {
    init_lsp_tracing("lsp_rust_analyzer");
    with_temp_project_build(vec!["rust".to_string()], "lsp-test-rust", |project_path| {
        let workspace_root = runtime_env(project_path, poly_bench_dsl::Lang::Rust);
        let workspace_root_str = workspace_root.to_string_lossy();

        let client = runtimes_rust::rust_analyzer_client::RustAnalyzerClient::new(&workspace_root_str)
            .map_err(|e| miette::miette!("RustAnalyzerClient::new failed: {}", e))?;

        client
            .initialize()
            .map_err(|e| miette::miette!("initialize failed: {}", e))?;

        let test_file = workspace_root.join("src").join("test.rs");
        let content = "fn foo(x: i32) -> i32 { x }\n";
        std::fs::write(&test_file, content)
            .map_err(|e| miette::miette!("Failed to write test.rs: {}", e))?;

        let uri = path_to_file_uri(&test_file);
        client
            .did_open(&uri, content, 1)
            .map_err(|e| miette::miette!("did_open failed: {}", e))?;

        let hover_result = client.hover(&uri, 0, 3);
        hover_result.map_err(|e| miette::miette!("hover failed: {}", e))?;

        Ok(())
    })
}
