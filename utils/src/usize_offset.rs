#[derive(Debug)]
pub enum OffsetErrors {
    OutOfBoundsError,
    IntegerOverflow,
}

pub trait UsizeOffset {
    fn offset(&self, value: isize) -> Result<Self, OffsetErrors>
    where
        Self: Sized;
}

impl UsizeOffset for usize {
    fn offset(&self, value: isize) -> Result<Self, OffsetErrors>
    where
        Self: Sized,
    {
        if value < 0 {
            let abs_offset = value.unsigned_abs();
            if abs_offset > *self {
                return Err(OffsetErrors::OutOfBoundsError);
            }
            Ok(*self - abs_offset)
        } else {
            self.checked_add(value as usize)
                .ok_or(OffsetErrors::IntegerOverflow)
        }
    }
}
