//
// Copyright (c) 2016-2021, Hubert Figuière
//
// License: BSD-2-Clause
// See top-level LICENSE file.
//

extern crate exempi2;
extern crate exempi_sys;

use exempi2::*;

#[test]
fn libary_tests() {
    // namespace registration tests.
    let result = exempi2::register_namespace("http://rust.figuiere.net/ns/rust/", "rust");
    assert!(result.is_ok());
    let prefix = result.unwrap();
    assert!(prefix.to_str() != Ok(""));
    let result = exempi2::namespace_prefix("http://rust.figuiere.net/ns/rust/");
    assert!(result.is_ok());
    let prefix2 = result.unwrap();
    assert!(prefix2 == prefix);

    let result = if let Ok(prefix) = prefix.to_str() {
        exempi2::prefix_namespace(prefix)
    } else {
        panic!("Prefix couldn't be decoded");
    };
    assert!(result.is_ok());
    let ns = result.unwrap();
    assert!(ns.to_str() == Ok("http://rust.figuiere.net/ns/rust/"));

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
    assert!(value.unwrap().to_str() == Ok("foobar"));
    assert!(optionbits == PropFlags::NONE);

    let result = xmpblock.serialize(
        SerialFlags::OMITPACKETWRAPPER | SerialFlags::USECOMPACTFORMAT,
        0,
    );
    assert!(result.is_ok());
    println!("{}", result.unwrap().to_string());
}
