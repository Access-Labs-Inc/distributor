use anchor_lang::error_code;

/// Error codes.
#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient unlocked tokens")]
    InsufficientUnlockedTokens,
    #[msg("Invalid Merkle proof.")]
    InvalidProof,
    #[msg("Exceeded maximum claim amount")]
    ExceededMaxClaim,
    #[msg("Exceeded maximum node count")]
    MaxNodesExceeded,
    #[msg("Account is not authorized to execute this instruction")]
    Unauthorized,
    #[msg("Token account owner did not match intended owner")]
    OwnerMismatch,
    #[msg("Clawback already claimed")]
    ClawbackAlreadyClaimed,
    #[msg("New and old admin are identical")]
    SameAdmin,
    #[msg("Claim window expired")]
    ClaimExpired,
    #[msg("Arithmetic Error (overflow/underflow)")]
    ArithmeticError,
    #[msg("Start Timestamp cannot be after end Timestamp")]
    StartTimestampAfterEnd,
    #[msg("Timestamps cannot be in the past")]
    TimestampsNotInFuture,
}
