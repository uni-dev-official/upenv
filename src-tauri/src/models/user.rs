use serde::{Deserialize, Serialize};

/// Authenticated user, mirrored from Supabase Auth.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
}
