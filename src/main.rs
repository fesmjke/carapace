use crate::lcg::LCG;
use crate::utils::unique;
use std::env::args;

mod lcg;
mod md5;
mod utils;

enum Module {
    LCG(u64, u64, u64, u64),
    MD5(String),
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

                let input = args[index + 1]
                    .parse::<String>()
                    .expect("Unable parse number of generated elements");

                config.set_num(0);

                config.set_module(Module::MD5(input));
            }
            _ => {
                // println!("Unexpected flag: '{}'", arg.as_str())
            }
        }
    }

    config
}

// cargo run -- -lcg 2^11 3^5 1 4 -u -n 100000 > nums.txt -> 88 unique of 2047
// cargo run -- -lcg 65538 75 74 0 -u -n 100000 > nums.txt -> 65000+ unique
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

            println!("{}", hash);
        }
    }
}
