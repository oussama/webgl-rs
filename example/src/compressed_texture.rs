use application::*;

use utils::*;
use webgl::*;
use dds::*;

pub fn main() {
    let size = (800, 600);
    let config = AppConfig::new("Test", size);
    let mut app = App::new(config);

    let vertices: Vec<f32> = vec![
        -0.5, 0.5, 0.0, 0.0, 0.0, 0.5, 0.5, 0.0, 1.0, 0.0, 0.5, -0.5, 0.0, 1.0, 1.0, -0.5, -0.5,
        0.0, 0.0, 1.0,
    ];
    let indices: Vec<u16> = vec![0, 1, 2, 0, 2, 3];
    let count = indices.len();

    #[cfg(not(target_arch = "wasm32"))]
    WebGL2RenderingContext::load_with(|name| app.get_proc_address(name));

    let gl = WebGL2RenderingContext::new("canvas");

    // Create an empty buffer object to store vertex buffer
    let vertex_buffer = gl.create_buffer();

    // Bind appropriate array buffer to it
    gl.bind_buffer(BufferKind::Array, &vertex_buffer);

    // Pass the vertex data to the buffer
    gl.buffer_data(BufferKind::Array, &vertices.into_bytes(), DrawMode::Static);

    // Unbind the buffer
    gl.unbind_buffer(BufferKind::Array);

    // Create an empty buffer object to store Index buffer
    let index_buffer = gl.create_buffer();

    // Bind appropriate array buffer to it
    gl.bind_buffer(BufferKind::ElementArray, &index_buffer);

    // Pass the vertex data to the buffer
    gl.buffer_data(
        BufferKind::ElementArray,
        &indices.into_bytes(),
        DrawMode::Static,
    );

    // Unbind the buffer
    gl.unbind_buffer(BufferKind::ElementArray);

    /*================ Shaders ====================*/

    // Vertex shader source code
    let vert_code = "attribute vec2 a_pos;
    attribute vec2 a_uv;
    varying vec2 uv;
    void main()
    {
        gl_Position = vec4(a_pos, 0.0, 1.0);
        uv = a_uv;
    }";

    // Create a vertex shader object
    let vert_shader = gl.create_shader(ShaderKind::Vertex);

    // Attach vertex shader source code
    gl.shader_source(&vert_shader, vert_code);

    // Compile the vertex shader
    gl.compile_shader(&vert_shader);

    //fragment shader source code
    let frag_code = "precision mediump float;
    uniform sampler2D tex;
    varying vec2 uv;
    void main(){
        gl_FragColor =  texture2D(tex,uv);
    }";

    // Create fragment shader object
    let frag_shader = gl.create_shader(ShaderKind::Fragment);

    // Attach fragment shader source code
    gl.shader_source(&frag_shader, frag_code);

    // Compile the fragmentt shader
    gl.compile_shader(&frag_shader);

    // Create a shader program object to store
    // the combined shader program
    let shader_program = gl.create_program();

    // Attach a vertex shader
    gl.attach_shader(&shader_program, &vert_shader);

    // Attach a fragment shader
    gl.attach_shader(&shader_program, &frag_shader);

    // Link both the programs
    gl.link_program(&shader_program);

    // Use the combined shader program object
    gl.use_program(&shader_program);

    /*======= Associating shaders to buffer objects =======*/

    // Bind vertex buffer object
    gl.bind_buffer(BufferKind::Array, &vertex_buffer);

    // Bind index buffer object
    gl.bind_buffer(BufferKind::ElementArray, &index_buffer);

    // Get the attribute location
    let coord = gl.get_attrib_location(&shader_program, "a_pos".into())
        .expect("Attribute not found");
    // Point an attribute to the currently bound VBO
    gl.vertex_attrib_pointer(coord, AttributeSize::Three, DataType::Float, false, 20, 0);
    // Enable the attribute
    gl.enable_vertex_attrib_array(coord);

    // Get the attribute location
    let uv = gl.get_attrib_location(&shader_program, "a_uv".into())
        .expect("Attribute not found");
    // Point an attribute to the currently bound VBO
    gl.vertex_attrib_pointer(uv, AttributeSize::Two, DataType::Float, false, 20, 12);
    // Enable the attribute
    gl.enable_vertex_attrib_array(uv);

    // Load texture
    let tex_bytes = include_bytes!("../assets/box.dds");
    let (width, height, tex_data, tex_compression, pixel_format) = parse(tex_bytes);

    println!("data size {}", tex_data.len());

    let texture = gl.create_texture();
    gl.bind_texture(&texture);
    //gl.log(tex_data.len().to_string());
    gl.compressed_tex_image2d(
        TextureBindPoint::Texture2d,
        0,
        tex_compression,
        width,
        height,
        &tex_data,
    );
    gl.tex_parameteri(
        TextureParameter::TextureMagFilter,
        TextureMagFilter::Linear as _,
    );
    gl.tex_parameteri(
        TextureParameter::TextureMinFilter,
        TextureMinFilter::Linear as _,
    );

    /*=========Drawing the triangle===========*/

    // Clear the canvas
    gl.clear_color(0.5, 0.5, 0.5, 0.9);

    // Enable the depth test
    gl.enable(Flag::DepthTest);

    // Clear the color buffer bit
    gl.clear(BufferBit::Color);
    gl.clear(BufferBit::Depth);

    // Set the view port
    gl.viewport(0, 0, size.0, size.1);

    app.run(move |_t:&mut App| {
        gl.clear(BufferBit::Color);
        gl.clear(BufferBit::Depth);
        gl.clear_color(1.0, 1.0, 1.0, 1.0);
        gl.draw_elements(Primitives::Triangles, count, DataType::U16, 0);
    });
}
