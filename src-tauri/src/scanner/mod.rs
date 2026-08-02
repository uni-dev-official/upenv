//! Scanner: read-only collectors. Each file inspects exactly one aspect of
//! the local machine and returns a serde-friendly struct. Scanners must
//! NEVER read or transmit secrets (see utils::redact and top-level Security
//! requirements in the spec).
pub mod applications;
pub mod brew;
pub mod docker;
pub mod git;
pub mod node;
pub mod python;
pub mod snapshot;
pub mod system;
pub mod vscode;
