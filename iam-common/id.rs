use anyhow::bail;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display},
    str::FromStr,
};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdType {
    Action,
    Group,
    User,
    App,
}

impl IdType {
    const fn as_str(&self) -> &'static str {
        match self {
            IdType::Action => "ActionID",
            IdType::Group => "GroupID",
            IdType::User => "UserID",
            IdType::App => "AppID",
        }
    }
}

impl FromStr for IdType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ty = match s {
            "ActionID" => IdType::Action,
            "UserID" => IdType::User,
            "AppID" => IdType::App,
            "GroupID" => IdType::Group,
            _ => bail!("invalid type"),
        };

        Ok(ty)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Id {
    uuid: Uuid,
    ty: IdType,
}

impl Id {
    fn new(ty: IdType) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            ty,
        }
    }

    #[inline]
    pub fn new_action() -> Self {
        Self::new(IdType::Action)
    }

    #[inline]
    pub fn new_group() -> Self {
        Self::new(IdType::Group)
    }

    #[inline]
    pub fn new_user() -> Self {
        Self::new(IdType::User)
    }

    #[inline]
    pub fn new_app() -> Self {
        Self::new(IdType::App)
    }

    #[inline]
    pub const fn get_type(&self) -> IdType {
        self.ty
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ty = self.get_type().as_str();

        let mut buf = Uuid::encode_buffer();
        let uuid = self.uuid.as_hyphenated().encode_lower(&mut buf);

        write!(f, "{}-{}", ty, uuid)
    }
}

impl FromStr for Id {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((ty, id_str)) = s.split_once('-') else {
            bail!("invalid format");
        };

        let ty = IdType::from_str(ty)?;

        let Ok(uuid) = Uuid::parse_str(id_str) else {
            bail!("invalid uuid");
        };

        Ok(Self { uuid, ty })
    }
}

impl Serialize for Id {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Id;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a uuid prefixed with its type")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match Id::from_str(v) {
                    Ok(id) => Ok(id),
                    Err(err) => Err(de::Error::custom(err)),
                }
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        let id = Id::from_str("UserID-00000000-0000-0000-0000-000000000000").unwrap();
        assert_eq!(id.get_type(), IdType::User);

        let id = Id::from_str("ActionID-00000000-0000-0000-0000-000000000000").unwrap();
        assert_eq!(id.get_type(), IdType::Action);

        let id = Id::from_str("UserID-00000000-0000-0000-0000-000000000000").unwrap();
        assert_eq!(id.get_type(), IdType::User);

        let id = Id::from_str("GroupID-00000000-0000-0000-0000-000000000000").unwrap();
        assert_eq!(id.get_type(), IdType::Group);
    }

    #[test]
    fn from_str_invalid_type() {
        let id = Id::from_str("Invalid-00000000-0000-0000-0000-000000000000");
        assert!(id.is_err());
    }

    #[test]
    fn from_str_invalid_uuid() {
        let id = Id::from_str("UserID-invalid");
        assert!(id.is_err());
    }

    #[test]
    fn from_str_invalid_format() {
        let id = Id::from_str("invalid");
        assert!(id.is_err());
    }

    #[test]
    fn to_string() {
        let str_id = "UserID-00000000-0000-0000-0000-000000000000";
        let id = Id::from_str(str_id).unwrap();

        assert_eq!(id.to_string(), str_id);

        let str_id = "ActionID-00000000-0000-0000-0000-000000000000";
        let id = Id::from_str(str_id).unwrap();

        assert_eq!(id.to_string(), str_id);

        let str_id = "AppID-00000000-0000-0000-0000-000000000000";
        let id = Id::from_str(str_id).unwrap();

        assert_eq!(id.to_string(), str_id);

        let str_id = "GroupID-00000000-0000-0000-0000-000000000000";
        let id = Id::from_str(str_id).unwrap();

        assert_eq!(id.to_string(), str_id);
    }
}
