# Consolidated command runner for daily dev:
# - fmt
# - build
# - pr
# - release
#
# Run `just` to list available commands.

set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

default:
  @just --list --unsorted

# Run any command in dev scope.
# Usage:
#   just dev fmt
#   just dev build debug
[group('scopes')]
dev +args:
  PB_ENV=dev just {{args}}

# Run any command in prod scope.
# Usage:
#   just prod build
#   just prod release minor
[group('scopes')]
prod +args:
  PB_ENV=prod just {{args}}

# Format Rust code.
# Usage:
#   just fmt                # format
#   just fmt check          # check-only for CI
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
# Usage:
#   just dev test             # cargo test --all
#   just dev test check       # fmt-check + clippy + test
#   just dev test cover       # coverage-friendly run
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
# Usage:
#   just dev lint
[group('dev')]
lint:
  cargo clippy --all-targets --

# Build poly-bench.
# Usage:
#   just build              # optimized release build
#   just build debug        # fast debug build
#   just build check        # compile check
#   just build watch        # rebuild on changes
[group('dev')]
build mode="release":
  #!/usr/bin/env bash
  set -euo pipefail
  case "{{mode}}" in
    release)
      printf "\033[36m[±]\033[0m Building poly-bench (release)...\r"
      cargo build --release --bin poly-bench
      printf "\033[32m[✓]\033[0m Built poly-bench (release)        \n"
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
      exit 1
      ;;
  esac

# Create a quick PR from staged changes.
# Usage:
#   just pr "my-pr-title"
[group('dev')]
pr title:
  ./scripts/quick-pr.sh "{{title}}"

# VSCode extension compile.
# Usage:
#   just dev vscode-compile
#   just prod vscode-compile
[group('vscode')]
vscode-compile:
  cd extensions/vscode && npm ci && npm run compile

# VSCode extension checks.
# Usage:
#   just dev vscode-test           # compile + lint
#   just dev vscode-test compile   # compile only
#   just dev vscode-test lint      # lint only
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
# Usage:
#   just dev vscode-bump patch
#   just dev vscode-bump minor
#   just dev vscode-bump major
#   just dev vscode-bump explicit 0.2.1
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
# Usage:
#   just dev vscode-build
#   just prod vscode-build
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
# Usage:
#   just dev vscode-publish   # uses publish:dev
#   just prod vscode-publish  # uses publish:ci
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
# Usage:
#   just release                    # defaults to patch bump
#   just release patch              # patch bump
#   just release major              # major bump
#   just release minor              # minor bump
#   just release explicit v0.2.1    # manual version override
#   just release docs               # docs-only production PR
# Recommended:
#   just prod release patch
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

# Release automation without VSCode extension bump/publish.
# Usage:
#   just prod release-no-vscode patch
#   just prod release-no-vscode minor
#   just prod release-no-vscode major
#   just prod release-no-vscode explicit v0.2.1
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

# Release aliases (short forms)
# Usage:
#   just rel-p
#   just rel-m
#   just rel-M
#   just rel-x v0.2.1
#   just rel-d
[group('prod')]
rel-p:
  just prod release patch

[group('prod')]
rel-m:
  just prod release minor

[group('prod')]
rel-M:
  just prod release major

[group('prod')]
rel-x version:
  just prod release explicit {{version}}

[group('prod')]
rel-d:
  just prod release docs
