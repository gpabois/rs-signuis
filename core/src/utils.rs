use base64::Engine;
use rand::RngCore;


/// Génère un jeton cryptographique suffisamment robuste pour
/// être utilisé comme secret de session par exemple.
/// 
/// # Exemple 
/// ```
/// let token = generate_token(16);
/// ```
pub fn generate_token(byte_size: usize) -> String {
    let mut buf = Vec::<u8>::with_capacity(byte_size);
    rand::thread_rng().fill_bytes(&mut buf);   
    base64::engine::general_purpose::STANDARD.encode(&buf)
}