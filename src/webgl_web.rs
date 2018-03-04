use std::ops::Deref;
use glenum::*;
use common::*;


pub type Reference = u32;

impl WebGLRenderingContext {
    pub fn new(canvas_id: &str) -> WebGLRenderingContext {
        WebGLRenderingContext {
            common: GLContext::new(canvas_id, "webgl"),
        }
    }
/*
    pub fn buffer_data_ptr(&self, kind: BufferKind, data_ptr: &[u8], draw: DrawMode) {
        js! { (&self.as_reference()}).bufferData(kind as u32}, data_ptr }, draw as u32}) };
    }*/
}

impl WebGL2RenderingContext {
    pub fn new(canvas: &str) -> WebGL2RenderingContext {
        WebGL2RenderingContext {
            common: GLContext::new(canvas, "webgl2"),
        }
    }
}

impl GLContext {
    pub fn log<T: Into<String>>(&self, msg: T) {
       // js!{ console.log(msg.into()})};
    }

    pub fn new<'a>(canvas: &str, context: &'a str) -> GLContext {
        // for some reason this needs to be called otherwise invalid format error, extension initialization?
/*        let ext = js! { return (
          (&gl}).getExtension("WEBGL_compressed_texture_s3tc") ||
          (&gl}).getExtension("MOZ_WEBGL_compressed_texture_s3tc") ||
          (&gl}).getExtension("WEBKIT_WEBGL_compressed_texture_s3tc")
        )};*/
        GLContext {
            reference: unsafe { get_context(canvas.as_ptr(),canvas.len() as _) },
        }
    }

    pub fn create_buffer(&self) -> WebGLBuffer {
        WebGLBuffer( unsafe { create_buffer() })
    }

    pub fn buffer_data(&self, kind: BufferKind, data: &[u8], draw: DrawMode) {
         unsafe { buffer_data(kind as _,data.as_ptr() as _,data.len() as _ ,draw as _) };
    }

    pub fn buffer_sub_data(&self, kind: BufferKind,offset:u32, data: &[u8]) {
//        let slice = unsafe { UnsafeTypedArray::new( data ) };
 //       js! { (&self.as_reference()}).bufferSubData(kind as u32},offset}, slice }) };
    }

    pub fn bind_buffer(&self, kind: BufferKind, buffer: &WebGLBuffer) {
         unsafe { bind_buffer(kind as _,buffer.0) };
    }

    pub fn unbind_buffer(&self, kind: BufferKind) {
        self.log("unbind_buffer");
    }

    pub fn create_shader(&self, kind: ShaderKind) -> WebGLShader {
        WebGLShader( unsafe { create_shader(kind as _) })
    }

    pub fn shader_source(&self, shader: &WebGLShader, code: &str) {
         unsafe { shader_source(shader.0,code.as_ptr(),code.len() as u32) };
    }

    pub fn compile_shader(&self, shader: &WebGLShader) {
         unsafe { compile_shader(shader.0) };
    }

    pub fn create_program(&self) -> WebGLProgram {
        WebGLProgram( unsafe { create_program()})
    }

    pub fn link_program(&self, program: &WebGLProgram) {
         unsafe { link_program(program.0)};
    }

    pub fn use_program(&self, program: &WebGLProgram) {
         unsafe { use_program(program.0)};
    }

    
    pub fn attach_shader(&self, program: &WebGLProgram, shader: &WebGLShader) {
         unsafe { attach_shader(program.0,shader.0)};
    }

    pub fn get_attrib_location(&self, program: &WebGLProgram, name: &str) -> Option<u32> {
        let location = unsafe { get_attrib_location(program.0,name.as_ptr() as _,name.len() as _) };
        if location >= 0 {
            Some(location as _)
        }else{
            None
        }
    }

    pub fn get_uniform_location(
        &self,
        program: &WebGLProgram,
        name: &str,
    ) -> Option<WebGLUniformLocation> {
        let location = unsafe {  get_uniform_location(program.0,name.as_ptr(),name.len() as _) };
        if location == -1 {
            None
        }else{
            Some(WebGLUniformLocation::new(name.into(),location as _))
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
         unsafe { vertex_attrib_pointer(location,size as _,kind as _,normalized,stride,offset) };
    }

    pub fn enable_vertex_attrib_array(&self, location: u32) {
         unsafe { enable_vertex_attrib_array(location) };
    }

    pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
         unsafe { clear_color(r,g,b,a) };
    }

    pub fn enable(&self, flag: Flag) {
        unsafe { enable(flag as _)};
    }

    pub fn clear(&self, bit: BufferBit) {
        unsafe { clear(bit as _)};
    }

    pub fn viewport(&self, x: i32, y: i32, width: u32, height: u32) {
         unsafe { viewport(x,y,width,height)};
    }

    pub fn draw_elements(&self, mode: Primitives, count: usize, kind: DataType, offset: u32) {
         unsafe { draw_elements(mode as _,count as _,kind as _,offset) };
    }

    pub fn draw_arrays(&self, mode: Primitives, count: usize) {
         unsafe { draw_arrays(mode as _,0,count as _) };
    }

    pub fn pixel_storei(&self, storage: PixelStorageMode, value: i32) {
         unsafe { pixel_storei(storage as _,value as _) };
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
         unsafe { tex_image2d(target as _,level as _,format as _ ,width as _,
            height as _,0,format as _,kind as _,pixels.as_ptr(),pixels.len() as _);
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
        unsafe { tex_sub_image2d(target as _,level as _,xoffset as _,yoffset as _,
            width as _,height as _,format as _,kind as _,pixels.as_ptr(),pixels.len() as _)
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
         unsafe { compressed_tex_image2d(
                target as _,
                level as _,
                compression as _,
                width as _,
                height as _,
                0,
                data.as_ptr(),
                data.len() as _
            );
        }
    }

    ///
    pub fn create_texture(&self) -> WebGLTexture {
        unsafe { WebGLTexture(create_texture()) }
    }

    pub fn delete_texture(&self, texture: &WebGLTexture) {
        self.log("delete_tex");
        //js!{ (self.as_reference()}).deleteTexture(&texture.0}) }
    }

    pub fn bind_texture(&self, texture: &WebGLTexture) {
         unsafe { bind_texture(TextureBindPoint::Texture2d as _,texture.0)};
    }

    pub fn unbind_texture(&self) {
        self.log("unbind_tex");
        //js!{ (self.as_reference()}).bindTexture(TextureBindPoint::Texture2d as u32 },null) }
    }

    pub fn blend_func(&self, b1: BlendMode, b2: BlendMode) {
        unsafe {  blend_func(b1 as _,b2 as _)};
    }

    pub fn uniform_matrix_4fv(&self, location: WebGLUniformLocation, value: &[[f32; 4]; 4]) {
        unsafe {  uniform_matrix_4fv(location.reference,value[0].as_ptr())};
    }

    pub fn uniform_matrix_3fv(&self, location: WebGLUniformLocation, value: &[[f32; 3]; 3]) {
        unsafe {  uniform_matrix_3fv(location.reference,value.as_ptr() as _)};
    }

    pub fn uniform_matrix_2fv(&self, location: WebGLUniformLocation, value: &[[f32; 2]; 2]) {
        unsafe {  uniform_matrix_2fv(location.reference,value.as_ptr() as _)};
    }

    pub fn uniform_1i(&self, location: WebGLUniformLocation, value: i32) {
        //js!{ (self.as_reference()}).uniform1i(location.deref()},value}) }
    }

    pub fn uniform_1f(&self, location: WebGLUniformLocation, value: f32) {
        //js!{ (self.as_reference()}).uniform1f(location.deref()},value}) }
    }

    pub fn uniform_4fv(&self, location: WebGLUniformLocation, value: (f32, f32, f32, f32)) {
        js!{ (self.as_reference()}).uniform4f(@{location.deref()},&[value.0,value.1,value.2,value.3].as_ptr() as _) }
    }

    pub fn uniform_4f(&self, location: WebGLUniformLocation, value: (f32, f32, f32, f32)) {
        js!{ (self.as_reference()}).uniform4f(@{location.deref()},@{value.0},@{value.1},@{value.2},@{value.3}) }
    }

    pub fn create_vertex_array(&self) -> WebGLVertexArray {
        self.log("create_vertex_array");
        //let val = js! { return (&self.as_reference()}).createVertexArray() };
        unsafe { WebGLVertexArray(0) }
    }

    pub fn bind_vertex_array(&self, vao: WebGLVertexArray) {
        self.log("bind_vertex_array");
        //js! { (&self.as_reference()}).bindVertexArray(vao.deref()}) }
    }

    pub fn unbind_vertex_array(&self) {
        self.log("unbind_vertex_array");
        //js! { (&self.as_reference()}).bindVertexArray(0) }
    }

    pub fn get_program_parameter(&self, program: &WebGLProgram, pname: ShaderParameter) -> i32 {
         unsafe { get_program_parameter(program.0,pname as _) }
    }

    pub fn get_active_uniform(&self, program: &WebGLProgram, location: u32) -> WebGLActiveInfo {
        /*
        let res =
            js! { return self.as_reference()}.getActiveUniform(program.deref()},location}) };
        let name = js! { return &res}.name };
        let size = js!{ return &res}.size };
        let kind = js!{ return &res}.type };
        let k: u32 = kind.try_into().unwrap();
        use std::mem;
        */
        WebGLActiveInfo::new(
            "",
            16,
            UniformType::FloatMat4,
            0,
        )
    }

    pub fn get_active_attrib(&self, program: &WebGLProgram, location: u32) -> WebGLActiveInfo {

        WebGLActiveInfo::new(
            "",
            0,
            UniformType::FloatMat4,
            0,
        )
    }

    pub fn tex_parameteri(&self, pname: TextureParameter, param: i32) {
         unsafe { tex_parameteri(TextureBindPoint::Texture2d as _,pname as _,param as _) };
    }

    pub fn tex_parameterfv(&self, pname: TextureParameter, param: f32) {
         unsafe { tex_parameterf(TextureBindPoint::Texture2d as _,pname as _,param as _)};
    }
}

