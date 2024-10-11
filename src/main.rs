use rand::{thread_rng, Rng};

// Função para gerar uma flag aleatória
fn generate_random_flag(length: usize) -> String {
    let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .chars()
        .collect();
    let mut rng = thread_rng();
    (0..length)
        .map(|_| charset[rng.gen_range(0..charset.len())])
        .collect()
}

// Função para converter uma string em binário
fn string_to_binary(input: &str) -> Vec<u8> {
    input.as_bytes().to_vec() // Converte a string em bytes
}

// Função para recuperar a senha original usando a flag
fn recover_password(final_binary: Vec<u8>, flag_binary: Vec<u8>) -> String {
    let recovered_bytes: Vec<u8> = final_binary
        .iter()
        .zip(flag_binary.iter())
        .map(|(&final_byte, &flag_byte)| final_byte ^ flag_byte)
        .collect();

    String::from_utf8_lossy(&recovered_bytes).into_owned() // Retorna a senha recuperada
}

// Função para converter bytes em uma senha alfanumérica
fn bytes_to_password(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|&byte| {
            // Gera um caractere alfabético a partir do byte
            let char_index = byte % 36; // 26 letras + 10 dígitos
            if char_index < 26 {
                (b'a' + char_index) as char // Letras 'a' a 'z'
            } else {
                (b'0' + (char_index - 26)) as char // Dígitos '0' a '9'
            }
        })
        .collect()
}

fn main() {
    let senha = "minha_senha_muito_longa";

    // Gerar uma flag aleatória com o mesmo tamanho da senha
    let flag = generate_random_flag(senha.len()); // Tamanho da flag deve ser o mesmo que a senha
    println!("Flag gerada: {}", flag);

    // Converter a senha e a flag em binário
    let senha_binary = string_to_binary(senha);
    let flag_binary = string_to_binary(&flag);

    // Aplicar a operação XOR para gerar a senha final em binário
    let final_binary: Vec<u8> = senha_binary
        .iter()
        .zip(flag_binary.iter())
        .map(|(&senha_byte, &flag_byte)| senha_byte ^ flag_byte)
        .collect();

    // Converter os bytes da senha final em uma string alfanumérica
    let final_password_string = bytes_to_password(&final_binary);
    println!("Senha final: {}", final_password_string);

    // Recuperar a senha original
    let recovered_password = recover_password(final_binary.clone(), flag_binary);
    println!("Senha recuperada: {}", recovered_password);
}
