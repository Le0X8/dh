use std::mem::size_of;

// marker trait
pub trait Primitive<T, const S: usize> {
    fn from_ne_bytes(bytes: [u8; S]) -> Self;
    fn from_le_bytes(bytes: [u8; S]) -> Self;
    fn from_be_bytes(bytes: [u8; S]) -> Self;

    fn to_ne_bytes(self) -> [u8; S];
    fn to_le_bytes(self) -> [u8; S];
    fn to_be_bytes(self) -> [u8; S];
}

// trait implementation macro
macro_rules! impl_primitive {
    ($type:ty, $type_upper:ident) => {
        const $type_upper: usize = size_of::<$type>();

        impl Primitive<$type, $type_upper> for $type {
            fn from_ne_bytes(bytes: [u8; $type_upper]) -> $type {
                <$type>::from_ne_bytes(bytes)
            }

            fn from_le_bytes(bytes: [u8; $type_upper]) -> $type {
                <$type>::from_le_bytes(bytes)
            }

            fn from_be_bytes(bytes: [u8; $type_upper]) -> $type {
                <$type>::from_be_bytes(bytes)
            }

            fn to_ne_bytes(self) -> [u8; $type_upper] {
                self.to_ne_bytes()
            }

            fn to_le_bytes(self) -> [u8; $type_upper] {
                self.to_le_bytes()
            }

            fn to_be_bytes(self) -> [u8; $type_upper] {
                self.to_be_bytes()
            }
        }
    };
}

// unsigned integers
impl_primitive!(u8, U8);
impl_primitive!(u16, U16);
impl_primitive!(u32, U32);
impl_primitive!(u64, U64);
impl_primitive!(u128, U128);
impl_primitive!(usize, USIZE);

// signed integers
impl_primitive!(i8, I8);
impl_primitive!(i16, I16);
impl_primitive!(i32, I32);
impl_primitive!(i64, I64);
impl_primitive!(i128, I128);
impl_primitive!(isize, ISIZE);

// floating point numbers
impl_primitive!(f32, F32);
impl_primitive!(f64, F64);

// other primitives (char is not handled as a primitive in dh!)
impl<const S: usize> Primitive<[u8; S], S> for [u8; S] {
    fn from_ne_bytes(bytes: [u8; S]) -> [u8; S] {
        bytes
    }

    fn from_le_bytes(bytes: [u8; S]) -> [u8; S] {
        bytes
    }

    fn from_be_bytes(bytes: [u8; S]) -> [u8; S] {
        bytes
    }

    fn to_ne_bytes(self) -> [u8; S] {
        self
    }

    fn to_le_bytes(self) -> [u8; S] {
        self
    }

    fn to_be_bytes(self) -> [u8; S] {
        self
    }
}
impl Primitive<bool, 1> for bool {
    fn from_ne_bytes(bytes: [u8; 1]) -> bool {
        bytes[0] & 1 == 1
    }

    fn from_le_bytes(bytes: [u8; 1]) -> bool {
        bytes[0] & 1 == 1
    }

    fn from_be_bytes(bytes: [u8; 1]) -> bool {
        bytes[0] & 1 == 1
    }

    fn to_ne_bytes(self) -> [u8; 1] {
        [if self { 1 } else { 0 }]
    }

    fn to_le_bytes(self) -> [u8; 1] {
        [if self { 1 } else { 0 }]
    }

    fn to_be_bytes(self) -> [u8; 1] {
        [if self { 1 } else { 0 }]
    }
}
impl Primitive<(), 0> for () {
    fn from_ne_bytes(_: [u8; 0]) {}
    fn from_le_bytes(_: [u8; 0]) {}
    fn from_be_bytes(_: [u8; 0]) {}

    fn to_ne_bytes(self) -> [u8; 0] {
        []
    }
    fn to_le_bytes(self) -> [u8; 0] {
        []
    }
    fn to_be_bytes(self) -> [u8; 0] {
        []
    }
}
