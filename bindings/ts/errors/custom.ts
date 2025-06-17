export type CustomError =
  | InsufficientUnlockedTokens
  | InvalidProof
  | ExceededMaxClaim
  | MaxNodesExceeded
  | Unauthorized
  | OwnerMismatch
  | ClawbackAlreadyClaimed
  | SameAdmin
  | ClaimExpired
  | ArithmeticError
  | StartTimestampAfterEnd
  | TimestampsNotInFuture

export class InsufficientUnlockedTokens extends Error {
  static readonly code = 6000
  readonly code = 6000
  readonly name = "InsufficientUnlockedTokens"
  readonly msg = "Insufficient unlocked tokens"

  constructor(readonly logs?: string[]) {
    super("6000: Insufficient unlocked tokens")
  }
}

export class InvalidProof extends Error {
  static readonly code = 6001
  readonly code = 6001
  readonly name = "InvalidProof"
  readonly msg = "Invalid Merkle proof."

  constructor(readonly logs?: string[]) {
    super("6001: Invalid Merkle proof.")
  }
}

export class ExceededMaxClaim extends Error {
  static readonly code = 6002
  readonly code = 6002
  readonly name = "ExceededMaxClaim"
  readonly msg = "Exceeded maximum claim amount"

  constructor(readonly logs?: string[]) {
    super("6002: Exceeded maximum claim amount")
  }
}

export class MaxNodesExceeded extends Error {
  static readonly code = 6003
  readonly code = 6003
  readonly name = "MaxNodesExceeded"
  readonly msg = "Exceeded maximum node count"

  constructor(readonly logs?: string[]) {
    super("6003: Exceeded maximum node count")
  }
}

export class Unauthorized extends Error {
  static readonly code = 6004
  readonly code = 6004
  readonly name = "Unauthorized"
  readonly msg = "Account is not authorized to execute this instruction"

  constructor(readonly logs?: string[]) {
    super("6004: Account is not authorized to execute this instruction")
  }
}

export class OwnerMismatch extends Error {
  static readonly code = 6005
  readonly code = 6005
  readonly name = "OwnerMismatch"
  readonly msg = "Token account owner did not match intended owner"

  constructor(readonly logs?: string[]) {
    super("6005: Token account owner did not match intended owner")
  }
}

export class ClawbackAlreadyClaimed extends Error {
  static readonly code = 6006
  readonly code = 6006
  readonly name = "ClawbackAlreadyClaimed"
  readonly msg = "Clawback already claimed"

  constructor(readonly logs?: string[]) {
    super("6006: Clawback already claimed")
  }
}

export class SameAdmin extends Error {
  static readonly code = 6007
  readonly code = 6007
  readonly name = "SameAdmin"
  readonly msg = "New and old admin are identical"

  constructor(readonly logs?: string[]) {
    super("6007: New and old admin are identical")
  }
}

export class ClaimExpired extends Error {
  static readonly code = 6008
  readonly code = 6008
  readonly name = "ClaimExpired"
  readonly msg = "Claim window expired"

  constructor(readonly logs?: string[]) {
    super("6008: Claim window expired")
  }
}

export class ArithmeticError extends Error {
  static readonly code = 6009
  readonly code = 6009
  readonly name = "ArithmeticError"
  readonly msg = "Arithmetic Error (overflow/underflow)"

  constructor(readonly logs?: string[]) {
    super("6009: Arithmetic Error (overflow/underflow)")
  }
}

export class StartTimestampAfterEnd extends Error {
  static readonly code = 6010
  readonly code = 6010
  readonly name = "StartTimestampAfterEnd"
  readonly msg = "Start Timestamp cannot be after end Timestamp"

  constructor(readonly logs?: string[]) {
    super("6010: Start Timestamp cannot be after end Timestamp")
  }
}

export class TimestampsNotInFuture extends Error {
  static readonly code = 6011
  readonly code = 6011
  readonly name = "TimestampsNotInFuture"
  readonly msg = "Timestamps cannot be in the past"

  constructor(readonly logs?: string[]) {
    super("6011: Timestamps cannot be in the past")
  }
}

export function fromCode(code: number, logs?: string[]): CustomError | null {
  switch (code) {
    case 6000:
      return new InsufficientUnlockedTokens(logs)
    case 6001:
      return new InvalidProof(logs)
    case 6002:
      return new ExceededMaxClaim(logs)
    case 6003:
      return new MaxNodesExceeded(logs)
    case 6004:
      return new Unauthorized(logs)
    case 6005:
      return new OwnerMismatch(logs)
    case 6006:
      return new ClawbackAlreadyClaimed(logs)
    case 6007:
      return new SameAdmin(logs)
    case 6008:
      return new ClaimExpired(logs)
    case 6009:
      return new ArithmeticError(logs)
    case 6010:
      return new StartTimestampAfterEnd(logs)
    case 6011:
      return new TimestampsNotInFuture(logs)
  }

  return null
}
