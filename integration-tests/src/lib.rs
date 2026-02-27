//! Shared helpers for integration tests.
//!
//! Temp projects are created under `integration-tests/.tmp-test-projects/` so they are visible
//! during test runs. Each test gets a unique subdir that is deleted when the test completes.
//! LSP diagnostic output is written to `integration-tests/.lsp-test-outputs/` (persistent).

use miette::Result;
use poly_bench_project::{build::BuildOptions, init::InitOptions, runtime_env};
use poly_bench_dsl::Lang;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing_subscriber::prelude::*;

static TRACING_INIT: std::sync::Once = std::sync::Once::new();

/// Base directory for LSP diagnostic outputs (persistent, not cleared).
fn lsp_test_outputs_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(".lsp-test-outputs")
}

/// Initialize tracing to print LSP request/response. Writes to stderr and to a persistent
/// log file in `.lsp-test-outputs/{test_name}-{timestamp}.log`. Call at start of LSP tests.
pub fn init_lsp_tracing(test_name: &str) {
    TRACING_INIT.call_once(|| {
        let filter = std::env::var("RUST_LOG")
            .unwrap_or_else(|_| "poly_bench_traits=trace".to_string());

        let out_dir = lsp_test_outputs_dir();
        let _ = std::fs::create_dir_all(&out_dir);

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let log_path = out_dir.join(format!("{}-{}.log", test_name, timestamp));

        if let Ok(file) = std::fs::File::create(&log_path) {
            eprintln!("[integration-test] LSP diagnostics -> {}", log_path.display());
            let (non_blocking, _guard) = tracing_appender::non_blocking(file);
            // Keep guard alive for the process lifetime so the file writer keeps working
            std::mem::forget(_guard);

            let layer_stderr = tracing_subscriber::fmt::layer()
                .with_writer(std::io::stderr)
                .with_ansi(true);
            let layer_file = tracing_subscriber::fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false);

            let _ = tracing_subscriber::registry()
                .with(tracing_subscriber::EnvFilter::new(&filter))
                .with(layer_stderr)
                .with(layer_file)
                .try_init();
        } else {
            let _ = tracing_subscriber::fmt()
                .with_env_filter(filter)
                .with_writer(std::io::stderr)
                .try_init();
        }
    });
}

/// Base directory for temp test projects (inside integration-tests crate).
/// Created on first use; add to .gitignore.
fn tmp_test_projects_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(".tmp-test-projects")
}

/// Clear all temp test projects. Call at start of test run and on Ctrl+C.
pub fn clear_tmp_test_projects() {
    let base = tmp_test_projects_dir();
    if base.exists() {
        let _ = std::fs::remove_dir_all(&base);
        eprintln!("[integration-test] Cleared {}", base.display());
    }
}

static CTRLC_REGISTERED: std::sync::Once = std::sync::Once::new();

/// Register Ctrl+C handler to clear tmp dir and exit. Call once at test binary start.
fn ensure_ctrlc_handler() {
    CTRLC_REGISTERED.call_once(|| {
        let _ = ctrlc::set_handler(move || {
            clear_tmp_test_projects();
            std::process::exit(130);
        });
    });
}

/// Create a temp project with init, run build (requires cwd to be project), then run the closure.
/// Temp dir is created in integration-tests/.tmp-test-projects/ and deleted when done.
/// Restores cwd after the closure returns.
pub fn with_temp_project_build<F, R>(languages: Vec<String>, project_name: &str, f: F) -> Result<R>
where
    F: FnOnce(&Path) -> Result<R>,
{
    ensure_ctrlc_handler();
    clear_tmp_test_projects();
    let base = tmp_test_projects_dir();
    std::fs::create_dir_all(&base)
        .map_err(|e| miette::miette!("Failed to create tmp test dir {}: {}", base.display(), e))?;

    let temp = tempfile::Builder::new()
        .prefix(&format!("{}-", project_name))
        .tempdir_in(&base)
        .map_err(|e| miette::miette!("Failed to create temp dir in {}: {}", base.display(), e))?;

    let project_path = temp.path().join(project_name);

    let options = InitOptions {
        name: project_path.to_string_lossy().to_string(),
        languages,
        no_example: true,
        quiet: true,
        defer_final_message: true,
    };
    poly_bench_project::init::init_project(&options)?;

    let project_path = project_path
        .canonicalize()
        .unwrap_or_else(|_| project_path.clone());

    eprintln!(
        "[integration-test] Building project at {} (uses real install_local_gopls, npm install, etc. - may take 1-2 min)...",
        project_path.display()
    );
    let build_options = BuildOptions::default();
    poly_bench_project::build::build_project_at(&project_path, &build_options)?;
    eprintln!("[integration-test] Build complete, running LSP test...");

    let result = f(&project_path);

    // TempDir dropped here -> directory deleted
    result
}

/// Create a temp project with init only (no build). For init-only tests.
/// Temp dir is created in integration-tests/.tmp-test-projects/ and deleted when done.
pub fn with_temp_project_init<F, R>(languages: Vec<String>, project_name: &str, f: F) -> Result<R>
where
    F: FnOnce(&Path) -> Result<R>,
{
    ensure_ctrlc_handler();
    clear_tmp_test_projects();
    let base = tmp_test_projects_dir();
    std::fs::create_dir_all(&base)
        .map_err(|e| miette::miette!("Failed to create tmp test dir {}: {}", base.display(), e))?;

    let temp = tempfile::Builder::new()
        .prefix(&format!("{}-", project_name))
        .tempdir_in(&base)
        .map_err(|e| miette::miette!("Failed to create temp dir in {}: {}", base.display(), e))?;

    let project_path = temp.path().join(project_name);

    let options = InitOptions {
        name: project_path.to_string_lossy().to_string(),
        languages,
        no_example: true,
        quiet: true,
        defer_final_message: true,
    };
    poly_bench_project::init::init_project(&options)?;

    let project_path = project_path
        .canonicalize()
        .unwrap_or_else(|_| project_path.clone());

    f(&project_path)
}

/// Get the runtime-env path for a language.
pub fn runtime_env_path(project_root: &Path, lang: Lang) -> std::path::PathBuf {
    runtime_env(project_root, lang)
}

/// Convert a path to a file:// URI for LSP. The path must be inside the workspace
/// so the language server can resolve package/module metadata.
pub fn path_to_file_uri(p: &Path) -> String {
    let path = p.canonicalize().unwrap_or_else(|_| p.to_path_buf());
    let s = path.to_string_lossy();
    if cfg!(windows) {
        format!("file:///{}", s.replace('\\', "/"))
    } else {
        format!("file://{}", s)
    }
}
