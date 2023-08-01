
            // TODO we should centralize/standardize where we add global variables to the JS, we are doing this in multiple places (i.e. the exports variable is not here, found in init/post_upgrade)
            globalThis.console = {
                ...globalThis.console,
                log: (...args) => {
                    ic.print(...args);
                }
            };

            
Object.defineProperty(exports, "__esModule", {
    value: true
});
exports.inc = exports.add = exports.get = exports.Principal = void 0;
var __create = Object.create;
var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __getProtoOf = Object.getPrototypeOf;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __markAsModule = (target)=>__defProp(target, "__esModule", {
        value: true
    })
;
var __commonJS = (cb, mod)=>function __require() {
        return mod || (0, cb[__getOwnPropNames(cb)[0]])((mod = {
            exports: {}
        }).exports, mod), mod.exports;
    }
;
var __reExport = (target, module2, copyDefault, desc)=>{
    if (module2 && typeof module2 === "object" || typeof module2 === "function") {
        for (let key of __getOwnPropNames(module2))if (!__hasOwnProp.call(target, key) && (copyDefault || key !== "default")) __defProp(target, key, {
            get: ()=>module2[key]
            ,
            enumerable: !(desc = __getOwnPropDesc(module2, key)) || desc.enumerable
        });
    }
    return target;
};
var __toESM = (module2, isNodeMode)=>{
    return __reExport(__markAsModule(__defProp(module2 != null ? __create(__getProtoOf(module2)) : {}, "default", !isNodeMode && module2 && module2.__esModule ? {
        get: ()=>module2.default
        ,
        enumerable: true
    } : {
        value: module2,
        enumerable: true
    })), module2);
};
// node_modules/js-sha256/src/sha256.js
var require_sha256 = __commonJS({
    "node_modules/js-sha256/src/sha256.js" (exports1, module) {
        (function() {
            
            var ERROR = "input is invalid type";
            var WINDOW = typeof window === "object";
            var root = WINDOW ? window : {};
            if (root.JS_SHA256_NO_WINDOW) {
                WINDOW = false;
            }
            var WEB_WORKER = !WINDOW && typeof self === "object";
            var NODE_JS = !root.JS_SHA256_NO_NODE_JS && typeof process === "object" && process.versions && process.versions.node;
            if (NODE_JS) {
                root = global;
            } else if (WEB_WORKER) {
                root = self;
            }
            var COMMON_JS = !root.JS_SHA256_NO_COMMON_JS && typeof module === "object" && module.exports;
            var AMD = typeof define === "function" && define.amd;
            var ARRAY_BUFFER = !root.JS_SHA256_NO_ARRAY_BUFFER && typeof ArrayBuffer !== "undefined";
            var HEX_CHARS = "0123456789abcdef".split("");
            var EXTRA = [
                -2147483648,
                8388608,
                32768,
                128
            ];
            var SHIFT = [
                24,
                16,
                8,
                0
            ];
            var K = [
                1116352408,
                1899447441,
                3049323471,
                3921009573,
                961987163,
                1508970993,
                2453635748,
                2870763221,
                3624381080,
                310598401,
                607225278,
                1426881987,
                1925078388,
                2162078206,
                2614888103,
                3248222580,
                3835390401,
                4022224774,
                264347078,
                604807628,
                770255983,
                1249150122,
                1555081692,
                1996064986,
                2554220882,
                2821834349,
                2952996808,
                3210313671,
                3336571891,
                3584528711,
                113926993,
                338241895,
                666307205,
                773529912,
                1294757372,
                1396182291,
                1695183700,
                1986661051,
                2177026350,
                2456956037,
                2730485921,
                2820302411,
                3259730800,
                3345764771,
                3516065817,
                3600352804,
                4094571909,
                275423344,
                430227734,
                506948616,
                659060556,
                883997877,
                958139571,
                1322822218,
                1537002063,
                1747873779,
                1955562222,
                2024104815,
                2227730452,
                2361852424,
                2428436474,
                2756734187,
                3204031479,
                3329325298
            ];
            var OUTPUT_TYPES = [
                "hex",
                "array",
                "digest",
                "arrayBuffer"
            ];
            var blocks = [];
            if (root.JS_SHA256_NO_NODE_JS || !Array.isArray) {
                Array.isArray = function(obj) {
                    return Object.prototype.toString.call(obj) === "[object Array]";
                };
            }
            if (ARRAY_BUFFER && (root.JS_SHA256_NO_ARRAY_BUFFER_IS_VIEW || !ArrayBuffer.isView)) {
                ArrayBuffer.isView = function(obj) {
                    return typeof obj === "object" && obj.buffer && obj.buffer.constructor === ArrayBuffer;
                };
            }
            var createOutputMethod = function(outputType, is2242) {
                return function(message) {
                    return new Sha256(is2242, true).update(message)[outputType]();
                };
            };
            var createMethod = function(is2242) {
                var method2 = createOutputMethod("hex", is2242);
                if (NODE_JS) {
                    method2 = nodeWrap(method2, is2242);
                }
                method2.create = function() {
                    return new Sha256(is2242);
                };
                method2.update = function(message) {
                    return method2.create().update(message);
                };
                for(var i2 = 0; i2 < OUTPUT_TYPES.length; ++i2){
                    var type = OUTPUT_TYPES[i2];
                    method2[type] = createOutputMethod(type, is2242);
                }
                return method2;
            };
            var nodeWrap = function(method, is224) {
                var crypto = eval("require('crypto')");
                var Buffer = eval("require('buffer').Buffer");
                var algorithm = is224 ? "sha224" : "sha256";
                var nodeMethod = function(message) {
                    if (typeof message === "string") {
                        return crypto.createHash(algorithm).update(message, "utf8").digest("hex");
                    } else {
                        if (message === null || message === void 0) {
                            throw new Error(ERROR);
                        } else if (message.constructor === ArrayBuffer) {
                            message = new Uint8Array(message);
                        }
                    }
                    if (Array.isArray(message) || ArrayBuffer.isView(message) || message.constructor === Buffer) {
                        return crypto.createHash(algorithm).update(new Buffer(message)).digest("hex");
                    } else {
                        return method(message);
                    }
                };
                return nodeMethod;
            };
            var createHmacOutputMethod = function(outputType, is2242) {
                return function(key, message) {
                    return new HmacSha256(key, is2242, true).update(message)[outputType]();
                };
            };
            var createHmacMethod = function(is2242) {
                var method2 = createHmacOutputMethod("hex", is2242);
                method2.create = function(key) {
                    return new HmacSha256(key, is2242);
                };
                method2.update = function(key, message) {
                    return method2.create(key).update(message);
                };
                for(var i3 = 0; i3 < OUTPUT_TYPES.length; ++i3){
                    var type = OUTPUT_TYPES[i3];
                    method2[type] = createHmacOutputMethod(type, is2242);
                }
                return method2;
            };
            function Sha256(is2242, sharedMemory) {
                if (sharedMemory) {
                    blocks[0] = blocks[16] = blocks[1] = blocks[2] = blocks[3] = blocks[4] = blocks[5] = blocks[6] = blocks[7] = blocks[8] = blocks[9] = blocks[10] = blocks[11] = blocks[12] = blocks[13] = blocks[14] = blocks[15] = 0;
                    this.blocks = blocks;
                } else {
                    this.blocks = [
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0
                    ];
                }
                if (is2242) {
                    this.h0 = 3238371032;
                    this.h1 = 914150663;
                    this.h2 = 812702999;
                    this.h3 = 4144912697;
                    this.h4 = 4290775857;
                    this.h5 = 1750603025;
                    this.h6 = 1694076839;
                    this.h7 = 3204075428;
                } else {
                    this.h0 = 1779033703;
                    this.h1 = 3144134277;
                    this.h2 = 1013904242;
                    this.h3 = 2773480762;
                    this.h4 = 1359893119;
                    this.h5 = 2600822924;
                    this.h6 = 528734635;
                    this.h7 = 1541459225;
                }
                this.block = this.start = this.bytes = this.hBytes = 0;
                this.finalized = this.hashed = false;
                this.first = true;
                this.is224 = is2242;
            }
            Sha256.prototype.update = function(message) {
                if (this.finalized) {
                    return;
                }
                var notString, type = typeof message;
                if (type !== "string") {
                    if (type === "object") {
                        if (message === null) {
                            throw new Error(ERROR);
                        } else if (ARRAY_BUFFER && message.constructor === ArrayBuffer) {
                            message = new Uint8Array(message);
                        } else if (!Array.isArray(message)) {
                            if (!ARRAY_BUFFER || !ArrayBuffer.isView(message)) {
                                throw new Error(ERROR);
                            }
                        }
                    } else {
                        throw new Error(ERROR);
                    }
                    notString = true;
                }
                var code, index = 0, i4, length = message.length, blocks2 = this.blocks;
                while(index < length){
                    if (this.hashed) {
                        this.hashed = false;
                        blocks2[0] = this.block;
                        blocks2[16] = blocks2[1] = blocks2[2] = blocks2[3] = blocks2[4] = blocks2[5] = blocks2[6] = blocks2[7] = blocks2[8] = blocks2[9] = blocks2[10] = blocks2[11] = blocks2[12] = blocks2[13] = blocks2[14] = blocks2[15] = 0;
                    }
                    if (notString) {
                        for(i4 = this.start; index < length && i4 < 64; ++index){
                            blocks2[i4 >> 2] |= message[index] << SHIFT[(i4++) & 3];
                        }
                    } else {
                        for(i4 = this.start; index < length && i4 < 64; ++index){
                            code = message.charCodeAt(index);
                            if (code < 128) {
                                blocks2[i4 >> 2] |= code << SHIFT[(i4++) & 3];
                            } else if (code < 2048) {
                                blocks2[i4 >> 2] |= (192 | code >> 6) << SHIFT[(i4++) & 3];
                                blocks2[i4 >> 2] |= (128 | code & 63) << SHIFT[(i4++) & 3];
                            } else if (code < 55296 || code >= 57344) {
                                blocks2[i4 >> 2] |= (224 | code >> 12) << SHIFT[(i4++) & 3];
                                blocks2[i4 >> 2] |= (128 | code >> 6 & 63) << SHIFT[(i4++) & 3];
                                blocks2[i4 >> 2] |= (128 | code & 63) << SHIFT[(i4++) & 3];
                            } else {
                                code = 65536 + ((code & 1023) << 10 | message.charCodeAt(++index) & 1023);
                                blocks2[i4 >> 2] |= (240 | code >> 18) << SHIFT[(i4++) & 3];
                                blocks2[i4 >> 2] |= (128 | code >> 12 & 63) << SHIFT[(i4++) & 3];
                                blocks2[i4 >> 2] |= (128 | code >> 6 & 63) << SHIFT[(i4++) & 3];
                                blocks2[i4 >> 2] |= (128 | code & 63) << SHIFT[(i4++) & 3];
                            }
                        }
                    }
                    this.lastByteIndex = i4;
                    this.bytes += i4 - this.start;
                    if (i4 >= 64) {
                        this.block = blocks2[16];
                        this.start = i4 - 64;
                        this.hash();
                        this.hashed = true;
                    } else {
                        this.start = i4;
                    }
                }
                if (this.bytes > 4294967295) {
                    this.hBytes += this.bytes / 4294967296 << 0;
                    this.bytes = this.bytes % 4294967296;
                }
                return this;
            };
            Sha256.prototype.finalize = function() {
                if (this.finalized) {
                    return;
                }
                this.finalized = true;
                var blocks2 = this.blocks, i5 = this.lastByteIndex;
                blocks2[16] = this.block;
                blocks2[i5 >> 2] |= EXTRA[i5 & 3];
                this.block = blocks2[16];
                if (i5 >= 56) {
                    if (!this.hashed) {
                        this.hash();
                    }
                    blocks2[0] = this.block;
                    blocks2[16] = blocks2[1] = blocks2[2] = blocks2[3] = blocks2[4] = blocks2[5] = blocks2[6] = blocks2[7] = blocks2[8] = blocks2[9] = blocks2[10] = blocks2[11] = blocks2[12] = blocks2[13] = blocks2[14] = blocks2[15] = 0;
                }
                blocks2[14] = this.hBytes << 3 | this.bytes >>> 29;
                blocks2[15] = this.bytes << 3;
                this.hash();
            };
            Sha256.prototype.hash = function() {
                var a = this.h0, b = this.h1, c = this.h2, d = this.h3, e = this.h4, f = this.h5, g = this.h6, h = this.h7, blocks2 = this.blocks, j, s0, s1, maj, t1, t2, ch, ab, da, cd, bc;
                for(j = 16; j < 64; ++j){
                    t1 = blocks2[j - 15];
                    s0 = (t1 >>> 7 | t1 << 25) ^ (t1 >>> 18 | t1 << 14) ^ t1 >>> 3;
                    t1 = blocks2[j - 2];
                    s1 = (t1 >>> 17 | t1 << 15) ^ (t1 >>> 19 | t1 << 13) ^ t1 >>> 10;
                    blocks2[j] = blocks2[j - 16] + s0 + blocks2[j - 7] + s1 << 0;
                }
                bc = b & c;
                for(j = 0; j < 64; j += 4){
                    if (this.first) {
                        if (this.is224) {
                            ab = 300032;
                            t1 = blocks2[0] - 1413257819;
                            h = t1 - 150054599 << 0;
                            d = t1 + 24177077 << 0;
                        } else {
                            ab = 704751109;
                            t1 = blocks2[0] - 210244248;
                            h = t1 - 1521486534 << 0;
                            d = t1 + 143694565 << 0;
                        }
                        this.first = false;
                    } else {
                        s0 = (a >>> 2 | a << 30) ^ (a >>> 13 | a << 19) ^ (a >>> 22 | a << 10);
                        s1 = (e >>> 6 | e << 26) ^ (e >>> 11 | e << 21) ^ (e >>> 25 | e << 7);
                        ab = a & b;
                        maj = ab ^ a & c ^ bc;
                        ch = e & f ^ ~e & g;
                        t1 = h + s1 + ch + K[j] + blocks2[j];
                        t2 = s0 + maj;
                        h = d + t1 << 0;
                        d = t1 + t2 << 0;
                    }
                    s0 = (d >>> 2 | d << 30) ^ (d >>> 13 | d << 19) ^ (d >>> 22 | d << 10);
                    s1 = (h >>> 6 | h << 26) ^ (h >>> 11 | h << 21) ^ (h >>> 25 | h << 7);
                    da = d & a;
                    maj = da ^ d & b ^ ab;
                    ch = h & e ^ ~h & f;
                    t1 = g + s1 + ch + K[j + 1] + blocks2[j + 1];
                    t2 = s0 + maj;
                    g = c + t1 << 0;
                    c = t1 + t2 << 0;
                    s0 = (c >>> 2 | c << 30) ^ (c >>> 13 | c << 19) ^ (c >>> 22 | c << 10);
                    s1 = (g >>> 6 | g << 26) ^ (g >>> 11 | g << 21) ^ (g >>> 25 | g << 7);
                    cd = c & d;
                    maj = cd ^ c & a ^ da;
                    ch = g & h ^ ~g & e;
                    t1 = f + s1 + ch + K[j + 2] + blocks2[j + 2];
                    t2 = s0 + maj;
                    f = b + t1 << 0;
                    b = t1 + t2 << 0;
                    s0 = (b >>> 2 | b << 30) ^ (b >>> 13 | b << 19) ^ (b >>> 22 | b << 10);
                    s1 = (f >>> 6 | f << 26) ^ (f >>> 11 | f << 21) ^ (f >>> 25 | f << 7);
                    bc = b & c;
                    maj = bc ^ b & d ^ cd;
                    ch = f & g ^ ~f & h;
                    t1 = e + s1 + ch + K[j + 3] + blocks2[j + 3];
                    t2 = s0 + maj;
                    e = a + t1 << 0;
                    a = t1 + t2 << 0;
                }
                this.h0 = this.h0 + a << 0;
                this.h1 = this.h1 + b << 0;
                this.h2 = this.h2 + c << 0;
                this.h3 = this.h3 + d << 0;
                this.h4 = this.h4 + e << 0;
                this.h5 = this.h5 + f << 0;
                this.h6 = this.h6 + g << 0;
                this.h7 = this.h7 + h << 0;
            };
            Sha256.prototype.hex = function() {
                this.finalize();
                var h0 = this.h0, h1 = this.h1, h2 = this.h2, h3 = this.h3, h4 = this.h4, h5 = this.h5, h6 = this.h6, h7 = this.h7;
                var hex = HEX_CHARS[h0 >> 28 & 15] + HEX_CHARS[h0 >> 24 & 15] + HEX_CHARS[h0 >> 20 & 15] + HEX_CHARS[h0 >> 16 & 15] + HEX_CHARS[h0 >> 12 & 15] + HEX_CHARS[h0 >> 8 & 15] + HEX_CHARS[h0 >> 4 & 15] + HEX_CHARS[h0 & 15] + HEX_CHARS[h1 >> 28 & 15] + HEX_CHARS[h1 >> 24 & 15] + HEX_CHARS[h1 >> 20 & 15] + HEX_CHARS[h1 >> 16 & 15] + HEX_CHARS[h1 >> 12 & 15] + HEX_CHARS[h1 >> 8 & 15] + HEX_CHARS[h1 >> 4 & 15] + HEX_CHARS[h1 & 15] + HEX_CHARS[h2 >> 28 & 15] + HEX_CHARS[h2 >> 24 & 15] + HEX_CHARS[h2 >> 20 & 15] + HEX_CHARS[h2 >> 16 & 15] + HEX_CHARS[h2 >> 12 & 15] + HEX_CHARS[h2 >> 8 & 15] + HEX_CHARS[h2 >> 4 & 15] + HEX_CHARS[h2 & 15] + HEX_CHARS[h3 >> 28 & 15] + HEX_CHARS[h3 >> 24 & 15] + HEX_CHARS[h3 >> 20 & 15] + HEX_CHARS[h3 >> 16 & 15] + HEX_CHARS[h3 >> 12 & 15] + HEX_CHARS[h3 >> 8 & 15] + HEX_CHARS[h3 >> 4 & 15] + HEX_CHARS[h3 & 15] + HEX_CHARS[h4 >> 28 & 15] + HEX_CHARS[h4 >> 24 & 15] + HEX_CHARS[h4 >> 20 & 15] + HEX_CHARS[h4 >> 16 & 15] + HEX_CHARS[h4 >> 12 & 15] + HEX_CHARS[h4 >> 8 & 15] + HEX_CHARS[h4 >> 4 & 15] + HEX_CHARS[h4 & 15] + HEX_CHARS[h5 >> 28 & 15] + HEX_CHARS[h5 >> 24 & 15] + HEX_CHARS[h5 >> 20 & 15] + HEX_CHARS[h5 >> 16 & 15] + HEX_CHARS[h5 >> 12 & 15] + HEX_CHARS[h5 >> 8 & 15] + HEX_CHARS[h5 >> 4 & 15] + HEX_CHARS[h5 & 15] + HEX_CHARS[h6 >> 28 & 15] + HEX_CHARS[h6 >> 24 & 15] + HEX_CHARS[h6 >> 20 & 15] + HEX_CHARS[h6 >> 16 & 15] + HEX_CHARS[h6 >> 12 & 15] + HEX_CHARS[h6 >> 8 & 15] + HEX_CHARS[h6 >> 4 & 15] + HEX_CHARS[h6 & 15];
                if (!this.is224) {
                    hex += HEX_CHARS[h7 >> 28 & 15] + HEX_CHARS[h7 >> 24 & 15] + HEX_CHARS[h7 >> 20 & 15] + HEX_CHARS[h7 >> 16 & 15] + HEX_CHARS[h7 >> 12 & 15] + HEX_CHARS[h7 >> 8 & 15] + HEX_CHARS[h7 >> 4 & 15] + HEX_CHARS[h7 & 15];
                }
                return hex;
            };
            Sha256.prototype.toString = Sha256.prototype.hex;
            Sha256.prototype.digest = function() {
                this.finalize();
                var h0 = this.h0, h1 = this.h1, h2 = this.h2, h3 = this.h3, h4 = this.h4, h5 = this.h5, h6 = this.h6, h7 = this.h7;
                var arr = [
                    h0 >> 24 & 255,
                    h0 >> 16 & 255,
                    h0 >> 8 & 255,
                    h0 & 255,
                    h1 >> 24 & 255,
                    h1 >> 16 & 255,
                    h1 >> 8 & 255,
                    h1 & 255,
                    h2 >> 24 & 255,
                    h2 >> 16 & 255,
                    h2 >> 8 & 255,
                    h2 & 255,
                    h3 >> 24 & 255,
                    h3 >> 16 & 255,
                    h3 >> 8 & 255,
                    h3 & 255,
                    h4 >> 24 & 255,
                    h4 >> 16 & 255,
                    h4 >> 8 & 255,
                    h4 & 255,
                    h5 >> 24 & 255,
                    h5 >> 16 & 255,
                    h5 >> 8 & 255,
                    h5 & 255,
                    h6 >> 24 & 255,
                    h6 >> 16 & 255,
                    h6 >> 8 & 255,
                    h6 & 255
                ];
                if (!this.is224) {
                    arr.push(h7 >> 24 & 255, h7 >> 16 & 255, h7 >> 8 & 255, h7 & 255);
                }
                return arr;
            };
            Sha256.prototype.array = Sha256.prototype.digest;
            Sha256.prototype.arrayBuffer = function() {
                this.finalize();
                var buffer = new ArrayBuffer(this.is224 ? 28 : 32);
                var dataView = new DataView(buffer);
                dataView.setUint32(0, this.h0);
                dataView.setUint32(4, this.h1);
                dataView.setUint32(8, this.h2);
                dataView.setUint32(12, this.h3);
                dataView.setUint32(16, this.h4);
                dataView.setUint32(20, this.h5);
                dataView.setUint32(24, this.h6);
                if (!this.is224) {
                    dataView.setUint32(28, this.h7);
                }
                return buffer;
            };
            function HmacSha256(key, is2242, sharedMemory) {
                var i6, type = typeof key;
                if (type === "string") {
                    var bytes = [], length = key.length, index = 0, code;
                    for(i6 = 0; i6 < length; ++i6){
                        code = key.charCodeAt(i6);
                        if (code < 128) {
                            bytes[index++] = code;
                        } else if (code < 2048) {
                            bytes[index++] = 192 | code >> 6;
                            bytes[index++] = 128 | code & 63;
                        } else if (code < 55296 || code >= 57344) {
                            bytes[index++] = 224 | code >> 12;
                            bytes[index++] = 128 | code >> 6 & 63;
                            bytes[index++] = 128 | code & 63;
                        } else {
                            code = 65536 + ((code & 1023) << 10 | key.charCodeAt(++i6) & 1023);
                            bytes[index++] = 240 | code >> 18;
                            bytes[index++] = 128 | code >> 12 & 63;
                            bytes[index++] = 128 | code >> 6 & 63;
                            bytes[index++] = 128 | code & 63;
                        }
                    }
                    key = bytes;
                } else {
                    if (type === "object") {
                        if (key === null) {
                            throw new Error(ERROR);
                        } else if (ARRAY_BUFFER && key.constructor === ArrayBuffer) {
                            key = new Uint8Array(key);
                        } else if (!Array.isArray(key)) {
                            if (!ARRAY_BUFFER || !ArrayBuffer.isView(key)) {
                                throw new Error(ERROR);
                            }
                        }
                    } else {
                        throw new Error(ERROR);
                    }
                }
                if (key.length > 64) {
                    key = new Sha256(is2242, true).update(key).array();
                }
                var oKeyPad = [], iKeyPad = [];
                for(i6 = 0; i6 < 64; ++i6){
                    var b = key[i6] || 0;
                    oKeyPad[i6] = 92 ^ b;
                    iKeyPad[i6] = 54 ^ b;
                }
                Sha256.call(this, is2242, sharedMemory);
                this.update(iKeyPad);
                this.oKeyPad = oKeyPad;
                this.inner = true;
                this.sharedMemory = sharedMemory;
            }
            HmacSha256.prototype = new Sha256();
            HmacSha256.prototype.finalize = function() {
                Sha256.prototype.finalize.call(this);
                if (this.inner) {
                    this.inner = false;
                    var innerHash = this.array();
                    Sha256.call(this, this.is224, this.sharedMemory);
                    this.update(this.oKeyPad);
                    this.update(innerHash);
                    Sha256.prototype.finalize.call(this);
                }
            };
            var exports = createMethod();
            exports.sha256 = exports;
            exports.sha224 = createMethod(true);
            exports.sha256.hmac = createHmacMethod();
            exports.sha224.hmac = createHmacMethod(true);
            if (COMMON_JS) {
                module.exports = exports;
            } else {
                root.sha256 = exports.sha256;
                root.sha224 = exports.sha224;
                if (AMD) {
                    define(function() {
                        return exports;
                    });
                }
            }
        })();
    }
});
// node_modules/@dfinity/principal/lib/esm/utils/base32.js
var alphabet = "abcdefghijklmnopqrstuvwxyz234567";
var lookupTable = /* @__PURE__ */ Object.create(null);
for(let i = 0; i < alphabet.length; i++){
    lookupTable[alphabet[i]] = i;
}
lookupTable["0"] = lookupTable.o;
lookupTable["1"] = lookupTable.i;
function encode(input) {
    let skip = 0;
    let bits = 0;
    let output = "";
    function encodeByte(byte) {
        if (skip < 0) {
            bits |= byte >> -skip;
        } else {
            bits = byte << skip & 248;
        }
        if (skip > 3) {
            skip -= 8;
            return 1;
        }
        if (skip < 4) {
            output += alphabet[bits >> 3];
            skip += 5;
        }
        return 0;
    }
    for(let i7 = 0; i7 < input.length;){
        i7 += encodeByte(input[i7]);
    }
    return output + (skip < 0 ? alphabet[bits >> 3] : "");
}
function decode(input) {
    let skip = 0;
    let byte = 0;
    const output = new Uint8Array(input.length * 4 / 3 | 0);
    let o = 0;
    function decodeChar(char) {
        let val = lookupTable[char.toLowerCase()];
        if (val === void 0) {
            throw new Error(`Invalid character: ${JSON.stringify(char)}`);
        }
        val <<= 3;
        byte |= val >>> skip;
        skip += 5;
        if (skip >= 8) {
            output[o++] = byte;
            skip -= 8;
            if (skip > 0) {
                byte = val << 5 - skip & 255;
            } else {
                byte = 0;
            }
        }
    }
    for (const c of input){
        decodeChar(c);
    }
    return output.slice(0, o);
}
// node_modules/@dfinity/principal/lib/esm/utils/getCrc.js
var lookUpTable = new Uint32Array([
    0,
    1996959894,
    3993919788,
    2567524794,
    124634137,
    1886057615,
    3915621685,
    2657392035,
    249268274,
    2044508324,
    3772115230,
    2547177864,
    162941995,
    2125561021,
    3887607047,
    2428444049,
    498536548,
    1789927666,
    4089016648,
    2227061214,
    450548861,
    1843258603,
    4107580753,
    2211677639,
    325883990,
    1684777152,
    4251122042,
    2321926636,
    335633487,
    1661365465,
    4195302755,
    2366115317,
    997073096,
    1281953886,
    3579855332,
    2724688242,
    1006888145,
    1258607687,
    3524101629,
    2768942443,
    901097722,
    1119000684,
    3686517206,
    2898065728,
    853044451,
    1172266101,
    3705015759,
    2882616665,
    651767980,
    1373503546,
    3369554304,
    3218104598,
    565507253,
    1454621731,
    3485111705,
    3099436303,
    671266974,
    1594198024,
    3322730930,
    2970347812,
    795835527,
    1483230225,
    3244367275,
    3060149565,
    1994146192,
    31158534,
    2563907772,
    4023717930,
    1907459465,
    112637215,
    2680153253,
    3904427059,
    2013776290,
    251722036,
    2517215374,
    3775830040,
    2137656763,
    141376813,
    2439277719,
    3865271297,
    1802195444,
    476864866,
    2238001368,
    4066508878,
    1812370925,
    453092731,
    2181625025,
    4111451223,
    1706088902,
    314042704,
    2344532202,
    4240017532,
    1658658271,
    366619977,
    2362670323,
    4224994405,
    1303535960,
    984961486,
    2747007092,
    3569037538,
    1256170817,
    1037604311,
    2765210733,
    3554079995,
    1131014506,
    879679996,
    2909243462,
    3663771856,
    1141124467,
    855842277,
    2852801631,
    3708648649,
    1342533948,
    654459306,
    3188396048,
    3373015174,
    1466479909,
    544179635,
    3110523913,
    3462522015,
    1591671054,
    702138776,
    2966460450,
    3352799412,
    1504918807,
    783551873,
    3082640443,
    3233442989,
    3988292384,
    2596254646,
    62317068,
    1957810842,
    3939845945,
    2647816111,
    81470997,
    1943803523,
    3814918930,
    2489596804,
    225274430,
    2053790376,
    3826175755,
    2466906013,
    167816743,
    2097651377,
    4027552580,
    2265490386,
    503444072,
    1762050814,
    4150417245,
    2154129355,
    426522225,
    1852507879,
    4275313526,
    2312317920,
    282753626,
    1742555852,
    4189708143,
    2394877945,
    397917763,
    1622183637,
    3604390888,
    2714866558,
    953729732,
    1340076626,
    3518719985,
    2797360999,
    1068828381,
    1219638859,
    3624741850,
    2936675148,
    906185462,
    1090812512,
    3747672003,
    2825379669,
    829329135,
    1181335161,
    3412177804,
    3160834842,
    628085408,
    1382605366,
    3423369109,
    3138078467,
    570562233,
    1426400815,
    3317316542,
    2998733608,
    733239954,
    1555261956,
    3268935591,
    3050360625,
    752459403,
    1541320221,
    2607071920,
    3965973030,
    1969922972,
    40735498,
    2617837225,
    3943577151,
    1913087877,
    83908371,
    2512341634,
    3803740692,
    2075208622,
    213261112,
    2463272603,
    3855990285,
    2094854071,
    198958881,
    2262029012,
    4057260610,
    1759359992,
    534414190,
    2176718541,
    4139329115,
    1873836001,
    414664567,
    2282248934,
    4279200368,
    1711684554,
    285281116,
    2405801727,
    4167216745,
    1634467795,
    376229701,
    2685067896,
    3608007406,
    1308918612,
    956543938,
    2808555105,
    3495958263,
    1231636301,
    1047427035,
    2932959818,
    3654703836,
    1088359270,
    936918000,
    2847714899,
    3736837829,
    1202900863,
    817233897,
    3183342108,
    3401237130,
    1404277552,
    615818150,
    3134207493,
    3453421203,
    1423857449,
    601450431,
    3009837614,
    3294710456,
    1567103746,
    711928724,
    3020668471,
    3272380065,
    1510334235,
    755167117
]);
function getCrc32(buf) {
    const b = new Uint8Array(buf);
    let crc = -1;
    for(let i8 = 0; i8 < b.length; i8++){
        const byte = b[i8];
        const t = (byte ^ crc) & 255;
        crc = lookUpTable[t] ^ crc >>> 8;
    }
    return (crc ^ -1) >>> 0;
}
// node_modules/@dfinity/principal/lib/esm/utils/sha224.js
var import_js_sha256 = __toESM(require_sha256());
function sha224(data) {
    const shaObj = import_js_sha256.sha224.create();
    shaObj.update(data);
    return new Uint8Array(shaObj.array());
}
// node_modules/@dfinity/principal/lib/esm/index.js
var SELF_AUTHENTICATING_SUFFIX = 2;
var ANONYMOUS_SUFFIX = 4;
var MANAGEMENT_CANISTER_PRINCIPAL_HEX_STR = "aaaaa-aa";
var fromHexString = (hexString)=>{
    var _a;
    return new Uint8Array(((_a = hexString.match(/.{1,2}/g)) !== null && _a !== void 0 ? _a : []).map((byte)=>parseInt(byte, 16)
    ));
};
var toHexString = (bytes)=>bytes.reduce((str, byte)=>str + byte.toString(16).padStart(2, "0")
    , "")
