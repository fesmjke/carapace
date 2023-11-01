use crate::lcg::LCG;
use crate::md5::MD5;
use crate::utils::unique;
use crate::Module::RC5;
use std::env::args;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

mod lcg;
mod md5;
mod rc5;
mod utils;

enum Module {
    LCG(u64, u64, u64, u64),
    MD5(String),
    RC5(String, String, String, String),
}

struct Config {
    module: Module,
    num: usize,
    unique: bool,
}

impl Config {
    pub fn new() -> Self {
        Self {
            module: Module::LCG(0, 0, 0, 0),
            num: 0,
            unique: false,
        }
    }
    pub fn set_num(&mut self, num: usize) {
        self.num = num;
    }
    pub fn set_unique(&mut self, unique: bool) {
        self.unique = unique;
    }
    pub fn set_module(&mut self, module: Module) {
        self.module = module;
    }
}

fn parse_args(args: &Vec<String>) -> Config {
    let mut config = Config::new();

    for (index, arg) in args.iter().enumerate() {
        // TODO: Refactor parsing
        match arg.as_str() {
            "-lcg" => {
                let modulus = args[index + 1].split("^").collect::<Vec<&str>>();

                let mut base = 0;
                let mut power = 0;

                if modulus.len() == 2 {
                    base = modulus[0]
                        .parse::<u64>()
                        .expect("Unable to parse base of modulus");
                    power = modulus[1]
                        .parse::<u32>()
                        .expect("Unable to parse power of modulus");
                } else {
                    base = modulus[0]
                        .parse::<u64>()
                        .expect("Unable to parse base of modulus");
                    power = 1;
                }

                // TODO: remove -1 if its necessary
                let modulus = base.pow(power) - 1;

                let multiplier = args[index + 2].split("^").collect::<Vec<&str>>();

                if multiplier.len() == 2 {
                    base = multiplier[0]
                        .parse::<u64>()
                        .expect("Unable to parse base of multiplier");
                    power = multiplier[1]
                        .parse::<u32>()
                        .expect("Unable to parse power of multiplier");
                } else {
                    base = multiplier[0]
                        .parse::<u64>()
                        .expect("Unable to parse base of multiplier");
                    power = 1;
                }

                let multiplier = base.pow(power);

                let increment = args[index + 3]
                    .parse::<u64>()
                    .expect("Unable to parse increment parameter");
                let seed = args[index + 4]
                    .parse::<u64>()
                    .expect("Unable to parse seed parameter");

                config.set_module(Module::LCG(modulus, multiplier, increment, seed));
            }
            "-n" => {
                let num = args[index + 1]
                    .parse::<usize>()
                    .expect("Unable parse number of generated elements");

                config.set_num(num);
            }
            "-u" => {
                config.set_unique(true);
            }
            "-m" => {}
            "-md5" => {
                config.set_unique(false);
                config.set_num(0);

                let input = args[index + 1]
                    .parse::<String>()
                    .expect("Unable to read string input to hash");

                config.set_module(Module::MD5(String::from("")));
            }
            "-r" => {
                let input = args[index + 1]
                    .parse::<String>()
                    .expect("Unable to read string input to hash");

                config.set_module(Module::MD5(input));
            }
            "-f" => {
                let file = args[index + 1]
                    .parse::<String>()
                    .expect("Unable to read file path");

                let file = File::open(file).expect("Unable to read input file");
                let mut buf_reader = BufReader::new(file);

                let mut contents = String::new();
                buf_reader
                    .read_to_string(&mut contents)
                    .expect("Unable to read file content");

                config.set_module(Module::MD5(contents));
            }
            "-c" => {
                // -md5 -c "raw-file.txt" "hashed-file.txt"

                // Reading raw-file to make from it a hash
                let raw_file = args[index + 1]
                    .parse::<String>()
                    .expect("Unable to read file path");

                let file = File::open(raw_file).expect("Unable to read raw file");
                let mut buf_reader = BufReader::new(file);

                let mut raw_contents = String::new();
                buf_reader
                    .read_to_string(&mut raw_contents)
                    .expect("Unable to read raw file content");

                let raw_file_hash = MD5::from(raw_contents.as_str());

                // Reading raw-file to make from it a hash
                let hashed_file = args[index + 2]
                    .parse::<String>()
                    .expect("Unable to read file path");

                let file = File::open(hashed_file).expect("Unable to read file that contains hash");
                let mut buf_reader = BufReader::new(file);

                let mut hashed_contents = String::new();
                buf_reader
                    .read_to_string(&mut hashed_contents)
                    .expect("Unable to read file with hash");

                if raw_file_hash == hashed_contents {
                    println!(
                        "\x1b[32mFile {} content is equal to hash that contains in file {}\x1b[0m",
                        args[index + 1],
                        args[index + 2]
                    );
                } else {
                    println!(
                        "\x1b[31mFile {} content is not equal to hash that contains in file {}\x1b[0m",
                        args[index + 1],
                        args[index + 2]
                    );
                }

                config.set_module(Module::MD5(raw_contents));
            }
            "-rc5" => {
                let mode = args[index + 1]
                    .parse::<String>()
                    .expect("Unable to read rc5 mode");
                let cipher = args[index + 2]
                    .parse::<String>()
                    .expect("Unable to read rc5 cipher mode");

                let file_path = match args[index + 3].parse::<String>() {
                    Ok(file) => file,
                    Err(_) => String::from(""),
                };

                // file

                let mut contents = String::new();

                let path = Path::new(file_path.as_str());

                if path.exists() {
                    let file = File::open(file_path).expect("Unable to read input file");

                    let mut buf_reader = BufReader::new(file);

                    buf_reader
                        .read_to_string(&mut contents)
                        .expect("Unable to read file content");
                }

                let mut input = String::new();

                if contents != "" {
                    input = contents;
                } else {
                    input = args[index + 3]
                        .parse::<String>()
                        .expect("Unable to read raw input!");
                }

                let key = args[index + 4]
                    .parse::<String>()
                    .expect("Unable to read key input!");

                config.set_module(RC5(mode, cipher, input, key));
            }
            _ => {
                // panic!("Unexpected flag: '{}'", arg.as_str())
            }
        }
    }

    config
}

