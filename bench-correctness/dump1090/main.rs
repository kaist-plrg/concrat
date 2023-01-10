#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(asm, c_variadic, extern_types, register_tool, rustc_private, untagged_unions)]
#[macro_use]
extern crate c2rust_asm_casts;
use ::c2rust_out::*;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type rtlsdr_dev;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtoll(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_longlong;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn round(_: libc::c_double) -> libc::c_double;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn signal(
        __sig: libc::c_int,
        __handler: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> __sighandler_t;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn rtlsdr_get_device_count() -> uint32_t;
    fn rtlsdr_get_device_usb_strings(
        index: uint32_t,
        manufact: *mut libc::c_char,
        product: *mut libc::c_char,
        serial: *mut libc::c_char,
    ) -> libc::c_int;
    fn rtlsdr_open(dev: *mut *mut rtlsdr_dev_t, index: uint32_t) -> libc::c_int;
    fn rtlsdr_close(dev: *mut rtlsdr_dev_t) -> libc::c_int;
    fn rtlsdr_set_center_freq(dev: *mut rtlsdr_dev_t, freq: uint32_t) -> libc::c_int;
    fn rtlsdr_set_freq_correction(
        dev: *mut rtlsdr_dev_t,
        ppm: libc::c_int,
    ) -> libc::c_int;
    fn rtlsdr_get_tuner_gains(
        dev: *mut rtlsdr_dev_t,
        gains: *mut libc::c_int,
    ) -> libc::c_int;
    fn rtlsdr_set_tuner_gain(dev: *mut rtlsdr_dev_t, gain: libc::c_int) -> libc::c_int;
    fn rtlsdr_get_tuner_gain(dev: *mut rtlsdr_dev_t) -> libc::c_int;
    fn rtlsdr_set_tuner_gain_mode(
        dev: *mut rtlsdr_dev_t,
        manual: libc::c_int,
    ) -> libc::c_int;
    fn rtlsdr_set_sample_rate(dev: *mut rtlsdr_dev_t, rate: uint32_t) -> libc::c_int;
    fn rtlsdr_set_agc_mode(dev: *mut rtlsdr_dev_t, on: libc::c_int) -> libc::c_int;
    fn rtlsdr_reset_buffer(dev: *mut rtlsdr_dev_t) -> libc::c_int;
    fn rtlsdr_read_async(
        dev: *mut rtlsdr_dev_t,
        cb: Option::<
            unsafe extern "C" fn(*mut libc::c_uchar, uint32_t, *mut libc::c_void) -> (),
        >,
        ctx: *mut libc::c_void,
        buf_num: uint32_t,
        buf_len: uint32_t,
    ) -> libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t) -> libc::c_int;
    fn getsockname(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __len: *mut socklen_t,
    ) -> libc::c_int;
    fn connect(
        __fd: libc::c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> libc::c_int;
    fn getpeername(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __len: *mut socklen_t,
    ) -> libc::c_int;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn accept(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
    fn inet_aton(__cp: *const libc::c_char, __inp: *mut in_addr) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn gethostbyname(__name: *const libc::c_char) -> *mut hostent;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type __gnuc_va_list = __builtin_va_list;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_fd_set_356711149 {
    pub __fds_bits: [__fd_mask; 16],
}
pub type fd_set = __anonstruct_fd_set_356711149;
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
pub struct __anonstruct___wseq32_112954846 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_506248102 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: __anonstruct___wseq32_112954846,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___g1_start32_112954847 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_100370678 {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: __anonstruct___g1_start32_112954847,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __annonCompField1: __anonunion____missing_field_name_506248102,
    pub __annonCompField2: __anonunion____missing_field_name_100370678,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_mutexattr_t_488594144 {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
pub type pthread_mutexattr_t = __anonunion_pthread_mutexattr_t_488594144;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_condattr_t_488594145 {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
pub type pthread_condattr_t = __anonunion_pthread_condattr_t_488594145;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_mutex_t_335460617 {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type pthread_mutex_t = __anonunion_pthread_mutex_t_335460617;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_cond_t_951761805 {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type pthread_cond_t = __anonunion_pthread_cond_t_951761805;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
pub type rtlsdr_dev_t = rtlsdr_dev;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client {
    pub fd: libc::c_int,
    pub service: libc::c_int,
    pub buf: [libc::c_char; 1025],
    pub buflen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aircraft {
    pub addr: uint32_t,
    pub hexaddr: [libc::c_char; 7],
    pub flight: [libc::c_char; 9],
    pub altitude: libc::c_int,
    pub speed: libc::c_int,
    pub track: libc::c_int,
    pub seen: time_t,
    pub messages: libc::c_long,
    pub odd_cprlat: libc::c_int,
    pub odd_cprlon: libc::c_int,
    pub even_cprlat: libc::c_int,
    pub even_cprlon: libc::c_int,
    pub lat: libc::c_double,
    pub lon: libc::c_double,
    pub odd_cprtime: libc::c_longlong,
    pub even_cprtime: libc::c_longlong,
    pub next: *mut aircraft,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_Modes_120292704 {
    pub reader_thread: pthread_t,
    pub data_mutex: pthread_mutex_t,
    pub data_cond: pthread_cond_t,
    pub data: *mut libc::c_uchar,
    pub magnitude: *mut uint16_t,
    pub data_len: uint32_t,
    pub fd: libc::c_int,
    pub data_ready: libc::c_int,
    pub icao_cache: *mut uint32_t,
    pub maglut: *mut uint16_t,
    pub exit: libc::c_int,
    pub dev_index: libc::c_int,
    pub gain: libc::c_int,
    pub enable_agc: libc::c_int,
    pub dev: *mut rtlsdr_dev_t,
    pub freq: libc::c_int,
    pub aneterr: [libc::c_char; 256],
    pub clients: [*mut client; 1024],
    pub maxfd: libc::c_int,
    pub sbsos: libc::c_int,
    pub ros: libc::c_int,
    pub ris: libc::c_int,
    pub https: libc::c_int,
    pub filename: *mut libc::c_char,
    pub loop_0: libc::c_int,
    pub fix_errors: libc::c_int,
    pub check_crc: libc::c_int,
    pub raw: libc::c_int,
    pub debug: libc::c_int,
    pub net: libc::c_int,
    pub net_only: libc::c_int,
    pub interactive: libc::c_int,
    pub interactive_rows: libc::c_int,
    pub interactive_ttl: libc::c_int,
    pub stats: libc::c_int,
    pub onlyaddr: libc::c_int,
    pub metric: libc::c_int,
    pub aggressive: libc::c_int,
    pub aircrafts: *mut aircraft,
    pub interactive_last_update: libc::c_longlong,
    pub stat_valid_preamble: libc::c_longlong,
    pub stat_demodulated: libc::c_longlong,
    pub stat_goodcrc: libc::c_longlong,
    pub stat_badcrc: libc::c_longlong,
    pub stat_fixed: libc::c_longlong,
    pub stat_single_bit_fix: libc::c_longlong,
    pub stat_two_bits_fix: libc::c_longlong,
    pub stat_http_requests: libc::c_longlong,
    pub stat_sbs_connections: libc::c_longlong,
    pub stat_out_of_phase: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct modesMessage {
    pub msg: [libc::c_uchar; 14],
    pub msgbits: libc::c_int,
    pub msgtype: libc::c_int,
    pub crcok: libc::c_int,
    pub crc: uint32_t,
    pub errorbit: libc::c_int,
    pub aa1: libc::c_int,
    pub aa2: libc::c_int,
    pub aa3: libc::c_int,
    pub phase_corrected: libc::c_int,
    pub ca: libc::c_int,
    pub metype: libc::c_int,
    pub mesub: libc::c_int,
    pub heading_is_valid: libc::c_int,
    pub heading: libc::c_int,
    pub aircraft_type: libc::c_int,
    pub fflag: libc::c_int,
    pub tflag: libc::c_int,
    pub raw_latitude: libc::c_int,
    pub raw_longitude: libc::c_int,
    pub flight: [libc::c_char; 9],
    pub ew_dir: libc::c_int,
    pub ew_velocity: libc::c_int,
    pub ns_dir: libc::c_int,
    pub ns_velocity: libc::c_int,
    pub vert_rate_source: libc::c_int,
    pub vert_rate_sign: libc::c_int,
    pub vert_rate: libc::c_int,
    pub velocity: libc::c_int,
    pub fs: libc::c_int,
    pub dr: libc::c_int,
    pub um: libc::c_int,
    pub identity: libc::c_int,
    pub altitude: libc::c_int,
    pub unit: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_modesNetServices_483049182 {
    pub descr: *mut libc::c_char,
    pub socket: *mut libc::c_int,
    pub port: libc::c_int,
}
pub type __socklen_t = libc::c_uint;
pub type mode_t = __mode_t;
pub type socklen_t = __socklen_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostent {
    pub h_name: *mut libc::c_char,
    pub h_aliases: *mut *mut libc::c_char,
    pub h_addrtype: libc::c_int,
    pub h_length: libc::c_int,
    pub h_addr_list: *mut *mut libc::c_char,
}
pub type va_list___0 = __gnuc_va_list;
#[inline]
unsafe extern "C" fn getchar() -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = getc(stdin);
    return tmp;
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = putc(__c, stdout);
    return tmp;
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    let mut tmp: libc::c_long = 0;
    tmp = strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    return tmp as libc::c_int;
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    let mut tmp: libc::c_double = 0.;
    tmp = strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
    return tmp;
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    let mut tmp: *mut *const __int32_t = 0 as *mut *const __int32_t;
    let mut tmp___0: __int32_t = 0;
    if __c >= -(128 as libc::c_int) {
        if __c < 256 as libc::c_int {
            tmp = __ctype_tolower_loc();
            tmp___0 = *(*tmp).offset(__c as isize);
        } else {
            tmp___0 = __c;
        }
    } else {
        tmp___0 = __c;
    }
    return tmp___0;
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = __xstat(1 as libc::c_int, __path, __statbuf);
    return tmp;
}
pub static mut Modes: __anonstruct_Modes_120292704 = __anonstruct_Modes_120292704 {
    reader_thread: 0,
    data_mutex: __anonunion_pthread_mutex_t_335460617 {
        __data: __pthread_mutex_s {
            __lock: 0,
            __count: 0,
            __owner: 0,
            __nusers: 0,
            __kind: 0,
            __spins: 0,
            __elision: 0,
            __list: __pthread_list_t {
                __prev: 0 as *const __pthread_internal_list
                    as *mut __pthread_internal_list,
                __next: 0 as *const __pthread_internal_list
                    as *mut __pthread_internal_list,
            },
        },
    },
    data_cond: __anonunion_pthread_cond_t_951761805 {
        __data: __pthread_cond_s {
            __annonCompField1: __anonunion____missing_field_name_506248102 {
                __wseq: 0,
            },
            __annonCompField2: __anonunion____missing_field_name_100370678 {
                __g1_start: 0,
            },
            __g_refs: [0; 2],
            __g_size: [0; 2],
            __g1_orig_size: 0,
            __wrefs: 0,
            __g_signals: [0; 2],
        },
    },
    data: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    magnitude: 0 as *const uint16_t as *mut uint16_t,
    data_len: 0,
    fd: 0,
    data_ready: 0,
    icao_cache: 0 as *const uint32_t as *mut uint32_t,
    maglut: 0 as *const uint16_t as *mut uint16_t,
    exit: 0,
    dev_index: 0,
    gain: 0,
    enable_agc: 0,
    dev: 0 as *const rtlsdr_dev_t as *mut rtlsdr_dev_t,
    freq: 0,
    aneterr: [0; 256],
    clients: [0 as *const client as *mut client; 1024],
    maxfd: 0,
    sbsos: 0,
    ros: 0,
    ris: 0,
    https: 0,
    filename: 0 as *const libc::c_char as *mut libc::c_char,
    loop_0: 0,
    fix_errors: 0,
    check_crc: 0,
    raw: 0,
    debug: 0,
    net: 0,
    net_only: 0,
    interactive: 0,
    interactive_rows: 0,
    interactive_ttl: 0,
    stats: 0,
    onlyaddr: 0,
    metric: 0,
    aggressive: 0,
    aircrafts: 0 as *const aircraft as *mut aircraft,
    interactive_last_update: 0,
    stat_valid_preamble: 0,
    stat_demodulated: 0,
    stat_goodcrc: 0,
    stat_badcrc: 0,
    stat_fixed: 0,
    stat_single_bit_fix: 0,
    stat_two_bits_fix: 0,
    stat_http_requests: 0,
    stat_sbs_connections: 0,
    stat_out_of_phase: 0,
};
unsafe extern "C" fn mstime() -> libc::c_longlong {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut mst: libc::c_longlong = 0;
    gettimeofday(&mut tv as *mut timeval, 0 as *mut libc::c_void);
    mst = tv.tv_sec as libc::c_longlong * 1000 as libc::c_longlong;
    mst += (tv.tv_usec / 1000 as libc::c_long) as libc::c_longlong;
    return mst;
}
pub unsafe extern "C" fn modesInitConfig() {
    Modes.gain = 999999 as libc::c_int;
    Modes.dev_index = 0 as libc::c_int;
    Modes.enable_agc = 0 as libc::c_int;
    Modes.freq = 1090000000 as libc::c_int;
    Modes.filename = 0 as *mut libc::c_void as *mut libc::c_char;
    Modes.fix_errors = 1 as libc::c_int;
    Modes.check_crc = 1 as libc::c_int;
    Modes.raw = 0 as libc::c_int;
    Modes.net = 0 as libc::c_int;
    Modes.net_only = 0 as libc::c_int;
    Modes.onlyaddr = 0 as libc::c_int;
    Modes.debug = 0 as libc::c_int;
    Modes.interactive = 0 as libc::c_int;
    Modes.interactive_rows = 15 as libc::c_int;
    Modes.interactive_ttl = 60 as libc::c_int;
    Modes.aggressive = 0 as libc::c_int;
    Modes.interactive_rows = getTermRows();
    Modes.loop_0 = 0 as libc::c_int;
}
pub unsafe extern "C" fn modesInit() {
    let mut i: libc::c_int = 0;
    let mut q: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: libc::c_double = 0.;
    let mut tmp___4: libc::c_double = 0.;
    pthread_mutex_init(
        &mut Modes.data_mutex,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
    pthread_cond_init(
        &mut Modes.data_cond as *mut pthread_cond_t,
        0 as *mut libc::c_void as *const pthread_condattr_t,
    );
    Modes.data_len = 262620 as libc::c_int as uint32_t;
    Modes.data_ready = 0 as libc::c_int;
    tmp = malloc(
        (::std::mem::size_of::<uint32_t>() as libc::c_ulong)
            .wrapping_mul(1024 as libc::c_ulong)
            .wrapping_mul(2 as libc::c_ulong),
    );
    Modes.icao_cache = tmp as *mut uint32_t;
    memset(
        Modes.icao_cache as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<uint32_t>() as libc::c_ulong)
            .wrapping_mul(1024 as libc::c_ulong)
            .wrapping_mul(2 as libc::c_ulong),
    );
    Modes.aircrafts = 0 as *mut libc::c_void as *mut aircraft;
    Modes.interactive_last_update = 0 as libc::c_longlong;
    tmp___0 = malloc(Modes.data_len as size_t);
    Modes.data = tmp___0 as *mut libc::c_uchar;
    if Modes.data as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"Out of memory allocating data buffer.\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    } else {
        tmp___1 = malloc((Modes.data_len).wrapping_mul(2 as libc::c_uint) as size_t);
        Modes.magnitude = tmp___1 as *mut uint16_t;
        if Modes.magnitude as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            fprintf(
                stderr,
                b"Out of memory allocating data buffer.\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    memset(
        Modes.data as *mut libc::c_void,
        127 as libc::c_int,
        Modes.data_len as size_t,
    );
    tmp___2 = malloc(33282 as libc::c_int as size_t);
    Modes.maglut = tmp___2 as *mut uint16_t;
    i = 0 as libc::c_int;
    while i <= 128 as libc::c_int {
        q = 0 as libc::c_int;
        while q <= 128 as libc::c_int {
            tmp___3 = sqrt((i * i + q * q) as libc::c_double);
            tmp___4 = round(tmp___3 * 360 as libc::c_int as libc::c_double);
            *(Modes.maglut)
                .offset((i * 129 as libc::c_int + q) as isize) = tmp___4 as uint16_t;
            q += 1;
        }
        i += 1;
    }
    Modes.stat_valid_preamble = 0 as libc::c_longlong;
    Modes.stat_demodulated = 0 as libc::c_longlong;
    Modes.stat_goodcrc = 0 as libc::c_longlong;
    Modes.stat_badcrc = 0 as libc::c_longlong;
    Modes.stat_fixed = 0 as libc::c_longlong;
    Modes.stat_single_bit_fix = 0 as libc::c_longlong;
    Modes.stat_two_bits_fix = 0 as libc::c_longlong;
    Modes.stat_http_requests = 0 as libc::c_longlong;
    Modes.stat_sbs_connections = 0 as libc::c_longlong;
    Modes.stat_out_of_phase = 0 as libc::c_longlong;
    Modes.exit = 0 as libc::c_int;
}
pub unsafe extern "C" fn modesInitRTLSDR() {
    let mut j: libc::c_int = 0;
    let mut device_count: libc::c_int = 0;
    let mut ppm_error: libc::c_int = 0;
    let mut vendor: [libc::c_char; 256] = [0; 256];
    let mut product: [libc::c_char; 256] = [0; 256];
    let mut serial: [libc::c_char; 256] = [0; 256];
    let mut tmp: uint32_t = 0;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut numgains: libc::c_int = 0;
    let mut gains: [libc::c_int; 100] = [0; 100];
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    ppm_error = 0 as libc::c_int;
    tmp = rtlsdr_get_device_count();
    device_count = tmp as libc::c_int;
    if device_count == 0 {
        fprintf(
            stderr,
            b"No supported RTLSDR devices found.\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    fprintf(
        stderr,
        b"Found %d device(s):\n\0" as *const u8 as *const libc::c_char,
        device_count,
    );
    j = 0 as libc::c_int;
    while j < device_count {
        rtlsdr_get_device_usb_strings(
            j as uint32_t,
            vendor.as_mut_ptr(),
            product.as_mut_ptr(),
            serial.as_mut_ptr(),
        );
        if j == Modes.dev_index {
            tmp___0 = b"(currently selected)\0" as *const u8 as *const libc::c_char;
        } else {
            tmp___0 = b"\0" as *const u8 as *const libc::c_char;
        }
        fprintf(
            stderr,
            b"%d: %s, %s, SN: %s %s\n\0" as *const u8 as *const libc::c_char,
            j,
            vendor.as_mut_ptr(),
            product.as_mut_ptr(),
            serial.as_mut_ptr(),
            tmp___0,
        );
        j += 1;
    }
    tmp___3 = rtlsdr_open(&mut Modes.dev, Modes.dev_index as uint32_t);
    if tmp___3 < 0 as libc::c_int {
        tmp___1 = __errno_location();
        tmp___2 = strerror(*tmp___1);
        fprintf(
            stderr,
            b"Error opening the RTLSDR device: %s\n\0" as *const u8
                as *const libc::c_char,
            tmp___2,
        );
        exit(1 as libc::c_int);
    }
    if Modes.gain == -(100 as libc::c_int) {
        tmp___4 = 0 as libc::c_int;
    } else {
        tmp___4 = 1 as libc::c_int;
    }
    rtlsdr_set_tuner_gain_mode(Modes.dev, tmp___4);
    if Modes.gain != -(100 as libc::c_int) {
        if Modes.gain == 999999 as libc::c_int {
            tmp___5 = rtlsdr_get_tuner_gains(Modes.dev, gains.as_mut_ptr());
            numgains = tmp___5;
            Modes.gain = gains[(numgains - 1 as libc::c_int) as usize];
            fprintf(
                stderr,
                b"Max available gain is: %.2f\n\0" as *const u8 as *const libc::c_char,
                Modes.gain as libc::c_double / 10.0f64,
            );
        }
        rtlsdr_set_tuner_gain(Modes.dev, Modes.gain);
        fprintf(
            stderr,
            b"Setting gain to: %.2f\n\0" as *const u8 as *const libc::c_char,
            Modes.gain as libc::c_double / 10.0f64,
        );
    } else {
        fprintf(
            stderr,
            b"Using automatic gain control.\n\0" as *const u8 as *const libc::c_char,
        );
    }
    rtlsdr_set_freq_correction(Modes.dev, ppm_error);
    if Modes.enable_agc != 0 {
        rtlsdr_set_agc_mode(Modes.dev, 1 as libc::c_int);
    }
    rtlsdr_set_center_freq(Modes.dev, Modes.freq as uint32_t);
    rtlsdr_set_sample_rate(Modes.dev, 2000000 as libc::c_int as uint32_t);
    rtlsdr_reset_buffer(Modes.dev);
    tmp___6 = rtlsdr_get_tuner_gain(Modes.dev);
    fprintf(
        stderr,
        b"Gain reported by device: %.2f\n\0" as *const u8 as *const libc::c_char,
        tmp___6 as libc::c_double / 10.0f64,
    );
}
pub unsafe extern "C" fn rtlsdrCallback(
    mut buf: *mut libc::c_uchar,
    mut len: uint32_t,
    mut ctx: *mut libc::c_void,
) {
    pthread_mutex_lock(&mut Modes.data_mutex);
    if len > 262144 as libc::c_uint {
        len = 262144 as libc::c_int as uint32_t;
    }
    memcpy(
        Modes.data as *mut libc::c_void,
        (Modes.data).offset(262144 as libc::c_int as isize) as *const libc::c_void,
        476 as libc::c_int as size_t,
    );
    memcpy(
        (Modes.data).offset(476 as libc::c_int as isize) as *mut libc::c_void,
        buf as *const libc::c_void,
        len as size_t,
    );
    Modes.data_ready = 1 as libc::c_int;
    pthread_cond_signal(&mut Modes.data_cond);
    pthread_mutex_unlock(&mut Modes.data_mutex);
}
pub unsafe extern "C" fn readDataFromFile() {
    let mut nread: ssize_t = 0;
    let mut toread: ssize_t = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp: __off_t = 0;
    pthread_mutex_lock(&mut Modes.data_mutex);
    loop {
        if Modes.data_ready != 0 {
            pthread_cond_wait(
                &mut Modes.data_cond as *mut pthread_cond_t,
                &mut Modes.data_mutex as *mut pthread_mutex_t,
            );
        } else {
            if Modes.interactive != 0 {
                pthread_mutex_unlock(&mut Modes.data_mutex);
                usleep(5000 as libc::c_int as __useconds_t);
                pthread_mutex_lock(&mut Modes.data_mutex);
            }
            memcpy(
                Modes.data as *mut libc::c_void,
                (Modes.data).offset(262144 as libc::c_int as isize)
                    as *const libc::c_void,
                476 as libc::c_int as size_t,
            );
            toread = 262144 as libc::c_int as ssize_t;
            p = (Modes.data).offset(476 as libc::c_int as isize);
            while toread != 0 {
                nread = read(Modes.fd, p as *mut libc::c_void, toread as size_t);
                if nread == 0 as libc::c_long {
                    if Modes.filename as libc::c_ulong
                        != 0 as *mut libc::c_void as libc::c_ulong
                    {
                        if Modes.fd != 0 as libc::c_int {
                            if Modes.loop_0 != 0 {
                                tmp = lseek(
                                    Modes.fd,
                                    0 as libc::c_int as __off_t,
                                    0 as libc::c_int,
                                );
                                if tmp != -(1 as libc::c_long) {
                                    continue;
                                }
                            }
                        }
                    }
                }
                if nread <= 0 as libc::c_long {
                    Modes.exit = 1 as libc::c_int;
                    break;
                } else {
                    p = p.offset(nread as isize);
                    toread -= nread;
                }
            }
            if toread != 0 {
                memset(p as *mut libc::c_void, 127 as libc::c_int, toread as size_t);
            }
            Modes.data_ready = 1 as libc::c_int;
            pthread_cond_signal(&mut Modes.data_cond);
        }
    };
}
pub unsafe extern "C" fn readerThreadEntryPoint(
    mut arg: *mut libc::c_void,
) -> *mut libc::c_void {
    if Modes.filename as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        rtlsdr_read_async(
            Modes.dev,
            Some(
                rtlsdrCallback
                    as unsafe extern "C" fn(
                        *mut libc::c_uchar,
                        uint32_t,
                        *mut libc::c_void,
                    ) -> (),
            ),
            0 as *mut libc::c_void,
            12 as libc::c_int as uint32_t,
            262144 as libc::c_int as uint32_t,
        );
    } else {
        readDataFromFile();
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn dumpMagnitudeBar(
    mut index___0: libc::c_int,
    mut magnitude: libc::c_int,
) {
    let mut set: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut div___0: libc::c_int = 0;
    let mut rem: libc::c_int = 0;
    let mut markchar: libc::c_int = 0;
    set = b" .-o\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    div___0 = magnitude / 256 as libc::c_int / 4 as libc::c_int;
    rem = magnitude / 256 as libc::c_int % 4 as libc::c_int;
    memset(buf.as_mut_ptr() as *mut libc::c_void, 'O' as i32, div___0 as size_t);
    buf[div___0 as usize] = *set.offset(rem as isize);
    buf[(div___0 + 1 as libc::c_int) as usize] = '\u{0}' as i32 as libc::c_char;
    if index___0 >= 0 as libc::c_int {
        markchar = ']' as i32;
        if index___0 == 0 as libc::c_int {
            markchar = '>' as i32;
        } else if index___0 == 2 as libc::c_int {
            markchar = '>' as i32;
        } else if index___0 == 7 as libc::c_int {
            markchar = '>' as i32;
        } else if index___0 == 9 as libc::c_int {
            markchar = '>' as i32;
        }
        if index___0 >= 16 as libc::c_int {
            if (index___0 - 16 as libc::c_int) / 2 as libc::c_int & 1 as libc::c_int != 0
            {
                markchar = '|' as i32;
            } else {
                markchar = ')' as i32;
            }
        }
        printf(
            b"[%.3d%c |%-66s %d\n\0" as *const u8 as *const libc::c_char,
            index___0,
            markchar,
            buf.as_mut_ptr(),
            magnitude,
        );
    } else {
        printf(
            b"[%.2d] |%-66s %d\n\0" as *const u8 as *const libc::c_char,
            index___0,
            buf.as_mut_ptr(),
            magnitude,
        );
    };
}
pub unsafe extern "C" fn dumpMagnitudeVector(
    mut m: *mut uint16_t,
    mut offset: uint32_t,
) {
    let mut padding: uint32_t = 0;
    let mut start: uint32_t = 0;
    let mut tmp: uint32_t = 0;
    let mut end: uint32_t = 0;
    let mut j: uint32_t = 0;
    padding = 5 as libc::c_int as uint32_t;
    if offset < padding {
        tmp = 0 as libc::c_int as uint32_t;
    } else {
        tmp = offset.wrapping_sub(padding);
    }
    start = tmp;
    end = offset
        .wrapping_add(16 as libc::c_uint)
        .wrapping_add(112 as libc::c_uint)
        .wrapping_sub(1 as libc::c_uint);
    j = start;
    while j <= end {
        dumpMagnitudeBar(
            j.wrapping_sub(offset) as libc::c_int,
            *m.offset(j as isize) as libc::c_int,
        );
        j = j.wrapping_add(1);
    }
}
pub unsafe extern "C" fn dumpRawMessageJS(
    mut descr: *mut libc::c_char,
    mut msg: *mut libc::c_uchar,
    mut m: *mut uint16_t,
    mut offset: uint32_t,
    mut fixable: libc::c_int,
) {
    let mut padding: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut j: libc::c_int = 0;
    let mut fix1: libc::c_int = 0;
    let mut fix2: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    padding = 5 as libc::c_int;
    start = offset.wrapping_sub(padding as uint32_t) as libc::c_int;
    end = offset
        .wrapping_add(16 as libc::c_uint)
        .wrapping_add(224 as libc::c_uint)
        .wrapping_sub(1 as libc::c_uint) as libc::c_int;
    fix1 = -(1 as libc::c_int);
    fix2 = -(1 as libc::c_int);
    if fixable != -(1 as libc::c_int) {
        fix1 = fixable & 255 as libc::c_int;
        if fixable > 255 as libc::c_int {
            fix2 = fixable >> 8 as libc::c_int;
        }
    }
    fp = fopen(
        b"frames.js\0" as *const u8 as *const libc::c_char,
        b"a\0" as *const u8 as *const libc::c_char,
    );
    if fp as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp = __errno_location();
        tmp___0 = strerror(*tmp);
        fprintf(
            stderr,
            b"Error opening frames.js: %s\n\0" as *const u8 as *const libc::c_char,
            tmp___0,
        );
        exit(1 as libc::c_int);
    }
    fprintf(
        fp,
        b"frames.push({\"descr\": \"%s\", \"mag\": [\0" as *const u8
            as *const libc::c_char,
        descr,
    );
    j = start;
    while j <= end {
        if j < 0 as libc::c_int {
            tmp___1 = 0 as libc::c_int;
        } else {
            tmp___1 = *m.offset(j as isize) as libc::c_int;
        }
        fprintf(fp, b"%d\0" as *const u8 as *const libc::c_char, tmp___1);
        if j != end {
            fprintf(fp, b",\0" as *const u8 as *const libc::c_char);
        }
        j += 1;
    }
    tmp___2 = modesMessageLenByType(
        *msg.offset(0 as libc::c_int as isize) as libc::c_int >> 3 as libc::c_int,
    );
    fprintf(
        fp,
        b"], \"fix1\": %d, \"fix2\": %d, \"bits\": %d, \"hex\": \"\0" as *const u8
            as *const libc::c_char,
        fix1,
        fix2,
        tmp___2,
    );
    j = 0 as libc::c_int;
    while j < 14 as libc::c_int {
        fprintf(
            fp,
            b"\\x%02x\0" as *const u8 as *const libc::c_char,
            *msg.offset(j as isize) as libc::c_int,
        );
        j += 1;
    }
    fprintf(fp, b"\"});\n\0" as *const u8 as *const libc::c_char);
    fclose(fp);
}
pub unsafe extern "C" fn dumpRawMessage(
    mut descr: *mut libc::c_char,
    mut msg: *mut libc::c_uchar,
    mut m: *mut uint16_t,
    mut offset: uint32_t,
) {
    let mut j: libc::c_int = 0;
    let mut msgtype: libc::c_int = 0;
    let mut fixable: libc::c_int = 0;
    let mut msgbits: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    msgtype = *msg.offset(0 as libc::c_int as isize) as libc::c_int >> 3 as libc::c_int;
    fixable = -(1 as libc::c_int);
    let mut current_block_11: u64;
    if msgtype == 11 as libc::c_int {
        current_block_11 = 16405154944230087110;
    } else if msgtype == 17 as libc::c_int {
        current_block_11 = 16405154944230087110;
    } else {
        current_block_11 = 3512920355445576850;
    }
    match current_block_11 {
        16405154944230087110 => {
            if msgtype == 11 as libc::c_int {
                tmp = 56 as libc::c_int;
            } else {
                tmp = 112 as libc::c_int;
            }
            msgbits = tmp;
            fixable = fixSingleBitErrors(msg, msgbits);
            if fixable == -(1 as libc::c_int) {
                fixable = fixTwoBitsErrors(msg, msgbits);
            }
        }
        _ => {}
    }
    if Modes.debug & (1 as libc::c_int) << 6 as libc::c_int != 0 {
        dumpRawMessageJS(descr, msg, m, offset, fixable);
        return;
    }
    printf(b"\n--- %s\n    \0" as *const u8 as *const libc::c_char, descr);
    j = 0 as libc::c_int;
    while j < 14 as libc::c_int {
        printf(
            b"%02x\0" as *const u8 as *const libc::c_char,
            *msg.offset(j as isize) as libc::c_int,
        );
        if j == 6 as libc::c_int {
            printf(b" ... \0" as *const u8 as *const libc::c_char);
        }
        j += 1;
    }
    printf(
        b" (DF %d, Fixable: %d)\n\0" as *const u8 as *const libc::c_char,
        msgtype,
        fixable,
    );
    dumpMagnitudeVector(m, offset);
    printf(b"---\n\n\0" as *const u8 as *const libc::c_char);
}
pub static mut modes_checksum_table: [uint32_t; 112] = [
    3749354 as libc::c_int as uint32_t,
    1874677 as libc::c_int as uint32_t,
    15841150 as libc::c_int as uint32_t,
    7920575 as libc::c_int as uint32_t,
    12818395 as libc::c_int as uint32_t,
    10367465 as libc::c_int as uint32_t,
    11592432 as libc::c_int as uint32_t,
    5796216 as libc::c_int as uint32_t,
    2898108 as libc::c_int as uint32_t,
    1449054 as libc::c_int as uint32_t,
    724527 as libc::c_int as uint32_t,
    16416019 as libc::c_int as uint32_t,
    8569997 as libc::c_int as uint32_t,
    12490818 as libc::c_int as uint32_t,
    6245409 as libc::c_int as uint32_t,
    13655060 as libc::c_int as uint32_t,
    6827530 as libc::c_int as uint32_t,
    3413765 as libc::c_int as uint32_t,
    15069574 as libc::c_int as uint32_t,
    7534787 as libc::c_int as uint32_t,
    13010533 as libc::c_int as uint32_t,
    10271030 as libc::c_int as uint32_t,
    5135515 as libc::c_int as uint32_t,
    14210121 as libc::c_int as uint32_t,
    9670688 as libc::c_int as uint32_t,
    4835344 as libc::c_int as uint32_t,
    2417672 as libc::c_int as uint32_t,
    1208836 as libc::c_int as uint32_t,
    604418 as libc::c_int as uint32_t,
    302209 as libc::c_int as uint32_t,
    16626756 as libc::c_int as uint32_t,
    8313378 as libc::c_int as uint32_t,
    4156689 as libc::c_int as uint32_t,
    14699660 as libc::c_int as uint32_t,
    7349830 as libc::c_int as uint32_t,
    3674915 as libc::c_int as uint32_t,
    14939029 as libc::c_int as uint32_t,
    9307086 as libc::c_int as uint32_t,
    4653543 as libc::c_int as uint32_t,
    14449399 as libc::c_int as uint32_t,
    9553791 as libc::c_int as uint32_t,
    11999675 as libc::c_int as uint32_t,
    10778329 as libc::c_int as uint32_t,
    11387240 as libc::c_int as uint32_t,
    5693620 as libc::c_int as uint32_t,
    2846810 as libc::c_int as uint32_t,
    1423405 as libc::c_int as uint32_t,
    16066066 as libc::c_int as uint32_t,
    8033033 as libc::c_int as uint32_t,
    12759936 as libc::c_int as uint32_t,
    6379968 as libc::c_int as uint32_t,
    3189984 as libc::c_int as uint32_t,
    1594992 as libc::c_int as uint32_t,
    797496 as libc::c_int as uint32_t,
    398748 as libc::c_int as uint32_t,
    199374 as libc::c_int as uint32_t,
    99687 as libc::c_int as uint32_t,
    16726199 as libc::c_int as uint32_t,
    8414815 as libc::c_int as uint32_t,
    12568875 as libc::c_int as uint32_t,
    10493585 as libc::c_int as uint32_t,
    11531596 as libc::c_int as uint32_t,
    5765798 as libc::c_int as uint32_t,
    2882899 as libc::c_int as uint32_t,
    15336621 as libc::c_int as uint32_t,
    9107538 as libc::c_int as uint32_t,
    4553769 as libc::c_int as uint32_t,
    14500880 as libc::c_int as uint32_t,
    7250440 as libc::c_int as uint32_t,
    3625220 as libc::c_int as uint32_t,
    1812610 as libc::c_int as uint32_t,
    906305 as libc::c_int as uint32_t,
    16322596 as libc::c_int as uint32_t,
    8161298 as libc::c_int as uint32_t,
    4080649 as libc::c_int as uint32_t,
    14735360 as libc::c_int as uint32_t,
    7367680 as libc::c_int as uint32_t,
    3683840 as libc::c_int as uint32_t,
    1841920 as libc::c_int as uint32_t,
    920960 as libc::c_int as uint32_t,
    460480 as libc::c_int as uint32_t,
    230240 as libc::c_int as uint32_t,
    115120 as libc::c_int as uint32_t,
    57560 as libc::c_int as uint32_t,
    28780 as libc::c_int as uint32_t,
    14390 as libc::c_int as uint32_t,
    7195 as libc::c_int as uint32_t,
    16774153 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
];
pub unsafe extern "C" fn modesChecksum(
    mut msg: *mut libc::c_uchar,
    mut bits: libc::c_int,
) -> uint32_t {
    let mut crc: uint32_t = 0;
    let mut offset: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut byte: libc::c_int = 0;
    let mut bit: libc::c_int = 0;
    let mut bitmask: libc::c_int = 0;
    crc = 0 as libc::c_int as uint32_t;
    if bits == 112 as libc::c_int {
        tmp = 0 as libc::c_int;
    } else {
        tmp = 56 as libc::c_int;
    }
    offset = tmp;
    j = 0 as libc::c_int;
    while j < bits {
        byte = j / 8 as libc::c_int;
        bit = j % 8 as libc::c_int;
        bitmask = (1 as libc::c_int) << 7 as libc::c_int - bit;
        if *msg.offset(byte as isize) as libc::c_int & bitmask != 0 {
            crc ^= modes_checksum_table[(j + offset) as usize];
        }
        j += 1;
    }
    return crc;
}
pub unsafe extern "C" fn modesMessageLenByType(mut type_0: libc::c_int) -> libc::c_int {
    if type_0 == 16 as libc::c_int {
        return 112 as libc::c_int
    } else if type_0 == 17 as libc::c_int {
        return 112 as libc::c_int
    } else if type_0 == 19 as libc::c_int {
        return 112 as libc::c_int
    } else if type_0 == 20 as libc::c_int {
        return 112 as libc::c_int
    } else if type_0 == 21 as libc::c_int {
        return 112 as libc::c_int
    } else {
        return 56 as libc::c_int
    };
}
pub unsafe extern "C" fn fixSingleBitErrors(
    mut msg: *mut libc::c_uchar,
    mut bits: libc::c_int,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut aux: [libc::c_uchar; 14] = [0; 14];
    let mut byte: libc::c_int = 0;
    let mut bitmask: libc::c_int = 0;
    let mut crc1: uint32_t = 0;
    let mut crc2: uint32_t = 0;
    j = 0 as libc::c_int;
    while j < bits {
        byte = j / 8 as libc::c_int;
        bitmask = (1 as libc::c_int) << 7 as libc::c_int - j % 8 as libc::c_int;
        memcpy(
            aux.as_mut_ptr() as *mut libc::c_void,
            msg as *const libc::c_void,
            (bits / 8 as libc::c_int) as size_t,
        );
        aux[byte
            as usize] = (aux[byte as usize] as libc::c_int ^ bitmask) as libc::c_uchar;
        crc1 = (aux[(bits / 8 as libc::c_int - 3 as libc::c_int) as usize] as uint32_t)
            << 16 as libc::c_int
            | (aux[(bits / 8 as libc::c_int - 2 as libc::c_int) as usize] as uint32_t)
                << 8 as libc::c_int
            | aux[(bits / 8 as libc::c_int - 1 as libc::c_int) as usize] as uint32_t;
        crc2 = modesChecksum(aux.as_mut_ptr(), bits);
        if crc1 == crc2 {
            memcpy(
                msg as *mut libc::c_void,
                aux.as_mut_ptr() as *const libc::c_void,
                (bits / 8 as libc::c_int) as size_t,
            );
            return j;
        }
        j += 1;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn fixTwoBitsErrors(
    mut msg: *mut libc::c_uchar,
    mut bits: libc::c_int,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut aux: [libc::c_uchar; 14] = [0; 14];
    let mut byte1: libc::c_int = 0;
    let mut bitmask1: libc::c_int = 0;
    let mut byte2: libc::c_int = 0;
    let mut bitmask2: libc::c_int = 0;
    let mut crc1: uint32_t = 0;
    let mut crc2: uint32_t = 0;
    j = 0 as libc::c_int;
    while j < bits {
        byte1 = j / 8 as libc::c_int;
        bitmask1 = (1 as libc::c_int) << 7 as libc::c_int - j % 8 as libc::c_int;
        i = j + 1 as libc::c_int;
        while i < bits {
            byte2 = i / 8 as libc::c_int;
            bitmask2 = (1 as libc::c_int) << 7 as libc::c_int - i % 8 as libc::c_int;
            memcpy(
                aux.as_mut_ptr() as *mut libc::c_void,
                msg as *const libc::c_void,
                (bits / 8 as libc::c_int) as size_t,
            );
            aux[byte1
                as usize] = (aux[byte1 as usize] as libc::c_int ^ bitmask1)
                as libc::c_uchar;
            aux[byte2
                as usize] = (aux[byte2 as usize] as libc::c_int ^ bitmask2)
                as libc::c_uchar;
            crc1 = (aux[(bits / 8 as libc::c_int - 3 as libc::c_int) as usize]
                as uint32_t) << 16 as libc::c_int
                | (aux[(bits / 8 as libc::c_int - 2 as libc::c_int) as usize]
                    as uint32_t) << 8 as libc::c_int
                | aux[(bits / 8 as libc::c_int - 1 as libc::c_int) as usize] as uint32_t;
            crc2 = modesChecksum(aux.as_mut_ptr(), bits);
            if crc1 == crc2 {
                memcpy(
                    msg as *mut libc::c_void,
                    aux.as_mut_ptr() as *const libc::c_void,
                    (bits / 8 as libc::c_int) as size_t,
                );
                return j | i << 8 as libc::c_int;
            }
            i += 1;
        }
        j += 1;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn ICAOCacheHashAddress(mut a: uint32_t) -> uint32_t {
    a = (a >> 16 as libc::c_int ^ a).wrapping_mul(73244475 as libc::c_uint);
    a = (a >> 16 as libc::c_int ^ a).wrapping_mul(73244475 as libc::c_uint);
    a = a >> 16 as libc::c_int ^ a;
    return a & 1023 as libc::c_uint;
}
pub unsafe extern "C" fn addRecentlySeenICAOAddr(mut addr: uint32_t) {
    let mut h: uint32_t = 0;
    let mut tmp: uint32_t = 0;
    let mut tmp___0: time_t = 0;
    tmp = ICAOCacheHashAddress(addr);
    h = tmp;
    *(Modes.icao_cache).offset(h.wrapping_mul(2 as libc::c_uint) as isize) = addr;
    tmp___0 = time(0 as *mut libc::c_void as *mut time_t);
    *(Modes.icao_cache)
        .offset(
            h.wrapping_mul(2 as libc::c_uint).wrapping_add(1 as libc::c_uint) as isize,
        ) = tmp___0 as uint32_t;
}
pub unsafe extern "C" fn ICAOAddressWasRecentlySeen(mut addr: uint32_t) -> libc::c_int {
    let mut h: uint32_t = 0;
    let mut tmp: uint32_t = 0;
    let mut a: uint32_t = 0;
    let mut t: uint32_t = 0;
    let mut tmp___0: time_t = 0;
    let mut tmp___1: libc::c_int = 0;
    tmp = ICAOCacheHashAddress(addr);
    h = tmp;
    a = *(Modes.icao_cache).offset(h.wrapping_mul(2 as libc::c_uint) as isize);
    t = *(Modes.icao_cache)
        .offset(
            h.wrapping_mul(2 as libc::c_uint).wrapping_add(1 as libc::c_uint) as isize,
        );
    if a != 0 {
        if a == addr {
            tmp___0 = time(0 as *mut libc::c_void as *mut time_t);
            if tmp___0 - t as time_t <= 60 as libc::c_long {
                tmp___1 = 1 as libc::c_int;
            } else {
                tmp___1 = 0 as libc::c_int;
            }
        } else {
            tmp___1 = 0 as libc::c_int;
        }
    } else {
        tmp___1 = 0 as libc::c_int;
    }
    return tmp___1;
}
pub unsafe extern "C" fn bruteForceAP(
    mut msg: *mut libc::c_uchar,
    mut mm: *mut modesMessage,
) -> libc::c_int {
    let mut current_block: u64;
    let mut aux: [libc::c_uchar; 14] = [0; 14];
    let mut msgtype: libc::c_int = 0;
    let mut msgbits: libc::c_int = 0;
    let mut addr: uint32_t = 0;
    let mut crc: uint32_t = 0;
    let mut lastbyte: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    msgtype = (*mm).msgtype;
    msgbits = (*mm).msgbits;
    if msgtype == 0 as libc::c_int {
        current_block = 890779300656278608;
    } else if msgtype == 4 as libc::c_int {
        current_block = 890779300656278608;
    } else if msgtype == 5 as libc::c_int {
        current_block = 890779300656278608;
    } else if msgtype == 16 as libc::c_int {
        current_block = 890779300656278608;
    } else if msgtype == 20 as libc::c_int {
        current_block = 890779300656278608;
    } else if msgtype == 21 as libc::c_int {
        current_block = 890779300656278608;
    } else if msgtype == 24 as libc::c_int {
        current_block = 890779300656278608;
    } else {
        current_block = 11194104282611034094;
    }
    match current_block {
        890779300656278608 => {
            lastbyte = msgbits / 8 as libc::c_int - 1 as libc::c_int;
            memcpy(
                aux.as_mut_ptr() as *mut libc::c_void,
                msg as *const libc::c_void,
                (msgbits / 8 as libc::c_int) as size_t,
            );
            crc = modesChecksum(aux.as_mut_ptr(), msgbits);
            aux[lastbyte
                as usize] = (aux[lastbyte as usize] as libc::c_uint
                ^ crc & 255 as libc::c_uint) as libc::c_uchar;
            aux[(lastbyte - 1 as libc::c_int)
                as usize] = (aux[(lastbyte - 1 as libc::c_int) as usize] as libc::c_uint
                ^ crc >> 8 as libc::c_int & 255 as libc::c_uint) as libc::c_uchar;
            aux[(lastbyte - 2 as libc::c_int)
                as usize] = (aux[(lastbyte - 2 as libc::c_int) as usize] as libc::c_uint
                ^ crc >> 16 as libc::c_int & 255 as libc::c_uint) as libc::c_uchar;
            addr = (aux[lastbyte as usize] as libc::c_int
                | (aux[(lastbyte - 1 as libc::c_int) as usize] as libc::c_int)
                    << 8 as libc::c_int
                | (aux[(lastbyte - 2 as libc::c_int) as usize] as libc::c_int)
                    << 16 as libc::c_int) as uint32_t;
            tmp = ICAOAddressWasRecentlySeen(addr);
            if tmp != 0 {
                (*mm).aa1 = aux[(lastbyte - 2 as libc::c_int) as usize] as libc::c_int;
                (*mm).aa2 = aux[(lastbyte - 1 as libc::c_int) as usize] as libc::c_int;
                (*mm).aa3 = aux[lastbyte as usize] as libc::c_int;
                return 1 as libc::c_int;
            }
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn decodeAC13Field(
    mut msg: *mut libc::c_uchar,
    mut unit: *mut libc::c_int,
) -> libc::c_int {
    let mut m_bit: libc::c_int = 0;
    let mut q_bit: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    m_bit = *msg.offset(3 as libc::c_int as isize) as libc::c_int
        & (1 as libc::c_int) << 6 as libc::c_int;
    q_bit = *msg.offset(3 as libc::c_int as isize) as libc::c_int
        & (1 as libc::c_int) << 4 as libc::c_int;
    if m_bit == 0 {
        *unit = 0 as libc::c_int;
        if q_bit != 0 {
            n = (*msg.offset(2 as libc::c_int as isize) as libc::c_int
                & 31 as libc::c_int) << 6 as libc::c_int
                | (*msg.offset(3 as libc::c_int as isize) as libc::c_int
                    & 128 as libc::c_int) >> 2 as libc::c_int
                | (*msg.offset(3 as libc::c_int as isize) as libc::c_int
                    & 32 as libc::c_int) >> 1 as libc::c_int
                | *msg.offset(3 as libc::c_int as isize) as libc::c_int
                    & 15 as libc::c_int;
            return n * 25 as libc::c_int - 1000 as libc::c_int;
        }
    } else {
        *unit = 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn decodeAC12Field(
    mut msg: *mut libc::c_uchar,
    mut unit: *mut libc::c_int,
) -> libc::c_int {
    let mut q_bit: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    q_bit = *msg.offset(5 as libc::c_int as isize) as libc::c_int & 1 as libc::c_int;
    if q_bit != 0 {
        *unit = 0 as libc::c_int;
        n = (*msg.offset(5 as libc::c_int as isize) as libc::c_int >> 1 as libc::c_int)
            << 4 as libc::c_int
            | (*msg.offset(6 as libc::c_int as isize) as libc::c_int
                & 240 as libc::c_int) >> 4 as libc::c_int;
        return n * 25 as libc::c_int - 1000 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
pub static mut ca_str: [*mut libc::c_char; 8] = [
    b"Level 1 (Survillance Only)\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Level 2 (DF0,4,5,11)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Level 3 (DF0,4,5,11,20,21)\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Level 4 (DF0,4,5,11,20,21,24)\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Level 2+3+4 (DF0,4,5,11,20,21,24,code7 - is on ground)\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Level 2+3+4 (DF0,4,5,11,20,21,24,code7 - is on airborne)\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Level 2+3+4 (DF0,4,5,11,20,21,24,code7)\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Level 7 ???\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
pub static mut fs_str: [*mut libc::c_char; 8] = [
    b"Normal, Airborne\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Normal, On the ground\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ALERT,  Airborne\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ALERT,  On the ground\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ALERT & Special Position Identification. Airborne or Ground\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Special Position Identification. Airborne or Ground\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Value 6 is not assigned\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Value 7 is not assigned\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
pub static mut me_str: [*mut libc::c_char; 0] = [];
pub unsafe extern "C" fn getMEDescription(
    mut metype: libc::c_int,
    mut mesub: libc::c_int,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut mename: *mut libc::c_char = 0 as *mut libc::c_char;
    mename = b"Unknown\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    if metype >= 1 as libc::c_int {
        if metype <= 4 as libc::c_int {
            mename = b"Aircraft Identification and Category\0" as *const u8
                as *const libc::c_char as *mut libc::c_char;
            current_block = 9437013279121998969;
        } else {
            current_block = 3274268373848840126;
        }
    } else {
        current_block = 3274268373848840126;
    }
    match current_block {
        3274268373848840126 => {
            if metype >= 5 as libc::c_int {
                if metype <= 8 as libc::c_int {
                    mename = b"Surface Position\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    current_block = 9437013279121998969;
                } else {
                    current_block = 1667684503068533797;
                }
            } else {
                current_block = 1667684503068533797;
            }
            match current_block {
                9437013279121998969 => {}
                _ => {
                    if metype >= 9 as libc::c_int {
                        if metype <= 18 as libc::c_int {
                            mename = b"Airborne Position (Baro Altitude)\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char;
                            current_block = 9437013279121998969;
                        } else {
                            current_block = 13739842543123985354;
                        }
                    } else {
                        current_block = 13739842543123985354;
                    }
                    match current_block {
                        9437013279121998969 => {}
                        _ => {
                            if metype == 19 as libc::c_int {
                                if mesub >= 1 as libc::c_int {
                                    if mesub <= 4 as libc::c_int {
                                        mename = b"Airborne Velocity\0" as *const u8
                                            as *const libc::c_char as *mut libc::c_char;
                                        current_block = 9437013279121998969;
                                    } else {
                                        current_block = 4623188485972666653;
                                    }
                                } else {
                                    current_block = 4623188485972666653;
                                }
                            } else {
                                current_block = 4623188485972666653;
                            }
                            match current_block {
                                9437013279121998969 => {}
                                _ => {
                                    let mut current_block_35: u64;
                                    if metype >= 20 as libc::c_int {
                                        if metype <= 22 as libc::c_int {
                                            mename = b"Airborne Position (GNSS Height)\0" as *const u8
                                                as *const libc::c_char as *mut libc::c_char;
                                            current_block_35 = 9512719473022792396;
                                        } else {
                                            current_block_35 = 6555870807602492068;
                                        }
                                    } else {
                                        current_block_35 = 6555870807602492068;
                                    }
                                    match current_block_35 {
                                        6555870807602492068 => {
                                            let mut current_block_34: u64;
                                            if metype == 23 as libc::c_int {
                                                if mesub == 0 as libc::c_int {
                                                    mename = b"Test Message\0" as *const u8
                                                        as *const libc::c_char as *mut libc::c_char;
                                                    current_block_34 = 168769493162332264;
                                                } else {
                                                    current_block_34 = 15117816922977958695;
                                                }
                                            } else {
                                                current_block_34 = 15117816922977958695;
                                            }
                                            match current_block_34 {
                                                15117816922977958695 => {
                                                    let mut current_block_33: u64;
                                                    if metype == 24 as libc::c_int {
                                                        if mesub == 1 as libc::c_int {
                                                            mename = b"Surface System Status\0" as *const u8
                                                                as *const libc::c_char as *mut libc::c_char;
                                                            current_block_33 = 5159818223158340697;
                                                        } else {
                                                            current_block_33 = 15227540185581375090;
                                                        }
                                                    } else {
                                                        current_block_33 = 15227540185581375090;
                                                    }
                                                    match current_block_33 {
                                                        15227540185581375090 => {
                                                            let mut current_block_32: u64;
                                                            if metype == 28 as libc::c_int {
                                                                if mesub == 1 as libc::c_int {
                                                                    mename = b"Extended Squitter Aircraft Status (Emergency)\0"
                                                                        as *const u8 as *const libc::c_char as *mut libc::c_char;
                                                                    current_block_32 = 13325891313334703151;
                                                                } else {
                                                                    current_block_32 = 11875477312844758197;
                                                                }
                                                            } else {
                                                                current_block_32 = 11875477312844758197;
                                                            }
                                                            match current_block_32 {
                                                                11875477312844758197 => {
                                                                    let mut current_block_31: u64;
                                                                    if metype == 28 as libc::c_int {
                                                                        if mesub == 2 as libc::c_int {
                                                                            mename = b"Extended Squitter Aircraft Status (1090ES TCAS RA)\0"
                                                                                as *const u8 as *const libc::c_char as *mut libc::c_char;
                                                                            current_block_31 = 7343950298149844727;
                                                                        } else {
                                                                            current_block_31 = 2981975133808773618;
                                                                        }
                                                                    } else {
                                                                        current_block_31 = 2981975133808773618;
                                                                    }
                                                                    match current_block_31 {
                                                                        2981975133808773618 => {
                                                                            let mut current_block_30: u64;
                                                                            if metype == 29 as libc::c_int {
                                                                                if mesub == 0 as libc::c_int {
                                                                                    mename = b"Target State and Status Message\0" as *const u8
                                                                                        as *const libc::c_char as *mut libc::c_char;
                                                                                    current_block_30 = 9353995356876505083;
                                                                                } else if mesub == 1 as libc::c_int {
                                                                                    mename = b"Target State and Status Message\0" as *const u8
                                                                                        as *const libc::c_char as *mut libc::c_char;
                                                                                    current_block_30 = 9353995356876505083;
                                                                                } else {
                                                                                    current_block_30 = 11768578289904235609;
                                                                                }
                                                                            } else {
                                                                                current_block_30 = 11768578289904235609;
                                                                            }
                                                                            match current_block_30 {
                                                                                11768578289904235609 => {
                                                                                    if metype == 31 as libc::c_int {
                                                                                        if mesub == 0 as libc::c_int {
                                                                                            mename = b"Aircraft Operational Status Message\0"
                                                                                                as *const u8 as *const libc::c_char as *mut libc::c_char;
                                                                                        } else if mesub == 1 as libc::c_int {
                                                                                            mename = b"Aircraft Operational Status Message\0"
                                                                                                as *const u8 as *const libc::c_char as *mut libc::c_char;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                _ => {}
                                                                            }
                                                                        }
                                                                        _ => {}
                                                                    }
                                                                }
                                                                _ => {}
                                                            }
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                                _ => {}
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    return mename;
}
pub unsafe extern "C" fn decodeModesMessage(
    mut mm: *mut modesMessage,
    mut msg: *mut libc::c_uchar,
) {
    let mut crc2: uint32_t = 0;
    let mut ais_charset: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut addr: uint32_t = 0;
    let mut tmp___2: libc::c_double = 0.;
    let mut ewv: libc::c_int = 0;
    let mut nsv: libc::c_int = 0;
    let mut heading: libc::c_double = 0.;
    ais_charset = b"?ABCDEFGHIJKLMNOPQRSTUVWXYZ????? ???????????????0123456789??????\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    memcpy(
        ((*mm).msg).as_mut_ptr() as *mut libc::c_void,
        msg as *const libc::c_void,
        14 as libc::c_int as size_t,
    );
    msg = ((*mm).msg).as_mut_ptr();
    (*mm)
        .msgtype = *msg.offset(0 as libc::c_int as isize) as libc::c_int
        >> 3 as libc::c_int;
    (*mm).msgbits = modesMessageLenByType((*mm).msgtype);
    (*mm)
        .crc = (*msg
        .offset(((*mm).msgbits / 8 as libc::c_int - 3 as libc::c_int) as isize)
        as uint32_t) << 16 as libc::c_int
        | (*msg.offset(((*mm).msgbits / 8 as libc::c_int - 2 as libc::c_int) as isize)
            as uint32_t) << 8 as libc::c_int
        | *msg.offset(((*mm).msgbits / 8 as libc::c_int - 1 as libc::c_int) as isize)
            as uint32_t;
    crc2 = modesChecksum(msg, (*mm).msgbits);
    (*mm).errorbit = -(1 as libc::c_int);
    (*mm).crcok = ((*mm).crc == crc2) as libc::c_int;
    if (*mm).crcok == 0 {
        if Modes.fix_errors != 0 {
            let mut current_block_24: u64;
            if (*mm).msgtype == 11 as libc::c_int {
                current_block_24 = 17357666782274949207;
            } else if (*mm).msgtype == 17 as libc::c_int {
                current_block_24 = 17357666782274949207;
            } else {
                current_block_24 = 15925075030174552612;
            }
            match current_block_24 {
                17357666782274949207 => {
                    tmp___0 = fixSingleBitErrors(msg, (*mm).msgbits);
                    (*mm).errorbit = tmp___0;
                    if tmp___0 != -(1 as libc::c_int) {
                        (*mm).crc = modesChecksum(msg, (*mm).msgbits);
                        (*mm).crcok = 1 as libc::c_int;
                    } else if Modes.aggressive != 0 {
                        if (*mm).msgtype == 17 as libc::c_int {
                            tmp = fixTwoBitsErrors(msg, (*mm).msgbits);
                            (*mm).errorbit = tmp;
                            if tmp != -(1 as libc::c_int) {
                                (*mm).crc = modesChecksum(msg, (*mm).msgbits);
                                (*mm).crcok = 1 as libc::c_int;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    (*mm).ca = *msg.offset(0 as libc::c_int as isize) as libc::c_int & 7 as libc::c_int;
    (*mm).aa1 = *msg.offset(1 as libc::c_int as isize) as libc::c_int;
    (*mm).aa2 = *msg.offset(2 as libc::c_int as isize) as libc::c_int;
    (*mm).aa3 = *msg.offset(3 as libc::c_int as isize) as libc::c_int;
    (*mm)
        .metype = *msg.offset(4 as libc::c_int as isize) as libc::c_int
        >> 3 as libc::c_int;
    (*mm)
        .mesub = *msg.offset(4 as libc::c_int as isize) as libc::c_int
        & 7 as libc::c_int;
    (*mm).fs = *msg.offset(0 as libc::c_int as isize) as libc::c_int & 7 as libc::c_int;
    (*mm)
        .dr = *msg.offset(1 as libc::c_int as isize) as libc::c_int >> 3 as libc::c_int
        & 31 as libc::c_int;
    (*mm)
        .um = (*msg.offset(1 as libc::c_int as isize) as libc::c_int & 7 as libc::c_int)
        << 3 as libc::c_int
        | *msg.offset(2 as libc::c_int as isize) as libc::c_int >> 5 as libc::c_int;
    a = (*msg.offset(3 as libc::c_int as isize) as libc::c_int & 128 as libc::c_int)
        >> 5 as libc::c_int
        | *msg.offset(2 as libc::c_int as isize) as libc::c_int & 2 as libc::c_int
        | (*msg.offset(2 as libc::c_int as isize) as libc::c_int & 8 as libc::c_int)
            >> 3 as libc::c_int;
    b = (*msg.offset(3 as libc::c_int as isize) as libc::c_int & 2 as libc::c_int)
        << 1 as libc::c_int
        | (*msg.offset(3 as libc::c_int as isize) as libc::c_int & 8 as libc::c_int)
            >> 2 as libc::c_int
        | (*msg.offset(3 as libc::c_int as isize) as libc::c_int & 32 as libc::c_int)
            >> 5 as libc::c_int;
    c = (*msg.offset(2 as libc::c_int as isize) as libc::c_int & 1 as libc::c_int)
        << 2 as libc::c_int
        | (*msg.offset(2 as libc::c_int as isize) as libc::c_int & 4 as libc::c_int)
            >> 1 as libc::c_int
        | (*msg.offset(2 as libc::c_int as isize) as libc::c_int & 16 as libc::c_int)
            >> 4 as libc::c_int;
    d = (*msg.offset(3 as libc::c_int as isize) as libc::c_int & 1 as libc::c_int)
        << 2 as libc::c_int
        | (*msg.offset(3 as libc::c_int as isize) as libc::c_int & 4 as libc::c_int)
            >> 1 as libc::c_int
        | (*msg.offset(3 as libc::c_int as isize) as libc::c_int & 16 as libc::c_int)
            >> 4 as libc::c_int;
    (*mm)
        .identity = a * 1000 as libc::c_int + b * 100 as libc::c_int
        + c * 10 as libc::c_int + d;
    let mut current_block_56: u64;
    if (*mm).msgtype != 11 as libc::c_int {
        if (*mm).msgtype != 17 as libc::c_int {
            tmp___1 = bruteForceAP(msg, mm);
            if tmp___1 != 0 {
                (*mm).crcok = 1 as libc::c_int;
            } else {
                (*mm).crcok = 0 as libc::c_int;
            }
            current_block_56 = 2516253395664191498;
        } else {
            current_block_56 = 8739316383967454452;
        }
    } else {
        current_block_56 = 8739316383967454452;
    }
    match current_block_56 {
        8739316383967454452 => {
            if (*mm).crcok != 0 {
                if (*mm).errorbit == -(1 as libc::c_int) {
                    addr = ((*mm).aa1 << 16 as libc::c_int
                        | (*mm).aa2 << 8 as libc::c_int | (*mm).aa3) as uint32_t;
                    addRecentlySeenICAOAddr(addr);
                }
            }
        }
        _ => {}
    }
    if (*mm).msgtype == 0 as libc::c_int {
        (*mm).altitude = decodeAC13Field(msg, &mut (*mm).unit);
    } else if (*mm).msgtype == 4 as libc::c_int {
        (*mm).altitude = decodeAC13Field(msg, &mut (*mm).unit);
    } else if (*mm).msgtype == 16 as libc::c_int {
        (*mm).altitude = decodeAC13Field(msg, &mut (*mm).unit);
    } else if (*mm).msgtype == 20 as libc::c_int {
        (*mm).altitude = decodeAC13Field(msg, &mut (*mm).unit);
    }
    if (*mm).msgtype == 17 as libc::c_int {
        let mut current_block_128: u64;
        if (*mm).metype >= 1 as libc::c_int {
            if (*mm).metype <= 4 as libc::c_int {
                (*mm).aircraft_type = (*mm).metype - 1 as libc::c_int;
                (*mm)
                    .flight[0 as libc::c_int
                    as usize] = *ais_charset
                    .offset(
                        (*msg.offset(5 as libc::c_int as isize) as libc::c_int
                            >> 2 as libc::c_int) as isize,
                    );
                (*mm)
                    .flight[1 as libc::c_int
                    as usize] = *ais_charset
                    .offset(
                        ((*msg.offset(5 as libc::c_int as isize) as libc::c_int
                            & 3 as libc::c_int) << 4 as libc::c_int
                            | *msg.offset(6 as libc::c_int as isize) as libc::c_int
                                >> 4 as libc::c_int) as isize,
                    );
                (*mm)
                    .flight[2 as libc::c_int
                    as usize] = *ais_charset
                    .offset(
                        ((*msg.offset(6 as libc::c_int as isize) as libc::c_int
                            & 15 as libc::c_int) << 2 as libc::c_int
                            | *msg.offset(7 as libc::c_int as isize) as libc::c_int
                                >> 6 as libc::c_int) as isize,
                    );
                (*mm)
                    .flight[3 as libc::c_int
                    as usize] = *ais_charset
                    .offset(
                        (*msg.offset(7 as libc::c_int as isize) as libc::c_int
                            & 63 as libc::c_int) as isize,
                    );
                (*mm)
                    .flight[4 as libc::c_int
                    as usize] = *ais_charset
                    .offset(
                        (*msg.offset(8 as libc::c_int as isize) as libc::c_int
                            >> 2 as libc::c_int) as isize,
                    );
                (*mm)
                    .flight[5 as libc::c_int
                    as usize] = *ais_charset
                    .offset(
                        ((*msg.offset(8 as libc::c_int as isize) as libc::c_int
                            & 3 as libc::c_int) << 4 as libc::c_int
                            | *msg.offset(9 as libc::c_int as isize) as libc::c_int
                                >> 4 as libc::c_int) as isize,
                    );
                (*mm)
                    .flight[6 as libc::c_int
                    as usize] = *ais_charset
                    .offset(
                        ((*msg.offset(9 as libc::c_int as isize) as libc::c_int
                            & 15 as libc::c_int) << 2 as libc::c_int
                            | *msg.offset(10 as libc::c_int as isize) as libc::c_int
                                >> 6 as libc::c_int) as isize,
                    );
                (*mm)
                    .flight[7 as libc::c_int
                    as usize] = *ais_charset
                    .offset(
                        (*msg.offset(10 as libc::c_int as isize) as libc::c_int
                            & 63 as libc::c_int) as isize,
                    );
                (*mm).flight[8 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
                current_block_128 = 18104233774012731761;
            } else {
                current_block_128 = 15444674478551160353;
            }
        } else {
            current_block_128 = 15444674478551160353;
        }
        match current_block_128 {
            15444674478551160353 => {
                let mut current_block_127: u64;
                if (*mm).metype >= 9 as libc::c_int {
                    if (*mm).metype <= 18 as libc::c_int {
                        (*mm)
                            .fflag = *msg.offset(6 as libc::c_int as isize)
                            as libc::c_int & (1 as libc::c_int) << 2 as libc::c_int;
                        (*mm)
                            .tflag = *msg.offset(6 as libc::c_int as isize)
                            as libc::c_int & (1 as libc::c_int) << 3 as libc::c_int;
                        (*mm).altitude = decodeAC12Field(msg, &mut (*mm).unit);
                        (*mm)
                            .raw_latitude = (*msg.offset(6 as libc::c_int as isize)
                            as libc::c_int & 3 as libc::c_int) << 15 as libc::c_int
                            | (*msg.offset(7 as libc::c_int as isize) as libc::c_int)
                                << 7 as libc::c_int
                            | *msg.offset(8 as libc::c_int as isize) as libc::c_int
                                >> 1 as libc::c_int;
                        (*mm)
                            .raw_longitude = (*msg.offset(8 as libc::c_int as isize)
                            as libc::c_int & 1 as libc::c_int) << 16 as libc::c_int
                            | (*msg.offset(9 as libc::c_int as isize) as libc::c_int)
                                << 8 as libc::c_int
                            | *msg.offset(10 as libc::c_int as isize) as libc::c_int;
                        current_block_127 = 6478348674394853609;
                    } else {
                        current_block_127 = 4824977408923351192;
                    }
                } else {
                    current_block_127 = 4824977408923351192;
                }
                match current_block_127 {
                    4824977408923351192 => {
                        if (*mm).metype == 19 as libc::c_int {
                            if (*mm).mesub >= 1 as libc::c_int {
                                if (*mm).mesub <= 4 as libc::c_int {
                                    let mut current_block_120: u64;
                                    if (*mm).mesub == 1 as libc::c_int {
                                        current_block_120 = 9021772824828667159;
                                    } else if (*mm).mesub == 2 as libc::c_int {
                                        current_block_120 = 9021772824828667159;
                                    } else {
                                        if (*mm).mesub == 3 as libc::c_int {
                                            (*mm)
                                                .heading_is_valid = *msg.offset(5 as libc::c_int as isize)
                                                as libc::c_int & (1 as libc::c_int) << 2 as libc::c_int;
                                            (*mm)
                                                .heading = (360.0f64 / 128 as libc::c_int as libc::c_double
                                                * ((*msg.offset(5 as libc::c_int as isize) as libc::c_int
                                                    & 3 as libc::c_int) << 5 as libc::c_int
                                                    | *msg.offset(6 as libc::c_int as isize) as libc::c_int
                                                        >> 3 as libc::c_int) as libc::c_double) as libc::c_int;
                                        } else if (*mm).mesub == 4 as libc::c_int {
                                            (*mm)
                                                .heading_is_valid = *msg.offset(5 as libc::c_int as isize)
                                                as libc::c_int & (1 as libc::c_int) << 2 as libc::c_int;
                                            (*mm)
                                                .heading = (360.0f64 / 128 as libc::c_int as libc::c_double
                                                * ((*msg.offset(5 as libc::c_int as isize) as libc::c_int
                                                    & 3 as libc::c_int) << 5 as libc::c_int
                                                    | *msg.offset(6 as libc::c_int as isize) as libc::c_int
                                                        >> 3 as libc::c_int) as libc::c_double) as libc::c_int;
                                        }
                                        current_block_120 = 7639320476250304355;
                                    }
                                    match current_block_120 {
                                        9021772824828667159 => {
                                            (*mm)
                                                .ew_dir = (*msg.offset(5 as libc::c_int as isize)
                                                as libc::c_int & 4 as libc::c_int) >> 2 as libc::c_int;
                                            (*mm)
                                                .ew_velocity = (*msg.offset(5 as libc::c_int as isize)
                                                as libc::c_int & 3 as libc::c_int) << 8 as libc::c_int
                                                | *msg.offset(6 as libc::c_int as isize) as libc::c_int;
                                            (*mm)
                                                .ns_dir = (*msg.offset(7 as libc::c_int as isize)
                                                as libc::c_int & 128 as libc::c_int) >> 7 as libc::c_int;
                                            (*mm)
                                                .ns_velocity = (*msg.offset(7 as libc::c_int as isize)
                                                as libc::c_int & 127 as libc::c_int) << 3 as libc::c_int
                                                | (*msg.offset(8 as libc::c_int as isize) as libc::c_int
                                                    & 224 as libc::c_int) >> 5 as libc::c_int;
                                            (*mm)
                                                .vert_rate_source = (*msg.offset(8 as libc::c_int as isize)
                                                as libc::c_int & 16 as libc::c_int) >> 4 as libc::c_int;
                                            (*mm)
                                                .vert_rate_sign = (*msg.offset(8 as libc::c_int as isize)
                                                as libc::c_int & 8 as libc::c_int) >> 3 as libc::c_int;
                                            (*mm)
                                                .vert_rate = (*msg.offset(8 as libc::c_int as isize)
                                                as libc::c_int & 7 as libc::c_int) << 6 as libc::c_int
                                                | (*msg.offset(9 as libc::c_int as isize) as libc::c_int
                                                    & 252 as libc::c_int) >> 2 as libc::c_int;
                                            tmp___2 = sqrt(
                                                ((*mm).ns_velocity * (*mm).ns_velocity
                                                    + (*mm).ew_velocity * (*mm).ew_velocity) as libc::c_double,
                                            );
                                            (*mm).velocity = tmp___2 as libc::c_int;
                                            if (*mm).velocity != 0 {
                                                ewv = (*mm).ew_velocity;
                                                nsv = (*mm).ns_velocity;
                                                if (*mm).ew_dir != 0 {
                                                    ewv *= -(1 as libc::c_int);
                                                }
                                                if (*mm).ns_dir != 0 {
                                                    nsv *= -(1 as libc::c_int);
                                                }
                                                heading = atan2(
                                                    ewv as libc::c_double,
                                                    nsv as libc::c_double,
                                                );
                                                (*mm)
                                                    .heading = (heading * 360 as libc::c_int as libc::c_double
                                                    / (3.14159265358979323846f64
                                                        * 2 as libc::c_int as libc::c_double)) as libc::c_int;
                                                if (*mm).heading < 0 as libc::c_int {
                                                    (*mm).heading += 360 as libc::c_int;
                                                }
                                            } else {
                                                (*mm).heading = 0 as libc::c_int;
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    (*mm).phase_corrected = 0 as libc::c_int;
}
pub unsafe extern "C" fn displayModesMessage(mut mm: *mut modesMessage) {
    let mut j: libc::c_int = 0;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___2: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___3: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ac_type_str: [*mut libc::c_char; 4] = [0 as *mut libc::c_char; 4];
    let mut tmp___5: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___6: *const libc::c_char = 0 as *const libc::c_char;
    if Modes.onlyaddr != 0 {
        printf(
            b"%02x%02x%02x\n\0" as *const u8 as *const libc::c_char,
            (*mm).aa1,
            (*mm).aa2,
            (*mm).aa3,
        );
        return;
    }
    printf(b"*\0" as *const u8 as *const libc::c_char);
    j = 0 as libc::c_int;
    while j < (*mm).msgbits / 8 as libc::c_int {
        printf(
            b"%02x\0" as *const u8 as *const libc::c_char,
            (*mm).msg[j as usize] as libc::c_int,
        );
        j += 1;
    }
    printf(b";\n\0" as *const u8 as *const libc::c_char);
    if Modes.raw != 0 {
        fflush(stdout);
        return;
    }
    if (*mm).crcok != 0 {
        tmp = b"ok\0" as *const u8 as *const libc::c_char;
    } else {
        tmp = b"wrong\0" as *const u8 as *const libc::c_char;
    }
    printf(
        b"CRC: %06x (%s)\n\0" as *const u8 as *const libc::c_char,
        (*mm).crc as libc::c_int,
        tmp,
    );
    if (*mm).errorbit != -(1 as libc::c_int) {
        printf(
            b"Single bit error fixed, bit %d\n\0" as *const u8 as *const libc::c_char,
            (*mm).errorbit,
        );
    }
    if (*mm).msgtype == 0 as libc::c_int {
        printf(
            b"DF 0: Short Air-Air Surveillance.\n\0" as *const u8 as *const libc::c_char,
        );
        if (*mm).unit == 1 as libc::c_int {
            tmp___0 = b"meters\0" as *const u8 as *const libc::c_char;
        } else {
            tmp___0 = b"feet\0" as *const u8 as *const libc::c_char;
        }
        printf(
            b"  Altitude       : %d %s\n\0" as *const u8 as *const libc::c_char,
            (*mm).altitude,
            tmp___0,
        );
        printf(
            b"  ICAO Address   : %02x%02x%02x\n\0" as *const u8 as *const libc::c_char,
            (*mm).aa1,
            (*mm).aa2,
            (*mm).aa3,
        );
    } else {
        let mut current_block_130: u64;
        if (*mm).msgtype == 4 as libc::c_int {
            current_block_130 = 4806731268500937409;
        } else if (*mm).msgtype == 20 as libc::c_int {
            current_block_130 = 4806731268500937409;
        } else {
            let mut current_block_129: u64;
            if (*mm).msgtype == 5 as libc::c_int {
                current_block_129 = 13383156954553174665;
            } else if (*mm).msgtype == 21 as libc::c_int {
                current_block_129 = 13383156954553174665;
            } else {
                if (*mm).msgtype == 11 as libc::c_int {
                    printf(
                        b"DF 11: All Call Reply.\n\0" as *const u8 as *const libc::c_char,
                    );
                    printf(
                        b"  Capability  : %s\n\0" as *const u8 as *const libc::c_char,
                        ca_str[(*mm).ca as usize],
                    );
                    printf(
                        b"  ICAO Address: %02x%02x%02x\n\0" as *const u8
                            as *const libc::c_char,
                        (*mm).aa1,
                        (*mm).aa2,
                        (*mm).aa3,
                    );
                } else if (*mm).msgtype == 17 as libc::c_int {
                    printf(
                        b"DF 17: ADS-B message.\n\0" as *const u8 as *const libc::c_char,
                    );
                    printf(
                        b"  Capability     : %d (%s)\n\0" as *const u8
                            as *const libc::c_char,
                        (*mm).ca,
                        ca_str[(*mm).ca as usize],
                    );
                    printf(
                        b"  ICAO Address   : %02x%02x%02x\n\0" as *const u8
                            as *const libc::c_char,
                        (*mm).aa1,
                        (*mm).aa2,
                        (*mm).aa3,
                    );
                    printf(
                        b"  Extended Squitter  Type: %d\n\0" as *const u8
                            as *const libc::c_char,
                        (*mm).metype,
                    );
                    printf(
                        b"  Extended Squitter  Sub : %d\n\0" as *const u8
                            as *const libc::c_char,
                        (*mm).mesub,
                    );
                    tmp___4 = getMEDescription((*mm).metype, (*mm).mesub);
                    printf(
                        b"  Extended Squitter  Name: %s\n\0" as *const u8
                            as *const libc::c_char,
                        tmp___4,
                    );
                    let mut current_block_122: u64;
                    if (*mm).metype >= 1 as libc::c_int {
                        if (*mm).metype <= 4 as libc::c_int {
                            ac_type_str[0 as libc::c_int
                                as usize] = b"Aircraft Type D\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char;
                            ac_type_str[1 as libc::c_int
                                as usize] = b"Aircraft Type C\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char;
                            ac_type_str[2 as libc::c_int
                                as usize] = b"Aircraft Type B\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char;
                            ac_type_str[3 as libc::c_int
                                as usize] = b"Aircraft Type A\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char;
                            printf(
                                b"    Aircraft Type  : %s\n\0" as *const u8
                                    as *const libc::c_char,
                                ac_type_str[(*mm).aircraft_type as usize],
                            );
                            printf(
                                b"    Identification : %s\n\0" as *const u8
                                    as *const libc::c_char,
                                ((*mm).flight).as_mut_ptr(),
                            );
                            current_block_122 = 3575278370434307847;
                        } else {
                            current_block_122 = 1112815485866451153;
                        }
                    } else {
                        current_block_122 = 1112815485866451153;
                    }
                    match current_block_122 {
                        1112815485866451153 => {
                            let mut current_block_121: u64;
                            if (*mm).metype >= 9 as libc::c_int {
                                if (*mm).metype <= 18 as libc::c_int {
                                    if (*mm).fflag != 0 {
                                        tmp___5 = b"odd\0" as *const u8 as *const libc::c_char;
                                    } else {
                                        tmp___5 = b"even\0" as *const u8 as *const libc::c_char;
                                    }
                                    printf(
                                        b"    F flag   : %s\n\0" as *const u8
                                            as *const libc::c_char,
                                        tmp___5,
                                    );
                                    if (*mm).tflag != 0 {
                                        tmp___6 = b"UTC\0" as *const u8 as *const libc::c_char;
                                    } else {
                                        tmp___6 = b"non-UTC\0" as *const u8 as *const libc::c_char;
                                    }
                                    printf(
                                        b"    T flag   : %s\n\0" as *const u8
                                            as *const libc::c_char,
                                        tmp___6,
                                    );
                                    printf(
                                        b"    Altitude : %d feet\n\0" as *const u8
                                            as *const libc::c_char,
                                        (*mm).altitude,
                                    );
                                    printf(
                                        b"    Latitude : %d (not decoded)\n\0" as *const u8
                                            as *const libc::c_char,
                                        (*mm).raw_latitude,
                                    );
                                    printf(
                                        b"    Longitude: %d (not decoded)\n\0" as *const u8
                                            as *const libc::c_char,
                                        (*mm).raw_longitude,
                                    );
                                    current_block_121 = 4983594971376015098;
                                } else {
                                    current_block_121 = 4614912094741165467;
                                }
                            } else {
                                current_block_121 = 4614912094741165467;
                            }
                            match current_block_121 {
                                4614912094741165467 => {
                                    if (*mm).metype == 19 as libc::c_int {
                                        if (*mm).mesub >= 1 as libc::c_int {
                                            if (*mm).mesub <= 4 as libc::c_int {
                                                let mut current_block_108: u64;
                                                if (*mm).mesub == 1 as libc::c_int {
                                                    current_block_108 = 17934513498363362370;
                                                } else if (*mm).mesub == 2 as libc::c_int {
                                                    current_block_108 = 17934513498363362370;
                                                } else {
                                                    if (*mm).mesub == 3 as libc::c_int {
                                                        printf(
                                                            b"    Heading status: %d\0" as *const u8
                                                                as *const libc::c_char,
                                                            (*mm).heading_is_valid,
                                                        );
                                                        printf(
                                                            b"    Heading: %d\0" as *const u8 as *const libc::c_char,
                                                            (*mm).heading,
                                                        );
                                                    } else if (*mm).mesub == 4 as libc::c_int {
                                                        printf(
                                                            b"    Heading status: %d\0" as *const u8
                                                                as *const libc::c_char,
                                                            (*mm).heading_is_valid,
                                                        );
                                                        printf(
                                                            b"    Heading: %d\0" as *const u8 as *const libc::c_char,
                                                            (*mm).heading,
                                                        );
                                                    }
                                                    current_block_108 = 1934991416718554651;
                                                }
                                                match current_block_108 {
                                                    17934513498363362370 => {
                                                        printf(
                                                            b"    EW direction      : %d\n\0" as *const u8
                                                                as *const libc::c_char,
                                                            (*mm).ew_dir,
                                                        );
                                                        printf(
                                                            b"    EW velocity       : %d\n\0" as *const u8
                                                                as *const libc::c_char,
                                                            (*mm).ew_velocity,
                                                        );
                                                        printf(
                                                            b"    NS direction      : %d\n\0" as *const u8
                                                                as *const libc::c_char,
                                                            (*mm).ns_dir,
                                                        );
                                                        printf(
                                                            b"    NS velocity       : %d\n\0" as *const u8
                                                                as *const libc::c_char,
                                                            (*mm).ns_velocity,
                                                        );
                                                        printf(
                                                            b"    Vertical rate src : %d\n\0" as *const u8
                                                                as *const libc::c_char,
                                                            (*mm).vert_rate_source,
                                                        );
                                                        printf(
                                                            b"    Vertical rate sign: %d\n\0" as *const u8
                                                                as *const libc::c_char,
                                                            (*mm).vert_rate_sign,
                                                        );
                                                        printf(
                                                            b"    Vertical rate     : %d\n\0" as *const u8
                                                                as *const libc::c_char,
                                                            (*mm).vert_rate,
                                                        );
                                                    }
                                                    _ => {}
                                                }
                                            } else {
                                                printf(
                                                    b"    Unrecognized ME type: %d subtype: %d\n\0" as *const u8
                                                        as *const libc::c_char,
                                                    (*mm).metype,
                                                    (*mm).mesub,
                                                );
                                            }
                                        } else {
                                            printf(
                                                b"    Unrecognized ME type: %d subtype: %d\n\0" as *const u8
                                                    as *const libc::c_char,
                                                (*mm).metype,
                                                (*mm).mesub,
                                            );
                                        }
                                    } else {
                                        printf(
                                            b"    Unrecognized ME type: %d subtype: %d\n\0" as *const u8
                                                as *const libc::c_char,
                                            (*mm).metype,
                                            (*mm).mesub,
                                        );
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                } else if Modes.check_crc != 0 {
                    printf(
                        b"DF %d with good CRC received (decoding still not implemented).\n\0"
                            as *const u8 as *const libc::c_char,
                        (*mm).msgtype,
                    );
                }
                current_block_129 = 14953815020842398287;
            }
            match current_block_129 {
                13383156954553174665 => {
                    if (*mm).msgtype == 5 as libc::c_int {
                        tmp___3 = b"Surveillance\0" as *const u8 as *const libc::c_char;
                    } else {
                        tmp___3 = b"Comm-B\0" as *const u8 as *const libc::c_char;
                    }
                    printf(
                        b"DF %d: %s, Identity Reply.\n\0" as *const u8
                            as *const libc::c_char,
                        (*mm).msgtype,
                        tmp___3,
                    );
                    printf(
                        b"  Flight Status  : %s\n\0" as *const u8 as *const libc::c_char,
                        fs_str[(*mm).fs as usize],
                    );
                    printf(
                        b"  DR             : %d\n\0" as *const u8 as *const libc::c_char,
                        (*mm).dr,
                    );
                    printf(
                        b"  UM             : %d\n\0" as *const u8 as *const libc::c_char,
                        (*mm).um,
                    );
                    printf(
                        b"  Squawk         : %d\n\0" as *const u8 as *const libc::c_char,
                        (*mm).identity,
                    );
                    printf(
                        b"  ICAO Address   : %02x%02x%02x\n\0" as *const u8
                            as *const libc::c_char,
                        (*mm).aa1,
                        (*mm).aa2,
                        (*mm).aa3,
                    );
                }
                _ => {}
            }
            current_block_130 = 8602574157404971894;
        }
        match current_block_130 {
            4806731268500937409 => {
                if (*mm).msgtype == 4 as libc::c_int {
                    tmp___1 = b"Surveillance\0" as *const u8 as *const libc::c_char;
                } else {
                    tmp___1 = b"Comm-B\0" as *const u8 as *const libc::c_char;
                }
                printf(
                    b"DF %d: %s, Altitude Reply.\n\0" as *const u8
                        as *const libc::c_char,
                    (*mm).msgtype,
                    tmp___1,
                );
                printf(
                    b"  Flight Status  : %s\n\0" as *const u8 as *const libc::c_char,
                    fs_str[(*mm).fs as usize],
                );
                printf(
                    b"  DR             : %d\n\0" as *const u8 as *const libc::c_char,
                    (*mm).dr,
                );
                printf(
                    b"  UM             : %d\n\0" as *const u8 as *const libc::c_char,
                    (*mm).um,
                );
                if (*mm).unit == 1 as libc::c_int {
                    tmp___2 = b"meters\0" as *const u8 as *const libc::c_char;
                } else {
                    tmp___2 = b"feet\0" as *const u8 as *const libc::c_char;
                }
                printf(
                    b"  Altitude       : %d %s\n\0" as *const u8 as *const libc::c_char,
                    (*mm).altitude,
                    tmp___2,
                );
                printf(
                    b"  ICAO Address   : %02x%02x%02x\n\0" as *const u8
                        as *const libc::c_char,
                    (*mm).aa1,
                    (*mm).aa2,
                    (*mm).aa3,
                );
            }
            _ => {}
        }
    };
}
pub unsafe extern "C" fn computeMagnitudeVector() {
    let mut m: *mut uint16_t = 0 as *mut uint16_t;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut j: uint32_t = 0;
    let mut i: libc::c_int = 0;
    let mut q: libc::c_int = 0;
    m = Modes.magnitude;
    p = Modes.data;
    j = 0 as libc::c_int as uint32_t;
    while j < Modes.data_len {
        i = *p.offset(j as isize) as libc::c_int - 127 as libc::c_int;
        q = *p.offset(j.wrapping_add(1 as libc::c_uint) as isize) as libc::c_int
            - 127 as libc::c_int;
        if i < 0 as libc::c_int {
            i = -i;
        }
        if q < 0 as libc::c_int {
            q = -q;
        }
        *m
            .offset(
                j.wrapping_div(2 as libc::c_uint) as isize,
            ) = *(Modes.maglut).offset((i * 129 as libc::c_int + q) as isize);
        j = (j as libc::c_uint).wrapping_add(2 as libc::c_uint) as uint32_t as uint32_t;
    }
}
pub unsafe extern "C" fn detectOutOfPhase(mut m: *mut uint16_t) -> libc::c_int {
    if *m.offset(3 as libc::c_int as isize) as libc::c_int
        > *m.offset(2 as libc::c_int as isize) as libc::c_int / 3 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if *m.offset(10 as libc::c_int as isize) as libc::c_int
        > *m.offset(9 as libc::c_int as isize) as libc::c_int / 3 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if *m.offset(6 as libc::c_int as isize) as libc::c_int
        > *m.offset(7 as libc::c_int as isize) as libc::c_int / 3 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if *m.offset(-(1 as libc::c_int) as isize) as libc::c_int
        > *m.offset(1 as libc::c_int as isize) as libc::c_int / 3 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn applyPhaseCorrection(mut m: *mut uint16_t) {
    let mut j: libc::c_int = 0;
    m = m.offset(16 as libc::c_int as isize);
    j = 0 as libc::c_int;
    while j < 222 as libc::c_int {
        if *m.offset(j as isize) as libc::c_int
            > *m.offset((j + 1 as libc::c_int) as isize) as libc::c_int
        {
            *m
                .offset(
                    (j + 2 as libc::c_int) as isize,
                ) = (*m.offset((j + 2 as libc::c_int) as isize) as libc::c_int
                * 5 as libc::c_int / 4 as libc::c_int) as uint16_t;
        } else {
            *m
                .offset(
                    (j + 2 as libc::c_int) as isize,
                ) = (*m.offset((j + 2 as libc::c_int) as isize) as libc::c_int
                * 4 as libc::c_int / 5 as libc::c_int) as uint16_t;
        }
        j += 2 as libc::c_int;
    }
}
pub unsafe extern "C" fn detectModeS(mut m: *mut uint16_t, mut mlen: uint32_t) {
    let mut current_block: u64;
    let mut bits: [libc::c_uchar; 112] = [0; 112];
    let mut msg: [libc::c_uchar; 56] = [0; 56];
    let mut aux: [uint16_t; 224] = [0; 224];
    let mut j: uint32_t = 0;
    let mut use_correction: libc::c_int = 0;
    let mut low: libc::c_int = 0;
    let mut high: libc::c_int = 0;
    let mut delta: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut errors: libc::c_int = 0;
    let mut good_message: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut msgtype: libc::c_int = 0;
    let mut msglen: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut mm: modesMessage = modesMessage {
        msg: [0; 14],
        msgbits: 0,
        msgtype: 0,
        crcok: 0,
        crc: 0,
        errorbit: 0,
        aa1: 0,
        aa2: 0,
        aa3: 0,
        phase_corrected: 0,
        ca: 0,
        metype: 0,
        mesub: 0,
        heading_is_valid: 0,
        heading: 0,
        aircraft_type: 0,
        fflag: 0,
        tflag: 0,
        raw_latitude: 0,
        raw_longitude: 0,
        flight: [0; 9],
        ew_dir: 0,
        ew_velocity: 0,
        ns_dir: 0,
        ns_velocity: 0,
        vert_rate_source: 0,
        vert_rate_sign: 0,
        vert_rate: 0,
        velocity: 0,
        fs: 0,
        dr: 0,
        um: 0,
        identity: 0,
        altitude: 0,
        unit: 0,
    };
    use_correction = 0 as libc::c_int;
    j = 0 as libc::c_int as uint32_t;
    while j < mlen.wrapping_sub(240 as libc::c_uint) {
        good_message = 0 as libc::c_int;
        if use_correction != 0 {
            current_block = 7294691061932571495;
        } else {
            if *m.offset(j as isize) as libc::c_int
                > *m.offset(j.wrapping_add(1 as libc::c_uint) as isize) as libc::c_int
            {
                if (*m.offset(j.wrapping_add(1 as libc::c_uint) as isize) as libc::c_int)
                    < *m.offset(j.wrapping_add(2 as libc::c_uint) as isize)
                        as libc::c_int
                {
                    if *m.offset(j.wrapping_add(2 as libc::c_uint) as isize)
                        as libc::c_int
                        > *m.offset(j.wrapping_add(3 as libc::c_uint) as isize)
                            as libc::c_int
                    {
                        if (*m.offset(j.wrapping_add(3 as libc::c_uint) as isize)
                            as libc::c_int) < *m.offset(j as isize) as libc::c_int
                        {
                            if (*m.offset(j.wrapping_add(4 as libc::c_uint) as isize)
                                as libc::c_int) < *m.offset(j as isize) as libc::c_int
                            {
                                if (*m.offset(j.wrapping_add(5 as libc::c_uint) as isize)
                                    as libc::c_int) < *m.offset(j as isize) as libc::c_int
                                {
                                    if (*m.offset(j.wrapping_add(6 as libc::c_uint) as isize)
                                        as libc::c_int) < *m.offset(j as isize) as libc::c_int
                                    {
                                        if *m.offset(j.wrapping_add(7 as libc::c_uint) as isize)
                                            as libc::c_int
                                            > *m.offset(j.wrapping_add(8 as libc::c_uint) as isize)
                                                as libc::c_int
                                        {
                                            if (*m.offset(j.wrapping_add(8 as libc::c_uint) as isize)
                                                as libc::c_int)
                                                < *m.offset(j.wrapping_add(9 as libc::c_uint) as isize)
                                                    as libc::c_int
                                            {
                                                if !(*m.offset(j.wrapping_add(9 as libc::c_uint) as isize)
                                                    as libc::c_int
                                                    > *m.offset(j.wrapping_add(6 as libc::c_uint) as isize)
                                                        as libc::c_int)
                                                {
                                                    current_block = 15946799889753539359;
                                                } else {
                                                    high = (*m.offset(j as isize) as libc::c_int
                                                        + *m.offset(j.wrapping_add(2 as libc::c_uint) as isize)
                                                            as libc::c_int
                                                        + *m.offset(j.wrapping_add(7 as libc::c_uint) as isize)
                                                            as libc::c_int
                                                        + *m.offset(j.wrapping_add(9 as libc::c_uint) as isize)
                                                            as libc::c_int) / 6 as libc::c_int;
                                                    if *m.offset(j.wrapping_add(4 as libc::c_uint) as isize)
                                                        as libc::c_int >= high
                                                    {
                                                        current_block = 13643677783189868280;
                                                    } else if *m
                                                            .offset(j.wrapping_add(5 as libc::c_uint) as isize)
                                                            as libc::c_int >= high
                                                        {
                                                        current_block = 13643677783189868280;
                                                    } else {
                                                        if *m.offset(j.wrapping_add(11 as libc::c_uint) as isize)
                                                            as libc::c_int >= high
                                                        {
                                                            current_block = 14686173634295280353;
                                                        } else if *m
                                                                .offset(j.wrapping_add(12 as libc::c_uint) as isize)
                                                                as libc::c_int >= high
                                                            {
                                                            current_block = 14686173634295280353;
                                                        } else if *m
                                                                .offset(j.wrapping_add(13 as libc::c_uint) as isize)
                                                                as libc::c_int >= high
                                                            {
                                                            current_block = 14686173634295280353;
                                                        } else if *m
                                                                .offset(j.wrapping_add(14 as libc::c_uint) as isize)
                                                                as libc::c_int >= high
                                                            {
                                                            current_block = 14686173634295280353;
                                                        } else {
                                                            Modes.stat_valid_preamble += 1;
                                                            current_block = 7294691061932571495;
                                                        }
                                                        match current_block {
                                                            7294691061932571495 => {}
                                                            _ => {
                                                                if Modes.debug & (1 as libc::c_int) << 4 as libc::c_int != 0
                                                                {
                                                                    if *m.offset(j as isize) as libc::c_int > 25 as libc::c_int
                                                                    {
                                                                        dumpRawMessage(
                                                                            b"Too high level in samples between 10 and 15\0"
                                                                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                                                                            msg.as_mut_ptr(),
                                                                            m,
                                                                            j,
                                                                        );
                                                                    }
                                                                }
                                                                current_block = 5851367039589866568;
                                                            }
                                                        }
                                                    }
                                                    match current_block {
                                                        7294691061932571495 => {}
                                                        5851367039589866568 => {}
                                                        _ => {
                                                            if Modes.debug & (1 as libc::c_int) << 4 as libc::c_int != 0
                                                            {
                                                                if *m.offset(j as isize) as libc::c_int > 25 as libc::c_int
                                                                {
                                                                    dumpRawMessage(
                                                                        b"Too high level in samples between 3 and 6\0" as *const u8
                                                                            as *const libc::c_char as *mut libc::c_char,
                                                                        msg.as_mut_ptr(),
                                                                        m,
                                                                        j,
                                                                    );
                                                                }
                                                            }
                                                            current_block = 5851367039589866568;
                                                        }
                                                    }
                                                }
                                            } else {
                                                current_block = 15946799889753539359;
                                            }
                                        } else {
                                            current_block = 15946799889753539359;
                                        }
                                    } else {
                                        current_block = 15946799889753539359;
                                    }
                                } else {
                                    current_block = 15946799889753539359;
                                }
                            } else {
                                current_block = 15946799889753539359;
                            }
                        } else {
                            current_block = 15946799889753539359;
                        }
                    } else {
                        current_block = 15946799889753539359;
                    }
                } else {
                    current_block = 15946799889753539359;
                }
            } else {
                current_block = 15946799889753539359;
            }
            match current_block {
                7294691061932571495 => {}
                5851367039589866568 => {}
                _ => {
                    if Modes.debug & (1 as libc::c_int) << 4 as libc::c_int != 0 {
                        if *m.offset(j as isize) as libc::c_int > 25 as libc::c_int {
                            dumpRawMessage(
                                b"Unexpected ratio among first 10 samples\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                                msg.as_mut_ptr(),
                                m,
                                j,
                            );
                        }
                    }
                    current_block = 5851367039589866568;
                }
            }
        }
        match current_block {
            7294691061932571495 => {
                if use_correction != 0 {
                    memcpy(
                        aux.as_mut_ptr() as *mut libc::c_void,
                        m.offset(j as isize).offset(16 as libc::c_int as isize)
                            as *const libc::c_void,
                        ::std::mem::size_of::<[uint16_t; 224]>() as libc::c_ulong,
                    );
                    if j != 0 {
                        tmp = detectOutOfPhase(m.offset(j as isize));
                        if tmp != 0 {
                            applyPhaseCorrection(m.offset(j as isize));
                            Modes.stat_out_of_phase += 1;
                        }
                    }
                }
                errors = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < 224 as libc::c_int {
                    low = *m
                        .offset(
                            j
                                .wrapping_add(i as uint32_t)
                                .wrapping_add(16 as libc::c_uint) as isize,
                        ) as libc::c_int;
                    high = *m
                        .offset(
                            j
                                .wrapping_add(i as uint32_t)
                                .wrapping_add(16 as libc::c_uint)
                                .wrapping_add(1 as libc::c_uint) as isize,
                        ) as libc::c_int;
                    delta = low - high;
                    if delta < 0 as libc::c_int {
                        delta = -delta;
                    }
                    let mut current_block_50: u64;
                    if i > 0 as libc::c_int {
                        if delta < 256 as libc::c_int {
                            bits[(i / 2 as libc::c_int)
                                as usize] = bits[(i / 2 as libc::c_int - 1 as libc::c_int)
                                as usize];
                            current_block_50 = 6838274324784804404;
                        } else {
                            current_block_50 = 5696151258243725727;
                        }
                    } else {
                        current_block_50 = 5696151258243725727;
                    }
                    match current_block_50 {
                        5696151258243725727 => {
                            if low == high {
                                bits[(i / 2 as libc::c_int)
                                    as usize] = 2 as libc::c_int as libc::c_uchar;
                                if i < 112 as libc::c_int {
                                    errors += 1;
                                }
                            } else if low > high {
                                bits[(i / 2 as libc::c_int)
                                    as usize] = 1 as libc::c_int as libc::c_uchar;
                            } else {
                                bits[(i / 2 as libc::c_int)
                                    as usize] = 0 as libc::c_int as libc::c_uchar;
                            }
                        }
                        _ => {}
                    }
                    i += 2 as libc::c_int;
                }
                if use_correction != 0 {
                    memcpy(
                        m.offset(j as isize).offset(16 as libc::c_int as isize)
                            as *mut libc::c_void,
                        aux.as_mut_ptr() as *const libc::c_void,
                        ::std::mem::size_of::<[uint16_t; 224]>() as libc::c_ulong,
                    );
                }
                i = 0 as libc::c_int;
                while i < 112 as libc::c_int {
                    msg[(i / 8 as libc::c_int)
                        as usize] = ((bits[i as usize] as libc::c_int)
                        << 7 as libc::c_int
                        | (bits[(i + 1 as libc::c_int) as usize] as libc::c_int)
                            << 6 as libc::c_int
                        | (bits[(i + 2 as libc::c_int) as usize] as libc::c_int)
                            << 5 as libc::c_int
                        | (bits[(i + 3 as libc::c_int) as usize] as libc::c_int)
                            << 4 as libc::c_int
                        | (bits[(i + 4 as libc::c_int) as usize] as libc::c_int)
                            << 3 as libc::c_int
                        | (bits[(i + 5 as libc::c_int) as usize] as libc::c_int)
                            << 2 as libc::c_int
                        | (bits[(i + 6 as libc::c_int) as usize] as libc::c_int)
                            << 1 as libc::c_int
                        | bits[(i + 7 as libc::c_int) as usize] as libc::c_int)
                        as libc::c_uchar;
                    i += 8 as libc::c_int;
                }
                msgtype = msg[0 as libc::c_int as usize] as libc::c_int
                    >> 3 as libc::c_int;
                tmp___0 = modesMessageLenByType(msgtype);
                msglen = tmp___0 / 8 as libc::c_int;
                delta = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < msglen * 8 as libc::c_int * 2 as libc::c_int {
                    tmp___1 = abs(
                        *m
                            .offset(
                                j
                                    .wrapping_add(i as uint32_t)
                                    .wrapping_add(16 as libc::c_uint) as isize,
                            ) as libc::c_int
                            - *m
                                .offset(
                                    j
                                        .wrapping_add(i as uint32_t)
                                        .wrapping_add(16 as libc::c_uint)
                                        .wrapping_add(1 as libc::c_uint) as isize,
                                ) as libc::c_int,
                    );
                    delta += tmp___1;
                    i += 2 as libc::c_int;
                }
                delta /= msglen * 4 as libc::c_int;
                if delta < 2550 as libc::c_int {
                    use_correction = 0 as libc::c_int;
                } else {
                    if errors == 0 as libc::c_int {
                        current_block = 4105741179806547613;
                    } else {
                        if Modes.aggressive != 0 {
                            if errors < 3 as libc::c_int {
                                current_block = 4105741179806547613;
                            } else {
                                current_block = 10781687707243313814;
                            }
                        } else {
                            current_block = 10781687707243313814;
                        }
                        match current_block {
                            4105741179806547613 => {}
                            _ => {
                                if Modes.debug & (1 as libc::c_int) << 1 as libc::c_int != 0
                                {
                                    if use_correction != 0 {
                                        printf(
                                            b"The following message has %d demod errors\n\0"
                                                as *const u8 as *const libc::c_char,
                                            errors,
                                        );
                                        dumpRawMessage(
                                            b"Demodulated with errors\0" as *const u8
                                                as *const libc::c_char as *mut libc::c_char,
                                            msg.as_mut_ptr(),
                                            m,
                                            j,
                                        );
                                    }
                                }
                                current_block = 1228639923084383292;
                            }
                        }
                    }
                    match current_block {
                        4105741179806547613 => {
                            decodeModesMessage(&mut mm, msg.as_mut_ptr());
                            let mut current_block_93: u64;
                            if mm.crcok != 0 {
                                current_block_93 = 3827544416127762281;
                            } else if use_correction != 0 {
                                current_block_93 = 3827544416127762281;
                            } else {
                                current_block_93 = 12696043255897098083;
                            }
                            match current_block_93 {
                                3827544416127762281 => {
                                    if errors == 0 as libc::c_int {
                                        Modes.stat_demodulated += 1;
                                    }
                                    if mm.errorbit == -(1 as libc::c_int) {
                                        if mm.crcok != 0 {
                                            Modes.stat_goodcrc += 1;
                                        } else {
                                            Modes.stat_badcrc += 1;
                                        }
                                    } else {
                                        Modes.stat_badcrc += 1;
                                        Modes.stat_fixed += 1;
                                        if mm.errorbit < 112 as libc::c_int {
                                            Modes.stat_single_bit_fix += 1;
                                        } else {
                                            Modes.stat_two_bits_fix += 1;
                                        }
                                    }
                                }
                                _ => {}
                            }
                            if use_correction == 0 as libc::c_int {
                                if Modes.debug & 1 as libc::c_int != 0 {
                                    dumpRawMessage(
                                        b"Demodulated with 0 errors\0" as *const u8
                                            as *const libc::c_char as *mut libc::c_char,
                                        msg.as_mut_ptr(),
                                        m,
                                        j,
                                    );
                                } else {
                                    if Modes.debug & (1 as libc::c_int) << 2 as libc::c_int != 0
                                    {
                                        if mm.msgtype == 17 as libc::c_int {
                                            if mm.crcok == 0 {
                                                dumpRawMessage(
                                                    b"Decoded with bad CRC\0" as *const u8
                                                        as *const libc::c_char as *mut libc::c_char,
                                                    msg.as_mut_ptr(),
                                                    m,
                                                    j,
                                                );
                                                current_block = 9008220588464243015;
                                            } else if mm.errorbit != -(1 as libc::c_int) {
                                                dumpRawMessage(
                                                    b"Decoded with bad CRC\0" as *const u8
                                                        as *const libc::c_char as *mut libc::c_char,
                                                    msg.as_mut_ptr(),
                                                    m,
                                                    j,
                                                );
                                                current_block = 9008220588464243015;
                                            } else {
                                                current_block = 3887096228345705911;
                                            }
                                        } else {
                                            current_block = 3887096228345705911;
                                        }
                                    } else {
                                        current_block = 3887096228345705911;
                                    }
                                    match current_block {
                                        9008220588464243015 => {}
                                        _ => {
                                            if Modes.debug & (1 as libc::c_int) << 3 as libc::c_int != 0
                                            {
                                                if mm.crcok != 0 {
                                                    if mm.errorbit == -(1 as libc::c_int) {
                                                        dumpRawMessage(
                                                            b"Decoded with good CRC\0" as *const u8
                                                                as *const libc::c_char as *mut libc::c_char,
                                                            msg.as_mut_ptr(),
                                                            m,
                                                            j,
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            if mm.crcok != 0 {
                                j = (j as libc::c_uint)
                                    .wrapping_add(
                                        ((8 as libc::c_int + msglen * 8 as libc::c_int)
                                            * 2 as libc::c_int) as uint32_t,
                                    ) as uint32_t as uint32_t;
                                good_message = 1 as libc::c_int;
                                if use_correction != 0 {
                                    mm.phase_corrected = 1 as libc::c_int;
                                }
                            }
                            useModesMessage(&mut mm);
                        }
                        _ => {}
                    }
                    if good_message == 0 {
                        if use_correction == 0 {
                            j = j.wrapping_sub(1);
                            use_correction = 1 as libc::c_int;
                        } else {
                            use_correction = 0 as libc::c_int;
                        }
                    } else {
                        use_correction = 0 as libc::c_int;
                    }
                }
            }
            _ => {}
        }
        j = j.wrapping_add(1);
    }
}
pub unsafe extern "C" fn useModesMessage(mut mm: *mut modesMessage) {
    let mut current_block: u64;
    let mut a: *mut aircraft = 0 as *mut aircraft;
    let mut tmp: *mut aircraft = 0 as *mut aircraft;
    if Modes.stats == 0 {
        if Modes.check_crc == 0 as libc::c_int {
            current_block = 10001216879927178565;
        } else if (*mm).crcok != 0 {
            current_block = 10001216879927178565;
        } else {
            current_block = 15768484401365413375;
        }
        match current_block {
            15768484401365413375 => {}
            _ => {
                if Modes.interactive != 0 {
                    current_block = 17263157218925100016;
                } else if Modes.stat_http_requests > 0 as libc::c_longlong {
                    current_block = 17263157218925100016;
                } else if Modes.stat_sbs_connections > 0 as libc::c_longlong {
                    current_block = 17263157218925100016;
                } else {
                    current_block = 3512920355445576850;
                }
                match current_block {
                    17263157218925100016 => {
                        tmp = interactiveReceiveData(mm);
                        a = tmp;
                        if !a.is_null() {
                            if Modes.stat_sbs_connections > 0 as libc::c_longlong {
                                modesSendSBSOutput(mm, a);
                            }
                        }
                    }
                    _ => {}
                }
                if Modes.interactive == 0 {
                    displayModesMessage(mm);
                    if Modes.raw == 0 {
                        if Modes.onlyaddr == 0 {
                            printf(b"\n\0" as *const u8 as *const libc::c_char);
                        }
                    }
                }
                if Modes.net != 0 {
                    modesSendRawOutput(mm);
                }
            }
        }
    }
}
pub unsafe extern "C" fn interactiveCreateAircraft(mut addr: uint32_t) -> *mut aircraft {
    let mut a: *mut aircraft = 0 as *mut aircraft;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<aircraft>() as libc::c_ulong);
    a = tmp as *mut aircraft;
    (*a).addr = addr;
    snprintf(
        ((*a).hexaddr).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong,
        b"%06x\0" as *const u8 as *const libc::c_char,
        addr as libc::c_int,
    );
    (*a).flight[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    (*a).altitude = 0 as libc::c_int;
    (*a).speed = 0 as libc::c_int;
    (*a).track = 0 as libc::c_int;
    (*a).odd_cprlat = 0 as libc::c_int;
    (*a).odd_cprlon = 0 as libc::c_int;
    (*a).odd_cprtime = 0 as libc::c_longlong;
    (*a).even_cprlat = 0 as libc::c_int;
    (*a).even_cprlon = 0 as libc::c_int;
    (*a).even_cprtime = 0 as libc::c_longlong;
    (*a).lat = 0 as libc::c_int as libc::c_double;
    (*a).lon = 0 as libc::c_int as libc::c_double;
    (*a).seen = time(0 as *mut libc::c_void as *mut time_t);
    (*a).messages = 0 as libc::c_long;
    (*a).next = 0 as *mut libc::c_void as *mut aircraft;
    return a;
}
pub unsafe extern "C" fn interactiveFindAircraft(mut addr: uint32_t) -> *mut aircraft {
    let mut a: *mut aircraft = 0 as *mut aircraft;
    a = Modes.aircrafts;
    while !a.is_null() {
        if (*a).addr == addr {
            return a;
        }
        a = (*a).next;
    }
    return 0 as *mut libc::c_void as *mut aircraft;
}
pub unsafe extern "C" fn cprModFunction(
    mut a: libc::c_int,
    mut b: libc::c_int,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    res = a % b;
    if res < 0 as libc::c_int {
        res += b;
    }
    return res;
}
pub unsafe extern "C" fn cprNLFunction(mut lat: libc::c_double) -> libc::c_int {
    if lat < 0 as libc::c_int as libc::c_double {
        lat = -lat;
    }
    if lat < 10.47047130f64 {
        return 59 as libc::c_int;
    }
    if lat < 14.82817437f64 {
        return 58 as libc::c_int;
    }
    if lat < 18.18626357f64 {
        return 57 as libc::c_int;
    }
    if lat < 21.02939493f64 {
        return 56 as libc::c_int;
    }
    if lat < 23.54504487f64 {
        return 55 as libc::c_int;
    }
    if lat < 25.82924707f64 {
        return 54 as libc::c_int;
    }
    if lat < 27.93898710f64 {
        return 53 as libc::c_int;
    }
    if lat < 29.91135686f64 {
        return 52 as libc::c_int;
    }
    if lat < 31.77209708f64 {
        return 51 as libc::c_int;
    }
    if lat < 33.53993436f64 {
        return 50 as libc::c_int;
    }
    if lat < 35.22899598f64 {
        return 49 as libc::c_int;
    }
    if lat < 36.85025108f64 {
        return 48 as libc::c_int;
    }
    if lat < 38.41241892f64 {
        return 47 as libc::c_int;
    }
    if lat < 39.92256684f64 {
        return 46 as libc::c_int;
    }
    if lat < 41.38651832f64 {
        return 45 as libc::c_int;
    }
    if lat < 42.80914012f64 {
        return 44 as libc::c_int;
    }
    if lat < 44.19454951f64 {
        return 43 as libc::c_int;
    }
    if lat < 45.54626723f64 {
        return 42 as libc::c_int;
    }
    if lat < 46.86733252f64 {
        return 41 as libc::c_int;
    }
    if lat < 48.16039128f64 {
        return 40 as libc::c_int;
    }
    if lat < 49.42776439f64 {
        return 39 as libc::c_int;
    }
    if lat < 50.67150166f64 {
        return 38 as libc::c_int;
    }
    if lat < 51.89342469f64 {
        return 37 as libc::c_int;
    }
    if lat < 53.09516153f64 {
        return 36 as libc::c_int;
    }
    if lat < 54.27817472f64 {
        return 35 as libc::c_int;
    }
    if lat < 55.44378444f64 {
        return 34 as libc::c_int;
    }
    if lat < 56.59318756f64 {
        return 33 as libc::c_int;
    }
    if lat < 57.72747354f64 {
        return 32 as libc::c_int;
    }
    if lat < 58.84763776f64 {
        return 31 as libc::c_int;
    }
    if lat < 59.95459277f64 {
        return 30 as libc::c_int;
    }
    if lat < 61.04917774f64 {
        return 29 as libc::c_int;
    }
    if lat < 62.13216659f64 {
        return 28 as libc::c_int;
    }
    if lat < 63.20427479f64 {
        return 27 as libc::c_int;
    }
    if lat < 64.26616523f64 {
        return 26 as libc::c_int;
    }
    if lat < 65.31845310f64 {
        return 25 as libc::c_int;
    }
    if lat < 66.36171008f64 {
        return 24 as libc::c_int;
    }
    if lat < 67.39646774f64 {
        return 23 as libc::c_int;
    }
    if lat < 68.42322022f64 {
        return 22 as libc::c_int;
    }
    if lat < 69.44242631f64 {
        return 21 as libc::c_int;
    }
    if lat < 70.45451075f64 {
        return 20 as libc::c_int;
    }
    if lat < 71.45986473f64 {
        return 19 as libc::c_int;
    }
    if lat < 72.45884545f64 {
        return 18 as libc::c_int;
    }
    if lat < 73.45177442f64 {
        return 17 as libc::c_int;
    }
    if lat < 74.43893416f64 {
        return 16 as libc::c_int;
    }
    if lat < 75.42056257f64 {
        return 15 as libc::c_int;
    }
    if lat < 76.39684391f64 {
        return 14 as libc::c_int;
    }
    if lat < 77.36789461f64 {
        return 13 as libc::c_int;
    }
    if lat < 78.33374083f64 {
        return 12 as libc::c_int;
    }
    if lat < 79.29428225f64 {
        return 11 as libc::c_int;
    }
    if lat < 80.24923213f64 {
        return 10 as libc::c_int;
    }
    if lat < 81.19801349f64 {
        return 9 as libc::c_int;
    }
    if lat < 82.13956981f64 {
        return 8 as libc::c_int;
    }
    if lat < 83.07199445f64 {
        return 7 as libc::c_int;
    }
    if lat < 83.99173563f64 {
        return 6 as libc::c_int;
    }
    if lat < 84.89166191f64 {
        return 5 as libc::c_int;
    }
    if lat < 85.75541621f64 {
        return 4 as libc::c_int;
    }
    if lat < 86.53536998f64 {
        return 3 as libc::c_int;
    }
    if lat < 87.00000000f64 { return 2 as libc::c_int } else { return 1 as libc::c_int };
}
pub unsafe extern "C" fn cprNFunction(
    mut lat: libc::c_double,
    mut isodd: libc::c_int,
) -> libc::c_int {
    let mut nl: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tmp = cprNLFunction(lat);
    nl = tmp - isodd;
    if nl < 1 as libc::c_int {
        nl = 1 as libc::c_int;
    }
    return nl;
}
pub unsafe extern "C" fn cprDlonFunction(
    mut lat: libc::c_double,
    mut isodd: libc::c_int,
) -> libc::c_double {
    let mut tmp: libc::c_int = 0;
    tmp = cprNFunction(lat, isodd);
    return 360.0f64 / tmp as libc::c_double;
}
pub unsafe extern "C" fn decodeCPR(mut a: *mut aircraft) {
    let mut AirDlat0: libc::c_double = 0.;
    let mut AirDlat1: libc::c_double = 0.;
    let mut lat0: libc::c_double = 0.;
    let mut lat1: libc::c_double = 0.;
    let mut lon0: libc::c_double = 0.;
    let mut lon1: libc::c_double = 0.;
    let mut j: libc::c_int = 0;
    let mut tmp: libc::c_double = 0.;
    let mut rlat0: libc::c_double = 0.;
    let mut tmp___0: libc::c_int = 0;
    let mut rlat1: libc::c_double = 0.;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut ni: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_double = 0.;
    let mut tmp___8: libc::c_double = 0.;
    let mut tmp___9: libc::c_int = 0;
    let mut ni___0: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut m___0: libc::c_int = 0;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: libc::c_int = 0;
    let mut tmp___13: libc::c_double = 0.;
    let mut tmp___14: libc::c_double = 0.;
    let mut tmp___15: libc::c_int = 0;
    AirDlat0 = 360.0f64 / 60 as libc::c_int as libc::c_double;
    AirDlat1 = 360.0f64 / 59 as libc::c_int as libc::c_double;
    lat0 = (*a).even_cprlat as libc::c_double;
    lat1 = (*a).odd_cprlat as libc::c_double;
    lon0 = (*a).even_cprlon as libc::c_double;
    lon1 = (*a).odd_cprlon as libc::c_double;
    tmp = floor(
        (59 as libc::c_int as libc::c_double * lat0
            - 60 as libc::c_int as libc::c_double * lat1)
            / 131072 as libc::c_int as libc::c_double + 0.5f64,
    );
    j = tmp as libc::c_int;
    tmp___0 = cprModFunction(j, 60 as libc::c_int);
    rlat0 = AirDlat0
        * (tmp___0 as libc::c_double + lat0 / 131072 as libc::c_int as libc::c_double);
    tmp___1 = cprModFunction(j, 59 as libc::c_int);
    rlat1 = AirDlat1
        * (tmp___1 as libc::c_double + lat1 / 131072 as libc::c_int as libc::c_double);
    if rlat0 >= 270 as libc::c_int as libc::c_double {
        rlat0 -= 360 as libc::c_int as libc::c_double;
    }
    if rlat1 >= 270 as libc::c_int as libc::c_double {
        rlat1 -= 360 as libc::c_int as libc::c_double;
    }
    tmp___2 = cprNLFunction(rlat0);
    tmp___3 = cprNLFunction(rlat1);
    if tmp___2 != tmp___3 {
        return;
    }
    if (*a).even_cprtime > (*a).odd_cprtime {
        tmp___4 = cprNFunction(rlat0, 0 as libc::c_int);
        ni = tmp___4;
        tmp___5 = cprNLFunction(rlat0);
        tmp___6 = cprNLFunction(rlat0);
        tmp___7 = floor(
            (lon0 * (tmp___5 - 1 as libc::c_int) as libc::c_double
                - lon1 * tmp___6 as libc::c_double)
                / 131072 as libc::c_int as libc::c_double + 0.5f64,
        );
        m = tmp___7 as libc::c_int;
        tmp___8 = cprDlonFunction(rlat0, 0 as libc::c_int);
        tmp___9 = cprModFunction(m, ni);
        (*a)
            .lon = tmp___8
            * (tmp___9 as libc::c_double
                + lon0 / 131072 as libc::c_int as libc::c_double);
        (*a).lat = rlat0;
    } else {
        tmp___10 = cprNFunction(rlat1, 1 as libc::c_int);
        ni___0 = tmp___10;
        tmp___11 = cprNLFunction(rlat1);
        tmp___12 = cprNLFunction(rlat1);
        tmp___13 = floor(
            (lon0 * (tmp___11 - 1 as libc::c_int) as libc::c_double
                - lon1 * tmp___12 as libc::c_double) / 131072.0f64 + 0.5f64,
        );
        m___0 = tmp___13 as libc::c_int;
        tmp___14 = cprDlonFunction(rlat1, 1 as libc::c_int);
        tmp___15 = cprModFunction(m___0, ni___0);
        (*a)
            .lon = tmp___14
            * (tmp___15 as libc::c_double
                + lon1 / 131072 as libc::c_int as libc::c_double);
        (*a).lat = rlat1;
    }
    if (*a).lon > 180 as libc::c_int as libc::c_double {
        (*a).lon -= 360 as libc::c_int as libc::c_double;
    }
}
pub unsafe extern "C" fn interactiveReceiveData(
    mut mm: *mut modesMessage,
) -> *mut aircraft {
    let mut addr: uint32_t = 0;
    let mut a: *mut aircraft = 0 as *mut aircraft;
    let mut tmp___0: libc::c_int = 0;
    if Modes.check_crc != 0 {
        if (*mm).crcok == 0 as libc::c_int {
            return 0 as *mut libc::c_void as *mut aircraft;
        }
    }
    addr = ((*mm).aa1 << 16 as libc::c_int | (*mm).aa2 << 8 as libc::c_int | (*mm).aa3)
        as uint32_t;
    a = interactiveFindAircraft(addr);
    if a.is_null() {
        a = interactiveCreateAircraft(addr);
        (*a).next = Modes.aircrafts;
        Modes.aircrafts = a;
    }
    (*a).seen = time(0 as *mut libc::c_void as *mut time_t);
    (*a).messages += 1;
    if (*mm).msgtype == 0 as libc::c_int {
        (*a).altitude = (*mm).altitude;
    } else if (*mm).msgtype == 4 as libc::c_int {
        (*a).altitude = (*mm).altitude;
    } else if (*mm).msgtype == 20 as libc::c_int {
        (*a).altitude = (*mm).altitude;
    } else if (*mm).msgtype == 17 as libc::c_int {
        let mut current_block_47: u64;
        if (*mm).metype >= 1 as libc::c_int {
            if (*mm).metype <= 4 as libc::c_int {
                memcpy(
                    ((*a).flight).as_mut_ptr() as *mut libc::c_void,
                    ((*mm).flight).as_mut_ptr() as *const libc::c_void,
                    ::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
                );
                current_block_47 = 9441801433784995173;
            } else {
                current_block_47 = 2272822364267007041;
            }
        } else {
            current_block_47 = 2272822364267007041;
        }
        match current_block_47 {
            2272822364267007041 => {
                let mut current_block_46: u64;
                if (*mm).metype >= 9 as libc::c_int {
                    if (*mm).metype <= 18 as libc::c_int {
                        (*a).altitude = (*mm).altitude;
                        if (*mm).fflag != 0 {
                            (*a).odd_cprlat = (*mm).raw_latitude;
                            (*a).odd_cprlon = (*mm).raw_longitude;
                            (*a).odd_cprtime = mstime();
                        } else {
                            (*a).even_cprlat = (*mm).raw_latitude;
                            (*a).even_cprlon = (*mm).raw_longitude;
                            (*a).even_cprtime = mstime();
                        }
                        tmp___0 = abs(
                            ((*a).even_cprtime - (*a).odd_cprtime) as libc::c_int,
                        );
                        if tmp___0 <= 10000 as libc::c_int {
                            decodeCPR(a);
                        }
                        current_block_46 = 1847472278776910194;
                    } else {
                        current_block_46 = 7756716813061980319;
                    }
                } else {
                    current_block_46 = 7756716813061980319;
                }
                match current_block_46 {
                    7756716813061980319 => {
                        if (*mm).metype == 19 as libc::c_int {
                            if (*mm).mesub == 1 as libc::c_int {
                                (*a).speed = (*mm).velocity;
                                (*a).track = (*mm).heading;
                            } else if (*mm).mesub == 2 as libc::c_int {
                                (*a).speed = (*mm).velocity;
                                (*a).track = (*mm).heading;
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    return a;
}
pub unsafe extern "C" fn interactiveShowData() {
    let mut a: *mut aircraft = 0 as *mut aircraft;
    let mut now: time_t = 0;
    let mut tmp: time_t = 0;
    let mut progress: [libc::c_char; 4] = [0; 4];
    let mut count: libc::c_int = 0;
    let mut tmp___0: time_t = 0;
    let mut altitude: libc::c_int = 0;
    let mut speed: libc::c_int = 0;
    a = Modes.aircrafts;
    tmp = time(0 as *mut libc::c_void as *mut time_t);
    now = tmp;
    count = 0 as libc::c_int;
    memset(
        progress.as_mut_ptr() as *mut libc::c_void,
        ' ' as i32,
        3 as libc::c_int as size_t,
    );
    tmp___0 = time(0 as *mut libc::c_void as *mut time_t);
    progress[(tmp___0 % 3 as libc::c_long) as usize] = '.' as i32 as libc::c_char;
    progress[3 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    printf(b"\x1B[H\x1B[2J\0" as *const u8 as *const libc::c_char);
    printf(
        b"Hex    Flight   Altitude  Speed   Lat       Lon       Track  Messages Seen %s\n--------------------------------------------------------------------------------\n\0"
            as *const u8 as *const libc::c_char,
        progress.as_mut_ptr(),
    );
    while !a.is_null() {
        if !(count < Modes.interactive_rows) {
            break;
        }
        altitude = (*a).altitude;
        speed = (*a).speed;
        if Modes.metric != 0 {
            altitude = (altitude as libc::c_double / 3.2828f64) as libc::c_int;
            speed = (speed as libc::c_double * 1.852f64) as libc::c_int;
        }
        printf(
            b"%-6s %-8s %-9d %-7d %-7.03f   %-7.03f   %-3d   %-9ld %d sec\n\0"
                as *const u8 as *const libc::c_char,
            ((*a).hexaddr).as_mut_ptr(),
            ((*a).flight).as_mut_ptr(),
            altitude,
            speed,
            (*a).lat,
            (*a).lon,
            (*a).track,
            (*a).messages,
            (now - (*a).seen) as libc::c_int,
        );
        a = (*a).next;
        count += 1;
    }
}
pub unsafe extern "C" fn interactiveRemoveStaleAircrafts() {
    let mut a: *mut aircraft = 0 as *mut aircraft;
    let mut prev: *mut aircraft = 0 as *mut aircraft;
    let mut now: time_t = 0;
    let mut tmp: time_t = 0;
    let mut next: *mut aircraft = 0 as *mut aircraft;
    a = Modes.aircrafts;
    prev = 0 as *mut libc::c_void as *mut aircraft;
    tmp = time(0 as *mut libc::c_void as *mut time_t);
    now = tmp;
    while !a.is_null() {
        if now - (*a).seen > Modes.interactive_ttl as time_t {
            next = (*a).next;
            free(a as *mut libc::c_void);
            if prev.is_null() {
                Modes.aircrafts = next;
            } else {
                (*prev).next = next;
            }
            a = next;
        } else {
            prev = a;
            a = (*a).next;
        }
    }
}
pub unsafe extern "C" fn snipMode(mut level: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut q: libc::c_int = 0;
    let mut c: libc::c_longlong = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    c = 0 as libc::c_longlong;
    loop {
        i = getchar();
        if !(i != -(1 as libc::c_int)) {
            break;
        }
        q = getchar();
        if !(q != -(1 as libc::c_int)) {
            break;
        }
        tmp = abs(i - 127 as libc::c_int);
        if tmp < level {
            tmp___0 = abs(q - 127 as libc::c_int);
            if tmp___0 < level {
                c += 1;
                if c > 32 as libc::c_longlong {
                    continue;
                }
            } else {
                c = 0 as libc::c_longlong;
            }
        } else {
            c = 0 as libc::c_longlong;
        }
        putchar(i);
        putchar(q);
    };
}
pub static mut modesNetServices: [__anonstruct_modesNetServices_483049182; 4] = [__anonstruct_modesNetServices_483049182 {
    descr: 0 as *mut libc::c_char,
    socket: 0 as *mut libc::c_int,
    port: 0,
}; 4];
pub unsafe extern "C" fn modesInitNet() {
    let mut j: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    memset(
        (Modes.clients).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[*mut client; 1024]>() as libc::c_ulong,
    );
    Modes.maxfd = -(1 as libc::c_int);
    j = 0 as libc::c_int;
    while j < 4 as libc::c_int {
        tmp = anetTcpServer(
            (Modes.aneterr).as_mut_ptr(),
            modesNetServices[j as usize].port,
            0 as *mut libc::c_void as *mut libc::c_char,
        );
        s = tmp;
        if s == -(1 as libc::c_int) {
            tmp___0 = __errno_location();
            tmp___1 = strerror(*tmp___0);
            fprintf(
                stderr,
                b"Error opening the listening port %d (%s): %s\n\0" as *const u8
                    as *const libc::c_char,
                modesNetServices[j as usize].port,
                modesNetServices[j as usize].descr,
                tmp___1,
            );
            exit(1 as libc::c_int);
        }
        anetNonBlock((Modes.aneterr).as_mut_ptr(), s);
        *modesNetServices[j as usize].socket = s;
        j += 1;
    }
    signal(
        13 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(1 as libc::c_int as libc::intptr_t),
    );
}
pub unsafe extern "C" fn modesAcceptClients() {
    let mut fd: libc::c_int = 0;
    let mut port: libc::c_int = 0;
    let mut j: libc::c_uint = 0;
    let mut c: *mut client = 0 as *mut client;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    j = 0 as libc::c_uint;
    while j < 4 as libc::c_uint {
        fd = anetTcpAccept(
            (Modes.aneterr).as_mut_ptr(),
            *modesNetServices[j as usize].socket,
            0 as *mut libc::c_void as *mut libc::c_char,
            &mut port,
        );
        if fd == -(1 as libc::c_int) {
            if Modes.debug & (1 as libc::c_int) << 5 as libc::c_int != 0 {
                tmp___1 = __errno_location();
                if *tmp___1 != 11 as libc::c_int {
                    tmp = __errno_location();
                    tmp___0 = strerror(*tmp);
                    printf(
                        b"Accept %d: %s\n\0" as *const u8 as *const libc::c_char,
                        *modesNetServices[j as usize].socket,
                        tmp___0,
                    );
                }
            }
        } else {
            if fd >= 1024 as libc::c_int {
                close(fd);
                return;
            }
            anetNonBlock((Modes.aneterr).as_mut_ptr(), fd);
            tmp___2 = malloc(::std::mem::size_of::<client>() as libc::c_ulong);
            c = tmp___2 as *mut client;
            (*c).service = *modesNetServices[j as usize].socket;
            (*c).fd = fd;
            (*c).buflen = 0 as libc::c_int;
            Modes.clients[fd as usize] = c;
            anetSetSendBuffer((Modes.aneterr).as_mut_ptr(), fd, 65536 as libc::c_int);
            if Modes.maxfd < fd {
                Modes.maxfd = fd;
            }
            if *modesNetServices[j as usize].socket == Modes.sbsos {
                Modes.stat_sbs_connections += 1;
            }
            j = j.wrapping_sub(1);
            if Modes.debug & (1 as libc::c_int) << 5 as libc::c_int != 0 {
                printf(
                    b"Created new client %d\n\0" as *const u8 as *const libc::c_char,
                    fd,
                );
            }
        }
        j = j.wrapping_add(1);
    }
}
pub unsafe extern "C" fn modesFreeClient(mut fd: libc::c_int) {
    let mut j: libc::c_int = 0;
    close(fd);
    free(Modes.clients[fd as usize] as *mut libc::c_void);
    Modes.clients[fd as usize] = 0 as *mut libc::c_void as *mut client;
    if Modes.debug & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        printf(b"Closing client %d\n\0" as *const u8 as *const libc::c_char, fd);
    }
    if Modes.maxfd == fd {
        Modes.maxfd = -(1 as libc::c_int);
        j = fd - 1 as libc::c_int;
        while j >= 0 as libc::c_int {
            if !(Modes.clients[j as usize]).is_null() {
                Modes.maxfd = j;
                break;
            } else {
                j -= 1;
            }
        }
    }
}
pub unsafe extern "C" fn modesSendAllClients(
    mut service: libc::c_int,
    mut msg: *mut libc::c_void,
    mut len: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    let mut c: *mut client = 0 as *mut client;
    let mut nwritten: libc::c_int = 0;
    let mut tmp: ssize_t = 0;
    j = 0 as libc::c_int;
    while j <= Modes.maxfd {
        c = Modes.clients[j as usize];
        if !c.is_null() {
            if (*c).service == service {
                tmp = write(j, msg as *const libc::c_void, len as size_t);
                nwritten = tmp as libc::c_int;
                if nwritten != len {
                    modesFreeClient(j);
                }
            }
        }
        j += 1;
    }
}
pub unsafe extern "C" fn modesSendRawOutput(mut mm: *mut modesMessage) {
    let mut msg: [libc::c_char; 128] = [0; 128];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut j: libc::c_int = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    p = msg.as_mut_ptr();
    tmp = p;
    p = p.offset(1);
    *tmp = '*' as i32 as libc::c_char;
    j = 0 as libc::c_int;
    while j < (*mm).msgbits / 8 as libc::c_int {
        sprintf(
            p,
            b"%02X\0" as *const u8 as *const libc::c_char,
            (*mm).msg[j as usize] as libc::c_int,
        );
        p = p.offset(2 as libc::c_int as isize);
        j += 1;
    }
    tmp___0 = p;
    p = p.offset(1);
    *tmp___0 = ';' as i32 as libc::c_char;
    tmp___1 = p;
    p = p.offset(1);
    *tmp___1 = '\n' as i32 as libc::c_char;
    modesSendAllClients(
        Modes.ros,
        msg.as_mut_ptr() as *mut libc::c_void,
        p.offset_from(msg.as_mut_ptr()) as libc::c_long as libc::c_int,
    );
}
pub unsafe extern "C" fn modesSendSBSOutput(
    mut mm: *mut modesMessage,
    mut a: *mut aircraft,
) {
    let mut current_block: u64;
    let mut msg: [libc::c_char; 256] = [0; 256];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut emergency: libc::c_int = 0;
    let mut ground: libc::c_int = 0;
    let mut alert: libc::c_int = 0;
    let mut spi: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut vr: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: *mut libc::c_char = 0 as *mut libc::c_char;
    p = msg.as_mut_ptr();
    emergency = 0 as libc::c_int;
    ground = 0 as libc::c_int;
    alert = 0 as libc::c_int;
    spi = 0 as libc::c_int;
    if (*mm).msgtype == 4 as libc::c_int {
        current_block = 17462100535891017519;
    } else if (*mm).msgtype == 5 as libc::c_int {
        current_block = 17462100535891017519;
    } else if (*mm).msgtype == 21 as libc::c_int {
        current_block = 17462100535891017519;
    } else {
        current_block = 7226443171521532240;
    }
    match current_block {
        17462100535891017519 => {
            if (*mm).identity == 7500 as libc::c_int {
                emergency = -(1 as libc::c_int);
            } else if (*mm).identity == 7600 as libc::c_int {
                emergency = -(1 as libc::c_int);
            } else if (*mm).identity == 7700 as libc::c_int {
                emergency = -(1 as libc::c_int);
            }
            if (*mm).fs == 1 as libc::c_int {
                ground = -(1 as libc::c_int);
            } else if (*mm).fs == 3 as libc::c_int {
                ground = -(1 as libc::c_int);
            }
            if (*mm).fs == 2 as libc::c_int {
                alert = -(1 as libc::c_int);
            } else if (*mm).fs == 3 as libc::c_int {
                alert = -(1 as libc::c_int);
            } else if (*mm).fs == 4 as libc::c_int {
                alert = -(1 as libc::c_int);
            }
            if (*mm).fs == 4 as libc::c_int {
                spi = -(1 as libc::c_int);
            } else if (*mm).fs == 5 as libc::c_int {
                spi = -(1 as libc::c_int);
            }
        }
        _ => {}
    }
    if (*mm).msgtype == 0 as libc::c_int {
        tmp = sprintf(
            p,
            b"MSG,5,,,%02X%02X%02X,,,,,,,%d,,,,,,,,,,\0" as *const u8
                as *const libc::c_char,
            (*mm).aa1,
            (*mm).aa2,
            (*mm).aa3,
            (*mm).altitude,
        );
        p = p.offset(tmp as isize);
    } else if (*mm).msgtype == 4 as libc::c_int {
        tmp___0 = sprintf(
            p,
            b"MSG,5,,,%02X%02X%02X,,,,,,,%d,,,,,,,%d,%d,%d,%d\0" as *const u8
                as *const libc::c_char,
            (*mm).aa1,
            (*mm).aa2,
            (*mm).aa3,
            (*mm).altitude,
            alert,
            emergency,
            spi,
            ground,
        );
        p = p.offset(tmp___0 as isize);
    } else if (*mm).msgtype == 5 as libc::c_int {
        tmp___1 = sprintf(
            p,
            b"MSG,6,,,%02X%02X%02X,,,,,,,,,,,,,%d,%d,%d,%d,%d\0" as *const u8
                as *const libc::c_char,
            (*mm).aa1,
            (*mm).aa2,
            (*mm).aa3,
            (*mm).identity,
            alert,
            emergency,
            spi,
            ground,
        );
        p = p.offset(tmp___1 as isize);
    } else if (*mm).msgtype == 11 as libc::c_int {
        tmp___2 = sprintf(
            p,
            b"MSG,8,,,%02X%02X%02X,,,,,,,,,,,,,,,,,\0" as *const u8
                as *const libc::c_char,
            (*mm).aa1,
            (*mm).aa2,
            (*mm).aa3,
        );
        p = p.offset(tmp___2 as isize);
    } else {
        if (*mm).msgtype == 17 as libc::c_int {
            if (*mm).metype == 4 as libc::c_int {
                tmp___3 = sprintf(
                    p,
                    b"MSG,1,,,%02X%02X%02X,,,,,,%s,,,,,,,,0,0,0,0\0" as *const u8
                        as *const libc::c_char,
                    (*mm).aa1,
                    (*mm).aa2,
                    (*mm).aa3,
                    ((*mm).flight).as_mut_ptr(),
                );
                p = p.offset(tmp___3 as isize);
                current_block = 14541395414537699361;
            } else {
                current_block = 9114401921673427385;
            }
        } else {
            current_block = 9114401921673427385;
        }
        match current_block {
            14541395414537699361 => {}
            _ => {
                if (*mm).msgtype == 17 as libc::c_int {
                    if (*mm).metype >= 9 as libc::c_int {
                        if (*mm).metype <= 18 as libc::c_int {
                            if (*a).lat == 0 as libc::c_int as libc::c_double {
                                if (*a).lon == 0 as libc::c_int as libc::c_double {
                                    tmp___4 = sprintf(
                                        p,
                                        b"MSG,3,,,%02X%02X%02X,,,,,,,%d,,,,,,,0,0,0,0\0"
                                            as *const u8 as *const libc::c_char,
                                        (*mm).aa1,
                                        (*mm).aa2,
                                        (*mm).aa3,
                                        (*mm).altitude,
                                    );
                                    p = p.offset(tmp___4 as isize);
                                } else {
                                    tmp___5 = sprintf(
                                        p,
                                        b"MSG,3,,,%02X%02X%02X,,,,,,,%d,,,%1.5f,%1.5f,,,0,0,0,0\0"
                                            as *const u8 as *const libc::c_char,
                                        (*mm).aa1,
                                        (*mm).aa2,
                                        (*mm).aa3,
                                        (*mm).altitude,
                                        (*a).lat,
                                        (*a).lon,
                                    );
                                    p = p.offset(tmp___5 as isize);
                                }
                            } else {
                                tmp___5 = sprintf(
                                    p,
                                    b"MSG,3,,,%02X%02X%02X,,,,,,,%d,,,%1.5f,%1.5f,,,0,0,0,0\0"
                                        as *const u8 as *const libc::c_char,
                                    (*mm).aa1,
                                    (*mm).aa2,
                                    (*mm).aa3,
                                    (*mm).altitude,
                                    (*a).lat,
                                    (*a).lon,
                                );
                                p = p.offset(tmp___5 as isize);
                            }
                            current_block = 14541395414537699361;
                        } else {
                            current_block = 531162408755124928;
                        }
                    } else {
                        current_block = 531162408755124928;
                    }
                } else {
                    current_block = 531162408755124928;
                }
                match current_block {
                    14541395414537699361 => {}
                    _ => {
                        if (*mm).msgtype == 17 as libc::c_int {
                            if (*mm).metype == 19 as libc::c_int {
                                if (*mm).mesub == 1 as libc::c_int {
                                    if (*mm).vert_rate_sign == 0 as libc::c_int {
                                        tmp___6 = 1 as libc::c_int;
                                    } else {
                                        tmp___6 = -(1 as libc::c_int);
                                    }
                                    vr = tmp___6 * ((*mm).vert_rate - 1 as libc::c_int)
                                        * 64 as libc::c_int;
                                    tmp___7 = sprintf(
                                        p,
                                        b"MSG,4,,,%02X%02X%02X,,,,,,,,%d,%d,,,%i,,0,0,0,0\0"
                                            as *const u8 as *const libc::c_char,
                                        (*mm).aa1,
                                        (*mm).aa2,
                                        (*mm).aa3,
                                        (*a).speed,
                                        (*a).track,
                                        vr,
                                    );
                                    p = p.offset(tmp___7 as isize);
                                    current_block = 14541395414537699361;
                                } else {
                                    current_block = 4696936569063667370;
                                }
                            } else {
                                current_block = 4696936569063667370;
                            }
                        } else {
                            current_block = 4696936569063667370;
                        }
                        match current_block {
                            14541395414537699361 => {}
                            _ => {
                                if (*mm).msgtype == 21 as libc::c_int {
                                    tmp___8 = sprintf(
                                        p,
                                        b"MSG,6,,,%02X%02X%02X,,,,,,,,,,,,,%d,%d,%d,%d,%d\0"
                                            as *const u8 as *const libc::c_char,
                                        (*mm).aa1,
                                        (*mm).aa2,
                                        (*mm).aa3,
                                        (*mm).identity,
                                        alert,
                                        emergency,
                                        spi,
                                        ground,
                                    );
                                    p = p.offset(tmp___8 as isize);
                                } else {
                                    return
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    tmp___9 = p;
    p = p.offset(1);
    *tmp___9 = '\n' as i32 as libc::c_char;
    modesSendAllClients(
        Modes.sbsos,
        msg.as_mut_ptr() as *mut libc::c_void,
        p.offset_from(msg.as_mut_ptr()) as libc::c_long as libc::c_int,
    );
}
pub unsafe extern "C" fn hexDigitVal(mut c: libc::c_int) -> libc::c_int {
    let mut __res: libc::c_int = 0;
    let mut tmp___0: *mut *const __int32_t = 0 as *mut *const __int32_t;
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong > 1 as libc::c_ulong {
        __res = tolower(c);
    } else {
        tmp___0 = __ctype_tolower_loc();
        __res = *(*tmp___0).offset(c as isize);
    }
    c = __res;
    if c >= 48 as libc::c_int {
        if c <= 57 as libc::c_int {
            return c - 48 as libc::c_int;
        }
    }
    if c >= 97 as libc::c_int {
        if c <= 102 as libc::c_int {
            return c - 97 as libc::c_int + 10 as libc::c_int
        } else {
            return -(1 as libc::c_int)
        }
    } else {
        return -(1 as libc::c_int)
    };
}
pub unsafe extern "C" fn decodeHexMessage(mut c: *mut client) -> libc::c_int {
    let mut hex: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut j: libc::c_int = 0;
    let mut msg: [libc::c_uchar; 14] = [0; 14];
    let mut mm: modesMessage = modesMessage {
        msg: [0; 14],
        msgbits: 0,
        msgtype: 0,
        crcok: 0,
        crc: 0,
        errorbit: 0,
        aa1: 0,
        aa2: 0,
        aa3: 0,
        phase_corrected: 0,
        ca: 0,
        metype: 0,
        mesub: 0,
        heading_is_valid: 0,
        heading: 0,
        aircraft_type: 0,
        fflag: 0,
        tflag: 0,
        raw_latitude: 0,
        raw_longitude: 0,
        flight: [0; 9],
        ew_dir: 0,
        ew_velocity: 0,
        ns_dir: 0,
        ns_velocity: 0,
        vert_rate_source: 0,
        vert_rate_sign: 0,
        vert_rate: 0,
        velocity: 0,
        fs: 0,
        dr: 0,
        um: 0,
        identity: 0,
        altitude: 0,
        unit: 0,
    };
    let mut tmp___0: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut tmp___1: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut high: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut low: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    hex = ((*c).buf).as_mut_ptr();
    tmp = strlen(hex as *const libc::c_char);
    l = tmp as libc::c_int;
    while l != 0 {
        tmp___0 = __ctype_b_loc();
        if *(*tmp___0)
            .offset(*hex.offset((l - 1 as libc::c_int) as isize) as libc::c_int as isize)
            as libc::c_int & 8192 as libc::c_int == 0
        {
            break;
        }
        *hex.offset((l - 1 as libc::c_int) as isize) = '\u{0}' as i32 as libc::c_char;
        l -= 1;
    }
    loop {
        tmp___1 = __ctype_b_loc();
        if *(*tmp___1).offset(*hex as libc::c_int as isize) as libc::c_int
            & 8192 as libc::c_int == 0
        {
            break;
        }
        hex = hex.offset(1);
        l -= 1;
    }
    if l < 2 as libc::c_int {
        return 0 as libc::c_int
    } else {
        if *hex.offset(0 as libc::c_int as isize) as libc::c_int != 42 as libc::c_int {
            return 0 as libc::c_int
        } else {
            if *hex.offset((l - 1 as libc::c_int) as isize) as libc::c_int
                != 59 as libc::c_int
            {
                return 0 as libc::c_int;
            }
        }
    }
    hex = hex.offset(1);
    l -= 2 as libc::c_int;
    if l > 28 as libc::c_int {
        return 0 as libc::c_int;
    }
    j = 0 as libc::c_int;
    while j < l {
        tmp___2 = hexDigitVal(*hex.offset(j as isize) as libc::c_int);
        high = tmp___2;
        tmp___3 = hexDigitVal(
            *hex.offset((j + 1 as libc::c_int) as isize) as libc::c_int,
        );
        low = tmp___3;
        if high == -(1 as libc::c_int) {
            return 0 as libc::c_int
        } else {
            if low == -(1 as libc::c_int) {
                return 0 as libc::c_int;
            }
        }
        msg[(j / 2 as libc::c_int)
            as usize] = (high << 4 as libc::c_int | low) as libc::c_uchar;
        j += 2 as libc::c_int;
    }
    decodeModesMessage(&mut mm, msg.as_mut_ptr());
    useModesMessage(&mut mm);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn aircraftsToJson(
    mut len: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut a: *mut aircraft = 0 as *mut aircraft;
    let mut buflen: libc::c_int = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    let mut altitude: libc::c_int = 0;
    let mut speed: libc::c_int = 0;
    let mut used: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    a = Modes.aircrafts;
    buflen = 1024 as libc::c_int;
    tmp = malloc(buflen as size_t);
    buf = tmp as *mut libc::c_char;
    p = buf;
    l = snprintf(p, buflen as size_t, b"[\n\0" as *const u8 as *const libc::c_char);
    p = p.offset(l as isize);
    buflen -= l;
    while !a.is_null() {
        altitude = (*a).altitude;
        speed = (*a).speed;
        if Modes.metric != 0 {
            altitude = (altitude as libc::c_double / 3.2828f64) as libc::c_int;
            speed = (speed as libc::c_double * 1.852f64) as libc::c_int;
        }
        if (*a).lat != 0 as libc::c_int as libc::c_double {
            if (*a).lon != 0 as libc::c_int as libc::c_double {
                l = snprintf(
                    p,
                    buflen as size_t,
                    b"{\"hex\":\"%s\", \"flight\":\"%s\", \"lat\":%f, \"lon\":%f, \"altitude\":%d, \"track\":%d, \"speed\":%d},\n\0"
                        as *const u8 as *const libc::c_char,
                    ((*a).hexaddr).as_mut_ptr(),
                    ((*a).flight).as_mut_ptr(),
                    (*a).lat,
                    (*a).lon,
                    (*a).altitude,
                    (*a).track,
                    (*a).speed,
                );
                p = p.offset(l as isize);
                buflen -= l;
                if buflen < 256 as libc::c_int {
                    used = p.offset_from(buf) as libc::c_long as libc::c_int;
                    buflen += 1024 as libc::c_int;
                    tmp___0 = realloc(
                        buf as *mut libc::c_void,
                        (used + buflen) as size_t,
                    );
                    buf = tmp___0 as *mut libc::c_char;
                    p = buf.offset(used as isize);
                }
            }
        }
        a = (*a).next;
    }
    if *p.offset(-(2 as libc::c_int as isize)) as libc::c_int == 44 as libc::c_int {
        *p.offset(-(2 as libc::c_int as isize)) = '\n' as i32 as libc::c_char;
        p = p.offset(-1);
        buflen += 1;
    }
    l = snprintf(p, buflen as size_t, b"]\n\0" as *const u8 as *const libc::c_char);
    p = p.offset(l as isize);
    buflen -= l;
    *len = p.offset_from(buf) as libc::c_long as libc::c_int;
    return buf;
}
pub unsafe extern "C" fn handleHTTPRequest(mut c: *mut client) -> libc::c_int {
    let mut hdr: [libc::c_char; 512] = [0; 512];
    let mut clen: libc::c_int = 0;
    let mut hdrlen: libc::c_int = 0;
    let mut httpver: libc::c_int = 0;
    let mut keepalive: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut url: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut content: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ctype: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut fd: libc::c_int = 0;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___4: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: ssize_t = 0;
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut tmp___7: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___8: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___11: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___12: ssize_t = 0;
    let mut tmp___13: ssize_t = 0;
    if Modes.debug & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        printf(
            b"\nHTTP request: %s\n\0" as *const u8 as *const libc::c_char,
            ((*c).buf).as_mut_ptr(),
        );
    }
    tmp___0 = strstr(
        ((*c).buf).as_mut_ptr() as *const libc::c_char,
        b"HTTP/1.1\0" as *const u8 as *const libc::c_char,
    );
    if tmp___0 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        httpver = 11 as libc::c_int;
    } else {
        httpver = 10 as libc::c_int;
    }
    if httpver == 10 as libc::c_int {
        tmp___1 = strstr(
            ((*c).buf).as_mut_ptr() as *const libc::c_char,
            b"Connection: keep-alive\0" as *const u8 as *const libc::c_char,
        );
        keepalive = (tmp___1 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong)
            as libc::c_int;
    } else if httpver == 11 as libc::c_int {
        tmp___2 = strstr(
            ((*c).buf).as_mut_ptr() as *const libc::c_char,
            b"Connection: close\0" as *const u8 as *const libc::c_char,
        );
        keepalive = (tmp___2 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong)
            as libc::c_int;
    }
    p = strchr(((*c).buf).as_mut_ptr() as *const libc::c_char, ' ' as i32);
    if p.is_null() {
        return 1 as libc::c_int;
    }
    p = p.offset(1);
    url = p;
    p = strchr(p as *const libc::c_char, ' ' as i32);
    if p.is_null() {
        return 1 as libc::c_int;
    }
    *p = '\u{0}' as i32 as libc::c_char;
    if Modes.debug & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        printf(
            b"\nHTTP keep alive: %d\n\0" as *const u8 as *const libc::c_char,
            keepalive,
        );
        printf(b"HTTP requested URL: %s\n\n\0" as *const u8 as *const libc::c_char, url);
    }
    tmp___10 = strstr(
        url as *const libc::c_char,
        b"/data.json\0" as *const u8 as *const libc::c_char,
    );
    if !tmp___10.is_null() {
        content = aircraftsToJson(&mut clen);
        ctype = b"application/json;charset=utf-8\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    } else {
        fd = -(1 as libc::c_int);
        tmp___9 = stat(
            b"gmap.html\0" as *const u8 as *const libc::c_char,
            &mut sbuf as *mut stat,
        );
        if tmp___9 != -(1 as libc::c_int) {
            fd = open(
                b"gmap.html\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
            if fd != -(1 as libc::c_int) {
                tmp___3 = malloc(sbuf.st_size as size_t);
                content = tmp___3 as *mut libc::c_char;
                tmp___6 = read(fd, content as *mut libc::c_void, sbuf.st_size as size_t);
                if tmp___6 == -(1 as libc::c_long) {
                    tmp___4 = __errno_location();
                    tmp___5 = strerror(*tmp___4);
                    snprintf(
                        content,
                        sbuf.st_size as size_t,
                        b"Error reading from file: %s\0" as *const u8
                            as *const libc::c_char,
                        tmp___5,
                    );
                }
                clen = sbuf.st_size as libc::c_int;
            } else {
                tmp___7 = __errno_location();
                tmp___8 = strerror(*tmp___7);
                clen = snprintf(
                    buf.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                    b"Error opening HTML file: %s\0" as *const u8 as *const libc::c_char,
                    tmp___8,
                );
                content = strdup(buf.as_mut_ptr() as *const libc::c_char);
            }
        } else {
            tmp___7 = __errno_location();
            tmp___8 = strerror(*tmp___7);
            clen = snprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                b"Error opening HTML file: %s\0" as *const u8 as *const libc::c_char,
                tmp___8,
            );
            content = strdup(buf.as_mut_ptr() as *const libc::c_char);
        }
        if fd != -(1 as libc::c_int) {
            close(fd);
        }
        ctype = b"text/html;charset=utf-8\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    if keepalive != 0 {
        tmp___11 = b"keep-alive\0" as *const u8 as *const libc::c_char;
    } else {
        tmp___11 = b"close\0" as *const u8 as *const libc::c_char;
    }
    hdrlen = snprintf(
        hdr.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
        b"HTTP/1.1 200 OK\r\nServer: Dump1090\r\nContent-Type: %s\r\nConnection: %s\r\nContent-Length: %d\r\nAccess-Control-Allow-Origin: *\r\n\r\n\0"
            as *const u8 as *const libc::c_char,
        ctype,
        tmp___11,
        clen,
    );
    if Modes.debug & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        printf(
            b"HTTP Reply header:\n%s\0" as *const u8 as *const libc::c_char,
            hdr.as_mut_ptr(),
        );
    }
    tmp___12 = write((*c).fd, hdr.as_mut_ptr() as *const libc::c_void, hdrlen as size_t);
    if tmp___12 != hdrlen as ssize_t {
        free(content as *mut libc::c_void);
        return 1 as libc::c_int;
    } else {
        tmp___13 = write((*c).fd, content as *const libc::c_void, clen as size_t);
        if tmp___13 != clen as ssize_t {
            free(content as *mut libc::c_void);
            return 1 as libc::c_int;
        }
    }
    free(content as *mut libc::c_void);
    Modes.stat_http_requests += 1;
    return (keepalive == 0) as libc::c_int;
}
pub unsafe extern "C" fn modesReadFromClient(
    mut c: *mut client,
    mut sep: *mut libc::c_char,
    mut handler: Option::<unsafe extern "C" fn(*mut client) -> libc::c_int>,
) {
    let mut left: libc::c_int = 0;
    let mut nread: libc::c_int = 0;
    let mut tmp: ssize_t = 0;
    let mut fullmsg: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: size_t = 0;
    loop {
        left = 1024 as libc::c_int - (*c).buflen;
        tmp = read(
            (*c).fd,
            ((*c).buf).as_mut_ptr().offset((*c).buflen as isize) as *mut libc::c_void,
            left as size_t,
        );
        nread = tmp as libc::c_int;
        fullmsg = 0 as libc::c_int;
        if nread <= 0 as libc::c_int {
            if nread == 0 as libc::c_int {
                modesFreeClient((*c).fd);
            } else {
                tmp___0 = __errno_location();
                if *tmp___0 != 11 as libc::c_int {
                    modesFreeClient((*c).fd);
                }
            }
            break;
        } else {
            (*c).buflen += nread;
            (*c).buf[(*c).buflen as usize] = '\u{0}' as i32 as libc::c_char;
            loop {
                p = strstr(
                    ((*c).buf).as_mut_ptr() as *const libc::c_char,
                    sep as *const libc::c_char,
                );
                if !(p as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
                    break;
                }
                i = p.offset_from(((*c).buf).as_mut_ptr()) as libc::c_long
                    as libc::c_int;
                (*c).buf[i as usize] = '\u{0}' as i32 as libc::c_char;
                tmp___1 = (Some(handler.expect("non-null function pointer")))
                    .expect("non-null function pointer")(c);
                if tmp___1 != 0 {
                    modesFreeClient((*c).fd);
                    return;
                }
                tmp___2 = strlen(sep as *const libc::c_char);
                i = (i as size_t).wrapping_add(tmp___2) as libc::c_int;
                memmove(
                    ((*c).buf).as_mut_ptr() as *mut libc::c_void,
                    ((*c).buf).as_mut_ptr().offset(i as isize) as *const libc::c_void,
                    ((*c).buflen - i) as size_t,
                );
                (*c).buflen -= i;
                (*c).buf[(*c).buflen as usize] = '\u{0}' as i32 as libc::c_char;
                fullmsg = 1 as libc::c_int;
            }
            if (*c).buflen == 1024 as libc::c_int {
                (*c).buflen = 0 as libc::c_int;
            } else if fullmsg == 0 {
                break;
            }
        }
    };
}
pub unsafe extern "C" fn modesReadFromClients() {
    let mut j: libc::c_int = 0;
    let mut c: *mut client = 0 as *mut client;
    j = 0 as libc::c_int;
    while j <= Modes.maxfd {
        c = Modes.clients[j as usize];
        if !(c as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
            if (*c).service == Modes.ris {
                modesReadFromClient(
                    c,
                    b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    Some(
                        decodeHexMessage
                            as unsafe extern "C" fn(*mut client) -> libc::c_int,
                    ),
                );
            } else if (*c).service == Modes.https {
                modesReadFromClient(
                    c,
                    b"\r\n\r\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    Some(
                        handleHTTPRequest
                            as unsafe extern "C" fn(*mut client) -> libc::c_int,
                    ),
                );
            }
        }
        j += 1;
    }
}
pub unsafe extern "C" fn modesWaitReadableClients(mut timeout_ms: libc::c_int) {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut fds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut j: libc::c_int = 0;
    let mut maxfd: libc::c_int = 0;
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    maxfd = Modes.maxfd;
    let fresh0 = &mut __d0;
    let fresh1;
    let fresh2 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh3 = &mut __d1;
    let fresh4;
    let fresh5 = &mut *(fds.__fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh0,
        fresh2) => fresh1, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh3,
        fresh5) => fresh4, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
    c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
    j = 0 as libc::c_int;
    while j <= Modes.maxfd {
        if !(Modes.clients[j as usize]).is_null() {
            fds
                .__fds_bits[(j
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                |= ((1 as libc::c_ulong)
                    << j
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask;
        }
        j += 1;
    }
    j = 0 as libc::c_int;
    while j < 4 as libc::c_int {
        s = *modesNetServices[j as usize].socket;
        fds
            .__fds_bits[(s
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << s
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        if s > maxfd {
            maxfd = s;
        }
        j += 1;
    }
    tv.tv_sec = (timeout_ms / 1000 as libc::c_int) as __time_t;
    tv
        .tv_usec = (timeout_ms % 1000 as libc::c_int * 1000 as libc::c_int)
        as __suseconds_t;
    select(
        maxfd + 1 as libc::c_int,
        &mut fds as *mut fd_set,
        0 as *mut libc::c_void as *mut fd_set,
        0 as *mut libc::c_void as *mut fd_set,
        &mut tv as *mut timeval,
    );
}
pub unsafe extern "C" fn sigWinchCallback() {
    signal(
        28 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(1 as libc::c_int as libc::intptr_t),
    );
    Modes.interactive_rows = getTermRows();
    interactiveShowData();
    signal(
        28 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(Some(sigWinchCallback as unsafe extern "C" fn() -> ())),
    );
}
pub unsafe extern "C" fn getTermRows() -> libc::c_int {
    let mut w: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    ioctl(1 as libc::c_int, 21523 as libc::c_ulong, &mut w as *mut winsize);
    return w.ws_row as libc::c_int;
}
pub unsafe extern "C" fn showHelp() {
    printf(
        b"--device-index <index>   Select RTL device (default: 0).\n--gain <db>              Set gain (default: max gain. Use -100 for auto-gain).\n--enable-agc             Enable the Automatic Gain Control (default: off).\n--freq <hz>              Set frequency (default: 1090 Mhz).\n--ifile <filename>       Read data from file (use '-' for stdin).\n--loop                   With --ifile, read the same file in a loop.\n--interactive            Interactive mode refreshing data on screen.\n--interactive-rows <num> Max number of rows in interactive mode (default: 15).\n--interactive-ttl <sec>  Remove from list if idle for <sec> (default: 60).\n--raw                    Show only messages hex values.\n--net                    Enable networking.\n--net-only               Enable just networking, no RTL device or file used.\n--net-ro-port <port>     TCP listening port for raw output (default: 30002).\n--net-ri-port <port>     TCP listening port for raw input (default: 30001).\n--net-http-port <port>   HTTP server port (default: 8080).\n--net-sbs-port <port>    TCP listening port for BaseStation format output (default: 30003).\n--no-fix                 Disable single-bits error correction using CRC.\n--no-crc-check           Disable messages with broken CRC (discouraged).\n--aggressive             More CPU for more messages (two bits fixes, ...).\n--stats                  With --ifile print stats at exit. No other output.\n--onlyaddr               Show only ICAO addresses (testing purposes).\n--metric                 Use metric units (meters, km/h, ...).\n--snip <level>           Strip IQ file removing samples < level.\n--debug <flags>          Debug mode (verbose), see README for details.\n--help                   Show this help.\n\nDebug mode flags: d = Log frames decoded with errors\n                  D = Log frames decoded with zero errors\n                  c = Log frames with bad CRC\n                  C = Log frames with good CRC\n                  p = Log frames with bad preamble\n                  n = Log network debugging info\n                  j = Log frames to frames.js, loadable by debug.html.\n\0"
            as *const u8 as *const libc::c_char,
    );
}
pub unsafe extern "C" fn backgroundTasks() {
    let mut tmp: libc::c_longlong = 0;
    if Modes.net != 0 {
        modesAcceptClients();
        modesReadFromClients();
        interactiveRemoveStaleAircrafts();
    }
    if Modes.interactive != 0 {
        tmp = mstime();
        if tmp - Modes.interactive_last_update > 250 as libc::c_longlong {
            interactiveRemoveStaleAircrafts();
            interactiveShowData();
            Modes.interactive_last_update = mstime();
        }
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut more: libc::c_int = 0;
    let mut tmp: libc::c_double = 0.;
    let mut tmp___0: libc::c_longlong = 0;
    let mut f: *mut libc::c_char = 0 as *mut libc::c_char;
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
    let mut tmp___15: libc::c_int = 0;
    let mut tmp___16: libc::c_int = 0;
    let mut tmp___17: libc::c_int = 0;
    let mut tmp___18: libc::c_int = 0;
    let mut tmp___19: libc::c_int = 0;
    let mut tmp___20: libc::c_int = 0;
    let mut tmp___21: libc::c_int = 0;
    let mut tmp___22: libc::c_int = 0;
    let mut tmp___23: libc::c_int = 0;
    let mut tmp___24: libc::c_int = 0;
    let mut tmp___25: libc::c_int = 0;
    let mut tmp___26: libc::c_int = 0;
    modesInitConfig();
    j = 1 as libc::c_int;
    while j < argc {
        more = ((j + 1 as libc::c_int) < argc) as libc::c_int;
        tmp___26 = strcmp(
            *argv.offset(j as isize) as *const libc::c_char,
            b"--device-index\0" as *const u8 as *const libc::c_char,
        );
        let mut current_block_150: u64;
        if tmp___26 != 0 {
            current_block_150 = 11900449061113753573;
        } else if more != 0 {
            j += 1;
            Modes.dev_index = atoi(*argv.offset(j as isize) as *const libc::c_char);
            current_block_150 = 13331918191865804447;
        } else {
            current_block_150 = 11900449061113753573;
        }
        match current_block_150 {
            11900449061113753573 => {
                tmp___25 = strcmp(
                    *argv.offset(j as isize) as *const libc::c_char,
                    b"--gain\0" as *const u8 as *const libc::c_char,
                );
                let mut current_block_146: u64;
                if tmp___25 != 0 {
                    current_block_146 = 5816247937169455600;
                } else if more != 0 {
                    j += 1;
                    tmp = atof(*argv.offset(j as isize) as *const libc::c_char);
                    Modes
                        .gain = (tmp * 10 as libc::c_int as libc::c_double)
                        as libc::c_int;
                    current_block_146 = 7468767852762055642;
                } else {
                    current_block_146 = 5816247937169455600;
                }
                match current_block_146 {
                    5816247937169455600 => {
                        tmp___24 = strcmp(
                            *argv.offset(j as isize) as *const libc::c_char,
                            b"--enable-agc\0" as *const u8 as *const libc::c_char,
                        );
                        if tmp___24 != 0 {
                            tmp___23 = strcmp(
                                *argv.offset(j as isize) as *const libc::c_char,
                                b"--freq\0" as *const u8 as *const libc::c_char,
                            );
                            let mut current_block_137: u64;
                            if tmp___23 != 0 {
                                current_block_137 = 3613265918928737804;
                            } else if more != 0 {
                                j += 1;
                                tmp___0 = strtoll(
                                    *argv.offset(j as isize) as *const libc::c_char,
                                    0 as *mut libc::c_void as *mut *mut libc::c_char,
                                    10 as libc::c_int,
                                );
                                Modes.freq = tmp___0 as libc::c_int;
                                current_block_137 = 12099607619007264150;
                            } else {
                                current_block_137 = 3613265918928737804;
                            }
                            match current_block_137 {
                                3613265918928737804 => {
                                    tmp___22 = strcmp(
                                        *argv.offset(j as isize) as *const libc::c_char,
                                        b"--ifile\0" as *const u8 as *const libc::c_char,
                                    );
                                    let mut current_block_132: u64;
                                    if tmp___22 != 0 {
                                        current_block_132 = 8122577082334716293;
                                    } else if more != 0 {
                                        j += 1;
                                        Modes
                                            .filename = strdup(
                                            *argv.offset(j as isize) as *const libc::c_char,
                                        );
                                        current_block_132 = 5089124893069931607;
                                    } else {
                                        current_block_132 = 8122577082334716293;
                                    }
                                    match current_block_132 {
                                        8122577082334716293 => {
                                            tmp___21 = strcmp(
                                                *argv.offset(j as isize) as *const libc::c_char,
                                                b"--loop\0" as *const u8 as *const libc::c_char,
                                            );
                                            if tmp___21 != 0 {
                                                tmp___20 = strcmp(
                                                    *argv.offset(j as isize) as *const libc::c_char,
                                                    b"--no-fix\0" as *const u8 as *const libc::c_char,
                                                );
                                                if tmp___20 != 0 {
                                                    tmp___19 = strcmp(
                                                        *argv.offset(j as isize) as *const libc::c_char,
                                                        b"--no-crc-check\0" as *const u8 as *const libc::c_char,
                                                    );
                                                    if tmp___19 != 0 {
                                                        tmp___18 = strcmp(
                                                            *argv.offset(j as isize) as *const libc::c_char,
                                                            b"--raw\0" as *const u8 as *const libc::c_char,
                                                        );
                                                        if tmp___18 != 0 {
                                                            tmp___17 = strcmp(
                                                                *argv.offset(j as isize) as *const libc::c_char,
                                                                b"--net\0" as *const u8 as *const libc::c_char,
                                                            );
                                                            if tmp___17 != 0 {
                                                                tmp___16 = strcmp(
                                                                    *argv.offset(j as isize) as *const libc::c_char,
                                                                    b"--net-only\0" as *const u8 as *const libc::c_char,
                                                                );
                                                                if tmp___16 != 0 {
                                                                    tmp___15 = strcmp(
                                                                        *argv.offset(j as isize) as *const libc::c_char,
                                                                        b"--net-ro-port\0" as *const u8 as *const libc::c_char,
                                                                    );
                                                                    let mut current_block_103: u64;
                                                                    if tmp___15 != 0 {
                                                                        current_block_103 = 6183997530727287201;
                                                                    } else if more != 0 {
                                                                        j += 1;
                                                                        modesNetServices[0 as libc::c_int as usize]
                                                                            .port = atoi(
                                                                            *argv.offset(j as isize) as *const libc::c_char,
                                                                        );
                                                                        current_block_103 = 16667286137552459707;
                                                                    } else {
                                                                        current_block_103 = 6183997530727287201;
                                                                    }
                                                                    match current_block_103 {
                                                                        6183997530727287201 => {
                                                                            tmp___14 = strcmp(
                                                                                *argv.offset(j as isize) as *const libc::c_char,
                                                                                b"--net-ri-port\0" as *const u8 as *const libc::c_char,
                                                                            );
                                                                            let mut current_block_99: u64;
                                                                            if tmp___14 != 0 {
                                                                                current_block_99 = 13623356663507771538;
                                                                            } else if more != 0 {
                                                                                j += 1;
                                                                                modesNetServices[1 as libc::c_int as usize]
                                                                                    .port = atoi(
                                                                                    *argv.offset(j as isize) as *const libc::c_char,
                                                                                );
                                                                                current_block_99 = 6665878751423064961;
                                                                            } else {
                                                                                current_block_99 = 13623356663507771538;
                                                                            }
                                                                            match current_block_99 {
                                                                                13623356663507771538 => {
                                                                                    tmp___13 = strcmp(
                                                                                        *argv.offset(j as isize) as *const libc::c_char,
                                                                                        b"--net-http-port\0" as *const u8 as *const libc::c_char,
                                                                                    );
                                                                                    let mut current_block_95: u64;
                                                                                    if tmp___13 != 0 {
                                                                                        current_block_95 = 7598673213363521085;
                                                                                    } else if more != 0 {
                                                                                        j += 1;
                                                                                        modesNetServices[2 as libc::c_int as usize]
                                                                                            .port = atoi(
                                                                                            *argv.offset(j as isize) as *const libc::c_char,
                                                                                        );
                                                                                        current_block_95 = 11674240781755647963;
                                                                                    } else {
                                                                                        current_block_95 = 7598673213363521085;
                                                                                    }
                                                                                    match current_block_95 {
                                                                                        7598673213363521085 => {
                                                                                            tmp___12 = strcmp(
                                                                                                *argv.offset(j as isize) as *const libc::c_char,
                                                                                                b"--net-sbs-port\0" as *const u8 as *const libc::c_char,
                                                                                            );
                                                                                            let mut current_block_91: u64;
                                                                                            if tmp___12 != 0 {
                                                                                                current_block_91 = 1558836074353746124;
                                                                                            } else if more != 0 {
                                                                                                j += 1;
                                                                                                modesNetServices[3 as libc::c_int as usize]
                                                                                                    .port = atoi(
                                                                                                    *argv.offset(j as isize) as *const libc::c_char,
                                                                                                );
                                                                                                current_block_91 = 15586796709793571329;
                                                                                            } else {
                                                                                                current_block_91 = 1558836074353746124;
                                                                                            }
                                                                                            match current_block_91 {
                                                                                                1558836074353746124 => {
                                                                                                    tmp___11 = strcmp(
                                                                                                        *argv.offset(j as isize) as *const libc::c_char,
                                                                                                        b"--onlyaddr\0" as *const u8 as *const libc::c_char,
                                                                                                    );
                                                                                                    if tmp___11 != 0 {
                                                                                                        tmp___10 = strcmp(
                                                                                                            *argv.offset(j as isize) as *const libc::c_char,
                                                                                                            b"--metric\0" as *const u8 as *const libc::c_char,
                                                                                                        );
                                                                                                        if tmp___10 != 0 {
                                                                                                            tmp___9 = strcmp(
                                                                                                                *argv.offset(j as isize) as *const libc::c_char,
                                                                                                                b"--aggressive\0" as *const u8 as *const libc::c_char,
                                                                                                            );
                                                                                                            if tmp___9 != 0 {
                                                                                                                tmp___8 = strcmp(
                                                                                                                    *argv.offset(j as isize) as *const libc::c_char,
                                                                                                                    b"--interactive\0" as *const u8 as *const libc::c_char,
                                                                                                                );
                                                                                                                if tmp___8 != 0 {
                                                                                                                    tmp___7 = strcmp(
                                                                                                                        *argv.offset(j as isize) as *const libc::c_char,
                                                                                                                        b"--interactive-rows\0" as *const u8 as *const libc::c_char,
                                                                                                                    );
                                                                                                                    if tmp___7 != 0 {
                                                                                                                        tmp___6 = strcmp(
                                                                                                                            *argv.offset(j as isize) as *const libc::c_char,
                                                                                                                            b"--interactive-ttl\0" as *const u8 as *const libc::c_char,
                                                                                                                        );
                                                                                                                        if tmp___6 != 0 {
                                                                                                                            tmp___5 = strcmp(
                                                                                                                                *argv.offset(j as isize) as *const libc::c_char,
                                                                                                                                b"--debug\0" as *const u8 as *const libc::c_char,
                                                                                                                            );
                                                                                                                            let mut current_block_61: u64;
                                                                                                                            if tmp___5 != 0 {
                                                                                                                                current_block_61 = 11244001500301384377;
                                                                                                                            } else if more != 0 {
                                                                                                                                j += 1;
                                                                                                                                f = *argv.offset(j as isize);
                                                                                                                                while *f != 0 {
                                                                                                                                    match *f as libc::c_int {
                                                                                                                                        68 => {
                                                                                                                                            Modes.debug |= 1 as libc::c_int;
                                                                                                                                        }
                                                                                                                                        100 => {
                                                                                                                                            Modes.debug |= (1 as libc::c_int) << 1 as libc::c_int;
                                                                                                                                        }
                                                                                                                                        67 => {
                                                                                                                                            Modes.debug |= (1 as libc::c_int) << 3 as libc::c_int;
                                                                                                                                        }
                                                                                                                                        99 => {
                                                                                                                                            Modes.debug |= (1 as libc::c_int) << 2 as libc::c_int;
                                                                                                                                        }
                                                                                                                                        112 => {
                                                                                                                                            Modes.debug |= (1 as libc::c_int) << 4 as libc::c_int;
                                                                                                                                        }
                                                                                                                                        110 => {
                                                                                                                                            Modes.debug |= (1 as libc::c_int) << 5 as libc::c_int;
                                                                                                                                        }
                                                                                                                                        106 => {
                                                                                                                                            Modes.debug |= (1 as libc::c_int) << 6 as libc::c_int;
                                                                                                                                        }
                                                                                                                                        _ => {
                                                                                                                                            fprintf(
                                                                                                                                                stderr,
                                                                                                                                                b"Unknown debugging flag: %c\n\0" as *const u8
                                                                                                                                                    as *const libc::c_char,
                                                                                                                                                *f as libc::c_int,
                                                                                                                                            );
                                                                                                                                            exit(1 as libc::c_int);
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    f = f.offset(1);
                                                                                                                                }
                                                                                                                                current_block_61 = 5207889489643863322;
                                                                                                                            } else {
                                                                                                                                current_block_61 = 11244001500301384377;
                                                                                                                            }
                                                                                                                            match current_block_61 {
                                                                                                                                11244001500301384377 => {
                                                                                                                                    tmp___4 = strcmp(
                                                                                                                                        *argv.offset(j as isize) as *const libc::c_char,
                                                                                                                                        b"--stats\0" as *const u8 as *const libc::c_char,
                                                                                                                                    );
                                                                                                                                    if tmp___4 != 0 {
                                                                                                                                        tmp___3 = strcmp(
                                                                                                                                            *argv.offset(j as isize) as *const libc::c_char,
                                                                                                                                            b"--snip\0" as *const u8 as *const libc::c_char,
                                                                                                                                        );
                                                                                                                                        if !(tmp___3 != 0) {
                                                                                                                                            if more != 0 {
                                                                                                                                                j += 1;
                                                                                                                                                tmp___1 = atoi(
                                                                                                                                                    *argv.offset(j as isize) as *const libc::c_char,
                                                                                                                                                );
                                                                                                                                                snipMode(tmp___1);
                                                                                                                                                exit(0 as libc::c_int);
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                        tmp___2 = strcmp(
                                                                                                                                            *argv.offset(j as isize) as *const libc::c_char,
                                                                                                                                            b"--help\0" as *const u8 as *const libc::c_char,
                                                                                                                                        );
                                                                                                                                        if tmp___2 != 0 {
                                                                                                                                            fprintf(
                                                                                                                                                stderr,
                                                                                                                                                b"Unknown or not enough arguments for option '%s'.\n\n\0"
                                                                                                                                                    as *const u8 as *const libc::c_char,
                                                                                                                                                *argv.offset(j as isize),
                                                                                                                                            );
                                                                                                                                            showHelp();
                                                                                                                                            exit(1 as libc::c_int);
                                                                                                                                        } else {
                                                                                                                                            showHelp();
                                                                                                                                            exit(0 as libc::c_int);
                                                                                                                                        }
                                                                                                                                    } else {
                                                                                                                                        Modes.stats = 1 as libc::c_int;
                                                                                                                                    }
                                                                                                                                }
                                                                                                                                _ => {}
                                                                                                                            }
                                                                                                                        } else {
                                                                                                                            j += 1;
                                                                                                                            Modes
                                                                                                                                .interactive_ttl = atoi(
                                                                                                                                *argv.offset(j as isize) as *const libc::c_char,
                                                                                                                            );
                                                                                                                        }
                                                                                                                    } else {
                                                                                                                        j += 1;
                                                                                                                        Modes
                                                                                                                            .interactive_rows = atoi(
                                                                                                                            *argv.offset(j as isize) as *const libc::c_char,
                                                                                                                        );
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    Modes.interactive = 1 as libc::c_int;
                                                                                                                }
                                                                                                            } else {
                                                                                                                Modes.aggressive += 1;
                                                                                                            }
                                                                                                        } else {
                                                                                                            Modes.metric = 1 as libc::c_int;
                                                                                                        }
                                                                                                    } else {
                                                                                                        Modes.onlyaddr = 1 as libc::c_int;
                                                                                                    }
                                                                                                }
                                                                                                _ => {}
                                                                                            }
                                                                                        }
                                                                                        _ => {}
                                                                                    }
                                                                                }
                                                                                _ => {}
                                                                            }
                                                                        }
                                                                        _ => {}
                                                                    }
                                                                } else {
                                                                    Modes.net = 1 as libc::c_int;
                                                                    Modes.net_only = 1 as libc::c_int;
                                                                }
                                                            } else {
                                                                Modes.net = 1 as libc::c_int;
                                                            }
                                                        } else {
                                                            Modes.raw = 1 as libc::c_int;
                                                        }
                                                    } else {
                                                        Modes.check_crc = 0 as libc::c_int;
                                                    }
                                                } else {
                                                    Modes.fix_errors = 0 as libc::c_int;
                                                }
                                            } else {
                                                Modes.loop_0 = 1 as libc::c_int;
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        } else {
                            Modes.enable_agc += 1;
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        j += 1;
    }
    if Modes.interactive == 1 as libc::c_int {
        signal(
            28 as libc::c_int,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
            >(Some(sigWinchCallback as unsafe extern "C" fn() -> ())),
        );
    }
    modesInit();
    if Modes.net_only != 0 {
        fprintf(
            stderr,
            b"Net-only mode, no RTL device or file open.\n\0" as *const u8
                as *const libc::c_char,
        );
    } else if Modes.filename as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong
        {
        modesInitRTLSDR();
    } else {
        let mut current_block_169: u64;
        if *(Modes.filename).offset(0 as libc::c_int as isize) as libc::c_int
            == 45 as libc::c_int
        {
            if *(Modes.filename).offset(1 as libc::c_int as isize) as libc::c_int
                == 0 as libc::c_int
            {
                Modes.fd = 0 as libc::c_int;
                current_block_169 = 14698008245370361992;
            } else {
                current_block_169 = 10313899219970016208;
            }
        } else {
            current_block_169 = 10313899219970016208;
        }
        match current_block_169 {
            10313899219970016208 => {
                Modes.fd = open(Modes.filename as *const libc::c_char, 0 as libc::c_int);
                if Modes.fd == -(1 as libc::c_int) {
                    perror(b"Opening data file\0" as *const u8 as *const libc::c_char);
                    exit(1 as libc::c_int);
                }
            }
            _ => {}
        }
    }
    if Modes.net != 0 {
        modesInitNet();
    }
    while Modes.net_only != 0 {
        backgroundTasks();
        modesWaitReadableClients(100 as libc::c_int);
    }
    pthread_create(
        &mut Modes.reader_thread as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(
            readerThreadEntryPoint
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        0 as *mut libc::c_void,
    );
    pthread_mutex_lock(&mut Modes.data_mutex);
    loop {
        if Modes.data_ready == 0 {
            pthread_cond_wait(
                &mut Modes.data_cond as *mut pthread_cond_t,
                &mut Modes.data_mutex as *mut pthread_mutex_t,
            );
        } else {
            computeMagnitudeVector();
            Modes.data_ready = 0 as libc::c_int;
            pthread_cond_signal(&mut Modes.data_cond);
            pthread_mutex_unlock(&mut Modes.data_mutex);
            detectModeS(
                Modes.magnitude,
                (Modes.data_len).wrapping_div(2 as libc::c_uint),
            );
            backgroundTasks();
            pthread_mutex_lock(&mut Modes.data_mutex);
            if Modes.exit != 0 {
                break;
            }
        }
    }
    if Modes.stats != 0 {
        if !(Modes.filename).is_null() {
            printf(
                b"%lld valid preambles\n\0" as *const u8 as *const libc::c_char,
                Modes.stat_valid_preamble,
            );
            printf(
                b"%lld demodulated again after phase correction\n\0" as *const u8
                    as *const libc::c_char,
                Modes.stat_out_of_phase,
            );
            printf(
                b"%lld demodulated with zero errors\n\0" as *const u8
                    as *const libc::c_char,
                Modes.stat_demodulated,
            );
            printf(
                b"%lld with good crc\n\0" as *const u8 as *const libc::c_char,
                Modes.stat_goodcrc,
            );
            printf(
                b"%lld with bad crc\n\0" as *const u8 as *const libc::c_char,
                Modes.stat_badcrc,
            );
            printf(
                b"%lld errors corrected\n\0" as *const u8 as *const libc::c_char,
                Modes.stat_fixed,
            );
            printf(
                b"%lld single bit errors\n\0" as *const u8 as *const libc::c_char,
                Modes.stat_single_bit_fix,
            );
            printf(
                b"%lld two bits errors\n\0" as *const u8 as *const libc::c_char,
                Modes.stat_two_bits_fix,
            );
            printf(
                b"%lld total usable messages\n\0" as *const u8 as *const libc::c_char,
                Modes.stat_goodcrc + Modes.stat_fixed,
            );
        }
    }
    rtlsdr_close(Modes.dev);
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 255 as libc::c_int
        | (__bsx as libc::c_int & 255 as libc::c_int) << 8 as libc::c_int) as __uint16_t;
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 4278190080 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 16711680 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 65280 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 255 as libc::c_uint) << 24 as libc::c_int;
}
unsafe extern "C" fn anetSetError(
    mut err: *mut libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    if err.is_null() {
        return;
    }
    ap = args.clone();
    vsnprintf(err, 256 as libc::c_int as size_t, fmt, ap.as_va_list());
}
pub unsafe extern "C" fn anetNonBlock(
    mut err: *mut libc::c_char,
    mut fd: libc::c_int,
) -> libc::c_int {
    let mut flags: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: libc::c_int = 0;
    flags = fcntl(fd, 3 as libc::c_int);
    if flags == -(1 as libc::c_int) {
        tmp = __errno_location();
        tmp___0 = strerror(*tmp);
        anetSetError(
            err,
            b"fcntl(F_GETFL): %s\0" as *const u8 as *const libc::c_char,
            tmp___0,
        );
        return -(1 as libc::c_int);
    }
    tmp___3 = fcntl(fd, 4 as libc::c_int, flags | 2048 as libc::c_int);
    if tmp___3 == -(1 as libc::c_int) {
        tmp___1 = __errno_location();
        tmp___2 = strerror(*tmp___1);
        anetSetError(
            err,
            b"fcntl(F_SETFL,O_NONBLOCK): %s\0" as *const u8 as *const libc::c_char,
            tmp___2,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn anetTcpNoDelay(
    mut err: *mut libc::c_char,
    mut fd: libc::c_int,
) -> libc::c_int {
    let mut yes: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    yes = 1 as libc::c_int;
    tmp___1 = setsockopt(
        fd,
        6 as libc::c_int,
        1 as libc::c_int,
        &mut yes as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    if tmp___1 == -(1 as libc::c_int) {
        tmp = __errno_location();
        tmp___0 = strerror(*tmp);
        anetSetError(
            err,
            b"setsockopt TCP_NODELAY: %s\0" as *const u8 as *const libc::c_char,
            tmp___0,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn anetSetSendBuffer(
    mut err: *mut libc::c_char,
    mut fd: libc::c_int,
    mut buffsize: libc::c_int,
) -> libc::c_int {
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    tmp___1 = setsockopt(
        fd,
        1 as libc::c_int,
        7 as libc::c_int,
        &mut buffsize as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    if tmp___1 == -(1 as libc::c_int) {
        tmp = __errno_location();
        tmp___0 = strerror(*tmp);
        anetSetError(
            err,
            b"setsockopt SO_SNDBUF: %s\0" as *const u8 as *const libc::c_char,
            tmp___0,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn anetTcpKeepAlive(
    mut err: *mut libc::c_char,
    mut fd: libc::c_int,
) -> libc::c_int {
    let mut yes: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    yes = 1 as libc::c_int;
    tmp___1 = setsockopt(
        fd,
        1 as libc::c_int,
        9 as libc::c_int,
        &mut yes as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    if tmp___1 == -(1 as libc::c_int) {
        tmp = __errno_location();
        tmp___0 = strerror(*tmp);
        anetSetError(
            err,
            b"setsockopt SO_KEEPALIVE: %s\0" as *const u8 as *const libc::c_char,
            tmp___0,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn anetResolve(
    mut err: *mut libc::c_char,
    mut host: *mut libc::c_char,
    mut ipbuf: *mut libc::c_char,
) -> libc::c_int {
    let mut sa: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut he: *mut hostent = 0 as *mut hostent;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    sa.sin_family = 2 as libc::c_int as sa_family_t;
    tmp = inet_aton(host as *const libc::c_char, &mut sa.sin_addr);
    if tmp == 0 as libc::c_int {
        he = gethostbyname(host as *const libc::c_char);
        if he as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            anetSetError(
                err,
                b"can't resolve: %s\0" as *const u8 as *const libc::c_char,
                host,
            );
            return -(1 as libc::c_int);
        }
        memcpy(
            &mut sa.sin_addr as *mut in_addr as *mut libc::c_void,
            *((*he).h_addr_list).offset(0 as libc::c_int as isize)
                as *const libc::c_void,
            ::std::mem::size_of::<in_addr>() as libc::c_ulong,
        );
    }
    tmp___0 = inet_ntoa(sa.sin_addr);
    strcpy(ipbuf, tmp___0 as *const libc::c_char);
    return 0 as libc::c_int;
}
unsafe extern "C" fn anetCreateSocket(
    mut err: *mut libc::c_char,
    mut domain: libc::c_int,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut on: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: libc::c_int = 0;
    on = 1 as libc::c_int;
    s = socket(domain, 1 as libc::c_int, 0 as libc::c_int);
    if s == -(1 as libc::c_int) {
        tmp = __errno_location();
        tmp___0 = strerror(*tmp);
        anetSetError(
            err,
            b"creating socket: %s\0" as *const u8 as *const libc::c_char,
            tmp___0,
        );
        return -(1 as libc::c_int);
    }
    tmp___3 = setsockopt(
        s,
        1 as libc::c_int,
        2 as libc::c_int,
        &mut on as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    if tmp___3 == -(1 as libc::c_int) {
        tmp___1 = __errno_location();
        tmp___2 = strerror(*tmp___1);
        anetSetError(
            err,
            b"setsockopt SO_REUSEADDR: %s\0" as *const u8 as *const libc::c_char,
            tmp___2,
        );
        return -(1 as libc::c_int);
    }
    return s;
}
unsafe extern "C" fn anetTcpGenericConnect(
    mut err: *mut libc::c_char,
    mut addr: *mut libc::c_char,
    mut port: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut sa: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut he: *mut hostent = 0 as *mut hostent;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: libc::c_int = 0;
    s = anetCreateSocket(err, 2 as libc::c_int);
    if s == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    sa.sin_family = 2 as libc::c_int as sa_family_t;
    sa.sin_port = __bswap_16(port as __uint16_t);
    tmp = inet_aton(addr as *const libc::c_char, &mut sa.sin_addr);
    if tmp == 0 as libc::c_int {
        he = gethostbyname(addr as *const libc::c_char);
        if he as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            anetSetError(
                err,
                b"can't resolve: %s\0" as *const u8 as *const libc::c_char,
                addr,
            );
            close(s);
            return -(1 as libc::c_int);
        }
        memcpy(
            &mut sa.sin_addr as *mut in_addr as *mut libc::c_void,
            *((*he).h_addr_list).offset(0 as libc::c_int as isize)
                as *const libc::c_void,
            ::std::mem::size_of::<in_addr>() as libc::c_ulong,
        );
    }
    if flags & 1 as libc::c_int != 0 {
        tmp___0 = anetNonBlock(err, s);
        if tmp___0 != 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    tmp___4 = connect(
        s,
        &mut sa as *mut sockaddr_in as *mut sockaddr as *const sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    );
    if tmp___4 == -(1 as libc::c_int) {
        tmp___1 = __errno_location();
        if *tmp___1 == 115 as libc::c_int {
            if flags & 1 as libc::c_int != 0 {
                return s;
            }
        }
        tmp___2 = __errno_location();
        tmp___3 = strerror(*tmp___2);
        anetSetError(err, b"connect: %s\0" as *const u8 as *const libc::c_char, tmp___3);
        close(s);
        return -(1 as libc::c_int);
    }
    return s;
}
pub unsafe extern "C" fn anetTcpConnect(
    mut err: *mut libc::c_char,
    mut addr: *mut libc::c_char,
    mut port: libc::c_int,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = anetTcpGenericConnect(err, addr, port, 0 as libc::c_int);
    return tmp;
}
pub unsafe extern "C" fn anetTcpNonBlockConnect(
    mut err: *mut libc::c_char,
    mut addr: *mut libc::c_char,
    mut port: libc::c_int,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = anetTcpGenericConnect(err, addr, port, 1 as libc::c_int);
    return tmp;
}
pub unsafe extern "C" fn anetUnixGenericConnect(
    mut err: *mut libc::c_char,
    mut path: *mut libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut sa: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: libc::c_int = 0;
    s = anetCreateSocket(err, 1 as libc::c_int);
    if s == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    sa.sun_family = 1 as libc::c_int as sa_family_t;
    strncpy(
        (sa.sun_path).as_mut_ptr(),
        path as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_ulong),
    );
    if flags & 1 as libc::c_int != 0 {
        tmp = anetNonBlock(err, s);
        if tmp != 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    tmp___3 = connect(
        s,
        &mut sa as *mut sockaddr_un as *mut sockaddr as *const sockaddr,
        ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong as socklen_t,
    );
    if tmp___3 == -(1 as libc::c_int) {
        tmp___0 = __errno_location();
        if *tmp___0 == 115 as libc::c_int {
            if flags & 1 as libc::c_int != 0 {
                return s;
            }
        }
        tmp___1 = __errno_location();
        tmp___2 = strerror(*tmp___1);
        anetSetError(err, b"connect: %s\0" as *const u8 as *const libc::c_char, tmp___2);
        close(s);
        return -(1 as libc::c_int);
    }
    return s;
}
pub unsafe extern "C" fn anetUnixConnect(
    mut err: *mut libc::c_char,
    mut path: *mut libc::c_char,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = anetUnixGenericConnect(err, path, 0 as libc::c_int);
    return tmp;
}
pub unsafe extern "C" fn anetUnixNonBlockConnect(
    mut err: *mut libc::c_char,
    mut path: *mut libc::c_char,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = anetUnixGenericConnect(err, path, 1 as libc::c_int);
    return tmp;
}
pub unsafe extern "C" fn anetRead(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_char,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut nread: libc::c_int = 0;
    let mut totlen: libc::c_int = 0;
    let mut tmp: ssize_t = 0;
    totlen = 0 as libc::c_int;
    while totlen != count {
        tmp = read(fd, buf as *mut libc::c_void, (count - totlen) as size_t);
        nread = tmp as libc::c_int;
        if nread == 0 as libc::c_int {
            return totlen;
        }
        if nread == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        totlen += nread;
        buf = buf.offset(nread as isize);
    }
    return totlen;
}
pub unsafe extern "C" fn anetWrite(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_char,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut nwritten: libc::c_int = 0;
    let mut totlen: libc::c_int = 0;
    let mut tmp: ssize_t = 0;
    totlen = 0 as libc::c_int;
    while totlen != count {
        tmp = write(fd, buf as *const libc::c_void, (count - totlen) as size_t);
        nwritten = tmp as libc::c_int;
        if nwritten == 0 as libc::c_int {
            return totlen;
        }
        if nwritten == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        totlen += nwritten;
        buf = buf.offset(nwritten as isize);
    }
    return totlen;
}
unsafe extern "C" fn anetListen(
    mut err: *mut libc::c_char,
    mut s: libc::c_int,
    mut sa: *mut sockaddr,
    mut len: socklen_t,
) -> libc::c_int {
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: libc::c_int = 0;
    tmp___1 = bind(s, sa as *const sockaddr, len);
    if tmp___1 == -(1 as libc::c_int) {
        tmp = __errno_location();
        tmp___0 = strerror(*tmp);
        anetSetError(err, b"bind: %s\0" as *const u8 as *const libc::c_char, tmp___0);
        close(s);
        return -(1 as libc::c_int);
    }
    tmp___4 = listen(s, 511 as libc::c_int);
    if tmp___4 == -(1 as libc::c_int) {
        tmp___2 = __errno_location();
        tmp___3 = strerror(*tmp___2);
        anetSetError(err, b"listen: %s\0" as *const u8 as *const libc::c_char, tmp___3);
        close(s);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn anetTcpServer(
    mut err: *mut libc::c_char,
    mut port: libc::c_int,
    mut bindaddr: *mut libc::c_char,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut sa: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    s = anetCreateSocket(err, 2 as libc::c_int);
    if s == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    memset(
        &mut sa as *mut sockaddr_in as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
    );
    sa.sin_family = 2 as libc::c_int as sa_family_t;
    sa.sin_port = __bswap_16(port as __uint16_t);
    sa.sin_addr.s_addr = __bswap_32(0 as libc::c_int as in_addr_t);
    if !bindaddr.is_null() {
        tmp = inet_aton(bindaddr as *const libc::c_char, &mut sa.sin_addr);
        if tmp == 0 as libc::c_int {
            anetSetError(
                err,
                b"invalid bind address\0" as *const u8 as *const libc::c_char,
            );
            close(s);
            return -(1 as libc::c_int);
        }
    }
    tmp___0 = anetListen(
        err,
        s,
        &mut sa as *mut sockaddr_in as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    );
    if tmp___0 == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    return s;
}
pub unsafe extern "C" fn anetUnixServer(
    mut err: *mut libc::c_char,
    mut path: *mut libc::c_char,
    mut perm: mode_t,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut sa: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut tmp: libc::c_int = 0;
    s = anetCreateSocket(err, 1 as libc::c_int);
    if s == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    memset(
        &mut sa as *mut sockaddr_un as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong,
    );
    sa.sun_family = 1 as libc::c_int as sa_family_t;
    strncpy(
        (sa.sun_path).as_mut_ptr(),
        path as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_ulong),
    );
    tmp = anetListen(
        err,
        s,
        &mut sa as *mut sockaddr_un as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong as socklen_t,
    );
    if tmp == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if perm != 0 {
        chmod((sa.sun_path).as_mut_ptr() as *const libc::c_char, perm);
    }
    return s;
}
unsafe extern "C" fn anetGenericAccept(
    mut err: *mut libc::c_char,
    mut s: libc::c_int,
    mut sa: *mut sockaddr,
    mut len: *mut socklen_t,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    loop {
        fd = accept(s, sa, len);
        if !(fd == -(1 as libc::c_int)) {
            break;
        }
        tmp___1 = __errno_location();
        if *tmp___1 == 4 as libc::c_int {
            continue;
        }
        tmp = __errno_location();
        tmp___0 = strerror(*tmp);
        anetSetError(err, b"accept: %s\0" as *const u8 as *const libc::c_char, tmp___0);
        return -(1 as libc::c_int);
    }
    return fd;
}
pub unsafe extern "C" fn anetTcpAccept(
    mut err: *mut libc::c_char,
    mut s: libc::c_int,
    mut ip: *mut libc::c_char,
    mut port: *mut libc::c_int,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut sa: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut salen: socklen_t = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: __uint16_t = 0;
    salen = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    fd = anetGenericAccept(
        err,
        s,
        &mut sa as *mut sockaddr_in as *mut sockaddr,
        &mut salen,
    );
    if fd == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if !ip.is_null() {
        tmp = inet_ntoa(sa.sin_addr);
        strcpy(ip, tmp as *const libc::c_char);
    }
    if !port.is_null() {
        tmp___0 = __bswap_16(sa.sin_port);
        *port = tmp___0 as libc::c_int;
    }
    return fd;
}
pub unsafe extern "C" fn anetUnixAccept(
    mut err: *mut libc::c_char,
    mut s: libc::c_int,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut sa: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut salen: socklen_t = 0;
    salen = ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong as socklen_t;
    fd = anetGenericAccept(
        err,
        s,
        &mut sa as *mut sockaddr_un as *mut sockaddr,
        &mut salen,
    );
    if fd == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    return fd;
}
pub unsafe extern "C" fn anetPeerToString(
    mut fd: libc::c_int,
    mut ip: *mut libc::c_char,
    mut port: *mut libc::c_int,
) -> libc::c_int {
    let mut sa: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut salen: socklen_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: __uint16_t = 0;
    salen = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    tmp = getpeername(
        fd,
        &mut sa as *mut sockaddr_in as *mut sockaddr,
        &mut salen as *mut socklen_t,
    );
    if tmp == -(1 as libc::c_int) {
        *port = 0 as libc::c_int;
        *ip.offset(0 as libc::c_int as isize) = '?' as i32 as libc::c_char;
        *ip.offset(1 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
        return -(1 as libc::c_int);
    }
    if !ip.is_null() {
        tmp___0 = inet_ntoa(sa.sin_addr);
        strcpy(ip, tmp___0 as *const libc::c_char);
    }
    if !port.is_null() {
        tmp___1 = __bswap_16(sa.sin_port);
        *port = tmp___1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn anetSockName(
    mut fd: libc::c_int,
    mut ip: *mut libc::c_char,
    mut port: *mut libc::c_int,
) -> libc::c_int {
    let mut sa: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut salen: socklen_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: __uint16_t = 0;
    salen = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    tmp = getsockname(
        fd,
        &mut sa as *mut sockaddr_in as *mut sockaddr,
        &mut salen as *mut socklen_t,
    );
    if tmp == -(1 as libc::c_int) {
        *port = 0 as libc::c_int;
        *ip.offset(0 as libc::c_int as isize) = '?' as i32 as libc::c_char;
        *ip.offset(1 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
        return -(1 as libc::c_int);
    }
    if !ip.is_null() {
        tmp___0 = inet_ntoa(sa.sin_addr);
        strcpy(ip, tmp___0 as *const libc::c_char);
    }
    if !port.is_null() {
        tmp___1 = __bswap_16(sa.sin_port);
        *port = tmp___1 as libc::c_int;
    }
    return 0 as libc::c_int;
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
unsafe extern "C" fn run_static_initializers() {
    modesNetServices = [
        {
            let mut init = __anonstruct_modesNetServices_483049182 {
                descr: b"Raw TCP output\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                socket: &mut Modes.ros,
                port: 30002 as libc::c_int,
            };
            init
        },
        {
            let mut init = __anonstruct_modesNetServices_483049182 {
                descr: b"Raw TCP input\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                socket: &mut Modes.ris,
                port: 30001 as libc::c_int,
            };
            init
        },
        {
            let mut init = __anonstruct_modesNetServices_483049182 {
                descr: b"HTTP server\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                socket: &mut Modes.https,
                port: 8080 as libc::c_int,
            };
            init
        },
        {
            let mut init = __anonstruct_modesNetServices_483049182 {
                descr: b"Basestation TCP output\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                socket: &mut Modes.sbsos,
                port: 30003 as libc::c_int,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
