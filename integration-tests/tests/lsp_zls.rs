//! Zig LSP integration test: build, start zls, initialize, did_open, hover.
//! Run with: cargo test -p integration-tests lsp_zls -- --ignored --nocapture

use integration_tests::{init_lsp_tracing, path_to_file_uri, with_temp_project_build};
use miette::Result;
use poly_bench_project::runtime_env;

#[test]
#[ignore = "requires network (ZLS download); run with: cargo test -p integration-tests lsp_zls -- --ignored --nocapture"]
fn lsp_zls() -> Result<()> {
    init_lsp_tracing("lsp_zls");
    with_temp_project_build(vec!["zig".to_string()], "lsp-test-zig", |project_path| {
        let workspace_root = runtime_env(project_path, poly_bench_dsl::Lang::Zig);
        let workspace_root_str = workspace_root.to_string_lossy();

        let client = runtimes_zig::zls_client::ZlsClient::new(&workspace_root_str)
            .map_err(|e| miette::miette!("ZlsClient::new failed: {}", e))?;

        client.initialize().map_err(|e| miette::miette!("initialize failed: {}", e))?;

        let test_file = workspace_root.join("src").join("test.zig");
        let content = "fn foo(x: i32) i32 { return x }\n";
        std::fs::write(&test_file, content)
            .map_err(|e| miette::miette!("Failed to write test.zig: {}", e))?;

        let uri = path_to_file_uri(&test_file);
        client.did_open(&uri, content, 1).map_err(|e| miette::miette!("did_open failed: {}", e))?;

        let hover_result = client.hover(&uri, 0, 3);
        hover_result.map_err(|e| miette::miette!("hover failed: {}", e))?;

        Ok(())
    })
}
