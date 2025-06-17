import { PublicKey, Connection } from "@solana/web3.js"
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface MerkleDistributorFields {
  /** Bump seed. */
  bump: number
  /** Version of the airdrop */
  version: BN
  /** The 256-bit merkle root. */
  root: Array<number>
  /** [Mint] of the token to be distributed. */
  mint: PublicKey
  /** Token Address of the vault */
  tokenVault: PublicKey
  /** Maximum number of tokens that can ever be claimed from this [MerkleDistributor]. */
  maxTotalClaim: BN
  /** Maximum number of nodes in [MerkleDistributor]. */
  maxNumNodes: BN
  /** Total amount of tokens that have been claimed. */
  totalAmountClaimed: BN
  /** Number of nodes that have been claimed. */
  numNodesClaimed: BN
  /** Lockup time start (Unix Timestamp) */
  startTs: BN
  /** Lockup time end (Unix Timestamp) */
  endTs: BN
  /** Creator wallet */
  creator: PublicKey
  /** Admin wallet */
  admin: PublicKey
  /** Whether or not the distributor has been clawed back */
  clawedBack: boolean
}

export interface MerkleDistributorJSON {
  /** Bump seed. */
  bump: number
  /** Version of the airdrop */
  version: string
  /** The 256-bit merkle root. */
  root: Array<number>
  /** [Mint] of the token to be distributed. */
  mint: string
  /** Token Address of the vault */
  tokenVault: string
  /** Maximum number of tokens that can ever be claimed from this [MerkleDistributor]. */
  maxTotalClaim: string
  /** Maximum number of nodes in [MerkleDistributor]. */
  maxNumNodes: string
  /** Total amount of tokens that have been claimed. */
  totalAmountClaimed: string
  /** Number of nodes that have been claimed. */
  numNodesClaimed: string
  /** Lockup time start (Unix Timestamp) */
  startTs: string
  /** Lockup time end (Unix Timestamp) */
  endTs: string
  /** Creator wallet */
  creator: string
  /** Admin wallet */
  admin: string
  /** Whether or not the distributor has been clawed back */
  clawedBack: boolean
}

/** State for the account which distributes tokens. */
export class MerkleDistributor {
  /** Bump seed. */
  readonly bump: number
  /** Version of the airdrop */
  readonly version: BN
  /** The 256-bit merkle root. */
  readonly root: Array<number>
  /** [Mint] of the token to be distributed. */
  readonly mint: PublicKey
  /** Token Address of the vault */
  readonly tokenVault: PublicKey
  /** Maximum number of tokens that can ever be claimed from this [MerkleDistributor]. */
  readonly maxTotalClaim: BN
  /** Maximum number of nodes in [MerkleDistributor]. */
  readonly maxNumNodes: BN
  /** Total amount of tokens that have been claimed. */
  readonly totalAmountClaimed: BN
  /** Number of nodes that have been claimed. */
  readonly numNodesClaimed: BN
  /** Lockup time start (Unix Timestamp) */
  readonly startTs: BN
  /** Lockup time end (Unix Timestamp) */
  readonly endTs: BN
  /** Creator wallet */
  readonly creator: PublicKey
  /** Admin wallet */
  readonly admin: PublicKey
  /** Whether or not the distributor has been clawed back */
  readonly clawedBack: boolean

  static readonly discriminator = Buffer.from([
    77, 119, 139, 70, 84, 247, 12, 26,
  ])

  static readonly layout = borsh.struct([
    borsh.u8("bump"),
    borsh.u64("version"),
    borsh.array(borsh.u8(), 32, "root"),
    borsh.publicKey("mint"),
    borsh.publicKey("tokenVault"),
    borsh.u64("maxTotalClaim"),
    borsh.u64("maxNumNodes"),
    borsh.u64("totalAmountClaimed"),
    borsh.u64("numNodesClaimed"),
    borsh.i64("startTs"),
    borsh.i64("endTs"),
    borsh.publicKey("creator"),
    borsh.publicKey("admin"),
    borsh.bool("clawedBack"),
  ])

