#!no_std]
// Simple example that doesn't depend on any particular embedded hardware that shows
// how you can use this library

use log::info;

fn main() {
    colog::init();
    info!("Hello!");
}