;
var Principal = class {
    static anonymous() {
        return new this(new Uint8Array([
            ANONYMOUS_SUFFIX
        ]));
    }
    static managementCanister() {
        return this.fromHex(MANAGEMENT_CANISTER_PRINCIPAL_HEX_STR);
    }
    static selfAuthenticating(publicKey) {
        const sha = sha224(publicKey);
        return new this(new Uint8Array([
            ...sha,
            SELF_AUTHENTICATING_SUFFIX
        ]));
    }
    static from(other) {
        if (typeof other === "string") {
            return Principal.fromText(other);
        } else if (typeof other === "object" && other !== null && other._isPrincipal === true) {
            return new Principal(other._arr);
        }
        throw new Error(`Impossible to convert ${JSON.stringify(other)} to Principal.`);
    }
    static fromHex(hex) {
        return new this(fromHexString(hex));
    }
    static fromText(text2) {
        const canisterIdNoDash = text2.toLowerCase().replace(/-/g, "");
        let arr = decode(canisterIdNoDash);
        arr = arr.slice(4, arr.length);
        const principal = new this(arr);
        if (principal.toText() !== text2) {
            throw new Error(`Principal "${principal.toText()}" does not have a valid checksum (original value "${text2}" may not be a valid Principal ID).`);
        }
        return principal;
    }
    static fromUint8Array(arr) {
        return new this(arr);
    }
    isAnonymous() {
        return this._arr.byteLength === 1 && this._arr[0] === ANONYMOUS_SUFFIX;
    }
    toUint8Array() {
        return this._arr;
    }
    toHex() {
        return toHexString(this._arr).toUpperCase();
    }
    toText() {
        const checksumArrayBuf = new ArrayBuffer(4);
        const view = new DataView(checksumArrayBuf);
        view.setUint32(0, getCrc32(this._arr));
        const checksum = new Uint8Array(checksumArrayBuf);
        const bytes = Uint8Array.from(this._arr);
        const array = new Uint8Array([
            ...checksum,
            ...bytes
        ]);
        const result = encode(array);
        const matches = result.match(/.{1,5}/g);
        if (!matches) {
            throw new Error();
        }
        return matches.join("-");
    }
    toString() {
        return this.toText();
    }
    compareTo(other) {
        for(let i9 = 0; i9 < Math.min(this._arr.length, other._arr.length); i9++){
            if (this._arr[i9] < other._arr[i9]) return "lt";
            else if (this._arr[i9] > other._arr[i9]) return "gt";
        }
        if (this._arr.length < other._arr.length) return "lt";
        if (this._arr.length > other._arr.length) return "gt";
        return "eq";
    }
    ltEq(other) {
        const cmp = this.compareTo(other);
        return cmp == "lt" || cmp == "eq";
    }
    gtEq(other) {
        const cmp = this.compareTo(other);
        return cmp == "gt" || cmp == "eq";
    }
    constructor(_arr){
        this._arr = _arr;
        this._isPrincipal = true;
    }
};
exports.Principal = Principal;
var _ic;
// node_modules/azle/src/lib/ic.ts
var ic = (_ic = globalThis.ic) !== null && _ic !== void 0 ? _ic : {};
// node_modules/azle/node_modules/@dfinity/principal/lib/esm/utils/base32.js
var alphabet2 = "abcdefghijklmnopqrstuvwxyz234567";
var lookupTable2 = /* @__PURE__ */ Object.create(null);
for(let i1 = 0; i1 < alphabet2.length; i1++){
    lookupTable2[alphabet2[i1]] = i1;
}
lookupTable2["0"] = lookupTable2.o;
lookupTable2["1"] = lookupTable2.i;
// node_modules/azle/node_modules/@dfinity/principal/lib/esm/utils/getCrc.js
var lookUpTable2 = new Uint32Array([
    0,
    1996959894,
    3993919788,
    2567524794,
    124634137,
    1886057615,
    3915621685,
    2657392035,
    249268274,
    2044508324,
    3772115230,
    2547177864,
    162941995,
    2125561021,
    3887607047,
    2428444049,
    498536548,
    1789927666,
    4089016648,
    2227061214,
    450548861,
    1843258603,
    4107580753,
    2211677639,
    325883990,
    1684777152,
    4251122042,
    2321926636,
    335633487,
    1661365465,
    4195302755,
    2366115317,
    997073096,
    1281953886,
    3579855332,
    2724688242,
    1006888145,
    1258607687,
    3524101629,
    2768942443,
    901097722,
    1119000684,
    3686517206,
    2898065728,
    853044451,
    1172266101,
    3705015759,
    2882616665,
    651767980,
    1373503546,
    3369554304,
    3218104598,
    565507253,
    1454621731,
    3485111705,
    3099436303,
    671266974,
    1594198024,
    3322730930,
    2970347812,
    795835527,
    1483230225,
    3244367275,
    3060149565,
    1994146192,
    31158534,
    2563907772,
    4023717930,
    1907459465,
    112637215,
    2680153253,
    3904427059,
    2013776290,
    251722036,
    2517215374,
    3775830040,
    2137656763,
    141376813,
    2439277719,
    3865271297,
    1802195444,
    476864866,
    2238001368,
    4066508878,
    1812370925,
    453092731,
    2181625025,
    4111451223,
    1706088902,
    314042704,
    2344532202,
    4240017532,
    1658658271,
    366619977,
    2362670323,
    4224994405,
    1303535960,
    984961486,
    2747007092,
    3569037538,
    1256170817,
    1037604311,
    2765210733,
    3554079995,
    1131014506,
    879679996,
    2909243462,
    3663771856,
    1141124467,
    855842277,
    2852801631,
    3708648649,
    1342533948,
    654459306,
    3188396048,
    3373015174,
    1466479909,
    544179635,
    3110523913,
    3462522015,
    1591671054,
    702138776,
    2966460450,
    3352799412,
    1504918807,
    783551873,
    3082640443,
    3233442989,
    3988292384,
    2596254646,
    62317068,
    1957810842,
    3939845945,
    2647816111,
    81470997,
    1943803523,
    3814918930,
    2489596804,
    225274430,
    2053790376,
    3826175755,
    2466906013,
    167816743,
    2097651377,
    4027552580,
    2265490386,
    503444072,
    1762050814,
    4150417245,
    2154129355,
    426522225,
    1852507879,
    4275313526,
    2312317920,
    282753626,
    1742555852,
    4189708143,
    2394877945,
    397917763,
    1622183637,
    3604390888,
    2714866558,
    953729732,
    1340076626,
    3518719985,
    2797360999,
    1068828381,
    1219638859,
    3624741850,
    2936675148,
    906185462,
    1090812512,
    3747672003,
    2825379669,
    829329135,
    1181335161,
    3412177804,
    3160834842,
    628085408,
    1382605366,
    3423369109,
    3138078467,
    570562233,
    1426400815,
    3317316542,
    2998733608,
    733239954,
    1555261956,
    3268935591,
    3050360625,
    752459403,
    1541320221,
    2607071920,
    3965973030,
    1969922972,
    40735498,
    2617837225,
    3943577151,
    1913087877,
    83908371,
    2512341634,
    3803740692,
    2075208622,
    213261112,
    2463272603,
    3855990285,
    2094854071,
    198958881,
    2262029012,
    4057260610,
    1759359992,
    534414190,
    2176718541,
    4139329115,
    1873836001,
    414664567,
    2282248934,
    4279200368,
    1711684554,
    285281116,
    2405801727,
    4167216745,
    1634467795,
    376229701,
    2685067896,
    3608007406,
    1308918612,
    956543938,
    2808555105,
    3495958263,
    1231636301,
    1047427035,
    2932959818,
    3654703836,
    1088359270,
    936918000,
    2847714899,
    3736837829,
    1202900863,
    817233897,
    3183342108,
    3401237130,
    1404277552,
    615818150,
    3134207493,
    3453421203,
    1423857449,
    601450431,
    3009837614,
    3294710456,
    1567103746,
    711928724,
    3020668471,
    3272380065,
    1510334235,
    755167117
]);
// node_modules/azle/node_modules/@dfinity/principal/lib/esm/utils/sha224.js
var import_js_sha2562 = __toESM(require_sha256());
// node_modules/azle/src/lib/candid_types/index.ts
var Opt = {
    Some: (value)=>({
            Some: value
        })
    ,
    None: Object.freeze({
        None: null
    })
};
// backend/index.ts
var counter = BigInt(0);
function get() {
    return counter;
}
exports.get = get;
function add(n) {
    counter += n;
    return counter;
}
exports.add = add;
function inc() {
    counter += BigInt(1);
    return counter;
}
exports.inc = inc;

        