use std::str;

use form_urlencoded::{Serializer as UrlEncodedSerializer, Target as UrlEncodedTarget};
use serde::ser::{Serialize, SerializeSeq};

use super::{
    part::{PartSerializer, Sink},
    Error,
};

pub struct ValueSink<'input, 'key, 'target, Target>
where
    Target: UrlEncodedTarget,
{
    urlencoder: &'target mut UrlEncodedSerializer<'input, Target>,
    key: &'key str,
    nested: bool,
}

impl<'input, 'key, 'target, Target> ValueSink<'input, 'key, 'target, Target>
where
    Target: 'target + UrlEncodedTarget,
{
    pub fn new(
        urlencoder: &'target mut UrlEncodedSerializer<'input, Target>,
        key: &'key str,
    ) -> Self {
        ValueSink { urlencoder, key, nested: false }
    }
}

impl<'target, Target> Sink for ValueSink<'_, '_, 'target, Target>
where
    Target: 'target + UrlEncodedTarget,
{
    type Ok = ();
    type SerializeSeq = Self;

    fn serialize_str(self, value: &str) -> Result<(), Error> {
        self.urlencoder.append_pair(self.key, value);
        Ok(())
    }

    fn serialize_static_str(self, value: &'static str) -> Result<(), Error> {
        self.serialize_str(value)
    }

    fn serialize_string(self, value: String) -> Result<(), Error> {
        self.serialize_str(&value)
    }

    fn serialize_none(self) -> Result<Self::Ok, Error> {
        Ok(())
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Error> {
        value.serialize(PartSerializer::new(self))
    }

    fn serialize_seq(self) -> Result<Self, Error> {
        if self.nested {
            Err(self.unsupported())
        } else {
            Ok(self)
        }
    }

    fn unsupported(self) -> Error {
        Error::unsupported_value()
    }
}

impl<'target, Target> SerializeSeq for ValueSink<'_, '_, 'target, Target>
where
    Target: 'target + UrlEncodedTarget,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        value.serialize(PartSerializer::new(ValueSink {
            urlencoder: self.urlencoder,
            key: self.key,
            nested: true,
        }))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
