use std::convert::TryFrom;
use super::types;

pub enum TransformType {
    Wide
}

impl TryFrom<&str> for TransformType {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        return match s {
            "wide" => Ok(Self::Wide),
            _ => Err("converter does not exist")
        }
    }
}

impl TransformType {
    pub fn transform(&self, s: &str) -> String {
        return match &self {
            TransformType::Wide => types::wide(s)
        }
    }
}
