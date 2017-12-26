use std::ops::Deref;
pub use stdweb::Reference;
use stdweb::web::*;
use stdweb::unstable::TryInto;
use glenum::*;
use common::*;


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


impl WebGLRenderingContextCommon {

    pub fn new<'a>(canvas: &Element,context:&'a str) -> WebGLRenderingContextCommon {
        let gl = js! { return (@{canvas}).getContext(@{context}); };
        WebGLRenderingContextCommon {
            reference: gl.into_reference().unwrap(),
        }
    }

    pub fn create_buffer(&self) -> WebGLBuffer {
        let value = js! { return (@{self.as_reference()}).createBuffer(); };
        WebGLBuffer {
            reference: value.into_reference().unwrap(),
        }
    }

    pub fn bind_buffer(&self, kind: BufferKind, buffer: &WebGLBuffer) {
        js! { (@{self.as_reference()}).bindBuffer(@{kind as u32},@{buffer.as_reference()}) };
    }

    pub fn unbind_buffer(&self, kind: BufferKind) {
        js! { (@{self.as_reference()}).bindBuffer(@{kind as u32},null) };
    }




    pub fn create_shader(&self, kind: ShaderKind) -> WebGLShader {
        let value = js! { return (@{self.as_reference()}).createShader(@{ kind as u32 }); };
        WebGLShader {
            reference: value.into_reference().unwrap(),
        }
    }

    pub fn shader_source(&self, shader: &WebGLShader, code: &str) {
        js! { (@{self.as_reference()}).shaderSource(@{shader.as_reference()},@{ code }) };
    }

    pub fn compile_shader(&self, shader: &WebGLShader) {
        js! { (@{self.as_reference()}).compileShader(@{shader.as_reference()}) };
    }

    pub fn create_program(&self) -> WebGLProgram {
        let value = js! { return (@{self.as_reference()}).createProgram(); };
        WebGLProgram {
            reference: value.into_reference().unwrap(),
        }
    }

    pub fn link_program(&self, program: &WebGLProgram) {
        js! { (@{self.as_reference()}).linkProgram(@{program.as_reference()}) };
    }

    pub fn use_program(&self, program: &WebGLProgram) {
        js! { (@{self.as_reference()}).useProgram(@{program.as_reference()}) };
    }

    pub fn attach_shader(&self, program: &WebGLProgram, shader: &WebGLShader) {
        js! { (@{self.as_reference()}).attachShader(@{program.as_reference()},@{shader.as_reference()}) };
    }

    pub fn get_attrib_location(&self, program: &WebGLProgram, name: String) -> Option<i32> {
        let value = js! { return (@{self.as_reference()}).getAttribLocation(@{program.as_reference()},@{name}) };
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
            (@{self.as_reference()}).vertexAttribPointer(p[0],p[1],p[2],p[3],@{stride},@{offset});
        };
    }

    pub fn enable_vertex_attrib_array(&self, location: i32) {
        js! { (@{self.as_reference()}).enableVertexAttribArray(@{location}) };
    }

    pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        let params = js! { return [@{r},@{g},@{b},@{a}] };
        js! {
            var p = @{params};
             (@{self.as_reference()}).clearColor(p[0],p[1],p[2],p[3]);
        };
    }

    pub fn enable(&self, flag: Flag) {
        js! { (@{self.as_reference()}).enable(@{flag as i32}) };
    }

    pub fn clear(&self, bit: BufferBit) {
        js! { (@{self.as_reference()}).clear(@{bit as i32}) };
    }

    pub fn viewport(&self, x: i32, y: i32, width: u32, height: u32) {
        let params = js! { return [@{x},@{y},@{width},@{height}] };
        js! {
            var p = @{params};
            (@{self.as_reference()}).viewport(p[0],p[1],p[2],p[3]);
        };
    }

    pub fn draw_elements(&self,mode:Primitives, count:usize,kind:DataType,offset:u32) {
        let params = js! { return [@{count as u32},@{offset}] };
        js! {
            var p = @{params};
            (@{self.as_reference()}).drawElements(@{mode as i32},p[0],@{kind as i32},p[1]);
        };
    }
}
