use ::libc;
use ::c2rust_bitfields;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type fuse_pollhandle;
    pub type fuse_dirhandle;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn bzero(_: *mut libc::c_void, _: libc::c_ulong);
    fn sendto(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: *const sockaddr,
        __addr_len: socklen_t,
    ) -> ssize_t;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: socklen_t,
    ) -> *const libc::c_char;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_error(__req: *mut gaicb) -> libc::c_int;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn exit(_: libc::c_int) -> !;
    static mut stdin: *mut FILE;
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
    fn perror(__s: *const libc::c_char);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn getaddrinfo_a(
        __mode: libc::c_int,
        __list: *mut *mut gaicb,
        __ent: libc::c_int,
        __sig: *mut sigevent,
    ) -> libc::c_int;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn fuse_opt_parse(
        args: *mut fuse_args,
        data: *mut libc::c_void,
        opts: *const fuse_opt,
        proc_0: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *const libc::c_char,
                libc::c_int,
                *mut fuse_args,
            ) -> libc::c_int,
        >,
    ) -> libc::c_int;
    fn fuse_opt_add_arg(args: *mut fuse_args, arg: *const libc::c_char) -> libc::c_int;
    fn fuse_opt_free_args(args: *mut fuse_args);
    fn fuse_main_real(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        op: *const fuse_operations,
        op_size: size_t,
        user_data: *mut libc::c_void,
    ) -> libc::c_int;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
    fn getuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn recvfrom(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> ssize_t;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
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
    fn pthread_cancel(__th: pthread_t) -> libc::c_int;
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
    fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [libc::c_char; 118],
    pub __ss_align: libc::c_ulong,
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
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: __anonunion___in6_u_979734923,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion___in6_u_979734923 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type icmp_type = libc::c_uint;
pub const ICMP_REPLY: icmp_type = 1;
pub const ICMP_REQUEST: icmp_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icmp_packet {
    pub peer: sockaddr_storage,
    pub peer_len: socklen_t,
    pub type_0: icmp_type,
    pub id: uint16_t,
    pub seqno: uint16_t,
    pub payload: *mut uint8_t,
    pub payload_len: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icmp_rule {
    pub request_type: libc::c_int,
    pub reply_type: libc::c_int,
    pub use_checksum: libc::c_int,
    pub strip_iphdr: libc::c_int,
}
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __clockid_t = libc::c_int;
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
pub type uint64_t = __uint64_t;
pub type clockid_t = __clockid_t;
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
pub struct gaicb {
    pub ar_name: *const libc::c_char,
    pub ar_service: *const libc::c_char,
    pub ar_request: *const addrinfo,
    pub ar_result: *mut addrinfo,
    pub __return: libc::c_int,
    pub __glibc_reserved: [libc::c_int; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct host {
    pub next: *mut host,
    pub sockaddr: sockaddr_storage,
    pub sockaddr_len: socklen_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linked_gaicb {
    pub gaicb: gaicb,
    pub next: *mut linked_gaicb,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eval_host {
    pub host: *mut host,
    pub sendtime: timespec,
    pub cur_seqno: uint16_t,
    pub id: uint16_t,
    pub payload: *mut uint8_t,
    pub payload_len: size_t,
    pub done: libc::c_int,
    pub num_tx: libc::c_int,
    pub num_rx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct evaldata {
    pub hosts: *mut eval_host,
    pub count: libc::c_int,
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __ino64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __pid_t = libc::c_int;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __fsblkcnt64_t = libc::c_ulong;
pub type __fsfilcnt64_t = libc::c_ulong;
pub type ino_t = __ino64_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type off_t = __off64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct__sigev_thread_746770901 {
    pub _function: Option::<unsafe extern "C" fn(__sigval_t) -> ()>,
    pub _attribute: *mut pthread_attr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion__sigev_un_233858830 {
    pub _pad: [libc::c_int; 12],
    pub _tid: __pid_t,
    pub _sigev_thread: __anonstruct__sigev_thread_746770901,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigevent {
    pub sigev_value: __sigval_t,
    pub sigev_signo: libc::c_int,
    pub sigev_notify: libc::c_int,
    pub _sigev_un: __anonunion__sigev_un_233858830,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
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
pub struct fuse_opt {
    pub templ: *const libc::c_char,
    pub offset: libc::c_ulong,
    pub value: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_args {
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub allocated: libc::c_int,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct fuse_file_info {
    pub flags: libc::c_int,
    pub fh_old: libc::c_ulong,
    pub writepage: libc::c_int,
    #[bitfield(name = "direct_io", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "keep_cache", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "flush", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "nonseekable", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "flock_release", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "padding", ty = "libc::c_uint", bits = "5..=31")]
    pub direct_io_keep_cache_flush_nonseekable_flock_release_padding: [u8; 4],
    pub fh: uint64_t,
    pub lock_owner: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_conn_info {
    pub proto_major: libc::c_uint,
    pub proto_minor: libc::c_uint,
    pub async_read: libc::c_uint,
    pub max_write: libc::c_uint,
    pub max_readahead: libc::c_uint,
    pub capable: libc::c_uint,
    pub want: libc::c_uint,
    pub max_background: libc::c_uint,
    pub congestion_threshold: libc::c_uint,
    pub reserved: [libc::c_uint; 23],
}
pub type fuse_buf_flags = libc::c_uint;
pub const FUSE_BUF_FD_RETRY: fuse_buf_flags = 8;
pub const FUSE_BUF_FD_SEEK: fuse_buf_flags = 4;
pub const FUSE_BUF_IS_FD: fuse_buf_flags = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_buf {
    pub size: size_t,
    pub flags: fuse_buf_flags,
    pub mem: *mut libc::c_void,
    pub fd: libc::c_int,
    pub pos: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_bufvec {
    pub count: size_t,
    pub idx: size_t,
    pub off: size_t,
    pub buf: [fuse_buf; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flock {
    pub l_type: libc::c_short,
    pub l_whence: libc::c_short,
    pub l_start: __off64_t,
    pub l_len: __off64_t,
    pub l_pid: __pid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utimbuf {
    pub actime: __time_t,
    pub modtime: __time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statvfs {
    pub f_bsize: libc::c_ulong,
    pub f_frsize: libc::c_ulong,
    pub f_blocks: __fsblkcnt64_t,
    pub f_bfree: __fsblkcnt64_t,
    pub f_bavail: __fsblkcnt64_t,
    pub f_files: __fsfilcnt64_t,
    pub f_ffree: __fsfilcnt64_t,
    pub f_favail: __fsfilcnt64_t,
    pub f_fsid: libc::c_ulong,
    pub f_flag: libc::c_ulong,
    pub f_namemax: libc::c_ulong,
    pub __f_spare: [libc::c_int; 6],
}
pub type fuse_dirh_t = *mut fuse_dirhandle;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct fuse_operations {
    pub getattr: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    >,
    pub readlink: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut libc::c_char,
            size_t,
        ) -> libc::c_int,
    >,
    pub getdir: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            fuse_dirh_t,
            Option::<
                unsafe extern "C" fn(
                    fuse_dirh_t,
                    *const libc::c_char,
                    libc::c_int,
                    ino_t,
                ) -> libc::c_int,
            >,
        ) -> libc::c_int,
    >,
    pub mknod: Option::<
        unsafe extern "C" fn(*const libc::c_char, mode_t, dev_t) -> libc::c_int,
    >,
    pub mkdir: Option::<
        unsafe extern "C" fn(*const libc::c_char, mode_t) -> libc::c_int,
    >,
    pub unlink: Option::<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int>,
    pub rmdir: Option::<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int>,
    pub symlink: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    >,
    pub rename: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    >,
    pub link: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    >,
    pub chmod: Option::<
        unsafe extern "C" fn(*const libc::c_char, mode_t) -> libc::c_int,
    >,
    pub chown: Option::<
        unsafe extern "C" fn(*const libc::c_char, uid_t, gid_t) -> libc::c_int,
    >,
    pub truncate: Option::<
        unsafe extern "C" fn(*const libc::c_char, off_t) -> libc::c_int,
    >,
    pub utime: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut utimbuf) -> libc::c_int,
    >,
    pub open: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut fuse_file_info) -> libc::c_int,
    >,
    pub read: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut libc::c_char,
            size_t,
            off_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub write: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
            size_t,
            off_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub statfs: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut statvfs) -> libc::c_int,
    >,
    pub flush: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut fuse_file_info) -> libc::c_int,
    >,
    pub release: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut fuse_file_info) -> libc::c_int,
    >,
    pub fsync: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            libc::c_int,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub setxattr: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
            *const libc::c_char,
            size_t,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub getxattr: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
            *mut libc::c_char,
            size_t,
        ) -> libc::c_int,
    >,
    pub listxattr: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut libc::c_char,
            size_t,
        ) -> libc::c_int,
    >,
    pub removexattr: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    >,
    pub opendir: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut fuse_file_info) -> libc::c_int,
    >,
    pub readdir: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    *const stat,
                    off_t,
                ) -> libc::c_int,
            >,
            off_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub releasedir: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut fuse_file_info) -> libc::c_int,
    >,
    pub fsyncdir: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            libc::c_int,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub init: Option::<unsafe extern "C" fn(*mut fuse_conn_info) -> *mut libc::c_void>,
    pub destroy: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub access: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub create: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            mode_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub ftruncate: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            off_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub fgetattr: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut stat,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub lock: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut fuse_file_info,
            libc::c_int,
            *mut flock,
        ) -> libc::c_int,
    >,
    pub utimens: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const timespec) -> libc::c_int,
    >,
    pub bmap: Option::<
        unsafe extern "C" fn(*const libc::c_char, size_t, *mut uint64_t) -> libc::c_int,
    >,
    #[bitfield(name = "flag_nullpath_ok", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "flag_nopath", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "flag_utime_omit_ok", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "flag_reserved", ty = "libc::c_uint", bits = "3..=31")]
    pub flag_nullpath_ok_flag_nopath_flag_utime_omit_ok_flag_reserved: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
    pub ioctl: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            libc::c_int,
            *mut libc::c_void,
            *mut fuse_file_info,
            libc::c_uint,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub poll: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut fuse_file_info,
            *mut fuse_pollhandle,
            *mut libc::c_uint,
        ) -> libc::c_int,
    >,
    pub write_buf: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut fuse_bufvec,
            off_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub read_buf: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut *mut fuse_bufvec,
            size_t,
            off_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub flock: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut fuse_file_info,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub fallocate: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            libc::c_int,
            off_t,
            off_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arginfo {
    pub hostfile: *mut libc::c_char,
    pub mountpoint: *mut libc::c_char,
    pub num_args: libc::c_int,
    pub timeout: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct io {
    pub fs_cond: pthread_cond_t,
    pub net_cond: pthread_cond_t,
    pub mutex: pthread_mutex_t,
    pub owner: io_owner,
    pub data: *mut uint8_t,
    pub len: size_t,
}
pub type io_owner = libc::c_uint;
pub const OWNER_NET: io_owner = 2;
pub const OWNER_FS: io_owner = 1;
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
    pub __annonCompField1: __anonunion____missing_field_name_752798888,
    pub __annonCompField2: __anonunion____missing_field_name_900150282,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_900150282 {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: __anonstruct___g1_start32_900150283,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___g1_start32_900150283 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_752798888 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: __anonstruct___wseq32_112954846,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___wseq32_112954846 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chunk {
    pub next_active: *mut chunk,
    pub next_file: *mut chunk,
    pub host: *mut host,
    pub io: *mut io,
    pub id: uint16_t,
    pub seqno: uint16_t,
    pub len: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file {
    pub next: *mut file,
    pub name: *const libc::c_char,
    pub chunks: *mut chunk,
    pub mode: mode_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_fd_set_356711149 {
    pub fds_bits: [__fd_mask; 16],
}
pub type fd_set = __anonstruct_fd_set_356711149;
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
pub struct icmp6_filter {
    pub icmp6_filt: [uint32_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pkt_stats {
    pub packets: libc::c_ulonglong,
    pub bytes: libc::c_ulonglong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct net_data {
    pub responder: pthread_t,
    pub status: pthread_t,
    pub stats_mutex: pthread_mutex_t,
    pub tx: pkt_stats,
    pub rx: pkt_stats,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_condattr_t_488594145 {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
pub type pthread_condattr_t = __anonunion_pthread_condattr_t_488594145;
static mut icmpv4: icmp_rule = {
    let mut init = icmp_rule {
        request_type: 8 as libc::c_int,
        reply_type: 0 as libc::c_int,
        use_checksum: 1 as libc::c_int,
        strip_iphdr: 1 as libc::c_int,
    };
    init
};
static mut icmpv6: icmp_rule = {
    let mut init = icmp_rule {
        request_type: 128 as libc::c_int,
        reply_type: 129 as libc::c_int,
        use_checksum: 0 as libc::c_int,
        strip_iphdr: 0 as libc::c_int,
    };
    init
};
unsafe extern "C" fn checksum(mut data: *mut uint8_t, mut len: uint32_t) -> uint16_t {
    let mut csum: uint32_t = 0;
    let mut i: uint32_t = 0;
    let mut c: uint16_t = 0;
    csum = 0 as libc::c_int as uint32_t;
    i = 0 as libc::c_int as uint32_t;
    while i < len {
        c = ((*data.offset(i as isize) as libc::c_int) << 8 as libc::c_int) as uint16_t;
        if i.wrapping_add(1 as libc::c_uint) < len {
            c = (c as libc::c_int
                | *data.offset(i.wrapping_add(1 as libc::c_uint) as isize)
                    as libc::c_int) as uint16_t;
        }
        csum = (csum as libc::c_uint).wrapping_add(c as uint32_t) as uint32_t
            as uint32_t;
        i = (i as libc::c_uint).wrapping_add(2 as libc::c_uint) as uint32_t as uint32_t;
    }
    csum = (csum >> 16 as libc::c_int).wrapping_add(csum & 65535 as libc::c_uint);
    csum = (csum as libc::c_uint).wrapping_add(csum >> 16 as libc::c_int) as uint32_t
        as uint32_t;
    return !csum as uint16_t;
}
unsafe extern "C" fn read16(mut data: *mut uint8_t) -> uint16_t {
    return ((*data.offset(0 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int
        | *data.offset(1 as libc::c_int as isize) as libc::c_int) as uint16_t;
}
unsafe extern "C" fn write16(mut data: *mut uint8_t, mut s: uint16_t) {
    *data
        .offset(
            0 as libc::c_int as isize,
        ) = (s as libc::c_int >> 8 as libc::c_int) as uint8_t;
    *data
        .offset(
            1 as libc::c_int as isize,
        ) = (s as libc::c_int & 255 as libc::c_int) as uint8_t;
}
unsafe extern "C" fn icmp_encode(
    mut pkt: *mut icmp_packet,
    mut len: *mut libc::c_int,
) -> *mut uint8_t {
    let mut rule: *const icmp_rule = 0 as *const icmp_rule;
    let mut tmp: *const icmp_rule = 0 as *const icmp_rule;
    let mut data: *mut uint8_t = 0 as *mut uint8_t;
    let mut pktlen: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: uint16_t = 0;
    if (*pkt).peer.ss_family as libc::c_int == 2 as libc::c_int {
        tmp = &icmpv4;
    } else {
        tmp = &icmpv6;
    }
    rule = tmp;
    pktlen = (8 as libc::c_uint).wrapping_add((*pkt).payload_len) as libc::c_int;
    tmp___0 = calloc(1 as libc::c_int as size_t, pktlen as size_t);
    data = tmp___0 as *mut uint8_t;
    if data.is_null() {
        return 0 as *mut libc::c_void as *mut uint8_t;
    }
    if (*pkt).type_0 as libc::c_uint == 0 as libc::c_uint {
        *data.offset(0 as libc::c_int as isize) = (*rule).request_type as uint8_t;
    } else {
        *data.offset(0 as libc::c_int as isize) = (*rule).reply_type as uint8_t;
    }
    write16(data.offset(4 as libc::c_int as isize), (*pkt).id);
    write16(data.offset(6 as libc::c_int as isize), (*pkt).seqno);
    if (*pkt).payload_len != 0 {
        memcpy(
            data.offset(8 as libc::c_int as isize) as *mut libc::c_void,
            (*pkt).payload as *const libc::c_void,
            (*pkt).payload_len as size_t,
        );
    }
    if (*rule).use_checksum != 0 {
        tmp___1 = checksum(data, pktlen as uint32_t);
        write16(data.offset(2 as libc::c_int as isize), tmp___1);
    }
    *len = pktlen;
    return data;
}
pub unsafe extern "C" fn icmp_send(
    mut socket___0: libc::c_int,
    mut pkt: *mut icmp_packet,
) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut icmpdata: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp: ssize_t = 0;
    icmpdata = icmp_encode(pkt, &mut len);
    if icmpdata.is_null() {
        return 0 as libc::c_int;
    }
    tmp = sendto(
        socket___0,
        icmpdata as *const libc::c_void,
        len as size_t,
        0 as libc::c_int,
        &mut (*pkt).peer as *mut sockaddr_storage as *mut sockaddr as *const sockaddr,
        (*pkt).peer_len,
    );
    len = tmp as libc::c_int;
    free(icmpdata as *mut libc::c_void);
    return len;
}
pub unsafe extern "C" fn icmp_parse(
    mut pkt: *mut icmp_packet,
    mut data: *mut uint8_t,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut rule: *const icmp_rule = 0 as *const icmp_rule;
    let mut tmp: *const icmp_rule = 0 as *const icmp_rule;
    let mut hdrlen: libc::c_int = 0;
    let mut tmp___0: uint16_t = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*pkt).peer.ss_family as libc::c_int == 2 as libc::c_int {
        tmp = &icmpv4;
    } else {
        tmp = &icmpv6;
    }
    rule = tmp;
    if (*rule).strip_iphdr != 0 {
        if len == 0 as libc::c_int {
            return -(3 as libc::c_int);
        }
        hdrlen = (*data.offset(0 as libc::c_int as isize) as libc::c_int
            & 15 as libc::c_int) << 2 as libc::c_int;
        if len < hdrlen {
            return -(4 as libc::c_int);
        }
        data = data.offset(hdrlen as isize);
        len -= hdrlen;
    }
    if len < 8 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*rule).use_checksum != 0 {
        tmp___0 = checksum(data, len as uint32_t);
        if tmp___0 as libc::c_int != 0 as libc::c_int {
            return -(2 as libc::c_int);
        }
    }
    if (*rule).request_type == *data.offset(0 as libc::c_int as isize) as libc::c_int {
        (*pkt).type_0 = ICMP_REQUEST;
    } else if (*rule).reply_type
            == *data.offset(0 as libc::c_int as isize) as libc::c_int
        {
        (*pkt).type_0 = ICMP_REPLY;
    } else {
        return -(5 as libc::c_int)
    }
    (*pkt).id = read16(data.offset(4 as libc::c_int as isize));
    (*pkt).seqno = read16(data.offset(6 as libc::c_int as isize));
    (*pkt).payload_len = (len - 8 as libc::c_int) as uint32_t;
    if (*pkt).payload_len != 0 {
        tmp___1 = malloc((*pkt).payload_len as size_t);
        (*pkt).payload = tmp___1 as *mut uint8_t;
        memcpy(
            (*pkt).payload as *mut libc::c_void,
            data.offset(8 as libc::c_int as isize) as *const libc::c_void,
            (*pkt).payload_len as size_t,
        );
    } else {
        (*pkt).payload = 0 as *mut libc::c_void as *mut uint8_t;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_in_addr(mut ss: *mut sockaddr_storage) -> *mut libc::c_void {
    if (*ss).ss_family as libc::c_int == 2 as libc::c_int {
        return &mut (*(ss as *mut sockaddr_in)).sin_addr as *mut in_addr
            as *mut libc::c_void
    } else {
        return &mut (*(ss as *mut sockaddr_in6)).sin6_addr as *mut in6_addr
            as *mut libc::c_void
    };
}
unsafe extern "C" fn icmp_type_str(mut pkt: *mut icmp_packet) -> *mut libc::c_char {
    if (*pkt).type_0 as libc::c_uint == 1 as libc::c_uint {
        return b"Reply from\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*pkt).type_0 as libc::c_uint == 0 as libc::c_uint {
        return b"Request to\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    return b"Other\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
pub unsafe extern "C" fn icmp_dump(mut pkt: *mut icmp_packet) {
    let mut ipaddr: [libc::c_char; 64] = [0; 64];
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    bzero(
        ipaddr.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
    );
    tmp = get_in_addr(&mut (*pkt).peer);
    inet_ntop(
        (*pkt).peer.ss_family as libc::c_int,
        tmp as *const libc::c_void,
        ipaddr.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as socklen_t,
    );
    tmp___0 = icmp_type_str(pkt);
    printf(
        b"%s %s, id %04X, seqno %04X, payload %d bytes\n\0" as *const u8
            as *const libc::c_char,
        tmp___0,
        ipaddr.as_mut_ptr(),
        (*pkt).id as libc::c_int,
        (*pkt).seqno as libc::c_int,
        (*pkt).payload_len,
    );
}
static mut addr_request: addrinfo = {
    let mut init = addrinfo {
        ai_flags: 0 as libc::c_int,
        ai_family: 0 as libc::c_int,
        ai_socktype: 3 as libc::c_int,
        ai_protocol: 0 as libc::c_int,
        ai_addrlen: 0 as libc::c_uint,
        ai_addr: 0 as *const sockaddr as *mut sockaddr,
        ai_canonname: 0 as *const libc::c_char as *mut libc::c_char,
        ai_next: 0 as *const addrinfo as *mut addrinfo,
    };
    init
};
pub unsafe extern "C" fn host_make_resolvlist(
    mut file: *mut FILE,
    mut list: *mut *mut *mut gaicb,
) -> libc::c_int {
    let mut hosts: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut l: *mut *mut gaicb = 0 as *mut *mut gaicb;
    let mut head: *mut linked_gaicb = 0 as *mut linked_gaicb;
    let mut tail: *mut linked_gaicb = 0 as *mut linked_gaicb;
    let mut lg: *mut linked_gaicb = 0 as *mut linked_gaicb;
    let mut hostname: [libc::c_char; 300] = [0; 300];
    let mut res: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    hosts = 0 as libc::c_int;
    head = 0 as *mut libc::c_void as *mut linked_gaicb;
    tail = 0 as *mut libc::c_void as *mut linked_gaicb;
    loop {
        memset(
            hostname.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[libc::c_char; 300]>() as libc::c_ulong,
        );
        res = fscanf(
            file,
            b"%256s\0" as *const u8 as *const libc::c_char,
            hostname.as_mut_ptr(),
        );
        if res == -(1 as libc::c_int) {
            break;
        }
        tmp = calloc(
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<linked_gaicb>() as libc::c_ulong,
        );
        lg = tmp as *mut linked_gaicb;
        if lg.is_null() {
            return 0 as libc::c_int;
        }
        tmp___0 = strlen(hostname.as_mut_ptr() as *const libc::c_char);
        tmp___1 = strndup(hostname.as_mut_ptr() as *const libc::c_char, tmp___0);
        (*lg).gaicb.ar_name = tmp___1 as *const libc::c_char;
        (*lg).gaicb.ar_request = &addr_request;
        if head.is_null() {
            head = lg;
        }
        if !tail.is_null() {
            (*tail).next = lg;
        }
        tail = lg;
        hosts += 1;
    }
    tmp___2 = calloc(
        hosts as size_t,
        ::std::mem::size_of::<*mut gaicb>() as libc::c_ulong,
    );
    l = tmp___2 as *mut *mut gaicb;
    if l.is_null() {
        return 0 as libc::c_int;
    }
    lg = head;
    i = 0 as libc::c_int;
    while i < hosts {
        let ref mut fresh0 = *l.offset(i as isize);
        *fresh0 = &mut (*lg).gaicb;
        lg = (*lg).next;
        i += 1;
    }
    *list = l;
    return hosts;
}
pub unsafe extern "C" fn host_free_resolvlist(
    mut list: *mut *mut gaicb,
    mut length: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < length {
        free(
            (**list.offset(i as isize)).ar_name as *mut libc::c_char as *mut libc::c_void,
        );
        if !((**list.offset(i as isize)).ar_result).is_null() {
            freeaddrinfo((**list.offset(i as isize)).ar_result);
        }
        free(*list.offset(i as isize) as *mut libc::c_void);
        i += 1;
    }
    free(list as *mut libc::c_void);
}
pub unsafe extern "C" fn host_create(
    mut list: *mut *mut gaicb,
    mut listlength: libc::c_int,
) -> *mut host {
    let mut hosts: *mut host = 0 as *mut host;
    let mut last: *mut host = 0 as *mut host;
    let mut i: libc::c_int = 0;
    let mut result: *mut addrinfo = 0 as *mut addrinfo;
    let mut h: *mut host = 0 as *mut host;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    hosts = 0 as *mut libc::c_void as *mut host;
    last = 0 as *mut libc::c_void as *mut host;
    i = 0 as libc::c_int;
    while i < listlength {
        tmp___0 = gai_error(*list.offset(i as isize));
        if tmp___0 == 0 as libc::c_int {
            result = (**list.offset(i as isize)).ar_result;
            while !result.is_null() {
                tmp = calloc(
                    1 as libc::c_int as size_t,
                    ::std::mem::size_of::<host>() as libc::c_ulong,
                );
                h = tmp as *mut host;
                if h.is_null() {
                    return 0 as *mut libc::c_void as *mut host;
                }
                memcpy(
                    &mut (*h).sockaddr as *mut sockaddr_storage as *mut libc::c_void,
                    (*result).ai_addr as *const libc::c_void,
                    (*result).ai_addrlen as size_t,
                );
                (*h).sockaddr_len = (*result).ai_addrlen;
                if hosts.is_null() {
                    hosts = h;
                }
                if !last.is_null() {
                    (*last).next = h;
                }
                last = h;
                result = (*result).ai_next;
            }
        }
        i += 1;
    }
    return hosts;
}
static mut latency_sum_us: uint64_t = 0;
static mut latency_count: uint32_t = 0;
unsafe extern "C" fn diff_add(mut start: *mut timespec, mut end: *mut timespec) {
    let mut us: uint64_t = 0;
    us = (((*end).tv_sec - (*start).tv_sec) * 1000000 as libc::c_long) as uint64_t;
    us = (us as libc::c_ulong)
        .wrapping_sub(((*start).tv_nsec / 1000 as libc::c_long) as uint64_t) as uint64_t
        as uint64_t;
    us = (us as libc::c_ulong)
        .wrapping_add(((*end).tv_nsec / 1000 as libc::c_long) as uint64_t) as uint64_t
        as uint64_t;
    latency_sum_us = (latency_sum_us as libc::c_ulong).wrapping_add(us) as uint64_t
        as uint64_t;
    latency_count = latency_count.wrapping_add(1);
}
unsafe extern "C" fn eval_reply(
    mut userdata: *mut libc::c_void,
    mut addr: *mut sockaddr_storage,
    mut addrlen: size_t,
    mut id: uint16_t,
    mut seqno: uint16_t,
    mut data: *mut *mut uint8_t,
    mut len: size_t,
) {
    let mut i: libc::c_int = 0;
    let mut eval: *mut evaldata = 0 as *mut evaldata;
    let mut recvtime: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut eh: *mut eval_host = 0 as *mut eval_host;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    eval = userdata as *mut evaldata;
    clock_gettime(4 as libc::c_int, &mut recvtime);
    i = 0 as libc::c_int;
    while i < (*eval).count {
        eh = ((*eval).hosts).offset(i as isize);
        if addrlen == (*(*eh).host).sockaddr_len as size_t {
            tmp = memcmp(
                addr as *const libc::c_void,
                &mut (*(*eh).host).sockaddr as *mut sockaddr_storage
                    as *const libc::c_void,
                addrlen,
            );
            if tmp == 0 as libc::c_int {
                if (*eh).payload_len == len {
                    tmp___0 = memcmp(
                        *data as *const libc::c_void,
                        (*eh).payload as *const libc::c_void,
                        (*eh).payload_len,
                    );
                    if tmp___0 == 0 as libc::c_int {
                        if (*eh).id as libc::c_int == id as libc::c_int {
                            if (*eh).cur_seqno as libc::c_int == seqno as libc::c_int {
                                (*eh).num_rx += 1;
                                (*eh).done = 1 as libc::c_int;
                                net_inc_rx((*eh).payload_len as libc::c_int);
                                (*eh)
                                    .cur_seqno = ((*eh).cur_seqno as libc::c_int
                                    + 1 as libc::c_int) as uint16_t;
                                diff_add(&mut (*eh).sendtime, &mut recvtime);
                                break;
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
}
pub unsafe extern "C" fn host_evaluate(
    mut hosts: *mut *mut host,
    mut length: libc::c_int,
    mut timeout___0: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut addr: libc::c_int = 0;
    let mut good_hosts: libc::c_int = 0;
    let mut host: *mut host = 0 as *mut host;
    let mut prev: *mut host = 0 as *mut host;
    let mut evaldata: evaldata = evaldata {
        hosts: 0 as *mut eval_host,
        count: 0,
    };
    let mut eval_payload: [uint8_t; 1024] = [0; 1024];
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut h: libc::c_int = 0;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut eh: *mut eval_host = 0 as *mut eval_host;
    let mut alldone: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut eh___0: *mut eval_host = 0 as *mut eval_host;
    let mut next: *mut host = 0 as *mut host;
    evaldata.count = length;
    tmp = calloc(length as size_t, ::std::mem::size_of::<eval_host>() as libc::c_ulong);
    evaldata.hosts = tmp as *mut eval_host;
    if (evaldata.hosts).is_null() {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < ::std::mem::size_of::<[uint8_t; 1024]>() as libc::c_ulong
    {
        eval_payload[i as usize] = (i & 255 as libc::c_int) as uint8_t;
        i += 1;
    }
    addr = 0 as libc::c_int;
    host = *hosts;
    i = 0 as libc::c_int;
    while i < length {
        let ref mut fresh1 = (*(evaldata.hosts).offset(addr as isize)).host;
        *fresh1 = host;
        (*(evaldata.hosts).offset(addr as isize)).id = addr as uint16_t;
        (*(evaldata.hosts).offset(addr as isize))
            .cur_seqno = (addr * 2 as libc::c_int) as uint16_t;
        let ref mut fresh2 = (*(evaldata.hosts).offset(addr as isize)).payload;
        *fresh2 = eval_payload.as_mut_ptr();
        (*(evaldata.hosts).offset(addr as isize))
            .payload_len = ::std::mem::size_of::<[uint8_t; 1024]>() as libc::c_ulong;
        addr += 1;
        host = (*host).next;
        i += 1;
    }
    printf(
        b"Evaluating %d hosts (timeout=%ds).\0" as *const u8 as *const libc::c_char,
        length,
        timeout___0,
    );
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        printf(b".\0" as *const u8 as *const libc::c_char);
        fflush(stdout);
        h = 0 as libc::c_int;
        while h < length {
            eh = (evaldata.hosts).offset(h as isize);
            clock_gettime(4 as libc::c_int, &mut (*eh).sendtime);
            (*eh).done = 0 as libc::c_int;
            (*eh).num_tx += 1;
            net_send(
                (*eh).host,
                (*eh).id,
                (*eh).cur_seqno,
                (*eh).payload as *const uint8_t,
                (*eh).payload_len,
            );
            h += 1;
        }
        tv.tv_sec = timeout___0 as __time_t;
        tv.tv_usec = 0 as libc::c_int as __suseconds_t;
        loop {
            alldone = 1 as libc::c_int;
            h = 0 as libc::c_int;
            while h < length {
                alldone &= (*(evaldata.hosts).offset(h as isize)).done;
                h += 1;
            }
            if alldone != 0 {
                break;
            }
            res = net_recv(
                &mut tv,
                Some(
                    eval_reply
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut sockaddr_storage,
                            size_t,
                            uint16_t,
                            uint16_t,
                            *mut *mut uint8_t,
                            size_t,
                        ) -> (),
                ),
                &mut evaldata as *mut evaldata as *mut libc::c_void,
            );
            if res == 0 {
                break;
            }
        }
        i += 1;
    }
    printf(b" done.\n\0" as *const u8 as *const libc::c_char);
    good_hosts = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < length {
        eh___0 = (evaldata.hosts).offset(i as isize);
        if (*eh___0).num_tx == 0 as libc::c_int {
            (*(*eh___0).host).sockaddr_len = 0 as libc::c_int as socklen_t;
        } else if (*eh___0).num_tx != (*eh___0).num_rx {
            (*(*eh___0).host).sockaddr_len = 0 as libc::c_int as socklen_t;
        } else {
            good_hosts += 1;
        }
        i += 1;
    }
    host = *hosts;
    prev = 0 as *mut libc::c_void as *mut host;
    while !host.is_null() {
        next = (*host).next;
        if (*host).sockaddr_len == 0 as libc::c_uint {
            if host as libc::c_ulong == *hosts as libc::c_ulong {
                *hosts = next;
            }
            if !prev.is_null() {
                (*prev).next = next;
            }
            free(host as *mut libc::c_void);
        } else {
            prev = host;
        }
        host = next;
    }
    free(evaldata.hosts as *mut libc::c_void);
    printf(
        b"%d of %d hosts responded correctly to all pings\0" as *const u8
            as *const libc::c_char,
        good_hosts,
        length,
    );
    if good_hosts != 0 {
        printf(
            b" (average RTT %.02f ms)\0" as *const u8 as *const libc::c_char,
            (latency_sum_us as libc::c_float
                / (latency_count as libc::c_float * 1000.0f32)) as libc::c_double,
        );
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    return good_hosts;
}
static mut hosts_start: *mut host = 0 as *const host as *mut host;
static mut hosts_cur: *mut host = 0 as *const host as *mut host;
pub unsafe extern "C" fn host_use(mut hosts: *mut host) {
    hosts_start = hosts;
}
pub unsafe extern "C" fn host_get_next() -> *mut host {
    let mut h: *mut host = 0 as *mut host;
    if hosts_start.is_null() {
        __assert_fail(
            b"hosts_start\0" as *const u8 as *const libc::c_char,
            b"host.c\0" as *const u8 as *const libc::c_char,
            307 as libc::c_uint,
            b"host_get_next\0" as *const u8 as *const libc::c_char,
        );
    }
    if hosts_cur.is_null() {
        hosts_cur = hosts_start;
    }
    h = hosts_cur;
    hosts_cur = (*hosts_cur).next;
    return h;
}
static mut pingfs_opts: [fuse_opt; 4] = [
    {
        let mut init = fuse_opt {
            templ: b"-h\0" as *const u8 as *const libc::c_char,
            offset: 4294967295 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"-u \0" as *const u8 as *const libc::c_char,
            offset: 4294967295 as libc::c_ulong,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: b"-t \0" as *const u8 as *const libc::c_char,
            offset: 4294967295 as libc::c_ulong,
            value: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = fuse_opt {
            templ: 0 as *const libc::c_void as *mut libc::c_void as *const libc::c_char,
            offset: 0 as libc::c_ulong,
            value: 0 as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn read_hostnames(
    mut hfile: *const libc::c_char,
    mut list: *mut *mut *mut gaicb,
) -> libc::c_int {
    let mut h: libc::c_int = 0;
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut tmp: libc::c_int = 0;
    h = 0 as libc::c_int;
    tmp = strcmp(b"-\0" as *const u8 as *const libc::c_char, hfile);
    if tmp == 0 as libc::c_int {
        file = stdin;
    } else {
        file = fopen(hfile, b"r\0" as *const u8 as *const libc::c_char);
        if file.is_null() {
            perror(b"Failed to read file\0" as *const u8 as *const libc::c_char);
            return h;
        }
    }
    h = host_make_resolvlist(file, list);
    fclose(file);
    return h;
}
unsafe extern "C" fn resolve_names(
    mut list: *mut *mut gaicb,
    mut names: libc::c_int,
    mut hosts: *mut *mut host,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut hostcount: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut result: *mut addrinfo = 0 as *mut addrinfo;
    fprintf(
        stderr,
        b"Resolving %d hostnames... \0" as *const u8 as *const libc::c_char,
        names,
    );
    fflush(stderr);
    ret = getaddrinfo_a(
        0 as libc::c_int,
        list,
        names,
        0 as *mut libc::c_void as *mut sigevent,
    );
    if ret != 0 as libc::c_int {
        tmp = gai_strerror(ret);
        fprintf(
            stderr,
            b"Resolving failed: %s\n\0" as *const u8 as *const libc::c_char,
            tmp,
        );
        return -(1 as libc::c_int);
    }
    fprintf(stderr, b"done.\n\0" as *const u8 as *const libc::c_char);
    hostcount = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < names {
        ret = gai_error(*list.offset(i as isize));
        if ret != 0 {
            tmp___0 = gai_strerror(ret);
            fprintf(
                stderr,
                b"Skipping %s: %s\n\0" as *const u8 as *const libc::c_char,
                (**list.offset(i as isize)).ar_name,
                tmp___0,
            );
        } else {
            result = (**list.offset(i as isize)).ar_result;
            loop {
                hostcount += 1;
                result = (*result).ai_next;
                if result.is_null() {
                    break;
                }
            }
        }
        i += 1;
    }
    if hostcount == 0 {
        fprintf(
            stderr,
            b"No hosts found! Exiting\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    *hosts = host_create(list, names);
    host_free_resolvlist(list, names);
    if *hosts as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"Failed creating list list, exiting\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return hostcount;
}
unsafe extern "C" fn print_usage(mut progname: *mut libc::c_char) {
    fprintf(
        stderr,
        b"Usage: %s [options] hostfile mountpoint\nOptions:\n -h           : Print this help and exit\n -u username  : Mount the filesystem as this user\n -t timeout   : Max time to wait for icmp reply (seconds, default 1)\n\0"
            as *const u8 as *const libc::c_char,
        progname,
    );
}
unsafe extern "C" fn pingfs_opt_proc(
    mut data: *mut libc::c_void,
    mut arg: *const libc::c_char,
    mut key: libc::c_int,
    mut outargs: *mut fuse_args,
) -> libc::c_int {
    let mut arginfo: *mut arginfo = 0 as *mut arginfo;
    let mut pw: *mut passwd = 0 as *mut passwd;
    let mut res: libc::c_int = 0;
    let mut userarg: [libc::c_char; 64] = [0; 64];
    arginfo = data as *mut arginfo;
    match key {
        -2 => {
            (*arginfo).num_args += 1;
            if ((*arginfo).hostfile).is_null() {
                (*arginfo).hostfile = strdup(arg);
                return 0 as libc::c_int;
            } else {
                if ((*arginfo).mountpoint).is_null() {
                    (*arginfo).mountpoint = strdup(arg);
                }
            }
        }
        0 => {
            print_usage(*((*outargs).argv).offset(0 as libc::c_int as isize));
            exit(0 as libc::c_int);
        }
        1 => {
            pw = getpwnam(arg.offset(2 as libc::c_int as isize));
            if !pw.is_null() {
                snprintf(
                    userarg.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                    b"-ouid=%d,gid=%d\0" as *const u8 as *const libc::c_char,
                    (*pw).pw_uid,
                    (*pw).pw_gid,
                );
                fuse_opt_add_arg(outargs, userarg.as_mut_ptr() as *const libc::c_char);
                return 0 as libc::c_int;
            } else {
                fprintf(
                    stderr,
                    b"Bad username given! Exiting\n\0" as *const u8
                        as *const libc::c_char,
                );
                print_usage(*((*outargs).argv).offset(0 as libc::c_int as isize));
                exit(1 as libc::c_int);
            }
        }
        2 => {
            res = sscanf(
                arg,
                b"-t%d\0" as *const u8 as *const libc::c_char,
                &mut (*arginfo).timeout as *mut libc::c_int,
            );
            if res == 1 as libc::c_int {
                if (*arginfo).timeout > 0 as libc::c_int {
                    if (*arginfo).timeout < 60 as libc::c_int {
                        return 0 as libc::c_int
                    } else {
                        fprintf(
                            stderr,
                            b"Bad timeout given! Exiting\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        print_usage(
                            *((*outargs).argv).offset(0 as libc::c_int as isize),
                        );
                        exit(1 as libc::c_int);
                    }
                } else {
                    fprintf(
                        stderr,
                        b"Bad timeout given! Exiting\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    print_usage(*((*outargs).argv).offset(0 as libc::c_int as isize));
                    exit(1 as libc::c_int);
                }
            } else {
                fprintf(
                    stderr,
                    b"Bad timeout given! Exiting\n\0" as *const u8 as *const libc::c_char,
                );
                print_usage(*((*outargs).argv).offset(0 as libc::c_int as isize));
                exit(1 as libc::c_int);
            }
        }
        _ => {}
    }
    return 1 as libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut list: *mut *mut gaicb = 0 as *mut *mut gaicb;
    let mut hosts: *mut host = 0 as *mut host;
    let mut h: *mut host = 0 as *mut host;
    let mut hostnames: libc::c_int = 0;
    let mut host_count: libc::c_int = 0;
    let mut args: fuse_args = fuse_args {
        argc: 0,
        argv: 0 as *mut *mut libc::c_char,
        allocated: 0,
    };
    let mut arginfo: arginfo = arginfo {
        hostfile: 0 as *mut libc::c_char,
        mountpoint: 0 as *mut libc::c_char,
        num_args: 0,
        timeout: 0,
    };
    let mut mountdir: stat = stat {
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
    let mut res: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut host: *mut host = 0 as *mut host;
    hosts = 0 as *mut libc::c_void as *mut host;
    args.argc = argc;
    args.argv = argv;
    args.allocated = 0 as libc::c_int;
    memset(
        &mut arginfo as *mut arginfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<arginfo>() as libc::c_ulong,
    );
    arginfo.timeout = 1 as libc::c_int;
    tmp = fuse_opt_parse(
        &mut args,
        &mut arginfo as *mut arginfo as *mut libc::c_void,
        pingfs_opts.as_ptr(),
        Some(
            pingfs_opt_proc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    libc::c_int,
                    *mut fuse_args,
                ) -> libc::c_int,
        ),
    );
    if tmp == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"Error parsing options!\n\0" as *const u8 as *const libc::c_char,
        );
        print_usage(*argv.offset(0 as libc::c_int as isize));
        return 1 as libc::c_int;
    }
    if arginfo.num_args != 2 as libc::c_int {
        fprintf(stderr, b"Need two arguments!\n\0" as *const u8 as *const libc::c_char);
        print_usage(*argv.offset(0 as libc::c_int as isize));
        return 1 as libc::c_int;
    }
    res = stat(arginfo.mountpoint as *const libc::c_char, &mut mountdir as *mut stat);
    if res != 0 {
        perror(b"Failed to check mountpoint\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    if !(mountdir.st_mode & 61440 as libc::c_uint == 16384 as libc::c_uint) {
        fprintf(
            stderr,
            b"Mountpoint must be a directory! Exiting\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    free(arginfo.mountpoint as *mut libc::c_void);
    hostnames = read_hostnames(arginfo.hostfile as *const libc::c_char, &mut list);
    free(arginfo.hostfile as *mut libc::c_void);
    if hostnames == 0 {
        fprintf(
            stderr,
            b"No hosts configured! Exiting\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    tmp___0 = net_open_sockets();
    if tmp___0 != 0 {
        fprintf(
            stderr,
            b"No raw sockets opened. Got root?\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    host_count = resolve_names(list, hostnames, &mut hosts);
    if host_count < 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    host_count = host_evaluate(&mut hosts, host_count, arginfo.timeout);
    if host_count == 0 {
        fprintf(
            stderr,
            b"No host passed the test\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    chunk_set_timeout(arginfo.timeout);
    host_use(hosts);
    fuse_opt_add_arg(&mut args, b"-f\0" as *const u8 as *const libc::c_char);
    fuse_opt_add_arg(&mut args, b"-s\0" as *const u8 as *const libc::c_char);
    fuse_opt_add_arg(
        &mut args,
        b"-odefault_permissions,allow_other\0" as *const u8 as *const libc::c_char,
    );
    fuse_opt_add_arg(&mut args, b"-odirect_io\0" as *const u8 as *const libc::c_char);
    printf(b"Mounting filesystem\n\0" as *const u8 as *const libc::c_char);
    fuse_main_real(
        args.argc,
        args.argv,
        &fs_ops,
        ::std::mem::size_of::<fuse_operations>() as libc::c_ulong,
        0 as *mut libc::c_void,
    );
    fuse_opt_free_args(&mut args);
    h = hosts;
    while !h.is_null() {
        host = h;
        h = (*h).next;
        free(host as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
pub static mut files: *mut file = 0 as *const file as *mut file;
unsafe extern "C" fn fs_free(mut f: *mut file) {
    let mut c: *mut chunk = 0 as *mut chunk;
    let mut next: *mut chunk = 0 as *mut chunk;
    c = (*f).chunks;
    while !c.is_null() {
        next = (*c).next_file;
        chunk_remove(c);
        chunk_free(c);
        c = next;
    }
    free((*f).name as *mut libc::c_void);
    free(f as *mut libc::c_void);
}
unsafe extern "C" fn file_size(mut f: *mut file) -> size_t {
    let mut c: *mut chunk = 0 as *mut chunk;
    let mut size: size_t = 0;
    size = 0 as libc::c_int as size_t;
    c = (*f).chunks;
    while !c.is_null() {
        size = (size as libc::c_ulong).wrapping_add((*c).len as size_t) as size_t
            as size_t;
        c = (*c).next_file;
    }
    return size;
}
unsafe extern "C" fn fs_init(mut conn: *mut fuse_conn_info) -> *mut libc::c_void {
    net_start();
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn fs_destroy(mut data: *mut libc::c_void) {
    let mut f: *mut file = 0 as *mut file;
    let mut next: *mut file = 0 as *mut file;
    net_stop();
    f = files;
    while !f.is_null() {
        next = (*f).next;
        fs_free(f);
        f = next;
    }
}
unsafe extern "C" fn find_file(mut name: *const libc::c_char) -> *mut file {
    let mut f: *mut file = 0 as *mut file;
    let mut tmp: libc::c_int = 0;
    f = files;
    while !f.is_null() {
        tmp = strcmp(name, (*f).name);
        if tmp == 0 as libc::c_int {
            return f;
        }
        f = (*f).next;
    }
    return 0 as *mut libc::c_void as *mut file;
}
unsafe extern "C" fn fs_mkdir(
    mut name: *const libc::c_char,
    mut mode: mode_t,
) -> libc::c_int {
    return -(95 as libc::c_int);
}
unsafe extern "C" fn fs_mknod(
    mut name: *const libc::c_char,
    mut mode: mode_t,
    mut device: dev_t,
) -> libc::c_int {
    let mut f: *mut file = 0 as *mut file;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(mode & 61440 as libc::c_uint == 32768 as libc::c_uint) {
        return -(95 as libc::c_int);
    }
    f = find_file(name);
    if !f.is_null() {
        return -(17 as libc::c_int);
    }
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<file>() as libc::c_ulong,
    );
    f = tmp as *mut file;
    if f.is_null() {
        tmp___0 = __errno_location();
        return -*tmp___0;
    }
    tmp___1 = strdup(name);
    (*f).name = tmp___1 as *const libc::c_char;
    (*f).mode = mode;
    (*f).next = files;
    files = f;
    return 0 as libc::c_int;
}
unsafe extern "C" fn fs_chmod(
    mut name: *const libc::c_char,
    mut mode: mode_t,
) -> libc::c_int {
    let mut f: *mut file = 0 as *mut file;
    f = find_file(name);
    if f.is_null() {
        return -(2 as libc::c_int);
    }
    (*f).mode = mode;
    return 0 as libc::c_int;
}
unsafe extern "C" fn fs_utime(
    mut name: *const libc::c_char,
    mut utim: *mut utimbuf,
) -> libc::c_int {
    let mut f: *mut file = 0 as *mut file;
    f = find_file(name);
    if f.is_null() {
        return -(2 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fs_getattr(
    mut name: *const libc::c_char,
    mut stat___0: *mut stat,
) -> libc::c_int {
    let mut f: *mut file = 0 as *mut file;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: size_t = 0;
    (*stat___0).st_nlink = 1 as libc::c_int as __nlink_t;
    (*stat___0).st_uid = getuid();
    (*stat___0).st_gid = getgid();
    (*stat___0).st_atim.tv_sec = 0 as libc::c_int as __time_t;
    (*stat___0).st_mtim.tv_sec = 0 as libc::c_int as __time_t;
    (*stat___0).st_ctim.tv_sec = 0 as libc::c_int as __time_t;
    tmp = strcmp(b"/\0" as *const u8 as *const libc::c_char, name);
    if tmp == 0 as libc::c_int {
        (*stat___0).st_mode = 16893 as libc::c_int as __mode_t;
        (*stat___0).st_size = 0 as libc::c_int as __off_t;
        (*stat___0).st_blksize = 0 as libc::c_int as __blksize_t;
        (*stat___0).st_blocks = 0 as libc::c_int as __blkcnt_t;
        return 0 as libc::c_int;
    }
    f = find_file(name);
    if f.is_null() {
        return -(2 as libc::c_int);
    }
    (*stat___0).st_mode = (*f).mode;
    tmp___0 = file_size(f);
    (*stat___0).st_size = tmp___0 as __off_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn fs_unlink(mut name: *const libc::c_char) -> libc::c_int {
    let mut f: *mut file = 0 as *mut file;
    let mut last: *mut file = 0 as *mut file;
    let mut tmp: libc::c_int = 0;
    f = files;
    last = 0 as *mut libc::c_void as *mut file;
    while !f.is_null() {
        tmp = strcmp(name, (*f).name);
        if tmp == 0 as libc::c_int {
            if !last.is_null() {
                (*last).next = (*f).next;
            } else {
                files = (*f).next;
            }
            fs_free(f);
            return 0 as libc::c_int;
        }
        last = f;
        f = (*f).next;
    }
    return -(2 as libc::c_int);
}
unsafe extern "C" fn fs_readdir(
    mut path: *const libc::c_char,
    mut buf: *mut libc::c_void,
    mut filler: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            *const stat,
            off_t,
        ) -> libc::c_int,
    >,
    mut offset: off_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut f: *mut file = 0 as *mut file;
    let mut tmp: libc::c_int = 0;
    tmp = strcmp(b"/\0" as *const u8 as *const libc::c_char, path);
    if tmp != 0 {
        return -(2 as libc::c_int);
    }
    (Some(filler.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        buf,
        b".\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_void as *const stat,
        0 as libc::c_int as off_t,
    );
    (Some(filler.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        buf,
        b"..\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_void as *const stat,
        0 as libc::c_int as off_t,
    );
    f = files;
    while !f.is_null() {
        (Some(filler.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            buf,
            ((*f).name).offset(1 as libc::c_int as isize),
            0 as *mut libc::c_void as *const stat,
            0 as libc::c_int as off_t,
        );
        f = (*f).next;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fs_open(
    mut name: *const libc::c_char,
    mut fileinfo: *mut fuse_file_info,
) -> libc::c_int {
    let mut f: *mut file = 0 as *mut file;
    f = find_file(name);
    if f.is_null() {
        return -(2 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fs_inner_write(
    mut f: *mut file,
    mut buf: *const libc::c_char,
    mut size: size_t,
    mut offset: off_t,
) -> libc::c_int {
    let mut c: *mut chunk = 0 as *mut chunk;
    let mut last: *mut chunk = 0 as *mut chunk;
    let mut chunkdata: *mut uint8_t = 0 as *mut uint8_t;
    let mut len: libc::c_int = 0;
    let mut clen: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    last = 0 as *mut libc::c_void as *mut chunk;
    c = (*f).chunks;
    while !c.is_null() {
        if !(offset >= (*c).len as off_t) {
            break;
        }
        if (*c).len as libc::c_int != 1024 as libc::c_int {
            break;
        }
        offset -= (*c).len as off_t;
        last = c;
        c = (*c).next_file;
    }
    if c.is_null() {
        c = chunk_create();
        if size < 1024 as libc::c_ulong {
            (*c).len = size as uint16_t;
        } else {
            (*c).len = 1024 as libc::c_int as uint16_t;
        }
        chunk_add(c);
        if !last.is_null() {
            (*last).next_file = c;
        } else {
            (*f).chunks = c;
        }
        (*c).host = host_get_next();
        net_send(
            (*c).host,
            (*c).id,
            (*c).seqno,
            buf as *const uint8_t,
            (*c).len as size_t,
        );
        return (*c).len as libc::c_int;
    }
    chunkdata = 0 as *mut libc::c_void as *mut uint8_t;
    clen = chunk_wait_for(c, &mut chunkdata);
    if clen <= 0 as libc::c_int {
        return clen;
    }
    if (1024 as libc::c_ulong) < size.wrapping_add(offset as size_t) {
        clen = 1024 as libc::c_int;
    } else {
        clen = size.wrapping_add(offset as size_t) as libc::c_int;
    }
    if ((clen as off_t - offset) as size_t) < size {
        len = (clen as off_t - offset) as libc::c_int;
    } else {
        len = size as libc::c_int;
    }
    tmp = realloc(chunkdata as *mut libc::c_void, clen as size_t);
    chunkdata = tmp as *mut uint8_t;
    memcpy(
        chunkdata.offset(offset as isize) as *mut libc::c_void,
        buf as *const libc::c_void,
        len as size_t,
    );
    chunk_done(c, chunkdata, clen as size_t);
    return len;
}
unsafe extern "C" fn fs_write(
    mut name: *const libc::c_char,
    mut buf: *const libc::c_char,
    mut size: size_t,
    mut offset: off_t,
    mut fileinfo: *mut fuse_file_info,
) -> libc::c_int {
    let mut f: *mut file = 0 as *mut file;
    let mut tmp: libc::c_int = 0;
    f = find_file(name);
    if f.is_null() {
        return -(2 as libc::c_int);
    }
    tmp = fs_inner_write(f, buf, size, offset);
    return tmp;
}
unsafe extern "C" fn fs_read(
    mut name: *const libc::c_char,
    mut buf: *mut libc::c_char,
    mut size: size_t,
    mut offset: off_t,
    mut fileinfo: *mut fuse_file_info,
) -> libc::c_int {
    let mut f: *mut file = 0 as *mut file;
    let mut c: *mut chunk = 0 as *mut chunk;
    let mut chunkdata: *mut uint8_t = 0 as *mut uint8_t;
    let mut len: libc::c_int = 0;
    let mut clen: libc::c_int = 0;
    f = find_file(name);
    if f.is_null() {
        return -(2 as libc::c_int);
    }
    c = (*f).chunks;
    while !c.is_null() {
        if !(offset >= (*c).len as off_t) {
            break;
        }
        offset -= (*c).len as off_t;
        c = (*c).next_file;
    }
    if c.is_null() {
        return 0 as libc::c_int;
    }
    if (((*c).len as off_t - offset) as size_t) < size {
        len = ((*c).len as off_t - offset) as libc::c_int;
    } else {
        len = size as libc::c_int;
    }
    chunkdata = 0 as *mut libc::c_void as *mut uint8_t;
    clen = chunk_wait_for(c, &mut chunkdata);
    if clen == 0 {
        return -(5 as libc::c_int);
    }
    memcpy(
        buf as *mut libc::c_void,
        chunkdata.offset(offset as isize) as *const libc::c_void,
        len as size_t,
    );
    chunk_done(c, chunkdata, clen as size_t);
    return len;
}
unsafe extern "C" fn shrink_file(mut f: *mut file, mut length: off_t) -> libc::c_int {
    let mut c: *mut chunk = 0 as *mut chunk;
    let mut prev: *mut chunk = 0 as *mut chunk;
    let mut next: *mut chunk = 0 as *mut chunk;
    let mut cdata: *mut uint8_t = 0 as *mut uint8_t;
    let mut clen: libc::c_int = 0;
    c = (*f).chunks;
    prev = 0 as *mut libc::c_void as *mut chunk;
    while (*c).len as off_t <= length {
        length -= (*c).len as off_t;
        prev = c;
        c = (*c).next_file;
    }
    if length == 0 {
        if !prev.is_null() {
            (*prev).next_file = 0 as *mut libc::c_void as *mut chunk;
        } else {
            (*f).chunks = 0 as *mut libc::c_void as *mut chunk;
        }
    }
    while !c.is_null() {
        next = (*c).next_file;
        if length != 0 {
            clen = chunk_wait_for(c, &mut cdata);
            if clen == 0 {
                return -(5 as libc::c_int);
            }
            chunk_done(c, cdata, length as size_t);
            (*c).next_file = 0 as *mut libc::c_void as *mut chunk;
            length = 0 as libc::c_int as off_t;
        } else {
            chunk_remove(c);
            chunk_free(c);
        }
        c = next;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn grow_file(mut f: *mut file, mut length: off_t) -> libc::c_int {
    let mut offset: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut to_grow: libc::c_int = 0;
    let mut zerobuf: [libc::c_char; 1024] = [0; 1024];
    let mut res: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    tmp = file_size(f);
    offset = tmp as libc::c_int;
    to_grow = (length - offset as off_t) as libc::c_int;
    memset(
        zerobuf.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    while to_grow != 0 {
        if to_grow < 1024 as libc::c_int {
            tmp___0 = to_grow;
        } else {
            tmp___0 = 1024 as libc::c_int;
        }
        tmp___1 = fs_inner_write(
            f,
            zerobuf.as_mut_ptr() as *const libc::c_char,
            tmp___0 as size_t,
            offset as off_t,
        );
        res = tmp___1;
        if res < 0 as libc::c_int {
            return res;
        }
        if res == 0 {
            __assert_fail(
                b"res\0" as *const u8 as *const libc::c_char,
                b"fs.c\0" as *const u8 as *const libc::c_char,
                379 as libc::c_uint,
                b"grow_file\0" as *const u8 as *const libc::c_char,
            );
        }
        offset += res;
        to_grow -= res;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fs_truncate(
    mut name: *const libc::c_char,
    mut length: off_t,
) -> libc::c_int {
    let mut f: *mut file = 0 as *mut file;
    let mut cur_size: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    f = find_file(name);
    if f.is_null() {
        return -(2 as libc::c_int);
    }
    tmp = file_size(f);
    cur_size = tmp as libc::c_int;
    if length > cur_size as off_t {
        tmp___0 = grow_file(f, length);
        return tmp___0;
    }
    if length < cur_size as off_t {
        tmp___1 = shrink_file(f, length);
        return tmp___1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fs_rename(
    mut name: *const libc::c_char,
    mut newname: *const libc::c_char,
) -> libc::c_int {
    let mut f: *mut file = 0 as *mut file;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    f = find_file(name);
    if f.is_null() {
        return -(2 as libc::c_int);
    }
    free((*f).name as *mut libc::c_void);
    tmp = strdup(newname);
    (*f).name = tmp as *const libc::c_char;
    return 0 as libc::c_int;
}
pub static mut fs_ops: fuse_operations = fuse_operations {
    getattr: None,
    readlink: None,
    getdir: None,
    mknod: None,
    mkdir: None,
    unlink: None,
    rmdir: None,
    symlink: None,
    rename: None,
    link: None,
    chmod: None,
    chown: None,
    truncate: None,
    utime: None,
    open: None,
    read: None,
    write: None,
    statfs: None,
    flush: None,
    release: None,
    fsync: None,
    setxattr: None,
    getxattr: None,
    listxattr: None,
    removexattr: None,
    opendir: None,
    readdir: None,
    releasedir: None,
    fsyncdir: None,
    init: None,
    destroy: None,
    access: None,
    create: None,
    ftruncate: None,
    fgetattr: None,
    lock: None,
    utimens: None,
    bmap: None,
    flag_nullpath_ok_flag_nopath_flag_utime_omit_ok_flag_reserved: [0; 4],
    c2rust_padding: [0; 4],
    ioctl: None,
    poll: None,
    write_buf: None,
    read_buf: None,
    flock: None,
    fallocate: None,
};
static mut sockv4: libc::c_int = 0;
static mut sockv6: libc::c_int = 0;
static mut netdata: net_data = net_data {
    responder: 0,
    status: 0,
    stats_mutex: __anonunion_pthread_mutex_t_335460617 {
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
    tx: pkt_stats { packets: 0, bytes: 0 },
    rx: pkt_stats { packets: 0, bytes: 0 },
};
unsafe extern "C" fn inc_stats(mut stats: *mut pkt_stats, mut packetsize: libc::c_int) {
    pthread_mutex_lock(&mut netdata.stats_mutex);
    (*stats).packets = ((*stats).packets).wrapping_add(1);
    (*stats)
        .bytes = ((*stats).bytes)
        .wrapping_add((packetsize + 8 as libc::c_int) as libc::c_ulonglong);
    pthread_mutex_unlock(&mut netdata.stats_mutex);
}
unsafe extern "C" fn net_inc_tx(mut packetsize: libc::c_int) {
    inc_stats(&mut netdata.tx, packetsize);
}
pub unsafe extern "C" fn net_inc_rx(mut packetsize: libc::c_int) {
    inc_stats(&mut netdata.rx, packetsize);
}
pub unsafe extern "C" fn net_open_sockets() -> libc::c_int {
    let mut rcvbuf: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut filter: icmp6_filter = icmp6_filter { icmp6_filt: [0; 8] };
    let mut res___0: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    rcvbuf = 1048576 as libc::c_int;
    sockv4 = socket(2 as libc::c_int, 3 as libc::c_int, 1 as libc::c_int);
    if sockv4 < 0 as libc::c_int {
        perror(b"Failed to open IPv4 socket\0" as *const u8 as *const libc::c_char);
    } else {
        tmp = setsockopt(
            sockv4,
            1 as libc::c_int,
            8 as libc::c_int,
            &mut rcvbuf as *mut libc::c_int as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
        );
        res = tmp;
        if res < 0 as libc::c_int {
            perror(
                b"Failed to set receive buffer size on IPv4 socket\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    sockv6 = socket(10 as libc::c_int, 3 as libc::c_int, 58 as libc::c_int);
    if sockv6 >= 0 as libc::c_int {
        tmp___0 = setsockopt(
            sockv6,
            1 as libc::c_int,
            8 as libc::c_int,
            &mut rcvbuf as *mut libc::c_int as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
        );
        res___0 = tmp___0;
        if res___0 < 0 as libc::c_int {
            perror(
                b"Failed to set receive buffer size on IPv6 socket\0" as *const u8
                    as *const libc::c_char,
            );
        }
        memset(
            &mut filter as *mut icmp6_filter as *mut libc::c_void,
            255 as libc::c_int,
            ::std::mem::size_of::<icmp6_filter>() as libc::c_ulong,
        );
        filter.icmp6_filt[(129 as libc::c_int >> 5 as libc::c_int) as usize]
            &= !((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint;
        res___0 = setsockopt(
            sockv6,
            58 as libc::c_int,
            1 as libc::c_int,
            &mut filter as *mut icmp6_filter as *const libc::c_void,
            ::std::mem::size_of::<icmp6_filter>() as libc::c_ulong as socklen_t,
        );
        if res___0 < 0 as libc::c_int {
            perror(
                b"Failed to set ICMP filters on IPv6 socket\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else {
        perror(b"Failed to open IPv6 socket\0" as *const u8 as *const libc::c_char);
    }
    if sockv4 < 0 as libc::c_int {
        if sockv6 < 0 as libc::c_int {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn net_send(
    mut host: *mut host,
    mut id: uint16_t,
    mut seqno: uint16_t,
    mut data: *const uint8_t,
    mut len: size_t,
) {
    let mut sock: libc::c_int = 0;
    let mut pkt: icmp_packet = icmp_packet {
        peer: sockaddr_storage {
            ss_family: 0,
            __ss_padding: [0; 118],
            __ss_align: 0,
        },
        peer_len: 0,
        type_0: ICMP_REQUEST,
        id: 0,
        seqno: 0,
        payload: 0 as *mut uint8_t,
        payload_len: 0,
    };
    let mut tmp: libc::c_int = 0;
    memcpy(
        &mut pkt.peer as *mut sockaddr_storage as *mut libc::c_void,
        &mut (*host).sockaddr as *mut sockaddr_storage as *const libc::c_void,
        (*host).sockaddr_len as size_t,
    );
    pkt.peer_len = (*host).sockaddr_len;
    pkt.type_0 = ICMP_REQUEST;
    pkt.id = id;
    pkt.seqno = seqno;
    pkt.payload = data as *mut uint8_t;
    pkt.payload_len = len as uint32_t;
    if pkt.peer.ss_family as libc::c_int == 2 as libc::c_int {
        sock = sockv4;
    } else {
        sock = sockv6;
    }
    if sock >= 0 as libc::c_int {
        net_inc_tx(pkt.payload_len as libc::c_int);
        tmp = icmp_send(sock, &mut pkt);
        if tmp == 0 {
            perror(b"Failed sending data packet\0" as *const u8 as *const libc::c_char);
        }
    }
}
unsafe extern "C" fn handle_recv(
    mut sock: libc::c_int,
    mut recv_fn: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut sockaddr_storage,
            size_t,
            uint16_t,
            uint16_t,
            *mut *mut uint8_t,
            size_t,
        ) -> (),
    >,
    mut recv_data: *mut libc::c_void,
) {
    let mut mypkt: icmp_packet = icmp_packet {
        peer: sockaddr_storage {
            ss_family: 0,
            __ss_padding: [0; 118],
            __ss_align: 0,
        },
        peer_len: 0,
        type_0: ICMP_REQUEST,
        id: 0,
        seqno: 0,
        payload: 0 as *mut uint8_t,
        payload_len: 0,
    };
    let mut buf: [uint8_t; 8192] = [0; 8192];
    let mut len: libc::c_int = 0;
    let mut tmp: ssize_t = 0;
    let mut tmp___0: libc::c_int = 0;
    mypkt
        .peer_len = ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong
        as socklen_t;
    tmp = recvfrom(
        sock,
        buf.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[uint8_t; 8192]>() as libc::c_ulong,
        0 as libc::c_int,
        &mut mypkt.peer as *mut sockaddr_storage as *mut sockaddr,
        &mut mypkt.peer_len as *mut socklen_t,
    );
    len = tmp as libc::c_int;
    if len > 0 as libc::c_int {
        tmp___0 = icmp_parse(&mut mypkt, buf.as_mut_ptr(), len);
        if tmp___0 == 0 as libc::c_int {
            if mypkt.type_0 as libc::c_uint == 1 as libc::c_uint {
                (Some(recv_fn.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    recv_data,
                    &mut mypkt.peer,
                    mypkt.peer_len as size_t,
                    mypkt.id,
                    mypkt.seqno,
                    &mut mypkt.payload,
                    mypkt.payload_len as size_t,
                );
            }
            free(mypkt.payload as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn net_recv(
    mut tv: *mut timeval,
    mut recv_fn: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut sockaddr_storage,
            size_t,
            uint16_t,
            uint16_t,
            *mut *mut uint8_t,
            size_t,
        ) -> (),
    >,
    mut recv_data: *mut libc::c_void,
) -> libc::c_int {
    let mut maxfd: libc::c_int = 0;
    let mut fds: fd_set = fd_set { fds_bits: [0; 16] };
    let mut i: libc::c_int = 0;
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh3 = &mut __d0;
    let fresh4;
    let fresh5 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh6 = &mut __d1;
    let fresh7;
    let fresh8 = &mut *(fds.fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh3,
        fresh5) => fresh4, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh6,
        fresh8) => fresh7, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
    c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
    if sockv4 >= 0 as libc::c_int {
        fds
            .fds_bits[(sockv4
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << sockv4
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
    }
    if sockv6 >= 0 as libc::c_int {
        fds
            .fds_bits[(sockv6
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << sockv6
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
    }
    if sockv4 > sockv6 {
        maxfd = sockv4;
    } else {
        maxfd = sockv6;
    }
    i = select(
        maxfd + 1 as libc::c_int,
        &mut fds as *mut fd_set,
        0 as *mut libc::c_void as *mut fd_set,
        0 as *mut libc::c_void as *mut fd_set,
        tv,
    );
    if sockv4 >= 0 as libc::c_int {
        if fds
            .fds_bits[(sockv4
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            & ((1 as libc::c_ulong)
                << sockv4
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask != 0 as libc::c_long
        {
            handle_recv(sockv4, recv_fn, recv_data);
        }
    }
    if sockv6 >= 0 as libc::c_int {
        if fds
            .fds_bits[(sockv6
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            & ((1 as libc::c_ulong)
                << sockv6
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask != 0 as libc::c_long
        {
            handle_recv(sockv6, recv_fn, recv_data);
        }
    }
    return i;
}
unsafe extern "C" fn responder_thread(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    loop {
        tv.tv_sec = 1 as libc::c_int as __time_t;
        tv.tv_usec = 0 as libc::c_int as __suseconds_t;
        net_recv(
            &mut tv,
            Some(
                chunk_reply
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut sockaddr_storage,
                        size_t,
                        uint16_t,
                        uint16_t,
                        *mut *mut uint8_t,
                        size_t,
                    ) -> (),
            ),
            0 as *mut libc::c_void,
        );
    };
}
unsafe extern "C" fn get_stats(mut rx: *mut pkt_stats, mut tx: *mut pkt_stats) {
    pthread_mutex_lock(&mut netdata.stats_mutex);
    memcpy(
        rx as *mut libc::c_void,
        &mut netdata.rx as *mut pkt_stats as *const libc::c_void,
        ::std::mem::size_of::<pkt_stats>() as libc::c_ulong,
    );
    memcpy(
        tx as *mut libc::c_void,
        &mut netdata.tx as *mut pkt_stats as *const libc::c_void,
        ::std::mem::size_of::<pkt_stats>() as libc::c_ulong,
    );
    pthread_mutex_unlock(&mut netdata.stats_mutex);
}
unsafe extern "C" fn format_bytes(
    mut bytes: libc::c_ulonglong,
    mut suffix: *mut *const libc::c_char,
) -> libc::c_float {
    let mut suffixes: [*const libc::c_char; 5] = [0 as *const libc::c_char; 5];
    let mut i: libc::c_int = 0;
    let mut bps: libc::c_float = 0.;
    suffixes[0 as libc::c_int as usize] = b"B\0" as *const u8 as *const libc::c_char;
    suffixes[1 as libc::c_int as usize] = b"kB\0" as *const u8 as *const libc::c_char;
    suffixes[2 as libc::c_int as usize] = b"MB\0" as *const u8 as *const libc::c_char;
    suffixes[3 as libc::c_int as usize] = b"GB\0" as *const u8 as *const libc::c_char;
    suffixes[4 as libc::c_int as usize] = 0 as *mut libc::c_void as *const libc::c_char;
    i = 0 as libc::c_int;
    bps = bytes as libc::c_float;
    while !(suffixes[(i + 1 as libc::c_int) as usize]).is_null() {
        if !(bps > 1300 as libc::c_int as libc::c_float) {
            break;
        }
        bps /= 1000.0f32;
        i += 1;
    }
    *suffix = suffixes[i as usize];
    return bps;
}
unsafe extern "C" fn diff_stats(mut new: *mut pkt_stats, mut old: *mut pkt_stats) {
    let mut diff: pkt_stats = pkt_stats { packets: 0, bytes: 0 };
    let mut bytes: libc::c_float = 0.;
    let mut byte_suffix: *const libc::c_char = 0 as *const libc::c_char;
    diff.packets = ((*new).packets).wrapping_sub((*old).packets);
    diff.bytes = ((*new).bytes).wrapping_sub((*old).bytes);
    memcpy(
        old as *mut libc::c_void,
        new as *const libc::c_void,
        ::std::mem::size_of::<pkt_stats>() as libc::c_ulong,
    );
    bytes = format_bytes(diff.bytes, &mut byte_suffix);
    printf(
        b"%6llu pkt/s, %7.01f %2s/s\0" as *const u8 as *const libc::c_char,
        diff.packets,
        bytes as libc::c_double,
        byte_suffix,
    );
}
static mut prev_rx: pkt_stats = pkt_stats { packets: 0, bytes: 0 };
static mut prev_tx: pkt_stats = pkt_stats { packets: 0, bytes: 0 };
unsafe extern "C" fn status_thread(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut status_sleep: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut rx: pkt_stats = pkt_stats { packets: 0, bytes: 0 };
    let mut tx: pkt_stats = pkt_stats { packets: 0, bytes: 0 };
    status_sleep.tv_sec = 1 as libc::c_int as __time_t;
    status_sleep.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
    get_stats(&mut prev_rx, &mut prev_tx);
    nanosleep(
        &mut status_sleep as *mut timespec as *const timespec,
        0 as *mut libc::c_void as *mut timespec,
    );
    loop {
        get_stats(&mut rx, &mut tx);
        printf(b"\rICMP in: \0" as *const u8 as *const libc::c_char);
        diff_stats(&mut rx, &mut prev_rx);
        printf(b"    ICMP out: \0" as *const u8 as *const libc::c_char);
        diff_stats(&mut tx, &mut prev_tx);
        fflush(stdout);
        nanosleep(
            &mut status_sleep as *mut timespec as *const timespec,
            0 as *mut libc::c_void as *mut timespec,
        );
    };
}
pub unsafe extern "C" fn net_start() {
    let mut tmp: libc::c_int = 0;
    tmp = pthread_mutex_init(
        &mut netdata.stats_mutex,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
    if tmp != 0 {
        perror(b"Fatal, failed to create a mutex\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    pthread_create(
        &mut netdata.responder as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(
            responder_thread
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        0 as *mut libc::c_void,
    );
    pthread_create(
        &mut netdata.status as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(
            status_thread as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        0 as *mut libc::c_void,
    );
}
pub unsafe extern "C" fn net_stop() {
    pthread_cancel(netdata.responder);
    pthread_join(netdata.responder, 0 as *mut libc::c_void as *mut *mut libc::c_void);
    pthread_cancel(netdata.status);
    pthread_join(netdata.status, 0 as *mut libc::c_void as *mut *mut libc::c_void);
    pthread_mutex_lock(&mut netdata.stats_mutex);
    printf(
        b"\n\nTotal network resources consumed:\nin:  %10llu packets, %10llu bytes\nout: %10llu packets, %10llu bytes\n (bytes counted above IP level)\n\0"
            as *const u8 as *const libc::c_char,
        netdata.rx.packets,
        netdata.rx.bytes,
        netdata.tx.packets,
        netdata.tx.bytes,
    );
    pthread_mutex_unlock(&mut netdata.stats_mutex);
}
static mut icmp_id: uint16_t = 0;
static mut timeout: libc::c_int = 0;
static mut chunk_head: *mut chunk = 0 as *const chunk as *mut chunk;
static mut chunk_mutex: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
pub unsafe extern "C" fn chunk_set_timeout(mut t: libc::c_int) {
    timeout = t;
}
pub unsafe extern "C" fn chunk_create() -> *mut chunk {
    let mut c: *mut chunk = 0 as *mut chunk;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: uint16_t = 0;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<chunk>() as libc::c_ulong,
    );
    c = tmp as *mut chunk;
    if c.is_null() {
        return 0 as *mut libc::c_void as *mut chunk;
    }
    tmp___0 = icmp_id;
    icmp_id = (icmp_id as libc::c_int + 1 as libc::c_int) as uint16_t;
    (*c).id = tmp___0;
    return c;
}
pub unsafe extern "C" fn chunk_free(mut c: *mut chunk) {
    free(c as *mut libc::c_void);
}
pub unsafe extern "C" fn chunk_add(mut c: *mut chunk) {
    pthread_mutex_lock(&mut chunk_mutex);
    (*c).next_active = chunk_head;
    chunk_head = c;
    pthread_mutex_unlock(&mut chunk_mutex);
}
pub unsafe extern "C" fn chunk_remove(mut c: *mut chunk) {
    let mut prev: *mut chunk = 0 as *mut chunk;
    let mut curr: *mut chunk = 0 as *mut chunk;
    prev = 0 as *mut libc::c_void as *mut chunk;
    pthread_mutex_lock(&mut chunk_mutex);
    curr = chunk_head;
    while !curr.is_null() {
        if curr as libc::c_ulong == c as libc::c_ulong {
            if !prev.is_null() {
                (*prev).next_active = (*curr).next_active;
            } else {
                chunk_head = (*curr).next_active;
            }
            break;
        } else {
            prev = curr;
            curr = (*curr).next_active;
        }
    }
    pthread_mutex_unlock(&mut chunk_mutex);
}
unsafe extern "C" fn process_chunk(mut c: *mut chunk, mut data: *mut *mut uint8_t) {
    let mut io: *mut io = 0 as *mut io;
    (*c).seqno = ((*c).seqno as libc::c_int + 1 as libc::c_int) as uint16_t;
    if !((*c).io).is_null() {
        io = (*c).io;
        pthread_mutex_lock(&mut (*io).mutex);
        (*io).data = *data;
        (*io).len = (*c).len as size_t;
        (*io).owner = OWNER_FS;
        pthread_cond_signal(&mut (*io).fs_cond);
        while (*io).owner as libc::c_uint != 2 as libc::c_uint {
            pthread_cond_wait(
                &mut (*io).net_cond as *mut pthread_cond_t,
                &mut (*io).mutex as *mut pthread_mutex_t,
            );
        }
        *data = (*io).data;
        pthread_mutex_unlock(&mut (*io).mutex);
        free((*c).io as *mut libc::c_void);
        (*c).io = 0 as *mut libc::c_void as *mut io;
    }
    net_send(
        (*c).host,
        (*c).id,
        (*c).seqno,
        *data as *const uint8_t,
        (*c).len as size_t,
    );
}
pub unsafe extern "C" fn chunk_reply(
    mut userdata: *mut libc::c_void,
    mut addr: *mut sockaddr_storage,
    mut addrlen: size_t,
    mut id: uint16_t,
    mut seqno: uint16_t,
    mut data: *mut *mut uint8_t,
    mut len: size_t,
) {
    let mut c: *mut chunk = 0 as *mut chunk;
    pthread_mutex_lock(&mut chunk_mutex);
    c = chunk_head;
    while !c.is_null() {
        if (*c).id as libc::c_int == id as libc::c_int {
            net_inc_rx(len as libc::c_int);
            if len == (*c).len as size_t {
                if seqno as libc::c_int == (*c).seqno as libc::c_int {
                    process_chunk(c, data);
                }
            }
            break;
        } else {
            c = (*c).next_active;
        }
    }
    pthread_mutex_unlock(&mut chunk_mutex);
}
pub unsafe extern "C" fn chunk_wait_for(
    mut c: *mut chunk,
    mut data: *mut *mut uint8_t,
) -> libc::c_int {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    pthread_mutex_lock(&mut chunk_mutex);
    if !((*c).io).is_null() {
        pthread_mutex_unlock(&mut chunk_mutex);
        return -(16 as libc::c_int);
    }
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<io>() as libc::c_ulong,
    );
    (*c).io = tmp as *mut io;
    if ((*c).io).is_null() {
        pthread_mutex_unlock(&mut chunk_mutex);
        return -(12 as libc::c_int);
    }
    pthread_mutex_unlock(&mut chunk_mutex);
    (*(*c).io).owner = OWNER_NET;
    pthread_cond_init(
        &mut (*(*c).io).fs_cond as *mut pthread_cond_t,
        0 as *mut libc::c_void as *const pthread_condattr_t,
    );
    pthread_cond_init(
        &mut (*(*c).io).net_cond as *mut pthread_cond_t,
        0 as *mut libc::c_void as *const pthread_condattr_t,
    );
    tmp___1 = pthread_mutex_init(
        &mut (*(*c).io).mutex,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
    if tmp___1 != 0 {
        pthread_mutex_unlock(&mut chunk_mutex);
        free((*c).io as *mut libc::c_void);
        tmp___0 = __errno_location();
        return -*tmp___0;
    }
    pthread_mutex_lock(&mut (*(*c).io).mutex);
    while (*(*c).io).owner as libc::c_uint != 1 as libc::c_uint {
        clock_gettime(0 as libc::c_int, &mut ts);
        ts.tv_sec += timeout as __time_t;
        res = pthread_cond_timedwait(
            &mut (*(*c).io).fs_cond as *mut pthread_cond_t,
            &mut (*(*c).io).mutex as *mut pthread_mutex_t,
            &mut ts as *mut timespec as *const timespec,
        );
        if res != 0 {
            pthread_mutex_unlock(&mut (*(*c).io).mutex);
            pthread_mutex_lock(&mut chunk_mutex);
            free((*c).io as *mut libc::c_void);
            (*c).io = 0 as *mut libc::c_void as *mut io;
            pthread_mutex_unlock(&mut chunk_mutex);
            return 0 as libc::c_int;
        }
    }
    *data = (*(*c).io).data;
    return (*(*c).io).len as libc::c_int;
}
pub unsafe extern "C" fn chunk_done(
    mut c: *mut chunk,
    mut data: *mut uint8_t,
    mut len: size_t,
) {
    (*(*c).io).data = data;
    (*(*c).io).len = len;
    (*c).len = (*(*c).io).len as uint16_t;
    (*(*c).io).owner = OWNER_NET;
    pthread_cond_signal(&mut (*(*c).io).net_cond);
    pthread_mutex_unlock(&mut (*(*c).io).mutex);
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
    fs_ops = {
        let mut init = fuse_operations {
            flag_nullpath_ok_flag_nopath_flag_utime_omit_ok_flag_reserved: [0; 4],
            c2rust_padding: [0; 4],
            getattr: Some(
                fs_getattr
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut stat,
                    ) -> libc::c_int,
            ),
            readlink: None,
            getdir: None,
            mknod: Some(
                fs_mknod
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        mode_t,
                        dev_t,
                    ) -> libc::c_int,
            ),
            mkdir: Some(
                fs_mkdir
                    as unsafe extern "C" fn(*const libc::c_char, mode_t) -> libc::c_int,
            ),
            unlink: Some(
                fs_unlink as unsafe extern "C" fn(*const libc::c_char) -> libc::c_int,
            ),
            rmdir: None,
            symlink: None,
            rename: Some(
                fs_rename
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *const libc::c_char,
                    ) -> libc::c_int,
            ),
            link: None,
            chmod: Some(
                fs_chmod
                    as unsafe extern "C" fn(*const libc::c_char, mode_t) -> libc::c_int,
            ),
            chown: None,
            truncate: Some(
                fs_truncate
                    as unsafe extern "C" fn(*const libc::c_char, off_t) -> libc::c_int,
            ),
            utime: Some(
                fs_utime
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut utimbuf,
                    ) -> libc::c_int,
            ),
            open: Some(
                fs_open
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut fuse_file_info,
                    ) -> libc::c_int,
            ),
            read: Some(
                fs_read
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut libc::c_char,
                        size_t,
                        off_t,
                        *mut fuse_file_info,
                    ) -> libc::c_int,
            ),
            write: Some(
                fs_write
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *const libc::c_char,
                        size_t,
                        off_t,
                        *mut fuse_file_info,
                    ) -> libc::c_int,
            ),
            statfs: None,
            flush: None,
            release: None,
            fsync: None,
            setxattr: None,
            getxattr: None,
            listxattr: None,
            removexattr: None,
            opendir: None,
            readdir: Some(
                fs_readdir
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut libc::c_void,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                *const libc::c_char,
                                *const stat,
                                off_t,
                            ) -> libc::c_int,
                        >,
                        off_t,
                        *mut fuse_file_info,
                    ) -> libc::c_int,
            ),
            releasedir: None,
            fsyncdir: None,
            init: Some(
                fs_init as unsafe extern "C" fn(*mut fuse_conn_info) -> *mut libc::c_void,
            ),
            destroy: Some(fs_destroy as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            access: None,
            create: None,
            ftruncate: None,
            fgetattr: None,
            lock: None,
            utimens: None,
            bmap: None,
            ioctl: None,
            poll: None,
            write_buf: None,
            read_buf: None,
            flock: None,
            fallocate: None,
        };
        init.set_flag_nullpath_ok(0 as libc::c_uint);
        init.set_flag_nopath(0 as libc::c_uint);
        init.set_flag_utime_omit_ok(0 as libc::c_uint);
        init.set_flag_reserved(0 as libc::c_uint);
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
