# Native LSP Runtime Spec

This document specifies how poly-bench installs and resolves native language servers (LSPs) for embedded code in `.bench` files. The design ensures poly-bench's LSP support is self-contained: no external prerequisites beyond the language toolchain used for benchmark execution.

## Canonical Layout

All runtimes use a consistent layout under `.polybench/runtime-env/{lang}/`:

```
.polybench/runtime-env/
├── go/
│   ├── go.mod
│   └── bin/                    # gopls (local install via go install)
├── ts/
│   ├── package.json            # includes typescript-language-server
│   ├── node_modules/
│   │   └── .bin/typescript-language-server
│   └── tsconfig.json
├── rust/
│   ├── Cargo.toml
│   └── bin/                    # rust-analyzer (downloaded prebuilt)
├── python/
│   ├── requirements.txt        # includes pyright
│   └── .venv/bin/              # pyright-langserver
├── zig/
│   └── bin/                    # zls (downloaded prebuilt)
└── csharp/
    └── .csharp-ls/             # roslyn-language-server (dotnet tool install)
```

## Resolution Order

Each runtime's LSP client resolves the executable in this order:

1. **Workspace-local**: `find_executable_in_workspace(workspace_root)` — checks the runtime-env path
2. **Fallback**: `find_executable()` — PATH, env vars, common locations

The `workspace_root` passed to LSP clients is `.polybench/runtime-env/{lang}/` when in a poly-bench project. See `poly-bench-lsp-v2/src/server.rs` and `find_module_root`.

## Per-Runtime Install Procedure

| Runtime | LSP | Install Step | Location |
|---------|-----|--------------|----------|
| Go | gopls | `GOBIN=<go_env>/bin go install golang.org/x/tools/gopls@latest` | `go/bin/gopls` |
| Rust | rust-analyzer | Download from GitHub releases, extract | `rust/bin/rust-analyzer` |
| TypeScript | typescript-language-server | `npm install` (in devDependencies) | `ts/node_modules/.bin/typescript-language-server` |
| Python | pyright | `pip install -r requirements.txt` (in requirements) | `python/.venv/bin/pyright-langserver` |
| Zig | ZLS | Download from GitHub releases, extract | `zig/bin/zls` |
| C# | roslyn-language-server | `dotnet tool install` or copy from `POLYBENCH_CSHARP_LSP_BIN`/PATH | `csharp/.csharp-ls/roslyn-language-server` |

## Pinned Versions

| LSP | Version | Rationale |
|-----|---------|-----------|
| gopls | `@latest` | Built with user's Go; compatibility automatic |
| rust-analyzer | Pinned release tag (e.g. `2024-11-25`) | Stable, supports current Rust editions |
| typescript-language-server | `^3.0.0` | npm semver; works with typescript ^5 |
| pyright | `pyright[nodejs]` latest | Already in requirements.txt |
| ZLS | `0.15.1` | Already pinned |
| roslyn-language-server | Pinned prerelease (e.g. 5.5.0-2.26103.6) | Microsoft's official LSP from dotnet/roslyn |

## External Prerequisites

For **LSP only**, poly-bench requires:

- **Go**: `go` in PATH (to run `go install` for gopls)
- **Rust**: None (prebuilt binary)
- **TypeScript**: `npm` in PATH (for `npm install`)
- **Python**: `python3` in PATH (for venv + pip)
- **Zig**: None (prebuilt binary)
- **C#**: `dotnet` in PATH (for `dotnet tool install`), or `roslyn-language-server`/`POLYBENCH_CSHARP_LSP_BIN` (fallback)

## C# Behavior

C# tries **Roslyn Language Server** first, then falls back to **csharp-ls** when roslyn-language-server has packaging issues (e.g. DotnetToolSettings.xml). `install_local_roslyn_language_server` runs `dotnet tool install roslyn-language-server --tool-path .csharp-ls`; on failure, runs `dotnet tool install csharp-ls --tool-path .csharp-ls`. If `POLYBENCH_CSHARP_LSP_BIN` or either tool in PATH is set, that binary is copied instead. The LSP client checks `find_executable_in_workspace` first (`.csharp-ls/roslyn-language-server` or `.csharp-ls/csharp-ls`), then falls back to PATH. Requires .NET 10+ for the tool install.
