use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum IdType {
    Action,
    Group,
    User,
    App,
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
    const fn get_prefix(&self) -> &'static str {
        match self.ty {
            IdType::Action => "ActionID",
            IdType::Group => "GroupID",
            IdType::User => "UserID",
            IdType::App => "AppID",
        }
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{}",
            self.get_prefix(),
            self.uuid
                .as_hyphenated()
                .encode_lower(&mut Uuid::encode_buffer())
        )
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
                let (ty, id_str) = if let Some(i) = v.strip_prefix("ActionID-") {
                    (IdType::Action, i)
                } else if let Some(i) = v.strip_prefix("GroupID-") {
                    (IdType::Group, i)
                } else if let Some(i) = v.strip_prefix("UserID-") {
                    (IdType::User, i)
                } else if let Some(i) = v.strip_prefix("AppID-") {
                    (IdType::App, i)
                } else {
                    return Err(de::Error::invalid_value(de::Unexpected::Str(v), &self));
                };

                let uuid = Uuid::parse_str(id_str)
                    .map_err(|_| de::Error::invalid_value(de::Unexpected::Str(id_str), &self))?;

                Ok(Id { ty, uuid })
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}
