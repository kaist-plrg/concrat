use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    pub type internal_state;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn getchar() -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn abs(_: libc::c_int) -> libc::c_int;
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
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn time(__timer: *mut time_t) -> time_t;
    fn mktime(__tp: *mut tm) -> time_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn ctime(__timer: *const time_t) -> *mut libc::c_char;
    fn signal(
        __sig: libc::c_int,
        __handler: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> __sighandler_t;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn utimes(__file: *const libc::c_char, __tvp: *const timeval) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off64_t, __whence: libc::c_int) -> __off64_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn chown(
        __file: *const libc::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn fsync(__fd: libc::c_int) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn zlibVersion() -> *const libc::c_char;
    fn deflate(strm_0: z_streamp, flush: libc::c_int) -> libc::c_int;
    fn deflateEnd(strm_0: z_streamp) -> libc::c_int;
    fn deflateSetDictionary(
        strm_0: z_streamp,
        dictionary: *const Bytef,
        dictLength: uInt,
    ) -> libc::c_int;
    fn deflateReset(strm_0: z_streamp) -> libc::c_int;
    fn deflateParams(
        strm_0: z_streamp,
        level: libc::c_int,
        strategy: libc::c_int,
    ) -> libc::c_int;
    fn deflatePending(
        strm_0: z_streamp,
        pending: *mut libc::c_uint,
        bits: *mut libc::c_int,
    ) -> libc::c_int;
    fn deflatePrime(
        strm_0: z_streamp,
        bits: libc::c_int,
        value: libc::c_int,
    ) -> libc::c_int;
    fn inflateBack(
        strm_0: z_streamp,
        in_1: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *mut *mut libc::c_uchar,
            ) -> libc::c_uint,
        >,
        in_desc: *mut libc::c_void,
        out_0: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *mut libc::c_uchar,
                libc::c_uint,
            ) -> libc::c_int,
        >,
        out_desc: *mut libc::c_void,
    ) -> libc::c_int;
    fn inflateBackEnd(strm_0: z_streamp) -> libc::c_int;
    fn adler32(adler: uLong, buf: *const Bytef, len: uInt) -> uLong;
    fn crc32(crc: uLong, buf: *const Bytef, len: uInt) -> uLong;
    fn deflateInit2_(
        strm_0: z_streamp,
        level: libc::c_int,
        method: libc::c_int,
        windowBits: libc::c_int,
        memLevel: libc::c_int,
        strategy: libc::c_int,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
    fn inflateBackInit_(
        strm_0: z_streamp,
        windowBits: libc::c_int,
        window: *mut libc::c_uchar,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
    fn get_crc_table() -> *const z_crc_t;
    fn ZopfliInitOptions(options: *mut ZopfliOptions);
    fn ZopfliDeflatePart(
        options: *const ZopfliOptions,
        btype: libc::c_int,
        final_0: libc::c_int,
        in_1: *const libc::c_uchar,
        instart: size_t,
        inend: size_t,
        bp: *mut libc::c_uchar,
        out_0: *mut *mut libc::c_uchar,
        outsize: *mut size_t,
    );
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn pthread_getspecific(__key: pthread_key_t) -> *mut libc::c_void;
    fn pthread_setspecific(
        __key: pthread_key_t,
        __pointer: *const libc::c_void,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn pthread_once(
        __once_control: *mut pthread_once_t,
        __init_routine: Option::<unsafe extern "C" fn() -> ()>,
    ) -> libc::c_int;
    fn pthread_key_create(
        __key: *mut pthread_key_t,
        __destr_function: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_equal(__thread1: pthread_t, __thread2: pthread_t) -> libc::c_int;
    fn pthread_attr_init(__attr: *mut pthread_attr_t) -> libc::c_int;
    fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> libc::c_int;
    fn pthread_attr_setdetachstate(
        __attr: *mut pthread_attr_t,
        __detachstate: libc::c_int,
    ) -> libc::c_int;
    fn __pthread_register_cancel(__buf: *mut __pthread_unwind_buf_t);
    fn __pthread_unregister_cancel(__buf: *mut __pthread_unwind_buf_t);
    fn __pthread_unwind_next(__buf: *mut __pthread_unwind_buf_t) -> !;
    fn __sigsetjmp(__env: *mut __jmp_buf_tag, __savemask: libc::c_int) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
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
pub type __uint32_t = libc::c_uint;
pub type __uint_least16_t = __uint16_t;
pub type __uintmax_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __ino64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
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
pub type va_list___0 = __gnuc_va_list;
pub type off_t = __off64_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type pthread_key_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub __tm_gmtoff: libc::c_long,
    pub __tm_zone: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___sigset_t_991265788 {
    pub __val: [libc::c_ulong; 16],
}
pub type __sigset_t = __anonstruct___sigset_t_991265788;
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
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino64_t,
    pub d_off: __off64_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
pub type uint32_t = __uint32_t;
pub type uint_least16_t = __uint_least16_t;
pub type uintmax_t = __uintmax_t;
pub type length_t = uintmax_t;
pub type crc_t = uint32_t;
pub type index_t = uint_least16_t;
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type voidpf = *mut libc::c_void;
pub type z_crc_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut libc::c_char,
    pub state: *mut internal_state,
    pub zalloc: Option::<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>,
    pub zfree: Option::<unsafe extern "C" fn(voidpf, voidpf) -> ()>,
    pub opaque: voidpf,
    pub data_type: libc::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thread_s {
    pub id: pthread_t,
    pub done: libc::c_int,
    pub next: *mut thread,
}
pub type thread = thread_s;
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lock_s {
    pub mutex: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub value: libc::c_long,
}
pub type pthread_cond_t = __anonunion_pthread_cond_t_951761805;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_cond_t_951761805 {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __annonCompField1: __anonunion____missing_field_name_419937108,
    pub __annonCompField2: __anonunion____missing_field_name_845759682,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_845759682 {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: __anonstruct___g1_start32_845759683,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___g1_start32_845759683 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_419937108 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: __anonstruct___wseq32_112954846,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___wseq32_112954846 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
pub type pthread_mutex_t = __anonunion_pthread_mutex_t_335460617;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_mutex_t_335460617 {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
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
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type lock = lock_s;
pub type twist_op = libc::c_uint;
pub const BY: twist_op = 1;
pub const TO: twist_op = 0;
pub type wait_op = libc::c_uint;
pub const TO_BE_LESS_THAN: wait_op = 3;
pub const TO_BE_MORE_THAN: wait_op = 2;
pub const NOT_TO_BE: wait_op = 1;
pub const TO_BE: wait_op = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliOptions {
    pub verbose: libc::c_int,
    pub verbose_more: libc::c_int,
    pub numiterations: libc::c_int,
    pub blocksplitting: libc::c_int,
    pub blocksplittinglast: libc::c_int,
    pub blocksplittingmax: libc::c_int,
}
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_try_ball_t__852378850 {
    pub ret: libc::c_int,
    pub code: libc::c_int,
    pub free: libc::c_int,
    pub why: *mut libc::c_char,
}
pub type try_ball_t_ = __anonstruct_try_ball_t__852378850;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct try_s_ {
    pub env: jmp_buf,
    pub ball: try_ball_t_,
    pub next: *mut try_t_,
}
pub type try_t_ = try_s_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_g_4656454 {
    pub ret: libc::c_int,
    pub prog: *mut libc::c_char,
    pub ind: libc::c_int,
    pub outd: libc::c_int,
    pub inf: *mut libc::c_char,
    pub inz: size_t,
    pub outf: *mut libc::c_char,
    pub verbosity: libc::c_int,
    pub headis: libc::c_int,
    pub pipeout: libc::c_int,
    pub keep: libc::c_int,
    pub force: libc::c_int,
    pub sync: libc::c_int,
    pub form: libc::c_int,
    pub magic1: libc::c_int,
    pub recurse: libc::c_int,
    pub sufx: *mut libc::c_char,
    pub name: *mut libc::c_char,
    pub alias: *mut libc::c_char,
    pub comment: *mut libc::c_char,
    pub mtime: time_t,
    pub list: libc::c_int,
    pub first: libc::c_int,
    pub decode: libc::c_int,
    pub level: libc::c_int,
    pub strategy: libc::c_int,
    pub zopts: ZopfliOptions,
    pub rsync: libc::c_int,
    pub procs: libc::c_int,
    pub setdict: libc::c_int,
    pub block: size_t,
    pub shift: crc_t,
    pub stamp: time_t,
    pub hname: *mut libc::c_char,
    pub hcomm: *mut libc::c_char,
    pub zip_crc: libc::c_ulong,
    pub zip_clen: length_t,
    pub zip_ulen: length_t,
    pub zip64: libc::c_int,
    pub in_buf: [libc::c_uchar; 32810],
    pub in_next: *mut libc::c_uchar,
    pub in_left: size_t,
    pub in_eof: libc::c_int,
    pub in_short: libc::c_int,
    pub in_tot: length_t,
    pub out_tot: length_t,
    pub out_check: libc::c_ulong,
    pub in_buf2: [libc::c_uchar; 32810],
    pub in_len: size_t,
    pub in_which: libc::c_int,
    pub load_state: *mut lock,
    pub load_thread: *mut thread,
}
pub type val_t = length_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pool {
    pub have: *mut lock,
    pub head: *mut space,
    pub size: size_t,
    pub limit: libc::c_int,
    pub made: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct space {
    pub use_0: *mut lock,
    pub buf: *mut libc::c_uchar,
    pub size: size_t,
    pub len: size_t,
    pub pool: *mut pool,
    pub next: *mut space,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct job {
    pub seq: libc::c_long,
    pub more: libc::c_int,
    pub in_0: *mut space,
    pub out: *mut space,
    pub lens: *mut space,
    pub check: libc::c_ulong,
    pub calc: *mut lock,
    pub next: *mut job,
}
pub type bits_t = libc::c_ulong;
pub type pthread_once_t = libc::c_int;
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
pub struct __anonstruct___cancel_jmp_buf_572769531 {
    pub __cancel_jmp_buf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___pthread_unwind_buf_t_348056374 {
    pub __cancel_jmp_buf: [__anonstruct___cancel_jmp_buf_572769531; 1],
    pub __pad: [*mut libc::c_void; 4],
}
pub type __pthread_unwind_buf_t = __anonstruct___pthread_unwind_buf_t_348056374;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct capsule {
    pub probe: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub payload: *mut libc::c_void,
    pub file: *const libc::c_char,
    pub line: libc::c_long,
}
static mut g: __anonstruct_g_4656454 = __anonstruct_g_4656454 {
    ret: 0,
    prog: 0 as *const libc::c_char as *mut libc::c_char,
    ind: 0,
    outd: 0,
    inf: 0 as *const libc::c_char as *mut libc::c_char,
    inz: 0,
    outf: 0 as *const libc::c_char as *mut libc::c_char,
    verbosity: 0,
    headis: 0,
    pipeout: 0,
    keep: 0,
    force: 0,
    sync: 0,
    form: 0,
    magic1: 0,
    recurse: 0,
    sufx: 0 as *const libc::c_char as *mut libc::c_char,
    name: 0 as *const libc::c_char as *mut libc::c_char,
    alias: 0 as *const libc::c_char as *mut libc::c_char,
    comment: 0 as *const libc::c_char as *mut libc::c_char,
    mtime: 0,
    list: 0,
    first: 0,
    decode: 0,
    level: 0,
    strategy: 0,
    zopts: ZopfliOptions {
        verbose: 0,
        verbose_more: 0,
        numiterations: 0,
        blocksplitting: 0,
        blocksplittinglast: 0,
        blocksplittingmax: 0,
    },
    rsync: 0,
    procs: 0,
    setdict: 0,
    block: 0,
    shift: 0,
    stamp: 0,
    hname: 0 as *const libc::c_char as *mut libc::c_char,
    hcomm: 0 as *const libc::c_char as *mut libc::c_char,
    zip_crc: 0,
    zip_clen: 0,
    zip_ulen: 0,
    zip64: 0,
    in_buf: [0; 32810],
    in_next: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    in_left: 0,
    in_eof: 0,
    in_short: 0,
    in_tot: 0,
    out_tot: 0,
    out_check: 0,
    in_buf2: [0; 32810],
    in_len: 0,
    in_which: 0,
    load_state: 0 as *const lock as *mut lock,
    load_thread: 0 as *const thread as *mut thread,
};
unsafe extern "C" fn message(mut fmt: *mut libc::c_char, mut ap: ::std::ffi::VaList) {
    if g.verbosity > 0 as libc::c_int {
        fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, g.prog);
        vfprintf(stderr, fmt as *const libc::c_char, ap.as_va_list());
        putc('\n' as i32, stderr);
        fflush(stderr);
    }
}
unsafe extern "C" fn complain(mut fmt: *mut libc::c_char, mut args: ...) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    g.ret = 1 as libc::c_int;
    ap = args.clone();
    message(fmt, ap.as_va_list());
    return 0 as libc::c_int;
}
unsafe extern "C" fn grumble(mut fmt: *mut libc::c_char, mut args: ...) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    message(fmt, ap.as_va_list());
    return 0 as libc::c_int;
}
unsafe extern "C" fn alloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    ptr = realloc(ptr, size);
    if ptr as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        try_throw_(
            12 as libc::c_int,
            b"not enough memory\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *mut libc::c_void,
        );
    }
    return ptr;
}
unsafe extern "C" fn cut_short(mut sig: libc::c_int) {
    let mut tmp: libc::c_int = 0;
    if g.outd != -(1 as libc::c_int) {
        if g.outd != 1 as libc::c_int {
            unlink(g.outf as *const libc::c_char);
            if g.outf as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                free(g.outf as *mut libc::c_void);
                g.outf = 0 as *mut libc::c_void as *mut libc::c_char;
            }
            g.outd = -(1 as libc::c_int);
        }
    }
    if sig < 0 as libc::c_int {
        tmp = -sig;
    } else {
        tmp = 4 as libc::c_int;
    }
    _exit(tmp);
}
#[inline]
unsafe extern "C" fn grow(mut size: size_t) -> size_t {
    let mut was: size_t = 0;
    let mut top: size_t = 0;
    let mut shift: libc::c_int = 0;
    was = size;
    size = (size as libc::c_ulong).wrapping_add(size >> 2 as libc::c_int) as size_t
        as size_t;
    top = size;
    shift = 0 as libc::c_int;
    while top > 7 as libc::c_ulong {
        top >>= 1 as libc::c_int;
        shift += 1;
    }
    if top == 7 as libc::c_ulong {
        size = (1 as libc::c_ulong) << shift + 3 as libc::c_int;
    }
    if size < 16 as libc::c_ulong {
        size = 16 as libc::c_int as size_t;
    }
    if size <= was {
        size = 18446744073709551615 as libc::c_ulonglong as size_t;
    }
    return size;
}
#[inline]
unsafe extern "C" fn vmemcpy(
    mut mem: *mut *mut libc::c_char,
    mut size: *mut size_t,
    mut off: size_t,
    mut cpy: *mut libc::c_void,
    mut len: size_t,
) -> size_t {
    let mut need: size_t = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    need = off.wrapping_add(len);
    if need < off {
        try_throw_(
            34 as libc::c_int,
            b"overflow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_void,
        );
    }
    if need > *size {
        need = grow(need);
        if off == 0 as libc::c_ulong {
            if *mem as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                free(*mem as *mut libc::c_void);
                *mem = 0 as *mut libc::c_void as *mut libc::c_char;
            }
            *size = 0 as libc::c_int as size_t;
        }
        tmp = alloc(*mem as *mut libc::c_void, need);
        *mem = tmp as *mut libc::c_char;
        *size = need;
    }
    memcpy(
        (*mem).offset(off as isize) as *mut libc::c_void,
        cpy as *const libc::c_void,
        len,
    );
    return off.wrapping_add(len);
}
#[inline]
unsafe extern "C" fn vstrcpy(
    mut str: *mut *mut libc::c_char,
    mut size: *mut size_t,
    mut off: size_t,
    mut cpy: *mut libc::c_void,
) -> size_t {
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    tmp = strlen(cpy as *const libc::c_char);
    tmp___0 = vmemcpy(str, size, off, cpy, tmp.wrapping_add(1 as libc::c_ulong));
    return tmp___0;
}
unsafe extern "C" fn readn(
    mut desc: libc::c_int,
    mut buf: *mut libc::c_uchar,
    mut len: size_t,
) -> size_t {
    let mut ret: ssize_t = 0;
    let mut got: size_t = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    got = 0 as libc::c_int as size_t;
    while len != 0 {
        ret = read(desc, buf as *mut libc::c_void, len);
        if ret < 0 as libc::c_long {
            tmp = __errno_location();
            tmp___0 = strerror(*tmp);
            tmp___1 = __errno_location();
            try_throw_(
                *tmp___1,
                b"read error on %s (%s)\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                g.inf,
                tmp___0,
                0 as *mut libc::c_void,
            );
        }
        if ret == 0 as libc::c_long {
            break;
        }
        buf = buf.offset(ret as isize);
        len = (len as libc::c_ulong).wrapping_sub(ret as size_t) as size_t as size_t;
        got = (got as libc::c_ulong).wrapping_add(ret as size_t) as size_t as size_t;
    }
    return got;
}
unsafe extern "C" fn writen(
    mut desc: libc::c_int,
    mut buf: *const libc::c_void,
    mut len: size_t,
) -> size_t {
    let mut next___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut left: size_t = 0;
    let mut max: size_t = 0;
    let mut ret: ssize_t = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: ssize_t = 0;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    next___0 = buf as *const libc::c_char;
    left = len;
    while left != 0 {
        max = 9223372036854775807 as libc::c_long as size_t;
        if left > max {
            tmp = max;
        } else {
            tmp = left;
        }
        tmp___0 = write(desc, next___0 as *const libc::c_void, tmp);
        ret = tmp___0;
        if ret < 1 as libc::c_long {
            tmp___1 = __errno_location();
            tmp___2 = strerror(*tmp___1);
            tmp___3 = __errno_location();
            try_throw_(
                *tmp___3,
                b"write error on %s (%s)\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                g.outf,
                tmp___2,
                0 as *mut libc::c_void,
            );
        }
        next___0 = next___0.offset(ret as isize);
        left = (left as libc::c_ulong).wrapping_sub(ret as size_t) as size_t as size_t;
    }
    return len;
}
unsafe extern "C" fn time2dos(mut t: time_t) -> libc::c_ulong {
    let mut tm: *mut tm = 0 as *mut tm;
    let mut dos: libc::c_ulong = 0;
    if t == 0 as libc::c_long {
        t = time(0 as *mut libc::c_void as *mut time_t);
    }
    tm = localtime(&mut t as *mut time_t as *const time_t);
    if (*tm).tm_year < 80 as libc::c_int {
        return 0 as libc::c_ulong
    } else {
        if (*tm).tm_year > 207 as libc::c_int {
            return 0 as libc::c_ulong;
        }
    }
    dos = (((*tm).tm_year - 80 as libc::c_int) as libc::c_ulong) << 25 as libc::c_int;
    dos = dos
        .wrapping_add(
            (((*tm).tm_mon + 1 as libc::c_int) as libc::c_ulong) << 21 as libc::c_int,
        );
    dos = dos.wrapping_add(((*tm).tm_mday as libc::c_ulong) << 16 as libc::c_int);
    dos = dos.wrapping_add(((*tm).tm_hour as libc::c_ulong) << 11 as libc::c_int);
    dos = dos.wrapping_add(((*tm).tm_min as libc::c_ulong) << 5 as libc::c_int);
    dos = dos
        .wrapping_add(
            ((*tm).tm_sec + 1 as libc::c_int) as libc::c_ulong >> 1 as libc::c_int,
        );
    return dos;
}
unsafe extern "C" fn put(mut out___0: libc::c_int, mut args: ...) -> libc::c_uint {
    let mut count: libc::c_uint = 0;
    let mut n: libc::c_int = 0;
    let mut ap: ::std::ffi::VaListImpl;
    let mut tmp___0: val_t = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut wrap: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut next___0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut val: val_t = 0;
    let mut tmp___5: val_t = 0;
    let mut tmp___6: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___7: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___8: libc::c_int = 0;
    count = 0 as libc::c_uint;
    ap = args.clone();
    loop {
        tmp___2 = ap.arg::<libc::c_int>();
        n = tmp___2;
        if !(n != 0 as libc::c_int) {
            break;
        }
        tmp___0 = ap.arg::<val_t>();
        tmp___1 = abs(n);
        count = count.wrapping_add(tmp___1 as libc::c_uint);
    }
    tmp___3 = alloc(0 as *mut libc::c_void, count as size_t);
    wrap = tmp___3 as *mut libc::c_uchar;
    next___0 = wrap;
    ap = args.clone();
    loop {
        tmp___8 = ap.arg::<libc::c_int>();
        n = tmp___8;
        if !(n != 0 as libc::c_int) {
            break;
        }
        tmp___5 = ap.arg::<val_t>();
        val = tmp___5;
        if n < 0 as libc::c_int {
            n = -n << 3 as libc::c_int;
            loop {
                n -= 8 as libc::c_int;
                tmp___6 = next___0;
                next___0 = next___0.offset(1);
                *tmp___6 = (val >> n) as libc::c_uchar;
                if n == 0 {
                    break;
                }
            }
        } else {
            loop {
                tmp___7 = next___0;
                next___0 = next___0.offset(1);
                *tmp___7 = val as libc::c_uchar;
                val >>= 8 as libc::c_int;
                n -= 1;
                if n == 0 {
                    break;
                }
            }
        }
    }
    writen(out___0, wrap as *const libc::c_void, count as size_t);
    free(wrap as *mut libc::c_void);
    return count;
}
unsafe extern "C" fn put_header() -> length_t {
    let mut len: length_t = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: libc::c_ulong = 0;
    let mut tmp___2: libc::c_uint = 0;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: size_t = 0;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: size_t = 0;
    let mut tmp___7: libc::c_uint = 0;
    let mut head: libc::c_uint = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: libc::c_uint = 0;
    let mut tmp___12: libc::c_int = 0;
    let mut tmp___13: libc::c_int = 0;
    let mut tmp___14: libc::c_int = 0;
    let mut tmp___15: libc::c_int = 0;
    let mut tmp___16: libc::c_uint = 0;
    let mut tmp___17: size_t = 0;
    let mut tmp___18: size_t = 0;
    let mut tmp___19: size_t = 0;
    let mut tmp___20: size_t = 0;
    if g.form > 1 as libc::c_int {
        if g.name as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            tmp = g.alias;
        } else {
            tmp = g.name;
        }
        tmp___0 = strlen(tmp as *const libc::c_char);
        tmp___1 = time2dos(g.mtime);
        tmp___2 = put(
            g.outd,
            4 as libc::c_int,
            67324752 as libc::c_int as val_t,
            2 as libc::c_int,
            45 as libc::c_int as val_t,
            2 as libc::c_int,
            8 as libc::c_int as val_t,
            2 as libc::c_int,
            8 as libc::c_int as val_t,
            4 as libc::c_int,
            tmp___1,
            4 as libc::c_int,
            0 as libc::c_int as val_t,
            4 as libc::c_int,
            4294967295 as libc::c_uint as val_t,
            4 as libc::c_int,
            4294967295 as libc::c_uint as val_t,
            2 as libc::c_int,
            tmp___0,
            2 as libc::c_int,
            29 as libc::c_int as val_t,
            0 as libc::c_int,
        );
        len = tmp___2 as length_t;
        if g.name as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            tmp___3 = g.alias;
        } else {
            tmp___3 = g.name;
        }
        tmp___4 = strlen(tmp___3 as *const libc::c_char);
        if g.name as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            tmp___5 = g.alias;
        } else {
            tmp___5 = g.name;
        }
        tmp___6 = writen(g.outd, tmp___5 as *const libc::c_void, tmp___4);
        len = (len as libc::c_ulong).wrapping_add(tmp___6) as length_t as length_t;
        tmp___7 = put(
            g.outd,
            2 as libc::c_int,
            1 as libc::c_int as val_t,
            2 as libc::c_int,
            16 as libc::c_int as val_t,
            8 as libc::c_int,
            0 as libc::c_int as val_t,
            8 as libc::c_int,
            0 as libc::c_int as val_t,
            2 as libc::c_int,
            21589 as libc::c_int as val_t,
            2 as libc::c_int,
            5 as libc::c_int as val_t,
            1 as libc::c_int,
            1 as libc::c_int as val_t,
            4 as libc::c_int,
            g.mtime as val_t,
            0 as libc::c_int,
        );
        len = (len as libc::c_ulong).wrapping_add(tmp___7 as length_t) as length_t
            as length_t;
    } else if g.form != 0 {
        if g.comment as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            complain(
                b"can't store comment in zlib format -- ignoring\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        if g.level >= 9 as libc::c_int {
            tmp___10 = (3 as libc::c_int) << 6 as libc::c_int;
        } else {
            if g.level == 1 as libc::c_int {
                tmp___9 = 0 as libc::c_int;
            } else {
                if g.level >= 6 as libc::c_int {
                    tmp___8 = (1 as libc::c_int) << 6 as libc::c_int;
                } else if g.level == -(1 as libc::c_int) {
                    tmp___8 = (1 as libc::c_int) << 6 as libc::c_int;
                } else {
                    tmp___8 = (2 as libc::c_int) << 6 as libc::c_int;
                }
                tmp___9 = tmp___8;
            }
            tmp___10 = tmp___9;
        }
        head = (((120 as libc::c_int) << 8 as libc::c_int) + tmp___10) as libc::c_uint;
        head = head
            .wrapping_add(
                (31 as libc::c_uint).wrapping_sub(head.wrapping_rem(31 as libc::c_uint)),
            );
        tmp___11 = put(g.outd, -(2 as libc::c_int), head as val_t, 0 as libc::c_int);
        len = tmp___11 as length_t;
    } else {
        if g.level >= 9 as libc::c_int {
            tmp___13 = 2 as libc::c_int;
        } else {
            if g.level == 1 as libc::c_int {
                tmp___12 = 4 as libc::c_int;
            } else {
                tmp___12 = 0 as libc::c_int;
            }
            tmp___13 = tmp___12;
        }
        if g.name as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            tmp___14 = 8 as libc::c_int;
        } else {
            tmp___14 = 0 as libc::c_int;
        }
        if g.comment as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            tmp___15 = 16 as libc::c_int;
        } else {
            tmp___15 = 0 as libc::c_int;
        }
        tmp___16 = put(
            g.outd,
            1 as libc::c_int,
            31 as libc::c_int as val_t,
            1 as libc::c_int,
            139 as libc::c_int as val_t,
            1 as libc::c_int,
            8 as libc::c_int as val_t,
            1 as libc::c_int,
            (tmp___14 + tmp___15) as val_t,
            4 as libc::c_int,
            g.mtime as val_t,
            1 as libc::c_int,
            tmp___13 as val_t,
            1 as libc::c_int,
            3 as libc::c_int as val_t,
            0 as libc::c_int,
        );
        len = tmp___16 as length_t;
        if g.name as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            tmp___17 = strlen(g.name as *const libc::c_char);
            tmp___18 = writen(
                g.outd,
                g.name as *const libc::c_void,
                tmp___17.wrapping_add(1 as libc::c_ulong),
            );
            len = (len as libc::c_ulong).wrapping_add(tmp___18) as length_t as length_t;
        }
        if g.comment as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            tmp___19 = strlen(g.comment as *const libc::c_char);
            tmp___20 = writen(
                g.outd,
                g.comment as *const libc::c_void,
                tmp___19.wrapping_add(1 as libc::c_ulong),
            );
            len = (len as libc::c_ulong).wrapping_add(tmp___20) as length_t as length_t;
        }
    }
    return len;
}
unsafe extern "C" fn put_trailer(
    mut ulen: length_t,
    mut clen: length_t,
    mut check: libc::c_ulong,
    mut head: length_t,
) {
    let mut desc: length_t = 0;
    let mut tmp: libc::c_uint = 0;
    let mut zip64: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut cent: length_t = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: size_t = 0;
    let mut tmp___6: length_t = 0;
    let mut tmp___7: length_t = 0;
    let mut tmp___8: libc::c_ulong = 0;
    let mut tmp___9: libc::c_uint = 0;
    let mut tmp___10: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___11: size_t = 0;
    let mut tmp___12: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___13: size_t = 0;
    let mut tmp___14: libc::c_uint = 0;
    let mut tmp___15: libc::c_uint = 0;
    let mut tmp___16: size_t = 0;
    let mut tmp___17: size_t = 0;
    let mut tmp___18: length_t = 0;
    let mut tmp___19: length_t = 0;
    let mut tmp___20: libc::c_int = 0;
    let mut tmp___21: libc::c_int = 0;
    if g.form > 1 as libc::c_int {
        tmp = put(
            g.outd,
            4 as libc::c_int,
            134695760 as libc::c_int as val_t,
            4 as libc::c_int,
            check,
            8 as libc::c_int,
            clen,
            8 as libc::c_int,
            ulen,
            0 as libc::c_int,
        );
        desc = tmp as length_t;
        if ulen >= 4294967295 as libc::c_ulong {
            tmp___0 = 1 as libc::c_int;
        } else if clen >= 4294967295 as libc::c_ulong {
            tmp___0 = 1 as libc::c_int;
        } else {
            tmp___0 = 0 as libc::c_int;
        }
        zip64 = tmp___0;
        if g.comment as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            tmp___2 = 0 as libc::c_int as size_t;
        } else {
            tmp___1 = strlen(g.comment as *const libc::c_char);
            tmp___2 = tmp___1;
        }
        if zip64 != 0 {
            tmp___3 = 29 as libc::c_int;
        } else {
            tmp___3 = 9 as libc::c_int;
        }
        if g.name as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            tmp___4 = g.alias;
        } else {
            tmp___4 = g.name;
        }
        tmp___5 = strlen(tmp___4 as *const libc::c_char);
        if zip64 != 0 {
            tmp___6 = 4294967295 as libc::c_uint as length_t;
        } else {
            tmp___6 = ulen;
        }
        if zip64 != 0 {
            tmp___7 = 4294967295 as libc::c_uint as length_t;
        } else {
            tmp___7 = clen;
        }
        tmp___8 = time2dos(g.mtime);
        tmp___9 = put(
            g.outd,
            4 as libc::c_int,
            33639248 as libc::c_int as val_t,
            1 as libc::c_int,
            45 as libc::c_int as val_t,
            1 as libc::c_int,
            255 as libc::c_int as val_t,
            2 as libc::c_int,
            45 as libc::c_int as val_t,
            2 as libc::c_int,
            8 as libc::c_int as val_t,
            2 as libc::c_int,
            8 as libc::c_int as val_t,
            4 as libc::c_int,
            tmp___8,
            4 as libc::c_int,
            check,
            4 as libc::c_int,
            tmp___7,
            4 as libc::c_int,
            tmp___6,
            2 as libc::c_int,
            tmp___5,
            2 as libc::c_int,
            tmp___3 as val_t,
            2 as libc::c_int,
            tmp___2,
            2 as libc::c_int,
            0 as libc::c_int as val_t,
            2 as libc::c_int,
            0 as libc::c_int as val_t,
            4 as libc::c_int,
            0 as libc::c_int as val_t,
            4 as libc::c_int,
            0 as libc::c_int as val_t,
            0 as libc::c_int,
        );
        cent = tmp___9 as length_t;
        if g.name as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            tmp___10 = g.alias;
        } else {
            tmp___10 = g.name;
        }
        tmp___11 = strlen(tmp___10 as *const libc::c_char);
        if g.name as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            tmp___12 = g.alias;
        } else {
            tmp___12 = g.name;
        }
        tmp___13 = writen(g.outd, tmp___12 as *const libc::c_void, tmp___11);
        cent = (cent as libc::c_ulong).wrapping_add(tmp___13) as length_t as length_t;
        if zip64 != 0 {
            tmp___14 = put(
                g.outd,
                2 as libc::c_int,
                1 as libc::c_int as val_t,
                2 as libc::c_int,
                16 as libc::c_int as val_t,
                8 as libc::c_int,
                ulen,
                8 as libc::c_int,
                clen,
                0 as libc::c_int,
            );
            cent = (cent as libc::c_ulong).wrapping_add(tmp___14 as length_t) as length_t
                as length_t;
        }
        tmp___15 = put(
            g.outd,
            2 as libc::c_int,
            21589 as libc::c_int as val_t,
            2 as libc::c_int,
            5 as libc::c_int as val_t,
            1 as libc::c_int,
            1 as libc::c_int as val_t,
            4 as libc::c_int,
            g.mtime as val_t,
            0 as libc::c_int,
        );
        cent = (cent as libc::c_ulong).wrapping_add(tmp___15 as length_t) as length_t
            as length_t;
        if g.comment as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            tmp___16 = strlen(g.comment as *const libc::c_char);
            tmp___17 = writen(g.outd, g.comment as *const libc::c_void, tmp___16);
            cent = (cent as libc::c_ulong).wrapping_add(tmp___17) as length_t
                as length_t;
        }
        zip64 = (head.wrapping_add(clen).wrapping_add(desc)
            >= 4294967295 as libc::c_ulong) as libc::c_int;
        if zip64 != 0 {
            put(
                g.outd,
                4 as libc::c_int,
                101075792 as libc::c_int as val_t,
                8 as libc::c_int,
                44 as libc::c_int as val_t,
                2 as libc::c_int,
                45 as libc::c_int as val_t,
                2 as libc::c_int,
                45 as libc::c_int as val_t,
                4 as libc::c_int,
                0 as libc::c_int as val_t,
                4 as libc::c_int,
                0 as libc::c_int as val_t,
                8 as libc::c_int,
                1 as libc::c_int as val_t,
                8 as libc::c_int,
                1 as libc::c_int as val_t,
                8 as libc::c_int,
                cent,
                8 as libc::c_int,
                head.wrapping_add(clen).wrapping_add(desc),
                4 as libc::c_int,
                117853008 as libc::c_int as val_t,
                4 as libc::c_int,
                0 as libc::c_int as val_t,
                8 as libc::c_int,
                head.wrapping_add(clen).wrapping_add(desc).wrapping_add(cent),
                4 as libc::c_int,
                1 as libc::c_int as val_t,
                0 as libc::c_int,
            );
        }
        if zip64 != 0 {
            tmp___18 = 4294967295 as libc::c_uint as length_t;
        } else {
            tmp___18 = head.wrapping_add(clen).wrapping_add(desc);
        }
        if zip64 != 0 {
            tmp___19 = 4294967295 as libc::c_uint as length_t;
        } else {
            tmp___19 = cent;
        }
        if zip64 != 0 {
            tmp___20 = 65535 as libc::c_int;
        } else {
            tmp___20 = 1 as libc::c_int;
        }
        if zip64 != 0 {
            tmp___21 = 65535 as libc::c_int;
        } else {
            tmp___21 = 1 as libc::c_int;
        }
        put(
            g.outd,
            4 as libc::c_int,
            101010256 as libc::c_int as val_t,
            2 as libc::c_int,
            0 as libc::c_int as val_t,
            2 as libc::c_int,
            0 as libc::c_int as val_t,
            2 as libc::c_int,
            tmp___21 as val_t,
            2 as libc::c_int,
            tmp___20 as val_t,
            4 as libc::c_int,
            tmp___19,
            4 as libc::c_int,
            tmp___18,
            2 as libc::c_int,
            0 as libc::c_int as val_t,
            0 as libc::c_int,
        );
    } else if g.form != 0 {
        put(g.outd, -(4 as libc::c_int), check, 0 as libc::c_int);
    } else {
        put(g.outd, 4 as libc::c_int, check, 4 as libc::c_int, ulen, 0 as libc::c_int);
    };
}
unsafe extern "C" fn adler32z(
    mut adler: libc::c_ulong,
    mut buf: *const libc::c_uchar,
    mut len: size_t,
) -> libc::c_ulong {
    let mut tmp: uLong = 0;
    while len > 4294967295 as libc::c_ulong {
        if !(buf as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
            break;
        }
        adler = adler32(adler, buf, 4294967295 as libc::c_uint);
        buf = buf.offset(4294967295 as libc::c_uint as isize);
        len = (len as libc::c_ulong).wrapping_sub(4294967295 as libc::c_ulong) as size_t
            as size_t;
    }
    tmp = adler32(adler, buf, len as libc::c_uint);
    return tmp;
}
unsafe extern "C" fn crc32z(
    mut crc: libc::c_ulong,
    mut buf: *const libc::c_uchar,
    mut len: size_t,
) -> libc::c_ulong {
    let mut tmp: uLong = 0;
    while len > 4294967295 as libc::c_ulong {
        if !(buf as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
            break;
        }
        crc = crc32(crc, buf, 4294967295 as libc::c_uint);
        buf = buf.offset(4294967295 as libc::c_uint as isize);
        len = (len as libc::c_ulong).wrapping_sub(4294967295 as libc::c_ulong) as size_t
            as size_t;
    }
    tmp = crc32(crc, buf, len as libc::c_uint);
    return tmp;
}
unsafe extern "C" fn zlib_vernum() -> libc::c_long {
    let mut ver: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut num___0: libc::c_long = 0;
    let mut left: libc::c_int = 0;
    let mut comp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_long = 0;
    tmp = zlibVersion();
    ver = tmp;
    num___0 = 0 as libc::c_long;
    left = 4 as libc::c_int;
    comp = 0 as libc::c_int;
    let mut current_block_15: u64;
    loop {
        if *ver as libc::c_int >= 48 as libc::c_int {
            if *ver as libc::c_int <= 57 as libc::c_int {
                comp = 10 as libc::c_int * comp + *ver as libc::c_int
                    - 48 as libc::c_int;
                current_block_15 = 7175849428784450219;
            } else {
                current_block_15 = 6516408161211065501;
            }
        } else {
            current_block_15 = 6516408161211065501;
        }
        match current_block_15 {
            6516408161211065501 => {
                if comp > 15 as libc::c_int {
                    tmp___0 = 15 as libc::c_int;
                } else {
                    tmp___0 = comp;
                }
                num___0 = (num___0 << 4 as libc::c_int) + tmp___0 as libc::c_long;
                left -= 1;
                if *ver as libc::c_int != 46 as libc::c_int {
                    break;
                }
                comp = 0 as libc::c_int;
            }
            _ => {}
        }
        ver = ver.offset(1);
        if left == 0 {
            break;
        }
    }
    if left < 2 as libc::c_int {
        tmp___1 = num___0 << (left << 2 as libc::c_int);
    } else {
        tmp___1 = -(1 as libc::c_long);
    }
    return tmp___1;
}
unsafe extern "C" fn multmodp(mut a: crc_t, mut b: crc_t) -> crc_t {
    let mut m: crc_t = 0;
    let mut p: crc_t = 0;
    m = (1 as libc::c_uint) << 31 as libc::c_int;
    p = 0 as libc::c_int as crc_t;
    loop {
        if a & m != 0 {
            p ^= b;
            if a & m.wrapping_sub(1 as libc::c_uint) == 0 as libc::c_uint {
                break;
            }
        }
        m >>= 1 as libc::c_int;
        if b & 1 as libc::c_uint != 0 {
            b = b >> 1 as libc::c_int ^ 3988292384 as libc::c_uint;
        } else {
            b >>= 1 as libc::c_int;
        }
    }
    return p;
}
static mut x2n_table: [crc_t; 32] = [
    1073741824 as libc::c_int as crc_t,
    536870912 as libc::c_int as crc_t,
    134217728 as libc::c_int as crc_t,
    8388608 as libc::c_int as crc_t,
    32768 as libc::c_int as crc_t,
    3988292384 as libc::c_uint,
    2984685714 as libc::c_uint,
    2691310871 as libc::c_uint,
    3982654894 as libc::c_uint,
    2295415911 as libc::c_uint,
    3619421802 as libc::c_uint,
    3963911953 as libc::c_uint,
    2390663536 as libc::c_uint,
    1680310286 as libc::c_int as crc_t,
    1296546528 as libc::c_int as crc_t,
    167662735 as libc::c_int as crc_t,
    2206543119 as libc::c_uint,
    808857370 as libc::c_int as crc_t,
    2069535939 as libc::c_int as crc_t,
    838779241 as libc::c_int as crc_t,
    2683044394 as libc::c_uint,
    1821240772 as libc::c_int as crc_t,
    366380877 as libc::c_int as crc_t,
    1608415822 as libc::c_int as crc_t,
    3134787127 as libc::c_uint,
    776888047 as libc::c_int as crc_t,
    1319870996 as libc::c_int as crc_t,
    2829349568 as libc::c_uint,
    1117427358 as libc::c_int as crc_t,
    344797226 as libc::c_int as crc_t,
    3289097936 as libc::c_uint,
    3303156796 as libc::c_uint,
];
unsafe extern "C" fn x2nmodp(mut n: size_t, mut k: libc::c_uint) -> crc_t {
    let mut p: crc_t = 0;
    p = (1 as libc::c_uint) << 31 as libc::c_int;
    while n != 0 {
        if n & 1 as libc::c_ulong != 0 {
            p = multmodp(x2n_table[(k & 31 as libc::c_uint) as usize], p);
        }
        n >>= 1 as libc::c_int;
        k = k.wrapping_add(1);
    }
    return p;
}
unsafe extern "C" fn crc32_comb(
    mut crc1: libc::c_ulong,
    mut crc2: libc::c_ulong,
    mut len2: size_t,
) -> libc::c_ulong {
    let mut tmp: crc_t = 0;
    let mut tmp___0: crc_t = 0;
    let mut tmp___1: crc_t = 0;
    if len2 == g.block {
        tmp___0 = g.shift;
    } else {
        tmp = x2nmodp(len2, 3 as libc::c_uint);
        tmp___0 = tmp;
    }
    tmp___1 = multmodp(tmp___0, crc1 as crc_t);
    return tmp___1 as libc::c_ulong ^ crc2;
}
unsafe extern "C" fn adler32_comb(
    mut adler1: libc::c_ulong,
    mut adler2: libc::c_ulong,
    mut len2: size_t,
) -> libc::c_ulong {
    let mut sum1: libc::c_ulong = 0;
    let mut sum2: libc::c_ulong = 0;
    let mut rem: libc::c_uint = 0;
    rem = len2.wrapping_rem(65521 as libc::c_ulong) as libc::c_uint;
    sum1 = adler1 & 65535 as libc::c_ulong;
    sum2 = (rem as libc::c_ulong)
        .wrapping_mul(sum1)
        .wrapping_rem(65521 as libc::c_ulong);
    sum1 = sum1
        .wrapping_add(
            (adler2 & 65535 as libc::c_ulong)
                .wrapping_add(65521 as libc::c_ulong)
                .wrapping_sub(1 as libc::c_ulong),
        );
    sum2 = sum2
        .wrapping_add(
            (adler1 >> 16 as libc::c_int & 65535 as libc::c_ulong)
                .wrapping_add(adler2 >> 16 as libc::c_int & 65535 as libc::c_ulong)
                .wrapping_add(65521 as libc::c_ulong)
                .wrapping_sub(rem as libc::c_ulong),
        );
    if sum1 >= 65521 as libc::c_ulong {
        sum1 = sum1.wrapping_sub(65521 as libc::c_ulong);
    }
    if sum1 >= 65521 as libc::c_ulong {
        sum1 = sum1.wrapping_sub(65521 as libc::c_ulong);
    }
    if sum2 >= ((65521 as libc::c_uint) << 1 as libc::c_int) as libc::c_ulong {
        sum2 = sum2
            .wrapping_sub(
                ((65521 as libc::c_uint) << 1 as libc::c_int) as libc::c_ulong,
            );
    }
    if sum2 >= 65521 as libc::c_ulong {
        sum2 = sum2.wrapping_sub(65521 as libc::c_ulong);
    }
    return sum1 | sum2 << 16 as libc::c_int;
}
unsafe extern "C" fn new_pool(
    mut pool: *mut pool,
    mut size: size_t,
    mut limit: libc::c_int,
) {
    (*pool)
        .have = new_lock_(
        0 as libc::c_long,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1457 as libc::c_long,
    );
    (*pool).head = 0 as *mut libc::c_void as *mut space;
    (*pool).size = size;
    (*pool).limit = limit;
    (*pool).made = 0 as libc::c_int;
}
unsafe extern "C" fn get_space(mut pool: *mut pool) -> *mut space {
    let mut space: *mut space = 0 as *mut space;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    possess_(
        (*pool).have,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1470 as libc::c_long,
    );
    if (*pool).limit == 0 as libc::c_int {
        wait_for_(
            (*pool).have,
            NOT_TO_BE,
            0 as libc::c_long,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1472 as libc::c_long,
        );
    }
    if (*pool).head as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        space = (*pool).head;
        (*pool).head = (*space).next;
        twist_(
            (*pool).have,
            BY,
            -(1 as libc::c_long),
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1478 as libc::c_long,
        );
        possess_(
            (*space).use_0,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1479 as libc::c_long,
        );
        twist_(
            (*space).use_0,
            TO,
            1 as libc::c_long,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1480 as libc::c_long,
        );
        (*space).len = 0 as libc::c_int as size_t;
        return space;
    }
    if !((*pool).limit != 0 as libc::c_int) {
        __assert_fail(
            b"pool->limit != 0\0" as *const u8 as *const libc::c_char,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1486 as libc::c_uint,
            b"get_space\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*pool).limit > 0 as libc::c_int {
        (*pool).limit -= 1;
    }
    (*pool).made += 1;
    release_(
        (*pool).have,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1490 as libc::c_long,
    );
    tmp___0 = alloc(
        0 as *mut libc::c_void,
        ::std::mem::size_of::<space>() as libc::c_ulong,
    );
    space = tmp___0 as *mut space;
    (*space)
        .use_0 = new_lock_(
        1 as libc::c_long,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1492 as libc::c_long,
    );
    tmp___1 = alloc(0 as *mut libc::c_void, (*pool).size);
    (*space).buf = tmp___1 as *mut libc::c_uchar;
    (*space).size = (*pool).size;
    (*space).len = 0 as libc::c_int as size_t;
    (*space).pool = pool;
    return space;
}
unsafe extern "C" fn grow_space(mut space: *mut space) {
    let mut more: size_t = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    more = grow((*space).size);
    if more == (*space).size {
        try_throw_(
            34 as libc::c_int,
            b"overflow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_void,
        );
    }
    tmp = alloc((*space).buf as *mut libc::c_void, more);
    (*space).buf = tmp as *mut libc::c_uchar;
    (*space).size = more;
}
unsafe extern "C" fn use_space(mut space: *mut space) {
    let mut use_0: libc::c_long = 0;
    possess_(
        (*space).use_0,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1519 as libc::c_long,
    );
    use_0 = peek_lock((*space).use_0);
    if !(use_0 != 0 as libc::c_long) {
        __assert_fail(
            b"use != 0\0" as *const u8 as *const libc::c_char,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1521 as libc::c_uint,
            b"use_space\0" as *const u8 as *const libc::c_char,
        );
    }
    twist_(
        (*space).use_0,
        BY,
        1 as libc::c_long,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1522 as libc::c_long,
    );
}
unsafe extern "C" fn drop_space(mut space: *mut space) {
    let mut use_0: libc::c_long = 0;
    let mut pool: *mut pool = 0 as *mut pool;
    if space as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    possess_(
        (*space).use_0,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1532 as libc::c_long,
    );
    use_0 = peek_lock((*space).use_0);
    if !(use_0 != 0 as libc::c_long) {
        __assert_fail(
            b"use != 0\0" as *const u8 as *const libc::c_char,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1534 as libc::c_uint,
            b"drop_space\0" as *const u8 as *const libc::c_char,
        );
    }
    twist_(
        (*space).use_0,
        BY,
        -(1 as libc::c_long),
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1535 as libc::c_long,
    );
    if use_0 == 1 as libc::c_long {
        pool = (*space).pool;
        possess_(
            (*pool).have,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1538 as libc::c_long,
        );
        (*space).next = (*pool).head;
        (*pool).head = space;
        twist_(
            (*pool).have,
            BY,
            1 as libc::c_long,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1541 as libc::c_long,
        );
    }
}
unsafe extern "C" fn free_pool(mut pool: *mut pool) -> libc::c_int {
    let mut count: libc::c_int = 0;
    let mut space: *mut space = 0 as *mut space;
    possess_(
        (*pool).have,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1551 as libc::c_long,
    );
    count = 0 as libc::c_int;
    loop {
        space = (*pool).head;
        if !(space as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
            break;
        }
        (*pool).head = (*space).next;
        free((*space).buf as *mut libc::c_void);
        free_lock_(
            (*space).use_0,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1556 as libc::c_long,
        );
        free(space as *mut libc::c_void);
        count += 1;
    }
    if !(count == (*pool).made) {
        __assert_fail(
            b"count == pool->made\0" as *const u8 as *const libc::c_char,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1560 as libc::c_uint,
            b"free_pool\0" as *const u8 as *const libc::c_char,
        );
    }
    release_(
        (*pool).have,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1561 as libc::c_long,
    );
    free_lock_(
        (*pool).have,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1562 as libc::c_long,
    );
    return count;
}
static mut in_pool: pool = pool {
    have: 0 as *const lock as *mut lock,
    head: 0 as *const space as *mut space,
    size: 0,
    limit: 0,
    made: 0,
};
static mut out_pool: pool = pool {
    have: 0 as *const lock as *mut lock,
    head: 0 as *const space as *mut space,
    size: 0,
    limit: 0,
    made: 0,
};
static mut dict_pool: pool = pool {
    have: 0 as *const lock as *mut lock,
    head: 0 as *const space as *mut space,
    size: 0,
    limit: 0,
    made: 0,
};
static mut lens_pool: pool = pool {
    have: 0 as *const lock as *mut lock,
    head: 0 as *const space as *mut space,
    size: 0,
    limit: 0,
    made: 0,
};
static mut compress_have: *mut lock = 0 as *const libc::c_void as *mut libc::c_void
    as *mut lock;
static mut compress_head: *mut job = 0 as *const job as *mut job;
static mut compress_tail: *mut *mut job = 0 as *const *mut job as *mut *mut job;
static mut write_first: *mut lock = 0 as *const lock as *mut lock;
static mut write_head: *mut job = 0 as *const job as *mut job;
static mut cthreads: libc::c_int = 0 as libc::c_int;
static mut writeth: *mut thread = 0 as *const libc::c_void as *mut libc::c_void
    as *mut thread;
unsafe extern "C" fn setup_jobs() {
    if compress_have as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    compress_have = new_lock_(
        0 as libc::c_long,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1609 as libc::c_long,
    );
    compress_head = 0 as *mut libc::c_void as *mut job;
    compress_tail = &mut compress_head;
    write_first = new_lock_(
        -(1 as libc::c_long),
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1612 as libc::c_long,
    );
    write_head = 0 as *mut libc::c_void as *mut job;
    new_pool(&mut in_pool, g.block, (g.procs << 1 as libc::c_int) + 3 as libc::c_int);
    new_pool(
        &mut out_pool,
        (g.block)
            .wrapping_add(g.block >> 4 as libc::c_int)
            .wrapping_add(32768 as libc::c_ulong),
        -(1 as libc::c_int),
    );
    new_pool(&mut dict_pool, 32768 as libc::c_uint as size_t, -(1 as libc::c_int));
    new_pool(&mut lens_pool, g.block >> 11 as libc::c_int, -(1 as libc::c_int));
}
unsafe extern "C" fn finish_jobs() {
    let mut job: job = job {
        seq: 0,
        more: 0,
        in_0: 0 as *mut space,
        out: 0 as *mut space,
        lens: 0 as *mut space,
        check: 0,
        calc: 0 as *mut lock,
        next: 0 as *mut job,
    };
    let mut caught: libc::c_int = 0;
    if compress_have as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    possess_(
        compress_have,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1635 as libc::c_long,
    );
    job.seq = -(1 as libc::c_long);
    job.next = 0 as *mut libc::c_void as *mut job;
    compress_head = &mut job;
    compress_tail = &mut job.next;
    twist_(
        compress_have,
        BY,
        1 as libc::c_long,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1640 as libc::c_long,
    );
    caught = join_all_(
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1643 as libc::c_long,
    );
    if !(caught == cthreads) {
        __assert_fail(
            b"caught == cthreads\0" as *const u8 as *const libc::c_char,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1645 as libc::c_uint,
            b"finish_jobs\0" as *const u8 as *const libc::c_char,
        );
    }
    cthreads = 0 as libc::c_int;
    caught = free_pool(&mut lens_pool);
    caught = free_pool(&mut dict_pool);
    caught = free_pool(&mut out_pool);
    caught = free_pool(&mut in_pool);
    free_lock_(
        write_first,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1657 as libc::c_long,
    );
    free_lock_(
        compress_have,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1658 as libc::c_long,
    );
    compress_have = 0 as *mut libc::c_void as *mut lock;
}
unsafe extern "C" fn deflate_engine(
    mut strm___0: *mut z_stream,
    mut out___0: *mut space,
    mut flush: libc::c_int,
) {
    let mut room: size_t = 0;
    loop {
        room = ((*out___0).size).wrapping_sub((*out___0).len);
        if room == 0 as libc::c_ulong {
            grow_space(out___0);
            room = ((*out___0).size).wrapping_sub((*out___0).len);
        }
        (*strm___0).next_out = ((*out___0).buf).offset((*out___0).len as isize);
        if room < 4294967295 as libc::c_ulong {
            (*strm___0).avail_out = room as libc::c_uint;
        } else {
            (*strm___0).avail_out = 4294967295 as libc::c_uint;
        }
        deflate(strm___0, flush);
        (*out___0)
            .len = ((*strm___0).next_out).offset_from((*out___0).buf) as libc::c_long
            as size_t;
        if !((*strm___0).avail_out == 0 as libc::c_uint) {
            break;
        }
    }
    if !((*strm___0).avail_in == 0 as libc::c_uint) {
        __assert_fail(
            b"strm->avail_in == 0\0" as *const u8 as *const libc::c_char,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1680 as libc::c_uint,
            b"deflate_engine\0" as *const u8 as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn compress_thread(mut dummy: *mut libc::c_void) {
    let mut job: *mut job = 0 as *mut job;
    let mut here: *mut job = 0 as *mut job;
    let mut prior: *mut *mut job = 0 as *mut *mut job;
    let mut check: libc::c_ulong = 0;
    let mut next___0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut left: size_t = 0;
    let mut len: size_t = 0;
    let mut bits: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut err: try_ball_t_ = try_ball_t_ {
        ret: 0,
        code: 0,
        free: 0,
        why: 0 as *mut libc::c_char,
    };
    let mut try_this_: try_t_ = try_t_ {
        env: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
        ball: try_ball_t_ {
            ret: 0,
            code: 0,
            free: 0,
            why: 0 as *mut libc::c_char,
        },
        next: 0 as *mut try_t_,
    };
    let mut try_pushed_: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: libc::c_int = 0;
    let mut strm___0: z_stream = z_stream {
        next_in: 0 as *mut Bytef,
        avail_in: 0,
        total_in: 0,
        next_out: 0 as *mut Bytef,
        avail_out: 0,
        total_out: 0,
        msg: 0 as *mut libc::c_char,
        state: 0 as *mut internal_state,
        zalloc: None,
        zfree: None,
        opaque: 0 as *mut libc::c_void,
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    let mut temp: *mut space = 0 as *mut space;
    let mut tmp___5: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___6: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___7: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___8: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___9: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___10: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___11: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___13: libc::c_long = 0;
    let mut bits___0: libc::c_uchar = 0;
    let mut out___0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut outsize: size_t = 0;
    let mut tmp___14: libc::c_int = 0;
    let mut tmp___16: size_t = 0;
    let mut tmp___17: size_t = 0;
    let mut tmp___18: size_t = 0;
    let mut tmp___19: size_t = 0;
    let mut tmp___20: size_t = 0;
    let mut tmp___21: size_t = 0;
    let mut tmp___22: size_t = 0;
    let mut tmp___23: size_t = 0;
    let mut tmp___24: size_t = 0;
    let mut tmp___25: size_t = 0;
    let mut tmp___26: size_t = 0;
    let mut tmp___27: libc::c_ulong = 0;
    let mut tmp___28: libc::c_ulong = 0;
    let mut tmp___29: libc::c_ulong = 0;
    let mut tmp___30: libc::c_ulong = 0;
    let mut tmp___31: libc::c_ulong = 0;
    let mut tmp___32: libc::c_ulong = 0;
    let mut tmp___33: libc::c_int = 0;
    let mut tmp___37: libc::c_int = 0;
    try_pushed_ = 1 as libc::c_int;
    try_this_.ball.ret = 0 as libc::c_int;
    try_this_.ball.code = 0 as libc::c_int;
    try_this_.ball.free = 0 as libc::c_int;
    try_this_.ball.why = 0 as *mut libc::c_void as *mut libc::c_char;
    try_setup_();
    tmp = pthread_getspecific(try_key_);
    try_this_.next = tmp as *mut try_t_;
    tmp___3 = pthread_setspecific(
        try_key_,
        &mut try_this_ as *mut try_t_ as *const libc::c_void,
    );
    if tmp___3 == 0 as libc::c_int {
        if (b"try: pthread_setspecific() failed\0" as *const u8 as *const libc::c_char)
            .is_null()
        {
            __assert_fail(
                b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1703 as libc::c_uint,
                b"compress_thread\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        __assert_fail(
            b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                as *const u8 as *const libc::c_char,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1703 as libc::c_uint,
            b"compress_thread\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___33 = _setjmp((try_this_.env).as_mut_ptr());
    if tmp___33 == 0 as libc::c_int {
        temp = 0 as *mut libc::c_void as *mut space;
        if g.level > 9 as libc::c_int {
            temp = get_space(&mut out_pool);
        } else {
            strm___0.zfree = None;
            strm___0.zalloc = None;
            strm___0.opaque = 0 as voidpf;
            ret = deflateInit2_(
                &mut strm___0,
                6 as libc::c_int,
                8 as libc::c_int,
                -(15 as libc::c_int),
                8 as libc::c_int,
                g.strategy,
                b"1.2.11\0" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
            );
            if ret == -(4 as libc::c_int) {
                try_throw_(
                    12 as libc::c_int,
                    b"not enough memory\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *mut libc::c_void,
                );
            }
            if ret != 0 as libc::c_int {
                try_throw_(
                    22 as libc::c_int,
                    b"internal error\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *mut libc::c_void,
                );
            }
        }
        loop {
            possess_(
                compress_have,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1727 as libc::c_long,
            );
            wait_for_(
                compress_have,
                NOT_TO_BE,
                0 as libc::c_long,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1728 as libc::c_long,
            );
            job = compress_head;
            if !(job as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
                __assert_fail(
                    b"job != NULL\0" as *const u8 as *const libc::c_char,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    1730 as libc::c_uint,
                    b"compress_thread\0" as *const u8 as *const libc::c_char,
                );
            }
            if (*job).seq == -(1 as libc::c_long) {
                break;
            }
            compress_head = (*job).next;
            if (*job).next as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                compress_tail = &mut compress_head;
            }
            twist_(
                compress_have,
                BY,
                -(1 as libc::c_long),
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1736 as libc::c_long,
            );
            if g.level <= 9 as libc::c_int {
                deflateReset(&mut strm___0);
                deflateParams(&mut strm___0, g.level, g.strategy);
            } else {
                (*temp).len = 0 as libc::c_int as size_t;
            }
            if (*job).out as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                len = (*(*job).out).len;
                if len < 32768 as libc::c_ulong {
                    left = len;
                } else {
                    left = 32768 as libc::c_uint as size_t;
                }
                if g.level <= 9 as libc::c_int {
                    deflateSetDictionary(
                        &mut strm___0,
                        ((*(*job).out).buf).offset(len.wrapping_sub(left) as isize)
                            as *const Bytef,
                        left as libc::c_uint,
                    );
                } else {
                    memcpy(
                        (*temp).buf as *mut libc::c_void,
                        ((*(*job).out).buf).offset(len.wrapping_sub(left) as isize)
                            as *const libc::c_void,
                        left,
                    );
                    (*temp).len = left;
                }
                drop_space((*job).out);
            }
            (*job).out = get_space(&mut out_pool);
            if g.level <= 9 as libc::c_int {
                strm___0.next_in = (*(*job).in_0).buf;
                strm___0.next_out = (*(*job).out).buf;
            } else {
                memcpy(
                    ((*temp).buf).offset((*temp).len as isize) as *mut libc::c_void,
                    (*(*job).in_0).buf as *const libc::c_void,
                    (*(*job).in_0).len,
                );
            }
            if (*job).lens as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                next___0 = 0 as *mut libc::c_void as *mut libc::c_uchar;
            } else {
                next___0 = (*(*job).lens).buf;
            }
            left = (*(*job).in_0).len;
            (*(*job).out).len = 0 as libc::c_int as size_t;
            loop {
                if next___0 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                    len = 128 as libc::c_int as size_t;
                } else {
                    tmp___5 = next___0;
                    next___0 = next___0.offset(1);
                    len = *tmp___5 as size_t;
                }
                if len < 128 as libc::c_ulong {
                    tmp___6 = next___0;
                    next___0 = next___0.offset(1);
                    len = (len << 8 as libc::c_int)
                        .wrapping_add(*tmp___6 as size_t)
                        .wrapping_add(64 as libc::c_ulong);
                } else if len == 128 as libc::c_ulong {
                    len = left;
                } else if len < 192 as libc::c_ulong {
                    len &= 63 as libc::c_ulong;
                } else if len < 224 as libc::c_ulong {
                    tmp___7 = next___0;
                    next___0 = next___0.offset(1);
                    len = ((len & 31 as libc::c_ulong) << 16 as libc::c_int)
                        .wrapping_add((*tmp___7 as size_t) << 8 as libc::c_int);
                    tmp___8 = next___0;
                    next___0 = next___0.offset(1);
                    len = (len as libc::c_ulong)
                        .wrapping_add(
                            (*tmp___8 as libc::c_uint)
                                .wrapping_add(32832 as libc::c_uint) as size_t,
                        ) as size_t as size_t;
                } else {
                    tmp___9 = next___0;
                    next___0 = next___0.offset(1);
                    len = ((len & 31 as libc::c_ulong) << 24 as libc::c_int)
                        .wrapping_add((*tmp___9 as size_t) << 16 as libc::c_int);
                    tmp___10 = next___0;
                    next___0 = next___0.offset(1);
                    len = (len as libc::c_ulong)
                        .wrapping_add((*tmp___10 as size_t) << 8 as libc::c_int)
                        as size_t as size_t;
                    tmp___11 = next___0;
                    next___0 = next___0.offset(1);
                    len = (len as libc::c_ulong)
                        .wrapping_add(
                            (*tmp___11 as size_t).wrapping_add(2129984 as libc::c_ulong),
                        ) as size_t as size_t;
                }
                left = (left as libc::c_ulong).wrapping_sub(len) as size_t as size_t;
                if g.level <= 9 as libc::c_int {
                    while len
                        > (4294967295 as libc::c_uint)
                            .wrapping_sub(4294967295 as libc::c_uint >> 1 as libc::c_int)
                            as size_t
                    {
                        strm___0
                            .avail_in = (4294967295 as libc::c_uint)
                            .wrapping_sub(
                                4294967295 as libc::c_uint >> 1 as libc::c_int,
                            );
                        deflate_engine(&mut strm___0, (*job).out, 0 as libc::c_int);
                        len = (len as libc::c_ulong)
                            .wrapping_sub(
                                (4294967295 as libc::c_uint)
                                    .wrapping_sub(
                                        4294967295 as libc::c_uint >> 1 as libc::c_int,
                                    ) as size_t,
                            ) as size_t as size_t;
                    }
                    strm___0.avail_in = len as libc::c_uint;
                    let mut current_block_148: u64;
                    if left != 0 {
                        current_block_148 = 8937186175033320391;
                    } else if (*job).more != 0 {
                        current_block_148 = 8937186175033320391;
                    } else {
                        deflate_engine(&mut strm___0, (*job).out, 4 as libc::c_int);
                        current_block_148 = 92352228884877657;
                    }
                    match current_block_148 {
                        8937186175033320391 => {
                            tmp___13 = zlib_vernum();
                            if tmp___13 >= 4704 as libc::c_long {
                                deflate_engine(&mut strm___0, (*job).out, 5 as libc::c_int);
                                deflatePending(
                                    &mut strm___0,
                                    0 as *mut libc::c_uint,
                                    &mut bits,
                                );
                                if bits & 1 as libc::c_int != 0 {
                                    deflate_engine(&mut strm___0, (*job).out, 2 as libc::c_int);
                                } else if g.setdict == 0 {
                                    deflate_engine(&mut strm___0, (*job).out, 2 as libc::c_int);
                                } else if bits & 7 as libc::c_int != 0 {
                                    loop {
                                        bits = deflatePrime(
                                            &mut strm___0,
                                            10 as libc::c_int,
                                            2 as libc::c_int,
                                        );
                                        if !(bits == 0 as libc::c_int) {
                                            __assert_fail(
                                                b"bits == Z_OK\0" as *const u8 as *const libc::c_char,
                                                b"pigz.c\0" as *const u8 as *const libc::c_char,
                                                1840 as libc::c_uint,
                                                b"compress_thread\0" as *const u8 as *const libc::c_char,
                                            );
                                        }
                                        deflatePending(
                                            &mut strm___0,
                                            0 as *mut libc::c_uint,
                                            &mut bits,
                                        );
                                        if bits & 7 as libc::c_int == 0 {
                                            break;
                                        }
                                    }
                                    deflate_engine(&mut strm___0, (*job).out, 5 as libc::c_int);
                                }
                            } else {
                                deflate_engine(&mut strm___0, (*job).out, 2 as libc::c_int);
                            }
                            if g.setdict == 0 {
                                deflate_engine(&mut strm___0, (*job).out, 3 as libc::c_int);
                            }
                        }
                        _ => {}
                    }
                } else {
                    out___0 = 0 as *mut libc::c_void as *mut libc::c_uchar;
                    outsize = 0 as libc::c_int as size_t;
                    bits___0 = 0 as libc::c_int as libc::c_uchar;
                    if left != 0 {
                        tmp___14 = 0 as libc::c_int;
                    } else if (*job).more != 0 {
                        tmp___14 = 0 as libc::c_int;
                    } else {
                        tmp___14 = 1 as libc::c_int;
                    }
                    ZopfliDeflatePart(
                        &mut g.zopts as *mut ZopfliOptions as *const ZopfliOptions,
                        2 as libc::c_int,
                        tmp___14,
                        (*temp).buf as *const libc::c_uchar,
                        (*temp).len,
                        ((*temp).len).wrapping_add(len),
                        &mut bits___0,
                        &mut out___0,
                        &mut outsize,
                    );
                    if !(((*(*job).out).len)
                        .wrapping_add(outsize)
                        .wrapping_add(5 as libc::c_ulong) <= (*(*job).out).size)
                    {
                        __assert_fail(
                            b"job->out->len + outsize + 5 <= job->out->size\0"
                                as *const u8 as *const libc::c_char,
                            b"pigz.c\0" as *const u8 as *const libc::c_char,
                            1869 as libc::c_uint,
                            b"compress_thread\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    memcpy(
                        ((*(*job).out).buf).offset((*(*job).out).len as isize)
                            as *mut libc::c_void,
                        out___0 as *const libc::c_void,
                        outsize,
                    );
                    free(out___0 as *mut libc::c_void);
                    (*(*job).out)
                        .len = ((*(*job).out).len as libc::c_ulong).wrapping_add(outsize)
                        as size_t as size_t;
                    let mut current_block_217: u64;
                    if left != 0 {
                        current_block_217 = 4224859204221704639;
                    } else if (*job).more != 0 {
                        current_block_217 = 4224859204221704639;
                    } else {
                        current_block_217 = 9974864727789713748;
                    }
                    match current_block_217 {
                        4224859204221704639 => {
                            bits___0 = (bits___0 as libc::c_int & 7 as libc::c_int)
                                as libc::c_uchar;
                            let mut current_block_199: u64;
                            if bits___0 as libc::c_int & 1 as libc::c_int != 0 {
                                current_block_199 = 9564479056845141492;
                            } else if g.setdict == 0 {
                                current_block_199 = 9564479056845141492;
                            } else {
                                if bits___0 != 0 {
                                    loop {
                                        *((*(*job).out).buf)
                                            .offset(
                                                ((*(*job).out).len).wrapping_sub(1 as libc::c_ulong)
                                                    as isize,
                                            ) = (*((*(*job).out).buf)
                                            .offset(
                                                ((*(*job).out).len).wrapping_sub(1 as libc::c_ulong)
                                                    as isize,
                                            ) as libc::c_int
                                            + ((2 as libc::c_int) << bits___0 as libc::c_int))
                                            as libc::c_uchar;
                                        tmp___21 = (*(*job).out).len;
                                        (*(*job).out).len = ((*(*job).out).len).wrapping_add(1);
                                        *((*(*job).out).buf)
                                            .offset(
                                                tmp___21 as isize,
                                            ) = 0 as libc::c_int as libc::c_uchar;
                                        bits___0 = (bits___0 as libc::c_int + 2 as libc::c_int)
                                            as libc::c_uchar;
                                        if !((bits___0 as libc::c_int) < 8 as libc::c_int) {
                                            break;
                                        }
                                    }
                                }
                                current_block_199 = 857031028540284188;
                            }
                            match current_block_199 {
                                9564479056845141492 => {
                                    if bits___0 as libc::c_int == 0 as libc::c_int {
                                        tmp___16 = (*(*job).out).len;
                                        (*(*job).out).len = ((*(*job).out).len).wrapping_add(1);
                                        *((*(*job).out).buf)
                                            .offset(
                                                tmp___16 as isize,
                                            ) = 0 as libc::c_int as libc::c_uchar;
                                    } else if bits___0 as libc::c_int > 5 as libc::c_int {
                                        tmp___16 = (*(*job).out).len;
                                        (*(*job).out).len = ((*(*job).out).len).wrapping_add(1);
                                        *((*(*job).out).buf)
                                            .offset(
                                                tmp___16 as isize,
                                            ) = 0 as libc::c_int as libc::c_uchar;
                                    }
                                    tmp___17 = (*(*job).out).len;
                                    (*(*job).out).len = ((*(*job).out).len).wrapping_add(1);
                                    *((*(*job).out).buf)
                                        .offset(
                                            tmp___17 as isize,
                                        ) = 0 as libc::c_int as libc::c_uchar;
                                    tmp___18 = (*(*job).out).len;
                                    (*(*job).out).len = ((*(*job).out).len).wrapping_add(1);
                                    *((*(*job).out).buf)
                                        .offset(
                                            tmp___18 as isize,
                                        ) = 0 as libc::c_int as libc::c_uchar;
                                    tmp___19 = (*(*job).out).len;
                                    (*(*job).out).len = ((*(*job).out).len).wrapping_add(1);
                                    *((*(*job).out).buf)
                                        .offset(
                                            tmp___19 as isize,
                                        ) = 255 as libc::c_int as libc::c_uchar;
                                    tmp___20 = (*(*job).out).len;
                                    (*(*job).out).len = ((*(*job).out).len).wrapping_add(1);
                                    *((*(*job).out).buf)
                                        .offset(
                                            tmp___20 as isize,
                                        ) = 255 as libc::c_int as libc::c_uchar;
                                }
                                _ => {}
                            }
                            if g.setdict == 0 {
                                tmp___22 = (*(*job).out).len;
                                (*(*job).out).len = ((*(*job).out).len).wrapping_add(1);
                                *((*(*job).out).buf)
                                    .offset(
                                        tmp___22 as isize,
                                    ) = 0 as libc::c_int as libc::c_uchar;
                                tmp___23 = (*(*job).out).len;
                                (*(*job).out).len = ((*(*job).out).len).wrapping_add(1);
                                *((*(*job).out).buf)
                                    .offset(
                                        tmp___23 as isize,
                                    ) = 0 as libc::c_int as libc::c_uchar;
                                tmp___24 = (*(*job).out).len;
                                (*(*job).out).len = ((*(*job).out).len).wrapping_add(1);
                                *((*(*job).out).buf)
                                    .offset(
                                        tmp___24 as isize,
                                    ) = 0 as libc::c_int as libc::c_uchar;
                                tmp___25 = (*(*job).out).len;
                                (*(*job).out).len = ((*(*job).out).len).wrapping_add(1);
                                *((*(*job).out).buf)
                                    .offset(
                                        tmp___25 as isize,
                                    ) = 255 as libc::c_int as libc::c_uchar;
                                tmp___26 = (*(*job).out).len;
                                (*(*job).out).len = ((*(*job).out).len).wrapping_add(1);
                                *((*(*job).out).buf)
                                    .offset(
                                        tmp___26 as isize,
                                    ) = 255 as libc::c_int as libc::c_uchar;
                            }
                        }
                        _ => {}
                    }
                    (*temp)
                        .len = ((*temp).len as libc::c_ulong).wrapping_add(len) as size_t
                        as size_t;
                }
                if left == 0 {
                    break;
                }
            }
            drop_space((*job).lens);
            (*job).lens = 0 as *mut libc::c_void as *mut space;
            use_space((*job).in_0);
            possess_(
                write_first,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1911 as libc::c_long,
            );
            prior = &mut write_head;
            loop {
                here = *prior;
                if !(here as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
                    break;
                }
                if (*here).seq > (*job).seq {
                    break;
                }
                prior = &mut (*here).next;
            }
            (*job).next = here;
            *prior = job;
            twist_(
                write_first,
                TO,
                (*write_head).seq,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1920 as libc::c_long,
            );
            len = (*(*job).in_0).len;
            next___0 = (*(*job).in_0).buf;
            if g.form == 1 as libc::c_int {
                tmp___27 = adler32z(
                    0 as libc::c_ulong,
                    0 as *const libc::c_uchar,
                    0 as libc::c_int as size_t,
                );
                check = tmp___27;
            } else {
                tmp___28 = crc32z(
                    0 as libc::c_ulong,
                    0 as *const libc::c_uchar,
                    0 as libc::c_int as size_t,
                );
                check = tmp___28;
            }
            while len
                > (4294967295 as libc::c_uint)
                    .wrapping_sub(4294967295 as libc::c_uint >> 1 as libc::c_int)
                    as size_t
            {
                if g.form == 1 as libc::c_int {
                    tmp___29 = adler32z(
                        check,
                        next___0 as *const libc::c_uchar,
                        (4294967295 as libc::c_uint)
                            .wrapping_sub(4294967295 as libc::c_uint >> 1 as libc::c_int)
                            as size_t,
                    );
                    check = tmp___29;
                } else {
                    tmp___30 = crc32z(
                        check,
                        next___0 as *const libc::c_uchar,
                        (4294967295 as libc::c_uint)
                            .wrapping_sub(4294967295 as libc::c_uint >> 1 as libc::c_int)
                            as size_t,
                    );
                    check = tmp___30;
                }
                len = (len as libc::c_ulong)
                    .wrapping_sub(
                        (4294967295 as libc::c_uint)
                            .wrapping_sub(4294967295 as libc::c_uint >> 1 as libc::c_int)
                            as size_t,
                    ) as size_t as size_t;
                next___0 = next___0
                    .offset(
                        (4294967295 as libc::c_uint)
                            .wrapping_sub(4294967295 as libc::c_uint >> 1 as libc::c_int)
                            as isize,
                    );
            }
            if g.form == 1 as libc::c_int {
                tmp___31 = adler32z(
                    check,
                    next___0 as *const libc::c_uchar,
                    len as libc::c_uint as size_t,
                );
                check = tmp___31;
            } else {
                tmp___32 = crc32z(
                    check,
                    next___0 as *const libc::c_uchar,
                    len as libc::c_uint as size_t,
                );
                check = tmp___32;
            }
            drop_space((*job).in_0);
            (*job).check = check;
            possess_(
                (*job).calc,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1937 as libc::c_long,
            );
            twist_(
                (*job).calc,
                TO,
                1 as libc::c_long,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1938 as libc::c_long,
            );
        }
        release_(
            compress_have,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1944 as libc::c_long,
        );
        if g.level > 9 as libc::c_int {
            drop_space(temp);
        } else {
            deflateEnd(&mut strm___0);
        }
    }
    if try_pushed_ != 0 {
        tmp___37 = pthread_setspecific(try_key_, try_this_.next as *const libc::c_void);
        if tmp___37 == 0 as libc::c_int {
            if (b"try: pthread_setspecific() failed\0" as *const u8
                as *const libc::c_char)
                .is_null()
            {
                __assert_fail(
                    b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                        as *const u8 as *const libc::c_char,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    1954 as libc::c_uint,
                    b"compress_thread\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            __assert_fail(
                b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1954 as libc::c_uint,
                b"compress_thread\0" as *const u8 as *const libc::c_char,
            );
        }
        try_pushed_ = 0 as libc::c_int;
    }
    err = try_this_.ball;
    if err.code != 0 {
        if err.code != 32 as libc::c_int {
            complain(
                b"abort: %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                err.why,
            );
        }
        if err.free != 0 {
            free(err.why as *mut libc::c_void);
            err.free = 0 as libc::c_int;
            err.why = 0 as *mut libc::c_void as *mut libc::c_char;
        }
        cut_short(-err.code);
    }
}
unsafe extern "C" fn write_thread(mut dummy: *mut libc::c_void) {
    let mut seq: libc::c_long = 0;
    let mut job: *mut job = 0 as *mut job;
    let mut len: size_t = 0;
    let mut more: libc::c_int = 0;
    let mut head: length_t = 0;
    let mut ulen: length_t = 0;
    let mut clen: length_t = 0;
    let mut check: libc::c_ulong = 0;
    let mut err: try_ball_t_ = try_ball_t_ {
        ret: 0,
        code: 0,
        free: 0,
        why: 0 as *mut libc::c_char,
    };
    let mut try_this_: try_t_ = try_t_ {
        env: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
        ball: try_ball_t_ {
            ret: 0,
            code: 0,
            free: 0,
            why: 0 as *mut libc::c_char,
        },
        next: 0 as *mut try_t_,
    };
    let mut try_pushed_: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_ulong = 0;
    let mut tmp___5: libc::c_ulong = 0;
    let mut tmp___6: libc::c_long = 0;
    let mut tmp___7: libc::c_ulong = 0;
    let mut tmp___8: libc::c_ulong = 0;
    let mut tmp___12: libc::c_long = 0;
    let mut tmp___14: libc::c_int = 0;
    let mut tmp___18: libc::c_int = 0;
    try_pushed_ = 1 as libc::c_int;
    try_this_.ball.ret = 0 as libc::c_int;
    try_this_.ball.code = 0 as libc::c_int;
    try_this_.ball.free = 0 as libc::c_int;
    try_this_.ball.why = 0 as *mut libc::c_void as *mut libc::c_char;
    try_setup_();
    tmp = pthread_getspecific(try_key_);
    try_this_.next = tmp as *mut try_t_;
    tmp___3 = pthread_setspecific(
        try_key_,
        &mut try_this_ as *mut try_t_ as *const libc::c_void,
    );
    if tmp___3 == 0 as libc::c_int {
        if (b"try: pthread_setspecific() failed\0" as *const u8 as *const libc::c_char)
            .is_null()
        {
            __assert_fail(
                b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1975 as libc::c_uint,
                b"write_thread\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        __assert_fail(
            b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                as *const u8 as *const libc::c_char,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1975 as libc::c_uint,
            b"write_thread\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___14 = _setjmp((try_this_.env).as_mut_ptr());
    if tmp___14 == 0 as libc::c_int {
        head = put_header();
        clen = 0 as libc::c_int as length_t;
        ulen = clen;
        if g.form == 1 as libc::c_int {
            tmp___4 = adler32z(
                0 as libc::c_ulong,
                0 as *const libc::c_uchar,
                0 as libc::c_int as size_t,
            );
            check = tmp___4;
        } else {
            tmp___5 = crc32z(
                0 as libc::c_ulong,
                0 as *const libc::c_uchar,
                0 as libc::c_int as size_t,
            );
            check = tmp___5;
        }
        seq = 0 as libc::c_long;
        loop {
            possess_(
                write_first,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1986 as libc::c_long,
            );
            wait_for_(
                write_first,
                TO_BE,
                seq,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1987 as libc::c_long,
            );
            job = write_head;
            write_head = (*job).next;
            if write_head as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                tmp___6 = -(1 as libc::c_long);
            } else {
                tmp___6 = (*write_head).seq;
            }
            twist_(
                write_first,
                TO,
                tmp___6,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1990 as libc::c_long,
            );
            more = (*job).more;
            len = (*(*job).in_0).len;
            drop_space((*job).in_0);
            ulen = (ulen as libc::c_ulong).wrapping_add(len) as length_t as length_t;
            clen = (clen as libc::c_ulong).wrapping_add((*(*job).out).len) as length_t
                as length_t;
            writen(g.outd, (*(*job).out).buf as *const libc::c_void, (*(*job).out).len);
            drop_space((*job).out);
            possess_(
                (*job).calc,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2007 as libc::c_long,
            );
            wait_for_(
                (*job).calc,
                TO_BE,
                1 as libc::c_long,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2008 as libc::c_long,
            );
            release_(
                (*job).calc,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2009 as libc::c_long,
            );
            if g.form == 1 as libc::c_int {
                tmp___7 = adler32_comb(check, (*job).check, len);
                check = tmp___7;
            } else {
                tmp___8 = crc32_comb(check, (*job).check, len);
                check = tmp___8;
            }
            free_lock_(
                (*job).calc,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2014 as libc::c_long,
            );
            free(job as *mut libc::c_void);
            seq += 1;
            if more == 0 {
                break;
            }
        }
        put_trailer(ulen, clen, check, head);
        possess_(
            compress_have,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            2025 as libc::c_long,
        );
        if compress_head as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            tmp___12 = peek_lock(compress_have);
            if !(tmp___12 == 0 as libc::c_long) {
                __assert_fail(
                    b"compress_head == NULL && peek_lock(compress_have) == 0\0"
                        as *const u8 as *const libc::c_char,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    2026 as libc::c_uint,
                    b"write_thread\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            __assert_fail(
                b"compress_head == NULL && peek_lock(compress_have) == 0\0" as *const u8
                    as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2026 as libc::c_uint,
                b"write_thread\0" as *const u8 as *const libc::c_char,
            );
        }
        release_(
            compress_have,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            2027 as libc::c_long,
        );
        possess_(
            write_first,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            2028 as libc::c_long,
        );
        if !(write_head as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
            __assert_fail(
                b"write_head == NULL\0" as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2029 as libc::c_uint,
                b"write_thread\0" as *const u8 as *const libc::c_char,
            );
        }
        twist_(
            write_first,
            TO,
            -(1 as libc::c_long),
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            2030 as libc::c_long,
        );
    }
    if try_pushed_ != 0 {
        tmp___18 = pthread_setspecific(try_key_, try_this_.next as *const libc::c_void);
        if tmp___18 == 0 as libc::c_int {
            if (b"try: pthread_setspecific() failed\0" as *const u8
                as *const libc::c_char)
                .is_null()
            {
                __assert_fail(
                    b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                        as *const u8 as *const libc::c_char,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    2032 as libc::c_uint,
                    b"write_thread\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            __assert_fail(
                b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2032 as libc::c_uint,
                b"write_thread\0" as *const u8 as *const libc::c_char,
            );
        }
        try_pushed_ = 0 as libc::c_int;
    }
    err = try_this_.ball;
    if err.code != 0 {
        if err.code != 32 as libc::c_int {
            complain(
                b"abort: %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                err.why,
            );
        }
        if err.free != 0 {
            free(err.why as *mut libc::c_void);
            err.free = 0 as libc::c_int;
            err.why = 0 as *mut libc::c_void as *mut libc::c_char;
        }
        cut_short(-err.code);
    }
}
unsafe extern "C" fn append_len(mut job: *mut job, mut len: size_t) {
    let mut lens: *mut space = 0 as *mut space;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: size_t = 0;
    let mut tmp___4: size_t = 0;
    let mut tmp___5: size_t = 0;
    let mut tmp___6: size_t = 0;
    let mut tmp___7: size_t = 0;
    let mut tmp___8: size_t = 0;
    let mut tmp___9: size_t = 0;
    if !(len < 539000896 as libc::c_ulong) {
        __assert_fail(
            b"len < 539000896UL\0" as *const u8 as *const libc::c_char,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            2041 as libc::c_uint,
            b"append_len\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*job).lens as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        (*job).lens = get_space(&mut lens_pool);
    }
    lens = (*job).lens;
    if (*lens).size < ((*lens).len).wrapping_add(3 as libc::c_ulong) {
        grow_space(lens);
    }
    if len < 64 as libc::c_ulong {
        tmp___0 = (*lens).len;
        (*lens).len = ((*lens).len).wrapping_add(1);
        *((*lens).buf)
            .offset(
                tmp___0 as isize,
            ) = len.wrapping_add(128 as libc::c_ulong) as libc::c_uchar;
    } else if len < 32832 as libc::c_ulong {
        len = (len as libc::c_ulong).wrapping_sub(64 as libc::c_ulong) as size_t
            as size_t;
        tmp___1 = (*lens).len;
        (*lens).len = ((*lens).len).wrapping_add(1);
        *((*lens).buf)
            .offset(tmp___1 as isize) = (len >> 8 as libc::c_int) as libc::c_uchar;
        tmp___2 = (*lens).len;
        (*lens).len = ((*lens).len).wrapping_add(1);
        *((*lens).buf).offset(tmp___2 as isize) = len as libc::c_uchar;
    } else if len < 2129984 as libc::c_ulong {
        len = (len as libc::c_ulong).wrapping_sub(32832 as libc::c_ulong) as size_t
            as size_t;
        tmp___3 = (*lens).len;
        (*lens).len = ((*lens).len).wrapping_add(1);
        *((*lens).buf)
            .offset(
                tmp___3 as isize,
            ) = (len >> 16 as libc::c_int).wrapping_add(192 as libc::c_ulong)
            as libc::c_uchar;
        tmp___4 = (*lens).len;
        (*lens).len = ((*lens).len).wrapping_add(1);
        *((*lens).buf)
            .offset(tmp___4 as isize) = (len >> 8 as libc::c_int) as libc::c_uchar;
        tmp___5 = (*lens).len;
        (*lens).len = ((*lens).len).wrapping_add(1);
        *((*lens).buf).offset(tmp___5 as isize) = len as libc::c_uchar;
    } else {
        len = (len as libc::c_ulong).wrapping_sub(2129984 as libc::c_ulong) as size_t
            as size_t;
        tmp___6 = (*lens).len;
        (*lens).len = ((*lens).len).wrapping_add(1);
        *((*lens).buf)
            .offset(
                tmp___6 as isize,
            ) = (len >> 24 as libc::c_int).wrapping_add(224 as libc::c_ulong)
            as libc::c_uchar;
        tmp___7 = (*lens).len;
        (*lens).len = ((*lens).len).wrapping_add(1);
        *((*lens).buf)
            .offset(tmp___7 as isize) = (len >> 16 as libc::c_int) as libc::c_uchar;
        tmp___8 = (*lens).len;
        (*lens).len = ((*lens).len).wrapping_add(1);
        *((*lens).buf)
            .offset(tmp___8 as isize) = (len >> 8 as libc::c_int) as libc::c_uchar;
        tmp___9 = (*lens).len;
        (*lens).len = ((*lens).len).wrapping_add(1);
        *((*lens).buf).offset(tmp___9 as isize) = len as libc::c_uchar;
    };
}
unsafe extern "C" fn parallel_compress() {
    let mut seq: libc::c_long = 0;
    let mut curr: *mut space = 0 as *mut space;
    let mut next___0: *mut space = 0 as *mut space;
    let mut hold: *mut space = 0 as *mut space;
    let mut dict: *mut space = 0 as *mut space;
    let mut job: *mut job = 0 as *mut job;
    let mut more: libc::c_int = 0;
    let mut hash: libc::c_uint = 0;
    let mut scan: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut last: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut left: size_t = 0;
    let mut len: size_t = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___1: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    setup_jobs();
    writeth = launch_(
        Some(write_thread as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        0 as *mut libc::c_void,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        2092 as libc::c_long,
    );
    seq = 0 as libc::c_long;
    next___0 = get_space(&mut in_pool);
    (*next___0).len = readn(g.ind, (*next___0).buf, (*next___0).size);
    hold = 0 as *mut libc::c_void as *mut space;
    dict = 0 as *mut libc::c_void as *mut space;
    scan = (*next___0).buf;
    hash = ((1 as libc::c_uint) << 12 as libc::c_int).wrapping_sub(1 as libc::c_uint)
        >> 1 as libc::c_int;
    left = 0 as libc::c_int as size_t;
    loop {
        tmp = alloc(
            0 as *mut libc::c_void,
            ::std::mem::size_of::<job>() as libc::c_ulong,
        );
        job = tmp as *mut job;
        (*job)
            .calc = new_lock_(
            0 as libc::c_long,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            2107 as libc::c_long,
        );
        curr = next___0;
        next___0 = hold;
        hold = 0 as *mut libc::c_void as *mut space;
        if next___0 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            next___0 = get_space(&mut in_pool);
            (*next___0).len = readn(g.ind, (*next___0).buf, (*next___0).size);
        }
        (*job).lens = 0 as *mut libc::c_void as *mut space;
        if g.rsync != 0 {
            if (*curr).len != 0 {
                if left == 0 as libc::c_ulong {
                    last = (*curr).buf;
                    end = ((*curr).buf).offset((*curr).len as isize);
                    while (scan as libc::c_ulong) < end as libc::c_ulong {
                        tmp___0 = scan;
                        scan = scan.offset(1);
                        hash = (hash << 1 as libc::c_int ^ *tmp___0 as libc::c_uint)
                            & ((1 as libc::c_uint) << 12 as libc::c_int)
                                .wrapping_sub(1 as libc::c_uint);
                        if hash
                            == ((1 as libc::c_uint) << 12 as libc::c_int)
                                .wrapping_sub(1 as libc::c_uint) >> 1 as libc::c_int
                        {
                            len = scan.offset_from(last) as libc::c_long as size_t;
                            append_len(job, len);
                            last = scan;
                        }
                    }
                    left = scan.offset_from(last) as libc::c_long as size_t;
                    scan = (*next___0).buf;
                }
                last = (*next___0).buf;
                len = ((*curr).size).wrapping_sub((*curr).len);
                if len > (*next___0).len {
                    len = (*next___0).len;
                }
                end = ((*next___0).buf).offset(len as isize);
                while (scan as libc::c_ulong) < end as libc::c_ulong {
                    tmp___1 = scan;
                    scan = scan.offset(1);
                    hash = (hash << 1 as libc::c_int ^ *tmp___1 as libc::c_uint)
                        & ((1 as libc::c_uint) << 12 as libc::c_int)
                            .wrapping_sub(1 as libc::c_uint);
                    if hash
                        == ((1 as libc::c_uint) << 12 as libc::c_int)
                            .wrapping_sub(1 as libc::c_uint) >> 1 as libc::c_int
                    {
                        len = (scan.offset_from(last) as libc::c_long as size_t)
                            .wrapping_add(left);
                        left = 0 as libc::c_int as size_t;
                        append_len(job, len);
                        last = scan;
                    }
                }
                append_len(job, 0 as libc::c_int as size_t);
                if (*(*job).lens).len == 1 as libc::c_ulong {
                    tmp___2 = scan;
                } else {
                    tmp___2 = last;
                }
                len = tmp___2.offset_from((*next___0).buf) as libc::c_long as size_t;
                if len != 0 {
                    memcpy(
                        ((*curr).buf).offset((*curr).len as isize) as *mut libc::c_void,
                        (*next___0).buf as *const libc::c_void,
                        len,
                    );
                    (*curr)
                        .len = ((*curr).len as libc::c_ulong).wrapping_add(len) as size_t
                        as size_t;
                    memmove(
                        (*next___0).buf as *mut libc::c_void,
                        ((*next___0).buf).offset(len as isize) as *const libc::c_void,
                        ((*next___0).len).wrapping_sub(len),
                    );
                    (*next___0)
                        .len = ((*next___0).len as libc::c_ulong).wrapping_sub(len)
                        as size_t as size_t;
                    scan = scan.offset(-(len as isize));
                    left = 0 as libc::c_int as size_t;
                } else if (*(*job).lens).len != 1 as libc::c_ulong {
                    if left != 0 {
                        if (*next___0).len != 0 {
                            hold = next___0;
                            next___0 = get_space(&mut in_pool);
                            memcpy(
                                (*next___0).buf as *mut libc::c_void,
                                ((*curr).buf)
                                    .offset(((*curr).len).wrapping_sub(left) as isize)
                                    as *const libc::c_void,
                                left,
                            );
                            (*next___0).len = left;
                            (*curr)
                                .len = ((*curr).len as libc::c_ulong).wrapping_sub(left)
                                as size_t as size_t;
                        } else {
                            left = 0 as libc::c_int as size_t;
                        }
                    } else {
                        left = 0 as libc::c_int as size_t;
                    }
                } else {
                    left = 0 as libc::c_int as size_t;
                }
            }
        }
        (*job).in_0 = curr;
        more = ((*next___0).len != 0 as libc::c_ulong) as libc::c_int;
        (*job).more = more;
        (*job).out = dict;
        if more != 0 {
            if g.setdict != 0 {
                if (*curr).len >= 32768 as libc::c_ulong {
                    dict = curr;
                    use_space(dict);
                } else if (*job).out as libc::c_ulong
                        == 0 as *mut libc::c_void as libc::c_ulong
                    {
                    dict = curr;
                    use_space(dict);
                } else {
                    dict = get_space(&mut dict_pool);
                    len = (32768 as libc::c_ulong).wrapping_sub((*curr).len);
                    memcpy(
                        (*dict).buf as *mut libc::c_void,
                        ((*(*job).out).buf)
                            .offset(((*(*job).out).len).wrapping_sub(len) as isize)
                            as *const libc::c_void,
                        len,
                    );
                    memcpy(
                        ((*dict).buf).offset(len as isize) as *mut libc::c_void,
                        (*curr).buf as *const libc::c_void,
                        (*curr).len,
                    );
                    (*dict).len = 32768 as libc::c_uint as size_t;
                }
            }
        }
        (*job).seq = seq;
        seq += 1;
        if seq < 1 as libc::c_long {
            try_throw_(
                34 as libc::c_int,
                b"overflow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *mut libc::c_void,
            );
        }
        if (cthreads as libc::c_long) < seq {
            if cthreads < g.procs {
                launch_(
                    Some(
                        compress_thread as unsafe extern "C" fn(*mut libc::c_void) -> (),
                    ),
                    0 as *mut libc::c_void,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    2228 as libc::c_long,
                );
                cthreads += 1;
            }
        }
        possess_(
            compress_have,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            2233 as libc::c_long,
        );
        (*job).next = 0 as *mut libc::c_void as *mut job;
        *compress_tail = job;
        compress_tail = &mut (*job).next;
        twist_(
            compress_have,
            BY,
            1 as libc::c_long,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            2237 as libc::c_long,
        );
        if more == 0 {
            break;
        }
    }
    drop_space(next___0);
    join_(
        writeth,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        2243 as libc::c_long,
    );
    writeth = 0 as *mut libc::c_void as *mut thread;
}
static mut out_size: libc::c_uint = 0;
static mut in_0: *mut libc::c_uchar = 0 as *const libc::c_uchar as *mut libc::c_uchar;
static mut next: *mut libc::c_uchar = 0 as *const libc::c_uchar as *mut libc::c_uchar;
static mut out: *mut libc::c_uchar = 0 as *const libc::c_uchar as *mut libc::c_uchar;
static mut strm: *mut z_stream = 0 as *const libc::c_void as *mut libc::c_void
    as *mut z_stream;
unsafe extern "C" fn single_compress(mut reset: libc::c_int) {
    let mut got: size_t = 0;
    let mut more: size_t = 0;
    let mut start: size_t = 0;
    let mut have: size_t = 0;
    let mut hist: size_t = 0;
    let mut fresh: libc::c_int = 0;
    let mut hash: libc::c_uint = 0;
    let mut scan: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut left: size_t = 0;
    let mut head: libc::c_ulong = 0;
    let mut ulen: length_t = 0;
    let mut clen: length_t = 0;
    let mut check: libc::c_ulong = 0;
    let mut ret: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: libc::c_ulong = 0;
    let mut tmp___4: libc::c_ulong = 0;
    let mut tmp___5: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___6: libc::c_ulong = 0;
    let mut tmp___7: libc::c_ulong = 0;
    let mut tmp___8: size_t = 0;
    let mut tmp___10: libc::c_ulong = 0;
    let mut tmp___11: libc::c_ulong = 0;
    let mut bits: libc::c_int = 0;
    let mut tmp___12: size_t = 0;
    let mut tmp___14: size_t = 0;
    let mut tmp___17: size_t = 0;
    let mut tmp___19: size_t = 0;
    let mut tmp___21: libc::c_long = 0;
    let mut tmp___22: size_t = 0;
    let mut tmp___24: size_t = 0;
    let mut bits___0: libc::c_uchar = 0;
    let mut def: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut size: size_t = 0;
    let mut off: size_t = 0;
    let mut tmp___26: libc::c_int = 0;
    let mut tmp___28: libc::c_ulong = 0;
    let mut tmp___29: libc::c_ulong = 0;
    let mut tmp___30: libc::c_ulong = 0;
    let mut tmp___31: libc::c_ulong = 0;
    if reset != 0 {
        if strm as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            deflateEnd(strm);
            free(strm as *mut libc::c_void);
            free(out as *mut libc::c_void);
            free(next as *mut libc::c_void);
            free(in_0 as *mut libc::c_void);
            strm = 0 as *mut libc::c_void as *mut z_stream;
        }
        return;
    }
    if strm as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        if g.block
            > (4294967295 as libc::c_uint)
                .wrapping_sub(4294967295 as libc::c_uint >> 1 as libc::c_int) as size_t
        {
            out_size = (4294967295 as libc::c_uint)
                .wrapping_sub(4294967295 as libc::c_uint >> 1 as libc::c_int);
        } else {
            out_size = g.block as libc::c_uint;
        }
        tmp = alloc(
            0 as *mut libc::c_void,
            (g.block).wrapping_add(32768 as libc::c_ulong),
        );
        in_0 = tmp as *mut libc::c_uchar;
        tmp___0 = alloc(
            0 as *mut libc::c_void,
            (g.block).wrapping_add(32768 as libc::c_ulong),
        );
        next = tmp___0 as *mut libc::c_uchar;
        tmp___1 = alloc(0 as *mut libc::c_void, out_size as size_t);
        out = tmp___1 as *mut libc::c_uchar;
        tmp___2 = alloc(
            0 as *mut libc::c_void,
            ::std::mem::size_of::<z_stream>() as libc::c_ulong,
        );
        strm = tmp___2 as *mut z_stream;
        (*strm).zfree = None;
        (*strm).zalloc = None;
        (*strm).opaque = 0 as voidpf;
        ret = deflateInit2_(
            strm,
            6 as libc::c_int,
            8 as libc::c_int,
            -(15 as libc::c_int),
            8 as libc::c_int,
            g.strategy,
            b"1.2.11\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
        );
        if ret == -(4 as libc::c_int) {
            try_throw_(
                12 as libc::c_int,
                b"not enough memory\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_void,
            );
        }
        if ret != 0 as libc::c_int {
            try_throw_(
                22 as libc::c_int,
                b"internal error\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_void,
            );
        }
    }
    head = put_header();
    if g.level <= 9 as libc::c_int {
        deflateReset(strm);
        deflateParams(strm, g.level, g.strategy);
    }
    got = 0 as libc::c_int as size_t;
    more = readn(g.ind, next, g.block);
    ulen = more;
    start = 0 as libc::c_int as size_t;
    hist = 0 as libc::c_int as size_t;
    clen = 0 as libc::c_int as length_t;
    have = 0 as libc::c_int as size_t;
    if g.form == 1 as libc::c_int {
        tmp___3 = adler32z(
            0 as libc::c_ulong,
            0 as *const libc::c_uchar,
            0 as libc::c_int as size_t,
        );
        check = tmp___3;
    } else {
        tmp___4 = crc32z(
            0 as libc::c_ulong,
            0 as *const libc::c_uchar,
            0 as libc::c_int as size_t,
        );
        check = tmp___4;
    }
    hash = ((1 as libc::c_uint) << 12 as libc::c_int).wrapping_sub(1 as libc::c_uint)
        >> 1 as libc::c_int;
    loop {
        if got == 0 as libc::c_ulong {
            scan = in_0;
            in_0 = next;
            next = scan;
            (*strm).next_in = in_0.offset(start as isize);
            got = more;
            if g.level > 9 as libc::c_int {
                left = start.wrapping_add(more).wrapping_sub(hist);
                if left > 32768 as libc::c_ulong {
                    left = 32768 as libc::c_uint as size_t;
                }
                memcpy(
                    next as *mut libc::c_void,
                    in_0.offset(start.wrapping_add(more).wrapping_sub(left) as isize)
                        as *const libc::c_void,
                    left,
                );
                start = left;
                hist = 0 as libc::c_int as size_t;
            } else {
                start = 0 as libc::c_int as size_t;
            }
            more = readn(g.ind, next.offset(start as isize), g.block);
            ulen = (ulen as libc::c_ulong).wrapping_add(more) as length_t as length_t;
        }
        left = 0 as libc::c_int as size_t;
        if g.rsync != 0 {
            if got != 0 {
                scan = (*strm).next_in;
                left = got;
                loop {
                    if left == 0 as libc::c_ulong {
                        if more == 0 as libc::c_ulong {
                            break;
                        }
                        if got == g.block {
                            break;
                        }
                        if g.level > 9 as libc::c_int {
                            left = (((*strm).next_in).offset_from(in_0) as libc::c_long
                                as size_t)
                                .wrapping_sub(hist);
                            if left > 32768 as libc::c_ulong {
                                left = 32768 as libc::c_uint as size_t;
                            }
                        }
                        memmove(
                            in_0 as *mut libc::c_void,
                            ((*strm).next_in).offset(-(left as isize))
                                as *const libc::c_void,
                            left.wrapping_add(got),
                        );
                        hist = 0 as libc::c_int as size_t;
                        (*strm).next_in = in_0.offset(left as isize);
                        scan = in_0.offset(left as isize).offset(got as isize);
                        if more > (g.block).wrapping_sub(got) {
                            left = (g.block).wrapping_sub(got);
                        } else {
                            left = more;
                        }
                        memcpy(
                            scan as *mut libc::c_void,
                            next.offset(start as isize) as *const libc::c_void,
                            left,
                        );
                        got = (got as libc::c_ulong).wrapping_add(left) as size_t
                            as size_t;
                        more = (more as libc::c_ulong).wrapping_sub(left) as size_t
                            as size_t;
                        start = (start as libc::c_ulong).wrapping_add(left) as size_t
                            as size_t;
                        if more == 0 as libc::c_ulong {
                            more = readn(g.ind, next, g.block);
                            ulen = (ulen as libc::c_ulong).wrapping_add(more) as length_t
                                as length_t;
                            start = 0 as libc::c_int as size_t;
                        }
                    }
                    left = left.wrapping_sub(1);
                    tmp___5 = scan;
                    scan = scan.offset(1);
                    hash = (hash << 1 as libc::c_int ^ *tmp___5 as libc::c_uint)
                        & ((1 as libc::c_uint) << 12 as libc::c_int)
                            .wrapping_sub(1 as libc::c_uint);
                    if !(hash
                        != ((1 as libc::c_uint) << 12 as libc::c_int)
                            .wrapping_sub(1 as libc::c_uint) >> 1 as libc::c_int)
                    {
                        break;
                    }
                }
                got = (got as libc::c_ulong).wrapping_sub(left) as size_t as size_t;
            }
        }
        fresh = 0 as libc::c_int;
        if g.setdict == 0 {
            have = (have as libc::c_ulong).wrapping_add(got) as size_t as size_t;
            if have > g.block {
                fresh = 1 as libc::c_int;
                have = got;
            }
        }
        if g.level <= 9 as libc::c_int {
            if fresh != 0 {
                deflateReset(strm);
            }
            while got
                > (4294967295 as libc::c_uint)
                    .wrapping_sub(4294967295 as libc::c_uint >> 1 as libc::c_int)
                    as size_t
            {
                (*strm)
                    .avail_in = (4294967295 as libc::c_uint)
                    .wrapping_sub(4294967295 as libc::c_uint >> 1 as libc::c_int);
                if g.form == 1 as libc::c_int {
                    tmp___6 = adler32z(
                        check,
                        (*strm).next_in as *const libc::c_uchar,
                        (*strm).avail_in as size_t,
                    );
                    check = tmp___6;
                } else {
                    tmp___7 = crc32z(
                        check,
                        (*strm).next_in as *const libc::c_uchar,
                        (*strm).avail_in as size_t,
                    );
                    check = tmp___7;
                }
                loop {
                    (*strm).avail_out = out_size;
                    (*strm).next_out = out;
                    deflate(strm, 0 as libc::c_int);
                    tmp___8 = writen(
                        g.outd,
                        out as *const libc::c_void,
                        out_size.wrapping_sub((*strm).avail_out) as size_t,
                    );
                    clen = (clen as libc::c_ulong).wrapping_add(tmp___8) as length_t
                        as length_t;
                    if !((*strm).avail_out == 0 as libc::c_uint) {
                        break;
                    }
                }
                if !((*strm).avail_in == 0 as libc::c_uint) {
                    __assert_fail(
                        b"strm->avail_in == 0\0" as *const u8 as *const libc::c_char,
                        b"pigz.c\0" as *const u8 as *const libc::c_char,
                        2421 as libc::c_uint,
                        b"single_compress\0" as *const u8 as *const libc::c_char,
                    );
                }
                got = (got as libc::c_ulong)
                    .wrapping_sub(
                        (4294967295 as libc::c_uint)
                            .wrapping_sub(4294967295 as libc::c_uint >> 1 as libc::c_int)
                            as size_t,
                    ) as size_t as size_t;
            }
            (*strm).avail_in = got as libc::c_uint;
            got = left;
            if g.form == 1 as libc::c_int {
                tmp___10 = adler32z(
                    check,
                    (*strm).next_in as *const libc::c_uchar,
                    (*strm).avail_in as size_t,
                );
                check = tmp___10;
            } else {
                tmp___11 = crc32z(
                    check,
                    (*strm).next_in as *const libc::c_uchar,
                    (*strm).avail_in as size_t,
                );
                check = tmp___11;
            }
            let mut current_block_230: u64;
            if more != 0 {
                current_block_230 = 13357164919558013274;
            } else if got != 0 {
                current_block_230 = 13357164919558013274;
            } else {
                loop {
                    (*strm).avail_out = out_size;
                    (*strm).next_out = out;
                    deflate(strm, 4 as libc::c_int);
                    tmp___24 = writen(
                        g.outd,
                        out as *const libc::c_void,
                        out_size.wrapping_sub((*strm).avail_out) as size_t,
                    );
                    clen = (clen as libc::c_ulong).wrapping_add(tmp___24) as length_t
                        as length_t;
                    if !((*strm).avail_out == 0 as libc::c_uint) {
                        break;
                    }
                }
                if !((*strm).avail_in == 0 as libc::c_uint) {
                    __assert_fail(
                        b"strm->avail_in == 0\0" as *const u8 as *const libc::c_char,
                        b"pigz.c\0" as *const u8 as *const libc::c_char,
                        2456 as libc::c_uint,
                        b"single_compress\0" as *const u8 as *const libc::c_char,
                    );
                }
                current_block_230 = 13198394165140872611;
            }
            match current_block_230 {
                13357164919558013274 => {
                    tmp___21 = zlib_vernum();
                    if tmp___21 >= 4704 as libc::c_long {
                        loop {
                            (*strm).avail_out = out_size;
                            (*strm).next_out = out;
                            deflate(strm, 5 as libc::c_int);
                            tmp___12 = writen(
                                g.outd,
                                out as *const libc::c_void,
                                out_size.wrapping_sub((*strm).avail_out) as size_t,
                            );
                            clen = (clen as libc::c_ulong).wrapping_add(tmp___12)
                                as length_t as length_t;
                            if !((*strm).avail_out == 0 as libc::c_uint) {
                                break;
                            }
                        }
                        if !((*strm).avail_in == 0 as libc::c_uint) {
                            __assert_fail(
                                b"strm->avail_in == 0\0" as *const u8
                                    as *const libc::c_char,
                                b"pigz.c\0" as *const u8 as *const libc::c_char,
                                2434 as libc::c_uint,
                                b"single_compress\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        deflatePending(strm, 0 as *mut libc::c_uint, &mut bits);
                        let mut current_block_193: u64;
                        if bits & 1 as libc::c_int != 0 {
                            current_block_193 = 2718244298171031309;
                        } else if g.setdict == 0 {
                            current_block_193 = 2718244298171031309;
                        } else {
                            if bits & 7 as libc::c_int != 0 {
                                loop {
                                    bits = deflatePrime(
                                        strm,
                                        10 as libc::c_int,
                                        2 as libc::c_int,
                                    );
                                    if !(bits == 0 as libc::c_int) {
                                        __assert_fail(
                                            b"bits == Z_OK\0" as *const u8 as *const libc::c_char,
                                            b"pigz.c\0" as *const u8 as *const libc::c_char,
                                            2441 as libc::c_uint,
                                            b"single_compress\0" as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    deflatePending(strm, 0 as *mut libc::c_uint, &mut bits);
                                    if bits & 7 as libc::c_int == 0 {
                                        break;
                                    }
                                }
                                loop {
                                    (*strm).avail_out = out_size;
                                    (*strm).next_out = out;
                                    deflate(strm, 0 as libc::c_int);
                                    tmp___17 = writen(
                                        g.outd,
                                        out as *const libc::c_void,
                                        out_size.wrapping_sub((*strm).avail_out) as size_t,
                                    );
                                    clen = (clen as libc::c_ulong).wrapping_add(tmp___17)
                                        as length_t as length_t;
                                    if !((*strm).avail_out == 0 as libc::c_uint) {
                                        break;
                                    }
                                }
                                if !((*strm).avail_in == 0 as libc::c_uint) {
                                    __assert_fail(
                                        b"strm->avail_in == 0\0" as *const u8
                                            as *const libc::c_char,
                                        b"pigz.c\0" as *const u8 as *const libc::c_char,
                                        2444 as libc::c_uint,
                                        b"single_compress\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                            }
                            current_block_193 = 1176253869785344635;
                        }
                        match current_block_193 {
                            2718244298171031309 => {
                                loop {
                                    (*strm).avail_out = out_size;
                                    (*strm).next_out = out;
                                    deflate(strm, 2 as libc::c_int);
                                    tmp___14 = writen(
                                        g.outd,
                                        out as *const libc::c_void,
                                        out_size.wrapping_sub((*strm).avail_out) as size_t,
                                    );
                                    clen = (clen as libc::c_ulong).wrapping_add(tmp___14)
                                        as length_t as length_t;
                                    if !((*strm).avail_out == 0 as libc::c_uint) {
                                        break;
                                    }
                                }
                                if !((*strm).avail_in == 0 as libc::c_uint) {
                                    __assert_fail(
                                        b"strm->avail_in == 0\0" as *const u8
                                            as *const libc::c_char,
                                        b"pigz.c\0" as *const u8 as *const libc::c_char,
                                        2437 as libc::c_uint,
                                        b"single_compress\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                            }
                            _ => {}
                        }
                    } else {
                        loop {
                            (*strm).avail_out = out_size;
                            (*strm).next_out = out;
                            deflate(strm, 2 as libc::c_int);
                            tmp___19 = writen(
                                g.outd,
                                out as *const libc::c_void,
                                out_size.wrapping_sub((*strm).avail_out) as size_t,
                            );
                            clen = (clen as libc::c_ulong).wrapping_add(tmp___19)
                                as length_t as length_t;
                            if !((*strm).avail_out == 0 as libc::c_uint) {
                                break;
                            }
                        }
                        if !((*strm).avail_in == 0 as libc::c_uint) {
                            __assert_fail(
                                b"strm->avail_in == 0\0" as *const u8
                                    as *const libc::c_char,
                                b"pigz.c\0" as *const u8 as *const libc::c_char,
                                2448 as libc::c_uint,
                                b"single_compress\0" as *const u8 as *const libc::c_char,
                            );
                        }
                    }
                    if g.setdict == 0 {
                        loop {
                            (*strm).avail_out = out_size;
                            (*strm).next_out = out;
                            deflate(strm, 3 as libc::c_int);
                            tmp___22 = writen(
                                g.outd,
                                out as *const libc::c_void,
                                out_size.wrapping_sub((*strm).avail_out) as size_t,
                            );
                            clen = (clen as libc::c_ulong).wrapping_add(tmp___22)
                                as length_t as length_t;
                            if !((*strm).avail_out == 0 as libc::c_uint) {
                                break;
                            }
                        }
                        if !((*strm).avail_in == 0 as libc::c_uint) {
                            __assert_fail(
                                b"strm->avail_in == 0\0" as *const u8
                                    as *const libc::c_char,
                                b"pigz.c\0" as *const u8 as *const libc::c_char,
                                2453 as libc::c_uint,
                                b"single_compress\0" as *const u8 as *const libc::c_char,
                            );
                        }
                    }
                }
                _ => {}
            }
        } else {
            off = ((*strm).next_in).offset_from(in_0) as libc::c_long as size_t;
            if fresh != 0 {
                hist = off;
            }
            def = 0 as *mut libc::c_void as *mut libc::c_uchar;
            size = 0 as libc::c_int as size_t;
            bits___0 = 0 as libc::c_int as libc::c_uchar;
            if more != 0 {
                tmp___26 = 0 as libc::c_int;
            } else if left != 0 {
                tmp___26 = 0 as libc::c_int;
            } else {
                tmp___26 = 1 as libc::c_int;
            }
            ZopfliDeflatePart(
                &mut g.zopts as *mut ZopfliOptions as *const ZopfliOptions,
                2 as libc::c_int,
                tmp___26,
                in_0.offset(hist as isize) as *const libc::c_uchar,
                off.wrapping_sub(hist),
                off.wrapping_sub(hist).wrapping_add(got),
                &mut bits___0,
                &mut def,
                &mut size,
            );
            bits___0 = (bits___0 as libc::c_int & 7 as libc::c_int) as libc::c_uchar;
            let mut current_block_276: u64;
            if more != 0 {
                current_block_276 = 12309578709166918009;
            } else if left != 0 {
                current_block_276 = 12309578709166918009;
            } else {
                writen(g.outd, def as *const libc::c_void, size);
                current_block_276 = 11475295656646479480;
            }
            match current_block_276 {
                12309578709166918009 => {
                    let mut current_block_270: u64;
                    if bits___0 as libc::c_int & 1 as libc::c_int != 0 {
                        current_block_270 = 4669928651309174885;
                    } else if g.setdict == 0 {
                        current_block_270 = 4669928651309174885;
                    } else {
                        if !(size > 0 as libc::c_ulong) {
                            __assert_fail(
                                b"size > 0\0" as *const u8 as *const libc::c_char,
                                b"pigz.c\0" as *const u8 as *const libc::c_char,
                                2484 as libc::c_uint,
                                b"single_compress\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        writen(
                            g.outd,
                            def as *const libc::c_void,
                            size.wrapping_sub(1 as libc::c_ulong),
                        );
                        if bits___0 != 0 {
                            loop {
                                *def
                                    .offset(
                                        size.wrapping_sub(1 as libc::c_ulong) as isize,
                                    ) = (*def
                                    .offset(size.wrapping_sub(1 as libc::c_ulong) as isize)
                                    as libc::c_int
                                    + ((2 as libc::c_int) << bits___0 as libc::c_int))
                                    as libc::c_uchar;
                                writen(
                                    g.outd,
                                    def
                                        .offset(size as isize)
                                        .offset(-(1 as libc::c_int as isize))
                                        as *const libc::c_void,
                                    1 as libc::c_int as size_t,
                                );
                                *def
                                    .offset(
                                        size.wrapping_sub(1 as libc::c_ulong) as isize,
                                    ) = 0 as libc::c_int as libc::c_uchar;
                                bits___0 = (bits___0 as libc::c_int + 2 as libc::c_int)
                                    as libc::c_uchar;
                                if !((bits___0 as libc::c_int) < 8 as libc::c_int) {
                                    break;
                                }
                            }
                        }
                        writen(
                            g.outd,
                            def
                                .offset(size as isize)
                                .offset(-(1 as libc::c_int as isize))
                                as *const libc::c_void,
                            1 as libc::c_int as size_t,
                        );
                        current_block_270 = 6785214093778810108;
                    }
                    match current_block_270 {
                        4669928651309174885 => {
                            writen(g.outd, def as *const libc::c_void, size);
                            if bits___0 as libc::c_int == 0 as libc::c_int {
                                writen(
                                    g.outd,
                                    b"\0\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_uchar as *const libc::c_void,
                                    1 as libc::c_int as size_t,
                                );
                            } else if bits___0 as libc::c_int > 5 as libc::c_int {
                                writen(
                                    g.outd,
                                    b"\0\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_uchar as *const libc::c_void,
                                    1 as libc::c_int as size_t,
                                );
                            }
                            writen(
                                g.outd,
                                b"\0\0\xFF\xFF\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_uchar as *const libc::c_void,
                                4 as libc::c_int as size_t,
                            );
                        }
                        _ => {}
                    }
                    if g.setdict == 0 {
                        writen(
                            g.outd,
                            b"\0\0\0\xFF\xFF\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_uchar as *const libc::c_void,
                            5 as libc::c_int as size_t,
                        );
                    }
                }
                _ => {}
            }
            free(def as *mut libc::c_void);
            while got
                > (4294967295 as libc::c_uint)
                    .wrapping_sub(4294967295 as libc::c_uint >> 1 as libc::c_int)
                    as size_t
            {
                if g.form == 1 as libc::c_int {
                    tmp___28 = adler32z(
                        check,
                        (*strm).next_in as *const libc::c_uchar,
                        (4294967295 as libc::c_uint)
                            .wrapping_sub(4294967295 as libc::c_uint >> 1 as libc::c_int)
                            as size_t,
                    );
                    check = tmp___28;
                } else {
                    tmp___29 = crc32z(
                        check,
                        (*strm).next_in as *const libc::c_uchar,
                        (4294967295 as libc::c_uint)
                            .wrapping_sub(4294967295 as libc::c_uint >> 1 as libc::c_int)
                            as size_t,
                    );
                    check = tmp___29;
                }
                (*strm)
                    .next_in = ((*strm).next_in)
                    .offset(
                        (4294967295 as libc::c_uint)
                            .wrapping_sub(4294967295 as libc::c_uint >> 1 as libc::c_int)
                            as isize,
                    );
                got = (got as libc::c_ulong)
                    .wrapping_sub(
                        (4294967295 as libc::c_uint)
                            .wrapping_sub(4294967295 as libc::c_uint >> 1 as libc::c_int)
                            as size_t,
                    ) as size_t as size_t;
            }
            if g.form == 1 as libc::c_int {
                tmp___30 = adler32z(
                    check,
                    (*strm).next_in as *const libc::c_uchar,
                    got as libc::c_uint as size_t,
                );
                check = tmp___30;
            } else {
                tmp___31 = crc32z(
                    check,
                    (*strm).next_in as *const libc::c_uchar,
                    got as libc::c_uint as size_t,
                );
                check = tmp___31;
            }
            (*strm).next_in = ((*strm).next_in).offset(got as isize);
            got = left;
        }
        if !(more == 0) {
            continue;
        }
        if got == 0 {
            break;
        }
    }
    put_trailer(ulen, clen, check, head);
}
unsafe extern "C" fn load_read(mut dummy: *mut libc::c_void) {
    let mut len: size_t = 0;
    let mut err: try_ball_t_ = try_ball_t_ {
        ret: 0,
        code: 0,
        free: 0,
        why: 0 as *mut libc::c_char,
    };
    let mut try_this_: try_t_ = try_t_ {
        env: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
        ball: try_ball_t_ {
            ret: 0,
            code: 0,
            free: 0,
            why: 0 as *mut libc::c_char,
        },
        next: 0 as *mut try_t_,
    };
    let mut try_pushed_: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_long = 0;
    let mut tmp___5: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    try_pushed_ = 1 as libc::c_int;
    try_this_.ball.ret = 0 as libc::c_int;
    try_this_.ball.code = 0 as libc::c_int;
    try_this_.ball.free = 0 as libc::c_int;
    try_this_.ball.why = 0 as *mut libc::c_void as *mut libc::c_char;
    try_setup_();
    tmp = pthread_getspecific(try_key_);
    try_this_.next = tmp as *mut try_t_;
    tmp___3 = pthread_setspecific(
        try_key_,
        &mut try_this_ as *mut try_t_ as *const libc::c_void,
    );
    if tmp___3 == 0 as libc::c_int {
        if (b"try: pthread_setspecific() failed\0" as *const u8 as *const libc::c_char)
            .is_null()
        {
            __assert_fail(
                b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2531 as libc::c_uint,
                b"load_read\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        __assert_fail(
            b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                as *const u8 as *const libc::c_char,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            2531 as libc::c_uint,
            b"load_read\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___6 = _setjmp((try_this_.env).as_mut_ptr());
    if tmp___6 == 0 as libc::c_int {
        loop {
            possess_(
                g.load_state,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2533 as libc::c_long,
            );
            wait_for_(
                g.load_state,
                NOT_TO_BE,
                0 as libc::c_long,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2534 as libc::c_long,
            );
            tmp___4 = peek_lock(g.load_state);
            if tmp___4 > 1 as libc::c_long {
                release_(
                    g.load_state,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    2536 as libc::c_long,
                );
                break;
            } else {
                if g.in_which != 0 {
                    tmp___5 = (g.in_buf).as_mut_ptr();
                } else {
                    tmp___5 = (g.in_buf2).as_mut_ptr();
                }
                len = readn(g.ind, tmp___5, 32768 as libc::c_int as size_t);
                g.in_len = len;
                twist_(
                    g.load_state,
                    TO,
                    0 as libc::c_long,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    2542 as libc::c_long,
                );
                if !(len == 32768 as libc::c_ulong) {
                    break;
                }
            }
        }
    }
    if try_pushed_ != 0 {
        tmp___10 = pthread_setspecific(try_key_, try_this_.next as *const libc::c_void);
        if tmp___10 == 0 as libc::c_int {
            if (b"try: pthread_setspecific() failed\0" as *const u8
                as *const libc::c_char)
                .is_null()
            {
                __assert_fail(
                    b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                        as *const u8 as *const libc::c_char,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    2545 as libc::c_uint,
                    b"load_read\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            __assert_fail(
                b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2545 as libc::c_uint,
                b"load_read\0" as *const u8 as *const libc::c_char,
            );
        }
        try_pushed_ = 0 as libc::c_int;
    }
    err = try_this_.ball;
    if err.code != 0 {
        if err.code != 32 as libc::c_int {
            complain(
                b"abort: %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                err.why,
            );
        }
        if err.free != 0 {
            free(err.why as *mut libc::c_void);
            err.free = 0 as libc::c_int;
            err.why = 0 as *mut libc::c_void as *mut libc::c_char;
        }
        cut_short(-err.code);
    }
}
unsafe extern "C" fn load_wait() {
    if g.in_which == -(1 as libc::c_int) {
        return;
    }
    possess_(
        g.load_state,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        2556 as libc::c_long,
    );
    wait_for_(
        g.load_state,
        TO_BE,
        0 as libc::c_long,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        2557 as libc::c_long,
    );
    release_(
        g.load_state,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        2558 as libc::c_long,
    );
}
unsafe extern "C" fn load() -> size_t {
    if g.in_short != 0 {
        g.in_eof = 1 as libc::c_int;
        g.in_left = 0 as libc::c_int as size_t;
        return 0 as libc::c_int as size_t;
    }
    if g.procs > 1 as libc::c_int {
        if g.in_which == -(1 as libc::c_int) {
            g.in_which = 1 as libc::c_int;
            g
                .load_state = new_lock_(
                1 as libc::c_long,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2582 as libc::c_long,
            );
            g
                .load_thread = launch_(
                Some(load_read as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                0 as *mut libc::c_void,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2583 as libc::c_long,
            );
        }
        load_wait();
        if g.in_which != 0 {
            g.in_next = (g.in_buf).as_mut_ptr();
        } else {
            g.in_next = (g.in_buf2).as_mut_ptr();
        }
        g.in_left = g.in_len;
        if g.in_len == 32768 as libc::c_ulong {
            g.in_which = 1 as libc::c_int - g.in_which;
            possess_(
                g.load_state,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2597 as libc::c_long,
            );
            twist_(
                g.load_state,
                TO,
                1 as libc::c_long,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2598 as libc::c_long,
            );
        } else {
            join_(
                g.load_thread,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2603 as libc::c_long,
            );
            free_lock_(
                g.load_state,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2604 as libc::c_long,
            );
            g.in_which = -(1 as libc::c_int);
        }
    } else {
        g.in_next = (g.in_buf).as_mut_ptr();
        g.in_left = readn(g.ind, g.in_next, 32768 as libc::c_int as size_t);
    }
    if g.in_left < 32768 as libc::c_ulong {
        g.in_short = 1 as libc::c_int;
        if g.in_left == 0 as libc::c_ulong {
            g.in_eof = 1 as libc::c_int;
        }
    }
    g
        .in_tot = (g.in_tot as libc::c_ulong).wrapping_add(g.in_left) as length_t
        as length_t;
    return g.in_left;
}
unsafe extern "C" fn load_end() {
    if g.in_which != -(1 as libc::c_int) {
        possess_(
            g.load_state,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            2637 as libc::c_long,
        );
        wait_for_(
            g.load_state,
            TO_BE,
            0 as libc::c_long,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            2638 as libc::c_long,
        );
        twist_(
            g.load_state,
            TO,
            2 as libc::c_long,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            2639 as libc::c_long,
        );
        join_(
            g.load_thread,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            2642 as libc::c_long,
        );
        free_lock_(
            g.load_state,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            2643 as libc::c_long,
        );
        g.in_which = -(1 as libc::c_int);
    }
    g.in_left = 0 as libc::c_int as size_t;
    g.in_short = 1 as libc::c_int;
    g.in_eof = 1 as libc::c_int;
    if g.ind != 0 as libc::c_int {
        close(g.ind);
    }
    if g.hname as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        free(g.hname as *mut libc::c_void);
        g.hname = 0 as *mut libc::c_void as *mut libc::c_char;
    }
    if g.hcomm as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        free(g.hcomm as *mut libc::c_void);
        g.hcomm = 0 as *mut libc::c_void as *mut libc::c_char;
    }
}
unsafe extern "C" fn in_init() {
    g.in_left = 0 as libc::c_int as size_t;
    g.in_eof = 0 as libc::c_int;
    g.in_short = 0 as libc::c_int;
    g.in_tot = 0 as libc::c_int as length_t;
    g.in_which = -(1 as libc::c_int);
}
unsafe extern "C" fn dos2time(mut dos: libc::c_ulong) -> time_t {
    let mut tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        __tm_gmtoff: 0,
        __tm_zone: 0 as *const libc::c_char,
    };
    let mut tmp: time_t = 0;
    let mut tmp___0: time_t = 0;
    if dos == 0 as libc::c_ulong {
        tmp = time(0 as *mut libc::c_void as *mut time_t);
        return tmp;
    }
    tm
        .tm_year = ((dos >> 25 as libc::c_int) as libc::c_int & 127 as libc::c_int)
        + 80 as libc::c_int;
    tm
        .tm_mon = ((dos >> 21 as libc::c_int) as libc::c_int & 15 as libc::c_int)
        - 1 as libc::c_int;
    tm.tm_mday = (dos >> 16 as libc::c_int) as libc::c_int & 31 as libc::c_int;
    tm.tm_hour = (dos >> 11 as libc::c_int) as libc::c_int & 31 as libc::c_int;
    tm.tm_min = (dos >> 5 as libc::c_int) as libc::c_int & 63 as libc::c_int;
    tm.tm_sec = (dos << 1 as libc::c_int) as libc::c_int & 62 as libc::c_int;
    tm.tm_isdst = -(1 as libc::c_int);
    tmp___0 = mktime(&mut tm);
    return tmp___0;
}
unsafe extern "C" fn tolong(mut val: libc::c_ulong) -> libc::c_long {
    return (val & 2147483647 as libc::c_ulong) as libc::c_long
        - (val & 2147483648 as libc::c_ulong) as libc::c_long;
}
unsafe extern "C" fn read_extra(
    mut len: libc::c_uint,
    mut save: libc::c_int,
) -> libc::c_int {
    let mut id: libc::c_uint = 0;
    let mut size: libc::c_uint = 0;
    let mut tmp2: libc::c_uint = 0;
    let mut tmp4: libc::c_ulong = 0;
    let mut tmp___0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___1: size_t = 0;
    let mut tmp___3: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: size_t = 0;
    let mut tmp___7: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___8: size_t = 0;
    let mut tmp___10: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: size_t = 0;
    let mut tmp___14: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___15: size_t = 0;
    let mut tmp___17: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___18: libc::c_int = 0;
    let mut tmp___19: size_t = 0;
    let mut tmp___21: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___22: size_t = 0;
    let mut tmp___24: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___25: libc::c_int = 0;
    let mut tmp___26: size_t = 0;
    let mut togo: size_t = 0;
    let mut tmp___27: size_t = 0;
    let mut tmp___29: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___30: size_t = 0;
    let mut tmp___32: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___33: libc::c_int = 0;
    let mut tmp___34: size_t = 0;
    let mut tmp___36: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___37: size_t = 0;
    let mut tmp___39: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___40: libc::c_int = 0;
    let mut tmp___41: size_t = 0;
    let mut togo___0: size_t = 0;
    let mut tmp___42: size_t = 0;
    let mut togo___1: size_t = 0;
    let mut tmp___43: size_t = 0;
    let mut tmp___45: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___46: size_t = 0;
    let mut tmp___48: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___49: libc::c_int = 0;
    let mut tmp___50: size_t = 0;
    let mut tmp___52: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___53: size_t = 0;
    let mut tmp___55: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___56: libc::c_int = 0;
    let mut tmp___57: size_t = 0;
    let mut tmp___59: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___60: size_t = 0;
    let mut tmp___62: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___63: libc::c_int = 0;
    let mut tmp___64: size_t = 0;
    let mut tmp___66: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___67: size_t = 0;
    let mut tmp___69: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___70: libc::c_int = 0;
    let mut tmp___71: size_t = 0;
    let mut tmp___73: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___74: libc::c_int = 0;
    let mut tmp___75: size_t = 0;
    let mut togo___2: size_t = 0;
    let mut tmp___76: size_t = 0;
    let mut togo___3: size_t = 0;
    let mut tmp___77: size_t = 0;
    while len >= 4 as libc::c_uint {
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp2 = 0 as libc::c_uint;
            } else {
                tmp___1 = load();
                if tmp___1 == 0 as libc::c_ulong {
                    tmp2 = 0 as libc::c_uint;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___0 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp2 = *tmp___0 as libc::c_uint;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___0 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp2 = *tmp___0 as libc::c_uint;
        }
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp___4 = 0 as libc::c_int;
            } else {
                tmp___5 = load();
                if tmp___5 == 0 as libc::c_ulong {
                    tmp___4 = 0 as libc::c_int;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___3 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp___4 = *tmp___3 as libc::c_int;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___3 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp___4 = *tmp___3 as libc::c_int;
        }
        id = tmp2.wrapping_add((tmp___4 as libc::c_uint) << 8 as libc::c_int);
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp2 = 0 as libc::c_uint;
            } else {
                tmp___8 = load();
                if tmp___8 == 0 as libc::c_ulong {
                    tmp2 = 0 as libc::c_uint;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___7 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp2 = *tmp___7 as libc::c_uint;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___7 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp2 = *tmp___7 as libc::c_uint;
        }
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp___11 = 0 as libc::c_int;
            } else {
                tmp___12 = load();
                if tmp___12 == 0 as libc::c_ulong {
                    tmp___11 = 0 as libc::c_int;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___10 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp___11 = *tmp___10 as libc::c_int;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___10 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp___11 = *tmp___10 as libc::c_int;
        }
        size = tmp2.wrapping_add((tmp___11 as libc::c_uint) << 8 as libc::c_int);
        if g.in_eof != 0 {
            return -(1 as libc::c_int);
        }
        len = len.wrapping_sub(4 as libc::c_uint);
        if size > len {
            break;
        }
        len = len.wrapping_sub(size);
        if id == 1 as libc::c_uint {
            g.zip64 = 1 as libc::c_int;
            if g.zip_ulen == 4294967295 as libc::c_ulong {
                if size >= 8 as libc::c_uint {
                    if g.in_left == 0 as libc::c_ulong {
                        if g.in_eof != 0 {
                            tmp2 = 0 as libc::c_uint;
                        } else {
                            tmp___15 = load();
                            if tmp___15 == 0 as libc::c_ulong {
                                tmp2 = 0 as libc::c_uint;
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                tmp___14 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                                tmp2 = *tmp___14 as libc::c_uint;
                            }
                        }
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___14 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp2 = *tmp___14 as libc::c_uint;
                    }
                    if g.in_left == 0 as libc::c_ulong {
                        if g.in_eof != 0 {
                            tmp___18 = 0 as libc::c_int;
                        } else {
                            tmp___19 = load();
                            if tmp___19 == 0 as libc::c_ulong {
                                tmp___18 = 0 as libc::c_int;
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                tmp___17 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                                tmp___18 = *tmp___17 as libc::c_int;
                            }
                        }
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___17 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp___18 = *tmp___17 as libc::c_int;
                    }
                    tmp4 = tmp2
                        .wrapping_add((tmp___18 as libc::c_uint) << 8 as libc::c_int)
                        as libc::c_ulong;
                    if g.in_left == 0 as libc::c_ulong {
                        if g.in_eof != 0 {
                            tmp2 = 0 as libc::c_uint;
                        } else {
                            tmp___22 = load();
                            if tmp___22 == 0 as libc::c_ulong {
                                tmp2 = 0 as libc::c_uint;
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                tmp___21 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                                tmp2 = *tmp___21 as libc::c_uint;
                            }
                        }
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___21 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp2 = *tmp___21 as libc::c_uint;
                    }
                    if g.in_left == 0 as libc::c_ulong {
                        if g.in_eof != 0 {
                            tmp___25 = 0 as libc::c_int;
                        } else {
                            tmp___26 = load();
                            if tmp___26 == 0 as libc::c_ulong {
                                tmp___25 = 0 as libc::c_int;
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                tmp___24 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                                tmp___25 = *tmp___24 as libc::c_int;
                            }
                        }
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___24 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp___25 = *tmp___24 as libc::c_int;
                    }
                    g
                        .zip_ulen = tmp4
                        .wrapping_add(
                            (tmp2
                                .wrapping_add(
                                    (tmp___25 as libc::c_uint) << 8 as libc::c_int,
                                ) as libc::c_ulong) << 16 as libc::c_int,
                        );
                    togo = 4 as libc::c_int as size_t;
                    while togo > g.in_left {
                        togo = (togo as libc::c_ulong).wrapping_sub(g.in_left) as size_t
                            as size_t;
                        tmp___27 = load();
                        if tmp___27 == 0 as libc::c_ulong {
                            return -(3 as libc::c_int);
                        }
                    }
                    g
                        .in_left = (g.in_left as libc::c_ulong).wrapping_sub(togo)
                        as size_t as size_t;
                    g.in_next = (g.in_next).offset(togo as isize);
                    size = size.wrapping_sub(8 as libc::c_uint);
                }
            }
            if g.zip_clen == 4294967295 as libc::c_ulong {
                if size >= 8 as libc::c_uint {
                    if g.in_left == 0 as libc::c_ulong {
                        if g.in_eof != 0 {
                            tmp2 = 0 as libc::c_uint;
                        } else {
                            tmp___30 = load();
                            if tmp___30 == 0 as libc::c_ulong {
                                tmp2 = 0 as libc::c_uint;
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                tmp___29 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                                tmp2 = *tmp___29 as libc::c_uint;
                            }
                        }
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___29 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp2 = *tmp___29 as libc::c_uint;
                    }
                    if g.in_left == 0 as libc::c_ulong {
                        if g.in_eof != 0 {
                            tmp___33 = 0 as libc::c_int;
                        } else {
                            tmp___34 = load();
                            if tmp___34 == 0 as libc::c_ulong {
                                tmp___33 = 0 as libc::c_int;
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                tmp___32 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                                tmp___33 = *tmp___32 as libc::c_int;
                            }
                        }
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___32 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp___33 = *tmp___32 as libc::c_int;
                    }
                    tmp4 = tmp2
                        .wrapping_add((tmp___33 as libc::c_uint) << 8 as libc::c_int)
                        as libc::c_ulong;
                    if g.in_left == 0 as libc::c_ulong {
                        if g.in_eof != 0 {
                            tmp2 = 0 as libc::c_uint;
                        } else {
                            tmp___37 = load();
                            if tmp___37 == 0 as libc::c_ulong {
                                tmp2 = 0 as libc::c_uint;
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                tmp___36 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                                tmp2 = *tmp___36 as libc::c_uint;
                            }
                        }
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___36 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp2 = *tmp___36 as libc::c_uint;
                    }
                    if g.in_left == 0 as libc::c_ulong {
                        if g.in_eof != 0 {
                            tmp___40 = 0 as libc::c_int;
                        } else {
                            tmp___41 = load();
                            if tmp___41 == 0 as libc::c_ulong {
                                tmp___40 = 0 as libc::c_int;
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                tmp___39 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                                tmp___40 = *tmp___39 as libc::c_int;
                            }
                        }
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___39 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp___40 = *tmp___39 as libc::c_int;
                    }
                    g
                        .zip_clen = tmp4
                        .wrapping_add(
                            (tmp2
                                .wrapping_add(
                                    (tmp___40 as libc::c_uint) << 8 as libc::c_int,
                                ) as libc::c_ulong) << 16 as libc::c_int,
                        );
                    togo___0 = 4 as libc::c_int as size_t;
                    while togo___0 > g.in_left {
                        togo___0 = (togo___0 as libc::c_ulong).wrapping_sub(g.in_left)
                            as size_t as size_t;
                        tmp___42 = load();
                        if tmp___42 == 0 as libc::c_ulong {
                            return -(3 as libc::c_int);
                        }
                    }
                    g
                        .in_left = (g.in_left as libc::c_ulong).wrapping_sub(togo___0)
                        as size_t as size_t;
                    g.in_next = (g.in_next).offset(togo___0 as isize);
                    size = size.wrapping_sub(8 as libc::c_uint);
                }
            }
        }
        if save != 0 {
            let mut current_block_381: u64;
            if id == 13 as libc::c_uint {
                current_block_381 = 3106107933516299137;
            } else if id == 22613 as libc::c_uint {
                current_block_381 = 3106107933516299137;
            } else {
                current_block_381 = 7803544491586723306;
            }
            match current_block_381 {
                3106107933516299137 => {
                    if size >= 8 as libc::c_uint {
                        togo___1 = 4 as libc::c_int as size_t;
                        while togo___1 > g.in_left {
                            togo___1 = (togo___1 as libc::c_ulong)
                                .wrapping_sub(g.in_left) as size_t as size_t;
                            tmp___43 = load();
                            if tmp___43 == 0 as libc::c_ulong {
                                return -(3 as libc::c_int);
                            }
                        }
                        g
                            .in_left = (g.in_left as libc::c_ulong)
                            .wrapping_sub(togo___1) as size_t as size_t;
                        g.in_next = (g.in_next).offset(togo___1 as isize);
                        if g.in_left == 0 as libc::c_ulong {
                            if g.in_eof != 0 {
                                tmp2 = 0 as libc::c_uint;
                            } else {
                                tmp___46 = load();
                                if tmp___46 == 0 as libc::c_ulong {
                                    tmp2 = 0 as libc::c_uint;
                                } else {
                                    g.in_left = (g.in_left).wrapping_sub(1);
                                    tmp___45 = g.in_next;
                                    g.in_next = (g.in_next).offset(1);
                                    tmp2 = *tmp___45 as libc::c_uint;
                                }
                            }
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            tmp___45 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            tmp2 = *tmp___45 as libc::c_uint;
                        }
                        if g.in_left == 0 as libc::c_ulong {
                            if g.in_eof != 0 {
                                tmp___49 = 0 as libc::c_int;
                            } else {
                                tmp___50 = load();
                                if tmp___50 == 0 as libc::c_ulong {
                                    tmp___49 = 0 as libc::c_int;
                                } else {
                                    g.in_left = (g.in_left).wrapping_sub(1);
                                    tmp___48 = g.in_next;
                                    g.in_next = (g.in_next).offset(1);
                                    tmp___49 = *tmp___48 as libc::c_int;
                                }
                            }
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            tmp___48 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            tmp___49 = *tmp___48 as libc::c_int;
                        }
                        tmp4 = tmp2
                            .wrapping_add((tmp___49 as libc::c_uint) << 8 as libc::c_int)
                            as libc::c_ulong;
                        if g.in_left == 0 as libc::c_ulong {
                            if g.in_eof != 0 {
                                tmp2 = 0 as libc::c_uint;
                            } else {
                                tmp___53 = load();
                                if tmp___53 == 0 as libc::c_ulong {
                                    tmp2 = 0 as libc::c_uint;
                                } else {
                                    g.in_left = (g.in_left).wrapping_sub(1);
                                    tmp___52 = g.in_next;
                                    g.in_next = (g.in_next).offset(1);
                                    tmp2 = *tmp___52 as libc::c_uint;
                                }
                            }
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            tmp___52 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            tmp2 = *tmp___52 as libc::c_uint;
                        }
                        if g.in_left == 0 as libc::c_ulong {
                            if g.in_eof != 0 {
                                tmp___56 = 0 as libc::c_int;
                            } else {
                                tmp___57 = load();
                                if tmp___57 == 0 as libc::c_ulong {
                                    tmp___56 = 0 as libc::c_int;
                                } else {
                                    g.in_left = (g.in_left).wrapping_sub(1);
                                    tmp___55 = g.in_next;
                                    g.in_next = (g.in_next).offset(1);
                                    tmp___56 = *tmp___55 as libc::c_int;
                                }
                            }
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            tmp___55 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            tmp___56 = *tmp___55 as libc::c_int;
                        }
                        g
                            .stamp = tolong(
                            tmp4
                                .wrapping_add(
                                    (tmp2
                                        .wrapping_add(
                                            (tmp___56 as libc::c_uint) << 8 as libc::c_int,
                                        ) as libc::c_ulong) << 16 as libc::c_int,
                                ),
                        );
                        size = size.wrapping_sub(8 as libc::c_uint);
                    }
                }
                _ => {}
            }
            if id == 21589 as libc::c_uint {
                if size >= 5 as libc::c_uint {
                    size = size.wrapping_sub(1);
                    if g.in_left == 0 as libc::c_ulong {
                        if g.in_eof != 0 {
                            tmp___74 = 0 as libc::c_int;
                        } else {
                            tmp___75 = load();
                            if tmp___75 == 0 as libc::c_ulong {
                                tmp___74 = 0 as libc::c_int;
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                tmp___73 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                                tmp___74 = *tmp___73 as libc::c_int;
                            }
                        }
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___73 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp___74 = *tmp___73 as libc::c_int;
                    }
                    if tmp___74 & 1 as libc::c_int != 0 {
                        if g.in_left == 0 as libc::c_ulong {
                            if g.in_eof != 0 {
                                tmp2 = 0 as libc::c_uint;
                            } else {
                                tmp___60 = load();
                                if tmp___60 == 0 as libc::c_ulong {
                                    tmp2 = 0 as libc::c_uint;
                                } else {
                                    g.in_left = (g.in_left).wrapping_sub(1);
                                    tmp___59 = g.in_next;
                                    g.in_next = (g.in_next).offset(1);
                                    tmp2 = *tmp___59 as libc::c_uint;
                                }
                            }
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            tmp___59 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            tmp2 = *tmp___59 as libc::c_uint;
                        }
                        if g.in_left == 0 as libc::c_ulong {
                            if g.in_eof != 0 {
                                tmp___63 = 0 as libc::c_int;
                            } else {
                                tmp___64 = load();
                                if tmp___64 == 0 as libc::c_ulong {
                                    tmp___63 = 0 as libc::c_int;
                                } else {
                                    g.in_left = (g.in_left).wrapping_sub(1);
                                    tmp___62 = g.in_next;
                                    g.in_next = (g.in_next).offset(1);
                                    tmp___63 = *tmp___62 as libc::c_int;
                                }
                            }
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            tmp___62 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            tmp___63 = *tmp___62 as libc::c_int;
                        }
                        tmp4 = tmp2
                            .wrapping_add((tmp___63 as libc::c_uint) << 8 as libc::c_int)
                            as libc::c_ulong;
                        if g.in_left == 0 as libc::c_ulong {
                            if g.in_eof != 0 {
                                tmp2 = 0 as libc::c_uint;
                            } else {
                                tmp___67 = load();
                                if tmp___67 == 0 as libc::c_ulong {
                                    tmp2 = 0 as libc::c_uint;
                                } else {
                                    g.in_left = (g.in_left).wrapping_sub(1);
                                    tmp___66 = g.in_next;
                                    g.in_next = (g.in_next).offset(1);
                                    tmp2 = *tmp___66 as libc::c_uint;
                                }
                            }
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            tmp___66 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            tmp2 = *tmp___66 as libc::c_uint;
                        }
                        if g.in_left == 0 as libc::c_ulong {
                            if g.in_eof != 0 {
                                tmp___70 = 0 as libc::c_int;
                            } else {
                                tmp___71 = load();
                                if tmp___71 == 0 as libc::c_ulong {
                                    tmp___70 = 0 as libc::c_int;
                                } else {
                                    g.in_left = (g.in_left).wrapping_sub(1);
                                    tmp___69 = g.in_next;
                                    g.in_next = (g.in_next).offset(1);
                                    tmp___70 = *tmp___69 as libc::c_int;
                                }
                            }
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            tmp___69 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            tmp___70 = *tmp___69 as libc::c_int;
                        }
                        g
                            .stamp = tolong(
                            tmp4
                                .wrapping_add(
                                    (tmp2
                                        .wrapping_add(
                                            (tmp___70 as libc::c_uint) << 8 as libc::c_int,
                                        ) as libc::c_ulong) << 16 as libc::c_int,
                                ),
                        );
                        size = size.wrapping_sub(4 as libc::c_uint);
                    }
                }
            }
        }
        togo___2 = size as size_t;
        while togo___2 > g.in_left {
            togo___2 = (togo___2 as libc::c_ulong).wrapping_sub(g.in_left) as size_t
                as size_t;
            tmp___76 = load();
            if tmp___76 == 0 as libc::c_ulong {
                return -(3 as libc::c_int);
            }
        }
        g
            .in_left = (g.in_left as libc::c_ulong).wrapping_sub(togo___2) as size_t
            as size_t;
        g.in_next = (g.in_next).offset(togo___2 as isize);
    }
    togo___3 = len as size_t;
    while togo___3 > g.in_left {
        togo___3 = (togo___3 as libc::c_ulong).wrapping_sub(g.in_left) as size_t
            as size_t;
        tmp___77 = load();
        if tmp___77 == 0 as libc::c_ulong {
            return -(3 as libc::c_int);
        }
    }
    g.in_left = (g.in_left as libc::c_ulong).wrapping_sub(togo___3) as size_t as size_t;
    g.in_next = (g.in_next).offset(togo___3 as isize);
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_header(mut save: libc::c_int) -> libc::c_int {
    let mut magic: libc::c_uint = 0;
    let mut method: libc::c_uint = 0;
    let mut flags: libc::c_uint = 0;
    let mut fname: libc::c_uint = 0;
    let mut extra: libc::c_uint = 0;
    let mut tmp2: libc::c_uint = 0;
    let mut tmp4: libc::c_ulong = 0;
    let mut crc: libc::c_ulong = 0;
    let mut tmp___0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___1: size_t = 0;
    let mut tmp___3: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: size_t = 0;
    let mut tmp___7: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___8: size_t = 0;
    let mut tmp___10: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: size_t = 0;
    let mut togo: size_t = 0;
    let mut tmp___13: size_t = 0;
    let mut tmp___15: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___16: size_t = 0;
    let mut tmp___18: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___19: libc::c_int = 0;
    let mut tmp___20: size_t = 0;
    let mut tmp___22: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___23: size_t = 0;
    let mut tmp___25: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___26: libc::c_int = 0;
    let mut tmp___27: size_t = 0;
    let mut tmp___29: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___30: size_t = 0;
    let mut tmp___32: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___33: libc::c_int = 0;
    let mut tmp___34: size_t = 0;
    let mut tmp___36: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___37: size_t = 0;
    let mut tmp___39: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___40: libc::c_int = 0;
    let mut tmp___41: size_t = 0;
    let mut togo___0: size_t = 0;
    let mut tmp___42: size_t = 0;
    let mut tmp___44: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___45: size_t = 0;
    let mut tmp___47: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___48: libc::c_int = 0;
    let mut tmp___49: size_t = 0;
    let mut tmp___51: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___52: size_t = 0;
    let mut tmp___54: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___55: libc::c_int = 0;
    let mut tmp___56: size_t = 0;
    let mut tmp___58: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___59: size_t = 0;
    let mut tmp___61: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___62: libc::c_int = 0;
    let mut tmp___63: size_t = 0;
    let mut tmp___65: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___66: size_t = 0;
    let mut tmp___68: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___69: libc::c_int = 0;
    let mut tmp___70: size_t = 0;
    let mut tmp___72: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___73: size_t = 0;
    let mut tmp___75: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___76: libc::c_int = 0;
    let mut tmp___77: size_t = 0;
    let mut tmp___79: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___80: size_t = 0;
    let mut tmp___82: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___83: libc::c_int = 0;
    let mut tmp___84: size_t = 0;
    let mut tmp___86: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___87: size_t = 0;
    let mut tmp___89: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___90: libc::c_int = 0;
    let mut tmp___91: size_t = 0;
    let mut tmp___93: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___94: size_t = 0;
    let mut tmp___96: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___97: libc::c_int = 0;
    let mut tmp___98: size_t = 0;
    let mut next___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___99: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___100: size_t = 0;
    let mut togo___1: size_t = 0;
    let mut tmp___101: size_t = 0;
    let mut tmp___102: libc::c_int = 0;
    let mut tmp___104: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___105: size_t = 0;
    let mut tmp___107: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___108: size_t = 0;
    let mut tmp___110: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___111: size_t = 0;
    let mut tmp___113: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___114: libc::c_int = 0;
    let mut tmp___115: size_t = 0;
    let mut tmp___117: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___118: size_t = 0;
    let mut tmp___120: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___121: libc::c_int = 0;
    let mut tmp___122: size_t = 0;
    let mut togo___2: size_t = 0;
    let mut tmp___123: size_t = 0;
    let mut togo___3: size_t = 0;
    let mut tmp___124: size_t = 0;
    let mut togo___4: size_t = 0;
    let mut tmp___126: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___127: size_t = 0;
    let mut tmp___129: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___130: libc::c_int = 0;
    let mut tmp___131: size_t = 0;
    let mut tmp___132: size_t = 0;
    let mut end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut copy: size_t = 0;
    let mut have: size_t = 0;
    let mut size: size_t = 0;
    let mut tmp___133: size_t = 0;
    let mut tmp___134: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___136: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___137: libc::c_int = 0;
    let mut tmp___138: size_t = 0;
    let mut end___0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut copy___0: size_t = 0;
    let mut have___0: size_t = 0;
    let mut size___0: size_t = 0;
    let mut tmp___139: size_t = 0;
    let mut tmp___140: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___142: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___143: libc::c_int = 0;
    let mut tmp___144: size_t = 0;
    let mut tmp___146: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___147: size_t = 0;
    let mut tmp___149: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___150: libc::c_int = 0;
    let mut tmp___151: size_t = 0;
    let mut tmp___152: libc::c_int = 0;
    if save != 0 {
        g.stamp = 0 as libc::c_int as time_t;
        if g.hname as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            free(g.hname as *mut libc::c_void);
            g.hname = 0 as *mut libc::c_void as *mut libc::c_char;
        }
        if g.hcomm as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            free(g.hcomm as *mut libc::c_void);
            g.hcomm = 0 as *mut libc::c_void as *mut libc::c_char;
        }
    }
    if g.in_left == 0 as libc::c_ulong {
        if g.in_eof != 0 {
            g.magic1 = 0 as libc::c_int;
        } else {
            tmp___1 = load();
            if tmp___1 == 0 as libc::c_ulong {
                g.magic1 = 0 as libc::c_int;
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                tmp___0 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                g.magic1 = *tmp___0 as libc::c_int;
            }
        }
    } else {
        g.in_left = (g.in_left).wrapping_sub(1);
        tmp___0 = g.in_next;
        g.in_next = (g.in_next).offset(1);
        g.magic1 = *tmp___0 as libc::c_int;
    }
    if g.in_eof != 0 {
        g.magic1 = -(1 as libc::c_int);
        return -(1 as libc::c_int);
    }
    magic = (g.magic1 as libc::c_uint) << 8 as libc::c_int;
    if g.in_left == 0 as libc::c_ulong {
        if g.in_eof != 0 {
            tmp___4 = 0 as libc::c_int;
        } else {
            tmp___5 = load();
            if tmp___5 == 0 as libc::c_ulong {
                tmp___4 = 0 as libc::c_int;
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                tmp___3 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                tmp___4 = *tmp___3 as libc::c_int;
            }
        }
    } else {
        g.in_left = (g.in_left).wrapping_sub(1);
        tmp___3 = g.in_next;
        g.in_next = (g.in_next).offset(1);
        tmp___4 = *tmp___3 as libc::c_int;
    }
    magic = magic.wrapping_add(tmp___4 as libc::c_uint);
    if g.in_eof != 0 {
        return -(2 as libc::c_int);
    }
    if magic.wrapping_rem(31 as libc::c_uint) == 0 as libc::c_uint {
        if magic & 36640 as libc::c_uint == 2048 as libc::c_uint {
            g.form = 1 as libc::c_int;
            return 8 as libc::c_int;
        }
    }
    if magic == 8093 as libc::c_uint {
        g.form = -(1 as libc::c_int);
        return 257 as libc::c_int;
    }
    if magic == 20555 as libc::c_uint {
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp2 = 0 as libc::c_uint;
            } else {
                tmp___8 = load();
                if tmp___8 == 0 as libc::c_ulong {
                    tmp2 = 0 as libc::c_uint;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___7 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp2 = *tmp___7 as libc::c_uint;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___7 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp2 = *tmp___7 as libc::c_uint;
        }
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp___11 = 0 as libc::c_int;
            } else {
                tmp___12 = load();
                if tmp___12 == 0 as libc::c_ulong {
                    tmp___11 = 0 as libc::c_int;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___10 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp___11 = *tmp___10 as libc::c_int;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___10 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp___11 = *tmp___10 as libc::c_int;
        }
        magic = tmp2.wrapping_add((tmp___11 as libc::c_uint) << 8 as libc::c_int);
        if g.in_eof != 0 {
            return -(3 as libc::c_int);
        }
        if magic == 513 as libc::c_uint {
            return -(5 as libc::c_int)
        } else {
            if magic == 2054 as libc::c_uint {
                return -(5 as libc::c_int);
            }
        }
        if magic != 1027 as libc::c_uint {
            return -(4 as libc::c_int);
        }
        g.zip64 = 0 as libc::c_int;
        togo = 2 as libc::c_int as size_t;
        while togo > g.in_left {
            togo = (togo as libc::c_ulong).wrapping_sub(g.in_left) as size_t as size_t;
            tmp___13 = load();
            if tmp___13 == 0 as libc::c_ulong {
                return -(3 as libc::c_int);
            }
        }
        g.in_left = (g.in_left as libc::c_ulong).wrapping_sub(togo) as size_t as size_t;
        g.in_next = (g.in_next).offset(togo as isize);
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp2 = 0 as libc::c_uint;
            } else {
                tmp___16 = load();
                if tmp___16 == 0 as libc::c_ulong {
                    tmp2 = 0 as libc::c_uint;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___15 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp2 = *tmp___15 as libc::c_uint;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___15 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp2 = *tmp___15 as libc::c_uint;
        }
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp___19 = 0 as libc::c_int;
            } else {
                tmp___20 = load();
                if tmp___20 == 0 as libc::c_ulong {
                    tmp___19 = 0 as libc::c_int;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___18 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp___19 = *tmp___18 as libc::c_int;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___18 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp___19 = *tmp___18 as libc::c_int;
        }
        flags = tmp2.wrapping_add((tmp___19 as libc::c_uint) << 8 as libc::c_int);
        if flags & 63472 as libc::c_uint != 0 {
            return -(4 as libc::c_int);
        }
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                method = 0 as libc::c_uint;
            } else {
                tmp___23 = load();
                if tmp___23 == 0 as libc::c_ulong {
                    method = 0 as libc::c_uint;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___22 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    method = *tmp___22 as libc::c_uint;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___22 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            method = *tmp___22 as libc::c_uint;
        }
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp___26 = 0 as libc::c_int;
            } else {
                tmp___27 = load();
                if tmp___27 == 0 as libc::c_ulong {
                    tmp___26 = 0 as libc::c_int;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___25 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp___26 = *tmp___25 as libc::c_int;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___25 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp___26 = *tmp___25 as libc::c_int;
        }
        if tmp___26 != 0 as libc::c_int {
            method = 256 as libc::c_uint;
        } else if flags & 1 as libc::c_uint != 0 {
            method = 256 as libc::c_uint;
        }
        if save != 0 {
            if g.in_left == 0 as libc::c_ulong {
                if g.in_eof != 0 {
                    tmp2 = 0 as libc::c_uint;
                } else {
                    tmp___30 = load();
                    if tmp___30 == 0 as libc::c_ulong {
                        tmp2 = 0 as libc::c_uint;
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___29 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp2 = *tmp___29 as libc::c_uint;
                    }
                }
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                tmp___29 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                tmp2 = *tmp___29 as libc::c_uint;
            }
            if g.in_left == 0 as libc::c_ulong {
                if g.in_eof != 0 {
                    tmp___33 = 0 as libc::c_int;
                } else {
                    tmp___34 = load();
                    if tmp___34 == 0 as libc::c_ulong {
                        tmp___33 = 0 as libc::c_int;
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___32 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp___33 = *tmp___32 as libc::c_int;
                    }
                }
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                tmp___32 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                tmp___33 = *tmp___32 as libc::c_int;
            }
            tmp4 = tmp2.wrapping_add((tmp___33 as libc::c_uint) << 8 as libc::c_int)
                as libc::c_ulong;
            if g.in_left == 0 as libc::c_ulong {
                if g.in_eof != 0 {
                    tmp2 = 0 as libc::c_uint;
                } else {
                    tmp___37 = load();
                    if tmp___37 == 0 as libc::c_ulong {
                        tmp2 = 0 as libc::c_uint;
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___36 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp2 = *tmp___36 as libc::c_uint;
                    }
                }
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                tmp___36 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                tmp2 = *tmp___36 as libc::c_uint;
            }
            if g.in_left == 0 as libc::c_ulong {
                if g.in_eof != 0 {
                    tmp___40 = 0 as libc::c_int;
                } else {
                    tmp___41 = load();
                    if tmp___41 == 0 as libc::c_ulong {
                        tmp___40 = 0 as libc::c_int;
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___39 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp___40 = *tmp___39 as libc::c_int;
                    }
                }
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                tmp___39 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                tmp___40 = *tmp___39 as libc::c_int;
            }
            g
                .stamp = dos2time(
                tmp4
                    .wrapping_add(
                        (tmp2
                            .wrapping_add((tmp___40 as libc::c_uint) << 8 as libc::c_int)
                            as libc::c_ulong) << 16 as libc::c_int,
                    ),
            );
        } else {
            togo___0 = 4 as libc::c_int as size_t;
            while togo___0 > g.in_left {
                togo___0 = (togo___0 as libc::c_ulong).wrapping_sub(g.in_left) as size_t
                    as size_t;
                tmp___42 = load();
                if tmp___42 == 0 as libc::c_ulong {
                    return -(3 as libc::c_int);
                }
            }
            g
                .in_left = (g.in_left as libc::c_ulong).wrapping_sub(togo___0) as size_t
                as size_t;
            g.in_next = (g.in_next).offset(togo___0 as isize);
        }
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp2 = 0 as libc::c_uint;
            } else {
                tmp___45 = load();
                if tmp___45 == 0 as libc::c_ulong {
                    tmp2 = 0 as libc::c_uint;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___44 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp2 = *tmp___44 as libc::c_uint;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___44 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp2 = *tmp___44 as libc::c_uint;
        }
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp___48 = 0 as libc::c_int;
            } else {
                tmp___49 = load();
                if tmp___49 == 0 as libc::c_ulong {
                    tmp___48 = 0 as libc::c_int;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___47 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp___48 = *tmp___47 as libc::c_int;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___47 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp___48 = *tmp___47 as libc::c_int;
        }
        tmp4 = tmp2.wrapping_add((tmp___48 as libc::c_uint) << 8 as libc::c_int)
            as libc::c_ulong;
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp2 = 0 as libc::c_uint;
            } else {
                tmp___52 = load();
                if tmp___52 == 0 as libc::c_ulong {
                    tmp2 = 0 as libc::c_uint;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___51 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp2 = *tmp___51 as libc::c_uint;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___51 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp2 = *tmp___51 as libc::c_uint;
        }
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp___55 = 0 as libc::c_int;
            } else {
                tmp___56 = load();
                if tmp___56 == 0 as libc::c_ulong {
                    tmp___55 = 0 as libc::c_int;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___54 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp___55 = *tmp___54 as libc::c_int;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___54 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp___55 = *tmp___54 as libc::c_int;
        }
        g
            .zip_crc = tmp4
            .wrapping_add(
                (tmp2.wrapping_add((tmp___55 as libc::c_uint) << 8 as libc::c_int)
                    as libc::c_ulong) << 16 as libc::c_int,
            );
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp2 = 0 as libc::c_uint;
            } else {
                tmp___59 = load();
                if tmp___59 == 0 as libc::c_ulong {
                    tmp2 = 0 as libc::c_uint;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___58 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp2 = *tmp___58 as libc::c_uint;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___58 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp2 = *tmp___58 as libc::c_uint;
        }
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp___62 = 0 as libc::c_int;
            } else {
                tmp___63 = load();
                if tmp___63 == 0 as libc::c_ulong {
                    tmp___62 = 0 as libc::c_int;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___61 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp___62 = *tmp___61 as libc::c_int;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___61 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp___62 = *tmp___61 as libc::c_int;
        }
        tmp4 = tmp2.wrapping_add((tmp___62 as libc::c_uint) << 8 as libc::c_int)
            as libc::c_ulong;
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp2 = 0 as libc::c_uint;
            } else {
                tmp___66 = load();
                if tmp___66 == 0 as libc::c_ulong {
                    tmp2 = 0 as libc::c_uint;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___65 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp2 = *tmp___65 as libc::c_uint;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___65 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp2 = *tmp___65 as libc::c_uint;
        }
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp___69 = 0 as libc::c_int;
            } else {
                tmp___70 = load();
                if tmp___70 == 0 as libc::c_ulong {
                    tmp___69 = 0 as libc::c_int;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___68 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp___69 = *tmp___68 as libc::c_int;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___68 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp___69 = *tmp___68 as libc::c_int;
        }
        g
            .zip_clen = tmp4
            .wrapping_add(
                (tmp2.wrapping_add((tmp___69 as libc::c_uint) << 8 as libc::c_int)
                    as libc::c_ulong) << 16 as libc::c_int,
            );
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp2 = 0 as libc::c_uint;
            } else {
                tmp___73 = load();
                if tmp___73 == 0 as libc::c_ulong {
                    tmp2 = 0 as libc::c_uint;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___72 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp2 = *tmp___72 as libc::c_uint;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___72 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp2 = *tmp___72 as libc::c_uint;
        }
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp___76 = 0 as libc::c_int;
            } else {
                tmp___77 = load();
                if tmp___77 == 0 as libc::c_ulong {
                    tmp___76 = 0 as libc::c_int;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___75 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp___76 = *tmp___75 as libc::c_int;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___75 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp___76 = *tmp___75 as libc::c_int;
        }
        tmp4 = tmp2.wrapping_add((tmp___76 as libc::c_uint) << 8 as libc::c_int)
            as libc::c_ulong;
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp2 = 0 as libc::c_uint;
            } else {
                tmp___80 = load();
                if tmp___80 == 0 as libc::c_ulong {
                    tmp2 = 0 as libc::c_uint;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___79 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp2 = *tmp___79 as libc::c_uint;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___79 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp2 = *tmp___79 as libc::c_uint;
        }
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp___83 = 0 as libc::c_int;
            } else {
                tmp___84 = load();
                if tmp___84 == 0 as libc::c_ulong {
                    tmp___83 = 0 as libc::c_int;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___82 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp___83 = *tmp___82 as libc::c_int;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___82 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp___83 = *tmp___82 as libc::c_int;
        }
        g
            .zip_ulen = tmp4
            .wrapping_add(
                (tmp2.wrapping_add((tmp___83 as libc::c_uint) << 8 as libc::c_int)
                    as libc::c_ulong) << 16 as libc::c_int,
            );
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp2 = 0 as libc::c_uint;
            } else {
                tmp___87 = load();
                if tmp___87 == 0 as libc::c_ulong {
                    tmp2 = 0 as libc::c_uint;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___86 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp2 = *tmp___86 as libc::c_uint;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___86 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp2 = *tmp___86 as libc::c_uint;
        }
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp___90 = 0 as libc::c_int;
            } else {
                tmp___91 = load();
                if tmp___91 == 0 as libc::c_ulong {
                    tmp___90 = 0 as libc::c_int;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___89 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp___90 = *tmp___89 as libc::c_int;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___89 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp___90 = *tmp___89 as libc::c_int;
        }
        fname = tmp2.wrapping_add((tmp___90 as libc::c_uint) << 8 as libc::c_int);
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp2 = 0 as libc::c_uint;
            } else {
                tmp___94 = load();
                if tmp___94 == 0 as libc::c_ulong {
                    tmp2 = 0 as libc::c_uint;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___93 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp2 = *tmp___93 as libc::c_uint;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___93 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp2 = *tmp___93 as libc::c_uint;
        }
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp___97 = 0 as libc::c_int;
            } else {
                tmp___98 = load();
                if tmp___98 == 0 as libc::c_ulong {
                    tmp___97 = 0 as libc::c_int;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___96 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp___97 = *tmp___96 as libc::c_int;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___96 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp___97 = *tmp___96 as libc::c_int;
        }
        extra = tmp2.wrapping_add((tmp___97 as libc::c_uint) << 8 as libc::c_int);
        if save != 0 {
            if g.in_eof != 0 {
                return -(3 as libc::c_int);
            }
            tmp___99 = alloc(
                0 as *mut libc::c_void,
                fname.wrapping_add(1 as libc::c_uint) as size_t,
            );
            g.hname = tmp___99 as *mut libc::c_char;
            next___0 = g.hname;
            while fname as size_t > g.in_left {
                memcpy(
                    next___0 as *mut libc::c_void,
                    g.in_next as *const libc::c_void,
                    g.in_left,
                );
                fname = (fname as size_t).wrapping_sub(g.in_left) as libc::c_uint;
                next___0 = next___0.offset(g.in_left as isize);
                tmp___100 = load();
                if tmp___100 == 0 as libc::c_ulong {
                    return -(3 as libc::c_int);
                }
            }
            memcpy(
                next___0 as *mut libc::c_void,
                g.in_next as *const libc::c_void,
                fname as size_t,
            );
            g
                .in_left = (g.in_left as libc::c_ulong).wrapping_sub(fname as size_t)
                as size_t as size_t;
            g.in_next = (g.in_next).offset(fname as isize);
            next___0 = next___0.offset(fname as isize);
            *next___0 = 0 as libc::c_int as libc::c_char;
        } else {
            togo___1 = fname as size_t;
            while togo___1 > g.in_left {
                togo___1 = (togo___1 as libc::c_ulong).wrapping_sub(g.in_left) as size_t
                    as size_t;
                tmp___101 = load();
                if tmp___101 == 0 as libc::c_ulong {
                    return -(3 as libc::c_int);
                }
            }
            g
                .in_left = (g.in_left as libc::c_ulong).wrapping_sub(togo___1) as size_t
                as size_t;
            g.in_next = (g.in_next).offset(togo___1 as isize);
        }
        read_extra(extra, save);
        g
            .form = (2 as libc::c_uint)
            .wrapping_add((flags & 8 as libc::c_uint) >> 3 as libc::c_int)
            as libc::c_int;
        if g.in_eof != 0 {
            tmp___102 = -(3 as libc::c_int);
        } else {
            tmp___102 = method as libc::c_int;
        }
        return tmp___102;
    }
    if magic != 8075 as libc::c_uint {
        g.in_left = (g.in_left).wrapping_add(1);
        g.in_next = (g.in_next).offset(-1);
        return -(2 as libc::c_int);
    }
    crc = 4142483145 as libc::c_ulong;
    if g.in_left == 0 as libc::c_ulong {
        if g.in_eof != 0 {
            method = 0 as libc::c_uint;
        } else {
            tmp___105 = load();
            if tmp___105 == 0 as libc::c_ulong {
                method = 0 as libc::c_uint;
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                crc = crc32z(
                    crc,
                    g.in_next as *const libc::c_uchar,
                    1 as libc::c_int as size_t,
                );
                tmp___104 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                method = *tmp___104 as libc::c_uint;
            }
        }
    } else {
        g.in_left = (g.in_left).wrapping_sub(1);
        crc = crc32z(crc, g.in_next as *const libc::c_uchar, 1 as libc::c_int as size_t);
        tmp___104 = g.in_next;
        g.in_next = (g.in_next).offset(1);
        method = *tmp___104 as libc::c_uint;
    }
    if g.in_left == 0 as libc::c_ulong {
        if g.in_eof != 0 {
            flags = 0 as libc::c_uint;
        } else {
            tmp___108 = load();
            if tmp___108 == 0 as libc::c_ulong {
                flags = 0 as libc::c_uint;
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                crc = crc32z(
                    crc,
                    g.in_next as *const libc::c_uchar,
                    1 as libc::c_int as size_t,
                );
                tmp___107 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                flags = *tmp___107 as libc::c_uint;
            }
        }
    } else {
        g.in_left = (g.in_left).wrapping_sub(1);
        crc = crc32z(crc, g.in_next as *const libc::c_uchar, 1 as libc::c_int as size_t);
        tmp___107 = g.in_next;
        g.in_next = (g.in_next).offset(1);
        flags = *tmp___107 as libc::c_uint;
    }
    if flags & 224 as libc::c_uint != 0 {
        return -(4 as libc::c_int);
    }
    if save != 0 {
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp2 = 0 as libc::c_uint;
            } else {
                tmp___111 = load();
                if tmp___111 == 0 as libc::c_ulong {
                    tmp2 = 0 as libc::c_uint;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    crc = crc32z(
                        crc,
                        g.in_next as *const libc::c_uchar,
                        1 as libc::c_int as size_t,
                    );
                    tmp___110 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp2 = *tmp___110 as libc::c_uint;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            crc = crc32z(
                crc,
                g.in_next as *const libc::c_uchar,
                1 as libc::c_int as size_t,
            );
            tmp___110 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp2 = *tmp___110 as libc::c_uint;
        }
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp___114 = 0 as libc::c_int;
            } else {
                tmp___115 = load();
                if tmp___115 == 0 as libc::c_ulong {
                    tmp___114 = 0 as libc::c_int;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    crc = crc32z(
                        crc,
                        g.in_next as *const libc::c_uchar,
                        1 as libc::c_int as size_t,
                    );
                    tmp___113 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp___114 = *tmp___113 as libc::c_int;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            crc = crc32z(
                crc,
                g.in_next as *const libc::c_uchar,
                1 as libc::c_int as size_t,
            );
            tmp___113 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp___114 = *tmp___113 as libc::c_int;
        }
        tmp4 = tmp2.wrapping_add((tmp___114 as libc::c_uint) << 8 as libc::c_int)
            as libc::c_ulong;
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp2 = 0 as libc::c_uint;
            } else {
                tmp___118 = load();
                if tmp___118 == 0 as libc::c_ulong {
                    tmp2 = 0 as libc::c_uint;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    crc = crc32z(
                        crc,
                        g.in_next as *const libc::c_uchar,
                        1 as libc::c_int as size_t,
                    );
                    tmp___117 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp2 = *tmp___117 as libc::c_uint;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            crc = crc32z(
                crc,
                g.in_next as *const libc::c_uchar,
                1 as libc::c_int as size_t,
            );
            tmp___117 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp2 = *tmp___117 as libc::c_uint;
        }
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp___121 = 0 as libc::c_int;
            } else {
                tmp___122 = load();
                if tmp___122 == 0 as libc::c_ulong {
                    tmp___121 = 0 as libc::c_int;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    crc = crc32z(
                        crc,
                        g.in_next as *const libc::c_uchar,
                        1 as libc::c_int as size_t,
                    );
                    tmp___120 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp___121 = *tmp___120 as libc::c_int;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            crc = crc32z(
                crc,
                g.in_next as *const libc::c_uchar,
                1 as libc::c_int as size_t,
            );
            tmp___120 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp___121 = *tmp___120 as libc::c_int;
        }
        g
            .stamp = tolong(
            tmp4
                .wrapping_add(
                    (tmp2.wrapping_add((tmp___121 as libc::c_uint) << 8 as libc::c_int)
                        as libc::c_ulong) << 16 as libc::c_int,
                ),
        );
    } else {
        togo___2 = 4 as libc::c_int as size_t;
        while togo___2 > g.in_left {
            crc = crc32z(crc, g.in_next as *const libc::c_uchar, g.in_left);
            togo___2 = (togo___2 as libc::c_ulong).wrapping_sub(g.in_left) as size_t
                as size_t;
            tmp___123 = load();
            if tmp___123 == 0 as libc::c_ulong {
                return -(3 as libc::c_int);
            }
        }
        crc = crc32z(crc, g.in_next as *const libc::c_uchar, togo___2);
        g
            .in_left = (g.in_left as libc::c_ulong).wrapping_sub(togo___2) as size_t
            as size_t;
        g.in_next = (g.in_next).offset(togo___2 as isize);
    }
    togo___3 = 2 as libc::c_int as size_t;
    while togo___3 > g.in_left {
        crc = crc32z(crc, g.in_next as *const libc::c_uchar, g.in_left);
        togo___3 = (togo___3 as libc::c_ulong).wrapping_sub(g.in_left) as size_t
            as size_t;
        tmp___124 = load();
        if tmp___124 == 0 as libc::c_ulong {
            return -(3 as libc::c_int);
        }
    }
    crc = crc32z(crc, g.in_next as *const libc::c_uchar, togo___3);
    g.in_left = (g.in_left as libc::c_ulong).wrapping_sub(togo___3) as size_t as size_t;
    g.in_next = (g.in_next).offset(togo___3 as isize);
    if flags & 4 as libc::c_uint != 0 {
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp2 = 0 as libc::c_uint;
            } else {
                tmp___127 = load();
                if tmp___127 == 0 as libc::c_ulong {
                    tmp2 = 0 as libc::c_uint;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    crc = crc32z(
                        crc,
                        g.in_next as *const libc::c_uchar,
                        1 as libc::c_int as size_t,
                    );
                    tmp___126 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp2 = *tmp___126 as libc::c_uint;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            crc = crc32z(
                crc,
                g.in_next as *const libc::c_uchar,
                1 as libc::c_int as size_t,
            );
            tmp___126 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp2 = *tmp___126 as libc::c_uint;
        }
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp___130 = 0 as libc::c_int;
            } else {
                tmp___131 = load();
                if tmp___131 == 0 as libc::c_ulong {
                    tmp___130 = 0 as libc::c_int;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    crc = crc32z(
                        crc,
                        g.in_next as *const libc::c_uchar,
                        1 as libc::c_int as size_t,
                    );
                    tmp___129 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp___130 = *tmp___129 as libc::c_int;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            crc = crc32z(
                crc,
                g.in_next as *const libc::c_uchar,
                1 as libc::c_int as size_t,
            );
            tmp___129 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp___130 = *tmp___129 as libc::c_int;
        }
        togo___4 = tmp2.wrapping_add((tmp___130 as libc::c_uint) << 8 as libc::c_int)
            as size_t;
        while togo___4 > g.in_left {
            crc = crc32z(crc, g.in_next as *const libc::c_uchar, g.in_left);
            togo___4 = (togo___4 as libc::c_ulong).wrapping_sub(g.in_left) as size_t
                as size_t;
            tmp___132 = load();
            if tmp___132 == 0 as libc::c_ulong {
                return -(3 as libc::c_int);
            }
        }
        crc = crc32z(crc, g.in_next as *const libc::c_uchar, togo___4);
        g
            .in_left = (g.in_left as libc::c_ulong).wrapping_sub(togo___4) as size_t
            as size_t;
        g.in_next = (g.in_next).offset(togo___4 as isize);
    }
    if flags & 8 as libc::c_uint != 0 {
        if save != 0 {
            size = 0 as libc::c_int as size_t;
            have = 0 as libc::c_int as size_t;
            loop {
                if g.in_left == 0 as libc::c_ulong {
                    tmp___133 = load();
                    if tmp___133 == 0 as libc::c_ulong {
                        return -(3 as libc::c_int);
                    }
                }
                tmp___134 = memchr(
                    g.in_next as *const libc::c_void,
                    0 as libc::c_int,
                    g.in_left,
                );
                end = tmp___134 as *mut libc::c_uchar;
                if end as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                    copy = g.in_left;
                } else {
                    copy = (end.offset_from(g.in_next) as libc::c_long as size_t)
                        .wrapping_add(1 as libc::c_ulong);
                }
                have = vmemcpy(
                    &mut g.hname,
                    &mut size,
                    have,
                    g.in_next as *mut libc::c_void,
                    copy,
                );
                g
                    .in_left = (g.in_left as libc::c_ulong).wrapping_sub(copy) as size_t
                    as size_t;
                g.in_next = (g.in_next).offset(copy as isize);
                if !(end as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
                    break;
                }
            }
            crc = crc32z(
                crc,
                g.hname as *mut libc::c_uchar as *const libc::c_uchar,
                have,
            );
        } else {
            loop {
                if g.in_left == 0 as libc::c_ulong {
                    if g.in_eof != 0 {
                        tmp___137 = 0 as libc::c_int;
                    } else {
                        tmp___138 = load();
                        if tmp___138 == 0 as libc::c_ulong {
                            tmp___137 = 0 as libc::c_int;
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            crc = crc32z(
                                crc,
                                g.in_next as *const libc::c_uchar,
                                1 as libc::c_int as size_t,
                            );
                            tmp___136 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            tmp___137 = *tmp___136 as libc::c_int;
                        }
                    }
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    crc = crc32z(
                        crc,
                        g.in_next as *const libc::c_uchar,
                        1 as libc::c_int as size_t,
                    );
                    tmp___136 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp___137 = *tmp___136 as libc::c_int;
                }
                if !(tmp___137 != 0 as libc::c_int) {
                    break;
                }
            }
        }
    }
    if flags & 16 as libc::c_uint != 0 {
        if save != 0 {
            size___0 = 0 as libc::c_int as size_t;
            have___0 = 0 as libc::c_int as size_t;
            loop {
                if g.in_left == 0 as libc::c_ulong {
                    tmp___139 = load();
                    if tmp___139 == 0 as libc::c_ulong {
                        return -(3 as libc::c_int);
                    }
                }
                tmp___140 = memchr(
                    g.in_next as *const libc::c_void,
                    0 as libc::c_int,
                    g.in_left,
                );
                end___0 = tmp___140 as *mut libc::c_uchar;
                if end___0 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                    copy___0 = g.in_left;
                } else {
                    copy___0 = (end___0.offset_from(g.in_next) as libc::c_long as size_t)
                        .wrapping_add(1 as libc::c_ulong);
                }
                have___0 = vmemcpy(
                    &mut g.hcomm,
                    &mut size___0,
                    have___0,
                    g.in_next as *mut libc::c_void,
                    copy___0,
                );
                g
                    .in_left = (g.in_left as libc::c_ulong).wrapping_sub(copy___0)
                    as size_t as size_t;
                g.in_next = (g.in_next).offset(copy___0 as isize);
                if !(end___0 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong)
                {
                    break;
                }
            }
            crc = crc32z(
                crc,
                g.hcomm as *mut libc::c_uchar as *const libc::c_uchar,
                have___0,
            );
        } else {
            loop {
                if g.in_left == 0 as libc::c_ulong {
                    if g.in_eof != 0 {
                        tmp___143 = 0 as libc::c_int;
                    } else {
                        tmp___144 = load();
                        if tmp___144 == 0 as libc::c_ulong {
                            tmp___143 = 0 as libc::c_int;
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            crc = crc32z(
                                crc,
                                g.in_next as *const libc::c_uchar,
                                1 as libc::c_int as size_t,
                            );
                            tmp___142 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            tmp___143 = *tmp___142 as libc::c_int;
                        }
                    }
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    crc = crc32z(
                        crc,
                        g.in_next as *const libc::c_uchar,
                        1 as libc::c_int as size_t,
                    );
                    tmp___142 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp___143 = *tmp___142 as libc::c_int;
                }
                if !(tmp___143 != 0 as libc::c_int) {
                    break;
                }
            }
        }
    }
    if flags & 2 as libc::c_uint != 0 {
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp2 = 0 as libc::c_uint;
            } else {
                tmp___147 = load();
                if tmp___147 == 0 as libc::c_ulong {
                    tmp2 = 0 as libc::c_uint;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___146 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp2 = *tmp___146 as libc::c_uint;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___146 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp2 = *tmp___146 as libc::c_uint;
        }
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                tmp___150 = 0 as libc::c_int;
            } else {
                tmp___151 = load();
                if tmp___151 == 0 as libc::c_ulong {
                    tmp___150 = 0 as libc::c_int;
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___149 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp___150 = *tmp___149 as libc::c_int;
                }
            }
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___149 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            tmp___150 = *tmp___149 as libc::c_int;
        }
        if tmp2.wrapping_add((tmp___150 as libc::c_uint) << 8 as libc::c_int)
            as libc::c_ulong != crc & 65535 as libc::c_ulong
        {
            return -(6 as libc::c_int);
        }
    }
    g.form = 0 as libc::c_int;
    if g.in_eof != 0 {
        tmp___152 = -(3 as libc::c_int);
    } else {
        tmp___152 = method as libc::c_int;
    }
    return tmp___152;
}
unsafe extern "C" fn more_zip_entries() -> libc::c_int {
    let mut sig: libc::c_ulong = 0;
    let mut ret: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut first: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp2: libc::c_uint = 0;
    let mut tmp4: libc::c_ulong = 0;
    let mut central: [libc::c_uchar; 4] = [0; 4];
    let mut tmp___0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___1: size_t = 0;
    let mut tmp___3: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: size_t = 0;
    let mut tmp___7: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___8: size_t = 0;
    let mut tmp___10: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: size_t = 0;
    let mut tmp___13: libc::c_int = 0;
    let mut tmp___14: size_t = 0;
    let mut tmp___15: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut head: [libc::c_uchar; 42] = [0; 42];
    let mut need: size_t = 0;
    let mut part: size_t = 0;
    let mut len: size_t = 0;
    let mut i: size_t = 0;
    let mut tmp___16: size_t = 0;
    let mut togo: size_t = 0;
    let mut tmp___17: size_t = 0;
    let mut tmp___18: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___19: size_t = 0;
    central[0 as libc::c_int as usize] = 80 as libc::c_int as libc::c_uchar;
    central[1 as libc::c_int as usize] = 75 as libc::c_int as libc::c_uchar;
    central[2 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    central[3 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    if g.in_left == 0 as libc::c_ulong {
        if g.in_eof != 0 {
            tmp2 = 0 as libc::c_uint;
        } else {
            tmp___1 = load();
            if tmp___1 == 0 as libc::c_ulong {
                tmp2 = 0 as libc::c_uint;
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                tmp___0 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                tmp2 = *tmp___0 as libc::c_uint;
            }
        }
    } else {
        g.in_left = (g.in_left).wrapping_sub(1);
        tmp___0 = g.in_next;
        g.in_next = (g.in_next).offset(1);
        tmp2 = *tmp___0 as libc::c_uint;
    }
    if g.in_left == 0 as libc::c_ulong {
        if g.in_eof != 0 {
            tmp___4 = 0 as libc::c_int;
        } else {
            tmp___5 = load();
            if tmp___5 == 0 as libc::c_ulong {
                tmp___4 = 0 as libc::c_int;
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                tmp___3 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                tmp___4 = *tmp___3 as libc::c_int;
            }
        }
    } else {
        g.in_left = (g.in_left).wrapping_sub(1);
        tmp___3 = g.in_next;
        g.in_next = (g.in_next).offset(1);
        tmp___4 = *tmp___3 as libc::c_int;
    }
    tmp4 = tmp2.wrapping_add((tmp___4 as libc::c_uint) << 8 as libc::c_int)
        as libc::c_ulong;
    if g.in_left == 0 as libc::c_ulong {
        if g.in_eof != 0 {
            tmp2 = 0 as libc::c_uint;
        } else {
            tmp___8 = load();
            if tmp___8 == 0 as libc::c_ulong {
                tmp2 = 0 as libc::c_uint;
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                tmp___7 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                tmp2 = *tmp___7 as libc::c_uint;
            }
        }
    } else {
        g.in_left = (g.in_left).wrapping_sub(1);
        tmp___7 = g.in_next;
        g.in_next = (g.in_next).offset(1);
        tmp2 = *tmp___7 as libc::c_uint;
    }
    if g.in_left == 0 as libc::c_ulong {
        if g.in_eof != 0 {
            tmp___11 = 0 as libc::c_int;
        } else {
            tmp___12 = load();
            if tmp___12 == 0 as libc::c_ulong {
                tmp___11 = 0 as libc::c_int;
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                tmp___10 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                tmp___11 = *tmp___10 as libc::c_int;
            }
        }
    } else {
        g.in_left = (g.in_left).wrapping_sub(1);
        tmp___10 = g.in_next;
        g.in_next = (g.in_next).offset(1);
        tmp___11 = *tmp___10 as libc::c_int;
    }
    sig = tmp4
        .wrapping_add(
            (tmp2.wrapping_add((tmp___11 as libc::c_uint) << 8 as libc::c_int)
                as libc::c_ulong) << 16 as libc::c_int,
        );
    if g.in_eof == 0 {
        if sig == 67324752 as libc::c_ulong {
            tmp___13 = 1 as libc::c_int;
        } else {
            tmp___13 = 0 as libc::c_int;
        }
    } else {
        tmp___13 = 0 as libc::c_int;
    }
    ret = tmp___13;
    if g.list == 0 {
        return ret
    } else {
        if g.verbosity < 2 as libc::c_int {
            return ret;
        }
    }
    if sig == 33639248 as libc::c_ulong {
        n = 4 as libc::c_int;
    } else {
        n = 0 as libc::c_int;
    }
    loop {
        if g.in_left == 0 as libc::c_ulong {
            tmp___14 = load();
            if tmp___14 == 0 as libc::c_ulong {
                return ret;
            }
        }
        if n == 0 as libc::c_int {
            tmp___15 = memchr(
                g.in_next as *const libc::c_void,
                central[0 as libc::c_int as usize] as libc::c_int,
                g.in_left,
            );
            first = tmp___15 as *mut libc::c_uchar;
            if first as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                g.in_left = 0 as libc::c_int as size_t;
            } else {
                n += 1;
                g
                    .in_left = (g.in_left as libc::c_ulong)
                    .wrapping_sub(
                        (first.offset_from(g.in_next) as libc::c_long
                            + 1 as libc::c_long) as size_t,
                    ) as size_t as size_t;
                g.in_next = first.offset(1 as libc::c_int as isize);
            }
        } else if n < 4 as libc::c_int {
            if *(g.in_next).offset(0 as libc::c_int as isize) as libc::c_int
                == central[n as usize] as libc::c_int
            {
                n += 1;
                g.in_next = (g.in_next).offset(1);
                g.in_left = (g.in_left).wrapping_sub(1);
            } else {
                n = 0 as libc::c_int;
            }
        } else {
            need = 42 as libc::c_int as size_t;
            part = 0 as libc::c_int as size_t;
            if need > g.in_left {
                part = g.in_left;
                memcpy(
                    head
                        .as_mut_ptr()
                        .offset(42 as libc::c_int as isize)
                        .offset(-(need as isize)) as *mut libc::c_void,
                    g.in_next as *const libc::c_void,
                    part,
                );
                need = (need as libc::c_ulong).wrapping_sub(part) as size_t as size_t;
                g.in_left = 0 as libc::c_int as size_t;
                tmp___16 = load();
                if tmp___16 == 0 as libc::c_ulong {
                    return ret;
                }
            }
            memcpy(
                head
                    .as_mut_ptr()
                    .offset(42 as libc::c_int as isize)
                    .offset(-(need as isize)) as *mut libc::c_void,
                g.in_next as *const libc::c_void,
                need,
            );
            if ((*head
                .as_mut_ptr()
                .offset(12 as libc::c_int as isize)
                .offset(0 as libc::c_int as isize) as libc::c_uint)
                .wrapping_add(
                    (*head
                        .as_mut_ptr()
                        .offset(12 as libc::c_int as isize)
                        .offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                ) as libc::c_ulong)
                .wrapping_add(
                    ((*head
                        .as_mut_ptr()
                        .offset(12 as libc::c_int as isize)
                        .offset(2 as libc::c_int as isize)
                        .offset(0 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*head
                                .as_mut_ptr()
                                .offset(12 as libc::c_int as isize)
                                .offset(2 as libc::c_int as isize)
                                .offset(1 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        ) as libc::c_ulong) << 16 as libc::c_int,
                ) == g.out_check
            {
                if ((*head
                    .as_mut_ptr()
                    .offset(38 as libc::c_int as isize)
                    .offset(0 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*head
                            .as_mut_ptr()
                            .offset(38 as libc::c_int as isize)
                            .offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    ) as libc::c_ulong)
                    .wrapping_add(
                        ((*head
                            .as_mut_ptr()
                            .offset(38 as libc::c_int as isize)
                            .offset(2 as libc::c_int as isize)
                            .offset(0 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*head
                                    .as_mut_ptr()
                                    .offset(38 as libc::c_int as isize)
                                    .offset(2 as libc::c_int as isize)
                                    .offset(1 as libc::c_int as isize) as libc::c_uint)
                                    << 8 as libc::c_int,
                            ) as libc::c_ulong) << 16 as libc::c_int,
                    ) == 0 as libc::c_ulong
                {
                    g.in_next = (g.in_next).offset(need as isize);
                    g
                        .in_left = (g.in_left as libc::c_ulong).wrapping_sub(need)
                        as size_t as size_t;
                    len = (*head
                        .as_mut_ptr()
                        .offset(28 as libc::c_int as isize)
                        .offset(0 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*head
                                .as_mut_ptr()
                                .offset(28 as libc::c_int as isize)
                                .offset(1 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        ) as size_t;
                    if len == 0 as libc::c_ulong {
                        return ret;
                    }
                    togo = ((*head
                        .as_mut_ptr()
                        .offset(24 as libc::c_int as isize)
                        .offset(0 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*head
                                .as_mut_ptr()
                                .offset(24 as libc::c_int as isize)
                                .offset(1 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        ) as libc::c_ulong)
                        .wrapping_add(
                            (*head
                                .as_mut_ptr()
                                .offset(26 as libc::c_int as isize)
                                .offset(0 as libc::c_int as isize) as libc::c_uint)
                                .wrapping_add(
                                    (*head
                                        .as_mut_ptr()
                                        .offset(26 as libc::c_int as isize)
                                        .offset(1 as libc::c_int as isize) as libc::c_uint)
                                        << 8 as libc::c_int,
                                ) as libc::c_ulong,
                        );
                    while togo > g.in_left {
                        togo = (togo as libc::c_ulong).wrapping_sub(g.in_left) as size_t
                            as size_t;
                        tmp___17 = load();
                        if tmp___17 == 0 as libc::c_ulong {
                            return -(3 as libc::c_int);
                        }
                    }
                    g
                        .in_left = (g.in_left as libc::c_ulong).wrapping_sub(togo)
                        as size_t as size_t;
                    g.in_next = (g.in_next).offset(togo as isize);
                    need = len;
                    tmp___18 = alloc(
                        0 as *mut libc::c_void,
                        len.wrapping_add(1 as libc::c_ulong),
                    );
                    g.hcomm = tmp___18 as *mut libc::c_char;
                    while need > g.in_left {
                        memcpy(
                            (g.hcomm).offset(len as isize).offset(-(need as isize))
                                as *mut libc::c_void,
                            g.in_next as *const libc::c_void,
                            g.in_left,
                        );
                        need = (need as libc::c_ulong).wrapping_sub(g.in_left) as size_t
                            as size_t;
                        g.in_left = 0 as libc::c_int as size_t;
                        tmp___19 = load();
                        if tmp___19 == 0 as libc::c_ulong {
                            if g.hcomm as libc::c_ulong
                                != 0 as *mut libc::c_void as libc::c_ulong
                            {
                                free(g.hcomm as *mut libc::c_void);
                                g.hcomm = 0 as *mut libc::c_void as *mut libc::c_char;
                            }
                            return ret;
                        }
                    }
                    memcpy(
                        (g.hcomm).offset(len as isize).offset(-(need as isize))
                            as *mut libc::c_void,
                        g.in_next as *const libc::c_void,
                        need,
                    );
                    g.in_next = (g.in_next).offset(need as isize);
                    g
                        .in_left = (g.in_left as libc::c_ulong).wrapping_sub(need)
                        as size_t as size_t;
                    i = 0 as libc::c_int as size_t;
                    while i < len {
                        if *(g.hcomm).offset(i as isize) as libc::c_int
                            == 0 as libc::c_int
                        {
                            *(g.hcomm).offset(i as isize) = ' ' as i32 as libc::c_char;
                        }
                        i = i.wrapping_add(1);
                    }
                    *(g.hcomm).offset(len as isize) = 0 as libc::c_int as libc::c_char;
                    return ret;
                }
            }
            if part != 0 {
                memmove(
                    (g.in_next).offset(part as isize) as *mut libc::c_void,
                    g.in_next as *const libc::c_void,
                    g.in_left,
                );
                memcpy(
                    g.in_next as *mut libc::c_void,
                    head.as_mut_ptr() as *const libc::c_void,
                    part,
                );
                g
                    .in_left = (g.in_left as libc::c_ulong).wrapping_add(part) as size_t
                    as size_t;
            }
            n = 0 as libc::c_int;
        }
    };
}
unsafe extern "C" fn compressed_suffix(mut nm: *mut libc::c_char) -> size_t {
    let mut len: size_t = 0;
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
    len = strlen(nm as *const libc::c_char);
    if len > 4 as libc::c_ulong {
        nm = nm.offset(len.wrapping_sub(4 as libc::c_ulong) as isize);
        len = 4 as libc::c_int as size_t;
        tmp = strcmp(
            nm as *const libc::c_char,
            b".zip\0" as *const u8 as *const libc::c_char,
        );
        if tmp == 0 as libc::c_int {
            return 4 as libc::c_int as size_t
        } else {
            tmp___0 = strcmp(
                nm as *const libc::c_char,
                b".ZIP\0" as *const u8 as *const libc::c_char,
            );
            if tmp___0 == 0 as libc::c_int {
                return 4 as libc::c_int as size_t
            } else {
                tmp___1 = strcmp(
                    nm as *const libc::c_char,
                    b".tgz\0" as *const u8 as *const libc::c_char,
                );
                if tmp___1 == 0 as libc::c_int {
                    return 4 as libc::c_int as size_t;
                }
            }
        }
    }
    if len > 3 as libc::c_ulong {
        nm = nm.offset(len.wrapping_sub(3 as libc::c_ulong) as isize);
        len = 3 as libc::c_int as size_t;
        tmp___2 = strcmp(
            nm as *const libc::c_char,
            b".gz\0" as *const u8 as *const libc::c_char,
        );
        if tmp___2 == 0 as libc::c_int {
            return 3 as libc::c_int as size_t
        } else {
            tmp___3 = strcmp(
                nm as *const libc::c_char,
                b"-gz\0" as *const u8 as *const libc::c_char,
            );
            if tmp___3 == 0 as libc::c_int {
                return 3 as libc::c_int as size_t
            } else {
                tmp___4 = strcmp(
                    nm as *const libc::c_char,
                    b".zz\0" as *const u8 as *const libc::c_char,
                );
                if tmp___4 == 0 as libc::c_int {
                    return 3 as libc::c_int as size_t
                } else {
                    tmp___5 = strcmp(
                        nm as *const libc::c_char,
                        b"-zz\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___5 == 0 as libc::c_int {
                        return 3 as libc::c_int as size_t;
                    }
                }
            }
        }
    }
    if len > 2 as libc::c_ulong {
        nm = nm.offset(len.wrapping_sub(2 as libc::c_ulong) as isize);
        tmp___6 = strcmp(
            nm as *const libc::c_char,
            b".z\0" as *const u8 as *const libc::c_char,
        );
        if tmp___6 == 0 as libc::c_int {
            return 2 as libc::c_int as size_t
        } else {
            tmp___7 = strcmp(
                nm as *const libc::c_char,
                b"-z\0" as *const u8 as *const libc::c_char,
            );
            if tmp___7 == 0 as libc::c_int {
                return 2 as libc::c_int as size_t
            } else {
                tmp___8 = strcmp(
                    nm as *const libc::c_char,
                    b"_z\0" as *const u8 as *const libc::c_char,
                );
                if tmp___8 == 0 as libc::c_int {
                    return 2 as libc::c_int as size_t
                } else {
                    tmp___9 = strcmp(
                        nm as *const libc::c_char,
                        b".Z\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___9 == 0 as libc::c_int {
                        return 2 as libc::c_int as size_t;
                    }
                }
            }
        }
    }
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn show_info(
    mut method: libc::c_int,
    mut check: libc::c_ulong,
    mut len: length_t,
    mut cont: libc::c_int,
) {
    let mut max: size_t = 0;
    let mut n: size_t = 0;
    let mut now: time_t = 0;
    let mut mod_0: [libc::c_char; 26] = [0; 26];
    let mut tag: [libc::c_char; 49] = [0; 49];
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: libc::c_int = 0;
    let mut red: libc::c_double = 0.;
    if g.verbosity > 1 as libc::c_int {
        max = 16 as libc::c_int as size_t;
    } else {
        max = 48 as libc::c_int as size_t;
    }
    memset(
        tag.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        max.wrapping_add(1 as libc::c_ulong),
    );
    if cont != 0 {
        strncpy(
            tag.as_mut_ptr(),
            b"<...>\0" as *const u8 as *const libc::c_char,
            max.wrapping_add(1 as libc::c_ulong),
        );
    } else if g.hname as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp = strlen(g.inf as *const libc::c_char);
        tmp___0 = compressed_suffix(g.inf);
        n = tmp.wrapping_sub(tmp___0);
        if n > max.wrapping_add(1 as libc::c_ulong) {
            tmp___1 = max.wrapping_add(1 as libc::c_ulong);
        } else {
            tmp___1 = n;
        }
        memcpy(
            tag.as_mut_ptr() as *mut libc::c_void,
            g.inf as *const libc::c_void,
            tmp___1,
        );
        tmp___2 = strcmp(
            (g.inf).offset(n as isize) as *const libc::c_char,
            b".tgz\0" as *const u8 as *const libc::c_char,
        );
        if tmp___2 == 0 as libc::c_int {
            if n < max.wrapping_add(1 as libc::c_ulong) {
                strncpy(
                    tag.as_mut_ptr().offset(n as isize),
                    b".tar\0" as *const u8 as *const libc::c_char,
                    max.wrapping_add(1 as libc::c_ulong).wrapping_sub(n),
                );
            }
        }
    } else {
        strncpy(
            tag.as_mut_ptr(),
            g.hname as *const libc::c_char,
            max.wrapping_add(1 as libc::c_ulong),
        );
    }
    if tag[max as usize] != 0 {
        strcpy(
            tag.as_mut_ptr().offset(max as isize).offset(-(3 as libc::c_int as isize)),
            b"...\0" as *const u8 as *const libc::c_char,
        );
    }
    if g.stamp != 0 {
        if cont == 0 {
            tmp___3 = ctime(&mut g.stamp as *mut time_t as *const time_t);
            strcpy(mod_0.as_mut_ptr(), tmp___3 as *const libc::c_char);
            now = time(0 as *mut libc::c_void as *mut time_t);
            tmp___4 = ctime(&mut now as *mut time_t as *const time_t);
            tmp___5 = strcmp(
                mod_0.as_mut_ptr().offset(20 as libc::c_int as isize)
                    as *const libc::c_char,
                tmp___4.offset(20 as libc::c_int as isize) as *const libc::c_char,
            );
            if tmp___5 != 0 as libc::c_int {
                strcpy(
                    mod_0.as_mut_ptr().offset(11 as libc::c_int as isize),
                    mod_0.as_mut_ptr().offset(19 as libc::c_int as isize)
                        as *const libc::c_char,
                );
            }
        } else {
            strcpy(
                mod_0.as_mut_ptr().offset(4 as libc::c_int as isize),
                b"------ -----\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        strcpy(
            mod_0.as_mut_ptr().offset(4 as libc::c_int as isize),
            b"------ -----\0" as *const u8 as *const libc::c_char,
        );
    }
    mod_0[16 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    if g.first != 0 {
        if g.verbosity > 1 as libc::c_int {
            fputs(
                b"method    check    timestamp    \0" as *const u8
                    as *const libc::c_char,
                stdout,
            );
        }
        if g.verbosity > 0 as libc::c_int {
            puts(
                b"compressed   original reduced  name\0" as *const u8
                    as *const libc::c_char,
            );
        }
        g.first = 0 as libc::c_int;
    }
    if g.verbosity > 1 as libc::c_int {
        let mut current_block_69: u64;
        if g.form == 3 as libc::c_int {
            if g.decode == 0 {
                printf(
                    b"zip%3d  --------  %s  \0" as *const u8 as *const libc::c_char,
                    method,
                    mod_0.as_mut_ptr().offset(4 as libc::c_int as isize),
                );
                current_block_69 = 3580086814630675314;
            } else {
                current_block_69 = 15401336509500856050;
            }
        } else {
            current_block_69 = 15401336509500856050;
        }
        match current_block_69 {
            15401336509500856050 => {
                if g.form > 1 as libc::c_int {
                    printf(
                        b"zip%3d  %08lx  %s  \0" as *const u8 as *const libc::c_char,
                        method,
                        check,
                        mod_0.as_mut_ptr().offset(4 as libc::c_int as isize),
                    );
                } else if g.form == 1 as libc::c_int {
                    printf(
                        b"zlib%2d  %08lx  %s  \0" as *const u8 as *const libc::c_char,
                        method,
                        check,
                        mod_0.as_mut_ptr().offset(4 as libc::c_int as isize),
                    );
                } else if method == 257 as libc::c_int {
                    printf(
                        b"lzw     --------  %s  \0" as *const u8 as *const libc::c_char,
                        mod_0.as_mut_ptr().offset(4 as libc::c_int as isize),
                    );
                } else {
                    printf(
                        b"gzip%2d  %08lx  %s  \0" as *const u8 as *const libc::c_char,
                        method,
                        check,
                        mod_0.as_mut_ptr().offset(4 as libc::c_int as isize),
                    );
                }
            }
            _ => {}
        }
    }
    if g.verbosity > 0 as libc::c_int {
        red = 100.0f64 * (len as libc::c_double - g.in_tot as libc::c_double)
            / len as libc::c_double;
        let mut current_block_87: u64;
        if g.form == 3 as libc::c_int {
            if g.decode == 0 {
                printf(
                    b"%10ju %10ju?  unk    %s\n\0" as *const u8 as *const libc::c_char,
                    g.in_tot,
                    len,
                    tag.as_mut_ptr(),
                );
                current_block_87 = 14294131666767243020;
            } else {
                current_block_87 = 6935079036334163347;
            }
        } else {
            current_block_87 = 6935079036334163347;
        }
        match current_block_87 {
            6935079036334163347 => {
                let mut current_block_86: u64;
                if method == 8 as libc::c_int {
                    if g.in_tot
                        > len
                            .wrapping_add(len >> 10 as libc::c_int)
                            .wrapping_add(12 as libc::c_ulong)
                    {
                        printf(
                            b"%10ju %10ju?  unk    %s\n\0" as *const u8
                                as *const libc::c_char,
                            g.in_tot,
                            len,
                            tag.as_mut_ptr(),
                        );
                        current_block_86 = 14648606000749551097;
                    } else {
                        current_block_86 = 11561166621664383601;
                    }
                } else {
                    current_block_86 = 11561166621664383601;
                }
                match current_block_86 {
                    11561166621664383601 => {
                        if method == 257 as libc::c_int {
                            if g.in_tot
                                > len
                                    .wrapping_add(len >> 1 as libc::c_int)
                                    .wrapping_add(3 as libc::c_ulong)
                            {
                                printf(
                                    b"%10ju %10ju?  unk    %s\n\0" as *const u8
                                        as *const libc::c_char,
                                    g.in_tot,
                                    len,
                                    tag.as_mut_ptr(),
                                );
                            } else {
                                printf(
                                    b"%10ju %10ju %6.1f%%  %s\n\0" as *const u8
                                        as *const libc::c_char,
                                    g.in_tot,
                                    len,
                                    red,
                                    tag.as_mut_ptr(),
                                );
                            }
                        } else {
                            printf(
                                b"%10ju %10ju %6.1f%%  %s\n\0" as *const u8
                                    as *const libc::c_char,
                                g.in_tot,
                                len,
                                red,
                                tag.as_mut_ptr(),
                            );
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    if g.verbosity > 1 as libc::c_int {
        if g.hcomm as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            puts(g.hcomm as *const libc::c_char);
        }
    }
}
unsafe extern "C" fn list_info() {
    let mut method: libc::c_int = 0;
    let mut n: size_t = 0;
    let mut at: off_t = 0;
    let mut tail: [libc::c_uchar; 8] = [0; 8];
    let mut check: libc::c_ulong = 0;
    let mut len: length_t = 0;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___2: length_t = 0;
    let mut tmp___3: size_t = 0;
    let mut tmp___4: size_t = 0;
    in_init();
    method = get_header(1 as libc::c_int);
    if method < 0 as libc::c_int {
        if method == -(6 as libc::c_int) {
            tmp___0 = b"skipping: %s corrupt: header crc error\0" as *const u8
                as *const libc::c_char;
        } else {
            if method == -(1 as libc::c_int) {
                tmp = b"skipping: %s empty\0" as *const u8 as *const libc::c_char;
            } else {
                tmp = b"skipping: %s unrecognized format\0" as *const u8
                    as *const libc::c_char;
            }
            tmp___0 = tmp;
        }
        complain(tmp___0 as *mut libc::c_char, g.inf);
        return;
    }
    load_wait();
    if g.form > 1 as libc::c_int {
        more_zip_entries();
        g.in_tot = g.zip_clen;
        show_info(method, g.zip_crc, g.zip_ulen, 0 as libc::c_int);
        return;
    }
    if g.form == 1 as libc::c_int {
        at = lseek(g.ind, 0 as libc::c_int as __off64_t, 2 as libc::c_int);
        if at == -(1 as libc::c_long) {
            check = 0 as libc::c_ulong;
            loop {
                if g.in_left < 4 as libc::c_ulong {
                    len = g.in_left;
                } else {
                    len = 4 as libc::c_int as length_t;
                }
                g.in_next = (g.in_next).offset((g.in_left).wrapping_sub(len) as isize);
                loop {
                    tmp___2 = len;
                    len = len.wrapping_sub(1);
                    if tmp___2 == 0 {
                        break;
                    }
                    tmp___1 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    check = (check << 8 as libc::c_int)
                        .wrapping_add(*tmp___1 as libc::c_ulong);
                }
                tmp___3 = load();
                if !(tmp___3 != 0 as libc::c_ulong) {
                    break;
                }
            }
            check &= 4294967295 as libc::c_ulong;
        } else {
            g.in_tot = at as length_t;
            lseek(g.ind, -(4 as libc::c_int) as __off64_t, 2 as libc::c_int);
            readn(g.ind, tail.as_mut_ptr(), 4 as libc::c_int as size_t);
            check = ((((tail[0 as libc::c_int as usize] as libc::c_uint)
                << 8 as libc::c_int)
                .wrapping_add(tail[1 as libc::c_int as usize] as libc::c_uint)
                as libc::c_ulong) << 16 as libc::c_int)
                .wrapping_add(
                    ((*tail
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(0 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int)
                        .wrapping_add(
                            *tail
                                .as_mut_ptr()
                                .offset(2 as libc::c_int as isize)
                                .offset(1 as libc::c_int as isize) as libc::c_uint,
                        ) as libc::c_ulong,
                );
        }
        g
            .in_tot = (g.in_tot as libc::c_ulong).wrapping_sub(6 as libc::c_ulong)
            as length_t as length_t;
        show_info(method, check, 0 as libc::c_int as length_t, 0 as libc::c_int);
        return;
    }
    if method == 257 as libc::c_int {
        at = lseek(g.ind, 0 as libc::c_int as __off64_t, 2 as libc::c_int);
        if at == -(1 as libc::c_long) {
            loop {
                tmp___4 = load();
                if !(tmp___4 != 0 as libc::c_ulong) {
                    break;
                }
            }
        } else {
            g.in_tot = at as length_t;
        }
        g
            .in_tot = (g.in_tot as libc::c_ulong).wrapping_sub(3 as libc::c_ulong)
            as length_t as length_t;
        show_info(
            method,
            0 as libc::c_ulong,
            0 as libc::c_int as length_t,
            0 as libc::c_int,
        );
        return;
    }
    if g.in_short != 0 {
        if g.in_left < 8 as libc::c_ulong {
            complain(
                b"skipping: %s not a valid gzip file\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                g.inf,
            );
            return;
        }
        g.in_tot = (g.in_left).wrapping_sub(8 as libc::c_ulong);
        memcpy(
            tail.as_mut_ptr() as *mut libc::c_void,
            (g.in_next).offset((g.in_left).wrapping_sub(8 as libc::c_ulong) as isize)
                as *const libc::c_void,
            8 as libc::c_int as size_t,
        );
    } else {
        at = lseek(g.ind, -(8 as libc::c_int) as __off64_t, 2 as libc::c_int);
        if at != -(1 as libc::c_long) {
            g.in_tot = (at as length_t).wrapping_sub(g.in_tot).wrapping_add(g.in_left);
            readn(g.ind, tail.as_mut_ptr(), 8 as libc::c_int as size_t);
        } else {
            len = (g.in_tot).wrapping_sub(g.in_left);
            loop {
                if g.in_left < 8 as libc::c_ulong {
                    n = g.in_left;
                } else {
                    n = 8 as libc::c_int as size_t;
                }
                memcpy(
                    tail.as_mut_ptr() as *mut libc::c_void,
                    (g.in_next).offset((g.in_left).wrapping_sub(n) as isize)
                        as *const libc::c_void,
                    n,
                );
                load();
                if !(g.in_left == 32768 as libc::c_ulong) {
                    break;
                }
            }
            if g.in_left < 8 as libc::c_ulong {
                if n.wrapping_add(g.in_left) < 8 as libc::c_ulong {
                    complain(
                        b"skipping: %s not a valid gzip file\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        g.inf,
                    );
                    return;
                }
                if g.in_left != 0 {
                    if n.wrapping_add(g.in_left) > 8 as libc::c_ulong {
                        memcpy(
                            tail.as_mut_ptr() as *mut libc::c_void,
                            tail
                                .as_mut_ptr()
                                .offset(n as isize)
                                .offset(
                                    -((8 as libc::c_ulong).wrapping_sub(g.in_left) as isize),
                                ) as *const libc::c_void,
                            (8 as libc::c_ulong).wrapping_sub(g.in_left),
                        );
                    }
                    memcpy(
                        tail
                            .as_mut_ptr()
                            .offset(8 as libc::c_int as isize)
                            .offset(-(g.in_left as isize)) as *mut libc::c_void,
                        g.in_next as *const libc::c_void,
                        g.in_left,
                    );
                }
            } else {
                memcpy(
                    tail.as_mut_ptr() as *mut libc::c_void,
                    (g.in_next)
                        .offset((g.in_left).wrapping_sub(8 as libc::c_ulong) as isize)
                        as *const libc::c_void,
                    8 as libc::c_int as size_t,
                );
            }
            g
                .in_tot = (g.in_tot as libc::c_ulong)
                .wrapping_sub(len.wrapping_add(8 as libc::c_ulong)) as length_t
                as length_t;
        }
    }
    if g.in_tot < 2 as libc::c_ulong {
        complain(
            b"skipping: %s not a valid gzip file\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            g.inf,
        );
        return;
    }
    check = ((tail[0 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(
            (tail[1 as libc::c_int as usize] as libc::c_uint) << 8 as libc::c_int,
        ) as libc::c_ulong)
        .wrapping_add(
            ((*tail
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize)
                .offset(0 as libc::c_int as isize) as libc::c_uint)
                .wrapping_add(
                    (*tail
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                ) as libc::c_ulong) << 16 as libc::c_int,
        );
    len = ((*tail
        .as_mut_ptr()
        .offset(4 as libc::c_int as isize)
        .offset(0 as libc::c_int as isize) as libc::c_uint)
        .wrapping_add(
            (*tail
                .as_mut_ptr()
                .offset(4 as libc::c_int as isize)
                .offset(1 as libc::c_int as isize) as libc::c_uint) << 8 as libc::c_int,
        ) as libc::c_ulong)
        .wrapping_add(
            ((*tail
                .as_mut_ptr()
                .offset(4 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize)
                .offset(0 as libc::c_int as isize) as libc::c_uint)
                .wrapping_add(
                    (*tail
                        .as_mut_ptr()
                        .offset(4 as libc::c_int as isize)
                        .offset(2 as libc::c_int as isize)
                        .offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                ) as libc::c_ulong) << 16 as libc::c_int,
        );
    show_info(method, check, len, 0 as libc::c_int);
}
unsafe extern "C" fn cat() {
    let mut buf: [libc::c_uchar; 1] = [0; 1];
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    if g.magic1 != -(1 as libc::c_int) {
        buf[0 as libc::c_int as usize] = g.magic1 as libc::c_uchar;
        tmp = writen(
            g.outd,
            buf.as_mut_ptr() as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
        g
            .out_tot = (g.out_tot as libc::c_ulong).wrapping_add(tmp) as length_t
            as length_t;
    }
    while g.in_left != 0 {
        tmp___0 = writen(g.outd, g.in_next as *const libc::c_void, g.in_left);
        g
            .out_tot = (g.out_tot as libc::c_ulong).wrapping_add(tmp___0) as length_t
            as length_t;
        g.in_left = 0 as libc::c_int as size_t;
        load();
    }
}
unsafe extern "C" fn inb(
    mut desc: *mut libc::c_void,
    mut buf: *mut *mut libc::c_uchar,
) -> libc::c_uint {
    let mut len: libc::c_uint = 0;
    let mut tmp: libc::c_uint = 0;
    if g.in_left == 0 as libc::c_ulong {
        load();
    }
    *buf = g.in_next;
    if g.in_left > 4294967295 as libc::c_ulong {
        tmp = 4294967295 as libc::c_uint;
    } else {
        tmp = g.in_left as libc::c_uint;
    }
    len = tmp;
    g.in_next = (g.in_next).offset(len as isize);
    g
        .in_left = (g.in_left as libc::c_ulong).wrapping_sub(len as size_t) as size_t
        as size_t;
    return len;
}
static mut out_buf: [libc::c_uchar; 32768] = [0; 32768];
static mut out_copy: [libc::c_uchar; 32768] = [0; 32768];
static mut out_len: size_t = 0;
static mut outb_write_more: *mut lock = 0 as *const libc::c_void as *mut libc::c_void
    as *mut lock;
static mut outb_check_more: *mut lock = 0 as *const lock as *mut lock;
unsafe extern "C" fn outb_write(mut dummy: *mut libc::c_void) {
    let mut len: size_t = 0;
    let mut err: try_ball_t_ = try_ball_t_ {
        ret: 0,
        code: 0,
        free: 0,
        why: 0 as *mut libc::c_char,
    };
    let mut try_this_: try_t_ = try_t_ {
        env: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
        ball: try_ball_t_ {
            ret: 0,
            code: 0,
            free: 0,
            why: 0 as *mut libc::c_char,
        },
        next: 0 as *mut try_t_,
    };
    let mut try_pushed_: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    try_pushed_ = 1 as libc::c_int;
    try_this_.ball.ret = 0 as libc::c_int;
    try_this_.ball.code = 0 as libc::c_int;
    try_this_.ball.free = 0 as libc::c_int;
    try_this_.ball.why = 0 as *mut libc::c_void as *mut libc::c_char;
    try_setup_();
    tmp = pthread_getspecific(try_key_);
    try_this_.next = tmp as *mut try_t_;
    tmp___3 = pthread_setspecific(
        try_key_,
        &mut try_this_ as *mut try_t_ as *const libc::c_void,
    );
    if tmp___3 == 0 as libc::c_int {
        if (b"try: pthread_setspecific() failed\0" as *const u8 as *const libc::c_char)
            .is_null()
        {
            __assert_fail(
                b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3350 as libc::c_uint,
                b"outb_write\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        __assert_fail(
            b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                as *const u8 as *const libc::c_char,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            3350 as libc::c_uint,
            b"outb_write\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___4 = _setjmp((try_this_.env).as_mut_ptr());
    if tmp___4 == 0 as libc::c_int {
        loop {
            possess_(
                outb_write_more,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3352 as libc::c_long,
            );
            wait_for_(
                outb_write_more,
                TO_BE,
                1 as libc::c_long,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3353 as libc::c_long,
            );
            len = out_len;
            if len != 0 {
                if g.decode == 1 as libc::c_int {
                    writen(g.outd, out_copy.as_mut_ptr() as *const libc::c_void, len);
                }
            }
            twist_(
                outb_write_more,
                TO,
                0 as libc::c_long,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3358 as libc::c_long,
            );
            if len == 0 {
                break;
            }
        }
    }
    if try_pushed_ != 0 {
        tmp___8 = pthread_setspecific(try_key_, try_this_.next as *const libc::c_void);
        if tmp___8 == 0 as libc::c_int {
            if (b"try: pthread_setspecific() failed\0" as *const u8
                as *const libc::c_char)
                .is_null()
            {
                __assert_fail(
                    b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                        as *const u8 as *const libc::c_char,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    3361 as libc::c_uint,
                    b"outb_write\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            __assert_fail(
                b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3361 as libc::c_uint,
                b"outb_write\0" as *const u8 as *const libc::c_char,
            );
        }
        try_pushed_ = 0 as libc::c_int;
    }
    err = try_this_.ball;
    if err.code != 0 {
        if err.code != 32 as libc::c_int {
            complain(
                b"abort: %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                err.why,
            );
        }
        if err.free != 0 {
            free(err.why as *mut libc::c_void);
            err.free = 0 as libc::c_int;
            err.why = 0 as *mut libc::c_void as *mut libc::c_char;
        }
        cut_short(-err.code);
    }
}
unsafe extern "C" fn outb_check(mut dummy: *mut libc::c_void) {
    let mut len: size_t = 0;
    let mut err: try_ball_t_ = try_ball_t_ {
        ret: 0,
        code: 0,
        free: 0,
        why: 0 as *mut libc::c_char,
    };
    let mut try_this_: try_t_ = try_t_ {
        env: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
        ball: try_ball_t_ {
            ret: 0,
            code: 0,
            free: 0,
            why: 0 as *mut libc::c_char,
        },
        next: 0 as *mut try_t_,
    };
    let mut try_pushed_: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_ulong = 0;
    let mut tmp___5: libc::c_ulong = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    try_pushed_ = 1 as libc::c_int;
    try_this_.ball.ret = 0 as libc::c_int;
    try_this_.ball.code = 0 as libc::c_int;
    try_this_.ball.free = 0 as libc::c_int;
    try_this_.ball.why = 0 as *mut libc::c_void as *mut libc::c_char;
    try_setup_();
    tmp = pthread_getspecific(try_key_);
    try_this_.next = tmp as *mut try_t_;
    tmp___3 = pthread_setspecific(
        try_key_,
        &mut try_this_ as *mut try_t_ as *const libc::c_void,
    );
    if tmp___3 == 0 as libc::c_int {
        if (b"try: pthread_setspecific() failed\0" as *const u8 as *const libc::c_char)
            .is_null()
        {
            __assert_fail(
                b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3375 as libc::c_uint,
                b"outb_check\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        __assert_fail(
            b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                as *const u8 as *const libc::c_char,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            3375 as libc::c_uint,
            b"outb_check\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___6 = _setjmp((try_this_.env).as_mut_ptr());
    if tmp___6 == 0 as libc::c_int {
        loop {
            possess_(
                outb_check_more,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3377 as libc::c_long,
            );
            wait_for_(
                outb_check_more,
                TO_BE,
                1 as libc::c_long,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3378 as libc::c_long,
            );
            len = out_len;
            if g.form == 1 as libc::c_int {
                tmp___4 = adler32z(
                    g.out_check,
                    out_copy.as_mut_ptr() as *const libc::c_uchar,
                    len,
                );
                g.out_check = tmp___4;
            } else {
                tmp___5 = crc32z(
                    g.out_check,
                    out_copy.as_mut_ptr() as *const libc::c_uchar,
                    len,
                );
                g.out_check = tmp___5;
            }
            twist_(
                outb_check_more,
                TO,
                0 as libc::c_long,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3382 as libc::c_long,
            );
            if len == 0 {
                break;
            }
        }
    }
    if try_pushed_ != 0 {
        tmp___10 = pthread_setspecific(try_key_, try_this_.next as *const libc::c_void);
        if tmp___10 == 0 as libc::c_int {
            if (b"try: pthread_setspecific() failed\0" as *const u8
                as *const libc::c_char)
                .is_null()
            {
                __assert_fail(
                    b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                        as *const u8 as *const libc::c_char,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    3385 as libc::c_uint,
                    b"outb_check\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            __assert_fail(
                b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3385 as libc::c_uint,
                b"outb_check\0" as *const u8 as *const libc::c_char,
            );
        }
        try_pushed_ = 0 as libc::c_int;
    }
    err = try_this_.ball;
    if err.code != 0 {
        if err.code != 32 as libc::c_int {
            complain(
                b"abort: %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                err.why,
            );
        }
        if err.free != 0 {
            free(err.why as *mut libc::c_void);
            err.free = 0 as libc::c_int;
            err.why = 0 as *mut libc::c_void as *mut libc::c_char;
        }
        cut_short(-err.code);
    }
}
static mut wr: *mut thread = 0 as *const thread as *mut thread;
static mut ch: *mut thread = 0 as *const thread as *mut thread;
unsafe extern "C" fn outb(
    mut desc: *mut libc::c_void,
    mut buf: *mut libc::c_uchar,
    mut len: libc::c_uint,
) -> libc::c_int {
    let mut tmp: libc::c_ulong = 0;
    let mut tmp___0: libc::c_ulong = 0;
    if g.procs > 1 as libc::c_int {
        if outb_write_more as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            outb_write_more = new_lock_(
                0 as libc::c_long,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3405 as libc::c_long,
            );
            outb_check_more = new_lock_(
                0 as libc::c_long,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3406 as libc::c_long,
            );
            wr = launch_(
                Some(outb_write as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                0 as *mut libc::c_void,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3407 as libc::c_long,
            );
            ch = launch_(
                Some(outb_check as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                0 as *mut libc::c_void,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3408 as libc::c_long,
            );
        }
        possess_(
            outb_check_more,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            3412 as libc::c_long,
        );
        wait_for_(
            outb_check_more,
            TO_BE,
            0 as libc::c_long,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            3413 as libc::c_long,
        );
        possess_(
            outb_write_more,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            3414 as libc::c_long,
        );
        wait_for_(
            outb_write_more,
            TO_BE,
            0 as libc::c_long,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            3415 as libc::c_long,
        );
        out_len = len as size_t;
        g
            .out_tot = (g.out_tot as libc::c_ulong).wrapping_add(len as length_t)
            as length_t as length_t;
        memcpy(
            out_copy.as_mut_ptr() as *mut libc::c_void,
            buf as *const libc::c_void,
            len as size_t,
        );
        twist_(
            outb_write_more,
            TO,
            1 as libc::c_long,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            3421 as libc::c_long,
        );
        twist_(
            outb_check_more,
            TO,
            1 as libc::c_long,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            3422 as libc::c_long,
        );
        if len == 0 as libc::c_uint {
            if outb_write_more as libc::c_ulong
                != 0 as *mut libc::c_void as libc::c_ulong
            {
                join_(
                    ch,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    3427 as libc::c_long,
                );
                join_(
                    wr,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    3428 as libc::c_long,
                );
                free_lock_(
                    outb_check_more,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    3429 as libc::c_long,
                );
                free_lock_(
                    outb_write_more,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    3430 as libc::c_long,
                );
                outb_write_more = 0 as *mut libc::c_void as *mut lock;
            }
        }
        return 0 as libc::c_int;
    }
    if len != 0 {
        if g.decode == 1 as libc::c_int {
            writen(g.outd, buf as *const libc::c_void, len as size_t);
        }
        if g.form == 1 as libc::c_int {
            tmp = adler32z(g.out_check, buf as *const libc::c_uchar, len as size_t);
            g.out_check = tmp;
        } else {
            tmp___0 = crc32z(g.out_check, buf as *const libc::c_uchar, len as size_t);
            g.out_check = tmp___0;
        }
        g
            .out_tot = (g.out_tot as libc::c_ulong).wrapping_add(len as length_t)
            as length_t as length_t;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn infchk() {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0;
    let mut cont: libc::c_int = 0;
    let mut more: libc::c_int = 0;
    let mut check: libc::c_ulong = 0;
    let mut len: libc::c_ulong = 0;
    let mut ktot: libc::c_ulong = 0;
    let mut strm___0: z_stream = z_stream {
        next_in: 0 as *mut Bytef,
        avail_in: 0,
        total_in: 0,
        next_out: 0 as *mut Bytef,
        avail_out: 0,
        total_out: 0,
        msg: 0 as *mut libc::c_char,
        state: 0 as *mut internal_state,
        zalloc: None,
        zfree: None,
        opaque: 0 as *mut libc::c_void,
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    let mut tmp2: libc::c_uint = 0;
    let mut tmp4: libc::c_ulong = 0;
    let mut clen: length_t = 0;
    let mut ctot: length_t = 0;
    let mut utot: length_t = 0;
    let mut tmp: libc::c_ulong = 0;
    let mut tmp___0: libc::c_ulong = 0;
    let mut tmp___1: libc::c_ulong = 0;
    let mut tmp___2: libc::c_ulong = 0;
    let mut tmp___4: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___5: size_t = 0;
    let mut tmp___7: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: size_t = 0;
    let mut tmp___11: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___12: size_t = 0;
    let mut tmp___14: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___15: libc::c_int = 0;
    let mut tmp___16: size_t = 0;
    let mut tmp___18: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___19: size_t = 0;
    let mut tmp___21: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___22: libc::c_int = 0;
    let mut tmp___23: size_t = 0;
    let mut tmp___25: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___26: size_t = 0;
    let mut tmp___28: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___29: libc::c_int = 0;
    let mut tmp___30: size_t = 0;
    let mut tmp___32: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___33: size_t = 0;
    let mut tmp___35: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___36: libc::c_int = 0;
    let mut tmp___37: size_t = 0;
    let mut tmp___39: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___40: size_t = 0;
    let mut tmp___42: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___43: libc::c_int = 0;
    let mut tmp___44: size_t = 0;
    let mut tmp___46: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___47: size_t = 0;
    let mut tmp___49: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___50: libc::c_int = 0;
    let mut tmp___51: size_t = 0;
    let mut tmp___53: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___54: size_t = 0;
    let mut tmp___56: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___57: libc::c_int = 0;
    let mut tmp___58: size_t = 0;
    let mut tmp___59: length_t = 0;
    let mut tmp___61: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___62: size_t = 0;
    let mut tmp___64: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___65: libc::c_int = 0;
    let mut tmp___66: size_t = 0;
    let mut tmp___68: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___69: size_t = 0;
    let mut tmp___71: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___72: libc::c_int = 0;
    let mut tmp___73: size_t = 0;
    let mut tmp___75: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___76: size_t = 0;
    let mut tmp___78: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___79: libc::c_int = 0;
    let mut tmp___80: size_t = 0;
    let mut tmp___82: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___83: size_t = 0;
    let mut tmp___85: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___86: libc::c_int = 0;
    let mut tmp___87: size_t = 0;
    let mut tmp___89: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___90: libc::c_int = 0;
    let mut tmp___91: size_t = 0;
    let mut tmp___93: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___94: libc::c_int = 0;
    let mut tmp___95: size_t = 0;
    let mut tmp___97: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___98: libc::c_int = 0;
    let mut tmp___99: size_t = 0;
    let mut tmp___101: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___102: libc::c_int = 0;
    let mut tmp___103: size_t = 0;
    let mut tmp___105: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___106: size_t = 0;
    let mut tmp___108: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___109: libc::c_int = 0;
    let mut tmp___110: size_t = 0;
    let mut tmp___112: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___113: size_t = 0;
    let mut tmp___115: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___116: libc::c_int = 0;
    let mut tmp___117: size_t = 0;
    let mut tmp___119: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___120: size_t = 0;
    let mut tmp___122: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___123: libc::c_int = 0;
    let mut tmp___124: size_t = 0;
    let mut tmp___126: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___127: size_t = 0;
    let mut tmp___129: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___130: libc::c_int = 0;
    let mut tmp___131: size_t = 0;
    let mut tmp___132: libc::c_ulong = 0;
    let mut tmp___133: libc::c_ulong = 0;
    let mut tmp___134: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___135: size_t = 0;
    utot = 0 as libc::c_int as length_t;
    ctot = utot;
    if g.form == 1 as libc::c_int {
        tmp = adler32z(
            0 as libc::c_ulong,
            0 as *const libc::c_uchar,
            0 as libc::c_int as size_t,
        );
        ktot = tmp;
    } else {
        tmp___0 = crc32z(
            0 as libc::c_ulong,
            0 as *const libc::c_uchar,
            0 as libc::c_int as size_t,
        );
        ktot = tmp___0;
    }
    more = 0 as libc::c_int;
    cont = more;
    loop {
        g.in_tot = g.in_left;
        g.out_tot = 0 as libc::c_int as length_t;
        if g.form == 1 as libc::c_int {
            tmp___1 = adler32z(
                0 as libc::c_ulong,
                0 as *const libc::c_uchar,
                0 as libc::c_int as size_t,
            );
            g.out_check = tmp___1;
        } else {
            tmp___2 = crc32z(
                0 as libc::c_ulong,
                0 as *const libc::c_uchar,
                0 as libc::c_int as size_t,
            );
            g.out_check = tmp___2;
        }
        strm___0.zalloc = None;
        strm___0.zfree = None;
        strm___0.opaque = 0 as voidpf;
        ret = inflateBackInit_(
            &mut strm___0,
            15 as libc::c_int,
            out_buf.as_mut_ptr(),
            b"1.2.11\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
        );
        if ret == -(4 as libc::c_int) {
            try_throw_(
                12 as libc::c_int,
                b"not enough memory\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_void,
            );
        }
        if ret != 0 as libc::c_int {
            try_throw_(
                22 as libc::c_int,
                b"internal error\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_void,
            );
        }
        strm___0.avail_in = 0 as libc::c_int as uInt;
        strm___0.next_in = 0 as *mut Bytef;
        ret = inflateBack(
            &mut strm___0,
            Some(
                inb
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut *mut libc::c_uchar,
                    ) -> libc::c_uint,
            ),
            0 as *mut libc::c_void,
            Some(
                outb
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_uchar,
                        libc::c_uint,
                    ) -> libc::c_int,
            ),
            0 as *mut libc::c_void,
        );
        inflateBackEnd(&mut strm___0);
        if ret == -(3 as libc::c_int) {
            try_throw_(
                33 as libc::c_int,
                b"%s: corrupted -- invalid deflate data (%s)\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                g.inf,
                strm___0.msg,
                0 as *mut libc::c_void,
            );
        }
        if ret == -(5 as libc::c_int) {
            try_throw_(
                33 as libc::c_int,
                b"%s: corrupted -- incomplete deflate data\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                g.inf,
                0 as *mut libc::c_void,
            );
        }
        if ret != 1 as libc::c_int {
            try_throw_(
                22 as libc::c_int,
                b"internal error\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_void,
            );
        }
        g
            .in_left = (g.in_left as libc::c_ulong)
            .wrapping_add(strm___0.avail_in as size_t) as size_t as size_t;
        g.in_next = strm___0.next_in;
        outb(
            0 as *mut libc::c_void,
            0 as *mut libc::c_void as *mut libc::c_uchar,
            0 as libc::c_uint,
        );
        clen = (g.in_tot).wrapping_sub(g.in_left);
        if g.form > 1 as libc::c_int {
            if g.form == 3 as libc::c_int {
                if g.in_left == 0 as libc::c_ulong {
                    if g.in_eof != 0 {
                        tmp2 = 0 as libc::c_uint;
                    } else {
                        tmp___5 = load();
                        if tmp___5 == 0 as libc::c_ulong {
                            tmp2 = 0 as libc::c_uint;
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            tmp___4 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            tmp2 = *tmp___4 as libc::c_uint;
                        }
                    }
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___4 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp2 = *tmp___4 as libc::c_uint;
                }
                if g.in_left == 0 as libc::c_ulong {
                    if g.in_eof != 0 {
                        tmp___8 = 0 as libc::c_int;
                    } else {
                        tmp___9 = load();
                        if tmp___9 == 0 as libc::c_ulong {
                            tmp___8 = 0 as libc::c_int;
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            tmp___7 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            tmp___8 = *tmp___7 as libc::c_int;
                        }
                    }
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___7 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp___8 = *tmp___7 as libc::c_int;
                }
                tmp4 = tmp2.wrapping_add((tmp___8 as libc::c_uint) << 8 as libc::c_int)
                    as libc::c_ulong;
                if g.in_left == 0 as libc::c_ulong {
                    if g.in_eof != 0 {
                        tmp2 = 0 as libc::c_uint;
                    } else {
                        tmp___12 = load();
                        if tmp___12 == 0 as libc::c_ulong {
                            tmp2 = 0 as libc::c_uint;
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            tmp___11 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            tmp2 = *tmp___11 as libc::c_uint;
                        }
                    }
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___11 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp2 = *tmp___11 as libc::c_uint;
                }
                if g.in_left == 0 as libc::c_ulong {
                    if g.in_eof != 0 {
                        tmp___15 = 0 as libc::c_int;
                    } else {
                        tmp___16 = load();
                        if tmp___16 == 0 as libc::c_ulong {
                            tmp___15 = 0 as libc::c_int;
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            tmp___14 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            tmp___15 = *tmp___14 as libc::c_int;
                        }
                    }
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___14 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp___15 = *tmp___14 as libc::c_int;
                }
                g
                    .zip_crc = tmp4
                    .wrapping_add(
                        (tmp2
                            .wrapping_add((tmp___15 as libc::c_uint) << 8 as libc::c_int)
                            as libc::c_ulong) << 16 as libc::c_int,
                    );
                if g.in_left == 0 as libc::c_ulong {
                    if g.in_eof != 0 {
                        tmp2 = 0 as libc::c_uint;
                    } else {
                        tmp___19 = load();
                        if tmp___19 == 0 as libc::c_ulong {
                            tmp2 = 0 as libc::c_uint;
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            tmp___18 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            tmp2 = *tmp___18 as libc::c_uint;
                        }
                    }
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___18 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp2 = *tmp___18 as libc::c_uint;
                }
                if g.in_left == 0 as libc::c_ulong {
                    if g.in_eof != 0 {
                        tmp___22 = 0 as libc::c_int;
                    } else {
                        tmp___23 = load();
                        if tmp___23 == 0 as libc::c_ulong {
                            tmp___22 = 0 as libc::c_int;
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            tmp___21 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            tmp___22 = *tmp___21 as libc::c_int;
                        }
                    }
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___21 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp___22 = *tmp___21 as libc::c_int;
                }
                tmp4 = tmp2.wrapping_add((tmp___22 as libc::c_uint) << 8 as libc::c_int)
                    as libc::c_ulong;
                if g.in_left == 0 as libc::c_ulong {
                    if g.in_eof != 0 {
                        tmp2 = 0 as libc::c_uint;
                    } else {
                        tmp___26 = load();
                        if tmp___26 == 0 as libc::c_ulong {
                            tmp2 = 0 as libc::c_uint;
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            tmp___25 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            tmp2 = *tmp___25 as libc::c_uint;
                        }
                    }
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___25 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp2 = *tmp___25 as libc::c_uint;
                }
                if g.in_left == 0 as libc::c_ulong {
                    if g.in_eof != 0 {
                        tmp___29 = 0 as libc::c_int;
                    } else {
                        tmp___30 = load();
                        if tmp___30 == 0 as libc::c_ulong {
                            tmp___29 = 0 as libc::c_int;
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            tmp___28 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            tmp___29 = *tmp___28 as libc::c_int;
                        }
                    }
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___28 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp___29 = *tmp___28 as libc::c_int;
                }
                g
                    .zip_clen = tmp4
                    .wrapping_add(
                        (tmp2
                            .wrapping_add((tmp___29 as libc::c_uint) << 8 as libc::c_int)
                            as libc::c_ulong) << 16 as libc::c_int,
                    );
                if g.in_left == 0 as libc::c_ulong {
                    if g.in_eof != 0 {
                        tmp2 = 0 as libc::c_uint;
                    } else {
                        tmp___33 = load();
                        if tmp___33 == 0 as libc::c_ulong {
                            tmp2 = 0 as libc::c_uint;
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            tmp___32 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            tmp2 = *tmp___32 as libc::c_uint;
                        }
                    }
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___32 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp2 = *tmp___32 as libc::c_uint;
                }
                if g.in_left == 0 as libc::c_ulong {
                    if g.in_eof != 0 {
                        tmp___36 = 0 as libc::c_int;
                    } else {
                        tmp___37 = load();
                        if tmp___37 == 0 as libc::c_ulong {
                            tmp___36 = 0 as libc::c_int;
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            tmp___35 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            tmp___36 = *tmp___35 as libc::c_int;
                        }
                    }
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___35 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp___36 = *tmp___35 as libc::c_int;
                }
                tmp4 = tmp2.wrapping_add((tmp___36 as libc::c_uint) << 8 as libc::c_int)
                    as libc::c_ulong;
                if g.in_left == 0 as libc::c_ulong {
                    if g.in_eof != 0 {
                        tmp2 = 0 as libc::c_uint;
                    } else {
                        tmp___40 = load();
                        if tmp___40 == 0 as libc::c_ulong {
                            tmp2 = 0 as libc::c_uint;
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            tmp___39 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            tmp2 = *tmp___39 as libc::c_uint;
                        }
                    }
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___39 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp2 = *tmp___39 as libc::c_uint;
                }
                if g.in_left == 0 as libc::c_ulong {
                    if g.in_eof != 0 {
                        tmp___43 = 0 as libc::c_int;
                    } else {
                        tmp___44 = load();
                        if tmp___44 == 0 as libc::c_ulong {
                            tmp___43 = 0 as libc::c_int;
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            tmp___42 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            tmp___43 = *tmp___42 as libc::c_int;
                        }
                    }
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    tmp___42 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    tmp___43 = *tmp___42 as libc::c_int;
                }
                g
                    .zip_ulen = tmp4
                    .wrapping_add(
                        (tmp2
                            .wrapping_add((tmp___43 as libc::c_uint) << 8 as libc::c_int)
                            as libc::c_ulong) << 16 as libc::c_int,
                    );
                if g.zip_crc == 134695760 as libc::c_ulong {
                    if g.out_check != 134695760 as libc::c_ulong {
                        current_block = 16648942257431943315;
                    } else if g.zip_clen == 134695760 as libc::c_ulong {
                        if clen & 4294967295 as libc::c_ulong
                            != 134695760 as libc::c_ulong
                        {
                            current_block = 16648942257431943315;
                        } else if g.zip_ulen == 134695760 as libc::c_ulong {
                            if g.zip64 != 0 {
                                tmp___59 = clen >> 32 as libc::c_int;
                            } else {
                                tmp___59 = g.out_tot;
                            }
                            if tmp___59 != 134695760 as libc::c_ulong {
                                current_block = 16648942257431943315;
                            } else {
                                current_block = 15648110200289423331;
                            }
                        } else {
                            current_block = 15648110200289423331;
                        }
                    } else {
                        current_block = 15648110200289423331;
                    }
                    match current_block {
                        15648110200289423331 => {}
                        _ => {
                            g.zip_crc = g.zip_clen;
                            g.zip_clen = g.zip_ulen;
                            if g.in_left == 0 as libc::c_ulong {
                                if g.in_eof != 0 {
                                    tmp2 = 0 as libc::c_uint;
                                } else {
                                    tmp___47 = load();
                                    if tmp___47 == 0 as libc::c_ulong {
                                        tmp2 = 0 as libc::c_uint;
                                    } else {
                                        g.in_left = (g.in_left).wrapping_sub(1);
                                        tmp___46 = g.in_next;
                                        g.in_next = (g.in_next).offset(1);
                                        tmp2 = *tmp___46 as libc::c_uint;
                                    }
                                }
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                tmp___46 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                                tmp2 = *tmp___46 as libc::c_uint;
                            }
                            if g.in_left == 0 as libc::c_ulong {
                                if g.in_eof != 0 {
                                    tmp___50 = 0 as libc::c_int;
                                } else {
                                    tmp___51 = load();
                                    if tmp___51 == 0 as libc::c_ulong {
                                        tmp___50 = 0 as libc::c_int;
                                    } else {
                                        g.in_left = (g.in_left).wrapping_sub(1);
                                        tmp___49 = g.in_next;
                                        g.in_next = (g.in_next).offset(1);
                                        tmp___50 = *tmp___49 as libc::c_int;
                                    }
                                }
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                tmp___49 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                                tmp___50 = *tmp___49 as libc::c_int;
                            }
                            tmp4 = tmp2
                                .wrapping_add(
                                    (tmp___50 as libc::c_uint) << 8 as libc::c_int,
                                ) as libc::c_ulong;
                            if g.in_left == 0 as libc::c_ulong {
                                if g.in_eof != 0 {
                                    tmp2 = 0 as libc::c_uint;
                                } else {
                                    tmp___54 = load();
                                    if tmp___54 == 0 as libc::c_ulong {
                                        tmp2 = 0 as libc::c_uint;
                                    } else {
                                        g.in_left = (g.in_left).wrapping_sub(1);
                                        tmp___53 = g.in_next;
                                        g.in_next = (g.in_next).offset(1);
                                        tmp2 = *tmp___53 as libc::c_uint;
                                    }
                                }
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                tmp___53 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                                tmp2 = *tmp___53 as libc::c_uint;
                            }
                            if g.in_left == 0 as libc::c_ulong {
                                if g.in_eof != 0 {
                                    tmp___57 = 0 as libc::c_int;
                                } else {
                                    tmp___58 = load();
                                    if tmp___58 == 0 as libc::c_ulong {
                                        tmp___57 = 0 as libc::c_int;
                                    } else {
                                        g.in_left = (g.in_left).wrapping_sub(1);
                                        tmp___56 = g.in_next;
                                        g.in_next = (g.in_next).offset(1);
                                        tmp___57 = *tmp___56 as libc::c_int;
                                    }
                                }
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                tmp___56 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                                tmp___57 = *tmp___56 as libc::c_int;
                            }
                            g
                                .zip_ulen = tmp4
                                .wrapping_add(
                                    (tmp2
                                        .wrapping_add(
                                            (tmp___57 as libc::c_uint) << 8 as libc::c_int,
                                        ) as libc::c_ulong) << 16 as libc::c_int,
                                );
                        }
                    }
                }
                if g.zip64 != 0 {
                    if g.in_left == 0 as libc::c_ulong {
                        if g.in_eof != 0 {
                            tmp2 = 0 as libc::c_uint;
                        } else {
                            tmp___62 = load();
                            if tmp___62 == 0 as libc::c_ulong {
                                tmp2 = 0 as libc::c_uint;
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                tmp___61 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                                tmp2 = *tmp___61 as libc::c_uint;
                            }
                        }
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___61 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp2 = *tmp___61 as libc::c_uint;
                    }
                    if g.in_left == 0 as libc::c_ulong {
                        if g.in_eof != 0 {
                            tmp___65 = 0 as libc::c_int;
                        } else {
                            tmp___66 = load();
                            if tmp___66 == 0 as libc::c_ulong {
                                tmp___65 = 0 as libc::c_int;
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                tmp___64 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                                tmp___65 = *tmp___64 as libc::c_int;
                            }
                        }
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___64 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp___65 = *tmp___64 as libc::c_int;
                    }
                    tmp4 = tmp2
                        .wrapping_add((tmp___65 as libc::c_uint) << 8 as libc::c_int)
                        as libc::c_ulong;
                    if g.in_left == 0 as libc::c_ulong {
                        if g.in_eof != 0 {
                            tmp2 = 0 as libc::c_uint;
                        } else {
                            tmp___69 = load();
                            if tmp___69 == 0 as libc::c_ulong {
                                tmp2 = 0 as libc::c_uint;
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                tmp___68 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                                tmp2 = *tmp___68 as libc::c_uint;
                            }
                        }
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___68 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp2 = *tmp___68 as libc::c_uint;
                    }
                    if g.in_left == 0 as libc::c_ulong {
                        if g.in_eof != 0 {
                            tmp___72 = 0 as libc::c_int;
                        } else {
                            tmp___73 = load();
                            if tmp___73 == 0 as libc::c_ulong {
                                tmp___72 = 0 as libc::c_int;
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                tmp___71 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                                tmp___72 = *tmp___71 as libc::c_int;
                            }
                        }
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___71 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp___72 = *tmp___71 as libc::c_int;
                    }
                    g
                        .zip_ulen = tmp4
                        .wrapping_add(
                            (tmp2
                                .wrapping_add(
                                    (tmp___72 as libc::c_uint) << 8 as libc::c_int,
                                ) as libc::c_ulong) << 16 as libc::c_int,
                        );
                    if g.in_left == 0 as libc::c_ulong {
                        if g.in_eof != 0 {
                            tmp2 = 0 as libc::c_uint;
                        } else {
                            tmp___76 = load();
                            if tmp___76 == 0 as libc::c_ulong {
                                tmp2 = 0 as libc::c_uint;
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                tmp___75 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                                tmp2 = *tmp___75 as libc::c_uint;
                            }
                        }
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___75 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp2 = *tmp___75 as libc::c_uint;
                    }
                    if g.in_left == 0 as libc::c_ulong {
                        if g.in_eof != 0 {
                            tmp___79 = 0 as libc::c_int;
                        } else {
                            tmp___80 = load();
                            if tmp___80 == 0 as libc::c_ulong {
                                tmp___79 = 0 as libc::c_int;
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                tmp___78 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                                tmp___79 = *tmp___78 as libc::c_int;
                            }
                        }
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___78 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp___79 = *tmp___78 as libc::c_int;
                    }
                    tmp4 = tmp2
                        .wrapping_add((tmp___79 as libc::c_uint) << 8 as libc::c_int)
                        as libc::c_ulong;
                    if g.in_left == 0 as libc::c_ulong {
                        if g.in_eof != 0 {
                            tmp2 = 0 as libc::c_uint;
                        } else {
                            tmp___83 = load();
                            if tmp___83 == 0 as libc::c_ulong {
                                tmp2 = 0 as libc::c_uint;
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                tmp___82 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                                tmp2 = *tmp___82 as libc::c_uint;
                            }
                        }
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___82 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp2 = *tmp___82 as libc::c_uint;
                    }
                    if g.in_left == 0 as libc::c_ulong {
                        if g.in_eof != 0 {
                            tmp___86 = 0 as libc::c_int;
                        } else {
                            tmp___87 = load();
                            if tmp___87 == 0 as libc::c_ulong {
                                tmp___86 = 0 as libc::c_int;
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                tmp___85 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                                tmp___86 = *tmp___85 as libc::c_int;
                            }
                        }
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___85 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp___86 = *tmp___85 as libc::c_int;
                    }
                }
                if g.in_eof != 0 {
                    try_throw_(
                        33 as libc::c_int,
                        b"%s: corrupted entry -- missing trailer\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        g.inf,
                        0 as *mut libc::c_void,
                    );
                }
            }
            check = g.zip_crc;
            if check != g.out_check {
                try_throw_(
                    33 as libc::c_int,
                    b"%s: corrupted entry -- crc32 mismatch\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    g.inf,
                    0 as *mut libc::c_void,
                );
            }
            if g.zip_clen != clen & 4294967295 as libc::c_ulong {
                try_throw_(
                    33 as libc::c_int,
                    b"%s: corrupted entry -- length mismatch\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    g.inf,
                    0 as *mut libc::c_void,
                );
            } else {
                if g.zip_ulen != g.out_tot & 4294967295 as libc::c_ulong {
                    try_throw_(
                        33 as libc::c_int,
                        b"%s: corrupted entry -- length mismatch\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        g.inf,
                        0 as *mut libc::c_void,
                    );
                }
            }
            more = more_zip_entries();
        } else if g.form == 1 as libc::c_int {
            if g.in_left == 0 as libc::c_ulong {
                if g.in_eof != 0 {
                    tmp___90 = 0 as libc::c_int;
                } else {
                    tmp___91 = load();
                    if tmp___91 == 0 as libc::c_ulong {
                        tmp___90 = 0 as libc::c_int;
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___89 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp___90 = *tmp___89 as libc::c_int;
                    }
                }
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                tmp___89 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                tmp___90 = *tmp___89 as libc::c_int;
            }
            check = (tmp___90 as libc::c_ulong) << 24 as libc::c_int;
            if g.in_left == 0 as libc::c_ulong {
                if g.in_eof != 0 {
                    tmp___94 = 0 as libc::c_int;
                } else {
                    tmp___95 = load();
                    if tmp___95 == 0 as libc::c_ulong {
                        tmp___94 = 0 as libc::c_int;
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___93 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp___94 = *tmp___93 as libc::c_int;
                    }
                }
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                tmp___93 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                tmp___94 = *tmp___93 as libc::c_int;
            }
            check = check.wrapping_add((tmp___94 as libc::c_ulong) << 16 as libc::c_int);
            if g.in_left == 0 as libc::c_ulong {
                if g.in_eof != 0 {
                    tmp___98 = 0 as libc::c_int;
                } else {
                    tmp___99 = load();
                    if tmp___99 == 0 as libc::c_ulong {
                        tmp___98 = 0 as libc::c_int;
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___97 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp___98 = *tmp___97 as libc::c_int;
                    }
                }
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                tmp___97 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                tmp___98 = *tmp___97 as libc::c_int;
            }
            check = check
                .wrapping_add(
                    ((tmp___98 as libc::c_uint) << 8 as libc::c_int) as libc::c_ulong,
                );
            if g.in_left == 0 as libc::c_ulong {
                if g.in_eof != 0 {
                    tmp___102 = 0 as libc::c_int;
                } else {
                    tmp___103 = load();
                    if tmp___103 == 0 as libc::c_ulong {
                        tmp___102 = 0 as libc::c_int;
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___101 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp___102 = *tmp___101 as libc::c_int;
                    }
                }
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                tmp___101 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                tmp___102 = *tmp___101 as libc::c_int;
            }
            check = check.wrapping_add(tmp___102 as libc::c_ulong);
            if g.in_eof != 0 {
                try_throw_(
                    33 as libc::c_int,
                    b"%s: corrupted -- missing trailer\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    g.inf,
                    0 as *mut libc::c_void,
                );
            }
            if check != g.out_check {
                try_throw_(
                    33 as libc::c_int,
                    b"%s: corrupted -- adler32 mismatch\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    g.inf,
                    0 as *mut libc::c_void,
                );
            }
        } else {
            if g.in_left == 0 as libc::c_ulong {
                if g.in_eof != 0 {
                    tmp2 = 0 as libc::c_uint;
                } else {
                    tmp___106 = load();
                    if tmp___106 == 0 as libc::c_ulong {
                        tmp2 = 0 as libc::c_uint;
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___105 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp2 = *tmp___105 as libc::c_uint;
                    }
                }
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                tmp___105 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                tmp2 = *tmp___105 as libc::c_uint;
            }
            if g.in_left == 0 as libc::c_ulong {
                if g.in_eof != 0 {
                    tmp___109 = 0 as libc::c_int;
                } else {
                    tmp___110 = load();
                    if tmp___110 == 0 as libc::c_ulong {
                        tmp___109 = 0 as libc::c_int;
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___108 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp___109 = *tmp___108 as libc::c_int;
                    }
                }
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                tmp___108 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                tmp___109 = *tmp___108 as libc::c_int;
            }
            tmp4 = tmp2.wrapping_add((tmp___109 as libc::c_uint) << 8 as libc::c_int)
                as libc::c_ulong;
            if g.in_left == 0 as libc::c_ulong {
                if g.in_eof != 0 {
                    tmp2 = 0 as libc::c_uint;
                } else {
                    tmp___113 = load();
                    if tmp___113 == 0 as libc::c_ulong {
                        tmp2 = 0 as libc::c_uint;
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___112 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp2 = *tmp___112 as libc::c_uint;
                    }
                }
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                tmp___112 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                tmp2 = *tmp___112 as libc::c_uint;
            }
            if g.in_left == 0 as libc::c_ulong {
                if g.in_eof != 0 {
                    tmp___116 = 0 as libc::c_int;
                } else {
                    tmp___117 = load();
                    if tmp___117 == 0 as libc::c_ulong {
                        tmp___116 = 0 as libc::c_int;
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___115 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp___116 = *tmp___115 as libc::c_int;
                    }
                }
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                tmp___115 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                tmp___116 = *tmp___115 as libc::c_int;
            }
            check = tmp4
                .wrapping_add(
                    (tmp2.wrapping_add((tmp___116 as libc::c_uint) << 8 as libc::c_int)
                        as libc::c_ulong) << 16 as libc::c_int,
                );
            if g.in_left == 0 as libc::c_ulong {
                if g.in_eof != 0 {
                    tmp2 = 0 as libc::c_uint;
                } else {
                    tmp___120 = load();
                    if tmp___120 == 0 as libc::c_ulong {
                        tmp2 = 0 as libc::c_uint;
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___119 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp2 = *tmp___119 as libc::c_uint;
                    }
                }
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                tmp___119 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                tmp2 = *tmp___119 as libc::c_uint;
            }
            if g.in_left == 0 as libc::c_ulong {
                if g.in_eof != 0 {
                    tmp___123 = 0 as libc::c_int;
                } else {
                    tmp___124 = load();
                    if tmp___124 == 0 as libc::c_ulong {
                        tmp___123 = 0 as libc::c_int;
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___122 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp___123 = *tmp___122 as libc::c_int;
                    }
                }
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                tmp___122 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                tmp___123 = *tmp___122 as libc::c_int;
            }
            tmp4 = tmp2.wrapping_add((tmp___123 as libc::c_uint) << 8 as libc::c_int)
                as libc::c_ulong;
            if g.in_left == 0 as libc::c_ulong {
                if g.in_eof != 0 {
                    tmp2 = 0 as libc::c_uint;
                } else {
                    tmp___127 = load();
                    if tmp___127 == 0 as libc::c_ulong {
                        tmp2 = 0 as libc::c_uint;
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___126 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp2 = *tmp___126 as libc::c_uint;
                    }
                }
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                tmp___126 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                tmp2 = *tmp___126 as libc::c_uint;
            }
            if g.in_left == 0 as libc::c_ulong {
                if g.in_eof != 0 {
                    tmp___130 = 0 as libc::c_int;
                } else {
                    tmp___131 = load();
                    if tmp___131 == 0 as libc::c_ulong {
                        tmp___130 = 0 as libc::c_int;
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        tmp___129 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        tmp___130 = *tmp___129 as libc::c_int;
                    }
                }
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                tmp___129 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                tmp___130 = *tmp___129 as libc::c_int;
            }
            len = tmp4
                .wrapping_add(
                    (tmp2.wrapping_add((tmp___130 as libc::c_uint) << 8 as libc::c_int)
                        as libc::c_ulong) << 16 as libc::c_int,
                );
            if g.in_eof != 0 {
                try_throw_(
                    33 as libc::c_int,
                    b"%s: corrupted -- missing trailer\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    g.inf,
                    0 as *mut libc::c_void,
                );
            }
            if check != g.out_check {
                try_throw_(
                    33 as libc::c_int,
                    b"%s: corrupted -- crc32 mismatch\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    g.inf,
                    0 as *mut libc::c_void,
                );
            }
            if len != g.out_tot & 4294967295 as libc::c_ulong {
                try_throw_(
                    33 as libc::c_int,
                    b"%s: corrupted -- length mismatch\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    g.inf,
                    0 as *mut libc::c_void,
                );
            }
        }
        if g.list != 0 {
            ctot = (ctot as libc::c_ulong).wrapping_add(clen) as length_t as length_t;
            utot = (utot as libc::c_ulong).wrapping_add(g.out_tot) as length_t
                as length_t;
            if g.form == 1 as libc::c_int {
                tmp___132 = adler32_comb(ktot, check, g.out_tot);
                ktot = tmp___132;
            } else {
                tmp___133 = crc32_comb(ktot, check, g.out_tot);
                ktot = tmp___133;
            }
            g.in_tot = clen;
            show_info(8 as libc::c_int, check, g.out_tot, cont);
            if cont != 0 {
                cont = 2 as libc::c_int;
            } else {
                cont = 1 as libc::c_int;
            }
        }
        if !(g.form == 0 as libc::c_int) {
            break;
        }
        ret = get_header(0 as libc::c_int);
        if !(ret == 8 as libc::c_int) {
            break;
        }
    }
    if cont > 1 as libc::c_int {
        if g.verbosity > 0 as libc::c_int {
            if g.verbosity > 1 as libc::c_int {
                printf(
                    b"        %08lx                \0" as *const u8
                        as *const libc::c_char,
                    ktot,
                );
            }
            printf(
                b"%10ju %10ju %6.1f%%  (total)\n\0" as *const u8 as *const libc::c_char,
                ctot,
                utot,
                100.0f64 * (utot as libc::c_double - ctot as libc::c_double)
                    / utot as libc::c_double,
            );
        }
    }
    if g.form == 0 as libc::c_int {
        if ret == -(2 as libc::c_int) {
            if g.force != 0 {
                if g.pipeout != 0 {
                    if g.decode != 2 as libc::c_int {
                        if g.list == 0 {
                            cat();
                            current_block = 14932464801677330285;
                        } else {
                            current_block = 5696817722570867445;
                        }
                    } else {
                        current_block = 5696817722570867445;
                    }
                } else {
                    current_block = 5696817722570867445;
                }
            } else {
                current_block = 5696817722570867445;
            }
        } else {
            current_block = 5696817722570867445;
        }
    } else {
        current_block = 5696817722570867445;
    }
    match current_block {
        5696817722570867445 => {
            if more != 0 {
                complain(
                    b"warning: %s: entries after the first were ignored\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    g.inf,
                );
                g.keep = 1 as libc::c_int;
            } else {
                if g.verbosity > 1 as libc::c_int {
                    if g.form == 0 as libc::c_int {
                        if ret != -(1 as libc::c_int) {
                            complain(
                                b"warning: %s: trailing junk was ignored\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                                g.inf,
                            );
                            current_block = 14932464801677330285;
                        } else {
                            current_block = 5093820687953443677;
                        }
                    } else {
                        current_block = 5093820687953443677;
                    }
                } else {
                    current_block = 5093820687953443677;
                }
                match current_block {
                    14932464801677330285 => {}
                    _ => {
                        if g.form == 1 as libc::c_int {
                            if g.in_left == 0 as libc::c_ulong {
                                if g.in_eof == 0 {
                                    tmp___135 = load();
                                    if !(tmp___135 == 0 as libc::c_ulong) {
                                        g.in_left = (g.in_left).wrapping_sub(1);
                                        tmp___134 = g.in_next;
                                        g.in_next = (g.in_next).offset(1);
                                    }
                                }
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                tmp___134 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                            }
                            if g.in_eof == 0 {
                                complain(
                                    b"warning: %s: trailing junk was ignored\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                    g.inf,
                                );
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn unlzw() {
    let mut bits: libc::c_uint = 0;
    let mut mask: libc::c_uint = 0;
    let mut buf: bits_t = 0;
    let mut left: libc::c_uint = 0;
    let mut mark: length_t = 0;
    let mut code: libc::c_uint = 0;
    let mut max: libc::c_uint = 0;
    let mut flags: libc::c_uint = 0;
    let mut end: libc::c_uint = 0;
    let mut prev: libc::c_uint = 0;
    let mut final_0: libc::c_uint = 0;
    let mut stack: libc::c_uint = 0;
    let mut outcnt: libc::c_uint = 0;
    let mut prefix: [index_t; 65536] = [0; 65536];
    let mut suffix: [libc::c_uchar; 65536] = [0; 65536];
    let mut match_0: [libc::c_uchar; 65282] = [0; 65282];
    let mut tmp: size_t = 0;
    let mut tmp___0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___3: size_t = 0;
    let mut tmp___4: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut rem: libc::c_uint = 0;
    let mut tmp___5: size_t = 0;
    let mut tmp___6: size_t = 0;
    let mut tmp___7: size_t = 0;
    let mut tmp___8: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___9: size_t = 0;
    let mut tmp___10: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut rem___0: libc::c_uint = 0;
    let mut tmp___11: size_t = 0;
    let mut temp: libc::c_uint = 0;
    let mut tmp___12: libc::c_uint = 0;
    let mut tmp___13: libc::c_uint = 0;
    let mut tmp___14: libc::c_uint = 0;
    let mut tmp___15: libc::c_uint = 0;
    let mut tmp___16: libc::c_uint = 0;
    g.out_tot = 0 as libc::c_int as length_t;
    if g.in_left == 0 as libc::c_ulong {
        if g.in_eof != 0 {
            try_throw_(
                33 as libc::c_int,
                b"%s: lzw premature end\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                g.inf,
                0 as *mut libc::c_void,
            );
        } else {
            tmp = load();
            if tmp == 0 as libc::c_ulong {
                try_throw_(
                    33 as libc::c_int,
                    b"%s: lzw premature end\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    g.inf,
                    0 as *mut libc::c_void,
                );
            }
        }
    }
    g.in_left = (g.in_left).wrapping_sub(1);
    tmp___0 = g.in_next;
    g.in_next = (g.in_next).offset(1);
    flags = *tmp___0 as libc::c_uint;
    if flags & 96 as libc::c_uint != 0 {
        try_throw_(
            33 as libc::c_int,
            b"%s: unknown lzw flags set\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            g.inf,
            0 as *mut libc::c_void,
        );
    }
    max = flags & 31 as libc::c_uint;
    if max < 9 as libc::c_uint {
        try_throw_(
            33 as libc::c_int,
            b"%s: lzw bits out of range\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            g.inf,
            0 as *mut libc::c_void,
        );
    } else {
        if max > 16 as libc::c_uint {
            try_throw_(
                33 as libc::c_int,
                b"%s: lzw bits out of range\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                g.inf,
                0 as *mut libc::c_void,
            );
        }
    }
    if max == 9 as libc::c_uint {
        max = 10 as libc::c_uint;
    }
    flags &= 128 as libc::c_uint;
    mark = (g.in_tot).wrapping_sub(g.in_left);
    bits = 9 as libc::c_uint;
    mask = 511 as libc::c_uint;
    if flags != 0 {
        end = 256 as libc::c_uint;
    } else {
        end = 255 as libc::c_uint;
    }
    if g.in_left == 0 as libc::c_ulong {
        if g.in_eof != 0 {
            return
        } else {
            tmp___1 = load();
            if tmp___1 == 0 as libc::c_ulong {
                return;
            }
        }
    }
    g.in_left = (g.in_left).wrapping_sub(1);
    tmp___2 = g.in_next;
    g.in_next = (g.in_next).offset(1);
    buf = *tmp___2 as libc::c_uint as bits_t;
    if g.in_left == 0 as libc::c_ulong {
        if g.in_eof != 0 {
            try_throw_(
                33 as libc::c_int,
                b"%s: lzw premature end\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                g.inf,
                0 as *mut libc::c_void,
            );
        } else {
            tmp___3 = load();
            if tmp___3 == 0 as libc::c_ulong {
                try_throw_(
                    33 as libc::c_int,
                    b"%s: lzw premature end\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    g.inf,
                    0 as *mut libc::c_void,
                );
            }
        }
    }
    g.in_left = (g.in_left).wrapping_sub(1);
    tmp___4 = g.in_next;
    g.in_next = (g.in_next).offset(1);
    buf = (buf as libc::c_ulong)
        .wrapping_add(((*tmp___4 as libc::c_uint) << 8 as libc::c_int) as bits_t)
        as bits_t as bits_t;
    prev = (buf & mask as libc::c_ulong) as libc::c_uint;
    final_0 = prev;
    buf >>= bits;
    left = (16 as libc::c_uint).wrapping_sub(bits);
    if prev > 255 as libc::c_uint {
        try_throw_(
            33 as libc::c_int,
            b"%s: invalid lzw code\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            g.inf,
            0 as *mut libc::c_void,
        );
    }
    out_buf[0 as libc::c_int as usize] = final_0 as libc::c_uchar;
    outcnt = 1 as libc::c_uint;
    stack = 0 as libc::c_uint;
    loop {
        if end >= mask {
            if bits < max {
                rem = (g.in_tot)
                    .wrapping_sub(g.in_left)
                    .wrapping_sub(mark)
                    .wrapping_rem(bits as libc::c_ulong) as libc::c_uint;
                if rem != 0 {
                    rem = bits.wrapping_sub(rem);
                    if g.in_left == 0 as libc::c_ulong {
                        if g.in_eof != 0 {
                            break;
                        }
                        tmp___5 = load();
                        if tmp___5 == 0 as libc::c_ulong {
                            break;
                        }
                    }
                    while rem as size_t > g.in_left {
                        rem = (rem as size_t).wrapping_sub(g.in_left) as libc::c_uint;
                        tmp___6 = load();
                        if tmp___6 == 0 as libc::c_ulong {
                            try_throw_(
                                33 as libc::c_int,
                                b"%s: lzw premature end\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                                g.inf,
                                0 as *mut libc::c_void,
                            );
                        }
                    }
                    g
                        .in_left = (g.in_left as libc::c_ulong)
                        .wrapping_sub(rem as size_t) as size_t as size_t;
                    g.in_next = (g.in_next).offset(rem as isize);
                }
                buf = 0 as libc::c_int as bits_t;
                left = 0 as libc::c_uint;
                mark = (g.in_tot).wrapping_sub(g.in_left);
                bits = bits.wrapping_add(1);
                mask <<= 1 as libc::c_int;
                mask = mask.wrapping_add(1);
            }
        }
        if g.in_left == 0 as libc::c_ulong {
            if g.in_eof != 0 {
                break;
            }
            tmp___7 = load();
            if tmp___7 == 0 as libc::c_ulong {
                break;
            }
        }
        g.in_left = (g.in_left).wrapping_sub(1);
        tmp___8 = g.in_next;
        g.in_next = (g.in_next).offset(1);
        buf = (buf as libc::c_ulong)
            .wrapping_add((*tmp___8 as libc::c_uint as bits_t) << left) as bits_t
            as bits_t;
        left = left.wrapping_add(8 as libc::c_uint);
        if left < bits {
            if g.in_left == 0 as libc::c_ulong {
                if g.in_eof != 0 {
                    try_throw_(
                        33 as libc::c_int,
                        b"%s: lzw premature end\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        g.inf,
                        0 as *mut libc::c_void,
                    );
                } else {
                    tmp___9 = load();
                    if tmp___9 == 0 as libc::c_ulong {
                        try_throw_(
                            33 as libc::c_int,
                            b"%s: lzw premature end\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            g.inf,
                            0 as *mut libc::c_void,
                        );
                    }
                }
            }
            g.in_left = (g.in_left).wrapping_sub(1);
            tmp___10 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            buf = (buf as libc::c_ulong)
                .wrapping_add((*tmp___10 as libc::c_uint as bits_t) << left) as bits_t
                as bits_t;
            left = left.wrapping_add(8 as libc::c_uint);
        }
        code = (buf & mask as libc::c_ulong) as libc::c_uint;
        buf >>= bits;
        left = left.wrapping_sub(bits);
        if code == 256 as libc::c_uint {
            if flags != 0 {
                rem___0 = (g.in_tot)
                    .wrapping_sub(g.in_left)
                    .wrapping_sub(mark)
                    .wrapping_rem(bits as libc::c_ulong) as libc::c_uint;
                if rem___0 != 0 {
                    rem___0 = bits.wrapping_sub(rem___0);
                    while rem___0 as size_t > g.in_left {
                        rem___0 = (rem___0 as size_t).wrapping_sub(g.in_left)
                            as libc::c_uint;
                        tmp___11 = load();
                        if tmp___11 == 0 as libc::c_ulong {
                            try_throw_(
                                33 as libc::c_int,
                                b"%s: lzw premature end\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                                g.inf,
                                0 as *mut libc::c_void,
                            );
                        }
                    }
                    g
                        .in_left = (g.in_left as libc::c_ulong)
                        .wrapping_sub(rem___0 as size_t) as size_t as size_t;
                    g.in_next = (g.in_next).offset(rem___0 as isize);
                }
                buf = 0 as libc::c_int as bits_t;
                left = 0 as libc::c_uint;
                mark = (g.in_tot).wrapping_sub(g.in_left);
                bits = 9 as libc::c_uint;
                mask = 511 as libc::c_uint;
                end = 255 as libc::c_uint;
                continue;
            }
        }
        temp = code;
        if code > end {
            if code != end.wrapping_add(1 as libc::c_uint) {
                try_throw_(
                    33 as libc::c_int,
                    b"%s: invalid lzw code\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    g.inf,
                    0 as *mut libc::c_void,
                );
            } else {
                if prev > end {
                    try_throw_(
                        33 as libc::c_int,
                        b"%s: invalid lzw code\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        g.inf,
                        0 as *mut libc::c_void,
                    );
                }
            }
            tmp___12 = stack;
            stack = stack.wrapping_add(1);
            match_0[tmp___12 as usize] = final_0 as libc::c_uchar;
            code = prev;
        }
        while code >= 256 as libc::c_uint {
            tmp___13 = stack;
            stack = stack.wrapping_add(1);
            match_0[tmp___13 as usize] = suffix[code as usize];
            code = prefix[code as usize] as libc::c_uint;
        }
        tmp___14 = stack;
        stack = stack.wrapping_add(1);
        match_0[tmp___14 as usize] = code as libc::c_uchar;
        final_0 = code;
        if end < mask {
            end = end.wrapping_add(1);
            prefix[end as usize] = prev as index_t;
            suffix[end as usize] = final_0 as libc::c_uchar;
        }
        prev = temp;
        while stack > (32768 as libc::c_uint).wrapping_sub(outcnt) {
            while outcnt < 32768 as libc::c_uint {
                tmp___15 = outcnt;
                outcnt = outcnt.wrapping_add(1);
                stack = stack.wrapping_sub(1);
                out_buf[tmp___15 as usize] = match_0[stack as usize];
            }
            g
                .out_tot = (g.out_tot as libc::c_ulong).wrapping_add(outcnt as length_t)
                as length_t as length_t;
            if g.decode == 1 as libc::c_int {
                writen(
                    g.outd,
                    out_buf.as_mut_ptr() as *const libc::c_void,
                    outcnt as size_t,
                );
            }
            outcnt = 0 as libc::c_uint;
        }
        loop {
            tmp___16 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            stack = stack.wrapping_sub(1);
            out_buf[tmp___16 as usize] = match_0[stack as usize];
            if stack == 0 {
                break;
            }
        }
    }
    g
        .out_tot = (g.out_tot as libc::c_ulong).wrapping_add(outcnt as length_t)
        as length_t as length_t;
    if outcnt != 0 {
        if g.decode == 1 as libc::c_int {
            writen(
                g.outd,
                out_buf.as_mut_ptr() as *const libc::c_void,
                outcnt as size_t,
            );
        }
    }
}
unsafe extern "C" fn justname(mut path: *mut libc::c_char) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    p = strrchr(path as *const libc::c_char, '/' as i32);
    if p as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp = path;
    } else {
        tmp = p.offset(1 as libc::c_int as isize);
    }
    return tmp;
}
unsafe extern "C" fn copymeta(
    mut from: *mut libc::c_char,
    mut to: *mut libc::c_char,
) -> libc::c_int {
    let mut st: stat = stat {
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
    let mut times: [timeval; 2] = [timeval { tv_sec: 0, tv_usec: 0 }; 2];
    let mut tmp: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    tmp = stat(from as *const libc::c_char, &mut st as *mut stat);
    if tmp != 0 as libc::c_int {
        return -(4 as libc::c_int)
    } else {
        if st.st_mode & 61440 as libc::c_uint != 32768 as libc::c_uint {
            return -(4 as libc::c_int);
        }
    }
    tmp___0 = chmod(to as *const libc::c_char, st.st_mode & 4095 as libc::c_uint);
    ret = tmp___0;
    tmp___1 = chown(to as *const libc::c_char, st.st_uid, st.st_gid);
    ret += tmp___1;
    times[0 as libc::c_int as usize].tv_sec = st.st_atim.tv_sec;
    times[0 as libc::c_int as usize].tv_usec = 0 as libc::c_int as __suseconds_t;
    times[1 as libc::c_int as usize].tv_sec = st.st_mtim.tv_sec;
    times[1 as libc::c_int as usize].tv_usec = 0 as libc::c_int as __suseconds_t;
    tmp___2 = utimes(to as *const libc::c_char, times.as_mut_ptr() as *const timeval);
    ret += tmp___2;
    return ret;
}
unsafe extern "C" fn touch(mut path: *mut libc::c_char, mut t: time_t) {
    let mut times: [timeval; 2] = [timeval { tv_sec: 0, tv_usec: 0 }; 2];
    times[0 as libc::c_int as usize].tv_sec = t;
    times[0 as libc::c_int as usize].tv_usec = 0 as libc::c_int as __suseconds_t;
    times[1 as libc::c_int as usize].tv_sec = t;
    times[1 as libc::c_int as usize].tv_usec = 0 as libc::c_int as __suseconds_t;
    utimes(path as *const libc::c_char, times.as_mut_ptr() as *const timeval);
}
unsafe extern "C" fn out_push() {
    let mut ret: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    if g.outd == -(1 as libc::c_int) {
        return;
    }
    tmp = fsync(g.outd);
    ret = tmp;
    if ret == -(1 as libc::c_int) {
        tmp___0 = __errno_location();
        tmp___1 = strerror(*tmp___0);
        tmp___2 = __errno_location();
        try_throw_(
            *tmp___2,
            b"sync error on %s (%s)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            g.outf,
            tmp___1,
            0 as *mut libc::c_void,
        );
    }
}
static mut sufs: [*mut libc::c_char; 12] = [
    b".z\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"-z\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"_z\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b".Z\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b".gz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"-gz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b".zz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"-zz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b".zip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b".ZIP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b".tgz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
];
unsafe extern "C" fn process(mut path: *mut libc::c_char) {
    let mut current_block: u64;
    let mut method: libc::c_int = 0;
    let mut len: size_t = 0;
    let mut st: stat = stat {
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
    let mut err: try_ball_t_ = try_ball_t_ {
        ret: 0,
        code: 0,
        free: 0,
        why: 0 as *mut libc::c_char,
    };
    let mut tmp___0: time_t = 0;
    let mut tmp___1: time_t = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut sufx: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut tmp___3: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut tmp___4: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___7: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___8: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___9: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___10: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___11: libc::c_int = 0;
    let mut roll: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: size_t = 0;
    let mut off: size_t = 0;
    let mut base: size_t = 0;
    let mut here: *mut DIR = 0 as *mut DIR;
    let mut next___0: *mut dirent = 0 as *mut dirent;
    let mut tmp___12: size_t = 0;
    let mut tmp___13: size_t = 0;
    let mut tmp___14: size_t = 0;
    let mut tmp___15: size_t = 0;
    let mut tmp___16: libc::c_int = 0;
    let mut suf: size_t = 0;
    let mut tmp___17: size_t = 0;
    let mut tmp___18: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___19: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___20: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___21: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___22: libc::c_int = 0;
    let mut tmp___23: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___24: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___25: *const libc::c_char = 0 as *const libc::c_char;
    let mut try_this_: try_t_ = try_t_ {
        env: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
        ball: try_ball_t_ {
            ret: 0,
            code: 0,
            free: 0,
            why: 0 as *mut libc::c_char,
        },
        next: 0 as *mut try_t_,
    };
    let mut try_pushed_: libc::c_int = 0;
    let mut tmp___26: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___30: libc::c_int = 0;
    let mut tmp___31: libc::c_int = 0;
    let mut tmp___35: libc::c_int = 0;
    let mut tmp___39: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___40: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___41: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___42: size_t = 0;
    let mut tmp___43: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___44: libc::c_int = 0;
    let mut to: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sufx___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pre: size_t = 0;
    let mut tmp___45: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___46: libc::c_int = 0;
    let mut tmp___47: size_t = 0;
    let mut tmp___48: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___49: libc::c_int = 0;
    let mut overwrite: libc::c_int = 0;
    let mut ch___0: libc::c_int = 0;
    let mut first: libc::c_int = 0;
    let mut tmp___50: libc::c_int = 0;
    let mut tmp___51: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___52: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___53: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___54: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut try_this____0: try_t_ = try_t_ {
        env: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
        ball: try_ball_t_ {
            ret: 0,
            code: 0,
            free: 0,
            why: 0 as *mut libc::c_char,
        },
        next: 0 as *mut try_t_,
    };
    let mut try_pushed____0: libc::c_int = 0;
    let mut tmp___55: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___59: libc::c_int = 0;
    let mut tmp___60: libc::c_int = 0;
    let mut tmp___64: libc::c_int = 0;
    let mut tmp___68: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___69: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___70: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___71: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___72: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___73: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___74: libc::c_int = 0;
    method = -(1 as libc::c_int);
    if path as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        vstrcpy(
            &mut g.inf,
            &mut g.inz,
            0 as libc::c_int as size_t,
            b"<stdin>\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
        );
        g.ind = 0 as libc::c_int;
        g.name = 0 as *mut libc::c_void as *mut libc::c_char;
        if g.headis & 2 as libc::c_int != 0 {
            tmp___2 = fstat(g.ind, &mut st);
            if tmp___2 != 0 {
                tmp___0 = time(0 as *mut libc::c_void as *mut time_t);
                tmp___1 = tmp___0;
            } else {
                tmp___1 = st.st_mtim.tv_sec;
            }
            g.mtime = tmp___1;
        } else {
            g.mtime = 0 as libc::c_int as time_t;
        }
        len = 0 as libc::c_int as size_t;
    } else {
        if path as libc::c_ulong != g.inf as libc::c_ulong {
            vstrcpy(
                &mut g.inf,
                &mut g.inz,
                0 as libc::c_int as size_t,
                path as *mut libc::c_void,
            );
        }
        len = strlen(g.inf as *const libc::c_char);
        tmp___11 = lstat(g.inf as *const libc::c_char, &mut st as *mut stat);
        if tmp___11 != 0 {
            tmp___7 = __errno_location();
            if *tmp___7 == 2 as libc::c_int {
                let mut current_block_32: u64;
                if g.list != 0 {
                    current_block_32 = 1057557327206608274;
                } else if g.decode != 0 {
                    current_block_32 = 1057557327206608274;
                } else {
                    current_block_32 = 13125627826496529465;
                }
                match current_block_32 {
                    1057557327206608274 => {
                        sufx = sufs.as_mut_ptr();
                        while !(*sufx as libc::c_ulong
                            == 0 as *mut libc::c_void as libc::c_ulong)
                        {
                            tmp___3 = sufx;
                            sufx = sufx.offset(1);
                            vstrcpy(
                                &mut g.inf,
                                &mut g.inz,
                                len,
                                *tmp___3 as *mut libc::c_void,
                            );
                            tmp___4 = __errno_location();
                            *tmp___4 = 0 as libc::c_int;
                            tmp___5 = lstat(
                                g.inf as *const libc::c_char,
                                &mut st as *mut stat,
                            );
                            if !(tmp___5 != 0) {
                                break;
                            }
                            tmp___6 = __errno_location();
                            if !(*tmp___6 == 2 as libc::c_int) {
                                break;
                            }
                        }
                    }
                    _ => {}
                }
            }
            tmp___8 = __errno_location();
            if *tmp___8 == 75 as libc::c_int {
                try_throw_(
                    33 as libc::c_int,
                    b"%s too large -- not compiled with large file support\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                    g.inf,
                    0 as *mut libc::c_void,
                );
            } else {
                tmp___9 = __errno_location();
                if *tmp___9 == 27 as libc::c_int {
                    try_throw_(
                        33 as libc::c_int,
                        b"%s too large -- not compiled with large file support\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                        g.inf,
                        0 as *mut libc::c_void,
                    );
                }
            }
            tmp___10 = __errno_location();
            if *tmp___10 != 0 {
                *(g.inf).offset(len as isize) = 0 as libc::c_int as libc::c_char;
                complain(
                    b"skipping: %s does not exist\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    g.inf,
                );
                return;
            }
            len = strlen(g.inf as *const libc::c_char);
        }
        if st.st_mode & 61440 as libc::c_uint != 32768 as libc::c_uint {
            if st.st_mode & 61440 as libc::c_uint != 4096 as libc::c_uint {
                if st.st_mode & 61440 as libc::c_uint != 40960 as libc::c_uint {
                    if st.st_mode & 61440 as libc::c_uint != 16384 as libc::c_uint {
                        complain(
                            b"skipping: %s is a special file or device\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            g.inf,
                        );
                        return;
                    }
                }
            }
        }
        if st.st_mode & 61440 as libc::c_uint == 40960 as libc::c_uint {
            if g.force == 0 {
                if g.pipeout == 0 {
                    complain(
                        b"skipping: %s is a symbolic link\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        g.inf,
                    );
                    return;
                }
            }
        }
        if st.st_mode & 61440 as libc::c_uint == 16384 as libc::c_uint {
            if g.recurse == 0 {
                complain(
                    b"skipping: %s is a directory\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    g.inf,
                );
                return;
            }
        }
        if st.st_mode & 61440 as libc::c_uint == 16384 as libc::c_uint {
            roll = 0 as *mut libc::c_void as *mut libc::c_char;
            size = 0 as libc::c_int as size_t;
            off = 0 as libc::c_int as size_t;
            here = opendir(g.inf as *const libc::c_char);
            if here as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                return;
            }
            loop {
                next___0 = readdir(here);
                if !(next___0 as libc::c_ulong
                    != 0 as *mut libc::c_void as libc::c_ulong)
                {
                    break;
                }
                if (*next___0).d_name[0 as libc::c_int as usize] as libc::c_int
                    == 0 as libc::c_int
                {
                    continue;
                }
                if (*next___0).d_name[0 as libc::c_int as usize] as libc::c_int
                    == 46 as libc::c_int
                {
                    if (*next___0).d_name[1 as libc::c_int as usize] as libc::c_int
                        == 0 as libc::c_int
                    {
                        continue;
                    }
                    if (*next___0).d_name[1 as libc::c_int as usize] as libc::c_int
                        == 46 as libc::c_int
                    {
                        if (*next___0).d_name[2 as libc::c_int as usize] as libc::c_int
                            == 0 as libc::c_int
                        {
                            continue;
                        }
                    }
                }
                off = vstrcpy(
                    &mut roll,
                    &mut size,
                    off,
                    ((*next___0).d_name).as_mut_ptr() as *mut libc::c_void,
                );
            }
            closedir(here);
            vstrcpy(
                &mut roll,
                &mut size,
                off,
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
            );
            if len != 0 {
                if *(g.inf).offset(len.wrapping_sub(1 as libc::c_ulong) as isize)
                    as libc::c_int != 47 as libc::c_int
                {
                    tmp___12 = vstrcpy(
                        &mut g.inf,
                        &mut g.inz,
                        len,
                        b"/\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
                    );
                    base = tmp___12.wrapping_sub(1 as libc::c_ulong);
                } else {
                    base = len;
                }
            } else {
                base = len;
            }
            off = 0 as libc::c_int as size_t;
            while *roll.offset(off as isize) != 0 {
                vstrcpy(
                    &mut g.inf,
                    &mut g.inz,
                    base,
                    roll.offset(off as isize) as *mut libc::c_void,
                );
                process(g.inf);
                tmp___13 = strlen(roll.offset(off as isize) as *const libc::c_char);
                off = (off as libc::c_ulong)
                    .wrapping_add(tmp___13.wrapping_add(1 as libc::c_ulong)) as size_t
                    as size_t;
            }
            *(g.inf).offset(len as isize) = 0 as libc::c_int as libc::c_char;
            free(roll as *mut libc::c_void);
            return;
        }
        if g.force == 0 {
            if g.list == 0 {
                if g.decode == 0 {
                    tmp___14 = strlen(g.sufx as *const libc::c_char);
                    if len >= tmp___14 {
                        tmp___15 = strlen(g.sufx as *const libc::c_char);
                        tmp___16 = strcmp(
                            (g.inf).offset(len as isize).offset(-(tmp___15 as isize))
                                as *const libc::c_char,
                            g.sufx as *const libc::c_char,
                        );
                        if tmp___16 == 0 as libc::c_int {
                            grumble(
                                b"skipping: %s ends with %s\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                                g.inf,
                                g.sufx,
                            );
                            return;
                        }
                    }
                }
            }
        }
        if g.decode == 1 as libc::c_int {
            if g.pipeout == 0 {
                if g.list == 0 {
                    tmp___17 = compressed_suffix(g.inf);
                    suf = tmp___17;
                    if suf == 0 as libc::c_ulong {
                        complain(
                            b"skipping: %s does not have compressed suffix\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                            g.inf,
                        );
                        return;
                    }
                    len = (len as libc::c_ulong).wrapping_sub(suf) as size_t as size_t;
                }
            }
        }
        g.ind = open(g.inf as *const libc::c_char, 0 as libc::c_int, 0 as libc::c_int);
        if g.ind < 0 as libc::c_int {
            tmp___18 = __errno_location();
            tmp___19 = strerror(*tmp___18);
            tmp___20 = __errno_location();
            try_throw_(
                *tmp___20,
                b"read error on %s (%s)\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                g.inf,
                tmp___19,
                0 as *mut libc::c_void,
            );
        }
        if g.headis & 1 as libc::c_int != 0 {
            tmp___21 = justname(g.inf);
            g.name = tmp___21;
        } else {
            g.name = 0 as *mut libc::c_void as *mut libc::c_char;
        }
        if g.headis & 2 as libc::c_int != 0 {
            g.mtime = st.st_mtim.tv_sec;
        } else {
            g.mtime = 0 as libc::c_int as time_t;
        }
    }
    if g.list != 0 {
        if g.decode != 2 as libc::c_int {
            list_info();
            load_end();
            return;
        }
    }
    if g.decode != 0 {
        in_init();
        tmp___22 = get_header(1 as libc::c_int);
        method = tmp___22;
        if method != 8 as libc::c_int {
            if method != 257 as libc::c_int {
                if method == -(1 as libc::c_int) {
                    current_block = 5597915028669338483;
                } else if method == -(2 as libc::c_int) {
                    current_block = 5597915028669338483;
                } else {
                    current_block = 15797053017066231880;
                }
                match current_block {
                    5597915028669338483 => {
                        if g.force != 0 {
                            if g.pipeout != 0 {
                                if g.decode != 2 as libc::c_int {
                                    if g.list != 0 {
                                        current_block = 15797053017066231880;
                                    } else {
                                        current_block = 14880025327978413542;
                                    }
                                } else {
                                    current_block = 15797053017066231880;
                                }
                            } else {
                                current_block = 15797053017066231880;
                            }
                        } else {
                            current_block = 15797053017066231880;
                        }
                    }
                    _ => {}
                }
                match current_block {
                    14880025327978413542 => {}
                    _ => {
                        load_end();
                        if method == -(6 as libc::c_int) {
                            tmp___25 = b"skipping: %s corrupt: header crc error\0"
                                as *const u8 as *const libc::c_char;
                        } else {
                            if method == -(1 as libc::c_int) {
                                tmp___24 = b"skipping: %s empty\0" as *const u8
                                    as *const libc::c_char;
                            } else {
                                if method < 0 as libc::c_int {
                                    tmp___23 = b"skipping: %s unrecognized format\0"
                                        as *const u8 as *const libc::c_char;
                                } else {
                                    tmp___23 = b"skipping: %s unknown compression method\0"
                                        as *const u8 as *const libc::c_char;
                                }
                                tmp___24 = tmp___23;
                            }
                            tmp___25 = tmp___24;
                        }
                        complain(tmp___25 as *mut libc::c_char, g.inf);
                        return;
                    }
                }
            }
        }
        if g.decode == 2 as libc::c_int {
            try_pushed_ = 1 as libc::c_int;
            try_this_.ball.ret = 0 as libc::c_int;
            try_this_.ball.code = 0 as libc::c_int;
            try_this_.ball.free = 0 as libc::c_int;
            try_this_.ball.why = 0 as *mut libc::c_void as *mut libc::c_char;
            try_setup_();
            tmp___26 = pthread_getspecific(try_key_);
            try_this_.next = tmp___26 as *mut try_t_;
            tmp___30 = pthread_setspecific(
                try_key_,
                &mut try_this_ as *mut try_t_ as *const libc::c_void,
            );
            if tmp___30 == 0 as libc::c_int {
                if (b"try: pthread_setspecific() failed\0" as *const u8
                    as *const libc::c_char)
                    .is_null()
                {
                    __assert_fail(
                        b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                            as *const u8 as *const libc::c_char,
                        b"pigz.c\0" as *const u8 as *const libc::c_char,
                        4068 as libc::c_uint,
                        b"process\0" as *const u8 as *const libc::c_char,
                    );
                }
            } else {
                __assert_fail(
                    b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                        as *const u8 as *const libc::c_char,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    4068 as libc::c_uint,
                    b"process\0" as *const u8 as *const libc::c_char,
                );
            }
            tmp___31 = _setjmp((try_this_.env).as_mut_ptr());
            if tmp___31 == 0 as libc::c_int {
                if method == 8 as libc::c_int {
                    infchk();
                } else {
                    unlzw();
                    if g.list != 0 {
                        g
                            .in_tot = (g.in_tot as libc::c_ulong)
                            .wrapping_sub(3 as libc::c_ulong) as length_t as length_t;
                        show_info(
                            method,
                            0 as libc::c_ulong,
                            g.out_tot,
                            0 as libc::c_int,
                        );
                    }
                }
            }
            if try_pushed_ != 0 {
                tmp___35 = pthread_setspecific(
                    try_key_,
                    try_this_.next as *const libc::c_void,
                );
                if tmp___35 == 0 as libc::c_int {
                    if (b"try: pthread_setspecific() failed\0" as *const u8
                        as *const libc::c_char)
                        .is_null()
                    {
                        __assert_fail(
                            b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                                as *const u8 as *const libc::c_char,
                            b"pigz.c\0" as *const u8 as *const libc::c_char,
                            4079 as libc::c_uint,
                            b"process\0" as *const u8 as *const libc::c_char,
                        );
                    }
                } else {
                    __assert_fail(
                        b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                            as *const u8 as *const libc::c_char,
                        b"pigz.c\0" as *const u8 as *const libc::c_char,
                        4079 as libc::c_uint,
                        b"process\0" as *const u8 as *const libc::c_char,
                    );
                }
                try_pushed_ = 0 as libc::c_int;
            }
            err = try_this_.ball;
            if err.code != 0 {
                if err.code != 33 as libc::c_int {
                    try_setup_();
                    tmp___39 = pthread_getspecific(try_key_);
                    if tmp___39 as *mut try_t_ as libc::c_ulong
                        != 0 as *mut libc::c_void as libc::c_ulong
                    {
                        if (b"try: naked punt\0" as *const u8 as *const libc::c_char)
                            .is_null()
                        {
                            __assert_fail(
                                b"try_stack_ != NULL && \"try: naked punt\"\0" as *const u8
                                    as *const libc::c_char,
                                b"pigz.c\0" as *const u8 as *const libc::c_char,
                                4081 as libc::c_uint,
                                b"process\0" as *const u8 as *const libc::c_char,
                            );
                        }
                    } else {
                        __assert_fail(
                            b"try_stack_ != NULL && \"try: naked punt\"\0" as *const u8
                                as *const libc::c_char,
                            b"pigz.c\0" as *const u8 as *const libc::c_char,
                            4081 as libc::c_uint,
                            b"process\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    tmp___40 = pthread_getspecific(try_key_);
                    (*(tmp___40 as *mut try_t_)).ball = err;
                    tmp___41 = pthread_getspecific(try_key_);
                    longjmp(
                        ((*(tmp___41 as *mut try_t_)).env).as_mut_ptr(),
                        1 as libc::c_int,
                    );
                }
                complain(
                    b"skipping: %s\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    err.why,
                );
                if err.free != 0 {
                    free(err.why as *mut libc::c_void);
                    err.free = 0 as libc::c_int;
                    err.why = 0 as *mut libc::c_void as *mut libc::c_char;
                }
                outb(
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void as *mut libc::c_uchar,
                    0 as libc::c_uint,
                );
            }
            load_end();
            return;
        }
    }
    if path as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        current_block = 15280446612767579470;
    } else if g.pipeout != 0 {
        current_block = 15280446612767579470;
    } else {
        to = g.inf;
        sufx___0 = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        pre = 0 as libc::c_int as size_t;
        if g.decode != 0 {
            let mut current_block_286: u64;
            if g.headis & 1 as libc::c_int != 0 as libc::c_int {
                if g.hname as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                    tmp___45 = justname(g.inf);
                    pre = tmp___45.offset_from(g.inf) as libc::c_long as size_t;
                    to = justname(g.hname);
                    len = strlen(to as *const libc::c_char);
                    current_block_286 = 5349685387690872341;
                } else {
                    current_block_286 = 18365960652321767921;
                }
            } else {
                current_block_286 = 18365960652321767921;
            }
            match current_block_286 {
                18365960652321767921 => {
                    tmp___46 = strcmp(
                        to.offset(len as isize) as *const libc::c_char,
                        b".tgz\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___46 == 0 as libc::c_int {
                        sufx___0 = b".tar\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                }
                _ => {}
            }
        } else {
            sufx___0 = g.sufx;
        }
        tmp___47 = strlen(sufx___0 as *const libc::c_char);
        tmp___48 = alloc(
            0 as *mut libc::c_void,
            pre.wrapping_add(len).wrapping_add(tmp___47).wrapping_add(1 as libc::c_ulong),
        );
        g.outf = tmp___48 as *mut libc::c_char;
        memcpy(g.outf as *mut libc::c_void, g.inf as *const libc::c_void, pre);
        memcpy(
            (g.outf).offset(pre as isize) as *mut libc::c_void,
            to as *const libc::c_void,
            len,
        );
        strcpy(
            (g.outf).offset(pre as isize).offset(len as isize),
            sufx___0 as *const libc::c_char,
        );
        if g.force != 0 {
            tmp___49 = 0 as libc::c_int;
        } else {
            tmp___49 = 128 as libc::c_int;
        }
        g
            .outd = open(
            g.outf as *const libc::c_char,
            577 as libc::c_int | tmp___49,
            384 as libc::c_int,
        );
        if g.outd < 0 as libc::c_int {
            tmp___51 = __errno_location();
            if *tmp___51 == 17 as libc::c_int {
                overwrite = 0 as libc::c_int;
                tmp___50 = isatty(0 as libc::c_int);
                if tmp___50 != 0 {
                    if g.verbosity != 0 {
                        fprintf(
                            stderr,
                            b"%s exists -- overwrite (y/n)? \0" as *const u8
                                as *const libc::c_char,
                            g.outf,
                        );
                        fflush(stderr);
                        first = 1 as libc::c_int;
                        loop {
                            ch___0 = getchar();
                            if first == 1 as libc::c_int {
                                if !(ch___0 == 32 as libc::c_int) {
                                    if !(ch___0 == 9 as libc::c_int) {
                                        if ch___0 == 121 as libc::c_int {
                                            overwrite = 1 as libc::c_int;
                                        } else if ch___0 == 89 as libc::c_int {
                                            overwrite = 1 as libc::c_int;
                                        }
                                        first = 0 as libc::c_int;
                                    }
                                }
                            }
                            if !(ch___0 != -(1 as libc::c_int)) {
                                break;
                            }
                            if !(ch___0 != 10 as libc::c_int) {
                                break;
                            }
                            if !(ch___0 != 13 as libc::c_int) {
                                break;
                            }
                        }
                    }
                }
                if overwrite == 0 {
                    complain(
                        b"skipping: %s exists\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        g.outf,
                    );
                    if g.outf as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong
                    {
                        free(g.outf as *mut libc::c_void);
                        g.outf = 0 as *mut libc::c_void as *mut libc::c_char;
                    }
                    load_end();
                    return;
                }
                g
                    .outd = open(
                    g.outf as *const libc::c_char,
                    577 as libc::c_int,
                    384 as libc::c_int,
                );
            }
        }
        if g.outd < 0 as libc::c_int {
            tmp___52 = __errno_location();
            tmp___53 = strerror(*tmp___52);
            tmp___54 = __errno_location();
            try_throw_(
                *tmp___54,
                b"write error on %s (%s)\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                g.outf,
                tmp___53,
                0 as *mut libc::c_void,
            );
        }
        current_block = 10648164479545198704;
    }
    match current_block {
        15280446612767579470 => {
            tmp___42 = strlen(b"<stdout>\0" as *const u8 as *const libc::c_char);
            tmp___43 = alloc(
                0 as *mut libc::c_void,
                tmp___42.wrapping_add(1 as libc::c_ulong),
            );
            g.outf = tmp___43 as *mut libc::c_char;
            strcpy(g.outf, b"<stdout>\0" as *const u8 as *const libc::c_char);
            g.outd = 1 as libc::c_int;
            if g.decode == 0 {
                if g.force == 0 {
                    tmp___44 = isatty(g.outd);
                    if tmp___44 != 0 {
                        try_throw_(
                            22 as libc::c_int,
                            b"trying to write compressed data to a terminal (use -f to force)\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                            0 as *mut libc::c_void,
                        );
                    }
                }
            }
        }
        _ => {}
    }
    if g.verbosity > 1 as libc::c_int {
        fprintf(
            stderr,
            b"%s to %s \0" as *const u8 as *const libc::c_char,
            g.inf,
            g.outf,
        );
    }
    if g.decode != 0 {
        try_pushed____0 = 1 as libc::c_int;
        try_this____0.ball.ret = 0 as libc::c_int;
        try_this____0.ball.code = 0 as libc::c_int;
        try_this____0.ball.free = 0 as libc::c_int;
        try_this____0.ball.why = 0 as *mut libc::c_void as *mut libc::c_char;
        try_setup_();
        tmp___55 = pthread_getspecific(try_key_);
        try_this____0.next = tmp___55 as *mut try_t_;
        tmp___59 = pthread_setspecific(
            try_key_,
            &mut try_this____0 as *mut try_t_ as *const libc::c_void,
        );
        if tmp___59 == 0 as libc::c_int {
            if (b"try: pthread_setspecific() failed\0" as *const u8
                as *const libc::c_char)
                .is_null()
            {
                __assert_fail(
                    b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                        as *const u8 as *const libc::c_char,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    4170 as libc::c_uint,
                    b"process\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            __assert_fail(
                b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                4170 as libc::c_uint,
                b"process\0" as *const u8 as *const libc::c_char,
            );
        }
        tmp___60 = _setjmp((try_this____0.env).as_mut_ptr());
        if tmp___60 == 0 as libc::c_int {
            if method == 8 as libc::c_int {
                infchk();
            } else if method == 257 as libc::c_int {
                unlzw();
            } else {
                cat();
            }
        }
        if try_pushed____0 != 0 {
            tmp___64 = pthread_setspecific(
                try_key_,
                try_this____0.next as *const libc::c_void,
            );
            if tmp___64 == 0 as libc::c_int {
                if (b"try: pthread_setspecific() failed\0" as *const u8
                    as *const libc::c_char)
                    .is_null()
                {
                    __assert_fail(
                        b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                            as *const u8 as *const libc::c_char,
                        b"pigz.c\0" as *const u8 as *const libc::c_char,
                        4178 as libc::c_uint,
                        b"process\0" as *const u8 as *const libc::c_char,
                    );
                }
            } else {
                __assert_fail(
                    b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                        as *const u8 as *const libc::c_char,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    4178 as libc::c_uint,
                    b"process\0" as *const u8 as *const libc::c_char,
                );
            }
            try_pushed____0 = 0 as libc::c_int;
        }
        err = try_this____0.ball;
        if err.code != 0 {
            if err.code != 33 as libc::c_int {
                try_setup_();
                tmp___68 = pthread_getspecific(try_key_);
                if tmp___68 as *mut try_t_ as libc::c_ulong
                    != 0 as *mut libc::c_void as libc::c_ulong
                {
                    if (b"try: naked punt\0" as *const u8 as *const libc::c_char)
                        .is_null()
                    {
                        __assert_fail(
                            b"try_stack_ != NULL && \"try: naked punt\"\0" as *const u8
                                as *const libc::c_char,
                            b"pigz.c\0" as *const u8 as *const libc::c_char,
                            4180 as libc::c_uint,
                            b"process\0" as *const u8 as *const libc::c_char,
                        );
                    }
                } else {
                    __assert_fail(
                        b"try_stack_ != NULL && \"try: naked punt\"\0" as *const u8
                            as *const libc::c_char,
                        b"pigz.c\0" as *const u8 as *const libc::c_char,
                        4180 as libc::c_uint,
                        b"process\0" as *const u8 as *const libc::c_char,
                    );
                }
                tmp___69 = pthread_getspecific(try_key_);
                (*(tmp___69 as *mut try_t_)).ball = err;
                tmp___70 = pthread_getspecific(try_key_);
                longjmp(
                    ((*(tmp___70 as *mut try_t_)).env).as_mut_ptr(),
                    1 as libc::c_int,
                );
            }
            complain(
                b"skipping: %s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                err.why,
            );
            if err.free != 0 {
                free(err.why as *mut libc::c_void);
                err.free = 0 as libc::c_int;
                err.why = 0 as *mut libc::c_void as *mut libc::c_char;
            }
            outb(
                0 as *mut libc::c_void,
                0 as *mut libc::c_void as *mut libc::c_uchar,
                0 as libc::c_uint,
            );
            if g.outd != -(1 as libc::c_int) {
                if g.outd != 1 as libc::c_int {
                    close(g.outd);
                    g.outd = -(1 as libc::c_int);
                    unlink(g.outf as *const libc::c_char);
                    if g.outf as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong
                    {
                        free(g.outf as *mut libc::c_void);
                        g.outf = 0 as *mut libc::c_void as *mut libc::c_char;
                    }
                }
            }
        }
    } else if g.procs > 1 as libc::c_int {
        parallel_compress();
    } else {
        single_compress(0 as libc::c_int);
    }
    if g.verbosity > 1 as libc::c_int {
        putc('\n' as i32, stderr);
        fflush(stderr);
    }
    load_end();
    if g.outd != -(1 as libc::c_int) {
        if g.outd != 1 as libc::c_int {
            if g.sync != 0 {
                out_push();
            }
            tmp___74 = close(g.outd);
            if tmp___74 != 0 {
                tmp___71 = __errno_location();
                tmp___72 = strerror(*tmp___71);
                tmp___73 = __errno_location();
                try_throw_(
                    *tmp___73,
                    b"write error on %s (%s)\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    g.outf,
                    tmp___72,
                    0 as *mut libc::c_void,
                );
            }
            g.outd = -(1 as libc::c_int);
            if g.ind != 0 as libc::c_int {
                copymeta(g.inf, g.outf);
                if g.keep == 0 {
                    if st.st_nlink > 1 as libc::c_ulong {
                        if g.force == 0 {
                            complain(
                                b"%s has hard links -- not unlinking\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                                g.inf,
                            );
                        } else {
                            unlink(g.inf as *const libc::c_char);
                        }
                    } else {
                        unlink(g.inf as *const libc::c_char);
                    }
                }
            }
            if g.decode != 0 {
                if g.headis & 2 as libc::c_int != 0 as libc::c_int {
                    if g.stamp != 0 {
                        touch(g.outf, g.stamp);
                    }
                }
            }
        }
    }
    if g.outf as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        free(g.outf as *mut libc::c_void);
        g.outf = 0 as *mut libc::c_void as *mut libc::c_char;
    }
}
static mut helptext: [*mut libc::c_char; 42] = [
    b"Usage: pigz [options] [files ...]\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"  will compress files in place, adding the suffix '.gz'. If no files are\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  specified, stdin will be compressed to stdout. pigz does what gzip does,\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  but spreads the work over multiple processors and cores when compressing.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Options:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  -0 to -9, -11        Compression level (level 11, zopfli, is much slower)\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  --fast, --best       Compression levels 1 and 9 respectively\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -A, --alias xxx      Use xxx as the name for any --zip entry from stdin\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  -b, --blocksize mmm  Set compression block size to mmmK (default 128K)\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  -c, --stdout         Write all processed output to stdout (won't delete)\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  -C, --comment ccc    Put comment ccc in the gzip or zip header\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -d, --decompress     Decompress the compressed input\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -f, --force          Force overwrite, compress .gz, links, and to terminal\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  -F  --first          Do iterations first, before block split for -11\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  -h, --help           Display a help screen and quit\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -H, --huffman        Use only Huffman coding for compression\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -i, --independent    Compress blocks independently for damage recovery\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  -I, --iterations n   Number of iterations for -11 optimization\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -J, --maxsplits n    Maximum number of split blocks for -11\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -k, --keep           Do not delete original file after processing\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -K, --zip            Compress to PKWare zip (.zip) single entry format\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  -l, --list           List the contents of the compressed input\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -L, --license        Display the pigz license and quit\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -m, --no-time        Do not store or restore mod time\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -M, --time           Store or restore mod time\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -n, --no-name        Do not store or restore file name or mod time\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  -N, --name           Store or restore file name and mod time\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -O  --oneblock       Do not split into smaller blocks for -11\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -p, --processes n    Allow up to n compression threads (default is the\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"                       number of online processors, or 8 if unknown)\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  -q, --quiet          Print no messages, even on error\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -r, --recursive      Process the contents of all subdirectories\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -R, --rsyncable      Input-determined block locations for rsync\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -S, --suffix .sss    Use suffix .sss instead of .gz (for compression)\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  -t, --test           Test the integrity of the compressed input\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -U, --rle            Use run-length encoding for compression\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -v, --verbose        Provide more verbose output\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -V  --version        Show the version of pigz\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -Y  --synchronous    Force output file write to permanent storage\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -z, --zlib           Compress to zlib (.zz) instead of gzip format\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  --                   All arguments after \"--\" are treated as files\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
];
unsafe extern "C" fn help() {
    let mut n: libc::c_int = 0;
    if g.verbosity == 0 as libc::c_int {
        return;
    }
    n = 0 as libc::c_int;
    while n
        < (::std::mem::size_of::<[*mut libc::c_char; 42]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            as libc::c_int
    {
        fprintf(
            stderr,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            helptext[n as usize],
        );
        n += 1;
    }
    fflush(stderr);
    exit(0 as libc::c_int);
}
unsafe extern "C" fn nprocs(mut n: libc::c_int) -> libc::c_int {
    let mut tmp: libc::c_long = 0;
    tmp = sysconf(84 as libc::c_int);
    n = tmp as libc::c_int;
    return n;
}
unsafe extern "C" fn defaults() {
    g.level = -(1 as libc::c_int);
    g.strategy = 0 as libc::c_int;
    ZopfliInitOptions(&mut g.zopts);
    g.block = 131072 as libc::c_ulong;
    g.procs = nprocs(8 as libc::c_int);
    g.shift = x2nmodp(g.block, 3 as libc::c_uint);
    g.rsync = 0 as libc::c_int;
    g.setdict = 1 as libc::c_int;
    g.verbosity = 1 as libc::c_int;
    g.headis = 3 as libc::c_int;
    g.pipeout = 0 as libc::c_int;
    g.sufx = b".gz\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    g.comment = 0 as *mut libc::c_void as *mut libc::c_char;
    g.decode = 0 as libc::c_int;
    g.list = 0 as libc::c_int;
    g.keep = 0 as libc::c_int;
    g.force = 0 as libc::c_int;
    g.sync = 0 as libc::c_int;
    g.recurse = 0 as libc::c_int;
    g.form = 0 as libc::c_int;
}
static mut longopts: [[*mut libc::c_char; 2]; 41] = [
    [
        b"LZW\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Z\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"lzw\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Z\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"alias\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"A\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"ascii\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"best\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"bits\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Z\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"blocksize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"b\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"decompress\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"fast\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"force\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"comment\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"C\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"first\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"F\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"iterations\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"I\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"maxsplits\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"J\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"oneblock\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"O\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"help\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"h\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"independent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"keep\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"k\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"license\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"L\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"list\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"l\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"name\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"N\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"no-name\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"no-time\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"m\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"processes\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"p\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"quiet\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"q\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"recursive\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"rsyncable\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"R\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"silent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"q\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"stdout\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"suffix\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"S\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"synchronous\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Y\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"test\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"t\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"time\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"M\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"to-stdout\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"uncompress\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"verbose\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"v\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"version\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"V\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"zip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"K\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"zlib\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"z\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"huffman\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"H\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"rle\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"U\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
];
unsafe extern "C" fn new_opts() {
    single_compress(1 as libc::c_int);
    finish_jobs();
}
unsafe extern "C" fn num(mut arg: *mut libc::c_char) -> size_t {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: size_t = 0;
    str = arg;
    val = 0 as libc::c_int as size_t;
    if *str as libc::c_int == 0 as libc::c_int {
        try_throw_(
            22 as libc::c_int,
            b"internal error: empty parameter\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *mut libc::c_void,
        );
    }
    loop {
        if (*str as libc::c_int) < 48 as libc::c_int {
            try_throw_(
                22 as libc::c_int,
                b"invalid numeric parameter: %s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                arg,
                0 as *mut libc::c_void,
            );
        } else {
            if *str as libc::c_int > 57 as libc::c_int {
                try_throw_(
                    22 as libc::c_int,
                    b"invalid numeric parameter: %s\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    arg,
                    0 as *mut libc::c_void,
                );
            } else {
                if val != 0 {
                    if (18446744073709551615 as libc::c_ulonglong)
                        .wrapping_sub(
                            (*str as libc::c_int - 48 as libc::c_int) as size_t
                                as libc::c_ulonglong,
                        )
                        .wrapping_div(val as libc::c_ulonglong)
                        < 10 as libc::c_ulong as libc::c_ulonglong
                    {
                        try_throw_(
                            22 as libc::c_int,
                            b"invalid numeric parameter: %s\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            arg,
                            0 as *mut libc::c_void,
                        );
                    }
                }
            }
        }
        val = val
            .wrapping_mul(10 as libc::c_ulong)
            .wrapping_add((*str as libc::c_int - 48 as libc::c_int) as size_t);
        str = str.offset(1);
        if *str == 0 {
            break;
        }
    }
    return val;
}
static mut get: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn option(mut arg: *mut libc::c_char) -> libc::c_int {
    let mut bad: [libc::c_char; 3] = [0; 3];
    let mut j: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut n: size_t = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: size_t = 0;
    bad[0 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    bad[1 as libc::c_int as usize] = 'X' as i32 as libc::c_char;
    bad[2 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    if get != 0 {
        if arg as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            bad[1 as libc::c_int
                as usize] = *(b"bpSIJAC\0" as *const u8 as *const libc::c_char)
                .offset((get - 1 as libc::c_int) as isize);
            try_throw_(
                22 as libc::c_int,
                b"missing parameter after %s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                bad.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        } else {
            if *arg as libc::c_int == 45 as libc::c_int {
                bad[1 as libc::c_int
                    as usize] = *(b"bpSIJAC\0" as *const u8 as *const libc::c_char)
                    .offset((get - 1 as libc::c_int) as isize);
                try_throw_(
                    22 as libc::c_int,
                    b"missing parameter after %s\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    bad.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
            }
        }
    }
    if arg as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 1 as libc::c_int;
    }
    if *arg as libc::c_int == 45 as libc::c_int {
        arg = arg.offset(1);
        if *arg as libc::c_int == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        if *arg as libc::c_int == 45 as libc::c_int {
            arg = arg.offset(1);
            j = (::std::mem::size_of::<[[*mut libc::c_char; 2]; 41]>() as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                        << 1 as libc::c_int,
                )
                .wrapping_sub(1 as libc::c_ulong) as libc::c_int;
            while j >= 0 as libc::c_int {
                tmp = strcmp(
                    arg as *const libc::c_char,
                    longopts[j as usize][0 as libc::c_int as usize]
                        as *const libc::c_char,
                );
                if tmp == 0 as libc::c_int {
                    arg = longopts[j as usize][1 as libc::c_int as usize];
                    break;
                } else {
                    j -= 1;
                }
            }
            if j < 0 as libc::c_int {
                try_throw_(
                    22 as libc::c_int,
                    b"invalid option: %s\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    arg.offset(-(2 as libc::c_int as isize)),
                    0 as *mut libc::c_void,
                );
            }
        }
        loop {
            if get != 0 {
                if get == 3 as libc::c_int {
                    try_throw_(
                        22 as libc::c_int,
                        b"invalid usage: -S must be followed by space\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        0 as *mut libc::c_void,
                    );
                }
                if get == 7 as libc::c_int {
                    try_throw_(
                        22 as libc::c_int,
                        b"invalid usage: -C must be followed by space\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        0 as *mut libc::c_void,
                    );
                }
                break;
            } else {
                bad[1 as libc::c_int as usize] = *arg;
                match *arg as libc::c_int {
                    57 | 56 | 55 | 54 | 53 | 52 | 51 | 50 | 49 | 48 => {
                        g.level = *arg as libc::c_int - 48 as libc::c_int;
                        while *arg.offset(1 as libc::c_int as isize) as libc::c_int
                            >= 48 as libc::c_int
                        {
                            if !(*arg.offset(1 as libc::c_int as isize) as libc::c_int
                                <= 57 as libc::c_int)
                            {
                                break;
                            }
                            if g.level != 0 {
                                if (2147483647 as libc::c_int
                                    - (*arg.offset(1 as libc::c_int as isize) as libc::c_int
                                        - 48 as libc::c_int)) / g.level < 10 as libc::c_int
                                {
                                    try_throw_(
                                        22 as libc::c_int,
                                        b"only levels 0..9 and 11 are allowed\0" as *const u8
                                            as *const libc::c_char as *mut libc::c_char,
                                        0 as *mut libc::c_void,
                                    );
                                }
                            }
                            arg = arg.offset(1);
                            g
                                .level = g.level * 10 as libc::c_int + *arg as libc::c_int
                                - 48 as libc::c_int;
                        }
                        if g.level == 10 as libc::c_int {
                            try_throw_(
                                22 as libc::c_int,
                                b"only levels 0..9 and 11 are allowed\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                                0 as *mut libc::c_void,
                            );
                        } else {
                            if g.level > 11 as libc::c_int {
                                try_throw_(
                                    22 as libc::c_int,
                                    b"only levels 0..9 and 11 are allowed\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                    0 as *mut libc::c_void,
                                );
                            }
                        }
                    }
                    65 => {
                        get = 6 as libc::c_int;
                    }
                    67 => {
                        get = 7 as libc::c_int;
                    }
                    70 => {
                        g.zopts.blocksplittinglast = 1 as libc::c_int;
                    }
                    72 => {
                        g.strategy = 2 as libc::c_int;
                    }
                    73 => {
                        get = 4 as libc::c_int;
                    }
                    74 => {
                        get = 5 as libc::c_int;
                    }
                    75 => {
                        g.form = 2 as libc::c_int;
                        g
                            .sufx = b".zip\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    76 => {
                        puts(b"pigz 2.7\0" as *const u8 as *const libc::c_char);
                        puts(
                            b"Copyright (C) 2007-2022 Mark Adler\0" as *const u8
                                as *const libc::c_char,
                        );
                        puts(
                            b"Subject to the terms of the zlib license.\0" as *const u8
                                as *const libc::c_char,
                        );
                        puts(
                            b"No warranty is provided or implied.\0" as *const u8
                                as *const libc::c_char,
                        );
                        exit(0 as libc::c_int);
                    }
                    77 => {
                        g.headis |= 10 as libc::c_int;
                    }
                    78 => {
                        g.headis = 15 as libc::c_int;
                    }
                    79 => {
                        g.zopts.blocksplitting = 0 as libc::c_int;
                    }
                    82 => {
                        g.rsync = 1 as libc::c_int;
                    }
                    83 => {
                        get = 3 as libc::c_int;
                    }
                    86 => {
                        puts(b"pigz 2.7\0" as *const u8 as *const libc::c_char);
                        if g.verbosity > 1 as libc::c_int {
                            tmp___0 = zlibVersion();
                            printf(
                                b"zlib %s\n\0" as *const u8 as *const libc::c_char,
                                tmp___0,
                            );
                        }
                        exit(0 as libc::c_int);
                    }
                    89 => {
                        g.sync = 1 as libc::c_int;
                    }
                    90 => {
                        try_throw_(
                            22 as libc::c_int,
                            b"invalid option: LZW output not supported: %s\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                            bad.as_mut_ptr(),
                            0 as *mut libc::c_void,
                        );
                    }
                    97 => {
                        try_throw_(
                            22 as libc::c_int,
                            b"invalid option: no ascii conversion: %s\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            bad.as_mut_ptr(),
                            0 as *mut libc::c_void,
                        );
                    }
                    98 => {
                        get = 1 as libc::c_int;
                    }
                    99 => {
                        g.pipeout = 1 as libc::c_int;
                    }
                    100 => {
                        if g.decode == 0 {
                            g.headis >>= 2 as libc::c_int;
                        }
                        g.decode = 1 as libc::c_int;
                    }
                    102 => {
                        g.force = 1 as libc::c_int;
                    }
                    104 => {
                        help();
                    }
                    105 => {
                        g.setdict = 0 as libc::c_int;
                    }
                    107 => {
                        g.keep = 1 as libc::c_int;
                    }
                    108 => {
                        g.list = 1 as libc::c_int;
                    }
                    110 => {
                        g.headis = 0 as libc::c_int;
                    }
                    109 | 84 => {
                        g.headis &= -(11 as libc::c_int);
                    }
                    112 => {
                        get = 2 as libc::c_int;
                    }
                    113 => {
                        g.verbosity = 0 as libc::c_int;
                    }
                    114 => {
                        g.recurse = 1 as libc::c_int;
                    }
                    116 => {
                        g.decode = 2 as libc::c_int;
                    }
                    85 => {
                        g.strategy = 3 as libc::c_int;
                    }
                    118 => {
                        g.verbosity += 1;
                    }
                    122 => {
                        g.form = 1 as libc::c_int;
                        g
                            .sufx = b".zz\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    _ => {
                        try_throw_(
                            22 as libc::c_int,
                            b"invalid option: %s\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            bad.as_mut_ptr(),
                            0 as *mut libc::c_void,
                        );
                    }
                }
                arg = arg.offset(1);
                if *arg == 0 {
                    break;
                }
            }
        }
        if *arg as libc::c_int == 0 as libc::c_int {
            return 1 as libc::c_int;
        }
    }
    if get != 0 {
        if get == 1 as libc::c_int {
            n = num(arg);
            g.block = n << 10 as libc::c_int;
            g.shift = x2nmodp(g.block, 3 as libc::c_uint);
            if g.block < 32768 as libc::c_ulong {
                try_throw_(
                    22 as libc::c_int,
                    b"block size too small (must be >= 32K)\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    0 as *mut libc::c_void,
                );
            }
            if n != g.block >> 10 as libc::c_int {
                try_throw_(
                    22 as libc::c_int,
                    b"block size too large: %s\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    arg,
                    0 as *mut libc::c_void,
                );
            } else {
                if (g.block)
                    .wrapping_add(g.block >> 4 as libc::c_int)
                    .wrapping_add(32768 as libc::c_ulong) < g.block
                {
                    try_throw_(
                        22 as libc::c_int,
                        b"block size too large: %s\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        arg,
                        0 as *mut libc::c_void,
                    );
                } else {
                    if ((g.block)
                        .wrapping_add(g.block >> 4 as libc::c_int)
                        .wrapping_add(32768 as libc::c_ulong) as ssize_t)
                        < 0 as libc::c_long
                    {
                        try_throw_(
                            22 as libc::c_int,
                            b"block size too large: %s\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            arg,
                            0 as *mut libc::c_void,
                        );
                    } else {
                        if g.block > (1 as libc::c_ulong) << 29 as libc::c_int {
                            try_throw_(
                                22 as libc::c_int,
                                b"block size too large: %s\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                                arg,
                                0 as *mut libc::c_void,
                            );
                        }
                    }
                }
            }
        } else if get == 2 as libc::c_int {
            n = num(arg);
            g.procs = n as libc::c_int;
            if g.procs < 1 as libc::c_int {
                try_throw_(
                    22 as libc::c_int,
                    b"invalid number of processes: %s\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    arg,
                    0 as *mut libc::c_void,
                );
            }
            if g.procs as size_t != n {
                try_throw_(
                    22 as libc::c_int,
                    b"too many processes: %s\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    arg,
                    0 as *mut libc::c_void,
                );
            } else {
                if ((g.procs << 1 as libc::c_int) + 3 as libc::c_int) < 1 as libc::c_int
                {
                    try_throw_(
                        22 as libc::c_int,
                        b"too many processes: %s\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        arg,
                        0 as *mut libc::c_void,
                    );
                }
            }
        } else if get == 3 as libc::c_int {
            if *arg as libc::c_int == 0 as libc::c_int {
                try_throw_(
                    22 as libc::c_int,
                    b"suffix cannot be empty\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *mut libc::c_void,
                );
            }
            g.sufx = arg;
        } else if get == 4 as libc::c_int {
            tmp___1 = num(arg);
            g.zopts.numiterations = tmp___1 as libc::c_int;
        } else if get == 5 as libc::c_int {
            tmp___2 = num(arg);
            g.zopts.blocksplittingmax = tmp___2 as libc::c_int;
        } else if get == 6 as libc::c_int {
            g.alias = arg;
        } else if get == 7 as libc::c_int {
            g.comment = arg;
        }
        get = 0 as libc::c_int;
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn cut_yarn(mut err: libc::c_int) {
    try_throw_(
        err,
        b"internal threads error\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        0 as *mut libc::c_void,
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut nop: libc::c_int = 0;
    let mut done: libc::c_int = 0;
    let mut k: size_t = 0;
    let mut opts: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut err: try_ball_t_ = try_ball_t_ {
        ret: 0,
        code: 0,
        free: 0,
        why: 0 as *mut libc::c_char,
    };
    let mut try_this_: try_t_ = try_t_ {
        env: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
        ball: try_ball_t_ {
            ret: 0,
            code: 0,
            free: 0,
            why: 0 as *mut libc::c_char,
        },
        next: 0 as *mut try_t_,
    };
    let mut try_pushed_: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_long = 0;
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
    let mut tmp___17: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___18: libc::c_int = 0;
    let mut tmp___19: libc::c_int = 0;
    let mut tmp___23: libc::c_int = 0;
    let mut tmp___27: libc::c_int = 0;
    g.ret = 0 as libc::c_int;
    try_pushed_ = 1 as libc::c_int;
    try_this_.ball.ret = 0 as libc::c_int;
    try_this_.ball.code = 0 as libc::c_int;
    try_this_.ball.free = 0 as libc::c_int;
    try_this_.ball.why = 0 as *mut libc::c_void as *mut libc::c_char;
    try_setup_();
    tmp = pthread_getspecific(try_key_);
    try_this_.next = tmp as *mut try_t_;
    tmp___3 = pthread_setspecific(
        try_key_,
        &mut try_this_ as *mut try_t_ as *const libc::c_void,
    );
    if tmp___3 == 0 as libc::c_int {
        if (b"try: pthread_setspecific() failed\0" as *const u8 as *const libc::c_char)
            .is_null()
        {
            __assert_fail(
                b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                4605 as libc::c_uint,
                b"main\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        __assert_fail(
            b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                as *const u8 as *const libc::c_char,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            4605 as libc::c_uint,
            b"main\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___19 = _setjmp((try_this_.env).as_mut_ptr());
    if tmp___19 == 0 as libc::c_int {
        g.inf = 0 as *mut libc::c_void as *mut libc::c_char;
        g.inz = 0 as libc::c_int as size_t;
        g.in_which = -(1 as libc::c_int);
        g.alias = b"-\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        g.outf = 0 as *mut libc::c_void as *mut libc::c_char;
        g.first = 1 as libc::c_int;
        g.hname = 0 as *mut libc::c_void as *mut libc::c_char;
        g.hcomm = 0 as *mut libc::c_void as *mut libc::c_char;
        p = strrchr(
            *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
            '/' as i32,
        );
        if p as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            p = *argv.offset(0 as libc::c_int as isize);
        } else {
            p = p.offset(1);
        }
        if *p != 0 {
            g.prog = p;
        } else {
            g.prog = b"pigz\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        signal(
            2 as libc::c_int,
            Some(cut_short as unsafe extern "C" fn(libc::c_int) -> ()),
        );
        yarn_prefix = g.prog;
        yarn_abort = Some(cut_yarn as unsafe extern "C" fn(libc::c_int) -> ());
        defaults();
        tmp___4 = zlib_vernum();
        if tmp___4 < 4656 as libc::c_long {
            try_throw_(
                22 as libc::c_int,
                b"zlib version less than 1.2.3\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_void,
            );
        }
        get_crc_table();
        opts = getenv(b"GZIP\0" as *const u8 as *const libc::c_char);
        if opts as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            while *opts != 0 {
                loop {
                    if !(*opts as libc::c_int == 32 as libc::c_int) {
                        if !(*opts as libc::c_int == 9 as libc::c_int) {
                            break;
                        }
                    }
                    opts = opts.offset(1);
                }
                p = opts;
                while *p != 0 {
                    if !(*p as libc::c_int != 32 as libc::c_int) {
                        break;
                    }
                    if !(*p as libc::c_int != 9 as libc::c_int) {
                        break;
                    }
                    p = p.offset(1);
                }
                n = *p as libc::c_int;
                *p = 0 as libc::c_int as libc::c_char;
                tmp___5 = option(opts);
                if tmp___5 == 0 {
                    try_throw_(
                        22 as libc::c_int,
                        b"cannot provide files in GZIP environment variable\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                        0 as *mut libc::c_void,
                    );
                }
                if n != 0 {
                    tmp___6 = 1 as libc::c_int;
                } else {
                    tmp___6 = 0 as libc::c_int;
                }
                opts = p.offset(tmp___6 as isize);
            }
            option(0 as *mut libc::c_void as *mut libc::c_char);
        }
        opts = getenv(b"PIGZ\0" as *const u8 as *const libc::c_char);
        if opts as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            while *opts != 0 {
                loop {
                    if !(*opts as libc::c_int == 32 as libc::c_int) {
                        if !(*opts as libc::c_int == 9 as libc::c_int) {
                            break;
                        }
                    }
                    opts = opts.offset(1);
                }
                p = opts;
                while *p != 0 {
                    if !(*p as libc::c_int != 32 as libc::c_int) {
                        break;
                    }
                    if !(*p as libc::c_int != 9 as libc::c_int) {
                        break;
                    }
                    p = p.offset(1);
                }
                n = *p as libc::c_int;
                *p = 0 as libc::c_int as libc::c_char;
                tmp___7 = option(opts);
                if tmp___7 == 0 {
                    try_throw_(
                        22 as libc::c_int,
                        b"cannot provide files in PIGZ environment variable\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                        0 as *mut libc::c_void,
                    );
                }
                if n != 0 {
                    tmp___8 = 1 as libc::c_int;
                } else {
                    tmp___8 = 0 as libc::c_int;
                }
                opts = p.offset(tmp___8 as isize);
            }
            option(0 as *mut libc::c_void as *mut libc::c_char);
        }
        tmp___9 = strcmp(
            g.prog as *const libc::c_char,
            b"unpigz\0" as *const u8 as *const libc::c_char,
        );
        let mut current_block_98: u64;
        if tmp___9 == 0 as libc::c_int {
            current_block_98 = 6975523133767143884;
        } else {
            tmp___10 = strcmp(
                g.prog as *const libc::c_char,
                b"gunzip\0" as *const u8 as *const libc::c_char,
            );
            if tmp___10 == 0 as libc::c_int {
                current_block_98 = 6975523133767143884;
            } else {
                current_block_98 = 16593409533420678784;
            }
        }
        match current_block_98 {
            6975523133767143884 => {
                if g.decode == 0 {
                    g.headis >>= 2 as libc::c_int;
                }
                g.decode = 1 as libc::c_int;
            }
            _ => {}
        }
        k = strlen(g.prog as *const libc::c_char);
        if k > 2 as libc::c_ulong {
            tmp___11 = strcmp(
                (g.prog).offset(k as isize).offset(-(3 as libc::c_int as isize))
                    as *const libc::c_char,
                b"cat\0" as *const u8 as *const libc::c_char,
            );
            if tmp___11 == 0 as libc::c_int {
                if g.decode == 0 {
                    g.headis >>= 2 as libc::c_int;
                }
                g.decode = 1 as libc::c_int;
                g.pipeout = 1 as libc::c_int;
            }
        }
        if argc < 2 as libc::c_int {
            if g.decode != 0 {
                tmp___12 = 0 as libc::c_int;
            } else {
                tmp___12 = 1 as libc::c_int;
            }
            tmp___13 = isatty(tmp___12);
            if tmp___13 != 0 {
                help();
            }
        }
        nop = argc;
        n = 1 as libc::c_int;
        while n < argc {
            tmp___15 = strcmp(
                *argv.offset(n as isize) as *const libc::c_char,
                b"--\0" as *const u8 as *const libc::c_char,
            );
            if tmp___15 == 0 as libc::c_int {
                nop = n;
                let ref mut fresh0 = *argv.offset(n as isize);
                *fresh0 = 0 as *mut libc::c_void as *mut libc::c_char;
                break;
            } else {
                tmp___14 = option(*argv.offset(n as isize));
                if tmp___14 != 0 {
                    let ref mut fresh1 = *argv.offset(n as isize);
                    *fresh1 = 0 as *mut libc::c_void as *mut libc::c_char;
                }
                n += 1;
            }
        }
        option(0 as *mut libc::c_void as *mut libc::c_char);
        done = 0 as libc::c_int;
        n = 1 as libc::c_int;
        while n < argc {
            if *argv.offset(n as isize) as libc::c_ulong
                != 0 as *mut libc::c_void as libc::c_ulong
            {
                if done == 1 as libc::c_int {
                    if g.pipeout != 0 {
                        if g.decode == 0 {
                            if g.list == 0 {
                                if g.form > 1 as libc::c_int {
                                    complain(
                                        b"warning: output will be concatenated zip files -- %s will not be able to extract\0"
                                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                                        g.prog,
                                    );
                                }
                            }
                        }
                    }
                }
                if n < nop {
                    tmp___18 = strcmp(
                        *argv.offset(n as isize) as *const libc::c_char,
                        b"-\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___18 == 0 as libc::c_int {
                        tmp___17 = 0 as *mut libc::c_void as *mut libc::c_char;
                    } else {
                        tmp___17 = *argv.offset(n as isize);
                    }
                } else {
                    tmp___17 = *argv.offset(n as isize);
                }
                process(tmp___17);
                done += 1;
            }
            n += 1;
        }
        if done == 0 as libc::c_int {
            process(0 as *mut libc::c_void as *mut libc::c_char);
        }
    }
    if try_pushed_ != 0 {
        tmp___23 = pthread_setspecific(try_key_, try_this_.next as *const libc::c_void);
        if tmp___23 == 0 as libc::c_int {
            if (b"try: pthread_setspecific() failed\0" as *const u8
                as *const libc::c_char)
                .is_null()
            {
                __assert_fail(
                    b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                        as *const u8 as *const libc::c_char,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    4727 as libc::c_uint,
                    b"main\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            __assert_fail(
                b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                4727 as libc::c_uint,
                b"main\0" as *const u8 as *const libc::c_char,
            );
        }
        try_pushed_ = 0 as libc::c_int;
    }
    if g.inf as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        free(g.inf as *mut libc::c_void);
        g.inf = 0 as *mut libc::c_void as *mut libc::c_char;
    }
    g.inz = 0 as libc::c_int as size_t;
    new_opts();
    if try_pushed_ != 0 {
        tmp___27 = pthread_setspecific(try_key_, try_this_.next as *const libc::c_void);
        if tmp___27 == 0 as libc::c_int {
            if (b"try: pthread_setspecific() failed\0" as *const u8
                as *const libc::c_char)
                .is_null()
            {
                __assert_fail(
                    b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                        as *const u8 as *const libc::c_char,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    4733 as libc::c_uint,
                    b"main\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            __assert_fail(
                b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                4733 as libc::c_uint,
                b"main\0" as *const u8 as *const libc::c_char,
            );
        }
        try_pushed_ = 0 as libc::c_int;
    }
    err = try_this_.ball;
    if err.code != 0 {
        if err.code != 32 as libc::c_int {
            complain(
                b"abort: %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                err.why,
            );
        }
        if err.free != 0 {
            free(err.why as *mut libc::c_void);
            err.free = 0 as libc::c_int;
            err.why = 0 as *mut libc::c_void as *mut libc::c_char;
        }
        cut_short(-err.code);
    }
    return g.ret;
}
pub static mut try_key_: pthread_key_t = 0;
static mut try_once_: pthread_once_t = 0 as libc::c_int;
unsafe extern "C" fn try_create_() {
    let mut ret: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tmp = pthread_key_create(
        &mut try_key_,
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(0 as *mut libc::c_void),
    );
    ret = tmp;
    if ret == 0 as libc::c_int {
        if (b"try: pthread_key_create() failed\0" as *const u8 as *const libc::c_char)
            .is_null()
        {
            __assert_fail(
                b"ret == 0 && \"try: pthread_key_create() failed\"\0" as *const u8
                    as *const libc::c_char,
                b"try.c\0" as *const u8 as *const libc::c_char,
                22 as libc::c_uint,
                b"try_create_\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        __assert_fail(
            b"ret == 0 && \"try: pthread_key_create() failed\"\0" as *const u8
                as *const libc::c_char,
            b"try.c\0" as *const u8 as *const libc::c_char,
            22 as libc::c_uint,
            b"try_create_\0" as *const u8 as *const libc::c_char,
        );
    };
}
pub unsafe extern "C" fn try_setup_() {
    let mut ret: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tmp = pthread_once(
        &mut try_once_,
        Some(try_create_ as unsafe extern "C" fn() -> ()),
    );
    ret = tmp;
    if ret == 0 as libc::c_int {
        if (b"try: pthread_once() failed\0" as *const u8 as *const libc::c_char)
            .is_null()
        {
            __assert_fail(
                b"ret == 0 && \"try: pthread_once() failed\"\0" as *const u8
                    as *const libc::c_char,
                b"try.c\0" as *const u8 as *const libc::c_char,
                27 as libc::c_uint,
                b"try_setup_\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        __assert_fail(
            b"ret == 0 && \"try: pthread_once() failed\"\0" as *const u8
                as *const libc::c_char,
            b"try.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_uint,
            b"try_setup_\0" as *const u8 as *const libc::c_char,
        );
    };
}
pub unsafe extern "C" fn try_throw_(
    mut code: libc::c_int,
    mut fmt: *mut libc::c_char,
    mut args: ...
) -> ! {
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___5: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___6: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut why: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nul: [libc::c_char; 1] = [0; 1];
    let mut len: size_t = 0;
    let mut ap1: ::std::ffi::VaListImpl;
    let mut ap2: ::std::ffi::VaListImpl;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___9: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___10: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___11: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___12: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___13: *mut libc::c_void = 0 as *mut libc::c_void;
    try_setup_();
    tmp___2 = pthread_getspecific(try_key_);
    if tmp___2 as *mut try_t_ as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong
    {
        if (b"try: naked throw\0" as *const u8 as *const libc::c_char).is_null() {
            __assert_fail(
                b"try_stack_ != NULL && \"try: naked throw\"\0" as *const u8
                    as *const libc::c_char,
                b"try.c\0" as *const u8 as *const libc::c_char,
                40 as libc::c_uint,
                b"try_throw_\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        __assert_fail(
            b"try_stack_ != NULL && \"try: naked throw\"\0" as *const u8
                as *const libc::c_char,
            b"try.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_uint,
            b"try_throw_\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___3 = pthread_getspecific(try_key_);
    (*(tmp___3 as *mut try_t_)).ball.ret = 1 as libc::c_int;
    tmp___4 = pthread_getspecific(try_key_);
    (*(tmp___4 as *mut try_t_)).ball.code = code;
    tmp___5 = pthread_getspecific(try_key_);
    (*(tmp___5 as *mut try_t_)).ball.free = 0 as libc::c_int;
    tmp___6 = pthread_getspecific(try_key_);
    let ref mut fresh2 = (*(tmp___6 as *mut try_t_)).ball.why;
    *fresh2 = fmt;
    if fmt as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp___12 = strchr(fmt as *const libc::c_char, '%' as i32);
        if tmp___12 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            ap1 = args.clone();
            ap2 = ap1.clone();
            tmp___7 = vsnprintf(
                nul.as_mut_ptr(),
                1 as libc::c_int as size_t,
                fmt as *const libc::c_char,
                ap1.as_va_list(),
            );
            len = tmp___7 as size_t;
            tmp___8 = malloc(len.wrapping_add(1 as libc::c_ulong));
            why = tmp___8 as *mut libc::c_char;
            if why as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                tmp___9 = pthread_getspecific(try_key_);
                let ref mut fresh3 = (*(tmp___9 as *mut try_t_)).ball.why;
                *fresh3 = b"try: out of memory\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            } else {
                vsnprintf(
                    why,
                    len.wrapping_add(1 as libc::c_ulong),
                    fmt as *const libc::c_char,
                    ap2.as_va_list(),
                );
                tmp___10 = pthread_getspecific(try_key_);
                (*(tmp___10 as *mut try_t_)).ball.free = 1 as libc::c_int;
                tmp___11 = pthread_getspecific(try_key_);
                let ref mut fresh4 = (*(tmp___11 as *mut try_t_)).ball.why;
                *fresh4 = why;
            }
        }
    }
    tmp___13 = pthread_getspecific(try_key_);
    longjmp(((*(tmp___13 as *mut try_t_)).env).as_mut_ptr(), 1 as libc::c_int);
}
pub static mut yarn_prefix: *mut libc::c_char = b"yarn\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
pub static mut yarn_abort: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = unsafe {
    ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    >(0 as *const libc::c_void as *mut libc::c_void)
};
unsafe extern "C" fn fail(
    mut err: libc::c_int,
    mut file: *const libc::c_char,
    mut line: libc::c_long,
    mut func: *const libc::c_char,
) -> ! {
    fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, yarn_prefix);
    match err {
        1 => {
            fputs(b"already unlocked\0" as *const u8 as *const libc::c_char, stderr);
        }
        3 => {
            fputs(b"no such thread\0" as *const u8 as *const libc::c_char, stderr);
        }
        35 => {
            fputs(b"resource deadlock\0" as *const u8 as *const libc::c_char, stderr);
        }
        12 => {
            fputs(b"out of memory\0" as *const u8 as *const libc::c_char, stderr);
        }
        16 => {
            fputs(
                b"can't destroy locked resource\0" as *const u8 as *const libc::c_char,
                stderr,
            );
        }
        22 => {
            fputs(b"invalid request\0" as *const u8 as *const libc::c_char, stderr);
        }
        11 => {
            fputs(b"resource unavailable\0" as *const u8 as *const libc::c_char, stderr);
        }
        _ => {
            fprintf(
                stderr,
                b"internal error %d\0" as *const u8 as *const libc::c_char,
                err,
            );
        }
    }
    fprintf(
        stderr,
        b" (%s:%ld:%s)\n\0" as *const u8 as *const libc::c_char,
        file,
        line,
        func,
    );
    if ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        libc::c_ulong,
    >(yarn_abort) != 0 as *mut libc::c_void as libc::c_ulong
    {
        (Some(yarn_abort.expect("non-null function pointer")))
            .expect("non-null function pointer")(err);
    }
    exit(err);
}
static mut my_malloc_f: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void> = Some(
    malloc as unsafe extern "C" fn(libc::c_ulong) -> *mut libc::c_void,
);
static mut my_free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()> = Some(
    free as unsafe extern "C" fn(*mut libc::c_void) -> (),
);
pub unsafe extern "C" fn yarn_mem(
    mut lease: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    mut vacate: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) {
    my_malloc_f = lease;
    my_free = vacate;
}
unsafe extern "C" fn my_malloc(
    mut size: size_t,
    mut file: *const libc::c_char,
    mut line: libc::c_long,
) -> *mut libc::c_void {
    let mut block: *mut libc::c_void = 0 as *mut libc::c_void;
    block = (Some(my_malloc_f.expect("non-null function pointer")))
        .expect("non-null function pointer")(size);
    if block as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fail(
            12 as libc::c_int,
            file,
            line,
            b"malloc\0" as *const u8 as *const libc::c_char,
        );
    }
    return block;
}
pub unsafe extern "C" fn new_lock_(
    mut initial: libc::c_long,
    mut file: *const libc::c_char,
    mut line: libc::c_long,
) -> *mut lock {
    let mut bolt: *mut lock = 0 as *mut lock;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ret: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = my_malloc(::std::mem::size_of::<lock_s>() as libc::c_ulong, file, line);
    bolt = tmp as *mut lock;
    tmp___0 = pthread_mutex_init(
        &mut (*bolt).mutex,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
    ret = tmp___0;
    if ret != 0 {
        fail(ret, file, line, b"mutex_init\0" as *const u8 as *const libc::c_char);
    }
    ret = pthread_cond_init(
        &mut (*bolt).cond as *mut pthread_cond_t,
        0 as *mut libc::c_void as *const pthread_condattr_t,
    );
    if ret != 0 {
        fail(ret, file, line, b"cond_init\0" as *const u8 as *const libc::c_char);
    }
    (*bolt).value = initial;
    return bolt;
}
pub unsafe extern "C" fn possess_(
    mut bolt: *mut lock,
    mut file: *const libc::c_char,
    mut line: libc::c_long,
) {
    let mut ret: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tmp = pthread_mutex_lock(&mut (*bolt).mutex);
    ret = tmp;
    if ret != 0 {
        fail(ret, file, line, b"mutex_lock\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn release_(
    mut bolt: *mut lock,
    mut file: *const libc::c_char,
    mut line: libc::c_long,
) {
    let mut ret: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tmp = pthread_mutex_unlock(&mut (*bolt).mutex);
    ret = tmp;
    if ret != 0 {
        fail(ret, file, line, b"mutex_unlock\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn twist_(
    mut bolt: *mut lock,
    mut op: twist_op,
    mut val: libc::c_long,
    mut file: *const libc::c_char,
    mut line: libc::c_long,
) {
    let mut ret: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    if op as libc::c_uint == 0 as libc::c_uint {
        (*bolt).value = val;
    } else if op as libc::c_uint == 1 as libc::c_uint {
        (*bolt).value += val;
    }
    tmp = pthread_cond_broadcast(&mut (*bolt).cond);
    ret = tmp;
    if ret != 0 {
        fail(ret, file, line, b"cond_broadcast\0" as *const u8 as *const libc::c_char);
    }
    ret = pthread_mutex_unlock(&mut (*bolt).mutex);
    if ret != 0 {
        fail(ret, file, line, b"mutex_unlock\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn wait_for_(
    mut bolt: *mut lock,
    mut op: wait_op,
    mut val: libc::c_long,
    mut file: *const libc::c_char,
    mut line: libc::c_long,
) {
    let mut ret: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut ret___0: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut ret___1: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut ret___2: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    match op as libc::c_uint {
        0 => {
            while !((*bolt).value == val) {
                tmp = pthread_cond_wait(
                    &mut (*bolt).cond as *mut pthread_cond_t,
                    &mut (*bolt).mutex as *mut pthread_mutex_t,
                );
                ret = tmp;
                if ret != 0 {
                    fail(
                        ret,
                        file,
                        line,
                        b"cond_wait\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
        1 => {
            while !((*bolt).value != val) {
                tmp___0 = pthread_cond_wait(
                    &mut (*bolt).cond as *mut pthread_cond_t,
                    &mut (*bolt).mutex as *mut pthread_mutex_t,
                );
                ret___0 = tmp___0;
                if ret___0 != 0 {
                    fail(
                        ret___0,
                        file,
                        line,
                        b"cond_wait\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
        2 => {
            while !((*bolt).value > val) {
                tmp___1 = pthread_cond_wait(
                    &mut (*bolt).cond as *mut pthread_cond_t,
                    &mut (*bolt).mutex as *mut pthread_mutex_t,
                );
                ret___1 = tmp___1;
                if ret___1 != 0 {
                    fail(
                        ret___1,
                        file,
                        line,
                        b"cond_wait\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
        3 => {
            while !((*bolt).value < val) {
                tmp___2 = pthread_cond_wait(
                    &mut (*bolt).cond as *mut pthread_cond_t,
                    &mut (*bolt).mutex as *mut pthread_mutex_t,
                );
                ret___2 = tmp___2;
                if ret___2 != 0 {
                    fail(
                        ret___2,
                        file,
                        line,
                        b"cond_wait\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
        _ => {}
    };
}
pub unsafe extern "C" fn peek_lock(mut bolt: *mut lock) -> libc::c_long {
    return (*bolt).value;
}
pub unsafe extern "C" fn free_lock_(
    mut bolt: *mut lock,
    mut file: *const libc::c_char,
    mut line: libc::c_long,
) {
    let mut ret: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    if bolt as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    tmp = pthread_cond_destroy(&mut (*bolt).cond);
    ret = tmp;
    if ret != 0 {
        fail(ret, file, line, b"cond_destroy\0" as *const u8 as *const libc::c_char);
    }
    ret = pthread_mutex_destroy(&mut (*bolt).mutex);
    if ret != 0 {
        fail(ret, file, line, b"mutex_destroy\0" as *const u8 as *const libc::c_char);
    }
    (Some(my_free.expect("non-null function pointer")))
        .expect("non-null function pointer")(bolt as *mut libc::c_void);
}
static mut threads_lock: lock = {
    let mut init = lock_s {
        mutex: __anonunion_pthread_mutex_t_335460617 {
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
        },
        cond: __anonunion_pthread_cond_t_951761805 {
            __data: {
                let mut init = __pthread_cond_s {
                    __annonCompField1: __anonunion____missing_field_name_419937108 {
                        __wseq: 0 as libc::c_ulonglong,
                    },
                    __annonCompField2: __anonunion____missing_field_name_845759682 {
                        __g1_start: 0 as libc::c_ulonglong,
                    },
                    __g_refs: [0 as libc::c_uint, 0 as libc::c_uint],
                    __g_size: [0 as libc::c_uint, 0 as libc::c_uint],
                    __g1_orig_size: 0 as libc::c_uint,
                    __wrefs: 0 as libc::c_uint,
                    __g_signals: [0 as libc::c_uint, 0 as libc::c_uint],
                };
                init
            },
        },
        value: 0 as libc::c_long,
    };
    init
};
static mut threads: *mut thread = 0 as *const libc::c_void as *mut libc::c_void
    as *mut thread;
unsafe extern "C" fn reenter(mut arg: *mut libc::c_void) {
    let mut capsule: *mut capsule = 0 as *mut capsule;
    let mut me: pthread_t = 0;
    let mut tmp: pthread_t = 0;
    let mut prior: *mut *mut thread = 0 as *mut *mut thread;
    let mut match_0: *mut thread = 0 as *mut thread;
    let mut tmp___0: libc::c_int = 0;
    capsule = arg as *mut capsule;
    tmp = pthread_self();
    me = tmp;
    possess_(&mut threads_lock, (*capsule).file, (*capsule).line);
    prior = &mut threads;
    loop {
        match_0 = *prior;
        if !(match_0 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
            break;
        }
        tmp___0 = pthread_equal((*match_0).id, me);
        if tmp___0 != 0 {
            break;
        }
        prior = &mut (*match_0).next;
    }
    if match_0 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fail(
            3 as libc::c_int,
            (*capsule).file,
            (*capsule).line,
            b"reenter lost\0" as *const u8 as *const libc::c_char,
        );
    }
    (*match_0).done = 1 as libc::c_int;
    if threads as libc::c_ulong != match_0 as libc::c_ulong {
        *prior = (*match_0).next;
        (*match_0).next = threads;
        threads = match_0;
    }
    twist_(&mut threads_lock, BY, 1 as libc::c_long, (*capsule).file, (*capsule).line);
    (Some(my_free.expect("non-null function pointer")))
        .expect("non-null function pointer")(capsule as *mut libc::c_void);
}
unsafe extern "C" fn ignition(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut capsule: *mut capsule = 0 as *mut capsule;
    let mut __cancel_buf: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
        __cancel_jmp_buf: [__anonstruct___cancel_jmp_buf_572769531 {
            __cancel_jmp_buf: [0; 8],
            __mask_was_saved: 0,
        }; 1],
        __pad: [0 as *mut libc::c_void; 4],
    };
    let mut __cancel_routine: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()> = None;
    let mut __cancel_arg: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut __not_first_call: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_long = 0;
    capsule = arg as *mut capsule;
    __cancel_routine = Some(reenter as unsafe extern "C" fn(*mut libc::c_void) -> ());
    __cancel_arg = arg;
    tmp = __sigsetjmp(
        (__cancel_buf.__cancel_jmp_buf).as_mut_ptr() as *mut libc::c_void
            as *mut __jmp_buf_tag,
        0 as libc::c_int,
    );
    __not_first_call = tmp;
    tmp___0 = __not_first_call as libc::c_long;
    if tmp___0 != 0 {
        (Some(__cancel_routine.expect("non-null function pointer")))
            .expect("non-null function pointer")(__cancel_arg);
        __pthread_unwind_next(&mut __cancel_buf);
    }
    __pthread_register_cancel(&mut __cancel_buf);
    (Some(((*capsule).probe).expect("non-null function pointer")))
        .expect("non-null function pointer")((*capsule).payload);
    __pthread_unregister_cancel(&mut __cancel_buf);
    (Some(__cancel_routine.expect("non-null function pointer")))
        .expect("non-null function pointer")(__cancel_arg);
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn launch_(
    mut probe: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    mut payload: *mut libc::c_void,
    mut file: *const libc::c_char,
    mut line: libc::c_long,
) -> *mut thread {
    let mut capsule: *mut capsule = 0 as *mut capsule;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut th: *mut thread = 0 as *mut thread;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    let mut ret: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    tmp = my_malloc(::std::mem::size_of::<capsule>() as libc::c_ulong, file, line);
    capsule = tmp as *mut capsule;
    (*capsule).probe = probe;
    (*capsule).payload = payload;
    (*capsule).file = file;
    (*capsule).line = line;
    possess_(&mut threads_lock, file, line);
    tmp___0 = my_malloc(::std::mem::size_of::<thread_s>() as libc::c_ulong, file, line);
    th = tmp___0 as *mut thread;
    tmp___1 = pthread_attr_init(&mut attr);
    ret = tmp___1;
    if ret != 0 {
        fail(ret, file, line, b"attr_init\0" as *const u8 as *const libc::c_char);
    }
    ret = pthread_attr_setdetachstate(&mut attr, 0 as libc::c_int);
    if ret != 0 {
        fail(
            ret,
            file,
            line,
            b"attr_setdetachstate\0" as *const u8 as *const libc::c_char,
        );
    }
    ret = pthread_create(
        &mut (*th).id as *mut pthread_t,
        &mut attr as *mut pthread_attr_t as *const pthread_attr_t,
        Some(ignition as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        capsule as *mut libc::c_void,
    );
    if ret != 0 {
        fail(ret, file, line, b"create\0" as *const u8 as *const libc::c_char);
    }
    ret = pthread_attr_destroy(&mut attr);
    if ret != 0 {
        fail(ret, file, line, b"attr_destroy\0" as *const u8 as *const libc::c_char);
    }
    (*th).done = 0 as libc::c_int;
    (*th).next = threads;
    threads = th;
    release_(&mut threads_lock, file, line);
    return th;
}
pub unsafe extern "C" fn join_(
    mut ally: *mut thread,
    mut file: *const libc::c_char,
    mut line: libc::c_long,
) {
    let mut ret: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut prior: *mut *mut thread = 0 as *mut *mut thread;
    let mut match_0: *mut thread = 0 as *mut thread;
    tmp = pthread_join((*ally).id, 0 as *mut libc::c_void as *mut *mut libc::c_void);
    ret = tmp;
    if ret != 0 {
        fail(ret, file, line, b"join\0" as *const u8 as *const libc::c_char);
    }
    possess_(&mut threads_lock, file, line);
    prior = &mut threads;
    loop {
        match_0 = *prior;
        if !(match_0 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
            break;
        }
        if match_0 as libc::c_ulong == ally as libc::c_ulong {
            break;
        }
        prior = &mut (*match_0).next;
    }
    if match_0 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fail(
            3 as libc::c_int,
            file,
            line,
            b"join lost\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*match_0).done != 0 {
        threads_lock.value -= 1;
    }
    *prior = (*match_0).next;
    release_(&mut threads_lock, file, line);
    (Some(my_free.expect("non-null function pointer")))
        .expect("non-null function pointer")(ally as *mut libc::c_void);
}
pub unsafe extern "C" fn join_all_(
    mut file: *const libc::c_char,
    mut line: libc::c_long,
) -> libc::c_int {
    let mut count: libc::c_int = 0;
    let mut prior: *mut *mut thread = 0 as *mut *mut thread;
    let mut match_0: *mut thread = 0 as *mut thread;
    let mut ret: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    count = 0 as libc::c_int;
    possess_(&mut threads_lock, file, line);
    while threads as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        wait_for_(&mut threads_lock, NOT_TO_BE, 0 as libc::c_long, file, line);
        prior = &mut threads;
        loop {
            match_0 = *prior;
            if !(match_0 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
                break;
            }
            if (*match_0).done != 0 {
                break;
            }
            prior = &mut (*match_0).next;
        }
        if match_0 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            fail(
                3 as libc::c_int,
                file,
                line,
                b"join_all lost\0" as *const u8 as *const libc::c_char,
            );
        }
        tmp = pthread_join(
            (*match_0).id,
            0 as *mut libc::c_void as *mut *mut libc::c_void,
        );
        ret = tmp;
        if ret != 0 {
            fail(ret, file, line, b"join\0" as *const u8 as *const libc::c_char);
        }
        threads_lock.value -= 1;
        *prior = (*match_0).next;
        (Some(my_free.expect("non-null function pointer")))
            .expect("non-null function pointer")(match_0 as *mut libc::c_void);
        count += 1;
    }
    release_(&mut threads_lock, file, line);
    return count;
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
