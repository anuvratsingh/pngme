

use std::{convert::Infallible, fmt, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ChunkType {
    ancillary: u8,
    private: u8,
    reserved: u8,
    safe_to_copy: u8,
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        [
            self.ancillary,
            self.private,
            self.reserved,
            self.safe_to_copy,
        ]
    }
    pub fn is_valid(&self) -> bool {
        if char::try_from(self.reserved)
            .expect("Invalid")
            .is_uppercase()
        {
            true
        } else {
            false
        }
    }
    pub fn is_critical(&self) -> bool {
        if char::try_from(self.ancillary)
            .expect("Invalid")
            .is_uppercase()
        {
            true
        } else {
            false
        }
    }
    pub fn is_public(&self) -> bool {
        if char::try_from(self.private)
            .expect("Invalid")
            .is_uppercase()
        {
            true
        } else {
            false
        }
    }
    pub fn is_reserved_bit_valid(&self) -> bool {
        if char::try_from(self.reserved)
            .expect("Invalid")
            .is_uppercase()
        {
            true
        } else {
            false
        }
    }
    pub fn is_safe_to_copy(&self) -> bool {
        if char::try_from(self.safe_to_copy)
            .expect("Invalid")
            .is_uppercase()
        {
            false
        } else {
            true
        }
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Infallible;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        match value {
            [length, chunk_type, chunk_data, crc] => Ok(Self {
                ancillary: length,
                private: chunk_type,
                reserved: chunk_data,
                safe_to_copy: crc,
            }),
        }
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arr = s.as_bytes();
        if arr.len() > 4 {
            return Err("Invalid Input");
        };

        for a in arr {
            if !char::from(a.clone()).is_alphabetic() {
                return Err("Invalid bit");
            };
        }

        Ok(Self {
            ancillary: arr[0],
            private: arr[1],
            reserved: arr[2],
            safe_to_copy: arr[3],
        })
    }
}

impl Into<String> for ChunkType {
    fn into(self) -> String {
        format!(
            "{}{}{}{}",
            char::from(self.ancillary),
            char::from(self.private),
            char::from(self.reserved),
            char::from(self.safe_to_copy)
        )
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            char::from(self.ancillary),
            char::from(self.private),
            char::from(self.reserved),
            char::from(self.safe_to_copy)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
