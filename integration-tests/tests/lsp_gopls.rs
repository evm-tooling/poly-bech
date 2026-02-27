//! Go LSP integration test: uses build_project_at (real install_local_gopls from build.rs),
//! starts gopls, initialize, did_open, hover. Temp project in
//! integration-tests/.tmp-test-projects/.
//!
//! Run: cargo test -p integration-tests lsp_gopls -- --ignored --nocapture
//! (go install gopls may take 1-2 min; --nocapture shows progress and debug output)

use integration_tests::{init_lsp_tracing, path_to_file_uri, with_temp_project_build};
use miette::Result;
use poly_bench_project::runtime_env;

#[test]
#[ignore = "requires network (go install gopls via build_project_at); run with: cargo test -p integration-tests lsp_gopls -- --ignored --nocapture"]
fn lsp_gopls() -> Result<()> {
    init_lsp_tracing("lsp_gopls");
    with_temp_project_build(vec!["go".to_string()], "lsp-test-go", |project_path| {
        eprintln!("[lsp_gopls] project_path={}", project_path.display());

        let workspace_root = runtime_env(project_path, poly_bench_dsl::Lang::Go);
        let workspace_root_str = workspace_root.to_string_lossy();
        eprintln!("[lsp_gopls] workspace_root={}", workspace_root_str);

        eprintln!("[lsp_gopls] Creating GoplsClient...");
        let client = runtimes_go::gopls_client::GoplsClient::new(&workspace_root_str)
            .map_err(|e| miette::miette!("GoplsClient::new failed: {}", e))?;
        eprintln!("[lsp_gopls] GoplsClient created, calling initialize()...");

        client.initialize().map_err(|e| miette::miette!("initialize failed: {}", e))?;
        eprintln!("[lsp_gopls] initialize() OK, writing test.go...");

        // File must be inside workspace so gopls has package metadata (go.mod)
        let test_file = workspace_root.join("test.go");
        let content = "package main\n\nfunc foo(x int) int { return x }\n";
        std::fs::write(&test_file, content)
            .map_err(|e| miette::miette!("Failed to write test.go: {}", e))?;
        eprintln!("[lsp_gopls] test.go written at {}, calling did_open()...", test_file.display());

        let uri = path_to_file_uri(&test_file);
        client.did_open(&uri, content, 1).map_err(|e| miette::miette!("did_open failed: {}", e))?;
        eprintln!("[lsp_gopls] did_open() OK, calling hover()...");

        // Hover on "foo" at line 2, char 5 (0-indexed)
        let hover_result = client.hover(&uri, 2, 5);
        eprintln!("[lsp_gopls] hover() returned: {:?}", hover_result.as_ref().map(|_| "Ok"));
        hover_result.map_err(|e| miette::miette!("hover failed: {}", e))?;

        eprintln!("[lsp_gopls] test passed");
        Ok(())
    })
}
