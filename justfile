# poly-bench development commands
# Run `just` to list all commands. Use `just <recipe>` to run.
#
# Quick reference:
#   Build:  just b (dev) | just b-p (prod) | just b-d-f (force) | just b-d-d (CLI only)
#   Release: just rel-p | just rel-m | just rel-M
#   Scopes: just dev <cmd> | just prod <cmd>

set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

[doc("Show this help and list all commands")]
default:
  #!/usr/bin/env bash
  echo "poly-bench development commands"
  echo ""
  echo "Usage: just <recipe> [args...]"
  echo ""
  echo "Common commands:"
  echo "  just b         - dev build (grammar, CLI, extension, VSIX)"
  echo "  just b-d-f     - dev build, force rebuild everything"
  echo "  just b-p       - prod build (release binary + extension)"
  echo "  just b-d-d     - CLI only (debug binary)"
  echo "  just dev fmt   - format Rust code"
  echo "  just dev lint  - clippy"
  echo "  just dev test  - run tests"
  echo "  just rel-p     - release patch (prod)"
  echo ""
  echo "All recipes:"
  just --list --unsorted

# Run any command in dev scope.
[doc("Run command in dev scope: just dev <recipe> [args]")]
[group('scopes')]
dev +args:
  PB_ENV=dev just {{args}}

# Run any command in prod scope.
[doc("Run command in prod scope: just prod <recipe> [args]")]
[group('scopes')]
prod +args:
  PB_ENV=prod just {{args}}

# Format Rust code.
[doc("Format Rust: just fmt [fix|check]")]
[group('dev')]
fmt mode="fix":
  #!/usr/bin/env bash
  set -euo pipefail
  case "{{mode}}" in
    fix)
      cargo +nightly fmt --all
      ;;
    check)
      cargo +nightly fmt --all -- --check
      ;;
    *)
      echo "Usage: just fmt [fix|check]"
      exit 1
      ;;
  esac

# Test poly-bench.
[doc("Run tests: just dev test [unit|check|cover]")]
[group('dev')]
test mode="unit":
  #!/usr/bin/env bash
  set -euo pipefail
  case "{{mode}}" in
    unit)
      cargo test --all
      ;;
    check)
      cargo +nightly fmt --all -- --check
      cargo clippy --all-targets --
      cargo test --all
      ;;
    cover)
      cargo test --all --no-run 2>/dev/null || true
      cargo test --all
      ;;
    *)
      echo "Usage: just test [unit|check|cover]"
      exit 1
      ;;
  esac

# Lint poly-bench.
[doc("Run clippy: just dev lint")]
[group('dev')]
lint:
  cargo clippy --all-targets --

# Build poly-bench (grammar, CLI, extension, VSIX).
# Shortcuts: b, b-d, b-d-f, b-p, b-d-d
[doc("Build: just dev build | just prod build | just build [debug|check|watch]. Shortcuts: b, b-d, b-d-f, b-p, b-d-d")]
[group('dev')]
build mode="release":
  #!/usr/bin/env bash
  set -euo pipefail
  pb_env="${PB_ENV:-}"
  case "{{mode}}" in
    release)
      if [[ "$pb_env" == "dev" ]]; then
        ./scripts/dev-build.sh
      elif [[ "$pb_env" == "prod" ]]; then
        ./scripts/prod-build.sh
      else
        printf "\033[36m[±]\033[0m Building poly-bench (release)...\r"
        cargo build --release --bin poly-bench
        printf "\033[32m[✓]\033[0m Built poly-bench (release)        \n"
      fi
      ;;
    force)
      if [[ "$pb_env" == "dev" ]]; then
        ./scripts/dev-build.sh --force
      else
        echo "Usage: just dev build force"
        exit 1
      fi
      ;;
    debug)
      cargo build --bin poly-bench
      echo "Built: target/debug/poly-bench"
      ;;
    check)
      cargo check --bin poly-bench
      ;;
    watch)
      cargo watch -x "build --bin poly-bench"
      ;;
    *)
      echo "Usage: just build [release|debug|check|watch]"
      echo "  just dev build       - full dev build (grammar, CLI, extension, VSIX)"
      echo "  just dev build force - full dev build, rebuild everything"
      echo "  just prod build      - full prod build"
      echo "  just build debug     - CLI only"
      echo "  just build release   - CLI only"
      exit 1
      ;;
  esac

# Create a quick PR from staged changes.
[doc("Create PR from staged files: just pr \"title\"")]
[group('dev')]
pr title:
  ./scripts/quick-pr.sh "{{title}}"

# VSCode extension compile.
[doc("Compile VS Code extension: just dev vscode-compile")]
[group('vscode')]
vscode-compile:
  cd extensions/vscode && npm ci && npm run compile

