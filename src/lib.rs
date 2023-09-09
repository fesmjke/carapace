mod lcg;
mod utils;

#[cfg(test)]
mod core {
    use super::*;

    mod lcg {
        use crate::lcg::LCG;

        #[test]
        fn lcg_lecture() {
            let modulus = 1 << 5; // 32
            let multiplier = 7;
            let increment = 0;
            let seed = 1;

            let mut lcg = LCG::new(modulus, multiplier, increment, seed);
            let mut nums = Vec::<i32>::with_capacity(32);

            for _ in 0..5 {
                let num = lcg.next().expect("Failed generate next number!");

                nums.push(num);
            }

            assert_eq!(vec![7, 17, 23, 1, 7], nums);
        }
        #[test]
        fn lcg_own() {
            let modulus = (1 << 11) - 1; // 2047
            let multiplier = 3_i32.pow(5); // 243
            let increment = 1;
            let seed = 4;

            let mut lcg = LCG::new(modulus, multiplier, increment, seed);

            let mut nums = Vec::<i32>::with_capacity(1000);
            for _ in 0..1000 {
                let num = lcg.next().expect("Failed generate next number!");

                nums.push(num);
            }

            println!("{:?}", nums);

            assert_eq!(nums.len(), 1000);
        }
    }
}
