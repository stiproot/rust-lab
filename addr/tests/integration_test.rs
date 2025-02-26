use addr::add;

mod common;

#[test]
fn it_adds() {
    common::setup();

    assert_eq!(add(1, 3), 4);
}
