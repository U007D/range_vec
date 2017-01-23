use std::ops::Range;
use std::ops::{Deref, DerefMut};

pub struct RangeVec(Vec<Range<usize>>);

impl RangeVec
{
    pub fn new() -> RangeVec
    {
       RangeVec(Vec::new())
    }
}

impl Deref for RangeVec
{
    type Target = Vec<Range<usize>>;
    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

impl DerefMut for RangeVec
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.0
    }
}

#[cfg(test)] mod unit_tests;
