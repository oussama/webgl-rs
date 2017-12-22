#[derive(Debug, Clone, Copy)]
pub enum ShaderKind {
    Vertex = 35633,
    Fragment = 35632,
}

#[derive(Debug, Clone, Copy)]
pub enum BufferKind {
    Array = 0x8892,
    ElementArray = 0x8893,
}

#[derive(Debug, Clone, Copy)]
pub enum DrawMode {
    Static = 0x88E4,
    Dynamic = 0x88E8,
    Stream = 0x88E0,
}

pub enum DataType {
    I8 = 0x1400,
    U8 = 0x1401,
    I16 = 0x1402,
    U16 = 0x1403,
    I32 = 0x1404,
    U32 = 0x1405,
    Float = 0x1406,
}


pub enum Flag {
    /// Passed to enable/disable to turn on/off blending. Can also be used with getParameter to find the current blending method.
    Blend = 0x0BE2,
    /// Passed to enable/disable to turn on/off the depth test. Can also be used with getParameter to query the depth test.
    DepthTest = 0x0B71,

}

pub enum BufferBit {
    /// Passed to clear to clear the current depth buffer.
    Depth = 0x00000100,
    /// Passed to clear to clear the current stencil buffer.
    Stencil = 0x00000400,
    /// Passed to clear to clear the current color buffer.
    Color = 0x00004000
}

pub enum Primitives {
    /// Passed to drawElements or drawArrays to draw triangles. Each set of three vertices creates a separate triangle.
    Triangles = 0x0004,
}