# VSCode extension checks.
[doc("Extension test: just dev vscode-test [all|compile|lint]")]
[group('vscode')]
vscode-test mode="all":
  #!/usr/bin/env bash
  set -euo pipefail
  cd extensions/vscode
  npm ci
  case "{{mode}}" in
    all)
      npm run compile
      npm run lint
      ;;
    compile)
      npm run compile
      ;;
    lint)
      npm run lint
      ;;
    *)
      echo "Usage: just vscode-test [all|compile|lint]"
      exit 1
      ;;
  esac

# VSCode extension version bump.
[doc("Bump extension version: just dev vscode-bump [patch|minor|major|explicit] [ver]")]
[group('vscode')]
vscode-bump kind="patch" version="":
  #!/usr/bin/env bash
  set -euo pipefail
  cd extensions/vscode

  current="$(node -e "const fs=require('fs');const j=JSON.parse(fs.readFileSync('package.json','utf8'));const m=j.version.match(/^([0-9]+\\.[0-9]+\\.[0-9]+)(-.+)?$/);if(!m){console.error('Invalid semver: '+j.version);process.exit(1)};process.stdout.write(j.version);")"

  if [[ "{{kind}}" == "explicit" ]]; then
    if [[ -z "{{version}}" ]]; then
      echo "Usage: just vscode-bump explicit X.Y.Z"
      exit 1
    fi
    next="{{version}}"
  else
    next="$(node -e "const v='$current';const m=v.match(/^(\\d+)\\.(\\d+)\\.(\\d+)(-.+)?$/);if(!m){console.error('Invalid semver: '+v);process.exit(1)};const pre=m[4]??'';const kind='{{kind}}';let out='';if(kind==='patch'){out=m[1]+'.'+m[2]+'.'+(Number(m[3])+1)+pre;}else if(kind==='minor'){out=m[1]+'.'+(Number(m[2])+1)+'.0'+pre;}else if(kind==='major'){out=(Number(m[1])+1)+'.0.0'+pre;}else{console.error('Usage: just vscode-bump [patch|minor|major|explicit] [version]');process.exit(1)};process.stdout.write(out);")"
  fi

  node -e "const fs=require('fs');const p='package.json';const j=JSON.parse(fs.readFileSync(p,'utf8'));j.version='$next';fs.writeFileSync(p,JSON.stringify(j,null,2)+'\n');console.log('VSCode extension version: '+j.version);"
  echo "Bumped VSCode extension: $current -> $next"

# VSCode extension build/package.
[doc("Package extension as .vsix: just dev vscode-build")]
[group('vscode')]
vscode-build:
  #!/usr/bin/env bash
  set -euo pipefail
  cd extensions/vscode
  npm ci
  npm run compile
  # Package extension as .vsix using on-demand vsce.
  npx @vscode/vsce package

# VSCode extension publish.
[doc("Publish extension to marketplace: just dev/prod vscode-publish")]
[group('vscode')]
[confirm("Publish VSCode extension?")]
vscode-publish:
  #!/usr/bin/env bash
  set -euo pipefail
  cd extensions/vscode
  npm ci

  pb_env="${PB_ENV:-dev}"
  if [[ "$pb_env" == "prod" ]]; then
    npm run publish:ci
  else
    npm run publish:dev
  fi

