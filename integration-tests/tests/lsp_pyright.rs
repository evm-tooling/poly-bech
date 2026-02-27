//! Python LSP integration test: build, start pyright, initialize, did_open, hover.
//! Run with: cargo test -p integration-tests lsp_pyright -- --ignored --nocapture

use integration_tests::{init_lsp_tracing, path_to_file_uri, with_temp_project_build};
use miette::Result;
use poly_bench_project::runtime_env;

#[test]
#[ignore = "requires network (pip install); run with: cargo test -p integration-tests lsp_pyright -- --ignored --nocapture"]
fn lsp_pyright() -> Result<()> {
    init_lsp_tracing("lsp_pyright");
    with_temp_project_build(vec!["python".to_string()], "lsp-test-python", |project_path| {
        let workspace_root = runtime_env(project_path, poly_bench_dsl::Lang::Python);
        let workspace_root_str = workspace_root.to_string_lossy();

        let client = runtimes_python::pyright_client::PyrightClient::new(&workspace_root_str)
            .map_err(|e| miette::miette!("PyrightClient::new failed: {}", e))?;

        client.initialize().map_err(|e| miette::miette!("initialize failed: {}", e))?;

        let test_file = workspace_root.join("test.py");
        let content = "def foo(x: int) -> int:\n    return x\n";
        std::fs::write(&test_file, content)
            .map_err(|e| miette::miette!("Failed to write test.py: {}", e))?;

        let uri = path_to_file_uri(&test_file);
        client.did_open(&uri, content, 1).map_err(|e| miette::miette!("did_open failed: {}", e))?;

        let hover_result = client.hover(&uri, 0, 4);
        hover_result.map_err(|e| miette::miette!("hover failed: {}", e))?;

        Ok(())
    })
}
