#![allow(warnings, unused)]
use azle_vm_value_derive::{CdkActTryIntoVmValue, CdkActTryFromVmValue};
use candid::{Decode, Encode};
use rand::Rng as _AzleTraitRng;
use slotmap::Key as _AzleTraitSlotMapKey;
use std::convert::TryInto as _AzleTraitTryInto;
use std::str::FromStr as _AzleTraitFromStr;
thread_local! {
    static BOA_CONTEXT_REF_CELL : std::cell::RefCell < boa_engine::Context < 'static >> =
    { struct Hooks; impl boa_engine::context::HostHooks for Hooks { fn utc_now(& self) ->
    chrono::NaiveDateTime { unwrap_or_trap(|| {
    chrono::NaiveDateTime::from_timestamp_opt((ic_cdk::api::time() / 1_000_000_000) as
    i64, 0,).ok_or_else(|| {
    RuntimeError::String("InternalError: Unable to determine host time".to_string(),) })
    }) } fn local_from_utc(& self, utc : chrono::NaiveDateTime) -> chrono::DateTime <
    chrono::FixedOffset > { chrono::DateTime::from_utc(utc,
    chrono::FixedOffset::east_opt(0).unwrap()) } } let hooks : & dyn
    boa_engine::context::HostHooks = & Hooks; let mut context =
    boa_engine::context::ContextBuilder::new().host_hooks(hooks).build().unwrap_or_else(|
    err | ic_cdk::trap(err.to_string().as_str())); context.runtime_limits_mut()
    .set_loop_iteration_limit(u64::MAX); context.runtime_limits_mut()
    .set_recursion_limit(usize::MAX); std::cell::RefCell::new(context) }; static
    PROMISE_MAP_REF_CELL : std::cell::RefCell < std::collections::HashMap < String,
    boa_engine::JsValue >, > = std::cell::RefCell::new(std::collections::HashMap::new());
    static UUID_REF_CELL : std::cell::RefCell < String > = std::cell::RefCell::new(""
    .to_string()); static METHOD_NAME_REF_CELL : std::cell::RefCell < String > =
    std::cell::RefCell::new("".to_string()); static MANUAL_REF_CELL : std::cell::RefCell
    < bool > = std::cell::RefCell::new(false);
}
static MAIN_JS: &'static str = "\n            // TODO we should centralize/standardize where we add global variables to the JS, we are doing this in multiple places (i.e. the exports variable is not here, found in init/post_upgrade)\n            globalThis.console = {\n                ...globalThis.console,\n                log: (...args) => {\n                    ic.print(...args);\n                }\n            };\n\n            \nObject.defineProperty(exports, \"__esModule\", {\n    value: true\n});\nexports.inc = exports.add = exports.get = exports.Principal = void 0;\nvar __create = Object.create;\nvar __defProp = Object.defineProperty;\nvar __getOwnPropDesc = Object.getOwnPropertyDescriptor;\nvar __getOwnPropNames = Object.getOwnPropertyNames;\nvar __getProtoOf = Object.getPrototypeOf;\nvar __hasOwnProp = Object.prototype.hasOwnProperty;\nvar __markAsModule = (target)=>__defProp(target, \"__esModule\", {\n        value: true\n    })\n;\nvar __commonJS = (cb, mod)=>function __require() {\n        return mod || (0, cb[__getOwnPropNames(cb)[0]])((mod = {\n            exports: {}\n        }).exports, mod), mod.exports;\n    }\n;\nvar __reExport = (target, module2, copyDefault, desc)=>{\n    if (module2 && typeof module2 === \"object\" || typeof module2 === \"function\") {\n        for (let key of __getOwnPropNames(module2))if (!__hasOwnProp.call(target, key) && (copyDefault || key !== \"default\")) __defProp(target, key, {\n            get: ()=>module2[key]\n            ,\n            enumerable: !(desc = __getOwnPropDesc(module2, key)) || desc.enumerable\n        });\n    }\n    return target;\n};\nvar __toESM = (module2, isNodeMode)=>{\n    return __reExport(__markAsModule(__defProp(module2 != null ? __create(__getProtoOf(module2)) : {}, \"default\", !isNodeMode && module2 && module2.__esModule ? {\n        get: ()=>module2.default\n        ,\n        enumerable: true\n    } : {\n        value: module2,\n        enumerable: true\n    })), module2);\n};\n// node_modules/js-sha256/src/sha256.js\nvar require_sha256 = __commonJS({\n    \"node_modules/js-sha256/src/sha256.js\" (exports1, module) {\n        (function() {\n            \n            var ERROR = \"input is invalid type\";\n            var WINDOW = typeof window === \"object\";\n            var root = WINDOW ? window : {};\n            if (root.JS_SHA256_NO_WINDOW) {\n                WINDOW = false;\n            }\n            var WEB_WORKER = !WINDOW && typeof self === \"object\";\n            var NODE_JS = !root.JS_SHA256_NO_NODE_JS && typeof process === \"object\" && process.versions && process.versions.node;\n            if (NODE_JS) {\n                root = global;\n            } else if (WEB_WORKER) {\n                root = self;\n            }\n            var COMMON_JS = !root.JS_SHA256_NO_COMMON_JS && typeof module === \"object\" && module.exports;\n            var AMD = typeof define === \"function\" && define.amd;\n            var ARRAY_BUFFER = !root.JS_SHA256_NO_ARRAY_BUFFER && typeof ArrayBuffer !== \"undefined\";\n            var HEX_CHARS = \"0123456789abcdef\".split(\"\");\n            var EXTRA = [\n                -2147483648,\n                8388608,\n                32768,\n                128\n            ];\n            var SHIFT = [\n                24,\n                16,\n                8,\n                0\n            ];\n            var K = [\n                1116352408,\n                1899447441,\n                3049323471,\n                3921009573,\n                961987163,\n                1508970993,\n                2453635748,\n                2870763221,\n                3624381080,\n                310598401,\n                607225278,\n                1426881987,\n                1925078388,\n                2162078206,\n                2614888103,\n                3248222580,\n                3835390401,\n                4022224774,\n                264347078,\n                604807628,\n                770255983,\n                1249150122,\n                1555081692,\n                1996064986,\n                2554220882,\n                2821834349,\n                2952996808,\n                3210313671,\n                3336571891,\n                3584528711,\n                113926993,\n                338241895,\n                666307205,\n                773529912,\n                1294757372,\n                1396182291,\n                1695183700,\n                1986661051,\n                2177026350,\n                2456956037,\n                2730485921,\n                2820302411,\n                3259730800,\n                3345764771,\n                3516065817,\n                3600352804,\n                4094571909,\n                275423344,\n                430227734,\n                506948616,\n                659060556,\n                883997877,\n                958139571,\n                1322822218,\n                1537002063,\n                1747873779,\n                1955562222,\n                2024104815,\n                2227730452,\n                2361852424,\n                2428436474,\n                2756734187,\n                3204031479,\n                3329325298\n            ];\n            var OUTPUT_TYPES = [\n                \"hex\",\n                \"array\",\n                \"digest\",\n                \"arrayBuffer\"\n            ];\n            var blocks = [];\n            if (root.JS_SHA256_NO_NODE_JS || !Array.isArray) {\n                Array.isArray = function(obj) {\n                    return Object.prototype.toString.call(obj) === \"[object Array]\";\n                };\n            }\n            if (ARRAY_BUFFER && (root.JS_SHA256_NO_ARRAY_BUFFER_IS_VIEW || !ArrayBuffer.isView)) {\n                ArrayBuffer.isView = function(obj) {\n                    return typeof obj === \"object\" && obj.buffer && obj.buffer.constructor === ArrayBuffer;\n                };\n            }\n            var createOutputMethod = function(outputType, is2242) {\n                return function(message) {\n                    return new Sha256(is2242, true).update(message)[outputType]();\n                };\n            };\n            var createMethod = function(is2242) {\n                var method2 = createOutputMethod(\"hex\", is2242);\n                if (NODE_JS) {\n                    method2 = nodeWrap(method2, is2242);\n                }\n                method2.create = function() {\n                    return new Sha256(is2242);\n                };\n                method2.update = function(message) {\n                    return method2.create().update(message);\n                };\n                for(var i2 = 0; i2 < OUTPUT_TYPES.length; ++i2){\n                    var type = OUTPUT_TYPES[i2];\n                    method2[type] = createOutputMethod(type, is2242);\n                }\n                return method2;\n            };\n            var nodeWrap = function(method, is224) {\n                var crypto = eval(\"require('crypto')\");\n                var Buffer = eval(\"require('buffer').Buffer\");\n                var algorithm = is224 ? \"sha224\" : \"sha256\";\n                var nodeMethod = function(message) {\n                    if (typeof message === \"string\") {\n                        return crypto.createHash(algorithm).update(message, \"utf8\").digest(\"hex\");\n                    } else {\n                        if (message === null || message === void 0) {\n                            throw new Error(ERROR);\n                        } else if (message.constructor === ArrayBuffer) {\n                            message = new Uint8Array(message);\n                        }\n                    }\n                    if (Array.isArray(message) || ArrayBuffer.isView(message) || message.constructor === Buffer) {\n                        return crypto.createHash(algorithm).update(new Buffer(message)).digest(\"hex\");\n                    } else {\n                        return method(message);\n                    }\n                };\n                return nodeMethod;\n            };\n            var createHmacOutputMethod = function(outputType, is2242) {\n                return function(key, message) {\n                    return new HmacSha256(key, is2242, true).update(message)[outputType]();\n                };\n            };\n            var createHmacMethod = function(is2242) {\n                var method2 = createHmacOutputMethod(\"hex\", is2242);\n                method2.create = function(key) {\n                    return new HmacSha256(key, is2242);\n                };\n                method2.update = function(key, message) {\n                    return method2.create(key).update(message);\n                };\n                for(var i3 = 0; i3 < OUTPUT_TYPES.length; ++i3){\n                    var type = OUTPUT_TYPES[i3];\n                    method2[type] = createHmacOutputMethod(type, is2242);\n                }\n                return method2;\n            };\n            function Sha256(is2242, sharedMemory) {\n                if (sharedMemory) {\n                    blocks[0] = blocks[16] = blocks[1] = blocks[2] = blocks[3] = blocks[4] = blocks[5] = blocks[6] = blocks[7] = blocks[8] = blocks[9] = blocks[10] = blocks[11] = blocks[12] = blocks[13] = blocks[14] = blocks[15] = 0;\n                    this.blocks = blocks;\n                } else {\n                    this.blocks = [\n                        0,\n                        0,\n                        0,\n                        0,\n                        0,\n                        0,\n                        0,\n                        0,\n                        0,\n                        0,\n                        0,\n                        0,\n                        0,\n                        0,\n                        0,\n                        0,\n                        0\n                    ];\n                }\n                if (is2242) {\n                    this.h0 = 3238371032;\n                    this.h1 = 914150663;\n                    this.h2 = 812702999;\n                    this.h3 = 4144912697;\n                    this.h4 = 4290775857;\n                    this.h5 = 1750603025;\n                    this.h6 = 1694076839;\n                    this.h7 = 3204075428;\n                } else {\n                    this.h0 = 1779033703;\n                    this.h1 = 3144134277;\n                    this.h2 = 1013904242;\n                    this.h3 = 2773480762;\n                    this.h4 = 1359893119;\n                    this.h5 = 2600822924;\n                    this.h6 = 528734635;\n                    this.h7 = 1541459225;\n                }\n                this.block = this.start = this.bytes = this.hBytes = 0;\n                this.finalized = this.hashed = false;\n                this.first = true;\n                this.is224 = is2242;\n            }\n            Sha256.prototype.update = function(message) {\n                if (this.finalized) {\n                    return;\n                }\n                var notString, type = typeof message;\n                if (type !== \"string\") {\n                    if (type === \"object\") {\n                        if (message === null) {\n                            throw new Error(ERROR);\n                        } else if (ARRAY_BUFFER && message.constructor === ArrayBuffer) {\n                            message = new Uint8Array(message);\n                        } else if (!Array.isArray(message)) {\n                            if (!ARRAY_BUFFER || !ArrayBuffer.isView(message)) {\n                                throw new Error(ERROR);\n                            }\n                        }\n                    } else {\n                        throw new Error(ERROR);\n                    }\n                    notString = true;\n                }\n                var code, index = 0, i4, length = message.length, blocks2 = this.blocks;\n                while(index < length){\n                    if (this.hashed) {\n                        this.hashed = false;\n                        blocks2[0] = this.block;\n                        blocks2[16] = blocks2[1] = blocks2[2] = blocks2[3] = blocks2[4] = blocks2[5] = blocks2[6] = blocks2[7] = blocks2[8] = blocks2[9] = blocks2[10] = blocks2[11] = blocks2[12] = blocks2[13] = blocks2[14] = blocks2[15] = 0;\n                    }\n                    if (notString) {\n                        for(i4 = this.start; index < length && i4 < 64; ++index){\n                            blocks2[i4 >> 2] |= message[index] << SHIFT[(i4++) & 3];\n                        }\n                    } else {\n                        for(i4 = this.start; index < length && i4 < 64; ++index){\n                            code = message.charCodeAt(index);\n                            if (code < 128) {\n                                blocks2[i4 >> 2] |= code << SHIFT[(i4++) & 3];\n                            } else if (code < 2048) {\n                                blocks2[i4 >> 2] |= (192 | code >> 6) << SHIFT[(i4++) & 3];\n                                blocks2[i4 >> 2] |= (128 | code & 63) << SHIFT[(i4++) & 3];\n                            } else if (code < 55296 || code >= 57344) {\n                                blocks2[i4 >> 2] |= (224 | code >> 12) << SHIFT[(i4++) & 3];\n                                blocks2[i4 >> 2] |= (128 | code >> 6 & 63) << SHIFT[(i4++) & 3];\n                                blocks2[i4 >> 2] |= (128 | code & 63) << SHIFT[(i4++) & 3];\n                            } else {\n                                code = 65536 + ((code & 1023) << 10 | message.charCodeAt(++index) & 1023);\n                                blocks2[i4 >> 2] |= (240 | code >> 18) << SHIFT[(i4++) & 3];\n                                blocks2[i4 >> 2] |= (128 | code >> 12 & 63) << SHIFT[(i4++) & 3];\n                                blocks2[i4 >> 2] |= (128 | code >> 6 & 63) << SHIFT[(i4++) & 3];\n                                blocks2[i4 >> 2] |= (128 | code & 63) << SHIFT[(i4++) & 3];\n                            }\n                        }\n                    }\n                    this.lastByteIndex = i4;\n                    this.bytes += i4 - this.start;\n                    if (i4 >= 64) {\n                        this.block = blocks2[16];\n                        this.start = i4 - 64;\n                        this.hash();\n                        this.hashed = true;\n                    } else {\n                        this.start = i4;\n                    }\n                }\n                if (this.bytes > 4294967295) {\n                    this.hBytes += this.bytes / 4294967296 << 0;\n                    this.bytes = this.bytes % 4294967296;\n                }\n                return this;\n            };\n            Sha256.prototype.finalize = function() {\n                if (this.finalized) {\n                    return;\n                }\n                this.finalized = true;\n                var blocks2 = this.blocks, i5 = this.lastByteIndex;\n                blocks2[16] = this.block;\n                blocks2[i5 >> 2] |= EXTRA[i5 & 3];\n                this.block = blocks2[16];\n                if (i5 >= 56) {\n                    if (!this.hashed) {\n                        this.hash();\n                    }\n                    blocks2[0] = this.block;\n                    blocks2[16] = blocks2[1] = blocks2[2] = blocks2[3] = blocks2[4] = blocks2[5] = blocks2[6] = blocks2[7] = blocks2[8] = blocks2[9] = blocks2[10] = blocks2[11] = blocks2[12] = blocks2[13] = blocks2[14] = blocks2[15] = 0;\n                }\n                blocks2[14] = this.hBytes << 3 | this.bytes >>> 29;\n                blocks2[15] = this.bytes << 3;\n                this.hash();\n            };\n            Sha256.prototype.hash = function() {\n                var a = this.h0, b = this.h1, c = this.h2, d = this.h3, e = this.h4, f = this.h5, g = this.h6, h = this.h7, blocks2 = this.blocks, j, s0, s1, maj, t1, t2, ch, ab, da, cd, bc;\n                for(j = 16; j < 64; ++j){\n                    t1 = blocks2[j - 15];\n                    s0 = (t1 >>> 7 | t1 << 25) ^ (t1 >>> 18 | t1 << 14) ^ t1 >>> 3;\n                    t1 = blocks2[j - 2];\n                    s1 = (t1 >>> 17 | t1 << 15) ^ (t1 >>> 19 | t1 << 13) ^ t1 >>> 10;\n                    blocks2[j] = blocks2[j - 16] + s0 + blocks2[j - 7] + s1 << 0;\n                }\n                bc = b & c;\n                for(j = 0; j < 64; j += 4){\n                    if (this.first) {\n                        if (this.is224) {\n                            ab = 300032;\n                            t1 = blocks2[0] - 1413257819;\n                            h = t1 - 150054599 << 0;\n                            d = t1 + 24177077 << 0;\n                        } else {\n                            ab = 704751109;\n                            t1 = blocks2[0] - 210244248;\n                            h = t1 - 1521486534 << 0;\n                            d = t1 + 143694565 << 0;\n                        }\n                        this.first = false;\n                    } else {\n                        s0 = (a >>> 2 | a << 30) ^ (a >>> 13 | a << 19) ^ (a >>> 22 | a << 10);\n                        s1 = (e >>> 6 | e << 26) ^ (e >>> 11 | e << 21) ^ (e >>> 25 | e << 7);\n                        ab = a & b;\n                        maj = ab ^ a & c ^ bc;\n                        ch = e & f ^ ~e & g;\n                        t1 = h + s1 + ch + K[j] + blocks2[j];\n                        t2 = s0 + maj;\n                        h = d + t1 << 0;\n                        d = t1 + t2 << 0;\n                    }\n                    s0 = (d >>> 2 | d << 30) ^ (d >>> 13 | d << 19) ^ (d >>> 22 | d << 10);\n                    s1 = (h >>> 6 | h << 26) ^ (h >>> 11 | h << 21) ^ (h >>> 25 | h << 7);\n                    da = d & a;\n                    maj = da ^ d & b ^ ab;\n                    ch = h & e ^ ~h & f;\n                    t1 = g + s1 + ch + K[j + 1] + blocks2[j + 1];\n                    t2 = s0 + maj;\n                    g = c + t1 << 0;\n                    c = t1 + t2 << 0;\n                    s0 = (c >>> 2 | c << 30) ^ (c >>> 13 | c << 19) ^ (c >>> 22 | c << 10);\n                    s1 = (g >>> 6 | g << 26) ^ (g >>> 11 | g << 21) ^ (g >>> 25 | g << 7);\n                    cd = c & d;\n                    maj = cd ^ c & a ^ da;\n                    ch = g & h ^ ~g & e;\n                    t1 = f + s1 + ch + K[j + 2] + blocks2[j + 2];\n                    t2 = s0 + maj;\n                    f = b + t1 << 0;\n                    b = t1 + t2 << 0;\n                    s0 = (b >>> 2 | b << 30) ^ (b >>> 13 | b << 19) ^ (b >>> 22 | b << 10);\n                    s1 = (f >>> 6 | f << 26) ^ (f >>> 11 | f << 21) ^ (f >>> 25 | f << 7);\n                    bc = b & c;\n                    maj = bc ^ b & d ^ cd;\n                    ch = f & g ^ ~f & h;\n                    t1 = e + s1 + ch + K[j + 3] + blocks2[j + 3];\n                    t2 = s0 + maj;\n                    e = a + t1 << 0;\n                    a = t1 + t2 << 0;\n                }\n                this.h0 = this.h0 + a << 0;\n                this.h1 = this.h1 + b << 0;\n                this.h2 = this.h2 + c << 0;\n                this.h3 = this.h3 + d << 0;\n                this.h4 = this.h4 + e << 0;\n                this.h5 = this.h5 + f << 0;\n                this.h6 = this.h6 + g << 0;\n                this.h7 = this.h7 + h << 0;\n            };\n            Sha256.prototype.hex = function() {\n                this.finalize();\n                var h0 = this.h0, h1 = this.h1, h2 = this.h2, h3 = this.h3, h4 = this.h4, h5 = this.h5, h6 = this.h6, h7 = this.h7;\n                var hex = HEX_CHARS[h0 >> 28 & 15] + HEX_CHARS[h0 >> 24 & 15] + HEX_CHARS[h0 >> 20 & 15] + HEX_CHARS[h0 >> 16 & 15] + HEX_CHARS[h0 >> 12 & 15] + HEX_CHARS[h0 >> 8 & 15] + HEX_CHARS[h0 >> 4 & 15] + HEX_CHARS[h0 & 15] + HEX_CHARS[h1 >> 28 & 15] + HEX_CHARS[h1 >> 24 & 15] + HEX_CHARS[h1 >> 20 & 15] + HEX_CHARS[h1 >> 16 & 15] + HEX_CHARS[h1 >> 12 & 15] + HEX_CHARS[h1 >> 8 & 15] + HEX_CHARS[h1 >> 4 & 15] + HEX_CHARS[h1 & 15] + HEX_CHARS[h2 >> 28 & 15] + HEX_CHARS[h2 >> 24 & 15] + HEX_CHARS[h2 >> 20 & 15] + HEX_CHARS[h2 >> 16 & 15] + HEX_CHARS[h2 >> 12 & 15] + HEX_CHARS[h2 >> 8 & 15] + HEX_CHARS[h2 >> 4 & 15] + HEX_CHARS[h2 & 15] + HEX_CHARS[h3 >> 28 & 15] + HEX_CHARS[h3 >> 24 & 15] + HEX_CHARS[h3 >> 20 & 15] + HEX_CHARS[h3 >> 16 & 15] + HEX_CHARS[h3 >> 12 & 15] + HEX_CHARS[h3 >> 8 & 15] + HEX_CHARS[h3 >> 4 & 15] + HEX_CHARS[h3 & 15] + HEX_CHARS[h4 >> 28 & 15] + HEX_CHARS[h4 >> 24 & 15] + HEX_CHARS[h4 >> 20 & 15] + HEX_CHARS[h4 >> 16 & 15] + HEX_CHARS[h4 >> 12 & 15] + HEX_CHARS[h4 >> 8 & 15] + HEX_CHARS[h4 >> 4 & 15] + HEX_CHARS[h4 & 15] + HEX_CHARS[h5 >> 28 & 15] + HEX_CHARS[h5 >> 24 & 15] + HEX_CHARS[h5 >> 20 & 15] + HEX_CHARS[h5 >> 16 & 15] + HEX_CHARS[h5 >> 12 & 15] + HEX_CHARS[h5 >> 8 & 15] + HEX_CHARS[h5 >> 4 & 15] + HEX_CHARS[h5 & 15] + HEX_CHARS[h6 >> 28 & 15] + HEX_CHARS[h6 >> 24 & 15] + HEX_CHARS[h6 >> 20 & 15] + HEX_CHARS[h6 >> 16 & 15] + HEX_CHARS[h6 >> 12 & 15] + HEX_CHARS[h6 >> 8 & 15] + HEX_CHARS[h6 >> 4 & 15] + HEX_CHARS[h6 & 15];\n                if (!this.is224) {\n                    hex += HEX_CHARS[h7 >> 28 & 15] + HEX_CHARS[h7 >> 24 & 15] + HEX_CHARS[h7 >> 20 & 15] + HEX_CHARS[h7 >> 16 & 15] + HEX_CHARS[h7 >> 12 & 15] + HEX_CHARS[h7 >> 8 & 15] + HEX_CHARS[h7 >> 4 & 15] + HEX_CHARS[h7 & 15];\n                }\n                return hex;\n            };\n            Sha256.prototype.toString = Sha256.prototype.hex;\n            Sha256.prototype.digest = function() {\n                this.finalize();\n                var h0 = this.h0, h1 = this.h1, h2 = this.h2, h3 = this.h3, h4 = this.h4, h5 = this.h5, h6 = this.h6, h7 = this.h7;\n                var arr = [\n                    h0 >> 24 & 255,\n                    h0 >> 16 & 255,\n                    h0 >> 8 & 255,\n                    h0 & 255,\n                    h1 >> 24 & 255,\n                    h1 >> 16 & 255,\n                    h1 >> 8 & 255,\n                    h1 & 255,\n                    h2 >> 24 & 255,\n                    h2 >> 16 & 255,\n                    h2 >> 8 & 255,\n                    h2 & 255,\n                    h3 >> 24 & 255,\n                    h3 >> 16 & 255,\n                    h3 >> 8 & 255,\n                    h3 & 255,\n                    h4 >> 24 & 255,\n                    h4 >> 16 & 255,\n                    h4 >> 8 & 255,\n                    h4 & 255,\n                    h5 >> 24 & 255,\n                    h5 >> 16 & 255,\n                    h5 >> 8 & 255,\n                    h5 & 255,\n                    h6 >> 24 & 255,\n                    h6 >> 16 & 255,\n                    h6 >> 8 & 255,\n                    h6 & 255\n                ];\n                if (!this.is224) {\n                    arr.push(h7 >> 24 & 255, h7 >> 16 & 255, h7 >> 8 & 255, h7 & 255);\n                }\n                return arr;\n            };\n            Sha256.prototype.array = Sha256.prototype.digest;\n            Sha256.prototype.arrayBuffer = function() {\n                this.finalize();\n                var buffer = new ArrayBuffer(this.is224 ? 28 : 32);\n                var dataView = new DataView(buffer);\n                dataView.setUint32(0, this.h0);\n                dataView.setUint32(4, this.h1);\n                dataView.setUint32(8, this.h2);\n                dataView.setUint32(12, this.h3);\n                dataView.setUint32(16, this.h4);\n                dataView.setUint32(20, this.h5);\n                dataView.setUint32(24, this.h6);\n                if (!this.is224) {\n                    dataView.setUint32(28, this.h7);\n                }\n                return buffer;\n            };\n            function HmacSha256(key, is2242, sharedMemory) {\n                var i6, type = typeof key;\n                if (type === \"string\") {\n                    var bytes = [], length = key.length, index = 0, code;\n                    for(i6 = 0; i6 < length; ++i6){\n                        code = key.charCodeAt(i6);\n                        if (code < 128) {\n                            bytes[index++] = code;\n                        } else if (code < 2048) {\n                            bytes[index++] = 192 | code >> 6;\n                            bytes[index++] = 128 | code & 63;\n                        } else if (code < 55296 || code >= 57344) {\n                            bytes[index++] = 224 | code >> 12;\n                            bytes[index++] = 128 | code >> 6 & 63;\n                            bytes[index++] = 128 | code & 63;\n                        } else {\n                            code = 65536 + ((code & 1023) << 10 | key.charCodeAt(++i6) & 1023);\n                            bytes[index++] = 240 | code >> 18;\n                            bytes[index++] = 128 | code >> 12 & 63;\n                            bytes[index++] = 128 | code >> 6 & 63;\n                            bytes[index++] = 128 | code & 63;\n                        }\n                    }\n                    key = bytes;\n                } else {\n                    if (type === \"object\") {\n                        if (key === null) {\n                            throw new Error(ERROR);\n                        } else if (ARRAY_BUFFER && key.constructor === ArrayBuffer) {\n                            key = new Uint8Array(key);\n                        } else if (!Array.isArray(key)) {\n                            if (!ARRAY_BUFFER || !ArrayBuffer.isView(key)) {\n                                throw new Error(ERROR);\n                            }\n                        }\n                    } else {\n                        throw new Error(ERROR);\n                    }\n                }\n                if (key.length > 64) {\n                    key = new Sha256(is2242, true).update(key).array();\n                }\n                var oKeyPad = [], iKeyPad = [];\n                for(i6 = 0; i6 < 64; ++i6){\n                    var b = key[i6] || 0;\n                    oKeyPad[i6] = 92 ^ b;\n                    iKeyPad[i6] = 54 ^ b;\n                }\n                Sha256.call(this, is2242, sharedMemory);\n                this.update(iKeyPad);\n                this.oKeyPad = oKeyPad;\n                this.inner = true;\n                this.sharedMemory = sharedMemory;\n            }\n            HmacSha256.prototype = new Sha256();\n            HmacSha256.prototype.finalize = function() {\n                Sha256.prototype.finalize.call(this);\n                if (this.inner) {\n                    this.inner = false;\n                    var innerHash = this.array();\n                    Sha256.call(this, this.is224, this.sharedMemory);\n                    this.update(this.oKeyPad);\n                    this.update(innerHash);\n                    Sha256.prototype.finalize.call(this);\n                }\n            };\n            var exports = createMethod();\n            exports.sha256 = exports;\n            exports.sha224 = createMethod(true);\n            exports.sha256.hmac = createHmacMethod();\n            exports.sha224.hmac = createHmacMethod(true);\n            if (COMMON_JS) {\n                module.exports = exports;\n            } else {\n                root.sha256 = exports.sha256;\n                root.sha224 = exports.sha224;\n                if (AMD) {\n                    define(function() {\n                        return exports;\n                    });\n                }\n            }\n        })();\n    }\n});\n// node_modules/@dfinity/principal/lib/esm/utils/base32.js\nvar alphabet = \"abcdefghijklmnopqrstuvwxyz234567\";\nvar lookupTable = /* @__PURE__ */ Object.create(null);\nfor(let i = 0; i < alphabet.length; i++){\n    lookupTable[alphabet[i]] = i;\n}\nlookupTable[\"0\"] = lookupTable.o;\nlookupTable[\"1\"] = lookupTable.i;\nfunction encode(input) {\n    let skip = 0;\n    let bits = 0;\n    let output = \"\";\n    function encodeByte(byte) {\n        if (skip < 0) {\n            bits |= byte >> -skip;\n        } else {\n            bits = byte << skip & 248;\n        }\n        if (skip > 3) {\n            skip -= 8;\n            return 1;\n        }\n        if (skip < 4) {\n            output += alphabet[bits >> 3];\n            skip += 5;\n        }\n        return 0;\n    }\n    for(let i7 = 0; i7 < input.length;){\n        i7 += encodeByte(input[i7]);\n    }\n    return output + (skip < 0 ? alphabet[bits >> 3] : \"\");\n}\nfunction decode(input) {\n    let skip = 0;\n    let byte = 0;\n    const output = new Uint8Array(input.length * 4 / 3 | 0);\n    let o = 0;\n    function decodeChar(char) {\n        let val = lookupTable[char.toLowerCase()];\n        if (val === void 0) {\n            throw new Error(`Invalid character: ${JSON.stringify(char)}`);\n        }\n        val <<= 3;\n        byte |= val >>> skip;\n        skip += 5;\n        if (skip >= 8) {\n            output[o++] = byte;\n            skip -= 8;\n            if (skip > 0) {\n                byte = val << 5 - skip & 255;\n            } else {\n                byte = 0;\n            }\n        }\n    }\n    for (const c of input){\n        decodeChar(c);\n    }\n    return output.slice(0, o);\n}\n// node_modules/@dfinity/principal/lib/esm/utils/getCrc.js\nvar lookUpTable = new Uint32Array([\n    0,\n    1996959894,\n    3993919788,\n    2567524794,\n    124634137,\n    1886057615,\n    3915621685,\n    2657392035,\n    249268274,\n    2044508324,\n    3772115230,\n    2547177864,\n    162941995,\n    2125561021,\n    3887607047,\n    2428444049,\n    498536548,\n    1789927666,\n    4089016648,\n    2227061214,\n    450548861,\n    1843258603,\n    4107580753,\n    2211677639,\n    325883990,\n    1684777152,\n    4251122042,\n    2321926636,\n    335633487,\n    1661365465,\n    4195302755,\n    2366115317,\n    997073096,\n    1281953886,\n    3579855332,\n    2724688242,\n    1006888145,\n    1258607687,\n    3524101629,\n    2768942443,\n    901097722,\n    1119000684,\n    3686517206,\n    2898065728,\n    853044451,\n    1172266101,\n    3705015759,\n    2882616665,\n    651767980,\n    1373503546,\n    3369554304,\n    3218104598,\n    565507253,\n    1454621731,\n    3485111705,\n    3099436303,\n    671266974,\n    1594198024,\n    3322730930,\n    2970347812,\n    795835527,\n    1483230225,\n    3244367275,\n    3060149565,\n    1994146192,\n    31158534,\n    2563907772,\n    4023717930,\n    1907459465,\n    112637215,\n    2680153253,\n    3904427059,\n    2013776290,\n    251722036,\n    2517215374,\n    3775830040,\n    2137656763,\n    141376813,\n    2439277719,\n    3865271297,\n    1802195444,\n    476864866,\n    2238001368,\n    4066508878,\n    1812370925,\n    453092731,\n    2181625025,\n    4111451223,\n    1706088902,\n    314042704,\n    2344532202,\n    4240017532,\n    1658658271,\n    366619977,\n    2362670323,\n    4224994405,\n    1303535960,\n    984961486,\n    2747007092,\n    3569037538,\n    1256170817,\n    1037604311,\n    2765210733,\n    3554079995,\n    1131014506,\n    879679996,\n    2909243462,\n    3663771856,\n    1141124467,\n    855842277,\n    2852801631,\n    3708648649,\n    1342533948,\n    654459306,\n    3188396048,\n    3373015174,\n    1466479909,\n    544179635,\n    3110523913,\n    3462522015,\n    1591671054,\n    702138776,\n    2966460450,\n    3352799412,\n    1504918807,\n    783551873,\n    3082640443,\n    3233442989,\n    3988292384,\n    2596254646,\n    62317068,\n    1957810842,\n    3939845945,\n    2647816111,\n    81470997,\n    1943803523,\n    3814918930,\n    2489596804,\n    225274430,\n    2053790376,\n    3826175755,\n    2466906013,\n    167816743,\n    2097651377,\n    4027552580,\n    2265490386,\n    503444072,\n    1762050814,\n    4150417245,\n    2154129355,\n    426522225,\n    1852507879,\n    4275313526,\n    2312317920,\n    282753626,\n    1742555852,\n    4189708143,\n    2394877945,\n    397917763,\n    1622183637,\n    3604390888,\n    2714866558,\n    953729732,\n    1340076626,\n    3518719985,\n    2797360999,\n    1068828381,\n    1219638859,\n    3624741850,\n    2936675148,\n    906185462,\n    1090812512,\n    3747672003,\n    2825379669,\n    829329135,\n    1181335161,\n    3412177804,\n    3160834842,\n    628085408,\n    1382605366,\n    3423369109,\n    3138078467,\n    570562233,\n    1426400815,\n    3317316542,\n    2998733608,\n    733239954,\n    1555261956,\n    3268935591,\n    3050360625,\n    752459403,\n    1541320221,\n    2607071920,\n    3965973030,\n    1969922972,\n    40735498,\n    2617837225,\n    3943577151,\n    1913087877,\n    83908371,\n    2512341634,\n    3803740692,\n    2075208622,\n    213261112,\n    2463272603,\n    3855990285,\n    2094854071,\n    198958881,\n    2262029012,\n    4057260610,\n    1759359992,\n    534414190,\n    2176718541,\n    4139329115,\n    1873836001,\n    414664567,\n    2282248934,\n    4279200368,\n    1711684554,\n    285281116,\n    2405801727,\n    4167216745,\n    1634467795,\n    376229701,\n    2685067896,\n    3608007406,\n    1308918612,\n    956543938,\n    2808555105,\n    3495958263,\n    1231636301,\n    1047427035,\n    2932959818,\n    3654703836,\n    1088359270,\n    936918000,\n    2847714899,\n    3736837829,\n    1202900863,\n    817233897,\n    3183342108,\n    3401237130,\n    1404277552,\n    615818150,\n    3134207493,\n    3453421203,\n    1423857449,\n    601450431,\n    3009837614,\n    3294710456,\n    1567103746,\n    711928724,\n    3020668471,\n    3272380065,\n    1510334235,\n    755167117\n]);\nfunction getCrc32(buf) {\n    const b = new Uint8Array(buf);\n    let crc = -1;\n    for(let i8 = 0; i8 < b.length; i8++){\n        const byte = b[i8];\n        const t = (byte ^ crc) & 255;\n        crc = lookUpTable[t] ^ crc >>> 8;\n    }\n    return (crc ^ -1) >>> 0;\n}\n// node_modules/@dfinity/principal/lib/esm/utils/sha224.js\nvar import_js_sha256 = __toESM(require_sha256());\nfunction sha224(data) {\n    const shaObj = import_js_sha256.sha224.create();\n    shaObj.update(data);\n    return new Uint8Array(shaObj.array());\n}\n// node_modules/@dfinity/principal/lib/esm/index.js\nvar SELF_AUTHENTICATING_SUFFIX = 2;\nvar ANONYMOUS_SUFFIX = 4;\nvar MANAGEMENT_CANISTER_PRINCIPAL_HEX_STR = \"aaaaa-aa\";\nvar fromHexString = (hexString)=>{\n    var _a;\n    return new Uint8Array(((_a = hexString.match(/.{1,2}/g)) !== null && _a !== void 0 ? _a : []).map((byte)=>parseInt(byte, 16)\n    ));\n};\nvar toHexString = (bytes)=>bytes.reduce((str, byte)=>str + byte.toString(16).padStart(2, \"0\")\n    , \"\")\n;\nvar Principal = class {\n    static anonymous() {\n        return new this(new Uint8Array([\n            ANONYMOUS_SUFFIX\n        ]));\n    }\n    static managementCanister() {\n        return this.fromHex(MANAGEMENT_CANISTER_PRINCIPAL_HEX_STR);\n    }\n    static selfAuthenticating(publicKey) {\n        const sha = sha224(publicKey);\n        return new this(new Uint8Array([\n            ...sha,\n            SELF_AUTHENTICATING_SUFFIX\n        ]));\n    }\n    static from(other) {\n        if (typeof other === \"string\") {\n            return Principal.fromText(other);\n        } else if (typeof other === \"object\" && other !== null && other._isPrincipal === true) {\n            return new Principal(other._arr);\n        }\n        throw new Error(`Impossible to convert ${JSON.stringify(other)} to Principal.`);\n    }\n    static fromHex(hex) {\n        return new this(fromHexString(hex));\n    }\n    static fromText(text2) {\n        const canisterIdNoDash = text2.toLowerCase().replace(/-/g, \"\");\n        let arr = decode(canisterIdNoDash);\n        arr = arr.slice(4, arr.length);\n        const principal = new this(arr);\n        if (principal.toText() !== text2) {\n            throw new Error(`Principal \"${principal.toText()}\" does not have a valid checksum (original value \"${text2}\" may not be a valid Principal ID).`);\n        }\n        return principal;\n    }\n    static fromUint8Array(arr) {\n        return new this(arr);\n    }\n    isAnonymous() {\n        return this._arr.byteLength === 1 && this._arr[0] === ANONYMOUS_SUFFIX;\n    }\n    toUint8Array() {\n        return this._arr;\n    }\n    toHex() {\n        return toHexString(this._arr).toUpperCase();\n    }\n    toText() {\n        const checksumArrayBuf = new ArrayBuffer(4);\n        const view = new DataView(checksumArrayBuf);\n        view.setUint32(0, getCrc32(this._arr));\n        const checksum = new Uint8Array(checksumArrayBuf);\n        const bytes = Uint8Array.from(this._arr);\n        const array = new Uint8Array([\n            ...checksum,\n            ...bytes\n        ]);\n        const result = encode(array);\n        const matches = result.match(/.{1,5}/g);\n        if (!matches) {\n            throw new Error();\n        }\n        return matches.join(\"-\");\n    }\n    toString() {\n        return this.toText();\n    }\n    compareTo(other) {\n        for(let i9 = 0; i9 < Math.min(this._arr.length, other._arr.length); i9++){\n            if (this._arr[i9] < other._arr[i9]) return \"lt\";\n            else if (this._arr[i9] > other._arr[i9]) return \"gt\";\n        }\n        if (this._arr.length < other._arr.length) return \"lt\";\n        if (this._arr.length > other._arr.length) return \"gt\";\n        return \"eq\";\n    }\n    ltEq(other) {\n        const cmp = this.compareTo(other);\n        return cmp == \"lt\" || cmp == \"eq\";\n    }\n    gtEq(other) {\n        const cmp = this.compareTo(other);\n        return cmp == \"gt\" || cmp == \"eq\";\n    }\n    constructor(_arr){\n        this._arr = _arr;\n        this._isPrincipal = true;\n    }\n};\nexports.Principal = Principal;\nvar _ic;\n// node_modules/azle/src/lib/ic.ts\nvar ic = (_ic = globalThis.ic) !== null && _ic !== void 0 ? _ic : {};\n// node_modules/azle/node_modules/@dfinity/principal/lib/esm/utils/base32.js\nvar alphabet2 = \"abcdefghijklmnopqrstuvwxyz234567\";\nvar lookupTable2 = /* @__PURE__ */ Object.create(null);\nfor(let i1 = 0; i1 < alphabet2.length; i1++){\n    lookupTable2[alphabet2[i1]] = i1;\n}\nlookupTable2[\"0\"] = lookupTable2.o;\nlookupTable2[\"1\"] = lookupTable2.i;\n// node_modules/azle/node_modules/@dfinity/principal/lib/esm/utils/getCrc.js\nvar lookUpTable2 = new Uint32Array([\n    0,\n    1996959894,\n    3993919788,\n    2567524794,\n    124634137,\n    1886057615,\n    3915621685,\n    2657392035,\n    249268274,\n    2044508324,\n    3772115230,\n    2547177864,\n    162941995,\n    2125561021,\n    3887607047,\n    2428444049,\n    498536548,\n    1789927666,\n    4089016648,\n    2227061214,\n    450548861,\n    1843258603,\n    4107580753,\n    2211677639,\n    325883990,\n    1684777152,\n    4251122042,\n    2321926636,\n    335633487,\n    1661365465,\n    4195302755,\n    2366115317,\n    997073096,\n    1281953886,\n    3579855332,\n    2724688242,\n    1006888145,\n    1258607687,\n    3524101629,\n    2768942443,\n    901097722,\n    1119000684,\n    3686517206,\n    2898065728,\n    853044451,\n    1172266101,\n    3705015759,\n    2882616665,\n    651767980,\n    1373503546,\n    3369554304,\n    3218104598,\n    565507253,\n    1454621731,\n    3485111705,\n    3099436303,\n    671266974,\n    1594198024,\n    3322730930,\n    2970347812,\n    795835527,\n    1483230225,\n    3244367275,\n    3060149565,\n    1994146192,\n    31158534,\n    2563907772,\n    4023717930,\n    1907459465,\n    112637215,\n    2680153253,\n    3904427059,\n    2013776290,\n    251722036,\n    2517215374,\n    3775830040,\n    2137656763,\n    141376813,\n    2439277719,\n    3865271297,\n    1802195444,\n    476864866,\n    2238001368,\n    4066508878,\n    1812370925,\n    453092731,\n    2181625025,\n    4111451223,\n    1706088902,\n    314042704,\n    2344532202,\n    4240017532,\n    1658658271,\n    366619977,\n    2362670323,\n    4224994405,\n    1303535960,\n    984961486,\n    2747007092,\n    3569037538,\n    1256170817,\n    1037604311,\n    2765210733,\n    3554079995,\n    1131014506,\n    879679996,\n    2909243462,\n    3663771856,\n    1141124467,\n    855842277,\n    2852801631,\n    3708648649,\n    1342533948,\n    654459306,\n    3188396048,\n    3373015174,\n    1466479909,\n    544179635,\n    3110523913,\n    3462522015,\n    1591671054,\n    702138776,\n    2966460450,\n    3352799412,\n    1504918807,\n    783551873,\n    3082640443,\n    3233442989,\n    3988292384,\n    2596254646,\n    62317068,\n    1957810842,\n    3939845945,\n    2647816111,\n    81470997,\n    1943803523,\n    3814918930,\n    2489596804,\n    225274430,\n    2053790376,\n    3826175755,\n    2466906013,\n    167816743,\n    2097651377,\n    4027552580,\n    2265490386,\n    503444072,\n    1762050814,\n    4150417245,\n    2154129355,\n    426522225,\n    1852507879,\n    4275313526,\n    2312317920,\n    282753626,\n    1742555852,\n    4189708143,\n    2394877945,\n    397917763,\n    1622183637,\n    3604390888,\n    2714866558,\n    953729732,\n    1340076626,\n    3518719985,\n    2797360999,\n    1068828381,\n    1219638859,\n    3624741850,\n    2936675148,\n    906185462,\n    1090812512,\n    3747672003,\n    2825379669,\n    829329135,\n    1181335161,\n    3412177804,\n    3160834842,\n    628085408,\n    1382605366,\n    3423369109,\n    3138078467,\n    570562233,\n    1426400815,\n    3317316542,\n    2998733608,\n    733239954,\n    1555261956,\n    3268935591,\n    3050360625,\n    752459403,\n    1541320221,\n    2607071920,\n    3965973030,\n    1969922972,\n    40735498,\n    2617837225,\n    3943577151,\n    1913087877,\n    83908371,\n    2512341634,\n    3803740692,\n    2075208622,\n    213261112,\n    2463272603,\n    3855990285,\n    2094854071,\n    198958881,\n    2262029012,\n    4057260610,\n    1759359992,\n    534414190,\n    2176718541,\n    4139329115,\n    1873836001,\n    414664567,\n    2282248934,\n    4279200368,\n    1711684554,\n    285281116,\n    2405801727,\n    4167216745,\n    1634467795,\n    376229701,\n    2685067896,\n    3608007406,\n    1308918612,\n    956543938,\n    2808555105,\n    3495958263,\n    1231636301,\n    1047427035,\n    2932959818,\n    3654703836,\n    1088359270,\n    936918000,\n    2847714899,\n    3736837829,\n    1202900863,\n    817233897,\n    3183342108,\n    3401237130,\n    1404277552,\n    615818150,\n    3134207493,\n    3453421203,\n    1423857449,\n    601450431,\n    3009837614,\n    3294710456,\n    1567103746,\n    711928724,\n    3020668471,\n    3272380065,\n    1510334235,\n    755167117\n]);\n// node_modules/azle/node_modules/@dfinity/principal/lib/esm/utils/sha224.js\nvar import_js_sha2562 = __toESM(require_sha256());\n// node_modules/azle/src/lib/candid_types/index.ts\nvar Opt = {\n    Some: (value)=>({\n            Some: value\n        })\n    ,\n    None: Object.freeze({\n        None: null\n    })\n};\n// backend/index.ts\nvar counter = BigInt(0);\nfunction get() {\n    return counter;\n}\nexports.get = get;\nfunction add(n) {\n    counter += n;\n    return counter;\n}\nexports.add = add;\nfunction inc() {\n    counter += BigInt(1);\n    return;\n}\nexports.inc = inc;\n\n        ";
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
thread_local! {
    static _CDK_RNG_REF_CELL : std::cell::RefCell < rand::rngs::StdRng > =
    std::cell::RefCell::new(rand::SeedableRng::from_seed([0u8; 32]));
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
fn custom_getrandom(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    _CDK_RNG_REF_CELL
        .with(|rng_ref_cell| {
            let mut rng = rng_ref_cell.borrow_mut();
            rng.fill(_buf);
        });
    Ok(())
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
getrandom::register_custom_getrandom!(custom_getrandom);
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
fn rng_seed() {
    ic_cdk::spawn(async move {
        let result: ic_cdk::api::call::CallResult<(Vec<u8>,)> = ic_cdk::api::management_canister::main::raw_rand()
            .await;
        _CDK_RNG_REF_CELL
            .with(|rng_ref_cell| {
                let mut rng = rng_ref_cell.borrow_mut();
                match result {
                    Ok(randomness) => {
                        *rng = rand::SeedableRng::from_seed(
                            randomness.0[..].try_into().unwrap(),
                        );
                    }
                    Err(err) => panic!(err),
                };
            });
    });
}
#[cfg(all(target_arch = "wasm32", target_os = "wasi"))]
fn rng_seed() {
    ic_cdk::spawn(async move {
        let result: ic_cdk::api::call::CallResult<(Vec<u8>,)> = ic_cdk::api::management_canister::main::raw_rand()
            .await;
        match result {
            Ok(randomness) => unsafe { ic_wasi_polyfill::init_seed(&randomness.0) }
            Err(err) => panic!(err),
        };
    });
}
pub trait CdkActTryIntoVmValue<Context, VmValue> {
    fn try_into_vm_value(
        self,
        context: Context,
    ) -> Result<VmValue, CdkActTryIntoVmValueError>;
}
#[derive(Debug)]
pub struct CdkActTryIntoVmValueError(pub String);
impl From<&str> for CdkActTryIntoVmValueError {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
impl From<String> for CdkActTryIntoVmValueError {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl From<boa_engine::JsError> for CdkActTryIntoVmValueError {
    fn from(value: boa_engine::JsError) -> Self {
        Self(value.to_string())
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue> for () {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(boa_engine::JsValue::Null)
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue> for bool {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(self.into())
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue> for String {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(self.into())
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue>
for candid::Empty {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Err(
            CdkActTryIntoVmValueError(
                "Empty cannot be converted into JsValue".to_string(),
            ),
        )
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue>
for candid::Reserved {
    fn try_into_vm_value(
        self,
        _: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(boa_engine::JsValue::Null)
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue>
for candid::Func {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(
            boa_engine::object::builtins::JsArray::from_iter(
                    [self.principal.try_into_vm_value(context)?, self.method.into()],
                    context,
                )
                .into(),
        )
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue>
for candid::Principal {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        let exports_js_value = context.eval(boa_engine::Source::from_bytes("exports"))?;
        let exports_js_object = exports_js_value
            .as_object()
            .ok_or_else(|| "TypeError: 'exports' is not an object")?;
        let principal_class_js_value = exports_js_object.get("Principal", context)?;
        let principal_class_js_object = principal_class_js_value
            .as_object()
            .ok_or_else(|| "ReferenceError: Principal is not defined")?;
        let from_text_js_value = principal_class_js_object.get("fromText", context)?;
        let from_text_js_object = from_text_js_value
            .as_object()
            .ok_or_else(|| "TypeError: Principal.fromText is not a function")?;
        let principal_js_value = from_text_js_object
            .call(&principal_class_js_value, &[self.to_text().into()], context)?;
        Ok(principal_js_value)
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue>
for ic_cdk_timers::TimerId {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        let timer_id_as_u64 = self.data().as_ffi();
        Ok(boa_engine::JsValue::BigInt(timer_id_as_u64.into()))
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue>
for ic_cdk::api::stable::StableMemoryError {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        match self {
            ic_cdk::api::stable::StableMemoryError::OutOfMemory => {
                Ok(
                    boa_engine::object::ObjectInitializer::new(context)
                        .property(
                            "OutOfMemory",
                            boa_engine::JsValue::Null,
                            boa_engine::property::Attribute::all(),
                        )
                        .build()
                        .into(),
                )
            }
            ic_cdk::api::stable::StableMemoryError::OutOfBounds => {
                Ok(
                    boa_engine::object::ObjectInitializer::new(context)
                        .property(
                            "OutOfBounds",
                            boa_engine::JsValue::Null,
                            boa_engine::property::Attribute::all(),
                        )
                        .build()
                        .into(),
                )
            }
        }
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue>
for ic_stable_structures::btreemap::InsertError {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        match self {
            ic_stable_structures::btreemap::InsertError::KeyTooLarge { given, max } => {
                let given_js_value = given.try_into_vm_value(context)?;
                let max_js_value = max.try_into_vm_value(context)?;
                let key_too_large_object = boa_engine::object::ObjectInitializer::new(
                        context,
                    )
                    .property(
                        "given",
                        given_js_value,
                        boa_engine::property::Attribute::all(),
                    )
                    .property(
                        "max",
                        max_js_value,
                        boa_engine::property::Attribute::all(),
                    )
                    .build();
                Ok(
                    boa_engine::object::ObjectInitializer::new(context)
                        .property(
                            "KeyTooLarge",
                            key_too_large_object,
                            boa_engine::property::Attribute::all(),
                        )
                        .build()
                        .into(),
                )
            }
            ic_stable_structures::btreemap::InsertError::ValueTooLarge {
                given,
                max,
            } => {
                let given_js_value = given.try_into_vm_value(context)?;
                let max_js_value = max.try_into_vm_value(context)?;
                let value_too_large_object = boa_engine::object::ObjectInitializer::new(
                        context,
                    )
                    .property(
                        "given",
                        given_js_value,
                        boa_engine::property::Attribute::all(),
                    )
                    .property(
                        "max",
                        max_js_value,
                        boa_engine::property::Attribute::all(),
                    )
                    .build();
                Ok(
                    boa_engine::object::ObjectInitializer::new(context)
                        .property(
                            "ValueTooLarge",
                            value_too_large_object,
                            boa_engine::property::Attribute::all(),
                        )
                        .build()
                        .into(),
                )
            }
        }
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue>
for ic_cdk::api::call::RejectionCode {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        match self {
            ic_cdk::api::call::RejectionCode::NoError => {
                Ok(
                    boa_engine::object::ObjectInitializer::new(context)
                        .property(
                            "NoError",
                            boa_engine::JsValue::Null,
                            boa_engine::property::Attribute::all(),
                        )
                        .build()
                        .into(),
                )
            }
            ic_cdk::api::call::RejectionCode::SysFatal => {
                Ok(
                    boa_engine::object::ObjectInitializer::new(context)
                        .property(
                            "SysFatal",
                            boa_engine::JsValue::Null,
                            boa_engine::property::Attribute::all(),
                        )
                        .build()
                        .into(),
                )
            }
            ic_cdk::api::call::RejectionCode::SysTransient => {
                Ok(
                    boa_engine::object::ObjectInitializer::new(context)
                        .property(
                            "SysTransient",
                            boa_engine::JsValue::Null,
                            boa_engine::property::Attribute::all(),
                        )
                        .build()
                        .into(),
                )
            }
            ic_cdk::api::call::RejectionCode::DestinationInvalid => {
                Ok(
                    boa_engine::object::ObjectInitializer::new(context)
                        .property(
                            "DestinationInvalid",
                            boa_engine::JsValue::Null,
                            boa_engine::property::Attribute::all(),
                        )
                        .build()
                        .into(),
                )
            }
            ic_cdk::api::call::RejectionCode::CanisterReject => {
                Ok(
                    boa_engine::object::ObjectInitializer::new(context)
                        .property(
                            "CanisterReject",
                            boa_engine::JsValue::Null,
                            boa_engine::property::Attribute::all(),
                        )
                        .build()
                        .into(),
                )
            }
            ic_cdk::api::call::RejectionCode::CanisterError => {
                Ok(
                    boa_engine::object::ObjectInitializer::new(context)
                        .property(
                            "CanisterError",
                            boa_engine::JsValue::Null,
                            boa_engine::property::Attribute::all(),
                        )
                        .build()
                        .into(),
                )
            }
            ic_cdk::api::call::RejectionCode::Unknown => {
                Ok(
                    boa_engine::object::ObjectInitializer::new(context)
                        .property(
                            "Unknown",
                            boa_engine::JsValue::Null,
                            boa_engine::property::Attribute::all(),
                        )
                        .build()
                        .into(),
                )
            }
        }
    }
}
impl<T> CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue> for (T,)
where
    T: for<'a, 'b> CdkActTryIntoVmValue<
        &'a mut boa_engine::Context<'b>,
        boa_engine::JsValue,
    >,
{
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(self.0.try_into_vm_value(context)?)
    }
}
impl<T> CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue>
for Box<T>
where
    T: for<'a, 'b> CdkActTryIntoVmValue<
        &'a mut boa_engine::Context<'b>,
        boa_engine::JsValue,
    >,
{
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok((*self).try_into_vm_value(context)?)
    }
}
impl<T> CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue>
for Option<T>
where
    T: for<'a, 'b> CdkActTryIntoVmValue<
        &'a mut boa_engine::Context<'b>,
        boa_engine::JsValue,
    >,
{
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        match self {
            Some(value) => {
                let some_js_value = value.try_into_vm_value(context)?;
                Ok(
                    boa_engine::object::ObjectInitializer::new(context)
                        .property(
                            "Some",
                            some_js_value,
                            boa_engine::property::Attribute::all(),
                        )
                        .build()
                        .into(),
                )
            }
            None => {
                Ok(
                    boa_engine::object::ObjectInitializer::new(context)
                        .property(
                            "None",
                            boa_engine::JsValue::Null,
                            boa_engine::property::Attribute::all(),
                        )
                        .build()
                        .into(),
                )
            }
        }
    }
}
impl<T, K> CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue>
for Result<T, K>
where
    T: for<'a, 'b> CdkActTryIntoVmValue<
        &'a mut boa_engine::Context<'b>,
        boa_engine::JsValue,
    >,
    K: for<'a, 'b> CdkActTryIntoVmValue<
        &'a mut boa_engine::Context<'b>,
        boa_engine::JsValue,
    >,
{
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        match self {
            Ok(ok) => {
                let ok_js_value = ok.try_into_vm_value(context)?;
                let result_js_object = boa_engine::object::ObjectInitializer::new(
                        context,
                    )
                    .property("Ok", ok_js_value, boa_engine::property::Attribute::all())
                    .build();
                let result_js_value = result_js_object.into();
                Ok(result_js_value)
            }
            Err(err) => {
                let err_js_value = err.try_into_vm_value(context)?;
                let result_js_object = boa_engine::object::ObjectInitializer::new(
                        context,
                    )
                    .property(
                        "Err",
                        err_js_value,
                        boa_engine::property::Attribute::all(),
                    )
                    .build();
                let result_js_value = result_js_object.into();
                Ok(result_js_value)
            }
        }
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue> for f64 {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(self.into())
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue>
for _CdkFloat64 {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(self.0.into())
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue> for f32 {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(self.into())
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue>
for _CdkFloat32 {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(self.0.into())
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue>
for candid::Int {
    fn try_into_vm_value(
        self,
        _: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(boa_engine::JsValue::BigInt(boa_engine::bigint::JsBigInt::new(self.0)))
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue> for i128 {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(
            boa_engine::bigint::JsBigInt::new(boa_engine::bigint::RawBigInt::from(self))
                .into(),
        )
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue> for i64 {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(boa_engine::JsValue::BigInt(self.into()))
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue> for i32 {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(self.into())
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue> for i16 {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(self.into())
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue> for i8 {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(self.into())
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue>
for candid::Nat {
    fn try_into_vm_value(
        self,
        _: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(
            boa_engine::JsValue::BigInt(
                boa_engine::bigint::JsBigInt::from_string(&self.0.to_string())
                    .ok_or_else(|| "TypeError: Value is not of type 'bigint'")?,
            ),
        )
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue> for u128 {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(
            boa_engine::bigint::JsBigInt::new(boa_engine::bigint::RawBigInt::from(self))
                .into(),
        )
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue> for u64 {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(boa_engine::JsValue::BigInt(self.into()))
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue> for usize {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(self.into())
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue> for u32 {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(self.into())
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue> for u16 {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(self.into())
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue> for u8 {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(self.into())
    }
}
trait AzleTryIntoVec {}
impl AzleTryIntoVec for () {}
impl AzleTryIntoVec for bool {}
impl AzleTryIntoVec for String {}
impl AzleTryIntoVec for candid::Empty {}
impl AzleTryIntoVec for candid::Reserved {}
impl AzleTryIntoVec for candid::Func {}
impl AzleTryIntoVec for candid::Principal {}
impl AzleTryIntoVec for ic_cdk_timers::TimerId {}
impl AzleTryIntoVec for ic_cdk::api::call::RejectionCode {}
impl AzleTryIntoVec for f64 {}
impl AzleTryIntoVec for _CdkFloat64 {}
impl AzleTryIntoVec for f32 {}
impl AzleTryIntoVec for _CdkFloat32 {}
impl AzleTryIntoVec for candid::Int {}
impl AzleTryIntoVec for i128 {}
impl AzleTryIntoVec for i64 {}
impl AzleTryIntoVec for i32 {}
impl AzleTryIntoVec for i16 {}
impl AzleTryIntoVec for i8 {}
impl AzleTryIntoVec for candid::Nat {}
impl AzleTryIntoVec for u128 {}
impl AzleTryIntoVec for u64 {}
impl AzleTryIntoVec for usize {}
impl AzleTryIntoVec for u32 {}
impl AzleTryIntoVec for u16 {}
impl<T> AzleTryIntoVec for Option<T> {}
impl<T> AzleTryIntoVec for Vec<T> {}
impl<T> CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue>
for Vec<T>
where
    T: AzleTryIntoVec,
    T: for<'a, 'b> CdkActTryIntoVmValue<
        &'a mut boa_engine::Context<'b>,
        boa_engine::JsValue,
    >,
{
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        try_into_vm_value_generic_array(self, context)
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context<'_>, boa_engine::JsValue>
for Vec<u8> {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        let js_array_buffer = boa_engine::object::builtins::JsArrayBuffer::from_byte_block(
            self,
            context,
        )?;
        let js_uint8_array = boa_engine::object::builtins::JsUint8Array::from_array_buffer(
            js_array_buffer,
            context,
        )?;
        Ok(js_uint8_array.into())
    }
}
fn try_into_vm_value_generic_array<T>(
    generic_array: Vec<T>,
    context: &mut boa_engine::Context,
) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError>
where
    T: for<'a, 'b> CdkActTryIntoVmValue<
        &'a mut boa_engine::Context<'b>,
        boa_engine::JsValue,
    >,
{
    let js_values: Vec<_> = generic_array
        .into_iter()
        .map(|item| item.try_into_vm_value(context))
        .collect::<Result<_, _>>()?;
    Ok(boa_engine::object::builtins::JsArray::from_iter(js_values, context).into())
}
pub trait CdkActTryFromVmValue<Ok, Err, Context> {
    fn try_from_vm_value(self, context: Context) -> Result<Ok, Err>;
}
impl CdkActTryFromVmValue<(), boa_engine::JsError, &mut boa_engine::Context<'_>>
for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<(), boa_engine::JsError> {
        if self.is_null() || self.is_undefined() {
            return Ok(());
        }
        Err("TypeError: Value is not of type 'null' or 'undefined'".to_js_error(None))?
    }
}
impl CdkActTryFromVmValue<bool, boa_engine::JsError, &mut boa_engine::Context<'_>>
for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<bool, boa_engine::JsError> {
        Ok(
            self
                .as_boolean()
                .ok_or_else(|| {
                    "TypeError: Value is not of type 'boolean'".to_js_error(None)
                })?,
        )
    }
}
impl CdkActTryFromVmValue<String, boa_engine::JsError, &mut boa_engine::Context<'_>>
for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<String, boa_engine::JsError> {
        let error_message = "TypeError: Value is not of type 'string'";
        Ok(
            self
                .as_string()
                .ok_or_else(|| error_message.to_js_error(None))?
                .to_std_string()
                .map_err(|err| {
                    let cause = err.to_string().to_js_error(None);
                    error_message.to_js_error(Some(cause))
                })?,
        )
    }
}
impl CdkActTryFromVmValue<
    candid::Empty,
    boa_engine::JsError,
    &mut boa_engine::Context<'_>,
> for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<candid::Empty, boa_engine::JsError> {
        Err("TypeError: Value cannot be converted into type 'empty'".to_js_error(None))?
    }
}
impl CdkActTryFromVmValue<
    candid::Reserved,
    boa_engine::JsError,
    &mut boa_engine::Context<'_>,
> for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<candid::Reserved, boa_engine::JsError> {
        Ok(candid::Reserved)
    }
}
impl CdkActTryFromVmValue<
    candid::Func,
    boa_engine::JsError,
    &mut boa_engine::Context<'_>,
> for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<candid::Func, boa_engine::JsError> {
        let error_message = "TypeError: Value is not of type 'Func'";
        let js_object = self.as_object().ok_or_else(|| error_message.to_js_error(None))?;
        if !js_object.is_array() {
            let cause = "TypeError: Expected 'Array', given 'Object'".to_js_error(None);
            return Err(error_message.to_js_error(Some(cause)));
        }
        let index0 = js_object.get("0", context)?;
        if index0.is_undefined() {
            let cause = "TypeError: Index '0' is undefined".to_js_error(None);
            return Err(error_message.to_js_error(Some(cause)));
        }
        let principal = index0
            .try_from_vm_value(&mut *context)
            .map_err(|principal_err| {
                let cause = "TypeError: Index '0' is not of type 'Principal'"
                    .to_js_error(Some(principal_err));
                error_message.to_js_error(Some(cause))
            })?;
        let index1 = js_object.get("1", context)?;
        if index1.is_undefined() {
            let cause = "TypeError: Index '1' is undefined".to_js_error(None);
            return Err(error_message.to_js_error(Some(cause)));
        }
        let method = index1
            .try_from_vm_value(&mut *context)
            .map_err(|str_err| {
                let cause = "TypeError: Index '1' is not of type 'string'"
                    .to_js_error(Some(str_err));
                error_message.to_js_error(Some(cause))
            })?;
        Ok(candid::Func { principal, method })
    }
}
impl CdkActTryFromVmValue<
    candid::Principal,
    boa_engine::JsError,
    &mut boa_engine::Context<'_>,
> for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<candid::Principal, boa_engine::JsError> {
        let error_message = "TypeError: Value is not of type 'Principal'";
        let principal_js_object = self
            .as_object()
            .ok_or_else(|| error_message.to_js_error(None))?;
        let principal_to_text_function_js_value = principal_js_object
            .get("toText", context)
            .map_err(|err| {
                let cause = "TypeError: Property 'toText' of object is not a function"
                    .to_js_error(None);
                error_message.to_js_error(Some(cause))
            })?;
        let principal_to_text_function_js_object = principal_to_text_function_js_value
            .as_object()
            .ok_or_else(|| {
                let cause = "TypeError: Property 'toText' of object is not a function"
                    .to_js_error(None);
                error_message.to_js_error(Some(cause))
            })?;
        let principal_text = principal_to_text_function_js_object
            .call(&self, &[], context)
            .map_err(|js_err| error_message.to_js_error(Some(js_err)))?
            .as_string()
            .ok_or_else(|| {
                let cause = "TypeError: Return value of method 'toText' is not of type 'string'"
                    .to_js_error(None);
                error_message.to_js_error(Some(cause))
            })?
            .to_std_string()
            .map_err(|err| {
                let cause = err.to_string().to_js_error(None);
                error_message.to_js_error(Some(cause))
            })?;
        let principal = candid::Principal::from_text(principal_text)
            .map_err(|principal_error| {
                let inner_error_name = match principal_error {
                    candid::types::principal::PrincipalError::BytesTooLong() => {
                        "BytesTooLongError"
                    }
                    candid::types::principal::PrincipalError::InvalidBase32() => {
                        "InvalidBase32Error"
                    }
                    candid::types::principal::PrincipalError::TextTooShort() => {
                        "TextTooShortError"
                    }
                    candid::types::principal::PrincipalError::TextTooLong() => {
                        "TextTooLongError"
                    }
                    candid::types::principal::PrincipalError::CheckSequenceNotMatch() => {
                        "CheckSequenceNotMatchError"
                    }
                    candid::types::principal::PrincipalError::AbnormalGrouped(_) => {
                        "AbnormalGroupedError"
                    }
                };
                let inner_error_message = principal_error.to_string();
                let inner_error = format!("{inner_error_name}: {inner_error_message}")
                    .to_js_error(None);
                error_message.to_js_error(Some(inner_error))
            })?;
        Ok(principal)
    }
}
impl CdkActTryFromVmValue<
    ic_cdk_timers::TimerId,
    boa_engine::JsError,
    &mut boa_engine::Context<'_>,
> for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<ic_cdk_timers::TimerId, boa_engine::JsError> {
        Ok(
            ic_cdk_timers::TimerId::from(
                slotmap::KeyData::from_ffi(self.try_from_vm_value(context)?),
            ),
        )
    }
}
impl<T> CdkActTryFromVmValue<(T,), boa_engine::JsError, &mut boa_engine::Context<'_>>
for boa_engine::JsValue
where
    boa_engine::JsValue: for<'a, 'b> CdkActTryFromVmValue<
        T,
        boa_engine::JsError,
        &'a mut boa_engine::Context<'b>,
    >,
{
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<(T,), boa_engine::JsError> {
        Ok((self.try_from_vm_value(context)?,))
    }
}
impl<T> CdkActTryFromVmValue<Box<T>, boa_engine::JsError, &mut boa_engine::Context<'_>>
for boa_engine::JsValue
where
    boa_engine::JsValue: for<'a, 'b> CdkActTryFromVmValue<
        T,
        boa_engine::JsError,
        &'a mut boa_engine::Context<'b>,
    >,
{
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<Box<T>, boa_engine::JsError> {
        Ok(Box::new(self.try_from_vm_value(context)?))
    }
}
impl<
    T,
> CdkActTryFromVmValue<Option<T>, boa_engine::JsError, &mut boa_engine::Context<'_>>
for boa_engine::JsValue
where
    boa_engine::JsValue: for<'a, 'b> CdkActTryFromVmValue<
        T,
        boa_engine::JsError,
        &'a mut boa_engine::Context<'b>,
    >,
{
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<Option<T>, boa_engine::JsError> {
        let error_message = "TypeError: Value is not of type 'Opt'";
        let js_object = self.as_object().ok_or_else(|| error_message.to_js_error(None))?;
        let has_none_property = js_object.has_own_property("None", context)?;
        let has_some_property = js_object.has_own_property("Some", context)?;
        if has_none_property && has_some_property {
            let cause = "TypeError: Value must contain exactly one of the \
                        following properties: ['Some', 'None']"
                .to_js_error(None);
            return Err(error_message.to_js_error(Some(cause)))?;
        }
        if has_none_property {
            let none_value = js_object
                .get("None", context)
                .map_err(|err| {
                    let cause = err.to_string().to_js_error(None);
                    error_message.to_js_error(Some(cause))
                })?;
            return if none_value.is_null() {
                Ok(None)
            } else {
                let cause = "TypeError: Value is not of type 'null'".to_js_error(None);
                Err(error_message.to_js_error(Some(cause)))
            };
        }
        if has_some_property {
            return Ok(
                Some(
                    js_object
                        .get("Some", context)
                        .map_err(|err| {
                            let cause = err.to_string().to_js_error(None);
                            error_message.to_js_error(Some(cause))
                        })?
                        .try_from_vm_value(context)
                        .map_err(|js_error| error_message.to_js_error(Some(js_error)))?,
                ),
            );
        }
        let cause = "TypeError: Value must contain exactly one of the \
                    following properties: ['Some', 'None']"
            .to_js_error(None);
        Err(error_message.to_js_error(Some(cause)))?
    }
}
impl CdkActTryFromVmValue<f64, boa_engine::JsError, &mut boa_engine::Context<'_>>
for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<f64, boa_engine::JsError> {
        Ok(
            self
                .as_number()
                .ok_or_else(|| {
                    "TypeError: Value is not of type 'float64'".to_js_error(None)
                })?,
        )
    }
}
impl CdkActTryFromVmValue<_CdkFloat64, boa_engine::JsError, &mut boa_engine::Context<'_>>
for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<_CdkFloat64, boa_engine::JsError> {
        Ok(
            _CdkFloat64(
                self
                    .as_number()
                    .ok_or_else(|| {
                        "TypeError: Value is not of type 'float64'".to_js_error(None)
                    })?,
            ),
        )
    }
}
impl CdkActTryFromVmValue<f32, boa_engine::JsError, &mut boa_engine::Context<'_>>
for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<f32, boa_engine::JsError> {
        Ok(
            self
                .as_number()
                .ok_or_else(|| {
                    "TypeError: Value is not of type 'float32'".to_js_error(None)
                })? as f32,
        )
    }
}
impl CdkActTryFromVmValue<_CdkFloat32, boa_engine::JsError, &mut boa_engine::Context<'_>>
for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<_CdkFloat32, boa_engine::JsError> {
        Ok(
            _CdkFloat32(
                self
                    .as_number()
                    .ok_or_else(|| {
                        "TypeError: Value is not of type 'float32'".to_js_error(None)
                    })? as f32,
            ),
        )
    }
}
impl CdkActTryFromVmValue<candid::Int, boa_engine::JsError, &mut boa_engine::Context<'_>>
for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<candid::Int, boa_engine::JsError> {
        let error_message = "TypeError: Value is not of type 'int'";
        let int_string = self
            .as_bigint()
            .ok_or_else(|| error_message.to_js_error(None))?
            .to_string();
        Ok(
            candid::Int::from_str(&int_string)
                .map_err(|err| {
                    let cause = err.to_string().to_js_error(None);
                    error_message.to_js_error(Some(cause))
                })?,
        )
    }
}
impl CdkActTryFromVmValue<i128, boa_engine::JsError, &mut boa_engine::Context<'_>>
for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<i128, boa_engine::JsError> {
        let error_message = "TypeError: Value is not of type 'int'";
        Ok(
            self
                .as_bigint()
                .ok_or_else(|| error_message.to_js_error(None))?
                .to_string()
                .parse::<i128>()
                .map_err(|err| {
                    let cause = err.to_string().to_js_error(None);
                    error_message.to_js_error(Some(cause))
                })?,
        )
    }
}
impl CdkActTryFromVmValue<i64, boa_engine::JsError, &mut boa_engine::Context<'_>>
for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<i64, boa_engine::JsError> {
        let error_message = "TypeError: Value is not of type 'int64'";
        Ok(
            self
                .as_bigint()
                .ok_or_else(|| error_message.to_js_error(None))?
                .to_string()
                .parse::<i64>()
                .map_err(|err| {
                    let cause = err.to_string().to_js_error(None);
                    error_message.to_js_error(Some(cause))
                })?,
        )
    }
}
impl CdkActTryFromVmValue<i32, boa_engine::JsError, &mut boa_engine::Context<'_>>
for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<i32, boa_engine::JsError> {
        Ok(
            self
                .as_number()
                .ok_or_else(|| {
                    "TypeError: Value is not of type 'int32'".to_js_error(None)
                })? as i32,
        )
    }
}
impl CdkActTryFromVmValue<i16, boa_engine::JsError, &mut boa_engine::Context<'_>>
for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<i16, boa_engine::JsError> {
        Ok(
            self
                .as_number()
                .ok_or_else(|| {
                    "TypeError: Value is not of type 'int16'".to_js_error(None)
                })? as i16,
        )
    }
}
impl CdkActTryFromVmValue<i8, boa_engine::JsError, &mut boa_engine::Context<'_>>
for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<i8, boa_engine::JsError> {
        Ok(
            self
                .as_number()
                .ok_or_else(|| {
                    "TypeError: Value is not of type 'int8'".to_js_error(None)
                })? as i8,
        )
    }
}
impl CdkActTryFromVmValue<candid::Nat, boa_engine::JsError, &mut boa_engine::Context<'_>>
for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<candid::Nat, boa_engine::JsError> {
        let error_message = "TypeError: Value is not of type 'nat'";
        let bigint_string = self
            .as_bigint()
            .ok_or_else(|| error_message.to_js_error(None))?
            .to_string();
        Ok(
            candid::Nat::from_str(&bigint_string)
                .map_err(|err| {
                    let cause = err.to_string().to_js_error(None);
                    error_message.to_js_error(Some(cause))
                })?,
        )
    }
}
impl CdkActTryFromVmValue<u128, boa_engine::JsError, &mut boa_engine::Context<'_>>
for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<u128, boa_engine::JsError> {
        let error_message = "TypeError: Value is not of type 'nat'";
        Ok(
            self
                .as_bigint()
                .ok_or_else(|| error_message.to_js_error(None))?
                .to_string()
                .parse::<u128>()
                .map_err(|err| {
                    let cause = err.to_string().to_js_error(None);
                    error_message.to_js_error(Some(cause))
                })?,
        )
    }
}
impl CdkActTryFromVmValue<u64, boa_engine::JsError, &mut boa_engine::Context<'_>>
for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<u64, boa_engine::JsError> {
        let error_message = "TypeError: Value is not of type 'nat64'";
        Ok(
            self
                .as_bigint()
                .ok_or_else(|| error_message.to_js_error(None))?
                .to_string()
                .parse::<u64>()
                .map_err(|err| {
                    let cause = err.to_string().to_js_error(None);
                    error_message.to_js_error(Some(cause))
                })?,
        )
    }
}
impl CdkActTryFromVmValue<u32, boa_engine::JsError, &mut boa_engine::Context<'_>>
for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<u32, boa_engine::JsError> {
        Ok(
            self
                .as_number()
                .ok_or_else(|| {
                    "TypeError: Value is not of type 'nat32'".to_js_error(None)
                })? as u32,
        )
    }
}
impl CdkActTryFromVmValue<u16, boa_engine::JsError, &mut boa_engine::Context<'_>>
for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<u16, boa_engine::JsError> {
        Ok(
            self
                .as_number()
                .ok_or_else(|| {
                    "TypeError: Value is not of type 'nat16'".to_js_error(None)
                })? as u16,
        )
    }
}
impl CdkActTryFromVmValue<u8, boa_engine::JsError, &mut boa_engine::Context<'_>>
for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<u8, boa_engine::JsError> {
        Ok(
            self
                .as_number()
                .ok_or_else(|| {
                    "TypeError: Value is not of type 'nat8'".to_js_error(None)
                })? as u8,
        )
    }
}
impl CdkActTryFromVmValue<
    Result<(), String>,
    boa_engine::JsError,
    &mut boa_engine::Context<'_>,
> for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<Result<(), String>, boa_engine::JsError> {
        let error_message = "TypeError: Value is not of type 'GuardResult'";
        let js_object = self.as_object().ok_or_else(|| error_message.to_js_error(None))?;
        let has_ok_property = js_object
            .has_own_property("Ok", context)
            .map_err(|err| {
                let cause = err.to_string().to_js_error(None);
                error_message.to_js_error(Some(cause))
            })?;
        let has_err_property = js_object
            .has_own_property("Err", context)
            .map_err(|err| {
                let cause = err.to_string().to_js_error(None);
                error_message.to_js_error(Some(cause))
            })?;
        if has_ok_property && has_err_property {
            let cause = "TypeError: TypeError: Value must contain exactly one of the \
                        following properties: ['Ok', 'Err']"
                .to_js_error(None);
            return Err(error_message.to_js_error(Some(cause)));
        }
        if has_ok_property {
            let ok_value = js_object
                .get("Ok", context)
                .map_err(|err| {
                    let cause = err.to_string().to_js_error(None);
                    error_message.to_js_error(Some(cause))
                })?;
            return if ok_value.is_null() {
                Ok(Ok(()))
            } else {
                let cause = "TypeError: Value is not of type 'null'".to_js_error(None);
                Err(error_message.to_js_error(Some(cause)))
            };
        }
        if has_err_property {
            return Ok(
                Err(
                    js_object
                        .get("Err", context)
                        .map_err(|err| {
                            let cause = err.to_string().to_js_error(None);
                            error_message.to_js_error(Some(cause))
                        })?
                        .try_from_vm_value(context)?,
                ),
            );
        }
        let cause = "TypeError: Value must contain exactly one of the \
                    following properties: ['Ok', 'Err']"
            .to_js_error(None);
        Err(error_message.to_js_error(Some(cause)))
    }
}
trait AzleTryFromVec {}
impl AzleTryFromVec for () {}
impl AzleTryFromVec for bool {}
impl AzleTryFromVec for String {}
impl AzleTryFromVec for candid::Empty {}
impl AzleTryFromVec for candid::Reserved {}
impl AzleTryFromVec for candid::Func {}
impl AzleTryFromVec for candid::Principal {}
impl AzleTryFromVec for ic_cdk_timers::TimerId {}
impl AzleTryFromVec for f64 {}
impl AzleTryFromVec for _CdkFloat64 {}
impl AzleTryFromVec for f32 {}
impl AzleTryFromVec for _CdkFloat32 {}
impl AzleTryFromVec for candid::Int {}
impl AzleTryFromVec for i128 {}
impl AzleTryFromVec for i64 {}
impl AzleTryFromVec for i32 {}
impl AzleTryFromVec for i16 {}
impl AzleTryFromVec for i8 {}
impl AzleTryFromVec for candid::Nat {}
impl AzleTryFromVec for u128 {}
impl AzleTryFromVec for u64 {}
impl AzleTryFromVec for u32 {}
impl AzleTryFromVec for u16 {}
impl<T> AzleTryFromVec for Option<T> {}
impl<T> AzleTryFromVec for Vec<T> {}
impl<T> CdkActTryFromVmValue<Vec<T>, boa_engine::JsError, &mut boa_engine::Context<'_>>
for boa_engine::JsValue
where
    T: AzleTryFromVec,
    boa_engine::JsValue: for<'a, 'b> CdkActTryFromVmValue<
        T,
        boa_engine::JsError,
        &'a mut boa_engine::Context<'b>,
    >,
{
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<Vec<T>, boa_engine::JsError> {
        try_from_vm_value_generic_array::<T>(self, context)
    }
}
impl CdkActTryFromVmValue<Vec<u8>, boa_engine::JsError, &mut boa_engine::Context<'_>>
for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<Vec<u8>, boa_engine::JsError> {
        let error_message = "TypeError: Value is not of type 'blob'";
        Ok(
            self
                .as_object()
                .ok_or_else(|| error_message.to_js_error(None))?
                .borrow()
                .as_typed_array()
                .ok_or_else(|| {
                    let cause = "TypeError: Value is not an instance of 'TypedArray'"
                        .to_js_error(None);
                    error_message.to_js_error(Some(cause))
                })?
                .viewed_array_buffer()
                .ok_or_else(|| {
                    let cause = "TypedArray does not have an associated DataView"
                        .to_js_error(None);
                    error_message.to_js_error(Some(cause))
                })?
                .borrow()
                .as_array_buffer()
                .ok_or_else(|| {
                    let cause = "TypedArray does not have an associated ArrayBuffer"
                        .to_js_error(None);
                    error_message.to_js_error(Some(cause))
                })?
                .array_buffer_data
                .clone()
                .ok_or_else(|| {
                    let cause = "No data in ArrayBuffer".to_js_error(None);
                    error_message.to_js_error(Some(cause))
                })?,
        )
    }
}
fn try_from_vm_value_generic_array<T>(
    js_value: boa_engine::JsValue,
    context: &mut boa_engine::Context,
) -> Result<Vec<T>, boa_engine::JsError>
where
    boa_engine::JsValue: for<'a, 'b> CdkActTryFromVmValue<
        T,
        boa_engine::JsError,
        &'a mut boa_engine::Context<'b>,
    >,
{
    let error_message = "TypeError: Value is not of type 'Vec'";
    let js_object = js_value.as_object().ok_or_else(|| error_message.to_js_error(None))?;
    if !js_object.is_array() {
        return Err(error_message.to_js_error(None));
    }
    let mut index: usize = 0;
    let mut result = vec![];
    loop {
        let js_value = js_object
            .get(index, context)
            .map_err(|err| error_message.to_js_error(Some(err)))?;
        if js_value.is_undefined() {
            break;
        }
        result
            .push(
                js_value
                    .try_from_vm_value(context)
                    .map_err(|err| error_message.to_js_error(Some(err)))?,
            );
        index += 1;
    }
    Ok(result)
}
enum RuntimeError {
    IntoVmValueError(String),
    JsError(boa_engine::JsError),
    ReferenceError(String),
    TypeError(String),
    String(String),
}
impl RuntimeError {
    pub fn to_string(&self) -> String {
        match self {
            Self::IntoVmValueError(msg) => msg.to_string(),
            Self::JsError(js_error) => {
                BOA_CONTEXT_REF_CELL
                    .with(|boa_context_ref_cell| {
                        let mut boa_context = boa_context_ref_cell.borrow_mut();
                        js_error.clone().to_std_string(&mut boa_context)
                    })
            }
            Self::ReferenceError(msg) => format!("ReferenceError: {msg}"),
            Self::TypeError(msg) => format!("TypeError: {msg}"),
            Self::String(msg) => msg.clone(),
        }
    }
}
impl From<boa_engine::JsError> for RuntimeError {
    fn from(value: boa_engine::JsError) -> Self {
        Self::JsError(value)
    }
}
impl From<CdkActTryIntoVmValueError> for RuntimeError {
    fn from(value: CdkActTryIntoVmValueError) -> Self {
        Self::IntoVmValueError(value.0)
    }
}
impl From<String> for RuntimeError {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
trait ToJsError {
    fn to_js_error(self, opt_cause: Option<boa_engine::JsError>) -> boa_engine::JsError;
}
impl ToJsError for CdkActTryIntoVmValueError {
    fn to_js_error(self, opt_cause: Option<boa_engine::JsError>) -> boa_engine::JsError {
        self.0.to_js_error(opt_cause)
    }
}
impl ToJsError for String {
    fn to_js_error(self, opt_cause: Option<boa_engine::JsError>) -> boa_engine::JsError {
        self.as_str().to_js_error(opt_cause)
    }
}
impl<'a> ToJsError for &'a str {
    fn to_js_error(self, opt_cause: Option<boa_engine::JsError>) -> boa_engine::JsError {
        let raw_error_message = self;
        let error_types = [
            "Error: ",
            "EvalError: ",
            "RangeError: ",
            "ReferenceError: ",
            "SyntaxError: ",
            "TypeError: ",
            "UriError: ",
        ];
        for error_type in error_types.iter() {
            if raw_error_message.starts_with(error_type) {
                let message = raw_error_message
                    .splitn(2, error_type)
                    .collect::<Vec<&str>>()[1];
                let js_native_error = match *error_type {
                    "Error: " => boa_engine::error::JsNativeError::error(),
                    "EvalError: " => boa_engine::error::JsNativeError::eval(),
                    "RangeError: " => boa_engine::error::JsNativeError::range(),
                    "ReferenceError: " => boa_engine::error::JsNativeError::reference(),
                    "SyntaxError: " => boa_engine::error::JsNativeError::syntax(),
                    "TypeError: " => boa_engine::error::JsNativeError::typ(),
                    "UriError: " => boa_engine::error::JsNativeError::uri(),
                    _ => unreachable!(),
                }
                    .with_message(message);
                return match opt_cause {
                    Some(cause) => js_native_error.with_cause(cause),
                    None => js_native_error,
                }
                    .into();
            }
        }
        return boa_engine::error::JsNativeError::error()
            .with_message(raw_error_message)
            .into();
    }
}
fn async_await_result_handler(
    boa_context: &mut boa_engine::Context,
    boa_return_value: &boa_engine::JsValue,
    uuid: &str,
    method_name: &str,
    manual: bool,
) -> Result<boa_engine::JsValue, String> {
    let boa_return_value_object = match boa_return_value.as_object() {
        Some(object) => object,
        None => return Ok(boa_return_value.clone()),
    };
    if !boa_return_value_object.is_promise() {
        return Ok(boa_return_value.clone());
    }
    boa_context.run_jobs();
    let js_promise = boa_engine::object::builtins::JsPromise::from_object(
            boa_return_value_object.clone(),
        )
        .map_err(|js_error| js_error.to_std_string(&mut *boa_context))?;
    let state = js_promise
        .state()
        .map_err(|js_error| js_error.to_std_string(&mut *boa_context))?;
    return match &state {
        boa_engine::builtins::promise::PromiseState::Fulfilled(js_value) => {
            PROMISE_MAP_REF_CELL
                .with(|promise_map_ref_cell| {
                    let mut promise_map = promise_map_ref_cell.borrow_mut();
                    promise_map.remove(uuid);
                });
            if manual == true {
                return Ok(boa_return_value.clone());
            }
            match method_name {
                "_AZLE_TIMER" => {}
                _ => {
                    return Err(
                        format!(
                            "\nUncaught ReferenceError: {} is not defined", method_name
                        ),
                    );
                }
            };
            return Ok(boa_return_value.clone());
        }
        boa_engine::builtins::promise::PromiseState::Rejected(js_value) => {
            PROMISE_MAP_REF_CELL
                .with(|promise_map_ref_cell| {
                    let mut promise_map = promise_map_ref_cell.borrow_mut();
                    promise_map.remove(uuid);
                });
            return Err(js_value.clone().to_std_string(&mut *boa_context));
        }
        boa_engine::builtins::promise::PromiseState::Pending => {
            PROMISE_MAP_REF_CELL
                .with(|promise_map_ref_cell| {
                    let mut promise_map = promise_map_ref_cell.borrow_mut();
                    promise_map.insert(uuid.to_string(), boa_return_value.clone());
                });
            return Ok(boa_return_value.clone());
        }
    };
}
fn accept_message(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    ic_cdk::api::call::accept_message()
        .try_into_vm_value(context)
        .map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn arg_data_raw(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    ic_cdk::api::call::arg_data_raw()
        .try_into_vm_value(context)
        .map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn arg_data_raw_size(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    ic_cdk::api::call::arg_data_raw_size()
        .try_into_vm_value(context)
        .map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn call_raw(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let (js_promise, js_promise_resolvers) = boa_engine::object::builtins::JsPromise::new_pending(
        context,
    );
    let canister_id_js_value = aargs
        .get(0)
        .ok_or_else(|| {
            "An argument for 'canisterId' was not provided".to_js_error(None)
        })?
        .clone();
    let method_js_value = aargs
        .get(1)
        .ok_or_else(|| "An argument for 'method' was not provided".to_js_error(None))?
        .clone();
    let args_raw_js_value = aargs
        .get(2)
        .ok_or_else(|| "An argument for 'argsRaw' was not provided".to_js_error(None))?
        .clone();
    let payment_js_value = aargs
        .get(3)
        .ok_or_else(|| "An argument for 'payment' was not provided".to_js_error(None))?
        .clone();
    let canister_id: candid::Principal = canister_id_js_value
        .try_from_vm_value(&mut *context)?;
    let method: String = method_js_value.try_from_vm_value(&mut *context)?;
    let args_raw: Vec<u8> = args_raw_js_value.try_from_vm_value(&mut *context)?;
    let payment: u64 = payment_js_value.try_from_vm_value(&mut *context)?;
    ic_cdk::spawn(async move {
        let uuid = UUID_REF_CELL.with(|uuid_ref_cell| uuid_ref_cell.borrow().clone());
        let method_name = METHOD_NAME_REF_CELL
            .with(|method_name_ref_cell| method_name_ref_cell.borrow().clone());
        let manual = MANUAL_REF_CELL
            .with(|manual_ref_cell| manual_ref_cell.borrow().clone());
        let call_result = ic_cdk::api::call::call_raw(
                canister_id,
                &method,
                &args_raw,
                payment,
            )
            .await;
        UUID_REF_CELL
            .with(|uuid_ref_cell| {
                let mut uuid_mut = uuid_ref_cell.borrow_mut();
                *uuid_mut = uuid.clone();
            });
        METHOD_NAME_REF_CELL
            .with(|method_name_ref_cell| {
                let mut method_name_mut = method_name_ref_cell.borrow_mut();
                *method_name_mut = method_name.clone();
            });
        MANUAL_REF_CELL
            .with(|manual_ref_cell| {
                let mut manual_mut = manual_ref_cell.borrow_mut();
                *manual_mut = manual;
            });
        BOA_CONTEXT_REF_CELL
            .with(|boa_context_ref_cell| {
                let mut boa_context = boa_context_ref_cell.borrow_mut();
                let call_result_js_value = match call_result {
                    Ok(value) => {
                        let js_value = match value.try_into_vm_value(&mut *boa_context) {
                            Ok(js_value) => js_value,
                            Err(vmc_err) => {
                                js_promise_resolvers
                                    .reject
                                    .call(
                                        &boa_engine::JsValue::undefined(),
                                        &[vmc_err.to_js_error(None).to_opaque(&mut *boa_context)],
                                        &mut *boa_context,
                                    )
                                    .unwrap_or_trap(&mut *boa_context)
                            }
                        };
                        let canister_result_js_object = boa_engine::object::ObjectInitializer::new(
                                &mut *boa_context,
                            )
                            .property(
                                "Ok",
                                js_value,
                                boa_engine::property::Attribute::all(),
                            )
                            .build();
                        let canister_result_js_value = canister_result_js_object.into();
                        canister_result_js_value
                    }
                    Err(err) => {
                        let js_value = match format!(
                            "Rejection code {rejection_code}, {error_message}",
                            rejection_code = (err.0 as i32).to_string(), error_message =
                            err.1
                        )
                            .try_into_vm_value(&mut *boa_context)
                        {
                            Ok(js_value) => js_value,
                            Err(vmc_err) => {
                                js_promise_resolvers
                                    .reject
                                    .call(
                                        &boa_engine::JsValue::undefined(),
                                        &[vmc_err.to_js_error(None).to_opaque(&mut *boa_context)],
                                        &mut *boa_context,
                                    )
                                    .unwrap_or_trap(&mut *boa_context)
                            }
                        };
                        let canister_result_js_object = boa_engine::object::ObjectInitializer::new(
                                &mut *boa_context,
                            )
                            .property(
                                "Err",
                                js_value,
                                boa_engine::property::Attribute::all(),
                            )
                            .build();
                        let canister_result_js_value = canister_result_js_object.into();
                        canister_result_js_value
                    }
                };
                js_promise_resolvers
                    .resolve
                    .call(
                        &boa_engine::JsValue::undefined(),
                        &[call_result_js_value],
                        &mut *boa_context,
                    )
                    .unwrap_or_trap(&mut *boa_context);
                let main_promise = PROMISE_MAP_REF_CELL
                    .with(|promise_map_ref_cell| {
                        let promise_map = promise_map_ref_cell.borrow().clone();
                        let main_promise = promise_map
                            .get(&uuid)
                            .unwrap_or_trap(
                                "ReferenceError: top-level promise is not defined",
                            );
                        main_promise.clone()
                    });
                async_await_result_handler(
                        &mut *boa_context,
                        &main_promise,
                        &uuid,
                        &method_name,
                        manual,
                    )
                    .unwrap_or_trap(&mut *boa_context);
            });
    });
    Ok(js_promise.into())
}
fn call_raw128(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let (js_promise, js_promise_resolvers) = boa_engine::object::builtins::JsPromise::new_pending(
        context,
    );
    let canister_id_js_value = aargs
        .get(0)
        .ok_or_else(|| {
            "An argument for 'canisterId' was not provided".to_js_error(None)
        })?
        .clone();
    let method_js_value = aargs
        .get(1)
        .ok_or_else(|| "An argument for 'method' was not provided".to_js_error(None))?
        .clone();
    let args_raw_js_value = aargs
        .get(2)
        .ok_or_else(|| "An argument for 'argsRaw' was not provided".to_js_error(None))?
        .clone();
    let payment_js_value = aargs
        .get(3)
        .ok_or_else(|| "An argument for 'payment' was not provided".to_js_error(None))?
        .clone();
    let canister_id: candid::Principal = canister_id_js_value
        .try_from_vm_value(&mut *context)?;
    let method: String = method_js_value.try_from_vm_value(&mut *context)?;
    let args_raw: Vec<u8> = args_raw_js_value.try_from_vm_value(&mut *context)?;
    let payment: u128 = payment_js_value.try_from_vm_value(&mut *context)?;
    ic_cdk::spawn(async move {
        let uuid = UUID_REF_CELL.with(|uuid_ref_cell| uuid_ref_cell.borrow().clone());
        let method_name = METHOD_NAME_REF_CELL
            .with(|method_name_ref_cell| method_name_ref_cell.borrow().clone());
        let manual = MANUAL_REF_CELL
            .with(|manual_ref_cell| manual_ref_cell.borrow().clone());
        let call_result = ic_cdk::api::call::call_raw128(
                canister_id,
                &method,
                &args_raw,
                payment,
            )
            .await;
        UUID_REF_CELL
            .with(|uuid_ref_cell| {
                let mut uuid_mut = uuid_ref_cell.borrow_mut();
                *uuid_mut = uuid.clone();
            });
        METHOD_NAME_REF_CELL
            .with(|method_name_ref_cell| {
                let mut method_name_mut = method_name_ref_cell.borrow_mut();
                *method_name_mut = method_name.clone();
            });
        MANUAL_REF_CELL
            .with(|manual_ref_cell| {
                let mut manual_mut = manual_ref_cell.borrow_mut();
                *manual_mut = manual;
            });
        BOA_CONTEXT_REF_CELL
            .with(|boa_context_ref_cell| {
                let mut boa_context = boa_context_ref_cell.borrow_mut();
                let call_result_js_value = match call_result {
                    Ok(value) => {
                        let js_value = match value.try_into_vm_value(&mut *boa_context) {
                            Ok(js_value) => js_value,
                            Err(vmc_err) => {
                                js_promise_resolvers
                                    .reject
                                    .call(
                                        &boa_engine::JsValue::undefined(),
                                        &[vmc_err.to_js_error(None).to_opaque(&mut *boa_context)],
                                        &mut *boa_context,
                                    )
                                    .unwrap_or_trap(&mut *boa_context)
                            }
                        };
                        let canister_result_js_object = boa_engine::object::ObjectInitializer::new(
                                &mut *boa_context,
                            )
                            .property(
                                "Ok",
                                js_value,
                                boa_engine::property::Attribute::all(),
                            )
                            .build();
                        let canister_result_js_value = canister_result_js_object.into();
                        canister_result_js_value
                    }
                    Err(err) => {
                        let js_value = match format!(
                            "Rejection code {rejection_code}, {error_message}",
                            rejection_code = (err.0 as i32).to_string(), error_message =
                            err.1
                        )
                            .try_into_vm_value(&mut *boa_context)
                        {
                            Ok(js_value) => js_value,
                            Err(vmc_err) => {
                                js_promise_resolvers
                                    .reject
                                    .call(
                                        &boa_engine::JsValue::undefined(),
                                        &[vmc_err.to_js_error(None).to_opaque(&mut *boa_context)],
                                        &mut *boa_context,
                                    )
                                    .unwrap_or_trap(&mut *boa_context)
                            }
                        };
                        let canister_result_js_object = boa_engine::object::ObjectInitializer::new(
                                &mut *boa_context,
                            )
                            .property(
                                "Err",
                                js_value,
                                boa_engine::property::Attribute::all(),
                            )
                            .build();
                        let canister_result_js_value = canister_result_js_object.into();
                        canister_result_js_value
                    }
                };
                js_promise_resolvers
                    .resolve
                    .call(
                        &boa_engine::JsValue::undefined(),
                        &[call_result_js_value],
                        &mut *boa_context,
                    )
                    .unwrap_or_trap(&mut *boa_context);
                let main_promise = PROMISE_MAP_REF_CELL
                    .with(|promise_map_ref_cell| {
                        let promise_map = promise_map_ref_cell.borrow().clone();
                        let main_promise = promise_map
                            .get(&uuid)
                            .unwrap_or_trap(
                                "ReferenceError: top-level promise is not defined",
                            );
                        main_promise.clone()
                    });
                async_await_result_handler(
                        &mut *boa_context,
                        &main_promise,
                        &uuid,
                        &method_name,
                        manual,
                    )
                    .unwrap_or_trap(&mut *boa_context);
            });
    });
    Ok(js_promise.into())
}
fn caller(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    ic_cdk::api::caller()
        .try_into_vm_value(context)
        .map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn candid_decode(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let candid_encoded: Vec<u8> = aargs
        .get(0)
        .ok_or_else(|| {
            "An argument for 'candidEncoded' was not provided".to_js_error(None)
        })?
        .clone()
        .try_from_vm_value(&mut *context)?;
    let candid_args: candid::IDLArgs = candid::IDLArgs::from_bytes(&candid_encoded)
        .map_err(|err| {
            boa_engine::error::JsNativeError::error()
                .with_message(err.to_string().as_str())
        })?;
    let candid_string = candid_args.to_string();
    candid_string.try_into_vm_value(context).map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn candid_encode(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let candid_string: String = aargs
        .get(0)
        .ok_or_else(|| {
            "An argument for 'candidString' was not provided".to_js_error(None)
        })?
        .clone()
        .try_from_vm_value(&mut *context)?;
    let candid_args: candid::IDLArgs = candid_string
        .parse()
        .map_err(|err: candid::error::Error| {
            boa_engine::error::JsNativeError::error()
                .with_message(err.to_string().as_str())
        })?;
    let candid_encoded: Vec<u8> = candid_args
        .to_bytes()
        .map_err(|err: candid::error::Error| {
            boa_engine::error::JsNativeError::error()
                .with_message(err.to_string().as_str())
        })?;
    candid_encoded
        .try_into_vm_value(&mut *context)
        .map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn canister_balance(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    ic_cdk::api::canister_balance()
        .try_into_vm_value(context)
        .map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn canister_balance128(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    ic_cdk::api::canister_balance128()
        .try_into_vm_value(context)
        .map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn canister_version(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    ic_cdk::api::canister_version()
        .try_into_vm_value(context)
        .map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn clear_timer(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let timer_id: ic_cdk_timers::TimerId = aargs
        .get(0)
        .ok_or_else(|| "An argument for 'id' was not provided".to_js_error(None))?
        .clone()
        .try_from_vm_value(&mut *context)?;
    ic_cdk_timers::clear_timer(timer_id);
    Ok(boa_engine::JsValue::Undefined)
}
fn data_certificate(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    ic_cdk::api::data_certificate()
        .try_into_vm_value(context)
        .map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn id(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    ic_cdk::api::id()
        .try_into_vm_value(context)
        .map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn instruction_counter(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    ic_cdk::api::instruction_counter()
        .try_into_vm_value(context)
        .map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn is_controller(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let principal: candid::Principal = aargs
        .get(0)
        .ok_or_else(|| "An argument for 'principal' was not provided".to_js_error(None))?
        .clone()
        .try_from_vm_value(&mut *context)?;
    ic_cdk::api::is_controller(&principal)
        .try_into_vm_value(context)
        .map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn method_name(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    Ok(ic_cdk::api::call::method_name().into())
}
fn msg_cycles_accept(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let max_amount: u64 = aargs
        .get(0)
        .ok_or_else(|| "An argument for 'maxAmount' was not provided".to_js_error(None))?
        .clone()
        .try_from_vm_value(&mut *context)?;
    let return_value: boa_engine::bigint::JsBigInt = ic_cdk::api::call::msg_cycles_accept(
            max_amount,
        )
        .into();
    Ok(return_value.into())
}
fn msg_cycles_accept128(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let max_amount: u128 = aargs
        .get(0)
        .ok_or_else(|| "An argument for 'maxAmount' was not provided".to_js_error(None))?
        .clone()
        .try_from_vm_value(&mut *context)?;
    let return_value = boa_engine::bigint::JsBigInt::new(
        boa_engine::bigint::RawBigInt::from(
            ic_cdk::api::call::msg_cycles_accept128(max_amount),
        ),
    );
    Ok(return_value.into())
}
fn msg_cycles_available(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let return_value: boa_engine::bigint::JsBigInt = ic_cdk::api::call::msg_cycles_available()
        .into();
    Ok(return_value.into())
}
fn msg_cycles_available128(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let return_value: boa_engine::bigint::JsBigInt = ic_cdk::api::call::msg_cycles_available()
        .into();
    Ok(return_value.into())
}
fn msg_cycles_refunded(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let return_value: boa_engine::bigint::JsBigInt = ic_cdk::api::call::msg_cycles_refunded()
        .into();
    Ok(return_value.into())
}
fn msg_cycles_refunded128(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let return_value = boa_engine::bigint::JsBigInt::new(
        boa_engine::bigint::RawBigInt::from(ic_cdk::api::call::msg_cycles_refunded128()),
    );
    Ok(return_value.into())
}
fn notify_raw(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let canister_id_js_value = aargs
        .get(0)
        .ok_or_else(|| {
            "An argument for 'canisterId' was not provided".to_js_error(None)
        })?
        .clone();
    let method_js_value = aargs
        .get(1)
        .ok_or_else(|| "An argument for 'method' was not provided".to_js_error(None))?
        .clone();
    let args_raw_js_value = aargs
        .get(2)
        .ok_or_else(|| "An argument for 'argsRaw' was not provided".to_js_error(None))?
        .clone();
    let payment_js_value = aargs
        .get(3)
        .ok_or_else(|| "An argument for 'payment' was not provided".to_js_error(None))?
        .clone();
    let canister_id_principal: candid::Principal = canister_id_js_value
        .try_from_vm_value(&mut *context)?;
    let method_string: String = method_js_value.try_from_vm_value(&mut *context)?;
    let args_raw_vec: Vec<u8> = args_raw_js_value.try_from_vm_value(&mut *context)?;
    let payment: u128 = payment_js_value.try_from_vm_value(&mut *context)?;
    let notify_result = ic_cdk::api::call::notify_raw(
        canister_id_principal,
        &method_string,
        &args_raw_vec,
        payment,
    );
    notify_result.try_into_vm_value(context).map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn performance_counter(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let counter_type: u32 = aargs
        .get(0)
        .ok_or_else(|| {
            "An argument for 'counterType' was not provided".to_js_error(None)
        })?
        .clone()
        .try_from_vm_value(context)?;
    let return_value: boa_engine::bigint::JsBigInt = ic_cdk::api::call::performance_counter(
            counter_type,
        )
        .into();
    Ok(return_value.into())
}
fn print(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    ic_cdk::println!("{:#?}", aargs);
    return Ok(boa_engine::JsValue::Undefined);
}
fn reject(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let message: String = aargs
        .get(0)
        .ok_or_else(|| "An argument for 'message' was not provided".to_js_error(None))?
        .clone()
        .try_from_vm_value(&mut *context)?;
    ic_cdk::api::call::reject(&message)
        .try_into_vm_value(&mut *context)
        .map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn reject_code(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    ic_cdk::api::call::reject_code()
        .try_into_vm_value(context)
        .map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn reject_message(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    ic_cdk::api::call::reject_message()
        .try_into_vm_value(context)
        .map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn reply(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let method_name = METHOD_NAME_REF_CELL
        .with(|method_name_ref_cell| method_name_ref_cell.borrow().clone());
    match &method_name[..] {
        _ => {
            Err(
                format!("Missing reply handler for method '{method_name}'")
                    .to_js_error(None),
            )
        }
    }
}
fn reply_raw(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let buf_vec: Vec<u8> = aargs
        .get(0)
        .ok_or_else(|| "An argument for 'buf' was not provided".to_js_error(None))?
        .clone()
        .try_from_vm_value(&mut *context)?;
    ic_cdk::api::call::reply_raw(&buf_vec)
        .try_into_vm_value(context)
        .map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn set_certified_data(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let data: Vec<u8> = aargs
        .get(0)
        .ok_or_else(|| "An argument for 'data' was not provided".to_js_error(None))?
        .clone()
        .try_from_vm_value(&mut *context)?;
    ic_cdk::api::set_certified_data(&data)
        .try_into_vm_value(context)
        .map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn set_timer(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let delay_js_value = aargs
        .get(0)
        .ok_or_else(|| "An argument for 'delay' was not provided".to_js_error(None))?
        .clone();
    let func_js_value = aargs
        .get(1)
        .ok_or_else(|| "An argument for 'callback' was not provided".to_js_error(None))?;
    let delay_as_u64: u64 = delay_js_value.try_from_vm_value(&mut *context)?;
    let delay = core::time::Duration::new(delay_as_u64, 0);
    if !func_js_value.is_callable() {
        return Err("TypeError: 'callback' is not a function".to_js_error(None));
    }
    let func_js_object = func_js_value
        .as_object()
        .ok_or_else(|| "TypeError: 'callback' is not a function".to_js_error(None))?
        .clone();
    let closure = move || {
        BOA_CONTEXT_REF_CELL
            .with(|boa_context_ref_cell| {
                let mut boa_context = boa_context_ref_cell.borrow_mut();
                let uuid = uuid::Uuid::new_v4().to_string();
                UUID_REF_CELL
                    .with(|uuid_ref_cell| {
                        let mut uuid_mut = uuid_ref_cell.borrow_mut();
                        *uuid_mut = uuid.clone();
                    });
                METHOD_NAME_REF_CELL
                    .with(|method_name_ref_cell| {
                        let mut method_name_mut = method_name_ref_cell.borrow_mut();
                        *method_name_mut = "_AZLE_TIMER".to_string();
                    });
                MANUAL_REF_CELL
                    .with(|manual_ref_cell| {
                        let mut manual_mut = manual_ref_cell.borrow_mut();
                        *manual_mut = false;
                    });
                let boa_return_value = func_js_object
                    .call(&boa_engine::JsValue::Null, &[], &mut *boa_context)
                    .unwrap_or_trap(&mut *boa_context);
                async_await_result_handler(
                        &mut boa_context,
                        &boa_return_value,
                        &uuid,
                        "_AZLE_TIMER",
                        false,
                    )
                    .unwrap_or_trap(&mut *boa_context);
            });
    };
    let timer_id = ic_cdk_timers::set_timer(delay, closure);
    timer_id.try_into_vm_value(context).map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn set_timer_interval(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let interval_js_value = aargs
        .get(0)
        .ok_or_else(|| "An argument for 'interval' was not provided".to_js_error(None))?
        .clone();
    let func_js_value = aargs
        .get(1)
        .ok_or_else(|| "An argument for 'callback' was not provided".to_js_error(None))?;
    let interval_as_u64: u64 = interval_js_value.try_from_vm_value(&mut *context)?;
    let interval = core::time::Duration::new(interval_as_u64, 0);
    if !func_js_value.is_callable() {
        return Err("TypeError: 'callback' is not a function".to_js_error(None));
    }
    let func_js_object = func_js_value
        .as_object()
        .ok_or_else(|| "TypeError: 'callback' is not a function".to_js_error(None))?
        .clone();
    let closure = move || {
        BOA_CONTEXT_REF_CELL
            .with(|boa_context_ref_cell| {
                let mut boa_context = boa_context_ref_cell.borrow_mut();
                let uuid = uuid::Uuid::new_v4().to_string();
                UUID_REF_CELL
                    .with(|uuid_ref_cell| {
                        let mut uuid_mut = uuid_ref_cell.borrow_mut();
                        *uuid_mut = uuid.clone();
                    });
                METHOD_NAME_REF_CELL
                    .with(|method_name_ref_cell| {
                        let mut method_name_mut = method_name_ref_cell.borrow_mut();
                        *method_name_mut = "_AZLE_TIMER".to_string();
                    });
                MANUAL_REF_CELL
                    .with(|manual_ref_cell| {
                        let mut manual_mut = manual_ref_cell.borrow_mut();
                        *manual_mut = false;
                    });
                let boa_return_value = func_js_object
                    .call(&boa_engine::JsValue::Null, &[], &mut *boa_context)
                    .unwrap_or_trap(&mut *boa_context);
                async_await_result_handler(
                        &mut boa_context,
                        &boa_return_value,
                        &uuid,
                        "_AZLE_TIMER",
                        false,
                    )
                    .unwrap_or_trap(&mut *boa_context);
            });
    };
    let timer_id = ic_cdk_timers::set_timer_interval(interval, closure);
    timer_id.try_into_vm_value(context).map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn stable64_grow(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let new_pages: u64 = aargs
        .get(0)
        .ok_or_else(|| "An argument for 'newPages' was not provided".to_js_error(None))?
        .clone()
        .try_from_vm_value(&mut *context)?;
    ic_cdk::api::stable::stable64_grow(new_pages)
        .try_into_vm_value(context)
        .map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn stable64_read(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let offset: u64 = aargs
        .get(0)
        .ok_or_else(|| "An argument for 'offset' was not provided".to_js_error(None))?
        .clone()
        .try_from_vm_value(&mut *context)?;
    let length: u64 = aargs
        .get(1)
        .ok_or_else(|| "An argument for 'length' was not provided".to_js_error(None))?
        .clone()
        .try_from_vm_value(&mut *context)?;
    let mut buf: Vec<u8> = vec![0; length as usize];
    ic_cdk::api::stable::stable64_read(offset, &mut buf);
    buf.to_vec().try_into_vm_value(context).map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn stable64_size(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    ic_cdk::api::stable::stable64_size()
        .try_into_vm_value(context)
        .map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn stable64_write(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let offset: u64 = aargs
        .get(0)
        .ok_or_else(|| "An argument for 'offset' was not provided".to_js_error(None))?
        .clone()
        .try_from_vm_value(&mut *context)?;
    let buf_vector: Vec<u8> = aargs
        .get(1)
        .ok_or_else(|| "An argument for 'buffer' was not provided".to_js_error(None))?
        .clone()
        .try_from_vm_value(&mut *context)?;
    let buf: &[u8] = &buf_vector[..];
    ic_cdk::api::stable::stable64_write(offset, buf);
    Ok(boa_engine::JsValue::Undefined)
}
fn stable_b_tree_map_contains_key(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let memory_id: u8 = aargs
        .get(0)
        .ok_or_else(|| "An argument for 'memoryId' was not provided".to_js_error(None))?
        .clone()
        .try_from_vm_value(&mut *context)?;
    let key_js_value = aargs
        .get(1)
        .ok_or_else(|| "An argument for 'key' was not provided".to_js_error(None))?
        .clone();
    match memory_id {
        _ => {
            Err(
                format!(
                    "Memory id {} does not have an associated StableBTreeMap", memory_id
                )
                    .to_js_error(None),
            )
        }
    }
}
fn stable_b_tree_map_get(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let memory_id: u8 = aargs
        .get(0)
        .ok_or_else(|| "An argument for 'memoryId' was not provided".to_js_error(None))?
        .clone()
        .try_from_vm_value(&mut *context)?;
    let key_js_value = aargs
        .get(1)
        .ok_or_else(|| "An argument for 'key' was not provided".to_js_error(None))?
        .clone();
    match memory_id {
        _ => {
            Err(
                format!(
                    "Memory id {} does not have an associated StableBTreeMap", memory_id
                )
                    .to_js_error(None),
            )
        }
    }
}
fn stable_b_tree_map_insert(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let memory_id: u8 = aargs
        .get(0)
        .ok_or_else(|| "An argument for 'memoryId' was not provided".to_js_error(None))?
        .clone()
        .try_from_vm_value(&mut *context)?;
    let key_js_value = aargs
        .get(1)
        .ok_or_else(|| "An argument for 'key' was not provided".to_js_error(None))?
        .clone();
    let value_js_value = aargs
        .get(2)
        .ok_or_else(|| "An argument for 'value' was not provided".to_js_error(None))?
        .clone();
    match memory_id {
        _ => {
            Err(
                format!(
                    "Memory id {} does not have an associated StableBTreeMap", memory_id
                )
                    .to_js_error(None),
            )
        }
    }
}
fn stable_b_tree_map_is_empty(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let memory_id: u8 = aargs
        .get(0)
        .ok_or_else(|| "An argument for 'memoryId' was not provided".to_js_error(None))?
        .clone()
        .try_from_vm_value(&mut *context)?;
    match memory_id {
        _ => {
            Err(
                format!(
                    "Memory id {} does not have an associated StableBTreeMap", memory_id
                )
                    .to_js_error(None),
            )
        }
    }
}
fn stable_b_tree_map_items(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let memory_id: u8 = aargs
        .get(0)
        .ok_or_else(|| "An argument for 'memoryId' was not provided".to_js_error(None))?
        .clone()
        .try_from_vm_value(&mut *context)?;
    match memory_id {
        _ => {
            Err(
                format!(
                    "Memory id {} does not have an associated StableBTreeMap", memory_id
                )
                    .to_js_error(None),
            )
        }
    }
}
fn stable_b_tree_map_keys(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let memory_id: u8 = aargs
        .get(0)
        .ok_or_else(|| "An argument for 'memoryId' was not provided".to_js_error(None))?
        .clone()
        .try_from_vm_value(&mut *context)?;
    match memory_id {
        _ => {
            Err(
                format!(
                    "Memory id {} does not have an associated StableBTreeMap", memory_id
                )
                    .to_js_error(None),
            )
        }
    }
}
fn stable_b_tree_map_len(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let memory_id: u8 = aargs
        .get(0)
        .ok_or_else(|| "An argument for 'memoryId' was not provided".to_js_error(None))?
        .clone()
        .try_from_vm_value(&mut *context)?;
    match memory_id {
        _ => {
            Err(
                format!(
                    "Memory id {} does not have an associated StableBTreeMap", memory_id
                )
                    .to_js_error(None),
            )
        }
    }
}
fn stable_b_tree_map_remove(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let memory_id: u8 = aargs
        .get(0)
        .ok_or_else(|| "An argument for 'memoryId' was not provided".to_js_error(None))?
        .clone()
        .try_from_vm_value(&mut *context)?;
    let key_js_value = aargs
        .get(1)
        .ok_or_else(|| "An argument for 'key' was not provided".to_js_error(None))?
        .clone();
    match memory_id {
        _ => {
            Err(
                format!(
                    "Memory id {} does not have an associated StableBTreeMap", memory_id
                )
                    .to_js_error(None),
            )
        }
    }
}
fn stable_b_tree_map_values(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let memory_id: u8 = aargs
        .get(0)
        .ok_or_else(|| "An argument for 'memoryId' was not provided".to_js_error(None))?
        .clone()
        .try_from_vm_value(&mut *context)?;
    match memory_id {
        _ => {
            Err(
                format!(
                    "Memory id {} does not have an associated StableBTreeMap", memory_id
                )
                    .to_js_error(None),
            )
        }
    }
}
fn stable_bytes(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    ic_cdk::api::stable::stable_bytes()
        .try_into_vm_value(context)
        .map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn stable_grow(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let new_pages: u32 = aargs
        .get(0)
        .ok_or_else(|| "An argument for 'newPages' was not provided".to_js_error(None))?
        .clone()
        .try_from_vm_value(&mut *context)?;
    ic_cdk::api::stable::stable_grow(new_pages)
        .try_into_vm_value(context)
        .map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn stable_read(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let offset: u32 = aargs
        .get(0)
        .ok_or_else(|| "An argument for 'offset' was not provided".to_js_error(None))?
        .clone()
        .try_from_vm_value(&mut *context)?;
    let length: u32 = aargs
        .get(1)
        .ok_or_else(|| "An argument for 'length' was not provided".to_js_error(None))?
        .clone()
        .try_from_vm_value(&mut *context)?;
    let mut buf: Vec<u8> = vec![0; length as usize];
    ic_cdk::api::stable::stable_read(offset, &mut buf);
    buf.to_vec().try_into_vm_value(context).map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn stable_size(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    ic_cdk::api::stable::stable_size()
        .try_into_vm_value(context)
        .map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn stable_write(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let offset: u32 = aargs
        .get(0)
        .ok_or_else(|| "An argument for 'offset' was not provided".to_js_error(None))?
        .clone()
        .try_from_vm_value(&mut *context)?;
    let buf_vector: Vec<u8> = aargs
        .get(1)
        .ok_or_else(|| "An argument for 'buffer' was not provided".to_js_error(None))?
        .clone()
        .try_from_vm_value(&mut *context)?;
    let buf: &[u8] = &buf_vector[..];
    ic_cdk::api::stable::stable_write(offset, buf);
    Ok(boa_engine::JsValue::Undefined)
}
fn time(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    ic_cdk::api::time()
        .try_into_vm_value(context)
        .map_err(|vmc_err| vmc_err.to_js_error(None))
}
fn trap(
    _this: &boa_engine::JsValue,
    aargs: &[boa_engine::JsValue],
    context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let message: String = aargs
        .get(0)
        .ok_or_else(|| "An argument for 'message' was not provided".to_js_error(None))?
        .clone()
        .try_from_vm_value(context)?;
    ic_cdk::api::trap(&message);
}
fn register_ic_object(boa_context: &mut boa_engine::Context) {
    let ic = boa_engine::object::ObjectInitializer::new(boa_context)
        .function(
            boa_engine::NativeFunction::from_fn_ptr(accept_message),
            "acceptMessage",
            0,
        )
        .function(boa_engine::NativeFunction::from_fn_ptr(arg_data_raw), "argDataRaw", 0)
        .function(
            boa_engine::NativeFunction::from_fn_ptr(arg_data_raw_size),
            "argDataRawSize",
            0,
        )
        .function(boa_engine::NativeFunction::from_fn_ptr(call_raw), "callRaw", 0)
        .function(boa_engine::NativeFunction::from_fn_ptr(call_raw128), "callRaw128", 0)
        .function(boa_engine::NativeFunction::from_fn_ptr(caller), "caller", 0)
        .function(
            boa_engine::NativeFunction::from_fn_ptr(candid_decode),
            "candidDecode",
            0,
        )
        .function(
            boa_engine::NativeFunction::from_fn_ptr(candid_encode),
            "candidEncode",
            0,
        )
        .function(
            boa_engine::NativeFunction::from_fn_ptr(canister_balance),
            "canisterBalance",
            0,
        )
        .function(
            boa_engine::NativeFunction::from_fn_ptr(canister_balance128),
            "canisterBalance128",
            0,
        )
        .function(
            boa_engine::NativeFunction::from_fn_ptr(canister_version),
            "canisterVersion",
            0,
        )
        .function(boa_engine::NativeFunction::from_fn_ptr(clear_timer), "clearTimer", 0)
        .function(
            boa_engine::NativeFunction::from_fn_ptr(data_certificate),
            "dataCertificate",
            0,
        )
        .function(boa_engine::NativeFunction::from_fn_ptr(id), "id", 0)
        .function(
            boa_engine::NativeFunction::from_fn_ptr(instruction_counter),
            "instructionCounter",
            0,
        )
        .function(
            boa_engine::NativeFunction::from_fn_ptr(is_controller),
            "isController",
            0,
        )
        .function(boa_engine::NativeFunction::from_fn_ptr(method_name), "methodName", 0)
        .function(
            boa_engine::NativeFunction::from_fn_ptr(msg_cycles_accept),
            "msgCyclesAccept",
            0,
        )
        .function(
            boa_engine::NativeFunction::from_fn_ptr(msg_cycles_accept128),
            "msgCyclesAccept128",
            0,
        )
        .function(
            boa_engine::NativeFunction::from_fn_ptr(msg_cycles_available),
            "msgCyclesAvailable",
            0,
        )
        .function(
            boa_engine::NativeFunction::from_fn_ptr(msg_cycles_available128),
            "msgCyclesAvailable128",
            0,
        )
        .function(
            boa_engine::NativeFunction::from_fn_ptr(msg_cycles_refunded),
            "msgCyclesRefunded",
            0,
        )
        .function(
            boa_engine::NativeFunction::from_fn_ptr(msg_cycles_refunded128),
            "msgCyclesRefunded128",
            0,
        )
        .function(boa_engine::NativeFunction::from_fn_ptr(notify_raw), "notifyRaw", 0)
        .function(
            boa_engine::NativeFunction::from_fn_ptr(performance_counter),
            "performanceCounter",
            0,
        )
        .function(boa_engine::NativeFunction::from_fn_ptr(print), "print", 0)
        .function(boa_engine::NativeFunction::from_fn_ptr(reject), "reject", 0)
        .function(boa_engine::NativeFunction::from_fn_ptr(reject_code), "rejectCode", 0)
        .function(
            boa_engine::NativeFunction::from_fn_ptr(reject_message),
            "rejectMessage",
            0,
        )
        .function(boa_engine::NativeFunction::from_fn_ptr(reply), "reply", 0)
        .function(boa_engine::NativeFunction::from_fn_ptr(reply_raw), "replyRaw", 0)
        .function(
            boa_engine::NativeFunction::from_fn_ptr(set_certified_data),
            "setCertifiedData",
            0,
        )
        .function(boa_engine::NativeFunction::from_fn_ptr(set_timer), "setTimer", 0)
        .function(
            boa_engine::NativeFunction::from_fn_ptr(set_timer_interval),
            "setTimerInterval",
            0,
        )
        .function(
            boa_engine::NativeFunction::from_fn_ptr(stable_bytes),
            "stableBytes",
            0,
        )
        .function(
            boa_engine::NativeFunction::from_fn_ptr(stable_b_tree_map_contains_key),
            "stableBTreeMapContainsKey",
            0,
        )
        .function(
            boa_engine::NativeFunction::from_fn_ptr(stable_b_tree_map_get),
            "stableBTreeMapGet",
            0,
        )
        .function(
            boa_engine::NativeFunction::from_fn_ptr(stable_b_tree_map_insert),
            "stableBTreeMapInsert",
            0,
        )
        .function(
            boa_engine::NativeFunction::from_fn_ptr(stable_b_tree_map_is_empty),
            "stableBTreeMapIsEmpty",
            0,
        )
        .function(
            boa_engine::NativeFunction::from_fn_ptr(stable_b_tree_map_items),
            "stableBTreeMapItems",
            0,
        )
        .function(
            boa_engine::NativeFunction::from_fn_ptr(stable_b_tree_map_keys),
            "stableBTreeMapKeys",
            0,
        )
        .function(
            boa_engine::NativeFunction::from_fn_ptr(stable_b_tree_map_values),
            "stableBTreeMapValues",
            0,
        )
        .function(
            boa_engine::NativeFunction::from_fn_ptr(stable_b_tree_map_len),
            "stableBTreeMapLen",
            0,
        )
        .function(
            boa_engine::NativeFunction::from_fn_ptr(stable_b_tree_map_remove),
            "stableBTreeMapRemove",
            0,
        )
        .function(boa_engine::NativeFunction::from_fn_ptr(stable_grow), "stableGrow", 0)
        .function(boa_engine::NativeFunction::from_fn_ptr(stable_read), "stableRead", 0)
        .function(boa_engine::NativeFunction::from_fn_ptr(stable_size), "stableSize", 0)
        .function(
            boa_engine::NativeFunction::from_fn_ptr(stable_write),
            "stableWrite",
            0,
        )
        .function(
            boa_engine::NativeFunction::from_fn_ptr(stable64_grow),
            "stable64Grow",
            0,
        )
        .function(
            boa_engine::NativeFunction::from_fn_ptr(stable64_read),
            "stable64Read",
            0,
        )
        .function(
            boa_engine::NativeFunction::from_fn_ptr(stable64_size),
            "stable64Size",
            0,
        )
        .function(
            boa_engine::NativeFunction::from_fn_ptr(stable64_write),
            "stable64Write",
            0,
        )
        .function(boa_engine::NativeFunction::from_fn_ptr(time), "time", 0)
        .function(boa_engine::NativeFunction::from_fn_ptr(trap), "trap", 0)
        .build();
    boa_context
        .register_global_property("ic", ic, boa_engine::property::Attribute::all());
}
thread_local! {
    static MEMORY_MANAGER_REF_CELL : std::cell::RefCell <
    ic_stable_structures::memory_manager::MemoryManager <
    ic_stable_structures::DefaultMemoryImpl >> =
    std::cell::RefCell::new(ic_stable_structures::memory_manager::MemoryManager::init(ic_stable_structures::DefaultMemoryImpl::default()));
}
fn unwrap_or_trap<SuccessValue, Callback>(callback: Callback) -> SuccessValue
where
    Callback: FnOnce() -> Result<SuccessValue, RuntimeError>,
{
    callback()
        .unwrap_or_else(|err| ic_cdk::api::trap(
            &format!("\nUncaught {}", err.to_string()),
        ))
}
pub trait UnwrapJsResultOrTrap {
    fn unwrap_or_trap(self, context: &mut boa_engine::Context) -> boa_engine::JsValue;
}
impl UnwrapJsResultOrTrap for boa_engine::JsResult<boa_engine::JsValue> {
    fn unwrap_or_trap(self, context: &mut boa_engine::Context) -> boa_engine::JsValue {
        match self {
            Ok(js_value) => js_value,
            Err(js_error) => {
                let error_message = js_error.to_std_string(context);
                ic_cdk::api::trap(&format!("\nUncaught {error_message}"));
            }
        }
    }
}
impl UnwrapJsResultOrTrap for Result<boa_engine::JsValue, String> {
    fn unwrap_or_trap(self, context: &mut boa_engine::Context) -> boa_engine::JsValue {
        match self {
            Ok(js_value) => js_value,
            Err(error_string) => {
                ic_cdk::api::trap(&format!("\nUncaught {error_string}"));
            }
        }
    }
}
pub trait UnwrapOrTrapWithMessage<T> {
    fn unwrap_or_trap(self, err_message: &str) -> T;
}
impl<T> UnwrapOrTrapWithMessage<T> for Option<T> {
    fn unwrap_or_trap(self, err_message: &str) -> T {
        match self {
            Some(some) => some,
            None => ic_cdk::trap(&format!("\nUncaught {err_message}")),
        }
    }
}
trait ToStdString {
    fn to_std_string(self, context: &mut boa_engine::Context) -> String;
}
impl ToStdString for boa_engine::JsError {
    fn to_std_string(self, context: &mut boa_engine::Context) -> String {
        self.to_opaque(context).to_std_string(context)
    }
}
impl ToStdString for boa_engine::JsValue {
    fn to_std_string(self, context: &mut boa_engine::Context) -> String {
        match &self {
            boa_engine::JsValue::BigInt(bigint) => bigint.to_string(),
            boa_engine::JsValue::Boolean(boolean) => boolean.to_string(),
            boa_engine::JsValue::Integer(integer) => integer.to_string(),
            boa_engine::JsValue::Null => "null".to_string(),
            boa_engine::JsValue::Object(object) => {
                js_object_to_string(&self, &object, context)
            }
            boa_engine::JsValue::Rational(rational) => rational.to_string(),
            boa_engine::JsValue::String(string) => {
                string
                    .to_std_string()
                    .unwrap_or_else(|err| format!("InternalError: {err}"))
            }
            boa_engine::JsValue::Symbol(symbol) => symbol.to_string(),
            boa_engine::JsValue::Undefined => "undefined".to_string(),
        }
    }
}
fn js_object_to_string(
    error_value: &boa_engine::JsValue,
    js_object: &boa_engine::JsObject,
    context: &mut boa_engine::Context,
) -> String {
    if js_object.is_error() {
        return js_error_object_to_string(js_object, context);
    }
    let to_string_js_value = match js_object.get("toString", context) {
        Ok(to_string_js_value) => to_string_js_value,
        Err(err) => {
            return "TypeError: Property 'toString' of object is not a function"
                .to_string();
        }
    };
    let to_string_js_object = match to_string_js_value.as_object() {
        Some(to_string_js_object) => to_string_js_object,
        None => {
            return "TypeError: Property 'toString' of object is not a function"
                .to_string();
        }
    };
    let string_js_value = match to_string_js_object.call(error_value, &[], context) {
        Ok(string_js_value) => string_js_value,
        Err(js_error) => return format!("InternalError: {js_error}"),
    };
    string_js_value
        .try_from_vm_value(context)
        .unwrap_or_else(|js_error| format!("InternalError: {js_error}"))
}
fn js_error_object_to_string(
    js_object: &boa_engine::JsObject,
    context: &mut boa_engine::Context,
) -> String {
    try_js_error_object_to_string(js_object, context)
        .unwrap_or_else(|js_error| {
            let cause = js_error.to_std_string(&mut *context);
            format!(
                "InternalError: Encountered an error while serializing an error\n  \
                        [cause]: {cause}"
            )
        })
}
fn try_js_error_object_to_string(
    js_object: &boa_engine::JsObject,
    context: &mut boa_engine::Context,
) -> Result<String, boa_engine::JsError> {
    let error_name = get_js_error_name(js_object, context)?;
    let error_message = get_js_error_message(js_object, context)?;
    let cause_opt = get_js_error_cause(js_object, context)?;
    let error_string = match cause_opt {
        Some(cause) => format!("{error_name}: {error_message}\n  [cause]: {cause}"),
        None => format!("{error_name}: {error_message}"),
    };
    Ok(error_string)
}
fn get_js_error_name(
    js_object: &boa_engine::JsObject,
    context: &mut boa_engine::Context,
) -> Result<String, boa_engine::JsError> {
    match js_object.prototype() {
        Some(prototype_js_object) => {
            prototype_js_object
                .get("name", &mut *context)?
                .try_from_vm_value(&mut *context)
        }
        None => Ok("Error".to_string()),
    }
}
fn get_js_error_message(
    js_object: &boa_engine::JsObject,
    context: &mut boa_engine::Context,
) -> Result<String, boa_engine::JsError> {
    js_object.get("message", &mut *context)?.try_from_vm_value(&mut *context)
}
fn get_js_error_cause(
    js_object: &boa_engine::JsObject,
    context: &mut boa_engine::Context,
) -> Result<Option<String>, boa_engine::JsError> {
    match js_object.get("cause", &mut *context) {
        Ok(cause_js_value) => {
            if cause_js_value.is_undefined() {
                Ok(None)
            } else {
                Ok(Some(cause_js_value.to_std_string(&mut *context)))
            }
        }
        Err(js_error) => Err(js_error),
    }
}
#[ic_cdk_macros::query]
fn __get_candid_interface_tmp_hack() -> String {
    __export_service()
}
#[ic_cdk_macros::init()]
#[candid::candid_method(init)]
fn init() {
    unwrap_or_trap(|| {
        BOA_CONTEXT_REF_CELL
            .with(|boa_context_ref_cell| {
                let mut boa_context = boa_context_ref_cell.borrow_mut();
                METHOD_NAME_REF_CELL
                    .with(|method_name_ref_cell| {
                        let mut method_name_mut = method_name_ref_cell.borrow_mut();
                        *method_name_mut = "DOES_NOT_EXIST".to_string();
                    });
                register_ic_object(&mut boa_context);
                let env = boa_engine::object::ObjectInitializer::new(&mut boa_context)
                    .build();
                let process = boa_engine::object::ObjectInitializer::new(
                        &mut boa_context,
                    )
                    .property("env", env, boa_engine::property::Attribute::all())
                    .build();
                boa_context
                    .register_global_property(
                        "process",
                        process,
                        boa_engine::property::Attribute::all(),
                    );
                boa_context
                    .eval(
                        boa_engine::Source::from_bytes(
                            &format!(
                                "let exports = {{}}; {compiled_js}", compiled_js = MAIN_JS
                            ),
                        ),
                    )?;
                ic_cdk_timers::set_timer(core::time::Duration::new(0, 0), rng_seed);
                Ok(())
            })
    })
}
#[ic_cdk_macros::post_upgrade()]
fn post_upgrade() {
    unwrap_or_trap(|| {
        BOA_CONTEXT_REF_CELL
            .with(|boa_context_ref_cell| {
                let mut boa_context = boa_context_ref_cell.borrow_mut();
                METHOD_NAME_REF_CELL
                    .with(|method_name_ref_cell| {
                        let mut method_name_mut = method_name_ref_cell.borrow_mut();
                        *method_name_mut = "DOES_NOT_EXIST".to_string();
                    });
                register_ic_object(&mut boa_context);
                let env = boa_engine::object::ObjectInitializer::new(&mut boa_context)
                    .build();
                let process = boa_engine::object::ObjectInitializer::new(
                        &mut boa_context,
                    )
                    .property("env", env, boa_engine::property::Attribute::all())
                    .build();
                boa_context
                    .register_global_property(
                        "process",
                        process,
                        boa_engine::property::Attribute::all(),
                    );
                boa_context
                    .eval(
                        boa_engine::Source::from_bytes(
                            &format!(
                                "let exports = {{}}; {compiled_js}", compiled_js = MAIN_JS
                            ),
                        ),
                    )?;
                ic_cdk_timers::set_timer(core::time::Duration::new(0, 0), rng_seed);
                Ok(())
            })
    })
}
#[ic_cdk_macros::query(name = "get")]
#[candid::candid_method(query, rename = "get")]
async fn _cdk_user_defined_get() -> (candid::Nat) {
    unwrap_or_trap(|| {
        BOA_CONTEXT_REF_CELL
            .with(|boa_context_ref_cell| {
                let mut boa_context = boa_context_ref_cell.borrow_mut();
                let uuid = uuid::Uuid::new_v4().to_string();
                UUID_REF_CELL
                    .with(|uuid_ref_cell| {
                        let mut uuid_mut = uuid_ref_cell.borrow_mut();
                        *uuid_mut = uuid.clone();
                    });
                METHOD_NAME_REF_CELL
                    .with(|method_name_ref_cell| {
                        let mut method_name_mut = method_name_ref_cell.borrow_mut();
                        *method_name_mut = "get".to_string();
                    });
                MANUAL_REF_CELL
                    .with(|manual_ref_cell| {
                        let mut manual_mut = manual_ref_cell.borrow_mut();
                        *manual_mut = false;
                    });
                let exports_js_value = boa_context
                    .eval(boa_engine::Source::from_bytes("exports"))?;
                let exports_js_object = exports_js_value
                    .as_object()
                    .ok_or_else(|| RuntimeError::TypeError(
                        "'exports' is not an object".to_string(),
                    ))?;
                let function_js_value = exports_js_object.get("get", &mut boa_context)?;
                let function_js_object = function_js_value
                    .as_object()
                    .ok_or_else(|| RuntimeError::ReferenceError(
                        format!("{} is not defined", "get"),
                    ))?;
                let boa_return_value = function_js_object
                    .call(&boa_engine::JsValue::Null, &[], &mut boa_context)?;
                let final_return_value = async_await_result_handler(
                    &mut boa_context,
                    &boa_return_value,
                    &uuid,
                    "get",
                    false,
                )?;
                Ok(final_return_value.try_from_vm_value(&mut *boa_context)?)
            })
    })
}
#[ic_cdk_macros::update(name = "add")]
#[candid::candid_method(update, rename = "add")]
async fn _cdk_user_defined_add(_cdk_user_defined_n: candid::Nat) -> (candid::Nat) {
    unwrap_or_trap(|| {
        BOA_CONTEXT_REF_CELL
            .with(|boa_context_ref_cell| {
                let mut boa_context = boa_context_ref_cell.borrow_mut();
                let uuid = uuid::Uuid::new_v4().to_string();
                UUID_REF_CELL
                    .with(|uuid_ref_cell| {
                        let mut uuid_mut = uuid_ref_cell.borrow_mut();
                        *uuid_mut = uuid.clone();
                    });
                METHOD_NAME_REF_CELL
                    .with(|method_name_ref_cell| {
                        let mut method_name_mut = method_name_ref_cell.borrow_mut();
                        *method_name_mut = "add".to_string();
                    });
                MANUAL_REF_CELL
                    .with(|manual_ref_cell| {
                        let mut manual_mut = manual_ref_cell.borrow_mut();
                        *manual_mut = false;
                    });
                let exports_js_value = boa_context
                    .eval(boa_engine::Source::from_bytes("exports"))?;
                let exports_js_object = exports_js_value
                    .as_object()
                    .ok_or_else(|| RuntimeError::TypeError(
                        "'exports' is not an object".to_string(),
                    ))?;
                let function_js_value = exports_js_object.get("add", &mut boa_context)?;
                let function_js_object = function_js_value
                    .as_object()
                    .ok_or_else(|| RuntimeError::ReferenceError(
                        format!("{} is not defined", "add"),
                    ))?;
                let boa_return_value = function_js_object
                    .call(
                        &boa_engine::JsValue::Null,
                        &[_cdk_user_defined_n.try_into_vm_value(&mut boa_context)?],
                        &mut boa_context,
                    )?;
                let final_return_value = async_await_result_handler(
                    &mut boa_context,
                    &boa_return_value,
                    &uuid,
                    "add",
                    false,
                )?;
                Ok(final_return_value.try_from_vm_value(&mut *boa_context)?)
            })
    })
}
#[ic_cdk_macros::update(name = "inc")]
#[candid::candid_method(update, rename = "inc")]
async fn _cdk_user_defined_inc() -> () {
    unwrap_or_trap(|| {
        BOA_CONTEXT_REF_CELL
            .with(|boa_context_ref_cell| {
                let mut boa_context = boa_context_ref_cell.borrow_mut();
                let uuid = uuid::Uuid::new_v4().to_string();
                UUID_REF_CELL
                    .with(|uuid_ref_cell| {
                        let mut uuid_mut = uuid_ref_cell.borrow_mut();
                        *uuid_mut = uuid.clone();
                    });
                METHOD_NAME_REF_CELL
                    .with(|method_name_ref_cell| {
                        let mut method_name_mut = method_name_ref_cell.borrow_mut();
                        *method_name_mut = "inc".to_string();
                    });
                MANUAL_REF_CELL
                    .with(|manual_ref_cell| {
                        let mut manual_mut = manual_ref_cell.borrow_mut();
                        *manual_mut = false;
                    });
                let exports_js_value = boa_context
                    .eval(boa_engine::Source::from_bytes("exports"))?;
                let exports_js_object = exports_js_value
                    .as_object()
                    .ok_or_else(|| RuntimeError::TypeError(
                        "'exports' is not an object".to_string(),
                    ))?;
                let function_js_value = exports_js_object.get("inc", &mut boa_context)?;
                let function_js_object = function_js_value
                    .as_object()
                    .ok_or_else(|| RuntimeError::ReferenceError(
                        format!("{} is not defined", "inc"),
                    ))?;
                let boa_return_value = function_js_object
                    .call(&boa_engine::JsValue::Null, &[], &mut boa_context)?;
                let final_return_value = async_await_result_handler(
                    &mut boa_context,
                    &boa_return_value,
                    &uuid,
                    "inc",
                    false,
                )?;
                if !final_return_value.is_undefined() {
                    return Err(
                        RuntimeError::TypeError(
                            "Value is not of type 'void'".to_string(),
                        ),
                    );
                }
                Ok(final_return_value.try_from_vm_value(&mut *boa_context)?)
            })
    })
}
type NotifyResult = (_AzleResult<(), RejectionCode>);
type Stable64GrowResult = (_AzleResult<u64, StableMemoryError>);
type StableGrowResult = (_AzleResult<u32, StableMemoryError>);
#[derive(
    serde::Deserialize,
    Debug,
    candid::CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq
)]
enum Opt<Value>
where
    Value: for<'a, 'b> CdkActTryIntoVmValue<
        &'a mut boa_engine::Context<'b>,
        boa_engine::JsValue,
    >,
    boa_engine::JsValue: for<'a, 'b> CdkActTryFromVmValue<
            Value,
            boa_engine::JsError,
            &'a mut boa_engine::Context<'b>,
        >
        + for<'a, 'b> CdkActTryFromVmValue<
            Box<Value>,
            boa_engine::JsError,
            &'a mut boa_engine::Context<'b>,
        >,
{
    Some(Box<Value>),
    None(()),
}
#[derive(
    serde::Deserialize,
    Debug,
    candid::CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq
)]
enum RejectionCode {
    NoError(()),
    SysFatal(()),
    SysTransient(()),
    DestinationInvalid(()),
    CanisterReject(()),
    CanisterError(()),
    Unknown(()),
}
#[derive(
    serde::Deserialize,
    Debug,
    candid::CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq
)]
enum StableMemoryError {
    OutOfMemory(()),
    OutOfBounds(()),
}
#[derive(
    serde::Deserialize,
    Debug,
    candid::CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq
)]
enum _AzleResult<Ok, Err>
where
    Ok: for<'a, 'b> CdkActTryIntoVmValue<
        &'a mut boa_engine::Context<'b>,
        boa_engine::JsValue,
    >,
    boa_engine::JsValue: for<'a, 'b> CdkActTryFromVmValue<
            Ok,
            boa_engine::JsError,
            &'a mut boa_engine::Context<'b>,
        >
        + for<'a, 'b> CdkActTryFromVmValue<
            Box<Ok>,
            boa_engine::JsError,
            &'a mut boa_engine::Context<'b>,
        >,
    Err: for<'a, 'b> CdkActTryIntoVmValue<
        &'a mut boa_engine::Context<'b>,
        boa_engine::JsValue,
    >,
    boa_engine::JsValue: for<'a, 'b> CdkActTryFromVmValue<
            Err,
            boa_engine::JsError,
            &'a mut boa_engine::Context<'b>,
        >
        + for<'a, 'b> CdkActTryFromVmValue<
            Box<Err>,
            boa_engine::JsError,
            &'a mut boa_engine::Context<'b>,
        >,
{
    Ok(Box<Ok>),
    Err(Box<Err>),
}
candid::export_service!();
#[no_mangle]
pub fn get_candid_pointer() -> *mut std::os::raw::c_char {
    let c_string = std::ffi::CString::new(__export_service()).unwrap();
    c_string.into_raw()
}
#[derive(serde::Deserialize, Clone, Debug, candid::CandidType)]
struct _CdkFloat64(f64);
impl std::cmp::Ord for _CdkFloat64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Less)
    }
}
impl std::cmp::PartialOrd for _CdkFloat64 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl std::cmp::Eq for _CdkFloat64 {}
impl std::cmp::PartialEq for _CdkFloat64 {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
#[derive(serde::Deserialize, Clone, Debug, candid::CandidType)]
struct _CdkFloat32(f32);
impl std::cmp::Ord for _CdkFloat32 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Less)
    }
}
impl std::cmp::PartialOrd for _CdkFloat32 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl std::cmp::Eq for _CdkFloat32 {}
impl std::cmp::PartialEq for _CdkFloat32 {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
