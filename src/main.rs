use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use aes_ctr::Aes256Ctr;
use aes_ctr::stream_cipher::generic_array::{GenericArray};
use aes_ctr::stream_cipher::{
    NewStreamCipher, SyncStreamCipher, SyncStreamCipherSeek
};
use clap::{Arg, App};
use sha2::{Sha256, Digest};
fn main() {
    let matches = App::new("Encrypter")
                     .version("0.1")
                     .author("Pontakorn Paesaeng <pontakorn61@gmail.com>")
                     .about("Encrypt and decrypt file")
                     .arg(Arg::with_name("mode")
                       .short("m")
                       .long("mode")
                       .value_name("MODE")
                       .help("Set mode")
                       .takes_value(true)
                    )
                      .arg(Arg::with_name("INPUT")
                        .help("Set input file")
                        .required(true)
                        .index(1)
                    )
                      .arg(Arg::with_name("OUTPUT")
                        .help("Set output file")
                        .required(true)
                        .index(2)
                    )
                    .get_matches();

    let key = Sha256::digest(b"J");
    let nouce = GenericArray::from_slice(b"F6477DDFF97D263D");
    let mut cipher = Aes256Ctr::new(&key, &nouce);
    let file = File::open(matches.value_of("INPUT").unwrap()).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = vec![];
    match buf_reader.read_to_end(&mut contents) {
        Ok(_) => println!("Content loaded.."),
        Err(_) => {
            println!("Erorr");
            return;
        }
    }
    let mut block = contents;
    cipher.apply_keystream(&mut block);
    let output_file = File::create(matches.value_of("OUTPUT").unwrap()).unwrap();
    let mut writer = BufWriter::new(output_file);
    writer.write(&block).unwrap();
    let output_file_again = File::open("./out/text_out.txt").unwrap();
    let mut reader_again = BufReader::new(output_file_again);
    let mut encrypted_contents = vec![];
    reader_again.read_to_end(&mut encrypted_contents).unwrap();
    cipher.seek(0);
    cipher.apply_keystream(&mut encrypted_contents);
    println!("Lol decrypt");
    println!("{}", "File successfully decrypted" );
}
