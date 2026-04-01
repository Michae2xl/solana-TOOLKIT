# Frontend Integration: IDL to TypeScript Client

After deploying your Anchor program, you need to connect it to your frontend. The IDL (Interface Description Language) is the bridge.

## Step-by-Step

### 1. Build and Generate IDL

```bash
cd contracts/
anchor build
# IDL generated at: target/idl/my_program.json
# TypeScript types at: target/types/my_program.ts
```

### 2. Copy IDL to Frontend

```bash
# Create IDL directory in frontend
mkdir -p frontend/src/idl

# Copy IDL and types
cp target/idl/my_program.json frontend/src/idl/
cp target/types/my_program.ts frontend/src/idl/
```

### 3. Install Dependencies

```bash
cd frontend/
npm install @coral-xyz/anchor @solana/wallet-adapter-react @solana/web3.js
```

### 4. Create the Program Hook

See `use-program.ts` in this directory for a ready-to-use React hook.

### 5. Use in Components

```tsx
import { useProgram } from "@/hooks/use-program";
import { useWallet } from "@solana/wallet-adapter-react";
import { PublicKey } from "@solana/web3.js";
import { BN } from "@coral-xyz/anchor";

function DepositButton({ vaultAddress }: { vaultAddress: string }) {
  const program = useProgram();
  const { publicKey } = useWallet();

  const handleDeposit = async () => {
    if (!program || !publicKey) return;

    const vaultPda = new PublicKey(vaultAddress);

    const tx = await program.methods
      .deposit(new BN(1_000_000)) // 0.001 SOL in lamports
      .accounts({
        owner: publicKey,
        vault: vaultPda,
        systemProgram: PublicKey.default,
      })
      .rpc();

    console.log("TX:", tx);
  };

  return <button onClick={handleDeposit}>Deposit 0.001 SOL</button>;
}
```

### 6. Fetch Account Data

```tsx
// Fetch a single account
const vault = await program.account.vault.fetch(vaultPda);
console.log("Balance:", vault.balance.toString());

// Fetch all accounts of a type
const allVaults = await program.account.vault.all();

// Fetch with filters
const myVaults = await program.account.vault.all([
  { memcmp: { offset: 8, bytes: publicKey.toBase58() } },
]);

// Subscribe to changes (real-time)
const listenerId = program.account.vault.subscribe(vaultPda, "confirmed");
program.account.vault.on("change", (account) => {
  console.log("Updated:", account.balance.toString());
});
```

## Keeping IDL in Sync

When you change your program and redeploy:

```bash
cd contracts/
anchor build
anchor deploy --provider.cluster devnet

# Re-copy IDL to frontend
cp target/idl/my_program.json frontend/src/idl/
cp target/types/my_program.ts frontend/src/idl/
```

### Automate with a Script

Add to `package.json`:

```json
{
  "scripts": {
    "sync-idl": "cp ../contracts/target/idl/*.json src/idl/ && cp ../contracts/target/types/*.ts src/idl/"
  }
}
```

Then: `npm run sync-idl`

## Common Patterns

| Pattern          | How                                                                 |
| ---------------- | ------------------------------------------------------------------- |
| Send transaction | `program.methods.myInstruction(args).accounts({...}).rpc()`         |
| Simulate first   | `program.methods.myInstruction(args).accounts({...}).simulate()`    |
| Get transaction  | `program.methods.myInstruction(args).accounts({...}).transaction()` |
| Fetch account    | `program.account.myAccount.fetch(address)`                          |
| Fetch all        | `program.account.myAccount.all()`                                   |
| Subscribe        | `program.account.myAccount.subscribe(address)`                      |
| PDA derivation   | `PublicKey.findProgramAddressSync([seeds], program.programId)`      |
