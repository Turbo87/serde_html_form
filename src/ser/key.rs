use std::{borrow::Cow, ops::Deref};

use serde::ser::{self, Serialize};

use super::{part::Sink, Error};

pub enum Key<'key> {
    Static(&'static str),
    Dynamic(Cow<'key, str>),
}

impl Deref for Key<'_> {
    type Target = str;

    fn deref(&self) -> &str {
        match *self {
            Key::Static(key) => key,
            Key::Dynamic(ref key) => key,
        }
    }
}

impl<'key> From<Key<'key>> for Cow<'static, str> {
    fn from(key: Key<'key>) -> Self {
        match key {
            Key::Static(key) => key.into(),
            Key::Dynamic(key) => key.into_owned().into(),
        }
    }
}

pub struct KeySink<End> {
    end: End,
}

impl<End, Ok> KeySink<End>
where
    End: for<'key> FnOnce(Key<'key>) -> Result<Ok, Error>,
{
    pub fn new(end: End) -> Self {
        KeySink { end }
    }
}

impl<End, Ok> Sink for KeySink<End>
where
    End: for<'key> FnOnce(Key<'key>) -> Result<Ok, Error>,
{
    type Ok = Ok;
    type SerializeSeq = ser::Impossible<Self::Ok, Error>;

    fn serialize_static_str(self, value: &'static str) -> Result<Ok, Error> {
        (self.end)(Key::Static(value))
    }

    fn serialize_str(self, value: &str) -> Result<Ok, Error> {
        (self.end)(Key::Dynamic(value.into()))
    }

    fn serialize_string(self, value: String) -> Result<Ok, Error> {
        (self.end)(Key::Dynamic(value.into()))
    }

    fn serialize_none(self) -> Result<Ok, Error> {
        Err(self.unsupported())
    }

    fn serialize_some<T: ?Sized + Serialize>(self, _value: &T) -> Result<Ok, Error> {
        Err(self.unsupported())
    }

    fn serialize_seq(self) -> Result<Self::SerializeSeq, Error> {
        Err(self.unsupported())
    }

    fn unsupported(self) -> Error {
        Error::unsupported_key()
    }
}
