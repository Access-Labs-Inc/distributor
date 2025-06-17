import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"
import { SystemProgram } from "@solana/web3.js"
import { SolCustody } from "../accounts/SolCustody"

export function stakeCustodySol(
  amountLamports: BN,
  owner: PublicKey,
  programId: PublicKey = PROGRAM_ID
): TransactionInstruction {
  const solCustody = SolCustody.getAddress(owner, programId)
  return SystemProgram.transfer({
    fromPubkey: owner,
    toPubkey: solCustody,
    lamports: amountLamports.toNumber(),
  })
}
