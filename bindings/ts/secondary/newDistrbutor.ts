import { PublicKey, SystemProgram } from "@solana/web3.js"
import BN from "bn.js"
import { PROGRAM_ID } from "../programId"
import { ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token"
import { TOKEN_PROGRAM_ID } from "@solana/spl-token"
import { MerkleDistributor } from "../accounts/MerkleDistributor"
import { getAssociatedTokenAddressSync } from "@solana/spl-token"
import { newDistributorRaw } from "../raw_instructions"

/**
 * Creates a new MerkleDistributor.
 * After creating this MerkleDistributor, the token_vault should be seeded with max_total_claim tokens.
 */
export function newDistributor(
    version: BN,
    root: Array<number>,
    maxTotalClaim: BN,
    maxNumNodes: BN,
    startVestingTs: BN,
    endVestingTs: BN,
    mint: PublicKey,
    creator: PublicKey,
    programId: PublicKey = PROGRAM_ID,
    systemProgram: PublicKey = SystemProgram.programId,
    associatedTokenProgram: PublicKey = ASSOCIATED_TOKEN_PROGRAM_ID,
    tokenProgram: PublicKey = TOKEN_PROGRAM_ID,
) {
    const distributor = MerkleDistributor.getAddress(creator, mint, version, programId)
    const tokenVault = getAssociatedTokenAddressSync(mint, distributor, true)

    return newDistributorRaw(
        { version, root, maxTotalClaim, maxNumNodes, startVestingTs, endVestingTs },
        { distributor, mint, tokenVault, creator, systemProgram, associatedTokenProgram, tokenProgram },
        programId
    )
}
