# Anchor Configuration Example

## Anchor.toml

The `Anchor.toml` file is the configuration hub for your Anchor project. It maps program names to deployed addresses, sets the default cluster, and configures test scripts.

### First-Time Setup

```bash
# 1. Build your programs (generates keypairs in target/deploy/)
anchor build

# 2. Get the generated program ID
solana address -k target/deploy/my_program-keypair.json
# Output: 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU

# 3. Update Anchor.toml with the real program ID
# [programs.devnet]
# my_program = "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU"

# 4. Update declare_id!() in your program's lib.rs to match
# declare_id!("7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU");

# 5. Rebuild with the correct ID
anchor build

# 6. Deploy
anchor deploy --provider.cluster devnet
```

### Switching Between Clusters

```bash
# Deploy to devnet (default if cluster = "devnet" in Anchor.toml)
anchor deploy

# Deploy to mainnet (override)
anchor deploy --provider.cluster mainnet

# Use a specific wallet
anchor deploy --provider.wallet ./deployer-keypair.json
```

### Multiple Programs

```toml
[programs.devnet]
vault_program = "VAULT_PROGRAM_ID"
token_program = "TOKEN_PROGRAM_ID"

[programs.mainnet]
vault_program = "VAULT_MAINNET_ID"
token_program = "TOKEN_MAINNET_ID"
```

### Clone Accounts for Local Testing

Uncomment the `[test.validator]` section to clone accounts from devnet into your local test validator. Useful for testing with real SPL Token mints, USDC, or other programs.
