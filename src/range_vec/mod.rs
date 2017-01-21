pub struct RangeVec;

impl RangeVec
{
    pub fn new() -> RangeVec
    {
        RangeVec {}
    }

    pub fn len(&self) -> usize
    {
        0
    }
}

#[cfg(test)] mod unit_tests;
