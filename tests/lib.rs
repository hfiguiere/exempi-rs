extern crate exempi2;
extern crate exempi_sys;

use exempi2::*;
use exempi_sys as c;

#[test]
fn libary_tests() {
    assert!(exempi2::init());

    assert!(exempi2::get_error() == Error::from(c::XmpError::Unknown));

    // namespace registration tests.
    let result = exempi2::register_namespace("http://rust.figuiere.net/ns/rust/", "rust");
    assert!(result != None);
    assert!(exempi2::get_error() == Error::from(c::XmpError::Unknown));
    let prefix = result.unwrap();
    assert!(prefix.to_str() != Some(""));
    let result = exempi2::namespace_prefix("http://rust.figuiere.net/ns/rust/");
    assert!(result != None);
    let prefix2 = result.unwrap();
    assert!(exempi2::get_error() == Error::from(c::XmpError::Unknown));
    assert!(prefix2 == prefix);

    let result = if let Some(prefix) = prefix.to_str() {
        exempi2::prefix_namespace(prefix)
    } else {
        panic!("Prefix couldn't be decoded");
    };
    assert!(result != None);
    let ns = result.unwrap();
    assert!(exempi2::get_error() == Error::from(c::XmpError::Unknown));
    assert!(ns.to_str() == Some("http://rust.figuiere.net/ns/rust/"));

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
    assert!(value.unwrap().to_str() == Some("foobar"));
    assert!(optionbits == PropFlags::NONE);

    let result = xmpblock.serialize(
        SerialFlags::OMITPACKETWRAPPER | SerialFlags::USECOMPACTFORMAT,
        0,
    );
    assert!(result.is_ok());
    println!("{}", result.unwrap().to_string());

    exempi2::terminate();
}
