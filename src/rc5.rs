use crate::lcg::LCG;
use crate::md5::MD5;
use num::traits::{AsPrimitive, WrappingAdd, WrappingSub};
use num::{NumCast, PrimInt};
use std::cmp::max;
use std::fmt::Debug;
use std::mem::size_of;
use std::ops;

pub trait Word: // u16 u32 u64
    Clone
    + Copy
    + Debug
    + WrappingAdd
    + WrappingSub
    + NumCast
    + PrimInt
    + ops::AddAssign
    + ops::Add<Output = Self>
    + ops::Sub<Output = Self>
    + ops::BitXor<Output = Self>
    + ops::Shl<Output = Self>
    + ops::Shr<Output = Self>
{
    const BITS: usize = size_of::<Self>() * 8;
    const ZERO: Self; // default 0
    const P: Self; // Odd((e − 2)2^w)
    const Q: Self; // Odd((φ − 1)2^w)

    fn from_le_bytes(bytes: &[u8]) -> Self;

    fn from_be_bytes(bytes: &[u8]) -> Self;
    fn to_le_bytes(&self) -> Vec<u8>;

    fn to_be_bytes(&self) -> Vec<u8>;
}

macro_rules! impl_word {
    ($typ:tt, $q:expr, $p:expr) => {
        impl Word for $typ {
            const P: $typ = $p;
            const Q: $typ = $q;
            const ZERO: Self = 0;

            fn to_le_bytes(&self) -> Vec<u8> {
                $typ::to_le_bytes(*self).to_vec()
            }

            fn to_be_bytes(&self) -> Vec<u8> {
                $typ::to_be_bytes(*self).to_vec()
            }

            fn from_le_bytes(bytes: &[u8]) -> Self {
                $typ::from_le_bytes(
                    bytes
                        .try_into()
                        .expect("Unable to convert bytes to le bytes"),
                )
            }

            fn from_be_bytes(bytes: &[u8]) -> Self {
                $typ::from_be_bytes(bytes.try_into().expect("Unable to convert to be bytes"))
            }
        }
    };
}

impl_word!(u32, 0x9E3779B9, 0xB7E15163);
impl_word!(u64, 0xB7E151628AED2A6B, 0x9E3779B97F4A7C15);

fn convert<W, U>(y: U) -> W
where
    W: Word + 'static,
    U: AsPrimitive<W>,
{
    y.as_()
}

pub struct RC5<W: Word> {
    word_size: W,
    rounds: usize,
    octets: usize,
    extended_part: usize,
}

