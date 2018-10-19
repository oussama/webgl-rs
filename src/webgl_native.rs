use std::ops::Deref;

use common::{WebGLBuffer, WebGLVertexArray};
use gl;
use glenum::*;
use std::ffi::CString;
use std::os::raw::c_void;

pub type Reference = u32;

const NAME_SIZE: usize = 64;

use common::*;

pub fn check_gl_error(msg: &str) {
    unsafe {
        use gl;
        let err = gl::GetError();
        if err != 0 {
            println!("GLError: {} {}", msg, err);
        }
    }
}

impl WebGLRenderingContext {
    pub fn new(_canvas: &isize) -> WebGLRenderingContext {
        WebGLRenderingContext {
            common: GLContext::new(),
        }
    }

    pub fn load_with<F>(loadfn: F)
    where
        F: FnMut(&str) -> *const c_void,
    {
        gl::load_with(loadfn);
    }
}

impl WebGL2RenderingContext {
    pub fn new(_canvas: &isize) -> WebGL2RenderingContext {
        WebGL2RenderingContext {
            common: GLContext::new(),
        }
    }

    pub fn load_with<F>(loadfn: F)
    where
        F: FnMut(&str) -> *const c_void,
    {
        gl::load_with(loadfn);
    }
}

impl GLContext {
    pub fn new() -> GLContext {
        //  unsafe { gl::Enable(gl::DEPTH_TEST) };

        GLContext { reference: 0 }
    }
    pub fn create_buffer(&self) -> WebGLBuffer {
        let mut buffer = WebGLBuffer(0);
        unsafe {
            gl::GenBuffers(1, &mut buffer.0);
        }
        check_gl_error("create_buffer");
        buffer
    }

    pub fn bind_buffer(&self, kind: BufferKind, buffer: &WebGLBuffer) {
        unsafe {
            gl::BindBuffer(kind as _, buffer.0);
        }
        check_gl_error("bind_buffer");
    }

    pub fn buffer_data(&self, kind: BufferKind, data: &[u8], draw: DrawMode) {
        unsafe {
            gl::BufferData(kind as _, data.len() as _, data.as_ptr() as _, draw as _);
        }
        check_gl_error("buffer_data");
    }

    pub fn buffer_sub_data(&self, kind: BufferKind, offset: u32, data: &[u8]) {
        unsafe {
            gl::BufferSubData(kind as _, offset as _, data.len() as _, data.as_ptr() as _);
        }
        check_gl_error("buffer_data");
    }

    pub fn unbind_buffer(&self, kind: BufferKind) {
        unsafe {
            gl::BindBuffer(kind as _, 0);
        }
        check_gl_error("unbind_buffer");
    }

    pub fn create_shader(&self, kind: ShaderKind) -> WebGLShader {
        check_gl_error("create_shader");
        unsafe { WebGLShader(gl::CreateShader(kind as _)) }
    }

    pub fn shader_source(&self, shader: &WebGLShader, source: &str) {
        let src = CString::new(source).unwrap();
        unsafe {
            use std::ptr;
            gl::ShaderSource(shader.0, 1, &src.as_ptr(), ptr::null());
        }
        check_gl_error("shader_source");
    }

    pub fn compile_shader(&self, shader: &WebGLShader) {
        unsafe {
            gl::CompileShader(shader.0);
        }
        check_gl_error("compile_shader");
    }

    pub fn create_program(&self) -> WebGLProgram {
        let p = unsafe { WebGLProgram(gl::CreateProgram()) };
        check_gl_error("create_program");
        p
    }

    pub fn link_program(&self, program: &WebGLProgram) {
        unsafe {
            gl::LinkProgram(program.0);
        }
        check_gl_error("link_program");
    }

    pub fn use_program(&self, program: &WebGLProgram) {
        unsafe {
            gl::UseProgram(program.0);
        }
        check_gl_error("use_program");
    }

    pub fn attach_shader(&self, program: &WebGLProgram, shader: &WebGLShader) {
        unsafe {
            gl::AttachShader(program.0, shader.0);
        }
        check_gl_error("attach_shader");
    }

    pub fn get_attrib_location(&self, program: &WebGLProgram, name: &str) -> Option<u32> {
        let c_name = CString::new(name).unwrap();
        unsafe {
            let location = gl::GetAttribLocation(program.0 as _, c_name.as_ptr());
            check_gl_error("get_attrib_location");
            if location == -1 {
                return None;
            }
            return Some(location as _);
        }
    }

