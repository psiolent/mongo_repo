use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;

use serde::{de, Deserialize, Deserializer, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
pub struct Name(String);

/// An error indicating a new name could not be created from an empty string.
#[derive(Debug, Clone)]
pub struct EmptyNameError;

impl FromStr for Name {
    type Err = EmptyNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(EmptyNameError)
        } else {
            Ok(Name(s.to_string()))
        }
    }
}

impl TryFrom<String> for Name {
    type Error = EmptyNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(EmptyNameError)
        } else {
            Ok(Name(value))
        }
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Deref for Name {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(NameStringVisitor)
    }
}

impl Error for EmptyNameError {}

impl Display for EmptyNameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "a name cannot be an empty string")
    }
}

/// A string visitor for deserializing names.
struct NameStringVisitor;

impl<'de> de::Visitor<'de> for NameStringVisitor {
    type Value = Name;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a non-empty string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Name::from_str(v).map_err(serde::de::Error::custom)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Name::try_from(v).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn non_empty_name_can_be_constructed() {
        let new_name_result = Name::from_str("name");
        assert!(new_name_result.is_ok());
    }

    #[test]
    fn empty_name_cannot_be_constructed() {
        let new_name_result = Name::from_str("");
        assert!(new_name_result.is_err());
    }

    #[test]
    fn name_is_equal_after_serializing_and_deserializing() {
        let orig_name = Name::from_str("name").unwrap();
        let ser_name = serde_json::to_string(&orig_name).unwrap();
        let deser_name_result = serde_json::from_str::<Name>(ser_name.as_str());

        assert!(deser_name_result.is_ok());

        let deser_name = deser_name_result.unwrap();
        assert_eq!(deser_name, orig_name);
    }

    #[test]
    fn empty_name_cannot_be_deserialized() {
        let json = r#""#;
        let deser_result = serde_json::from_str::<Name>(json);
        assert!(deser_result.is_err());
    }
}
