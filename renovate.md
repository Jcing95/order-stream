1. Create the App (browser)

Open <https://github.com/settings/apps/new>

- GitHub App name: order-stream-renovate (must be globally unique; adjust if taken)
- Homepage URL: <https://github.com/jcing95/order-stream>
- Webhook: uncheck Active
- Repository permissions:
  - Contents: Read and write
  - Pull requests: Read and write
  - Issues: Read and write (Dependency Dashboard)
  - Workflows: Read and write (Renovate updates the action pins in its own workflow file)
- Where can this App be installed: Only on this account

Create it. Note the App ID shown at the top of the settings page.

1. Generate the private key (browser)

On the App's settings page, scroll to Private keys → Generate a private key. A .pem downloads. Move it somewhere you can reach from the repo:

mv ~/Downloads/order-stream-renovate.*.private-key.pem /tmp/renovate-key.pem

1. Install it on the repo (browser)

App settings → Install App → your account → Only select repositories → order-stream → Install.

1. Confirm the install and read the App ID with gh

gh api /user/installations --jq '.installations[] | select(.app_slug|test("renovate")) | {app_id, app_slug, id}'

This should print the App ID and slug. If it returns nothing, the install in step 3 didn't take.

1. Create the secrets

gh secret set RENOVATE_APP_ID --body "<app-id-from-step-4>"
gh secret set RENOVATE_APP_PRIVATE_KEY < /tmp/renovate-key.pem
gh secret list

Reading the key from stdin is what preserves the newlines and the BEGIN/END lines; pasting it into --body will break it. Then delete the local copy:

rm /tmp/renovate-key.pem

1. Commit onto the branch

git add .gitignore Cargo.lock flake.nix renovate.json .github/workflows/renovate.yml
git commit -m "$(cat <<'EOF'
chore: add self-hosted Renovate

Covers cargo, npm, docker-compose and nix flake inputs.

Cargo.lock is now tracked: without it Renovate's cargo manager
(rangeStrategy update-lockfile) has nothing to update, so in-range
releases produce no PR.

flake.nix pinned wasm-bindgen-cli at 0.2.120 while Cargo.lock resolves
the crate at 0.2.129; the two must match or cargo leptos build fails.

Co-Authored-By: Claude Opus 5 (1M context) <noreply@anthropic.com>
EOF
)"
git push

1. Verify the build before merging

cargo leptos build fails today because your installed CLI is 0.2.120 against a 0.2.129 lock. Confirm the fix:

direnv reload # binstalls wasm-bindgen-cli 0.2.129
cargo leptos build

1. Merge PR #3

gh pr merge 3 --squash

Nothing before this point makes Renovate runnable. The workflow has to be on main.

1. Dry run

gh workflow run renovate.yml -f dryRun=true -f logLevel=debug
gh run watch

Then read the log:

gh run view --log | grep -E 'Dependency extraction|packageFile|dryRun'

You want four managers extracting: cargo (~30 crates), npm (the three end2end devDeps), docker-compose (postgres), and nix (nixpkgs, flake-utils, fenix). Also confirm the custom manager found wasm-bindgen-cli in flake.nix.

If nix extracts nothing, drop "nix": { "enabled": true } from renovate.json; it's a beta manager and may not provision in the container. Nothing else is affected.

1. Real run

gh workflow run renovate.yml
gh run watch
gh pr list --author app/order-stream-renovate
gh issue list --search "Dependency Dashboard"

Expect a Dependency Dashboard issue, one grouped end2end dev dependencies PR (Playwright 1.44 → current, @types/node 20 → 24, TS 5.4 → 5.9), and lock maintenance PRs for Cargo.lock and flake.lock. A Postgres major should appear as an unchecked box on the dashboard, not a PR.

Check one Cargo PR's diff touches only Cargo.lock:

gh pr diff <n> --name-only

If it instead shows an artifact-update error in the PR body, the container couldn't provision the nightly toolchain from rust-toolchain.toml. Fix by adding "constraints": { "rust": "stable" } to renovate.json.

After that, the weekly Monday 04:00 UTC cron takes over.
