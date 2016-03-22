extern crate exempi;

use exempi::*;

#[test]
fn libary_tests() {
    assert!(exempi::init());

    assert!(exempi::get_error() == Error::Unknown);

    // namespace registration tests.
    let result = exempi::register_namespace("http://rust.figuiere.net/ns/rust/",
                                     "rust");
    assert!(result != None);
    assert!(exempi::get_error() == Error::Unknown);
    let prefix = result.unwrap();
    assert!(prefix.to_str() != "");
    let result = exempi::namespace_prefix("http://rust.figuiere.net/ns/rust/");
    assert!(result != None);
    let prefix2 = result.unwrap();
    assert!(exempi::get_error() == Error::Unknown);
    assert!(prefix2 == prefix);

    let result = exempi::prefix_namespace(prefix.to_str());
    assert!(result != None);
    let ns = result.unwrap();
    assert!(exempi::get_error() == Error::Unknown);
    assert!(ns.to_str() == "http://rust.figuiere.net/ns/rust/");

    let mut xmpblock = Xmp::new();
    assert!(!xmpblock.is_null());

    assert!(!xmpblock.has_property("http://rust.figuiere.net/ns/rust/", "test"));
    assert!(xmpblock.set_property("http://rust.figuiere.net/ns/rust/", "test",
                                   "foobar", PROP_NONE));
    assert!(xmpblock.has_property("http://rust.figuiere.net/ns/rust/", "test"));
    let mut optionbits: PropFlags = PROP_NONE;
    let value = xmpblock.get_property("http://rust.figuiere.net/ns/rust/",
                                      "test", &mut optionbits);
    assert!(value != None);
    assert!(value.unwrap().to_str() == "foobar");
    assert!(optionbits == PROP_NONE);

    let result = xmpblock.serialize(SERIAL_OMITPACKETWRAPPER |
                                    SERIAL_USECOMPACTFORMAT, 0);
    assert!(result != None);
    println!("{}", result.unwrap().to_str());

    exempi::terminate();
}
