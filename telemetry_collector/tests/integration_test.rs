use collectorlib;

use collectorlib::show_hello_world;

#[test]
fn test_hello_world() {
    assert_eq!("Hello World", show_hello_world())
}
