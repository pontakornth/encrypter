use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::str;
use aes_ctr::Aes256Ctr;
use aes_ctr::stream_cipher::generic_array::{GenericArray};
use aes_ctr::stream_cipher::{
    NewStreamCipher, SyncStreamCipher, SyncStreamCipherSeek
};
use sha2::{Sha256, Digest};
fn main() {
    let key = Sha256::digest(b"J");
    let nouce = GenericArray::from_slice(b"F6477DDFF97D263D");
    let mut cipher = Aes256Ctr::new(&key, &nouce);
    let file = File::open("text.txt").unwrap();
    let mut buf_reader =BufReader::new(file);
    let mut contents = String::new();
    match buf_reader.read_to_string(&mut contents) {
        Ok(_) => println!("{}",contents),
        Err(_) => {
            println!("Erorr");
            return;
        }
    }
    let mut block = contents.into_bytes();
    cipher.apply_keystream(&mut block);
    let output_file = File::create("text_out.txt").unwrap();
    let mut writer = BufWriter::new(output_file);
    writer.write(&block).unwrap();

    let output_file_again = File::open("text_out.txt").unwrap();
    let mut reader_again = BufReader::new(output_file_again);
    let mut encrypted_contents = vec![];
    reader_again.read_to_end(&mut encrypted_contents).unwrap();
    cipher.seek(0);
    cipher.apply_keystream(&mut encrypted_contents);
    println!("Lol decrypt");
    println!("{}", str::from_utf8(&encrypted_contents).unwrap() );
}