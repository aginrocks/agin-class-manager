use sha2::{Digest, Sha256};

pub fn hash_pat(pat: &str) -> String {
    format!("{:x}", Sha256::digest(pat))
}
