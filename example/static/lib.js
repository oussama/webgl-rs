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
        ctx = document.querySelector('#' + uintToString(HEAPU8.subarray(ptr, ptr + len))).getContext("webgl");
        ctxs.push(ctx);
        return ctxs.length - 1;
    };
    WebGL.prototype.draw_elements = function (mode, count, type, offset) {
        ctx.drawElements(mode, count, type, offset);
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
        var buffer = HEAPU32.subarray(buffer_ptr, buffer_ptr + length);
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
        var buffer = HEAPF32.subarray(buffer_ptr, buffer_ptr + buffer_length);
        ctx.texImage2D(target, level, internalformat, width, height, border, format, type, buffer);
    };
    WebGL.prototype.tex_sub_image2d = function (target, level, x, y, width, height, format, type, buffer_ptr, buffer_length) {
        var buffer = HEAPF32.subarray(buffer_ptr, buffer_ptr + buffer_length);
        ctx.texSubImage2D(target, level, x, y, width, height, format, type, buffer);
    };
    WebGL.prototype.compressed_tex_image2d = function (target, level, internalformat, width, height, border, buffer_ptr, length) {
        var buffer = HEAPF32.subarray(buffer_ptr, buffer_ptr + length);
        ctx.compressedTexImage2D(target, level, internalformat, width, height, border, buffer);
    };
    WebGL.prototype.compressed_tex_sub_image2d = function (target, level, x, y, width, height, border, buffer_ptr, length) {
        var buffer = HEAPF32.subarray(buffer_ptr, buffer_ptr + length);
        ctx.compressedTexSubImage2D(target, level, x, y, width, height, border, buffer);
    };
    WebGL.prototype.get_uniform_location = function (program_ref, buffer_ptr, buffer_length) {
        var buffer = HEAPU8.subarray(buffer_ptr, buffer_ptr + (buffer_length * 4));
        var location = ctx.getUniformLocation(programs[program_ref], uintToString(buffer));
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
    return WebGL;
}());
