pub fn hello() -> String

{
    format!("Hello, {}-bit world!", 0usize.count_zeros())
}

#[allow(dead_code)]
fn private() -> bool
{
    true
}

#[cfg(test)] mod unit_tests;
