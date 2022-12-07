#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

use std::fs::read;

use cargo_toml::Manifest;

fn main() {
    let m = Manifest::from_slice(&read("Cargo.toml").unwrap()).unwrap();
    let version = m.package.as_ref().unwrap().version.to_owned();
    println!("cargo:rustc-env=CFA_VERSION=v{version}");
}
