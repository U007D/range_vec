use super::*;

#[test]
fn virgin_range_vec_has_zero_len()
{
    let rangeVec = RangeVec::new();
    assert!(rangeVec.len() == 0);
}
