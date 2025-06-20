import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface ClawbackAccounts {
  /** The [MerkleDistributor]. */
  distributor: PublicKey
  /** Distributor ATA containing the tokens to distribute. */
  from: PublicKey
  /** The Clawback token account. */
  to: PublicKey
  /**
   * Admin account
   * Only admin can claw back
   */
  admin: PublicKey
  /** The [System] program. */
  systemProgram: PublicKey
  /** SPL [Token] program. */
  tokenProgram: PublicKey
}

export function clawback(
  accounts: ClawbackAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.distributor, isSigner: false, isWritable: true },
    { pubkey: accounts.from, isSigner: false, isWritable: true },
    { pubkey: accounts.to, isSigner: false, isWritable: true },
    { pubkey: accounts.admin, isSigner: true, isWritable: true },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
    { pubkey: accounts.tokenProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([111, 92, 142, 79, 33, 234, 82, 27])
  const data = identifier
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
