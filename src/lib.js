var ctx;
var ctxs = [];
var programs = [];
var shaders = [];
var buffers = [];
var textures = [];
var uniforms = [];
var HEAP8 = null;
var HEAP16 = null;
var HEAP32 = null;
var HEAPU8 = null;
var HEAPU16 = null;
var HEAPU32 = null;
var HEAPF32 = null;
var HEAPF64 = null;
// uniforms
function uintToString(uintArray) {
    var encodedString = String.fromCharCode.apply(null, uintArray), decodedString = decodeURIComponent(escape(encodedString));
    return decodedString;
}
var WebGL = /** @class */ (function () {
    function WebGL() {
    }
    WebGL.prototype.get_context = function (ptr, len) {
        console.log(str(ptr, len));
        ctx = document.querySelector('canvas').getContext("webgl");
        ctxs.push(ctx);
        var ext = ctx.getExtension("WEBGL_compressed_texture_s3tc") ||
            ctx.getExtension("MOZ_WEBGL_compressed_texture_s3tc") ||
            ctx.getExtension("WEBKIT_WEBGL_compressed_texture_s3tc");
        return ctxs.length - 1;
    };
    WebGL.prototype.create_buffer = function () {
        buffers.push(ctx.createBuffer());
        return buffers.length - 1;
    };
    WebGL.prototype.draw_elements = function (mode, count, type, offset) {
        ctx.drawElements(ctx.TRIANGLES, count, type, offset);
    };
    WebGL.prototype.draw_arrays = function (mode, first, count) {
        ctx.drawArrays(mode, first, count);
    };
    WebGL.prototype.clear = function (mask) {
        ctx.clear(mask);
    };
    WebGL.prototype.clear_color = function (r, g, b, a) {
        ctx.clearColor(r, g, b, a);
    };
    WebGL.prototype.enable = function (cap) {
        ctx.enable(cap);
    };
    WebGL.prototype.create_program = function () {
        programs.push(ctx.createProgram());
        return programs.length - 1;
    };
    WebGL.prototype.attach_shader = function (programRef, shaderRef) {
        ctx.attachShader(programs[programRef], shaders[shaderRef]);
    };
    WebGL.prototype.compile_shader = function (shaderRef) {
        ctx.compileShader(shaders[shaderRef]);
    };
    WebGL.prototype.bind_buffer = function (target, bufferRef) {
        ctx.bindBuffer(target, buffers[bufferRef]);
    };
    WebGL.prototype.buffer_data = function (target, buffer_ptr, length, usage) {
        console.log(buffer_ptr, length);
        var buffer = HEAPU8.subarray(buffer_ptr, buffer_ptr + length);
        console.log(buffer);
        ctx.bufferData(target, buffer, usage);
    };
    /// textures
    WebGL.prototype.create_texture = function () {
        textures.push(ctx.createTexture());
        return textures.length - 1;
    };
    WebGL.prototype.bind_texture = function (target, textureRef) {
        ctx.bindTexture(target, textures[textureRef]);
    };
    WebGL.prototype.tex_image2d = function (target, level, internalformat, width, height, border, format, type, buffer_ptr, buffer_length) {
        var buffer = HEAPU8.subarray(buffer_ptr, buffer_ptr + buffer_length);
        ctx.texImage2D(target, level, internalformat, width, height, border, format, type, buffer);
    };
    WebGL.prototype.tex_sub_image2d = function (target, level, x, y, width, height, format, type, buffer_ptr, buffer_length) {
        var buffer = HEAPU8.subarray(buffer_ptr, buffer_ptr + buffer_length);
        ctx.texSubImage2D(target, level, x, y, width, height, format, type, buffer);
    };
    WebGL.prototype.compressed_tex_image2d = function (target, level, internalformat, width, height, border, buffer_ptr, length) {
        var buffer = HEAPU8.subarray(buffer_ptr, buffer_ptr + length);
        ctx.compressedTexImage2D(target, level, internalformat, width, height, border, buffer);
    };
    WebGL.prototype.compressed_tex_sub_image2d = function (target, level, x, y, width, height, border, buffer_ptr, length) {
        var buffer = HEAPU8.subarray(buffer_ptr, buffer_ptr + length);
        ctx.compressedTexSubImage2D(target, level, x, y, width, height, border, buffer);
    };
    WebGL.prototype.get_uniform_location = function (program_ref, buffer_ptr, buffer_length) {
        var location = ctx.getUniformLocation(programs[program_ref], str(buffer_ptr, buffer_length));
        uniforms.push(location);
        return uniforms.length - 1;
    };
    WebGL.prototype.uniform_matrix_4fv = function (locationRef, buffer_ptr) {
        var buffer = HEAPF32.subarray(buffer_ptr, buffer_ptr + 16);
        ctx.uniformMatrix4fv(uniforms[locationRef], false, buffer);
    };
    WebGL.prototype.uniform_matrix_3fv = function (locationRef, buffer_ptr) {
        var buffer = HEAPF32.subarray(buffer_ptr, buffer_ptr + 9);
        ctx.uniformMatrix4fv(uniforms[locationRef], false, buffer);
    };
    WebGL.prototype.uniform_matrix_2fv = function (locationRef, buffer_ptr) {
        var buffer = HEAPF32.subarray(buffer_ptr, buffer_ptr + 4);
        ctx.uniformMatrix4fv(uniforms[locationRef], false, buffer);
    };
    // shaders
    WebGL.prototype.create_shader = function (kind) {
        shaders.push(ctx.createShader(kind));
        return shaders.length - 1;
    };
    WebGL.prototype.use_program = function (ref) {
        ctx.useProgram(programs[ref]);
    };
    WebGL.prototype.link_program = function (ref) {
        ctx.linkProgram(programs[ref]);
    };
    WebGL.prototype.enable_vertex_attrib_array = function (index) {
        ctx.enableVertexAttribArray(index);
    };
    WebGL.prototype.get_attrib_location = function (ref, ptr, len) {
        return ctx.getAttribLocation(programs[ref], str(ptr, len));
    };
    WebGL.prototype.shader_source = function (ref, ptr, len) {
        ctx.shaderSource(shaders[ref], str(ptr, len));
    };
    WebGL.prototype.vertex_attrib_pointer = function (index, size, kind, normalized, stride, offset) {
        ctx.vertexAttribPointer(index, size, kind, normalized, stride, offset);
    };
    WebGL.prototype.viewport = function (x, y, width, height) {
        ctx.viewport(x, y, width, height);
    };
    WebGL.prototype.tex_parameteri = function (target, pname, param) {
        ctx.texParameteri(target, pname, param);
    };
    WebGL.prototype.tex_parameterf = function (target, pname, param) {
        ctx.texParameterf(target, pname, param);
    };
    WebGL.prototype.blend_func = function (p1, p2) {
        ctx.blendFunc(p1, p2);
    };
    WebGL.prototype.get_program_parameter = function (ref, pname) {
        ctx.getProgramParameter(programs[ref], pname);
    };
    WebGL.prototype.pixel_storei = function (pname, param) {
        ctx.pixelStorei(pname, param);
    };
    return WebGL;
}());
function str(ptr, len) {
    var buffer = HEAPU8.subarray(ptr, ptr + len);
    var res = uintToString(buffer);
    console.log(res);
    return res;
}
