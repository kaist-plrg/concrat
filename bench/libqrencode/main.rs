use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strncat(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn abs(_: libc::c_int) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
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
pub struct __anonstruct_QRcode_929122250 {
    pub version: libc::c_int,
    pub width: libc::c_int,
    pub data: *mut libc::c_uchar,
}
pub type QRcode = __anonstruct_QRcode_929122250;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QRcode_List {
    pub code: *mut QRcode,
    pub next: *mut _QRcode_List,
}
pub type QRcode_List = _QRcode_List;
pub type imageType = libc::c_uint;
pub const ANSIUTF8i_TYPE: imageType = 13;
pub const UTF8i_TYPE: imageType = 12;
pub const ANSI256UTF8_TYPE: imageType = 11;
pub const ANSIUTF8_TYPE: imageType = 10;
pub const UTF8_TYPE: imageType = 9;
pub const ASCIIi_TYPE: imageType = 8;
pub const ASCII_TYPE: imageType = 7;
pub const ANSI256_TYPE: imageType = 6;
pub const ANSI_TYPE: imageType = 5;
pub const XPM_TYPE: imageType = 4;
pub const SVG_TYPE: imageType = 3;
pub const EPS_TYPE: imageType = 2;
pub const PNG32_TYPE: imageType = 1;
pub const PNG_TYPE: imageType = 0;
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
static mut casesensitive: libc::c_int = 1 as libc::c_int;
static mut eightbit: libc::c_int = 0 as libc::c_int;
static mut version: libc::c_int = 0 as libc::c_int;
static mut size: libc::c_int = 3 as libc::c_int;
static mut margin: libc::c_int = -(1 as libc::c_int);
static mut dpi: libc::c_int = 72 as libc::c_int;
static mut structured: libc::c_int = 0 as libc::c_int;
static mut rle: libc::c_int = 0 as libc::c_int;
static mut svg_path: libc::c_int = 0 as libc::c_int;
static mut micro: libc::c_int = 0 as libc::c_int;
static mut inline_svg: libc::c_int = 0 as libc::c_int;
static mut strict_versioning: libc::c_int = 0 as libc::c_int;
static mut level: QRecLevel = QR_ECLEVEL_L;
static mut hint: QRencodeMode = QR_MODE_8;
static mut fg_color: [libc::c_uchar; 4] = [
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
];
static mut bg_color: [libc::c_uchar; 4] = [
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
];
static mut verbose: libc::c_int = 0 as libc::c_int;
static mut image_type: imageType = PNG_TYPE;
static mut options: [option; 24] = unsafe {
    [
        {
            let mut init = option {
                name: b"help\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
                val: 'h' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"output\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
                val: 'o' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"read-from\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
                val: 'r' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"level\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
                val: 'l' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"size\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
                val: 's' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"symversion\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
                val: 'v' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"margin\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
                val: 'm' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"dpi\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
                val: 'd' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"type\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
                val: 't' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"structured\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
                val: 'S' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"kanji\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
                val: 'k' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"casesensitive\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
                val: 'c' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"ignorecase\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
                val: 'i' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"8bit\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
                val: '8' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"micro\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
                val: 'M' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"rle\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &rle as *const libc::c_int as *mut libc::c_int,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"svg-path\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &svg_path as *const libc::c_int as *mut libc::c_int,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"inline\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &inline_svg as *const libc::c_int as *mut libc::c_int,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"strict-version\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &strict_versioning as *const libc::c_int as *mut libc::c_int,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"foreground\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
                val: 'f' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"background\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
                val: 'b' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"version\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
                val: 'V' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"verbose\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &verbose as *const libc::c_int as *mut libc::c_int,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: 0 as *const libc::c_void as *mut libc::c_void
                    as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
    ]
};
static mut optstring: *mut libc::c_char = b"ho:r:l:s:v:m:d:t:Skci8MV\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn usage(
    mut help: libc::c_int,
    mut longopt: libc::c_int,
    mut status: libc::c_int,
) {
    let mut out: *mut FILE = 0 as *mut FILE;
    let mut tmp: *mut FILE = 0 as *mut FILE;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    if status != 0 {
        tmp = stderr;
    } else {
        tmp = stdout;
    }
    out = tmp;
    tmp___0 = QRcode_APIVersionString();
    fprintf(
        out,
        b"qrencode version %s\nCopyright (C) 2006-2017 Kentaro Fukuchi\n\0" as *const u8
            as *const libc::c_char,
        tmp___0,
    );
    if help != 0 {
        if longopt != 0 {
            fprintf(
                out,
                b"Usage: qrencode [-o FILENAME] [OPTION]... [STRING]\nEncode input data in a QR Code and save as a PNG or EPS image.\n\n  -h, --help   display the help message. -h displays only the help of short\n               options.\n\n  -o FILENAME, --output=FILENAME\n               write image to FILENAME. If '-' is specified, the result\n               will be output to standard output. If -S is given, structured\n               symbols are written to FILENAME-01.png, FILENAME-02.png, ...\n               (suffix is removed from FILENAME, if specified)\n\n  -r FILENAME, --read-from=FILENAME\n               read input data from FILENAME.\n\n  -s NUMBER, --size=NUMBER\n               specify module size in dots (pixels). (default=3)\n\n  -l {LMQH}, --level={LMQH}\n               specify error correction level from L (lowest) to H (highest).\n               (default=L)\n\n  -v NUMBER, --symversion=NUMBER\n               specify the minimum version of the symbol. See SYMBOL VERSIONS\n               for more information. (default=auto)\n\n  -m NUMBER, --margin=NUMBER\n               specify the width of the margins. (default=4 (2 for Micro QR)))\n\n  -d NUMBER, --dpi=NUMBER\n               specify the DPI of the generated PNG. (default=72)\n\n  -t {PNG,PNG32,EPS,SVG,XPM,ANSI,ANSI256,ASCII,ASCIIi,UTF8,UTF8i,ANSIUTF8,ANSIUTF8i,ANSI256UTF8},\n  --type={PNG,PNG32,EPS,SVG,XPM,ANSI,ANSI256,ASCII,ASCIIi,UTF8,UTF8i,ANSIUTF8,ANSIUTF8i,ANSI256UTF8}\n               specify the type of the generated image. (default=PNG)\n\n  -S, --structured\n               make structured symbols. Version must be specified with '-v'.\n\n  -k, --kanji  assume that the input text contains kanji (shift-jis).\n\n  -c, --casesensitive\n               encode lower-case alphabet characters in 8-bit mode. (default)\n\n  -i, --ignorecase\n               ignore case distinctions and use only upper-case characters.\n\n  -8, --8bit   encode entire data in 8-bit mode. -k, -c and -i will be ignored.\n\n  -M, --micro  encode in a Micro QR Code.\n\n      --rle    enable run-length encoding for SVG.\n\n      --svg-path\n               use single path to draw modules for SVG.\n\n      --inline only useful for SVG output, generates an SVG without the XML tag.\n\n      --foreground=RRGGBB[AA]\n      --background=RRGGBB[AA]\n               specify foreground/background color in hexadecimal notation.\n               6-digit (RGB) or 8-digit (RGBA) form are supported.\n               Color output support available only in PNG, EPS and SVG.\n\n      --strict-version\n               disable automatic version number adjustment. If the input data is\n               too large for the specified version, the program exits with the\n               code of 1.\n\n  -V, --version\n               display the version number and copyrights of the qrencode.\n\n      --verbose\n               display verbose information to stderr.\n\n  [STRING]     input data. If it is not specified, data will be taken from\n               standard input.\n\nSYMBOL VERSIONS\n               The symbol versions of QR Code range from Version 1 to Version\n               40. Each version has a different module configuration or number\n               of modules, ranging from Version 1 (21 x 21 modules) up to\n               Version 40 (177 x 177 modules). Each higher version number\n               comprises 4 additional modules per side by default. See\n               http://www.qrcode.com/en/about/version.html for a detailed\n               version list.\n\0"
                    as *const u8 as *const libc::c_char,
            );
        } else {
            fprintf(
                out,
                b"Usage: qrencode [-o FILENAME] [OPTION]... [STRING]\nEncode input data in a QR Code and save as a PNG or EPS image.\n\n  -h           display this message.\n  --help       display the usage of long options.\n  -o FILENAME  write image to FILENAME. If '-' is specified, the result\n               will be output to standard output. If -S is given, structured\n               symbols are written to FILENAME-01.png, FILENAME-02.png, ...\n               (suffix is removed from FILENAME, if specified)\n  -r FILENAME  read input data from FILENAME.\n  -s NUMBER    specify module size in dots (pixels). (default=3)\n  -l {LMQH}    specify error correction level from L (lowest) to H (highest).\n               (default=L)\n  -v NUMBER    specify the minimum version of the symbol. (default=auto)\n  -m NUMBER    specify the width of the margins. (default=4 (2 for Micro))\n  -d NUMBER    specify the DPI of the generated PNG. (default=72)\n  -t {PNG,PNG32,EPS,SVG,XPM,ANSI,ANSI256,ASCII,ASCIIi,UTF8,UTF8i,ANSIUTF8,ANSIUTF8i,ANSI256UTF8}\n               specify the type of the generated image. (default=PNG)\n  -S           make structured symbols. Version number must be specified with '-v'.\n  -k           assume that the input text contains kanji (shift-jis).\n  -c           encode lower-case alphabet characters in 8-bit mode. (default)\n  -i           ignore case distinctions and use only upper-case characters.\n  -8           encode entire data in 8-bit mode. -k, -c and -i will be ignored.\n  -M           encode in a Micro QR Code.\n  -V           display the version number and copyrights of the qrencode.\n  [STRING]     input data. If it is not specified, data will be taken from\n               standard input.\n\n  Try \"qrencode --help\" for more options.\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    }
}
unsafe extern "C" fn color_set(
    mut color: *mut libc::c_uchar,
    mut value: *const libc::c_char,
) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut col: [libc::c_uint; 4] = [0; 4];
    tmp = strlen(value);
    len = tmp as libc::c_int;
    if len == 6 as libc::c_int {
        count = sscanf(
            value,
            b"%02x%02x%02x%n\0" as *const u8 as *const libc::c_char,
            &mut *col.as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut libc::c_uint,
            &mut *col.as_mut_ptr().offset(1 as libc::c_int as isize)
                as *mut libc::c_uint,
            &mut *col.as_mut_ptr().offset(2 as libc::c_int as isize)
                as *mut libc::c_uint,
            &mut len as *mut libc::c_int,
        );
        if count < 3 as libc::c_int {
            return -(1 as libc::c_int)
        } else {
            if len != 6 as libc::c_int {
                return -(1 as libc::c_int);
            }
        }
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            *color.offset(i as isize) = col[i as usize] as libc::c_uchar;
            i += 1;
        }
        *color.offset(3 as libc::c_int as isize) = 255 as libc::c_int as libc::c_uchar;
    } else if len == 8 as libc::c_int {
        count = sscanf(
            value,
            b"%02x%02x%02x%02x%n\0" as *const u8 as *const libc::c_char,
            &mut *col.as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut libc::c_uint,
            &mut *col.as_mut_ptr().offset(1 as libc::c_int as isize)
                as *mut libc::c_uint,
            &mut *col.as_mut_ptr().offset(2 as libc::c_int as isize)
                as *mut libc::c_uint,
            &mut *col.as_mut_ptr().offset(3 as libc::c_int as isize)
                as *mut libc::c_uint,
            &mut len as *mut libc::c_int,
        );
        if count < 4 as libc::c_int {
            return -(1 as libc::c_int)
        } else {
            if len != 8 as libc::c_int {
                return -(1 as libc::c_int);
            }
        }
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            *color.offset(i as isize) = col[i as usize] as libc::c_uchar;
            i += 1;
        }
    } else {
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
static mut data_buffer: [libc::c_uchar; 14180] = [0; 14180];
unsafe extern "C" fn readFile(
    mut fp: *mut FILE,
    mut length: *mut libc::c_int,
) -> *mut libc::c_uchar {
    let mut ret: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = fread(
        data_buffer.as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int as size_t,
        14180 as libc::c_int as size_t,
        fp,
    );
    ret = tmp as libc::c_int;
    if ret == 0 as libc::c_int {
        fprintf(stderr, b"No input data.\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    tmp___0 = feof(fp);
    if tmp___0 == 0 as libc::c_int {
        fprintf(
            stderr,
            b"Input data is too large.\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    data_buffer[ret as usize] = '\u{0}' as i32 as libc::c_uchar;
    *length = ret;
    return data_buffer.as_mut_ptr();
}
unsafe extern "C" fn openFile(mut outfile: *const libc::c_char) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    if outfile as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fp = stdout;
    } else {
        let mut current_block_9: u64;
        if *outfile.offset(0 as libc::c_int as isize) as libc::c_int == 45 as libc::c_int
        {
            if *outfile.offset(1 as libc::c_int as isize) as libc::c_int
                == 0 as libc::c_int
            {
                fp = stdout;
                current_block_9 = 2968425633554183086;
            } else {
                current_block_9 = 15768395813763426840;
            }
        } else {
            current_block_9 = 15768395813763426840;
        }
        match current_block_9 {
            15768395813763426840 => {
                fp = fopen(outfile, b"wb\0" as *const u8 as *const libc::c_char);
                if fp as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                    fprintf(
                        stderr,
                        b"Failed to create file: %s\n\0" as *const u8
                            as *const libc::c_char,
                        outfile,
                    );
                    perror(0 as *mut libc::c_void as *const libc::c_char);
                    exit(1 as libc::c_int);
                }
            }
            _ => {}
        }
    }
    return fp;
}
unsafe extern "C" fn writePNG(
    mut qrcode: *const QRcode,
    mut outfile: *const libc::c_char,
    mut type_0: imageType,
) -> libc::c_int {
    fputs(
        b"PNG output is disabled at compile time. No output generated.\n\0" as *const u8
            as *const libc::c_char,
        stderr,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn writeEPS(
    mut qrcode: *const QRcode,
    mut outfile: *const libc::c_char,
) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut row: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut yy: libc::c_int = 0;
    let mut realwidth: libc::c_int = 0;
    fp = openFile(outfile);
    realwidth = ((*qrcode).width + margin * 2 as libc::c_int) * size;
    fprintf(
        fp,
        b"%%!PS-Adobe-2.0 EPSF-1.2\n%%%%BoundingBox: 0 0 %d %d\n%%%%Pages: 1 1\n%%%%EndComments\n\0"
            as *const u8 as *const libc::c_char,
        realwidth,
        realwidth,
    );
    fprintf(
        fp,
        b"/p { moveto 0 1 rlineto 1 0 rlineto 0 -1 rlineto fill } bind def\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(fp, b"gsave\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fp,
        b"%f %f %f setrgbcolor\n\0" as *const u8 as *const libc::c_char,
        (bg_color[0 as libc::c_int as usize] as libc::c_float
            / 255 as libc::c_int as libc::c_float) as libc::c_double,
        (bg_color[1 as libc::c_int as usize] as libc::c_float
            / 255 as libc::c_int as libc::c_float) as libc::c_double,
        (bg_color[2 as libc::c_int as usize] as libc::c_float
            / 255 as libc::c_int as libc::c_float) as libc::c_double,
    );
    fprintf(
        fp,
        b"%d %d scale\n\0" as *const u8 as *const libc::c_char,
        realwidth,
        realwidth,
    );
    fprintf(fp, b"0 0 p\ngrestore\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fp,
        b"%f %f %f setrgbcolor\n\0" as *const u8 as *const libc::c_char,
        (fg_color[0 as libc::c_int as usize] as libc::c_float
            / 255 as libc::c_int as libc::c_float) as libc::c_double,
        (fg_color[1 as libc::c_int as usize] as libc::c_float
            / 255 as libc::c_int as libc::c_float) as libc::c_double,
        (fg_color[2 as libc::c_int as usize] as libc::c_float
            / 255 as libc::c_int as libc::c_float) as libc::c_double,
    );
    fprintf(fp, b"%d %d scale\n\0" as *const u8 as *const libc::c_char, size, size);
    p = (*qrcode).data;
    y = 0 as libc::c_int;
    while y < (*qrcode).width {
        row = p.offset((y * (*qrcode).width) as isize);
        yy = margin + (*qrcode).width - y - 1 as libc::c_int;
        x = 0 as libc::c_int;
        while x < (*qrcode).width {
            if *row.offset(x as isize) as libc::c_int & 1 as libc::c_int != 0 {
                fprintf(
                    fp,
                    b"%d %d p \0" as *const u8 as *const libc::c_char,
                    margin + x,
                    yy,
                );
            }
            x += 1;
        }
        y += 1;
    }
    fprintf(fp, b"\n%%%%EOF\n\0" as *const u8 as *const libc::c_char);
    fclose(fp);
    return 0 as libc::c_int;
}
unsafe extern "C" fn writeSVG_drawModules(
    mut fp: *mut FILE,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut col: *const libc::c_char,
    mut opacity: libc::c_float,
) {
    if svg_path != 0 {
        fprintf(fp, b"M%d,%dh%d\0" as *const u8 as *const libc::c_char, x, y, width);
    } else if fg_color[3 as libc::c_int as usize] as libc::c_int != 255 as libc::c_int {
        fprintf(
            fp,
            b"\t\t\t<rect x=\"%d\" y=\"%d\" width=\"%d\" height=\"1\" fill=\"#%s\" fill-opacity=\"%f\"/>\n\0"
                as *const u8 as *const libc::c_char,
            x,
            y,
            width,
            col,
            opacity as libc::c_double,
        );
    } else {
        fprintf(
            fp,
            b"\t\t\t<rect x=\"%d\" y=\"%d\" width=\"%d\" height=\"1\" fill=\"#%s\"/>\n\0"
                as *const u8 as *const libc::c_char,
            x,
            y,
            width,
            col,
        );
    };
}
unsafe extern "C" fn writeSVG(
    mut qrcode: *const QRcode,
    mut outfile: *const libc::c_char,
) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut row: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x0: libc::c_int = 0;
    let mut pen: libc::c_int = 0;
    let mut symwidth: libc::c_int = 0;
    let mut realwidth: libc::c_int = 0;
    let mut scale: libc::c_float = 0.;
    let mut fg: [libc::c_char; 7] = [0; 7];
    let mut bg: [libc::c_char; 7] = [0; 7];
    let mut fg_opacity: libc::c_float = 0.;
    let mut bg_opacity: libc::c_float = 0.;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    fp = openFile(outfile);
    scale = (dpi as libc::c_double * (100.0f64 / 2.54f64) / 100.0f64) as libc::c_float;
    symwidth = (*qrcode).width + margin * 2 as libc::c_int;
    realwidth = symwidth * size;
    snprintf(
        fg.as_mut_ptr(),
        7 as libc::c_int as size_t,
        b"%02x%02x%02x\0" as *const u8 as *const libc::c_char,
        fg_color[0 as libc::c_int as usize] as libc::c_int,
        fg_color[1 as libc::c_int as usize] as libc::c_int,
        fg_color[2 as libc::c_int as usize] as libc::c_int,
    );
    snprintf(
        bg.as_mut_ptr(),
        7 as libc::c_int as size_t,
        b"%02x%02x%02x\0" as *const u8 as *const libc::c_char,
        bg_color[0 as libc::c_int as usize] as libc::c_int,
        bg_color[1 as libc::c_int as usize] as libc::c_int,
        bg_color[2 as libc::c_int as usize] as libc::c_int,
    );
    fg_opacity = fg_color[3 as libc::c_int as usize] as libc::c_float
        / 255 as libc::c_int as libc::c_float;
    bg_opacity = bg_color[3 as libc::c_int as usize] as libc::c_float
        / 255 as libc::c_int as libc::c_float;
    if inline_svg == 0 {
        fputs(
            b"<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n\0"
                as *const u8 as *const libc::c_char,
            fp,
        );
    }
    tmp = QRcode_APIVersionString();
    fprintf(
        fp,
        b"<!-- Created with qrencode %s (https://fukuchi.org/works/qrencode/index.html) -->\n\0"
            as *const u8 as *const libc::c_char,
        tmp,
    );
    fprintf(
        fp,
        b"<svg width=\"%.2fcm\" height=\"%.2fcm\" viewBox=\"0 0 %d %d\" preserveAspectRatio=\"none\" version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\">\n\0"
            as *const u8 as *const libc::c_char,
        (realwidth as libc::c_float / scale) as libc::c_double,
        (realwidth as libc::c_float / scale) as libc::c_double,
        symwidth,
        symwidth,
    );
    fputs(b"\t<g id=\"QRcode\">\n\0" as *const u8 as *const libc::c_char, fp);
    if bg_color[3 as libc::c_int as usize] as libc::c_int != 255 as libc::c_int {
        fprintf(
            fp,
            b"\t\t<rect x=\"0\" y=\"0\" width=\"%d\" height=\"%d\" fill=\"#%s\" fill-opacity=\"%f\"/>\n\0"
                as *const u8 as *const libc::c_char,
            symwidth,
            symwidth,
            bg.as_mut_ptr(),
            bg_opacity as libc::c_double,
        );
    } else {
        fprintf(
            fp,
            b"\t\t<rect x=\"0\" y=\"0\" width=\"%d\" height=\"%d\" fill=\"#%s\"/>\n\0"
                as *const u8 as *const libc::c_char,
            symwidth,
            symwidth,
            bg.as_mut_ptr(),
        );
    }
    if svg_path != 0 {
        if fg_color[3 as libc::c_int as usize] as libc::c_int != 255 as libc::c_int {
            fprintf(
                fp,
                b"\t\t<path style=\"stroke:#%s;stroke-opacity:%f\" transform=\"translate(%d,%d.5)\" d=\"\0"
                    as *const u8 as *const libc::c_char,
                fg.as_mut_ptr(),
                fg_opacity as libc::c_double,
                margin,
                margin,
            );
        } else {
            fprintf(
                fp,
                b"\t\t<path style=\"stroke:#%s\" transform=\"translate(%d,%d.5)\" d=\"\0"
                    as *const u8 as *const libc::c_char,
                fg.as_mut_ptr(),
                margin,
                margin,
            );
        }
    } else {
        fprintf(
            fp,
            b"\t\t<g id=\"Pattern\" transform=\"translate(%d,%d)\">\n\0" as *const u8
                as *const libc::c_char,
            margin,
            margin,
        );
    }
    p = (*qrcode).data;
    y = 0 as libc::c_int;
    while y < (*qrcode).width {
        row = p.offset((y * (*qrcode).width) as isize);
        if rle == 0 {
            x = 0 as libc::c_int;
            while x < (*qrcode).width {
                if *row.offset(x as isize) as libc::c_int & 1 as libc::c_int != 0 {
                    writeSVG_drawModules(
                        fp,
                        x,
                        y,
                        1 as libc::c_int,
                        fg.as_mut_ptr() as *const libc::c_char,
                        fg_opacity,
                    );
                }
                x += 1;
            }
        } else {
            pen = 0 as libc::c_int;
            x0 = 0 as libc::c_int;
            x = 0 as libc::c_int;
            while x < (*qrcode).width {
                if pen == 0 {
                    pen = *row.offset(x as isize) as libc::c_int & 1 as libc::c_int;
                    x0 = x;
                } else if *row.offset(x as isize) as libc::c_int & 1 as libc::c_int == 0
                    {
                    writeSVG_drawModules(
                        fp,
                        x0,
                        y,
                        x - x0,
                        fg.as_mut_ptr() as *const libc::c_char,
                        fg_opacity,
                    );
                    pen = 0 as libc::c_int;
                }
                x += 1;
            }
            if pen != 0 {
                writeSVG_drawModules(
                    fp,
                    x0,
                    y,
                    (*qrcode).width - x0,
                    fg.as_mut_ptr() as *const libc::c_char,
                    fg_opacity,
                );
            }
        }
        y += 1;
    }
    if svg_path != 0 {
        fputs(b"\"/>\n\0" as *const u8 as *const libc::c_char, fp);
    } else {
        fputs(b"\t\t</g>\n\0" as *const u8 as *const libc::c_char, fp);
    }
    fputs(b"\t</g>\n\0" as *const u8 as *const libc::c_char, fp);
    fputs(b"</svg>\n\0" as *const u8 as *const libc::c_char, fp);
    fclose(fp);
    return 0 as libc::c_int;
}
unsafe extern "C" fn writeXPM(
    mut qrcode: *const QRcode,
    mut outfile: *const libc::c_char,
) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut x: libc::c_int = 0;
    let mut xx: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut yy: libc::c_int = 0;
    let mut realwidth: libc::c_int = 0;
    let mut realmargin: libc::c_int = 0;
    let mut row: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fg: [libc::c_char; 7] = [0; 7];
    let mut bg: [libc::c_char; 7] = [0; 7];
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    fp = openFile(outfile);
    realwidth = ((*qrcode).width + margin * 2 as libc::c_int) * size;
    realmargin = margin * size;
    tmp = malloc((realwidth as size_t).wrapping_add(1 as libc::c_ulong));
    row = tmp as *mut libc::c_char;
    if row.is_null() {
        fprintf(
            stderr,
            b"Failed to allocate memory.\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    snprintf(
        fg.as_mut_ptr(),
        7 as libc::c_int as size_t,
        b"%02x%02x%02x\0" as *const u8 as *const libc::c_char,
        fg_color[0 as libc::c_int as usize] as libc::c_int,
        fg_color[1 as libc::c_int as usize] as libc::c_int,
        fg_color[2 as libc::c_int as usize] as libc::c_int,
    );
    snprintf(
        bg.as_mut_ptr(),
        7 as libc::c_int as size_t,
        b"%02x%02x%02x\0" as *const u8 as *const libc::c_char,
        bg_color[0 as libc::c_int as usize] as libc::c_int,
        bg_color[1 as libc::c_int as usize] as libc::c_int,
        bg_color[2 as libc::c_int as usize] as libc::c_int,
    );
    fputs(b"/* XPM */\n\0" as *const u8 as *const libc::c_char, fp);
    fputs(
        b"static const char *const qrcode_xpm[] = {\n\0" as *const u8
            as *const libc::c_char,
        fp,
    );
    fputs(
        b"/* width height ncolors chars_per_pixel */\n\0" as *const u8
            as *const libc::c_char,
        fp,
    );
    fprintf(
        fp,
        b"\"%d %d 2 1\",\n\0" as *const u8 as *const libc::c_char,
        realwidth,
        realwidth,
    );
    fputs(b"/* colors */\n\0" as *const u8 as *const libc::c_char, fp);
    fprintf(
        fp,
        b"\"F c #%s\",\n\0" as *const u8 as *const libc::c_char,
        fg.as_mut_ptr(),
    );
    fprintf(
        fp,
        b"\"B c #%s\",\n\0" as *const u8 as *const libc::c_char,
        bg.as_mut_ptr(),
    );
    fputs(b"/* pixels */\n\0" as *const u8 as *const libc::c_char, fp);
    memset(row as *mut libc::c_void, 'B' as i32, realwidth as size_t);
    *row.offset(realwidth as isize) = '\u{0}' as i32 as libc::c_char;
    y = 0 as libc::c_int;
    while y < realmargin {
        fprintf(fp, b"\"%s\",\n\0" as *const u8 as *const libc::c_char, row);
        y += 1;
    }
    p = (*qrcode).data;
    y = 0 as libc::c_int;
    while y < (*qrcode).width {
        yy = 0 as libc::c_int;
        while yy < size {
            fputs(b"\"\0" as *const u8 as *const libc::c_char, fp);
            x = 0 as libc::c_int;
            while x < margin {
                xx = 0 as libc::c_int;
                while xx < size {
                    fputs(b"B\0" as *const u8 as *const libc::c_char, fp);
                    xx += 1;
                }
                x += 1;
            }
            x = 0 as libc::c_int;
            while x < (*qrcode).width {
                xx = 0 as libc::c_int;
                while xx < size {
                    if *p.offset((y * (*qrcode).width + x) as isize) as libc::c_int
                        & 1 as libc::c_int != 0
                    {
                        fputs(b"F\0" as *const u8 as *const libc::c_char, fp);
                    } else {
                        fputs(b"B\0" as *const u8 as *const libc::c_char, fp);
                    }
                    xx += 1;
                }
                x += 1;
            }
            x = 0 as libc::c_int;
            while x < margin {
                xx = 0 as libc::c_int;
                while xx < size {
                    fputs(b"B\0" as *const u8 as *const libc::c_char, fp);
                    xx += 1;
                }
                x += 1;
            }
            fputs(b"\",\n\0" as *const u8 as *const libc::c_char, fp);
            yy += 1;
        }
        y += 1;
    }
    y = 0 as libc::c_int;
    while y < realmargin {
        if y < size - 1 as libc::c_int {
            tmp___0 = b",\0" as *const u8 as *const libc::c_char;
        } else {
            tmp___0 = b"};\0" as *const u8 as *const libc::c_char;
        }
        fprintf(fp, b"\"%s\"%s\n\0" as *const u8 as *const libc::c_char, row, tmp___0);
        y += 1;
    }
    free(row as *mut libc::c_void);
    fclose(fp);
    return 0 as libc::c_int;
}
unsafe extern "C" fn writeANSI_margin(
    mut fp: *mut FILE,
    mut realwidth: libc::c_int,
    mut buffer: *mut libc::c_char,
    mut white: *const libc::c_char,
    mut white_s: libc::c_int,
) {
    let mut y: libc::c_int = 0;
    strncpy(buffer, white, white_s as size_t);
    memset(
        buffer.offset(white_s as isize) as *mut libc::c_void,
        ' ' as i32,
        (realwidth as size_t).wrapping_mul(2 as libc::c_ulong),
    );
    strcpy(
        buffer.offset(white_s as isize).offset((realwidth * 2 as libc::c_int) as isize),
        b"\x1B[0m\n\0" as *const u8 as *const libc::c_char,
    );
    y = 0 as libc::c_int;
    while y < margin {
        fputs(buffer as *const libc::c_char, fp);
        y += 1;
    }
}
unsafe extern "C" fn writeANSI(
    mut qrcode: *const QRcode,
    mut outfile: *const libc::c_char,
) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut row: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut realwidth: libc::c_int = 0;
    let mut last: libc::c_int = 0;
    let mut white: *const libc::c_char = 0 as *const libc::c_char;
    let mut black: *const libc::c_char = 0 as *const libc::c_char;
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut white_s: libc::c_int = 0;
    let mut black_s: libc::c_int = 0;
    let mut buffer_s: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    if image_type as libc::c_uint == 6 as libc::c_uint {
        white = b"\x1B[48;5;231m\0" as *const u8 as *const libc::c_char;
        white_s = 11 as libc::c_int;
        black = b"\x1B[48;5;16m\0" as *const u8 as *const libc::c_char;
        black_s = 10 as libc::c_int;
    } else {
        white = b"\x1B[47m\0" as *const u8 as *const libc::c_char;
        white_s = 5 as libc::c_int;
        black = b"\x1B[40m\0" as *const u8 as *const libc::c_char;
        black_s = 5 as libc::c_int;
    }
    size = 1 as libc::c_int;
    fp = openFile(outfile);
    realwidth = ((*qrcode).width + margin * 2 as libc::c_int) * size;
    buffer_s = realwidth * white_s * 2 as libc::c_int;
    tmp = malloc(buffer_s as size_t);
    buffer = tmp as *mut libc::c_char;
    if buffer as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"Failed to allocate memory.\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    writeANSI_margin(fp, realwidth, buffer, white, white_s);
    p = (*qrcode).data;
    y = 0 as libc::c_int;
    while y < (*qrcode).width {
        row = p.offset((y * (*qrcode).width) as isize);
        memset(buffer as *mut libc::c_void, 0 as libc::c_int, buffer_s as size_t);
        strncpy(buffer, white, white_s as size_t);
        x = 0 as libc::c_int;
        while x < margin {
            strncat(
                buffer,
                b"  \0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as size_t,
            );
            x += 1;
        }
        last = 0 as libc::c_int;
        x = 0 as libc::c_int;
        while x < (*qrcode).width {
            if *row.offset(x as isize) as libc::c_int & 1 as libc::c_int != 0 {
                if last != 1 as libc::c_int {
                    strncat(buffer, black, black_s as size_t);
                    last = 1 as libc::c_int;
                }
            } else if last != 0 as libc::c_int {
                strncat(buffer, white, white_s as size_t);
                last = 0 as libc::c_int;
            }
            strncat(
                buffer,
                b"  \0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as size_t,
            );
            x += 1;
        }
        if last != 0 as libc::c_int {
            strncat(buffer, white, white_s as size_t);
        }
        x = 0 as libc::c_int;
        while x < margin {
            strncat(
                buffer,
                b"  \0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as size_t,
            );
            x += 1;
        }
        strncat(
            buffer,
            b"\x1B[0m\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        );
        fputs(buffer as *const libc::c_char, fp);
        y += 1;
    }
    writeANSI_margin(fp, realwidth, buffer, white, white_s);
    fclose(fp);
    free(buffer as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn writeUTF8_margin(
    mut fp: *mut FILE,
    mut realwidth: libc::c_int,
    mut white: *const libc::c_char,
    mut reset: *const libc::c_char,
    mut full: *const libc::c_char,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < margin / 2 as libc::c_int {
        fputs(white, fp);
        x = 0 as libc::c_int;
        while x < realwidth {
            fputs(full, fp);
            x += 1;
        }
        fputs(reset, fp);
        fputc('\n' as i32, fp);
        y += 1;
    }
}
unsafe extern "C" fn writeUTF8(
    mut qrcode: *const QRcode,
    mut outfile: *const libc::c_char,
    mut use_ansi: libc::c_int,
    mut invert: libc::c_int,
) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut realwidth: libc::c_int = 0;
    let mut white: *const libc::c_char = 0 as *const libc::c_char;
    let mut reset: *const libc::c_char = 0 as *const libc::c_char;
    let mut empty: *const libc::c_char = 0 as *const libc::c_char;
    let mut lowhalf: *const libc::c_char = 0 as *const libc::c_char;
    let mut uphalf: *const libc::c_char = 0 as *const libc::c_char;
    let mut full: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut row1: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut row2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    empty = b" \0" as *const u8 as *const libc::c_char;
    lowhalf = b"\xE2\x96\x84\0" as *const u8 as *const libc::c_char;
    uphalf = b"\xE2\x96\x80\0" as *const u8 as *const libc::c_char;
    full = b"\xE2\x96\x88\0" as *const u8 as *const libc::c_char;
    if invert != 0 {
        tmp = empty;
        empty = full;
        full = tmp;
        tmp = lowhalf;
        lowhalf = uphalf;
        uphalf = tmp;
    }
    if use_ansi != 0 {
        if use_ansi == 2 as libc::c_int {
            white = b"\x1B[38;5;231m\x1B[48;5;16m\0" as *const u8 as *const libc::c_char;
        } else {
            white = b"\x1B[40;37;1m\0" as *const u8 as *const libc::c_char;
        }
        reset = b"\x1B[0m\0" as *const u8 as *const libc::c_char;
    } else {
        white = b"\0" as *const u8 as *const libc::c_char;
        reset = b"\0" as *const u8 as *const libc::c_char;
    }
    fp = openFile(outfile);
    realwidth = (*qrcode).width + margin * 2 as libc::c_int;
    writeUTF8_margin(fp, realwidth, white, reset, full);
    y = 0 as libc::c_int;
    while y < (*qrcode).width {
        row1 = ((*qrcode).data).offset((y * (*qrcode).width) as isize);
        row2 = row1.offset((*qrcode).width as isize);
        fputs(white, fp);
        x = 0 as libc::c_int;
        while x < margin {
            fputs(full, fp);
            x += 1;
        }
        x = 0 as libc::c_int;
        while x < (*qrcode).width {
            if *row1.offset(x as isize) as libc::c_int & 1 as libc::c_int != 0 {
                if y < (*qrcode).width - 1 as libc::c_int {
                    if *row2.offset(x as isize) as libc::c_int & 1 as libc::c_int != 0 {
                        fputs(empty, fp);
                    } else {
                        fputs(lowhalf, fp);
                    }
                } else {
                    fputs(lowhalf, fp);
                }
            } else if y < (*qrcode).width - 1 as libc::c_int {
                if *row2.offset(x as isize) as libc::c_int & 1 as libc::c_int != 0 {
                    fputs(uphalf, fp);
                } else {
                    fputs(full, fp);
                }
            } else {
                fputs(full, fp);
            }
            x += 1;
        }
        x = 0 as libc::c_int;
        while x < margin {
            fputs(full, fp);
            x += 1;
        }
        fputs(reset, fp);
        fputc('\n' as i32, fp);
        y += 2 as libc::c_int;
    }
    writeUTF8_margin(fp, realwidth, white, reset, full);
    fclose(fp);
    return 0 as libc::c_int;
}
unsafe extern "C" fn writeASCII_margin(
    mut fp: *mut FILE,
    mut realwidth: libc::c_int,
    mut buffer: *mut libc::c_char,
    mut invert: libc::c_int,
) {
    let mut y: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    h = margin;
    if invert != 0 {
        tmp = '#' as i32;
    } else {
        tmp = ' ' as i32;
    }
    memset(buffer as *mut libc::c_void, tmp, realwidth as size_t);
    *buffer.offset(realwidth as isize) = '\n' as i32 as libc::c_char;
    *buffer
        .offset(
            (realwidth + 1 as libc::c_int) as isize,
        ) = '\u{0}' as i32 as libc::c_char;
    y = 0 as libc::c_int;
    while y < h {
        fputs(buffer as *const libc::c_char, fp);
        y += 1;
    }
}
unsafe extern "C" fn writeASCII(
    mut qrcode: *const QRcode,
    mut outfile: *const libc::c_char,
    mut invert: libc::c_int,
) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut row: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut realwidth: libc::c_int = 0;
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buffer_s: libc::c_int = 0;
    let mut black: libc::c_char = 0;
    let mut white: libc::c_char = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    black = '#' as i32 as libc::c_char;
    white = ' ' as i32 as libc::c_char;
    if invert != 0 {
        black = ' ' as i32 as libc::c_char;
        white = '#' as i32 as libc::c_char;
    }
    size = 1 as libc::c_int;
    fp = openFile(outfile);
    realwidth = ((*qrcode).width + margin * 2 as libc::c_int) * 2 as libc::c_int;
    buffer_s = realwidth + 2 as libc::c_int;
    tmp = malloc(buffer_s as size_t);
    buffer = tmp as *mut libc::c_char;
    if buffer as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"Failed to allocate memory.\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    writeASCII_margin(fp, realwidth, buffer, invert);
    y = 0 as libc::c_int;
    while y < (*qrcode).width {
        row = ((*qrcode).data).offset((y * (*qrcode).width) as isize);
        p = buffer;
        memset(
            p as *mut libc::c_void,
            white as libc::c_int,
            (margin as size_t).wrapping_mul(2 as libc::c_ulong),
        );
        p = p.offset((margin * 2 as libc::c_int) as isize);
        x = 0 as libc::c_int;
        while x < (*qrcode).width {
            if *row.offset(x as isize) as libc::c_int & 1 as libc::c_int != 0 {
                tmp___0 = p;
                p = p.offset(1);
                *tmp___0 = black;
                tmp___1 = p;
                p = p.offset(1);
                *tmp___1 = black;
            } else {
                tmp___2 = p;
                p = p.offset(1);
                *tmp___2 = white;
                tmp___3 = p;
                p = p.offset(1);
                *tmp___3 = white;
            }
            x += 1;
        }
        memset(
            p as *mut libc::c_void,
            white as libc::c_int,
            (margin as size_t).wrapping_mul(2 as libc::c_ulong),
        );
        p = p.offset((margin * 2 as libc::c_int) as isize);
        tmp___4 = p;
        p = p.offset(1);
        *tmp___4 = '\n' as i32 as libc::c_char;
        tmp___5 = p;
        p = p.offset(1);
        *tmp___5 = '\u{0}' as i32 as libc::c_char;
        fputs(buffer as *const libc::c_char, fp);
        y += 1;
    }
    writeASCII_margin(fp, realwidth, buffer, invert);
    fclose(fp);
    free(buffer as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn encode(
    mut intext: *const libc::c_uchar,
    mut length: libc::c_int,
) -> *mut QRcode {
    let mut code: *mut QRcode = 0 as *mut QRcode;
    if micro != 0 {
        if eightbit != 0 {
            code = QRcode_encodeDataMQR(length, intext, version, level);
        } else {
            code = QRcode_encodeStringMQR(
                intext as *mut libc::c_char as *const libc::c_char,
                version,
                level,
                hint,
                casesensitive,
            );
        }
    } else if eightbit != 0 {
        code = QRcode_encodeData(length, intext, version, level);
    } else {
        code = QRcode_encodeString(
            intext as *mut libc::c_char as *const libc::c_char,
            version,
            level,
            hint,
            casesensitive,
        );
    }
    return code;
}
unsafe extern "C" fn qrencode(
    mut intext: *const libc::c_uchar,
    mut length: libc::c_int,
    mut outfile: *const libc::c_char,
) {
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    qrcode = encode(intext, length);
    if qrcode as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp = __errno_location();
        if *tmp == 34 as libc::c_int {
            fprintf(
                stderr,
                b"Failed to encode the input data: Input data too large\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            perror(
                b"Failed to encode the input data\0" as *const u8 as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    }
    if strict_versioning != 0 {
        if version > 0 as libc::c_int {
            if (*qrcode).version != version {
                fprintf(
                    stderr,
                    b"Failed to encode the input data: Input data too large\n\0"
                        as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
        }
    }
    if verbose != 0 {
        if outfile as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            tmp___0 = outfile;
        } else {
            tmp___0 = b"(stdout)\0" as *const u8 as *const libc::c_char;
        }
        fprintf(
            stderr,
            b"File: %s, Version: %d\n\0" as *const u8 as *const libc::c_char,
            tmp___0,
            (*qrcode).version,
        );
    }
    match image_type as libc::c_uint {
        1 | 0 => {
            writePNG(qrcode as *const QRcode, outfile, image_type);
        }
        2 => {
            writeEPS(qrcode as *const QRcode, outfile);
        }
        3 => {
            writeSVG(qrcode as *const QRcode, outfile);
        }
        4 => {
            writeXPM(qrcode as *const QRcode, outfile);
        }
        6 | 5 => {
            writeANSI(qrcode as *const QRcode, outfile);
        }
        8 => {
            writeASCII(qrcode as *const QRcode, outfile, 1 as libc::c_int);
        }
        7 => {
            writeASCII(qrcode as *const QRcode, outfile, 0 as libc::c_int);
        }
        9 => {
            writeUTF8(
                qrcode as *const QRcode,
                outfile,
                0 as libc::c_int,
                0 as libc::c_int,
            );
        }
        10 => {
            writeUTF8(
                qrcode as *const QRcode,
                outfile,
                1 as libc::c_int,
                0 as libc::c_int,
            );
        }
        11 => {
            writeUTF8(
                qrcode as *const QRcode,
                outfile,
                2 as libc::c_int,
                0 as libc::c_int,
            );
        }
        12 => {
            writeUTF8(
                qrcode as *const QRcode,
                outfile,
                0 as libc::c_int,
                1 as libc::c_int,
            );
        }
        13 => {
            writeUTF8(
                qrcode as *const QRcode,
                outfile,
                1 as libc::c_int,
                1 as libc::c_int,
            );
        }
        _ => {
            fprintf(
                stderr,
                b"Unknown image type.\n\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    QRcode_free(qrcode);
}
unsafe extern "C" fn encodeStructured(
    mut intext: *const libc::c_uchar,
    mut length: libc::c_int,
) -> *mut QRcode_List {
    let mut list: *mut QRcode_List = 0 as *mut QRcode_List;
    if eightbit != 0 {
        list = QRcode_encodeDataStructured(length, intext, version, level);
    } else {
        list = QRcode_encodeStringStructured(
            intext as *mut libc::c_char as *const libc::c_char,
            version,
            level,
            hint,
            casesensitive,
        );
    }
    return list;
}
unsafe extern "C" fn qrencodeStructured(
    mut intext: *const libc::c_uchar,
    mut length: libc::c_int,
    mut outfile: *const libc::c_char,
) {
    let mut qrlist: *mut QRcode_List = 0 as *mut QRcode_List;
    let mut p: *mut QRcode_List = 0 as *mut QRcode_List;
    let mut filename: [libc::c_char; 4096] = [0; 4096];
    let mut base: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut suffix: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut type_suffix: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut suffix_size: size_t = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    suffix = 0 as *mut libc::c_void as *mut libc::c_char;
    i = 1 as libc::c_int;
    match image_type as libc::c_uint {
        0 => {
            type_suffix = b".png\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            type_suffix = b".eps\0" as *const u8 as *const libc::c_char;
        }
        3 => {
            type_suffix = b".svg\0" as *const u8 as *const libc::c_char;
        }
        4 => {
            type_suffix = b".xpm\0" as *const u8 as *const libc::c_char;
        }
        13 | 12 | 10 | 9 | 7 | 6 | 5 => {
            type_suffix = b".txt\0" as *const u8 as *const libc::c_char;
        }
        _ => {
            fprintf(
                stderr,
                b"Unknown image type.\n\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    if outfile as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"An output filename must be specified to store the structured images.\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    base = strdup(outfile);
    if base as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"Failed to allocate memory.\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    suffix_size = strlen(type_suffix);
    tmp___1 = strlen(base as *const libc::c_char);
    if tmp___1 > suffix_size {
        tmp = strlen(base as *const libc::c_char);
        q = base.offset(tmp as isize).offset(-(suffix_size as isize));
        tmp___0 = strcasecmp(type_suffix, q as *const libc::c_char);
        if tmp___0 == 0 as libc::c_int {
            suffix = strdup(q as *const libc::c_char);
            *q = '\u{0}' as i32 as libc::c_char;
        }
    }
    qrlist = encodeStructured(intext, length);
    if qrlist as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___2 = __errno_location();
        if *tmp___2 == 34 as libc::c_int {
            fprintf(
                stderr,
                b"Failed to encode the input data: Input data too large\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            perror(
                b"Failed to encode the input data\0" as *const u8 as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    }
    p = qrlist;
    while p as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        if (*p).code as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            fprintf(
                stderr,
                b"Failed to encode the input data.\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        if !suffix.is_null() {
            snprintf(
                filename.as_mut_ptr(),
                4096 as libc::c_int as size_t,
                b"%s-%02d%s\0" as *const u8 as *const libc::c_char,
                base,
                i,
                suffix,
            );
        } else {
            snprintf(
                filename.as_mut_ptr(),
                4096 as libc::c_int as size_t,
                b"%s-%02d\0" as *const u8 as *const libc::c_char,
                base,
                i,
            );
        }
        if verbose != 0 {
            fprintf(
                stderr,
                b"File: %s, Version: %d\n\0" as *const u8 as *const libc::c_char,
                filename.as_mut_ptr(),
                (*(*p).code).version,
            );
        }
        match image_type as libc::c_uint {
            1 | 0 => {
                writePNG(
                    (*p).code as *const QRcode,
                    filename.as_mut_ptr() as *const libc::c_char,
                    image_type,
                );
            }
            2 => {
                writeEPS(
                    (*p).code as *const QRcode,
                    filename.as_mut_ptr() as *const libc::c_char,
                );
            }
            3 => {
                writeSVG(
                    (*p).code as *const QRcode,
                    filename.as_mut_ptr() as *const libc::c_char,
                );
            }
            4 => {
                writeXPM(
                    (*p).code as *const QRcode,
                    filename.as_mut_ptr() as *const libc::c_char,
                );
            }
            6 | 5 => {
                writeANSI(
                    (*p).code as *const QRcode,
                    filename.as_mut_ptr() as *const libc::c_char,
                );
            }
            8 => {
                writeASCII(
                    (*p).code as *const QRcode,
                    filename.as_mut_ptr() as *const libc::c_char,
                    1 as libc::c_int,
                );
            }
            7 => {
                writeASCII(
                    (*p).code as *const QRcode,
                    filename.as_mut_ptr() as *const libc::c_char,
                    0 as libc::c_int,
                );
            }
            9 => {
                writeUTF8(
                    (*p).code as *const QRcode,
                    filename.as_mut_ptr() as *const libc::c_char,
                    0 as libc::c_int,
                    0 as libc::c_int,
                );
            }
            10 => {
                writeUTF8(
                    (*p).code as *const QRcode,
                    filename.as_mut_ptr() as *const libc::c_char,
                    0 as libc::c_int,
                    0 as libc::c_int,
                );
            }
            11 => {
                writeUTF8(
                    (*p).code as *const QRcode,
                    filename.as_mut_ptr() as *const libc::c_char,
                    0 as libc::c_int,
                    0 as libc::c_int,
                );
            }
            12 => {
                writeUTF8(
                    (*p).code as *const QRcode,
                    filename.as_mut_ptr() as *const libc::c_char,
                    0 as libc::c_int,
                    1 as libc::c_int,
                );
            }
            13 => {
                writeUTF8(
                    (*p).code as *const QRcode,
                    filename.as_mut_ptr() as *const libc::c_char,
                    0 as libc::c_int,
                    1 as libc::c_int,
                );
            }
            _ => {
                fprintf(
                    stderr,
                    b"Unknown image type.\n\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
        }
        i += 1;
        p = (*p).next;
    }
    free(base as *mut libc::c_void);
    if !suffix.is_null() {
        free(suffix as *mut libc::c_void);
    }
    QRcode_List_free(qrlist);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut opt: libc::c_int = 0;
    let mut lindex: libc::c_int = 0;
    let mut outfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut infile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut intext: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut length: libc::c_int = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: libc::c_int = 0;
    let mut tmp___13: libc::c_int = 0;
    let mut tmp___14: libc::c_int = 0;
    let mut tmp___15: size_t = 0;
    let mut tmp___16: *mut FILE = 0 as *mut FILE;
    lindex = -(1 as libc::c_int);
    outfile = 0 as *mut libc::c_void as *mut libc::c_char;
    infile = 0 as *mut libc::c_void as *mut libc::c_char;
    intext = 0 as *mut libc::c_void as *mut libc::c_uchar;
    length = 0 as libc::c_int;
    loop {
        opt = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            optstring as *const libc::c_char,
            options.as_ptr(),
            &mut lindex,
        );
        if !(opt != -(1 as libc::c_int)) {
            break;
        }
        match opt {
            104 => {
                if lindex == 0 as libc::c_int {
                    usage(1 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int);
                } else {
                    usage(1 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int);
                }
                exit(0 as libc::c_int);
            }
            111 => {
                outfile = optarg;
            }
            114 => {
                infile = optarg;
            }
            115 => {
                size = atoi(optarg as *const libc::c_char);
                if size <= 0 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Invalid size: %d\n\0" as *const u8 as *const libc::c_char,
                        size,
                    );
                    exit(1 as libc::c_int);
                }
            }
            118 => {
                version = atoi(optarg as *const libc::c_char);
                if version < 0 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Invalid version: %d\n\0" as *const u8 as *const libc::c_char,
                        version,
                    );
                    exit(1 as libc::c_int);
                }
            }
            108 => {
                match *optarg as libc::c_int {
                    76 | 108 => {
                        level = QR_ECLEVEL_L;
                    }
                    77 | 109 => {
                        level = QR_ECLEVEL_M;
                    }
                    81 | 113 => {
                        level = QR_ECLEVEL_Q;
                    }
                    72 | 104 => {
                        level = QR_ECLEVEL_H;
                    }
                    _ => {
                        fprintf(
                            stderr,
                            b"Invalid level: %s\n\0" as *const u8 as *const libc::c_char,
                            optarg,
                        );
                        exit(1 as libc::c_int);
                    }
                }
            }
            109 => {
                margin = atoi(optarg as *const libc::c_char);
                if margin < 0 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Invalid margin: %d\n\0" as *const u8 as *const libc::c_char,
                        margin,
                    );
                    exit(1 as libc::c_int);
                }
            }
            100 => {
                dpi = atoi(optarg as *const libc::c_char);
                if dpi < 0 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Invalid DPI: %d\n\0" as *const u8 as *const libc::c_char,
                        dpi,
                    );
                    exit(1 as libc::c_int);
                }
            }
            116 => {
                tmp___12 = strcasecmp(
                    optarg as *const libc::c_char,
                    b"png32\0" as *const u8 as *const libc::c_char,
                );
                if tmp___12 == 0 as libc::c_int {
                    image_type = PNG32_TYPE;
                } else {
                    tmp___11 = strcasecmp(
                        optarg as *const libc::c_char,
                        b"png\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___11 == 0 as libc::c_int {
                        image_type = PNG_TYPE;
                    } else {
                        tmp___10 = strcasecmp(
                            optarg as *const libc::c_char,
                            b"eps\0" as *const u8 as *const libc::c_char,
                        );
                        if tmp___10 == 0 as libc::c_int {
                            image_type = EPS_TYPE;
                        } else {
                            tmp___9 = strcasecmp(
                                optarg as *const libc::c_char,
                                b"svg\0" as *const u8 as *const libc::c_char,
                            );
                            if tmp___9 == 0 as libc::c_int {
                                image_type = SVG_TYPE;
                            } else {
                                tmp___8 = strcasecmp(
                                    optarg as *const libc::c_char,
                                    b"xpm\0" as *const u8 as *const libc::c_char,
                                );
                                if tmp___8 == 0 as libc::c_int {
                                    image_type = XPM_TYPE;
                                } else {
                                    tmp___7 = strcasecmp(
                                        optarg as *const libc::c_char,
                                        b"ansi\0" as *const u8 as *const libc::c_char,
                                    );
                                    if tmp___7 == 0 as libc::c_int {
                                        image_type = ANSI_TYPE;
                                    } else {
                                        tmp___6 = strcasecmp(
                                            optarg as *const libc::c_char,
                                            b"ansi256\0" as *const u8 as *const libc::c_char,
                                        );
                                        if tmp___6 == 0 as libc::c_int {
                                            image_type = ANSI256_TYPE;
                                        } else {
                                            tmp___5 = strcasecmp(
                                                optarg as *const libc::c_char,
                                                b"asciii\0" as *const u8 as *const libc::c_char,
                                            );
                                            if tmp___5 == 0 as libc::c_int {
                                                image_type = ASCIIi_TYPE;
                                            } else {
                                                tmp___4 = strcasecmp(
                                                    optarg as *const libc::c_char,
                                                    b"ascii\0" as *const u8 as *const libc::c_char,
                                                );
                                                if tmp___4 == 0 as libc::c_int {
                                                    image_type = ASCII_TYPE;
                                                } else {
                                                    tmp___3 = strcasecmp(
                                                        optarg as *const libc::c_char,
                                                        b"utf8\0" as *const u8 as *const libc::c_char,
                                                    );
                                                    if tmp___3 == 0 as libc::c_int {
                                                        image_type = UTF8_TYPE;
                                                    } else {
                                                        tmp___2 = strcasecmp(
                                                            optarg as *const libc::c_char,
                                                            b"ansiutf8\0" as *const u8 as *const libc::c_char,
                                                        );
                                                        if tmp___2 == 0 as libc::c_int {
                                                            image_type = ANSIUTF8_TYPE;
                                                        } else {
                                                            tmp___1 = strcasecmp(
                                                                optarg as *const libc::c_char,
                                                                b"ansi256utf8\0" as *const u8 as *const libc::c_char,
                                                            );
                                                            if tmp___1 == 0 as libc::c_int {
                                                                image_type = ANSI256UTF8_TYPE;
                                                            } else {
                                                                tmp___0 = strcasecmp(
                                                                    optarg as *const libc::c_char,
                                                                    b"utf8i\0" as *const u8 as *const libc::c_char,
                                                                );
                                                                if tmp___0 == 0 as libc::c_int {
                                                                    image_type = UTF8i_TYPE;
                                                                } else {
                                                                    tmp = strcasecmp(
                                                                        optarg as *const libc::c_char,
                                                                        b"ansiutf8i\0" as *const u8 as *const libc::c_char,
                                                                    );
                                                                    if tmp == 0 as libc::c_int {
                                                                        image_type = ANSIUTF8i_TYPE;
                                                                    } else {
                                                                        fprintf(
                                                                            stderr,
                                                                            b"Invalid image type: %s\n\0" as *const u8
                                                                                as *const libc::c_char,
                                                                            optarg,
                                                                        );
                                                                        exit(1 as libc::c_int);
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            83 => {
                structured = 1 as libc::c_int;
            }
            107 => {
                hint = QR_MODE_KANJI;
            }
            99 => {
                casesensitive = 1 as libc::c_int;
            }
            105 => {
                casesensitive = 0 as libc::c_int;
            }
            56 => {
                eightbit = 1 as libc::c_int;
            }
            77 => {
                micro = 1 as libc::c_int;
            }
            102 => {
                tmp___13 = color_set(
                    fg_color.as_mut_ptr(),
                    optarg as *const libc::c_char,
                );
                if tmp___13 != 0 {
                    fprintf(
                        stderr,
                        b"Invalid foreground color value.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
            }
            98 => {
                tmp___14 = color_set(
                    bg_color.as_mut_ptr(),
                    optarg as *const libc::c_char,
                );
                if tmp___14 != 0 {
                    fprintf(
                        stderr,
                        b"Invalid background color value.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
            }
            86 => {
                usage(0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int);
                exit(0 as libc::c_int);
            }
            0 => {}
            _ => {
                fprintf(
                    stderr,
                    b"Try \"qrencode --help\" for more information.\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
        }
    }
    if argc == 1 as libc::c_int {
        usage(1 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int);
        exit(1 as libc::c_int);
    }
    if outfile as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        if image_type as libc::c_uint == 0 as libc::c_uint {
            fprintf(
                stderr,
                b"No output filename is given.\n\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    if optind < argc {
        intext = *argv.offset(optind as isize) as *mut libc::c_uchar;
        tmp___15 = strlen(intext as *mut libc::c_char as *const libc::c_char);
        length = tmp___15 as libc::c_int;
    }
    if intext as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        if infile as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            fp = stdin;
        } else {
            tmp___16 = fopen(
                infile as *const libc::c_char,
                b"r\0" as *const u8 as *const libc::c_char,
            );
            fp = tmp___16;
        }
        if fp as libc::c_ulong == 0 as *mut FILE as libc::c_ulong {
            fprintf(
                stderr,
                b"Cannot read input file %s.\n\0" as *const u8 as *const libc::c_char,
                infile,
            );
            exit(1 as libc::c_int);
        }
        intext = readFile(fp, &mut length);
    }
    if micro != 0 {
        if version > 4 as libc::c_int {
            fprintf(
                stderr,
                b"Version number should be less or equal to %d.\n\0" as *const u8
                    as *const libc::c_char,
                4 as libc::c_int,
            );
            exit(1 as libc::c_int);
        }
    }
    if micro == 0 {
        if version > 40 as libc::c_int {
            fprintf(
                stderr,
                b"Version number should be less or equal to %d.\n\0" as *const u8
                    as *const libc::c_char,
                40 as libc::c_int,
            );
            exit(1 as libc::c_int);
        }
    }
    if margin < 0 as libc::c_int {
        if micro != 0 {
            margin = 2 as libc::c_int;
        } else {
            margin = 4 as libc::c_int;
        }
    }
    if micro != 0 {
        if structured != 0 {
            fprintf(
                stderr,
                b"Micro QR Code does not support structured symbols.\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    if structured != 0 {
        if version == 0 as libc::c_int {
            fprintf(
                stderr,
                b"Version number must be specified to encode structured symbols.\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        qrencodeStructured(
            intext as *const libc::c_uchar,
            length,
            outfile as *const libc::c_char,
        );
    } else {
        qrencode(intext as *const libc::c_uchar, length, outfile as *const libc::c_char);
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
unsafe extern "C" fn QRraw_new(mut input: *mut QRinput) -> *mut QRRawCode {
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
unsafe extern "C" fn QRraw_getCode(mut raw: *mut QRRawCode) -> libc::c_uchar {
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
unsafe extern "C" fn QRraw_free(mut raw: *mut QRRawCode) {
    if raw as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        free((*raw).datacode as *mut libc::c_void);
        free((*raw).ecccode as *mut libc::c_void);
        free((*raw).rsblock as *mut libc::c_void);
        free(raw as *mut libc::c_void);
    }
}
unsafe extern "C" fn MQRraw_new(mut input: *mut QRinput) -> *mut MQRRawCode {
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
unsafe extern "C" fn MQRraw_getCode(mut raw: *mut MQRRawCode) -> libc::c_uchar {
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
unsafe extern "C" fn MQRraw_free(mut raw: *mut MQRRawCode) {
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
unsafe extern "C" fn QRcode_new(
    mut version___0: libc::c_int,
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
    (*qrcode).version = version___0;
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
unsafe extern "C" fn QRcode_encodeMask(
    mut input: *mut QRinput,
    mut mask: libc::c_int,
) -> *mut QRcode {
    let mut current_block: u64;
    let mut width: libc::c_int = 0;
    let mut version___0: libc::c_int = 0;
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
    version___0 = (*raw).version;
    width = QRspec_getWidth(version___0);
    frame = QRspec_newFrame(version___0);
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
            p = FrameFiller_next(&mut filler);
            if p as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                current_block = 18339457858746495779;
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
                    p = FrameFiller_next(&mut filler);
                    if p as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                        current_block = 18339457858746495779;
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
                18339457858746495779 => {}
                _ => {
                    QRraw_free(raw);
                    raw = 0 as *mut libc::c_void as *mut QRRawCode;
                    j = QRspec_getRemainder(version___0);
                    i = 0 as libc::c_int;
                    loop {
                        if !(i < j) {
                            current_block = 168769493162332264;
                            break;
                        }
                        p = FrameFiller_next(&mut filler);
                        if p as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong
                        {
                            current_block = 18339457858746495779;
                            break;
                        }
                        *p = 2 as libc::c_int as libc::c_uchar;
                        i += 1;
                    }
                    match current_block {
                        18339457858746495779 => {}
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
                                qrcode = QRcode_new(version___0, width, masked);
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
unsafe extern "C" fn QRcode_encodeMaskMQR(
    mut input: *mut QRinput,
    mut mask: libc::c_int,
) -> *mut QRcode {
    let mut current_block: u64;
    let mut width: libc::c_int = 0;
    let mut version___0: libc::c_int = 0;
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
    version___0 = (*raw).version;
    width = MQRspec_getWidth(version___0);
    frame = MQRspec_newFrame(version___0);
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
            p = FrameFiller_next(&mut filler);
            if p as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                current_block = 1043431933058879630;
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
                    p = FrameFiller_next(&mut filler);
                    if p as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                        current_block = 1043431933058879630;
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
                1043431933058879630 => {}
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
                        masked = MMask_mask(version___0, frame, (*input).level);
                    } else {
                        masked = MMask_makeMask(
                            version___0,
                            frame,
                            mask,
                            (*input).level,
                        );
                    }
                    if !(masked as libc::c_ulong
                        == 0 as *mut libc::c_void as libc::c_ulong)
                    {
                        qrcode = QRcode_new(version___0, width, masked);
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
    mut version___0: libc::c_int,
    mut level___0: QRecLevel,
    mut mqr: libc::c_int,
    mut hint___0: QRencodeMode,
    mut casesensitive___0: libc::c_int,
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
    if hint___0 as libc::c_int != 2 as libc::c_int {
        if hint___0 as libc::c_int != 3 as libc::c_int {
            tmp___0 = __errno_location();
            *tmp___0 = 22 as libc::c_int;
            return 0 as *mut libc::c_void as *mut QRcode;
        }
    }
    if mqr != 0 {
        input = QRinput_newMQR(version___0, level___0);
    } else {
        input = QRinput_new2(version___0, level___0);
    }
    if input as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut QRcode;
    }
    ret = Split_splitStringToQRinput(string, input, hint___0, casesensitive___0);
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
    mut version___0: libc::c_int,
    mut level___0: QRecLevel,
    mut hint___0: QRencodeMode,
    mut casesensitive___0: libc::c_int,
) -> *mut QRcode {
    let mut tmp: *mut QRcode = 0 as *mut QRcode;
    tmp = QRcode_encodeStringReal(
        string,
        version___0,
        level___0,
        0 as libc::c_int,
        hint___0,
        casesensitive___0,
    );
    return tmp;
}
pub unsafe extern "C" fn QRcode_encodeStringMQR(
    mut string: *const libc::c_char,
    mut version___0: libc::c_int,
    mut level___0: QRecLevel,
    mut hint___0: QRencodeMode,
    mut casesensitive___0: libc::c_int,
) -> *mut QRcode {
    let mut i: libc::c_int = 0;
    let mut code: *mut QRcode = 0 as *mut QRcode;
    let mut tmp: *mut QRcode = 0 as *mut QRcode;
    if version___0 == 0 as libc::c_int {
        version___0 = 1 as libc::c_int;
    }
    i = version___0;
    while i <= 4 as libc::c_int {
        tmp = QRcode_encodeStringReal(
            string,
            i,
            level___0,
            1 as libc::c_int,
            hint___0,
            casesensitive___0,
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
    mut version___0: libc::c_int,
    mut level___0: QRecLevel,
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
        input = QRinput_newMQR(version___0, level___0);
    } else {
        input = QRinput_new2(version___0, level___0);
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
    mut size___0: libc::c_int,
    mut data: *const libc::c_uchar,
    mut version___0: libc::c_int,
    mut level___0: QRecLevel,
) -> *mut QRcode {
    let mut tmp: *mut QRcode = 0 as *mut QRcode;
    tmp = QRcode_encodeDataReal(
        data,
        size___0,
        version___0,
        level___0,
        0 as libc::c_int,
    );
    return tmp;
}
pub unsafe extern "C" fn QRcode_encodeString8bit(
    mut string: *const libc::c_char,
    mut version___0: libc::c_int,
    mut level___0: QRecLevel,
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
        version___0,
        level___0,
        0 as libc::c_int,
    );
    return tmp___1;
}
pub unsafe extern "C" fn QRcode_encodeDataMQR(
    mut size___0: libc::c_int,
    mut data: *const libc::c_uchar,
    mut version___0: libc::c_int,
    mut level___0: QRecLevel,
) -> *mut QRcode {
    let mut i: libc::c_int = 0;
    let mut code: *mut QRcode = 0 as *mut QRcode;
    let mut tmp: *mut QRcode = 0 as *mut QRcode;
    if version___0 == 0 as libc::c_int {
        version___0 = 1 as libc::c_int;
    }
    i = version___0;
    while i <= 4 as libc::c_int {
        tmp = QRcode_encodeDataReal(data, size___0, i, level___0, 1 as libc::c_int);
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
    mut version___0: libc::c_int,
    mut level___0: QRecLevel,
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
    if version___0 == 0 as libc::c_int {
        version___0 = 1 as libc::c_int;
    }
    i = version___0;
    while i <= 4 as libc::c_int {
        tmp___0 = strlen(string);
        tmp___1 = QRcode_encodeDataReal(
            string as *mut libc::c_uchar as *const libc::c_uchar,
            tmp___0 as libc::c_int,
            i,
            level___0,
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
    let mut size___0: libc::c_int = 0;
    list = qrlist;
    size___0 = 0 as libc::c_int;
    while list as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        size___0 += 1;
        list = (*list).next;
    }
    return size___0;
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
                current_block = 12441108671205489790;
                break;
            }
            head = entry;
            tail = head;
        } else {
            entry = QRcode_List_newEntry();
            if entry as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                current_block = 12441108671205489790;
                break;
            }
            (*tail).next = entry;
            tail = (*tail).next;
        }
        (*tail).code = QRcode_encodeInput((*list).input);
        if (*tail).code as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            current_block = 12441108671205489790;
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
    mut size___0: libc::c_int,
    mut data: *const libc::c_uchar,
    mut version___0: libc::c_int,
    mut level___0: QRecLevel,
    mut eightbit___0: libc::c_int,
    mut hint___0: QRencodeMode,
    mut casesensitive___0: libc::c_int,
) -> *mut QRcode_List {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut codes: *mut QRcode_List = 0 as *mut QRcode_List;
    let mut ret: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    if version___0 <= 0 as libc::c_int {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
        return 0 as *mut libc::c_void as *mut QRcode_List;
    }
    if eightbit___0 == 0 {
        if hint___0 as libc::c_int != 2 as libc::c_int {
            if hint___0 as libc::c_int != 3 as libc::c_int {
                tmp___0 = __errno_location();
                *tmp___0 = 22 as libc::c_int;
                return 0 as *mut libc::c_void as *mut QRcode_List;
            }
        }
    }
    input = QRinput_new2(version___0, level___0);
    if input as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut QRcode_List;
    }
    if eightbit___0 != 0 {
        ret = QRinput_append(input, QR_MODE_8, size___0, data);
    } else {
        ret = Split_splitStringToQRinput(
            data as *mut libc::c_char as *const libc::c_char,
            input,
            hint___0,
            casesensitive___0,
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
    mut size___0: libc::c_int,
    mut data: *const libc::c_uchar,
    mut version___0: libc::c_int,
    mut level___0: QRecLevel,
) -> *mut QRcode_List {
    let mut tmp: *mut QRcode_List = 0 as *mut QRcode_List;
    tmp = QRcode_encodeDataStructuredReal(
        size___0,
        data,
        version___0,
        level___0,
        1 as libc::c_int,
        QR_MODE_NUL,
        0 as libc::c_int,
    );
    return tmp;
}
pub unsafe extern "C" fn QRcode_encodeString8bitStructured(
    mut string: *const libc::c_char,
    mut version___0: libc::c_int,
    mut level___0: QRecLevel,
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
        version___0,
        level___0,
    );
    return tmp___1;
}
pub unsafe extern "C" fn QRcode_encodeStringStructured(
    mut string: *const libc::c_char,
    mut version___0: libc::c_int,
    mut level___0: QRecLevel,
    mut hint___0: QRencodeMode,
    mut casesensitive___0: libc::c_int,
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
        version___0,
        level___0,
        0 as libc::c_int,
        hint___0,
        casesensitive___0,
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
    mut size___0: libc::c_int,
    mut data: *const libc::c_uchar,
) -> *mut QRinput_List {
    let mut entry: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp___0 = QRinput_check(mode, size___0, data);
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
    (*entry).size = size___0;
    (*entry).data = 0 as *mut libc::c_void as *mut libc::c_uchar;
    if size___0 > 0 as libc::c_int {
        tmp___2 = malloc(size___0 as size_t);
        (*entry).data = tmp___2 as *mut libc::c_uchar;
        if (*entry).data as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            free(entry as *mut libc::c_void);
            return 0 as *mut libc::c_void as *mut QRinput_List;
        }
        memcpy(
            (*entry).data as *mut libc::c_void,
            data as *const libc::c_void,
            size___0 as size_t,
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
    mut version___0: libc::c_int,
    mut level___0: QRecLevel,
) -> *mut QRinput {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    if version___0 < 0 as libc::c_int {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
        return 0 as *mut libc::c_void as *mut QRinput;
    } else {
        if version___0 > 40 as libc::c_int {
            tmp = __errno_location();
            *tmp = 22 as libc::c_int;
            return 0 as *mut libc::c_void as *mut QRinput;
        } else {
            if (level___0 as libc::c_uint) < 0 as libc::c_uint {
                tmp = __errno_location();
                *tmp = 22 as libc::c_int;
                return 0 as *mut libc::c_void as *mut QRinput;
            } else {
                if level___0 as libc::c_uint > 3 as libc::c_uint {
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
    (*input).version = version___0;
    (*input).level = level___0;
    (*input).mqr = 0 as libc::c_int;
    (*input).fnc1 = 0 as libc::c_int;
    return input;
}
pub unsafe extern "C" fn QRinput_newMQR(
    mut version___0: libc::c_int,
    mut level___0: QRecLevel,
) -> *mut QRinput {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    if !(version___0 <= 0 as libc::c_int) {
        if !(version___0 > 4 as libc::c_int) {
            tmp = MQRspec_getECCLength(version___0, level___0);
            if !(tmp == 0 as libc::c_int) {
                input = QRinput_new2(version___0, level___0);
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
    mut version___0: libc::c_int,
) -> libc::c_int {
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    if (*input).mqr != 0 {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
        return -(1 as libc::c_int);
    } else {
        if version___0 < 0 as libc::c_int {
            tmp = __errno_location();
            *tmp = 22 as libc::c_int;
            return -(1 as libc::c_int);
        } else {
            if version___0 > 40 as libc::c_int {
                tmp = __errno_location();
                *tmp = 22 as libc::c_int;
                return -(1 as libc::c_int);
            }
        }
    }
    (*input).version = version___0;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_getErrorCorrectionLevel(
    mut input: *mut QRinput,
) -> QRecLevel {
    return (*input).level;
}
pub unsafe extern "C" fn QRinput_setErrorCorrectionLevel(
    mut input: *mut QRinput,
    mut level___0: QRecLevel,
) -> libc::c_int {
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    if (*input).mqr != 0 {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
        return -(1 as libc::c_int);
    } else {
        if level___0 as libc::c_uint > 3 as libc::c_uint {
            tmp = __errno_location();
            *tmp = 22 as libc::c_int;
            return -(1 as libc::c_int);
        }
    }
    (*input).level = level___0;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_setVersionAndErrorCorrectionLevel(
    mut input: *mut QRinput,
    mut version___0: libc::c_int,
    mut level___0: QRecLevel,
) -> libc::c_int {
    let mut current_block: u64;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    if (*input).mqr != 0 {
        if version___0 <= 0 as libc::c_int {
            current_block = 15369704502871485246;
        } else if version___0 > 4 as libc::c_int {
            current_block = 15369704502871485246;
        } else {
            tmp = MQRspec_getECCLength(version___0, level___0);
            if tmp == 0 as libc::c_int {
                current_block = 15369704502871485246;
            } else {
                current_block = 1841672684692190573;
            }
        }
    } else if version___0 < 0 as libc::c_int {
        current_block = 15369704502871485246;
    } else if version___0 > 40 as libc::c_int {
        current_block = 15369704502871485246;
    } else if level___0 as libc::c_uint > 3 as libc::c_uint {
        current_block = 15369704502871485246;
    } else {
        current_block = 1841672684692190573;
    }
    match current_block {
        15369704502871485246 => {
            tmp___0 = __errno_location();
            *tmp___0 = 22 as libc::c_int;
            return -(1 as libc::c_int);
        }
        _ => {
            (*input).version = version___0;
            (*input).level = level___0;
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
    mut size___0: libc::c_int,
    mut data: *const libc::c_uchar,
) -> libc::c_int {
    let mut entry: *mut QRinput_List = 0 as *mut QRinput_List;
    entry = QRinput_List_newEntry(mode, size___0, data);
    if entry as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    QRinput_appendEntry(input, entry);
    return 0 as libc::c_int;
}
unsafe extern "C" fn QRinput_insertStructuredAppendHeader(
    mut input: *mut QRinput,
    mut size___0: libc::c_int,
    mut number: libc::c_int,
    mut parity: libc::c_uchar,
) -> libc::c_int {
    let mut entry: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut buf: [libc::c_uchar; 3] = [0; 3];
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    if size___0 > 16 as libc::c_int {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if number <= 0 as libc::c_int {
        tmp___0 = __errno_location();
        *tmp___0 = 22 as libc::c_int;
        return -(1 as libc::c_int);
    } else {
        if number > size___0 {
            tmp___0 = __errno_location();
            *tmp___0 = 22 as libc::c_int;
            return -(1 as libc::c_int);
        }
    }
    buf[0 as libc::c_int as usize] = size___0 as libc::c_uchar;
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
    mut size___0: libc::c_int,
    mut data: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size___0 {
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
    mut size___0: libc::c_int,
) -> libc::c_int {
    let mut w: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    w = size___0 / 3 as libc::c_int;
    bits = w * 10 as libc::c_int;
    match size___0 - w * 3 as libc::c_int {
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
    mut version___0: libc::c_int,
    mut mqr: libc::c_int,
) -> libc::c_int {
    let mut words: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut val: libc::c_uint = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    if mqr != 0 {
        if version___0 > 1 as libc::c_int {
            ret = BitStream_appendNum(
                bstream,
                (version___0 - 1 as libc::c_int) as size_t,
                0 as libc::c_uint,
            );
            if ret < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
        }
        tmp = MQRspec_lengthIndicator(QR_MODE_NUM, version___0);
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
        tmp___0 = QRspec_lengthIndicator(QR_MODE_NUM, version___0);
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
    mut size___0: libc::c_int,
    mut data: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size___0 {
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
    mut size___0: libc::c_int,
) -> libc::c_int {
    let mut w: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    w = size___0 / 2 as libc::c_int;
    bits = w * 11 as libc::c_int;
    if size___0 & 1 as libc::c_int != 0 {
        bits += 6 as libc::c_int;
    }
    return bits;
}
unsafe extern "C" fn QRinput_encodeModeAn(
    mut entry: *mut QRinput_List,
    mut bstream: *mut BitStream,
    mut version___0: libc::c_int,
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
        if version___0 < 2 as libc::c_int {
            tmp = __errno_location();
            *tmp = 34 as libc::c_int;
            return -(1 as libc::c_int);
        }
        ret = BitStream_appendNum(
            bstream,
            (version___0 - 1 as libc::c_int) as size_t,
            1 as libc::c_uint,
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        tmp___0 = MQRspec_lengthIndicator(QR_MODE_AN, version___0);
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
        tmp___1 = QRspec_lengthIndicator(QR_MODE_AN, version___0);
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
    mut size___0: libc::c_int,
) -> libc::c_int {
    return size___0 * 8 as libc::c_int;
}
unsafe extern "C" fn QRinput_encodeMode8(
    mut entry: *mut QRinput_List,
    mut bstream: *mut BitStream,
    mut version___0: libc::c_int,
    mut mqr: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    if mqr != 0 {
        if version___0 < 3 as libc::c_int {
            tmp = __errno_location();
            *tmp = 34 as libc::c_int;
            return -(1 as libc::c_int);
        }
        ret = BitStream_appendNum(
            bstream,
            (version___0 - 1 as libc::c_int) as size_t,
            2 as libc::c_uint,
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        tmp___0 = MQRspec_lengthIndicator(QR_MODE_8, version___0);
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
        tmp___1 = QRspec_lengthIndicator(QR_MODE_8, version___0);
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
    mut size___0: libc::c_int,
) -> libc::c_int {
    return size___0 / 2 as libc::c_int * 13 as libc::c_int;
}
unsafe extern "C" fn QRinput_checkModeKanji(
    mut size___0: libc::c_int,
    mut data: *const libc::c_uchar,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut val: libc::c_uint = 0;
    if size___0 & 1 as libc::c_int != 0 {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < size___0 {
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
    mut version___0: libc::c_int,
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
        if version___0 < 2 as libc::c_int {
            tmp = __errno_location();
            *tmp = 34 as libc::c_int;
            return -(1 as libc::c_int);
        }
        ret = BitStream_appendNum(
            bstream,
            (version___0 - 1 as libc::c_int) as size_t,
            3 as libc::c_uint,
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        tmp___0 = MQRspec_lengthIndicator(QR_MODE_KANJI, version___0);
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
        tmp___1 = QRspec_lengthIndicator(QR_MODE_KANJI, version___0);
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
unsafe extern "C" fn QRinput_checkModeFNC1Second(
    mut size___0: libc::c_int,
) -> libc::c_int {
    if size___0 != 1 as libc::c_int {
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
    mut size___0: libc::c_int,
    mut data: *const libc::c_uchar,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    if mode as libc::c_int == 6 as libc::c_int {
        if size___0 < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    if size___0 <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    match mode as libc::c_int {
        0 => {
            tmp = QRinput_checkModeNum(size___0, data as *const libc::c_char);
            return tmp;
        }
        1 => {
            tmp___0 = QRinput_checkModeAn(size___0, data as *const libc::c_char);
            return tmp___0;
        }
        3 => {
            tmp___1 = QRinput_checkModeKanji(size___0, data);
            return tmp___1;
        }
        2 => return 0 as libc::c_int,
        4 => return 0 as libc::c_int,
        5 => return 0 as libc::c_int,
        6 => return 0 as libc::c_int,
        7 => {
            tmp___2 = QRinput_checkModeFNC1Second(size___0);
            return tmp___2;
        }
        -1 | _ => {}
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn QRinput_estimateBitStreamSizeOfEntry(
    mut entry: *mut QRinput_List,
    mut version___0: libc::c_int,
    mut mqr: libc::c_int,
) -> libc::c_int {
    let mut bits: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    bits = 0 as libc::c_int;
    if version___0 == 0 as libc::c_int {
        version___0 = 1 as libc::c_int;
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
        l = MQRspec_lengthIndicator((*entry).mode, version___0);
        m = version___0 - 1 as libc::c_int;
        bits += l + m;
    } else {
        l = QRspec_lengthIndicator((*entry).mode, version___0);
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
unsafe extern "C" fn QRinput_estimateBitStreamSize(
    mut input: *mut QRinput,
    mut version___0: libc::c_int,
) -> libc::c_int {
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut bits: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    bits = 0 as libc::c_int;
    list = (*input).head;
    while list as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp = QRinput_estimateBitStreamSizeOfEntry(list, version___0, (*input).mqr);
        bits += tmp;
        list = (*list).next;
    }
    return bits;
}
unsafe extern "C" fn QRinput_estimateVersion(mut input: *mut QRinput) -> libc::c_int {
    let mut bits: libc::c_int = 0;
    let mut version___0: libc::c_int = 0;
    let mut prev: libc::c_int = 0;
    version___0 = 0 as libc::c_int;
    loop {
        prev = version___0;
        bits = QRinput_estimateBitStreamSize(input, prev);
        version___0 = QRspec_getMinimumVersion(
            (bits + 7 as libc::c_int) / 8 as libc::c_int,
            (*input).level,
        );
        if prev == 0 as libc::c_int {
            if version___0 > 1 as libc::c_int {
                version___0 -= 1;
            }
        }
        if !(version___0 > prev) {
            break;
        }
    }
    return version___0;
}
unsafe extern "C" fn QRinput_lengthOfCode(
    mut mode: QRencodeMode,
    mut version___0: libc::c_int,
    mut bits: libc::c_int,
) -> libc::c_int {
    let mut payload: libc::c_int = 0;
    let mut size___0: libc::c_int = 0;
    let mut chunks: libc::c_int = 0;
    let mut remain: libc::c_int = 0;
    let mut maxsize: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tmp = QRspec_lengthIndicator(mode, version___0);
    payload = bits - 4 as libc::c_int - tmp;
    match mode as libc::c_int {
        0 => {
            chunks = payload / 10 as libc::c_int;
            remain = payload - chunks * 10 as libc::c_int;
            size___0 = chunks * 3 as libc::c_int;
            if remain >= 7 as libc::c_int {
                size___0 += 2 as libc::c_int;
            } else if remain >= 4 as libc::c_int {
                size___0 += 1;
            }
        }
        1 => {
            chunks = payload / 11 as libc::c_int;
            remain = payload - chunks * 11 as libc::c_int;
            size___0 = chunks * 2 as libc::c_int;
            if remain >= 6 as libc::c_int {
                size___0 += 1;
            }
        }
        2 => {
            size___0 = payload / 8 as libc::c_int;
        }
        3 => {
            size___0 = payload / 13 as libc::c_int * 2 as libc::c_int;
        }
        4 => {
            size___0 = payload / 8 as libc::c_int;
        }
        _ => {
            size___0 = 0 as libc::c_int;
        }
    }
    maxsize = QRspec_maximumWords(mode, version___0);
    if size___0 < 0 as libc::c_int {
        size___0 = 0 as libc::c_int;
    }
    if maxsize > 0 as libc::c_int {
        if size___0 > maxsize {
            size___0 = maxsize;
        }
    }
    return size___0;
}
unsafe extern "C" fn QRinput_encodeBitStream(
    mut entry: *mut QRinput_List,
    mut bstream: *mut BitStream,
    mut version___0: libc::c_int,
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
        words = MQRspec_maximumWords((*entry).mode, version___0);
    } else {
        words = QRspec_maximumWords((*entry).mode, version___0);
    }
    if words != 0 as libc::c_int {
        if (*entry).size > words {
            st1 = QRinput_List_newEntry(
                (*entry).mode,
                words,
                (*entry).data as *const libc::c_uchar,
            );
            if st1 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                current_block = 10104863907319303966;
            } else {
                st2 = QRinput_List_newEntry(
                    (*entry).mode,
                    (*entry).size - words,
                    ((*entry).data).offset(words as isize) as *const libc::c_uchar,
                );
                if st2 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                    current_block = 10104863907319303966;
                } else {
                    ret = QRinput_encodeBitStream(st1, bstream, version___0, mqr);
                    if ret < 0 as libc::c_int {
                        current_block = 10104863907319303966;
                    } else {
                        ret = QRinput_encodeBitStream(st2, bstream, version___0, mqr);
                        if ret < 0 as libc::c_int {
                            current_block = 10104863907319303966;
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
            current_block = 15389380602719119621;
        }
    } else {
        current_block = 15389380602719119621;
    }
    match current_block {
        15389380602719119621 => {
            ret = 0 as libc::c_int;
            match (*entry).mode as libc::c_int {
                0 => {
                    ret = QRinput_encodeModeNum(entry, bstream, version___0, mqr);
                }
                1 => {
                    ret = QRinput_encodeModeAn(entry, bstream, version___0, mqr);
                }
                2 => {
                    ret = QRinput_encodeMode8(entry, bstream, version___0, mqr);
                }
                3 => {
                    ret = QRinput_encodeModeKanji(entry, bstream, version___0, mqr);
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
unsafe extern "C" fn QRinput_mergeBitStream(
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
unsafe extern "C" fn QRinput_getBitStream(
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
unsafe extern "C" fn QRinput_splitEntry(
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
                        current_block = 11126108056613275728;
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
                        current_block = 11126108056613275728;
                        break;
                    }
                    if bytes > 0 as libc::c_int {
                        ret = QRinput_splitEntry(list, bytes);
                        if ret < 0 as libc::c_int {
                            QRinput_free(p);
                            current_block = 11126108056613275728;
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
                        current_block = 11126108056613275728;
                        break;
                    } else {
                        input = p;
                        bits = 0 as libc::c_int;
                    }
                }
            }
            match current_block {
                11126108056613275728 => {}
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
    mut size___0: size_t,
    mut data: *mut libc::c_uchar,
) {
    let mut mask: libc::c_uchar = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    p = dest;
    i = 0 as libc::c_int as size_t;
    while i < size___0 {
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
    mut size___0: size_t,
    mut data: *mut libc::c_uchar,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if size___0 == 0 as libc::c_ulong {
        return 0 as libc::c_int;
    }
    while ((*bstream).datasize).wrapping_sub((*bstream).length)
        < size___0.wrapping_mul(8 as libc::c_ulong)
    {
        ret = BitStream_expand(bstream);
        if ret < 0 as libc::c_int {
            return ret;
        }
    }
    BitStream_writeBytes(
        ((*bstream).data).offset((*bstream).length as isize),
        size___0,
        data,
    );
    (*bstream)
        .length = ((*bstream).length as libc::c_ulong)
        .wrapping_add(size___0.wrapping_mul(8 as libc::c_ulong)) as size_t as size_t;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn BitStream_toByte(
    mut bstream: *mut BitStream,
) -> *mut libc::c_uchar {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut size___0: size_t = 0;
    let mut bytes: size_t = 0;
    let mut oddbits: size_t = 0;
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut v: libc::c_uchar = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    size___0 = (*bstream).length;
    if size___0 == 0 as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut libc::c_uchar;
    }
    tmp = malloc(
        size___0.wrapping_add(7 as libc::c_ulong).wrapping_div(8 as libc::c_ulong),
    );
    data = tmp as *mut libc::c_uchar;
    if data as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut libc::c_uchar;
    }
    bytes = size___0.wrapping_div(8 as libc::c_ulong);
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
    oddbits = size___0 & 7 as libc::c_ulong;
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
    mut version___0: libc::c_int,
    mut level___0: QRecLevel,
) -> libc::c_int {
    return qrspecCapacity[version___0 as usize].words
        - qrspecCapacity[version___0 as usize].ec[level___0 as usize];
}
pub unsafe extern "C" fn QRspec_getECCLength(
    mut version___0: libc::c_int,
    mut level___0: QRecLevel,
) -> libc::c_int {
    return qrspecCapacity[version___0 as usize].ec[level___0 as usize];
}
pub unsafe extern "C" fn QRspec_getMinimumVersion(
    mut size___0: libc::c_int,
    mut level___0: QRecLevel,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut words: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= 40 as libc::c_int {
        words = qrspecCapacity[i as usize].words
            - qrspecCapacity[i as usize].ec[level___0 as usize];
        if words >= size___0 {
            return i;
        }
        i += 1;
    }
    return 40 as libc::c_int;
}
pub unsafe extern "C" fn QRspec_getWidth(mut version___0: libc::c_int) -> libc::c_int {
    return qrspecCapacity[version___0 as usize].width;
}
pub unsafe extern "C" fn QRspec_getRemainder(
    mut version___0: libc::c_int,
) -> libc::c_int {
    return qrspecCapacity[version___0 as usize].remainder;
}
static mut lengthTableBits: [[libc::c_int; 3]; 4] = [
    [10 as libc::c_int, 12 as libc::c_int, 14 as libc::c_int],
    [9 as libc::c_int, 11 as libc::c_int, 13 as libc::c_int],
    [8 as libc::c_int, 16 as libc::c_int, 16 as libc::c_int],
    [8 as libc::c_int, 10 as libc::c_int, 12 as libc::c_int],
];
pub unsafe extern "C" fn QRspec_lengthIndicator(
    mut mode: QRencodeMode,
    mut version___0: libc::c_int,
) -> libc::c_int {
    let mut l: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tmp = QRinput_isSplittableMode(mode);
    if tmp == 0 {
        return 0 as libc::c_int;
    }
    if version___0 <= 9 as libc::c_int {
        l = 0 as libc::c_int;
    } else if version___0 <= 26 as libc::c_int {
        l = 1 as libc::c_int;
    } else {
        l = 2 as libc::c_int;
    }
    return lengthTableBits[mode as usize][l as usize];
}
pub unsafe extern "C" fn QRspec_maximumWords(
    mut mode: QRencodeMode,
    mut version___0: libc::c_int,
) -> libc::c_int {
    let mut l: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    let mut words: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tmp = QRinput_isSplittableMode(mode);
    if tmp == 0 {
        return 0 as libc::c_int;
    }
    if version___0 <= 9 as libc::c_int {
        l = 0 as libc::c_int;
    } else if version___0 <= 26 as libc::c_int {
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
    mut version___0: libc::c_int,
    mut level___0: QRecLevel,
    mut spec: *mut libc::c_int,
) {
    let mut b1: libc::c_int = 0;
    let mut b2: libc::c_int = 0;
    let mut data: libc::c_int = 0;
    let mut ecc: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    b1 = eccTable[version___0 as usize][level___0 as usize][0 as libc::c_int as usize];
    b2 = eccTable[version___0 as usize][level___0 as usize][1 as libc::c_int as usize];
    data = QRspec_getDataLength(version___0, level___0);
    ecc = QRspec_getECCLength(version___0, level___0);
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
    mut version___0: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut width: libc::c_int,
) {
    let mut d: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut cx: libc::c_int = 0;
    let mut cy: libc::c_int = 0;
    if version___0 < 2 as libc::c_int {
        return;
    }
    d = alignmentPattern[version___0 as usize][1 as libc::c_int as usize]
        - alignmentPattern[version___0 as usize][0 as libc::c_int as usize];
    if d < 0 as libc::c_int {
        w = 2 as libc::c_int;
    } else {
        w = (width - alignmentPattern[version___0 as usize][0 as libc::c_int as usize])
            / d + 2 as libc::c_int;
    }
    if w * w - 3 as libc::c_int == 1 as libc::c_int {
        x = alignmentPattern[version___0 as usize][0 as libc::c_int as usize];
        y = alignmentPattern[version___0 as usize][0 as libc::c_int as usize];
        QRspec_putAlignmentMarker(frame, width, x, y);
        return;
    }
    cx = alignmentPattern[version___0 as usize][0 as libc::c_int as usize];
    x = 1 as libc::c_int;
    while x < w - 1 as libc::c_int {
        QRspec_putAlignmentMarker(frame, width, 6 as libc::c_int, cx);
        QRspec_putAlignmentMarker(frame, width, cx, 6 as libc::c_int);
        cx += d;
        x += 1;
    }
    cy = alignmentPattern[version___0 as usize][0 as libc::c_int as usize];
    y = 0 as libc::c_int;
    while y < w - 1 as libc::c_int {
        cx = alignmentPattern[version___0 as usize][0 as libc::c_int as usize];
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
    mut version___0: libc::c_int,
) -> libc::c_uint {
    if version___0 < 7 as libc::c_int {
        return 0 as libc::c_uint
    } else {
        if version___0 > 40 as libc::c_int {
            return 0 as libc::c_uint;
        }
    }
    return versionPattern[(version___0 - 7 as libc::c_int) as usize];
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
    mut level___0: QRecLevel,
) -> libc::c_uint {
    if mask < 0 as libc::c_int {
        return 0 as libc::c_uint
    } else {
        if mask > 7 as libc::c_int {
            return 0 as libc::c_uint;
        }
    }
    return formatInfo[level___0 as usize][mask as usize];
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
unsafe extern "C" fn QRspec_createFrame(
    mut version___0: libc::c_int,
) -> *mut libc::c_uchar {
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut q: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut width: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut verinfo: libc::c_uint = 0;
    let mut v: libc::c_uint = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    width = qrspecCapacity[version___0 as usize].width;
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
    QRspec_putAlignmentPattern(version___0, frame, width);
    if version___0 >= 7 as libc::c_int {
        verinfo = QRspec_getVersionPattern(version___0);
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
    mut version___0: libc::c_int,
) -> *mut libc::c_uchar {
    let mut tmp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if version___0 < 1 as libc::c_int {
        return 0 as *mut libc::c_void as *mut libc::c_uchar
    } else {
        if version___0 > 40 as libc::c_int {
            return 0 as *mut libc::c_void as *mut libc::c_uchar;
        }
    }
    tmp = QRspec_createFrame(version___0);
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
    mut hint___0: QRencodeMode,
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
            if hint___0 as libc::c_int == 3 as libc::c_int {
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
    mut hint___0: QRencodeMode,
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
    mode = Split_identifyMode(p, hint___0);
    if mode as libc::c_int == 2 as libc::c_int {
        tmp = QRinput_estimateBitsModeNum(run);
        tmp___0 = QRinput_estimateBitsMode8(1 as libc::c_int);
        tmp___1 = QRinput_estimateBitsMode8(run + 1 as libc::c_int);
        dif = tmp + 4 as libc::c_int + ln + tmp___0 - tmp___1;
        if dif > 0 as libc::c_int {
            tmp___2 = Split_eat8(string, input, hint___0);
            return tmp___2;
        }
    }
    if mode as libc::c_int == 1 as libc::c_int {
        tmp___3 = QRinput_estimateBitsModeNum(run);
        tmp___4 = QRinput_estimateBitsModeAn(1 as libc::c_int);
        tmp___5 = QRinput_estimateBitsModeAn(run + 1 as libc::c_int);
        dif = tmp___3 + 4 as libc::c_int + ln + tmp___4 - tmp___5;
        if dif > 0 as libc::c_int {
            tmp___6 = Split_eatAn(string, input, hint___0);
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
    mut hint___0: QRencodeMode,
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
                tmp___9 = Split_eat8(string, input, hint___0);
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
    mut hint___0: QRencodeMode,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: libc::c_int = 0;
    let mut run: libc::c_int = 0;
    let mut tmp: QRencodeMode = QR_MODE_NUM;
    p = string;
    loop {
        tmp = Split_identifyMode(p, hint___0);
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
    mut hint___0: QRencodeMode,
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
        mode = Split_identifyMode(p, hint___0);
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
            tmp = Split_identifyMode(q, hint___0);
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
            tmp___4 = Split_identifyMode(q, hint___0);
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
    mut hint___0: QRencodeMode,
) -> libc::c_int {
    let mut length: libc::c_int = 0;
    let mut mode: QRencodeMode = QR_MODE_NUM;
    while *string as libc::c_int != 0 as libc::c_int {
        mode = Split_identifyMode(string, hint___0);
        if mode as libc::c_int == 0 as libc::c_int {
            length = Split_eatNum(string, input, hint___0);
        } else if mode as libc::c_int == 1 as libc::c_int {
            length = Split_eatAn(string, input, hint___0);
        } else if mode as libc::c_int == 3 as libc::c_int {
            if hint___0 as libc::c_int == 3 as libc::c_int {
                length = Split_eatKanji(string, input, hint___0);
            } else {
                length = Split_eat8(string, input, hint___0);
            }
        } else {
            length = Split_eat8(string, input, hint___0);
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
    mut hint___0: QRencodeMode,
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
        mode = Split_identifyMode(p as *const libc::c_char, hint___0);
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
    mut hint___0: QRencodeMode,
    mut casesensitive___0: libc::c_int,
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
    if casesensitive___0 == 0 {
        newstr = dupAndToUpper(string, hint___0);
        if newstr as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            return -(1 as libc::c_int);
        }
        ret = Split_splitString(newstr as *const libc::c_char, input, hint___0);
        free(newstr as *mut libc::c_void);
    } else {
        ret = Split_splitString(string, input, hint___0);
    }
    return ret;
}
unsafe extern "C" fn Mask_writeFormatInformation(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut mask: libc::c_int,
    mut level___0: QRecLevel,
) -> libc::c_int {
    let mut format: libc::c_uint = 0;
    let mut v: libc::c_uchar = 0;
    let mut i: libc::c_int = 0;
    let mut blacks: libc::c_int = 0;
    blacks = 0 as libc::c_int;
    format = QRspec_getFormatInfo(mask, level___0);
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
pub unsafe extern "C" fn Mask_makeMask(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut mask: libc::c_int,
    mut level___0: QRecLevel,
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
    Mask_writeFormatInformation(width, masked, mask, level___0);
    return masked;
}
unsafe extern "C" fn Mask_calcN1N3(
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
unsafe extern "C" fn Mask_calcN2(
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
unsafe extern "C" fn Mask_calcRunLengthH(
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
unsafe extern "C" fn Mask_calcRunLengthV(
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
unsafe extern "C" fn Mask_evaluateSymbol(
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
    mut level___0: QRecLevel,
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
        tmp___1 = Mask_writeFormatInformation(width, mask, i, level___0);
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
    mut version___0: libc::c_int,
    mut level___0: QRecLevel,
) -> libc::c_int {
    let mut w: libc::c_int = 0;
    let mut ecc: libc::c_int = 0;
    w = mqrspecCapacity[version___0 as usize].width - 1 as libc::c_int;
    ecc = mqrspecCapacity[version___0 as usize].ec[level___0 as usize];
    if ecc == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return w * w - 64 as libc::c_int - ecc * 8 as libc::c_int;
}
pub unsafe extern "C" fn MQRspec_getDataLength(
    mut version___0: libc::c_int,
    mut level___0: QRecLevel,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = MQRspec_getDataLengthBit(version___0, level___0);
    return (tmp + 4 as libc::c_int) / 8 as libc::c_int;
}
pub unsafe extern "C" fn MQRspec_getECCLength(
    mut version___0: libc::c_int,
    mut level___0: QRecLevel,
) -> libc::c_int {
    return mqrspecCapacity[version___0 as usize].ec[level___0 as usize];
}
pub unsafe extern "C" fn MQRspec_getWidth(mut version___0: libc::c_int) -> libc::c_int {
    return mqrspecCapacity[version___0 as usize].width;
}
static mut lengthTableBits___0: [[libc::c_int; 4]; 4] = [
    [3 as libc::c_int, 4 as libc::c_int, 5 as libc::c_int, 6 as libc::c_int],
    [0 as libc::c_int, 3 as libc::c_int, 4 as libc::c_int, 5 as libc::c_int],
    [0 as libc::c_int, 0 as libc::c_int, 4 as libc::c_int, 5 as libc::c_int],
    [0 as libc::c_int, 0 as libc::c_int, 3 as libc::c_int, 4 as libc::c_int],
];
pub unsafe extern "C" fn MQRspec_lengthIndicator(
    mut mode: QRencodeMode,
    mut version___0: libc::c_int,
) -> libc::c_int {
    return lengthTableBits___0[mode as usize][(version___0 - 1 as libc::c_int) as usize];
}
pub unsafe extern "C" fn MQRspec_maximumWords(
    mut mode: QRencodeMode,
    mut version___0: libc::c_int,
) -> libc::c_int {
    let mut bits: libc::c_int = 0;
    let mut words: libc::c_int = 0;
    bits = lengthTableBits___0[mode as usize][(version___0 - 1 as libc::c_int) as usize];
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
    mut version___0: libc::c_int,
    mut level___0: QRecLevel,
) -> libc::c_uint {
    let mut type_0: libc::c_int = 0;
    if mask < 0 as libc::c_int {
        return 0 as libc::c_uint
    } else {
        if mask > 3 as libc::c_int {
            return 0 as libc::c_uint;
        }
    }
    if version___0 <= 0 as libc::c_int {
        return 0 as libc::c_uint
    } else {
        if version___0 > 4 as libc::c_int {
            return 0 as libc::c_uint;
        }
    }
    if level___0 as libc::c_uint == 3 as libc::c_uint {
        return 0 as libc::c_uint;
    }
    type_0 = typeTable[version___0 as usize][level___0 as usize];
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
    mut version___0: libc::c_int,
) -> *mut libc::c_uchar {
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut q: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut width: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    width = mqrspecCapacity[version___0 as usize].width;
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
    mut version___0: libc::c_int,
) -> *mut libc::c_uchar {
    let mut tmp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if version___0 < 1 as libc::c_int {
        return 0 as *mut libc::c_void as *mut libc::c_uchar
    } else {
        if version___0 > 4 as libc::c_int {
            return 0 as *mut libc::c_void as *mut libc::c_uchar;
        }
    }
    tmp = MQRspec_createFrame(version___0);
    return tmp;
}
unsafe extern "C" fn MMask_writeFormatInformation(
    mut version___0: libc::c_int,
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut mask: libc::c_int,
    mut level___0: QRecLevel,
) {
    let mut format: libc::c_uint = 0;
    let mut v: libc::c_uchar = 0;
    let mut i: libc::c_int = 0;
    format = MQRspec_getFormatInfo(mask, version___0, level___0);
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
pub unsafe extern "C" fn MMask_makeMask(
    mut version___0: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut mask: libc::c_int,
    mut level___0: QRecLevel,
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
    width = MQRspec_getWidth(version___0);
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
    MMask_writeFormatInformation(version___0, width, masked, mask, level___0);
    return masked;
}
unsafe extern "C" fn MMask_evaluateSymbol(
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
    mut version___0: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut level___0: QRecLevel,
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
    width = MQRspec_getWidth(version___0);
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
        MMask_writeFormatInformation(version___0, width, mask, i, level___0);
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
