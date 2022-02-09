//! Macros

/// Generates the signer seeds for a [crate::YiToken].
#[macro_export]
macro_rules! yitoken_seeds {
    ($yitoken: expr) => {
        &[&[
            b"YiToken" as &[u8],
            &$yitoken.mint.to_bytes(),
            &[$yitoken.bump],
        ]]
    };
}
