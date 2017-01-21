use hello::*;

#[test]
fn hello_returns_greeting()
{
    assert!( hello().is_empty() == false );
}

#[test]
fn can_access_private()
{
    assert!( private() == true );
}
