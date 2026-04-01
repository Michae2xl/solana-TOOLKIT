# Solana Production Toolkit — Detailed Roadmap

> Complete guide from scaffold to mainnet. Updated 2026-03-31.

## Toolkit Status

### CLI Tools Installed

| Tool         | Version | Purpose                                | Status       |
| ------------ | ------- | -------------------------------------- | ------------ |
| Solana CLI   | 3.1.12  | Deploy, wallets, on-chain interactions | ✅ Installed |
| Anchor       | 0.31.1  | Build, test, deploy Anchor programs    | ✅ Installed |
| Rust         | 1.94.1  | Compile Solana programs                | ✅ Installed |
| Lean 4       | 4.29.0  | Theorem prover for formal verification | ✅ Installed |
| qedgen       | 1.2.0   | Formal proofs for Solana contracts     | ✅ Installed |
| cargo-audit  | 0.22.1  | Dependency vulnerability scanner       | ✅ Installed |
| cargo-expand | 1.0.121 | Macro expansion viewer                 | ✅ Installed |
| Node.js      | 25.8.1  | Frontend / scripts                     | ✅ Installed |

### solana-claude Config (v1.2.3)

| Component    | Count | Purpose                                                           |
| ------------ | ----- | ----------------------------------------------------------------- |
| Agents       | 15    | Specialized dev agents (architect, anchor, defi, qa, etc.)        |
| Commands     | 24    | /build-program, /audit-solana, /deploy, /profile-cu, etc.         |
| Rules        | 7     | Auto-loading (Anchor, Rust, Pinocchio, TypeScript, .NET)          |
| MCP Servers  | 6     | Helius, solana-dev, Context7, Playwright, context-mode, memsearch |
| Skills (ext) | 9     | Solana Foundation, SendAI, Trail of Bits, QEDGen, etc.            |

---

## Getting Started: Zero to Devnet

### Step 0: Scaffold a New Project

```bash
# Option A — Anchor only (contracts)
anchor init my-project
cd my-project

# Option B — Full-stack dApp (Next.js + Anchor)
npx create-solana-dapp my-dapp
cd my-dapp
```

After scaffolding, install the toolkit into your project:

```bash
curl -fsSL https://raw.githubusercontent.com/solanabr/solana-claude/main/install.sh | bash
cp path/to/solana-TOOLKIT/.github/workflows/* .github/workflows/
cp path/to/solana-TOOLKIT/.env.example .env
```

### Step 1: Configure Solana CLI for Devnet

```bash
# Set devnet as default
solana config set --url https://api.devnet.solana.com

# Create a wallet (if you don't have one)
solana-keygen new --no-passphrase

# Check your address
solana address
```

### Step 2: Get Devnet SOL

SOL is required for transaction fees and rent on devnet.

**Option A — Solana Faucet (recommended, up to 10 SOL/day with GitHub login):**

