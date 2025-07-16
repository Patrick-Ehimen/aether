/// @title Utility Functions Module
/// @notice Provides utility functions for timestamp retrieval and SHA-256 hashing.
/// @dev Uses `ring` crate for cryptographic operations and standard library for time functions.
/// @module utils
use ring::digest::{Context, SHA256};
use std::time::{SystemTime, UNIX_EPOCH};

/// @notice Returns the current timestamp in milliseconds since the Unix epoch.
/// @dev Panics if system time is before the Unix epoch.
/// @return The current timestamp as an `i64` representing milliseconds since 1970-01-01 00:00:00 UTC.
pub fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as i64
}

/// @notice Computes the SHA-256 digest of the provided data.
/// @dev Uses the `ring` crate's SHA-256 implementation.
/// @param data The input byte slice to hash.
/// @return A `Vec<u8>` containing the SHA-256 hash of the input data.
pub fn sha256_digest(data: &[u8]) -> Vec<u8> {
    let mut context = Context::new(&SHA256);
    context.update(data);
    let digest = context.finish();
    digest.as_ref().to_vec()
}
