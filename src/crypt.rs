use blake3::hash;
use threefish_cipher::Threefish256;
use block_cipher_trait::BlockCipher;
use generic_array::GenericArray;
use zero85::{ToZ85, FromZ85};

// wrapper for all other crypt functions to be directly used by frontend
pub fn cipher(message: String, password: String, decrypt: bool) -> String {
   
    // crypt
    let output = ecb_crypt(
        if !decrypt {
            split_message(message.as_bytes().to_vec())
        } else {
            split_message(message.from_z85().unwrap())
        },
        *hash(password.as_bytes()).as_bytes(),
    decrypt);
  
    // gen output bytes
    if !decrypt {
        output.to_z85().unwrap()
    } else {
        String::from_utf8(clean_message(output)).unwrap()
    }
}

// split string into byte blocks
fn split_message(message: Vec<u8>) -> Vec<[u8; 32]> {
    let mut blocks: Vec<[u8; 32]> = Vec::new();

    // split into 256 bit blocks 
    let mut index: usize = 0;
    let mut count: usize = 1;
    let mut buffer = [0u8; 32];
    for byte in message {
        buffer[index] = byte;
        if index == 31 {
            blocks.push(buffer); 
            index = 0;
            count += 1;
            buffer = [0u8; 32];
        } else {
            index += 1;
        }
    }
    blocks.push(buffer);
    
    // padding break
    blocks[count - 1][index] = 2;
    blocks
} 

// remove padding from byte array
fn clean_message(input: Vec<u8>) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut content = false;
    for byte in input.iter().rev() {
        if content {
            buffer.push(*byte);
        }
        else if *byte == 2 {
            content = true;
        }
    }
    buffer.reverse();
    buffer
}

// ecb encryption of all blocks
fn ecb_crypt(blocks: Vec<[u8; 32]>, key: [u8; 32], decrypt: bool) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();
    for block in blocks {
        for byte in crypt_block(block, key, decrypt).iter() {
            buffer.push(*byte);
        }
    }
    buffer
}

// encrypt or decrypt 1024 bit block
fn crypt_block(block: [u8; 32], key: [u8; 32], decrypt: bool) -> [u8; 32] {
    let mut buffer = block;
    let cipher = Threefish256::new(GenericArray::from_slice(&key));
    if !decrypt {
        cipher.encrypt_block(GenericArray::from_mut_slice(&mut buffer));
    }
    else {
        cipher.decrypt_block(GenericArray::from_mut_slice(&mut buffer));
    }
    buffer
}