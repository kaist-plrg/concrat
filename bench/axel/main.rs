use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    static mut stdout: *mut FILE;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
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
    fn pthread_setcancelstate(
        __state: libc::c_int,
        __oldstate: *mut libc::c_int,
    ) -> libc::c_int;
    fn pthread_setcanceltype(
        __type: libc::c_int,
        __oldtype: *mut libc::c_int,
    ) -> libc::c_int;
    fn pthread_cancel(__th: pthread_t) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_trylock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfscanf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn strtoull(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulonglong;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: Option::<
            unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        >,
    );
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
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
    fn inet_pton(
        __af: libc::c_int,
        __cp: *const libc::c_char,
        __buf: *mut libc::c_void,
    ) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    static mut opterr: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn signal(
        __sig: libc::c_int,
        __handler: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> __sighandler_t;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
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
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type pthread_t = libc::c_ulong;
pub type __gnuc_va_list = __builtin_va_list;
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
pub type socklen_t = __socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_abuf_t_653462436 {
    pub p: *mut libc::c_char,
    pub len: size_t,
}
pub type abuf_t = __anonstruct_abuf_t_653462436;
pub type __uint16_t = libc::c_ushort;
pub type __uint64_t = libc::c_ulong;
pub type __suseconds_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type time_t = __time_t;
pub type mode_t = __mode_t;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_fd_set_356711149 {
    pub fds_bits: [__fd_mask; 16],
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
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_message_t_933122616 {
    pub next: *mut libc::c_void,
    pub text: [libc::c_char; 1024],
}
pub type message_t = __anonstruct_message_t_933122616;
pub type url_t = message_t;
pub type if_t = message_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_conf_t_915110938 {
    pub default_filename: [libc::c_char; 1024],
    pub http_proxy: [libc::c_char; 1024],
    pub no_proxy: [libc::c_char; 1024],
    pub num_connections: uint16_t,
    pub strip_cgi_parameters: libc::c_int,
    pub save_state_interval: libc::c_int,
    pub connection_timeout: libc::c_int,
    pub reconnect_delay: libc::c_int,
    pub max_redirect: libc::c_int,
    pub buffer_size: libc::c_int,
    pub max_speed: libc::c_ulonglong,
    pub verbose: libc::c_int,
    pub alternate_output: libc::c_int,
    pub insecure: libc::c_int,
    pub no_clobber: libc::c_int,
    pub percentage: libc::c_int,
    pub interfaces: *mut if_t,
    pub ai_family: sa_family_t,
    pub search_timeout: libc::c_int,
    pub search_threads: libc::c_int,
    pub search_amount: libc::c_int,
    pub search_top: libc::c_int,
    pub io_timeout: libc::c_uint,
    pub add_header_count: libc::c_int,
    pub add_header: [[libc::c_char; 1024]; 10],
}
pub type conf_t = __anonstruct_conf_t_915110938;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_tcp_t_13748673 {
    pub fd: libc::c_int,
    pub ai_family: sa_family_t,
}
pub type tcp_t = __anonstruct_tcp_t_13748673;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_ftp_t_515051304 {
    pub cwd: [libc::c_char; 1024],
    pub message: *mut libc::c_char,
    pub status: libc::c_int,
    pub tcp: tcp_t,
    pub data_tcp: tcp_t,
    pub proto: libc::c_int,
    pub ftp_mode: libc::c_int,
    pub local_if: *mut libc::c_char,
}
pub type ftp_t = __anonstruct_ftp_t_515051304;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_http_t_677330235 {
    pub host: [libc::c_char; 1024],
    pub auth: [libc::c_char; 1024],
    pub request: [abuf_t; 1],
    pub headers: [abuf_t; 1],
    pub port: libc::c_int,
    pub proto: libc::c_int,
    pub proxy: libc::c_int,
    pub proxy_auth: [libc::c_char; 1024],
    pub firstbyte: off_t,
    pub lastbyte: off_t,
    pub status: libc::c_int,
    pub tcp: tcp_t,
    pub local_if: *mut libc::c_char,
}
pub type http_t = __anonstruct_http_t_677330235;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_conn_t_702709736 {
    pub conf: *mut conf_t,
    pub proto: libc::c_int,
    pub port: libc::c_int,
    pub proxy: libc::c_int,
    pub host: [libc::c_char; 1024],
    pub dir: [libc::c_char; 1024],
    pub file: [libc::c_char; 1024],
    pub user: [libc::c_char; 1024],
    pub pass: [libc::c_char; 1024],
    pub output_filename: [libc::c_char; 1024],
    pub ftp: [ftp_t; 1],
    pub http: [http_t; 1],
    pub size: off_t,
    pub currentbyte: off_t,
    pub lastbyte: off_t,
    pub tcp: *mut tcp_t,
    pub enabled: bool,
    pub supported: bool,
    pub last_transfer: libc::c_int,
    pub message: *mut libc::c_char,
    pub local_if: *mut libc::c_char,
    pub state: bool,
    pub setup_thread: [pthread_t; 1],
    pub lock: pthread_mutex_t,
}
pub type conn_t = __anonstruct_conn_t_702709736;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_search_t_969092730 {
    pub url: [libc::c_char; 1024],
    pub speed_start_time: libc::c_double,
    pub speed: off_t,
    pub size: off_t,
    pub speed_thread: [pthread_t; 1],
    pub conf: *mut conf_t,
}
pub type search_t = __anonstruct_search_t_969092730;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_axel_t_40439293 {
    pub conn: *mut conn_t,
    pub conf: *mut conf_t,
    pub filename: [libc::c_char; 1024],
    pub start_time: libc::c_double,
    pub next_state: libc::c_int,
    pub finish_time: libc::c_int,
    pub bytes_done: off_t,
    pub start_byte: off_t,
    pub size: off_t,
    pub bytes_per_second: libc::c_longlong,
    pub delay_time: timespec,
    pub outfd: libc::c_int,
    pub ready: libc::c_int,
    pub message: *mut message_t,
    pub last_message: *mut message_t,
    pub url: *mut url_t,
}
pub type axel_t = __anonstruct_axel_t_40439293;
pub type __uint32_t = libc::c_uint;
pub type __caddr_t = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifmap {
    pub mem_start: libc::c_ulong,
    pub mem_end: libc::c_ulong,
    pub base_addr: libc::c_ushort,
    pub irq: libc::c_uchar,
    pub dma: libc::c_uchar,
    pub port: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_ifr_ifrn_352126815 {
    pub ifrn_name: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_ifr_ifru_537349870 {
    pub ifru_addr: sockaddr,
    pub ifru_dstaddr: sockaddr,
    pub ifru_broadaddr: sockaddr,
    pub ifru_netmask: sockaddr,
    pub ifru_hwaddr: sockaddr,
    pub ifru_flags: libc::c_short,
    pub ifru_ivalue: libc::c_int,
    pub ifru_mtu: libc::c_int,
    pub ifru_map: ifmap,
    pub ifru_slave: [libc::c_char; 16],
    pub ifru_newname: [libc::c_char; 16],
    pub ifru_data: __caddr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifreq {
    pub ifr_ifrn: __anonunion_ifr_ifrn_352126815,
    pub ifr_ifru: __anonunion_ifr_ifru_537349870,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
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
unsafe extern "C" fn vprintf(
    mut __fmt: *const libc::c_char,
    mut __arg: ::std::ffi::VaList,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = vfprintf(stdout, __fmt, __arg.as_va_list());
    return tmp;
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = putc(__c, stdout);
    return tmp;
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
pub unsafe extern "C" fn abuf_setup(
    mut abuf: *mut abuf_t,
    mut len: size_t,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = realloc((*abuf).p as *mut libc::c_void, len);
    p = tmp as *mut libc::c_char;
    if p.is_null() {
        if len != 0 {
            return -(12 as libc::c_int);
        }
    }
    (*abuf).p = p;
    (*abuf).len = len;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn abuf_printf(
    mut abuf: *mut abuf_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    let mut len: size_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    ap = args.clone();
    loop {
        tmp = vsnprintf((*abuf).p, (*abuf).len, fmt, ap.as_va_list());
        len = tmp as size_t;
        if len < (*abuf).len {
            break;
        }
        tmp___0 = abuf_setup(abuf, len.wrapping_add(1 as libc::c_ulong));
        r = tmp___0;
        if r < 0 as libc::c_int {
            return r;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn abuf_strcat(
    mut abuf: *mut abuf_t,
    mut src: *const libc::c_char,
) -> libc::c_int {
    let mut nread: size_t = 0;
    let mut tmp: size_t = 0;
    let mut done: size_t = 0;
    let mut ret: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = strlcat((*abuf).p, src, (*abuf).len);
    nread = tmp;
    if nread > (*abuf).len {
        done = ((*abuf).len).wrapping_sub(1 as libc::c_ulong);
        tmp___0 = abuf_setup(abuf, nread);
        ret = tmp___0;
        if ret < 0 as libc::c_int {
            return ret;
        }
        memcpy(
            ((*abuf).p).offset(done as isize) as *mut libc::c_void,
            src.offset(done as isize) as *const libc::c_void,
            nread.wrapping_sub(done),
        );
    }
    return 0 as libc::c_int;
}
static mut buffer: *mut libc::c_char = 0 as *const libc::c_void as *mut libc::c_void
    as *mut libc::c_char;
unsafe extern "C" fn stfile_makename(
    mut bname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut suffix: [libc::c_char; 4] = [0; 4];
    let mut bname_len: size_t = 0;
    let mut tmp: size_t = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    suffix[0 as libc::c_int as usize] = '.' as i32 as libc::c_char;
    suffix[1 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    suffix[2 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    suffix[3 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    tmp = strlen(bname);
    bname_len = tmp;
    tmp___0 = malloc(
        bname_len
            .wrapping_add(::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong),
    );
    buf = tmp___0 as *mut libc::c_char;
    if buf.is_null() {
        perror(b"stfile_open\0" as *const u8 as *const libc::c_char);
        abort();
    }
    memcpy(buf as *mut libc::c_void, bname as *const libc::c_void, bname_len);
    memcpy(
        buf.offset(bname_len as isize) as *mut libc::c_void,
        suffix.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong,
    );
    return buf;
}
unsafe extern "C" fn stfile_unlink(mut bname: *const libc::c_char) -> libc::c_int {
    let mut stname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = stfile_makename(bname);
    stname = tmp;
    tmp___0 = unlink(stname as *const libc::c_char);
    ret = tmp___0;
    free(stname as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn stfile_access(
    mut bname: *const libc::c_char,
    mut mode: libc::c_int,
) -> libc::c_int {
    let mut stname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = stfile_makename(bname);
    stname = tmp;
    tmp___0 = access(stname as *const libc::c_char, mode);
    ret = tmp___0;
    free(stname as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn stfile_open(
    mut bname: *const libc::c_char,
    mut flags: libc::c_int,
    mut mode: mode_t,
) -> libc::c_int {
    let mut stname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fd: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = stfile_makename(bname);
    stname = tmp;
    tmp___0 = open(stname as *const libc::c_char, flags, mode);
    fd = tmp___0;
    free(stname as *mut libc::c_void);
    return fd;
}
pub unsafe extern "C" fn axel_new(
    mut conf: *mut conf_t,
    mut count: libc::c_int,
    mut res: *const search_t,
) -> *mut axel_t {
    let mut current_block: u64;
    let mut axel: *mut axel_t = 0 as *mut axel_t;
    let mut status: libc::c_int = 0;
    let mut delay: uint64_t = 0;
    let mut u: *mut url_t = 0 as *mut url_t;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___8: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut msg: [libc::c_char; 80] = [0; 80];
    let mut code: libc::c_int = 0;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hsize: [libc::c_char; 32] = [0; 32];
    let mut tmp___13: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___14: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___15: size_t = 0;
    let mut tmp___16: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___17: *mut libc::c_char = 0 as *mut libc::c_char;
    if count == 0 {
        return 0 as *mut libc::c_void as *mut axel_t
    } else {
        if res.is_null() {
            return 0 as *mut libc::c_void as *mut axel_t;
        }
    }
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<axel_t>() as libc::c_ulong,
    );
    axel = tmp as *mut axel_t;
    if !axel.is_null() {
        (*axel).conf = conf;
        tmp___0 = calloc(
            (*(*axel).conf).num_connections as size_t,
            ::std::mem::size_of::<conn_t>() as libc::c_ulong,
        );
        (*axel).conn = tmp___0 as *mut conn_t;
        if !((*axel).conn).is_null() {
            i = 0 as libc::c_int;
            while i < (*(*axel).conf).num_connections as libc::c_int {
                pthread_mutex_init(
                    &mut (*((*axel).conn).offset(i as isize)).lock,
                    0 as *mut libc::c_void as *const pthread_mutexattr_t,
                );
                i += 1;
            }
            if (*(*axel).conf).max_speed > 0 as libc::c_ulonglong {
                if (16 as libc::c_ulonglong)
                    .wrapping_mul((*(*axel).conf).max_speed)
                    .wrapping_div((*(*axel).conf).buffer_size as libc::c_ulonglong)
                    < 8 as libc::c_ulonglong
                {
                    if (*(*axel).conf).verbose >= 2 as libc::c_int {
                        tmp___1 = dcgettext(
                            0 as *mut libc::c_void as *const libc::c_char,
                            b"Buffer resized for this speed.\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        );
                        axel_message(axel, tmp___1 as *const libc::c_char);
                    }
                    (*(*axel).conf)
                        .buffer_size = (*(*axel).conf).max_speed as libc::c_int;
                }
                delay = ((1000000000 as libc::c_int * (*(*axel).conf).buffer_size
                    * (*(*axel).conf).num_connections as libc::c_int)
                    as libc::c_ulonglong)
                    .wrapping_div((*(*axel).conf).max_speed) as uint64_t;
                (*axel)
                    .delay_time
                    .tv_sec = delay.wrapping_div(1000000000 as libc::c_ulong)
                    as __time_t;
                (*axel)
                    .delay_time
                    .tv_nsec = delay.wrapping_rem(1000000000 as libc::c_ulong)
                    as __syscall_slong_t;
            }
            if buffer as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                tmp___2 = malloc((*(*axel).conf).buffer_size as size_t);
                buffer = tmp___2 as *mut libc::c_char;
                if buffer.is_null() {
                    current_block = 8822364330071213878;
                } else {
                    current_block = 7659304154607701039;
                }
            } else {
                current_block = 7659304154607701039;
            }
            match current_block {
                8822364330071213878 => {}
                _ => {
                    tmp___3 = malloc(
                        (::std::mem::size_of::<url_t>() as libc::c_ulong)
                            .wrapping_mul(count as libc::c_ulong),
                    );
                    u = tmp___3 as *mut url_t;
                    if !u.is_null() {
                        (*axel).url = u;
                        i = 0 as libc::c_int;
                        while i < count {
                            strlcpy(
                                ((*u.offset(i as isize)).text).as_mut_ptr(),
                                ((*res.offset(i as isize)).url).as_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 1024]>()
                                    as libc::c_ulong,
                            );
                            let ref mut fresh0 = (*u.offset(i as isize)).next;
                            *fresh0 = u.offset((i + 1 as libc::c_int) as isize)
                                as *mut libc::c_void;
                            i += 1;
                        }
                        let ref mut fresh1 = (*u
                            .offset((count - 1 as libc::c_int) as isize))
                            .next;
                        *fresh1 = u as *mut libc::c_void;
                        let ref mut fresh2 = (*((*axel).conn)
                            .offset(0 as libc::c_int as isize))
                            .conf;
                        *fresh2 = (*axel).conf;
                        tmp___5 = conn_set(
                            ((*axel).conn).offset(0 as libc::c_int as isize),
                            ((*(*axel).url).text).as_mut_ptr() as *const libc::c_char,
                        );
                        if tmp___5 == 0 {
                            tmp___4 = dcgettext(
                                0 as *mut libc::c_void as *const libc::c_char,
                                b"Could not parse URL.\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            );
                            axel_message(axel, tmp___4 as *const libc::c_char);
                            (*axel).ready = -(1 as libc::c_int);
                            return axel;
                        }
                        let ref mut fresh3 = (*((*axel).conn)
                            .offset(0 as libc::c_int as isize))
                            .local_if;
                        *fresh3 = ((*(*(*axel).conf).interfaces).text).as_mut_ptr();
                        (*(*axel).conf)
                            .interfaces = (*(*(*axel).conf).interfaces).next
                            as *mut if_t;
                        strlcpy(
                            ((*axel).filename).as_mut_ptr(),
                            ((*((*axel).conn).offset(0 as libc::c_int as isize)).file)
                                .as_mut_ptr() as *const libc::c_char,
                            ::std::mem::size_of::<[libc::c_char; 1024]>()
                                as libc::c_ulong,
                        );
                        http_decode(((*axel).filename).as_mut_ptr());
                        s = strchr(
                            ((*axel).filename).as_mut_ptr() as *const libc::c_char,
                            '?' as i32,
                        );
                        if s as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong
                        {
                            if (*(*axel).conf).strip_cgi_parameters != 0 {
                                *s = 0 as libc::c_int as libc::c_char;
                            }
                        }
                        if (*axel).filename[0 as libc::c_int as usize] as libc::c_int
                            == 0 as libc::c_int
                        {
                            strlcpy(
                                ((*axel).filename).as_mut_ptr(),
                                ((*(*axel).conf).default_filename).as_mut_ptr()
                                    as *const libc::c_char,
                                ::std::mem::size_of::<[libc::c_char; 1024]>()
                                    as libc::c_ulong,
                            );
                        }
                        if (*(*axel).conf).no_clobber != 0 {
                            tmp___9 = access(
                                ((*axel).filename).as_mut_ptr() as *const libc::c_char,
                                0 as libc::c_int,
                            );
                            if tmp___9 == 0 as libc::c_int {
                                tmp___6 = stfile_access(
                                    ((*axel).filename).as_mut_ptr() as *const libc::c_char,
                                    0 as libc::c_int,
                                );
                                ret = tmp___6;
                                if ret != 0 {
                                    tmp___7 = dcgettext(
                                        0 as *mut libc::c_void as *const libc::c_char,
                                        b"File '%s' already there; not retrieving.\n\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    );
                                    printf(
                                        tmp___7 as *const libc::c_char,
                                        ((*axel).filename).as_mut_ptr(),
                                    );
                                    (*axel).ready = -(1 as libc::c_int);
                                    return axel;
                                }
                                tmp___8 = dcgettext(
                                    0 as *mut libc::c_void as *const libc::c_char,
                                    b"Incomplete download found, ignoring no-clobber option\n\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                );
                                printf(tmp___8 as *const libc::c_char);
                            }
                        }
                        loop {
                            tmp___10 = conn_init(
                                ((*axel).conn).offset(0 as libc::c_int as isize),
                            );
                            if tmp___10 == 0 {
                                axel_message(
                                    axel,
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    (*((*axel).conn).offset(0 as libc::c_int as isize)).message,
                                );
                                (*axel).ready = -(1 as libc::c_int);
                                return axel;
                            }
                            status = conn_info(
                                ((*axel).conn).offset(0 as libc::c_int as isize),
                            );
                            if status == 0 {
                                tmp___11 = conn_info_status_get(
                                    msg.as_mut_ptr(),
                                    ::std::mem::size_of::<[libc::c_char; 80]>()
                                        as libc::c_ulong,
                                    (*axel).conn,
                                );
                                code = tmp___11;
                                tmp___12 = dcgettext(
                                    0 as *mut libc::c_void as *const libc::c_char,
                                    b"ERROR %d: %s.\n\0" as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                );
                                fprintf(
                                    stderr,
                                    tmp___12 as *const libc::c_char,
                                    code,
                                    msg.as_mut_ptr(),
                                );
                                (*axel).ready = -(1 as libc::c_int);
                                return axel;
                            }
                            if !(status == -(1 as libc::c_int)) {
                                break;
                            }
                        }
                        conn_url(
                            ((*(*axel).url).text).as_mut_ptr(),
                            (::std::mem::size_of::<[libc::c_char; 1024]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_ulong),
                            (*axel).conn,
                        );
                        (*axel)
                            .size = (*((*axel).conn).offset(0 as libc::c_int as isize))
                            .size;
                        if (*(*axel).conf).verbose > 0 as libc::c_int {
                            if (*axel).size as libc::c_longlong
                                != 9223372036854775807 as libc::c_longlong
                            {
                                axel_size_human(
                                    hsize.as_mut_ptr(),
                                    ::std::mem::size_of::<[libc::c_char; 32]>()
                                        as libc::c_ulong,
                                    (*axel).size as size_t,
                                );
                                tmp___13 = dcgettext(
                                    0 as *mut libc::c_void as *const libc::c_char,
                                    b"File size: %s (%jd bytes)\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                );
                                axel_message(
                                    axel,
                                    tmp___13 as *const libc::c_char,
                                    hsize.as_mut_ptr(),
                                    (*axel).size,
                                );
                            } else {
                                tmp___14 = dcgettext(
                                    0 as *mut libc::c_void as *const libc::c_char,
                                    b"File size: unavailable\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                );
                                axel_message(axel, tmp___14 as *const libc::c_char);
                            }
                        }
                        tmp___15 = strcspn(
                            ((*axel).filename).as_mut_ptr() as *const libc::c_char,
                            b"*?\0" as *const u8 as *const libc::c_char,
                        );
                        if (*axel).filename[tmp___15 as usize] != 0 {
                            strlcpy(
                                ((*axel).filename).as_mut_ptr(),
                                ((*((*axel).conn).offset(0 as libc::c_int as isize)).file)
                                    .as_mut_ptr() as *const libc::c_char,
                                ::std::mem::size_of::<[libc::c_char; 1024]>()
                                    as libc::c_ulong,
                            );
                        }
                        if (*((*axel).conn).offset(0 as libc::c_int as isize))
                            .output_filename[0 as libc::c_int as usize] as libc::c_int
                            != 0 as libc::c_int
                        {
                            strlcpy(
                                ((*axel).filename).as_mut_ptr(),
                                ((*((*axel).conn).offset(0 as libc::c_int as isize))
                                    .output_filename)
                                    .as_mut_ptr() as *const libc::c_char,
                                ::std::mem::size_of::<[libc::c_char; 1024]>()
                                    as libc::c_ulong,
                            );
                        }
                        return axel;
                    }
                }
            }
        }
    }
    axel_close(axel);
    tmp___16 = __errno_location();
    tmp___17 = strerror(*tmp___16);
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, tmp___17);
    return 0 as *mut libc::c_void as *mut axel_t;
}
pub unsafe extern "C" fn axel_open(mut axel: *mut axel_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    let mut nread: ssize_t = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_conn: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut old_format: libc::c_int = 0;
    let mut stsize: off_t = 0;
    let mut tmp___2: __off_t = 0;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_conn___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___5: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___7: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut j: off_t = 0;
    let mut nwrite: ssize_t = 0;
    let mut tmp___12: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___13: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___14: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut __a: off_t = 0;
    let mut __b: libc::c_int = 0;
    let mut tmp___15: off_t = 0;
    let mut tmp___16: __off_t = 0;
    if (*(*axel).conf).verbose > 0 as libc::c_int {
        tmp = dcgettext(
            0 as *mut libc::c_void as *const libc::c_char,
            b"Opening output file %s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        axel_message(axel, tmp as *const libc::c_char, ((*axel).filename).as_mut_ptr());
    }
    (*axel).outfd = -(1 as libc::c_int);
    if !(*((*axel).conn).offset(0 as libc::c_int as isize)).supported {
        tmp___0 = dcgettext(
            0 as *mut libc::c_void as *const libc::c_char,
            b"Server unsupported, starting from scratch with one connection.\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        axel_message(axel, tmp___0 as *const libc::c_char);
        (*(*axel).conf).num_connections = 1 as libc::c_int as uint16_t;
        tmp___1 = realloc(
            (*axel).conn as *mut libc::c_void,
            ::std::mem::size_of::<conn_t>() as libc::c_ulong,
        );
        new_conn = tmp___1;
        if new_conn.is_null() {
            return 0 as libc::c_int;
        }
        (*axel).conn = new_conn as *mut conn_t;
        axel_divide(axel);
    } else {
        fd = stfile_open(
            ((*axel).filename).as_mut_ptr() as *const libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int as mode_t,
        );
        if fd != -(1 as libc::c_int) {
            old_format = 0 as libc::c_int;
            tmp___2 = lseek(fd, 0 as libc::c_int as __off_t, 2 as libc::c_int);
            stsize = tmp___2;
            lseek(fd, 0 as libc::c_int as __off_t, 0 as libc::c_int);
            nread = read(
                fd,
                &mut (*(*axel).conf).num_connections as *mut uint16_t
                    as *mut libc::c_void,
                ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
            );
            if nread as libc::c_ulong
                != ::std::mem::size_of::<uint16_t>() as libc::c_ulong
            {
                tmp___3 = dcgettext(
                    0 as *mut libc::c_void as *const libc::c_char,
                    b"%s.st: Error, truncated state file\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                );
                printf(tmp___3 as *const libc::c_char, ((*axel).filename).as_mut_ptr());
                close(fd);
                return 0 as libc::c_int;
            }
            if ((*(*axel).conf).num_connections as libc::c_int) < 1 as libc::c_int {
                tmp___4 = dcgettext(
                    0 as *mut libc::c_void as *const libc::c_char,
                    b"Bogus number of connections stored in state file\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                );
                fprintf(stderr, tmp___4 as *const libc::c_char);
                close(fd);
                return 0 as libc::c_int;
            }
            if stsize
                < (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<off_t>() as libc::c_ulong)
                    .wrapping_add(
                        ((2 as libc::c_int
                            * (*(*axel).conf).num_connections as libc::c_int)
                            as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<off_t>() as libc::c_ulong,
                            ),
                    ) as off_t
            {
                old_format = 1 as libc::c_int;
            }
            tmp___5 = realloc(
                (*axel).conn as *mut libc::c_void,
                (::std::mem::size_of::<conn_t>() as libc::c_ulong)
                    .wrapping_mul((*(*axel).conf).num_connections as libc::c_ulong),
            );
            new_conn___0 = tmp___5;
            if new_conn___0.is_null() {
                close(fd);
                return 0 as libc::c_int;
            }
            (*axel).conn = new_conn___0 as *mut conn_t;
            memset(
                ((*axel).conn).offset(1 as libc::c_int as isize) as *mut libc::c_void,
                0 as libc::c_int,
                (::std::mem::size_of::<conn_t>() as libc::c_ulong)
                    .wrapping_mul(
                        ((*(*axel).conf).num_connections as libc::c_int
                            - 1 as libc::c_int) as libc::c_ulong,
                    ),
            );
            if old_format != 0 {
                axel_divide(axel);
            }
            nread = read(
                fd,
                &mut (*axel).bytes_done as *mut off_t as *mut libc::c_void,
                ::std::mem::size_of::<off_t>() as libc::c_ulong,
            );
            i = 0 as libc::c_int;
            while i < (*(*axel).conf).num_connections as libc::c_int {
                nread = read(
                    fd,
                    &mut (*((*axel).conn).offset(i as isize)).currentbyte as *mut off_t
                        as *mut libc::c_void,
                    ::std::mem::size_of::<off_t>() as libc::c_ulong,
                );
                if old_format == 0 {
                    nread = read(
                        fd,
                        &mut (*((*axel).conn).offset(i as isize)).lastbyte as *mut off_t
                            as *mut libc::c_void,
                        ::std::mem::size_of::<off_t>() as libc::c_ulong,
                    );
                }
                i += 1;
            }
            tmp___6 = dcgettext(
                0 as *mut libc::c_void as *const libc::c_char,
                b"State file found: %jd bytes downloaded, %jd to go.\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            );
            axel_message(
                axel,
                tmp___6 as *const libc::c_char,
                (*axel).bytes_done,
                (*axel).size - (*axel).bytes_done,
            );
            close(fd);
            tmp___8 = open(
                ((*axel).filename).as_mut_ptr() as *const libc::c_char,
                1 as libc::c_int,
                438 as libc::c_int,
            );
            (*axel).outfd = tmp___8;
            if tmp___8 == -(1 as libc::c_int) {
                tmp___7 = dcgettext(
                    0 as *mut libc::c_void as *const libc::c_char,
                    b"Error opening local file\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                );
                axel_message(axel, tmp___7 as *const libc::c_char);
                return 0 as libc::c_int;
            }
        }
    }
    if (*axel).outfd == -(1 as libc::c_int) {
        axel_divide(axel);
        tmp___10 = open(
            ((*axel).filename).as_mut_ptr() as *const libc::c_char,
            65 as libc::c_int,
            438 as libc::c_int,
        );
        (*axel).outfd = tmp___10;
        if tmp___10 == -(1 as libc::c_int) {
            tmp___9 = dcgettext(
                0 as *mut libc::c_void as *const libc::c_char,
                b"Error opening local file\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            axel_message(axel, tmp___9 as *const libc::c_char);
            return 0 as libc::c_int;
        }
        tmp___16 = lseek((*axel).outfd, (*axel).size, 0 as libc::c_int);
        if tmp___16 == -(1 as libc::c_long) {
            if (*(*axel).conf).num_connections as libc::c_int > 1 as libc::c_int {
                tmp___11 = dcgettext(
                    0 as *mut libc::c_void as *const libc::c_char,
                    b"Crappy filesystem/OS.. Working around. :-(\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                );
                axel_message(axel, tmp___11 as *const libc::c_char);
                lseek((*axel).outfd, 0 as libc::c_int as __off_t, 0 as libc::c_int);
                memset(
                    buffer as *mut libc::c_void,
                    0 as libc::c_int,
                    (*(*axel).conf).buffer_size as size_t,
                );
                j = (*axel).size;
                while j > 0 as libc::c_long {
                    __a = j;
                    __b = (*(*axel).conf).buffer_size;
                    if __a < __b as off_t {
                        tmp___15 = __a;
                    } else {
                        tmp___15 = __b as off_t;
                    }
                    nwrite = write(
                        (*axel).outfd,
                        buffer as *const libc::c_void,
                        tmp___15 as size_t,
                    );
                    if nwrite < 0 as libc::c_long {
                        tmp___12 = __errno_location();
                        if *tmp___12 == 4 as libc::c_int {
                            continue;
                        }
                        tmp___13 = __errno_location();
                        if *tmp___13 == 11 as libc::c_int {
                            continue;
                        }
                        tmp___14 = dcgettext(
                            0 as *mut libc::c_void as *const libc::c_char,
                            b"Error creating local file\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        );
                        axel_message(axel, tmp___14 as *const libc::c_char);
                        return 0 as libc::c_int;
                    } else {
                        j -= nwrite;
                    }
                }
            }
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn reactivate_connection(
    mut axel: *mut axel_t,
    mut thread: libc::c_int,
) {
    let mut max_remaining: off_t = 0;
    let mut idx: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut remaining: off_t = 0;
    max_remaining = 102399 as libc::c_int as off_t;
    idx = -(1 as libc::c_int);
    if (*((*axel).conn).offset(thread as isize)).enabled {
        return
    } else {
        if (*((*axel).conn).offset(thread as isize)).currentbyte
            < (*((*axel).conn).offset(thread as isize)).lastbyte
        {
            return;
        }
    }
    j = 0 as libc::c_int;
    while j < (*(*axel).conf).num_connections as libc::c_int {
        remaining = (*((*axel).conn).offset(j as isize)).lastbyte
            - (*((*axel).conn).offset(j as isize)).currentbyte;
        if remaining > max_remaining {
            max_remaining = remaining;
            idx = j;
        }
        j += 1;
    }
    if idx == -(1 as libc::c_int) {
        return;
    }
    (*((*axel).conn).offset(thread as isize))
        .lastbyte = (*((*axel).conn).offset(idx as isize)).lastbyte;
    (*((*axel).conn).offset(idx as isize))
        .lastbyte = (*((*axel).conn).offset(idx as isize)).currentbyte
        + max_remaining / 2 as libc::c_long;
    (*((*axel).conn).offset(thread as isize))
        .currentbyte = (*((*axel).conn).offset(idx as isize)).lastbyte;
}
pub unsafe extern "C" fn axel_start(mut axel: *mut axel_t) {
    let mut i: libc::c_int = 0;
    let mut url_ptr: *mut url_t = 0 as *mut url_t;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: libc::c_int = 0;
    url_ptr = (*axel).url;
    i = 0 as libc::c_int;
    while i < (*(*axel).conf).num_connections as libc::c_int {
        conn_set(
            ((*axel).conn).offset(i as isize),
            ((*url_ptr).text).as_mut_ptr() as *const libc::c_char,
        );
        url_ptr = (*url_ptr).next as *mut url_t;
        let ref mut fresh4 = (*((*axel).conn).offset(i as isize)).local_if;
        *fresh4 = ((*(*(*axel).conf).interfaces).text).as_mut_ptr();
        (*(*axel).conf).interfaces = (*(*(*axel).conf).interfaces).next as *mut if_t;
        let ref mut fresh5 = (*((*axel).conn).offset(i as isize)).conf;
        *fresh5 = (*axel).conf;
        if i != 0 {
            (*((*axel).conn).offset(i as isize)).supported = 1 as libc::c_int != 0;
        }
        i += 1;
    }
    if (*(*axel).conf).verbose > 0 as libc::c_int {
        tmp = dcgettext(
            0 as *mut libc::c_void as *const libc::c_char,
            b"Starting download\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        axel_message(axel, tmp as *const libc::c_char);
    }
    i = 0 as libc::c_int;
    while i < (*(*axel).conf).num_connections as libc::c_int {
        if (*((*axel).conn).offset(i as isize)).currentbyte
            >= (*((*axel).conn).offset(i as isize)).lastbyte
        {
            pthread_mutex_lock(&mut (*((*axel).conn).offset(i as isize)).lock);
            reactivate_connection(axel, i);
            pthread_mutex_unlock(&mut (*((*axel).conn).offset(i as isize)).lock);
        } else if (*((*axel).conn).offset(i as isize)).currentbyte
                < (*((*axel).conn).offset(i as isize)).lastbyte
            {
            if (*(*axel).conf).verbose >= 2 as libc::c_int {
                tmp___0 = dcgettext(
                    0 as *mut libc::c_void as *const libc::c_char,
                    b"Connection %i downloading from %s:%i using interface %s\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                );
                axel_message(
                    axel,
                    tmp___0 as *const libc::c_char,
                    i,
                    ((*((*axel).conn).offset(i as isize)).host).as_mut_ptr(),
                    (*((*axel).conn).offset(i as isize)).port,
                    (*((*axel).conn).offset(i as isize)).local_if,
                );
            }
            (*((*axel).conn).offset(i as isize)).state = 1 as libc::c_int != 0;
            tmp___2 = pthread_create(
                ((*((*axel).conn).offset(i as isize)).setup_thread).as_mut_ptr(),
                0 as *mut libc::c_void as *const pthread_attr_t,
                Some(
                    setup_thread
                        as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                ),
                ((*axel).conn).offset(i as isize) as *mut libc::c_void,
            );
            if tmp___2 != 0 as libc::c_int {
                tmp___1 = dcgettext(
                    0 as *mut libc::c_void as *const libc::c_char,
                    b"pthread error!!!\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                );
                axel_message(axel, tmp___1 as *const libc::c_char);
                (*axel).ready = -(1 as libc::c_int);
            }
        }
        i += 1;
    }
    (*axel).start_time = axel_gettime();
    (*axel).ready = 0 as libc::c_int;
}
pub unsafe extern "C" fn axel_do(mut axel: *mut axel_t) {
    let mut fds: [fd_set; 1] = [fd_set { fds_bits: [0; 16] }; 1];
    let mut hifd: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut remaining: off_t = 0;
    let mut size: off_t = 0;
    let mut timeval: [timeval; 1] = [timeval { tv_sec: 0, tv_usec: 0 }; 1];
    let mut url_ptr: *mut url_t = 0 as *mut url_t;
    let mut delay: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut max_speed_ratio: libc::c_ulonglong = 0;
    let mut tmp: libc::c_double = 0.;
    let mut tmp___0: libc::c_double = 0.;
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let mut __a: libc::c_int = 0;
    let mut __b: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut timeout: time_t = 0;
    let mut tmp___9: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___10: libc::c_double = 0.;
    let mut tmp___11: libc::c_double = 0.;
    let mut tmp___12: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___13: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___14: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___15: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___16: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___17: ssize_t = 0;
    let mut tmp___18: libc::c_int = 0;
    let mut tmp___19: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___20: libc::c_double = 0.;
    let mut tmp___21: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___22: libc::c_int = 0;
    let mut tmp___23: libc::c_double = 0.;
    let mut tmp___24: libc::c_double = 0.;
    let mut tmp___25: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___26: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___27: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___28: libc::c_int = 0;
    delay.tv_sec = 0 as libc::c_int as __time_t;
    delay.tv_nsec = 100000000 as libc::c_int as __syscall_slong_t;
    tmp___0 = axel_gettime();
    if tmp___0 > (*axel).next_state as libc::c_double {
        save_state(axel);
        tmp = axel_gettime();
        (*axel)
            .next_state = (tmp + (*(*axel).conf).save_state_interval as libc::c_double)
            as libc::c_int;
    }
    let fresh6 = &mut __d0;
    let fresh7;
    let fresh8 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh9 = &mut __d1;
    let fresh10;
    let fresh11 = &mut *((*fds.as_mut_ptr().offset(0 as libc::c_int as isize)).fds_bits)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh6,
        fresh8) => fresh7, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh9,
        fresh11) => fresh10, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
    c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
    hifd = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*(*axel).conf).num_connections as libc::c_int {
        tmp___2 = pthread_mutex_trylock(&mut (*((*axel).conn).offset(i as isize)).lock);
        if tmp___2 == 0 {
            if (*((*axel).conn).offset(i as isize)).enabled {
                fds[0 as libc::c_int as usize]
                    .fds_bits[((*(*((*axel).conn).offset(i as isize)).tcp).fd
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    |= ((1 as libc::c_ulong)
                        << (*(*((*axel).conn).offset(i as isize)).tcp).fd
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask;
                __a = hifd;
                __b = (*(*((*axel).conn).offset(i as isize)).tcp).fd;
                if __a > __b {
                    tmp___1 = __a;
                } else {
                    tmp___1 = __b;
                }
                hifd = tmp___1;
            }
            pthread_mutex_unlock(&mut (*((*axel).conn).offset(i as isize)).lock);
        }
        i += 1;
    }
    if hifd == 0 as libc::c_int {
        tmp___6 = axel_sleep(delay);
        if tmp___6 < 0 as libc::c_int {
            tmp___3 = __errno_location();
            tmp___4 = strerror(*tmp___3);
            tmp___5 = dcgettext(
                0 as *mut libc::c_void as *const libc::c_char,
                b"Error while waiting for connection: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            );
            axel_message(axel, tmp___5 as *const libc::c_char, tmp___4);
            (*axel).ready = -(1 as libc::c_int);
            return;
        }
    } else {
        timeval[0 as libc::c_int as usize].tv_sec = 0 as libc::c_int as __time_t;
        timeval[0 as libc::c_int as usize]
            .tv_usec = 100000 as libc::c_int as __suseconds_t;
        tmp___7 = select(
            hifd + 1 as libc::c_int,
            fds.as_mut_ptr(),
            0 as *mut libc::c_void as *mut fd_set,
            0 as *mut libc::c_void as *mut fd_set,
            timeval.as_mut_ptr(),
        );
        if tmp___7 == -(1 as libc::c_int) {
            (*axel).ready = -(1 as libc::c_int);
            return;
        }
        i = 0 as libc::c_int;
        while i < (*(*axel).conf).num_connections as libc::c_int {
            tmp___8 = pthread_mutex_trylock(
                &mut (*((*axel).conn).offset(i as isize)).lock,
            );
            if !(tmp___8 != 0) {
                if (*((*axel).conn).offset(i as isize)).enabled {
                    if !(fds[0 as libc::c_int as usize]
                        .fds_bits[((*(*((*axel).conn).offset(i as isize)).tcp).fd
                        / (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as usize]
                        & ((1 as libc::c_ulong)
                            << (*(*((*axel).conn).offset(i as isize)).tcp).fd
                                % (8 as libc::c_int
                                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                        as libc::c_int)) as __fd_mask != 0 as libc::c_long)
                    {
                        timeout = ((*((*axel).conn).offset(i as isize)).last_transfer
                            + (*(*axel).conf).connection_timeout) as time_t;
                        tmp___10 = axel_gettime();
                        if tmp___10 > timeout as libc::c_double {
                            if (*(*axel).conf).verbose != 0 {
                                tmp___9 = dcgettext(
                                    0 as *mut libc::c_void as *const libc::c_char,
                                    b"Connection %i timed out\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                );
                                axel_message(axel, tmp___9 as *const libc::c_char, i);
                            }
                            conn_disconnect(((*axel).conn).offset(i as isize));
                        }
                    } else {
                        tmp___11 = axel_gettime();
                        (*((*axel).conn).offset(i as isize))
                            .last_transfer = tmp___11 as libc::c_int;
                        size = tcp_read(
                            (*((*axel).conn).offset(i as isize)).tcp,
                            buffer as *mut libc::c_void,
                            (*(*axel).conf).buffer_size,
                        );
                        if size == -(1 as libc::c_long) {
                            if (*(*axel).conf).verbose != 0 {
                                tmp___12 = dcgettext(
                                    0 as *mut libc::c_void as *const libc::c_char,
                                    b"Error on connection %i! Connection closed\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                );
                                axel_message(axel, tmp___12 as *const libc::c_char, i);
                            }
                            conn_disconnect(((*axel).conn).offset(i as isize));
                        } else if size == 0 as libc::c_long {
                            if (*(*axel).conf).verbose != 0 {
                                if (*((*axel).conn).offset(i as isize)).currentbyte
                                    < (*((*axel).conn).offset(i as isize)).lastbyte
                                {
                                    if (*axel).size as libc::c_longlong
                                        != 9223372036854775807 as libc::c_longlong
                                    {
                                        tmp___13 = dcgettext(
                                            0 as *mut libc::c_void as *const libc::c_char,
                                            b"Connection %i unexpectedly closed\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        );
                                        axel_message(axel, tmp___13 as *const libc::c_char, i);
                                    } else {
                                        tmp___14 = dcgettext(
                                            0 as *mut libc::c_void as *const libc::c_char,
                                            b"Connection %i finished\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        );
                                        axel_message(axel, tmp___14 as *const libc::c_char, i);
                                    }
                                } else {
                                    tmp___14 = dcgettext(
                                        0 as *mut libc::c_void as *const libc::c_char,
                                        b"Connection %i finished\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    );
                                    axel_message(axel, tmp___14 as *const libc::c_char, i);
                                }
                            }
                            if !(*((*axel).conn).offset(0 as libc::c_int as isize))
                                .supported
                            {
                                (*axel).ready = 1 as libc::c_int;
                            }
                            conn_disconnect(((*axel).conn).offset(i as isize));
                            reactivate_connection(axel, i);
                        } else {
                            remaining = (*((*axel).conn).offset(i as isize)).lastbyte
                                - (*((*axel).conn).offset(i as isize)).currentbyte;
                            if remaining < size {
                                if (*(*axel).conf).verbose != 0 {
                                    tmp___15 = dcgettext(
                                        0 as *mut libc::c_void as *const libc::c_char,
                                        b"Connection %i finished\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    );
                                    axel_message(axel, tmp___15 as *const libc::c_char, i);
                                }
                                conn_disconnect(((*axel).conn).offset(i as isize));
                                size = remaining;
                            }
                            lseek(
                                (*axel).outfd,
                                (*((*axel).conn).offset(i as isize)).currentbyte,
                                0 as libc::c_int,
                            );
                            tmp___17 = write(
                                (*axel).outfd,
                                buffer as *const libc::c_void,
                                size as size_t,
                            );
                            if tmp___17 != size {
                                tmp___16 = dcgettext(
                                    0 as *mut libc::c_void as *const libc::c_char,
                                    b"Write error!\0" as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                );
                                axel_message(axel, tmp___16 as *const libc::c_char);
                                (*axel).ready = -(1 as libc::c_int);
                                pthread_mutex_unlock(
                                    &mut (*((*axel).conn).offset(i as isize)).lock,
                                );
                                return;
                            }
                            let ref mut fresh12 = (*((*axel).conn).offset(i as isize))
                                .currentbyte;
                            *fresh12 += size;
                            (*axel).bytes_done += size;
                            if remaining == size {
                                reactivate_connection(axel, i);
                            }
                        }
                    }
                }
                pthread_mutex_unlock(&mut (*((*axel).conn).offset(i as isize)).lock);
            }
            i += 1;
        }
        if (*axel).ready != 0 {
            return;
        }
    }
    url_ptr = (*axel).url;
    i = 0 as libc::c_int;
    while i < (*(*axel).conf).num_connections as libc::c_int {
        tmp___18 = pthread_mutex_trylock(&mut (*((*axel).conn).offset(i as isize)).lock);
        if !(tmp___18 != 0) {
            if !(*((*axel).conn).offset(i as isize)).enabled {
                if (*((*axel).conn).offset(i as isize)).currentbyte
                    < (*((*axel).conn).offset(i as isize)).lastbyte
                {
                    if !(*((*axel).conn).offset(i as isize)).state {
                        pthread_join(
                            (*((*axel).conn).offset(i as isize))
                                .setup_thread[0 as libc::c_int as usize],
                            0 as *mut libc::c_void as *mut *mut libc::c_void,
                        );
                        conn_set(
                            ((*axel).conn).offset(i as isize),
                            ((*url_ptr).text).as_mut_ptr() as *const libc::c_char,
                        );
                        url_ptr = (*url_ptr).next as *mut url_t;
                        if (*(*axel).conf).verbose >= 2 as libc::c_int {
                            tmp___19 = dcgettext(
                                0 as *mut libc::c_void as *const libc::c_char,
                                b"Connection %i downloading from %s:%i using interface %s\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            );
                            axel_message(
                                axel,
                                tmp___19 as *const libc::c_char,
                                i,
                                ((*((*axel).conn).offset(i as isize)).host).as_mut_ptr(),
                                (*((*axel).conn).offset(i as isize)).port,
                                (*((*axel).conn).offset(i as isize)).local_if,
                            );
                        }
                        (*((*axel).conn).offset(i as isize))
                            .state = 1 as libc::c_int != 0;
                        tmp___22 = pthread_create(
                            ((*((*axel).conn).offset(i as isize)).setup_thread)
                                .as_mut_ptr(),
                            0 as *mut libc::c_void as *const pthread_attr_t,
                            Some(
                                setup_thread
                                    as unsafe extern "C" fn(
                                        *mut libc::c_void,
                                    ) -> *mut libc::c_void,
                            ),
                            ((*axel).conn).offset(i as isize) as *mut libc::c_void,
                        );
                        if tmp___22 == 0 as libc::c_int {
                            tmp___20 = axel_gettime();
                            (*((*axel).conn).offset(i as isize))
                                .last_transfer = tmp___20 as libc::c_int;
                        } else {
                            tmp___21 = dcgettext(
                                0 as *mut libc::c_void as *const libc::c_char,
                                b"pthread error!!!\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            );
                            axel_message(axel, tmp___21 as *const libc::c_char);
                            (*axel).ready = -(1 as libc::c_int);
                        }
                    } else {
                        tmp___23 = axel_gettime();
                        if tmp___23
                            > ((*((*axel).conn).offset(i as isize)).last_transfer
                                + (*(*axel).conf).reconnect_delay) as libc::c_double
                        {
                            pthread_cancel(
                                (*((*axel).conn).offset(i as isize))
                                    .setup_thread[0 as libc::c_int as usize],
                            );
                            (*((*axel).conn).offset(i as isize))
                                .state = 0 as libc::c_int != 0;
                            pthread_join(
                                (*((*axel).conn).offset(i as isize))
                                    .setup_thread[0 as libc::c_int as usize],
                                0 as *mut libc::c_void as *mut *mut libc::c_void,
                            );
                        }
                    }
                }
            }
            pthread_mutex_unlock(&mut (*((*axel).conn).offset(i as isize)).lock);
        }
        i += 1;
    }
    tmp___24 = axel_gettime();
    (*axel)
        .bytes_per_second = (((*axel).bytes_done - (*axel).start_byte) as libc::c_double
        / (tmp___24 - (*axel).start_time)) as off_t as libc::c_longlong;
    if (*axel).bytes_per_second != 0 as libc::c_longlong {
        (*axel)
            .finish_time = ((*axel).start_time
            + ((*axel).size - (*axel).start_byte) as libc::c_double
                / (*axel).bytes_per_second as libc::c_double) as libc::c_int;
    } else {
        (*axel).finish_time = 2147483647 as libc::c_int;
    }
    if (*(*axel).conf).max_speed > 0 as libc::c_ulonglong {
        max_speed_ratio = ((1000 as libc::c_longlong * (*axel).bytes_per_second)
            as libc::c_ulonglong)
            .wrapping_div((*(*axel).conf).max_speed);
        if max_speed_ratio > 1050 as libc::c_ulonglong {
            (*axel).delay_time.tv_nsec += 10000000 as libc::c_long;
            if (*axel).delay_time.tv_nsec >= 1000000000 as libc::c_long {
                (*axel).delay_time.tv_sec += 1;
                (*axel).delay_time.tv_nsec -= 1000000000 as libc::c_long;
            }
        } else if max_speed_ratio < 950 as libc::c_ulonglong {
            if (*axel).delay_time.tv_nsec >= 10000000 as libc::c_long {
                (*axel).delay_time.tv_nsec -= 10000000 as libc::c_long;
            } else if (*axel).delay_time.tv_sec > 0 as libc::c_long {
                (*axel).delay_time.tv_sec -= 1;
                (*axel).delay_time.tv_nsec += 999000000 as libc::c_long;
            } else {
                (*axel).delay_time.tv_sec = 0 as libc::c_int as __time_t;
                (*axel).delay_time.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
            }
        }
        tmp___28 = axel_sleep((*axel).delay_time);
        if tmp___28 < 0 as libc::c_int {
            tmp___25 = __errno_location();
            tmp___26 = strerror(*tmp___25);
            tmp___27 = dcgettext(
                0 as *mut libc::c_void as *const libc::c_char,
                b"Error while enforcing throttling: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            );
            axel_message(axel, tmp___27 as *const libc::c_char, tmp___26);
            (*axel).ready = -(1 as libc::c_int);
            return;
        }
    }
    if (*axel).bytes_done == (*axel).size {
        (*axel).ready = 1 as libc::c_int;
    }
}
pub unsafe extern "C" fn axel_close(mut axel: *mut axel_t) {
    let mut i: libc::c_int = 0;
    if axel.is_null() {
        return;
    }
    i = 0 as libc::c_int;
    while i < (*(*axel).conf).num_connections as libc::c_int {
        if (*((*axel).conn).offset(i as isize)).setup_thread[0 as libc::c_int as usize]
            != 0 as libc::c_ulong
        {
            pthread_cancel(
                (*((*axel).conn).offset(i as isize))
                    .setup_thread[0 as libc::c_int as usize],
            );
            pthread_join(
                (*((*axel).conn).offset(i as isize))
                    .setup_thread[0 as libc::c_int as usize],
                0 as *mut libc::c_void as *mut *mut libc::c_void,
            );
        }
        conn_disconnect(((*axel).conn).offset(i as isize));
        i += 1;
    }
    free((*axel).url as *mut libc::c_void);
    if (*axel).ready == 1 as libc::c_int {
        stfile_unlink(((*axel).filename).as_mut_ptr() as *const libc::c_char);
    } else if (*axel).bytes_done > 0 as libc::c_long {
        save_state(axel);
    }
    print_messages(axel);
    close((*axel).outfd);
    if !((*(*axel).conn).proto & (1 as libc::c_int) << 1 as libc::c_int
        == 0 as libc::c_int)
    {
        abuf_setup(
            ((*(*axel).conn).http[0 as libc::c_int as usize].request).as_mut_ptr(),
            0 as libc::c_int as size_t,
        );
        abuf_setup(
            ((*(*axel).conn).http[0 as libc::c_int as usize].headers).as_mut_ptr(),
            0 as libc::c_int as size_t,
        );
    } else if (*(*axel).conn).proxy != 0 {
        abuf_setup(
            ((*(*axel).conn).http[0 as libc::c_int as usize].request).as_mut_ptr(),
            0 as libc::c_int as size_t,
        );
        abuf_setup(
            ((*(*axel).conn).http[0 as libc::c_int as usize].headers).as_mut_ptr(),
            0 as libc::c_int as size_t,
        );
    }
    free((*axel).conn as *mut libc::c_void);
    free(axel as *mut libc::c_void);
    free(buffer as *mut libc::c_void);
}
pub unsafe extern "C" fn axel_gettime() -> libc::c_double {
    let mut time___0: [timeval; 1] = [timeval { tv_sec: 0, tv_usec: 0 }; 1];
    gettimeofday(time___0.as_mut_ptr(), 0 as *mut libc::c_void);
    return time___0[0 as libc::c_int as usize].tv_sec as libc::c_double
        + time___0[0 as libc::c_int as usize].tv_usec as libc::c_double
            / 1000000 as libc::c_int as libc::c_double;
}
unsafe extern "C" fn save_state(mut axel: *mut axel_t) {
    let mut fd: libc::c_int = 0;
    let mut nwrite: ssize_t = 0;
    let mut i: libc::c_int = 0;
    if !(*((*axel).conn).offset(0 as libc::c_int as isize)).supported {
        return;
    }
    fd = stfile_open(
        ((*axel).filename).as_mut_ptr() as *const libc::c_char,
        577 as libc::c_int,
        438 as libc::c_int as mode_t,
    );
    if fd == -(1 as libc::c_int) {
        return;
    }
    nwrite = write(
        fd,
        &mut (*(*axel).conf).num_connections as *mut uint16_t as *const libc::c_void,
        ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
    );
    nwrite = write(
        fd,
        &mut (*axel).bytes_done as *mut off_t as *const libc::c_void,
        ::std::mem::size_of::<off_t>() as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while i < (*(*axel).conf).num_connections as libc::c_int {
        nwrite = write(
            fd,
            &mut (*((*axel).conn).offset(i as isize)).currentbyte as *mut off_t
                as *const libc::c_void,
            ::std::mem::size_of::<off_t>() as libc::c_ulong,
        );
        nwrite = write(
            fd,
            &mut (*((*axel).conn).offset(i as isize)).lastbyte as *mut off_t
                as *const libc::c_void,
            ::std::mem::size_of::<off_t>() as libc::c_ulong,
        );
        i += 1;
    }
    close(fd);
}
unsafe extern "C" fn setup_thread(mut c: *mut libc::c_void) -> *mut libc::c_void {
    let mut conn: *mut conn_t = 0 as *mut conn_t;
    let mut oldstate: libc::c_int = 0;
    let mut tmp: libc::c_double = 0.;
    let mut tmp___0: libc::c_double = 0.;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut current_block_14: u64;
    conn = c as *mut conn_t;
    pthread_setcancelstate(0 as libc::c_int, &mut oldstate);
    pthread_setcanceltype(1 as libc::c_int, &mut oldstate);
    pthread_mutex_lock(&mut (*conn).lock);
    tmp___2 = conn_setup(conn);
    if tmp___2 != 0 {
        tmp = axel_gettime();
        (*conn).last_transfer = tmp as libc::c_int;
        tmp___1 = conn_exec(conn);
        if tmp___1 != 0 {
            tmp___0 = axel_gettime();
            (*conn).last_transfer = tmp___0 as libc::c_int;
            (*conn).enabled = 1 as libc::c_int != 0;
            current_block_14 = 16310787081864580311;
        } else {
            current_block_14 = 12209867499936983673;
        }
    } else {
        current_block_14 = 12209867499936983673;
    }
    match current_block_14 {
        12209867499936983673 => {
            conn_disconnect(conn);
        }
        _ => {}
    }
    (*conn).state = 0 as libc::c_int != 0;
    pthread_mutex_unlock(&mut (*conn).lock);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn axel_message(
    mut axel: *mut axel_t,
    mut format: *const libc::c_char,
    mut args: ...
) {
    let mut m: *mut message_t = 0 as *mut message_t;
    let mut params: ::std::ffi::VaListImpl;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut message_t = 0 as *mut message_t;
    if !axel.is_null() {
        tmp = calloc(
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<message_t>() as libc::c_ulong,
        );
        m = tmp as *mut message_t;
        if !m.is_null() {
            params = args.clone();
            vsnprintf(
                ((*m).text).as_mut_ptr(),
                1024 as libc::c_int as size_t,
                format,
                params.as_va_list(),
            );
            if (*axel).message as libc::c_ulong
                == 0 as *mut libc::c_void as libc::c_ulong
            {
                tmp___0 = m;
                (*axel).last_message = tmp___0;
                (*axel).message = tmp___0;
            } else {
                (*(*axel).last_message).next = m as *mut libc::c_void;
                (*axel).last_message = m;
            }
            return;
        }
    }
    print_messages(axel);
    params = args.clone();
    vprintf(format, params.as_va_list());
}
unsafe extern "C" fn axel_divide(mut axel: *mut axel_t) {
    let mut maxconns: off_t = 0;
    let mut __a: libc::c_uint = 0;
    let mut __b: off_t = 0;
    let mut tmp: off_t = 0;
    let mut seg_len: off_t = 0;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_conn: *mut conn_t = 0 as *mut conn_t;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    let mut tail: size_t = 0;
    __a = 1 as libc::c_uint;
    __b = (*axel).size / 102400 as libc::c_long;
    if __a as off_t > __b {
        tmp = __a as off_t;
    } else {
        tmp = __b;
    }
    maxconns = tmp;
    if maxconns < (*(*axel).conf).num_connections as off_t {
        (*(*axel).conf).num_connections = maxconns as uint16_t;
    }
    seg_len = (*axel).size / (*(*axel).conf).num_connections as off_t;
    if seg_len == 0 {
        tmp___0 = dcgettext(
            0 as *mut libc::c_void as *const libc::c_char,
            b"Too few bytes remaining, forcing a single connection\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        );
        printf(tmp___0 as *const libc::c_char);
        (*(*axel).conf).num_connections = 1 as libc::c_int as uint16_t;
        seg_len = (*axel).size;
        tmp___1 = realloc(
            (*axel).conn as *mut libc::c_void,
            ::std::mem::size_of::<conn_t>() as libc::c_ulong,
        );
        new_conn = tmp___1 as *mut conn_t;
        if !new_conn.is_null() {
            (*axel).conn = new_conn;
        }
    }
    i = 0 as libc::c_int;
    while i < (*(*axel).conf).num_connections as libc::c_int {
        (*((*axel).conn).offset(i as isize)).currentbyte = seg_len * i as off_t;
        (*((*axel).conn).offset(i as isize)).lastbyte = seg_len * i as off_t + seg_len;
        i += 1;
    }
    tail = ((*axel).size % seg_len) as size_t;
    (*((*axel).conn)
        .offset(
            ((*(*axel).conf).num_connections as libc::c_int - 1 as libc::c_int) as isize,
        ))
        .lastbyte = ((*((*axel).conn)
        .offset(
            ((*(*axel).conf).num_connections as libc::c_int - 1 as libc::c_int) as isize,
        ))
        .lastbyte as size_t)
        .wrapping_add(tail) as off_t;
}
pub unsafe extern "C" fn axel_sleep(mut delay: timespec) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    loop {
        res = nanosleep(&mut delay as *mut timespec as *const timespec, &mut delay);
        if !(res != 0) {
            break;
        }
        tmp = __errno_location();
        if !(*tmp == 4 as libc::c_int) {
            break;
        }
    }
    return res;
}
#[inline]
unsafe extern "C" fn conf_hdr_make(
    mut dst: *mut libc::c_char,
    mut k: *const libc::c_char,
    mut v: *const libc::c_char,
) {
    snprintf(
        dst,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        b"%s: %s\0" as *const u8 as *const libc::c_char,
        k,
        v,
    );
}
unsafe extern "C" fn axel_fscanf(
    mut fp: *mut FILE,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut params: ::std::ffi::VaListImpl;
    let mut ret: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    params = args.clone();
    ret = vfscanf(fp, format, params.as_va_list());
    if ret == -(1 as libc::c_int) {
        tmp = ferror(fp);
        if tmp != 0 {
            tmp___0 = 0 as libc::c_int;
        } else {
            tmp___0 = 1 as libc::c_int;
        }
    } else {
        tmp___0 = 1 as libc::c_int;
    }
    ret = tmp___0;
    if ret == 0 {
        tmp___1 = __errno_location();
        tmp___2 = strerror(*tmp___1);
        tmp___3 = dcgettext(
            0 as *mut libc::c_void as *const libc::c_char,
            b"I/O error while reading config file: %s\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        );
        fprintf(stderr, tmp___3 as *const libc::c_char, tmp___2);
    }
    return ret;
}
unsafe extern "C" fn parse_protocol(
    mut conf: *mut conf_t,
    mut value: *const libc::c_char,
) -> libc::c_int {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    tmp___1 = strcasecmp(value, b"ipv4\0" as *const u8 as *const libc::c_char);
    if tmp___1 == 0 as libc::c_int {
        (*conf).ai_family = 2 as libc::c_int as sa_family_t;
    } else {
        tmp___0 = strcasecmp(value, b"ipv6\0" as *const u8 as *const libc::c_char);
        if tmp___0 == 0 as libc::c_int {
            (*conf).ai_family = 10 as libc::c_int as sa_family_t;
        } else {
            tmp = dcgettext(
                0 as *mut libc::c_void as *const libc::c_char,
                b"Unknown protocol %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            fprintf(stderr, tmp as *const libc::c_char, value);
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn conf_loadfile(
    mut conf: *mut conf_t,
    mut file: *const libc::c_char,
) -> libc::c_int {
    let mut line: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut s: [libc::c_char; 1024] = [0; 1024];
    let mut key: [libc::c_char; 1024] = [0; 1024];
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dst: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut tmp___3: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
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
    let mut num: libc::c_int = 0;
    let mut tmp___26: libc::c_int = 0;
    let mut tmp___27: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___28: libc::c_int = 0;
    let mut tmp___29: libc::c_int = 0;
    let mut tmp___30: libc::c_int = 0;
    let mut tmp___31: libc::c_int = 0;
    let mut tmp___32: libc::c_int = 0;
    let mut tmp___33: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___34: libc::c_int = 0;
    line = 0 as libc::c_int;
    ret = 1 as libc::c_int;
    fp = fopen(file, b"r\0" as *const u8 as *const libc::c_char);
    if fp as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 1 as libc::c_int;
    }
    let mut current_block_103: u64;
    loop {
        tmp___34 = feof(fp);
        if tmp___34 != 0 {
            break;
        }
        value = 0 as *mut libc::c_void as *mut libc::c_char;
        line += 1;
        s[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        ret = axel_fscanf(
            fp,
            b"%100[^\n#]s\0" as *const u8 as *const libc::c_char,
            s.as_mut_ptr(),
        );
        if ret == 0 {
            break;
        }
        ret = axel_fscanf(fp, b"%*[^\n]s\0" as *const u8 as *const libc::c_char);
        if ret == 0 {
            break;
        }
        tmp___0 = fgetc(fp);
        if tmp___0 != 10 as libc::c_int {
            tmp___1 = feof(fp);
            if tmp___1 == 0 {
                fprintf(
                    stderr,
                    b"Expected newline\n\0" as *const u8 as *const libc::c_char,
                );
                current_block_103 = 11329907657253921262;
            } else {
                current_block_103 = 11385396242402735691;
            }
        } else {
            current_block_103 = 11385396242402735691;
        }
        match current_block_103 {
            11385396242402735691 => {
                tmp = strchr(s.as_mut_ptr() as *const libc::c_char, '=' as i32);
                if tmp as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                    continue;
                }
                sscanf(
                    s.as_mut_ptr() as *const libc::c_char,
                    b"%[^= \t]s\0" as *const u8 as *const libc::c_char,
                    key.as_mut_ptr(),
                );
                loop {
                    tmp___2 = __ctype_b_loc();
                    tmp = tmp.offset(1);
                    if *(*tmp___2).offset(*tmp as libc::c_int as isize) as libc::c_int
                        & 8192 as libc::c_int == 0
                    {
                        break;
                    }
                }
                value = tmp;
                while *tmp != 0 {
                    tmp___3 = __ctype_b_loc();
                    if *(*tmp___3).offset(*tmp as libc::c_int as isize) as libc::c_int
                        & 8192 as libc::c_int != 0
                    {
                        break;
                    }
                    tmp = tmp.offset(1);
                }
                *tmp = '\u{0}' as i32 as libc::c_char;
                tmp___6 = strcmp(
                    key.as_mut_ptr() as *const libc::c_char,
                    b"default_filename\0" as *const u8 as *const libc::c_char,
                );
                if tmp___6 != 0 {
                    tmp___5 = strcmp(
                        key.as_mut_ptr() as *const libc::c_char,
                        b"http_proxy\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___5 != 0 {
                        tmp___4 = strcmp(
                            key.as_mut_ptr() as *const libc::c_char,
                            b"no_proxy\0" as *const u8 as *const libc::c_char,
                        );
                        if tmp___4 != 0 {
                            tmp___22 = strcmp(
                                key.as_mut_ptr() as *const libc::c_char,
                                b"strip_cgi_parameters\0" as *const u8
                                    as *const libc::c_char,
                            );
                            if tmp___22 != 0 {
                                tmp___21 = strcmp(
                                    key.as_mut_ptr() as *const libc::c_char,
                                    b"save_state_interval\0" as *const u8 as *const libc::c_char,
                                );
                                if tmp___21 != 0 {
                                    tmp___20 = strcmp(
                                        key.as_mut_ptr() as *const libc::c_char,
                                        b"connection_timeout\0" as *const u8 as *const libc::c_char,
                                    );
                                    if tmp___20 != 0 {
                                        tmp___19 = strcmp(
                                            key.as_mut_ptr() as *const libc::c_char,
                                            b"reconnect_delay\0" as *const u8 as *const libc::c_char,
                                        );
                                        if tmp___19 != 0 {
                                            tmp___18 = strcmp(
                                                key.as_mut_ptr() as *const libc::c_char,
                                                b"max_redirect\0" as *const u8 as *const libc::c_char,
                                            );
                                            if tmp___18 != 0 {
                                                tmp___17 = strcmp(
                                                    key.as_mut_ptr() as *const libc::c_char,
                                                    b"buffer_size\0" as *const u8 as *const libc::c_char,
                                                );
                                                if tmp___17 != 0 {
                                                    tmp___16 = strcmp(
                                                        key.as_mut_ptr() as *const libc::c_char,
                                                        b"max_speed\0" as *const u8 as *const libc::c_char,
                                                    );
                                                    if tmp___16 != 0 {
                                                        tmp___15 = strcmp(
                                                            key.as_mut_ptr() as *const libc::c_char,
                                                            b"verbose\0" as *const u8 as *const libc::c_char,
                                                        );
                                                        if tmp___15 != 0 {
                                                            tmp___14 = strcmp(
                                                                key.as_mut_ptr() as *const libc::c_char,
                                                                b"alternate_output\0" as *const u8 as *const libc::c_char,
                                                            );
                                                            if tmp___14 != 0 {
                                                                tmp___13 = strcmp(
                                                                    key.as_mut_ptr() as *const libc::c_char,
                                                                    b"percentage\0" as *const u8 as *const libc::c_char,
                                                                );
                                                                if tmp___13 != 0 {
                                                                    tmp___12 = strcmp(
                                                                        key.as_mut_ptr() as *const libc::c_char,
                                                                        b"insecure\0" as *const u8 as *const libc::c_char,
                                                                    );
                                                                    if tmp___12 != 0 {
                                                                        tmp___11 = strcmp(
                                                                            key.as_mut_ptr() as *const libc::c_char,
                                                                            b"no_clobber\0" as *const u8 as *const libc::c_char,
                                                                        );
                                                                        if tmp___11 != 0 {
                                                                            tmp___10 = strcmp(
                                                                                key.as_mut_ptr() as *const libc::c_char,
                                                                                b"search_timeout\0" as *const u8 as *const libc::c_char,
                                                                            );
                                                                            if tmp___10 != 0 {
                                                                                tmp___9 = strcmp(
                                                                                    key.as_mut_ptr() as *const libc::c_char,
                                                                                    b"search_threads\0" as *const u8 as *const libc::c_char,
                                                                                );
                                                                                if tmp___9 != 0 {
                                                                                    tmp___8 = strcmp(
                                                                                        key.as_mut_ptr() as *const libc::c_char,
                                                                                        b"search_amount\0" as *const u8 as *const libc::c_char,
                                                                                    );
                                                                                    if tmp___8 != 0 {
                                                                                        tmp___7 = strcmp(
                                                                                            key.as_mut_ptr() as *const libc::c_char,
                                                                                            b"search_top\0" as *const u8 as *const libc::c_char,
                                                                                        );
                                                                                        if tmp___7 != 0 {
                                                                                            tmp___23 = strcmp(
                                                                                                key.as_mut_ptr() as *const libc::c_char,
                                                                                                b"max_speed\0" as *const u8 as *const libc::c_char,
                                                                                            );
                                                                                            if tmp___23 != 0 {
                                                                                                tmp___32 = strcmp(
                                                                                                    key.as_mut_ptr() as *const libc::c_char,
                                                                                                    b"speed_type\0" as *const u8 as *const libc::c_char,
                                                                                                );
                                                                                                if tmp___32 == 0 as libc::c_int {
                                                                                                    continue;
                                                                                                }
                                                                                                tmp___31 = strcmp(
                                                                                                    key.as_mut_ptr() as *const libc::c_char,
                                                                                                    b"interfaces\0" as *const u8 as *const libc::c_char,
                                                                                                );
                                                                                                if tmp___31 == 0 as libc::c_int {
                                                                                                    tmp___24 = parse_interfaces(conf, value);
                                                                                                    if tmp___24 != 0 {
                                                                                                        continue;
                                                                                                    }
                                                                                                } else {
                                                                                                    tmp___30 = strcmp(
                                                                                                        key.as_mut_ptr() as *const libc::c_char,
                                                                                                        b"use_protocol\0" as *const u8 as *const libc::c_char,
                                                                                                    );
                                                                                                    if tmp___30 == 0 as libc::c_int {
                                                                                                        tmp___25 = parse_protocol(
                                                                                                            conf,
                                                                                                            value as *const libc::c_char,
                                                                                                        );
                                                                                                        if tmp___25 != 0 {
                                                                                                            continue;
                                                                                                        }
                                                                                                    } else {
                                                                                                        tmp___29 = strcmp(
                                                                                                            key.as_mut_ptr() as *const libc::c_char,
                                                                                                            b"num_connections\0" as *const u8 as *const libc::c_char,
                                                                                                        );
                                                                                                        if tmp___29 == 0 as libc::c_int {
                                                                                                            tmp___26 = atoi(value as *const libc::c_char);
                                                                                                            num = tmp___26;
                                                                                                            if num <= 65535 as libc::c_int {
                                                                                                                (*conf).num_connections = num as uint16_t;
                                                                                                                continue;
                                                                                                            } else {
                                                                                                                tmp___27 = dcgettext(
                                                                                                                    0 as *mut libc::c_void as *const libc::c_char,
                                                                                                                    b"Requested too many connections, max is %i\n\0"
                                                                                                                        as *const u8 as *const libc::c_char,
                                                                                                                    5 as libc::c_int,
                                                                                                                );
                                                                                                                fprintf(
                                                                                                                    stderr,
                                                                                                                    tmp___27 as *const libc::c_char,
                                                                                                                    65535 as libc::c_int,
                                                                                                                );
                                                                                                            }
                                                                                                        } else {
                                                                                                            tmp___28 = strcmp(
                                                                                                                key.as_mut_ptr() as *const libc::c_char,
                                                                                                                b"user_agent\0" as *const u8 as *const libc::c_char,
                                                                                                            );
                                                                                                            if tmp___28 == 0 {
                                                                                                                conf_hdr_make(
                                                                                                                    ((*conf).add_header[0 as libc::c_int as usize])
                                                                                                                        .as_mut_ptr(),
                                                                                                                    b"User-Agent\0" as *const u8 as *const libc::c_char,
                                                                                                                    b"Axel/2.17.11+gab2f84 (linux-gnu)\0" as *const u8
                                                                                                                        as *const libc::c_char,
                                                                                                                );
                                                                                                                continue;
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                dst = &mut (*conf).max_speed as *mut libc::c_ulonglong
                                                                                                    as *mut libc::c_void;
                                                                                                *(dst
                                                                                                    as *mut libc::c_ulonglong) = strtoull(
                                                                                                    value as *const libc::c_char,
                                                                                                    0 as *mut libc::c_void as *mut *mut libc::c_char,
                                                                                                    10 as libc::c_int,
                                                                                                );
                                                                                                continue;
                                                                                            }
                                                                                            current_block_103 = 11329907657253921262;
                                                                                        } else {
                                                                                            dst = &mut (*conf).search_top as *mut libc::c_int
                                                                                                as *mut libc::c_void;
                                                                                            current_block_103 = 12963528325254160332;
                                                                                        }
                                                                                    } else {
                                                                                        dst = &mut (*conf).search_amount as *mut libc::c_int
                                                                                            as *mut libc::c_void;
                                                                                        current_block_103 = 12963528325254160332;
                                                                                    }
                                                                                } else {
                                                                                    dst = &mut (*conf).search_threads as *mut libc::c_int
                                                                                        as *mut libc::c_void;
                                                                                    current_block_103 = 12963528325254160332;
                                                                                }
                                                                            } else {
                                                                                dst = &mut (*conf).search_timeout as *mut libc::c_int
                                                                                    as *mut libc::c_void;
                                                                                current_block_103 = 12963528325254160332;
                                                                            }
                                                                        } else {
                                                                            dst = &mut (*conf).no_clobber as *mut libc::c_int
                                                                                as *mut libc::c_void;
                                                                            current_block_103 = 12963528325254160332;
                                                                        }
                                                                    } else {
                                                                        dst = &mut (*conf).insecure as *mut libc::c_int
                                                                            as *mut libc::c_void;
                                                                        current_block_103 = 12963528325254160332;
                                                                    }
                                                                } else {
                                                                    dst = &mut (*conf).percentage as *mut libc::c_int
                                                                        as *mut libc::c_void;
                                                                    current_block_103 = 12963528325254160332;
                                                                }
                                                            } else {
                                                                dst = &mut (*conf).alternate_output as *mut libc::c_int
                                                                    as *mut libc::c_void;
                                                                current_block_103 = 12963528325254160332;
                                                            }
                                                        } else {
                                                            dst = &mut (*conf).verbose as *mut libc::c_int
                                                                as *mut libc::c_void;
                                                            current_block_103 = 12963528325254160332;
                                                        }
                                                    } else {
                                                        dst = &mut (*conf).max_speed as *mut libc::c_ulonglong
                                                            as *mut libc::c_void;
                                                        current_block_103 = 12963528325254160332;
                                                    }
                                                } else {
                                                    dst = &mut (*conf).buffer_size as *mut libc::c_int
                                                        as *mut libc::c_void;
                                                    current_block_103 = 12963528325254160332;
                                                }
                                            } else {
                                                dst = &mut (*conf).max_redirect as *mut libc::c_int
                                                    as *mut libc::c_void;
                                                current_block_103 = 12963528325254160332;
                                            }
                                        } else {
                                            dst = &mut (*conf).reconnect_delay as *mut libc::c_int
                                                as *mut libc::c_void;
                                            current_block_103 = 12963528325254160332;
                                        }
                                    } else {
                                        dst = &mut (*conf).connection_timeout as *mut libc::c_int
                                            as *mut libc::c_void;
                                        current_block_103 = 12963528325254160332;
                                    }
                                } else {
                                    dst = &mut (*conf).save_state_interval as *mut libc::c_int
                                        as *mut libc::c_void;
                                    current_block_103 = 12963528325254160332;
                                }
                            } else {
                                dst = &mut (*conf).strip_cgi_parameters as *mut libc::c_int
                                    as *mut libc::c_void;
                                current_block_103 = 12963528325254160332;
                            }
                            match current_block_103 {
                                11329907657253921262 => {}
                                _ => {
                                    *(dst
                                        as *mut libc::c_int) = atoi(value as *const libc::c_char);
                                    continue;
                                }
                            }
                        } else {
                            dst = &mut (*conf).no_proxy as *mut [libc::c_char; 1024]
                                as *mut libc::c_void;
                            current_block_103 = 13303144130133872306;
                        }
                    } else {
                        dst = &mut (*conf).http_proxy as *mut [libc::c_char; 1024]
                            as *mut libc::c_void;
                        current_block_103 = 13303144130133872306;
                    }
                } else {
                    dst = &mut (*conf).default_filename as *mut [libc::c_char; 1024]
                        as *mut libc::c_void;
                    current_block_103 = 13303144130133872306;
                }
                match current_block_103 {
                    11329907657253921262 => {}
                    _ => {
                        strlcpy(
                            dst as *mut libc::c_char,
                            value as *const libc::c_char,
                            1024 as libc::c_int as size_t,
                        );
                        continue;
                    }
                }
            }
            _ => {}
        }
        tmp___33 = dcgettext(
            0 as *mut libc::c_void as *const libc::c_char,
            b"Error in %s line %i.\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        fprintf(stderr, tmp___33 as *const libc::c_char, file, line);
        ret = 0 as libc::c_int;
        break;
    }
    fclose(fp);
    return ret;
}
pub unsafe extern "C" fn conf_init(mut conf: *mut conf_t) -> libc::c_int {
    let mut s2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    let mut s: [libc::c_char; 1024] = [0; 1024];
    let mut ret: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: libc::c_int = 0;
    memset(
        conf as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<conf_t>() as libc::c_ulong,
    );
    strlcpy(
        ((*conf).default_filename).as_mut_ptr(),
        b"default\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    (*conf).http_proxy[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    (*conf).no_proxy[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    (*conf).strip_cgi_parameters = 1 as libc::c_int;
    (*conf).save_state_interval = 10 as libc::c_int;
    (*conf).connection_timeout = 45 as libc::c_int;
    (*conf).reconnect_delay = 20 as libc::c_int;
    (*conf).num_connections = 4 as libc::c_int as uint16_t;
    (*conf).max_redirect = 20 as libc::c_int;
    (*conf).io_timeout = 120 as libc::c_uint;
    (*conf).buffer_size = 5120 as libc::c_int;
    (*conf).max_speed = 0 as libc::c_ulonglong;
    (*conf).verbose = 1 as libc::c_int;
    (*conf).insecure = 0 as libc::c_int;
    (*conf).no_clobber = 0 as libc::c_int;
    (*conf).search_timeout = 10 as libc::c_int;
    (*conf).search_threads = 3 as libc::c_int;
    (*conf).search_amount = 15 as libc::c_int;
    (*conf).search_top = 3 as libc::c_int;
    (*conf).ai_family = 0 as libc::c_int as sa_family_t;
    conf_hdr_make(
        ((*conf).add_header[0 as libc::c_int as usize]).as_mut_ptr(),
        b"User-Agent\0" as *const u8 as *const libc::c_char,
        b"Axel/2.17.11+gab2f84 (linux-gnu)\0" as *const u8 as *const libc::c_char,
    );
    (*conf).add_header_count = 1 as libc::c_int;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<if_t>() as libc::c_ulong,
    );
    (*conf).interfaces = tmp as *mut if_t;
    if ((*conf).interfaces).is_null() {
        return 0 as libc::c_int;
    }
    (*(*conf).interfaces).next = (*conf).interfaces as *mut libc::c_void;
    (*conf).alternate_output = isatty(1 as libc::c_int);
    s2 = getenv(b"http_proxy\0" as *const u8 as *const libc::c_char);
    if !s2.is_null() {
        strlcpy(
            ((*conf).http_proxy).as_mut_ptr(),
            s2 as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
    } else {
        s2 = getenv(b"HTTP_PROXY\0" as *const u8 as *const libc::c_char);
        if !s2.is_null() {
            strlcpy(
                ((*conf).http_proxy).as_mut_ptr(),
                s2 as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            );
        }
    }
    tmp___0 = conf_loadfile(conf, b"/etc/axelrc\0" as *const u8 as *const libc::c_char);
    if tmp___0 == 0 {
        return 0 as libc::c_int;
    }
    s2 = getenv(b"HOME\0" as *const u8 as *const libc::c_char);
    if s2 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        ret = snprintf(
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            b"%s/.axelrc\0" as *const u8 as *const libc::c_char,
            s2,
        );
        if ret
            >= ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                as libc::c_int
        {
            tmp___1 = dcgettext(
                0 as *mut libc::c_void as *const libc::c_char,
                b"HOME env variable too long\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            fprintf(stderr, tmp___1 as *const libc::c_char);
            return 0 as libc::c_int;
        }
        tmp___2 = conf_loadfile(conf, s.as_mut_ptr() as *const libc::c_char);
        if tmp___2 == 0 {
            return 0 as libc::c_int;
        }
    }
    i = 0 as libc::c_int;
    while (*conf).no_proxy[i as usize] != 0 {
        if (*conf).no_proxy[i as usize] as libc::c_int == 44 as libc::c_int {
            (*conf).no_proxy[i as usize] = 0 as libc::c_int as libc::c_char;
        }
        i += 1;
    }
    (*conf).no_proxy[(i + 1 as libc::c_int) as usize] = 0 as libc::c_int as libc::c_char;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn conf_free(mut conf: *mut conf_t) {
    free((*conf).interfaces as *mut libc::c_void);
}
pub unsafe extern "C" fn parse_interfaces(
    mut conf: *mut conf_t,
    mut s: *mut libc::c_char,
) -> libc::c_int {
    let mut s2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut iface: *mut if_t = 0 as *mut if_t;
    let mut i: *mut if_t = 0 as *mut if_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    iface = (*(*conf).interfaces).next as *mut if_t;
    while iface as libc::c_ulong != (*conf).interfaces as libc::c_ulong {
        i = (*iface).next as *mut if_t;
        free(iface as *mut libc::c_void);
        iface = i;
    }
    free((*conf).interfaces as *mut libc::c_void);
    if *s == 0 {
        tmp = calloc(
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<if_t>() as libc::c_ulong,
        );
        (*conf).interfaces = tmp as *mut if_t;
        if ((*conf).interfaces).is_null() {
            return 0 as libc::c_int;
        }
        (*(*conf).interfaces).next = (*conf).interfaces as *mut libc::c_void;
        return 1 as libc::c_int;
    }
    tmp___0 = strlen(s as *const libc::c_char);
    *s
        .offset(
            tmp___0.wrapping_add(1 as libc::c_ulong) as isize,
        ) = 0 as libc::c_int as libc::c_char;
    tmp___1 = malloc(::std::mem::size_of::<if_t>() as libc::c_ulong);
    iface = tmp___1 as *mut if_t;
    (*conf).interfaces = iface;
    if ((*conf).interfaces).is_null() {
        return 0 as libc::c_int;
    }
    loop {
        loop {
            if !(*s as libc::c_int == 32 as libc::c_int) {
                if !(*s as libc::c_int == 9 as libc::c_int) {
                    break;
                }
            }
            if *s == 0 {
                break;
            }
            s = s.offset(1);
        }
        s2 = s;
        while *s2 as libc::c_int != 32 as libc::c_int {
            if !(*s2 as libc::c_int != 9 as libc::c_int) {
                break;
            }
            if *s2 == 0 {
                break;
            }
            s2 = s2.offset(1);
        }
        *s2 = 0 as libc::c_int as libc::c_char;
        if (*s as libc::c_int) < 48 as libc::c_int {
            get_if_ip(
                ((*iface).text).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                s as *const libc::c_char,
            );
        } else if *s as libc::c_int > 57 as libc::c_int {
            get_if_ip(
                ((*iface).text).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                s as *const libc::c_char,
            );
        } else {
            strlcpy(
                ((*iface).text).as_mut_ptr(),
                s as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            );
        }
        s = s2.offset(1 as libc::c_int as isize);
        if *s != 0 {
            (*iface).next = malloc(::std::mem::size_of::<if_t>() as libc::c_ulong);
            if ((*iface).next).is_null() {
                return 0 as libc::c_int;
            }
            iface = (*iface).next as *mut if_t;
        } else {
            (*iface).next = (*conf).interfaces as *mut libc::c_void;
            break;
        }
    }
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn is_proto_http(mut proto: libc::c_int) -> libc::c_int {
    return (proto & (1 as libc::c_int) << 1 as libc::c_int
        == (1 as libc::c_int) << 1 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn conn_set(
    mut conn: *mut conn_t,
    mut set_url: *const libc::c_char,
) -> libc::c_int {
    let mut url: [libc::c_char; 1024] = [0; 1024];
    let mut i: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut j: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut proto_len: libc::c_int = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: libc::c_char = 0;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___7: *mut libc::c_char = 0 as *mut libc::c_char;
    i = strstr(set_url, b"://\0" as *const u8 as *const libc::c_char);
    if i as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        (*conn).proto = (1 as libc::c_int) << 1 as libc::c_int;
        (*conn).port = 80 as libc::c_int;
        strlcpy(
            url.as_mut_ptr(),
            set_url,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
    } else {
        proto_len = i.offset_from(set_url as *mut libc::c_char) as libc::c_long
            as libc::c_int;
        tmp___3 = strncmp(
            set_url,
            b"ftp\0" as *const u8 as *const libc::c_char,
            proto_len as size_t,
        );
        if tmp___3 == 0 as libc::c_int {
            (*conn).proto = 0 as libc::c_int;
            (*conn).port = 21 as libc::c_int;
        } else {
            tmp___2 = strncmp(
                set_url,
                b"http\0" as *const u8 as *const libc::c_char,
                proto_len as size_t,
            );
            if tmp___2 == 0 as libc::c_int {
                (*conn).proto = (1 as libc::c_int) << 1 as libc::c_int;
                (*conn).port = 80 as libc::c_int;
            } else {
                tmp___1 = strncmp(
                    set_url,
                    b"ftps\0" as *const u8 as *const libc::c_char,
                    proto_len as size_t,
                );
                if tmp___1 == 0 as libc::c_int {
                    (*conn).proto = 1 as libc::c_int;
                    (*conn).port = 990 as libc::c_int;
                } else {
                    tmp___0 = strncmp(
                        set_url,
                        b"https\0" as *const u8 as *const libc::c_char,
                        proto_len as size_t,
                    );
                    if tmp___0 == 0 as libc::c_int {
                        (*conn)
                            .proto = (1 as libc::c_int) << 1 as libc::c_int
                            | 1 as libc::c_int;
                        (*conn).port = 443 as libc::c_int;
                    } else {
                        tmp = dcgettext(
                            0 as *mut libc::c_void as *const libc::c_char,
                            b"Unsupported protocol\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        );
                        fprintf(stderr, tmp as *const libc::c_char);
                        return 0 as libc::c_int;
                    }
                }
            }
        }
        if (*conn).proto & 1 as libc::c_int == 1 as libc::c_int {
            tmp___4 = dcgettext(
                0 as *mut libc::c_void as *const libc::c_char,
                b"Secure protocol is not supported\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            );
            fprintf(stderr, tmp___4 as *const libc::c_char);
            return 0 as libc::c_int;
        }
        strlcpy(
            url.as_mut_ptr(),
            i.offset(3 as libc::c_int as isize) as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
    }
    i = strchr(url.as_mut_ptr() as *const libc::c_char, '/' as i32);
    if i as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        strcpy(((*conn).dir).as_mut_ptr(), b"/\0" as *const u8 as *const libc::c_char);
    } else {
        *i = 0 as libc::c_int as libc::c_char;
        snprintf(
            ((*conn).dir).as_mut_ptr(),
            1024 as libc::c_int as size_t,
            b"/%s\0" as *const u8 as *const libc::c_char,
            i.offset(1 as libc::c_int as isize),
        );
        if (*conn).proto == (1 as libc::c_int) << 1 as libc::c_int {
            http_encode(
                ((*conn).dir).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            );
        } else if (*conn).proto
                == (1 as libc::c_int) << 1 as libc::c_int | 1 as libc::c_int
            {
            http_encode(
                ((*conn).dir).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            );
        }
    }
    j = strchr(((*conn).dir).as_mut_ptr() as *const libc::c_char, '?' as i32);
    if j as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        *j = 0 as libc::c_int as libc::c_char;
    }
    i = strrchr(((*conn).dir).as_mut_ptr() as *const libc::c_char, '/' as i32);
    if i as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        *i = 0 as libc::c_int as libc::c_char;
    }
    if j as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        *j = '?' as i32 as libc::c_char;
    }
    if i as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        strlcpy(
            ((*conn).file).as_mut_ptr(),
            ((*conn).dir).as_mut_ptr() as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
        strcpy(((*conn).dir).as_mut_ptr(), b"/\0" as *const u8 as *const libc::c_char);
    } else {
        strlcpy(
            ((*conn).file).as_mut_ptr(),
            i.offset(1 as libc::c_int as isize) as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
        strlcat(
            ((*conn).dir).as_mut_ptr(),
            b"/\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
    }
    tmp___6 = strrchr(url.as_mut_ptr() as *const libc::c_char, '@' as i32);
    if tmp___6 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        strlcpy(
            ((*conn).user).as_mut_ptr(),
            url.as_mut_ptr() as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
        i = strrchr(((*conn).user).as_mut_ptr() as *const libc::c_char, '@' as i32);
        *i = 0 as libc::c_int as libc::c_char;
        strlcpy(
            url.as_mut_ptr(),
            i.offset(1 as libc::c_int as isize) as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
        (*conn).pass[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    } else if (*conn).proto & (1 as libc::c_int) << 1 as libc::c_int == 0 as libc::c_int
        {
        strcpy(
            ((*conn).user).as_mut_ptr(),
            b"anonymous\0" as *const u8 as *const libc::c_char,
        );
        strcpy(
            ((*conn).pass).as_mut_ptr(),
            b"mailto:axel@axel.project\0" as *const u8 as *const libc::c_char,
        );
    } else {
        tmp___5 = 0 as libc::c_int as libc::c_char;
        (*conn).pass[0 as libc::c_int as usize] = tmp___5;
        (*conn).user[0 as libc::c_int as usize] = tmp___5;
    }
    i = strchr(((*conn).user).as_mut_ptr() as *const libc::c_char, ':' as i32);
    if i as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        *i = 0 as libc::c_int as libc::c_char;
        strlcpy(
            ((*conn).pass).as_mut_ptr(),
            i.offset(1 as libc::c_int as isize) as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
    }
    if url[0 as libc::c_int as usize] as libc::c_int == 91 as libc::c_int {
        strlcpy(
            ((*conn).host).as_mut_ptr(),
            url.as_mut_ptr().offset(1 as libc::c_int as isize) as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
        i = strrchr(((*conn).host).as_mut_ptr() as *const libc::c_char, ']' as i32);
        if i as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            tmp___7 = i;
            i = i.offset(1);
            *tmp___7 = 0 as libc::c_int as libc::c_char;
        } else {
            return 0 as libc::c_int
        }
    } else {
        strlcpy(
            ((*conn).host).as_mut_ptr(),
            url.as_mut_ptr() as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
        i = ((*conn).host).as_mut_ptr();
        while *i != 0 {
            if !(*i as libc::c_int != 58 as libc::c_int) {
                break;
            }
            i = i.offset(1);
        }
    }
    if *i as libc::c_int == 58 as libc::c_int {
        *i = 0 as libc::c_int as libc::c_char;
        sscanf(
            i.offset(1 as libc::c_int as isize) as *const libc::c_char,
            b"%i\0" as *const u8 as *const libc::c_char,
            &mut (*conn).port as *mut libc::c_int,
        );
        i = ((*conn).host).as_mut_ptr();
    }
    return ((*conn).port > 0 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn scheme_from_proto(
    mut proto: libc::c_int,
) -> *const libc::c_char {
    match proto {
        0 => return b"ftp://\0" as *const u8 as *const libc::c_char,
        1 => return b"ftps://\0" as *const u8 as *const libc::c_char,
        3 => return b"https://\0" as *const u8 as *const libc::c_char,
        _ => return b"http://\0" as *const u8 as *const libc::c_char,
    };
}
pub unsafe extern "C" fn conn_url(
    mut dst: *mut libc::c_char,
    mut len: size_t,
    mut conn: *mut conn_t,
) -> *mut libc::c_char {
    let mut prefix: *const libc::c_char = 0 as *const libc::c_char;
    let mut postfix: *const libc::c_char = 0 as *const libc::c_char;
    let mut scheme: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut scheme_len: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut plen: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut plen___0: libc::c_int = 0;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    prefix = b"\0" as *const u8 as *const libc::c_char;
    postfix = b"\0" as *const u8 as *const libc::c_char;
    tmp = scheme_from_proto((*conn).proto);
    scheme = tmp;
    tmp___0 = strlcpy(dst, scheme, len);
    scheme_len = tmp___0;
    if scheme_len > len {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    len = (len as libc::c_ulong).wrapping_sub(scheme_len) as size_t as size_t;
    p = dst.offset(scheme_len as isize);
    if (*conn).user[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int {
        tmp___2 = strcmp(
            ((*conn).user).as_mut_ptr() as *const libc::c_char,
            b"anonymous\0" as *const u8 as *const libc::c_char,
        );
        if tmp___2 != 0 as libc::c_int {
            tmp___1 = snprintf(
                p,
                len,
                b"%s:%s@\0" as *const u8 as *const libc::c_char,
                ((*conn).user).as_mut_ptr(),
                ((*conn).pass).as_mut_ptr(),
            );
            plen = tmp___1;
            if plen < 0 as libc::c_int {
                return 0 as *mut libc::c_void as *mut libc::c_char;
            }
            len = (len as libc::c_ulong).wrapping_sub(plen as size_t) as size_t
                as size_t;
            p = p.offset(plen as isize);
        }
    }
    tmp___3 = is_ipv6_addr(((*conn).host).as_mut_ptr() as *const libc::c_char);
    if tmp___3 != 0 {
        prefix = b"[\0" as *const u8 as *const libc::c_char;
        postfix = b"]\0" as *const u8 as *const libc::c_char;
    }
    plen___0 = snprintf(
        p,
        len,
        b"%s%s%s:%i%s%s\0" as *const u8 as *const libc::c_char,
        prefix,
        ((*conn).host).as_mut_ptr(),
        postfix,
        (*conn).port,
        ((*conn).dir).as_mut_ptr(),
        ((*conn).file).as_mut_ptr(),
    );
    if plen___0 < 0 as libc::c_int {
        tmp___4 = 0 as *mut libc::c_void as *mut libc::c_char;
    } else {
        tmp___4 = dst;
    }
    return tmp___4;
}
pub unsafe extern "C" fn conn_disconnect(mut conn: *mut conn_t) {
    if (*conn).proto & (1 as libc::c_int) << 1 as libc::c_int == 0 as libc::c_int {
        if (*conn).proxy == 0 {
            ftp_disconnect(((*conn).ftp).as_mut_ptr());
        } else {
            http_disconnect(((*conn).http).as_mut_ptr());
        }
    } else {
        http_disconnect(((*conn).http).as_mut_ptr());
    }
    (*conn).tcp = 0 as *mut libc::c_void as *mut tcp_t;
    (*conn).enabled = 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn conn_init(mut conn: *mut conn_t) -> libc::c_int {
    let mut proxy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut host: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    proxy = ((*(*conn).conf).http_proxy).as_mut_ptr();
    host = ((*(*conn).conf).no_proxy).as_mut_ptr();
    if (*(*conn).conf).http_proxy[0 as libc::c_int as usize] as libc::c_int
        == 0 as libc::c_int
    {
        proxy = 0 as *mut libc::c_void as *mut libc::c_char;
    } else if (*(*conn).conf).no_proxy[0 as libc::c_int as usize] as libc::c_int
            != 0 as libc::c_int
        {
        i = 0 as libc::c_int;
        loop {
            if (*(*conn).conf).no_proxy[i as usize] as libc::c_int == 0 as libc::c_int {
                tmp = strstr(
                    ((*conn).host).as_mut_ptr() as *const libc::c_char,
                    host as *const libc::c_char,
                );
                if tmp as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                    proxy = 0 as *mut libc::c_void as *mut libc::c_char;
                }
                host = &mut *((*(*conn).conf).no_proxy)
                    .as_mut_ptr()
                    .offset((i + 1 as libc::c_int) as isize) as *mut libc::c_char;
                if (*(*conn).conf).no_proxy[(i + 1 as libc::c_int) as usize]
                    as libc::c_int == 0 as libc::c_int
                {
                    break;
                }
            }
            i += 1;
        }
    }
    (*conn)
        .proxy = (proxy as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong)
        as libc::c_int;
    let mut current_block_41: u64;
    if (*conn).proto & (1 as libc::c_int) << 1 as libc::c_int == 0 as libc::c_int {
        if (*conn).proxy == 0 {
            (*conn).ftp[0 as libc::c_int as usize].local_if = (*conn).local_if;
            (*conn).ftp[0 as libc::c_int as usize].ftp_mode = 1 as libc::c_int;
            (*conn)
                .ftp[0 as libc::c_int as usize]
                .tcp
                .ai_family = (*(*conn).conf).ai_family;
            tmp___0 = ftp_connect(
                ((*conn).ftp).as_mut_ptr(),
                (*conn).proto,
                ((*conn).host).as_mut_ptr(),
                (*conn).port,
                ((*conn).user).as_mut_ptr(),
                ((*conn).pass).as_mut_ptr(),
                (*(*conn).conf).io_timeout,
            );
            if tmp___0 == 0 {
                (*conn).message = (*conn).ftp[0 as libc::c_int as usize].message;
                conn_disconnect(conn);
                return 0 as libc::c_int;
            }
            (*conn).message = (*conn).ftp[0 as libc::c_int as usize].message;
            tmp___1 = ftp_cwd(((*conn).ftp).as_mut_ptr(), ((*conn).dir).as_mut_ptr());
            if tmp___1 == 0 {
                conn_disconnect(conn);
                return 0 as libc::c_int;
            }
            current_block_41 = 7746103178988627676;
        } else {
            current_block_41 = 10660898149709080077;
        }
    } else {
        current_block_41 = 10660898149709080077;
    }
    match current_block_41 {
        10660898149709080077 => {
            (*conn).http[0 as libc::c_int as usize].local_if = (*conn).local_if;
            (*conn)
                .http[0 as libc::c_int as usize]
                .tcp
                .ai_family = (*(*conn).conf).ai_family;
            tmp___2 = http_connect(
                ((*conn).http).as_mut_ptr(),
                (*conn).proto,
                proxy,
                ((*conn).host).as_mut_ptr(),
                (*conn).port,
                ((*conn).user).as_mut_ptr(),
                ((*conn).pass).as_mut_ptr(),
                (*(*conn).conf).io_timeout,
            );
            if tmp___2 == 0 {
                (*conn)
                    .message = (*conn)
                    .http[0 as libc::c_int as usize]
                    .headers[0 as libc::c_int as usize]
                    .p;
                conn_disconnect(conn);
                return 0 as libc::c_int;
            }
            (*conn)
                .message = (*conn)
                .http[0 as libc::c_int as usize]
                .headers[0 as libc::c_int as usize]
                .p;
            (*conn)
                .tcp = &mut (*((*conn).http)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
                .tcp;
        }
        _ => {}
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn conn_setup(mut conn: *mut conn_t) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut s: [libc::c_char; 2048] = [0; 2048];
    let mut i: libc::c_int = 0;
    if (*conn).ftp[0 as libc::c_int as usize].tcp.fd <= 0 as libc::c_int {
        if (*conn).http[0 as libc::c_int as usize].tcp.fd <= 0 as libc::c_int {
            tmp = conn_init(conn);
            if tmp == 0 {
                return 0 as libc::c_int;
            }
        }
    }
    let mut current_block_36: u64;
    if (*conn).proto & (1 as libc::c_int) << 1 as libc::c_int == 0 as libc::c_int {
        if (*conn).proxy == 0 {
            tmp___0 = ftp_data(((*conn).ftp).as_mut_ptr(), (*(*conn).conf).io_timeout);
            if tmp___0 == 0 {
                return 0 as libc::c_int;
            }
            (*conn)
                .tcp = &mut (*((*conn).ftp)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
                .data_tcp;
            if (*conn).currentbyte != 0 {
                ftp_command(
                    ((*conn).ftp).as_mut_ptr(),
                    b"REST %jd\0" as *const u8 as *const libc::c_char,
                    (*conn).currentbyte,
                );
                tmp___1 = ftp_wait(((*conn).ftp).as_mut_ptr());
                if tmp___1 / 100 as libc::c_int != 3 as libc::c_int {
                    if (*conn).ftp[0 as libc::c_int as usize].status / 100 as libc::c_int
                        != 2 as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                }
            }
            current_block_36 = 17500079516916021833;
        } else {
            current_block_36 = 6643260861007203544;
        }
    } else {
        current_block_36 = 6643260861007203544;
    }
    match current_block_36 {
        6643260861007203544 => {
            snprintf(
                s.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong,
                b"%s%s\0" as *const u8 as *const libc::c_char,
                ((*conn).dir).as_mut_ptr(),
                ((*conn).file).as_mut_ptr(),
            );
            if (*conn).supported {
                (*conn).http[0 as libc::c_int as usize].firstbyte = (*conn).currentbyte;
            } else {
                (*conn)
                    .http[0 as libc::c_int as usize]
                    .firstbyte = -(1 as libc::c_int) as off_t;
            }
            (*conn).http[0 as libc::c_int as usize].lastbyte = (*conn).lastbyte;
            abuf_setup(
                ((*conn).http[0 as libc::c_int as usize].request).as_mut_ptr(),
                2048 as libc::c_int as size_t,
            );
            http_get(((*conn).http).as_mut_ptr(), s.as_mut_ptr());
            i = 0 as libc::c_int;
            while i < (*(*conn).conf).add_header_count {
                http_addheader(
                    ((*conn).http).as_mut_ptr(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    ((*(*conn).conf).add_header[i as usize]).as_mut_ptr(),
                );
                i += 1;
            }
        }
        _ => {}
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn conn_exec(mut conn: *mut conn_t) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    if (*conn).proto & (1 as libc::c_int) << 1 as libc::c_int == 0 as libc::c_int {
        if (*conn).proxy == 0 {
            tmp = ftp_command(
                ((*conn).ftp).as_mut_ptr(),
                b"RETR %s\0" as *const u8 as *const libc::c_char,
                ((*conn).file).as_mut_ptr(),
            );
            if tmp == 0 {
                return 0 as libc::c_int;
            }
            tmp___0 = ftp_wait(((*conn).ftp).as_mut_ptr());
            return (tmp___0 / 100 as libc::c_int == 1 as libc::c_int) as libc::c_int;
        }
    }
    abuf_setup(
        ((*conn).http[0 as libc::c_int as usize].headers).as_mut_ptr(),
        1024 as libc::c_int as size_t,
    );
    tmp___1 = http_exec(((*conn).http).as_mut_ptr());
    if tmp___1 == 0 {
        return 0 as libc::c_int;
    }
    return ((*conn).http[0 as libc::c_int as usize].status / 100 as libc::c_int
        == 2 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn conn_info_ftp(mut conn: *mut conn_t) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    ftp_command(
        ((*conn).ftp).as_mut_ptr(),
        b"REST %d\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    tmp = ftp_wait(((*conn).ftp).as_mut_ptr());
    if tmp / 100 as libc::c_int == 3 as libc::c_int {
        (*conn).supported = 1 as libc::c_int != 0;
        ftp_command(
            ((*conn).ftp).as_mut_ptr(),
            b"REST %d\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        ftp_wait(((*conn).ftp).as_mut_ptr());
    } else if (*conn).ftp[0 as libc::c_int as usize].status / 100 as libc::c_int
            == 2 as libc::c_int
        {
        (*conn).supported = 1 as libc::c_int != 0;
        ftp_command(
            ((*conn).ftp).as_mut_ptr(),
            b"REST %d\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        ftp_wait(((*conn).ftp).as_mut_ptr());
    } else {
        (*conn).supported = 0 as libc::c_int != 0;
    }
    tmp___0 = ftp_cwd(((*conn).ftp).as_mut_ptr(), ((*conn).dir).as_mut_ptr());
    if tmp___0 == 0 {
        return 0 as libc::c_int;
    }
    (*conn)
        .size = ftp_size(
        ((*conn).ftp).as_mut_ptr(),
        ((*conn).file).as_mut_ptr(),
        (*(*conn).conf).max_redirect,
        (*(*conn).conf).io_timeout,
    );
    if (*conn).size < 0 as libc::c_long {
        (*conn).supported = 0 as libc::c_int != 0;
    }
    if (*conn).size == -(1 as libc::c_long) {
        return 0 as libc::c_int
    } else {
        if (*conn).size == -(2 as libc::c_long) {
            (*conn).size = 9223372036854775807 as libc::c_longlong as off_t;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn conn_info(mut conn: *mut conn_t) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut s: [libc::c_char; 1005] = [0; 1005];
    let mut i: libc::c_longlong = 0;
    let mut t: *const libc::c_char = 0 as *const libc::c_char;
    let mut setup_ret: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: libc::c_int = 0;
    let mut __a: off_t = 0;
    let mut __b: off_t = 0;
    let mut tmp___6: off_t = 0;
    let mut tmp___7: off_t = 0;
    if (*conn).proto & (1 as libc::c_int) << 1 as libc::c_int == 0 as libc::c_int {
        if (*conn).proxy == 0 {
            tmp = conn_info_ftp(conn);
            return tmp;
        }
    }
    i = 0 as libc::c_longlong;
    loop {
        (*conn).supported = 1 as libc::c_int != 0;
        (*conn).currentbyte = 0 as libc::c_int as off_t;
        pthread_mutex_lock(&mut (*conn).lock);
        tmp___0 = conn_setup(conn);
        setup_ret = tmp___0;
        pthread_mutex_unlock(&mut (*conn).lock);
        if setup_ret == 0 {
            return 0 as libc::c_int;
        }
        conn_exec(conn);
        conn_disconnect(conn);
        http_filename(
            ((*conn).http).as_mut_ptr() as *const http_t,
            ((*conn).output_filename).as_mut_ptr(),
        );
        if (*conn).http[0 as libc::c_int as usize].status / 100 as libc::c_int
            != 3 as libc::c_int
        {
            break;
        }
        t = http_header(
            ((*conn).http).as_mut_ptr() as *const http_t,
            b"location:\0" as *const u8 as *const libc::c_char,
        );
        if t as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            return 0 as libc::c_int;
        }
        sscanf(t, b"%1000s\0" as *const u8 as *const libc::c_char, s.as_mut_ptr());
        if s[0 as libc::c_int as usize] as libc::c_int == 47 as libc::c_int {
            tmp___1 = scheme_from_proto((*conn).proto);
            abuf_printf(
                ((*conn).http[0 as libc::c_int as usize].headers).as_mut_ptr(),
                b"%s%s:%i%s\0" as *const u8 as *const libc::c_char,
                tmp___1,
                ((*conn).host).as_mut_ptr(),
                (*conn).port,
                s.as_mut_ptr(),
            );
            strlcpy(
                s.as_mut_ptr(),
                (*conn)
                    .http[0 as libc::c_int as usize]
                    .headers[0 as libc::c_int as usize]
                    .p as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 1005]>() as libc::c_ulong,
            );
        } else {
            tmp___2 = strstr(
                s.as_mut_ptr() as *const libc::c_char,
                b"://\0" as *const u8 as *const libc::c_char,
            );
            if tmp___2 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                conn_url(
                    (*conn)
                        .http[0 as libc::c_int as usize]
                        .headers[0 as libc::c_int as usize]
                        .p,
                    (*conn)
                        .http[0 as libc::c_int as usize]
                        .headers[0 as libc::c_int as usize]
                        .len,
                    conn,
                );
                strlcat(
                    (*conn)
                        .http[0 as libc::c_int as usize]
                        .headers[0 as libc::c_int as usize]
                        .p,
                    s.as_mut_ptr() as *const libc::c_char,
                    (*conn)
                        .http[0 as libc::c_int as usize]
                        .headers[0 as libc::c_int as usize]
                        .len,
                );
                strlcpy(
                    s.as_mut_ptr(),
                    (*conn)
                        .http[0 as libc::c_int as usize]
                        .headers[0 as libc::c_int as usize]
                        .p as *const libc::c_char,
                    ::std::mem::size_of::<[libc::c_char; 1005]>() as libc::c_ulong,
                );
            }
        }
        tmp___3 = conn_set(conn, s.as_mut_ptr() as *const libc::c_char);
        if tmp___3 == 0 {
            return 0 as libc::c_int;
        }
        if (*conn).proto & (1 as libc::c_int) << 1 as libc::c_int == 0 as libc::c_int {
            if (*conn).proxy == 0 {
                return -(1 as libc::c_int);
            }
        }
        i += 1;
        if i >= (*(*conn).conf).max_redirect as libc::c_longlong {
            tmp___4 = dcgettext(
                0 as *mut libc::c_void as *const libc::c_char,
                b"Too many redirects.\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            fprintf(stderr, tmp___4 as *const libc::c_char);
            return 0 as libc::c_int;
        }
        if !((*conn).http[0 as libc::c_int as usize].status / 100 as libc::c_int
            == 3 as libc::c_int)
        {
            break;
        }
    }
    if (*conn).http[0 as libc::c_int as usize].status != 416 as libc::c_int {
        if (*conn).http[0 as libc::c_int as usize].status / 100 as libc::c_int
            != 2 as libc::c_int
        {
            return 0 as libc::c_int;
        }
    }
    (*conn).size = http_size_from_range(((*conn).http).as_mut_ptr());
    if (*conn).http[0 as libc::c_int as usize].status == 206 as libc::c_int {
        tmp___5 = 1 as libc::c_int;
    } else if (*conn).size > 0 as libc::c_long {
        tmp___5 = 1 as libc::c_int;
    } else {
        tmp___5 = 0 as libc::c_int;
    }
    (*conn).supported = tmp___5 != 0;
    if (*conn).size <= 0 as libc::c_long {
        match (*conn).http[0 as libc::c_int as usize].status {
            416 | 200 => {
                (*conn).supported = 0 as libc::c_int != 0;
            }
            206 => {}
            _ => return 0 as libc::c_int,
        }
    }
    __a = (*conn).size;
    tmp___6 = http_size(((*conn).http).as_mut_ptr());
    __b = tmp___6;
    if __a > __b {
        tmp___7 = __a;
    } else {
        tmp___7 = __b;
    }
    (*conn).size = tmp___7;
    if (*conn).size <= 0 as libc::c_long {
        (*conn).size = 9223372036854775807 as libc::c_longlong as off_t;
        (*conn).supported = 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn conn_info_status_get(
    mut msg: *mut libc::c_char,
    mut size: size_t,
    mut conn: *mut conn_t,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut tmp___1: size_t = 0;
    let mut __a: size_t = 0;
    let mut __b: size_t = 0;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp___3 = is_proto_http((*conn).proto);
    if tmp___3 != 0 {
        p = (*conn).http[0 as libc::c_int as usize].headers[0 as libc::c_int as usize].p;
        loop {
            tmp = p;
            p = p.offset(1);
            if !(*tmp as libc::c_int != 32 as libc::c_int) {
                break;
            }
        }
        loop {
            tmp___0 = p;
            p = p.offset(1);
            if !(*tmp___0 as libc::c_int != 32 as libc::c_int) {
                break;
            }
        }
        tmp___1 = strcspn(
            p as *const libc::c_char,
            b"\r\n\0" as *const u8 as *const libc::c_char,
        );
        len = tmp___1;
        if len != 0 {
            __a = len.wrapping_add(1 as libc::c_ulong);
            __b = size;
            if __a < __b {
                tmp___2 = __a;
            } else {
                tmp___2 = __b;
            }
            strlcpy(msg, p as *const libc::c_char, tmp___2);
            return (*conn).http[0 as libc::c_int as usize].status;
        }
    }
    tmp___4 = dcgettext(
        0 as *mut libc::c_void as *const libc::c_char,
        b"Unknown Error\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    );
    strlcpy(msg, tmp___4 as *const libc::c_char, size);
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn ftp_connect(
    mut conn: *mut ftp_t,
    mut proto: libc::c_int,
    mut host: *mut libc::c_char,
    mut port: libc::c_int,
    mut user: *mut libc::c_char,
    mut pass: *mut libc::c_char,
    mut io_timeout: libc::c_uint,
) -> libc::c_int {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    (*conn).data_tcp.fd = -(1 as libc::c_int);
    tmp = malloc(1024 as libc::c_int as size_t);
    (*conn).message = tmp as *mut libc::c_char;
    if ((*conn).message).is_null() {
        return 0 as libc::c_int;
    }
    (*conn).proto = proto;
    tmp___0 = tcp_connect(
        &mut (*conn).tcp,
        host,
        port,
        ((*conn).proto & 1 as libc::c_int == 1 as libc::c_int) as libc::c_int,
        (*conn).local_if,
        io_timeout,
    );
    if tmp___0 == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    tmp___1 = ftp_wait(conn);
    if tmp___1 / 100 as libc::c_int != 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    ftp_command(conn, b"USER %s\0" as *const u8 as *const libc::c_char, user);
    tmp___3 = ftp_wait(conn);
    if tmp___3 / 100 as libc::c_int != 2 as libc::c_int {
        if (*conn).status / 100 as libc::c_int == 3 as libc::c_int {
            ftp_command(conn, b"PASS %s\0" as *const u8 as *const libc::c_char, pass);
            tmp___2 = ftp_wait(conn);
            if tmp___2 / 100 as libc::c_int != 2 as libc::c_int {
                return 0 as libc::c_int;
            }
        } else {
            return 0 as libc::c_int
        }
    }
    ftp_command(conn, b"TYPE I\0" as *const u8 as *const libc::c_char);
    tmp___4 = ftp_wait(conn);
    if tmp___4 / 100 as libc::c_int != 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn ftp_disconnect(mut conn: *mut ftp_t) {
    tcp_close(&mut (*conn).tcp);
    tcp_close(&mut (*conn).data_tcp);
    if !((*conn).message).is_null() {
        free((*conn).message as *mut libc::c_void);
        (*conn).message = 0 as *mut libc::c_void as *mut libc::c_char;
    }
    (*conn).cwd[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
}
pub unsafe extern "C" fn ftp_cwd(
    mut conn: *mut ftp_t,
    mut cwd: *mut libc::c_char,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    tmp = strncmp(
        ((*conn).cwd).as_mut_ptr() as *const libc::c_char,
        cwd as *const libc::c_char,
        1024 as libc::c_int as size_t,
    );
    if tmp == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    ftp_command(conn, b"CWD %s\0" as *const u8 as *const libc::c_char, cwd);
    tmp___1 = ftp_wait(conn);
    if tmp___1 / 100 as libc::c_int != 2 as libc::c_int {
        tmp___0 = dcgettext(
            0 as *mut libc::c_void as *const libc::c_char,
            b"Can't change directory to %s\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        fprintf(stderr, tmp___0 as *const libc::c_char, cwd);
        return 0 as libc::c_int;
    }
    strlcpy(
        ((*conn).cwd).as_mut_ptr(),
        cwd as *const libc::c_char,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn ftp_size(
    mut conn: *mut ftp_t,
    mut file: *mut libc::c_char,
    mut maxredir: libc::c_int,
    mut io_timeout: libc::c_uint,
) -> off_t {
    let mut i: off_t = 0;
    let mut j: off_t = 0;
    let mut size: off_t = 0;
    let mut reply: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fn_0: [libc::c_char; 1024] = [0; 1024];
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___7: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___8: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___11: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___12: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___13: off_t = 0;
    let mut tmp___14: libc::c_int = 0;
    let mut tmp___15: libc::c_int = 0;
    size = 1024 as libc::c_int as size_t as off_t;
    tmp___1 = strchr(file as *const libc::c_char, '*' as i32);
    if tmp___1.is_null() {
        tmp___2 = strchr(file as *const libc::c_char, '?' as i32);
        if tmp___2.is_null() {
            ftp_command(conn, b"SIZE %s\0" as *const u8 as *const libc::c_char, file);
            tmp___0 = ftp_wait(conn);
            if tmp___0 / 100 as libc::c_int == 2 as libc::c_int {
                sscanf(
                    (*conn).message as *const libc::c_char,
                    b"%*i %jd\0" as *const u8 as *const libc::c_char,
                    &mut i as *mut off_t,
                );
                return i;
            } else {
                if (*conn).status / 10 as libc::c_int != 50 as libc::c_int {
                    tmp = dcgettext(
                        0 as *mut libc::c_void as *const libc::c_char,
                        b"File not found.\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    );
                    fprintf(stderr, tmp as *const libc::c_char);
                    return -(1 as libc::c_int) as off_t;
                }
            }
        }
    }
    if maxredir == 0 as libc::c_int {
        tmp___3 = dcgettext(
            0 as *mut libc::c_void as *const libc::c_char,
            b"Too many redirects.\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        fprintf(stderr, tmp___3 as *const libc::c_char);
        return -(1 as libc::c_int) as off_t;
    }
    tmp___4 = ftp_data(conn, io_timeout);
    if tmp___4 == 0 {
        return -(1 as libc::c_int) as off_t;
    }
    ftp_command(conn, b"LIST %s\0" as *const u8 as *const libc::c_char, file);
    tmp___5 = ftp_wait(conn);
    if tmp___5 / 100 as libc::c_int != 1 as libc::c_int {
        return -(1 as libc::c_int) as off_t;
    }
    tmp___6 = calloc(1 as libc::c_int as size_t, size as size_t);
    reply = tmp___6 as *mut libc::c_char;
    if reply.is_null() {
        return -(1 as libc::c_int) as off_t;
    }
    *reply = '\n' as i32 as libc::c_char;
    i = 1 as libc::c_int as off_t;
    loop {
        j = tcp_read(
            &mut (*conn).data_tcp,
            reply.offset(i as isize) as *mut libc::c_void,
            (size - i - 3 as libc::c_long) as libc::c_int,
        );
        if !(j > 0 as libc::c_long) {
            break;
        }
        i += j;
        *reply.offset(i as isize) = 0 as libc::c_int as libc::c_char;
        if size - i <= 10 as libc::c_long {
            size *= 2 as libc::c_long;
            tmp___8 = realloc(reply as *mut libc::c_void, size as size_t);
            tmp___7 = tmp___8 as *mut libc::c_char;
            if tmp___7.is_null() {
                free(reply as *mut libc::c_void);
                return -(1 as libc::c_int) as off_t;
            }
            reply = tmp___7;
            memset(
                reply.offset((size / 2 as libc::c_long) as isize) as *mut libc::c_void,
                0 as libc::c_int,
                (size / 2 as libc::c_long) as size_t,
            );
        }
    }
    tcp_close(&mut (*conn).data_tcp);
    tmp___9 = ftp_wait(conn);
    if tmp___9 / 100 as libc::c_int != 2 as libc::c_int {
        free(reply as *mut libc::c_void);
        return -(1 as libc::c_int) as off_t;
    }
    j = 0 as libc::c_int as off_t;
    i = 1 as libc::c_int as off_t;
    while *reply.offset(i as isize) != 0 {
        if *reply.offset((i + 1 as libc::c_long) as isize) == 0 {
            break;
        }
        if *reply.offset(i as isize) as libc::c_int == 45 as libc::c_int {
            j += 1;
        } else if *reply.offset(i as isize) as libc::c_int == 108 as libc::c_int {
            j += 1;
        } else {
            while *reply.offset(i as isize) as libc::c_int != 10 as libc::c_int {
                if *reply.offset(i as isize) == 0 {
                    break;
                }
                i += 1;
            }
        }
        i += 1;
    }
    if j != 1 as libc::c_long {
        if j == 0 as libc::c_long {
            tmp___10 = dcgettext(
                0 as *mut libc::c_void as *const libc::c_char,
                b"File not found.\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            fprintf(stderr, tmp___10 as *const libc::c_char);
        } else {
            tmp___11 = dcgettext(
                0 as *mut libc::c_void as *const libc::c_char,
                b"Multiple matches for this URL.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            );
            fprintf(stderr, tmp___11 as *const libc::c_char);
        }
        free(reply as *mut libc::c_void);
        return -(1 as libc::c_int) as off_t;
    }
    s = strstr(
        reply as *const libc::c_char,
        b"\nl\0" as *const u8 as *const libc::c_char,
    );
    if s as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        sscanf(
            s as *const libc::c_char,
            b"%*s %*i %*s %*s %*i %*s %*i %*s %100s\0" as *const u8
                as *const libc::c_char,
            fn_0.as_mut_ptr(),
        );
        strcpy(file, fn_0.as_mut_ptr() as *const libc::c_char);
        tmp___12 = strstr(
            s as *const libc::c_char,
            b"->\0" as *const u8 as *const libc::c_char,
        );
        strlcpy(
            fn_0.as_mut_ptr(),
            tmp___12.offset(3 as libc::c_int as isize) as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
        fn_0[(::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_ulong) as usize] = '\u{0}' as i32 as libc::c_char;
        free(reply as *mut libc::c_void);
        reply = strchr(fn_0.as_mut_ptr() as *const libc::c_char, '\r' as i32);
        if reply as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            *reply = 0 as libc::c_int as libc::c_char;
        }
        reply = strchr(fn_0.as_mut_ptr() as *const libc::c_char, '\n' as i32);
        if reply as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            *reply = 0 as libc::c_int as libc::c_char;
        }
        tmp___13 = ftp_size(
            conn,
            fn_0.as_mut_ptr(),
            maxredir - 1 as libc::c_int,
            io_timeout,
        );
        return tmp___13;
    } else {
        s = strstr(
            reply as *const libc::c_char,
            b"\n-\0" as *const u8 as *const libc::c_char,
        );
        tmp___14 = sscanf(
            s as *const libc::c_char,
            b"%*s %*i %*s %*s %jd %*s %*i %*s %100s\0" as *const u8
                as *const libc::c_char,
            &mut size as *mut off_t,
            fn_0.as_mut_ptr(),
        );
        i = tmp___14 as off_t;
        if i < 2 as libc::c_long {
            tmp___15 = sscanf(
                s as *const libc::c_char,
                b"%*s %*i %jd %*i %*s %*i %*i %100s\0" as *const u8
                    as *const libc::c_char,
                &mut size as *mut off_t,
                fn_0.as_mut_ptr(),
            );
            i = tmp___15 as off_t;
            if i < 2 as libc::c_long {
                return -(2 as libc::c_int) as off_t;
            }
        }
        strcpy(file, fn_0.as_mut_ptr() as *const libc::c_char);
        free(reply as *mut libc::c_void);
        return size;
    };
}
pub unsafe extern "C" fn ftp_data(
    mut conn: *mut ftp_t,
    mut io_timeout: libc::c_uint,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut info: [libc::c_int; 6] = [0; 6];
    let mut host: [libc::c_char; 1024] = [0; 1024];
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: libc::c_int = 0;
    if (*conn).data_tcp.fd > 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    ftp_command(conn, b"PASV\0" as *const u8 as *const libc::c_char);
    tmp = ftp_wait(conn);
    if tmp / 100 as libc::c_int != 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    host[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    i = 0 as libc::c_int;
    while *((*conn).message).offset(i as isize) != 0 {
        tmp___0 = sscanf(
            ((*conn).message).offset(i as isize) as *const libc::c_char,
            b"%i,%i,%i,%i,%i,%i\0" as *const u8 as *const libc::c_char,
            &mut *info.as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut libc::c_int,
            &mut *info.as_mut_ptr().offset(1 as libc::c_int as isize)
                as *mut libc::c_int,
            &mut *info.as_mut_ptr().offset(2 as libc::c_int as isize)
                as *mut libc::c_int,
            &mut *info.as_mut_ptr().offset(3 as libc::c_int as isize)
                as *mut libc::c_int,
            &mut *info.as_mut_ptr().offset(4 as libc::c_int as isize)
                as *mut libc::c_int,
            &mut *info.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut libc::c_int,
        );
        if tmp___0 == 6 as libc::c_int {
            snprintf(
                host.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                b"%i.%i.%i.%i\0" as *const u8 as *const libc::c_char,
                info[0 as libc::c_int as usize],
                info[1 as libc::c_int as usize],
                info[2 as libc::c_int as usize],
                info[3 as libc::c_int as usize],
            );
            break;
        } else {
            i += 1;
        }
    }
    if host[0 as libc::c_int as usize] == 0 {
        tmp___1 = dcgettext(
            0 as *mut libc::c_void as *const libc::c_char,
            b"Error opening passive data connection.\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        );
        fprintf(stderr, tmp___1 as *const libc::c_char);
        return 0 as libc::c_int;
    }
    tmp___2 = tcp_connect(
        &mut (*conn).data_tcp,
        host.as_mut_ptr(),
        info[4 as libc::c_int as usize] * 256 as libc::c_int
            + info[5 as libc::c_int as usize],
        ((*conn).proto & 1 as libc::c_int == 1 as libc::c_int) as libc::c_int,
        (*conn).local_if,
        io_timeout,
    );
    if tmp___2 == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn ftp_command(
    mut conn: *mut ftp_t,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut params: ::std::ffi::VaListImpl;
    let mut cmd: [libc::c_char; 1024] = [0; 1024];
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: ssize_t = 0;
    let mut tmp___2: size_t = 0;
    params = args.clone();
    vsnprintf(
        cmd.as_mut_ptr(),
        (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
            .wrapping_sub(3 as libc::c_ulong),
        format,
        params.as_va_list(),
    );
    strlcat(
        cmd.as_mut_ptr(),
        b"\r\n\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    tmp___0 = strlen(cmd.as_mut_ptr() as *const libc::c_char);
    tmp___1 = tcp_write(
        &mut (*conn).tcp,
        cmd.as_mut_ptr() as *mut libc::c_void,
        tmp___0 as libc::c_int,
    );
    tmp___2 = strlen(cmd.as_mut_ptr() as *const libc::c_char);
    if tmp___1 != tmp___2 as ssize_t {
        tmp = dcgettext(
            0 as *mut libc::c_void as *const libc::c_char,
            b"Error writing command %s\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        fprintf(stderr, tmp as *const libc::c_char, cmd.as_mut_ptr());
        return 0 as libc::c_int;
    } else {
        return 1 as libc::c_int
    };
}
pub unsafe extern "C" fn ftp_wait(mut conn: *mut ftp_t) -> libc::c_int {
    let mut size: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut complete: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut new_msg: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: ssize_t = 0;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_msg___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut k: libc::c_int = 0;
    let mut tmp___3: size_t = 0;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    size = 1024 as libc::c_int as size_t as libc::c_int;
    r = 0 as libc::c_int;
    tmp = realloc((*conn).message as *mut libc::c_void, size as size_t);
    new_msg = tmp;
    if new_msg.is_null() {
        return -(1 as libc::c_int);
    }
    (*conn).message = new_msg as *mut libc::c_char;
    loop {
        loop {
            tmp___0 = tcp_read(
                &mut (*conn).tcp,
                ((*conn).message).offset(r as isize) as *mut libc::c_void,
                1 as libc::c_int,
            );
            i = tmp___0 as libc::c_int;
            r += i;
            if i <= 0 as libc::c_int {
                tmp___1 = dcgettext(
                    0 as *mut libc::c_void as *const libc::c_char,
                    b"Connection gone.\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                );
                fprintf(stderr, tmp___1 as *const libc::c_char);
                return -(1 as libc::c_int);
            }
            if r + 10 as libc::c_int >= size {
                size = (size as size_t).wrapping_add(1024 as libc::c_ulong)
                    as libc::c_int;
                tmp___2 = realloc((*conn).message as *mut libc::c_void, size as size_t);
                new_msg___0 = tmp___2;
                if new_msg___0.is_null() {
                    return -(1 as libc::c_int);
                }
                (*conn).message = new_msg___0 as *mut libc::c_char;
            }
            if !(*((*conn).message).offset((r - 1 as libc::c_int) as isize)
                as libc::c_int != 10 as libc::c_int)
            {
                break;
            }
        }
        *((*conn).message).offset(r as isize) = 0 as libc::c_int as libc::c_char;
        sscanf(
            (*conn).message as *const libc::c_char,
            b"%i\0" as *const u8 as *const libc::c_char,
            &mut (*conn).status as *mut libc::c_int,
        );
        if *((*conn).message).offset(3 as libc::c_int as isize) as libc::c_int
            == 32 as libc::c_int
        {
            complete = 1 as libc::c_int;
        } else {
            complete = 0 as libc::c_int;
        }
        i = 0 as libc::c_int;
        while *((*conn).message).offset(i as isize) != 0 {
            if *((*conn).message).offset(i as isize) as libc::c_int == 10 as libc::c_int
            {
                if complete == 1 as libc::c_int {
                    complete = 2 as libc::c_int;
                    break;
                } else if *((*conn).message).offset((i + 4 as libc::c_int) as isize)
                        as libc::c_int == 32 as libc::c_int
                    {
                    j = -(1 as libc::c_int);
                    sscanf(
                        ((*conn).message).offset((i + 1 as libc::c_int) as isize)
                            as *const libc::c_char,
                        b"%3i\0" as *const u8 as *const libc::c_char,
                        &mut j as *mut libc::c_int,
                    );
                    if j == (*conn).status {
                        complete = 1 as libc::c_int;
                    }
                }
            }
            i += 1;
        }
        if !(complete != 2 as libc::c_int) {
            break;
        }
    }
    tmp___3 = strcspn(
        (*conn).message as *const libc::c_char,
        b"\r\n\0" as *const u8 as *const libc::c_char,
    );
    k = tmp___3 as libc::c_int;
    *((*conn).message).offset(k as isize) = 0 as libc::c_int as libc::c_char;
    tmp___4 = realloc(
        (*conn).message as *mut libc::c_void,
        (k + 1 as libc::c_int) as size_t,
    );
    (*conn).message = tmp___4 as *mut libc::c_char;
    return (*conn).status;
}
#[inline]
unsafe extern "C" fn is_default_port(
    mut proto: libc::c_int,
    mut port: libc::c_int,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut current_block_10: u64;
    if proto == (1 as libc::c_int) << 1 as libc::c_int {
        if port == 80 as libc::c_int {
            tmp = 1 as libc::c_int;
            current_block_10 = 1856101646708284338;
        } else {
            current_block_10 = 15061602190030536282;
        }
    } else {
        current_block_10 = 15061602190030536282;
    }
    match current_block_10 {
        15061602190030536282 => {
            if proto == (1 as libc::c_int) << 1 as libc::c_int | 1 as libc::c_int {
                if port == 443 as libc::c_int {
                    tmp = 1 as libc::c_int;
                } else {
                    tmp = 0 as libc::c_int;
                }
            } else {
                tmp = 0 as libc::c_int;
            }
        }
        _ => {}
    }
    return tmp;
}
#[inline]
unsafe extern "C" fn chain_next(mut p: *mut *mut *const libc::c_char) -> libc::c_char {
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    while !(**p).is_null() {
        if ***p != 0 {
            break;
        }
        *p = (*p).offset(1);
    }
    if !(**p).is_null() {
        tmp = **p;
        **p = (**p).offset(1);
        tmp___0 = *tmp as libc::c_int;
    } else {
        tmp___0 = 0 as libc::c_int;
    }
    return tmp___0 as libc::c_char;
}
unsafe extern "C" fn http_auth_token(
    mut token: *mut libc::c_char,
    mut user: *const libc::c_char,
    mut pass: *const libc::c_char,
) {
    let mut base64_encode: [libc::c_char; 64] = [0; 64];
    let mut auth: [*const libc::c_char; 4] = [0 as *const libc::c_char; 4];
    let mut p: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut a: libc::c_char = 0;
    let mut tmp: libc::c_char = 0;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut b: libc::c_char = 0;
    let mut tmp___1: libc::c_char = 0;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = 0;
    let mut tmp___5: libc::c_char = 0;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___7: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___8: *mut libc::c_char = 0 as *mut libc::c_char;
    base64_encode[0 as libc::c_int as usize] = 'A' as i32 as libc::c_char;
    base64_encode[1 as libc::c_int as usize] = 'B' as i32 as libc::c_char;
    base64_encode[2 as libc::c_int as usize] = 'C' as i32 as libc::c_char;
    base64_encode[3 as libc::c_int as usize] = 'D' as i32 as libc::c_char;
    base64_encode[4 as libc::c_int as usize] = 'E' as i32 as libc::c_char;
    base64_encode[5 as libc::c_int as usize] = 'F' as i32 as libc::c_char;
    base64_encode[6 as libc::c_int as usize] = 'G' as i32 as libc::c_char;
    base64_encode[7 as libc::c_int as usize] = 'H' as i32 as libc::c_char;
    base64_encode[8 as libc::c_int as usize] = 'I' as i32 as libc::c_char;
    base64_encode[9 as libc::c_int as usize] = 'J' as i32 as libc::c_char;
    base64_encode[10 as libc::c_int as usize] = 'K' as i32 as libc::c_char;
    base64_encode[11 as libc::c_int as usize] = 'L' as i32 as libc::c_char;
    base64_encode[12 as libc::c_int as usize] = 'M' as i32 as libc::c_char;
    base64_encode[13 as libc::c_int as usize] = 'N' as i32 as libc::c_char;
    base64_encode[14 as libc::c_int as usize] = 'O' as i32 as libc::c_char;
    base64_encode[15 as libc::c_int as usize] = 'P' as i32 as libc::c_char;
    base64_encode[16 as libc::c_int as usize] = 'Q' as i32 as libc::c_char;
    base64_encode[17 as libc::c_int as usize] = 'R' as i32 as libc::c_char;
    base64_encode[18 as libc::c_int as usize] = 'S' as i32 as libc::c_char;
    base64_encode[19 as libc::c_int as usize] = 'T' as i32 as libc::c_char;
    base64_encode[20 as libc::c_int as usize] = 'U' as i32 as libc::c_char;
    base64_encode[21 as libc::c_int as usize] = 'V' as i32 as libc::c_char;
    base64_encode[22 as libc::c_int as usize] = 'W' as i32 as libc::c_char;
    base64_encode[23 as libc::c_int as usize] = 'X' as i32 as libc::c_char;
    base64_encode[24 as libc::c_int as usize] = 'Y' as i32 as libc::c_char;
    base64_encode[25 as libc::c_int as usize] = 'Z' as i32 as libc::c_char;
    base64_encode[26 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    base64_encode[27 as libc::c_int as usize] = 'b' as i32 as libc::c_char;
    base64_encode[28 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    base64_encode[29 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
    base64_encode[30 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    base64_encode[31 as libc::c_int as usize] = 'f' as i32 as libc::c_char;
    base64_encode[32 as libc::c_int as usize] = 'g' as i32 as libc::c_char;
    base64_encode[33 as libc::c_int as usize] = 'h' as i32 as libc::c_char;
    base64_encode[34 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    base64_encode[35 as libc::c_int as usize] = 'j' as i32 as libc::c_char;
    base64_encode[36 as libc::c_int as usize] = 'k' as i32 as libc::c_char;
    base64_encode[37 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    base64_encode[38 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    base64_encode[39 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    base64_encode[40 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    base64_encode[41 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    base64_encode[42 as libc::c_int as usize] = 'q' as i32 as libc::c_char;
    base64_encode[43 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    base64_encode[44 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    base64_encode[45 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    base64_encode[46 as libc::c_int as usize] = 'u' as i32 as libc::c_char;
    base64_encode[47 as libc::c_int as usize] = 'v' as i32 as libc::c_char;
    base64_encode[48 as libc::c_int as usize] = 'w' as i32 as libc::c_char;
    base64_encode[49 as libc::c_int as usize] = 'x' as i32 as libc::c_char;
    base64_encode[50 as libc::c_int as usize] = 'y' as i32 as libc::c_char;
    base64_encode[51 as libc::c_int as usize] = 'z' as i32 as libc::c_char;
    base64_encode[52 as libc::c_int as usize] = '0' as i32 as libc::c_char;
    base64_encode[53 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    base64_encode[54 as libc::c_int as usize] = '2' as i32 as libc::c_char;
    base64_encode[55 as libc::c_int as usize] = '3' as i32 as libc::c_char;
    base64_encode[56 as libc::c_int as usize] = '4' as i32 as libc::c_char;
    base64_encode[57 as libc::c_int as usize] = '5' as i32 as libc::c_char;
    base64_encode[58 as libc::c_int as usize] = '6' as i32 as libc::c_char;
    base64_encode[59 as libc::c_int as usize] = '7' as i32 as libc::c_char;
    base64_encode[60 as libc::c_int as usize] = '8' as i32 as libc::c_char;
    base64_encode[61 as libc::c_int as usize] = '9' as i32 as libc::c_char;
    base64_encode[62 as libc::c_int as usize] = '+' as i32 as libc::c_char;
    base64_encode[63 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    auth[0 as libc::c_int as usize] = user;
    auth[1 as libc::c_int as usize] = b":\0" as *const u8 as *const libc::c_char;
    auth[2 as libc::c_int as usize] = pass;
    auth[3 as libc::c_int as usize] = 0 as *mut libc::c_void as *const libc::c_char;
    p = auth.as_mut_ptr();
    while !(*p).is_null() {
        tmp = chain_next(&mut p);
        a = tmp;
        if a == 0 {
            break;
        }
        tmp___0 = token;
        token = token.offset(1);
        *tmp___0 = base64_encode[(a as libc::c_int >> 2 as libc::c_int) as usize];
        tmp___1 = chain_next(&mut p);
        b = tmp___1;
        tmp___2 = token;
        token = token.offset(1);
        *tmp___2 = base64_encode[((a as libc::c_int & 3 as libc::c_int)
            << 4 as libc::c_int | b as libc::c_int >> 4 as libc::c_int) as usize];
        if b == 0 {
            tmp___3 = token;
            token = token.offset(1);
            *tmp___3 = '=' as i32 as libc::c_char;
            tmp___4 = token;
            token = token.offset(1);
            *tmp___4 = '=' as i32 as libc::c_char;
            break;
        } else {
            tmp___5 = chain_next(&mut p);
            c = tmp___5;
            tmp___6 = token;
            token = token.offset(1);
            *tmp___6 = base64_encode[((b as libc::c_int & 15 as libc::c_int)
                << 2 as libc::c_int | c as libc::c_int >> 6 as libc::c_int) as usize];
            if c == 0 {
                tmp___7 = token;
                token = token.offset(1);
                *tmp___7 = '=' as i32 as libc::c_char;
                break;
            } else {
                tmp___8 = token;
                token = token.offset(1);
                *tmp___8 = base64_encode[(c as libc::c_int & 63 as libc::c_int)
                    as usize];
            }
        }
    }
}
pub unsafe extern "C" fn http_connect(
    mut conn: *mut http_t,
    mut proto: libc::c_int,
    mut proxy: *mut libc::c_char,
    mut host: *mut libc::c_char,
    mut port: libc::c_int,
    mut user: *mut libc::c_char,
    mut pass: *mut libc::c_char,
    mut io_timeout: libc::c_uint,
) -> libc::c_int {
    let mut puser: *const libc::c_char = 0 as *const libc::c_char;
    let mut ppass: *const libc::c_char = 0 as *const libc::c_char;
    let mut tconn: [conn_t; 1] = [conn_t {
        conf: 0 as *mut conf_t,
        proto: 0,
        port: 0,
        proxy: 0,
        host: [0; 1024],
        dir: [0; 1024],
        file: [0; 1024],
        user: [0; 1024],
        pass: [0; 1024],
        output_filename: [0; 1024],
        ftp: [ftp_t {
            cwd: [0; 1024],
            message: 0 as *mut libc::c_char,
            status: 0,
            tcp: tcp_t { fd: 0, ai_family: 0 },
            data_tcp: tcp_t { fd: 0, ai_family: 0 },
            proto: 0,
            ftp_mode: 0,
            local_if: 0 as *mut libc::c_char,
        }; 1],
        http: [http_t {
            host: [0; 1024],
            auth: [0; 1024],
            request: [abuf_t {
                p: 0 as *mut libc::c_char,
                len: 0,
            }; 1],
            headers: [abuf_t {
                p: 0 as *mut libc::c_char,
                len: 0,
            }; 1],
            port: 0,
            proto: 0,
            proxy: 0,
            proxy_auth: [0; 1024],
            firstbyte: 0,
            lastbyte: 0,
            status: 0,
            tcp: tcp_t { fd: 0, ai_family: 0 },
            local_if: 0 as *mut libc::c_char,
        }; 1],
        size: 0,
        currentbyte: 0,
        lastbyte: 0,
        tcp: 0 as *mut tcp_t,
        enabled: false,
        supported: false,
        last_transfer: 0,
        message: 0 as *mut libc::c_char,
        local_if: 0 as *mut libc::c_char,
        state: false,
        setup_thread: [0; 1],
        lock: __anonunion_pthread_mutex_t_335460617 {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_list_t {
                    __prev: 0 as *mut __pthread_internal_list,
                    __next: 0 as *mut __pthread_internal_list,
                },
            },
        },
    }; 1];
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    puser = 0 as *mut libc::c_void as *const libc::c_char;
    ppass = b"\0" as *const u8 as *const libc::c_char;
    strlcpy(
        ((*conn).host).as_mut_ptr(),
        host as *const libc::c_char,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    (*conn).port = port;
    (*conn).proto = proto;
    if !proxy.is_null() {
        if *proxy != 0 {
            tmp___0 = conn_set(tconn.as_mut_ptr(), proxy as *const libc::c_char);
            if tmp___0 == 0 {
                tmp = dcgettext(
                    0 as *mut libc::c_void as *const libc::c_char,
                    b"Invalid proxy string: %s\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                );
                fprintf(stderr, tmp as *const libc::c_char, proxy);
                return 0 as libc::c_int;
            }
            host = (tconn[0 as libc::c_int as usize].host).as_mut_ptr();
            port = tconn[0 as libc::c_int as usize].port;
            proto = tconn[0 as libc::c_int as usize].proto;
            puser = (tconn[0 as libc::c_int as usize].user).as_mut_ptr()
                as *const libc::c_char;
            ppass = (tconn[0 as libc::c_int as usize].pass).as_mut_ptr()
                as *const libc::c_char;
            (*conn).proxy = 1 as libc::c_int;
        }
    }
    tmp___1 = tcp_connect(
        &mut (*conn).tcp,
        host,
        port,
        (proto & 1 as libc::c_int == 1 as libc::c_int) as libc::c_int,
        (*conn).local_if,
        io_timeout,
    );
    if tmp___1 == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    if *user as libc::c_int == 0 as libc::c_int {
        (*conn).auth[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    } else {
        http_auth_token(
            ((*conn).auth).as_mut_ptr(),
            user as *const libc::c_char,
            pass as *const libc::c_char,
        );
    }
    if (*conn).proxy == 0 {
        (*conn).proxy_auth[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    } else if puser.is_null() {
        (*conn).proxy_auth[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    } else if *puser as libc::c_int == 0 as libc::c_int {
        (*conn).proxy_auth[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    } else {
        http_auth_token(((*conn).proxy_auth).as_mut_ptr(), puser, ppass);
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn http_disconnect(mut conn: *mut http_t) {
    tcp_close(&mut (*conn).tcp);
}
pub unsafe extern "C" fn http_get(mut conn: *mut http_t, mut lurl: *mut libc::c_char) {
    let mut prefix: *const libc::c_char = 0 as *const libc::c_char;
    let mut postfix: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut proto: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    prefix = b"\0" as *const u8 as *const libc::c_char;
    postfix = b"\0" as *const u8 as *const libc::c_char;
    tmp = is_ipv6_addr(((*conn).host).as_mut_ptr() as *const libc::c_char);
    if tmp != 0 {
        prefix = b"[\0" as *const u8 as *const libc::c_char;
        postfix = b"]\0" as *const u8 as *const libc::c_char;
    }
    *(*conn).request[0 as libc::c_int as usize].p = 0 as libc::c_int as libc::c_char;
    if (*conn).proxy != 0 {
        tmp___0 = scheme_from_proto((*conn).proto);
        proto = tmp___0;
        tmp___1 = is_default_port((*conn).proto, (*conn).port);
        if tmp___1 != 0 {
            http_addheader(
                conn,
                b"GET %s%s%s%s%s HTTP/1.0\0" as *const u8 as *const libc::c_char,
                proto,
                prefix,
                ((*conn).host).as_mut_ptr(),
                postfix,
                lurl,
            );
        } else {
            http_addheader(
                conn,
                b"GET %s%s%s%s:%i%s HTTP/1.0\0" as *const u8 as *const libc::c_char,
                proto,
                prefix,
                ((*conn).host).as_mut_ptr(),
                postfix,
                (*conn).port,
                lurl,
            );
        }
    } else {
        http_addheader(
            conn,
            b"GET %s HTTP/1.0\0" as *const u8 as *const libc::c_char,
            lurl,
        );
        tmp___2 = is_default_port((*conn).proto, (*conn).port);
        if tmp___2 != 0 {
            http_addheader(
                conn,
                b"Host: %s%s%s\0" as *const u8 as *const libc::c_char,
                prefix,
                ((*conn).host).as_mut_ptr(),
                postfix,
            );
        } else {
            http_addheader(
                conn,
                b"Host: %s%s%s:%i\0" as *const u8 as *const libc::c_char,
                prefix,
                ((*conn).host).as_mut_ptr(),
                postfix,
                (*conn).port,
            );
        }
    }
    if (*conn).auth[0 as libc::c_int as usize] != 0 {
        http_addheader(
            conn,
            b"Authorization: Basic %s\0" as *const u8 as *const libc::c_char,
            ((*conn).auth).as_mut_ptr(),
        );
    }
    if (*conn).proxy_auth[0 as libc::c_int as usize] != 0 {
        http_addheader(
            conn,
            b"Proxy-Authorization: Basic %s\0" as *const u8 as *const libc::c_char,
            ((*conn).proxy_auth).as_mut_ptr(),
        );
    }
    http_addheader(conn, b"Accept: */*\0" as *const u8 as *const libc::c_char);
    http_addheader(
        conn,
        b"Accept-Encoding: identity\0" as *const u8 as *const libc::c_char,
    );
    let mut current_block_38: u64;
    if (*conn).lastbyte != 0 {
        if (*conn).firstbyte >= 0 as libc::c_long {
            http_addheader(
                conn,
                b"Range: bytes=%jd-%jd\0" as *const u8 as *const libc::c_char,
                (*conn).firstbyte,
                (*conn).lastbyte - 1 as libc::c_long,
            );
            current_block_38 = 2873832966593178012;
        } else {
            current_block_38 = 15126892627961977460;
        }
    } else {
        current_block_38 = 15126892627961977460;
    }
    match current_block_38 {
        15126892627961977460 => {
            if (*conn).firstbyte >= 0 as libc::c_long {
                http_addheader(
                    conn,
                    b"Range: bytes=%jd-\0" as *const u8 as *const libc::c_char,
                    (*conn).firstbyte,
                );
            }
        }
        _ => {}
    };
}
pub unsafe extern "C" fn http_addheader(
    mut conn: *mut http_t,
    mut format: *const libc::c_char,
    mut args: ...
) {
    let mut s: [libc::c_char; 1024] = [0; 1024];
    let mut params: ::std::ffi::VaListImpl;
    let mut tmp: libc::c_int = 0;
    params = args.clone();
    vsnprintf(
        s.as_mut_ptr(),
        (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
            .wrapping_sub(3 as libc::c_ulong),
        format,
        params.as_va_list(),
    );
    strlcat(
        s.as_mut_ptr(),
        b"\r\n\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    tmp = abuf_strcat(
        ((*conn).request).as_mut_ptr(),
        s.as_mut_ptr() as *const libc::c_char,
    );
    if tmp < 0 as libc::c_int {
        fprintf(stderr, b"Out of memory\n\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn http_exec(mut conn: *mut http_t) -> libc::c_int {
    let mut s2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut reqlen: size_t = 0;
    let mut tmp: size_t = 0;
    let mut nwrite: size_t = 0;
    let mut tmp___0: ssize_t = 0;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: ssize_t = 0;
    let mut pos: size_t = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut reslen: size_t = 0;
    let mut ret: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    strlcat(
        (*conn).request[0 as libc::c_int as usize].p,
        b"\r\n\0" as *const u8 as *const libc::c_char,
        (*conn).request[0 as libc::c_int as usize].len,
    );
    tmp = strlen((*conn).request[0 as libc::c_int as usize].p as *const libc::c_char);
    reqlen = tmp;
    nwrite = 0 as libc::c_int as size_t;
    while nwrite < reqlen {
        tmp___0 = tcp_write(
            &mut (*conn).tcp,
            ((*conn).request[0 as libc::c_int as usize].p).offset(nwrite as isize)
                as *mut libc::c_void,
            reqlen.wrapping_sub(nwrite) as libc::c_int,
        );
        if tmp___0 < 0 as libc::c_long {
            tmp___1 = __errno_location();
            if *tmp___1 == 4 as libc::c_int {
                continue;
            }
            tmp___2 = __errno_location();
            if *tmp___2 == 11 as libc::c_int {
                continue;
            }
            tmp___3 = dcgettext(
                0 as *mut libc::c_void as *const libc::c_char,
                b"Connection gone while writing.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            );
            fprintf(stderr, tmp___3 as *const libc::c_char);
            return 0 as libc::c_int;
        } else {
            nwrite = (nwrite as libc::c_ulong).wrapping_add(tmp___0 as size_t) as size_t
                as size_t;
        }
    }
    *(*conn).headers[0 as libc::c_int as usize].p = 0 as libc::c_int as libc::c_char;
    s = (*conn).headers[0 as libc::c_int as usize].p;
    loop {
        tmp___5 = tcp_read(&mut (*conn).tcp, s as *mut libc::c_void, 1 as libc::c_int);
        if tmp___5 <= 0 as libc::c_long {
            tmp___4 = dcgettext(
                0 as *mut libc::c_void as *const libc::c_char,
                b"Connection gone.\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            fprintf(stderr, tmp___4 as *const libc::c_char);
            return 0 as libc::c_int;
        }
        if *s as libc::c_int == 13 as libc::c_int {
            continue;
        }
        if *s as libc::c_int == 10 as libc::c_int {
            if s as libc::c_ulong
                > (*conn).headers[0 as libc::c_int as usize].p as libc::c_ulong
            {
                if *s.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    == 10 as libc::c_int
                {
                    *s = 0 as libc::c_int as libc::c_char;
                    break;
                }
            }
        }
        s = s.offset(1);
        pos = s.offset_from((*conn).headers[0 as libc::c_int as usize].p) as libc::c_long
            as size_t;
        if pos.wrapping_add(10 as libc::c_ulong)
            < (*conn).headers[0 as libc::c_int as usize].len
        {
            tmp___7 = abuf_setup(
                ((*conn).headers).as_mut_ptr(),
                ((*conn).headers[0 as libc::c_int as usize].len)
                    .wrapping_add(512 as libc::c_ulong),
            );
            tmp___6 = tmp___7;
            if tmp___6 < 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"Out of memory\n\0" as *const u8 as *const libc::c_char,
                );
                return 0 as libc::c_int;
            }
            s = ((*conn).headers[0 as libc::c_int as usize].p).offset(pos as isize);
        }
    }
    sscanf(
        (*conn).headers[0 as libc::c_int as usize].p as *const libc::c_char,
        b"%*s %3i\0" as *const u8 as *const libc::c_char,
        &mut (*conn).status as *mut libc::c_int,
    );
    s2 = strchr(
        (*conn).headers[0 as libc::c_int as usize].p as *const libc::c_char,
        '\n' as i32,
    );
    if !s2.is_null() {
        *s2 = 0 as libc::c_int as libc::c_char;
    }
    reslen = (s2.offset_from((*conn).headers[0 as libc::c_int as usize].p)
        as libc::c_long + 1 as libc::c_long) as size_t;
    if (*conn).request[0 as libc::c_int as usize].len < reqlen {
        tmp___8 = abuf_setup(((*conn).request).as_mut_ptr(), reslen);
        ret = tmp___8;
        if ret < 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    memcpy(
        (*conn).request[0 as libc::c_int as usize].p as *mut libc::c_void,
        (*conn).headers[0 as libc::c_int as usize].p as *const libc::c_void,
        reslen,
    );
    *s2 = '\n' as i32 as libc::c_char;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn http_header(
    mut conn: *const http_t,
    mut header: *const libc::c_char,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut hlen: size_t = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: libc::c_int = 0;
    p = (*conn).headers[0 as libc::c_int as usize].p as *const libc::c_char;
    tmp = strlen(header);
    hlen = tmp;
    loop {
        tmp___0 = strncasecmp(p, header, hlen);
        if tmp___0 == 0 as libc::c_int {
            return p.offset(hlen as isize);
        }
        while *p as libc::c_int != 10 as libc::c_int {
            if *p == 0 {
                break;
            }
            p = p.offset(1);
        }
        if *p as libc::c_int == 10 as libc::c_int {
            p = p.offset(1);
        }
        if *p == 0 {
            break;
        }
    }
    return 0 as *mut libc::c_void as *const libc::c_char;
}
pub unsafe extern "C" fn http_size(mut conn: *mut http_t) -> off_t {
    let mut i: *const libc::c_char = 0 as *const libc::c_char;
    let mut j: off_t = 0;
    i = http_header(
        conn as *const http_t,
        b"Content-Length:\0" as *const u8 as *const libc::c_char,
    );
    if i as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(2 as libc::c_int) as off_t;
    }
    sscanf(i, b"%jd\0" as *const u8 as *const libc::c_char, &mut j as *mut off_t);
    return j;
}
pub unsafe extern "C" fn http_size_from_range(mut conn: *mut http_t) -> off_t {
    let mut i: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut j: off_t = 0;
    let mut tmp___1: libc::c_longlong = 0;
    i = http_header(
        conn as *const http_t,
        b"Content-Range:\0" as *const u8 as *const libc::c_char,
    );
    if i as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(2 as libc::c_int) as off_t;
    }
    tmp = strchr(i, '/' as i32);
    i = tmp as *const libc::c_char;
    tmp___0 = i;
    i = i.offset(1);
    if tmp___0.is_null() {
        return -(2 as libc::c_int) as off_t;
    }
    tmp___1 = strtoll(
        i,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    j = tmp___1 as off_t;
    if j == 0 {
        if *i as libc::c_int != 48 as libc::c_int {
            return -(3 as libc::c_int) as off_t;
        }
    }
    return j;
}
pub unsafe extern "C" fn http_filename(
    mut conn: *const http_t,
    mut filename: *mut libc::c_char,
) {
    let mut h: *const libc::c_char = 0 as *const libc::c_char;
    let mut space: [libc::c_char; 3] = [0; 3];
    let mut n: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: size_t = 0;
    let mut invalid: [libc::c_char; 10] = [0; 10];
    let mut replacement: libc::c_char = 0;
    let mut i: *mut libc::c_char = 0 as *mut libc::c_char;
    h = http_header(conn, b"Content-Disposition:\0" as *const u8 as *const libc::c_char);
    if h as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        sscanf(
            h,
            b"%*s%*[ \t]filename%*[ \t=\"'-]%254[^\n\"']\0" as *const u8
                as *const libc::c_char,
            filename,
        );
        space[0 as libc::c_int as usize] = '\t' as i32 as libc::c_char;
        space[1 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
        space[2 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        p = filename;
        loop {
            p = strpbrk(
                p as *const libc::c_char,
                space.as_mut_ptr() as *const libc::c_char,
            );
            if p.is_null() {
                break;
            }
            tmp = strspn(
                p as *const libc::c_char,
                space.as_mut_ptr() as *const libc::c_char,
            );
            n = p.offset(tmp as isize);
            if *n == 0 {
                *p = 0 as libc::c_int as libc::c_char;
                break;
            } else {
                p = n;
            }
        }
        invalid[0 as libc::c_int as usize] = '/' as i32 as libc::c_char;
        invalid[1 as libc::c_int as usize] = '\\' as i32 as libc::c_char;
        invalid[2 as libc::c_int as usize] = '?' as i32 as libc::c_char;
        invalid[3 as libc::c_int as usize] = '%' as i32 as libc::c_char;
        invalid[4 as libc::c_int as usize] = '*' as i32 as libc::c_char;
        invalid[5 as libc::c_int as usize] = ':' as i32 as libc::c_char;
        invalid[6 as libc::c_int as usize] = '|' as i32 as libc::c_char;
        invalid[7 as libc::c_int as usize] = '<' as i32 as libc::c_char;
        invalid[8 as libc::c_int as usize] = '>' as i32 as libc::c_char;
        invalid[9 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        replacement = '_' as i32 as libc::c_char;
        i = filename;
        loop {
            i = strpbrk(
                i as *const libc::c_char,
                invalid.as_mut_ptr() as *const libc::c_char,
            );
            if i.is_null() {
                break;
            }
            *i = replacement;
            i = i.offset(1);
        }
    }
}
#[inline]
unsafe extern "C" fn decode_nibble(mut n: libc::c_char) -> libc::c_char {
    if n as libc::c_int <= 57 as libc::c_int {
        return (n as libc::c_int - 48 as libc::c_int) as libc::c_char;
    }
    if n as libc::c_int >= 97 as libc::c_int {
        n = (n as libc::c_int - 32 as libc::c_int) as libc::c_char;
    }
    return (n as libc::c_int - 65 as libc::c_int + 10 as libc::c_int) as libc::c_char;
}
#[inline]
unsafe extern "C" fn encode_nibble(mut n: libc::c_char) -> libc::c_char {
    let mut tmp: libc::c_int = 0;
    if n as libc::c_int > 9 as libc::c_int {
        tmp = n as libc::c_int + 97 as libc::c_int - 10 as libc::c_int;
    } else {
        tmp = n as libc::c_int + 48 as libc::c_int;
    }
    return tmp as libc::c_char;
}
#[inline]
unsafe extern "C" fn encode_byte(mut dst: *mut libc::c_char, mut n: libc::c_char) {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = dst;
    dst = dst.offset(1);
    *tmp = '%' as i32 as libc::c_char;
    tmp___0 = dst;
    dst = dst.offset(1);
    *tmp___0 = encode_nibble((n as libc::c_int >> 4 as libc::c_int) as libc::c_char);
    *dst = encode_nibble((n as libc::c_int & 15 as libc::c_int) as libc::c_char);
}
pub unsafe extern "C" fn http_decode(mut s: *mut libc::c_char) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_char = 0;
    let mut tmp___1: libc::c_char = 0;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    while *s != 0 {
        if !(*s as libc::c_int != 37 as libc::c_int) {
            break;
        }
        s = s.offset(1);
    }
    if *s == 0 {
        return;
    }
    p = s;
    while !(*s.offset(1 as libc::c_int as isize) == 0) {
        if *s.offset(2 as libc::c_int as isize) == 0 {
            break;
        }
        tmp = p;
        p = p.offset(1);
        tmp___0 = decode_nibble(*s.offset(1 as libc::c_int as isize));
        tmp___1 = decode_nibble(*s.offset(2 as libc::c_int as isize));
        *tmp = ((tmp___0 as libc::c_int) << 4 as libc::c_int | tmp___1 as libc::c_int)
            as libc::c_char;
        s = s.offset(3 as libc::c_int as isize);
        while *s != 0 {
            if !(*s as libc::c_int != 37 as libc::c_int) {
                break;
            }
            tmp___2 = p;
            p = p.offset(1);
            tmp___3 = s;
            s = s.offset(1);
            *tmp___2 = *tmp___3;
        }
        if !(*s as libc::c_int == 37 as libc::c_int) {
            break;
        }
    }
    *p = 0 as libc::c_int as libc::c_char;
}
pub unsafe extern "C" fn http_encode(mut s: *mut libc::c_char, mut len: size_t) {
    let mut t: [libc::c_char; 1024] = [0; 1024];
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    j = 0 as libc::c_uint;
    i = j;
    let mut current_block_6: u64;
    while *s.offset(i as isize) != 0 {
        if !((j as libc::c_ulong)
            < (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_ulong))
        {
            break;
        }
        t[j as usize] = *s.offset(i as isize);
        if *s.offset(i as isize) as libc::c_int <= 32 as libc::c_int {
            current_block_6 = 7443571596885081620;
        } else if *s.offset(i as isize) as libc::c_int >= 127 as libc::c_int {
            current_block_6 = 7443571596885081620;
        } else {
            current_block_6 = 12800627514080957624;
        }
        match current_block_6 {
            7443571596885081620 => {
                if j as libc::c_ulong
                    >= (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                        .wrapping_sub(3 as libc::c_ulong)
                {
                    break;
                }
                encode_byte(t.as_mut_ptr().offset(j as isize), *s.offset(i as isize));
                j = j.wrapping_add(2 as libc::c_uint);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        j = j.wrapping_add(1);
    }
    t[j as usize] = 0 as libc::c_int as libc::c_char;
    strlcpy(s, t.as_mut_ptr() as *const libc::c_char, len);
}
pub unsafe extern "C" fn search_makelist(
    mut results: *mut search_t,
    mut orig_url: *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut size: libc::c_int = 0;
    let mut conn: [conn_t; 1] = [conn_t {
        conf: 0 as *mut conf_t,
        proto: 0,
        port: 0,
        proxy: 0,
        host: [0; 1024],
        dir: [0; 1024],
        file: [0; 1024],
        user: [0; 1024],
        pass: [0; 1024],
        output_filename: [0; 1024],
        ftp: [ftp_t {
            cwd: [0; 1024],
            message: 0 as *mut libc::c_char,
            status: 0,
            tcp: tcp_t { fd: 0, ai_family: 0 },
            data_tcp: tcp_t { fd: 0, ai_family: 0 },
            proto: 0,
            ftp_mode: 0,
            local_if: 0 as *mut libc::c_char,
        }; 1],
        http: [http_t {
            host: [0; 1024],
            auth: [0; 1024],
            request: [abuf_t {
                p: 0 as *mut libc::c_char,
                len: 0,
            }; 1],
            headers: [abuf_t {
                p: 0 as *mut libc::c_char,
                len: 0,
            }; 1],
            port: 0,
            proto: 0,
            proxy: 0,
            proxy_auth: [0; 1024],
            firstbyte: 0,
            lastbyte: 0,
            status: 0,
            tcp: tcp_t { fd: 0, ai_family: 0 },
            local_if: 0 as *mut libc::c_char,
        }; 1],
        size: 0,
        currentbyte: 0,
        lastbyte: 0,
        tcp: 0 as *mut tcp_t,
        enabled: false,
        supported: false,
        last_transfer: 0,
        message: 0 as *mut libc::c_char,
        local_if: 0 as *mut libc::c_char,
        state: false,
        setup_thread: [0; 1],
        lock: __anonunion_pthread_mutex_t_335460617 {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_list_t {
                    __prev: 0 as *mut __pthread_internal_list,
                    __next: 0 as *mut __pthread_internal_list,
                },
            },
        },
    }; 1];
    let mut t: libc::c_double = 0.;
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut orig_len: size_t = 0;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: libc::c_double = 0.;
    let mut nresults: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tmp___9: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___10: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___11: ssize_t = 0;
    let mut tmp___12: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___13: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut url: *const libc::c_char = 0 as *const libc::c_char;
    let mut eol: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___14: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___15: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___16: libc::c_int = 0;
    size = 8192 as libc::c_int;
    memset(
        conn.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<conn_t>() as libc::c_ulong,
    );
    conn[0 as libc::c_int as usize].conf = (*results).conf;
    t = axel_gettime();
    tmp = conn_set(conn.as_mut_ptr(), orig_url as *const libc::c_char);
    if tmp != 0 {
        tmp___0 = conn_init(conn.as_mut_ptr());
        if tmp___0 != 0 {
            tmp___1 = conn_info(conn.as_mut_ptr());
            if tmp___1 == 0 {
                return -(1 as libc::c_int);
            }
        } else {
            return -(1 as libc::c_int)
        }
    } else {
        return -(1 as libc::c_int)
    }
    tmp___2 = strlcpy(
        ((*results.offset(0 as libc::c_int as isize)).url).as_mut_ptr(),
        orig_url as *const libc::c_char,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    orig_len = tmp___2;
    tmp___3 = axel_gettime();
    (*results.offset(0 as libc::c_int as isize))
        .speed = (1 as libc::c_int as libc::c_double
        + 1000 as libc::c_int as libc::c_double * (tmp___3 - t)) as off_t;
    (*results.offset(0 as libc::c_int as isize))
        .size = conn[0 as libc::c_int as usize].size;
    nresults = 1 as libc::c_int;
    tmp___4 = malloc(size as size_t);
    s = tmp___4 as *mut libc::c_char;
    if s.is_null() {
        return 1 as libc::c_int;
    }
    snprintf(
        s,
        size as size_t,
        b"http://www.filesearching.com/cgi-bin/s?w=a&x=15&y=15&s=on&e=on&l=en&t=f&o=n&q=%s&m=%i&s1=%jd&s2=%jd\0"
            as *const u8 as *const libc::c_char,
        (conn[0 as libc::c_int as usize].file).as_mut_ptr(),
        (*(*results).conf).search_amount,
        conn[0 as libc::c_int as usize].size,
        conn[0 as libc::c_int as usize].size,
    );
    conn_disconnect(conn.as_mut_ptr());
    memset(
        conn.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<conn_t>() as libc::c_ulong,
    );
    conn[0 as libc::c_int as usize].conf = (*results).conf;
    tmp___5 = conn_set(conn.as_mut_ptr(), s as *const libc::c_char);
    if !(tmp___5 == 0) {
        pthread_mutex_lock(
            &mut (*conn.as_mut_ptr().offset(0 as libc::c_int as isize)).lock,
        );
        tmp___7 = conn_setup(conn.as_mut_ptr());
        tmp___6 = tmp___7;
        pthread_mutex_unlock(
            &mut (*conn.as_mut_ptr().offset(0 as libc::c_int as isize)).lock,
        );
        if !(tmp___6 == 0) {
            tmp___8 = conn_exec(conn.as_mut_ptr());
            if !(tmp___8 == 0) {
                j = 0 as libc::c_int;
                loop {
                    tmp___11 = tcp_read(
                        conn[0 as libc::c_int as usize].tcp,
                        s.offset(j as isize) as *mut libc::c_void,
                        size - j,
                    );
                    i = tmp___11 as libc::c_int;
                    if !(i > 0 as libc::c_int) {
                        current_block = 17747245473264231573;
                        break;
                    }
                    j += i;
                    if !(j + 10 as libc::c_int >= size) {
                        continue;
                    }
                    size *= 2 as libc::c_int;
                    tmp___10 = realloc(s as *mut libc::c_void, size as size_t);
                    tmp___9 = tmp___10 as *mut libc::c_char;
                    if tmp___9.is_null() {
                        current_block = 14498912063345108135;
                        break;
                    }
                    s = tmp___9;
                    memset(
                        s.offset((size / 2 as libc::c_int) as isize)
                            as *mut libc::c_void,
                        0 as libc::c_int,
                        (size / 2 as libc::c_int) as size_t,
                    );
                }
                match current_block {
                    14498912063345108135 => {}
                    _ => {
                        *s.offset(j as isize) = '\u{0}' as i32 as libc::c_char;
                        conn_disconnect(conn.as_mut_ptr());
                        tmp___12 = strstr(
                            s as *const libc::c_char,
                            b"<pre class=list\0" as *const u8 as *const libc::c_char,
                        );
                        start = tmp___12 as *const libc::c_char;
                        if !start.is_null() {
                            tmp___13 = strstr(
                                start,
                                b"</pre>\0" as *const u8 as *const libc::c_char,
                            );
                            end = tmp___13 as *const libc::c_char;
                            if !end.is_null() {
                                while (start as libc::c_ulong) < end as libc::c_ulong {
                                    if !(nresults < (*(*results).conf).search_amount) {
                                        break;
                                    }
                                    tmp___14 = strchr(start, '\n' as i32);
                                    eol = tmp___14 as *const libc::c_char;
                                    if eol as libc::c_ulong > end as libc::c_ulong {
                                        eol = end;
                                    } else if eol.is_null() {
                                        eol = end;
                                    }
                                    loop {
                                        url = start;
                                        tmp___15 = strstr(
                                            start,
                                            b"<a href=\0" as *const u8 as *const libc::c_char,
                                        );
                                        start = tmp___15.offset(8 as libc::c_int as isize)
                                            as *const libc::c_char;
                                        if !((start as libc::c_ulong) < eol as libc::c_ulong) {
                                            break;
                                        }
                                    }
                                    tmp___16 = strncmp(
                                        url,
                                        orig_url as *const libc::c_char,
                                        orig_len,
                                    );
                                    if !(tmp___16 == 0) {
                                        strlcpy(
                                            ((*results.offset(nresults as isize)).url).as_mut_ptr(),
                                            url,
                                            ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                as libc::c_ulong,
                                        );
                                        (*results.offset(nresults as isize))
                                            .size = (*results.offset(0 as libc::c_int as isize)).size;
                                        let ref mut fresh13 = (*results.offset(nresults as isize))
                                            .conf;
                                        *fresh13 = (*results).conf;
                                        nresults += 1;
                                    }
                                    start = eol.offset(1 as libc::c_int as isize);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    free(s as *mut libc::c_void);
    return nresults;
}
pub unsafe extern "C" fn search_getspeeds(
    mut results: *mut search_t,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut delay: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut left: libc::c_int = 0;
    let mut correct: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut running: libc::c_int = 0;
    let mut i___0: libc::c_int = 0;
    let mut tmp: libc::c_double = 0.;
    let mut tmp___0: libc::c_int = 0;
    delay.tv_sec = 0 as libc::c_long;
    delay.tv_nsec = 10000000 as libc::c_int as __syscall_slong_t;
    left = count;
    correct = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < count {
        if (*results.offset(i as isize)).speed != 0 {
            (*results.offset(i as isize))
                .speed_start_time = 0 as libc::c_int as libc::c_double;
            left -= 1;
            if (*results.offset(i as isize)).speed > 0 as libc::c_long {
                correct += 1;
            }
        }
        i += 1;
    }
    running = 0 as libc::c_int;
    while left > 0 as libc::c_int {
        i___0 = 0 as libc::c_int;
        while i___0 < count {
            match (*results.offset(i___0 as isize)).speed {
                -3 => {
                    tmp = axel_gettime();
                    if tmp
                        < (*results.offset(i___0 as isize)).speed_start_time
                            + (*(*results).conf).search_timeout as libc::c_double
                    {
                        current_block = 2783333934130355412;
                    } else {
                        pthread_cancel(
                            (*results.offset(i___0 as isize))
                                .speed_thread[0 as libc::c_int as usize],
                        );
                        current_block = 1538046216550696469;
                    }
                }
                -2 => {
                    current_block = 1538046216550696469;
                }
                -1 => {
                    current_block = 2783333934130355412;
                }
                0 => {
                    if running >= (*(*results).conf).search_threads {
                        current_block = 2783333934130355412;
                    } else {
                        (*results.offset(i___0 as isize))
                            .speed = -(3 as libc::c_int) as off_t;
                        (*results.offset(i___0 as isize))
                            .speed_start_time = axel_gettime();
                        tmp___0 = pthread_create(
                            ((*results.offset(i___0 as isize)).speed_thread)
                                .as_mut_ptr(),
                            0 as *mut libc::c_void as *const pthread_attr_t,
                            Some(
                                search_speedtest
                                    as unsafe extern "C" fn(
                                        *mut libc::c_void,
                                    ) -> *mut libc::c_void,
                            ),
                            results.offset(i___0 as isize) as *mut libc::c_void,
                        );
                        if tmp___0 == 0 as libc::c_int {
                            running += 1;
                        } else {
                            (*results.offset(i___0 as isize))
                                .speed = 0 as libc::c_int as off_t;
                        }
                        current_block = 2783333934130355412;
                    }
                }
                _ => {
                    if (*results.offset(i___0 as isize)).speed_start_time == 0. {
                        current_block = 2783333934130355412;
                    } else {
                        current_block = 1538046216550696469;
                    }
                }
            }
            match current_block {
                1538046216550696469 => {
                    pthread_join(
                        (*results.offset(i___0 as isize))
                            .speed_thread[0 as libc::c_int as usize],
                        0 as *mut libc::c_void as *mut *mut libc::c_void,
                    );
                    running -= 1;
                    left -= 1;
                    match (*results.offset(i___0 as isize)).speed {
                        -2 | -3 => {
                            (*results.offset(i___0 as isize))
                                .speed = -(1 as libc::c_int) as off_t;
                        }
                        _ => {
                            (*results.offset(i___0 as isize))
                                .speed_start_time = 0 as libc::c_int as libc::c_double;
                            if (*results.offset(i___0 as isize)).speed
                                > 0 as libc::c_long
                            {
                                correct += 1;
                            }
                        }
                    }
                }
                _ => {}
            }
            i___0 += 1;
        }
        axel_sleep(delay);
    }
    return correct;
}
unsafe extern "C" fn search_speedtest(mut r: *mut libc::c_void) -> *mut libc::c_void {
    let mut results: *mut search_t = 0 as *mut search_t;
    let mut conn: [conn_t; 1] = [conn_t {
        conf: 0 as *mut conf_t,
        proto: 0,
        port: 0,
        proxy: 0,
        host: [0; 1024],
        dir: [0; 1024],
        file: [0; 1024],
        user: [0; 1024],
        pass: [0; 1024],
        output_filename: [0; 1024],
        ftp: [ftp_t {
            cwd: [0; 1024],
            message: 0 as *mut libc::c_char,
            status: 0,
            tcp: tcp_t { fd: 0, ai_family: 0 },
            data_tcp: tcp_t { fd: 0, ai_family: 0 },
            proto: 0,
            ftp_mode: 0,
            local_if: 0 as *mut libc::c_char,
        }; 1],
        http: [http_t {
            host: [0; 1024],
            auth: [0; 1024],
            request: [abuf_t {
                p: 0 as *mut libc::c_char,
                len: 0,
            }; 1],
            headers: [abuf_t {
                p: 0 as *mut libc::c_char,
                len: 0,
            }; 1],
            port: 0,
            proto: 0,
            proxy: 0,
            proxy_auth: [0; 1024],
            firstbyte: 0,
            lastbyte: 0,
            status: 0,
            tcp: tcp_t { fd: 0, ai_family: 0 },
            local_if: 0 as *mut libc::c_char,
        }; 1],
        size: 0,
        currentbyte: 0,
        lastbyte: 0,
        tcp: 0 as *mut tcp_t,
        enabled: false,
        supported: false,
        last_transfer: 0,
        message: 0 as *mut libc::c_char,
        local_if: 0 as *mut libc::c_char,
        state: false,
        setup_thread: [0; 1],
        lock: __anonunion_pthread_mutex_t_335460617 {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_list_t {
                    __prev: 0 as *mut __pthread_internal_list,
                    __next: 0 as *mut __pthread_internal_list,
                },
            },
        },
    }; 1];
    let mut oldstate: libc::c_int = 0;
    let mut tmp: libc::c_double = 0.;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    results = r as *mut search_t;
    pthread_setcancelstate(0 as libc::c_int, &mut oldstate);
    pthread_setcanceltype(1 as libc::c_int, &mut oldstate);
    memset(
        conn.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<conn_t>() as libc::c_ulong,
    );
    conn[0 as libc::c_int as usize].conf = (*results).conf;
    tmp___0 = conn_set(
        conn.as_mut_ptr(),
        ((*results).url).as_mut_ptr() as *const libc::c_char,
    );
    if tmp___0 != 0 {
        tmp___1 = conn_init(conn.as_mut_ptr());
        if tmp___1 != 0 {
            tmp___2 = conn_info(conn.as_mut_ptr());
            if tmp___2 != 0 {
                if conn[0 as libc::c_int as usize].size == (*results).size {
                    tmp = axel_gettime();
                    (*results)
                        .speed = (1 as libc::c_int as libc::c_double
                        + 1000 as libc::c_int as libc::c_double
                            * (tmp - (*results).speed_start_time)) as off_t;
                } else {
                    (*results).speed = -(2 as libc::c_int) as off_t;
                }
            } else {
                (*results).speed = -(2 as libc::c_int) as off_t;
            }
        } else {
            (*results).speed = -(2 as libc::c_int) as off_t;
        }
    } else {
        (*results).speed = -(2 as libc::c_int) as off_t;
    }
    conn_disconnect(conn.as_mut_ptr());
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn search_sortlist(
    mut results: *mut search_t,
    mut count: libc::c_int,
) {
    qsort(
        results as *mut libc::c_void,
        count as size_t,
        ::std::mem::size_of::<search_t>() as libc::c_ulong,
        Some(
            search_sortlist_qsort
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn search_sortlist_qsort(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    if (*(a as *mut search_t)).speed < 0 as libc::c_long {
        if (*(b as *mut search_t)).speed > 0 as libc::c_long {
            return 1 as libc::c_int;
        }
    }
    if (*(a as *mut search_t)).speed > 0 as libc::c_long {
        if (*(b as *mut search_t)).speed < 0 as libc::c_long {
            return -(1 as libc::c_int);
        }
    }
    if (*(a as *mut search_t)).speed < (*(b as *mut search_t)).speed {
        return -(1 as libc::c_int)
    } else {
        return ((*(a as *mut search_t)).speed > (*(b as *mut search_t)).speed)
            as libc::c_int
    };
}
pub unsafe extern "C" fn is_ipv6_addr(mut hostname: *const libc::c_char) -> libc::c_int {
    let mut buf: [libc::c_char; 16] = [0; 16];
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    if !hostname.is_null() {
        tmp = inet_pton(
            10 as libc::c_int,
            hostname,
            buf.as_mut_ptr() as *mut libc::c_void,
        );
        if 1 as libc::c_int == tmp {
            tmp___0 = 1 as libc::c_int;
        } else {
            tmp___0 = 0 as libc::c_int;
        }
    } else {
        tmp___0 = 0 as libc::c_int;
    }
    return tmp___0;
}
#[inline]
unsafe extern "C" fn tcp_error(
    mut hostname: *mut libc::c_char,
    mut port: libc::c_int,
    mut reason: *const libc::c_char,
) {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = dcgettext(
        0 as *mut libc::c_void as *const libc::c_char,
        b"Unable to connect to server %s:%i: %s\n\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    );
    fprintf(stderr, tmp as *const libc::c_char, hostname, port, reason);
}
pub unsafe extern "C" fn tcp_connect(
    mut tcp: *mut tcp_t,
    mut hostname: *mut libc::c_char,
    mut port: libc::c_int,
    mut secure: libc::c_int,
    mut local_if: *mut libc::c_char,
    mut io_timeout: libc::c_uint,
) -> libc::c_int {
    let mut local_addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut portstr: [libc::c_char; 10] = [0; 10];
    let mut ai_hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut gai_results: *mut addrinfo = 0 as *mut addrinfo;
    let mut gai_result: *mut addrinfo = 0 as *mut addrinfo;
    let mut ret: libc::c_int = 0;
    let mut sock_fd: libc::c_int = 0;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tcp_fastopen: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut fdset: fd_set = fd_set { fds_bits: [0; 16] };
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let mut tout: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tout___0: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    sock_fd = -(1 as libc::c_int);
    memset(
        &mut local_addr as *mut sockaddr_in as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
    );
    if !local_if.is_null() {
        if *local_if == 0 {
            local_if = 0 as *mut libc::c_void as *mut libc::c_char;
        } else if (*tcp).ai_family as libc::c_int != 2 as libc::c_int {
            local_if = 0 as *mut libc::c_void as *mut libc::c_char;
        } else {
            local_addr.sin_family = 2 as libc::c_int as sa_family_t;
            local_addr.sin_port = 0 as libc::c_int as in_port_t;
            local_addr.sin_addr.s_addr = inet_addr(local_if as *const libc::c_char);
        }
    }
    snprintf(
        portstr.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
        b"%d\0" as *const u8 as *const libc::c_char,
        port,
    );
    memset(
        &mut ai_hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    ai_hints.ai_family = (*tcp).ai_family as libc::c_int;
    ai_hints.ai_socktype = 1 as libc::c_int;
    ai_hints.ai_flags = 32 as libc::c_int;
    ai_hints.ai_protocol = 0 as libc::c_int;
    ret = getaddrinfo(
        hostname as *const libc::c_char,
        portstr.as_mut_ptr() as *const libc::c_char,
        &mut ai_hints as *mut addrinfo as *const addrinfo,
        &mut gai_results as *mut *mut addrinfo,
    );
    if ret != 0 as libc::c_int {
        tmp = gai_strerror(ret);
        tcp_error(hostname, port, tmp);
        return -(1 as libc::c_int);
    }
    gai_result = gai_results;
    loop {
        tcp_fastopen = -(1 as libc::c_int);
        if sock_fd != -(1 as libc::c_int) {
            close(sock_fd);
            sock_fd = -(1 as libc::c_int);
        }
        sock_fd = socket(
            (*gai_result).ai_family,
            (*gai_result).ai_socktype,
            (*gai_result).ai_protocol,
        );
        if !(sock_fd == -(1 as libc::c_int)) {
            if !local_if.is_null() {
                if (*gai_result).ai_family == 2 as libc::c_int {
                    bind(
                        sock_fd,
                        &mut local_addr as *mut sockaddr_in as *mut sockaddr
                            as *const sockaddr,
                        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
                            as socklen_t,
                    );
                }
            }
            tcp_fastopen = setsockopt(
                sock_fd,
                6 as libc::c_int,
                30 as libc::c_int,
                0 as *mut libc::c_void as *const libc::c_void,
                0 as libc::c_int as socklen_t,
            );
            ret = connect(
                sock_fd,
                (*gai_result).ai_addr as *const sockaddr,
                (*gai_result).ai_addrlen,
            );
            if ret != -(1 as libc::c_int) {
                break;
            }
            tmp___0 = __errno_location();
            if !(*tmp___0 != 115 as libc::c_int) {
                if tcp_fastopen != -(1 as libc::c_int) {
                    break;
                }
                let fresh14 = &mut __d0;
                let fresh15;
                let fresh16 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
                let fresh17 = &mut __d1;
                let fresh18;
                let fresh19 = &mut *(fdset.fds_bits)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as *mut __fd_mask;
                asm!(
                    "cld; rep; stosq", inlateout("cx")
                    c2rust_asm_casts::AsmCast::cast_in(fresh14, fresh16) => fresh15,
                    inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh17, fresh19)
                    => fresh18, inlateout("ax") 0 as libc::c_int => _,
                    options(preserves_flags, att_syntax)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh14, fresh16, fresh15);
                c2rust_asm_casts::AsmCast::cast_out(fresh17, fresh19, fresh18);
                fdset
                    .fds_bits[(sock_fd
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    |= ((1 as libc::c_ulong)
                        << sock_fd
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask;
                tout.tv_sec = io_timeout as __time_t;
                tout.tv_usec = 0 as libc::c_long;
                ret = select(
                    sock_fd + 1 as libc::c_int,
                    0 as *mut libc::c_void as *mut fd_set,
                    &mut fdset as *mut fd_set,
                    0 as *mut libc::c_void as *mut fd_set,
                    &mut tout as *mut timeval,
                );
                if ret != -(1 as libc::c_int) {
                    break;
                }
            }
        }
        gai_result = (*gai_result).ai_next;
        if gai_result.is_null() {
            break;
        }
    }
    freeaddrinfo(gai_results);
    if sock_fd == -(1 as libc::c_int) {
        tmp___1 = __errno_location();
        tmp___2 = strerror(*tmp___1);
        tcp_error(hostname, port, tmp___2 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    fcntl(sock_fd, 4 as libc::c_int, 0 as libc::c_int);
    (*tcp).fd = sock_fd;
    tout___0.tv_sec = io_timeout as __time_t;
    tout___0.tv_usec = 0 as libc::c_long;
    setsockopt(
        sock_fd,
        1 as libc::c_int,
        20 as libc::c_int,
        &mut tout___0 as *mut timeval as *const libc::c_void,
        ::std::mem::size_of::<timeval>() as libc::c_ulong as socklen_t,
    );
    setsockopt(
        sock_fd,
        1 as libc::c_int,
        21 as libc::c_int,
        &mut tout___0 as *mut timeval as *const libc::c_void,
        ::std::mem::size_of::<timeval>() as libc::c_ulong as socklen_t,
    );
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn tcp_read(
    mut tcp: *mut tcp_t,
    mut buffer___0: *mut libc::c_void,
    mut size: libc::c_int,
) -> ssize_t {
    let mut tmp: ssize_t = 0;
    tmp = read((*tcp).fd, buffer___0, size as size_t);
    return tmp;
}
pub unsafe extern "C" fn tcp_write(
    mut tcp: *mut tcp_t,
    mut buffer___0: *mut libc::c_void,
    mut size: libc::c_int,
) -> ssize_t {
    let mut tmp: ssize_t = 0;
    tmp = write((*tcp).fd, buffer___0 as *const libc::c_void, size as size_t);
    return tmp;
}
pub unsafe extern "C" fn tcp_close(mut tcp: *mut tcp_t) {
    if (*tcp).fd > 0 as libc::c_int {
        close((*tcp).fd);
        (*tcp).fd = -(1 as libc::c_int);
    }
}
pub unsafe extern "C" fn get_if_ip(
    mut dst: *mut libc::c_char,
    mut len: size_t,
    mut iface: *const libc::c_char,
) -> libc::c_int {
    let mut ifr: ifreq = ifreq {
        ifr_ifrn: __anonunion_ifr_ifrn_352126815 {
            ifrn_name: [0; 16],
        },
        ifr_ifru: __anonunion_ifr_ifru_537349870 {
            ifru_addr: sockaddr {
                sa_family: 0,
                sa_data: [0; 14],
            },
        },
    };
    let mut ret: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut x: *mut sockaddr_in = 0 as *mut sockaddr_in;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = socket(2 as libc::c_int, 2 as libc::c_int, 0 as libc::c_int);
    fd = tmp;
    if fd < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    memset(
        &mut ifr as *mut ifreq as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<ifreq>() as libc::c_ulong,
    );
    strlcpy(
        (ifr.ifr_ifrn.ifrn_name).as_mut_ptr(),
        iface,
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
    );
    ifr.ifr_ifru.ifru_addr.sa_family = 2 as libc::c_int as sa_family_t;
    tmp___0 = ioctl(fd, 35093 as libc::c_ulong, &mut ifr as *mut ifreq);
    if tmp___0 != 0 {
        tmp___1 = 0 as libc::c_int;
    } else {
        tmp___1 = 1 as libc::c_int;
    }
    ret = tmp___1;
    if ret != 0 {
        x = &mut ifr.ifr_ifru.ifru_addr as *mut sockaddr as *mut sockaddr_in;
        tmp___2 = inet_ntoa((*x).sin_addr);
        strlcpy(dst, tmp___2 as *const libc::c_char, len);
    }
    close(fd);
    return ret;
}
pub static mut run: libc::c_int = 1 as libc::c_int;
static mut axel_options: [option; 20] = [
    {
        let mut init = option {
            name: b"max-speed\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"num-connections\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"max-redirect\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 256 as libc::c_int,
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
            name: b"search\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'S' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ipv4\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: '4' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ipv6\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: '6' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-proxy\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'N' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"quiet\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'q' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"verbose\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
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
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'V' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"alternate\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'a' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"percentage\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"insecure\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'k' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-clobber\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"header\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'H' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"user-agent\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'U' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"timeout\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'T' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const libc::c_void as *mut libc::c_void as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn calc_percentage(mut cur: off_t, mut total: off_t) -> libc::c_uint {
    let mut __a: libc::c_int = 0;
    let mut __b: off_t = 0;
    let mut tmp: off_t = 0;
    __a = 100 as libc::c_int;
    __b = (100 as libc::c_long * cur + total / 2 as libc::c_long) / total;
    if (__a as off_t) < __b {
        tmp = __a as off_t;
    } else {
        tmp = __b;
    }
    return tmp as libc::c_uint;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut fn_0: [libc::c_char; 1024] = [0; 1024];
    let mut do_search: libc::c_int = 0;
    let mut search: *mut search_t = 0 as *mut search_t;
    let mut conf: [conf_t; 1] = [conf_t {
        default_filename: [0; 1024],
        http_proxy: [0; 1024],
        no_proxy: [0; 1024],
        num_connections: 0,
        strip_cgi_parameters: 0,
        save_state_interval: 0,
        connection_timeout: 0,
        reconnect_delay: 0,
        max_redirect: 0,
        buffer_size: 0,
        max_speed: 0,
        verbose: 0,
        alternate_output: 0,
        insecure: 0,
        no_clobber: 0,
        percentage: 0,
        interfaces: 0 as *mut if_t,
        ai_family: 0,
        search_timeout: 0,
        search_threads: 0,
        search_amount: 0,
        search_top: 0,
        io_timeout: 0,
        add_header_count: 0,
        add_header: [[0; 1024]; 10],
    }; 1];
    let mut axel: *mut axel_t = 0 as *mut axel_t;
    let mut j: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut option: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_ulong = 0;
    let mut tmp___10: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___11: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___12: libc::c_int = 0;
    let mut tmp___13: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___14: size_t = 0;
    let mut tmp___15: libc::c_int = 0;
    let mut tmp___16: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___17: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___18: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut tmp___19: libc::c_int = 0;
    let mut tmp___20: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___21: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___22: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___23: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut __a: libc::c_int = 0;
    let mut __b: libc::c_int = 0;
    let mut tmp___24: libc::c_int = 0;
    let mut tmp___25: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___26: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i___0: libc::c_int = 0;
    let mut buf: stat = stat {
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
    let mut fnlen: size_t = 0;
    let mut tmp___27: size_t = 0;
    let mut axelfnlen: size_t = 0;
    let mut tmp___28: size_t = 0;
    let mut tmp___29: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___30: libc::c_int = 0;
    let mut statefn: [libc::c_char; 1027] = [0; 1027];
    let mut tmp___31: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___32: libc::c_int = 0;
    let mut tmp___33: libc::c_int = 0;
    let mut tmp___34: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___35: libc::c_int = 0;
    let mut tmp___36: libc::c_int = 0;
    let mut tmp___37: size_t = 0;
    let mut i___1: libc::c_int = 0;
    let mut statefn___0: [libc::c_char; 1027] = [0; 1027];
    let mut f_exists: libc::c_int = 0;
    let mut tmp___38: libc::c_int = 0;
    let mut tmp___39: libc::c_int = 0;
    let mut st_exists: libc::c_int = 0;
    let mut tmp___40: libc::c_int = 0;
    let mut tmp___41: libc::c_int = 0;
    let mut tmp___42: libc::c_int = 0;
    let mut prev: off_t = 0;
    let mut tmp___43: libc::c_uint = 0;
    let mut hsize: [libc::c_char; 512] = [0; 512];
    let mut htime: [libc::c_char; 512] = [0; 512];
    let mut tmp___44: libc::c_double = 0.;
    let mut tmp___45: *mut libc::c_char = 0 as *mut libc::c_char;
    do_search = 0 as libc::c_int;
    ret = 1 as libc::c_int;
    fn_0[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"axel\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"axel\0" as *const u8 as *const libc::c_char);
    tmp = conf_init(conf.as_mut_ptr());
    if tmp == 0 {
        return 1 as libc::c_int;
    }
    opterr = 0 as libc::c_int;
    j = -(1 as libc::c_int);
    loop {
        tmp___0 = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            b"s:n:o:S::46NqvhVapkcH:U:T:\0" as *const u8 as *const libc::c_char,
            axel_options.as_mut_ptr() as *const option,
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        option = tmp___0;
        if option == -(1 as libc::c_int) {
            current_block = 178030534879405462;
            break;
        }
        match option {
            85 => {
                conf_hdr_make(
                    (conf[0 as libc::c_int as usize]
                        .add_header[0 as libc::c_int as usize])
                        .as_mut_ptr(),
                    b"User-Agent\0" as *const u8 as *const libc::c_char,
                    optarg as *const libc::c_char,
                );
            }
            72 => {
                if !(conf[0 as libc::c_int as usize].add_header_count
                    < 10 as libc::c_int)
                {
                    tmp___1 = dcgettext(
                        0 as *mut libc::c_void as *const libc::c_char,
                        b"Too many custom headers (-H)! Currently only %u custom headers can be appended.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    );
                    fprintf(stderr, tmp___1 as *const libc::c_char, 9 as libc::c_int);
                    current_block = 6213160851590473481;
                    break;
                } else {
                    tmp___2 = conf[0 as libc::c_int as usize].add_header_count;
                    conf[0 as libc::c_int as usize].add_header_count += 1;
                    strlcpy(
                        (conf[0 as libc::c_int as usize].add_header[tmp___2 as usize])
                            .as_mut_ptr(),
                        optarg as *const libc::c_char,
                        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                    );
                }
            }
            115 => {
                tmp___3 = sscanf(
                    optarg as *const libc::c_char,
                    b"%llu\0" as *const u8 as *const libc::c_char,
                    &mut (*conf.as_mut_ptr().offset(0 as libc::c_int as isize)).max_speed
                        as *mut libc::c_ulonglong,
                );
                if !(tmp___3 == 0) {
                    continue;
                }
                print_help();
                current_block = 6213160851590473481;
                break;
            }
            110 => {
                tmp___4 = sscanf(
                    optarg as *const libc::c_char,
                    b"%hu\0" as *const u8 as *const libc::c_char,
                    &mut (*conf.as_mut_ptr().offset(0 as libc::c_int as isize))
                        .num_connections as *mut uint16_t,
                );
                if !(tmp___4 == 0) {
                    continue;
                }
                print_help();
                current_block = 6213160851590473481;
                break;
            }
            256 => {
                tmp___5 = sscanf(
                    optarg as *const libc::c_char,
                    b"%i\0" as *const u8 as *const libc::c_char,
                    &mut (*conf.as_mut_ptr().offset(0 as libc::c_int as isize))
                        .max_redirect as *mut libc::c_int,
                );
                if tmp___5 == 0 {
                    print_help();
                    return 1 as libc::c_int;
                }
            }
            111 => {
                strlcpy(
                    fn_0.as_mut_ptr(),
                    optarg as *const libc::c_char,
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                );
            }
            83 => {
                do_search = 1 as libc::c_int;
                if optarg.is_null() {
                    continue;
                }
                tmp___6 = sscanf(
                    optarg as *const libc::c_char,
                    b"%i\0" as *const u8 as *const libc::c_char,
                    &mut (*conf.as_mut_ptr().offset(0 as libc::c_int as isize))
                        .search_top as *mut libc::c_int,
                );
                if !(tmp___6 == 0) {
                    continue;
                }
                print_help();
                current_block = 6213160851590473481;
                break;
            }
            54 => {
                conf[0 as libc::c_int as usize]
                    .ai_family = 10 as libc::c_int as sa_family_t;
            }
            52 => {
                conf[0 as libc::c_int as usize]
                    .ai_family = 2 as libc::c_int as sa_family_t;
            }
            97 => {
                conf[0 as libc::c_int as usize].alternate_output = 1 as libc::c_int;
            }
            112 => {
                conf[0 as libc::c_int as usize].percentage = 1 as libc::c_int;
            }
            107 => {
                conf[0 as libc::c_int as usize].insecure = 1 as libc::c_int;
            }
            99 => {
                conf[0 as libc::c_int as usize].no_clobber = 1 as libc::c_int;
            }
            78 => {
                conf[0 as libc::c_int as usize]
                    .http_proxy[0 as libc::c_int
                    as usize] = 0 as libc::c_int as libc::c_char;
            }
            104 => {
                print_help();
                ret = 0 as libc::c_int;
                current_block = 6213160851590473481;
                break;
            }
            118 => {
                if j == -(1 as libc::c_int) {
                    j = 1 as libc::c_int;
                } else {
                    j += 1;
                }
            }
            86 => {
                print_version_info();
                ret = 0 as libc::c_int;
                current_block = 6213160851590473481;
                break;
            }
            113 => {
                close(1 as libc::c_int);
                conf[0 as libc::c_int as usize].verbose = -(1 as libc::c_int);
                tmp___8 = open(
                    b"/dev/null\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                );
                if !(tmp___8 != 1 as libc::c_int) {
                    continue;
                }
                tmp___7 = dcgettext(
                    0 as *mut libc::c_void as *const libc::c_char,
                    b"Can't redirect stdout to /dev/null.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                );
                fprintf(stderr, tmp___7 as *const libc::c_char);
                current_block = 6213160851590473481;
                break;
            }
            84 => {
                tmp___9 = strtoul(
                    optarg as *const libc::c_char,
                    0 as *mut libc::c_void as *mut *mut libc::c_char,
                    0 as libc::c_int,
                );
                conf[0 as libc::c_int as usize].io_timeout = tmp___9 as libc::c_uint;
            }
            _ => {
                print_help();
                current_block = 6213160851590473481;
                break;
            }
        }
    }
    match current_block {
        178030534879405462 => {
            if conf[0 as libc::c_int as usize].verbose < 0 as libc::c_int {
                conf[0 as libc::c_int as usize].alternate_output = 0 as libc::c_int;
                conf[0 as libc::c_int as usize].percentage = 0 as libc::c_int;
            } else if j > -(1 as libc::c_int) {
                conf[0 as libc::c_int as usize].verbose = j;
            }
            if (conf[0 as libc::c_int as usize].num_connections as libc::c_int)
                < 1 as libc::c_int
            {
                print_help();
            } else {
                if conf[0 as libc::c_int as usize].max_redirect < 0 as libc::c_int {
                    print_help();
                    return 1 as libc::c_int;
                }
                if argc - optind == 0 as libc::c_int {
                    print_help();
                } else {
                    tmp___15 = strcmp(
                        *argv.offset(optind as isize) as *const libc::c_char,
                        b"-\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___15 == 0 as libc::c_int {
                        tmp___10 = malloc(1024 as libc::c_int as size_t);
                        s = tmp___10 as *mut libc::c_char;
                        if s.is_null() {
                            current_block = 6213160851590473481;
                        } else {
                            tmp___12 = scanf(
                                b"%1024[^\n]s\0" as *const u8 as *const libc::c_char,
                                s,
                            );
                            if tmp___12 != 1 as libc::c_int {
                                tmp___11 = dcgettext(
                                    0 as *mut libc::c_void as *const libc::c_char,
                                    b"Error when trying to read URL (Too long?).\n\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                );
                                fprintf(stderr, tmp___11 as *const libc::c_char);
                                free(s as *mut libc::c_void);
                                current_block = 6213160851590473481;
                            } else {
                                current_block = 14027225908442187354;
                            }
                        }
                    } else {
                        s = *argv.offset(optind as isize);
                        tmp___14 = strlen(s as *const libc::c_char);
                        if tmp___14 > 1024 as libc::c_ulong {
                            tmp___13 = dcgettext(
                                0 as *mut libc::c_void as *const libc::c_char,
                                b"Can't handle URLs of length over %zu\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            );
                            fprintf(
                                stderr,
                                tmp___13 as *const libc::c_char,
                                1024 as libc::c_int as size_t,
                            );
                            current_block = 6213160851590473481;
                        } else {
                            current_block = 14027225908442187354;
                        }
                    }
                    match current_block {
                        6213160851590473481 => {}
                        _ => {
                            if conf[0 as libc::c_int as usize].percentage == 0 {
                                tmp___16 = dcgettext(
                                    0 as *mut libc::c_void as *const libc::c_char,
                                    b"Initializing download: %s\n\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                );
                                printf(tmp___16 as *const libc::c_char, s);
                            }
                            if do_search != 0 {
                                tmp___17 = calloc(
                                    (conf[0 as libc::c_int as usize].search_amount
                                        + 1 as libc::c_int) as size_t,
                                    ::std::mem::size_of::<search_t>() as libc::c_ulong,
                                );
                                search = tmp___17 as *mut search_t;
                                if search.is_null() {
                                    current_block = 6213160851590473481;
                                } else {
                                    let ref mut fresh20 = (*search
                                        .offset(0 as libc::c_int as isize))
                                        .conf;
                                    *fresh20 = conf.as_mut_ptr();
                                    if conf[0 as libc::c_int as usize].verbose != 0 {
                                        tmp___18 = dcgettext(
                                            0 as *mut libc::c_void as *const libc::c_char,
                                            b"Doing search...\n\0" as *const u8 as *const libc::c_char,
                                            5 as libc::c_int,
                                        );
                                        printf(tmp___18 as *const libc::c_char);
                                    }
                                    tmp___19 = search_makelist(search, s);
                                    i = tmp___19;
                                    if i < 0 as libc::c_int {
                                        tmp___20 = dcgettext(
                                            0 as *mut libc::c_void as *const libc::c_char,
                                            b"File not found\n\0" as *const u8 as *const libc::c_char,
                                            5 as libc::c_int,
                                        );
                                        fprintf(stderr, tmp___20 as *const libc::c_char);
                                        current_block = 6213160851590473481;
                                    } else {
                                        if conf[0 as libc::c_int as usize].verbose != 0 {
                                            tmp___21 = dcgettext(
                                                0 as *mut libc::c_void as *const libc::c_char,
                                                b"Testing speeds, this can take a while...\n\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            );
                                            printf(tmp___21 as *const libc::c_char);
                                        }
                                        j = search_getspeeds(search, i);
                                        if j < 0 as libc::c_int {
                                            tmp___22 = dcgettext(
                                                0 as *mut libc::c_void as *const libc::c_char,
                                                b"Speed testing failed\n\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            );
                                            fprintf(stderr, tmp___22 as *const libc::c_char);
                                            return 1 as libc::c_int;
                                        }
                                        search_sortlist(search, i);
                                        if conf[0 as libc::c_int as usize].verbose != 0 {
                                            tmp___23 = dcgettext(
                                                0 as *mut libc::c_void as *const libc::c_char,
                                                b"%i usable servers found, will use these URLs:\n\0"
                                                    as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            );
                                            printf(tmp___23 as *const libc::c_char, j);
                                            __a = j;
                                            __b = conf[0 as libc::c_int as usize].search_top;
                                            if __a < __b {
                                                tmp___24 = __a;
                                            } else {
                                                tmp___24 = __b;
                                            }
                                            j = tmp___24;
                                            tmp___25 = dcgettext(
                                                0 as *mut libc::c_void as *const libc::c_char,
                                                b"Speed\0" as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            );
                                            printf(
                                                b"%-60s %15s\n\0" as *const u8 as *const libc::c_char,
                                                b"URL\0" as *const u8 as *const libc::c_char,
                                                tmp___25,
                                            );
                                            i = 0 as libc::c_int;
                                            while i < j {
                                                printf(
                                                    b"%-70.70s %5jd\n\0" as *const u8 as *const libc::c_char,
                                                    ((*search.offset(i as isize)).url).as_mut_ptr(),
                                                    (*search.offset(i as isize)).speed,
                                                );
                                                i += 1;
                                            }
                                            printf(b"\n\0" as *const u8 as *const libc::c_char);
                                        }
                                        axel = axel_new(
                                            conf.as_mut_ptr(),
                                            j,
                                            search as *const search_t,
                                        );
                                        free(search as *mut libc::c_void);
                                        if axel.is_null() {
                                            print_messages(axel);
                                            current_block = 9791982079364390377;
                                        } else if (*axel).ready == -(1 as libc::c_int) {
                                            print_messages(axel);
                                            current_block = 9791982079364390377;
                                        } else {
                                            current_block = 8466485602140941539;
                                        }
                                    }
                                }
                            } else {
                                tmp___26 = calloc(
                                    (argc - optind) as size_t,
                                    ::std::mem::size_of::<search_t>() as libc::c_ulong,
                                );
                                search = tmp___26 as *mut search_t;
                                if search.is_null() {
                                    current_block = 6213160851590473481;
                                } else {
                                    i___0 = 0 as libc::c_int;
                                    while i___0 < argc - optind {
                                        strlcpy(
                                            ((*search.offset(i___0 as isize)).url).as_mut_ptr(),
                                            *argv.offset((optind + i___0) as isize)
                                                as *const libc::c_char,
                                            ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                as libc::c_ulong,
                                        );
                                        i___0 += 1;
                                    }
                                    axel = axel_new(
                                        conf.as_mut_ptr(),
                                        argc - optind,
                                        search as *const search_t,
                                    );
                                    free(search as *mut libc::c_void);
                                    if axel.is_null() {
                                        print_messages(axel);
                                        current_block = 9791982079364390377;
                                    } else if (*axel).ready == -(1 as libc::c_int) {
                                        print_messages(axel);
                                        current_block = 9791982079364390377;
                                    } else {
                                        current_block = 8466485602140941539;
                                    }
                                }
                            }
                            match current_block {
                                6213160851590473481 => {}
                                _ => {
                                    match current_block {
                                        8466485602140941539 => {
                                            print_messages(axel);
                                            if s as libc::c_ulong
                                                != *argv.offset(optind as isize) as libc::c_ulong
                                            {
                                                free(s as *mut libc::c_void);
                                            }
                                            if fn_0[0 as libc::c_int as usize] != 0 {
                                                tmp___30 = stat(
                                                    fn_0.as_mut_ptr() as *const libc::c_char,
                                                    &mut buf as *mut stat,
                                                );
                                                if tmp___30 == 0 as libc::c_int {
                                                    if buf.st_mode & 61440 as libc::c_uint
                                                        == 16384 as libc::c_uint
                                                    {
                                                        tmp___27 = strlen(fn_0.as_mut_ptr() as *const libc::c_char);
                                                        fnlen = tmp___27;
                                                        tmp___28 = strlen(
                                                            ((*axel).filename).as_mut_ptr() as *const libc::c_char,
                                                        );
                                                        axelfnlen = tmp___28;
                                                        if fnlen
                                                            .wrapping_add(1 as libc::c_ulong)
                                                            .wrapping_add(axelfnlen)
                                                            .wrapping_add(1 as libc::c_ulong) > 1024 as libc::c_ulong
                                                        {
                                                            tmp___29 = dcgettext(
                                                                0 as *mut libc::c_void as *const libc::c_char,
                                                                b"Filename too long!\n\0" as *const u8
                                                                    as *const libc::c_char,
                                                                5 as libc::c_int,
                                                            );
                                                            fprintf(stderr, tmp___29 as *const libc::c_char);
                                                            current_block = 9791982079364390377;
                                                        } else {
                                                            fn_0[fnlen as usize] = '/' as i32 as libc::c_char;
                                                            memcpy(
                                                                fn_0
                                                                    .as_mut_ptr()
                                                                    .offset(fnlen as isize)
                                                                    .offset(1 as libc::c_int as isize) as *mut libc::c_void,
                                                                ((*axel).filename).as_mut_ptr() as *const libc::c_void,
                                                                axelfnlen,
                                                            );
                                                            fn_0[fnlen
                                                                .wrapping_add(1 as libc::c_ulong)
                                                                .wrapping_add(axelfnlen)
                                                                as usize] = '\u{0}' as i32 as libc::c_char;
                                                            current_block = 2920409193602730479;
                                                        }
                                                    } else {
                                                        current_block = 2920409193602730479;
                                                    }
                                                } else {
                                                    current_block = 2920409193602730479;
                                                }
                                                match current_block {
                                                    9791982079364390377 => {}
                                                    _ => {
                                                        snprintf(
                                                            statefn.as_mut_ptr(),
                                                            ::std::mem::size_of::<[libc::c_char; 1027]>()
                                                                as libc::c_ulong,
                                                            b"%s.st\0" as *const u8 as *const libc::c_char,
                                                            fn_0.as_mut_ptr(),
                                                        );
                                                        tmp___32 = access(
                                                            fn_0.as_mut_ptr() as *const libc::c_char,
                                                            0 as libc::c_int,
                                                        );
                                                        if tmp___32 == 0 as libc::c_int {
                                                            tmp___33 = access(
                                                                statefn.as_mut_ptr() as *const libc::c_char,
                                                                0 as libc::c_int,
                                                            );
                                                            if tmp___33 != 0 as libc::c_int {
                                                                tmp___31 = dcgettext(
                                                                    0 as *mut libc::c_void as *const libc::c_char,
                                                                    b"No state file, cannot resume!\n\0" as *const u8
                                                                        as *const libc::c_char,
                                                                    5 as libc::c_int,
                                                                );
                                                                fprintf(stderr, tmp___31 as *const libc::c_char);
                                                                current_block = 9791982079364390377;
                                                            } else {
                                                                current_block = 4466262843398566590;
                                                            }
                                                        } else {
                                                            current_block = 4466262843398566590;
                                                        }
                                                        match current_block {
                                                            9791982079364390377 => {}
                                                            _ => {
                                                                tmp___35 = access(
                                                                    statefn.as_mut_ptr() as *const libc::c_char,
                                                                    0 as libc::c_int,
                                                                );
                                                                if tmp___35 == 0 as libc::c_int {
                                                                    tmp___36 = access(
                                                                        fn_0.as_mut_ptr() as *const libc::c_char,
                                                                        0 as libc::c_int,
                                                                    );
                                                                    if tmp___36 != 0 as libc::c_int {
                                                                        tmp___34 = dcgettext(
                                                                            0 as *mut libc::c_void as *const libc::c_char,
                                                                            b"State file found, but no downloaded data. Starting from scratch.\n\0"
                                                                                as *const u8 as *const libc::c_char,
                                                                            5 as libc::c_int,
                                                                        );
                                                                        printf(tmp___34 as *const libc::c_char);
                                                                        unlink(statefn.as_mut_ptr() as *const libc::c_char);
                                                                    }
                                                                }
                                                                strlcpy(
                                                                    ((*axel).filename).as_mut_ptr(),
                                                                    fn_0.as_mut_ptr() as *const libc::c_char,
                                                                    ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                                        as libc::c_ulong,
                                                                );
                                                                current_block = 12691661418490471423;
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                tmp___37 = strlen(
                                                    ((*axel).filename).as_mut_ptr() as *const libc::c_char,
                                                );
                                                s = ((*axel).filename)
                                                    .as_mut_ptr()
                                                    .offset(tmp___37 as isize);
                                                i___1 = 0 as libc::c_int;
                                                loop {
                                                    snprintf(
                                                        statefn___0.as_mut_ptr(),
                                                        ::std::mem::size_of::<[libc::c_char; 1027]>()
                                                            as libc::c_ulong,
                                                        b"%s.st\0" as *const u8 as *const libc::c_char,
                                                        ((*axel).filename).as_mut_ptr(),
                                                    );
                                                    tmp___38 = access(
                                                        ((*axel).filename).as_mut_ptr() as *const libc::c_char,
                                                        0 as libc::c_int,
                                                    );
                                                    if tmp___38 != 0 {
                                                        tmp___39 = 0 as libc::c_int;
                                                    } else {
                                                        tmp___39 = 1 as libc::c_int;
                                                    }
                                                    f_exists = tmp___39;
                                                    tmp___40 = access(
                                                        statefn___0.as_mut_ptr() as *const libc::c_char,
                                                        0 as libc::c_int,
                                                    );
                                                    if tmp___40 != 0 {
                                                        tmp___41 = 0 as libc::c_int;
                                                    } else {
                                                        tmp___41 = 1 as libc::c_int;
                                                    }
                                                    st_exists = tmp___41;
                                                    if f_exists != 0 {
                                                        if (*((*axel).conn).offset(0 as libc::c_int as isize))
                                                            .supported
                                                        {
                                                            if st_exists != 0 {
                                                                break;
                                                            }
                                                        }
                                                    } else if st_exists == 0 {
                                                        break;
                                                    }
                                                    snprintf(
                                                        s,
                                                        ((*axel).filename)
                                                            .as_mut_ptr()
                                                            .offset(
                                                                ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                                    as libc::c_ulong as isize,
                                                            )
                                                            .offset_from(s) as libc::c_long as size_t,
                                                        b".%i\0" as *const u8 as *const libc::c_char,
                                                        i___1,
                                                    );
                                                    i___1 += 1;
                                                }
                                                current_block = 12691661418490471423;
                                            }
                                            match current_block {
                                                9791982079364390377 => {}
                                                _ => {
                                                    tmp___42 = axel_open(axel);
                                                    if tmp___42 == 0 {
                                                        print_messages(axel);
                                                    } else {
                                                        print_messages(axel);
                                                        axel_start(axel);
                                                        print_messages(axel);
                                                        if conf[0 as libc::c_int as usize].alternate_output != 0 {
                                                            putchar('\n' as i32);
                                                        } else if conf[0 as libc::c_int as usize].percentage != 0 {
                                                            putchar('\n' as i32);
                                                        } else if (*axel).bytes_done > 0 as libc::c_long {
                                                            putchar('\n' as i32);
                                                            print_commas((*axel).bytes_done);
                                                            fflush(stdout);
                                                        }
                                                        (*axel).start_byte = (*axel).bytes_done;
                                                        signal(
                                                            2 as libc::c_int,
                                                            Some(stop as unsafe extern "C" fn(libc::c_int) -> ()),
                                                        );
                                                        signal(
                                                            15 as libc::c_int,
                                                            Some(stop as unsafe extern "C" fn(libc::c_int) -> ()),
                                                        );
                                                        while (*axel).ready == 0 {
                                                            if run == 0 {
                                                                break;
                                                            }
                                                            prev = (*axel).bytes_done;
                                                            axel_do(axel);
                                                            if conf[0 as libc::c_int as usize].percentage != 0 {
                                                                if ((*axel).message).is_null() {
                                                                    if prev != (*axel).bytes_done {
                                                                        tmp___43 = calc_percentage(
                                                                            (*axel).bytes_done,
                                                                            (*axel).size,
                                                                        );
                                                                        printf(
                                                                            b"%u\n\0" as *const u8 as *const libc::c_char,
                                                                            tmp___43,
                                                                        );
                                                                    }
                                                                }
                                                            } else if conf[0 as libc::c_int as usize].alternate_output
                                                                    != 0
                                                                {
                                                                if ((*axel).message).is_null() {
                                                                    if prev != (*axel).bytes_done {
                                                                        print_alternate_output(axel);
                                                                    }
                                                                }
                                                            } else if conf[0 as libc::c_int as usize].verbose
                                                                    > -(1 as libc::c_int)
                                                                {
                                                                print_progress(
                                                                    (*axel).bytes_done,
                                                                    prev,
                                                                    (*axel).size,
                                                                    (*axel).bytes_per_second as libc::c_double
                                                                        / 1024 as libc::c_int as libc::c_double,
                                                                );
                                                            }
                                                            if !((*axel).message).is_null() {
                                                                if conf[0 as libc::c_int as usize].alternate_output
                                                                    == 1 as libc::c_int
                                                                {
                                                                    fputs(
                                                                        b"\x1B[2K\r\0" as *const u8 as *const libc::c_char,
                                                                        stdout,
                                                                    );
                                                                } else {
                                                                    putchar('\n' as i32);
                                                                }
                                                                print_messages(axel);
                                                                if (*axel).ready == 0 {
                                                                    if conf[0 as libc::c_int as usize].alternate_output
                                                                        != 1 as libc::c_int
                                                                    {
                                                                        print_commas((*axel).bytes_done);
                                                                    } else {
                                                                        print_alternate_output(axel);
                                                                    }
                                                                }
                                                            } else if (*axel).ready != 0 {
                                                                putchar('\n' as i32);
                                                            }
                                                            fflush(stdout);
                                                        }
                                                        tmp___44 = axel_gettime();
                                                        time_human(
                                                            htime.as_mut_ptr(),
                                                            ::std::mem::size_of::<[libc::c_char; 512]>()
                                                                as libc::c_ulong,
                                                            (tmp___44 - (*axel).start_time) as libc::c_uint,
                                                        );
                                                        axel_size_human(
                                                            hsize.as_mut_ptr(),
                                                            ::std::mem::size_of::<[libc::c_char; 512]>()
                                                                as libc::c_ulong,
                                                            ((*axel).bytes_done - (*axel).start_byte) as size_t,
                                                        );
                                                        tmp___45 = dcgettext(
                                                            0 as *mut libc::c_void as *const libc::c_char,
                                                            b"\nDownloaded %s in %s. (%.2f KB/s)\n\0" as *const u8
                                                                as *const libc::c_char,
                                                            5 as libc::c_int,
                                                        );
                                                        printf(
                                                            tmp___45 as *const libc::c_char,
                                                            hsize.as_mut_ptr(),
                                                            htime.as_mut_ptr(),
                                                            (*axel).bytes_per_second as libc::c_double
                                                                / 1024 as libc::c_int as libc::c_double,
                                                        );
                                                        if (*axel).ready != 0 {
                                                            ret = 0 as libc::c_int;
                                                        } else {
                                                            ret = 2 as libc::c_int;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                    axel_close(axel);
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    conf_free(conf.as_mut_ptr());
    return ret;
}
unsafe extern "C" fn stop(mut signal___0: libc::c_int) {
    run = 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn log2i(mut x: libc::c_ulonglong) -> libc::c_uint {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_ulong = 0;
    if x != 0 {
        tmp = x.leading_zeros() as i32;
        tmp___0 = (::std::mem::size_of::<libc::c_ulonglong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_ulong)
            .wrapping_sub(1 as libc::c_ulong)
            .wrapping_sub(tmp as libc::c_ulong);
    } else {
        tmp___0 = 0 as libc::c_ulong;
    }
    return tmp___0 as libc::c_uint;
}
pub unsafe extern "C" fn axel_size_human(
    mut dst: *mut libc::c_char,
    mut len: size_t,
    mut value: size_t,
) -> *mut libc::c_char {
    let mut fval: libc::c_double = 0.;
    let mut oname: [*const libc::c_char; 5] = [0 as *const libc::c_char; 5];
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut order: libc::c_uint = 0;
    let mut __a: libc::c_ulong = 0;
    let mut __b: libc::c_uint = 0;
    let mut tmp___4: libc::c_uint = 0;
    let mut tmp___5: libc::c_ulong = 0;
    let mut ret: libc::c_int = 0;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: *mut libc::c_char = 0 as *mut libc::c_char;
    fval = value as libc::c_double;
    tmp = dcgettext(
        0 as *mut libc::c_void as *const libc::c_char,
        b"Kilo\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    );
    tmp___0 = dcgettext(
        0 as *mut libc::c_void as *const libc::c_char,
        b"Mega\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    );
    tmp___1 = dcgettext(
        0 as *mut libc::c_void as *const libc::c_char,
        b"Giga\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    );
    tmp___2 = dcgettext(
        0 as *mut libc::c_void as *const libc::c_char,
        b"Tera\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    );
    oname[0 as libc::c_int as usize] = b"\0" as *const u8 as *const libc::c_char;
    oname[1 as libc::c_int as usize] = tmp as *const libc::c_char;
    oname[2 as libc::c_int as usize] = tmp___0 as *const libc::c_char;
    oname[3 as libc::c_int as usize] = tmp___1 as *const libc::c_char;
    oname[4 as libc::c_int as usize] = tmp___2 as *const libc::c_char;
    __a = (::std::mem::size_of::<[*const libc::c_char; 5]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_ulong);
    tmp___4 = log2i(fval as libc::c_ulonglong);
    __b = tmp___4.wrapping_div(10 as libc::c_uint);
    if __a < __b as libc::c_ulong {
        tmp___5 = __a;
    } else {
        tmp___5 = __b as libc::c_ulong;
    }
    order = tmp___5 as libc::c_uint;
    fval
        /= ((1 as libc::c_int) << order.wrapping_mul(10 as libc::c_uint))
            as libc::c_double;
    tmp___6 = dcgettext(
        0 as *mut libc::c_void as *const libc::c_char,
        b"%g %sbyte(s)\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    );
    tmp___7 = snprintf(
        dst,
        len,
        tmp___6 as *const libc::c_char,
        fval,
        oname[order as usize],
    );
    ret = tmp___7;
    if ret < 0 as libc::c_int {
        tmp___8 = 0 as *mut libc::c_void as *mut libc::c_char;
    } else {
        tmp___8 = dst;
    }
    return tmp___8;
}
unsafe extern "C" fn time_human(
    mut dst: *mut libc::c_char,
    mut len: size_t,
    mut value: libc::c_uint,
) -> *mut libc::c_char {
    let mut hh: libc::c_uint = 0;
    let mut mm: libc::c_uint = 0;
    let mut ss: libc::c_uint = 0;
    let mut ret: libc::c_int = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    ss = value.wrapping_rem(60 as libc::c_uint);
    mm = value.wrapping_div(60 as libc::c_uint).wrapping_rem(60 as libc::c_uint);
    hh = value.wrapping_div(3600 as libc::c_uint);
    if hh != 0 {
        tmp = dcgettext(
            0 as *mut libc::c_void as *const libc::c_char,
            b"%i:%02i:%02i hour(s)\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        ret = snprintf(dst, len, tmp as *const libc::c_char, hh, mm, ss);
    } else if mm != 0 {
        tmp___0 = dcgettext(
            0 as *mut libc::c_void as *const libc::c_char,
            b"%i:%02i minute(s)\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        ret = snprintf(dst, len, tmp___0 as *const libc::c_char, mm, ss);
    } else {
        tmp___1 = dcgettext(
            0 as *mut libc::c_void as *const libc::c_char,
            b"%i second(s)\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        ret = snprintf(dst, len, tmp___1 as *const libc::c_char, ss);
    }
    if ret < 0 as libc::c_int {
        tmp___2 = 0 as *mut libc::c_void as *mut libc::c_char;
    } else {
        tmp___2 = dst;
    }
    return tmp___2;
}
unsafe extern "C" fn print_commas(mut bytes_done: off_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    printf(b"       \0" as *const u8 as *const libc::c_char);
    j = (bytes_done / 1024 as libc::c_long % 50 as libc::c_long) as libc::c_int;
    if j == 0 as libc::c_int {
        j = 50 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < j {
        if i % 10 as libc::c_int == 0 as libc::c_int {
            putchar(' ' as i32);
        }
        putchar(',' as i32);
        i += 1;
    }
}
unsafe extern "C" fn print_progress(
    mut cur: off_t,
    mut prev: off_t,
    mut total: off_t,
    mut kbps: libc::c_double,
) {
    let mut print_speed: bool = false;
    let mut i: off_t = 0;
    let mut tmp: libc::c_uint = 0;
    prev /= 1024 as libc::c_long;
    cur /= 1024 as libc::c_long;
    print_speed = prev > 0 as libc::c_long;
    i = prev;
    while i < cur {
        if i % 50 as libc::c_long == 0 as libc::c_long {
            if print_speed {
                printf(b"  [%6.1fKB/s]\0" as *const u8 as *const libc::c_char, kbps);
            }
            if total as libc::c_longlong == 9223372036854775807 as libc::c_longlong {
                printf(b"\n[ N/A]  \0" as *const u8 as *const libc::c_char);
            } else {
                tmp = calc_percentage(1024 as libc::c_long * i, total);
                printf(b"\n[%3u%%]  \0" as *const u8 as *const libc::c_char, tmp);
            }
        } else if i % 10 as libc::c_long == 0 as libc::c_long {
            putchar(' ' as i32);
        }
        putchar('.' as i32);
        i += 1;
    }
}
unsafe extern "C" fn alt_id(mut n: libc::c_int) -> libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: libc::c_int = 0;
    p = b"09AZaz\0" as *const u8 as *const libc::c_char;
    while *p != 0 {
        if !(n
            > *p.offset(1 as libc::c_int as isize) as libc::c_int
                - *p.offset(0 as libc::c_int as isize) as libc::c_int)
        {
            break;
        }
        n
            -= *p.offset(1 as libc::c_int as isize) as libc::c_int
                - *p.offset(0 as libc::c_int as isize) as libc::c_int + 1 as libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
    }
    if *p != 0 {
        tmp = *p as libc::c_int + n;
    } else {
        tmp = '*' as i32;
    }
    return tmp as libc::c_char;
}
unsafe extern "C" fn print_alternate_output_progress(
    mut axel: *mut axel_t,
    mut progress: *mut libc::c_char,
    mut width: libc::c_int,
    mut done: off_t,
    mut total: off_t,
    mut now: libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut __a: libc::c_int = 0;
    let mut __b: off_t = 0;
    let mut tmp: off_t = 0;
    let mut tmp___0: libc::c_uint = 0;
    if width == 0 {
        width = 1 as libc::c_int;
    }
    if total == 0 {
        total = 1 as libc::c_int as off_t;
    }
    i = 0 as libc::c_int;
    while i < (*(*axel).conf).num_connections as libc::c_int {
        offset = ((*((*axel).conn).offset(i as isize)).currentbyte * width as off_t
            / total) as libc::c_int;
        if (*((*axel).conn).offset(i as isize)).currentbyte
            < (*((*axel).conn).offset(i as isize)).lastbyte
        {
            if now
                <= ((*((*axel).conn).offset(i as isize)).last_transfer
                    + (*(*axel).conf).connection_timeout / 2 as libc::c_int)
                    as libc::c_double
            {
                *progress.offset(offset as isize) = alt_id(i);
            } else {
                *progress.offset(offset as isize) = '#' as i32 as libc::c_char;
            }
        }
        __a = 0 as libc::c_int;
        __b = (*((*axel).conn).offset(i as isize)).lastbyte * width as off_t / total
            - offset as off_t - 1 as libc::c_long;
        if __a as off_t > __b {
            tmp = __a as off_t;
        } else {
            tmp = __b;
        }
        memset(
            progress.offset(offset as isize).offset(1 as libc::c_int as isize)
                as *mut libc::c_void,
            ' ' as i32,
            tmp as size_t,
        );
        i += 1;
    }
    *progress.offset(width as isize) = '\u{0}' as i32 as libc::c_char;
    tmp___0 = calc_percentage(done, total);
    printf(b"\r[%3u%%] [%s\0" as *const u8 as *const libc::c_char, tmp___0, progress);
}
unsafe extern "C" fn print_alternate_output(mut axel: *mut axel_t) {
    let mut done: off_t = 0;
    let mut total: off_t = 0;
    let mut now: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    let mut width: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut progress: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut seconds: libc::c_int = 0;
    let mut minutes: libc::c_int = 0;
    let mut hours: libc::c_int = 0;
    let mut days: libc::c_int = 0;
    done = (*axel).bytes_done;
    total = (*axel).size;
    tmp = axel_gettime();
    now = tmp;
    tmp___0 = get_term_width();
    width = tmp___0;
    if width < 40 as libc::c_int {
        tmp___1 = dcgettext(
            0 as *mut libc::c_void as *const libc::c_char,
            b"Can't setup alternate output. Deactivating.\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        );
        fprintf(stderr, tmp___1 as *const libc::c_char);
        (*(*axel).conf).alternate_output = 0 as libc::c_int;
        return;
    }
    width -= 30 as libc::c_int;
    tmp___2 = malloc((width + 1 as libc::c_int) as size_t);
    progress = tmp___2 as *mut libc::c_char;
    if progress.is_null() {
        return;
    }
    memset(progress as *mut libc::c_void, '.' as i32, width as size_t);
    if total as libc::c_longlong != 9223372036854775807 as libc::c_longlong {
        print_alternate_output_progress(axel, progress, width, done, total, now);
    } else {
        *progress.offset(width as isize) = '\u{0}' as i32 as libc::c_char;
        printf(b"\r[ N/A] [%s\0" as *const u8 as *const libc::c_char, progress);
    }
    if (*axel).bytes_per_second > 1048576 as libc::c_longlong {
        printf(
            b"] [%6.1fMB/s]\0" as *const u8 as *const libc::c_char,
            (*axel).bytes_per_second as libc::c_double
                / 1048576 as libc::c_int as libc::c_double,
        );
    } else if (*axel).bytes_per_second > 1024 as libc::c_longlong {
        printf(
            b"] [%6.1fKB/s]\0" as *const u8 as *const libc::c_char,
            (*axel).bytes_per_second as libc::c_double
                / 1024 as libc::c_int as libc::c_double,
        );
    } else {
        printf(
            b"] [%6.1fB/s]\0" as *const u8 as *const libc::c_char,
            (*axel).bytes_per_second as libc::c_double,
        );
    }
    if total as libc::c_longlong != 9223372036854775807 as libc::c_longlong {
        if done < total {
            seconds = ((*axel).finish_time as libc::c_double - now) as libc::c_int;
            minutes = seconds / 60 as libc::c_int;
            seconds -= minutes * 60 as libc::c_int;
            hours = minutes / 60 as libc::c_int;
            minutes -= hours * 60 as libc::c_int;
            days = hours / 24 as libc::c_int;
            hours -= days * 24 as libc::c_int;
            if days != 0 {
                printf(b" [%2dd%2d]\0" as *const u8 as *const libc::c_char, days, hours);
            } else if hours != 0 {
                printf(
                    b" [%2dh%02d]\0" as *const u8 as *const libc::c_char,
                    hours,
                    minutes,
                );
            } else {
                printf(
                    b" [%02d:%02d]\0" as *const u8 as *const libc::c_char,
                    minutes,
                    seconds,
                );
            }
        }
    }
    free(progress as *mut libc::c_void);
}
unsafe extern "C" fn get_term_width() -> libc::c_int {
    let mut w: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    ioctl(1 as libc::c_int, 21523 as libc::c_ulong, &mut w as *mut winsize);
    return w.ws_col as libc::c_int;
}
unsafe extern "C" fn print_help() {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    print_version();
    tmp = dcgettext(
        0 as *mut libc::c_void as *const libc::c_char,
        b"Usage: axel [options] url1 [url2] [url...]\n\n--max-speed=x\t\t-s x\tSpecify maximum speed (bytes per second)\n--num-connections=x\t-n x\tSpecify maximum number of connections\n--max-redirect=x\t\tSpecify maximum number of redirections\n--output=f\t\t-o f\tSpecify local output file\n--search[=n]\t\t-S[n]\tSearch for mirrors and download from n servers\n--ipv4\t\t\t-4\tUse the IPv4 protocol\n--ipv6\t\t\t-6\tUse the IPv6 protocol\n--header=x\t\t-H x\tAdd HTTP header string\n--user-agent=x\t\t-U x\tSet user agent\n--no-proxy\t\t-N\tJust don't use any proxy server\n--insecure\t\t-k\tDon't verify the SSL certificate\n--no-clobber\t\t-c\tSkip download if file already exists\n--quiet\t\t\t-q\tLeave stdout alone\n--verbose\t\t-v\tMore status information\n--alternate\t\t-a\tAlternate progress indicator\n--percentage\t\t-p\tPrint simple percentages instead of progress bar (0-100)\n--help\t\t\t-h\tThis information\n--timeout=x\t\t-T x\tSet I/O and connection timeout\n--version\t\t-V\tVersion information\n\nVisit https://github.com/axel-download-accelerator/axel/issues to report bugs\n\0"
            as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    );
    printf(tmp as *const libc::c_char);
}
unsafe extern "C" fn print_version() {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = dcgettext(
        0 as *mut libc::c_void as *const libc::c_char,
        b"Axel %s (%s)\n\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    );
    printf(
        tmp as *const libc::c_char,
        b"2.17.11+gab2f84\0" as *const u8 as *const libc::c_char,
        b"linux-gnu\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn print_version_info() {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    print_version();
    tmp = dcgettext(
        0 as *mut libc::c_void as *const libc::c_char,
        b"Please, see the CREDITS file.\n\n\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    );
    tmp___0 = dcgettext(
        0 as *mut libc::c_void as *const libc::c_char,
        b"and others.\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    );
    printf(
        b"\nCopyright 2001-2007 Wilmer van der Gaast,\n\t  2007-2009 Giridhar Appaji Nag,\n\t  2008-2010 Philipp Hagemeister,\n\t  2015-2017 Joao Eriberto Mota Filho,\n\t  2016-2017 Stephen Thirlwall,\n\t  2017      Ismael Luceno,\n\t  2017      Antonio Quartulli,\n\t\t    %s\n%s\n\n\0"
            as *const u8 as *const libc::c_char,
        tmp___0,
        tmp,
    );
}
pub unsafe extern "C" fn print_messages(mut axel: *mut axel_t) {
    let mut m: *mut message_t = 0 as *mut message_t;
    if axel.is_null() {
        return;
    }
    loop {
        m = (*axel).message;
        if m.is_null() {
            break;
        }
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, ((*m).text).as_mut_ptr());
        (*axel).message = (*m).next as *mut message_t;
        free(m as *mut libc::c_void);
    };
}
pub unsafe extern "C" fn strlcpy(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut dsize: size_t,
) -> size_t {
    let mut osrc: *const libc::c_char = 0 as *const libc::c_char;
    let mut nleft: size_t = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_char = 0;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___2: *const libc::c_char = 0 as *const libc::c_char;
    osrc = src;
    nleft = dsize;
    if nleft != 0 as libc::c_ulong {
        loop {
            nleft = nleft.wrapping_sub(1);
            if !(nleft != 0 as libc::c_ulong) {
                break;
            }
            tmp = dst;
            dst = dst.offset(1);
            tmp___1 = src;
            src = src.offset(1);
            tmp___0 = *tmp___1;
            *tmp = tmp___0;
            if tmp___0 as libc::c_int == 0 as libc::c_int {
                break;
            }
        }
    }
    if nleft == 0 as libc::c_ulong {
        if dsize != 0 as libc::c_ulong {
            *dst = '\u{0}' as i32 as libc::c_char;
        }
        loop {
            tmp___2 = src;
            src = src.offset(1);
            if *tmp___2 == 0 {
                break;
            }
        }
    }
    return (src.offset_from(osrc) as libc::c_long - 1 as libc::c_long) as size_t;
}
pub unsafe extern "C" fn strlcat(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut dsize: size_t,
) -> size_t {
    let mut odst: *const libc::c_char = 0 as *const libc::c_char;
    let mut osrc: *const libc::c_char = 0 as *const libc::c_char;
    let mut n: size_t = 0;
    let mut dlen: size_t = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    odst = dst as *const libc::c_char;
    osrc = src;
    n = dsize;
    loop {
        tmp = n;
        n = n.wrapping_sub(1);
        if !(tmp != 0 as libc::c_ulong) {
            break;
        }
        if !(*dst as libc::c_int != 0 as libc::c_int) {
            break;
        }
        dst = dst.offset(1);
    }
    dlen = dst.offset_from(odst as *mut libc::c_char) as libc::c_long as size_t;
    n = dsize.wrapping_sub(dlen);
    tmp___1 = n;
    n = n.wrapping_sub(1);
    if tmp___1 == 0 as libc::c_ulong {
        tmp___0 = strlen(src);
        return dlen.wrapping_add(tmp___0);
    }
    while *src as libc::c_int != 0 as libc::c_int {
        if n != 0 as libc::c_ulong {
            tmp___2 = dst;
            dst = dst.offset(1);
            *tmp___2 = *src;
            n = n.wrapping_sub(1);
        }
        src = src.offset(1);
    }
    *dst = '\u{0}' as i32 as libc::c_char;
    return dlen.wrapping_add(src.offset_from(osrc) as libc::c_long as size_t);
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
