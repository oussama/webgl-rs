#![feature(nll)]
#![recursion_limit="128"]
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

    use glenum::*;
    use std::ops::Deref;

    #[cfg(not(target_arch = "wasm32"))]
    use webgl_native::*;
    #[cfg(target_arch = "wasm32")]
    use webgl::*;

    #[derive(Debug,PartialEq)]
    pub struct GLContext {
        pub reference: Reference,
    }

    #[derive(Debug)]
    pub struct WebGLRenderingContext {
        pub common: GLContext,
    }

    #[derive(Debug)]
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

    #[derive(Debug)]
    pub struct WebGLBuffer(pub Reference);

    impl Deref for WebGLBuffer {
        type Target = Reference;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    #[derive(Debug)]
    pub struct WebGLShader(pub Reference);
    impl Deref for WebGLShader {
        type Target = Reference;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    #[derive(Debug,PartialEq)]
    pub struct WebGLProgram(pub Reference);
    impl Deref for WebGLProgram {
        type Target = Reference;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    #[derive(Debug)]
    pub struct WebGLActiveInfo {
        reference:Reference,
        name:String,
        size:u32,
        kind:UniformType
    }

    impl WebGLActiveInfo {
        pub fn new<T:Into<String>>(name:T,size:u32,kind:UniformType,reference:Reference) -> WebGLActiveInfo {
            let nam = name.into();
            WebGLActiveInfo {
                reference,
                name:nam,
                size,kind
            }
        }
    }

    #[derive(Debug,PartialEq)]
    pub struct WebGLTexture(pub Reference);
    impl Deref for WebGLTexture {
        type Target = Reference;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    #[derive(Debug)]
    pub struct WebGLVertexArray(pub Reference);
    impl Deref for WebGLVertexArray {
        type Target = Reference;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    

}


pub use common::*;
