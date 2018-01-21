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
    pub fn log<T: Into<String>>(&self, msg: T) {
        js!{ console.log(@{msg.into()})};
    }

    pub fn new<'a>(canvas: &Element, context: &'a str) -> GLContext {
        let gl = js! { return (@{canvas}).getContext(@{context}); };
        GLContext {
            reference: gl.into_reference().unwrap(),
        }
    }

    pub fn create_buffer(&self) -> WebGLBuffer {
        self.log("create_buffer");
        let value = js! { return (@{self.as_reference()}).createBuffer(); };
        WebGLBuffer(value.into_reference().expect("error: create_buffer"))
    }

    pub fn buffer_data(&self, kind: BufferKind, data: &[u8], draw: DrawMode) {
        self.log("buffer_data");
        js! { (@{&self.as_reference()}).bufferData(@{kind as u32},@{ TypedArray::from(data) }, @{draw as u32}) };
    }

    pub fn bind_buffer(&self, kind: BufferKind, buffer: &WebGLBuffer) {
        self.log("bind_buffer");
        js! { (@{self.as_reference()}).bindBuffer(@{kind as u32},@{buffer.deref()}) };
    }

    pub fn unbind_buffer(&self, kind: BufferKind) {
        self.log("unbind_buffer");
        js! { (@{self.as_reference()}).bindBuffer(@{kind as u32},null) };
    }

    pub fn create_shader(&self, kind: ShaderKind) -> WebGLShader {
        self.log("create_shader");
        let value = js! { return (@{self.as_reference()}).createShader(@{ kind as u32 }); };
        WebGLShader(value.into_reference().unwrap())
    }

    pub fn shader_source(&self, shader: &WebGLShader, code: &str) {
        self.log("shader_source");
        js! { (@{self.as_reference()}).shaderSource(@{shader.deref()},@{ code }) };
    }

    pub fn compile_shader(&self, shader: &WebGLShader) {
        self.log("compile_shader");
        js! { (@{self.as_reference()}).compileShader(@{shader.deref()}) };
        js! {
            var compiled = (@{self.as_reference()}).getShaderParameter(@{shader.deref()}, 0x8B81);
            console.log("Shader compiled successfully:", compiled);
            var compilationLog = (@{self.as_reference()}).getShaderInfoLog(@{shader.deref()});
            console.log("Shader compiler log:",compilationLog);
        };
    }

    pub fn create_program(&self) -> WebGLProgram {
        self.log("create_program");
        let value = js! { return (@{self.as_reference()}).createProgram(); };
        WebGLProgram(value.into_reference().unwrap())
    }

    pub fn link_program(&self, program: &WebGLProgram) {
        self.log("link_program");
        js! { (@{self.as_reference()}).linkProgram(@{program.deref()}) };
    }

    pub fn use_program(&self, program: &WebGLProgram) {
        self.log("use_program");
        js! { (@{self.as_reference()}).useProgram(@{program.deref()}) };
    }

    pub fn attach_shader(&self, program: &WebGLProgram, shader: &WebGLShader) {
        self.log("attach_shader");
        js! { (@{self.as_reference()}).attachShader(@{program.deref()},@{shader.deref()}) };
    }

    pub fn get_attrib_location(&self, program: &WebGLProgram, name: &str) -> Option<u32> {
        self.log("get_attrib_location");
        let value =
            js! { return (@{self.as_reference()}).getAttribLocation(@{program.deref()},@{name}) };
        value.try_into().ok() as _
    }

    pub fn get_uniform_location(
        &self,
        program: &WebGLProgram,
        name: &str,
    ) -> Option<WebGLUniformLocation> {
        self.log("get_uniform_location");
        let value = js! { var res = (@{self.as_reference()}).getUniformLocation(@{program.deref()},@{name});
            console.log(@{name},res);
            return res;
        };
        value
            .into_reference()
            .map(|reference| WebGLUniformLocation{reference,name:name.into()})
    }

    pub fn vertex_attrib_pointer(
        &self,
        location: u32,
        size: AttributeSize,
        kind: DataType,
        normalized: bool,
        stride: u32,
        offset: u32,
    ) {
        self.log("vertex_attribute_pointer");
        let params = js! { return [@{location},@{size as u16},@{kind as i32},@{normalized}] };
        js! {
            var p = @{params};
            (@{self.as_reference()}).vertexAttribPointer(p[0],p[1],p[2],p[3],@{stride},@{offset});
        };
    }

    pub fn enable_vertex_attrib_array(&self, location: u32) {
        self.log("enabled_vertex_attrib_array");
        js! { (@{self.as_reference()}).enableVertexAttribArray(@{location}) };
    }

    pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        self.log("clear_color");
        let params = js! { return [@{r},@{g},@{b},@{a}] };
        js! {
            var p = @{params};
             (@{self.as_reference()}).clearColor(p[0],p[1],p[2],p[3]);
        };
    }

    pub fn enable(&self, flag: Flag) {
        self.log("enable");
        js! { (@{self.as_reference()}).enable(@{flag as i32}) };
    }

    pub fn clear(&self, bit: BufferBit) {
        self.log("clear");
        js! { (@{self.as_reference()}).clear(@{bit as i32}) };
    }

    pub fn viewport(&self, x: i32, y: i32, width: u32, height: u32) {
        self.log("viewport");
        let params = js! { return [@{x},@{y},@{width},@{height}] };
        js! {
            var p = @{params};
            (@{self.as_reference()}).viewport(p[0],p[1],p[2],p[3]);
        };
    }

    pub fn draw_elements(&self, mode: Primitives, count: usize, kind: DataType, offset: u32) {
        self.log("draw_elemnts");
        let params = js! { return [@{count as u32},@{offset}] };
        js! {
            var p = @{params};
            (@{self.as_reference()}).drawElements(@{mode as i32},p[0],@{kind as i32},p[1]);
        };
    }

    pub fn draw_arrays(&self, mode: Primitives, count: usize) {
        self.log("draw_arrays");
        js! {
            (@{self.as_reference()}).drawArrays(@{mode as i32},0,@{count as i32});
        };
    }

    pub fn pixel_storei(&self, storage: PixelStorageMode, value: i32) {
        self.log("pixel_storei");
        js!{
            (@{self.as_reference()}).pixelStorei(@{storage as i32},@{value});
        }
    }

    pub fn tex_image2d(
        &self,
        target: TextureBindPoint,
        level: u8,
        width: u16,
        height: u16,
        format: PixelFormat,
        kind: DataType,
        pixels: &[u8],
    ) {
        self.log("tex_img2d");
        let params1 = js! { return [@{target as u32},@{level as u32},@{format as u32}] };
        let params2 =
            js! { return [@{width as u32},@{height as u32},@{format as u32},@{kind as u32}] };
        js!{
            var p = @{params1}.concat(@{params2});
            (@{self.as_reference()}).texImage2D(p[0],p[1], p[2] ,p[3],p[4],0,p[2],p[6],@{TypedArray::from(pixels)});
        };
    }

    pub fn tex_sub_image2d(
        &self,
        target: TextureBindPoint,
        level: u8,
        xoffset: u16,
        yoffset: u16,
        width: u16,
        height: u16,
        format: PixelFormat,
        kind: DataType,
        pixels: &[u8],
    ) {
        self.log("sub_tex_img2d");
        let params1 =
            js! { return [@{target as u32},@{level as u32},@{xoffset as u32},@{yoffset as u32}] };
        let params2 =
            js! { return [@{width as u32},@{height as u32},@{format as u32},@{kind as u32}] };
        js!{
            var p = @{params1}.concat(@{params2});
            (@{self.as_reference()}).texSubImage2D(p[0],p[1],p[2],p[3],p[4],p[5],p[6],p[7],@{TypedArray::from(pixels)});
        };
    }

    pub fn compressed_tex_image2d(
        &self,
        target: TextureBindPoint,
        level: u8,
        compression: TextureCompression,
        width: u16,
        height: u16,
        data: &[u8],
    ) {
        self.log("compressed_tex_img2d");
        let params =
            js! { return [@{target as u32},@{level as u32},@{width as u32},@{height as u32}] };
        // for some reason this needs to be called otherwise invalid format error, extension initialization?
        js! {
          (@{self.as_reference()}).getExtension("WEBGL_compressed_texture_s3tc") ||
          (@{self.as_reference()}).getExtension("MOZ_WEBGL_compressed_texture_s3tc") ||
          (@{self.as_reference()}).getExtension("WEBKIT_WEBGL_compressed_texture_s3tc")
        };
        js! {

            (@{self.as_reference()}).compressedTexImage2D(
                p[0],
                p[1],
                @{compression as u16},
                p[2],
                p[3],
                0,
                @{TypedArray::from(data)}
            );
        }
    }

    ///
    pub fn create_texture(&self) -> WebGLTexture {
        self.log("create_tex");
        let handle = js!{
           return @{self.as_reference()}.createTexture()
        };
        WebGLTexture(handle.into_reference().unwrap())
    }

    pub fn delete_texture(&self, texture: &WebGLTexture) {
        self.log("delete_tex");
        js!{ (@{self.as_reference()}).deleteTexture(@{&texture.0}) }
    }

    pub fn bind_texture(&self, texture: &WebGLTexture) {
        self.log("bind_tex");
        js!{ (@{self.as_reference()}).bindTexture(@{TextureBindPoint::Texture2d as u32 }, @{&texture.0}) }
    }

    pub fn unbind_texture(&self) {
        self.log("unbind_tex");
        js!{ (@{self.as_reference()}).bindTexture(@{TextureBindPoint::Texture2d as u32 },null) }
    }

    pub fn blend_func(&self, b1: BlendMode, b2: BlendMode) {
        self.log("blend_func");
        js!{ (@{self.as_reference()}).blendFunc(@{b1 as u32},@{b2 as u32}) }
    }

    pub fn uniform_matrix_4fv(&self, location: WebGLUniformLocation, value: &[[f32; 4]; 4]) {
        self.log("uniform_matrix_4fv");
        use std::mem;
        let array = unsafe { mem::transmute::<&[[f32; 4]; 4], &[f32; 16]>(value) as &[f32] };
        js!{ (@{self.as_reference()}).uniformMatrix4fv(@{location.deref()},false,@{&array}) }
    }

    pub fn uniform_matrix_3fv(&self, location: WebGLUniformLocation, value: &[[f32; 3]; 3]) {
        self.log("uniform_matrix_3fv");
        use std::mem;
        let array = unsafe { mem::transmute::<&[[f32; 3]; 3], &[f32; 9]>(value) as &[f32] };
        js!{ (@{self.as_reference()}).uniformMatrix3fv(@{location.deref()},false,@{&array}) }
    }

    pub fn uniform_matrix_2fv(&self, location: WebGLUniformLocation, value: &[[f32; 2]; 2]) {
        use std::mem;
        let array = unsafe { mem::transmute::<&[[f32; 2]; 2], &[f32; 4]>(value) as &[f32] };
        js!{ (@{self.as_reference()}).uniformMatrix2fv(@{location.deref()},false,@{&array}) }
    }

    pub fn uniform_1i(&self, location: WebGLUniformLocation, value: i32) {
        js!{ (@{self.as_reference()}).uniform1i(@{location.deref()},@{value}) }
    }

    pub fn uniform_1f(&self, location: WebGLUniformLocation, value: f32) {
        js!{ (@{self.as_reference()}).uniform1f(@{location.deref()},@{value}) }
    }

    pub fn uniform_4f(&self, location: WebGLUniformLocation, value: (f32, f32, f32, f32)) {
        js!{
            var p = [@{value.0},@{value.1},@{value.2},@{value.3}];
        }
        js!{ (@{self.as_reference()}).uniform4f(@{location.deref()},p[0],p[1],p[2],p[3]) }
    }

    pub fn create_vertex_array(&self) -> WebGLVertexArray {
        self.log("create_vertex_array");
        let val = js! { return (@{&self.as_reference()}).createVertexArray() };
        WebGLVertexArray(val.into_reference().unwrap())
    }

    pub fn bind_vertex_array(&self, vao: WebGLVertexArray) {
        self.log("bind_vertex_array");
        js! { (@{&self.as_reference()}).bindVertexArray(@{vao.deref()}) }
    }

    pub fn unbind_vertex_array(&self) {
        self.log("unbind_vertex_array");
        js! { (@{&self.as_reference()}).bindVertexArray(0) }
    }

    pub fn get_program_parameter(&self, program: &WebGLProgram, pname: ShaderParameter) -> i32 {
        let res = js! { return (@{&self.as_reference()}).getProgramParameter(@{program.deref()},@{pname as u32}); };
        res.try_into().unwrap()
    }

    pub fn get_active_uniform(&self, program: &WebGLProgram, location: u32) -> WebGLActiveInfo {
        let res =
            js! { return @{self.as_reference()}.getActiveUniform(@{program.deref()},@{location}) };
        let name = js! { return @{&res}.name };
        let size = js!{ return @{&res}.size };
        let kind = js!{ return @{&res}.type };
        let k: u32 = kind.try_into().unwrap();
        use std::mem;
        WebGLActiveInfo::new(
            name.into_string().unwrap(),
            size.try_into().unwrap(),
            unsafe { mem::transmute::<u16, UniformType>(k as _) },
            res.into_reference().unwrap(),
        )
    }

    pub fn get_active_attrib(&self, program: &WebGLProgram, location: u32) -> WebGLActiveInfo {
        let res =
            js! { return @{self.as_reference()}.getActiveAttrib(@{program.deref()},@{location}) };
        let name = js! { return @{&res}.name };
        let size = js!{ return @{&res}.size };
        let kind = js!{ return @{&res}.type };
        let k: u32 = kind.try_into().unwrap();
        use std::mem;
        WebGLActiveInfo::new(
            name.into_string().unwrap(),
            size.try_into().unwrap(),
            unsafe { mem::transmute::<u16, UniformType>(k as _) },
            res.into_reference().unwrap(),
        )
    }

    pub fn tex_parameteri(&self, pname: TextureParameter, param: i32) {
        js! { return @{self.as_reference()}.texParameteri(@{TextureBindPoint::Texture2d as u32},@{pname as u32},@{param}) };
    }

    pub fn tex_parameterfv(&self, pname: TextureParameter, param: f32) {
        js! { return @{self.as_reference()}.texParameterf(@{TextureBindPoint::Texture2d as u32},@{pname as u32},@{param}) };
    }
}