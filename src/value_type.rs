use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Word(pub u16);

impl Serialize for Word {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = vec![self.0 as u8, (self.0 >> 8) as u8];
        serializer.serialize_bytes(&bytes)
    }
}
struct WordVisitor;

impl<'de> Visitor<'de> for WordVisitor {
    type Value = Word;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an u16 between 2^0 and 2^16")
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let bytes = v.to_be_bytes();
        let word = u16::from_le_bytes(bytes);
        Ok(Word(word))
    }
}

impl<'de> Deserialize<'de> for Word {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u16(WordVisitor)
    }
}
