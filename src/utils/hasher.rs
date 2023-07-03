pub use bcrypt::verify;
use bcrypt::{hash, BcryptResult, DEFAULT_COST};

pub fn hash_password(raw_password: &str) -> BcryptResult<String> {
    hash(raw_password, DEFAULT_COST)
}
