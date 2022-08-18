use ::libc;
use std::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn random() -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vprintf(_: *const libc::c_char, _: ::std::ffi::VaList) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn pthread_spin_trylock(__lock: *mut pthread_spinlock_t) -> libc::c_int;
    fn pthread_spin_unlock(__lock: *mut pthread_spinlock_t) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn pthread_sigmask(
        __how: libc::c_int,
        __newmask: *const __sigset_t,
        __oldmask: *mut __sigset_t,
    ) -> libc::c_int;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn pthread_spin_init(
        __lock: *mut pthread_spinlock_t,
        __pshared: libc::c_int,
    ) -> libc::c_int;
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
    fn sendto(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: *const sockaddr,
        __addr_len: socklen_t,
    ) -> ssize_t;
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
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn ntohs(__netshort: uint16_t) -> uint16_t;
    fn htonl(__hostlong: uint32_t) -> uint32_t;
    fn htons(__hostshort: uint16_t) -> uint16_t;
    fn epoll_create(__size: libc::c_int) -> libc::c_int;
    fn epoll_ctl(
        __epfd: libc::c_int,
        __op: libc::c_int,
        __fd: libc::c_int,
        __event: *mut epoll_event,
    ) -> libc::c_int;
    fn inet_pton(
        __af: libc::c_int,
        __cp: *const libc::c_char,
        __buf: *mut libc::c_void,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getpagesize() -> libc::c_int;
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
    fn pthread_spin_lock(__lock: *mut pthread_spinlock_t) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn ntohl(__netlong: uint32_t) -> uint32_t;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_setaffinity_np(
        __th: pthread_t,
        __cpusetsize: size_t,
        __cpuset: *const cpu_set_t,
    ) -> libc::c_int;
    fn accept(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> libc::c_int;
    fn epoll_wait(
        __epfd: libc::c_int,
        __events: *mut epoll_event,
        __maxevents: libc::c_int,
        __timeout: libc::c_int,
    ) -> libc::c_int;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn pthread_detach(__th: pthread_t) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn daemon(__nochdir: libc::c_int, __noclose: libc::c_int) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn sigtimedwait(
        __set: *const sigset_t,
        __info: *mut siginfo_t,
        __timeout: *const timespec,
    ) -> libc::c_int;
    fn shmget(__key: key_t, __size: size_t, __shmflg: libc::c_int) -> libc::c_int;
    fn shmat(
        __shmid: libc::c_int,
        __shmaddr: *const libc::c_void,
        __shmflg: libc::c_int,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
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
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___sigset_t_991265788 {
    pub __val: [libc::c_ulong; 16],
}
pub type __sigset_t = __anonstruct___sigset_t_991265788;
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type pthread_spinlock_t = libc::c_int;
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct__kill_244518854 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct__timer_490064738 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct__rt_619254530 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct__sigchld_284671705 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct__addr_bnd_5259977 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion__bounds_1031523602 {
    pub _addr_bnd: __anonstruct__addr_bnd_5259977,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct__sigfault_192557564 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: __anonunion__bounds_1031523602,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct__sigpoll_386613454 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct__sigsys_44812255 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion__sifields_998240460 {
    pub _pad: [libc::c_int; 28],
    pub _kill: __anonstruct__kill_244518854,
    pub _timer: __anonstruct__timer_490064738,
    pub _rt: __anonstruct__rt_619254530,
    pub _sigchld: __anonstruct__sigchld_284671705,
    pub _sigfault: __anonstruct__sigfault_192557564,
    pub _sigpoll: __anonstruct__sigpoll_386613454,
    pub _sigsys: __anonstruct__sigsys_44812255,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_siginfo_t_267242570 {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: __anonunion__sifields_998240460,
}
pub type siginfo_t = __anonstruct_siginfo_t_267242570;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion___sigaction_handler_363639592 {
    pub sa_handler: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: __anonunion___sigaction_handler_363639592,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
pub type uint___0 = libc::c_uint;
pub type uchar = libc::c_uchar;
pub type ushort___0 = libc::c_ushort;
pub type ulong___0 = libc::c_ulong;
pub type hashval_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _packet_type {
    pub label_count: uint8_t,
    pub domain: [uchar; 256],
    pub label: [*mut uint8_t; 64],
    pub label_offsets: [uint8_t; 64],
    pub label_len: [uint8_t; 64],
    pub hash: [hashval_t; 64],
}
pub type packet_type = _packet_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_node {
    pub data: *mut libc::c_void,
    pub next: *mut list_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list {
    pub lock: pthread_spinlock_t,
    pub head: *mut list_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ttlnode {
    pub exp: uint___0,
    pub dlen: ushort___0,
    pub type_0: ushort___0,
    pub hash: *mut hashval_t,
    pub lowerdomain: *mut packet_type,
    pub data: *mut uchar,
}
pub type rrtype = libc::c_uint;
pub const ANY: rrtype = 255;
pub const MAILA: rrtype = 254;
pub const MAILB: rrtype = 253;
pub const AXFR: rrtype = 252;
pub const TKEY: rrtype = 249;
pub const DHCID: rrtype = 49;
pub const DNSKEY: rrtype = 48;
pub const NSEC: rrtype = 47;
pub const RRSIG: rrtype = 46;
pub const DS: rrtype = 43;
pub const APL: rrtype = 42;
pub const OPT: rrtype = 41;
pub const DNAME: rrtype = 39;
pub const A6: rrtype = 38;
pub const CERT: rrtype = 37;
pub const SRV: rrtype = 33;
pub const NXT: rrtype = 30;
pub const AAAA: rrtype = 28;
pub const KEY: rrtype = 25;
pub const SIG: rrtype = 24;
pub const AFSDB: rrtype = 18;
pub const RP: rrtype = 17;
pub const TXT: rrtype = 16;
pub const MX: rrtype = 15;
pub const MINFO: rrtype = 14;
pub const HINFO: rrtype = 13;
pub const PTR: rrtype = 12;
pub const WKS: rrtype = 11;
pub const NUL: rrtype = 10;
pub const MR: rrtype = 9;
pub const MG: rrtype = 8;
pub const MB: rrtype = 7;
pub const SOA: rrtype = 6;
pub const CNAME: rrtype = 5;
pub const MF: rrtype = 4;
pub const MD: rrtype = 3;
pub const NS: rrtype = 2;
pub const A: rrtype = 1;
pub const BEGIN_TYPE: rrtype = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _type_value {
    pub A: *mut uchar,
    pub NS: *mut uchar,
    pub CNAME: *mut uchar,
    pub SOA: *mut uchar,
    pub MX: *mut uchar,
    pub TXT: *mut uchar,
    pub AAAA: *mut uchar,
    pub SRV: *mut uchar,
    pub PTR: *mut uchar,
}
pub type type_value = _type_value;
pub type comprbt = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
    *mut libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rbnode {
    pub parent: *mut rbnode,
    pub left: *mut rbnode,
    pub right: *mut rbnode,
    pub color: libc::c_int,
    pub key: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rbtree {
    pub root: *mut rbnode,
    pub nil: rbnode,
    pub lock: pthread_spinlock_t,
    pub size: uint___0,
    pub c: Option::<comprbt>,
    pub argv: *mut libc::c_void,
}
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
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
pub struct prod {
    pub watermark: uint32_t,
    pub sp_enqueue: uint32_t,
    pub size: uint32_t,
    pub mask: uint32_t,
    pub head: uint32_t,
    pub tail: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cons {
    pub sc_dequeue: uint32_t,
    pub size: uint32_t,
    pub mask: uint32_t,
    pub head: uint32_t,
    pub tail: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbuf_ring {
    pub prod: prod,
    pub cons: cons,
    pub ring: [*mut libc::c_void; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _mem_buf {
    pub mbuf: *mut mbuf_ring,
    pub fetch_len: uint___0,
    pub socktype: uint___0,
    pub fd: libc::c_int,
    pub addr: *mut sockaddr_in,
    pub caddr: sockaddr_in,
    pub aaddr: sockaddr_in,
    pub data: [uchar; 4096],
    pub qtype: rrtype,
    pub err: libc::c_int,
    pub dlen: libc::c_int,
    pub id: ushort___0,
    pub lowerdomain: packet_type,
    pub origindomain: *mut uchar,
    pub buflen: libc::c_int,
    pub buf: *mut uchar,
    pub td: *mut uchar,
    pub cid: ushort___0,
    pub qlen: ushort___0,
    pub lables: ushort___0,
    pub qing: *mut uchar,
    pub qhash: *mut hashval_t,
    pub backid: ushort___0,
    pub aid: ushort___0,
    pub mask: ushort___0,
    pub qname: ushort___0,
    pub sq: ushort___0,
    pub qtimes: ushort___0,
    pub auth_socktype: ushort___0,
    pub stat: ushort___0,
    pub qbuffer: [uchar; 256],
    pub qbuffer_hash: hashval_t,
    pub tdbuffer: *mut uchar,
    pub tempbuffer: *mut uchar,
    pub dmbuffer: *mut uchar,
    pub ipbuffer: *mut uchar,
    pub hascname: ushort___0,
    pub tcpfd: libc::c_int,
    pub tcpnums: libc::c_int,
    pub mxtry: libc::c_int,
    pub qns: libc::c_int,
    pub stime: uint64_t,
}
pub type mbuf_type = _mem_buf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msgcache {
    pub head: uint64_t,
    pub tail: uint64_t,
    pub size: uint32_t,
    pub pkt: uint32_t,
    pub lock: pthread_spinlock_t,
    pub data: [uchar; 0],
}
pub type hashfunc = unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> hashval_t;
pub type comparefunc = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_355715386 {
    pub vals: [*mut uchar; 9],
    pub val: type_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hentry {
    pub __annonCompField4: __anonunion____missing_field_name_355715386,
    pub next: *mut hentry,
    pub count: libc::c_int,
    pub key: [uchar; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hdata {
    pub list: *mut hentry,
    pub now: uint64_t,
    pub lock: pthread_spinlock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htable {
    pub lock: pthread_spinlock_t,
    pub table: *mut hdata,
    pub edata: *mut uchar,
    pub h: Option::<hashfunc>,
    pub size: uint___0,
    pub mask: uint___0,
    pub now: uint___0,
    pub c: Option::<comparefunc>,
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
pub struct sockinfo {
    pub addr: sockaddr_in,
    pub fd: libc::c_int,
    pub buflen: libc::c_int,
    pub socktype: libc::c_int,
    pub buf: *mut uchar,
    pub lowerdomain: *mut packet_type,
    pub mbuf: *mut mbuf_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qoutinfo {
    pub td: *mut uchar,
    pub type_0: uchar,
    pub lowerdomain: *mut packet_type,
    pub cli: *mut sockinfo,
    pub cid: ushort___0,
    pub dlen: ushort___0,
    pub qlen: ushort___0,
    pub lables: ushort___0,
    pub qing: *mut uchar,
    pub qhash: *mut hashval_t,
    pub backid: ushort___0,
    pub aid: ushort___0,
    pub mask: ushort___0,
    pub qname: ushort___0,
    pub sq: ushort___0,
    pub qtimes: ushort___0,
    pub socktype: ushort___0,
    pub stat: ushort___0,
    pub qbuffer: [uchar; 256],
    pub qbuffer_hash: hashval_t,
    pub tdbuffer: *mut uchar,
    pub tempbuffer: *mut uchar,
    pub dmbuffer: *mut uchar,
    pub ipbuffer: *mut uchar,
    pub hascname: ushort___0,
    pub tcpfd: libc::c_int,
    pub tcpnums: libc::c_int,
    pub mxtry: libc::c_int,
    pub qns: libc::c_int,
    pub stime: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct log_info {
    pub logfd: libc::c_int,
    pub lastlog: time_t,
    pub log_type: libc::c_int,
    pub log_cache: [uchar; 1048576],
    pub log_cache_cursor: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eptcpfds {
    pub ret: libc::c_int,
    pub domain: [uchar; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server {
    pub nquizzer: ushort___0,
    pub nfetcher: ushort___0,
    pub ludp: libc::c_int,
    pub ltcp: libc::c_int,
    pub fetchers: *mut fetcher,
    pub authors: *mut author,
    pub eventlist: list,
    pub datasets: *mut htable,
    pub forward: *mut htable,
    pub qlist: *mut htable,
    pub pkg: ulong___0,
    pub logpath: [uchar; 255],
    pub recordsindb: ulong___0,
    pub ttlexp: *mut rbtree,
    pub refreshflag: uint16_t,
    pub lastrefresh: time_t,
    pub is_forward: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct author {
    pub audp: libc::c_int,
    pub cudp: libc::c_int,
    pub idx: libc::c_int,
    pub s: *mut server,
    pub lock: pthread_spinlock_t,
    pub list: [*mut qoutinfo; 10000],
    pub qnum: libc::c_int,
    pub response: libc::c_int,
    pub qidx: libc::c_int,
    pub timex: libc::c_int,
    pub el: *mut list,
    pub bdepfd: libc::c_int,
    pub loginfo: *mut log_info,
    // pub dblock: [pthread_spinlock_t; 101],
    pub databuffer: [uchar; 65528],
    pub randombuffer: [uchar; 3000],
    pub tmpbuffer: [uchar; 2000],
    pub tdbuffer: [uchar; 256],
    pub tempbuffer: [uchar; 2000],
    pub dmbuffer: [uchar; 512],
    pub ipbuffer: [uchar; 512],
    pub e: [epoll_event; 1000],
    pub rndidx: libc::c_int,
    pub dataidx: libc::c_int,
    pub ip: [uchar; 2000],
    pub eptcpfds: [eptcpfds; 65530],
    pub rdb: uint___0,
    pub ddbefore: libc::c_int,
    pub underattack: libc::c_int,
    pub tcpinuse: libc::c_int,
    pub fwd: *mut htable,
    pub ds: *mut htable,
    pub dupbefore: libc::c_int,
    pub limits: libc::c_int,
    pub hsidx: libc::c_int,
    pub quizz: uint___0,
    pub drop: uint___0,
    pub timeout: uint___0,
    pub start: libc::c_int,
    pub end: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fetcher {
    pub idx: libc::c_int,
    pub s: *mut server,
    pub mc: *mut msgcache,
    pub el: *mut list,
    pub loginfo: *mut log_info,
    pub originbuffer: [uchar; 65528],
    pub tdbuffer: [uchar; 256],
    pub databuffer: [uchar; 65528],
    pub cbbuffer: [uchar; 512],
    pub dataidx: libc::c_int,
    pub qidx: libc::c_int,
    pub pkg: uint64_t,
    pub send: uint64_t,
    pub miss: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thread_query_info {
    pub query_num: [libc::c_ulong; 9],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct global_query_info {
    pub thread_num: libc::c_int,
    pub log_flag: libc::c_int,
    pub query_info: [thread_query_info; 65],
}
pub type __socklen_t = libc::c_uint;
pub type socklen_t = __socklen_t;
pub type SA = sockaddr;
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mvalue {
    pub len: uint16_t,
    pub num: uint16_t,
    pub ttl: uint32_t,
    pub hits: uint32_t,
    pub seg: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_hlp {
    pub ht: *mut htable,
    pub idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct setheader {
    pub an: ushort___0,
    pub ns: ushort___0,
    pub id: ushort___0,
    pub dlen: ushort___0,
    pub od: *mut uchar,
    pub itor: *mut uchar,
    pub type_0: ushort___0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hlpp {
    pub stype: *mut libc::c_int,
    pub ds: *mut htable,
    pub rbt: *mut rbtree,
    pub buf: *mut uchar,
    pub datalen: libc::c_int,
    pub dms: *mut uchar,
    pub dmsidx: libc::c_int,
    pub section: libc::c_int,
    pub tmpbuf: *mut uchar,
    pub domainbuf: *mut uchar,
    pub dmbuf: *mut uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hlpc {
    pub name: *mut uchar,
    pub off: libc::c_short,
    pub level: libc::c_short,
    pub ref_0: libc::c_short,
    pub mt: libc::c_short,
    pub len: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hlpf {
    pub type_0: ushort___0,
    pub len: ushort___0,
    pub ttl: uint___0,
    pub hdr: *mut uchar,
    pub from: *mut uchar,
    pub to: *mut uchar,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct fillmsg {
    pub type_0: uint16_t,
    pub dclass: uint16_t,
    pub ttl: uint32_t,
    pub len: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct tag_dnsheader {
    pub id: uint16_t,
    pub flags: uint16_t,
    pub qdcount: uint16_t,
    pub ancount: uint16_t,
    pub nscount: uint16_t,
    pub arcount: uint16_t,
}
pub type dnsheader = tag_dnsheader;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct tag_dq {
    pub type_0: uint16_t,
    pub dclass: uint16_t,
}
pub type qdns = tag_dq;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct srv {
    pub pri: uint16_t,
    pub wei: uint16_t,
    pub port: uint16_t,
}
pub type __mode_t = libc::c_uint;
pub type mode_t = __mode_t;
pub type __cpu_mask = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_cpu_set_t_826868708 {
    pub __bits: [__cpu_mask; 16],
}
pub type cpu_set_t = __anonstruct_cpu_set_t_826868708;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_data {
    pub fd: libc::c_int,
    pub cb: Option::<
        unsafe extern "C" fn(
            *mut event_data,
            *mut libc::c_void,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub ext: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iner_event {
    pub epfd: libc::c_int,
    pub buf: *mut libc::c_char,
    pub e: [epoll_event; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub size: libc::c_int,
    pub onexit: libc::c_int,
    pub ie: *mut iner_event,
    pub data: [event_data; 0],
}
pub type event_type = libc::c_uint;
pub const ET_ALL: event_type = 3;
pub const ET_WRITE: event_type = 2;
pub const ET_READ: event_type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_help {
    pub fd: libc::c_int,
    pub spfd: libc::c_int,
    pub num: libc::c_int,
    pub type_0: event_type,
    pub to: *mut timeval,
    pub cb: Option::<
        unsafe extern "C" fn(
            *mut event_data,
            *mut libc::c_void,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub ext: *mut libc::c_void,
}
pub type __int32_t = libc::c_int;
pub type __useconds_t = libc::c_uint;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union grifa {
    pub val: libc::c_int,
    pub randombuffer: [uchar; 4],
}
pub type __key_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type key_t = __key_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type sig_atomic_t = __sig_atomic_t;
pub type __dev_t = libc::c_ulong;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __nlink_t = libc::c_ulong;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
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
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
pub static mut global_serv: *mut server = 0 as *const server as *mut server;
pub static mut g_nameservers: [*mut libc::c_char; 2] = [0 as *const libc::c_char
    as *mut libc::c_char; 2];
pub static mut global_out_info: *mut global_query_info = 0 as *const global_query_info
    as *mut global_query_info;
pub static mut query_type_map: [libc::c_int; 256] = [0; 256];
pub unsafe extern "C" fn slog(
    mut msg: *mut uchar,
    mut fd: libc::c_int,
    mut lock: *mut pthread_spinlock_t,
) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn get_random_data(
    mut buffer: *mut uchar,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut tmp: ssize_t = 0;
    fd = -(1 as libc::c_int);
    ret = 0 as libc::c_int;
    if buffer as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int)
    } else {
        if len < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    fd = open(b"/dev/urandom\0" as *const u8 as *const libc::c_char, 0 as libc::c_int);
    if fd <= 0 as libc::c_int {
        return fd;
    }
    tmp = read(fd, buffer as *mut libc::c_void, len as size_t);
    ret = tmp as libc::c_int;
    if ret == -(1 as libc::c_int) {
        perror(b"read\0" as *const u8 as *const libc::c_char);
    }
    close(fd);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn get_str(
    mut str: *mut uchar,
    mut len: libc::c_int,
) -> *mut uchar {
    let mut ret: *mut uchar = 0 as *mut uchar;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc((len + 1 as libc::c_int) as size_t);
    ret = tmp as *mut uchar;
    strncpy(
        ret as *mut libc::c_char,
        str as *mut libc::c_char as *const libc::c_char,
        (len + 1 as libc::c_int) as size_t,
    );
    *ret.offset(len as isize) = 0 as libc::c_int as uchar;
    return ret;
}
pub unsafe extern "C" fn put_str(mut str: *mut uchar) {
    free(str as *mut libc::c_void);
}
pub unsafe extern "C" fn flush_all_to_disk(mut s: *mut server) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut log: *mut log_info = 0 as *mut log_info;
    let mut tmp: ssize_t = 0;
    let mut tmp___0: ssize_t = 0;
    log = 0 as *mut libc::c_void as *mut log_info;
    i = 0 as libc::c_int;
    while i < (*s).nfetcher as libc::c_int {
        log = (*((*s).fetchers).offset(i as isize)).loginfo;
        tmp = write(
            (*log).logfd,
            ((*log).log_cache).as_mut_ptr() as *const libc::c_void,
            (*log).log_cache_cursor as size_t,
        );
        ret = tmp as libc::c_int;
        if ret == -(1 as libc::c_int) {
            perror(b"write\0" as *const u8 as *const libc::c_char);
        }
        (*log).log_cache_cursor = 0 as libc::c_int;
        close((*log).logfd);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*s).nquizzer as libc::c_int {
        log = (*((*s).authors).offset(i as isize)).loginfo;
        tmp___0 = write(
            (*log).logfd,
            ((*log).log_cache).as_mut_ptr() as *const libc::c_void,
            (*log).log_cache_cursor as size_t,
        );
        ret = tmp___0 as libc::c_int;
        if ret == -(1 as libc::c_int) {
            perror(b"write\0" as *const u8 as *const libc::c_char);
        }
        (*log).log_cache_cursor = 0 as libc::c_int;
        close((*log).logfd);
        i += 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn sig_segment_fault(mut signo: libc::c_int) {
    printf(b"sig number is %d\n\0" as *const u8 as *const libc::c_char, signo);
    flush_all_to_disk(global_serv);
    exit(0 as libc::c_int);
}
pub unsafe extern "C" fn trig_signals(mut sig: libc::c_int) -> libc::c_int {
    let mut bset: sigset_t = sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = sigset_t { __val: [0; 16] };
    let mut sigs: [libc::c_int; 4] = [0; 4];
    let mut i: libc::c_int = 0;
    let mut sig_num: libc::c_int = 0;
    let mut sa: sigaction = sigaction {
        __sigaction_handler: __anonunion___sigaction_handler_363639592 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut oa: sigaction = sigaction {
        __sigaction_handler: __anonunion___sigaction_handler_363639592 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut tmp: libc::c_int = 0;
    sigs[0 as libc::c_int as usize] = 2 as libc::c_int;
    sigs[1 as libc::c_int as usize] = 7 as libc::c_int;
    sigs[2 as libc::c_int as usize] = 11 as libc::c_int;
    sigs[3 as libc::c_int as usize] = 13 as libc::c_int;
    memset(
        &mut sa as *mut sigaction as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sigaction>() as libc::c_ulong,
    );
    sa
        .__sigaction_handler
        .sa_handler = Some(sig_segment_fault as unsafe extern "C" fn(libc::c_int) -> ());
    sa.sa_flags = 268435456 as libc::c_int;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[libc::c_int; 4]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
    {
        sig_num = sigs[i as usize];
        sigaction(
            sig_num,
            &mut sa as *mut sigaction as *const sigaction,
            &mut oa as *mut sigaction,
        );
        i += 1;
    }
    sigemptyset(&mut bset);
    sigaddset(&mut bset, 10 as libc::c_int);
    tmp = pthread_sigmask(
        0 as libc::c_int,
        &mut bset as *mut sigset_t as *const __sigset_t,
        &mut oset as *mut sigset_t as *mut __sigset_t,
    );
    if tmp != 0 as libc::c_int {
        dns_error(
            0 as libc::c_int,
            b"sig error\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn drop_privilege(mut root: *mut libc::c_char) {
    if root as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
}
pub unsafe extern "C" fn dict_comp_uint_equ(
    mut a: *mut libc::c_void,
    mut b: *mut libc::c_void,
) -> libc::c_int {
    let mut u1: *mut uint___0 = 0 as *mut uint___0;
    let mut u2: *mut uint___0 = 0 as *mut uint___0;
    let mut tmp: libc::c_int = 0;
    u1 = a as *mut uint___0;
    u2 = b as *mut uint___0;
    if u1 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    if u2 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 1 as libc::c_int;
    }
    if *u1 == *u2 {
        return 0 as libc::c_int;
    }
    if u1 as libc::c_ulong > u2 as libc::c_ulong {
        tmp = 1 as libc::c_int;
    } else {
        tmp = -(1 as libc::c_int);
    }
    return tmp;
}
pub unsafe extern "C" fn dict_comp_str_equ(
    mut a: *mut libc::c_void,
    mut b: *mut libc::c_void,
) -> libc::c_int {
    let mut d1: *mut uchar = 0 as *mut uchar;
    let mut d2: *mut uchar = 0 as *mut uchar;
    let mut to: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    d1 = a as *mut uchar;
    d2 = b as *mut uchar;
    to = 256 as libc::c_int;
    if d1 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    if d2 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 1 as libc::c_int;
    }
    while *d1 as libc::c_int != 0 as libc::c_int {
        if !(*d2 as libc::c_int != 0 as libc::c_int) {
            break;
        }
        if *d1 as libc::c_int > *d2 as libc::c_int {
            return 1 as libc::c_int;
        }
        if (*d1 as libc::c_int) < *d2 as libc::c_int {
            return -(1 as libc::c_int);
        }
        d1 = d1.offset(1);
        d2 = d2.offset(1);
        tmp = to;
        to -= 1;
        if tmp == 0 as libc::c_int {
            printf(b"str compare error\n\0" as *const u8 as *const libc::c_char);
            exit(0 as libc::c_int);
        }
    }
    if *d1 as libc::c_int == 0 as libc::c_int {
        if *d2 as libc::c_int == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    if *d1 as libc::c_int == 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn rbt_comp_ttl_gt(
    mut v1: *mut libc::c_void,
    mut v2: *mut libc::c_void,
    mut argv: *mut libc::c_void,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut n1: *mut ttlnode = 0 as *mut ttlnode;
    let mut n2: *mut ttlnode = 0 as *mut ttlnode;
    if v2 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 1 as libc::c_int;
    }
    if v1 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    n1 = v1 as *mut ttlnode;
    n2 = v2 as *mut ttlnode;
    if (*n1).exp > (*n2).exp {
        return 1 as libc::c_int;
    }
    if (*n1).exp < (*n2).exp {
        return -(1 as libc::c_int);
    }
    if (*n1).type_0 as libc::c_int > (*n2).type_0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if ((*n1).type_0 as libc::c_int) < (*n2).type_0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    ret = dict_comp_str_equ(
        (*n1).data as *mut libc::c_void,
        (*n2).data as *mut libc::c_void,
    );
    if ret > 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn rbt_comp_uint_gt(
    mut v1: *mut libc::c_void,
    mut v2: *mut libc::c_void,
    mut argv: *mut libc::c_void,
) -> libc::c_int {
    let mut n1: uint___0 = 0;
    let mut n2: uint___0 = 0;
    let mut tmp: libc::c_int = 0;
    if v2 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 1 as libc::c_int;
    }
    if v1 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    n1 = *(v1 as *mut uint___0);
    n2 = *(v2 as *mut uint___0);
    if n1 == n2 {
        return 0 as libc::c_int;
    }
    if n1 > n2 {
        tmp = 1 as libc::c_int;
    } else {
        tmp = -(1 as libc::c_int);
    }
    return tmp;
}
pub static mut LowerTable: [libc::c_uchar; 256] = [
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    17 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    20 as libc::c_int as libc::c_uchar,
    21 as libc::c_int as libc::c_uchar,
    22 as libc::c_int as libc::c_uchar,
    23 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    25 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    27 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    31 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    33 as libc::c_int as libc::c_uchar,
    34 as libc::c_int as libc::c_uchar,
    35 as libc::c_int as libc::c_uchar,
    36 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    38 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    40 as libc::c_int as libc::c_uchar,
    41 as libc::c_int as libc::c_uchar,
    42 as libc::c_int as libc::c_uchar,
    43 as libc::c_int as libc::c_uchar,
    44 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    46 as libc::c_int as libc::c_uchar,
    47 as libc::c_int as libc::c_uchar,
    48 as libc::c_int as libc::c_uchar,
    49 as libc::c_int as libc::c_uchar,
    50 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    52 as libc::c_int as libc::c_uchar,
    53 as libc::c_int as libc::c_uchar,
    54 as libc::c_int as libc::c_uchar,
    55 as libc::c_int as libc::c_uchar,
    56 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    58 as libc::c_int as libc::c_uchar,
    59 as libc::c_int as libc::c_uchar,
    60 as libc::c_int as libc::c_uchar,
    61 as libc::c_int as libc::c_uchar,
    62 as libc::c_int as libc::c_uchar,
    63 as libc::c_int as libc::c_uchar,
    64 as libc::c_int as libc::c_uchar,
    97 as libc::c_int as libc::c_uchar,
    98 as libc::c_int as libc::c_uchar,
    99 as libc::c_int as libc::c_uchar,
    100 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    102 as libc::c_int as libc::c_uchar,
    103 as libc::c_int as libc::c_uchar,
    104 as libc::c_int as libc::c_uchar,
    105 as libc::c_int as libc::c_uchar,
    106 as libc::c_int as libc::c_uchar,
    107 as libc::c_int as libc::c_uchar,
    108 as libc::c_int as libc::c_uchar,
    109 as libc::c_int as libc::c_uchar,
    110 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    112 as libc::c_int as libc::c_uchar,
    113 as libc::c_int as libc::c_uchar,
    114 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    117 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    119 as libc::c_int as libc::c_uchar,
    120 as libc::c_int as libc::c_uchar,
    121 as libc::c_int as libc::c_uchar,
    122 as libc::c_int as libc::c_uchar,
    91 as libc::c_int as libc::c_uchar,
    92 as libc::c_int as libc::c_uchar,
    93 as libc::c_int as libc::c_uchar,
    94 as libc::c_int as libc::c_uchar,
    95 as libc::c_int as libc::c_uchar,
    96 as libc::c_int as libc::c_uchar,
    97 as libc::c_int as libc::c_uchar,
    98 as libc::c_int as libc::c_uchar,
    99 as libc::c_int as libc::c_uchar,
    100 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    102 as libc::c_int as libc::c_uchar,
    103 as libc::c_int as libc::c_uchar,
    104 as libc::c_int as libc::c_uchar,
    105 as libc::c_int as libc::c_uchar,
    106 as libc::c_int as libc::c_uchar,
    107 as libc::c_int as libc::c_uchar,
    108 as libc::c_int as libc::c_uchar,
    109 as libc::c_int as libc::c_uchar,
    110 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    112 as libc::c_int as libc::c_uchar,
    113 as libc::c_int as libc::c_uchar,
    114 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    117 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    119 as libc::c_int as libc::c_uchar,
    120 as libc::c_int as libc::c_uchar,
    121 as libc::c_int as libc::c_uchar,
    122 as libc::c_int as libc::c_uchar,
    123 as libc::c_int as libc::c_uchar,
    124 as libc::c_int as libc::c_uchar,
    125 as libc::c_int as libc::c_uchar,
    126 as libc::c_int as libc::c_uchar,
    127 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    129 as libc::c_int as libc::c_uchar,
    130 as libc::c_int as libc::c_uchar,
    131 as libc::c_int as libc::c_uchar,
    132 as libc::c_int as libc::c_uchar,
    133 as libc::c_int as libc::c_uchar,
    134 as libc::c_int as libc::c_uchar,
    135 as libc::c_int as libc::c_uchar,
    136 as libc::c_int as libc::c_uchar,
    137 as libc::c_int as libc::c_uchar,
    138 as libc::c_int as libc::c_uchar,
    139 as libc::c_int as libc::c_uchar,
    140 as libc::c_int as libc::c_uchar,
    141 as libc::c_int as libc::c_uchar,
    142 as libc::c_int as libc::c_uchar,
    143 as libc::c_int as libc::c_uchar,
    144 as libc::c_int as libc::c_uchar,
    145 as libc::c_int as libc::c_uchar,
    146 as libc::c_int as libc::c_uchar,
    147 as libc::c_int as libc::c_uchar,
    148 as libc::c_int as libc::c_uchar,
    149 as libc::c_int as libc::c_uchar,
    150 as libc::c_int as libc::c_uchar,
    151 as libc::c_int as libc::c_uchar,
    152 as libc::c_int as libc::c_uchar,
    153 as libc::c_int as libc::c_uchar,
    154 as libc::c_int as libc::c_uchar,
    155 as libc::c_int as libc::c_uchar,
    156 as libc::c_int as libc::c_uchar,
    157 as libc::c_int as libc::c_uchar,
    158 as libc::c_int as libc::c_uchar,
    159 as libc::c_int as libc::c_uchar,
    160 as libc::c_int as libc::c_uchar,
    161 as libc::c_int as libc::c_uchar,
    162 as libc::c_int as libc::c_uchar,
    163 as libc::c_int as libc::c_uchar,
    164 as libc::c_int as libc::c_uchar,
    165 as libc::c_int as libc::c_uchar,
    166 as libc::c_int as libc::c_uchar,
    167 as libc::c_int as libc::c_uchar,
    168 as libc::c_int as libc::c_uchar,
    169 as libc::c_int as libc::c_uchar,
    170 as libc::c_int as libc::c_uchar,
    171 as libc::c_int as libc::c_uchar,
    172 as libc::c_int as libc::c_uchar,
    173 as libc::c_int as libc::c_uchar,
    174 as libc::c_int as libc::c_uchar,
    175 as libc::c_int as libc::c_uchar,
    176 as libc::c_int as libc::c_uchar,
    177 as libc::c_int as libc::c_uchar,
    178 as libc::c_int as libc::c_uchar,
    179 as libc::c_int as libc::c_uchar,
    180 as libc::c_int as libc::c_uchar,
    181 as libc::c_int as libc::c_uchar,
    182 as libc::c_int as libc::c_uchar,
    183 as libc::c_int as libc::c_uchar,
    184 as libc::c_int as libc::c_uchar,
    185 as libc::c_int as libc::c_uchar,
    186 as libc::c_int as libc::c_uchar,
    187 as libc::c_int as libc::c_uchar,
    188 as libc::c_int as libc::c_uchar,
    189 as libc::c_int as libc::c_uchar,
    190 as libc::c_int as libc::c_uchar,
    191 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    194 as libc::c_int as libc::c_uchar,
    195 as libc::c_int as libc::c_uchar,
    196 as libc::c_int as libc::c_uchar,
    197 as libc::c_int as libc::c_uchar,
    198 as libc::c_int as libc::c_uchar,
    199 as libc::c_int as libc::c_uchar,
    200 as libc::c_int as libc::c_uchar,
    201 as libc::c_int as libc::c_uchar,
    202 as libc::c_int as libc::c_uchar,
    203 as libc::c_int as libc::c_uchar,
    204 as libc::c_int as libc::c_uchar,
    205 as libc::c_int as libc::c_uchar,
    206 as libc::c_int as libc::c_uchar,
    207 as libc::c_int as libc::c_uchar,
    208 as libc::c_int as libc::c_uchar,
    209 as libc::c_int as libc::c_uchar,
    210 as libc::c_int as libc::c_uchar,
    211 as libc::c_int as libc::c_uchar,
    212 as libc::c_int as libc::c_uchar,
    213 as libc::c_int as libc::c_uchar,
    214 as libc::c_int as libc::c_uchar,
    215 as libc::c_int as libc::c_uchar,
    216 as libc::c_int as libc::c_uchar,
    217 as libc::c_int as libc::c_uchar,
    218 as libc::c_int as libc::c_uchar,
    219 as libc::c_int as libc::c_uchar,
    220 as libc::c_int as libc::c_uchar,
    221 as libc::c_int as libc::c_uchar,
    222 as libc::c_int as libc::c_uchar,
    223 as libc::c_int as libc::c_uchar,
    224 as libc::c_int as libc::c_uchar,
    225 as libc::c_int as libc::c_uchar,
    226 as libc::c_int as libc::c_uchar,
    227 as libc::c_int as libc::c_uchar,
    228 as libc::c_int as libc::c_uchar,
    229 as libc::c_int as libc::c_uchar,
    230 as libc::c_int as libc::c_uchar,
    231 as libc::c_int as libc::c_uchar,
    232 as libc::c_int as libc::c_uchar,
    233 as libc::c_int as libc::c_uchar,
    234 as libc::c_int as libc::c_uchar,
    235 as libc::c_int as libc::c_uchar,
    236 as libc::c_int as libc::c_uchar,
    237 as libc::c_int as libc::c_uchar,
    238 as libc::c_int as libc::c_uchar,
    239 as libc::c_int as libc::c_uchar,
    240 as libc::c_int as libc::c_uchar,
    241 as libc::c_int as libc::c_uchar,
    242 as libc::c_int as libc::c_uchar,
    243 as libc::c_int as libc::c_uchar,
    244 as libc::c_int as libc::c_uchar,
    245 as libc::c_int as libc::c_uchar,
    246 as libc::c_int as libc::c_uchar,
    247 as libc::c_int as libc::c_uchar,
    248 as libc::c_int as libc::c_uchar,
    249 as libc::c_int as libc::c_uchar,
    250 as libc::c_int as libc::c_uchar,
    251 as libc::c_int as libc::c_uchar,
    252 as libc::c_int as libc::c_uchar,
    253 as libc::c_int as libc::c_uchar,
    254 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
];
pub static mut UpperTable: [libc::c_uchar; 256] = [
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    17 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    20 as libc::c_int as libc::c_uchar,
    21 as libc::c_int as libc::c_uchar,
    22 as libc::c_int as libc::c_uchar,
    23 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    25 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    27 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    31 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    33 as libc::c_int as libc::c_uchar,
    34 as libc::c_int as libc::c_uchar,
    35 as libc::c_int as libc::c_uchar,
    36 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    38 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    40 as libc::c_int as libc::c_uchar,
    41 as libc::c_int as libc::c_uchar,
    42 as libc::c_int as libc::c_uchar,
    43 as libc::c_int as libc::c_uchar,
    44 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    46 as libc::c_int as libc::c_uchar,
    47 as libc::c_int as libc::c_uchar,
    48 as libc::c_int as libc::c_uchar,
    49 as libc::c_int as libc::c_uchar,
    50 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    52 as libc::c_int as libc::c_uchar,
    53 as libc::c_int as libc::c_uchar,
    54 as libc::c_int as libc::c_uchar,
    55 as libc::c_int as libc::c_uchar,
    56 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    58 as libc::c_int as libc::c_uchar,
    59 as libc::c_int as libc::c_uchar,
    60 as libc::c_int as libc::c_uchar,
    61 as libc::c_int as libc::c_uchar,
    62 as libc::c_int as libc::c_uchar,
    63 as libc::c_int as libc::c_uchar,
    64 as libc::c_int as libc::c_uchar,
    65 as libc::c_int as libc::c_uchar,
    66 as libc::c_int as libc::c_uchar,
    67 as libc::c_int as libc::c_uchar,
    68 as libc::c_int as libc::c_uchar,
    69 as libc::c_int as libc::c_uchar,
    70 as libc::c_int as libc::c_uchar,
    71 as libc::c_int as libc::c_uchar,
    72 as libc::c_int as libc::c_uchar,
    73 as libc::c_int as libc::c_uchar,
    74 as libc::c_int as libc::c_uchar,
    75 as libc::c_int as libc::c_uchar,
    76 as libc::c_int as libc::c_uchar,
    77 as libc::c_int as libc::c_uchar,
    78 as libc::c_int as libc::c_uchar,
    79 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
    81 as libc::c_int as libc::c_uchar,
    82 as libc::c_int as libc::c_uchar,
    83 as libc::c_int as libc::c_uchar,
    84 as libc::c_int as libc::c_uchar,
    85 as libc::c_int as libc::c_uchar,
    86 as libc::c_int as libc::c_uchar,
    87 as libc::c_int as libc::c_uchar,
    88 as libc::c_int as libc::c_uchar,
    89 as libc::c_int as libc::c_uchar,
    90 as libc::c_int as libc::c_uchar,
    91 as libc::c_int as libc::c_uchar,
    92 as libc::c_int as libc::c_uchar,
    93 as libc::c_int as libc::c_uchar,
    94 as libc::c_int as libc::c_uchar,
    95 as libc::c_int as libc::c_uchar,
    96 as libc::c_int as libc::c_uchar,
    65 as libc::c_int as libc::c_uchar,
    66 as libc::c_int as libc::c_uchar,
    67 as libc::c_int as libc::c_uchar,
    68 as libc::c_int as libc::c_uchar,
    69 as libc::c_int as libc::c_uchar,
    70 as libc::c_int as libc::c_uchar,
    71 as libc::c_int as libc::c_uchar,
    72 as libc::c_int as libc::c_uchar,
    73 as libc::c_int as libc::c_uchar,
    74 as libc::c_int as libc::c_uchar,
    75 as libc::c_int as libc::c_uchar,
    76 as libc::c_int as libc::c_uchar,
    77 as libc::c_int as libc::c_uchar,
    78 as libc::c_int as libc::c_uchar,
    79 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
    81 as libc::c_int as libc::c_uchar,
    82 as libc::c_int as libc::c_uchar,
    83 as libc::c_int as libc::c_uchar,
    84 as libc::c_int as libc::c_uchar,
    85 as libc::c_int as libc::c_uchar,
    86 as libc::c_int as libc::c_uchar,
    87 as libc::c_int as libc::c_uchar,
    88 as libc::c_int as libc::c_uchar,
    89 as libc::c_int as libc::c_uchar,
    90 as libc::c_int as libc::c_uchar,
    123 as libc::c_int as libc::c_uchar,
    124 as libc::c_int as libc::c_uchar,
    125 as libc::c_int as libc::c_uchar,
    126 as libc::c_int as libc::c_uchar,
    127 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    129 as libc::c_int as libc::c_uchar,
    130 as libc::c_int as libc::c_uchar,
    131 as libc::c_int as libc::c_uchar,
    132 as libc::c_int as libc::c_uchar,
    133 as libc::c_int as libc::c_uchar,
    134 as libc::c_int as libc::c_uchar,
    135 as libc::c_int as libc::c_uchar,
    136 as libc::c_int as libc::c_uchar,
    137 as libc::c_int as libc::c_uchar,
    138 as libc::c_int as libc::c_uchar,
    139 as libc::c_int as libc::c_uchar,
    140 as libc::c_int as libc::c_uchar,
    141 as libc::c_int as libc::c_uchar,
    142 as libc::c_int as libc::c_uchar,
    143 as libc::c_int as libc::c_uchar,
    144 as libc::c_int as libc::c_uchar,
    145 as libc::c_int as libc::c_uchar,
    146 as libc::c_int as libc::c_uchar,
    147 as libc::c_int as libc::c_uchar,
    148 as libc::c_int as libc::c_uchar,
    149 as libc::c_int as libc::c_uchar,
    150 as libc::c_int as libc::c_uchar,
    151 as libc::c_int as libc::c_uchar,
    152 as libc::c_int as libc::c_uchar,
    153 as libc::c_int as libc::c_uchar,
    154 as libc::c_int as libc::c_uchar,
    155 as libc::c_int as libc::c_uchar,
    156 as libc::c_int as libc::c_uchar,
    157 as libc::c_int as libc::c_uchar,
    158 as libc::c_int as libc::c_uchar,
    159 as libc::c_int as libc::c_uchar,
    160 as libc::c_int as libc::c_uchar,
    161 as libc::c_int as libc::c_uchar,
    162 as libc::c_int as libc::c_uchar,
    163 as libc::c_int as libc::c_uchar,
    164 as libc::c_int as libc::c_uchar,
    165 as libc::c_int as libc::c_uchar,
    166 as libc::c_int as libc::c_uchar,
    167 as libc::c_int as libc::c_uchar,
    168 as libc::c_int as libc::c_uchar,
    169 as libc::c_int as libc::c_uchar,
    170 as libc::c_int as libc::c_uchar,
    171 as libc::c_int as libc::c_uchar,
    172 as libc::c_int as libc::c_uchar,
    173 as libc::c_int as libc::c_uchar,
    174 as libc::c_int as libc::c_uchar,
    175 as libc::c_int as libc::c_uchar,
    176 as libc::c_int as libc::c_uchar,
    177 as libc::c_int as libc::c_uchar,
    178 as libc::c_int as libc::c_uchar,
    179 as libc::c_int as libc::c_uchar,
    180 as libc::c_int as libc::c_uchar,
    181 as libc::c_int as libc::c_uchar,
    182 as libc::c_int as libc::c_uchar,
    183 as libc::c_int as libc::c_uchar,
    184 as libc::c_int as libc::c_uchar,
    185 as libc::c_int as libc::c_uchar,
    186 as libc::c_int as libc::c_uchar,
    187 as libc::c_int as libc::c_uchar,
    188 as libc::c_int as libc::c_uchar,
    189 as libc::c_int as libc::c_uchar,
    190 as libc::c_int as libc::c_uchar,
    191 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    194 as libc::c_int as libc::c_uchar,
    195 as libc::c_int as libc::c_uchar,
    196 as libc::c_int as libc::c_uchar,
    197 as libc::c_int as libc::c_uchar,
    198 as libc::c_int as libc::c_uchar,
    199 as libc::c_int as libc::c_uchar,
    200 as libc::c_int as libc::c_uchar,
    201 as libc::c_int as libc::c_uchar,
    202 as libc::c_int as libc::c_uchar,
    203 as libc::c_int as libc::c_uchar,
    204 as libc::c_int as libc::c_uchar,
    205 as libc::c_int as libc::c_uchar,
    206 as libc::c_int as libc::c_uchar,
    207 as libc::c_int as libc::c_uchar,
    208 as libc::c_int as libc::c_uchar,
    209 as libc::c_int as libc::c_uchar,
    210 as libc::c_int as libc::c_uchar,
    211 as libc::c_int as libc::c_uchar,
    212 as libc::c_int as libc::c_uchar,
    213 as libc::c_int as libc::c_uchar,
    214 as libc::c_int as libc::c_uchar,
    215 as libc::c_int as libc::c_uchar,
    216 as libc::c_int as libc::c_uchar,
    217 as libc::c_int as libc::c_uchar,
    218 as libc::c_int as libc::c_uchar,
    219 as libc::c_int as libc::c_uchar,
    220 as libc::c_int as libc::c_uchar,
    221 as libc::c_int as libc::c_uchar,
    222 as libc::c_int as libc::c_uchar,
    223 as libc::c_int as libc::c_uchar,
    224 as libc::c_int as libc::c_uchar,
    225 as libc::c_int as libc::c_uchar,
    226 as libc::c_int as libc::c_uchar,
    227 as libc::c_int as libc::c_uchar,
    228 as libc::c_int as libc::c_uchar,
    229 as libc::c_int as libc::c_uchar,
    230 as libc::c_int as libc::c_uchar,
    231 as libc::c_int as libc::c_uchar,
    232 as libc::c_int as libc::c_uchar,
    233 as libc::c_int as libc::c_uchar,
    234 as libc::c_int as libc::c_uchar,
    235 as libc::c_int as libc::c_uchar,
    236 as libc::c_int as libc::c_uchar,
    237 as libc::c_int as libc::c_uchar,
    238 as libc::c_int as libc::c_uchar,
    239 as libc::c_int as libc::c_uchar,
    240 as libc::c_int as libc::c_uchar,
    241 as libc::c_int as libc::c_uchar,
    242 as libc::c_int as libc::c_uchar,
    243 as libc::c_int as libc::c_uchar,
    244 as libc::c_int as libc::c_uchar,
    245 as libc::c_int as libc::c_uchar,
    246 as libc::c_int as libc::c_uchar,
    247 as libc::c_int as libc::c_uchar,
    248 as libc::c_int as libc::c_uchar,
    249 as libc::c_int as libc::c_uchar,
    250 as libc::c_int as libc::c_uchar,
    251 as libc::c_int as libc::c_uchar,
    252 as libc::c_int as libc::c_uchar,
    253 as libc::c_int as libc::c_uchar,
    254 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
];
pub unsafe extern "C" fn to_lowercase(
    mut msg: *mut uchar,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < len {
        *msg.offset(i as isize) = LowerTable[*msg.offset(i as isize) as usize];
        i += 1;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn to_uppercase(
    mut buf: *mut uchar,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        *buf.offset(i as isize) = UpperTable[*buf.offset(i as isize) as usize];
        i += 1;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn str_to_uchar4(
    mut addr: *const libc::c_char,
    mut val: *mut uchar,
) -> libc::c_int {
    let mut tv: [uint___0; 4] = [0; 4];
    let mut tmp: libc::c_uint = 0;
    let mut idx: uint___0 = 0;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut tmp___0: size_t = 0;
    tv[0 as libc::c_int as usize] = 0 as libc::c_int as uint___0;
    tmp = 1 as libc::c_uint;
    while !(tmp >= 4 as libc::c_uint) {
        tv[tmp as usize] = 0 as libc::c_uint;
        tmp = tmp.wrapping_add(1);
    }
    idx = 0 as libc::c_int as uint___0;
    tmp___0 = strlen(addr);
    n = tmp___0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        let mut current_block_20: u64;
        if *addr.offset(i as isize) as libc::c_int >= 48 as libc::c_int {
            if *addr.offset(i as isize) as libc::c_int <= 57 as libc::c_int {
                tv[idx
                    as usize] = (tv[idx as usize])
                    .wrapping_mul(10 as libc::c_uint)
                    .wrapping_add(*addr.offset(i as isize) as uint___0)
                    .wrapping_sub(48 as libc::c_uint);
                current_block_20 = 4495394744059808450;
            } else {
                current_block_20 = 9667575034259535015;
            }
        } else {
            current_block_20 = 9667575034259535015;
        }
        match current_block_20 {
            9667575034259535015 => {
                idx = idx.wrapping_add(1);
                if *addr.offset(i as isize) as libc::c_int != 46 as libc::c_int {
                    *val = 0 as libc::c_int as uchar;
                    return -(1 as libc::c_int);
                } else {
                    if idx == 4 as libc::c_uint {
                        *val = 0 as libc::c_int as uchar;
                        return -(1 as libc::c_int);
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        *val.offset(i as isize) = tv[i as usize] as uchar;
        i += 1;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn str_to_uchar6(
    mut addr: *mut uchar,
    mut val: *mut uchar,
) -> libc::c_int {
    let mut tv: [ushort___0; 8] = [0; 8];
    let mut tmp: libc::c_uint = 0;
    let mut idx: libc::c_int = 0;
    let mut gap: libc::c_int = 0;
    let mut gapidx: libc::c_int = 0;
    let mut hasgap: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: libc::c_char = 0;
    let mut tmp___2: uchar = 0;
    let mut tmp___3: uchar = 0;
    let mut tmp___4: uchar = 0;
    tv[0 as libc::c_int as usize] = 0 as libc::c_int as ushort___0;
    tmp = 1 as libc::c_uint;
    while !(tmp >= 8 as libc::c_uint) {
        tv[tmp as usize] = 0 as libc::c_int as libc::c_ushort;
        tmp = tmp.wrapping_add(1);
    }
    idx = 0 as libc::c_int;
    gap = 0 as libc::c_int;
    gapidx = -(1 as libc::c_int);
    hasgap = 0 as libc::c_int;
    tmp___0 = strlen(addr as *const libc::c_char);
    n = tmp___0 as libc::c_int;
    to_lowercase(addr, n);
    memset(val as *mut libc::c_void, 0 as libc::c_int, 16 as libc::c_int as size_t);
    i = 0 as libc::c_int;
    while i < n {
        tmp___1 = *addr.offset(i as isize) as libc::c_char;
        let mut current_block_42: u64;
        if tmp___1 as libc::c_int >= 48 as libc::c_int {
            if tmp___1 as libc::c_int <= 57 as libc::c_int {
                gap = 0 as libc::c_int;
                tv[idx
                    as usize] = (tv[idx as usize] as libc::c_int * 16 as libc::c_int
                    + tmp___1 as libc::c_int - 48 as libc::c_int) as ushort___0;
                current_block_42 = 3938820862080741272;
            } else {
                current_block_42 = 15547988767264126066;
            }
        } else {
            current_block_42 = 15547988767264126066;
        }
        match current_block_42 {
            15547988767264126066 => {
                let mut current_block_41: u64;
                if tmp___1 as libc::c_int >= 97 as libc::c_int {
                    if tmp___1 as libc::c_int <= 122 as libc::c_int {
                        gap = 0 as libc::c_int;
                        tv[idx
                            as usize] = (tv[idx as usize] as libc::c_int
                            * 16 as libc::c_int + tmp___1 as libc::c_int
                            - 97 as libc::c_int + 10 as libc::c_int) as ushort___0;
                        current_block_41 = 2543120759711851213;
                    } else {
                        current_block_41 = 9809238161333413355;
                    }
                } else {
                    current_block_41 = 9809238161333413355;
                }
                match current_block_41 {
                    9809238161333413355 => {
                        idx += 1;
                        if gap == 1 as libc::c_int {
                            if hasgap == 1 as libc::c_int {
                                return -(1 as libc::c_int);
                            }
                            hasgap = 1 as libc::c_int;
                            gapidx = idx - 1 as libc::c_int;
                        }
                        if gap == 0 as libc::c_int {
                            gap = 1 as libc::c_int;
                        }
                        's_213: {
                            if !(tmp___1 as libc::c_int != 58 as libc::c_int) {
                                if !(idx == 8 as libc::c_int) {
                                    break 's_213;
                                }
                            }
                            tmp___4 = 0 as libc::c_int as uchar;
                            *val.offset(3 as libc::c_int as isize) = tmp___4;
                            tmp___3 = tmp___4;
                            *val.offset(2 as libc::c_int as isize) = tmp___3;
                            tmp___2 = tmp___3;
                            *val.offset(1 as libc::c_int as isize) = tmp___2;
                            *val.offset(0 as libc::c_int as isize) = tmp___2;
                            return 0 as libc::c_int;
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        i += 1;
    }
    if hasgap == 1 as libc::c_int {
        i = 0 as libc::c_int;
        while i < gapidx {
            *val
                .offset(
                    (i * 2 as libc::c_int) as isize,
                ) = (tv[i as usize] as libc::c_int / 256 as libc::c_int) as uchar;
            *val
                .offset(
                    (i * 2 as libc::c_int + 1 as libc::c_int) as isize,
                ) = (tv[i as usize] as libc::c_int % 256 as libc::c_int) as uchar;
            i += 1;
        }
        i = idx - 1 as libc::c_int;
        while i >= gapidx {
            *val
                .offset(
                    ((i + 7 as libc::c_int - idx + 1 as libc::c_int) * 2 as libc::c_int)
                        as isize,
                ) = (tv[(i + 1 as libc::c_int) as usize] as libc::c_int
                / 256 as libc::c_int) as uchar;
            *val
                .offset(
                    ((i + 7 as libc::c_int - idx + 1 as libc::c_int) * 2 as libc::c_int
                        + 1 as libc::c_int) as isize,
                ) = (tv[(i + 1 as libc::c_int) as usize] as libc::c_int
                % 256 as libc::c_int) as uchar;
            i -= 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            *val
                .offset(
                    (i * 2 as libc::c_int) as isize,
                ) = (tv[i as usize] as libc::c_int / 256 as libc::c_int) as uchar;
            *val
                .offset(
                    (i * 2 as libc::c_int + 1 as libc::c_int) as isize,
                ) = (tv[i as usize] as libc::c_int % 256 as libc::c_int) as uchar;
            i += 1;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn fix_tail(mut domain: *mut libc::c_char) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut c: uchar = 0;
    tmp = strlen(domain as *const libc::c_char);
    len = tmp as libc::c_int;
    len -= 1;
    c = *domain.offset(len as isize) as uchar;
    if c as libc::c_int == 13 as libc::c_int {
        *domain.offset(len as isize) = 0 as libc::c_int as libc::c_char;
        len -= 1;
    } else if c as libc::c_int == 10 as libc::c_int {
        *domain.offset(len as isize) = 0 as libc::c_int as libc::c_char;
        len -= 1;
    }
    c = *domain.offset(len as isize) as uchar;
    if c as libc::c_int == 13 as libc::c_int {
        *domain.offset(len as isize) = 0 as libc::c_int as libc::c_char;
        len -= 1;
    } else if c as libc::c_int == 10 as libc::c_int {
        *domain.offset(len as isize) = 0 as libc::c_int as libc::c_char;
        len -= 1;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn opr_bit(
    mut bit: *mut libc::c_ushort,
    mut off: libc::c_int,
    mut set: libc::c_int,
) -> libc::c_int {
    let mut mask: libc::c_ushort = 0;
    mask = 1 as libc::c_int as libc::c_ushort;
    if off > 15 as libc::c_int {
        return -(1 as libc::c_int)
    } else {
        if off < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    mask = ((mask as libc::c_int) << off) as libc::c_ushort;
    if set == 0 as libc::c_int {
        mask = !(mask as libc::c_int) as libc::c_ushort;
    }
    if set == 0 as libc::c_int {
        *bit = (*bit as libc::c_int & mask as libc::c_int) as libc::c_ushort;
    } else {
        *bit = (*bit as libc::c_int | mask as libc::c_int) as libc::c_ushort;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn set_bit(
    mut bit: *mut libc::c_ushort,
    mut off: libc::c_int,
) -> libc::c_int {
    opr_bit(bit, off, 1 as libc::c_int);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn clr_bit(
    mut bit: *mut libc::c_ushort,
    mut off: libc::c_int,
) -> libc::c_int {
    opr_bit(bit, off, 0 as libc::c_int);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tst_bit(
    bit: libc::c_ushort,
    mut off: libc::c_int,
) -> libc::c_int {
    let mut mask: libc::c_ushort = 0;
    mask = 1 as libc::c_int as libc::c_ushort;
    if off > 15 as libc::c_int {
        return -(1 as libc::c_int)
    } else {
        if off < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    mask = ((mask as libc::c_int) << off) as libc::c_ushort;
    if bit as libc::c_int & mask as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn get_time_usage(
    mut tv: *mut timeval,
    mut start: libc::c_int,
) -> libc::c_int {
    let mut msec: ulong___0 = 0;
    let mut tmp: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    msec = 0 as libc::c_int as ulong___0;
    if tv as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    if start == 1 as libc::c_int {
        gettimeofday(tv, 0 as *mut libc::c_void);
    } else {
        tmp = *tv;
        gettimeofday(tv, 0 as *mut libc::c_void);
        if (*tv).tv_usec < tmp.tv_usec {
            msec = (((*tv).tv_usec - tmp.tv_usec + 1000000 as libc::c_long)
                / 1000 as libc::c_long) as ulong___0;
            (*tv).tv_sec -= 1;
        }
        printf(
            b"%lu s,%lu ms\n\0" as *const u8 as *const libc::c_char,
            (*tv).tv_sec - tmp.tv_sec,
            msec,
        );
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn is_uppercase(mut c: libc::c_int) -> libc::c_int {
    if c >= 65 as libc::c_int {
        if c <= 90 as libc::c_int {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn is_lowercase(mut c: libc::c_int) -> libc::c_int {
    if c >= 97 as libc::c_int {
        if c <= 122 as libc::c_int {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn empty_function(mut i: libc::c_int) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn insert_mem_bar() {
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut tmp: libc::c_long = 0;
    let mut ptr: *mut uchar = 0 as *mut uchar;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = random();
    size = (tmp % 99 as libc::c_long + 10000 as libc::c_long) as libc::c_int;
    tmp___0 = malloc(size as size_t);
    ptr = tmp___0 as *mut uchar;
    if ptr as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    i = 0 as libc::c_int;
    while i < size {
        *ptr.offset(i as isize) = i as uchar;
        i += 1;
    }
    free(ptr as *mut libc::c_void);
}
// pub unsafe extern "C" fn test_lock(mut l: *mut pthread_spinlock_t) -> libc::c_int {
//     let mut tmp: libc::c_int = 0;
//     tmp = pthread_spin_trylock(l);
//     if tmp < 0 as libc::c_int {
//         return -(1 as libc::c_int);
//     }
//     pthread_spin_unlock(l);
//     return 0 as libc::c_int;
// }
pub unsafe extern "C" fn dbg_print_bit(mut bit: libc::c_ushort) {
    let mut i: libc::c_int = 0;
    let mut val: libc::c_ushort = 0;
    val = ((1 as libc::c_int) << 15 as libc::c_int) as libc::c_ushort;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        if bit as libc::c_int & val as libc::c_int == 0 as libc::c_int {
            printf(b"0\0" as *const u8 as *const libc::c_char);
        } else {
            printf(b"1\0" as *const u8 as *const libc::c_char);
        }
        if (i + 1 as libc::c_int) % 4 as libc::c_int == 0 as libc::c_int {
            if i != 15 as libc::c_int {
                printf(b",\0" as *const u8 as *const libc::c_char);
            }
        }
        val = (val as libc::c_int >> 1 as libc::c_int) as libc::c_ushort;
        i += 1;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn print_hex(mut val: *mut uchar, mut n: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        printf(
            b"%x,\0" as *const u8 as *const libc::c_char,
            *val.offset(i as isize) as libc::c_int,
        );
        i += 1;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn dns_error(mut level: libc::c_int, mut msg: *mut libc::c_char) {
    dbg(b"Error:%s\n\0" as *const u8 as *const libc::c_char, msg);
    fflush(stdout);
    if level == 0 as libc::c_int {
        exit(-(1 as libc::c_int));
    }
}
pub unsafe extern "C" fn dbg(
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    printf(b"dbg:\0" as *const u8 as *const libc::c_char);
    ret = vprintf(format, ap.as_va_list());
    return ret;
}
pub unsafe extern "C" fn uint_hash_function(mut ptr: *mut libc::c_void) -> hashval_t {
    let mut key: uint___0 = 0;
    key = *(ptr as *mut uint___0);
    key = (key as libc::c_uint).wrapping_add(!(key << 15 as libc::c_int)) as uint___0
        as uint___0;
    key ^= key >> 10 as libc::c_int;
    key = (key as libc::c_uint).wrapping_add(key << 3 as libc::c_int) as uint___0
        as uint___0;
    key ^= key >> 6 as libc::c_int;
    key = (key as libc::c_uint).wrapping_add(!(key << 11 as libc::c_int)) as uint___0
        as uint___0;
    key ^= key >> 16 as libc::c_int;
    return key;
}
pub unsafe extern "C" fn nocase_char_hash_function(
    mut argv: *mut libc::c_void,
    mut klen: libc::c_int,
) -> hashval_t {
    let mut len: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut buf: *mut uchar = 0 as *mut uchar;
    let mut hash: hashval_t = 0;
    let mut tmp___0: *mut uchar = 0 as *mut uchar;
    let mut tmp___1: libc::c_int = 0;
    if klen == 2 as libc::c_int {
        tmp = 1 as libc::c_int;
    } else {
        tmp = klen;
    }
    len = tmp;
    buf = argv as *mut uchar;
    hash = 5381 as libc::c_int as hashval_t;
    loop {
        tmp___1 = len;
        len -= 1;
        if tmp___1 == 0 {
            break;
        }
        tmp___0 = buf;
        buf = buf.offset(1);
        hash = (hash << 5 as libc::c_int)
            .wrapping_add(hash)
            .wrapping_add(*tmp___0 as hashval_t);
    }
    return hash;
}
unsafe extern "C" fn left_rotate(mut rbt: *mut rbtree, mut node: *mut rbnode) {
    let mut tmp: *mut rbnode = 0 as *mut rbnode;
    tmp = (*node).right;
    (*node).right = (*tmp).left;
    if (*tmp).left as libc::c_ulong != &mut (*rbt).nil as *mut rbnode as libc::c_ulong {
        (*(*tmp).left).parent = node;
    }
    (*tmp).parent = (*node).parent;
    if (*node).parent as libc::c_ulong == &mut (*rbt).nil as *mut rbnode as libc::c_ulong
    {
        (*rbt).root = tmp;
    } else if node as libc::c_ulong == (*(*node).parent).left as libc::c_ulong {
        (*(*node).parent).left = tmp;
    } else {
        (*(*node).parent).right = tmp;
    }
    (*tmp).left = node;
    (*node).parent = tmp;
}
unsafe extern "C" fn right_rotate(mut rbt: *mut rbtree, mut node: *mut rbnode) {
    let mut tmp: *mut rbnode = 0 as *mut rbnode;
    tmp = (*node).left;
    (*node).left = (*tmp).right;
    if (*tmp).right as libc::c_ulong != &mut (*rbt).nil as *mut rbnode as libc::c_ulong {
        (*(*tmp).right).parent = node;
    }
    (*tmp).parent = (*node).parent;
    if (*node).parent as libc::c_ulong == &mut (*rbt).nil as *mut rbnode as libc::c_ulong
    {
        (*rbt).root = tmp;
    } else if node as libc::c_ulong == (*(*node).parent).left as libc::c_ulong {
        (*(*node).parent).left = tmp;
    } else {
        (*(*node).parent).right = tmp;
    }
    (*tmp).right = node;
    (*node).parent = tmp;
}
unsafe extern "C" fn insert_fixup(mut rbt: *mut rbtree, mut nd: *mut rbnode) {
    let mut tmp: *mut rbnode = 0 as *mut rbnode;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    while (*(*nd).parent).color == 1 as libc::c_int {
        if (*nd).parent as libc::c_ulong
            == (*(*(*nd).parent).parent).left as libc::c_ulong
        {
            tmp = (*(*(*nd).parent).parent).right;
            if (*tmp).color == 1 as libc::c_int {
                tmp___0 = 0 as libc::c_int;
                (*tmp).color = tmp___0;
                (*(*nd).parent).color = tmp___0;
                (*(*(*nd).parent).parent).color = 1 as libc::c_int;
                nd = (*(*nd).parent).parent;
            } else {
                if nd as libc::c_ulong == (*(*nd).parent).right as libc::c_ulong {
                    nd = (*nd).parent;
                    left_rotate(rbt, nd);
                }
                (*(*nd).parent).color = 0 as libc::c_int;
                (*(*(*nd).parent).parent).color = 1 as libc::c_int;
                right_rotate(rbt, (*(*nd).parent).parent);
            }
        } else {
            tmp = (*(*(*nd).parent).parent).left;
            if (*tmp).color == 1 as libc::c_int {
                tmp___1 = 0 as libc::c_int;
                (*tmp).color = tmp___1;
                (*(*nd).parent).color = tmp___1;
                (*(*(*nd).parent).parent).color = 1 as libc::c_int;
                nd = (*(*nd).parent).parent;
            } else {
                if nd as libc::c_ulong == (*(*nd).parent).left as libc::c_ulong {
                    nd = (*nd).parent;
                    right_rotate(rbt, nd);
                }
                (*(*nd).parent).color = 0 as libc::c_int;
                (*(*(*nd).parent).parent).color = 1 as libc::c_int;
                left_rotate(rbt, (*(*nd).parent).parent);
            }
        }
    }
    (*(*rbt).root).color = 0 as libc::c_int;
}
pub unsafe extern "C" fn find_node(
    mut rbt: *mut rbtree,
    mut key: *mut libc::c_void,
) -> *mut rbnode {
    let mut nd: *mut rbnode = 0 as *mut rbnode;
    let mut i: libc::c_int = 0;
    nd = &mut (*rbt).nil;
    nd = (*rbt).root;
    while nd as libc::c_ulong != &mut (*rbt).nil as *mut rbnode as libc::c_ulong {
        i = (Some(((*rbt).c).expect("non-null function pointer")))
            .expect("non-null function pointer")((*nd).key, key, (*rbt).argv);
        if i > 0 as libc::c_int {
            nd = (*nd).left;
        }
        if i < 0 as libc::c_int {
            nd = (*nd).right;
        }
        if nd as libc::c_ulong == &mut (*rbt).nil as *mut rbnode as libc::c_ulong {
            break;
        }
        if i == 0 as libc::c_int {
            return nd;
        }
    }
    return 0 as *mut libc::c_void as *mut rbnode;
}
pub unsafe extern "C" fn insert_node(
    mut rbt: *mut rbtree,
    mut pnd: *mut rbnode,
) -> libc::c_int {
    let mut tmp: *mut rbnode = 0 as *mut rbnode;
    let mut itor: *mut rbnode = 0 as *mut rbnode;
    let mut nd: *mut rbnode = 0 as *mut rbnode;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: *mut rbnode = 0 as *mut rbnode;
    tmp = &mut (*rbt).nil;
    itor = (*rbt).root;
    tmp___0 = malloc(::std::mem::size_of::<rbnode>() as libc::c_ulong);
    nd = tmp___0 as *mut rbnode;
    if nd as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    *nd = *pnd;
    while itor as libc::c_ulong != &mut (*rbt).nil as *mut rbnode as libc::c_ulong {
        tmp = itor;
        tmp___1 = (Some(((*rbt).c).expect("non-null function pointer")))
            .expect("non-null function pointer")((*itor).key, (*nd).key, (*rbt).argv);
        if tmp___1 > 0 as libc::c_int {
            itor = (*itor).left;
        } else {
            itor = (*itor).right;
        }
    }
    (*nd).parent = tmp;
    if tmp as libc::c_ulong == &mut (*rbt).nil as *mut rbnode as libc::c_ulong {
        (*rbt).root = nd;
    } else {
        tmp___2 = (Some(((*rbt).c).expect("non-null function pointer")))
            .expect("non-null function pointer")((*tmp).key, (*nd).key, (*rbt).argv);
        if tmp___2 > 0 as libc::c_int {
            (*tmp).left = nd;
        } else {
            (*tmp).right = nd;
        }
    }
    tmp___3 = &mut (*rbt).nil;
    (*nd).right = tmp___3;
    (*nd).left = tmp___3;
    (*nd).color = 1 as libc::c_int;
    insert_fixup(rbt, nd);
    (*rbt).size = ((*rbt).size).wrapping_add(1);
    return 0 as libc::c_int;
}
unsafe extern "C" fn rbt_successor(
    mut rbt: *mut rbtree,
    mut nd: *mut rbnode,
) -> *mut rbnode {
    let mut min: *mut rbnode = 0 as *mut rbnode;
    min = &mut (*rbt).nil;
    if (*nd).right as libc::c_ulong != &mut (*rbt).nil as *mut rbnode as libc::c_ulong {
        min = (*nd).right;
        while (*min).left as libc::c_ulong
            != &mut (*rbt).nil as *mut rbnode as libc::c_ulong
        {
            min = (*min).left;
        }
        return min;
    }
    min = (*nd).parent;
    while min as libc::c_ulong != &mut (*rbt).nil as *mut rbnode as libc::c_ulong {
        if !(nd as libc::c_ulong == (*min).right as libc::c_ulong) {
            break;
        }
        nd = min;
        min = (*min).parent;
    }
    return min;
}
unsafe extern "C" fn delete_fixup(mut rbt: *mut rbtree, mut nd: *mut rbnode) {
    let mut tmp: *mut rbnode = 0 as *mut rbnode;
    tmp = &mut (*rbt).nil;
    while nd as libc::c_ulong != (*rbt).root as libc::c_ulong {
        if !((*nd).color == 0 as libc::c_int) {
            break;
        }
        if nd as libc::c_ulong == (*(*nd).parent).left as libc::c_ulong {
            tmp = (*(*nd).parent).right;
            if (*tmp).color == 1 as libc::c_int {
                (*tmp).color = 0 as libc::c_int;
                (*(*nd).parent).color = 1 as libc::c_int;
                left_rotate(rbt, (*nd).parent);
                tmp = (*(*nd).parent).right;
            }
            let mut current_block_21: u64;
            if (*(*tmp).left).color == 0 as libc::c_int {
                if (*(*tmp).right).color == 0 as libc::c_int {
                    (*tmp).color = 1 as libc::c_int;
                    nd = (*nd).parent;
                    current_block_21 = 10043043949733653460;
                } else {
                    current_block_21 = 8410686024737438156;
                }
            } else {
                current_block_21 = 8410686024737438156;
            }
            match current_block_21 {
                8410686024737438156 => {
                    if (*(*tmp).right).color == 0 as libc::c_int {
                        (*(*tmp).left).color = 0 as libc::c_int;
                        (*tmp).color = 1 as libc::c_int;
                        right_rotate(rbt, tmp);
                        tmp = (*(*nd).parent).right;
                    }
                    (*tmp).color = (*(*nd).parent).color;
                    (*(*nd).parent).color = 0 as libc::c_int;
                    (*(*tmp).right).color = 0 as libc::c_int;
                    left_rotate(rbt, (*nd).parent);
                    nd = (*rbt).root;
                }
                _ => {}
            }
        } else {
            tmp = (*(*nd).parent).left;
            if (*tmp).color == 1 as libc::c_int {
                (*tmp).color = 0 as libc::c_int;
                (*(*nd).parent).color = 1 as libc::c_int;
                right_rotate(rbt, (*nd).parent);
                tmp = (*(*nd).parent).left;
            }
            let mut current_block_44: u64;
            if (*(*tmp).right).color == 0 as libc::c_int {
                if (*(*tmp).left).color == 0 as libc::c_int {
                    (*tmp).color = 1 as libc::c_int;
                    nd = (*nd).parent;
                    current_block_44 = 12997042908615822766;
                } else {
                    current_block_44 = 5591790895087476119;
                }
            } else {
                current_block_44 = 5591790895087476119;
            }
            match current_block_44 {
                5591790895087476119 => {
                    if (*(*tmp).left).color == 0 as libc::c_int {
                        (*(*tmp).right).color = 0 as libc::c_int;
                        (*tmp).color = 1 as libc::c_int;
                        left_rotate(rbt, tmp);
                        tmp = (*(*nd).parent).left;
                    }
                    (*tmp).color = (*(*nd).parent).color;
                    (*(*nd).parent).color = 0 as libc::c_int;
                    (*(*tmp).left).color = 0 as libc::c_int;
                    right_rotate(rbt, (*nd).parent);
                    nd = (*rbt).root;
                }
                _ => {}
            }
        }
    }
    (*nd).color = 0 as libc::c_int;
}
pub unsafe extern "C" fn min_node(mut rbt: *mut rbtree) -> *mut rbnode {
    let mut tmp: *mut rbnode = 0 as *mut rbnode;
    let mut ret: *mut rbnode = 0 as *mut rbnode;
    tmp = (*rbt).root;
    ret = &mut (*rbt).nil;
    if tmp as libc::c_ulong == &mut (*rbt).nil as *mut rbnode as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut rbnode;
    }
    while tmp as libc::c_ulong != &mut (*rbt).nil as *mut rbnode as libc::c_ulong {
        ret = tmp;
        tmp = (*tmp).left;
    }
    if ret as libc::c_ulong == &mut (*rbt).nil as *mut rbnode as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut rbnode;
    }
    return ret;
}
pub unsafe extern "C" fn delete_node(
    mut rbt: *mut rbtree,
    mut nd: *mut rbnode,
) -> *mut libc::c_void {
    let mut val: *mut ttlnode = 0 as *mut ttlnode;
    let mut ret: *mut rbnode = 0 as *mut rbnode;
    let mut tmp: *mut rbnode = 0 as *mut rbnode;
    let mut itor: *mut rbnode = 0 as *mut rbnode;
    val = 0 as *mut libc::c_void as *mut ttlnode;
    ret = nd;
    if nd as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void
    } else {
        if rbt as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            return 0 as *mut libc::c_void;
        }
    }
    val = (*nd).key as *mut ttlnode;
    if (*nd).left as libc::c_ulong == &mut (*rbt).nil as *mut rbnode as libc::c_ulong {
        tmp = nd;
    } else if (*nd).right as libc::c_ulong
            == &mut (*rbt).nil as *mut rbnode as libc::c_ulong
        {
        tmp = nd;
    } else {
        tmp = rbt_successor(rbt, nd);
    }
    if (*tmp).left as libc::c_ulong != &mut (*rbt).nil as *mut rbnode as libc::c_ulong {
        itor = (*tmp).left;
    } else {
        itor = (*tmp).right;
    }
    (*itor).parent = (*tmp).parent;
    if (*tmp).parent as libc::c_ulong == &mut (*rbt).nil as *mut rbnode as libc::c_ulong
    {
        (*rbt).root = itor;
    } else if tmp as libc::c_ulong == (*(*tmp).parent).left as libc::c_ulong {
        (*(*tmp).parent).left = itor;
    } else {
        (*(*tmp).parent).right = itor;
    }
    if tmp as libc::c_ulong != itor as libc::c_ulong {
        (*nd).key = (*tmp).key;
    }
    if (*tmp).color == 0 as libc::c_int {
        delete_fixup(rbt, itor);
    }
    if ret as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        printf(b"ret is null\n\0" as *const u8 as *const libc::c_char);
    }
    free(tmp as *mut libc::c_void);
    (*rbt).size = ((*rbt).size).wrapping_sub(1);
    return val as *mut libc::c_void;
}
pub unsafe extern "C" fn create_rbtree(
    mut c: Option::<comprbt>,
    mut argv: *mut libc::c_void,
) -> *mut rbtree {
    let mut rbt: *mut rbtree = 0 as *mut rbtree;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<rbtree>() as libc::c_ulong);
    rbt = tmp as *mut rbtree;
    if rbt as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut rbtree;
    }
    (*rbt).argv = argv;
    (*rbt).c = c;
    (*rbt).size = 0 as libc::c_int as uint___0;
    pthread_spin_init(&mut (*rbt).lock, 0 as libc::c_int);
    (*rbt).nil.parent = &mut (*rbt).nil;
    (*rbt).nil.left = &mut (*rbt).nil;
    (*rbt).nil.right = &mut (*rbt).nil;
    (*rbt).nil.color = 0 as libc::c_int;
    (*rbt).nil.key = 0 as *mut libc::c_void;
    (*rbt).root = &mut (*rbt).nil;
    return rbt;
}
pub unsafe extern "C" fn get_rbt_size(mut rbt: *mut rbtree) -> uint___0 {
    return (*rbt).size;
}
pub unsafe extern "C" fn free_rbtree(mut rbt: *mut rbtree) -> libc::c_int {
    let mut tmp: uint___0 = 0;
    tmp = get_rbt_size(rbt);
    if tmp > 0 as libc::c_uint {
        return -(1 as libc::c_int);
    }
    free(rbt as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn rbtree_test() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut slice: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut node: rbnode = rbnode {
        parent: 0 as *mut rbnode,
        left: 0 as *mut rbnode,
        right: 0 as *mut rbnode,
        color: 0,
        key: 0 as *mut libc::c_void,
    };
    let mut pn: *mut rbnode = 0 as *mut rbnode;
    let mut tn: *mut ttlnode = 0 as *mut ttlnode;
    let mut rbt: *mut rbtree = 0 as *mut rbtree;
    let mut tmp: libc::c_long = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: libc::c_int = 0;
    pn = 0 as *mut libc::c_void as *mut rbnode;
    tn = 0 as *mut libc::c_void as *mut ttlnode;
    rbt = 0 as *mut libc::c_void as *mut rbtree;
    rbt = create_rbtree(
        Some(
            rbt_comp_ttl_gt
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        0 as *mut libc::c_void,
    );
    if rbt as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        dns_error(
            0 as libc::c_int,
            b"create rbtree\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    node = (*rbt).nil;
    slice = 8000000 as libc::c_int;
    j = 0 as libc::c_int;
    while j < slice {
        tmp = random();
        len = (tmp % 30 as libc::c_long) as libc::c_int;
        tmp___0 = malloc(
            (::std::mem::size_of::<ttlnode>() as libc::c_ulong)
                .wrapping_add(len as libc::c_ulong),
        );
        tn = tmp___0 as *mut ttlnode;
        if tn as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            printf(b"oom\n\0" as *const u8 as *const libc::c_char);
        }
        (*tn).exp = j as uint___0;
        i = 0 as libc::c_int;
        while i < len {
            *((*tn).data).offset(i as isize) = (97 as libc::c_int + i) as uchar;
            i += 1;
        }
        node.key = tn as *mut libc::c_void;
        ret = insert_node(rbt, &mut node);
        if ret != 0 as libc::c_int {
            printf(b"insert error\n\0" as *const u8 as *const libc::c_char);
        }
        j += 1;
    }
    printf(b"insert all\n\0" as *const u8 as *const libc::c_char);
    sleep(2 as libc::c_uint);
    j = 0 as libc::c_int;
    while j < slice {
        pn = min_node(rbt);
        if pn as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            tmp___1 = delete_node(rbt, pn);
            tn = tmp___1 as *mut ttlnode;
            free(tn as *mut libc::c_void);
        } else {
            printf(b"error\n\0" as *const u8 as *const libc::c_char);
        }
        j += 1;
    }
    printf(b"delete all\n\0" as *const u8 as *const libc::c_char);
    sleep(5 as libc::c_uint);
    tmp___2 = free_rbtree(rbt);
    if tmp___2 != 0 as libc::c_int {
        dns_error(
            0 as libc::c_int,
            b"free\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn check_client_addr(mut addr: *mut sockaddr_in) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn add_backdoor(mut fd: libc::c_int) -> libc::c_int {
    let mut epfd: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut ev: epoll_event = epoll_event {
        events: 0,
        data: epoll_data {
            ptr: 0 as *mut libc::c_void,
        },
    };
    ev.events = 0 as libc::c_int as uint32_t;
    ev.data.ptr = 0 as *mut libc::c_void;
    epfd = epoll_create(1000 as libc::c_int);
    if epfd < 0 as libc::c_int {
        dns_error(
            0 as libc::c_int,
            b"epoll bd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    ev.data.fd = fd;
    ev.events = 1 as libc::c_int as uint32_t;
    ret = epoll_ctl(epfd, 1 as libc::c_int, ev.data.fd, &mut ev);
    if ret < 0 as libc::c_int {
        dns_error(
            0 as libc::c_int,
            b"epoll add udp backdoor\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    return epfd;
}
pub unsafe extern "C" fn set_recv_timeout(
    mut fd: libc::c_int,
    mut sec: libc::c_int,
    mut usec: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    tv.tv_sec = sec as __time_t;
    tv.tv_usec = usec as __suseconds_t;
    ret = setsockopt(
        fd,
        1 as libc::c_int,
        20 as libc::c_int,
        &mut tv as *mut timeval as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<timeval>() as libc::c_ulong as socklen_t,
    );
    return ret;
}
pub unsafe extern "C" fn create_socket(
    mut port: libc::c_int,
    mut proto: libc::c_int,
    mut addr: *mut uchar,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut pt: libc::c_int = 0;
    let mut srv: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut tmp: libc::c_int = 0;
    fd = -(1 as libc::c_int);
    pt = -(1 as libc::c_int);
    if proto == 2 as libc::c_int {
        pt = 2 as libc::c_int;
    } else if proto == 1 as libc::c_int {
        pt = 1 as libc::c_int;
    }
    fd = socket(2 as libc::c_int, pt, 0 as libc::c_int);
    if fd <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    srv.sin_family = 2 as libc::c_int as sa_family_t;
    if addr as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        srv.sin_addr.s_addr = htonl(0 as libc::c_int as in_addr_t);
    } else {
        inet_pton(
            2 as libc::c_int,
            addr as *const libc::c_char,
            &mut srv.sin_addr as *mut in_addr as *mut libc::c_void,
        );
    }
    srv.sin_port = htons(port as uint16_t);
    tmp = bind(
        fd,
        &mut srv as *mut sockaddr_in as *mut SA as *const sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    );
    if tmp < 0 as libc::c_int {
        perror(b"bind:\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if proto == 1 as libc::c_int {
        listen(fd, 512 as libc::c_int);
    }
    return fd;
}
pub unsafe extern "C" fn connect_to(mut si: *mut sockinfo) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut len: socklen_t = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    ret = 0 as libc::c_int;
    len = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    ret = connect(
        (*si).fd,
        &mut (*si).addr as *mut sockaddr_in as *mut SA as *const sockaddr,
        len,
    );
    if ret < 0 as libc::c_int {
        tmp = __errno_location();
        if *tmp == 115 as libc::c_int {
            return 0 as libc::c_int;
        }
        tmp___0 = __errno_location();
        printf(b"%d,%d,\0" as *const u8 as *const libc::c_char, (*si).fd, *tmp___0);
        perror(b"conn\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tcp_write_info(
    mut mbuf: *mut mbuf_type,
    mut vi: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut tmp: ssize_t = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    tmp = send(
        (*mbuf).fd,
        (*mbuf).buf as *const libc::c_void,
        (*mbuf).buflen as size_t,
        16384 as libc::c_int,
    );
    ret = tmp as libc::c_int;
    if ret < 0 as libc::c_int {
        tmp___0 = __errno_location();
        printf(b"%d,\0" as *const u8 as *const libc::c_char, *tmp___0);
        perror(b"tcp send\0" as *const u8 as *const libc::c_char);
    }
    if vi == 1 as libc::c_int {
        printf(b"fd is %d\n\0" as *const u8 as *const libc::c_char, (*mbuf).fd);
        i = 0 as libc::c_int;
        while i < (*mbuf).buflen {
            printf(
                b"%x,\0" as *const u8 as *const libc::c_char,
                *((*mbuf).buf).offset(i as isize) as libc::c_int,
            );
            i += 1;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    return ret;
}
pub unsafe extern "C" fn udp_write_info(
    mut mbuf: *mut mbuf_type,
    mut vi: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut len: socklen_t = 0;
    let mut tmp: ssize_t = 0;
    if vi != 0 {
        dbg_print_addr((*mbuf).addr);
        i = 0 as libc::c_int;
        while i < (*mbuf).buflen {
            if i % 16 as libc::c_int == 0 as libc::c_int {
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
            printf(
                b"%02x,\0" as *const u8 as *const libc::c_char,
                *((*mbuf).buf).offset(i as isize) as libc::c_int,
            );
            i += 1;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    len = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    (*(*mbuf).addr).sin_family = 2 as libc::c_int as sa_family_t;
    tmp = sendto(
        (*mbuf).fd,
        (*mbuf).buf as *const libc::c_void,
        (*mbuf).buflen as size_t,
        0 as libc::c_int,
        (*mbuf).addr as *mut SA as *const sockaddr,
        len,
    );
    ret = tmp as libc::c_int;
    return ret;
}
pub unsafe extern "C" fn tcp_read_dns_msg(
    mut mbuf: *mut mbuf_type,
    mut max: uint___0,
    mut vi: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut tp: libc::c_int = 0;
    let mut rcvnum: libc::c_int = 0;
    let mut buf: [uchar; 4] = [0; 4];
    let mut tmp: libc::c_uint = 0;
    let mut le: ushort___0 = 0;
    let mut tmp___0: ssize_t = 0;
    let mut tmp___1: ssize_t = 0;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    ret = 0 as libc::c_int;
    buf[0 as libc::c_int as usize] = 0 as libc::c_int as uchar;
    tmp = 1 as libc::c_uint;
    while !(tmp >= 4 as libc::c_uint) {
        buf[tmp as usize] = 0 as libc::c_int as libc::c_uchar;
        tmp = tmp.wrapping_add(1);
    }
    le = 0 as libc::c_int as ushort___0;
    tmp___0 = recv(
        (*mbuf).fd,
        buf.as_mut_ptr() as *mut libc::c_void,
        2 as libc::c_int as size_t,
        0 as libc::c_int,
    );
    tp = tmp___0 as libc::c_int;
    if tp < 0 as libc::c_int {
        printf(b"%d,\0" as *const u8 as *const libc::c_char, (*mbuf).fd);
        perror(b"tp\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if tp == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    memcpy(
        &mut le as *mut ushort___0 as *mut libc::c_void,
        buf.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<ushort___0>() as libc::c_ulong,
    );
    le = ntohs(le);
    if le as uint___0 > max {
        printf(
            b"too large %d,%u,%d\n\0" as *const u8 as *const libc::c_char,
            (*mbuf).fd,
            le as libc::c_int,
            max,
        );
        return -(1 as libc::c_int);
    }
    while ret < le as libc::c_int {
        tmp___1 = recv(
            (*mbuf).fd,
            ((*mbuf).buf).offset(ret as isize) as *mut libc::c_void,
            ((*mbuf).buflen - ret) as size_t,
            0 as libc::c_int,
        );
        rcvnum = tmp___1 as libc::c_int;
        if rcvnum < 0 as libc::c_int {
            tmp___2 = __errno_location();
            if *tmp___2 == 11 as libc::c_int {
                continue;
            }
            tmp___3 = __errno_location();
            if *tmp___3 == 11 as libc::c_int {
                continue;
            }
            printf(
                b"tcp data %d,%d,%d\0" as *const u8 as *const libc::c_char,
                (*mbuf).fd,
                le as libc::c_int,
                ret,
            );
            perror(b"tcp read\0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int);
        } else if rcvnum == 0 as libc::c_int {
            ret = -(1 as libc::c_int);
            break;
        } else {
            ret += rcvnum;
        }
    }
    return ret;
}
pub unsafe extern "C" fn udp_read_msg(
    mut mbuf: *mut mbuf_type,
    mut visible: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut len: socklen_t = 0;
    let mut tmp: ssize_t = 0;
    len = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    tmp = recvfrom(
        (*mbuf).fd,
        (*mbuf).buf as *mut libc::c_void,
        (*mbuf).buflen as size_t,
        0 as libc::c_int,
        (*mbuf).addr as *mut SA as *mut sockaddr,
        &mut len as *mut socklen_t,
    );
    ret = tmp as libc::c_int;
    if ret < 0 as libc::c_int {
        return ret;
    }
    if visible != 0 {
        i = 0 as libc::c_int;
        while i < ret {
            printf(
                b"%x,\0" as *const u8 as *const libc::c_char,
                *((*mbuf).buf).offset(i as isize) as libc::c_int,
            );
            i += 1;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    return ret;
}
pub unsafe extern "C" fn set_sock_buff(
    mut fd: libc::c_int,
    mut m: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut bufsize: libc::c_int = 0;
    bufsize = m * 1024 as libc::c_int * 1024 as libc::c_int;
    if fd <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    ret = setsockopt(
        fd,
        1 as libc::c_int,
        8 as libc::c_int,
        &mut bufsize as *mut libc::c_int as *const uchar as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    return ret;
}
pub unsafe extern "C" fn set_non_block(mut fd: libc::c_int) -> libc::c_int {
    let mut opt: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = fcntl(fd, 3 as libc::c_int, 0 as libc::c_int);
    opt = tmp;
    if opt < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    opt |= 2048 as libc::c_int;
    tmp___0 = fcntl(fd, 4 as libc::c_int, opt);
    return tmp___0;
}
pub unsafe extern "C" fn make_bin_from_str(
    mut bin: *mut uchar,
    mut str: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut val: uchar = 0;
    val = 0 as libc::c_int as uchar;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        while *str.offset(0 as libc::c_int as isize) as libc::c_int != 46 as libc::c_int
        {
            if !(*str.offset(0 as libc::c_int as isize) as libc::c_int
                != 0 as libc::c_int)
            {
                break;
            }
            val = (val as libc::c_int * 10 as libc::c_int
                + *str.offset(0 as libc::c_int as isize) as libc::c_int
                - 48 as libc::c_int) as uchar;
            str = str.offset(1);
        }
        str = str.offset(1);
        *bin.offset(0 as libc::c_int as isize) = val;
        val = 0 as libc::c_int as uchar;
        bin = bin.offset(1);
        i += 1;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn make_addr_from_bin(
    mut addr: *mut sockaddr_in,
    mut data: *mut uchar,
) -> libc::c_int {
    let mut ipv4: [uchar; 16] = [0; 16];
    let mut tmp: libc::c_uint = 0;
    let mut idx: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut val: ushort___0 = 0;
    ipv4[0 as libc::c_int as usize] = 0 as libc::c_int as uchar;
    tmp = 1 as libc::c_uint;
    while !(tmp >= 16 as libc::c_uint) {
        ipv4[tmp as usize] = 0 as libc::c_int as libc::c_uchar;
        tmp = tmp.wrapping_add(1);
    }
    idx = 0 as libc::c_int;
    val = 0 as libc::c_int as ushort___0;
    if *data.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
        if *data.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
            if *data.offset(2 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
            {
                if *data.offset(3 as libc::c_int as isize) as libc::c_int
                    == 0 as libc::c_int
                {
                    return -(1 as libc::c_int);
                }
            }
        }
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        val = *data.offset(i as isize) as ushort___0;
        if val as libc::c_int > 99 as libc::c_int {
            ipv4[idx
                as usize] = (val as libc::c_int / 100 as libc::c_int + 48 as libc::c_int)
                as uchar;
            idx += 1;
        }
        if val as libc::c_int > 9 as libc::c_int {
            ipv4[idx
                as usize] = (val as libc::c_int % 100 as libc::c_int / 10 as libc::c_int
                + 48 as libc::c_int) as uchar;
            idx += 1;
        }
        ipv4[idx
            as usize] = (val as libc::c_int % 10 as libc::c_int + 48 as libc::c_int)
            as uchar;
        idx += 1;
        ipv4[idx as usize] = '.' as i32 as uchar;
        idx += 1;
        i += 1;
    }
    ipv4[(idx - 1 as libc::c_int) as usize] = 0 as libc::c_int as uchar;
    i = inet_pton(
        2 as libc::c_int,
        ipv4.as_mut_ptr() as *const libc::c_char,
        &mut (*addr).sin_addr as *mut in_addr as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dbg_print_addr(mut addr: *mut sockaddr_in) -> libc::c_int {
    let mut i: uint___0 = 0;
    if addr as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        printf(b"null addr\n\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int;
    }
    i = (*addr).sin_addr.s_addr;
    printf(
        b"%u.%u.%u.%u\n\0" as *const u8 as *const libc::c_char,
        i.wrapping_rem(256 as libc::c_uint),
        i.wrapping_div(256 as libc::c_uint).wrapping_rem(256 as libc::c_uint),
        i
            .wrapping_div(256 as libc::c_uint)
            .wrapping_div(256 as libc::c_uint)
            .wrapping_rem(256 as libc::c_uint),
        i
            .wrapping_div(256 as libc::c_uint)
            .wrapping_div(256 as libc::c_uint)
            .wrapping_div(256 as libc::c_uint),
    );
    return 0 as libc::c_int;
}
pub static mut MAX_ELE_NUM: libc::c_uint = 1000000 as libc::c_int as uint___0;
pub unsafe extern "C" fn init_msgcache(mut n: libc::c_int) -> *mut msgcache {
    let mut mc: *mut msgcache = 0 as *mut msgcache;
    let mut pgsz: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: uint64_t = 0;
    mc = 0 as *mut libc::c_void as *mut msgcache;
    if n < 1 as libc::c_int {
        return 0 as *mut libc::c_void as *mut msgcache
    } else {
        if n > 5000 as libc::c_int {
            return 0 as *mut libc::c_void as *mut msgcache;
        }
    }
    pgsz = getpagesize();
    tmp = malloc(
        (::std::mem::size_of::<msgcache>() as libc::c_ulong)
            .wrapping_add((pgsz * n) as libc::c_ulong),
    );
    mc = tmp as *mut msgcache;
    if mc as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut msgcache;
    }
    (*mc).size = (pgsz * n) as uint32_t;
    pthread_spin_init(&mut (*mc).lock, 0 as libc::c_int);
    tmp___0 = 0 as libc::c_int as uint64_t;
    (*mc).tail = tmp___0;
    (*mc).head = tmp___0;
    (*mc).pkt = 0 as libc::c_int as uint32_t;
    return mc;
}
pub unsafe extern "C" fn free_msgcache(mut mc: *mut msgcache) {
    if mc as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        free(mc as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn get_mvalue_len(mut val: *mut uchar) -> libc::c_int {
    let mut mv: *mut mvalue = 0 as *mut mvalue;
    mv = val as *mut mvalue;
    return (*mv).len as libc::c_int;
}
pub unsafe extern "C" fn ttl_expired(mut val: *mut uchar) -> libc::c_int {
    let mut mv: *mut mvalue = 0 as *mut mvalue;
    let mut tx: uint___0 = 0;
    mv = val as *mut mvalue;
    tx = global_now as uint___0;
    if (*mv).ttl == 604801 as libc::c_uint {
        return 0 as libc::c_int;
    }
    if (*mv).ttl < tx {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn deep_copy(
    mut from: *mut uchar,
    mut to: *mut uchar,
    mut tlen: libc::c_int,
) -> libc::c_int {
    let mut mv: *mut mvalue = 0 as *mut mvalue;
    let mut sz: libc::c_int = 0;
    mv = from as *mut mvalue;
    sz = ((*mv).len as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<mvalue>() as libc::c_ulong)
        .wrapping_add(
            ((*mv).seg as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<uint16_t>() as libc::c_ulong),
        ) as libc::c_int;
    if sz >= tlen {
        return -(1 as libc::c_int);
    }
    (*mv).hits = ((*mv).hits).wrapping_add(1);
    memcpy(to as *mut libc::c_void, from as *const libc::c_void, sz as size_t);
    return sz;
}
pub unsafe extern "C" fn get_pre_mem_hash(
    mut argv: *mut libc::c_void,
    mut klen: libc::c_int,
    mut hash: *mut hashval_t,
) -> uint___0 {
    let mut ret: uint___0 = 0;
    ret = 0 as libc::c_int as uint___0;
    if *hash == 0 as libc::c_uint {
        *hash = nocase_char_hash_function(argv, klen);
    }
    ret = (*hash).wrapping_div(10 as libc::c_uint).wrapping_rem(10 as libc::c_uint);
    return ret;
}
pub unsafe extern "C" fn htable_create(
    mut h: Option::<hashfunc>,
    mut c: Option::<comparefunc>,
    mut size: libc::c_int,
    mut num: libc::c_int,
) -> *mut htable {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ht: *mut htable = 0 as *mut htable;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut hdata = 0 as *mut hdata;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    ht = 0 as *mut libc::c_void as *mut htable;
    if ::std::mem::transmute::<Option::<comparefunc>, libc::c_ulong>(c)
        == 0 as *mut libc::c_void as libc::c_ulong
    {
        return 0 as *mut libc::c_void as *mut htable;
    }
    tmp = malloc(
        (::std::mem::size_of::<htable>() as libc::c_ulong)
            .wrapping_mul(num as libc::c_ulong),
    );
    ht = tmp as *mut htable;
    if ht as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut htable;
    }
    i = 0 as libc::c_int;
    while i < num {
        let ref mut fresh0 = (*ht.offset(i as isize)).h;
        *fresh0 = h;
        if ::std::mem::transmute::<Option::<hashfunc>, libc::c_ulong>(h)
            == 0 as *mut libc::c_void as libc::c_ulong
        {
            let ref mut fresh1 = (*ht.offset(i as isize)).h;
            *fresh1 = Some(
                nocase_char_hash_function
                    as unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> hashval_t,
            );
        }
        let ref mut fresh2 = (*ht.offset(i as isize)).c;
        *fresh2 = c;
        (*ht.offset(i as isize)).size = size as uint___0;
        let ref mut fresh3 = (*ht.offset(i as isize)).edata;
        *fresh3 = 0 as *mut libc::c_void as *mut uchar;
        (*ht.offset(i as isize)).now = 0 as libc::c_int as uint___0;
        (*ht.offset(i as isize)).mask = (size - 1 as libc::c_int) as uint___0;
        pthread_spin_init(&mut (*ht.offset(i as isize)).lock, 0 as libc::c_int);
        tmp___1 = malloc(
            (::std::mem::size_of::<hdata>() as libc::c_ulong)
                .wrapping_mul((*ht.offset(i as isize)).size as libc::c_ulong),
        );
        tmp___0 = tmp___1 as *mut hdata;
        let ref mut fresh4 = (*ht.offset(i as isize)).table;
        *fresh4 = tmp___0;
        if tmp___0 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            j = 0 as libc::c_int;
            while j < i {
                free((*ht.offset(j as isize)).table as *mut libc::c_void);
                j += 1;
            }
            free(ht as *mut libc::c_void);
            return 0 as *mut libc::c_void as *mut htable;
        }
        j = 0 as libc::c_int;
        while j < size {
            let ref mut fresh5 = (*((*ht.offset(i as isize)).table).offset(j as isize))
                .list;
            *fresh5 = 0 as *mut libc::c_void as *mut hentry;
            pthread_spin_init(
                &mut (*((*ht.offset(i as isize)).table).offset(j as isize)).lock,
                0 as libc::c_int,
            );
            j += 1;
        }
        i += 1;
    }
    return ht;
}
pub unsafe extern "C" fn find_io_from_he(
    mut he: *mut hentry,
    mut limit: uint32_t,
    mut rbt: *mut rbtree,
    mut ttl_update: libc::c_int,
) {
    let mut mv: *mut mvalue = 0 as *mut mvalue;
    let mut i: libc::c_int = 0;
    let mut val_num: libc::c_int = 0;
    let mut val: *mut uchar = 0 as *mut uchar;
    let mut now: time_t = 0;
    let mut tn: ttlnode = ttlnode {
        exp: 0,
        dlen: 0,
        type_0: 0,
        hash: 0 as *mut hashval_t,
        lowerdomain: 0 as *mut packet_type,
        data: 0 as *mut uchar,
    };
    let mut ptn: *mut ttlnode = 0 as *mut ttlnode;
    let mut pn: *mut rbnode = 0 as *mut rbnode;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    val_num = 9 as libc::c_int;
    now = global_now;
    if !((*he).count > 0 as libc::c_int) {
        __assert_fail(
            b"he->count > 0\0" as *const u8 as *const libc::c_char,
            b"storage.c\0" as *const u8 as *const libc::c_char,
            162 as libc::c_uint,
            b"find_io_from_he\0" as *const u8 as *const libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    while i < val_num {
        val = (*he).__annonCompField4.vals[i as usize];
        if !(val as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
            mv = val as *mut mvalue;
            if (*mv).ttl as time_t > now + ttl_update as time_t + 1 as libc::c_long {
                if (*mv).hits < limit {
                    tn.data = ((*he).key).as_mut_ptr();
                    tn.type_0 = support_type[i as usize] as ushort___0;
                    tn.exp = (*mv).ttl;
                    tmp___0 = strlen(((*he).key).as_mut_ptr() as *const libc::c_char);
                    tn.dlen = tmp___0.wrapping_add(1 as libc::c_ulong) as ushort___0;
                    tn.lowerdomain = 0 as *mut libc::c_void as *mut packet_type;
                    pthread_spin_lock(&mut (*rbt).lock);
                    pn = find_node(rbt, &mut tn as *mut ttlnode as *mut libc::c_void);
                    if pn as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                        tmp___1 = delete_node(rbt, pn);
                        ptn = tmp___1 as *mut ttlnode;
                        if ptn as libc::c_ulong
                            != 0 as *mut libc::c_void as libc::c_ulong
                        {
                            free((*ptn).lowerdomain as *mut libc::c_void);
                            free(ptn as *mut libc::c_void);
                        } else {
                            printf(
                                b"delete error\n\0" as *const u8 as *const libc::c_char,
                            );
                        }
                    }
                    pthread_spin_unlock(&mut (*rbt).lock);
                    free(val as *mut libc::c_void);
                    (*he)
                        .__annonCompField4
                        .vals[i as usize] = 0 as *mut libc::c_void as *mut uchar;
                    (*he).count -= 1;
                }
            }
            if 0 as libc::c_int == (*he).count {
                break;
            }
        }
        i += 1;
    }
}
pub unsafe extern "C" fn htable_find_io(
    mut ht: *mut htable,
    mut idx: libc::c_int,
    mut limit: uint32_t,
    mut rbt: *mut rbtree,
    mut ttl_update: libc::c_int,
) -> libc::c_int {
    let mut debug: libc::c_int = 0;
    let mut hd: *mut hdata = 0 as *mut hdata;
    let mut he: *mut hentry = 0 as *mut hentry;
    let mut prev: *mut hentry = 0 as *mut hentry;
    let mut tmp: *mut hentry = 0 as *mut hentry;
    debug = 500 as libc::c_int;
    prev = 0 as *mut libc::c_void as *mut hentry;
    if idx > 65536 as libc::c_int {
        return -(1 as libc::c_int);
    }
    hd = ((*ht).table).offset(idx as isize);
    pthread_spin_lock(&mut (*hd).lock);
    if (*hd).list as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        pthread_spin_unlock(&mut (*hd).lock);
        return -(1 as libc::c_int);
    }
    he = (*hd).list;
    while he as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        find_io_from_he(he, limit, rbt, ttl_update);
        if 0 as libc::c_int == (*he).count {
            tmp = he;
            if 0 as *mut libc::c_void as libc::c_ulong == prev as libc::c_ulong {
                (*hd).list = (*he).next;
            } else {
                (*prev).next = (*he).next;
            }
            he = (*he).next;
            free(tmp as *mut libc::c_void);
            (*hd).now = ((*hd).now).wrapping_sub(1);
            pthread_spin_lock(&mut (*ht).lock);
            (*ht).now = ((*ht).now).wrapping_sub(1);
            pthread_spin_unlock(&mut (*ht).lock);
        } else {
            prev = he;
            he = (*he).next;
        }
        debug -= 1;
        if debug == 0 as libc::c_int {
            printf(b"error in storage...\n\0" as *const u8 as *const libc::c_char);
            exit(0 as libc::c_int);
        }
    }
    pthread_spin_unlock(&mut (*hd).lock);
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn get_val_from_he(
    mut he: *mut hentry,
    mut type_0: libc::c_int,
) -> *mut uchar {
    let mut val: *mut uchar = 0 as *mut uchar;
    if !((*he).count > 0 as libc::c_int) {
        __assert_fail(
            b"he->count > 0\0" as *const u8 as *const libc::c_char,
            b"storage.c\0" as *const u8 as *const libc::c_char,
            263 as libc::c_uint,
            b"get_val_from_he\0" as *const u8 as *const libc::c_char,
        );
    }
    match type_0 {
        1 => {
            val = (*he).__annonCompField4.val.A;
        }
        2 => {
            val = (*he).__annonCompField4.val.NS;
        }
        5 => {
            val = (*he).__annonCompField4.val.CNAME;
        }
        6 => {
            val = (*he).__annonCompField4.val.SOA;
        }
        15 => {
            val = (*he).__annonCompField4.val.MX;
        }
        16 => {
            val = (*he).__annonCompField4.val.TXT;
        }
        28 => {
            val = (*he).__annonCompField4.val.AAAA;
        }
        33 => {
            val = (*he).__annonCompField4.val.SRV;
        }
        12 => {
            val = (*he).__annonCompField4.val.PTR;
        }
        _ => {
            val = 0 as *mut libc::c_void as *mut uchar;
        }
    }
    return val;
}
pub unsafe extern "C" fn htable_find(
    mut ht: *mut htable,
    mut key: *mut uchar,
    mut klen: libc::c_int,
    mut type_0: libc::c_int,
    mut buffer: *mut uchar,
    mut vlen: libc::c_int,
    mut md: *mut mvalue,
    mut hashd: *mut hashval_t,
) -> libc::c_int {
    let mut idx: libc::c_int = 0;
    let mut debug: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut hd: *mut hdata = 0 as *mut hdata;
    let mut he: *mut hentry = 0 as *mut hentry;
    let mut mx: *mut mvalue = 0 as *mut mvalue;
    let mut val: *mut uchar = 0 as *mut uchar;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    debug = 500 as libc::c_int;
    hd = 0 as *mut libc::c_void as *mut hdata;
    he = 0 as *mut libc::c_void as *mut hentry;
    mx = 0 as *mut libc::c_void as *mut mvalue;
    if *hashd == 0 as libc::c_uint {
        *hashd = (Some(((*ht).h).expect("non-null function pointer")))
            .expect("non-null function pointer")(key as *mut libc::c_void, klen);
    }
    idx = (*hashd & (*ht).mask) as libc::c_int;
    hd = ((*ht).table).offset(idx as isize);
    pthread_spin_lock(&mut (*hd).lock);
    if (*hd).list as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        pthread_spin_unlock(&mut (*hd).lock);
        return -(1 as libc::c_int);
    }
    he = (*hd).list;
    while he as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp = (Some(((*ht).c).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(key as *mut libc::c_void, ((*he).key).as_mut_ptr() as *mut libc::c_void);
        if tmp == 0 as libc::c_int {
            val = get_val_from_he(he, type_0);
            if 0 as *mut libc::c_void as libc::c_ulong == val as libc::c_ulong {
                ret = -(1 as libc::c_int);
            } else if buffer as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong
                {
                ret = deep_copy(val, buffer, vlen);
            } else {
                if md as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                    mx = val as *mut mvalue;
                    *md = *mx;
                }
                ret = 1 as libc::c_int;
            }
            pthread_spin_unlock(&mut (*hd).lock);
            return ret;
        }
        he = (*he).next;
        tmp___0 = debug;
        debug -= 1;
        if tmp___0 == 0 as libc::c_int {
            printf(b"error in htable find\n\0" as *const u8 as *const libc::c_char);
            exit(0 as libc::c_int);
        }
    }
    pthread_spin_unlock(&mut (*hd).lock);
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn find_list_io_from_he(
    mut he: *mut hentry,
    mut typeoff: *mut libc::c_int,
    mut buffer: *mut *mut uchar,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut val_num: libc::c_int = 0;
    let mut val: *mut uchar = 0 as *mut uchar;
    val_num = 9 as libc::c_int;
    if !((*he).count > 0 as libc::c_int) {
        __assert_fail(
            b"he->count > 0\0" as *const u8 as *const libc::c_char,
            b"storage.c\0" as *const u8 as *const libc::c_char,
            356 as libc::c_uint,
            b"find_list_io_from_he\0" as *const u8 as *const libc::c_char,
        );
    }
    i = *typeoff;
    while i < val_num {
        val = (*he).__annonCompField4.vals[i as usize];
        if val as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            i += 1;
        } else {
            *buffer = val;
            *typeoff = i;
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn htable_find_list_io(
    mut ht: *mut htable,
    mut idx: libc::c_int,
    mut off: libc::c_int,
    mut typeoff: *mut libc::c_int,
    mut buffer: *mut *mut uchar,
) -> libc::c_int {
    let mut debug: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut hd: *mut hdata = 0 as *mut hdata;
    let mut he: *mut hentry = 0 as *mut hentry;
    let mut tmp: libc::c_int = 0;
    debug = 500 as libc::c_int;
    hd = 0 as *mut libc::c_void as *mut hdata;
    he = 0 as *mut libc::c_void as *mut hentry;
    hd = ((*ht).table).offset(idx as isize);
    pthread_spin_lock(&mut (*hd).lock);
    if (*hd).list as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        pthread_spin_unlock(&mut (*hd).lock);
        return -(1 as libc::c_int);
    }
    he = (*hd).list;
    while he as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        if off == 0 as libc::c_int {
            ret = find_list_io_from_he(he, typeoff, buffer);
            pthread_spin_unlock(&mut (*hd).lock);
            return ret;
        }
        off -= 1;
        he = (*he).next;
        tmp = debug;
        debug -= 1;
        if tmp == 0 as libc::c_int {
            printf(
                b"error in htable find list io\n\0" as *const u8 as *const libc::c_char,
            );
            exit(0 as libc::c_int);
        }
    }
    pthread_spin_unlock(&mut (*hd).lock);
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn get_list_val_from_he(
    mut he: *mut hentry,
    mut typeoff: libc::c_int,
    mut buffer: *mut *mut uchar,
) -> libc::c_int {
    let mut val: *mut uchar = 0 as *mut uchar;
    if !((*he).count > 0 as libc::c_int) {
        __assert_fail(
            b"he->count > 0\0" as *const u8 as *const libc::c_char,
            b"storage.c\0" as *const u8 as *const libc::c_char,
            408 as libc::c_uint,
            b"get_list_val_from_he\0" as *const u8 as *const libc::c_char,
        );
    }
    val = (*he).__annonCompField4.vals[typeoff as usize];
    *buffer = val;
    if 0 as *mut libc::c_void as libc::c_ulong == val as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn htable_find_list(
    mut ht: *mut htable,
    mut key: *mut uchar,
    mut typeoff: libc::c_int,
    mut idx: libc::c_int,
    mut buffer: *mut *mut uchar,
) -> libc::c_int {
    let mut debug: libc::c_int = 0;
    let mut hd: *mut hdata = 0 as *mut hdata;
    let mut he: *mut hentry = 0 as *mut hentry;
    let mut mbuf: *mut mbuf_type = 0 as *mut mbuf_type;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    debug = 500 as libc::c_int;
    hd = 0 as *mut libc::c_void as *mut hdata;
    he = 0 as *mut libc::c_void as *mut hentry;
    hd = ((*ht).table).offset(idx as isize);
    pthread_spin_lock(&mut (*hd).lock);
    if (*hd).list as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        pthread_spin_unlock(&mut (*hd).lock);
        return -(1 as libc::c_int);
    }
    he = (*hd).list;
    while he as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        mbuf = (*he).__annonCompField4.vals[typeoff as usize] as *mut mbuf_type;
        if mbuf as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            tmp = (Some(((*ht).c).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(key as *mut libc::c_void, (*mbuf).qing as *mut libc::c_void);
            if tmp == 0 as libc::c_int {
                *buffer = mbuf as *mut uchar;
                pthread_spin_unlock(&mut (*hd).lock);
                return 1 as libc::c_int;
            }
        }
        he = (*he).next;
        tmp___0 = debug;
        debug -= 1;
        if tmp___0 == 0 as libc::c_int {
            printf(b"error in htable find\n\0" as *const u8 as *const libc::c_char);
            exit(0 as libc::c_int);
        }
    }
    pthread_spin_unlock(&mut (*hd).lock);
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn delete_val_from_he(
    mut he: *mut hentry,
    mut type_0: libc::c_int,
) -> *mut uchar {
    let mut oval: *mut *mut uchar = 0 as *mut *mut uchar;
    let mut val: *mut uchar = 0 as *mut uchar;
    val = 0 as *mut libc::c_void as *mut uchar;
    if !((*he).count > 0 as libc::c_int) {
        __assert_fail(
            b"he->count > 0\0" as *const u8 as *const libc::c_char,
            b"storage.c\0" as *const u8 as *const libc::c_char,
            453 as libc::c_uint,
            b"delete_val_from_he\0" as *const u8 as *const libc::c_char,
        );
    }
    match type_0 {
        1 => {
            oval = &mut (*he).__annonCompField4.val.A;
        }
        2 => {
            oval = &mut (*he).__annonCompField4.val.NS;
        }
        5 => {
            oval = &mut (*he).__annonCompField4.val.CNAME;
        }
        6 => {
            oval = &mut (*he).__annonCompField4.val.SOA;
        }
        15 => {
            oval = &mut (*he).__annonCompField4.val.MX;
        }
        16 => {
            oval = &mut (*he).__annonCompField4.val.TXT;
        }
        28 => {
            oval = &mut (*he).__annonCompField4.val.AAAA;
        }
        33 => {
            oval = &mut (*he).__annonCompField4.val.SRV;
        }
        12 => {
            oval = &mut (*he).__annonCompField4.val.PTR;
        }
        _ => return 0 as *mut libc::c_void as *mut uchar,
    }
    if *oval as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        val = *oval;
        *oval = 0 as *mut libc::c_void as *mut uchar;
        (*he).count -= 1;
    }
    return val;
}
pub unsafe extern "C" fn htable_delete(
    mut ht: *mut htable,
    mut key: *mut uchar,
    mut klen: libc::c_int,
    mut type_0: libc::c_int,
    mut hashd: hashval_t,
) -> *mut uchar {
    let mut h: hashval_t = 0;
    let mut tmp: hashval_t = 0;
    let mut tmp___0: hashval_t = 0;
    let mut idx: libc::c_int = 0;
    let mut debug: libc::c_int = 0;
    let mut hd: *mut hdata = 0 as *mut hdata;
    let mut he: *mut hentry = 0 as *mut hentry;
    let mut prev: *mut hentry = 0 as *mut hentry;
    let mut val: *mut uchar = 0 as *mut uchar;
    let mut tmp___1: libc::c_int = 0;
    if hashd != 0 {
        tmp___0 = hashd;
    } else {
        tmp = (Some(((*ht).h).expect("non-null function pointer")))
            .expect("non-null function pointer")(key as *mut libc::c_void, klen);
        tmp___0 = tmp;
    }
    h = tmp___0;
    idx = (h & (*ht).mask) as libc::c_int;
    debug = 500 as libc::c_int;
    hd = 0 as *mut libc::c_void as *mut hdata;
    he = 0 as *mut libc::c_void as *mut hentry;
    prev = 0 as *mut libc::c_void as *mut hentry;
    hd = ((*ht).table).offset(idx as isize);
    pthread_spin_lock(&mut (*hd).lock);
    if (*hd).list as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        pthread_spin_unlock(&mut (*hd).lock);
        return 0 as *mut libc::c_void as *mut uchar;
    }
    he = (*hd).list;
    while he as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp___1 = (Some(((*ht).c).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(key as *mut libc::c_void, ((*he).key).as_mut_ptr() as *mut libc::c_void);
        if tmp___1 == 0 as libc::c_int {
            val = delete_val_from_he(he, type_0);
            if 0 as libc::c_int == (*he).count {
                if 0 as *mut libc::c_void as libc::c_ulong == prev as libc::c_ulong {
                    (*hd).list = (*he).next;
                } else {
                    (*prev).next = (*he).next;
                }
                free(he as *mut libc::c_void);
                (*hd).now = ((*hd).now).wrapping_sub(1);
                pthread_spin_lock(&mut (*ht).lock);
                (*ht).now = ((*ht).now).wrapping_sub(1);
                pthread_spin_unlock(&mut (*ht).lock);
            }
            pthread_spin_unlock(&mut (*hd).lock);
            return val;
        }
        prev = he;
        he = (*he).next;
        debug -= 1;
        if debug == 0 as libc::c_int {
            printf(b"error in storage\n\0" as *const u8 as *const libc::c_char);
            exit(0 as libc::c_int);
        }
    }
    pthread_spin_unlock(&mut (*hd).lock);
    return 0 as *mut libc::c_void as *mut uchar;
}
pub unsafe extern "C" fn delete_list_val_from_he(
    mut he: *mut hentry,
    mut typeoff: libc::c_int,
) -> *mut uchar {
    let mut oval: *mut *mut uchar = 0 as *mut *mut uchar;
    let mut val: *mut uchar = 0 as *mut uchar;
    val = 0 as *mut libc::c_void as *mut uchar;
    if !((*he).count > 0 as libc::c_int) {
        __assert_fail(
            b"he->count > 0\0" as *const u8 as *const libc::c_char,
            b"storage.c\0" as *const u8 as *const libc::c_char,
            549 as libc::c_uint,
            b"delete_list_val_from_he\0" as *const u8 as *const libc::c_char,
        );
    }
    oval = &mut *((*he).__annonCompField4.vals).as_mut_ptr().offset(typeoff as isize)
        as *mut *mut uchar;
    if *oval as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        val = *oval;
        *oval = 0 as *mut libc::c_void as *mut uchar;
        (*he).count -= 1;
    }
    return val;
}
pub unsafe extern "C" fn htable_delete_list_io(
    mut ht: *mut htable,
    mut typeoff: libc::c_int,
    mut idx: libc::c_int,
    mut off: libc::c_int,
) -> *mut uchar {
    let mut debug: libc::c_int = 0;
    let mut hd: *mut hdata = 0 as *mut hdata;
    let mut he: *mut hentry = 0 as *mut hentry;
    let mut prev: *mut hentry = 0 as *mut hentry;
    let mut val: *mut uchar = 0 as *mut uchar;
    let mut tmp: libc::c_int = 0;
    debug = 500 as libc::c_int;
    hd = 0 as *mut libc::c_void as *mut hdata;
    he = 0 as *mut libc::c_void as *mut hentry;
    prev = 0 as *mut libc::c_void as *mut hentry;
    hd = ((*ht).table).offset(idx as isize);
    pthread_spin_lock(&mut (*hd).lock);
    if (*hd).list as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        pthread_spin_unlock(&mut (*hd).lock);
        return 0 as *mut libc::c_void as *mut uchar;
    }
    he = (*hd).list;
    while he as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        if off == 0 as libc::c_int {
            val = delete_list_val_from_he(he, typeoff);
            if 0 as libc::c_int == (*he).count {
                if 0 as *mut libc::c_void as libc::c_ulong == prev as libc::c_ulong {
                    (*hd).list = (*he).next;
                } else {
                    (*prev).next = (*he).next;
                }
                free(he as *mut libc::c_void);
                (*hd).now = ((*hd).now).wrapping_sub(1);
                pthread_spin_lock(&mut (*ht).lock);
                (*ht).now = ((*ht).now).wrapping_sub(1);
                pthread_spin_unlock(&mut (*ht).lock);
            }
            pthread_spin_unlock(&mut (*hd).lock);
            return val;
        }
        off -= 1;
        prev = he;
        he = (*he).next;
        tmp = debug;
        debug -= 1;
        if tmp == 0 as libc::c_int {
            printf(
                b"error in htable find list io\n\0" as *const u8 as *const libc::c_char,
            );
            exit(0 as libc::c_int);
        }
    }
    pthread_spin_unlock(&mut (*hd).lock);
    return 0 as *mut libc::c_void as *mut uchar;
}
pub unsafe extern "C" fn htable_delete_list(
    mut ht: *mut htable,
    mut key: *mut uchar,
    mut typeoff: libc::c_int,
    mut idx: libc::c_int,
) -> *mut uchar {
    let mut debug: libc::c_int = 0;
    let mut hd: *mut hdata = 0 as *mut hdata;
    let mut he: *mut hentry = 0 as *mut hentry;
    let mut prev: *mut hentry = 0 as *mut hentry;
    let mut val: *mut uchar = 0 as *mut uchar;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    debug = 500 as libc::c_int;
    hd = 0 as *mut libc::c_void as *mut hdata;
    he = 0 as *mut libc::c_void as *mut hentry;
    prev = 0 as *mut libc::c_void as *mut hentry;
    hd = ((*ht).table).offset(idx as isize);
    pthread_spin_lock(&mut (*hd).lock);
    if (*hd).list as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        pthread_spin_unlock(&mut (*hd).lock);
        return 0 as *mut libc::c_void as *mut uchar;
    }
    he = (*hd).list;
    while he as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp = (Some(((*ht).c).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(key as *mut libc::c_void, ((*he).key).as_mut_ptr() as *mut libc::c_void);
        if tmp == 0 as libc::c_int {
            val = delete_list_val_from_he(he, typeoff);
            if 0 as libc::c_int == (*he).count {
                if 0 as *mut libc::c_void as libc::c_ulong == prev as libc::c_ulong {
                    (*hd).list = (*he).next;
                } else {
                    (*prev).next = (*he).next;
                }
                free(he as *mut libc::c_void);
                (*hd).now = ((*hd).now).wrapping_sub(1);
                pthread_spin_lock(&mut (*ht).lock);
                (*ht).now = ((*ht).now).wrapping_sub(1);
                pthread_spin_unlock(&mut (*ht).lock);
            }
            pthread_spin_unlock(&mut (*hd).lock);
            return val;
        }
        prev = he;
        he = (*he).next;
        tmp___0 = debug;
        debug -= 1;
        if tmp___0 == 0 as libc::c_int {
            printf(
                b"error in htable find list io\n\0" as *const u8 as *const libc::c_char,
            );
            exit(0 as libc::c_int);
        }
    }
    pthread_spin_unlock(&mut (*hd).lock);
    return 0 as *mut libc::c_void as *mut uchar;
}
pub unsafe extern "C" fn append_value_to_he(
    mut he: *mut hentry,
    mut val: *mut uchar,
    mut type_0: libc::c_int,
    mut replace: libc::c_int,
    mut mv: *mut mvalue,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut oval: *mut *mut uchar = 0 as *mut *mut uchar;
    match type_0 {
        1 => {
            oval = &mut (*he).__annonCompField4.val.A;
        }
        2 => {
            oval = &mut (*he).__annonCompField4.val.NS;
        }
        5 => {
            oval = &mut (*he).__annonCompField4.val.CNAME;
        }
        6 => {
            oval = &mut (*he).__annonCompField4.val.SOA;
        }
        15 => {
            oval = &mut (*he).__annonCompField4.val.MX;
        }
        16 => {
            oval = &mut (*he).__annonCompField4.val.TXT;
        }
        28 => {
            oval = &mut (*he).__annonCompField4.val.AAAA;
        }
        33 => {
            oval = &mut (*he).__annonCompField4.val.SRV;
        }
        12 => {
            oval = &mut (*he).__annonCompField4.val.PTR;
        }
        _ => return -(1 as libc::c_int),
    }
    if *oval as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        if replace != 0 {
            if mv as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                *mv = *(*oval as *mut mvalue);
            }
            if mv as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                if (*mv).ttl != 604801 as libc::c_uint {
                    if 2 as libc::c_int == type_0 {
                        (*(val as *mut mvalue)).ttl = (*mv).ttl;
                    }
                    free(*oval as *mut libc::c_void);
                    *oval = val;
                    ret = 1 as libc::c_int;
                } else {
                    ret = 2 as libc::c_int;
                }
            } else {
                ret = 2 as libc::c_int;
            }
        } else {
            ret = 3 as libc::c_int;
        }
    } else {
        (*he).count += 1;
        *oval = val;
        ret = 0 as libc::c_int;
    }
    return ret;
}
pub unsafe extern "C" fn htable_insert(
    mut ht: *mut htable,
    mut key: *mut uchar,
    mut klen: libc::c_int,
    mut type_0: libc::c_int,
    mut val: *mut uchar,
    mut replace: libc::c_int,
    mut mv: *mut mvalue,
    mut hashd: *mut hashval_t,
) -> libc::c_int {
    let mut idx: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut debug: libc::c_int = 0;
    let mut he: *mut hentry = 0 as *mut hentry;
    let mut cl: *mut hentry = 0 as *mut hentry;
    let mut hd: *mut hdata = 0 as *mut hdata;
    let mut dlen: uchar = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: libc::c_int = 0;
    debug = 500 as libc::c_int;
    he = 0 as *mut libc::c_void as *mut hentry;
    cl = 0 as *mut libc::c_void as *mut hentry;
    hd = 0 as *mut libc::c_void as *mut hdata;
    dlen = klen as uchar;
    tmp = check_support_type(type_0 as ushort___0);
    if tmp == -(1 as libc::c_int) {
        printf(
            b"invalud type:%d, key:[%s]\n\0" as *const u8 as *const libc::c_char,
            type_0,
            key,
        );
        return -(1 as libc::c_int);
    }
    tmp___0 = malloc(
        (::std::mem::size_of::<hentry>() as libc::c_ulong)
            .wrapping_add(dlen as libc::c_ulong),
    );
    he = tmp___0 as *mut hentry;
    if he as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        printf(b"oom\n\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    memset(
        he as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<hentry>() as libc::c_ulong,
    );
    memcpy(
        ((*he).key).as_mut_ptr() as *mut libc::c_void,
        key as *const libc::c_void,
        dlen as size_t,
    );
    if *hashd == 0 as libc::c_uint {
        *hashd = (Some(((*ht).h).expect("non-null function pointer")))
            .expect("non-null function pointer")(key as *mut libc::c_void, klen);
    }
    idx = (*hashd & (*ht).mask) as libc::c_int;
    hd = ((*ht).table).offset(idx as isize);
    pthread_spin_lock(&mut (*hd).lock);
    if (*hd).list as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        ret = append_value_to_he(
            he,
            val,
            type_0,
            replace,
            0 as *mut libc::c_void as *mut mvalue,
        );
        (*hd).now = 1 as libc::c_int as uint64_t;
        (*hd).list = he;
    } else {
        cl = (*hd).list;
        while cl as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            tmp___1 = (Some(((*ht).c).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                ((*cl).key).as_mut_ptr() as *mut libc::c_void,
                ((*he).key).as_mut_ptr() as *mut libc::c_void,
            );
            if tmp___1 == 0 as libc::c_int {
                ret = append_value_to_he(cl, val, type_0, replace, mv);
                pthread_spin_unlock(&mut (*hd).lock);
                free(he as *mut libc::c_void);
                return ret;
            }
            cl = (*cl).next;
            debug -= 1;
            if debug == 0 as libc::c_int {
                printf(b"error in storage2\n\0" as *const u8 as *const libc::c_char);
                exit(0 as libc::c_int);
            }
        }
        ret = append_value_to_he(
            he,
            val,
            type_0,
            replace,
            0 as *mut libc::c_void as *mut mvalue,
        );
        (*he).next = (*hd).list;
        (*hd).list = he;
        (*hd).now = ((*hd).now).wrapping_add(1);
    }
    pthread_spin_unlock(&mut (*hd).lock);
    pthread_spin_lock(&mut (*ht).lock);
    (*ht).now = ((*ht).now).wrapping_add(1);
    pthread_spin_unlock(&mut (*ht).lock);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn htable_insert_list(
    mut ht: *mut htable,
    mut key: *mut uchar,
    mut klen: libc::c_int,
    mut type_0: libc::c_int,
    mut val: *mut uchar,
    mut replace: libc::c_int,
    mut mv: *mut mvalue,
    mut hashd: *mut hashval_t,
) -> libc::c_int {
    let mut idx: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut debug: libc::c_int = 0;
    let mut he: *mut hentry = 0 as *mut hentry;
    let mut cl: *mut hentry = 0 as *mut hentry;
    let mut prev: *mut hentry = 0 as *mut hentry;
    let mut hd: *mut hdata = 0 as *mut hdata;
    let mut dlen: uchar = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: libc::c_int = 0;
    debug = 500 as libc::c_int;
    he = 0 as *mut libc::c_void as *mut hentry;
    cl = 0 as *mut libc::c_void as *mut hentry;
    prev = 0 as *mut libc::c_void as *mut hentry;
    hd = 0 as *mut libc::c_void as *mut hdata;
    dlen = klen as uchar;
    tmp = check_support_type(type_0 as ushort___0);
    if tmp == -(1 as libc::c_int) {
        printf(
            b"invalud type:%d, key:[%s]\n\0" as *const u8 as *const libc::c_char,
            type_0,
            key,
        );
        return -(1 as libc::c_int);
    }
    tmp___0 = malloc(
        (::std::mem::size_of::<hentry>() as libc::c_ulong)
            .wrapping_add(dlen as libc::c_ulong),
    );
    he = tmp___0 as *mut hentry;
    if he as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        printf(b"oom\n\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    memset(
        he as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<hentry>() as libc::c_ulong,
    );
    memcpy(
        ((*he).key).as_mut_ptr() as *mut libc::c_void,
        key as *const libc::c_void,
        dlen as size_t,
    );
    if *hashd == 0 as libc::c_uint {
        *hashd = (Some(((*ht).h).expect("non-null function pointer")))
            .expect("non-null function pointer")(key as *mut libc::c_void, klen);
    }
    idx = (*hashd & (*ht).mask) as libc::c_int;
    hd = ((*ht).table).offset(idx as isize);
    pthread_spin_lock(&mut (*hd).lock);
    if (*hd).list as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        ret = append_value_to_he(
            he,
            val,
            type_0,
            replace,
            0 as *mut libc::c_void as *mut mvalue,
        );
        (*hd).now = 1 as libc::c_int as uint64_t;
        (*hd).list = he;
    } else {
        cl = (*hd).list;
        while cl as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            tmp___1 = (Some(((*ht).c).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                ((*cl).key).as_mut_ptr() as *mut libc::c_void,
                ((*he).key).as_mut_ptr() as *mut libc::c_void,
            );
            if tmp___1 == 0 as libc::c_int {
                ret = append_value_to_he(cl, val, type_0, replace, mv);
                pthread_spin_unlock(&mut (*hd).lock);
                free(he as *mut libc::c_void);
                return ret;
            }
            prev = cl;
            cl = (*cl).next;
            debug -= 1;
            if debug == 0 as libc::c_int {
                printf(b"error in storage3\n\0" as *const u8 as *const libc::c_char);
                exit(0 as libc::c_int);
            }
        }
        ret = append_value_to_he(
            he,
            val,
            type_0,
            replace,
            0 as *mut libc::c_void as *mut mvalue,
        );
        (*prev).next = he;
        (*hd).now = ((*hd).now).wrapping_add(1);
    }
    pthread_spin_unlock(&mut (*hd).lock);
    pthread_spin_lock(&mut (*ht).lock);
    (*ht).now = ((*ht).now).wrapping_add(1);
    pthread_spin_unlock(&mut (*ht).lock);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn find_record_with_ttl(
    mut ht: *mut htable,
    mut key: *mut uchar,
    mut klen: libc::c_int,
    mut type_0: libc::c_int,
    mut val: *mut uchar,
    mut vlen: libc::c_int,
    mut md: *mut mvalue,
    mut hash: *mut hashval_t,
) -> libc::c_int {
    let mut idx: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut oval: *mut uchar = 0 as *mut uchar;
    let mut tmp: uint___0 = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = get_pre_mem_hash(key as *mut libc::c_void, klen, hash);
    idx = tmp as libc::c_int;
    ret = htable_find(ht.offset(idx as isize), key, klen, type_0, val, vlen, md, hash);
    if ret > 0 as libc::c_int {
        tmp___0 = ttl_expired(val);
        if tmp___0 == 1 as libc::c_int {
            oval = htable_delete(ht.offset(idx as isize), key, klen, type_0, *hash);
            if oval as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                free(oval as *mut libc::c_void);
            }
        } else {
            return ret
        }
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn st_th(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut i: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut key: [uchar; 50] = [0; 50];
    let mut tmp: libc::c_uint = 0;
    let mut klen: libc::c_int = 0;
    let mut val: *mut uchar = 0 as *mut uchar;
    let mut pre: libc::c_int = 0;
    let mut oval: *mut uchar = 0 as *mut uchar;
    let mut ht: *mut htable = 0 as *mut htable;
    let mut sh: *mut st_hlp = 0 as *mut st_hlp;
    let mut hash: hashval_t = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: uint___0 = 0;
    let mut tmp___3: size_t = 0;
    let mut tmp___4: uint___0 = 0;
    key[0 as libc::c_int as usize] = 0 as libc::c_int as uchar;
    tmp = 1 as libc::c_uint;
    while !(tmp >= 50 as libc::c_uint) {
        key[tmp as usize] = 0 as libc::c_int as libc::c_uchar;
        tmp = tmp.wrapping_add(1);
    }
    val = 0 as *mut libc::c_void as *mut uchar;
    pre = 0 as libc::c_int;
    sh = arg as *mut st_hlp;
    idx = (*sh).idx;
    ht = (*sh).ht;
    i = idx * 10000 as libc::c_int;
    while i < (idx + 1 as libc::c_int) * 10000 as libc::c_int {
        hash = 0 as libc::c_int as hashval_t;
        sprintf(
            key.as_mut_ptr() as *mut libc::c_char,
            b"%dkey\0" as *const u8 as *const libc::c_char,
            i,
        );
        tmp___0 = malloc(50 as libc::c_int as size_t);
        val = tmp___0 as *mut uchar;
        sprintf(
            val as *mut libc::c_char,
            b"%dval\0" as *const u8 as *const libc::c_char,
            i,
        );
        tmp___1 = strlen(key.as_mut_ptr() as *const libc::c_char);
        klen = tmp___1.wrapping_add(1 as libc::c_ulong) as libc::c_int;
        tmp___2 = get_pre_mem_hash(
            key.as_mut_ptr() as *mut libc::c_void,
            klen,
            &mut hash,
        );
        pre = tmp___2 as libc::c_int;
        htable_insert(
            ht.offset(pre as isize),
            key.as_mut_ptr(),
            klen,
            1 as libc::c_int,
            val,
            0 as libc::c_int,
            0 as *mut libc::c_void as *mut mvalue,
            &mut hash,
        );
        i += 1;
    }
    if idx == 4 as libc::c_int {
        idx = -(1 as libc::c_int);
    }
    sleep(2 as libc::c_uint);
    i = (idx + 1 as libc::c_int) * 10000 as libc::c_int;
    while i < (idx + 2 as libc::c_int) * 10000 as libc::c_int {
        hash = 0 as libc::c_int as hashval_t;
        sprintf(
            key.as_mut_ptr() as *mut libc::c_char,
            b"%dkey\0" as *const u8 as *const libc::c_char,
            i,
        );
        tmp___3 = strlen(key.as_mut_ptr() as *const libc::c_char);
        klen = tmp___3.wrapping_add(1 as libc::c_ulong) as libc::c_int;
        tmp___4 = get_pre_mem_hash(
            key.as_mut_ptr() as *mut libc::c_void,
            klen,
            &mut hash,
        );
        pre = tmp___4 as libc::c_int;
        oval = htable_delete(
            ht.offset(pre as isize),
            key.as_mut_ptr(),
            klen,
            1 as libc::c_int,
            hash,
        );
        if oval as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            printf(
                b"error in test %s,%d,%d\n\0" as *const u8 as *const libc::c_char,
                key.as_mut_ptr(),
                idx,
                i,
            );
        } else {
            free(oval as *mut libc::c_void);
        }
        i += 1;
    }
    sleep(5 as libc::c_uint);
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn storage_test() -> libc::c_int {
    let mut ht: *mut htable = 0 as *mut htable;
    let mut pt: [pthread_t; 5] = [0; 5];
    let mut i: libc::c_int = 0;
    let mut sh: [st_hlp; 5] = [st_hlp {
        ht: 0 as *mut htable,
        idx: 0,
    }; 5];
    let mut tmp: libc::c_int = 0;
    ht = 0 as *mut libc::c_void as *mut htable;
    if ht as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        dns_error(
            0 as libc::c_int,
            b"create htable error\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        sh[i as usize].ht = ht;
        sh[i as usize].idx = i;
        tmp = pthread_create(
            pt.as_mut_ptr().offset(i as isize),
            0 as *mut libc::c_void as *const pthread_attr_t,
            Some(st_th as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
            sh.as_mut_ptr().offset(i as isize) as *mut libc::c_void,
        );
        if tmp != 0 {
            dns_error(
                0 as libc::c_int,
                b"create pthread\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        pthread_join(pt[i as usize], 0 as *mut libc::c_void as *mut *mut libc::c_void);
        i += 1;
    }
    sleep(2 as libc::c_uint);
    return 0 as libc::c_int;
}
pub static mut support_type: [rrtype; 9] = [A, NS, CNAME, SOA, MX, TXT, AAAA, SRV, PTR];
pub unsafe extern "C" fn str_to_len_label(
    mut domain: *mut uchar,
    mut len: libc::c_int,
) -> *mut uchar {
    let mut l: uchar = 0;
    let mut i: libc::c_int = 0;
    l = 0 as libc::c_int as uchar;
    if *domain.offset((len - 1 as libc::c_int) as isize) as libc::c_int
        != 0 as libc::c_int
    {
        return 0 as *mut libc::c_void as *mut uchar
    } else {
        if *domain.offset((len - 2 as libc::c_int) as isize) as libc::c_int
            != 46 as libc::c_int
        {
            return 0 as *mut libc::c_void as *mut uchar;
        }
    }
    i = len - 2 as libc::c_int;
    while i > 0 as libc::c_int {
        *domain.offset(i as isize) = *domain.offset((i - 1 as libc::c_int) as isize);
        l = (l as libc::c_int + 1 as libc::c_int) as uchar;
        if *domain.offset(i as isize) as libc::c_int == 46 as libc::c_int {
            *domain.offset(i as isize) = (l as libc::c_int - 1 as libc::c_int) as uchar;
            l = 0 as libc::c_int as uchar;
        }
        i -= 1;
    }
    *domain.offset(0 as libc::c_int as isize) = l;
    return domain;
}
pub static mut SupportTypeTable: [libc::c_uchar; 256] = [
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    33 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
pub unsafe extern "C" fn check_support_type(mut type_0: ushort___0) -> libc::c_int {
    if type_0 as libc::c_int > 255 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if SupportTypeTable[type_0 as libc::c_uchar as usize] as libc::c_int
        != 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn passer_dns_data(mut mbuf: *mut mbuf_type) {
    let mut buf: *mut uchar = 0 as *mut uchar;
    let mut num: libc::c_int = 0;
    let mut dlen: libc::c_int = 0;
    let mut tail: *mut uchar = 0 as *mut uchar;
    let mut hdr: *mut dnsheader = 0 as *mut dnsheader;
    let mut tmp: uint16_t = 0;
    let mut tmp___0: uint16_t = 0;
    let mut tmp___1: uint16_t = 0;
    let mut tmp___2: uint16_t = 0;
    let mut tmp___3: *mut uchar = 0 as *mut uchar;
    let mut tmp___4: uint16_t = 0;
    let mut tmp___5: libc::c_int = 0;
    buf = (*mbuf).buf;
    dlen = 0 as libc::c_int;
    tail = 0 as *mut libc::c_void as *mut uchar;
    hdr = buf as *mut dnsheader;
    (*mbuf).err = 1 as libc::c_int;
    tmp = ntohs((*hdr).qdcount);
    num = tmp as libc::c_int;
    if num != 1 as libc::c_int {
        return;
    }
    tmp___0 = ntohs((*hdr).ancount);
    num = tmp___0 as libc::c_int;
    if num != 0 as libc::c_int {
        return;
    }
    tmp___1 = ntohs((*hdr).nscount);
    num = tmp___1 as libc::c_int;
    if num != 0 as libc::c_int {
        return;
    }
    tmp___2 = ntohs((*hdr).arcount);
    num = tmp___2 as libc::c_int;
    if num > 1 as libc::c_int {
        return;
    }
    (*mbuf).id = (*hdr).id;
    dlen = check_dns_name(
        buf.offset(::std::mem::size_of::<dnsheader>() as libc::c_ulong as isize),
        &mut (*mbuf).lowerdomain,
    );
    if dlen < 0 as libc::c_int {
        return;
    }
    (*mbuf).dlen = dlen;
    tmp___3 = buf.offset(::std::mem::size_of::<dnsheader>() as libc::c_ulong as isize);
    (*mbuf).origindomain = tmp___3;
    tail = tmp___3;
    if dlen == 2 as libc::c_int {
        tail = tail.offset(1);
    } else {
        tail = tail.offset(dlen as isize);
    }
    tmp___4 = ntohs(*(tail as *mut ushort___0));
    (*mbuf).qtype = tmp___4 as rrtype;
    tmp___5 = check_support_type((*mbuf).qtype as ushort___0);
    if tmp___5 == 0 as libc::c_int {
        (*mbuf).err = 0 as libc::c_int;
    }
}
pub unsafe extern "C" fn send_tc_to_client(mut mbuf: *mut mbuf_type) -> libc::c_int {
    let mut itor: *mut uchar = 0 as *mut uchar;
    let mut hdr: *mut dnsheader = 0 as *mut dnsheader;
    let mut qd: *mut qdns = 0 as *mut qdns;
    let mut tmp: uint16_t = 0;
    let mut tmp___0: uint16_t = 0;
    itor = (*mbuf).buf;
    hdr = itor as *mut dnsheader;
    qd = 0 as *mut libc::c_void as *mut qdns;
    if (*mbuf).td as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    (*hdr).id = (*mbuf).id;
    (*hdr).flags = 0 as libc::c_int as uint16_t;
    (*hdr).flags = ((*hdr).flags as libc::c_int | 32768 as libc::c_int) as uint16_t;
    (*hdr).flags = ((*hdr).flags as libc::c_int | 128 as libc::c_int) as uint16_t;
    (*hdr).flags = ((*hdr).flags as libc::c_int | 512 as libc::c_int) as uint16_t;
    (*hdr).flags = htons((*hdr).flags);
    (*hdr).qdcount = htons(1 as libc::c_int as uint16_t);
    tmp___0 = htons(0 as libc::c_int as uint16_t);
    (*hdr).arcount = tmp___0;
    tmp = tmp___0;
    (*hdr).nscount = tmp;
    (*hdr).ancount = tmp;
    itor = itor.offset(::std::mem::size_of::<dnsheader>() as libc::c_ulong as isize);
    memcpy(
        itor as *mut libc::c_void,
        (*mbuf).td as *const libc::c_void,
        (*mbuf).dlen as size_t,
    );
    itor = itor.offset((*mbuf).dlen as isize);
    qd = itor as *mut qdns;
    (*qd).type_0 = htons((*mbuf).qtype as uint16_t);
    (*qd).dclass = htons(1 as libc::c_int as uint16_t);
    itor = itor.offset(::std::mem::size_of::<qdns>() as libc::c_ulong as isize);
    (*mbuf).buflen = itor.offset_from((*mbuf).buf) as libc::c_long as libc::c_int;
    udp_write_info(mbuf, 0 as libc::c_int);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn get_domain_from_msg(
    mut itor: *mut uchar,
    mut hdr: *mut uchar,
    mut to: *mut uchar,
    mut tmplen: *mut libc::c_int,
) -> libc::c_int {
    let mut len: uchar = 0;
    let mut offset: ushort___0 = 0;
    let mut dlen: libc::c_int = 0;
    let mut hasptr: libc::c_int = 0;
    let mut infinite: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    offset = 0 as libc::c_int as ushort___0;
    len = *itor.offset(0 as libc::c_int as isize);
    dlen = 0 as libc::c_int;
    hasptr = 0 as libc::c_int;
    infinite = 20 as libc::c_int;
    offset = ntohs(*(itor as *mut ushort___0));
    *tmplen = 0 as libc::c_int;
    while len as libc::c_int != 0 as libc::c_int {
        tmp = infinite;
        infinite -= 1;
        if tmp == 0 {
            break;
        }
        if offset as libc::c_int >= 49152 as libc::c_int {
            if offset as libc::c_int <= 53247 as libc::c_int {
                itor = hdr
                    .offset((offset as libc::c_int & 16383 as libc::c_int) as isize);
                if hasptr == 0 as libc::c_int {
                    dlen = 2 as libc::c_int;
                    if *tmplen != 0 as libc::c_int {
                        dlen += *tmplen;
                    }
                }
                hasptr = 1 as libc::c_int;
                offset = ntohs(*(itor as *mut ushort___0));
                continue;
            }
        }
        *to.offset(0 as libc::c_int as isize) = *itor.offset(0 as libc::c_int as isize);
        *tmplen += 1;
        *tmplen += *to.offset(0 as libc::c_int as isize) as libc::c_int;
        if *to.offset(0 as libc::c_int as isize) as libc::c_int > 64 as libc::c_int {
            return -(1 as libc::c_int);
        }
        to = to.offset(1);
        memcpy(
            to as *mut libc::c_void,
            itor.offset(1 as libc::c_int as isize) as *const libc::c_void,
            *itor.offset(0 as libc::c_int as isize) as size_t,
        );
        to = to.offset(*itor.offset(0 as libc::c_int as isize) as libc::c_int as isize);
        itor = itor
            .offset(*itor.offset(0 as libc::c_int as isize) as libc::c_int as isize)
            .offset(1 as libc::c_int as isize);
        len = *itor.offset(0 as libc::c_int as isize);
        offset = ntohs(*(itor as *mut ushort___0));
    }
    if infinite <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    *to.offset(0 as libc::c_int as isize) = 0 as libc::c_int as uchar;
    to = to.offset(1);
    *tmplen += 1;
    if dlen == 0 as libc::c_int {
        dlen = *tmplen;
    }
    if dlen > 255 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return dlen;
}
pub unsafe extern "C" fn insert_into_ttltree(
    mut rbt: *mut rbtree,
    mut td: *mut uchar,
    mut len: libc::c_int,
    mut type_0: libc::c_int,
    mut ttl: uint___0,
    mut lowerdomain: *mut packet_type,
) -> libc::c_int {
    let mut node: rbnode = rbnode {
        parent: 0 as *mut rbnode,
        left: 0 as *mut rbnode,
        right: 0 as *mut rbnode,
        color: 0,
        key: 0 as *mut libc::c_void,
    };
    let mut tn: *mut ttlnode = 0 as *mut ttlnode;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut packet_type = 0 as *mut packet_type;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    node.parent = 0 as *mut rbnode;
    node.left = 0 as *mut rbnode;
    node.right = 0 as *mut rbnode;
    node.color = 0 as libc::c_int;
    node.key = 0 as *mut libc::c_void;
    tn = 0 as *mut libc::c_void as *mut ttlnode;
    tmp = malloc(::std::mem::size_of::<ttlnode>() as libc::c_ulong);
    tn = tmp as *mut ttlnode;
    if tn as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    tmp___1 = malloc(::std::mem::size_of::<packet_type>() as libc::c_ulong);
    tmp___0 = tmp___1 as *mut packet_type;
    (*tn).lowerdomain = tmp___0;
    if tmp___0 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        free(tn as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    (*tn).dlen = len as ushort___0;
    (*tn).exp = ttl;
    (*tn).type_0 = type_0 as ushort___0;
    (*tn)
        .hash = &mut *((*(*tn).lowerdomain).hash)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut hashval_t;
    memcpy(
        (*tn).lowerdomain as *mut libc::c_void,
        lowerdomain as *const libc::c_void,
        ::std::mem::size_of::<packet_type>() as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while i < (*(*tn).lowerdomain).label_count as libc::c_int {
        (*(*tn).lowerdomain)
            .label[i
            as usize] = ((*(*tn).lowerdomain).domain)
            .as_mut_ptr()
            .offset(
                (*(*tn).lowerdomain).label_offsets[i as usize] as libc::c_int as isize,
            );
        i += 1;
    }
    (*tn).data = ((*(*tn).lowerdomain).domain).as_mut_ptr();
    node.key = tn as *mut libc::c_void;
    insert_node(rbt, &mut node);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn random_ttl(mut ttl: uint___0) -> uint___0 {
    let mut ret: uint___0 = 0;
    ret = ttl.wrapping_rem(7 as libc::c_uint);
    ttl = (ttl as libc::c_uint).wrapping_add(ret.wrapping_mul(3 as libc::c_uint))
        as uint___0 as uint___0;
    if ttl > 604800 as libc::c_uint {
        ttl = (604800 as libc::c_uint)
            .wrapping_sub(ttl.wrapping_rem(604800 as libc::c_uint));
    }
    return ttl;
}
pub unsafe extern "C" fn is_parent(
    mut parent: *mut uchar,
    mut son: *mut uchar,
) -> libc::c_int {
    let mut sp: libc::c_int = 0;
    let mut ss: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: libc::c_int = 0;
    tmp = strlen(parent as *const libc::c_char);
    sp = tmp as libc::c_int;
    tmp___0 = strlen(son as *const libc::c_char);
    ss = tmp___0 as libc::c_int;
    if ss < sp {
        return -(1 as libc::c_int);
    }
    x = ss - sp;
    son = son.offset(x as isize);
    tmp___1 = strcmp(parent as *const libc::c_char, son as *const libc::c_char);
    if tmp___1 == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn check_dms(
    mut ck: *mut uchar,
    mut dms: *mut uchar,
    mut num: libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn process_rdata(
    mut hlp: *mut hlpp,
    mut label: *mut uchar,
    mut n: libc::c_int,
) -> *mut uchar {
    let mut buffer: *mut uchar = 0 as *mut uchar;
    let mut type_0: ushort___0 = 0;
    let mut classin: ushort___0 = 0;
    let mut lth: ushort___0 = 0;
    let mut tmptype: ushort___0 = 0;
    let mut ttl: uint___0 = 0;
    let mut tmpttl: uint___0 = 0;
    let mut tx: uint___0 = 0;
    let mut i: libc::c_int = 0;
    let mut dlen: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut tmplen: libc::c_int = 0;
    let mut stype: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ds: *mut htable = 0 as *mut htable;
    let mut rbt: *mut rbtree = 0 as *mut rbtree;
    let mut hdr: *mut uchar = 0 as *mut uchar;
    let mut mlen: libc::c_int = 0;
    let mut mv: *mut mvalue = 0 as *mut mvalue;
    let mut tmpdomain: *mut uchar = 0 as *mut uchar;
    let mut dm: *mut uchar = 0 as *mut uchar;
    let mut itor: *mut uchar = 0 as *mut uchar;
    let mut lowerdomain: packet_type = packet_type {
        label_count: 0,
        domain: [0; 256],
        label: [0 as *mut uint8_t; 64],
        label_offsets: [0; 64],
        label_len: [0; 64],
        hash: [0; 64],
    };
    let mut tmp: uchar = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: uint___0 = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: uint___0 = 0;
    buffer = (*hlp).tmpbuf;
    type_0 = 0 as libc::c_int as ushort___0;
    tmptype = 0 as libc::c_int as ushort___0;
    ttl = 0 as libc::c_int as uint___0;
    tmpttl = 0 as libc::c_int as uint___0;
    tmplen = 0 as libc::c_int;
    stype = (*hlp).stype;
    ds = (*hlp).ds;
    rbt = (*hlp).rbt;
    hdr = (*hlp).buf;
    mlen = (*hlp).datalen;
    mv = buffer as *mut mvalue;
    tmpdomain = (*hlp).domainbuf;
    itor = 0 as *mut libc::c_void as *mut uchar;
    dm = (lowerdomain.domain).as_mut_ptr();
    memset(
        mv as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mvalue>() as libc::c_ulong,
    );
    itor = buffer.offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize);
    tx = global_now as uint___0;
    tmp = 0 as libc::c_int as uchar;
    *dm.offset(1 as libc::c_int as isize) = tmp;
    *dm.offset(0 as libc::c_int as isize) = tmp;
    i = 0 as libc::c_int;
    while i < n {
        dlen = get_domain_from_msg(label, hdr, tmpdomain, &mut tmplen);
        if *dm.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
            if *dm.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
                check_dns_name(tmpdomain, &mut lowerdomain);
            }
        }
        if dlen < 0 as libc::c_int {
            return 0 as *mut libc::c_void as *mut uchar;
        }
        label = label.offset(dlen as isize);
        tmp___0 = get_dns_info(label, &mut tmptype, &mut classin, &mut ttl, &mut lth);
        if tmp___0 < 0 as libc::c_int {
            return 0 as *mut libc::c_void as *mut uchar;
        }
        if ttl < 10 as libc::c_uint {
            ttl = 10 as libc::c_int as uint___0;
        }
        ttl = random_ttl(ttl.wrapping_add(n as uint___0));
        label = label.offset(10 as libc::c_int as isize);
        let mut current_block_43: u64;
        if tmptype as libc::c_int == 6 as libc::c_int {
            current_block_43 = 1370985878501326100;
        } else if tmptype as libc::c_int == 5 as libc::c_int {
            current_block_43 = 1370985878501326100;
        } else {
            current_block_43 = 1847472278776910194;
        }
        match current_block_43 {
            1370985878501326100 => {
                if i == n - 1 as libc::c_int {
                    *stype = tmptype as libc::c_int;
                }
            }
            _ => {}
        }
        if type_0 as libc::c_int == 0 as libc::c_int {
            type_0 = tmptype;
        }
        if ttl > 604800 as libc::c_uint {
            ttl = 604800 as libc::c_int as uint___0;
        }
        if tmpttl == 0 as libc::c_uint {
            tmpttl = ttl;
        }
        tmp___2 = dict_comp_str_equ(
            tmpdomain as *mut libc::c_void,
            dm as *mut libc::c_void,
        );
        let mut current_block_63: u64;
        if tmp___2 != 0 as libc::c_int {
            current_block_63 = 14848998955190965439;
        } else if type_0 as libc::c_int != tmptype as libc::c_int {
            current_block_63 = 14848998955190965439;
        } else {
            current_block_63 = 16203797167131938757;
        }
        match current_block_63 {
            14848998955190965439 => {
                tmp___1 = random_ttl(
                    tmpttl
                        .wrapping_add(i as uint___0)
                        .wrapping_add(tx.wrapping_rem(5 as libc::c_uint)),
                );
                (*mv).ttl = tmp___1.wrapping_add(tx);
                if *dm
                    .offset(
                        (*dm.offset(0 as libc::c_int as isize) as libc::c_int
                            + 2 as libc::c_int) as isize,
                    ) as libc::c_int != 0 as libc::c_int
                {
                    insert_kv_mem(
                        rbt,
                        ds,
                        dm,
                        lowerdomain.label_len[0 as libc::c_int as usize] as libc::c_int,
                        type_0 as libc::c_int,
                        buffer,
                        ((*mv).len as libc::c_ulong)
                            .wrapping_add(
                                ::std::mem::size_of::<mvalue>() as libc::c_ulong,
                            ) as libc::c_int,
                        0 as libc::c_int,
                        &mut lowerdomain,
                    );
                }
                type_0 = tmptype;
                check_dns_name(tmpdomain, &mut lowerdomain);
                memset(
                    mv as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<mvalue>() as libc::c_ulong,
                );
                itor = buffer
                    .offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize);
            }
            _ => {}
        }
        ret = fill_rrset_in_buffer(
            itor,
            label,
            hdr,
            lth as libc::c_int,
            type_0 as libc::c_int,
            hlp,
        );
        if ret > 0 as libc::c_int {
            itor = itor.offset(ret as isize);
            (*mv).len = ((*mv).len as libc::c_int + ret) as uint16_t;
            (*mv).num = ((*mv).num as libc::c_int + 1 as libc::c_int) as uint16_t;
        }
        tmpttl = ttl;
        label = label.offset(lth as libc::c_int as isize);
        if (label as libc::c_ulong) < hdr as libc::c_ulong {
            return 0 as *mut libc::c_void as *mut uchar
        } else {
            if label as libc::c_ulong > hdr.offset(mlen as isize) as libc::c_ulong {
                return 0 as *mut libc::c_void as *mut uchar;
            }
        }
        i += 1;
    }
    if (*mv).num as libc::c_int > 0 as libc::c_int {
        tmp___3 = random_ttl(
            tmpttl
                .wrapping_add(i as uint___0)
                .wrapping_add(tx.wrapping_rem(5 as libc::c_uint)),
        );
        (*mv).ttl = tmp___3.wrapping_add(tx);
        (*mv).hits = 0 as libc::c_int as uint32_t;
        (*mv).seg = 0 as libc::c_int as uint16_t;
        if *dm
            .offset(
                (*dm.offset(0 as libc::c_int as isize) as libc::c_int + 2 as libc::c_int)
                    as isize,
            ) as libc::c_int != 0 as libc::c_int
        {
            insert_kv_mem(
                rbt,
                ds,
                dm,
                lowerdomain.label_len[0 as libc::c_int as usize] as libc::c_int,
                type_0 as libc::c_int,
                buffer,
                ((*mv).len as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<mvalue>() as libc::c_ulong)
                    as libc::c_int,
                0 as libc::c_int,
                &mut lowerdomain,
            );
        }
    }
    return label;
}
pub unsafe extern "C" fn check_domain_mask(
    mut domain: *mut uchar,
    mut origin: *mut uchar,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = strncmp(
        origin as *const libc::c_char,
        domain as *const libc::c_char,
        len as size_t,
    );
    return tmp;
}
pub unsafe extern "C" fn get_dns_info(
    mut label: *mut uchar,
    mut tp: *mut ushort___0,
    mut cls: *mut ushort___0,
    mut ttl: *mut uint___0,
    mut lth: *mut ushort___0,
) -> libc::c_int {
    let mut us: *mut ushort___0 = 0 as *mut ushort___0;
    let mut ui: *mut uint___0 = 0 as *mut uint___0;
    us = 0 as *mut libc::c_void as *mut ushort___0;
    ui = 0 as *mut libc::c_void as *mut uint___0;
    us = label as *mut ushort___0;
    *tp = ntohs(*us);
    if *tp as libc::c_int > 254 as libc::c_int {
        printf(
            b"type is %u\n\0" as *const u8 as *const libc::c_char,
            *tp as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    label = label.offset(::std::mem::size_of::<ushort___0>() as libc::c_ulong as isize);
    us = label as *mut ushort___0;
    *cls = ntohs(*us);
    if *cls as libc::c_int != 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    label = label.offset(::std::mem::size_of::<ushort___0>() as libc::c_ulong as isize);
    ui = label as *mut uint___0;
    *ttl = ntohl(*ui);
    label = label.offset(::std::mem::size_of::<uint___0>() as libc::c_ulong as isize);
    us = label as *mut ushort___0;
    *lth = ntohs(*us);
    return 0 as libc::c_int;
}
pub static mut DnsNameTable: [libc::c_uchar; 256] = [
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    42 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    48 as libc::c_int as libc::c_uchar,
    49 as libc::c_int as libc::c_uchar,
    50 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    52 as libc::c_int as libc::c_uchar,
    53 as libc::c_int as libc::c_uchar,
    54 as libc::c_int as libc::c_uchar,
    55 as libc::c_int as libc::c_uchar,
    56 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    97 as libc::c_int as libc::c_uchar,
    98 as libc::c_int as libc::c_uchar,
    99 as libc::c_int as libc::c_uchar,
    100 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    102 as libc::c_int as libc::c_uchar,
    103 as libc::c_int as libc::c_uchar,
    104 as libc::c_int as libc::c_uchar,
    105 as libc::c_int as libc::c_uchar,
    106 as libc::c_int as libc::c_uchar,
    107 as libc::c_int as libc::c_uchar,
    108 as libc::c_int as libc::c_uchar,
    109 as libc::c_int as libc::c_uchar,
    110 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    112 as libc::c_int as libc::c_uchar,
    113 as libc::c_int as libc::c_uchar,
    114 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    117 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    119 as libc::c_int as libc::c_uchar,
    120 as libc::c_int as libc::c_uchar,
    121 as libc::c_int as libc::c_uchar,
    122 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    95 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    97 as libc::c_int as libc::c_uchar,
    98 as libc::c_int as libc::c_uchar,
    99 as libc::c_int as libc::c_uchar,
    100 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    102 as libc::c_int as libc::c_uchar,
    103 as libc::c_int as libc::c_uchar,
    104 as libc::c_int as libc::c_uchar,
    105 as libc::c_int as libc::c_uchar,
    106 as libc::c_int as libc::c_uchar,
    107 as libc::c_int as libc::c_uchar,
    108 as libc::c_int as libc::c_uchar,
    109 as libc::c_int as libc::c_uchar,
    110 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    112 as libc::c_int as libc::c_uchar,
    113 as libc::c_int as libc::c_uchar,
    114 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    117 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    119 as libc::c_int as libc::c_uchar,
    120 as libc::c_int as libc::c_uchar,
    121 as libc::c_int as libc::c_uchar,
    122 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
pub static mut InvalidDnsNameTable: [libc::c_uchar; 256] = [
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
];
pub unsafe extern "C" fn check_dns_name(
    mut domain: *mut uchar,
    mut lowerdomain: *mut packet_type,
) -> libc::c_int {
    let mut len: uchar = 0;
    let mut i: uchar = 0;
    let mut tlen: libc::c_int = 0;
    let mut dst: *mut uchar = 0 as *mut uchar;
    let mut hash: *mut hashval_t = 0 as *mut hashval_t;
    let mut tmp: *mut uchar = 0 as *mut uchar;
    let mut tmp___0: *mut uchar = 0 as *mut uchar;
    len = *domain.offset(0 as libc::c_int as isize);
    tlen = 0 as libc::c_int;
    dst = ((*lowerdomain).domain).as_mut_ptr();
    hash = &mut *((*lowerdomain).hash).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut hashval_t;
    (*lowerdomain).label_count = 0 as libc::c_int as uint8_t;
    (*lowerdomain).label[(*lowerdomain).label_count as usize] = dst;
    (*lowerdomain)
        .label_offsets[(*lowerdomain).label_count
        as usize] = 0 as libc::c_int as uint8_t;
    (*lowerdomain).hash[0 as libc::c_int as usize] = 5381 as libc::c_int as hashval_t;
    *dst = len;
    tmp = dst;
    dst = dst.offset(1);
    *hash = (*hash << 5 as libc::c_int)
        .wrapping_add(*hash)
        .wrapping_add(*tmp as hashval_t);
    domain = domain.offset(1);
    if len as libc::c_int == 0 as libc::c_int {
        *dst = '\u{0}' as i32 as uchar;
        tlen = 2 as libc::c_int;
        return tlen;
    }
    while len as libc::c_int != 0 as libc::c_int {
        if len as libc::c_int > 63 as libc::c_int {
            return -(1 as libc::c_int);
        }
        i = 0 as libc::c_int as uchar;
        while (i as libc::c_int) < len as libc::c_int {
            *dst = DnsNameTable[*domain.offset(i as libc::c_int as isize) as usize];
            if *dst == 0 {
                return -(1 as libc::c_int);
            }
            *hash = (*hash << 5 as libc::c_int)
                .wrapping_add(*hash)
                .wrapping_add(*dst as hashval_t);
            dst = dst.offset(1);
            i = (i as libc::c_int + 1 as libc::c_int) as uchar;
        }
        domain = domain.offset(len as libc::c_int as isize);
        len = *domain.offset(0 as libc::c_int as isize);
        (*lowerdomain)
            .label_count = ((*lowerdomain).label_count as libc::c_int + 1 as libc::c_int)
            as uint8_t;
        (*lowerdomain).label[(*lowerdomain).label_count as usize] = dst;
        (*lowerdomain)
            .label_offsets[(*lowerdomain).label_count
            as usize] = dst.offset_from(((*lowerdomain).domain).as_mut_ptr())
            as libc::c_long as uint8_t;
        (*lowerdomain)
            .hash[(*lowerdomain).label_count as usize] = 0 as libc::c_int as hashval_t;
        *dst = len;
        tmp___0 = dst;
        dst = dst.offset(1);
        *hash = (*hash << 5 as libc::c_int)
            .wrapping_add(*hash)
            .wrapping_add(*tmp___0 as hashval_t);
        domain = domain.offset(1);
    }
    i = 0 as libc::c_int as uchar;
    while (i as libc::c_int) < (*lowerdomain).label_count as libc::c_int {
        (*lowerdomain)
            .label_len[i
            as usize] = dst.offset_from((*lowerdomain).label[i as usize]) as libc::c_long
            as uint8_t;
        i = (i as libc::c_int + 1 as libc::c_int) as uchar;
    }
    tlen = (*lowerdomain).label_len[0 as libc::c_int as usize] as libc::c_int;
    if tlen > 255 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return tlen;
}
pub unsafe extern "C" fn make_type_domain(
    mut domain: *mut uchar,
    mut dlen: libc::c_int,
    mut type_0: libc::c_int,
    mut buffer: *mut uchar,
) -> libc::c_int {
    if buffer as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int)
    } else {
        if domain as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            return -(1 as libc::c_int);
        }
    }
    *buffer.offset(0 as libc::c_int as isize) = type_0 as uchar;
    memcpy(
        buffer.offset(1 as libc::c_int as isize) as *mut libc::c_void,
        domain as *const libc::c_void,
        dlen as size_t,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn check_memcpy(
    mut to: *mut uchar,
    mut from: *mut uchar,
    mut vlen: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < vlen {
        if *to.offset(i as isize) as libc::c_int
            != *from.offset(i as isize) as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn insert_kv_mem(
    mut rbt: *mut rbtree,
    mut ds: *mut htable,
    mut k: *mut uchar,
    mut klen: libc::c_int,
    mut type_0: libc::c_int,
    mut v: *mut uchar,
    mut vlen: libc::c_int,
    mut hijack___0: libc::c_int,
    mut lowerdomain: *mut packet_type,
) -> libc::c_int {
    let mut val: *mut uchar = 0 as *mut uchar;
    let mut mv: *mut mvalue = 0 as *mut mvalue;
    let mut tmp: mvalue = mvalue {
        len: 0,
        num: 0,
        ttl: 0,
        hits: 0,
        seg: 0,
    };
    let mut ret: libc::c_int = 0;
    let mut pn: *mut rbnode = 0 as *mut rbnode;
    let mut tn: ttlnode = ttlnode {
        exp: 0,
        dlen: 0,
        type_0: 0,
        hash: 0 as *mut hashval_t,
        lowerdomain: 0 as *mut packet_type,
        data: 0 as *mut uchar,
    };
    let mut tmp_tn: *mut ttlnode = 0 as *mut ttlnode;
    let mut idx: libc::c_int = 0;
    let mut hash: *mut hashval_t = 0 as *mut hashval_t;
    let mut tmp___0: uint___0 = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    val = 0 as *mut libc::c_void as *mut uchar;
    mv = 0 as *mut libc::c_void as *mut mvalue;
    tmp.len = 0 as libc::c_int as uint16_t;
    tmp.num = 0 as libc::c_int as libc::c_ushort;
    tmp.ttl = 0 as libc::c_uint;
    tmp.hits = 0 as libc::c_uint;
    tmp.seg = 0 as libc::c_int as libc::c_ushort;
    ret = -(1 as libc::c_int);
    pn = 0 as *mut libc::c_void as *mut rbnode;
    tn.exp = 0 as libc::c_int as uint___0;
    tn.dlen = 0 as libc::c_int as libc::c_ushort;
    tn.type_0 = 0 as libc::c_int as libc::c_ushort;
    tn.hash = 0 as *mut hashval_t;
    tn.lowerdomain = 0 as *mut packet_type;
    tn.data = 0 as *mut uchar;
    tmp_tn = 0 as *mut libc::c_void as *mut ttlnode;
    if vlen < 0 as libc::c_int {
        return -(1 as libc::c_int)
    } else {
        if vlen > 4096 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    hash = &mut *((*lowerdomain).hash).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut hashval_t;
    tmp___0 = get_pre_mem_hash(k as *mut libc::c_void, klen, hash);
    idx = tmp___0 as libc::c_int;
    tmp___1 = malloc(vlen as size_t);
    val = tmp___1 as *mut uchar;
    if val as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    memcpy(val as *mut libc::c_void, v as *const libc::c_void, vlen as size_t);
    mv = v as *mut mvalue;
    ret = htable_insert(
        ds.offset(idx as isize),
        k,
        klen,
        type_0,
        val,
        1 as libc::c_int,
        &mut tmp,
        hash,
    );
    if ret >= 2 as libc::c_int {
        free(val as *mut libc::c_void);
    }
    if !rbt.is_null() {
        if ret == 1 as libc::c_int {
            pthread_spin_lock(&mut (*rbt).lock);
            tn.dlen = klen as ushort___0;
            tn.exp = tmp.ttl;
            tn.type_0 = type_0 as ushort___0;
            tn.lowerdomain = 0 as *mut libc::c_void as *mut packet_type;
            tn.data = k;
            pn = find_node(rbt, &mut tn as *mut ttlnode as *mut libc::c_void);
            if pn as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                tmp___2 = delete_node(rbt, pn);
                tmp_tn = tmp___2 as *mut ttlnode;
                if !tmp_tn.is_null() {
                    free((*tmp_tn).lowerdomain as *mut libc::c_void);
                    free(tmp_tn as *mut libc::c_void);
                }
            }
            pthread_spin_unlock(&mut (*rbt).lock);
        }
    }
    if (*mv).ttl == 604801 as libc::c_uint {
        return 0 as libc::c_int;
    }
    if rbt as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as libc::c_int;
    }
    pthread_spin_lock(&mut (*rbt).lock);
    if type_0 != 2 as libc::c_int {
        if ret == 1 as libc::c_int {
            ret = insert_into_ttltree(rbt, k, klen, type_0, (*mv).ttl, lowerdomain);
        } else {
            ret = insert_into_ttltree(rbt, k, klen, type_0, tmp.ttl, lowerdomain);
        }
    } else {
        ret = insert_into_ttltree(rbt, k, klen, type_0, tmp.ttl, lowerdomain);
    }
    pthread_spin_unlock(&mut (*rbt).lock);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn get_level(mut itor: *mut uchar) -> libc::c_int {
    let mut lvl: libc::c_int = 0;
    let mut len: uchar = 0;
    lvl = 0 as libc::c_int;
    len = *itor.offset(0 as libc::c_int as isize);
    while len as libc::c_int != 0 as libc::c_int {
        lvl += 1;
        itor = itor
            .offset(
                (*itor.offset(0 as libc::c_int as isize) as libc::c_int
                    + 1 as libc::c_int) as isize,
            );
        len = *itor.offset(0 as libc::c_int as isize);
        if len as libc::c_int > 63 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    return lvl;
}
pub unsafe extern "C" fn fill_all_records_in_msg(
    mut h: *mut hlpc,
    mut hf: *mut hlpf,
    mut pidx: *mut libc::c_int,
) -> *mut uchar {
    let mut step: libc::c_int = 0;
    let mut txtlen: uint16_t = 0;
    let mut tmp: *mut uchar = 0 as *mut uchar;
    let mut to: *mut uchar = 0 as *mut uchar;
    let mut from: *mut uchar = 0 as *mut uchar;
    let mut fm: *mut fillmsg = 0 as *mut fillmsg;
    let mut idx: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    step = 0 as libc::c_int;
    tmp = 0 as *mut libc::c_void as *mut uchar;
    to = (*hf).to;
    from = (*hf).from;
    fm = (*hf).to as *mut fillmsg;
    idx = *pidx;
    (*fm).type_0 = htons((*hf).type_0);
    (*fm).dclass = htons(1 as libc::c_int as uint16_t);
    (*fm).ttl = htonl(((*hf).ttl as time_t - global_now) as uint32_t);
    if (*hf).ttl == 604801 as libc::c_uint {
        (*fm).ttl = htonl(((*hf).ttl).wrapping_sub(1 as libc::c_uint));
    }
    to = to.offset(::std::mem::size_of::<fillmsg>() as libc::c_ulong as isize);
    if (*hf).type_0 as libc::c_int == 1 as libc::c_int {
        step = 4 as libc::c_int;
    }
    if (*hf).type_0 as libc::c_int == 28 as libc::c_int {
        step = 16 as libc::c_int;
    }
    match (*hf).type_0 as libc::c_int {
        28 | 1 => {
            (*fm).len = htons(step as uint16_t);
            memcpy(to as *mut libc::c_void, from as *const libc::c_void, step as size_t);
            to = to.offset(step as isize);
        }
        2 | 5 => {
            idx += 1;
            *pidx = idx;
            let ref mut fresh6 = (*h.offset(idx as isize)).name;
            *fresh6 = from;
            (*h.offset(idx as isize))
                .off = to.offset_from((*hf).hdr) as libc::c_long as libc::c_short;
            (*h.offset(idx as isize)).ref_0 = -(1 as libc::c_int) as libc::c_short;
            tmp___0 = get_level((*h.offset(idx as isize)).name);
            (*h.offset(idx as isize)).level = tmp___0 as libc::c_short;
            (*h.offset(idx as isize)).mt = 0 as libc::c_int as libc::c_short;
            (*h.offset(idx as isize)).len = (*hf).len as libc::c_short;
            tmp = fill_name_in_msg(h, to, idx);
            (*fm).len = htons(tmp.offset_from(to) as libc::c_long as uint16_t);
            to = tmp;
        }
        15 => {
            memcpy(
                to as *mut libc::c_void,
                from as *const libc::c_void,
                ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
            );
            from = from
                .offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
            to = to.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
            idx += 1;
            *pidx = idx;
            let ref mut fresh7 = (*h.offset(idx as isize)).name;
            *fresh7 = from;
            (*h.offset(idx as isize))
                .off = to.offset_from((*hf).hdr) as libc::c_long as libc::c_short;
            (*h.offset(idx as isize)).ref_0 = -(1 as libc::c_int) as libc::c_short;
            tmp___1 = get_level((*h.offset(idx as isize)).name);
            (*h.offset(idx as isize)).level = tmp___1 as libc::c_short;
            (*h.offset(idx as isize)).mt = 0 as libc::c_int as libc::c_short;
            (*h.offset(idx as isize)).len = (*hf).len as libc::c_short;
            tmp = fill_name_in_msg(h, to, idx);
            (*fm)
                .len = htons(
                (tmp.offset_from(to) as libc::c_long as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                    as uint16_t,
            );
            to = tmp;
        }
        16 => {
            txtlen = *(from as *mut uint16_t);
            from = from
                .offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
            memcpy(
                to as *mut libc::c_void,
                from as *const libc::c_void,
                txtlen as size_t,
            );
            (*fm).len = htons(txtlen);
            to = to.offset(txtlen as libc::c_int as isize);
        }
        33 => {
            memcpy(
                to as *mut libc::c_void,
                from as *const libc::c_void,
                (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                    .wrapping_mul(3 as libc::c_ulong),
            );
            from = from
                .offset(
                    (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                        .wrapping_mul(3 as libc::c_ulong) as isize,
                );
            to = to
                .offset(
                    (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                        .wrapping_mul(3 as libc::c_ulong) as isize,
                );
            idx += 1;
            *pidx = idx;
            let ref mut fresh8 = (*h.offset(idx as isize)).name;
            *fresh8 = from;
            (*h.offset(idx as isize))
                .off = to.offset_from((*hf).hdr) as libc::c_long as libc::c_short;
            (*h.offset(idx as isize)).ref_0 = -(1 as libc::c_int) as libc::c_short;
            tmp___2 = get_level((*h.offset(idx as isize)).name);
            (*h.offset(idx as isize)).level = tmp___2 as libc::c_short;
            (*h.offset(idx as isize)).mt = 0 as libc::c_int as libc::c_short;
            (*h.offset(idx as isize)).len = (*hf).len as libc::c_short;
            tmp = fill_name_in_msg(h, to, idx);
            (*fm)
                .len = htons(
                (tmp.offset_from(to) as libc::c_long as libc::c_ulong)
                    .wrapping_add(
                        (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                            .wrapping_mul(3 as libc::c_ulong),
                    ) as uint16_t,
            );
            to = tmp;
        }
        _ => {}
    }
    return to;
}
pub unsafe extern "C" fn reverse_compare(
    mut from: *mut uchar,
    mut flen: libc::c_int,
    mut to: *mut uchar,
    mut tolen: libc::c_int,
) -> libc::c_int {
    let mut fi: uchar = 0;
    let mut ti: uchar = 0;
    let mut rec: uchar = 0;
    let mut match_0: libc::c_int = 0;
    rec = 0 as libc::c_int as uchar;
    match_0 = 0 as libc::c_int;
    flen -= 1;
    tolen -= 1;
    loop {
        flen -= 1;
        fi = *from.offset(flen as isize);
        tolen -= 1;
        ti = *to.offset(tolen as isize);
        if fi as libc::c_int != ti as libc::c_int {
            break;
        }
        rec = (rec as libc::c_int + 1 as libc::c_int) as uchar;
        if fi as libc::c_int == rec as libc::c_int - 1 as libc::c_int {
            match_0 += 1;
            rec = 0 as libc::c_int as uchar;
        }
        if !(flen != 0) {
            break;
        }
        if tolen == 0 {
            break;
        }
    }
    return match_0;
}
pub unsafe extern "C" fn fill_name_in_msg(
    mut h: *mut hlpc,
    mut to: *mut uchar,
    mut idx: libc::c_int,
) -> *mut uchar {
    let mut i: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut fill: libc::c_int = 0;
    let mut jump: libc::c_int = 0;
    let mut off: libc::c_int = 0;
    let mut base: ushort___0 = 0;
    let mut itor: *mut uchar = 0 as *mut uchar;
    let mut dn: *mut uchar = 0 as *mut uchar;
    m = 0 as libc::c_int;
    fill = 0 as libc::c_int;
    jump = 0 as libc::c_int;
    off = 0 as libc::c_int;
    base = 49152 as libc::c_int as ushort___0;
    itor = (*h.offset(idx as isize)).name;
    dn = 0 as *mut libc::c_void as *mut uchar;
    if idx == 0 as libc::c_int {
        *(to
            as *mut ushort___0) = htons(
            ((*h.offset(0 as libc::c_int as isize)).off as libc::c_int
                + base as libc::c_int) as uint16_t,
        );
        to = to.offset(::std::mem::size_of::<ushort___0>() as libc::c_ulong as isize);
        return to;
    }
    len = (*h.offset(idx as isize)).len as libc::c_int;
    i = idx - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        m = reverse_compare(
            (*h.offset(i as isize)).name,
            (*h.offset(i as isize)).len as libc::c_int,
            (*h.offset(idx as isize)).name,
            len,
        );
        if m > (*h.offset(idx as isize)).mt as libc::c_int {
            (*h.offset(idx as isize)).mt = m as libc::c_short;
            (*h.offset(idx as isize)).ref_0 = i as libc::c_short;
        }
        i -= 1;
    }
    if (*h.offset(idx as isize)).mt as libc::c_int >= 0 as libc::c_int {
        fill = (*h.offset(idx as isize)).level as libc::c_int
            - (*h.offset(idx as isize)).mt as libc::c_int;
    } else {
        fill = (*h.offset(idx as isize)).level as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < fill {
        memcpy(
            to as *mut libc::c_void,
            itor as *const libc::c_void,
            (*itor.offset(0 as libc::c_int as isize) as libc::c_int + 1 as libc::c_int)
                as size_t,
        );
        to = to
            .offset(*itor.offset(0 as libc::c_int as isize) as libc::c_int as isize)
            .offset(1 as libc::c_int as isize);
        itor = itor
            .offset(*itor.offset(0 as libc::c_int as isize) as libc::c_int as isize)
            .offset(1 as libc::c_int as isize);
        i += 1;
    }
    len = 0 as libc::c_int;
    if (*h.offset(idx as isize)).ref_0 as libc::c_int >= 0 as libc::c_int {
        dn = (*h.offset((*h.offset(idx as isize)).ref_0 as libc::c_int as isize)).name;
        jump = (*h.offset((*h.offset(idx as isize)).ref_0 as libc::c_int as isize)).level
            as libc::c_int - (*h.offset(idx as isize)).mt as libc::c_int;
        i = 0 as libc::c_int;
        while i < jump {
            len
                += *dn.offset(0 as libc::c_int as isize) as libc::c_int
                    + 1 as libc::c_int;
            dn = dn
                .offset(
                    (*dn.offset(0 as libc::c_int as isize) as libc::c_int
                        + 1 as libc::c_int) as isize,
                );
            i += 1;
        }
        off = (*h.offset((*h.offset(idx as isize)).ref_0 as libc::c_int as isize)).off
            as libc::c_int + len;
        *(to as *mut ushort___0) = htons((off + base as libc::c_int) as uint16_t);
        to = to.offset(2 as libc::c_int as isize);
    } else {
        *to.offset(0 as libc::c_int as isize) = 0 as libc::c_int as uchar;
        to = to.offset(1);
    }
    return to;
}
pub unsafe extern "C" fn fill_rrset_in_msg(
    mut h: *mut hlpc,
    mut from: *mut uchar,
    mut to: *mut uchar,
    mut pn: *mut libc::c_int,
    mut hdr: *mut uchar,
) -> *mut uchar {
    let mut type_0: uchar = 0;
    let mut i: libc::c_int = 0;
    let mut step: libc::c_int = 0;
    let mut txtlen: uint16_t = 0;
    let mut hf: hlpf = hlpf {
        type_0: 0,
        len: 0,
        ttl: 0,
        hdr: 0 as *mut uchar,
        from: 0 as *mut uchar,
        to: 0 as *mut uchar,
    };
    let mut num: libc::c_int = 0;
    let mut mv: *mut mvalue = 0 as *mut mvalue;
    let mut n: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: size_t = 0;
    step = 0 as libc::c_int;
    txtlen = 0 as libc::c_int as uint16_t;
    num = 0 as libc::c_int;
    mv = 0 as *mut libc::c_void as *mut mvalue;
    n = *pn;
    type_0 = *from.offset(0 as libc::c_int as isize);
    from = from.offset(1);
    mv = from as *mut mvalue;
    from = from.offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize);
    num = (*mv).num as libc::c_int;
    if num > 32 as libc::c_int {
        num = 32 as libc::c_int;
    }
    hf.hdr = hdr;
    hf.ttl = (*mv).ttl;
    hf.type_0 = type_0 as ushort___0;
    if type_0 as libc::c_int == 1 as libc::c_int {
        step = 4 as libc::c_int;
    }
    if type_0 as libc::c_int == 28 as libc::c_int {
        step = 16 as libc::c_int;
    }
    match type_0 as libc::c_int {
        28 | 1 => {
            i = 0 as libc::c_int;
            while i < num {
                to = fill_name_in_msg(h, to, n);
                hf.from = from;
                hf.to = to;
                to = fill_all_records_in_msg(h, &mut hf, pn);
                from = from.offset(step as isize);
                i += 1;
            }
            return to;
        }
        5 => {
            to = fill_name_in_msg(h, to, n);
            hf.from = from;
            hf.to = to;
            tmp = strlen(from as *const libc::c_char);
            hf.len = tmp.wrapping_add(1 as libc::c_ulong) as ushort___0;
            to = fill_all_records_in_msg(h, &mut hf, pn);
            return to;
        }
        2 => {
            i = 0 as libc::c_int;
            while i < num {
                to = fill_name_in_msg(h, to, n);
                hf.from = from;
                hf.to = to;
                tmp___0 = strlen(from as *const libc::c_char);
                hf.len = tmp___0.wrapping_add(1 as libc::c_ulong) as ushort___0;
                to = fill_all_records_in_msg(h, &mut hf, pn);
                from = from.offset(hf.len as libc::c_int as isize);
                i += 1;
            }
            return to;
        }
        15 => {
            i = 0 as libc::c_int;
            while i < num {
                to = fill_name_in_msg(h, to, n);
                hf.from = from;
                hf.to = to;
                tmp___1 = strlen(
                    from
                        .offset(
                            ::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize,
                        ) as *const libc::c_char,
                );
                hf.len = tmp___1.wrapping_add(1 as libc::c_ulong) as ushort___0;
                to = fill_all_records_in_msg(h, &mut hf, pn);
                from = from
                    .offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
                from = from.offset(hf.len as libc::c_int as isize);
                i += 1;
            }
            return to;
        }
        16 => {
            i = 0 as libc::c_int;
            while i < num {
                txtlen = *(from as *mut uint16_t);
                (*h.offset(n as isize)).len = txtlen as libc::c_short;
                to = fill_name_in_msg(h, to, n);
                hf.from = from;
                hf.to = to;
                to = fill_all_records_in_msg(h, &mut hf, pn);
                from = from
                    .offset(txtlen as libc::c_int as isize)
                    .offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
                i += 1;
            }
            return to;
        }
        33 => {
            i = 0 as libc::c_int;
            while i < num {
                to = fill_name_in_msg(h, to, n);
                hf.from = from;
                hf.to = to;
                tmp___2 = strlen(
                    from
                        .offset(
                            (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                                .wrapping_mul(3 as libc::c_ulong) as isize,
                        ) as *const libc::c_char,
                );
                hf.len = tmp___2.wrapping_add(1 as libc::c_ulong) as ushort___0;
                to = fill_all_records_in_msg(h, &mut hf, pn);
                from = from
                    .offset(
                        (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                            .wrapping_mul(3 as libc::c_ulong) as isize,
                    );
                from = from.offset(hf.len as libc::c_int as isize);
                i += 1;
            }
            return to;
        }
        _ => {
            printf(
                b"not support or error in fill msg\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    return 0 as *mut libc::c_void as *mut uchar;
}
pub unsafe extern "C" fn fill_header_in_msg(mut sh: *mut setheader) -> *mut uchar {
    let mut itor: *mut uchar = 0 as *mut uchar;
    let mut hdr: *mut dnsheader = 0 as *mut dnsheader;
    let mut qd: *mut qdns = 0 as *mut qdns;
    itor = (*sh).itor;
    hdr = (*sh).itor as *mut dnsheader;
    (*hdr).flags = 0 as libc::c_int as uint16_t;
    (*hdr).flags = ((*hdr).flags as libc::c_int | 32768 as libc::c_int) as uint16_t;
    (*hdr).flags = ((*hdr).flags as libc::c_int | 128 as libc::c_int) as uint16_t;
    (*hdr)
        .flags = ((*hdr).flags as libc::c_int >> 8 as libc::c_int
        | (((*hdr).flags as libc::c_int) << 8 as libc::c_int) as uint16_t as libc::c_int)
        as uint16_t;
    (*hdr)
        .ancount = ((*sh).an as libc::c_int >> 8 as libc::c_int
        | (((*sh).an as libc::c_int) << 8 as libc::c_int) as uint16_t as libc::c_int)
        as uint16_t;
    (*hdr)
        .nscount = ((*sh).ns as libc::c_int >> 8 as libc::c_int
        | (((*sh).ns as libc::c_int) << 8 as libc::c_int) as uint16_t as libc::c_int)
        as uint16_t;
    (*hdr).arcount = 0 as libc::c_int as uint16_t;
    itor = itor.offset(::std::mem::size_of::<dnsheader>() as libc::c_ulong as isize);
    itor = itor.offset((*sh).dlen as libc::c_int as isize);
    qd = itor as *mut qdns;
    (*qd)
        .type_0 = ((*sh).type_0 as libc::c_int >> 8 as libc::c_int
        | (((*sh).type_0 as libc::c_int) << 8 as libc::c_int) as uint16_t as libc::c_int)
        as uint16_t;
    (*qd)
        .dclass = (1 as libc::c_int >> 8 as libc::c_int
        | ((1 as libc::c_int) << 8 as libc::c_int) as uint16_t as libc::c_int)
        as uint16_t;
    itor = itor.offset(::std::mem::size_of::<qdns>() as libc::c_ulong as isize);
    return itor;
}
pub unsafe extern "C" fn make_dns_msg_for_new(
    mut itor: *mut uchar,
    mut msgid: ushort___0,
    mut d: *mut uchar,
    mut len: libc::c_int,
    mut type_0: ushort___0,
) -> libc::c_int {
    let mut buf: *mut uchar = 0 as *mut uchar;
    let mut hdr: *mut dnsheader = 0 as *mut dnsheader;
    let mut qd: *mut qdns = 0 as *mut qdns;
    let mut tmp: uint16_t = 0;
    let mut tmp___0: uint16_t = 0;
    buf = itor;
    hdr = 0 as *mut libc::c_void as *mut dnsheader;
    qd = 0 as *mut libc::c_void as *mut qdns;
    hdr = buf as *mut dnsheader;
    (*hdr).id = msgid;
    (*hdr).flags = htons(256 as libc::c_int as uint16_t);
    (*hdr).qdcount = htons(1 as libc::c_int as uint16_t);
    tmp___0 = htons(0 as libc::c_int as uint16_t);
    (*hdr).arcount = tmp___0;
    tmp = tmp___0;
    (*hdr).nscount = tmp;
    (*hdr).ancount = tmp;
    buf = buf.offset(::std::mem::size_of::<dnsheader>() as libc::c_ulong as isize);
    memcpy(buf as *mut libc::c_void, d as *const libc::c_void, len as size_t);
    *buf.offset((len - 1 as libc::c_int) as isize) = 0 as libc::c_int as uchar;
    buf = buf.offset(len as isize);
    qd = buf as *mut qdns;
    (*qd).type_0 = htons(type_0);
    (*qd).dclass = htons(1 as libc::c_int as uint16_t);
    buf = buf.offset(4 as libc::c_int as isize);
    return buf.offset_from(itor) as libc::c_long as libc::c_int;
}
pub unsafe extern "C" fn fill_rrset_in_buffer(
    mut buffer: *mut uchar,
    mut label: *mut uchar,
    mut hdr: *mut uchar,
    mut lth: libc::c_int,
    mut type_0: libc::c_int,
    mut hlp: *mut hlpp,
) -> libc::c_int {
    let mut mlen: libc::c_int = 0;
    let mut len: uint16_t = 0;
    let mut from: *mut srv = 0 as *mut srv;
    let mut to: *mut srv = 0 as *mut srv;
    mlen = 0 as libc::c_int;
    len = lth as uint16_t;
    match type_0 {
        1 => {
            mlen = 4 as libc::c_int;
            memcpy(
                buffer as *mut libc::c_void,
                label as *const libc::c_void,
                4 as libc::c_int as size_t,
            );
        }
        5 | 2 => {
            get_domain_from_msg(label, hdr, buffer, &mut mlen);
            to_lowercase(buffer, mlen);
        }
        6 => {
            mlen = 0 as libc::c_int;
        }
        28 => {
            mlen = 16 as libc::c_int;
            memcpy(
                buffer as *mut libc::c_void,
                label as *const libc::c_void,
                16 as libc::c_int as size_t,
            );
        }
        15 => {
            memcpy(
                buffer as *mut libc::c_void,
                label as *const libc::c_void,
                2 as libc::c_int as size_t,
            );
            label = label.offset(2 as libc::c_int as isize);
            buffer = buffer.offset(2 as libc::c_int as isize);
            get_domain_from_msg(label, hdr, buffer, &mut mlen);
            mlen += 2 as libc::c_int;
        }
        33 => {
            from = label as *mut srv;
            to = buffer as *mut srv;
            (*to).pri = (*from).pri;
            (*to).wei = (*from).wei;
            (*to).port = (*from).port;
            buffer = buffer
                .offset(
                    (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                        .wrapping_mul(3 as libc::c_ulong) as isize,
                );
            label = label
                .offset(
                    (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                        .wrapping_mul(3 as libc::c_ulong) as isize,
                );
            get_domain_from_msg(label, hdr, buffer, &mut mlen);
            mlen = (mlen as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                        .wrapping_mul(3 as libc::c_ulong),
                ) as libc::c_int;
        }
        16 => {
            memcpy(
                buffer as *mut libc::c_void,
                &mut len as *mut uint16_t as *const libc::c_void,
                ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
            );
            buffer = buffer
                .offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
            memcpy(
                buffer as *mut libc::c_void,
                label as *const libc::c_void,
                lth as size_t,
            );
            mlen = (lth as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                as libc::c_int;
        }
        _ => return -(1 as libc::c_int),
    }
    return mlen;
}
pub unsafe extern "C" fn check_an_msg(
    mut flag: ushort___0,
    mut domain: *mut uchar,
    mut bk: *mut libc::c_int,
) -> libc::c_int {
    let mut get: uint___0 = 0;
    get = 0 as libc::c_int as uint___0;
    flag = ntohs(flag);
    get = ((flag as libc::c_int & 32768 as libc::c_int) / 32768 as libc::c_int)
        as uint___0;
    if get == 0 as libc::c_uint {
        printf(b"answer set Q sign\n\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    get = ((flag as libc::c_int & 30720 as libc::c_int) >> 11 as libc::c_int)
        as uint___0;
    get = ((flag as libc::c_int & 1024 as libc::c_int) / 1024 as libc::c_int)
        as uint___0;
    get = ((flag as libc::c_int & 512 as libc::c_int) / 512 as libc::c_int) as uint___0;
    if get == 1 as libc::c_uint {
        return 1 as libc::c_int;
    }
    get = ((flag as libc::c_int & 256 as libc::c_int) / 256 as libc::c_int) as uint___0;
    get = (flag as libc::c_int & 7 as libc::c_int) as uint___0;
    if get != 0 as libc::c_uint {
        if get != 3 as libc::c_uint {
            match get {
                2 | 1 | 4 | 5 | _ => {}
            }
            return 2 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn check_out_msg(
    mut cid: ushort___0,
    mut buf: *mut uchar,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut hdr: *mut dnsheader = 0 as *mut dnsheader;
    hdr = buf as *mut dnsheader;
    (*hdr).id = cid;
    (*hdr).flags = 0 as libc::c_int as uint16_t;
    (*hdr)
        .flags = htons(((*hdr).flags as libc::c_int | 32768 as libc::c_int) as uint16_t);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn check_td(mut td: *mut uchar) -> libc::c_int {
    let mut type_0: uchar = 0;
    let mut itor: *mut uchar = 0 as *mut uchar;
    let mut len: uchar = 0;
    type_0 = *td.offset(0 as libc::c_int as isize);
    itor = td.offset(1 as libc::c_int as isize);
    len = *itor.offset(0 as libc::c_int as isize);
    if type_0 as libc::c_int != 1 as libc::c_int {
        if type_0 as libc::c_int != 2 as libc::c_int {
            if type_0 as libc::c_int != 5 as libc::c_int {
                return -(1 as libc::c_int);
            }
        }
    }
    while len as libc::c_int != 0 as libc::c_int {
        if len as libc::c_int > 50 as libc::c_int {
            return -(1 as libc::c_int);
        }
        itor = itor
            .offset(len as libc::c_int as isize)
            .offset(1 as libc::c_int as isize);
        len = *itor.offset(0 as libc::c_int as isize);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn is_glue(
    mut domain: *mut uchar,
    mut ns: *mut uchar,
) -> libc::c_int {
    let mut d: uchar = 0;
    let mut n: uchar = 0;
    let mut dlen: libc::c_int = 0;
    let mut nlen: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    tmp = strlen(domain as *const libc::c_char);
    dlen = tmp as libc::c_int;
    tmp___0 = strlen(ns as *const libc::c_char);
    nlen = tmp___0 as libc::c_int;
    dlen -= 1;
    nlen -= 1;
    if dlen >= nlen {
        return 0 as libc::c_int;
    }
    d = *domain.offset(dlen as isize);
    n = *ns.offset(nlen as isize);
    while d as libc::c_int == n as libc::c_int {
        dlen -= 1;
        nlen -= 1;
        if dlen == 0 as libc::c_int {
            return 1 as libc::c_int;
        }
        d = *domain.offset(dlen as isize);
        n = *ns.offset(nlen as isize);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn pre_find(
    mut mbuf: *mut mbuf_type,
    mut fwd: *mut htable,
    mut ht: *mut htable,
    mut ip: *mut uchar,
) -> libc::c_int {
    let mut td: *mut uchar = 0 as *mut uchar;
    let mut itor: *mut uchar = 0 as *mut uchar;
    let mut xlen: libc::c_int = 0;
    let mut dbg___0: libc::c_int = 0;
    let mut mv: *mut mvalue = 0 as *mut mvalue;
    let mut td_len: libc::c_int = 0;
    let mut new_td_len: libc::c_int = 0;
    let mut hash: *mut hashval_t = 0 as *mut hashval_t;
    let mut thash: hashval_t = 0;
    let mut new_td: *mut uchar = 0 as *mut uchar;
    let mut rlen: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    itor = 0 as *mut libc::c_void as *mut uchar;
    xlen = 0 as libc::c_int;
    dbg___0 = 100 as libc::c_int;
    mv = 0 as *mut libc::c_void as *mut mvalue;
    thash = 0 as libc::c_int as hashval_t;
    (*mbuf).qname = 4 as libc::c_int as ushort___0;
    if (*mbuf).hascname as libc::c_int == 1 as libc::c_int {
        (*mbuf).qing = ((*mbuf).qbuffer).as_mut_ptr();
        td_len = (*mbuf).qlen as libc::c_int;
        td = ((*mbuf).qbuffer).as_mut_ptr();
        (*mbuf).qhash = &mut (*mbuf).qbuffer_hash;
    } else {
        td_len = (*mbuf).dlen;
        (*mbuf).qing = (*mbuf).td;
        (*mbuf).qlen = (*mbuf).dlen as ushort___0;
        td = (*mbuf).td;
        (*mbuf)
            .qhash = &mut *((*mbuf).lowerdomain.hash)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut hashval_t;
    }
    hash = (*mbuf).qhash;
    xlen = htable_find(
        fwd,
        td,
        td_len,
        1 as libc::c_int,
        ip,
        1900 as libc::c_int,
        0 as *mut libc::c_void as *mut mvalue,
        hash,
    );
    if xlen > 0 as libc::c_int {
        ip = ip.offset(xlen as isize);
        mv = ip as *mut mvalue;
        (*mv).num = 0 as libc::c_int as uint16_t;
        (*mv).ttl = 0 as libc::c_int as uint32_t;
        (*mv).hits = 0 as libc::c_int as uint32_t;
        (*mv).len = 0 as libc::c_int as uint16_t;
        return xlen;
    } else {
        new_td = (*mbuf).tdbuffer;
        if (*mbuf).lowerdomain.label_count as libc::c_int > 1 as libc::c_int {
            *new_td.offset(0 as libc::c_int as isize) = 1 as libc::c_int as uchar;
            *new_td.offset(1 as libc::c_int as isize) = '*' as i32 as uchar;
            new_td_len = (*mbuf)
                .lowerdomain
                .label_len[((*mbuf).lowerdomain.label_count as libc::c_int
                - 2 as libc::c_int) as usize] as libc::c_int;
            memcpy(
                new_td.offset(2 as libc::c_int as isize) as *mut libc::c_void,
                (*mbuf)
                    .lowerdomain
                    .label[((*mbuf).lowerdomain.label_count as libc::c_int
                    - 2 as libc::c_int) as usize] as *const libc::c_void,
                new_td_len as size_t,
            );
            thash = 0 as libc::c_int as hashval_t;
            tmp = htable_find(
                fwd,
                new_td,
                new_td_len + 2 as libc::c_int,
                1 as libc::c_int,
                ip,
                1900 as libc::c_int,
                0 as *mut libc::c_void as *mut mvalue,
                &mut thash,
            );
            rlen = tmp;
            if rlen > 0 as libc::c_int {
                ip = ip.offset(rlen as isize);
                mv = ip as *mut mvalue;
                (*mv).num = 0 as libc::c_int as uint16_t;
                (*mv).ttl = 0 as libc::c_int as uint32_t;
                (*mv).hits = 0 as libc::c_int as uint32_t;
                (*mv).len = 0 as libc::c_int as uint16_t;
                return rlen;
            }
        }
    }
    if (*mbuf).qtype as libc::c_uint == 5 as libc::c_uint {
        return 0 as libc::c_int;
    }
    itor = (*mbuf).tempbuffer;
    loop {
        xlen = find_record_with_ttl(
            ht,
            td,
            td_len,
            5 as libc::c_int,
            itor,
            2000 as libc::c_int,
            0 as *mut libc::c_void as *mut mvalue,
            hash,
        );
        if !(xlen > 0 as libc::c_int) {
            break;
        }
        (*mbuf).qname = 3 as libc::c_int as ushort___0;
        (*mbuf).hascname = 1 as libc::c_int as ushort___0;
        mv = itor as *mut mvalue;
        itor = itor.offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize);
        if (*mv).len as libc::c_int > 255 as libc::c_int {
            return -(1 as libc::c_int);
        }
        memcpy(
            ((*mbuf).qbuffer).as_mut_ptr() as *mut libc::c_void,
            itor as *const libc::c_void,
            (*mv).len as size_t,
        );
        (*mbuf).qing = ((*mbuf).qbuffer).as_mut_ptr();
        td_len = (*mv).len as libc::c_int;
        (*mbuf).qlen = td_len as ushort___0;
        (*mbuf).qbuffer_hash = 0 as libc::c_int as hashval_t;
        hash = &mut (*mbuf).qbuffer_hash;
        td = ((*mbuf).qbuffer).as_mut_ptr();
        tmp___0 = dbg___0;
        dbg___0 -= 1;
        if tmp___0 == 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn transfer_record_to_msg(
    mut buff: *mut uchar,
    mut key: *mut uchar,
    mut msg: *mut uchar,
    mut msglen: libc::c_int,
    mut ttloff: *mut uint16_t,
) -> libc::c_int {
    let mut segs: uint16_t = 0;
    let mut totallen: uint16_t = 0;
    let mut itor: *mut uchar = 0 as *mut uchar;
    let mut mv: *mut mvalue = 0 as *mut mvalue;
    segs = *ttloff.offset(0 as libc::c_int as isize);
    totallen = 0 as libc::c_int as uint16_t;
    itor = 0 as *mut libc::c_void as *mut uchar;
    mv = 0 as *mut libc::c_void as *mut mvalue;
    if segs as libc::c_int == 0 as libc::c_int {
        return -(1 as libc::c_int)
    } else {
        if segs as libc::c_int > 100 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    totallen = msglen as uint16_t;
    totallen = (totallen as libc::c_ulong)
        .wrapping_add(
            (segs as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<uint16_t>() as libc::c_ulong),
        )
        .wrapping_add(::std::mem::size_of::<mvalue>() as libc::c_ulong) as uint16_t;
    if totallen as libc::c_int > 2048 as libc::c_int {
        return -(1 as libc::c_int);
    }
    itor = buff;
    mv = itor as *mut mvalue;
    (*mv).seg = segs;
    (*mv).len = msglen as uint16_t;
    itor = itor.offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize);
    memcpy(
        itor as *mut libc::c_void,
        ttloff.offset(1 as libc::c_int as isize) as *const libc::c_void,
        (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
            .wrapping_mul(segs as libc::c_ulong),
    );
    itor = itor
        .offset(
            (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                .wrapping_mul(segs as libc::c_ulong) as isize,
        );
    memcpy(itor as *mut libc::c_void, msg as *const libc::c_void, msglen as size_t);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn make_A_record_from_segment(
    mut ipmsg: *mut uchar,
    mut iitor: *mut uchar,
) -> libc::c_int {
    let mut reallen: libc::c_int = 0;
    let mut ipto: *mut uchar = 0 as *mut uchar;
    let mut ipfrom: *mut uchar = 0 as *mut uchar;
    let mut mv: *mut mvalue = 0 as *mut mvalue;
    let mut off: uint16_t = 0;
    let mut segs: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    reallen = 0 as libc::c_int;
    ipto = 0 as *mut libc::c_void as *mut uchar;
    ipfrom = 0 as *mut libc::c_void as *mut uchar;
    mv = 0 as *mut libc::c_void as *mut mvalue;
    segs = 0 as libc::c_int;
    mv = ipmsg as *mut mvalue;
    segs = (*mv).seg as libc::c_int;
    ipto = iitor.offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize);
    i = 0 as libc::c_int;
    while i < segs {
        off = *(ipmsg
            .offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize)
            .offset(
                (i as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                    as isize,
            ) as *mut uint16_t);
        ipfrom = ipmsg.offset(off as libc::c_int as isize);
        memcpy(
            ipto as *mut libc::c_void,
            ipfrom as *const libc::c_void,
            4 as libc::c_int as size_t,
        );
        reallen += 4 as libc::c_int;
        ipto = ipto.offset(4 as libc::c_int as isize);
        i += 1;
    }
    (*mv).len = reallen as uint16_t;
    memcpy(
        iitor as *mut libc::c_void,
        ipmsg as *const libc::c_void,
        ::std::mem::size_of::<mvalue>() as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn retrive_ip(
    mut mbuf: *mut mbuf_type,
    mut itor: *mut uchar,
    mut num: libc::c_int,
    mut ip: *mut uchar,
    mut ht: *mut htable,
    mut fq: *mut libc::c_int,
) -> libc::c_int {
    let mut mi: *mut mvalue = 0 as *mut mvalue;
    let mut i: libc::c_int = 0;
    let mut xlen: libc::c_int = 0;
    let mut iplen: libc::c_int = 0;
    let mut got: libc::c_int = 0;
    let mut ipbuffer: *mut uchar = 0 as *mut uchar;
    let mut nstd: *mut uchar = 0 as *mut uchar;
    let mut iitor: *mut uchar = 0 as *mut uchar;
    let mut hash: hashval_t = 0;
    let mut tmp: size_t = 0;
    mi = 0 as *mut libc::c_void as *mut mvalue;
    iplen = 2000 as libc::c_int;
    got = 0 as libc::c_int;
    ipbuffer = (*mbuf).ipbuffer;
    *fq = 0 as libc::c_int;
    iitor = ip;
    i = 0 as libc::c_int;
    while i < num {
        tmp = strlen(itor as *const libc::c_char);
        xlen = tmp.wrapping_add(1 as libc::c_ulong) as libc::c_int;
        nstd = itor;
        itor = itor.offset(xlen as isize);
        hash = 0 as libc::c_int as hashval_t;
        xlen = find_record_with_ttl(
            ht,
            nstd,
            xlen,
            1 as libc::c_int,
            ipbuffer,
            (iplen as libc::c_ulong)
                .wrapping_sub(::std::mem::size_of::<mvalue>() as libc::c_ulong)
                as libc::c_int,
            0 as *mut libc::c_void as *mut mvalue,
            &mut hash,
        );
        if xlen > 0 as libc::c_int {
            mi = ipbuffer as *mut mvalue;
            if (*mi).seg as libc::c_int > 0 as libc::c_int {
                make_A_record_from_segment(ipbuffer, iitor);
            } else {
                memcpy(
                    iitor as *mut libc::c_void,
                    ipbuffer as *const libc::c_void,
                    ((*mi).len as libc::c_ulong)
                        .wrapping_add(::std::mem::size_of::<mvalue>() as libc::c_ulong),
                );
            }
            iitor = iitor
                .offset((*mi).len as libc::c_int as isize)
                .offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize);
            iplen = ((iplen - (*mi).len as libc::c_int) as libc::c_ulong)
                .wrapping_sub(::std::mem::size_of::<mvalue>() as libc::c_ulong)
                as libc::c_int;
            got += 1;
        }
        if xlen < 0 as libc::c_int {
            *fq = i;
            break;
        } else {
            i += 1;
        }
    }
    if iitor as libc::c_ulong != ip as libc::c_ulong {
        mi = iitor as *mut mvalue;
        (*mi).num = 0 as libc::c_int as uint16_t;
        (*mi).ttl = 0 as libc::c_int as uint32_t;
        (*mi).hits = 0 as libc::c_int as uint32_t;
        (*mi).len = 0 as libc::c_int as uint16_t;
        return got;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn fill_extra_addr(mut ip: *mut uchar) -> libc::c_int {
    let mut extra: [*const libc::c_char; 2] = [0 as *const libc::c_char; 2];
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut mv: *mut mvalue = 0 as *mut mvalue;
    let mut tmp: libc::c_int = 0;
    extra[0 as libc::c_int
        as usize] = g_nameservers[0 as libc::c_int as usize] as *const libc::c_char;
    extra[1 as libc::c_int
        as usize] = g_nameservers[1 as libc::c_int as usize] as *const libc::c_char;
    mv = 0 as *mut libc::c_void as *mut mvalue;
    n = (::std::mem::size_of::<[*const libc::c_char; 2]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
        as libc::c_int;
    mv = ip as *mut mvalue;
    ip = ip.offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize);
    (*mv).num = 0 as libc::c_int as uint16_t;
    (*mv).ttl = 0 as libc::c_int as uint32_t;
    (*mv).hits = 0 as libc::c_int as uint32_t;
    (*mv).len = 0 as libc::c_int as uint16_t;
    (*mv).seg = 0 as libc::c_int as uint16_t;
    i = 0 as libc::c_int;
    while i < n {
        tmp = make_bin_from_str(ip, extra[i as usize]);
        if tmp == 0 as libc::c_int {
            (*mv).num = ((*mv).num as libc::c_int + 1 as libc::c_int) as uint16_t;
            (*mv).len = ((*mv).len as libc::c_int + 4 as libc::c_int) as uint16_t;
            ip = ip.offset(4 as libc::c_int as isize);
        }
        i += 1;
    }
    mv = ip as *mut mvalue;
    (*mv).num = 0 as libc::c_int as uint16_t;
    (*mv).ttl = 0 as libc::c_int as uint32_t;
    (*mv).hits = 0 as libc::c_int as uint32_t;
    (*mv).len = 0 as libc::c_int as uint16_t;
    (*mv).seg = 0 as libc::c_int as uint16_t;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn find_addr(
    mut fwd: *mut htable,
    mut ht: *mut htable,
    mut mbuf: *mut mbuf_type,
    mut ip: *mut uchar,
    mut forward: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut xlen: libc::c_int = 0;
    let mut dbg___0: libc::c_int = 0;
    let mut first_query: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut mv: *mut mvalue = 0 as *mut mvalue;
    let mut td: *mut uchar = 0 as *mut uchar;
    let mut buffer: *mut uchar = 0 as *mut uchar;
    let mut itor: *mut uchar = 0 as *mut uchar;
    let mut glue: *mut uchar = 0 as *mut uchar;
    let mut td_len: libc::c_int = 0;
    let mut diff_len: libc::c_int = 0;
    let mut ori_flag: libc::c_int = 0;
    let mut root_flag: libc::c_int = 0;
    let mut thash: hashval_t = 0;
    let mut hash: *mut hashval_t = 0 as *mut hashval_t;
    let mut label_count: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    xlen = 0 as libc::c_int;
    dbg___0 = 100 as libc::c_int;
    mv = 0 as *mut libc::c_void as *mut mvalue;
    buffer = (*mbuf).tempbuffer;
    itor = 0 as *mut libc::c_void as *mut uchar;
    glue = 0 as *mut libc::c_void as *mut uchar;
    ori_flag = 0 as libc::c_int;
    root_flag = 0 as libc::c_int;
    label_count = 0 as libc::c_int;
    if (*mbuf).qtimes as libc::c_int > 12 as libc::c_int {
        fill_extra_addr(ip);
        return 0 as libc::c_int;
    }
    ret = pre_find(mbuf, fwd, ht, ip);
    if ret > 0 as libc::c_int {
        return 0 as libc::c_int
    } else {
        if ret < 0 as libc::c_int {
            return ret
        } else {
            if forward != 0 {
                fill_extra_addr(ip);
                return 0 as libc::c_int;
            }
        }
    }
    td = (*mbuf).qing;
    itor = td;
    hash = (*mbuf).qhash;
    td_len = (*mbuf).qlen as libc::c_int;
    if (*mbuf).hascname != 0 {
        ori_flag = 1 as libc::c_int;
    }
    loop {
        loop {
            ret = find_record_with_ttl(
                ht,
                itor,
                td_len,
                2 as libc::c_int,
                buffer,
                2000 as libc::c_int,
                0 as *mut libc::c_void as *mut mvalue,
                hash,
            );
            if ret > 0 as libc::c_int {
                break;
            }
            tmp = dbg___0;
            dbg___0 -= 1;
            if tmp == 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            if ori_flag != 0 {
                diff_len = *itor.offset(0 as libc::c_int as isize) as libc::c_int
                    + 1 as libc::c_int;
                itor = itor.offset(diff_len as isize);
                if *itor.offset(0 as libc::c_int as isize) as libc::c_int
                    == 0 as libc::c_int
                {
                    if root_flag == 0 as libc::c_int {
                        *itor
                            .offset(1 as libc::c_int as isize) = '\u{0}' as i32 as uchar;
                        root_flag = 1 as libc::c_int;
                        td_len = 2 as libc::c_int;
                    } else {
                        return -(1 as libc::c_int)
                    }
                } else {
                    td_len -= diff_len;
                }
                thash = 0 as libc::c_int as hashval_t;
                hash = &mut thash;
            } else {
                label_count += 1;
                if label_count > (*mbuf).lowerdomain.label_count as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if label_count == (*mbuf).lowerdomain.label_count as libc::c_int {
                    itor = ((*mbuf)
                        .lowerdomain
                        .label[(label_count - 1 as libc::c_int) as usize])
                        .offset(
                            (*mbuf)
                                .lowerdomain
                                .label_len[(label_count - 1 as libc::c_int) as usize]
                                as libc::c_int as isize,
                        );
                    *itor.offset(1 as libc::c_int as isize) = '\u{0}' as i32 as uchar;
                    td_len = 2 as libc::c_int;
                    thash = 0 as libc::c_int as hashval_t;
                    hash = &mut thash;
                } else {
                    itor = (*mbuf).lowerdomain.label[label_count as usize];
                    td_len = (*mbuf).lowerdomain.label_len[label_count as usize]
                        as libc::c_int;
                    hash = &mut *((*mbuf).lowerdomain.hash)
                        .as_mut_ptr()
                        .offset(label_count as isize) as *mut hashval_t;
                }
            }
        }
        mv = buffer as *mut mvalue;
        glue = itor;
        itor = buffer.offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize);
        ret = retrive_ip(mbuf, itor, (*mv).num as libc::c_int, ip, ht, &mut first_query);
        if ret > 0 as libc::c_int {
            if ret < (*mv).num as libc::c_int {
                if (*mbuf).qns == 1 as libc::c_int {
                    (*mbuf).qns = 0 as libc::c_int;
                    i = 0 as libc::c_int;
                    while i < first_query {
                        tmp___0 = strlen(itor as *const libc::c_char);
                        xlen = tmp___0.wrapping_add(1 as libc::c_ulong) as libc::c_int;
                        itor = itor.offset(xlen as isize);
                        i += 1;
                    }
                } else {
                    return 0 as libc::c_int
                }
            } else {
                return 0 as libc::c_int
            }
        }
        tmp___2 = is_glue(glue, itor);
        if tmp___2 != 1 as libc::c_int {
            if ori_flag == 0 {
                ori_flag = 1 as libc::c_int;
            }
            tmp___1 = strlen(itor as *const libc::c_char);
            xlen = tmp___1.wrapping_add(1 as libc::c_ulong) as libc::c_int;
            if xlen > 255 as libc::c_int {
                return -(1 as libc::c_int)
            } else {
                if *itor.offset(0 as libc::c_int as isize) as libc::c_int
                    == 0 as libc::c_int
                {
                    return -(1 as libc::c_int)
                } else {
                    if *itor.offset(0 as libc::c_int as isize) as libc::c_int > xlen {
                        return -(1 as libc::c_int);
                    }
                }
            }
            memcpy(
                ((*mbuf).qbuffer).as_mut_ptr() as *mut libc::c_void,
                itor as *const libc::c_void,
                xlen as size_t,
            );
            (*mbuf).qbuffer_hash = 0 as libc::c_int as hashval_t;
            (*mbuf).qing = ((*mbuf).qbuffer).as_mut_ptr();
            (*mbuf).qhash = &mut (*mbuf).qbuffer_hash;
            (*mbuf).qlen = xlen as ushort___0;
            hash = (*mbuf).qhash;
            td_len = (*mbuf).qlen as libc::c_int;
            td = (*mbuf).qing;
            itor = td;
        } else if ori_flag != 0 {
            diff_len = *glue.offset(0 as libc::c_int as isize) as libc::c_int
                + 1 as libc::c_int;
            itor = glue.offset(diff_len as isize);
            if *itor.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            td_len -= diff_len;
            thash = 0 as libc::c_int as hashval_t;
            hash = &mut thash;
        } else {
            label_count += 1;
            if label_count >= (*mbuf).lowerdomain.label_count as libc::c_int {
                return -(1 as libc::c_int);
            }
            itor = (*mbuf).lowerdomain.label[label_count as usize];
            td_len = (*mbuf).lowerdomain.label_len[label_count as usize] as libc::c_int;
            hash = &mut *((*mbuf).lowerdomain.hash)
                .as_mut_ptr()
                .offset(label_count as isize) as *mut hashval_t;
        }
        (*mbuf).qname = 6 as libc::c_int as ushort___0;
        tmp___3 = dbg___0;
        dbg___0 -= 1;
        if tmp___3 == 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    };
}
pub unsafe extern "C" fn check_qo(mut qo: *mut qoutinfo) -> libc::c_int {
    if qo as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if (*qo).hascname as libc::c_int > 1 as libc::c_int {
        printf(b"qo error\n\0" as *const u8 as *const libc::c_char);
    }
    if (*qo).td as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        printf(b"qo error2\n\0" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dbg_print_label(
    mut label: *mut uchar,
    mut visible: libc::c_int,
) -> *mut uchar {
    let mut i: uchar = 0;
    let mut len: uchar = 0;
    len = *label;
    if visible == 1 as libc::c_int {
        i = 1 as libc::c_int as uchar;
        while (i as libc::c_int) < len as libc::c_int + 1 as libc::c_int {
            printf(
                b"%c\0" as *const u8 as *const libc::c_char,
                *label.offset(i as libc::c_int as isize) as libc::c_int,
            );
            i = (i as libc::c_int + 1 as libc::c_int) as uchar;
        }
    }
    return label
        .offset(*label.offset(0 as libc::c_int as isize) as libc::c_int as isize)
        .offset(1 as libc::c_int as isize);
}
pub unsafe extern "C" fn dbg_print_domain(
    mut hdr: *mut uchar,
    mut itor: *mut uchar,
) -> *mut uchar {
    let mut len: uchar = 0;
    let mut tmp: *mut uchar = 0 as *mut uchar;
    let mut offset: ushort___0 = 0;
    let mut debug: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = 0 as *mut libc::c_void as *mut uchar;
    debug = 100 as libc::c_int;
    len = *itor.offset(0 as libc::c_int as isize);
    if len as libc::c_int == 0 as libc::c_int {
        printf(b"root\n\0" as *const u8 as *const libc::c_char);
        return 0 as *mut uchar;
    }
    offset = ntohs(*(itor as *mut ushort___0));
    if offset as libc::c_int >= 49152 as libc::c_int {
        if offset as libc::c_int <= 53247 as libc::c_int {
            itor = hdr.offset((offset as libc::c_int & 16383 as libc::c_int) as isize);
        }
    }
    while len as libc::c_int != 0 as libc::c_int {
        tmp___0 = debug;
        debug -= 1;
        if tmp___0 == 0 {
            break;
        }
        if offset as libc::c_int >= 49152 as libc::c_int {
            if offset as libc::c_int <= 53247 as libc::c_int {
                tmp = itor.offset(2 as libc::c_int as isize);
                itor = dbg_print_label(
                    hdr.offset((offset as libc::c_int & 16383 as libc::c_int) as isize),
                    1 as libc::c_int,
                );
            } else {
                itor = dbg_print_label(itor, 1 as libc::c_int);
            }
        } else {
            itor = dbg_print_label(itor, 1 as libc::c_int);
        }
        printf(b".\0" as *const u8 as *const libc::c_char);
        len = *itor.offset(0 as libc::c_int as isize);
        offset = ntohs(*(itor as *mut ushort___0));
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if tmp as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp = itor.offset(1 as libc::c_int as isize);
    }
    return tmp;
}
pub unsafe extern "C" fn dbg_print_ip(mut ip: *mut uchar, mut type_0: rrtype) {
    let mut i: libc::c_int = 0;
    let mut ipv4: [uint___0; 4] = [0; 4];
    let mut tmp: libc::c_uint = 0;
    ipv4[0 as libc::c_int as usize] = 0 as libc::c_int as uint___0;
    tmp = 1 as libc::c_uint;
    while !(tmp >= 4 as libc::c_uint) {
        ipv4[tmp as usize] = 0 as libc::c_uint;
        tmp = tmp.wrapping_add(1);
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        ipv4[i as usize] = *ip.offset(i as isize) as uint___0;
        i += 1;
    }
    if type_0 as libc::c_uint == 1 as libc::c_uint {
        printf(
            b"%u.%u.%u.%u\n\0" as *const u8 as *const libc::c_char,
            ipv4[0 as libc::c_int as usize] as libc::c_ushort as libc::c_int,
            ipv4[1 as libc::c_int as usize],
            ipv4[2 as libc::c_int as usize],
            ipv4[3 as libc::c_int as usize],
        );
    } else if type_0 as libc::c_uint == 28 as libc::c_uint {
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            if *ip.offset((i * 2 as libc::c_int) as isize) as libc::c_int
                != 0 as libc::c_int
            {
                if (*ip.offset((i * 2 as libc::c_int) as isize) as libc::c_int)
                    < 16 as libc::c_int
                {
                    printf(b"0\0" as *const u8 as *const libc::c_char);
                }
                printf(
                    b"%x\0" as *const u8 as *const libc::c_char,
                    *ip.offset((i * 2 as libc::c_int) as isize) as uint___0,
                );
            }
            if (*ip.offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize)
                as libc::c_int) < 16 as libc::c_int
            {
                printf(b"0\0" as *const u8 as *const libc::c_char);
            }
            printf(
                b"%x\0" as *const u8 as *const libc::c_char,
                *ip.offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize)
                    as uint___0,
            );
            if i != 7 as libc::c_int {
                printf(b":\0" as *const u8 as *const libc::c_char);
            }
            i += 1;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(
            b"unknow type %d\n\0" as *const u8 as *const libc::c_char,
            type_0 as libc::c_uint,
        );
    };
}
pub unsafe extern "C" fn dbg_print_td(mut td: *mut uchar) -> libc::c_int {
    let mut c: uchar = 0;
    c = *td.offset(0 as libc::c_int as isize);
    printf(b"%d,\0" as *const u8 as *const libc::c_char, c as libc::c_int);
    dbg_print_domain(
        0 as *mut libc::c_void as *mut uchar,
        td.offset(1 as libc::c_int as isize),
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn jump_space(mut itor: *mut uchar) -> *mut uchar {
    let mut t: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    t = 100 as libc::c_int;
    while *itor.offset(0 as libc::c_int as isize) as libc::c_int != 32 as libc::c_int {
        if !(*itor.offset(0 as libc::c_int as isize) as libc::c_int != 9 as libc::c_int)
        {
            break;
        }
        tmp = t;
        t -= 1;
        if tmp == 0 {
            break;
        }
        itor = itor.offset(1);
    }
    *itor.offset(0 as libc::c_int as isize) = 0 as libc::c_int as uchar;
    itor = itor.offset(1);
    loop {
        if !(*itor.offset(0 as libc::c_int as isize) as libc::c_int == 32 as libc::c_int)
        {
            if !(*itor.offset(0 as libc::c_int as isize) as libc::c_int
                == 9 as libc::c_int)
            {
                break;
            }
        }
        itor = itor.offset(1);
        tmp___0 = t;
        t -= 1;
        if tmp___0 == 0 as libc::c_int {
            printf(b"error line in file\n\0" as *const u8 as *const libc::c_char);
            return 0 as *mut libc::c_void as *mut uchar;
        }
    }
    return itor;
}
pub unsafe extern "C" fn read_records_from_file(
    mut fn_0: *const libc::c_char,
    mut ds: *mut htable,
    mut rbt: *mut rbtree,
    mut hijack___0: libc::c_int,
) -> libc::c_int {
    let mut fd: *mut FILE = 0 as *mut FILE;
    let mut vbuffer: [uchar; 5000] = [0; 5000];
    let mut tmp: libc::c_uint = 0;
    let mut ipv4: [uchar; 4] = [0; 4];
    let mut ipv6: [uchar; 16] = [0; 16];
    let mut rbuffer: [uchar; 1024] = [0; 1024];
    let mut tmp___0: libc::c_uint = 0;
    let mut tmpdomain: [uchar; 256] = [0; 256];
    let mut tmp___1: libc::c_uint = 0;
    let mut tmptype: [uchar; 10] = [0; 10];
    let mut tmp___2: libc::c_uint = 0;
    let mut ps: [*mut uchar; 5] = [0 as *mut uchar; 5];
    let mut tmp___3: libc::c_uint = 0;
    let mut ritor: *mut uchar = 0 as *mut uchar;
    let mut vitor: *mut uchar = 0 as *mut uchar;
    let mut tmplen: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut kbuffer: *mut uchar = 0 as *mut uchar;
    let mut ttl: uint___0 = 0;
    let mut tmpttl: uint___0 = 0;
    let mut dlen: libc::c_int = 0;
    let mut mv: *mut mvalue = 0 as *mut mvalue;
    let mut lowerdomain: packet_type = packet_type {
        label_count: 0,
        domain: [0; 256],
        label: [0 as *mut uint8_t; 64],
        label_offsets: [0; 64],
        label_len: [0; 64],
        hash: [0; 64],
    };
    let mut lowerns: packet_type = packet_type {
        label_count: 0,
        domain: [0; 256],
        label: [0 as *mut uint8_t; 64],
        label_offsets: [0; 64],
        label_len: [0; 64],
        hash: [0; 64],
    };
    let mut tmp___4: size_t = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: size_t = 0;
    let mut tmp___11: size_t = 0;
    let mut tmp___12: size_t = 0;
    let mut tmp___13: libc::c_int = 0;
    let mut tmp___14: libc::c_int = 0;
    let mut tmp___15: size_t = 0;
    let mut tmp___16: libc::c_int = 0;
    let mut tmp___17: libc::c_int = 0;
    let mut tmp___18: libc::c_int = 0;
    let mut tmp___19: libc::c_int = 0;
    let mut tmp___20: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___21: libc::c_int = 0;
    let mut tmp___22: libc::c_int = 0;
    let mut tmp___23: libc::c_int = 0;
    let mut tmp___24: size_t = 0;
    fd = 0 as *mut libc::c_void as *mut FILE;
    vbuffer[0 as libc::c_int as usize] = 0 as libc::c_int as uchar;
    tmp = 1 as libc::c_uint;
    while !(tmp >= 5000 as libc::c_uint) {
        vbuffer[tmp as usize] = 0 as libc::c_int as libc::c_uchar;
        tmp = tmp.wrapping_add(1);
    }
    rbuffer[0 as libc::c_int as usize] = 0 as libc::c_int as uchar;
    tmp___0 = 1 as libc::c_uint;
    while !(tmp___0 >= 1024 as libc::c_uint) {
        rbuffer[tmp___0 as usize] = 0 as libc::c_int as libc::c_uchar;
        tmp___0 = tmp___0.wrapping_add(1);
    }
    tmpdomain[0 as libc::c_int as usize] = '.' as i32 as uchar;
    tmpdomain[1 as libc::c_int as usize] = '\u{0}' as i32 as uchar;
    tmp___1 = 2 as libc::c_uint;
    while !(tmp___1 >= 256 as libc::c_uint) {
        tmpdomain[tmp___1 as usize] = 0 as libc::c_int as libc::c_uchar;
        tmp___1 = tmp___1.wrapping_add(1);
    }
    tmptype[0 as libc::c_int as usize] = 'N' as i32 as uchar;
    tmptype[1 as libc::c_int as usize] = 'S' as i32 as uchar;
    tmptype[2 as libc::c_int as usize] = '\u{0}' as i32 as uchar;
    tmp___2 = 3 as libc::c_uint;
    while !(tmp___2 >= 10 as libc::c_uint) {
        tmptype[tmp___2 as usize] = 0 as libc::c_int as libc::c_uchar;
        tmp___2 = tmp___2.wrapping_add(1);
    }
    ps[0 as libc::c_int as usize] = 0 as *mut uchar;
    tmp___3 = 1 as libc::c_uint;
    while !(tmp___3 >= 5 as libc::c_uint) {
        ps[tmp___3 as usize] = 0 as *mut uchar;
        tmp___3 = tmp___3.wrapping_add(1);
    }
    ritor = 0 as *mut libc::c_void as *mut uchar;
    vitor = vbuffer.as_mut_ptr();
    tmplen = 0 as libc::c_int;
    type_0 = 0 as libc::c_int;
    ttl = 0 as libc::c_int as uint___0;
    tmpttl = 0 as libc::c_int as uint___0;
    mv = vbuffer.as_mut_ptr() as *mut mvalue;
    if ds as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        dns_error(
            0 as libc::c_int,
            b"datasets null\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    fd = fopen(fn_0, b"r\0" as *const u8 as *const libc::c_char);
    if fd as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"open file %s error\n\0" as *const u8 as *const libc::c_char,
            fn_0,
        );
        perror(b"fopen\0" as *const u8 as *const libc::c_char);
        dns_error(
            0 as libc::c_int,
            b"open file root.z\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    kbuffer = (lowerdomain.domain).as_mut_ptr();
    (*mv).num = 0 as libc::c_int as uint16_t;
    (*mv).ttl = 0 as libc::c_int as uint32_t;
    (*mv).len = 0 as libc::c_int as uint16_t;
    (*mv).seg = 0 as libc::c_int as uint16_t;
    vitor = vbuffer
        .as_mut_ptr()
        .offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize);
    loop {
        tmp___20 = fgets(
            rbuffer.as_mut_ptr() as *mut libc::c_char,
            1024 as libc::c_int,
            fd,
        );
        if !(tmp___20 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
            break;
        }
        ritor = rbuffer.as_mut_ptr();
        ps[0 as libc::c_int as usize] = ritor;
        i = 1 as libc::c_int;
        while i < 5 as libc::c_int {
            ritor = jump_space(ritor);
            ps[i as usize] = ritor;
            i += 1;
        }
        tmp___4 = strlen(ps[0 as libc::c_int as usize] as *const libc::c_char);
        to_lowercase(
            ps[0 as libc::c_int as usize],
            tmp___4.wrapping_add(1 as libc::c_ulong) as libc::c_int,
        );
        fix_tail(ps[4 as libc::c_int as usize] as *mut libc::c_char);
        tmp___5 = atoi(ps[1 as libc::c_int as usize] as *const libc::c_char);
        tmpttl = tmp___5 as uint___0;
        ttl = (tmpttl as time_t + global_now) as uint___0;
        if tmpttl >= 604801 as libc::c_uint {
            ttl = tmpttl;
        }
        if tmpttl == 172800 as libc::c_uint {
            ttl = 604801 as libc::c_int as uint___0;
        }
        if tmpttl == 518400 as libc::c_uint {
            ttl = 604801 as libc::c_int as uint___0;
        }
        tmp___13 = strcmp(
            ps[0 as libc::c_int as usize] as *const libc::c_char,
            tmpdomain.as_mut_ptr() as *const libc::c_char,
        );
        let mut current_block_115: u64;
        if tmp___13 != 0 as libc::c_int {
            current_block_115 = 13489891472770870993;
        } else {
            tmp___14 = strcmp(
                ps[3 as libc::c_int as usize] as *const libc::c_char,
                tmptype.as_mut_ptr() as *const libc::c_char,
            );
            if tmp___14 != 0 as libc::c_int {
                current_block_115 = 13489891472770870993;
            } else {
                current_block_115 = 14155412868135599428;
            }
        }
        match current_block_115 {
            13489891472770870993 => {
                tmp___8 = strcmp(
                    tmptype.as_mut_ptr() as *const libc::c_char,
                    b"NS\0" as *const u8 as *const libc::c_char,
                );
                if tmp___8 == 0 as libc::c_int {
                    type_0 = 2 as libc::c_int;
                } else {
                    tmp___7 = strcmp(
                        tmptype.as_mut_ptr() as *const libc::c_char,
                        b"A\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___7 == 0 as libc::c_int {
                        type_0 = 1 as libc::c_int;
                    } else {
                        tmp___6 = strcmp(
                            tmptype.as_mut_ptr() as *const libc::c_char,
                            b"AAAA\0" as *const u8 as *const libc::c_char,
                        );
                        if tmp___6 == 0 as libc::c_int {
                            type_0 = 28 as libc::c_int;
                        }
                    }
                }
                tmp___9 = strcmp(
                    tmptype.as_mut_ptr() as *const libc::c_char,
                    b"CNAME\0" as *const u8 as *const libc::c_char,
                );
                if tmp___9 == 0 as libc::c_int {
                    type_0 = 5 as libc::c_int;
                }
                tmp___10 = strlen(tmpdomain.as_mut_ptr() as *const libc::c_char);
                dlen = tmp___10.wrapping_add(1 as libc::c_ulong) as libc::c_int;
                if dlen > 1 as libc::c_int {
                    str_to_len_label(tmpdomain.as_mut_ptr(), dlen);
                    check_dns_name(tmpdomain.as_mut_ptr(), &mut lowerdomain);
                    insert_kv_mem(
                        rbt,
                        ds,
                        kbuffer,
                        dlen,
                        type_0,
                        vbuffer.as_mut_ptr(),
                        ((*mv).len as libc::c_ulong)
                            .wrapping_add(
                                ::std::mem::size_of::<mvalue>() as libc::c_ulong,
                            ) as libc::c_int,
                        hijack___0,
                        &mut lowerdomain,
                    );
                }
                tmp___11 = strlen(ps[3 as libc::c_int as usize] as *const libc::c_char);
                memcpy(
                    tmptype.as_mut_ptr() as *mut libc::c_void,
                    ps[3 as libc::c_int as usize] as *const libc::c_void,
                    tmp___11.wrapping_add(1 as libc::c_ulong),
                );
                tmp___12 = strlen(ps[0 as libc::c_int as usize] as *const libc::c_char);
                memcpy(
                    tmpdomain.as_mut_ptr() as *mut libc::c_void,
                    ps[0 as libc::c_int as usize] as *const libc::c_void,
                    tmp___12.wrapping_add(1 as libc::c_ulong),
                );
                vitor = vbuffer
                    .as_mut_ptr()
                    .offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize);
                (*mv).num = 0 as libc::c_int as uint16_t;
                (*mv).ttl = 0 as libc::c_int as uint32_t;
                (*mv).len = 0 as libc::c_int as uint16_t;
                (*mv).seg = 0 as libc::c_int as uint16_t;
            }
            _ => {}
        }
        if ttl > (*mv).ttl {
            (*mv).ttl = ttl;
        }
        tmp___18 = strcmp(
            ps[3 as libc::c_int as usize] as *const libc::c_char,
            b"NS\0" as *const u8 as *const libc::c_char,
        );
        let mut current_block_148: u64;
        if tmp___18 == 0 as libc::c_int {
            current_block_148 = 12027190957106789002;
        } else {
            tmp___19 = strcmp(
                ps[3 as libc::c_int as usize] as *const libc::c_char,
                b"CNAME\0" as *const u8 as *const libc::c_char,
            );
            if tmp___19 == 0 as libc::c_int {
                current_block_148 = 12027190957106789002;
            } else {
                tmp___17 = strcmp(
                    ps[3 as libc::c_int as usize] as *const libc::c_char,
                    b"A\0" as *const u8 as *const libc::c_char,
                );
                if tmp___17 == 0 as libc::c_int {
                    str_to_uchar4(
                        ps[4 as libc::c_int as usize] as *const libc::c_char,
                        ipv4.as_mut_ptr(),
                    );
                    memcpy(
                        vitor as *mut libc::c_void,
                        ipv4.as_mut_ptr() as *const libc::c_void,
                        4 as libc::c_int as size_t,
                    );
                    vitor = vitor.offset(4 as libc::c_int as isize);
                    (*mv)
                        .len = ((*mv).len as libc::c_int + 4 as libc::c_int) as uint16_t;
                    (*mv)
                        .num = ((*mv).num as libc::c_int + 1 as libc::c_int) as uint16_t;
                } else {
                    tmp___16 = strcmp(
                        ps[3 as libc::c_int as usize] as *const libc::c_char,
                        b"AAAA\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___16 == 0 as libc::c_int {
                        str_to_uchar6(ps[4 as libc::c_int as usize], ipv6.as_mut_ptr());
                        memcpy(
                            vitor as *mut libc::c_void,
                            ipv6.as_mut_ptr() as *const libc::c_void,
                            16 as libc::c_int as size_t,
                        );
                        vitor = vitor.offset(16 as libc::c_int as isize);
                        (*mv)
                            .len = ((*mv).len as libc::c_int + 16 as libc::c_int)
                            as uint16_t;
                        (*mv)
                            .num = ((*mv).num as libc::c_int + 1 as libc::c_int)
                            as uint16_t;
                    }
                }
                current_block_148 = 3921975509081277429;
            }
        }
        match current_block_148 {
            12027190957106789002 => {
                tmp___15 = strlen(ps[4 as libc::c_int as usize] as *const libc::c_char);
                str_to_len_label(
                    ps[4 as libc::c_int as usize],
                    tmp___15.wrapping_add(1 as libc::c_ulong) as libc::c_int,
                );
                tmplen = check_dns_name(ps[4 as libc::c_int as usize], &mut lowerns);
                if tmplen > 0 as libc::c_int {
                    memcpy(
                        vitor as *mut libc::c_void,
                        (lowerns.domain).as_mut_ptr() as *const libc::c_void,
                        tmplen as size_t,
                    );
                    vitor = vitor.offset(tmplen as isize);
                    (*mv).len = ((*mv).len as libc::c_int + tmplen) as uint16_t;
                    (*mv)
                        .num = ((*mv).num as libc::c_int + 1 as libc::c_int) as uint16_t;
                }
            }
            _ => {}
        }
    }
    tmp___21 = strcmp(
        tmptype.as_mut_ptr() as *const libc::c_char,
        b"NS\0" as *const u8 as *const libc::c_char,
    );
    if tmp___21 == 0 as libc::c_int {
        type_0 = 2 as libc::c_int;
    }
    tmp___22 = strcmp(
        tmptype.as_mut_ptr() as *const libc::c_char,
        b"A\0" as *const u8 as *const libc::c_char,
    );
    if tmp___22 == 0 as libc::c_int {
        type_0 = 1 as libc::c_int;
    }
    tmp___23 = strcmp(
        tmptype.as_mut_ptr() as *const libc::c_char,
        b"AAAA\0" as *const u8 as *const libc::c_char,
    );
    if tmp___23 == 0 as libc::c_int {
        type_0 = 28 as libc::c_int;
    }
    tmp___24 = strlen(tmpdomain.as_mut_ptr() as *const libc::c_char);
    dlen = tmp___24.wrapping_add(1 as libc::c_ulong) as libc::c_int;
    if dlen > 1 as libc::c_int {
        str_to_len_label(tmpdomain.as_mut_ptr(), dlen);
        check_dns_name(tmpdomain.as_mut_ptr(), &mut lowerdomain);
        insert_kv_mem(
            rbt,
            ds,
            kbuffer,
            dlen,
            type_0,
            vbuffer.as_mut_ptr(),
            ((*mv).len as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<mvalue>() as libc::c_ulong)
                as libc::c_int,
            hijack___0,
            &mut lowerdomain,
        );
    }
    fclose(fd);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn read_root(
    mut ds: *mut htable,
    mut rbt: *mut rbtree,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = read_records_from_file(
        b"../root.z\0" as *const u8 as *const libc::c_char,
        ds,
        rbt,
        0 as libc::c_int,
    );
    return tmp;
}
pub unsafe extern "C" fn refresh_records(
    mut ds: *mut htable,
    mut rbt: *mut rbtree,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    printf(b"read from records.z\n\0" as *const u8 as *const libc::c_char);
    tmp = read_records_from_file(
        b"../records.z\0" as *const u8 as *const libc::c_char,
        ds,
        rbt,
        1 as libc::c_int,
    );
    return tmp;
}
pub unsafe extern "C" fn create_transfer_point(
    mut name: *mut uchar,
    mut fwd: *mut htable,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut dlen: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut ipv4: [uchar; 4] = [0; 4];
    let mut tmp: libc::c_uint = 0;
    let mut addr: *mut uchar = 0 as *mut uchar;
    let mut itor: *mut uchar = 0 as *mut uchar;
    let mut vbuffer: [uchar; 1000] = [0; 1000];
    let mut tmp___0: libc::c_uint = 0;
    let mut v: *mut uchar = 0 as *mut uchar;
    let mut hash: hashval_t = 0;
    let mut tmp___1: size_t = 0;
    let mut mv: *mut mvalue = 0 as *mut mvalue;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    i = -(1 as libc::c_int);
    ipv4[0 as libc::c_int as usize] = 0 as libc::c_int as uchar;
    tmp = 1 as libc::c_uint;
    while !(tmp >= 4 as libc::c_uint) {
        ipv4[tmp as usize] = 0 as libc::c_int as libc::c_uchar;
        tmp = tmp.wrapping_add(1);
    }
    addr = 0 as *mut libc::c_void as *mut uchar;
    vbuffer[0 as libc::c_int as usize] = 0 as libc::c_int as uchar;
    tmp___0 = 1 as libc::c_uint;
    while !(tmp___0 >= 1000 as libc::c_uint) {
        vbuffer[tmp___0 as usize] = 0 as libc::c_int as libc::c_uchar;
        tmp___0 = tmp___0.wrapping_add(1);
    }
    v = 0 as *mut libc::c_void as *mut uchar;
    hash = 0 as libc::c_int as hashval_t;
    tmp___1 = strlen(name as *const libc::c_char);
    dlen = tmp___1.wrapping_add(1 as libc::c_ulong) as libc::c_int;
    str_to_len_label(name, dlen);
    addr = name.offset(dlen as isize);
    mv = vbuffer.as_mut_ptr() as *mut mvalue;
    (*mv).num = 0 as libc::c_int as uint16_t;
    (*mv).ttl = 604801 as libc::c_int as uint32_t;
    (*mv).len = 0 as libc::c_int as uint16_t;
    itor = vbuffer
        .as_mut_ptr()
        .offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize);
    i = 0 as libc::c_int;
    while i < n {
        str_to_uchar4(addr as *const libc::c_char, ipv4.as_mut_ptr());
        memcpy(
            itor as *mut libc::c_void,
            ipv4.as_mut_ptr() as *const libc::c_void,
            4 as libc::c_int as size_t,
        );
        tmp___2 = strlen(addr as *const libc::c_char);
        addr = addr.offset(tmp___2 as isize).offset(1 as libc::c_int as isize);
        itor = itor.offset(4 as libc::c_int as isize);
        (*mv).len = ((*mv).len as libc::c_int + 4 as libc::c_int) as uint16_t;
        (*mv).num = ((*mv).num as libc::c_int + 1 as libc::c_int) as uint16_t;
        if *addr.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
            break;
        }
        i += 1;
    }
    tmp___3 = malloc(
        ((*mv).len as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<mvalue>() as libc::c_ulong),
    );
    v = tmp___3 as *mut uchar;
    memcpy(
        v as *mut libc::c_void,
        vbuffer.as_mut_ptr() as *const libc::c_void,
        ((*mv).len as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<mvalue>() as libc::c_ulong),
    );
    ret = htable_insert(
        fwd,
        name,
        dlen,
        1 as libc::c_int,
        v,
        0 as libc::c_int,
        0 as *mut libc::c_void as *mut mvalue,
        &mut hash,
    );
    if !(ret == 0 as libc::c_int) {
        __assert_fail(
            b"ret == HTABLE_INSERT_RET_NORMAL\0" as *const u8 as *const libc::c_char,
            b"io.c\0" as *const u8 as *const libc::c_char,
            241 as libc::c_uint,
            b"create_transfer_point\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn read_resolve(
    mut fd: *mut FILE,
    mut nameservers: *mut *mut libc::c_char,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut tmp: libc::c_uint = 0;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut placeholder: [libc::c_char; 128] = [0; 128];
    let mut tmp___1: libc::c_uint = 0;
    let mut temp: [libc::c_char; 32] = [0; 32];
    let mut tmp___2: libc::c_uint = 0;
    let mut tmp___3: size_t = 0;
    let mut tmp___4: size_t = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    buf[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    tmp = 1 as libc::c_uint;
    while !(tmp >= 1024 as libc::c_uint) {
        buf[tmp as usize] = 0 as libc::c_int as libc::c_char;
        tmp = tmp.wrapping_add(1);
    }
    tmp___0 = 0 as *mut libc::c_void as *mut libc::c_char;
    i = 0 as libc::c_int;
    placeholder[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    tmp___1 = 1 as libc::c_uint;
    while !(tmp___1 >= 128 as libc::c_uint) {
        placeholder[tmp___1 as usize] = 0 as libc::c_int as libc::c_char;
        tmp___1 = tmp___1.wrapping_add(1);
    }
    temp[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    tmp___2 = 1 as libc::c_uint;
    while !(tmp___2 >= 32 as libc::c_uint) {
        temp[tmp___2 as usize] = 0 as libc::c_int as libc::c_char;
        tmp___2 = tmp___2.wrapping_add(1);
    }
    if fd as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int)
    } else {
        if n <= 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    i = 0 as libc::c_int;
    loop {
        tmp___6 = fgets(buf.as_mut_ptr(), 1024 as libc::c_int, fd);
        if !(tmp___6 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
            break;
        }
        fix_tail(buf.as_mut_ptr());
        if buf[0 as libc::c_int as usize] as libc::c_int == 58 as libc::c_int {
            break;
        }
        tmp___0 = strstr(
            buf.as_mut_ptr() as *const libc::c_char,
            b"nameserver\0" as *const u8 as *const libc::c_char,
        );
        if tmp___0.is_null() {
            continue;
        }
        if i + 1 as libc::c_int > n {
            continue;
        }
        memset(
            placeholder.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            128 as libc::c_int as size_t,
        );
        memset(
            temp.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            32 as libc::c_int as size_t,
        );
        sscanf(
            buf.as_mut_ptr() as *const libc::c_char,
            b"%s %s\0" as *const u8 as *const libc::c_char,
            placeholder.as_mut_ptr(),
            temp.as_mut_ptr(),
        );
        tmp___3 = strlen(temp.as_mut_ptr() as *const libc::c_char);
        if tmp___3 > 15 as libc::c_ulong {
            continue;
        }
        tmp___4 = strlen(temp.as_mut_ptr() as *const libc::c_char);
        if tmp___4 < 7 as libc::c_ulong {
            continue;
        }
        tmp___5 = i;
        i += 1;
        let ref mut fresh9 = *nameservers.offset(tmp___5 as isize);
        *fresh9 = strdup(temp.as_mut_ptr() as *const libc::c_char);
    }
    return i;
}
pub unsafe extern "C" fn read_logpath(
    mut fd: *mut FILE,
    mut path: *mut libc::c_char,
) -> libc::c_int {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: libc::c_int = 0;
    tmp = fgets(path, 512 as libc::c_int, fd);
    if tmp as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        memcpy(
            path as *mut libc::c_void,
            b"/var/dnspod-sr/log/\0" as *const u8 as *const libc::c_char
                as *const libc::c_void,
            20 as libc::c_int as size_t,
        );
    }
    fix_tail(path);
    tmp___1 = mkdir(path as *const libc::c_char, 493 as libc::c_int as __mode_t);
    if tmp___1 != 0 as libc::c_int {
        tmp___0 = __errno_location();
        if *tmp___0 == 17 as libc::c_int {
            return 0 as libc::c_int
        } else {
            dns_error(
                0 as libc::c_int,
                b"create log parent dir failed\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn read_transfer(
    mut fd: *mut FILE,
    mut fwd: *mut htable,
) -> libc::c_int {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut tmp: libc::c_uint = 0;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    buf[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    tmp = 1 as libc::c_uint;
    while !(tmp >= 1024 as libc::c_uint) {
        buf[tmp as usize] = 0 as libc::c_int as libc::c_char;
        tmp = tmp.wrapping_add(1);
    }
    tmp___0 = 0 as *mut libc::c_void as *mut libc::c_char;
    if fd as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int)
    } else {
        if fwd as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            return -(1 as libc::c_int);
        }
    }
    loop {
        tmp___1 = fgets(buf.as_mut_ptr(), 1024 as libc::c_int, fd);
        if !(tmp___1 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
            break;
        }
        fix_tail(buf.as_mut_ptr());
        if buf[0 as libc::c_int as usize] as libc::c_int == 58 as libc::c_int {
            break;
        }
        tmp___0 = strstr(
            buf.as_mut_ptr() as *const libc::c_char,
            b":\0" as *const u8 as *const libc::c_char,
        );
        if tmp___0 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            *tmp___0
                .offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
            tmp___0 = tmp___0.offset(1);
            n = 1 as libc::c_int;
            i = 0 as libc::c_int;
            while i < 8 as libc::c_int {
                tmp___0 = strstr(
                    tmp___0 as *const libc::c_char,
                    b",\0" as *const u8 as *const libc::c_char,
                );
                if tmp___0 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                    break;
                }
                n += 1;
                *tmp___0
                    .offset(
                        0 as libc::c_int as isize,
                    ) = 0 as libc::c_int as libc::c_char;
                tmp___0 = tmp___0.offset(1);
                i += 1;
            }
            if i != 8 as libc::c_int {
                create_transfer_point(buf.as_mut_ptr() as *mut uchar, fwd, n);
            }
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn read_config(
    mut fn_0: *const libc::c_char,
    mut logpath: *mut libc::c_char,
    mut forward: *mut htable,
    mut nameservers: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut fd: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut tmp: libc::c_uint = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    fd = 0 as *mut libc::c_void as *mut FILE;
    buf[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    tmp = 1 as libc::c_uint;
    while !(tmp >= 1024 as libc::c_uint) {
        buf[tmp as usize] = 0 as libc::c_int as libc::c_char;
        tmp = tmp.wrapping_add(1);
    }
    if fn_0 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    fd = fopen(fn_0, b"r\0" as *const u8 as *const libc::c_char);
    if fd as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    loop {
        tmp___3 = fgets(buf.as_mut_ptr(), 1024 as libc::c_int, fd);
        if !(tmp___3 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
            break;
        }
        fix_tail(buf.as_mut_ptr());
        tmp___0 = strcmp(
            buf.as_mut_ptr() as *const libc::c_char,
            b"xfer:\0" as *const u8 as *const libc::c_char,
        );
        if tmp___0 == 0 as libc::c_int {
            read_transfer(fd, forward);
        } else {
            tmp___1 = strcmp(
                buf.as_mut_ptr() as *const libc::c_char,
                b"log_path:\0" as *const u8 as *const libc::c_char,
            );
            if tmp___1 == 0 as libc::c_int {
                read_logpath(fd, logpath);
            } else {
                tmp___2 = strcmp(
                    buf.as_mut_ptr() as *const libc::c_char,
                    b"resolve:\0" as *const u8 as *const libc::c_char,
                );
                if !(tmp___2 == 0 as libc::c_int) {
                    continue;
                }
                read_resolve(fd, nameservers, 2 as libc::c_int);
            }
        }
    }
    fclose(fd);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn fill_domain_to_len_label(
    mut from: *const libc::c_char,
    mut to: *mut libc::c_char,
) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut itor: *const libc::c_char = 0 as *const libc::c_char;
    len = 0 as libc::c_int;
    itor = from;
    if *itor.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
        *to.offset(0 as libc::c_int as isize) = '.' as i32 as libc::c_char;
        return 1 as libc::c_int;
    }
    while *itor.offset(0 as libc::c_int as isize) as libc::c_int != 0 as libc::c_int {
        memcpy(
            to as *mut libc::c_void,
            itor.offset(1 as libc::c_int as isize) as *const libc::c_void,
            *itor.offset(0 as libc::c_int as isize) as size_t,
        );
        to = to.offset(*itor.offset(0 as libc::c_int as isize) as libc::c_int as isize);
        len += *itor.offset(0 as libc::c_int as isize) as libc::c_int;
        itor = itor
            .offset(*itor.offset(0 as libc::c_int as isize) as libc::c_int as isize)
            .offset(1 as libc::c_int as isize);
        *to.offset(0 as libc::c_int as isize) = '.' as i32 as libc::c_char;
        to = to.offset(1);
        len += 1;
    }
    return len;
}
pub unsafe extern "C" fn write_loginfo_into_file(
    mut log: *mut log_info,
    mut domain: *const uchar,
    mut dlen: libc::c_int,
    mut type_0: libc::c_int,
    mut addr: *mut sockaddr_in,
) -> libc::c_int {
    let mut tp: uchar = 0;
    let mut fd: libc::c_int = 0;
    let mut tmplen: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut tmp: ssize_t = 0;
    tp = (type_0 % 256 as libc::c_int) as uchar;
    fd = (*log).logfd;
    tmplen = 0 as libc::c_int;
    ret = 0 as libc::c_int;
    if fd <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if domain as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmplen = 8 as libc::c_int + dlen;
        if (*log).log_cache_cursor + tmplen >= 1048576 as libc::c_int {
            tmp = write(
                (*log).logfd,
                ((*log).log_cache).as_mut_ptr() as *const libc::c_void,
                (*log).log_cache_cursor as size_t,
            );
            ret = tmp as libc::c_int;
            if ret == -(1 as libc::c_int) {
                perror(b"write\0" as *const u8 as *const libc::c_char);
            }
            (*log).log_cache_cursor = 0 as libc::c_int;
        }
        if tmplen >= 1048576 as libc::c_int {
            return 0 as libc::c_int
        } else {
            (*log)
                .log_cache[(*log).log_cache_cursor
                as usize] = 255 as libc::c_int as uchar;
            (*log).log_cache[((*log).log_cache_cursor + 1 as libc::c_int) as usize] = tp;
            (*log)
                .log_cache[((*log).log_cache_cursor + 2 as libc::c_int)
                as usize] = 2 as libc::c_int as uchar;
            memcpy(
                ((*log).log_cache)
                    .as_mut_ptr()
                    .offset((*log).log_cache_cursor as isize)
                    .offset(3 as libc::c_int as isize) as *mut libc::c_void,
                &mut (*addr).sin_addr.s_addr as *mut in_addr_t as *const libc::c_void,
                ::std::mem::size_of::<in_addr_t>() as libc::c_ulong,
            );
            memcpy(
                ((*log).log_cache)
                    .as_mut_ptr()
                    .offset((*log).log_cache_cursor as isize)
                    .offset(3 as libc::c_int as isize)
                    .offset(::std::mem::size_of::<in_addr_t>() as libc::c_ulong as isize)
                    as *mut libc::c_void,
                domain as *const libc::c_void,
                dlen as size_t,
            );
            (*log).log_cache_cursor += tmplen;
            (*log)
                .log_cache[((*log).log_cache_cursor - 1 as libc::c_int)
                as usize] = '#' as i32 as uchar;
        }
    }
    return 0 as libc::c_int;
}
static mut pf: [libc::c_char; 50] = [
    0 as libc::c_int as libc::c_char,
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
    0,
];
pub unsafe extern "C" fn create_new_log(
    mut prefix: *mut uchar,
    mut idx: libc::c_int,
    mut type_0: libc::c_int,
) -> libc::c_int {
    let mut filename: [libc::c_char; 80] = [0; 80];
    let mut tmp: libc::c_uint = 0;
    let mut final_0: [libc::c_char; 130] = [0; 130];
    let mut tmp___0: libc::c_uint = 0;
    let mut fd: libc::c_int = 0;
    let mut bit: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut mode: mode_t = 0;
    let mut prev: time_t = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: size_t = 0;
    let mut tmp___4: size_t = 0;
    filename[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    tmp = 1 as libc::c_uint;
    while !(tmp >= 80 as libc::c_uint) {
        filename[tmp as usize] = 0 as libc::c_int as libc::c_char;
        tmp = tmp.wrapping_add(1);
    }
    final_0[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    tmp___0 = 1 as libc::c_uint;
    while !(tmp___0 >= 130 as libc::c_uint) {
        final_0[tmp___0 as usize] = 0 as libc::c_int as libc::c_char;
        tmp___0 = tmp___0.wrapping_add(1);
    }
    fd = -(1 as libc::c_int);
    mode = (384 as libc::c_int | 256 as libc::c_int >> 3 as libc::c_int
        | 256 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as mode_t;
    if pf[0 as libc::c_int as usize] as libc::c_int == 0 as libc::c_int {
        tmp___1 = strlen(prefix as *const libc::c_char);
        memcpy(
            pf.as_mut_ptr() as *mut libc::c_void,
            prefix as *const libc::c_void,
            tmp___1.wrapping_add(1 as libc::c_ulong),
        );
    }
    filename[0 as libc::c_int as usize] = 'f' as i32 as libc::c_char;
    if type_0 != 233 as libc::c_int {
        if type_0 != 112 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    if type_0 == 233 as libc::c_int {
        filename[0 as libc::c_int as usize] = 'q' as i32 as libc::c_char;
    }
    bit = idx / 100 as libc::c_int;
    filename[1 as libc::c_int as usize] = (bit + 48 as libc::c_int) as libc::c_char;
    bit = idx % 100 as libc::c_int / 10 as libc::c_int;
    filename[2 as libc::c_int as usize] = (bit + 48 as libc::c_int) as libc::c_char;
    bit = idx % 10 as libc::c_int;
    filename[3 as libc::c_int as usize] = (bit + 48 as libc::c_int) as libc::c_char;
    prev = global_now - global_now % 900 as libc::c_long;
    sprintf(
        filename.as_mut_ptr().offset(4 as libc::c_int as isize),
        b"%lu\0" as *const u8 as *const libc::c_char,
        prev,
    );
    tmp___2 = strlen(filename.as_mut_ptr() as *const libc::c_char);
    memcpy(
        filename.as_mut_ptr().offset(tmp___2 as isize) as *mut libc::c_void,
        b".log\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        5 as libc::c_int as size_t,
    );
    tmp___3 = strlen(pf.as_mut_ptr() as *const libc::c_char);
    len = tmp___3 as libc::c_int;
    memcpy(
        final_0.as_mut_ptr() as *mut libc::c_void,
        pf.as_mut_ptr() as *const libc::c_void,
        len as size_t,
    );
    tmp___4 = strlen(filename.as_mut_ptr() as *const libc::c_char);
    memcpy(
        final_0.as_mut_ptr().offset(len as isize) as *mut libc::c_void,
        filename.as_mut_ptr() as *const libc::c_void,
        tmp___4.wrapping_add(1 as libc::c_ulong),
    );
    fd = open(final_0.as_mut_ptr() as *const libc::c_char, 65 as libc::c_int, mode);
    return fd;
}
pub unsafe extern "C" fn write_log(
    mut log: *mut log_info,
    mut idx: libc::c_int,
    mut domain: *const uchar,
    mut dlen: libc::c_int,
    mut type_0: libc::c_int,
    mut addr: *mut sockaddr_in,
) -> libc::c_int {
    let mut lfd: libc::c_int = 0;
    add_query_info((*log).log_type, idx, type_0 as uint16_t);
    lfd = (*log).logfd;
    if global_now % 900 as libc::c_long == 0 as libc::c_long {
        if global_now > (*log).lastlog {
            close(lfd);
            lfd = create_new_log(
                0 as *mut libc::c_void as *mut uchar,
                idx,
                (*log).log_type,
            );
            (*log).logfd = lfd;
        }
    }
    write_loginfo_into_file(log, domain, dlen, type_0, addr);
    (*log).lastlog = global_now;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn create_event(mut size: libc::c_int) -> *mut event {
    let mut ev: *mut event = 0 as *mut event;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut epfd: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(
        (::std::mem::size_of::<event>() as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<event_data>() as libc::c_ulong)
                    .wrapping_mul(size as libc::c_ulong),
            ),
    );
    ev = tmp as *mut event;
    tmp___0 = epoll_create(size);
    epfd = tmp___0;
    if epfd == 0 as libc::c_int {
        dns_error(
            0 as libc::c_int,
            b"epoll create\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    (*ev).size = size;
    tmp___1 = malloc(
        (::std::mem::size_of::<iner_event>() as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<epoll_event>() as libc::c_ulong)
                    .wrapping_mul((*ev).size as libc::c_ulong),
            ),
    );
    (*ev).ie = tmp___1 as *mut iner_event;
    if (*ev).ie as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        dns_error(
            0 as libc::c_int,
            b"alloc iner event\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    memset(
        (*ev).ie as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<iner_event>() as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<epoll_event>() as libc::c_ulong)
                    .wrapping_mul((*ev).size as libc::c_ulong),
            ),
    );
    (*(*ev).ie).epfd = epfd;
    return ev;
}
pub unsafe extern "C" fn add_event(
    mut ev: *mut event,
    mut help___0: *mut event_help,
) -> libc::c_int {
    let mut e: epoll_event = epoll_event {
        events: 0,
        data: epoll_data {
            ptr: 0 as *mut libc::c_void,
        },
    };
    let mut ret: libc::c_int = 0;
    let mut epfd: libc::c_int = 0;
    e.events = 0 as libc::c_int as uint32_t;
    e.data.ptr = 0 as *mut libc::c_void;
    ret = 0 as libc::c_int;
    epfd = (*(*ev).ie).epfd;
    e.data.fd = (*help___0).fd;
    if e.data.fd < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*help___0).type_0 as libc::c_uint == 1 as libc::c_uint {
        e.events = 1 as libc::c_int as uint32_t;
    }
    if (*help___0).type_0 as libc::c_uint == 2 as libc::c_uint {
        e.events = 4 as libc::c_int as uint32_t;
    }
    let ref mut fresh10 = (*((*ev).data).as_mut_ptr().offset((*help___0).fd as isize))
        .cb;
    *fresh10 = (*help___0).cb;
    if (*help___0).ext as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        let ref mut fresh11 = (*((*ev).data)
            .as_mut_ptr()
            .offset((*help___0).fd as isize))
            .ext;
        *fresh11 = (*help___0).ext;
    }
    ret = epoll_ctl(epfd, 1 as libc::c_int, (*help___0).fd, &mut e);
    if ret < 0 as libc::c_int {
        printf(b"fd is %d\n\0" as *const u8 as *const libc::c_char, (*help___0).fd);
        perror(b"epoll_ctl\0" as *const u8 as *const libc::c_char);
    }
    return ret;
}
pub unsafe extern "C" fn del_event(
    mut ev: *mut event,
    mut help___0: *mut event_help,
) -> libc::c_int {
    let mut e: epoll_event = epoll_event {
        events: 0,
        data: epoll_data {
            ptr: 0 as *mut libc::c_void,
        },
    };
    let mut ie: *mut iner_event = 0 as *mut iner_event;
    let mut ret: libc::c_int = 0;
    ie = (*ev).ie;
    ret = 0 as libc::c_int;
    e.data.fd = (*help___0).fd;
    ret = epoll_ctl((*ie).epfd, 2 as libc::c_int, (*help___0).fd, &mut e);
    return ret;
}
pub unsafe extern "C" fn handle_event(
    mut ev: *mut event,
    mut to: libc::c_int,
) -> libc::c_int {
    let mut num: libc::c_int = 0;
    let mut ie: *mut iner_event = 0 as *mut iner_event;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    num = 0 as libc::c_int;
    ie = (*ev).ie;
    if to == 0 as libc::c_int {
        to = -(1 as libc::c_int);
    } else {
        to *= 100 as libc::c_int;
    }
    (*ev).size = 100 as libc::c_int;
    loop {
        num = epoll_wait((*ie).epfd, ((*ie).e).as_mut_ptr(), (*ev).size, to);
        if num >= 0 as libc::c_int {
            break;
        }
        if !(num < 0 as libc::c_int) {
            continue;
        }
        tmp = __errno_location();
        *tmp == 4 as libc::c_int;
    }
    return num;
}
pub unsafe extern "C" fn cb_get_tcp_msg(
    mut data: *mut event_data,
    mut v: *mut libc::c_void,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut szhdr: libc::c_int = 0;
    let mut mc: *mut msgcache = 0 as *mut msgcache;
    let mut f: *mut fetcher = 0 as *mut fetcher;
    let mut mbuf: *mut mbuf_type = 0 as *mut mbuf_type;
    szhdr = ::std::mem::size_of::<dnsheader>() as libc::c_ulong as libc::c_int;
    f = v as *mut fetcher;
    mbuf = mbuf_alloc();
    if 0 as *mut libc::c_void as libc::c_ulong == mbuf as libc::c_ulong {
        return 0 as libc::c_int;
    }
    memset(
        mbuf as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mbuf_type>() as libc::c_ulong,
    );
    mc = (*f.offset(idx as isize)).mc;
    pthread_spin_lock(&mut (*mc).lock);
    if ((*mc).tail).wrapping_add(8 as libc::c_ulong) > (*mc).size as uint64_t {
        (*mc).tail = 0 as libc::c_int as uint64_t;
    }
    if ((*mc).tail).wrapping_add(8 as libc::c_ulong) > (*mc).head {
        if (*mc).tail < (*mc).head {
            close((*data).fd);
            let ref mut fresh12 = (*f.offset(idx as isize)).miss;
            *fresh12 = (*fresh12).wrapping_add(1);
            pthread_spin_unlock(&mut (*mc).lock);
            mbuf_free(mbuf);
            return 0 as libc::c_int;
        }
    }
    if (*mc).tail == (*mc).head {
        if (*mc).pkt != 0 as libc::c_uint {
            close((*data).fd);
            let ref mut fresh13 = (*f.offset(idx as isize)).miss;
            *fresh13 = (*fresh13).wrapping_add(1);
            pthread_spin_unlock(&mut (*mc).lock);
            mbuf_free(mbuf);
            return 0 as libc::c_int;
        }
    }
    let ref mut fresh14 = (*f.offset(idx as isize)).pkg;
    *fresh14 = (*fresh14).wrapping_add(1);
    (*mbuf).socktype = 1 as libc::c_int as uint___0;
    (*mbuf).fd = (*data).fd;
    (*mbuf).buf = ((*mbuf).data).as_mut_ptr();
    (*mbuf).buflen = 4096 as libc::c_int;
    ret = tcp_read_dns_msg(mbuf, 4096 as libc::c_int as uint___0, 0 as libc::c_int);
    if ret < szhdr {
        pthread_spin_unlock(&mut (*mc).lock);
        mbuf_free(mbuf);
        return -(1 as libc::c_int);
    }
    (*mbuf).fetch_len = ret as uint___0;
    memcpy(
        ((*mc).data).as_mut_ptr().offset((*mc).tail as isize) as *mut libc::c_void,
        &mut mbuf as *mut *mut mbuf_type as *const libc::c_void,
        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
    );
    (*mc)
        .tail = ((*mc).tail as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
        as uint64_t as uint64_t;
    if ((*mc).tail).wrapping_add(8 as libc::c_ulong) > (*mc).size as uint64_t {
        (*mc).tail = 0 as libc::c_int as uint64_t;
    }
    (*mc).pkt = ((*mc).pkt).wrapping_add(1);
    pthread_spin_unlock(&mut (*mc).lock);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn fake_recv(
    mut data: *mut event_data,
    mut v: *mut libc::c_void,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut f: *mut fetcher = 0 as *mut fetcher;
    let mut addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut buffer: [uchar; 512] = [0; 512];
    let mut tmp: libc::c_uint = 0;
    let mut ret: libc::c_int = 0;
    let mut len: socklen_t = 0;
    let mut tmp___0: ssize_t = 0;
    f = v as *mut fetcher;
    buffer[0 as libc::c_int as usize] = 0 as libc::c_int as uchar;
    tmp = 1 as libc::c_uint;
    while !(tmp >= 512 as libc::c_uint) {
        buffer[tmp as usize] = 0 as libc::c_int as libc::c_uchar;
        tmp = tmp.wrapping_add(1);
    }
    idx = 0 as libc::c_int;
    len = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    loop {
        tmp___0 = recvfrom(
            (*data).fd,
            buffer.as_mut_ptr() as *mut libc::c_void,
            512 as libc::c_int as size_t,
            0 as libc::c_int,
            &mut addr as *mut sockaddr_in as *mut SA as *mut sockaddr,
            &mut len as *mut socklen_t,
        );
        ret = tmp___0 as libc::c_int;
        if ret > 0 as libc::c_int {
            let ref mut fresh15 = (*f.offset(idx as isize)).pkg;
            *fresh15 = (*fresh15).wrapping_add(1);
        } else {
            return 0 as libc::c_int
        }
    };
}
pub unsafe extern "C" fn cb_get_udp_msg(
    mut data: *mut event_data,
    mut v: *mut libc::c_void,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut szhdr: libc::c_int = 0;
    let mut mc: *mut msgcache = 0 as *mut msgcache;
    let mut f: *mut fetcher = 0 as *mut fetcher;
    let mut mbuf: *mut mbuf_type = 0 as *mut mbuf_type;
    szhdr = ::std::mem::size_of::<dnsheader>() as libc::c_ulong as libc::c_int;
    mc = 0 as *mut libc::c_void as *mut msgcache;
    f = v as *mut fetcher;
    loop {
        mbuf = mbuf_alloc();
        if 0 as *mut libc::c_void as libc::c_ulong == mbuf as libc::c_ulong {
            return 0 as libc::c_int;
        }
        memset(
            mbuf as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<mbuf_type>() as libc::c_ulong,
        );
        mc = (*f.offset(idx as isize)).mc;
        pthread_spin_lock(&mut (*mc).lock);
        if ((*mc).tail).wrapping_add(8 as libc::c_ulong) > (*mc).head {
            if (*mc).tail < (*mc).head {
                let ref mut fresh16 = (*f.offset(idx as isize)).miss;
                *fresh16 = (*fresh16).wrapping_add(1);
                pthread_spin_unlock(&mut (*mc).lock);
                mbuf_free(mbuf);
                return 0 as libc::c_int;
            }
        }
        if (*mc).tail == (*mc).head {
            if (*mc).pkt != 0 as libc::c_uint {
                let ref mut fresh17 = (*f.offset(idx as isize)).miss;
                *fresh17 = (*fresh17).wrapping_add(1);
                pthread_spin_unlock(&mut (*mc).lock);
                mbuf_free(mbuf);
                return 0 as libc::c_int;
            }
        }
        let ref mut fresh18 = (*f.offset(idx as isize)).pkg;
        *fresh18 = (*fresh18).wrapping_add(1);
        (*mbuf).socktype = 2 as libc::c_int as uint___0;
        (*mbuf).fd = (*data).fd;
        (*mbuf).buf = ((*mbuf).data).as_mut_ptr().offset(2 as libc::c_int as isize);
        (*mbuf).buflen = 4094 as libc::c_int;
        (*mbuf).addr = &mut (*mbuf).caddr;
        ret = udp_read_msg(mbuf, 0 as libc::c_int);
        if ret < szhdr {
            pthread_spin_unlock(&mut (*mc).lock);
            mbuf_free(mbuf);
            return -(1 as libc::c_int);
        }
        (*mbuf).fetch_len = ret as uint___0;
        memcpy(
            ((*mc).data).as_mut_ptr().offset((*mc).tail as isize) as *mut libc::c_void,
            &mut mbuf as *mut *mut mbuf_type as *const libc::c_void,
            ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
        );
        (*mc)
            .tail = ((*mc).tail as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            as uint64_t as uint64_t;
        if ((*mc).tail).wrapping_add(8 as libc::c_ulong) > (*mc).size as uint64_t {
            (*mc).tail = 0 as libc::c_int as uint64_t;
        }
        (*mc).pkt = ((*mc).pkt).wrapping_add(1);
        pthread_spin_unlock(&mut (*mc).lock);
    };
}
pub unsafe extern "C" fn insert_events(
    mut ev: *mut event,
    mut fd: libc::c_int,
    mut type_0: libc::c_int,
) -> libc::c_int {
    let mut h: event_help = event_help {
        fd: 0,
        spfd: 0,
        num: 0,
        type_0: 0 as event_type,
        to: 0 as *mut timeval,
        cb: None,
        ext: 0 as *mut libc::c_void,
    };
    let mut tmp: libc::c_int = 0;
    if fd > 0 as libc::c_int {
        memset(
            &mut h as *mut event_help as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<event_help>() as libc::c_ulong,
        );
        h.type_0 = ET_READ;
        h.fd = fd;
        if type_0 == 2 as libc::c_int {
            h
                .cb = Some(
                cb_get_udp_msg
                    as unsafe extern "C" fn(
                        *mut event_data,
                        *mut libc::c_void,
                        libc::c_int,
                    ) -> libc::c_int,
            );
        } else {
            h
                .cb = Some(
                cb_get_tcp_msg
                    as unsafe extern "C" fn(
                        *mut event_data,
                        *mut libc::c_void,
                        libc::c_int,
                    ) -> libc::c_int,
            );
        }
        tmp = add_event(ev, &mut h);
        if tmp < 0 as libc::c_int {
            dns_error(
                1 as libc::c_int,
                b"add event notify\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn run_sentinel(mut s: *mut server) -> libc::c_int {
    let mut num: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ls: libc::c_int = 0;
    let mut connfd: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut fidx: libc::c_int = 0;
    let mut addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut h: event_help = event_help {
        fd: 0,
        spfd: 0,
        num: 0,
        type_0: 0 as event_type,
        to: 0 as *mut timeval,
        cb: None,
        ext: 0 as *mut libc::c_void,
    };
    let mut len: socklen_t = 0;
    let mut f: *mut fetcher = 0 as *mut fetcher;
    let mut ev: *mut event = 0 as *mut event;
    let mut tmp: *mut event = 0 as *mut event;
    let mut cpuinfo: cpu_set_t = cpu_set_t { __bits: [0; 16] };
    let mut pt: pthread_t = 0;
    let mut tmp___0: pthread_t = 0;
    let mut __cpu: size_t = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    let mut cb: Option::<
        unsafe extern "C" fn(
            *mut event_data,
            *mut libc::c_void,
            libc::c_int,
        ) -> libc::c_int,
    > = None;
    fidx = 0 as libc::c_int;
    len = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    f = (*s).fetchers;
    tmp = create_event(1000 as libc::c_int);
    ev = tmp;
    tmp___0 = pthread_self();
    pt = tmp___0;
    libc::memset(
        &mut cpuinfo as *mut cpu_set_t as *mut libc::c_void,
        '\u{0}' as i32,
        ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong as libc::c_int
            as libc::c_ulong as libc::size_t,
    );
    __cpu = 0 as libc::c_int as size_t;
    if __cpu.wrapping_div(8 as libc::c_ulong)
        < ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong
    {
        cpuinfo
            .__bits[__cpu
            .wrapping_div(
                (8 as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<__cpu_mask>() as libc::c_ulong),
            ) as usize]
            |= (1 as libc::c_ulong)
                << __cpu
                    .wrapping_rem(
                        (8 as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                            ),
                    );
    }
    tmp___1 = pthread_setaffinity_np(
        pt,
        ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong,
        &mut cpuinfo as *mut cpu_set_t as *const cpu_set_t,
    );
    if 0 as libc::c_int != tmp___1 {
        printf(b"set affinity fetcher\n\0" as *const u8 as *const libc::c_char);
        exit(0 as libc::c_int);
    }
    if ev as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        dns_error(
            0 as libc::c_int,
            b"create event st\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    insert_events(ev, (*s).ludp, 2 as libc::c_int);
    insert_events(ev, (*s).ltcp, 1 as libc::c_int);
    ls = (*s).ltcp;
    loop {
        num = handle_event(ev, 1 as libc::c_int);
        global_cron(s);
        i = 0 as libc::c_int;
        while i < num {
            fd = (*((*(*ev).ie).e).as_mut_ptr().offset(i as isize)).data.fd;
            cb = (*((*ev).data).as_mut_ptr().offset(fd as isize)).cb;
            (*((*ev).data).as_mut_ptr().offset(fd as isize)).fd = fd;
            if fd == ls {
                connfd = accept(
                    fd,
                    &mut addr as *mut sockaddr_in as *mut SA as *mut sockaddr,
                    &mut len as *mut socklen_t,
                );
                set_non_block(connfd);
                insert_events(ev, connfd, 1 as libc::c_int);
            } else if ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut event_data,
                            *mut libc::c_void,
                            libc::c_int,
                        ) -> libc::c_int,
                    >,
                    libc::c_ulong,
                >(cb) != 0 as *mut libc::c_void as libc::c_ulong
                {
                fidx += 1;
                fidx %= 2 as libc::c_int;
                if fidx >= 2 as libc::c_int {
                    fidx = 1 as libc::c_int;
                }
                ret = (Some(cb.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*ev).data).as_mut_ptr().offset(fd as isize),
                    f as *mut libc::c_void,
                    fidx,
                );
                if ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut event_data,
                            *mut libc::c_void,
                            libc::c_int,
                        ) -> libc::c_int,
                    >,
                    libc::c_ulong,
                >(cb)
                    == ::std::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(
                                *mut event_data,
                                *mut libc::c_void,
                                libc::c_int,
                            ) -> libc::c_int,
                        >,
                        libc::c_ulong,
                    >(
                        Some(
                            cb_get_tcp_msg
                                as unsafe extern "C" fn(
                                    *mut event_data,
                                    *mut libc::c_void,
                                    libc::c_int,
                                ) -> libc::c_int,
                        ),
                    )
                {
                    if ret == -(1 as libc::c_int) {
                        close(fd);
                    }
                    h.fd = fd;
                    del_event(ev, &mut h);
                }
            } else {
                dns_error(
                    1 as libc::c_int,
                    b"call back func is null\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            i += 1;
        }
    };
}
pub static mut qlist_val: [uchar; 10] = [
    'q' as i32 as uchar,
    'l' as i32 as uchar,
    'i' as i32 as uchar,
    's' as i32 as uchar,
    't' as i32 as uchar,
    ' ' as i32 as uchar,
    'v' as i32 as uchar,
    'a' as i32 as uchar,
    'l' as i32 as uchar,
    '\u{0}' as i32 as uchar,
];
pub unsafe extern "C" fn add_query_info(
    mut log_type: libc::c_int,
    mut idx: libc::c_int,
    mut type_0: uint16_t,
) -> libc::c_int {
    let mut thread_num: libc::c_int = 0;
    let mut query_type_num: libc::c_int = 0;
    thread_num = 0 as libc::c_int;
    if log_type == 112 as libc::c_int {
        thread_num = idx;
    } else if log_type == 233 as libc::c_int {
        thread_num = idx + 2 as libc::c_int;
    } else {
        return -(1 as libc::c_int)
    }
    query_type_num = query_type_map[type_0 as usize];
    if query_type_num < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*global_out_info)
        .query_info[thread_num as usize]
        .query_num[query_type_num
        as usize] = ((*global_out_info)
        .query_info[thread_num as usize]
        .query_num[query_type_num as usize])
        .wrapping_add(1);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn get_random_int_from_author(
    mut author: *mut author,
) -> libc::c_int {
    let mut val: libc::c_int = 0;
    let mut tmp: grifa = grifa { val: 0 };
    val = 0 as libc::c_int;
    if ((*author).rndidx as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        >= 3000 as libc::c_ulong
    {
        get_random_data(((*author).randombuffer).as_mut_ptr(), 3000 as libc::c_int);
        (*author).rndidx = 0 as libc::c_int;
    }
    memcpy(
        (tmp.randombuffer).as_mut_ptr() as *mut libc::c_void,
        ((*author).randombuffer).as_mut_ptr().offset((*author).rndidx as isize)
            as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    );
    val = tmp.val;
    (*author)
        .rndidx = ((*author).rndidx as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_int;
    return val;
}
pub unsafe extern "C" fn delete_close_event(
    mut fd: libc::c_int,
    mut f: *mut fetcher,
) -> libc::c_int {
    let mut el: *mut list = 0 as *mut list;
    let mut nd: *mut list_node = 0 as *mut list_node;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    el = 0 as *mut libc::c_void as *mut list;
    nd = 0 as *mut libc::c_void as *mut list_node;
    el = (*f).el;
    if el as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    tmp = malloc(::std::mem::size_of::<list_node>() as libc::c_ulong);
    nd = tmp as *mut list_node;
    if nd as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    (*nd).data = malloc(::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    if (*nd).data as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        free(nd as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    memcpy(
        (*nd).data,
        &mut fd as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    );
    pthread_spin_lock(&mut (*el).lock);
    (*nd).next = (*el).head;
    (*el).head = nd;
    pthread_spin_unlock(&mut (*el).lock);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn write_back_to_client(
    mut mbuf: *mut mbuf_type,
    mut fr: *mut uchar,
    mut vlen: libc::c_int,
) -> libc::c_int {
    let mut sh: setheader = setheader {
        an: 0,
        ns: 0,
        id: 0,
        dlen: 0,
        od: 0 as *mut uchar,
        itor: 0 as *mut uchar,
        type_0: 0,
    };
    let mut main_val: libc::c_int = 0;
    let mut dnslen: libc::c_int = 0;
    let mut msg: *mut uchar = 0 as *mut uchar;
    let mut type_0: uchar = 0;
    let mut from: *mut uchar = 0 as *mut uchar;
    let mut to: *mut uchar = 0 as *mut uchar;
    let mut mv: *mut mvalue = 0 as *mut mvalue;
    let mut jump: libc::c_int = 0;
    let mut temp: uint16_t = 0;
    let mut hlp: [hlpc; 100] = [hlpc {
        name: 0 as *mut uchar,
        off: 0,
        level: 0,
        ref_0: 0,
        mt: 0,
        len: 0,
    }; 100];
    sh.an = 0 as libc::c_int as ushort___0;
    sh.ns = 0 as libc::c_int as libc::c_ushort;
    sh.id = 0 as libc::c_int as libc::c_ushort;
    sh.dlen = 0 as libc::c_int as libc::c_ushort;
    sh.od = 0 as *mut uchar;
    sh.itor = 0 as *mut uchar;
    sh.type_0 = 0 as libc::c_int as libc::c_ushort;
    main_val = 0 as libc::c_int;
    dnslen = 0 as libc::c_int;
    msg = (*mbuf).buf;
    from = fr;
    to = msg;
    mv = 0 as *mut libc::c_void as *mut mvalue;
    jump = 0 as libc::c_int;
    temp = 0 as libc::c_int as uint16_t;
    hlp[0 as libc::c_int as usize].name = (*mbuf).td;
    hlp[0 as libc::c_int as usize]
        .off = ::std::mem::size_of::<dnsheader>() as libc::c_ulong as libc::c_short;
    hlp[0 as libc::c_int as usize]
        .level = (*mbuf).lowerdomain.label_count as libc::c_short;
    hlp[0 as libc::c_int as usize].ref_0 = -(1 as libc::c_int) as libc::c_short;
    hlp[0 as libc::c_int as usize].mt = 0 as libc::c_int as libc::c_short;
    hlp[0 as libc::c_int as usize].len = (*mbuf).dlen as libc::c_short;
    if (*mbuf).dlen == 2 as libc::c_int {
        jump = (::std::mem::size_of::<dnsheader>() as libc::c_ulong)
            .wrapping_add(1 as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<qdns>() as libc::c_ulong) as libc::c_int;
    } else {
        jump = (::std::mem::size_of::<dnsheader>() as libc::c_ulong)
            .wrapping_add((*mbuf).dlen as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<qdns>() as libc::c_ulong) as libc::c_int;
    }
    to = to.offset(jump as isize);
    while vlen > 1 as libc::c_int {
        type_0 = *from.offset(0 as libc::c_int as isize);
        mv = from.offset(1 as libc::c_int as isize) as *mut mvalue;
        to = fill_rrset_in_msg(hlp.as_mut_ptr(), from, to, &mut main_val, msg);
        if to as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            return -(1 as libc::c_int);
        }
        vlen = ((vlen - 1 as libc::c_int - (*mv).len as libc::c_int) as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<mvalue>() as libc::c_ulong)
            as libc::c_int;
        sh.an = (sh.an as libc::c_int + (*mv).num as libc::c_int) as ushort___0;
        from = from
            .offset((*mv).len as libc::c_int as isize)
            .offset(1 as libc::c_int as isize)
            .offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize);
        if !(type_0 as libc::c_int == 5 as libc::c_int) {
            continue;
        }
        if !(vlen > 1 as libc::c_int) {
            continue;
        }
        if !(*from.offset(0 as libc::c_int as isize) as libc::c_int == 1 as libc::c_int)
        {
            if !(*from.offset(0 as libc::c_int as isize) as libc::c_int
                == 28 as libc::c_int)
            {
                if !(*from.offset(0 as libc::c_int as isize) as libc::c_int
                    == 5 as libc::c_int)
                {
                    continue;
                }
            }
        }
        main_val += 1;
        hlp[main_val as usize].name = hlp[(main_val - 1 as libc::c_int) as usize].name;
        hlp[main_val as usize].off = hlp[(main_val - 1 as libc::c_int) as usize].off;
        hlp[main_val as usize].level = hlp[(main_val - 1 as libc::c_int) as usize].level;
        hlp[main_val as usize].len = hlp[(main_val - 1 as libc::c_int) as usize].len;
        hlp[main_val as usize].ref_0 = -(1 as libc::c_int) as libc::c_short;
        hlp[main_val as usize].mt = 0 as libc::c_int as libc::c_short;
    }
    sh.itor = msg;
    if (*mbuf).dlen == 2 as libc::c_int {
        sh.dlen = 1 as libc::c_int as ushort___0;
    } else {
        sh.dlen = (*mbuf).dlen as ushort___0;
    }
    sh.od = (*mbuf).td;
    sh.id = (*mbuf).id;
    sh.type_0 = (*mbuf).qtype as ushort___0;
    fill_header_in_msg(&mut sh);
    dnslen = to.offset_from(msg) as libc::c_long as libc::c_int;
    (*mbuf).buflen = dnslen;
    (*mbuf).addr = &mut (*mbuf).caddr;
    if (*mbuf).socktype == 2 as libc::c_uint {
        if dnslen > 512 as libc::c_int {
            send_tc_to_client(mbuf);
        } else {
            udp_write_info(mbuf, 0 as libc::c_int);
        }
    } else {
        temp = (dnslen as uint16_t as libc::c_int >> 8 as libc::c_int
            | (dnslen << 8 as libc::c_int) as uint16_t as libc::c_int) as uint16_t;
        memcpy(
            msg.offset(-(2 as libc::c_int as isize)) as *mut libc::c_void,
            &mut temp as *mut uint16_t as *const libc::c_void,
            ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
        );
        (*mbuf).buflen = dnslen + 2 as libc::c_int;
        (*mbuf).buf = msg.offset(-(2 as libc::c_int as isize));
        tcp_write_info(mbuf, 0 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn passer_related_data(
    mut si: *mut sockinfo,
    mut mbuf: *mut mbuf_type,
    mut author: *mut author,
) -> libc::c_int {
    let mut buf: *mut uchar = 0 as *mut uchar;
    let mut tail: *mut uchar = 0 as *mut uchar;
    let mut stype: libc::c_int = 0;
    let mut rbt: *mut rbtree = 0 as *mut rbtree;
    let mut datalen: libc::c_int = 0;
    let mut n: ushort___0 = 0;
    let mut hlp: hlpp = hlpp {
        stype: 0 as *mut libc::c_int,
        ds: 0 as *mut htable,
        rbt: 0 as *mut rbtree,
        buf: 0 as *mut uchar,
        datalen: 0,
        dms: 0 as *mut uchar,
        dmsidx: 0,
        section: 0,
        tmpbuf: 0 as *mut uchar,
        domainbuf: 0 as *mut uchar,
        dmbuf: 0 as *mut uchar,
    };
    let mut hdr: *mut dnsheader = 0 as *mut dnsheader;
    let mut opt_owner: uint8_t = 0;
    let mut opt_type: uint16_t = 0;
    buf = (*si).buf;
    tail = 0 as *mut libc::c_void as *mut uchar;
    stype = 0 as libc::c_int;
    datalen = 0 as libc::c_int;
    hdr = buf as *mut dnsheader;
    tail = buf
        .offset(::std::mem::size_of::<dnsheader>() as libc::c_ulong as isize)
        .offset(
            (*(*si).lowerdomain).label_len[0 as libc::c_int as usize] as libc::c_int
                as isize,
        );
    tail = tail.offset(4 as libc::c_int as isize);
    datalen = (*si).buflen;
    rbt = (*(*author).s).ttlexp;
    n = ntohs((*hdr).ancount);
    hlp.stype = &mut stype;
    hlp.ds = (*(*author).s).datasets;
    hlp.rbt = rbt;
    hlp.buf = buf;
    hlp.datalen = datalen;
    hlp.tmpbuf = (*mbuf).tempbuffer;
    hlp.domainbuf = (*mbuf).tdbuffer;
    hlp.dmbuf = (*mbuf).dmbuffer;
    if n as libc::c_int > 0 as libc::c_int {
        hlp.section = 2 as libc::c_int;
        tail = process_rdata(&mut hlp, tail, n as libc::c_int);
        if tail as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            return -(1 as libc::c_int);
        }
    }
    n = ntohs((*hdr).nscount);
    if n as libc::c_int > 0 as libc::c_int {
        hlp.section = 5 as libc::c_int;
        tail = process_rdata(&mut hlp, tail, n as libc::c_int);
        if tail as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            return -(1 as libc::c_int);
        }
    }
    n = ntohs((*hdr).arcount);
    if n as libc::c_int > 0 as libc::c_int {
        if tail.offset(9 as libc::c_uint as isize).offset(2 as libc::c_int as isize)
            as libc::c_ulong <= buf.offset(datalen as isize) as libc::c_ulong
        {
            opt_owner = *tail;
            opt_type = *(tail.offset(1 as libc::c_int as isize) as *mut uint16_t);
            if opt_owner as libc::c_int == 0 as libc::c_int {
                if opt_type as libc::c_int >> 8 as libc::c_int
                    | ((opt_type as libc::c_int) << 8 as libc::c_int) as uint16_t
                        as libc::c_int == 41 as libc::c_int
                {
                    return stype;
                }
            }
        }
        hlp.section = 7 as libc::c_int;
        tail = process_rdata(&mut hlp, tail, n as libc::c_int);
        if tail as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            return -(1 as libc::c_int);
        }
    }
    return stype;
}
pub unsafe extern "C" fn send_msg_tcp(
    mut author: *mut author,
    mut fd: libc::c_int,
) -> libc::c_int {
    let mut id: ushort___0 = 0;
    let mut typeoff: ushort___0 = 0;
    let mut temp: ushort___0 = 0;
    let mut type_0: ushort___0 = 0;
    let mut buffer: *mut uchar = 0 as *mut uchar;
    let mut len: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut mbuf: *mut mbuf_type = 0 as *mut mbuf_type;
    let mut domain: *mut uchar = 0 as *mut uchar;
    buffer = ((*author).tmpbuffer).as_mut_ptr();
    ret = (*author).eptcpfds[fd as usize].ret;
    if ret <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    domain = ((*author).eptcpfds[fd as usize].domain).as_mut_ptr();
    id = (ret & 4095 as libc::c_int) as ushort___0;
    typeoff = (ret >> 12 as libc::c_int) as ushort___0;
    ret = htable_find_list(
        (*(*author).s).qlist,
        domain,
        typeoff as libc::c_int,
        id as libc::c_int,
        &mut mbuf as *mut *mut mbuf_type as *mut *mut uchar,
    );
    if ret < 0 as libc::c_int {
        return ret;
    }
    type_0 = (*mbuf).qtype as ushort___0;
    if (*mbuf).qname as libc::c_int == 6 as libc::c_int {
        type_0 = 1 as libc::c_int as ushort___0;
    }
    len = make_dns_msg_for_new(
        buffer.offset(2 as libc::c_int as isize),
        (*mbuf).aid,
        (*mbuf).qing,
        (*mbuf).qlen as libc::c_int,
        type_0,
    );
    temp = htons(len as uint16_t);
    memcpy(
        buffer as *mut libc::c_void,
        &mut temp as *mut ushort___0 as *const libc::c_void,
        ::std::mem::size_of::<ushort___0>() as libc::c_ulong,
    );
    (*mbuf).fd = fd;
    (*mbuf).buf = buffer;
    (*mbuf).buflen = len + 2 as libc::c_int;
    tcp_write_info(mbuf, 0 as libc::c_int);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn query_from_auth_tcp(
    mut author: *mut author,
    mut mbuf: *mut mbuf_type,
) -> libc::c_int {
    let mut si: sockinfo = sockinfo {
        addr: sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        },
        fd: 0,
        buflen: 0,
        socktype: 0,
        buf: 0 as *mut uchar,
        lowerdomain: 0 as *mut packet_type,
        mbuf: 0 as *mut mbuf_type,
    };
    let mut i: libc::c_int = 0;
    let mut st: libc::c_int = 0;
    let mut ip: *mut uchar = 0 as *mut uchar;
    let mut mv: *mut mvalue = 0 as *mut mvalue;
    st = 0 as libc::c_int;
    ip = ((*author).ip).as_mut_ptr();
    mv = 0 as *mut libc::c_void as *mut mvalue;
    mv = ip as *mut mvalue;
    while (*mv).num as libc::c_int > 0 as libc::c_int {
        ip = ip.offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize);
        i = 0 as libc::c_int;
        while i < (*mv).num as libc::c_int {
            if st == (*mbuf).tcpnums - 1 as libc::c_int {
                si.fd = (*mbuf).tcpfd;
                make_addr_from_bin(&mut si.addr, ip);
                si.addr.sin_port = htons(53 as libc::c_int as uint16_t);
                si.addr.sin_family = 2 as libc::c_int as sa_family_t;
                connect_to(&mut si);
                st = 4 as libc::c_int;
            }
            st += 1;
            i += 1;
        }
        ip = ip.offset((*mv).len as libc::c_int as isize);
        mv = ip as *mut mvalue;
        if st > 3 as libc::c_int {
            break;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn query_from_auth_server(
    mut mbuf: *mut mbuf_type,
    mut author: *mut author,
) -> libc::c_int {
    let mut id: ushort___0 = 0;
    let mut type_0: ushort___0 = 0;
    let mut buffer: *mut uchar = 0 as *mut uchar;
    let mut ip: *mut uchar = 0 as *mut uchar;
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut st: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut mv: *mut mvalue = 0 as *mut mvalue;
    id = (*mbuf).aid;
    buffer = (*mbuf).tempbuffer;
    ip = ((*author).ip).as_mut_ptr();
    st = 1 as libc::c_int;
    mv = 0 as *mut libc::c_void as *mut mvalue;
    if (*mbuf).qname as libc::c_int == 6 as libc::c_int {
        type_0 = 1 as libc::c_int as ushort___0;
    } else {
        type_0 = (*mbuf).qtype as ushort___0;
    }
    (*mbuf).mxtry += 1;
    if (*mbuf).socktype == 2 as libc::c_uint {
        len = make_dns_msg_for_new(
            buffer,
            id,
            (*mbuf).qing,
            (*mbuf).qlen as libc::c_int,
            type_0,
        );
        (*mbuf).buf = buffer;
        (*mbuf).buflen = len;
        (*mbuf).fd = (*author).audp;
        mv = ip as *mut mvalue;
        while (*mv).num as libc::c_int > 0 as libc::c_int {
            ip = ip.offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize);
            i = 0 as libc::c_int;
            while i < (*mv).num as libc::c_int {
                make_addr_from_bin(
                    &mut (*mbuf).aaddr,
                    ip.offset((i * 4 as libc::c_int) as isize),
                );
                (*mbuf).aaddr.sin_port = htons(53 as libc::c_int as uint16_t);
                (*mbuf).addr = &mut (*mbuf).aaddr;
                ret = udp_write_info(mbuf, 0 as libc::c_int);
                if ret > 0 as libc::c_int {
                    st += 1;
                }
                if st > (*mbuf).mxtry {
                    return 0 as libc::c_int;
                }
                i += 1;
            }
            ip = ip.offset((*mv).len as libc::c_int as isize);
            mv = ip as *mut mvalue;
            if st > 3 as libc::c_int {
                break;
            }
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn release_qoutinfo(
    mut author: *mut author,
    mut mbuf: *mut mbuf_type,
    mut idx: uint32_t,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut epfd: libc::c_int = 0;
    let mut id: libc::c_int = 0;
    let mut typeoff: libc::c_int = 0;
    let mut val: *mut uchar = 0 as *mut uchar;
    let mut ev: epoll_event = epoll_event {
        events: 0,
        data: epoll_data {
            ptr: 0 as *mut libc::c_void,
        },
    };
    fd = (*mbuf).tcpfd;
    if fd > 0 as libc::c_int {
        ev.events = 0 as libc::c_int as uint32_t;
        ev.data.ptr = 0 as *mut libc::c_void;
        epfd = (*author).bdepfd;
        (*author).tcpinuse -= 1;
        epoll_ctl(epfd, 2 as libc::c_int, fd, &mut ev);
        (*author).eptcpfds[fd as usize].ret = 0 as libc::c_int;
        close(fd);
    }
    id = (idx & 4095 as libc::c_uint) as libc::c_int;
    typeoff = (idx >> 12 as libc::c_int) as libc::c_int;
    val = htable_delete_list(
        (*(*author).s).qlist,
        ((*mbuf).lowerdomain.domain).as_mut_ptr(),
        typeoff,
        id,
    );
    if val as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        printf(
            b"del list val =0, mbuf:0x%0x\n\0" as *const u8 as *const libc::c_char,
            mbuf,
        );
        return 0 as libc::c_int;
    }
    if !(val as libc::c_ulong == mbuf as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"val == (void *)mbuf\0" as *const u8 as *const libc::c_char,
            b"author.c\0" as *const u8 as *const libc::c_char,
            446 as libc::c_uint,
            b"release_qoutinfo\0" as *const u8 as *const libc::c_char,
        );
    }
    mbuf_free(mbuf);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn init_qoutinfo(mut mbuf: *mut mbuf_type) -> libc::c_int {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut tv as *mut timeval, 0 as *mut libc::c_void);
    (*mbuf).socktype = 2 as libc::c_int as uint___0;
    (*mbuf).mxtry = 0 as libc::c_int;
    (*mbuf).qns = 1 as libc::c_int;
    (*mbuf).sq = 1 as libc::c_int as ushort___0;
    (*mbuf)
        .stime = (tv.tv_sec * 1000 as libc::c_long + tv.tv_usec / 1000 as libc::c_long)
        as uint64_t;
    (*mbuf).tcpfd = 0 as libc::c_int;
    (*mbuf).qtimes = 0 as libc::c_int as ushort___0;
    (*mbuf).tdbuffer = 0 as *mut libc::c_void as *mut uchar;
    (*mbuf).tempbuffer = 0 as *mut libc::c_void as *mut uchar;
    (*mbuf).dmbuffer = 0 as *mut libc::c_void as *mut uchar;
    (*mbuf).ipbuffer = 0 as *mut libc::c_void as *mut uchar;
    (*mbuf).hascname = 0 as libc::c_int as ushort___0;
    (*mbuf).tcpnums = 0 as libc::c_int;
    (*mbuf).stat = 0 as libc::c_int as ushort___0;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn check_enter(
    mut author: *mut author,
    mut buf: *mut uchar,
    mut idx: *mut libc::c_int,
    mut mbuf: *mut *mut mbuf_type,
    mut lowerdomain: *mut packet_type,
) -> libc::c_int {
    let mut id: int32_t = 0;
    let mut typeoff: int32_t = 0;
    let mut ret: libc::c_int = 0;
    let mut tx: libc::c_int = 0;
    let mut hdr: *mut dnsheader = 0 as *mut dnsheader;
    tx = 0 as libc::c_int;
    hdr = buf as *mut dnsheader;
    *idx = (*hdr).id as libc::c_int;
    id = (*hdr).id as libc::c_int & 4095 as libc::c_int;
    typeoff = (*hdr).id as libc::c_int >> 12 as libc::c_int;
    if id >= 4095 as libc::c_int {
        return -(1 as libc::c_int)
    } else {
        if typeoff >= 9 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    ret = check_dns_name(
        buf.offset(::std::mem::size_of::<dnsheader>() as libc::c_ulong as isize),
        lowerdomain,
    );
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    ret = htable_find_list(
        (*(*author).s).qlist,
        ((*lowerdomain).domain).as_mut_ptr(),
        typeoff,
        id,
        mbuf as *mut *mut uchar,
    );
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (**mbuf).stat as libc::c_int == 0 as libc::c_int {
        *mbuf = 0 as *mut libc::c_void as *mut mbuf_type;
        return -(1 as libc::c_int);
    }
    ret = check_an_msg((*hdr).flags, 0 as *mut libc::c_void as *mut uchar, &mut tx);
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if ret == 1 as libc::c_int {
        return -(2 as libc::c_int);
    }
    if ret == 2 as libc::c_int {
        if tx == 1 as libc::c_int {
            return -(3 as libc::c_int);
        }
    }
    (**mbuf).socktype = 2 as libc::c_int as uint___0;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn passer_auth_data(
    mut author: *mut author,
    mut buf: *mut uchar,
    mut si: *mut sockinfo,
) -> libc::c_int {
    let mut idx: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut pret: libc::c_int = 0;
    let mut mbuf: *mut mbuf_type = 0 as *mut mbuf_type;
    let mut xtype: ushort___0 = 0;
    let mut hdr: *mut dnsheader = 0 as *mut dnsheader;
    let mut lowerdomain: packet_type = packet_type {
        label_count: 0,
        domain: [0; 256],
        label: [0 as *mut uint8_t; 64],
        label_offsets: [0; 64],
        label_len: [0; 64],
        hash: [0; 64],
    };
    let mut tmp: uint16_t = 0;
    mbuf = 0 as *mut libc::c_void as *mut mbuf_type;
    xtype = 0 as libc::c_int as ushort___0;
    hdr = buf as *mut dnsheader;
    ret = check_enter(author, buf, &mut idx, &mut mbuf, &mut lowerdomain);
    mbuf_free((*si).mbuf);
    (*si).mbuf = mbuf;
    if ret == -(2 as libc::c_int) {
        return -idx - 1 as libc::c_int;
    }
    if ret == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if ret == -(1 as libc::c_int) {
        return idx + 1 as libc::c_int;
    }
    (*mbuf).mxtry -= 1;
    if ret == -(3 as libc::c_int) {
        (*mbuf)
            .qtimes = ((*mbuf).qtimes as libc::c_int + 1 as libc::c_int) as ushort___0;
        return 0 as libc::c_int;
    }
    (*si).lowerdomain = &mut lowerdomain;
    pret = passer_related_data(si, mbuf, author);
    if pret < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    (*mbuf).fd = (*(*author).s).ludp;
    (*mbuf).addr = &mut (*mbuf).caddr;
    if pret == 5 as libc::c_int {
        if (*mbuf).qtype as libc::c_uint == 5 as libc::c_uint {
            if (*mbuf).fd != -(1 as libc::c_int) {
                *(buf as *mut ushort___0) = (*mbuf).cid;
                (*mbuf).buf = buf;
                (*mbuf).buflen = (*si).buflen;
                if (*si).buflen > 512 as libc::c_int {
                    send_tc_to_client(mbuf);
                } else {
                    udp_write_info(mbuf, 0 as libc::c_int);
                    write_log(
                        (*author).loginfo,
                        (*author).idx,
                        (*mbuf).td as *const uchar,
                        (*mbuf).dlen,
                        (*mbuf).qtype as libc::c_int,
                        (*mbuf).addr,
                    );
                }
            }
            return idx + 1 as libc::c_int;
        }
    }
    if pret == 5 as libc::c_int {
        (*mbuf).stat = 1 as libc::c_int as ushort___0;
        (*mbuf).socktype = 2 as libc::c_int as uint___0;
        return 0 as libc::c_int;
    } else {
        if (*mbuf).qname as libc::c_int != 4 as libc::c_int {
            (*mbuf).stat = 1 as libc::c_int as ushort___0;
            (*mbuf).socktype = 2 as libc::c_int as uint___0;
            return 0 as libc::c_int;
        }
    }
    's_328: {
        if !(pret == 6 as libc::c_int) {
            tmp = ntohs((*hdr).ancount);
            if !(tmp as libc::c_int > 0 as libc::c_int) {
                break 's_328;
            }
        }
        if (*mbuf).fd != -(1 as libc::c_int) {
            if (*mbuf).hascname as libc::c_int == 0 as libc::c_int {
                *(buf as *mut ushort___0) = (*mbuf).cid;
                (*mbuf).buf = buf;
                (*mbuf).buflen = (*si).buflen;
                if (*si).buflen > 512 as libc::c_int {
                    send_tc_to_client(mbuf);
                } else {
                    udp_write_info(mbuf, 0 as libc::c_int);
                    write_log(
                        (*author).loginfo,
                        (*author).idx,
                        (*mbuf).td as *const uchar,
                        (*mbuf).dlen,
                        (*mbuf).qtype as libc::c_int,
                        (*mbuf).addr,
                    );
                }
            } else {
                if pret == 6 as libc::c_int {
                    xtype = 5 as libc::c_int as ushort___0;
                } else {
                    xtype = (*mbuf).qtype as ushort___0;
                }
                ret = find_record_from_mem(
                    (*mbuf).td,
                    (*mbuf).dlen,
                    xtype as libc::c_int,
                    (*(*author).s).datasets,
                    ((*author).tmpbuffer).as_mut_ptr(),
                    ((*author).databuffer).as_mut_ptr(),
                    &mut *((*mbuf).lowerdomain.hash)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize),
                );
                if ret > 0 as libc::c_int {
                    (*author).response += 1;
                    if (*mbuf).fd != -(1 as libc::c_int) {
                        (*mbuf)
                            .buf = ((*mbuf).data)
                            .as_mut_ptr()
                            .offset(2 as libc::c_int as isize);
                        write_back_to_client(
                            mbuf,
                            ((*author).databuffer).as_mut_ptr(),
                            ret,
                        );
                    }
                    write_log(
                        (*author).loginfo,
                        (*author).idx,
                        (*mbuf).td as *const uchar,
                        (*mbuf).dlen,
                        (*mbuf).qtype as libc::c_int,
                        (*mbuf).addr,
                    );
                }
            }
        }
        return idx + 1 as libc::c_int;
    }
    (*mbuf).stat = 1 as libc::c_int as ushort___0;
    (*mbuf).socktype = 2 as libc::c_int as uint___0;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn cb_read_auth(
    mut ev: *mut epoll_event,
    mut si: *mut sockinfo,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut szhdr: libc::c_int = 0;
    let mut mbuf: *mut mbuf_type = 0 as *mut mbuf_type;
    let mut tmp: *mut mbuf_type = 0 as *mut mbuf_type;
    let mut tmp___0: libc::c_int = 0;
    szhdr = ::std::mem::size_of::<dnsheader>() as libc::c_ulong as libc::c_int;
    tmp = mbuf_alloc();
    mbuf = tmp;
    if 0 as *mut libc::c_void as libc::c_ulong == mbuf as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    memset(
        mbuf as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mbuf_type>() as libc::c_ulong,
    );
    (*mbuf).fd = (*ev).data.fd;
    (*mbuf).buf = (*si).buf;
    (*mbuf).buflen = 2000 as libc::c_int;
    (*mbuf).addr = &mut (*mbuf).aaddr;
    if (*si).socktype == 1 as libc::c_int {
        ret = tcp_read_dns_msg(mbuf, 4094 as libc::c_int as uint___0, 0 as libc::c_int);
    } else {
        ret = udp_read_msg(mbuf, 0 as libc::c_int);
    }
    if ret < szhdr {
        mbuf_free(mbuf);
        return -(1 as libc::c_int);
    }
    tmp___0 = ret;
    (*mbuf).buflen = tmp___0;
    (*si).buflen = tmp___0;
    (*si).mbuf = mbuf;
    return ret;
}
pub unsafe extern "C" fn launch_new_query(mut author: *mut author) -> libc::c_int {
    let mut new_query: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut mbuf: *mut mbuf_type = 0 as *mut mbuf_type;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut msnow: uint64_t = 0;
    let mut slotoff: libc::c_int = 0;
    let mut typeoff: libc::c_int = 0;
    new_query = 0 as libc::c_int;
    msnow = 0 as libc::c_int as uint64_t;
    start = (*author).start;
    end = (*author).end;
    gettimeofday(&mut tv as *mut timeval, 0 as *mut libc::c_void);
    msnow = (tv.tv_sec * 1000 as libc::c_long + tv.tv_usec / 1000 as libc::c_long)
        as uint64_t;
    i = start;
    while i < end {
        slotoff = 0 as libc::c_int;
        typeoff = 0 as libc::c_int;
        mbuf = 0 as *mut libc::c_void as *mut mbuf_type;
        ret = htable_find_list_io(
            (*(*author).s).qlist,
            i,
            slotoff,
            &mut typeoff,
            &mut mbuf as *mut *mut mbuf_type as *mut *mut uchar,
        );
        while ret >= 0 as libc::c_int {
            if ret > 0 as libc::c_int {
                if (*mbuf).qtimes as libc::c_int > 15 as libc::c_int {
                    release_qoutinfo(
                        author,
                        mbuf,
                        (i | typeoff << 12 as libc::c_int) as uint32_t,
                    );
                } else if msnow.wrapping_sub((*mbuf).stime) > 5000 as libc::c_ulong {
                    release_qoutinfo(
                        author,
                        mbuf,
                        (i | typeoff << 12 as libc::c_int) as uint32_t,
                    );
                } else {
                    if (*mbuf).stat as libc::c_int == 0 as libc::c_int {
                        if i < 4095 as libc::c_int {
                            if !(typeoff < 9 as libc::c_int) {
                                __assert_fail(
                                    b"i < QLIST_TABLE_SIZE && typeoff < SUPPORT_TYPE_NUM\0"
                                        as *const u8 as *const libc::c_char,
                                    b"author.c\0" as *const u8 as *const libc::c_char,
                                    684 as libc::c_uint,
                                    b"launch_new_query\0" as *const u8 as *const libc::c_char,
                                );
                            }
                        } else {
                            __assert_fail(
                                b"i < QLIST_TABLE_SIZE && typeoff < SUPPORT_TYPE_NUM\0"
                                    as *const u8 as *const libc::c_char,
                                b"author.c\0" as *const u8 as *const libc::c_char,
                                684 as libc::c_uint,
                                b"launch_new_query\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        (*mbuf).aid = (i | typeoff << 12 as libc::c_int) as ushort___0;
                        (*mbuf).backid = (*mbuf).aid;
                        (*mbuf).mxtry = 0 as libc::c_int;
                        if (*mbuf).fd != -(1 as libc::c_int) {
                            (*mbuf).fd = (*author).cudp;
                        }
                        (*mbuf).tdbuffer = ((*author).tdbuffer).as_mut_ptr();
                        (*mbuf).tempbuffer = ((*author).tempbuffer).as_mut_ptr();
                        (*mbuf).dmbuffer = ((*author).dmbuffer).as_mut_ptr();
                        (*mbuf).ipbuffer = ((*author).ipbuffer).as_mut_ptr();
                        new_query += 1;
                        (*mbuf).stat = 1 as libc::c_int as ushort___0;
                    }
                    if msnow.wrapping_sub((*mbuf).stime) > 1000 as libc::c_ulong {
                        if (*mbuf).sq as libc::c_int == 0 as libc::c_int {
                            (*mbuf).sq = 1 as libc::c_int as ushort___0;
                        }
                    }
                    if (*mbuf).socktype == 2 as libc::c_uint {
                        if (*mbuf).sq as libc::c_int == 1 as libc::c_int {
                            ret = find_addr(
                                (*(*author).s).forward,
                                (*(*author).s).datasets,
                                mbuf,
                                ((*author).ip).as_mut_ptr(),
                                (*(*author).s).is_forward,
                            );
                            if (*mbuf).stat as libc::c_int == 1 as libc::c_int {
                                if ret == 0 as libc::c_int {
                                    query_from_auth_server(mbuf, author);
                                }
                            }
                            (*mbuf)
                                .qtimes = ((*mbuf).qtimes as libc::c_int + 1 as libc::c_int)
                                as ushort___0;
                        }
                    }
                }
            }
            if ret == 0 as libc::c_int {
                slotoff += 1;
                typeoff = 0 as libc::c_int;
            } else if typeoff == 8 as libc::c_int {
                slotoff += 1;
                typeoff = 0 as libc::c_int;
            } else {
                typeoff += 1;
            }
            mbuf = 0 as *mut libc::c_void as *mut mbuf_type;
            ret = htable_find_list_io(
                (*(*author).s).qlist,
                i,
                slotoff,
                &mut typeoff,
                &mut mbuf as *mut *mut mbuf_type as *mut *mut uchar,
            );
        }
        i += 1;
    }
    return new_query;
}
pub unsafe extern "C" fn after_pass_data(
    mut ret: libc::c_int,
    mut author: *mut author,
    mut mbuf: *mut mbuf_type,
) -> libc::c_int {
    let mut ev: epoll_event = epoll_event {
        events: 0,
        data: epoll_data {
            ptr: 0 as *mut libc::c_void,
        },
    };
    let mut fd: libc::c_int = 0;
    ev.events = 0 as libc::c_int as uint32_t;
    ev.data.ptr = 0 as *mut libc::c_void;
    if ret == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if mbuf as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    if ret < 0 as libc::c_int {
        ret += 1;
        ret = -ret;
        if (*mbuf).tcpfd > 0 as libc::c_int {
            if (*mbuf).qtimes as libc::c_int % 5 as libc::c_int == 0 as libc::c_int {
                ev.data.fd = (*mbuf).tcpfd;
                (*mbuf).tcpfd = 0 as libc::c_int;
                (*author).tcpinuse -= 1;
                epoll_ctl((*author).bdepfd, 2 as libc::c_int, ev.data.fd, &mut ev);
                close(ev.data.fd);
            }
        }
        if (*mbuf).tcpfd > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        if (*author).tcpinuse > 1000 as libc::c_int {
            fd = -(1 as libc::c_int);
        } else {
            (*mbuf).tcpnums += 1;
            fd = socket(2 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int);
        }
        if fd > 0 as libc::c_int {
            (*author).tcpinuse += 1;
            (*mbuf).tcpfd = fd;
            (*mbuf).socktype = 1 as libc::c_int as uint___0;
            ev.data.fd = fd;
            ev.events = 4 as libc::c_int as uint32_t;
            (*author).eptcpfds[fd as usize].ret = ret;
            memcpy(
                ((*author).eptcpfds[fd as usize].domain).as_mut_ptr()
                    as *mut libc::c_void,
                (*mbuf).td as *const libc::c_void,
                (*mbuf).dlen as size_t,
            );
            set_non_block(fd);
            set_recv_timeout(fd, 0 as libc::c_int, 500 as libc::c_int);
            epoll_ctl((*author).bdepfd, 1 as libc::c_int, fd, &mut ev);
            query_from_auth_tcp(author, mbuf);
            return 0 as libc::c_int;
        } else {
            ret += 1;
        }
    }
    if ret > 0 as libc::c_int {
        ret -= 1;
        release_qoutinfo(author, mbuf, ret as uint32_t);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn handle_back_event(mut author: *mut author) -> libc::c_int {
    let mut infinite: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut epfd: libc::c_int = 0;
    let mut si: sockinfo = sockinfo {
        addr: sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        },
        fd: 0,
        buflen: 0,
        socktype: 0,
        buf: 0 as *mut uchar,
        lowerdomain: 0 as *mut packet_type,
        mbuf: 0 as *mut mbuf_type,
    };
    let mut bf: libc::c_int = 0;
    let mut rx: libc::c_int = 0;
    let mut ev: epoll_event = epoll_event {
        events: 0,
        data: epoll_data {
            ptr: 0 as *mut libc::c_void,
        },
    };
    let mut e: *mut epoll_event = 0 as *mut epoll_event;
    let mut buf: *mut uchar = 0 as *mut uchar;
    let mut tmp: libc::c_int = 0;
    infinite = 1 as libc::c_int;
    epfd = (*author).bdepfd;
    si.addr.sin_family = 0 as libc::c_int as sa_family_t;
    si.addr.sin_port = 0 as libc::c_int as libc::c_ushort;
    si.addr.sin_addr.s_addr = 0 as libc::c_uint;
    si.addr.sin_zero[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    si.addr.sin_zero[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    si.addr.sin_zero[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    si.addr.sin_zero[3 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    si.addr.sin_zero[4 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    si.addr.sin_zero[5 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    si.addr.sin_zero[6 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    si.addr.sin_zero[7 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    si.fd = 0 as libc::c_int;
    si.buflen = 0 as libc::c_int;
    si.socktype = 0 as libc::c_int;
    si.buf = 0 as *mut uchar;
    si.lowerdomain = 0 as *mut packet_type;
    si.mbuf = 0 as *mut mbuf_type;
    bf = 0 as libc::c_int;
    ev.events = 0 as libc::c_int as uint32_t;
    ev.data.ptr = 0 as *mut libc::c_void;
    e = ((*author).e).as_mut_ptr();
    buf = ((*author).tmpbuffer).as_mut_ptr();
    while infinite != 0 {
        bf = (*author).audp;
        ret = epoll_wait(epfd, e, 1000 as libc::c_int, 500 as libc::c_int);
        if ret <= 0 as libc::c_int {
            break;
        }
        i = 0 as libc::c_int;
        while i < ret {
            memset(
                &mut si as *mut sockinfo as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<sockinfo>() as libc::c_ulong,
            );
            si.buf = buf;
            if (*e.offset(i as isize)).data.fd == bf {
                si.socktype = 2 as libc::c_int;
                loop {
                    tmp = cb_read_auth(e.offset(i as isize), &mut si);
                    if !(tmp > 0 as libc::c_int) {
                        break;
                    }
                    rx = passer_auth_data(author, buf, &mut si);
                    after_pass_data(rx, author, si.mbuf);
                }
            } else if (*e.offset(i as isize)).data.fd > 0 as libc::c_int {
                if (*e.offset(i as isize)).events == 4 as libc::c_uint {
                    rx = send_msg_tcp(author, (*e.offset(i as isize)).data.fd);
                    if rx < 0 as libc::c_int {
                        printf(
                            b"send msg tcp error\n\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    ev.data.fd = (*e.offset(i as isize)).data.fd;
                    ev.events = 1 as libc::c_int as uint32_t;
                    epoll_ctl(
                        epfd,
                        3 as libc::c_int,
                        (*e.offset(i as isize)).data.fd,
                        &mut ev,
                    );
                } else if (*e.offset(i as isize)).events == 1 as libc::c_uint {
                    si.socktype = 1 as libc::c_int;
                    rx = cb_read_auth(e.offset(i as isize), &mut si);
                    if rx < 0 as libc::c_int {
                        (*author)
                            .eptcpfds[(*e.offset(i as isize)).data.fd as usize]
                            .ret = 0 as libc::c_int;
                        close((*e.offset(i as isize)).data.fd);
                        ev.data.fd = (*e.offset(i as isize)).data.fd;
                        epoll_ctl(epfd, 2 as libc::c_int, ev.data.fd, &mut ev);
                    } else {
                        rx = passer_auth_data(author, buf, &mut si);
                        after_pass_data(rx, author, si.mbuf);
                    }
                } else {
                    ev.data.fd = (*e.offset(i as isize)).data.fd;
                    rx = epoll_ctl(
                        epfd,
                        2 as libc::c_int,
                        (*e.offset(i as isize)).data.fd,
                        &mut ev,
                    );
                    (*author)
                        .eptcpfds[(*e.offset(i as isize)).data.fd as usize]
                        .ret = 0 as libc::c_int;
                    close((*e.offset(i as isize)).data.fd);
                }
            }
            i += 1;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dup_data_into_db(mut a: *mut author) -> libc::c_int {
    let mut i: uint___0 = 0;
    let mut limit: uint___0 = 0;
    let mut rbt: *mut rbtree = 0 as *mut rbtree;
    let mut dboff: uint___0 = 0;
    let mut dbidx: uint___0 = 0;
    rbt = (*(*a).s).ttlexp;
    if (*a).dupbefore == 1 as libc::c_int {
        (*a).limits += 5 as libc::c_int;
        if (*a).limits > 1000 as libc::c_int {
            (*a).limits = 1000 as libc::c_int;
        }
    }
    limit = (*a).limits as uint___0;
    (*a).hsidx += 1;
    if (*a).hsidx == 10 as libc::c_int {
        (*a).hsidx = 0 as libc::c_int;
    }
    i = 0 as libc::c_int as uint___0;
    while i < 65536 as libc::c_uint {
        dbidx = i;
        dboff = (*a).hsidx as uint___0;
        htable_find_io(
            ((*(*a).s).datasets).offset(dboff as isize),
            dbidx as libc::c_int,
            limit,
            rbt,
            3 as libc::c_int,
        );
        i = i.wrapping_add(1);
    }
    (*a).dupbefore = 1 as libc::c_int;
    return 0 as libc::c_int;
}
static mut tmx: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn check_mm_cache(mut author: *mut author) -> libc::c_int {
    let mut total: uint___0 = 0;
    let mut i: libc::c_int = 0;
    total = 0 as libc::c_int as uint___0;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        pthread_spin_lock(&mut (*((*(*author).s).datasets).offset(i as isize)).lock);
        total = (total as libc::c_uint)
            .wrapping_add((*((*(*author).s).datasets).offset(i as isize)).now)
            as uint___0 as uint___0;
        pthread_spin_unlock(&mut (*((*(*author).s).datasets).offset(i as isize)).lock);
        i += 1;
    }
    tmx += 1;
    if total > MAX_ELE_NUM {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn check_ttl_expire(mut author: *mut author) -> libc::c_int {
    let mut now: time_t = 0;
    let mut tn: *mut ttlnode = 0 as *mut ttlnode;
    let mut pn: *mut rbnode = 0 as *mut rbnode;
    let mut mbuf: *mut mbuf_type = 0 as *mut mbuf_type;
    let mut ret: libc::c_int = 0;
    let mut rbt: *mut rbtree = 0 as *mut rbtree;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    tn = 0 as *mut libc::c_void as *mut ttlnode;
    pn = 0 as *mut libc::c_void as *mut rbnode;
    ret = -(1 as libc::c_int);
    rbt = 0 as *mut libc::c_void as *mut rbtree;
    mbuf = mbuf_alloc();
    if 0 as *mut libc::c_void as libc::c_ulong == mbuf as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    now = global_now;
    rbt = (*(*author).s).ttlexp;
    pthread_spin_lock(&mut (*rbt).lock);
    pn = min_node(rbt);
    while pn as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tn = (*pn).key as *mut ttlnode;
        if (*tn).exp as time_t > now + 3 as libc::c_long {
            break;
        }
        tmp = delete_node(rbt, pn);
        tn = tmp as *mut ttlnode;
        pthread_spin_unlock(&mut (*rbt).lock);
        if tn as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            memset(
                mbuf as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<mbuf_type>() as libc::c_ulong,
            );
            (*mbuf).qname = (*tn).type_0;
            (*mbuf).qtype = (*tn).type_0 as rrtype;
            (*mbuf).dlen = (*tn).dlen as libc::c_int;
            memcpy(
                &mut (*mbuf).lowerdomain as *mut packet_type as *mut libc::c_void,
                (*tn).lowerdomain as *const libc::c_void,
                ::std::mem::size_of::<packet_type>() as libc::c_ulong,
            );
            i = 0 as libc::c_int;
            while i < (*(*tn).lowerdomain).label_count as libc::c_int {
                (*mbuf)
                    .lowerdomain
                    .label[i
                    as usize] = ((*mbuf).lowerdomain.domain)
                    .as_mut_ptr()
                    .offset(
                        (*mbuf).lowerdomain.label_offsets[i as usize] as libc::c_int
                            as isize,
                    );
                i += 1;
            }
            (*mbuf)
                .qhash = &mut *((*mbuf).lowerdomain.hash)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut hashval_t;
            (*mbuf).td = ((*mbuf).lowerdomain.domain).as_mut_ptr();
            (*mbuf).qing = (*mbuf).td;
            (*mbuf).qlen = (*mbuf).dlen as ushort___0;
            (*mbuf).cid = 0 as libc::c_int as ushort___0;
            (*mbuf).fd = -(1 as libc::c_int);
            init_qoutinfo(mbuf);
            ret = htable_insert_list(
                (*(*author).s).qlist,
                (*tn).data,
                (*tn).dlen as libc::c_int,
                (*tn).type_0 as libc::c_int,
                mbuf as *mut uchar,
                0 as libc::c_int,
                0 as *mut libc::c_void as *mut mvalue,
                (*tn).hash,
            );
            if 0 as libc::c_int == ret {
                mbuf = mbuf_alloc();
                if 0 as *mut libc::c_void as libc::c_ulong == mbuf as libc::c_ulong {
                    free((*tn).lowerdomain as *mut libc::c_void);
                    free(tn as *mut libc::c_void);
                    return -(1 as libc::c_int);
                }
            }
            free((*tn).lowerdomain as *mut libc::c_void);
            free(tn as *mut libc::c_void);
        }
        pthread_spin_lock(&mut (*rbt).lock);
        pn = min_node(rbt);
    }
    mbuf_free(mbuf);
    pthread_spin_unlock(&mut (*rbt).lock);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn check_refresh_flag(mut author: *mut author) -> libc::c_int {
    let mut s: *mut server = 0 as *mut server;
    s = (*author).s;
    if (*s).lastrefresh + 10 as libc::c_long > global_now {
        return 0 as libc::c_int;
    }
    if (*s).refreshflag as libc::c_int == 1 as libc::c_int {
        (*s).refreshflag = 0 as libc::c_int as uint16_t;
        (*s).lastrefresh = global_now;
        refresh_records((*s).datasets, (*s).ttlexp);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn run_quizzer(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut author: *mut author = 0 as *mut author;
    let mut epfd: libc::c_int = 0;
    let mut tmp: pthread_t = 0;
    let mut tmp___0: libc::c_int = 0;
    author = arg as *mut author;
    tmp = pthread_self();
    pthread_detach(tmp);
    epfd = add_backdoor((*author).audp);
    (*author).bdepfd = epfd;
    loop {
        launch_new_query(author);
        handle_back_event(author);
        if (*author).idx == 0 as libc::c_int {
            check_ttl_expire(author);
            tmp___0 = check_mm_cache(author);
            if tmp___0 == 1 as libc::c_int {
                dup_data_into_db(author);
            } else {
                (*author).dupbefore = 0 as libc::c_int;
            }
            check_refresh_flag(author);
        }
    };
}
pub unsafe extern "C" fn add_to_quizzer(
    mut qo: *mut qoutinfo,
    mut s: *mut server,
    mut qidx: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut randomoff: libc::c_int = 0;
    let mut qi: *mut qoutinfo = 0 as *mut qoutinfo;
    let mut tmp: libc::c_long = 0;
    randomoff = 0 as libc::c_int;
    qi = qo;
    (*qi).stat = 0 as libc::c_int as ushort___0;
    tmp = random();
    randomoff = (tmp % 10000 as libc::c_long) as libc::c_int;
    j = qidx;
    while j < 2 as libc::c_int {
        i = randomoff;
        while i < 10000 as libc::c_int {
            if (*((*s).authors).offset(j as isize)).list[i as usize] as libc::c_ulong
                == 0 as *mut libc::c_void as libc::c_ulong
            {
                pthread_spin_lock(&mut (*((*s).authors).offset(j as isize)).lock);
                if (*((*s).authors).offset(j as isize)).list[i as usize] as libc::c_ulong
                    != 0 as *mut libc::c_void as libc::c_ulong
                {
                    pthread_spin_unlock(&mut (*((*s).authors).offset(j as isize)).lock);
                } else {
                    let ref mut fresh19 = (*((*s).authors).offset(j as isize))
                        .list[i as usize];
                    *fresh19 = qi;
                    let ref mut fresh20 = (*((*s).authors).offset(j as isize)).qnum;
                    *fresh20 += 1;
                    pthread_spin_unlock(&mut (*((*s).authors).offset(j as isize)).lock);
                    return 0 as libc::c_int;
                }
            }
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < randomoff {
            if (*((*s).authors).offset(j as isize)).list[i as usize] as libc::c_ulong
                == 0 as *mut libc::c_void as libc::c_ulong
            {
                pthread_spin_lock(&mut (*((*s).authors).offset(j as isize)).lock);
                if (*((*s).authors).offset(j as isize)).list[i as usize] as libc::c_ulong
                    != 0 as *mut libc::c_void as libc::c_ulong
                {
                    pthread_spin_unlock(&mut (*((*s).authors).offset(j as isize)).lock);
                } else {
                    let ref mut fresh21 = (*((*s).authors).offset(j as isize))
                        .list[i as usize];
                    *fresh21 = qi;
                    let ref mut fresh22 = (*((*s).authors).offset(j as isize)).qnum;
                    *fresh22 += 1;
                    pthread_spin_unlock(&mut (*((*s).authors).offset(j as isize)).lock);
                    return 0 as libc::c_int;
                }
            }
            i += 1;
        }
        j += 1;
    }
    j = 0 as libc::c_int;
    while j < qidx {
        i = randomoff;
        while i < 10000 as libc::c_int {
            if (*((*s).authors).offset(j as isize)).list[i as usize] as libc::c_ulong
                == 0 as *mut libc::c_void as libc::c_ulong
            {
                pthread_spin_lock(&mut (*((*s).authors).offset(j as isize)).lock);
                if (*((*s).authors).offset(j as isize)).list[i as usize] as libc::c_ulong
                    != 0 as *mut libc::c_void as libc::c_ulong
                {
                    pthread_spin_unlock(&mut (*((*s).authors).offset(j as isize)).lock);
                } else {
                    let ref mut fresh23 = (*((*s).authors).offset(j as isize))
                        .list[i as usize];
                    *fresh23 = qi;
                    let ref mut fresh24 = (*((*s).authors).offset(j as isize)).qnum;
                    *fresh24 += 1;
                    pthread_spin_unlock(&mut (*((*s).authors).offset(j as isize)).lock);
                    return 0 as libc::c_int;
                }
            }
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < randomoff {
            if (*((*s).authors).offset(j as isize)).list[i as usize] as libc::c_ulong
                == 0 as *mut libc::c_void as libc::c_ulong
            {
                pthread_spin_lock(&mut (*((*s).authors).offset(j as isize)).lock);
                if (*((*s).authors).offset(j as isize)).list[i as usize] as libc::c_ulong
                    != 0 as *mut libc::c_void as libc::c_ulong
                {
                    pthread_spin_unlock(&mut (*((*s).authors).offset(j as isize)).lock);
                } else {
                    let ref mut fresh25 = (*((*s).authors).offset(j as isize))
                        .list[i as usize];
                    *fresh25 = qi;
                    let ref mut fresh26 = (*((*s).authors).offset(j as isize)).qnum;
                    *fresh26 += 1;
                    pthread_spin_unlock(&mut (*((*s).authors).offset(j as isize)).lock);
                    return 0 as libc::c_int;
                }
            }
            i += 1;
        }
        j += 1;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn lock_and_add_to_quizz(
    mut mbuf: *mut mbuf_type,
    mut f: *mut fetcher,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if (*mbuf).dlen < 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*mbuf).qname = (*mbuf).qtype as ushort___0;
    (*mbuf).td = ((*mbuf).lowerdomain.domain).as_mut_ptr();
    (*mbuf).qing = (*mbuf).td;
    (*mbuf)
        .qhash = &mut *((*mbuf).lowerdomain.hash)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut hashval_t;
    (*mbuf).qlen = (*mbuf).dlen as ushort___0;
    (*mbuf).cid = (*mbuf).id;
    init_qoutinfo(mbuf);
    ret = htable_insert_list(
        (*(*f).s).qlist,
        ((*mbuf).lowerdomain.domain).as_mut_ptr(),
        (*mbuf).dlen,
        (*mbuf).qtype as libc::c_int,
        mbuf as *mut uchar,
        0 as libc::c_int,
        0 as *mut libc::c_void as *mut mvalue,
        &mut *((*mbuf).lowerdomain.hash).as_mut_ptr().offset(0 as libc::c_int as isize),
    );
    if ret != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn find_record_from_mem(
    mut otd: *mut uchar,
    mut dlen: libc::c_int,
    mut type_0: libc::c_int,
    mut datasets: *mut htable,
    mut tdbuffer: *mut uchar,
    mut databuffer: *mut uchar,
    mut hash: *mut hashval_t,
) -> libc::c_int {
    let mut td: *mut uchar = 0 as *mut uchar;
    let mut ret: libc::c_int = 0;
    let mut dataidx: libc::c_int = 0;
    let mut clen: libc::c_int = 0;
    let mut debug: libc::c_int = 0;
    let mut thash: hashval_t = 0;
    let mut h: *mut hashval_t = 0 as *mut hashval_t;
    let mut tmp: libc::c_int = 0;
    td = otd;
    dataidx = 0 as libc::c_int;
    debug = 100 as libc::c_int;
    h = hash;
    dataidx += 1;
    if type_0 != 5 as libc::c_int {
        loop {
            ret = find_record_with_ttl(
                datasets,
                td,
                dlen,
                5 as libc::c_int,
                databuffer.offset(dataidx as isize),
                65528 as libc::c_int - dataidx,
                0 as *mut libc::c_void as *mut mvalue,
                h,
            );
            if !(ret > 0 as libc::c_int) {
                break;
            }
            *databuffer
                .offset(
                    (dataidx - 1 as libc::c_int) as isize,
                ) = 5 as libc::c_int as uchar;
            clen = (ret as libc::c_ulong)
                .wrapping_sub(::std::mem::size_of::<mvalue>() as libc::c_ulong)
                as libc::c_int;
            td = tdbuffer;
            memcpy(
                td as *mut libc::c_void,
                databuffer
                    .offset(dataidx as isize)
                    .offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize)
                    as *const libc::c_void,
                clen as size_t,
            );
            dataidx += ret;
            dataidx += 1;
            tmp = debug;
            debug -= 1;
            if tmp == 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            thash = 0 as libc::c_int as hashval_t;
            h = &mut thash;
            dlen = clen;
        }
        thash = 0 as libc::c_int as hashval_t;
    }
    ret = find_record_with_ttl(
        datasets,
        td,
        dlen,
        type_0,
        databuffer.offset(dataidx as isize),
        65528 as libc::c_int - dataidx,
        0 as *mut libc::c_void as *mut mvalue,
        h,
    );
    if ret > 0 as libc::c_int {
        *databuffer.offset((dataidx - 1 as libc::c_int) as isize) = type_0 as uchar;
        dataidx += ret;
        return dataidx;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn global_cron(mut s: *mut server) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut nds: *mut list_node = 0 as *mut list_node;
    let mut tmp: *mut list_node = 0 as *mut list_node;
    let mut el: *mut list = 0 as *mut list;
    fd = -(1 as libc::c_int);
    el = &mut (*s).eventlist;
    pthread_spin_lock(&mut (*el).lock);
    nds = (*el).head;
    (*el).head = 0 as *mut libc::c_void as *mut list_node;
    pthread_spin_unlock(&mut (*el).lock);
    while nds as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        fd = *((*nds).data as *mut libc::c_int);
        if fd > 0 as libc::c_int {
            close(fd);
        }
        tmp = (*nds).next;
        free((*nds).data);
        free(nds as *mut libc::c_void);
        nds = tmp;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn run_fetcher(mut f: *mut fetcher) -> libc::c_int {
    let mut mc: *mut msgcache = 0 as *mut msgcache;
    let mut ret: libc::c_int = 0;
    let mut mbuf: *mut mbuf_type = 0 as *mut mbuf_type;
    let mut fd: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    mc = (*f).mc;
    ret = 0 as libc::c_int;
    loop {
        fd = -(1 as libc::c_int);
        pthread_spin_lock(&mut (*mc).lock);
        if (*mc).pkt == 0 as libc::c_uint {
            pthread_spin_unlock(&mut (*mc).lock);
            usleep(1000 as libc::c_int as __useconds_t);
        } else {
            memcpy(
                &mut mbuf as *mut *mut mbuf_type as *mut libc::c_void,
                ((*mc).data).as_mut_ptr().offset((*mc).head as isize)
                    as *const libc::c_void,
                ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
            );
            (*mc)
                .head = ((*mc).head as libc::c_ulong)
                .wrapping_add(
                    ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                ) as uint64_t as uint64_t;
            if ((*mc).head).wrapping_add(8 as libc::c_ulong) > (*mc).size as uint64_t {
                (*mc).head = 0 as libc::c_int as uint64_t;
            }
            (*mc).pkt = ((*mc).pkt).wrapping_sub(1);
            pthread_spin_unlock(&mut (*mc).lock);
            if (*mbuf).socktype == 2 as libc::c_uint {
                (*mbuf).fd = (*(*f).s).ludp;
            }
            passer_dns_data(mbuf);
            if (*mbuf).err == 1 as libc::c_int {
                mbuf_free(mbuf);
            } else {
                (*f).dataidx = 0 as libc::c_int;
                (*mbuf).td = ((*mbuf).lowerdomain.domain).as_mut_ptr();
                ret = find_record_from_mem(
                    (*mbuf).td,
                    (*mbuf).dlen,
                    (*mbuf).qtype as libc::c_int,
                    (*(*f).s).datasets,
                    ((*f).tdbuffer).as_mut_ptr(),
                    ((*f).databuffer).as_mut_ptr(),
                    &mut *((*mbuf).lowerdomain.hash)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize),
                );
                if ret > 0 as libc::c_int {
                    write_back_to_client(mbuf, ((*f).databuffer).as_mut_ptr(), ret);
                    write_log(
                        (*f).loginfo,
                        (*f).idx,
                        (*mbuf).td as *const uchar,
                        (*mbuf).dlen - 1 as libc::c_int,
                        (*mbuf).qtype as libc::c_int,
                        (*mbuf).addr,
                    );
                    mbuf_free(mbuf);
                } else {
                    if (*mbuf).socktype == 1 as libc::c_uint {
                        fd = (*mbuf).fd;
                        (*mbuf).fd = -(1 as libc::c_int);
                    }
                    tmp = lock_and_add_to_quizz(mbuf, f);
                    if tmp < 0 as libc::c_int {
                        (*f).miss = ((*f).miss).wrapping_add(1);
                        mbuf_free(mbuf);
                    }
                }
                if fd != -(1 as libc::c_int) {
                    delete_close_event(fd, f);
                }
            }
        }
    };
}
pub static mut global_now: time_t = 0 as libc::c_int as time_t;
pub static mut gnlock: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
    __data: __pthread_mutex_s {
        __lock: 0,
        __count: 0,
        __owner: 0,
        __nusers: 0,
        __kind: 0,
        __spins: 0,
        __elision: 0,
        __list: __pthread_list_t {
            __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
            __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
        },
    },
};
pub static mut refresh_record: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn daemonrize(mut dm: libc::c_int) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    if dm == 1 as libc::c_int {
        tmp = daemon(1 as libc::c_int, 0 as libc::c_int);
        if tmp == -(1 as libc::c_int) {
            dns_error(
                0 as libc::c_int,
                b"daemonrize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else {
            printf(b"daemon!!!\n\0" as *const u8 as *const libc::c_char);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn create_listen_ports(
    mut port: libc::c_int,
    mut proto: libc::c_int,
    mut addr: *mut uchar,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    fd = -(1 as libc::c_int);
    fd = create_socket(port, proto, addr);
    if fd < 0 as libc::c_int {
        printf(b"port:%d,proto:%d\n\0" as *const u8 as *const libc::c_char, port, proto);
        dns_error(
            0 as libc::c_int,
            b"fd < 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else {
        tmp = set_non_block(fd);
        if tmp < 0 as libc::c_int {
            printf(
                b"port:%d,proto:%d\n\0" as *const u8 as *const libc::c_char,
                port,
                proto,
            );
            dns_error(
                0 as libc::c_int,
                b"fd < 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
    }
    return fd;
}
pub unsafe extern "C" fn create_author(
    mut s: *mut server,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut authors: *mut author = 0 as *mut author;
    let mut cpuinfo: cpu_set_t = cpu_set_t { __bits: [0; 16] };
    let mut apt: [pthread_t; 2] = [0; 2];
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: libc::c_int = 0;
    let mut __cpu: size_t = 0;
    let mut tmp___2: libc::c_int = 0;
    authors = 0 as *mut libc::c_void as *mut author;
    if n < 1 as libc::c_int {
        dns_error(
            0 as libc::c_int,
            b"quizzer bad range\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    } else if n > 50 as libc::c_int {
        dns_error(
            0 as libc::c_int,
            b"quizzer bad range\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    tmp = malloc(
        (::std::mem::size_of::<author>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    );
    authors = tmp as *mut author;
    if authors as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        dns_error(
            0 as libc::c_int,
            b"out of memory in quizzer\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    memset(
        authors as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<author>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    );
    (*s).authors = authors;
    i = 0 as libc::c_int;
    while i < n {
        (*authors.offset(i as isize)).idx = i;
        (*authors.offset(i as isize)).cudp = (*s).ludp;
        (*authors.offset(i as isize))
            .audp = create_listen_ports(
            i * 1000 as libc::c_int + 998 as libc::c_int,
            2 as libc::c_int,
            0 as *mut libc::c_void as *mut uchar,
        );
        if (*authors.offset(i as isize)).audp < 0 as libc::c_int {
            dns_error(
                0 as libc::c_int,
                b"auth fd error\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        set_sock_buff((*authors.offset(i as isize)).audp, 1 as libc::c_int);
        let ref mut fresh27 = (*authors.offset(i as isize)).el;
        *fresh27 = &mut (*s).eventlist;
        let ref mut fresh28 = (*authors.offset(i as isize)).s;
        *fresh28 = s;
        get_random_data(
            ((*authors.offset(i as isize)).randombuffer).as_mut_ptr(),
            3000 as libc::c_int,
        );
        (*authors.offset(i as isize)).rndidx = 0 as libc::c_int;
        (*authors.offset(i as isize)).dupbefore = 0 as libc::c_int;
        (*authors.offset(i as isize)).limits = 10 as libc::c_int;
        (*authors.offset(i as isize)).bdepfd = 0 as libc::c_int;
        let ref mut fresh29 = (*authors.offset(i as isize)).fwd;
        *fresh29 = (*s).forward;
        let ref mut fresh30 = (*authors.offset(i as isize)).ds;
        *fresh30 = (*s).datasets;
        (*authors.offset(i as isize)).qnum = 0 as libc::c_int;
        (*authors.offset(i as isize)).underattack = 0 as libc::c_int;
        (*authors.offset(i as isize)).timex = 0 as libc::c_int;
        (*authors.offset(i as isize)).response = 0 as libc::c_int;
        (*authors.offset(i as isize)).tcpinuse = 0 as libc::c_int;
        (*authors.offset(i as isize)).rdb = 0 as libc::c_int as uint___0;
        (*authors.offset(i as isize)).quizz = 0 as libc::c_int as uint___0;
        (*authors.offset(i as isize)).drop = 0 as libc::c_int as uint___0;
        (*authors.offset(i as isize)).timeout = 0 as libc::c_int as uint___0;
        (*authors.offset(i as isize)).qidx = 0 as libc::c_int;
        (*authors.offset(i as isize)).start = 2047 as libc::c_int * i;
        if i == 1 as libc::c_int {
            (*authors.offset(i as isize)).end = 4095 as libc::c_int;
        } else {
            (*authors.offset(i as isize))
                .end = 2047 as libc::c_int * (i + 1 as libc::c_int);
        }
        memset(
            ((*authors.offset(i as isize)).ip).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            2000 as libc::c_int as size_t,
        );
        tmp___0 = malloc(::std::mem::size_of::<log_info>() as libc::c_ulong);
        let ref mut fresh31 = (*authors.offset(i as isize)).loginfo;
        *fresh31 = tmp___0 as *mut log_info;
        memset(
            (*authors.offset(i as isize)).loginfo as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<log_info>() as libc::c_ulong,
        );
        (*(*authors.offset(i as isize)).loginfo).log_type = 233 as libc::c_int;
        (*(*authors.offset(i as isize)).loginfo)
            .logfd = create_new_log(((*s).logpath).as_mut_ptr(), i, 233 as libc::c_int);
        j = 0 as libc::c_int;
        // while j < 101 as libc::c_int {
        //     pthread_spin_init(
        //         &mut *((*authors.offset(i as isize)).dblock)
        //             .as_mut_ptr()
        //             .offset(j as isize),
        //         0 as libc::c_int,
        //     );
        //     j += 1;
        // }
        j = 0 as libc::c_int;
        while j < 10000 as libc::c_int {
            let ref mut fresh32 = (*authors.offset(i as isize)).list[j as usize];
            *fresh32 = 0 as *mut libc::c_void as *mut qoutinfo;
            j += 1;
        }
        j = 0 as libc::c_int;
        while j < 65530 as libc::c_int {
            (*authors.offset(i as isize)).eptcpfds[j as usize].ret = -(1 as libc::c_int);
            j += 1;
        }
        pthread_spin_init(&mut (*authors.offset(i as isize)).lock, 0 as libc::c_int);
        (*(*authors.offset(i as isize)).loginfo).lastlog = global_now;
        if (*authors.offset(i as isize)).cudp < 0 as libc::c_int {
            dns_error(
                0 as libc::c_int,
                b"create quizzer2\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        } else if (*authors.offset(i as isize)).audp < 0 as libc::c_int {
            dns_error(
                0 as libc::c_int,
                b"create quizzer2\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        tmp___1 = pthread_create(
            apt.as_mut_ptr().offset(i as isize),
            0 as *mut libc::c_void as *const pthread_attr_t,
            Some(
                run_quizzer
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            authors.offset(i as isize) as *mut libc::c_void,
        );
        if tmp___1 != 0 as libc::c_int {
            dns_error(
                0 as libc::c_int,
                b"create quizzer\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        i += 1;
    }
    (*global_out_info).thread_num += i;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        libc::memset(
            &mut cpuinfo as *mut cpu_set_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong as libc::c_int
                as libc::c_ulong as libc::size_t,
        );
        __cpu = (i + 2 as libc::c_int + 1 as libc::c_int) as size_t;
        if __cpu.wrapping_div(8 as libc::c_ulong)
            < ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong
        {
            cpuinfo
                .__bits[__cpu
                .wrapping_div(
                    (8 as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                        ),
                ) as usize]
                |= (1 as libc::c_ulong)
                    << __cpu
                        .wrapping_rem(
                            (8 as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                ),
                        );
        }
        tmp___2 = pthread_setaffinity_np(
            apt[i as usize],
            ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong,
            &mut cpuinfo as *mut cpu_set_t as *const cpu_set_t,
        );
        if 0 as libc::c_int != tmp___2 {
            printf(
                b"set affinity quizzer failed, may be the cpu cores num less than (FETCHER_NUM + QUIZZER_NUM + 1)\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn create_fetcher(
    mut s: *mut server,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ws: *mut fetcher = 0 as *mut fetcher;
    let mut tmp: *mut fetcher = 0 as *mut fetcher;
    let mut cpuinfo: cpu_set_t = cpu_set_t { __bits: [0; 16] };
    let mut fpt: [pthread_t; 2] = [0; 2];
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: libc::c_int = 0;
    let mut __cpu: size_t = 0;
    let mut tmp___3: libc::c_int = 0;
    if n < 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    tmp___0 = malloc(
        (::std::mem::size_of::<fetcher>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    );
    ws = tmp___0 as *mut fetcher;
    if ws as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    memset(
        ws as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<fetcher>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    );
    (*s).fetchers = ws;
    i = 0 as libc::c_int;
    while i < n {
        tmp = ws.offset(i as isize);
        (*tmp).s = s;
        (*tmp).idx = i;
        (*tmp).pkg = 0 as libc::c_int as uint64_t;
        (*tmp).send = 0 as libc::c_int as uint64_t;
        (*tmp).miss = 0 as libc::c_int as uint64_t;
        (*tmp).el = &mut (*s).eventlist;
        (*tmp).qidx = i % 2 as libc::c_int;
        (*tmp).mc = init_msgcache(100 as libc::c_int);
        if (*tmp).mc as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            dns_error(
                0 as libc::c_int,
                b"get msgcache\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        tmp___1 = malloc(::std::mem::size_of::<log_info>() as libc::c_ulong);
        (*tmp).loginfo = tmp___1 as *mut log_info;
        memset(
            (*tmp).loginfo as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<log_info>() as libc::c_ulong,
        );
        (*(*tmp).loginfo).lastlog = global_now;
        (*(*tmp).loginfo).log_type = 112 as libc::c_int;
        (*(*tmp).loginfo)
            .logfd = create_new_log(((*s).logpath).as_mut_ptr(), i, 112 as libc::c_int);
        if (*(*tmp).loginfo).logfd < 0 as libc::c_int {
            dns_error(
                0 as libc::c_int,
                b"log file error\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        tmp___2 = pthread_create(
            fpt.as_mut_ptr().offset(i as isize),
            0 as *mut libc::c_void as *const pthread_attr_t,
            ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
            >(
                ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut fetcher) -> libc::c_int>,
                    *mut libc::c_void,
                >(Some(run_fetcher as unsafe extern "C" fn(*mut fetcher) -> libc::c_int)),
            ),
            tmp as *mut libc::c_void,
        );
        if tmp___2 != 0 as libc::c_int {
            dns_error(
                0 as libc::c_int,
                b"init worker\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        i += 1;
    }
    (*global_out_info).thread_num += i;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        libc::memset(
            &mut cpuinfo as *mut cpu_set_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong as libc::c_int
                as libc::c_ulong as libc::size_t,
        );
        __cpu = (i + 1 as libc::c_int) as size_t;
        if __cpu.wrapping_div(8 as libc::c_ulong)
            < ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong
        {
            cpuinfo
                .__bits[__cpu
                .wrapping_div(
                    (8 as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                        ),
                ) as usize]
                |= (1 as libc::c_ulong)
                    << __cpu
                        .wrapping_rem(
                            (8 as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                ),
                        );
        }
        tmp___3 = pthread_setaffinity_np(
            fpt[i as usize],
            ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong,
            &mut cpuinfo as *mut cpu_set_t as *const cpu_set_t,
        );
        if 0 as libc::c_int != tmp___3 {
            printf(
                b"set affinity fetcher failed,  may be the cpu cores num less than (FETCHER_NUM + QUIZZER_NUM + 1)\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn server_init() -> *mut server {
    let mut s: *mut server = 0 as *mut server;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    tmp = malloc(::std::mem::size_of::<server>() as libc::c_ulong);
    s = tmp as *mut server;
    if s as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        dns_error(
            0 as libc::c_int,
            b"out of memory in server_init\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    (*s).nfetcher = 2 as libc::c_int as ushort___0;
    (*s).nquizzer = 2 as libc::c_int as ushort___0;
    (*s).authors = 0 as *mut libc::c_void as *mut author;
    (*s).fetchers = 0 as *mut libc::c_void as *mut fetcher;
    (*s).pkg = 0 as libc::c_int as ulong___0;
    pthread_spin_init(&mut (*s).eventlist.lock, 0 as libc::c_int);
    (*s).eventlist.head = 0 as *mut libc::c_void as *mut list_node;
    tmp___0 = create_listen_ports(
        53 as libc::c_int,
        2 as libc::c_int,
        b"0.0.0.0\0" as *const u8 as *const libc::c_char as *mut uchar,
    );
    (*s).ludp = tmp___0;
    if tmp___0 < 0 as libc::c_int {
        dns_error(
            0 as libc::c_int,
            b"can not open udp\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    set_sock_buff((*s).ludp, 10 as libc::c_int);
    tmp___1 = create_listen_ports(
        53 as libc::c_int,
        1 as libc::c_int,
        b"0.0.0.0\0" as *const u8 as *const libc::c_char as *mut uchar,
    );
    (*s).ltcp = tmp___1;
    if tmp___1 < 0 as libc::c_int {
        dns_error(
            0 as libc::c_int,
            b"can not open tcp\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    (*s)
        .datasets = htable_create(
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<hashfunc>,
        >(0 as *mut libc::c_void),
        Some(
            dict_comp_str_equ
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        65536 as libc::c_int,
        10 as libc::c_int,
    );
    if (*s).datasets as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        dns_error(
            0 as libc::c_int,
            b"htable create\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    (*s)
        .forward = htable_create(
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<hashfunc>,
        >(0 as *mut libc::c_void),
        Some(
            dict_comp_str_equ
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        1024 as libc::c_int,
        1 as libc::c_int,
    );
    if (*s).forward as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        dns_error(
            0 as libc::c_int,
            b"create forward\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    (*s)
        .qlist = htable_create(
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<hashfunc>,
        >(0 as *mut libc::c_void),
        Some(
            dict_comp_str_equ
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        4095 as libc::c_int,
        1 as libc::c_int,
    );
    if (*s).qlist as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        dns_error(
            0 as libc::c_int,
            b"create qlist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    (*s)
        .ttlexp = create_rbtree(
        Some(
            rbt_comp_ttl_gt
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        0 as *mut libc::c_void,
    );
    if (*s).ttlexp as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        dns_error(
            0 as libc::c_int,
            b"create ttl tree\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    (*s).recordsindb = 0 as libc::c_int as ulong___0;
    (*s).refreshflag = 0 as libc::c_int as uint16_t;
    (*s).lastrefresh = global_now;
    (*s).is_forward = 0 as libc::c_int;
    return s;
}
pub unsafe extern "C" fn time_cron(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut s: *mut server = 0 as *mut server;
    let mut tv: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut waitset: sigset_t = sigset_t { __val: [0; 16] };
    let mut info: siginfo_t = siginfo_t {
        si_signo: 0,
        si_errno: 0,
        si_code: 0,
        __pad0: 0,
        _sifields: __anonunion__sifields_998240460 {
            _pad: [0; 28],
        },
    };
    let mut ret: libc::c_int = 0;
    s = arg as *mut server;
    tv.tv_sec = 0 as libc::c_int as __time_t;
    tv.tv_nsec = 0 as libc::c_long;
    sigemptyset(&mut waitset);
    sigaddset(&mut waitset, 10 as libc::c_int);
    global_now = time(0 as *mut libc::c_void as *mut time_t);
    loop {
        tv.tv_sec = 1 as libc::c_int as __time_t;
        tv.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
        ret = sigtimedwait(
            &mut waitset as *mut sigset_t as *const sigset_t,
            &mut info as *mut siginfo_t,
            &mut tv as *mut timespec as *const timespec,
        );
        if ret > 0 as libc::c_int {
            (*s).refreshflag = 1 as libc::c_int as uint16_t;
        }
        global_now = time(0 as *mut libc::c_void as *mut time_t);
    };
}
pub unsafe extern "C" fn recv_update(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut s: *mut server = 0 as *mut server;
    s = arg as *mut server;
    start_local_server(s);
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn sanity_test(mut exi: libc::c_int) -> libc::c_int {
    if exi != 0 {
        exit(0 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn print_basic_debug() -> libc::c_int {
    printf(
        b"[DBG:] dnspod-sr is successfully running now!!\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"[DBG:] max_ele_size is %u - 1808\n\0" as *const u8 as *const libc::c_char,
        MAX_ELE_NUM,
    );
    printf(
        b"[DBG:] server may contain %u useful records\n\0" as *const u8
            as *const libc::c_char,
        MAX_ELE_NUM.wrapping_sub(1808 as libc::c_uint).wrapping_div(3 as libc::c_uint),
    );
    printf(
        b"[DBG:] hash_table_size is %u\n\0" as *const u8 as *const libc::c_char,
        65536 as libc::c_int,
    );
    printf(
        b"[DBG:] we have %u hash tables\n\0" as *const u8 as *const libc::c_char,
        10 as libc::c_int,
    );
    printf(
        b"[DBG:] we have %u fetchers,%u quizzers\n\0" as *const u8
            as *const libc::c_char,
        2 as libc::c_int,
        2 as libc::c_int,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn help(mut progname: *const libc::c_char) {
    printf(b"DNSPod recursive dns server\n\0" as *const u8 as *const libc::c_char);
    printf(b"version 0.01\n\0" as *const u8 as *const libc::c_char);
    printf(b"Usage: %s [-c config]\n\0" as *const u8 as *const libc::c_char, progname);
}
pub unsafe extern "C" fn init_globe() -> libc::c_int {
    let mut shmid: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    shmid = shmget(
        38899 as libc::c_int,
        ::std::mem::size_of::<global_query_info>() as libc::c_ulong,
        896 as libc::c_int,
    );
    if shmid < 0 as libc::c_int {
        printf(
            b"%lu\n\0" as *const u8 as *const libc::c_char,
            (38899 as libc::c_ulong)
                .wrapping_add(
                    ::std::mem::size_of::<global_query_info>() as libc::c_ulong,
                ),
        );
        perror(b"shmget\0" as *const u8 as *const libc::c_char);
        dns_error(
            0 as libc::c_int,
            b"shmget error\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    tmp = shmat(shmid, 0 as *mut libc::c_void as *const libc::c_void, 0 as libc::c_int);
    global_out_info = tmp as *mut global_query_info;
    memset(
        global_out_info as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<global_query_info>() as libc::c_ulong,
    );
    (*global_out_info).thread_num = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[libc::c_int; 256]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
    {
        query_type_map[i as usize] = -(1 as libc::c_int);
        i += 1;
    }
    query_type_map[1 as libc::c_int as usize] = 0 as libc::c_int;
    query_type_map[2 as libc::c_int as usize] = 1 as libc::c_int;
    query_type_map[5 as libc::c_int as usize] = 2 as libc::c_int;
    query_type_map[6 as libc::c_int as usize] = 3 as libc::c_int;
    query_type_map[15 as libc::c_int as usize] = 4 as libc::c_int;
    query_type_map[16 as libc::c_int as usize] = 5 as libc::c_int;
    query_type_map[28 as libc::c_int as usize] = 6 as libc::c_int;
    query_type_map[33 as libc::c_int as usize] = 7 as libc::c_int;
    query_type_map[255 as libc::c_int as usize] = 8 as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn init_mempool() {
    let mut ret: libc::c_int = 0;
    ret = mempool_create(65536 as libc::c_int as uint32_t);
    if ret < 0 as libc::c_int {
        dns_error(
            0 as libc::c_int,
            b"create mempool failed\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut s: *mut server = 0 as *mut server;
    let mut pt: pthread_t = 0;
    let mut ctl: pthread_t = 0;
    let mut c: libc::c_int = 0;
    let mut is_forward: libc::c_int = 0;
    let mut config: *const libc::c_char = 0 as *const libc::c_char;
    let mut daemon___0: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    s = 0 as *mut libc::c_void as *mut server;
    is_forward = 0 as libc::c_int;
    config = b"../sr.conf\0" as *const u8 as *const libc::c_char;
    daemon___0 = 0 as libc::c_int;
    loop {
        c = getopt(
            argc,
            argv as *const *mut libc::c_char,
            b"c:vhfd\0" as *const u8 as *const libc::c_char,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            99 => {
                config = optarg as *const libc::c_char;
            }
            104 => {
                help(*argv.offset(0 as libc::c_int as isize) as *const libc::c_char);
                exit(0 as libc::c_int);
            }
            102 => {
                is_forward = 1 as libc::c_int;
            }
            100 => {
                daemon___0 = 1 as libc::c_int;
            }
            63 => {
                printf(b"Try -h please\n\0" as *const u8 as *const libc::c_char);
                exit(0 as libc::c_int);
            }
            118 => {
                printf(b"dnspod-sr 0.01\n\0" as *const u8 as *const libc::c_char);
                exit(0 as libc::c_int);
            }
            _ => {
                exit(0 as libc::c_int);
            }
        }
    }
    sanity_test(0 as libc::c_int);
    drop_privilege(b"./\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    daemonrize(daemon___0);
    trig_signals(1 as libc::c_int);
    global_now = time(0 as *mut libc::c_void as *mut time_t);
    g_nameservers[1 as libc::c_int
        as usize] = 0 as *mut libc::c_void as *mut libc::c_char;
    g_nameservers[0 as libc::c_int as usize] = g_nameservers[1 as libc::c_int as usize];
    init_globe();
    init_mempool();
    s = server_init();
    (*s).is_forward = is_forward;
    read_config(
        config,
        ((*s).logpath).as_mut_ptr() as *mut libc::c_char,
        (*s).forward,
        g_nameservers.as_mut_ptr(),
    );
    if g_nameservers[0 as libc::c_int as usize] as libc::c_ulong
        == 0 as *mut libc::c_void as libc::c_ulong
    {
        if !(g_nameservers[1 as libc::c_int as usize] as libc::c_ulong
            == 0 as *mut libc::c_void as libc::c_ulong)
        {
            __assert_fail(
                b"g_nameservers[1] == NULL\0" as *const u8 as *const libc::c_char,
                b"init.c\0" as *const u8 as *const libc::c_char,
                412 as libc::c_uint,
                b"main\0" as *const u8 as *const libc::c_char,
            );
        }
        g_nameservers[0 as libc::c_int
            as usize] = strdup(b"119.29.29.29\0" as *const u8 as *const libc::c_char);
        g_nameservers[1 as libc::c_int
            as usize] = strdup(b"8.8.4.4\0" as *const u8 as *const libc::c_char);
    }
    if g_nameservers[1 as libc::c_int as usize] as libc::c_ulong
        == 0 as *mut libc::c_void as libc::c_ulong
    {
        tmp___0 = strcmp(
            g_nameservers[0 as libc::c_int as usize] as *const libc::c_char,
            b"119.29.29.29\0" as *const u8 as *const libc::c_char,
        );
        if tmp___0 == 0 as libc::c_int {
            g_nameservers[1 as libc::c_int
                as usize] = strdup(b"8.8.4.4\0" as *const u8 as *const libc::c_char);
        } else {
            g_nameservers[1 as libc::c_int
                as usize] = strdup(
                b"119.29.29.29\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    tmp___1 = create_fetcher(s, (*s).nfetcher as libc::c_int);
    if tmp___1 < 0 as libc::c_int {
        dns_error(
            0 as libc::c_int,
            b"create worker\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    tmp___2 = create_author(s, (*s).nquizzer as libc::c_int);
    if tmp___2 < 0 as libc::c_int {
        dns_error(
            0 as libc::c_int,
            b"create author\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    tmp___3 = pthread_create(
        &mut pt as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        >(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
                *mut libc::c_void,
            >(
                Some(
                    time_cron
                        as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                ),
            ),
        ),
        s as *mut libc::c_void,
    );
    if tmp___3 != 0 as libc::c_int {
        dns_error(
            0 as libc::c_int,
            b"time cron error\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    tmp___4 = pthread_create(
        &mut ctl as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        >(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
                *mut libc::c_void,
            >(
                Some(
                    recv_update
                        as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                ),
            ),
        ),
        s as *mut libc::c_void,
    );
    if tmp___4 != 0 as libc::c_int {
        dns_error(
            0 as libc::c_int,
            b"recv update thread error\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    read_root((*s).datasets, (*s).ttlexp);
    print_basic_debug();
    global_serv = s;
    run_sentinel(s);
    return 0 as libc::c_int;
}
pub static mut local_socket_name: *const libc::c_char = b"/tmp/foo.socket\0" as *const u8
    as *const libc::c_char;
pub unsafe extern "C" fn create_local_server(
    mut path: *const libc::c_char,
) -> libc::c_int {
    let mut sock: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut addr: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut tmp: size_t = 0;
    sock = socket(1 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int);
    if sock < 0 as libc::c_int {
        perror(b"socket error\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    memset(
        &mut addr as *mut sockaddr_un as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong,
    );
    addr.sun_family = 1 as libc::c_int as sa_family_t;
    strcpy((addr.sun_path).as_mut_ptr(), path);
    tmp = strlen(path);
    size = (&mut (*(0 as *mut sockaddr_un)).sun_path as *mut [libc::c_char; 108]
        as libc::c_ulong)
        .wrapping_add(tmp) as libc::c_int;
    unlink((addr.sun_path).as_mut_ptr() as *const libc::c_char);
    ret = bind(
        sock,
        &mut addr as *mut sockaddr_un as *mut sockaddr as *const sockaddr,
        size as socklen_t,
    );
    if ret < 0 as libc::c_int {
        perror(b"bind error\0" as *const u8 as *const libc::c_char);
        close(sock);
        return -(2 as libc::c_int);
    }
    ret = listen(sock, 10 as libc::c_int);
    if ret < 0 as libc::c_int {
        perror(b"listen error\0" as *const u8 as *const libc::c_char);
        close(sock);
        return -(3 as libc::c_int);
    }
    return sock;
}
pub unsafe extern "C" fn ctl_fd(
    mut epfd: libc::c_int,
    mut fd: libc::c_int,
    mut ctl: libc::c_int,
    mut events: uint32_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut ev: epoll_event = epoll_event {
        events: 0,
        data: epoll_data {
            ptr: 0 as *mut libc::c_void,
        },
    };
    memset(
        &mut ev as *mut epoll_event as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<epoll_event>() as libc::c_ulong,
    );
    ev.data.fd = fd;
    if ctl == 1 as libc::c_int {
        ev.events = events;
    }
    ret = epoll_ctl(epfd, ctl, fd, &mut ev);
    if ret < 0 as libc::c_int {
        perror(b"epoll_ctl\0" as *const u8 as *const libc::c_char);
        fprintf(stderr, b"ctl fd %d error\n\0" as *const u8 as *const libc::c_char, fd);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn accept_client(
    mut epfd: libc::c_int,
    mut sock: libc::c_int,
) -> libc::c_int {
    let mut clifd: libc::c_int = 0;
    let mut len: socklen_t = 0;
    let mut cli_addr: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut statbuf: stat = stat {
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
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    tmp = 0 as *mut libc::c_void as *mut libc::c_char;
    len = ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong as socklen_t;
    clifd = accept(
        sock,
        &mut cli_addr as *mut sockaddr_un as *mut sockaddr,
        &mut len as *mut socklen_t,
    );
    if clifd < 0 as libc::c_int {
        fprintf(stderr, b"accept error\n\0" as *const u8 as *const libc::c_char);
        close(clifd);
        return -(1 as libc::c_int);
    }
    len = (len as libc::c_ulong)
        .wrapping_sub(
            &mut (*(0 as *mut sockaddr_un)).sun_path as *mut [libc::c_char; 108]
                as libc::c_ulong,
        ) as socklen_t;
    if len > 0 as libc::c_uint {
        cli_addr.sun_path[len as usize] = '\u{0}' as i32 as libc::c_char;
        tmp___0 = stat(
            (cli_addr.sun_path).as_mut_ptr() as *const libc::c_char,
            &mut statbuf as *mut stat,
        );
        if tmp___0 < 0 as libc::c_int {
            fprintf(stderr, b"no this socket\n\0" as *const u8 as *const libc::c_char);
            close(clifd);
            return -(2 as libc::c_int);
        }
        if (statbuf.st_mode & 61440 as libc::c_uint == 49152 as libc::c_uint)
            as libc::c_int == 0 as libc::c_int
        {
            fprintf(stderr, b"not a socket\n\0" as *const u8 as *const libc::c_char);
            close(clifd);
            return -(3 as libc::c_int);
        }
        unlink((cli_addr.sun_path).as_mut_ptr() as *const libc::c_char);
        tmp = strrchr(
            (cli_addr.sun_path).as_mut_ptr() as *const libc::c_char,
            '/' as i32,
        );
        if !tmp.is_null() {
            printf(
                b"client pid: %s connected: %d\n\0" as *const u8 as *const libc::c_char,
                tmp.offset(1 as libc::c_int as isize),
                clifd,
            );
        }
    } else {
        printf(
            b"new client (unknown pid) connected: %d\n\0" as *const u8
                as *const libc::c_char,
            clifd,
        );
    }
    tmp___1 = ctl_fd(epfd, clifd, 1 as libc::c_int, 1 as libc::c_int as uint32_t);
    if tmp___1 != 0 {
        close(clifd);
        return -(1 as libc::c_int);
    }
    tv.tv_sec = 3 as libc::c_int as __time_t;
    tv.tv_usec = 0 as libc::c_int as __suseconds_t;
    setsockopt(
        clifd,
        1 as libc::c_int,
        20 as libc::c_int,
        &mut tv as *mut timeval as *const libc::c_void,
        ::std::mem::size_of::<timeval>() as libc::c_ulong as socklen_t,
    );
    return clifd;
}
pub unsafe extern "C" fn disconnect_client(
    mut epfd: libc::c_int,
    mut client: libc::c_int,
) {
    ctl_fd(epfd, client, 2 as libc::c_int, 0 as libc::c_int as uint32_t);
    close(client);
    printf(b"disconnect with client %d\n\0" as *const u8 as *const libc::c_char, client);
}
pub unsafe extern "C" fn get_type_from_str(
    mut str_type: *const libc::c_char,
) -> uint16_t {
    let mut type_0: uint16_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    type_0 = 0 as libc::c_int as uint16_t;
    if str_type.is_null() {
        type_0 = 1 as libc::c_int as uint16_t;
    } else if *str_type.offset(0 as libc::c_int as isize) as libc::c_int
            == 0 as libc::c_int
        {
        type_0 = 1 as libc::c_int as uint16_t;
    } else {
        tmp___7 = strcmp(str_type, b"A\0" as *const u8 as *const libc::c_char);
        if tmp___7 == 0 as libc::c_int {
            type_0 = 1 as libc::c_int as uint16_t;
        } else {
            tmp___6 = strcmp(str_type, b"CNAME\0" as *const u8 as *const libc::c_char);
            if tmp___6 == 0 as libc::c_int {
                type_0 = 5 as libc::c_int as uint16_t;
            } else {
                tmp___5 = strcmp(
                    str_type,
                    b"AAAA\0" as *const u8 as *const libc::c_char,
                );
                if tmp___5 == 0 as libc::c_int {
                    type_0 = 28 as libc::c_int as uint16_t;
                } else {
                    tmp___4 = strcmp(
                        str_type,
                        b"MX\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___4 == 0 as libc::c_int {
                        type_0 = 15 as libc::c_int as uint16_t;
                    } else {
                        tmp___3 = strcmp(
                            str_type,
                            b"TXT\0" as *const u8 as *const libc::c_char,
                        );
                        if tmp___3 == 0 as libc::c_int {
                            type_0 = 16 as libc::c_int as uint16_t;
                        } else {
                            tmp___2 = strcmp(
                                str_type,
                                b"SRV\0" as *const u8 as *const libc::c_char,
                            );
                            if tmp___2 == 0 as libc::c_int {
                                type_0 = 33 as libc::c_int as uint16_t;
                            } else {
                                tmp___1 = strcmp(
                                    str_type,
                                    b"NS\0" as *const u8 as *const libc::c_char,
                                );
                                if tmp___1 == 0 as libc::c_int {
                                    type_0 = 2 as libc::c_int as uint16_t;
                                } else {
                                    tmp___0 = strcmp(
                                        str_type,
                                        b"SOA\0" as *const u8 as *const libc::c_char,
                                    );
                                    if tmp___0 == 0 as libc::c_int {
                                        type_0 = 6 as libc::c_int as uint16_t;
                                    } else {
                                        tmp = strcmp(
                                            str_type,
                                            b"PTR\0" as *const u8 as *const libc::c_char,
                                        );
                                        if tmp == 0 as libc::c_int {
                                            type_0 = 12 as libc::c_int as uint16_t;
                                        } else {
                                            dns_error(
                                                1 as libc::c_int,
                                                b"invalid cache flush type\0" as *const u8
                                                    as *const libc::c_char as *mut libc::c_char,
                                            );
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
    return type_0;
}
pub unsafe extern "C" fn cmd_analyze(
    mut str: *mut libc::c_char,
    mut domain: *mut uchar,
    mut type_0: *mut uint16_t,
) -> libc::c_int {
    let mut p: *mut uchar = 0 as *mut uchar;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp: *mut uchar = 0 as *mut uchar;
    let mut str_type: [uchar; 32] = [0; 32];
    let mut tmp___0: libc::c_uint = 0;
    let mut len: size_t = 0;
    let mut cmd_type: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    tmp = strchr(str as *const libc::c_char, ':' as i32);
    p = tmp as *mut uchar;
    temp = 0 as *mut libc::c_void as *mut uchar;
    str_type[0 as libc::c_int as usize] = 0 as libc::c_int as uchar;
    tmp___0 = 1 as libc::c_uint;
    while !(tmp___0 >= 32 as libc::c_uint) {
        str_type[tmp___0 as usize] = 0 as libc::c_int as libc::c_uchar;
        tmp___0 = tmp___0.wrapping_add(1);
    }
    cmd_type = -(1 as libc::c_int);
    if p.is_null() {
        tmp___1 = strcmp(
            str as *const libc::c_char,
            b"hijack\0" as *const u8 as *const libc::c_char,
        );
        if tmp___1 == 0 as libc::c_int {
            cmd_type = 2 as libc::c_int;
        }
    } else {
        tmp___3 = strncmp(
            str as *const libc::c_char,
            b"cache flush\0" as *const u8 as *const libc::c_char,
            11 as libc::c_int as size_t,
        );
        if tmp___3 == 0 as libc::c_int {
            cmd_type = 1 as libc::c_int;
        } else {
            tmp___2 = strncmp(
                str as *const libc::c_char,
                b"hijack\0" as *const u8 as *const libc::c_char,
                6 as libc::c_int as size_t,
            );
            if tmp___2 == 0 as libc::c_int {
                cmd_type = 2 as libc::c_int;
            }
        }
        p = p.offset(1);
        temp = jump_space(p);
        fix_tail(temp as *mut libc::c_char);
        sscanf(
            temp as *const libc::c_char,
            b"%s %s\0" as *const u8 as *const libc::c_char,
            domain,
            str_type.as_mut_ptr(),
        );
        *type_0 = get_type_from_str(str_type.as_mut_ptr() as *const libc::c_char);
        len = strlen(domain as *const libc::c_char);
        if *domain.offset(len.wrapping_sub(1 as libc::c_ulong) as isize) as libc::c_int
            != 46 as libc::c_int
        {
            *domain.offset(len as isize) = '.' as i32 as uchar;
            *domain
                .offset(
                    len.wrapping_add(1 as libc::c_ulong) as isize,
                ) = '\u{0}' as i32 as uchar;
        }
    }
    return cmd_type;
}
pub unsafe extern "C" fn talk_with_client(
    mut epfd: libc::c_int,
    mut client: libc::c_int,
    mut s: *mut server,
) -> libc::c_int {
    let mut buffer: [libc::c_char; 8192] = [0; 8192];
    let mut tmp: libc::c_uint = 0;
    let mut ret: libc::c_int = 0;
    let mut tmp___0: ssize_t = 0;
    let mut domain: [uchar; 512] = [0; 512];
    let mut tmp___1: libc::c_uint = 0;
    let mut type_0: uint16_t = 0;
    let mut ret___0: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: size_t = 0;
    let mut tmp___4: size_t = 0;
    buffer[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    tmp = 1 as libc::c_uint;
    while !(tmp >= 8192 as libc::c_uint) {
        buffer[tmp as usize] = 0 as libc::c_int as libc::c_char;
        tmp = tmp.wrapping_add(1);
    }
    tmp___0 = recv(
        client,
        buffer.as_mut_ptr() as *mut libc::c_void,
        8191 as libc::c_int as size_t,
        0 as libc::c_int,
    );
    ret = tmp___0 as libc::c_int;
    if ret == 0 as libc::c_int {
        disconnect_client(epfd, client);
    }
    if ret > 0 as libc::c_int {
        printf(
            b"recv from client [%d] %d bytes: %s\n\0" as *const u8
                as *const libc::c_char,
            client,
            ret,
            buffer.as_mut_ptr(),
        );
        domain[0 as libc::c_int as usize] = 0 as libc::c_int as uchar;
        tmp___1 = 1 as libc::c_uint;
        while !(tmp___1 >= 512 as libc::c_uint) {
            domain[tmp___1 as usize] = 0 as libc::c_int as libc::c_uchar;
            tmp___1 = tmp___1.wrapping_add(1);
        }
        type_0 = 0 as libc::c_int as uint16_t;
        tmp___2 = cmd_analyze(buffer.as_mut_ptr(), domain.as_mut_ptr(), &mut type_0);
        ret___0 = tmp___2;
        if ret___0 == 2 as libc::c_int {
            hijack(domain.as_mut_ptr(), type_0, (*s).datasets, (*s).ttlexp);
        } else if ret___0 == 1 as libc::c_int {
            if type_0 as libc::c_int != 0 as libc::c_int {
                tmp___3 = strlen(
                    domain.as_mut_ptr() as *mut libc::c_char as *const libc::c_char,
                );
                if tmp___3 > 3 as libc::c_ulong {
                    cache_flush(domain.as_mut_ptr(), type_0, (*s).datasets, (*s).ttlexp);
                }
            }
        }
        tmp___4 = strlen(buffer.as_mut_ptr() as *const libc::c_char);
        send(
            client,
            buffer.as_mut_ptr() as *const libc::c_void,
            tmp___4,
            0 as libc::c_int,
        );
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn start_local_server(mut s: *mut server) -> libc::c_int {
    let mut server: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut epfd: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    let mut e: [epoll_event; 1024] = [epoll_event {
        events: 0,
        data: epoll_data {
            ptr: 0 as *mut libc::c_void,
        },
    }; 1024];
    let mut tmp: libc::c_int = 0;
    server = create_local_server(local_socket_name);
    if server < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    epfd = epoll_create(1 as libc::c_int);
    if epfd < 0 as libc::c_int {
        perror(b"epoll_create error\0" as *const u8 as *const libc::c_char);
        close(server);
        return -(1 as libc::c_int);
    }
    tmp = ctl_fd(epfd, server, 1 as libc::c_int, 1 as libc::c_int as uint32_t);
    if tmp != 0 as libc::c_int {
        close(server);
        return -(1 as libc::c_int);
    }
    loop {
        ret = epoll_wait(epfd, e.as_mut_ptr(), 1024 as libc::c_int, 1000 as libc::c_int);
        if !(ret < 0 as libc::c_int) {
            i = 0 as libc::c_int;
            while i < ret {
                fd = e[i as usize].data.fd;
                if fd == server {
                    accept_client(epfd, server);
                } else if e[i as usize].events & 16 as libc::c_uint != 0 {
                    disconnect_client(epfd, fd);
                } else if e[i as usize].events & 8 as libc::c_uint != 0 {
                    disconnect_client(epfd, fd);
                } else if e[i as usize].events & 1 as libc::c_uint != 0 {
                    talk_with_client(epfd, fd, s);
                }
                i += 1;
            }
        }
    };
}
pub unsafe extern "C" fn refresh_ttl_with_td(
    mut key: *mut uchar,
    mut len: libc::c_int,
    mut type_0: libc::c_int,
    mut ht: *mut htable,
    mut ttlexp: *mut rbtree,
    mut lowerdomain: *mut packet_type,
) -> libc::c_int {
    let mut tmp: uint___0 = 0;
    let mut tmp___0: uint___0 = 0;
    pthread_spin_lock(&mut (*ttlexp).lock);
    tmp = get_rbt_size(ttlexp);
    printf(
        b"after delete, before insert, rbt size: %d\n\0" as *const u8
            as *const libc::c_char,
        tmp,
    );
    insert_into_ttltree(
        ttlexp,
        key,
        len,
        type_0,
        0 as libc::c_int as uint___0,
        lowerdomain,
    );
    tmp___0 = get_rbt_size(ttlexp);
    printf(
        b"after insert, rbt size: %d\n\0" as *const u8 as *const libc::c_char,
        tmp___0,
    );
    pthread_spin_unlock(&mut (*ttlexp).lock);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn hijack(
    mut domain: *mut uchar,
    mut type_0: uint16_t,
    mut ht: *mut htable,
    mut ttlexp: *mut rbtree,
) -> libc::c_int {
    let mut tmp: __pid_t = 0;
    if domain.is_null() {
        tmp = getpid();
        kill(tmp, 10 as libc::c_int);
    } else if *domain.offset(0 as libc::c_int as isize) as libc::c_int
            == 0 as libc::c_int
        {
        tmp = getpid();
        kill(tmp, 10 as libc::c_int);
    } else if type_0 as libc::c_int <= 0 as libc::c_int {
        tmp = getpid();
        kill(tmp, 10 as libc::c_int);
    } else if type_0 as libc::c_int > 255 as libc::c_int {
        tmp = getpid();
        kill(tmp, 10 as libc::c_int);
    } else {
        cache_flush(domain, type_0, ht, ttlexp);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn cache_flush(
    mut domain: *mut uchar,
    mut type_0: uint16_t,
    mut ht: *mut htable,
    mut ttlexp: *mut rbtree,
) -> libc::c_int {
    let mut dlen: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut hash: hashval_t = 0;
    let mut lowerdomain: packet_type = packet_type {
        label_count: 0,
        domain: [0; 256],
        label: [0 as *mut uint8_t; 64],
        label_offsets: [0; 64],
        label_len: [0; 64],
        hash: [0; 64],
    };
    let mut idx: libc::c_int = 0;
    let mut tmp___0: uint___0 = 0;
    let mut val: *mut uchar = 0 as *mut uchar;
    let mut tmp___1: *mut uchar = 0 as *mut uchar;
    let mut tmp___2: *mut mvalue = 0 as *mut mvalue;
    let mut tn: ttlnode = ttlnode {
        exp: 0,
        dlen: 0,
        type_0: 0,
        hash: 0 as *mut hashval_t,
        lowerdomain: 0 as *mut packet_type,
        data: 0 as *mut uchar,
    };
    let mut tmp_tn: *mut ttlnode = 0 as *mut ttlnode;
    let mut pn: *mut rbnode = 0 as *mut rbnode;
    let mut tmp___3: *mut rbnode = 0 as *mut rbnode;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    printf(b"cache flush domain %s\n\0" as *const u8 as *const libc::c_char, domain);
    tmp = strlen(domain as *const libc::c_char);
    dlen = tmp.wrapping_add(1 as libc::c_ulong) as libc::c_int;
    hash = 0 as libc::c_int as hashval_t;
    str_to_len_label(domain, dlen);
    check_dns_name(domain, &mut lowerdomain);
    domain = (lowerdomain.domain).as_mut_ptr();
    tmp___0 = get_pre_mem_hash(domain as *mut libc::c_void, dlen, &mut hash);
    idx = tmp___0 as libc::c_int;
    tmp___1 = htable_delete(
        ht.offset(idx as isize),
        domain,
        dlen,
        type_0 as libc::c_int,
        hash,
    );
    val = tmp___1;
    if !val.is_null() {
        tmp___2 = val as *mut mvalue;
        tn.exp = 0 as libc::c_int as uint___0;
        tn.dlen = 0 as libc::c_int as libc::c_ushort;
        tn.type_0 = 0 as libc::c_int as libc::c_ushort;
        tn.hash = 0 as *mut hashval_t;
        tn.lowerdomain = 0 as *mut packet_type;
        tn.data = 0 as *mut uchar;
        tmp_tn = 0 as *mut libc::c_void as *mut ttlnode;
        pthread_spin_lock(&mut (*ttlexp).lock);
        tn.dlen = dlen as ushort___0;
        tn.exp = (*tmp___2).ttl;
        tn.type_0 = type_0;
        tn.data = domain;
        tn.lowerdomain = 0 as *mut libc::c_void as *mut packet_type;
        tmp___3 = find_node(ttlexp, &mut tn as *mut ttlnode as *mut libc::c_void);
        pn = tmp___3;
        if pn as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            tmp___4 = delete_node(ttlexp, pn);
            tmp_tn = tmp___4 as *mut ttlnode;
            if !tmp_tn.is_null() {
                free((*tmp_tn).lowerdomain as *mut libc::c_void);
                free(tmp_tn as *mut libc::c_void);
            } else {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"control.c\0" as *const u8 as *const libc::c_char,
                    70 as libc::c_uint,
                    b"cache_flush\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        pthread_spin_unlock(&mut (*ttlexp).lock);
        free(val as *mut libc::c_void);
        refresh_ttl_with_td(
            domain,
            dlen,
            type_0 as libc::c_int,
            ht,
            ttlexp,
            &mut lowerdomain,
        );
    }
    return 0 as libc::c_int;
}
pub static mut mbuf_ring: *mut mbuf_ring = 0 as *const libc::c_void as *mut libc::c_void
    as *mut mbuf_ring;
pub unsafe extern "C" fn mbuf_ring_create(mut count: uint32_t) -> *mut mbuf_ring {
    let mut r: *mut mbuf_ring = 0 as *mut mbuf_ring;
    let mut ring_size: uint64_t = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: uint32_t = 0;
    let mut tmp___1: uint32_t = 0;
    let mut tmp___2: uint32_t = 0;
    let mut tmp___3: uint32_t = 0;
    ring_size = (count as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<mbuf_ring>() as libc::c_ulong);
    tmp = malloc(ring_size);
    r = tmp as *mut mbuf_ring;
    if r as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        memset(
            r as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<mbuf_ring>() as libc::c_ulong,
        );
        (*r).prod.watermark = count;
        tmp___0 = count;
        (*r).cons.size = tmp___0;
        (*r).prod.size = tmp___0;
        tmp___1 = count.wrapping_sub(1 as libc::c_uint);
        (*r).cons.mask = tmp___1;
        (*r).prod.mask = tmp___1;
        tmp___2 = 0 as libc::c_int as uint32_t;
        (*r).cons.head = tmp___2;
        (*r).prod.head = tmp___2;
        tmp___3 = 0 as libc::c_int as uint32_t;
        (*r).cons.tail = tmp___3;
        (*r).prod.tail = tmp___3;
    }
    return r;
}
pub unsafe extern "C" fn mempool_create(mut num: uint32_t) -> libc::c_int {
    let mut tmp: *mut mbuf_type = 0 as *mut mbuf_type;
    let mut i: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: uint32_t = 0;
    mbuf_ring = mbuf_ring_create(num);
    if 0 as *mut libc::c_void as libc::c_ulong == mbuf_ring as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while (i as uint32_t) < num {
        tmp___0 = malloc(::std::mem::size_of::<mbuf_type>() as libc::c_ulong);
        tmp = tmp___0 as *mut mbuf_type;
        if 0 as *mut libc::c_void as libc::c_ulong == tmp as libc::c_ulong {
            return -(1 as libc::c_int);
        }
        (*tmp).mbuf = mbuf_ring;
        let ref mut fresh33 = *((*mbuf_ring).ring).as_mut_ptr().offset(i as isize);
        *fresh33 = tmp as *mut libc::c_void;
        i += 1;
    }
    tmp___1 = num.wrapping_sub(1 as libc::c_uint);
    (*mbuf_ring).prod.tail = tmp___1;
    (*mbuf_ring).prod.head = tmp___1;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn rte_atomic32_cmpset(
    mut dst: *mut uint32_t,
    mut exp: uint32_t,
    mut src: uint32_t,
) -> libc::c_int {
    let mut res: uint32_t = 0;
    res = std::intrinsics::atomic_cxchg(dst, exp, src).0;
    return res as libc::c_int;
}
pub unsafe extern "C" fn mbuf_alloc() -> *mut mbuf_type {
    let mut cons_head: uint32_t = 0;
    let mut prod_tail: uint32_t = 0;
    let mut cons_next: uint32_t = 0;
    let mut entries: uint32_t = 0;
    let mut success: libc::c_int = 0;
    let mut mask: uint32_t = 0;
    let mut mbuf: *mut mbuf_type = 0 as *mut mbuf_type;
    mask = (*mbuf_ring).prod.mask;
    cons_head = (*mbuf_ring).cons.head;
    prod_tail = (*mbuf_ring).prod.tail;
    entries = prod_tail.wrapping_sub(cons_head);
    if 0 as libc::c_uint == entries {
        dns_error(
            1 as libc::c_int,
            b"out of mbuf ring(memory pool)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 0 as *mut libc::c_void as *mut mbuf_type;
    }
    cons_next = cons_head.wrapping_add(1 as libc::c_uint);
    success = rte_atomic32_cmpset(&mut (*mbuf_ring).cons.head, cons_head, cons_next);
    if success != 1 as libc::c_int {
        return 0 as *mut libc::c_void as *mut mbuf_type;
    }
    mbuf = *((*mbuf_ring).ring).as_mut_ptr().offset((cons_head & mask) as isize)
        as *mut mbuf_type;
    if !(mbuf as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"mbuf != NULL\0" as *const u8 as *const libc::c_char,
            b"memory.c\0" as *const u8 as *const libc::c_char,
            129 as libc::c_uint,
            b"mbuf_alloc\0" as *const u8 as *const libc::c_char,
        );
    }
    asm!("", options(preserves_flags, att_syntax));
    while (*mbuf_ring).cons.tail != cons_head {}
    (*mbuf_ring).cons.tail = cons_next;
    return mbuf;
}
pub unsafe extern "C" fn mbuf_free(mut mbuf: *mut mbuf_type) -> libc::c_int {
    let mut prod_head: uint32_t = 0;
    let mut prod_next: uint32_t = 0;
    let mut cons_tail: uint32_t = 0;
    let mut free_entries: uint32_t = 0;
    let mut success: libc::c_int = 0;
    let mut mask: uint32_t = 0;
    mask = (*mbuf_ring).prod.mask;
    if 0 as *mut libc::c_void as libc::c_ulong == mbuf as libc::c_ulong {
        return 0 as libc::c_int;
    }
    loop {
        prod_head = (*mbuf_ring).prod.head;
        cons_tail = (*mbuf_ring).cons.tail;
        free_entries = mask
            .wrapping_add(1 as libc::c_uint)
            .wrapping_add(cons_tail)
            .wrapping_sub(prod_head);
        if !(free_entries > 0 as libc::c_uint) {
            __assert_fail(
                b"free_entries > 0\0" as *const u8 as *const libc::c_char,
                b"memory.c\0" as *const u8 as *const libc::c_char,
                155 as libc::c_uint,
                b"mbuf_free\0" as *const u8 as *const libc::c_char,
            );
        }
        prod_next = prod_head.wrapping_add(1 as libc::c_uint);
        success = rte_atomic32_cmpset(&mut (*mbuf_ring).prod.head, prod_head, prod_next);
        if !(0 as libc::c_int == success) {
            break;
        }
    }
    let ref mut fresh34 = *((*mbuf_ring).ring)
        .as_mut_ptr()
        .offset((prod_head & mask) as isize);
    *fresh34 = mbuf as *mut libc::c_void;
    asm!("", options(preserves_flags, att_syntax));
    while (*mbuf_ring).prod.tail != prod_head {}
    (*mbuf_ring).prod.tail = prod_next;
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
