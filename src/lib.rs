#![feature(nll)]
#![recursion_limit = "128"]
/* common */
extern crate glenum;

/* wasm32 with stdweb */

#[cfg(all(target_arch = "wasm32",feature = "stdw"))]
#[macro_use]
extern crate stdweb;

#[cfg(all(target_arch = "wasm32",feature = "stdw"))]
mod webgl;

#[cfg(all(target_arch = "wasm32",feature = "stdw"))]
pub use webgl::*;


/* wasm32 with manual ffi */

#[cfg(all(target_arch = "wasm32",not(feature = "stdw")))]
mod webgl_web;

#[cfg(all(target_arch = "wasm32",not(feature = "stdw")))]
pub use webgl_web::*;



/* other than wasm32 */

#[cfg(not(target_arch = "wasm32"))]
extern crate gl;
#[cfg(not(target_arch = "wasm32"))]
mod webgl_native;
#[cfg(not(target_arch = "wasm32"))]
pub use webgl_native::*;



mod common;

pub use glenum::*;
pub use common::*;
