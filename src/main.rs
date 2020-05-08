use clap::{Arg, App};
use rpassword::read_password_from_tty;
mod cipher;
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
                       .required(true)
                       .possible_values(&["encrypt", "decrypt"])
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
    let mut key = read_password_from_tty(Some(&"Please enter the key : ")).unwrap();
    key = key.trim().to_string();
    println!("Your key is {}",key);
    let mode = matches.value_of("mode").unwrap();
    let input_path = matches.value_of("INPUT").unwrap();
    let output_path = matches.value_of("OUTPUT").unwrap();
    match mode {
        "encrypt" => cipher::encrypt(&input_path, &output_path, &key).unwrap(),
        "decrypt" => cipher::decrypt(&input_path, &output_path, &key).unwrap(),
        _ => panic!("Wrong mode")
    }
    
}
