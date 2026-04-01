# SharkTank — Solana Production Toolkit

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

### Deployed Contracts

**Solana Devnet:**
| Program | Program ID |
|---------|-----------|
| claw_shark (Launch Vault) | `JB5KCYi96KaeoLhcyGmi9sEo6wQs4PFbGaPA4Xh1tKQq` |
| claw_shark_curve (Bonding Curve) | `8AzK5ZoKFNALWq7Hx29e3UTsfxfXzDAaHbbg7sdTB8X6` |
| Authority | `8HXNYtEzNwhGFjZbr5rSY6iDLVH6cUc2iKnvK3mf4df8` |

---

## Devnet → Mainnet Roadmap

### PHASE 1: Foundation ✅ DONE

| Item                           | Status | Notes                                       |
| ------------------------------ | ------ | ------------------------------------------- |
| Anchor programs compile        | ✅     | claw-shark + claw-shark-curve               |
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

## File Locations

```
shark/
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
│   │   ├── claw-shark/           # Launch Vault program
│   │   └── claw-shark-curve/     # Bonding Curve program
│   └── trident-tests/            # Fuzz testing
├── frontend/
│   ├── e2e/                      # Playwright E2E tests
│   └── playwright.config.ts
├── docs/
│   └── SOLANA-TOOLKIT.md         # This file
└── CLAUDE.md                     # AI dev config (solana-claude)
```
