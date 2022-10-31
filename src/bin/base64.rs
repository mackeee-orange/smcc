use anyhow::Result;
use serde_json::{json, Value};
use std::env;

static BLOCKS_PER_FAST_LOOP: usize = 4;
static PAD_BYTE: u8 = b'=';
static ENCODE_TABLE_JSON: &'static str = r#"{"110000": "w", "110001": "x", "110101": "1", "110100": "0", "010100": "U", "010101": "V", "001100": "M", "001101": "N", "011110": "e", "011111": "f", "001001": "J", "001000": "I", "011011": "b", "011010": "a", "000110": "G", "000111": "H", "000011": "D", "000010": "C", "100100": "k", "100101": "l", "111100": "8", "111101": "9", "100010": "i", "100011": "j", "101110": "u", "101111": "v", "111001": "5", "111000": "4", "101011": "r", "101010": "q", "110011": "z", "110010": "y", "010010": "S", "010011": "T", "010111": "X", "010110": "W", "110110": "2", "110111": "3", "011000": "Y", "011001": "Z", "001111": "P", "001110": "O", "011101": "d", "011100": "c", "001010": "K", "001011": "L", "101101": "t", "000000": "A", "000001": "B", "100111": "n", "100110": "m", "000101": "F", "000100": "E", "111111": "/", "111110": "+", "100001": "h", "100000": "g", "010001": "R", "010000": "Q", "101100": "s", "111010": "6", "111011": "7", "101000": "o", "101001": "p", "000000": "="}"#;
static LOW_SIX_BITS: u64 = 0x3F;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let encoded = base64_encode(args[1].as_str())?;
    println!("{encoded}");

    Ok(())
}

fn base64_encode(input: &str) -> Result<String> {
    let encoding_table: Value = serde_json::from_str(ENCODE_TABLE_JSON)?;
    let input_bytes_vec: Vec<String> = input
        .as_bytes()
        .iter()
        .map(|byte| format!("0{:b}", byte))
        .collect();
    let mut input_bytes = input_bytes_vec.join("");
    let rem = input_bytes.len() % 6;
    for _ in 0..rem {
        input_bytes += "0";
    }

    let output = input_bytes
        .chars()
        .collect::<Vec<_>>()
        .chunks(6)
        .map(|byte_str_vec| {
            let key: String = byte_str_vec.iter().collect();
            format!("{}", encoding_table[key])
        })
        .collect::<String>();
    println!("{:?}", output);
    // .collect::<Vec<String>>()
    //     .join("");


    Ok("hoge".to_string())
}
//
// fn encoding_len(bytes_len: usize) -> Option<usize> {
//     let rem = bytes_len % 3;
//
//     let complete_input_chunks = bytes_len / 3;
//     let complete_chunk_output = complete_input_chunks.checked_mul(4);
//
//     if rem > 0 {
//         complete_chunk_output.and_then(|c| c.checked_add(4))
//     } else {
//         complete_chunk_output
//     }
// }
//
// fn add_padding(input_len: usize, output: &mut [u8]) {
//     let rem = input_len % 3;
//     for (i, _) in (0..((3 - rem) % 3)).enumerate() {
//         output[i] = PAD_BYTE;
//     }
// }
