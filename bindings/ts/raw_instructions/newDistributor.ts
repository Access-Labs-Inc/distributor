import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface NewDistributorArgs {
  version: BN
  root: Array<number>
  maxTotalClaim: BN
  maxNumNodes: BN
  startVestingTs: BN
  endVestingTs: BN
}

export interface NewDistributorAccounts {
  /** [MerkleDistributor]. */
  distributor: PublicKey
  /** The mint to distribute. */
  mint: PublicKey
  /** Token vault */
  tokenVault: PublicKey
  /**
   * Creator wallet, responsible for creating the distributor and paying for the transaction.
   * Also is set as the admin
   */
  creator: PublicKey
  /** The [System] program. */
  systemProgram: PublicKey
  /** The [Associated Token] program. */
  associatedTokenProgram: PublicKey
  /** The [Token] program. */
  tokenProgram: PublicKey
}

export const layout = borsh.struct([
  borsh.u64("version"),
  borsh.array(borsh.u8(), 32, "root"),
  borsh.u64("maxTotalClaim"),
  borsh.u64("maxNumNodes"),
  borsh.i64("startVestingTs"),
  borsh.i64("endVestingTs"),
])

/**
 * Creates a new MerkleDistributor.
 * After creating this MerkleDistributor, the token_vault should be seeded with max_total_claim tokens.
 */
export function newDistributor(
  args: NewDistributorArgs,
  accounts: NewDistributorAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.distributor, isSigner: false, isWritable: true },
    { pubkey: accounts.mint, isSigner: false, isWritable: false },
    { pubkey: accounts.tokenVault, isSigner: false, isWritable: false },
    { pubkey: accounts.creator, isSigner: true, isWritable: true },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
    {
      pubkey: accounts.associatedTokenProgram,
      isSigner: false,
      isWritable: false,
    },
    { pubkey: accounts.tokenProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([32, 139, 112, 171, 0, 2, 225, 155])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      version: args.version,
      root: args.root,
      maxTotalClaim: args.maxTotalClaim,
      maxNumNodes: args.maxNumNodes,
      startVestingTs: args.startVestingTs,
      endVestingTs: args.endVestingTs,
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).slice(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
