# poly-bench VS Code / Cursor extension

Adds syntax highlighting for `.bench` files:

- **DSL**: `suite`, `setup`, `fixture`, `bench`, `description`, `iterations`, `warmup`, `hex`, and `#` comments
- **Embedded Go**: Code inside `setup go { ... }` is highlighted as Go
- **Embedded TypeScript**: Code inside `setup ts { ... }` is highlighted as TypeScript

## Install (development)

1. Open the `vscode` folder in VS Code or Cursor.
2. Press F5 (or Run > Start Debugging) to launch a new window with the extension loaded.
3. Open a `.bench` file to see highlighting.

## Install (from repo)

1. In VS Code/Cursor: **Extensions** → **...** → **Install from VSIX** (if you built one).
2. Or copy the `vscode` folder into your workspace and use **Developer: Install Extension from Location** with the `vscode` path.

## Publishing to the Marketplace

1. **Create a publisher** (one-time): Go to [Visual Studio Marketplace - Publishers](https://marketplace.visualstudio.com/manage) and sign in with Microsoft/GitHub. Create a new publisher; use the same id as `publisher` in `package.json` (e.g. `poly-bench`).

2. **Install the publishing tool**:
   ```bash
   npm install -g @vscode/vsce
   ```

3. **Build and package** (from the `vscode` folder):
   ```bash
   npm run compile
   vsce package
   ```
   This creates `poly-bench-0.0.1.vsix`.

4. **Publish** (requires a Personal Access Token with Marketplace (Publish) scope):
   - Create a PAT at [Azure DevOps → User settings → Personal access tokens](https://dev.azure.com) with **Marketplace (Publish)** scope.
   - Run:
   ```bash
   vsce login poly-bench
   # enter the PAT when prompted
   vsce publish
   ```
   Or bump version and publish: `vsce publish patch` (0.0.1 → 0.0.2).

5. **Install from VSIX** (without publishing): Use the `.vsix` from step 3: in VS Code, **Extensions** → **...** → **Install from VSIX...** and select the file.

## Formatting notes

- **Suite closing brace**: The `}` that closes the top-level `suite` block should be at **column 0** (no leading spaces). This avoids the grammar ending the suite at an inner `}`.
- **Setup closing brace**: `setup go { }` and `setup ts { }` end at a `}` on its own line with **2–4 leading spaces**. Use consistent 2- or 4-space indent for the block body so the first such `}` is the setup closer.
