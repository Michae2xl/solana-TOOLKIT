/**
 * Example: Connect Anchor IDL to a React/Next.js frontend.
 *
 * After `anchor build`, the IDL is generated at:
 *   target/idl/my_program.json
 *
 * Copy it to your frontend:
 *   cp target/idl/my_program.json frontend/src/idl/
 *
 * This hook gives you a typed Program instance to call instructions.
 */

import { useMemo } from "react";
import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { AnchorProvider, Program } from "@coral-xyz/anchor";
import type { MyProgram } from "../idl/my_program"; // generated types
import idl from "../idl/my_program.json";

// Program ID — must match Anchor.toml and declare_id!()
const PROGRAM_ID = "YOUR_PROGRAM_ID_HERE";

/**
 * Hook: returns a typed Anchor Program instance.
 *
 * Usage:
 *   const program = useProgram();
 *   const vault = await program.account.vault.fetch(vaultPda);
 *   await program.methods.deposit(new BN(1000)).accounts({...}).rpc();
 */
export function useProgram() {
  const { connection } = useConnection();
  const wallet = useWallet();

  return useMemo(() => {
    if (!wallet.publicKey) return null;

    const provider = new AnchorProvider(connection, wallet as never, {
      commitment: "confirmed",
    });

    return new Program<MyProgram>(idl as never, provider);
  }, [connection, wallet]);
}
