use std::str;

const BASE64: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];
const BASE64_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const BASE64URL: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '-', '_',
];
const BASE64URL_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
const BASE32: [char; 32] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '2', '3', '4', '5', '6', '7',
];
const BASE32_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
const BASE32HEX: [char; 32] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
];
const BASE32HEX_STR: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUV";
const BASE16: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];
const BASE16_STR: &str = "0123456789ABCDEF";

pub enum EncodeType {
    Base64,
    Base64Url,
    Base32,
    Base32Hex,
    Base16,
}

pub struct Encoder {
    encode_type: EncodeType,
    split_range: usize,
    char_group_len: usize,
}

impl Encoder {
    pub fn new(encode_type: EncodeType) -> Self {
        let (split_range, char_group_len): (usize, usize) = match encode_type {
            EncodeType::Base64 => (6, 4),
            EncodeType::Base64Url => (6, 4),
            EncodeType::Base32 => (5, 8),
            EncodeType::Base32Hex => (5, 8),
            EncodeType::Base16 => (4, 2),
        };
        Self {
            encode_type,
            split_range,
            char_group_len,
        }
    }

    pub fn encode(&self, input: &[u8]) -> String {
        let mut bits = String::new();
        input.iter().for_each(|byte| {
            bits = format!("{}{:08b}", bits, byte);
        });
        let mut left: usize = 0;
        let last = bits.len();
        let mut result = String::new();
        let mut use_padding = true;

        while left < last {
            let right = if left.saturating_add(self.split_range) > last {
                last
            } else {
                left.saturating_add(self.split_range)
            };
            let pos = usize::from_str_radix(
                &format!("{:0<pad$}", &bits[left..right], pad = self.split_range),
                2,
            )
            .unwrap();
            let ch = match self.encode_type {
                EncodeType::Base64 => BASE64[pos],
                EncodeType::Base64Url => {
                    use_padding = false;
                    BASE64URL[pos]
                }
                EncodeType::Base32 => BASE32[pos],
                EncodeType::Base32Hex => BASE32HEX[pos],
                EncodeType::Base16 => BASE16[pos],
            };
            result = format!("{}{}", result, ch);
            left = right;
        }

        if result.len() % self.char_group_len > 0 && use_padding {
            for _ in 0..(self.char_group_len - (result.len() % self.char_group_len)) {
                result = format!("{}=", result);
            }
        }

        result
    }

    pub fn decode(&self, input: &str) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        let mut bits = String::new();

        for c in input.chars() {
            let base64_pos = match self.encode_type {
                EncodeType::Base64 => BASE64_STR.find(c),
                EncodeType::Base64Url => BASE64URL_STR.find(c),
                EncodeType::Base32 => BASE32_STR.find(c),
                EncodeType::Base32Hex => BASE32HEX_STR.find(c),
                EncodeType::Base16 => BASE16_STR.find(c),
            };
            bits = format!(
                "{}{:0>pad$b}",
                bits,
                base64_pos.unwrap_or(0),
                pad = self.split_range
            );
        }

