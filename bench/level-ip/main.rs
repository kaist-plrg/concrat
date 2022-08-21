use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_rwlock_init(
        __rwlock: *mut pthread_rwlock_t,
        __attr: *const pthread_rwlockattr_t,
    ) -> libc::c_int;
    fn pthread_rwlock_rdlock(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
    fn pthread_rwlock_wrlock(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
    fn pthread_rwlock_unlock(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn ntohl(__netlong: uint32_t) -> uint32_t;
    fn ntohs(__netshort: uint16_t) -> uint16_t;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn perror(__s: *const libc::c_char);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn htonl(__hostlong: uint32_t) -> uint32_t;
    fn htons(__hostshort: uint16_t) -> uint16_t;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    static mut optind: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn inet_pton(
        __af: libc::c_int,
        __cp: *const libc::c_char,
        __buf: *mut libc::c_void,
    ) -> libc::c_int;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn strnlen(__string: *const libc::c_char, __maxlen: size_t) -> size_t;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t) -> libc::c_int;
    fn send(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn accept(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigwait(__set: *const sigset_t, __sig: *mut libc::c_int) -> libc::c_int;
    fn pthread_sigmask(
        __how: libc::c_int,
        __newmask: *const __sigset_t,
        __oldmask: *mut __sigset_t,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_cancel(__th: pthread_t) -> libc::c_int;
    fn prctl(__option: libc::c_int, _: ...) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn pthread_mutex_trylock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sk_buff {
    pub list: list_head,
    pub rt: *mut rtentry,
    pub dev: *mut netdev,
    pub refcnt: libc::c_int,
    pub protocol: uint16_t,
    pub len: uint32_t,
    pub dlen: uint32_t,
    pub seq: uint32_t,
    pub end_seq: uint32_t,
    pub end: *mut uint8_t,
    pub head: *mut uint8_t,
    pub data: *mut uint8_t,
    pub payload: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct netdev {
    pub addr: uint32_t,
    pub addr_len: uint8_t,
    pub hwaddr: [uint8_t; 6],
    pub mtu: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rtentry {
    pub list: list_head,
    pub dst: uint32_t,
    pub gateway: uint32_t,
    pub netmask: uint32_t,
    pub flags: uint8_t,
    pub metric: uint32_t,
    pub dev: *mut netdev,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __useconds_t = libc::c_uint;
pub type __socklen_t = libc::c_uint;
pub type pid_t = __pid_t;
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
pub struct __pthread_rwlock_arch_t {
    pub __readers: libc::c_uint,
    pub __writers: libc::c_uint,
    pub __wrphase_futex: libc::c_uint,
    pub __writers_futex: libc::c_uint,
    pub __pad3: libc::c_uint,
    pub __pad4: libc::c_uint,
    pub __cur_writer: libc::c_int,
    pub __shared: libc::c_int,
    pub __rwelision: libc::c_schar,
    pub __pad1: [libc::c_uchar; 7],
    pub __pad2: libc::c_ulong,
    pub __flags: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___wseq32_112954846 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_711291139 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: __anonstruct___wseq32_112954846,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___g1_start32_214694448 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_214694447 {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: __anonstruct___g1_start32_214694448,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __annonCompField1: __anonunion____missing_field_name_711291139,
    pub __annonCompField2: __anonunion____missing_field_name_214694447,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
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
pub union __anonunion_pthread_condattr_t_488594145 {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
pub type pthread_condattr_t = __anonunion_pthread_condattr_t_488594145;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_rwlock_t_656928968 {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type pthread_rwlock_t = __anonunion_pthread_rwlock_t_656928968;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_rwlockattr_t_145707745 {
    pub __size: [libc::c_char; 8],
    pub __align: libc::c_long,
}
pub type pthread_rwlockattr_t = __anonunion_pthread_rwlockattr_t_145707745;
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
pub type socklen_t = __socklen_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wait_lock {
    pub ready: pthread_cond_t,
    pub lock: pthread_mutex_t,
    pub sleeping: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sk_buff_head {
    pub head: list_head,
    pub qlen: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sock {
    pub sock: *mut socket,
    pub ops: *mut net_ops,
    pub recv_wait: wait_lock,
    pub receive_queue: sk_buff_head,
    pub write_queue: sk_buff_head,
    pub protocol: libc::c_int,
    pub state: libc::c_int,
    pub err: libc::c_int,
    pub poll_events: libc::c_short,
    pub sport: uint16_t,
    pub dport: uint16_t,
    pub saddr: uint32_t,
    pub daddr: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct net_ops {
    pub alloc_sock: Option::<unsafe extern "C" fn(libc::c_int) -> *mut sock>,
    pub init: Option::<unsafe extern "C" fn(*mut sock) -> libc::c_int>,
    pub connect: Option::<
        unsafe extern "C" fn(
            *mut sock,
            *const sockaddr,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub disconnect: Option::<
        unsafe extern "C" fn(*mut sock, libc::c_int) -> libc::c_int,
    >,
    pub write: Option::<
        unsafe extern "C" fn(*mut sock, *const libc::c_void, libc::c_int) -> libc::c_int,
    >,
    pub read: Option::<
        unsafe extern "C" fn(*mut sock, *mut libc::c_void, libc::c_int) -> libc::c_int,
    >,
    pub recv_notify: Option::<unsafe extern "C" fn(*mut sock) -> libc::c_int>,
    pub close: Option::<unsafe extern "C" fn(*mut sock) -> libc::c_int>,
    pub abort: Option::<unsafe extern "C" fn(*mut sock) -> libc::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct socket {
    pub list: list_head,
    pub fd: libc::c_int,
    pub pid: pid_t,
    pub refcnt: libc::c_int,
    pub state: socket_state,
    pub type_0: libc::c_short,
    pub flags: libc::c_int,
    pub sk: *mut sock,
    pub ops: *mut sock_ops,
    pub sleep: wait_lock,
    pub lock: pthread_rwlock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sock_ops {
    pub connect: Option::<
        unsafe extern "C" fn(
            *mut socket,
            *const sockaddr,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub write: Option::<
        unsafe extern "C" fn(
            *mut socket,
            *const libc::c_void,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub read: Option::<
        unsafe extern "C" fn(*mut socket, *mut libc::c_void, libc::c_int) -> libc::c_int,
    >,
    pub close: Option::<unsafe extern "C" fn(*mut socket) -> libc::c_int>,
    pub free: Option::<unsafe extern "C" fn(*mut socket) -> libc::c_int>,
    pub abort: Option::<unsafe extern "C" fn(*mut socket) -> libc::c_int>,
    pub poll: Option::<unsafe extern "C" fn(*mut socket) -> libc::c_int>,
    pub getpeername: Option::<
        unsafe extern "C" fn(*mut socket, *mut sockaddr, *mut socklen_t) -> libc::c_int,
    >,
    pub getsockname: Option::<
        unsafe extern "C" fn(*mut socket, *mut sockaddr, *mut socklen_t) -> libc::c_int,
    >,
}
pub type socket_state = libc::c_uint;
pub const SS_DISCONNECTING: socket_state = 4;
pub const SS_CONNECTED: socket_state = 3;
pub const SS_CONNECTING: socket_state = 2;
pub const SS_UNCONNECTED: socket_state = 1;
pub const SS_FREE: socket_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct net_family {
    pub create: Option::<unsafe extern "C" fn(*mut socket, libc::c_int) -> libc::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct eth_hdr {
    pub dmac: [uint8_t; 6],
    pub smac: [uint8_t; 6],
    pub ethertype: uint16_t,
    pub payload: [uint8_t; 0],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C, packed)]
pub struct iphdr {
    #[bitfield(name = "ihl", ty = "uint8_t", bits = "0..=3")]
    #[bitfield(name = "version", ty = "uint8_t", bits = "4..=7")]
    pub ihl_version: [u8; 1],
    pub tos: uint8_t,
    pub len: uint16_t,
    pub id: uint16_t,
    pub frag_offset: uint16_t,
    pub ttl: uint8_t,
    pub proto: uint8_t,
    pub csum: uint16_t,
    pub saddr: uint32_t,
    pub daddr: uint32_t,
    pub data: [uint8_t; 0],
}
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_sync_serial_settings_328012391 {
    pub clock_rate: libc::c_uint,
    pub clock_type: libc::c_uint,
    pub loopback: libc::c_ushort,
}
pub type sync_serial_settings = __anonstruct_sync_serial_settings_328012391;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_te1_settings_503328802 {
    pub clock_rate: libc::c_uint,
    pub clock_type: libc::c_uint,
    pub loopback: libc::c_ushort,
    pub slot_map: libc::c_uint,
}
pub type te1_settings = __anonstruct_te1_settings_503328802;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_raw_hdlc_proto_1014196403 {
    pub encoding: libc::c_ushort,
    pub parity: libc::c_ushort,
}
pub type raw_hdlc_proto = __anonstruct_raw_hdlc_proto_1014196403;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_fr_proto_380219538 {
    pub t391: libc::c_uint,
    pub t392: libc::c_uint,
    pub n391: libc::c_uint,
    pub n392: libc::c_uint,
    pub n393: libc::c_uint,
    pub lmi: libc::c_ushort,
    pub dce: libc::c_ushort,
}
pub type fr_proto = __anonstruct_fr_proto_380219538;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_fr_proto_pvc_888776187 {
    pub dlci: libc::c_uint,
}
pub type fr_proto_pvc = __anonstruct_fr_proto_pvc_888776187;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_fr_proto_pvc_info_1072026035 {
    pub dlci: libc::c_uint,
    pub master: [libc::c_char; 16],
}
pub type fr_proto_pvc_info = __anonstruct_fr_proto_pvc_info_1072026035;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_cisco_proto_1072026036 {
    pub interval: libc::c_uint,
    pub timeout: libc::c_uint,
}
pub type cisco_proto = __anonstruct_cisco_proto_1072026036;
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
pub union __anonunion_ifs_ifsu_6890304 {
    pub raw_hdlc: *mut raw_hdlc_proto,
    pub cisco: *mut cisco_proto,
    pub fr: *mut fr_proto,
    pub fr_pvc: *mut fr_proto_pvc,
    pub fr_pvc_info: *mut fr_proto_pvc_info,
    pub sync: *mut sync_serial_settings,
    pub te1: *mut te1_settings,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct if_settings {
    pub type_0: libc::c_uint,
    pub size: libc::c_uint,
    pub ifs_ifsu: __anonunion_ifs_ifsu_6890304,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_ifr_ifrn_352126815 {
    pub ifrn_name: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_ifr_ifru_32340664 {
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
    pub ifru_data: *mut libc::c_void,
    pub ifru_settings: if_settings,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifreq {
    pub ifr_ifrn: __anonunion_ifr_ifrn_352126815,
    pub ifr_ifru: __anonunion_ifr_ifru_32340664,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct icmp_v4 {
    pub type_0: uint8_t,
    pub code: uint8_t,
    pub csum: uint16_t,
    pub data: [uint8_t; 0],
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
pub struct sock_type {
    pub sock_ops: *mut sock_ops,
    pub net_ops: *mut net_ops,
    pub type_0: libc::c_int,
    pub protocol: libc::c_int,
}
pub type __int32_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timer {
    pub list: list_head,
    pub refcnt: libc::c_int,
    pub expires: uint32_t,
    pub cancelled: libc::c_int,
    pub handler: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    pub arg: *mut libc::c_void,
    pub lock: pthread_mutex_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C, packed)]
pub struct tcphdr {
    pub sport: uint16_t,
    pub dport: uint16_t,
    pub seq: uint32_t,
    pub ack_seq: uint32_t,
    #[bitfield(name = "rsvd", ty = "uint8_t", bits = "0..=3")]
    #[bitfield(name = "hl", ty = "uint8_t", bits = "4..=7")]
    #[bitfield(name = "fin", ty = "uint8_t", bits = "8..=8")]
    #[bitfield(name = "syn", ty = "uint8_t", bits = "9..=9")]
    #[bitfield(name = "rst", ty = "uint8_t", bits = "10..=10")]
    #[bitfield(name = "psh", ty = "uint8_t", bits = "11..=11")]
    #[bitfield(name = "ack", ty = "uint8_t", bits = "12..=12")]
    #[bitfield(name = "urg", ty = "uint8_t", bits = "13..=13")]
    #[bitfield(name = "ece", ty = "uint8_t", bits = "14..=14")]
    #[bitfield(name = "cwr", ty = "uint8_t", bits = "15..=15")]
    pub rsvd_hl_fin_syn_rst_psh_ack_urg_ece_cwr: [u8; 2],
    pub win: uint16_t,
    pub csum: uint16_t,
    pub urp: uint16_t,
    pub data: [uint8_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tcb {
    pub snd_una: uint32_t,
    pub snd_nxt: uint32_t,
    pub snd_wnd: uint32_t,
    pub snd_up: uint32_t,
    pub snd_wl1: uint32_t,
    pub snd_wl2: uint32_t,
    pub iss: uint32_t,
    pub rcv_nxt: uint32_t,
    pub rcv_wnd: uint32_t,
    pub rcv_up: uint32_t,
    pub irs: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct tcp_sack_block {
    pub left: uint32_t,
    pub right: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tcp_sock {
    pub sk: sock,
    pub fd: libc::c_int,
    pub tcp_header_len: uint16_t,
    pub tcb: tcb,
    pub flags: uint8_t,
    pub backoff: uint8_t,
    pub srtt: int32_t,
    pub rttvar: int32_t,
    pub rto: uint32_t,
    pub retransmit: *mut timer,
    pub delack: *mut timer,
    pub keepalive: *mut timer,
    pub linger: *mut timer,
    pub delacks: uint8_t,
    pub rmss: uint16_t,
    pub smss: uint16_t,
    pub cwnd: uint16_t,
    pub inflight: uint32_t,
    pub sackok: uint8_t,
    pub sacks_allowed: uint8_t,
    pub sacklen: uint8_t,
    pub sacks: [tcp_sack_block; 4],
    pub tsopt: uint8_t,
    pub ofo_queue: sk_buff_head,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tcp_options {
    pub options: uint16_t,
    pub mss: uint16_t,
    pub sack: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct tcp_opt_mss {
    pub kind: uint8_t,
    pub len: uint8_t,
    pub mss: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct arp_hdr {
    pub hwtype: uint16_t,
    pub protype: uint16_t,
    pub hwsize: uint8_t,
    pub prosize: uint8_t,
    pub opcode: uint16_t,
    pub data: [libc::c_uchar; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct arp_ipv4 {
    pub smac: [libc::c_uchar; 6],
    pub sip: uint32_t,
    pub dmac: [libc::c_uchar; 6],
    pub dip: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arp_cache_entry {
    pub list: list_head,
    pub hwtype: uint16_t,
    pub sip: uint32_t,
    pub smac: [libc::c_uchar; 6],
    pub state: libc::c_uint,
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
pub type __mode_t = libc::c_uint;
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_thread {
    pub list: list_head,
    pub sock: libc::c_int,
    pub id: pthread_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ipc_msg {
    pub type_0: uint16_t,
    pub pid: pid_t,
    pub data: [uint8_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ipc_err {
    pub rc: libc::c_int,
    pub err: libc::c_int,
    pub data: [uint8_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ipc_socket {
    pub domain: libc::c_int,
    pub type_0: libc::c_int,
    pub protocol: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ipc_connect {
    pub sockfd: libc::c_int,
    pub addr: sockaddr,
    pub addrlen: socklen_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ipc_write {
    pub sockfd: libc::c_int,
    pub len: size_t,
    pub buf: [uint8_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ipc_read {
    pub sockfd: libc::c_int,
    pub len: size_t,
    pub buf: [uint8_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ipc_close {
    pub sockfd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ipc_pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ipc_poll {
    pub nfds: nfds_t,
    pub timeout: libc::c_int,
    pub fds: [ipc_pollfd; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ipc_fcntl {
    pub sockfd: libc::c_int,
    pub cmd: libc::c_int,
    pub data: [uint8_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ipc_sockopt {
    pub fd: libc::c_int,
    pub level: libc::c_int,
    pub optname: libc::c_int,
    pub optlen: socklen_t,
    pub optval: [uint8_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_sockname {
    pub socket: libc::c_int,
    pub address_len: socklen_t,
    pub sa_data: [uint8_t; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___sigset_t_991265788 {
    pub __val: [libc::c_ulong; 16],
}
pub type __sigset_t = __anonstruct___sigset_t_991265788;
pub type sigset_t = __sigset_t;
#[inline]
unsafe extern "C" fn list_init(mut head: *mut list_head) {
    let mut tmp: *mut list_head = 0 as *mut list_head;
    tmp = head;
    (*head).next = tmp;
    (*head).prev = tmp;
}
pub unsafe extern "C" fn alloc_skb(mut size: libc::c_uint) -> *mut sk_buff {
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<sk_buff>() as libc::c_ulong);
    skb = tmp as *mut sk_buff;
    memset(
        skb as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sk_buff>() as libc::c_ulong,
    );
    tmp___0 = malloc(size as size_t);
    (*skb).data = tmp___0 as *mut uint8_t;
    memset((*skb).data as *mut libc::c_void, 0 as libc::c_int, size as size_t);
    (*skb).refcnt = 0 as libc::c_int;
    (*skb).head = (*skb).data;
    (*skb).end = ((*skb).data).offset(size as isize);
    list_init(&mut (*skb).list);
    return skb;
}
pub unsafe extern "C" fn free_skb(mut skb: *mut sk_buff) {
    if (*skb).refcnt < 1 as libc::c_int {
        free((*skb).head as *mut libc::c_void);
        free(skb as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn skb_reserve(
    mut skb: *mut sk_buff,
    mut len: libc::c_uint,
) -> *mut libc::c_void {
    (*skb).data = ((*skb).data).offset(len as isize);
    return (*skb).data as *mut libc::c_void;
}
pub unsafe extern "C" fn skb_push(
    mut skb: *mut sk_buff,
    mut len: libc::c_uint,
) -> *mut uint8_t {
    (*skb).data = ((*skb).data).offset(-(len as isize));
    (*skb).len = ((*skb).len as libc::c_uint).wrapping_add(len) as uint32_t as uint32_t;
    return (*skb).data;
}
pub unsafe extern "C" fn skb_head(mut skb: *mut sk_buff) -> *mut uint8_t {
    return (*skb).head;
}
pub unsafe extern "C" fn skb_reset_header(mut skb: *mut sk_buff) {
    (*skb).data = ((*skb).end).offset(-((*skb).dlen as isize));
    (*skb).len = (*skb).dlen;
}
#[inline]
unsafe extern "C" fn wait_init(mut w: *mut wait_lock) -> libc::c_int {
    pthread_cond_init(
        &mut (*w).ready as *mut pthread_cond_t,
        0 as *mut libc::c_void as *const pthread_condattr_t,
    );
    pthread_mutex_init(
        &mut (*w).lock,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
    (*w).sleeping = 0 as libc::c_int as uint8_t;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn wait_wakeup(mut w: *mut wait_lock) -> libc::c_int {
    pthread_mutex_lock(&mut (*w).lock);
    pthread_cond_signal(&mut (*w).ready);
    (*w).sleeping = 0 as libc::c_int as uint8_t;
    pthread_mutex_unlock(&mut (*w).lock);
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn wait_free(mut w: *mut wait_lock) {
    wait_wakeup(w);
    pthread_mutex_destroy(&mut (*w).lock);
    pthread_cond_destroy(&mut (*w).ready);
}
#[inline]
unsafe extern "C" fn list_add_tail(mut new: *mut list_head, mut head: *mut list_head) {
    (*(*head).prev).next = new;
    (*new).prev = (*head).prev;
    (*new).next = head;
    (*head).prev = new;
}
#[inline]
unsafe extern "C" fn list_del(mut elem: *mut list_head) {
    let mut prev: *mut list_head = 0 as *mut list_head;
    let mut next: *mut list_head = 0 as *mut list_head;
    prev = (*elem).prev;
    next = (*elem).next;
    (*prev).next = next;
    (*next).prev = prev;
}
static mut sock_amount: libc::c_int = 0 as libc::c_int;
static mut sockets: list_head = unsafe {
    {
        let mut init = list_head {
            next: &sockets as *const list_head as *mut list_head,
            prev: &sockets as *const list_head as *mut list_head,
        };
        init
    }
};
static mut slock: pthread_rwlock_t = __anonunion_pthread_rwlock_t_656928968 {
    __data: {
        let mut init = __pthread_rwlock_arch_t {
            __readers: 0 as libc::c_uint,
            __writers: 0 as libc::c_uint,
            __wrphase_futex: 0 as libc::c_uint,
            __writers_futex: 0 as libc::c_uint,
            __pad3: 0 as libc::c_uint,
            __pad4: 0 as libc::c_uint,
            __cur_writer: 0 as libc::c_int,
            __shared: 0 as libc::c_int,
            __rwelision: 0 as libc::c_int as libc::c_schar,
            __pad1: [
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
            ],
            __pad2: 0 as libc::c_ulong,
            __flags: 0 as libc::c_uint,
        };
        init
    },
};
static mut families: [*mut net_family; 128] = unsafe {
    [
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        &inet as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
    ]
};
static mut fd: libc::c_int = 4097 as libc::c_int;
unsafe extern "C" fn alloc_socket(mut pid: pid_t) -> *mut socket {
    let mut sock: *mut socket = 0 as *mut socket;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    tmp = malloc(::std::mem::size_of::<socket>() as libc::c_ulong);
    sock = tmp as *mut socket;
    list_init(&mut (*sock).list);
    (*sock).pid = pid;
    (*sock).refcnt = 1 as libc::c_int;
    pthread_rwlock_wrlock(&mut slock);
    tmp___0 = fd;
    fd += 1;
    (*sock).fd = tmp___0;
    pthread_rwlock_unlock(&mut slock);
    (*sock).state = SS_UNCONNECTED;
    (*sock).ops = 0 as *mut libc::c_void as *mut sock_ops;
    (*sock).flags = 2 as libc::c_int;
    wait_init(&mut (*sock).sleep);
    pthread_rwlock_init(
        &mut (*sock).lock as *mut pthread_rwlock_t,
        0 as *mut libc::c_void as *const pthread_rwlockattr_t,
    );
    return sock;
}
pub unsafe extern "C" fn socket_rd_acquire(mut sock: *mut socket) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tmp = pthread_rwlock_rdlock(&mut (*sock).lock);
    rc = tmp;
    (*sock).refcnt += 1;
    return rc;
}
pub unsafe extern "C" fn socket_wr_acquire(mut sock: *mut socket) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tmp = pthread_rwlock_wrlock(&mut (*sock).lock);
    rc = tmp;
    (*sock).refcnt += 1;
    return rc;
}
pub unsafe extern "C" fn socket_release(mut sock: *mut socket) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    rc = 0 as libc::c_int;
    (*sock).refcnt -= 1;
    if (*sock).refcnt == 0 as libc::c_int {
        rc = pthread_rwlock_unlock(&mut (*sock).lock);
        free(sock as *mut libc::c_void);
    } else {
        rc = pthread_rwlock_unlock(&mut (*sock).lock);
    }
    return rc;
}
pub unsafe extern "C" fn socket_free(mut sock: *mut socket) -> libc::c_int {
    pthread_rwlock_wrlock(&mut slock);
    socket_wr_acquire(sock);
    list_del(&mut (*sock).list);
    sock_amount -= 1;
    pthread_rwlock_unlock(&mut slock);
    if !((*sock).ops).is_null() {
        (Some(((*(*sock).ops).free).expect("non-null function pointer")))
            .expect("non-null function pointer")(sock);
    }
    wait_free(&mut (*sock).sleep);
    socket_release(sock);
    return 0 as libc::c_int;
}
unsafe extern "C" fn socket_garbage_collect(
    mut arg: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut sock: *mut socket = 0 as *mut socket;
    let mut tmp: *mut socket = 0 as *mut socket;
    tmp = socket_find(arg as *mut socket);
    sock = tmp;
    if sock as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void;
    }
    socket_free(sock);
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn socket_delete(mut sock: *mut socket) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    rc = 0 as libc::c_int;
    if !((*sock).state as libc::c_uint == 4 as libc::c_uint) {
        (*sock).state = SS_DISCONNECTING;
        timer_oneshot(
            10000 as libc::c_int as uint32_t,
            Some(
                socket_garbage_collect
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            sock as *mut libc::c_void,
        );
    }
    return rc;
}
pub unsafe extern "C" fn abort_sockets() {
    let mut item: *mut list_head = 0 as *mut list_head;
    let mut tmp: *mut list_head = 0 as *mut list_head;
    let mut sock: *mut socket = 0 as *mut socket;
    item = sockets.next;
    tmp = (*item).next;
    while item as libc::c_ulong != &mut sockets as *mut list_head as libc::c_ulong {
        sock = (item as *mut libc::c_char)
            .offset(
                -(&mut (*(0 as *mut socket)).list as *mut list_head as libc::c_ulong
                    as isize),
            ) as *mut socket;
        (Some(((*(*sock).ops).abort).expect("non-null function pointer")))
            .expect("non-null function pointer")(sock);
        item = tmp;
        tmp = (*item).next;
    }
}
unsafe extern "C" fn get_socket(mut pid: pid_t, mut fd___0: uint32_t) -> *mut socket {
    let mut item: *mut list_head = 0 as *mut list_head;
    let mut sock: *mut socket = 0 as *mut socket;
    let mut current_block_7: u64;
    sock = 0 as *mut libc::c_void as *mut socket;
    pthread_rwlock_rdlock(&mut slock);
    item = sockets.next;
    loop {
        if !(item as libc::c_ulong != &mut sockets as *mut list_head as libc::c_ulong) {
            current_block_7 = 14523784380283086299;
            break;
        }
        sock = (item as *mut libc::c_char)
            .offset(
                -(&mut (*(0 as *mut socket)).list as *mut list_head as libc::c_ulong
                    as isize),
            ) as *mut socket;
        if (*sock).pid == pid {
            if (*sock).fd as uint32_t == fd___0 {
                current_block_7 = 7609136404762307920;
                break;
            }
        }
        item = (*item).next;
    }
    match current_block_7 {
        14523784380283086299 => {
            sock = 0 as *mut libc::c_void as *mut socket;
        }
        _ => {}
    }
    pthread_rwlock_unlock(&mut slock);
    return sock;
}
pub unsafe extern "C" fn socket_lookup(
    mut remoteport: uint16_t,
    mut localport: uint16_t,
) -> *mut socket {
    let mut current_block: u64;
    let mut item: *mut list_head = 0 as *mut list_head;
    let mut sock: *mut socket = 0 as *mut socket;
    let mut sk: *mut sock = 0 as *mut sock;
    sock = 0 as *mut libc::c_void as *mut socket;
    sk = 0 as *mut libc::c_void as *mut sock;
    pthread_rwlock_rdlock(&mut slock);
    item = sockets.next;
    loop {
        if !(item as libc::c_ulong != &mut sockets as *mut list_head as libc::c_ulong) {
            current_block = 13586036798005543211;
            break;
        }
        sock = (item as *mut libc::c_char)
            .offset(
                -(&mut (*(0 as *mut socket)).list as *mut list_head as libc::c_ulong
                    as isize),
            ) as *mut socket;
        if !(sock as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
            if !((*sock).sk as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong)
            {
                sk = (*sock).sk;
                if (*sk).sport as libc::c_int == localport as libc::c_int {
                    if (*sk).dport as libc::c_int == remoteport as libc::c_int {
                        current_block = 7258759493297106313;
                        break;
                    }
                }
            }
        }
        item = (*item).next;
    }
    match current_block {
        13586036798005543211 => {
            sock = 0 as *mut libc::c_void as *mut socket;
        }
        _ => {}
    }
    pthread_rwlock_unlock(&mut slock);
    return sock;
}
pub unsafe extern "C" fn socket_find(mut find: *mut socket) -> *mut socket {
    let mut item: *mut list_head = 0 as *mut list_head;
    let mut sock: *mut socket = 0 as *mut socket;
    let mut current_block_7: u64;
    sock = 0 as *mut libc::c_void as *mut socket;
    pthread_rwlock_rdlock(&mut slock);
    item = sockets.next;
    loop {
        if !(item as libc::c_ulong != &mut sockets as *mut list_head as libc::c_ulong) {
            current_block_7 = 4906268039856690917;
            break;
        }
        sock = (item as *mut libc::c_char)
            .offset(
                -(&mut (*(0 as *mut socket)).list as *mut list_head as libc::c_ulong
                    as isize),
            ) as *mut socket;
        if sock as libc::c_ulong == find as libc::c_ulong {
            current_block_7 = 373821200400482761;
            break;
        }
        item = (*item).next;
    }
    match current_block_7 {
        4906268039856690917 => {
            sock = 0 as *mut libc::c_void as *mut socket;
        }
        _ => {}
    }
    pthread_rwlock_unlock(&mut slock);
    return sock;
}
pub unsafe extern "C" fn socket_debug() {}
pub unsafe extern "C" fn _socket(
    mut pid: pid_t,
    mut domain: libc::c_int,
    mut type_0: libc::c_int,
    mut protocol: libc::c_int,
) -> libc::c_int {
    let mut sock: *mut socket = 0 as *mut socket;
    let mut family: *mut net_family = 0 as *mut net_family;
    let mut tmp: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    sock = alloc_socket(pid);
    if sock as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"Could not alloc socket\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*sock).type_0 = type_0 as libc::c_short;
    family = families[domain as usize];
    if family.is_null() {
        fprintf(
            stderr,
            b"Domain not supported: %d\n\0" as *const u8 as *const libc::c_char,
            domain,
        );
    } else {
        tmp = (Some(((*family).create).expect("non-null function pointer")))
            .expect("non-null function pointer")(sock, protocol);
        if tmp != 0 as libc::c_int {
            fprintf(
                stderr,
                b"Creating domain failed\n\0" as *const u8 as *const libc::c_char,
            );
        } else {
            pthread_rwlock_wrlock(&mut slock);
            list_add_tail(&mut (*sock).list, &mut sockets);
            sock_amount += 1;
            socket_rd_acquire(sock);
            pthread_rwlock_unlock(&mut slock);
            rc = (*sock).fd;
            socket_release(sock);
            return rc;
        }
    }
    socket_free(sock);
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn _connect(
    mut pid: pid_t,
    mut sockfd: libc::c_int,
    mut addr: *const sockaddr,
    mut addrlen: socklen_t,
) -> libc::c_int {
    let mut sock: *mut socket = 0 as *mut socket;
    let mut rc: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    sock = get_socket(pid, sockfd as uint32_t);
    if sock as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"Connect: could not find socket (fd %u) for connection (pid %d)\n\0"
                as *const u8 as *const libc::c_char,
            sockfd,
            pid,
        );
        return -(9 as libc::c_int);
    }
    socket_wr_acquire(sock);
    tmp = (Some(((*(*sock).ops).connect).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(sock, addr, addrlen as libc::c_int, 0 as libc::c_int);
    rc = tmp;
    match rc {
        -110 | -111 | -97 | -22 => {
            socket_release(sock);
            socket_free(sock);
        }
        _ => {
            socket_release(sock);
        }
    }
    return rc;
}
pub unsafe extern "C" fn _write(
    mut pid: pid_t,
    mut sockfd: libc::c_int,
    mut buf: *const libc::c_void,
    count: libc::c_uint,
) -> libc::c_int {
    let mut sock: *mut socket = 0 as *mut socket;
    let mut rc: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    sock = get_socket(pid, sockfd as uint32_t);
    if sock as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"Write: could not find socket (fd %u) for connection (pid %d)\n\0"
                as *const u8 as *const libc::c_char,
            sockfd,
            pid,
        );
        return -(9 as libc::c_int);
    }
    socket_wr_acquire(sock);
    tmp = (Some(((*(*sock).ops).write).expect("non-null function pointer")))
        .expect("non-null function pointer")(sock, buf, count as libc::c_int);
    rc = tmp;
    socket_release(sock);
    return rc;
}
pub unsafe extern "C" fn _read(
    mut pid: pid_t,
    mut sockfd: libc::c_int,
    mut buf: *mut libc::c_void,
    count: libc::c_uint,
) -> libc::c_int {
    let mut sock: *mut socket = 0 as *mut socket;
    let mut rc: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    sock = get_socket(pid, sockfd as uint32_t);
    if sock as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"Read: could not find socket (fd %u) for connection (pid %d)\n\0"
                as *const u8 as *const libc::c_char,
            sockfd,
            pid,
        );
        return -(9 as libc::c_int);
    }
    socket_wr_acquire(sock);
    tmp = (Some(((*(*sock).ops).read).expect("non-null function pointer")))
        .expect("non-null function pointer")(sock, buf, count as libc::c_int);
    rc = tmp;
    socket_release(sock);
    return rc;
}
pub unsafe extern "C" fn _close(mut pid: pid_t, mut sockfd: libc::c_int) -> libc::c_int {
    let mut sock: *mut socket = 0 as *mut socket;
    let mut rc: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    sock = get_socket(pid, sockfd as uint32_t);
    if sock as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"Close: could not find socket (fd %u) for connection (pid %d)\n\0"
                as *const u8 as *const libc::c_char,
            sockfd,
            pid,
        );
        return -(9 as libc::c_int);
    }
    socket_wr_acquire(sock);
    tmp = (Some(((*(*sock).ops).close).expect("non-null function pointer")))
        .expect("non-null function pointer")(sock);
    rc = tmp;
    socket_release(sock);
    return rc;
}
pub unsafe extern "C" fn _poll(
    mut pid: pid_t,
    mut fds: *mut pollfd,
    mut nfds: nfds_t,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut polled: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut sock: *mut socket = 0 as *mut socket;
    let mut poll___0: *mut pollfd = 0 as *mut pollfd;
    loop {
        polled = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while (i as nfds_t) < nfds {
            poll___0 = fds.offset(i as isize);
            sock = get_socket(pid, (*poll___0).fd as uint32_t);
            if sock as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                fprintf(
                    stderr,
                    b"Poll: could not find socket (fd %u) for connection (pid %d)\n\0"
                        as *const u8 as *const libc::c_char,
                    (*poll___0).fd,
                    pid,
                );
                return -(9 as libc::c_int);
            }
            socket_rd_acquire(sock);
            (*poll___0)
                .revents = ((*(*sock).sk).poll_events as libc::c_int
                & ((*poll___0).events as libc::c_int | 16 as libc::c_int
                    | 8 as libc::c_int | 32 as libc::c_int)) as libc::c_short;
            if (*poll___0).revents as libc::c_int > 0 as libc::c_int {
                polled += 1;
            }
            socket_release(sock);
            i += 1;
        }
        if polled > 0 as libc::c_int {
            return polled
        } else {
            if timeout == 0 as libc::c_int {
                return polled
            } else {
                if timeout > 0 as libc::c_int {
                    if timeout > 10 as libc::c_int {
                        timeout -= 10 as libc::c_int;
                    } else {
                        timeout = 0 as libc::c_int;
                    }
                }
                usleep(10000 as libc::c_int as __useconds_t);
            }
        }
    };
}
pub unsafe extern "C" fn _fcntl(
    mut pid: pid_t,
    mut fildes: libc::c_int,
    mut cmd: libc::c_int,
    mut args: ...
) -> libc::c_int {
    let mut sock: *mut socket = 0 as *mut socket;
    let mut ap: ::std::ffi::VaListImpl;
    let mut rc: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    sock = get_socket(pid, fildes as uint32_t);
    if sock as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"Fcntl: could not find socket (fd %u) for connection (pid %d)\n\0"
                as *const u8 as *const libc::c_char,
            fildes,
            pid,
        );
        return -(9 as libc::c_int);
    }
    socket_wr_acquire(sock);
    rc = 0 as libc::c_int;
    match cmd {
        3 => {
            rc = (*sock).flags;
        }
        4 => {
            ap = args.clone();
            tmp = ap.arg::<libc::c_int>();
            (*sock).flags = tmp;
            rc = 0 as libc::c_int;
        }
        _ => {
            rc = -(1 as libc::c_int);
        }
    }
    socket_release(sock);
    return rc;
}
pub unsafe extern "C" fn _getsockopt(
    mut pid: pid_t,
    mut fd___0: libc::c_int,
    mut level: libc::c_int,
    mut optname: libc::c_int,
    mut optval: *mut libc::c_void,
    mut optlen: *mut socklen_t,
) -> libc::c_int {
    let mut sock: *mut socket = 0 as *mut socket;
    let mut rc: libc::c_int = 0;
    sock = get_socket(pid, fd___0 as uint32_t);
    if sock as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"Getsockopt: could not find socket (fd %u) for connection (pid %d)\n\0"
                as *const u8 as *const libc::c_char,
            fd___0,
            pid,
        );
        return -(9 as libc::c_int);
    }
    rc = 0 as libc::c_int;
    socket_rd_acquire(sock);
    match level {
        1 => {
            match optname {
                4 => {
                    *optlen = 4 as libc::c_int as socklen_t;
                    *(optval as *mut libc::c_int) = (*(*sock).sk).err;
                    rc = 0 as libc::c_int;
                }
                _ => {
                    fprintf(
                        stderr,
                        b"Getsockopt unsupported optname %d\n\0" as *const u8
                            as *const libc::c_char,
                        optname,
                    );
                    rc = -(92 as libc::c_int);
                }
            }
        }
        _ => {
            fprintf(
                stderr,
                b"Getsockopt: Unsupported level %d\n\0" as *const u8
                    as *const libc::c_char,
                level,
            );
            rc = -(22 as libc::c_int);
        }
    }
    socket_release(sock);
    return rc;
}
pub unsafe extern "C" fn _getpeername(
    mut pid: pid_t,
    mut socket___0: libc::c_int,
    mut address: *mut sockaddr,
    mut address_len: *mut socklen_t,
) -> libc::c_int {
    let mut sock: *mut socket = 0 as *mut socket;
    let mut rc: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    sock = get_socket(pid, socket___0 as uint32_t);
    if sock as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"Getpeername: could not find socket (fd %u) for connection (pid %d)\n\0"
                as *const u8 as *const libc::c_char,
            socket___0,
            pid,
        );
        return -(9 as libc::c_int);
    }
    socket_rd_acquire(sock);
    tmp = (Some(((*(*sock).ops).getpeername).expect("non-null function pointer")))
        .expect("non-null function pointer")(sock, address, address_len);
    rc = tmp;
    socket_release(sock);
    return rc;
}
pub unsafe extern "C" fn _getsockname(
    mut pid: pid_t,
    mut socket___0: libc::c_int,
    mut address: *mut sockaddr,
    mut address_len: *mut socklen_t,
) -> libc::c_int {
    let mut sock: *mut socket = 0 as *mut socket;
    let mut rc: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    sock = get_socket(pid, socket___0 as uint32_t);
    if sock as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"Getsockname: could not find socket (fd %u) for connection (pid %d)\n\0"
                as *const u8 as *const libc::c_char,
            socket___0,
            pid,
        );
        return -(9 as libc::c_int);
    }
    socket_rd_acquire(sock);
    tmp = (Some(((*(*sock).ops).getsockname).expect("non-null function pointer")))
        .expect("non-null function pointer")(sock, address, address_len);
    rc = tmp;
    socket_release(sock);
    return rc;
}
#[inline]
unsafe extern "C" fn ip_hdr(mut skb: *const sk_buff) -> *mut iphdr {
    return ((*skb).head)
        .offset(::std::mem::size_of::<eth_hdr>() as libc::c_ulong as isize)
        as *mut iphdr;
}
unsafe extern "C" fn ip_init_pkt(mut ih: *mut iphdr) {
    (*ih).saddr = ntohl((*ih).saddr);
    (*ih).daddr = ntohl((*ih).daddr);
    (*ih).len = ntohs((*ih).len);
    (*ih).id = ntohs((*ih).id);
}
pub unsafe extern "C" fn ip_rcv(mut skb: *mut sk_buff) -> libc::c_int {
    let mut ih: *mut iphdr = 0 as *mut iphdr;
    let mut tmp: *mut iphdr = 0 as *mut iphdr;
    let mut csum: uint16_t = 0;
    tmp = ip_hdr(skb as *const sk_buff);
    ih = tmp;
    csum = -(1 as libc::c_int) as uint16_t;
    if (*ih).version() as libc::c_int != 4 as libc::c_int {
        fprintf(
            stderr,
            b"Datagram version was not IPv4\n\0" as *const u8 as *const libc::c_char,
        );
    } else if ((*ih).ihl() as libc::c_int) < 5 as libc::c_int {
        fprintf(
            stderr,
            b"IPv4 header length must be at least 5\n\0" as *const u8
                as *const libc::c_char,
        );
    } else if (*ih).ttl as libc::c_int == 0 as libc::c_int {
        fprintf(
            stderr,
            b"Time to live of datagram reached 0\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        csum = checksum(
            ih as *mut libc::c_void,
            (*ih).ihl() as libc::c_int * 4 as libc::c_int,
            0 as libc::c_int,
        );
        if !(csum as libc::c_int != 0 as libc::c_int) {
            ip_init_pkt(ih);
            match (*ih).proto as libc::c_int {
                1 => {
                    icmpv4_incoming(skb);
                    return 0 as libc::c_int;
                }
                6 => {
                    tcp_in(skb);
                    return 0 as libc::c_int;
                }
                _ => {
                    fprintf(
                        stderr,
                        b"Unknown IP header proto\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
        }
    }
    free_skb(skb);
    return 0 as libc::c_int;
}
static mut tun_fd: libc::c_int = 0;
static mut dev: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut tapaddr: *mut libc::c_char = b"10.0.0.5\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
pub static mut taproute: *mut libc::c_char = b"10.0.0.0/24\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn set_if_route(
    mut dev___0: *mut libc::c_char,
    mut cidr: *mut libc::c_char,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = run_cmd(
        b"ip route add dev %s %s\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        dev___0,
        cidr,
    );
    return tmp;
}
unsafe extern "C" fn set_if_address(
    mut dev___0: *mut libc::c_char,
    mut cidr: *mut libc::c_char,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = run_cmd(
        b"ip address add dev %s local %s\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        dev___0,
        cidr,
    );
    return tmp;
}
unsafe extern "C" fn set_if_up(mut dev___0: *mut libc::c_char) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = run_cmd(
        b"ip link set dev %s up\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        dev___0,
    );
    return tmp;
}
unsafe extern "C" fn tun_alloc(mut dev___0: *mut libc::c_char) -> libc::c_int {
    let mut ifr: ifreq = ifreq {
        ifr_ifrn: __anonunion_ifr_ifrn_352126815 {
            ifrn_name: [0; 16],
        },
        ifr_ifru: __anonunion_ifr_ifru_32340664 {
            ifru_addr: sockaddr {
                sa_family: 0,
                sa_data: [0; 14],
            },
        },
    };
    let mut fd___0: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    fd___0 = open(
        b"/dev/net/tap\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
    );
    if fd___0 < 0 as libc::c_int {
        perror(
            b"Cannot open TUN/TAP dev\nMake sure one exists with '$ mknod /dev/net/tap c 10 200'\0"
                as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    memset(
        &mut ifr as *mut ifreq as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<ifreq>() as libc::c_ulong,
    );
    ifr.ifr_ifru.ifru_flags = 4098 as libc::c_int as libc::c_short;
    if *dev___0 != 0 {
        strncpy(
            (ifr.ifr_ifrn.ifrn_name).as_mut_ptr(),
            dev___0 as *const libc::c_char,
            16 as libc::c_int as size_t,
        );
    }
    err = ioctl(
        fd___0,
        ((1 as libc::c_uint) << 30 as libc::c_int
            | ((84 as libc::c_int) << 8 as libc::c_int) as libc::c_uint
            | 202 as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                << 16 as libc::c_int,
        &mut ifr as *mut ifreq as *mut libc::c_void,
    );
    if err < 0 as libc::c_int {
        perror(b"ERR: Could not ioctl tun\0" as *const u8 as *const libc::c_char);
        close(fd___0);
        return err;
    }
    strcpy(dev___0, (ifr.ifr_ifrn.ifrn_name).as_mut_ptr() as *const libc::c_char);
    return fd___0;
}
pub unsafe extern "C" fn tun_read(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut tmp: ssize_t = 0;
    tmp = read(tun_fd, buf as *mut libc::c_void, len as size_t);
    return tmp as libc::c_int;
}
pub unsafe extern "C" fn tun_write(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut tmp: ssize_t = 0;
    tmp = write(tun_fd, buf as *const libc::c_void, len as size_t);
    return tmp as libc::c_int;
}
pub unsafe extern "C" fn tun_init() {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    tmp = calloc(10 as libc::c_int as size_t, 1 as libc::c_int as size_t);
    dev = tmp as *mut libc::c_char;
    tun_fd = tun_alloc(dev);
    tmp___0 = set_if_up(dev);
    if tmp___0 != 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR when setting up if\n\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___1 = set_if_route(dev, taproute);
    if tmp___1 != 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR when setting route for if\n\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___2 = set_if_address(dev, tapaddr);
    if tmp___2 != 0 as libc::c_int {
        fprintf(
            stderr,
            b"ERROR when setting addr for if\n\0" as *const u8 as *const libc::c_char,
        );
    }
}
pub unsafe extern "C" fn free_tun() {
    free(dev as *mut libc::c_void);
}
pub unsafe extern "C" fn icmpv4_incoming(mut skb: *mut sk_buff) {
    let mut iphdr: *mut iphdr = 0 as *mut iphdr;
    let mut tmp: *mut iphdr = 0 as *mut iphdr;
    let mut icmp: *mut icmp_v4 = 0 as *mut icmp_v4;
    tmp = ip_hdr(skb as *const sk_buff);
    iphdr = tmp;
    icmp = ((*iphdr).data).as_mut_ptr() as *mut icmp_v4;
    match (*icmp).type_0 as libc::c_int {
        8 => {
            icmpv4_reply(skb);
            return;
        }
        3 => {
            fprintf(
                stderr,
                b"ICMPv4 received 'dst unreachable' code %d, check your routes and firewall rules\n\0"
                    as *const u8 as *const libc::c_char,
                (*icmp).code as libc::c_int,
            );
        }
        _ => {
            fprintf(
                stderr,
                b"ICMPv4 did not match supported types\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    free_skb(skb);
}
pub unsafe extern "C" fn icmpv4_reply(mut skb: *mut sk_buff) {
    let mut iphdr: *mut iphdr = 0 as *mut iphdr;
    let mut tmp: *mut iphdr = 0 as *mut iphdr;
    let mut icmp: *mut icmp_v4 = 0 as *mut icmp_v4;
    let mut sk: sock = sock {
        sock: 0 as *mut socket,
        ops: 0 as *mut net_ops,
        recv_wait: wait_lock {
            ready: __anonunion_pthread_cond_t_951761805 {
                __data: __pthread_cond_s {
                    __annonCompField1: __anonunion____missing_field_name_711291139 {
                        __wseq: 0,
                    },
                    __annonCompField2: __anonunion____missing_field_name_214694447 {
                        __g1_start: 0,
                    },
                    __g_refs: [0; 2],
                    __g_size: [0; 2],
                    __g1_orig_size: 0,
                    __wrefs: 0,
                    __g_signals: [0; 2],
                },
            },
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
            sleeping: 0,
        },
        receive_queue: sk_buff_head {
            head: list_head {
                next: 0 as *mut list_head,
                prev: 0 as *mut list_head,
            },
            qlen: 0,
        },
        write_queue: sk_buff_head {
            head: list_head {
                next: 0 as *mut list_head,
                prev: 0 as *mut list_head,
            },
            qlen: 0,
        },
        protocol: 0,
        state: 0,
        err: 0,
        poll_events: 0,
        sport: 0,
        dport: 0,
        saddr: 0,
        daddr: 0,
    };
    let mut icmp_len: uint16_t = 0;
    tmp = ip_hdr(skb as *const sk_buff);
    iphdr = tmp;
    memset(
        &mut sk as *mut sock as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sock>() as libc::c_ulong,
    );
    icmp_len = ((*iphdr).len as libc::c_int
        - (*iphdr).ihl() as libc::c_int * 4 as libc::c_int) as uint16_t;
    skb_reserve(
        skb,
        (::std::mem::size_of::<eth_hdr>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<iphdr>() as libc::c_ulong)
            .wrapping_add(icmp_len as libc::c_ulong) as libc::c_uint,
    );
    skb_push(skb, icmp_len as libc::c_uint);
    icmp = (*skb).data as *mut icmp_v4;
    (*icmp).type_0 = 0 as libc::c_int as uint8_t;
    (*icmp).csum = 0 as libc::c_int as uint16_t;
    (*icmp)
        .csum = checksum(
        icmp as *mut libc::c_void,
        icmp_len as libc::c_int,
        0 as libc::c_int,
    );
    (*skb).protocol = 1 as libc::c_int as uint16_t;
    sk.daddr = (*iphdr).saddr;
    ip_output(&mut sk, skb);
    free_skb(skb);
}
#[inline]
unsafe extern "C" fn wait_sleep(mut w: *mut wait_lock) -> libc::c_int {
    (*w).sleeping = 1 as libc::c_int as uint8_t;
    pthread_cond_wait(
        &mut (*w).ready as *mut pthread_cond_t,
        &mut (*w).lock as *mut pthread_mutex_t,
    );
    return 0 as libc::c_int;
}
static mut INET_OPS: libc::c_int = 1 as libc::c_int;
pub static mut inet: net_family = {
    let mut init = net_family {
        create: Some(
            inet_create as unsafe extern "C" fn(*mut socket, libc::c_int) -> libc::c_int,
        ),
    };
    init
};
static mut inet_stream_ops: sock_ops = {
    let mut init = sock_ops {
        connect: Some(
            inet_stream_connect
                as unsafe extern "C" fn(
                    *mut socket,
                    *const sockaddr,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        write: Some(
            inet_write
                as unsafe extern "C" fn(
                    *mut socket,
                    *const libc::c_void,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        read: Some(
            inet_read
                as unsafe extern "C" fn(
                    *mut socket,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        close: Some(inet_close as unsafe extern "C" fn(*mut socket) -> libc::c_int),
        free: Some(inet_free as unsafe extern "C" fn(*mut socket) -> libc::c_int),
        abort: Some(inet_abort as unsafe extern "C" fn(*mut socket) -> libc::c_int),
        poll: None,
        getpeername: Some(
            inet_getpeername
                as unsafe extern "C" fn(
                    *mut socket,
                    *mut sockaddr,
                    *mut socklen_t,
                ) -> libc::c_int,
        ),
        getsockname: Some(
            inet_getsockname
                as unsafe extern "C" fn(
                    *mut socket,
                    *mut sockaddr,
                    *mut socklen_t,
                ) -> libc::c_int,
        ),
    };
    init
};
static mut inet_ops: [sock_type; 1] = unsafe {
    [
        {
            let mut init = sock_type {
                sock_ops: &inet_stream_ops as *const sock_ops as *mut sock_ops,
                net_ops: &tcp_ops as *const net_ops as *mut net_ops,
                type_0: 1 as libc::c_int,
                protocol: 6 as libc::c_int,
            };
            init
        },
    ]
};
pub unsafe extern "C" fn inet_create(
    mut sock: *mut socket,
    mut protocol: libc::c_int,
) -> libc::c_int {
    let mut sk: *mut sock = 0 as *mut sock;
    let mut skt: *mut sock_type = 0 as *mut sock_type;
    let mut i: libc::c_int = 0;
    skt = 0 as *mut libc::c_void as *mut sock_type;
    i = 0 as libc::c_int;
    while i < INET_OPS {
        if inet_ops[i as usize].type_0 & (*sock).type_0 as libc::c_int != 0 {
            skt = &mut *inet_ops.as_mut_ptr().offset(i as isize) as *mut sock_type;
            break;
        } else {
            i += 1;
        }
    }
    if skt.is_null() {
        fprintf(
            stderr,
            b"Could not find socktype for socket\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    (*sock).ops = (*skt).sock_ops;
    sk = sk_alloc((*skt).net_ops, protocol);
    (*sk).protocol = protocol;
    sock_init_data(sock, sk);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn inet_socket(
    mut sock: *mut socket,
    mut protocol: libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn inet_connect(
    mut sock: *mut socket,
    mut addr: *mut sockaddr,
    mut addr_len: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
unsafe extern "C" fn inet_stream_connect(
    mut sock: *mut socket,
    mut addr: *const sockaddr,
    mut addr_len: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut sk: *mut sock = 0 as *mut sock;
    let mut rc: libc::c_int = 0;
    sk = (*sock).sk;
    rc = 0 as libc::c_int;
    if (addr_len as libc::c_ulong)
        < ::std::mem::size_of::<sa_family_t>() as libc::c_ulong
    {
        return -(22 as libc::c_int);
    }
    if (*addr).sa_family as libc::c_int == 0 as libc::c_int {
        (Some(((*(*sk).ops).disconnect).expect("non-null function pointer")))
            .expect("non-null function pointer")(sk, flags);
        return -(97 as libc::c_int);
    }
    match (*sock).state as libc::c_uint {
        3 => {
            (*sk).err = -(106 as libc::c_int);
        }
        2 => {
            (*sk).err = -(114 as libc::c_int);
        }
        1 => {
            (*sk).err = -(106 as libc::c_int);
            if !((*sk).state != 6 as libc::c_int) {
                (Some(((*(*sk).ops).connect).expect("non-null function pointer")))
                    .expect("non-null function pointer")(sk, addr, addr_len, flags);
                (*sock).state = SS_CONNECTING;
                (*sk).err = -(115 as libc::c_int);
                if !((*sock).flags & 2048 as libc::c_int != 0) {
                    pthread_mutex_lock(&mut (*sock).sleep.lock);
                    while (*sock).state as libc::c_uint == 2 as libc::c_uint {
                        if !((*sk).err == -(115 as libc::c_int)) {
                            break;
                        }
                        socket_release(sock);
                        wait_sleep(&mut (*sock).sleep);
                        socket_wr_acquire(sock);
                    }
                    pthread_mutex_unlock(&mut (*sock).sleep.lock);
                    socket_wr_acquire(sock);
                    match (*sk).err {
                        -111 | -110 => {
                            rc = (*sk).err;
                            return rc;
                        }
                        _ => {
                            if !((*sk).err != 0 as libc::c_int) {
                                (*sock).state = SS_CONNECTED;
                            }
                        }
                    }
                }
            }
        }
        _ => {
            (*sk).err = -(22 as libc::c_int);
        }
    }
    return (*sk).err;
}
pub unsafe extern "C" fn inet_write(
    mut sock: *mut socket,
    mut buf: *const libc::c_void,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut sk: *mut sock = 0 as *mut sock;
    let mut tmp: libc::c_int = 0;
    sk = (*sock).sk;
    tmp = (Some(((*(*sk).ops).write).expect("non-null function pointer")))
        .expect("non-null function pointer")(sk, buf, len);
    return tmp;
}
pub unsafe extern "C" fn inet_read(
    mut sock: *mut socket,
    mut buf: *mut libc::c_void,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut sk: *mut sock = 0 as *mut sock;
    let mut tmp: libc::c_int = 0;
    sk = (*sock).sk;
    tmp = (Some(((*(*sk).ops).read).expect("non-null function pointer")))
        .expect("non-null function pointer")(sk, buf, len);
    return tmp;
}
pub unsafe extern "C" fn inet_lookup(
    mut skb: *mut sk_buff,
    mut sport: uint16_t,
    mut dport: uint16_t,
) -> *mut sock {
    let mut sock: *mut socket = 0 as *mut socket;
    let mut tmp: *mut socket = 0 as *mut socket;
    tmp = socket_lookup(sport, dport);
    sock = tmp;
    if sock as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut sock;
    }
    return (*sock).sk;
}
pub unsafe extern "C" fn inet_close(mut sock: *mut socket) -> libc::c_int {
    let mut sk: *mut sock = 0 as *mut sock;
    let mut tmp: libc::c_int = 0;
    if sock.is_null() {
        return 0 as libc::c_int;
    }
    sk = (*sock).sk;
    tmp = (Some(((*(*(*sock).sk).ops).close).expect("non-null function pointer")))
        .expect("non-null function pointer")(sk);
    return tmp;
}
pub unsafe extern "C" fn inet_free(mut sock: *mut socket) -> libc::c_int {
    let mut sk: *mut sock = 0 as *mut sock;
    sk = (*sock).sk;
    sock_free(sk);
    free((*sock).sk as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn inet_abort(mut sock: *mut socket) -> libc::c_int {
    let mut sk: *mut sock = 0 as *mut sock;
    sk = (*sock).sk;
    if !sk.is_null() {
        (Some(((*(*sk).ops).abort).expect("non-null function pointer")))
            .expect("non-null function pointer")(sk);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn inet_getpeername(
    mut sock: *mut socket,
    mut address: *mut sockaddr,
    mut address_len: *mut socklen_t,
) -> libc::c_int {
    let mut sk: *mut sock = 0 as *mut sock;
    let mut res: *mut sockaddr_in = 0 as *mut sockaddr_in;
    sk = (*sock).sk;
    if sk as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    res = address as *mut sockaddr_in;
    (*res).sin_family = 2 as libc::c_int as sa_family_t;
    (*res).sin_port = htons((*sk).dport);
    (*res).sin_addr.s_addr = htonl((*sk).daddr);
    *address_len = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn inet_getsockname(
    mut sock: *mut socket,
    mut address: *mut sockaddr,
    mut address_len: *mut socklen_t,
) -> libc::c_int {
    let mut sk: *mut sock = 0 as *mut sock;
    let mut res: *mut sockaddr_in = 0 as *mut sockaddr_in;
    sk = (*sock).sk;
    if sk as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    res = address as *mut sockaddr_in;
    (*res).sin_family = 2 as libc::c_int as sa_family_t;
    (*res).sin_port = htons((*sk).sport);
    (*res).sin_addr.s_addr = htonl((*sk).saddr);
    *address_len = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn skb_queue_len(mut list: *const sk_buff_head) -> uint32_t {
    return (*list).qlen;
}
#[inline]
unsafe extern "C" fn skb_queue_init(mut list: *mut sk_buff_head) {
    list_init(&mut (*list).head);
    (*list).qlen = 0 as libc::c_int as uint32_t;
}
#[inline]
unsafe extern "C" fn skb_dequeue(mut list: *mut sk_buff_head) -> *mut sk_buff {
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    skb = ((*list).head.next as *mut libc::c_char)
        .offset(
            -(&mut (*(0 as *mut sk_buff)).list as *mut list_head as libc::c_ulong
                as isize),
        ) as *mut sk_buff;
    list_del(&mut (*skb).list);
    (*list).qlen = ((*list).qlen).wrapping_sub(1);
    return skb;
}
#[inline]
unsafe extern "C" fn skb_queue_empty(mut list: *const sk_buff_head) -> libc::c_int {
    let mut tmp: uint32_t = 0;
    tmp = skb_queue_len(list);
    return (tmp < 1 as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn skb_peek(mut list: *mut sk_buff_head) -> *mut sk_buff {
    let mut tmp: libc::c_int = 0;
    tmp = skb_queue_empty(list as *const sk_buff_head);
    if tmp != 0 {
        return 0 as *mut libc::c_void as *mut sk_buff;
    }
    return ((*list).head.next as *mut libc::c_char)
        .offset(
            -(&mut (*(0 as *mut sk_buff)).list as *mut list_head as libc::c_ulong
                as isize),
        ) as *mut sk_buff;
}
#[inline]
unsafe extern "C" fn skb_queue_free(mut list: *mut sk_buff_head) {
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    skb = 0 as *mut libc::c_void as *mut sk_buff;
    loop {
        skb = skb_peek(list);
        if !(skb as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
            break;
        }
        skb_dequeue(list);
        (*skb).refcnt -= 1;
        free_skb(skb);
    };
}
static mut tcplock: pthread_rwlock_t = __anonunion_pthread_rwlock_t_656928968 {
    __data: {
        let mut init = __pthread_rwlock_arch_t {
            __readers: 0 as libc::c_uint,
            __writers: 0 as libc::c_uint,
            __wrphase_futex: 0 as libc::c_uint,
            __writers_futex: 0 as libc::c_uint,
            __pad3: 0 as libc::c_uint,
            __pad4: 0 as libc::c_uint,
            __cur_writer: 0 as libc::c_int,
            __shared: 0 as libc::c_int,
            __rwelision: 0 as libc::c_int as libc::c_schar,
            __pad1: [
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
            ],
            __pad2: 0 as libc::c_ulong,
            __flags: 0 as libc::c_uint,
        };
        init
    },
};
pub static mut tcp_ops: net_ops = unsafe {
    {
        let mut init = net_ops {
            alloc_sock: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> *mut sock>,
                Option::<unsafe extern "C" fn(libc::c_int) -> *mut sock>,
            >(Some(tcp_alloc_sock as unsafe extern "C" fn() -> *mut sock)),
            init: Some(
                tcp_v4_init_sock as unsafe extern "C" fn(*mut sock) -> libc::c_int,
            ),
            connect: Some(
                tcp_v4_connect
                    as unsafe extern "C" fn(
                        *mut sock,
                        *const sockaddr,
                        libc::c_int,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            disconnect: Some(
                tcp_disconnect
                    as unsafe extern "C" fn(*mut sock, libc::c_int) -> libc::c_int,
            ),
            write: Some(
                tcp_write
                    as unsafe extern "C" fn(
                        *mut sock,
                        *const libc::c_void,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            read: Some(
                tcp_read
                    as unsafe extern "C" fn(
                        *mut sock,
                        *mut libc::c_void,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            recv_notify: Some(
                tcp_recv_notify as unsafe extern "C" fn(*mut sock) -> libc::c_int,
            ),
            close: Some(tcp_close as unsafe extern "C" fn(*mut sock) -> libc::c_int),
            abort: Some(tcp_abort as unsafe extern "C" fn(*mut sock) -> libc::c_int),
        };
        init
    }
};
pub unsafe extern "C" fn tcp_init() {}
unsafe extern "C" fn tcp_init_segment(
    mut th: *mut tcphdr,
    mut ih: *mut iphdr,
    mut skb: *mut sk_buff,
) {
    (*th).sport = ntohs((*th).sport);
    (*th).dport = ntohs((*th).dport);
    (*th).seq = ntohl((*th).seq);
    (*th).ack_seq = ntohl((*th).ack_seq);
    (*th).win = ntohs((*th).win);
    (*th).csum = ntohs((*th).csum);
    (*th).urp = ntohs((*th).urp);
    (*skb).seq = (*th).seq;
    (*skb)
        .dlen = ((*ih).len as libc::c_int - (*ih).ihl() as libc::c_int * 4 as libc::c_int
        - (((*th).hl() as libc::c_int) << 2 as libc::c_int)) as uint32_t;
    (*skb)
        .len = ((*skb).dlen)
        .wrapping_add((*th).syn() as uint32_t)
        .wrapping_add((*th).fin() as uint32_t);
    (*skb).end_seq = ((*skb).seq).wrapping_add((*skb).dlen);
    (*skb).payload = ((*th).data).as_mut_ptr();
}
unsafe extern "C" fn tcp_clear_queues(mut tsk: *mut tcp_sock) {
    skb_queue_free(&mut (*tsk).ofo_queue);
}
pub unsafe extern "C" fn tcp_in(mut skb: *mut sk_buff) {
    let mut sk: *mut sock = 0 as *mut sock;
    let mut iph: *mut iphdr = 0 as *mut iphdr;
    let mut th: *mut tcphdr = 0 as *mut tcphdr;
    iph = ip_hdr(skb as *const sk_buff);
    th = ((*iph).data).as_mut_ptr() as *mut tcphdr;
    tcp_init_segment(th, iph, skb);
    sk = inet_lookup(skb, (*th).sport, (*th).dport);
    if sk as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"No TCP socket for sport %d dport %d\n\0" as *const u8
                as *const libc::c_char,
            (*th).sport as libc::c_int,
            (*th).dport as libc::c_int,
        );
        free_skb(skb);
        return;
    }
    socket_wr_acquire((*sk).sock);
    tcp_input_state(sk, th, skb);
    socket_release((*sk).sock);
}
pub unsafe extern "C" fn tcp_udp_checksum(
    mut saddr: uint32_t,
    mut daddr: uint32_t,
    mut proto: uint8_t,
    mut data: *mut uint8_t,
    mut len: uint16_t,
) -> libc::c_int {
    let mut sum: uint32_t = 0;
    let mut tmp: uint16_t = 0;
    let mut tmp___0: uint16_t = 0;
    let mut tmp___1: uint16_t = 0;
    sum = 0 as libc::c_int as uint32_t;
    sum = (sum as libc::c_uint).wrapping_add(saddr) as uint32_t as uint32_t;
    sum = (sum as libc::c_uint).wrapping_add(daddr) as uint32_t as uint32_t;
    tmp = htons(proto as uint16_t);
    sum = (sum as libc::c_uint).wrapping_add(tmp as uint32_t) as uint32_t as uint32_t;
    tmp___0 = htons(len);
    sum = (sum as libc::c_uint).wrapping_add(tmp___0 as uint32_t) as uint32_t
        as uint32_t;
    tmp___1 = checksum(
        data as *mut libc::c_void,
        len as libc::c_int,
        sum as libc::c_int,
    );
    return tmp___1 as libc::c_int;
}
pub unsafe extern "C" fn tcp_v4_checksum(
    mut skb: *mut sk_buff,
    mut saddr: uint32_t,
    mut daddr: uint32_t,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = tcp_udp_checksum(
        saddr,
        daddr,
        6 as libc::c_int as uint8_t,
        (*skb).data,
        (*skb).len as uint16_t,
    );
    return tmp;
}
pub unsafe extern "C" fn tcp_alloc_sock() -> *mut sock {
    let mut tsk: *mut tcp_sock = 0 as *mut tcp_sock;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<tcp_sock>() as libc::c_ulong);
    tsk = tmp as *mut tcp_sock;
    memset(
        tsk as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<tcp_sock>() as libc::c_ulong,
    );
    (*tsk).sk.state = 6 as libc::c_int;
    (*tsk).sackok = 1 as libc::c_int as uint8_t;
    (*tsk).rmss = 1460 as libc::c_int as uint16_t;
    (*tsk).smss = 536 as libc::c_int as uint16_t;
    skb_queue_init(&mut (*tsk).ofo_queue);
    return tsk as *mut sock;
}
pub unsafe extern "C" fn tcp_v4_init_sock(mut sk: *mut sock) -> libc::c_int {
    tcp_init_sock(sk);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tcp_init_sock(mut sk: *mut sock) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn __tcp_set_state(mut sk: *mut sock, mut state: uint32_t) {
    (*sk).state = state as libc::c_int;
}
static mut port: libc::c_int = 40000 as libc::c_int;
unsafe extern "C" fn generate_port() -> uint16_t {
    let mut copy: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    pthread_rwlock_wrlock(&mut tcplock);
    port += 1;
    tmp = timer_get_tick();
    copy = port + tmp % 10000 as libc::c_int;
    pthread_rwlock_unlock(&mut tcplock);
    return copy as uint16_t;
}
pub unsafe extern "C" fn generate_iss() -> libc::c_int {
    let mut tmp: time_t = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = time(0 as *mut libc::c_void as *mut time_t);
    tmp___0 = rand();
    return tmp as libc::c_int * tmp___0;
}
pub unsafe extern "C" fn tcp_v4_connect(
    mut sk: *mut sock,
    mut addr: *const sockaddr,
    mut addrlen: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut dport: uint16_t = 0;
    let mut daddr: uint32_t = 0;
    let mut tmp: libc::c_int = 0;
    dport = (*(addr as *mut sockaddr_in)).sin_port;
    daddr = (*(addr as *mut sockaddr_in)).sin_addr.s_addr;
    (*sk).dport = ntohs(dport);
    (*sk).sport = generate_port();
    (*sk).daddr = ntohl(daddr);
    (*sk)
        .saddr = parse_ipv4_string(
        b"10.0.0.4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    tmp = tcp_connect(sk);
    return tmp;
}
pub unsafe extern "C" fn tcp_disconnect(
    mut sk: *mut sock,
    mut flags: libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tcp_write(
    mut sk: *mut sock,
    mut buf: *const libc::c_void,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut tsk: *mut tcp_sock = 0 as *mut tcp_sock;
    let mut ret: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tsk = sk as *mut tcp_sock;
    ret = (*sk).err;
    if !(ret != 0 as libc::c_int) {
        match (*sk).state {
            7 | 3 => {
                tmp = tcp_send(tsk, buf, len);
                return tmp;
            }
            _ => {
                ret = -(9 as libc::c_int);
            }
        }
    }
    return ret;
}
pub unsafe extern "C" fn tcp_read(
    mut sk: *mut sock,
    mut buf: *mut libc::c_void,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut tsk: *mut tcp_sock = 0 as *mut tcp_sock;
    let mut ret: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tsk = sk as *mut tcp_sock;
    ret = -(1 as libc::c_int);
    match (*sk).state {
        6 => {
            ret = -(9 as libc::c_int);
            current_block = 2207976713908481771;
        }
        5 | 4 | 3 | 2 | 1 | 0 => {
            current_block = 17860125682698302841;
        }
        7 => {
            tmp = skb_queue_empty(
                &mut (*tsk).sk.receive_queue as *mut sk_buff_head as *const sk_buff_head,
            );
            if tmp == 0 {
                current_block = 17860125682698302841;
            } else {
                if (*tsk).flags as libc::c_int & 1 as libc::c_int != 0 {
                    (*tsk)
                        .flags = ((*tsk).flags as libc::c_int & -(2 as libc::c_int))
                        as uint8_t;
                    return 0 as libc::c_int;
                }
                current_block = 17860125682698302841;
            }
        }
        10 | 9 | 8 => {
            ret = (*sk).err;
            current_block = 2207976713908481771;
        }
        _ => {
            current_block = 2207976713908481771;
        }
    }
    match current_block {
        2207976713908481771 => return ret,
        _ => {
            tmp___0 = tcp_receive(tsk, buf, len);
            return tmp___0;
        }
    };
}
pub unsafe extern "C" fn tcp_recv_notify(mut sk: *mut sock) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    if !(&mut (*sk).recv_wait as *mut wait_lock).is_null() {
        tmp = wait_wakeup(&mut (*sk).recv_wait);
        return tmp;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn tcp_close(mut sk: *mut sock) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    match (*sk).state {
        5 | 4 | 10 | 9 | 8 | 6 => {
            (*sk).err = -(9 as libc::c_int);
            return -(1 as libc::c_int);
        }
        2 | 1 | 0 => {
            tmp = tcp_done(sk);
            return tmp;
        }
        3 => {
            __tcp_set_state(sk, 4 as libc::c_int as uint32_t);
            tcp_queue_fin(sk);
        }
        7 => {
            tcp_queue_fin(sk);
        }
        _ => {
            fprintf(
                stderr,
                b"Unknown TCP state for close\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tcp_abort(mut sk: *mut sock) -> libc::c_int {
    let mut tsk: *mut tcp_sock = 0 as *mut tcp_sock;
    let mut tmp: libc::c_int = 0;
    tsk = sk as *mut tcp_sock;
    tcp_send_reset(tsk);
    tmp = tcp_done(sk);
    return tmp;
}
unsafe extern "C" fn tcp_free(mut sk: *mut sock) -> libc::c_int {
    let mut tsk: *mut tcp_sock = 0 as *mut tcp_sock;
    tsk = sk as *mut tcp_sock;
    tcp_clear_timers(sk);
    tcp_clear_queues(tsk);
    wait_wakeup(&mut (*(*sk).sock).sleep);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tcp_done(mut sk: *mut sock) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    __tcp_set_state(sk, 8 as libc::c_int as uint32_t);
    tcp_free(sk);
    tmp = socket_delete((*sk).sock);
    return tmp;
}
pub unsafe extern "C" fn tcp_clear_timers(mut sk: *mut sock) {
    let mut tsk: *mut tcp_sock = 0 as *mut tcp_sock;
    tsk = sk as *mut tcp_sock;
    tcp_stop_rto_timer(tsk);
    tcp_stop_delack_timer(tsk);
    timer_cancel((*tsk).keepalive);
    (*tsk).keepalive = 0 as *mut libc::c_void as *mut timer;
    timer_cancel((*tsk).linger);
    (*tsk).linger = 0 as *mut libc::c_void as *mut timer;
}
pub unsafe extern "C" fn tcp_stop_rto_timer(mut tsk: *mut tcp_sock) {
    if !tsk.is_null() {
        timer_cancel((*tsk).retransmit);
        (*tsk).retransmit = 0 as *mut libc::c_void as *mut timer;
        (*tsk).backoff = 0 as libc::c_int as uint8_t;
    }
}
pub unsafe extern "C" fn tcp_release_rto_timer(mut tsk: *mut tcp_sock) {
    if !tsk.is_null() {
        timer_release((*tsk).retransmit);
        (*tsk).retransmit = 0 as *mut libc::c_void as *mut timer;
    }
}
pub unsafe extern "C" fn tcp_stop_delack_timer(mut tsk: *mut tcp_sock) {
    timer_cancel((*tsk).delack);
    (*tsk).delack = 0 as *mut libc::c_void as *mut timer;
}
pub unsafe extern "C" fn tcp_release_delack_timer(mut tsk: *mut tcp_sock) {
    timer_release((*tsk).delack);
    (*tsk).delack = 0 as *mut libc::c_void as *mut timer;
}
pub unsafe extern "C" fn tcp_handle_fin_state(mut sk: *mut sock) {
    match (*sk).state {
        7 => {
            __tcp_set_state(sk, 9 as libc::c_int as uint32_t);
        }
        3 => {
            __tcp_set_state(sk, 4 as libc::c_int as uint32_t);
        }
        _ => {}
    };
}
unsafe extern "C" fn tcp_linger(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut sk: *mut sock = 0 as *mut sock;
    let mut tsk: *mut tcp_sock = 0 as *mut tcp_sock;
    sk = arg as *mut sock;
    socket_wr_acquire((*sk).sock);
    tsk = sk as *mut tcp_sock;
    timer_cancel((*tsk).linger);
    (*tsk).linger = 0 as *mut libc::c_void as *mut timer;
    tcp_done(sk);
    socket_release((*sk).sock);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn tcp_user_timeout(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut sk: *mut sock = 0 as *mut sock;
    let mut tsk: *mut tcp_sock = 0 as *mut tcp_sock;
    sk = arg as *mut sock;
    socket_wr_acquire((*sk).sock);
    tsk = sk as *mut tcp_sock;
    timer_cancel((*tsk).linger);
    (*tsk).linger = 0 as *mut libc::c_void as *mut timer;
    tcp_abort(sk);
    socket_release((*sk).sock);
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn tcp_enter_time_wait(mut sk: *mut sock) {
    let mut tsk: *mut tcp_sock = 0 as *mut tcp_sock;
    tsk = sk as *mut tcp_sock;
    __tcp_set_state(sk, 10 as libc::c_int as uint32_t);
    tcp_clear_timers(sk);
    (*tsk)
        .linger = timer_add(
        60000 as libc::c_int as uint32_t,
        Some(tcp_linger as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        sk as *mut libc::c_void,
    );
}
pub unsafe extern "C" fn tcp_rearm_user_timeout(mut sk: *mut sock) {
    let mut tsk: *mut tcp_sock = 0 as *mut tcp_sock;
    tsk = sk as *mut tcp_sock;
    if (*sk).state == 10 as libc::c_int {
        return;
    }
    timer_cancel((*tsk).linger);
    (*tsk)
        .linger = timer_add(
        180000 as libc::c_int as uint32_t,
        Some(
            tcp_user_timeout
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        sk as *mut libc::c_void,
    );
}
pub unsafe extern "C" fn tcp_rtt(mut tsk: *mut tcp_sock) {
    let mut r: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut beta: libc::c_double = 0.;
    let mut alpha: libc::c_double = 0.;
    let mut tmp___0: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    if (*tsk).backoff as libc::c_int > 0 as libc::c_int {
        return
    } else {
        if ((*tsk).retransmit).is_null() {
            return;
        }
    }
    tmp = timer_get_tick();
    r = (tmp as uint32_t)
        .wrapping_sub(((*(*tsk).retransmit).expires).wrapping_sub((*tsk).rto))
        as libc::c_int;
    if r < 0 as libc::c_int {
        return;
    }
    if (*tsk).srtt == 0 {
        (*tsk).srtt = r;
        (*tsk).rttvar = r / 2 as libc::c_int;
    } else {
        beta = 0.25f64;
        alpha = 0.125f64;
        tmp___0 = abs((*tsk).srtt - r);
        (*tsk)
            .rttvar = ((1 as libc::c_int as libc::c_double - beta)
            * (*tsk).rttvar as libc::c_double + beta * tmp___0 as libc::c_double)
            as int32_t;
        (*tsk)
            .srtt = ((1 as libc::c_int as libc::c_double - alpha)
            * (*tsk).srtt as libc::c_double + alpha * r as libc::c_double) as int32_t;
    }
    k = 4 as libc::c_int * (*tsk).rttvar;
    if k < 200 as libc::c_int {
        k = 200 as libc::c_int;
    }
    (*tsk).rto = ((*tsk).srtt + k) as uint32_t;
}
pub unsafe extern "C" fn tcp_calculate_sacks(mut tsk: *mut tcp_sock) -> libc::c_int {
    let mut sb: *mut tcp_sack_block = 0 as *mut tcp_sack_block;
    let mut next: *mut sk_buff = 0 as *mut sk_buff;
    let mut item: *mut list_head = 0 as *mut list_head;
    let mut tmp: *mut list_head = 0 as *mut list_head;
    sb = &mut *((*tsk).sacks).as_mut_ptr().offset((*tsk).sacklen as isize)
        as *mut tcp_sack_block;
    (*sb).left = 0 as libc::c_int as uint32_t;
    (*sb).right = 0 as libc::c_int as uint32_t;
    item = (*tsk).ofo_queue.head.next;
    tmp = (*item).next;
    while item as libc::c_ulong
        != &mut (*tsk).ofo_queue.head as *mut list_head as libc::c_ulong
    {
        next = (item as *mut libc::c_char)
            .offset(
                -(&mut (*(0 as *mut sk_buff)).list as *mut list_head as libc::c_ulong
                    as isize),
            ) as *mut sk_buff;
        if (*sb).left == 0 as libc::c_uint {
            (*sb).left = (*next).seq;
            (*tsk)
                .sacklen = ((*tsk).sacklen as libc::c_int + 1 as libc::c_int) as uint8_t;
        }
        if (*sb).right == 0 as libc::c_uint {
            (*sb).right = (*next).end_seq;
        } else if (*sb).right == (*next).seq {
            (*sb).right = (*next).end_seq;
        } else {
            if (*tsk).sacklen as libc::c_int >= (*tsk).sacks_allowed as libc::c_int {
                break;
            }
            sb = &mut *((*tsk).sacks).as_mut_ptr().offset((*tsk).sacklen as isize)
                as *mut tcp_sack_block;
            (*sb).left = (*next).seq;
            (*sb).right = (*next).end_seq;
            (*tsk)
                .sacklen = ((*tsk).sacklen as libc::c_int + 1 as libc::c_int) as uint8_t;
        }
        item = tmp;
        tmp = (*item).next;
    }
    return 0 as libc::c_int;
}
pub static mut debug: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn usage(mut app: *mut libc::c_char) {
    fprintf(stderr, b"Usage: %s\n\0" as *const u8 as *const libc::c_char, app);
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        stderr,
        b"Linux TCP/IP stack implemented with TUN/TAP devices.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"Requires the CAP_NET_ADMIN capability. See capabilities(7).\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"See https://www.kernel.org/doc/Documentation/networking/tuntap.txt\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(stderr, b"Options:\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        stderr,
        b"  -d Debug logging and tracing\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(stderr, b"  -h Print usage\n\0" as *const u8 as *const libc::c_char);
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    exit(1 as libc::c_int);
}
unsafe extern "C" fn parse_opts(
    mut argc: *mut libc::c_int,
    mut argv: *mut *mut *mut libc::c_char,
) -> libc::c_int {
    let mut opt: libc::c_int = 0;
    loop {
        opt = getopt(
            *argc,
            *argv as *const *mut libc::c_char,
            b"hd\0" as *const u8 as *const libc::c_char,
        );
        if !(opt != -(1 as libc::c_int)) {
            break;
        }
        match opt {
            100 => {
                debug = 1 as libc::c_int;
            }
            _ => {
                usage(**argv.offset(0 as libc::c_int as isize));
            }
        }
    }
    *argc -= optind;
    *argv = (*argv).offset(optind as isize);
    return optind;
}
pub unsafe extern "C" fn parse_cli(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    parse_opts(&mut argc, &mut argv);
}
pub unsafe extern "C" fn sk_alloc(
    mut ops: *mut net_ops,
    mut protocol: libc::c_int,
) -> *mut sock {
    let mut sk: *mut sock = 0 as *mut sock;
    sk = (Some(((*ops).alloc_sock).expect("non-null function pointer")))
        .expect("non-null function pointer")(protocol);
    (*sk).ops = ops;
    return sk;
}
pub unsafe extern "C" fn sock_init_data(mut sock: *mut socket, mut sk: *mut sock) {
    (*sock).sk = sk;
    (*sk).sock = sock;
    wait_init(&mut (*sk).recv_wait);
    skb_queue_init(&mut (*sk).receive_queue);
    skb_queue_init(&mut (*sk).write_queue);
    (*sk).poll_events = 0 as libc::c_int as libc::c_short;
    (Some(((*(*sk).ops).init).expect("non-null function pointer")))
        .expect("non-null function pointer")(sk);
}
pub unsafe extern "C" fn sock_free(mut sk: *mut sock) {
    skb_queue_free(&mut (*sk).receive_queue);
    skb_queue_free(&mut (*sk).write_queue);
}
pub unsafe extern "C" fn sock_connected(mut sk: *mut sock) {
    let mut sock: *mut socket = 0 as *mut socket;
    sock = (*sk).sock;
    (*sock).state = SS_CONNECTED;
    (*sk).err = 0 as libc::c_int;
    (*sk).poll_events = 772 as libc::c_int as libc::c_short;
    wait_wakeup(&mut (*sock).sleep);
}
#[inline]
unsafe extern "C" fn skb_queue_tail(mut list: *mut sk_buff_head, mut new: *mut sk_buff) {
    list_add_tail(&mut (*new).list, &mut (*list).head);
    (*list).qlen = ((*list).qlen).wrapping_add(1);
}
#[inline]
unsafe extern "C" fn write_queue_head(mut sk: *mut sock) -> *mut sk_buff {
    let mut tmp: *mut sk_buff = 0 as *mut sk_buff;
    tmp = skb_peek(&mut (*sk).write_queue);
    return tmp;
}
#[inline]
unsafe extern "C" fn tcp_hdr(mut skb: *const sk_buff) -> *mut tcphdr {
    return ((*skb).head)
        .offset(::std::mem::size_of::<eth_hdr>() as libc::c_ulong as isize)
        .offset(::std::mem::size_of::<iphdr>() as libc::c_ulong as isize) as *mut tcphdr;
}
unsafe extern "C" fn tcp_alloc_skb(
    mut optlen: libc::c_int,
    mut size: libc::c_int,
) -> *mut sk_buff {
    let mut reserved: libc::c_int = 0;
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    let mut tmp: *mut sk_buff = 0 as *mut sk_buff;
    reserved = (::std::mem::size_of::<eth_hdr>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<iphdr>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<tcphdr>() as libc::c_ulong)
        .wrapping_add(optlen as libc::c_ulong)
        .wrapping_add(size as libc::c_ulong) as libc::c_int;
    tmp = alloc_skb(reserved as libc::c_uint);
    skb = tmp;
    skb_reserve(skb, reserved as libc::c_uint);
    (*skb).protocol = 6 as libc::c_int as uint16_t;
    (*skb).dlen = size as uint32_t;
    (*skb).seq = 0 as libc::c_int as uint32_t;
    return skb;
}
unsafe extern "C" fn tcp_write_syn_options(
    mut th: *mut tcphdr,
    mut opts: *mut tcp_options,
    mut optlen: libc::c_int,
) -> libc::c_int {
    let mut opt_mss: *mut tcp_opt_mss = 0 as *mut tcp_opt_mss;
    let mut i: uint32_t = 0;
    let mut tmp: uint32_t = 0;
    let mut tmp___0: uint32_t = 0;
    let mut tmp___1: uint32_t = 0;
    let mut tmp___2: uint32_t = 0;
    opt_mss = ((*th).data).as_mut_ptr() as *mut tcp_opt_mss;
    i = 0 as libc::c_int as uint32_t;
    (*opt_mss).kind = 2 as libc::c_int as uint8_t;
    (*opt_mss).len = 4 as libc::c_int as uint8_t;
    (*opt_mss).mss = htons((*opts).mss);
    i = (i as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<tcp_opt_mss>() as libc::c_ulong) as uint32_t;
    if (*opts).sack != 0 {
        tmp = i;
        i = i.wrapping_add(1);
        *((*th).data).as_mut_ptr().offset(tmp as isize) = 1 as libc::c_int as uint8_t;
        tmp___0 = i;
        i = i.wrapping_add(1);
        *((*th).data)
            .as_mut_ptr()
            .offset(tmp___0 as isize) = 1 as libc::c_int as uint8_t;
        tmp___1 = i;
        i = i.wrapping_add(1);
        *((*th).data)
            .as_mut_ptr()
            .offset(tmp___1 as isize) = 4 as libc::c_int as uint8_t;
        tmp___2 = i;
        i = i.wrapping_add(1);
        *((*th).data)
            .as_mut_ptr()
            .offset(tmp___2 as isize) = 2 as libc::c_int as uint8_t;
    }
    (*th)
        .set_hl(
            (::std::mem::size_of::<tcphdr>() as libc::c_ulong)
                .wrapping_div(4 as libc::c_ulong)
                .wrapping_add((optlen / 4 as libc::c_int) as libc::c_ulong) as uint8_t,
        );
    return 0 as libc::c_int;
}
unsafe extern "C" fn tcp_syn_options(
    mut sk: *mut sock,
    mut opts: *mut tcp_options,
) -> libc::c_int {
    let mut tsk: *mut tcp_sock = 0 as *mut tcp_sock;
    let mut optlen: libc::c_int = 0;
    tsk = sk as *mut tcp_sock;
    optlen = 0 as libc::c_int;
    (*opts).mss = (*tsk).rmss;
    optlen += 4 as libc::c_int;
    if (*tsk).sackok != 0 {
        (*opts).sack = 1 as libc::c_int as uint8_t;
        optlen += 2 as libc::c_int;
        optlen += 2 as libc::c_int;
    } else {
        (*opts).sack = 0 as libc::c_int as uint8_t;
    }
    return optlen;
}
unsafe extern "C" fn tcp_write_options(
    mut tsk: *mut tcp_sock,
    mut th: *mut tcphdr,
) -> libc::c_int {
    let mut ptr: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___0: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___1: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___2: *mut uint8_t = 0 as *mut uint8_t;
    let mut sb: *mut tcp_sack_block = 0 as *mut tcp_sack_block;
    let mut i: libc::c_int = 0;
    ptr = ((*th).data).as_mut_ptr();
    if (*tsk).sackok == 0 {
        return 0 as libc::c_int
    } else {
        if (*tsk).sacks[0 as libc::c_int as usize].left == 0 as libc::c_uint {
            return 0 as libc::c_int;
        }
    }
    tmp = ptr;
    ptr = ptr.offset(1);
    *tmp = 1 as libc::c_int as uint8_t;
    tmp___0 = ptr;
    ptr = ptr.offset(1);
    *tmp___0 = 1 as libc::c_int as uint8_t;
    tmp___1 = ptr;
    ptr = ptr.offset(1);
    *tmp___1 = 5 as libc::c_int as uint8_t;
    tmp___2 = ptr;
    ptr = ptr.offset(1);
    *tmp___2 = (2 as libc::c_int + (*tsk).sacklen as libc::c_int * 8 as libc::c_int)
        as uint8_t;
    sb = ptr as *mut tcp_sack_block;
    i = (*tsk).sacklen as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        (*sb).left = htonl((*tsk).sacks[i as usize].left);
        (*sb).right = htonl((*tsk).sacks[i as usize].right);
        (*tsk).sacks[i as usize].left = 0 as libc::c_int as uint32_t;
        (*tsk).sacks[i as usize].right = 0 as libc::c_int as uint32_t;
        sb = sb.offset(1);
        ptr = ptr
            .offset(::std::mem::size_of::<tcp_sack_block>() as libc::c_ulong as isize);
        i -= 1;
    }
    (*tsk).sacklen = 0 as libc::c_int as uint8_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn tcp_transmit_skb(
    mut sk: *mut sock,
    mut skb: *mut sk_buff,
    mut seq: uint32_t,
) -> libc::c_int {
    let mut tsk: *mut tcp_sock = 0 as *mut tcp_sock;
    let mut tcb: *mut tcb = 0 as *mut tcb;
    let mut thdr: *mut tcphdr = 0 as *mut tcphdr;
    let mut tmp: *mut tcphdr = 0 as *mut tcphdr;
    let mut tmp___0: uint32_t = 0;
    let mut tmp___1: uint32_t = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    tsk = sk as *mut tcp_sock;
    tcb = &mut (*tsk).tcb;
    tmp = tcp_hdr(skb as *const sk_buff);
    thdr = tmp;
    if (*thdr).hl() as libc::c_int == 0 as libc::c_int {
        (*thdr)
            .set_hl(
                (::std::mem::size_of::<tcphdr>() as libc::c_ulong)
                    .wrapping_div(4 as libc::c_ulong) as uint8_t,
            );
    }
    skb_push(skb, ((*thdr).hl() as libc::c_int * 4 as libc::c_int) as libc::c_uint);
    (*thdr).sport = (*sk).sport;
    (*thdr).dport = (*sk).dport;
    (*thdr).seq = seq;
    (*thdr).ack_seq = (*tcb).rcv_nxt;
    (*thdr).set_rsvd(0 as libc::c_int as uint8_t);
    (*thdr).win = (*tcb).rcv_wnd as uint16_t;
    (*thdr).csum = 0 as libc::c_int as uint16_t;
    (*thdr).urp = 0 as libc::c_int as uint16_t;
    if (*thdr).hl() as libc::c_int > 5 as libc::c_int {
        tcp_write_options(tsk, thdr);
    }
    (*thdr).sport = htons((*thdr).sport);
    (*thdr).dport = htons((*thdr).dport);
    (*thdr).seq = htonl((*thdr).seq);
    (*thdr).ack_seq = htonl((*thdr).ack_seq);
    (*thdr).win = htons((*thdr).win);
    (*thdr).csum = htons((*thdr).csum);
    (*thdr).urp = htons((*thdr).urp);
    tmp___0 = htonl((*sk).daddr);
    tmp___1 = htonl((*sk).saddr);
    tmp___2 = tcp_v4_checksum(skb, tmp___1, tmp___0);
    (*thdr).csum = tmp___2 as uint16_t;
    tmp___3 = ip_output(sk, skb);
    return tmp___3;
}
unsafe extern "C" fn tcp_queue_transmit_skb(
    mut sk: *mut sock,
    mut skb: *mut sk_buff,
) -> libc::c_int {
    let mut tsk: *mut tcp_sock = 0 as *mut tcp_sock;
    let mut tcb: *mut tcb = 0 as *mut tcb;
    let mut th: *mut tcphdr = 0 as *mut tcphdr;
    let mut tmp: *mut tcphdr = 0 as *mut tcphdr;
    let mut rc: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tsk = sk as *mut tcp_sock;
    tcb = &mut (*tsk).tcb;
    tmp = tcp_hdr(skb as *const sk_buff);
    th = tmp;
    rc = 0 as libc::c_int;
    tmp___0 = skb_queue_empty(
        &mut (*sk).write_queue as *mut sk_buff_head as *const sk_buff_head,
    );
    if tmp___0 != 0 {
        tcp_rearm_rto_timer(tsk);
    }
    if (*tsk).inflight == 0 as libc::c_uint {
        rc = tcp_transmit_skb(sk, skb, (*tcb).snd_nxt);
        (*tsk).inflight = ((*tsk).inflight).wrapping_add(1);
        (*skb).seq = (*tcb).snd_nxt;
        (*tcb)
            .snd_nxt = ((*tcb).snd_nxt as libc::c_uint).wrapping_add((*skb).dlen)
            as uint32_t as uint32_t;
        (*skb).end_seq = (*tcb).snd_nxt;
        if (*th).fin() != 0 {
            (*tcb).snd_nxt = ((*tcb).snd_nxt).wrapping_add(1);
        }
    }
    skb_queue_tail(&mut (*sk).write_queue, skb);
    return rc;
}
pub unsafe extern "C" fn tcp_send_synack(mut sk: *mut sock) -> libc::c_int {
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    let mut th: *mut tcphdr = 0 as *mut tcphdr;
    let mut tcb: *mut tcb = 0 as *mut tcb;
    let mut rc: libc::c_int = 0;
    if (*sk).state != 1 as libc::c_int {
        fprintf(
            stderr,
            b"TCP synack: Socket was not in correct state (SYN_SENT)\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    tcb = &mut (*(sk as *mut tcp_sock)).tcb;
    rc = 0 as libc::c_int;
    skb = tcp_alloc_skb(0 as libc::c_int, 0 as libc::c_int);
    th = tcp_hdr(skb as *const sk_buff);
    (*th).set_syn(1 as libc::c_int as uint8_t);
    (*th).set_ack(1 as libc::c_int as uint8_t);
    rc = tcp_transmit_skb(sk, skb, (*tcb).snd_nxt);
    free_skb(skb);
    return rc;
}
pub unsafe extern "C" fn tcp_send_delack(
    mut arg: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut sk: *mut sock = 0 as *mut sock;
    let mut tsk: *mut tcp_sock = 0 as *mut tcp_sock;
    sk = arg as *mut sock;
    socket_wr_acquire((*sk).sock);
    tsk = sk as *mut tcp_sock;
    (*tsk).delacks = 0 as libc::c_int as uint8_t;
    tcp_release_delack_timer(tsk);
    tcp_send_ack(sk);
    socket_release((*sk).sock);
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn tcp_send_next(
    mut sk: *mut sock,
    mut amount: libc::c_int,
) -> libc::c_int {
    let mut tsk: *mut tcp_sock = 0 as *mut tcp_sock;
    let mut tcb: *mut tcb = 0 as *mut tcb;
    let mut th: *mut tcphdr = 0 as *mut tcphdr;
    let mut next: *mut sk_buff = 0 as *mut sk_buff;
    let mut item: *mut list_head = 0 as *mut list_head;
    let mut tmp: *mut list_head = 0 as *mut list_head;
    let mut i: libc::c_int = 0;
    tsk = sk as *mut tcp_sock;
    tcb = &mut (*tsk).tcb;
    i = 0 as libc::c_int;
    item = (*sk).write_queue.head.next;
    tmp = (*item).next;
    while item as libc::c_ulong
        != &mut (*sk).write_queue.head as *mut list_head as libc::c_ulong
    {
        i += 1;
        if i > amount {
            break;
        }
        next = (item as *mut libc::c_char)
            .offset(
                -(&mut (*(0 as *mut sk_buff)).list as *mut list_head as libc::c_ulong
                    as isize),
            ) as *mut sk_buff;
        if next as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            return -(1 as libc::c_int);
        }
        skb_reset_header(next);
        tcp_transmit_skb(sk, next, (*tcb).snd_nxt);
        (*next).seq = (*tcb).snd_nxt;
        (*tcb)
            .snd_nxt = ((*tcb).snd_nxt as libc::c_uint).wrapping_add((*next).dlen)
            as uint32_t as uint32_t;
        (*next).end_seq = (*tcb).snd_nxt;
        th = tcp_hdr(next as *const sk_buff);
        if (*th).fin() != 0 {
            (*tcb).snd_nxt = ((*tcb).snd_nxt).wrapping_add(1);
        }
        item = tmp;
        tmp = (*item).next;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn tcp_options_len(mut sk: *mut sock) -> libc::c_int {
    let mut tsk: *mut tcp_sock = 0 as *mut tcp_sock;
    let mut optlen: uint8_t = 0;
    let mut i: libc::c_int = 0;
    tsk = sk as *mut tcp_sock;
    optlen = 0 as libc::c_int as uint8_t;
    if (*tsk).sackok != 0 {
        if (*tsk).sacklen as libc::c_int > 0 as libc::c_int {
            i = 0 as libc::c_int;
            while i < (*tsk).sacklen as libc::c_int {
                if (*tsk).sacks[i as usize].left != 0 as libc::c_uint {
                    optlen = (optlen as libc::c_int + 8 as libc::c_int) as uint8_t;
                }
                i += 1;
            }
            optlen = (optlen as libc::c_int + 2 as libc::c_int) as uint8_t;
        }
    }
    while optlen as libc::c_int % 4 as libc::c_int > 0 as libc::c_int {
        optlen = (optlen as libc::c_int + 1 as libc::c_int) as uint8_t;
    }
    return optlen as libc::c_int;
}
pub unsafe extern "C" fn tcp_send_ack(mut sk: *mut sock) -> libc::c_int {
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    let mut th: *mut tcphdr = 0 as *mut tcphdr;
    let mut tcb: *mut tcb = 0 as *mut tcb;
    let mut rc: libc::c_int = 0;
    let mut optlen: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    if (*sk).state == 6 as libc::c_int {
        return 0 as libc::c_int;
    }
    tcb = &mut (*(sk as *mut tcp_sock)).tcb;
    rc = 0 as libc::c_int;
    tmp = tcp_options_len(sk);
    optlen = tmp;
    skb = tcp_alloc_skb(optlen, 0 as libc::c_int);
    th = tcp_hdr(skb as *const sk_buff);
    (*th).set_ack(1 as libc::c_int as uint8_t);
    (*th)
        .set_hl(
            (::std::mem::size_of::<tcphdr>() as libc::c_ulong)
                .wrapping_div(4 as libc::c_ulong)
                .wrapping_add((optlen / 4 as libc::c_int) as libc::c_ulong) as uint8_t,
        );
    rc = tcp_transmit_skb(sk, skb, (*tcb).snd_nxt);
    free_skb(skb);
    return rc;
}
unsafe extern "C" fn tcp_send_syn(mut sk: *mut sock) -> libc::c_int {
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    let mut th: *mut tcphdr = 0 as *mut tcphdr;
    let mut opts: tcp_options = tcp_options {
        options: 0,
        mss: 0,
        sack: 0,
    };
    let mut tcp_options_len___0: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    if (*sk).state != 1 as libc::c_int {
        if (*sk).state != 6 as libc::c_int {
            if (*sk).state != 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"Socket was not in correct state (closed or listen)\n\0"
                        as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
        }
    }
    opts.options = 0 as libc::c_int as uint16_t;
    opts.mss = 0 as libc::c_int as libc::c_ushort;
    opts.sack = 0 as libc::c_int as libc::c_uchar;
    tcp_options_len___0 = 0 as libc::c_int;
    tcp_options_len___0 = tcp_syn_options(sk, &mut opts);
    skb = tcp_alloc_skb(tcp_options_len___0, 0 as libc::c_int);
    th = tcp_hdr(skb as *const sk_buff);
    tcp_write_syn_options(th, &mut opts, tcp_options_len___0);
    (*sk).state = 1 as libc::c_int;
    (*th).set_syn(1 as libc::c_int as uint8_t);
    tmp = tcp_queue_transmit_skb(sk, skb);
    return tmp;
}
pub unsafe extern "C" fn tcp_send_fin(mut sk: *mut sock) -> libc::c_int {
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    let mut th: *mut tcphdr = 0 as *mut tcphdr;
    let mut rc: libc::c_int = 0;
    if (*sk).state == 6 as libc::c_int {
        return 0 as libc::c_int;
    }
    rc = 0 as libc::c_int;
    skb = tcp_alloc_skb(0 as libc::c_int, 0 as libc::c_int);
    th = tcp_hdr(skb as *const sk_buff);
    (*th).set_fin(1 as libc::c_int as uint8_t);
    (*th).set_ack(1 as libc::c_int as uint8_t);
    rc = tcp_queue_transmit_skb(sk, skb);
    return rc;
}
pub unsafe extern "C" fn tcp_select_initial_window(mut rcv_wnd: *mut uint32_t) {
    *rcv_wnd = 44477 as libc::c_int as uint32_t;
}
unsafe extern "C" fn tcp_notify_user(mut sk: *mut sock) {
    match (*sk).state {
        7 => {
            wait_wakeup(&mut (*(*sk).sock).sleep);
        }
        _ => {}
    };
}
unsafe extern "C" fn tcp_connect_rto(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut tsk: *mut tcp_sock = 0 as *mut tcp_sock;
    let mut tcb: *mut tcb = 0 as *mut tcb;
    let mut sk: *mut sock = 0 as *mut sock;
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    let mut tmp: *mut sk_buff = 0 as *mut sk_buff;
    tsk = arg as *mut tcp_sock;
    tcb = &mut (*tsk).tcb;
    sk = &mut (*tsk).sk;
    socket_wr_acquire((*sk).sock);
    tcp_release_rto_timer(tsk);
    if (*sk).state == 1 as libc::c_int {
        if (*tsk).backoff as libc::c_int > 3 as libc::c_int {
            (*tsk).sk.err = -(110 as libc::c_int);
            (*sk)
                .poll_events = ((*sk).poll_events as libc::c_int | 28 as libc::c_int)
                as libc::c_short;
            tcp_done(sk);
        } else {
            tmp = write_queue_head(sk);
            skb = tmp;
            if !skb.is_null() {
                skb_reset_header(skb);
                tcp_transmit_skb(sk, skb, (*tcb).snd_una);
                (*tsk)
                    .backoff = ((*tsk).backoff as libc::c_int + 1 as libc::c_int)
                    as uint8_t;
                tcp_rearm_rto_timer(tsk);
            }
        }
    } else {
        fprintf(
            stderr,
            b"TCP connect RTO triggered even when not in SYNSENT\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    socket_release((*sk).sock);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn tcp_retransmission_timeout(
    mut arg: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut tsk: *mut tcp_sock = 0 as *mut tcp_sock;
    let mut tcb: *mut tcb = 0 as *mut tcb;
    let mut sk: *mut sock = 0 as *mut sock;
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    let mut tmp: *mut sk_buff = 0 as *mut sk_buff;
    let mut th: *mut tcphdr = 0 as *mut tcphdr;
    let mut tmp___0: *mut tcphdr = 0 as *mut tcphdr;
    tsk = arg as *mut tcp_sock;
    tcb = &mut (*tsk).tcb;
    sk = &mut (*tsk).sk;
    socket_wr_acquire((*sk).sock);
    tcp_release_rto_timer(tsk);
    tmp = write_queue_head(sk);
    skb = tmp;
    if skb.is_null() {
        (*tsk).backoff = 0 as libc::c_int as uint8_t;
        tcp_notify_user(sk);
    } else {
        tmp___0 = tcp_hdr(skb as *const sk_buff);
        th = tmp___0;
        skb_reset_header(skb);
        tcp_transmit_skb(sk, skb, (*tcb).snd_una);
        if (*tsk).rto > 60000 as libc::c_uint {
            tcp_done(sk);
            (*tsk).sk.err = -(110 as libc::c_int);
            (*sk)
                .poll_events = ((*sk).poll_events as libc::c_int | 28 as libc::c_int)
                as libc::c_short;
            socket_release((*sk).sock);
            return 0 as *mut libc::c_void;
        } else {
            (*tsk)
                .rto = ((*tsk).rto as libc::c_uint).wrapping_mul(2 as libc::c_uint)
                as uint32_t as uint32_t;
            (*tsk)
                .backoff = ((*tsk).backoff as libc::c_int + 1 as libc::c_int) as uint8_t;
            (*tsk)
                .retransmit = timer_add(
                (*tsk).rto,
                Some(
                    tcp_retransmission_timeout
                        as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                ),
                tsk as *mut libc::c_void,
            );
            if (*th).fin() != 0 {
                tcp_handle_fin_state(sk);
            }
        }
    }
    socket_release((*sk).sock);
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn tcp_rearm_rto_timer(mut tsk: *mut tcp_sock) {
    let mut sk: *mut sock = 0 as *mut sock;
    sk = &mut (*tsk).sk;
    tcp_release_rto_timer(tsk);
    if (*sk).state == 1 as libc::c_int {
        (*tsk)
            .retransmit = timer_add(
            ((500 as libc::c_int) << (*tsk).backoff as libc::c_int) as uint32_t,
            Some(
                tcp_connect_rto
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            tsk as *mut libc::c_void,
        );
    } else {
        (*tsk)
            .retransmit = timer_add(
            (*tsk).rto,
            Some(
                tcp_retransmission_timeout
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            tsk as *mut libc::c_void,
        );
    };
}
pub unsafe extern "C" fn tcp_connect(mut sk: *mut sock) -> libc::c_int {
    let mut tsk: *mut tcp_sock = 0 as *mut tcp_sock;
    let mut tcb: *mut tcb = 0 as *mut tcb;
    let mut rc: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tsk = sk as *mut tcp_sock;
    tcb = &mut (*tsk).tcb;
    rc = 0 as libc::c_int;
    (*tsk).tcp_header_len = ::std::mem::size_of::<tcphdr>() as libc::c_ulong as uint16_t;
    tmp = generate_iss();
    (*tcb).iss = tmp as uint32_t;
    (*tcb).snd_wnd = 0 as libc::c_int as uint32_t;
    (*tcb).snd_wl1 = 0 as libc::c_int as uint32_t;
    (*tcb).snd_una = (*tcb).iss;
    (*tcb).snd_up = (*tcb).iss;
    (*tcb).snd_nxt = (*tcb).iss;
    (*tcb).rcv_nxt = 0 as libc::c_int as uint32_t;
    tcp_select_initial_window(&mut (*tsk).tcb.rcv_wnd);
    rc = tcp_send_syn(sk);
    (*tcb).snd_nxt = ((*tcb).snd_nxt).wrapping_add(1);
    return rc;
}
pub unsafe extern "C" fn tcp_send(
    mut tsk: *mut tcp_sock,
    mut buf: *const libc::c_void,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    let mut th: *mut tcphdr = 0 as *mut tcphdr;
    let mut slen: libc::c_int = 0;
    let mut mss: libc::c_int = 0;
    let mut dlen: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    slen = len;
    mss = (*tsk).smss as libc::c_int;
    dlen = 0 as libc::c_int;
    while slen > 0 as libc::c_int {
        if slen > mss {
            dlen = mss;
        } else {
            dlen = slen;
        }
        slen -= dlen;
        skb = tcp_alloc_skb(0 as libc::c_int, dlen);
        skb_push(skb, dlen as libc::c_uint);
        memcpy((*skb).data as *mut libc::c_void, buf, dlen as size_t);
        buf = buf.offset(dlen as isize);
        th = tcp_hdr(skb as *const sk_buff);
        (*th).set_ack(1 as libc::c_int as uint8_t);
        if slen == 0 as libc::c_int {
            (*th).set_psh(1 as libc::c_int as uint8_t);
        }
        tmp = tcp_queue_transmit_skb(&mut (*tsk).sk, skb);
        if tmp == -(1 as libc::c_int) {
            perror(b"Error on TCP skb queueing\0" as *const u8 as *const libc::c_char);
        }
    }
    tcp_rearm_user_timeout(&mut (*tsk).sk);
    return len;
}
pub unsafe extern "C" fn tcp_send_reset(mut tsk: *mut tcp_sock) -> libc::c_int {
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    let mut th: *mut tcphdr = 0 as *mut tcphdr;
    let mut tcb: *mut tcb = 0 as *mut tcb;
    let mut rc: libc::c_int = 0;
    rc = 0 as libc::c_int;
    skb = tcp_alloc_skb(0 as libc::c_int, 0 as libc::c_int);
    th = tcp_hdr(skb as *const sk_buff);
    tcb = &mut (*tsk).tcb;
    (*th).set_rst(1 as libc::c_int as uint8_t);
    (*tcb).snd_una = (*tcb).snd_nxt;
    rc = tcp_transmit_skb(&mut (*tsk).sk, skb, (*tcb).snd_nxt);
    free_skb(skb);
    return rc;
}
pub unsafe extern "C" fn tcp_send_challenge_ack(
    mut sk: *mut sock,
    mut skb: *mut sk_buff,
) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tcp_queue_fin(mut sk: *mut sock) -> libc::c_int {
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    let mut th: *mut tcphdr = 0 as *mut tcphdr;
    let mut rc: libc::c_int = 0;
    rc = 0 as libc::c_int;
    skb = tcp_alloc_skb(0 as libc::c_int, 0 as libc::c_int);
    th = tcp_hdr(skb as *const sk_buff);
    (*th).set_fin(1 as libc::c_int as uint8_t);
    (*th).set_ack(1 as libc::c_int as uint8_t);
    rc = tcp_queue_transmit_skb(sk, skb);
    return rc;
}
#[inline]
unsafe extern "C" fn arp_hdr(mut skb: *mut sk_buff) -> *mut arp_hdr {
    return ((*skb).head)
        .offset(::std::mem::size_of::<eth_hdr>() as libc::c_ulong as isize)
        as *mut arp_hdr;
}
static mut broadcast_hw: [uint8_t; 6] = [
    255 as libc::c_int as uint8_t,
    255 as libc::c_int as uint8_t,
    255 as libc::c_int as uint8_t,
    255 as libc::c_int as uint8_t,
    255 as libc::c_int as uint8_t,
    255 as libc::c_int as uint8_t,
];
static mut arp_cache: list_head = unsafe {
    {
        let mut init = list_head {
            next: &arp_cache as *const list_head as *mut list_head,
            prev: &arp_cache as *const list_head as *mut list_head,
        };
        init
    }
};
static mut lock: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
unsafe extern "C" fn arp_alloc_skb() -> *mut sk_buff {
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    let mut tmp: *mut sk_buff = 0 as *mut sk_buff;
    tmp = alloc_skb(
        (::std::mem::size_of::<eth_hdr>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<arp_hdr>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<arp_ipv4>() as libc::c_ulong)
            as libc::c_uint,
    );
    skb = tmp;
    skb_reserve(
        skb,
        (::std::mem::size_of::<eth_hdr>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<arp_hdr>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<arp_ipv4>() as libc::c_ulong)
            as libc::c_uint,
    );
    (*skb).protocol = htons(2054 as libc::c_int as uint16_t);
    return skb;
}
unsafe extern "C" fn arp_entry_alloc(
    mut hdr: *mut arp_hdr,
    mut data: *mut arp_ipv4,
) -> *mut arp_cache_entry {
    let mut entry: *mut arp_cache_entry = 0 as *mut arp_cache_entry;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<arp_cache_entry>() as libc::c_ulong);
    entry = tmp as *mut arp_cache_entry;
    list_init(&mut (*entry).list);
    (*entry).state = 2 as libc::c_uint;
    (*entry).hwtype = (*hdr).hwtype;
    (*entry).sip = (*data).sip;
    memcpy(
        ((*entry).smac).as_mut_ptr() as *mut libc::c_void,
        ((*data).smac).as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; 6]>() as libc::c_ulong,
    );
    return entry;
}
unsafe extern "C" fn insert_arp_translation_table(
    mut hdr: *mut arp_hdr,
    mut data: *mut arp_ipv4,
) -> libc::c_int {
    let mut entry: *mut arp_cache_entry = 0 as *mut arp_cache_entry;
    let mut tmp: *mut arp_cache_entry = 0 as *mut arp_cache_entry;
    tmp = arp_entry_alloc(hdr, data);
    entry = tmp;
    pthread_mutex_lock(&mut lock);
    list_add_tail(&mut (*entry).list, &mut arp_cache);
    pthread_mutex_unlock(&mut lock);
    return 0 as libc::c_int;
}
unsafe extern "C" fn update_arp_translation_table(
    mut hdr: *mut arp_hdr,
    mut data: *mut arp_ipv4,
) -> libc::c_int {
    let mut item: *mut list_head = 0 as *mut list_head;
    let mut entry: *mut arp_cache_entry = 0 as *mut arp_cache_entry;
    pthread_mutex_lock(&mut lock);
    item = arp_cache.next;
    while item as libc::c_ulong != &mut arp_cache as *mut list_head as libc::c_ulong {
        entry = (item as *mut libc::c_char)
            .offset(
                -(&mut (*(0 as *mut arp_cache_entry)).list as *mut list_head
                    as libc::c_ulong as isize),
            ) as *mut arp_cache_entry;
        if (*entry).hwtype as libc::c_int == (*hdr).hwtype as libc::c_int {
            if (*entry).sip == (*data).sip {
                memcpy(
                    ((*entry).smac).as_mut_ptr() as *mut libc::c_void,
                    ((*data).smac).as_mut_ptr() as *const libc::c_void,
                    6 as libc::c_int as size_t,
                );
                pthread_mutex_unlock(&mut lock);
                return 1 as libc::c_int;
            }
        }
        item = (*item).next;
    }
    pthread_mutex_unlock(&mut lock);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn arp_init() {}
pub unsafe extern "C" fn arp_rcv(mut skb: *mut sk_buff) {
    let mut current_block: u64;
    let mut arphdr: *mut arp_hdr = 0 as *mut arp_hdr;
    let mut arpdata: *mut arp_ipv4 = 0 as *mut arp_ipv4;
    let mut netdev___0: *mut netdev = 0 as *mut netdev;
    let mut merge: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    merge = 0 as libc::c_int;
    arphdr = arp_hdr(skb);
    (*arphdr).hwtype = ntohs((*arphdr).hwtype);
    (*arphdr).protype = ntohs((*arphdr).protype);
    (*arphdr).opcode = ntohs((*arphdr).opcode);
    if (*arphdr).hwtype as libc::c_int != 1 as libc::c_int {
        printf(b"ARP: Unsupported HW type\n\0" as *const u8 as *const libc::c_char);
    } else if (*arphdr).protype as libc::c_int != 2048 as libc::c_int {
        printf(b"ARP: Unsupported protocol\n\0" as *const u8 as *const libc::c_char);
    } else {
        arpdata = ((*arphdr).data).as_mut_ptr() as *mut arp_ipv4;
        (*arpdata).sip = ntohl((*arpdata).sip);
        (*arpdata).dip = ntohl((*arpdata).dip);
        merge = update_arp_translation_table(arphdr, arpdata);
        netdev___0 = netdev_get((*arpdata).dip);
        if netdev___0.is_null() {
            printf(b"ARP was not for us\n\0" as *const u8 as *const libc::c_char);
        } else {
            if merge == 0 {
                tmp = insert_arp_translation_table(arphdr, arpdata);
                if tmp != 0 as libc::c_int {
                    fprintf(
                        stderr,
                        b"ERR: No free space in ARP translation table\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    current_block = 7334016002892010387;
                } else {
                    current_block = 6009453772311597924;
                }
            } else {
                current_block = 6009453772311597924;
            }
            match current_block {
                7334016002892010387 => {}
                _ => {
                    match (*arphdr).opcode as libc::c_int {
                        1 => {
                            arp_reply(skb, netdev___0);
                            return;
                        }
                        _ => {
                            printf(
                                b"ARP: Opcode not supported\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                    }
                }
            }
        }
    }
    free_skb(skb);
}
pub unsafe extern "C" fn arp_request(
    mut sip: uint32_t,
    mut dip: uint32_t,
    mut netdev___0: *mut netdev,
) -> libc::c_int {
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    let mut arp: *mut arp_hdr = 0 as *mut arp_hdr;
    let mut payload: *mut arp_ipv4 = 0 as *mut arp_ipv4;
    let mut rc: libc::c_int = 0;
    let mut tmp: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___0: *mut uint8_t = 0 as *mut uint8_t;
    rc = 0 as libc::c_int;
    skb = arp_alloc_skb();
    if skb.is_null() {
        return -(1 as libc::c_int);
    }
    (*skb).dev = netdev___0;
    tmp = skb_push(
        skb,
        ::std::mem::size_of::<arp_ipv4>() as libc::c_ulong as libc::c_uint,
    );
    payload = tmp as *mut arp_ipv4;
    memcpy(
        ((*payload).smac).as_mut_ptr() as *mut libc::c_void,
        ((*netdev___0).hwaddr).as_mut_ptr() as *const libc::c_void,
        (*netdev___0).addr_len as size_t,
    );
    (*payload).sip = sip;
    memcpy(
        ((*payload).dmac).as_mut_ptr() as *mut libc::c_void,
        broadcast_hw.as_mut_ptr() as *const libc::c_void,
        (*netdev___0).addr_len as size_t,
    );
    (*payload).dip = dip;
    tmp___0 = skb_push(
        skb,
        ::std::mem::size_of::<arp_hdr>() as libc::c_ulong as libc::c_uint,
    );
    arp = tmp___0 as *mut arp_hdr;
    (*arp).opcode = htons(1 as libc::c_int as uint16_t);
    (*arp).hwtype = htons(1 as libc::c_int as uint16_t);
    (*arp).protype = htons(2048 as libc::c_int as uint16_t);
    (*arp).hwsize = (*netdev___0).addr_len;
    (*arp).prosize = 4 as libc::c_int as uint8_t;
    (*payload).sip = htonl((*payload).sip);
    (*payload).dip = htonl((*payload).dip);
    rc = netdev_transmit(
        skb,
        broadcast_hw.as_mut_ptr(),
        2054 as libc::c_int as uint16_t,
    );
    free_skb(skb);
    return rc;
}
pub unsafe extern "C" fn arp_reply(mut skb: *mut sk_buff, mut netdev___0: *mut netdev) {
    let mut arphdr: *mut arp_hdr = 0 as *mut arp_hdr;
    let mut arpdata: *mut arp_ipv4 = 0 as *mut arp_ipv4;
    arphdr = arp_hdr(skb);
    skb_reserve(
        skb,
        (::std::mem::size_of::<eth_hdr>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<arp_hdr>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<arp_ipv4>() as libc::c_ulong)
            as libc::c_uint,
    );
    skb_push(
        skb,
        (::std::mem::size_of::<arp_hdr>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<arp_ipv4>() as libc::c_ulong)
            as libc::c_uint,
    );
    arpdata = ((*arphdr).data).as_mut_ptr() as *mut arp_ipv4;
    memcpy(
        ((*arpdata).dmac).as_mut_ptr() as *mut libc::c_void,
        ((*arpdata).smac).as_mut_ptr() as *const libc::c_void,
        6 as libc::c_int as size_t,
    );
    (*arpdata).dip = (*arpdata).sip;
    memcpy(
        ((*arpdata).smac).as_mut_ptr() as *mut libc::c_void,
        ((*netdev___0).hwaddr).as_mut_ptr() as *const libc::c_void,
        6 as libc::c_int as size_t,
    );
    (*arpdata).sip = (*netdev___0).addr;
    (*arphdr).opcode = 2 as libc::c_int as uint16_t;
    (*arphdr).opcode = htons((*arphdr).opcode);
    (*arphdr).hwtype = htons((*arphdr).hwtype);
    (*arphdr).protype = htons((*arphdr).protype);
    (*arpdata).sip = htonl((*arpdata).sip);
    (*arpdata).dip = htonl((*arpdata).dip);
    (*skb).dev = netdev___0;
    netdev_transmit(
        skb,
        ((*arpdata).dmac).as_mut_ptr(),
        2054 as libc::c_int as uint16_t,
    );
    free_skb(skb);
}
pub unsafe extern "C" fn arp_get_hwaddr(mut sip: uint32_t) -> *mut libc::c_uchar {
    let mut item: *mut list_head = 0 as *mut list_head;
    let mut entry: *mut arp_cache_entry = 0 as *mut arp_cache_entry;
    let mut copy: *mut uint8_t = 0 as *mut uint8_t;
    pthread_mutex_lock(&mut lock);
    item = arp_cache.next;
    while item as libc::c_ulong != &mut arp_cache as *mut list_head as libc::c_ulong {
        entry = (item as *mut libc::c_char)
            .offset(
                -(&mut (*(0 as *mut arp_cache_entry)).list as *mut list_head
                    as libc::c_ulong as isize),
            ) as *mut arp_cache_entry;
        if (*entry).state == 2 as libc::c_uint {
            if (*entry).sip == sip {
                copy = ((*entry).smac).as_mut_ptr();
                pthread_mutex_unlock(&mut lock);
                return copy;
            }
        }
        item = (*item).next;
    }
    pthread_mutex_unlock(&mut lock);
    return 0 as *mut libc::c_void as *mut libc::c_uchar;
}
pub unsafe extern "C" fn free_arp() {
    let mut item: *mut list_head = 0 as *mut list_head;
    let mut tmp: *mut list_head = 0 as *mut list_head;
    let mut entry: *mut arp_cache_entry = 0 as *mut arp_cache_entry;
    item = arp_cache.next;
    tmp = (*item).next;
    while item as libc::c_ulong != &mut arp_cache as *mut list_head as libc::c_ulong {
        entry = (item as *mut libc::c_char)
            .offset(
                -(&mut (*(0 as *mut arp_cache_entry)).list as *mut list_head
                    as libc::c_ulong as isize),
            ) as *mut arp_cache_entry;
        list_del(item);
        free(entry as *mut libc::c_void);
        item = tmp;
        tmp = (*item).next;
    }
}
#[inline]
unsafe extern "C" fn eth_hdr(mut skb: *mut sk_buff) -> *mut eth_hdr {
    let mut hdr: *mut eth_hdr = 0 as *mut eth_hdr;
    let mut tmp: *mut uint8_t = 0 as *mut uint8_t;
    tmp = skb_head(skb);
    hdr = tmp as *mut eth_hdr;
    (*hdr).ethertype = ntohs((*hdr).ethertype);
    return hdr;
}
#[inline]
unsafe extern "C" fn ip_parse(mut addr: *mut libc::c_char) -> uint32_t {
    let mut dst: uint32_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: uint32_t = 0;
    dst = 0 as libc::c_int as uint32_t;
    tmp = inet_pton(
        2 as libc::c_int,
        addr as *const libc::c_char,
        &mut dst as *mut uint32_t as *mut libc::c_void,
    );
    if tmp != 1 as libc::c_int {
        perror(
            b"ERR: Parsing inet address failed\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    tmp___0 = ntohl(dst);
    return tmp___0;
}
pub static mut loop_0: *mut netdev = 0 as *const netdev as *mut netdev;
pub static mut netdev: *mut netdev = 0 as *const netdev as *mut netdev;
unsafe extern "C" fn netdev_alloc(
    mut addr: *mut libc::c_char,
    mut hwaddr: *mut libc::c_char,
    mut mtu: uint32_t,
) -> *mut netdev {
    let mut dev___0: *mut netdev = 0 as *mut netdev;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<netdev>() as libc::c_ulong);
    dev___0 = tmp as *mut netdev;
    (*dev___0).addr = ip_parse(addr);
    sscanf(
        hwaddr as *const libc::c_char,
        b"%hhx:%hhx:%hhx:%hhx:%hhx:%hhx\0" as *const u8 as *const libc::c_char,
        &mut *((*dev___0).hwaddr).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut uint8_t,
        &mut *((*dev___0).hwaddr).as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut uint8_t,
        &mut *((*dev___0).hwaddr).as_mut_ptr().offset(2 as libc::c_int as isize)
            as *mut uint8_t,
        &mut *((*dev___0).hwaddr).as_mut_ptr().offset(3 as libc::c_int as isize)
            as *mut uint8_t,
        &mut *((*dev___0).hwaddr).as_mut_ptr().offset(4 as libc::c_int as isize)
            as *mut uint8_t,
        &mut *((*dev___0).hwaddr).as_mut_ptr().offset(5 as libc::c_int as isize)
            as *mut uint8_t,
    );
    (*dev___0).addr_len = 6 as libc::c_int as uint8_t;
    (*dev___0).mtu = mtu;
    return dev___0;
}
pub unsafe extern "C" fn netdev_init(
    mut addr: *mut libc::c_char,
    mut hwaddr: *mut libc::c_char,
) {
    loop_0 = netdev_alloc(
        b"127.0.0.1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"00:00:00:00:00:00\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1500 as libc::c_int as uint32_t,
    );
    netdev = netdev_alloc(
        b"10.0.0.4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"00:0c:29:6d:50:25\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1500 as libc::c_int as uint32_t,
    );
}
pub unsafe extern "C" fn netdev_transmit(
    mut skb: *mut sk_buff,
    mut dst_hw: *mut uint8_t,
    mut ethertype: uint16_t,
) -> libc::c_int {
    let mut dev___0: *mut netdev = 0 as *mut netdev;
    let mut hdr: *mut eth_hdr = 0 as *mut eth_hdr;
    let mut ret: libc::c_int = 0;
    ret = 0 as libc::c_int;
    dev___0 = (*skb).dev;
    skb_push(skb, ::std::mem::size_of::<eth_hdr>() as libc::c_ulong as libc::c_uint);
    hdr = (*skb).data as *mut eth_hdr;
    memcpy(
        ((*hdr).dmac).as_mut_ptr() as *mut libc::c_void,
        dst_hw as *const libc::c_void,
        (*dev___0).addr_len as size_t,
    );
    memcpy(
        ((*hdr).smac).as_mut_ptr() as *mut libc::c_void,
        ((*dev___0).hwaddr).as_mut_ptr() as *const libc::c_void,
        (*dev___0).addr_len as size_t,
    );
    (*hdr).ethertype = htons(ethertype);
    ret = tun_write((*skb).data as *mut libc::c_char, (*skb).len as libc::c_int);
    return ret;
}
unsafe extern "C" fn netdev_receive(mut skb: *mut sk_buff) -> libc::c_int {
    let mut hdr: *mut eth_hdr = 0 as *mut eth_hdr;
    let mut tmp: *mut eth_hdr = 0 as *mut eth_hdr;
    tmp = eth_hdr(skb);
    hdr = tmp;
    match (*hdr).ethertype as libc::c_int {
        2054 => {
            arp_rcv(skb);
        }
        2048 => {
            ip_rcv(skb);
        }
        _ => {
            printf(
                b"Unsupported ethertype %x\n\0" as *const u8 as *const libc::c_char,
                (*hdr).ethertype as libc::c_int,
            );
            free_skb(skb);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn netdev_rx_loop() -> *mut libc::c_void {
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    let mut tmp: *mut sk_buff = 0 as *mut sk_buff;
    let mut tmp___0: libc::c_int = 0;
    while running != 0 {
        tmp = alloc_skb(1600 as libc::c_uint);
        skb = tmp;
        tmp___0 = tun_read((*skb).data as *mut libc::c_char, 1600 as libc::c_int);
        if tmp___0 < 0 as libc::c_int {
            perror(b"ERR: Read from tun_fd\0" as *const u8 as *const libc::c_char);
            free_skb(skb);
            return 0 as *mut libc::c_void;
        }
        netdev_receive(skb);
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn netdev_get(mut sip: uint32_t) -> *mut netdev {
    if (*netdev).addr == sip {
        return netdev
    } else {
        return 0 as *mut libc::c_void as *mut netdev
    };
}
pub unsafe extern "C" fn free_netdev() {
    free(loop_0 as *mut libc::c_void);
    free(netdev as *mut libc::c_void);
}
pub unsafe extern "C" fn run_cmd(
    mut cmd: *mut libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    let mut buf: [libc::c_char; 100] = [0; 100];
    let mut tmp: libc::c_int = 0;
    ap = args.clone();
    vsnprintf(
        buf.as_mut_ptr(),
        100 as libc::c_int as size_t,
        cmd as *const libc::c_char,
        ap.as_va_list(),
    );
    if debug != 0 {
        printf(b"EXEC: %s\n\0" as *const u8 as *const libc::c_char, buf.as_mut_ptr());
    }
    tmp = system(buf.as_mut_ptr() as *const libc::c_char);
    return tmp;
}
pub unsafe extern "C" fn sum_every_16bits(
    mut addr: *mut libc::c_void,
    mut count: libc::c_int,
) -> uint32_t {
    let mut sum: uint32_t = 0;
    let mut ptr: *mut uint16_t = 0 as *mut uint16_t;
    let mut tmp: *mut uint16_t = 0 as *mut uint16_t;
    sum = 0 as libc::c_int as uint32_t;
    ptr = addr as *mut uint16_t;
    while count > 1 as libc::c_int {
        tmp = ptr;
        ptr = ptr.offset(1);
        sum = (sum as libc::c_uint).wrapping_add(*tmp as uint32_t) as uint32_t
            as uint32_t;
        count -= 2 as libc::c_int;
    }
    if count > 0 as libc::c_int {
        sum = (sum as libc::c_uint).wrapping_add(*(ptr as *mut uint8_t) as uint32_t)
            as uint32_t as uint32_t;
    }
    return sum;
}
pub unsafe extern "C" fn checksum(
    mut addr: *mut libc::c_void,
    mut count: libc::c_int,
    mut start_sum: libc::c_int,
) -> uint16_t {
    let mut sum: uint32_t = 0;
    let mut tmp: uint32_t = 0;
    sum = start_sum as uint32_t;
    tmp = sum_every_16bits(addr, count);
    sum = (sum as libc::c_uint).wrapping_add(tmp) as uint32_t as uint32_t;
    while sum >> 16 as libc::c_int != 0 {
        sum = (sum & 65535 as libc::c_uint).wrapping_add(sum >> 16 as libc::c_int);
    }
    return !sum as uint16_t;
}
pub unsafe extern "C" fn get_address(
    mut host: *mut libc::c_char,
    mut port___0: *mut libc::c_char,
    mut addr: *mut sockaddr,
) -> libc::c_int {
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut result: *mut addrinfo = 0 as *mut addrinfo;
    let mut rp: *mut addrinfo = 0 as *mut addrinfo;
    let mut s: libc::c_int = 0;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_family = 2 as libc::c_int;
    hints.ai_socktype = 1 as libc::c_int;
    s = getaddrinfo(
        host as *const libc::c_char,
        port___0 as *const libc::c_char,
        &mut hints as *mut addrinfo as *const addrinfo,
        &mut result as *mut *mut addrinfo,
    );
    if s != 0 as libc::c_int {
        tmp = gai_strerror(s);
        fprintf(stderr, b"getaddrinfo: %s\n\0" as *const u8 as *const libc::c_char, tmp);
        exit(1 as libc::c_int);
    }
    rp = result;
    if rp as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        *addr = *(*rp).ai_addr;
        freeaddrinfo(result);
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn parse_ipv4_string(mut addr: *mut libc::c_char) -> uint32_t {
    let mut addr_bytes: [uint8_t; 4] = [0; 4];
    sscanf(
        addr as *const libc::c_char,
        b"%hhu.%hhu.%hhu.%hhu\0" as *const u8 as *const libc::c_char,
        &mut *addr_bytes.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut uint8_t,
        &mut *addr_bytes.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut uint8_t,
        &mut *addr_bytes.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut uint8_t,
        &mut *addr_bytes.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut uint8_t,
    );
    return (addr_bytes[0 as libc::c_int as usize] as libc::c_int
        | (addr_bytes[1 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int
        | (addr_bytes[2 as libc::c_int as usize] as libc::c_int) << 16 as libc::c_int
        | (addr_bytes[3 as libc::c_int as usize] as libc::c_int) << 24 as libc::c_int)
        as uint32_t;
}
pub unsafe extern "C" fn min(mut x: uint32_t, mut y: uint32_t) -> uint32_t {
    let mut tmp: uint32_t = 0;
    if x > y {
        tmp = y;
    } else {
        tmp = x;
    }
    return tmp;
}
static mut sockets___0: list_head = unsafe {
    {
        let mut init = list_head {
            next: &sockets___0 as *const list_head as *mut list_head,
            prev: &sockets___0 as *const list_head as *mut list_head,
        };
        init
    }
};
static mut lock___0: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
static mut socket_count: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn ipc_alloc_thread(mut sock: libc::c_int) -> *mut ipc_thread {
    let mut th: *mut ipc_thread = 0 as *mut ipc_thread;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        ::std::mem::size_of::<ipc_thread>() as libc::c_ulong,
        1 as libc::c_int as size_t,
    );
    th = tmp as *mut ipc_thread;
    list_init(&mut (*th).list);
    (*th).sock = sock;
    pthread_mutex_lock(&mut lock___0);
    list_add_tail(&mut (*th).list, &mut sockets___0);
    socket_count += 1;
    pthread_mutex_unlock(&mut lock___0);
    return th;
}
unsafe extern "C" fn ipc_free_thread(mut sock: libc::c_int) {
    let mut item: *mut list_head = 0 as *mut list_head;
    let mut tmp: *mut list_head = 0 as *mut list_head;
    let mut th: *mut ipc_thread = 0 as *mut ipc_thread;
    tmp = 0 as *mut libc::c_void as *mut list_head;
    th = 0 as *mut libc::c_void as *mut ipc_thread;
    pthread_mutex_lock(&mut lock___0);
    item = sockets___0.next;
    tmp = (*item).next;
    while item as libc::c_ulong != &mut sockets___0 as *mut list_head as libc::c_ulong {
        th = (item as *mut libc::c_char)
            .offset(
                -(&mut (*(0 as *mut ipc_thread)).list as *mut list_head as libc::c_ulong
                    as isize),
            ) as *mut ipc_thread;
        if (*th).sock == sock {
            list_del(&mut (*th).list);
            close((*th).sock);
            free(th as *mut libc::c_void);
            socket_count -= 1;
            break;
        } else {
            item = tmp;
            tmp = (*item).next;
        }
    }
    pthread_mutex_unlock(&mut lock___0);
}
unsafe extern "C" fn ipc_try_send(
    mut sockfd: libc::c_int,
    mut buf: *const libc::c_void,
    mut len: size_t,
) -> libc::c_int {
    let mut tmp: ssize_t = 0;
    tmp = send(sockfd, buf, len, 16384 as libc::c_int);
    return tmp as libc::c_int;
}
unsafe extern "C" fn ipc_write_rc(
    mut sockfd: libc::c_int,
    mut pid: pid_t,
    mut type_0: uint16_t,
    mut rc: libc::c_int,
) -> libc::c_int {
    let mut resplen: libc::c_int = 0;
    let mut response: *mut ipc_msg = 0 as *mut ipc_msg;
    let mut tmp = 0 as *mut _;
    let mut err: ipc_err = ipc_err { rc: 0, err: 0, data: [] };
    let mut tmp___0: libc::c_int = 0;
    resplen = (::std::mem::size_of::<ipc_msg>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ipc_err>() as libc::c_ulong) as libc::c_int;
    let mut fresh0 = ::std::vec::from_elem(0, resplen as libc::c_ulong as usize);
    tmp = fresh0.as_mut_ptr();
    response = tmp as *mut ipc_msg;
    if response as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"Could not allocate memory for IPC write response\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*response).type_0 = type_0;
    (*response).pid = pid;
    if rc < 0 as libc::c_int {
        err.err = -rc;
        err.rc = -(1 as libc::c_int);
    } else {
        err.err = 0 as libc::c_int;
        err.rc = rc;
    }
    memcpy(
        ((*response).data).as_mut_ptr() as *mut libc::c_void,
        &mut err as *mut ipc_err as *const libc::c_void,
        ::std::mem::size_of::<ipc_err>() as libc::c_ulong,
    );
    tmp___0 = ipc_try_send(
        sockfd,
        response as *mut libc::c_char as *const libc::c_void,
        resplen as size_t,
    );
    if tmp___0 == -(1 as libc::c_int) {
        perror(
            b"Error on writing IPC write response\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ipc_read(
    mut sockfd: libc::c_int,
    mut msg: *mut ipc_msg,
) -> libc::c_int {
    let mut requested: *mut ipc_read = 0 as *mut ipc_read;
    let mut pid: pid_t = 0;
    let mut rlen: libc::c_int = 0;
    let mut resplen: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut response: *mut ipc_msg = 0 as *mut ipc_msg;
    let mut tmp___0 = 0 as *mut _;
    let mut error: *mut ipc_err = 0 as *mut ipc_err;
    let mut actual: *mut ipc_read = 0 as *mut ipc_read;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    requested = ((*msg).data).as_mut_ptr() as *mut ipc_read;
    pid = (*msg).pid;
    rlen = -(1 as libc::c_int);
    let vla = (*requested).len as usize;
    let mut rbuf: Vec::<libc::c_char> = ::std::vec::from_elem(0, vla);
    memset(rbuf.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int, (*requested).len);
    rlen = _read(
        pid,
        (*requested).sockfd,
        rbuf.as_mut_ptr() as *mut libc::c_void,
        (*requested).len as libc::c_uint,
    );
    if rlen > 0 as libc::c_int {
        tmp = rlen;
    } else {
        tmp = 0 as libc::c_int;
    }
    resplen = (::std::mem::size_of::<ipc_msg>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ipc_err>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ipc_read>() as libc::c_ulong)
        .wrapping_add(tmp as libc::c_ulong) as libc::c_int;
    let mut fresh1 = ::std::vec::from_elem(0, resplen as libc::c_ulong as usize);
    tmp___0 = fresh1.as_mut_ptr();
    response = tmp___0 as *mut ipc_msg;
    error = ((*response).data).as_mut_ptr() as *mut ipc_err;
    actual = ((*error).data).as_mut_ptr() as *mut ipc_read;
    if response as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"Could not allocate memory for IPC read response\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*response).type_0 = 4 as libc::c_int as uint16_t;
    (*response).pid = pid;
    if rlen < 0 as libc::c_int {
        (*error).rc = -(1 as libc::c_int);
    } else {
        (*error).rc = rlen;
    }
    if rlen < 0 as libc::c_int {
        (*error).err = -rlen;
    } else {
        (*error).err = 0 as libc::c_int;
    }
    (*actual).sockfd = (*requested).sockfd;
    (*actual).len = rlen as size_t;
    if rlen > 0 as libc::c_int {
        tmp___1 = rlen;
    } else {
        tmp___1 = 0 as libc::c_int;
    }
    memcpy(
        ((*actual).buf).as_mut_ptr() as *mut libc::c_void,
        rbuf.as_mut_ptr() as *const libc::c_void,
        tmp___1 as size_t,
    );
    tmp___2 = ipc_try_send(
        sockfd,
        response as *mut libc::c_char as *const libc::c_void,
        resplen as size_t,
    );
    if tmp___2 == -(1 as libc::c_int) {
        perror(
            b"Error on writing IPC read response\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ipc_write(
    mut sockfd: libc::c_int,
    mut msg: *mut ipc_msg,
) -> libc::c_int {
    let mut payload: *mut ipc_write = 0 as *mut ipc_write;
    let mut pid: pid_t = 0;
    let mut rc: libc::c_int = 0;
    let mut head: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut tail: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut tmp___0: ssize_t = 0;
    let mut tmp___1: libc::c_int = 0;
    payload = ((*msg).data).as_mut_ptr() as *mut ipc_write;
    pid = (*msg).pid;
    rc = -(1 as libc::c_int);
    head = (8192 as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<ipc_write>() as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<ipc_msg>() as libc::c_ulong) as libc::c_int;
    let vla = (*payload).len as usize;
    let mut buf: Vec::<libc::c_char> = ::std::vec::from_elem(0, vla);
    memset(buf.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int, (*payload).len);
    if (*payload).len > head as size_t {
        tmp = head as size_t;
    } else {
        tmp = (*payload).len;
    }
    memcpy(
        buf.as_mut_ptr() as *mut libc::c_void,
        ((*payload).buf).as_mut_ptr() as *const libc::c_void,
        tmp,
    );
    if (*payload).len > head as size_t {
        tail = ((*payload).len).wrapping_sub(head as size_t) as libc::c_int;
        tmp___0 = read(
            sockfd,
            &mut *buf.as_mut_ptr().offset(head as isize) as *mut libc::c_char
                as *mut libc::c_void,
            tail as size_t,
        );
        res = tmp___0 as libc::c_int;
        if res == -(1 as libc::c_int) {
            perror(b"Read on IPC payload guard\0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int);
        } else {
            if res != tail {
                fprintf(
                    stderr,
                    b"Hmm, we did not read exact payload amount in IPC write\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    rc = _write(
        pid,
        (*payload).sockfd,
        buf.as_mut_ptr() as *const libc::c_void,
        (*payload).len as libc::c_uint,
    );
    tmp___1 = ipc_write_rc(sockfd, pid, 3 as libc::c_int as uint16_t, rc);
    return tmp___1;
}
unsafe extern "C" fn ipc_connect(
    mut sockfd: libc::c_int,
    mut msg: *mut ipc_msg,
) -> libc::c_int {
    let mut payload: *mut ipc_connect = 0 as *mut ipc_connect;
    let mut pid: pid_t = 0;
    let mut rc: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    payload = ((*msg).data).as_mut_ptr() as *mut ipc_connect;
    pid = (*msg).pid;
    rc = -(1 as libc::c_int);
    rc = _connect(
        pid,
        (*payload).sockfd,
        &mut (*payload).addr as *mut sockaddr as *const sockaddr,
        (*payload).addrlen,
    );
    tmp = ipc_write_rc(sockfd, pid, 2 as libc::c_int as uint16_t, rc);
    return tmp;
}
unsafe extern "C" fn ipc_socket(
    mut sockfd: libc::c_int,
    mut msg: *mut ipc_msg,
) -> libc::c_int {
    let mut sock: *mut ipc_socket = 0 as *mut ipc_socket;
    let mut pid: pid_t = 0;
    let mut rc: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    sock = ((*msg).data).as_mut_ptr() as *mut ipc_socket;
    pid = (*msg).pid;
    rc = -(1 as libc::c_int);
    rc = _socket(pid, (*sock).domain, (*sock).type_0, (*sock).protocol);
    tmp = ipc_write_rc(sockfd, pid, 1 as libc::c_int as uint16_t, rc);
    return tmp;
}
unsafe extern "C" fn ipc_close(
    mut sockfd: libc::c_int,
    mut msg: *mut ipc_msg,
) -> libc::c_int {
    let mut payload: *mut ipc_close = 0 as *mut ipc_close;
    let mut pid: pid_t = 0;
    let mut rc: libc::c_int = 0;
    payload = ((*msg).data).as_mut_ptr() as *mut ipc_close;
    pid = (*msg).pid;
    rc = -(1 as libc::c_int);
    rc = _close(pid, (*payload).sockfd);
    rc = ipc_write_rc(sockfd, pid, 5 as libc::c_int as uint16_t, rc);
    return rc;
}
unsafe extern "C" fn ipc_poll(
    mut sockfd: libc::c_int,
    mut msg: *mut ipc_msg,
) -> libc::c_int {
    let mut data: *mut ipc_poll = 0 as *mut ipc_poll;
    let mut pid: pid_t = 0;
    let mut rc: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut resplen: libc::c_int = 0;
    let mut response: *mut ipc_msg = 0 as *mut ipc_msg;
    let mut tmp = 0 as *mut _;
    let mut err: ipc_err = ipc_err { rc: 0, err: 0, data: [] };
    let mut polled: *mut ipc_pollfd = 0 as *mut ipc_pollfd;
    let mut i___0: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    data = ((*msg).data).as_mut_ptr() as *mut ipc_poll;
    pid = (*msg).pid;
    rc = -(1 as libc::c_int);
    let vla = (*data).nfds as usize;
    let mut fds: Vec::<pollfd> = ::std::vec::from_elem(
        pollfd {
            fd: 0,
            events: 0,
            revents: 0,
        },
        vla,
    );
    i = 0 as libc::c_int;
    while (i as nfds_t) < (*data).nfds {
        (*fds.as_mut_ptr().offset(i as isize))
            .fd = (*((*data).fds).as_mut_ptr().offset(i as isize)).fd;
        (*fds.as_mut_ptr().offset(i as isize))
            .events = (*((*data).fds).as_mut_ptr().offset(i as isize)).events;
        (*fds.as_mut_ptr().offset(i as isize))
            .revents = (*((*data).fds).as_mut_ptr().offset(i as isize)).revents;
        i += 1;
    }
    rc = _poll(pid, fds.as_mut_ptr(), (*data).nfds, (*data).timeout);
    resplen = (::std::mem::size_of::<ipc_msg>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ipc_err>() as libc::c_ulong)
        .wrapping_add(
            (::std::mem::size_of::<ipc_pollfd>() as libc::c_ulong)
                .wrapping_mul((*data).nfds),
        ) as libc::c_int;
    let mut fresh2 = ::std::vec::from_elem(0, resplen as libc::c_ulong as usize);
    tmp = fresh2.as_mut_ptr();
    response = tmp as *mut ipc_msg;
    if response as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"Could not allocate memory for IPC write response\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*response).type_0 = 6 as libc::c_int as uint16_t;
    (*response).pid = pid;
    if rc < 0 as libc::c_int {
        err.err = -rc;
        err.rc = -(1 as libc::c_int);
    } else {
        err.err = 0 as libc::c_int;
        err.rc = rc;
    }
    memcpy(
        ((*response).data).as_mut_ptr() as *mut libc::c_void,
        &mut err as *mut ipc_err as *const libc::c_void,
        ::std::mem::size_of::<ipc_err>() as libc::c_ulong,
    );
    polled = ((*(((*response).data).as_mut_ptr() as *mut ipc_err)).data).as_mut_ptr()
        as *mut ipc_pollfd;
    i___0 = 0 as libc::c_int;
    while (i___0 as nfds_t) < (*data).nfds {
        (*polled.offset(i___0 as isize))
            .fd = (*fds.as_mut_ptr().offset(i___0 as isize)).fd;
        (*polled.offset(i___0 as isize))
            .events = (*fds.as_mut_ptr().offset(i___0 as isize)).events;
        (*polled.offset(i___0 as isize))
            .revents = (*fds.as_mut_ptr().offset(i___0 as isize)).revents;
        i___0 += 1;
    }
    tmp___0 = ipc_try_send(
        sockfd,
        response as *mut libc::c_char as *const libc::c_void,
        resplen as size_t,
    );
    if tmp___0 == -(1 as libc::c_int) {
        perror(
            b"Error on writing IPC poll response\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ipc_fcntl(
    mut sockfd: libc::c_int,
    mut msg: *mut ipc_msg,
) -> libc::c_int {
    let mut fc: *mut ipc_fcntl = 0 as *mut ipc_fcntl;
    let mut pid: pid_t = 0;
    let mut rc: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    fc = ((*msg).data).as_mut_ptr() as *mut ipc_fcntl;
    pid = (*msg).pid;
    rc = -(1 as libc::c_int);
    match (*fc).cmd {
        3 => {
            rc = _fcntl(pid, (*fc).sockfd, (*fc).cmd);
        }
        4 => {
            rc = _fcntl(
                pid,
                (*fc).sockfd,
                (*fc).cmd,
                *(((*fc).data).as_mut_ptr() as *mut libc::c_int),
            );
        }
        _ => {
            fprintf(
                stderr,
                b"IPC Fcntl cmd not supported %d\n\0" as *const u8
                    as *const libc::c_char,
                (*fc).cmd,
            );
            rc = -(22 as libc::c_int);
        }
    }
    tmp = ipc_write_rc(sockfd, pid, 7 as libc::c_int as uint16_t, rc);
    return tmp;
}
unsafe extern "C" fn ipc_getsockopt(
    mut sockfd: libc::c_int,
    mut msg: *mut ipc_msg,
) -> libc::c_int {
    let mut opts: *mut ipc_sockopt = 0 as *mut ipc_sockopt;
    let mut pid: pid_t = 0;
    let mut rc: libc::c_int = 0;
    let mut resplen: libc::c_int = 0;
    let mut response: *mut ipc_msg = 0 as *mut ipc_msg;
    let mut tmp = 0 as *mut _;
    let mut err: ipc_err = ipc_err { rc: 0, err: 0, data: [] };
    let mut optres: *mut ipc_sockopt = 0 as *mut ipc_sockopt;
    let mut tmp___0: libc::c_int = 0;
    opts = ((*msg).data).as_mut_ptr() as *mut ipc_sockopt;
    pid = (*msg).pid;
    rc = -(1 as libc::c_int);
    rc = _getsockopt(
        pid,
        (*opts).fd,
        (*opts).level,
        (*opts).optname,
        ((*opts).optval).as_mut_ptr() as *mut libc::c_void,
        &mut (*opts).optlen,
    );
    resplen = (::std::mem::size_of::<ipc_msg>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ipc_err>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ipc_sockopt>() as libc::c_ulong)
        .wrapping_add((*opts).optlen as libc::c_ulong) as libc::c_int;
    let mut fresh3 = ::std::vec::from_elem(0, resplen as libc::c_ulong as usize);
    tmp = fresh3.as_mut_ptr();
    response = tmp as *mut ipc_msg;
    if response as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"Could not allocate memory for IPC getsockopt response\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*response).type_0 = 8 as libc::c_int as uint16_t;
    (*response).pid = pid;
    if rc < 0 as libc::c_int {
        err.err = -rc;
        err.rc = -(1 as libc::c_int);
    } else {
        err.err = 0 as libc::c_int;
        err.rc = rc;
    }
    memcpy(
        ((*response).data).as_mut_ptr() as *mut libc::c_void,
        &mut err as *mut ipc_err as *const libc::c_void,
        ::std::mem::size_of::<ipc_err>() as libc::c_ulong,
    );
    optres = ((*(((*response).data).as_mut_ptr() as *mut ipc_err)).data).as_mut_ptr()
        as *mut ipc_sockopt;
    (*optres).fd = (*opts).fd;
    (*optres).level = (*opts).level;
    (*optres).optname = (*opts).optname;
    (*optres).optlen = (*opts).optlen;
    memcpy(
        &mut (*optres).optval as *mut [uint8_t; 0] as *mut libc::c_void,
        ((*opts).optval).as_mut_ptr() as *const libc::c_void,
        (*opts).optlen as size_t,
    );
    tmp___0 = ipc_try_send(
        sockfd,
        response as *mut libc::c_char as *const libc::c_void,
        resplen as size_t,
    );
    if tmp___0 == -(1 as libc::c_int) {
        perror(
            b"Error on writing IPC getsockopt response\0" as *const u8
                as *const libc::c_char,
        );
    }
    return rc;
}
unsafe extern "C" fn ipc_getpeername(
    mut sockfd: libc::c_int,
    mut msg: *mut ipc_msg,
) -> libc::c_int {
    let mut name: *mut ipc_sockname = 0 as *mut ipc_sockname;
    let mut pid: pid_t = 0;
    let mut rc: libc::c_int = 0;
    let mut resplen: libc::c_int = 0;
    let mut response: *mut ipc_msg = 0 as *mut ipc_msg;
    let mut tmp = 0 as *mut _;
    let mut nameres: *mut ipc_sockname = 0 as *mut ipc_sockname;
    let mut err: ipc_err = ipc_err { rc: 0, err: 0, data: [] };
    let mut tmp___0: libc::c_int = 0;
    name = ((*msg).data).as_mut_ptr() as *mut ipc_sockname;
    pid = (*msg).pid;
    rc = -(1 as libc::c_int);
    resplen = (::std::mem::size_of::<ipc_msg>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ipc_err>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ipc_sockname>() as libc::c_ulong)
        as libc::c_int;
    let mut fresh4 = ::std::vec::from_elem(0, resplen as libc::c_ulong as usize);
    tmp = fresh4.as_mut_ptr();
    response = tmp as *mut ipc_msg;
    if response as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"Could not allocate memory for IPC getpeername response\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*response).type_0 = 10 as libc::c_int as uint16_t;
    (*response).pid = pid;
    nameres = ((*(((*response).data).as_mut_ptr() as *mut ipc_err)).data).as_mut_ptr()
        as *mut ipc_sockname;
    rc = _getpeername(
        pid,
        (*name).socket,
        ((*nameres).sa_data).as_mut_ptr() as *mut sockaddr,
        &mut (*nameres).address_len as *mut socklen_t,
    );
    if rc < 0 as libc::c_int {
        err.err = -rc;
        err.rc = -(1 as libc::c_int);
    } else {
        err.err = 0 as libc::c_int;
        err.rc = rc;
    }
    memcpy(
        ((*response).data).as_mut_ptr() as *mut libc::c_void,
        &mut err as *mut ipc_err as *const libc::c_void,
        ::std::mem::size_of::<ipc_err>() as libc::c_ulong,
    );
    (*nameres).socket = (*name).socket;
    tmp___0 = ipc_try_send(
        sockfd,
        response as *mut libc::c_char as *const libc::c_void,
        resplen as size_t,
    );
    if tmp___0 == -(1 as libc::c_int) {
        perror(
            b"Error on writing IPC getpeername response\0" as *const u8
                as *const libc::c_char,
        );
    }
    return rc;
}
unsafe extern "C" fn ipc_getsockname(
    mut sockfd: libc::c_int,
    mut msg: *mut ipc_msg,
) -> libc::c_int {
    let mut name: *mut ipc_sockname = 0 as *mut ipc_sockname;
    let mut pid: pid_t = 0;
    let mut rc: libc::c_int = 0;
    let mut resplen: libc::c_int = 0;
    let mut response: *mut ipc_msg = 0 as *mut ipc_msg;
    let mut tmp = 0 as *mut _;
    let mut nameres: *mut ipc_sockname = 0 as *mut ipc_sockname;
    let mut err: ipc_err = ipc_err { rc: 0, err: 0, data: [] };
    let mut tmp___0: libc::c_int = 0;
    name = ((*msg).data).as_mut_ptr() as *mut ipc_sockname;
    pid = (*msg).pid;
    rc = -(1 as libc::c_int);
    resplen = (::std::mem::size_of::<ipc_msg>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ipc_err>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ipc_sockname>() as libc::c_ulong)
        as libc::c_int;
    let mut fresh5 = ::std::vec::from_elem(0, resplen as libc::c_ulong as usize);
    tmp = fresh5.as_mut_ptr();
    response = tmp as *mut ipc_msg;
    if response as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"Could not allocate memory for IPC getsockname response\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*response).type_0 = 11 as libc::c_int as uint16_t;
    (*response).pid = pid;
    nameres = ((*(((*response).data).as_mut_ptr() as *mut ipc_err)).data).as_mut_ptr()
        as *mut ipc_sockname;
    rc = _getsockname(
        pid,
        (*name).socket,
        ((*nameres).sa_data).as_mut_ptr() as *mut sockaddr,
        &mut (*nameres).address_len as *mut socklen_t,
    );
    if rc < 0 as libc::c_int {
        err.err = -rc;
        err.rc = -(1 as libc::c_int);
    } else {
        err.err = 0 as libc::c_int;
        err.rc = rc;
    }
    memcpy(
        ((*response).data).as_mut_ptr() as *mut libc::c_void,
        &mut err as *mut ipc_err as *const libc::c_void,
        ::std::mem::size_of::<ipc_err>() as libc::c_ulong,
    );
    (*nameres).socket = (*name).socket;
    tmp___0 = ipc_try_send(
        sockfd,
        response as *mut libc::c_char as *const libc::c_void,
        resplen as size_t,
    );
    if tmp___0 == -(1 as libc::c_int) {
        perror(
            b"Error on writing IPC getsockname response\0" as *const u8
                as *const libc::c_char,
        );
    }
    return rc;
}
unsafe extern "C" fn demux_ipc_socket_call(
    mut sockfd: libc::c_int,
    mut cmdbuf: *mut libc::c_char,
    mut blen: libc::c_int,
) -> libc::c_int {
    let mut msg: *mut ipc_msg = 0 as *mut ipc_msg;
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
    msg = cmdbuf as *mut ipc_msg;
    match (*msg).type_0 as libc::c_int {
        1 => {
            tmp = ipc_socket(sockfd, msg);
            return tmp;
        }
        2 => {
            tmp___0 = ipc_connect(sockfd, msg);
            return tmp___0;
        }
        3 => {
            tmp___1 = ipc_write(sockfd, msg);
            return tmp___1;
        }
        4 => {
            tmp___2 = ipc_read(sockfd, msg);
            return tmp___2;
        }
        5 => {
            tmp___3 = ipc_close(sockfd, msg);
            return tmp___3;
        }
        6 => {
            tmp___4 = ipc_poll(sockfd, msg);
            return tmp___4;
        }
        7 => {
            tmp___5 = ipc_fcntl(sockfd, msg);
            return tmp___5;
        }
        8 => {
            tmp___6 = ipc_getsockopt(sockfd, msg);
            return tmp___6;
        }
        10 => {
            tmp___7 = ipc_getpeername(sockfd, msg);
            return tmp___7;
        }
        11 => {
            tmp___8 = ipc_getsockname(sockfd, msg);
            return tmp___8;
        }
        _ => {
            fprintf(
                stderr,
                b"No such IPC type %d\n\0" as *const u8 as *const libc::c_char,
                (*msg).type_0 as libc::c_int,
            );
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn socket_ipc_open(
    mut args: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut blen: libc::c_int = 0;
    let mut sockfd: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut tmp: ssize_t = 0;
    blen = 8192 as libc::c_int;
    let vla = blen as usize;
    let mut buf: Vec::<libc::c_char> = ::std::vec::from_elem(0, vla);
    sockfd = *(args as *mut libc::c_int);
    rc = -(1 as libc::c_int);
    loop {
        tmp = read(sockfd, buf.as_mut_ptr() as *mut libc::c_void, blen as size_t);
        rc = tmp as libc::c_int;
        if !(rc > 0 as libc::c_int) {
            break;
        }
        rc = demux_ipc_socket_call(sockfd, buf.as_mut_ptr(), blen);
        if rc == -(1 as libc::c_int) {
            fprintf(
                stderr,
                b"Error on demuxing IPC socket call\n\0" as *const u8
                    as *const libc::c_char,
            );
            close(sockfd);
            return 0 as *mut libc::c_void;
        }
    }
    ipc_free_thread(sockfd);
    if rc == -(1 as libc::c_int) {
        perror(b"socket ipc read\0" as *const u8 as *const libc::c_char);
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn start_ipc_listener() -> *mut libc::c_void {
    let mut fd___0: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut datasock: libc::c_int = 0;
    let mut un: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut sockname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: size_t = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut th: *mut ipc_thread = 0 as *mut ipc_thread;
    let mut tmp___1: *mut ipc_thread = 0 as *mut ipc_thread;
    let mut tmp___2: libc::c_int = 0;
    sockname = b"/tmp/lvlip.socket\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    unlink(sockname as *const libc::c_char);
    tmp = strnlen(
        sockname as *const libc::c_char,
        ::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong,
    );
    if tmp == ::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong {
        fprintf(
            stderr,
            b"Path for UNIX socket is too long\n\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    fd___0 = socket(1 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int);
    if fd___0 < 0 as libc::c_int {
        perror(b"IPC listener UNIX socket\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    memset(
        &mut un as *mut sockaddr_un as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong,
    );
    un.sun_family = 1 as libc::c_int as sa_family_t;
    strncpy(
        (un.sun_path).as_mut_ptr(),
        sockname as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_ulong),
    );
    rc = bind(
        fd___0,
        &mut un as *mut sockaddr_un as *const sockaddr,
        ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong as socklen_t,
    );
    if rc == -(1 as libc::c_int) {
        perror(b"IPC bind\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    rc = listen(fd___0, 20 as libc::c_int);
    if rc == -(1 as libc::c_int) {
        perror(b"IPC listen\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    tmp___0 = chmod(
        sockname as *const libc::c_char,
        (448 as libc::c_int | 256 as libc::c_int >> 3 as libc::c_int
            | 128 as libc::c_int >> 3 as libc::c_int
            | 64 as libc::c_int >> 3 as libc::c_int
            | 256 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            | 128 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            | 64 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as __mode_t,
    );
    if tmp___0 == -(1 as libc::c_int) {
        perror(
            b"Chmod on lvl-ip IPC UNIX socket failed\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    loop {
        datasock = accept(
            fd___0,
            0 as *mut libc::c_void as *mut sockaddr,
            0 as *mut libc::c_void as *mut socklen_t,
        );
        if datasock == -(1 as libc::c_int) {
            perror(b"IPC accept\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        tmp___1 = ipc_alloc_thread(datasock);
        th = tmp___1;
        tmp___2 = pthread_create(
            &mut (*th).id as *mut pthread_t,
            0 as *mut libc::c_void as *const pthread_attr_t,
            Some(
                socket_ipc_open
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            &mut (*th).sock as *mut libc::c_int as *mut libc::c_void,
        );
        if tmp___2 != 0 as libc::c_int {
            fprintf(
                stderr,
                b"Error on socket thread creation\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    };
}
pub unsafe extern "C" fn ip_send_check(mut ihdr: *mut iphdr) {
    let mut csum: uint32_t = 0;
    let mut tmp: uint16_t = 0;
    tmp = checksum(
        ihdr as *mut libc::c_void,
        (*ihdr).ihl() as libc::c_int * 4 as libc::c_int,
        0 as libc::c_int,
    );
    csum = tmp as uint32_t;
    (*ihdr).csum = csum as uint16_t;
}
pub unsafe extern "C" fn ip_output(
    mut sk: *mut sock,
    mut skb: *mut sk_buff,
) -> libc::c_int {
    let mut rt: *mut rtentry = 0 as *mut rtentry;
    let mut ihdr: *mut iphdr = 0 as *mut iphdr;
    let mut tmp: *mut iphdr = 0 as *mut iphdr;
    let mut tmp___0: libc::c_int = 0;
    tmp = ip_hdr(skb as *const sk_buff);
    ihdr = tmp;
    rt = route_lookup((*sk).daddr);
    if rt.is_null() {
        fprintf(
            stderr,
            b"IP output route lookup fail\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*skb).dev = (*rt).dev;
    (*skb).rt = rt;
    skb_push(skb, ::std::mem::size_of::<iphdr>() as libc::c_ulong as libc::c_uint);
    (*ihdr).set_version(4 as libc::c_int as uint8_t);
    (*ihdr).set_ihl(5 as libc::c_int as uint8_t);
    (*ihdr).tos = 0 as libc::c_int as uint8_t;
    (*ihdr).len = (*skb).len as uint16_t;
    (*ihdr).id = (*ihdr).id;
    (*ihdr).frag_offset = 16384 as libc::c_int as uint16_t;
    (*ihdr).ttl = 64 as libc::c_int as uint8_t;
    (*ihdr).proto = (*skb).protocol as uint8_t;
    (*ihdr).saddr = (*(*skb).dev).addr;
    (*ihdr).daddr = (*sk).daddr;
    (*ihdr).csum = 0 as libc::c_int as uint16_t;
    (*ihdr).len = htons((*ihdr).len);
    (*ihdr).id = htons((*ihdr).id);
    (*ihdr).daddr = htonl((*ihdr).daddr);
    (*ihdr).saddr = htonl((*ihdr).saddr);
    (*ihdr).csum = htons((*ihdr).csum);
    (*ihdr).frag_offset = htons((*ihdr).frag_offset);
    ip_send_check(ihdr);
    tmp___0 = dst_neigh_output(skb);
    return tmp___0;
}
static mut threads: [pthread_t; 4] = [0; 4];
pub static mut running: libc::c_int = 1 as libc::c_int;
pub static mut mask: sigset_t = sigset_t { __val: [0; 16] };
unsafe extern "C" fn create_thread(
    mut id: pthread_t,
    mut func: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
) {
    let mut tmp: libc::c_int = 0;
    tmp = pthread_create(
        &mut *threads.as_mut_ptr().offset(id as isize) as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        func,
        0 as *mut libc::c_void,
    );
    if tmp != 0 as libc::c_int {
        fprintf(
            stderr,
            b"Could not create core thread\n\0" as *const u8 as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn stop_stack_handler(
    mut arg: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut err: libc::c_int = 0;
    let mut signo: libc::c_int = 0;
    loop {
        err = sigwait(
            &mut mask as *mut sigset_t as *const sigset_t,
            &mut signo as *mut libc::c_int,
        );
        if err != 0 as libc::c_int {
            fprintf(
                stderr,
                b"Sigwait failed: %d\n\0" as *const u8 as *const libc::c_char,
                err,
            );
        }
        match signo {
            3 | 2 => {
                running = 0 as libc::c_int;
                pthread_cancel(threads[2 as libc::c_int as usize]);
                pthread_cancel(threads[0 as libc::c_int as usize]);
                pthread_cancel(threads[1 as libc::c_int as usize]);
                return 0 as *mut libc::c_void;
            }
            _ => {
                printf(
                    b"Unexpected signal %d\n\0" as *const u8 as *const libc::c_char,
                    signo,
                );
            }
        }
    };
}
unsafe extern "C" fn init_signals() {
    let mut err: libc::c_int = 0;
    sigemptyset(&mut mask);
    sigaddset(&mut mask, 2 as libc::c_int);
    sigaddset(&mut mask, 3 as libc::c_int);
    err = pthread_sigmask(
        0 as libc::c_int,
        &mut mask as *mut sigset_t as *const __sigset_t,
        0 as *mut libc::c_void as *mut __sigset_t,
    );
    if err != 0 as libc::c_int {
        fprintf(stderr, b"SIG_BLOCK error\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
}
unsafe extern "C" fn init_stack() {
    tun_init();
    netdev_init(0 as *mut libc::c_char, 0 as *mut libc::c_char);
    route_init();
    arp_init();
    tcp_init();
}
unsafe extern "C" fn run_threads() {
    create_thread(
        0 as libc::c_int as pthread_t,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        >(Some(netdev_rx_loop as unsafe extern "C" fn() -> *mut libc::c_void)),
    );
    create_thread(
        1 as libc::c_int as pthread_t,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        >(Some(timers_start as unsafe extern "C" fn() -> *mut libc::c_void)),
    );
    create_thread(
        2 as libc::c_int as pthread_t,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        >(Some(start_ipc_listener as unsafe extern "C" fn() -> *mut libc::c_void)),
    );
    create_thread(
        3 as libc::c_int as pthread_t,
        Some(
            stop_stack_handler
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
    );
}
unsafe extern "C" fn wait_for_threads() {
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        tmp = pthread_join(
            threads[i as usize],
            0 as *mut libc::c_void as *mut *mut libc::c_void,
        );
        if tmp != 0 as libc::c_int {
            fprintf(
                stderr,
                b"Error when joining threads\n\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        i += 1;
    }
}
pub unsafe extern "C" fn free_stack() {
    abort_sockets();
    free_arp();
    free_routes();
    free_netdev();
    free_tun();
}
pub unsafe extern "C" fn init_security() {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = prctl(24 as libc::c_int, 12 as libc::c_int);
    if tmp == -(1 as libc::c_int) {
        perror(
            b"Error on network admin capability drop\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    tmp___0 = prctl(24 as libc::c_int, 8 as libc::c_int);
    if tmp___0 == -(1 as libc::c_int) {
        perror(b"Error on capability set drop\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    parse_cli(argc, argv);
    init_signals();
    init_stack();
    init_security();
    run_threads();
    wait_for_threads();
    free_stack();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dst_neigh_output(mut skb: *mut sk_buff) -> libc::c_int {
    let mut iphdr: *mut iphdr = 0 as *mut iphdr;
    let mut tmp: *mut iphdr = 0 as *mut iphdr;
    let mut netdev___0: *mut netdev = 0 as *mut netdev;
    let mut rt: *mut rtentry = 0 as *mut rtentry;
    let mut daddr: uint32_t = 0;
    let mut tmp___0: uint32_t = 0;
    let mut saddr: uint32_t = 0;
    let mut tmp___1: uint32_t = 0;
    let mut dmac: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___2: libc::c_int = 0;
    tmp = ip_hdr(skb as *const sk_buff);
    iphdr = tmp;
    netdev___0 = (*skb).dev;
    rt = (*skb).rt;
    tmp___0 = ntohl((*iphdr).daddr);
    daddr = tmp___0;
    tmp___1 = ntohl((*iphdr).saddr);
    saddr = tmp___1;
    if (*rt).flags as libc::c_int & 2 as libc::c_int != 0 {
        daddr = (*rt).gateway;
    }
    dmac = arp_get_hwaddr(daddr);
    if !dmac.is_null() {
        tmp___2 = netdev_transmit(skb, dmac, 2048 as libc::c_int as uint16_t);
        return tmp___2;
    } else {
        arp_request(saddr, daddr, netdev___0);
        return -(1 as libc::c_int);
    };
}
unsafe extern "C" fn tcp_parse_opts(
    mut tsk: *mut tcp_sock,
    mut th: *mut tcphdr,
) -> libc::c_int {
    let mut ptr: *mut uint8_t = 0 as *mut uint8_t;
    let mut optlen: uint8_t = 0;
    let mut opt_mss: *mut tcp_opt_mss = 0 as *mut tcp_opt_mss;
    let mut sack_seen: uint8_t = 0;
    let mut tsopt_seen: uint8_t = 0;
    let mut mss: uint16_t = 0;
    let mut tmp: uint16_t = 0;
    ptr = ((*th).data).as_mut_ptr();
    optlen = ((((*th).hl() as libc::c_int) << 2 as libc::c_int) - 20 as libc::c_int)
        as uint8_t;
    opt_mss = 0 as *mut libc::c_void as *mut tcp_opt_mss;
    sack_seen = 0 as libc::c_int as uint8_t;
    tsopt_seen = 0 as libc::c_int as uint8_t;
    while optlen as libc::c_int > 0 as libc::c_int {
        if !((optlen as libc::c_int) < 20 as libc::c_int) {
            break;
        }
        match *ptr as libc::c_int {
            2 => {
                opt_mss = ptr as *mut tcp_opt_mss;
                tmp = ntohs((*opt_mss).mss);
                mss = tmp;
                if mss as libc::c_int > 536 as libc::c_int {
                    if mss as libc::c_int <= 1460 as libc::c_int {
                        (*tsk).smss = mss;
                    }
                }
                ptr = ptr
                    .offset(
                        ::std::mem::size_of::<tcp_opt_mss>() as libc::c_ulong as isize,
                    );
                optlen = (optlen as libc::c_int - 4 as libc::c_int) as uint8_t;
            }
            1 => {
                ptr = ptr.offset(1);
                optlen = (optlen as libc::c_int - 1 as libc::c_int) as uint8_t;
            }
            4 => {
                sack_seen = 1 as libc::c_int as uint8_t;
                optlen = (optlen as libc::c_int - 1 as libc::c_int) as uint8_t;
            }
            8 => {
                tsopt_seen = 1 as libc::c_int as uint8_t;
                optlen = (optlen as libc::c_int - 1 as libc::c_int) as uint8_t;
            }
            _ => {
                fprintf(
                    stderr,
                    b"Unrecognized TCPOPT\n\0" as *const u8 as *const libc::c_char,
                );
                optlen = (optlen as libc::c_int - 1 as libc::c_int) as uint8_t;
            }
        }
    }
    if tsopt_seen == 0 {
        (*tsk).tsopt = 0 as libc::c_int as uint8_t;
    }
    if sack_seen != 0 {
        if (*tsk).sackok != 0 {
            if (*tsk).tsopt != 0 {
                (*tsk).sacks_allowed = 3 as libc::c_int as uint8_t;
            } else {
                (*tsk).sacks_allowed = 4 as libc::c_int as uint8_t;
            }
        } else {
            (*tsk).sackok = 0 as libc::c_int as uint8_t;
        }
    } else {
        (*tsk).sackok = 0 as libc::c_int as uint8_t;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn tcp_clean_rto_queue(
    mut sk: *mut sock,
    mut una: uint32_t,
) -> libc::c_int {
    let mut tsk: *mut tcp_sock = 0 as *mut tcp_sock;
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    let mut rc: libc::c_int = 0;
    tsk = sk as *mut tcp_sock;
    rc = 0 as libc::c_int;
    loop {
        skb = skb_peek(&mut (*sk).write_queue);
        if !(skb as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
            break;
        }
        if !((*skb).seq > 0 as libc::c_uint) {
            break;
        }
        if !((*skb).end_seq <= una) {
            break;
        }
        skb_dequeue(&mut (*sk).write_queue);
        (*skb).refcnt -= 1;
        free_skb(skb);
        if (*tsk).inflight > 0 as libc::c_uint {
            (*tsk).inflight = ((*tsk).inflight).wrapping_sub(1);
        }
    }
    if skb as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tcp_stop_rto_timer(tsk);
    } else if (*tsk).inflight == 0 as libc::c_uint {
        tcp_stop_rto_timer(tsk);
    }
    return rc;
}
#[inline]
unsafe extern "C" fn __tcp_drop(
    mut sk: *mut sock,
    mut skb: *mut sk_buff,
) -> libc::c_int {
    free_skb(skb);
    return 0 as libc::c_int;
}
unsafe extern "C" fn tcp_verify_segment(
    mut tsk: *mut tcp_sock,
    mut th: *mut tcphdr,
    mut skb: *mut sk_buff,
) -> libc::c_int {
    let mut tcb: *mut tcb = 0 as *mut tcb;
    tcb = &mut (*tsk).tcb;
    if (*skb).dlen > 0 as libc::c_uint {
        if (*tcb).rcv_wnd == 0 as libc::c_uint {
            return 0 as libc::c_int;
        }
    }
    if (*th).seq < (*tcb).rcv_nxt {
        return 0 as libc::c_int
    } else {
        if (*th).seq > ((*tcb).rcv_nxt).wrapping_add((*tcb).rcv_wnd) {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn tcp_reset(mut sk: *mut sock) {
    (*sk).poll_events = 284 as libc::c_int as libc::c_short;
    match (*sk).state {
        1 => {
            (*sk).err = -(111 as libc::c_int);
        }
        7 => {
            (*sk).err = -(32 as libc::c_int);
        }
        6 => return,
        _ => {
            (*sk).err = -(104 as libc::c_int);
        }
    }
    tcp_done(sk);
}
#[inline]
unsafe extern "C" fn tcp_discard(
    mut tsk: *mut tcp_sock,
    mut skb: *mut sk_buff,
    mut th: *mut tcphdr,
) -> libc::c_int {
    free_skb(skb);
    return 0 as libc::c_int;
}
unsafe extern "C" fn tcp_listen(
    mut tsk: *mut tcp_sock,
    mut skb: *mut sk_buff,
    mut th: *mut tcphdr,
) -> libc::c_int {
    free_skb(skb);
    return 0 as libc::c_int;
}
unsafe extern "C" fn tcp_synsent(
    mut tsk: *mut tcp_sock,
    mut skb: *mut sk_buff,
    mut th: *mut tcphdr,
) -> libc::c_int {
    let mut current_block: u64;
    let mut tcb: *mut tcb = 0 as *mut tcb;
    let mut sk: *mut sock = 0 as *mut sock;
    tcb = &mut (*tsk).tcb;
    sk = &mut (*tsk).sk;
    if (*th).ack() != 0 {
        if (*th).ack_seq <= (*tcb).iss {
            current_block = 18168028710492445723;
        } else if (*th).ack_seq > (*tcb).snd_nxt {
            current_block = 18168028710492445723;
        } else if (*th).ack_seq < (*tcb).snd_una {
            current_block = 15585759012684959852;
        } else if (*th).ack_seq > (*tcb).snd_nxt {
            current_block = 15585759012684959852;
        } else {
            current_block = 17965632435239708295;
        }
        match current_block {
            17965632435239708295 => {}
            _ => {
                match current_block {
                    18168028710492445723 => {
                        if (*th).rst() != 0 {
                            current_block = 7456931364482661208;
                        } else {
                            current_block = 15585759012684959852;
                        }
                    }
                    _ => {}
                }
                match current_block {
                    7456931364482661208 => {}
                    _ => {
                        __tcp_drop(sk, skb);
                        return 0 as libc::c_int;
                    }
                }
            }
        }
    } else {
        current_block = 17965632435239708295;
    }
    match current_block {
        17965632435239708295 => {
            if (*th).rst() != 0 {
                tcp_reset(&mut (*tsk).sk);
            } else if !((*th).syn() == 0) {
                (*tcb).rcv_nxt = ((*th).seq).wrapping_add(1 as libc::c_uint);
                (*tcb).irs = (*th).seq;
                if (*th).ack() != 0 {
                    (*tcb).snd_una = (*th).ack_seq;
                    tcp_clean_rto_queue(sk, (*tcb).snd_una);
                }
                if (*tcb).snd_una > (*tcb).iss {
                    __tcp_set_state(sk, 3 as libc::c_int as uint32_t);
                    (*tcb).snd_una = (*tcb).snd_nxt;
                    (*tsk).backoff = 0 as libc::c_int as uint8_t;
                    (*tsk).rto = 1000 as libc::c_int as uint32_t;
                    tcp_send_ack(&mut (*tsk).sk);
                    tcp_rearm_user_timeout(&mut (*tsk).sk);
                    tcp_parse_opts(tsk, th);
                    sock_connected(sk);
                } else {
                    __tcp_set_state(sk, 2 as libc::c_int as uint32_t);
                    (*tcb).snd_una = (*tcb).iss;
                    tcp_send_synack(&mut (*tsk).sk);
                }
            }
        }
        _ => {}
    }
    __tcp_drop(sk, skb);
    return 0 as libc::c_int;
}
unsafe extern "C" fn tcp_closed(
    mut tsk: *mut tcp_sock,
    mut skb: *mut sk_buff,
    mut th: *mut tcphdr,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    rc = -(1 as libc::c_int);
    if (*th).rst() != 0 {
        tcp_discard(tsk, skb, th);
        rc = 0 as libc::c_int;
    } else {
        rc = tcp_send_reset(tsk);
        free_skb(skb);
    }
    return rc;
}
pub unsafe extern "C" fn tcp_input_state(
    mut sk: *mut sock,
    mut th: *mut tcphdr,
    mut skb: *mut sk_buff,
) -> libc::c_int {
    let mut tsk: *mut tcp_sock = 0 as *mut tcp_sock;
    let mut tcb: *mut tcb = 0 as *mut tcb;
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
    let mut expected: libc::c_int = 0;
    let mut tmp___12: libc::c_int = 0;
    let mut pending: libc::c_int = 0;
    let mut tmp___13: uint32_t = 0;
    let mut tmp___14: uint32_t = 0;
    let mut current_block_129: u64;
    tsk = sk as *mut tcp_sock;
    tcb = &mut (*tsk).tcb;
    match (*sk).state {
        6 => {
            tmp = tcp_closed(tsk, skb, th);
            return tmp;
        }
        0 => {
            tmp___0 = tcp_listen(tsk, skb, th);
            return tmp___0;
        }
        1 => {
            tmp___1 = tcp_synsent(tsk, skb, th);
            return tmp___1;
        }
        _ => {}
    }
    tmp___3 = tcp_verify_segment(tsk, th, skb);
    if tmp___3 == 0 {
        if (*th).rst() == 0 {
            tcp_send_ack(sk);
        }
        tmp___2 = __tcp_drop(sk, skb);
        return tmp___2;
    }
    if (*th).rst() != 0 {
        free_skb(skb);
        tcp_enter_time_wait(sk);
        (Some(((*(*tsk).sk.ops).recv_notify).expect("non-null function pointer")))
            .expect("non-null function pointer")(&mut (*tsk).sk);
        return 0 as libc::c_int;
    }
    if (*th).syn() != 0 {
        tcp_send_challenge_ack(sk, skb);
        tmp___4 = __tcp_drop(sk, skb);
        return tmp___4;
    }
    if (*th).ack() == 0 {
        tmp___5 = __tcp_drop(sk, skb);
        return tmp___5;
    }
    let mut current_block_66: u64;
    match (*sk).state {
        2 => {
            if (*tcb).snd_una <= (*th).ack_seq {
                if (*th).ack_seq < (*tcb).snd_nxt {
                    __tcp_set_state(sk, 3 as libc::c_int as uint32_t);
                } else {
                    tmp___6 = __tcp_drop(sk, skb);
                    return tmp___6;
                }
            } else {
                tmp___6 = __tcp_drop(sk, skb);
                return tmp___6;
            }
            current_block_66 = 13474788557421650121;
        }
        9 | 8 | 7 | 5 | 4 | 3 => {
            current_block_66 = 13474788557421650121;
        }
        _ => {
            current_block_66 = 13303144130133872306;
        }
    }
    match current_block_66 {
        13474788557421650121 => {
            if (*tcb).snd_una < (*th).ack_seq {
                if (*th).ack_seq <= (*tcb).snd_nxt {
                    (*tcb).snd_una = (*th).ack_seq;
                    tcp_rtt(tsk);
                    tcp_clean_rto_queue(sk, (*tcb).snd_una);
                }
            }
            if (*th).ack_seq < (*tcb).snd_una {
                tmp___7 = __tcp_drop(sk, skb);
                return tmp___7;
            }
            if (*th).ack_seq > (*tcb).snd_nxt {
                tmp___8 = __tcp_drop(sk, skb);
                return tmp___8;
            }
            if (*tcb).snd_una < (*th).ack_seq {
                if (*th).ack_seq <= (*tcb).snd_nxt {
                    tmp___9 = 1 as libc::c_int;
                } else {
                    tmp___9 = 0 as libc::c_int;
                }
            } else {
                tmp___9 = 0 as libc::c_int;
            }
        }
        _ => {}
    }
    tmp___11 = skb_queue_empty(
        &mut (*sk).write_queue as *mut sk_buff_head as *const sk_buff_head,
    );
    if tmp___11 != 0 {
        match (*sk).state {
            4 => {
                __tcp_set_state(sk, 5 as libc::c_int as uint32_t);
            }
            8 => {
                __tcp_set_state(sk, 10 as libc::c_int as uint32_t);
            }
            9 => {
                free_skb(skb);
                tmp___10 = tcp_done(sk);
                return tmp___10;
            }
            10 => {
                if (*tcb).rcv_nxt == (*th).seq {
                    (*tsk)
                        .flags = ((*tsk).flags as libc::c_int | 1 as libc::c_int)
                        as uint8_t;
                    tcp_send_ack(sk);
                }
            }
            5 | _ => {}
        }
    }
    expected = ((*skb).seq == (*tcb).rcv_nxt) as libc::c_int;
    match (*sk).state {
        5 | 4 | 3 => {
            if (*th).psh() != 0 {
                tcp_data_queue(tsk, th, skb);
            } else if (*skb).dlen > 0 as libc::c_uint {
                tcp_data_queue(tsk, th, skb);
            }
        }
        10 | 9 | 8 | 7 | _ => {}
    }
    if (*th).fin() != 0 {
        if expected != 0 {
            match (*sk).state {
                1 | 0 | 6 => {
                    __tcp_drop(sk, skb);
                    current_block_129 = 14441615526472673720;
                }
                _ => {
                    (*tcb).rcv_nxt = ((*tcb).rcv_nxt).wrapping_add(1);
                    (*tsk)
                        .flags = ((*tsk).flags as libc::c_int | 1 as libc::c_int)
                        as uint8_t;
                    (*sk)
                        .poll_events = ((*sk).poll_events as libc::c_int
                        | 195 as libc::c_int) as libc::c_short;
                    tcp_send_ack(sk);
                    (Some(
                        ((*(*tsk).sk.ops).recv_notify)
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")(&mut (*tsk).sk);
                    match (*sk).state {
                        3 | 2 => {
                            __tcp_set_state(sk, 7 as libc::c_int as uint32_t);
                        }
                        4 => {
                            tmp___12 = skb_queue_empty(
                                &mut (*sk).write_queue as *mut sk_buff_head
                                    as *const sk_buff_head,
                            );
                            if tmp___12 != 0 {
                                tcp_enter_time_wait(sk);
                            } else {
                                __tcp_set_state(sk, 8 as libc::c_int as uint32_t);
                            }
                        }
                        5 => {
                            tcp_enter_time_wait(sk);
                        }
                        9 | 8 | 7 | 10 | _ => {}
                    }
                    current_block_129 = 4691324637564808323;
                }
            }
        } else {
            current_block_129 = 4691324637564808323;
        }
    } else {
        current_block_129 = 4691324637564808323;
    }
    match current_block_129 {
        4691324637564808323 => {
            match (*sk).state {
                5 | 4 | 3 => {
                    if expected != 0 {
                        tcp_stop_delack_timer(tsk);
                        tmp___13 = skb_queue_len(
                            &mut (*sk).write_queue as *mut sk_buff_head
                                as *const sk_buff_head,
                        );
                        tmp___14 = min(tmp___13, 3 as libc::c_int as uint32_t);
                        pending = tmp___14 as libc::c_int;
                        let mut current_block_122: u64;
                        if (*tsk).inflight == 0 as libc::c_uint {
                            if pending > 0 as libc::c_int {
                                tcp_send_next(sk, pending);
                                (*tsk)
                                    .inflight = ((*tsk).inflight as libc::c_uint)
                                    .wrapping_add(pending as uint32_t) as uint32_t as uint32_t;
                                tcp_rearm_rto_timer(tsk);
                                current_block_122 = 12696043255897098083;
                            } else {
                                current_block_122 = 17084335079990513572;
                            }
                        } else {
                            current_block_122 = 17084335079990513572;
                        }
                        match current_block_122 {
                            17084335079990513572 => {
                                if (*th).psh() != 0 {
                                    (*tsk).delacks = 0 as libc::c_int as uint8_t;
                                    tcp_send_ack(sk);
                                } else {
                                    let mut current_block_120: u64;
                                    if (*skb).dlen > 1000 as libc::c_uint {
                                        (*tsk)
                                            .delacks = ((*tsk).delacks as libc::c_int
                                            + 1 as libc::c_int) as uint8_t;
                                        if (*tsk).delacks as libc::c_int > 1 as libc::c_int {
                                            (*tsk).delacks = 0 as libc::c_int as uint8_t;
                                            tcp_send_ack(sk);
                                            current_block_120 = 16314074004867283505;
                                        } else {
                                            current_block_120 = 4184854853956156943;
                                        }
                                    } else {
                                        current_block_120 = 4184854853956156943;
                                    }
                                    match current_block_120 {
                                        4184854853956156943 => {
                                            if (*skb).dlen > 0 as libc::c_uint {
                                                (*tsk)
                                                    .delack = timer_add(
                                                    200 as libc::c_int as uint32_t,
                                                    Some(
                                                        tcp_send_delack
                                                            as unsafe extern "C" fn(
                                                                *mut libc::c_void,
                                                            ) -> *mut libc::c_void,
                                                    ),
                                                    &mut (*tsk).sk as *mut sock as *mut libc::c_void,
                                                );
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
            free_skb(skb);
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tcp_receive(
    mut tsk: *mut tcp_sock,
    mut buf: *mut libc::c_void,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut rlen: libc::c_int = 0;
    let mut curlen: libc::c_int = 0;
    let mut sk: *mut sock = 0 as *mut sock;
    let mut sock: *mut socket = 0 as *mut socket;
    rlen = 0 as libc::c_int;
    curlen = 0 as libc::c_int;
    sk = &mut (*tsk).sk;
    sock = (*sk).sock;
    memset(buf, 0 as libc::c_int, len as size_t);
    while rlen < len {
        curlen = tcp_data_dequeue(tsk, buf.offset(rlen as isize), len - rlen);
        rlen += curlen;
        if (*tsk).flags as libc::c_int & 8 as libc::c_int != 0 {
            (*tsk)
                .flags = ((*tsk).flags as libc::c_int & -(9 as libc::c_int)) as uint8_t;
            break;
        } else {
            if (*tsk).flags as libc::c_int & 1 as libc::c_int != 0 {
                break;
            }
            if rlen == len {
                break;
            }
            if (*sock).flags & 2048 as libc::c_int != 0 {
                if rlen == 0 as libc::c_int {
                    rlen = -(11 as libc::c_int);
                }
                break;
            } else {
                pthread_mutex_lock(&mut (*tsk).sk.recv_wait.lock);
                socket_release(sock);
                wait_sleep(&mut (*tsk).sk.recv_wait);
                pthread_mutex_unlock(&mut (*tsk).sk.recv_wait.lock);
                socket_wr_acquire(sock);
            }
        }
    }
    if rlen >= 0 as libc::c_int {
        tcp_rearm_user_timeout(sk);
    }
    return rlen;
}
static mut timers: list_head = unsafe {
    {
        let mut init = list_head {
            next: &timers as *const list_head as *mut list_head,
            prev: &timers as *const list_head as *mut list_head,
        };
        init
    }
};
static mut tick: libc::c_int = 0 as libc::c_int;
static mut lock___1: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
static mut rwlock: pthread_rwlock_t = __anonunion_pthread_rwlock_t_656928968 {
    __data: {
        let mut init = __pthread_rwlock_arch_t {
            __readers: 0 as libc::c_uint,
            __writers: 0 as libc::c_uint,
            __wrphase_futex: 0 as libc::c_uint,
            __writers_futex: 0 as libc::c_uint,
            __pad3: 0 as libc::c_uint,
            __pad4: 0 as libc::c_uint,
            __cur_writer: 0 as libc::c_int,
            __shared: 0 as libc::c_int,
            __rwelision: 0 as libc::c_int as libc::c_schar,
            __pad1: [
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
            ],
            __pad2: 0 as libc::c_ulong,
            __flags: 0 as libc::c_uint,
        };
        init
    },
};
unsafe extern "C" fn timer_debug() {}
unsafe extern "C" fn timer_free(mut t: *mut timer) {
    pthread_mutex_destroy(&mut (*t).lock);
    free(t as *mut libc::c_void);
}
unsafe extern "C" fn timer_alloc() -> *mut timer {
    let mut t: *mut timer = 0 as *mut timer;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        ::std::mem::size_of::<timer>() as libc::c_ulong,
        1 as libc::c_int as size_t,
    );
    t = tmp as *mut timer;
    pthread_mutex_init(
        &mut (*t).lock,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
    return t;
}
unsafe extern "C" fn timers_tick() {
    let mut item: *mut list_head = 0 as *mut list_head;
    let mut tmp: *mut list_head = 0 as *mut list_head;
    let mut t: *mut timer = 0 as *mut timer;
    let mut rc: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut th: pthread_t = 0;
    tmp = 0 as *mut libc::c_void as *mut list_head;
    t = 0 as *mut libc::c_void as *mut timer;
    rc = 0 as libc::c_int;
    rc = pthread_mutex_lock(&mut lock___1);
    if rc != 0 as libc::c_int {
        tmp___0 = strerror(rc);
        fprintf(
            stderr,
            b"Timer tick lock not acquired: %s\n\0" as *const u8 as *const libc::c_char,
            tmp___0,
        );
        return;
    }
    item = timers.next;
    tmp = (*item).next;
    while item as libc::c_ulong != &mut timers as *mut list_head as libc::c_ulong {
        if !item.is_null() {
            t = (item as *mut libc::c_char)
                .offset(
                    -(&mut (*(0 as *mut timer)).list as *mut list_head as libc::c_ulong
                        as isize),
                ) as *mut timer;
            rc = pthread_mutex_trylock(&mut (*t).lock);
            if rc != 0 as libc::c_int {
                if rc != 16 as libc::c_int {
                    tmp___1 = strerror(rc);
                    fprintf(
                        stderr,
                        b"Timer free mutex lock: %s\n\0" as *const u8
                            as *const libc::c_char,
                        tmp___1,
                    );
                }
            } else {
                if (*t).cancelled == 0 {
                    if (*t).expires < tick as uint32_t {
                        (*t).cancelled = 1 as libc::c_int;
                        pthread_create(
                            &mut th as *mut pthread_t,
                            0 as *mut libc::c_void as *const pthread_attr_t,
                            (*t).handler,
                            (*t).arg,
                        );
                    }
                }
                if (*t).cancelled != 0 {
                    if (*t).refcnt == 0 as libc::c_int {
                        list_del(&mut (*t).list);
                        pthread_mutex_unlock(&mut (*t).lock);
                        timer_free(t);
                    } else {
                        pthread_mutex_unlock(&mut (*t).lock);
                    }
                } else {
                    pthread_mutex_unlock(&mut (*t).lock);
                }
            }
        }
        item = tmp;
        tmp = (*item).next;
    }
    pthread_mutex_unlock(&mut lock___1);
}
pub unsafe extern "C" fn timer_oneshot(
    mut expire: uint32_t,
    mut handler: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    mut arg: *mut libc::c_void,
) {
    let mut t: *mut timer = 0 as *mut timer;
    let mut tmp: *mut timer = 0 as *mut timer;
    let mut tick___0: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = timer_alloc();
    t = tmp;
    tmp___0 = timer_get_tick();
    tick___0 = tmp___0;
    (*t).refcnt = 0 as libc::c_int;
    (*t).expires = (tick___0 as uint32_t).wrapping_add(expire);
    (*t).cancelled = 0 as libc::c_int;
    if (*t).expires < tick___0 as uint32_t {
        fprintf(
            stderr,
            b"ERR: Timer expiry integer wrap around\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*t).handler = handler;
    (*t).arg = arg;
    pthread_mutex_lock(&mut lock___1);
    list_add_tail(&mut (*t).list, &mut timers);
    pthread_mutex_unlock(&mut lock___1);
}
pub unsafe extern "C" fn timer_add(
    mut expire: uint32_t,
    mut handler: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    mut arg: *mut libc::c_void,
) -> *mut timer {
    let mut t: *mut timer = 0 as *mut timer;
    let mut tmp: *mut timer = 0 as *mut timer;
    let mut tick___0: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = timer_alloc();
    t = tmp;
    tmp___0 = timer_get_tick();
    tick___0 = tmp___0;
    (*t).refcnt = 1 as libc::c_int;
    (*t).expires = (tick___0 as uint32_t).wrapping_add(expire);
    (*t).cancelled = 0 as libc::c_int;
    if (*t).expires < tick___0 as uint32_t {
        fprintf(
            stderr,
            b"ERR: Timer expiry integer wrap around\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*t).handler = handler;
    (*t).arg = arg;
    pthread_mutex_lock(&mut lock___1);
    list_add_tail(&mut (*t).list, &mut timers);
    pthread_mutex_unlock(&mut lock___1);
    return t;
}
pub unsafe extern "C" fn timer_release(mut t: *mut timer) {
    let mut rc: libc::c_int = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    rc = 0 as libc::c_int;
    if t.is_null() {
        return;
    }
    rc = pthread_mutex_lock(&mut (*t).lock);
    if rc != 0 as libc::c_int {
        tmp = strerror(rc);
        fprintf(
            stderr,
            b"Timer release lock: %s\n\0" as *const u8 as *const libc::c_char,
            tmp,
        );
        return;
    }
    (*t).refcnt -= 1;
    pthread_mutex_unlock(&mut (*t).lock);
}
pub unsafe extern "C" fn timer_cancel(mut t: *mut timer) {
    let mut rc: libc::c_int = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    rc = 0 as libc::c_int;
    if t.is_null() {
        return;
    }
    rc = pthread_mutex_lock(&mut (*t).lock);
    if rc != 0 as libc::c_int {
        tmp = strerror(rc);
        fprintf(
            stderr,
            b"Timer cancel lock: %s\n\0" as *const u8 as *const libc::c_char,
            tmp,
        );
        return;
    }
    (*t).refcnt -= 1;
    (*t).cancelled = 1 as libc::c_int;
    pthread_mutex_unlock(&mut (*t).lock);
}
pub unsafe extern "C" fn timers_start() -> *mut libc::c_void {
    let mut tmp: libc::c_int = 0;
    loop {
        tmp = usleep(10000 as libc::c_int as __useconds_t);
        if tmp != 0 as libc::c_int {
            perror(b"Timer usleep\0" as *const u8 as *const libc::c_char);
        }
        pthread_rwlock_wrlock(&mut rwlock);
        tick += 10 as libc::c_int;
        pthread_rwlock_unlock(&mut rwlock);
        timers_tick();
        if tick % 5000 as libc::c_int == 0 as libc::c_int {
            socket_debug();
            timer_debug();
        }
    };
}
pub unsafe extern "C" fn timer_get_tick() -> libc::c_int {
    let mut copy: libc::c_int = 0;
    copy = 0 as libc::c_int;
    pthread_rwlock_rdlock(&mut rwlock);
    copy = tick;
    pthread_rwlock_unlock(&mut rwlock);
    return copy;
}
#[inline]
unsafe extern "C" fn skb_queue_add(
    mut list: *mut sk_buff_head,
    mut new: *mut sk_buff,
    mut next: *mut sk_buff,
) {
    list_add_tail(&mut (*new).list, &mut (*next).list);
    (*list).qlen = ((*list).qlen).wrapping_add(1);
}
unsafe extern "C" fn tcp_data_insert_ordered(
    mut queue: *mut sk_buff_head,
    mut skb: *mut sk_buff,
) {
    let mut next: *mut sk_buff = 0 as *mut sk_buff;
    let mut item: *mut list_head = 0 as *mut list_head;
    let mut tmp: *mut list_head = 0 as *mut list_head;
    item = (*queue).head.next;
    tmp = (*item).next;
    while item as libc::c_ulong != &mut (*queue).head as *mut list_head as libc::c_ulong
    {
        next = (item as *mut libc::c_char)
            .offset(
                -(&mut (*(0 as *mut sk_buff)).list as *mut list_head as libc::c_ulong
                    as isize),
            ) as *mut sk_buff;
        if (*skb).seq < (*next).seq {
            if (*skb).end_seq > (*next).seq {
                fprintf(
                    stderr,
                    b"Could not join skbs\n\0" as *const u8 as *const libc::c_char,
                );
            } else {
                (*skb).refcnt += 1;
                skb_queue_add(queue, skb, next);
                return;
            }
        } else if (*skb).seq == (*next).seq {
            return
        }
        item = tmp;
        tmp = (*item).next;
    }
    (*skb).refcnt += 1;
    skb_queue_tail(queue, skb);
}
unsafe extern "C" fn tcp_consume_ofo_queue(mut tsk: *mut tcp_sock) {
    let mut sk: *mut sock = 0 as *mut sock;
    let mut tcb: *mut tcb = 0 as *mut tcb;
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    sk = &mut (*tsk).sk;
    tcb = &mut (*tsk).tcb;
    skb = 0 as *mut libc::c_void as *mut sk_buff;
    loop {
        skb = skb_peek(&mut (*tsk).ofo_queue);
        if !(skb as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
            break;
        }
        if !((*tcb).rcv_nxt == (*skb).seq) {
            break;
        }
        (*tcb)
            .rcv_nxt = ((*tcb).rcv_nxt as libc::c_uint).wrapping_add((*skb).dlen)
            as uint32_t as uint32_t;
        skb_dequeue(&mut (*tsk).ofo_queue);
        skb_queue_tail(&mut (*sk).receive_queue, skb);
    };
}
pub unsafe extern "C" fn tcp_data_dequeue(
    mut tsk: *mut tcp_sock,
    mut user_buf: *mut libc::c_void,
    mut userlen: libc::c_int,
) -> libc::c_int {
    let mut sk: *mut sock = 0 as *mut sock;
    let mut th: *mut tcphdr = 0 as *mut tcphdr;
    let mut rlen: libc::c_int = 0;
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    let mut tmp: *mut sk_buff = 0 as *mut sk_buff;
    let mut dlen: libc::c_int = 0;
    let mut tmp___0: uint32_t = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    sk = &mut (*tsk).sk;
    rlen = 0 as libc::c_int;
    loop {
        tmp___1 = skb_queue_empty(
            &mut (*sk).receive_queue as *mut sk_buff_head as *const sk_buff_head,
        );
        if tmp___1 != 0 {
            break;
        }
        if !(rlen < userlen) {
            break;
        }
        tmp = skb_peek(&mut (*sk).receive_queue);
        skb = tmp;
        if skb as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            break;
        }
        th = tcp_hdr(skb as *const sk_buff);
        if (rlen as uint32_t).wrapping_add((*skb).dlen) > userlen as uint32_t {
            tmp___0 = (userlen - rlen) as uint32_t;
        } else {
            tmp___0 = (*skb).dlen;
        }
        dlen = tmp___0 as libc::c_int;
        memcpy(user_buf, (*skb).payload as *const libc::c_void, dlen as size_t);
        (*skb)
            .dlen = ((*skb).dlen as libc::c_uint).wrapping_sub(dlen as uint32_t)
            as uint32_t as uint32_t;
        (*skb).payload = ((*skb).payload).offset(dlen as isize);
        rlen += dlen;
        user_buf = user_buf.offset(dlen as isize);
        if (*skb).dlen == 0 as libc::c_uint {
            if (*th).psh() != 0 {
                (*tsk)
                    .flags = ((*tsk).flags as libc::c_int | 8 as libc::c_int) as uint8_t;
            }
            skb_dequeue(&mut (*sk).receive_queue);
            (*skb).refcnt -= 1;
            free_skb(skb);
        }
    }
    tmp___2 = skb_queue_empty(
        &mut (*sk).receive_queue as *mut sk_buff_head as *const sk_buff_head,
    );
    if tmp___2 != 0 {
        if (*tsk).flags as libc::c_int & 1 as libc::c_int == 0 {
            (*sk)
                .poll_events = ((*sk).poll_events as libc::c_int & -(2 as libc::c_int))
                as libc::c_short;
        }
    }
    return rlen;
}
pub unsafe extern "C" fn tcp_data_queue(
    mut tsk: *mut tcp_sock,
    mut th: *mut tcphdr,
    mut skb: *mut sk_buff,
) -> libc::c_int {
    let mut sk: *mut sock = 0 as *mut sock;
    let mut tcb: *mut tcb = 0 as *mut tcb;
    let mut rc: libc::c_int = 0;
    let mut expected: libc::c_int = 0;
    sk = &mut (*tsk).sk;
    tcb = &mut (*tsk).tcb;
    rc = 0 as libc::c_int;
    if (*tcb).rcv_wnd == 0 {
        free_skb(skb);
        return -(1 as libc::c_int);
    }
    expected = ((*skb).seq == (*tcb).rcv_nxt) as libc::c_int;
    if expected != 0 {
        (*tcb)
            .rcv_nxt = ((*tcb).rcv_nxt as libc::c_uint).wrapping_add((*skb).dlen)
            as uint32_t as uint32_t;
        (*skb).refcnt += 1;
        skb_queue_tail(&mut (*sk).receive_queue, skb);
        tcp_consume_ofo_queue(tsk);
        (*sk)
            .poll_events = ((*sk).poll_events as libc::c_int | 195 as libc::c_int)
            as libc::c_short;
        (Some(((*(*tsk).sk.ops).recv_notify).expect("non-null function pointer")))
            .expect("non-null function pointer")(&mut (*tsk).sk);
    } else {
        tcp_data_insert_ordered(&mut (*tsk).ofo_queue, skb);
        if (*tsk).sackok != 0 {
            tcp_calculate_sacks(tsk);
        }
        tcp_send_ack(sk);
    }
    return rc;
}
static mut routes: list_head = unsafe {
    {
        let mut init = list_head {
            next: &routes as *const list_head as *mut list_head,
            prev: &routes as *const list_head as *mut list_head,
        };
        init
    }
};
unsafe extern "C" fn route_alloc(
    mut dst: uint32_t,
    mut gateway: uint32_t,
    mut netmask: uint32_t,
    mut flags: uint8_t,
    mut metric: uint32_t,
    mut dev___0: *mut netdev,
) -> *mut rtentry {
    let mut rt: *mut rtentry = 0 as *mut rtentry;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<rtentry>() as libc::c_ulong);
    rt = tmp as *mut rtentry;
    list_init(&mut (*rt).list);
    (*rt).dst = dst;
    (*rt).gateway = gateway;
    (*rt).netmask = netmask;
    (*rt).flags = flags;
    (*rt).metric = metric;
    (*rt).dev = dev___0;
    return rt;
}
pub unsafe extern "C" fn route_add(
    mut dst: uint32_t,
    mut gateway: uint32_t,
    mut netmask: uint32_t,
    mut flags: uint8_t,
    mut metric: uint32_t,
    mut dev___0: *mut netdev,
) {
    let mut rt: *mut rtentry = 0 as *mut rtentry;
    let mut tmp: *mut rtentry = 0 as *mut rtentry;
    tmp = route_alloc(dst, gateway, netmask, flags, metric, dev___0);
    rt = tmp;
    list_add_tail(&mut (*rt).list, &mut routes);
}
pub unsafe extern "C" fn route_init() {
    let mut tmp: uint32_t = 0;
    route_add(
        (*loop_0).addr,
        0 as libc::c_int as uint32_t,
        4278190080 as libc::c_uint,
        1 as libc::c_int as uint8_t,
        0 as libc::c_int as uint32_t,
        loop_0,
    );
    route_add(
        (*netdev).addr,
        0 as libc::c_int as uint32_t,
        4294967040 as libc::c_uint,
        4 as libc::c_int as uint8_t,
        0 as libc::c_int as uint32_t,
        netdev,
    );
    tmp = ip_parse(tapaddr);
    route_add(
        0 as libc::c_int as uint32_t,
        tmp,
        0 as libc::c_int as uint32_t,
        2 as libc::c_int as uint8_t,
        0 as libc::c_int as uint32_t,
        netdev,
    );
}
pub unsafe extern "C" fn route_lookup(mut daddr: uint32_t) -> *mut rtentry {
    let mut item: *mut list_head = 0 as *mut list_head;
    let mut rt: *mut rtentry = 0 as *mut rtentry;
    rt = 0 as *mut libc::c_void as *mut rtentry;
    item = routes.next;
    while item as libc::c_ulong != &mut routes as *mut list_head as libc::c_ulong {
        rt = (item as *mut libc::c_char)
            .offset(
                -(&mut (*(0 as *mut rtentry)).list as *mut list_head as libc::c_ulong
                    as isize),
            ) as *mut rtentry;
        if daddr & (*rt).netmask == (*rt).dst & (*rt).netmask {
            break;
        }
        item = (*item).next;
    }
    return rt;
}
pub unsafe extern "C" fn free_routes() {
    let mut item: *mut list_head = 0 as *mut list_head;
    let mut tmp: *mut list_head = 0 as *mut list_head;
    let mut rt: *mut rtentry = 0 as *mut rtentry;
    item = routes.next;
    tmp = (*item).next;
    while item as libc::c_ulong != &mut routes as *mut list_head as libc::c_ulong {
        rt = (item as *mut libc::c_char)
            .offset(
                -(&mut (*(0 as *mut rtentry)).list as *mut list_head as libc::c_ulong
                    as isize),
            ) as *mut rtentry;
        list_del(item);
        free(rt as *mut libc::c_void);
        item = tmp;
        tmp = (*item).next;
    }
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
