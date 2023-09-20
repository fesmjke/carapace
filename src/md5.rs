#![allow(non_snake_case)]
static BUFFER: [u32; 4] = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476];

fn elements_table() -> [u32; 64] {
    let mut table = [0; 64];
    for i in 1..=64 {
        let x = i as f64;
        let sin_eval = x.sin().abs();
        table[i - 1] = ((2_u64.pow(32)) as f64 * sin_eval) as u32;
    }

    table
}

pub struct MD5;

impl MD5 {
    pub fn from(input: &str) -> String {
        // Step 1 Append Padding Bits
        let mut raw_bytes = input.bytes().collect::<Vec<_>>();

        let bits_length = raw_bytes.len().saturating_mul(8) as u64;

        // 128_u8 is the equivalent of padding 1 as an unsigned 8-bit
        raw_bytes.push(128_u8);

        while (raw_bytes.len() * 8) % 512 != 448 {
            raw_bytes.push(0_u8);
        }

        // Step 2 Append Length  (64 bit integer)
        for i in 0..8 {
            raw_bytes.push((bits_length >> (i * 8)) as u8);
        }

        // Step 3 Initialize MD Buffer
        let mut A = BUFFER[0];
        let mut B = BUFFER[1];
        let mut C = BUFFER[2];
        let mut D = BUFFER[3];

        // Step 4 Process Message in 16-Word Blocks
        let F = |X: u32, Y: u32, Z: u32| -> u32 { X & Y | !X & Z };
        let G = |X: u32, Y: u32, Z: u32| -> u32 { X & Z | Y & !Z };
        let H = |X: u32, Y: u32, Z: u32| -> u32 { X ^ Y ^ Z };
        let I = |X: u32, Y: u32, Z: u32| -> u32 { Y ^ (X | !Z) };

        let table = elements_table();

        // Step 5 Process blocks
        for mut block in raw_bytes.chunks_exact_mut(64) {
            let mut X = unsafe { std::mem::transmute::<&mut [u8], &mut [u32]>(block) };
            #[cfg(target_endian = "big")]
            for i in 0..16 {
                X[i] = X[i].swap_bytes();
            }

            let AA = A;
            let BB = B;
            let CC = C;
            let DD = D;

            macro_rules! compress {
                ($a:ident,$b:ident,$c:ident,$d:ident,$F:expr,$k:expr,$s:expr,$i:expr) => {
                    $a = $b.wrapping_add(
                        ($a.wrapping_add($F($b, $c, $d))
                            .wrapping_add(X[$k])
                            .wrapping_add(table[$i]))
                        .rotate_left($s),
                    )
                };
            }

            // Round 1. -> fF table[0..15], x[p1 * i]
            // fH = (buffer[B] & buffer[C]) | (!buffer[B] & buffer[D])
            // p1 = i
            // shift = (i11 = 7, i12 = 12, i13 = 17, i14 = 22)

            compress!(A, B, C, D, F, 0, 7, 0);
            compress!(D, A, B, C, F, 1, 12, 1);
            compress!(C, D, A, B, F, 2, 17, 2);
            compress!(B, C, D, A, F, 3, 22, 3);

            compress!(A, B, C, D, F, 4, 7, 4);
            compress!(D, A, B, C, F, 5, 12, 5);
            compress!(C, D, A, B, F, 6, 17, 6);
            compress!(B, C, D, A, F, 7, 22, 7);

            compress!(A, B, C, D, F, 8, 7, 8);
            compress!(D, A, B, C, F, 9, 12, 9);
            compress!(C, D, A, B, F, 10, 17, 10);
            compress!(B, C, D, A, F, 11, 22, 11);

            compress!(A, B, C, D, F, 12, 7, 12);
            compress!(D, A, B, C, F, 13, 12, 13);
            compress!(C, D, A, B, F, 14, 17, 14);
            compress!(B, C, D, A, F, 15, 22, 15);

            // Round 2. -> fG table[16..31], x[p2 * i]
            // p2 = (1 + 5i) % 16
            // shift = (i21 = 5, i22 = 9, i23 = 14, i24 = 20)
            // fG = (buffer[B] & buffer[D]) | (buffer[C] & !buffer[D])

            compress!(A, B, C, D, G, 1, 5, 16);
            compress!(D, A, B, C, G, 6, 9, 17);
            compress!(C, D, A, B, G, 11, 14, 18);
            compress!(B, C, D, A, G, 0, 20, 19);

            compress!(A, B, C, D, G, 5, 5, 20);
            compress!(D, A, B, C, G, 10, 9, 21);
            compress!(C, D, A, B, G, 15, 14, 22);
            compress!(B, C, D, A, G, 4, 20, 23);

            compress!(A, B, C, D, G, 9, 5, 24);
            compress!(D, A, B, C, G, 14, 9, 25);
            compress!(C, D, A, B, G, 3, 14, 26);
            compress!(B, C, D, A, G, 8, 20, 27);

            compress!(A, B, C, D, G, 13, 5, 28);
            compress!(D, A, B, C, G, 2, 9, 29);
            compress!(C, D, A, B, G, 7, 14, 30);
            compress!(B, C, D, A, G, 12, 20, 31);

            // Round 3. -> fH table[32..47], x[p3 * i]
            // p3 = (5 + 3i) % 16
            // shift = (i31 = 4, i32 = 11, i33 = 16, i34 = 23)
            // fH = buffer[B] ^ buffer[C] ^ buffer[D]

            compress!(A, B, C, D, H, 5, 4, 32);
            compress!(D, A, B, C, H, 8, 11, 33);
            compress!(C, D, A, B, H, 11, 16, 34);
            compress!(B, C, D, A, H, 14, 23, 35);

            compress!(A, B, C, D, H, 1, 4, 36);
            compress!(D, A, B, C, H, 4, 11, 37);
            compress!(C, D, A, B, H, 7, 16, 38);
            compress!(B, C, D, A, H, 10, 23, 39);

            compress!(A, B, C, D, H, 13, 4, 40);
            compress!(D, A, B, C, H, 0, 11, 41);
            compress!(C, D, A, B, H, 3, 16, 42);
            compress!(B, C, D, A, H, 6, 23, 43);

            compress!(A, B, C, D, H, 9, 4, 44);
            compress!(D, A, B, C, H, 12, 11, 45);
            compress!(C, D, A, B, H, 15, 16, 46);
            compress!(B, C, D, A, H, 2, 23, 47);

            // Round 4. -> fG table[48..63], x[p4 * i]
            // p4 = 7i % 16
            // shift = (i41 = 6, i42 = 10, i43 = 15, i44 = 21)
            // fG = buffer[C] ^ (buffer[B] | !buffer[D])

            compress!(A, B, C, D, I, 0, 6, 48);
            compress!(D, A, B, C, I, 7, 10, 49);
            compress!(C, D, A, B, I, 14, 15, 50);
            compress!(B, C, D, A, I, 5, 21, 51);

            compress!(A, B, C, D, I, 12, 6, 52);
            compress!(D, A, B, C, I, 3, 10, 53);
            compress!(C, D, A, B, I, 10, 15, 54);
            compress!(B, C, D, A, I, 1, 21, 55);

            compress!(A, B, C, D, I, 8, 6, 56);
            compress!(D, A, B, C, I, 15, 10, 57);
            compress!(C, D, A, B, I, 6, 15, 58);
            compress!(B, C, D, A, I, 13, 21, 59);

            compress!(A, B, C, D, I, 4, 6, 60);
            compress!(D, A, B, C, I, 11, 10, 61);
            compress!(C, D, A, B, I, 2, 15, 62);
            compress!(B, C, D, A, I, 9, 21, 63);

            A = A.wrapping_add(AA);
            B = B.wrapping_add(BB);
            C = C.wrapping_add(CC);
            D = D.wrapping_add(DD);
        }

        format!(
            "{:08X}{:08X}{:08X}{:08X}",
            A.swap_bytes(),
            B.swap_bytes(),
            C.swap_bytes(),
            D.swap_bytes()
        )
    }
}