  static readonly getAddress = (creator: PublicKey, mint: PublicKey, version: BN, programId: PublicKey = PROGRAM_ID) => {
    return PublicKey.findProgramAddressSync([Buffer.from("MerkleDistributor"), mint.toBuffer(), creator.toBuffer(), version.toBuffer()], programId)[0]
  }

  constructor(fields: MerkleDistributorFields) {
    this.bump = fields.bump
    this.version = fields.version
    this.root = fields.root
    this.mint = fields.mint
    this.tokenVault = fields.tokenVault
    this.maxTotalClaim = fields.maxTotalClaim
    this.maxNumNodes = fields.maxNumNodes
    this.totalAmountClaimed = fields.totalAmountClaimed
    this.numNodesClaimed = fields.numNodesClaimed
    this.startTs = fields.startTs
    this.endTs = fields.endTs
    this.creator = fields.creator
    this.admin = fields.admin
    this.clawedBack = fields.clawedBack
  }

  static async fetch(
    c: Connection,
    address: PublicKey,
    programId: PublicKey = PROGRAM_ID
  ): Promise<MerkleDistributor | null> {
    const info = await c.getAccountInfo(address)

    if (info === null) {
      return null
    }
    if (!info.owner.equals(programId)) {
      throw new Error("account doesn't belong to this program")
    }

    return this.decode(info.data)
  }

  static async fetchMultiple(
    c: Connection,
    addresses: PublicKey[],
    programId: PublicKey = PROGRAM_ID
  ): Promise<Array<MerkleDistributor | null>> {
    const infos = await c.getMultipleAccountsInfo(addresses)

    return infos.map((info) => {
      if (info === null) {
        return null
      }
      if (!info.owner.equals(programId)) {
        throw new Error("account doesn't belong to this program")
      }

      return this.decode(info.data)
    })
  }

  static decode(data: Buffer): MerkleDistributor {
    if (!data.slice(0, 8).equals(MerkleDistributor.discriminator)) {
      throw new Error("invalid account discriminator")
    }

    const dec = MerkleDistributor.layout.decode(data.slice(8))

    return new MerkleDistributor({
      bump: dec.bump,
      version: dec.version,
      root: dec.root,
      mint: dec.mint,
      tokenVault: dec.tokenVault,
      maxTotalClaim: dec.maxTotalClaim,
      maxNumNodes: dec.maxNumNodes,
      totalAmountClaimed: dec.totalAmountClaimed,
      numNodesClaimed: dec.numNodesClaimed,
      startTs: dec.startTs,
      endTs: dec.endTs,
      creator: dec.creator,
      admin: dec.admin,
      clawedBack: dec.clawedBack,
    })
  }

  toJSON(): MerkleDistributorJSON {
    return {
      bump: this.bump,
      version: this.version.toString(),
      root: this.root,
      mint: this.mint.toString(),
      tokenVault: this.tokenVault.toString(),
      maxTotalClaim: this.maxTotalClaim.toString(),
      maxNumNodes: this.maxNumNodes.toString(),
      totalAmountClaimed: this.totalAmountClaimed.toString(),
      numNodesClaimed: this.numNodesClaimed.toString(),
      startTs: this.startTs.toString(),
      endTs: this.endTs.toString(),
      creator: this.creator.toString(),
      admin: this.admin.toString(),
      clawedBack: this.clawedBack,
    }
  }

  static fromJSON(obj: MerkleDistributorJSON): MerkleDistributor {
    return new MerkleDistributor({
      bump: obj.bump,
      version: new BN(obj.version),
      root: obj.root,
      mint: new PublicKey(obj.mint),
      tokenVault: new PublicKey(obj.tokenVault),
      maxTotalClaim: new BN(obj.maxTotalClaim),
      maxNumNodes: new BN(obj.maxNumNodes),
      totalAmountClaimed: new BN(obj.totalAmountClaimed),
      numNodesClaimed: new BN(obj.numNodesClaimed),
      startTs: new BN(obj.startTs),
      endTs: new BN(obj.endTs),
      creator: new PublicKey(obj.creator),
      admin: new PublicKey(obj.admin),
      clawedBack: obj.clawedBack,
    })
  }
}
