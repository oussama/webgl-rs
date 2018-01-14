#![feature(nll)]

#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;

#[cfg(not(target_arch = "wasm32"))]
#[macro_use]
extern crate glutin;

extern crate application;
extern crate byteorder;
extern crate webgl;

mod utils;
mod dds;
mod simple;
mod compressed_texture;

fn main() {
    //simple::main();
    compressed_texture::main();
}
