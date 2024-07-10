//! [FromWire] and [IntoWire] for protobuf scalars

use crate::{Error, FromWire, IntoWire, Message, VarInt, WireType, WireTypeView};

impl FromWire for f64 {
    #[inline(always)]
    fn from_wire(wire: WireTypeView) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match wire {
            WireTypeView::FixedI64(data) => {
                let array: [u8; 8] = data.try_into().expect("I64 is always 8 bytes");
                Ok(f64::from_le_bytes(array))
            }
            _ => Err(Error::UnexpectedWireType),
        }
    }
}

impl IntoWire for f64 {
    #[inline(always)]
    fn into_wire(self) -> WireType {
        WireType::FixedI64(self.to_le_bytes())
    }

    fn size_hint(&self, tag: u32) -> usize {
        8 + tag.required_space() as usize
    }
}

impl FromWire for f32 {
    #[inline(always)]
    fn from_wire(wire: WireTypeView) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match wire {
            WireTypeView::FixedI32(data) => {
                let array: [u8; 4] = data.try_into().expect("I32 is always 4 bytes");
                Ok(f32::from_le_bytes(array))
            }
            _ => Err(Error::UnexpectedWireType),
        }
    }
}

impl IntoWire for f32 {
    #[inline(always)]
    fn into_wire(self) -> WireType {
        WireType::FixedI32(self.to_le_bytes())
    }

    fn size_hint(&self, tag: u32) -> usize {
        4 + tag.required_space() as usize
    }
}

impl FromWire for u64 {
    #[inline(always)]
    fn from_wire(wire: WireTypeView) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match wire {
            WireTypeView::VarInt(data) => {
                let (value, _) = u64::decode_var(data).ok_or(Error::InvalidVarInt)?;
                Ok(value)
            }
            _ => Err(Error::UnexpectedWireType),
        }
    }
}

impl IntoWire for u64 {
    #[inline(always)]
    fn into_wire(self) -> WireType {
        let mut data = [0u8; 10];
        let size = self.encode_var(&mut data);
        WireType::VarInt(data, size)
    }

    fn size_hint(&self, tag: u32) -> usize {
        self.required_space() as usize + tag.required_space() as usize
    }
}

impl FromWire for i64 {
    #[inline(always)]
    fn from_wire(wire: WireTypeView) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match wire {
            WireTypeView::VarInt(data) => {
                let (value, _) = i64::decode_var(data).ok_or(Error::InvalidVarInt)?;
                Ok(value)
            }
            _ => Err(Error::UnexpectedWireType),
        }
    }
}

impl IntoWire for i64 {
    #[inline(always)]
    fn into_wire(self) -> WireType {
        let mut data = [0u8; 10];
        let size = self.encode_var(&mut data);
        WireType::VarInt(data, size)
    }

    fn size_hint(&self, tag: u32) -> usize {
        self.required_space() as usize + tag.required_space() as usize
    }
}

impl FromWire for u32 {
    #[inline(always)]
    fn from_wire(wire: WireTypeView) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match wire {
            WireTypeView::VarInt(data) => {
                let (value, _) = u32::decode_var(data).ok_or(Error::InvalidVarInt)?;
                Ok(value)
            }
            _ => Err(Error::UnexpectedWireType),
        }
    }
}

impl IntoWire for u32 {
    #[inline(always)]
    fn into_wire(self) -> WireType {
        let mut data = [0u8; 10];
        let size = self.encode_var(&mut data);
        WireType::VarInt(data, size)
    }

    fn size_hint(&self, tag: u32) -> usize {
        self.required_space() as usize + tag.required_space() as usize
    }
}

impl FromWire for i32 {
    #[inline(always)]
    fn from_wire(wire: WireTypeView) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match wire {
            WireTypeView::VarInt(data) => {
                let (value, _) = i32::decode_var(data).ok_or(Error::InvalidVarInt)?;
                Ok(value)
            }
            _ => Err(Error::UnexpectedWireType),
        }
    }
}

impl IntoWire for i32 {
    #[inline(always)]
    fn into_wire(self) -> WireType {
        let mut data = [0u8; 10];
        let size = self.encode_var(&mut data);
        WireType::VarInt(data, size)
    }

    fn size_hint(&self, tag: u32) -> usize {
        self.required_space() as usize + tag.required_space() as usize
    }
}

impl FromWire for String {
    #[inline(always)]
    fn from_wire(wire: WireTypeView) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match wire {
            WireTypeView::LengthEncoded(data) => Ok(String::from_utf8(data.to_vec())?),
            _ => Err(Error::UnexpectedWireType),
        }
    }
}

impl IntoWire for String {
    #[inline(always)]
    fn into_wire(self) -> WireType {
        WireType::LengthEncoded(self.into_bytes().into())
    }

    fn size_hint(&self, tag: u32) -> usize {
        let len = self.len();
        len.required_space() as usize + tag.required_space() as usize + len
    }
}

impl FromWire for bool {
    #[inline(always)]
    fn from_wire(wire: WireTypeView) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match wire {
            WireTypeView::VarInt(data) => {
                let (value, _) = i32::decode_var(data).ok_or(Error::InvalidVarInt)?;
                Ok(value != 0)
            }
            _ => Err(Error::UnexpectedWireType),
        }
    }
}

impl IntoWire for bool {
    #[inline(always)]
    fn into_wire(self) -> WireType {
        let mut data = [0u8; 10];
        let size = if self { 1u32 } else { 0u32 }.encode_var(&mut data);

        WireType::VarInt(data, size)
    }

    fn size_hint(&self, tag: u32) -> usize {
        if *self { 1u32 } else { 0u32 }.required_space() as usize + tag.required_space() as usize
    }
}

impl<T> IntoWire for T
where
    T: Message,
{
    fn into_wire(self) -> WireType {
        let size = self.size_hint();
        let mut buffer = smallvec::SmallVec::<[u8; 1024]>::new();
        buffer.resize(size, 0);
        let mut buffer_ref = buffer.as_mut_slice();
        self.serialize(&mut buffer_ref);
        let written = size - buffer_ref.len();
        WireType::LengthEncoded(bytes::Bytes::copy_from_slice(&buffer[0..written]))
    }

    fn size_hint(&self, tag: u32) -> usize {
        // println!("size hint {} + {}", tag.required_space(), self.size_hint());
        tag.required_space() as usize + self.size_hint()
    }
}

impl<T> FromWire for T
where
    T: Message,
{
    fn from_wire(wire: WireTypeView) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match wire {
            WireTypeView::LengthEncoded(data) => {
                let (value, _) = T::deserialize(data)?;
                Ok(value)
            }
            _ => Err(Error::UnexpectedWireType),
        }
    }
}