# Release automation.
[doc("Release: just prod release [patch|minor|major|explicit|docs] [ver]. Shortcuts: rel-p, rel-m, rel-M")]
[group('prod')]
[confirm("Proceed with release workflow?")]
release kind="patch" version="":
  #!/usr/bin/env bash
  set -euo pipefail

  pb_env="${PB_ENV:-dev}"
  if [[ "$pb_env" != "prod" ]]; then
    echo "Release commands require prod scope."
    echo "Run with: just prod release {{kind}} {{version}}"
    exit 1
  fi

  kind="{{kind}}"
  version="{{version}}"

  if [[ "$kind" == "docs" ]]; then
    if [[ -n "$version" ]]; then
      echo "Docs release does not use a version argument."
      exit 1
    fi
    gh auth status >/dev/null
    gh pr create \
      --base production \
      --head main \
      --title "Docs release" \
      --body "## Docs release\n\nThis PR deploys docs-only changes to production.\n\nAfter merge, comment \`/release\` on this PR to run the release workflow (it will skip binary publish for docs-only changes)."
    echo "Docs production PR created."
    exit 0
  fi

  if [[ "$kind" == "explicit" ]]; then
    if [[ -z "$version" ]]; then
      echo "Usage: just release explicit vX.Y.Z"
      exit 1
    fi
    rel_version="$version"
    ext_ver="${version#v}"
  else
    current="$(node -e "const fs=require('fs');const m=fs.readFileSync('Cargo.toml','utf8').match(/^version = \"([0-9]+\\.[0-9]+\\.[0-9]+(?:-[0-9A-Za-z.-]+)?)\"/m);if(!m){console.error('Could not parse Cargo.toml version');process.exit(1)};process.stdout.write(m[1]);")"
    ext_current="$(node -e "const fs=require('fs');const j=JSON.parse(fs.readFileSync('extensions/vscode/package.json','utf8'));const m=j.version.match(/^([0-9]+\\.[0-9]+\\.[0-9]+)(-.+)?$/);if(!m){console.error('Invalid VSCode extension semver: '+j.version);process.exit(1)};process.stdout.write(j.version);")"
    case "$kind" in
      patch)
        rel_version="$(node -e "const v='$current';const m=v.match(/^(\\d+)\\.(\\d+)\\.(\\d+)(?:-.+)?$/);if(!m){console.error('Invalid semver: '+v);process.exit(1)};process.stdout.write('v'+m[1]+'.'+m[2]+'.'+(Number(m[3])+1));")"
        ext_ver="$(node -e "const v='$ext_current';const m=v.match(/^(\\d+)\\.(\\d+)\\.(\\d+)(-.+)?$/);if(!m){console.error('Invalid semver: '+v);process.exit(1)};const pre=m[4]??'';process.stdout.write(m[1]+'.'+m[2]+'.'+(Number(m[3])+1)+pre);")"
        ;;
      minor)
        rel_version="$(node -e "const v='$current';const m=v.match(/^(\\d+)\\.(\\d+)\\.(\\d+)(?:-.+)?$/);if(!m){console.error('Invalid semver: '+v);process.exit(1)};process.stdout.write('v'+m[1]+'.'+(Number(m[2])+1)+'.0');")"
        ext_ver="$(node -e "const v='$ext_current';const m=v.match(/^(\\d+)\\.(\\d+)\\.(\\d+)(-.+)?$/);if(!m){console.error('Invalid semver: '+v);process.exit(1)};const pre=m[4]??'';process.stdout.write(m[1]+'.'+(Number(m[2])+1)+'.0'+pre);")"
        ;;
      major)
        rel_version="$(node -e "const v='$current';const m=v.match(/^(\\d+)\\.(\\d+)\\.(\\d+)(?:-.+)?$/);if(!m){console.error('Invalid semver: '+v);process.exit(1)};process.stdout.write('v'+(Number(m[1])+1)+'.0.0');")"
        ext_ver="$(node -e "const v='$ext_current';const m=v.match(/^(\\d+)\\.(\\d+)\\.(\\d+)(-.+)?$/);if(!m){console.error('Invalid semver: '+v);process.exit(1)};const pre=m[4]??'';process.stdout.write((Number(m[1])+1)+'.0.0'+pre);")"
        ;;
      *)
        echo "Usage: just release [patch|minor|major|explicit|docs] [version]"
        exit 1
        ;;
    esac
    echo "Releasing $kind versions:"
    echo "  poly-bench: $current -> ${rel_version#v}"
    echo "  vscode-ext: $ext_current -> $ext_ver"
  fi

  gh auth status >/dev/null
  git checkout main
  git pull origin main

  ver="${rel_version#v}"

  sed -i.bak "s/^version = \".*\"/version = \"$ver\"/" Cargo.toml
  rm -f Cargo.toml.bak

  node -e "const fs=require('fs');const p='extensions/vscode/package.json';const j=JSON.parse(fs.readFileSync(p,'utf8'));j.version='$ext_ver';fs.writeFileSync(p,JSON.stringify(j,null,2)+'\n');console.log('VSCode extension version -> '+j.version);"

  git add Cargo.toml extensions/vscode/package.json
  if git diff --staged --quiet; then
    echo "No version changes to commit."
  else
    git commit -m "chore: release $rel_version"
    git push origin main
  fi

  git tag -a "$rel_version" -m "Release $rel_version"
  git push origin "$rel_version"

  gh release create "$rel_version" \
    --title "$rel_version" \
    --generate-notes \
    --prerelease

  gh pr create \
    --base production \
    --head main \
    --title "Release $rel_version" \
    --body "## Release $rel_version\n\nThis PR releases $rel_version to production.\n\nWhen merged, comment \`/release\` on this PR to promote the prerelease to the latest release."

  echo "Done. Review and merge the release PR to publish."

