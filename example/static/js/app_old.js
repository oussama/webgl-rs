"use strict";

if( typeof Rust === 'undefined' ) {
    var Rust = {};
}

(function( root, factory ) {
    if( typeof define === "function" && define.amd ) {
        define( [], factory );
    } else if( typeof module === "object" && module.exports ) {
        module.exports = factory();
    } else {
        factory();
    }
}( this, function() {
    const Module = {};
    let HEAP8 = null;
    let HEAP16 = null;
    let HEAP32 = null;
    let HEAPU8 = null;
    let HEAPU16 = null;
    let HEAPU32 = null;
    let HEAPF32 = null;
    let HEAPF64 = null;

    Object.defineProperty( Module, 'nodejs', { value: (typeof window === 'undefined') } );
    Object.defineProperty( Module, 'exports', { value: {} } );

    const __imports = {
        env: {
            "__extjs_d8a439451216bbc6cd9f3012f189d2ad6a2e9459": function($0) {
                Module.STDWEB.decrement_refcount( $0 );
            },
            "__extjs_5f316a066fa2631a8bedd93bfe7ed9f23e41ddf1": function($0, $1, $2) {
                var canvas = document.createElement('canvas');
                canvas.width = 800;
                canvas.height = 600;
                document.querySelector('body').appendChild(canvas);
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);Module.STDWEB.from_js($0, (function(){return document.querySelector (($1)). getContext (($2));})());
            },
            "__extjs_de942ef9ccd064c41dc92d5b5bf83c61aeb00278": function($0) {
                Module.STDWEB.increment_refcount( $0 );
            },
            "__extjs_10d8c7414139488227fda64d5a39c9e9228f1cd6": function($0, $1, $2, $3, $4) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);$3 = Module.STDWEB.to_js($3);$4 = Module.STDWEB.to_js($4);Module.STDWEB.from_js($0, (function(){(($1)). bufferData (($2), ($3), ($4))})());
            },
            "__extjs_c415fcb479e5b01e004e467d1ff7f9e904219b84": function($0, $1) {
                $1 = Module.STDWEB.to_js($1);Module.STDWEB.from_js($0, (function(){return (($1)). createBuffer ();})());
            },
            "__extjs_614569be5d0636763db3c841d395263611f9107d": function($0, $1, $2, $3) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);$3 = Module.STDWEB.to_js($3);Module.STDWEB.from_js($0, (function(){(($1)). bindBuffer (($2), ($3))})());
            },
            "__extjs_e2e2655921ae9317bc7fde1db50253430f147c24": function($0, $1, $2) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);Module.STDWEB.from_js($0, (function(){(($1)). bindBuffer (($2), null)})());
            },
            "__extjs_60fd85d3a624582c5d2cb5a3f7f51dcca4124f38": function($0, $1, $2) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);Module.STDWEB.from_js($0, (function(){return (($1)). createShader (($2));})());
            },
            "__extjs_35525ea53667961f229695aee8d1c858da2dc0a6": function($0, $1, $2, $3) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);$3 = Module.STDWEB.to_js($3);Module.STDWEB.from_js($0, (function(){(($1)). shaderSource (($2), ($3))})());
            },
            "__extjs_1172c06cb9c5322a5bef26b04e49b10a1dc303b0": function($0, $1, $2) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);Module.STDWEB.from_js($0, (function(){(($1)). compileShader (($2))})());
            },
            "__extjs_b66783f408a64c7cbac56d61bba878a161bf3a4b": function($0, $1, $2, $3, $4) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);$3 = Module.STDWEB.to_js($3);$4 = Module.STDWEB.to_js($4);Module.STDWEB.from_js($0, (function(){var compiled = (($1)). getShaderParameter (($2), 0x8B81); console.log ("Shader compiled successfully:" , compiled); var compilationLog = (($3)). getShaderInfoLog (($4)); console.log ("Shader compiler log:" , compilationLog);})());
            },
            "__extjs_1e39bcc9d820ed581d80c2c95251f31f9ceaae0c": function($0, $1) {
                $1 = Module.STDWEB.to_js($1);Module.STDWEB.from_js($0, (function(){return (($1)). createProgram ();})());
            },
            "__extjs_815ba7bddf63d413c8bf1733f8ea15065a87f4b8": function($0, $1, $2) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);Module.STDWEB.from_js($0, (function(){(($1)). linkProgram (($2))})());
            },
            "__extjs_1e142aecc43da9fda56780628d7a853ee29d6852": function($0, $1, $2) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);Module.STDWEB.from_js($0, (function(){(($1)). useProgram (($2))})());
            },
            "__extjs_54fa3a1818b20a08032c5525f0352044738befeb": function($0, $1, $2, $3) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);$3 = Module.STDWEB.to_js($3);Module.STDWEB.from_js($0, (function(){(($1)). attachShader (($2), ($3))})());
            },
            "__extjs_bc4e27f881617e7ceae57e9b16dadaf694b28a31": function($0, $1, $2, $3) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);$3 = Module.STDWEB.to_js($3);Module.STDWEB.from_js($0, (function(){return (($1)). getAttribLocation (($2), ($3))})());
            },
            "__extjs_a2abe0205ec62cdc2cff60556472727bcf15d5f5": function($0, $1, $2, $3, $4, $5, $6, $7) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);$3 = Module.STDWEB.to_js($3);$4 = Module.STDWEB.to_js($4);$5 = Module.STDWEB.to_js($5);$6 = Module.STDWEB.to_js($6);$7 = Module.STDWEB.to_js($7);Module.STDWEB.from_js($0, (function(){(($1)). vertexAttribPointer (($2), ($3), ($4), ($5), ($6), ($7));})());
            },
            "__extjs_c01d757436da4763ef550ad7de7a438f660c87d8": function($0, $1, $2) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);Module.STDWEB.from_js($0, (function(){(($1)). enableVertexAttribArray (($2))})());
            },
            "__extjs_6c8094463474b1358c9c7b61d8897f7a78245f03": function($0, $1, $2, $3, $4, $5) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);$3 = Module.STDWEB.to_js($3);$4 = Module.STDWEB.to_js($4);$5 = Module.STDWEB.to_js($5);Module.STDWEB.from_js($0, (function(){(($1)). clearColor (($2), ($3), ($4), ($5))})());
            },
            "__extjs_a5a4b88af7201b74e7cdf515b0aa0a2b0ef479a1": function($0, $1, $2) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);Module.STDWEB.from_js($0, (function(){(($1)). enable (($2))})());
            },
            "__extjs_5b53ccabc91f3ca0ae8ac275097b3f9b8427769d": function($0, $1, $2) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);Module.STDWEB.from_js($0, (function(){(($1)). clear (($2))})());
            },
            "__extjs_715b7ba399c5afdf408c3803101c11297d923db3": function($0, $1, $2, $3, $4, $5) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);$3 = Module.STDWEB.to_js($3);$4 = Module.STDWEB.to_js($4);$5 = Module.STDWEB.to_js($5);Module.STDWEB.from_js($0, (function(){(($1)). viewport (($2), ($3), ($4), ($5));})());
            },
            "__extjs_48fea0d137c4af5b6b3f3e207d926d26d4fa5091": function($0, $1, $2, $3, $4, $5) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);$3 = Module.STDWEB.to_js($3);$4 = Module.STDWEB.to_js($4);$5 = Module.STDWEB.to_js($5);Module.STDWEB.from_js($0, (function(){(($1)). drawElements (($2), ($3), ($4), ($5));})());
            },
            "__extjs_7323e99c2e027aa09197d6361875d75b4b415808": function($0, $1, $2, $3) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);$3 = Module.STDWEB.to_js($3);Module.STDWEB.from_js($0, (function(){(($1)). getExtension ("WEBGL_compressed_texture_s3tc")|| (($2)). getExtension ("MOZ_WEBGL_compressed_texture_s3tc")|| (($3)). getExtension ("WEBKIT_WEBGL_compressed_texture_s3tc")})());
            },
            "__extjs_37d1cbcbd5792b6175769c331fc0c0ba309c219c": function($0, $1, $2, $3, $4, $5, $6, $7) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);$3 = Module.STDWEB.to_js($3);$4 = Module.STDWEB.to_js($4);$5 = Module.STDWEB.to_js($5);$6 = Module.STDWEB.to_js($6);$7 = Module.STDWEB.to_js($7);Module.STDWEB.from_js($0, (function(){(($1)). compressedTexImage2D (($2), ($3), ($4), ($5), ($6), 0 , ($7));})());
            },
            "__extjs_bbdf1327659a5c7f9b36cac27442123e23e63a17": function($0, $1) {
                $1 = Module.STDWEB.to_js($1);Module.STDWEB.from_js($0, (function(){return ($1). createTexture ()})());
            },
            "__extjs_62fa8b5ff17cd42e229ec5dfa9fb7c3d745cbbff": function($0, $1, $2, $3) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);$3 = Module.STDWEB.to_js($3);Module.STDWEB.from_js($0, (function(){(($1)). bindTexture (($2), ($3))})());
            },
            "__extjs_1f514560a22dff0f0408d31599f13351c26587ad": function($0, $1, $2, $3, $4) {
                $1 = Module.STDWEB.to_js($1);$2 = Module.STDWEB.to_js($2);$3 = Module.STDWEB.to_js($3);$4 = Module.STDWEB.to_js($4);Module.STDWEB.from_js($0, (function(){return ($1). texParameteri (($2), ($3), ($4))})());
            },
            "__extjs_f0da9e3af46afb4353410c272d5cdc083a223958": function($0) {
                return (Module.STDWEB.acquire_js_reference( $0 ) instanceof Uint8Array) | 0;
            },
            "__extjs_94be576145abfa284eb52bcbf98871bcbf01d427": function($0, $1) {
                return Module.STDWEB.acquire_rust_reference( HEAPU8.slice( $0, $1 ) );
            },
            "__extjs_8adab11649c3c91785d157bc51cb5d1ce1d8d8f8": function() {
                Module.STDWEB = {};   Module.STDWEB.to_utf8 = function to_utf8( str, addr ) {     for( var i = 0; i < str.length; ++i ) {                                    var u = str.charCodeAt( i );          if( u >= 0xD800 && u <= 0xDFFF ) {             u = 0x10000 + ((u & 0x3FF) << 10) | (str.charCodeAt( ++i ) & 0x3FF);         }          if( u <= 0x7F ) {             HEAPU8[ addr++ ] = u;         } else if( u <= 0x7FF ) {             HEAPU8[ addr++ ] = 0xC0 | (u >> 6);             HEAPU8[ addr++ ] = 0x80 | (u & 63);         } else if( u <= 0xFFFF ) {             HEAPU8[ addr++ ] = 0xE0 | (u >> 12);             HEAPU8[ addr++ ] = 0x80 | ((u >> 6) & 63);             HEAPU8[ addr++ ] = 0x80 | (u & 63);         } else if( u <= 0x1FFFFF ) {             HEAPU8[ addr++ ] = 0xF0 | (u >> 18);             HEAPU8[ addr++ ] = 0x80 | ((u >> 12) & 63);             HEAPU8[ addr++ ] = 0x80 | ((u >> 6) & 63);             HEAPU8[ addr++ ] = 0x80 | (u & 63);         } else if( u <= 0x3FFFFFF ) {             HEAPU8[ addr++ ] = 0xF8 | (u >> 24);             HEAPU8[ addr++ ] = 0x80 | ((u >> 18) & 63);             HEAPU8[ addr++ ] = 0x80 | ((u >> 12) & 63);             HEAPU8[ addr++ ] = 0x80 | ((u >> 6) & 63);             HEAPU8[ addr++ ] = 0x80 | (u & 63);         } else {             HEAPU8[ addr++ ] = 0xFC | (u >> 30);             HEAPU8[ addr++ ] = 0x80 | ((u >> 24) & 63);             HEAPU8[ addr++ ] = 0x80 | ((u >> 18) & 63);             HEAPU8[ addr++ ] = 0x80 | ((u >> 12) & 63);             HEAPU8[ addr++ ] = 0x80 | ((u >> 6) & 63);             HEAPU8[ addr++ ] = 0x80 | (u & 63);         }     } };  Module.STDWEB.noop = function() {}; Module.STDWEB.to_js = function to_js( address ) {     var kind = HEAPU8[ address + 12 ];     if( kind === 0 ) {         return undefined;     } else if( kind === 1 ) {         return null;     } else if( kind === 2 ) {         return HEAP32[ address / 4 ];     } else if( kind === 3 ) {         return HEAPF64[ address / 8 ];     } else if( kind === 4 ) {         var pointer = HEAPU32[ address / 4 ];         var length = HEAPU32[ (address + 4) / 4 ];         return Module.STDWEB.to_js_string( pointer, length );     } else if( kind === 5 ) {         return false;     } else if( kind === 6 ) {         return true;     } else if( kind === 7 ) {         var pointer = HEAPU32[ address / 4 ];         var length = HEAPU32[ (address + 4) / 4 ];         var output = [];         for( var i = 0; i < length; ++i ) {             output.push( Module.STDWEB.to_js( pointer + i * 16 ) );         }         return output;     } else if( kind === 8 ) {         var value_array_pointer = HEAPU32[ address / 4 ];         var length = HEAPU32[ (address + 4) / 4 ];         var key_array_pointer = HEAPU32[ (address + 8) / 4 ];         var output = {};         for( var i = 0; i < length; ++i ) {             var key_pointer = HEAPU32[ (key_array_pointer + i * 8) / 4 ];             var key_length = HEAPU32[ (key_array_pointer + 4 + i * 8) / 4 ];             var key = Module.STDWEB.to_js_string( key_pointer, key_length );             var value = Module.STDWEB.to_js( value_array_pointer + i * 16 );             output[ key ] = value;         }         return output;     } else if( kind === 9 || kind === 11 || kind === 12 ) {         return Module.STDWEB.acquire_js_reference( HEAP32[ address / 4 ] );     } else if( kind === 10 ) {         var adapter_pointer = HEAPU32[ address / 4 ];         var pointer = HEAPU32[ (address + 4) / 4 ];         var deallocator_pointer = HEAPU32[ (address + 8) / 4 ];         var output = function() {             if( pointer === 0 ) {                 throw new ReferenceError( "Already dropped Rust function called!" );             }              var args = Module.STDWEB.alloc( 16 );             Module.STDWEB.serialize_array( args, arguments );             Module.STDWEB.dyncall( "vii", adapter_pointer, [pointer, args] );             var result = Module.STDWEB.tmp;             Module.STDWEB.tmp = null;              return result;         };          output.drop = function() {             output.drop = Module.STDWEB.noop;             var function_pointer = pointer;             pointer = 0;              Module.STDWEB.dyncall( "vi", deallocator_pointer, [function_pointer] );         };          return output;     } else if( kind === 13 ) {         var adapter_pointer = HEAPU32[ address / 4 ];         var pointer = HEAPU32[ (address + 4) / 4 ];         var deallocator_pointer = HEAPU32[ (address + 8) / 4 ];         var output = function() {             if( pointer === 0 ) {                 throw new ReferenceError( "Already called or dropped FnOnce function called!" );             }              output.drop = Module.STDWEB.noop;             var function_pointer = pointer;             pointer = 0;              var args = Module.STDWEB.alloc( 16 );             Module.STDWEB.serialize_array( args, arguments );             Module.STDWEB.dyncall( "vii", adapter_pointer, [function_pointer, args] );             var result = Module.STDWEB.tmp;             Module.STDWEB.tmp = null;              return result;         };          output.drop = function() {             output.drop = Module.STDWEB.noop;             var function_pointer = pointer;             pointer = 0;              Module.STDWEB.dyncall( "vi", deallocator_pointer, [function_pointer] );         };          return output;     } else if( kind === 14 ) {         var pointer = HEAPU32[ address / 4 ];         var length = HEAPU32[ (address + 4) / 4 ];         var array_kind = HEAPU32[ (address + 8) / 4 ];         var pointer_end = pointer + length;          switch( array_kind ) {             case 0:                 return HEAPU8.subarray( pointer, pointer_end );             case 1:                 return HEAP8.subarray( pointer, pointer_end );             case 2:                 return HEAPU16.subarray( pointer, pointer_end );             case 3:                 return HEAP16.subarray( pointer, pointer_end );             case 4:                 return HEAPU32.subarray( pointer, pointer_end );             case 5:                 return HEAP32.subarray( pointer, pointer_end );             case 6:                 return HEAPF32.subarray( pointer, pointer_end );             case 7:                 return HEAPF64.subarray( pointer, pointer_end );         }     } };  Module.STDWEB.serialize_object = function serialize_object( address, value ) {     var keys = Object.keys( value );     var length = keys.length;     var key_array_pointer = Module.STDWEB.alloc( length * 8 );     var value_array_pointer = Module.STDWEB.alloc( length * 16 );     HEAPU8[ address + 12 ] = 8;     HEAPU32[ address / 4 ] = value_array_pointer;     HEAPU32[ (address + 4) / 4 ] = length;     HEAPU32[ (address + 8) / 4 ] = key_array_pointer;     for( var i = 0; i < length; ++i ) {         var key = keys[ i ];         var key_length = Module.STDWEB.utf8_len( key );         var key_pointer = Module.STDWEB.alloc( key_length );         Module.STDWEB.to_utf8( key, key_pointer );          var key_address = key_array_pointer + i * 8;         HEAPU32[ key_address / 4 ] = key_pointer;         HEAPU32[ (key_address + 4) / 4 ] = key_length;          Module.STDWEB.from_js( value_array_pointer + i * 16, value[ key ] );     } };  Module.STDWEB.serialize_array = function serialize_array( address, value ) {     var length = value.length;     var pointer = Module.STDWEB.alloc( length * 16 );     HEAPU8[ address + 12 ] = 7;     HEAPU32[ address / 4 ] = pointer;     HEAPU32[ (address + 4) / 4 ] = length;     for( var i = 0; i < length; ++i ) {         Module.STDWEB.from_js( pointer + i * 16, value[ i ] );     } };  Module.STDWEB.from_js = function from_js( address, value ) {     var kind = Object.prototype.toString.call( value );     if( kind === "[object String]" ) {         var length = Module.STDWEB.utf8_len( value );         var pointer = 0;         if( length > 0 ) {             pointer = Module.STDWEB.alloc( length );             Module.STDWEB.to_utf8( value, pointer );         }         HEAPU8[ address + 12 ] = 4;         HEAPU32[ address / 4 ] = pointer;         HEAPU32[ (address + 4) / 4 ] = length;     } else if( kind === "[object Number]" ) {         if( value === (value|0) ) {             HEAPU8[ address + 12 ] = 2;             HEAP32[ address / 4 ] = value;         } else {             HEAPU8[ address + 12 ] = 3;             HEAPF64[ address / 8 ] = value;         }     } else if( value === null ) {         HEAPU8[ address + 12 ] = 1;     } else if( value === undefined ) {         HEAPU8[ address + 12 ] = 0;     } else if( value === false ) {         HEAPU8[ address + 12 ] = 5;     } else if( value === true ) {         HEAPU8[ address + 12 ] = 6;     } else {         var refid = Module.STDWEB.acquire_rust_reference( value );         var id = 9;         if( kind === "[object Object]" ) {             id = 11;         } else if( kind === "[object Array]" || kind === "[object Arguments]" ) {             id = 12;         }          HEAPU8[ address + 12 ] = id;         HEAP32[ address / 4 ] = refid;     } };    Module.STDWEB.to_js_string = function to_js_string( index, length ) {     index = index|0;     length = length|0;     var end = (index|0) + (length|0);     var output = "";     while( index < end ) {         var x = HEAPU8[ index++ ];         if( x < 128 ) {             output += String.fromCharCode( x );             continue;         }         var init = (x & (0x7F >> 2));         var y = 0;         if( index < end ) {             y = HEAPU8[ index++ ];         }         var ch = (init << 6) | (y & 63);         if( x >= 0xE0 ) {             var z = 0;             if( index < end ) {                 z = HEAPU8[ index++ ];             }             var y_z = ((y & 63) << 6) | (z & 63);             ch = init << 12 | y_z;             if( x >= 0xF0 ) {                 var w = 0;                 if( index < end ) {                     w = HEAPU8[ index++ ];                 }                 ch = (init & 7) << 18 | ((y_z << 6) | (w & 63));             }         }         output += String.fromCharCode( ch );         continue;     }     return output; };  var id_to_ref_map = {}; var id_to_refcount_map = {}; var ref_to_id_map = new WeakMap(); var ref_to_id_symbol_map = {}; var last_refid = 1;  Module.STDWEB.acquire_rust_reference = function( reference ) {     if( reference === undefined || reference === null ) {         return 0;     }      var refid = ref_to_id_map.get( reference );     if( refid === undefined ) {         refid = ref_to_id_symbol_map[ reference ];     }      if( refid === undefined ) {         refid = last_refid++;         if( typeof reference === "symbol" ) {             ref_to_id_symbol_map[ reference ] = refid;         } else {             ref_to_id_map.set( reference, refid );         }         id_to_ref_map[ refid ] = reference;         id_to_refcount_map[ refid ] = 1;     } else {         id_to_refcount_map[ refid ]++;     }      return refid; };  Module.STDWEB.acquire_js_reference = function( refid ) {     return id_to_ref_map[ refid ]; };  Module.STDWEB.increment_refcount = function( refid ) {     id_to_refcount_map[ refid ]++; };  Module.STDWEB.decrement_refcount = function( refid ) {     id_to_refcount_map[ refid ]--;     if( id_to_refcount_map[ refid ] === 0 ) {         var reference = id_to_ref_map[ refid ];         delete id_to_ref_map[ refid ];         delete id_to_refcount_map[ refid ];         if( typeof reference === "symbol" ) {             delete ref_to_id_symbol_map[ reference ];         } else {             ref_to_id_map.delete( reference );         }     } }; Module.STDWEB.alloc = Module.web_malloc; Module.STDWEB.dyncall = function( signature, ptr, args ) {     return Module.web_table.get( ptr ).apply( null, args ); };  Module.STDWEB.utf8_len = function utf8_len( str ) {     let len = 0;     for( let i = 0; i < str.length; ++i ) {                           let u = str.charCodeAt( i );          if( u >= 0xD800 && u <= 0xDFFF ) {             u = 0x10000 + ((u & 0x3FF) << 10) | (str.charCodeAt( ++i ) & 0x3FF);         }          if( u <= 0x7F ) {             ++len;         } else if( u <= 0x7FF ) {             len += 2;         } else if( u <= 0xFFFF ) {             len += 3;         } else if( u <= 0x1FFFFF ) {             len += 4;         } else if( u <= 0x3FFFFFF ) {             len += 5;         } else {             len += 6;         }     }     return len; };
            },
            "__extjs_dc2fd915bd92f9e9c6a3bd15174f1414eee3dbaf": function() {
                console.error( 'Encountered a panic!' );
            },
            "__extjs_b00b05929b445348eab177b6d3f509bcaa28782e": function($0, $1) {
                console.error( 'Panic error message:', Module.STDWEB.to_js_string( $0, $1 ) );
            },
            "__extjs_20637d8f642203b38c263a5d0f43b9d88ec67c31": function($0, $1, $2) {
                console.error( 'Panic location:', Module.STDWEB.to_js_string( $0, $1 ) + ':' + $2 );
            },
            "__web_on_grow": function() {
                const buffer = Module.instance.exports.memory.buffer;
                HEAP8 = new Int8Array( buffer );
                HEAP16 = new Int16Array( buffer );
                HEAP32 = new Int32Array( buffer );
                HEAPU8 = new Uint8Array( buffer );
                HEAPU16 = new Uint16Array( buffer );
                HEAPU32 = new Uint32Array( buffer );
                HEAPF32 = new Float32Array( buffer );
                HEAPF64 = new Float64Array( buffer );
            }
        }
    };

    function __load( instance ) {
        Object.defineProperty( Module, 'instance', { value: instance } );
        Object.defineProperty( Module, 'web_malloc', { value: Module.instance.exports.__web_malloc } );
        Object.defineProperty( Module, 'web_free', { value: Module.instance.exports.__web_free } );
        Object.defineProperty( Module, 'web_table', { value: Module.instance.exports.__web_table } );

        if( typeof module !== 'undefined' && module.exports ) {
            module.exports = Module.exports;
        } else {
            Rust.wasm_webgl_rs.exports = Module.exports;
        }

        __imports.env.__web_on_grow();
        Module.instance.exports.__web_main();
    }

    if( Module.nodejs ) {
        const fs = require( 'fs' );
        const path = require( 'path' );
        const wasm_path = path.join( __dirname, "wasm-webgl-rs.wasm" );
        const buffer = fs.readFileSync( wasm_path );
        const mod = new WebAssembly.Module( buffer );
        const instance = new WebAssembly.Instance( mod, __imports );
        __load( instance );
        return Module.exports;
    } else {
        const __promise = fetch( "wasm-webgl-rs.wasm" )
            .then( response => response.arrayBuffer() )
            .then( bytes => WebAssembly.instantiate( bytes, __imports ) )
            .then( results => {
                __load( results.instance );
                console.log( "Finished loading Rust wasm module 'wasm_webgl_rs'" );
                return Module.exports;
            })
            .catch( error => {
                console.log( "Error loading Rust wasm module 'wasm_webgl_rs':", error );
                throw error;
            });

        Rust.wasm_webgl_rs = __promise;
        return __promise;
    }
}));
