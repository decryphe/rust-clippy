use std::fmt::Write;
use std::path::Path;

use crate::utils::{UpdateMode, clippy_version, replace_region_in_file};

const CARGO_TOML_FILES: [&str; 4] = [
    "clippy_config/Cargo.toml",
    "clippy_lints/Cargo.toml",
    "clippy_utils/Cargo.toml",
    "Cargo.toml",
];

pub fn bump_version() {
    let (minor, mut patch) = clippy_version();
    patch += 1;
    for file in &CARGO_TOML_FILES {
        replace_region_in_file(
            UpdateMode::Change,
            Path::new(file),
            "# begin autogenerated version\n",
            "# end autogenerated version",
            |res| {
                writeln!(res, "version = \"0.{minor}.{patch}\"").unwrap();
            },
        );
    }
}