import { TransactionInstruction, PublicKey, AccountMeta, SystemProgram } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"
import { SolCustody } from "../accounts/SolCustody"

export interface WithdrawCustodySolArgs {
  amountLamports: BN
  owner: PublicKey
}

export interface WithdrawCustodySolAccounts {
  /** [SolCustody].\ */
  solCustody: PublicKey
  /** The owner of the sol custody. */
  owner: PublicKey
}

export const layout = borsh.struct([borsh.u64("amountLamports")])

export function withdrawCustodySol(
  args: WithdrawCustodySolArgs,
  programId: PublicKey = PROGRAM_ID
) {

  const solCustody = SolCustody.getAddress(args.owner, programId)
  const accounts: WithdrawCustodySolAccounts = {
    solCustody: solCustody,
    owner: args.owner,
  }
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.solCustody, isSigner: false, isWritable: true },
    { pubkey: accounts.owner, isSigner: true, isWritable: true },
    { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([118, 134, 84, 39, 59, 124, 217, 154])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      amountLamports: args.amountLamports,
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).slice(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