    pub fn get_uniform_location(
        &self,
        program: &WebGLProgram,
        name: &str,
    ) -> Option<WebGLUniformLocation> {
        let c_name = CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(program.0 as _, c_name.as_ptr());
            check_gl_error("get_uniform_location");
            if location == -1 {
                return None;
            }
            return Some(WebGLUniformLocation {
                reference: location as _,
                name: name.into(),
            });
        }
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
        unsafe {
            gl::VertexAttribPointer(
                location as _,
                size as _,
                kind as _,
                normalized as _,
                stride as _,
                offset as _,
            );
        }
        check_gl_error("vertex_attrib_pointer");
    }

    pub fn enable_vertex_attrib_array(&self, location: u32) {
        unsafe {
            gl::EnableVertexAttribArray(location as _);
        }
        check_gl_error("enable_vertex_attrib_array");
    }

    pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
        check_gl_error("clear_color");
    }

    pub fn enable(&self, flag: Flag) {
        unsafe {
            gl::Enable(flag as _);
        }
        check_gl_error("enable");
    }

    pub fn clear(&self, bit: BufferBit) {
        unsafe {
            gl::Clear(bit as _);
        }
        check_gl_error("clear");
    }

    pub fn viewport(&self, x: i32, y: i32, width: u32, height: u32) {
        unsafe {
            gl::Viewport(x, y, width as _, height as _);
        };
        check_gl_error("viewport");
    }

    pub fn draw_elements(&self, mode: Primitives, count: usize, kind: DataType, offset: u32) {
        unsafe {
            gl::DrawElements(mode as _, count as _, kind as _, offset as _);
        };
        check_gl_error("draw_elements");
    }

    pub fn draw_arrays(&self, mode: Primitives, count: usize) {
        unsafe {
            gl::DrawArrays(mode as _, 0, count as _);
        };
        check_gl_error("draw_arrays");
    }

    pub fn pixel_storei(&self, storage: PixelStorageMode, value: i32) {
        unsafe {
            gl::PixelStorei(storage as _, value);
        }
        check_gl_error("pixel_storei");
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
        unsafe {
            gl::TexImage2D(
                target as _,
                level as _,
                format as _,
                width as _,
                height as _,
                0,
                format as _,
                kind as _,
                pixels.as_ptr() as _,
            );
        }
        check_gl_error("tex_image2d");
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
        unsafe {
            gl::TexSubImage2D(
                target as _,
                level as _,
                xoffset as _,
                yoffset as _,
                width as _,
                height as _,
                format as _,
                kind as _,
                pixels.as_ptr() as _,
            );
        }
        check_gl_error("tex_sub_image2d");
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
        unsafe {
            gl::CompressedTexImage2D(
                target as _,
                level as _,
                compression as _,
                width as _,
                height as _,
                0,
                (data.len() - 128) as _, //gl::UNSIGNED_BYTE as _,
                &data[128] as *const u8 as _,
            );
        }
        check_gl_error("compressed_tex_image2d");
    }

    pub fn get_program_parameter(&self, program: &WebGLProgram, pname: ShaderParameter) -> i32 {
        let mut res = 0;
        unsafe {
            gl::GetProgramiv(program.0, pname as _, &mut res);
        }
        check_gl_error("get_program_parameter");
        res
    }

    pub fn get_active_uniform(&self, program: &WebGLProgram, location: u32) -> WebGLActiveInfo {
        let mut name: Vec<u8> = Vec::with_capacity(NAME_SIZE);
        let mut size = 0i32;
        let mut len = 0i32;
        let mut kind = 0u32;

        unsafe {
            gl::GetActiveUniform(
                program.0,
                location as _,
                NAME_SIZE as _,
                &mut len,
                &mut size,
                &mut kind,
                name.as_mut_ptr() as _,
            );
            name.set_len(len as _);
        };
        check_gl_error("get_active_uniform");
        use std::mem;

        WebGLActiveInfo::new(
            String::from_utf8(name).unwrap(),
            //location as _,
            size as _,
            unsafe { mem::transmute::<u16, UniformType>(kind as _) },
            0
            //unsafe { mem::transmute::<u16, DataType>(kind as _) },
        )
    }

    pub fn get_active_attrib(&self, program: &WebGLProgram, location: u32) -> WebGLActiveInfo {
        let mut name: Vec<u8> = Vec::with_capacity(NAME_SIZE);
        let mut size = 0i32;
        let mut len = 0i32;
        let mut kind = 0u32;

        unsafe {
            gl::GetActiveAttrib(
                program.0,
                location as _,
                NAME_SIZE as _,
                &mut len,
                &mut size,
                &mut kind,
                name.as_mut_ptr() as _,
            );
            name.set_len(len as _);
        }
        println!("name {:?}", name);
        use std::mem;
        check_gl_error("get_active_attrib");
        //let c_name = unsafe { CString::from_raw(name[0..(len+1)].as_mut_ptr())};
        WebGLActiveInfo::new(
            String::from_utf8(name).expect("utf8 parse failed"),
            //location,
            size as _,
            //DataType::Float
            unsafe { mem::transmute::<u16, UniformType>(kind as _) },
            0,
        )
    }

    ///
    pub fn create_texture(&self) -> WebGLTexture {
        let mut handle = WebGLTexture(0);
        unsafe {
            gl::GenTextures(1, &mut handle.0);
        }
        check_gl_error("create_texture");
        handle
    }

    pub fn delete_texture(&self, texture: &WebGLTexture) {
        unsafe {
            gl::DeleteTextures(1, texture.0 as _);
        }
        check_gl_error("delete_texture");
    }

    pub fn bind_texture(&self, texture: &WebGLTexture) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture.0);
        }
        check_gl_error("bind_texture");
    }

    pub fn unbind_texture(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        check_gl_error("unbind_texture");
    }

    pub fn blend_func(&self, b1: BlendMode, b2: BlendMode) {
        unsafe {
            gl::BlendFunc(b1 as _, b2 as _);
        }
        check_gl_error("blend_func");
    }

    pub fn uniform_matrix_4fv(&self, location: WebGLUniformLocation, value: &[[f32; 4]; 4]) {
        unsafe {
            gl::UniformMatrix4fv(*location.deref() as i32, 1, false as _, &value[0] as _);
        }
        check_gl_error("uniform_matrix_4fv");
    }

    pub fn uniform_matrix_3fv(&self, location: WebGLUniformLocation, value: &[[f32; 3]; 3]) {
        unsafe {
            gl::UniformMatrix3fv(*location.deref() as i32, 1, false as _, &value[0] as _);
        }
        check_gl_error("uniform_matrix_3fv");
    }

    pub fn uniform_matrix_2fv(&self, location: WebGLUniformLocation, value: &[[f32; 2]; 2]) {
        unsafe {
            gl::UniformMatrix2fv(*location.deref() as i32, 1, false as _, &value[0] as _);
        }
        check_gl_error("uniform_matrix_2fv");
    }

    pub fn uniform_1i(&self, location: WebGLUniformLocation, value: i32) {
        unsafe {
            gl::Uniform1i(*location as i32, value as _);
        }
        check_gl_error("uniform_1i");
    }

    pub fn uniform_1f(&self, location: WebGLUniformLocation, value: f32) {
        unsafe {
            gl::Uniform1f(*location as i32, value as _);
        }
        check_gl_error("uniform_1f");
    }

    pub fn uniform_4f(&self, location: WebGLUniformLocation, value: (f32, f32, f32, f32)) {
        unsafe {
            gl::Uniform4f(*location.deref() as _, value.0, value.1, value.2, value.3);
        }
        check_gl_error("uniform_4f");
    }

    pub fn uniform_4fv(&self, location: WebGLUniformLocation, value: &[f32; 4]) {
        unsafe {
            gl::Uniform4fv(*location.deref() as _, 1 as _, &value[0] as _);
        }
        check_gl_error("uniform_4fv");
    }

    pub fn tex_parameteri(&self, pname: TextureParameter, param: i32) {
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, pname as _, param);
        }
        check_gl_error("tex_parameteri");
    }

    pub fn tex_parameterfv(&self, pname: TextureParameter, param: f32) {
        unsafe {
            gl::TexParameterfv(gl::TEXTURE_2D, pname as _, &param);
        }
        check_gl_error("tex_parameterfv");
    }

    pub fn create_vertex_array(&self) -> WebGLVertexArray {
        let mut vao = WebGLVertexArray(0);
        unsafe {
            gl::GenVertexArrays(1, &mut vao.0);
        }
        check_gl_error("create_vertex_array");
        vao
    }

    pub fn bind_vertex_array(&self, vao: WebGLVertexArray) {
        unsafe {
            gl::BindVertexArray(vao.0);
        }
        check_gl_error("bind_vertex_array");
    }

    pub fn unbind_vertex_array(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
        check_gl_error("unbind_vertex_array");
    }
}
