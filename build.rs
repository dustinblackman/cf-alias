#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

use vergen::{vergen, Config};

fn main() {
    vergen(Config::default()).unwrap();
}
