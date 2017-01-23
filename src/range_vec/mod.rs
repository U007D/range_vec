use std::ops::Range;
use std::ops::{Deref, DerefMut};

pub struct RangeVec
{
    range_vec: Vec<Range<usize>>,
}

impl RangeVec
{
    pub fn new() -> RangeVec
    {
       RangeVec{ range_vec: Vec::new()}
    }
}

impl Deref for RangeVec
{
    type Target = Vec<Range<usize>>;
    fn deref(&self) -> &Self::Target
    {
        &self.range_vec
    }
}

impl DerefMut for RangeVec
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.range_vec
    }
}

#[cfg(test)] mod unit_tests;
