use sha2::{Digest, Sha256};

fn hash_from_text(text: &str) -> String {
    let hash = Sha256::digest(text.as_bytes());
    format!("{:x}", hash)
}
fn main() {
    let text = "Paolo";
    println!("SHA256 hash of '{}' is {}", text, hash_from_text(text));
}