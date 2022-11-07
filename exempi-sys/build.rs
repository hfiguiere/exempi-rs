extern crate pkg_config;

fn main() {
    // We don't want to link this to build docs.rs.
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }
    match pkg_config::find_library("exempi-2.0 >= 2.4.0") {
        Ok(_) => (),
        Err(e) => {
            println!("Exempi not found");
            panic!("{}", e);
        }
    }
}
