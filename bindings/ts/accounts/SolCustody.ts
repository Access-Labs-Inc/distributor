import { PublicKey, Connection } from "@solana/web3.js"
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface SolCustodyFields {}

/** State for the account which distributes tokens. */
export class SolCustody {
  static readonly getAddress = (owner: PublicKey, programId: PublicKey = PROGRAM_ID) => {
    return PublicKey.findProgramAddressSync([Buffer.from("SolCustody"), owner.toBuffer()], programId)[0]
  }
}