        let mut idx: usize = 0;
        while idx < bits.len() {
            let end = idx.saturating_add(8);
            let b = if end <= bits.len() {
                bits[idx..end].to_string()
            } else {
                format!("{:0<8}", &bits[idx..(bits.len() - 1)])
            };
            let u8 = u8::from_str_radix(&b, 2).unwrap();
            if u8 != 0 {
                result.push(u8);
            }
            idx = idx.saturating_add(8);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode() {
        let test = vec![
            "",
            "f",
            "fo",
            "foo",
            "foob",
            "fooba",
            "foobar",
            "abc123!?$*&()'-=@~",
        ];
        let test_bytes: Vec<&[u8]> = test.iter().map(|t| t.as_bytes()).collect();
        let expect = vec![
            "",
            "Zg==",
            "Zm8=",
            "Zm9v",
            "Zm9vYg==",
            "Zm9vYmE=",
            "Zm9vYmFy",
            "YWJjMTIzIT8kKiYoKSctPUB+",
        ];
        let encoder = Encoder::new(EncodeType::Base64);
        test_bytes.iter().enumerate().for_each(|(i, t)| {
            assert_eq!(expect[i], encoder.encode(t));
        });
    }

    #[test]
    fn test_base64_decode() {
        let test = vec![
            "",
            "Zg==",
            "Zm8=",
            "Zm9v",
            "Zm9vYg==",
            "Zm9vYmE=",
            "Zm9vYmFy",
            "YWJjMTIzIT8kKiYoKSctPUB+",
        ];
        let expect = vec![
            "",
            "f",
            "fo",
            "foo",
            "foob",
            "fooba",
            "foobar",
            "abc123!?$*&()'-=@~",
        ];
        let expect_bytes: Vec<&[u8]> = expect.iter().map(|e| e.as_bytes()).collect();
        let encoder = Encoder::new(EncodeType::Base64);
        test.iter().enumerate().for_each(|(i, t)| {
            assert_eq!(expect_bytes[i], encoder.decode(t));
        });
    }

    #[test]
    fn test_base64url_encode() {
        let test = vec![
            "",
            "f",
            "fo",
            "foo",
            "foob",
            "fooba",
            "foobar",
            "abc123!?$*&()'-=@~",
        ];
        let test_bytes: Vec<&[u8]> = test.iter().map(|t| t.as_bytes()).collect();
        let expect = vec![
            "",
            "Zg",
            "Zm8",
            "Zm9v",
            "Zm9vYg",
            "Zm9vYmE",
            "Zm9vYmFy",
            "YWJjMTIzIT8kKiYoKSctPUB-",
        ];
        let encoder = Encoder::new(EncodeType::Base64Url);
        test_bytes.iter().enumerate().for_each(|(i, t)| {
            assert_eq!(expect[i], encoder.encode(t));
        });
    }

    #[test]
    fn test_base64url_decode() {
        let test = vec![
            "",
            "Zg",
            "Zm8",
            "Zm9v",
            "Zm9vYg",
            "Zm9vYmE",
            "Zm9vYmFy",
            "YWJjMTIzIT8kKiYoKSctPUB-",
        ];
        let expect = vec![
            "",
            "f",
            "fo",
            "foo",
            "foob",
            "fooba",
            "foobar",
            "abc123!?$*&()'-=@~",
        ];
        let expect_bytes: Vec<&[u8]> = expect.iter().map(|e| e.as_bytes()).collect();
        let encoder = Encoder::new(EncodeType::Base64Url);
        test.iter().enumerate().for_each(|(i, t)| {
            assert_eq!(expect_bytes[i], encoder.decode(t));
        });
    }

    #[test]
    fn test_base32_encode() {
        let test = vec![
            "",
            "f",
            "fo",
            "foo",
            "foob",
            "fooba",
            "foobar",
            "abc123!?$*&()'-=@~",
        ];
        let test_bytes: Vec<&[u8]> = test.iter().map(|t| t.as_bytes()).collect();
        let expect = vec![
            "",
            "MY======",
            "MZXQ====",
            "MZXW6===",
            "MZXW6YQ=",
            "MZXW6YTB",
            "MZXW6YTBOI======",
            "MFRGGMJSGMQT6JBKEYUCSJZNHVAH4===",
        ];
        let encoder = Encoder::new(EncodeType::Base32);
        test_bytes.iter().enumerate().for_each(|(i, t)| {
            assert_eq!(expect[i], encoder.encode(t));
        });
    }

    #[test]
    fn test_base32_decode() {
        let test = vec![
            "",
            "MY======",
            "MZXQ====",
            "MZXW6===",
            "MZXW6YQ=",
            "MZXW6YTB",
            "MZXW6YTBOI======",
            "MFRGGMJSGMQT6JBKEYUCSJZNHVAH4===",
        ];
        let expect = vec![
            "",
            "f",
            "fo",
            "foo",
            "foob",
            "fooba",
            "foobar",
            "abc123!?$*&()'-=@~",
        ];
        let expect_bytes: Vec<&[u8]> = expect.iter().map(|e| e.as_bytes()).collect();
        let encoder = Encoder::new(EncodeType::Base32);
        test.iter().enumerate().for_each(|(i, t)| {
            assert_eq!(expect_bytes[i], encoder.decode(t));
        });
    }

    #[test]
    fn test_base32hex_encode() {
        let test = vec![
            "",
            "f",
            "fo",
            "foo",
            "foob",
            "fooba",
            "foobar",
            "abc123!?$*&()'-=@~",
        ];
        let test_bytes: Vec<&[u8]> = test.iter().map(|t| t.as_bytes()).collect();
        let expect = vec![
            "",
            "CO======",
            "CPNG====",
            "CPNMU===",
            "CPNMUOG=",
            "CPNMUOJ1",
            "CPNMUOJ1E8======",
            "C5H66C9I6CGJU91A4OK2I9PD7L07S===",
        ];
        let encoder = Encoder::new(EncodeType::Base32Hex);
        test_bytes.iter().enumerate().for_each(|(i, t)| {
            assert_eq!(expect[i], encoder.encode(t));
        });
    }

    #[test]
    fn test_base32hex_decode() {
        let test = vec![
            "",
            "CO======",
            "CPNG====",
            "CPNMU===",
            "CPNMUOG=",
            "CPNMUOJ1",
            "CPNMUOJ1E8======",
            "C5H66C9I6CGJU91A4OK2I9PD7L07S===",
        ];
        let expect = vec![
            "",
            "f",
            "fo",
            "foo",
            "foob",
            "fooba",
            "foobar",
            "abc123!?$*&()'-=@~",
        ];
        let expect_bytes: Vec<&[u8]> = expect.iter().map(|e| e.as_bytes()).collect();
        let encoder = Encoder::new(EncodeType::Base32Hex);
        test.iter().enumerate().for_each(|(i, t)| {
            assert_eq!(expect_bytes[i], encoder.decode(t));
        });
    }

    #[test]
    fn test_base16_encode() {
        let test = vec![
            "",
            "f",
            "fo",
            "foo",
            "foob",
            "fooba",
            "foobar",
            "abc123!?$*&()'-=@~",
        ];
        let test_bytes: Vec<&[u8]> = test.iter().map(|t| t.as_bytes()).collect();
        let expect = vec![
            "",
            "66",
            "666F",
            "666F6F",
            "666F6F62",
            "666F6F6261",
            "666F6F626172",
            "616263313233213F242A262829272D3D407E",
        ];
        let encoder = Encoder::new(EncodeType::Base16);
        test_bytes.iter().enumerate().for_each(|(i, t)| {
            assert_eq!(expect[i], encoder.encode(t));
        });
    }

    #[test]
    fn test_base16_decode() {
        let test = vec![
            "",
            "66",
            "666F",
            "666F6F",
            "666F6F62",
            "666F6F6261",
            "666F6F626172",
            "616263313233213F242A262829272D3D407E",
        ];
        let expect = vec![
            "",
            "f",
            "fo",
            "foo",
            "foob",
            "fooba",
            "foobar",
            "abc123!?$*&()'-=@~",
        ];
        let expect_bytes: Vec<&[u8]> = expect.iter().map(|e| e.as_bytes()).collect();
        let encoder = Encoder::new(EncodeType::Base16);
        test.iter().enumerate().for_each(|(i, t)| {
            assert_eq!(expect_bytes[i], encoder.decode(t));
        });
    }
}