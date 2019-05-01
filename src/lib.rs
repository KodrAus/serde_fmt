use std::fmt::{self, Debug, Display};

use serde::ser::{
    self, Serialize, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant,
    SerializeTuple, SerializeTupleStruct, SerializeTupleVariant, Serializer,
};

pub trait ToDebug {
    fn to_debug(&self) -> SerializeDebug<&Self> {
        SerializeDebug(self)
    }
}

impl<S> ToDebug for S
where
    S: ?Sized + Serialize,
{
    
}

pub fn to_formatter(value: impl Serialize, fmt: &mut fmt::Formatter) -> fmt::Result {
    value.serialize(Formatter::new(fmt)).map_err(Into::into)
}

pub struct SerializeDebug<T>(T);

impl<T> Debug for SerializeDebug<T>
where
    T: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        to_formatter(&self.0, f)
    }
}

struct Formatter<'a, 'b: 'a>(&'a mut fmt::Formatter<'b>);

impl<'a, 'b: 'a> Formatter<'a, 'b> {
    fn new(fmt: &'a mut fmt::Formatter<'b>) -> Self {
        Formatter(fmt)
    }

    fn fmt(self, v: impl Debug) -> Result<(), Error> {
        v.fmt(self.0).map_err(Into::into)
    }
}

impl<'a, 'b: 'a> Serializer for Formatter<'a, 'b> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = DebugSeq<'a, 'b>;
    type SerializeTuple = DebugTuple<'a, 'b>;
    type SerializeTupleStruct = DebugTupleStruct<'a, 'b>;
    type SerializeTupleVariant = DebugTupleVariant<'a, 'b>;
    type SerializeMap = DebugMap<'a, 'b>;
    type SerializeStruct = DebugStruct<'a, 'b>;
    type SerializeStructVariant = DebugStructVariant<'a, 'b>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        write!(self.0, "None")?;
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        write!(self.0, "()")?;
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_tuple_struct(name, 0)?.end()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_tuple_struct(variant, 0)?.end()
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut tuple = self.serialize_tuple_struct(name, 1)?;
        tuple.serialize_field(value)?;
        tuple.end()
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut tuple = self.serialize_tuple_struct(variant, 1)?;
        tuple.serialize_field(value)?;
        tuple.end()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(DebugSeq(self.0.debug_list()))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(DebugTuple(self.0.debug_tuple("")))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(DebugTupleStruct(self.0.debug_tuple(name)))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(DebugTupleVariant(self.0.debug_tuple(variant)))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let alternate = self.0.alternate();

        Ok(DebugMap::new(self.0.debug_map(), alternate))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(DebugStruct(self.0.debug_struct(name)))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(DebugStructVariant(self.0.debug_struct(variant)))
    }
}

struct DebugSeq<'a, 'b: 'a>(fmt::DebugList<'a, 'b>);

impl<'a, 'b: 'a> SerializeSeq for DebugSeq<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.entry(&value.to_debug());
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.finish().map_err(Into::into)
    }
}

struct DebugTuple<'a, 'b: 'a>(fmt::DebugTuple<'a, 'b>);

impl<'a, 'b: 'a> SerializeTuple for DebugTuple<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.field(&value.to_debug());
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.finish().map_err(Into::into)
    }
}

struct DebugTupleStruct<'a, 'b: 'a>(fmt::DebugTuple<'a, 'b>);

impl<'a, 'b: 'a> SerializeTupleStruct for DebugTupleStruct<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.field(&value.to_debug());
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.finish().map_err(Into::into)
    }
}

struct DebugTupleVariant<'a, 'b: 'a>(fmt::DebugTuple<'a, 'b>);

impl<'a, 'b: 'a> SerializeTupleVariant for DebugTupleVariant<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.field(&value.to_debug());
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.finish().map_err(Into::into)
    }
}

struct DebugStruct<'a, 'b: 'a>(fmt::DebugStruct<'a, 'b>);

impl<'a, 'b: 'a> SerializeStruct for DebugStruct<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.field(key, &value.to_debug());
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.finish().map_err(Into::into)
    }
}

struct DebugStructVariant<'a, 'b: 'a>(fmt::DebugStruct<'a, 'b>);

impl<'a, 'b: 'a> SerializeStructVariant for DebugStructVariant<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.field(key, &value.to_debug());
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.finish().map_err(Into::into)
    }
}

struct DebugMap<'a, 'b: 'a> {
    fmt: fmt::DebugMap<'a, 'b>,
    alternate: bool,
    key_buf: KeyBuf,
    has_key: bool,
}

impl<'a, 'b: 'a> DebugMap<'a, 'b> {
    fn new(fmt: fmt::DebugMap<'a, 'b>, alternate: bool) -> Self {
        DebugMap {
            fmt,
            alternate,
            key_buf: KeyBuf(String::new()),
            has_key: false,
        }
    }
}

impl<'a, 'b: 'a> SerializeMap for DebugMap<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<Self::Ok, Self::Error>
    where
        K: ?Sized + Serialize,
        V: ?Sized + Serialize,
    {
        if self.has_key {
            return Err(Error);
        }

        self.fmt.entry(&key.to_debug(), &value.to_debug());
        Ok(())
    }

    fn serialize_key<T>(&mut self, key: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        use std::fmt::Write;

        if self.has_key {
            return Err(Error);
        }

        // If a key is given independently then we need to buffer it
        // NOTE: It's possible this buffering is resulting in lost flags
        self.key_buf.0.clear();
        if self.alternate {
            write!(&mut self.key_buf.0, "{:#?}", key.to_debug())?;
        } else {
            write!(&mut self.key_buf.0, "{:?}", key.to_debug())?;
        }

        self.has_key = true;
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.has_key {
            return Err(Error);
        }

        self.fmt.entry(&self.key_buf, &value.to_debug());
        self.has_key = false;
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.fmt.finish().map_err(Into::into)
    }
}

struct KeyBuf(String);

impl fmt::Debug for KeyBuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug)]
struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "serde_fmt error")
    }
}

impl From<Error> for fmt::Error {
    fn from(_: Error) -> fmt::Error {
        fmt::Error
    }
}

impl From<fmt::Error> for Error {
    fn from(_: fmt::Error) -> Error {
        Error
    }
}

impl std::error::Error for Error {}

impl ser::Error for Error {
    fn custom<T>(_: T) -> Self
    where
        T: Display,
    {
        Error
    }
}
