use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let output_file = PathBuf::from(crate_dir.clone()).join("include").join("libminiaes.h");
    cbindgen::generate(crate_dir).unwrap()
        .write_to_file(output_file);
}
