use super::super::*;

#[test]
fn virgin_range_vec_has_zero_len()
{
    let rvec = RangeVec::new();
    assert!(rvec.len() == 0);
}
