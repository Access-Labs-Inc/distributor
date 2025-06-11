import typing
from anchorpy.error import ProgramError


class InsufficientUnlockedTokens(ProgramError):
    def __init__(self) -> None:
        super().__init__(6000, "Insufficient unlocked tokens")

    code = 6000
    name = "InsufficientUnlockedTokens"
    msg = "Insufficient unlocked tokens"


class InvalidProof(ProgramError):
    def __init__(self) -> None:
        super().__init__(6001, "Invalid Merkle proof.")

    code = 6001
    name = "InvalidProof"
    msg = "Invalid Merkle proof."


class ExceededMaxClaim(ProgramError):
    def __init__(self) -> None:
        super().__init__(6002, "Exceeded maximum claim amount")

    code = 6002
    name = "ExceededMaxClaim"
    msg = "Exceeded maximum claim amount"


class MaxNodesExceeded(ProgramError):
    def __init__(self) -> None:
        super().__init__(6003, "Exceeded maximum node count")

    code = 6003
    name = "MaxNodesExceeded"
    msg = "Exceeded maximum node count"


class Unauthorized(ProgramError):
    def __init__(self) -> None:
        super().__init__(6004, "Account is not authorized to execute this instruction")

    code = 6004
    name = "Unauthorized"
    msg = "Account is not authorized to execute this instruction"


class OwnerMismatch(ProgramError):
    def __init__(self) -> None:
        super().__init__(6005, "Token account owner did not match intended owner")

    code = 6005
    name = "OwnerMismatch"
    msg = "Token account owner did not match intended owner"


class ClawbackAlreadyClaimed(ProgramError):
    def __init__(self) -> None:
        super().__init__(6006, "Clawback already claimed")

    code = 6006
    name = "ClawbackAlreadyClaimed"
    msg = "Clawback already claimed"


class SameAdmin(ProgramError):
    def __init__(self) -> None:
        super().__init__(6007, "New and old admin are identical")

    code = 6007
    name = "SameAdmin"
    msg = "New and old admin are identical"


class ClaimExpired(ProgramError):
    def __init__(self) -> None:
        super().__init__(6008, "Claim window expired")

    code = 6008
    name = "ClaimExpired"
    msg = "Claim window expired"


class ArithmeticError(ProgramError):
    def __init__(self) -> None:
        super().__init__(6009, "Arithmetic Error (overflow/underflow)")

    code = 6009
    name = "ArithmeticError"
    msg = "Arithmetic Error (overflow/underflow)"


class StartTimestampAfterEnd(ProgramError):
    def __init__(self) -> None:
        super().__init__(6010, "Start Timestamp cannot be after end Timestamp")

    code = 6010
    name = "StartTimestampAfterEnd"
    msg = "Start Timestamp cannot be after end Timestamp"


class TimestampsNotInFuture(ProgramError):
    def __init__(self) -> None:
        super().__init__(6011, "Timestamps cannot be in the past")

    code = 6011
    name = "TimestampsNotInFuture"
    msg = "Timestamps cannot be in the past"


CustomError = typing.Union[
    InsufficientUnlockedTokens,
    InvalidProof,
    ExceededMaxClaim,
    MaxNodesExceeded,
    Unauthorized,
    OwnerMismatch,
    ClawbackAlreadyClaimed,
    SameAdmin,
    ClaimExpired,
    ArithmeticError,
    StartTimestampAfterEnd,
    TimestampsNotInFuture,
]
CUSTOM_ERROR_MAP: dict[int, CustomError] = {
    6000: InsufficientUnlockedTokens(),
    6001: InvalidProof(),
    6002: ExceededMaxClaim(),
    6003: MaxNodesExceeded(),
    6004: Unauthorized(),
    6005: OwnerMismatch(),
    6006: ClawbackAlreadyClaimed(),
    6007: SameAdmin(),
    6008: ClaimExpired(),
    6009: ArithmeticError(),
    6010: StartTimestampAfterEnd(),
    6011: TimestampsNotInFuture(),
}


def from_code(code: int) -> typing.Optional[CustomError]:
    maybe_err = CUSTOM_ERROR_MAP.get(code)
    if maybe_err is None:
        return None
    return maybe_err
