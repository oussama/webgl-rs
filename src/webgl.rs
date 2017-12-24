use std::ops::Deref;
use stdweb::Reference;
use stdweb::web::*;
use stdweb::unstable::TryInto;
use glenum::*;

pub struct WebGLRenderingContext {
    common:WebGLRenderingContextCommon,
}

pub struct WebGL2RenderingContext {
    common:WebGLRenderingContextCommon,
}

pub trait AsReference {
    fn as_reference(&self) -> &Reference;
}

impl AsReference for WebGLRenderingContext {
    fn as_reference(&self) -> &Reference {
        &self.common.gl
    }
}

impl AsReference for WebGL2RenderingContext {
    fn as_reference(&self) -> &Reference {
        &self.common.gl
    }
}

pub struct WebGLBuffer {
    reference: Reference,
}

impl WebGLBuffer {
    pub fn as_reference(&self) -> &Reference {
        &self.reference
    }
}


pub struct WebGLShader {
    reference: Reference,
}

impl WebGLShader {
    pub fn as_reference(&self) -> &Reference {
        &self.reference
    }
}

pub struct WebGLProgram {
    reference: Reference,
}

impl WebGLProgram {
    pub fn as_reference(&self) -> &Reference {
        &self.reference
    }
}


impl WebGLRenderingContext {
    pub fn new(canvas: &Element) -> WebGLRenderingContext {
        WebGLRenderingContext {
            common: WebGLRenderingContextCommon::new(canvas, "webgl"),
        }
    }

    pub fn buffer_data_ptr(&self, kind: BufferKind, data_ptr: &[u8], draw: DrawMode) {
        js! { (@{&self.as_reference()}).bufferData(@{kind as u32},@{ data_ptr }, @{draw as u32}) };
    }

    pub fn buffer_data(&self, kind: BufferKind, data: &[u8], draw: DrawMode) {
        js! { (@{&self.as_reference()}).bufferData(@{kind as u32},@{ TypedArray::from(data) }, @{draw as u32}) };
    }
}

impl WebGL2RenderingContext {
    pub fn new(canvas: &Element) -> WebGL2RenderingContext {
        WebGL2RenderingContext {
            common: WebGLRenderingContextCommon::new(canvas, "webgl2"),
        }
    }
}

impl Deref for WebGLRenderingContext {
    type Target = WebGLRenderingContextCommon;
    fn deref(&self) -> &WebGLRenderingContextCommon {
        &self.common
    }
}

impl Deref for WebGL2RenderingContext {
    type Target = WebGLRenderingContextCommon;
    fn deref(&self) -> &WebGLRenderingContextCommon {
        &self.common
    }
}

pub struct WebGLRenderingContextCommon {
    gl:Reference,
}

impl WebGLRenderingContextCommon {

    pub fn new<'a>(canvas: &Element,context:&'a str) -> WebGLRenderingContextCommon {
        let gl = js! { return (@{canvas}).getContext(@{context}); };
        WebGLRenderingContextCommon {
            gl: gl.into_reference().unwrap(),
        }
    }

    pub fn create_buffer(&self) -> WebGLBuffer {
        let value = js! { return (@{&self.gl}).createBuffer(); };
        WebGLBuffer {
            reference: value.into_reference().unwrap(),
        }
    }

    pub fn bind_buffer(&self, kind: BufferKind, buffer: &WebGLBuffer) {
        js! { (@{&self.gl}).bindBuffer(@{kind as u32},@{buffer.as_reference()}) };
    }

    pub fn unbind_buffer(&self, kind: BufferKind) {
        js! { (@{&self.gl}).bindBuffer(@{kind as u32},null) };
    }




    pub fn create_shader(&self, kind: ShaderKind) -> WebGLShader {
        let value = js! { return (@{&self.gl}).createShader(@{ kind as u32 }); };
        WebGLShader {
            reference: value.into_reference().unwrap(),
        }
    }

    pub fn shader_source(&self, shader: &WebGLShader, code: &str) {
        js! { (@{&self.gl}).shaderSource(@{shader.as_reference()},@{ code }) };
    }

    pub fn compile_shader(&self, shader: &WebGLShader) {
        js! { (@{&self.gl}).compileShader(@{shader.as_reference()}) };
    }

    pub fn create_program(&self) -> WebGLProgram {
        let value = js! { return (@{&self.gl}).createProgram(); };
        WebGLProgram {
            reference: value.into_reference().unwrap(),
        }
    }

    pub fn link_program(&self, program: &WebGLProgram) {
        js! { (@{&self.gl}).linkProgram(@{program.as_reference()}) };
    }

    pub fn use_program(&self, program: &WebGLProgram) {
        js! { (@{&self.gl}).useProgram(@{program.as_reference()}) };
    }

    pub fn attach_shader(&self, program: &WebGLProgram, shader: &WebGLShader) {
        js! { (@{&self.gl}).attachShader(@{program.as_reference()},@{shader.as_reference()}) };
    }

    pub fn get_attrib_location(&self, program: &WebGLProgram, name: String) -> Option<i32> {
        let value = js! { return (@{&self.gl}).getAttribLocation(@{program.as_reference()},@{name}) };
        value.try_into().ok()
    }

    pub fn vertex_attrib_pointer(
        &self,
        location: i32,
        size: i32,
        kind: DataType,
        normalized: bool,
        stride: i32,
        offset: i32,
    ) {
        let params = js! { return [@{location},@{size},@{kind as i32},@{normalized}] };
        js! {
            var p = @{params};
            (@{&self.gl}).vertexAttribPointer(p[0],p[1],p[2],p[3],@{stride},@{offset});
        };
    }

    pub fn enable_vertex_attrib_array(&self, location: i32) {
        js! { (@{&self.gl}).enableVertexAttribArray(@{location}) };
    }

    pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        let params = js! { return [@{r},@{g},@{b},@{a}] };
        js! {
            var p = @{params};
             (@{&self.gl}).clearColor(p[0],p[1],p[2],p[3]);
        };
    }

    pub fn enable(&self, flag: Flag) {
        js! { (@{&self.gl}).enable(@{flag as i32}) };
    }

    pub fn clear(&self, bit: BufferBit) {
        js! { (@{&self.gl}).clear(@{bit as i32}) };
    }

    pub fn viewport(&self, x: f32, y: f32, width: f32, height: f32) {
        let params = js! { return [@{x},@{y},@{width},@{height}] };
        js! {
            var p = @{params};
            (@{&self.gl}).viewport(p[0],p[1],p[2],p[3]);
        };
    }

    pub fn draw_elements(&self,mode:Primitives, count:usize,kind:DataType,offset:u32) {
        let params = js! { return [@{count as u32},@{offset}] };
        js! {
            var p = @{params};
            (@{&self.gl}).drawElements(@{mode as i32},p[0],@{kind as i32},p[1]);
        };
    }
}
