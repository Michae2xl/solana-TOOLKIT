# Solana TOOLKIT

> Complete Solana development toolkit — from scaffold to mainnet.

Production-grade setup for building, testing, auditing, and deploying Solana programs with AI-powered agents, formal verification, and automated security pipelines.

## What's Included

### CLI Tools

| Tool                | Purpose                                                          |
| ------------------- | ---------------------------------------------------------------- |
| **Solana CLI**      | Deploy, manage wallets, on-chain interactions                    |
| **Anchor**          | Build, test, deploy Anchor programs                              |
| **Rust**            | Compile Solana programs                                          |
| **Lean 4 + qedgen** | Formal verification — mathematical proofs of program correctness |
| **cargo-audit**     | Dependency vulnerability scanner                                 |
| **cargo-expand**    | Macro expansion viewer                                           |

### AI Agent Config (solana-claude)

| Component       | Count | Details                                                                                                               |
| --------------- | ----- | --------------------------------------------------------------------------------------------------------------------- |
| **Agents**      | 15    | Specialized: architect, anchor-engineer, defi-engineer, pinocchio-engineer, qa-engineer, frontend, mobile, game, etc. |
| **Commands**    | 24    | `/build-program`, `/audit-solana`, `/deploy`, `/profile-cu`, `/test-and-fix`, `/scaffold`, etc.                       |
| **Rules**       | 7     | Auto-loading for Anchor, Rust, Pinocchio, TypeScript, .NET                                                            |
| **MCP Servers** | 6     | Helius (60+ tools), Solana Foundation docs, Context7, Playwright, context-mode, memsearch                             |
| **Skills**      | 9     | Solana Foundation, SendAI DeFi, Trail of Bits security, QEDGen, Cloudflare, Colosseum, Game, Mobile, Safe Builder     |

### CI/CD Pipeline (GitHub Actions)

| Workflow         | Jobs                                                                                         | Trigger           |
| ---------------- | -------------------------------------------------------------------------------------------- | ----------------- |
| **ci.yml**       | 7 jobs: fmt, clippy, anchor build, cargo-audit, frontend build, backend check, devnet deploy | PR + push to main |
| **security.yml** | Weekly cargo-audit scan, auto-creates issues                                                 | Monday 08:00 UTC  |

### Testing

| Type       | Tool                         | Coverage                                          |
| ---------- | ---------------------------- | ------------------------------------------------- |
| **Unit**   | `anchor test` / `cargo test` | Program logic                                     |
| **E2E**    | Playwright (22 tests)        | Navigation, wallet, launch flow                   |
| **Fuzz**   | Trident                      | Arithmetic overflow, invalid accounts, edge cases |
| **Formal** | qedgen (Lean 4)              | Mathematical proofs of correctness                |

## Quick Install

```bash
# 1. Clone this repo
git clone https://github.com/Michae2xl/solana-TOOLKIT.git
cd solana-TOOLKIT

# 2. Run the installer
./install.sh

# 3. Configure API keys
cp .env.example .env
# Edit .env with your Helius API key (free at https://dev.helius.xyz)
```

## Manual Install

### Prerequisites

```bash
# Solana CLI
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"

# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --force
avm install 0.31.1
avm use 0.31.1

# Lean 4 + qedgen (formal verification)
curl -sSf https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh | sh -s -- -y
cargo install --git https://github.com/qedgen/solana-skills qedgen

# Security tools
cargo install cargo-audit
cargo install cargo-expand
```

### solana-claude

```bash
# Install into your Solana project
cd your-solana-project
curl -fsSL https://raw.githubusercontent.com/solanabr/solana-claude/main/install.sh | bash
```

## Getting Started: Zero to Devnet

### 0. Scaffold a New Project

```bash
# Option A — Anchor only (contracts)
anchor init my-project
cd my-project

# Option B — Full-stack (Next.js + Anchor)
npx create-solana-dapp my-dapp
cd my-dapp
```

After scaffolding, install the toolkit:

```bash
# From inside your project
curl -fsSL https://raw.githubusercontent.com/solanabr/solana-claude/main/install.sh | bash
cp path/to/solana-TOOLKIT/.github/workflows/* .github/workflows/
cp path/to/solana-TOOLKIT/.env.example .env
```

### 1. Configure Solana for Devnet

```bash
solana config set --url https://api.devnet.solana.com
solana-keygen new --no-passphrase     # skip if you already have a wallet
solana address                         # your devnet address
```

### 2. Get Devnet SOL (transaction fees)

**Web faucet (recommended — up to 10 SOL/day with GitHub login):**