// lcg - cargo run -- -lcg 2^11 3^5 1 4 -u -n 100000 > nums.txt -> 88 unique of 2047
// lcg - cargo run -- -lcg 65538 75 74 0 -u -n 100000 > nums.txt -> 65000+ unique
// md5 - cargo run -- -md5 "" -> input raw
// md5 - cargo run -- -md5 file.txt >> hash.txt -> input file
// md5 - cargo run -- -md5 file.txt hash.txt -> compare raw and hash
// rc5 - cargo run -- -rc5 -ecb encrypt/decrypt "test" key > file.txt
// rc5 - cargo run -- -rc5 -cbc encrypt/decrypt plain.txt key > ciphertext.txt
// rc5 - cargo run -- -rc5 -cbc_md5 encrypt/decrypt plain.txt key > ciphertext.txt

fn main() {
    let args = args().into_iter().collect::<Vec<String>>();

    let config = parse_args(&args);

    match config.module {
        Module::LCG(modular, multiplier, increment, seed) => {
            let mut lcg = LCG::new(modular, multiplier, increment, seed);

            let mut nums = Vec::<u64>::with_capacity(config.num);

            // TODO: add threads for big input num

            for _ in 0..config.num {
                let number = lcg.next().expect("Failed generate next number");
                nums.push(number);
                print!("{} ", number);
            }

            println!();

            if config.unique {
                println!("Number of unique elements - {}", unique(&nums));
            }
        }
        Module::MD5(input) => {
            let hash = md5::MD5::from(input.as_str());

            print!("{}", hash);
        }
        Module::RC5(mode, cipher_mode, input, key) => {
            // println!("{mode} {cipher_mode} {input} {key}");
            let flag = match mode.as_str() {
                "-ecb" => rc5::Flags::ECB,
                "-cbc" => rc5::Flags::CBC,
                "-cbc_md5" => rc5::Flags::CBC_MD5,
                _ => rc5::Flags::ECB,
            };

            let rc5 = rc5::RC5::<u32>::new(12, 16, flag);

            match cipher_mode.as_str() {
                "encrypt" => {
                    let ciphertext = rc5.encrypt(input.as_bytes(), key.as_bytes());

                    print!("{}", String::from_utf8_lossy(&ciphertext[..]));
                }
                "decrypt" => {
                    let plaintext = rc5.decrypt(input.as_bytes(), key.as_bytes());

                    print!("{}", String::from_utf8_lossy(&plaintext[..]));
                }
                _ => {
                    panic!("Cannot handle a '{}' mode", cipher_mode);
                }
            }
        }
    }
}
