use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pipe2(__pipedes: *mut libc::c_int, __flags: libc::c_int) -> libc::c_int;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execlp(
        __file: *const libc::c_char,
        __arg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fork() -> __pid_t;
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn send(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
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
    fn ntohs(__netshort: uint16_t) -> uint16_t;
    fn htons(__hostshort: uint16_t) -> uint16_t;
    fn gethostent() -> *mut hostent;
    fn getservbyname(
        __name: *const libc::c_char,
        __proto: *const libc::c_char,
    ) -> *mut servent;
    fn getservbyname_r(
        __name: *const libc::c_char,
        __proto: *const libc::c_char,
        __result_buf: *mut servent,
        __buf: *mut libc::c_char,
        __buflen: size_t,
        __result: *mut *mut servent,
    ) -> libc::c_int;
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
    fn inet_pton(
        __af: libc::c_int,
        __cp: *const libc::c_char,
        __buf: *mut libc::c_void,
    ) -> libc::c_int;
    fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: socklen_t,
    ) -> *const libc::c_char;
    fn inet_aton(__cp: *const libc::c_char, __inp: *mut in_addr) -> libc::c_int;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn srand(__seed: libc::c_uint);
    fn abort() -> !;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn ntohl(__netlong: uint32_t) -> uint32_t;
    fn htonl(__hostlong: uint32_t) -> uint32_t;
    fn dlsym(
        __handle: *mut libc::c_void,
        __name: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn dlerror() -> *mut libc::c_char;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn pthread_once(
        __once_control: *mut pthread_once_t,
        __init_routine: Option::<unsafe extern "C" fn() -> ()>,
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
    fn pthread_attr_init(__attr: *mut pthread_attr_t) -> libc::c_int;
    fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> libc::c_int;
    fn pthread_attr_setstacksize(
        __attr: *mut pthread_attr_t,
        __stacksize: size_t,
    ) -> libc::c_int;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn dprintf(__fd: libc::c_int, __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn strerror_r(
        __errnum: libc::c_int,
        __buf: *mut libc::c_char,
        __buflen: size_t,
    ) -> *mut libc::c_char;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn recvfrom(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> ssize_t;
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
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
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
pub type pid_t = __pid_t;
pub type socklen_t = __socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
pub union __anonunion_pthread_mutexattr_t_488594144 {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
pub type pthread_mutexattr_t = __anonunion_pthread_mutexattr_t_488594144;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_mutex_t_335460617 {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type pthread_mutex_t = __anonunion_pthread_mutex_t_335460617;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostent {
    pub h_name: *mut libc::c_char,
    pub h_aliases: *mut *mut libc::c_char,
    pub h_addrtype: libc::c_int,
    pub h_length: libc::c_int,
    pub h_addr_list: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct servent {
    pub s_name: *mut libc::c_char,
    pub s_aliases: *mut *mut libc::c_char,
    pub s_port: libc::c_int,
    pub s_proto: *mut libc::c_char,
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
pub union __anonunion_ip_type4_826858479 {
    pub octet: [libc::c_uchar; 4],
    pub as_int: uint32_t,
}
pub type ip_type4 = __anonunion_ip_type4_826858479;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_addr_69655148 {
    pub v4: ip_type4,
    pub v6: [libc::c_uchar; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_ip_type_888880172 {
    pub addr: __anonunion_addr_69655148,
    pub is_v6: libc::c_char,
}
pub type ip_type = __anonstruct_ip_type_888880172;
pub type __anonenum_proxy_type_52082966 = libc::c_uint;
pub const RAW_TYPE: __anonenum_proxy_type_52082966 = 3;
pub const SOCKS5_TYPE: __anonenum_proxy_type_52082966 = 2;
pub const SOCKS4_TYPE: __anonenum_proxy_type_52082966 = 1;
pub const HTTP_TYPE: __anonenum_proxy_type_52082966 = 0;
pub type proxy_type = __anonenum_proxy_type_52082966;
pub type __anonenum_chain_type_723445919 = libc::c_uint;
pub const ROUND_ROBIN_TYPE: __anonenum_chain_type_723445919 = 3;
pub const RANDOM_TYPE: __anonenum_chain_type_723445919 = 2;
pub const STRICT_TYPE: __anonenum_chain_type_723445919 = 1;
pub const DYNAMIC_TYPE: __anonenum_chain_type_723445919 = 0;
pub type chain_type = __anonenum_chain_type_723445919;
pub type __anonenum_proxy_state_523521024 = libc::c_uint;
pub const BUSY_STATE: __anonenum_proxy_state_523521024 = 3;
pub const BLOCKED_STATE: __anonenum_proxy_state_523521024 = 2;
pub const DOWN_STATE: __anonenum_proxy_state_523521024 = 1;
pub const PLAY_STATE: __anonenum_proxy_state_523521024 = 0;
pub type proxy_state = __anonenum_proxy_state_523521024;
pub type __anonenum_select_type_1061206784 = libc::c_uint;
pub const FIFOLY: __anonenum_select_type_1061206784 = 1;
pub const RANDOMLY: __anonenum_select_type_1061206784 = 0;
pub type select_type = __anonenum_select_type_1061206784;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_proxy_data_655008577 {
    pub ip: ip_type,
    pub port: libc::c_ushort,
    pub pt: proxy_type,
    pub ps: proxy_state,
    pub user: [libc::c_char; 256],
    pub pass: [libc::c_char; 256],
}
pub type proxy_data = __anonstruct_proxy_data_655008577;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gethostbyname_data {
    pub hostent_space: hostent,
    pub resolved_addr: in_addr_t,
    pub resolved_addr_p: [*mut libc::c_char; 2],
    pub addr_name: [libc::c_char; 256],
}
pub type dns_lookup_flavor = libc::c_uint;
pub const DNSLF_RDNS_DAEMON: dns_lookup_flavor = 3;
pub const DNSLF_RDNS_THREAD: dns_lookup_flavor = 2;
pub const DNSLF_RDNS_START: dns_lookup_flavor = 2;
pub const DNSLF_FORKEXEC: dns_lookup_flavor = 1;
pub const DNSLF_LIBC: dns_lookup_flavor = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo_data {
    pub addrinfo_space: addrinfo,
    pub sockaddr_space: sockaddr_storage,
    pub addr_name: [libc::c_char; 256],
}
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type clockid_t = __clockid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type pthread_once_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct____missing_field_name_625006857 {
    pub in_addr: in_addr,
    pub in_mask: in_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct____missing_field_name_944355870 {
    pub in6_addr: in6_addr,
    pub in6_prefix: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_165330731 {
    pub __annonCompField4: __anonstruct____missing_field_name_625006857,
    pub __annonCompField5: __anonstruct____missing_field_name_944355870,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_localaddr_arg_565560637 {
    pub family: sa_family_t,
    pub port: libc::c_ushort,
    pub __annonCompField6: __anonunion____missing_field_name_165330731,
}
pub type localaddr_arg = __anonstruct_localaddr_arg_565560637;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_dnat_arg_1073158885 {
    pub orig_dst: in_addr,
    pub new_dst: in_addr,
    pub orig_port: libc::c_ushort,
    pub new_port: libc::c_ushort,
}
pub type dnat_arg = __anonstruct_dnat_arg_1073158885;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_close_range_args_t_888776187 {
    pub first: libc::c_uint,
    pub last: libc::c_uint,
    pub flags: libc::c_uint,
}
pub type close_range_args_t = __anonstruct_close_range_args_t_888776187;
pub type __anonenum_rs_proxyType_741041632 = libc::c_uint;
pub const RS_PT_HTTP: __anonenum_rs_proxyType_741041632 = 3;
pub const RS_PT_SOCKS5: __anonenum_rs_proxyType_741041632 = 2;
pub const RS_PT_SOCKS4: __anonenum_rs_proxyType_741041632 = 1;
pub const RS_PT_NONE: __anonenum_rs_proxyType_741041632 = 0;
pub type rs_proxyType = __anonenum_rs_proxyType_741041632;
pub type __int16_t = libc::c_short;
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type int16_t = __int16_t;
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_fd_set_356711149 {
    pub fds_bits: [__fd_mask; 16],
}
pub type fd_set = __anonstruct_fd_set_356711149;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct at_msghdr {
    pub msgtype: libc::c_uchar,
    pub reserved: libc::c_char,
    pub datalen: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_m_242812694 {
    pub host: [libc::c_char; 260],
    pub ip: ip_type4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct at_msg {
    pub h: at_msghdr,
    pub m: __anonunion_m_242812694,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_string_hash_tuple_67209659 {
    pub hash: uint32_t,
    pub string: *mut libc::c_char,
}
pub type string_hash_tuple = __anonstruct_string_hash_tuple_67209659;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_internal_ip_lookup_table_78056353 {
    pub counter: uint32_t,
    pub capa: uint32_t,
    pub list: *mut *mut string_hash_tuple,
}
pub type internal_ip_lookup_table = __anonstruct_internal_ip_lookup_table_78056353;
pub type at_direction = libc::c_uint;
pub const ATD_MAX: at_direction = 2;
pub const ATD_CLIENT: at_direction = 1;
pub const ATD_SERVER: at_direction = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostsreader {
    pub f: *mut FILE,
    pub ip: *mut libc::c_char,
    pub name: *mut libc::c_char,
}
pub type uint_fast32_t = libc::c_ulong;
static mut version: [libc::c_char; 20] = [
    '4' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
];
pub unsafe extern "C" fn proxychains_get_version() -> *const libc::c_char {
    return version.as_ptr();
}
unsafe extern "C" fn poll_retry(
    mut fds: *mut pollfd,
    mut nfsd: nfds_t,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut time_remain: libc::c_int = 0;
    let mut time_elapsed: libc::c_int = 0;
    let mut start_time: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    time_remain = timeout;
    time_elapsed = 0 as libc::c_int;
    gettimeofday(&mut start_time as *mut timeval, 0 as *mut libc::c_void);
    loop {
        ret = poll(fds, nfsd, time_remain);
        gettimeofday(&mut tv as *mut timeval, 0 as *mut libc::c_void);
        time_elapsed = ((tv.tv_sec - start_time.tv_sec) * 1000 as libc::c_long
            + (tv.tv_usec - start_time.tv_usec) / 1000 as libc::c_long) as libc::c_int;
        time_remain = timeout - time_elapsed;
        if !(ret == -(1 as libc::c_int)) {
            break;
        }
        tmp = __errno_location();
        if !(*tmp == 4 as libc::c_int) {
            break;
        }
        if !(time_remain > 0 as libc::c_int) {
            break;
        }
    }
    return ret;
}
static mut base64: [libc::c_char; 65] = [
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
    'a' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
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
    '+' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
];
unsafe extern "C" fn encode_base_64(
    mut src: *mut libc::c_char,
    mut dest: *mut libc::c_char,
    mut max_len: libc::c_int,
) {
    let mut n: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___7: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___8: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___9: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___10: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___11: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___12: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = strlen(src as *const libc::c_char);
    l = tmp as libc::c_int;
    max_len = (max_len - 1 as libc::c_int) / 4 as libc::c_int;
    i = 0 as libc::c_int;
    while i < max_len {
        match l {
            0 => {}
            1 => {
                n = (*src.offset(0 as libc::c_int as isize) as libc::c_int)
                    << 16 as libc::c_int;
                tmp___0 = dest;
                dest = dest.offset(1);
                *tmp___0 = base64[(n >> 18 as libc::c_int & 63 as libc::c_int) as usize];
                tmp___1 = dest;
                dest = dest.offset(1);
                *tmp___1 = base64[(n >> 12 as libc::c_int & 63 as libc::c_int) as usize];
                tmp___2 = dest;
                dest = dest.offset(1);
                *tmp___2 = '=' as i32 as libc::c_char;
                tmp___3 = dest;
                dest = dest.offset(1);
                *tmp___3 = '=' as i32 as libc::c_char;
            }
            2 => {
                n = (*src.offset(0 as libc::c_int as isize) as libc::c_int)
                    << 16 as libc::c_int
                    | (*src.offset(1 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int;
                tmp___4 = dest;
                dest = dest.offset(1);
                *tmp___4 = base64[(n >> 18 as libc::c_int & 63 as libc::c_int) as usize];
                tmp___5 = dest;
                dest = dest.offset(1);
                *tmp___5 = base64[(n >> 12 as libc::c_int & 63 as libc::c_int) as usize];
                tmp___6 = dest;
                dest = dest.offset(1);
                *tmp___6 = base64[(n >> 6 as libc::c_int & 63 as libc::c_int) as usize];
                tmp___7 = dest;
                dest = dest.offset(1);
                *tmp___7 = '=' as i32 as libc::c_char;
            }
            _ => {
                n = (*src.offset(0 as libc::c_int as isize) as libc::c_int)
                    << 16 as libc::c_int
                    | (*src.offset(1 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int
                    | *src.offset(2 as libc::c_int as isize) as libc::c_int;
                tmp___8 = dest;
                dest = dest.offset(1);
                *tmp___8 = base64[(n >> 18 as libc::c_int & 63 as libc::c_int) as usize];
                tmp___9 = dest;
                dest = dest.offset(1);
                *tmp___9 = base64[(n >> 12 as libc::c_int & 63 as libc::c_int) as usize];
                tmp___10 = dest;
                dest = dest.offset(1);
                *tmp___10 = base64[(n >> 6 as libc::c_int & 63 as libc::c_int) as usize];
                tmp___11 = dest;
                dest = dest.offset(1);
                *tmp___11 = base64[(n & 63 as libc::c_int) as usize];
            }
        }
        if l < 3 as libc::c_int {
            break;
        }
        i += 1;
        src = src.offset(3 as libc::c_int as isize);
        l -= 3 as libc::c_int;
    }
    tmp___12 = dest;
    dest = dest.offset(1);
    *tmp___12 = 0 as libc::c_int as libc::c_char;
}
pub unsafe extern "C" fn proxychains_write_log(
    mut str: *mut libc::c_char,
    mut args: ...
) {
    let mut buff: [libc::c_char; 4096] = [0; 4096];
    let mut arglist: ::std::ffi::VaListImpl;
    if proxychains_quiet_mode == 0 {
        arglist = args.clone();
        vsnprintf(
            buff.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
            str as *const libc::c_char,
            arglist.as_va_list(),
        );
        fprintf(stderr, b"%s\0" as *const u8 as *const libc::c_char, buff.as_mut_ptr());
        fflush(stderr);
    }
}
unsafe extern "C" fn write_n_bytes(
    mut fd: libc::c_int,
    mut buff: *mut libc::c_char,
    mut size: size_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut wrote: size_t = 0;
    let mut tmp: ssize_t = 0;
    i = 0 as libc::c_int;
    wrote = 0 as libc::c_int as size_t;
    loop {
        tmp = write(
            fd,
            buff.offset(wrote as isize) as *const libc::c_void,
            size.wrapping_sub(wrote),
        );
        i = tmp as libc::c_int;
        if i <= 0 as libc::c_int {
            return i;
        }
        wrote = (wrote as libc::c_ulong).wrapping_add(i as size_t) as size_t as size_t;
        if wrote == size {
            return wrote as libc::c_int;
        }
    };
}
unsafe extern "C" fn read_n_bytes(
    mut fd: libc::c_int,
    mut buff: *mut libc::c_char,
    mut size: size_t,
) -> libc::c_int {
    let mut ready: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut pfd: [pollfd; 1] = [pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    }; 1];
    let mut tmp: ssize_t = 0;
    pfd[0 as libc::c_int as usize].fd = fd;
    pfd[0 as libc::c_int as usize].events = 1 as libc::c_int as libc::c_short;
    i = 0 as libc::c_int as size_t;
    while i < size {
        pfd[0 as libc::c_int as usize].revents = 0 as libc::c_int as libc::c_short;
        ready = poll_retry(
            pfd.as_mut_ptr(),
            1 as libc::c_int as nfds_t,
            tcp_read_time_out,
        );
        if ready != 1 as libc::c_int {
            return -(1 as libc::c_int)
        } else {
            if pfd[0 as libc::c_int as usize].revents as libc::c_int & 1 as libc::c_int
                == 0
            {
                return -(1 as libc::c_int)
            } else {
                tmp = read(
                    fd,
                    buff.offset(i as isize) as *mut libc::c_void,
                    1 as libc::c_int as size_t,
                );
                if 1 as libc::c_long != tmp {
                    return -(1 as libc::c_int);
                }
            }
        }
        i = i.wrapping_add(1);
    }
    return size as libc::c_int;
}
unsafe extern "C" fn timed_connect(
    mut sock: libc::c_int,
    mut addr: *const sockaddr,
    mut len: socklen_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut value: libc::c_int = 0;
    let mut value_len: socklen_t = 0;
    let mut pfd: [pollfd; 1] = [pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    }; 1];
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    pfd[0 as libc::c_int as usize].fd = sock;
    pfd[0 as libc::c_int as usize].events = 4 as libc::c_int as libc::c_short;
    fcntl(sock, 4 as libc::c_int, 2048 as libc::c_int);
    ret = (Some(true_connect.expect("non-null function pointer")))
        .expect("non-null function pointer")(sock, addr, len);
    let mut current_block_25: u64;
    if ret == -(1 as libc::c_int) {
        tmp = __errno_location();
        if *tmp == 115 as libc::c_int {
            ret = poll_retry(
                pfd.as_mut_ptr(),
                1 as libc::c_int as nfds_t,
                tcp_connect_time_out,
            );
            if ret == 1 as libc::c_int {
                value_len = ::std::mem::size_of::<socklen_t>() as libc::c_ulong
                    as socklen_t;
                getsockopt(
                    sock,
                    1 as libc::c_int,
                    4 as libc::c_int,
                    &mut value as *mut libc::c_int as *mut libc::c_void,
                    &mut value_len as *mut socklen_t,
                );
                if value == 0 {
                    ret = 0 as libc::c_int;
                } else {
                    ret = -(1 as libc::c_int);
                }
            } else {
                ret = -(1 as libc::c_int);
            }
            current_block_25 = 5689316957504528238;
        } else {
            current_block_25 = 5904599828758192;
        }
    } else {
        current_block_25 = 5904599828758192;
    }
    match current_block_25 {
        5904599828758192 => {
            if ret != 0 as libc::c_int {
                ret = -(1 as libc::c_int);
            }
        }
        _ => {}
    }
    fcntl(sock, 4 as libc::c_int, 0 as libc::c_int);
    return ret;
}
unsafe extern "C" fn tunnel_to(
    mut sock: libc::c_int,
    mut ip: ip_type,
    mut port: libc::c_ushort,
    mut pt: proxy_type,
    mut user: *mut libc::c_char,
    mut pass: *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut dns_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hostnamebuf: [libc::c_char; 256] = [0; 256];
    let mut dns_len: size_t = 0;
    let mut ulen: size_t = 0;
    let mut tmp: size_t = 0;
    let mut passlen: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut len: libc::c_int = 0;
    let mut buff: [libc::c_uchar; 1024] = [0; 1024];
    let mut ip_buf: [libc::c_char; 46] = [0; 46];
    let mut v6: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *const libc::c_char = 0 as *const libc::c_char;
    let mut src: [libc::c_char; 512] = [0; 512];
    let mut dst: [libc::c_char; 2048] = [0; 2048];
    let mut hs_port: uint16_t = 0;
    let mut tmp___3: uint16_t = 0;
    let mut tmp___4: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___5: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___6: ssize_t = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_int = 0;
    let mut n_methods: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: libc::c_int = 0;
    let mut in_0: [libc::c_char; 2] = [0; 2];
    let mut out: [libc::c_char; 515] = [0; 515];
    let mut cur: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: size_t = 0;
    let mut tmp___13: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___14: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___15: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___16: libc::c_int = 0;
    let mut tmp___17: libc::c_int = 0;
    let mut buff_iter: libc::c_int = 0;
    let mut tmp___18: libc::c_int = 0;
    let mut tmp___19: libc::c_int = 0;
    let mut tmp___20: libc::c_int = 0;
    let mut tmp___21: libc::c_int = 0;
    let mut tmp___22: libc::c_int = 0;
    let mut tmp___23: libc::c_int = 0;
    let mut tmp___24: libc::c_int = 0;
    let mut tmp___25: libc::c_int = 0;
    let mut tmp___26: libc::c_int = 0;
    let mut tmp___27: libc::c_int = 0;
    let mut tmp___28: libc::c_int = 0;
    let mut tmp___29: libc::c_int = 0;
    dns_name = 0 as *mut libc::c_void as *mut libc::c_char;
    dns_len = 0 as libc::c_int as size_t;
    if ip.is_v6 == 0 {
        if proxychains_resolver as libc::c_uint >= 2 as libc::c_uint {
            if ip.addr.v4.octet[0 as libc::c_int as usize] as libc::c_uint
                == remote_dns_subnet
            {
                dns_len = rdns_get_host_for_ip(ip.addr.v4, hostnamebuf.as_mut_ptr());
                if dns_len == 0 {
                    current_block = 8149651411747124477;
                } else {
                    dns_name = hostnamebuf.as_mut_ptr();
                    current_block = 14434620278749266018;
                }
            } else {
                current_block = 14434620278749266018;
            }
        } else {
            current_block = 14434620278749266018;
        }
    } else {
        current_block = 14434620278749266018;
    }
    match current_block {
        14434620278749266018 => {
            tmp = strlen(user as *const libc::c_char);
            ulen = tmp;
            tmp___0 = strlen(pass as *const libc::c_char);
            passlen = tmp___0;
            if ulen > 255 as libc::c_ulong {
                proxychains_write_log(
                    b"[proxychains] error: maximum size of 255 for user/pass or domain name!\n\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            } else if passlen > 255 as libc::c_ulong {
                proxychains_write_log(
                    b"[proxychains] error: maximum size of 255 for user/pass or domain name!\n\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            } else if dns_len > 255 as libc::c_ulong {
                proxychains_write_log(
                    b"[proxychains] error: maximum size of 255 for user/pass or domain name!\n\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            } else {
                v6 = ip.is_v6 as libc::c_int;
                match pt as libc::c_uint {
                    3 => {
                        current_block = 2069327002525158974;
                        match current_block {
                            2069327002525158974 => return 0 as libc::c_int,
                            14004463445576069816 => {
                                if v6 != 0 {
                                    proxychains_write_log(
                                        b"[proxychains] error: SOCKS4 doesn't support ipv6 addresses\n\0"
                                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                                    );
                                } else {
                                    buff[0 as libc::c_int
                                        as usize] = 4 as libc::c_int as libc::c_uchar;
                                    buff[1 as libc::c_int
                                        as usize] = 1 as libc::c_int as libc::c_uchar;
                                    memcpy(
                                        &mut *buff.as_mut_ptr().offset(2 as libc::c_int as isize)
                                            as *mut libc::c_uchar as *mut libc::c_void,
                                        &mut port as *mut libc::c_ushort as *const libc::c_void,
                                        2 as libc::c_int as size_t,
                                    );
                                    if dns_len != 0 {
                                        ip
                                            .addr
                                            .v4
                                            .octet[0 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                        ip
                                            .addr
                                            .v4
                                            .octet[1 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                        ip
                                            .addr
                                            .v4
                                            .octet[2 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                        ip
                                            .addr
                                            .v4
                                            .octet[3 as libc::c_int
                                            as usize] = 1 as libc::c_int as libc::c_uchar;
                                    }
                                    memcpy(
                                        &mut *buff.as_mut_ptr().offset(4 as libc::c_int as isize)
                                            as *mut libc::c_uchar as *mut libc::c_void,
                                        &mut ip.addr.v4 as *mut ip_type4 as *const libc::c_void,
                                        4 as libc::c_int as size_t,
                                    );
                                    len = ulen.wrapping_add(1 as libc::c_ulong) as libc::c_int;
                                    if len > 1 as libc::c_int {
                                        memcpy(
                                            &mut *buff.as_mut_ptr().offset(8 as libc::c_int as isize)
                                                as *mut libc::c_uchar as *mut libc::c_void,
                                            user as *const libc::c_void,
                                            len as size_t,
                                        );
                                    } else {
                                        buff[8 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                    }
                                    if dns_len != 0 {
                                        memcpy(
                                            &mut *buff
                                                .as_mut_ptr()
                                                .offset((8 as libc::c_int + len) as isize)
                                                as *mut libc::c_uchar as *mut libc::c_void,
                                            dns_name as *const libc::c_void,
                                            dns_len.wrapping_add(1 as libc::c_ulong),
                                        );
                                        len = (len as size_t)
                                            .wrapping_add(dns_len.wrapping_add(1 as libc::c_ulong))
                                            as libc::c_int;
                                    }
                                    tmp___8 = write_n_bytes(
                                        sock,
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        (8 as libc::c_int + len) as size_t,
                                    );
                                    if !(len + 8 as libc::c_int != tmp___8) {
                                        tmp___9 = read_n_bytes(
                                            sock,
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            8 as libc::c_int as size_t,
                                        );
                                        if !(8 as libc::c_int != tmp___9) {
                                            if buff[0 as libc::c_int as usize] as libc::c_int
                                                != 0 as libc::c_int
                                            {
                                                return 5 as libc::c_int
                                            } else {
                                                if buff[1 as libc::c_int as usize] as libc::c_int
                                                    != 90 as libc::c_int
                                                {
                                                    return 5 as libc::c_int;
                                                }
                                            }
                                            return 0 as libc::c_int;
                                        }
                                    }
                                }
                            }
                            11638966953701390245 => {
                                if dns_len == 0 {
                                    if v6 != 0 {
                                        tmp___1 = 10 as libc::c_int;
                                    } else {
                                        tmp___1 = 2 as libc::c_int;
                                    }
                                    tmp___2 = inet_ntop(
                                        tmp___1,
                                        (ip.addr.v6).as_mut_ptr() as *const libc::c_void,
                                        ip_buf.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong
                                            as socklen_t,
                                    );
                                    if tmp___2.is_null() {
                                        proxychains_write_log(
                                            b"[proxychains] error: ip address conversion failed\n\0"
                                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                                        );
                                        current_block = 8149651411747124477;
                                    } else {
                                        dns_name = ip_buf.as_mut_ptr();
                                        current_block = 7018308795614528254;
                                    }
                                } else {
                                    current_block = 7018308795614528254;
                                }
                                match current_block {
                                    8149651411747124477 => {}
                                    _ => {
                                        if ulen != 0 {
                                            snprintf(
                                                src.as_mut_ptr(),
                                                ::std::mem::size_of::<[libc::c_char; 512]>()
                                                    as libc::c_ulong,
                                                b"%s:%s\0" as *const u8 as *const libc::c_char,
                                                user,
                                                pass,
                                            );
                                            encode_base_64(
                                                src.as_mut_ptr(),
                                                dst.as_mut_ptr(),
                                                ::std::mem::size_of::<[libc::c_char; 2048]>()
                                                    as libc::c_ulong as libc::c_int,
                                            );
                                        } else {
                                            dst[0 as libc::c_int
                                                as usize] = 0 as libc::c_int as libc::c_char;
                                        }
                                        tmp___3 = ntohs(port);
                                        hs_port = tmp___3;
                                        if ulen != 0 {
                                            tmp___4 = b"\r\n\0" as *const u8 as *const libc::c_char;
                                        } else {
                                            tmp___4 = dst.as_mut_ptr() as *const libc::c_char;
                                        }
                                        if ulen != 0 {
                                            tmp___5 = b"Proxy-Authorization: Basic \0" as *const u8
                                                as *const libc::c_char;
                                        } else {
                                            tmp___5 = dst.as_mut_ptr() as *const libc::c_char;
                                        }
                                        len = snprintf(
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            ::std::mem::size_of::<[libc::c_uchar; 1024]>()
                                                as libc::c_ulong,
                                            b"CONNECT %s:%d HTTP/1.0\r\nHost: %s:%d\r\n%s%s%s\r\n\0"
                                                as *const u8 as *const libc::c_char,
                                            dns_name,
                                            hs_port as libc::c_int,
                                            dns_name,
                                            hs_port as libc::c_int,
                                            tmp___5,
                                            dst.as_mut_ptr(),
                                            tmp___4,
                                        );
                                        if !(len < 0 as libc::c_int) {
                                            tmp___6 = send(
                                                sock,
                                                buff.as_mut_ptr() as *const libc::c_void,
                                                len as size_t,
                                                0 as libc::c_int,
                                            );
                                            if !(len as ssize_t != tmp___6) {
                                                len = 0 as libc::c_int;
                                                loop {
                                                    if !(len < 1024 as libc::c_int) {
                                                        current_block = 15587532755333643506;
                                                        break;
                                                    }
                                                    tmp___7 = read_n_bytes(
                                                        sock,
                                                        buff.as_mut_ptr().offset(len as isize) as *mut libc::c_char,
                                                        1 as libc::c_int as size_t,
                                                    );
                                                    if !(1 as libc::c_int == tmp___7) {
                                                        current_block = 8149651411747124477;
                                                        break;
                                                    }
                                                    len += 1;
                                                    if !(len > 4 as libc::c_int) {
                                                        continue;
                                                    }
                                                    if !(buff[(len - 1 as libc::c_int) as usize] as libc::c_int
                                                        == 10 as libc::c_int)
                                                    {
                                                        continue;
                                                    }
                                                    if !(buff[(len - 2 as libc::c_int) as usize] as libc::c_int
                                                        == 13 as libc::c_int)
                                                    {
                                                        continue;
                                                    }
                                                    if !(buff[(len - 3 as libc::c_int) as usize] as libc::c_int
                                                        == 10 as libc::c_int)
                                                    {
                                                        continue;
                                                    }
                                                    if buff[(len - 4 as libc::c_int) as usize] as libc::c_int
                                                        == 13 as libc::c_int
                                                    {
                                                        current_block = 15587532755333643506;
                                                        break;
                                                    }
                                                }
                                                match current_block {
                                                    8149651411747124477 => {}
                                                    _ => {
                                                        if !(len == 1024 as libc::c_int) {
                                                            if buff[9 as libc::c_int as usize] as libc::c_int
                                                                == 50 as libc::c_int
                                                            {
                                                                if buff[10 as libc::c_int as usize] as libc::c_int
                                                                    == 48 as libc::c_int
                                                                {
                                                                    if buff[11 as libc::c_int as usize] as libc::c_int
                                                                        == 48 as libc::c_int
                                                                    {
                                                                        return 0 as libc::c_int;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        return 5 as libc::c_int;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {
                                if ulen != 0 {
                                    tmp___10 = 2 as libc::c_int;
                                } else {
                                    tmp___10 = 1 as libc::c_int;
                                }
                                n_methods = tmp___10;
                                buff[0 as libc::c_int
                                    as usize] = 5 as libc::c_int as libc::c_uchar;
                                buff[1 as libc::c_int
                                    as usize] = n_methods as libc::c_uchar;
                                buff[2 as libc::c_int
                                    as usize] = 0 as libc::c_int as libc::c_uchar;
                                if ulen != 0 {
                                    buff[3 as libc::c_int
                                        as usize] = 2 as libc::c_int as libc::c_uchar;
                                }
                                tmp___11 = write_n_bytes(
                                    sock,
                                    buff.as_mut_ptr() as *mut libc::c_char,
                                    (2 as libc::c_int + n_methods) as size_t,
                                );
                                if !(2 as libc::c_int + n_methods != tmp___11) {
                                    tmp___12 = read_n_bytes(
                                        sock,
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        2 as libc::c_int as size_t,
                                    );
                                    if !(2 as libc::c_int != tmp___12) {
                                        if buff[0 as libc::c_int as usize] as libc::c_int
                                            != 5 as libc::c_int
                                        {
                                            current_block = 10418296791801432365;
                                        } else {
                                            if buff[1 as libc::c_int as usize] as libc::c_int
                                                != 0 as libc::c_int
                                            {
                                                if buff[1 as libc::c_int as usize] as libc::c_int
                                                    != 2 as libc::c_int
                                                {
                                                    current_block = 10418296791801432365;
                                                } else {
                                                    current_block = 5219368551394180541;
                                                }
                                            } else {
                                                current_block = 5219368551394180541;
                                            }
                                            match current_block {
                                                10418296791801432365 => {}
                                                _ => {
                                                    if buff[1 as libc::c_int as usize] as libc::c_int
                                                        == 2 as libc::c_int
                                                    {
                                                        cur = out.as_mut_ptr();
                                                        tmp___13 = cur;
                                                        cur = cur.offset(1);
                                                        *tmp___13 = 1 as libc::c_int as libc::c_char;
                                                        c = ulen & 255 as libc::c_ulong;
                                                        tmp___14 = cur;
                                                        cur = cur.offset(1);
                                                        *tmp___14 = c as libc::c_char;
                                                        memcpy(
                                                            cur as *mut libc::c_void,
                                                            user as *const libc::c_void,
                                                            c,
                                                        );
                                                        cur = cur.offset(c as isize);
                                                        c = passlen & 255 as libc::c_ulong;
                                                        tmp___15 = cur;
                                                        cur = cur.offset(1);
                                                        *tmp___15 = c as libc::c_char;
                                                        memcpy(
                                                            cur as *mut libc::c_void,
                                                            pass as *const libc::c_void,
                                                            c,
                                                        );
                                                        cur = cur.offset(c as isize);
                                                        tmp___16 = write_n_bytes(
                                                            sock,
                                                            out.as_mut_ptr(),
                                                            cur.offset_from(out.as_mut_ptr()) as libc::c_long as size_t,
                                                        );
                                                        if cur.offset_from(out.as_mut_ptr()) as libc::c_long
                                                            != tmp___16 as libc::c_long
                                                        {
                                                            current_block = 8149651411747124477;
                                                        } else {
                                                            tmp___17 = read_n_bytes(
                                                                sock,
                                                                in_0.as_mut_ptr(),
                                                                2 as libc::c_int as size_t,
                                                            );
                                                            if 2 as libc::c_int != tmp___17 {
                                                                current_block = 8149651411747124477;
                                                            } else {
                                                                if !(in_0[0 as libc::c_int as usize] as libc::c_int
                                                                    == 5 as libc::c_int)
                                                                {
                                                                    if !(in_0[0 as libc::c_int as usize] as libc::c_int
                                                                        == 1 as libc::c_int)
                                                                    {
                                                                        current_block = 8149651411747124477;
                                                                    } else {
                                                                        current_block = 6662862405959679103;
                                                                    }
                                                                } else {
                                                                    current_block = 6662862405959679103;
                                                                }
                                                                match current_block {
                                                                    8149651411747124477 => {}
                                                                    _ => {
                                                                        if in_0[1 as libc::c_int as usize] as libc::c_int
                                                                            != 0 as libc::c_int
                                                                        {
                                                                            return 5 as libc::c_int;
                                                                        }
                                                                        current_block = 7371321987304394147;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        current_block = 7371321987304394147;
                                                    }
                                                    match current_block {
                                                        8149651411747124477 => {}
                                                        _ => {
                                                            buff_iter = 0 as libc::c_int;
                                                            tmp___18 = buff_iter;
                                                            buff_iter += 1;
                                                            buff[tmp___18 as usize] = 5 as libc::c_int as libc::c_uchar;
                                                            tmp___19 = buff_iter;
                                                            buff_iter += 1;
                                                            buff[tmp___19 as usize] = 1 as libc::c_int as libc::c_uchar;
                                                            tmp___20 = buff_iter;
                                                            buff_iter += 1;
                                                            buff[tmp___20 as usize] = 0 as libc::c_int as libc::c_uchar;
                                                            if dns_len == 0 {
                                                                tmp___21 = buff_iter;
                                                                buff_iter += 1;
                                                                if v6 != 0 {
                                                                    buff[tmp___21 as usize] = 4 as libc::c_int as libc::c_uchar;
                                                                } else {
                                                                    buff[tmp___21 as usize] = 1 as libc::c_int as libc::c_uchar;
                                                                }
                                                                if v6 != 0 {
                                                                    tmp___22 = 16 as libc::c_int;
                                                                } else {
                                                                    tmp___22 = 4 as libc::c_int;
                                                                }
                                                                memcpy(
                                                                    buff.as_mut_ptr().offset(buff_iter as isize)
                                                                        as *mut libc::c_void,
                                                                    (ip.addr.v6).as_mut_ptr() as *const libc::c_void,
                                                                    tmp___22 as size_t,
                                                                );
                                                                if v6 != 0 {
                                                                    tmp___23 = 16 as libc::c_int;
                                                                } else {
                                                                    tmp___23 = 4 as libc::c_int;
                                                                }
                                                                buff_iter += tmp___23;
                                                            } else {
                                                                tmp___24 = buff_iter;
                                                                buff_iter += 1;
                                                                buff[tmp___24 as usize] = 3 as libc::c_int as libc::c_uchar;
                                                                tmp___25 = buff_iter;
                                                                buff_iter += 1;
                                                                buff[tmp___25
                                                                    as usize] = (dns_len & 255 as libc::c_ulong)
                                                                    as libc::c_uchar;
                                                                memcpy(
                                                                    buff.as_mut_ptr().offset(buff_iter as isize)
                                                                        as *mut libc::c_void,
                                                                    dns_name as *const libc::c_void,
                                                                    dns_len,
                                                                );
                                                                buff_iter = (buff_iter as size_t).wrapping_add(dns_len)
                                                                    as libc::c_int;
                                                            }
                                                            memcpy(
                                                                buff.as_mut_ptr().offset(buff_iter as isize)
                                                                    as *mut libc::c_void,
                                                                &mut port as *mut libc::c_ushort as *const libc::c_void,
                                                                2 as libc::c_int as size_t,
                                                            );
                                                            buff_iter += 2 as libc::c_int;
                                                            tmp___26 = write_n_bytes(
                                                                sock,
                                                                buff.as_mut_ptr() as *mut libc::c_char,
                                                                buff_iter as size_t,
                                                            );
                                                            if buff_iter != tmp___26 {
                                                                current_block = 8149651411747124477;
                                                            } else {
                                                                tmp___27 = read_n_bytes(
                                                                    sock,
                                                                    buff.as_mut_ptr() as *mut libc::c_char,
                                                                    4 as libc::c_int as size_t,
                                                                );
                                                                if 4 as libc::c_int != tmp___27 {
                                                                    current_block = 8149651411747124477;
                                                                } else if buff[0 as libc::c_int as usize] as libc::c_int
                                                                        != 5 as libc::c_int
                                                                    {
                                                                    current_block = 8149651411747124477;
                                                                } else if buff[1 as libc::c_int as usize] as libc::c_int
                                                                        != 0 as libc::c_int
                                                                    {
                                                                    current_block = 8149651411747124477;
                                                                } else {
                                                                    match buff[3 as libc::c_int as usize] as libc::c_int {
                                                                        1 => {
                                                                            current_block = 5117978787054293485;
                                                                            match current_block {
                                                                                1683520436886296249 => {
                                                                                    len = 16 as libc::c_int;
                                                                                    current_block = 6930811285952240378;
                                                                                }
                                                                                5117978787054293485 => {
                                                                                    len = 4 as libc::c_int;
                                                                                    current_block = 6930811285952240378;
                                                                                }
                                                                                _ => {
                                                                                    len = 0 as libc::c_int;
                                                                                    tmp___28 = read_n_bytes(
                                                                                        sock,
                                                                                        &mut len as *mut libc::c_int as *mut libc::c_char,
                                                                                        1 as libc::c_int as size_t,
                                                                                    );
                                                                                    if 1 as libc::c_int != tmp___28 {
                                                                                        current_block = 8149651411747124477;
                                                                                    } else {
                                                                                        current_block = 6930811285952240378;
                                                                                    }
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                8149651411747124477 => {}
                                                                                _ => {
                                                                                    tmp___29 = read_n_bytes(
                                                                                        sock,
                                                                                        buff.as_mut_ptr() as *mut libc::c_char,
                                                                                        (len + 2 as libc::c_int) as size_t,
                                                                                    );
                                                                                    if len + 2 as libc::c_int != tmp___29 {
                                                                                        current_block = 8149651411747124477;
                                                                                    } else {
                                                                                        return 0 as libc::c_int
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        4 => {
                                                                            current_block = 1683520436886296249;
                                                                            match current_block {
                                                                                1683520436886296249 => {
                                                                                    len = 16 as libc::c_int;
                                                                                    current_block = 6930811285952240378;
                                                                                }
                                                                                5117978787054293485 => {
                                                                                    len = 4 as libc::c_int;
                                                                                    current_block = 6930811285952240378;
                                                                                }
                                                                                _ => {
                                                                                    len = 0 as libc::c_int;
                                                                                    tmp___28 = read_n_bytes(
                                                                                        sock,
                                                                                        &mut len as *mut libc::c_int as *mut libc::c_char,
                                                                                        1 as libc::c_int as size_t,
                                                                                    );
                                                                                    if 1 as libc::c_int != tmp___28 {
                                                                                        current_block = 8149651411747124477;
                                                                                    } else {
                                                                                        current_block = 6930811285952240378;
                                                                                    }
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                8149651411747124477 => {}
                                                                                _ => {
                                                                                    tmp___29 = read_n_bytes(
                                                                                        sock,
                                                                                        buff.as_mut_ptr() as *mut libc::c_char,
                                                                                        (len + 2 as libc::c_int) as size_t,
                                                                                    );
                                                                                    if len + 2 as libc::c_int != tmp___29 {
                                                                                        current_block = 8149651411747124477;
                                                                                    } else {
                                                                                        return 0 as libc::c_int
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        3 => {
                                                                            current_block = 5428223447338481681;
                                                                            match current_block {
                                                                                1683520436886296249 => {
                                                                                    len = 16 as libc::c_int;
                                                                                    current_block = 6930811285952240378;
                                                                                }
                                                                                5117978787054293485 => {
                                                                                    len = 4 as libc::c_int;
                                                                                    current_block = 6930811285952240378;
                                                                                }
                                                                                _ => {
                                                                                    len = 0 as libc::c_int;
                                                                                    tmp___28 = read_n_bytes(
                                                                                        sock,
                                                                                        &mut len as *mut libc::c_int as *mut libc::c_char,
                                                                                        1 as libc::c_int as size_t,
                                                                                    );
                                                                                    if 1 as libc::c_int != tmp___28 {
                                                                                        current_block = 8149651411747124477;
                                                                                    } else {
                                                                                        current_block = 6930811285952240378;
                                                                                    }
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                8149651411747124477 => {}
                                                                                _ => {
                                                                                    tmp___29 = read_n_bytes(
                                                                                        sock,
                                                                                        buff.as_mut_ptr() as *mut libc::c_char,
                                                                                        (len + 2 as libc::c_int) as size_t,
                                                                                    );
                                                                                    if len + 2 as libc::c_int != tmp___29 {
                                                                                        current_block = 8149651411747124477;
                                                                                    } else {
                                                                                        return 0 as libc::c_int
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        _ => {
                                                                            current_block = 8149651411747124477;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        match current_block {
                                            8149651411747124477 => {}
                                            _ => {
                                                if buff[0 as libc::c_int as usize] as libc::c_int
                                                    == 5 as libc::c_int
                                                {
                                                    if buff[1 as libc::c_int as usize] as libc::c_int
                                                        == 255 as libc::c_int
                                                    {
                                                        return 5 as libc::c_int;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    0 => {
                        current_block = 11638966953701390245;
                        match current_block {
                            2069327002525158974 => return 0 as libc::c_int,
                            14004463445576069816 => {
                                if v6 != 0 {
                                    proxychains_write_log(
                                        b"[proxychains] error: SOCKS4 doesn't support ipv6 addresses\n\0"
                                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                                    );
                                } else {
                                    buff[0 as libc::c_int
                                        as usize] = 4 as libc::c_int as libc::c_uchar;
                                    buff[1 as libc::c_int
                                        as usize] = 1 as libc::c_int as libc::c_uchar;
                                    memcpy(
                                        &mut *buff.as_mut_ptr().offset(2 as libc::c_int as isize)
                                            as *mut libc::c_uchar as *mut libc::c_void,
                                        &mut port as *mut libc::c_ushort as *const libc::c_void,
                                        2 as libc::c_int as size_t,
                                    );
                                    if dns_len != 0 {
                                        ip
                                            .addr
                                            .v4
                                            .octet[0 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                        ip
                                            .addr
                                            .v4
                                            .octet[1 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                        ip
                                            .addr
                                            .v4
                                            .octet[2 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                        ip
                                            .addr
                                            .v4
                                            .octet[3 as libc::c_int
                                            as usize] = 1 as libc::c_int as libc::c_uchar;
                                    }
                                    memcpy(
                                        &mut *buff.as_mut_ptr().offset(4 as libc::c_int as isize)
                                            as *mut libc::c_uchar as *mut libc::c_void,
                                        &mut ip.addr.v4 as *mut ip_type4 as *const libc::c_void,
                                        4 as libc::c_int as size_t,
                                    );
                                    len = ulen.wrapping_add(1 as libc::c_ulong) as libc::c_int;
                                    if len > 1 as libc::c_int {
                                        memcpy(
                                            &mut *buff.as_mut_ptr().offset(8 as libc::c_int as isize)
                                                as *mut libc::c_uchar as *mut libc::c_void,
                                            user as *const libc::c_void,
                                            len as size_t,
                                        );
                                    } else {
                                        buff[8 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                    }
                                    if dns_len != 0 {
                                        memcpy(
                                            &mut *buff
                                                .as_mut_ptr()
                                                .offset((8 as libc::c_int + len) as isize)
                                                as *mut libc::c_uchar as *mut libc::c_void,
                                            dns_name as *const libc::c_void,
                                            dns_len.wrapping_add(1 as libc::c_ulong),
                                        );
                                        len = (len as size_t)
                                            .wrapping_add(dns_len.wrapping_add(1 as libc::c_ulong))
                                            as libc::c_int;
                                    }
                                    tmp___8 = write_n_bytes(
                                        sock,
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        (8 as libc::c_int + len) as size_t,
                                    );
                                    if !(len + 8 as libc::c_int != tmp___8) {
                                        tmp___9 = read_n_bytes(
                                            sock,
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            8 as libc::c_int as size_t,
                                        );
                                        if !(8 as libc::c_int != tmp___9) {
                                            if buff[0 as libc::c_int as usize] as libc::c_int
                                                != 0 as libc::c_int
                                            {
                                                return 5 as libc::c_int
                                            } else {
                                                if buff[1 as libc::c_int as usize] as libc::c_int
                                                    != 90 as libc::c_int
                                                {
                                                    return 5 as libc::c_int;
                                                }
                                            }
                                            return 0 as libc::c_int;
                                        }
                                    }
                                }
                            }
                            11638966953701390245 => {
                                if dns_len == 0 {
                                    if v6 != 0 {
                                        tmp___1 = 10 as libc::c_int;
                                    } else {
                                        tmp___1 = 2 as libc::c_int;
                                    }
                                    tmp___2 = inet_ntop(
                                        tmp___1,
                                        (ip.addr.v6).as_mut_ptr() as *const libc::c_void,
                                        ip_buf.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong
                                            as socklen_t,
                                    );
                                    if tmp___2.is_null() {
                                        proxychains_write_log(
                                            b"[proxychains] error: ip address conversion failed\n\0"
                                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                                        );
                                        current_block = 8149651411747124477;
                                    } else {
                                        dns_name = ip_buf.as_mut_ptr();
                                        current_block = 7018308795614528254;
                                    }
                                } else {
                                    current_block = 7018308795614528254;
                                }
                                match current_block {
                                    8149651411747124477 => {}
                                    _ => {
                                        if ulen != 0 {
                                            snprintf(
                                                src.as_mut_ptr(),
                                                ::std::mem::size_of::<[libc::c_char; 512]>()
                                                    as libc::c_ulong,
                                                b"%s:%s\0" as *const u8 as *const libc::c_char,
                                                user,
                                                pass,
                                            );
                                            encode_base_64(
                                                src.as_mut_ptr(),
                                                dst.as_mut_ptr(),
                                                ::std::mem::size_of::<[libc::c_char; 2048]>()
                                                    as libc::c_ulong as libc::c_int,
                                            );
                                        } else {
                                            dst[0 as libc::c_int
                                                as usize] = 0 as libc::c_int as libc::c_char;
                                        }
                                        tmp___3 = ntohs(port);
                                        hs_port = tmp___3;
                                        if ulen != 0 {
                                            tmp___4 = b"\r\n\0" as *const u8 as *const libc::c_char;
                                        } else {
                                            tmp___4 = dst.as_mut_ptr() as *const libc::c_char;
                                        }
                                        if ulen != 0 {
                                            tmp___5 = b"Proxy-Authorization: Basic \0" as *const u8
                                                as *const libc::c_char;
                                        } else {
                                            tmp___5 = dst.as_mut_ptr() as *const libc::c_char;
                                        }
                                        len = snprintf(
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            ::std::mem::size_of::<[libc::c_uchar; 1024]>()
                                                as libc::c_ulong,
                                            b"CONNECT %s:%d HTTP/1.0\r\nHost: %s:%d\r\n%s%s%s\r\n\0"
                                                as *const u8 as *const libc::c_char,
                                            dns_name,
                                            hs_port as libc::c_int,
                                            dns_name,
                                            hs_port as libc::c_int,
                                            tmp___5,
                                            dst.as_mut_ptr(),
                                            tmp___4,
                                        );
                                        if !(len < 0 as libc::c_int) {
                                            tmp___6 = send(
                                                sock,
                                                buff.as_mut_ptr() as *const libc::c_void,
                                                len as size_t,
                                                0 as libc::c_int,
                                            );
                                            if !(len as ssize_t != tmp___6) {
                                                len = 0 as libc::c_int;
                                                loop {
                                                    if !(len < 1024 as libc::c_int) {
                                                        current_block = 15587532755333643506;
                                                        break;
                                                    }
                                                    tmp___7 = read_n_bytes(
                                                        sock,
                                                        buff.as_mut_ptr().offset(len as isize) as *mut libc::c_char,
                                                        1 as libc::c_int as size_t,
                                                    );
                                                    if !(1 as libc::c_int == tmp___7) {
                                                        current_block = 8149651411747124477;
                                                        break;
                                                    }
                                                    len += 1;
                                                    if !(len > 4 as libc::c_int) {
                                                        continue;
                                                    }
                                                    if !(buff[(len - 1 as libc::c_int) as usize] as libc::c_int
                                                        == 10 as libc::c_int)
                                                    {
                                                        continue;
                                                    }
                                                    if !(buff[(len - 2 as libc::c_int) as usize] as libc::c_int
                                                        == 13 as libc::c_int)
                                                    {
                                                        continue;
                                                    }
                                                    if !(buff[(len - 3 as libc::c_int) as usize] as libc::c_int
                                                        == 10 as libc::c_int)
                                                    {
                                                        continue;
                                                    }
                                                    if buff[(len - 4 as libc::c_int) as usize] as libc::c_int
                                                        == 13 as libc::c_int
                                                    {
                                                        current_block = 15587532755333643506;
                                                        break;
                                                    }
                                                }
                                                match current_block {
                                                    8149651411747124477 => {}
                                                    _ => {
                                                        if !(len == 1024 as libc::c_int) {
                                                            if buff[9 as libc::c_int as usize] as libc::c_int
                                                                == 50 as libc::c_int
                                                            {
                                                                if buff[10 as libc::c_int as usize] as libc::c_int
                                                                    == 48 as libc::c_int
                                                                {
                                                                    if buff[11 as libc::c_int as usize] as libc::c_int
                                                                        == 48 as libc::c_int
                                                                    {
                                                                        return 0 as libc::c_int;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        return 5 as libc::c_int;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {
                                if ulen != 0 {
                                    tmp___10 = 2 as libc::c_int;
                                } else {
                                    tmp___10 = 1 as libc::c_int;
                                }
                                n_methods = tmp___10;
                                buff[0 as libc::c_int
                                    as usize] = 5 as libc::c_int as libc::c_uchar;
                                buff[1 as libc::c_int
                                    as usize] = n_methods as libc::c_uchar;
                                buff[2 as libc::c_int
                                    as usize] = 0 as libc::c_int as libc::c_uchar;
                                if ulen != 0 {
                                    buff[3 as libc::c_int
                                        as usize] = 2 as libc::c_int as libc::c_uchar;
                                }
                                tmp___11 = write_n_bytes(
                                    sock,
                                    buff.as_mut_ptr() as *mut libc::c_char,
                                    (2 as libc::c_int + n_methods) as size_t,
                                );
                                if !(2 as libc::c_int + n_methods != tmp___11) {
                                    tmp___12 = read_n_bytes(
                                        sock,
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        2 as libc::c_int as size_t,
                                    );
                                    if !(2 as libc::c_int != tmp___12) {
                                        if buff[0 as libc::c_int as usize] as libc::c_int
                                            != 5 as libc::c_int
                                        {
                                            current_block = 10418296791801432365;
                                        } else {
                                            if buff[1 as libc::c_int as usize] as libc::c_int
                                                != 0 as libc::c_int
                                            {
                                                if buff[1 as libc::c_int as usize] as libc::c_int
                                                    != 2 as libc::c_int
                                                {
                                                    current_block = 10418296791801432365;
                                                } else {
                                                    current_block = 5219368551394180541;
                                                }
                                            } else {
                                                current_block = 5219368551394180541;
                                            }
                                            match current_block {
                                                10418296791801432365 => {}
                                                _ => {
                                                    if buff[1 as libc::c_int as usize] as libc::c_int
                                                        == 2 as libc::c_int
                                                    {
                                                        cur = out.as_mut_ptr();
                                                        tmp___13 = cur;
                                                        cur = cur.offset(1);
                                                        *tmp___13 = 1 as libc::c_int as libc::c_char;
                                                        c = ulen & 255 as libc::c_ulong;
                                                        tmp___14 = cur;
                                                        cur = cur.offset(1);
                                                        *tmp___14 = c as libc::c_char;
                                                        memcpy(
                                                            cur as *mut libc::c_void,
                                                            user as *const libc::c_void,
                                                            c,
                                                        );
                                                        cur = cur.offset(c as isize);
                                                        c = passlen & 255 as libc::c_ulong;
                                                        tmp___15 = cur;
                                                        cur = cur.offset(1);
                                                        *tmp___15 = c as libc::c_char;
                                                        memcpy(
                                                            cur as *mut libc::c_void,
                                                            pass as *const libc::c_void,
                                                            c,
                                                        );
                                                        cur = cur.offset(c as isize);
                                                        tmp___16 = write_n_bytes(
                                                            sock,
                                                            out.as_mut_ptr(),
                                                            cur.offset_from(out.as_mut_ptr()) as libc::c_long as size_t,
                                                        );
                                                        if cur.offset_from(out.as_mut_ptr()) as libc::c_long
                                                            != tmp___16 as libc::c_long
                                                        {
                                                            current_block = 8149651411747124477;
                                                        } else {
                                                            tmp___17 = read_n_bytes(
                                                                sock,
                                                                in_0.as_mut_ptr(),
                                                                2 as libc::c_int as size_t,
                                                            );
                                                            if 2 as libc::c_int != tmp___17 {
                                                                current_block = 8149651411747124477;
                                                            } else {
                                                                if !(in_0[0 as libc::c_int as usize] as libc::c_int
                                                                    == 5 as libc::c_int)
                                                                {
                                                                    if !(in_0[0 as libc::c_int as usize] as libc::c_int
                                                                        == 1 as libc::c_int)
                                                                    {
                                                                        current_block = 8149651411747124477;
                                                                    } else {
                                                                        current_block = 6662862405959679103;
                                                                    }
                                                                } else {
                                                                    current_block = 6662862405959679103;
                                                                }
                                                                match current_block {
                                                                    8149651411747124477 => {}
                                                                    _ => {
                                                                        if in_0[1 as libc::c_int as usize] as libc::c_int
                                                                            != 0 as libc::c_int
                                                                        {
                                                                            return 5 as libc::c_int;
                                                                        }
                                                                        current_block = 7371321987304394147;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        current_block = 7371321987304394147;
                                                    }
                                                    match current_block {
                                                        8149651411747124477 => {}
                                                        _ => {
                                                            buff_iter = 0 as libc::c_int;
                                                            tmp___18 = buff_iter;
                                                            buff_iter += 1;
                                                            buff[tmp___18 as usize] = 5 as libc::c_int as libc::c_uchar;
                                                            tmp___19 = buff_iter;
                                                            buff_iter += 1;
                                                            buff[tmp___19 as usize] = 1 as libc::c_int as libc::c_uchar;
                                                            tmp___20 = buff_iter;
                                                            buff_iter += 1;
                                                            buff[tmp___20 as usize] = 0 as libc::c_int as libc::c_uchar;
                                                            if dns_len == 0 {
                                                                tmp___21 = buff_iter;
                                                                buff_iter += 1;
                                                                if v6 != 0 {
                                                                    buff[tmp___21 as usize] = 4 as libc::c_int as libc::c_uchar;
                                                                } else {
                                                                    buff[tmp___21 as usize] = 1 as libc::c_int as libc::c_uchar;
                                                                }
                                                                if v6 != 0 {
                                                                    tmp___22 = 16 as libc::c_int;
                                                                } else {
                                                                    tmp___22 = 4 as libc::c_int;
                                                                }
                                                                memcpy(
                                                                    buff.as_mut_ptr().offset(buff_iter as isize)
                                                                        as *mut libc::c_void,
                                                                    (ip.addr.v6).as_mut_ptr() as *const libc::c_void,
                                                                    tmp___22 as size_t,
                                                                );
                                                                if v6 != 0 {
                                                                    tmp___23 = 16 as libc::c_int;
                                                                } else {
                                                                    tmp___23 = 4 as libc::c_int;
                                                                }
                                                                buff_iter += tmp___23;
                                                            } else {
                                                                tmp___24 = buff_iter;
                                                                buff_iter += 1;
                                                                buff[tmp___24 as usize] = 3 as libc::c_int as libc::c_uchar;
                                                                tmp___25 = buff_iter;
                                                                buff_iter += 1;
                                                                buff[tmp___25
                                                                    as usize] = (dns_len & 255 as libc::c_ulong)
                                                                    as libc::c_uchar;
                                                                memcpy(
                                                                    buff.as_mut_ptr().offset(buff_iter as isize)
                                                                        as *mut libc::c_void,
                                                                    dns_name as *const libc::c_void,
                                                                    dns_len,
                                                                );
                                                                buff_iter = (buff_iter as size_t).wrapping_add(dns_len)
                                                                    as libc::c_int;
                                                            }
                                                            memcpy(
                                                                buff.as_mut_ptr().offset(buff_iter as isize)
                                                                    as *mut libc::c_void,
                                                                &mut port as *mut libc::c_ushort as *const libc::c_void,
                                                                2 as libc::c_int as size_t,
                                                            );
                                                            buff_iter += 2 as libc::c_int;
                                                            tmp___26 = write_n_bytes(
                                                                sock,
                                                                buff.as_mut_ptr() as *mut libc::c_char,
                                                                buff_iter as size_t,
                                                            );
                                                            if buff_iter != tmp___26 {
                                                                current_block = 8149651411747124477;
                                                            } else {
                                                                tmp___27 = read_n_bytes(
                                                                    sock,
                                                                    buff.as_mut_ptr() as *mut libc::c_char,
                                                                    4 as libc::c_int as size_t,
                                                                );
                                                                if 4 as libc::c_int != tmp___27 {
                                                                    current_block = 8149651411747124477;
                                                                } else if buff[0 as libc::c_int as usize] as libc::c_int
                                                                        != 5 as libc::c_int
                                                                    {
                                                                    current_block = 8149651411747124477;
                                                                } else if buff[1 as libc::c_int as usize] as libc::c_int
                                                                        != 0 as libc::c_int
                                                                    {
                                                                    current_block = 8149651411747124477;
                                                                } else {
                                                                    match buff[3 as libc::c_int as usize] as libc::c_int {
                                                                        1 => {
                                                                            current_block = 5117978787054293485;
                                                                            match current_block {
                                                                                1683520436886296249 => {
                                                                                    len = 16 as libc::c_int;
                                                                                    current_block = 6930811285952240378;
                                                                                }
                                                                                5117978787054293485 => {
                                                                                    len = 4 as libc::c_int;
                                                                                    current_block = 6930811285952240378;
                                                                                }
                                                                                _ => {
                                                                                    len = 0 as libc::c_int;
                                                                                    tmp___28 = read_n_bytes(
                                                                                        sock,
                                                                                        &mut len as *mut libc::c_int as *mut libc::c_char,
                                                                                        1 as libc::c_int as size_t,
                                                                                    );
                                                                                    if 1 as libc::c_int != tmp___28 {
                                                                                        current_block = 8149651411747124477;
                                                                                    } else {
                                                                                        current_block = 6930811285952240378;
                                                                                    }
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                8149651411747124477 => {}
                                                                                _ => {
                                                                                    tmp___29 = read_n_bytes(
                                                                                        sock,
                                                                                        buff.as_mut_ptr() as *mut libc::c_char,
                                                                                        (len + 2 as libc::c_int) as size_t,
                                                                                    );
                                                                                    if len + 2 as libc::c_int != tmp___29 {
                                                                                        current_block = 8149651411747124477;
                                                                                    } else {
                                                                                        return 0 as libc::c_int
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        4 => {
                                                                            current_block = 1683520436886296249;
                                                                            match current_block {
                                                                                1683520436886296249 => {
                                                                                    len = 16 as libc::c_int;
                                                                                    current_block = 6930811285952240378;
                                                                                }
                                                                                5117978787054293485 => {
                                                                                    len = 4 as libc::c_int;
                                                                                    current_block = 6930811285952240378;
                                                                                }
                                                                                _ => {
                                                                                    len = 0 as libc::c_int;
                                                                                    tmp___28 = read_n_bytes(
                                                                                        sock,
                                                                                        &mut len as *mut libc::c_int as *mut libc::c_char,
                                                                                        1 as libc::c_int as size_t,
                                                                                    );
                                                                                    if 1 as libc::c_int != tmp___28 {
                                                                                        current_block = 8149651411747124477;
                                                                                    } else {
                                                                                        current_block = 6930811285952240378;
                                                                                    }
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                8149651411747124477 => {}
                                                                                _ => {
                                                                                    tmp___29 = read_n_bytes(
                                                                                        sock,
                                                                                        buff.as_mut_ptr() as *mut libc::c_char,
                                                                                        (len + 2 as libc::c_int) as size_t,
                                                                                    );
                                                                                    if len + 2 as libc::c_int != tmp___29 {
                                                                                        current_block = 8149651411747124477;
                                                                                    } else {
                                                                                        return 0 as libc::c_int
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        3 => {
                                                                            current_block = 5428223447338481681;
                                                                            match current_block {
                                                                                1683520436886296249 => {
                                                                                    len = 16 as libc::c_int;
                                                                                    current_block = 6930811285952240378;
                                                                                }
                                                                                5117978787054293485 => {
                                                                                    len = 4 as libc::c_int;
                                                                                    current_block = 6930811285952240378;
                                                                                }
                                                                                _ => {
                                                                                    len = 0 as libc::c_int;
                                                                                    tmp___28 = read_n_bytes(
                                                                                        sock,
                                                                                        &mut len as *mut libc::c_int as *mut libc::c_char,
                                                                                        1 as libc::c_int as size_t,
                                                                                    );
                                                                                    if 1 as libc::c_int != tmp___28 {
                                                                                        current_block = 8149651411747124477;
                                                                                    } else {
                                                                                        current_block = 6930811285952240378;
                                                                                    }
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                8149651411747124477 => {}
                                                                                _ => {
                                                                                    tmp___29 = read_n_bytes(
                                                                                        sock,
                                                                                        buff.as_mut_ptr() as *mut libc::c_char,
                                                                                        (len + 2 as libc::c_int) as size_t,
                                                                                    );
                                                                                    if len + 2 as libc::c_int != tmp___29 {
                                                                                        current_block = 8149651411747124477;
                                                                                    } else {
                                                                                        return 0 as libc::c_int
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        _ => {
                                                                            current_block = 8149651411747124477;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        match current_block {
                                            8149651411747124477 => {}
                                            _ => {
                                                if buff[0 as libc::c_int as usize] as libc::c_int
                                                    == 5 as libc::c_int
                                                {
                                                    if buff[1 as libc::c_int as usize] as libc::c_int
                                                        == 255 as libc::c_int
                                                    {
                                                        return 5 as libc::c_int;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    1 => {
                        current_block = 14004463445576069816;
                        match current_block {
                            2069327002525158974 => return 0 as libc::c_int,
                            14004463445576069816 => {
                                if v6 != 0 {
                                    proxychains_write_log(
                                        b"[proxychains] error: SOCKS4 doesn't support ipv6 addresses\n\0"
                                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                                    );
                                } else {
                                    buff[0 as libc::c_int
                                        as usize] = 4 as libc::c_int as libc::c_uchar;
                                    buff[1 as libc::c_int
                                        as usize] = 1 as libc::c_int as libc::c_uchar;
                                    memcpy(
                                        &mut *buff.as_mut_ptr().offset(2 as libc::c_int as isize)
                                            as *mut libc::c_uchar as *mut libc::c_void,
                                        &mut port as *mut libc::c_ushort as *const libc::c_void,
                                        2 as libc::c_int as size_t,
                                    );
                                    if dns_len != 0 {
                                        ip
                                            .addr
                                            .v4
                                            .octet[0 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                        ip
                                            .addr
                                            .v4
                                            .octet[1 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                        ip
                                            .addr
                                            .v4
                                            .octet[2 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                        ip
                                            .addr
                                            .v4
                                            .octet[3 as libc::c_int
                                            as usize] = 1 as libc::c_int as libc::c_uchar;
                                    }
                                    memcpy(
                                        &mut *buff.as_mut_ptr().offset(4 as libc::c_int as isize)
                                            as *mut libc::c_uchar as *mut libc::c_void,
                                        &mut ip.addr.v4 as *mut ip_type4 as *const libc::c_void,
                                        4 as libc::c_int as size_t,
                                    );
                                    len = ulen.wrapping_add(1 as libc::c_ulong) as libc::c_int;
                                    if len > 1 as libc::c_int {
                                        memcpy(
                                            &mut *buff.as_mut_ptr().offset(8 as libc::c_int as isize)
                                                as *mut libc::c_uchar as *mut libc::c_void,
                                            user as *const libc::c_void,
                                            len as size_t,
                                        );
                                    } else {
                                        buff[8 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                    }
                                    if dns_len != 0 {
                                        memcpy(
                                            &mut *buff
                                                .as_mut_ptr()
                                                .offset((8 as libc::c_int + len) as isize)
                                                as *mut libc::c_uchar as *mut libc::c_void,
                                            dns_name as *const libc::c_void,
                                            dns_len.wrapping_add(1 as libc::c_ulong),
                                        );
                                        len = (len as size_t)
                                            .wrapping_add(dns_len.wrapping_add(1 as libc::c_ulong))
                                            as libc::c_int;
                                    }
                                    tmp___8 = write_n_bytes(
                                        sock,
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        (8 as libc::c_int + len) as size_t,
                                    );
                                    if !(len + 8 as libc::c_int != tmp___8) {
                                        tmp___9 = read_n_bytes(
                                            sock,
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            8 as libc::c_int as size_t,
                                        );
                                        if !(8 as libc::c_int != tmp___9) {
                                            if buff[0 as libc::c_int as usize] as libc::c_int
                                                != 0 as libc::c_int
                                            {
                                                return 5 as libc::c_int
                                            } else {
                                                if buff[1 as libc::c_int as usize] as libc::c_int
                                                    != 90 as libc::c_int
                                                {
                                                    return 5 as libc::c_int;
                                                }
                                            }
                                            return 0 as libc::c_int;
                                        }
                                    }
                                }
                            }
                            11638966953701390245 => {
                                if dns_len == 0 {
                                    if v6 != 0 {
                                        tmp___1 = 10 as libc::c_int;
                                    } else {
                                        tmp___1 = 2 as libc::c_int;
                                    }
                                    tmp___2 = inet_ntop(
                                        tmp___1,
                                        (ip.addr.v6).as_mut_ptr() as *const libc::c_void,
                                        ip_buf.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong
                                            as socklen_t,
                                    );
                                    if tmp___2.is_null() {
                                        proxychains_write_log(
                                            b"[proxychains] error: ip address conversion failed\n\0"
                                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                                        );
                                        current_block = 8149651411747124477;
                                    } else {
                                        dns_name = ip_buf.as_mut_ptr();
                                        current_block = 7018308795614528254;
                                    }
                                } else {
                                    current_block = 7018308795614528254;
                                }
                                match current_block {
                                    8149651411747124477 => {}
                                    _ => {
                                        if ulen != 0 {
                                            snprintf(
                                                src.as_mut_ptr(),
                                                ::std::mem::size_of::<[libc::c_char; 512]>()
                                                    as libc::c_ulong,
                                                b"%s:%s\0" as *const u8 as *const libc::c_char,
                                                user,
                                                pass,
                                            );
                                            encode_base_64(
                                                src.as_mut_ptr(),
                                                dst.as_mut_ptr(),
                                                ::std::mem::size_of::<[libc::c_char; 2048]>()
                                                    as libc::c_ulong as libc::c_int,
                                            );
                                        } else {
                                            dst[0 as libc::c_int
                                                as usize] = 0 as libc::c_int as libc::c_char;
                                        }
                                        tmp___3 = ntohs(port);
                                        hs_port = tmp___3;
                                        if ulen != 0 {
                                            tmp___4 = b"\r\n\0" as *const u8 as *const libc::c_char;
                                        } else {
                                            tmp___4 = dst.as_mut_ptr() as *const libc::c_char;
                                        }
                                        if ulen != 0 {
                                            tmp___5 = b"Proxy-Authorization: Basic \0" as *const u8
                                                as *const libc::c_char;
                                        } else {
                                            tmp___5 = dst.as_mut_ptr() as *const libc::c_char;
                                        }
                                        len = snprintf(
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            ::std::mem::size_of::<[libc::c_uchar; 1024]>()
                                                as libc::c_ulong,
                                            b"CONNECT %s:%d HTTP/1.0\r\nHost: %s:%d\r\n%s%s%s\r\n\0"
                                                as *const u8 as *const libc::c_char,
                                            dns_name,
                                            hs_port as libc::c_int,
                                            dns_name,
                                            hs_port as libc::c_int,
                                            tmp___5,
                                            dst.as_mut_ptr(),
                                            tmp___4,
                                        );
                                        if !(len < 0 as libc::c_int) {
                                            tmp___6 = send(
                                                sock,
                                                buff.as_mut_ptr() as *const libc::c_void,
                                                len as size_t,
                                                0 as libc::c_int,
                                            );
                                            if !(len as ssize_t != tmp___6) {
                                                len = 0 as libc::c_int;
                                                loop {
                                                    if !(len < 1024 as libc::c_int) {
                                                        current_block = 15587532755333643506;
                                                        break;
                                                    }
                                                    tmp___7 = read_n_bytes(
                                                        sock,
                                                        buff.as_mut_ptr().offset(len as isize) as *mut libc::c_char,
                                                        1 as libc::c_int as size_t,
                                                    );
                                                    if !(1 as libc::c_int == tmp___7) {
                                                        current_block = 8149651411747124477;
                                                        break;
                                                    }
                                                    len += 1;
                                                    if !(len > 4 as libc::c_int) {
                                                        continue;
                                                    }
                                                    if !(buff[(len - 1 as libc::c_int) as usize] as libc::c_int
                                                        == 10 as libc::c_int)
                                                    {
                                                        continue;
                                                    }
                                                    if !(buff[(len - 2 as libc::c_int) as usize] as libc::c_int
                                                        == 13 as libc::c_int)
                                                    {
                                                        continue;
                                                    }
                                                    if !(buff[(len - 3 as libc::c_int) as usize] as libc::c_int
                                                        == 10 as libc::c_int)
                                                    {
                                                        continue;
                                                    }
                                                    if buff[(len - 4 as libc::c_int) as usize] as libc::c_int
                                                        == 13 as libc::c_int
                                                    {
                                                        current_block = 15587532755333643506;
                                                        break;
                                                    }
                                                }
                                                match current_block {
                                                    8149651411747124477 => {}
                                                    _ => {
                                                        if !(len == 1024 as libc::c_int) {
                                                            if buff[9 as libc::c_int as usize] as libc::c_int
                                                                == 50 as libc::c_int
                                                            {
                                                                if buff[10 as libc::c_int as usize] as libc::c_int
                                                                    == 48 as libc::c_int
                                                                {
                                                                    if buff[11 as libc::c_int as usize] as libc::c_int
                                                                        == 48 as libc::c_int
                                                                    {
                                                                        return 0 as libc::c_int;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        return 5 as libc::c_int;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {
                                if ulen != 0 {
                                    tmp___10 = 2 as libc::c_int;
                                } else {
                                    tmp___10 = 1 as libc::c_int;
                                }
                                n_methods = tmp___10;
                                buff[0 as libc::c_int
                                    as usize] = 5 as libc::c_int as libc::c_uchar;
                                buff[1 as libc::c_int
                                    as usize] = n_methods as libc::c_uchar;
                                buff[2 as libc::c_int
                                    as usize] = 0 as libc::c_int as libc::c_uchar;
                                if ulen != 0 {
                                    buff[3 as libc::c_int
                                        as usize] = 2 as libc::c_int as libc::c_uchar;
                                }
                                tmp___11 = write_n_bytes(
                                    sock,
                                    buff.as_mut_ptr() as *mut libc::c_char,
                                    (2 as libc::c_int + n_methods) as size_t,
                                );
                                if !(2 as libc::c_int + n_methods != tmp___11) {
                                    tmp___12 = read_n_bytes(
                                        sock,
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        2 as libc::c_int as size_t,
                                    );
                                    if !(2 as libc::c_int != tmp___12) {
                                        if buff[0 as libc::c_int as usize] as libc::c_int
                                            != 5 as libc::c_int
                                        {
                                            current_block = 10418296791801432365;
                                        } else {
                                            if buff[1 as libc::c_int as usize] as libc::c_int
                                                != 0 as libc::c_int
                                            {
                                                if buff[1 as libc::c_int as usize] as libc::c_int
                                                    != 2 as libc::c_int
                                                {
                                                    current_block = 10418296791801432365;
                                                } else {
                                                    current_block = 5219368551394180541;
                                                }
                                            } else {
                                                current_block = 5219368551394180541;
                                            }
                                            match current_block {
                                                10418296791801432365 => {}
                                                _ => {
                                                    if buff[1 as libc::c_int as usize] as libc::c_int
                                                        == 2 as libc::c_int
                                                    {
                                                        cur = out.as_mut_ptr();
                                                        tmp___13 = cur;
                                                        cur = cur.offset(1);
                                                        *tmp___13 = 1 as libc::c_int as libc::c_char;
                                                        c = ulen & 255 as libc::c_ulong;
                                                        tmp___14 = cur;
                                                        cur = cur.offset(1);
                                                        *tmp___14 = c as libc::c_char;
                                                        memcpy(
                                                            cur as *mut libc::c_void,
                                                            user as *const libc::c_void,
                                                            c,
                                                        );
                                                        cur = cur.offset(c as isize);
                                                        c = passlen & 255 as libc::c_ulong;
                                                        tmp___15 = cur;
                                                        cur = cur.offset(1);
                                                        *tmp___15 = c as libc::c_char;
                                                        memcpy(
                                                            cur as *mut libc::c_void,
                                                            pass as *const libc::c_void,
                                                            c,
                                                        );
                                                        cur = cur.offset(c as isize);
                                                        tmp___16 = write_n_bytes(
                                                            sock,
                                                            out.as_mut_ptr(),
                                                            cur.offset_from(out.as_mut_ptr()) as libc::c_long as size_t,
                                                        );
                                                        if cur.offset_from(out.as_mut_ptr()) as libc::c_long
                                                            != tmp___16 as libc::c_long
                                                        {
                                                            current_block = 8149651411747124477;
                                                        } else {
                                                            tmp___17 = read_n_bytes(
                                                                sock,
                                                                in_0.as_mut_ptr(),
                                                                2 as libc::c_int as size_t,
                                                            );
                                                            if 2 as libc::c_int != tmp___17 {
                                                                current_block = 8149651411747124477;
                                                            } else {
                                                                if !(in_0[0 as libc::c_int as usize] as libc::c_int
                                                                    == 5 as libc::c_int)
                                                                {
                                                                    if !(in_0[0 as libc::c_int as usize] as libc::c_int
                                                                        == 1 as libc::c_int)
                                                                    {
                                                                        current_block = 8149651411747124477;
                                                                    } else {
                                                                        current_block = 6662862405959679103;
                                                                    }
                                                                } else {
                                                                    current_block = 6662862405959679103;
                                                                }
                                                                match current_block {
                                                                    8149651411747124477 => {}
                                                                    _ => {
                                                                        if in_0[1 as libc::c_int as usize] as libc::c_int
                                                                            != 0 as libc::c_int
                                                                        {
                                                                            return 5 as libc::c_int;
                                                                        }
                                                                        current_block = 7371321987304394147;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        current_block = 7371321987304394147;
                                                    }
                                                    match current_block {
                                                        8149651411747124477 => {}
                                                        _ => {
                                                            buff_iter = 0 as libc::c_int;
                                                            tmp___18 = buff_iter;
                                                            buff_iter += 1;
                                                            buff[tmp___18 as usize] = 5 as libc::c_int as libc::c_uchar;
                                                            tmp___19 = buff_iter;
                                                            buff_iter += 1;
                                                            buff[tmp___19 as usize] = 1 as libc::c_int as libc::c_uchar;
                                                            tmp___20 = buff_iter;
                                                            buff_iter += 1;
                                                            buff[tmp___20 as usize] = 0 as libc::c_int as libc::c_uchar;
                                                            if dns_len == 0 {
                                                                tmp___21 = buff_iter;
                                                                buff_iter += 1;
                                                                if v6 != 0 {
                                                                    buff[tmp___21 as usize] = 4 as libc::c_int as libc::c_uchar;
                                                                } else {
                                                                    buff[tmp___21 as usize] = 1 as libc::c_int as libc::c_uchar;
                                                                }
                                                                if v6 != 0 {
                                                                    tmp___22 = 16 as libc::c_int;
                                                                } else {
                                                                    tmp___22 = 4 as libc::c_int;
                                                                }
                                                                memcpy(
                                                                    buff.as_mut_ptr().offset(buff_iter as isize)
                                                                        as *mut libc::c_void,
                                                                    (ip.addr.v6).as_mut_ptr() as *const libc::c_void,
                                                                    tmp___22 as size_t,
                                                                );
                                                                if v6 != 0 {
                                                                    tmp___23 = 16 as libc::c_int;
                                                                } else {
                                                                    tmp___23 = 4 as libc::c_int;
                                                                }
                                                                buff_iter += tmp___23;
                                                            } else {
                                                                tmp___24 = buff_iter;
                                                                buff_iter += 1;
                                                                buff[tmp___24 as usize] = 3 as libc::c_int as libc::c_uchar;
                                                                tmp___25 = buff_iter;
                                                                buff_iter += 1;
                                                                buff[tmp___25
                                                                    as usize] = (dns_len & 255 as libc::c_ulong)
                                                                    as libc::c_uchar;
                                                                memcpy(
                                                                    buff.as_mut_ptr().offset(buff_iter as isize)
                                                                        as *mut libc::c_void,
                                                                    dns_name as *const libc::c_void,
                                                                    dns_len,
                                                                );
                                                                buff_iter = (buff_iter as size_t).wrapping_add(dns_len)
                                                                    as libc::c_int;
                                                            }
                                                            memcpy(
                                                                buff.as_mut_ptr().offset(buff_iter as isize)
                                                                    as *mut libc::c_void,
                                                                &mut port as *mut libc::c_ushort as *const libc::c_void,
                                                                2 as libc::c_int as size_t,
                                                            );
                                                            buff_iter += 2 as libc::c_int;
                                                            tmp___26 = write_n_bytes(
                                                                sock,
                                                                buff.as_mut_ptr() as *mut libc::c_char,
                                                                buff_iter as size_t,
                                                            );
                                                            if buff_iter != tmp___26 {
                                                                current_block = 8149651411747124477;
                                                            } else {
                                                                tmp___27 = read_n_bytes(
                                                                    sock,
                                                                    buff.as_mut_ptr() as *mut libc::c_char,
                                                                    4 as libc::c_int as size_t,
                                                                );
                                                                if 4 as libc::c_int != tmp___27 {
                                                                    current_block = 8149651411747124477;
                                                                } else if buff[0 as libc::c_int as usize] as libc::c_int
                                                                        != 5 as libc::c_int
                                                                    {
                                                                    current_block = 8149651411747124477;
                                                                } else if buff[1 as libc::c_int as usize] as libc::c_int
                                                                        != 0 as libc::c_int
                                                                    {
                                                                    current_block = 8149651411747124477;
                                                                } else {
                                                                    match buff[3 as libc::c_int as usize] as libc::c_int {
                                                                        1 => {
                                                                            current_block = 5117978787054293485;
                                                                            match current_block {
                                                                                1683520436886296249 => {
                                                                                    len = 16 as libc::c_int;
                                                                                    current_block = 6930811285952240378;
                                                                                }
                                                                                5117978787054293485 => {
                                                                                    len = 4 as libc::c_int;
                                                                                    current_block = 6930811285952240378;
                                                                                }
                                                                                _ => {
                                                                                    len = 0 as libc::c_int;
                                                                                    tmp___28 = read_n_bytes(
                                                                                        sock,
                                                                                        &mut len as *mut libc::c_int as *mut libc::c_char,
                                                                                        1 as libc::c_int as size_t,
                                                                                    );
                                                                                    if 1 as libc::c_int != tmp___28 {
                                                                                        current_block = 8149651411747124477;
                                                                                    } else {
                                                                                        current_block = 6930811285952240378;
                                                                                    }
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                8149651411747124477 => {}
                                                                                _ => {
                                                                                    tmp___29 = read_n_bytes(
                                                                                        sock,
                                                                                        buff.as_mut_ptr() as *mut libc::c_char,
                                                                                        (len + 2 as libc::c_int) as size_t,
                                                                                    );
                                                                                    if len + 2 as libc::c_int != tmp___29 {
                                                                                        current_block = 8149651411747124477;
                                                                                    } else {
                                                                                        return 0 as libc::c_int
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        4 => {
                                                                            current_block = 1683520436886296249;
                                                                            match current_block {
                                                                                1683520436886296249 => {
                                                                                    len = 16 as libc::c_int;
                                                                                    current_block = 6930811285952240378;
                                                                                }
                                                                                5117978787054293485 => {
                                                                                    len = 4 as libc::c_int;
                                                                                    current_block = 6930811285952240378;
                                                                                }
                                                                                _ => {
                                                                                    len = 0 as libc::c_int;
                                                                                    tmp___28 = read_n_bytes(
                                                                                        sock,
                                                                                        &mut len as *mut libc::c_int as *mut libc::c_char,
                                                                                        1 as libc::c_int as size_t,
                                                                                    );
                                                                                    if 1 as libc::c_int != tmp___28 {
                                                                                        current_block = 8149651411747124477;
                                                                                    } else {
                                                                                        current_block = 6930811285952240378;
                                                                                    }
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                8149651411747124477 => {}
                                                                                _ => {
                                                                                    tmp___29 = read_n_bytes(
                                                                                        sock,
                                                                                        buff.as_mut_ptr() as *mut libc::c_char,
                                                                                        (len + 2 as libc::c_int) as size_t,
                                                                                    );
                                                                                    if len + 2 as libc::c_int != tmp___29 {
                                                                                        current_block = 8149651411747124477;
                                                                                    } else {
                                                                                        return 0 as libc::c_int
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        3 => {
                                                                            current_block = 5428223447338481681;
                                                                            match current_block {
                                                                                1683520436886296249 => {
                                                                                    len = 16 as libc::c_int;
                                                                                    current_block = 6930811285952240378;
                                                                                }
                                                                                5117978787054293485 => {
                                                                                    len = 4 as libc::c_int;
                                                                                    current_block = 6930811285952240378;
                                                                                }
                                                                                _ => {
                                                                                    len = 0 as libc::c_int;
                                                                                    tmp___28 = read_n_bytes(
                                                                                        sock,
                                                                                        &mut len as *mut libc::c_int as *mut libc::c_char,
                                                                                        1 as libc::c_int as size_t,
                                                                                    );
                                                                                    if 1 as libc::c_int != tmp___28 {
                                                                                        current_block = 8149651411747124477;
                                                                                    } else {
                                                                                        current_block = 6930811285952240378;
                                                                                    }
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                8149651411747124477 => {}
                                                                                _ => {
                                                                                    tmp___29 = read_n_bytes(
                                                                                        sock,
                                                                                        buff.as_mut_ptr() as *mut libc::c_char,
                                                                                        (len + 2 as libc::c_int) as size_t,
                                                                                    );
                                                                                    if len + 2 as libc::c_int != tmp___29 {
                                                                                        current_block = 8149651411747124477;
                                                                                    } else {
                                                                                        return 0 as libc::c_int
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        _ => {
                                                                            current_block = 8149651411747124477;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        match current_block {
                                            8149651411747124477 => {}
                                            _ => {
                                                if buff[0 as libc::c_int as usize] as libc::c_int
                                                    == 5 as libc::c_int
                                                {
                                                    if buff[1 as libc::c_int as usize] as libc::c_int
                                                        == 255 as libc::c_int
                                                    {
                                                        return 5 as libc::c_int;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    2 => {
                        current_block = 3317322622165405134;
                        match current_block {
                            2069327002525158974 => return 0 as libc::c_int,
                            14004463445576069816 => {
                                if v6 != 0 {
                                    proxychains_write_log(
                                        b"[proxychains] error: SOCKS4 doesn't support ipv6 addresses\n\0"
                                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                                    );
                                } else {
                                    buff[0 as libc::c_int
                                        as usize] = 4 as libc::c_int as libc::c_uchar;
                                    buff[1 as libc::c_int
                                        as usize] = 1 as libc::c_int as libc::c_uchar;
                                    memcpy(
                                        &mut *buff.as_mut_ptr().offset(2 as libc::c_int as isize)
                                            as *mut libc::c_uchar as *mut libc::c_void,
                                        &mut port as *mut libc::c_ushort as *const libc::c_void,
                                        2 as libc::c_int as size_t,
                                    );
                                    if dns_len != 0 {
                                        ip
                                            .addr
                                            .v4
                                            .octet[0 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                        ip
                                            .addr
                                            .v4
                                            .octet[1 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                        ip
                                            .addr
                                            .v4
                                            .octet[2 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                        ip
                                            .addr
                                            .v4
                                            .octet[3 as libc::c_int
                                            as usize] = 1 as libc::c_int as libc::c_uchar;
                                    }
                                    memcpy(
                                        &mut *buff.as_mut_ptr().offset(4 as libc::c_int as isize)
                                            as *mut libc::c_uchar as *mut libc::c_void,
                                        &mut ip.addr.v4 as *mut ip_type4 as *const libc::c_void,
                                        4 as libc::c_int as size_t,
                                    );
                                    len = ulen.wrapping_add(1 as libc::c_ulong) as libc::c_int;
                                    if len > 1 as libc::c_int {
                                        memcpy(
                                            &mut *buff.as_mut_ptr().offset(8 as libc::c_int as isize)
                                                as *mut libc::c_uchar as *mut libc::c_void,
                                            user as *const libc::c_void,
                                            len as size_t,
                                        );
                                    } else {
                                        buff[8 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                    }
                                    if dns_len != 0 {
                                        memcpy(
                                            &mut *buff
                                                .as_mut_ptr()
                                                .offset((8 as libc::c_int + len) as isize)
                                                as *mut libc::c_uchar as *mut libc::c_void,
                                            dns_name as *const libc::c_void,
                                            dns_len.wrapping_add(1 as libc::c_ulong),
                                        );
                                        len = (len as size_t)
                                            .wrapping_add(dns_len.wrapping_add(1 as libc::c_ulong))
                                            as libc::c_int;
                                    }
                                    tmp___8 = write_n_bytes(
                                        sock,
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        (8 as libc::c_int + len) as size_t,
                                    );
                                    if !(len + 8 as libc::c_int != tmp___8) {
                                        tmp___9 = read_n_bytes(
                                            sock,
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            8 as libc::c_int as size_t,
                                        );
                                        if !(8 as libc::c_int != tmp___9) {
                                            if buff[0 as libc::c_int as usize] as libc::c_int
                                                != 0 as libc::c_int
                                            {
                                                return 5 as libc::c_int
                                            } else {
                                                if buff[1 as libc::c_int as usize] as libc::c_int
                                                    != 90 as libc::c_int
                                                {
                                                    return 5 as libc::c_int;
                                                }
                                            }
                                            return 0 as libc::c_int;
                                        }
                                    }
                                }
                            }
                            11638966953701390245 => {
                                if dns_len == 0 {
                                    if v6 != 0 {
                                        tmp___1 = 10 as libc::c_int;
                                    } else {
                                        tmp___1 = 2 as libc::c_int;
                                    }
                                    tmp___2 = inet_ntop(
                                        tmp___1,
                                        (ip.addr.v6).as_mut_ptr() as *const libc::c_void,
                                        ip_buf.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong
                                            as socklen_t,
                                    );
                                    if tmp___2.is_null() {
                                        proxychains_write_log(
                                            b"[proxychains] error: ip address conversion failed\n\0"
                                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                                        );
                                        current_block = 8149651411747124477;
                                    } else {
                                        dns_name = ip_buf.as_mut_ptr();
                                        current_block = 7018308795614528254;
                                    }
                                } else {
                                    current_block = 7018308795614528254;
                                }
                                match current_block {
                                    8149651411747124477 => {}
                                    _ => {
                                        if ulen != 0 {
                                            snprintf(
                                                src.as_mut_ptr(),
                                                ::std::mem::size_of::<[libc::c_char; 512]>()
                                                    as libc::c_ulong,
                                                b"%s:%s\0" as *const u8 as *const libc::c_char,
                                                user,
                                                pass,
                                            );
                                            encode_base_64(
                                                src.as_mut_ptr(),
                                                dst.as_mut_ptr(),
                                                ::std::mem::size_of::<[libc::c_char; 2048]>()
                                                    as libc::c_ulong as libc::c_int,
                                            );
                                        } else {
                                            dst[0 as libc::c_int
                                                as usize] = 0 as libc::c_int as libc::c_char;
                                        }
                                        tmp___3 = ntohs(port);
                                        hs_port = tmp___3;
                                        if ulen != 0 {
                                            tmp___4 = b"\r\n\0" as *const u8 as *const libc::c_char;
                                        } else {
                                            tmp___4 = dst.as_mut_ptr() as *const libc::c_char;
                                        }
                                        if ulen != 0 {
                                            tmp___5 = b"Proxy-Authorization: Basic \0" as *const u8
                                                as *const libc::c_char;
                                        } else {
                                            tmp___5 = dst.as_mut_ptr() as *const libc::c_char;
                                        }
                                        len = snprintf(
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            ::std::mem::size_of::<[libc::c_uchar; 1024]>()
                                                as libc::c_ulong,
                                            b"CONNECT %s:%d HTTP/1.0\r\nHost: %s:%d\r\n%s%s%s\r\n\0"
                                                as *const u8 as *const libc::c_char,
                                            dns_name,
                                            hs_port as libc::c_int,
                                            dns_name,
                                            hs_port as libc::c_int,
                                            tmp___5,
                                            dst.as_mut_ptr(),
                                            tmp___4,
                                        );
                                        if !(len < 0 as libc::c_int) {
                                            tmp___6 = send(
                                                sock,
                                                buff.as_mut_ptr() as *const libc::c_void,
                                                len as size_t,
                                                0 as libc::c_int,
                                            );
                                            if !(len as ssize_t != tmp___6) {
                                                len = 0 as libc::c_int;
                                                loop {
                                                    if !(len < 1024 as libc::c_int) {
                                                        current_block = 15587532755333643506;
                                                        break;
                                                    }
                                                    tmp___7 = read_n_bytes(
                                                        sock,
                                                        buff.as_mut_ptr().offset(len as isize) as *mut libc::c_char,
                                                        1 as libc::c_int as size_t,
                                                    );
                                                    if !(1 as libc::c_int == tmp___7) {
                                                        current_block = 8149651411747124477;
                                                        break;
                                                    }
                                                    len += 1;
                                                    if !(len > 4 as libc::c_int) {
                                                        continue;
                                                    }
                                                    if !(buff[(len - 1 as libc::c_int) as usize] as libc::c_int
                                                        == 10 as libc::c_int)
                                                    {
                                                        continue;
                                                    }
                                                    if !(buff[(len - 2 as libc::c_int) as usize] as libc::c_int
                                                        == 13 as libc::c_int)
                                                    {
                                                        continue;
                                                    }
                                                    if !(buff[(len - 3 as libc::c_int) as usize] as libc::c_int
                                                        == 10 as libc::c_int)
                                                    {
                                                        continue;
                                                    }
                                                    if buff[(len - 4 as libc::c_int) as usize] as libc::c_int
                                                        == 13 as libc::c_int
                                                    {
                                                        current_block = 15587532755333643506;
                                                        break;
                                                    }
                                                }
                                                match current_block {
                                                    8149651411747124477 => {}
                                                    _ => {
                                                        if !(len == 1024 as libc::c_int) {
                                                            if buff[9 as libc::c_int as usize] as libc::c_int
                                                                == 50 as libc::c_int
                                                            {
                                                                if buff[10 as libc::c_int as usize] as libc::c_int
                                                                    == 48 as libc::c_int
                                                                {
                                                                    if buff[11 as libc::c_int as usize] as libc::c_int
                                                                        == 48 as libc::c_int
                                                                    {
                                                                        return 0 as libc::c_int;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        return 5 as libc::c_int;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {
                                if ulen != 0 {
                                    tmp___10 = 2 as libc::c_int;
                                } else {
                                    tmp___10 = 1 as libc::c_int;
                                }
                                n_methods = tmp___10;
                                buff[0 as libc::c_int
                                    as usize] = 5 as libc::c_int as libc::c_uchar;
                                buff[1 as libc::c_int
                                    as usize] = n_methods as libc::c_uchar;
                                buff[2 as libc::c_int
                                    as usize] = 0 as libc::c_int as libc::c_uchar;
                                if ulen != 0 {
                                    buff[3 as libc::c_int
                                        as usize] = 2 as libc::c_int as libc::c_uchar;
                                }
                                tmp___11 = write_n_bytes(
                                    sock,
                                    buff.as_mut_ptr() as *mut libc::c_char,
                                    (2 as libc::c_int + n_methods) as size_t,
                                );
                                if !(2 as libc::c_int + n_methods != tmp___11) {
                                    tmp___12 = read_n_bytes(
                                        sock,
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        2 as libc::c_int as size_t,
                                    );
                                    if !(2 as libc::c_int != tmp___12) {
                                        if buff[0 as libc::c_int as usize] as libc::c_int
                                            != 5 as libc::c_int
                                        {
                                            current_block = 10418296791801432365;
                                        } else {
                                            if buff[1 as libc::c_int as usize] as libc::c_int
                                                != 0 as libc::c_int
                                            {
                                                if buff[1 as libc::c_int as usize] as libc::c_int
                                                    != 2 as libc::c_int
                                                {
                                                    current_block = 10418296791801432365;
                                                } else {
                                                    current_block = 5219368551394180541;
                                                }
                                            } else {
                                                current_block = 5219368551394180541;
                                            }
                                            match current_block {
                                                10418296791801432365 => {}
                                                _ => {
                                                    if buff[1 as libc::c_int as usize] as libc::c_int
                                                        == 2 as libc::c_int
                                                    {
                                                        cur = out.as_mut_ptr();
                                                        tmp___13 = cur;
                                                        cur = cur.offset(1);
                                                        *tmp___13 = 1 as libc::c_int as libc::c_char;
                                                        c = ulen & 255 as libc::c_ulong;
                                                        tmp___14 = cur;
                                                        cur = cur.offset(1);
                                                        *tmp___14 = c as libc::c_char;
                                                        memcpy(
                                                            cur as *mut libc::c_void,
                                                            user as *const libc::c_void,
                                                            c,
                                                        );
                                                        cur = cur.offset(c as isize);
                                                        c = passlen & 255 as libc::c_ulong;
                                                        tmp___15 = cur;
                                                        cur = cur.offset(1);
                                                        *tmp___15 = c as libc::c_char;
                                                        memcpy(
                                                            cur as *mut libc::c_void,
                                                            pass as *const libc::c_void,
                                                            c,
                                                        );
                                                        cur = cur.offset(c as isize);
                                                        tmp___16 = write_n_bytes(
                                                            sock,
                                                            out.as_mut_ptr(),
                                                            cur.offset_from(out.as_mut_ptr()) as libc::c_long as size_t,
                                                        );
                                                        if cur.offset_from(out.as_mut_ptr()) as libc::c_long
                                                            != tmp___16 as libc::c_long
                                                        {
                                                            current_block = 8149651411747124477;
                                                        } else {
                                                            tmp___17 = read_n_bytes(
                                                                sock,
                                                                in_0.as_mut_ptr(),
                                                                2 as libc::c_int as size_t,
                                                            );
                                                            if 2 as libc::c_int != tmp___17 {
                                                                current_block = 8149651411747124477;
                                                            } else {
                                                                if !(in_0[0 as libc::c_int as usize] as libc::c_int
                                                                    == 5 as libc::c_int)
                                                                {
                                                                    if !(in_0[0 as libc::c_int as usize] as libc::c_int
                                                                        == 1 as libc::c_int)
                                                                    {
                                                                        current_block = 8149651411747124477;
                                                                    } else {
                                                                        current_block = 6662862405959679103;
                                                                    }
                                                                } else {
                                                                    current_block = 6662862405959679103;
                                                                }
                                                                match current_block {
                                                                    8149651411747124477 => {}
                                                                    _ => {
                                                                        if in_0[1 as libc::c_int as usize] as libc::c_int
                                                                            != 0 as libc::c_int
                                                                        {
                                                                            return 5 as libc::c_int;
                                                                        }
                                                                        current_block = 7371321987304394147;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        current_block = 7371321987304394147;
                                                    }
                                                    match current_block {
                                                        8149651411747124477 => {}
                                                        _ => {
                                                            buff_iter = 0 as libc::c_int;
                                                            tmp___18 = buff_iter;
                                                            buff_iter += 1;
                                                            buff[tmp___18 as usize] = 5 as libc::c_int as libc::c_uchar;
                                                            tmp___19 = buff_iter;
                                                            buff_iter += 1;
                                                            buff[tmp___19 as usize] = 1 as libc::c_int as libc::c_uchar;
                                                            tmp___20 = buff_iter;
                                                            buff_iter += 1;
                                                            buff[tmp___20 as usize] = 0 as libc::c_int as libc::c_uchar;
                                                            if dns_len == 0 {
                                                                tmp___21 = buff_iter;
                                                                buff_iter += 1;
                                                                if v6 != 0 {
                                                                    buff[tmp___21 as usize] = 4 as libc::c_int as libc::c_uchar;
                                                                } else {
                                                                    buff[tmp___21 as usize] = 1 as libc::c_int as libc::c_uchar;
                                                                }
                                                                if v6 != 0 {
                                                                    tmp___22 = 16 as libc::c_int;
                                                                } else {
                                                                    tmp___22 = 4 as libc::c_int;
                                                                }
                                                                memcpy(
                                                                    buff.as_mut_ptr().offset(buff_iter as isize)
                                                                        as *mut libc::c_void,
                                                                    (ip.addr.v6).as_mut_ptr() as *const libc::c_void,
                                                                    tmp___22 as size_t,
                                                                );
                                                                if v6 != 0 {
                                                                    tmp___23 = 16 as libc::c_int;
                                                                } else {
                                                                    tmp___23 = 4 as libc::c_int;
                                                                }
                                                                buff_iter += tmp___23;
                                                            } else {
                                                                tmp___24 = buff_iter;
                                                                buff_iter += 1;
                                                                buff[tmp___24 as usize] = 3 as libc::c_int as libc::c_uchar;
                                                                tmp___25 = buff_iter;
                                                                buff_iter += 1;
                                                                buff[tmp___25
                                                                    as usize] = (dns_len & 255 as libc::c_ulong)
                                                                    as libc::c_uchar;
                                                                memcpy(
                                                                    buff.as_mut_ptr().offset(buff_iter as isize)
                                                                        as *mut libc::c_void,
                                                                    dns_name as *const libc::c_void,
                                                                    dns_len,
                                                                );
                                                                buff_iter = (buff_iter as size_t).wrapping_add(dns_len)
                                                                    as libc::c_int;
                                                            }
                                                            memcpy(
                                                                buff.as_mut_ptr().offset(buff_iter as isize)
                                                                    as *mut libc::c_void,
                                                                &mut port as *mut libc::c_ushort as *const libc::c_void,
                                                                2 as libc::c_int as size_t,
                                                            );
                                                            buff_iter += 2 as libc::c_int;
                                                            tmp___26 = write_n_bytes(
                                                                sock,
                                                                buff.as_mut_ptr() as *mut libc::c_char,
                                                                buff_iter as size_t,
                                                            );
                                                            if buff_iter != tmp___26 {
                                                                current_block = 8149651411747124477;
                                                            } else {
                                                                tmp___27 = read_n_bytes(
                                                                    sock,
                                                                    buff.as_mut_ptr() as *mut libc::c_char,
                                                                    4 as libc::c_int as size_t,
                                                                );
                                                                if 4 as libc::c_int != tmp___27 {
                                                                    current_block = 8149651411747124477;
                                                                } else if buff[0 as libc::c_int as usize] as libc::c_int
                                                                        != 5 as libc::c_int
                                                                    {
                                                                    current_block = 8149651411747124477;
                                                                } else if buff[1 as libc::c_int as usize] as libc::c_int
                                                                        != 0 as libc::c_int
                                                                    {
                                                                    current_block = 8149651411747124477;
                                                                } else {
                                                                    match buff[3 as libc::c_int as usize] as libc::c_int {
                                                                        1 => {
                                                                            current_block = 5117978787054293485;
                                                                            match current_block {
                                                                                1683520436886296249 => {
                                                                                    len = 16 as libc::c_int;
                                                                                    current_block = 6930811285952240378;
                                                                                }
                                                                                5117978787054293485 => {
                                                                                    len = 4 as libc::c_int;
                                                                                    current_block = 6930811285952240378;
                                                                                }
                                                                                _ => {
                                                                                    len = 0 as libc::c_int;
                                                                                    tmp___28 = read_n_bytes(
                                                                                        sock,
                                                                                        &mut len as *mut libc::c_int as *mut libc::c_char,
                                                                                        1 as libc::c_int as size_t,
                                                                                    );
                                                                                    if 1 as libc::c_int != tmp___28 {
                                                                                        current_block = 8149651411747124477;
                                                                                    } else {
                                                                                        current_block = 6930811285952240378;
                                                                                    }
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                8149651411747124477 => {}
                                                                                _ => {
                                                                                    tmp___29 = read_n_bytes(
                                                                                        sock,
                                                                                        buff.as_mut_ptr() as *mut libc::c_char,
                                                                                        (len + 2 as libc::c_int) as size_t,
                                                                                    );
                                                                                    if len + 2 as libc::c_int != tmp___29 {
                                                                                        current_block = 8149651411747124477;
                                                                                    } else {
                                                                                        return 0 as libc::c_int
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        4 => {
                                                                            current_block = 1683520436886296249;
                                                                            match current_block {
                                                                                1683520436886296249 => {
                                                                                    len = 16 as libc::c_int;
                                                                                    current_block = 6930811285952240378;
                                                                                }
                                                                                5117978787054293485 => {
                                                                                    len = 4 as libc::c_int;
                                                                                    current_block = 6930811285952240378;
                                                                                }
                                                                                _ => {
                                                                                    len = 0 as libc::c_int;
                                                                                    tmp___28 = read_n_bytes(
                                                                                        sock,
                                                                                        &mut len as *mut libc::c_int as *mut libc::c_char,
                                                                                        1 as libc::c_int as size_t,
                                                                                    );
                                                                                    if 1 as libc::c_int != tmp___28 {
                                                                                        current_block = 8149651411747124477;
                                                                                    } else {
                                                                                        current_block = 6930811285952240378;
                                                                                    }
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                8149651411747124477 => {}
                                                                                _ => {
                                                                                    tmp___29 = read_n_bytes(
                                                                                        sock,
                                                                                        buff.as_mut_ptr() as *mut libc::c_char,
                                                                                        (len + 2 as libc::c_int) as size_t,
                                                                                    );
                                                                                    if len + 2 as libc::c_int != tmp___29 {
                                                                                        current_block = 8149651411747124477;
                                                                                    } else {
                                                                                        return 0 as libc::c_int
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        3 => {
                                                                            current_block = 5428223447338481681;
                                                                            match current_block {
                                                                                1683520436886296249 => {
                                                                                    len = 16 as libc::c_int;
                                                                                    current_block = 6930811285952240378;
                                                                                }
                                                                                5117978787054293485 => {
                                                                                    len = 4 as libc::c_int;
                                                                                    current_block = 6930811285952240378;
                                                                                }
                                                                                _ => {
                                                                                    len = 0 as libc::c_int;
                                                                                    tmp___28 = read_n_bytes(
                                                                                        sock,
                                                                                        &mut len as *mut libc::c_int as *mut libc::c_char,
                                                                                        1 as libc::c_int as size_t,
                                                                                    );
                                                                                    if 1 as libc::c_int != tmp___28 {
                                                                                        current_block = 8149651411747124477;
                                                                                    } else {
                                                                                        current_block = 6930811285952240378;
                                                                                    }
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                8149651411747124477 => {}
                                                                                _ => {
                                                                                    tmp___29 = read_n_bytes(
                                                                                        sock,
                                                                                        buff.as_mut_ptr() as *mut libc::c_char,
                                                                                        (len + 2 as libc::c_int) as size_t,
                                                                                    );
                                                                                    if len + 2 as libc::c_int != tmp___29 {
                                                                                        current_block = 8149651411747124477;
                                                                                    } else {
                                                                                        return 0 as libc::c_int
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        _ => {
                                                                            current_block = 8149651411747124477;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        match current_block {
                                            8149651411747124477 => {}
                                            _ => {
                                                if buff[0 as libc::c_int as usize] as libc::c_int
                                                    == 5 as libc::c_int
                                                {
                                                    if buff[1 as libc::c_int as usize] as libc::c_int
                                                        == 255 as libc::c_int
                                                    {
                                                        return 5 as libc::c_int;
                                                    }
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
            }
        }
        _ => {}
    }
    return 2 as libc::c_int;
}
unsafe extern "C" fn start_chain(
    mut fd: *mut libc::c_int,
    mut pd: *mut proxy_data,
    mut begin_mark: *mut libc::c_char,
) -> libc::c_int {
    let mut v6: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut ip_buf: [libc::c_char; 46] = [0; 46];
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___2: uint16_t = 0;
    let mut addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut addr6: sockaddr_in6 = sockaddr_in6 {
        sin6_family: 0,
        sin6_port: 0,
        sin6_flowinfo: 0,
        sin6_addr: in6_addr {
            __in6_u: __anonunion___in6_u_979734923 {
                __u6_addr8: [0; 16],
            },
        },
        sin6_scope_id: 0,
    };
    let mut tmp___3: libc::c_ulong = 0;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___5: libc::c_int = 0;
    v6 = (*pd).ip.is_v6 as libc::c_int;
    if v6 != 0 {
        tmp = 10 as libc::c_int;
    } else {
        tmp = 2 as libc::c_int;
    }
    *fd = socket(tmp, 1 as libc::c_int, 0 as libc::c_int);
    if !(*fd == -(1 as libc::c_int)) {
        if v6 != 0 {
            tmp___0 = 10 as libc::c_int;
        } else {
            tmp___0 = 2 as libc::c_int;
        }
        tmp___1 = inet_ntop(
            tmp___0,
            ((*pd).ip.addr.v6).as_mut_ptr() as *const libc::c_void,
            ip_buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong as socklen_t,
        );
        if !tmp___1.is_null() {
            tmp___2 = htons((*pd).port);
            proxychains_write_log(
                b"[proxychains] %s  ...  %s:%d \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                begin_mark,
                ip_buf.as_mut_ptr(),
                tmp___2 as libc::c_int,
            );
            (*pd).ps = PLAY_STATE;
            addr.sin_family = 2 as libc::c_int as sa_family_t;
            addr.sin_port = (*pd).port;
            addr.sin_addr.s_addr = (*pd).ip.addr.v4.as_int;
            addr.sin_zero[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
            addr.sin_zero[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
            addr.sin_zero[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
            addr.sin_zero[3 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
            addr.sin_zero[4 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
            addr.sin_zero[5 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
            addr.sin_zero[6 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
            addr.sin_zero[7 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
            addr6.sin6_family = 10 as libc::c_int as sa_family_t;
            addr6.sin6_port = (*pd).port;
            addr6.sin6_flowinfo = 0 as libc::c_uint;
            addr6
                .sin6_addr
                .__in6_u
                .__u6_addr8[0 as libc::c_int
                as usize] = 0 as libc::c_int as libc::c_uchar;
            addr6
                .sin6_addr
                .__in6_u
                .__u6_addr8[1 as libc::c_int
                as usize] = 0 as libc::c_int as libc::c_uchar;
            addr6
                .sin6_addr
                .__in6_u
                .__u6_addr8[2 as libc::c_int
                as usize] = 0 as libc::c_int as libc::c_uchar;
            addr6
                .sin6_addr
                .__in6_u
                .__u6_addr8[3 as libc::c_int
                as usize] = 0 as libc::c_int as libc::c_uchar;
            addr6
                .sin6_addr
                .__in6_u
                .__u6_addr8[4 as libc::c_int
                as usize] = 0 as libc::c_int as libc::c_uchar;
            addr6
                .sin6_addr
                .__in6_u
                .__u6_addr8[5 as libc::c_int
                as usize] = 0 as libc::c_int as libc::c_uchar;
            addr6
                .sin6_addr
                .__in6_u
                .__u6_addr8[6 as libc::c_int
                as usize] = 0 as libc::c_int as libc::c_uchar;
            addr6
                .sin6_addr
                .__in6_u
                .__u6_addr8[7 as libc::c_int
                as usize] = 0 as libc::c_int as libc::c_uchar;
            addr6
                .sin6_addr
                .__in6_u
                .__u6_addr8[8 as libc::c_int
                as usize] = 0 as libc::c_int as libc::c_uchar;
            addr6
                .sin6_addr
                .__in6_u
                .__u6_addr8[9 as libc::c_int
                as usize] = 0 as libc::c_int as libc::c_uchar;
            addr6
                .sin6_addr
                .__in6_u
                .__u6_addr8[10 as libc::c_int
                as usize] = 0 as libc::c_int as libc::c_uchar;
            addr6
                .sin6_addr
                .__in6_u
                .__u6_addr8[11 as libc::c_int
                as usize] = 0 as libc::c_int as libc::c_uchar;
            addr6
                .sin6_addr
                .__in6_u
                .__u6_addr8[12 as libc::c_int
                as usize] = 0 as libc::c_int as libc::c_uchar;
            addr6
                .sin6_addr
                .__in6_u
                .__u6_addr8[13 as libc::c_int
                as usize] = 0 as libc::c_int as libc::c_uchar;
            addr6
                .sin6_addr
                .__in6_u
                .__u6_addr8[14 as libc::c_int
                as usize] = 0 as libc::c_int as libc::c_uchar;
            addr6
                .sin6_addr
                .__in6_u
                .__u6_addr8[15 as libc::c_int
                as usize] = 0 as libc::c_int as libc::c_uchar;
            addr6.sin6_scope_id = 0 as libc::c_uint;
            if v6 != 0 {
                memcpy(
                    &mut addr6.sin6_addr.__in6_u.__u6_addr8 as *mut [uint8_t; 16]
                        as *mut libc::c_void,
                    ((*pd).ip.addr.v6).as_mut_ptr() as *const libc::c_void,
                    16 as libc::c_int as size_t,
                );
            }
            if v6 != 0 {
                tmp___3 = ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong;
            } else {
                tmp___3 = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong;
            }
            if v6 != 0 {
                tmp___4 = &mut addr6 as *mut sockaddr_in6 as *mut libc::c_void;
            } else {
                tmp___4 = &mut addr as *mut sockaddr_in as *mut libc::c_void;
            }
            tmp___5 = timed_connect(
                *fd,
                tmp___4 as *mut sockaddr as *const sockaddr,
                tmp___3 as socklen_t,
            );
            if tmp___5 != 0 {
                (*pd).ps = DOWN_STATE;
                proxychains_write_log(
                    b" ...  timeout\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            } else {
                (*pd).ps = BUSY_STATE;
                return 0 as libc::c_int;
            }
        }
    }
    if *fd != -(1 as libc::c_int) {
        close(*fd);
    }
    return 2 as libc::c_int;
}
unsafe extern "C" fn select_proxy(
    mut how: select_type,
    mut pd: *mut proxy_data,
    mut proxy_count: libc::c_uint,
    mut offset: *mut libc::c_uint,
) -> *mut proxy_data {
    let mut i: libc::c_uint = 0;
    let mut k: libc::c_uint = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut proxy_data = 0 as *mut proxy_data;
    i = 0 as libc::c_uint;
    k = 0 as libc::c_uint;
    if *offset >= proxy_count {
        return 0 as *mut libc::c_void as *mut proxy_data;
    }
    match how as libc::c_uint {
        0 => {
            loop {
                k = k.wrapping_add(1);
                tmp = rand();
                i = (tmp as libc::c_uint).wrapping_rem(proxy_count);
                if !((*pd.offset(i as isize)).ps as libc::c_uint != 0 as libc::c_uint) {
                    break;
                }
                if !(k < proxy_count.wrapping_mul(100 as libc::c_uint)) {
                    break;
                }
            }
        }
        1 => {
            i = *offset;
            while i < proxy_count {
                if (*pd.offset(i as isize)).ps as libc::c_uint == 0 as libc::c_uint {
                    *offset = i;
                    break;
                } else {
                    i = i.wrapping_add(1);
                }
            }
        }
        _ => {}
    }
    if i >= proxy_count {
        i = 0 as libc::c_uint;
    }
    if (*pd.offset(i as isize)).ps as libc::c_uint == 0 as libc::c_uint {
        tmp___0 = pd.offset(i as isize);
    } else {
        tmp___0 = 0 as *mut libc::c_void as *mut proxy_data;
    }
    return tmp___0;
}
unsafe extern "C" fn release_all(
    mut pd: *mut proxy_data,
    mut proxy_count: libc::c_uint,
) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_uint;
    while i < proxy_count {
        (*pd.offset(i as isize)).ps = PLAY_STATE;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn release_busy(
    mut pd: *mut proxy_data,
    mut proxy_count: libc::c_uint,
) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_uint;
    while i < proxy_count {
        if (*pd.offset(i as isize)).ps as libc::c_uint == 3 as libc::c_uint {
            (*pd.offset(i as isize)).ps = PLAY_STATE;
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn calc_alive(
    mut pd: *mut proxy_data,
    mut proxy_count: libc::c_uint,
) -> libc::c_uint {
    let mut i: libc::c_uint = 0;
    let mut alive_count: libc::c_int = 0;
    alive_count = 0 as libc::c_int;
    release_busy(pd, proxy_count);
    i = 0 as libc::c_uint;
    while i < proxy_count {
        if (*pd.offset(i as isize)).ps as libc::c_uint == 0 as libc::c_uint {
            alive_count += 1;
        }
        i = i.wrapping_add(1);
    }
    return alive_count as libc::c_uint;
}
unsafe extern "C" fn chain_step(
    mut ns: libc::c_int,
    mut pfrom: *mut proxy_data,
    mut pto: *mut proxy_data,
) -> libc::c_int {
    let mut current_block: u64;
    let mut retcode: libc::c_int = 0;
    let mut hostname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hostname_buf: [libc::c_char; 256] = [0; 256];
    let mut ip_buf: [libc::c_char; 46] = [0; 46];
    let mut v6: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___2: uint16_t = 0;
    retcode = -(1 as libc::c_int);
    v6 = (*pto).ip.is_v6 as libc::c_int;
    if v6 == 0 {
        if proxychains_resolver as libc::c_uint >= 2 as libc::c_uint {
            if (*pto).ip.addr.v4.octet[0 as libc::c_int as usize] as libc::c_uint
                == remote_dns_subnet
            {
                tmp = rdns_get_host_for_ip((*pto).ip.addr.v4, hostname_buf.as_mut_ptr());
                if tmp != 0 {
                    hostname = hostname_buf.as_mut_ptr();
                    current_block = 15345278821338558188;
                } else {
                    current_block = 3416470763250070661;
                }
            } else {
                current_block = 3416470763250070661;
            }
        } else {
            current_block = 3416470763250070661;
        }
    } else {
        current_block = 3416470763250070661;
    }
    match current_block {
        3416470763250070661 => {
            if v6 != 0 {
                tmp___0 = 10 as libc::c_int;
            } else {
                tmp___0 = 2 as libc::c_int;
            }
            tmp___1 = inet_ntop(
                tmp___0,
                ((*pto).ip.addr.v6).as_mut_ptr() as *const libc::c_void,
                ip_buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong as socklen_t,
            );
            if tmp___1.is_null() {
                (*pto).ps = DOWN_STATE;
                proxychains_write_log(
                    b"<--ip conversion error!\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                close(ns);
                return 2 as libc::c_int;
            }
            hostname = ip_buf.as_mut_ptr();
        }
        _ => {}
    }
    tmp___2 = htons((*pto).port);
    proxychains_write_log(
        b" ...  %s:%d \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        hostname,
        tmp___2 as libc::c_int,
    );
    retcode = tunnel_to(
        ns,
        (*pto).ip,
        (*pto).port,
        (*pfrom).pt,
        ((*pfrom).user).as_mut_ptr(),
        ((*pfrom).pass).as_mut_ptr(),
    );
    match retcode {
        0 => {
            (*pto).ps = BUSY_STATE;
        }
        5 => {
            (*pto).ps = BLOCKED_STATE;
            proxychains_write_log(
                b"<--denied\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            close(ns);
        }
        2 => {
            (*pto).ps = DOWN_STATE;
            proxychains_write_log(
                b"<--socket error or timeout!\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            close(ns);
        }
        _ => {}
    }
    return retcode;
}
pub unsafe extern "C" fn connect_proxy_chain(
    mut sock: libc::c_int,
    mut target_ip: ip_type,
    mut target_port: libc::c_ushort,
    mut pd: *mut proxy_data,
    mut proxy_count: libc::c_uint,
    mut ct: chain_type,
    mut max_chain: libc::c_uint,
) -> libc::c_int {
    let mut current_block: u64;
    let mut p4: proxy_data = proxy_data {
        ip: ip_type {
            addr: __anonunion_addr_69655148 {
                v4: __anonunion_ip_type4_826858479 {
                    octet: [0; 4],
                },
            },
            is_v6: 0,
        },
        port: 0,
        pt: HTTP_TYPE,
        ps: PLAY_STATE,
        user: [0; 256],
        pass: [0; 256],
    };
    let mut p1: *mut proxy_data = 0 as *mut proxy_data;
    let mut p2: *mut proxy_data = 0 as *mut proxy_data;
    let mut p3: *mut proxy_data = 0 as *mut proxy_data;
    let mut ns: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut offset: libc::c_uint = 0;
    let mut alive_count: libc::c_uint = 0;
    let mut curr_len: libc::c_uint = 0;
    let mut looped: libc::c_uint = 0;
    let mut rr_loop_max: libc::c_uint = 0;
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
    let mut tmp___10: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___11: *mut libc::c_int = 0 as *mut libc::c_int;
    ns = -(1 as libc::c_int);
    rc = -(1 as libc::c_int);
    offset = 0 as libc::c_uint;
    alive_count = 0 as libc::c_uint;
    curr_len = 0 as libc::c_uint;
    looped = 0 as libc::c_uint;
    rr_loop_max = 14 as libc::c_uint;
    p3 = &mut p4;
    '_again: loop {
        rc = -(1 as libc::c_int);
        match ct as libc::c_uint {
            0 => {
                alive_count = calc_alive(pd, proxy_count);
                offset = 0 as libc::c_uint;
                loop {
                    p1 = select_proxy(FIFOLY, pd, proxy_count, &mut offset);
                    if p1.is_null() {
                        current_block = 14266008684865719659;
                        break '_again;
                    }
                    tmp = start_chain(
                        &mut ns,
                        p1,
                        b"Dynamic chain\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    if !(0 as libc::c_int != tmp) {
                        break;
                    }
                    if !(offset < proxy_count) {
                        break;
                    }
                }
                loop {
                    p2 = select_proxy(FIFOLY, pd, proxy_count, &mut offset);
                    if p2.is_null() {
                        break;
                    }
                    tmp___0 = chain_step(ns, p1, p2);
                    if 0 as libc::c_int != tmp___0 {
                        continue '_again;
                    }
                    p1 = p2;
                }
                (*p3).ip = target_ip;
                (*p3).port = target_port;
                tmp___1 = chain_step(ns, p1, p3);
                if 0 as libc::c_int != tmp___1 {
                    current_block = 16208957377130606339;
                    break;
                } else {
                    current_block = 6584656659744957450;
                    break;
                }
            }
            3 => {
                alive_count = calc_alive(pd, proxy_count);
                offset = proxychains_proxy_offset;
                if alive_count < max_chain {
                    current_block = 14266008684865719659;
                    break;
                }
                while rc != 0 as libc::c_int {
                    p1 = select_proxy(FIFOLY, pd, proxy_count, &mut offset);
                    if p1.is_null() {
                        offset = 0 as libc::c_uint;
                        looped = looped.wrapping_add(1);
                        if looped > rr_loop_max {
                            proxychains_proxy_offset = 0 as libc::c_uint;
                            current_block = 14266008684865719659;
                            break '_again;
                        } else {
                            release_all(pd, proxy_count);
                            usleep((10000 as libc::c_uint).wrapping_mul(looped));
                        }
                    } else {
                        rc = start_chain(
                            &mut ns,
                            p1,
                            b"Round Robin chain\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                    }
                }
                curr_len = 1 as libc::c_uint;
                while curr_len < max_chain {
                    p2 = select_proxy(FIFOLY, pd, proxy_count, &mut offset);
                    if p2.is_null() {
                        offset = 0 as libc::c_uint;
                    } else {
                        tmp___2 = chain_step(ns, p1, p2);
                        if 0 as libc::c_int != tmp___2 {
                            continue '_again;
                        }
                        p1 = p2;
                        curr_len = curr_len.wrapping_add(1);
                    }
                }
                (*p3).ip = target_ip;
                (*p3).port = target_port;
                proxychains_proxy_offset = offset.wrapping_add(1 as libc::c_uint);
                tmp___3 = chain_step(ns, p1, p3);
                if 0 as libc::c_int != tmp___3 {
                    current_block = 16208957377130606339;
                    break;
                } else {
                    current_block = 6584656659744957450;
                    break;
                }
            }
            1 => {
                alive_count = calc_alive(pd, proxy_count);
                offset = 0 as libc::c_uint;
                p1 = select_proxy(FIFOLY, pd, proxy_count, &mut offset);
                if p1.is_null() {
                    current_block = 5431927413890720344;
                    break;
                } else {
                    current_block = 10393716428851982524;
                    break;
                }
            }
            2 => {
                alive_count = calc_alive(pd, proxy_count);
                if alive_count < max_chain {
                    current_block = 14266008684865719659;
                    break;
                }
                offset = 0 as libc::c_uint;
                curr_len = offset;
                loop {
                    p1 = select_proxy(RANDOMLY, pd, proxy_count, &mut offset);
                    if p1.is_null() {
                        current_block = 14266008684865719659;
                        break '_again;
                    }
                    tmp___7 = start_chain(
                        &mut ns,
                        p1,
                        b"Random chain\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    if !(0 as libc::c_int != tmp___7) {
                        break;
                    }
                    if !(offset < max_chain) {
                        break;
                    }
                }
                loop {
                    curr_len = curr_len.wrapping_add(1);
                    if !(curr_len < max_chain) {
                        break;
                    }
                    p2 = select_proxy(RANDOMLY, pd, proxy_count, &mut offset);
                    if p2.is_null() {
                        current_block = 14266008684865719659;
                        break '_again;
                    }
                    tmp___8 = chain_step(ns, p1, p2);
                    if 0 as libc::c_int != tmp___8 {
                        continue '_again;
                    }
                    p1 = p2;
                }
                (*p3).ip = target_ip;
                (*p3).port = target_port;
                tmp___9 = chain_step(ns, p1, p3);
                if 0 as libc::c_int != tmp___9 {
                    current_block = 16208957377130606339;
                    break;
                } else {
                    current_block = 6584656659744957450;
                    break;
                }
            }
            _ => {
                current_block = 6584656659744957450;
                break;
            }
        }
    }
    match current_block {
        10393716428851982524 => {
            tmp___4 = start_chain(
                &mut ns,
                p1,
                b"Strict chain\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            if 0 as libc::c_int != tmp___4 {
                current_block = 5431927413890720344;
            } else {
                loop {
                    if !(offset < proxy_count) {
                        current_block = 4488496028633655612;
                        break;
                    }
                    p2 = select_proxy(FIFOLY, pd, proxy_count, &mut offset);
                    if p2.is_null() {
                        current_block = 4488496028633655612;
                        break;
                    }
                    tmp___5 = chain_step(ns, p1, p2);
                    if 0 as libc::c_int != tmp___5 {
                        current_block = 5431927413890720344;
                        break;
                    }
                    p1 = p2;
                }
                match current_block {
                    5431927413890720344 => {}
                    _ => {
                        (*p3).ip = target_ip;
                        (*p3).port = target_port;
                        tmp___6 = chain_step(ns, p1, p3);
                        if 0 as libc::c_int != tmp___6 {
                            current_block = 16208957377130606339;
                        } else {
                            current_block = 6584656659744957450;
                        }
                    }
                }
            }
        }
        14266008684865719659 => {
            proxychains_write_log(
                b"\n!!!need more proxies!!!\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            current_block = 5431927413890720344;
        }
        _ => {}
    }
    match current_block {
        5431927413890720344 => {
            release_all(pd, proxy_count);
            if ns != -(1 as libc::c_int) {
                close(ns);
            }
            tmp___11 = __errno_location();
            *tmp___11 = 110 as libc::c_int;
            return -(1 as libc::c_int);
        }
        16208957377130606339 => {
            if ns != -(1 as libc::c_int) {
                close(ns);
            }
            tmp___10 = __errno_location();
            *tmp___10 = 111 as libc::c_int;
            return -(1 as libc::c_int);
        }
        _ => {
            proxychains_write_log(
                b" ...  OK\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            dup2(ns, sock);
            close(ns);
            return 0 as libc::c_int;
        }
    };
}
static mut servbyname_lock: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
pub unsafe extern "C" fn core_initialize() {
    pthread_mutex_init(
        &mut servbyname_lock,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
}
pub unsafe extern "C" fn core_unload() {
    pthread_mutex_destroy(&mut servbyname_lock);
}
unsafe extern "C" fn gethostbyname_data_setstring(
    mut data: *mut gethostbyname_data,
    mut name: *mut libc::c_char,
) {
    snprintf(
        ((*data).addr_name).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        name,
    );
    (*data).hostent_space.h_name = ((*data).addr_name).as_mut_ptr();
}
static mut hostent_space: hostent = hostent {
    h_name: 0 as *const libc::c_char as *mut libc::c_char,
    h_aliases: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    h_addrtype: 0,
    h_length: 0,
    h_addr_list: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
};
static mut resolved_addr: in_addr_t = 0;
static mut resolved_addr_p: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut addr_name: [libc::c_char; 256] = [0; 256];
pub unsafe extern "C" fn proxy_gethostbyname_old(
    mut name: *const libc::c_char,
) -> *mut hostent {
    let mut current_block: u64;
    let mut pipe_fd: [libc::c_int; 2] = [0; 2];
    let mut buff: [libc::c_char; 256] = [0; 256];
    let mut addr: in_addr_t = 0;
    let mut pid: pid_t = 0;
    let mut status: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut l: size_t = 0;
    let mut hp: *mut hostent = 0 as *mut hostent;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    hostent_space.h_addr_list = &mut resolved_addr_p;
    *hostent_space
        .h_addr_list = &mut resolved_addr as *mut in_addr_t as *mut libc::c_char;
    resolved_addr = 0 as libc::c_int as in_addr_t;
    tmp = pc_isnumericipv4(name);
    if tmp != 0 {
        strcpy(buff.as_mut_ptr(), name);
        current_block = 5631081420871876180;
    } else {
        gethostname(
            buff.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
        tmp___0 = strcmp(buff.as_mut_ptr() as *const libc::c_char, name);
        if tmp___0 == 0 {
            current_block = 5631081420871876180;
        } else {
            memset(
                buff.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            );
            loop {
                hp = gethostent();
                if hp.is_null() {
                    break;
                }
                tmp___1 = strcmp((*hp).h_name as *const libc::c_char, name);
                if tmp___1 == 0 {
                    return hp;
                }
            }
            ret = pipe2(pipe_fd.as_mut_ptr(), 524288 as libc::c_int);
            if ret != 0 {
                current_block = 8160726858084670948;
            } else {
                pid = fork();
                match pid {
                    0 => {
                        proxychains_write_log(
                            b"|DNS-request| %s \n\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            name,
                        );
                        close(pipe_fd[0 as libc::c_int as usize]);
                        dup2(pipe_fd[1 as libc::c_int as usize], 1 as libc::c_int);
                        close(pipe_fd[1 as libc::c_int as usize]);
                        execlp(
                            b"proxyresolv\0" as *const u8 as *const libc::c_char,
                            b"proxyresolv\0" as *const u8 as *const libc::c_char,
                            name,
                            0 as *mut libc::c_void,
                        );
                        perror(
                            b"can't exec proxyresolv\0" as *const u8
                                as *const libc::c_char,
                        );
                        exit(2 as libc::c_int);
                    }
                    -1 => {
                        close(pipe_fd[0 as libc::c_int as usize]);
                        close(pipe_fd[1 as libc::c_int as usize]);
                        perror(b"can't fork\0" as *const u8 as *const libc::c_char);
                        current_block = 8160726858084670948;
                    }
                    _ => {
                        close(pipe_fd[1 as libc::c_int as usize]);
                        waitpid(pid, &mut status, 0 as libc::c_int);
                        buff[0 as libc::c_int
                            as usize] = 0 as libc::c_int as libc::c_char;
                        read(
                            pipe_fd[0 as libc::c_int as usize],
                            &mut buff as *mut [libc::c_char; 256] as *mut libc::c_void,
                            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                        );
                        close(pipe_fd[0 as libc::c_int as usize]);
                        current_block = 5631081420871876180;
                    }
                }
            }
        }
    }
    match current_block {
        5631081420871876180 => {
            l = strlen(buff.as_mut_ptr() as *const libc::c_char);
            if l != 0 {
                if buff[l.wrapping_sub(1 as libc::c_ulong) as usize] as libc::c_int
                    == 10 as libc::c_int
                {
                    buff[l.wrapping_sub(1 as libc::c_ulong)
                        as usize] = 0 as libc::c_int as libc::c_char;
                }
            }
            addr = inet_addr(buff.as_mut_ptr() as *const libc::c_char);
            if addr == 4294967295 as libc::c_uint {
                proxychains_write_log(
                    b"|DNS-response|: %s does not exist\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    name,
                );
                perror(b"err_dns\0" as *const u8 as *const libc::c_char);
            } else {
                memcpy(
                    *hostent_space.h_addr_list as *mut libc::c_void,
                    &mut addr as *mut in_addr_t as *const libc::c_void,
                    ::std::mem::size_of::<in_addr>() as libc::c_ulong,
                );
                hostent_space.h_name = addr_name.as_mut_ptr();
                snprintf(
                    addr_name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    buff.as_mut_ptr(),
                );
                hostent_space
                    .h_length = ::std::mem::size_of::<in_addr_t>() as libc::c_ulong
                    as libc::c_int;
                hostent_space.h_addrtype = 2 as libc::c_int;
                tmp___2 = inet_ntoa(*(&mut addr as *mut in_addr_t as *mut in_addr));
                proxychains_write_log(
                    b"|DNS-response| %s is %s\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    name,
                    tmp___2,
                );
                return &mut hostent_space;
            }
        }
        _ => {}
    }
    return 0 as *mut libc::c_void as *mut hostent;
}
pub unsafe extern "C" fn proxy_gethostbyname(
    mut name: *const libc::c_char,
    mut data: *mut gethostbyname_data,
) -> *mut hostent {
    let mut buff: [libc::c_char; 256] = [0; 256];
    let mut tmp: libc::c_int = 0;
    let mut __constr_expr_0: ip_type4 = __anonunion_ip_type4_826858479 {
        octet: [0; 4],
    };
    let mut tmp___0: libc::c_int = 0;
    let mut hdb_res: ip_type4 = __anonunion_ip_type4_826858479 {
        octet: [0; 4],
    };
    let mut tmp___1: ip_type4 = __anonunion_ip_type4_826858479 {
        octet: [0; 4],
    };
    let mut __constr_expr_1: ip_type4 = __anonunion_ip_type4_826858479 {
        octet: [0; 4],
    };
    let mut tmp___2: size_t = 0;
    let mut tmp___3: ip_type4 = __anonunion_ip_type4_826858479 {
        octet: [0; 4],
    };
    let mut __constr_expr_2: ip_type4 = __anonunion_ip_type4_826858479 {
        octet: [0; 4],
    };
    (*data)
        .resolved_addr_p[0 as libc::c_int
        as usize] = &mut (*data).resolved_addr as *mut in_addr_t as *mut libc::c_char;
    (*data)
        .resolved_addr_p[1 as libc::c_int
        as usize] = 0 as *mut libc::c_void as *mut libc::c_char;
    (*data).hostent_space.h_addr_list = ((*data).resolved_addr_p).as_mut_ptr();
    (*data)
        .hostent_space
        .h_aliases = &mut *((*data).resolved_addr_p)
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize) as *mut *mut libc::c_char;
    (*data).resolved_addr = 0 as libc::c_int as in_addr_t;
    (*data).hostent_space.h_addrtype = 2 as libc::c_int;
    (*data)
        .hostent_space
        .h_length = ::std::mem::size_of::<in_addr_t>() as libc::c_ulong as libc::c_int;
    tmp = pc_isnumericipv4(name);
    if tmp != 0 {
        (*data).resolved_addr = inet_addr(name);
    } else {
        gethostname(
            buff.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
        tmp___0 = strcmp(buff.as_mut_ptr() as *const libc::c_char, name);
        if tmp___0 == 0 {
            (*data).resolved_addr = inet_addr(buff.as_mut_ptr() as *const libc::c_char);
            if (*data).resolved_addr == 4294967295 as libc::c_uint {
                __constr_expr_0
                    .octet[0 as libc::c_int
                    as usize] = 127 as libc::c_int as libc::c_uchar;
                __constr_expr_0
                    .octet[1 as libc::c_int
                    as usize] = 0 as libc::c_int as libc::c_uchar;
                __constr_expr_0
                    .octet[2 as libc::c_int
                    as usize] = 0 as libc::c_int as libc::c_uchar;
                __constr_expr_0
                    .octet[3 as libc::c_int
                    as usize] = 1 as libc::c_int as libc::c_uchar;
                (*data).resolved_addr = __constr_expr_0.as_int;
            }
        } else {
            tmp___1 = hostsreader_get_numeric_ip_for_name(name);
            hdb_res = tmp___1;
            __constr_expr_1.as_int = -(1 as libc::c_int) as uint32_t;
            if hdb_res.as_int != __constr_expr_1.as_int {
                (*data).resolved_addr = hdb_res.as_int;
            } else {
                tmp___2 = strlen(name);
                tmp___3 = rdns_get_ip_for_host(name as *mut libc::c_char, tmp___2);
                (*data).resolved_addr = tmp___3.as_int;
                __constr_expr_2.as_int = -(1 as libc::c_int) as uint32_t;
                if (*data).resolved_addr == __constr_expr_2.as_int {
                    return 0 as *mut libc::c_void as *mut hostent;
                }
            }
        }
    }
    gethostbyname_data_setstring(data, name as *mut libc::c_char);
    return &mut (*data).hostent_space;
}
pub unsafe extern "C" fn proxy_freeaddrinfo(mut res: *mut addrinfo) {
    free(res as *mut libc::c_void);
}
unsafe extern "C" fn mygetservbyname_r(
    mut name: *const libc::c_char,
    mut proto: *const libc::c_char,
    mut result_buf: *mut servent,
    mut buf___0: *mut libc::c_char,
    mut buflen: size_t,
    mut result: *mut *mut servent,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut res: *mut servent = 0 as *mut servent;
    let mut ret: libc::c_int = 0;
    tmp = getservbyname_r(name, proto, result_buf, buf___0, buflen, result);
    return tmp;
}
unsafe extern "C" fn looks_like_numeric_ipv6(
    mut node: *const libc::c_char,
) -> libc::c_int {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    tmp = strchr(node, ':' as i32);
    if tmp.is_null() {
        return 0 as libc::c_int;
    }
    p = node;
    loop {
        tmp___0 = p;
        p = p.offset(1);
        match *tmp___0 as libc::c_int {
            0 => return 1 as libc::c_int,
            102 | 101 | 100 | 99 | 98 | 97 | 70 | 69 | 68 | 67 | 66 | 65 | 57 | 56 | 55
            | 54 | 53 | 52 | 51 | 50 | 49 | 48 | 46 | 58 => {}
            _ => return 0 as libc::c_int,
        }
    };
}
unsafe extern "C" fn my_inet_aton(
    mut node: *const libc::c_char,
    mut space: *mut addrinfo_data,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    (*(&mut (*space).sockaddr_space as *mut sockaddr_storage as *mut sockaddr_in))
        .sin_family = 2 as libc::c_int as sa_family_t;
    ret = inet_aton(
        node,
        &mut (*(&mut (*space).sockaddr_space as *mut sockaddr_storage
            as *mut sockaddr_in))
            .sin_addr,
    );
    if ret != 0 {
        return ret
    } else {
        tmp = looks_like_numeric_ipv6(node);
        if tmp == 0 {
            return ret;
        }
    }
    ret = inet_pton(
        10 as libc::c_int,
        node,
        &mut (*(&mut (*space).sockaddr_space as *mut sockaddr_storage
            as *mut sockaddr_in6))
            .sin6_addr as *mut in6_addr as *mut libc::c_void,
    );
    if ret != 0 {
        (*(&mut (*space).sockaddr_space as *mut sockaddr_storage as *mut sockaddr_in6))
            .sin6_family = 10 as libc::c_int as sa_family_t;
    }
    return ret;
}
pub unsafe extern "C" fn proxy_getaddrinfo(
    mut node: *const libc::c_char,
    mut service: *const libc::c_char,
    mut hints: *const addrinfo,
    mut res: *mut *mut addrinfo,
) -> libc::c_int {
    let mut ghdata: gethostbyname_data = gethostbyname_data {
        hostent_space: hostent {
            h_name: 0 as *const libc::c_char as *mut libc::c_char,
            h_aliases: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            h_addrtype: 0,
            h_length: 0,
            h_addr_list: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        },
        resolved_addr: 0,
        resolved_addr_p: [0 as *mut libc::c_char; 2],
        addr_name: [0; 256],
    };
    let mut space: *mut addrinfo_data = 0 as *mut addrinfo_data;
    let mut se: *mut servent = 0 as *mut servent;
    let mut hp: *mut hostent = 0 as *mut hostent;
    let mut se_buf: servent = servent {
        s_name: 0 as *mut libc::c_char,
        s_aliases: 0 as *mut *mut libc::c_char,
        s_port: 0,
        s_proto: 0 as *mut libc::c_char,
    };
    let mut p: *mut addrinfo = 0 as *mut addrinfo;
    let mut buf___0: [libc::c_char; 1024] = [0; 1024];
    let mut port: libc::c_int = 0;
    let mut af: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: sa_family_t = 0;
    let mut __constr_expr_3: [libc::c_char; 4] = [0; 4];
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: uint16_t = 0;
    let mut tmp___5: sa_family_t = 0;
    let mut current_block_88: u64;
    se = 0 as *mut libc::c_void as *mut servent;
    hp = 0 as *mut libc::c_void as *mut hostent;
    af = 2 as libc::c_int;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<addrinfo_data>() as libc::c_ulong,
    );
    space = tmp as *mut addrinfo_data;
    if !space.is_null() {
        if !node.is_null() {
            tmp___1 = my_inet_aton(node, space);
            if tmp___1 != 0 {
                current_block_88 = 2449367154687302544;
            } else {
                if !hints.is_null() {
                    if (*hints).ai_flags & 4 as libc::c_int != 0 {
                        free(space as *mut libc::c_void);
                        return -(2 as libc::c_int);
                    }
                }
                if proxychains_resolver as libc::c_uint == 1 as libc::c_uint {
                    hp = proxy_gethostbyname_old(node);
                } else {
                    hp = proxy_gethostbyname(node, &mut ghdata);
                }
                if !hp.is_null() {
                    memcpy(
                        &mut (*(&mut (*space).sockaddr_space as *mut sockaddr_storage
                            as *mut sockaddr_in))
                            .sin_addr as *mut in_addr as *mut libc::c_void,
                        *(*hp).h_addr_list as *const libc::c_void,
                        ::std::mem::size_of::<in_addr_t>() as libc::c_ulong,
                    );
                    current_block_88 = 6450597802325118133;
                } else {
                    free(space as *mut libc::c_void);
                    current_block_88 = 10172424475578961115;
                }
            }
        } else {
            current_block_88 = 2449367154687302544;
        }
        match current_block_88 {
            10172424475578961115 => {}
            _ => {
                match current_block_88 {
                    2449367154687302544 => {
                        if !node.is_null() {
                            af = (*(&mut (*space).sockaddr_space as *mut sockaddr_storage
                                as *mut sockaddr_in))
                                .sin_family as libc::c_int;
                        } else if node.is_null() {
                            if (*hints).ai_flags & 1 as libc::c_int == 0 {
                                tmp___0 = 2 as libc::c_int as sa_family_t;
                                (*(&mut (*space).sockaddr_space as *mut sockaddr_storage
                                    as *mut sockaddr_in))
                                    .sin_family = tmp___0;
                                af = tmp___0 as libc::c_int;
                                __constr_expr_3[0 as libc::c_int
                                    as usize] = 127 as libc::c_int as libc::c_char;
                                __constr_expr_3[1 as libc::c_int
                                    as usize] = 0 as libc::c_int as libc::c_char;
                                __constr_expr_3[2 as libc::c_int
                                    as usize] = 0 as libc::c_int as libc::c_char;
                                __constr_expr_3[3 as libc::c_int
                                    as usize] = 1 as libc::c_int as libc::c_char;
                                memcpy(
                                    &mut (*(&mut (*space).sockaddr_space
                                        as *mut sockaddr_storage as *mut sockaddr_in))
                                        .sin_addr as *mut in_addr as *mut libc::c_void,
                                    __constr_expr_3.as_mut_ptr() as *const libc::c_void,
                                    4 as libc::c_int as size_t,
                                );
                            }
                        }
                    }
                    _ => {}
                }
                if !service.is_null() {
                    mygetservbyname_r(
                        service,
                        0 as *mut libc::c_void as *const libc::c_char,
                        &mut se_buf,
                        buf___0.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                        &mut se,
                    );
                }
                if !se.is_null() {
                    port = (*se).s_port;
                } else {
                    if !service.is_null() {
                        tmp___2 = service;
                    } else {
                        tmp___2 = b"0\0" as *const u8 as *const libc::c_char;
                    }
                    tmp___3 = atoi(tmp___2);
                    tmp___4 = htons(tmp___3 as uint16_t);
                    port = tmp___4 as libc::c_int;
                }
                if af == 2 as libc::c_int {
                    (*(&mut (*space).sockaddr_space as *mut sockaddr_storage
                        as *mut sockaddr_in))
                        .sin_port = port as in_port_t;
                } else {
                    (*(&mut (*space).sockaddr_space as *mut sockaddr_storage
                        as *mut sockaddr_in6))
                        .sin6_port = port as in_port_t;
                }
                p = &mut (*space).addrinfo_space;
                *res = p;
                if !(p as size_t == space as size_t) {
                    __assert_fail(
                        b"(size_t)p == (size_t) space\0" as *const u8
                            as *const libc::c_char,
                        b"src/core.c\0" as *const u8 as *const libc::c_char,
                        1007 as libc::c_uint,
                        b"proxy_getaddrinfo\0" as *const u8 as *const libc::c_char,
                    );
                }
                (*p)
                    .ai_addr = &mut (*space).sockaddr_space as *mut sockaddr_storage
                    as *mut libc::c_void as *mut sockaddr;
                if !node.is_null() {
                    snprintf(
                        ((*space).addr_name).as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        node,
                    );
                }
                (*p).ai_canonname = ((*space).addr_name).as_mut_ptr();
                (*p).ai_next = 0 as *mut libc::c_void as *mut addrinfo;
                tmp___5 = af as sa_family_t;
                (*space).sockaddr_space.ss_family = tmp___5;
                (*p).ai_family = tmp___5 as libc::c_int;
                if af == 2 as libc::c_int {
                    (*p)
                        .ai_addrlen = ::std::mem::size_of::<sockaddr_in>()
                        as libc::c_ulong as socklen_t;
                } else {
                    (*p)
                        .ai_addrlen = ::std::mem::size_of::<sockaddr_in6>()
                        as libc::c_ulong as socklen_t;
                }
                if !hints.is_null() {
                    (*p).ai_socktype = (*hints).ai_socktype;
                    (*p).ai_flags = (*hints).ai_flags;
                    (*p).ai_protocol = (*hints).ai_protocol;
                    if (*p).ai_socktype == 0 {
                        if (*p).ai_protocol == 6 as libc::c_int {
                            (*p).ai_socktype = 1 as libc::c_int;
                        }
                    }
                } else {
                    (*p).ai_flags = 40 as libc::c_int;
                }
                return 0 as libc::c_int;
            }
        }
    }
    return 1 as libc::c_int;
}
pub static mut proxy_type_strmap: [*const libc::c_char; 3] = [
    b"http\0" as *const u8 as *const libc::c_char,
    b"socks4\0" as *const u8 as *const libc::c_char,
    b"socks5\0" as *const u8 as *const libc::c_char,
];
pub static mut chain_type_strmap: [*const libc::c_char; 4] = [
    b"dynamic_chain\0" as *const u8 as *const libc::c_char,
    b"strict_chain\0" as *const u8 as *const libc::c_char,
    b"random_chain\0" as *const u8 as *const libc::c_char,
    b"round_robin_chain\0" as *const u8 as *const libc::c_char,
];
pub static mut proxy_state_strmap: [*const libc::c_char; 4] = [
    b"play\0" as *const u8 as *const libc::c_char,
    b"down\0" as *const u8 as *const libc::c_char,
    b"blocked\0" as *const u8 as *const libc::c_char,
    b"busy\0" as *const u8 as *const libc::c_char,
];
pub unsafe extern "C" fn pc_isnumericipv4(
    mut ipstring: *const libc::c_char,
) -> libc::c_int {
    let mut x: size_t = 0;
    let mut n: size_t = 0;
    let mut d: size_t = 0;
    let mut wasdot: libc::c_int = 0;
    x = 0 as libc::c_int as size_t;
    n = 0 as libc::c_int as size_t;
    d = 0 as libc::c_int as size_t;
    wasdot = 0 as libc::c_int;
    loop {
        match *ipstring.offset(x as isize) as libc::c_int {
            0 => {
                break;
            }
            46 => {
                if n == 0 {
                    return 0 as libc::c_int
                } else {
                    if wasdot != 0 {
                        return 0 as libc::c_int;
                    }
                }
                d = d.wrapping_add(1);
                wasdot = 1 as libc::c_int;
            }
            57 | 56 | 55 | 54 | 53 | 52 | 51 | 50 | 49 | 48 => {
                n = n.wrapping_add(1);
                wasdot = 0 as libc::c_int;
            }
            _ => return 0 as libc::c_int,
        }
        x = x.wrapping_add(1);
    }
    if d == 3 as libc::c_ulong {
        if n >= 4 as libc::c_ulong {
            if n <= 12 as libc::c_ulong {
                return 1 as libc::c_int;
            }
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn pc_stringfromipv4(
    mut ip_buf_4_bytes: *mut libc::c_uchar,
    mut outbuf_16_bytes: *mut libc::c_char,
) {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut o: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_uchar = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    o = outbuf_16_bytes;
    p = ip_buf_4_bytes;
    while (p as libc::c_ulong)
        < ip_buf_4_bytes.offset(4 as libc::c_int as isize) as libc::c_ulong
    {
        n = *p;
        if *p as libc::c_int >= 100 as libc::c_int {
            if *p as libc::c_int >= 200 as libc::c_int {
                tmp = o;
                o = o.offset(1);
                *tmp = '2' as i32 as libc::c_char;
            } else {
                tmp___0 = o;
                o = o.offset(1);
                *tmp___0 = '1' as i32 as libc::c_char;
            }
            n = (n as libc::c_int % 100 as libc::c_int) as libc::c_uchar;
        }
        if *p as libc::c_int >= 10 as libc::c_int {
            tmp___1 = o;
            o = o.offset(1);
            *tmp___1 = (n as libc::c_int / 10 as libc::c_int + 48 as libc::c_int)
                as libc::c_char;
            n = (n as libc::c_int % 10 as libc::c_int) as libc::c_uchar;
        }
        tmp___2 = o;
        o = o.offset(1);
        *tmp___2 = (n as libc::c_int + 48 as libc::c_int) as libc::c_char;
        tmp___3 = o;
        o = o.offset(1);
        *tmp___3 = '.' as i32 as libc::c_char;
        p = p.offset(1);
    }
    *o.offset(-(1 as libc::c_int) as isize) = 0 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn check_path(mut path: *mut libc::c_char) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    if path.is_null() {
        return 0 as libc::c_int;
    }
    tmp = access(path as *const libc::c_char, 4 as libc::c_int);
    return (tmp != -(1 as libc::c_int)) as libc::c_int;
}
pub unsafe extern "C" fn get_config_path(
    mut default_path: *mut libc::c_char,
    mut pbuf: *mut libc::c_char,
    mut bufsize: size_t,
) -> *mut libc::c_char {
    let mut buf___0: [libc::c_char; 512] = [0; 512];
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    path = default_path;
    tmp = check_path(path);
    if !(tmp != 0) {
        path = getenv(b"PROXYCHAINS_CONF_FILE\0" as *const u8 as *const libc::c_char);
        tmp___0 = check_path(path);
        if !(tmp___0 != 0) {
            path = getcwd(
                buf___0.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
            );
            snprintf(
                pbuf,
                bufsize,
                b"%s/%s\0" as *const u8 as *const libc::c_char,
                path,
                b"proxychains.conf\0" as *const u8 as *const libc::c_char,
            );
            path = pbuf;
            tmp___1 = check_path(path);
            if !(tmp___1 != 0) {
                path = getenv(b"HOME\0" as *const u8 as *const libc::c_char);
                snprintf(
                    pbuf,
                    bufsize,
                    b"%s/.proxychains/%s\0" as *const u8 as *const libc::c_char,
                    path,
                    b"proxychains.conf\0" as *const u8 as *const libc::c_char,
                );
                path = pbuf;
                tmp___2 = check_path(path);
                if !(tmp___2 != 0) {
                    path = getenv(b"HOME\0" as *const u8 as *const libc::c_char);
                    snprintf(
                        pbuf,
                        bufsize,
                        b"%s/config/settings/%s\0" as *const u8 as *const libc::c_char,
                        path,
                        b"proxychains.conf\0" as *const u8 as *const libc::c_char,
                    );
                    path = pbuf;
                    tmp___3 = check_path(path);
                    if !(tmp___3 != 0) {
                        path = b"/usr/local/etc/proxychains.conf\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char;
                        tmp___4 = check_path(path);
                        if !(tmp___4 != 0) {
                            path = b"/etc/proxychains.conf\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char;
                            tmp___5 = check_path(path);
                            if !(tmp___5 != 0) {
                                perror(
                                    b"couldnt find configuration file\0" as *const u8
                                        as *const libc::c_char,
                                );
                                exit(1 as libc::c_int);
                            }
                        }
                    }
                }
            }
        }
    }
    return path;
}
pub static mut true_connect: Option::<
    unsafe extern "C" fn(libc::c_int, *const sockaddr, socklen_t) -> libc::c_int,
> = None;
pub static mut true_gethostbyname: Option::<
    unsafe extern "C" fn(*const libc::c_char) -> *mut hostent,
> = None;
pub static mut true_getaddrinfo: Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        *const libc::c_char,
        *const addrinfo,
        *mut *mut addrinfo,
    ) -> libc::c_int,
> = None;
pub static mut true_freeaddrinfo: Option::<
    unsafe extern "C" fn(*mut addrinfo) -> libc::c_int,
> = None;
pub static mut true_getnameinfo: Option::<
    unsafe extern "C" fn(
        *const sockaddr,
        socklen_t,
        *mut libc::c_char,
        socklen_t,
        *mut libc::c_char,
        socklen_t,
        libc::c_int,
    ) -> libc::c_int,
> = None;
pub static mut true_gethostbyaddr: Option::<
    unsafe extern "C" fn(*const libc::c_void, socklen_t, libc::c_int) -> *mut hostent,
> = None;
pub static mut true_close: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int> = None;
pub static mut true_close_range: Option::<
    unsafe extern "C" fn(libc::c_uint, libc::c_uint, libc::c_int) -> libc::c_int,
> = None;
pub static mut true_sendto: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        *const libc::c_void,
        size_t,
        libc::c_int,
        *const sockaddr,
        socklen_t,
    ) -> ssize_t,
> = None;
pub static mut tcp_read_time_out: libc::c_int = 0;
pub static mut tcp_connect_time_out: libc::c_int = 0;
pub static mut proxychains_ct: chain_type = DYNAMIC_TYPE;
pub static mut proxychains_pd: [proxy_data; 512] = [proxy_data {
    ip: ip_type {
        addr: __anonunion_addr_69655148 {
            v4: __anonunion_ip_type4_826858479 {
                octet: [0; 4],
            },
        },
        is_v6: 0,
    },
    port: 0,
    pt: HTTP_TYPE,
    ps: PLAY_STATE,
    user: [0; 256],
    pass: [0; 256],
}; 512];
pub static mut proxychains_proxy_count: libc::c_uint = 0 as libc::c_uint;
pub static mut proxychains_proxy_offset: libc::c_uint = 0 as libc::c_uint;
pub static mut proxychains_got_chain_data: libc::c_int = 0 as libc::c_int;
pub static mut proxychains_max_chain: libc::c_uint = 1 as libc::c_uint;
pub static mut proxychains_quiet_mode: libc::c_int = 0 as libc::c_int;
pub static mut proxychains_resolver: dns_lookup_flavor = DNSLF_LIBC;
pub static mut localnet_addr: [localaddr_arg; 64] = [localaddr_arg {
    family: 0,
    port: 0,
    __annonCompField6: __anonunion____missing_field_name_165330731 {
        __annonCompField4: __anonstruct____missing_field_name_625006857 {
            in_addr: in_addr { s_addr: 0 },
            in_mask: in_addr { s_addr: 0 },
        },
    },
}; 64];
pub static mut num_localnet_addr: size_t = 0 as libc::c_int as size_t;
pub static mut dnats: [dnat_arg; 64] = [dnat_arg {
    orig_dst: in_addr { s_addr: 0 },
    new_dst: in_addr { s_addr: 0 },
    orig_port: 0,
    new_port: 0,
}; 64];
pub static mut num_dnats: size_t = 0 as libc::c_int as size_t;
pub static mut remote_dns_subnet: libc::c_uint = 224 as libc::c_uint;
pub static mut init_once: pthread_once_t = 0 as libc::c_int;
static mut init_l: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn load_sym(
    mut symname: *mut libc::c_char,
    mut proxyfunc: *mut libc::c_void,
    mut is_mandatory: libc::c_int,
) -> *mut libc::c_void {
    let mut funcptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = dlsym(
        -(1 as libc::c_long) as *mut libc::c_void,
        symname as *const libc::c_char,
    );
    funcptr = tmp;
    if is_mandatory != 0 {
        if funcptr.is_null() {
            tmp___0 = dlerror();
            fprintf(
                stderr,
                b"Cannot load symbol '%s' %s\n\0" as *const u8 as *const libc::c_char,
                symname,
                tmp___0,
            );
            exit(1 as libc::c_int);
        }
    }
    if funcptr.is_null() {
        return funcptr;
    }
    if funcptr as libc::c_ulong == proxyfunc as libc::c_ulong {
        abort();
    }
    return funcptr;
}
static mut close_fds: [libc::c_int; 16] = [0; 16];
static mut close_fds_cnt: libc::c_int = 0 as libc::c_int;
static mut close_range_buffer: [close_range_args_t; 16] = [close_range_args_t {
    first: 0,
    last: 0,
    flags: 0,
}; 16];
static mut close_range_buffer_cnt: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn get_rand_seed() -> libc::c_uint {
    let mut now: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(0 as libc::c_int, &mut now);
    return (now.tv_sec ^ now.tv_nsec) as libc::c_uint;
}
unsafe extern "C" fn do_init() {
    let mut tmp: libc::c_uint = 0;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    tmp = get_rand_seed();
    srand(tmp);
    core_initialize();
    get_chain_data(
        proxychains_pd.as_mut_ptr(),
        &mut proxychains_proxy_count,
        &mut proxychains_ct,
    );
    tmp___0 = proxychains_get_version();
    proxychains_write_log(
        b"[proxychains] DLL init: proxychains-ng %s\n\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        tmp___0,
    );
    setup_hooks();
    while close_fds_cnt != 0 {
        close_fds_cnt -= 1;
        (Some(true_close.expect("non-null function pointer")))
            .expect("non-null function pointer")(close_fds[close_fds_cnt as usize]);
    }
    while close_range_buffer_cnt != 0 {
        close_range_buffer_cnt -= 1;
        i = close_range_buffer_cnt;
        (Some(true_close_range.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            close_range_buffer[i as usize].first,
            close_range_buffer[i as usize].last,
            close_range_buffer[i as usize].flags as libc::c_int,
        );
    }
    init_l = 1 as libc::c_int;
    rdns_init(proxychains_resolver);
}
unsafe extern "C" fn init_lib_wrapper(mut caller: *const libc::c_char) {
    init_l == 0;
    pthread_once(&mut init_once, Some(do_init as unsafe extern "C" fn() -> ()));
}
unsafe extern "C" fn proxy_from_string(
    mut proxystring: *const libc::c_char,
    mut type_buf: *mut libc::c_char,
    mut host_buf: *mut libc::c_char,
    mut port_n: *mut libc::c_int,
    mut user_buf: *mut libc::c_char,
    mut pass_buf: *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut proxytype: rs_proxyType = RS_PT_NONE;
    let mut next_token: size_t = 0;
    let mut ul: size_t = 0;
    let mut pl: size_t = 0;
    let mut hl: size_t = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: size_t = 0;
    let mut at: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut u: *const libc::c_char = 0 as *const libc::c_char;
    let mut h: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    next_token = 6 as libc::c_int as size_t;
    ul = 0 as libc::c_int as size_t;
    pl = 0 as libc::c_int as size_t;
    if !(*proxystring.offset(0 as libc::c_int as isize) == 0) {
        if !(*proxystring.offset(1 as libc::c_int as isize) == 0) {
            if !(*proxystring.offset(2 as libc::c_int as isize) == 0) {
                if !(*proxystring.offset(3 as libc::c_int as isize) == 0) {
                    if !(*proxystring.offset(4 as libc::c_int as isize) == 0) {
                        if !(*proxystring.offset(5 as libc::c_int as isize) == 0) {
                            if *proxystring as libc::c_int == 115 as libc::c_int {
                                match *proxystring.offset(5 as libc::c_int as isize)
                                    as libc::c_int
                                {
                                    53 => {
                                        current_block = 16059836657911256072;
                                        match current_block {
                                            8506069866241663125 => {
                                                proxytype = RS_PT_SOCKS4;
                                            }
                                            _ => {
                                                proxytype = RS_PT_SOCKS5;
                                            }
                                        }
                                        current_block = 11057878835866523405;
                                    }
                                    52 => {
                                        current_block = 8506069866241663125;
                                        match current_block {
                                            8506069866241663125 => {
                                                proxytype = RS_PT_SOCKS4;
                                            }
                                            _ => {
                                                proxytype = RS_PT_SOCKS5;
                                            }
                                        }
                                        current_block = 11057878835866523405;
                                    }
                                    _ => {
                                        current_block = 2408497934441841405;
                                    }
                                }
                            } else if *proxystring as libc::c_int == 104 as libc::c_int {
                                proxytype = RS_PT_HTTP;
                                next_token = 4 as libc::c_int as size_t;
                                current_block = 11057878835866523405;
                            } else {
                                current_block = 2408497934441841405;
                            }
                            match current_block {
                                2408497934441841405 => {}
                                _ => {
                                    tmp = next_token;
                                    next_token = next_token.wrapping_add(1);
                                    if !(*proxystring.offset(tmp as isize) as libc::c_int
                                        != 58 as libc::c_int)
                                    {
                                        tmp___0 = next_token;
                                        next_token = next_token.wrapping_add(1);
                                        if !(*proxystring.offset(tmp___0 as isize) as libc::c_int
                                            != 47 as libc::c_int)
                                        {
                                            tmp___1 = next_token;
                                            next_token = next_token.wrapping_add(1);
                                            if !(*proxystring.offset(tmp___1 as isize) as libc::c_int
                                                != 47 as libc::c_int)
                                            {
                                                tmp___2 = strrchr(
                                                    proxystring.offset(next_token as isize),
                                                    '@' as i32,
                                                );
                                                at = tmp___2 as *const libc::c_char;
                                                if !at.is_null() {
                                                    if proxytype as libc::c_uint == 1 as libc::c_uint {
                                                        return 0 as libc::c_int;
                                                    }
                                                    tmp___3 = strchr(
                                                        proxystring.offset(next_token as isize),
                                                        ':' as i32,
                                                    );
                                                    p = tmp___3 as *const libc::c_char;
                                                    if p.is_null() {
                                                        current_block = 2408497934441841405;
                                                    } else if p as libc::c_ulong >= at as libc::c_ulong {
                                                        current_block = 2408497934441841405;
                                                    } else {
                                                        u = proxystring.offset(next_token as isize);
                                                        ul = p.offset_from(u) as libc::c_long as size_t;
                                                        p = p.offset(1);
                                                        pl = at.offset_from(p) as libc::c_long as size_t;
                                                        if proxytype as libc::c_uint == 2 as libc::c_uint {
                                                            if ul > 255 as libc::c_ulong {
                                                                return 0 as libc::c_int
                                                            } else {
                                                                if pl > 255 as libc::c_ulong {
                                                                    return 0 as libc::c_int;
                                                                }
                                                            }
                                                        }
                                                        memcpy(
                                                            user_buf as *mut libc::c_void,
                                                            u as *const libc::c_void,
                                                            ul,
                                                        );
                                                        *user_buf
                                                            .offset(ul as isize) = 0 as libc::c_int as libc::c_char;
                                                        memcpy(
                                                            pass_buf as *mut libc::c_void,
                                                            p as *const libc::c_void,
                                                            pl,
                                                        );
                                                        *pass_buf
                                                            .offset(pl as isize) = 0 as libc::c_int as libc::c_char;
                                                        next_token = (next_token as libc::c_ulong)
                                                            .wrapping_add(
                                                                (2 as libc::c_ulong).wrapping_add(ul).wrapping_add(pl),
                                                            ) as size_t as size_t;
                                                        current_block = 9859671972921157070;
                                                    }
                                                } else {
                                                    *user_buf
                                                        .offset(
                                                            0 as libc::c_int as isize,
                                                        ) = 0 as libc::c_int as libc::c_char;
                                                    *pass_buf
                                                        .offset(
                                                            0 as libc::c_int as isize,
                                                        ) = 0 as libc::c_int as libc::c_char;
                                                    current_block = 9859671972921157070;
                                                }
                                                match current_block {
                                                    2408497934441841405 => {}
                                                    _ => {
                                                        h = proxystring.offset(next_token as isize);
                                                        tmp___4 = strchr(h, ':' as i32);
                                                        p = tmp___4 as *const libc::c_char;
                                                        if !p.is_null() {
                                                            hl = p.offset_from(h) as libc::c_long as size_t;
                                                            if hl > 255 as libc::c_ulong {
                                                                return 0 as libc::c_int;
                                                            }
                                                            memcpy(
                                                                host_buf as *mut libc::c_void,
                                                                h as *const libc::c_void,
                                                                hl,
                                                            );
                                                            *host_buf
                                                                .offset(hl as isize) = 0 as libc::c_int as libc::c_char;
                                                            *port_n = atoi(p.offset(1 as libc::c_int as isize));
                                                            match proxytype as libc::c_uint {
                                                                1 => {
                                                                    strcpy(
                                                                        type_buf,
                                                                        b"socks4\0" as *const u8 as *const libc::c_char,
                                                                    );
                                                                }
                                                                2 => {
                                                                    strcpy(
                                                                        type_buf,
                                                                        b"socks5\0" as *const u8 as *const libc::c_char,
                                                                    );
                                                                }
                                                                3 => {
                                                                    strcpy(
                                                                        type_buf,
                                                                        b"http\0" as *const u8 as *const libc::c_char,
                                                                    );
                                                                }
                                                                _ => return 0 as libc::c_int,
                                                            }
                                                            return 1 as libc::c_int;
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
    return 0 as libc::c_int;
}
unsafe extern "C" fn bool_str(mut bool_val: libc::c_int) -> *const libc::c_char {
    if bool_val != 0 {
        return b"true\0" as *const u8 as *const libc::c_char;
    }
    return b"false\0" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn get_chain_data(
    mut pd: *mut proxy_data,
    mut proxy_count: *mut libc::c_uint,
    mut ct: *mut chain_type,
) {
    let mut current_block: u64;
    let mut count: libc::c_int = 0;
    let mut port_n: libc::c_int = 0;
    let mut list___0: libc::c_int = 0;
    let mut buf___0: [libc::c_char; 1024] = [0; 1024];
    let mut type_0: [libc::c_char; 1024] = [0; 1024];
    let mut host: [libc::c_char; 1024] = [0; 1024];
    let mut user: [libc::c_char; 1024] = [0; 1024];
    let mut buff: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut env: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut local_addr_port: [libc::c_char; 64] = [0; 64];
    let mut local_addr: [libc::c_char; 64] = [0; 64];
    let mut local_netmask: [libc::c_char; 32] = [0; 32];
    let mut dnat_orig_addr_port: [libc::c_char; 32] = [0; 32];
    let mut dnat_new_addr_port: [libc::c_char; 32] = [0; 32];
    let mut dnat_orig_addr: [libc::c_char; 32] = [0; 32];
    let mut dnat_orig_port: [libc::c_char; 32] = [0; 32];
    let mut dnat_new_addr: [libc::c_char; 32] = [0; 32];
    let mut dnat_new_port: [libc::c_char; 32] = [0; 32];
    let mut rdnsd_addr: [libc::c_char; 32] = [0; 32];
    let mut rdnsd_port: [libc::c_char; 8] = [0; 8];
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut ret: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___7: libc::c_int = 0;
    let mut host_ip: *mut ip_type = 0 as *mut ip_type;
    let mut internal_ip: ip_type4 = __anonunion_ip_type4_826858479 {
        octet: [0; 4],
    };
    let mut tmp___8: size_t = 0;
    let mut tmp___9: ip_type4 = __anonunion_ip_type4_826858479 {
        octet: [0; 4],
    };
    let mut __constr_expr_4: ip_type4 = __anonunion_ip_type4_826858479 {
        octet: [0; 4],
    };
    let mut tmp___10: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___11: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___12: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___13: libc::c_int = 0;
    let mut tmp___14: libc::c_int = 0;
    let mut tmp___15: libc::c_int = 0;
    let mut tmp___16: libc::c_int = 0;
    let mut tmp___17: libc::c_int = 0;
    let mut tmp___18: libc::c_int = 0;
    let mut colon: libc::c_char = 0;
    let mut extra: libc::c_char = 0;
    let mut right_bracket: [libc::c_char; 2] = [0; 2];
    let mut local_port: libc::c_ushort = 0;
    let mut local_prefix: libc::c_ushort = 0;
    let mut local_family: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut valid: libc::c_int = 0;
    let mut tmp___19: libc::c_int = 0;
    let mut tmp___20: libc::c_int = 0;
    let mut tmp___21: libc::c_int = 0;
    let mut tmp___22: libc::c_int = 0;
    let mut tmp___23: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___24: libc::c_int = 0;
    let mut tmp___25: libc::c_int = 0;
    let mut tmp___26: libc::c_int = 0;
    let mut tmp___27: libc::c_int = 0;
    let mut tmp___28: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut rdns_server_buffer: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut tmp___29: libc::c_int = 0;
    let mut error: libc::c_int = 0;
    let mut tmp___30: libc::c_int = 0;
    let mut tmp___31: libc::c_int = 0;
    let mut tmp___32: libc::c_int = 0;
    let mut error___0: libc::c_int = 0;
    let mut tmp___33: libc::c_int = 0;
    let mut tmp___34: libc::c_int = 0;
    let mut tmp___35: libc::c_int = 0;
    let mut tmp___36: libc::c_int = 0;
    let mut tmp___37: libc::c_int = 0;
    let mut tmp___38: libc::c_int = 0;
    let mut tmp___39: libc::c_int = 0;
    let mut tmp___40: libc::c_int = 0;
    let mut tmp___41: libc::c_int = 0;
    let mut tmp___42: libc::c_int = 0;
    let mut tmp___43: libc::c_int = 0;
    let mut tmp___44: libc::c_int = 0;
    let mut tmp___45: libc::c_int = 0;
    let mut tmp___46: libc::c_int = 0;
    let mut tmp___47: libc::c_int = 0;
    let mut tmp___48: libc::c_int = 0;
    let mut tmp___49: libc::c_int = 0;
    let mut tmp___50: *mut libc::c_char = 0 as *mut libc::c_char;
    count = 0 as libc::c_int;
    port_n = 0 as libc::c_int;
    list___0 = 0 as libc::c_int;
    file = 0 as *mut libc::c_void as *mut FILE;
    if proxychains_got_chain_data != 0 {
        return;
    }
    tcp_read_time_out = 4000 as libc::c_int;
    tcp_connect_time_out = 10000 as libc::c_int;
    *ct = DYNAMIC_TYPE;
    tmp = getenv(b"PROXYCHAINS_CONF_FILE\0" as *const u8 as *const libc::c_char);
    env = get_config_path(
        tmp,
        buf___0.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    file = fopen(env as *const libc::c_char, b"r\0" as *const u8 as *const libc::c_char);
    if file as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        perror(b"couldnt read configuration file\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    env = getenv(b"PROXYCHAINS_QUIET_MODE\0" as *const u8 as *const libc::c_char);
    if !env.is_null() {
        if *env as libc::c_int == 49 as libc::c_int {
            proxychains_quiet_mode = 1 as libc::c_int;
        }
    }
    loop {
        tmp___50 = fgets(
            buf___0.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                as libc::c_int,
            file,
        );
        if tmp___50.is_null() {
            break;
        }
        buff = buf___0.as_mut_ptr();
        loop {
            tmp___0 = __ctype_b_loc();
            if *(*tmp___0).offset(*buff as libc::c_int as isize) as libc::c_int
                & 8192 as libc::c_int == 0
            {
                break;
            }
            buff = buff.offset(1);
        }
        p = strrchr(buff as *const libc::c_char, '\n' as i32);
        if !p.is_null() {
            *p = 0 as libc::c_int as libc::c_char;
        }
        tmp___1 = strlen(buff as *const libc::c_char);
        p = buff.offset(tmp___1 as isize).offset(-(1 as libc::c_int as isize));
        while p as libc::c_ulong >= buff as libc::c_ulong {
            tmp___3 = __ctype_b_loc();
            if *(*tmp___3).offset(*p as libc::c_int as isize) as libc::c_int
                & 8192 as libc::c_int == 0
            {
                break;
            }
            tmp___2 = p;
            p = p.offset(-1);
            *tmp___2 = 0 as libc::c_int as libc::c_char;
        }
        if *buff == 0 {
            continue;
        }
        if *buff as libc::c_int == 35 as libc::c_int {
            continue;
        }
        if list___0 != 0 {
            if count >= 512 as libc::c_int {
                break;
            }
            memset(
                pd.offset(count as isize) as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<proxy_data>() as libc::c_ulong,
            );
            (*pd.offset(count as isize)).ps = PLAY_STATE;
            port_n = 0 as libc::c_int;
            tmp___4 = sscanf(
                buff as *const libc::c_char,
                b"%s %s %d %s %s\0" as *const u8 as *const libc::c_char,
                type_0.as_mut_ptr(),
                host.as_mut_ptr(),
                &mut port_n as *mut libc::c_int,
                ((*pd.offset(count as isize)).user).as_mut_ptr(),
                ((*pd.offset(count as isize)).pass).as_mut_ptr(),
            );
            ret = tmp___4;
            if ret < 3 as libc::c_int {
                current_block = 1415260729884300661;
            } else if ret == -(1 as libc::c_int) {
                current_block = 1415260729884300661;
            } else {
                current_block = 12070711452894729854;
            }
            match current_block {
                1415260729884300661 => {
                    tmp___5 = proxy_from_string(
                        buff as *const libc::c_char,
                        type_0.as_mut_ptr(),
                        host.as_mut_ptr(),
                        &mut port_n,
                        ((*pd.offset(count as isize)).user).as_mut_ptr(),
                        ((*pd.offset(count as isize)).pass).as_mut_ptr(),
                    );
                    if tmp___5 == 0 {
                        current_block = 13088741992317914500;
                    } else {
                        current_block = 12070711452894729854;
                    }
                }
                _ => {}
            }
            match current_block {
                12070711452894729854 => {
                    memset(
                        &mut (*pd.offset(count as isize)).ip as *mut ip_type
                            as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<ip_type>() as libc::c_ulong,
                    );
                    tmp___6 = strchr(
                        host.as_mut_ptr() as *const libc::c_char,
                        ':' as i32,
                    );
                    if !tmp___6.is_null() {
                        tmp___7 = 1 as libc::c_int;
                    } else {
                        tmp___7 = 0 as libc::c_int;
                    }
                    (*pd.offset(count as isize)).ip.is_v6 = tmp___7 as libc::c_char;
                    (*pd.offset(count as isize)).port = htons(port_n as libc::c_ushort);
                    host_ip = &mut (*pd.offset(count as isize)).ip;
                    if (*host_ip).is_v6 != 0 {
                        tmp___13 = 10 as libc::c_int;
                    } else {
                        tmp___13 = 2 as libc::c_int;
                    }
                    tmp___14 = inet_pton(
                        tmp___13,
                        host.as_mut_ptr() as *const libc::c_char,
                        ((*host_ip).addr.v6).as_mut_ptr() as *mut libc::c_void,
                    );
                    if 1 as libc::c_int != tmp___14 {
                        if *ct as libc::c_uint == 1 as libc::c_uint {
                            if proxychains_resolver as libc::c_uint >= 2 as libc::c_uint
                            {
                                if count > 0 as libc::c_int {
                                    rdns_init(proxychains_resolver);
                                    tmp___8 = strlen(host.as_mut_ptr() as *const libc::c_char);
                                    tmp___9 = at_get_ip_for_host(host.as_mut_ptr(), tmp___8);
                                    internal_ip = tmp___9;
                                    (*pd.offset(count as isize))
                                        .ip
                                        .is_v6 = 0 as libc::c_int as libc::c_char;
                                    (*host_ip).addr.v4 = internal_ip;
                                    __constr_expr_4.as_int = -(1 as libc::c_int) as uint32_t;
                                    if internal_ip.as_int == __constr_expr_4.as_int {
                                        current_block = 966533126182163892;
                                    } else {
                                        current_block = 17808209642927821499;
                                    }
                                } else {
                                    current_block = 966533126182163892;
                                }
                            } else {
                                current_block = 966533126182163892;
                            }
                        } else {
                            current_block = 966533126182163892;
                        }
                        match current_block {
                            17808209642927821499 => {}
                            _ => {
                                fprintf(
                                    stderr,
                                    b"proxy %s has invalid value or is not numeric\n\0"
                                        as *const u8 as *const libc::c_char,
                                    host.as_mut_ptr(),
                                );
                                fprintf(
                                    stderr,
                                    b"non-numeric ips are only allowed under the following circumstances:\n\0"
                                        as *const u8 as *const libc::c_char,
                                );
                                tmp___10 = rdns_resolver_string(proxychains_resolver);
                                tmp___11 = bool_str(
                                    (count > 0 as libc::c_int) as libc::c_int,
                                );
                                tmp___12 = bool_str(
                                    (*ct as libc::c_uint == 1 as libc::c_uint) as libc::c_int,
                                );
                                fprintf(
                                    stderr,
                                    b"chaintype == strict (%s), proxy is not first in list (%s), proxy_dns active (%s)\n\n\0"
                                        as *const u8 as *const libc::c_char,
                                    tmp___12,
                                    tmp___11,
                                    tmp___10,
                                );
                                exit(1 as libc::c_int);
                            }
                        }
                    }
                    tmp___18 = strcmp(
                        type_0.as_mut_ptr() as *const libc::c_char,
                        b"http\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___18 != 0 {
                        tmp___17 = strcmp(
                            type_0.as_mut_ptr() as *const libc::c_char,
                            b"raw\0" as *const u8 as *const libc::c_char,
                        );
                        if tmp___17 != 0 {
                            tmp___16 = strcmp(
                                type_0.as_mut_ptr() as *const libc::c_char,
                                b"socks4\0" as *const u8 as *const libc::c_char,
                            );
                            if tmp___16 != 0 {
                                tmp___15 = strcmp(
                                    type_0.as_mut_ptr() as *const libc::c_char,
                                    b"socks5\0" as *const u8 as *const libc::c_char,
                                );
                                if tmp___15 != 0 {
                                    current_block = 13088741992317914500;
                                } else {
                                    (*pd.offset(count as isize)).pt = SOCKS5_TYPE;
                                    current_block = 914440069034635393;
                                }
                            } else {
                                (*pd.offset(count as isize)).pt = SOCKS4_TYPE;
                                current_block = 914440069034635393;
                            }
                        } else {
                            (*pd.offset(count as isize)).pt = RAW_TYPE;
                            current_block = 914440069034635393;
                        }
                    } else {
                        (*pd.offset(count as isize)).pt = HTTP_TYPE;
                        current_block = 914440069034635393;
                    }
                    match current_block {
                        13088741992317914500 => {}
                        _ => {
                            if port_n != 0 {
                                count += 1;
                            }
                            continue;
                        }
                    }
                }
                _ => {}
            }
            fprintf(
                stderr,
                b"error: invalid item in proxylist section: %s\0" as *const u8
                    as *const libc::c_char,
                buff,
            );
            exit(1 as libc::c_int);
        } else {
            tmp___49 = strcmp(
                buff as *const libc::c_char,
                b"[ProxyList]\0" as *const u8 as *const libc::c_char,
            );
            if tmp___49 != 0 {
                tmp___48 = strcmp(
                    buff as *const libc::c_char,
                    b"random_chain\0" as *const u8 as *const libc::c_char,
                );
                if tmp___48 != 0 {
                    tmp___47 = strcmp(
                        buff as *const libc::c_char,
                        b"strict_chain\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___47 != 0 {
                        tmp___46 = strcmp(
                            buff as *const libc::c_char,
                            b"dynamic_chain\0" as *const u8 as *const libc::c_char,
                        );
                        if tmp___46 != 0 {
                            tmp___45 = strcmp(
                                buff as *const libc::c_char,
                                b"round_robin_chain\0" as *const u8 as *const libc::c_char,
                            );
                            if tmp___45 != 0 {
                                tmp___44 = strncmp(
                                    buff as *const libc::c_char,
                                    b"tcp_read_time_out\0" as *const u8 as *const libc::c_char,
                                    (::std::mem::size_of::<[libc::c_char; 18]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_ulong),
                                );
                                if tmp___44 != 0 {
                                    tmp___43 = strncmp(
                                        buff as *const libc::c_char,
                                        b"tcp_connect_time_out\0" as *const u8
                                            as *const libc::c_char,
                                        (::std::mem::size_of::<[libc::c_char; 21]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_ulong),
                                    );
                                    if tmp___43 != 0 {
                                        tmp___42 = strncmp(
                                            buff as *const libc::c_char,
                                            b"remote_dns_subnet\0" as *const u8 as *const libc::c_char,
                                            (::std::mem::size_of::<[libc::c_char; 18]>()
                                                as libc::c_ulong)
                                                .wrapping_sub(1 as libc::c_ulong),
                                        );
                                        if tmp___42 != 0 {
                                            tmp___41 = strncmp(
                                                buff as *const libc::c_char,
                                                b"localnet\0" as *const u8 as *const libc::c_char,
                                                (::std::mem::size_of::<[libc::c_char; 9]>()
                                                    as libc::c_ulong)
                                                    .wrapping_sub(1 as libc::c_ulong),
                                            );
                                            if tmp___41 != 0 {
                                                tmp___40 = strncmp(
                                                    buff as *const libc::c_char,
                                                    b"chain_len\0" as *const u8 as *const libc::c_char,
                                                    (::std::mem::size_of::<[libc::c_char; 10]>()
                                                        as libc::c_ulong)
                                                        .wrapping_sub(1 as libc::c_ulong),
                                                );
                                                if tmp___40 != 0 {
                                                    tmp___39 = strcmp(
                                                        buff as *const libc::c_char,
                                                        b"quiet_mode\0" as *const u8 as *const libc::c_char,
                                                    );
                                                    if tmp___39 != 0 {
                                                        tmp___38 = strcmp(
                                                            buff as *const libc::c_char,
                                                            b"proxy_dns_old\0" as *const u8 as *const libc::c_char,
                                                        );
                                                        if tmp___38 != 0 {
                                                            tmp___37 = strcmp(
                                                                buff as *const libc::c_char,
                                                                b"proxy_dns\0" as *const u8 as *const libc::c_char,
                                                            );
                                                            if tmp___37 != 0 {
                                                                tmp___36 = strncmp(
                                                                    buff as *const libc::c_char,
                                                                    b"proxy_dns_daemon\0" as *const u8 as *const libc::c_char,
                                                                    (::std::mem::size_of::<[libc::c_char; 17]>()
                                                                        as libc::c_ulong)
                                                                        .wrapping_sub(1 as libc::c_ulong),
                                                                );
                                                                if tmp___36 != 0 {
                                                                    tmp___35 = strncmp(
                                                                        buff as *const libc::c_char,
                                                                        b"dnat\0" as *const u8 as *const libc::c_char,
                                                                        (::std::mem::size_of::<[libc::c_char; 5]>()
                                                                            as libc::c_ulong)
                                                                            .wrapping_sub(1 as libc::c_ulong),
                                                                    );
                                                                    if tmp___35 == 0 {
                                                                        tmp___32 = sscanf(
                                                                            buff as *const libc::c_char,
                                                                            b"%s %21[^ ] %21s\n\0" as *const u8 as *const libc::c_char,
                                                                            user.as_mut_ptr(),
                                                                            dnat_orig_addr_port.as_mut_ptr(),
                                                                            dnat_new_addr_port.as_mut_ptr(),
                                                                        );
                                                                        if tmp___32 < 3 as libc::c_int {
                                                                            fprintf(
                                                                                stderr,
                                                                                b"dnat format error\0" as *const u8 as *const libc::c_char,
                                                                            );
                                                                            exit(1 as libc::c_int);
                                                                        }
                                                                        memset(
                                                                            dnat_orig_port.as_mut_ptr() as *mut libc::c_void,
                                                                            0 as libc::c_int,
                                                                            (::std::mem::size_of::<[libc::c_char; 32]>()
                                                                                as libc::c_ulong)
                                                                                .wrapping_div(
                                                                                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                                                                ),
                                                                        );
                                                                        memset(
                                                                            dnat_new_port.as_mut_ptr() as *mut libc::c_void,
                                                                            0 as libc::c_int,
                                                                            (::std::mem::size_of::<[libc::c_char; 32]>()
                                                                                as libc::c_ulong)
                                                                                .wrapping_div(
                                                                                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                                                                ),
                                                                        );
                                                                        sscanf(
                                                                            dnat_orig_addr_port.as_mut_ptr() as *const libc::c_char,
                                                                            b"%15[^:]:%5s\0" as *const u8 as *const libc::c_char,
                                                                            dnat_orig_addr.as_mut_ptr(),
                                                                            dnat_orig_port.as_mut_ptr(),
                                                                        );
                                                                        sscanf(
                                                                            dnat_new_addr_port.as_mut_ptr() as *const libc::c_char,
                                                                            b"%15[^:]:%5s\0" as *const u8 as *const libc::c_char,
                                                                            dnat_new_addr.as_mut_ptr(),
                                                                            dnat_new_port.as_mut_ptr(),
                                                                        );
                                                                        if num_dnats < 64 as libc::c_ulong {
                                                                            error___0 = inet_pton(
                                                                                2 as libc::c_int,
                                                                                dnat_orig_addr.as_mut_ptr() as *const libc::c_char,
                                                                                &mut (*dnats.as_mut_ptr().offset(num_dnats as isize))
                                                                                    .orig_dst as *mut in_addr as *mut libc::c_void,
                                                                            );
                                                                            if error___0 <= 0 as libc::c_int {
                                                                                fprintf(
                                                                                    stderr,
                                                                                    b"dnat original destination address error\n\0" as *const u8
                                                                                        as *const libc::c_char,
                                                                                );
                                                                                exit(1 as libc::c_int);
                                                                            }
                                                                            error___0 = inet_pton(
                                                                                2 as libc::c_int,
                                                                                dnat_new_addr.as_mut_ptr() as *const libc::c_char,
                                                                                &mut (*dnats.as_mut_ptr().offset(num_dnats as isize))
                                                                                    .new_dst as *mut in_addr as *mut libc::c_void,
                                                                            );
                                                                            if error___0 <= 0 as libc::c_int {
                                                                                fprintf(
                                                                                    stderr,
                                                                                    b"dnat effective destination address error\n\0" as *const u8
                                                                                        as *const libc::c_char,
                                                                                );
                                                                                exit(1 as libc::c_int);
                                                                            }
                                                                            if dnat_orig_port[0 as libc::c_int as usize] != 0 {
                                                                                tmp___33 = atoi(
                                                                                    dnat_orig_port.as_mut_ptr() as *const libc::c_char,
                                                                                );
                                                                                dnats[num_dnats as usize]
                                                                                    .orig_port = tmp___33 as libc::c_short as libc::c_ushort;
                                                                            } else {
                                                                                dnats[num_dnats as usize]
                                                                                    .orig_port = 0 as libc::c_int as libc::c_ushort;
                                                                            }
                                                                            if dnat_new_port[0 as libc::c_int as usize] != 0 {
                                                                                tmp___34 = atoi(
                                                                                    dnat_new_port.as_mut_ptr() as *const libc::c_char,
                                                                                );
                                                                                dnats[num_dnats as usize]
                                                                                    .new_port = tmp___34 as libc::c_short as libc::c_ushort;
                                                                            } else {
                                                                                dnats[num_dnats as usize]
                                                                                    .new_port = 0 as libc::c_int as libc::c_ushort;
                                                                            }
                                                                            num_dnats = num_dnats.wrapping_add(1);
                                                                        } else {
                                                                            fprintf(
                                                                                stderr,
                                                                                b"# of dnat exceed %d.\n\0" as *const u8
                                                                                    as *const libc::c_char,
                                                                                64 as libc::c_int,
                                                                            );
                                                                        }
                                                                    }
                                                                } else {
                                                                    tmp___29 = sscanf(
                                                                        buff as *const libc::c_char,
                                                                        b"%s %15[^:]:%5s\0" as *const u8 as *const libc::c_char,
                                                                        user.as_mut_ptr(),
                                                                        rdnsd_addr.as_mut_ptr(),
                                                                        rdnsd_port.as_mut_ptr(),
                                                                    );
                                                                    if tmp___29 < 3 as libc::c_int {
                                                                        fprintf(
                                                                            stderr,
                                                                            b"proxy_dns_daemon format error\n\0" as *const u8
                                                                                as *const libc::c_char,
                                                                        );
                                                                        exit(1 as libc::c_int);
                                                                    }
                                                                    rdns_server_buffer
                                                                        .sin_family = 2 as libc::c_int as sa_family_t;
                                                                    tmp___30 = inet_pton(
                                                                        2 as libc::c_int,
                                                                        rdnsd_addr.as_mut_ptr() as *const libc::c_char,
                                                                        &mut rdns_server_buffer.sin_addr as *mut in_addr
                                                                            as *mut libc::c_void,
                                                                    );
                                                                    error = tmp___30;
                                                                    if error <= 0 as libc::c_int {
                                                                        fprintf(
                                                                            stderr,
                                                                            b"bogus proxy_dns_daemon address\n\0" as *const u8
                                                                                as *const libc::c_char,
                                                                        );
                                                                        exit(1 as libc::c_int);
                                                                    }
                                                                    tmp___31 = atoi(
                                                                        rdnsd_port.as_mut_ptr() as *const libc::c_char,
                                                                    );
                                                                    rdns_server_buffer.sin_port = htons(tmp___31 as uint16_t);
                                                                    proxychains_resolver = DNSLF_RDNS_DAEMON;
                                                                    rdns_set_daemon(&mut rdns_server_buffer);
                                                                }
                                                            } else {
                                                                proxychains_resolver = DNSLF_RDNS_START;
                                                            }
                                                        } else {
                                                            proxychains_resolver = DNSLF_FORKEXEC;
                                                        }
                                                    } else {
                                                        proxychains_quiet_mode = 1 as libc::c_int;
                                                    }
                                                } else {
                                                    pc = strchr(buff as *const libc::c_char, '=' as i32);
                                                    if pc.is_null() {
                                                        fprintf(
                                                            stderr,
                                                            b"error: missing equals sign '=' in chain_len directive.\n\0"
                                                                as *const u8 as *const libc::c_char,
                                                        );
                                                        exit(1 as libc::c_int);
                                                    }
                                                    pc = pc.offset(1);
                                                    len = atoi(pc as *const libc::c_char);
                                                    if len != 0 {
                                                        proxychains_max_chain = len as libc::c_uint;
                                                    } else {
                                                        proxychains_max_chain = 1 as libc::c_uint;
                                                    }
                                                }
                                            } else {
                                                local_port = 0 as libc::c_int as libc::c_ushort;
                                                tmp___19 = sscanf(
                                                    buff as *const libc::c_char,
                                                    b"%s %53[^/]/%15s%c\0" as *const u8 as *const libc::c_char,
                                                    user.as_mut_ptr(),
                                                    local_addr_port.as_mut_ptr(),
                                                    local_netmask.as_mut_ptr(),
                                                    &mut extra as *mut libc::c_char,
                                                );
                                                if tmp___19 != 3 as libc::c_int {
                                                    fprintf(
                                                        stderr,
                                                        b"localnet format error\0" as *const u8
                                                            as *const libc::c_char,
                                                    );
                                                    exit(1 as libc::c_int);
                                                }
                                                p = strchr(
                                                    local_addr_port.as_mut_ptr() as *const libc::c_char,
                                                    ':' as i32,
                                                );
                                                let mut current_block_238: u64;
                                                if p.is_null() {
                                                    current_block_238 = 799969075006691772;
                                                } else {
                                                    tmp___23 = strrchr(
                                                        local_addr_port.as_mut_ptr() as *const libc::c_char,
                                                        ':' as i32,
                                                    );
                                                    if p as libc::c_ulong == tmp___23 as libc::c_ulong {
                                                        current_block_238 = 799969075006691772;
                                                    } else {
                                                        if local_addr_port[0 as libc::c_int as usize] as libc::c_int
                                                            == 91 as libc::c_int
                                                        {
                                                            local_family = 10 as libc::c_int;
                                                            n = sscanf(
                                                                local_addr_port.as_mut_ptr() as *const libc::c_char,
                                                                b"[%45[^][]%1[]]%c%5hu%c\0" as *const u8
                                                                    as *const libc::c_char,
                                                                local_addr.as_mut_ptr(),
                                                                right_bracket.as_mut_ptr(),
                                                                &mut colon as *mut libc::c_char,
                                                                &mut local_port as *mut libc::c_ushort,
                                                                &mut extra as *mut libc::c_char,
                                                            );
                                                            if n == 2 as libc::c_int {
                                                                tmp___21 = 1 as libc::c_int;
                                                            } else if n == 4 as libc::c_int {
                                                                if colon as libc::c_int == 58 as libc::c_int {
                                                                    tmp___21 = 1 as libc::c_int;
                                                                } else {
                                                                    tmp___21 = 0 as libc::c_int;
                                                                }
                                                            } else {
                                                                tmp___21 = 0 as libc::c_int;
                                                            }
                                                            valid = tmp___21;
                                                        } else {
                                                            local_family = 10 as libc::c_int;
                                                            tmp___22 = sscanf(
                                                                local_addr_port.as_mut_ptr() as *const libc::c_char,
                                                                b"%45[^][]%c\0" as *const u8 as *const libc::c_char,
                                                                local_addr.as_mut_ptr(),
                                                                &mut extra as *mut libc::c_char,
                                                            );
                                                            valid = (tmp___22 == 1 as libc::c_int) as libc::c_int;
                                                        }
                                                        current_block_238 = 10405964186021900795;
                                                    }
                                                }
                                                match current_block_238 {
                                                    799969075006691772 => {
                                                        local_family = 2 as libc::c_int;
                                                        n = sscanf(
                                                            local_addr_port.as_mut_ptr() as *const libc::c_char,
                                                            b"%15[^:]%c%5hu%c\0" as *const u8 as *const libc::c_char,
                                                            local_addr.as_mut_ptr(),
                                                            &mut colon as *mut libc::c_char,
                                                            &mut local_port as *mut libc::c_ushort,
                                                            &mut extra as *mut libc::c_char,
                                                        );
                                                        if n == 1 as libc::c_int {
                                                            tmp___20 = 1 as libc::c_int;
                                                        } else if n == 3 as libc::c_int {
                                                            if colon as libc::c_int == 58 as libc::c_int {
                                                                tmp___20 = 1 as libc::c_int;
                                                            } else {
                                                                tmp___20 = 0 as libc::c_int;
                                                            }
                                                        } else {
                                                            tmp___20 = 0 as libc::c_int;
                                                        }
                                                        valid = tmp___20;
                                                    }
                                                    _ => {}
                                                }
                                                if valid == 0 {
                                                    fprintf(
                                                        stderr,
                                                        b"localnet address or port error\n\0" as *const u8
                                                            as *const libc::c_char,
                                                    );
                                                    exit(1 as libc::c_int);
                                                }
                                                local_port != 0;
                                                if num_localnet_addr < 64 as libc::c_ulong {
                                                    localnet_addr[num_localnet_addr as usize]
                                                        .family = local_family as sa_family_t;
                                                    localnet_addr[num_localnet_addr as usize].port = local_port;
                                                    valid = 0 as libc::c_int;
                                                    if local_family == 2 as libc::c_int {
                                                        tmp___24 = inet_pton(
                                                            local_family,
                                                            local_addr.as_mut_ptr() as *const libc::c_char,
                                                            &mut (*localnet_addr
                                                                .as_mut_ptr()
                                                                .offset(num_localnet_addr as isize))
                                                                .__annonCompField6
                                                                .__annonCompField4
                                                                .in_addr as *mut in_addr as *mut libc::c_void,
                                                        );
                                                        valid = (tmp___24 > 0 as libc::c_int) as libc::c_int;
                                                    } else if local_family == 10 as libc::c_int {
                                                        tmp___25 = inet_pton(
                                                            local_family,
                                                            local_addr.as_mut_ptr() as *const libc::c_char,
                                                            &mut (*localnet_addr
                                                                .as_mut_ptr()
                                                                .offset(num_localnet_addr as isize))
                                                                .__annonCompField6
                                                                .__annonCompField5
                                                                .in6_addr as *mut in6_addr as *mut libc::c_void,
                                                        );
                                                        valid = (tmp___25 > 0 as libc::c_int) as libc::c_int;
                                                    }
                                                    if valid == 0 {
                                                        fprintf(
                                                            stderr,
                                                            b"localnet address error\n\0" as *const u8
                                                                as *const libc::c_char,
                                                        );
                                                        exit(1 as libc::c_int);
                                                    }
                                                    let mut current_block_283: u64;
                                                    if local_family == 2 as libc::c_int {
                                                        tmp___28 = strchr(
                                                            local_netmask.as_mut_ptr() as *const libc::c_char,
                                                            '.' as i32,
                                                        );
                                                        if !tmp___28.is_null() {
                                                            tmp___26 = inet_pton(
                                                                local_family,
                                                                local_netmask.as_mut_ptr() as *const libc::c_char,
                                                                &mut (*localnet_addr
                                                                    .as_mut_ptr()
                                                                    .offset(num_localnet_addr as isize))
                                                                    .__annonCompField6
                                                                    .__annonCompField4
                                                                    .in_mask as *mut in_addr as *mut libc::c_void,
                                                            );
                                                            valid = (tmp___26 > 0 as libc::c_int) as libc::c_int;
                                                            current_block_283 = 9740084192901665250;
                                                        } else {
                                                            current_block_283 = 6095238872854002309;
                                                        }
                                                    } else {
                                                        current_block_283 = 6095238872854002309;
                                                    }
                                                    match current_block_283 {
                                                        6095238872854002309 => {
                                                            tmp___27 = sscanf(
                                                                local_netmask.as_mut_ptr() as *const libc::c_char,
                                                                b"%hu%c\0" as *const u8 as *const libc::c_char,
                                                                &mut local_prefix as *mut libc::c_ushort,
                                                                &mut extra as *mut libc::c_char,
                                                            );
                                                            valid = (tmp___27 == 1 as libc::c_int) as libc::c_int;
                                                            if valid != 0 {
                                                                let mut current_block_280: u64;
                                                                if local_family == 2 as libc::c_int {
                                                                    if local_prefix as libc::c_int <= 32 as libc::c_int {
                                                                        localnet_addr[num_localnet_addr as usize]
                                                                            .__annonCompField6
                                                                            .__annonCompField4
                                                                            .in_mask
                                                                            .s_addr = htonl(
                                                                            (4294967295 as libc::c_uint)
                                                                                << (32 as libc::c_uint)
                                                                                    .wrapping_sub(local_prefix as libc::c_uint),
                                                                        );
                                                                        current_block_280 = 9530373920617481355;
                                                                    } else {
                                                                        current_block_280 = 12176204743996301969;
                                                                    }
                                                                } else {
                                                                    current_block_280 = 12176204743996301969;
                                                                }
                                                                match current_block_280 {
                                                                    12176204743996301969 => {
                                                                        if local_family == 10 as libc::c_int {
                                                                            if local_prefix as libc::c_int <= 128 as libc::c_int {
                                                                                localnet_addr[num_localnet_addr as usize]
                                                                                    .__annonCompField6
                                                                                    .__annonCompField5
                                                                                    .in6_prefix = local_prefix as libc::c_uchar;
                                                                            } else {
                                                                                valid = 0 as libc::c_int;
                                                                            }
                                                                        } else {
                                                                            valid = 0 as libc::c_int;
                                                                        }
                                                                    }
                                                                    _ => {}
                                                                }
                                                            }
                                                        }
                                                        _ => {}
                                                    }
                                                    if valid == 0 {
                                                        fprintf(
                                                            stderr,
                                                            b"localnet netmask error\n\0" as *const u8
                                                                as *const libc::c_char,
                                                        );
                                                        exit(1 as libc::c_int);
                                                    }
                                                    num_localnet_addr = num_localnet_addr.wrapping_add(1);
                                                } else {
                                                    fprintf(
                                                        stderr,
                                                        b"# of localnet exceed %d.\n\0" as *const u8
                                                            as *const libc::c_char,
                                                        64 as libc::c_int,
                                                    );
                                                }
                                            }
                                        } else {
                                            sscanf(
                                                buff as *const libc::c_char,
                                                b"%s %u\0" as *const u8 as *const libc::c_char,
                                                user.as_mut_ptr(),
                                                &mut remote_dns_subnet as *mut libc::c_uint,
                                            );
                                            if remote_dns_subnet >= 256 as libc::c_uint {
                                                fprintf(
                                                    stderr,
                                                    b"remote_dns_subnet: invalid value. requires a number between 0 and 255.\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                );
                                                exit(1 as libc::c_int);
                                            }
                                        }
                                    } else {
                                        sscanf(
                                            buff as *const libc::c_char,
                                            b"%s %d\0" as *const u8 as *const libc::c_char,
                                            user.as_mut_ptr(),
                                            &mut tcp_connect_time_out as *mut libc::c_int,
                                        );
                                    }
                                } else {
                                    sscanf(
                                        buff as *const libc::c_char,
                                        b"%s %d\0" as *const u8 as *const libc::c_char,
                                        user.as_mut_ptr(),
                                        &mut tcp_read_time_out as *mut libc::c_int,
                                    );
                                }
                            } else {
                                *ct = ROUND_ROBIN_TYPE;
                            }
                        } else {
                            *ct = DYNAMIC_TYPE;
                        }
                    } else {
                        *ct = STRICT_TYPE;
                    }
                } else {
                    *ct = RANDOM_TYPE;
                }
            } else {
                list___0 = 1 as libc::c_int;
            }
        }
    }
    fclose(file);
    if count == 0 {
        fprintf(
            stderr,
            b"error: no valid proxy found in config\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    *proxy_count = count as libc::c_uint;
    proxychains_got_chain_data = 1 as libc::c_int;
}
pub unsafe extern "C" fn close(mut fd: libc::c_int) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    if init_l == 0 {
        if !(close_fds_cnt as libc::c_ulong
            >= (::std::mem::size_of::<[libc::c_int; 16]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong))
        {
            tmp = close_fds_cnt;
            close_fds_cnt += 1;
            close_fds[tmp as usize] = fd;
            tmp___0 = __errno_location();
            *tmp___0 = 0 as libc::c_int;
            return 0 as libc::c_int;
        }
    } else {
        if proxychains_resolver as libc::c_uint != 2 as libc::c_uint {
            tmp___1 = (Some(true_close.expect("non-null function pointer")))
                .expect("non-null function pointer")(fd);
            return tmp___1;
        }
        if fd != req_pipefd[0 as libc::c_int as usize] {
            if fd != req_pipefd[1 as libc::c_int as usize] {
                if fd != resp_pipefd[0 as libc::c_int as usize] {
                    if fd != resp_pipefd[1 as libc::c_int as usize] {
                        tmp___2 = (Some(true_close.expect("non-null function pointer")))
                            .expect("non-null function pointer")(fd);
                        return tmp___2;
                    }
                }
            }
        }
    }
    tmp___3 = __errno_location();
    *tmp___3 = 9 as libc::c_int;
    return -(1 as libc::c_int);
}
unsafe extern "C" fn is_v4inv6(mut a: *const in6_addr) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = memcmp(
        ((*a).__in6_u.__u6_addr8).as_ptr() as *const libc::c_void,
        b"\0\0\0\0\0\0\0\0\0\0\xFF\xFF\0" as *const u8 as *const libc::c_char
            as *const libc::c_void,
        12 as libc::c_int as size_t,
    );
    if tmp != 0 {
        tmp___0 = 0 as libc::c_int;
    } else {
        tmp___0 = 1 as libc::c_int;
    }
    return tmp___0;
}
unsafe extern "C" fn intsort(mut a: *mut libc::c_int, mut n: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        j = i + 1 as libc::c_int;
        while j < n {
            if *a.offset(j as isize) < *a.offset(i as isize) {
                s = *a.offset(i as isize);
                *a.offset(i as isize) = *a.offset(j as isize);
                *a.offset(j as isize) = s;
            }
            j += 1;
        }
        i += 1;
    }
}
pub unsafe extern "C" fn close_range(
    mut first: libc::c_uint,
    mut last: libc::c_uint,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut uerrno: libc::c_int = 0;
    let mut i___0: libc::c_int = 0;
    let mut protected_fds: [libc::c_int; 4] = [0; 4];
    let mut next_fd_to_close: libc::c_int = 0;
    let mut prev: libc::c_int = 0;
    let mut tmp___4: libc::c_uint = 0;
    let mut tmp___5: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: *mut libc::c_int = 0 as *mut libc::c_int;
    if ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(libc::c_uint, libc::c_uint, libc::c_int) -> libc::c_int,
        >,
        libc::c_ulong,
    >(true_close_range) == 0 as *mut libc::c_void as libc::c_ulong
    {
        fprintf(
            stderr,
            b"Calling close_range, but this platform does not provide this system call. \0"
                as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if init_l == 0 {
        if close_range_buffer_cnt as libc::c_ulong
            >= (::std::mem::size_of::<[close_range_args_t; 16]>() as libc::c_ulong)
                .wrapping_div(
                    ::std::mem::size_of::<close_range_args_t>() as libc::c_ulong,
                )
        {
            tmp = __errno_location();
            *tmp = 12 as libc::c_int;
            return -(1 as libc::c_int);
        }
        tmp___0 = close_range_buffer_cnt;
        close_range_buffer_cnt += 1;
        i = tmp___0;
        close_range_buffer[i as usize].first = first;
        close_range_buffer[i as usize].last = last;
        close_range_buffer[i as usize].flags = flags as libc::c_uint;
        tmp___1 = __errno_location();
        tmp___2 = 0 as libc::c_int;
        *tmp___1 = tmp___2;
        return tmp___2;
    }
    if proxychains_resolver as libc::c_uint != 2 as libc::c_uint {
        tmp___3 = (Some(true_close_range.expect("non-null function pointer")))
            .expect("non-null function pointer")(first, last, flags);
        return tmp___3;
    }
    res = 0 as libc::c_int;
    uerrno = 0 as libc::c_int;
    protected_fds[0 as libc::c_int as usize] = req_pipefd[0 as libc::c_int as usize];
    protected_fds[1 as libc::c_int as usize] = req_pipefd[1 as libc::c_int as usize];
    protected_fds[2 as libc::c_int as usize] = resp_pipefd[0 as libc::c_int as usize];
    protected_fds[3 as libc::c_int as usize] = resp_pipefd[1 as libc::c_int as usize];
    intsort(protected_fds.as_mut_ptr(), 4 as libc::c_int);
    next_fd_to_close = first as libc::c_int;
    i___0 = 0 as libc::c_int;
    while i___0 < 4 as libc::c_int {
        if !((protected_fds[i___0 as usize] as libc::c_uint) < first) {
            if !(protected_fds[i___0 as usize] as libc::c_uint > last) {
                if i___0 == 0 as libc::c_int {
                    tmp___4 = first;
                } else if (protected_fds[(i___0 - 1 as libc::c_int) as usize]
                        as libc::c_uint) < first
                    {
                    tmp___4 = first;
                } else {
                    tmp___4 = (protected_fds[(i___0 - 1 as libc::c_int) as usize]
                        + 1 as libc::c_int) as libc::c_uint;
                }
                prev = tmp___4 as libc::c_int;
                if prev != protected_fds[i___0 as usize] {
                    tmp___6 = (Some(
                        true_close_range.expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        prev as libc::c_uint,
                        (protected_fds[i___0 as usize] - 1 as libc::c_int)
                            as libc::c_uint,
                        flags,
                    );
                    if -(1 as libc::c_int) == tmp___6 {
                        res = -(1 as libc::c_int);
                        tmp___5 = __errno_location();
                        uerrno = *tmp___5;
                    }
                }
                next_fd_to_close = protected_fds[i___0 as usize] + 1 as libc::c_int;
            }
        }
        i___0 += 1;
    }
    if next_fd_to_close as libc::c_uint <= last {
        tmp___8 = (Some(true_close_range.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(next_fd_to_close as libc::c_uint, last, flags);
        if -(1 as libc::c_int) == tmp___8 {
            res = -(1 as libc::c_int);
            tmp___7 = __errno_location();
            uerrno = *tmp___7;
        }
    }
    tmp___9 = __errno_location();
    *tmp___9 = uerrno;
    return res;
}
pub unsafe extern "C" fn connect(
    mut sock: libc::c_int,
    mut addr: *const sockaddr,
    mut len: libc::c_uint,
) -> libc::c_int {
    let mut current_block: u64;
    let mut socktype: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut optlen: socklen_t = 0;
    let mut dest_ip: ip_type = ip_type {
        addr: __anonunion_addr_69655148 {
            v4: __anonunion_ip_type4_826858479 {
                octet: [0; 4],
            },
        },
        is_v6: 0,
    };
    let mut p_addr_in: *mut in_addr = 0 as *mut in_addr;
    let mut p_addr_in6: *mut in6_addr = 0 as *mut in6_addr;
    let mut dnat: *mut dnat_arg = 0 as *mut dnat_arg;
    let mut port: libc::c_ushort = 0;
    let mut i: size_t = 0;
    let mut remote_dns_connect: libc::c_int = 0;
    let mut fam: sa_family_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut v6: libc::c_int = 0;
    let mut tmp___0: uint16_t = 0;
    let mut tmp___1: uint16_t = 0;
    let mut v4inv6: in_addr = in_addr { s_addr: 0 };
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: uint32_t = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut prefix_bytes: size_t = 0;
    let mut prefix_bits: size_t = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___12: uint16_t = 0;
    let mut tmp___13: *mut libc::c_int = 0 as *mut libc::c_int;
    init_lib_wrapper(b"connect\0" as *const u8 as *const libc::c_char);
    socktype = 0 as libc::c_int;
    flags = 0 as libc::c_int;
    ret = 0 as libc::c_int;
    optlen = 0 as libc::c_int as socklen_t;
    dnat = 0 as *mut libc::c_void as *mut dnat_arg;
    remote_dns_connect = 0 as libc::c_int;
    optlen = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
    fam = (*(addr as *mut sockaddr_in)).sin_family;
    getsockopt(
        sock,
        1 as libc::c_int,
        3 as libc::c_int,
        &mut socktype as *mut libc::c_int as *mut libc::c_void,
        &mut optlen as *mut socklen_t,
    );
    if !(fam as libc::c_int == 2 as libc::c_int) {
        if !(fam as libc::c_int == 10 as libc::c_int) {
            tmp = (Some(true_connect.expect("non-null function pointer")))
                .expect("non-null function pointer")(sock, addr, len);
            return tmp;
        }
    }
    if !(socktype == 1 as libc::c_int) {
        tmp = (Some(true_connect.expect("non-null function pointer")))
            .expect("non-null function pointer")(sock, addr, len);
        return tmp;
    }
    dest_ip
        .is_v6 = (fam as libc::c_int == 10 as libc::c_int) as libc::c_int
        as libc::c_char;
    v6 = dest_ip.is_v6 as libc::c_int;
    p_addr_in = &mut (*(addr as *mut sockaddr_in)).sin_addr;
    p_addr_in6 = &mut (*(addr as *mut sockaddr_in6)).sin6_addr;
    if v6 == 0 {
        tmp___0 = ntohs((*(addr as *mut sockaddr_in)).sin_port);
        port = tmp___0;
    } else {
        tmp___1 = ntohs((*(addr as *mut sockaddr_in6)).sin6_port);
        port = tmp___1;
    }
    if v6 != 0 {
        tmp___2 = is_v4inv6(p_addr_in6 as *const in6_addr);
        if tmp___2 != 0 {
            memcpy(
                &mut v4inv6.s_addr as *mut in_addr_t as *mut libc::c_void,
                &mut *((*p_addr_in6).__in6_u.__u6_addr8)
                    .as_mut_ptr()
                    .offset(12 as libc::c_int as isize) as *mut uint8_t
                    as *const libc::c_void,
                4 as libc::c_int as size_t,
            );
            dest_ip.is_v6 = 0 as libc::c_int as libc::c_char;
            v6 = dest_ip.is_v6 as libc::c_int;
            p_addr_in = &mut v4inv6;
        }
    }
    if v6 == 0 {
        tmp___4 = memcmp(
            p_addr_in as *const libc::c_void,
            b"\0\0\0\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as size_t,
        );
        if tmp___4 == 0 {
            tmp___3 = __errno_location();
            *tmp___3 = 111 as libc::c_int;
            return -(1 as libc::c_int);
        }
    }
    if v6 == 0 {
        tmp___5 = ntohl((*p_addr_in).s_addr);
        if tmp___5 >> 24 as libc::c_int == remote_dns_subnet {
            tmp___6 = 1 as libc::c_int;
        } else {
            tmp___6 = 0 as libc::c_int;
        }
    } else {
        tmp___6 = 0 as libc::c_int;
    }
    remote_dns_connect = tmp___6;
    if v6 == 0 {
        i = 0 as libc::c_int as size_t;
        while i < num_dnats {
            if !(remote_dns_connect == 0) {
                break;
            }
            if !dnat.is_null() {
                break;
            }
            if dnats[i as usize].orig_dst.s_addr == (*p_addr_in).s_addr {
                if dnats[i as usize].orig_port != 0 {
                    if dnats[i as usize].orig_port as libc::c_int == port as libc::c_int
                    {
                        dnat = &mut *dnats.as_mut_ptr().offset(i as isize)
                            as *mut dnat_arg;
                    }
                }
            }
            i = i.wrapping_add(1);
        }
    }
    if v6 == 0 {
        i = 0 as libc::c_int as size_t;
        while i < num_dnats {
            if !(remote_dns_connect == 0) {
                break;
            }
            if !dnat.is_null() {
                break;
            }
            if dnats[i as usize].orig_dst.s_addr == (*p_addr_in).s_addr {
                if dnats[i as usize].orig_port == 0 {
                    dnat = &mut *dnats.as_mut_ptr().offset(i as isize) as *mut dnat_arg;
                }
            }
            i = i.wrapping_add(1);
        }
    }
    if !dnat.is_null() {
        p_addr_in = &mut (*dnat).new_dst;
        if (*dnat).new_port != 0 {
            port = (*dnat).new_port;
        }
    }
    i = 0 as libc::c_int as size_t;
    while i < num_localnet_addr {
        if remote_dns_connect != 0 {
            break;
        }
        if localnet_addr[i as usize].port != 0 {
            if localnet_addr[i as usize].port as libc::c_int != port as libc::c_int {
                current_block = 5119602906463854295;
            } else {
                current_block = 14851765859726653900;
            }
        } else {
            current_block = 14851765859726653900;
        }
        match current_block {
            14851765859726653900 => {
                if v6 != 0 {
                    tmp___7 = 10 as libc::c_int;
                } else {
                    tmp___7 = 2 as libc::c_int;
                }
                if !(localnet_addr[i as usize].family as libc::c_int != tmp___7) {
                    if v6 != 0 {
                        prefix_bytes = (localnet_addr[i as usize]
                            .__annonCompField6
                            .__annonCompField5
                            .in6_prefix as libc::c_int / 8 as libc::c_int) as size_t;
                        prefix_bits = (localnet_addr[i as usize]
                            .__annonCompField6
                            .__annonCompField5
                            .in6_prefix as libc::c_int % 8 as libc::c_int) as size_t;
                        if prefix_bytes != 0 {
                            tmp___8 = memcmp(
                                ((*p_addr_in6).__in6_u.__u6_addr8).as_mut_ptr()
                                    as *const libc::c_void,
                                (localnet_addr[i as usize]
                                    .__annonCompField6
                                    .__annonCompField5
                                    .in6_addr
                                    .__in6_u
                                    .__u6_addr8)
                                    .as_mut_ptr() as *const libc::c_void,
                                prefix_bytes,
                            );
                            if tmp___8 != 0 as libc::c_int {
                                current_block = 5119602906463854295;
                            } else {
                                current_block = 10041771570435381152;
                            }
                        } else {
                            current_block = 10041771570435381152;
                        }
                        match current_block {
                            5119602906463854295 => {}
                            _ => {
                                if prefix_bits != 0 {
                                    if ((*p_addr_in6).__in6_u.__u6_addr8[prefix_bytes as usize]
                                        as libc::c_int
                                        ^ localnet_addr[i as usize]
                                            .__annonCompField6
                                            .__annonCompField5
                                            .in6_addr
                                            .__in6_u
                                            .__u6_addr8[prefix_bytes as usize] as libc::c_int)
                                        >> (8 as libc::c_ulong).wrapping_sub(prefix_bits) != 0
                                    {
                                        current_block = 5119602906463854295;
                                    } else {
                                        current_block = 12696043255897098083;
                                    }
                                } else {
                                    current_block = 12696043255897098083;
                                }
                            }
                        }
                    } else if ((*p_addr_in).s_addr
                            ^ localnet_addr[i as usize]
                                .__annonCompField6
                                .__annonCompField4
                                .in_addr
                                .s_addr)
                            & localnet_addr[i as usize]
                                .__annonCompField6
                                .__annonCompField4
                                .in_mask
                                .s_addr != 0
                        {
                        current_block = 5119602906463854295;
                    } else {
                        current_block = 12696043255897098083;
                    }
                    match current_block {
                        5119602906463854295 => {}
                        _ => {
                            tmp___9 = (Some(
                                true_connect.expect("non-null function pointer"),
                            ))
                                .expect("non-null function pointer")(sock, addr, len);
                            return tmp___9;
                        }
                    }
                }
            }
            _ => {}
        }
        i = i.wrapping_add(1);
    }
    flags = fcntl(sock, 3 as libc::c_int, 0 as libc::c_int);
    if flags & 2048 as libc::c_int != 0 {
        fcntl(sock, 4 as libc::c_int, 0 as libc::c_int);
    }
    if v6 != 0 {
        tmp___10 = 16 as libc::c_int;
    } else {
        tmp___10 = 4 as libc::c_int;
    }
    if v6 != 0 {
        tmp___11 = p_addr_in6 as *mut libc::c_void;
    } else {
        tmp___11 = p_addr_in as *mut libc::c_void;
    }
    memcpy(
        (dest_ip.addr.v6).as_mut_ptr() as *mut libc::c_void,
        tmp___11 as *const libc::c_void,
        tmp___10 as size_t,
    );
    tmp___12 = htons(port);
    ret = connect_proxy_chain(
        sock,
        dest_ip,
        tmp___12,
        proxychains_pd.as_mut_ptr(),
        proxychains_proxy_count,
        proxychains_ct,
        proxychains_max_chain,
    );
    fcntl(sock, 4 as libc::c_int, flags);
    if ret != 0 as libc::c_int {
        tmp___13 = __errno_location();
        *tmp___13 = 111 as libc::c_int;
    }
    return ret;
}
static mut ghbndata: gethostbyname_data = gethostbyname_data {
    hostent_space: hostent {
        h_name: 0 as *const libc::c_char as *mut libc::c_char,
        h_aliases: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        h_addrtype: 0,
        h_length: 0,
        h_addr_list: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    },
    resolved_addr: 0,
    resolved_addr_p: [0 as *mut libc::c_char; 2],
    addr_name: [0; 256],
};
pub unsafe extern "C" fn gethostbyname(mut name: *const libc::c_char) -> *mut hostent {
    let mut tmp: *mut hostent = 0 as *mut hostent;
    let mut tmp___0: *mut hostent = 0 as *mut hostent;
    let mut tmp___1: *mut hostent = 0 as *mut hostent;
    init_lib_wrapper(b"gethostbyname\0" as *const u8 as *const libc::c_char);
    if proxychains_resolver as libc::c_uint == 1 as libc::c_uint {
        tmp = proxy_gethostbyname_old(name);
        return tmp;
    } else if proxychains_resolver as libc::c_uint == 0 as libc::c_uint {
        tmp___0 = (Some(true_gethostbyname.expect("non-null function pointer")))
            .expect("non-null function pointer")(name);
        return tmp___0;
    } else {
        tmp___1 = proxy_gethostbyname(name, &mut ghbndata);
        return tmp___1;
    };
}
pub unsafe extern "C" fn getaddrinfo(
    mut node: *const libc::c_char,
    mut service: *const libc::c_char,
    mut hints: *const addrinfo,
    mut res: *mut *mut addrinfo,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    init_lib_wrapper(b"getaddrinfo\0" as *const u8 as *const libc::c_char);
    if proxychains_resolver as libc::c_uint != 0 as libc::c_uint {
        tmp = proxy_getaddrinfo(node, service, hints, res);
        return tmp;
    } else {
        tmp___0 = (Some(true_getaddrinfo.expect("non-null function pointer")))
            .expect("non-null function pointer")(node, service, hints, res);
        return tmp___0;
    };
}
pub unsafe extern "C" fn freeaddrinfo(mut res: *mut addrinfo) {
    init_lib_wrapper(b"freeaddrinfo\0" as *const u8 as *const libc::c_char);
    if proxychains_resolver as libc::c_uint == 0 as libc::c_uint {
        (Some(true_freeaddrinfo.expect("non-null function pointer")))
            .expect("non-null function pointer")(res);
    } else {
        proxy_freeaddrinfo(res);
    };
}
pub unsafe extern "C" fn getnameinfo(
    mut sa: *const sockaddr,
    mut salen: socklen_t,
    mut host: *mut libc::c_char,
    mut hostlen: socklen_t,
    mut serv: *mut libc::c_char,
    mut servlen: socklen_t,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut v6: libc::c_int = 0;
    let mut tmp___0: libc::c_ulong = 0;
    let mut v4inv6buf: [libc::c_uchar; 4] = [0; 4];
    let mut ip: *const libc::c_void = 0 as *const libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut scopeid: libc::c_uint = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: *const libc::c_char = 0 as *const libc::c_char;
    let mut l: size_t = 0;
    let mut tmp___5: size_t = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: uint16_t = 0;
    let mut tmp___8: libc::c_int = 0;
    init_lib_wrapper(b"getnameinfo\0" as *const u8 as *const libc::c_char);
    if proxychains_resolver as libc::c_uint == 0 as libc::c_uint {
        tmp = (Some(true_getnameinfo.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(sa, salen, host, hostlen, serv, servlen, flags);
        return tmp;
    } else {
        if salen == 0 {
            return -(6 as libc::c_int)
        } else {
            if !((*(sa as *mut sockaddr_in)).sin_family as libc::c_int
                == 2 as libc::c_int)
            {
                if !((*(sa as *mut sockaddr_in)).sin_family as libc::c_int
                    == 10 as libc::c_int)
                {
                    return -(6 as libc::c_int);
                }
            }
        }
        v6 = ((*(sa as *mut sockaddr_in)).sin_family as libc::c_int == 10 as libc::c_int)
            as libc::c_int;
        if v6 != 0 {
            tmp___0 = ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong;
        } else {
            tmp___0 = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong;
        }
        if (salen as libc::c_ulong) < tmp___0 {
            return -(6 as libc::c_int);
        }
        if hostlen != 0 {
            if v6 != 0 {
                tmp___1 = &mut (*(sa as *mut sockaddr_in6)).sin6_addr as *mut in6_addr
                    as *mut libc::c_void;
            } else {
                tmp___1 = &mut (*(sa as *mut sockaddr_in)).sin_addr as *mut in_addr
                    as *mut libc::c_void;
            }
            ip = tmp___1 as *const libc::c_void;
            scopeid = 0 as libc::c_uint;
            if v6 != 0 {
                tmp___2 = is_v4inv6(
                    &mut (*(sa as *mut sockaddr_in6)).sin6_addr as *mut in6_addr
                        as *const in6_addr,
                );
                if tmp___2 != 0 {
                    memcpy(
                        v4inv6buf.as_mut_ptr() as *mut libc::c_void,
                        &mut *((*(sa as *mut sockaddr_in6)).sin6_addr.__in6_u.__u6_addr8)
                            .as_mut_ptr()
                            .offset(12 as libc::c_int as isize) as *mut uint8_t
                            as *const libc::c_void,
                        4 as libc::c_int as size_t,
                    );
                    ip = v4inv6buf.as_mut_ptr() as *const libc::c_void;
                    v6 = 0 as libc::c_int;
                } else {
                    scopeid = (*(sa as *mut sockaddr_in6)).sin6_scope_id;
                }
            }
            if v6 != 0 {
                tmp___3 = 10 as libc::c_int;
            } else {
                tmp___3 = 2 as libc::c_int;
            }
            tmp___4 = inet_ntop(tmp___3, ip, host, hostlen);
            if tmp___4.is_null() {
                return -(12 as libc::c_int);
            }
            if scopeid != 0 {
                tmp___5 = strlen(host as *const libc::c_char);
                l = tmp___5;
                tmp___6 = snprintf(
                    host.offset(l as isize),
                    (hostlen as size_t).wrapping_sub(l),
                    b"%%%u\0" as *const u8 as *const libc::c_char,
                    scopeid,
                );
                if tmp___6 as size_t >= (hostlen as size_t).wrapping_sub(l) {
                    return -(12 as libc::c_int);
                }
            }
        }
        if servlen != 0 {
            tmp___7 = ntohs((*(sa as *mut sockaddr_in)).sin_port);
            tmp___8 = snprintf(
                serv,
                servlen as size_t,
                b"%d\0" as *const u8 as *const libc::c_char,
                tmp___7 as libc::c_int,
            );
            if tmp___8 as socklen_t >= servlen {
                return -(12 as libc::c_int);
            }
        }
    }
    return 0 as libc::c_int;
}
static mut buf: [libc::c_char; 16] = [0; 16];
static mut ipv4: [libc::c_char; 4] = [0; 4];
static mut list: [*mut libc::c_char; 2] = [0 as *const libc::c_char
    as *mut libc::c_char; 2];
static mut aliases: [*mut libc::c_char; 1] = [0 as *const libc::c_char
    as *mut libc::c_char; 1];
static mut he: hostent = hostent {
    h_name: 0 as *const libc::c_char as *mut libc::c_char,
    h_aliases: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    h_addrtype: 0,
    h_length: 0,
    h_addr_list: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
};
pub unsafe extern "C" fn gethostbyaddr(
    mut addr: *const libc::c_void,
    mut len: __socklen_t,
    mut type_0: libc::c_int,
) -> *mut hostent {
    let mut tmp: *mut hostent = 0 as *mut hostent;
    init_lib_wrapper(b"gethostbyaddr\0" as *const u8 as *const libc::c_char);
    if proxychains_resolver as libc::c_uint == 0 as libc::c_uint {
        tmp = (Some(true_gethostbyaddr.expect("non-null function pointer")))
            .expect("non-null function pointer")(addr, len, type_0);
        return tmp;
    } else {
        if len != 4 as libc::c_uint {
            return 0 as *mut libc::c_void as *mut hostent;
        }
        he.h_name = buf.as_mut_ptr();
        memcpy(ipv4.as_mut_ptr() as *mut libc::c_void, addr, 4 as libc::c_int as size_t);
        list[0 as libc::c_int as usize] = ipv4.as_mut_ptr();
        list[1 as libc::c_int as usize] = 0 as *mut libc::c_void as *mut libc::c_char;
        he.h_addr_list = list.as_mut_ptr();
        he.h_addrtype = 2 as libc::c_int;
        aliases[0 as libc::c_int as usize] = 0 as *mut libc::c_void as *mut libc::c_char;
        he.h_aliases = aliases.as_mut_ptr();
        he.h_length = 4 as libc::c_int;
        pc_stringfromipv4(addr as *mut libc::c_uchar, buf.as_mut_ptr());
        return &mut he;
    };
}
pub unsafe extern "C" fn sendto(
    mut sockfd: libc::c_int,
    mut buf___0: *const libc::c_void,
    mut len: size_t,
    mut flags: libc::c_int,
    mut dest_addr: *const sockaddr,
    mut addrlen: socklen_t,
) -> ssize_t {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: ssize_t = 0;
    init_lib_wrapper(b"sendto\0" as *const u8 as *const libc::c_char);
    if flags & 536870912 as libc::c_int != 0 {
        tmp = connect(sockfd, dest_addr, addrlen);
        if tmp == 0 {
            tmp___0 = __errno_location();
            if *tmp___0 != 115 as libc::c_int {
                return -(1 as libc::c_int) as ssize_t;
            }
        }
        dest_addr = 0 as *mut libc::c_void as *const sockaddr;
        addrlen = 0 as libc::c_int as socklen_t;
        flags &= -(536870913 as libc::c_int);
    }
    tmp___1 = (Some(true_sendto.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(sockfd, buf___0, len, flags, dest_addr, addrlen);
    return tmp___1;
}
unsafe extern "C" fn setup_hooks() {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___5: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___6: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___7: *mut libc::c_void = 0 as *mut libc::c_void;
    if true_connect.is_none() {
        tmp = load_sym(
            b"connect\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        libc::c_int,
                        *const sockaddr,
                        libc::c_uint,
                    ) -> libc::c_int,
                >,
                *mut libc::c_void,
            >(
                Some(
                    connect
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const sockaddr,
                            libc::c_uint,
                        ) -> libc::c_int,
                ),
            ),
            1 as libc::c_int,
        );
        true_connect = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(
                    libc::c_int,
                    *const sockaddr,
                    socklen_t,
                ) -> libc::c_int,
            >,
        >(tmp);
    }
    if true_sendto.is_none() {
        tmp___0 = load_sym(
            b"sendto\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        libc::c_int,
                        *const libc::c_void,
                        size_t,
                        libc::c_int,
                        *const sockaddr,
                        socklen_t,
                    ) -> ssize_t,
                >,
                *mut libc::c_void,
            >(
                Some(
                    sendto
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const libc::c_void,
                            size_t,
                            libc::c_int,
                            *const sockaddr,
                            socklen_t,
                        ) -> ssize_t,
                ),
            ),
            1 as libc::c_int,
        );
        true_sendto = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(
                    libc::c_int,
                    *const libc::c_void,
                    size_t,
                    libc::c_int,
                    *const sockaddr,
                    socklen_t,
                ) -> ssize_t,
            >,
        >(tmp___0);
    }
    if true_gethostbyname.is_none() {
        tmp___1 = load_sym(
            b"gethostbyname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*const libc::c_char) -> *mut hostent>,
                *mut libc::c_void,
            >(
                Some(
                    gethostbyname
                        as unsafe extern "C" fn(*const libc::c_char) -> *mut hostent,
                ),
            ),
            1 as libc::c_int,
        );
        true_gethostbyname = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(*const libc::c_char) -> *mut hostent>,
        >(tmp___1);
    }
    if true_getaddrinfo.is_none() {
        tmp___2 = load_sym(
            b"getaddrinfo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const libc::c_char,
                        *const libc::c_char,
                        *const addrinfo,
                        *mut *mut addrinfo,
                    ) -> libc::c_int,
                >,
                *mut libc::c_void,
            >(
                Some(
                    getaddrinfo
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *const addrinfo,
                            *mut *mut addrinfo,
                        ) -> libc::c_int,
                ),
            ),
            1 as libc::c_int,
        );
        true_getaddrinfo = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                    *const addrinfo,
                    *mut *mut addrinfo,
                ) -> libc::c_int,
            >,
        >(tmp___2);
    }
    if true_freeaddrinfo.is_none() {
        tmp___3 = load_sym(
            b"freeaddrinfo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut addrinfo) -> ()>,
                *mut libc::c_void,
            >(Some(freeaddrinfo as unsafe extern "C" fn(*mut addrinfo) -> ())),
            1 as libc::c_int,
        );
        true_freeaddrinfo = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(*mut addrinfo) -> libc::c_int>,
        >(tmp___3);
    }
    if true_gethostbyaddr.is_none() {
        tmp___4 = load_sym(
            b"gethostbyaddr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const libc::c_void,
                        __socklen_t,
                        libc::c_int,
                    ) -> *mut hostent,
                >,
                *mut libc::c_void,
            >(
                Some(
                    gethostbyaddr
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            __socklen_t,
                            libc::c_int,
                        ) -> *mut hostent,
                ),
            ),
            1 as libc::c_int,
        );
        true_gethostbyaddr = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_void,
                    socklen_t,
                    libc::c_int,
                ) -> *mut hostent,
            >,
        >(tmp___4);
    }
    if true_getnameinfo.is_none() {
        tmp___5 = load_sym(
            b"getnameinfo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const sockaddr,
                        socklen_t,
                        *mut libc::c_char,
                        socklen_t,
                        *mut libc::c_char,
                        socklen_t,
                        libc::c_int,
                    ) -> libc::c_int,
                >,
                *mut libc::c_void,
            >(
                Some(
                    getnameinfo
                        as unsafe extern "C" fn(
                            *const sockaddr,
                            socklen_t,
                            *mut libc::c_char,
                            socklen_t,
                            *mut libc::c_char,
                            socklen_t,
                            libc::c_int,
                        ) -> libc::c_int,
                ),
            ),
            1 as libc::c_int,
        );
        true_getnameinfo = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(
                    *const sockaddr,
                    socklen_t,
                    *mut libc::c_char,
                    socklen_t,
                    *mut libc::c_char,
                    socklen_t,
                    libc::c_int,
                ) -> libc::c_int,
            >,
        >(tmp___5);
    }
    if true_close.is_none() {
        tmp___6 = load_sym(
            b"close\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
                *mut libc::c_void,
            >(Some(close as unsafe extern "C" fn(libc::c_int) -> libc::c_int)),
            1 as libc::c_int,
        );
        true_close = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
        >(tmp___6);
    }
    if true_close_range.is_none() {
        tmp___7 = load_sym(
            b"close_range\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        libc::c_uint,
                        libc::c_uint,
                        libc::c_int,
                    ) -> libc::c_int,
                >,
                *mut libc::c_void,
            >(
                Some(
                    close_range
                        as unsafe extern "C" fn(
                            libc::c_uint,
                            libc::c_uint,
                            libc::c_int,
                        ) -> libc::c_int,
                ),
            ),
            0 as libc::c_int,
        );
        true_close_range = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(
                    libc::c_uint,
                    libc::c_uint,
                    libc::c_int,
                ) -> libc::c_int,
            >,
        >(tmp___7);
    }
}
pub static mut req_pipefd: [libc::c_int; 2] = [0; 2];
pub static mut resp_pipefd: [libc::c_int; 2] = [0; 2];
unsafe extern "C" fn dumpstring(
    mut s: *mut libc::c_char,
    mut len: size_t,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(len);
    p = tmp as *mut libc::c_char;
    if !p.is_null() {
        memcpy(p as *mut libc::c_void, s as *const libc::c_void, len);
    }
    return p as *mut libc::c_void;
}
static mut internal_ips_lock: *mut pthread_mutex_t = 0 as *const pthread_mutex_t
    as *mut pthread_mutex_t;
static mut internal_ips: *mut internal_ip_lookup_table = 0
    as *const internal_ip_lookup_table as *mut internal_ip_lookup_table;
pub unsafe extern "C" fn index_from_internal_ip(mut internalip: ip_type4) -> uint32_t {
    let mut tmp: ip_type4 = __anonunion_ip_type4_826858479 {
        octet: [0; 4],
    };
    let mut ret: uint32_t = 0;
    tmp = internalip;
    ret = (tmp.octet[3 as libc::c_int as usize] as libc::c_int
        + ((tmp.octet[2 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int)
        + ((tmp.octet[1 as libc::c_int as usize] as libc::c_int) << 16 as libc::c_int))
        as uint32_t;
    ret = ret.wrapping_sub(1);
    return ret;
}
pub unsafe extern "C" fn string_from_internal_ip(
    mut internalip: ip_type4,
) -> *mut libc::c_char {
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut index___0: uint32_t = 0;
    let mut tmp: uint32_t = 0;
    res = 0 as *mut libc::c_void as *mut libc::c_char;
    tmp = index_from_internal_ip(internalip);
    index___0 = tmp;
    if index___0 < (*internal_ips).counter {
        res = (**((*internal_ips).list).offset(index___0 as isize)).string;
    }
    return res;
}
pub unsafe extern "C" fn make_internal_ip(mut index___0: uint32_t) -> ip_type4 {
    let mut ret: ip_type4 = __anonunion_ip_type4_826858479 {
        octet: [0; 4],
    };
    let mut __constr_expr_5: ip_type4 = __anonunion_ip_type4_826858479 {
        octet: [0; 4],
    };
    index___0 = index___0.wrapping_add(1);
    if index___0 > 16777215 as libc::c_uint {
        __constr_expr_5.as_int = -(1 as libc::c_int) as uint32_t;
        return __constr_expr_5;
    }
    ret
        .octet[0 as libc::c_int
        as usize] = (remote_dns_subnet & 255 as libc::c_uint) as libc::c_uchar;
    ret
        .octet[1 as libc::c_int
        as usize] = ((index___0 & 16711680 as libc::c_uint) >> 16 as libc::c_int)
        as libc::c_uchar;
    ret
        .octet[2 as libc::c_int
        as usize] = ((index___0 & 65280 as libc::c_uint) >> 8 as libc::c_int)
        as libc::c_uchar;
    ret
        .octet[3 as libc::c_int
        as usize] = (index___0 & 255 as libc::c_uint) as libc::c_uchar;
    return ret;
}
unsafe extern "C" fn ip_from_internal_list(
    mut name: *mut libc::c_char,
    mut len: size_t,
) -> ip_type4 {
    let mut current_block: u64;
    let mut hash: uint32_t = 0;
    let mut tmp: uint32_t = 0;
    let mut i: size_t = 0;
    let mut res: ip_type4 = __anonunion_ip_type4_826858479 {
        octet: [0; 4],
    };
    let mut new_mem: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    let mut __constr_expr_6: ip_type4 = __anonunion_ip_type4_826858479 {
        octet: [0; 4],
    };
    let mut tmp___1: string_hash_tuple = string_hash_tuple {
        hash: 0,
        string: 0 as *mut libc::c_char,
    };
    let mut __constr_expr_7: ip_type4 = __anonunion_ip_type4_826858479 {
        octet: [0; 4],
    };
    tmp = dalias_hash(name);
    hash = tmp;
    if (*internal_ips).counter != 0 {
        i = 0 as libc::c_int as size_t;
        loop {
            if !(i < (*internal_ips).counter as size_t) {
                current_block = 8236137900636309791;
                break;
            }
            if (**((*internal_ips).list).offset(i as isize)).hash == hash {
                tmp___0 = strcmp(
                    name as *const libc::c_char,
                    (**((*internal_ips).list).offset(i as isize)).string
                        as *const libc::c_char,
                );
                if tmp___0 == 0 {
                    res = make_internal_ip(i as uint32_t);
                    current_block = 2112896983504075249;
                    break;
                }
            }
            i = i.wrapping_add(1);
        }
    } else {
        current_block = 8236137900636309791;
    }
    match current_block {
        8236137900636309791 => {
            if (*internal_ips).capa
                < ((*internal_ips).counter).wrapping_add(1 as libc::c_uint)
            {
                new_mem = realloc(
                    (*internal_ips).list as *mut libc::c_void,
                    (((*internal_ips).capa).wrapping_add(16 as libc::c_uint)
                        as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                        ),
                );
                if !new_mem.is_null() {
                    (*internal_ips)
                        .capa = ((*internal_ips).capa as libc::c_uint)
                        .wrapping_add(16 as libc::c_uint) as uint32_t as uint32_t;
                    (*internal_ips).list = new_mem as *mut *mut string_hash_tuple;
                    current_block = 13472856163611868459;
                } else {
                    current_block = 15090052786889560393;
                }
            } else {
                current_block = 13472856163611868459;
            }
            match current_block {
                13472856163611868459 => {
                    res = make_internal_ip((*internal_ips).counter);
                    __constr_expr_6.as_int = -(1 as libc::c_int) as uint32_t;
                    if res.as_int == __constr_expr_6.as_int {
                        current_block = 15090052786889560393;
                    } else {
                        tmp___1.hash = 0 as libc::c_int as uint32_t;
                        tmp___1.string = 0 as *mut libc::c_char;
                        new_mem = dumpstring(
                            &mut tmp___1 as *mut string_hash_tuple as *mut libc::c_char,
                            ::std::mem::size_of::<string_hash_tuple>() as libc::c_ulong,
                        );
                        if new_mem.is_null() {
                            current_block = 15090052786889560393;
                        } else {
                            let ref mut fresh0 = *((*internal_ips).list)
                                .offset((*internal_ips).counter as isize);
                            *fresh0 = new_mem as *mut string_hash_tuple;
                            (**((*internal_ips).list)
                                .offset((*internal_ips).counter as isize))
                                .hash = hash;
                            new_mem = dumpstring(name, len);
                            if new_mem.is_null() {
                                let ref mut fresh1 = *((*internal_ips).list)
                                    .offset((*internal_ips).counter as isize);
                                *fresh1 = 0 as *mut string_hash_tuple;
                                current_block = 15090052786889560393;
                            } else {
                                let ref mut fresh2 = (**((*internal_ips).list)
                                    .offset((*internal_ips).counter as isize))
                                    .string;
                                *fresh2 = new_mem as *mut libc::c_char;
                                (*internal_ips)
                                    .counter = ((*internal_ips).counter).wrapping_add(1);
                                current_block = 2112896983504075249;
                            }
                        }
                    }
                }
                _ => {}
            }
            match current_block {
                2112896983504075249 => {}
                _ => {
                    __constr_expr_7.as_int = -(1 as libc::c_int) as uint32_t;
                    return __constr_expr_7;
                }
            }
        }
        _ => {}
    }
    return res;
}
static mut allocator_thread: pthread_t = 0;
unsafe extern "C" fn wait_data(mut readfd___0: libc::c_int) -> libc::c_int {
    let mut fds: fd_set = fd_set { fds_bits: [0; 16] };
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut emsg: [libc::c_char; 1024] = [0; 1024];
    let mut x: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
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
    fds
        .fds_bits[(readfd___0
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        |= ((1 as libc::c_ulong)
            << readfd___0
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask;
    loop {
        ret = select(
            readfd___0 + 1 as libc::c_int,
            &mut fds as *mut fd_set,
            0 as *mut libc::c_void as *mut fd_set,
            0 as *mut libc::c_void as *mut fd_set,
            0 as *mut libc::c_void as *mut timeval,
        );
        if !(ret <= 0 as libc::c_int) {
            break;
        }
        if !(ret < 0 as libc::c_int) {
            continue;
        }
        tmp = __errno_location();
        e = *tmp;
        if e == 4 as libc::c_int {
            continue;
        }
        tmp___0 = __errno_location();
        tmp___1 = strerror_r(
            *tmp___0,
            emsg.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
        x = tmp___1;
        dprintf(
            2 as libc::c_int,
            b"select2: %s\n\0" as *const u8 as *const libc::c_char,
            x,
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn trywrite(
    mut fd: libc::c_int,
    mut buf___0: *mut libc::c_void,
    mut bytes: size_t,
) -> libc::c_int {
    let mut ret: ssize_t = 0;
    let mut out: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    out = buf___0 as *mut libc::c_uchar;
    loop {
        ret = write(fd, out as *const libc::c_void, bytes);
        match ret {
            -1 => {
                tmp = __errno_location();
                if !(*tmp == 4 as libc::c_int) {
                    break;
                }
            }
            0 => {
                break;
            }
            _ => {
                if ret as size_t == bytes {
                    return 1 as libc::c_int
                } else {
                    if bytes == 0 {
                        return 1 as libc::c_int;
                    }
                }
                out = out.offset(ret as isize);
                bytes = (bytes as libc::c_ulong).wrapping_sub(ret as size_t) as size_t
                    as size_t;
            }
        }
    }
    return 0 as libc::c_int;
}
static mut destfd: [*mut libc::c_int; 2] = [0 as *const libc::c_int
    as *mut libc::c_int; 2];
unsafe extern "C" fn sendmessage(
    mut dir: at_direction,
    mut msg: *mut at_msg,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    if !((*msg).h.datalen as libc::c_int <= 256 as libc::c_int) {
        __assert_fail(
            b"msg->h.datalen <= MSG_LEN_MAX\0" as *const u8 as *const libc::c_char,
            b"src/allocator_thread.c\0" as *const u8 as *const libc::c_char,
            193 as libc::c_uint,
            b"sendmessage\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp = trywrite(
        *destfd[dir as usize],
        msg as *mut libc::c_void,
        (::std::mem::size_of::<at_msghdr>() as libc::c_ulong)
            .wrapping_add((*msg).h.datalen as libc::c_ulong),
    );
    ret = tmp;
    if !((*msg).h.datalen as libc::c_int <= 256 as libc::c_int) {
        __assert_fail(
            b"msg->h.datalen <= MSG_LEN_MAX\0" as *const u8 as *const libc::c_char,
            b"src/allocator_thread.c\0" as *const u8 as *const libc::c_char,
            195 as libc::c_uint,
            b"sendmessage\0" as *const u8 as *const libc::c_char,
        );
    }
    return ret;
}
unsafe extern "C" fn tryread(
    mut fd: libc::c_int,
    mut buf___0: *mut libc::c_void,
    mut bytes: size_t,
) -> libc::c_int {
    let mut ret: ssize_t = 0;
    let mut out: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    out = buf___0 as *mut libc::c_uchar;
    loop {
        ret = read(fd, out as *mut libc::c_void, bytes);
        match ret {
            -1 => {
                tmp = __errno_location();
                if !(*tmp == 4 as libc::c_int) {
                    break;
                }
            }
            0 => {
                break;
            }
            _ => {
                if ret as size_t == bytes {
                    return 1 as libc::c_int
                } else {
                    if bytes == 0 {
                        return 1 as libc::c_int;
                    }
                }
                out = out.offset(ret as isize);
                bytes = (bytes as libc::c_ulong).wrapping_sub(ret as size_t) as size_t
                    as size_t;
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn readmsg(mut fd: libc::c_int, mut msg: *mut at_msg) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = tryread(
        fd,
        msg as *mut libc::c_void,
        ::std::mem::size_of::<at_msghdr>() as libc::c_ulong,
    );
    ret = tmp;
    if ret != 1 as libc::c_int {
        return ret;
    }
    tmp___0 = tryread(
        fd,
        &mut (*msg).m as *mut __anonunion_m_242812694 as *mut libc::c_void,
        (*msg).h.datalen as size_t,
    );
    return tmp___0;
}
static mut readfd: [*mut libc::c_int; 2] = [0 as *const libc::c_int
    as *mut libc::c_int; 2];
unsafe extern "C" fn getmessage(
    mut dir: at_direction,
    mut msg: *mut at_msg,
) -> libc::c_int {
    let mut ret: ssize_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp___0 = wait_data(*readfd[dir as usize]);
    ret = tmp___0 as ssize_t;
    if ret != 0 {
        tmp = readmsg(*readfd[dir as usize], msg);
        if tmp == 0 {
            return 0 as libc::c_int;
        }
        if !((*msg).h.datalen as libc::c_int <= 256 as libc::c_int) {
            __assert_fail(
                b"msg->h.datalen <= MSG_LEN_MAX\0" as *const u8 as *const libc::c_char,
                b"src/allocator_thread.c\0" as *const u8 as *const libc::c_char,
                228 as libc::c_uint,
                b"getmessage\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    return ret as libc::c_int;
}
unsafe extern "C" fn threadfunc(mut x: *mut libc::c_void) -> *mut libc::c_void {
    let mut ret: libc::c_int = 0;
    let mut msg: at_msg = at_msg {
        h: at_msghdr {
            msgtype: 0,
            reserved: 0,
            datalen: 0,
        },
        m: __anonunion_m_242812694 {
            host: [0; 260],
        },
    };
    let mut host: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: size_t = 0;
    let mut tmp___0: size_t = 0;
    loop {
        ret = getmessage(ATD_SERVER, &mut msg);
        if ret == 0 {
            break;
        }
        match msg.h.msgtype as libc::c_int {
            0 => {
                msg
                    .m
                    .ip = ip_from_internal_list(
                    (msg.m.host).as_mut_ptr(),
                    msg.h.datalen as size_t,
                );
                msg
                    .h
                    .datalen = ::std::mem::size_of::<ip_type4>() as libc::c_ulong
                    as libc::c_ushort;
            }
            1 => {
                tmp = string_from_internal_ip(msg.m.ip);
                host = tmp;
                if !host.is_null() {
                    tmp___0 = strlen(host as *const libc::c_char);
                    l = tmp___0;
                    if !(l.wrapping_add(1 as libc::c_ulong) < 256 as libc::c_ulong) {
                        __assert_fail(
                            b"l+1 < MSG_LEN_MAX\0" as *const u8 as *const libc::c_char,
                            b"src/allocator_thread.c\0" as *const u8
                                as *const libc::c_char,
                            249 as libc::c_uint,
                            b"threadfunc\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    memcpy(
                        (msg.m.host).as_mut_ptr() as *mut libc::c_void,
                        host as *const libc::c_void,
                        l.wrapping_add(1 as libc::c_ulong),
                    );
                    msg.h.datalen = l.wrapping_add(1 as libc::c_ulong) as libc::c_ushort;
                } else {
                    msg.h.datalen = 0 as libc::c_int as libc::c_ushort;
                }
            }
            3 => return 0 as *mut libc::c_void,
            _ => {
                abort();
            }
        }
        ret = sendmessage(ATD_CLIENT, &mut msg);
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn at_get_ip_for_host(
    mut host: *mut libc::c_char,
    mut len: size_t,
) -> ip_type4 {
    let mut current_block: u64;
    let mut readbuf: ip_type4 = __anonunion_ip_type4_826858479 {
        octet: [0; 4],
    };
    let mut msg: at_msg = at_msg {
        h: at_msghdr {
            msgtype: 0,
            reserved: 0,
            datalen: 0,
        },
        m: __anonunion_m_242812694 {
            host: [0; 260],
        },
    };
    let mut __constr_expr_8: ip_type4 = __anonunion_ip_type4_826858479 {
        octet: [0; 4],
    };
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    pthread_mutex_lock(internal_ips_lock);
    if len > 256 as libc::c_ulong {
        current_block = 13325343358447810352;
    } else {
        msg.h.msgtype = 0 as libc::c_int as libc::c_uchar;
        msg.h.reserved = 0 as libc::c_int as libc::c_char;
        msg.h.datalen = len.wrapping_add(1 as libc::c_ulong) as libc::c_ushort;
        msg.m.host[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[3 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[4 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[5 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[6 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[7 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[9 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[10 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[11 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[12 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[13 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[14 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[15 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[16 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[17 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[18 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[19 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[20 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[21 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[22 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[23 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[24 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[25 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[26 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[27 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[28 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[29 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[30 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[31 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[32 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[33 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[34 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[35 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[36 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[37 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[38 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[39 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[40 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[41 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[42 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[43 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[44 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[45 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[46 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[47 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[48 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[49 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[50 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[51 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[52 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[53 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[54 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[55 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[56 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[57 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[58 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[59 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[60 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[61 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[62 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[63 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[64 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[65 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[66 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[67 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[68 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[69 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[70 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[71 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[72 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[73 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[74 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[75 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[76 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[77 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[78 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[79 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[80 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[81 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[82 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[83 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[84 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[85 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[86 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[87 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[88 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[89 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[90 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[91 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[92 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[93 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[94 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[95 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[96 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[97 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[98 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[99 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[100 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[101 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[102 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[103 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[104 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[105 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[106 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[107 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[108 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[109 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[110 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[111 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[112 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[113 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[114 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[115 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[116 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[117 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[118 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[119 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[120 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[121 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[122 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[123 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[124 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[125 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[126 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[127 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[128 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[129 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[130 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[131 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[132 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[133 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[134 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[135 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[136 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[137 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[138 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[139 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[140 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[141 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[142 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[143 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[144 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[145 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[146 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[147 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[148 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[149 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[150 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[151 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[152 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[153 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[154 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[155 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[156 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[157 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[158 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[159 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[160 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[161 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[162 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[163 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[164 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[165 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[166 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[167 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[168 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[169 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[170 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[171 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[172 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[173 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[174 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[175 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[176 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[177 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[178 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[179 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[180 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[181 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[182 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[183 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[184 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[185 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[186 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[187 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[188 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[189 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[190 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[191 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[192 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[193 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[194 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[195 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[196 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[197 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[198 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[199 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[200 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[201 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[202 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[203 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[204 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[205 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[206 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[207 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[208 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[209 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[210 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[211 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[212 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[213 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[214 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[215 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[216 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[217 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[218 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[219 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[220 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[221 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[222 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[223 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[224 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[225 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[226 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[227 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[228 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[229 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[230 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[231 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[232 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[233 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[234 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[235 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[236 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[237 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[238 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[239 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[240 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[241 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[242 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[243 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[244 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[245 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[246 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[247 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[248 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[249 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[250 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[251 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[252 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[253 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[254 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[255 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[256 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[257 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[258 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        msg.m.host[259 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        memcpy(
            (msg.m.host).as_mut_ptr() as *mut libc::c_void,
            host as *const libc::c_void,
            len.wrapping_add(1 as libc::c_ulong),
        );
        tmp = sendmessage(ATD_SERVER, &mut msg);
        if tmp != 0 {
            tmp___0 = getmessage(ATD_CLIENT, &mut msg);
            if tmp___0 != 0 {
                readbuf = msg.m.ip;
                current_block = 9872982588115335912;
            } else {
                current_block = 13325343358447810352;
            }
        } else {
            current_block = 13325343358447810352;
        }
    }
    match current_block {
        13325343358447810352 => {
            __constr_expr_8.as_int = -(1 as libc::c_int) as uint32_t;
            readbuf = __constr_expr_8;
        }
        _ => {}
    }
    if !(msg.h.msgtype as libc::c_int == 0 as libc::c_int) {
        __assert_fail(
            b"msg.h.msgtype == ATM_GETIP\0" as *const u8 as *const libc::c_char,
            b"src/allocator_thread.c\0" as *const u8 as *const libc::c_char,
            281 as libc::c_uint,
            b"at_get_ip_for_host\0" as *const u8 as *const libc::c_char,
        );
    }
    pthread_mutex_unlock(internal_ips_lock);
    return readbuf;
}
pub unsafe extern "C" fn at_get_host_for_ip(
    mut ip: ip_type4,
    mut readbuf: *mut libc::c_char,
) -> size_t {
    let mut msg: at_msg = at_msg {
        h: at_msghdr {
            msgtype: 0,
            reserved: 0,
            datalen: 0,
        },
        m: __anonunion_m_242812694 {
            host: [0; 260],
        },
    };
    let mut res: size_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    msg.h.msgtype = 1 as libc::c_int as libc::c_uchar;
    msg.h.reserved = 0 as libc::c_int as libc::c_char;
    msg.h.datalen = ::std::mem::size_of::<ip_type4>() as libc::c_ulong as libc::c_ushort;
    msg.m.ip = ip;
    res = 0 as libc::c_int as size_t;
    pthread_mutex_lock(internal_ips_lock);
    tmp = sendmessage(ATD_SERVER, &mut msg);
    if tmp != 0 {
        tmp___0 = getmessage(ATD_CLIENT, &mut msg);
        if tmp___0 != 0 {
            if msg.h.datalen as int16_t as libc::c_int <= 0 as libc::c_int {
                res = 0 as libc::c_int as size_t;
            } else {
                memcpy(
                    readbuf as *mut libc::c_void,
                    (msg.m.host).as_mut_ptr() as *const libc::c_void,
                    msg.h.datalen as size_t,
                );
                res = (msg.h.datalen as libc::c_int - 1 as libc::c_int) as size_t;
            }
        }
    }
    if !(msg.h.msgtype as libc::c_int == 1 as libc::c_int) {
        __assert_fail(
            b"msg.h.msgtype == ATM_GETNAME\0" as *const u8 as *const libc::c_char,
            b"src/allocator_thread.c\0" as *const u8 as *const libc::c_char,
            297 as libc::c_uint,
            b"at_get_host_for_ip\0" as *const u8 as *const libc::c_char,
        );
    }
    pthread_mutex_unlock(internal_ips_lock);
    return res;
}
unsafe extern "C" fn initpipe(mut fds: *mut libc::c_int) {
    let mut retval: libc::c_int = 0;
    retval = pipe2(fds, 524288 as libc::c_int);
    if retval == -(1 as libc::c_int) {
        perror(b"pipe\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
}
pub unsafe extern "C" fn at_init() {
    let mut shm: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut allocator_thread_attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    tmp = mmap(
        0 as *mut libc::c_void,
        4096 as libc::c_int as size_t,
        3 as libc::c_int,
        33 as libc::c_int,
        -(1 as libc::c_int),
        0 as libc::c_int as __off_t,
    );
    shm = tmp;
    if shm.is_null() {
        __assert_fail(
            b"shm\0" as *const u8 as *const libc::c_char,
            b"src/allocator_thread.c\0" as *const u8 as *const libc::c_char,
            335 as libc::c_uint,
            b"at_init\0" as *const u8 as *const libc::c_char,
        );
    }
    internal_ips_lock = shm as *mut pthread_mutex_t;
    internal_ips = (shm as *mut libc::c_char).offset(2048 as libc::c_int as isize)
        as *mut libc::c_void as *mut internal_ip_lookup_table;
    pthread_mutex_init(
        internal_ips_lock,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
    memset(
        internal_ips as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<internal_ip_lookup_table>() as libc::c_ulong,
    );
    initpipe(req_pipefd.as_mut_ptr());
    initpipe(resp_pipefd.as_mut_ptr());
    pthread_attr_init(&mut allocator_thread_attr);
    pthread_attr_setstacksize(
        &mut allocator_thread_attr,
        16384 as libc::c_int as size_t,
    );
    pthread_create(
        &mut allocator_thread as *mut pthread_t,
        &mut allocator_thread_attr as *mut pthread_attr_t as *const pthread_attr_t,
        Some(threadfunc as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        0 as *mut libc::c_void,
    );
    pthread_attr_destroy(&mut allocator_thread_attr);
}
pub unsafe extern "C" fn at_close() {
    let mut msg: libc::c_int = 0;
    msg = 3 as libc::c_int;
    write(
        req_pipefd[1 as libc::c_int as usize],
        &mut msg as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    );
    pthread_join(allocator_thread, 0 as *mut libc::c_void as *mut *mut libc::c_void);
    close(req_pipefd[0 as libc::c_int as usize]);
    close(req_pipefd[1 as libc::c_int as usize]);
    close(resp_pipefd[0 as libc::c_int as usize]);
    close(resp_pipefd[1 as libc::c_int as usize]);
    pthread_mutex_destroy(internal_ips_lock);
}
static mut rdns_server: sockaddr_in = sockaddr_in {
    sin_family: 0,
    sin_port: 0,
    sin_addr: in_addr { s_addr: 0 },
    sin_zero: [0; 8],
};
pub unsafe extern "C" fn rdns_daemon_get_host_for_ip(
    mut ip: ip_type4,
    mut readbuf: *mut libc::c_char,
) -> size_t {
    let mut msg: at_msg = at_msg {
        h: at_msghdr {
            msgtype: 0,
            reserved: 0,
            datalen: 0,
        },
        m: __anonunion_m_242812694 {
            host: [0; 260],
        },
    };
    let mut tmp: uint16_t = 0;
    let mut fd: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = htons(4 as libc::c_int as uint16_t);
    msg.h.msgtype = 1 as libc::c_int as libc::c_uchar;
    msg.h.reserved = 0 as libc::c_int as libc::c_char;
    msg.h.datalen = tmp;
    msg.m.ip = ip;
    tmp___0 = socket(2 as libc::c_int, 524290 as libc::c_int, 0 as libc::c_int);
    fd = tmp___0;
    sendto(
        fd,
        &mut msg as *mut at_msg as *const libc::c_void,
        (::std::mem::size_of::<at_msghdr>() as libc::c_ulong)
            .wrapping_add(4 as libc::c_ulong),
        0 as libc::c_int,
        &mut rdns_server as *mut sockaddr_in as *mut libc::c_void as *const sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    );
    recvfrom(
        fd,
        &mut msg as *mut at_msg as *mut libc::c_void,
        ::std::mem::size_of::<at_msg>() as libc::c_ulong,
        0 as libc::c_int,
        0 as *mut libc::c_void as *mut sockaddr,
        0 as *mut libc::c_void as *mut socklen_t,
    );
    close(fd);
    msg.h.datalen = ntohs(msg.h.datalen);
    if msg.h.datalen == 0 {
        return 0 as libc::c_int as size_t
    } else {
        if msg.h.datalen as libc::c_int > 256 as libc::c_int {
            return 0 as libc::c_int as size_t;
        }
    }
    memcpy(
        readbuf as *mut libc::c_void,
        (msg.m.host).as_mut_ptr() as *const libc::c_void,
        msg.h.datalen as size_t,
    );
    return (msg.h.datalen as libc::c_int - 1 as libc::c_int) as size_t;
}
unsafe extern "C" fn rdns_daemon_get_ip_for_host(
    mut host: *mut libc::c_char,
    mut len: size_t,
) -> ip_type4 {
    let mut msg: at_msg = at_msg {
        h: at_msghdr {
            msgtype: 0,
            reserved: 0,
            datalen: 0,
        },
        m: __anonunion_m_242812694 {
            host: [0; 260],
        },
    };
    let mut __constr_expr_9: ip_type4 = __anonunion_ip_type4_826858479 {
        octet: [0; 4],
    };
    let mut fd: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut __constr_expr_10: ip_type4 = __anonunion_ip_type4_826858479 {
        octet: [0; 4],
    };
    let mut tmp___0: uint16_t = 0;
    msg.h.msgtype = 0 as libc::c_int as libc::c_uchar;
    msg.h.reserved = 0 as libc::c_int as libc::c_char;
    msg.h.datalen = 0 as libc::c_int as libc::c_ushort;
    msg.m.host[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[3 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[4 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[5 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[6 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[7 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[9 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[10 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[11 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[12 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[13 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[14 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[15 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[16 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[17 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[18 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[19 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[20 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[21 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[22 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[23 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[24 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[25 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[26 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[27 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[28 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[29 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[30 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[31 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[32 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[33 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[34 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[35 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[36 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[37 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[38 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[39 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[40 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[41 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[42 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[43 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[44 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[45 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[46 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[47 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[48 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[49 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[50 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[51 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[52 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[53 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[54 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[55 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[56 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[57 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[58 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[59 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[60 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[61 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[62 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[63 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[64 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[65 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[66 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[67 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[68 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[69 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[70 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[71 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[72 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[73 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[74 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[75 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[76 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[77 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[78 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[79 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[80 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[81 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[82 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[83 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[84 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[85 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[86 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[87 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[88 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[89 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[90 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[91 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[92 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[93 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[94 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[95 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[96 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[97 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[98 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[99 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[100 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[101 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[102 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[103 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[104 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[105 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[106 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[107 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[108 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[109 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[110 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[111 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[112 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[113 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[114 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[115 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[116 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[117 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[118 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[119 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[120 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[121 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[122 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[123 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[124 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[125 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[126 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[127 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[128 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[129 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[130 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[131 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[132 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[133 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[134 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[135 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[136 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[137 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[138 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[139 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[140 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[141 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[142 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[143 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[144 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[145 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[146 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[147 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[148 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[149 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[150 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[151 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[152 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[153 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[154 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[155 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[156 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[157 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[158 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[159 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[160 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[161 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[162 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[163 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[164 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[165 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[166 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[167 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[168 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[169 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[170 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[171 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[172 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[173 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[174 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[175 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[176 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[177 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[178 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[179 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[180 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[181 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[182 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[183 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[184 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[185 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[186 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[187 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[188 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[189 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[190 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[191 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[192 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[193 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[194 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[195 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[196 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[197 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[198 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[199 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[200 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[201 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[202 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[203 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[204 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[205 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[206 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[207 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[208 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[209 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[210 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[211 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[212 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[213 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[214 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[215 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[216 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[217 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[218 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[219 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[220 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[221 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[222 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[223 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[224 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[225 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[226 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[227 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[228 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[229 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[230 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[231 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[232 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[233 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[234 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[235 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[236 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[237 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[238 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[239 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[240 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[241 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[242 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[243 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[244 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[245 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[246 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[247 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[248 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[249 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[250 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[251 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[252 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[253 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[254 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[255 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[256 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[257 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[258 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    msg.m.host[259 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    if len >= 256 as libc::c_ulong {
        __constr_expr_9.as_int = -(1 as libc::c_int) as uint32_t;
        return __constr_expr_9;
    }
    memcpy(
        (msg.m.host).as_mut_ptr() as *mut libc::c_void,
        host as *const libc::c_void,
        len.wrapping_add(1 as libc::c_ulong),
    );
    msg.h.datalen = htons(len.wrapping_add(1 as libc::c_ulong) as uint16_t);
    tmp = socket(2 as libc::c_int, 524290 as libc::c_int, 0 as libc::c_int);
    fd = tmp;
    sendto(
        fd,
        &mut msg as *mut at_msg as *const libc::c_void,
        (::std::mem::size_of::<at_msghdr>() as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_ulong),
        0 as libc::c_int,
        &mut rdns_server as *mut sockaddr_in as *mut libc::c_void as *const sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    );
    recvfrom(
        fd,
        &mut msg as *mut at_msg as *mut libc::c_void,
        ::std::mem::size_of::<at_msg>() as libc::c_ulong,
        0 as libc::c_int,
        0 as *mut libc::c_void as *mut sockaddr,
        0 as *mut libc::c_void as *mut socklen_t,
    );
    close(fd);
    tmp___0 = ntohs(msg.h.datalen);
    if tmp___0 as libc::c_int != 4 as libc::c_int {
        __constr_expr_10.as_int = -(1 as libc::c_int) as uint32_t;
        return __constr_expr_10;
    }
    return msg.m.ip;
}
static mut tab: [[libc::c_char; 7]; 4] = [
    [
        'o' as i32 as libc::c_char,
        'f' as i32 as libc::c_char,
        'f' as i32 as libc::c_char,
        '\u{0}' as i32 as libc::c_char,
        0,
        0,
        0,
    ],
    [
        'o' as i32 as libc::c_char,
        'l' as i32 as libc::c_char,
        'd' as i32 as libc::c_char,
        '\u{0}' as i32 as libc::c_char,
        0,
        0,
        0,
    ],
    [
        't' as i32 as libc::c_char,
        'h' as i32 as libc::c_char,
        'r' as i32 as libc::c_char,
        'e' as i32 as libc::c_char,
        'a' as i32 as libc::c_char,
        'd' as i32 as libc::c_char,
        '\u{0}' as i32 as libc::c_char,
    ],
    [
        'd' as i32 as libc::c_char,
        'a' as i32 as libc::c_char,
        'e' as i32 as libc::c_char,
        'm' as i32 as libc::c_char,
        'o' as i32 as libc::c_char,
        'n' as i32 as libc::c_char,
        '\u{0}' as i32 as libc::c_char,
    ],
];
pub unsafe extern "C" fn rdns_resolver_string(
    mut flavor: dns_lookup_flavor,
) -> *const libc::c_char {
    return (tab[flavor as usize]).as_ptr();
}
static mut init_done: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn rdns_init(mut flavor: dns_lookup_flavor) {
    if init_done == 0 {
        match flavor as libc::c_uint {
            2 => {
                at_init();
            }
            _ => {}
        }
    }
    init_done = 1 as libc::c_int;
}
pub unsafe extern "C" fn rdns_set_daemon(mut addr: *mut sockaddr_in) {
    rdns_server = *addr;
}
pub unsafe extern "C" fn rdns_get_host_for_ip(
    mut ip: ip_type4,
    mut readbuf: *mut libc::c_char,
) -> size_t {
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    match proxychains_resolver as libc::c_uint {
        2 => {
            tmp = at_get_host_for_ip(ip, readbuf);
            return tmp;
        }
        3 => {
            tmp___0 = rdns_daemon_get_host_for_ip(ip, readbuf);
            return tmp___0;
        }
        _ => {
            abort();
        }
    };
}
pub unsafe extern "C" fn rdns_get_ip_for_host(
    mut host: *mut libc::c_char,
    mut len: size_t,
) -> ip_type4 {
    let mut tmp: ip_type4 = __anonunion_ip_type4_826858479 {
        octet: [0; 4],
    };
    let mut tmp___0: ip_type4 = __anonunion_ip_type4_826858479 {
        octet: [0; 4],
    };
    match proxychains_resolver as libc::c_uint {
        2 => {
            tmp = at_get_ip_for_host(host, len);
            return tmp;
        }
        3 => {
            tmp___0 = rdns_daemon_get_ip_for_host(host, len);
            return tmp___0;
        }
        _ => {
            abort();
        }
    };
}
pub unsafe extern "C" fn hostsreader_open(mut ctx: *mut hostsreader) -> libc::c_int {
    let mut tmp: *mut FILE = 0 as *mut FILE;
    tmp = fopen(
        b"/etc/hosts\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    (*ctx).f = tmp;
    if tmp.is_null() {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn hostsreader_close(mut ctx: *mut hostsreader) {
    fclose((*ctx).f);
}
pub unsafe extern "C" fn hostsreader_get(
    mut ctx: *mut hostsreader,
    mut buf___0: *mut libc::c_char,
    mut bufsize: size_t,
) -> libc::c_int {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: size_t = 0;
    let mut tmp___0: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut tmp___1: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut tmp___2: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut tmp___3: libc::c_int = 0;
    loop {
        tmp = fgets(buf___0, bufsize as libc::c_int, (*ctx).f);
        if tmp.is_null() {
            return 0 as libc::c_int;
        }
        if *buf___0 as libc::c_int == 35 as libc::c_int {
            continue;
        }
        p = buf___0;
        l = bufsize;
        (*ctx).ip = p;
        while *p != 0 {
            tmp___0 = __ctype_b_loc();
            if *(*tmp___0).offset(*p as libc::c_int as isize) as libc::c_int
                & 8192 as libc::c_int != 0
            {
                break;
            }
            if l == 0 {
                break;
            }
            p = p.offset(1);
            l = l.wrapping_sub(1);
        }
        if l == 0 {
            continue;
        }
        if *p == 0 {
            continue;
        }
        if p as libc::c_ulong == (*ctx).ip as libc::c_ulong {
            continue;
        }
        *p = 0 as libc::c_int as libc::c_char;
        p = p.offset(1);
        while *p != 0 {
            tmp___1 = __ctype_b_loc();
            if !(*(*tmp___1).offset(*p as libc::c_int as isize) as libc::c_int
                & 8192 as libc::c_int != 0)
            {
                break;
            }
            if l == 0 {
                break;
            }
            p = p.offset(1);
            l = l.wrapping_sub(1);
        }
        if l == 0 {
            continue;
        }
        if *p == 0 {
            continue;
        }
        (*ctx).name = p;
        while *p != 0 {
            tmp___2 = __ctype_b_loc();
            if *(*tmp___2).offset(*p as libc::c_int as isize) as libc::c_int
                & 8192 as libc::c_int != 0
            {
                break;
            }
            if l == 0 {
                break;
            }
            p = p.offset(1);
            l = l.wrapping_sub(1);
        }
        if l == 0 {
            continue;
        }
        if *p == 0 {
            continue;
        }
        *p = 0 as libc::c_int as libc::c_char;
        tmp___3 = pc_isnumericipv4((*ctx).ip as *const libc::c_char);
        if tmp___3 != 0 {
            return 1 as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn hostsreader_get_ip_for_name(
    mut name: *const libc::c_char,
    mut buf___0: *mut libc::c_char,
    mut bufsize: size_t,
) -> *mut libc::c_char {
    let mut ctx: hostsreader = hostsreader {
        f: 0 as *mut FILE,
        ip: 0 as *mut libc::c_char,
        name: 0 as *mut libc::c_char,
    };
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    res = 0 as *mut libc::c_char;
    tmp = hostsreader_open(&mut ctx);
    if tmp == 0 {
        return 0 as *mut libc::c_char;
    }
    loop {
        tmp___1 = hostsreader_get(&mut ctx, buf___0, bufsize);
        if tmp___1 == 0 {
            break;
        }
        tmp___0 = strcmp(ctx.name as *const libc::c_char, name);
        if !(tmp___0 == 0) {
            continue;
        }
        res = ctx.ip;
        break;
    }
    hostsreader_close(&mut ctx);
    return res;
}
pub unsafe extern "C" fn hostsreader_get_numeric_ip_for_name(
    mut name: *const libc::c_char,
) -> ip_type4 {
    let mut hres: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf___0: [libc::c_char; 320] = [0; 320];
    let mut c: in_addr = in_addr { s_addr: 0 };
    let mut res: ip_type4 = __anonunion_ip_type4_826858479 {
        octet: [0; 4],
    };
    let mut __constr_expr_11: ip_type4 = __anonunion_ip_type4_826858479 {
        octet: [0; 4],
    };
    hres = hostsreader_get_ip_for_name(
        name,
        buf___0.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 320]>() as libc::c_ulong,
    );
    if !hres.is_null() {
        inet_aton(hres as *const libc::c_char, &mut c);
        memcpy(
            (res.octet).as_mut_ptr() as *mut libc::c_void,
            &mut c.s_addr as *mut in_addr_t as *const libc::c_void,
            4 as libc::c_int as size_t,
        );
        return res;
    } else {
        __constr_expr_11.as_int = -(1 as libc::c_int) as uint32_t;
        return __constr_expr_11;
    };
}
pub unsafe extern "C" fn dalias_hash(mut s0: *mut libc::c_char) -> uint32_t {
    let mut s: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut h: uint_fast32_t = 0;
    let mut tmp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    s = s0 as *mut libc::c_void as *mut libc::c_uchar;
    h = 0 as libc::c_int as uint_fast32_t;
    while *s != 0 {
        tmp = s;
        s = s.offset(1);
        h = (16 as libc::c_ulong).wrapping_mul(h).wrapping_add(*tmp as uint_fast32_t);
        h ^= h >> 24 as libc::c_int & 240 as libc::c_ulong;
    }
    return (h & 268435455 as libc::c_ulong) as uint32_t;
}
unsafe extern "C" fn run_static_initializers() {
    destfd = [
        &mut *req_pipefd.as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut libc::c_int,
        &mut *resp_pipefd.as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut libc::c_int,
    ];
    readfd = [
        &mut *req_pipefd.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut libc::c_int,
        &mut *resp_pipefd.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut libc::c_int,
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
