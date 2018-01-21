#[cfg(not(target_arch = "wasm32"))]
use webgl_native::*;
#[cfg(all(target_arch = "wasm32",feature = "stdw"))]
use webgl::*;
#[cfg(all(target_arch = "wasm32",not(feature = "stdw")))]
use webgl_web::*;


use glenum::*;
use std::ops::Deref;


#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct WebGLProgram(pub Reference);
impl Deref for WebGLProgram {
    type Target = Reference;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct WebGLActiveInfo {
    reference: Reference,
    name: String,
    size: u32,
    kind: UniformType,
}

impl WebGLActiveInfo {
    pub fn new<T: Into<String>>(
        name: T,
        size: u32,
        kind: UniformType,
        reference: Reference,
    ) -> WebGLActiveInfo {
        let nam = name.into();
        WebGLActiveInfo {
            reference,
            name: nam,
            size,
            kind,
        }
    }
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug,Clone, PartialEq)]
pub struct WebGLUniformLocation{
    pub reference:Reference,
    pub name:String,
}

impl Deref for WebGLUniformLocation {
    type Target = Reference;
    fn deref(&self) -> &Self::Target {
        &self.reference
    }
}

impl WebGLUniformLocation {
    pub fn new(name:String,reference:Reference) -> WebGLUniformLocation {
        WebGLUniformLocation {
            name,
            reference
        }
    }
}