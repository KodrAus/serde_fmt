/*!
# `serde::Serialize` -> `std::fmt::Debug`

This library lets you take any `Serialize` and format it as if it's `Debug`.
*/

// https://github.com/rust-lang/rust/issues/62482
#![feature(debug_map_key_value)]

#![no_std]

#[cfg(not(feature = "std"))]
extern crate core as std;

#[cfg(feature = "std")]
extern crate std;

use crate::std::fmt::{self, Debug, Display};

use serde::ser::{
    self, Serialize, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant,
    SerializeTuple, SerializeTupleStruct, SerializeTupleVariant, Serializer,
};

/**
Format the given value into the formatter.
*/
pub fn to_formatter(v: impl Serialize, fmt: &mut fmt::Formatter) -> fmt::Result {
    v.serialize(Formatter::new(fmt)).map_err(Into::into)
}

/**
Treat a type implementing `Serialize` like a type implementing `Debug`.
*/
pub fn to_debug<T>(v: T) -> SerializeDebug<T>
where
    T: Serialize,
{
    SerializeDebug(v)
}

/**
Treat a type implementing `Serialize` like a type implementing `Debug`.
*/
pub trait ToDebug {
    /**
    Get a formattable reference.
    */
    fn to_debug(&self) -> SerializeDebug<&Self> {
        SerializeDebug(self)
    }
}

impl<S> ToDebug for S
where
    S: ?Sized + Serialize,
{
    
}

/**
The result of calling [`ToDebug::to_debug`].
*/
#[derive(Clone, Copy)]
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

    fn collect_str<T: ?Sized>(self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Display,
    {
        self.fmt(format_args!("{}", v))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        write!(self.0, "None")?;
        Ok(())
    }

    fn serialize_some<T>(self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_newtype_struct("Some", v)
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
        v: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut tuple = self.serialize_tuple_struct(name, 1)?;
        tuple.serialize_field(v)?;
        tuple.end()
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        v: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut tuple = self.serialize_tuple_struct(variant, 1)?;
        tuple.serialize_field(v)?;
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
        Ok(DebugMap(self.0.debug_map()))
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

    fn serialize_element<T>(&mut self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.entry(&v.to_debug());
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

    fn serialize_element<T>(&mut self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.field(&v.to_debug());
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

    fn serialize_field<T>(&mut self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.field(&v.to_debug());
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

    fn serialize_field<T>(&mut self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.field(&v.to_debug());
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

    fn serialize_field<T>(&mut self, k: &'static str, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.field(k, &v.to_debug());
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

    fn serialize_field<T>(&mut self, k: &'static str, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.field(k, &v.to_debug());
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.finish().map_err(Into::into)
    }
}

struct DebugMap<'a, 'b: 'a>(fmt::DebugMap<'a, 'b>);

impl<'a, 'b: 'a> SerializeMap for DebugMap<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_entry<K, V>(&mut self, k: &K, v: &V) -> Result<Self::Ok, Self::Error>
    where
        K: ?Sized + Serialize,
        V: ?Sized + Serialize,
    {
        self.0.entry(&k.to_debug(), &v.to_debug());
        Ok(())
    }

    fn serialize_key<T>(&mut self, k: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.key(&k.to_debug());
        Ok(())
    }

    fn serialize_value<T>(&mut self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.value(&v.to_debug());
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.finish().map_err(Into::into)
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

#[cfg(feature = "std")]
impl crate::std::error::Error for Error {}

impl ser::Error for Error {
    fn custom<T>(_: T) -> Self
    where
        T: Display,
    {
        Error
    }
}

#[cfg(test)]
#[macro_use]
extern crate serde_derive;

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;

    fn check_fmt(v: (impl fmt::Debug + Serialize)) {
        use crate::std::format;

        assert_eq!(format!("{:?}", v), format!("{:?}", v.to_debug()));
    }

    #[test]
    fn struct_fmt_is_consitent() {
        #[derive(Serialize, Debug)]
        struct Struct {
            a: Signed,
            b: Unsigned,
            c: char,
            d: &'static str,
            e: &'static [u8],
            f: (),
        }

        #[derive(Serialize, Debug)]
        struct Signed {
            a: i8,
            b: i16,
            c: i32,
            d: i64,
        }

        #[derive(Serialize, Debug)]
        struct Unsigned {
            a: u8,
            b: u16,
            c: u32,
            d: u64,
        }

        check_fmt(Struct {
            a: Signed {
                a: -1,
                b: 42,
                c: -42,
                d: 42,
            },
            b: Unsigned {
                a: 1,
                b: 42,
                c: 1,
                d: 42
            },
            c: 'a',
            d: "a string",
            e: &[1, 2, 3],
            f: (),
        });
    }

    #[test]
    fn option_fmt_is_consistent() {
        check_fmt(Option::Some::<i32>(42));
        check_fmt(Option::None::<i32>);
    }

    #[test]
    fn result_fmt_is_consistent() {
        check_fmt(Result::Ok::<i32, i32>(42));
        check_fmt(Result::Err::<i32, i32>(42));
    }

    #[test]
    fn tuple_fmt_is_consistent() {
        check_fmt((42, 17));
    }

    #[test]
    fn tagged_fmt_is_consistent() {
        #[derive(Serialize, Debug)]
        enum Tagged {
            Unit,
            NewType(i32),
            Tuple(i32, i32),
            Struct { a: i32, b: i32 },
        }

        check_fmt(Tagged::Unit);
        check_fmt(Tagged::NewType(42));
        check_fmt(Tagged::Tuple(42, 17));
        check_fmt(Tagged::Struct { a: 42, b: 17 });
    }
}
