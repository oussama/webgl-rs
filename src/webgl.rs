use std::ops::Deref;
pub use stdweb::Reference;
use stdweb::web::*;
use stdweb::unstable::TryInto;
use glenum::*;
use common::*;


impl WebGLRenderingContext {
    pub fn new(canvas: &Element) -> WebGLRenderingContext {
        WebGLRenderingContext {
            common: GLContext::new(canvas, "webgl"),
        }
    }

    pub fn buffer_data_ptr(&self, kind: BufferKind, data_ptr: &[u8], draw: DrawMode) {
        js! { (@{&self.as_reference()}).bufferData(@{kind as u32},@{ data_ptr }, @{draw as u32}) };
    }
}

impl WebGL2RenderingContext {

    pub fn new(canvas: &Element) -> WebGL2RenderingContext {
        WebGL2RenderingContext {
            common: GLContext::new(canvas, "webgl2"),
        }
    }

}


impl GLContext {

    pub fn new<'a>(canvas: &Element,context:&'a str) -> GLContext {
        let gl = js! { return (@{canvas}).getContext(@{context}); };
        GLContext {
            reference: gl.into_reference().unwrap(),
        }
    }

    pub fn create_buffer(&self) -> WebGLBuffer {
        let value = js! { return (@{self.as_reference()}).createBuffer(); };
        WebGLBuffer {
            reference: value.into_reference().unwrap(),
        }
    }

    pub fn buffer_data(&self, kind: BufferKind, data: &[u8], draw: DrawMode) {
        js! { (@{&self.as_reference()}).bufferData(@{kind as u32},@{ TypedArray::from(data) }, @{draw as u32}) };
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

    pub fn get_attrib_location(&self, program: &WebGLProgram, name: &str) -> Option<u32> {
        let value = js! { return (@{self.as_reference()}).getAttribLocation(@{program.as_reference()},@{name}) };
        value.try_into().ok() as _
    }

    pub fn get_uniform_location(&self, program: &WebGLProgram, name: &str) -> Option<u32> {
        let value = js! { return (@{self.as_reference()}).getUniformLocation(@{program.as_reference()},@{name}) };
        value.try_into().ok() as _
    }

    pub fn vertex_attrib_pointer(
        &self,
        location: u32,
        size: u32,
        kind: DataType,
        normalized: bool,
        stride: u32,
        offset: u32,
    ) {
        let params = js! { return [@{location},@{size},@{kind as i32},@{normalized}] };
        js! {
            var p = @{params};
            (@{self.as_reference()}).vertexAttribPointer(p[0],p[1],p[2],p[3],@{stride},@{offset});
        };
    }

    pub fn enable_vertex_attrib_array(&self, location: u32) {
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

    pub fn draw_arrays(&self, mode: Primitives, count: usize) {
        js! {
            (@{self.as_reference()}).drawArrays(@{mode as i32},0,@{count});
        };
    }

    pub fn pixel_storei(&self,storage:PixelStorageMode,value:i32) {
        js!{
            (@{self.as_reference()}).pixelStorei(@{storage as i32},@{value});
        }
    }

    pub fn tex_sub_image2d(&self,target:TextureBindPoint,level:u8,xoffset:u16,yoffset:u16,width:u16,height:u16,format:PixelFormat,kind:DataType,pixels:&[u8]) {
        let params1 = js! { return [@{target as u32},@{level as u32},@{xoffset as u32},@{yoffset as u32}] };
        let params2 = js! { return [@{width as u32},@{height as u32},@{format as u32},@{kind as u32}] };
        js!{
            var p = @{params1}.concat(@{params2});
            (@{self.as_reference()}).pixelStorei(@{p[0]},@{p[1]},@{p[2]},@{p[3]},@{p[4]},@{p[5]},@{p[6]},@{p[7]},@{pixels});
        }
    }

        pub fn compressed_tex_image2d(
        &self,
        target:TextureBindPoint,
        level:u8,
        compression:TextureCompression,
        width:u16,
        height:u16,
        data:&[u8]
    ) {
        let params = js! { return [@{target as u32},@{level as u32},@{width as u32},@{height as u32}] };
        js! {
            var p = @{params};
            (@{self.as_reference()}).compressedTexImage2D(
                p[0],
                p[1],
                @{compression as u32},
                p[2],
                p[3],
                0,
                @{data}
            );
        }
    }




    ///
    pub fn create_texture(&self)-> WebGLTexture {
        let handle = js!{
           return (@{self.as_reference()}).createTexture() })
        }
        WebGLTexture(handle)
    }

    pub fn delete_texture(&self,texture:WebGLTexture) {
        js!{ (@{self.as_reference()}).deleteTexture(@{texture.0}) }) }
    }

    pub fn bind_texture(&self,texture:&WebGLTexture) {
        js!{ (@{self.as_reference()}).bindTexture(@{texture.0}) }) }
    }

    pub fn unbind_texture(&self) {
        js!{ (@{self.as_reference()}).bindTexture(null) }) }
    }

    pub fn blend_func(&self,b1:BlendMode,b2:BlendMode) {
        js!{ (@{self.as_reference()}).blendFunc(@{b1 as u32},@{b2 as u32}) }) }
    }

    
    pub fn uniform_matrix_4fv(&self,location:u32,value:&[[f32; 4]; 4]){
        js!{ (@{self.as_reference()}).uniformMatrix4fv(@{location},@{&value[0] as _}) }) }
    }


    pub fn uniform_matrix_3fv(&self, location: u32, value: &[[f32; 3]; 3]) {
        unsafe {
            gl::UniformMatrix3fv(location as i32, 1, false as _, &value[0] as _);
        }
    }

    pub fn uniform_matrix_2fv(&self, location: u32, value: &[[f32; 2]; 2]) {
        unsafe {
            gl::UniformMatrix2fv(location as i32, 1, false as _, &value[0] as _);
        }
    }

    pub fn uniform_1i(&self, location: u32, value: i32) {
        unsafe {
            gl::Uniform1i(location as i32, value as _);
        }
    }

    pub fn uniform_1f(&self, location: u32, value: f32) {
        unsafe {
            gl::Uniform1f(location as i32, value as _);
        }
    }

    pub fn uniform_4f(&self, location: u32, value: (f32, f32, f32, f32)) {
        unsafe {
            gl::Uniform4f(location as i32, value.0, value.1, value.2, value.3);
        }
    }



    pub fn create_vertex_array(&self) -> WebGLVertexArray {
        let reference = js! { return (@{&self.as_reference()}).createVertexArray() }
        WebGLVertexArray(reference)
    }

    pub fn bind_vertex_array(&self,vao:WebGLVertexArray) {
        js! { (@{&self.as_reference()}).bindVertexArray(@{vao}) }
    }

    pub fn unbind_vertex_array(&self) {
        js! { (@{&self.as_reference()}).bindVertexArray(0) }
    }
}
