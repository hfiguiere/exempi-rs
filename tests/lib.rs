extern crate exempi;
extern crate exempi_sys;

use exempi_sys as c;
use exempi::*;

#[test]
fn libary_tests() {
    assert!(exempi::init());

    assert!(exempi::get_error() == Error::from(c::XmpError::Unknown));

    // namespace registration tests.
    let result = exempi::register_namespace("http://rust.figuiere.net/ns/rust/", "rust");
    assert!(result != None);
    assert!(exempi::get_error() == Error::from(c::XmpError::Unknown));
    let prefix = result.unwrap();
    assert!(prefix.to_str() != "");
    let result = exempi::namespace_prefix("http://rust.figuiere.net/ns/rust/");
    assert!(result != None);
    let prefix2 = result.unwrap();
    assert!(exempi::get_error() == Error::from(c::XmpError::Unknown));
    assert!(prefix2 == prefix);

    let result = exempi::prefix_namespace(prefix.to_str());
    assert!(result != None);
    let ns = result.unwrap();
    assert!(exempi::get_error() == Error::from(c::XmpError::Unknown));
    assert!(ns.to_str() == "http://rust.figuiere.net/ns/rust/");

    let mut xmpblock = Xmp::new();
    assert!(!xmpblock.is_null());

    assert!(!xmpblock.has_property("http://rust.figuiere.net/ns/rust/", "test"));
    assert!(xmpblock
        .set_property(
            "http://rust.figuiere.net/ns/rust/",
            "test",
            "foobar",
            PropFlags::NONE
        )
        .is_ok());
    assert!(xmpblock.has_property("http://rust.figuiere.net/ns/rust/", "test"));
    let mut optionbits: PropFlags = PropFlags::NONE;
    let value = xmpblock.get_property("http://rust.figuiere.net/ns/rust/", "test", &mut optionbits);
    assert!(value.is_ok());
    assert!(value.unwrap().to_str() == "foobar");
    assert!(optionbits == PropFlags::NONE);

    let result = xmpblock.serialize(
        SerialFlags::OMITPACKETWRAPPER | SerialFlags::USECOMPACTFORMAT,
        0,
    );
    assert!(result.is_ok());
    println!("{}", result.unwrap().to_str());

    exempi::terminate();
}
