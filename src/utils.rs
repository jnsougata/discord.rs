use ed25519_dalek::{VerifyingKey, Signature, Verifier};



fn bytes_fromhex(input: &str) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    
    let mut chars = input.chars();
    while let (Some(a), Some(b)) = (chars.next(), chars.next()) {
        let byte = u8::from_str_radix(&format!("{}{}", a, b), 16).unwrap();
        result.push(byte);
    }
    
    result
}

pub(crate) fn verify_signature(
    signature: &str,
    public_key: &str,
    body: &[u8],
    timestamp: &[u8],
) -> bool {
    let mut message = Vec::new();
    message.extend_from_slice(timestamp);
    message.extend_from_slice(&body);

    let pk_bytes = bytes_fromhex(public_key);
    let signature_bytes = bytes_fromhex(signature);


    let mut pk_bytes_slice: [u8; 32] = [0; 32];
    let mut signature_bytes_slice: [u8; 64] = [0; 64];

    for (i, byte) in pk_bytes.iter().enumerate() {
        pk_bytes_slice[i] = *byte;
    }

    for (i, byte) in signature_bytes.iter().enumerate() {
        signature_bytes_slice[i] = *byte;
    }

    let vk = VerifyingKey::from_bytes(&pk_bytes_slice).unwrap();
    let signature = Signature::from_bytes(&signature_bytes_slice);

    let result = vk.verify(&message, &signature);

    result.is_ok()
    
}