1. Go to [faucet.solana.com](https://faucet.solana.com)
2. **Login with GitHub** — unlocks up to **10 SOL per day** (without login: ~2 SOL/day, rate-limited)
3. Select **Devnet**, paste your address, request SOL

**CLI airdrop (quick, but limited):**

```bash
solana airdrop 2            # max 2 SOL per request
solana balance              # verify
```

**Other faucets:** [solfaucet.com](https://solfaucet.com) | [QuickNode](https://faucet.quicknode.com/solana/devnet)

### 3. Get Devnet USDC (token transfers, DeFi testing)

1. Go to [faucet.circle.com](https://faucet.circle.com)
2. Select **USDC** on **Solana** → **Devnet**
3. Paste your address → claim up to **10 USDC**

USDC Devnet Mint: `4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU`

### 4. Build & Test Contracts

```bash
cd contracts/

anchor build                          # compile programs
anchor test                           # unit + integration tests
trident fuzz run --timeout 600        # fuzz testing (10 min)

# Security checks
cargo clippy -- -D warnings -W clippy::unwrap_used -W clippy::arithmetic_side_effects
cargo audit
```

### 5. Deploy to Devnet

```bash
cd contracts/

solana balance                        # need >= 5 SOL for deployment
anchor deploy --provider.cluster devnet
solana program show <PROGRAM_ID>      # verify deployment
```

After deploying, update program IDs in:

- `Anchor.toml`
- `programs/*/src/lib.rs` (`declare_id!()`)
- `.env` files (frontend/backend)

### 6. Setup Phantom for Devnet (Frontend Testing)

To test your dApp frontend, Phantom needs to be on devnet:

1. Open **Phantom** wallet (browser extension)
2. Click the **hamburger menu** (top-left) → **Settings**
3. **Developer Settings** → enable **Testnet Mode**
4. Select **Solana Devnet**
5. Your wallet now shows devnet balances and signs devnet transactions

> After getting SOL/USDC from faucets (steps 2-3), they appear in Phantom on devnet.

### Devnet Cost Summary

| Item                 | Cost                                      |
| -------------------- | ----------------------------------------- |
| SOL (faucet)         | Free — up to 10 SOL/day with GitHub login |
| USDC (Circle faucet) | Free — up to 10 USDC per request          |
| Program deployment   | Free (devnet SOL)                         |
| RPC calls            | Free (public devnet endpoint)             |

---

## Devnet to Mainnet Roadmap

### Phase 1: Foundation (Free)

- [x] Anchor programs compile and deploy
- [x] Checked arithmetic (no `unwrap()` in program code)
- [x] Dev tooling installed (solana-claude, qedgen, cargo-audit)
- [x] Devnet deployment

### Phase 2: Hardening (Free)

- [x] CI/CD pipeline (GitHub Actions)
- [x] E2E tests (Playwright)
- [x] Fuzz testing (Trident)
- [ ] Formal verification (`qedgen verify`)
- [ ] Verifiable builds (`anchor build --verifiable`)

### Phase 3: Infrastructure ($50-200/mo)

- [ ] Dedicated RPC (Helius paid plan)
- [ ] RPC fallback (2+ providers)
- [ ] Dynamic priority fees
- [ ] PostgreSQL + Redis
- [ ] On-chain indexer (Helius webhooks)

### Phase 4: Monitoring (Free tier)

- [ ] Error tracking (Sentry)
- [ ] Uptime monitoring
- [ ] On-chain alerts
- [ ] TX dashboard
- [ ] Log aggregation

### Phase 5: Frontend Production (Free)

- [ ] Wallet adapter (@solana/wallet-adapter-react)
- [ ] Multi-wallet support
- [ ] TX confirmation UX
- [ ] SEO / Open Graph

### Phase 6: Security Audit ($30K-80K)

- [ ] Professional audit (OtterSec, Neodyme, Zellic)
- [ ] Bug bounty program (Immunefi)
- [ ] Verifiable builds published
- [ ] Security documentation

### Phase 7: Mainnet Launch

- [ ] Multisig upgrade authority (Squads Protocol)
- [ ] Gradual rollout
- [ ] 24/7 monitoring
- [ ] Incident response plan
- [ ] Legal docs (ToS, Privacy Policy)

## What Tools Don't Cover

| Gap                      | Why                                                             |
| ------------------------ | --------------------------------------------------------------- |
| **Professional audit**   | No tool replaces human auditors for mainnet financial programs  |
| **Economic security**    | Bonding curve fairness, flash loan vectors, oracle manipulation |
| **Legal compliance**     | Terms of Service, Privacy Policy, securities classification     |
| **Operational security** | Private key management, multisig, phishing resistance           |
| **Infrastructure SLAs**  | Free tiers have rate limits, no uptime guarantees               |
| **Oracle data**          | Needs Pyth/Switchboard for on-chain price feeds                 |

## Cost Summary

| Phase                  | Monthly | One-time |
| ---------------------- | ------- | -------- |
| Foundation + Hardening | $0      | $0       |
| Infrastructure         | $50-320 | $0       |
| Security Audit         | $0      | $30K-80K |
| Bug Bounty             | $0      | $5K-50K  |
| Legal                  | $0      | $0-4K    |

## Project Structure

```
your-project/
├── .github/workflows/
│   ├── ci.yml                    # 7-job CI/CD pipeline
│   └── security.yml              # Weekly vulnerability scan
├── .claude/
│   ├── agents/                   # 15 specialized AI agents
│   ├── commands/                 # 24 slash commands
│   ├── rules/                    # 7 auto-loading rule sets
│   ├── skills/                   # SKILL.md + 9 external repos
│   ├── mcp.json                  # 6 MCP server configs
│   └── settings.json             # Permissions, hooks
├── contracts/
│   ├── programs/                 # Anchor programs
│   └── trident-tests/            # Fuzz testing
├── frontend/
│   ├── e2e/                      # Playwright E2E tests
│   └── playwright.config.ts
├── docs/
│   └── SOLANA-TOOLKIT.md         # Detailed roadmap + costs
└── CLAUDE.md                     # AI dev configuration
```

## Common Commands

```bash
# Build
anchor build                          # Compile programs
npx next build                        # Compile frontend

# Test
anchor test                           # Rust unit tests
npx playwright test                   # E2E tests
trident fuzz run --timeout 600        # Fuzz testing (10 min)

# Security
cargo audit                           # Dependency scan
cargo clippy -- -D warnings \
  -W clippy::unwrap_used \
  -W clippy::arithmetic_side_effects  # Security lints
qedgen verify --idl target/idl/X.json # Formal verification

# Deploy
anchor deploy --provider.cluster devnet

# AI commands (in Claude Code)
/build-program                        # Build with all checks
/audit-solana                         # Full security audit
/profile-cu                           # Optimize compute units
/deploy                               # Deploy with safety gates
/test-and-fix                         # Run tests, auto-fix
```

## CI/CD: GitHub Secrets Setup

The CI pipeline (`ci.yml`) deploys to devnet automatically on push to `main`. It needs a deployer keypair stored as a GitHub secret.

### Setup Steps

```bash
# 1. Generate a dedicated deploy keypair (don't reuse your dev wallet)
solana-keygen new --no-passphrase -o deployer-keypair.json

# 2. Fund it with devnet SOL (need ~5 SOL for deployments)
solana airdrop 5 $(solana-keygen pubkey deployer-keypair.json) --url devnet

# 3. Copy the JSON content
cat deployer-keypair.json
# Output: [123,45,67,...]
```

Then in GitHub:

1. Go to your repo → **Settings** → **Secrets and variables** → **Actions**
2. Click **New repository secret**
3. Name: `DEVNET_DEPLOYER_KEYPAIR`
4. Value: paste the full JSON array from step 3
5. Click **Add secret**

Also create a **deployment environment**:

1. **Settings** → **Environments** → **New environment**
2. Name: `devnet`
3. (Optional) Add required reviewers for manual approval before deploy

> Never commit keypair files to git. The `.gitignore` already excludes `*.json` keypairs.

---

## Troubleshooting

### "Airdrop request failed"

```bash
# Rate-limited by the devnet RPC. Solutions:
# 1. Use the web faucet instead (login with GitHub for 10 SOL/day)
open https://faucet.solana.com

# 2. Wait ~24h for rate limit reset

# 3. Try a different RPC
solana airdrop 2 --url https://devnet.helius-rpc.com/?api-key=YOUR_KEY
```

### "Insufficient funds" on deploy

Program deployment requires ~3-5 SOL for rent. Check and top up:

```bash
solana balance                        # check current balance
# Use faucet.solana.com with GitHub login for up to 10 SOL/day
```

### "Account does not exist"

```bash
# The program isn't deployed, or you're on the wrong network
solana config get                     # verify you're on devnet
solana program show <PROGRAM_ID>      # check if program exists
```

### "Transaction simulation failed"

```bash
# Watch real-time program logs to see the actual error
solana logs <PROGRAM_ID>

# In another terminal, run your failing transaction again
# The logs will show the exact instruction that failed and why
```

### Phantom won't connect to localhost

```
# Phantom blocks HTTP by default. Solutions:
# 1. Use HTTPS in dev (Next.js supports this)
# 2. Or allow insecure localhost in Phantom:
#    Settings > Developer Settings > Trust localhost
```

### anchor build fails with "lock file needs update"

```bash
cd contracts
cargo update                          # update Cargo.lock
anchor build                          # retry
```

### CI deploy job fails

1. Check that `DEVNET_DEPLOYER_KEYPAIR` secret is set (see CI/CD section above)
2. Check that the `devnet` environment exists in repo settings
3. Check that the deployer wallet has enough SOL:

```bash
solana balance <DEPLOYER_ADDRESS> --url devnet
```

---

## References

- [solana-claude](https://github.com/solanabr/solana-claude) — AI agent config for Solana development
- [qedgen/solana-skills](https://github.com/qedgen/solana-skills) — Formal verification with Lean 4
- [Anchor](https://www.anchor-lang.com/) — Solana program framework
- [Helius](https://dev.helius.xyz/) — Solana RPC and developer tools
- [Trident](https://github.com/Ackee-Blockchain/trident) — Fuzz testing for Anchor

## License

MIT
