#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool, rustc_private, untagged_unions)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn putchar(__c: libc::c_int) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn iconv_open(
        __tocode: *const libc::c_char,
        __fromcode: *const libc::c_char,
    ) -> iconv_t;
    fn iconv(
        __cd: iconv_t,
        __inbuf: *mut *mut libc::c_char,
        __inbytesleft: *mut size_t,
        __outbuf: *mut *mut libc::c_char,
        __outbytesleft: *mut size_t,
    ) -> size_t;
    fn iconv_close(__cd: iconv_t) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn rand() -> libc::c_int;
    fn abort() -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn abs(_: libc::c_int) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __anonenum_QRencodeMode_707716630 = libc::c_int;
pub const QR_MODE_FNC1SECOND: __anonenum_QRencodeMode_707716630 = 7;
pub const QR_MODE_FNC1FIRST: __anonenum_QRencodeMode_707716630 = 6;
pub const QR_MODE_ECI: __anonenum_QRencodeMode_707716630 = 5;
pub const QR_MODE_STRUCTURE: __anonenum_QRencodeMode_707716630 = 4;
pub const QR_MODE_KANJI: __anonenum_QRencodeMode_707716630 = 3;
pub const QR_MODE_8: __anonenum_QRencodeMode_707716630 = 2;
pub const QR_MODE_AN: __anonenum_QRencodeMode_707716630 = 1;
pub const QR_MODE_NUM: __anonenum_QRencodeMode_707716630 = 0;
pub const QR_MODE_NUL: __anonenum_QRencodeMode_707716630 = -1;
pub type QRencodeMode = __anonenum_QRencodeMode_707716630;
pub type __anonenum_QRecLevel_598008851 = libc::c_uint;
pub const QR_ECLEVEL_H: __anonenum_QRecLevel_598008851 = 3;
pub const QR_ECLEVEL_Q: __anonenum_QRecLevel_598008851 = 2;
pub const QR_ECLEVEL_M: __anonenum_QRecLevel_598008851 = 1;
pub const QR_ECLEVEL_L: __anonenum_QRecLevel_598008851 = 0;
pub type QRecLevel = __anonenum_QRecLevel_598008851;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QRinput {
    pub version: libc::c_int,
    pub level: QRecLevel,
    pub head: *mut QRinput_List,
    pub tail: *mut QRinput_List,
    pub mqr: libc::c_int,
    pub fnc1: libc::c_int,
    pub appid: libc::c_uchar,
}
pub type QRinput_List = _QRinput_List;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QRinput_List {
    pub mode: QRencodeMode,
    pub size: libc::c_int,
    pub data: *mut libc::c_uchar,
    pub bstream: *mut BitStream,
    pub next: *mut QRinput_List,
}
pub type BitStream = __anonstruct_BitStream_209270119;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_BitStream_209270119 {
    pub length: size_t,
    pub datasize: size_t,
    pub data: *mut libc::c_uchar,
}
pub type QRinput = _QRinput;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QRinput_Struct {
    pub size: libc::c_int,
    pub parity: libc::c_int,
    pub head: *mut QRinput_InputList,
    pub tail: *mut QRinput_InputList,
}
pub type QRinput_InputList = _QRinput_InputList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QRinput_InputList {
    pub input: *mut QRinput,
    pub next: *mut QRinput_InputList,
}
pub type QRinput_Struct = _QRinput_Struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_QRcode_929122250 {
    pub version: libc::c_int,
    pub width: libc::c_int,
    pub data: *mut libc::c_uchar,
}
pub type QRcode = __anonstruct_QRcode_929122250;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_RSblock_1044616699 {
    pub dataLength: libc::c_int,
    pub eccLength: libc::c_int,
    pub data: *mut libc::c_uchar,
    pub ecc: *mut libc::c_uchar,
}
pub type RSblock = __anonstruct_RSblock_1044616699;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_QRRawCode_1053786244 {
    pub version: libc::c_int,
    pub dataLength: libc::c_int,
    pub eccLength: libc::c_int,
    pub datacode: *mut libc::c_uchar,
    pub ecccode: *mut libc::c_uchar,
    pub b1: libc::c_int,
    pub blocks: libc::c_int,
    pub rsblock: *mut RSblock,
    pub count: libc::c_int,
}
pub type QRRawCode = __anonstruct_QRRawCode_1053786244;
pub type iconv_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _DataChunk {
    pub mode: QRencodeMode,
    pub size: libc::c_int,
    pub bits: libc::c_int,
    pub data: *mut libc::c_uchar,
    pub next: *mut _DataChunk,
}
pub type DataChunk = _DataChunk;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_QRdata_391029226 {
    pub data: *mut libc::c_uchar,
    pub size: libc::c_int,
    pub mqr: libc::c_int,
    pub version: libc::c_int,
    pub level: QRecLevel,
    pub chunks: *mut DataChunk,
    pub last: *mut DataChunk,
    pub eccResult: libc::c_int,
}
pub type QRdata = __anonstruct_QRdata_391029226;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FormatInfo {
    pub version: libc::c_int,
    pub level: QRecLevel,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_FrameFiller_528872232 {
    pub width: libc::c_int,
    pub frame: *mut libc::c_uchar,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub dir: libc::c_int,
    pub bit: libc::c_int,
    pub mqr: libc::c_int,
}
pub type FrameFiller = __anonstruct_FrameFiller_528872232;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QRcode_List {
    pub code: *mut QRcode,
    pub next: *mut _QRcode_List,
}
pub type QRcode_List = _QRcode_List;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_MQRRawCode_892681476 {
    pub version: libc::c_int,
    pub dataLength: libc::c_int,
    pub eccLength: libc::c_int,
    pub datacode: *mut libc::c_uchar,
    pub ecccode: *mut libc::c_uchar,
    pub rsblock: *mut RSblock,
    pub oddbits: libc::c_int,
    pub count: libc::c_int,
}
pub type MQRRawCode = __anonstruct_MQRRawCode_892681476;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_TestString_48814953 {
    pub str_0: *mut libc::c_char,
    pub version: libc::c_int,
    pub level: QRecLevel,
    pub hint: QRencodeMode,
    pub casesensitive: libc::c_int,
}
pub type TestString = __anonstruct_TestString_48814953;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_QRspec_Capacity_897426155 {
    pub width: libc::c_int,
    pub words: libc::c_int,
    pub remainder: libc::c_int,
    pub ec: [libc::c_int; 4],
}
pub type QRspec_Capacity = __anonstruct_QRspec_Capacity_897426155;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_mutex_t_335460617 {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type pthread_mutex_t = __anonunion_pthread_mutex_t_335460617;
pub type MaskMaker = unsafe extern "C" fn(
    libc::c_int,
    *const libc::c_uchar,
    *mut libc::c_uchar,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_MQRspec_Capacity_991265789 {
    pub width: libc::c_int,
    pub ec: [libc::c_int; 4],
}
pub type MQRspec_Capacity = __anonstruct_MQRspec_Capacity_991265789;
pub type MaskMaker___0 = unsafe extern "C" fn(
    libc::c_int,
    *const libc::c_uchar,
    *mut libc::c_uchar,
) -> ();
static mut tests: libc::c_int = 0 as libc::c_int;
static mut failed: libc::c_int = 0 as libc::c_int;
pub static mut assertionFailed: libc::c_int = 0 as libc::c_int;
pub static mut assertionNum: libc::c_int = 0 as libc::c_int;
static mut testName: *const libc::c_char = 0 as *const libc::c_void as *mut libc::c_void
    as *const libc::c_char;
static mut testFunc: *const libc::c_char = 0 as *const libc::c_void as *mut libc::c_void
    as *const libc::c_char;
pub static mut levelChar: [libc::c_char; 4] = [
    'L' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
];
pub static mut modeStr: [*const libc::c_char; 5] = [
    b"nm\0" as *const u8 as *const libc::c_char,
    b"an\0" as *const u8 as *const libc::c_char,
    b"8\0" as *const u8 as *const libc::c_char,
    b"kj\0" as *const u8 as *const libc::c_char,
    b"st\0" as *const u8 as *const libc::c_char,
];
pub unsafe extern "C" fn ncmpBin(
    mut correct: *mut libc::c_char,
    mut bstream: *mut BitStream,
    mut len: size_t,
) -> libc::c_int {
    let mut bit: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if len != (*bstream).length {
        printf(
            b"Length is not match: %zu, %zu expected.\n\0" as *const u8
                as *const libc::c_char,
            (*bstream).length,
            len,
        );
        return -(1 as libc::c_int);
    }
    p = correct;
    i = 0 as libc::c_int as size_t;
    while *p as libc::c_int != 0 as libc::c_int {
        while *p as libc::c_int == 32 as libc::c_int {
            p = p.offset(1);
        }
        if *p as libc::c_int == 49 as libc::c_int {
            bit = 1 as libc::c_int;
        } else {
            bit = 0 as libc::c_int;
        }
        if *((*bstream).data).offset(i as isize) as libc::c_int != bit {
            return -(1 as libc::c_int);
        }
        i = i.wrapping_add(1);
        p = p.offset(1);
        if i == len {
            break;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn cmpBin(
    mut correct: *mut libc::c_char,
    mut bstream: *mut BitStream,
) -> libc::c_int {
    let mut len: size_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: libc::c_int = 0;
    len = 0 as libc::c_int as size_t;
    p = correct;
    while *p as libc::c_int != 0 as libc::c_int {
        if *p as libc::c_int != 32 as libc::c_int {
            len = len.wrapping_add(1);
        }
        p = p.offset(1);
    }
    tmp = ncmpBin(correct, bstream, len);
    return tmp;
}
pub unsafe extern "C" fn testInit(mut tests___0: libc::c_int) {
    printf(b"1..%d\n\0" as *const u8 as *const libc::c_char, tests___0);
}
pub unsafe extern "C" fn testStartReal(
    mut func: *const libc::c_char,
    mut name: *const libc::c_char,
) {
    tests += 1;
    testName = name;
    testFunc = func;
    assertionFailed = 0 as libc::c_int;
    assertionNum = 0 as libc::c_int;
}
pub unsafe extern "C" fn testEnd(mut result: libc::c_int) {
    if result != 0 {
        printf(
            b"not ok %d %s: %s\n\0" as *const u8 as *const libc::c_char,
            tests,
            testFunc,
            testName,
        );
        failed += 1;
    } else {
        printf(
            b"ok %d %s: %s\n\0" as *const u8 as *const libc::c_char,
            tests,
            testFunc,
            testName,
        );
    };
}
pub unsafe extern "C" fn testFinish() {
    if assertionFailed != 0 {
        printf(
            b"not ok %d %s: %s (%d assertions failed.)\n\0" as *const u8
                as *const libc::c_char,
            tests,
            testFunc,
            testName,
            assertionFailed,
        );
        failed += 1;
    } else {
        printf(
            b"ok %d %s: %s (%d assertions passed.)\n\0" as *const u8
                as *const libc::c_char,
            tests,
            testFunc,
            testName,
            assertionNum,
        );
    };
}
pub unsafe extern "C" fn testReport(mut expectedTests: libc::c_int) {
    printf(
        b"Total %d tests, %d fails.\n\0" as *const u8 as *const libc::c_char,
        tests,
        failed,
    );
    if failed != 0 {
        exit(-(1 as libc::c_int));
    }
    if expectedTests != tests {
        printf(
            b"WARNING: the number of the executed tests (%d) is not equal to the expecetd (%d).\n\0"
                as *const u8 as *const libc::c_char,
            tests,
            expectedTests,
        );
    }
}
pub unsafe extern "C" fn testNum() -> libc::c_int {
    return tests;
}
pub unsafe extern "C" fn printBinary(
    mut data: *mut libc::c_uchar,
    mut length: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    i = 0 as libc::c_int;
    while i < length {
        if *data.offset(i as isize) != 0 {
            tmp = b"1\0" as *const u8 as *const libc::c_char;
        } else {
            tmp = b"0\0" as *const u8 as *const libc::c_char;
        }
        printf(tmp);
        i += 1;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn printBstream(mut bstream: *mut BitStream) {
    printBinary((*bstream).data, (*bstream).length as libc::c_int);
}
pub unsafe extern "C" fn printQRinput(mut input: *mut QRinput) {
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut i: libc::c_int = 0;
    list = (*input).head;
    while list as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        i = 0 as libc::c_int;
        while i < (*list).size {
            printf(
                b"0x%02x,\0" as *const u8 as *const libc::c_char,
                *((*list).data).offset(i as isize) as libc::c_int,
            );
            i += 1;
        }
        list = (*list).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn printQRinputInfo(mut input: *mut QRinput) {
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut b: *mut BitStream = 0 as *mut BitStream;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    printf(b"QRinput info:\n\0" as *const u8 as *const libc::c_char);
    printf(b" version: %d\n\0" as *const u8 as *const libc::c_char, (*input).version);
    printf(
        b" level  : %c\n\0" as *const u8 as *const libc::c_char,
        levelChar[(*input).level as usize] as libc::c_int,
    );
    list = (*input).head;
    i = 0 as libc::c_int;
    while list as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        i += 1;
        list = (*list).next;
    }
    printf(b"  chunks: %d\n\0" as *const u8 as *const libc::c_char, i);
    b = BitStream_new();
    ret = QRinput_mergeBitStream(input, b);
    if ret == 0 as libc::c_int {
        printf(
            b"  bitstream-size: %zu\n\0" as *const u8 as *const libc::c_char,
            (*b).length,
        );
        BitStream_free(b);
    }
    list = (*input).head;
    i = 0 as libc::c_int;
    while list as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        printf(
            b"\t#%d: mode = %s, size = %d\n\0" as *const u8 as *const libc::c_char,
            i,
            modeStr[(*list).mode as usize],
            (*list).size,
        );
        i += 1;
        list = (*list).next;
    }
}
pub unsafe extern "C" fn printQRinputStruct(mut s: *mut QRinput_Struct) {
    let mut list: *mut QRinput_InputList = 0 as *mut QRinput_InputList;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    printf(b"Struct size: %d\n\0" as *const u8 as *const libc::c_char, (*s).size);
    printf(b"Struct parity: %08x\n\0" as *const u8 as *const libc::c_char, (*s).parity);
    list = (*s).head;
    while list as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        printf(b"Symbol %d - \0" as *const u8 as *const libc::c_char, i);
        printQRinputInfo((*list).input);
        i += 1;
        list = (*list).next;
    }
}
pub unsafe extern "C" fn printFrame(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut tmp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    y = 0 as libc::c_int;
    while y < width {
        x = 0 as libc::c_int;
        while x < width {
            tmp = frame;
            frame = frame.offset(1);
            printf(b"%02x \0" as *const u8 as *const libc::c_char, *tmp as libc::c_int);
            x += 1;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        y += 1;
    }
}
pub unsafe extern "C" fn printQRcode(mut code: *mut QRcode) {
    printFrame((*code).width, (*code).data);
}
pub unsafe extern "C" fn printQRRawCodeFromQRinput(mut input: *mut QRinput) {
    let mut raw: *mut QRRawCode = 0 as *mut QRRawCode;
    let mut i: libc::c_int = 0;
    puts(b"QRRawCode dump image:\0" as *const u8 as *const libc::c_char);
    raw = QRraw_new(input);
    if raw as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        puts(
            b"Failed to generate QRRawCode from this input.\n\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    i = 0 as libc::c_int;
    while i < (*raw).dataLength {
        printf(
            b" %02x\0" as *const u8 as *const libc::c_char,
            *((*raw).datacode).offset(i as isize) as libc::c_int,
        );
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*raw).eccLength {
        printf(
            b" %02x\0" as *const u8 as *const libc::c_char,
            *((*raw).ecccode).offset(i as isize) as libc::c_int,
        );
        i += 1;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    QRraw_free(raw);
}
pub unsafe extern "C" fn show_QRcode(mut qrcode: *mut QRcode) {}
pub unsafe extern "C" fn DataChunk_new(mut mode: QRencodeMode) -> *mut DataChunk {
    let mut chunk: *mut DataChunk = 0 as *mut DataChunk;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<DataChunk>() as libc::c_ulong,
    );
    chunk = tmp as *mut DataChunk;
    if chunk as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut DataChunk;
    }
    (*chunk).mode = mode;
    return chunk;
}
pub unsafe extern "C" fn DataChunk_free(mut chunk: *mut DataChunk) {
    if !chunk.is_null() {
        if !((*chunk).data).is_null() {
            free((*chunk).data as *mut libc::c_void);
        }
        free(chunk as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn DataChunk_freeList(mut list: *mut DataChunk) {
    let mut next: *mut DataChunk = 0 as *mut DataChunk;
    while list as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        next = (*list).next;
        DataChunk_free(list);
        list = next;
    }
}
unsafe extern "C" fn dumpNum(mut chunk: *mut DataChunk) {
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, (*chunk).data);
}
unsafe extern "C" fn dumpAn(mut chunk: *mut DataChunk) {
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, (*chunk).data);
}
unsafe extern "C" fn dump8(mut chunk: *mut DataChunk) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut c: libc::c_uchar = 0;
    let mut count: libc::c_int = 0;
    let mut buf: [libc::c_uchar; 16] = [0; 16];
    count = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*chunk).size {
        buf[count as usize] = *((*chunk).data).offset(i as isize);
        c = *((*chunk).data).offset(i as isize);
        if c as libc::c_int >= 32 as libc::c_int {
            if c as libc::c_int <= 126 as libc::c_int {
                putchar(c as libc::c_int);
            } else {
                putchar('.' as i32);
            }
        } else {
            putchar('.' as i32);
        }
        count += 1;
        if count >= 16 as libc::c_int {
            putchar(' ' as i32);
            j = 0 as libc::c_int;
            while j < 16 as libc::c_int {
                printf(
                    b" %02x\0" as *const u8 as *const libc::c_char,
                    buf[j as usize] as libc::c_int,
                );
                j += 1;
            }
            count = 0 as libc::c_int;
            putchar('\n' as i32);
        }
        i += 1;
    }
    if count > 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 16 as libc::c_int - count {
            putchar(' ' as i32);
            i += 1;
        }
        putchar(' ' as i32);
        j = 0 as libc::c_int;
        while j < count {
            printf(
                b" %02x\0" as *const u8 as *const libc::c_char,
                buf[j as usize] as libc::c_int,
            );
            j += 1;
        }
        count = 0 as libc::c_int;
        putchar('\n' as i32);
    }
}
unsafe extern "C" fn dumpKanji(mut chunk: *mut DataChunk) {
    let mut conv: iconv_t = 0 as *mut libc::c_void;
    let mut inbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut inbytes: size_t = 0;
    let mut outbytes: size_t = 0;
    let mut ret: size_t = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    conv = iconv_open(
        b"UTF-8\0" as *const u8 as *const libc::c_char,
        b"SHIFT_JIS\0" as *const u8 as *const libc::c_char,
    );
    inbytes = (*chunk).size as size_t;
    inbuf = (*chunk).data as *mut libc::c_char;
    outbytes = inbytes.wrapping_mul(4 as libc::c_ulong).wrapping_add(1 as libc::c_ulong);
    tmp = malloc(
        inbytes.wrapping_mul(4 as libc::c_ulong).wrapping_add(1 as libc::c_ulong),
    );
    outbuf = tmp as *mut libc::c_char;
    outp = outbuf;
    ret = iconv(
        conv,
        &mut inbuf as *mut *mut libc::c_char,
        &mut inbytes as *mut size_t,
        &mut outp as *mut *mut libc::c_char,
        &mut outbytes as *mut size_t,
    );
    if ret == 0xffffffffffffffff as libc::c_ulong {
        perror(0 as *mut libc::c_void as *const libc::c_char);
    }
    *outp = '\u{0}' as i32 as libc::c_char;
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, outbuf);
    iconv_close(conv);
    free(outbuf as *mut libc::c_void);
}
unsafe extern "C" fn dumpChunk(mut chunk: *mut DataChunk) {
    match (*chunk).mode as libc::c_int {
        0 => {
            printf(
                b"Numeric: %d bytes\n\0" as *const u8 as *const libc::c_char,
                (*chunk).size,
            );
            dumpNum(chunk);
        }
        1 => {
            printf(
                b"AlphaNumeric: %d bytes\n\0" as *const u8 as *const libc::c_char,
                (*chunk).size,
            );
            dumpAn(chunk);
        }
        2 => {
            printf(
                b"8-bit data: %d bytes\n\0" as *const u8 as *const libc::c_char,
                (*chunk).size,
            );
            dump8(chunk);
        }
        3 => {
            printf(
                b"Kanji: %d bytes\n\0" as *const u8 as *const libc::c_char,
                (*chunk).size,
            );
            dumpKanji(chunk);
        }
        _ => {
            printf(
                b"Invalid or reserved: %d bytes\n\0" as *const u8 as *const libc::c_char,
                (*chunk).size,
            );
            dump8(chunk);
        }
    };
}
pub unsafe extern "C" fn DataChunk_dumpChunkList(mut list: *mut DataChunk) {
    while list as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        dumpChunk(list);
        list = (*list).next;
    }
}
pub unsafe extern "C" fn DataChunk_totalSize(mut list: *mut DataChunk) -> libc::c_int {
    let mut size: libc::c_int = 0;
    size = 0 as libc::c_int;
    while list as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        size += (*list).size;
        list = (*list).next;
    }
    return size;
}
pub unsafe extern "C" fn DataChunk_concatChunkList(
    mut list: *mut DataChunk,
    mut retsize: *mut libc::c_int,
) -> *mut libc::c_uchar {
    let mut size: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    size = DataChunk_totalSize(list);
    if size <= 0 as libc::c_int {
        return 0 as *mut libc::c_void as *mut libc::c_uchar;
    }
    tmp = malloc((size + 1 as libc::c_int) as size_t);
    data = tmp as *mut libc::c_uchar;
    idx = 0 as libc::c_int;
    while list as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        memcpy(
            data.offset(idx as isize) as *mut libc::c_void,
            (*list).data as *const libc::c_void,
            (*list).size as size_t,
        );
        idx += (*list).size;
        list = (*list).next;
    }
    *data.offset(size as isize) = '\u{0}' as i32 as libc::c_uchar;
    *retsize = size;
    return data;
}
unsafe extern "C" fn bitToInt(
    mut bits: *mut libc::c_uchar,
    mut length: libc::c_int,
) -> libc::c_uint {
    let mut i: libc::c_int = 0;
    let mut val: libc::c_uint = 0;
    val = 0 as libc::c_uint;
    i = 0 as libc::c_int;
    while i < length {
        val <<= 1 as libc::c_int;
        val
            |= (*bits.offset(i as isize) as libc::c_int & 1 as libc::c_int)
                as libc::c_uint;
        i += 1;
    }
    return val;
}
unsafe extern "C" fn decodeLength(
    mut bits_length: *mut libc::c_int,
    mut bits: *mut *mut libc::c_uchar,
    mut mode: QRencodeMode,
    mut version: libc::c_int,
    mut mqr: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut lbits: libc::c_int = 0;
    length = 0 as libc::c_int;
    if mqr != 0 {
        lbits = MQRspec_lengthIndicator(mode, version);
    } else {
        lbits = QRspec_lengthIndicator(mode, version);
    }
    if *bits_length < lbits {
        printf(
            b"Bit length is too short: %d\n\0" as *const u8 as *const libc::c_char,
            *bits_length,
        );
        return 0 as libc::c_int;
    }
    length = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < lbits {
        length <<= 1 as libc::c_int;
        length += *(*bits).offset(i as isize) as libc::c_int;
        i += 1;
    }
    *bits_length -= lbits;
    *bits = (*bits).offset(lbits as isize);
    return length;
}
unsafe extern "C" fn decodeNum(
    mut bits_length: *mut libc::c_int,
    mut bits: *mut *mut libc::c_uchar,
    mut version: libc::c_int,
    mut mqr: libc::c_int,
) -> *mut DataChunk {
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut sizeInBit: libc::c_int = 0;
    let mut words: libc::c_int = 0;
    let mut remain: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: libc::c_uint = 0;
    let mut chunk: *mut DataChunk = 0 as *mut DataChunk;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    size = decodeLength(bits_length, bits, QR_MODE_NUM, version, mqr);
    if size < 0 as libc::c_int {
        return 0 as *mut libc::c_void as *mut DataChunk;
    }
    words = size / 3 as libc::c_int;
    remain = size - words * 3 as libc::c_int;
    sizeInBit = words * 10 as libc::c_int;
    if remain == 2 as libc::c_int {
        sizeInBit += 7 as libc::c_int;
    } else if remain == 1 as libc::c_int {
        sizeInBit += 4 as libc::c_int;
    }
    if *bits_length < sizeInBit {
        printf(
            b"Bit length is too short: %d, expected %d.\n\0" as *const u8
                as *const libc::c_char,
            *bits_length,
            sizeInBit,
        );
        return 0 as *mut libc::c_void as *mut DataChunk;
    }
    tmp = malloc((size as size_t).wrapping_add(1 as libc::c_ulong));
    buf = tmp as *mut libc::c_char;
    p = *bits;
    q = buf;
    i = 0 as libc::c_int;
    while i < words {
        val = bitToInt(p, 10 as libc::c_int);
        sprintf(q, b"%03d\0" as *const u8 as *const libc::c_char, val);
        p = p.offset(10 as libc::c_int as isize);
        q = q.offset(3 as libc::c_int as isize);
        i += 1;
    }
    if remain == 2 as libc::c_int {
        val = bitToInt(p, 7 as libc::c_int);
        sprintf(q, b"%02d\0" as *const u8 as *const libc::c_char, val);
    } else if remain == 1 as libc::c_int {
        val = bitToInt(p, 4 as libc::c_int);
        sprintf(q, b"%1d\0" as *const u8 as *const libc::c_char, val);
    }
    *buf.offset(size as isize) = '\u{0}' as i32 as libc::c_char;
    chunk = DataChunk_new(QR_MODE_NUM);
    (*chunk).size = size;
    (*chunk).data = buf as *mut libc::c_uchar;
    *bits_length -= sizeInBit;
    *bits = (*bits).offset(sizeInBit as isize);
    return chunk;
}
static mut decodeAnTable: [libc::c_char; 45] = [
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '$' as i32 as libc::c_char,
    '%' as i32 as libc::c_char,
    '*' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    ':' as i32 as libc::c_char,
];
unsafe extern "C" fn decodeAn(
    mut bits_length: *mut libc::c_int,
    mut bits: *mut *mut libc::c_uchar,
    mut version: libc::c_int,
    mut mqr: libc::c_int,
) -> *mut DataChunk {
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut sizeInBit: libc::c_int = 0;
    let mut words: libc::c_int = 0;
    let mut remain: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: libc::c_uint = 0;
    let mut ch: libc::c_int = 0;
    let mut cl: libc::c_int = 0;
    let mut chunk: *mut DataChunk = 0 as *mut DataChunk;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    size = decodeLength(bits_length, bits, QR_MODE_AN, version, mqr);
    if size < 0 as libc::c_int {
        return 0 as *mut libc::c_void as *mut DataChunk;
    }
    words = size / 2 as libc::c_int;
    remain = size - words * 2 as libc::c_int;
    sizeInBit = words * 11 as libc::c_int + remain * 6 as libc::c_int;
    if *bits_length < sizeInBit {
        printf(
            b"Bit length is too short: %d, expected %d.\n\0" as *const u8
                as *const libc::c_char,
            *bits_length,
            sizeInBit,
        );
        return 0 as *mut libc::c_void as *mut DataChunk;
    }
    tmp = malloc((size as size_t).wrapping_add(1 as libc::c_ulong));
    buf = tmp as *mut libc::c_char;
    p = *bits;
    q = buf;
    i = 0 as libc::c_int;
    while i < words {
        val = bitToInt(p, 11 as libc::c_int);
        ch = val.wrapping_div(45 as libc::c_uint) as libc::c_int;
        cl = val.wrapping_rem(45 as libc::c_uint) as libc::c_int;
        sprintf(
            q,
            b"%c%c\0" as *const u8 as *const libc::c_char,
            decodeAnTable[ch as usize] as libc::c_int,
            decodeAnTable[cl as usize] as libc::c_int,
        );
        p = p.offset(11 as libc::c_int as isize);
        q = q.offset(2 as libc::c_int as isize);
        i += 1;
    }
    if remain == 1 as libc::c_int {
        val = bitToInt(p, 6 as libc::c_int);
        sprintf(
            q,
            b"%c\0" as *const u8 as *const libc::c_char,
            decodeAnTable[val as usize] as libc::c_int,
        );
    }
    chunk = DataChunk_new(QR_MODE_AN);
    (*chunk).size = size;
    (*chunk).data = buf as *mut libc::c_uchar;
    *bits_length -= sizeInBit;
    *bits = (*bits).offset(sizeInBit as isize);
    return chunk;
}
unsafe extern "C" fn decode8(
    mut bits_length: *mut libc::c_int,
    mut bits: *mut *mut libc::c_uchar,
    mut version: libc::c_int,
    mut mqr: libc::c_int,
) -> *mut DataChunk {
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut sizeInBit: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut q: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut chunk: *mut DataChunk = 0 as *mut DataChunk;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_uint = 0;
    size = decodeLength(bits_length, bits, QR_MODE_8, version, mqr);
    if size < 0 as libc::c_int {
        return 0 as *mut libc::c_void as *mut DataChunk;
    }
    sizeInBit = size * 8 as libc::c_int;
    if *bits_length < sizeInBit {
        printf(
            b"Bit length is too short: %d, expected %d.\n\0" as *const u8
                as *const libc::c_char,
            *bits_length,
            sizeInBit,
        );
        return 0 as *mut libc::c_void as *mut DataChunk;
    }
    tmp = malloc(size as size_t);
    buf = tmp as *mut libc::c_uchar;
    p = *bits;
    q = buf;
    i = 0 as libc::c_int;
    while i < size {
        tmp___0 = bitToInt(p, 8 as libc::c_int);
        *q = tmp___0 as libc::c_uchar;
        p = p.offset(8 as libc::c_int as isize);
        q = q.offset(1);
        i += 1;
    }
    chunk = DataChunk_new(QR_MODE_8);
    (*chunk).size = size;
    (*chunk).data = buf;
    *bits_length -= sizeInBit;
    *bits = (*bits).offset(sizeInBit as isize);
    return chunk;
}
unsafe extern "C" fn decodeKanji(
    mut bits_length: *mut libc::c_int,
    mut bits: *mut *mut libc::c_uchar,
    mut version: libc::c_int,
    mut mqr: libc::c_int,
) -> *mut DataChunk {
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut sizeInBit: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: libc::c_uint = 0;
    let mut ch: libc::c_uint = 0;
    let mut cl: libc::c_uint = 0;
    let mut chunk: *mut DataChunk = 0 as *mut DataChunk;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    size = decodeLength(bits_length, bits, QR_MODE_KANJI, version, mqr);
    if size < 0 as libc::c_int {
        return 0 as *mut libc::c_void as *mut DataChunk;
    }
    sizeInBit = size * 13 as libc::c_int;
    if *bits_length < sizeInBit {
        printf(
            b"Bit length is too short: %d, expected %d.\n\0" as *const u8
                as *const libc::c_char,
            *bits_length,
            sizeInBit,
        );
        return 0 as *mut libc::c_void as *mut DataChunk;
    }
    tmp = malloc(
        (size as size_t)
            .wrapping_mul(2 as libc::c_ulong)
            .wrapping_add(1 as libc::c_ulong),
    );
    buf = tmp as *mut libc::c_char;
    p = *bits;
    q = buf;
    i = 0 as libc::c_int;
    while i < size {
        val = bitToInt(p, 13 as libc::c_int);
        ch = val.wrapping_div(192 as libc::c_uint);
        cl = val.wrapping_sub(ch.wrapping_mul(192 as libc::c_uint));
        val = ch.wrapping_mul(256 as libc::c_uint).wrapping_add(cl);
        if val >= 7936 as libc::c_uint {
            val = val.wrapping_add(49472 as libc::c_uint);
        } else {
            val = val.wrapping_add(33088 as libc::c_uint);
        }
        sprintf(
            q,
            b"%c%c\0" as *const u8 as *const libc::c_char,
            val >> 8 as libc::c_int & 255 as libc::c_uint,
            val & 255 as libc::c_uint,
        );
        p = p.offset(13 as libc::c_int as isize);
        q = q.offset(2 as libc::c_int as isize);
        i += 1;
    }
    chunk = DataChunk_new(QR_MODE_KANJI);
    (*chunk).size = size * 2 as libc::c_int;
    (*chunk).data = buf as *mut libc::c_uchar;
    *bits_length -= sizeInBit;
    *bits = (*bits).offset(sizeInBit as isize);
    return chunk;
}
unsafe extern "C" fn decodeChunk(
    mut bits_length: *mut libc::c_int,
    mut bits: *mut *mut libc::c_uchar,
    mut version: libc::c_int,
) -> *mut DataChunk {
    let mut val: libc::c_uint = 0;
    let mut tmp: *mut DataChunk = 0 as *mut DataChunk;
    let mut tmp___0: *mut DataChunk = 0 as *mut DataChunk;
    let mut tmp___1: *mut DataChunk = 0 as *mut DataChunk;
    let mut tmp___2: *mut DataChunk = 0 as *mut DataChunk;
    if *bits_length < 4 as libc::c_int {
        return 0 as *mut libc::c_void as *mut DataChunk;
    }
    val = bitToInt(*bits, 4 as libc::c_int);
    *bits_length -= 4 as libc::c_int;
    *bits = (*bits).offset(4 as libc::c_int as isize);
    match val {
        0 => return 0 as *mut libc::c_void as *mut DataChunk,
        1 => {
            tmp = decodeNum(bits_length, bits, version, 0 as libc::c_int);
            return tmp;
        }
        2 => {
            tmp___0 = decodeAn(bits_length, bits, version, 0 as libc::c_int);
            return tmp___0;
        }
        4 => {
            tmp___1 = decode8(bits_length, bits, version, 0 as libc::c_int);
            return tmp___1;
        }
        8 => {
            tmp___2 = decodeKanji(bits_length, bits, version, 0 as libc::c_int);
            return tmp___2;
        }
        _ => {}
    }
    printf(b"Invalid mode in a chunk: %d\n\0" as *const u8 as *const libc::c_char, val);
    return 0 as *mut libc::c_void as *mut DataChunk;
}
unsafe extern "C" fn decodeChunkMQR(
    mut bits_length: *mut libc::c_int,
    mut bits: *mut *mut libc::c_uchar,
    mut version: libc::c_int,
) -> *mut DataChunk {
    let mut modebits: libc::c_int = 0;
    let mut termbits: libc::c_int = 0;
    let mut val: libc::c_uint = 0;
    let mut tmp: *mut DataChunk = 0 as *mut DataChunk;
    let mut tmp___0: *mut DataChunk = 0 as *mut DataChunk;
    let mut tmp___1: *mut DataChunk = 0 as *mut DataChunk;
    let mut tmp___2: *mut DataChunk = 0 as *mut DataChunk;
    modebits = version - 1 as libc::c_int;
    termbits = version * 2 as libc::c_int + 1 as libc::c_int;
    if *bits_length >= termbits {
        val = bitToInt(*bits, termbits);
        if val == 0 as libc::c_uint {
            *bits = (*bits).offset(termbits as isize);
            *bits_length -= termbits;
            return 0 as *mut libc::c_void as *mut DataChunk;
        }
    } else {
        if *bits_length < modebits {
            val = bitToInt(*bits, *bits_length);
        } else {
            val = bitToInt(*bits, modebits);
        }
        if val == 0 as libc::c_uint {
            return 0 as *mut libc::c_void as *mut DataChunk
        } else {
            printf(
                b"Terminating bits include 1-bit.\n\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as *mut libc::c_void as *mut DataChunk;
        }
    }
    val = bitToInt(*bits, modebits);
    if version == 4 as libc::c_int {
        if val > 3 as libc::c_uint {
            printf(
                b"Invalid mode number %d.\n\0" as *const u8 as *const libc::c_char,
                val,
            );
        }
    }
    *bits_length -= modebits;
    *bits = (*bits).offset(modebits as isize);
    match val {
        0 => {
            tmp = decodeNum(bits_length, bits, version, 1 as libc::c_int);
            return tmp;
        }
        1 => {
            tmp___0 = decodeAn(bits_length, bits, version, 1 as libc::c_int);
            return tmp___0;
        }
        2 => {
            tmp___1 = decode8(bits_length, bits, version, 1 as libc::c_int);
            return tmp___1;
        }
        3 => {
            tmp___2 = decodeKanji(bits_length, bits, version, 1 as libc::c_int);
            return tmp___2;
        }
        _ => {}
    }
    printf(b"Invalid mode in a chunk: %d\n\0" as *const u8 as *const libc::c_char, val);
    return 0 as *mut libc::c_void as *mut DataChunk;
}
unsafe extern "C" fn appendChunk(
    mut qrdata: *mut QRdata,
    mut bits_length: *mut libc::c_int,
    mut bits: *mut *mut libc::c_uchar,
) -> libc::c_int {
    let mut chunk: *mut DataChunk = 0 as *mut DataChunk;
    if (*qrdata).mqr != 0 {
        chunk = decodeChunkMQR(bits_length, bits, (*qrdata).version);
    } else {
        chunk = decodeChunk(bits_length, bits, (*qrdata).version);
    }
    if chunk as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 1 as libc::c_int;
    }
    if (*qrdata).last as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        (*qrdata).chunks = chunk;
    } else {
        (*(*qrdata).last).next = chunk;
    }
    (*qrdata).last = chunk;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRdata_new() -> *mut QRdata {
    let mut qrdata: *mut QRdata = 0 as *mut QRdata;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        ::std::mem::size_of::<QRdata>() as libc::c_ulong,
        1 as libc::c_int as size_t,
    );
    qrdata = tmp as *mut QRdata;
    if qrdata as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut QRdata;
    }
    (*qrdata).eccResult = 0 as libc::c_int;
    return qrdata;
}
pub unsafe extern "C" fn QRdata_newMQR() -> *mut QRdata {
    let mut qrdata: *mut QRdata = 0 as *mut QRdata;
    qrdata = QRdata_new();
    if qrdata as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut QRdata;
    }
    (*qrdata).mqr = 1 as libc::c_int;
    return qrdata;
}
pub unsafe extern "C" fn QRdata_free(mut qrdata: *mut QRdata) {
    DataChunk_freeList((*qrdata).chunks);
    if (*qrdata).data as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        free((*qrdata).data as *mut libc::c_void);
    }
    free(qrdata as *mut libc::c_void);
}
unsafe extern "C" fn QRdata_decodeBits(
    mut qrdata: *mut QRdata,
    mut length: libc::c_int,
    mut bits: *mut libc::c_uchar,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = 0 as libc::c_int;
    while ret == 0 as libc::c_int {
        ret = appendChunk(qrdata, &mut length, &mut bits);
    }
    return length;
}
pub unsafe extern "C" fn QRdata_decodeBitStream(
    mut qrdata: *mut QRdata,
    mut bstream: *mut BitStream,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = QRdata_decodeBits(qrdata, (*bstream).length as libc::c_int, (*bstream).data);
    return tmp;
}
pub unsafe extern "C" fn QRdata_dump(mut data: *mut QRdata) {
    DataChunk_dumpChunkList((*data).chunks);
}
pub unsafe extern "C" fn QRcode_decodeVersion(mut code: *mut QRcode) -> libc::c_int {
    let mut v1: libc::c_uint = 0;
    let mut v2: libc::c_uint = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    width = (*code).width;
    if width < 45 as libc::c_int {
        return (width - 21 as libc::c_int) / 4 as libc::c_int + 1 as libc::c_int;
    }
    v1 = 0 as libc::c_uint;
    p = ((*code).data)
        .offset((width * (width - 9 as libc::c_int)) as isize)
        .offset(5 as libc::c_int as isize);
    x = 0 as libc::c_int;
    while x < 6 as libc::c_int {
        y = 0 as libc::c_int;
        while y < 3 as libc::c_int {
            v1 <<= 1 as libc::c_int;
            v1
                |= (*p.offset(-((y * width) as isize)).offset(-(x as isize))
                    as libc::c_int & 1 as libc::c_int) as libc::c_uint;
            y += 1;
        }
        x += 1;
    }
    v2 = 0 as libc::c_uint;
    p = ((*code).data)
        .offset((width * 5 as libc::c_int) as isize)
        .offset(width as isize)
        .offset(-(9 as libc::c_int as isize));
    y = 0 as libc::c_int;
    while y < 6 as libc::c_int {
        x = 0 as libc::c_int;
        while x < 3 as libc::c_int {
            v2 <<= 1 as libc::c_int;
            v2
                |= (*p.offset(-((y * width) as isize)).offset(-(x as isize))
                    as libc::c_int & 1 as libc::c_int) as libc::c_uint;
            x += 1;
        }
        y += 1;
    }
    if v1 != v2 {
        printf(
            b"Two verion patterns are different.\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return (v1 >> 12 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn QRcode_decodeFormat(
    mut code: *mut QRcode,
    mut level: *mut QRecLevel,
    mut mask: *mut libc::c_int,
) -> libc::c_int {
    let mut v1: libc::c_uint = 0;
    let mut v2: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    width = (*code).width;
    v1 = 0 as libc::c_uint;
    p = ((*code).data).offset((width * 8 as libc::c_int) as isize);
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        v1 <<= 1 as libc::c_int;
        if i < 6 as libc::c_int {
            v1
                |= (*p.offset(i as isize) as libc::c_int & 1 as libc::c_int)
                    as libc::c_uint;
        } else {
            v1
                |= (*p.offset(i as isize).offset(1 as libc::c_int as isize)
                    as libc::c_int & 1 as libc::c_int) as libc::c_uint;
        }
        i += 1;
    }
    p = ((*code).data)
        .offset((width * 7 as libc::c_int) as isize)
        .offset(8 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        v1 <<= 1 as libc::c_int;
        if i < 1 as libc::c_int {
            v1
                |= (*p.offset(-((width * i) as isize)) as libc::c_int & 1 as libc::c_int)
                    as libc::c_uint;
        } else {
            v1
                |= (*p.offset(-((width * (i + 1 as libc::c_int)) as isize))
                    as libc::c_int & 1 as libc::c_int) as libc::c_uint;
        }
        i += 1;
    }
    v2 = 0 as libc::c_uint;
    p = ((*code).data)
        .offset((width * (width - 1 as libc::c_int)) as isize)
        .offset(8 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        v2 <<= 1 as libc::c_int;
        v2
            |= (*p.offset(-((width * i) as isize)) as libc::c_int & 1 as libc::c_int)
                as libc::c_uint;
        i += 1;
    }
    p = ((*code).data)
        .offset((width * 8 as libc::c_int) as isize)
        .offset(width as isize)
        .offset(-(8 as libc::c_int as isize));
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        v2 <<= 1 as libc::c_int;
        v2 |= (*p.offset(i as isize) as libc::c_int & 1 as libc::c_int) as libc::c_uint;
        i += 1;
    }
    if v1 != v2 {
        printf(
            b"Two format infos are different.\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    v1 = (v1 ^ 21522 as libc::c_uint) >> 10 as libc::c_int;
    *mask = (v1 & 7 as libc::c_uint) as libc::c_int;
    match v1 >> 3 as libc::c_int & 3 as libc::c_uint {
        1 => {
            *level = QR_ECLEVEL_L;
        }
        0 => {
            *level = QR_ECLEVEL_M;
        }
        3 => {
            *level = QR_ECLEVEL_Q;
        }
        2 => {
            *level = QR_ECLEVEL_H;
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn unmask(
    mut code: *mut QRcode,
    mut level: QRecLevel,
    mut mask: libc::c_int,
) -> *mut libc::c_uchar {
    let mut unmasked: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    unmasked = Mask_makeMask((*code).width, (*code).data, mask, level);
    return unmasked;
}
pub unsafe extern "C" fn QRcode_unmask(mut code: *mut QRcode) -> *mut libc::c_uchar {
    let mut ret: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut level: QRecLevel = QR_ECLEVEL_L;
    let mut tmp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    version = QRcode_decodeVersion(code);
    if version < 1 as libc::c_int {
        return 0 as *mut libc::c_void as *mut libc::c_uchar;
    }
    ret = QRcode_decodeFormat(code, &mut level, &mut mask);
    if ret < 0 as libc::c_int {
        return 0 as *mut libc::c_void as *mut libc::c_uchar;
    }
    tmp = unmask(code, level, mask);
    return tmp;
}
unsafe extern "C" fn FrameFiller_new(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut mqr: libc::c_int,
) -> *mut FrameFiller {
    let mut filler: *mut FrameFiller = 0 as *mut FrameFiller;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<FrameFiller>() as libc::c_ulong);
    filler = tmp as *mut FrameFiller;
    if filler as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut FrameFiller;
    }
    (*filler).width = width;
    (*filler).frame = frame;
    (*filler).x = width - 1 as libc::c_int;
    (*filler).y = width - 1 as libc::c_int;
    (*filler).dir = -(1 as libc::c_int);
    (*filler).bit = -(1 as libc::c_int);
    (*filler).mqr = mqr;
    return filler;
}
unsafe extern "C" fn FrameFiller_next(
    mut filler: *mut FrameFiller,
) -> *mut libc::c_uchar {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut tmp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if (*filler).bit == -(1 as libc::c_int) {
        (*filler).bit = 0 as libc::c_int;
        return ((*filler).frame)
            .offset(((*filler).y * (*filler).width) as isize)
            .offset((*filler).x as isize);
    }
    x = (*filler).x;
    y = (*filler).y;
    p = (*filler).frame;
    w = (*filler).width;
    if (*filler).bit == 0 as libc::c_int {
        x -= 1;
        (*filler).bit += 1;
    } else {
        x += 1;
        y += (*filler).dir;
        (*filler).bit -= 1;
    }
    if (*filler).dir < 0 as libc::c_int {
        if y < 0 as libc::c_int {
            y = 0 as libc::c_int;
            x -= 2 as libc::c_int;
            (*filler).dir = 1 as libc::c_int;
            if (*filler).mqr == 0 {
                if x == 6 as libc::c_int {
                    x -= 1;
                    y = 9 as libc::c_int;
                }
            }
        }
    } else if y == w {
        y = w - 1 as libc::c_int;
        x -= 2 as libc::c_int;
        (*filler).dir = -(1 as libc::c_int);
        if (*filler).mqr == 0 {
            if x == 6 as libc::c_int {
                x -= 1;
                y -= 8 as libc::c_int;
            }
        }
    }
    if x < 0 as libc::c_int {
        return 0 as *mut libc::c_void as *mut libc::c_uchar
    } else {
        if y < 0 as libc::c_int {
            return 0 as *mut libc::c_void as *mut libc::c_uchar;
        }
    }
    (*filler).x = x;
    (*filler).y = y;
    if *p.offset((y * w + x) as isize) as libc::c_int & 128 as libc::c_int != 0 {
        tmp = FrameFiller_next(filler);
        return tmp;
    }
    return p.offset((y * w + x) as isize);
}
unsafe extern "C" fn extractBits(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut spec: *mut libc::c_int,
) -> *mut BitStream {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut bits: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut q: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut filler: *mut FrameFiller = 0 as *mut FrameFiller;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut d1: libc::c_int = 0;
    let mut b1: libc::c_int = 0;
    let mut blocks: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut words: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    blocks = *spec.offset(0 as libc::c_int as isize)
        + *spec.offset(3 as libc::c_int as isize);
    words = *spec.offset(0 as libc::c_int as isize)
        * *spec.offset(1 as libc::c_int as isize)
        + *spec.offset(3 as libc::c_int as isize)
            * *spec.offset(4 as libc::c_int as isize);
    d1 = *spec.offset(1 as libc::c_int as isize);
    b1 = *spec.offset(0 as libc::c_int as isize);
    tmp = malloc((words as size_t).wrapping_mul(8 as libc::c_ulong));
    bits = tmp as *mut libc::c_uchar;
    col = 0 as libc::c_int;
    row = col;
    filler = FrameFiller_new(width, frame, 0 as libc::c_int);
    i = 0 as libc::c_int;
    while i < words {
        col = i / blocks;
        if col >= d1 {
            tmp___0 = b1;
        } else {
            tmp___0 = 0 as libc::c_int;
        }
        row = i % blocks + tmp___0;
        if row > b1 {
            tmp___1 = row - b1;
        } else {
            tmp___1 = 0 as libc::c_int;
        }
        idx = d1 * row + col + tmp___1;
        q = bits.offset((idx * 8 as libc::c_int) as isize);
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            p = FrameFiller_next(filler);
            *q
                .offset(
                    j as isize,
                ) = (*p as libc::c_int & 1 as libc::c_int) as libc::c_uchar;
            j += 1;
        }
        i += 1;
    }
    free(filler as *mut libc::c_void);
    bstream = BitStream_newWithBits(
        (words as size_t).wrapping_mul(8 as libc::c_ulong),
        bits,
    );
    free(bits as *mut libc::c_void);
    return bstream;
}
pub unsafe extern "C" fn QRcode_extractBits(
    mut code: *mut QRcode,
    mut dataLength: *mut libc::c_int,
    mut eccLength: *mut libc::c_int,
) -> *mut BitStream {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut unmasked: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut spec: [libc::c_int; 5] = [0; 5];
    let mut ret: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut level: QRecLevel = QR_ECLEVEL_L;
    version = QRcode_decodeVersion(code);
    if version < 1 as libc::c_int {
        return 0 as *mut libc::c_void as *mut BitStream;
    }
    ret = QRcode_decodeFormat(code, &mut level, &mut mask);
    if ret < 0 as libc::c_int {
        return 0 as *mut libc::c_void as *mut BitStream;
    }
    QRspec_getEccSpec(version, level, spec.as_mut_ptr());
    unmasked = unmask(code, level, mask);
    if unmasked as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut BitStream;
    }
    bstream = extractBits((*code).width, unmasked, spec.as_mut_ptr());
    free(unmasked as *mut libc::c_void);
    *dataLength = (spec[0 as libc::c_int as usize] * spec[1 as libc::c_int as usize]
        + spec[3 as libc::c_int as usize] * spec[4 as libc::c_int as usize])
        * 8 as libc::c_int;
    *eccLength = (spec[0 as libc::c_int as usize] + spec[3 as libc::c_int as usize])
        * spec[2 as libc::c_int as usize] * 8 as libc::c_int;
    return bstream;
}
unsafe extern "C" fn checkRemainderWords(
    mut length: libc::c_int,
    mut bits: *mut libc::c_uchar,
    mut remainder: libc::c_int,
) -> libc::c_int {
    let mut rbits: libc::c_int = 0;
    let mut words: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut v: libc::c_uchar = 0;
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_uint = 0;
    let mut tmp___0: libc::c_int = 0;
    words = remainder / 8 as libc::c_int;
    rbits = remainder - words * 8 as libc::c_int;
    bits = bits.offset((length - remainder) as isize);
    i = 0 as libc::c_int;
    while i < rbits {
        if *bits.offset(i as isize) as libc::c_int & 1 as libc::c_int != 0 as libc::c_int
        {
            printf(
                b"Terminating code includes 1-bit.\n\0" as *const u8
                    as *const libc::c_char,
            );
            printBinary(bits, remainder);
            return -(1 as libc::c_int);
        }
        i += 1;
    }
    p = bits.offset(rbits as isize);
    i = 0 as libc::c_int;
    while i < words {
        tmp = bitToInt(p, 8 as libc::c_int);
        v = tmp as libc::c_uchar;
        if i & 1 as libc::c_int != 0 {
            tmp___0 = 17 as libc::c_int;
        } else {
            tmp___0 = 236 as libc::c_int;
        }
        if v as libc::c_int != tmp___0 {
            printf(
                b"Remainder codewords wrong.\n\0" as *const u8 as *const libc::c_char,
            );
            printBinary(bits, remainder);
            return -(1 as libc::c_int);
        }
        p = p.offset(8 as libc::c_int as isize);
        i += 1;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRcode_decodeBits(mut code: *mut QRcode) -> *mut QRdata {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut unmasked: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut spec: [libc::c_int; 5] = [0; 5];
    let mut ret: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut level: QRecLevel = QR_ECLEVEL_L;
    let mut qrdata: *mut QRdata = 0 as *mut QRdata;
    version = QRcode_decodeVersion(code);
    if version < 1 as libc::c_int {
        return 0 as *mut libc::c_void as *mut QRdata;
    }
    ret = QRcode_decodeFormat(code, &mut level, &mut mask);
    if ret < 0 as libc::c_int {
        return 0 as *mut libc::c_void as *mut QRdata;
    }
    QRspec_getEccSpec(version, level, spec.as_mut_ptr());
    length = (spec[0 as libc::c_int as usize] * spec[1 as libc::c_int as usize]
        + spec[3 as libc::c_int as usize] * spec[4 as libc::c_int as usize])
        * 8 as libc::c_int;
    unmasked = unmask(code, level, mask);
    if unmasked as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut QRdata;
    }
    bstream = extractBits((*code).width, unmasked, spec.as_mut_ptr());
    free(unmasked as *mut libc::c_void);
    qrdata = QRdata_new();
    (*qrdata).version = version;
    (*qrdata).level = level;
    ret = QRdata_decodeBitStream(qrdata, bstream);
    if ret > 0 as libc::c_int {
        checkRemainderWords(length, (*bstream).data, ret);
    }
    BitStream_free(bstream);
    return qrdata;
}
pub unsafe extern "C" fn QRdata_concatChunks(mut qrdata: *mut QRdata) {
    (*qrdata).data = DataChunk_concatChunkList((*qrdata).chunks, &mut (*qrdata).size);
}
pub unsafe extern "C" fn QRcode_decode(mut code: *mut QRcode) -> *mut QRdata {
    let mut qrdata: *mut QRdata = 0 as *mut QRdata;
    qrdata = QRcode_decodeBits(code);
    QRdata_concatChunks(qrdata);
    return qrdata;
}
pub static mut MQRformat: [FormatInfo; 8] = [
    {
        let mut init = FormatInfo {
            version: 1 as libc::c_int,
            level: QR_ECLEVEL_L,
        };
        init
    },
    {
        let mut init = FormatInfo {
            version: 2 as libc::c_int,
            level: QR_ECLEVEL_L,
        };
        init
    },
    {
        let mut init = FormatInfo {
            version: 2 as libc::c_int,
            level: QR_ECLEVEL_M,
        };
        init
    },
    {
        let mut init = FormatInfo {
            version: 3 as libc::c_int,
            level: QR_ECLEVEL_L,
        };
        init
    },
    {
        let mut init = FormatInfo {
            version: 3 as libc::c_int,
            level: QR_ECLEVEL_M,
        };
        init
    },
    {
        let mut init = FormatInfo {
            version: 4 as libc::c_int,
            level: QR_ECLEVEL_L,
        };
        init
    },
    {
        let mut init = FormatInfo {
            version: 4 as libc::c_int,
            level: QR_ECLEVEL_M,
        };
        init
    },
    {
        let mut init = FormatInfo {
            version: 4 as libc::c_int,
            level: QR_ECLEVEL_Q,
        };
        init
    },
];
pub unsafe extern "C" fn QRcode_decodeFormatMQR(
    mut code: *mut QRcode,
    mut version: *mut libc::c_int,
    mut level: *mut QRecLevel,
    mut mask: *mut libc::c_int,
) -> libc::c_int {
    let mut v: libc::c_uint = 0;
    let mut t: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    width = (*code).width;
    v = 0 as libc::c_uint;
    p = ((*code).data)
        .offset((width * 8 as libc::c_int) as isize)
        .offset(1 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        v <<= 1 as libc::c_int;
        v |= (*p.offset(i as isize) as libc::c_int & 1 as libc::c_int) as libc::c_uint;
        i += 1;
    }
    p = ((*code).data)
        .offset((width * 7 as libc::c_int) as isize)
        .offset(8 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        v <<= 1 as libc::c_int;
        v
            |= (*p.offset(-((width * i) as isize)) as libc::c_int & 1 as libc::c_int)
                as libc::c_uint;
        i += 1;
    }
    v ^= 17477 as libc::c_uint;
    *mask = (v >> 10 as libc::c_int & 3 as libc::c_uint) as libc::c_int;
    t = v >> 12 as libc::c_int & 7 as libc::c_uint;
    *version = MQRformat[t as usize].version;
    *level = MQRformat[t as usize].level;
    if *version * 2 as libc::c_int + 9 as libc::c_int != width {
        printf(
            b"Decoded version number does not match to the size.\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn unmaskMQR(
    mut code: *mut QRcode,
    mut level: QRecLevel,
    mut mask: libc::c_int,
) -> *mut libc::c_uchar {
    let mut unmasked: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    unmasked = MMask_makeMask((*code).version, (*code).data, mask, level);
    return unmasked;
}
pub unsafe extern "C" fn QRcode_unmaskMQR(mut code: *mut QRcode) -> *mut libc::c_uchar {
    let mut ret: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut level: QRecLevel = QR_ECLEVEL_L;
    let mut tmp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    ret = QRcode_decodeFormatMQR(code, &mut version, &mut level, &mut mask);
    if ret < 0 as libc::c_int {
        return 0 as *mut libc::c_void as *mut libc::c_uchar;
    }
    tmp = unmaskMQR(code, level, mask);
    return tmp;
}
unsafe extern "C" fn extractBitsMQR(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> *mut BitStream {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut bits: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut filler: *mut FrameFiller = 0 as *mut FrameFiller;
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    tmp = MQRspec_getDataLengthBit(version, level);
    tmp___0 = MQRspec_getECCLength(version, level);
    size = tmp + tmp___0 * 8 as libc::c_int;
    tmp___1 = malloc(size as size_t);
    bits = tmp___1 as *mut libc::c_uchar;
    filler = FrameFiller_new(width, frame, 1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < size {
        tmp___2 = FrameFiller_next(filler);
        *bits
            .offset(
                i as isize,
            ) = (*tmp___2 as libc::c_int & 1 as libc::c_int) as libc::c_uchar;
        i += 1;
    }
    free(filler as *mut libc::c_void);
    bstream = BitStream_newWithBits(size as size_t, bits);
    free(bits as *mut libc::c_void);
    return bstream;
}
pub unsafe extern "C" fn QRcode_extractBitsMQR(
    mut code: *mut QRcode,
    mut dataLength: *mut libc::c_int,
    mut eccLength: *mut libc::c_int,
    mut version: *mut libc::c_int,
    mut level: *mut QRecLevel,
) -> *mut BitStream {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut unmasked: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ret: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    ret = QRcode_decodeFormatMQR(code, version, level, &mut mask);
    if ret < 0 as libc::c_int {
        return 0 as *mut libc::c_void as *mut BitStream;
    }
    unmasked = unmaskMQR(code, *level, mask);
    if unmasked as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut BitStream;
    }
    *dataLength = MQRspec_getDataLengthBit(*version, *level);
    tmp = MQRspec_getECCLength(*version, *level);
    *eccLength = tmp * 8 as libc::c_int;
    bstream = extractBitsMQR((*code).width, unmasked, *version, *level);
    free(unmasked as *mut libc::c_void);
    return bstream;
}
unsafe extern "C" fn checkRemainderWordsMQR(
    mut length: libc::c_int,
    mut bits: *mut libc::c_uchar,
    mut remainder: libc::c_int,
    mut version: libc::c_int,
) -> libc::c_int {
    let mut rbits: libc::c_int = 0;
    let mut words: libc::c_int = 0;
    let mut paddings: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut v: libc::c_uchar = 0;
    let mut i: libc::c_int = 0;
    let mut decoded: libc::c_int = 0;
    let mut tmp: libc::c_uint = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_uint = 0;
    let mut tmp___2: libc::c_int = 0;
    decoded = length - remainder;
    bits = bits.offset(decoded as isize);
    words = (decoded + 7 as libc::c_int) / 8 as libc::c_int;
    rbits = words * 8 as libc::c_int - decoded;
    i = 0 as libc::c_int;
    while i < rbits {
        if *bits.offset(i as isize) as libc::c_int & 1 as libc::c_int != 0 as libc::c_int
        {
            printf(
                b"Terminating code includes 1-bit.\n\0" as *const u8
                    as *const libc::c_char,
            );
            printBinary(bits, remainder);
            return -(1 as libc::c_int);
        }
        i += 1;
    }
    paddings = (length - words * 8 as libc::c_int) / 8 as libc::c_int;
    p = bits.offset(rbits as isize);
    i = 0 as libc::c_int;
    while i < paddings {
        tmp = bitToInt(p, 8 as libc::c_int);
        v = tmp as libc::c_uchar;
        if i & 1 as libc::c_int != 0 {
            tmp___0 = 17 as libc::c_int;
        } else {
            tmp___0 = 236 as libc::c_int;
        }
        if v as libc::c_int != tmp___0 {
            printf(
                b"Remainder codewords wrong.\n\0" as *const u8 as *const libc::c_char,
            );
            printBinary(bits, remainder);
            return -(1 as libc::c_int);
        }
        p = p.offset(8 as libc::c_int as isize);
        i += 1;
    }
    rbits = length - (paddings + words) * 8 as libc::c_int;
    if rbits > 0 as libc::c_int {
        's_231: {
            let mut current_block_49: u64;
            if version == 1 as libc::c_int {
                current_block_49 = 330682787277640468;
            } else if version == 3 as libc::c_int {
                current_block_49 = 330682787277640468;
            } else {
                current_block_49 = 3667527089883048861;
            }
            match current_block_49 {
                330682787277640468 => {
                    if rbits == 4 as libc::c_int {
                        tmp___1 = bitToInt(p, 4 as libc::c_int);
                        v = tmp___1 as libc::c_uchar;
                        if v as libc::c_int != 0 as libc::c_int {
                            printf(
                                b"Last padding bits include 1-bit.\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            return -(1 as libc::c_int);
                        }
                        break 's_231;
                    }
                }
                _ => {}
            }
            if version == 1 as libc::c_int {
                tmp___2 = 4 as libc::c_int;
            } else if version == 3 as libc::c_int {
                tmp___2 = 4 as libc::c_int;
            } else {
                tmp___2 = 0 as libc::c_int;
            }
            printf(
                b"The length of the last padding bits is %d, not %d.\n\0" as *const u8
                    as *const libc::c_char,
                rbits,
                tmp___2,
            );
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRcode_decodeBitsMQR(mut code: *mut QRcode) -> *mut QRdata {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut ret: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut dataLength: libc::c_int = 0;
    let mut eccLength: libc::c_int = 0;
    let mut level: QRecLevel = QR_ECLEVEL_L;
    let mut qrdata: *mut QRdata = 0 as *mut QRdata;
    bstream = QRcode_extractBitsMQR(
        code,
        &mut dataLength,
        &mut eccLength,
        &mut version,
        &mut level,
    );
    if bstream as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut QRdata;
    }
    qrdata = QRdata_newMQR();
    (*qrdata).version = version;
    (*qrdata).level = level;
    ret = QRdata_decodeBits(qrdata, dataLength, (*bstream).data);
    if ret > 0 as libc::c_int {
        ret = checkRemainderWordsMQR(dataLength, (*bstream).data, ret, version);
        if ret < 0 as libc::c_int {
            QRdata_free(qrdata);
            qrdata = 0 as *mut libc::c_void as *mut QRdata;
        }
    }
    BitStream_free(bstream);
    return qrdata;
}
pub unsafe extern "C" fn QRcode_decodeMQR(mut code: *mut QRcode) -> *mut QRdata {
    let mut qrdata: *mut QRdata = 0 as *mut QRdata;
    qrdata = QRcode_decodeBitsMQR(code);
    if qrdata as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        QRdata_concatChunks(qrdata);
    }
    return qrdata;
}
static mut decodeAnTable___0: [libc::c_char; 45] = [
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '$' as i32 as libc::c_char,
    '%' as i32 as libc::c_char,
    '*' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    ':' as i32 as libc::c_char,
];
unsafe extern "C" fn test_qrraw_new() {
    let mut i: libc::c_int = 0;
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut num: [libc::c_char; 9] = [0; 9];
    let mut raw: *mut QRRawCode = 0 as *mut QRRawCode;
    num[0 as libc::c_int as usize] = '0' as i32 as libc::c_char;
    num[1 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    num[2 as libc::c_int as usize] = '2' as i32 as libc::c_char;
    num[3 as libc::c_int as usize] = '3' as i32 as libc::c_char;
    num[4 as libc::c_int as usize] = '4' as i32 as libc::c_char;
    num[5 as libc::c_int as usize] = '5' as i32 as libc::c_char;
    num[6 as libc::c_int as usize] = '6' as i32 as libc::c_char;
    num[7 as libc::c_int as usize] = '7' as i32 as libc::c_char;
    num[8 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    testStartReal(
        b"test_qrraw_new\0" as *const u8 as *const libc::c_char,
        b"Test QRraw_new()\0" as *const u8 as *const libc::c_char,
    );
    stream = QRinput_new();
    QRinput_setVersion(stream, 10 as libc::c_int);
    QRinput_setErrorCorrectionLevel(stream, QR_ECLEVEL_Q);
    QRinput_append(
        stream,
        QR_MODE_NUM,
        8 as libc::c_int,
        num.as_mut_ptr() as *mut libc::c_uchar as *const libc::c_uchar,
    );
    raw = QRraw_new(stream);
    assertionNum += 1;
    if !(raw as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        assertionFailed += 1;
        printf(b"Failed QRraw_new().\n\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    if !((*raw).count == 0 as libc::c_int) {
        assertionFailed += 1;
        printf(
            b"QRraw.count = %d != 0\n\0" as *const u8 as *const libc::c_char,
            (*raw).count,
        );
    }
    assertionNum += 1;
    if !((*raw).version == 10 as libc::c_int) {
        assertionFailed += 1;
        printf(
            b"QRraw.version was not as expected. (%d)\n\0" as *const u8
                as *const libc::c_char,
            (*raw).version,
        );
    }
    assertionNum += 1;
    if !((*raw).dataLength == 154 as libc::c_int) {
        assertionFailed += 1;
        printf(
            b"QRraw.dataLength was not as expected.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    assertionNum += 1;
    if !((*raw).eccLength == 192 as libc::c_int) {
        assertionFailed += 1;
        printf(
            b"QRraw.eccLength was not as expected.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    assertionNum += 1;
    if !((*raw).b1 == 6 as libc::c_int) {
        assertionFailed += 1;
        printf(b"QRraw.b1 was not as expected.\n\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    if !((*raw).blocks == 8 as libc::c_int) {
        assertionFailed += 1;
        printf(
            b"QRraw.blocks was not as expected.\n\0" as *const u8 as *const libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    while i < (*raw).b1 {
        assertionNum += 1;
        if !((*((*raw).rsblock).offset(i as isize)).dataLength == 19 as libc::c_int) {
            assertionFailed += 1;
            printf(
                b"QRraw.rsblock[%d].dataLength was not as expected. (%d)\n\0"
                    as *const u8 as *const libc::c_char,
                i,
                (*((*raw).rsblock).offset(i as isize)).dataLength,
            );
        }
        i += 1;
    }
    i = (*raw).b1;
    while i < (*raw).blocks {
        assertionNum += 1;
        if !((*((*raw).rsblock).offset(i as isize)).dataLength == 20 as libc::c_int) {
            assertionFailed += 1;
            printf(
                b"QRraw.rsblock[%d].dataLength was not as expected. (%d)\n\0"
                    as *const u8 as *const libc::c_char,
                i,
                (*((*raw).rsblock).offset(i as isize)).dataLength,
            );
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*raw).blocks {
        assertionNum += 1;
        if !((*((*raw).rsblock).offset(i as isize)).eccLength == 24 as libc::c_int) {
            assertionFailed += 1;
            printf(
                b"QRraw.rsblock[%d].eccLength was not as expected. (%d)\n\0" as *const u8
                    as *const libc::c_char,
                i,
                (*((*raw).rsblock).offset(i as isize)).eccLength,
            );
        }
        i += 1;
    }
    QRinput_free(stream);
    QRraw_free(raw);
    testFinish();
}
unsafe extern "C" fn test_iterate() {
    let mut i: libc::c_int = 0;
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut num: [libc::c_char; 9] = [0; 9];
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut raw: *mut QRRawCode = 0 as *mut QRRawCode;
    let mut err: libc::c_int = 0;
    let mut tmp: libc::c_uchar = 0;
    num[0 as libc::c_int as usize] = '0' as i32 as libc::c_char;
    num[1 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    num[2 as libc::c_int as usize] = '2' as i32 as libc::c_char;
    num[3 as libc::c_int as usize] = '3' as i32 as libc::c_char;
    num[4 as libc::c_int as usize] = '4' as i32 as libc::c_char;
    num[5 as libc::c_int as usize] = '5' as i32 as libc::c_char;
    num[6 as libc::c_int as usize] = '6' as i32 as libc::c_char;
    num[7 as libc::c_int as usize] = '7' as i32 as libc::c_char;
    num[8 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    err = 0 as libc::c_int;
    testStartReal(
        b"test_iterate\0" as *const u8 as *const libc::c_char,
        b"Test getCode (1-L)\0" as *const u8 as *const libc::c_char,
    );
    stream = QRinput_new();
    QRinput_setVersion(stream, 1 as libc::c_int);
    QRinput_setErrorCorrectionLevel(stream, QR_ECLEVEL_L);
    QRinput_append(
        stream,
        QR_MODE_NUM,
        8 as libc::c_int,
        num.as_mut_ptr() as *mut libc::c_uchar as *const libc::c_uchar,
    );
    raw = QRraw_new(stream);
    data = (*raw).datacode;
    i = 0 as libc::c_int;
    while i < (*raw).dataLength {
        tmp = QRraw_getCode(raw);
        if *data.offset(i as isize) as libc::c_int != tmp as libc::c_int {
            err += 1;
        }
        i += 1;
    }
    QRinput_free(stream);
    QRraw_free(raw);
    testEnd(err);
}
unsafe extern "C" fn test_iterate2() {
    let mut i: libc::c_int = 0;
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut num: [libc::c_char; 9] = [0; 9];
    let mut raw: *mut QRRawCode = 0 as *mut QRRawCode;
    let mut err: libc::c_int = 0;
    let mut correct: [libc::c_uchar; 134] = [0; 134];
    let mut tmp: libc::c_uchar = 0;
    num[0 as libc::c_int as usize] = '0' as i32 as libc::c_char;
    num[1 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    num[2 as libc::c_int as usize] = '2' as i32 as libc::c_char;
    num[3 as libc::c_int as usize] = '3' as i32 as libc::c_char;
    num[4 as libc::c_int as usize] = '4' as i32 as libc::c_char;
    num[5 as libc::c_int as usize] = '5' as i32 as libc::c_char;
    num[6 as libc::c_int as usize] = '6' as i32 as libc::c_char;
    num[7 as libc::c_int as usize] = '7' as i32 as libc::c_char;
    num[8 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    err = 0 as libc::c_int;
    correct[0 as libc::c_int as usize] = 16 as libc::c_int as libc::c_uchar;
    correct[1 as libc::c_int as usize] = 17 as libc::c_int as libc::c_uchar;
    correct[2 as libc::c_int as usize] = 236 as libc::c_int as libc::c_uchar;
    correct[3 as libc::c_int as usize] = 236 as libc::c_int as libc::c_uchar;
    correct[4 as libc::c_int as usize] = 32 as libc::c_int as libc::c_uchar;
    correct[5 as libc::c_int as usize] = 236 as libc::c_int as libc::c_uchar;
    correct[6 as libc::c_int as usize] = 17 as libc::c_int as libc::c_uchar;
    correct[7 as libc::c_int as usize] = 17 as libc::c_int as libc::c_uchar;
    correct[8 as libc::c_int as usize] = 12 as libc::c_int as libc::c_uchar;
    correct[9 as libc::c_int as usize] = 17 as libc::c_int as libc::c_uchar;
    correct[10 as libc::c_int as usize] = 236 as libc::c_int as libc::c_uchar;
    correct[11 as libc::c_int as usize] = 236 as libc::c_int as libc::c_uchar;
    correct[12 as libc::c_int as usize] = 86 as libc::c_int as libc::c_uchar;
    correct[13 as libc::c_int as usize] = 236 as libc::c_int as libc::c_uchar;
    correct[14 as libc::c_int as usize] = 17 as libc::c_int as libc::c_uchar;
    correct[15 as libc::c_int as usize] = 17 as libc::c_int as libc::c_uchar;
    correct[16 as libc::c_int as usize] = 97 as libc::c_int as libc::c_uchar;
    correct[17 as libc::c_int as usize] = 17 as libc::c_int as libc::c_uchar;
    correct[18 as libc::c_int as usize] = 236 as libc::c_int as libc::c_uchar;
    correct[19 as libc::c_int as usize] = 236 as libc::c_int as libc::c_uchar;
    correct[20 as libc::c_int as usize] = 128 as libc::c_int as libc::c_uchar;
    correct[21 as libc::c_int as usize] = 236 as libc::c_int as libc::c_uchar;
    correct[22 as libc::c_int as usize] = 17 as libc::c_int as libc::c_uchar;
    correct[23 as libc::c_int as usize] = 17 as libc::c_int as libc::c_uchar;
    correct[24 as libc::c_int as usize] = 236 as libc::c_int as libc::c_uchar;
    correct[25 as libc::c_int as usize] = 17 as libc::c_int as libc::c_uchar;
    correct[26 as libc::c_int as usize] = 236 as libc::c_int as libc::c_uchar;
    correct[27 as libc::c_int as usize] = 236 as libc::c_int as libc::c_uchar;
    correct[28 as libc::c_int as usize] = 17 as libc::c_int as libc::c_uchar;
    correct[29 as libc::c_int as usize] = 236 as libc::c_int as libc::c_uchar;
    correct[30 as libc::c_int as usize] = 17 as libc::c_int as libc::c_uchar;
    correct[31 as libc::c_int as usize] = 17 as libc::c_int as libc::c_uchar;
    correct[32 as libc::c_int as usize] = 236 as libc::c_int as libc::c_uchar;
    correct[33 as libc::c_int as usize] = 17 as libc::c_int as libc::c_uchar;
    correct[34 as libc::c_int as usize] = 236 as libc::c_int as libc::c_uchar;
    correct[35 as libc::c_int as usize] = 236 as libc::c_int as libc::c_uchar;
    correct[36 as libc::c_int as usize] = 17 as libc::c_int as libc::c_uchar;
    correct[37 as libc::c_int as usize] = 236 as libc::c_int as libc::c_uchar;
    correct[38 as libc::c_int as usize] = 17 as libc::c_int as libc::c_uchar;
    correct[39 as libc::c_int as usize] = 17 as libc::c_int as libc::c_uchar;
    correct[40 as libc::c_int as usize] = 236 as libc::c_int as libc::c_uchar;
    correct[41 as libc::c_int as usize] = 17 as libc::c_int as libc::c_uchar;
    correct[42 as libc::c_int as usize] = 236 as libc::c_int as libc::c_uchar;
    correct[43 as libc::c_int as usize] = 236 as libc::c_int as libc::c_uchar;
    correct[44 as libc::c_int as usize] = 17 as libc::c_int as libc::c_uchar;
    correct[45 as libc::c_int as usize] = 17 as libc::c_int as libc::c_uchar;
    correct[46 as libc::c_int as usize] = 92 as libc::c_int as libc::c_uchar;
    correct[47 as libc::c_int as usize] = 222 as libc::c_int as libc::c_uchar;
    correct[48 as libc::c_int as usize] = 104 as libc::c_int as libc::c_uchar;
    correct[49 as libc::c_int as usize] = 104 as libc::c_int as libc::c_uchar;
    correct[50 as libc::c_int as usize] = 77 as libc::c_int as libc::c_uchar;
    correct[51 as libc::c_int as usize] = 179 as libc::c_int as libc::c_uchar;
    correct[52 as libc::c_int as usize] = 219 as libc::c_int as libc::c_uchar;
    correct[53 as libc::c_int as usize] = 219 as libc::c_int as libc::c_uchar;
    correct[54 as libc::c_int as usize] = 213 as libc::c_int as libc::c_uchar;
    correct[55 as libc::c_int as usize] = 20 as libc::c_int as libc::c_uchar;
    correct[56 as libc::c_int as usize] = 225 as libc::c_int as libc::c_uchar;
    correct[57 as libc::c_int as usize] = 225 as libc::c_int as libc::c_uchar;
    correct[58 as libc::c_int as usize] = 91 as libc::c_int as libc::c_uchar;
    correct[59 as libc::c_int as usize] = 42 as libc::c_int as libc::c_uchar;
    correct[60 as libc::c_int as usize] = 31 as libc::c_int as libc::c_uchar;
    correct[61 as libc::c_int as usize] = 31 as libc::c_int as libc::c_uchar;
    correct[62 as libc::c_int as usize] = 73 as libc::c_int as libc::c_uchar;
    correct[63 as libc::c_int as usize] = 196 as libc::c_int as libc::c_uchar;
    correct[64 as libc::c_int as usize] = 120 as libc::c_int as libc::c_uchar;
    correct[65 as libc::c_int as usize] = 120 as libc::c_int as libc::c_uchar;
    correct[66 as libc::c_int as usize] = 247 as libc::c_int as libc::c_uchar;
    correct[67 as libc::c_int as usize] = 224 as libc::c_int as libc::c_uchar;
    correct[68 as libc::c_int as usize] = 91 as libc::c_int as libc::c_uchar;
    correct[69 as libc::c_int as usize] = 91 as libc::c_int as libc::c_uchar;
    correct[70 as libc::c_int as usize] = 195 as libc::c_int as libc::c_uchar;
    correct[71 as libc::c_int as usize] = 167 as libc::c_int as libc::c_uchar;
    correct[72 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[73 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[74 as libc::c_int as usize] = 93 as libc::c_int as libc::c_uchar;
    correct[75 as libc::c_int as usize] = 154 as libc::c_int as libc::c_uchar;
    correct[76 as libc::c_int as usize] = 234 as libc::c_int as libc::c_uchar;
    correct[77 as libc::c_int as usize] = 234 as libc::c_int as libc::c_uchar;
    correct[78 as libc::c_int as usize] = 72 as libc::c_int as libc::c_uchar;
    correct[79 as libc::c_int as usize] = 173 as libc::c_int as libc::c_uchar;
    correct[80 as libc::c_int as usize] = 157 as libc::c_int as libc::c_uchar;
    correct[81 as libc::c_int as usize] = 157 as libc::c_int as libc::c_uchar;
    correct[82 as libc::c_int as usize] = 88 as libc::c_int as libc::c_uchar;
    correct[83 as libc::c_int as usize] = 179 as libc::c_int as libc::c_uchar;
    correct[84 as libc::c_int as usize] = 63 as libc::c_int as libc::c_uchar;
    correct[85 as libc::c_int as usize] = 63 as libc::c_int as libc::c_uchar;
    correct[86 as libc::c_int as usize] = 16 as libc::c_int as libc::c_uchar;
    correct[87 as libc::c_int as usize] = 219 as libc::c_int as libc::c_uchar;
    correct[88 as libc::c_int as usize] = 191 as libc::c_int as libc::c_uchar;
    correct[89 as libc::c_int as usize] = 191 as libc::c_int as libc::c_uchar;
    correct[90 as libc::c_int as usize] = 235 as libc::c_int as libc::c_uchar;
    correct[91 as libc::c_int as usize] = 236 as libc::c_int as libc::c_uchar;
    correct[92 as libc::c_int as usize] = 5 as libc::c_int as libc::c_uchar;
    correct[93 as libc::c_int as usize] = 5 as libc::c_int as libc::c_uchar;
    correct[94 as libc::c_int as usize] = 152 as libc::c_int as libc::c_uchar;
    correct[95 as libc::c_int as usize] = 53 as libc::c_int as libc::c_uchar;
    correct[96 as libc::c_int as usize] = 131 as libc::c_int as libc::c_uchar;
    correct[97 as libc::c_int as usize] = 131 as libc::c_int as libc::c_uchar;
    correct[98 as libc::c_int as usize] = 169 as libc::c_int as libc::c_uchar;
    correct[99 as libc::c_int as usize] = 149 as libc::c_int as libc::c_uchar;
    correct[100 as libc::c_int as usize] = 166 as libc::c_int as libc::c_uchar;
    correct[101 as libc::c_int as usize] = 166 as libc::c_int as libc::c_uchar;
    correct[102 as libc::c_int as usize] = 234 as libc::c_int as libc::c_uchar;
    correct[103 as libc::c_int as usize] = 123 as libc::c_int as libc::c_uchar;
    correct[104 as libc::c_int as usize] = 141 as libc::c_int as libc::c_uchar;
    correct[105 as libc::c_int as usize] = 141 as libc::c_int as libc::c_uchar;
    correct[106 as libc::c_int as usize] = 4 as libc::c_int as libc::c_uchar;
    correct[107 as libc::c_int as usize] = 60 as libc::c_int as libc::c_uchar;
    correct[108 as libc::c_int as usize] = 8 as libc::c_int as libc::c_uchar;
    correct[109 as libc::c_int as usize] = 8 as libc::c_int as libc::c_uchar;
    correct[110 as libc::c_int as usize] = 100 as libc::c_int as libc::c_uchar;
    correct[111 as libc::c_int as usize] = 206 as libc::c_int as libc::c_uchar;
    correct[112 as libc::c_int as usize] = 62 as libc::c_int as libc::c_uchar;
    correct[113 as libc::c_int as usize] = 62 as libc::c_int as libc::c_uchar;
    correct[114 as libc::c_int as usize] = 77 as libc::c_int as libc::c_uchar;
    correct[115 as libc::c_int as usize] = 155 as libc::c_int as libc::c_uchar;
    correct[116 as libc::c_int as usize] = 48 as libc::c_int as libc::c_uchar;
    correct[117 as libc::c_int as usize] = 48 as libc::c_int as libc::c_uchar;
    correct[118 as libc::c_int as usize] = 78 as libc::c_int as libc::c_uchar;
    correct[119 as libc::c_int as usize] = 101 as libc::c_int as libc::c_uchar;
    correct[120 as libc::c_int as usize] = 214 as libc::c_int as libc::c_uchar;
    correct[121 as libc::c_int as usize] = 214 as libc::c_int as libc::c_uchar;
    correct[122 as libc::c_int as usize] = 228 as libc::c_int as libc::c_uchar;
    correct[123 as libc::c_int as usize] = 83 as libc::c_int as libc::c_uchar;
    correct[124 as libc::c_int as usize] = 44 as libc::c_int as libc::c_uchar;
    correct[125 as libc::c_int as usize] = 44 as libc::c_int as libc::c_uchar;
    correct[126 as libc::c_int as usize] = 70 as libc::c_int as libc::c_uchar;
    correct[127 as libc::c_int as usize] = 29 as libc::c_int as libc::c_uchar;
    correct[128 as libc::c_int as usize] = 46 as libc::c_int as libc::c_uchar;
    correct[129 as libc::c_int as usize] = 46 as libc::c_int as libc::c_uchar;
    correct[130 as libc::c_int as usize] = 41 as libc::c_int as libc::c_uchar;
    correct[131 as libc::c_int as usize] = 22 as libc::c_int as libc::c_uchar;
    correct[132 as libc::c_int as usize] = 39 as libc::c_int as libc::c_uchar;
    correct[133 as libc::c_int as usize] = 39 as libc::c_int as libc::c_uchar;
    testStartReal(
        b"test_iterate2\0" as *const u8 as *const libc::c_char,
        b"Test getCode (5-H)\0" as *const u8 as *const libc::c_char,
    );
    stream = QRinput_new();
    QRinput_setVersion(stream, 5 as libc::c_int);
    QRinput_setErrorCorrectionLevel(stream, QR_ECLEVEL_H);
    QRinput_append(
        stream,
        QR_MODE_NUM,
        8 as libc::c_int,
        num.as_mut_ptr() as *mut libc::c_uchar as *const libc::c_uchar,
    );
    raw = QRraw_new(stream);
    i = 0 as libc::c_int;
    while i < (*raw).dataLength {
        tmp = QRraw_getCode(raw);
        if correct[i as usize] as libc::c_int != tmp as libc::c_int {
            err += 1;
        }
        i += 1;
    }
    QRinput_free(stream);
    QRraw_free(raw);
    testEnd(err);
}
unsafe extern "C" fn print_filler() {
    let mut width: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    version = 7 as libc::c_int;
    puts(b"\nPrinting debug info of FrameFiller.\0" as *const u8 as *const libc::c_char);
    width = QRspec_getWidth(version);
    frame = FrameFiller_test(version);
    if frame as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        abort();
    }
    printFrame(width, frame);
    free(frame as *mut libc::c_void);
}
unsafe extern "C" fn test_filler() {
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    testStartReal(
        b"test_filler\0" as *const u8 as *const libc::c_char,
        b"Frame filler test\0" as *const u8 as *const libc::c_char,
    );
    i = 1 as libc::c_int;
    while i <= 40 as libc::c_int {
        tmp = QRspec_getDataLength(i, QR_ECLEVEL_L);
        tmp___0 = QRspec_getECCLength(i, QR_ECLEVEL_L);
        tmp___1 = QRspec_getRemainder(i);
        length = tmp * 8 as libc::c_int + tmp___0 * 8 as libc::c_int + tmp___1;
        frame = FrameFiller_test(i);
        if frame as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            assertionNum += 1;
            if !(frame as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
                assertionFailed += 1;
                printf(
                    b"Something wrong in version %d\n\0" as *const u8
                        as *const libc::c_char,
                    i,
                );
            }
        } else {
            w = QRspec_getWidth(i);
            e = 0 as libc::c_int;
            j = 0 as libc::c_int;
            while j < w * w {
                if *frame.offset(j as isize) as libc::c_int == 0 as libc::c_int {
                    e += 1;
                }
                j += 1;
            }
            assertionNum += 1;
            if !(e == 0 as libc::c_int) {
                assertionFailed += 1;
                printf(
                    b"Not filled bit is found. (%d,%d)\n\0" as *const u8
                        as *const libc::c_char,
                    j % w,
                    j / w,
                );
            }
            if i > 6 as libc::c_int {
                tmp___2 = 3 as libc::c_int;
            } else {
                tmp___2 = 0 as libc::c_int;
            }
            e = w * (w - 9 as libc::c_int - tmp___2);
            assertionNum += 1;
            if !(*frame.offset(e as isize) as libc::c_int
                == (length - 1 as libc::c_int & 127 as libc::c_int) as libc::c_uchar
                    as libc::c_int | 128 as libc::c_int)
            {
                assertionFailed += 1;
                printf(
                    b"Number of cell does not match.\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            free(frame as *mut libc::c_void);
        }
        i += 1;
    }
    testFinish();
}
unsafe extern "C" fn print_fillerMQR() {
    let mut width: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    version = 3 as libc::c_int;
    puts(
        b"\nPrinting debug info of FrameFiller for Micro QR.\0" as *const u8
            as *const libc::c_char,
    );
    version = 1 as libc::c_int;
    while version <= 4 as libc::c_int {
        width = MQRspec_getWidth(version);
        frame = FrameFiller_testMQR(version);
        if frame as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            abort();
        }
        printFrame(width, frame);
        version += 1;
    }
}
unsafe extern "C" fn test_fillerMQR() {
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    testStartReal(
        b"test_fillerMQR\0" as *const u8 as *const libc::c_char,
        b"Micro QR Code Frame filler test\0" as *const u8 as *const libc::c_char,
    );
    i = 1 as libc::c_int;
    while i <= 4 as libc::c_int {
        tmp = MQRspec_getDataLengthBit(i, QR_ECLEVEL_L);
        tmp___0 = MQRspec_getECCLength(i, QR_ECLEVEL_L);
        length = tmp + tmp___0 * 8 as libc::c_int;
        frame = FrameFiller_testMQR(i);
        if frame as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            assertionNum += 1;
            if !(frame as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
                assertionFailed += 1;
                printf(
                    b"Something wrong in version %d\n\0" as *const u8
                        as *const libc::c_char,
                    i,
                );
            }
        } else {
            w = MQRspec_getWidth(i);
            e = 0 as libc::c_int;
            j = 0 as libc::c_int;
            while j < w * w {
                if *frame.offset(j as isize) as libc::c_int == 0 as libc::c_int {
                    e += 1;
                }
                j += 1;
            }
            assertionNum += 1;
            if !(e == 0 as libc::c_int) {
                assertionFailed += 1;
                printf(
                    b"Not filled bit is found. (%d,%d)\n\0" as *const u8
                        as *const libc::c_char,
                    j % w,
                    j / w,
                );
            }
            if i & 1 as libc::c_int != 0 {
                e = w * 9 as libc::c_int + 1 as libc::c_int;
            } else {
                e = w * (w - 1 as libc::c_int) + 1 as libc::c_int;
            }
            assertionNum += 1;
            if !(*frame.offset(e as isize) as libc::c_int
                == (length - 1 as libc::c_int & 127 as libc::c_int) as libc::c_uchar
                    as libc::c_int | 128 as libc::c_int)
            {
                assertionFailed += 1;
                printf(
                    b"Number of cell does not match in version %d.\n\0" as *const u8
                        as *const libc::c_char,
                    i,
                );
            }
            free(frame as *mut libc::c_void);
        }
        i += 1;
    }
    testFinish();
}
unsafe extern "C" fn test_format() {
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut format: libc::c_uint = 0;
    let mut width: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut decode: libc::c_uint = 0;
    let mut blacks: libc::c_int = 0;
    let mut b1: libc::c_int = 0;
    let mut b2: libc::c_int = 0;
    b1 = 0 as libc::c_int;
    b2 = 0 as libc::c_int;
    testStartReal(
        b"test_format\0" as *const u8 as *const libc::c_char,
        b"Test format information(level L,mask 0)\0" as *const u8 as *const libc::c_char,
    );
    width = QRspec_getWidth(1 as libc::c_int);
    frame = QRspec_newFrame(1 as libc::c_int);
    if !(frame as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
        format = QRspec_getFormatInfo(1 as libc::c_int, QR_ECLEVEL_L);
        blacks = Mask_writeFormatInformation(
            width,
            frame,
            1 as libc::c_int,
            QR_ECLEVEL_L,
        );
        decode = 0 as libc::c_uint;
        i = 0 as libc::c_int;
        while i < 15 as libc::c_int {
            if ((1 as libc::c_int) << i) as libc::c_uint & format != 0 {
                b2 += 2 as libc::c_int;
            }
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            decode <<= 1 as libc::c_int;
            decode
                |= (*frame
                    .offset(
                        (width * 8 as libc::c_int + i
                            + (i > 5 as libc::c_int) as libc::c_int) as isize,
                    ) as libc::c_int & 1 as libc::c_int) as libc::c_uint;
            if decode & 1 as libc::c_uint != 0 {
                b1 += 1;
            }
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < 7 as libc::c_int {
            decode <<= 1 as libc::c_int;
            decode
                |= (*frame
                    .offset(
                        (width
                            * (6 as libc::c_int - i
                                + (i < 1 as libc::c_int) as libc::c_int) + 8 as libc::c_int)
                            as isize,
                    ) as libc::c_int & 1 as libc::c_int) as libc::c_uint;
            if decode & 1 as libc::c_uint != 0 {
                b1 += 1;
            }
            i += 1;
        }
        if decode != format {
            printf(
                b"Upper-left format information is invalid.\n\0" as *const u8
                    as *const libc::c_char,
            );
            printf(
                b"%08x, %08x\n\0" as *const u8 as *const libc::c_char,
                format,
                decode,
            );
            testEnd(1 as libc::c_int);
            return;
        }
        decode = 0 as libc::c_uint;
        i = 0 as libc::c_int;
        while i < 7 as libc::c_int {
            decode <<= 1 as libc::c_int;
            decode
                |= (*frame
                    .offset(
                        (width * (width - 1 as libc::c_int - i) + 8 as libc::c_int)
                            as isize,
                    ) as libc::c_int & 1 as libc::c_int) as libc::c_uint;
            if decode & 1 as libc::c_uint != 0 {
                b1 += 1;
            }
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            decode <<= 1 as libc::c_int;
            decode
                |= (*frame
                    .offset(
                        (width * 8 as libc::c_int + width - 8 as libc::c_int + i)
                            as isize,
                    ) as libc::c_int & 1 as libc::c_int) as libc::c_uint;
            if decode & 1 as libc::c_uint != 0 {
                b1 += 1;
            }
            i += 1;
        }
        if decode != format {
            printf(
                b"Bottom and right format information is invalid.\n\0" as *const u8
                    as *const libc::c_char,
            );
            printf(
                b"%08x, %08x\n\0" as *const u8 as *const libc::c_char,
                format,
                decode,
            );
            testEnd(1 as libc::c_int);
            return;
        }
        if b2 != blacks {
            printf(
                b"Number of dark modules is incorrect.\n\0" as *const u8
                    as *const libc::c_char,
            );
            printf(
                b"Return value: %d, dark modules in frame: %d, should be: %d\n\0"
                    as *const u8 as *const libc::c_char,
                blacks,
                b1,
                b2,
            );
            testEnd(1 as libc::c_int);
            return;
        } else {
            if b1 != b2 {
                printf(
                    b"Number of dark modules is incorrect.\n\0" as *const u8
                        as *const libc::c_char,
                );
                printf(
                    b"Return value: %d, dark modules in frame: %d, should be: %d\n\0"
                        as *const u8 as *const libc::c_char,
                    blacks,
                    b1,
                    b2,
                );
                testEnd(1 as libc::c_int);
                return;
            }
        }
        free(frame as *mut libc::c_void);
    }
    testEnd(0 as libc::c_int);
}
pub static mut m1pat: [[libc::c_uint; 21]; 8] = [
    [
        2082687 as libc::c_uint,
        1072193 as libc::c_uint,
        1526877 as libc::c_uint,
        1526621 as libc::c_uint,
        1530717 as libc::c_uint,
        1065537 as libc::c_uint,
        2086271 as libc::c_uint,
        0 as libc::c_uint,
        1393938 as libc::c_uint,
        1709730 as libc::c_uint,
        227054 as libc::c_uint,
        1682354 as libc::c_uint,
        323297 as libc::c_uint,
        5186 as libc::c_uint,
        2081041 as libc::c_uint,
        1066059 as libc::c_uint,
        1531229 as libc::c_uint,
        1526446 as libc::c_uint,
        1530597 as libc::c_uint,
        1065912 as libc::c_uint,
        2085605 as libc::c_uint,
    ],
    [
        2088319 as libc::c_uint,
        1066561 as libc::c_uint,
        1529437 as libc::c_uint,
        1524061 as libc::c_uint,
        1524061 as libc::c_uint,
        1071169 as libc::c_uint,
        2086271 as libc::c_uint,
        2560 as libc::c_uint,
        1339173 as libc::c_uint,
        1096712 as libc::c_uint,
        646212 as libc::c_uint,
        1245464 as libc::c_uint,
        934987 as libc::c_uint,
        7912 as libc::c_uint,
        2087867 as libc::c_uint,
        1068769 as libc::c_uint,
        1525751 as libc::c_uint,
        1523716 as libc::c_uint,
        1527887 as libc::c_uint,
        1067282 as libc::c_uint,
        2086991 as libc::c_uint,
    ],
    [
        2083711 as libc::c_uint,
        1068865 as libc::c_uint,
        1527901 as libc::c_uint,
        1529949 as libc::c_uint,
        1529693 as libc::c_uint,
        1069377 as libc::c_uint,
        2086271 as libc::c_uint,
        4864 as libc::c_uint,
        1558908 as libc::c_uint,
        177452 as libc::c_uint,
        289439 as libc::c_uint,
        67644 as libc::c_uint,
        258704 as libc::c_uint,
        6092 as libc::c_uint,
        2084192 as libc::c_uint,
        1071045 as libc::c_uint,
        1528108 as libc::c_uint,
        1530144 as libc::c_uint,
        1529492 as libc::c_uint,
        1065014 as libc::c_uint,
        2088596 as libc::c_uint,
    ],
    [
        2087807 as libc::c_uint,
        1070145 as libc::c_uint,
        1527133 as libc::c_uint,
        1529949 as libc::c_uint,
        1526877 as libc::c_uint,
        1068097 as libc::c_uint,
        2086271 as libc::c_uint,
        6144 as libc::c_uint,
        1500235 as libc::c_uint,
        177452 as libc::c_uint,
        1241586 as libc::c_uint,
        1713546 as libc::c_uint,
        258704 as libc::c_uint,
        7329 as libc::c_uint,
        2085078 as libc::c_uint,
        1071045 as libc::c_uint,
        1526337 as libc::c_uint,
        1528982 as libc::c_uint,
        1529492 as libc::c_uint,
        1067867 as libc::c_uint,
        2085666 as libc::c_uint,
    ],
    [
        2085759 as libc::c_uint,
        1066817 as libc::c_uint,
        1525597 as libc::c_uint,
        1531741 as libc::c_uint,
        1531741 as libc::c_uint,
        1071425 as libc::c_uint,
        2086271 as libc::c_uint,
        5120 as libc::c_uint,
        1143289 as libc::c_uint,
        822735 as libc::c_uint,
        1437059 as libc::c_uint,
        1085216 as libc::c_uint,
        903795 as libc::c_uint,
        7983 as libc::c_uint,
        2087548 as libc::c_uint,
        1065177 as libc::c_uint,
        1530319 as libc::c_uint,
        1524163 as libc::c_uint,
        1524104 as libc::c_uint,
        1066794 as libc::c_uint,
        2086519 as libc::c_uint,
    ],
    [
        2084223 as libc::c_uint,
        1070913 as libc::c_uint,
        1527901 as libc::c_uint,
        1528925 as libc::c_uint,
        1525597 as libc::c_uint,
        1067329 as libc::c_uint,
        2086271 as libc::c_uint,
        6912 as libc::c_uint,
        1071566 as libc::c_uint,
        371037 as libc::c_uint,
        289439 as libc::c_uint,
        196636 as libc::c_uint,
        934987 as libc::c_uint,
        8172 as libc::c_uint,
        2084192 as libc::c_uint,
        1067956 as libc::c_uint,
        1524012 as libc::c_uint,
        1523968 as libc::c_uint,
        1523791 as libc::c_uint,
        1067030 as libc::c_uint,
        2088596 as libc::c_uint,
    ],
    [
        2088319 as libc::c_uint,
        1070913 as libc::c_uint,
        1528925 as libc::c_uint,
        1524829 as libc::c_uint,
        1529181 as libc::c_uint,
        1068865 as libc::c_uint,
        2086271 as libc::c_uint,
        2816 as libc::c_uint,
        1310103 as libc::c_uint,
        371037 as libc::c_uint,
        63702 as libc::c_uint,
        165380 as libc::c_uint,
        934987 as libc::c_uint,
        7983 as libc::c_uint,
        2087410 as libc::c_uint,
        1072052 as libc::c_uint,
        1528677 as libc::c_uint,
        1529624 as libc::c_uint,
        1523791 as libc::c_uint,
        1067221 as libc::c_uint,
        2087430 as libc::c_uint,
    ],
    [
        2082687 as libc::c_uint,
        1067073 as libc::c_uint,
        1527389 as libc::c_uint,
        1526621 as libc::c_uint,
        1527645 as libc::c_uint,
        1069121 as libc::c_uint,
        2086271 as libc::c_uint,
        1024 as libc::c_uint,
        1234848 as libc::c_uint,
        1709730 as libc::c_uint,
        676476 as libc::c_uint,
        1915387 as libc::c_uint,
        323297 as libc::c_uint,
        4304 as libc::c_uint,
        2081624 as libc::c_uint,
        1070155 as libc::c_uint,
        1526223 as libc::c_uint,
        1530087 as libc::c_uint,
        1526501 as libc::c_uint,
        1066794 as libc::c_uint,
        2085036 as libc::c_uint,
    ],
];
unsafe extern "C" fn test_encode() {
    let mut current_block: u64;
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut num: [libc::c_char; 9] = [0; 9];
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut err: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    num[0 as libc::c_int as usize] = '0' as i32 as libc::c_char;
    num[1 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    num[2 as libc::c_int as usize] = '2' as i32 as libc::c_char;
    num[3 as libc::c_int as usize] = '3' as i32 as libc::c_char;
    num[4 as libc::c_int as usize] = '4' as i32 as libc::c_char;
    num[5 as libc::c_int as usize] = '5' as i32 as libc::c_char;
    num[6 as libc::c_int as usize] = '6' as i32 as libc::c_char;
    num[7 as libc::c_int as usize] = '7' as i32 as libc::c_char;
    num[8 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    err = 0 as libc::c_int;
    testStartReal(
        b"test_encode\0" as *const u8 as *const libc::c_char,
        b"Test encode (1-M)\0" as *const u8 as *const libc::c_char,
    );
    stream = QRinput_new();
    if !(stream as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
        QRinput_append(
            stream,
            QR_MODE_NUM,
            8 as libc::c_int,
            num.as_mut_ptr() as *mut libc::c_uchar as *const libc::c_uchar,
        );
        mask = 0 as libc::c_int;
        loop {
            if !(mask < 8 as libc::c_int) {
                current_block = 7205609094909031804;
                break;
            }
            QRinput_setVersion(stream, 1 as libc::c_int);
            QRinput_setErrorCorrectionLevel(stream, QR_ECLEVEL_M);
            qrcode = QRcode_encodeMask(stream, mask);
            if qrcode as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                current_block = 9439336792921840085;
                break;
            }
            w = (*qrcode).width;
            frame = (*qrcode).data;
            y = 0 as libc::c_int;
            while y < w {
                x = 0 as libc::c_int;
                while x < w {
                    if m1pat[mask as usize][y as usize] >> 20 as libc::c_int - x
                        & 1 as libc::c_uint
                        != (*frame.offset((y * w + x) as isize) as libc::c_int
                            & 1 as libc::c_int) as libc::c_uint
                    {
                        printf(
                            b"Diff in mask=%d (%d,%d)\n\0" as *const u8
                                as *const libc::c_char,
                            mask,
                            x,
                            y,
                        );
                        err += 1;
                    }
                    x += 1;
                }
                y += 1;
            }
            QRcode_free(qrcode);
            mask += 1;
        }
        match current_block {
            9439336792921840085 => {}
            _ => {
                QRinput_free(stream);
            }
        }
    }
    testEnd(err);
}
unsafe extern "C" fn test_encode2() {
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    testStartReal(
        b"test_encode2\0" as *const u8 as *const libc::c_char,
        b"Test encode (2-H) (no padding test)\0" as *const u8 as *const libc::c_char,
    );
    qrcode = QRcode_encodeString(
        b"abcdefghijk123456789012\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        QR_ECLEVEL_H,
        QR_MODE_8,
        0 as libc::c_int,
    );
    testEnd(!((*qrcode).version == 2 as libc::c_int) as libc::c_int);
    QRcode_free(qrcode);
}
unsafe extern "C" fn test_encode3() {
    let mut code1: *mut QRcode = 0 as *mut QRcode;
    let mut code2: *mut QRcode = 0 as *mut QRcode;
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut tmp: libc::c_int = 0;
    testStartReal(
        b"test_encode3\0" as *const u8 as *const libc::c_char,
        b"Compare encodeString and encodeInput\0" as *const u8 as *const libc::c_char,
    );
    code1 = QRcode_encodeString(
        b"0123456\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        QR_ECLEVEL_L,
        QR_MODE_8,
        0 as libc::c_int,
    );
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    QRinput_append(
        input,
        QR_MODE_NUM,
        7 as libc::c_int,
        b"0123456\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar
            as *const libc::c_uchar,
    );
    code2 = QRcode_encodeInput(input);
    tmp = memcmp(
        (*code1).data as *const libc::c_void,
        (*code2).data as *const libc::c_void,
        ((*code1).width * (*code1).width) as size_t,
    );
    testEnd(tmp);
    QRcode_free(code1);
    QRcode_free(code2);
    QRinput_free(input);
}
unsafe extern "C" fn test_encodeNull() {
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    testStartReal(
        b"test_encodeNull\0" as *const u8 as *const libc::c_char,
        b"Test encode NULL.\0" as *const u8 as *const libc::c_char,
    );
    qrcode = QRcode_encodeString(
        0 as *mut libc::c_void as *const libc::c_char,
        0 as libc::c_int,
        QR_ECLEVEL_H,
        QR_MODE_8,
        0 as libc::c_int,
    );
    assertionNum += 1;
    if !(qrcode as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
        assertionFailed += 1;
        printf(
            b"QRcode_encodeString() returned something.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    testFinish();
    if qrcode as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        QRcode_free(qrcode);
    }
}
unsafe extern "C" fn test_encodeEmpty() {
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    testStartReal(
        b"test_encodeEmpty\0" as *const u8 as *const libc::c_char,
        b"Test encode an empty string.\0" as *const u8 as *const libc::c_char,
    );
    qrcode = QRcode_encodeString(
        b"\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        QR_ECLEVEL_H,
        QR_MODE_8,
        0 as libc::c_int,
    );
    assertionNum += 1;
    if !(qrcode as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
        assertionFailed += 1;
        printf(
            b"QRcode_encodeString() returned something.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    testFinish();
    if qrcode as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        QRcode_free(qrcode);
    }
}
unsafe extern "C" fn test_encodeNull8() {
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    testStartReal(
        b"test_encodeNull8\0" as *const u8 as *const libc::c_char,
        b"Test encode NULL.\0" as *const u8 as *const libc::c_char,
    );
    qrcode = QRcode_encodeString8bit(
        0 as *mut libc::c_void as *const libc::c_char,
        0 as libc::c_int,
        QR_ECLEVEL_H,
    );
    assertionNum += 1;
    if !(qrcode as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
        assertionFailed += 1;
        printf(
            b"QRcode_encodeString8bit() returned something.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    testFinish();
    if qrcode as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        QRcode_free(qrcode);
    }
}
unsafe extern "C" fn test_encodeEmpty8() {
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    testStartReal(
        b"test_encodeEmpty8\0" as *const u8 as *const libc::c_char,
        b"Test encode an empty string.\0" as *const u8 as *const libc::c_char,
    );
    qrcode = QRcode_encodeString8bit(
        b"\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        QR_ECLEVEL_H,
    );
    assertionNum += 1;
    if !(qrcode as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
        assertionFailed += 1;
        printf(
            b"QRcode_encodeString8bit() returned something.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    testFinish();
    if qrcode as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        QRcode_free(qrcode);
    }
}
unsafe extern "C" fn test_encodeLongData() {
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut data: [libc::c_uchar; 7090] = [0; 7090];
    let mut maxlength: [[libc::c_int; 4]; 4] = [[0; 4]; 4];
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    maxlength[0 as libc::c_int
        as usize][0 as libc::c_int as usize] = 7089 as libc::c_int;
    maxlength[0 as libc::c_int
        as usize][1 as libc::c_int as usize] = 5596 as libc::c_int;
    maxlength[0 as libc::c_int
        as usize][2 as libc::c_int as usize] = 3993 as libc::c_int;
    maxlength[0 as libc::c_int
        as usize][3 as libc::c_int as usize] = 3057 as libc::c_int;
    maxlength[1 as libc::c_int
        as usize][0 as libc::c_int as usize] = 4296 as libc::c_int;
    maxlength[1 as libc::c_int
        as usize][1 as libc::c_int as usize] = 3391 as libc::c_int;
    maxlength[1 as libc::c_int
        as usize][2 as libc::c_int as usize] = 2420 as libc::c_int;
    maxlength[1 as libc::c_int
        as usize][3 as libc::c_int as usize] = 1852 as libc::c_int;
    maxlength[2 as libc::c_int
        as usize][0 as libc::c_int as usize] = 2953 as libc::c_int;
    maxlength[2 as libc::c_int
        as usize][1 as libc::c_int as usize] = 2331 as libc::c_int;
    maxlength[2 as libc::c_int
        as usize][2 as libc::c_int as usize] = 1663 as libc::c_int;
    maxlength[2 as libc::c_int
        as usize][3 as libc::c_int as usize] = 1273 as libc::c_int;
    maxlength[3 as libc::c_int
        as usize][0 as libc::c_int as usize] = 3634 as libc::c_int;
    maxlength[3 as libc::c_int
        as usize][1 as libc::c_int as usize] = 2870 as libc::c_int;
    maxlength[3 as libc::c_int
        as usize][2 as libc::c_int as usize] = 2048 as libc::c_int;
    maxlength[3 as libc::c_int
        as usize][3 as libc::c_int as usize] = 1568 as libc::c_int;
    testStartReal(
        b"test_encodeLongData\0" as *const u8 as *const libc::c_char,
        b"Encoding long data.\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i <= 3 as libc::c_int {
        if i != 3 as libc::c_int {
            memset(
                data.as_mut_ptr() as *mut libc::c_void,
                '0' as i32,
                (maxlength[i as usize][0 as libc::c_int as usize] + 1 as libc::c_int)
                    as size_t,
            );
        } else {
            l = 0 as libc::c_int;
            while l
                <= maxlength[i as usize][0 as libc::c_int as usize] / 2 as libc::c_int
                    + 1 as libc::c_int
            {
                data[(l * 2 as libc::c_int)
                    as usize] = 147 as libc::c_int as libc::c_uchar;
                data[(l * 2 as libc::c_int + 1 as libc::c_int)
                    as usize] = 95 as libc::c_int as libc::c_uchar;
                l += 1;
            }
        }
        l = 0 as libc::c_int;
        while l <= 3 as libc::c_int {
            stream = QRinput_new2(0 as libc::c_int, l as QRecLevel);
            ret = QRinput_append(
                stream,
                i as QRencodeMode,
                maxlength[i as usize][l as usize],
                data.as_mut_ptr() as *const libc::c_uchar,
            );
            assertionNum += 1;
            if !(ret == 0 as libc::c_int) {
                assertionFailed += 1;
                printf(
                    b"Failed to add %d-byte %s to a QRinput\n\0" as *const u8
                        as *const libc::c_char,
                    maxlength[i as usize][l as usize],
                    modeStr[i as usize],
                );
            }
            qrcode = QRcode_encodeInput(stream);
            assertionNum += 1;
            if !(qrcode as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
                assertionFailed += 1;
                printf(
                    b"(QRcode_encodeInput) failed to encode %d-byte %s in level %d.\n\0"
                        as *const u8 as *const libc::c_char,
                    maxlength[i as usize][l as usize],
                    modeStr[i as usize],
                    l,
                );
            }
            if qrcode as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                QRcode_free(qrcode);
            }
            QRinput_free(stream);
            stream = QRinput_new2(0 as libc::c_int, l as QRecLevel);
            len = maxlength[i as usize][l as usize];
            if i == 3 as libc::c_int {
                len += 2 as libc::c_int;
            } else {
                len += 1;
            }
            ret = QRinput_append(
                stream,
                i as QRencodeMode,
                len,
                data.as_mut_ptr() as *const libc::c_uchar,
            );
            if ret == 0 as libc::c_int {
                qrcode = QRcode_encodeInput(stream);
                assertionNum += 1;
                if !(qrcode as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong)
                {
                    assertionFailed += 1;
                    printf(
                        b"(QRcode_encodeInput) incorrectly succeeded to encode %d-byte %s in level %d.\n\0"
                            as *const u8 as *const libc::c_char,
                        len,
                        modeStr[i as usize],
                        l,
                    );
                }
                if qrcode as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                    printf(
                        b"version: %d\n\0" as *const u8 as *const libc::c_char,
                        (*qrcode).version,
                    );
                    QRcode_free(qrcode);
                }
            }
            QRinput_free(stream);
            l += 1;
        }
        i += 1;
    }
    testFinish();
}
unsafe extern "C" fn test_encodeVer26Num() {
    let mut data: [libc::c_char; 3284] = [0; 3284];
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    testStartReal(
        b"test_encodeVer26Num\0" as *const u8 as *const libc::c_char,
        b"Encoding 3283 digits number. (issue #160)\0" as *const u8
            as *const libc::c_char,
    );
    memset(
        data.as_mut_ptr() as *mut libc::c_void,
        '0' as i32,
        3283 as libc::c_int as size_t,
    );
    data[3283 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    qrcode = QRcode_encodeString(
        data.as_mut_ptr() as *const libc::c_char,
        0 as libc::c_int,
        QR_ECLEVEL_L,
        QR_MODE_8,
        0 as libc::c_int,
    );
    assertionNum += 1;
    if !(qrcode as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        assertionFailed += 1;
        printf(
            b"(QRcode_encodeString) failed to encode 3283 digits number in level L.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    assertionNum += 1;
    if !((*qrcode).version == 26 as libc::c_int) {
        assertionFailed += 1;
        printf(
            b"version number is %d (26 expected)\n\0" as *const u8
                as *const libc::c_char,
            (*qrcode).version,
        );
    }
    if qrcode as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        QRcode_free(qrcode);
    }
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(
        data.as_mut_ptr() as *const libc::c_char,
        input,
        QR_MODE_8,
        0 as libc::c_int,
    );
    list = (*input).head;
    assertionNum += 1;
    if !((*list).size == 3283 as libc::c_int) {
        assertionFailed += 1;
        printf(
            b"chunk size is wrong. (%d, 3283 expected)\n\0" as *const u8
                as *const libc::c_char,
            (*list).size,
        );
    }
    QRinput_free(input);
    testFinish();
}
unsafe extern "C" fn test_01234567() {
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut num: [libc::c_char; 9] = [0; 9];
    let mut i: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    let mut correct: [libc::c_uchar; 441] = [0; 441];
    num[0 as libc::c_int as usize] = '0' as i32 as libc::c_char;
    num[1 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    num[2 as libc::c_int as usize] = '2' as i32 as libc::c_char;
    num[3 as libc::c_int as usize] = '3' as i32 as libc::c_char;
    num[4 as libc::c_int as usize] = '4' as i32 as libc::c_char;
    num[5 as libc::c_int as usize] = '5' as i32 as libc::c_char;
    num[6 as libc::c_int as usize] = '6' as i32 as libc::c_char;
    num[7 as libc::c_int as usize] = '7' as i32 as libc::c_char;
    num[8 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    err = 0 as libc::c_int;
    correct[0 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[1 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[2 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[3 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[4 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[5 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[6 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[7 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[8 as libc::c_int as usize] = 132 as libc::c_int as libc::c_uchar;
    correct[9 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[10 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[11 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[12 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[13 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[14 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[15 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[16 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[17 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[18 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[19 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[20 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[21 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[22 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[23 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[24 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[25 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[26 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[27 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[28 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[29 as libc::c_int as usize] = 132 as libc::c_int as libc::c_uchar;
    correct[30 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[31 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[32 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[33 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[34 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[35 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[36 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[37 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[38 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[39 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[40 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[41 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[42 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[43 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[44 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[45 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[46 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[47 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[48 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[49 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[50 as libc::c_int as usize] = 133 as libc::c_int as libc::c_uchar;
    correct[51 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[52 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[53 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[54 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[55 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[56 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[57 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[58 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[59 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[60 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[61 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[62 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[63 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[64 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[65 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[66 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[67 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[68 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[69 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[70 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[71 as libc::c_int as usize] = 133 as libc::c_int as libc::c_uchar;
    correct[72 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[73 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[74 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[75 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[76 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[77 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[78 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[79 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[80 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[81 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[82 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[83 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[84 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[85 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[86 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[87 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[88 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[89 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[90 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[91 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[92 as libc::c_int as usize] = 133 as libc::c_int as libc::c_uchar;
    correct[93 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[94 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[95 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[96 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[97 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[98 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[99 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[100 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[101 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[102 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[103 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[104 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[105 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[106 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[107 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[108 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[109 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[110 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[111 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[112 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[113 as libc::c_int as usize] = 133 as libc::c_int as libc::c_uchar;
    correct[114 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[115 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[116 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[117 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[118 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[119 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[120 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[121 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[122 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[123 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[124 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[125 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[126 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[127 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[128 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[129 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[130 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[131 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[132 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[133 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[134 as libc::c_int as usize] = 145 as libc::c_int as libc::c_uchar;
    correct[135 as libc::c_int as usize] = 144 as libc::c_int as libc::c_uchar;
    correct[136 as libc::c_int as usize] = 145 as libc::c_int as libc::c_uchar;
    correct[137 as libc::c_int as usize] = 144 as libc::c_int as libc::c_uchar;
    correct[138 as libc::c_int as usize] = 145 as libc::c_int as libc::c_uchar;
    correct[139 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[140 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[141 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[142 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[143 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[144 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[145 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[146 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[147 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[148 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[149 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[150 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[151 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[152 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[153 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[154 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[155 as libc::c_int as usize] = 133 as libc::c_int as libc::c_uchar;
    correct[156 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[157 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[158 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[159 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[160 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[161 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[162 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[163 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[164 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[165 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[166 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[167 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[168 as libc::c_int as usize] = 133 as libc::c_int as libc::c_uchar;
    correct[169 as libc::c_int as usize] = 132 as libc::c_int as libc::c_uchar;
    correct[170 as libc::c_int as usize] = 133 as libc::c_int as libc::c_uchar;
    correct[171 as libc::c_int as usize] = 133 as libc::c_int as libc::c_uchar;
    correct[172 as libc::c_int as usize] = 133 as libc::c_int as libc::c_uchar;
    correct[173 as libc::c_int as usize] = 133 as libc::c_int as libc::c_uchar;
    correct[174 as libc::c_int as usize] = 145 as libc::c_int as libc::c_uchar;
    correct[175 as libc::c_int as usize] = 132 as libc::c_int as libc::c_uchar;
    correct[176 as libc::c_int as usize] = 132 as libc::c_int as libc::c_uchar;
    correct[177 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[178 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[179 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[180 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[181 as libc::c_int as usize] = 132 as libc::c_int as libc::c_uchar;
    correct[182 as libc::c_int as usize] = 133 as libc::c_int as libc::c_uchar;
    correct[183 as libc::c_int as usize] = 133 as libc::c_int as libc::c_uchar;
    correct[184 as libc::c_int as usize] = 133 as libc::c_int as libc::c_uchar;
    correct[185 as libc::c_int as usize] = 133 as libc::c_int as libc::c_uchar;
    correct[186 as libc::c_int as usize] = 133 as libc::c_int as libc::c_uchar;
    correct[187 as libc::c_int as usize] = 132 as libc::c_int as libc::c_uchar;
    correct[188 as libc::c_int as usize] = 132 as libc::c_int as libc::c_uchar;
    correct[189 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[190 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[191 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[192 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[193 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[194 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[195 as libc::c_int as usize] = 144 as libc::c_int as libc::c_uchar;
    correct[196 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[197 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[198 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[199 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[200 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[201 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[202 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[203 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[204 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[205 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[206 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[207 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[208 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[209 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[210 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[211 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[212 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[213 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[214 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[215 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[216 as libc::c_int as usize] = 145 as libc::c_int as libc::c_uchar;
    correct[217 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[218 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[219 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[220 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[221 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[222 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[223 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[224 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[225 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[226 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[227 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[228 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[229 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[230 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[231 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[232 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[233 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[234 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[235 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[236 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[237 as libc::c_int as usize] = 144 as libc::c_int as libc::c_uchar;
    correct[238 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[239 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[240 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[241 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[242 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[243 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[244 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[245 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[246 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[247 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[248 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[249 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[250 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[251 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[252 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[253 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[254 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[255 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[256 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[257 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[258 as libc::c_int as usize] = 145 as libc::c_int as libc::c_uchar;
    correct[259 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[260 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[261 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[262 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[263 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[264 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[265 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[266 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[267 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[268 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[269 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[270 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[271 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[272 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[273 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[274 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[275 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[276 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[277 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[278 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[279 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[280 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[281 as libc::c_int as usize] = 129 as libc::c_int as libc::c_uchar;
    correct[282 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[283 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[284 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[285 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[286 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[287 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[288 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[289 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[290 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[291 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[292 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[293 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[294 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[295 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[296 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[297 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[298 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[299 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[300 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[301 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[302 as libc::c_int as usize] = 132 as libc::c_int as libc::c_uchar;
    correct[303 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[304 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[305 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[306 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[307 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[308 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[309 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[310 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[311 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[312 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[313 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[314 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[315 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[316 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[317 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[318 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[319 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[320 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[321 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[322 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[323 as libc::c_int as usize] = 133 as libc::c_int as libc::c_uchar;
    correct[324 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[325 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[326 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[327 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[328 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[329 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[330 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[331 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[332 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[333 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[334 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[335 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[336 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[337 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[338 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[339 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[340 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[341 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[342 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[343 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[344 as libc::c_int as usize] = 133 as libc::c_int as libc::c_uchar;
    correct[345 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[346 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[347 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[348 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[349 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[350 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[351 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[352 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[353 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[354 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[355 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[356 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[357 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[358 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[359 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[360 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[361 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[362 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[363 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[364 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[365 as libc::c_int as usize] = 133 as libc::c_int as libc::c_uchar;
    correct[366 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[367 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[368 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[369 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[370 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[371 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[372 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[373 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[374 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[375 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[376 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[377 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[378 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[379 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[380 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[381 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[382 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[383 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[384 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[385 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[386 as libc::c_int as usize] = 133 as libc::c_int as libc::c_uchar;
    correct[387 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[388 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[389 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[390 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[391 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[392 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[393 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[394 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[395 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[396 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[397 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[398 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[399 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[400 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[401 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[402 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[403 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[404 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[405 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[406 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[407 as libc::c_int as usize] = 132 as libc::c_int as libc::c_uchar;
    correct[408 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[409 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    correct[410 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[411 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[412 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[413 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[414 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[415 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[416 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[417 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[418 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[419 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[420 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[421 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[422 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[423 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[424 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[425 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[426 as libc::c_int as usize] = 193 as libc::c_int as libc::c_uchar;
    correct[427 as libc::c_int as usize] = 192 as libc::c_int as libc::c_uchar;
    correct[428 as libc::c_int as usize] = 133 as libc::c_int as libc::c_uchar;
    correct[429 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[430 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    correct[431 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[432 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[433 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[434 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[435 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[436 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[437 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[438 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    correct[439 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    correct[440 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    testStartReal(
        b"test_01234567\0" as *const u8 as *const libc::c_char,
        b"Encode 01234567 in 1-M\0" as *const u8 as *const libc::c_char,
    );
    stream = QRinput_new2(1 as libc::c_int, QR_ECLEVEL_M);
    QRinput_append(
        stream,
        QR_MODE_NUM,
        8 as libc::c_int,
        num.as_mut_ptr() as *mut libc::c_uchar as *const libc::c_uchar,
    );
    qrcode = QRcode_encodeInput(stream);
    i = 0 as libc::c_int;
    while i < (*qrcode).width * (*qrcode).width {
        if *((*qrcode).data).offset(i as isize) as libc::c_int
            != correct[i as usize] as libc::c_int
        {
            err += 1;
        }
        i += 1;
    }
    testEnd(err);
    QRinput_free(stream);
    QRcode_free(qrcode);
}
unsafe extern "C" fn print_01234567() {
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut num: [libc::c_char; 9] = [0; 9];
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    num[0 as libc::c_int as usize] = '0' as i32 as libc::c_char;
    num[1 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    num[2 as libc::c_int as usize] = '2' as i32 as libc::c_char;
    num[3 as libc::c_int as usize] = '3' as i32 as libc::c_char;
    num[4 as libc::c_int as usize] = '4' as i32 as libc::c_char;
    num[5 as libc::c_int as usize] = '5' as i32 as libc::c_char;
    num[6 as libc::c_int as usize] = '6' as i32 as libc::c_char;
    num[7 as libc::c_int as usize] = '7' as i32 as libc::c_char;
    num[8 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    puts(b"\nPrinting QR code of '01234567'.\0" as *const u8 as *const libc::c_char);
    stream = QRinput_new2(1 as libc::c_int, QR_ECLEVEL_M);
    QRinput_append(
        stream,
        QR_MODE_NUM,
        8 as libc::c_int,
        num.as_mut_ptr() as *mut libc::c_uchar as *const libc::c_uchar,
    );
    qrcode = QRcode_encodeInput(stream);
    printQRcode(qrcode);
    QRinput_free(stream);
    QRcode_free(qrcode);
}
unsafe extern "C" fn test_invalid_input() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut code: *mut QRcode = 0 as *mut QRcode;
    testStartReal(
        b"test_invalid_input\0" as *const u8 as *const libc::c_char,
        b"Testing invalid input.\0" as *const u8 as *const libc::c_char,
    );
    input = QRinput_new();
    QRinput_append(
        input,
        QR_MODE_AN,
        5 as libc::c_int,
        b"TEST1\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar
            as *const libc::c_uchar,
    );
    (*input).version = -(1 as libc::c_int);
    (*input).level = QR_ECLEVEL_L;
    code = QRcode_encodeInput(input);
    assertionNum += 1;
    if !(code as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
        assertionFailed += 1;
        printf(
            b"invalid version(-1)  was not checked.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if code as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        QRcode_free(code);
    }
    (*input).version = 41 as libc::c_int;
    (*input).level = QR_ECLEVEL_L;
    code = QRcode_encodeInput(input);
    assertionNum += 1;
    if !(code as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
        assertionFailed += 1;
        printf(
            b"invalid version(41) access was not checked.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if code as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        QRcode_free(code);
    }
    (*input).version = 1 as libc::c_int;
    (*input).level = 4 as QRecLevel;
    code = QRcode_encodeInput(input);
    assertionNum += 1;
    if !(code as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
        assertionFailed += 1;
        printf(
            b"invalid level(H+1) access was not checked.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if code as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        QRcode_free(code);
    }
    (*input).version = 1 as libc::c_int;
    (*input).level = 4294967295 as QRecLevel;
    code = QRcode_encodeInput(input);
    assertionNum += 1;
    if !(code as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
        assertionFailed += 1;
        printf(
            b"invalid level(-1) access was not checked.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if code as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        QRcode_free(code);
    }
    QRinput_free(input);
    testFinish();
}
unsafe extern "C" fn test_struct_semilong() {
    let mut codes: *mut QRcode_List = 0 as *mut QRcode_List;
    let mut list: *mut QRcode_List = 0 as *mut QRcode_List;
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    let mut num: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    str = b"asdfasdfasdfasdfasdfASDFASDASDFASDFAsdfasdfasdfasdASDFASDFADSADadsfasdf\0"
        as *const u8 as *const libc::c_char;
    testStartReal(
        b"test_struct_semilong\0" as *const u8 as *const libc::c_char,
        b"Testing semi-long structured-append symbols\0" as *const u8
            as *const libc::c_char,
    );
    codes = QRcode_encodeString8bitStructured(str, 1 as libc::c_int, QR_ECLEVEL_L);
    list = codes;
    num = 0 as libc::c_int;
    while list as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        num += 1;
        assertionNum += 1;
        if !((*(*list).code).version == 1 as libc::c_int) {
            assertionFailed += 1;
            printf(
                b"version number is %d (1 expected)\n\0" as *const u8
                    as *const libc::c_char,
                (*(*list).code).version,
            );
        }
        list = (*list).next;
    }
    size = QRcode_List_size(codes);
    assertionNum += 1;
    if !(num == size) {
        assertionFailed += 1;
        printf(
            b"QRcode_List_size returns wrong size?\0" as *const u8 as *const libc::c_char,
        );
    }
    QRcode_List_free(codes);
    codes = QRcode_encodeStringStructured(
        str,
        1 as libc::c_int,
        QR_ECLEVEL_L,
        QR_MODE_8,
        1 as libc::c_int,
    );
    list = codes;
    num = 0 as libc::c_int;
    while list as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        num += 1;
        assertionNum += 1;
        if !((*(*list).code).version == 1 as libc::c_int) {
            assertionFailed += 1;
            printf(
                b"version number is %d (1 expected)\n\0" as *const u8
                    as *const libc::c_char,
                (*(*list).code).version,
            );
        }
        list = (*list).next;
    }
    size = QRcode_List_size(codes);
    assertionNum += 1;
    if !(num == size) {
        assertionFailed += 1;
        printf(
            b"QRcode_List_size returns wrong size?\0" as *const u8 as *const libc::c_char,
        );
    }
    QRcode_List_free(codes);
    testFinish();
}
unsafe extern "C" fn test_struct_example() {
    let mut codes: *mut QRcode_List = 0 as *mut QRcode_List;
    let mut list: *mut QRcode_List = 0 as *mut QRcode_List;
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    let mut num: libc::c_int = 0;
    str = b"an example of four Structured Append symbols,\0" as *const u8
        as *const libc::c_char;
    testStartReal(
        b"test_struct_example\0" as *const u8 as *const libc::c_char,
        b"Testing the example of structured-append symbols\0" as *const u8
            as *const libc::c_char,
    );
    codes = QRcode_encodeString8bitStructured(str, 1 as libc::c_int, QR_ECLEVEL_M);
    list = codes;
    num = 0 as libc::c_int;
    while list as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        num += 1;
        assertionNum += 1;
        if !((*(*list).code).version == 1 as libc::c_int) {
            assertionFailed += 1;
            printf(
                b"version number is %d (1 expected)\n\0" as *const u8
                    as *const libc::c_char,
                (*(*list).code).version,
            );
        }
        list = (*list).next;
    }
    assertionNum += 1;
    if !(num == 4 as libc::c_int) {
        assertionFailed += 1;
        printf(
            b"number of symbols is %d (4 expected).\0" as *const u8
                as *const libc::c_char,
            num,
        );
    }
    testFinish();
    QRcode_List_free(codes);
}
unsafe extern "C" fn test_null_free() {
    testStartReal(
        b"test_null_free\0" as *const u8 as *const libc::c_char,
        b"Testing free NULL pointers\0" as *const u8 as *const libc::c_char,
    );
    printf(b"Check QRcode_free(NULL).\n\0" as *const u8 as *const libc::c_char);
    QRcode_free(0 as *mut libc::c_void as *mut QRcode);
    printf(b"Check QRcode_List_free(NULL).\n\0" as *const u8 as *const libc::c_char);
    QRcode_List_free(0 as *mut libc::c_void as *mut QRcode_List);
    printf(b"Check QRraw_free(NULL).\n\0" as *const u8 as *const libc::c_char);
    QRraw_free(0 as *mut libc::c_void as *mut QRRawCode);
    testFinish();
}
unsafe extern "C" fn test_encodeTooLongMQR() {
    let mut code: *mut QRcode = 0 as *mut QRcode;
    let mut data: [*mut libc::c_char; 4] = [0 as *mut libc::c_char; 4];
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    data[0 as libc::c_int
        as usize] = b"012345\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    data[1 as libc::c_int
        as usize] = b"ABC0EFG\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    data[2 as libc::c_int
        as usize] = b"0123456789\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    data[3 as libc::c_int
        as usize] = b"0123456789ABCDEFG\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    testStartReal(
        b"test_encodeTooLongMQR\0" as *const u8 as *const libc::c_char,
        b"Encode too large data for MQR.\0" as *const u8 as *const libc::c_char,
    );
    code = QRcode_encodeStringMQR(
        data[0 as libc::c_int as usize] as *const libc::c_char,
        1 as libc::c_int,
        QR_ECLEVEL_L,
        QR_MODE_8,
        0 as libc::c_int,
    );
    assertionNum += 1;
    if !(code as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        assertionFailed += 1;
        printf(
            b"6 byte length numeric string should be accepted to version 2 or larger.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    assertionNum += 1;
    if !((*code).version == 2 as libc::c_int) {
        assertionFailed += 1;
        printf(
            b"6 byte length numeric string should be accepted to version 2.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    code = QRcode_encodeStringMQR(
        data[1 as libc::c_int as usize] as *const libc::c_char,
        2 as libc::c_int,
        QR_ECLEVEL_L,
        QR_MODE_8,
        0 as libc::c_int,
    );
    assertionNum += 1;
    if !(code as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        assertionFailed += 1;
        printf(
            b"7 byte length alphanumeric string should be accepted to version 3 or larger.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    assertionNum += 1;
    if !((*code).version == 3 as libc::c_int) {
        assertionFailed += 1;
        printf(
            b"7 byte length alphanumeric string should be accepted to version 3.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    code = QRcode_encodeString8bitMQR(
        data[2 as libc::c_int as usize] as *const libc::c_char,
        3 as libc::c_int,
        QR_ECLEVEL_L,
    );
    assertionNum += 1;
    if !(code as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        assertionFailed += 1;
        printf(
            b"9 byte length 8bit string should be accepted to version 4.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    assertionNum += 1;
    if !((*code).version == 4 as libc::c_int) {
        assertionFailed += 1;
        printf(
            b"9 byte length 8bit string should be accepted to version 4.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    code = QRcode_encodeString8bitMQR(
        data[3 as libc::c_int as usize] as *const libc::c_char,
        4 as libc::c_int,
        QR_ECLEVEL_L,
    );
    assertionNum += 1;
    if !(code as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
        assertionFailed += 1;
        printf(
            b"16 byte length 8bit string was accepted to version 4.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    assertionNum += 1;
    tmp = __errno_location();
    if !(*tmp == 34 as libc::c_int) {
        assertionFailed += 1;
        printf(b"errno != ERANGE\n\0" as *const u8 as *const libc::c_char);
    }
    testFinish();
    if code as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        printQRcode(code);
        QRcode_free(code);
    }
}
unsafe extern "C" fn test_mqrraw_new() {
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut num: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut datacode: [libc::c_uchar; 3] = [0; 3];
    let mut raw: *mut MQRRawCode = 0 as *mut MQRRawCode;
    let mut tmp: libc::c_int = 0;
    num = b"01234\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    datacode[0 as libc::c_int as usize] = 160 as libc::c_int as libc::c_uchar;
    datacode[1 as libc::c_int as usize] = 98 as libc::c_int as libc::c_uchar;
    datacode[2 as libc::c_int as usize] = 32 as libc::c_int as libc::c_uchar;
    testStartReal(
        b"test_mqrraw_new\0" as *const u8 as *const libc::c_char,
        b"Test MQRRaw_new()\0" as *const u8 as *const libc::c_char,
    );
    stream = QRinput_newMQR(1 as libc::c_int, QR_ECLEVEL_L);
    QRinput_append(
        stream,
        QR_MODE_NUM,
        5 as libc::c_int,
        num as *mut libc::c_uchar as *const libc::c_uchar,
    );
    raw = MQRraw_new(stream);
    assertionNum += 1;
    if !(raw as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        assertionFailed += 1;
        printf(b"Failed MQRraw_new().\n\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    if !((*raw).count == 0 as libc::c_int) {
        assertionFailed += 1;
        printf(
            b"MQRraw.count = %d != 0\n\0" as *const u8 as *const libc::c_char,
            (*raw).count,
        );
    }
    assertionNum += 1;
    if !((*raw).version == 1 as libc::c_int) {
        assertionFailed += 1;
        printf(
            b"MQRraw.version was not as expected. (%d)\n\0" as *const u8
                as *const libc::c_char,
            (*raw).version,
        );
    }
    assertionNum += 1;
    if !((*raw).dataLength == 3 as libc::c_int) {
        assertionFailed += 1;
        printf(
            b"MQRraw.dataLength was not as expected.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    assertionNum += 1;
    if !((*raw).eccLength == 2 as libc::c_int) {
        assertionFailed += 1;
        printf(
            b"MQRraw.eccLength was not as expected.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    assertionNum += 1;
    tmp = memcmp(
        (*raw).datacode as *const libc::c_void,
        datacode.as_mut_ptr() as *const libc::c_void,
        3 as libc::c_int as size_t,
    );
    if !(tmp == 0 as libc::c_int) {
        assertionFailed += 1;
        printf(b"Datacode doesn't match.\n\0" as *const u8 as *const libc::c_char);
    }
    QRinput_free(stream);
    MQRraw_free(raw);
    testFinish();
}
unsafe extern "C" fn test_encodeData() {
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    testStartReal(
        b"test_encodeData\0" as *const u8 as *const libc::c_char,
        b"Test QRencode_encodeData.\0" as *const u8 as *const libc::c_char,
    );
    qrcode = QRcode_encodeData(
        0 as libc::c_int,
        0 as *mut libc::c_void as *const libc::c_uchar,
        0 as libc::c_int,
        QR_ECLEVEL_H,
    );
    assertionNum += 1;
    if !(qrcode as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
        assertionFailed += 1;
        printf(
            b"QRcode_encodeData(NULL, 0) returned something.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if qrcode as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        QRcode_free(qrcode);
    }
    qrcode = QRcode_encodeData(
        10 as libc::c_int,
        b"test\0\0test\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar
            as *const libc::c_uchar,
        0 as libc::c_int,
        QR_ECLEVEL_H,
    );
    assertionNum += 1;
    if !(qrcode as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        assertionFailed += 1;
        printf(b"QRcode_encodeData() failed.\n\0" as *const u8 as *const libc::c_char);
    }
    if qrcode as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        QRcode_free(qrcode);
    }
    testFinish();
}
unsafe extern "C" fn test_formatInfo() {
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    let mut level: QRecLevel = QR_ECLEVEL_L;
    let mut mask: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    testStartReal(
        b"test_formatInfo\0" as *const u8 as *const libc::c_char,
        b"Test format info in QR code.\0" as *const u8 as *const libc::c_char,
    );
    qrcode = QRcode_encodeString(
        b"AC-42\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        QR_ECLEVEL_H,
        QR_MODE_8,
        1 as libc::c_int,
    );
    ret = QRcode_decodeFormat(qrcode, &mut level, &mut mask);
    assertionNum += 1;
    if !(ret == 0 as libc::c_int) {
        assertionFailed += 1;
        printf(b"Failed to decode.\n\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    if !(level as libc::c_uint == 3 as libc::c_uint) {
        assertionFailed += 1;
        printf(b"Decoded format is wrong.\n\0" as *const u8 as *const libc::c_char);
    }
    if qrcode as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        QRcode_free(qrcode);
    }
    testFinish();
}
unsafe extern "C" fn test_formatInfoMQR() {
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    let mut level: QRecLevel = QR_ECLEVEL_L;
    let mut version: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    testStartReal(
        b"test_formatInfoMQR\0" as *const u8 as *const libc::c_char,
        b"Test format info in Micro QR code.\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        qrcode = QRcode_encodeStringMQR(
            b"1\0" as *const u8 as *const libc::c_char,
            MQRformat[i as usize].version,
            MQRformat[i as usize].level,
            QR_MODE_8,
            1 as libc::c_int,
        );
        ret = QRcode_decodeFormatMQR(qrcode, &mut version, &mut level, &mut mask);
        assertionNum += 1;
        if !(ret == 0 as libc::c_int) {
            assertionFailed += 1;
            printf(b"Failed to decode.\n\0" as *const u8 as *const libc::c_char);
        }
        assertionNum += 1;
        if !(MQRformat[i as usize].version == version) {
            assertionFailed += 1;
            printf(b"Decoded verion is wrong.\n\0" as *const u8 as *const libc::c_char);
        }
        assertionNum += 1;
        if !(MQRformat[i as usize].level as libc::c_uint == level as libc::c_uint) {
            assertionFailed += 1;
            printf(b"Decoded level is wrong.\n\0" as *const u8 as *const libc::c_char);
        }
        QRcode_free(qrcode);
        i += 1;
    }
    testFinish();
}
unsafe extern "C" fn test_decodeSimple() {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    let mut qrdata: *mut QRdata = 0 as *mut QRdata;
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: libc::c_int = 0;
    str = b"AC-42\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    testStartReal(
        b"test_decodeSimple\0" as *const u8 as *const libc::c_char,
        b"Test code words.\0" as *const u8 as *const libc::c_char,
    );
    qrcode = QRcode_encodeString(
        str as *const libc::c_char,
        1 as libc::c_int,
        QR_ECLEVEL_H,
        QR_MODE_8,
        1 as libc::c_int,
    );
    qrdata = QRcode_decode(qrcode);
    assertionNum += 1;
    if !(qrdata as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        assertionFailed += 1;
        printf(b"Failed to decode.\n\0" as *const u8 as *const libc::c_char);
    }
    if qrdata as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        assertionNum += 1;
        tmp___0 = strlen(str as *const libc::c_char);
        if !(tmp___0 == (*qrdata).size as size_t) {
            assertionFailed += 1;
            tmp = strlen(str as *const libc::c_char);
            printf(
                b"Lengths of input/output mismatched: %d, expected %d.\n\0" as *const u8
                    as *const libc::c_char,
                (*qrdata).size,
                tmp as libc::c_int,
            );
        }
        assertionNum += 1;
        tmp___1 = strncmp(
            str as *const libc::c_char,
            (*qrdata).data as *mut libc::c_char as *const libc::c_char,
            (*qrdata).size as size_t,
        );
        if !(tmp___1 == 0 as libc::c_int) {
            assertionFailed += 1;
            printf(
                b"Decoded data %s is different from the original %s\n\0" as *const u8
                    as *const libc::c_char,
                (*qrdata).data,
                str,
            );
        }
    }
    if qrdata as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        QRdata_free(qrdata);
    }
    if qrcode as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        QRcode_free(qrcode);
    }
    testFinish();
}
unsafe extern "C" fn test_decodeLong() {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    let mut qrdata: *mut QRdata = 0 as *mut QRdata;
    let mut tmp: size_t = 0;
    let mut tmp___0: libc::c_int = 0;
    str = b"12345678901234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ?????????????\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    testStartReal(
        b"test_decodeLong\0" as *const u8 as *const libc::c_char,
        b"Test code words (long, splitted).\0" as *const u8 as *const libc::c_char,
    );
    qrcode = QRcode_encodeString(
        str as *const libc::c_char,
        0 as libc::c_int,
        QR_ECLEVEL_H,
        QR_MODE_8,
        1 as libc::c_int,
    );
    qrdata = QRcode_decode(qrcode);
    assertionNum += 1;
    if !(qrdata as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        assertionFailed += 1;
        printf(b"Failed to decode.\n\0" as *const u8 as *const libc::c_char);
    }
    if qrdata as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        assertionNum += 1;
        tmp = strlen(str as *const libc::c_char);
        if !(tmp == (*qrdata).size as size_t) {
            assertionFailed += 1;
            printf(
                b"Lengths of input/output mismatched.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        assertionNum += 1;
        tmp___0 = strncmp(
            str as *const libc::c_char,
            (*qrdata).data as *mut libc::c_char as *const libc::c_char,
            (*qrdata).size as size_t,
        );
        if !(tmp___0 == 0 as libc::c_int) {
            assertionFailed += 1;
            printf(
                b"Decoded data %s is different from the original %s\n\0" as *const u8
                    as *const libc::c_char,
                (*qrdata).data,
                str,
            );
        }
    }
    if qrdata as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        QRdata_free(qrdata);
    }
    if qrcode as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        QRcode_free(qrcode);
    }
    testFinish();
}
unsafe extern "C" fn test_decodeVeryLong() {
    let mut str: [libc::c_char; 4000] = [0; 4000];
    let mut i: libc::c_int = 0;
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    let mut qrdata: *mut QRdata = 0 as *mut QRdata;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: libc::c_int = 0;
    testStartReal(
        b"test_decodeVeryLong\0" as *const u8 as *const libc::c_char,
        b"Test code words (very long string).\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 3999 as libc::c_int {
        tmp = rand();
        str[i
            as usize] = decodeAnTable___0[(45 as libc::c_int as libc::c_double
            * tmp as libc::c_double
            / (2147483647 as libc::c_int as libc::c_double + 1.0f64)) as libc::c_int
            as usize];
        i += 1;
    }
    str[3999 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    qrcode = QRcode_encodeString(
        str.as_mut_ptr() as *const libc::c_char,
        0 as libc::c_int,
        QR_ECLEVEL_L,
        QR_MODE_8,
        0 as libc::c_int,
    );
    qrdata = QRcode_decode(qrcode);
    assertionNum += 1;
    if !(qrdata as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        assertionFailed += 1;
        printf(b"Failed to decode.\n\0" as *const u8 as *const libc::c_char);
    }
    if qrdata as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        assertionNum += 1;
        tmp___0 = strlen(str.as_mut_ptr() as *const libc::c_char);
        if !(tmp___0 == (*qrdata).size as size_t) {
            assertionFailed += 1;
            printf(
                b"Lengths of input/output mismatched.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        assertionNum += 1;
        tmp___1 = strncmp(
            str.as_mut_ptr() as *const libc::c_char,
            (*qrdata).data as *mut libc::c_char as *const libc::c_char,
            (*qrdata).size as size_t,
        );
        if !(tmp___1 == 0 as libc::c_int) {
            assertionFailed += 1;
            printf(
                b"Decoded data %s is different from the original %s\n\0" as *const u8
                    as *const libc::c_char,
                (*qrdata).data,
                str.as_mut_ptr(),
            );
        }
    }
    if qrdata as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        QRdata_free(qrdata);
    }
    if qrcode as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        QRcode_free(qrcode);
    }
    testFinish();
}
unsafe extern "C" fn test_decodeShortMQR() {
    let mut str: [libc::c_char; 3] = [0; 3];
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    let mut qrdata: *mut QRdata = 0 as *mut QRdata;
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    str[0 as libc::c_int as usize] = '5' as i32 as libc::c_char;
    str[1 as libc::c_int as usize] = '5' as i32 as libc::c_char;
    str[2 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    testStartReal(
        b"test_decodeShortMQR\0" as *const u8 as *const libc::c_char,
        b"Test code words (MQR).\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        qrcode = QRcode_encodeStringMQR(
            str.as_mut_ptr() as *const libc::c_char,
            MQRformat[i as usize].version,
            MQRformat[i as usize].level,
            QR_MODE_8,
            1 as libc::c_int,
        );
        qrdata = QRcode_decodeMQR(qrcode);
        assertionNum += 1;
        if !(qrdata as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
            assertionFailed += 1;
            printf(b"Failed to decode.\n\0" as *const u8 as *const libc::c_char);
        }
        assertionNum += 1;
        tmp = strcmp(
            (*qrdata).data as *mut libc::c_char as *const libc::c_char,
            str.as_mut_ptr() as *const libc::c_char,
        );
        if !(tmp == 0 as libc::c_int) {
            assertionFailed += 1;
            printf(
                b"Decoded data (%s) mismatched (%s)\n\0" as *const u8
                    as *const libc::c_char,
                (*qrdata).data as *mut libc::c_char,
                str.as_mut_ptr(),
            );
        }
        if qrdata as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            QRdata_free(qrdata);
        }
        if qrcode as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            QRcode_free(qrcode);
        }
        i += 1;
    }
    testFinish();
}
unsafe extern "C" fn test_oddBitCalcMQR() {
    let mut tests___0: [TestString; 2] = [TestString {
        str_0: 0 as *mut libc::c_char,
        version: 0,
        level: QR_ECLEVEL_L,
        hint: QR_MODE_NUM,
        casesensitive: 0,
    }; 2];
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    let mut qrdata: *mut QRdata = 0 as *mut QRdata;
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tests___0[0 as libc::c_int as usize]
        .str_0 = b"46194\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    tests___0[0 as libc::c_int as usize].version = 1 as libc::c_int;
    tests___0[0 as libc::c_int as usize].level = QR_ECLEVEL_L;
    tests___0[0 as libc::c_int as usize].hint = QR_MODE_8;
    tests___0[0 as libc::c_int as usize].casesensitive = 1 as libc::c_int;
    tests___0[1 as libc::c_int as usize]
        .str_0 = b"WBA5Y47YPQQ\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    tests___0[1 as libc::c_int as usize].version = 3 as libc::c_int;
    tests___0[1 as libc::c_int as usize].level = QR_ECLEVEL_L;
    tests___0[1 as libc::c_int as usize].hint = QR_MODE_8;
    tests___0[1 as libc::c_int as usize].casesensitive = 1 as libc::c_int;
    testStartReal(
        b"test_oddBitCalcMQR\0" as *const u8 as *const libc::c_char,
        b"Odd bits calculation bug checking (MQR).\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[TestString; 2]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<TestString>() as libc::c_ulong)
    {
        qrcode = QRcode_encodeStringMQR(
            tests___0[i as usize].str_0 as *const libc::c_char,
            tests___0[i as usize].version,
            tests___0[i as usize].level,
            tests___0[i as usize].hint,
            tests___0[i as usize].casesensitive,
        );
        assertionNum += 1;
        if !(qrcode as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
            assertionFailed += 1;
            printf(
                b"Failed to encode: %s\n\0" as *const u8 as *const libc::c_char,
                tests___0[i as usize].str_0,
            );
        }
        if !(qrcode as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
            qrdata = QRcode_decodeMQR(qrcode);
            assertionNum += 1;
            if !(qrdata as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
                assertionFailed += 1;
                printf(b"Failed to decode.\n\0" as *const u8 as *const libc::c_char);
            }
            assertionNum += 1;
            tmp = strcmp(
                (*qrdata).data as *mut libc::c_char as *const libc::c_char,
                tests___0[i as usize].str_0 as *const libc::c_char,
            );
            if !(tmp == 0 as libc::c_int) {
                assertionFailed += 1;
                printf(
                    b"Decoded data (%s) mismatched (%s)\n\0" as *const u8
                        as *const libc::c_char,
                    (*qrdata).data as *mut libc::c_char,
                    tests___0[i as usize].str_0,
                );
            }
            if qrdata as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                QRdata_free(qrdata);
            }
            QRcode_free(qrcode);
        }
        i += 1;
    }
    testFinish();
}
unsafe extern "C" fn test_invalid_inputMQR() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut code: *mut QRcode = 0 as *mut QRcode;
    testStartReal(
        b"test_invalid_inputMQR\0" as *const u8 as *const libc::c_char,
        b"Testing invalid input (MQR).\0" as *const u8 as *const libc::c_char,
    );
    input = QRinput_newMQR(1 as libc::c_int, QR_ECLEVEL_L);
    QRinput_append(
        input,
        QR_MODE_AN,
        5 as libc::c_int,
        b"TEST1\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar
            as *const libc::c_uchar,
    );
    (*input).version = -(1 as libc::c_int);
    (*input).level = QR_ECLEVEL_L;
    code = QRcode_encodeInput(input);
    assertionNum += 1;
    if !(code as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
        assertionFailed += 1;
        printf(
            b"invalid version(-1)  was not checked.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if code as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        QRcode_free(code);
    }
    (*input).version = 5 as libc::c_int;
    (*input).level = QR_ECLEVEL_L;
    code = QRcode_encodeInput(input);
    assertionNum += 1;
    if !(code as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
        assertionFailed += 1;
        printf(
            b"invalid version(5) access was not checked.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if code as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        QRcode_free(code);
    }
    (*input).version = 1 as libc::c_int;
    (*input).level = QR_ECLEVEL_H;
    code = QRcode_encodeInput(input);
    assertionNum += 1;
    if !(code as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
        assertionFailed += 1;
        printf(
            b"invalid level(H) access was not checked.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if code as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        QRcode_free(code);
    }
    (*input).version = 1 as libc::c_int;
    (*input).level = 4294967295 as QRecLevel;
    code = QRcode_encodeInput(input);
    assertionNum += 1;
    if !(code as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
        assertionFailed += 1;
        printf(
            b"invalid level(-1) access was not checked.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if code as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        QRcode_free(code);
    }
    QRinput_free(input);
    testFinish();
}
unsafe extern "C" fn test_mqrencode() {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pattern: [libc::c_char; 226] = [0; 226];
    let mut qrcode: QRcode = QRcode {
        version: 0,
        width: 0,
        data: 0 as *mut libc::c_uchar,
    };
    let mut qrdata: *mut QRdata = 0 as *mut QRdata;
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    str = b"MICROQR\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    pattern[0 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[1 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[2 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[3 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[4 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[5 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[6 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[7 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[8 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[9 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[10 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[11 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[12 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[13 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[14 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[15 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[16 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[17 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[18 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[19 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[20 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[21 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[22 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[23 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[24 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[25 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[26 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[27 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[28 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[29 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[30 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[31 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[32 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[33 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[34 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[35 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[36 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[37 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[38 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[39 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[40 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[41 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[42 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[43 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[44 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[45 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[46 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[47 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[48 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[49 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[50 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[51 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[52 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[53 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[54 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[55 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[56 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[57 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[58 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[59 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[60 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[61 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[62 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[63 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[64 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[65 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[66 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[67 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[68 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[69 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[70 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[71 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[72 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[73 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[74 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[75 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[76 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[77 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[78 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[79 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[80 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[81 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[82 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[83 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[84 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[85 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[86 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[87 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[88 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[89 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[90 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[91 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[92 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[93 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[94 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[95 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[96 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[97 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[98 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[99 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[100 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[101 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[102 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[103 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[104 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[105 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[106 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[107 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[108 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[109 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[110 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[111 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[112 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[113 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[114 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[115 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[116 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[117 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[118 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[119 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[120 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[121 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[122 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[123 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[124 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[125 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[126 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[127 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[128 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[129 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[130 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[131 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[132 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[133 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[134 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[135 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[136 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[137 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[138 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[139 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[140 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[141 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[142 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[143 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[144 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[145 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[146 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[147 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[148 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[149 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[150 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[151 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[152 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[153 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[154 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[155 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[156 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[157 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[158 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[159 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[160 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[161 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[162 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[163 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[164 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[165 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[166 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[167 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[168 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[169 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[170 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[171 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[172 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[173 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[174 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[175 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[176 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[177 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[178 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[179 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[180 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[181 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[182 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[183 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[184 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[185 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[186 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[187 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[188 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[189 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[190 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[191 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[192 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[193 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[194 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[195 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[196 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[197 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[198 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[199 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[200 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[201 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[202 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[203 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[204 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[205 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[206 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[207 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[208 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[209 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[210 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[211 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[212 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[213 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[214 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[215 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[216 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[217 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[218 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[219 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[220 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[221 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[222 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[223 as libc::c_int as usize] = '#' as i32 as libc::c_char;
    pattern[224 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    pattern[225 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    testStartReal(
        b"test_mqrencode\0" as *const u8 as *const libc::c_char,
        b"Encoding test (MQR).\0" as *const u8 as *const libc::c_char,
    );
    qrcode.width = 15 as libc::c_int;
    qrcode.version = 3 as libc::c_int;
    frame = MQRspec_newFrame(qrcode.version);
    i = 0 as libc::c_int;
    while i < 225 as libc::c_int {
        if pattern[i as usize] as libc::c_int == 35 as libc::c_int {
            tmp = 1 as libc::c_int;
        } else {
            tmp = 0 as libc::c_int;
        }
        *frame
            .offset(
                i as isize,
            ) = (*frame.offset(i as isize) as libc::c_int ^ tmp) as libc::c_uchar;
        i += 1;
    }
    qrcode.data = frame;
    qrdata = QRcode_decodeMQR(&mut qrcode);
    assertionNum += 1;
    if !((*qrdata).version == 3 as libc::c_int) {
        assertionFailed += 1;
        printf(
            b"Format info decoder returns wrong version number: %d (%d expected)\n\0"
                as *const u8 as *const libc::c_char,
            (*qrdata).version,
            3 as libc::c_int,
        );
    }
    assertionNum += 1;
    if !((*qrdata).level as libc::c_uint == 1 as libc::c_uint) {
        assertionFailed += 1;
        printf(
            b"Format info decoder returns wrong level: %d (%d expected)\n\0" as *const u8
                as *const libc::c_char,
            (*qrdata).level as libc::c_uint,
            1 as libc::c_int,
        );
    }
    assertionNum += 1;
    tmp___0 = strcmp(
        (*qrdata).data as *mut libc::c_char as *const libc::c_char,
        str as *const libc::c_char,
    );
    if !(tmp___0 == 0 as libc::c_int) {
        assertionFailed += 1;
        printf(
            b"Decoded data (%s) mismatched (%s)\n\0" as *const u8 as *const libc::c_char,
            (*qrdata).data as *mut libc::c_char,
            str,
        );
    }
    QRdata_free(qrdata);
    free(frame as *mut libc::c_void);
    testFinish();
}
unsafe extern "C" fn test_apiversion() {
    let mut major_version: libc::c_int = 0;
    let mut minor_version: libc::c_int = 0;
    let mut micro_version: libc::c_int = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: libc::c_int = 0;
    testStartReal(
        b"test_apiversion\0" as *const u8 as *const libc::c_char,
        b"API Version check\0" as *const u8 as *const libc::c_char,
    );
    QRcode_APIVersion(&mut major_version, &mut minor_version, &mut micro_version);
    assertionNum += 1;
    if !(major_version == 4 as libc::c_int) {
        assertionFailed += 1;
        printf(
            b"Major version number mismatched: %d (%d expected)\n\0" as *const u8
                as *const libc::c_char,
            major_version,
            4 as libc::c_int,
        );
    }
    assertionNum += 1;
    if !(minor_version == 1 as libc::c_int) {
        assertionFailed += 1;
        printf(
            b"Minor version number mismatched: %d (%d expected)\n\0" as *const u8
                as *const libc::c_char,
            minor_version,
            1 as libc::c_int,
        );
    }
    assertionNum += 1;
    if !(micro_version == 1 as libc::c_int) {
        assertionFailed += 1;
        printf(
            b"Micro version number mismatched: %d (%d expected)\n\0" as *const u8
                as *const libc::c_char,
            micro_version,
            1 as libc::c_int,
        );
    }
    str = QRcode_APIVersionString();
    str2 = QRcode_APIVersionString();
    assertionNum += 1;
    tmp = strcmp(
        b"4.1.1\0" as *const u8 as *const libc::c_char,
        str as *const libc::c_char,
    );
    if !(tmp == 0 as libc::c_int) {
        assertionFailed += 1;
        printf(
            b"Version string mismatched: %s (%s expected)\n\0" as *const u8
                as *const libc::c_char,
            str,
            b"4.1.1\0" as *const u8 as *const libc::c_char,
        );
    }
    assertionNum += 1;
    if !(str as libc::c_ulong == str2 as libc::c_ulong) {
        assertionFailed += 1;
        printf(
            b"Version strings are not identical.\0" as *const u8 as *const libc::c_char,
        );
    }
    testFinish();
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut tests___0: libc::c_int = 0;
    tests___0 = 33 as libc::c_int;
    testInit(tests___0);
    test_iterate();
    test_iterate2();
    test_filler();
    test_format();
    test_encode();
    test_encode2();
    test_encode3();
    test_encodeNull();
    test_encodeEmpty();
    test_encodeNull8();
    test_encodeEmpty8();
    test_encodeLongData();
    test_encodeVer26Num();
    test_01234567();
    test_invalid_input();
    test_struct_example();
    test_struct_semilong();
    test_null_free();
    test_qrraw_new();
    test_mqrraw_new();
    test_encodeData();
    test_formatInfo();
    test_decodeSimple();
    test_decodeLong();
    test_decodeVeryLong();
    test_fillerMQR();
    test_formatInfoMQR();
    test_encodeTooLongMQR();
    test_decodeShortMQR();
    test_oddBitCalcMQR();
    test_invalid_inputMQR();
    test_mqrencode();
    test_apiversion();
    testReport(tests___0);
    if argc > 1 as libc::c_int {
        print_filler();
        print_01234567();
        print_fillerMQR();
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn RSblock_initBlock(
    mut block: *mut RSblock,
    mut dl: libc::c_int,
    mut data: *mut libc::c_uchar,
    mut el: libc::c_int,
    mut ecc: *mut libc::c_uchar,
) {
    (*block).dataLength = dl;
    (*block).data = data;
    (*block).eccLength = el;
    (*block).ecc = ecc;
    RSECC_encode(dl as size_t, el as size_t, data as *const libc::c_uchar, ecc);
}
unsafe extern "C" fn RSblock_init(
    mut blocks: *mut RSblock,
    mut spec: *mut libc::c_int,
    mut data: *mut libc::c_uchar,
    mut ecc: *mut libc::c_uchar,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut block: *mut RSblock = 0 as *mut RSblock;
    let mut dp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ep: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut el: libc::c_int = 0;
    let mut dl: libc::c_int = 0;
    dl = *spec.offset(1 as libc::c_int as isize);
    el = *spec.offset(2 as libc::c_int as isize);
    block = blocks;
    dp = data;
    ep = ecc;
    i = 0 as libc::c_int;
    while i < *spec.offset(0 as libc::c_int as isize) {
        RSblock_initBlock(block, dl, dp, el, ep);
        dp = dp.offset(dl as isize);
        ep = ep.offset(el as isize);
        block = block.offset(1);
        i += 1;
    }
    if *spec.offset(3 as libc::c_int as isize) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    dl = *spec.offset(4 as libc::c_int as isize);
    el = *spec.offset(2 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < *spec.offset(3 as libc::c_int as isize) {
        RSblock_initBlock(block, dl, dp, el, ep);
        dp = dp.offset(dl as isize);
        ep = ep.offset(el as isize);
        block = block.offset(1);
        i += 1;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRraw_new(mut input: *mut QRinput) -> *mut QRRawCode {
    let mut raw: *mut QRRawCode = 0 as *mut QRRawCode;
    let mut spec: [libc::c_int; 5] = [0; 5];
    let mut ret: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<QRRawCode>() as libc::c_ulong);
    raw = tmp as *mut QRRawCode;
    if raw as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut QRRawCode;
    }
    (*raw).datacode = QRinput_getByteStream(input);
    if (*raw).datacode as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        free(raw as *mut libc::c_void);
        return 0 as *mut libc::c_void as *mut QRRawCode;
    }
    QRspec_getEccSpec((*input).version, (*input).level, spec.as_mut_ptr());
    (*raw).version = (*input).version;
    (*raw).b1 = spec[0 as libc::c_int as usize];
    (*raw)
        .dataLength = spec[0 as libc::c_int as usize] * spec[1 as libc::c_int as usize]
        + spec[3 as libc::c_int as usize] * spec[4 as libc::c_int as usize];
    (*raw)
        .eccLength = (spec[0 as libc::c_int as usize] + spec[3 as libc::c_int as usize])
        * spec[2 as libc::c_int as usize];
    tmp___0 = malloc((*raw).eccLength as size_t);
    (*raw).ecccode = tmp___0 as *mut libc::c_uchar;
    if (*raw).ecccode as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        free((*raw).datacode as *mut libc::c_void);
        free(raw as *mut libc::c_void);
        return 0 as *mut libc::c_void as *mut QRRawCode;
    }
    (*raw).blocks = spec[0 as libc::c_int as usize] + spec[3 as libc::c_int as usize];
    tmp___1 = calloc(
        (*raw).blocks as size_t,
        ::std::mem::size_of::<RSblock>() as libc::c_ulong,
    );
    (*raw).rsblock = tmp___1 as *mut RSblock;
    if (*raw).rsblock as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        QRraw_free(raw);
        return 0 as *mut libc::c_void as *mut QRRawCode;
    }
    ret = RSblock_init(
        (*raw).rsblock,
        spec.as_mut_ptr(),
        (*raw).datacode,
        (*raw).ecccode,
    );
    if ret < 0 as libc::c_int {
        QRraw_free(raw);
        return 0 as *mut libc::c_void as *mut QRRawCode;
    }
    (*raw).count = 0 as libc::c_int;
    return raw;
}
pub unsafe extern "C" fn QRraw_getCode(mut raw: *mut QRRawCode) -> libc::c_uchar {
    let mut col: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut ret: libc::c_uchar = 0;
    if (*raw).count < (*raw).dataLength {
        row = (*raw).count % (*raw).blocks;
        col = (*raw).count / (*raw).blocks;
        if col >= (*((*raw).rsblock).offset(0 as libc::c_int as isize)).dataLength {
            row += (*raw).b1;
        }
        ret = *((*((*raw).rsblock).offset(row as isize)).data).offset(col as isize);
    } else if (*raw).count < (*raw).dataLength + (*raw).eccLength {
        row = ((*raw).count - (*raw).dataLength) % (*raw).blocks;
        col = ((*raw).count - (*raw).dataLength) / (*raw).blocks;
        ret = *((*((*raw).rsblock).offset(row as isize)).ecc).offset(col as isize);
    } else {
        return 0 as libc::c_int as libc::c_uchar
    }
    (*raw).count += 1;
    return ret;
}
pub unsafe extern "C" fn QRraw_free(mut raw: *mut QRRawCode) {
    if raw as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        free((*raw).datacode as *mut libc::c_void);
        free((*raw).ecccode as *mut libc::c_void);
        free((*raw).rsblock as *mut libc::c_void);
        free(raw as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn MQRraw_new(mut input: *mut QRinput) -> *mut MQRRawCode {
    let mut raw: *mut MQRRawCode = 0 as *mut MQRRawCode;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<MQRRawCode>() as libc::c_ulong);
    raw = tmp as *mut MQRRawCode;
    if raw as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut MQRRawCode;
    }
    (*raw).version = (*input).version;
    (*raw).dataLength = MQRspec_getDataLength((*input).version, (*input).level);
    (*raw).eccLength = MQRspec_getECCLength((*input).version, (*input).level);
    tmp___0 = MQRspec_getDataLengthBit((*input).version, (*input).level);
    (*raw).oddbits = (*raw).dataLength * 8 as libc::c_int - tmp___0;
    (*raw).datacode = QRinput_getByteStream(input);
    if (*raw).datacode as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        free(raw as *mut libc::c_void);
        return 0 as *mut libc::c_void as *mut MQRRawCode;
    }
    tmp___1 = malloc((*raw).eccLength as size_t);
    (*raw).ecccode = tmp___1 as *mut libc::c_uchar;
    if (*raw).ecccode as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        free((*raw).datacode as *mut libc::c_void);
        free(raw as *mut libc::c_void);
        return 0 as *mut libc::c_void as *mut MQRRawCode;
    }
    tmp___2 = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<RSblock>() as libc::c_ulong,
    );
    (*raw).rsblock = tmp___2 as *mut RSblock;
    if (*raw).rsblock as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        MQRraw_free(raw);
        return 0 as *mut libc::c_void as *mut MQRRawCode;
    }
    RSblock_initBlock(
        (*raw).rsblock,
        (*raw).dataLength,
        (*raw).datacode,
        (*raw).eccLength,
        (*raw).ecccode,
    );
    (*raw).count = 0 as libc::c_int;
    return raw;
}
pub unsafe extern "C" fn MQRraw_getCode(mut raw: *mut MQRRawCode) -> libc::c_uchar {
    let mut ret: libc::c_uchar = 0;
    if (*raw).count < (*raw).dataLength {
        ret = *((*raw).datacode).offset((*raw).count as isize);
    } else if (*raw).count < (*raw).dataLength + (*raw).eccLength {
        ret = *((*raw).ecccode).offset(((*raw).count - (*raw).dataLength) as isize);
    } else {
        return 0 as libc::c_int as libc::c_uchar
    }
    (*raw).count += 1;
    return ret;
}
pub unsafe extern "C" fn MQRraw_free(mut raw: *mut MQRRawCode) {
    if raw as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        free((*raw).datacode as *mut libc::c_void);
        free((*raw).ecccode as *mut libc::c_void);
        free((*raw).rsblock as *mut libc::c_void);
        free(raw as *mut libc::c_void);
    }
}
unsafe extern "C" fn FrameFiller_set(
    mut filler: *mut FrameFiller,
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut mqr: libc::c_int,
) {
    (*filler).width = width;
    (*filler).frame = frame;
    (*filler).x = width - 1 as libc::c_int;
    (*filler).y = width - 1 as libc::c_int;
    (*filler).dir = -(1 as libc::c_int);
    (*filler).bit = -(1 as libc::c_int);
    (*filler).mqr = mqr;
}
unsafe extern "C" fn FrameFiller_next___0(
    mut filler: *mut FrameFiller,
) -> *mut libc::c_uchar {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut tmp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if (*filler).bit == -(1 as libc::c_int) {
        (*filler).bit = 0 as libc::c_int;
        return ((*filler).frame)
            .offset(((*filler).y * (*filler).width) as isize)
            .offset((*filler).x as isize);
    }
    x = (*filler).x;
    y = (*filler).y;
    p = (*filler).frame;
    w = (*filler).width;
    if (*filler).bit == 0 as libc::c_int {
        x -= 1;
        (*filler).bit += 1;
    } else {
        x += 1;
        y += (*filler).dir;
        (*filler).bit -= 1;
    }
    if (*filler).dir < 0 as libc::c_int {
        if y < 0 as libc::c_int {
            y = 0 as libc::c_int;
            x -= 2 as libc::c_int;
            (*filler).dir = 1 as libc::c_int;
            if (*filler).mqr == 0 {
                if x == 6 as libc::c_int {
                    x -= 1;
                    y = 9 as libc::c_int;
                }
            }
        }
    } else if y == w {
        y = w - 1 as libc::c_int;
        x -= 2 as libc::c_int;
        (*filler).dir = -(1 as libc::c_int);
        if (*filler).mqr == 0 {
            if x == 6 as libc::c_int {
                x -= 1;
                y -= 8 as libc::c_int;
            }
        }
    }
    if x < 0 as libc::c_int {
        return 0 as *mut libc::c_void as *mut libc::c_uchar
    } else {
        if y < 0 as libc::c_int {
            return 0 as *mut libc::c_void as *mut libc::c_uchar;
        }
    }
    (*filler).x = x;
    (*filler).y = y;
    if *p.offset((y * w + x) as isize) as libc::c_int & 128 as libc::c_int != 0 {
        tmp = FrameFiller_next___0(filler);
        return tmp;
    }
    return p.offset((y * w + x) as isize);
}
pub unsafe extern "C" fn FrameFiller_test(
    mut version: libc::c_int,
) -> *mut libc::c_uchar {
    let mut width: libc::c_int = 0;
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut filler: FrameFiller = FrameFiller {
        width: 0,
        frame: 0 as *mut libc::c_uchar,
        x: 0,
        y: 0,
        dir: 0,
        bit: 0,
        mqr: 0,
    };
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    width = QRspec_getWidth(version);
    frame = QRspec_newFrame(version);
    if frame as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut libc::c_uchar;
    }
    FrameFiller_set(&mut filler, width, frame, 0 as libc::c_int);
    tmp = QRspec_getDataLength(version, QR_ECLEVEL_L);
    tmp___0 = QRspec_getECCLength(version, QR_ECLEVEL_L);
    tmp___1 = QRspec_getRemainder(version);
    length = tmp * 8 as libc::c_int + tmp___0 * 8 as libc::c_int + tmp___1;
    i = 0 as libc::c_int;
    while i < length {
        p = FrameFiller_next___0(&mut filler);
        if p as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            free(frame as *mut libc::c_void);
            return 0 as *mut libc::c_void as *mut libc::c_uchar;
        }
        *p = ((i & 127 as libc::c_int) as libc::c_uchar as libc::c_int
            | 128 as libc::c_int) as libc::c_uchar;
        i += 1;
    }
    return frame;
}
pub unsafe extern "C" fn FrameFiller_testMQR(
    mut version: libc::c_int,
) -> *mut libc::c_uchar {
    let mut width: libc::c_int = 0;
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut filler: FrameFiller = FrameFiller {
        width: 0,
        frame: 0 as *mut libc::c_uchar,
        x: 0,
        y: 0,
        dir: 0,
        bit: 0,
        mqr: 0,
    };
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    width = MQRspec_getWidth(version);
    frame = MQRspec_newFrame(version);
    if frame as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut libc::c_uchar;
    }
    FrameFiller_set(&mut filler, width, frame, 1 as libc::c_int);
    tmp = MQRspec_getDataLengthBit(version, QR_ECLEVEL_L);
    tmp___0 = MQRspec_getECCLength(version, QR_ECLEVEL_L);
    length = tmp + tmp___0 * 8 as libc::c_int;
    i = 0 as libc::c_int;
    while i < length {
        p = FrameFiller_next___0(&mut filler);
        if p as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            fprintf(
                stderr,
                b"Frame filler run over the frame!\n\0" as *const u8
                    as *const libc::c_char,
            );
            return frame;
        }
        *p = ((i & 127 as libc::c_int) as libc::c_uchar as libc::c_int
            | 128 as libc::c_int) as libc::c_uchar;
        i += 1;
    }
    return frame;
}
pub unsafe extern "C" fn QRcode_new(
    mut version: libc::c_int,
    mut width: libc::c_int,
    mut data: *mut libc::c_uchar,
) -> *mut QRcode {
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<QRcode>() as libc::c_ulong);
    qrcode = tmp as *mut QRcode;
    if qrcode as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut QRcode;
    }
    (*qrcode).version = version;
    (*qrcode).width = width;
    (*qrcode).data = data;
    return qrcode;
}
pub unsafe extern "C" fn QRcode_free(mut qrcode: *mut QRcode) {
    if qrcode as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        free((*qrcode).data as *mut libc::c_void);
        free(qrcode as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn QRcode_encodeMask(
    mut input: *mut QRinput,
    mut mask: libc::c_int,
) -> *mut QRcode {
    let mut current_block: u64;
    let mut width: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut raw: *mut QRRawCode = 0 as *mut QRRawCode;
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut masked: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut code: libc::c_uchar = 0;
    let mut bit: libc::c_uchar = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    let mut filler: FrameFiller = FrameFiller {
        width: 0,
        frame: 0 as *mut libc::c_uchar,
        x: 0,
        y: 0,
        dir: 0,
        bit: 0,
        mqr: 0,
    };
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    qrcode = 0 as *mut libc::c_void as *mut QRcode;
    if (*input).mqr != 0 {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
        return 0 as *mut libc::c_void as *mut QRcode;
    }
    if (*input).version < 0 as libc::c_int {
        tmp___0 = __errno_location();
        *tmp___0 = 22 as libc::c_int;
        return 0 as *mut libc::c_void as *mut QRcode;
    } else {
        if (*input).version > 40 as libc::c_int {
            tmp___0 = __errno_location();
            *tmp___0 = 22 as libc::c_int;
            return 0 as *mut libc::c_void as *mut QRcode;
        }
    }
    if (*input).level as libc::c_uint >= 0 as libc::c_uint {
        if !((*input).level as libc::c_uint <= 3 as libc::c_uint) {
            tmp___1 = __errno_location();
            *tmp___1 = 22 as libc::c_int;
            return 0 as *mut libc::c_void as *mut QRcode;
        }
    } else {
        tmp___1 = __errno_location();
        *tmp___1 = 22 as libc::c_int;
        return 0 as *mut libc::c_void as *mut QRcode;
    }
    raw = QRraw_new(input);
    if raw as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut QRcode;
    }
    version = (*raw).version;
    width = QRspec_getWidth(version);
    frame = QRspec_newFrame(version);
    if frame as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        QRraw_free(raw);
        return 0 as *mut libc::c_void as *mut QRcode;
    }
    FrameFiller_set(&mut filler, width, frame, 0 as libc::c_int);
    i = 0 as libc::c_int;
    's_164: loop {
        if !(i < (*raw).dataLength) {
            current_block = 12199444798915819164;
            break;
        }
        code = QRraw_getCode(raw);
        bit = 128 as libc::c_int as libc::c_uchar;
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            p = FrameFiller_next___0(&mut filler);
            if p as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                current_block = 11523662474893251375;
                break 's_164;
            }
            *p = (bit as libc::c_int & code as libc::c_int != 0 as libc::c_int)
                as libc::c_int as libc::c_uchar;
            bit = (bit as libc::c_int >> 1 as libc::c_int) as libc::c_uchar;
            j += 1;
        }
        i += 1;
    }
    match current_block {
        12199444798915819164 => {
            i = 0 as libc::c_int;
            's_214: loop {
                if !(i < (*raw).eccLength) {
                    current_block = 11777552016271000781;
                    break;
                }
                code = QRraw_getCode(raw);
                bit = 128 as libc::c_int as libc::c_uchar;
                j = 0 as libc::c_int;
                while j < 8 as libc::c_int {
                    p = FrameFiller_next___0(&mut filler);
                    if p as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                        current_block = 11523662474893251375;
                        break 's_214;
                    }
                    *p = (2 as libc::c_int
                        | (bit as libc::c_int & code as libc::c_int != 0 as libc::c_int)
                            as libc::c_int) as libc::c_uchar;
                    bit = (bit as libc::c_int >> 1 as libc::c_int) as libc::c_uchar;
                    j += 1;
                }
                i += 1;
            }
            match current_block {
                11523662474893251375 => {}
                _ => {
                    QRraw_free(raw);
                    raw = 0 as *mut libc::c_void as *mut QRRawCode;
                    j = QRspec_getRemainder(version);
                    i = 0 as libc::c_int;
                    loop {
                        if !(i < j) {
                            current_block = 168769493162332264;
                            break;
                        }
                        p = FrameFiller_next___0(&mut filler);
                        if p as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong
                        {
                            current_block = 11523662474893251375;
                            break;
                        }
                        *p = 2 as libc::c_int as libc::c_uchar;
                        i += 1;
                    }
                    match current_block {
                        11523662474893251375 => {}
                        _ => {
                            if mask == -(2 as libc::c_int) {
                                tmp___2 = malloc((width * width) as size_t);
                                masked = tmp___2 as *mut libc::c_uchar;
                                memcpy(
                                    masked as *mut libc::c_void,
                                    frame as *const libc::c_void,
                                    (width * width) as size_t,
                                );
                            } else if mask < 0 as libc::c_int {
                                masked = Mask_mask(width, frame, (*input).level);
                            } else {
                                masked = Mask_makeMask(width, frame, mask, (*input).level);
                            }
                            if !(masked as libc::c_ulong
                                == 0 as *mut libc::c_void as libc::c_ulong)
                            {
                                qrcode = QRcode_new(version, width, masked);
                                if qrcode as libc::c_ulong
                                    == 0 as *mut libc::c_void as libc::c_ulong
                                {
                                    free(masked as *mut libc::c_void);
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    QRraw_free(raw);
    free(frame as *mut libc::c_void);
    return qrcode;
}
pub unsafe extern "C" fn QRcode_encodeMaskMQR(
    mut input: *mut QRinput,
    mut mask: libc::c_int,
) -> *mut QRcode {
    let mut current_block: u64;
    let mut width: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut raw: *mut MQRRawCode = 0 as *mut MQRRawCode;
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut masked: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut code: libc::c_uchar = 0;
    let mut bit: libc::c_uchar = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    let mut filler: FrameFiller = FrameFiller {
        width: 0,
        frame: 0 as *mut libc::c_uchar,
        x: 0,
        y: 0,
        dir: 0,
        bit: 0,
        mqr: 0,
    };
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    qrcode = 0 as *mut libc::c_void as *mut QRcode;
    if (*input).mqr == 0 {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
        return 0 as *mut libc::c_void as *mut QRcode;
    }
    if (*input).version <= 0 as libc::c_int {
        tmp___0 = __errno_location();
        *tmp___0 = 22 as libc::c_int;
        return 0 as *mut libc::c_void as *mut QRcode;
    } else {
        if (*input).version > 4 as libc::c_int {
            tmp___0 = __errno_location();
            *tmp___0 = 22 as libc::c_int;
            return 0 as *mut libc::c_void as *mut QRcode;
        }
    }
    if (*input).level as libc::c_uint >= 0 as libc::c_uint {
        if !((*input).level as libc::c_uint <= 2 as libc::c_uint) {
            tmp___1 = __errno_location();
            *tmp___1 = 22 as libc::c_int;
            return 0 as *mut libc::c_void as *mut QRcode;
        }
    } else {
        tmp___1 = __errno_location();
        *tmp___1 = 22 as libc::c_int;
        return 0 as *mut libc::c_void as *mut QRcode;
    }
    raw = MQRraw_new(input);
    if raw as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut QRcode;
    }
    version = (*raw).version;
    width = MQRspec_getWidth(version);
    frame = MQRspec_newFrame(version);
    if frame as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        MQRraw_free(raw);
        return 0 as *mut libc::c_void as *mut QRcode;
    }
    FrameFiller_set(&mut filler, width, frame, 1 as libc::c_int);
    i = 0 as libc::c_int;
    's_166: loop {
        if !(i < (*raw).dataLength) {
            current_block = 13619784596304402172;
            break;
        }
        code = MQRraw_getCode(raw);
        bit = 128 as libc::c_int as libc::c_uchar;
        if (*raw).oddbits != 0 {
            if i == (*raw).dataLength - 1 as libc::c_int {
                length = (*raw).oddbits;
            } else {
                length = 8 as libc::c_int;
            }
        } else {
            length = 8 as libc::c_int;
        }
        j = 0 as libc::c_int;
        while j < length {
            p = FrameFiller_next___0(&mut filler);
            if p as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                current_block = 16279170580966144131;
                break 's_166;
            }
            *p = (bit as libc::c_int & code as libc::c_int != 0 as libc::c_int)
                as libc::c_int as libc::c_uchar;
            bit = (bit as libc::c_int >> 1 as libc::c_int) as libc::c_uchar;
            j += 1;
        }
        i += 1;
    }
    match current_block {
        13619784596304402172 => {
            i = 0 as libc::c_int;
            's_253: loop {
                if !(i < (*raw).eccLength) {
                    current_block = 13303144130133872306;
                    break;
                }
                code = MQRraw_getCode(raw);
                bit = 128 as libc::c_int as libc::c_uchar;
                length = 8 as libc::c_int;
                j = 0 as libc::c_int;
                while j < length {
                    p = FrameFiller_next___0(&mut filler);
                    if p as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                        current_block = 16279170580966144131;
                        break 's_253;
                    }
                    *p = (2 as libc::c_int
                        | (bit as libc::c_int & code as libc::c_int != 0 as libc::c_int)
                            as libc::c_int) as libc::c_uchar;
                    bit = (bit as libc::c_int >> 1 as libc::c_int) as libc::c_uchar;
                    j += 1;
                }
                i += 1;
            }
            match current_block {
                16279170580966144131 => {}
                _ => {
                    MQRraw_free(raw);
                    raw = 0 as *mut libc::c_void as *mut MQRRawCode;
                    if mask == -(2 as libc::c_int) {
                        tmp___2 = malloc((width * width) as size_t);
                        masked = tmp___2 as *mut libc::c_uchar;
                        memcpy(
                            masked as *mut libc::c_void,
                            frame as *const libc::c_void,
                            (width * width) as size_t,
                        );
                    } else if mask < 0 as libc::c_int {
                        masked = MMask_mask(version, frame, (*input).level);
                    } else {
                        masked = MMask_makeMask(version, frame, mask, (*input).level);
                    }
                    if !(masked as libc::c_ulong
                        == 0 as *mut libc::c_void as libc::c_ulong)
                    {
                        qrcode = QRcode_new(version, width, masked);
                        if qrcode as libc::c_ulong
                            == 0 as *mut libc::c_void as libc::c_ulong
                        {
                            free(masked as *mut libc::c_void);
                        }
                    }
                }
            }
        }
        _ => {}
    }
    MQRraw_free(raw);
    free(frame as *mut libc::c_void);
    return qrcode;
}
pub unsafe extern "C" fn QRcode_encodeInput(mut input: *mut QRinput) -> *mut QRcode {
    let mut tmp: *mut QRcode = 0 as *mut QRcode;
    let mut tmp___0: *mut QRcode = 0 as *mut QRcode;
    if (*input).mqr != 0 {
        tmp = QRcode_encodeMaskMQR(input, -(1 as libc::c_int));
        return tmp;
    } else {
        tmp___0 = QRcode_encodeMask(input, -(1 as libc::c_int));
        return tmp___0;
    };
}
unsafe extern "C" fn QRcode_encodeStringReal(
    mut string: *const libc::c_char,
    mut version: libc::c_int,
    mut level: QRecLevel,
    mut mqr: libc::c_int,
    mut hint: QRencodeMode,
    mut casesensitive: libc::c_int,
) -> *mut QRcode {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut code: *mut QRcode = 0 as *mut QRcode;
    let mut ret: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    if string as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
        return 0 as *mut libc::c_void as *mut QRcode;
    }
    if hint as libc::c_int != 2 as libc::c_int {
        if hint as libc::c_int != 3 as libc::c_int {
            tmp___0 = __errno_location();
            *tmp___0 = 22 as libc::c_int;
            return 0 as *mut libc::c_void as *mut QRcode;
        }
    }
    if mqr != 0 {
        input = QRinput_newMQR(version, level);
    } else {
        input = QRinput_new2(version, level);
    }
    if input as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut QRcode;
    }
    ret = Split_splitStringToQRinput(string, input, hint, casesensitive);
    if ret < 0 as libc::c_int {
        QRinput_free(input);
        return 0 as *mut libc::c_void as *mut QRcode;
    }
    code = QRcode_encodeInput(input);
    QRinput_free(input);
    return code;
}
pub unsafe extern "C" fn QRcode_encodeString(
    mut string: *const libc::c_char,
    mut version: libc::c_int,
    mut level: QRecLevel,
    mut hint: QRencodeMode,
    mut casesensitive: libc::c_int,
) -> *mut QRcode {
    let mut tmp: *mut QRcode = 0 as *mut QRcode;
    tmp = QRcode_encodeStringReal(
        string,
        version,
        level,
        0 as libc::c_int,
        hint,
        casesensitive,
    );
    return tmp;
}
pub unsafe extern "C" fn QRcode_encodeStringMQR(
    mut string: *const libc::c_char,
    mut version: libc::c_int,
    mut level: QRecLevel,
    mut hint: QRencodeMode,
    mut casesensitive: libc::c_int,
) -> *mut QRcode {
    let mut i: libc::c_int = 0;
    let mut code: *mut QRcode = 0 as *mut QRcode;
    let mut tmp: *mut QRcode = 0 as *mut QRcode;
    if version == 0 as libc::c_int {
        version = 1 as libc::c_int;
    }
    i = version;
    while i <= 4 as libc::c_int {
        tmp = QRcode_encodeStringReal(
            string,
            i,
            level,
            1 as libc::c_int,
            hint,
            casesensitive,
        );
        code = tmp;
        if code as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            return code;
        }
        i += 1;
    }
    return 0 as *mut libc::c_void as *mut QRcode;
}
unsafe extern "C" fn QRcode_encodeDataReal(
    mut data: *const libc::c_uchar,
    mut length: libc::c_int,
    mut version: libc::c_int,
    mut level: QRecLevel,
    mut mqr: libc::c_int,
) -> *mut QRcode {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut code: *mut QRcode = 0 as *mut QRcode;
    let mut ret: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    if data as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
        return 0 as *mut libc::c_void as *mut QRcode;
    } else {
        if length == 0 as libc::c_int {
            tmp = __errno_location();
            *tmp = 22 as libc::c_int;
            return 0 as *mut libc::c_void as *mut QRcode;
        }
    }
    if mqr != 0 {
        input = QRinput_newMQR(version, level);
    } else {
        input = QRinput_new2(version, level);
    }
    if input as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut QRcode;
    }
    ret = QRinput_append(input, QR_MODE_8, length, data);
    if ret < 0 as libc::c_int {
        QRinput_free(input);
        return 0 as *mut libc::c_void as *mut QRcode;
    }
    code = QRcode_encodeInput(input);
    QRinput_free(input);
    return code;
}
pub unsafe extern "C" fn QRcode_encodeData(
    mut size: libc::c_int,
    mut data: *const libc::c_uchar,
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> *mut QRcode {
    let mut tmp: *mut QRcode = 0 as *mut QRcode;
    tmp = QRcode_encodeDataReal(data, size, version, level, 0 as libc::c_int);
    return tmp;
}
pub unsafe extern "C" fn QRcode_encodeString8bit(
    mut string: *const libc::c_char,
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> *mut QRcode {
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: *mut QRcode = 0 as *mut QRcode;
    if string as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
        return 0 as *mut libc::c_void as *mut QRcode;
    }
    tmp___0 = strlen(string);
    tmp___1 = QRcode_encodeDataReal(
        string as *mut libc::c_uchar as *const libc::c_uchar,
        tmp___0 as libc::c_int,
        version,
        level,
        0 as libc::c_int,
    );
    return tmp___1;
}
pub unsafe extern "C" fn QRcode_encodeDataMQR(
    mut size: libc::c_int,
    mut data: *const libc::c_uchar,
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> *mut QRcode {
    let mut i: libc::c_int = 0;
    let mut code: *mut QRcode = 0 as *mut QRcode;
    let mut tmp: *mut QRcode = 0 as *mut QRcode;
    if version == 0 as libc::c_int {
        version = 1 as libc::c_int;
    }
    i = version;
    while i <= 4 as libc::c_int {
        tmp = QRcode_encodeDataReal(data, size, i, level, 1 as libc::c_int);
        code = tmp;
        if code as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            return code;
        }
        i += 1;
    }
    return 0 as *mut libc::c_void as *mut QRcode;
}
pub unsafe extern "C" fn QRcode_encodeString8bitMQR(
    mut string: *const libc::c_char,
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> *mut QRcode {
    let mut i: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut code: *mut QRcode = 0 as *mut QRcode;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: *mut QRcode = 0 as *mut QRcode;
    if string as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
        return 0 as *mut libc::c_void as *mut QRcode;
    }
    if version == 0 as libc::c_int {
        version = 1 as libc::c_int;
    }
    i = version;
    while i <= 4 as libc::c_int {
        tmp___0 = strlen(string);
        tmp___1 = QRcode_encodeDataReal(
            string as *mut libc::c_uchar as *const libc::c_uchar,
            tmp___0 as libc::c_int,
            i,
            level,
            1 as libc::c_int,
        );
        code = tmp___1;
        if code as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            return code;
        }
        i += 1;
    }
    return 0 as *mut libc::c_void as *mut QRcode;
}
unsafe extern "C" fn QRcode_List_newEntry() -> *mut QRcode_List {
    let mut entry: *mut QRcode_List = 0 as *mut QRcode_List;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<QRcode_List>() as libc::c_ulong);
    entry = tmp as *mut QRcode_List;
    if entry as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut QRcode_List;
    }
    (*entry).next = 0 as *mut libc::c_void as *mut _QRcode_List;
    (*entry).code = 0 as *mut libc::c_void as *mut QRcode;
    return entry;
}
unsafe extern "C" fn QRcode_List_freeEntry(mut entry: *mut QRcode_List) {
    if entry as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        QRcode_free((*entry).code);
        free(entry as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn QRcode_List_free(mut qrlist: *mut QRcode_List) {
    let mut list: *mut QRcode_List = 0 as *mut QRcode_List;
    let mut next: *mut QRcode_List = 0 as *mut QRcode_List;
    list = qrlist;
    while list as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        next = (*list).next;
        QRcode_List_freeEntry(list);
        list = next;
    }
}
pub unsafe extern "C" fn QRcode_List_size(mut qrlist: *mut QRcode_List) -> libc::c_int {
    let mut list: *mut QRcode_List = 0 as *mut QRcode_List;
    let mut size: libc::c_int = 0;
    list = qrlist;
    size = 0 as libc::c_int;
    while list as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        size += 1;
        list = (*list).next;
    }
    return size;
}
pub unsafe extern "C" fn QRcode_encodeInputStructured(
    mut s: *mut QRinput_Struct,
) -> *mut QRcode_List {
    let mut current_block: u64;
    let mut head: *mut QRcode_List = 0 as *mut QRcode_List;
    let mut tail: *mut QRcode_List = 0 as *mut QRcode_List;
    let mut entry: *mut QRcode_List = 0 as *mut QRcode_List;
    let mut list: *mut QRinput_InputList = 0 as *mut QRinput_InputList;
    head = 0 as *mut libc::c_void as *mut QRcode_List;
    tail = 0 as *mut libc::c_void as *mut QRcode_List;
    list = (*s).head;
    loop {
        if !(list as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
            current_block = 8831408221741692167;
            break;
        }
        if head as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            entry = QRcode_List_newEntry();
            if entry as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                current_block = 11107710465922088125;
                break;
            }
            head = entry;
            tail = head;
        } else {
            entry = QRcode_List_newEntry();
            if entry as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                current_block = 11107710465922088125;
                break;
            }
            (*tail).next = entry;
            tail = (*tail).next;
        }
        (*tail).code = QRcode_encodeInput((*list).input);
        if (*tail).code as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            current_block = 11107710465922088125;
            break;
        }
        list = (*list).next;
    }
    match current_block {
        8831408221741692167 => return head,
        _ => {
            QRcode_List_free(head);
            return 0 as *mut libc::c_void as *mut QRcode_List;
        }
    };
}
unsafe extern "C" fn QRcode_encodeInputToStructured(
    mut input: *mut QRinput,
) -> *mut QRcode_List {
    let mut s: *mut QRinput_Struct = 0 as *mut QRinput_Struct;
    let mut codes: *mut QRcode_List = 0 as *mut QRcode_List;
    s = QRinput_splitQRinputToStruct(input);
    if s as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut QRcode_List;
    }
    codes = QRcode_encodeInputStructured(s);
    QRinput_Struct_free(s);
    return codes;
}
unsafe extern "C" fn QRcode_encodeDataStructuredReal(
    mut size: libc::c_int,
    mut data: *const libc::c_uchar,
    mut version: libc::c_int,
    mut level: QRecLevel,
    mut eightbit: libc::c_int,
    mut hint: QRencodeMode,
    mut casesensitive: libc::c_int,
) -> *mut QRcode_List {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut codes: *mut QRcode_List = 0 as *mut QRcode_List;
    let mut ret: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    if version <= 0 as libc::c_int {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
        return 0 as *mut libc::c_void as *mut QRcode_List;
    }
    if eightbit == 0 {
        if hint as libc::c_int != 2 as libc::c_int {
            if hint as libc::c_int != 3 as libc::c_int {
                tmp___0 = __errno_location();
                *tmp___0 = 22 as libc::c_int;
                return 0 as *mut libc::c_void as *mut QRcode_List;
            }
        }
    }
    input = QRinput_new2(version, level);
    if input as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut QRcode_List;
    }
    if eightbit != 0 {
        ret = QRinput_append(input, QR_MODE_8, size, data);
    } else {
        ret = Split_splitStringToQRinput(
            data as *mut libc::c_char as *const libc::c_char,
            input,
            hint,
            casesensitive,
        );
    }
    if ret < 0 as libc::c_int {
        QRinput_free(input);
        return 0 as *mut libc::c_void as *mut QRcode_List;
    }
    codes = QRcode_encodeInputToStructured(input);
    QRinput_free(input);
    return codes;
}
pub unsafe extern "C" fn QRcode_encodeDataStructured(
    mut size: libc::c_int,
    mut data: *const libc::c_uchar,
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> *mut QRcode_List {
    let mut tmp: *mut QRcode_List = 0 as *mut QRcode_List;
    tmp = QRcode_encodeDataStructuredReal(
        size,
        data,
        version,
        level,
        1 as libc::c_int,
        QR_MODE_NUL,
        0 as libc::c_int,
    );
    return tmp;
}
pub unsafe extern "C" fn QRcode_encodeString8bitStructured(
    mut string: *const libc::c_char,
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> *mut QRcode_List {
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: *mut QRcode_List = 0 as *mut QRcode_List;
    if string as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
        return 0 as *mut libc::c_void as *mut QRcode_List;
    }
    tmp___0 = strlen(string);
    tmp___1 = QRcode_encodeDataStructured(
        tmp___0 as libc::c_int,
        string as *mut libc::c_uchar as *const libc::c_uchar,
        version,
        level,
    );
    return tmp___1;
}
pub unsafe extern "C" fn QRcode_encodeStringStructured(
    mut string: *const libc::c_char,
    mut version: libc::c_int,
    mut level: QRecLevel,
    mut hint: QRencodeMode,
    mut casesensitive: libc::c_int,
) -> *mut QRcode_List {
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: *mut QRcode_List = 0 as *mut QRcode_List;
    if string as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
        return 0 as *mut libc::c_void as *mut QRcode_List;
    }
    tmp___0 = strlen(string);
    tmp___1 = QRcode_encodeDataStructuredReal(
        tmp___0 as libc::c_int,
        string as *mut libc::c_uchar as *const libc::c_uchar,
        version,
        level,
        0 as libc::c_int,
        hint,
        casesensitive,
    );
    return tmp___1;
}
pub unsafe extern "C" fn QRcode_APIVersion(
    mut major_version: *mut libc::c_int,
    mut minor_version: *mut libc::c_int,
    mut micro_version: *mut libc::c_int,
) {
    if major_version as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        *major_version = 4 as libc::c_int;
    }
    if minor_version as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        *minor_version = 1 as libc::c_int;
    }
    if micro_version as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        *micro_version = 1 as libc::c_int;
    }
}
pub unsafe extern "C" fn QRcode_APIVersionString() -> *mut libc::c_char {
    return b"4.1.1\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
pub unsafe extern "C" fn QRcode_clearCache() {}
pub unsafe extern "C" fn QRinput_isSplittableMode(
    mut mode: QRencodeMode,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    if mode as libc::c_int >= 0 as libc::c_int {
        if mode as libc::c_int <= 3 as libc::c_int {
            tmp = 1 as libc::c_int;
        } else {
            tmp = 0 as libc::c_int;
        }
    } else {
        tmp = 0 as libc::c_int;
    }
    return tmp;
}
unsafe extern "C" fn QRinput_List_newEntry(
    mut mode: QRencodeMode,
    mut size: libc::c_int,
    mut data: *const libc::c_uchar,
) -> *mut QRinput_List {
    let mut entry: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp___0 = QRinput_check(mode, size, data);
    if tmp___0 != 0 {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
        return 0 as *mut libc::c_void as *mut QRinput_List;
    }
    tmp___1 = malloc(::std::mem::size_of::<QRinput_List>() as libc::c_ulong);
    entry = tmp___1 as *mut QRinput_List;
    if entry as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut QRinput_List;
    }
    (*entry).mode = mode;
    (*entry).size = size;
    (*entry).data = 0 as *mut libc::c_void as *mut libc::c_uchar;
    if size > 0 as libc::c_int {
        tmp___2 = malloc(size as size_t);
        (*entry).data = tmp___2 as *mut libc::c_uchar;
        if (*entry).data as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            free(entry as *mut libc::c_void);
            return 0 as *mut libc::c_void as *mut QRinput_List;
        }
        memcpy(
            (*entry).data as *mut libc::c_void,
            data as *const libc::c_void,
            size as size_t,
        );
    }
    (*entry).bstream = 0 as *mut libc::c_void as *mut BitStream;
    (*entry).next = 0 as *mut libc::c_void as *mut QRinput_List;
    return entry;
}
unsafe extern "C" fn QRinput_List_freeEntry(mut entry: *mut QRinput_List) {
    if entry as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        free((*entry).data as *mut libc::c_void);
        BitStream_free((*entry).bstream);
        free(entry as *mut libc::c_void);
    }
}
unsafe extern "C" fn QRinput_List_dup(
    mut entry: *mut QRinput_List,
) -> *mut QRinput_List {
    let mut n: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<QRinput_List>() as libc::c_ulong);
    n = tmp as *mut QRinput_List;
    if n as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut QRinput_List;
    }
    (*n).mode = (*entry).mode;
    (*n).size = (*entry).size;
    tmp___0 = malloc((*n).size as size_t);
    (*n).data = tmp___0 as *mut libc::c_uchar;
    if (*n).data as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        free(n as *mut libc::c_void);
        return 0 as *mut libc::c_void as *mut QRinput_List;
    }
    memcpy(
        (*n).data as *mut libc::c_void,
        (*entry).data as *const libc::c_void,
        (*entry).size as size_t,
    );
    (*n).bstream = 0 as *mut libc::c_void as *mut BitStream;
    (*n).next = 0 as *mut libc::c_void as *mut QRinput_List;
    return n;
}
pub unsafe extern "C" fn QRinput_new() -> *mut QRinput {
    let mut tmp: *mut QRinput = 0 as *mut QRinput;
    tmp = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    return tmp;
}
pub unsafe extern "C" fn QRinput_new2(
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> *mut QRinput {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    if version < 0 as libc::c_int {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
        return 0 as *mut libc::c_void as *mut QRinput;
    } else {
        if version > 40 as libc::c_int {
            tmp = __errno_location();
            *tmp = 22 as libc::c_int;
            return 0 as *mut libc::c_void as *mut QRinput;
        } else {
            if (level as libc::c_uint) < 0 as libc::c_uint {
                tmp = __errno_location();
                *tmp = 22 as libc::c_int;
                return 0 as *mut libc::c_void as *mut QRinput;
            } else {
                if level as libc::c_uint > 3 as libc::c_uint {
                    tmp = __errno_location();
                    *tmp = 22 as libc::c_int;
                    return 0 as *mut libc::c_void as *mut QRinput;
                }
            }
        }
    }
    tmp___0 = malloc(::std::mem::size_of::<QRinput>() as libc::c_ulong);
    input = tmp___0 as *mut QRinput;
    if input as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut QRinput;
    }
    (*input).head = 0 as *mut libc::c_void as *mut QRinput_List;
    (*input).tail = 0 as *mut libc::c_void as *mut QRinput_List;
    (*input).version = version;
    (*input).level = level;
    (*input).mqr = 0 as libc::c_int;
    (*input).fnc1 = 0 as libc::c_int;
    return input;
}
pub unsafe extern "C" fn QRinput_newMQR(
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> *mut QRinput {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    if !(version <= 0 as libc::c_int) {
        if !(version > 4 as libc::c_int) {
            tmp = MQRspec_getECCLength(version, level);
            if !(tmp == 0 as libc::c_int) {
                input = QRinput_new2(version, level);
                if input as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                    return 0 as *mut libc::c_void as *mut QRinput;
                }
                (*input).mqr = 1 as libc::c_int;
                return input;
            }
        }
    }
    tmp___0 = __errno_location();
    *tmp___0 = 22 as libc::c_int;
    return 0 as *mut libc::c_void as *mut QRinput;
}
pub unsafe extern "C" fn QRinput_getVersion(mut input: *mut QRinput) -> libc::c_int {
    return (*input).version;
}
pub unsafe extern "C" fn QRinput_setVersion(
    mut input: *mut QRinput,
    mut version: libc::c_int,
) -> libc::c_int {
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    if (*input).mqr != 0 {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
        return -(1 as libc::c_int);
    } else {
        if version < 0 as libc::c_int {
            tmp = __errno_location();
            *tmp = 22 as libc::c_int;
            return -(1 as libc::c_int);
        } else {
            if version > 40 as libc::c_int {
                tmp = __errno_location();
                *tmp = 22 as libc::c_int;
                return -(1 as libc::c_int);
            }
        }
    }
    (*input).version = version;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_getErrorCorrectionLevel(
    mut input: *mut QRinput,
) -> QRecLevel {
    return (*input).level;
}
pub unsafe extern "C" fn QRinput_setErrorCorrectionLevel(
    mut input: *mut QRinput,
    mut level: QRecLevel,
) -> libc::c_int {
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    if (*input).mqr != 0 {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
        return -(1 as libc::c_int);
    } else {
        if level as libc::c_uint > 3 as libc::c_uint {
            tmp = __errno_location();
            *tmp = 22 as libc::c_int;
            return -(1 as libc::c_int);
        }
    }
    (*input).level = level;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_setVersionAndErrorCorrectionLevel(
    mut input: *mut QRinput,
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> libc::c_int {
    let mut current_block: u64;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    if (*input).mqr != 0 {
        if version <= 0 as libc::c_int {
            current_block = 10093325537592258782;
        } else if version > 4 as libc::c_int {
            current_block = 10093325537592258782;
        } else {
            tmp = MQRspec_getECCLength(version, level);
            if tmp == 0 as libc::c_int {
                current_block = 10093325537592258782;
            } else {
                current_block = 1841672684692190573;
            }
        }
    } else if version < 0 as libc::c_int {
        current_block = 10093325537592258782;
    } else if version > 40 as libc::c_int {
        current_block = 10093325537592258782;
    } else if level as libc::c_uint > 3 as libc::c_uint {
        current_block = 10093325537592258782;
    } else {
        current_block = 1841672684692190573;
    }
    match current_block {
        10093325537592258782 => {
            tmp___0 = __errno_location();
            *tmp___0 = 22 as libc::c_int;
            return -(1 as libc::c_int);
        }
        _ => {
            (*input).version = version;
            (*input).level = level;
            return 0 as libc::c_int;
        }
    };
}
unsafe extern "C" fn QRinput_appendEntry(
    mut input: *mut QRinput,
    mut entry: *mut QRinput_List,
) {
    if (*input).tail as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        (*input).head = entry;
        (*input).tail = entry;
    } else {
        (*(*input).tail).next = entry;
        (*input).tail = entry;
    }
    (*entry).next = 0 as *mut libc::c_void as *mut QRinput_List;
}
pub unsafe extern "C" fn QRinput_append(
    mut input: *mut QRinput,
    mut mode: QRencodeMode,
    mut size: libc::c_int,
    mut data: *const libc::c_uchar,
) -> libc::c_int {
    let mut entry: *mut QRinput_List = 0 as *mut QRinput_List;
    entry = QRinput_List_newEntry(mode, size, data);
    if entry as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    QRinput_appendEntry(input, entry);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_insertStructuredAppendHeader(
    mut input: *mut QRinput,
    mut size: libc::c_int,
    mut number: libc::c_int,
    mut parity: libc::c_uchar,
) -> libc::c_int {
    let mut entry: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut buf: [libc::c_uchar; 3] = [0; 3];
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    if size > 16 as libc::c_int {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if number <= 0 as libc::c_int {
        tmp___0 = __errno_location();
        *tmp___0 = 22 as libc::c_int;
        return -(1 as libc::c_int);
    } else {
        if number > size {
            tmp___0 = __errno_location();
            *tmp___0 = 22 as libc::c_int;
            return -(1 as libc::c_int);
        }
    }
    buf[0 as libc::c_int as usize] = size as libc::c_uchar;
    buf[1 as libc::c_int as usize] = number as libc::c_uchar;
    buf[2 as libc::c_int as usize] = parity;
    entry = QRinput_List_newEntry(
        QR_MODE_STRUCTURE,
        3 as libc::c_int,
        buf.as_mut_ptr() as *const libc::c_uchar,
    );
    if entry as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    (*entry).next = (*input).head;
    (*input).head = entry;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_appendECIheader(
    mut input: *mut QRinput,
    mut ecinum: libc::c_uint,
) -> libc::c_int {
    let mut data: [libc::c_uchar; 4] = [0; 4];
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: libc::c_int = 0;
    if ecinum > 999999 as libc::c_uint {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    data[0 as libc::c_int as usize] = (ecinum & 255 as libc::c_uint) as libc::c_uchar;
    data[1 as libc::c_int
        as usize] = (ecinum >> 8 as libc::c_int & 255 as libc::c_uint) as libc::c_uchar;
    data[2 as libc::c_int
        as usize] = (ecinum >> 16 as libc::c_int & 255 as libc::c_uint) as libc::c_uchar;
    data[3 as libc::c_int
        as usize] = (ecinum >> 24 as libc::c_int & 255 as libc::c_uint) as libc::c_uchar;
    tmp___0 = QRinput_append(
        input,
        QR_MODE_ECI,
        4 as libc::c_int,
        data.as_mut_ptr() as *const libc::c_uchar,
    );
    return tmp___0;
}
pub unsafe extern "C" fn QRinput_free(mut input: *mut QRinput) {
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut next: *mut QRinput_List = 0 as *mut QRinput_List;
    if input as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        list = (*input).head;
        while list as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            next = (*list).next;
            QRinput_List_freeEntry(list);
            list = next;
        }
        free(input as *mut libc::c_void);
    }
}
unsafe extern "C" fn QRinput_calcParity(mut input: *mut QRinput) -> libc::c_uchar {
    let mut parity: libc::c_uchar = 0;
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut i: libc::c_int = 0;
    parity = 0 as libc::c_int as libc::c_uchar;
    list = (*input).head;
    while list as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        if (*list).mode as libc::c_int != 4 as libc::c_int {
            i = (*list).size - 1 as libc::c_int;
            while i >= 0 as libc::c_int {
                parity = (parity as libc::c_int
                    ^ *((*list).data).offset(i as isize) as libc::c_int)
                    as libc::c_uchar;
                i -= 1;
            }
        }
        list = (*list).next;
    }
    return parity;
}
pub unsafe extern "C" fn QRinput_dup(mut input: *mut QRinput) -> *mut QRinput {
    let mut n: *mut QRinput = 0 as *mut QRinput;
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut e: *mut QRinput_List = 0 as *mut QRinput_List;
    if (*input).mqr != 0 {
        n = QRinput_newMQR((*input).version, (*input).level);
    } else {
        n = QRinput_new2((*input).version, (*input).level);
    }
    if n as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut QRinput;
    }
    list = (*input).head;
    while list as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        e = QRinput_List_dup(list);
        if e as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            QRinput_free(n);
            return 0 as *mut libc::c_void as *mut QRinput;
        }
        QRinput_appendEntry(n, e);
        list = (*list).next;
    }
    return n;
}
unsafe extern "C" fn QRinput_checkModeNum(
    mut size: libc::c_int,
    mut data: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        if (*data.offset(i as isize) as libc::c_int) < 48 as libc::c_int {
            return -(1 as libc::c_int)
        } else {
            if *data.offset(i as isize) as libc::c_int > 57 as libc::c_int {
                return -(1 as libc::c_int);
            }
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_estimateBitsModeNum(
    mut size: libc::c_int,
) -> libc::c_int {
    let mut w: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    w = size / 3 as libc::c_int;
    bits = w * 10 as libc::c_int;
    match size - w * 3 as libc::c_int {
        1 => {
            bits += 4 as libc::c_int;
        }
        2 => {
            bits += 7 as libc::c_int;
        }
        _ => {}
    }
    return bits;
}
unsafe extern "C" fn QRinput_encodeModeNum(
    mut entry: *mut QRinput_List,
    mut bstream: *mut BitStream,
    mut version: libc::c_int,
    mut mqr: libc::c_int,
) -> libc::c_int {
    let mut words: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut val: libc::c_uint = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    if mqr != 0 {
        if version > 1 as libc::c_int {
            ret = BitStream_appendNum(
                bstream,
                (version - 1 as libc::c_int) as size_t,
                0 as libc::c_uint,
            );
            if ret < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
        }
        tmp = MQRspec_lengthIndicator(QR_MODE_NUM, version);
        ret = BitStream_appendNum(bstream, tmp as size_t, (*entry).size as libc::c_uint);
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    } else {
        ret = BitStream_appendNum(
            bstream,
            4 as libc::c_int as size_t,
            1 as libc::c_uint,
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        tmp___0 = QRspec_lengthIndicator(QR_MODE_NUM, version);
        ret = BitStream_appendNum(
            bstream,
            tmp___0 as size_t,
            (*entry).size as libc::c_uint,
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    words = (*entry).size / 3 as libc::c_int;
    i = 0 as libc::c_int;
    while i < words {
        val = ((*((*entry).data).offset((i * 3 as libc::c_int) as isize) as libc::c_int
            - 48 as libc::c_int) as libc::c_uint)
            .wrapping_mul(100 as libc::c_uint);
        val = val
            .wrapping_add(
                ((*((*entry).data)
                    .offset((i * 3 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int - 48 as libc::c_int) as libc::c_uint)
                    .wrapping_mul(10 as libc::c_uint),
            );
        val = val
            .wrapping_add(
                (*((*entry).data)
                    .offset((i * 3 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int - 48 as libc::c_int) as libc::c_uint,
            );
        ret = BitStream_appendNum(bstream, 10 as libc::c_int as size_t, val);
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        i += 1;
    }
    if (*entry).size - words * 3 as libc::c_int == 1 as libc::c_int {
        val = (*((*entry).data).offset((words * 3 as libc::c_int) as isize)
            as libc::c_int - 48 as libc::c_int) as libc::c_uint;
        ret = BitStream_appendNum(bstream, 4 as libc::c_int as size_t, val);
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    } else if (*entry).size - words * 3 as libc::c_int == 2 as libc::c_int {
        val = ((*((*entry).data).offset((words * 3 as libc::c_int) as isize)
            as libc::c_int - 48 as libc::c_int) as libc::c_uint)
            .wrapping_mul(10 as libc::c_uint);
        val = val
            .wrapping_add(
                (*((*entry).data)
                    .offset((words * 3 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int - 48 as libc::c_int) as libc::c_uint,
            );
        ret = BitStream_appendNum(bstream, 7 as libc::c_int as size_t, val);
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
pub static mut QRinput_anTable: [libc::c_schar; 128] = [
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    36 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    37 as libc::c_int as libc::c_schar,
    38 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    39 as libc::c_int as libc::c_schar,
    40 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    41 as libc::c_int as libc::c_schar,
    42 as libc::c_int as libc::c_schar,
    43 as libc::c_int as libc::c_schar,
    0 as libc::c_int as libc::c_schar,
    1 as libc::c_int as libc::c_schar,
    2 as libc::c_int as libc::c_schar,
    3 as libc::c_int as libc::c_schar,
    4 as libc::c_int as libc::c_schar,
    5 as libc::c_int as libc::c_schar,
    6 as libc::c_int as libc::c_schar,
    7 as libc::c_int as libc::c_schar,
    8 as libc::c_int as libc::c_schar,
    9 as libc::c_int as libc::c_schar,
    44 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    10 as libc::c_int as libc::c_schar,
    11 as libc::c_int as libc::c_schar,
    12 as libc::c_int as libc::c_schar,
    13 as libc::c_int as libc::c_schar,
    14 as libc::c_int as libc::c_schar,
    15 as libc::c_int as libc::c_schar,
    16 as libc::c_int as libc::c_schar,
    17 as libc::c_int as libc::c_schar,
    18 as libc::c_int as libc::c_schar,
    19 as libc::c_int as libc::c_schar,
    20 as libc::c_int as libc::c_schar,
    21 as libc::c_int as libc::c_schar,
    22 as libc::c_int as libc::c_schar,
    23 as libc::c_int as libc::c_schar,
    24 as libc::c_int as libc::c_schar,
    25 as libc::c_int as libc::c_schar,
    26 as libc::c_int as libc::c_schar,
    27 as libc::c_int as libc::c_schar,
    28 as libc::c_int as libc::c_schar,
    29 as libc::c_int as libc::c_schar,
    30 as libc::c_int as libc::c_schar,
    31 as libc::c_int as libc::c_schar,
    32 as libc::c_int as libc::c_schar,
    33 as libc::c_int as libc::c_schar,
    34 as libc::c_int as libc::c_schar,
    35 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
];
unsafe extern "C" fn QRinput_checkModeAn(
    mut size: libc::c_int,
    mut data: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        if *data.offset(i as isize) as libc::c_int & 128 as libc::c_int != 0 {
            tmp = -(1 as libc::c_int);
        } else {
            tmp = QRinput_anTable[*data.offset(i as isize) as libc::c_int as usize]
                as libc::c_int;
        }
        if tmp < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_estimateBitsModeAn(
    mut size: libc::c_int,
) -> libc::c_int {
    let mut w: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    w = size / 2 as libc::c_int;
    bits = w * 11 as libc::c_int;
    if size & 1 as libc::c_int != 0 {
        bits += 6 as libc::c_int;
    }
    return bits;
}
unsafe extern "C" fn QRinput_encodeModeAn(
    mut entry: *mut QRinput_List,
    mut bstream: *mut BitStream,
    mut version: libc::c_int,
    mut mqr: libc::c_int,
) -> libc::c_int {
    let mut words: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut val: libc::c_uint = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    if mqr != 0 {
        if version < 2 as libc::c_int {
            tmp = __errno_location();
            *tmp = 34 as libc::c_int;
            return -(1 as libc::c_int);
        }
        ret = BitStream_appendNum(
            bstream,
            (version - 1 as libc::c_int) as size_t,
            1 as libc::c_uint,
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        tmp___0 = MQRspec_lengthIndicator(QR_MODE_AN, version);
        ret = BitStream_appendNum(
            bstream,
            tmp___0 as size_t,
            (*entry).size as libc::c_uint,
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    } else {
        ret = BitStream_appendNum(
            bstream,
            4 as libc::c_int as size_t,
            2 as libc::c_uint,
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        tmp___1 = QRspec_lengthIndicator(QR_MODE_AN, version);
        ret = BitStream_appendNum(
            bstream,
            tmp___1 as size_t,
            (*entry).size as libc::c_uint,
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    words = (*entry).size / 2 as libc::c_int;
    i = 0 as libc::c_int;
    while i < words {
        if *((*entry).data).offset((i * 2 as libc::c_int) as isize) as libc::c_int
            & 128 as libc::c_int != 0
        {
            tmp___2 = -(1 as libc::c_int);
        } else {
            tmp___2 = QRinput_anTable[*((*entry).data)
                .offset((i * 2 as libc::c_int) as isize) as libc::c_int as usize]
                as libc::c_int;
        }
        val = (tmp___2 as libc::c_uint).wrapping_mul(45 as libc::c_uint);
        if *((*entry).data).offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize)
            as libc::c_int & 128 as libc::c_int != 0
        {
            tmp___3 = -(1 as libc::c_int);
        } else {
            tmp___3 = QRinput_anTable[*((*entry).data)
                .offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize)
                as libc::c_int as usize] as libc::c_int;
        }
        val = val.wrapping_add(tmp___3 as libc::c_uint);
        ret = BitStream_appendNum(bstream, 11 as libc::c_int as size_t, val);
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        i += 1;
    }
    if (*entry).size & 1 as libc::c_int != 0 {
        if *((*entry).data).offset((words * 2 as libc::c_int) as isize) as libc::c_int
            & 128 as libc::c_int != 0
        {
            tmp___4 = -(1 as libc::c_int);
        } else {
            tmp___4 = QRinput_anTable[*((*entry).data)
                .offset((words * 2 as libc::c_int) as isize) as libc::c_int as usize]
                as libc::c_int;
        }
        val = tmp___4 as libc::c_uint;
        ret = BitStream_appendNum(bstream, 6 as libc::c_int as size_t, val);
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_estimateBitsMode8(
    mut size: libc::c_int,
) -> libc::c_int {
    return size * 8 as libc::c_int;
}
unsafe extern "C" fn QRinput_encodeMode8(
    mut entry: *mut QRinput_List,
    mut bstream: *mut BitStream,
    mut version: libc::c_int,
    mut mqr: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    if mqr != 0 {
        if version < 3 as libc::c_int {
            tmp = __errno_location();
            *tmp = 34 as libc::c_int;
            return -(1 as libc::c_int);
        }
        ret = BitStream_appendNum(
            bstream,
            (version - 1 as libc::c_int) as size_t,
            2 as libc::c_uint,
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        tmp___0 = MQRspec_lengthIndicator(QR_MODE_8, version);
        ret = BitStream_appendNum(
            bstream,
            tmp___0 as size_t,
            (*entry).size as libc::c_uint,
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    } else {
        ret = BitStream_appendNum(
            bstream,
            4 as libc::c_int as size_t,
            4 as libc::c_uint,
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        tmp___1 = QRspec_lengthIndicator(QR_MODE_8, version);
        ret = BitStream_appendNum(
            bstream,
            tmp___1 as size_t,
            (*entry).size as libc::c_uint,
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    ret = BitStream_appendBytes(bstream, (*entry).size as size_t, (*entry).data);
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_estimateBitsModeKanji(
    mut size: libc::c_int,
) -> libc::c_int {
    return size / 2 as libc::c_int * 13 as libc::c_int;
}
unsafe extern "C" fn QRinput_checkModeKanji(
    mut size: libc::c_int,
    mut data: *const libc::c_uchar,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut val: libc::c_uint = 0;
    if size & 1 as libc::c_int != 0 {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < size {
        val = (*data.offset(i as isize) as libc::c_uint) << 8 as libc::c_int
            | *data.offset((i + 1 as libc::c_int) as isize) as libc::c_uint;
        if val < 33088 as libc::c_uint {
            return -(1 as libc::c_int)
        } else {
            if val > 40956 as libc::c_uint {
                if val < 57408 as libc::c_uint {
                    return -(1 as libc::c_int);
                }
            }
            if val > 60351 as libc::c_uint {
                return -(1 as libc::c_int);
            }
        }
        i += 2 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn QRinput_encodeModeKanji(
    mut entry: *mut QRinput_List,
    mut bstream: *mut BitStream,
    mut version: libc::c_int,
    mut mqr: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut val: libc::c_uint = 0;
    let mut h: libc::c_uint = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    if mqr != 0 {
        if version < 2 as libc::c_int {
            tmp = __errno_location();
            *tmp = 34 as libc::c_int;
            return -(1 as libc::c_int);
        }
        ret = BitStream_appendNum(
            bstream,
            (version - 1 as libc::c_int) as size_t,
            3 as libc::c_uint,
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        tmp___0 = MQRspec_lengthIndicator(QR_MODE_KANJI, version);
        ret = BitStream_appendNum(
            bstream,
            tmp___0 as size_t,
            ((*entry).size as libc::c_uint).wrapping_div(2 as libc::c_uint),
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    } else {
        ret = BitStream_appendNum(
            bstream,
            4 as libc::c_int as size_t,
            8 as libc::c_uint,
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        tmp___1 = QRspec_lengthIndicator(QR_MODE_KANJI, version);
        ret = BitStream_appendNum(
            bstream,
            tmp___1 as size_t,
            ((*entry).size as libc::c_uint).wrapping_div(2 as libc::c_uint),
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    i = 0 as libc::c_int;
    while i < (*entry).size {
        val = (*((*entry).data).offset(i as isize) as libc::c_uint) << 8 as libc::c_int
            | *((*entry).data).offset((i + 1 as libc::c_int) as isize) as libc::c_uint;
        if val <= 40956 as libc::c_uint {
            val = val.wrapping_sub(33088 as libc::c_uint);
        } else {
            val = val.wrapping_sub(49472 as libc::c_uint);
        }
        h = (val >> 8 as libc::c_int).wrapping_mul(192 as libc::c_uint);
        val = (val & 255 as libc::c_uint).wrapping_add(h);
        ret = BitStream_appendNum(bstream, 13 as libc::c_int as size_t, val);
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        i += 2 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn QRinput_encodeModeStructure(
    mut entry: *mut QRinput_List,
    mut bstream: *mut BitStream,
    mut mqr: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    if mqr != 0 {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    ret = BitStream_appendNum(bstream, 4 as libc::c_int as size_t, 3 as libc::c_uint);
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    ret = BitStream_appendNum(
        bstream,
        4 as libc::c_int as size_t,
        (*((*entry).data).offset(1 as libc::c_int as isize) as libc::c_uint)
            .wrapping_sub(1 as libc::c_uint),
    );
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    ret = BitStream_appendNum(
        bstream,
        4 as libc::c_int as size_t,
        (*((*entry).data).offset(0 as libc::c_int as isize) as libc::c_uint)
            .wrapping_sub(1 as libc::c_uint),
    );
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    ret = BitStream_appendNum(
        bstream,
        8 as libc::c_int as size_t,
        *((*entry).data).offset(2 as libc::c_int as isize) as libc::c_uint,
    );
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn QRinput_checkModeFNC1Second(mut size: libc::c_int) -> libc::c_int {
    if size != 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn QRinput_encodeModeFNC1Second(
    mut entry: *mut QRinput_List,
    mut bstream: *mut BitStream,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = BitStream_appendNum(bstream, 4 as libc::c_int as size_t, 9 as libc::c_uint);
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    ret = BitStream_appendBytes(bstream, 1 as libc::c_int as size_t, (*entry).data);
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn QRinput_decodeECIfromByteArray(
    mut data: *mut libc::c_uchar,
) -> libc::c_uint {
    let mut i: libc::c_int = 0;
    let mut ecinum: libc::c_uint = 0;
    ecinum = 0 as libc::c_uint;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        ecinum <<= 8 as libc::c_int;
        ecinum |= *data.offset((3 as libc::c_int - i) as isize) as libc::c_uint;
        i += 1;
    }
    return ecinum;
}
unsafe extern "C" fn QRinput_estimateBitsModeECI(
    mut data: *mut libc::c_uchar,
) -> libc::c_int {
    let mut ecinum: libc::c_uint = 0;
    ecinum = QRinput_decodeECIfromByteArray(data);
    if ecinum < 128 as libc::c_uint {
        return 12 as libc::c_int
    } else if ecinum < 16384 as libc::c_uint {
        return 20 as libc::c_int
    } else {
        return 28 as libc::c_int
    };
}
unsafe extern "C" fn QRinput_encodeModeECI(
    mut entry: *mut QRinput_List,
    mut bstream: *mut BitStream,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut words: libc::c_int = 0;
    let mut ecinum: libc::c_uint = 0;
    let mut code: libc::c_uint = 0;
    ecinum = QRinput_decodeECIfromByteArray((*entry).data);
    if ecinum < 128 as libc::c_uint {
        words = 1 as libc::c_int;
        code = ecinum;
    } else if ecinum < 16384 as libc::c_uint {
        words = 2 as libc::c_int;
        code = (32768 as libc::c_uint).wrapping_add(ecinum);
    } else {
        words = 3 as libc::c_int;
        code = (786432 as libc::c_uint).wrapping_add(ecinum);
    }
    ret = BitStream_appendNum(bstream, 4 as libc::c_int as size_t, 7 as libc::c_uint);
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    ret = BitStream_appendNum(
        bstream,
        (words as size_t).wrapping_mul(8 as libc::c_ulong),
        code,
    );
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_check(
    mut mode: QRencodeMode,
    mut size: libc::c_int,
    mut data: *const libc::c_uchar,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    if mode as libc::c_int == 6 as libc::c_int {
        if size < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    if size <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    match mode as libc::c_int {
        0 => {
            tmp = QRinput_checkModeNum(size, data as *const libc::c_char);
            return tmp;
        }
        1 => {
            tmp___0 = QRinput_checkModeAn(size, data as *const libc::c_char);
            return tmp___0;
        }
        3 => {
            tmp___1 = QRinput_checkModeKanji(size, data);
            return tmp___1;
        }
        2 => return 0 as libc::c_int,
        4 => return 0 as libc::c_int,
        5 => return 0 as libc::c_int,
        6 => return 0 as libc::c_int,
        7 => {
            tmp___2 = QRinput_checkModeFNC1Second(size);
            return tmp___2;
        }
        -1 | _ => {}
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn QRinput_estimateBitStreamSizeOfEntry(
    mut entry: *mut QRinput_List,
    mut version: libc::c_int,
    mut mqr: libc::c_int,
) -> libc::c_int {
    let mut bits: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    bits = 0 as libc::c_int;
    if version == 0 as libc::c_int {
        version = 1 as libc::c_int;
    }
    match (*entry).mode as libc::c_int {
        0 => {
            bits = QRinput_estimateBitsModeNum((*entry).size);
        }
        1 => {
            bits = QRinput_estimateBitsModeAn((*entry).size);
        }
        2 => {
            bits = QRinput_estimateBitsMode8((*entry).size);
        }
        3 => {
            bits = QRinput_estimateBitsModeKanji((*entry).size);
        }
        4 => return 20 as libc::c_int,
        5 => {
            bits = QRinput_estimateBitsModeECI((*entry).data);
        }
        6 => return 4 as libc::c_int,
        7 => return 12 as libc::c_int,
        _ => return 0 as libc::c_int,
    }
    if mqr != 0 {
        l = MQRspec_lengthIndicator((*entry).mode, version);
        m = version - 1 as libc::c_int;
        bits += l + m;
    } else {
        l = QRspec_lengthIndicator((*entry).mode, version);
        m = (1 as libc::c_int) << l;
        if (*entry).mode as libc::c_int == 3 as libc::c_int {
            num = ((*entry).size / 2 as libc::c_int + m - 1 as libc::c_int) / m;
        } else {
            num = ((*entry).size + m - 1 as libc::c_int) / m;
        }
        bits += num * (4 as libc::c_int + l);
    }
    return bits;
}
pub unsafe extern "C" fn QRinput_estimateBitStreamSize(
    mut input: *mut QRinput,
    mut version: libc::c_int,
) -> libc::c_int {
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut bits: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    bits = 0 as libc::c_int;
    list = (*input).head;
    while list as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp = QRinput_estimateBitStreamSizeOfEntry(list, version, (*input).mqr);
        bits += tmp;
        list = (*list).next;
    }
    return bits;
}
pub unsafe extern "C" fn QRinput_estimateVersion(
    mut input: *mut QRinput,
) -> libc::c_int {
    let mut bits: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut prev: libc::c_int = 0;
    version = 0 as libc::c_int;
    loop {
        prev = version;
        bits = QRinput_estimateBitStreamSize(input, prev);
        version = QRspec_getMinimumVersion(
            (bits + 7 as libc::c_int) / 8 as libc::c_int,
            (*input).level,
        );
        if prev == 0 as libc::c_int {
            if version > 1 as libc::c_int {
                version -= 1;
            }
        }
        if !(version > prev) {
            break;
        }
    }
    return version;
}
pub unsafe extern "C" fn QRinput_lengthOfCode(
    mut mode: QRencodeMode,
    mut version: libc::c_int,
    mut bits: libc::c_int,
) -> libc::c_int {
    let mut payload: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut chunks: libc::c_int = 0;
    let mut remain: libc::c_int = 0;
    let mut maxsize: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tmp = QRspec_lengthIndicator(mode, version);
    payload = bits - 4 as libc::c_int - tmp;
    match mode as libc::c_int {
        0 => {
            chunks = payload / 10 as libc::c_int;
            remain = payload - chunks * 10 as libc::c_int;
            size = chunks * 3 as libc::c_int;
            if remain >= 7 as libc::c_int {
                size += 2 as libc::c_int;
            } else if remain >= 4 as libc::c_int {
                size += 1;
            }
        }
        1 => {
            chunks = payload / 11 as libc::c_int;
            remain = payload - chunks * 11 as libc::c_int;
            size = chunks * 2 as libc::c_int;
            if remain >= 6 as libc::c_int {
                size += 1;
            }
        }
        2 => {
            size = payload / 8 as libc::c_int;
        }
        3 => {
            size = payload / 13 as libc::c_int * 2 as libc::c_int;
        }
        4 => {
            size = payload / 8 as libc::c_int;
        }
        _ => {
            size = 0 as libc::c_int;
        }
    }
    maxsize = QRspec_maximumWords(mode, version);
    if size < 0 as libc::c_int {
        size = 0 as libc::c_int;
    }
    if maxsize > 0 as libc::c_int {
        if size > maxsize {
            size = maxsize;
        }
    }
    return size;
}
unsafe extern "C" fn QRinput_encodeBitStream(
    mut entry: *mut QRinput_List,
    mut bstream: *mut BitStream,
    mut version: libc::c_int,
    mut mqr: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut words: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut st1: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut st2: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut prevsize: libc::c_int = 0;
    st1 = 0 as *mut libc::c_void as *mut QRinput_List;
    st2 = 0 as *mut libc::c_void as *mut QRinput_List;
    prevsize = (*bstream).length as libc::c_int;
    if mqr != 0 {
        words = MQRspec_maximumWords((*entry).mode, version);
    } else {
        words = QRspec_maximumWords((*entry).mode, version);
    }
    if words != 0 as libc::c_int {
        if (*entry).size > words {
            st1 = QRinput_List_newEntry(
                (*entry).mode,
                words,
                (*entry).data as *const libc::c_uchar,
            );
            if st1 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                current_block = 423349656505572008;
            } else {
                st2 = QRinput_List_newEntry(
                    (*entry).mode,
                    (*entry).size - words,
                    ((*entry).data).offset(words as isize) as *const libc::c_uchar,
                );
                if st2 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                    current_block = 423349656505572008;
                } else {
                    ret = QRinput_encodeBitStream(st1, bstream, version, mqr);
                    if ret < 0 as libc::c_int {
                        current_block = 423349656505572008;
                    } else {
                        ret = QRinput_encodeBitStream(st2, bstream, version, mqr);
                        if ret < 0 as libc::c_int {
                            current_block = 423349656505572008;
                        } else {
                            QRinput_List_freeEntry(st1);
                            QRinput_List_freeEntry(st2);
                            current_block = 2604890879466389055;
                        }
                    }
                }
            }
            match current_block {
                2604890879466389055 => {}
                _ => {
                    QRinput_List_freeEntry(st1);
                    QRinput_List_freeEntry(st2);
                    return -(1 as libc::c_int);
                }
            }
        } else {
            current_block = 4757021903413980562;
        }
    } else {
        current_block = 4757021903413980562;
    }
    match current_block {
        4757021903413980562 => {
            ret = 0 as libc::c_int;
            match (*entry).mode as libc::c_int {
                0 => {
                    ret = QRinput_encodeModeNum(entry, bstream, version, mqr);
                }
                1 => {
                    ret = QRinput_encodeModeAn(entry, bstream, version, mqr);
                }
                2 => {
                    ret = QRinput_encodeMode8(entry, bstream, version, mqr);
                }
                3 => {
                    ret = QRinput_encodeModeKanji(entry, bstream, version, mqr);
                }
                4 => {
                    ret = QRinput_encodeModeStructure(entry, bstream, mqr);
                }
                5 => {
                    ret = QRinput_encodeModeECI(entry, bstream);
                }
                7 => {
                    ret = QRinput_encodeModeFNC1Second(entry, bstream);
                }
                _ => {}
            }
            if ret < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
        }
        _ => {}
    }
    return (*bstream).length as libc::c_int - prevsize;
}
unsafe extern "C" fn QRinput_createBitStream(
    mut input: *mut QRinput,
    mut bstream: *mut BitStream,
) -> libc::c_int {
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut bits: libc::c_int = 0;
    let mut total: libc::c_int = 0;
    total = 0 as libc::c_int;
    list = (*input).head;
    while list as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        bits = QRinput_encodeBitStream(list, bstream, (*input).version, (*input).mqr);
        if bits < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        total += bits;
        list = (*list).next;
    }
    return total;
}
unsafe extern "C" fn QRinput_convertData(
    mut input: *mut QRinput,
    mut bstream: *mut BitStream,
) -> libc::c_int {
    let mut bits: libc::c_int = 0;
    let mut ver: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    ver = QRinput_estimateVersion(input);
    tmp = QRinput_getVersion(input);
    if ver > tmp {
        QRinput_setVersion(input, ver);
    }
    loop {
        (*bstream).length = 0 as libc::c_int as size_t;
        bits = QRinput_createBitStream(input, bstream);
        if bits < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        ver = QRspec_getMinimumVersion(
            (bits + 7 as libc::c_int) / 8 as libc::c_int,
            (*input).level,
        );
        tmp___0 = QRinput_getVersion(input);
        if !(ver > tmp___0) {
            break;
        }
        QRinput_setVersion(input, ver);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn QRinput_appendPaddingBit(
    mut bstream: *mut BitStream,
    mut input: *mut QRinput,
) -> libc::c_int {
    let mut bits: libc::c_int = 0;
    let mut maxbits: libc::c_int = 0;
    let mut words: libc::c_int = 0;
    let mut maxwords: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut padlen: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    bits = (*bstream).length as libc::c_int;
    maxwords = QRspec_getDataLength((*input).version, (*input).level);
    maxbits = maxwords * 8 as libc::c_int;
    if maxbits < bits {
        tmp = __errno_location();
        *tmp = 34 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if maxbits == bits {
        return 0 as libc::c_int;
    }
    if maxbits - bits <= 4 as libc::c_int {
        tmp___0 = BitStream_appendNum(
            bstream,
            (maxbits - bits) as size_t,
            0 as libc::c_uint,
        );
        return tmp___0;
    }
    words = (bits + 4 as libc::c_int + 7 as libc::c_int) / 8 as libc::c_int;
    tmp___1 = BitStream_appendNum(
        bstream,
        (words * 8 as libc::c_int - bits) as size_t,
        0 as libc::c_uint,
    );
    ret = tmp___1;
    if ret < 0 as libc::c_int {
        return ret;
    }
    padlen = maxwords - words;
    if padlen > 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < padlen {
            if i & 1 as libc::c_int != 0 {
                tmp___2 = 17 as libc::c_int;
            } else {
                tmp___2 = 236 as libc::c_int;
            }
            tmp___3 = BitStream_appendNum(
                bstream,
                8 as libc::c_int as size_t,
                tmp___2 as libc::c_uint,
            );
            ret = tmp___3;
            if ret < 0 as libc::c_int {
                return ret;
            }
            i += 1;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn QRinput_appendPaddingBitMQR(
    mut bstream: *mut BitStream,
    mut input: *mut QRinput,
) -> libc::c_int {
    let mut bits: libc::c_int = 0;
    let mut maxbits: libc::c_int = 0;
    let mut words: libc::c_int = 0;
    let mut maxwords: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut termbits: libc::c_int = 0;
    let mut padlen: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    bits = (*bstream).length as libc::c_int;
    maxbits = MQRspec_getDataLengthBit((*input).version, (*input).level);
    maxwords = maxbits / 8 as libc::c_int;
    if maxbits < bits {
        tmp = __errno_location();
        *tmp = 34 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if maxbits == bits {
        return 0 as libc::c_int;
    }
    termbits = (*input).version * 2 as libc::c_int + 1 as libc::c_int;
    if maxbits - bits <= termbits {
        tmp___0 = BitStream_appendNum(
            bstream,
            (maxbits - bits) as size_t,
            0 as libc::c_uint,
        );
        return tmp___0;
    }
    bits += termbits;
    words = (bits + 7 as libc::c_int) / 8 as libc::c_int;
    if maxbits - words * 8 as libc::c_int > 0 as libc::c_int {
        termbits += words * 8 as libc::c_int - bits;
        if words == maxwords {
            termbits += maxbits - words * 8 as libc::c_int;
        }
    } else {
        termbits += words * 8 as libc::c_int - bits;
    }
    tmp___1 = BitStream_appendNum(bstream, termbits as size_t, 0 as libc::c_uint);
    ret = tmp___1;
    if ret < 0 as libc::c_int {
        return ret;
    }
    padlen = maxwords - words;
    if padlen > 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < padlen {
            if i & 1 as libc::c_int != 0 {
                tmp___2 = 17 as libc::c_int;
            } else {
                tmp___2 = 236 as libc::c_int;
            }
            tmp___3 = BitStream_appendNum(
                bstream,
                8 as libc::c_int as size_t,
                tmp___2 as libc::c_uint,
            );
            ret = tmp___3;
            if ret < 0 as libc::c_int {
                return ret;
            }
            i += 1;
        }
        termbits = maxbits - maxwords * 8 as libc::c_int;
        if termbits > 0 as libc::c_int {
            tmp___4 = BitStream_appendNum(
                bstream,
                termbits as size_t,
                0 as libc::c_uint,
            );
            ret = tmp___4;
            if ret < 0 as libc::c_int {
                return ret;
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn QRinput_insertFNC1Header(mut input: *mut QRinput) -> libc::c_int {
    let mut entry: *mut QRinput_List = 0 as *mut QRinput_List;
    entry = 0 as *mut libc::c_void as *mut QRinput_List;
    if (*input).fnc1 == 1 as libc::c_int {
        entry = QRinput_List_newEntry(
            QR_MODE_FNC1FIRST,
            0 as libc::c_int,
            0 as *mut libc::c_void as *const libc::c_uchar,
        );
    } else if (*input).fnc1 == 2 as libc::c_int {
        entry = QRinput_List_newEntry(
            QR_MODE_FNC1SECOND,
            1 as libc::c_int,
            &mut (*input).appid as *mut libc::c_uchar as *const libc::c_uchar,
        );
    }
    if entry as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    if (*(*input).head).mode as libc::c_int != 4 as libc::c_int {
        if (*(*input).head).mode as libc::c_int != 5 as libc::c_int {
            (*entry).next = (*input).head;
            (*input).head = entry;
        } else {
            (*entry).next = (*(*input).head).next;
            (*(*input).head).next = entry;
        }
    } else {
        (*entry).next = (*(*input).head).next;
        (*(*input).head).next = entry;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_mergeBitStream(
    mut input: *mut QRinput,
    mut bstream: *mut BitStream,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    if (*input).mqr != 0 {
        tmp = QRinput_createBitStream(input, bstream);
        if tmp < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    } else {
        if (*input).fnc1 != 0 {
            tmp___0 = QRinput_insertFNC1Header(input);
            if tmp___0 < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
        }
        tmp___1 = QRinput_convertData(input, bstream);
        if tmp___1 < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_getBitStream(
    mut input: *mut QRinput,
    mut bstream: *mut BitStream,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = QRinput_mergeBitStream(input, bstream);
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*input).mqr != 0 {
        ret = QRinput_appendPaddingBitMQR(bstream, input);
    } else {
        ret = QRinput_appendPaddingBit(bstream, input);
    }
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_getByteStream(
    mut input: *mut QRinput,
) -> *mut libc::c_uchar {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut array: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ret: libc::c_int = 0;
    bstream = BitStream_new();
    if bstream as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut libc::c_uchar;
    }
    ret = QRinput_getBitStream(input, bstream);
    if ret < 0 as libc::c_int {
        BitStream_free(bstream);
        return 0 as *mut libc::c_void as *mut libc::c_uchar;
    }
    array = BitStream_toByte(bstream);
    BitStream_free(bstream);
    return array;
}
unsafe extern "C" fn QRinput_InputList_newEntry(
    mut input: *mut QRinput,
) -> *mut QRinput_InputList {
    let mut entry: *mut QRinput_InputList = 0 as *mut QRinput_InputList;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<QRinput_InputList>() as libc::c_ulong);
    entry = tmp as *mut QRinput_InputList;
    if entry as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut QRinput_InputList;
    }
    (*entry).input = input;
    (*entry).next = 0 as *mut libc::c_void as *mut QRinput_InputList;
    return entry;
}
unsafe extern "C" fn QRinput_InputList_freeEntry(mut entry: *mut QRinput_InputList) {
    if entry as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        QRinput_free((*entry).input);
        free(entry as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn QRinput_Struct_new() -> *mut QRinput_Struct {
    let mut s: *mut QRinput_Struct = 0 as *mut QRinput_Struct;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<QRinput_Struct>() as libc::c_ulong);
    s = tmp as *mut QRinput_Struct;
    if s as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut QRinput_Struct;
    }
    (*s).size = 0 as libc::c_int;
    (*s).parity = -(1 as libc::c_int);
    (*s).head = 0 as *mut libc::c_void as *mut QRinput_InputList;
    (*s).tail = 0 as *mut libc::c_void as *mut QRinput_InputList;
    return s;
}
pub unsafe extern "C" fn QRinput_Struct_setParity(
    mut s: *mut QRinput_Struct,
    mut parity: libc::c_uchar,
) {
    (*s).parity = parity as libc::c_int;
}
pub unsafe extern "C" fn QRinput_Struct_appendInput(
    mut s: *mut QRinput_Struct,
    mut input: *mut QRinput,
) -> libc::c_int {
    let mut e: *mut QRinput_InputList = 0 as *mut QRinput_InputList;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    if (*input).mqr != 0 {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    e = QRinput_InputList_newEntry(input);
    if e as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    (*s).size += 1;
    if (*s).tail as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        (*s).head = e;
        (*s).tail = e;
    } else {
        (*(*s).tail).next = e;
        (*s).tail = e;
    }
    return (*s).size;
}
pub unsafe extern "C" fn QRinput_Struct_free(mut s: *mut QRinput_Struct) {
    let mut list: *mut QRinput_InputList = 0 as *mut QRinput_InputList;
    let mut next: *mut QRinput_InputList = 0 as *mut QRinput_InputList;
    if s as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        list = (*s).head;
        while list as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            next = (*list).next;
            QRinput_InputList_freeEntry(list);
            list = next;
        }
        free(s as *mut libc::c_void);
    }
}
unsafe extern "C" fn QRinput_Struct_calcParity(
    mut s: *mut QRinput_Struct,
) -> libc::c_uchar {
    let mut list: *mut QRinput_InputList = 0 as *mut QRinput_InputList;
    let mut parity: libc::c_uchar = 0;
    let mut tmp: libc::c_uchar = 0;
    parity = 0 as libc::c_int as libc::c_uchar;
    list = (*s).head;
    while list as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp = QRinput_calcParity((*list).input);
        parity = (parity as libc::c_int ^ tmp as libc::c_int) as libc::c_uchar;
        list = (*list).next;
    }
    QRinput_Struct_setParity(s, parity);
    return parity;
}
unsafe extern "C" fn QRinput_List_shrinkEntry(
    mut entry: *mut QRinput_List,
    mut bytes: libc::c_int,
) -> libc::c_int {
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(bytes as size_t);
    data = tmp as *mut libc::c_uchar;
    if data as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    memcpy(
        data as *mut libc::c_void,
        (*entry).data as *const libc::c_void,
        bytes as size_t,
    );
    free((*entry).data as *mut libc::c_void);
    (*entry).data = data;
    (*entry).size = bytes;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_splitEntry(
    mut entry: *mut QRinput_List,
    mut bytes: libc::c_int,
) -> libc::c_int {
    let mut e: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut ret: libc::c_int = 0;
    e = QRinput_List_newEntry(
        (*entry).mode,
        (*entry).size - bytes,
        ((*entry).data).offset(bytes as isize) as *const libc::c_uchar,
    );
    if e as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    ret = QRinput_List_shrinkEntry(entry, bytes);
    if ret < 0 as libc::c_int {
        QRinput_List_freeEntry(e);
        return -(1 as libc::c_int);
    }
    (*e).next = (*entry).next;
    (*entry).next = e;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_splitQRinputToStruct(
    mut input: *mut QRinput,
) -> *mut QRinput_Struct {
    let mut current_block: u64;
    let mut p: *mut QRinput = 0 as *mut QRinput;
    let mut s: *mut QRinput_Struct = 0 as *mut QRinput_Struct;
    let mut bits: libc::c_int = 0;
    let mut maxbits: libc::c_int = 0;
    let mut nextbits: libc::c_int = 0;
    let mut bytes: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut next: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut prev: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: libc::c_uchar = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    p = 0 as *mut libc::c_void as *mut QRinput;
    s = 0 as *mut libc::c_void as *mut QRinput_Struct;
    bstream = 0 as *mut libc::c_void as *mut BitStream;
    if (*input).mqr != 0 {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
        return 0 as *mut libc::c_void as *mut QRinput_Struct;
    }
    s = QRinput_Struct_new();
    if s as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut QRinput_Struct;
    }
    input = QRinput_dup(input);
    if input as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        QRinput_Struct_free(s);
        return 0 as *mut libc::c_void as *mut QRinput_Struct;
    }
    tmp___0 = QRinput_calcParity(input);
    QRinput_Struct_setParity(s, tmp___0);
    tmp___1 = QRspec_getDataLength((*input).version, (*input).level);
    maxbits = tmp___1 * 8 as libc::c_int - 20 as libc::c_int;
    if !(maxbits <= 0 as libc::c_int) {
        bstream = BitStream_new();
        if !(bstream as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
            bits = 0 as libc::c_int;
            list = (*input).head;
            prev = 0 as *mut libc::c_void as *mut QRinput_List;
            loop {
                if !(list as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
                    current_block = 9627623479216730126;
                    break;
                }
                nextbits = QRinput_estimateBitStreamSizeOfEntry(
                    list,
                    (*input).version,
                    (*input).mqr,
                );
                if bits + nextbits <= maxbits {
                    (*bstream).length = 0 as libc::c_int as size_t;
                    ret = QRinput_encodeBitStream(
                        list,
                        bstream,
                        (*input).version,
                        (*input).mqr,
                    );
                    if ret < 0 as libc::c_int {
                        current_block = 873342603503260231;
                        break;
                    }
                    bits += ret;
                    prev = list;
                    list = (*list).next;
                } else {
                    bytes = QRinput_lengthOfCode(
                        (*list).mode,
                        (*input).version,
                        maxbits - bits,
                    );
                    p = QRinput_new2((*input).version, (*input).level);
                    if p as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                        current_block = 873342603503260231;
                        break;
                    }
                    if bytes > 0 as libc::c_int {
                        ret = QRinput_splitEntry(list, bytes);
                        if ret < 0 as libc::c_int {
                            QRinput_free(p);
                            current_block = 873342603503260231;
                            break;
                        } else {
                            next = (*list).next;
                            (*list).next = 0 as *mut libc::c_void as *mut QRinput_List;
                            (*p).head = next;
                            (*p).tail = (*input).tail;
                            (*input).tail = list;
                            prev = list;
                            list = next;
                        }
                    } else {
                        (*prev).next = 0 as *mut libc::c_void as *mut QRinput_List;
                        (*p).head = list;
                        (*p).tail = (*input).tail;
                        (*input).tail = prev;
                    }
                    ret = QRinput_Struct_appendInput(s, input);
                    if ret < 0 as libc::c_int {
                        QRinput_free(p);
                        current_block = 873342603503260231;
                        break;
                    } else {
                        input = p;
                        bits = 0 as libc::c_int;
                    }
                }
            }
            match current_block {
                873342603503260231 => {}
                _ => {
                    ret = QRinput_Struct_appendInput(s, input);
                    if !(ret < 0 as libc::c_int) {
                        if (*s).size > 16 as libc::c_int {
                            tmp___2 = __errno_location();
                            *tmp___2 = 34 as libc::c_int;
                            QRinput_Struct_free(s);
                            BitStream_free(bstream);
                            return 0 as *mut libc::c_void as *mut QRinput_Struct;
                        }
                        ret = QRinput_Struct_insertStructuredAppendHeaders(s);
                        if ret < 0 as libc::c_int {
                            QRinput_Struct_free(s);
                            BitStream_free(bstream);
                            return 0 as *mut libc::c_void as *mut QRinput_Struct;
                        }
                        BitStream_free(bstream);
                        return s;
                    }
                }
            }
        }
    }
    BitStream_free(bstream);
    QRinput_free(input);
    QRinput_Struct_free(s);
    return 0 as *mut libc::c_void as *mut QRinput_Struct;
}
pub unsafe extern "C" fn QRinput_Struct_insertStructuredAppendHeaders(
    mut s: *mut QRinput_Struct,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut list: *mut QRinput_InputList = 0 as *mut QRinput_InputList;
    let mut tmp: libc::c_int = 0;
    if (*s).size == 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*s).parity < 0 as libc::c_int {
        QRinput_Struct_calcParity(s);
    }
    i = 1 as libc::c_int;
    list = (*s).head;
    while list as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp = QRinput_insertStructuredAppendHeader(
            (*list).input,
            (*s).size,
            i,
            (*s).parity as libc::c_uchar,
        );
        if tmp != 0 {
            return -(1 as libc::c_int);
        }
        i += 1;
        list = (*list).next;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_setFNC1First(mut input: *mut QRinput) -> libc::c_int {
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    if (*input).mqr != 0 {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    (*input).fnc1 = 1 as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_setFNC1Second(
    mut input: *mut QRinput,
    mut appid: libc::c_uchar,
) -> libc::c_int {
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    if (*input).mqr != 0 {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    (*input).fnc1 = 2 as libc::c_int;
    (*input).appid = appid;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn BitStream_new() -> *mut BitStream {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<BitStream>() as libc::c_ulong);
    bstream = tmp as *mut BitStream;
    if bstream as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut BitStream;
    }
    (*bstream).length = 0 as libc::c_int as size_t;
    tmp___0 = malloc(128 as libc::c_int as size_t);
    (*bstream).data = tmp___0 as *mut libc::c_uchar;
    if (*bstream).data as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        free(bstream as *mut libc::c_void);
        return 0 as *mut libc::c_void as *mut BitStream;
    }
    (*bstream).datasize = 128 as libc::c_int as size_t;
    return bstream;
}
pub unsafe extern "C" fn BitStream_newWithBits(
    mut size: size_t,
    mut bits: *mut libc::c_uchar,
) -> *mut BitStream {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut tmp: *mut BitStream = 0 as *mut BitStream;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    if size == 0 as libc::c_ulong {
        tmp = BitStream_new();
        return tmp;
    }
    tmp___0 = malloc(::std::mem::size_of::<BitStream>() as libc::c_ulong);
    bstream = tmp___0 as *mut BitStream;
    if bstream as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut BitStream;
    }
    tmp___1 = malloc(size);
    (*bstream).data = tmp___1 as *mut libc::c_uchar;
    if (*bstream).data as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        free(bstream as *mut libc::c_void);
        return 0 as *mut libc::c_void as *mut BitStream;
    }
    (*bstream).length = size;
    (*bstream).datasize = size;
    memcpy((*bstream).data as *mut libc::c_void, bits as *const libc::c_void, size);
    return bstream;
}
unsafe extern "C" fn BitStream_expand(mut bstream: *mut BitStream) -> libc::c_int {
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = realloc(
        (*bstream).data as *mut libc::c_void,
        ((*bstream).datasize).wrapping_mul(2 as libc::c_ulong),
    );
    data = tmp as *mut libc::c_uchar;
    if data as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    (*bstream).data = data;
    (*bstream)
        .datasize = ((*bstream).datasize as libc::c_ulong)
        .wrapping_mul(2 as libc::c_ulong) as size_t as size_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn BitStream_writeNum(
    mut dest: *mut libc::c_uchar,
    mut bits: size_t,
    mut num: libc::c_uint,
) {
    let mut mask: libc::c_uint = 0;
    let mut i: size_t = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    p = dest;
    mask = (1 as libc::c_uint) << bits.wrapping_sub(1 as libc::c_ulong);
    i = 0 as libc::c_int as size_t;
    while i < bits {
        if num & mask != 0 {
            *p = 1 as libc::c_int as libc::c_uchar;
        } else {
            *p = 0 as libc::c_int as libc::c_uchar;
        }
        p = p.offset(1);
        mask >>= 1 as libc::c_int;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn BitStream_writeBytes(
    mut dest: *mut libc::c_uchar,
    mut size: size_t,
    mut data: *mut libc::c_uchar,
) {
    let mut mask: libc::c_uchar = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    p = dest;
    i = 0 as libc::c_int as size_t;
    while i < size {
        mask = 128 as libc::c_int as libc::c_uchar;
        j = 0 as libc::c_int as size_t;
        while j < 8 as libc::c_ulong {
            if *data.offset(i as isize) as libc::c_int & mask as libc::c_int != 0 {
                *p = 1 as libc::c_int as libc::c_uchar;
            } else {
                *p = 0 as libc::c_int as libc::c_uchar;
            }
            p = p.offset(1);
            mask = (mask as libc::c_int >> 1 as libc::c_int) as libc::c_uchar;
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
}
pub unsafe extern "C" fn BitStream_append(
    mut bstream: *mut BitStream,
    mut arg: *mut BitStream,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if arg as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    if (*arg).length == 0 as libc::c_ulong {
        return 0 as libc::c_int;
    }
    while ((*bstream).length).wrapping_add((*arg).length) > (*bstream).datasize {
        ret = BitStream_expand(bstream);
        if ret < 0 as libc::c_int {
            return ret;
        }
    }
    memcpy(
        ((*bstream).data).offset((*bstream).length as isize) as *mut libc::c_void,
        (*arg).data as *const libc::c_void,
        (*arg).length,
    );
    (*bstream)
        .length = ((*bstream).length as libc::c_ulong).wrapping_add((*arg).length)
        as size_t as size_t;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn BitStream_appendNum(
    mut bstream: *mut BitStream,
    mut bits: size_t,
    mut num: libc::c_uint,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if bits == 0 as libc::c_ulong {
        return 0 as libc::c_int;
    }
    while ((*bstream).datasize).wrapping_sub((*bstream).length) < bits {
        ret = BitStream_expand(bstream);
        if ret < 0 as libc::c_int {
            return ret;
        }
    }
    BitStream_writeNum(((*bstream).data).offset((*bstream).length as isize), bits, num);
    (*bstream)
        .length = ((*bstream).length as libc::c_ulong).wrapping_add(bits) as size_t
        as size_t;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn BitStream_appendBytes(
    mut bstream: *mut BitStream,
    mut size: size_t,
    mut data: *mut libc::c_uchar,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if size == 0 as libc::c_ulong {
        return 0 as libc::c_int;
    }
    while ((*bstream).datasize).wrapping_sub((*bstream).length)
        < size.wrapping_mul(8 as libc::c_ulong)
    {
        ret = BitStream_expand(bstream);
        if ret < 0 as libc::c_int {
            return ret;
        }
    }
    BitStream_writeBytes(
        ((*bstream).data).offset((*bstream).length as isize),
        size,
        data,
    );
    (*bstream)
        .length = ((*bstream).length as libc::c_ulong)
        .wrapping_add(size.wrapping_mul(8 as libc::c_ulong)) as size_t as size_t;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn BitStream_toByte(
    mut bstream: *mut BitStream,
) -> *mut libc::c_uchar {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut size: size_t = 0;
    let mut bytes: size_t = 0;
    let mut oddbits: size_t = 0;
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut v: libc::c_uchar = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    size = (*bstream).length;
    if size == 0 as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut libc::c_uchar;
    }
    tmp = malloc(size.wrapping_add(7 as libc::c_ulong).wrapping_div(8 as libc::c_ulong));
    data = tmp as *mut libc::c_uchar;
    if data as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut libc::c_uchar;
    }
    bytes = size.wrapping_div(8 as libc::c_ulong);
    p = (*bstream).data;
    i = 0 as libc::c_int as size_t;
    while i < bytes {
        v = 0 as libc::c_int as libc::c_uchar;
        j = 0 as libc::c_int as size_t;
        while j < 8 as libc::c_ulong {
            v = ((v as libc::c_int) << 1 as libc::c_int) as libc::c_uchar;
            v = (v as libc::c_int | *p as libc::c_int) as libc::c_uchar;
            p = p.offset(1);
            j = j.wrapping_add(1);
        }
        *data.offset(i as isize) = v;
        i = i.wrapping_add(1);
    }
    oddbits = size & 7 as libc::c_ulong;
    if oddbits > 0 as libc::c_ulong {
        v = 0 as libc::c_int as libc::c_uchar;
        j = 0 as libc::c_int as size_t;
        while j < oddbits {
            v = ((v as libc::c_int) << 1 as libc::c_int) as libc::c_uchar;
            v = (v as libc::c_int | *p as libc::c_int) as libc::c_uchar;
            p = p.offset(1);
            j = j.wrapping_add(1);
        }
        *data
            .offset(
                bytes as isize,
            ) = ((v as libc::c_int) << (8 as libc::c_ulong).wrapping_sub(oddbits))
            as libc::c_uchar;
    }
    return data;
}
pub unsafe extern "C" fn BitStream_free(mut bstream: *mut BitStream) {
    if bstream as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        free((*bstream).data as *mut libc::c_void);
        free(bstream as *mut libc::c_void);
    }
}
static mut qrspecCapacity: [QRspec_Capacity; 41] = [
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 0 as libc::c_int,
            words: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 21 as libc::c_int,
            words: 26 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [
                7 as libc::c_int,
                10 as libc::c_int,
                13 as libc::c_int,
                17 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 25 as libc::c_int,
            words: 44 as libc::c_int,
            remainder: 7 as libc::c_int,
            ec: [
                10 as libc::c_int,
                16 as libc::c_int,
                22 as libc::c_int,
                28 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 29 as libc::c_int,
            words: 70 as libc::c_int,
            remainder: 7 as libc::c_int,
            ec: [
                15 as libc::c_int,
                26 as libc::c_int,
                36 as libc::c_int,
                44 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 33 as libc::c_int,
            words: 100 as libc::c_int,
            remainder: 7 as libc::c_int,
            ec: [
                20 as libc::c_int,
                36 as libc::c_int,
                52 as libc::c_int,
                64 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 37 as libc::c_int,
            words: 134 as libc::c_int,
            remainder: 7 as libc::c_int,
            ec: [
                26 as libc::c_int,
                48 as libc::c_int,
                72 as libc::c_int,
                88 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 41 as libc::c_int,
            words: 172 as libc::c_int,
            remainder: 7 as libc::c_int,
            ec: [
                36 as libc::c_int,
                64 as libc::c_int,
                96 as libc::c_int,
                112 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 45 as libc::c_int,
            words: 196 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [
                40 as libc::c_int,
                72 as libc::c_int,
                108 as libc::c_int,
                130 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 49 as libc::c_int,
            words: 242 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [
                48 as libc::c_int,
                88 as libc::c_int,
                132 as libc::c_int,
                156 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 53 as libc::c_int,
            words: 292 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [
                60 as libc::c_int,
                110 as libc::c_int,
                160 as libc::c_int,
                192 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 57 as libc::c_int,
            words: 346 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [
                72 as libc::c_int,
                130 as libc::c_int,
                192 as libc::c_int,
                224 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 61 as libc::c_int,
            words: 404 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [
                80 as libc::c_int,
                150 as libc::c_int,
                224 as libc::c_int,
                264 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 65 as libc::c_int,
            words: 466 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [
                96 as libc::c_int,
                176 as libc::c_int,
                260 as libc::c_int,
                308 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 69 as libc::c_int,
            words: 532 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [
                104 as libc::c_int,
                198 as libc::c_int,
                288 as libc::c_int,
                352 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 73 as libc::c_int,
            words: 581 as libc::c_int,
            remainder: 3 as libc::c_int,
            ec: [
                120 as libc::c_int,
                216 as libc::c_int,
                320 as libc::c_int,
                384 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 77 as libc::c_int,
            words: 655 as libc::c_int,
            remainder: 3 as libc::c_int,
            ec: [
                132 as libc::c_int,
                240 as libc::c_int,
                360 as libc::c_int,
                432 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 81 as libc::c_int,
            words: 733 as libc::c_int,
            remainder: 3 as libc::c_int,
            ec: [
                144 as libc::c_int,
                280 as libc::c_int,
                408 as libc::c_int,
                480 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 85 as libc::c_int,
            words: 815 as libc::c_int,
            remainder: 3 as libc::c_int,
            ec: [
                168 as libc::c_int,
                308 as libc::c_int,
                448 as libc::c_int,
                532 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 89 as libc::c_int,
            words: 901 as libc::c_int,
            remainder: 3 as libc::c_int,
            ec: [
                180 as libc::c_int,
                338 as libc::c_int,
                504 as libc::c_int,
                588 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 93 as libc::c_int,
            words: 991 as libc::c_int,
            remainder: 3 as libc::c_int,
            ec: [
                196 as libc::c_int,
                364 as libc::c_int,
                546 as libc::c_int,
                650 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 97 as libc::c_int,
            words: 1085 as libc::c_int,
            remainder: 3 as libc::c_int,
            ec: [
                224 as libc::c_int,
                416 as libc::c_int,
                600 as libc::c_int,
                700 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 101 as libc::c_int,
            words: 1156 as libc::c_int,
            remainder: 4 as libc::c_int,
            ec: [
                224 as libc::c_int,
                442 as libc::c_int,
                644 as libc::c_int,
                750 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 105 as libc::c_int,
            words: 1258 as libc::c_int,
            remainder: 4 as libc::c_int,
            ec: [
                252 as libc::c_int,
                476 as libc::c_int,
                690 as libc::c_int,
                816 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 109 as libc::c_int,
            words: 1364 as libc::c_int,
            remainder: 4 as libc::c_int,
            ec: [
                270 as libc::c_int,
                504 as libc::c_int,
                750 as libc::c_int,
                900 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 113 as libc::c_int,
            words: 1474 as libc::c_int,
            remainder: 4 as libc::c_int,
            ec: [
                300 as libc::c_int,
                560 as libc::c_int,
                810 as libc::c_int,
                960 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 117 as libc::c_int,
            words: 1588 as libc::c_int,
            remainder: 4 as libc::c_int,
            ec: [
                312 as libc::c_int,
                588 as libc::c_int,
                870 as libc::c_int,
                1050 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 121 as libc::c_int,
            words: 1706 as libc::c_int,
            remainder: 4 as libc::c_int,
            ec: [
                336 as libc::c_int,
                644 as libc::c_int,
                952 as libc::c_int,
                1110 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 125 as libc::c_int,
            words: 1828 as libc::c_int,
            remainder: 4 as libc::c_int,
            ec: [
                360 as libc::c_int,
                700 as libc::c_int,
                1020 as libc::c_int,
                1200 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 129 as libc::c_int,
            words: 1921 as libc::c_int,
            remainder: 3 as libc::c_int,
            ec: [
                390 as libc::c_int,
                728 as libc::c_int,
                1050 as libc::c_int,
                1260 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 133 as libc::c_int,
            words: 2051 as libc::c_int,
            remainder: 3 as libc::c_int,
            ec: [
                420 as libc::c_int,
                784 as libc::c_int,
                1140 as libc::c_int,
                1350 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 137 as libc::c_int,
            words: 2185 as libc::c_int,
            remainder: 3 as libc::c_int,
            ec: [
                450 as libc::c_int,
                812 as libc::c_int,
                1200 as libc::c_int,
                1440 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 141 as libc::c_int,
            words: 2323 as libc::c_int,
            remainder: 3 as libc::c_int,
            ec: [
                480 as libc::c_int,
                868 as libc::c_int,
                1290 as libc::c_int,
                1530 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 145 as libc::c_int,
            words: 2465 as libc::c_int,
            remainder: 3 as libc::c_int,
            ec: [
                510 as libc::c_int,
                924 as libc::c_int,
                1350 as libc::c_int,
                1620 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 149 as libc::c_int,
            words: 2611 as libc::c_int,
            remainder: 3 as libc::c_int,
            ec: [
                540 as libc::c_int,
                980 as libc::c_int,
                1440 as libc::c_int,
                1710 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 153 as libc::c_int,
            words: 2761 as libc::c_int,
            remainder: 3 as libc::c_int,
            ec: [
                570 as libc::c_int,
                1036 as libc::c_int,
                1530 as libc::c_int,
                1800 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 157 as libc::c_int,
            words: 2876 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [
                570 as libc::c_int,
                1064 as libc::c_int,
                1590 as libc::c_int,
                1890 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 161 as libc::c_int,
            words: 3034 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [
                600 as libc::c_int,
                1120 as libc::c_int,
                1680 as libc::c_int,
                1980 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 165 as libc::c_int,
            words: 3196 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [
                630 as libc::c_int,
                1204 as libc::c_int,
                1770 as libc::c_int,
                2100 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 169 as libc::c_int,
            words: 3362 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [
                660 as libc::c_int,
                1260 as libc::c_int,
                1860 as libc::c_int,
                2220 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 173 as libc::c_int,
            words: 3532 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [
                720 as libc::c_int,
                1316 as libc::c_int,
                1950 as libc::c_int,
                2310 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_QRspec_Capacity_897426155 {
            width: 177 as libc::c_int,
            words: 3706 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [
                750 as libc::c_int,
                1372 as libc::c_int,
                2040 as libc::c_int,
                2430 as libc::c_int,
            ],
        };
        init
    },
];
pub unsafe extern "C" fn QRspec_getDataLength(
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> libc::c_int {
    return qrspecCapacity[version as usize].words
        - qrspecCapacity[version as usize].ec[level as usize];
}
pub unsafe extern "C" fn QRspec_getECCLength(
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> libc::c_int {
    return qrspecCapacity[version as usize].ec[level as usize];
}
pub unsafe extern "C" fn QRspec_getMinimumVersion(
    mut size: libc::c_int,
    mut level: QRecLevel,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut words: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= 40 as libc::c_int {
        words = qrspecCapacity[i as usize].words
            - qrspecCapacity[i as usize].ec[level as usize];
        if words >= size {
            return i;
        }
        i += 1;
    }
    return 40 as libc::c_int;
}
pub unsafe extern "C" fn QRspec_getWidth(mut version: libc::c_int) -> libc::c_int {
    return qrspecCapacity[version as usize].width;
}
pub unsafe extern "C" fn QRspec_getRemainder(mut version: libc::c_int) -> libc::c_int {
    return qrspecCapacity[version as usize].remainder;
}
static mut lengthTableBits: [[libc::c_int; 3]; 4] = [
    [10 as libc::c_int, 12 as libc::c_int, 14 as libc::c_int],
    [9 as libc::c_int, 11 as libc::c_int, 13 as libc::c_int],
    [8 as libc::c_int, 16 as libc::c_int, 16 as libc::c_int],
    [8 as libc::c_int, 10 as libc::c_int, 12 as libc::c_int],
];
pub unsafe extern "C" fn QRspec_lengthIndicator(
    mut mode: QRencodeMode,
    mut version: libc::c_int,
) -> libc::c_int {
    let mut l: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tmp = QRinput_isSplittableMode(mode);
    if tmp == 0 {
        return 0 as libc::c_int;
    }
    if version <= 9 as libc::c_int {
        l = 0 as libc::c_int;
    } else if version <= 26 as libc::c_int {
        l = 1 as libc::c_int;
    } else {
        l = 2 as libc::c_int;
    }
    return lengthTableBits[mode as usize][l as usize];
}
pub unsafe extern "C" fn QRspec_maximumWords(
    mut mode: QRencodeMode,
    mut version: libc::c_int,
) -> libc::c_int {
    let mut l: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    let mut words: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tmp = QRinput_isSplittableMode(mode);
    if tmp == 0 {
        return 0 as libc::c_int;
    }
    if version <= 9 as libc::c_int {
        l = 0 as libc::c_int;
    } else if version <= 26 as libc::c_int {
        l = 1 as libc::c_int;
    } else {
        l = 2 as libc::c_int;
    }
    bits = lengthTableBits[mode as usize][l as usize];
    words = ((1 as libc::c_int) << bits) - 1 as libc::c_int;
    if mode as libc::c_int == 3 as libc::c_int {
        words *= 2 as libc::c_int;
    }
    return words;
}
static mut eccTable: [[[libc::c_int; 2]; 4]; 41] = [
    [
        [0 as libc::c_int, 0 as libc::c_int],
        [0 as libc::c_int, 0 as libc::c_int],
        [0 as libc::c_int, 0 as libc::c_int],
        [0 as libc::c_int, 0 as libc::c_int],
    ],
    [
        [1 as libc::c_int, 0 as libc::c_int],
        [1 as libc::c_int, 0 as libc::c_int],
        [1 as libc::c_int, 0 as libc::c_int],
        [1 as libc::c_int, 0 as libc::c_int],
    ],
    [
        [1 as libc::c_int, 0 as libc::c_int],
        [1 as libc::c_int, 0 as libc::c_int],
        [1 as libc::c_int, 0 as libc::c_int],
        [1 as libc::c_int, 0 as libc::c_int],
    ],
    [
        [1 as libc::c_int, 0 as libc::c_int],
        [1 as libc::c_int, 0 as libc::c_int],
        [2 as libc::c_int, 0 as libc::c_int],
        [2 as libc::c_int, 0 as libc::c_int],
    ],
    [
        [1 as libc::c_int, 0 as libc::c_int],
        [2 as libc::c_int, 0 as libc::c_int],
        [2 as libc::c_int, 0 as libc::c_int],
        [4 as libc::c_int, 0 as libc::c_int],
    ],
    [
        [1 as libc::c_int, 0 as libc::c_int],
        [2 as libc::c_int, 0 as libc::c_int],
        [2 as libc::c_int, 2 as libc::c_int],
        [2 as libc::c_int, 2 as libc::c_int],
    ],
    [
        [2 as libc::c_int, 0 as libc::c_int],
        [4 as libc::c_int, 0 as libc::c_int],
        [4 as libc::c_int, 0 as libc::c_int],
        [4 as libc::c_int, 0 as libc::c_int],
    ],
    [
        [2 as libc::c_int, 0 as libc::c_int],
        [4 as libc::c_int, 0 as libc::c_int],
        [2 as libc::c_int, 4 as libc::c_int],
        [4 as libc::c_int, 1 as libc::c_int],
    ],
    [
        [2 as libc::c_int, 0 as libc::c_int],
        [2 as libc::c_int, 2 as libc::c_int],
        [4 as libc::c_int, 2 as libc::c_int],
        [4 as libc::c_int, 2 as libc::c_int],
    ],
    [
        [2 as libc::c_int, 0 as libc::c_int],
        [3 as libc::c_int, 2 as libc::c_int],
        [4 as libc::c_int, 4 as libc::c_int],
        [4 as libc::c_int, 4 as libc::c_int],
    ],
    [
        [2 as libc::c_int, 2 as libc::c_int],
        [4 as libc::c_int, 1 as libc::c_int],
        [6 as libc::c_int, 2 as libc::c_int],
        [6 as libc::c_int, 2 as libc::c_int],
    ],
    [
        [4 as libc::c_int, 0 as libc::c_int],
        [1 as libc::c_int, 4 as libc::c_int],
        [4 as libc::c_int, 4 as libc::c_int],
        [3 as libc::c_int, 8 as libc::c_int],
    ],
    [
        [2 as libc::c_int, 2 as libc::c_int],
        [6 as libc::c_int, 2 as libc::c_int],
        [4 as libc::c_int, 6 as libc::c_int],
        [7 as libc::c_int, 4 as libc::c_int],
    ],
    [
        [4 as libc::c_int, 0 as libc::c_int],
        [8 as libc::c_int, 1 as libc::c_int],
        [8 as libc::c_int, 4 as libc::c_int],
        [12 as libc::c_int, 4 as libc::c_int],
    ],
    [
        [3 as libc::c_int, 1 as libc::c_int],
        [4 as libc::c_int, 5 as libc::c_int],
        [11 as libc::c_int, 5 as libc::c_int],
        [11 as libc::c_int, 5 as libc::c_int],
    ],
    [
        [5 as libc::c_int, 1 as libc::c_int],
        [5 as libc::c_int, 5 as libc::c_int],
        [5 as libc::c_int, 7 as libc::c_int],
        [11 as libc::c_int, 7 as libc::c_int],
    ],
    [
        [5 as libc::c_int, 1 as libc::c_int],
        [7 as libc::c_int, 3 as libc::c_int],
        [15 as libc::c_int, 2 as libc::c_int],
        [3 as libc::c_int, 13 as libc::c_int],
    ],
    [
        [1 as libc::c_int, 5 as libc::c_int],
        [10 as libc::c_int, 1 as libc::c_int],
        [1 as libc::c_int, 15 as libc::c_int],
        [2 as libc::c_int, 17 as libc::c_int],
    ],
    [
        [5 as libc::c_int, 1 as libc::c_int],
        [9 as libc::c_int, 4 as libc::c_int],
        [17 as libc::c_int, 1 as libc::c_int],
        [2 as libc::c_int, 19 as libc::c_int],
    ],
    [
        [3 as libc::c_int, 4 as libc::c_int],
        [3 as libc::c_int, 11 as libc::c_int],
        [17 as libc::c_int, 4 as libc::c_int],
        [9 as libc::c_int, 16 as libc::c_int],
    ],
    [
        [3 as libc::c_int, 5 as libc::c_int],
        [3 as libc::c_int, 13 as libc::c_int],
        [15 as libc::c_int, 5 as libc::c_int],
        [15 as libc::c_int, 10 as libc::c_int],
    ],
    [
        [4 as libc::c_int, 4 as libc::c_int],
        [17 as libc::c_int, 0 as libc::c_int],
        [17 as libc::c_int, 6 as libc::c_int],
        [19 as libc::c_int, 6 as libc::c_int],
    ],
    [
        [2 as libc::c_int, 7 as libc::c_int],
        [17 as libc::c_int, 0 as libc::c_int],
        [7 as libc::c_int, 16 as libc::c_int],
        [34 as libc::c_int, 0 as libc::c_int],
    ],
    [
        [4 as libc::c_int, 5 as libc::c_int],
        [4 as libc::c_int, 14 as libc::c_int],
        [11 as libc::c_int, 14 as libc::c_int],
        [16 as libc::c_int, 14 as libc::c_int],
    ],
    [
        [6 as libc::c_int, 4 as libc::c_int],
        [6 as libc::c_int, 14 as libc::c_int],
        [11 as libc::c_int, 16 as libc::c_int],
        [30 as libc::c_int, 2 as libc::c_int],
    ],
    [
        [8 as libc::c_int, 4 as libc::c_int],
        [8 as libc::c_int, 13 as libc::c_int],
        [7 as libc::c_int, 22 as libc::c_int],
        [22 as libc::c_int, 13 as libc::c_int],
    ],
    [
        [10 as libc::c_int, 2 as libc::c_int],
        [19 as libc::c_int, 4 as libc::c_int],
        [28 as libc::c_int, 6 as libc::c_int],
        [33 as libc::c_int, 4 as libc::c_int],
    ],
    [
        [8 as libc::c_int, 4 as libc::c_int],
        [22 as libc::c_int, 3 as libc::c_int],
        [8 as libc::c_int, 26 as libc::c_int],
        [12 as libc::c_int, 28 as libc::c_int],
    ],
    [
        [3 as libc::c_int, 10 as libc::c_int],
        [3 as libc::c_int, 23 as libc::c_int],
        [4 as libc::c_int, 31 as libc::c_int],
        [11 as libc::c_int, 31 as libc::c_int],
    ],
    [
        [7 as libc::c_int, 7 as libc::c_int],
        [21 as libc::c_int, 7 as libc::c_int],
        [1 as libc::c_int, 37 as libc::c_int],
        [19 as libc::c_int, 26 as libc::c_int],
    ],
    [
        [5 as libc::c_int, 10 as libc::c_int],
        [19 as libc::c_int, 10 as libc::c_int],
        [15 as libc::c_int, 25 as libc::c_int],
        [23 as libc::c_int, 25 as libc::c_int],
    ],
    [
        [13 as libc::c_int, 3 as libc::c_int],
        [2 as libc::c_int, 29 as libc::c_int],
        [42 as libc::c_int, 1 as libc::c_int],
        [23 as libc::c_int, 28 as libc::c_int],
    ],
    [
        [17 as libc::c_int, 0 as libc::c_int],
        [10 as libc::c_int, 23 as libc::c_int],
        [10 as libc::c_int, 35 as libc::c_int],
        [19 as libc::c_int, 35 as libc::c_int],
    ],
    [
        [17 as libc::c_int, 1 as libc::c_int],
        [14 as libc::c_int, 21 as libc::c_int],
        [29 as libc::c_int, 19 as libc::c_int],
        [11 as libc::c_int, 46 as libc::c_int],
    ],
    [
        [13 as libc::c_int, 6 as libc::c_int],
        [14 as libc::c_int, 23 as libc::c_int],
        [44 as libc::c_int, 7 as libc::c_int],
        [59 as libc::c_int, 1 as libc::c_int],
    ],
    [
        [12 as libc::c_int, 7 as libc::c_int],
        [12 as libc::c_int, 26 as libc::c_int],
        [39 as libc::c_int, 14 as libc::c_int],
        [22 as libc::c_int, 41 as libc::c_int],
    ],
    [
        [6 as libc::c_int, 14 as libc::c_int],
        [6 as libc::c_int, 34 as libc::c_int],
        [46 as libc::c_int, 10 as libc::c_int],
        [2 as libc::c_int, 64 as libc::c_int],
    ],
    [
        [17 as libc::c_int, 4 as libc::c_int],
        [29 as libc::c_int, 14 as libc::c_int],
        [49 as libc::c_int, 10 as libc::c_int],
        [24 as libc::c_int, 46 as libc::c_int],
    ],
    [
        [4 as libc::c_int, 18 as libc::c_int],
        [13 as libc::c_int, 32 as libc::c_int],
        [48 as libc::c_int, 14 as libc::c_int],
        [42 as libc::c_int, 32 as libc::c_int],
    ],
    [
        [20 as libc::c_int, 4 as libc::c_int],
        [40 as libc::c_int, 7 as libc::c_int],
        [43 as libc::c_int, 22 as libc::c_int],
        [10 as libc::c_int, 67 as libc::c_int],
    ],
    [
        [19 as libc::c_int, 6 as libc::c_int],
        [18 as libc::c_int, 31 as libc::c_int],
        [34 as libc::c_int, 34 as libc::c_int],
        [20 as libc::c_int, 61 as libc::c_int],
    ],
];
pub unsafe extern "C" fn QRspec_getEccSpec(
    mut version: libc::c_int,
    mut level: QRecLevel,
    mut spec: *mut libc::c_int,
) {
    let mut b1: libc::c_int = 0;
    let mut b2: libc::c_int = 0;
    let mut data: libc::c_int = 0;
    let mut ecc: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    b1 = eccTable[version as usize][level as usize][0 as libc::c_int as usize];
    b2 = eccTable[version as usize][level as usize][1 as libc::c_int as usize];
    data = QRspec_getDataLength(version, level);
    ecc = QRspec_getECCLength(version, level);
    if b2 == 0 as libc::c_int {
        *spec.offset(0 as libc::c_int as isize) = b1;
        *spec.offset(1 as libc::c_int as isize) = data / b1;
        *spec.offset(2 as libc::c_int as isize) = ecc / b1;
        tmp = 0 as libc::c_int;
        *spec.offset(4 as libc::c_int as isize) = tmp;
        *spec.offset(3 as libc::c_int as isize) = tmp;
    } else {
        *spec.offset(0 as libc::c_int as isize) = b1;
        *spec.offset(1 as libc::c_int as isize) = data / (b1 + b2);
        *spec.offset(2 as libc::c_int as isize) = ecc / (b1 + b2);
        *spec.offset(3 as libc::c_int as isize) = b2;
        *spec
            .offset(
                4 as libc::c_int as isize,
            ) = *spec.offset(1 as libc::c_int as isize) + 1 as libc::c_int;
    };
}
static mut alignmentPattern: [[libc::c_int; 2]; 41] = [
    [0 as libc::c_int, 0 as libc::c_int],
    [0 as libc::c_int, 0 as libc::c_int],
    [18 as libc::c_int, 0 as libc::c_int],
    [22 as libc::c_int, 0 as libc::c_int],
    [26 as libc::c_int, 0 as libc::c_int],
    [30 as libc::c_int, 0 as libc::c_int],
    [34 as libc::c_int, 0 as libc::c_int],
    [22 as libc::c_int, 38 as libc::c_int],
    [24 as libc::c_int, 42 as libc::c_int],
    [26 as libc::c_int, 46 as libc::c_int],
    [28 as libc::c_int, 50 as libc::c_int],
    [30 as libc::c_int, 54 as libc::c_int],
    [32 as libc::c_int, 58 as libc::c_int],
    [34 as libc::c_int, 62 as libc::c_int],
    [26 as libc::c_int, 46 as libc::c_int],
    [26 as libc::c_int, 48 as libc::c_int],
    [26 as libc::c_int, 50 as libc::c_int],
    [30 as libc::c_int, 54 as libc::c_int],
    [30 as libc::c_int, 56 as libc::c_int],
    [30 as libc::c_int, 58 as libc::c_int],
    [34 as libc::c_int, 62 as libc::c_int],
    [28 as libc::c_int, 50 as libc::c_int],
    [26 as libc::c_int, 50 as libc::c_int],
    [30 as libc::c_int, 54 as libc::c_int],
    [28 as libc::c_int, 54 as libc::c_int],
    [32 as libc::c_int, 58 as libc::c_int],
    [30 as libc::c_int, 58 as libc::c_int],
    [34 as libc::c_int, 62 as libc::c_int],
    [26 as libc::c_int, 50 as libc::c_int],
    [30 as libc::c_int, 54 as libc::c_int],
    [26 as libc::c_int, 52 as libc::c_int],
    [30 as libc::c_int, 56 as libc::c_int],
    [34 as libc::c_int, 60 as libc::c_int],
    [30 as libc::c_int, 58 as libc::c_int],
    [34 as libc::c_int, 62 as libc::c_int],
    [30 as libc::c_int, 54 as libc::c_int],
    [24 as libc::c_int, 50 as libc::c_int],
    [28 as libc::c_int, 54 as libc::c_int],
    [32 as libc::c_int, 58 as libc::c_int],
    [26 as libc::c_int, 54 as libc::c_int],
    [30 as libc::c_int, 58 as libc::c_int],
];
static mut finder: [libc::c_uchar; 25] = [
    161 as libc::c_int as libc::c_uchar,
    161 as libc::c_int as libc::c_uchar,
    161 as libc::c_int as libc::c_uchar,
    161 as libc::c_int as libc::c_uchar,
    161 as libc::c_int as libc::c_uchar,
    161 as libc::c_int as libc::c_uchar,
    160 as libc::c_int as libc::c_uchar,
    160 as libc::c_int as libc::c_uchar,
    160 as libc::c_int as libc::c_uchar,
    161 as libc::c_int as libc::c_uchar,
    161 as libc::c_int as libc::c_uchar,
    160 as libc::c_int as libc::c_uchar,
    161 as libc::c_int as libc::c_uchar,
    160 as libc::c_int as libc::c_uchar,
    161 as libc::c_int as libc::c_uchar,
    161 as libc::c_int as libc::c_uchar,
    160 as libc::c_int as libc::c_uchar,
    160 as libc::c_int as libc::c_uchar,
    160 as libc::c_int as libc::c_uchar,
    161 as libc::c_int as libc::c_uchar,
    161 as libc::c_int as libc::c_uchar,
    161 as libc::c_int as libc::c_uchar,
    161 as libc::c_int as libc::c_uchar,
    161 as libc::c_int as libc::c_uchar,
    161 as libc::c_int as libc::c_uchar,
];
unsafe extern "C" fn QRspec_putAlignmentMarker(
    mut frame: *mut libc::c_uchar,
    mut width: libc::c_int,
    mut ox: libc::c_int,
    mut oy: libc::c_int,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut s: *const libc::c_uchar = 0 as *const libc::c_uchar;
    frame = frame
        .offset(((oy - 2 as libc::c_int) * width + ox - 2 as libc::c_int) as isize);
    s = finder.as_ptr();
    y = 0 as libc::c_int;
    while y < 5 as libc::c_int {
        x = 0 as libc::c_int;
        while x < 5 as libc::c_int {
            *frame.offset(x as isize) = *s.offset(x as isize);
            x += 1;
        }
        frame = frame.offset(width as isize);
        s = s.offset(5 as libc::c_int as isize);
        y += 1;
    }
}
unsafe extern "C" fn QRspec_putAlignmentPattern(
    mut version: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut width: libc::c_int,
) {
    let mut d: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut cx: libc::c_int = 0;
    let mut cy: libc::c_int = 0;
    if version < 2 as libc::c_int {
        return;
    }
    d = alignmentPattern[version as usize][1 as libc::c_int as usize]
        - alignmentPattern[version as usize][0 as libc::c_int as usize];
    if d < 0 as libc::c_int {
        w = 2 as libc::c_int;
    } else {
        w = (width - alignmentPattern[version as usize][0 as libc::c_int as usize]) / d
            + 2 as libc::c_int;
    }
    if w * w - 3 as libc::c_int == 1 as libc::c_int {
        x = alignmentPattern[version as usize][0 as libc::c_int as usize];
        y = alignmentPattern[version as usize][0 as libc::c_int as usize];
        QRspec_putAlignmentMarker(frame, width, x, y);
        return;
    }
    cx = alignmentPattern[version as usize][0 as libc::c_int as usize];
    x = 1 as libc::c_int;
    while x < w - 1 as libc::c_int {
        QRspec_putAlignmentMarker(frame, width, 6 as libc::c_int, cx);
        QRspec_putAlignmentMarker(frame, width, cx, 6 as libc::c_int);
        cx += d;
        x += 1;
    }
    cy = alignmentPattern[version as usize][0 as libc::c_int as usize];
    y = 0 as libc::c_int;
    while y < w - 1 as libc::c_int {
        cx = alignmentPattern[version as usize][0 as libc::c_int as usize];
        x = 0 as libc::c_int;
        while x < w - 1 as libc::c_int {
            QRspec_putAlignmentMarker(frame, width, cx, cy);
            cx += d;
            x += 1;
        }
        cy += d;
        y += 1;
    }
}
static mut versionPattern: [libc::c_uint; 34] = [
    31892 as libc::c_int as libc::c_uint,
    34236 as libc::c_int as libc::c_uint,
    39577 as libc::c_int as libc::c_uint,
    42195 as libc::c_int as libc::c_uint,
    48118 as libc::c_int as libc::c_uint,
    51042 as libc::c_int as libc::c_uint,
    55367 as libc::c_int as libc::c_uint,
    58893 as libc::c_int as libc::c_uint,
    63784 as libc::c_int as libc::c_uint,
    68472 as libc::c_int as libc::c_uint,
    70749 as libc::c_int as libc::c_uint,
    76311 as libc::c_int as libc::c_uint,
    79154 as libc::c_int as libc::c_uint,
    84390 as libc::c_int as libc::c_uint,
    87683 as libc::c_int as libc::c_uint,
    92361 as libc::c_int as libc::c_uint,
    96236 as libc::c_int as libc::c_uint,
    102084 as libc::c_int as libc::c_uint,
    102881 as libc::c_int as libc::c_uint,
    110507 as libc::c_int as libc::c_uint,
    110734 as libc::c_int as libc::c_uint,
    117786 as libc::c_int as libc::c_uint,
    119615 as libc::c_int as libc::c_uint,
    126325 as libc::c_int as libc::c_uint,
    127568 as libc::c_int as libc::c_uint,
    133589 as libc::c_int as libc::c_uint,
    136944 as libc::c_int as libc::c_uint,
    141498 as libc::c_int as libc::c_uint,
    145311 as libc::c_int as libc::c_uint,
    150283 as libc::c_int as libc::c_uint,
    152622 as libc::c_int as libc::c_uint,
    158308 as libc::c_int as libc::c_uint,
    161089 as libc::c_int as libc::c_uint,
    167017 as libc::c_int as libc::c_uint,
];
pub unsafe extern "C" fn QRspec_getVersionPattern(
    mut version: libc::c_int,
) -> libc::c_uint {
    if version < 7 as libc::c_int {
        return 0 as libc::c_uint
    } else {
        if version > 40 as libc::c_int {
            return 0 as libc::c_uint;
        }
    }
    return versionPattern[(version - 7 as libc::c_int) as usize];
}
static mut formatInfo: [[libc::c_uint; 8]; 4] = [
    [
        30660 as libc::c_int as libc::c_uint,
        29427 as libc::c_int as libc::c_uint,
        32170 as libc::c_int as libc::c_uint,
        30877 as libc::c_int as libc::c_uint,
        26159 as libc::c_int as libc::c_uint,
        25368 as libc::c_int as libc::c_uint,
        27713 as libc::c_int as libc::c_uint,
        26998 as libc::c_int as libc::c_uint,
    ],
    [
        21522 as libc::c_int as libc::c_uint,
        20773 as libc::c_int as libc::c_uint,
        24188 as libc::c_int as libc::c_uint,
        23371 as libc::c_int as libc::c_uint,
        17913 as libc::c_int as libc::c_uint,
        16590 as libc::c_int as libc::c_uint,
        20375 as libc::c_int as libc::c_uint,
        19104 as libc::c_int as libc::c_uint,
    ],
    [
        13663 as libc::c_int as libc::c_uint,
        12392 as libc::c_int as libc::c_uint,
        16177 as libc::c_int as libc::c_uint,
        14854 as libc::c_int as libc::c_uint,
        9396 as libc::c_int as libc::c_uint,
        8579 as libc::c_int as libc::c_uint,
        11994 as libc::c_int as libc::c_uint,
        11245 as libc::c_int as libc::c_uint,
    ],
    [
        5769 as libc::c_int as libc::c_uint,
        5054 as libc::c_int as libc::c_uint,
        7399 as libc::c_int as libc::c_uint,
        6608 as libc::c_int as libc::c_uint,
        1890 as libc::c_int as libc::c_uint,
        597 as libc::c_int as libc::c_uint,
        3340 as libc::c_int as libc::c_uint,
        2107 as libc::c_int as libc::c_uint,
    ],
];
pub unsafe extern "C" fn QRspec_getFormatInfo(
    mut mask: libc::c_int,
    mut level: QRecLevel,
) -> libc::c_uint {
    if mask < 0 as libc::c_int {
        return 0 as libc::c_uint
    } else {
        if mask > 7 as libc::c_int {
            return 0 as libc::c_uint;
        }
    }
    return formatInfo[level as usize][mask as usize];
}
static mut finder___0: [libc::c_uchar; 49] = [
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
];
unsafe extern "C" fn putFinderPattern(
    mut frame: *mut libc::c_uchar,
    mut width: libc::c_int,
    mut ox: libc::c_int,
    mut oy: libc::c_int,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut s: *const libc::c_uchar = 0 as *const libc::c_uchar;
    frame = frame.offset((oy * width + ox) as isize);
    s = finder___0.as_ptr();
    y = 0 as libc::c_int;
    while y < 7 as libc::c_int {
        x = 0 as libc::c_int;
        while x < 7 as libc::c_int {
            *frame.offset(x as isize) = *s.offset(x as isize);
            x += 1;
        }
        frame = frame.offset(width as isize);
        s = s.offset(7 as libc::c_int as isize);
        y += 1;
    }
}
unsafe extern "C" fn QRspec_createFrame(mut version: libc::c_int) -> *mut libc::c_uchar {
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut q: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut width: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut verinfo: libc::c_uint = 0;
    let mut v: libc::c_uint = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    width = qrspecCapacity[version as usize].width;
    tmp = malloc((width * width) as size_t);
    frame = tmp as *mut libc::c_uchar;
    if frame as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut libc::c_uchar;
    }
    memset(frame as *mut libc::c_void, 0 as libc::c_int, (width * width) as size_t);
    putFinderPattern(frame, width, 0 as libc::c_int, 0 as libc::c_int);
    putFinderPattern(frame, width, width - 7 as libc::c_int, 0 as libc::c_int);
    putFinderPattern(frame, width, 0 as libc::c_int, width - 7 as libc::c_int);
    p = frame;
    q = frame.offset((width * (width - 7 as libc::c_int)) as isize);
    y = 0 as libc::c_int;
    while y < 7 as libc::c_int {
        *p.offset(7 as libc::c_int as isize) = 192 as libc::c_int as libc::c_uchar;
        *p
            .offset(
                (width - 8 as libc::c_int) as isize,
            ) = 192 as libc::c_int as libc::c_uchar;
        *q.offset(7 as libc::c_int as isize) = 192 as libc::c_int as libc::c_uchar;
        p = p.offset(width as isize);
        q = q.offset(width as isize);
        y += 1;
    }
    memset(
        frame.offset((width * 7 as libc::c_int) as isize) as *mut libc::c_void,
        192 as libc::c_int,
        8 as libc::c_int as size_t,
    );
    memset(
        frame
            .offset((width * 8 as libc::c_int) as isize)
            .offset(-(8 as libc::c_int as isize)) as *mut libc::c_void,
        192 as libc::c_int,
        8 as libc::c_int as size_t,
    );
    memset(
        frame.offset((width * (width - 8 as libc::c_int)) as isize) as *mut libc::c_void,
        192 as libc::c_int,
        8 as libc::c_int as size_t,
    );
    memset(
        frame.offset((width * 8 as libc::c_int) as isize) as *mut libc::c_void,
        132 as libc::c_int,
        9 as libc::c_int as size_t,
    );
    memset(
        frame
            .offset((width * 9 as libc::c_int) as isize)
            .offset(-(8 as libc::c_int as isize)) as *mut libc::c_void,
        132 as libc::c_int,
        8 as libc::c_int as size_t,
    );
    p = frame.offset(8 as libc::c_int as isize);
    y = 0 as libc::c_int;
    while y < 8 as libc::c_int {
        *p = 132 as libc::c_int as libc::c_uchar;
        p = p.offset(width as isize);
        y += 1;
    }
    p = frame
        .offset((width * (width - 7 as libc::c_int)) as isize)
        .offset(8 as libc::c_int as isize);
    y = 0 as libc::c_int;
    while y < 7 as libc::c_int {
        *p = 132 as libc::c_int as libc::c_uchar;
        p = p.offset(width as isize);
        y += 1;
    }
    p = frame
        .offset((width * 6 as libc::c_int) as isize)
        .offset(8 as libc::c_int as isize);
    q = frame
        .offset((width * 8 as libc::c_int) as isize)
        .offset(6 as libc::c_int as isize);
    x = 1 as libc::c_int;
    while x < width - 15 as libc::c_int {
        *p = (144 as libc::c_int | x & 1 as libc::c_int) as libc::c_uchar;
        *q = (144 as libc::c_int | x & 1 as libc::c_int) as libc::c_uchar;
        p = p.offset(1);
        q = q.offset(width as isize);
        x += 1;
    }
    QRspec_putAlignmentPattern(version, frame, width);
    if version >= 7 as libc::c_int {
        verinfo = QRspec_getVersionPattern(version);
        p = frame.offset((width * (width - 11 as libc::c_int)) as isize);
        v = verinfo;
        x = 0 as libc::c_int;
        while x < 6 as libc::c_int {
            y = 0 as libc::c_int;
            while y < 3 as libc::c_int {
                *p
                    .offset(
                        (width * y + x) as isize,
                    ) = (136 as libc::c_uint | v & 1 as libc::c_uint) as libc::c_uchar;
                v >>= 1 as libc::c_int;
                y += 1;
            }
            x += 1;
        }
        p = frame.offset(width as isize).offset(-(11 as libc::c_int as isize));
        v = verinfo;
        y = 0 as libc::c_int;
        while y < 6 as libc::c_int {
            x = 0 as libc::c_int;
            while x < 3 as libc::c_int {
                *p
                    .offset(
                        x as isize,
                    ) = (136 as libc::c_uint | v & 1 as libc::c_uint) as libc::c_uchar;
                v >>= 1 as libc::c_int;
                x += 1;
            }
            p = p.offset(width as isize);
            y += 1;
        }
    }
    *frame
        .offset(
            (width * (width - 8 as libc::c_int) + 8 as libc::c_int) as isize,
        ) = 129 as libc::c_int as libc::c_uchar;
    return frame;
}
pub unsafe extern "C" fn QRspec_newFrame(
    mut version: libc::c_int,
) -> *mut libc::c_uchar {
    let mut tmp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if version < 1 as libc::c_int {
        return 0 as *mut libc::c_void as *mut libc::c_uchar
    } else {
        if version > 40 as libc::c_int {
            return 0 as *mut libc::c_void as *mut libc::c_uchar;
        }
    }
    tmp = QRspec_createFrame(version);
    return tmp;
}
static mut RSECC_mutex: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_uint,
            __kind: 0 as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static mut initialized: libc::c_int = 0 as libc::c_int;
static mut proot: libc::c_uint = 285 as libc::c_int as libc::c_uint;
static mut alpha: [libc::c_uchar; 256] = [0; 256];
static mut aindex: [libc::c_uchar; 256] = [0; 256];
static mut generator: [[libc::c_uchar; 31]; 29] = [[0; 31]; 29];
static mut generatorInitialized: [libc::c_uchar; 29] = [0; 29];
unsafe extern "C" fn RSECC_initLookupTable() {
    let mut i: libc::c_uint = 0;
    let mut b: libc::c_uint = 0;
    alpha[((1 as libc::c_uint) << 8 as libc::c_int).wrapping_sub(1 as libc::c_uint)
        as usize] = 0 as libc::c_int as libc::c_uchar;
    aindex[0 as libc::c_int
        as usize] = ((1 as libc::c_uint) << 8 as libc::c_int)
        .wrapping_sub(1 as libc::c_uint) as libc::c_uchar;
    b = 1 as libc::c_uint;
    i = 0 as libc::c_uint;
    while i < ((1 as libc::c_uint) << 8 as libc::c_int).wrapping_sub(1 as libc::c_uint) {
        alpha[i as usize] = b as libc::c_uchar;
        aindex[b as usize] = i as libc::c_uchar;
        b <<= 1 as libc::c_int;
        if b
            & ((1 as libc::c_uint) << 8 as libc::c_int)
                .wrapping_sub(1 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint) != 0
        {
            b ^= proot;
        }
        b &= ((1 as libc::c_uint) << 8 as libc::c_int).wrapping_sub(1 as libc::c_uint);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn RSECC_init() {
    RSECC_initLookupTable();
    memset(
        generatorInitialized.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        29 as libc::c_int as size_t,
    );
    initialized = 1 as libc::c_int;
}
unsafe extern "C" fn generator_init(mut length: size_t) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut g: [libc::c_int; 31] = [0; 31];
    g[0 as libc::c_int as usize] = 1 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < length {
        g[i.wrapping_add(1 as libc::c_ulong) as usize] = 1 as libc::c_int;
        j = i;
        while j > 0 as libc::c_ulong {
            g[j
                as usize] = g[j.wrapping_sub(1 as libc::c_ulong) as usize]
                ^ alpha[(aindex[g[j as usize] as usize] as size_t)
                    .wrapping_add(i)
                    .wrapping_rem(
                        ((1 as libc::c_uint) << 8 as libc::c_int)
                            .wrapping_sub(1 as libc::c_uint) as libc::c_ulong,
                    ) as usize] as libc::c_int;
            j = j.wrapping_sub(1);
        }
        g[0 as libc::c_int
            as usize] = alpha[(aindex[g[0 as libc::c_int as usize] as usize] as size_t)
            .wrapping_add(i)
            .wrapping_rem(
                ((1 as libc::c_uint) << 8 as libc::c_int).wrapping_sub(1 as libc::c_uint)
                    as libc::c_ulong,
            ) as usize] as libc::c_int;
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as size_t;
    while i <= length {
        generator[length.wrapping_sub(2 as libc::c_ulong)
            as usize][i as usize] = aindex[g[i as usize] as usize];
        i = i.wrapping_add(1);
    }
    generatorInitialized[length.wrapping_sub(2 as libc::c_ulong)
        as usize] = 1 as libc::c_int as libc::c_uchar;
}
pub unsafe extern "C" fn RSECC_encode(
    mut data_length: size_t,
    mut ecc_length: size_t,
    mut data: *const libc::c_uchar,
    mut ecc: *mut libc::c_uchar,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut feedback: libc::c_uchar = 0;
    let mut gen: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    pthread_mutex_lock(&mut RSECC_mutex);
    if initialized == 0 {
        RSECC_init();
    }
    pthread_mutex_unlock(&mut RSECC_mutex);
    if ecc_length > 30 as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    memset(ecc as *mut libc::c_void, 0 as libc::c_int, ecc_length);
    pthread_mutex_lock(&mut RSECC_mutex);
    if generatorInitialized[ecc_length.wrapping_sub(2 as libc::c_ulong) as usize] == 0 {
        generator_init(ecc_length);
    }
    pthread_mutex_unlock(&mut RSECC_mutex);
    gen = (generator[ecc_length.wrapping_sub(2 as libc::c_ulong) as usize]).as_mut_ptr();
    i = 0 as libc::c_int as size_t;
    while i < data_length {
        feedback = aindex[(*data.offset(i as isize) as libc::c_int
            ^ *ecc.offset(0 as libc::c_int as isize) as libc::c_int) as usize];
        if feedback as libc::c_uint
            != ((1 as libc::c_uint) << 8 as libc::c_int).wrapping_sub(1 as libc::c_uint)
        {
            j = 1 as libc::c_int as size_t;
            while j < ecc_length {
                *ecc
                    .offset(
                        j as isize,
                    ) = (*ecc.offset(j as isize) as libc::c_int
                    ^ alpha[((feedback as libc::c_int
                        + *gen.offset(ecc_length.wrapping_sub(j) as isize)
                            as libc::c_int) as libc::c_uint)
                        .wrapping_rem(
                            ((1 as libc::c_uint) << 8 as libc::c_int)
                                .wrapping_sub(1 as libc::c_uint),
                        ) as usize] as libc::c_int) as libc::c_uchar;
                j = j.wrapping_add(1);
            }
        }
        memmove(
            ecc.offset(0 as libc::c_int as isize) as *mut libc::c_void,
            ecc.offset(1 as libc::c_int as isize) as *const libc::c_void,
            ecc_length.wrapping_sub(1 as libc::c_ulong),
        );
        if feedback as libc::c_uint
            != ((1 as libc::c_uint) << 8 as libc::c_int).wrapping_sub(1 as libc::c_uint)
        {
            *ecc
                .offset(
                    ecc_length.wrapping_sub(1 as libc::c_ulong) as isize,
                ) = alpha[((feedback as libc::c_int
                + *gen.offset(0 as libc::c_int as isize) as libc::c_int) as libc::c_uint)
                .wrapping_rem(
                    ((1 as libc::c_uint) << 8 as libc::c_int)
                        .wrapping_sub(1 as libc::c_uint),
                ) as usize];
        } else {
            *ecc
                .offset(
                    ecc_length.wrapping_sub(1 as libc::c_ulong) as isize,
                ) = 0 as libc::c_int as libc::c_uchar;
        }
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn Split_identifyMode(
    mut string: *const libc::c_char,
    mut hint: QRencodeMode,
) -> QRencodeMode {
    let mut c: libc::c_uchar = 0;
    let mut d: libc::c_uchar = 0;
    let mut word: libc::c_uint = 0;
    let mut tmp: libc::c_int = 0;
    c = *string.offset(0 as libc::c_int as isize) as libc::c_uchar;
    if c as libc::c_int == 0 as libc::c_int {
        return QR_MODE_NUL;
    }
    if ((c as libc::c_schar as libc::c_int - 48 as libc::c_int) as libc::c_uchar
        as libc::c_int) < 10 as libc::c_int
    {
        return QR_MODE_NUM
    } else {
        if c as libc::c_int & 128 as libc::c_int != 0 {
            tmp = -(1 as libc::c_int);
        } else {
            tmp = QRinput_anTable[c as libc::c_int as usize] as libc::c_int;
        }
        if tmp >= 0 as libc::c_int {
            return QR_MODE_AN
        } else {
            if hint as libc::c_int == 3 as libc::c_int {
                d = *string.offset(1 as libc::c_int as isize) as libc::c_uchar;
                if d as libc::c_int != 0 as libc::c_int {
                    word = (c as libc::c_uint) << 8 as libc::c_int | d as libc::c_uint;
                    if word >= 33088 as libc::c_uint {
                        if word <= 40956 as libc::c_uint {
                            return QR_MODE_KANJI;
                        }
                    }
                    if word >= 57408 as libc::c_uint {
                        if word <= 60351 as libc::c_uint {
                            return QR_MODE_KANJI;
                        }
                    }
                }
            }
        }
    }
    return QR_MODE_8;
}
unsafe extern "C" fn Split_eatNum(
    mut string: *const libc::c_char,
    mut input: *mut QRinput,
    mut hint: QRencodeMode,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: libc::c_int = 0;
    let mut run: libc::c_int = 0;
    let mut dif: libc::c_int = 0;
    let mut ln: libc::c_int = 0;
    let mut mode: QRencodeMode = QR_MODE_NUM;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    ln = QRspec_lengthIndicator(QR_MODE_NUM, (*input).version);
    p = string;
    while ((*p as libc::c_schar as libc::c_int - 48 as libc::c_int) as libc::c_uchar
        as libc::c_int) < 10 as libc::c_int
    {
        p = p.offset(1);
    }
    run = p.offset_from(string) as libc::c_long as libc::c_int;
    mode = Split_identifyMode(p, hint);
    if mode as libc::c_int == 2 as libc::c_int {
        tmp = QRinput_estimateBitsModeNum(run);
        tmp___0 = QRinput_estimateBitsMode8(1 as libc::c_int);
        tmp___1 = QRinput_estimateBitsMode8(run + 1 as libc::c_int);
        dif = tmp + 4 as libc::c_int + ln + tmp___0 - tmp___1;
        if dif > 0 as libc::c_int {
            tmp___2 = Split_eat8(string, input, hint);
            return tmp___2;
        }
    }
    if mode as libc::c_int == 1 as libc::c_int {
        tmp___3 = QRinput_estimateBitsModeNum(run);
        tmp___4 = QRinput_estimateBitsModeAn(1 as libc::c_int);
        tmp___5 = QRinput_estimateBitsModeAn(run + 1 as libc::c_int);
        dif = tmp___3 + 4 as libc::c_int + ln + tmp___4 - tmp___5;
        if dif > 0 as libc::c_int {
            tmp___6 = Split_eatAn(string, input, hint);
            return tmp___6;
        }
    }
    ret = QRinput_append(
        input,
        QR_MODE_NUM,
        run,
        string as *mut libc::c_uchar as *const libc::c_uchar,
    );
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return run;
}
unsafe extern "C" fn Split_eatAn(
    mut string: *const libc::c_char,
    mut input: *mut QRinput,
    mut hint: QRencodeMode,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: libc::c_int = 0;
    let mut run: libc::c_int = 0;
    let mut dif: libc::c_int = 0;
    let mut la: libc::c_int = 0;
    let mut ln: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    la = QRspec_lengthIndicator(QR_MODE_AN, (*input).version);
    ln = QRspec_lengthIndicator(QR_MODE_NUM, (*input).version);
    p = string;
    loop {
        if *p as libc::c_int & 128 as libc::c_int != 0 {
            tmp___5 = -(1 as libc::c_int);
        } else {
            tmp___5 = QRinput_anTable[*p as libc::c_int as usize] as libc::c_int;
        }
        if !(tmp___5 >= 0 as libc::c_int) {
            break;
        }
        if ((*p as libc::c_schar as libc::c_int - 48 as libc::c_int) as libc::c_uchar
            as libc::c_int) < 10 as libc::c_int
        {
            q = p;
            while ((*q as libc::c_schar as libc::c_int - 48 as libc::c_int)
                as libc::c_uchar as libc::c_int) < 10 as libc::c_int
            {
                q = q.offset(1);
            }
            tmp = QRinput_estimateBitsModeAn(
                p.offset_from(string) as libc::c_long as libc::c_int,
            );
            tmp___0 = QRinput_estimateBitsModeNum(
                q.offset_from(p) as libc::c_long as libc::c_int,
            );
            if *q as libc::c_int & 128 as libc::c_int != 0 {
                tmp___3 = -(1 as libc::c_int);
            } else {
                tmp___3 = QRinput_anTable[*q as libc::c_int as usize] as libc::c_int;
            }
            if tmp___3 >= 0 as libc::c_int {
                tmp___2 = 4 as libc::c_int + ln;
            } else {
                tmp___2 = 0 as libc::c_int;
            }
            tmp___4 = QRinput_estimateBitsModeAn(
                q.offset_from(string) as libc::c_long as libc::c_int,
            );
            dif = tmp + tmp___0 + 4 as libc::c_int + ln + tmp___2 - tmp___4;
            if dif < 0 as libc::c_int {
                break;
            }
            p = q;
        } else {
            p = p.offset(1);
        }
    }
    run = p.offset_from(string) as libc::c_long as libc::c_int;
    if *p != 0 {
        if *p as libc::c_int & 128 as libc::c_int != 0 {
            tmp___10 = -(1 as libc::c_int);
        } else {
            tmp___10 = QRinput_anTable[*p as libc::c_int as usize] as libc::c_int;
        }
        if !(tmp___10 >= 0 as libc::c_int) {
            tmp___6 = QRinput_estimateBitsModeAn(run);
            tmp___7 = QRinput_estimateBitsMode8(1 as libc::c_int);
            tmp___8 = QRinput_estimateBitsMode8(run + 1 as libc::c_int);
            dif = tmp___6 + 4 as libc::c_int + la + tmp___7 - tmp___8;
            if dif > 0 as libc::c_int {
                tmp___9 = Split_eat8(string, input, hint);
                return tmp___9;
            }
        }
    }
    ret = QRinput_append(
        input,
        QR_MODE_AN,
        run,
        string as *mut libc::c_uchar as *const libc::c_uchar,
    );
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return run;
}
unsafe extern "C" fn Split_eatKanji(
    mut string: *const libc::c_char,
    mut input: *mut QRinput,
    mut hint: QRencodeMode,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: libc::c_int = 0;
    let mut run: libc::c_int = 0;
    let mut tmp: QRencodeMode = QR_MODE_NUM;
    p = string;
    loop {
        tmp = Split_identifyMode(p, hint);
        if !(tmp as libc::c_int == 3 as libc::c_int) {
            break;
        }
        p = p.offset(2 as libc::c_int as isize);
    }
    run = p.offset_from(string) as libc::c_long as libc::c_int;
    ret = QRinput_append(
        input,
        QR_MODE_KANJI,
        run,
        string as *mut libc::c_uchar as *const libc::c_uchar,
    );
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return run;
}
unsafe extern "C" fn Split_eat8(
    mut string: *const libc::c_char,
    mut input: *mut QRinput,
    mut hint: QRencodeMode,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    let mut mode: QRencodeMode = QR_MODE_NUM;
    let mut ret: libc::c_int = 0;
    let mut run: libc::c_int = 0;
    let mut dif: libc::c_int = 0;
    let mut la: libc::c_int = 0;
    let mut ln: libc::c_int = 0;
    let mut l8: libc::c_int = 0;
    let mut swcost: libc::c_int = 0;
    let mut tmp: QRencodeMode = QR_MODE_NUM;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: QRencodeMode = QR_MODE_NUM;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    la = QRspec_lengthIndicator(QR_MODE_AN, (*input).version);
    ln = QRspec_lengthIndicator(QR_MODE_NUM, (*input).version);
    l8 = QRspec_lengthIndicator(QR_MODE_8, (*input).version);
    p = string.offset(1 as libc::c_int as isize);
    while *p as libc::c_int != 0 as libc::c_int {
        mode = Split_identifyMode(p, hint);
        if mode as libc::c_int == 3 as libc::c_int {
            break;
        }
        if mode as libc::c_int == 0 as libc::c_int {
            q = p;
            while ((*q as libc::c_schar as libc::c_int - 48 as libc::c_int)
                as libc::c_uchar as libc::c_int) < 10 as libc::c_int
            {
                q = q.offset(1);
            }
            tmp = Split_identifyMode(q, hint);
            if tmp as libc::c_int == 2 as libc::c_int {
                swcost = 4 as libc::c_int + l8;
            } else {
                swcost = 0 as libc::c_int;
            }
            tmp___0 = QRinput_estimateBitsMode8(
                p.offset_from(string) as libc::c_long as libc::c_int,
            );
            tmp___1 = QRinput_estimateBitsModeNum(
                q.offset_from(p) as libc::c_long as libc::c_int,
            );
            tmp___2 = QRinput_estimateBitsMode8(
                q.offset_from(string) as libc::c_long as libc::c_int,
            );
            dif = tmp___0 + tmp___1 + 4 as libc::c_int + ln + swcost - tmp___2;
            if dif < 0 as libc::c_int {
                break;
            }
            p = q;
        } else if mode as libc::c_int == 1 as libc::c_int {
            q = p;
            loop {
                if *q as libc::c_int & 128 as libc::c_int != 0 {
                    tmp___3 = -(1 as libc::c_int);
                } else {
                    tmp___3 = QRinput_anTable[*q as libc::c_int as usize] as libc::c_int;
                }
                if !(tmp___3 >= 0 as libc::c_int) {
                    break;
                }
                q = q.offset(1);
            }
            tmp___4 = Split_identifyMode(q, hint);
            if tmp___4 as libc::c_int == 2 as libc::c_int {
                swcost = 4 as libc::c_int + l8;
            } else {
                swcost = 0 as libc::c_int;
            }
            tmp___5 = QRinput_estimateBitsMode8(
                p.offset_from(string) as libc::c_long as libc::c_int,
            );
            tmp___6 = QRinput_estimateBitsModeAn(
                q.offset_from(p) as libc::c_long as libc::c_int,
            );
            tmp___7 = QRinput_estimateBitsMode8(
                q.offset_from(string) as libc::c_long as libc::c_int,
            );
            dif = tmp___5 + tmp___6 + 4 as libc::c_int + la + swcost - tmp___7;
            if dif < 0 as libc::c_int {
                break;
            }
            p = q;
        } else {
            p = p.offset(1);
        }
    }
    run = p.offset_from(string) as libc::c_long as libc::c_int;
    ret = QRinput_append(
        input,
        QR_MODE_8,
        run,
        string as *mut libc::c_uchar as *const libc::c_uchar,
    );
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return run;
}
unsafe extern "C" fn Split_splitString(
    mut string: *const libc::c_char,
    mut input: *mut QRinput,
    mut hint: QRencodeMode,
) -> libc::c_int {
    let mut length: libc::c_int = 0;
    let mut mode: QRencodeMode = QR_MODE_NUM;
    while *string as libc::c_int != 0 as libc::c_int {
        mode = Split_identifyMode(string, hint);
        if mode as libc::c_int == 0 as libc::c_int {
            length = Split_eatNum(string, input, hint);
        } else if mode as libc::c_int == 1 as libc::c_int {
            length = Split_eatAn(string, input, hint);
        } else if mode as libc::c_int == 3 as libc::c_int {
            if hint as libc::c_int == 3 as libc::c_int {
                length = Split_eatKanji(string, input, hint);
            } else {
                length = Split_eat8(string, input, hint);
            }
        } else {
            length = Split_eat8(string, input, hint);
        }
        if length == 0 as libc::c_int {
            break;
        }
        if length < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        string = string.offset(length as isize);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn dupAndToUpper(
    mut str: *const libc::c_char,
    mut hint: QRencodeMode,
) -> *mut libc::c_char {
    let mut newstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mode: QRencodeMode = QR_MODE_NUM;
    newstr = strdup(str);
    if newstr as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    p = newstr;
    while *p as libc::c_int != 0 as libc::c_int {
        mode = Split_identifyMode(p as *const libc::c_char, hint);
        if mode as libc::c_int == 3 as libc::c_int {
            p = p.offset(2 as libc::c_int as isize);
        } else {
            if *p as libc::c_int >= 97 as libc::c_int {
                if *p as libc::c_int <= 122 as libc::c_int {
                    *p = (*p as libc::c_int - 32 as libc::c_int) as libc::c_char;
                }
            }
            p = p.offset(1);
        }
    }
    return newstr;
}
pub unsafe extern "C" fn Split_splitStringToQRinput(
    mut string: *const libc::c_char,
    mut input: *mut QRinput,
    mut hint: QRencodeMode,
    mut casesensitive: libc::c_int,
) -> libc::c_int {
    let mut newstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    if string as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
        return -(1 as libc::c_int);
    } else {
        if *string as libc::c_int == 0 as libc::c_int {
            tmp = __errno_location();
            *tmp = 22 as libc::c_int;
            return -(1 as libc::c_int);
        }
    }
    if casesensitive == 0 {
        newstr = dupAndToUpper(string, hint);
        if newstr as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            return -(1 as libc::c_int);
        }
        ret = Split_splitString(newstr as *const libc::c_char, input, hint);
        free(newstr as *mut libc::c_void);
    } else {
        ret = Split_splitString(string, input, hint);
    }
    return ret;
}
pub unsafe extern "C" fn Mask_writeFormatInformation(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut mask: libc::c_int,
    mut level: QRecLevel,
) -> libc::c_int {
    let mut format: libc::c_uint = 0;
    let mut v: libc::c_uchar = 0;
    let mut i: libc::c_int = 0;
    let mut blacks: libc::c_int = 0;
    blacks = 0 as libc::c_int;
    format = QRspec_getFormatInfo(mask, level);
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if format & 1 as libc::c_uint != 0 {
            blacks += 2 as libc::c_int;
            v = 133 as libc::c_int as libc::c_uchar;
        } else {
            v = 132 as libc::c_int as libc::c_uchar;
        }
        *frame
            .offset(
                (width * 8 as libc::c_int + width - 1 as libc::c_int - i) as isize,
            ) = v;
        if i < 6 as libc::c_int {
            *frame.offset((width * i + 8 as libc::c_int) as isize) = v;
        } else {
            *frame
                .offset(
                    (width * (i + 1 as libc::c_int) + 8 as libc::c_int) as isize,
                ) = v;
        }
        format >>= 1 as libc::c_int;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        if format & 1 as libc::c_uint != 0 {
            blacks += 2 as libc::c_int;
            v = 133 as libc::c_int as libc::c_uchar;
        } else {
            v = 132 as libc::c_int as libc::c_uchar;
        }
        *frame
            .offset(
                (width * (width - 7 as libc::c_int + i) + 8 as libc::c_int) as isize,
            ) = v;
        if i == 0 as libc::c_int {
            *frame.offset((width * 8 as libc::c_int + 7 as libc::c_int) as isize) = v;
        } else {
            *frame
                .offset((width * 8 as libc::c_int + 6 as libc::c_int - i) as isize) = v;
        }
        format >>= 1 as libc::c_int;
        i += 1;
    }
    return blacks;
}
unsafe extern "C" fn Mask_mask0(
    mut width: libc::c_int,
    mut s: *const libc::c_uchar,
    mut d: *mut libc::c_uchar,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    b = 0 as libc::c_int;
    y = 0 as libc::c_int;
    while y < width {
        x = 0 as libc::c_int;
        while x < width {
            if *s as libc::c_int & 128 as libc::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as libc::c_int
                    ^ (x + y & 1 as libc::c_int == 0 as libc::c_int) as libc::c_int)
                    as libc::c_uchar;
            }
            b += *d as libc::c_int & 1 as libc::c_int;
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
    return b;
}
unsafe extern "C" fn Mask_mask1(
    mut width: libc::c_int,
    mut s: *const libc::c_uchar,
    mut d: *mut libc::c_uchar,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    b = 0 as libc::c_int;
    y = 0 as libc::c_int;
    while y < width {
        x = 0 as libc::c_int;
        while x < width {
            if *s as libc::c_int & 128 as libc::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as libc::c_int
                    ^ (y & 1 as libc::c_int == 0 as libc::c_int) as libc::c_int)
                    as libc::c_uchar;
            }
            b += *d as libc::c_int & 1 as libc::c_int;
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
    return b;
}
unsafe extern "C" fn Mask_mask2(
    mut width: libc::c_int,
    mut s: *const libc::c_uchar,
    mut d: *mut libc::c_uchar,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    b = 0 as libc::c_int;
    y = 0 as libc::c_int;
    while y < width {
        x = 0 as libc::c_int;
        while x < width {
            if *s as libc::c_int & 128 as libc::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as libc::c_int
                    ^ (x % 3 as libc::c_int == 0 as libc::c_int) as libc::c_int)
                    as libc::c_uchar;
            }
            b += *d as libc::c_int & 1 as libc::c_int;
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
    return b;
}
unsafe extern "C" fn Mask_mask3(
    mut width: libc::c_int,
    mut s: *const libc::c_uchar,
    mut d: *mut libc::c_uchar,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    b = 0 as libc::c_int;
    y = 0 as libc::c_int;
    while y < width {
        x = 0 as libc::c_int;
        while x < width {
            if *s as libc::c_int & 128 as libc::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as libc::c_int
                    ^ ((x + y) % 3 as libc::c_int == 0 as libc::c_int) as libc::c_int)
                    as libc::c_uchar;
            }
            b += *d as libc::c_int & 1 as libc::c_int;
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
    return b;
}
unsafe extern "C" fn Mask_mask4(
    mut width: libc::c_int,
    mut s: *const libc::c_uchar,
    mut d: *mut libc::c_uchar,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    b = 0 as libc::c_int;
    y = 0 as libc::c_int;
    while y < width {
        x = 0 as libc::c_int;
        while x < width {
            if *s as libc::c_int & 128 as libc::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as libc::c_int
                    ^ (y / 2 as libc::c_int + x / 3 as libc::c_int & 1 as libc::c_int
                        == 0 as libc::c_int) as libc::c_int) as libc::c_uchar;
            }
            b += *d as libc::c_int & 1 as libc::c_int;
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
    return b;
}
unsafe extern "C" fn Mask_mask5(
    mut width: libc::c_int,
    mut s: *const libc::c_uchar,
    mut d: *mut libc::c_uchar,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    b = 0 as libc::c_int;
    y = 0 as libc::c_int;
    while y < width {
        x = 0 as libc::c_int;
        while x < width {
            if *s as libc::c_int & 128 as libc::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as libc::c_int
                    ^ ((x * y & 1 as libc::c_int) + x * y % 3 as libc::c_int
                        == 0 as libc::c_int) as libc::c_int) as libc::c_uchar;
            }
            b += *d as libc::c_int & 1 as libc::c_int;
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
    return b;
}
unsafe extern "C" fn Mask_mask6(
    mut width: libc::c_int,
    mut s: *const libc::c_uchar,
    mut d: *mut libc::c_uchar,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    b = 0 as libc::c_int;
    y = 0 as libc::c_int;
    while y < width {
        x = 0 as libc::c_int;
        while x < width {
            if *s as libc::c_int & 128 as libc::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as libc::c_int
                    ^ ((x * y & 1 as libc::c_int) + x * y % 3 as libc::c_int
                        & 1 as libc::c_int == 0 as libc::c_int) as libc::c_int)
                    as libc::c_uchar;
            }
            b += *d as libc::c_int & 1 as libc::c_int;
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
    return b;
}
unsafe extern "C" fn Mask_mask7(
    mut width: libc::c_int,
    mut s: *const libc::c_uchar,
    mut d: *mut libc::c_uchar,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    b = 0 as libc::c_int;
    y = 0 as libc::c_int;
    while y < width {
        x = 0 as libc::c_int;
        while x < width {
            if *s as libc::c_int & 128 as libc::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as libc::c_int
                    ^ (x * y % 3 as libc::c_int + (x + y & 1 as libc::c_int)
                        & 1 as libc::c_int == 0 as libc::c_int) as libc::c_int)
                    as libc::c_uchar;
            }
            b += *d as libc::c_int & 1 as libc::c_int;
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
    return b;
}
static mut maskMakers: [Option::<MaskMaker>; 8] = [
    Some(
        Mask_mask0
            as unsafe extern "C" fn(
                libc::c_int,
                *const libc::c_uchar,
                *mut libc::c_uchar,
            ) -> libc::c_int,
    ),
    Some(
        Mask_mask1
            as unsafe extern "C" fn(
                libc::c_int,
                *const libc::c_uchar,
                *mut libc::c_uchar,
            ) -> libc::c_int,
    ),
    Some(
        Mask_mask2
            as unsafe extern "C" fn(
                libc::c_int,
                *const libc::c_uchar,
                *mut libc::c_uchar,
            ) -> libc::c_int,
    ),
    Some(
        Mask_mask3
            as unsafe extern "C" fn(
                libc::c_int,
                *const libc::c_uchar,
                *mut libc::c_uchar,
            ) -> libc::c_int,
    ),
    Some(
        Mask_mask4
            as unsafe extern "C" fn(
                libc::c_int,
                *const libc::c_uchar,
                *mut libc::c_uchar,
            ) -> libc::c_int,
    ),
    Some(
        Mask_mask5
            as unsafe extern "C" fn(
                libc::c_int,
                *const libc::c_uchar,
                *mut libc::c_uchar,
            ) -> libc::c_int,
    ),
    Some(
        Mask_mask6
            as unsafe extern "C" fn(
                libc::c_int,
                *const libc::c_uchar,
                *mut libc::c_uchar,
            ) -> libc::c_int,
    ),
    Some(
        Mask_mask7
            as unsafe extern "C" fn(
                libc::c_int,
                *const libc::c_uchar,
                *mut libc::c_uchar,
            ) -> libc::c_int,
    ),
];
pub unsafe extern "C" fn Mask_makeMaskedFrame(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut mask: libc::c_int,
) -> *mut libc::c_uchar {
    let mut masked: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc((width * width) as size_t);
    masked = tmp as *mut libc::c_uchar;
    if masked as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut libc::c_uchar;
    }
    (Some(
        (*maskMakers.as_mut_ptr().offset(mask as isize))
            .expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(width, frame as *const libc::c_uchar, masked);
    return masked;
}
pub unsafe extern "C" fn Mask_makeMask(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut mask: libc::c_int,
    mut level: QRecLevel,
) -> *mut libc::c_uchar {
    let mut masked: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    if mask < 0 as libc::c_int {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
        return 0 as *mut libc::c_void as *mut libc::c_uchar;
    } else {
        if mask >= 8 as libc::c_int {
            tmp = __errno_location();
            *tmp = 22 as libc::c_int;
            return 0 as *mut libc::c_void as *mut libc::c_uchar;
        }
    }
    tmp___0 = malloc((width * width) as size_t);
    masked = tmp___0 as *mut libc::c_uchar;
    if masked as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut libc::c_uchar;
    }
    (Some(
        (*maskMakers.as_mut_ptr().offset(mask as isize))
            .expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(width, frame as *const libc::c_uchar, masked);
    Mask_writeFormatInformation(width, masked, mask, level);
    return masked;
}
pub unsafe extern "C" fn Mask_calcN1N3(
    mut length: libc::c_int,
    mut runLength: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut demerit: libc::c_int = 0;
    let mut fact: libc::c_int = 0;
    demerit = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < length {
        if *runLength.offset(i as isize) >= 5 as libc::c_int {
            demerit
                += 3 as libc::c_int + (*runLength.offset(i as isize) - 5 as libc::c_int);
        }
        if i & 1 as libc::c_int != 0 {
            if i >= 3 as libc::c_int {
                if i < length - 2 as libc::c_int {
                    if *runLength.offset(i as isize) % 3 as libc::c_int
                        == 0 as libc::c_int
                    {
                        fact = *runLength.offset(i as isize) / 3 as libc::c_int;
                        if *runLength.offset((i - 2 as libc::c_int) as isize) == fact {
                            if *runLength.offset((i - 1 as libc::c_int) as isize) == fact
                            {
                                if *runLength.offset((i + 1 as libc::c_int) as isize)
                                    == fact
                                {
                                    if *runLength.offset((i + 2 as libc::c_int) as isize)
                                        == fact
                                    {
                                        if i == 3 as libc::c_int {
                                            demerit += 40 as libc::c_int;
                                        } else if *runLength.offset((i - 3 as libc::c_int) as isize)
                                                >= 4 as libc::c_int * fact
                                            {
                                            demerit += 40 as libc::c_int;
                                        } else if i + 4 as libc::c_int >= length {
                                            demerit += 40 as libc::c_int;
                                        } else if *runLength.offset((i + 3 as libc::c_int) as isize)
                                                >= 4 as libc::c_int * fact
                                            {
                                            demerit += 40 as libc::c_int;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    return demerit;
}
pub unsafe extern "C" fn Mask_calcN2(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut b22: libc::c_uchar = 0;
    let mut w22: libc::c_uchar = 0;
    let mut demerit: libc::c_int = 0;
    demerit = 0 as libc::c_int;
    p = frame.offset(width as isize).offset(1 as libc::c_int as isize);
    y = 1 as libc::c_int;
    while y < width {
        x = 1 as libc::c_int;
        while x < width {
            b22 = (*p.offset(0 as libc::c_int as isize) as libc::c_int
                & *p.offset(-(1 as libc::c_int) as isize) as libc::c_int
                & *p.offset(-width as isize) as libc::c_int
                & *p.offset((-width - 1 as libc::c_int) as isize) as libc::c_int)
                as libc::c_uchar;
            w22 = (*p.offset(0 as libc::c_int as isize) as libc::c_int
                | *p.offset(-(1 as libc::c_int) as isize) as libc::c_int
                | *p.offset(-width as isize) as libc::c_int
                | *p.offset((-width - 1 as libc::c_int) as isize) as libc::c_int)
                as libc::c_uchar;
            if (b22 as libc::c_int | w22 as libc::c_int ^ 1 as libc::c_int)
                & 1 as libc::c_int != 0
            {
                demerit += 3 as libc::c_int;
            }
            p = p.offset(1);
            x += 1;
        }
        p = p.offset(1);
        y += 1;
    }
    return demerit;
}
pub unsafe extern "C" fn Mask_calcRunLengthH(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut runLength: *mut libc::c_int,
) -> libc::c_int {
    let mut head: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut prev: libc::c_uchar = 0;
    if *frame.offset(0 as libc::c_int as isize) as libc::c_int & 1 as libc::c_int != 0 {
        *runLength.offset(0 as libc::c_int as isize) = -(1 as libc::c_int);
        head = 1 as libc::c_int;
    } else {
        head = 0 as libc::c_int;
    }
    *runLength.offset(head as isize) = 1 as libc::c_int;
    prev = *frame.offset(0 as libc::c_int as isize);
    i = 1 as libc::c_int;
    while i < width {
        if (*frame.offset(i as isize) as libc::c_int ^ prev as libc::c_int)
            & 1 as libc::c_int != 0
        {
            head += 1;
            *runLength.offset(head as isize) = 1 as libc::c_int;
            prev = *frame.offset(i as isize);
        } else {
            let ref mut fresh0 = *runLength.offset(head as isize);
            *fresh0 += 1;
        }
        i += 1;
    }
    return head + 1 as libc::c_int;
}
pub unsafe extern "C" fn Mask_calcRunLengthV(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut runLength: *mut libc::c_int,
) -> libc::c_int {
    let mut head: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut prev: libc::c_uchar = 0;
    if *frame.offset(0 as libc::c_int as isize) as libc::c_int & 1 as libc::c_int != 0 {
        *runLength.offset(0 as libc::c_int as isize) = -(1 as libc::c_int);
        head = 1 as libc::c_int;
    } else {
        head = 0 as libc::c_int;
    }
    *runLength.offset(head as isize) = 1 as libc::c_int;
    prev = *frame.offset(0 as libc::c_int as isize);
    i = 1 as libc::c_int;
    while i < width {
        if (*frame.offset((i * width) as isize) as libc::c_int ^ prev as libc::c_int)
            & 1 as libc::c_int != 0
        {
            head += 1;
            *runLength.offset(head as isize) = 1 as libc::c_int;
            prev = *frame.offset((i * width) as isize);
        } else {
            let ref mut fresh1 = *runLength.offset(head as isize);
            *fresh1 += 1;
        }
        i += 1;
    }
    return head + 1 as libc::c_int;
}
pub unsafe extern "C" fn Mask_evaluateSymbol(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut demerit: libc::c_int = 0;
    let mut runLength: [libc::c_int; 178] = [0; 178];
    let mut length: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    demerit = 0 as libc::c_int;
    tmp = Mask_calcN2(width, frame);
    demerit += tmp;
    y = 0 as libc::c_int;
    while y < width {
        length = Mask_calcRunLengthH(
            width,
            frame.offset((y * width) as isize),
            runLength.as_mut_ptr(),
        );
        tmp___0 = Mask_calcN1N3(length, runLength.as_mut_ptr());
        demerit += tmp___0;
        y += 1;
    }
    x = 0 as libc::c_int;
    while x < width {
        length = Mask_calcRunLengthV(
            width,
            frame.offset(x as isize),
            runLength.as_mut_ptr(),
        );
        tmp___1 = Mask_calcN1N3(length, runLength.as_mut_ptr());
        demerit += tmp___1;
        x += 1;
    }
    return demerit;
}
pub unsafe extern "C" fn Mask_mask(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut level: QRecLevel,
) -> *mut libc::c_uchar {
    let mut i: libc::c_int = 0;
    let mut mask: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut bestMask: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut minDemerit: libc::c_int = 0;
    let mut blacks: libc::c_int = 0;
    let mut bratio: libc::c_int = 0;
    let mut demerit: libc::c_int = 0;
    let mut w2: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    minDemerit = 2147483647 as libc::c_int;
    w2 = width * width;
    tmp = malloc(w2 as size_t);
    mask = tmp as *mut libc::c_uchar;
    if mask as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut libc::c_uchar;
    }
    tmp___0 = malloc(w2 as size_t);
    bestMask = tmp___0 as *mut libc::c_uchar;
    if bestMask as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        free(mask as *mut libc::c_void);
        return 0 as *mut libc::c_void as *mut libc::c_uchar;
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        demerit = 0 as libc::c_int;
        blacks = (Some(
            (*maskMakers.as_mut_ptr().offset(i as isize))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(width, frame as *const libc::c_uchar, mask);
        tmp___1 = Mask_writeFormatInformation(width, mask, i, level);
        blacks += tmp___1;
        bratio = (200 as libc::c_int * blacks + w2) / w2 / 2 as libc::c_int;
        tmp___2 = abs(bratio - 50 as libc::c_int);
        demerit = tmp___2 / 5 as libc::c_int * 10 as libc::c_int;
        tmp___3 = Mask_evaluateSymbol(width, mask);
        demerit += tmp___3;
        if demerit < minDemerit {
            minDemerit = demerit;
            memcpy(
                bestMask as *mut libc::c_void,
                mask as *const libc::c_void,
                w2 as size_t,
            );
        }
        i += 1;
    }
    free(mask as *mut libc::c_void);
    return bestMask;
}
static mut mqrspecCapacity: [MQRspec_Capacity; 5] = [
    {
        let mut init = __anonstruct_MQRspec_Capacity_991265789 {
            width: 0 as libc::c_int,
            ec: [0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int],
        };
        init
    },
    {
        let mut init = __anonstruct_MQRspec_Capacity_991265789 {
            width: 11 as libc::c_int,
            ec: [2 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int],
        };
        init
    },
    {
        let mut init = __anonstruct_MQRspec_Capacity_991265789 {
            width: 13 as libc::c_int,
            ec: [5 as libc::c_int, 6 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int],
        };
        init
    },
    {
        let mut init = __anonstruct_MQRspec_Capacity_991265789 {
            width: 15 as libc::c_int,
            ec: [6 as libc::c_int, 8 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int],
        };
        init
    },
    {
        let mut init = __anonstruct_MQRspec_Capacity_991265789 {
            width: 17 as libc::c_int,
            ec: [
                8 as libc::c_int,
                10 as libc::c_int,
                14 as libc::c_int,
                0 as libc::c_int,
            ],
        };
        init
    },
];
pub unsafe extern "C" fn MQRspec_getDataLengthBit(
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> libc::c_int {
    let mut w: libc::c_int = 0;
    let mut ecc: libc::c_int = 0;
    w = mqrspecCapacity[version as usize].width - 1 as libc::c_int;
    ecc = mqrspecCapacity[version as usize].ec[level as usize];
    if ecc == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return w * w - 64 as libc::c_int - ecc * 8 as libc::c_int;
}
pub unsafe extern "C" fn MQRspec_getDataLength(
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = MQRspec_getDataLengthBit(version, level);
    return (tmp + 4 as libc::c_int) / 8 as libc::c_int;
}
pub unsafe extern "C" fn MQRspec_getECCLength(
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> libc::c_int {
    return mqrspecCapacity[version as usize].ec[level as usize];
}
pub unsafe extern "C" fn MQRspec_getWidth(mut version: libc::c_int) -> libc::c_int {
    return mqrspecCapacity[version as usize].width;
}
static mut lengthTableBits___0: [[libc::c_int; 4]; 4] = [
    [3 as libc::c_int, 4 as libc::c_int, 5 as libc::c_int, 6 as libc::c_int],
    [0 as libc::c_int, 3 as libc::c_int, 4 as libc::c_int, 5 as libc::c_int],
    [0 as libc::c_int, 0 as libc::c_int, 4 as libc::c_int, 5 as libc::c_int],
    [0 as libc::c_int, 0 as libc::c_int, 3 as libc::c_int, 4 as libc::c_int],
];
pub unsafe extern "C" fn MQRspec_lengthIndicator(
    mut mode: QRencodeMode,
    mut version: libc::c_int,
) -> libc::c_int {
    return lengthTableBits___0[mode as usize][(version - 1 as libc::c_int) as usize];
}
pub unsafe extern "C" fn MQRspec_maximumWords(
    mut mode: QRencodeMode,
    mut version: libc::c_int,
) -> libc::c_int {
    let mut bits: libc::c_int = 0;
    let mut words: libc::c_int = 0;
    bits = lengthTableBits___0[mode as usize][(version - 1 as libc::c_int) as usize];
    words = ((1 as libc::c_int) << bits) - 1 as libc::c_int;
    if mode as libc::c_int == 3 as libc::c_int {
        words *= 2 as libc::c_int;
    }
    return words;
}
static mut formatInfo___0: [[libc::c_uint; 8]; 4] = [
    [
        17477 as libc::c_int as libc::c_uint,
        21934 as libc::c_int as libc::c_uint,
        26515 as libc::c_int as libc::c_uint,
        30328 as libc::c_int as libc::c_uint,
        1758 as libc::c_int as libc::c_uint,
        5941 as libc::c_int as libc::c_uint,
        9480 as libc::c_int as libc::c_uint,
        13539 as libc::c_int as libc::c_uint,
    ],
    [
        16754 as libc::c_int as libc::c_uint,
        20633 as libc::c_int as libc::c_uint,
        25252 as libc::c_int as libc::c_uint,
        29519 as libc::c_int as libc::c_uint,
        1001 as libc::c_int as libc::c_uint,
        4610 as libc::c_int as libc::c_uint,
        8255 as libc::c_int as libc::c_uint,
        12756 as libc::c_int as libc::c_uint,
    ],
    [
        20011 as libc::c_int as libc::c_uint,
        24512 as libc::c_int as libc::c_uint,
        28157 as libc::c_int as libc::c_uint,
        31766 as libc::c_int as libc::c_uint,
        3248 as libc::c_int as libc::c_uint,
        7515 as libc::c_int as libc::c_uint,
        12134 as libc::c_int as libc::c_uint,
        16013 as libc::c_int as libc::c_uint,
    ],
    [
        19228 as libc::c_int as libc::c_uint,
        23287 as libc::c_int as libc::c_uint,
        26826 as libc::c_int as libc::c_uint,
        31009 as libc::c_int as libc::c_uint,
        2439 as libc::c_int as libc::c_uint,
        6252 as libc::c_int as libc::c_uint,
        10833 as libc::c_int as libc::c_uint,
        15290 as libc::c_int as libc::c_uint,
    ],
];
static mut typeTable: [[libc::c_int; 3]; 5] = [
    [-(1 as libc::c_int), -(1 as libc::c_int), -(1 as libc::c_int)],
    [0 as libc::c_int, -(1 as libc::c_int), -(1 as libc::c_int)],
    [1 as libc::c_int, 2 as libc::c_int, -(1 as libc::c_int)],
    [3 as libc::c_int, 4 as libc::c_int, -(1 as libc::c_int)],
    [5 as libc::c_int, 6 as libc::c_int, 7 as libc::c_int],
];
pub unsafe extern "C" fn MQRspec_getFormatInfo(
    mut mask: libc::c_int,
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> libc::c_uint {
    let mut type_0: libc::c_int = 0;
    if mask < 0 as libc::c_int {
        return 0 as libc::c_uint
    } else {
        if mask > 3 as libc::c_int {
            return 0 as libc::c_uint;
        }
    }
    if version <= 0 as libc::c_int {
        return 0 as libc::c_uint
    } else {
        if version > 4 as libc::c_int {
            return 0 as libc::c_uint;
        }
    }
    if level as libc::c_uint == 3 as libc::c_uint {
        return 0 as libc::c_uint;
    }
    type_0 = typeTable[version as usize][level as usize];
    if type_0 < 0 as libc::c_int {
        return 0 as libc::c_uint;
    }
    return formatInfo___0[mask as usize][type_0 as usize];
}
static mut finder___1: [libc::c_uchar; 49] = [
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
];
unsafe extern "C" fn putFinderPattern___0(
    mut frame: *mut libc::c_uchar,
    mut width: libc::c_int,
    mut ox: libc::c_int,
    mut oy: libc::c_int,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut s: *const libc::c_uchar = 0 as *const libc::c_uchar;
    frame = frame.offset((oy * width + ox) as isize);
    s = finder___1.as_ptr();
    y = 0 as libc::c_int;
    while y < 7 as libc::c_int {
        x = 0 as libc::c_int;
        while x < 7 as libc::c_int {
            *frame.offset(x as isize) = *s.offset(x as isize);
            x += 1;
        }
        frame = frame.offset(width as isize);
        s = s.offset(7 as libc::c_int as isize);
        y += 1;
    }
}
unsafe extern "C" fn MQRspec_createFrame(
    mut version: libc::c_int,
) -> *mut libc::c_uchar {
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut q: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut width: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    width = mqrspecCapacity[version as usize].width;
    tmp = malloc((width * width) as size_t);
    frame = tmp as *mut libc::c_uchar;
    if frame as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut libc::c_uchar;
    }
    memset(frame as *mut libc::c_void, 0 as libc::c_int, (width * width) as size_t);
    putFinderPattern___0(frame, width, 0 as libc::c_int, 0 as libc::c_int);
    p = frame;
    y = 0 as libc::c_int;
    while y < 7 as libc::c_int {
        *p.offset(7 as libc::c_int as isize) = 192 as libc::c_int as libc::c_uchar;
        p = p.offset(width as isize);
        y += 1;
    }
    memset(
        frame.offset((width * 7 as libc::c_int) as isize) as *mut libc::c_void,
        192 as libc::c_int,
        8 as libc::c_int as size_t,
    );
    memset(
        frame
            .offset((width * 8 as libc::c_int) as isize)
            .offset(1 as libc::c_int as isize) as *mut libc::c_void,
        132 as libc::c_int,
        8 as libc::c_int as size_t,
    );
    p = frame.offset(width as isize).offset(8 as libc::c_int as isize);
    y = 0 as libc::c_int;
    while y < 7 as libc::c_int {
        *p = 132 as libc::c_int as libc::c_uchar;
        p = p.offset(width as isize);
        y += 1;
    }
    p = frame.offset(8 as libc::c_int as isize);
    q = frame.offset((width * 8 as libc::c_int) as isize);
    x = 1 as libc::c_int;
    while x < width - 7 as libc::c_int {
        *p = (144 as libc::c_int | x & 1 as libc::c_int) as libc::c_uchar;
        *q = (144 as libc::c_int | x & 1 as libc::c_int) as libc::c_uchar;
        p = p.offset(1);
        q = q.offset(width as isize);
        x += 1;
    }
    return frame;
}
pub unsafe extern "C" fn MQRspec_newFrame(
    mut version: libc::c_int,
) -> *mut libc::c_uchar {
    let mut tmp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if version < 1 as libc::c_int {
        return 0 as *mut libc::c_void as *mut libc::c_uchar
    } else {
        if version > 4 as libc::c_int {
            return 0 as *mut libc::c_void as *mut libc::c_uchar;
        }
    }
    tmp = MQRspec_createFrame(version);
    return tmp;
}
pub unsafe extern "C" fn MMask_writeFormatInformation(
    mut version: libc::c_int,
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut mask: libc::c_int,
    mut level: QRecLevel,
) {
    let mut format: libc::c_uint = 0;
    let mut v: libc::c_uchar = 0;
    let mut i: libc::c_int = 0;
    format = MQRspec_getFormatInfo(mask, version, level);
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        v = (132 as libc::c_uint | format & 1 as libc::c_uint) as libc::c_uchar;
        *frame.offset((width * (i + 1 as libc::c_int) + 8 as libc::c_int) as isize) = v;
        format >>= 1 as libc::c_int;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        v = (132 as libc::c_uint | format & 1 as libc::c_uint) as libc::c_uchar;
        *frame.offset((width * 8 as libc::c_int + 7 as libc::c_int - i) as isize) = v;
        format >>= 1 as libc::c_int;
        i += 1;
    }
}
unsafe extern "C" fn Mask_mask0___0(
    mut width: libc::c_int,
    mut s: *const libc::c_uchar,
    mut d: *mut libc::c_uchar,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < width {
        x = 0 as libc::c_int;
        while x < width {
            if *s as libc::c_int & 128 as libc::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as libc::c_int
                    ^ (y & 1 as libc::c_int == 0 as libc::c_int) as libc::c_int)
                    as libc::c_uchar;
            }
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
}
unsafe extern "C" fn Mask_mask1___0(
    mut width: libc::c_int,
    mut s: *const libc::c_uchar,
    mut d: *mut libc::c_uchar,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < width {
        x = 0 as libc::c_int;
        while x < width {
            if *s as libc::c_int & 128 as libc::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as libc::c_int
                    ^ (y / 2 as libc::c_int + x / 3 as libc::c_int & 1 as libc::c_int
                        == 0 as libc::c_int) as libc::c_int) as libc::c_uchar;
            }
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
}
unsafe extern "C" fn Mask_mask2___0(
    mut width: libc::c_int,
    mut s: *const libc::c_uchar,
    mut d: *mut libc::c_uchar,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < width {
        x = 0 as libc::c_int;
        while x < width {
            if *s as libc::c_int & 128 as libc::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as libc::c_int
                    ^ ((x * y & 1 as libc::c_int) + x * y % 3 as libc::c_int
                        & 1 as libc::c_int == 0 as libc::c_int) as libc::c_int)
                    as libc::c_uchar;
            }
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
}
unsafe extern "C" fn Mask_mask3___0(
    mut width: libc::c_int,
    mut s: *const libc::c_uchar,
    mut d: *mut libc::c_uchar,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < width {
        x = 0 as libc::c_int;
        while x < width {
            if *s as libc::c_int & 128 as libc::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as libc::c_int
                    ^ ((x + y & 1 as libc::c_int) + x * y % 3 as libc::c_int
                        & 1 as libc::c_int == 0 as libc::c_int) as libc::c_int)
                    as libc::c_uchar;
            }
            s = s.offset(1);
            d = d.offset(1);
            x += 1;
        }
        y += 1;
    }
}
static mut maskMakers___0: [Option::<MaskMaker___0>; 4] = [
    Some(
        Mask_mask0___0
            as unsafe extern "C" fn(
                libc::c_int,
                *const libc::c_uchar,
                *mut libc::c_uchar,
            ) -> (),
    ),
    Some(
        Mask_mask1___0
            as unsafe extern "C" fn(
                libc::c_int,
                *const libc::c_uchar,
                *mut libc::c_uchar,
            ) -> (),
    ),
    Some(
        Mask_mask2___0
            as unsafe extern "C" fn(
                libc::c_int,
                *const libc::c_uchar,
                *mut libc::c_uchar,
            ) -> (),
    ),
    Some(
        Mask_mask3___0
            as unsafe extern "C" fn(
                libc::c_int,
                *const libc::c_uchar,
                *mut libc::c_uchar,
            ) -> (),
    ),
];
pub unsafe extern "C" fn MMask_makeMaskedFrame(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut mask: libc::c_int,
) -> *mut libc::c_uchar {
    let mut masked: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc((width * width) as size_t);
    masked = tmp as *mut libc::c_uchar;
    if masked as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut libc::c_uchar;
    }
    (Some(
        (*maskMakers___0.as_mut_ptr().offset(mask as isize))
            .expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(width, frame as *const libc::c_uchar, masked);
    return masked;
}
pub unsafe extern "C" fn MMask_makeMask(
    mut version: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut mask: libc::c_int,
    mut level: QRecLevel,
) -> *mut libc::c_uchar {
    let mut masked: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut width: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    if mask < 0 as libc::c_int {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
        return 0 as *mut libc::c_void as *mut libc::c_uchar;
    } else {
        if mask >= 4 as libc::c_int {
            tmp = __errno_location();
            *tmp = 22 as libc::c_int;
            return 0 as *mut libc::c_void as *mut libc::c_uchar;
        }
    }
    width = MQRspec_getWidth(version);
    tmp___0 = malloc((width * width) as size_t);
    masked = tmp___0 as *mut libc::c_uchar;
    if masked as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut libc::c_uchar;
    }
    (Some(
        (*maskMakers___0.as_mut_ptr().offset(mask as isize))
            .expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(width, frame as *const libc::c_uchar, masked);
    MMask_writeFormatInformation(version, width, masked, mask, level);
    return masked;
}
pub unsafe extern "C" fn MMask_evaluateSymbol(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut sum1: libc::c_int = 0;
    let mut sum2: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    sum1 = 0 as libc::c_int;
    sum2 = 0 as libc::c_int;
    p = frame.offset((width * (width - 1 as libc::c_int)) as isize);
    x = 1 as libc::c_int;
    while x < width {
        sum1 += *p.offset(x as isize) as libc::c_int & 1 as libc::c_int;
        x += 1;
    }
    p = frame
        .offset((width * 2 as libc::c_int) as isize)
        .offset(-(1 as libc::c_int as isize));
    y = 1 as libc::c_int;
    while y < width {
        sum2 += *p as libc::c_int & 1 as libc::c_int;
        p = p.offset(width as isize);
        y += 1;
    }
    if sum1 <= sum2 {
        tmp = sum1 * 16 as libc::c_int + sum2;
    } else {
        tmp = sum2 * 16 as libc::c_int + sum1;
    }
    return tmp;
}
pub unsafe extern "C" fn MMask_mask(
    mut version: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut level: QRecLevel,
) -> *mut libc::c_uchar {
    let mut i: libc::c_int = 0;
    let mut mask: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut bestMask: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut maxScore: libc::c_int = 0;
    let mut score: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    maxScore = 0 as libc::c_int;
    width = MQRspec_getWidth(version);
    tmp = malloc((width * width) as size_t);
    mask = tmp as *mut libc::c_uchar;
    if mask as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut libc::c_uchar;
    }
    bestMask = 0 as *mut libc::c_void as *mut libc::c_uchar;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        score = 0 as libc::c_int;
        (Some(
            (*maskMakers___0.as_mut_ptr().offset(i as isize))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(width, frame as *const libc::c_uchar, mask);
        MMask_writeFormatInformation(version, width, mask, i, level);
        score = MMask_evaluateSymbol(width, mask);
        if score > maxScore {
            maxScore = score;
            free(bestMask as *mut libc::c_void);
            bestMask = mask;
            tmp___0 = malloc((width * width) as size_t);
            mask = tmp___0 as *mut libc::c_uchar;
            if mask as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                break;
            }
        }
        i += 1;
    }
    free(mask as *mut libc::c_void);
    return bestMask;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
