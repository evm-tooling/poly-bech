//! C# LSP integration test: build, start Roslyn Language Server, initialize, did_open, hover.
//!
//! Uses Microsoft's official roslyn-language-server (dotnet/roslyn) installed via
//! `dotnet tool install` during build. Run with:
//!   cargo test -p integration-tests lsp_csharp -- --ignored --nocapture

use integration_tests::{init_lsp_tracing, path_to_file_uri, with_temp_project_build};
use miette::Result;
use poly_bench_project::runtime_env;
use runtimes_csharp::omnisharp_client::OmniSharpClient;

#[test]
#[ignore = "requires network (dotnet tool install) and .NET 8+; run with: cargo test -p integration-tests lsp_csharp -- --ignored --nocapture"]
fn lsp_csharp() -> Result<()> {
    init_lsp_tracing("lsp_csharp");
    with_temp_project_build(vec!["csharp".to_string()], "lsp-test-csharp", |project_path| {
        eprintln!("[lsp_csharp] project_path={}", project_path.display());

        let workspace_root = runtime_env(project_path, poly_bench_dsl::Lang::CSharp);
        let workspace_root_str = workspace_root.to_string_lossy();
        eprintln!("[lsp_csharp] workspace_root={}", workspace_root_str);

        eprintln!("[lsp_csharp] Creating OmniSharpClient (Roslyn Language Server)...");
        let client = OmniSharpClient::new(&workspace_root_str)
            .map_err(|e| miette::miette!("OmniSharpClient::new failed: {}", e))?;
        eprintln!("[lsp_csharp] Client created, calling initialize()...");

        client.initialize().map_err(|e| miette::miette!("initialize failed: {}", e))?;
        eprintln!("[lsp_csharp] initialize() OK, writing Test.cs...");

        // File must be inside workspace so Roslyn can resolve project (polybench.csproj)
        let test_file = workspace_root.join("Test.cs");
        let content = r#"public static class Test { public static int Foo(int x) => x; }
"#;
        std::fs::write(&test_file, content)
            .map_err(|e| miette::miette!("Failed to write Test.cs: {}", e))?;
        eprintln!("[lsp_csharp] Test.cs written at {}, calling did_open()...", test_file.display());

        let uri = path_to_file_uri(&test_file);
        client.did_open(&uri, content, 1).map_err(|e| miette::miette!("did_open failed: {}", e))?;
        eprintln!("[lsp_csharp] did_open() OK, calling hover()...");

        // Hover on "Foo" at line 0, char ~25
        let hover_result = client.hover(&uri, 0, 25);
        eprintln!("[lsp_csharp] hover() returned: {:?}", hover_result.as_ref().map(|_| "Ok"));
        hover_result.map_err(|e| miette::miette!("hover failed: {}", e))?;

        eprintln!("[lsp_csharp] test passed");
        Ok(())
    })
}
