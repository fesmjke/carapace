// TODO: add list of 2^32 * |sin(i)| values in range 0..63
const TABLE: Vec<u32> = vec![];

pub struct MD5<'a> {
    // TODO: add md5 parts
    message: &'a str,     // input str
    message_raw: Vec<u8>, // converted to bytes -> (bits)
    append_part: Vec<u8>, // append part in bytes -> (bits)
    buffer: [u32; 4],     // buffer that will contain 4 32-bit registers
    length: u64,          // output length of message
    output: u8,           // hashed output
}

impl MD5 {
    pub fn from(input: &str) -> String {
        todo!()
    }

    pub fn new() -> Self {
        Self {
            message: "",
            message_raw: vec![],
            append_part: vec![],
            buffer: [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476],
            length: 0,
        }
    }

    fn generate(input: &str) -> String {
        // TODO: step 1 -> 512 - 64 (message % 512 == 0)
        // (message len in bits) - 64 % 512 == 0
        // example: let message will be 448 bits long, then we add 512 bits to message to make
        // it 960 bits long, then after adding 64 bits we will have 1024 bits long message
        // it will divide by 512 with remainder equal to 0
        // bits structure that we add have following sequence -> 0...1
        // pseudo:
        // get message bits (bytes)

        // TODO: step 2 -> after append a 'append part'
        // we will have a:
        // [message] + [append part (from 1 to 512 bits)] + [length of output message]
        //
        todo!()
    }

    fn compress(&self) {
        // TODO: implement a compress cycles
        // 1 -> fF table[0..15], x[p1 * i]
        // fH = (buffer[B] & buffer[C]) | (!buffer[B] & buffer[D])
        // p1 = i
        // shift = (i11 = 7, i12 = 12, i13 = 17, i14 = 22)
        // 2 -> fG table[16..31], x[p2 * i]
        // p2 = (1 + 5i) % 16
        // shift = (i21 = 5, i22 = 9, i23 = 14, i24 = 20)
        // fG = (buffer[B] & buffer[D]) | (buffer[C] & !buffer[D])
        // 3 -> fH table[32..47], x[p3 * i]
        // p3 = (5 + 3i) % 16
        // shift = (i31 = 4, i32 = 11, i33 = 16, i34 = 23)
        // fH = buffer[B] ^ buffer[C] ^ buffer[D]
        // 4 -> fG table[48..63], x[p4 * i]
        // p4 = 7i % 16
        // shift = (i41 = 6, i42 = 10, i43 = 15, i44 = 21)
        // fG = buffer[C] ^ (buffer[B] | !buffer[D])
    }
}
