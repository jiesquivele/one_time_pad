use std::io;

fn main() {

    let mut message = String::new();
    println!("Go ahead, type the message you wish to encrypt:");
    io::stdin()
        .read_line(&mut message)
        .expect("Failed to read message.");
    println!("The message is: {}", message);

    let mut key = String::new();
    println!("Now, type a key to encrypt the message above (Note: key must be of the same length as the message):");
    io::stdin()
        .read_line(&mut key)
        .expect("Failed to read key.");
    println!("The key is: {}", key);

    let mut ciphertext= encrypt(&mut key, &mut message);
    println!("The message has been encrypted. The resulting ciphertext is: {}", ciphertext);

    let decrypted_message = decrypt(&mut key, &mut ciphertext);
    println!("The message has been decrypted. It says: {}", decrypted_message);
}

// FUNCTION 1. CONVERTS TEXT TO BYTES
fn text_to_bytes(text: &mut String) -> &[u8] {
    text.as_bytes()
}

// FUNCTION 2. PERFORMS XOR BETWEEN TWO ARRAYS OF BYTES
fn apply_xor(key_bytes: &[u8],text_bytes: &[u8]) -> Vec<u8> {
    let mut total_values_xor: Vec<u8> = Vec::new();
    for element in 0..key_bytes.len() {
        let individual_value_xor = key_bytes[element] ^ text_bytes[element];
        total_values_xor.push(individual_value_xor);
    }
    return total_values_xor;
}

// FUNCTION 3. CONVERTS BYTES TO TEXT
fn bytes_to_text(text: Vec<u8>) -> String {
    let text = String::from_utf8(Vec::from(text)).unwrap();
    return text;
}

// FUNCTION 4: ENCRYPTION FUNCTION
fn encrypt(key: &mut String, message: &mut String) -> String {
    let message_bytes = text_to_bytes(message);
    let key_bytes = text_to_bytes(key);
    let ciphertext_bytes = apply_xor(key_bytes,message_bytes);
    let ciphertext = bytes_to_text(ciphertext_bytes);
    ciphertext
}

// FUNCTION 5: DECRYPTION FUNCTION
fn decrypt(key: &mut String, ciphertext: &mut String) -> String {
    let ciphertext_bytes = text_to_bytes(ciphertext);
    let key_bytes = text_to_bytes(key);
    let message_bytes = apply_xor(key_bytes,ciphertext_bytes);
    let message = bytes_to_text(message_bytes);
    message
}