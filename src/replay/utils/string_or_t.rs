use core::fmt;

use serde::{Deserialize, de::Visitor, Serialize};



pub struct StringOrTVisitor<T>{
    pub marker: std::marker::PhantomData<T>
}

#[derive(Debug, Clone, PartialEq)]
pub enum StringOrObject<T> {
    String(String),
    Object(T)
}

impl<'de, T: Deserialize<'de>> Visitor<'de> for StringOrTVisitor<T> {
    type Value = StringOrObject<T>;
    
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {

        formatter.write_fmt(format_args!("a string or an object of type {}", std::any::type_name::<T>()))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>, {
        Ok(StringOrObject::Object(
            serde::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))?
        ))

    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        Ok(StringOrObject::String(v.to_string()))
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for StringOrObject<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>, {
        deserializer.deserialize_any(StringOrTVisitor {
            marker: std::marker::PhantomData::<T>
        })
    }
}

impl<T: Serialize> Serialize for StringOrObject<T> {  
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer, {
        match self {
            StringOrObject::String(string) => string.serialize(serializer),
            StringOrObject::Object(obj) => obj.serialize(serializer)
        }
    }
}