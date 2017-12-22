# webgl-rs
webgl for wasm32-unknown-unknown using stdweb

## Example:

To run the example in "example" folder, run:
```
cd ./example
cargo web start --target-webasm
````

Example from https://www.tutorialspoint.com/webgl/webgl_drawing_a_triangle.htm in Rust

![alt text](https://raw.githubusercontent.com/oussama/webgl-rs/master/example/triangle.PNG)

```rust
let res = stdweb::initialize();
let canvas = document().create_element("canvas");
document()
    .query_selector("body")
    .unwrap()
    .append_child(&canvas);
let vertices: Vec<f32> = vec![-0.5, 0.5, 0.0, -0.5, -0.5, 0.0, 0.5, -0.5, 0.0];
let indices: Vec<u16> = vec![0, 1, 2];
let count = indices.len();
let gl = WebGL2RenderingContext::new(&canvas);
// Create an empty buffer object to store vertex buffer
let vertex_buffer = gl.create_buffer();
// Bind appropriate array buffer to it
gl.bind_buffer(BufferKind::Array, &vertex_buffer);
// Pass the vertex data to the buffer
gl.buffer_data(BufferKind::Array, &vertices.into_bytes(), DrawMode::Static);
// Unbind the buffer
gl.unbind_buffer(BufferKind::Array);
// Create an empty buffer object to store Index buffer
let Index_Buffer = gl.create_buffer();
// Bind appropriate array buffer to it
gl.bind_buffer(BufferKind::ElementArray, &Index_Buffer);
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
let vert_code = "attribute vec3 coordinates;
void main(void) {
    gl_Position = vec4(coordinates, 1.0);
}";
// Create a vertex shader object
let vertShader = gl.create_shader(ShaderKind::Vertex);
// Attach vertex shader source code
gl.shader_source(&vertShader, vert_code);
// Compile the vertex shader
gl.compile_shader(&vertShader);
//fragment shader source code
let frag_code = "void main(void) {
    gl_FragColor = vec4(1, 0.5, 0.0, 1);
}";
// Create fragment shader object
let fragShader = gl.create_shader(ShaderKind::Fragment);
// Attach fragment shader source code
gl.shader_source(&fragShader, frag_code);
// Compile the fragmentt shader
gl.compile_shader(&fragShader);
// Create a shader program object to store
// the combined shader program
let shaderProgram = gl.create_program();
// Attach a vertex shader
gl.attach_shader(&shaderProgram, &vertShader);
// Attach a fragment shader
gl.attach_shader(&shaderProgram, &fragShader);
// Link both the programs
gl.link_program(&shaderProgram);
// Use the combined shader program object
gl.use_program(&shaderProgram);
/*======= Associating shaders to buffer objects =======*/
// Bind vertex buffer object
gl.bind_buffer(BufferKind::Array, &vertex_buffer);
// Bind index buffer object
gl.bind_buffer(BufferKind::ElementArray, &Index_Buffer);
// Get the attribute location
let coord = gl.get_attrib_location(&shaderProgram, "coordinates".into())
    .unwrap();
// Point an attribute to the currently bound VBO
gl.vertex_attrib_pointer(coord, 3, DataType::Float, false, 0, 0);
// Enable the attribute
gl.enable_vertex_attrib_array(coord);
/*=========Drawing the triangle===========*/
// Clear the canvas
gl.clear_color(0.5, 0.5, 0.5, 0.9);
// Enable the depth test
gl.enable(Flag::DepthTest);
// Clear the color buffer bit
gl.clear(BufferBit::Color);
// Set the view port
gl.viewport(0.0, 0.0, 300.0, 150.0);
// Draw the triangle
gl.draw_elements(Primitives::Triangles, count, DataType::U16, 0);
stdweb::event_loop();
```