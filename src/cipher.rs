use std::fs::File;
use std::io::{BufReader, BufWriter, Error};
use std::io::prelude::*;
use aes_ctr::Aes256Ctr;
use aes_ctr::stream_cipher::generic_array::{GenericArray};
use aes_ctr::stream_cipher::{
    NewStreamCipher, SyncStreamCipher, SyncStreamCipherSeek
};
use sha2::{Sha256, Digest};

pub fn encrypt(input_path: &str, output_path: &str, raw_key: &str) -> Result<(), Error> {
    let key = Sha256::digest(raw_key.as_bytes());
    let nouce = GenericArray::from_slice(b"F6477DDFF97D263D");
    let mut cipher = Aes256Ctr::new(&key, &nouce);
    let file = File::open(input_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = vec![];
    match buf_reader.read_to_end(&mut contents) {
        Ok(_) => println!("Content loaded.."),
        Err(x) => {
            println!("Erorr");
            return Err(x);
        }
    }
    let mut block = contents;
    cipher.apply_keystream(&mut block);
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);
    writer.write(&block)?;
    println!("File successfully encrypted");
    Ok(())
}

pub fn decrypt(input_path: &str, output_path: &str, raw_key: &str) -> Result<(), Error> {
    let input_file = File::open(&input_path)?;
    let mut reader = BufReader::new(input_file);
    let mut encrypted_content = vec![];
    reader.read_to_end(&mut encrypted_content)?;
    let key = Sha256::digest(raw_key.as_bytes());
    let nouce = GenericArray::from_slice(b"F6477DDFF97D263D");
    let mut cipher = Aes256Ctr::new(&key, &nouce);
    cipher.seek(0);
    cipher.apply_keystream(&mut encrypted_content);
    let output_file = File::create(&output_path)?;
    let mut writer = BufWriter::new(output_file);
    writer.write(&encrypted_content)?;
    println!("File successfully decrypted");
    Ok(())

}