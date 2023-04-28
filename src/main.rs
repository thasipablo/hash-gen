use sha2::{Digest, Sha256};

fn main() {
    let text = "Paolo";
    let hash = Sha256::digest(text.as_bytes());
    println!("SHA256 hash of '{}' is {:x}", text, hash);
}