# Release automation without VSCode extension.
[doc("Release without VSCode: just prod release-no-vscode [patch|minor|major|explicit|docs]")]
[group('prod')]
[confirm("Proceed with release workflow (no VSCode extension bump/publish)?")]
release-no-vscode kind="patch" version="":
  #!/usr/bin/env bash
  set -euo pipefail

  pb_env="${PB_ENV:-dev}"
  if [[ "$pb_env" != "prod" ]]; then
    echo "Release commands require prod scope."
    echo "Run with: just prod release-no-vscode {{kind}} {{version}}"
    exit 1
  fi

  kind="{{kind}}"
  version="{{version}}"

  if [[ "$kind" == "docs" ]]; then
    if [[ -n "$version" ]]; then
      echo "Docs release does not use a version argument."
      exit 1
    fi
    gh auth status >/dev/null
    gh pr create \
      --base production \
      --head main \
      --title "Docs release" \
      --body "## Docs release\n\nThis PR deploys docs-only changes to production.\n\nAfter merge, comment \`/release\` on this PR to run the release workflow (it will skip binary publish for docs-only changes)."
    echo "Docs production PR created."
    exit 0
  fi

  if [[ "$kind" == "explicit" ]]; then
    if [[ -z "$version" ]]; then
      echo "Usage: just release-no-vscode explicit vX.Y.Z"
      exit 1
    fi
    rel_version="$version"
  else
    current="$(node -e "const fs=require('fs');const m=fs.readFileSync('Cargo.toml','utf8').match(/^version = \"([0-9]+\\.[0-9]+\\.[0-9]+(?:-[0-9A-Za-z.-]+)?)\"/m);if(!m){console.error('Could not parse Cargo.toml version');process.exit(1)};process.stdout.write(m[1]);")"
    case "$kind" in
      patch)
        rel_version="$(node -e "const v='$current';const m=v.match(/^(\\d+)\\.(\\d+)\\.(\\d+)(?:-.+)?$/);if(!m){console.error('Invalid semver: '+v);process.exit(1)};process.stdout.write('v'+m[1]+'.'+m[2]+'.'+(Number(m[3])+1));")"
        ;;
      minor)
        rel_version="$(node -e "const v='$current';const m=v.match(/^(\\d+)\\.(\\d+)\\.(\\d+)(?:-.+)?$/);if(!m){console.error('Invalid semver: '+v);process.exit(1)};process.stdout.write('v'+m[1]+'.'+(Number(m[2])+1)+'.0');")"
        ;;
      major)
        rel_version="$(node -e "const v='$current';const m=v.match(/^(\\d+)\\.(\\d+)\\.(\\d+)(?:-.+)?$/);if(!m){console.error('Invalid semver: '+v);process.exit(1)};process.stdout.write('v'+(Number(m[1])+1)+'.0.0');")"
        ;;
      *)
        echo "Usage: just release-no-vscode [patch|minor|major|explicit|docs] [version]"
        exit 1
        ;;
    esac
    echo "Releasing $kind version (no VSCode extension):"
    echo "  poly-bench: $current -> ${rel_version#v}"
  fi

  gh auth status >/dev/null
  git checkout main
  git pull origin main

  ver="${rel_version#v}"

  sed -i.bak "s/^version = \".*\"/version = \"$ver\"/" Cargo.toml
  rm -f Cargo.toml.bak

  git add Cargo.toml
  if git diff --staged --quiet; then
    echo "No version changes to commit."
  else
    git commit -m "chore: release $rel_version (no vscode)"
    git push origin main
  fi

  git tag -a "$rel_version" -m "Release $rel_version"
  git push origin "$rel_version"

  gh release create "$rel_version" \
    --title "$rel_version" \
    --generate-notes \
    --prerelease

  gh pr create \
    --base production \
    --head main \
    --title "Release $rel_version (no VSCode extension)" \
    --body "## Release $rel_version\n\nThis PR releases $rel_version to production without a VSCode extension version bump/publish.\n\nWhen merged, comment \`/release\` on this PR to promote the prerelease to the latest release."

  echo "Done. Review and merge the release PR to publish."

# Build shortcuts (see `build` for full docs)
[doc("build: dev build (full) - grammar, CLI, extension, VSIX")]
[group('dev')]
b:
  just dev build

[doc("build: dev build (same as b)")]
[group('dev')]
b-d:
  just dev build

[doc("build: dev build force - rebuild everything")]
[group('dev')]
b-d-f:
  just dev build force

[doc("build: prod build - release binary + extension + VSIX")]
[group('prod')]
b-p:
  just prod build

[doc("build: dev build debug - CLI only")]
[group('dev')]
b-d-d:
  just dev build debug

# Release shortcuts (see `release` for full docs)
[doc("release: patch bump")]
[group('prod')]
rel-p:
  just prod release patch

[doc("release: minor bump")]
[group('prod')]
rel-m:
  just prod release minor

[doc("release: major bump")]
[group('prod')]
rel-M:
  just prod release major

[doc("release: explicit version")]
[group('prod')]
rel-x version:
  just prod release explicit {{version}}

[doc("release: docs-only PR")]
[group('prod')]
rel-d:
  just prod release docs
