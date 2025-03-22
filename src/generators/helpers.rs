use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use rand::{Rng, seq::SliceRandom};
use std::iter;
use uuid::Uuid;

/// Generic function to generate a collection of items
pub(crate) fn generate_items<T, F>(count: usize, mut generator_fn: F) -> Vec<T>
where
    F: FnMut(usize) -> T,
{
    (0..count).map(|i| generator_fn(i)).collect()
}

/// Generate a random password
pub(crate) fn generate_password(rng: &mut impl Rng, length: usize) -> String {
    iter::repeat_with(|| (rng.gen_range(0..26) + b'a') as char)
        .take(length)
        .collect()
}

/// Hash a password using Argon2
pub(crate) fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

/// Choose a random element from a slice of UUIDs
pub(crate) fn choose_random_id(rng: &mut impl Rng, ids: &[Uuid]) -> Uuid {
    *ids.choose(rng).expect("Cannot choose from empty list")
}
