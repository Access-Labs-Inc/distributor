import { PublicKey, SystemProgram, TransactionInstruction } from "@solana/web3.js"
import BN from "bn.js"
import { PROGRAM_ID } from "../programId"
import { TOKEN_PROGRAM_ID } from "@solana/spl-token"
import { getAssociatedTokenAddressSync } from "@solana/spl-token"
import { newClaimRaw, claimLockedRaw } from "../raw_instructions"
import { MerkleDistributor } from "../accounts"

/**
 * Creates claim instructions for both unlocked and locked tokens.
 * Returns both instructions if both amounts are > 0, otherwise returns the relevant one.
 */
export function newClaim(
    amountUnlocked: BN,
    amountLocked: BN,
    proof: Array<Array<number>>,
    distributor: PublicKey,
    claimant: PublicKey,
    mint: PublicKey,
    programId: PublicKey = PROGRAM_ID,
    tokenProgram: PublicKey = TOKEN_PROGRAM_ID,
    systemProgram: PublicKey = SystemProgram.programId,
) { // Get claim status PDA
    const claimStatus = PublicKey.findProgramAddressSync(
        [
            Buffer.from("claim_status"),
            distributor.toBuffer(),
            claimant.toBuffer(),
        ],
        programId
    )[0]

    // Get distributor's token vault
    const from = getAssociatedTokenAddressSync(mint, distributor)

    // Get claimant's token account
    const to = getAssociatedTokenAddressSync(mint, claimant)

    const instructions: TransactionInstruction[] = []

    // Add unlocked claim instruction if amount > 0
    if (amountUnlocked.gt(new BN(0))) {
        instructions.push(
            newClaimRaw(
                {
                    amountUnlocked,
                    amountLocked,
                    proof,
                },
                {
                    distributor,
                    claimStatus,
                    from,
                    to,
                    claimant,
                    tokenProgram,
                    systemProgram,
                },
                programId
            )
        )
    }

    // Add locked claim instruction if amount > 0
    if (amountLocked.gt(new BN(0))) {
        instructions.push(
            claimLockedRaw(
                {
                    distributor: distributor,
                    claimStatus,
                    from,
                    to,
                    claimant: claimant,
                    tokenProgram,
                },
                programId
            )
        )
    }

    return instructions
}
