use crate::lcg::LCG;
use crate::utils::unique;
use std::env::args;

mod lcg;
mod utils;

enum Module {
    LCG(i32, i32, i32, i32),
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
                let mut modulus = args[index + 1].split("^");
                let base = modulus
                    .next()
                    .expect("Unable to find base of modulus")
                    .parse::<i32>()
                    .expect("Unable to parse base of modulus");
                let power = modulus
                    .next()
                    .expect("Unable to find power of modulus")
                    .parse::<u32>()
                    .expect("Unable to parse power of modulus");

                // TODO: remove -1 if its necessary
                let modulus = base.pow(power) - 1;

                let mut multiplier = args[index + 2].split("^");
                let base = multiplier
                    .next()
                    .expect("Unable to find base of multiplier")
                    .parse::<i32>()
                    .expect("Unable to parse base of multiplier");
                let power = multiplier
                    .next()
                    .expect("Unable to find power of multiplier")
                    .parse::<u32>()
                    .expect("Unable to parse power of multiplier");

                let multiplier = base.pow(power);

                let increment = args[index + 3]
                    .parse::<i32>()
                    .expect("Unable to parse increment parameter");
                let seed = args[index + 4]
                    .parse::<i32>()
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
            _ => {
                // println!("Unexpected flag: '{}'", arg.as_str())
            }
        }
    }

    config
}

// cargo run -- -lcg 2^11 3^5 1 4 -u -n 100000 >> file.txt
fn main() {
    let args = args().into_iter().collect::<Vec<String>>();

    let config = parse_args(&args);

    match config.module {
        Module::LCG(modular, multiplier, increment, seed) => {
            let mut lcg = LCG::new(modular, multiplier, increment, seed);
            let mut nums = Vec::<i32>::with_capacity(config.num);
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
    }
}
