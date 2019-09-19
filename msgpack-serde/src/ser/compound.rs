use std::io;

use crate::ser::{error::SerError, Serializer};
use serde::ser::{self, Serialize};

#[derive(Debug)]
pub struct Compound<'a, W: 'a> {
    se: &'a mut Serializer<W>,
}

impl<W: io::Write> Serializer<W> {
    #[inline]
    pub fn compound(&mut self) -> Compound<W> {
        Compound { se: self }
    }
}

impl<'a, W: io::Write + 'a> ser::SerializeMap for Compound<'a, W> {
    type Ok = ();
    type Error = SerError;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<Self::Ok, Self::Error> {
        key.serialize(&mut *self.se)
    }

    fn serialize_value<T: ?Sized + Serialize>(
        &mut self,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(&mut *self.se)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W: io::Write + 'a> ser::SerializeSeq for Compound<'a, W> {
    type Ok = ();
    type Error = SerError;

    fn serialize_element<T: ?Sized + Serialize>(
        &mut self,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(&mut *self.se)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W: io::Write + 'a> ser::SerializeTuple for Compound<'a, W> {
    type Ok = ();
    type Error = SerError;

    fn serialize_element<T: ?Sized + Serialize>(
        &mut self,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(&mut *self.se)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W: io::Write + 'a> ser::SerializeTupleStruct for Compound<'a, W> {
    type Ok = ();
    type Error = SerError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(&mut *self.se)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W: io::Write + 'a> ser::SerializeTupleVariant for Compound<'a, W> {
    type Ok = ();
    type Error = SerError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(&mut *self.se)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W: io::Write + 'a> ser::SerializeStruct for Compound<'a, W> {
    type Ok = ();
    type Error = SerError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        key.serialize(&mut *self.se)?;
        value.serialize(&mut *self.se)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W: io::Write + 'a> ser::SerializeStructVariant for Compound<'a, W> {
    type Ok = ();
    type Error = SerError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        key.serialize(&mut *self.se)?;
        value.serialize(&mut *self.se)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
