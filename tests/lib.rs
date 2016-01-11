extern crate exempi;

use exempi::*;

#[test]
fn libary_tests() {
    assert!(exempi::init());

    assert!(exempi::get_error() == 0);

    // namespace registration tests.
    let mut prefix = XmpString::new();
    assert!(!prefix.is_null());

    assert!(exempi::register_namespace("http://rust.figuiere.net/ns/rust/",
                                       "rust", &mut prefix));
    assert!(exempi::get_error() == 0);
    assert!(prefix.to_str() != "");
    let mut prefix2 = XmpString::new();
    assert!(exempi::namespace_prefix("http://rust.figuiere.net/ns/rust/",
                                     &mut prefix2));
    assert!(exempi::get_error() == 0);
    assert!(prefix2 == prefix);

    let mut ns = XmpString::new();
    assert!(exempi::prefix_namespace(prefix.to_str(), &mut ns));
    assert!(exempi::get_error() == 0);
    assert!(ns.to_str() == "http://rust.figuiere.net/ns/rust/");

    let mut xmpblock = Xmp::new();
    assert!(!xmpblock.is_null());

    assert!(!xmpblock.has_property("http://rust.figuiere.net/ns/rust/", "test"));
    assert!(xmpblock.set_property("http://rust.figuiere.net/ns/rust/", "test",
                                   "foobar", 0));
    assert!(xmpblock.has_property("http://rust.figuiere.net/ns/rust/", "test"));
    let mut value = XmpString::new();
    let mut optionbits: u32 = 0;
    assert!(xmpblock.get_property("http://rust.figuiere.net/ns/rust/", "test",
                                  &mut value, &mut optionbits));
    assert!(value.to_str() == "foobar");
    assert!(optionbits == 0);

    let mut buffer = XmpString::new();
    // XXX we should use the constants... that we need to define.
    assert!(xmpblock.serialize(&mut buffer, 0x50, 0));
    println!("{}", buffer.to_str());

    exempi::terminate();
}
