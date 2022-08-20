use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
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
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
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
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
    fn gethostent() -> *mut hostent;
    fn getservbyname_r(
        __name: *const libc::c_char,
        __proto: *const libc::c_char,
        __result_buf: *mut servent,
        __buf: *mut libc::c_char,
        __buflen: size_t,
        __result: *mut *mut servent,
    ) -> libc::c_int;
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
    fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: socklen_t,
    ) -> *const libc::c_char;
    fn inet_aton(__cp: *const libc::c_char, __inp: *mut in_addr) -> libc::c_int;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn perror(__s: *const libc::c_char);
    fn dlsym(
        __handle: *mut libc::c_void,
        __name: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn dlerror() -> *mut libc::c_char;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn abort() -> !;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn inet_pton(
        __af: libc::c_int,
        __cp: *const libc::c_char,
        __buf: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_once(
        __once_control: *mut pthread_once_t,
        __init_routine: Option::<unsafe extern "C" fn() -> ()>,
    ) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
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
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
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
pub union __anonunion_pthread_mutex_t_335460617 {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type pthread_mutex_t = __anonunion_pthread_mutex_t_335460617;
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
pub type uint_fast32_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_ip_type_826858479 {
    pub octet: [libc::c_uchar; 4],
    pub as_int: uint32_t,
}
pub type ip_type = __anonunion_ip_type_826858479;
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
pub type __anonenum_proxy_type_303997851 = libc::c_uint;
pub const SOCKS5_TYPE: __anonenum_proxy_type_303997851 = 3;
pub const SOCKS4_TYPE: __anonenum_proxy_type_303997851 = 2;
pub const RAW_TYPE: __anonenum_proxy_type_303997851 = 1;
pub const HTTP_TYPE: __anonenum_proxy_type_303997851 = 0;
pub type proxy_type = __anonenum_proxy_type_303997851;
pub type __anonenum_chain_type_847959665 = libc::c_uint;
pub const RANDOM_TYPE: __anonenum_chain_type_847959665 = 2;
pub const STRICT_TYPE: __anonenum_chain_type_847959665 = 1;
pub const DYNAMIC_TYPE: __anonenum_chain_type_847959665 = 0;
pub type chain_type = __anonenum_chain_type_847959665;
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
    pub addr_name: [libc::c_char; 8192],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo_data {
    pub addrinfo_space: addrinfo,
    pub sockaddr_space: sockaddr,
    pub addr_name: [libc::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_mutexattr_t_488594144 {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
pub type pthread_mutexattr_t = __anonunion_pthread_mutexattr_t_488594144;
pub type pthread_once_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_localaddr_arg_1073158885 {
    pub in_addr: in_addr,
    pub netmask: in_addr,
    pub port: libc::c_ushort,
}
pub type localaddr_arg = __anonstruct_localaddr_arg_1073158885;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_dnat_arg_1073158886 {
    pub orig_dst: in_addr,
    pub new_dst: in_addr,
    pub orig_port: libc::c_ushort,
    pub new_port: libc::c_ushort,
}
pub type dnat_arg = __anonstruct_dnat_arg_1073158886;
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 255 as libc::c_int
        | (__bsx as libc::c_int & 255 as libc::c_int) << 8 as libc::c_int) as __uint16_t;
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
pub static mut internal_ips_lock: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
pub static mut internal_ips: internal_ip_lookup_table = {
    let mut init = __anonstruct_internal_ip_lookup_table_78056353 {
        counter: 0 as libc::c_int as uint32_t,
        capa: 0 as libc::c_int as uint32_t,
        list: 0 as *const libc::c_void as *mut libc::c_void
            as *mut *mut string_hash_tuple,
    };
    init
};
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
pub unsafe extern "C" fn index_from_internal_ip(mut internalip: ip_type) -> uint32_t {
    let mut tmp: ip_type = __anonunion_ip_type_826858479 {
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
    mut internalip: ip_type,
) -> *mut libc::c_char {
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut index___0: uint32_t = 0;
    let mut tmp: uint32_t = 0;
    res = 0 as *mut libc::c_void as *mut libc::c_char;
    tmp = index_from_internal_ip(internalip);
    index___0 = tmp;
    pthread_mutex_lock(&mut internal_ips_lock);
    if index___0 < internal_ips.counter {
        res = (**(internal_ips.list).offset(index___0 as isize)).string;
    }
    pthread_mutex_unlock(&mut internal_ips_lock);
    return res;
}
pub unsafe extern "C" fn make_internal_ip(mut index___0: uint32_t) -> in_addr_t {
    let mut ret: ip_type = __anonunion_ip_type_826858479 {
        octet: [0; 4],
    };
    index___0 = index___0.wrapping_add(1);
    if index___0 > 16777215 as libc::c_uint {
        return -(1 as libc::c_int) as in_addr_t;
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
    return ret.as_int;
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
        time_elapsed = (tv.tv_sec - start_time.tv_sec) as libc::c_int
            * 1000 as libc::c_int
            + (tv.tv_usec - start_time.tv_usec) as libc::c_int / 1000 as libc::c_int;
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
unsafe extern "C" fn encode_base_64(
    mut src: *mut libc::c_char,
    mut dest: *mut libc::c_char,
    mut max_len: libc::c_int,
) {
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut l: size_t = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
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
    l = strlen(src as *const libc::c_char);
    max_len = (max_len - 1 as libc::c_int) / 4 as libc::c_int;
    i = 0 as libc::c_int;
    while i < max_len {
        match l {
            0 => {}
            1 => {
                n = (*src.offset(0 as libc::c_int as isize) as libc::c_int)
                    << 16 as libc::c_int;
                tmp = dest;
                dest = dest.offset(1);
                *tmp = base64[(n >> 18 as libc::c_int & 63 as libc::c_int) as usize];
                tmp___0 = dest;
                dest = dest.offset(1);
                *tmp___0 = base64[(n >> 12 as libc::c_int & 63 as libc::c_int) as usize];
                tmp___1 = dest;
                dest = dest.offset(1);
                *tmp___1 = '=' as i32 as libc::c_char;
                tmp___2 = dest;
                dest = dest.offset(1);
                *tmp___2 = '=' as i32 as libc::c_char;
            }
            2 => {
                n = (*src.offset(0 as libc::c_int as isize) as libc::c_int)
                    << 16 as libc::c_int
                    | (*src.offset(1 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int;
                tmp___3 = dest;
                dest = dest.offset(1);
                *tmp___3 = base64[(n >> 18 as libc::c_int & 63 as libc::c_int) as usize];
                tmp___4 = dest;
                dest = dest.offset(1);
                *tmp___4 = base64[(n >> 12 as libc::c_int & 63 as libc::c_int) as usize];
                tmp___5 = dest;
                dest = dest.offset(1);
                *tmp___5 = base64[(n >> 6 as libc::c_int & 63 as libc::c_int) as usize];
                tmp___6 = dest;
                dest = dest.offset(1);
                *tmp___6 = '=' as i32 as libc::c_char;
            }
            _ => {
                n = (*src.offset(0 as libc::c_int as isize) as libc::c_int)
                    << 16 as libc::c_int
                    | (*src.offset(1 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int
                    | *src.offset(2 as libc::c_int as isize) as libc::c_int;
                tmp___7 = dest;
                dest = dest.offset(1);
                *tmp___7 = base64[(n >> 18 as libc::c_int & 63 as libc::c_int) as usize];
                tmp___8 = dest;
                dest = dest.offset(1);
                *tmp___8 = base64[(n >> 12 as libc::c_int & 63 as libc::c_int) as usize];
                tmp___9 = dest;
                dest = dest.offset(1);
                *tmp___9 = base64[(n >> 6 as libc::c_int & 63 as libc::c_int) as usize];
                tmp___10 = dest;
                dest = dest.offset(1);
                *tmp___10 = base64[(n & 63 as libc::c_int) as usize];
            }
        }
        if l < 3 as libc::c_ulong {
            break;
        }
        i += 1;
        src = src.offset(3 as libc::c_int as isize);
        l = (l as libc::c_ulong).wrapping_sub(3 as libc::c_ulong) as size_t as size_t;
    }
    tmp___11 = dest;
    dest = dest.offset(1);
    *tmp___11 = 0 as libc::c_int as libc::c_char;
}
pub unsafe extern "C" fn proxychains_write_log(
    mut str: *mut libc::c_char,
    mut args: ...
) {
    let mut buff: [libc::c_char; 20480] = [0; 20480];
    let mut arglist: ::std::ffi::VaListImpl;
    if proxychains_quiet_mode == 0 {
        arglist = args.clone();
        vsnprintf(
            buff.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 20480]>() as libc::c_ulong,
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
) -> size_t {
    let mut i: size_t = 0;
    let mut wrote: size_t = 0;
    let mut tmp: ssize_t = 0;
    i = 0 as libc::c_int as size_t;
    wrote = 0 as libc::c_int as size_t;
    loop {
        tmp = write(
            fd,
            buff.offset(wrote as isize) as *const libc::c_void,
            size.wrapping_sub(wrote),
        );
        i = tmp as size_t;
        if i <= 0 as libc::c_ulong {
            return i;
        }
        wrote = (wrote as libc::c_ulong).wrapping_add(i) as size_t as size_t;
        if wrote == size {
            return wrote;
        }
    };
}
unsafe extern "C" fn read_n_bytes(
    mut fd: libc::c_int,
    mut buff: *mut libc::c_char,
    mut size: size_t,
) -> size_t {
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
            return 0 as libc::c_int as size_t
        } else {
            if pfd[0 as libc::c_int as usize].revents as libc::c_int & 1 as libc::c_int
                == 0
            {
                return 0 as libc::c_int as size_t
            } else {
                tmp = read(
                    fd,
                    buff.offset(i as isize) as *mut libc::c_void,
                    1 as libc::c_int as size_t,
                );
                if 1 as libc::c_long != tmp {
                    return 0 as libc::c_int as size_t;
                }
            }
        }
        i = i.wrapping_add(1);
    }
    return size;
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
    let mut current_block_23: u64;
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
            current_block_23 = 2232869372362427478;
        } else {
            current_block_23 = 5199137752231400485;
        }
    } else {
        current_block_23 = 5199137752231400485;
    }
    match current_block_23 {
        5199137752231400485 => {
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
    let mut dns_len: size_t = 0;
    let mut ulen: size_t = 0;
    let mut tmp: size_t = 0;
    let mut passlen: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut len: size_t = 0;
    let mut buff: [libc::c_uchar; 8192] = [0; 8192];
    let mut ip_buf: [libc::c_char; 16] = [0; 16];
    let mut tmp___1: __uint16_t = 0;
    let mut src: [libc::c_char; 512] = [0; 512];
    let mut dst: [libc::c_char; 2048] = [0; 2048];
    let mut tmp___2: ssize_t = 0;
    let mut tmp___3: size_t = 0;
    let mut tmp___4: size_t = 0;
    let mut tmp___5: size_t = 0;
    let mut tmp___6: size_t = 0;
    let mut tmp___7: size_t = 0;
    let mut tmp___8: size_t = 0;
    let mut in_0: [libc::c_char; 2] = [0; 2];
    let mut out: [libc::c_char; 515] = [0; 515];
    let mut cur: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    let mut tmp___9: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___10: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___11: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___12: size_t = 0;
    let mut tmp___13: size_t = 0;
    let mut buff_iter: size_t = 0;
    let mut tmp___14: size_t = 0;
    let mut tmp___15: size_t = 0;
    let mut tmp___16: size_t = 0;
    let mut tmp___17: size_t = 0;
    let mut tmp___18: size_t = 0;
    let mut tmp___19: size_t = 0;
    let mut tmp___20: size_t = 0;
    let mut tmp___21: size_t = 0;
    let mut tmp___22: size_t = 0;
    let mut tmp___23: size_t = 0;
    dns_name = 0 as *mut libc::c_void as *mut libc::c_char;
    dns_len = 0 as libc::c_int as size_t;
    if ip.octet[0 as libc::c_int as usize] as libc::c_uint == remote_dns_subnet {
        dns_name = string_from_internal_ip(ip);
        if dns_name.is_null() {
            current_block = 16889246570190342985;
        } else {
            dns_len = strlen(dns_name as *const libc::c_char);
            if dns_len == 0 {
                current_block = 16889246570190342985;
            } else {
                current_block = 18386322304582297246;
            }
        }
    } else {
        current_block = 18386322304582297246;
    }
    match current_block {
        18386322304582297246 => {
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
                match pt as libc::c_uint {
                    1 => {
                        current_block = 5643496190901308087;
                        match current_block {
                            5643496190901308087 => return 0 as libc::c_int,
                            13093596192184245794 => {
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
                                        .octet[0 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    ip
                                        .octet[1 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    ip
                                        .octet[2 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    ip
                                        .octet[3 as libc::c_int
                                        as usize] = 1 as libc::c_int as libc::c_uchar;
                                }
                                memcpy(
                                    &mut *buff.as_mut_ptr().offset(4 as libc::c_int as isize)
                                        as *mut libc::c_uchar as *mut libc::c_void,
                                    &mut ip as *mut ip_type as *const libc::c_void,
                                    4 as libc::c_int as size_t,
                                );
                                len = ulen.wrapping_add(1 as libc::c_ulong);
                                if len > 1 as libc::c_ulong {
                                    memcpy(
                                        &mut *buff.as_mut_ptr().offset(8 as libc::c_int as isize)
                                            as *mut libc::c_uchar as *mut libc::c_void,
                                        user as *const libc::c_void,
                                        len,
                                    );
                                } else {
                                    buff[8 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                }
                                if dns_len != 0 {
                                    memcpy(
                                        &mut *buff
                                            .as_mut_ptr()
                                            .offset((8 as libc::c_ulong).wrapping_add(len) as isize)
                                            as *mut libc::c_uchar as *mut libc::c_void,
                                        dns_name as *const libc::c_void,
                                        dns_len.wrapping_add(1 as libc::c_ulong),
                                    );
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(dns_len.wrapping_add(1 as libc::c_ulong))
                                        as size_t as size_t;
                                }
                                tmp___4 = write_n_bytes(
                                    sock,
                                    buff.as_mut_ptr() as *mut libc::c_char,
                                    (8 as libc::c_ulong).wrapping_add(len),
                                );
                                if !(len.wrapping_add(8 as libc::c_ulong) != tmp___4) {
                                    tmp___5 = read_n_bytes(
                                        sock,
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        8 as libc::c_int as size_t,
                                    );
                                    if !(8 as libc::c_ulong != tmp___5) {
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
                            2930800982520623112 => {
                                if dns_len == 0 {
                                    inet_ntop(
                                        2 as libc::c_int,
                                        &mut *(ip.octet)
                                            .as_mut_ptr()
                                            .offset(0 as libc::c_int as isize) as *mut libc::c_uchar
                                            as *const libc::c_void,
                                        ip_buf.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                            as socklen_t,
                                    );
                                    dns_name = ip_buf.as_mut_ptr();
                                }
                                tmp___1 = __bswap_16(port);
                                snprintf(
                                    buff.as_mut_ptr() as *mut libc::c_char,
                                    ::std::mem::size_of::<[libc::c_uchar; 8192]>()
                                        as libc::c_ulong,
                                    b"CONNECT %s:%d HTTP/1.0\r\n\0" as *const u8
                                        as *const libc::c_char,
                                    dns_name,
                                    tmp___1 as libc::c_int,
                                );
                                if *user.offset(0 as libc::c_int as isize) != 0 {
                                    memcpy(
                                        src.as_mut_ptr() as *mut libc::c_void,
                                        user as *const libc::c_void,
                                        ulen,
                                    );
                                    memcpy(
                                        src.as_mut_ptr().offset(ulen as isize) as *mut libc::c_void,
                                        b":\0" as *const u8 as *const libc::c_char
                                            as *const libc::c_void,
                                        1 as libc::c_int as size_t,
                                    );
                                    memcpy(
                                        src
                                            .as_mut_ptr()
                                            .offset(ulen as isize)
                                            .offset(1 as libc::c_int as isize) as *mut libc::c_void,
                                        pass as *const libc::c_void,
                                        passlen,
                                    );
                                    src[ulen
                                        .wrapping_add(1 as libc::c_ulong)
                                        .wrapping_add(passlen)
                                        as usize] = 0 as libc::c_int as libc::c_char;
                                    encode_base_64(
                                        src.as_mut_ptr(),
                                        dst.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 2048]>()
                                            as libc::c_ulong as libc::c_int,
                                    );
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        b"Proxy-Authorization: Basic \0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        dst.as_mut_ptr() as *const libc::c_char,
                                    );
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        b"\r\n\r\n\0" as *const u8 as *const libc::c_char,
                                    );
                                } else {
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        b"\r\n\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                len = strlen(
                                    buff.as_mut_ptr() as *mut libc::c_char
                                        as *const libc::c_char,
                                );
                                tmp___2 = send(
                                    sock,
                                    buff.as_mut_ptr() as *const libc::c_void,
                                    len,
                                    0 as libc::c_int,
                                );
                                if !(len != tmp___2 as size_t) {
                                    len = 0 as libc::c_int as size_t;
                                    loop {
                                        if !(len < 8192 as libc::c_ulong) {
                                            current_block = 15594839951440953787;
                                            break;
                                        }
                                        tmp___3 = read_n_bytes(
                                            sock,
                                            buff.as_mut_ptr().offset(len as isize) as *mut libc::c_char,
                                            1 as libc::c_int as size_t,
                                        );
                                        if !(1 as libc::c_ulong == tmp___3) {
                                            current_block = 16889246570190342985;
                                            break;
                                        }
                                        len = len.wrapping_add(1);
                                        if !(len > 4 as libc::c_ulong) {
                                            continue;
                                        }
                                        if !(buff[len.wrapping_sub(1 as libc::c_ulong) as usize]
                                            as libc::c_int == 10 as libc::c_int)
                                        {
                                            continue;
                                        }
                                        if !(buff[len.wrapping_sub(2 as libc::c_ulong) as usize]
                                            as libc::c_int == 13 as libc::c_int)
                                        {
                                            continue;
                                        }
                                        if !(buff[len.wrapping_sub(3 as libc::c_ulong) as usize]
                                            as libc::c_int == 10 as libc::c_int)
                                        {
                                            continue;
                                        }
                                        if buff[len.wrapping_sub(4 as libc::c_ulong) as usize]
                                            as libc::c_int == 13 as libc::c_int
                                        {
                                            current_block = 15594839951440953787;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        16889246570190342985 => {}
                                        _ => {
                                            if len == 8192 as libc::c_ulong {
                                                return 5 as libc::c_int
                                            } else {
                                                if buff[9 as libc::c_int as usize] as libc::c_int
                                                    == 50 as libc::c_int
                                                {
                                                    if buff[10 as libc::c_int as usize] as libc::c_int
                                                        == 48 as libc::c_int
                                                    {
                                                        if !(buff[11 as libc::c_int as usize] as libc::c_int
                                                            == 48 as libc::c_int)
                                                        {
                                                            return 5 as libc::c_int;
                                                        }
                                                    } else {
                                                        return 5 as libc::c_int
                                                    }
                                                } else {
                                                    return 5 as libc::c_int
                                                }
                                            }
                                            return 0 as libc::c_int;
                                        }
                                    }
                                }
                            }
                            _ => {
                                if !user.is_null() {
                                    buff[0 as libc::c_int
                                        as usize] = 5 as libc::c_int as libc::c_uchar;
                                    buff[1 as libc::c_int
                                        as usize] = 2 as libc::c_int as libc::c_uchar;
                                    buff[2 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    buff[3 as libc::c_int
                                        as usize] = 2 as libc::c_int as libc::c_uchar;
                                    tmp___6 = write_n_bytes(
                                        sock,
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        4 as libc::c_int as size_t,
                                    );
                                    if 4 as libc::c_ulong != tmp___6 {
                                        current_block = 16889246570190342985;
                                    } else {
                                        current_block = 13201766686570145889;
                                    }
                                } else {
                                    buff[0 as libc::c_int
                                        as usize] = 5 as libc::c_int as libc::c_uchar;
                                    buff[1 as libc::c_int
                                        as usize] = 1 as libc::c_int as libc::c_uchar;
                                    buff[2 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    tmp___7 = write_n_bytes(
                                        sock,
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        3 as libc::c_int as size_t,
                                    );
                                    if 3 as libc::c_ulong != tmp___7 {
                                        current_block = 16889246570190342985;
                                    } else {
                                        current_block = 13201766686570145889;
                                    }
                                }
                                match current_block {
                                    16889246570190342985 => {}
                                    _ => {
                                        tmp___8 = read_n_bytes(
                                            sock,
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            2 as libc::c_int as size_t,
                                        );
                                        if !(2 as libc::c_ulong != tmp___8) {
                                            if buff[0 as libc::c_int as usize] as libc::c_int
                                                != 5 as libc::c_int
                                            {
                                                current_block = 6887742563588325581;
                                            } else {
                                                if buff[1 as libc::c_int as usize] as libc::c_int
                                                    != 0 as libc::c_int
                                                {
                                                    if buff[1 as libc::c_int as usize] as libc::c_int
                                                        != 2 as libc::c_int
                                                    {
                                                        current_block = 6887742563588325581;
                                                    } else {
                                                        current_block = 16314074004867283505;
                                                    }
                                                } else {
                                                    current_block = 16314074004867283505;
                                                }
                                                match current_block {
                                                    6887742563588325581 => {}
                                                    _ => {
                                                        if buff[1 as libc::c_int as usize] as libc::c_int
                                                            == 2 as libc::c_int
                                                        {
                                                            cur = out.as_mut_ptr();
                                                            tmp___9 = cur;
                                                            cur = cur.offset(1);
                                                            *tmp___9 = 1 as libc::c_int as libc::c_char;
                                                            c = (ulen & 255 as libc::c_ulong) as libc::c_int;
                                                            tmp___10 = cur;
                                                            cur = cur.offset(1);
                                                            *tmp___10 = c as libc::c_char;
                                                            memcpy(
                                                                cur as *mut libc::c_void,
                                                                user as *const libc::c_void,
                                                                c as size_t,
                                                            );
                                                            cur = cur.offset(c as isize);
                                                            c = (passlen & 255 as libc::c_ulong) as libc::c_int;
                                                            tmp___11 = cur;
                                                            cur = cur.offset(1);
                                                            *tmp___11 = c as libc::c_char;
                                                            memcpy(
                                                                cur as *mut libc::c_void,
                                                                pass as *const libc::c_void,
                                                                c as size_t,
                                                            );
                                                            cur = cur.offset(c as isize);
                                                            tmp___12 = write_n_bytes(
                                                                sock,
                                                                out.as_mut_ptr(),
                                                                cur.offset_from(out.as_mut_ptr()) as libc::c_long as size_t,
                                                            );
                                                            if cur.offset_from(out.as_mut_ptr()) as libc::c_long
                                                                as size_t != tmp___12
                                                            {
                                                                current_block = 16889246570190342985;
                                                            } else {
                                                                tmp___13 = read_n_bytes(
                                                                    sock,
                                                                    in_0.as_mut_ptr(),
                                                                    2 as libc::c_int as size_t,
                                                                );
                                                                if 2 as libc::c_ulong != tmp___13 {
                                                                    current_block = 16889246570190342985;
                                                                } else {
                                                                    if in_0[0 as libc::c_int as usize] as libc::c_int
                                                                        != 1 as libc::c_int
                                                                    {
                                                                        current_block = 7368271564846522885;
                                                                    } else if in_0[1 as libc::c_int as usize] as libc::c_int
                                                                            != 0 as libc::c_int
                                                                        {
                                                                        current_block = 7368271564846522885;
                                                                    } else {
                                                                        current_block = 14883390358315838856;
                                                                    }
                                                                    match current_block {
                                                                        14883390358315838856 => {}
                                                                        _ => {
                                                                            if in_0[0 as libc::c_int as usize] as libc::c_int
                                                                                != 1 as libc::c_int
                                                                            {
                                                                                current_block = 16889246570190342985;
                                                                            } else {
                                                                                return 5 as libc::c_int
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            current_block = 14883390358315838856;
                                                        }
                                                        match current_block {
                                                            16889246570190342985 => {}
                                                            _ => {
                                                                buff_iter = 0 as libc::c_int as size_t;
                                                                tmp___14 = buff_iter;
                                                                buff_iter = buff_iter.wrapping_add(1);
                                                                buff[tmp___14 as usize] = 5 as libc::c_int as libc::c_uchar;
                                                                tmp___15 = buff_iter;
                                                                buff_iter = buff_iter.wrapping_add(1);
                                                                buff[tmp___15 as usize] = 1 as libc::c_int as libc::c_uchar;
                                                                tmp___16 = buff_iter;
                                                                buff_iter = buff_iter.wrapping_add(1);
                                                                buff[tmp___16 as usize] = 0 as libc::c_int as libc::c_uchar;
                                                                if dns_len == 0 {
                                                                    tmp___17 = buff_iter;
                                                                    buff_iter = buff_iter.wrapping_add(1);
                                                                    buff[tmp___17 as usize] = 1 as libc::c_int as libc::c_uchar;
                                                                    memcpy(
                                                                        buff.as_mut_ptr().offset(buff_iter as isize)
                                                                            as *mut libc::c_void,
                                                                        &mut ip as *mut ip_type as *const libc::c_void,
                                                                        4 as libc::c_int as size_t,
                                                                    );
                                                                    buff_iter = (buff_iter as libc::c_ulong)
                                                                        .wrapping_add(4 as libc::c_ulong) as size_t as size_t;
                                                                } else {
                                                                    tmp___18 = buff_iter;
                                                                    buff_iter = buff_iter.wrapping_add(1);
                                                                    buff[tmp___18 as usize] = 3 as libc::c_int as libc::c_uchar;
                                                                    tmp___19 = buff_iter;
                                                                    buff_iter = buff_iter.wrapping_add(1);
                                                                    buff[tmp___19
                                                                        as usize] = (dns_len & 255 as libc::c_ulong)
                                                                        as libc::c_uchar;
                                                                    memcpy(
                                                                        buff.as_mut_ptr().offset(buff_iter as isize)
                                                                            as *mut libc::c_void,
                                                                        dns_name as *const libc::c_void,
                                                                        dns_len,
                                                                    );
                                                                    buff_iter = (buff_iter as libc::c_ulong)
                                                                        .wrapping_add(dns_len) as size_t as size_t;
                                                                }
                                                                memcpy(
                                                                    buff.as_mut_ptr().offset(buff_iter as isize)
                                                                        as *mut libc::c_void,
                                                                    &mut port as *mut libc::c_ushort as *const libc::c_void,
                                                                    2 as libc::c_int as size_t,
                                                                );
                                                                buff_iter = (buff_iter as libc::c_ulong)
                                                                    .wrapping_add(2 as libc::c_ulong) as size_t as size_t;
                                                                tmp___20 = write_n_bytes(
                                                                    sock,
                                                                    buff.as_mut_ptr() as *mut libc::c_char,
                                                                    buff_iter,
                                                                );
                                                                if buff_iter != tmp___20 {
                                                                    current_block = 16889246570190342985;
                                                                } else {
                                                                    tmp___21 = read_n_bytes(
                                                                        sock,
                                                                        buff.as_mut_ptr() as *mut libc::c_char,
                                                                        4 as libc::c_int as size_t,
                                                                    );
                                                                    if 4 as libc::c_ulong != tmp___21 {
                                                                        current_block = 16889246570190342985;
                                                                    } else if buff[0 as libc::c_int as usize] as libc::c_int
                                                                            != 5 as libc::c_int
                                                                        {
                                                                        current_block = 16889246570190342985;
                                                                    } else if buff[1 as libc::c_int as usize] as libc::c_int
                                                                            != 0 as libc::c_int
                                                                        {
                                                                        current_block = 16889246570190342985;
                                                                    } else {
                                                                        match buff[3 as libc::c_int as usize] as libc::c_int {
                                                                            1 => {
                                                                                current_block = 9157488150074734138;
                                                                                match current_block {
                                                                                    14120677076171701346 => {
                                                                                        len = 16 as libc::c_int as size_t;
                                                                                        current_block = 9879293939512133550;
                                                                                    }
                                                                                    9157488150074734138 => {
                                                                                        len = 4 as libc::c_int as size_t;
                                                                                        current_block = 9879293939512133550;
                                                                                    }
                                                                                    _ => {
                                                                                        len = 0 as libc::c_int as size_t;
                                                                                        tmp___22 = read_n_bytes(
                                                                                            sock,
                                                                                            &mut len as *mut size_t as *mut libc::c_char,
                                                                                            1 as libc::c_int as size_t,
                                                                                        );
                                                                                        if 1 as libc::c_ulong != tmp___22 {
                                                                                            current_block = 16889246570190342985;
                                                                                        } else {
                                                                                            current_block = 9879293939512133550;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                match current_block {
                                                                                    16889246570190342985 => {}
                                                                                    _ => {
                                                                                        tmp___23 = read_n_bytes(
                                                                                            sock,
                                                                                            buff.as_mut_ptr() as *mut libc::c_char,
                                                                                            len.wrapping_add(2 as libc::c_ulong),
                                                                                        );
                                                                                        if len.wrapping_add(2 as libc::c_ulong) != tmp___23 {
                                                                                            current_block = 16889246570190342985;
                                                                                        } else {
                                                                                            return 0 as libc::c_int
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            4 => {
                                                                                current_block = 14120677076171701346;
                                                                                match current_block {
                                                                                    14120677076171701346 => {
                                                                                        len = 16 as libc::c_int as size_t;
                                                                                        current_block = 9879293939512133550;
                                                                                    }
                                                                                    9157488150074734138 => {
                                                                                        len = 4 as libc::c_int as size_t;
                                                                                        current_block = 9879293939512133550;
                                                                                    }
                                                                                    _ => {
                                                                                        len = 0 as libc::c_int as size_t;
                                                                                        tmp___22 = read_n_bytes(
                                                                                            sock,
                                                                                            &mut len as *mut size_t as *mut libc::c_char,
                                                                                            1 as libc::c_int as size_t,
                                                                                        );
                                                                                        if 1 as libc::c_ulong != tmp___22 {
                                                                                            current_block = 16889246570190342985;
                                                                                        } else {
                                                                                            current_block = 9879293939512133550;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                match current_block {
                                                                                    16889246570190342985 => {}
                                                                                    _ => {
                                                                                        tmp___23 = read_n_bytes(
                                                                                            sock,
                                                                                            buff.as_mut_ptr() as *mut libc::c_char,
                                                                                            len.wrapping_add(2 as libc::c_ulong),
                                                                                        );
                                                                                        if len.wrapping_add(2 as libc::c_ulong) != tmp___23 {
                                                                                            current_block = 16889246570190342985;
                                                                                        } else {
                                                                                            return 0 as libc::c_int
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            3 => {
                                                                                current_block = 2261659062341822993;
                                                                                match current_block {
                                                                                    14120677076171701346 => {
                                                                                        len = 16 as libc::c_int as size_t;
                                                                                        current_block = 9879293939512133550;
                                                                                    }
                                                                                    9157488150074734138 => {
                                                                                        len = 4 as libc::c_int as size_t;
                                                                                        current_block = 9879293939512133550;
                                                                                    }
                                                                                    _ => {
                                                                                        len = 0 as libc::c_int as size_t;
                                                                                        tmp___22 = read_n_bytes(
                                                                                            sock,
                                                                                            &mut len as *mut size_t as *mut libc::c_char,
                                                                                            1 as libc::c_int as size_t,
                                                                                        );
                                                                                        if 1 as libc::c_ulong != tmp___22 {
                                                                                            current_block = 16889246570190342985;
                                                                                        } else {
                                                                                            current_block = 9879293939512133550;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                match current_block {
                                                                                    16889246570190342985 => {}
                                                                                    _ => {
                                                                                        tmp___23 = read_n_bytes(
                                                                                            sock,
                                                                                            buff.as_mut_ptr() as *mut libc::c_char,
                                                                                            len.wrapping_add(2 as libc::c_ulong),
                                                                                        );
                                                                                        if len.wrapping_add(2 as libc::c_ulong) != tmp___23 {
                                                                                            current_block = 16889246570190342985;
                                                                                        } else {
                                                                                            return 0 as libc::c_int
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            _ => {
                                                                                current_block = 16889246570190342985;
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
                                                16889246570190342985 => {}
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
                    }
                    0 => {
                        current_block = 2930800982520623112;
                        match current_block {
                            5643496190901308087 => return 0 as libc::c_int,
                            13093596192184245794 => {
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
                                        .octet[0 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    ip
                                        .octet[1 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    ip
                                        .octet[2 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    ip
                                        .octet[3 as libc::c_int
                                        as usize] = 1 as libc::c_int as libc::c_uchar;
                                }
                                memcpy(
                                    &mut *buff.as_mut_ptr().offset(4 as libc::c_int as isize)
                                        as *mut libc::c_uchar as *mut libc::c_void,
                                    &mut ip as *mut ip_type as *const libc::c_void,
                                    4 as libc::c_int as size_t,
                                );
                                len = ulen.wrapping_add(1 as libc::c_ulong);
                                if len > 1 as libc::c_ulong {
                                    memcpy(
                                        &mut *buff.as_mut_ptr().offset(8 as libc::c_int as isize)
                                            as *mut libc::c_uchar as *mut libc::c_void,
                                        user as *const libc::c_void,
                                        len,
                                    );
                                } else {
                                    buff[8 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                }
                                if dns_len != 0 {
                                    memcpy(
                                        &mut *buff
                                            .as_mut_ptr()
                                            .offset((8 as libc::c_ulong).wrapping_add(len) as isize)
                                            as *mut libc::c_uchar as *mut libc::c_void,
                                        dns_name as *const libc::c_void,
                                        dns_len.wrapping_add(1 as libc::c_ulong),
                                    );
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(dns_len.wrapping_add(1 as libc::c_ulong))
                                        as size_t as size_t;
                                }
                                tmp___4 = write_n_bytes(
                                    sock,
                                    buff.as_mut_ptr() as *mut libc::c_char,
                                    (8 as libc::c_ulong).wrapping_add(len),
                                );
                                if !(len.wrapping_add(8 as libc::c_ulong) != tmp___4) {
                                    tmp___5 = read_n_bytes(
                                        sock,
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        8 as libc::c_int as size_t,
                                    );
                                    if !(8 as libc::c_ulong != tmp___5) {
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
                            2930800982520623112 => {
                                if dns_len == 0 {
                                    inet_ntop(
                                        2 as libc::c_int,
                                        &mut *(ip.octet)
                                            .as_mut_ptr()
                                            .offset(0 as libc::c_int as isize) as *mut libc::c_uchar
                                            as *const libc::c_void,
                                        ip_buf.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                            as socklen_t,
                                    );
                                    dns_name = ip_buf.as_mut_ptr();
                                }
                                tmp___1 = __bswap_16(port);
                                snprintf(
                                    buff.as_mut_ptr() as *mut libc::c_char,
                                    ::std::mem::size_of::<[libc::c_uchar; 8192]>()
                                        as libc::c_ulong,
                                    b"CONNECT %s:%d HTTP/1.0\r\n\0" as *const u8
                                        as *const libc::c_char,
                                    dns_name,
                                    tmp___1 as libc::c_int,
                                );
                                if *user.offset(0 as libc::c_int as isize) != 0 {
                                    memcpy(
                                        src.as_mut_ptr() as *mut libc::c_void,
                                        user as *const libc::c_void,
                                        ulen,
                                    );
                                    memcpy(
                                        src.as_mut_ptr().offset(ulen as isize) as *mut libc::c_void,
                                        b":\0" as *const u8 as *const libc::c_char
                                            as *const libc::c_void,
                                        1 as libc::c_int as size_t,
                                    );
                                    memcpy(
                                        src
                                            .as_mut_ptr()
                                            .offset(ulen as isize)
                                            .offset(1 as libc::c_int as isize) as *mut libc::c_void,
                                        pass as *const libc::c_void,
                                        passlen,
                                    );
                                    src[ulen
                                        .wrapping_add(1 as libc::c_ulong)
                                        .wrapping_add(passlen)
                                        as usize] = 0 as libc::c_int as libc::c_char;
                                    encode_base_64(
                                        src.as_mut_ptr(),
                                        dst.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 2048]>()
                                            as libc::c_ulong as libc::c_int,
                                    );
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        b"Proxy-Authorization: Basic \0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        dst.as_mut_ptr() as *const libc::c_char,
                                    );
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        b"\r\n\r\n\0" as *const u8 as *const libc::c_char,
                                    );
                                } else {
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        b"\r\n\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                len = strlen(
                                    buff.as_mut_ptr() as *mut libc::c_char
                                        as *const libc::c_char,
                                );
                                tmp___2 = send(
                                    sock,
                                    buff.as_mut_ptr() as *const libc::c_void,
                                    len,
                                    0 as libc::c_int,
                                );
                                if !(len != tmp___2 as size_t) {
                                    len = 0 as libc::c_int as size_t;
                                    loop {
                                        if !(len < 8192 as libc::c_ulong) {
                                            current_block = 15594839951440953787;
                                            break;
                                        }
                                        tmp___3 = read_n_bytes(
                                            sock,
                                            buff.as_mut_ptr().offset(len as isize) as *mut libc::c_char,
                                            1 as libc::c_int as size_t,
                                        );
                                        if !(1 as libc::c_ulong == tmp___3) {
                                            current_block = 16889246570190342985;
                                            break;
                                        }
                                        len = len.wrapping_add(1);
                                        if !(len > 4 as libc::c_ulong) {
                                            continue;
                                        }
                                        if !(buff[len.wrapping_sub(1 as libc::c_ulong) as usize]
                                            as libc::c_int == 10 as libc::c_int)
                                        {
                                            continue;
                                        }
                                        if !(buff[len.wrapping_sub(2 as libc::c_ulong) as usize]
                                            as libc::c_int == 13 as libc::c_int)
                                        {
                                            continue;
                                        }
                                        if !(buff[len.wrapping_sub(3 as libc::c_ulong) as usize]
                                            as libc::c_int == 10 as libc::c_int)
                                        {
                                            continue;
                                        }
                                        if buff[len.wrapping_sub(4 as libc::c_ulong) as usize]
                                            as libc::c_int == 13 as libc::c_int
                                        {
                                            current_block = 15594839951440953787;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        16889246570190342985 => {}
                                        _ => {
                                            if len == 8192 as libc::c_ulong {
                                                return 5 as libc::c_int
                                            } else {
                                                if buff[9 as libc::c_int as usize] as libc::c_int
                                                    == 50 as libc::c_int
                                                {
                                                    if buff[10 as libc::c_int as usize] as libc::c_int
                                                        == 48 as libc::c_int
                                                    {
                                                        if !(buff[11 as libc::c_int as usize] as libc::c_int
                                                            == 48 as libc::c_int)
                                                        {
                                                            return 5 as libc::c_int;
                                                        }
                                                    } else {
                                                        return 5 as libc::c_int
                                                    }
                                                } else {
                                                    return 5 as libc::c_int
                                                }
                                            }
                                            return 0 as libc::c_int;
                                        }
                                    }
                                }
                            }
                            _ => {
                                if !user.is_null() {
                                    buff[0 as libc::c_int
                                        as usize] = 5 as libc::c_int as libc::c_uchar;
                                    buff[1 as libc::c_int
                                        as usize] = 2 as libc::c_int as libc::c_uchar;
                                    buff[2 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    buff[3 as libc::c_int
                                        as usize] = 2 as libc::c_int as libc::c_uchar;
                                    tmp___6 = write_n_bytes(
                                        sock,
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        4 as libc::c_int as size_t,
                                    );
                                    if 4 as libc::c_ulong != tmp___6 {
                                        current_block = 16889246570190342985;
                                    } else {
                                        current_block = 13201766686570145889;
                                    }
                                } else {
                                    buff[0 as libc::c_int
                                        as usize] = 5 as libc::c_int as libc::c_uchar;
                                    buff[1 as libc::c_int
                                        as usize] = 1 as libc::c_int as libc::c_uchar;
                                    buff[2 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    tmp___7 = write_n_bytes(
                                        sock,
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        3 as libc::c_int as size_t,
                                    );
                                    if 3 as libc::c_ulong != tmp___7 {
                                        current_block = 16889246570190342985;
                                    } else {
                                        current_block = 13201766686570145889;
                                    }
                                }
                                match current_block {
                                    16889246570190342985 => {}
                                    _ => {
                                        tmp___8 = read_n_bytes(
                                            sock,
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            2 as libc::c_int as size_t,
                                        );
                                        if !(2 as libc::c_ulong != tmp___8) {
                                            if buff[0 as libc::c_int as usize] as libc::c_int
                                                != 5 as libc::c_int
                                            {
                                                current_block = 6887742563588325581;
                                            } else {
                                                if buff[1 as libc::c_int as usize] as libc::c_int
                                                    != 0 as libc::c_int
                                                {
                                                    if buff[1 as libc::c_int as usize] as libc::c_int
                                                        != 2 as libc::c_int
                                                    {
                                                        current_block = 6887742563588325581;
                                                    } else {
                                                        current_block = 16314074004867283505;
                                                    }
                                                } else {
                                                    current_block = 16314074004867283505;
                                                }
                                                match current_block {
                                                    6887742563588325581 => {}
                                                    _ => {
                                                        if buff[1 as libc::c_int as usize] as libc::c_int
                                                            == 2 as libc::c_int
                                                        {
                                                            cur = out.as_mut_ptr();
                                                            tmp___9 = cur;
                                                            cur = cur.offset(1);
                                                            *tmp___9 = 1 as libc::c_int as libc::c_char;
                                                            c = (ulen & 255 as libc::c_ulong) as libc::c_int;
                                                            tmp___10 = cur;
                                                            cur = cur.offset(1);
                                                            *tmp___10 = c as libc::c_char;
                                                            memcpy(
                                                                cur as *mut libc::c_void,
                                                                user as *const libc::c_void,
                                                                c as size_t,
                                                            );
                                                            cur = cur.offset(c as isize);
                                                            c = (passlen & 255 as libc::c_ulong) as libc::c_int;
                                                            tmp___11 = cur;
                                                            cur = cur.offset(1);
                                                            *tmp___11 = c as libc::c_char;
                                                            memcpy(
                                                                cur as *mut libc::c_void,
                                                                pass as *const libc::c_void,
                                                                c as size_t,
                                                            );
                                                            cur = cur.offset(c as isize);
                                                            tmp___12 = write_n_bytes(
                                                                sock,
                                                                out.as_mut_ptr(),
                                                                cur.offset_from(out.as_mut_ptr()) as libc::c_long as size_t,
                                                            );
                                                            if cur.offset_from(out.as_mut_ptr()) as libc::c_long
                                                                as size_t != tmp___12
                                                            {
                                                                current_block = 16889246570190342985;
                                                            } else {
                                                                tmp___13 = read_n_bytes(
                                                                    sock,
                                                                    in_0.as_mut_ptr(),
                                                                    2 as libc::c_int as size_t,
                                                                );
                                                                if 2 as libc::c_ulong != tmp___13 {
                                                                    current_block = 16889246570190342985;
                                                                } else {
                                                                    if in_0[0 as libc::c_int as usize] as libc::c_int
                                                                        != 1 as libc::c_int
                                                                    {
                                                                        current_block = 7368271564846522885;
                                                                    } else if in_0[1 as libc::c_int as usize] as libc::c_int
                                                                            != 0 as libc::c_int
                                                                        {
                                                                        current_block = 7368271564846522885;
                                                                    } else {
                                                                        current_block = 14883390358315838856;
                                                                    }
                                                                    match current_block {
                                                                        14883390358315838856 => {}
                                                                        _ => {
                                                                            if in_0[0 as libc::c_int as usize] as libc::c_int
                                                                                != 1 as libc::c_int
                                                                            {
                                                                                current_block = 16889246570190342985;
                                                                            } else {
                                                                                return 5 as libc::c_int
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            current_block = 14883390358315838856;
                                                        }
                                                        match current_block {
                                                            16889246570190342985 => {}
                                                            _ => {
                                                                buff_iter = 0 as libc::c_int as size_t;
                                                                tmp___14 = buff_iter;
                                                                buff_iter = buff_iter.wrapping_add(1);
                                                                buff[tmp___14 as usize] = 5 as libc::c_int as libc::c_uchar;
                                                                tmp___15 = buff_iter;
                                                                buff_iter = buff_iter.wrapping_add(1);
                                                                buff[tmp___15 as usize] = 1 as libc::c_int as libc::c_uchar;
                                                                tmp___16 = buff_iter;
                                                                buff_iter = buff_iter.wrapping_add(1);
                                                                buff[tmp___16 as usize] = 0 as libc::c_int as libc::c_uchar;
                                                                if dns_len == 0 {
                                                                    tmp___17 = buff_iter;
                                                                    buff_iter = buff_iter.wrapping_add(1);
                                                                    buff[tmp___17 as usize] = 1 as libc::c_int as libc::c_uchar;
                                                                    memcpy(
                                                                        buff.as_mut_ptr().offset(buff_iter as isize)
                                                                            as *mut libc::c_void,
                                                                        &mut ip as *mut ip_type as *const libc::c_void,
                                                                        4 as libc::c_int as size_t,
                                                                    );
                                                                    buff_iter = (buff_iter as libc::c_ulong)
                                                                        .wrapping_add(4 as libc::c_ulong) as size_t as size_t;
                                                                } else {
                                                                    tmp___18 = buff_iter;
                                                                    buff_iter = buff_iter.wrapping_add(1);
                                                                    buff[tmp___18 as usize] = 3 as libc::c_int as libc::c_uchar;
                                                                    tmp___19 = buff_iter;
                                                                    buff_iter = buff_iter.wrapping_add(1);
                                                                    buff[tmp___19
                                                                        as usize] = (dns_len & 255 as libc::c_ulong)
                                                                        as libc::c_uchar;
                                                                    memcpy(
                                                                        buff.as_mut_ptr().offset(buff_iter as isize)
                                                                            as *mut libc::c_void,
                                                                        dns_name as *const libc::c_void,
                                                                        dns_len,
                                                                    );
                                                                    buff_iter = (buff_iter as libc::c_ulong)
                                                                        .wrapping_add(dns_len) as size_t as size_t;
                                                                }
                                                                memcpy(
                                                                    buff.as_mut_ptr().offset(buff_iter as isize)
                                                                        as *mut libc::c_void,
                                                                    &mut port as *mut libc::c_ushort as *const libc::c_void,
                                                                    2 as libc::c_int as size_t,
                                                                );
                                                                buff_iter = (buff_iter as libc::c_ulong)
                                                                    .wrapping_add(2 as libc::c_ulong) as size_t as size_t;
                                                                tmp___20 = write_n_bytes(
                                                                    sock,
                                                                    buff.as_mut_ptr() as *mut libc::c_char,
                                                                    buff_iter,
                                                                );
                                                                if buff_iter != tmp___20 {
                                                                    current_block = 16889246570190342985;
                                                                } else {
                                                                    tmp___21 = read_n_bytes(
                                                                        sock,
                                                                        buff.as_mut_ptr() as *mut libc::c_char,
                                                                        4 as libc::c_int as size_t,
                                                                    );
                                                                    if 4 as libc::c_ulong != tmp___21 {
                                                                        current_block = 16889246570190342985;
                                                                    } else if buff[0 as libc::c_int as usize] as libc::c_int
                                                                            != 5 as libc::c_int
                                                                        {
                                                                        current_block = 16889246570190342985;
                                                                    } else if buff[1 as libc::c_int as usize] as libc::c_int
                                                                            != 0 as libc::c_int
                                                                        {
                                                                        current_block = 16889246570190342985;
                                                                    } else {
                                                                        match buff[3 as libc::c_int as usize] as libc::c_int {
                                                                            1 => {
                                                                                current_block = 9157488150074734138;
                                                                                match current_block {
                                                                                    14120677076171701346 => {
                                                                                        len = 16 as libc::c_int as size_t;
                                                                                        current_block = 9879293939512133550;
                                                                                    }
                                                                                    9157488150074734138 => {
                                                                                        len = 4 as libc::c_int as size_t;
                                                                                        current_block = 9879293939512133550;
                                                                                    }
                                                                                    _ => {
                                                                                        len = 0 as libc::c_int as size_t;
                                                                                        tmp___22 = read_n_bytes(
                                                                                            sock,
                                                                                            &mut len as *mut size_t as *mut libc::c_char,
                                                                                            1 as libc::c_int as size_t,
                                                                                        );
                                                                                        if 1 as libc::c_ulong != tmp___22 {
                                                                                            current_block = 16889246570190342985;
                                                                                        } else {
                                                                                            current_block = 9879293939512133550;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                match current_block {
                                                                                    16889246570190342985 => {}
                                                                                    _ => {
                                                                                        tmp___23 = read_n_bytes(
                                                                                            sock,
                                                                                            buff.as_mut_ptr() as *mut libc::c_char,
                                                                                            len.wrapping_add(2 as libc::c_ulong),
                                                                                        );
                                                                                        if len.wrapping_add(2 as libc::c_ulong) != tmp___23 {
                                                                                            current_block = 16889246570190342985;
                                                                                        } else {
                                                                                            return 0 as libc::c_int
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            4 => {
                                                                                current_block = 14120677076171701346;
                                                                                match current_block {
                                                                                    14120677076171701346 => {
                                                                                        len = 16 as libc::c_int as size_t;
                                                                                        current_block = 9879293939512133550;
                                                                                    }
                                                                                    9157488150074734138 => {
                                                                                        len = 4 as libc::c_int as size_t;
                                                                                        current_block = 9879293939512133550;
                                                                                    }
                                                                                    _ => {
                                                                                        len = 0 as libc::c_int as size_t;
                                                                                        tmp___22 = read_n_bytes(
                                                                                            sock,
                                                                                            &mut len as *mut size_t as *mut libc::c_char,
                                                                                            1 as libc::c_int as size_t,
                                                                                        );
                                                                                        if 1 as libc::c_ulong != tmp___22 {
                                                                                            current_block = 16889246570190342985;
                                                                                        } else {
                                                                                            current_block = 9879293939512133550;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                match current_block {
                                                                                    16889246570190342985 => {}
                                                                                    _ => {
                                                                                        tmp___23 = read_n_bytes(
                                                                                            sock,
                                                                                            buff.as_mut_ptr() as *mut libc::c_char,
                                                                                            len.wrapping_add(2 as libc::c_ulong),
                                                                                        );
                                                                                        if len.wrapping_add(2 as libc::c_ulong) != tmp___23 {
                                                                                            current_block = 16889246570190342985;
                                                                                        } else {
                                                                                            return 0 as libc::c_int
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            3 => {
                                                                                current_block = 2261659062341822993;
                                                                                match current_block {
                                                                                    14120677076171701346 => {
                                                                                        len = 16 as libc::c_int as size_t;
                                                                                        current_block = 9879293939512133550;
                                                                                    }
                                                                                    9157488150074734138 => {
                                                                                        len = 4 as libc::c_int as size_t;
                                                                                        current_block = 9879293939512133550;
                                                                                    }
                                                                                    _ => {
                                                                                        len = 0 as libc::c_int as size_t;
                                                                                        tmp___22 = read_n_bytes(
                                                                                            sock,
                                                                                            &mut len as *mut size_t as *mut libc::c_char,
                                                                                            1 as libc::c_int as size_t,
                                                                                        );
                                                                                        if 1 as libc::c_ulong != tmp___22 {
                                                                                            current_block = 16889246570190342985;
                                                                                        } else {
                                                                                            current_block = 9879293939512133550;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                match current_block {
                                                                                    16889246570190342985 => {}
                                                                                    _ => {
                                                                                        tmp___23 = read_n_bytes(
                                                                                            sock,
                                                                                            buff.as_mut_ptr() as *mut libc::c_char,
                                                                                            len.wrapping_add(2 as libc::c_ulong),
                                                                                        );
                                                                                        if len.wrapping_add(2 as libc::c_ulong) != tmp___23 {
                                                                                            current_block = 16889246570190342985;
                                                                                        } else {
                                                                                            return 0 as libc::c_int
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            _ => {
                                                                                current_block = 16889246570190342985;
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
                                                16889246570190342985 => {}
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
                    }
                    2 => {
                        current_block = 13093596192184245794;
                        match current_block {
                            5643496190901308087 => return 0 as libc::c_int,
                            13093596192184245794 => {
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
                                        .octet[0 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    ip
                                        .octet[1 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    ip
                                        .octet[2 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    ip
                                        .octet[3 as libc::c_int
                                        as usize] = 1 as libc::c_int as libc::c_uchar;
                                }
                                memcpy(
                                    &mut *buff.as_mut_ptr().offset(4 as libc::c_int as isize)
                                        as *mut libc::c_uchar as *mut libc::c_void,
                                    &mut ip as *mut ip_type as *const libc::c_void,
                                    4 as libc::c_int as size_t,
                                );
                                len = ulen.wrapping_add(1 as libc::c_ulong);
                                if len > 1 as libc::c_ulong {
                                    memcpy(
                                        &mut *buff.as_mut_ptr().offset(8 as libc::c_int as isize)
                                            as *mut libc::c_uchar as *mut libc::c_void,
                                        user as *const libc::c_void,
                                        len,
                                    );
                                } else {
                                    buff[8 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                }
                                if dns_len != 0 {
                                    memcpy(
                                        &mut *buff
                                            .as_mut_ptr()
                                            .offset((8 as libc::c_ulong).wrapping_add(len) as isize)
                                            as *mut libc::c_uchar as *mut libc::c_void,
                                        dns_name as *const libc::c_void,
                                        dns_len.wrapping_add(1 as libc::c_ulong),
                                    );
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(dns_len.wrapping_add(1 as libc::c_ulong))
                                        as size_t as size_t;
                                }
                                tmp___4 = write_n_bytes(
                                    sock,
                                    buff.as_mut_ptr() as *mut libc::c_char,
                                    (8 as libc::c_ulong).wrapping_add(len),
                                );
                                if !(len.wrapping_add(8 as libc::c_ulong) != tmp___4) {
                                    tmp___5 = read_n_bytes(
                                        sock,
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        8 as libc::c_int as size_t,
                                    );
                                    if !(8 as libc::c_ulong != tmp___5) {
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
                            2930800982520623112 => {
                                if dns_len == 0 {
                                    inet_ntop(
                                        2 as libc::c_int,
                                        &mut *(ip.octet)
                                            .as_mut_ptr()
                                            .offset(0 as libc::c_int as isize) as *mut libc::c_uchar
                                            as *const libc::c_void,
                                        ip_buf.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                            as socklen_t,
                                    );
                                    dns_name = ip_buf.as_mut_ptr();
                                }
                                tmp___1 = __bswap_16(port);
                                snprintf(
                                    buff.as_mut_ptr() as *mut libc::c_char,
                                    ::std::mem::size_of::<[libc::c_uchar; 8192]>()
                                        as libc::c_ulong,
                                    b"CONNECT %s:%d HTTP/1.0\r\n\0" as *const u8
                                        as *const libc::c_char,
                                    dns_name,
                                    tmp___1 as libc::c_int,
                                );
                                if *user.offset(0 as libc::c_int as isize) != 0 {
                                    memcpy(
                                        src.as_mut_ptr() as *mut libc::c_void,
                                        user as *const libc::c_void,
                                        ulen,
                                    );
                                    memcpy(
                                        src.as_mut_ptr().offset(ulen as isize) as *mut libc::c_void,
                                        b":\0" as *const u8 as *const libc::c_char
                                            as *const libc::c_void,
                                        1 as libc::c_int as size_t,
                                    );
                                    memcpy(
                                        src
                                            .as_mut_ptr()
                                            .offset(ulen as isize)
                                            .offset(1 as libc::c_int as isize) as *mut libc::c_void,
                                        pass as *const libc::c_void,
                                        passlen,
                                    );
                                    src[ulen
                                        .wrapping_add(1 as libc::c_ulong)
                                        .wrapping_add(passlen)
                                        as usize] = 0 as libc::c_int as libc::c_char;
                                    encode_base_64(
                                        src.as_mut_ptr(),
                                        dst.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 2048]>()
                                            as libc::c_ulong as libc::c_int,
                                    );
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        b"Proxy-Authorization: Basic \0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        dst.as_mut_ptr() as *const libc::c_char,
                                    );
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        b"\r\n\r\n\0" as *const u8 as *const libc::c_char,
                                    );
                                } else {
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        b"\r\n\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                len = strlen(
                                    buff.as_mut_ptr() as *mut libc::c_char
                                        as *const libc::c_char,
                                );
                                tmp___2 = send(
                                    sock,
                                    buff.as_mut_ptr() as *const libc::c_void,
                                    len,
                                    0 as libc::c_int,
                                );
                                if !(len != tmp___2 as size_t) {
                                    len = 0 as libc::c_int as size_t;
                                    loop {
                                        if !(len < 8192 as libc::c_ulong) {
                                            current_block = 15594839951440953787;
                                            break;
                                        }
                                        tmp___3 = read_n_bytes(
                                            sock,
                                            buff.as_mut_ptr().offset(len as isize) as *mut libc::c_char,
                                            1 as libc::c_int as size_t,
                                        );
                                        if !(1 as libc::c_ulong == tmp___3) {
                                            current_block = 16889246570190342985;
                                            break;
                                        }
                                        len = len.wrapping_add(1);
                                        if !(len > 4 as libc::c_ulong) {
                                            continue;
                                        }
                                        if !(buff[len.wrapping_sub(1 as libc::c_ulong) as usize]
                                            as libc::c_int == 10 as libc::c_int)
                                        {
                                            continue;
                                        }
                                        if !(buff[len.wrapping_sub(2 as libc::c_ulong) as usize]
                                            as libc::c_int == 13 as libc::c_int)
                                        {
                                            continue;
                                        }
                                        if !(buff[len.wrapping_sub(3 as libc::c_ulong) as usize]
                                            as libc::c_int == 10 as libc::c_int)
                                        {
                                            continue;
                                        }
                                        if buff[len.wrapping_sub(4 as libc::c_ulong) as usize]
                                            as libc::c_int == 13 as libc::c_int
                                        {
                                            current_block = 15594839951440953787;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        16889246570190342985 => {}
                                        _ => {
                                            if len == 8192 as libc::c_ulong {
                                                return 5 as libc::c_int
                                            } else {
                                                if buff[9 as libc::c_int as usize] as libc::c_int
                                                    == 50 as libc::c_int
                                                {
                                                    if buff[10 as libc::c_int as usize] as libc::c_int
                                                        == 48 as libc::c_int
                                                    {
                                                        if !(buff[11 as libc::c_int as usize] as libc::c_int
                                                            == 48 as libc::c_int)
                                                        {
                                                            return 5 as libc::c_int;
                                                        }
                                                    } else {
                                                        return 5 as libc::c_int
                                                    }
                                                } else {
                                                    return 5 as libc::c_int
                                                }
                                            }
                                            return 0 as libc::c_int;
                                        }
                                    }
                                }
                            }
                            _ => {
                                if !user.is_null() {
                                    buff[0 as libc::c_int
                                        as usize] = 5 as libc::c_int as libc::c_uchar;
                                    buff[1 as libc::c_int
                                        as usize] = 2 as libc::c_int as libc::c_uchar;
                                    buff[2 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    buff[3 as libc::c_int
                                        as usize] = 2 as libc::c_int as libc::c_uchar;
                                    tmp___6 = write_n_bytes(
                                        sock,
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        4 as libc::c_int as size_t,
                                    );
                                    if 4 as libc::c_ulong != tmp___6 {
                                        current_block = 16889246570190342985;
                                    } else {
                                        current_block = 13201766686570145889;
                                    }
                                } else {
                                    buff[0 as libc::c_int
                                        as usize] = 5 as libc::c_int as libc::c_uchar;
                                    buff[1 as libc::c_int
                                        as usize] = 1 as libc::c_int as libc::c_uchar;
                                    buff[2 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    tmp___7 = write_n_bytes(
                                        sock,
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        3 as libc::c_int as size_t,
                                    );
                                    if 3 as libc::c_ulong != tmp___7 {
                                        current_block = 16889246570190342985;
                                    } else {
                                        current_block = 13201766686570145889;
                                    }
                                }
                                match current_block {
                                    16889246570190342985 => {}
                                    _ => {
                                        tmp___8 = read_n_bytes(
                                            sock,
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            2 as libc::c_int as size_t,
                                        );
                                        if !(2 as libc::c_ulong != tmp___8) {
                                            if buff[0 as libc::c_int as usize] as libc::c_int
                                                != 5 as libc::c_int
                                            {
                                                current_block = 6887742563588325581;
                                            } else {
                                                if buff[1 as libc::c_int as usize] as libc::c_int
                                                    != 0 as libc::c_int
                                                {
                                                    if buff[1 as libc::c_int as usize] as libc::c_int
                                                        != 2 as libc::c_int
                                                    {
                                                        current_block = 6887742563588325581;
                                                    } else {
                                                        current_block = 16314074004867283505;
                                                    }
                                                } else {
                                                    current_block = 16314074004867283505;
                                                }
                                                match current_block {
                                                    6887742563588325581 => {}
                                                    _ => {
                                                        if buff[1 as libc::c_int as usize] as libc::c_int
                                                            == 2 as libc::c_int
                                                        {
                                                            cur = out.as_mut_ptr();
                                                            tmp___9 = cur;
                                                            cur = cur.offset(1);
                                                            *tmp___9 = 1 as libc::c_int as libc::c_char;
                                                            c = (ulen & 255 as libc::c_ulong) as libc::c_int;
                                                            tmp___10 = cur;
                                                            cur = cur.offset(1);
                                                            *tmp___10 = c as libc::c_char;
                                                            memcpy(
                                                                cur as *mut libc::c_void,
                                                                user as *const libc::c_void,
                                                                c as size_t,
                                                            );
                                                            cur = cur.offset(c as isize);
                                                            c = (passlen & 255 as libc::c_ulong) as libc::c_int;
                                                            tmp___11 = cur;
                                                            cur = cur.offset(1);
                                                            *tmp___11 = c as libc::c_char;
                                                            memcpy(
                                                                cur as *mut libc::c_void,
                                                                pass as *const libc::c_void,
                                                                c as size_t,
                                                            );
                                                            cur = cur.offset(c as isize);
                                                            tmp___12 = write_n_bytes(
                                                                sock,
                                                                out.as_mut_ptr(),
                                                                cur.offset_from(out.as_mut_ptr()) as libc::c_long as size_t,
                                                            );
                                                            if cur.offset_from(out.as_mut_ptr()) as libc::c_long
                                                                as size_t != tmp___12
                                                            {
                                                                current_block = 16889246570190342985;
                                                            } else {
                                                                tmp___13 = read_n_bytes(
                                                                    sock,
                                                                    in_0.as_mut_ptr(),
                                                                    2 as libc::c_int as size_t,
                                                                );
                                                                if 2 as libc::c_ulong != tmp___13 {
                                                                    current_block = 16889246570190342985;
                                                                } else {
                                                                    if in_0[0 as libc::c_int as usize] as libc::c_int
                                                                        != 1 as libc::c_int
                                                                    {
                                                                        current_block = 7368271564846522885;
                                                                    } else if in_0[1 as libc::c_int as usize] as libc::c_int
                                                                            != 0 as libc::c_int
                                                                        {
                                                                        current_block = 7368271564846522885;
                                                                    } else {
                                                                        current_block = 14883390358315838856;
                                                                    }
                                                                    match current_block {
                                                                        14883390358315838856 => {}
                                                                        _ => {
                                                                            if in_0[0 as libc::c_int as usize] as libc::c_int
                                                                                != 1 as libc::c_int
                                                                            {
                                                                                current_block = 16889246570190342985;
                                                                            } else {
                                                                                return 5 as libc::c_int
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            current_block = 14883390358315838856;
                                                        }
                                                        match current_block {
                                                            16889246570190342985 => {}
                                                            _ => {
                                                                buff_iter = 0 as libc::c_int as size_t;
                                                                tmp___14 = buff_iter;
                                                                buff_iter = buff_iter.wrapping_add(1);
                                                                buff[tmp___14 as usize] = 5 as libc::c_int as libc::c_uchar;
                                                                tmp___15 = buff_iter;
                                                                buff_iter = buff_iter.wrapping_add(1);
                                                                buff[tmp___15 as usize] = 1 as libc::c_int as libc::c_uchar;
                                                                tmp___16 = buff_iter;
                                                                buff_iter = buff_iter.wrapping_add(1);
                                                                buff[tmp___16 as usize] = 0 as libc::c_int as libc::c_uchar;
                                                                if dns_len == 0 {
                                                                    tmp___17 = buff_iter;
                                                                    buff_iter = buff_iter.wrapping_add(1);
                                                                    buff[tmp___17 as usize] = 1 as libc::c_int as libc::c_uchar;
                                                                    memcpy(
                                                                        buff.as_mut_ptr().offset(buff_iter as isize)
                                                                            as *mut libc::c_void,
                                                                        &mut ip as *mut ip_type as *const libc::c_void,
                                                                        4 as libc::c_int as size_t,
                                                                    );
                                                                    buff_iter = (buff_iter as libc::c_ulong)
                                                                        .wrapping_add(4 as libc::c_ulong) as size_t as size_t;
                                                                } else {
                                                                    tmp___18 = buff_iter;
                                                                    buff_iter = buff_iter.wrapping_add(1);
                                                                    buff[tmp___18 as usize] = 3 as libc::c_int as libc::c_uchar;
                                                                    tmp___19 = buff_iter;
                                                                    buff_iter = buff_iter.wrapping_add(1);
                                                                    buff[tmp___19
                                                                        as usize] = (dns_len & 255 as libc::c_ulong)
                                                                        as libc::c_uchar;
                                                                    memcpy(
                                                                        buff.as_mut_ptr().offset(buff_iter as isize)
                                                                            as *mut libc::c_void,
                                                                        dns_name as *const libc::c_void,
                                                                        dns_len,
                                                                    );
                                                                    buff_iter = (buff_iter as libc::c_ulong)
                                                                        .wrapping_add(dns_len) as size_t as size_t;
                                                                }
                                                                memcpy(
                                                                    buff.as_mut_ptr().offset(buff_iter as isize)
                                                                        as *mut libc::c_void,
                                                                    &mut port as *mut libc::c_ushort as *const libc::c_void,
                                                                    2 as libc::c_int as size_t,
                                                                );
                                                                buff_iter = (buff_iter as libc::c_ulong)
                                                                    .wrapping_add(2 as libc::c_ulong) as size_t as size_t;
                                                                tmp___20 = write_n_bytes(
                                                                    sock,
                                                                    buff.as_mut_ptr() as *mut libc::c_char,
                                                                    buff_iter,
                                                                );
                                                                if buff_iter != tmp___20 {
                                                                    current_block = 16889246570190342985;
                                                                } else {
                                                                    tmp___21 = read_n_bytes(
                                                                        sock,
                                                                        buff.as_mut_ptr() as *mut libc::c_char,
                                                                        4 as libc::c_int as size_t,
                                                                    );
                                                                    if 4 as libc::c_ulong != tmp___21 {
                                                                        current_block = 16889246570190342985;
                                                                    } else if buff[0 as libc::c_int as usize] as libc::c_int
                                                                            != 5 as libc::c_int
                                                                        {
                                                                        current_block = 16889246570190342985;
                                                                    } else if buff[1 as libc::c_int as usize] as libc::c_int
                                                                            != 0 as libc::c_int
                                                                        {
                                                                        current_block = 16889246570190342985;
                                                                    } else {
                                                                        match buff[3 as libc::c_int as usize] as libc::c_int {
                                                                            1 => {
                                                                                current_block = 9157488150074734138;
                                                                                match current_block {
                                                                                    14120677076171701346 => {
                                                                                        len = 16 as libc::c_int as size_t;
                                                                                        current_block = 9879293939512133550;
                                                                                    }
                                                                                    9157488150074734138 => {
                                                                                        len = 4 as libc::c_int as size_t;
                                                                                        current_block = 9879293939512133550;
                                                                                    }
                                                                                    _ => {
                                                                                        len = 0 as libc::c_int as size_t;
                                                                                        tmp___22 = read_n_bytes(
                                                                                            sock,
                                                                                            &mut len as *mut size_t as *mut libc::c_char,
                                                                                            1 as libc::c_int as size_t,
                                                                                        );
                                                                                        if 1 as libc::c_ulong != tmp___22 {
                                                                                            current_block = 16889246570190342985;
                                                                                        } else {
                                                                                            current_block = 9879293939512133550;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                match current_block {
                                                                                    16889246570190342985 => {}
                                                                                    _ => {
                                                                                        tmp___23 = read_n_bytes(
                                                                                            sock,
                                                                                            buff.as_mut_ptr() as *mut libc::c_char,
                                                                                            len.wrapping_add(2 as libc::c_ulong),
                                                                                        );
                                                                                        if len.wrapping_add(2 as libc::c_ulong) != tmp___23 {
                                                                                            current_block = 16889246570190342985;
                                                                                        } else {
                                                                                            return 0 as libc::c_int
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            4 => {
                                                                                current_block = 14120677076171701346;
                                                                                match current_block {
                                                                                    14120677076171701346 => {
                                                                                        len = 16 as libc::c_int as size_t;
                                                                                        current_block = 9879293939512133550;
                                                                                    }
                                                                                    9157488150074734138 => {
                                                                                        len = 4 as libc::c_int as size_t;
                                                                                        current_block = 9879293939512133550;
                                                                                    }
                                                                                    _ => {
                                                                                        len = 0 as libc::c_int as size_t;
                                                                                        tmp___22 = read_n_bytes(
                                                                                            sock,
                                                                                            &mut len as *mut size_t as *mut libc::c_char,
                                                                                            1 as libc::c_int as size_t,
                                                                                        );
                                                                                        if 1 as libc::c_ulong != tmp___22 {
                                                                                            current_block = 16889246570190342985;
                                                                                        } else {
                                                                                            current_block = 9879293939512133550;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                match current_block {
                                                                                    16889246570190342985 => {}
                                                                                    _ => {
                                                                                        tmp___23 = read_n_bytes(
                                                                                            sock,
                                                                                            buff.as_mut_ptr() as *mut libc::c_char,
                                                                                            len.wrapping_add(2 as libc::c_ulong),
                                                                                        );
                                                                                        if len.wrapping_add(2 as libc::c_ulong) != tmp___23 {
                                                                                            current_block = 16889246570190342985;
                                                                                        } else {
                                                                                            return 0 as libc::c_int
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            3 => {
                                                                                current_block = 2261659062341822993;
                                                                                match current_block {
                                                                                    14120677076171701346 => {
                                                                                        len = 16 as libc::c_int as size_t;
                                                                                        current_block = 9879293939512133550;
                                                                                    }
                                                                                    9157488150074734138 => {
                                                                                        len = 4 as libc::c_int as size_t;
                                                                                        current_block = 9879293939512133550;
                                                                                    }
                                                                                    _ => {
                                                                                        len = 0 as libc::c_int as size_t;
                                                                                        tmp___22 = read_n_bytes(
                                                                                            sock,
                                                                                            &mut len as *mut size_t as *mut libc::c_char,
                                                                                            1 as libc::c_int as size_t,
                                                                                        );
                                                                                        if 1 as libc::c_ulong != tmp___22 {
                                                                                            current_block = 16889246570190342985;
                                                                                        } else {
                                                                                            current_block = 9879293939512133550;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                match current_block {
                                                                                    16889246570190342985 => {}
                                                                                    _ => {
                                                                                        tmp___23 = read_n_bytes(
                                                                                            sock,
                                                                                            buff.as_mut_ptr() as *mut libc::c_char,
                                                                                            len.wrapping_add(2 as libc::c_ulong),
                                                                                        );
                                                                                        if len.wrapping_add(2 as libc::c_ulong) != tmp___23 {
                                                                                            current_block = 16889246570190342985;
                                                                                        } else {
                                                                                            return 0 as libc::c_int
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            _ => {
                                                                                current_block = 16889246570190342985;
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
                                                16889246570190342985 => {}
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
                    }
                    3 => {
                        current_block = 2991993021755221704;
                        match current_block {
                            5643496190901308087 => return 0 as libc::c_int,
                            13093596192184245794 => {
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
                                        .octet[0 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    ip
                                        .octet[1 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    ip
                                        .octet[2 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    ip
                                        .octet[3 as libc::c_int
                                        as usize] = 1 as libc::c_int as libc::c_uchar;
                                }
                                memcpy(
                                    &mut *buff.as_mut_ptr().offset(4 as libc::c_int as isize)
                                        as *mut libc::c_uchar as *mut libc::c_void,
                                    &mut ip as *mut ip_type as *const libc::c_void,
                                    4 as libc::c_int as size_t,
                                );
                                len = ulen.wrapping_add(1 as libc::c_ulong);
                                if len > 1 as libc::c_ulong {
                                    memcpy(
                                        &mut *buff.as_mut_ptr().offset(8 as libc::c_int as isize)
                                            as *mut libc::c_uchar as *mut libc::c_void,
                                        user as *const libc::c_void,
                                        len,
                                    );
                                } else {
                                    buff[8 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                }
                                if dns_len != 0 {
                                    memcpy(
                                        &mut *buff
                                            .as_mut_ptr()
                                            .offset((8 as libc::c_ulong).wrapping_add(len) as isize)
                                            as *mut libc::c_uchar as *mut libc::c_void,
                                        dns_name as *const libc::c_void,
                                        dns_len.wrapping_add(1 as libc::c_ulong),
                                    );
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(dns_len.wrapping_add(1 as libc::c_ulong))
                                        as size_t as size_t;
                                }
                                tmp___4 = write_n_bytes(
                                    sock,
                                    buff.as_mut_ptr() as *mut libc::c_char,
                                    (8 as libc::c_ulong).wrapping_add(len),
                                );
                                if !(len.wrapping_add(8 as libc::c_ulong) != tmp___4) {
                                    tmp___5 = read_n_bytes(
                                        sock,
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        8 as libc::c_int as size_t,
                                    );
                                    if !(8 as libc::c_ulong != tmp___5) {
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
                            2930800982520623112 => {
                                if dns_len == 0 {
                                    inet_ntop(
                                        2 as libc::c_int,
                                        &mut *(ip.octet)
                                            .as_mut_ptr()
                                            .offset(0 as libc::c_int as isize) as *mut libc::c_uchar
                                            as *const libc::c_void,
                                        ip_buf.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                            as socklen_t,
                                    );
                                    dns_name = ip_buf.as_mut_ptr();
                                }
                                tmp___1 = __bswap_16(port);
                                snprintf(
                                    buff.as_mut_ptr() as *mut libc::c_char,
                                    ::std::mem::size_of::<[libc::c_uchar; 8192]>()
                                        as libc::c_ulong,
                                    b"CONNECT %s:%d HTTP/1.0\r\n\0" as *const u8
                                        as *const libc::c_char,
                                    dns_name,
                                    tmp___1 as libc::c_int,
                                );
                                if *user.offset(0 as libc::c_int as isize) != 0 {
                                    memcpy(
                                        src.as_mut_ptr() as *mut libc::c_void,
                                        user as *const libc::c_void,
                                        ulen,
                                    );
                                    memcpy(
                                        src.as_mut_ptr().offset(ulen as isize) as *mut libc::c_void,
                                        b":\0" as *const u8 as *const libc::c_char
                                            as *const libc::c_void,
                                        1 as libc::c_int as size_t,
                                    );
                                    memcpy(
                                        src
                                            .as_mut_ptr()
                                            .offset(ulen as isize)
                                            .offset(1 as libc::c_int as isize) as *mut libc::c_void,
                                        pass as *const libc::c_void,
                                        passlen,
                                    );
                                    src[ulen
                                        .wrapping_add(1 as libc::c_ulong)
                                        .wrapping_add(passlen)
                                        as usize] = 0 as libc::c_int as libc::c_char;
                                    encode_base_64(
                                        src.as_mut_ptr(),
                                        dst.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 2048]>()
                                            as libc::c_ulong as libc::c_int,
                                    );
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        b"Proxy-Authorization: Basic \0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        dst.as_mut_ptr() as *const libc::c_char,
                                    );
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        b"\r\n\r\n\0" as *const u8 as *const libc::c_char,
                                    );
                                } else {
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        b"\r\n\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                len = strlen(
                                    buff.as_mut_ptr() as *mut libc::c_char
                                        as *const libc::c_char,
                                );
                                tmp___2 = send(
                                    sock,
                                    buff.as_mut_ptr() as *const libc::c_void,
                                    len,
                                    0 as libc::c_int,
                                );
                                if !(len != tmp___2 as size_t) {
                                    len = 0 as libc::c_int as size_t;
                                    loop {
                                        if !(len < 8192 as libc::c_ulong) {
                                            current_block = 15594839951440953787;
                                            break;
                                        }
                                        tmp___3 = read_n_bytes(
                                            sock,
                                            buff.as_mut_ptr().offset(len as isize) as *mut libc::c_char,
                                            1 as libc::c_int as size_t,
                                        );
                                        if !(1 as libc::c_ulong == tmp___3) {
                                            current_block = 16889246570190342985;
                                            break;
                                        }
                                        len = len.wrapping_add(1);
                                        if !(len > 4 as libc::c_ulong) {
                                            continue;
                                        }
                                        if !(buff[len.wrapping_sub(1 as libc::c_ulong) as usize]
                                            as libc::c_int == 10 as libc::c_int)
                                        {
                                            continue;
                                        }
                                        if !(buff[len.wrapping_sub(2 as libc::c_ulong) as usize]
                                            as libc::c_int == 13 as libc::c_int)
                                        {
                                            continue;
                                        }
                                        if !(buff[len.wrapping_sub(3 as libc::c_ulong) as usize]
                                            as libc::c_int == 10 as libc::c_int)
                                        {
                                            continue;
                                        }
                                        if buff[len.wrapping_sub(4 as libc::c_ulong) as usize]
                                            as libc::c_int == 13 as libc::c_int
                                        {
                                            current_block = 15594839951440953787;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        16889246570190342985 => {}
                                        _ => {
                                            if len == 8192 as libc::c_ulong {
                                                return 5 as libc::c_int
                                            } else {
                                                if buff[9 as libc::c_int as usize] as libc::c_int
                                                    == 50 as libc::c_int
                                                {
                                                    if buff[10 as libc::c_int as usize] as libc::c_int
                                                        == 48 as libc::c_int
                                                    {
                                                        if !(buff[11 as libc::c_int as usize] as libc::c_int
                                                            == 48 as libc::c_int)
                                                        {
                                                            return 5 as libc::c_int;
                                                        }
                                                    } else {
                                                        return 5 as libc::c_int
                                                    }
                                                } else {
                                                    return 5 as libc::c_int
                                                }
                                            }
                                            return 0 as libc::c_int;
                                        }
                                    }
                                }
                            }
                            _ => {
                                if !user.is_null() {
                                    buff[0 as libc::c_int
                                        as usize] = 5 as libc::c_int as libc::c_uchar;
                                    buff[1 as libc::c_int
                                        as usize] = 2 as libc::c_int as libc::c_uchar;
                                    buff[2 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    buff[3 as libc::c_int
                                        as usize] = 2 as libc::c_int as libc::c_uchar;
                                    tmp___6 = write_n_bytes(
                                        sock,
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        4 as libc::c_int as size_t,
                                    );
                                    if 4 as libc::c_ulong != tmp___6 {
                                        current_block = 16889246570190342985;
                                    } else {
                                        current_block = 13201766686570145889;
                                    }
                                } else {
                                    buff[0 as libc::c_int
                                        as usize] = 5 as libc::c_int as libc::c_uchar;
                                    buff[1 as libc::c_int
                                        as usize] = 1 as libc::c_int as libc::c_uchar;
                                    buff[2 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    tmp___7 = write_n_bytes(
                                        sock,
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        3 as libc::c_int as size_t,
                                    );
                                    if 3 as libc::c_ulong != tmp___7 {
                                        current_block = 16889246570190342985;
                                    } else {
                                        current_block = 13201766686570145889;
                                    }
                                }
                                match current_block {
                                    16889246570190342985 => {}
                                    _ => {
                                        tmp___8 = read_n_bytes(
                                            sock,
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            2 as libc::c_int as size_t,
                                        );
                                        if !(2 as libc::c_ulong != tmp___8) {
                                            if buff[0 as libc::c_int as usize] as libc::c_int
                                                != 5 as libc::c_int
                                            {
                                                current_block = 6887742563588325581;
                                            } else {
                                                if buff[1 as libc::c_int as usize] as libc::c_int
                                                    != 0 as libc::c_int
                                                {
                                                    if buff[1 as libc::c_int as usize] as libc::c_int
                                                        != 2 as libc::c_int
                                                    {
                                                        current_block = 6887742563588325581;
                                                    } else {
                                                        current_block = 16314074004867283505;
                                                    }
                                                } else {
                                                    current_block = 16314074004867283505;
                                                }
                                                match current_block {
                                                    6887742563588325581 => {}
                                                    _ => {
                                                        if buff[1 as libc::c_int as usize] as libc::c_int
                                                            == 2 as libc::c_int
                                                        {
                                                            cur = out.as_mut_ptr();
                                                            tmp___9 = cur;
                                                            cur = cur.offset(1);
                                                            *tmp___9 = 1 as libc::c_int as libc::c_char;
                                                            c = (ulen & 255 as libc::c_ulong) as libc::c_int;
                                                            tmp___10 = cur;
                                                            cur = cur.offset(1);
                                                            *tmp___10 = c as libc::c_char;
                                                            memcpy(
                                                                cur as *mut libc::c_void,
                                                                user as *const libc::c_void,
                                                                c as size_t,
                                                            );
                                                            cur = cur.offset(c as isize);
                                                            c = (passlen & 255 as libc::c_ulong) as libc::c_int;
                                                            tmp___11 = cur;
                                                            cur = cur.offset(1);
                                                            *tmp___11 = c as libc::c_char;
                                                            memcpy(
                                                                cur as *mut libc::c_void,
                                                                pass as *const libc::c_void,
                                                                c as size_t,
                                                            );
                                                            cur = cur.offset(c as isize);
                                                            tmp___12 = write_n_bytes(
                                                                sock,
                                                                out.as_mut_ptr(),
                                                                cur.offset_from(out.as_mut_ptr()) as libc::c_long as size_t,
                                                            );
                                                            if cur.offset_from(out.as_mut_ptr()) as libc::c_long
                                                                as size_t != tmp___12
                                                            {
                                                                current_block = 16889246570190342985;
                                                            } else {
                                                                tmp___13 = read_n_bytes(
                                                                    sock,
                                                                    in_0.as_mut_ptr(),
                                                                    2 as libc::c_int as size_t,
                                                                );
                                                                if 2 as libc::c_ulong != tmp___13 {
                                                                    current_block = 16889246570190342985;
                                                                } else {
                                                                    if in_0[0 as libc::c_int as usize] as libc::c_int
                                                                        != 1 as libc::c_int
                                                                    {
                                                                        current_block = 7368271564846522885;
                                                                    } else if in_0[1 as libc::c_int as usize] as libc::c_int
                                                                            != 0 as libc::c_int
                                                                        {
                                                                        current_block = 7368271564846522885;
                                                                    } else {
                                                                        current_block = 14883390358315838856;
                                                                    }
                                                                    match current_block {
                                                                        14883390358315838856 => {}
                                                                        _ => {
                                                                            if in_0[0 as libc::c_int as usize] as libc::c_int
                                                                                != 1 as libc::c_int
                                                                            {
                                                                                current_block = 16889246570190342985;
                                                                            } else {
                                                                                return 5 as libc::c_int
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            current_block = 14883390358315838856;
                                                        }
                                                        match current_block {
                                                            16889246570190342985 => {}
                                                            _ => {
                                                                buff_iter = 0 as libc::c_int as size_t;
                                                                tmp___14 = buff_iter;
                                                                buff_iter = buff_iter.wrapping_add(1);
                                                                buff[tmp___14 as usize] = 5 as libc::c_int as libc::c_uchar;
                                                                tmp___15 = buff_iter;
                                                                buff_iter = buff_iter.wrapping_add(1);
                                                                buff[tmp___15 as usize] = 1 as libc::c_int as libc::c_uchar;
                                                                tmp___16 = buff_iter;
                                                                buff_iter = buff_iter.wrapping_add(1);
                                                                buff[tmp___16 as usize] = 0 as libc::c_int as libc::c_uchar;
                                                                if dns_len == 0 {
                                                                    tmp___17 = buff_iter;
                                                                    buff_iter = buff_iter.wrapping_add(1);
                                                                    buff[tmp___17 as usize] = 1 as libc::c_int as libc::c_uchar;
                                                                    memcpy(
                                                                        buff.as_mut_ptr().offset(buff_iter as isize)
                                                                            as *mut libc::c_void,
                                                                        &mut ip as *mut ip_type as *const libc::c_void,
                                                                        4 as libc::c_int as size_t,
                                                                    );
                                                                    buff_iter = (buff_iter as libc::c_ulong)
                                                                        .wrapping_add(4 as libc::c_ulong) as size_t as size_t;
                                                                } else {
                                                                    tmp___18 = buff_iter;
                                                                    buff_iter = buff_iter.wrapping_add(1);
                                                                    buff[tmp___18 as usize] = 3 as libc::c_int as libc::c_uchar;
                                                                    tmp___19 = buff_iter;
                                                                    buff_iter = buff_iter.wrapping_add(1);
                                                                    buff[tmp___19
                                                                        as usize] = (dns_len & 255 as libc::c_ulong)
                                                                        as libc::c_uchar;
                                                                    memcpy(
                                                                        buff.as_mut_ptr().offset(buff_iter as isize)
                                                                            as *mut libc::c_void,
                                                                        dns_name as *const libc::c_void,
                                                                        dns_len,
                                                                    );
                                                                    buff_iter = (buff_iter as libc::c_ulong)
                                                                        .wrapping_add(dns_len) as size_t as size_t;
                                                                }
                                                                memcpy(
                                                                    buff.as_mut_ptr().offset(buff_iter as isize)
                                                                        as *mut libc::c_void,
                                                                    &mut port as *mut libc::c_ushort as *const libc::c_void,
                                                                    2 as libc::c_int as size_t,
                                                                );
                                                                buff_iter = (buff_iter as libc::c_ulong)
                                                                    .wrapping_add(2 as libc::c_ulong) as size_t as size_t;
                                                                tmp___20 = write_n_bytes(
                                                                    sock,
                                                                    buff.as_mut_ptr() as *mut libc::c_char,
                                                                    buff_iter,
                                                                );
                                                                if buff_iter != tmp___20 {
                                                                    current_block = 16889246570190342985;
                                                                } else {
                                                                    tmp___21 = read_n_bytes(
                                                                        sock,
                                                                        buff.as_mut_ptr() as *mut libc::c_char,
                                                                        4 as libc::c_int as size_t,
                                                                    );
                                                                    if 4 as libc::c_ulong != tmp___21 {
                                                                        current_block = 16889246570190342985;
                                                                    } else if buff[0 as libc::c_int as usize] as libc::c_int
                                                                            != 5 as libc::c_int
                                                                        {
                                                                        current_block = 16889246570190342985;
                                                                    } else if buff[1 as libc::c_int as usize] as libc::c_int
                                                                            != 0 as libc::c_int
                                                                        {
                                                                        current_block = 16889246570190342985;
                                                                    } else {
                                                                        match buff[3 as libc::c_int as usize] as libc::c_int {
                                                                            1 => {
                                                                                current_block = 9157488150074734138;
                                                                                match current_block {
                                                                                    14120677076171701346 => {
                                                                                        len = 16 as libc::c_int as size_t;
                                                                                        current_block = 9879293939512133550;
                                                                                    }
                                                                                    9157488150074734138 => {
                                                                                        len = 4 as libc::c_int as size_t;
                                                                                        current_block = 9879293939512133550;
                                                                                    }
                                                                                    _ => {
                                                                                        len = 0 as libc::c_int as size_t;
                                                                                        tmp___22 = read_n_bytes(
                                                                                            sock,
                                                                                            &mut len as *mut size_t as *mut libc::c_char,
                                                                                            1 as libc::c_int as size_t,
                                                                                        );
                                                                                        if 1 as libc::c_ulong != tmp___22 {
                                                                                            current_block = 16889246570190342985;
                                                                                        } else {
                                                                                            current_block = 9879293939512133550;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                match current_block {
                                                                                    16889246570190342985 => {}
                                                                                    _ => {
                                                                                        tmp___23 = read_n_bytes(
                                                                                            sock,
                                                                                            buff.as_mut_ptr() as *mut libc::c_char,
                                                                                            len.wrapping_add(2 as libc::c_ulong),
                                                                                        );
                                                                                        if len.wrapping_add(2 as libc::c_ulong) != tmp___23 {
                                                                                            current_block = 16889246570190342985;
                                                                                        } else {
                                                                                            return 0 as libc::c_int
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            4 => {
                                                                                current_block = 14120677076171701346;
                                                                                match current_block {
                                                                                    14120677076171701346 => {
                                                                                        len = 16 as libc::c_int as size_t;
                                                                                        current_block = 9879293939512133550;
                                                                                    }
                                                                                    9157488150074734138 => {
                                                                                        len = 4 as libc::c_int as size_t;
                                                                                        current_block = 9879293939512133550;
                                                                                    }
                                                                                    _ => {
                                                                                        len = 0 as libc::c_int as size_t;
                                                                                        tmp___22 = read_n_bytes(
                                                                                            sock,
                                                                                            &mut len as *mut size_t as *mut libc::c_char,
                                                                                            1 as libc::c_int as size_t,
                                                                                        );
                                                                                        if 1 as libc::c_ulong != tmp___22 {
                                                                                            current_block = 16889246570190342985;
                                                                                        } else {
                                                                                            current_block = 9879293939512133550;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                match current_block {
                                                                                    16889246570190342985 => {}
                                                                                    _ => {
                                                                                        tmp___23 = read_n_bytes(
                                                                                            sock,
                                                                                            buff.as_mut_ptr() as *mut libc::c_char,
                                                                                            len.wrapping_add(2 as libc::c_ulong),
                                                                                        );
                                                                                        if len.wrapping_add(2 as libc::c_ulong) != tmp___23 {
                                                                                            current_block = 16889246570190342985;
                                                                                        } else {
                                                                                            return 0 as libc::c_int
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            3 => {
                                                                                current_block = 2261659062341822993;
                                                                                match current_block {
                                                                                    14120677076171701346 => {
                                                                                        len = 16 as libc::c_int as size_t;
                                                                                        current_block = 9879293939512133550;
                                                                                    }
                                                                                    9157488150074734138 => {
                                                                                        len = 4 as libc::c_int as size_t;
                                                                                        current_block = 9879293939512133550;
                                                                                    }
                                                                                    _ => {
                                                                                        len = 0 as libc::c_int as size_t;
                                                                                        tmp___22 = read_n_bytes(
                                                                                            sock,
                                                                                            &mut len as *mut size_t as *mut libc::c_char,
                                                                                            1 as libc::c_int as size_t,
                                                                                        );
                                                                                        if 1 as libc::c_ulong != tmp___22 {
                                                                                            current_block = 16889246570190342985;
                                                                                        } else {
                                                                                            current_block = 9879293939512133550;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                match current_block {
                                                                                    16889246570190342985 => {}
                                                                                    _ => {
                                                                                        tmp___23 = read_n_bytes(
                                                                                            sock,
                                                                                            buff.as_mut_ptr() as *mut libc::c_char,
                                                                                            len.wrapping_add(2 as libc::c_ulong),
                                                                                        );
                                                                                        if len.wrapping_add(2 as libc::c_ulong) != tmp___23 {
                                                                                            current_block = 16889246570190342985;
                                                                                        } else {
                                                                                            return 0 as libc::c_int
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            _ => {
                                                                                current_block = 16889246570190342985;
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
                                                16889246570190342985 => {}
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
    let mut addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut ip_buf: [libc::c_char; 16] = [0; 16];
    let mut tmp: __uint16_t = 0;
    let mut tmp___0: libc::c_int = 0;
    *fd = socket(2 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int);
    if !(*fd == -(1 as libc::c_int)) {
        inet_ntop(
            2 as libc::c_int,
            &mut *((*pd).ip.octet).as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut libc::c_uchar as *const libc::c_void,
            ip_buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as socklen_t,
        );
        tmp = __bswap_16((*pd).port);
        proxychains_write_log(
            b"[proxychains] %s  ...  %s:%d \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            begin_mark,
            ip_buf.as_mut_ptr(),
            tmp as libc::c_int,
        );
        (*pd).ps = PLAY_STATE;
        memset(
            &mut addr as *mut sockaddr_in as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
        );
        addr.sin_family = 2 as libc::c_int as sa_family_t;
        addr.sin_addr.s_addr = (*pd).ip.as_int;
        addr.sin_port = (*pd).port;
        tmp___0 = timed_connect(
            *fd,
            &mut addr as *mut sockaddr_in as *mut sockaddr as *const sockaddr,
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
        );
        if tmp___0 != 0 {
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
    if *fd != -(1 as libc::c_int) {
        close(*fd);
    }
    return 2 as libc::c_int;
}
static mut fp: *mut FILE = 0 as *const FILE as *mut FILE;
pub unsafe extern "C" fn get_rand_int(mut range: libc::c_uint) -> libc::c_uint {
    let mut randval: libc::c_uint = 0;
    let mut tmp: time_t = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: size_t = 0;
    if fp.is_null() {
        fp = fopen(
            b"/dev/urandom\0" as *const u8 as *const libc::c_char,
            b"r\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___1 = fread(
        &mut randval as *mut libc::c_uint as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
        1 as libc::c_int as size_t,
        fp,
    );
    if tmp___1 != 0 {
        return randval.wrapping_rem(range)
    } else {
        tmp = time(0 as *mut libc::c_void as *mut time_t);
        srand(tmp as libc::c_uint);
        tmp___0 = rand();
        return (tmp___0 as libc::c_uint).wrapping_rem(range);
    };
}
unsafe extern "C" fn select_proxy(
    mut how: select_type,
    mut pd: *mut proxy_data,
    mut proxy_count: libc::c_uint,
    mut offset: *mut libc::c_uint,
) -> *mut proxy_data {
    let mut i: libc::c_uint = 0;
    let mut k: libc::c_uint = 0;
    let mut tmp: libc::c_uint = 0;
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
                tmp = get_rand_int(proxy_count);
                i = tmp;
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
    let mut alive_count: libc::c_uint = 0;
    alive_count = 0 as libc::c_uint;
    release_busy(pd, proxy_count);
    i = 0 as libc::c_uint;
    while i < proxy_count {
        if (*pd.offset(i as isize)).ps as libc::c_uint == 0 as libc::c_uint {
            alive_count = alive_count.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    return alive_count;
}
unsafe extern "C" fn chain_step(
    mut ns: libc::c_int,
    mut pfrom: *mut proxy_data,
    mut pto: *mut proxy_data,
) -> libc::c_int {
    let mut retcode: libc::c_int = 0;
    let mut hostname___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ip_buf: [libc::c_char; 16] = [0; 16];
    let mut tmp: __uint16_t = 0;
    retcode = -(1 as libc::c_int);
    let mut current_block_4: u64;
    if (*pto).ip.octet[0 as libc::c_int as usize] as libc::c_uint == remote_dns_subnet {
        hostname___0 = string_from_internal_ip((*pto).ip);
        if hostname___0.is_null() {
            current_block_4 = 12589646774865678522;
        } else {
            current_block_4 = 1394248824506584008;
        }
    } else {
        current_block_4 = 12589646774865678522;
    }
    match current_block_4 {
        12589646774865678522 => {
            inet_ntop(
                2 as libc::c_int,
                &mut *((*pto).ip.octet).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut libc::c_uchar as *const libc::c_void,
                ip_buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as socklen_t,
            );
            hostname___0 = ip_buf.as_mut_ptr();
        }
        _ => {}
    }
    tmp = __bswap_16((*pto).port);
    proxychains_write_log(
        b" ...  %s:%d \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        hostname___0,
        tmp as libc::c_int,
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
        ip: __anonunion_ip_type_826858479 {
            octet: [0; 4],
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
    let mut offset: libc::c_uint = 0;
    let mut alive_count: libc::c_uint = 0;
    let mut curr_len: libc::c_uint = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___9: *mut libc::c_int = 0 as *mut libc::c_int;
    ns = -(1 as libc::c_int);
    offset = 0 as libc::c_uint;
    alive_count = 0 as libc::c_uint;
    curr_len = 0 as libc::c_uint;
    p3 = &mut p4;
    '_again: loop {
        match ct as libc::c_uint {
            0 => {
                calc_alive(pd, proxy_count);
                offset = 0 as libc::c_uint;
                loop {
                    p1 = select_proxy(FIFOLY, pd, proxy_count, &mut offset);
                    if p1.is_null() {
                        current_block = 5968063871281718162;
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
                    current_block = 3927114851844792182;
                    break;
                } else {
                    current_block = 993425571616822999;
                    break;
                }
            }
            1 => {
                calc_alive(pd, proxy_count);
                offset = 0 as libc::c_uint;
                p1 = select_proxy(FIFOLY, pd, proxy_count, &mut offset);
                if p1.is_null() {
                    current_block = 496303045384785551;
                    break;
                } else {
                    current_block = 980989089337379490;
                    break;
                }
            }
            2 => {
                alive_count = calc_alive(pd, proxy_count);
                if alive_count < max_chain {
                    current_block = 5968063871281718162;
                    break;
                }
                offset = 0 as libc::c_uint;
                curr_len = offset;
                loop {
                    p1 = select_proxy(RANDOMLY, pd, proxy_count, &mut offset);
                    if p1.is_null() {
                        current_block = 5968063871281718162;
                        break '_again;
                    }
                    tmp___5 = start_chain(
                        &mut ns,
                        p1,
                        b"Random chain\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    if !(0 as libc::c_int != tmp___5) {
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
                        current_block = 5968063871281718162;
                        break '_again;
                    }
                    tmp___6 = chain_step(ns, p1, p2);
                    if 0 as libc::c_int != tmp___6 {
                        continue '_again;
                    }
                    p1 = p2;
                }
                (*p3).ip = target_ip;
                (*p3).port = target_port;
                tmp___7 = chain_step(ns, p1, p3);
                if 0 as libc::c_int != tmp___7 {
                    current_block = 3927114851844792182;
                    break;
                } else {
                    current_block = 993425571616822999;
                    break;
                }
            }
            _ => {
                current_block = 993425571616822999;
                break;
            }
        }
    }
    match current_block {
        980989089337379490 => {
            tmp___2 = start_chain(
                &mut ns,
                p1,
                b"Strict chain\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            if 0 as libc::c_int != tmp___2 {
                current_block = 496303045384785551;
            } else {
                loop {
                    if !(offset < proxy_count) {
                        current_block = 9627623479216730126;
                        break;
                    }
                    p2 = select_proxy(FIFOLY, pd, proxy_count, &mut offset);
                    if p2.is_null() {
                        current_block = 9627623479216730126;
                        break;
                    }
                    tmp___3 = chain_step(ns, p1, p2);
                    if 0 as libc::c_int != tmp___3 {
                        current_block = 496303045384785551;
                        break;
                    }
                    p1 = p2;
                }
                match current_block {
                    496303045384785551 => {}
                    _ => {
                        (*p3).ip = target_ip;
                        (*p3).port = target_port;
                        tmp___4 = chain_step(ns, p1, p3);
                        if 0 as libc::c_int != tmp___4 {
                            current_block = 3927114851844792182;
                        } else {
                            current_block = 993425571616822999;
                        }
                    }
                }
            }
        }
        5968063871281718162 => {
            proxychains_write_log(
                b"\n!!!need more proxies!!!\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            current_block = 496303045384785551;
        }
        _ => {}
    }
    match current_block {
        496303045384785551 => {
            release_all(pd, proxy_count);
            if ns != -(1 as libc::c_int) {
                close(ns);
            }
            tmp___9 = __errno_location();
            *tmp___9 = 110 as libc::c_int;
            return -(1 as libc::c_int);
        }
        3927114851844792182 => {
            if ns != -(1 as libc::c_int) {
                close(ns);
            }
            tmp___8 = __errno_location();
            *tmp___8 = 111 as libc::c_int;
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
static mut local_host: __anonunion_ip_type_826858479 = __anonunion_ip_type_826858479 {
    octet: [
        127 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
    ],
};
pub static mut hostname: [libc::c_char; 256] = [0; 256];
pub unsafe extern "C" fn proxy_gethostbyname(
    mut name: *const libc::c_char,
    mut data: *mut gethostbyname_data,
) -> *mut hostent {
    let mut current_block: u64;
    let mut buff: [libc::c_char; 256] = [0; 256];
    let mut i: uint32_t = 0;
    let mut hash: uint32_t = 0;
    let mut new_mem: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut l: size_t = 0;
    let mut hp: *mut hostent = 0 as *mut hostent;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    (*data)
        .resolved_addr_p[0 as libc::c_int
        as usize] = &mut (*data).resolved_addr as *mut in_addr_t as *mut libc::c_char;
    (*data)
        .resolved_addr_p[1 as libc::c_int
        as usize] = 0 as *mut libc::c_void as *mut libc::c_char;
    (*data).hostent_space.h_addr_list = ((*data).resolved_addr_p).as_mut_ptr();
    (*data).resolved_addr = 0 as libc::c_int as in_addr_t;
    gethostname(
        buff.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
    );
    tmp = strcmp(buff.as_mut_ptr() as *const libc::c_char, name);
    if tmp == 0 {
        (*data).resolved_addr = inet_addr(buff.as_mut_ptr() as *const libc::c_char);
        if (*data).resolved_addr == 4294967295 as libc::c_uint {
            (*data).resolved_addr = local_host.as_int;
        }
        snprintf(
            hostname.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            b"%s\0" as *const u8 as *const libc::c_char,
            name,
        );
        (*data).hostent_space.h_name = hostname.as_mut_ptr();
        (*data)
            .hostent_space
            .h_length = ::std::mem::size_of::<in_addr_t>() as libc::c_ulong
            as libc::c_int;
        (*data).hostent_space.h_addrtype = 2 as libc::c_int;
        return &mut (*data).hostent_space;
    }
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
        tmp___0 = strcmp((*hp).h_name as *const libc::c_char, name);
        if tmp___0 == 0 {
            return hp;
        }
    }
    hash = dalias_hash(name as *mut libc::c_char);
    pthread_mutex_lock(&mut internal_ips_lock);
    if internal_ips.counter != 0 {
        i = 0 as libc::c_int as uint32_t;
        loop {
            if !(i < internal_ips.counter) {
                current_block = 1538046216550696469;
                break;
            }
            if (**(internal_ips.list).offset(i as isize)).hash == hash {
                tmp___1 = strcmp(
                    name,
                    (**(internal_ips.list).offset(i as isize)).string
                        as *const libc::c_char,
                );
                if tmp___1 == 0 {
                    (*data).resolved_addr = make_internal_ip(i);
                    current_block = 4316544856007887735;
                    break;
                }
            }
            i = i.wrapping_add(1);
        }
    } else {
        current_block = 1538046216550696469;
    }
    match current_block {
        1538046216550696469 => {
            if internal_ips.capa < (internal_ips.counter).wrapping_add(1 as libc::c_uint)
            {
                new_mem = realloc(
                    internal_ips.list as *mut libc::c_void,
                    ((internal_ips.capa).wrapping_add(16 as libc::c_uint)
                        as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                        ),
                );
                if !new_mem.is_null() {
                    internal_ips
                        .capa = (internal_ips.capa as libc::c_uint)
                        .wrapping_add(16 as libc::c_uint) as uint32_t as uint32_t;
                    internal_ips.list = new_mem as *mut *mut string_hash_tuple;
                    current_block = 572715077006366937;
                } else {
                    current_block = 5300578207441769350;
                }
            } else {
                current_block = 572715077006366937;
            }
            match current_block {
                572715077006366937 => {
                    (*data).resolved_addr = make_internal_ip(internal_ips.counter);
                    if (*data).resolved_addr == 4294967295 as libc::c_uint {
                        current_block = 9492292218138856045;
                    } else {
                        l = strlen(name);
                        new_mem = malloc(
                            (::std::mem::size_of::<string_hash_tuple>() as libc::c_ulong)
                                .wrapping_add(l)
                                .wrapping_add(1 as libc::c_ulong),
                        );
                        if new_mem.is_null() {
                            current_block = 5300578207441769350;
                        } else {
                            let ref mut fresh0 = *(internal_ips.list)
                                .offset(internal_ips.counter as isize);
                            *fresh0 = new_mem as *mut string_hash_tuple;
                            (**(internal_ips.list).offset(internal_ips.counter as isize))
                                .hash = hash;
                            let ref mut fresh1 = (**(internal_ips.list)
                                .offset(internal_ips.counter as isize))
                                .string;
                            *fresh1 = (new_mem as *mut libc::c_char)
                                .offset(
                                    ::std::mem::size_of::<string_hash_tuple>() as libc::c_ulong
                                        as isize,
                                );
                            memcpy(
                                (**(internal_ips.list)
                                    .offset(internal_ips.counter as isize))
                                    .string as *mut libc::c_void,
                                name as *const libc::c_void,
                                l.wrapping_add(1 as libc::c_ulong),
                            );
                            internal_ips
                                .counter = (internal_ips.counter).wrapping_add(1);
                            current_block = 4316544856007887735;
                        }
                    }
                }
                _ => {}
            }
            match current_block {
                4316544856007887735 => {}
                _ => {
                    match current_block {
                        5300578207441769350 => {
                            proxychains_write_log(
                                b"out of mem\n\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        }
                        _ => {}
                    }
                    pthread_mutex_unlock(&mut internal_ips_lock);
                    return 0 as *mut libc::c_void as *mut hostent;
                }
            }
        }
        _ => {}
    }
    pthread_mutex_unlock(&mut internal_ips_lock);
    strncpy(
        ((*data).addr_name).as_mut_ptr(),
        name,
        (::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_ulong),
    );
    (*data).hostent_space.h_name = ((*data).addr_name).as_mut_ptr();
    (*data)
        .hostent_space
        .h_length = ::std::mem::size_of::<in_addr_t>() as libc::c_ulong as libc::c_int;
    (*data).hostent_space.h_addrtype = 2 as libc::c_int;
    return &mut (*data).hostent_space;
}
pub unsafe extern "C" fn proxy_freeaddrinfo(mut res: *mut addrinfo) {
    free(res as *mut libc::c_void);
}
pub unsafe extern "C" fn proxy_getservbyname(
    mut service: *const libc::c_char,
    mut se_buf: *mut servent,
    mut buf___0: *mut libc::c_char,
    mut buf_len: size_t,
    mut se_result: *mut *mut servent,
) {
    getservbyname_r(
        service,
        0 as *mut libc::c_void as *const libc::c_char,
        se_buf,
        buf___0,
        buf_len,
        se_result,
    );
}
pub unsafe extern "C" fn proxy_getaddrinfo(
    mut node: *const libc::c_char,
    mut service: *const libc::c_char,
    mut hints: *const addrinfo,
    mut res: *mut *mut addrinfo,
) -> libc::c_int {
    let mut ghdata: gethostbyname_data = gethostbyname_data {
        hostent_space: hostent {
            h_name: 0 as *mut libc::c_char,
            h_aliases: 0 as *mut *mut libc::c_char,
            h_addrtype: 0,
            h_length: 0,
            h_addr_list: 0 as *mut *mut libc::c_char,
        },
        resolved_addr: 0,
        resolved_addr_p: [0 as *mut libc::c_char; 2],
        addr_name: [0; 8192],
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
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: __uint16_t = 0;
    let mut tmp___4: sa_family_t = 0;
    se = 0 as *mut libc::c_void as *mut servent;
    hp = 0 as *mut libc::c_void as *mut hostent;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<addrinfo_data>() as libc::c_ulong,
    );
    space = tmp as *mut addrinfo_data;
    if space.is_null() {
        return 1 as libc::c_int;
    }
    if !node.is_null() {
        tmp___0 = inet_aton(
            node,
            &mut (*(&mut (*space).sockaddr_space as *mut sockaddr as *mut sockaddr_in))
                .sin_addr,
        );
        if tmp___0 == 0 {
            hp = proxy_gethostbyname(node, &mut ghdata);
            if !hp.is_null() {
                memcpy(
                    &mut (*(&mut (*space).sockaddr_space as *mut sockaddr
                        as *mut sockaddr_in))
                        .sin_addr as *mut in_addr as *mut libc::c_void,
                    *(*hp).h_addr_list as *const libc::c_void,
                    ::std::mem::size_of::<in_addr_t>() as libc::c_ulong,
                );
            } else {
                free(space as *mut libc::c_void);
                return 1 as libc::c_int;
            }
        }
    }
    if !service.is_null() {
        proxy_getservbyname(
            service,
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
            tmp___1 = service;
        } else {
            tmp___1 = b"0\0" as *const u8 as *const libc::c_char;
        }
        tmp___2 = atoi(tmp___1);
        tmp___3 = __bswap_16(tmp___2 as uint16_t);
        port = tmp___3 as libc::c_int;
    }
    (*(&mut (*space).sockaddr_space as *mut sockaddr as *mut sockaddr_in))
        .sin_port = port as in_port_t;
    p = &mut (*space).addrinfo_space;
    *res = p;
    if !(p as size_t == space as size_t) {
        __assert_fail(
            b"(size_t)p == (size_t) space\0" as *const u8 as *const libc::c_char,
            b"src/core.c\0" as *const u8 as *const libc::c_char,
            913 as libc::c_uint,
            b"proxy_getaddrinfo\0" as *const u8 as *const libc::c_char,
        );
    }
    (*p).ai_addr = &mut (*space).sockaddr_space;
    if !node.is_null() {
        strncpy(
            ((*space).addr_name).as_mut_ptr(),
            node,
            (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_ulong),
        );
    }
    (*p).ai_canonname = ((*space).addr_name).as_mut_ptr();
    (*p).ai_next = 0 as *mut libc::c_void as *mut addrinfo;
    tmp___4 = 2 as libc::c_int as sa_family_t;
    (*space).sockaddr_space.sa_family = tmp___4;
    (*p).ai_family = tmp___4 as libc::c_int;
    (*p).ai_addrlen = ::std::mem::size_of::<sockaddr>() as libc::c_ulong as socklen_t;
    if !hints.is_null() {
        (*p).ai_socktype = (*hints).ai_socktype;
        (*p).ai_flags = (*hints).ai_flags;
        (*p).ai_protocol = (*hints).ai_protocol;
    } else {
        (*p).ai_flags = 32 as libc::c_int;
    }
    return 0 as libc::c_int;
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
    path = default_path;
    tmp = check_path(path);
    if tmp != 0 {
        return path;
    }
    if !pbuf.is_null() {
        path = getenv(b"PROXYCHAINS_CONF_FILE\0" as *const u8 as *const libc::c_char);
        tmp___0 = check_path(path);
        if tmp___0 != 0 {
            return path;
        }
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
        if tmp___1 != 0 {
            return path;
        }
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
        if tmp___2 != 0 {
            return path;
        }
        path = b"/usr/local/etc/proxychains.conf\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        tmp___3 = check_path(path);
        if tmp___3 != 0 {
            return path;
        }
        path = b"/etc/proxychains.conf\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        tmp___4 = check_path(path);
        if tmp___4 != 0 {
            return path;
        }
    }
    perror(b"couldnt find configuration file\0" as *const u8 as *const libc::c_char);
    exit(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 4278190080 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 16711680 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 65280 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 255 as libc::c_uint) << 24 as libc::c_int;
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
pub static mut tcp_read_time_out: libc::c_int = 0;
pub static mut tcp_connect_time_out: libc::c_int = 0;
pub static mut proxychains_got_chain_data: libc::c_int = 0 as libc::c_int;
pub static mut proxychains_quiet_mode: libc::c_int = 0 as libc::c_int;
pub static mut proxychains_resolver: libc::c_int = 0 as libc::c_int;
pub static mut proxychains_proxy_count: libc::c_uint = 0 as libc::c_uint;
pub static mut proxychains_max_chain: libc::c_uint = 1 as libc::c_uint;
pub static mut remote_dns_subnet: libc::c_uint = 224 as libc::c_uint;
pub static mut localnet_addr: [localaddr_arg; 64] = [localaddr_arg {
    in_addr: in_addr { s_addr: 0 },
    netmask: in_addr { s_addr: 0 },
    port: 0,
}; 64];
pub static mut proxychains_ct: chain_type = DYNAMIC_TYPE;
pub static mut proxychains_pd: [proxy_data; 512] = [proxy_data {
    ip: __anonunion_ip_type_826858479 {
        octet: [0; 4],
    },
    port: 0,
    pt: HTTP_TYPE,
    ps: PLAY_STATE,
    user: [0; 256],
    pass: [0; 256],
}; 512];
pub static mut num_localnet_addr: size_t = 0 as libc::c_int as size_t;
pub static mut dnats: [dnat_arg; 64] = [dnat_arg {
    orig_dst: in_addr { s_addr: 0 },
    new_dst: in_addr { s_addr: 0 },
    orig_port: 0,
    new_port: 0,
}; 64];
pub static mut num_dnats: size_t = 0 as libc::c_int as size_t;
pub static mut init_once: pthread_once_t = 0 as libc::c_int;
static mut init_l: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn load_sym(
    mut symname: *mut libc::c_char,
    mut proxyfunc: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut funcptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = dlsym(
        -(1 as libc::c_long) as *mut libc::c_void,
        symname as *const libc::c_char,
    );
    funcptr = tmp;
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
    if funcptr as libc::c_ulong == proxyfunc as libc::c_ulong {
        abort();
    }
    return funcptr;
}
unsafe extern "C" fn do_init() {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    pthread_mutex_init(
        &mut internal_ips_lock,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
    simple_socks5_env(
        proxychains_pd.as_mut_ptr(),
        &mut proxychains_proxy_count,
        &mut proxychains_ct,
    );
    get_chain_data(
        proxychains_pd.as_mut_ptr(),
        &mut proxychains_proxy_count,
        &mut proxychains_ct,
    );
    proxychains_write_log(
        b"[proxychains] DLL init\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    tmp = load_sym(
        b"connect\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    libc::c_int,
                    *const sockaddr,
                    socklen_t,
                ) -> libc::c_int,
            >,
            *mut libc::c_void,
        >(
            Some(
                connect
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *const sockaddr,
                        socklen_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    true_connect = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(libc::c_int, *const sockaddr, socklen_t) -> libc::c_int,
        >,
    >(tmp);
    tmp___0 = load_sym(
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
    );
    true_gethostbyname = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(*const libc::c_char) -> *mut hostent>,
    >(tmp___0);
    tmp___1 = load_sym(
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
    >(tmp___1);
    tmp___2 = load_sym(
        b"freeaddrinfo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut addrinfo) -> ()>,
            *mut libc::c_void,
        >(Some(freeaddrinfo as unsafe extern "C" fn(*mut addrinfo) -> ())),
    );
    true_freeaddrinfo = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(*mut addrinfo) -> libc::c_int>,
    >(tmp___2);
    tmp___3 = load_sym(
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
    >(tmp___3);
    tmp___4 = load_sym(
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
    >(tmp___4);
    init_l = 1 as libc::c_int;
}
unsafe extern "C" fn init_lib_wrapper(mut caller: *const libc::c_char) {
    init_l == 0;
    pthread_once(&mut init_once, Some(do_init as unsafe extern "C" fn() -> ()));
}
pub unsafe extern "C" fn open_config_file() -> *mut FILE {
    let mut home_conf: [libc::c_char; 4096] = [0; 4096];
    let mut prefix_conf: [libc::c_char; 4096] = [0; 4096];
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = getenv(b"HOME\0" as *const u8 as *const libc::c_char);
    snprintf(
        home_conf.as_mut_ptr(),
        256 as libc::c_int as size_t,
        b"%s/.proxychains/proxychains.conf\0" as *const u8 as *const libc::c_char,
        tmp,
    );
    snprintf(
        prefix_conf.as_mut_ptr(),
        256 as libc::c_int as size_t,
        b"%s/etc/proxychains.conf\0" as *const u8 as *const libc::c_char,
        b"/usr/local\0" as *const u8 as *const libc::c_char,
    );
    file = fopen(
        b"./proxychains.conf\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if file.is_null() {
        file = fopen(
            home_conf.as_mut_ptr() as *const libc::c_char,
            b"r\0" as *const u8 as *const libc::c_char,
        );
        if file.is_null() {
            file = fopen(
                prefix_conf.as_mut_ptr() as *const libc::c_char,
                b"r\0" as *const u8 as *const libc::c_char,
            );
            if file.is_null() {
                file = fopen(
                    b"/etc/proxychains.conf\0" as *const u8 as *const libc::c_char,
                    b"r\0" as *const u8 as *const libc::c_char,
                );
                if file.is_null() {
                    perror(
                        b"Can't locate proxychains.conf\0" as *const u8
                            as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
            }
        }
    }
    return file;
}
unsafe extern "C" fn load_default_settings(mut ct: *mut chain_type) {
    let mut env: *mut libc::c_char = 0 as *mut libc::c_char;
    tcp_read_time_out = 4000 as libc::c_int;
    tcp_connect_time_out = 10000 as libc::c_int;
    *ct = DYNAMIC_TYPE;
    env = getenv(b"PROXYCHAINS_QUIET_MODE\0" as *const u8 as *const libc::c_char);
    if !env.is_null() {
        if *env as libc::c_int == 49 as libc::c_int {
            proxychains_quiet_mode = 1 as libc::c_int;
        }
    }
}
unsafe extern "C" fn get_chain_data(
    mut pd: *mut proxy_data,
    mut proxy_count: *mut libc::c_uint,
    mut ct: *mut chain_type,
) {
    let mut count: libc::c_uint = 0;
    let mut port_n: libc::c_int = 0;
    let mut list___0: libc::c_int = 0;
    let mut buff: [libc::c_char; 1024] = [0; 1024];
    let mut type_0: [libc::c_char; 1024] = [0; 1024];
    let mut host: [libc::c_char; 1024] = [0; 1024];
    let mut user: [libc::c_char; 1024] = [0; 1024];
    let mut env: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut local_in_addr_port: [libc::c_char; 32] = [0; 32];
    let mut local_in_addr: [libc::c_char; 32] = [0; 32];
    let mut local_in_port: [libc::c_char; 32] = [0; 32];
    let mut local_netmask: [libc::c_char; 32] = [0; 32];
    let mut dnat_orig_addr_port: [libc::c_char; 32] = [0; 32];
    let mut dnat_new_addr_port: [libc::c_char; 32] = [0; 32];
    let mut dnat_orig_addr: [libc::c_char; 32] = [0; 32];
    let mut dnat_orig_port: [libc::c_char; 32] = [0; 32];
    let mut dnat_new_addr: [libc::c_char; 32] = [0; 32];
    let mut dnat_new_port: [libc::c_char; 32] = [0; 32];
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: in_addr_t = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut error: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut pc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_int = 0;
    let mut error___0: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___13: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___14: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___15: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___16: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___17: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___18: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___19: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___20: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___21: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___22: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___23: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___24: size_t = 0;
    let mut tmp___25: *mut libc::c_char = 0 as *mut libc::c_char;
    count = 0 as libc::c_uint;
    port_n = 0 as libc::c_int;
    list___0 = 0 as libc::c_int;
    file = 0 as *mut libc::c_void as *mut FILE;
    if proxychains_got_chain_data != 0 {
        return;
    }
    load_default_settings(ct);
    tmp = getenv(b"PROXYCHAINS_CONF_FILE\0" as *const u8 as *const libc::c_char);
    env = get_config_path(
        tmp,
        buff.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    file = fopen(env as *const libc::c_char, b"r\0" as *const u8 as *const libc::c_char);
    loop {
        tmp___25 = fgets(
            buff.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                as libc::c_int,
            file,
        );
        if tmp___25.is_null() {
            break;
        }
        if !(buff[0 as libc::c_int as usize] as libc::c_int != 10 as libc::c_int) {
            continue;
        }
        tmp___24 = strspn(
            buff.as_mut_ptr() as *const libc::c_char,
            b" \0" as *const u8 as *const libc::c_char,
        );
        if !(buff[tmp___24 as usize] as libc::c_int != 35 as libc::c_int) {
            continue;
        }
        if list___0 != 0 {
            if count >= 512 as libc::c_uint {
                break;
            }
            memset(
                pd.offset(count as isize) as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<proxy_data>() as libc::c_ulong,
            );
            (*pd.offset(count as isize)).ps = PLAY_STATE;
            port_n = 0 as libc::c_int;
            sscanf(
                buff.as_mut_ptr() as *const libc::c_char,
                b"%s %s %d %s %s\0" as *const u8 as *const libc::c_char,
                type_0.as_mut_ptr(),
                host.as_mut_ptr(),
                &mut port_n as *mut libc::c_int,
                ((*pd.offset(count as isize)).user).as_mut_ptr(),
                ((*pd.offset(count as isize)).pass).as_mut_ptr(),
            );
            tmp___0 = inet_addr(host.as_mut_ptr() as *const libc::c_char);
            (*pd.offset(count as isize)).ip.as_int = tmp___0;
            (*pd.offset(count as isize)).port = __bswap_16(port_n as libc::c_ushort);
            tmp___4 = strcmp(
                type_0.as_mut_ptr() as *const libc::c_char,
                b"http\0" as *const u8 as *const libc::c_char,
            );
            if tmp___4 != 0 {
                tmp___3 = strcmp(
                    type_0.as_mut_ptr() as *const libc::c_char,
                    b"raw\0" as *const u8 as *const libc::c_char,
                );
                if tmp___3 != 0 {
                    tmp___2 = strcmp(
                        type_0.as_mut_ptr() as *const libc::c_char,
                        b"socks4\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___2 != 0 {
                        tmp___1 = strcmp(
                            type_0.as_mut_ptr() as *const libc::c_char,
                            b"socks5\0" as *const u8 as *const libc::c_char,
                        );
                        if tmp___1 != 0 {
                            continue;
                        }
                        (*pd.offset(count as isize)).pt = SOCKS5_TYPE;
                    } else {
                        (*pd.offset(count as isize)).pt = SOCKS4_TYPE;
                    }
                } else {
                    (*pd.offset(count as isize)).pt = RAW_TYPE;
                }
            } else {
                (*pd.offset(count as isize)).pt = HTTP_TYPE;
            }
            if (*pd.offset(count as isize)).ip.as_int != 0 {
                if port_n != 0 {
                    if (*pd.offset(count as isize)).ip.as_int
                        != 4294967295 as libc::c_uint
                    {
                        count = count.wrapping_add(1);
                    }
                }
            }
        } else {
            tmp___23 = strstr(
                buff.as_mut_ptr() as *const libc::c_char,
                b"[ProxyList]\0" as *const u8 as *const libc::c_char,
            );
            if !tmp___23.is_null() {
                list___0 = 1 as libc::c_int;
            } else {
                tmp___22 = strstr(
                    buff.as_mut_ptr() as *const libc::c_char,
                    b"random_chain\0" as *const u8 as *const libc::c_char,
                );
                if !tmp___22.is_null() {
                    *ct = RANDOM_TYPE;
                } else {
                    tmp___21 = strstr(
                        buff.as_mut_ptr() as *const libc::c_char,
                        b"strict_chain\0" as *const u8 as *const libc::c_char,
                    );
                    if !tmp___21.is_null() {
                        *ct = STRICT_TYPE;
                    } else {
                        tmp___20 = strstr(
                            buff.as_mut_ptr() as *const libc::c_char,
                            b"dynamic_chain\0" as *const u8 as *const libc::c_char,
                        );
                        if !tmp___20.is_null() {
                            *ct = DYNAMIC_TYPE;
                        } else {
                            tmp___19 = strstr(
                                buff.as_mut_ptr() as *const libc::c_char,
                                b"tcp_read_time_out\0" as *const u8 as *const libc::c_char,
                            );
                            if !tmp___19.is_null() {
                                sscanf(
                                    buff.as_mut_ptr() as *const libc::c_char,
                                    b"%s %d\0" as *const u8 as *const libc::c_char,
                                    user.as_mut_ptr(),
                                    &mut tcp_read_time_out as *mut libc::c_int,
                                );
                            } else {
                                tmp___18 = strstr(
                                    buff.as_mut_ptr() as *const libc::c_char,
                                    b"tcp_connect_time_out\0" as *const u8
                                        as *const libc::c_char,
                                );
                                if !tmp___18.is_null() {
                                    sscanf(
                                        buff.as_mut_ptr() as *const libc::c_char,
                                        b"%s %d\0" as *const u8 as *const libc::c_char,
                                        user.as_mut_ptr(),
                                        &mut tcp_connect_time_out as *mut libc::c_int,
                                    );
                                } else {
                                    tmp___17 = strstr(
                                        buff.as_mut_ptr() as *const libc::c_char,
                                        b"remote_dns_subnet\0" as *const u8 as *const libc::c_char,
                                    );
                                    if !tmp___17.is_null() {
                                        sscanf(
                                            buff.as_mut_ptr() as *const libc::c_char,
                                            b"%s %d\0" as *const u8 as *const libc::c_char,
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
                                    } else {
                                        tmp___16 = strstr(
                                            buff.as_mut_ptr() as *const libc::c_char,
                                            b"localnet\0" as *const u8 as *const libc::c_char,
                                        );
                                        if !tmp___16.is_null() {
                                            tmp___5 = sscanf(
                                                buff.as_mut_ptr() as *const libc::c_char,
                                                b"%s %21[^/]/%15s\0" as *const u8 as *const libc::c_char,
                                                user.as_mut_ptr(),
                                                local_in_addr_port.as_mut_ptr(),
                                                local_netmask.as_mut_ptr(),
                                            );
                                            if tmp___5 < 3 as libc::c_int {
                                                fprintf(
                                                    stderr,
                                                    b"localnet format error\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                                exit(1 as libc::c_int);
                                            }
                                            memset(
                                                local_in_port.as_mut_ptr() as *mut libc::c_void,
                                                0 as libc::c_int,
                                                (::std::mem::size_of::<[libc::c_char; 32]>()
                                                    as libc::c_ulong)
                                                    .wrapping_div(
                                                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                                    ),
                                            );
                                            tmp___6 = sscanf(
                                                local_in_addr_port.as_mut_ptr() as *const libc::c_char,
                                                b"%15[^:]:%5s\0" as *const u8 as *const libc::c_char,
                                                local_in_addr.as_mut_ptr(),
                                                local_in_port.as_mut_ptr(),
                                            );
                                            tmp___6 < 2 as libc::c_int;
                                            if num_localnet_addr < 64 as libc::c_ulong {
                                                error = inet_pton(
                                                    2 as libc::c_int,
                                                    local_in_addr.as_mut_ptr() as *const libc::c_char,
                                                    &mut (*localnet_addr
                                                        .as_mut_ptr()
                                                        .offset(num_localnet_addr as isize))
                                                        .in_addr as *mut in_addr as *mut libc::c_void,
                                                );
                                                if error <= 0 as libc::c_int {
                                                    fprintf(
                                                        stderr,
                                                        b"localnet address error\n\0" as *const u8
                                                            as *const libc::c_char,
                                                    );
                                                    exit(1 as libc::c_int);
                                                }
                                                error = inet_pton(
                                                    2 as libc::c_int,
                                                    local_netmask.as_mut_ptr() as *const libc::c_char,
                                                    &mut (*localnet_addr
                                                        .as_mut_ptr()
                                                        .offset(num_localnet_addr as isize))
                                                        .netmask as *mut in_addr as *mut libc::c_void,
                                                );
                                                if error <= 0 as libc::c_int {
                                                    fprintf(
                                                        stderr,
                                                        b"localnet netmask error\n\0" as *const u8
                                                            as *const libc::c_char,
                                                    );
                                                    exit(1 as libc::c_int);
                                                }
                                                if local_in_port[0 as libc::c_int as usize] != 0 {
                                                    tmp___7 = atoi(
                                                        local_in_port.as_mut_ptr() as *const libc::c_char,
                                                    );
                                                    localnet_addr[num_localnet_addr as usize]
                                                        .port = tmp___7 as libc::c_ushort;
                                                } else {
                                                    localnet_addr[num_localnet_addr as usize]
                                                        .port = 0 as libc::c_int as libc::c_ushort;
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
                                        } else {
                                            tmp___15 = strstr(
                                                buff.as_mut_ptr() as *const libc::c_char,
                                                b"chain_len\0" as *const u8 as *const libc::c_char,
                                            );
                                            if !tmp___15.is_null() {
                                                pc = strchr(
                                                    buff.as_mut_ptr() as *const libc::c_char,
                                                    '=' as i32,
                                                );
                                                pc = pc.offset(1);
                                                len = atoi(pc as *const libc::c_char);
                                                if len != 0 {
                                                    tmp___8 = len;
                                                } else {
                                                    tmp___8 = 1 as libc::c_int;
                                                }
                                                proxychains_max_chain = tmp___8 as libc::c_uint;
                                            } else {
                                                tmp___14 = strstr(
                                                    buff.as_mut_ptr() as *const libc::c_char,
                                                    b"quiet_mode\0" as *const u8 as *const libc::c_char,
                                                );
                                                if !tmp___14.is_null() {
                                                    proxychains_quiet_mode = 1 as libc::c_int;
                                                } else {
                                                    tmp___13 = strstr(
                                                        buff.as_mut_ptr() as *const libc::c_char,
                                                        b"proxy_dns\0" as *const u8 as *const libc::c_char,
                                                    );
                                                    if !tmp___13.is_null() {
                                                        proxychains_resolver = 1 as libc::c_int;
                                                    } else {
                                                        tmp___12 = strstr(
                                                            buff.as_mut_ptr() as *const libc::c_char,
                                                            b"dnat\0" as *const u8 as *const libc::c_char,
                                                        );
                                                        if !tmp___12.is_null() {
                                                            tmp___9 = sscanf(
                                                                buff.as_mut_ptr() as *const libc::c_char,
                                                                b"%s %21[^ ] %21s\n\0" as *const u8 as *const libc::c_char,
                                                                user.as_mut_ptr(),
                                                                dnat_orig_addr_port.as_mut_ptr(),
                                                                dnat_new_addr_port.as_mut_ptr(),
                                                            );
                                                            if tmp___9 < 3 as libc::c_int {
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
                                                                    tmp___10 = atoi(
                                                                        dnat_orig_port.as_mut_ptr() as *const libc::c_char,
                                                                    );
                                                                    dnats[num_dnats as usize]
                                                                        .orig_port = tmp___10 as libc::c_ushort;
                                                                } else {
                                                                    dnats[num_dnats as usize]
                                                                        .orig_port = 0 as libc::c_int as libc::c_ushort;
                                                                }
                                                                if dnat_new_port[0 as libc::c_int as usize] != 0 {
                                                                    tmp___11 = atoi(
                                                                        dnat_new_port.as_mut_ptr() as *const libc::c_char,
                                                                    );
                                                                    dnats[num_dnats as usize]
                                                                        .new_port = tmp___11 as libc::c_ushort;
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
    fclose(file);
    *proxy_count = count;
    proxychains_got_chain_data = 1 as libc::c_int;
}
unsafe extern "C" fn simple_socks5_env(
    mut pd: *mut proxy_data,
    mut proxy_count: *mut libc::c_uint,
    mut ct: *mut chain_type,
) {
    let mut port_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut host_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: in_addr_t = 0;
    let mut tmp___0: libc::c_long = 0;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    if proxychains_got_chain_data != 0 {
        return;
    }
    load_default_settings(ct);
    port_string = getenv(
        b"PROXYCHAINS_SOCKS5_PORT\0" as *const u8 as *const libc::c_char,
    );
    if port_string.is_null() {
        return;
    }
    host_string = getenv(
        b"PROXYCHAINS_SOCKS5_HOST\0" as *const u8 as *const libc::c_char,
    );
    if host_string.is_null() {
        host_string = b"127.0.0.1\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    memset(
        pd as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<proxy_data>() as libc::c_ulong,
    );
    (*pd.offset(0 as libc::c_int as isize)).ps = PLAY_STATE;
    tmp = inet_addr(host_string as *const libc::c_char);
    (*pd.offset(0 as libc::c_int as isize)).ip.as_int = tmp;
    tmp___0 = strtol(
        port_string as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        0 as libc::c_int,
    );
    (*pd.offset(0 as libc::c_int as isize)).port = __bswap_16(tmp___0 as libc::c_ushort);
    (*pd.offset(0 as libc::c_int as isize)).pt = SOCKS5_TYPE;
    proxychains_max_chain = 1 as libc::c_uint;
    tmp___1 = getenv(b"PROXYCHAINS_DNS\0" as *const u8 as *const libc::c_char);
    if !tmp___1.is_null() {
        proxychains_resolver = 1 as libc::c_int;
    }
    *proxy_count = 1 as libc::c_uint;
    proxychains_got_chain_data = 1 as libc::c_int;
}
pub unsafe extern "C" fn connect(
    mut sock: libc::c_int,
    mut addr: *const sockaddr,
    mut len: socklen_t,
) -> libc::c_int {
    let mut socktype: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut optlen: socklen_t = 0;
    let mut dest_ip: ip_type = __anonunion_ip_type_826858479 {
        octet: [0; 4],
    };
    let mut p_addr_in: *mut in_addr = 0 as *mut in_addr;
    let mut new_addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut dnat: *mut dnat_arg = 0 as *mut dnat_arg;
    let mut port: libc::c_ushort = 0;
    let mut i: size_t = 0;
    let mut remote_dns_connect: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: __uint32_t = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    socktype = 0 as libc::c_int;
    flags = 0 as libc::c_int;
    ret = 0 as libc::c_int;
    optlen = 0 as libc::c_int as socklen_t;
    dnat = 0 as *mut libc::c_void as *mut dnat_arg;
    remote_dns_connect = 0 as libc::c_int;
    init_lib_wrapper(b"connect\0" as *const u8 as *const libc::c_char);
    optlen = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
    getsockopt(
        sock,
        1 as libc::c_int,
        3 as libc::c_int,
        &mut socktype as *mut libc::c_int as *mut libc::c_void,
        &mut optlen as *mut socklen_t,
    );
    if (*(addr as *mut sockaddr_in)).sin_family as libc::c_int == 2 as libc::c_int {
        if !(socktype == 1 as libc::c_int) {
            tmp = (Some(true_connect.expect("non-null function pointer")))
                .expect("non-null function pointer")(sock, addr, len);
            return tmp;
        }
    } else {
        tmp = (Some(true_connect.expect("non-null function pointer")))
            .expect("non-null function pointer")(sock, addr, len);
        return tmp;
    }
    p_addr_in = &mut (*(addr as *mut sockaddr_in)).sin_addr;
    port = __bswap_16((*(addr as *mut sockaddr_in)).sin_port);
    tmp___0 = __bswap_32((*p_addr_in).s_addr);
    remote_dns_connect = (tmp___0 >> 24 as libc::c_int == remote_dns_subnet)
        as libc::c_int;
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
                if dnats[i as usize].orig_port as libc::c_int == port as libc::c_int {
                    dnat = &mut *dnats.as_mut_ptr().offset(i as isize) as *mut dnat_arg;
                }
            }
        }
        i = i.wrapping_add(1);
    }
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
    if !dnat.is_null() {
        if (*dnat).new_port != 0 {
            new_addr.sin_port = __bswap_16((*dnat).new_port);
        } else {
            new_addr.sin_port = __bswap_16(port);
        }
        new_addr.sin_addr = (*dnat).new_dst;
        addr = &mut new_addr as *mut sockaddr_in as *mut sockaddr as *const sockaddr;
        p_addr_in = &mut (*(addr as *mut sockaddr_in)).sin_addr;
        port = __bswap_16((*(addr as *mut sockaddr_in)).sin_port);
    }
    i = 0 as libc::c_int as size_t;
    while i < num_localnet_addr {
        if remote_dns_connect != 0 {
            break;
        }
        if localnet_addr[i as usize].in_addr.s_addr
            & localnet_addr[i as usize].netmask.s_addr
            == (*p_addr_in).s_addr & localnet_addr[i as usize].netmask.s_addr
        {
            's_339: {
                if !(localnet_addr[i as usize].port == 0) {
                    if !(localnet_addr[i as usize].port as libc::c_int
                        == port as libc::c_int)
                    {
                        break 's_339;
                    }
                }
                tmp___1 = (Some(true_connect.expect("non-null function pointer")))
                    .expect("non-null function pointer")(sock, addr, len);
                return tmp___1;
            }
        }
        i = i.wrapping_add(1);
    }
    flags = fcntl(sock, 3 as libc::c_int, 0 as libc::c_int);
    if flags & 2048 as libc::c_int != 0 {
        fcntl(sock, 4 as libc::c_int, 0 as libc::c_int);
    }
    dest_ip.as_int = (*(addr as *mut sockaddr_in)).sin_addr.s_addr;
    ret = connect_proxy_chain(
        sock,
        dest_ip,
        (*(addr as *mut sockaddr_in)).sin_port,
        proxychains_pd.as_mut_ptr(),
        proxychains_proxy_count,
        proxychains_ct,
        proxychains_max_chain,
    );
    fcntl(sock, 4 as libc::c_int, flags);
    if ret != 0 as libc::c_int {
        tmp___2 = __errno_location();
        *tmp___2 = 111 as libc::c_int;
    }
    return ret;
}
static mut ghbndata: gethostbyname_data = gethostbyname_data {
    hostent_space: hostent {
        h_name: 0 as *mut libc::c_char,
        h_aliases: 0 as *mut *mut libc::c_char,
        h_addrtype: 0,
        h_length: 0,
        h_addr_list: 0 as *mut *mut libc::c_char,
    },
    resolved_addr: 0,
    resolved_addr_p: [0 as *mut libc::c_char; 2],
    addr_name: [0; 8192],
};
pub unsafe extern "C" fn gethostbyname(mut name: *const libc::c_char) -> *mut hostent {
    let mut tmp: *mut hostent = 0 as *mut hostent;
    let mut tmp___0: *mut hostent = 0 as *mut hostent;
    init_lib_wrapper(b"gethostbyname\0" as *const u8 as *const libc::c_char);
    if proxychains_resolver != 0 {
        tmp = proxy_gethostbyname(name, &mut ghbndata);
        return tmp;
    } else {
        tmp___0 = (Some(true_gethostbyname.expect("non-null function pointer")))
            .expect("non-null function pointer")(name);
        return tmp___0;
    };
}
pub unsafe extern "C" fn getaddrinfo(
    mut node: *const libc::c_char,
    mut service: *const libc::c_char,
    mut hints: *const addrinfo,
    mut res: *mut *mut addrinfo,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = 0 as libc::c_int;
    init_lib_wrapper(b"getaddrinfo\0" as *const u8 as *const libc::c_char);
    if proxychains_resolver != 0 {
        ret = proxy_getaddrinfo(node, service, hints, res);
    } else {
        ret = (Some(true_getaddrinfo.expect("non-null function pointer")))
            .expect("non-null function pointer")(node, service, hints, res);
    }
    return ret;
}
pub unsafe extern "C" fn freeaddrinfo(mut res: *mut addrinfo) {
    init_lib_wrapper(b"freeaddrinfo\0" as *const u8 as *const libc::c_char);
    if proxychains_resolver == 0 {
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
    let mut ip_buf: [libc::c_char; 16] = [0; 16];
    let mut ret: libc::c_int = 0;
    let mut tmp: __uint16_t = 0;
    ret = 0 as libc::c_int;
    init_lib_wrapper(b"getnameinfo\0" as *const u8 as *const libc::c_char);
    if proxychains_resolver == 0 {
        ret = (Some(true_getnameinfo.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(sa, salen, host, hostlen, serv, servlen, flags);
    } else {
        if hostlen != 0 {
            inet_ntop(
                2 as libc::c_int,
                &mut (*(sa as *mut sockaddr_in)).sin_addr as *mut in_addr
                    as *mut libc::c_uchar as *const libc::c_void,
                ip_buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as socklen_t,
            );
            strncpy(host, ip_buf.as_mut_ptr() as *const libc::c_char, hostlen as size_t);
        }
        if servlen != 0 {
            tmp = __bswap_16((*(sa as *mut sockaddr_in)).sin_port);
            snprintf(
                serv,
                servlen as size_t,
                b"%d\0" as *const u8 as *const libc::c_char,
                tmp as libc::c_int,
            );
        }
    }
    return ret;
}
static mut buf: [libc::c_char; 16] = [0; 16];
static mut ipv4: [libc::c_char; 4] = [0; 4];
static mut list: [*mut libc::c_char; 2] = [0 as *const libc::c_char
    as *mut libc::c_char; 2];
static mut he: hostent = hostent {
    h_name: 0 as *mut libc::c_char,
    h_aliases: 0 as *mut *mut libc::c_char,
    h_addrtype: 0,
    h_length: 0,
    h_addr_list: 0 as *mut *mut libc::c_char,
};
pub unsafe extern "C" fn gethostbyaddr(
    mut addr: *const libc::c_void,
    mut len: __socklen_t,
    mut type_0: libc::c_int,
) -> *mut hostent {
    let mut tmp: *mut hostent = 0 as *mut hostent;
    init_lib_wrapper(b"gethostbyaddr\0" as *const u8 as *const libc::c_char);
    if proxychains_resolver == 0 {
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
        he.h_aliases = 0 as *mut libc::c_void as *mut *mut libc::c_char;
        he.h_length = 4 as libc::c_int;
        inet_ntop(
            2 as libc::c_int,
            addr,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as socklen_t,
        );
        return &mut he;
    };
}
