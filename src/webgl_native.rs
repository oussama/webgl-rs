use std::os::raw::c_void;
use std::ops::Deref;
use glenum::*;
use gl;
use std::ffi::CString;

pub type Reference = u32;


use common::*;


pub fn check_gl_error() {
    unsafe {
        use gl;
        let err = gl::GetError();
        if err != 0 {
            println!("gl_err {}", err);
        }
    }
}

impl WebGLRenderingContext {
    pub fn new(_: &isize) -> WebGLRenderingContext {
        WebGLRenderingContext {
            common: GLContext { reference: 0 },
        }
    }

    pub fn buffer_data(&self, kind: BufferKind, data: &[u8], draw: DrawMode) {
        unsafe {
            gl::BufferData(kind as _, data.len() as _, data.as_ptr() as _, draw as _);
        }
        check_gl_error();
    }

    pub fn load_with<F>(loadfn: F) where F: FnMut(&str) -> *const c_void,
    {
        gl::load_with(loadfn);
    }
}

impl WebGL2RenderingContext {
    pub fn new(canvas: &isize) -> WebGL2RenderingContext {
        WebGL2RenderingContext {
            common: GLContext { reference: 0 },
        }
    }
}



impl GLContext {
    pub fn create_buffer(&self) -> WebGLBuffer {
        let mut buffer = WebGLBuffer { reference: 0 };
        unsafe {
            gl::GenBuffers(1, &mut buffer.reference);
        }
        check_gl_error();
        buffer
    }

    pub fn bind_buffer(&self, kind: BufferKind, buffer: &WebGLBuffer) {
        unsafe {
            gl::BindBuffer(kind as _, *buffer.as_reference());
        }
        check_gl_error();
    }

    pub fn unbind_buffer(&self, kind: BufferKind) {
        unsafe {
            gl::BindBuffer(kind as _, 0);
        }
        check_gl_error();
    }

    pub fn create_shader(&self, kind: ShaderKind) -> WebGLShader {
        check_gl_error();
        unsafe {
            WebGLShader {
                reference: gl::CreateShader(kind as _),
            }
        }
    }

    pub fn shader_source(&self, shader: &WebGLShader, source: &str) {
        let src = CString::new(source).unwrap();
        unsafe {
            use std::ptr;
            gl::ShaderSource(*shader.as_reference(), 1, &src.as_ptr(), ptr::null());
        }
        check_gl_error();
    }

    pub fn compile_shader(&self, shader: &WebGLShader) {
        unsafe {
            gl::CompileShader(*shader.as_reference());
        }
        check_gl_error();
    }

    pub fn create_program(&self) -> WebGLProgram {
        check_gl_error();
        unsafe {
            WebGLProgram {
                reference: gl::CreateProgram(),
            }
        }
    }

    pub fn link_program(&self, program: &WebGLProgram) {
        unsafe {
            gl::LinkProgram(*program.as_reference());
        }
        check_gl_error();
    }

    pub fn use_program(&self, program: &WebGLProgram) {
        unsafe {
            gl::UseProgram(*program.as_reference());
        }
        check_gl_error();
    }

    pub fn attach_shader(&self, program: &WebGLProgram, shader: &WebGLShader) {
        unsafe {
            gl::AttachShader(*program.as_reference(), *shader.as_reference());
        }
        check_gl_error();
    }

    pub fn get_attrib_location(&self, program: &WebGLProgram, name: String) -> Option<i32> {
        let c_name = CString::new(name).unwrap();
        unsafe {
            let location = gl::GetAttribLocation(*program.as_reference() as _, c_name.as_ptr());
            check_gl_error();
            if location == -1 {
                return None;
            }
            {
                return Some(location);
            }
        }
        
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
        unsafe {
            gl::VertexAttribPointer(
                location as _,
                size as _,
                kind as u32,
                normalized as u8,
                stride as _,
                offset as _,
            );
        }
        check_gl_error();
    }

    pub fn enable_vertex_attrib_array(&self, location: i32) {
        unsafe {
            gl::EnableVertexAttribArray(location as _);
        }
        check_gl_error();
    }

    pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
        check_gl_error();
    }

    pub fn enable(&self, flag: Flag) {
        unsafe {
            gl::Enable(flag as _);
        }
        check_gl_error();
    }

    pub fn clear(&self, bit: BufferBit) {
        unsafe {
            gl::Clear(bit as _);
        }
        check_gl_error();
    }

    pub fn viewport(&self, x: i32, y: i32, width: u32, height: u32) {
        unsafe {
            gl::Viewport(x, y, width as _, height as _);
        };
        check_gl_error();
    }

    pub fn draw_elements(&self, mode: Primitives, count: usize, kind: DataType, offset: u32) {
        unsafe {
            gl::DrawElements(mode as _, count as _, kind as _, offset as _);
        };
        check_gl_error();
    }
}
