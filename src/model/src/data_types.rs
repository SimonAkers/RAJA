pub struct Integer(pub i32);

impl From<Integer> for u32 {
    fn from(value: Integer) -> Self {
        value.0 as u32
    }
}

impl From<u32> for Integer {
    fn from(value: u32) -> Self {
        Integer(value as i32)
    }
}