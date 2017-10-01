#![crate_name = "consistent_hash"]
#![crate_type = "lib"]

extern crate crypto;
mod consistent_hash;
pub use consistent_hash::ConsistentHash;