1. Go to [faucet.solana.com](https://faucet.solana.com)
2. **Login with GitHub** to unlock the higher limit (up to **10 SOL per day**)
   - Without login: limited to ~2 SOL/day and rate-limited
3. Select **Devnet**
4. Paste your wallet address
5. Request SOL

**Option B — CLI airdrop (quick, but rate-limited):**

```bash
solana airdrop 2          # Request 2 SOL (max per request)
solana airdrop 2          # Repeat if needed
solana balance            # Verify balance
```

> CLI airdrops are rate-limited (~2 SOL/24h). If you hit the limit, use the web faucet with GitHub login.

**Option C — Alternative faucets:**

- [solfaucet.com](https://solfaucet.com)
- [QuickNode faucet](https://faucet.quicknode.com/solana/devnet)

### Step 3: Get Devnet USDC

USDC is needed for testing token transfers, x402 payments, and DeFi flows.

1. Go to [faucet.circle.com](https://faucet.circle.com)
2. Select **USDC** on **Solana**
3. Select **Devnet**
4. Paste your wallet address
5. Claim up to **10 USDC**

USDC Devnet Mint: `4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU`

### Step 4: Build & Test Contracts

```bash
cd contracts/

# Build Anchor programs
anchor build

# Run unit + integration tests
anchor test

# Fuzz testing (10 min session)
trident fuzz run --timeout 600

# Security lints
cargo clippy -- -D warnings -W clippy::unwrap_used -W clippy::arithmetic_side_effects
cargo audit
```

### Step 5: Deploy to Devnet

```bash
cd contracts/

# Ensure you have enough SOL (>= 5 SOL recommended for program deployment)
solana balance

# Deploy programs
anchor deploy --provider.cluster devnet

# Verify deployment
solana program show <PROGRAM_ID>
```

After deployment, update program IDs in:

- `Anchor.toml`
- `programs/*/src/lib.rs` (in `declare_id!()`)
- `.env` files (frontend/backend)

### Step 6: Setup Phantom Wallet for Devnet (Frontend Testing)

To test your dApp frontend, Phantom needs to be on devnet:

1. Open **Phantom** wallet (browser extension)
2. Click the **hamburger menu** (top-left) → **Settings**
3. **Developer Settings** → enable **Testnet Mode**
4. Select **Solana Devnet**
5. Your wallet now shows devnet balances and signs devnet transactions

> After getting SOL/USDC from faucets (steps 2-3), they appear in Phantom on devnet.

### Cost Summary (Devnet Testing)

| Item                 | Cost                                      |
| -------------------- | ----------------------------------------- |
| SOL (faucet)         | Free — up to 10 SOL/day with GitHub login |
| USDC (Circle faucet) | Free — up to 10 USDC per request          |
| Program deployment   | Free (devnet SOL)                         |
| RPC calls            | Free (public devnet endpoint)             |

---

## Devnet → Mainnet Roadmap

### PHASE 1: Foundation ✅ DONE

| Item                           | Status | Notes                                       |
| ------------------------------ | ------ | ------------------------------------------- |
| Anchor programs compile        | ✅     | Anchor programs compiled                    |
| Checked arithmetic (no unwrap) | ✅     | All unwrap() replaced with ok_or(ErrorCode) |
| Frontend functional (27 pages) | ✅     | Next.js 15, SSE, wallet connect             |
| Backend API                    | ✅     | Hono, LLM integration, x402                 |
| Dev tooling installed          | ✅     | solana-claude, qedgen, cargo-audit          |
| Devnet deploy                  | ✅     | Both programs deployed                      |

### PHASE 2: Hardening ⏳ IN PROGRESS

| Item                         | Status | How                                     | Cost                     |
| ---------------------------- | ------ | --------------------------------------- | ------------------------ |
| CI/CD pipeline               | ✅     | GitHub Actions (ci.yml + security.yml)  | Free                     |
| E2E tests (Playwright)       | ✅     | 3 critical flow tests                   | Free                     |
| Fuzz testing (Trident)       | ✅     | 10h+ fuzzing on contracts               | Free                     |
| Formal verification (qedgen) | ⏳     | `qedgen verify` — needs MISTRAL_API_KEY | Free (Mistral free tier) |
| Verifiable build             | ⏳     | `anchor build --verifiable`             | Free                     |
| cargo-audit clean            | ✅     | Zero vulnerabilities                    | Free                     |

### PHASE 3: Infrastructure 🔜

| Item                    | Status | How                                 | Cost                 |
| ----------------------- | ------ | ----------------------------------- | -------------------- |
| RPC dedicado            | ❌     | Helius paid plan                    | $50-200/mo           |
| RPC fallback            | ❌     | 2+ providers with failover          | Included in RPC cost |
| Priority fees dinâmicos | ❌     | Helius priority fee API             | Included in Helius   |
| PostgreSQL              | ❌     | Supabase free tier or Railway       | $0-20/mo             |
| Redis                   | ❌     | Upstash free tier                   | $0/mo                |
| Indexer                 | ❌     | Helius webhooks for on-chain events | Included in Helius   |
| WebSocket reliability   | ❌     | Auto-reconnect, heartbeat, backoff  | Free (code change)   |
| Rate limiting           | ❌     | Backend middleware                  | Free (code change)   |

### PHASE 4: Monitoring 🔜

| Item                    | Status | How                                     | Cost             |
| ----------------------- | ------ | --------------------------------------- | ---------------- |
| Error tracking          | ❌     | Sentry (free tier: 5K events/mo)        | $0/mo            |
| Uptime monitoring       | ❌     | UptimeRobot or Better Stack (free tier) | $0/mo            |
| On-chain monitoring     | ❌     | Helius webhooks + custom alerts         | $0 (with Helius) |
| TX monitoring dashboard | ❌     | Custom Grafana or Helius dashboard      | $0-20/mo         |
| Log aggregation         | ❌     | Axiom free tier (500MB/mo)              | $0/mo            |

### PHASE 5: Frontend Production 🔜

| Item                                               | Status | How                                     | Cost  |
| -------------------------------------------------- | ------ | --------------------------------------- | ----- |
| Wallet adapter oficial                             | ❌     | Migrate to @solana/wallet-adapter-react | Free  |
| Multi-wallet (Phantom, Solflare, Backpack, Ledger) | ❌     | wallet-adapter handles this             | Free  |
| TX confirmation UX                                 | ❌     | Loading states, retry, error recovery   | Free  |
| Mobile responsive (E2E tested)                     | ❌     | Playwright mobile viewport tests        | Free  |
| SEO / Open Graph                                   | ❌     | Meta tags, Twitter cards                | Free  |
| Analytics                                          | ❌     | Plausible (self-host) or PostHog        | $0/mo |

### PHASE 6: Security Audit 💰

| Item                        | Status | How                                | Cost         |
| --------------------------- | ------ | ---------------------------------- | ------------ |
| Professional audit          | ❌     | OtterSec, Neodyme, Zellic, Halborn | $30K-80K     |
| Bug bounty program          | ❌     | Immunefi                           | $5K-50K pool |
| Verifiable builds published | ❌     | anchor verify on mainnet           | Free         |
| Security documentation      | ❌     | Threat model, incident response    | Free         |

### PHASE 7: Mainnet Launch 💰

| Item                       | Status | How                                | Cost        |
| -------------------------- | ------ | ---------------------------------- | ----------- |
| Multisig upgrade authority | ❌     | Squads Protocol (Solana multisig)  | Free        |
| Gradual rollout            | ❌     | Beta users → public, feature flags | Free        |
| Monitoring 24/7            | ❌     | PagerDuty or OpsGenie              | $0-20/mo    |
| Incident response plan     | ❌     | Documented playbook                | Free        |
| Terms of Service           | ❌     | Legal template                     | Free-$2K    |
| Privacy Policy             | ❌     | LGPD/GDPR compliance               | Free-$2K    |
| Risk disclaimers           | ❌     | DeFi risk warnings                 | Free        |
| Geo-blocking               | ❌     | Block restricted jurisdictions     | Free (code) |

---

## Cost Summary

### Free (covered by toolkit)

- CI/CD (GitHub Actions free tier)
- E2E testing (Playwright)
- Fuzz testing (Trident)
- Formal verification (qedgen + Mistral free tier)
- Security linting (cargo-audit, clippy)
- Error tracking (Sentry free)
- Uptime monitoring (free tier)
- Analytics (Plausible self-host)
- Wallet adapter migration
- All code improvements

### Monthly costs (production)

| Item                        | Min     | Max      |
| --------------------------- | ------- | -------- |
| RPC (Helius)                | $50     | $200     |
| Database (Supabase/Railway) | $0      | $20      |
| Hosting (Vercel/Railway)    | $0      | $50      |
| Monitoring (Sentry + Axiom) | $0      | $50      |
| **Monthly total**           | **$50** | **$320** |

### One-time costs

| Item               | Min         | Max          |
| ------------------ | ----------- | ------------ |
| Professional audit | $30,000     | $80,000      |
| Bug bounty pool    | $5,000      | $50,000      |
| Legal docs         | $0          | $4,000       |
| **One-time total** | **$35,000** | **$134,000** |

---

## Quick Reference: Common Commands

```bash
# Build
anchor build                          # Compile programs
npx next build                        # Compile frontend

# Test
anchor test                           # Rust tests
npx playwright test                   # E2E tests
trident fuzz run --timeout 600        # Fuzz testing (10min)

# Security
cargo audit                           # Dependency vulnerabilities
cargo clippy -- -D warnings \
  -W clippy::unwrap_used \
  -W clippy::arithmetic_side_effects  # Security lints
qedgen verify --idl target/idl/X.json # Formal verification

# Deploy
anchor deploy --provider.cluster devnet  # Devnet
# Mainnet requires multisig approval — see PHASE 7

# solana-claude commands
/build-program                        # Build with checks
/audit-solana                         # Full security audit
/profile-cu                           # Compute unit optimization
/deploy                               # Deploy with safety gates
/test-and-fix                         # Run tests, auto-fix failures
/diff-review                          # Review changes for AI slop
```

---

## What Tools DON'T Cover

These require human judgment, money, or external services:

### Professional Audit ($30K-80K)

No tool replaces a human security auditor for mainnet financial programs. qedgen provides formal proofs, cargo-audit catches known CVEs, clippy catches patterns — but a professional auditor catches business logic flaws, economic attacks, and novel exploits.

**When to get audited:** After all fuzz tests pass, formal verification is clean, and the codebase is frozen for mainnet.

**Recommended firms:** OtterSec, Neodyme, Zellic, Halborn, Trail of Bits

### Economic Security

Tools can verify arithmetic, but not economic design:

- Is the bonding curve pricing fair?
- Can whales manipulate the market?
- Is the fee structure sustainable?
- Are there flash loan attack vectors?
- Oracle manipulation risks?

**Mitigation:** Economic modeling, simulation, and professional DeFi audit.

### Legal Compliance

No dev tool covers:

- Terms of Service
- Privacy Policy (LGPD/GDPR)
- Securities law compliance (token classification)
- Geo-blocking requirements
- Tax implications

**Mitigation:** Legal counsel familiar with crypto/DeFi in your jurisdiction.

### Social Engineering / Operational Security

Tools protect code, not people:

- Private key management (hardware wallets, multisig)
- Phishing resistance (team training)
- Insider threat (access controls)
- DNS/domain security (DNSSEC, registrar lock)
- Social media account security

**Mitigation:** OpSec practices, hardware wallets, Squads multisig for program authority.

### Infrastructure Reliability

Free tiers have limits:

- RPC rate limits (public endpoints: ~10 req/s)
- Database size limits
- CDN bandwidth limits
- No SLA guarantees

**Mitigation:** Paid infrastructure for production (Helius, Supabase, Vercel Pro).

### Real-time Market Data

For DeFi applications:

- Price feeds (Pyth, Switchboard)
- Liquidity data
- Order book depth
- Cross-chain prices

**Mitigation:** Oracle integration (Pyth Network recommended for Solana).

---

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

## Anchor.toml Configuration

The `Anchor.toml` file is the configuration hub — it maps program names to deployed addresses and sets the default cluster.

```toml
[programs.devnet]
my_program = "YOUR_PROGRAM_ID_HERE"

[programs.mainnet]
my_program = "YOUR_PROGRAM_ID_HERE"

[provider]
cluster = "devnet"
wallet = "~/.config/solana/id.json"

[scripts]
test = "npx ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"
```

### First Deploy Flow

```bash
# 1. Build (generates keypairs in target/deploy/)
anchor build

# 2. Get the generated program ID
solana address -k target/deploy/my_program-keypair.json

# 3. Update Anchor.toml [programs.devnet] with the real ID
# 4. Update declare_id!() in programs/my_program/src/lib.rs
# 5. Rebuild with correct ID
anchor build

# 6. Deploy
anchor deploy --provider.cluster devnet
```

See `examples/anchor/` for a full annotated `Anchor.toml` and multi-program setup.

---

## Program Upgrades

After the first deploy, use `anchor upgrade` (not `anchor deploy`) to update your program without changing its address.

### Upgrade Flow

```bash
# 1. Make code changes
# 2. Build
anchor build

# 3. Upgrade (keeps the same program ID)
anchor upgrade --program-id YOUR_PROGRAM_ID target/deploy/my_program.so --provider.cluster devnet

# 4. Verify
solana program show YOUR_PROGRAM_ID
```

### Deploy vs Upgrade

| Command          | When to use        | What happens                                 |
| ---------------- | ------------------ | -------------------------------------------- |
| `anchor deploy`  | First time         | Creates new program at generated address     |
| `anchor upgrade` | Subsequent changes | Updates existing program, keeps same address |

### Upgrade Authority

The wallet that deployed the program is its **upgrade authority**. Only this wallet can upgrade it.

```bash
# Check who can upgrade a program
solana program show YOUR_PROGRAM_ID

# Transfer authority to another wallet (e.g., multisig)
solana program set-upgrade-authority YOUR_PROGRAM_ID \
  --new-upgrade-authority NEW_AUTHORITY_PUBKEY

# Freeze program (no more upgrades — IRREVERSIBLE)
solana program set-upgrade-authority YOUR_PROGRAM_ID --final
```

> For mainnet: transfer upgrade authority to a **Squads multisig** before launch (see Phase 7).

---

## IDL to Frontend: Connecting Contracts to Your dApp

After deploying, the IDL bridges your Anchor program to the TypeScript frontend.

### Quick Setup

```bash
# 1. Build generates IDL + types
cd contracts/
anchor build
# Output:
#   target/idl/my_program.json       ← IDL (account schemas, instructions)
#   target/types/my_program.ts       ← TypeScript types

# 2. Copy to frontend
mkdir -p frontend/src/idl
cp target/idl/my_program.json frontend/src/idl/
cp target/types/my_program.ts frontend/src/idl/

# 3. Install dependencies
cd frontend/
npm install @coral-xyz/anchor @solana/wallet-adapter-react @solana/web3.js
```

### Use in React

```tsx
import { AnchorProvider, Program, BN } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import idl from "../idl/my_program.json";

// Create program instance
const provider = new AnchorProvider(connection, wallet, {
  commitment: "confirmed",
});
const program = new Program(idl, provider);

// Call an instruction
await program.methods
  .deposit(new BN(1_000_000))
  .accounts({ owner: wallet.publicKey, vault: vaultPda })
  .rpc();

// Fetch account data
const vault = await program.account.vault.fetch(vaultPda);

// Derive a PDA
const [vaultPda] = PublicKey.findProgramAddressSync(
  [Buffer.from("vault"), wallet.publicKey.toBuffer()],
  program.programId,
);
```

### Keep IDL in Sync

Add to your frontend `package.json`:

```json
{
  "scripts": {
    "sync-idl": "cp ../contracts/target/idl/*.json src/idl/ && cp ../contracts/target/types/*.ts src/idl/"
  }
}
```

Then after any contract change: `anchor build && cd frontend && npm run sync-idl`

See `examples/frontend/` for a complete `useProgram` hook and usage patterns.

---

## File Locations

```
your-project/
├── .github/workflows/
│   ├── ci.yml                    # CI/CD pipeline
│   └── security.yml              # Weekly security scan
├── .claude/
│   ├── agents/                   # 15 specialized agents
│   ├── commands/                 # 24 slash commands
│   ├── rules/                    # 7 auto-loading rule sets
│   ├── skills/                   # SKILL.md + ext/ (9 repos)
│   ├── mcp.json                  # 6 MCP server configs
│   └── settings.json             # Permissions, hooks
├── contracts/
│   ├── programs/
│   │   ├── your-program/           # Launch Vault program
│   │   └── your-program-curve/     # Bonding Curve program
│   └── trident-tests/            # Fuzz testing
├── frontend/
│   ├── e2e/                      # Playwright E2E tests
│   └── playwright.config.ts
├── docs/
│   └── SOLANA-TOOLKIT.md         # This file
└── CLAUDE.md                     # AI dev config (solana-claude)
```
