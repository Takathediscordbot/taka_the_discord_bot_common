use core::fmt;

use serde::{Deserialize, Serialize, de::Visitor};



#[derive(Debug, Clone, PartialEq)]
pub enum ObjectOrArrayContainer<T> {
    Object(T),
    Array(Vec<T>)
}

pub struct ObjectOrArrayVisitor<T>{
    pub marker: std::marker::PhantomData<T>
}

impl<'de, T: Deserialize<'de>> Visitor<'de> for ObjectOrArrayVisitor<T> {
    type Value = ObjectOrArrayContainer<T>;
    
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {

        formatter.write_fmt(format_args!("an array of objects of type {} or object of type {}", std::any::type_name::<T>(), std::any::type_name::<T>()))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>, {
        
        Ok(ObjectOrArrayContainer::Object(
            serde::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))?)
        )
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>, {
        Ok(ObjectOrArrayContainer::Array(
            serde::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))?
        ))
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for ObjectOrArrayContainer<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>, {
        deserializer.deserialize_any(ObjectOrArrayVisitor {
            marker: std::marker::PhantomData::<T>
        })
    }
}

impl<T: Serialize> Serialize for ObjectOrArrayContainer<T> {  
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer, {
        match self {
            ObjectOrArrayContainer::Object(obj) => obj.serialize(serializer),
            ObjectOrArrayContainer::Array(arr) => arr.serialize(serializer)
        }
    }
}
