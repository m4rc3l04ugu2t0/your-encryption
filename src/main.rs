// Função para converter uma string em bits
fn string_to_bits(input: &str) -> String {
    input
        .as_bytes() // Converte a string para bytes
        .iter() // Itera sobre os bytes
        .map(|byte| format!("{:08b}", byte)) // Converte cada byte para 8 bits (binário)
        .collect::<Vec<String>>() // Coleta o resultado em um vetor de strings
        .join("") // Junta todos os bits em uma única string
}

// Função simples de compressão de bits
// (Aqui podemos aplicar operações matemáticas mais complexas depois)
fn compress_bits(bits: &str, compressed_length: usize) -> String {
    let compressed_bits: String = bits
        .chars()
        .enumerate() // Enumera para pegar o índice
        .filter(|(i, _)| i % 2 == 0) // Exemplo: Pega apenas caracteres em posições pares
        .map(|(_, bit)| bit) // Extrai o bit
        .collect();

    compressed_bits[0..compressed_length.min(compressed_bits.len())].to_string()
}

// Função para converter os bits comprimidos em uma senha (alfabética ou numérica)
fn bits_to_password(bits: &str) -> String {
    bits.chars()
        .map(|bit| if bit == '0' { 'a' } else { 'b' }) // Exemplo: transforma '0' em 'a', '1' em 'b'
        .collect()
}

fn main() {
    let senha = "minha_senha_muito_longa";

    // Converter a senha para bits
    let bits = string_to_bits(senha);
    println!("Bits da senha: {}", bits);

    // Comprimir os bits (por exemplo, pegando apenas posições pares)
    let senha_comprimida_bits = compress_bits(&bits, 16); // Comprimir para 16 bits (pode ser ajustado)
    println!("Bits comprimidos: {}", senha_comprimida_bits);

    // Converter a senha comprimida para algo legível (alfabética)
    let senha_final = bits_to_password(&senha_comprimida_bits);
    println!("Senha final: {}", senha_final);
}