extern {

    pub fn get_context(ptr:*const u8,len:u32) -> u32;
    pub fn create_buffer() -> u32;
    pub fn create_shader(kind:u32) -> u32;
    pub fn use_program(program_ref:u32);
    pub fn get_program_parameter(program_ref:u32,pname:u32) -> i32;

  //  #[link_name = "uniform_matrix_4fv"]

    pub fn create_program() -> u32;
    
    #[link_name = "attach_shader"]
    pub fn attach_shader(program_ref:u32,shader_ref:u32);

    pub fn compile_shader(shader_ref:u32);

    pub fn bind_buffer(target:u32,buffer_ref:u32);

    pub fn buffer_data(target:u32,buffer_ptr:*const u8,buffer_len:u32,usage:u32);

    pub fn create_texture() -> u32;

    pub fn bind_texture(target:u32,texture_ref:u32);

    pub fn tex_image2d(
        target:u32,level:u32,internalformat:u32,
        width:u32,height:u32,border:u32,
        format:u32,kind:u32,buffer_ptr:*const u8,buffer_len:u32
    );

    pub fn tex_sub_image2d(
        target:u32,level:u32,x:u32,y:u32,
        width:u32,height:u32,format:u32,
        kind:u32,buffer_ptr:*const u8,buffer_len:u32
    );

    pub fn compressed_tex_image2d(
        target:u32,level:u32,internalformat:u32,
        width:u32,height:u32,border:u32,
        buffer_ptr:*const u8,buffer_len:u32
    );

    pub fn compressed_tex_sub_image2d(
        target:u32,level:u32,x:u32,y:u32,
        width:u32,height:u32,border:u32,
        buffer_ptr:*const u8,buffer_len:u32
    );
    
    pub fn get_uniform_location(program_ref: u32, buffer_ptr: *const u8, buffer_length: u32) -> i32;

    pub fn uniform_matrix_4fv(locationRef: u32, buffer_ptr: *const f32);

    pub fn uniform_matrix_3fv (locationRef: u32, buffer_ptr: *const u8);

    pub fn uniform_matrix_2fv(locationRef: u32, buffer_ptr: *const u8);


    pub fn shader_source(shader_ref:u32,ptr:*const u8,len:u32);

    pub fn link_program(program_ref:u32);


    pub fn vertex_attrib_pointer(location:u32,size:u32,kind:u32,normalized:bool,stride:u32,offset:u32);


    pub fn tex_parameteri(target:u32, p:u32, param: i32);

    pub fn tex_parameterf(target:u32, p:u32, param: f32);


    pub fn enable_vertex_attrib_array(location:u32);
    pub fn get_attrib_location(program_ref:u32,ptr:*const u8,len:u32) -> i32;

    pub fn blend_func(p1:u32,p2:u32);

    pub fn pixel_storei(storage:u32,value:u32);

    pub fn clear_color(r:f32,g:f32,b:f32,a:f32);
    pub fn enable(flag:u32);
    pub fn draw_elements(mode:u32,count:u32,kind:u32,offset:u32);
    pub fn draw_arrays(mode:u32,p:u32,count:u32);
    
    pub fn viewport(x:i32,y:i32,width:u32,height:u32);

    pub fn clear(bit:u32);
}