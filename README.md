exempi2
=======

This is a renaming of the crate previously known as `exempi`

The crate provide safe Rust binding for Exempi, an XMP library that
provide a stable ABI for Adobe XMP Toolkit.

Exempi2 can be found at http://libopenraw.freedesktop.org/wiki/Exempi/

Exempi-sys provide unsafe bindings to the C API.

Source
------

exempi-rs source code is at:
https://github.com/hfiguiere/exempi-rs

Dependencies
------------

- Rust 2018 edition (tested, other versions, YMMV)
- exempi 2.4 must be installed and findable with pkg_config
- crates (pulled by cargo):
 - libc
 - pkg-config
 - flagbits

License
-------

2-clauses BSD. See the LICENSE file enclosed.

Note: Exempi is under the original 3-clauses BSD license. Be aware of
this if shipping your Rust program.

Maintainer
----------

Hubert Figuière <hub@figuiere.net>
