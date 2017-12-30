#![feature(nll)]

/* common */
extern crate glenum;

/* wasm32 */
#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;


#[cfg(target_arch = "wasm32")]
mod webgl;
#[cfg(target_arch = "wasm32")]
pub use webgl::*;


/* other than wasm32 */
#[cfg(not(target_arch = "wasm32"))]
extern crate gl;
#[cfg(not(target_arch = "wasm32"))]
mod webgl_native;
#[cfg(not(target_arch = "wasm32"))]
pub use webgl_native::*;


pub use glenum::*;

pub mod common {

    use std::ops::Deref;

    #[cfg(not(target_arch = "wasm32"))]
    use webgl_native::*;
    #[cfg(target_arch = "wasm32")]
    use webgl::*;

    pub struct GLContext {
        pub reference: Reference,
    }

    pub struct WebGLRenderingContext {
        pub common: GLContext,
    }

    pub struct WebGL2RenderingContext {
        pub common: GLContext,
    }


    pub trait AsReference {
        fn as_reference(&self) -> &Reference;
    }

    impl AsReference for GLContext {
        fn as_reference(&self) -> &Reference {
            &self.reference
        }
    }

    impl AsReference for WebGLRenderingContext {
        fn as_reference(&self) -> &Reference {
            &self.common.as_reference()
        }
    }

    impl AsReference for WebGL2RenderingContext {
        fn as_reference(&self) -> &Reference {
            &self.common.as_reference()
        }
    }


    impl Deref for WebGLRenderingContext {
        type Target = GLContext;
        fn deref(&self) -> &GLContext {
            &self.common
        }
    }

    impl Deref for WebGL2RenderingContext {
        type Target = GLContext;
        fn deref(&self) -> &GLContext {
            &self.common
        }
    }

    pub struct WebGLBuffer {
        pub reference: Reference,
    }



    impl WebGLBuffer {
        pub fn as_reference(&self) -> &Reference {
            &self.reference
        }
    }


    pub struct WebGLShader {
        pub reference: Reference,
    }

    impl WebGLShader {
        pub fn as_reference(&self) -> &Reference {
            &self.reference
        }
    }

    pub struct WebGLProgram {
        pub reference: Reference,
    }

    impl WebGLProgram {
        pub fn as_reference(&self) -> &Reference {
            &self.reference
        }
    }


}

pub use common::*;
