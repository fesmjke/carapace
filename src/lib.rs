mod lcg;
mod md5;
mod utils;

#[cfg(test)]
mod core {
    mod lcg {
        use crate::lcg::LCG;
        use crate::utils::unique;

        #[test]
        fn lcg_lecture() {
            let modulus = 1 << 5; // 32
            let multiplier = 7;
            let increment = 0;
            let seed = 1;

            let mut lcg = LCG::new(modulus, multiplier, increment, seed);
            let mut nums = Vec::<u64>::with_capacity(32);

            for _ in 0..5 {
                let num = lcg.next().expect("Failed generate next number!");

                nums.push(num);
            }

            assert_eq!(vec![7, 17, 23, 1, 7], nums);
            assert_eq!(unique(&nums), 4);
        }
        #[test]
        fn lcg_own() {
            let modulus = (1 << 11) - 1; // 2047
            let multiplier = 3_u64.pow(5); // 243
            let increment = 1;
            let seed = 4;

            let mut lcg = LCG::new(modulus, multiplier, increment, seed);

            let mut nums = Vec::<u64>::with_capacity(1000);
            for _ in 0..1000 {
                let num = lcg.next().expect("Failed generate next number!");

                nums.push(num);
            }

            println!("{:?}", nums);
            println!("{}", unique(&nums));

            assert_eq!(nums.len(), 1000);
            assert!(unique(&nums) >= 50);
        }
    }
    mod md5 {
        use crate::md5::MD5;

        #[test]
        fn empty_string() {
            assert_eq!(MD5::from(""), "D41D8CD98F00B204E9800998ECF8427E");
        }

        #[test]
        fn single_letter() {
            assert_eq!(MD5::from("a"), "0CC175B9C0F1B6A831C399E269772661");
        }

        #[test]
        fn abc_letters() {
            assert_eq!(MD5::from("abc"), "900150983CD24FB0D6963F7D28E17F72");
        }

        #[test]
        fn long_message() {
            assert_eq!(
                MD5::from("message digest"),
                "F96B697D7CB7938D525A2F31AAF161D0"
            );
        }

        #[test]
        fn alphabet_message() {
            assert_eq!(
                MD5::from("abcdefghijklmnopqrstuvwxyz"),
                "C3FCD3D76192E4007DFB496CCA67E13B"
            );
        }

        #[test]
        fn letters_numbers_message() {
            assert_eq!(
                MD5::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"),
                "D174AB98D277D9F5A5611C2C9F419D9F"
            );
        }

        #[test]
        fn repeated_numbers() {
            assert_eq!(
                MD5::from("12345678901234567890123456789012345678901234567890123456789012345678901234567890"),
                "57EDF4A22BE3C955AC49DA2E2107B67A"
            );
        }
    }
}
