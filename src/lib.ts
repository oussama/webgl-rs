

let ctx: WebGLRenderingContext;
let ctxs: WebGLRenderingContext[] = [];
let programs: WebGLProgram[] = [];
let shaders: WebGLShader[] = [];
let buffers: WebGLBuffer[] = [];
let textures: WebGLTexture[] = [];
let uniforms: WebGLUniformLocation[] = [];

let HEAP8 = null;
let HEAP16 = null;
let HEAP32 = null;
let HEAPU8 = null;
let HEAPU16 = null;
let HEAPU32: Uint32Array = null;
let HEAPF32: Float32Array = null;
let HEAPF64 = null;




declare function escape(str);

// uniforms
function uintToString(uintArray) {
    var encodedString = String.fromCharCode.apply(null, uintArray),
        decodedString = decodeURIComponent(escape(encodedString));
    return decodedString;
}

class WebGL {


    get_context(ptr: number, len: number) {
        console.log(str(ptr, len));
        ctx = (<any>document.querySelector('canvas')).getContext("webgl");
        ctxs.push(ctx);
        let ext = ctx.getExtension("WEBGL_compressed_texture_s3tc") ||
            ctx.getExtension("MOZ_WEBGL_compressed_texture_s3tc") ||
            ctx.getExtension("WEBKIT_WEBGL_compressed_texture_s3tc")
        return ctxs.length - 1;
    }

    create_buffer() {
        buffers.push(ctx.createBuffer());
        return buffers.length - 1;
    }

    draw_elements(mode: number, count: number, type: number, offset: number) {
        ctx.drawElements(ctx.TRIANGLES, count, type, offset);
    }

    draw_arrays(mode: number, first: number, count: number) {
        ctx.drawArrays(mode, first, count);
    }

    clear(mask: number) {
        ctx.clear(mask);
    }

    clear_color(r, g, b, a) {
        ctx.clearColor(r, g, b, a);
    }

    enable(cap: number) {
        ctx.enable(cap);
    }

    create_program() {
        programs.push(ctx.createProgram());
        return programs.length - 1;
    }

    attach_shader(programRef: number, shaderRef: number) {
        ctx.attachShader(programs[programRef], shaders[shaderRef]);
    }

    compile_shader(shaderRef: number) {
        ctx.compileShader(shaders[shaderRef]);
    }

    bind_buffer(target: number, bufferRef: number) {
        ctx.bindBuffer(target, buffers[bufferRef]);
    }

    buffer_data(target: number, buffer_ptr: number, length: number, usage: number) {
        console.log(buffer_ptr, length);
        let buffer = HEAPU8.subarray(buffer_ptr, buffer_ptr + length);
        console.log(buffer);
        ctx.bufferData(target, buffer, usage);
    }



    /// textures

    create_texture() {
        textures.push(ctx.createTexture());
        return textures.length - 1;
    }

    bind_texture(target: number, textureRef: number) {
        ctx.bindTexture(target, textures[textureRef]);
    }

    tex_image2d(
        target: number, level: number, internalformat: number,
        width: number, height: number, border: number,
        format: number, type: number, buffer_ptr, buffer_length
    ) {
        let buffer = HEAPU8.subarray(buffer_ptr, buffer_ptr + buffer_length);
        ctx.texImage2D(target, level, internalformat, width, height, border, format, type, buffer);
    }

    tex_sub_image2d(
        target: number, level: number, x: number, y: number,
        width: number, height: number, format: number,
        type: number, buffer_ptr, buffer_length
    ) {
        let buffer = HEAPU8.subarray(buffer_ptr, buffer_ptr + buffer_length);
        ctx.texSubImage2D(target, level, x, y, width, height, format, type, buffer);
    }

    compressed_tex_image2d(target: number, level: number, internalformat: number,
        width: number, height: number, border: number,
        buffer_ptr: number, length: number) {
        let buffer = HEAPU8.subarray(buffer_ptr, buffer_ptr + length);
        ctx.compressedTexImage2D(target, level, internalformat, width, height, border, buffer);
    }

    compressed_tex_sub_image2d(target: number, level: number, x: number, y: number,
        width: number, height: number, border: number,
        buffer_ptr: number, length: number) {
        let buffer = HEAPU8.subarray(buffer_ptr, buffer_ptr + length);
        ctx.compressedTexSubImage2D(target, level, x, y, width, height, border, buffer);
    }

    get_uniform_location(program_ref: number, buffer_ptr: number, buffer_length: number) {
        let location = ctx.getUniformLocation(programs[program_ref], str(buffer_ptr, buffer_length));
        uniforms.push(location);
        return uniforms.length - 1;
    }

    uniform_matrix_4fv(locationRef: number, buffer_ptr: number) {
        let buffer = HEAPF32.subarray(buffer_ptr, buffer_ptr + 16);
        ctx.uniformMatrix4fv(uniforms[locationRef], false, buffer);
    }

    uniform_matrix_3fv(locationRef: number, buffer_ptr: number) {
        let buffer = HEAPF32.subarray(buffer_ptr, buffer_ptr + 9);
        ctx.uniformMatrix4fv(uniforms[locationRef], false, buffer);
    }

    uniform_matrix_2fv(locationRef: number, buffer_ptr: number) {
        let buffer = HEAPF32.subarray(buffer_ptr, buffer_ptr + 4);
        ctx.uniformMatrix4fv(uniforms[locationRef], false, buffer);
    }


    // shaders

    create_shader(kind: number) {
        shaders.push(ctx.createShader(kind));
        return shaders.length - 1;
    }

    use_program(ref) {
        ctx.useProgram(programs[ref]);
    }

    link_program(ref) {
        ctx.linkProgram(programs[ref]);
    }

    enable_vertex_attrib_array(index) {
        ctx.enableVertexAttribArray(index);
    }

    get_attrib_location(ref: number, ptr: number, len: number) {
        return ctx.getAttribLocation(programs[ref], str(ptr, len));
    }

    shader_source(ref, ptr, len) {
        ctx.shaderSource(shaders[ref], str(ptr, len));
    }

    vertex_attrib_pointer(index, size, kind, normalized, stride, offset) {
        ctx.vertexAttribPointer(index, size, kind, normalized, stride, offset)
    }

    viewport(x, y, width, height) {
        ctx.viewport(x, y, width, height);
    }

    tex_parameteri(target, pname, param) {
        ctx.texParameteri(target, pname, param);
    }

    tex_parameterf(target, pname, param) {
        ctx.texParameterf(target, pname, param);
    }

    blend_func(p1,p2){
        ctx.blendFunc(p1,p2);
    }

    get_program_parameter(ref,pname){
        ctx.getProgramParameter(programs[ref],pname);
    }

    pixel_storei(pname,param){
        ctx.pixelStorei(pname,param);
    }

}


function str(ptr, len) {
    let buffer = HEAPU8.subarray(ptr, ptr + len);
    let res = uintToString(buffer);
    console.log(res);
    return res;
}