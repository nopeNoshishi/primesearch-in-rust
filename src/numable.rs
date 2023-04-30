//! Numable trait for primitive number

pub trait Numable {
    fn to_num(self) -> usize;
}

impl Numable for u8 {
    fn to_num(self) -> usize {
        self as usize
    }
}

impl Numable for i8 {
    fn to_num(self) -> usize {
        self as usize
    }
}

impl Numable for u16 {
    fn to_num(self) -> usize {
        self as usize
    }
}

impl Numable for i16 {
    fn to_num(self) -> usize {
        self as usize
    }
}

impl Numable for u32 {
    fn to_num(self) -> usize {
        self as usize
    }
}

impl Numable for i32 {
    fn to_num(self) -> usize {
        self as usize
    }
}

impl Numable for u64 {
    fn to_num(self) -> usize {
        self as usize
    }
}

impl Numable for i64 {
    fn to_num(self) -> usize {
        self as usize
    }
}

impl Numable for u128 {
    fn to_num(self) -> usize {
        self as usize
    }
}

impl Numable for i128 {
    fn to_num(self) -> usize {
        self as usize
    }
}
