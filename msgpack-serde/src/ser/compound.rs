use std::io;

use crate::ser::{error::SerError, ExtSerializer, Serializer};
use serde::ser::{self, Serialize};

#[derive(Debug)]
pub enum Compound<'a, W: 'a> {
    Normal(&'a mut Serializer<W>),
    Ext(ExtSerializer<'a, W>),
}

impl<'a, W: io::Write> Serializer<W> {
    #[inline]
    pub fn compound(&mut self) -> Compound<W> {
        Compound::Normal(self)
    }

    pub fn compound_ext(&mut self) -> Compound<W> {
        let ext_se = ExtSerializer { wr: &mut self.wr };
        Compound::Ext(ext_se)
    }
}

impl<'a, W: io::Write + 'a> ser::SerializeMap for Compound<'a, W> {
    type Ok = ();
    type Error = SerError;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<Self::Ok, Self::Error> {
        match *self {
            Compound::Normal(ref mut ser) => key.serialize(&mut **ser),
            Compound::Ext(_) => unreachable!(),
        }
    }

    fn serialize_value<T: ?Sized + Serialize>(
        &mut self,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        match self {
            Compound::Normal(ref mut ser) => value.serialize(&mut **ser),
            Compound::Ext(_) => unreachable!(),
        }
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
        match self {
            Compound::Normal(ref mut ser) => value.serialize(&mut **ser),
            Compound::Ext(_) => unreachable!(),
        }
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
        match self {
            Compound::Normal(ref mut ser) => value.serialize(&mut **ser),
            Compound::Ext(_) => unreachable!(),
        }
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
        match self {
            Compound::Normal(ref mut ser) => value.serialize(&mut **ser),
            Compound::Ext(_) => unreachable!(),
        }
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
        match self {
            Compound::Normal(ref mut ser) => value.serialize(&mut **ser),
            Compound::Ext(ref mut ser) => value.serialize(&mut *ser),
        }
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
        match self {
            Compound::Normal(ref mut ser) => {
                key.serialize(&mut **ser)?;
                value.serialize(&mut **ser)
            }
            Compound::Ext(_) => unreachable!(),
        }
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
        match self {
            Compound::Normal(ref mut ser) => {
                key.serialize(&mut **ser)?;
                value.serialize(&mut **ser)
            }
            Compound::Ext(_) => unreachable!(),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
