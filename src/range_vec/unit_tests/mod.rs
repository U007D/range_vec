use super::*;

#[test]
fn virgin_range_vec_has_zero_len()
{
    let rangeVec = RangeVec::new();
    assert!(rangeVec.len() == 0);
}

#[test]
fn range_vec_with_one_valid_range_has_len_one()
{
    let rangeVec = RangeVec {0: vec![(1..101)]};
    assert!(rangeVec.len() == 1);
}