impl<W> RC5<W>
where
    W: Word + 'static,
    u64: AsPrimitive<W>,
{
    pub fn new(rounds: usize, octets: usize) -> Self {
        Self {
            word_size: W::ZERO,
            rounds,
            octets,
            extended_part: usize::default(),
        }
    }

    fn key_to_words(&self, key: &[u8]) -> Vec<W> {
        let words_len = max(key.len(), 1) / size_of::<W>();

        let mut words = vec![W::ZERO; words_len];

        for i in (0..key.len()).rev() {
            let word_index = i / size_of::<W>();
            let word = W::from(key[i]).expect("Min word size is 8");

            words[word_index] = words[word_index].rotate_left(8).wrapping_add(&word);
        }

        words
    }

    fn key_expand(&self, key: &[u8]) -> Vec<W> {
        // parse md5
        let hashed_key = MD5::from(String::from_utf8_lossy(key).to_string().as_str());

        let summarize: u128 = hashed_key
            .chars()
            .collect::<Vec<_>>()
            .chunks(8)
            .map(|x| x.iter().collect::<String>())
            .collect::<Vec<_>>()
            .iter()
            .map(|x| u128::from_str_radix(x, 16).expect("Unable to parse as u128"))
            .sum();

        let bytes = summarize.to_be_bytes().into_iter().collect::<Vec<_>>();

        // TODO replace key with md5 hash!

        let mut words = self.key_to_words(key); // &bytes[..]

        let subkeys_count = 2 * (self.rounds + 1);
        let mut subkeys = vec![W::ZERO; subkeys_count];

        subkeys[0] = W::P;
        for i in 1..subkeys_count {
            subkeys[i] = subkeys[i - 1].wrapping_add(&W::Q);
        }

        let mut i = 0;
        let mut j = 0;
        let mut a = W::zero();
        let mut b = W::zero();

        // 3 * max(t, c)
        let iters = max(subkeys.len(), words.len()) * 3;

        for _ in 0..iters {
            subkeys[i] = subkeys[i].wrapping_add(&a).wrapping_add(&b).rotate_left(3);
            a = subkeys[i];

            let rotation = a
                .wrapping_add(&b)
                .to_u128()
                .expect("Unable to cast to u128")
                % W::BITS as u128;

            words[j] = words[j]
                .wrapping_add(&a)
                .wrapping_add(&b)
                .rotate_left(rotation as u32);
            b = words[j];

            i = (i + 1) % subkeys.len();
            j = (j + 1) % words.len();
        }

        subkeys
    }

    fn encrypt_block(&self, pt: [W; 2], key: &[u8]) -> [W; 2] {
        let s = self.key_expand(key);

        let [mut a, mut b] = pt;

        a = a.wrapping_add(&s[0]);
        b = b.wrapping_add(&s[1]);

        for i in 1..=self.rounds {
            let rotation_b =
                b.to_u128().expect("Unable to parse as u128 at encrypt!") % W::BITS as u128;
            a = ((a ^ b).rotate_left(rotation_b as u32)).wrapping_add(&s[2 * i]);

            let rotation_a =
                a.to_u128().expect("Unable to parse as u128 at encrypt!") % W::BITS as u128;

            b = ((b ^ a).rotate_left(rotation_a as u32)).wrapping_add(&s[2 * i + 1]);
        }

        [a, b]
    }

    fn decrypt_block(&self, ct: [W; 2], key: &[u8]) -> [W; 2] {
        let s = self.key_expand(key);

        let [mut a, mut b] = ct;

        for i in (1..=self.rounds).rev() {
            let rotation_a =
                a.to_u128().expect("Unable to parse as u128 at encrypt!") % W::BITS as u128;
            b = ((b.wrapping_sub(&s[2 * i + 1])).rotate_right(rotation_a as u32)) ^ a;
            let rotation_b =
                b.to_u128().expect("Unable to parse as u128 at encrypt!") % W::BITS as u128;
            a = ((a.wrapping_sub(&s[2 * i])).rotate_right(rotation_b as u32)) ^ b;
        }

        [a.wrapping_sub(&s[0]), b.wrapping_sub(&s[1])]
    }

    pub fn encrypt(&self, plain: &[u8], key: &[u8]) -> Vec<u8> {
        let word_bytes = size_of::<W>();
        let block_size = 2 * word_bytes;

        let mut ciphertext = Vec::<u8>::with_capacity(plain.len());

        for block in plain.chunks(block_size) {
            let block = [
                W::from_le_bytes(&block[0..word_bytes]),
                W::from_le_bytes(&block[word_bytes..block_size]),
            ];

            ciphertext.extend(
                self.encrypt_block(block, key)
                    .into_iter()
                    .map(|w| w.to_le_bytes())
                    .flatten(),
            )
        }

        ciphertext
    }

    pub fn decrypt(&self, ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
        let word_bytes = size_of::<W>();
        let block_size = 2 * word_bytes;

        let mut plain = Vec::<u8>::with_capacity(ciphertext.len());

        for block in ciphertext.chunks(block_size) {
            let block = [
                W::from_le_bytes(&block[0..word_bytes]),
                W::from_le_bytes(&block[word_bytes..block_size]),
            ];

            plain.extend(
                self.decrypt_block(block, key)
                    .into_iter()
                    .map(|w| w.to_le_bytes())
                    .flatten(),
            )
        }

        plain
    }

    pub fn encrypt_cbc(&mut self, plain: &[u8], key: &[u8]) -> Vec<u8> {
        let mut plain_clone = plain.clone().to_vec();
        let plaintext_len = plain.len();
        let word_bytes = size_of::<W>();
        let block_size = 2 * word_bytes;

        let steps = (plaintext_len + (block_size - 1)) / block_size;

        self.extended_part = steps * block_size - plaintext_len;

        plain_clone.extend(vec![0u8; steps * block_size - plaintext_len]);

        let mut ciphertext = Vec::<u8>::with_capacity(plain_clone.len());

        // todo: add lcg generator as initialization vector

        let mut ct = [W::ZERO; 2];

        for (round, block) in plain_clone.chunks(block_size).enumerate() {
            let block = [
                W::from_le_bytes(&block[0..word_bytes]),
                W::from_le_bytes(&block[word_bytes..block_size]),
            ];

            let pt = match round {
                0 => block,
                _ => [block[0] ^ ct[0], block[1] ^ ct[1]],
            };

            ct = self.encrypt_block(pt, &key);

            ciphertext.extend(ct[0].to_le_bytes());
            ciphertext.extend(ct[1].to_le_bytes());
        }

        ciphertext
    }

    pub fn decrypt_cbc(&mut self, ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
        let word_bytes = size_of::<W>();
        let block_size = 2 * word_bytes;

        let mut plaintext = Vec::<u8>::new();

        // todo: add lcg generator as initialization vector

        let mut ct_prev = [W::ZERO; 2];

        for (round, block) in ciphertext.chunks(block_size).enumerate() {
            let block = [
                W::from_le_bytes(&block[0..word_bytes]),
                W::from_le_bytes(&block[word_bytes..block_size]),
            ];

            let ct = [block[0], block[1]];

            let pt = self.decrypt_block(ct, &key);

            let pt = match round {
                0 => pt,
                _ => [pt[0] ^ ct_prev[0], pt[1] ^ ct_prev[1]],
            };

            ct_prev = ct;

            plaintext.extend(pt[0].to_le_bytes());
            plaintext.extend(pt[1].to_le_bytes());
        }

        plaintext.drain(plaintext.len() - self.extended_part..plaintext.len());

        plaintext
    }
}
