use std::io;

fn main() {

    // READS THE MESSAGE
    println!("Go ahead, type the message you wish to encrypt: ");
    let mut message = read_user_input();
    println!("The messasge you typed is:  {}", message);

    // READS THE KEY
    println!("Now, type a key to encrypt the message above (Note: the key must be of the same length as the message): ");
    let mut key = read_user_input();
    println!("The key you typed is:  {}", key);

    // ENCRYPTS THE MESSAGE
    let mut ciphertext = encrypt(&mut key, &mut message);
    println!("The message has been encrypted. The resulting ciphertext is: {}", ciphertext);

    // DECRYPTS THE MESSAGE
    let decrypted_message = decrypt(&mut key, &mut ciphertext);
    println!("The message has been decrypted. It says: {}", decrypted_message);
}

// FUNCTION 0. READS USER INPUT
fn read_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    input
}

// FUNCTION 1. CONVERTS TEXT TO BYTES
fn text_to_bytes(text: &mut String) -> &[u8] {
    text.as_bytes()
}

// FUNCTION 2. PERFORMS XOR BETWEEN TWO ARRAYS OF BYTES
fn apply_xor(key_bytes: &[u8], text_bytes: &[u8]) -> Vec<u8> {
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
    let ciphertext_bytes = apply_xor(key_bytes, message_bytes);
    let ciphertext = bytes_to_text(ciphertext_bytes);
    ciphertext
}

// FUNCTION 5: DECRYPTION FUNCTION
fn decrypt(key: &mut String, ciphertext: &mut String) -> String {
    let ciphertext_bytes = text_to_bytes(ciphertext);
    let key_bytes = text_to_bytes(key);
    let message_bytes = apply_xor(key_bytes, ciphertext_bytes);
    let message = bytes_to_text(message_bytes);
    message
}