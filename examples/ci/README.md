# CI/CD Pipeline Examples

These are the GitHub Actions workflows included in the toolkit. Copy them to your project's `.github/workflows/` directory.

## Workflows

### ci.yml — Build, Test & Deploy Pipeline

**Trigger:** PR + push to `main`

| Job             | What it does                                                        |
| --------------- | ------------------------------------------------------------------- |
| `rust-fmt`      | Checks Rust formatting (`cargo fmt --check`)                        |
| `rust-clippy`   | Security lints — blocks `unwrap()` and unchecked arithmetic         |
| `anchor-build`  | Compiles all Anchor programs                                        |
| `cargo-audit`   | Scans dependencies for known vulnerabilities                        |
| `frontend`      | Builds Next.js frontend (`npx next build`)                          |
| `backend`       | Type-checks backend (`npm run build`)                               |
| `deploy-devnet` | Deploys to devnet (main branch only, requires `devnet` environment) |

### security.yml — Weekly Vulnerability Scan

**Trigger:** Every Monday at 08:00 UTC (or manual)

- Runs `cargo audit` against `contracts/Cargo.lock`
- Automatically creates a GitHub issue if vulnerabilities are found
- Won't create duplicate issues if one is already open

## Installation

```bash
# From your project root
mkdir -p .github/workflows
cp examples/ci/ci.yml .github/workflows/
cp examples/ci/security.yml .github/workflows/
```

## Required Secrets

| Secret                    | Where to get it                       | Used by                      |
| ------------------------- | ------------------------------------- | ---------------------------- |
| `DEVNET_DEPLOYER_KEYPAIR` | `solana-keygen new` → copy JSON array | `ci.yml` (deploy-devnet job) |

## Required Environments

| Environment | Purpose                                                        |
| ----------- | -------------------------------------------------------------- |
| `devnet`    | Gates the deploy-devnet job (optional: add required reviewers) |

See the main [README](../../README.md#cicd-github-secrets-setup) for step-by-step setup instructions.
