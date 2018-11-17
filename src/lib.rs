#![recursion_limit = "128"]
/* common */
extern crate glenum;
/* wasm32 with stdweb */

#[cfg(all(target_arch = "wasm32"))]
extern crate web_sys;

/* wasm32 with manual ffi */

#[cfg(all(target_arch = "wasm32"))]
pub use web_sys::*;

/* other than wasm32 */

#[cfg(not(target_arch = "wasm32"))]
extern crate gl;
#[cfg(not(target_arch = "wasm32"))]
mod webgl_native;
#[cfg(not(target_arch = "wasm32"))]
pub use webgl_native::*;
#[cfg(not(target_arch = "wasm32"))]
mod common;

mod tests {}

pub use glenum::*;
