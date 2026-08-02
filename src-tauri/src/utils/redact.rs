//! Guards against accidentally uploading secrets. Any file content headed
//! for a snapshot MUST pass through `is_safe_to_upload` first.
//!
//! Per the spec, Restorely must NEVER upload: passwords, browser passwords,
//! SSH private keys, API keys, tokens, Keychain contents, or environment
//! variables containing secrets. This module is the single choke point for
//! that guarantee — scanners should call it before including any file text
//! in a Snapshot, rather than each scanner re-implementing its own checks.

const SECRET_PATTERNS: &[&str] = &[
    "PRIVATE KEY",
    "BEGIN RSA",
    "BEGIN OPENSSH",
    "AKIA", // AWS access key prefix
    "api_key",
    "apikey",
    "secret",
    "token",
    "password",
];

/// Returns false if the given text looks like it contains a secret.
pub fn is_safe_to_upload(content: &str) -> bool {
    let lower = content.to_lowercase();
    !SECRET_PATTERNS
        .iter()
        .any(|pattern| lower.contains(&pattern.to_lowercase()))
}

/// Filenames that must never have their contents read/uploaded, regardless
/// of content scanning above (defense in depth).
pub const FORBIDDEN_FILENAMES: &[&str] =
    &["id_rsa", "id_ed25519", "id_ecdsa", ".env", "credentials"];

pub fn is_forbidden_filename(name: &str) -> bool {
    FORBIDDEN_FILENAMES
        .iter()
        .any(|forbidden| name.eq_ignore_ascii_case(forbidden))
}
