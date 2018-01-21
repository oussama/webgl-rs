#![feature(nll)]

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
