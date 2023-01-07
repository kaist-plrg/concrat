use ::libc;
use ::c2rust_bitfields;
use ::f128;
use ::num_traits;
use num_traits::ToPrimitive;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ssl_st;
    pub type ssl_ctx_st;
    pub type lua_State;
    pub type x509_store_ctx_st;
    pub type ossl_init_settings_st;
    pub type ssl_method_st;
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
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
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
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn SSL_new(ctx: *mut SSL_CTX) -> *mut SSL;
    fn __errno_location() -> *mut libc::c_int;
    fn ERR_print_errors_fp(fp: *mut FILE);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
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
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn signal(
        __sig: libc::c_int,
        __handler: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> __sighandler_t;
    fn sigfillset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn OPENSSL_init_crypto(
        opts: uint64_t,
        settings: *const OPENSSL_INIT_SETTINGS,
    ) -> libc::c_int;
    fn SSL_CTX_new(meth: *const SSL_METHOD) -> *mut SSL_CTX;
    fn SSL_clear(s: *mut SSL) -> libc::c_int;
    fn SSL_pending(s: *const SSL) -> libc::c_int;
    fn SSL_set_fd(s: *mut SSL, fd: libc::c_int) -> libc::c_int;
    fn SSL_CTX_set_verify(
        ctx: *mut SSL_CTX,
        mode: libc::c_int,
        callback: Option::<
            unsafe extern "C" fn(libc::c_int, *mut X509_STORE_CTX) -> libc::c_int,
        >,
    );
    fn SSL_CTX_set_verify_depth(ctx: *mut SSL_CTX, depth: libc::c_int);
    fn SSL_connect(ssl: *mut SSL) -> libc::c_int;
    fn SSL_read(ssl: *mut SSL, buf: *mut libc::c_void, num: libc::c_int) -> libc::c_int;
    fn SSL_write(
        ssl: *mut SSL,
        buf: *const libc::c_void,
        num: libc::c_int,
    ) -> libc::c_int;
    fn SSL_ctrl(
        ssl: *mut SSL,
        cmd: libc::c_int,
        larg: libc::c_long,
        parg: *mut libc::c_void,
    ) -> libc::c_long;
    fn SSL_CTX_ctrl(
        ctx: *mut SSL_CTX,
        cmd: libc::c_int,
        larg: libc::c_long,
        parg: *mut libc::c_void,
    ) -> libc::c_long;
    fn SSL_get_error(s: *const SSL, ret_code: libc::c_int) -> libc::c_int;
    fn TLS_client_method() -> *const SSL_METHOD;
    fn SSL_shutdown(s: *mut SSL) -> libc::c_int;
    fn OPENSSL_init_ssl(
        opts: uint64_t,
        settings: *const OPENSSL_INIT_SETTINGS,
    ) -> libc::c_int;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn round(_: libc::c_double) -> libc::c_double;
    fn powl(_: f128::f128, _: f128::f128) -> f128::f128;
    fn sqrtl(_: f128::f128) -> f128::f128;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn lua_settop(L: *mut lua_State, idx: libc::c_int);
    fn lua_pushvalue(L: *mut lua_State, idx: libc::c_int);
    fn lua_type(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_typename(L: *mut lua_State, tp: libc::c_int) -> *const libc::c_char;
    fn lua_tonumber(L: *mut lua_State, idx: libc::c_int) -> lua_Number;
    fn lua_toboolean(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_tolstring(
        L: *mut lua_State,
        idx: libc::c_int,
        len: *mut size_t,
    ) -> *const libc::c_char;
    fn lua_objlen(L: *mut lua_State, idx: libc::c_int) -> size_t;
    fn lua_pushnil(L: *mut lua_State);
    fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
    fn lua_pushlstring(L: *mut lua_State, s: *const libc::c_char, l: size_t);
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char);
    fn lua_pushfstring(
        L: *mut lua_State,
        fmt: *const libc::c_char,
        _: ...
    ) -> *const libc::c_char;
    fn lua_pushcclosure(
        L: *mut lua_State,
        fn_0: Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>,
        n: libc::c_int,
    );
    fn lua_pushboolean(L: *mut lua_State, b: libc::c_int);
    fn lua_getfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char);
    fn lua_createtable(L: *mut lua_State, narr: libc::c_int, nrec: libc::c_int);
    fn lua_newuserdata(L: *mut lua_State, sz: size_t) -> *mut libc::c_void;
    fn lua_settable(L: *mut lua_State, idx: libc::c_int);
    fn lua_setfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char);
    fn lua_rawset(L: *mut lua_State, idx: libc::c_int);
    fn lua_rawseti(L: *mut lua_State, idx: libc::c_int, n: libc::c_int);
    fn lua_setmetatable(L: *mut lua_State, objindex: libc::c_int) -> libc::c_int;
    fn lua_call(L: *mut lua_State, nargs: libc::c_int, nresults: libc::c_int);
    fn lua_pcall(
        L: *mut lua_State,
        nargs: libc::c_int,
        nresults: libc::c_int,
        errfunc: libc::c_int,
    ) -> libc::c_int;
    fn lua_next(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn luaL_openlibs(L: *mut lua_State);
    fn luaL_register(
        L: *mut lua_State,
        libname: *const libc::c_char,
        l: *const luaL_Reg,
    );
    fn luaL_argerror(
        L: *mut lua_State,
        numarg: libc::c_int,
        extramsg: *const libc::c_char,
    ) -> libc::c_int;
    fn luaL_checknumber(L: *mut lua_State, numArg: libc::c_int) -> lua_Number;
    fn luaL_newmetatable(L: *mut lua_State, tname: *const libc::c_char) -> libc::c_int;
    fn luaL_checkudata(
        L: *mut lua_State,
        ud: libc::c_int,
        tname: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn luaL_loadfile(L: *mut lua_State, filename: *const libc::c_char) -> libc::c_int;
    fn luaL_loadstring(L: *mut lua_State, s: *const libc::c_char) -> libc::c_int;
    fn luaL_newstate() -> *mut lua_State;
    fn luaL_pushresult(B: *mut luaL_Buffer);
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn getnameinfo(
        __sa: *const sockaddr,
        __salen: socklen_t,
        __host: *mut libc::c_char,
        __hostlen: socklen_t,
        __serv: *mut libc::c_char,
        __servlen: socklen_t,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
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
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn abort() -> !;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type __sig_atomic_t = libc::c_int;
pub type size_t = libc::c_ulong;
pub type time_t = __time_t;
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type int64_t = __int64_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub type SSL = ssl_st;
pub type SSL_CTX = ssl_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_errors_1007178897 {
    pub connect: uint32_t,
    pub read: uint32_t,
    pub write: uint32_t,
    pub status: uint32_t,
    pub timeout: uint32_t,
}
pub type errors = __anonstruct_errors_1007178897;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_stats_804253929 {
    pub count: uint64_t,
    pub limit: uint64_t,
    pub min: uint64_t,
    pub max: uint64_t,
    pub data: [uint64_t; 0],
}
pub type stats = __anonstruct_stats_804253929;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeEventLoop {
    pub maxfd: libc::c_int,
    pub setsize: libc::c_int,
    pub timeEventNextId: libc::c_longlong,
    pub lastTime: time_t,
    pub events: *mut aeFileEvent,
    pub fired: *mut aeFiredEvent,
    pub timeEventHead: *mut aeTimeEvent,
    pub stop: libc::c_int,
    pub apidata: *mut libc::c_void,
    pub beforesleep: Option::<aeBeforeSleepProc>,
}
pub type aeBeforeSleepProc = unsafe extern "C" fn(*mut aeEventLoop) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeTimeEvent {
    pub id: libc::c_longlong,
    pub when_sec: libc::c_long,
    pub when_ms: libc::c_long,
    pub timeProc: Option::<aeTimeProc>,
    pub finalizerProc: Option::<aeEventFinalizerProc>,
    pub clientData: *mut libc::c_void,
    pub next: *mut aeTimeEvent,
}
pub type aeEventFinalizerProc = unsafe extern "C" fn(
    *mut aeEventLoop,
    *mut libc::c_void,
) -> ();
pub type aeTimeProc = unsafe extern "C" fn(
    *mut aeEventLoop,
    libc::c_longlong,
    *mut libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeFiredEvent {
    pub fd: libc::c_int,
    pub mask: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeFileEvent {
    pub mask: libc::c_int,
    pub rfileProc: Option::<aeFileProc>,
    pub wfileProc: Option::<aeFileProc>,
    pub clientData: *mut libc::c_void,
}
pub type aeFileProc = unsafe extern "C" fn(
    *mut aeEventLoop,
    libc::c_int,
    *mut libc::c_void,
    libc::c_int,
) -> ();
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct http_parser {
    #[bitfield(name = "type_0", ty = "libc::c_uint", bits = "0..=1")]
    #[bitfield(name = "flags", ty = "libc::c_uint", bits = "2..=9")]
    #[bitfield(name = "state", ty = "libc::c_uint", bits = "10..=16")]
    #[bitfield(name = "header_state", ty = "libc::c_uint", bits = "17..=23")]
    #[bitfield(name = "index", ty = "libc::c_uint", bits = "24..=30")]
    #[bitfield(name = "lenient_http_headers", ty = "libc::c_uint", bits = "31..=31")]
    pub type_0_flags_state_header_state_index_lenient_http_headers: [u8; 4],
    pub nread: uint32_t,
    pub content_length: uint64_t,
    pub http_major: libc::c_ushort,
    pub http_minor: libc::c_ushort,
    #[bitfield(name = "status_code", ty = "libc::c_uint", bits = "0..=15")]
    #[bitfield(name = "method", ty = "libc::c_uint", bits = "16..=23")]
    #[bitfield(name = "http_errno", ty = "libc::c_uint", bits = "24..=30")]
    #[bitfield(name = "upgrade", ty = "libc::c_uint", bits = "31..=31")]
    pub status_code_method_http_errno_upgrade: [u8; 4],
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_parser_settings {
    pub on_message_begin: Option::<
        unsafe extern "C" fn(*mut http_parser) -> libc::c_int,
    >,
    pub on_url: Option::<
        unsafe extern "C" fn(
            *mut http_parser,
            *const libc::c_char,
            size_t,
        ) -> libc::c_int,
    >,
    pub on_status: Option::<
        unsafe extern "C" fn(
            *mut http_parser,
            *const libc::c_char,
            size_t,
        ) -> libc::c_int,
    >,
    pub on_header_field: Option::<
        unsafe extern "C" fn(
            *mut http_parser,
            *const libc::c_char,
            size_t,
        ) -> libc::c_int,
    >,
    pub on_header_value: Option::<
        unsafe extern "C" fn(
            *mut http_parser,
            *const libc::c_char,
            size_t,
        ) -> libc::c_int,
    >,
    pub on_headers_complete: Option::<
        unsafe extern "C" fn(*mut http_parser) -> libc::c_int,
    >,
    pub on_body: Option::<
        unsafe extern "C" fn(
            *mut http_parser,
            *const libc::c_char,
            size_t,
        ) -> libc::c_int,
    >,
    pub on_message_complete: Option::<
        unsafe extern "C" fn(*mut http_parser) -> libc::c_int,
    >,
    pub on_chunk_header: Option::<unsafe extern "C" fn(*mut http_parser) -> libc::c_int>,
    pub on_chunk_complete: Option::<
        unsafe extern "C" fn(*mut http_parser) -> libc::c_int,
    >,
}
pub type http_parser_type = libc::c_uint;
pub const HTTP_BOTH: http_parser_type = 2;
pub const HTTP_RESPONSE: http_parser_type = 1;
pub const HTTP_REQUEST: http_parser_type = 0;
pub type http_parser_url_fields = libc::c_uint;
pub const UF_MAX: http_parser_url_fields = 7;
pub const UF_USERINFO: http_parser_url_fields = 6;
pub const UF_FRAGMENT: http_parser_url_fields = 5;
pub const UF_QUERY: http_parser_url_fields = 4;
pub const UF_PATH: http_parser_url_fields = 3;
pub const UF_PORT: http_parser_url_fields = 2;
pub const UF_HOST: http_parser_url_fields = 1;
pub const UF_SCHEMA: http_parser_url_fields = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_field_data_753912357 {
    pub off: uint16_t,
    pub len: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_parser_url {
    pub field_set: uint16_t,
    pub port: uint16_t,
    pub field_data: [__anonstruct_field_data_753912357; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct connection {
    pub thread: *mut thread,
    pub parser: http_parser,
    pub state: __anonenum_state_47808738,
    pub fd: libc::c_int,
    pub ssl: *mut SSL,
    pub delayed: bool,
    pub start: uint64_t,
    pub request: *mut libc::c_char,
    pub length: size_t,
    pub written: size_t,
    pub pending: uint64_t,
    pub headers: buffer,
    pub body: buffer,
    pub buf: [libc::c_char; 8192],
}
pub type buffer = __anonstruct_buffer_827665108;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_buffer_827665108 {
    pub buffer: *mut libc::c_char,
    pub length: size_t,
    pub cursor: *mut libc::c_char,
}
pub type __anonenum_state_47808738 = libc::c_uint;
pub const VALUE: __anonenum_state_47808738 = 1;
pub const FIELD: __anonenum_state_47808738 = 0;
pub type thread = __anonstruct_thread_238483422;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_thread_238483422 {
    pub thread: pthread_t,
    pub loop_0: *mut aeEventLoop,
    pub addr: *mut addrinfo,
    pub connections: uint64_t,
    pub complete: uint64_t,
    pub requests: uint64_t,
    pub bytes: uint64_t,
    pub start: uint64_t,
    pub L: *mut lua_State,
    pub errors: errors,
    pub cs: *mut connection,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type sig_atomic_t = __sig_atomic_t;
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
pub union __anonunion__bounds_341140457 {
    pub _addr_bnd: __anonstruct__addr_bnd_5259977,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct__sigfault_496270274 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: __anonunion__bounds_341140457,
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
pub union __anonunion__sifields_534194718 {
    pub _pad: [libc::c_int; 28],
    pub _kill: __anonstruct__kill_244518854,
    pub _timer: __anonstruct__timer_490064738,
    pub _rt: __anonstruct__rt_619254530,
    pub _sigchld: __anonstruct__sigchld_284671705,
    pub _sigfault: __anonstruct__sigfault_496270274,
    pub _sigpoll: __anonstruct__sigpoll_386613454,
    pub _sigsys: __anonstruct__sigsys_44812255,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_siginfo_t_981507777 {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: __anonunion__sifields_534194718,
}
pub type siginfo_t = __anonstruct_siginfo_t_981507777;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
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
pub type __anonenum_status_995329011 = libc::c_uint;
pub const RETRY: __anonenum_status_995329011 = 2;
pub const ERROR: __anonenum_status_995329011 = 1;
pub const OK: __anonenum_status_995329011 = 0;
pub type status = __anonenum_status_995329011;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sock {
    pub connect: Option::<
        unsafe extern "C" fn(*mut connection, *mut libc::c_char) -> status,
    >,
    pub close: Option::<unsafe extern "C" fn(*mut connection) -> status>,
    pub read: Option::<unsafe extern "C" fn(*mut connection, *mut size_t) -> status>,
    pub write: Option::<
        unsafe extern "C" fn(
            *mut connection,
            *mut libc::c_char,
            size_t,
            *mut size_t,
        ) -> status,
    >,
    pub readable: Option::<unsafe extern "C" fn(*mut connection) -> size_t>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config {
    pub connections: uint64_t,
    pub duration: uint64_t,
    pub threads: uint64_t,
    pub timeout: uint64_t,
    pub pipeline: uint64_t,
    pub delay: bool,
    pub dynamic: bool,
    pub latency: bool,
    pub host: *mut libc::c_char,
    pub script: *mut libc::c_char,
    pub ctx: *mut SSL_CTX,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_statistics_580573181 {
    pub latency: *mut stats,
    pub requests: *mut stats,
}
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type X509_STORE_CTX = x509_store_ctx_st;
pub type OPENSSL_INIT_SETTINGS = ossl_init_settings_st;
pub type SSL_METHOD = ssl_method_st;
pub type va_list___0 = __gnuc_va_list;
pub type ptrdiff_t = libc::c_long;
pub type lua_Number = libc::c_double;
pub type lua_Integer = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Buffer {
    pub p: *mut libc::c_char,
    pub lvl: libc::c_int,
    pub L: *mut lua_State,
    pub buffer: [libc::c_char; 8192],
}
pub type http_errno = libc::c_uint;
pub const HPE_UNKNOWN: http_errno = 32;
pub const HPE_PAUSED: http_errno = 31;
pub const HPE_STRICT: http_errno = 30;
pub const HPE_INVALID_INTERNAL_STATE: http_errno = 29;
pub const HPE_INVALID_CONSTANT: http_errno = 28;
pub const HPE_INVALID_CHUNK_SIZE: http_errno = 27;
pub const HPE_UNEXPECTED_CONTENT_LENGTH: http_errno = 26;
pub const HPE_INVALID_CONTENT_LENGTH: http_errno = 25;
pub const HPE_INVALID_HEADER_TOKEN: http_errno = 24;
pub const HPE_LF_EXPECTED: http_errno = 23;
pub const HPE_INVALID_FRAGMENT: http_errno = 22;
pub const HPE_INVALID_QUERY_STRING: http_errno = 21;
pub const HPE_INVALID_PATH: http_errno = 20;
pub const HPE_INVALID_PORT: http_errno = 19;
pub const HPE_INVALID_HOST: http_errno = 18;
pub const HPE_INVALID_URL: http_errno = 17;
pub const HPE_INVALID_METHOD: http_errno = 16;
pub const HPE_INVALID_STATUS: http_errno = 15;
pub const HPE_INVALID_VERSION: http_errno = 14;
pub const HPE_CLOSED_CONNECTION: http_errno = 13;
pub const HPE_HEADER_OVERFLOW: http_errno = 12;
pub const HPE_INVALID_EOF_STATE: http_errno = 11;
pub const HPE_CB_chunk_complete: http_errno = 10;
pub const HPE_CB_chunk_header: http_errno = 9;
pub const HPE_CB_status: http_errno = 8;
pub const HPE_CB_message_complete: http_errno = 7;
pub const HPE_CB_body: http_errno = 6;
pub const HPE_CB_headers_complete: http_errno = 5;
pub const HPE_CB_header_value: http_errno = 4;
pub const HPE_CB_header_field: http_errno = 3;
pub const HPE_CB_url: http_errno = 2;
pub const HPE_CB_message_begin: http_errno = 1;
pub const HPE_OK: http_errno = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_table_field_66454024 {
    pub name: *mut libc::c_char,
    pub type_0: libc::c_int,
    pub value: *mut libc::c_void,
}
pub type table_field = __anonstruct_table_field_66454024;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_units_255056401 {
    pub scale: libc::c_int,
    pub base: *mut libc::c_char,
    pub units: [*mut libc::c_char; 0],
}
pub type units = __anonstruct_units_255056401;
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
pub struct aeApiState {
    pub epfd: libc::c_int,
    pub events: *mut epoll_event,
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
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type int8_t = __int8_t;
pub type uint8_t = __uint8_t;
pub type http_method = libc::c_uint;
pub const HTTP_UNLINK: http_method = 32;
pub const HTTP_LINK: http_method = 31;
pub const HTTP_MKCALENDAR: http_method = 30;
pub const HTTP_PURGE: http_method = 29;
pub const HTTP_PATCH: http_method = 28;
pub const HTTP_UNSUBSCRIBE: http_method = 27;
pub const HTTP_SUBSCRIBE: http_method = 26;
pub const HTTP_NOTIFY: http_method = 25;
pub const HTTP_MSEARCH: http_method = 24;
pub const HTTP_MERGE: http_method = 23;
pub const HTTP_CHECKOUT: http_method = 22;
pub const HTTP_MKACTIVITY: http_method = 21;
pub const HTTP_REPORT: http_method = 20;
pub const HTTP_ACL: http_method = 19;
pub const HTTP_UNBIND: http_method = 18;
pub const HTTP_REBIND: http_method = 17;
pub const HTTP_BIND: http_method = 16;
pub const HTTP_UNLOCK: http_method = 15;
pub const HTTP_SEARCH: http_method = 14;
pub const HTTP_PROPPATCH: http_method = 13;
pub const HTTP_PROPFIND: http_method = 12;
pub const HTTP_MOVE: http_method = 11;
pub const HTTP_MKCOL: http_method = 10;
pub const HTTP_LOCK: http_method = 9;
pub const HTTP_COPY: http_method = 8;
pub const HTTP_TRACE: http_method = 7;
pub const HTTP_OPTIONS: http_method = 6;
pub const HTTP_CONNECT: http_method = 5;
pub const HTTP_PUT: http_method = 4;
pub const HTTP_POST: http_method = 3;
pub const HTTP_HEAD: http_method = 2;
pub const HTTP_GET: http_method = 1;
pub const HTTP_DELETE: http_method = 0;
pub type state = libc::c_uint;
pub const s_message_done: state = 62;
pub const s_body_identity_eof: state = 61;
pub const s_body_identity: state = 60;
pub const s_chunk_data_done: state = 59;
pub const s_chunk_data_almost_done: state = 58;
pub const s_chunk_data: state = 57;
pub const s_headers_done: state = 56;
pub const s_headers_almost_done: state = 55;
pub const s_chunk_size_almost_done: state = 54;
pub const s_chunk_parameters: state = 53;
pub const s_chunk_size: state = 52;
pub const s_chunk_size_start: state = 51;
pub const s_header_almost_done: state = 50;
pub const s_header_value_lws: state = 49;
pub const s_header_value: state = 48;
pub const s_header_value_start: state = 47;
pub const s_header_value_discard_lws: state = 46;
pub const s_header_value_discard_ws_almost_done: state = 45;
pub const s_header_value_discard_ws: state = 44;
pub const s_header_field: state = 43;
pub const s_header_field_start: state = 42;
pub const s_req_line_almost_done: state = 41;
pub const s_req_http_end: state = 40;
pub const s_req_http_minor: state = 39;
pub const s_req_http_dot: state = 38;
pub const s_req_http_major: state = 37;
pub const s_req_http_HTTP: state = 36;
pub const s_req_http_HTT: state = 35;
pub const s_req_http_HT: state = 34;
pub const s_req_http_H: state = 33;
pub const s_req_http_start: state = 32;
pub const s_req_fragment: state = 31;
pub const s_req_fragment_start: state = 30;
pub const s_req_query_string: state = 29;
pub const s_req_query_string_start: state = 28;
pub const s_req_path: state = 27;
pub const s_req_server_with_at: state = 26;
pub const s_req_server: state = 25;
pub const s_req_server_start: state = 24;
pub const s_req_schema_slash_slash: state = 23;
pub const s_req_schema_slash: state = 22;
pub const s_req_schema: state = 21;
pub const s_req_spaces_before_url: state = 20;
pub const s_req_method: state = 19;
pub const s_start_req: state = 18;
pub const s_res_line_almost_done: state = 17;
pub const s_res_status: state = 16;
pub const s_res_status_start: state = 15;
pub const s_res_status_code: state = 14;
pub const s_res_first_status_code: state = 13;
pub const s_res_http_end: state = 12;
pub const s_res_http_minor: state = 11;
pub const s_res_http_dot: state = 10;
pub const s_res_http_major: state = 9;
pub const s_res_HTTP: state = 8;
pub const s_res_HTT: state = 7;
pub const s_res_HT: state = 6;
pub const s_res_H: state = 5;
pub const s_start_res: state = 4;
pub const s_res_or_resp_H: state = 3;
pub const s_start_req_or_res: state = 2;
pub const s_dead: state = 1;
pub type header_states = libc::c_uint;
pub const h_connection_upgrade: header_states = 22;
pub const h_connection_close: header_states = 21;
pub const h_connection_keep_alive: header_states = 20;
pub const h_transfer_encoding_chunked: header_states = 19;
pub const h_matching_connection_token: header_states = 18;
pub const h_matching_connection_upgrade: header_states = 17;
pub const h_matching_connection_close: header_states = 16;
pub const h_matching_connection_keep_alive: header_states = 15;
pub const h_matching_connection_token_start: header_states = 14;
pub const h_matching_transfer_encoding_chunked: header_states = 13;
pub const h_upgrade: header_states = 12;
pub const h_transfer_encoding: header_states = 11;
pub const h_content_length: header_states = 10;
pub const h_connection: header_states = 9;
pub const h_matching_upgrade: header_states = 8;
pub const h_matching_transfer_encoding: header_states = 7;
pub const h_matching_content_length: header_states = 6;
pub const h_matching_proxy_connection: header_states = 5;
pub const h_matching_connection: header_states = 4;
pub const h_CON: header_states = 3;
pub const h_CO: header_states = 2;
pub const h_C: header_states = 1;
pub const h_general: header_states = 0;
pub type http_host_state = libc::c_uint;
pub const s_http_host_port: http_host_state = 12;
pub const s_http_host_port_start: http_host_state = 11;
pub const s_http_host_v6_zone: http_host_state = 10;
pub const s_http_host_v6_zone_start: http_host_state = 9;
pub const s_http_host_v6_end: http_host_state = 8;
pub const s_http_host_v6: http_host_state = 7;
pub const s_http_host: http_host_state = 6;
pub const s_http_host_v6_start: http_host_state = 5;
pub const s_http_host_start: http_host_state = 4;
pub const s_http_userinfo: http_host_state = 3;
pub const s_http_userinfo_start: http_host_state = 2;
pub const s_http_host_dead: http_host_state = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_http_strerror_tab_527861670 {
    pub name: *const libc::c_char,
    pub description: *const libc::c_char,
}
static mut cfg: config = config {
    connections: 0,
    duration: 0,
    threads: 0,
    timeout: 0,
    pipeline: 0,
    delay: false,
    dynamic: false,
    latency: false,
    host: 0 as *const libc::c_char as *mut libc::c_char,
    script: 0 as *const libc::c_char as *mut libc::c_char,
    ctx: 0 as *const SSL_CTX as *mut SSL_CTX,
};
static mut statistics: __anonstruct_statistics_580573181 = __anonstruct_statistics_580573181 {
    latency: 0 as *const stats as *mut stats,
    requests: 0 as *const stats as *mut stats,
};
static mut sock: sock = {
    let mut init = sock {
        connect: Some(
            sock_connect
                as unsafe extern "C" fn(*mut connection, *mut libc::c_char) -> status,
        ),
        close: Some(sock_close as unsafe extern "C" fn(*mut connection) -> status),
        read: Some(
            sock_read as unsafe extern "C" fn(*mut connection, *mut size_t) -> status,
        ),
        write: Some(
            sock_write
                as unsafe extern "C" fn(
                    *mut connection,
                    *mut libc::c_char,
                    size_t,
                    *mut size_t,
                ) -> status,
        ),
        readable: Some(sock_readable as unsafe extern "C" fn(*mut connection) -> size_t),
    };
    init
};
static mut parser_settings: http_parser_settings = {
    let mut init = http_parser_settings {
        on_message_begin: None,
        on_url: None,
        on_status: None,
        on_header_field: None,
        on_header_value: None,
        on_headers_complete: None,
        on_body: None,
        on_message_complete: Some(
            response_complete as unsafe extern "C" fn(*mut http_parser) -> libc::c_int,
        ),
        on_chunk_header: None,
        on_chunk_complete: None,
    };
    init
};
static mut stop: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn handler(mut sig: libc::c_int) {
    stop = 1 as libc::c_int;
}
unsafe extern "C" fn usage() {
    printf(
        b"Usage: wrk <options> <url>                            \n  Options:                                            \n    -c, --connections <N>  Connections to keep open   \n    -d, --duration    <T>  Duration of test           \n    -t, --threads     <N>  Number of threads to use   \n                                                      \n    -s, --script      <S>  Load Lua script file       \n    -H, --header      <H>  Add header to request      \n        --latency          Print latency statistics   \n        --timeout     <T>  Socket/request timeout     \n    -v, --version          Print version details      \n                                                      \n  Numeric arguments may include a SI unit (1k, 1M, 1G)\n  Time arguments may include a time unit (2s, 2m, 2h)\n\0"
            as *const u8 as *const libc::c_char,
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut url: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut headers: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut parts: http_parser_url = http_parser_url {
        field_set: 0,
        port: 0,
        field_data: [__anonstruct_field_data_753912357 {
            off: 0,
            len: 0,
        }; 7],
    };
    let mut tmp___0: libc::c_int = 0;
    let mut schema: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut host: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut port: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut service: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: libc::c_int = 0;
    let mut threads: *mut thread = 0 as *mut thread;
    let mut tmp___6: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut L: *mut lua_State = 0 as *mut lua_State;
    let mut tmp___7: *mut lua_State = 0 as *mut lua_State;
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___8: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___9: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___10: bool = false;
    let mut i: uint64_t = 0;
    let mut t: *mut thread = 0 as *mut thread;
    let mut tmp___11: bool = false;
    let mut tmp___12: libc::c_int = 0;
    let mut tmp___13: bool = false;
    let mut msg___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___14: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___15: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___16: libc::c_int = 0;
    let mut sa: sigaction = sigaction {
        __sigaction_handler: __anonunion___sigaction_handler_363639592 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut time___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___17: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: uint64_t = 0;
    let mut tmp___18: uint64_t = 0;
    let mut complete: uint64_t = 0;
    let mut bytes: uint64_t = 0;
    let mut errors___0: errors = errors {
        connect: 0,
        read: 0,
        write: 0,
        status: 0,
        timeout: 0,
    };
    let mut i___0: uint64_t = 0;
    let mut t___0: *mut thread = 0 as *mut thread;
    let mut runtime_us: uint64_t = 0;
    let mut tmp___19: uint64_t = 0;
    let mut runtime_s: f128::f128 = f128::f128::ZERO;
    let mut req_per_s: f128::f128 = f128::f128::ZERO;
    let mut bytes_per_s: f128::f128 = f128::f128::ZERO;
    let mut interval: int64_t = 0;
    let mut runtime_msg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___20: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___21: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___22: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___23: bool = false;
    tmp = zmalloc(
        (argc as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    );
    headers = tmp as *mut *mut libc::c_char;
    parts.field_set = 0 as libc::c_int as uint16_t;
    parts.port = 0 as libc::c_int as libc::c_ushort;
    parts.field_data[0 as libc::c_int as usize].off = 0 as libc::c_int as libc::c_ushort;
    parts.field_data[0 as libc::c_int as usize].len = 0 as libc::c_int as libc::c_ushort;
    parts.field_data[1 as libc::c_int as usize].off = 0 as libc::c_int as libc::c_ushort;
    parts.field_data[1 as libc::c_int as usize].len = 0 as libc::c_int as libc::c_ushort;
    parts.field_data[2 as libc::c_int as usize].off = 0 as libc::c_int as libc::c_ushort;
    parts.field_data[2 as libc::c_int as usize].len = 0 as libc::c_int as libc::c_ushort;
    parts.field_data[3 as libc::c_int as usize].off = 0 as libc::c_int as libc::c_ushort;
    parts.field_data[3 as libc::c_int as usize].len = 0 as libc::c_int as libc::c_ushort;
    parts.field_data[4 as libc::c_int as usize].off = 0 as libc::c_int as libc::c_ushort;
    parts.field_data[4 as libc::c_int as usize].len = 0 as libc::c_int as libc::c_ushort;
    parts.field_data[5 as libc::c_int as usize].off = 0 as libc::c_int as libc::c_ushort;
    parts.field_data[5 as libc::c_int as usize].len = 0 as libc::c_int as libc::c_ushort;
    parts.field_data[6 as libc::c_int as usize].off = 0 as libc::c_int as libc::c_ushort;
    parts.field_data[6 as libc::c_int as usize].len = 0 as libc::c_int as libc::c_ushort;
    tmp___0 = parse_args(&mut cfg, &mut url, &mut parts, headers, argc, argv);
    if tmp___0 != 0 {
        usage();
        exit(1 as libc::c_int);
    }
    tmp___1 = copy_url_part(url, &mut parts, UF_SCHEMA);
    schema = tmp___1;
    tmp___2 = copy_url_part(url, &mut parts, UF_HOST);
    host = tmp___2;
    tmp___3 = copy_url_part(url, &mut parts, UF_PORT);
    port = tmp___3;
    if !port.is_null() {
        tmp___4 = port;
    } else {
        tmp___4 = schema;
    }
    service = tmp___4;
    tmp___5 = strncmp(
        b"https\0" as *const u8 as *const libc::c_char,
        schema as *const libc::c_char,
        5 as libc::c_int as size_t,
    );
    if tmp___5 == 0 {
        cfg.ctx = ssl_init();
        if cfg.ctx as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            fprintf(
                stderr,
                b"unable to initialize SSL\n\0" as *const u8 as *const libc::c_char,
            );
            ERR_print_errors_fp(stderr);
            exit(1 as libc::c_int);
        }
        sock
            .connect = Some(
            ssl_connect
                as unsafe extern "C" fn(*mut connection, *mut libc::c_char) -> status,
        );
        sock.close = Some(ssl_close as unsafe extern "C" fn(*mut connection) -> status);
        sock
            .read = Some(
            ssl_read as unsafe extern "C" fn(*mut connection, *mut size_t) -> status,
        );
        sock
            .write = Some(
            ssl_write
                as unsafe extern "C" fn(
                    *mut connection,
                    *mut libc::c_char,
                    size_t,
                    *mut size_t,
                ) -> status,
        );
        sock
            .readable = Some(
            ssl_readable as unsafe extern "C" fn(*mut connection) -> size_t,
        );
    }
    signal(
        13 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(1 as libc::c_int as libc::intptr_t),
    );
    signal(
        2 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(1 as libc::c_int as libc::intptr_t),
    );
    statistics.latency = stats_alloc((cfg.timeout).wrapping_mul(1000 as libc::c_ulong));
    statistics.requests = stats_alloc(10000000 as libc::c_int as uint64_t);
    tmp___6 = zcalloc(
        (cfg.threads).wrapping_mul(::std::mem::size_of::<thread>() as libc::c_ulong),
    );
    threads = tmp___6 as *mut thread;
    tmp___7 = script_create(cfg.script, url, headers);
    L = tmp___7;
    tmp___10 = script_resolve(L, host, service);
    if !tmp___10 {
        tmp___8 = __errno_location();
        tmp___9 = strerror(*tmp___8);
        msg = tmp___9;
        fprintf(
            stderr,
            b"unable to connect to %s:%s %s\n\0" as *const u8 as *const libc::c_char,
            host,
            service,
            msg,
        );
        exit(1 as libc::c_int);
    }
    cfg.host = host;
    i = 0 as libc::c_int as uint64_t;
    while i < cfg.threads {
        t = threads.offset(i as isize);
        (*t)
            .loop_0 = aeCreateEventLoop(
            (10 as libc::c_ulong)
                .wrapping_add((cfg.connections).wrapping_mul(3 as libc::c_ulong))
                as libc::c_int,
        );
        (*t).connections = (cfg.connections).wrapping_div(cfg.threads);
        (*t).L = script_create(cfg.script, url, headers);
        script_init(L, t, argc - optind, argv.offset(optind as isize));
        if i == 0 as libc::c_ulong {
            cfg.pipeline = script_verify_request((*t).L);
            tmp___11 = script_is_static((*t).L);
            if tmp___11 {
                tmp___12 = 0 as libc::c_int;
            } else {
                tmp___12 = 1 as libc::c_int;
            }
            cfg.dynamic = tmp___12 != 0;
            cfg.delay = script_has_delay((*t).L);
            tmp___13 = script_want_response((*t).L);
            if tmp___13 {
                parser_settings
                    .on_header_field = Some(
                    header_field
                        as unsafe extern "C" fn(
                            *mut http_parser,
                            *const libc::c_char,
                            size_t,
                        ) -> libc::c_int,
                );
                parser_settings
                    .on_header_value = Some(
                    header_value
                        as unsafe extern "C" fn(
                            *mut http_parser,
                            *const libc::c_char,
                            size_t,
                        ) -> libc::c_int,
                );
                parser_settings
                    .on_body = Some(
                    response_body
                        as unsafe extern "C" fn(
                            *mut http_parser,
                            *const libc::c_char,
                            size_t,
                        ) -> libc::c_int,
                );
            }
        }
        if ((*t).loop_0).is_null() {
            tmp___14 = __errno_location();
            tmp___15 = strerror(*tmp___14);
            msg___0 = tmp___15;
            fprintf(
                stderr,
                b"unable to create thread %lu: %s\n\0" as *const u8
                    as *const libc::c_char,
                i,
                msg___0,
            );
            exit(2 as libc::c_int);
        } else {
            tmp___16 = pthread_create(
                &mut (*t).thread as *mut pthread_t,
                0 as *mut libc::c_void as *const pthread_attr_t,
                Some(
                    thread_main
                        as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                ),
                t as *mut libc::c_void,
            );
            if tmp___16 != 0 {
                tmp___14 = __errno_location();
                tmp___15 = strerror(*tmp___14);
                msg___0 = tmp___15;
                fprintf(
                    stderr,
                    b"unable to create thread %lu: %s\n\0" as *const u8
                        as *const libc::c_char,
                    i,
                    msg___0,
                );
                exit(2 as libc::c_int);
            }
        }
        i = i.wrapping_add(1);
    }
    sa
        .__sigaction_handler
        .sa_handler = Some(handler as unsafe extern "C" fn(libc::c_int) -> ());
    sa.sa_mask.__val[0 as libc::c_int as usize] = 0 as libc::c_ulong;
    sa.sa_mask.__val[1 as libc::c_int as usize] = 0 as libc::c_ulong;
    sa.sa_mask.__val[2 as libc::c_int as usize] = 0 as libc::c_ulong;
    sa.sa_mask.__val[3 as libc::c_int as usize] = 0 as libc::c_ulong;
    sa.sa_mask.__val[4 as libc::c_int as usize] = 0 as libc::c_ulong;
    sa.sa_mask.__val[5 as libc::c_int as usize] = 0 as libc::c_ulong;
    sa.sa_mask.__val[6 as libc::c_int as usize] = 0 as libc::c_ulong;
    sa.sa_mask.__val[7 as libc::c_int as usize] = 0 as libc::c_ulong;
    sa.sa_mask.__val[8 as libc::c_int as usize] = 0 as libc::c_ulong;
    sa.sa_mask.__val[9 as libc::c_int as usize] = 0 as libc::c_ulong;
    sa.sa_mask.__val[10 as libc::c_int as usize] = 0 as libc::c_ulong;
    sa.sa_mask.__val[11 as libc::c_int as usize] = 0 as libc::c_ulong;
    sa.sa_mask.__val[12 as libc::c_int as usize] = 0 as libc::c_ulong;
    sa.sa_mask.__val[13 as libc::c_int as usize] = 0 as libc::c_ulong;
    sa.sa_mask.__val[14 as libc::c_int as usize] = 0 as libc::c_ulong;
    sa.sa_mask.__val[15 as libc::c_int as usize] = 0 as libc::c_ulong;
    sa.sa_flags = 0 as libc::c_int;
    sa.sa_restorer = None;
    sigfillset(&mut sa.sa_mask);
    sigaction(
        2 as libc::c_int,
        &mut sa as *mut sigaction as *const sigaction,
        0 as *mut libc::c_void as *mut sigaction,
    );
    tmp___17 = format_time_s(f128::f128::new(cfg.duration));
    time___0 = tmp___17;
    printf(
        b"Running %s test @ %s\n\0" as *const u8 as *const libc::c_char,
        time___0,
        url,
    );
    printf(
        b"  %lu threads and %lu connections\n\0" as *const u8 as *const libc::c_char,
        cfg.threads,
        cfg.connections,
    );
    tmp___18 = time_us();
    start = tmp___18;
    complete = 0 as libc::c_int as uint64_t;
    bytes = 0 as libc::c_int as uint64_t;
    errors___0.connect = 0 as libc::c_int as uint32_t;
    errors___0.read = 0 as libc::c_uint;
    errors___0.write = 0 as libc::c_uint;
    errors___0.status = 0 as libc::c_uint;
    errors___0.timeout = 0 as libc::c_uint;
    sleep(cfg.duration as libc::c_uint);
    stop = 1 as libc::c_int;
    i___0 = 0 as libc::c_int as uint64_t;
    while i___0 < cfg.threads {
        t___0 = threads.offset(i___0 as isize);
        pthread_join((*t___0).thread, 0 as *mut libc::c_void as *mut *mut libc::c_void);
        complete = (complete as libc::c_ulong).wrapping_add((*t___0).complete)
            as uint64_t as uint64_t;
        bytes = (bytes as libc::c_ulong).wrapping_add((*t___0).bytes) as uint64_t
            as uint64_t;
        errors___0
            .connect = (errors___0.connect as libc::c_uint)
            .wrapping_add((*t___0).errors.connect) as uint32_t as uint32_t;
        errors___0
            .read = (errors___0.read as libc::c_uint).wrapping_add((*t___0).errors.read)
            as uint32_t as uint32_t;
        errors___0
            .write = (errors___0.write as libc::c_uint)
            .wrapping_add((*t___0).errors.write) as uint32_t as uint32_t;
        errors___0
            .timeout = (errors___0.timeout as libc::c_uint)
            .wrapping_add((*t___0).errors.timeout) as uint32_t as uint32_t;
        errors___0
            .status = (errors___0.status as libc::c_uint)
            .wrapping_add((*t___0).errors.status) as uint32_t as uint32_t;
        i___0 = i___0.wrapping_add(1);
    }
    tmp___19 = time_us();
    runtime_us = tmp___19.wrapping_sub(start);
    runtime_s = f128::f128::new(runtime_us as libc::c_double / 1000000.0f64);
    req_per_s = f128::f128::new(complete) / runtime_s;
    bytes_per_s = f128::f128::new(bytes) / runtime_s;
    if complete.wrapping_div(cfg.connections) > 0 as libc::c_ulong {
        interval = runtime_us.wrapping_div(complete.wrapping_div(cfg.connections))
            as int64_t;
        stats_correct(statistics.latency, interval);
    }
    print_stats_header();
    print_stats(
        b"Latency\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        statistics.latency,
        Some(format_time_us as unsafe extern "C" fn(f128::f128) -> *mut libc::c_char),
    );
    print_stats(
        b"Req/Sec\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        statistics.requests,
        Some(format_metric as unsafe extern "C" fn(f128::f128) -> *mut libc::c_char),
    );
    if cfg.latency {
        print_stats_latency(statistics.latency);
    }
    tmp___20 = format_time_us(f128::f128::new(runtime_us));
    runtime_msg = tmp___20;
    tmp___21 = format_binary(f128::f128::new(bytes));
    printf(
        b"  %lu requests in %s, %sB read\n\0" as *const u8 as *const libc::c_char,
        complete,
        runtime_msg,
        tmp___21,
    );
    if errors___0.connect != 0 {
        printf(
            b"  Socket errors: connect %d, read %d, write %d, timeout %d\n\0"
                as *const u8 as *const libc::c_char,
            errors___0.connect,
            errors___0.read,
            errors___0.write,
            errors___0.timeout,
        );
    } else if errors___0.read != 0 {
        printf(
            b"  Socket errors: connect %d, read %d, write %d, timeout %d\n\0"
                as *const u8 as *const libc::c_char,
            errors___0.connect,
            errors___0.read,
            errors___0.write,
            errors___0.timeout,
        );
    } else if errors___0.write != 0 {
        printf(
            b"  Socket errors: connect %d, read %d, write %d, timeout %d\n\0"
                as *const u8 as *const libc::c_char,
            errors___0.connect,
            errors___0.read,
            errors___0.write,
            errors___0.timeout,
        );
    } else if errors___0.timeout != 0 {
        printf(
            b"  Socket errors: connect %d, read %d, write %d, timeout %d\n\0"
                as *const u8 as *const libc::c_char,
            errors___0.connect,
            errors___0.read,
            errors___0.write,
            errors___0.timeout,
        );
    }
    if errors___0.status != 0 {
        printf(
            b"  Non-2xx or 3xx responses: %d\n\0" as *const u8 as *const libc::c_char,
            errors___0.status,
        );
    }
    printf(b"Requests/sec: %9.2Lf\n\0" as *const u8 as *const libc::c_char, req_per_s);
    tmp___22 = format_binary(bytes_per_s);
    printf(b"Transfer/sec: %10sB\n\0" as *const u8 as *const libc::c_char, tmp___22);
    tmp___23 = script_has_done(L);
    if tmp___23 {
        script_summary(L, runtime_us, complete, bytes);
        script_errors(L, &mut errors___0);
        script_done(L, statistics.latency, statistics.requests);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn thread_main(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut thread___0: *mut thread = 0 as *mut thread;
    let mut request: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut length: size_t = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut c: *mut connection = 0 as *mut connection;
    let mut i: uint64_t = 0;
    let mut tmp___0: *mut SSL = 0 as *mut SSL;
    let mut loop_0: *mut aeEventLoop = 0 as *mut aeEventLoop;
    thread___0 = arg as *mut thread;
    request = 0 as *mut libc::c_void as *mut libc::c_char;
    length = 0 as libc::c_int as size_t;
    if !cfg.dynamic {
        script_request((*thread___0).L, &mut request, &mut length);
    }
    tmp = zcalloc(
        ((*thread___0).connections)
            .wrapping_mul(::std::mem::size_of::<connection>() as libc::c_ulong),
    );
    (*thread___0).cs = tmp as *mut connection;
    c = (*thread___0).cs;
    i = 0 as libc::c_int as uint64_t;
    while i < (*thread___0).connections {
        (*c).thread = thread___0;
        if !(cfg.ctx).is_null() {
            tmp___0 = SSL_new(cfg.ctx);
            (*c).ssl = tmp___0;
        } else {
            (*c).ssl = 0 as *mut libc::c_void as *mut SSL;
        }
        (*c).request = request;
        (*c).length = length;
        (*c).delayed = cfg.delay;
        connect_socket(thread___0, c);
        i = i.wrapping_add(1);
        c = c.offset(1);
    }
    loop_0 = (*thread___0).loop_0;
    aeCreateTimeEvent(
        loop_0,
        100 as libc::c_longlong,
        Some(
            record_rate
                as unsafe extern "C" fn(
                    *mut aeEventLoop,
                    libc::c_longlong,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        thread___0 as *mut libc::c_void,
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<aeEventFinalizerProc>,
        >(0 as *mut libc::c_void),
    );
    (*thread___0).start = time_us();
    aeMain(loop_0);
    aeDeleteEventLoop(loop_0);
    zfree((*thread___0).cs as *mut libc::c_void);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn connect_socket(
    mut thread___0: *mut thread,
    mut c: *mut connection,
) -> libc::c_int {
    let mut addr: *mut addrinfo = 0 as *mut addrinfo;
    let mut loop_0: *mut aeEventLoop = 0 as *mut aeEventLoop;
    let mut fd: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut current_block_18: u64;
    addr = (*thread___0).addr;
    loop_0 = (*thread___0).loop_0;
    fd = socket((*addr).ai_family, (*addr).ai_socktype, (*addr).ai_protocol);
    flags = fcntl(fd, 3 as libc::c_int, 0 as libc::c_int);
    fcntl(fd, 4 as libc::c_int, flags | 2048 as libc::c_int);
    tmp___0 = connect(fd, (*addr).ai_addr as *const sockaddr, (*addr).ai_addrlen);
    if tmp___0 == -(1 as libc::c_int) {
        tmp = __errno_location();
        if *tmp != 115 as libc::c_int {
            current_block_18 = 9801133618967520601;
        } else {
            current_block_18 = 3640593987805443782;
        }
    } else {
        current_block_18 = 3640593987805443782;
    }
    match current_block_18 {
        3640593987805443782 => {
            flags = 1 as libc::c_int;
            setsockopt(
                fd,
                6 as libc::c_int,
                1 as libc::c_int,
                &mut flags as *mut libc::c_int as *const libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
            );
            flags = 3 as libc::c_int;
            tmp___1 = aeCreateFileEvent(
                loop_0,
                fd,
                flags,
                Some(
                    socket_connected
                        as unsafe extern "C" fn(
                            *mut aeEventLoop,
                            libc::c_int,
                            *mut libc::c_void,
                            libc::c_int,
                        ) -> (),
                ),
                c as *mut libc::c_void,
            );
            if tmp___1 == 0 as libc::c_int {
                (*c).parser.data = c as *mut libc::c_void;
                (*c).fd = fd;
                return fd;
            }
        }
        _ => {}
    }
    (*thread___0).errors.connect = ((*thread___0).errors.connect).wrapping_add(1);
    close(fd);
    return -(1 as libc::c_int);
}
unsafe extern "C" fn reconnect_socket(
    mut thread___0: *mut thread,
    mut c: *mut connection,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    aeDeleteFileEvent((*thread___0).loop_0, (*c).fd, 3 as libc::c_int);
    (Some((sock.close).expect("non-null function pointer")))
        .expect("non-null function pointer")(c);
    close((*c).fd);
    tmp = connect_socket(thread___0, c);
    return tmp;
}
unsafe extern "C" fn record_rate(
    mut loop_0: *mut aeEventLoop,
    mut id: libc::c_longlong,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut thread___0: *mut thread = 0 as *mut thread;
    let mut elapsed_ms: uint64_t = 0;
    let mut tmp: uint64_t = 0;
    let mut requests: uint64_t = 0;
    thread___0 = data as *mut thread;
    if (*thread___0).requests > 0 as libc::c_ulong {
        tmp = time_us();
        elapsed_ms = tmp
            .wrapping_sub((*thread___0).start)
            .wrapping_div(1000 as libc::c_ulong);
        requests = ((*thread___0).requests as libc::c_double
            / elapsed_ms as libc::c_double * 1000 as libc::c_int as libc::c_double)
            as uint64_t;
        stats_record(statistics.requests, requests);
        (*thread___0).requests = 0 as libc::c_int as uint64_t;
        (*thread___0).start = time_us();
    }
    if stop != 0 {
        aeStop(loop_0);
    }
    return 100 as libc::c_int;
}
unsafe extern "C" fn delay_request(
    mut loop_0: *mut aeEventLoop,
    mut id: libc::c_longlong,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut c: *mut connection = 0 as *mut connection;
    c = data as *mut connection;
    (*c).delayed = 0 as libc::c_int != 0;
    aeCreateFileEvent(
        loop_0,
        (*c).fd,
        2 as libc::c_int,
        Some(
            socket_writeable
                as unsafe extern "C" fn(
                    *mut aeEventLoop,
                    libc::c_int,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> (),
        ),
        c as *mut libc::c_void,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn header_field(
    mut parser: *mut http_parser,
    mut at: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    let mut c: *mut connection = 0 as *mut connection;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    c = (*parser).data as *mut connection;
    if (*c).state as libc::c_uint == 1 as libc::c_uint {
        tmp = (*c).headers.cursor;
        (*c).headers.cursor = ((*c).headers.cursor).offset(1);
        *tmp = '\u{0}' as i32 as libc::c_char;
        (*c).state = FIELD;
    }
    buffer_append(&mut (*c).headers, at, len);
    return 0 as libc::c_int;
}
unsafe extern "C" fn header_value(
    mut parser: *mut http_parser,
    mut at: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    let mut c: *mut connection = 0 as *mut connection;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    c = (*parser).data as *mut connection;
    if (*c).state as libc::c_uint == 0 as libc::c_uint {
        tmp = (*c).headers.cursor;
        (*c).headers.cursor = ((*c).headers.cursor).offset(1);
        *tmp = '\u{0}' as i32 as libc::c_char;
        (*c).state = VALUE;
    }
    buffer_append(&mut (*c).headers, at, len);
    return 0 as libc::c_int;
}
unsafe extern "C" fn response_body(
    mut parser: *mut http_parser,
    mut at: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    let mut c: *mut connection = 0 as *mut connection;
    c = (*parser).data as *mut connection;
    buffer_append(&mut (*c).body, at, len);
    return 0 as libc::c_int;
}
unsafe extern "C" fn response_complete(mut parser: *mut http_parser) -> libc::c_int {
    let mut c: *mut connection = 0 as *mut connection;
    let mut thread___0: *mut thread = 0 as *mut thread;
    let mut now: uint64_t = 0;
    let mut tmp: uint64_t = 0;
    let mut status___0: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    c = (*parser).data as *mut connection;
    thread___0 = (*c).thread;
    tmp = time_us();
    now = tmp;
    status___0 = (*parser).status_code() as libc::c_int;
    (*thread___0).complete = ((*thread___0).complete).wrapping_add(1);
    (*thread___0).requests = ((*thread___0).requests).wrapping_add(1);
    if status___0 > 399 as libc::c_int {
        (*thread___0).errors.status = ((*thread___0).errors.status).wrapping_add(1);
    }
    if !((*c).headers.buffer).is_null() {
        tmp___0 = (*c).headers.cursor;
        (*c).headers.cursor = ((*c).headers.cursor).offset(1);
        *tmp___0 = '\u{0}' as i32 as libc::c_char;
        script_response((*thread___0).L, status___0, &mut (*c).headers, &mut (*c).body);
        (*c).state = FIELD;
    }
    (*c).pending = ((*c).pending).wrapping_sub(1);
    if (*c).pending == 0 as libc::c_ulong {
        tmp___1 = stats_record(statistics.latency, now.wrapping_sub((*c).start));
        if tmp___1 == 0 {
            (*thread___0)
                .errors
                .timeout = ((*thread___0).errors.timeout).wrapping_add(1);
        }
        (*c).delayed = cfg.delay;
        aeCreateFileEvent(
            (*thread___0).loop_0,
            (*c).fd,
            2 as libc::c_int,
            Some(
                socket_writeable
                    as unsafe extern "C" fn(
                        *mut aeEventLoop,
                        libc::c_int,
                        *mut libc::c_void,
                        libc::c_int,
                    ) -> (),
            ),
            c as *mut libc::c_void,
        );
    }
    tmp___2 = http_should_keep_alive(parser as *const http_parser);
    if tmp___2 == 0 {
        reconnect_socket(thread___0, c);
    } else {
        http_parser_init(parser, HTTP_RESPONSE);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn socket_connected(
    mut loop_0: *mut aeEventLoop,
    mut fd: libc::c_int,
    mut data: *mut libc::c_void,
    mut mask: libc::c_int,
) {
    let mut c: *mut connection = 0 as *mut connection;
    let mut tmp: status = OK;
    c = data as *mut connection;
    tmp = (Some((sock.connect).expect("non-null function pointer")))
        .expect("non-null function pointer")(c, cfg.host);
    match tmp as libc::c_uint {
        1 => {
            (*(*c).thread)
                .errors
                .connect = ((*(*c).thread).errors.connect).wrapping_add(1);
            reconnect_socket((*c).thread, c);
            return;
        }
        2 => return,
        0 | _ => {
            http_parser_init(&mut (*c).parser, HTTP_RESPONSE);
            (*c).written = 0 as libc::c_int as size_t;
            aeCreateFileEvent(
                (*(*c).thread).loop_0,
                fd,
                1 as libc::c_int,
                Some(
                    socket_readable
                        as unsafe extern "C" fn(
                            *mut aeEventLoop,
                            libc::c_int,
                            *mut libc::c_void,
                            libc::c_int,
                        ) -> (),
                ),
                c as *mut libc::c_void,
            );
            aeCreateFileEvent(
                (*(*c).thread).loop_0,
                fd,
                2 as libc::c_int,
                Some(
                    socket_writeable
                        as unsafe extern "C" fn(
                            *mut aeEventLoop,
                            libc::c_int,
                            *mut libc::c_void,
                            libc::c_int,
                        ) -> (),
                ),
                c as *mut libc::c_void,
            );
            return;
        }
    };
}
unsafe extern "C" fn socket_writeable(
    mut loop_0: *mut aeEventLoop,
    mut fd: libc::c_int,
    mut data: *mut libc::c_void,
    mut mask: libc::c_int,
) {
    let mut c: *mut connection = 0 as *mut connection;
    let mut thread___0: *mut thread = 0 as *mut thread;
    let mut delay: uint64_t = 0;
    let mut tmp: uint64_t = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut n: size_t = 0;
    let mut tmp___0: status = OK;
    c = data as *mut connection;
    thread___0 = (*c).thread;
    if (*c).delayed {
        tmp = script_delay((*thread___0).L);
        delay = tmp;
        aeDeleteFileEvent(loop_0, fd, 2 as libc::c_int);
        aeCreateTimeEvent(
            loop_0,
            delay as libc::c_longlong,
            Some(
                delay_request
                    as unsafe extern "C" fn(
                        *mut aeEventLoop,
                        libc::c_longlong,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            c as *mut libc::c_void,
            ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<aeEventFinalizerProc>,
            >(0 as *mut libc::c_void),
        );
        return;
    }
    if (*c).written == 0 {
        if cfg.dynamic {
            script_request((*thread___0).L, &mut (*c).request, &mut (*c).length);
        }
        (*c).start = time_us();
        (*c).pending = cfg.pipeline;
    }
    buf = ((*c).request).offset((*c).written as isize);
    len = ((*c).length).wrapping_sub((*c).written);
    tmp___0 = (Some((sock.write).expect("non-null function pointer")))
        .expect("non-null function pointer")(c, buf, len, &mut n);
    match tmp___0 as libc::c_uint {
        1 => {
            (*thread___0).errors.write = ((*thread___0).errors.write).wrapping_add(1);
            reconnect_socket(thread___0, c);
            return;
        }
        2 => return,
        0 | _ => {
            (*c)
                .written = ((*c).written as libc::c_ulong).wrapping_add(n) as size_t
                as size_t;
            if (*c).written == (*c).length {
                (*c).written = 0 as libc::c_int as size_t;
                aeDeleteFileEvent(loop_0, fd, 2 as libc::c_int);
            }
            return;
        }
    };
}
unsafe extern "C" fn socket_readable(
    mut loop_0: *mut aeEventLoop,
    mut fd: libc::c_int,
    mut data: *mut libc::c_void,
    mut mask: libc::c_int,
) {
    let mut current_block: u64;
    let mut c: *mut connection = 0 as *mut connection;
    let mut n: size_t = 0;
    let mut tmp: status = OK;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: size_t = 0;
    c = data as *mut connection;
    loop {
        tmp = (Some((sock.read).expect("non-null function pointer")))
            .expect("non-null function pointer")(c, &mut n);
        match tmp as libc::c_uint {
            1 => {
                current_block = 17528326052349332511;
                break;
            }
            2 => return,
            0 | _ => {
                tmp___0 = http_parser_execute(
                    &mut (*c).parser,
                    &mut parser_settings as *mut http_parser_settings
                        as *const http_parser_settings,
                    ((*c).buf).as_mut_ptr() as *const libc::c_char,
                    n,
                );
                if tmp___0 != n {
                    current_block = 17528326052349332511;
                    break;
                }
                if n == 0 as libc::c_ulong {
                    tmp___1 = http_body_is_final(
                        &mut (*c).parser as *mut http_parser as *const http_parser,
                    );
                    if tmp___1 == 0 {
                        current_block = 17528326052349332511;
                        break;
                    }
                }
                (*(*c).thread)
                    .bytes = ((*(*c).thread).bytes as libc::c_ulong).wrapping_add(n)
                    as uint64_t as uint64_t;
                if !(n == 8192 as libc::c_ulong) {
                    current_block = 5948590327928692120;
                    break;
                }
                tmp___2 = (Some((sock.readable).expect("non-null function pointer")))
                    .expect("non-null function pointer")(c);
                if !(tmp___2 > 0 as libc::c_ulong) {
                    current_block = 5948590327928692120;
                    break;
                }
            }
        }
    }
    match current_block {
        5948590327928692120 => return,
        _ => {
            (*(*c).thread).errors.read = ((*(*c).thread).errors.read).wrapping_add(1);
            reconnect_socket((*c).thread, c);
            return;
        }
    };
}
unsafe extern "C" fn time_us() -> uint64_t {
    let mut t: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut t as *mut timeval, 0 as *mut libc::c_void);
    return (t.tv_sec * 1000000 as libc::c_long + t.tv_usec) as uint64_t;
}
unsafe extern "C" fn copy_url_part(
    mut url: *mut libc::c_char,
    mut parts: *mut http_parser_url,
    mut field: http_parser_url_fields,
) -> *mut libc::c_char {
    let mut part: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut off: uint16_t = 0;
    let mut len: uint16_t = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    part = 0 as *mut libc::c_void as *mut libc::c_char;
    if (*parts).field_set as libc::c_int & (1 as libc::c_int) << field as libc::c_uint
        != 0
    {
        off = (*parts).field_data[field as usize].off;
        len = (*parts).field_data[field as usize].len;
        tmp = zcalloc(
            (len as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
        part = tmp as *mut libc::c_char;
        memcpy(
            part as *mut libc::c_void,
            url.offset(off as libc::c_int as isize) as *const libc::c_void,
            len as size_t,
        );
    }
    return part;
}
static mut longopts: [option; 10] = [
    {
        let mut init = option {
            name: b"connections\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"duration\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"threads\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"script\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 's' as i32,
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
            name: b"latency\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'L' as i32,
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
            val: 'v' as i32,
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
unsafe extern "C" fn parse_args(
    mut cfg___0: *mut config,
    mut url: *mut *mut libc::c_char,
    mut parts: *mut http_parser_url,
    mut headers: *mut *mut libc::c_char,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut header: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut c: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: libc::c_int = 0;
    header = headers;
    memset(
        cfg___0 as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<config>() as libc::c_ulong,
    );
    (*cfg___0).threads = 2 as libc::c_int as uint64_t;
    (*cfg___0).connections = 10 as libc::c_int as uint64_t;
    (*cfg___0).duration = 10 as libc::c_int as uint64_t;
    (*cfg___0).timeout = 2000 as libc::c_int as uint64_t;
    loop {
        c = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            b"t:c:d:s:H:T:Lrv?\0" as *const u8 as *const libc::c_char,
            longopts.as_mut_ptr() as *const option,
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            116 => {
                tmp = scan_metric(optarg, &mut (*cfg___0).threads);
                if tmp != 0 {
                    return -(1 as libc::c_int);
                }
            }
            99 => {
                tmp___0 = scan_metric(optarg, &mut (*cfg___0).connections);
                if tmp___0 != 0 {
                    return -(1 as libc::c_int);
                }
            }
            100 => {
                tmp___1 = scan_time(optarg, &mut (*cfg___0).duration);
                if tmp___1 != 0 {
                    return -(1 as libc::c_int);
                }
            }
            115 => {
                (*cfg___0).script = optarg;
            }
            72 => {
                tmp___2 = header;
                header = header.offset(1);
                *tmp___2 = optarg;
            }
            76 => {
                (*cfg___0).latency = 1 as libc::c_int != 0;
            }
            84 => {
                tmp___3 = scan_time(optarg, &mut (*cfg___0).timeout);
                if tmp___3 != 0 {
                    return -(1 as libc::c_int);
                }
                (*cfg___0)
                    .timeout = ((*cfg___0).timeout as libc::c_ulong)
                    .wrapping_mul(1000 as libc::c_ulong) as uint64_t as uint64_t;
            }
            118 => {
                tmp___4 = aeGetApiName();
                printf(
                    b"wrk %s [%s] \0" as *const u8 as *const libc::c_char,
                    VERSION,
                    tmp___4,
                );
                printf(
                    b"Copyright (C) 2012 Will Glozer\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            _ => return -(1 as libc::c_int),
        }
    }
    if optind == argc {
        return -(1 as libc::c_int)
    } else {
        if (*cfg___0).threads == 0 {
            return -(1 as libc::c_int)
        } else {
            if (*cfg___0).duration == 0 {
                return -(1 as libc::c_int);
            }
        }
    }
    tmp___5 = script_parse_url(*argv.offset(optind as isize), parts);
    if tmp___5 == 0 {
        fprintf(
            stderr,
            b"invalid URL: %s\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(optind as isize),
        );
        return -(1 as libc::c_int);
    }
    if (*cfg___0).connections == 0 {
        fprintf(
            stderr,
            b"number of connections must be >= threads\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    } else {
        if (*cfg___0).connections < (*cfg___0).threads {
            fprintf(
                stderr,
                b"number of connections must be >= threads\n\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    *url = *argv.offset(optind as isize);
    *header = 0 as *mut libc::c_void as *mut libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn print_stats_header() {
    printf(
        b"  Thread Stats%6s%11s%8s%12s\n\0" as *const u8 as *const libc::c_char,
        b"Avg\0" as *const u8 as *const libc::c_char,
        b"Stdev\0" as *const u8 as *const libc::c_char,
        b"Max\0" as *const u8 as *const libc::c_char,
        b"+/- Stdev\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn print_units(
    mut n: f128::f128,
    mut fmt: Option::<unsafe extern "C" fn(f128::f128) -> *mut libc::c_char>,
    mut width: libc::c_int,
) {
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut tmp___0: size_t = 0;
    let mut pad: libc::c_int = 0;
    let mut tmp___1: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut tmp___2: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    tmp = (Some(fmt.expect("non-null function pointer")))
        .expect("non-null function pointer")(n);
    msg = tmp;
    tmp___0 = strlen(msg as *const libc::c_char);
    len = tmp___0 as libc::c_int;
    pad = 2 as libc::c_int;
    tmp___1 = __ctype_b_loc();
    if *(*tmp___1)
        .offset(*msg.offset((len - 1 as libc::c_int) as isize) as libc::c_int as isize)
        as libc::c_int & 1024 as libc::c_int != 0
    {
        pad -= 1;
    }
    tmp___2 = __ctype_b_loc();
    if *(*tmp___2)
        .offset(*msg.offset((len - 2 as libc::c_int) as isize) as libc::c_int as isize)
        as libc::c_int & 1024 as libc::c_int != 0
    {
        pad -= 1;
    }
    width -= pad;
    printf(
        b"%*.*s%.*s\0" as *const u8 as *const libc::c_char,
        width,
        width,
        msg,
        pad,
        b"  \0" as *const u8 as *const libc::c_char,
    );
    free(msg as *mut libc::c_void);
}
unsafe extern "C" fn print_stats(
    mut name: *mut libc::c_char,
    mut stats___0: *mut stats,
    mut fmt: Option::<unsafe extern "C" fn(f128::f128) -> *mut libc::c_char>,
) {
    let mut max: uint64_t = 0;
    let mut mean: f128::f128 = f128::f128::ZERO;
    let mut tmp: f128::f128 = f128::f128::ZERO;
    let mut stdev: f128::f128 = f128::f128::ZERO;
    let mut tmp___0: f128::f128 = f128::f128::ZERO;
    let mut tmp___1: f128::f128 = f128::f128::ZERO;
    max = (*stats___0).max;
    tmp = stats_mean(stats___0);
    mean = tmp;
    tmp___0 = stats_stdev(stats___0, mean);
    stdev = tmp___0;
    printf(b"    %-10s\0" as *const u8 as *const libc::c_char, name);
    print_units(mean, fmt, 8 as libc::c_int);
    print_units(stdev, fmt, 10 as libc::c_int);
    print_units(f128::f128::new(max), fmt, 9 as libc::c_int);
    tmp___1 = stats_within_stdev(stats___0, mean, stdev, 1 as libc::c_int as uint64_t);
    printf(b"%8.2Lf%%\n\0" as *const u8 as *const libc::c_char, tmp___1);
}
unsafe extern "C" fn print_stats_latency(mut stats___0: *mut stats) {
    let mut percentiles: [f128::f128; 4] = [f128::f128::ZERO; 4];
    let mut i: size_t = 0;
    let mut p: f128::f128 = f128::f128::ZERO;
    let mut n: uint64_t = 0;
    let mut tmp: uint64_t = 0;
    percentiles[0 as libc::c_int as usize] = f128::f128::new(50.0f64);
    percentiles[1 as libc::c_int as usize] = f128::f128::new(75.0f64);
    percentiles[2 as libc::c_int as usize] = f128::f128::new(90.0f64);
    percentiles[3 as libc::c_int as usize] = f128::f128::new(99.0f64);
    printf(b"  Latency Distribution\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[f128::f128; 4]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<f128::f128>() as libc::c_ulong)
    {
        p = percentiles[i as usize];
        tmp = stats_percentile(stats___0, p);
        n = tmp;
        printf(b"%7.0Lf%%\0" as *const u8 as *const libc::c_char, p);
        print_units(
            f128::f128::new(n),
            Some(
                format_time_us as unsafe extern "C" fn(f128::f128) -> *mut libc::c_char,
            ),
            10 as libc::c_int,
        );
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i = i.wrapping_add(1);
    }
}
pub unsafe extern "C" fn sock_connect(
    mut c: *mut connection,
    mut host: *mut libc::c_char,
) -> status {
    return OK;
}
pub unsafe extern "C" fn sock_close(mut c: *mut connection) -> status {
    return OK;
}
pub unsafe extern "C" fn sock_read(
    mut c: *mut connection,
    mut n: *mut size_t,
) -> status {
    let mut r: ssize_t = 0;
    let mut tmp: ssize_t = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = read(
        (*c).fd,
        ((*c).buf).as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
    );
    r = tmp;
    *n = r as size_t;
    if r >= 0 as libc::c_long {
        tmp___0 = 0 as libc::c_int;
    } else {
        tmp___0 = 1 as libc::c_int;
    }
    return tmp___0 as status;
}
pub unsafe extern "C" fn sock_write(
    mut c: *mut connection,
    mut buf: *mut libc::c_char,
    mut len: size_t,
    mut n: *mut size_t,
) -> status {
    let mut r: ssize_t = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    r = write((*c).fd, buf as *const libc::c_void, len);
    if r == -(1 as libc::c_long) {
        tmp = __errno_location();
        match *tmp {
            11 => return RETRY,
            _ => return ERROR,
        }
    }
    *n = r as size_t;
    return OK;
}
pub unsafe extern "C" fn sock_readable(mut c: *mut connection) -> size_t {
    let mut n: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    rc = ioctl((*c).fd, 21531 as libc::c_ulong, &mut n as *mut libc::c_int);
    if rc == -(1 as libc::c_int) {
        tmp = 0 as libc::c_int;
    } else {
        tmp = n;
    }
    return tmp as size_t;
}
pub unsafe extern "C" fn ssl_init() -> *mut SSL_CTX {
    let mut ctx: *mut SSL_CTX = 0 as *mut SSL_CTX;
    let mut tmp: *const SSL_METHOD = 0 as *const SSL_METHOD;
    ctx = 0 as *mut libc::c_void as *mut SSL_CTX;
    OPENSSL_init_ssl(
        2097154 as libc::c_long as uint64_t,
        0 as *mut libc::c_void as *const OPENSSL_INIT_SETTINGS,
    );
    OPENSSL_init_ssl(
        0 as libc::c_int as uint64_t,
        0 as *mut libc::c_void as *const OPENSSL_INIT_SETTINGS,
    );
    OPENSSL_init_crypto(
        12 as libc::c_long as uint64_t,
        0 as *mut libc::c_void as *const OPENSSL_INIT_SETTINGS,
    );
    tmp = TLS_client_method();
    ctx = SSL_CTX_new(tmp);
    if !ctx.is_null() {
        SSL_CTX_set_verify(
            ctx,
            0 as libc::c_int,
            ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<
                    unsafe extern "C" fn(libc::c_int, *mut X509_STORE_CTX) -> libc::c_int,
                >,
            >(0 as *mut libc::c_void),
        );
        SSL_CTX_set_verify_depth(ctx, 0 as libc::c_int);
        SSL_CTX_ctrl(ctx, 33 as libc::c_int, 4 as libc::c_long, 0 as *mut libc::c_void);
        SSL_CTX_ctrl(ctx, 44 as libc::c_int, 1 as libc::c_long, 0 as *mut libc::c_void);
    }
    return ctx;
}
pub unsafe extern "C" fn ssl_connect(
    mut c: *mut connection,
    mut host: *mut libc::c_char,
) -> status {
    let mut r: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    SSL_set_fd((*c).ssl, (*c).fd);
    SSL_ctrl((*c).ssl, 55 as libc::c_int, 0 as libc::c_long, host as *mut libc::c_void);
    r = SSL_connect((*c).ssl);
    if r != 1 as libc::c_int {
        tmp = SSL_get_error((*c).ssl as *const SSL, r);
        match tmp {
            2 => return RETRY,
            3 => return RETRY,
            _ => return ERROR,
        }
    }
    return OK;
}
pub unsafe extern "C" fn ssl_close(mut c: *mut connection) -> status {
    SSL_shutdown((*c).ssl);
    SSL_clear((*c).ssl);
    return OK;
}
pub unsafe extern "C" fn ssl_read(mut c: *mut connection, mut n: *mut size_t) -> status {
    let mut r: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    r = SSL_read(
        (*c).ssl,
        ((*c).buf).as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong as libc::c_int,
    );
    if r <= 0 as libc::c_int {
        tmp = SSL_get_error((*c).ssl as *const SSL, r);
        match tmp {
            2 => return RETRY,
            3 => return RETRY,
            _ => return ERROR,
        }
    }
    *n = r as size_t;
    return OK;
}
pub unsafe extern "C" fn ssl_write(
    mut c: *mut connection,
    mut buf: *mut libc::c_char,
    mut len: size_t,
    mut n: *mut size_t,
) -> status {
    let mut r: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    r = SSL_write((*c).ssl, buf as *const libc::c_void, len as libc::c_int);
    if r <= 0 as libc::c_int {
        tmp = SSL_get_error((*c).ssl as *const SSL, r);
        match tmp {
            2 => return RETRY,
            3 => return RETRY,
            _ => return ERROR,
        }
    }
    *n = r as size_t;
    return OK;
}
pub unsafe extern "C" fn ssl_readable(mut c: *mut connection) -> size_t {
    let mut tmp: libc::c_int = 0;
    tmp = SSL_pending((*c).ssl as *const SSL);
    return tmp as size_t;
}
pub unsafe extern "C" fn aprintf(
    mut s: *mut *mut libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> *mut libc::c_char {
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ap: ::std::ffi::VaListImpl;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    c = 0 as *mut libc::c_void as *mut libc::c_char;
    ap = args.clone();
    tmp = vsnprintf(
        0 as *mut libc::c_void as *mut libc::c_char,
        0 as libc::c_int as size_t,
        fmt,
        ap.as_va_list(),
    );
    n = tmp + 1 as libc::c_int;
    if !(*s).is_null() {
        tmp___0 = strlen(*s as *const libc::c_char);
        len = tmp___0 as libc::c_int;
    } else {
        len = 0 as libc::c_int;
    }
    tmp___2 = realloc(
        *s as *mut libc::c_void,
        ((len + n) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    tmp___1 = tmp___2 as *mut libc::c_char;
    *s = tmp___1;
    if !tmp___1.is_null() {
        c = (*s).offset(len as isize);
        ap = args.clone();
        vsnprintf(c, n as size_t, fmt, ap.as_va_list());
    }
    return c;
}
pub unsafe extern "C" fn stats_alloc(mut max: uint64_t) -> *mut stats {
    let mut limit: uint64_t = 0;
    let mut s: *mut stats = 0 as *mut stats;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    limit = max.wrapping_add(1 as libc::c_ulong);
    tmp = zcalloc(
        (::std::mem::size_of::<stats>() as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<uint64_t>() as libc::c_ulong).wrapping_mul(limit),
            ),
    );
    s = tmp as *mut stats;
    (*s).limit = limit;
    (*s).min = 18446744073709551615 as libc::c_ulonglong as uint64_t;
    return s;
}
pub unsafe extern "C" fn stats_free(mut stats___0: *mut stats) {
    zfree(stats___0 as *mut libc::c_void);
}
pub unsafe extern "C" fn stats_record(
    mut stats___0: *mut stats,
    mut n: uint64_t,
) -> libc::c_int {
    let mut min: uint64_t = 0;
    let mut max: uint64_t = 0;
    if n >= (*stats___0).limit {
        return 0 as libc::c_int;
    }
    ::std::intrinsics::atomic_xadd_seqcst(
        &mut *((*stats___0).data).as_mut_ptr().offset(n as isize) as *mut uint64_t,
        1 as libc::c_int as uint64_t,
    );
    ::std::intrinsics::atomic_xadd_seqcst(
        &mut (*stats___0).count,
        1 as libc::c_int as uint64_t,
    );
    min = (*stats___0).min;
    max = (*stats___0).max;
    while n < min {
        min = (::std::intrinsics::atomic_cxchg(
            &mut (*stats___0).min as *mut uint64_t,
            min,
            n,
        ))
            .0;
    }
    while n > max {
        max = (::std::intrinsics::atomic_cxchg(
            &mut (*stats___0).max as *mut uint64_t,
            max,
            n,
        ))
            .0;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn stats_correct(
    mut stats___0: *mut stats,
    mut expected: int64_t,
) {
    let mut n: uint64_t = 0;
    let mut count: uint64_t = 0;
    let mut m: int64_t = 0;
    n = (expected * 2 as libc::c_long) as uint64_t;
    while n <= (*stats___0).max {
        count = *((*stats___0).data).as_mut_ptr().offset(n as isize);
        m = n as int64_t - expected;
        while count != 0 {
            if !(m > expected) {
                break;
            }
            let ref mut fresh0 = *((*stats___0).data).as_mut_ptr().offset(m as isize);
            *fresh0 = (*fresh0 as libc::c_ulong).wrapping_add(count) as uint64_t
                as uint64_t;
            (*stats___0)
                .count = ((*stats___0).count as libc::c_ulong).wrapping_add(count)
                as uint64_t as uint64_t;
            m -= expected;
        }
        n = n.wrapping_add(1);
    }
}
pub unsafe extern "C" fn stats_mean(mut stats___0: *mut stats) -> f128::f128 {
    let mut sum: uint64_t = 0;
    let mut i: uint64_t = 0;
    if (*stats___0).count == 0 as libc::c_ulong {
        return f128::f128::new(0.0f64);
    }
    sum = 0 as libc::c_int as uint64_t;
    i = (*stats___0).min;
    while i <= (*stats___0).max {
        sum = (sum as libc::c_ulong)
            .wrapping_add(
                (*((*stats___0).data).as_mut_ptr().offset(i as isize)).wrapping_mul(i),
            ) as uint64_t as uint64_t;
        i = i.wrapping_add(1);
    }
    return f128::f128::new(sum) / f128::f128::new((*stats___0).count);
}
pub unsafe extern "C" fn stats_stdev(
    mut stats___0: *mut stats,
    mut mean: f128::f128,
) -> f128::f128 {
    let mut sum: f128::f128 = f128::f128::ZERO;
    let mut i: uint64_t = 0;
    let mut tmp: f128::f128 = f128::f128::ZERO;
    let mut tmp___0: f128::f128 = f128::f128::ZERO;
    sum = f128::f128::new(0.0f64);
    if (*stats___0).count < 2 as libc::c_ulong {
        return f128::f128::new(0.0f64);
    }
    i = (*stats___0).min;
    while i <= (*stats___0).max {
        if *((*stats___0).data).as_mut_ptr().offset(i as isize) != 0 {
            tmp = powl(f128::f128::new(i) - mean, f128::f128::new(2 as libc::c_int));
            sum
                += tmp
                    * f128::f128::new(
                        *((*stats___0).data).as_mut_ptr().offset(i as isize),
                    );
        }
        i = i.wrapping_add(1);
    }
    tmp___0 = sqrtl(
        sum / f128::f128::new(((*stats___0).count).wrapping_sub(1 as libc::c_ulong)),
    );
    return tmp___0;
}
pub unsafe extern "C" fn stats_within_stdev(
    mut stats___0: *mut stats,
    mut mean: f128::f128,
    mut stdev: f128::f128,
    mut n: uint64_t,
) -> f128::f128 {
    let mut upper: f128::f128 = f128::f128::ZERO;
    let mut lower: f128::f128 = f128::f128::ZERO;
    let mut sum: uint64_t = 0;
    let mut i: uint64_t = 0;
    upper = mean + stdev * f128::f128::new(n);
    lower = mean - stdev * f128::f128::new(n);
    sum = 0 as libc::c_int as uint64_t;
    i = (*stats___0).min;
    while i <= (*stats___0).max {
        if f128::f128::new(i) >= lower {
            if f128::f128::new(i) <= upper {
                sum = (sum as libc::c_ulong)
                    .wrapping_add(*((*stats___0).data).as_mut_ptr().offset(i as isize))
                    as uint64_t as uint64_t;
            }
        }
        i = i.wrapping_add(1);
    }
    return f128::f128::new(sum) / f128::f128::new((*stats___0).count)
        * f128::f128::new(100 as libc::c_int);
}
pub unsafe extern "C" fn stats_percentile(
    mut stats___0: *mut stats,
    mut p: f128::f128,
) -> uint64_t {
    let mut rank: uint64_t = 0;
    let mut tmp: libc::c_double = 0.;
    let mut total: uint64_t = 0;
    let mut i: uint64_t = 0;
    tmp = round(
        (p / f128::f128::new(100.0f64) * f128::f128::new((*stats___0).count)
            + f128::f128::new(0.5f64))
            .to_f64()
            .unwrap(),
    );
    rank = tmp as uint64_t;
    total = 0 as libc::c_int as uint64_t;
    i = (*stats___0).min;
    while i <= (*stats___0).max {
        total = (total as libc::c_ulong)
            .wrapping_add(*((*stats___0).data).as_mut_ptr().offset(i as isize))
            as uint64_t as uint64_t;
        if total >= rank {
            return i;
        }
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int as uint64_t;
}
pub unsafe extern "C" fn stats_popcount(mut stats___0: *mut stats) -> uint64_t {
    let mut count: uint64_t = 0;
    let mut i: uint64_t = 0;
    count = 0 as libc::c_int as uint64_t;
    i = (*stats___0).min;
    while i <= (*stats___0).max {
        if *((*stats___0).data).as_mut_ptr().offset(i as isize) != 0 {
            count = count.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    return count;
}
pub unsafe extern "C" fn stats_value_at(
    mut stats___0: *mut stats,
    mut index: uint64_t,
    mut count: *mut uint64_t,
) -> uint64_t {
    let mut i: uint64_t = 0;
    let mut tmp: uint64_t = 0;
    *count = 0 as libc::c_int as uint64_t;
    i = (*stats___0).min;
    while i <= (*stats___0).max {
        if *((*stats___0).data).as_mut_ptr().offset(i as isize) != 0 {
            tmp = *count;
            *count = (*count).wrapping_add(1);
            if tmp == index {
                *count = *((*stats___0).data).as_mut_ptr().offset(i as isize);
                return i;
            }
        }
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int as uint64_t;
}
static mut addrlib: [luaL_Reg; 3] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"__tostring\0" as *const u8 as *const libc::c_char,
                func: Some(
                    script_addr_tostring
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"__gc\0" as *const u8 as *const libc::c_char,
                func: Some(
                    script_addr_gc as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: 0 as *const libc::c_void as *mut libc::c_void
                    as *const libc::c_char,
                func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
    ]
};
static mut statslib: [luaL_Reg; 4] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"__call\0" as *const u8 as *const libc::c_char,
                func: Some(
                    script_stats_call
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"__index\0" as *const u8 as *const libc::c_char,
                func: Some(
                    script_stats_index
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"__len\0" as *const u8 as *const libc::c_char,
                func: Some(
                    script_stats_len
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: 0 as *const libc::c_void as *mut libc::c_void
                    as *const libc::c_char,
                func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
    ]
};
static mut threadlib: [luaL_Reg; 3] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"__index\0" as *const u8 as *const libc::c_char,
                func: Some(
                    script_thread_index
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"__newindex\0" as *const u8 as *const libc::c_char,
                func: Some(
                    script_thread_newindex
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: 0 as *const libc::c_void as *mut libc::c_void
                    as *const libc::c_char,
                func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
    ]
};
pub unsafe extern "C" fn script_create(
    mut file: *mut libc::c_char,
    mut url: *mut libc::c_char,
    mut headers: *mut *mut libc::c_char,
) -> *mut lua_State {
    let mut L: *mut lua_State = 0 as *mut lua_State;
    let mut tmp: *mut lua_State = 0 as *mut lua_State;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut parts: http_parser_url = http_parser_url {
        field_set: 0,
        port: 0,
        field_data: [__anonstruct_field_data_753912357 {
            off: 0,
            len: 0,
        }; 7],
    };
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fields: [table_field; 4] = [table_field {
        name: 0 as *mut libc::c_char,
        type_0: 0,
        value: 0 as *mut libc::c_void,
    }; 4];
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut h: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cause: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___7: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_int = 0;
    tmp = luaL_newstate();
    L = tmp;
    luaL_openlibs(L);
    tmp___0 = luaL_loadstring(
        L,
        b"wrk = require \"wrk\"\0" as *const u8 as *const libc::c_char,
    );
    if tmp___0 != 0 {
        tmp___2 = 1 as libc::c_int;
    } else {
        tmp___1 = lua_pcall(L, 0 as libc::c_int, -(1 as libc::c_int), 0 as libc::c_int);
        if tmp___1 != 0 {
            tmp___2 = 1 as libc::c_int;
        } else {
            tmp___2 = 0 as libc::c_int;
        }
    }
    luaL_newmetatable(L, b"wrk.addr\0" as *const u8 as *const libc::c_char);
    luaL_register(L, 0 as *mut libc::c_void as *const libc::c_char, addrlib.as_ptr());
    luaL_newmetatable(L, b"wrk.stats\0" as *const u8 as *const libc::c_char);
    luaL_register(L, 0 as *mut libc::c_void as *const libc::c_char, statslib.as_ptr());
    luaL_newmetatable(L, b"wrk.thread\0" as *const u8 as *const libc::c_char);
    luaL_register(L, 0 as *mut libc::c_void as *const libc::c_char, threadlib.as_ptr());
    parts.field_set = 0 as libc::c_int as uint16_t;
    parts.port = 0 as libc::c_int as libc::c_ushort;
    parts.field_data[0 as libc::c_int as usize].off = 0 as libc::c_int as libc::c_ushort;
    parts.field_data[0 as libc::c_int as usize].len = 0 as libc::c_int as libc::c_ushort;
    parts.field_data[1 as libc::c_int as usize].off = 0 as libc::c_int as libc::c_ushort;
    parts.field_data[1 as libc::c_int as usize].len = 0 as libc::c_int as libc::c_ushort;
    parts.field_data[2 as libc::c_int as usize].off = 0 as libc::c_int as libc::c_ushort;
    parts.field_data[2 as libc::c_int as usize].len = 0 as libc::c_int as libc::c_ushort;
    parts.field_data[3 as libc::c_int as usize].off = 0 as libc::c_int as libc::c_ushort;
    parts.field_data[3 as libc::c_int as usize].len = 0 as libc::c_int as libc::c_ushort;
    parts.field_data[4 as libc::c_int as usize].off = 0 as libc::c_int as libc::c_ushort;
    parts.field_data[4 as libc::c_int as usize].len = 0 as libc::c_int as libc::c_ushort;
    parts.field_data[5 as libc::c_int as usize].off = 0 as libc::c_int as libc::c_ushort;
    parts.field_data[5 as libc::c_int as usize].len = 0 as libc::c_int as libc::c_ushort;
    parts.field_data[6 as libc::c_int as usize].off = 0 as libc::c_int as libc::c_ushort;
    parts.field_data[6 as libc::c_int as usize].len = 0 as libc::c_int as libc::c_ushort;
    script_parse_url(url, &mut parts);
    path = b"/\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    if parts.field_set as libc::c_int & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        path = url
            .offset(
                parts.field_data[3 as libc::c_int as usize].off as libc::c_int as isize,
            );
    }
    fields[0 as libc::c_int as usize]
        .name = b"lookup\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    fields[0 as libc::c_int as usize].type_0 = 6 as libc::c_int;
    fields[0 as libc::c_int as usize]
        .value = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>,
        *mut libc::c_void,
    >(Some(script_wrk_lookup as unsafe extern "C" fn(*mut lua_State) -> libc::c_int));
    fields[1 as libc::c_int as usize]
        .name = b"connect\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    fields[1 as libc::c_int as usize].type_0 = 6 as libc::c_int;
    fields[1 as libc::c_int as usize]
        .value = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>,
        *mut libc::c_void,
    >(Some(script_wrk_connect as unsafe extern "C" fn(*mut lua_State) -> libc::c_int));
    fields[2 as libc::c_int as usize]
        .name = b"path\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    fields[2 as libc::c_int as usize].type_0 = 4 as libc::c_int;
    fields[2 as libc::c_int as usize].value = path as *mut libc::c_void;
    fields[3 as libc::c_int as usize].name = 0 as *mut libc::c_void as *mut libc::c_char;
    fields[3 as libc::c_int as usize].type_0 = 0 as libc::c_int;
    fields[3 as libc::c_int as usize].value = 0 as *mut libc::c_void;
    lua_getfield(
        L,
        -(10002 as libc::c_int),
        b"wrk\0" as *const u8 as *const libc::c_char,
    );
    tmp___3 = push_url_part(L, url, &mut parts, UF_SCHEMA);
    set_field(
        L,
        4 as libc::c_int,
        b"scheme\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        tmp___3,
    );
    tmp___4 = push_url_part(L, url, &mut parts, UF_HOST);
    set_field(
        L,
        4 as libc::c_int,
        b"host\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        tmp___4,
    );
    tmp___5 = push_url_part(L, url, &mut parts, UF_PORT);
    set_field(
        L,
        4 as libc::c_int,
        b"port\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        tmp___5,
    );
    set_fields(L, 4 as libc::c_int, fields.as_mut_ptr() as *const table_field);
    lua_getfield(L, 4 as libc::c_int, b"headers\0" as *const u8 as *const libc::c_char);
    h = headers;
    while !(*h).is_null() {
        tmp___6 = strchr(*h as *const libc::c_char, ':' as i32);
        p = tmp___6;
        if !p.is_null() {
            if *p.offset(1 as libc::c_int as isize) as libc::c_int == 32 as libc::c_int {
                lua_pushlstring(
                    L,
                    *h as *const libc::c_char,
                    p.offset_from(*h) as libc::c_long as size_t,
                );
                lua_pushstring(
                    L,
                    p.offset(2 as libc::c_int as isize) as *const libc::c_char,
                );
                lua_settable(L, 5 as libc::c_int);
            }
        }
        h = h.offset(1);
    }
    lua_settop(L, -(6 as libc::c_int));
    if !file.is_null() {
        tmp___8 = luaL_loadfile(L, file as *const libc::c_char);
        if tmp___8 != 0 {
            tmp___7 = lua_tolstring(
                L,
                -(1 as libc::c_int),
                0 as *mut libc::c_void as *mut size_t,
            );
            cause = tmp___7;
            fprintf(
                stderr,
                b"%s: %s\n\0" as *const u8 as *const libc::c_char,
                file,
                cause,
            );
        } else {
            tmp___9 = lua_pcall(
                L,
                0 as libc::c_int,
                -(1 as libc::c_int),
                0 as libc::c_int,
            );
            if tmp___9 != 0 {
                tmp___7 = lua_tolstring(
                    L,
                    -(1 as libc::c_int),
                    0 as *mut libc::c_void as *mut size_t,
                );
                cause = tmp___7;
                fprintf(
                    stderr,
                    b"%s: %s\n\0" as *const u8 as *const libc::c_char,
                    file,
                    cause,
                );
            }
        }
    }
    return L;
}
pub unsafe extern "C" fn script_resolve(
    mut L: *mut lua_State,
    mut host: *mut libc::c_char,
    mut service: *mut libc::c_char,
) -> bool {
    let mut count: size_t = 0;
    let mut tmp: size_t = 0;
    lua_getfield(
        L,
        -(10002 as libc::c_int),
        b"wrk\0" as *const u8 as *const libc::c_char,
    );
    lua_getfield(
        L,
        -(1 as libc::c_int),
        b"resolve\0" as *const u8 as *const libc::c_char,
    );
    lua_pushstring(L, host as *const libc::c_char);
    lua_pushstring(L, service as *const libc::c_char);
    lua_call(L, 2 as libc::c_int, 0 as libc::c_int);
    lua_getfield(L, -(1 as libc::c_int), b"addrs\0" as *const u8 as *const libc::c_char);
    tmp = lua_objlen(L, -(1 as libc::c_int));
    count = tmp;
    lua_settop(L, -(3 as libc::c_int));
    return count > 0 as libc::c_ulong;
}
pub unsafe extern "C" fn script_push_thread(mut L: *mut lua_State, mut t: *mut thread) {
    let mut ptr: *mut *mut thread = 0 as *mut *mut thread;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = lua_newuserdata(L, ::std::mem::size_of::<*mut *mut thread>() as libc::c_ulong);
    ptr = tmp as *mut *mut thread;
    *ptr = t;
    lua_getfield(
        L,
        -(10000 as libc::c_int),
        b"wrk.thread\0" as *const u8 as *const libc::c_char,
    );
    lua_setmetatable(L, -(2 as libc::c_int));
}
pub unsafe extern "C" fn script_init(
    mut L: *mut lua_State,
    mut t: *mut thread,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut i: libc::c_int = 0;
    lua_getfield(
        (*t).L,
        -(10002 as libc::c_int),
        b"wrk\0" as *const u8 as *const libc::c_char,
    );
    script_push_thread((*t).L, t);
    lua_setfield(
        (*t).L,
        -(2 as libc::c_int),
        b"thread\0" as *const u8 as *const libc::c_char,
    );
    lua_getfield(
        L,
        -(10002 as libc::c_int),
        b"wrk\0" as *const u8 as *const libc::c_char,
    );
    lua_getfield(L, -(1 as libc::c_int), b"setup\0" as *const u8 as *const libc::c_char);
    script_push_thread(L, t);
    lua_call(L, 1 as libc::c_int, 0 as libc::c_int);
    lua_settop(L, -(2 as libc::c_int));
    lua_getfield(
        (*t).L,
        -(1 as libc::c_int),
        b"init\0" as *const u8 as *const libc::c_char,
    );
    lua_createtable((*t).L, 0 as libc::c_int, 0 as libc::c_int);
    i = 0 as libc::c_int;
    while i < argc {
        lua_pushstring((*t).L, *argv.offset(i as isize) as *const libc::c_char);
        lua_rawseti((*t).L, -(2 as libc::c_int), i);
        i += 1;
    }
    lua_call((*t).L, 1 as libc::c_int, 0 as libc::c_int);
    lua_settop((*t).L, -(2 as libc::c_int));
}
pub unsafe extern "C" fn script_delay(mut L: *mut lua_State) -> uint64_t {
    let mut delay: uint64_t = 0;
    let mut tmp: lua_Number = 0.;
    lua_getfield(
        L,
        -(10002 as libc::c_int),
        b"delay\0" as *const u8 as *const libc::c_char,
    );
    lua_call(L, 0 as libc::c_int, 1 as libc::c_int);
    tmp = lua_tonumber(L, -(1 as libc::c_int));
    delay = tmp as uint64_t;
    lua_settop(L, -(2 as libc::c_int));
    return delay;
}
pub unsafe extern "C" fn script_request(
    mut L: *mut lua_State,
    mut buf: *mut *mut libc::c_char,
    mut len: *mut size_t,
) {
    let mut pop: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    pop = 1 as libc::c_int;
    lua_getfield(
        L,
        -(10002 as libc::c_int),
        b"request\0" as *const u8 as *const libc::c_char,
    );
    tmp = lua_type(L, -(1 as libc::c_int));
    if !(tmp == 6 as libc::c_int) {
        lua_getfield(
            L,
            -(10002 as libc::c_int),
            b"wrk\0" as *const u8 as *const libc::c_char,
        );
        lua_getfield(
            L,
            -(1 as libc::c_int),
            b"request\0" as *const u8 as *const libc::c_char,
        );
        pop += 2 as libc::c_int;
    }
    lua_call(L, 0 as libc::c_int, 1 as libc::c_int);
    tmp___0 = lua_tolstring(L, -(1 as libc::c_int), len);
    str = tmp___0;
    tmp___1 = realloc(*buf as *mut libc::c_void, *len);
    *buf = tmp___1 as *mut libc::c_char;
    memcpy(*buf as *mut libc::c_void, str as *const libc::c_void, *len);
    lua_settop(L, -pop - 1 as libc::c_int);
}
pub unsafe extern "C" fn script_response(
    mut L: *mut lua_State,
    mut status: libc::c_int,
    mut headers: *mut buffer,
    mut body: *mut buffer,
) {
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    lua_getfield(
        L,
        -(10002 as libc::c_int),
        b"response\0" as *const u8 as *const libc::c_char,
    );
    lua_pushinteger(L, status as lua_Integer);
    lua_createtable(L, 0 as libc::c_int, 0 as libc::c_int);
    c = (*headers).buffer;
    while (c as libc::c_ulong) < (*headers).cursor as libc::c_ulong {
        c = buffer_pushlstring(L, c);
        c = buffer_pushlstring(L, c);
        lua_rawset(L, -(3 as libc::c_int));
    }
    lua_pushlstring(
        L,
        (*body).buffer as *const libc::c_char,
        ((*body).cursor).offset_from((*body).buffer) as libc::c_long as size_t,
    );
    lua_call(L, 3 as libc::c_int, 0 as libc::c_int);
    buffer_reset(headers);
    buffer_reset(body);
}
pub unsafe extern "C" fn script_is_function(
    mut L: *mut lua_State,
    mut name: *mut libc::c_char,
) -> bool {
    let mut is_function: bool = false;
    let mut tmp: libc::c_int = 0;
    lua_getfield(L, -(10002 as libc::c_int), name as *const libc::c_char);
    tmp = lua_type(L, -(1 as libc::c_int));
    is_function = tmp == 6 as libc::c_int;
    lua_settop(L, -(2 as libc::c_int));
    return is_function;
}
pub unsafe extern "C" fn script_is_static(mut L: *mut lua_State) -> bool {
    let mut tmp: bool = false;
    let mut tmp___0: libc::c_int = 0;
    tmp = script_is_function(
        L,
        b"request\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if tmp {
        tmp___0 = 0 as libc::c_int;
    } else {
        tmp___0 = 1 as libc::c_int;
    }
    return tmp___0 != 0;
}
pub unsafe extern "C" fn script_want_response(mut L: *mut lua_State) -> bool {
    let mut tmp: bool = false;
    tmp = script_is_function(
        L,
        b"response\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return tmp;
}
pub unsafe extern "C" fn script_has_delay(mut L: *mut lua_State) -> bool {
    let mut tmp: bool = false;
    tmp = script_is_function(
        L,
        b"delay\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return tmp;
}
pub unsafe extern "C" fn script_has_done(mut L: *mut lua_State) -> bool {
    let mut tmp: bool = false;
    tmp = script_is_function(
        L,
        b"done\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return tmp;
}
pub unsafe extern "C" fn script_header_done(
    mut L: *mut lua_State,
    mut buffer___0: *mut luaL_Buffer,
) {
    luaL_pushresult(buffer___0);
}
pub unsafe extern "C" fn script_summary(
    mut L: *mut lua_State,
    mut duration: uint64_t,
    mut requests: uint64_t,
    mut bytes: uint64_t,
) {
    let mut fields: [table_field; 4] = [table_field {
        name: 0 as *mut libc::c_char,
        type_0: 0,
        value: 0 as *mut libc::c_void,
    }; 4];
    fields[0 as libc::c_int as usize]
        .name = b"duration\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    fields[0 as libc::c_int as usize].type_0 = 3 as libc::c_int;
    fields[0 as libc::c_int as usize]
        .value = &mut duration as *mut uint64_t as *mut libc::c_void;
    fields[1 as libc::c_int as usize]
        .name = b"requests\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    fields[1 as libc::c_int as usize].type_0 = 3 as libc::c_int;
    fields[1 as libc::c_int as usize]
        .value = &mut requests as *mut uint64_t as *mut libc::c_void;
    fields[2 as libc::c_int as usize]
        .name = b"bytes\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    fields[2 as libc::c_int as usize].type_0 = 3 as libc::c_int;
    fields[2 as libc::c_int as usize]
        .value = &mut bytes as *mut uint64_t as *mut libc::c_void;
    fields[3 as libc::c_int as usize].name = 0 as *mut libc::c_void as *mut libc::c_char;
    fields[3 as libc::c_int as usize].type_0 = 0 as libc::c_int;
    fields[3 as libc::c_int as usize].value = 0 as *mut libc::c_void;
    lua_createtable(L, 0 as libc::c_int, 0 as libc::c_int);
    set_fields(L, 1 as libc::c_int, fields.as_mut_ptr() as *const table_field);
}
pub unsafe extern "C" fn script_errors(
    mut L: *mut lua_State,
    mut errors___0: *mut errors,
) {
    let mut e: [uint64_t; 5] = [0; 5];
    let mut fields: [table_field; 6] = [table_field {
        name: 0 as *mut libc::c_char,
        type_0: 0,
        value: 0 as *mut libc::c_void,
    }; 6];
    e[0 as libc::c_int as usize] = (*errors___0).connect as uint64_t;
    e[1 as libc::c_int as usize] = (*errors___0).read as uint64_t;
    e[2 as libc::c_int as usize] = (*errors___0).write as uint64_t;
    e[3 as libc::c_int as usize] = (*errors___0).status as uint64_t;
    e[4 as libc::c_int as usize] = (*errors___0).timeout as uint64_t;
    fields[0 as libc::c_int as usize]
        .name = b"connect\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    fields[0 as libc::c_int as usize].type_0 = 3 as libc::c_int;
    fields[0 as libc::c_int as usize]
        .value = &mut *e.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut uint64_t
        as *mut libc::c_void;
    fields[1 as libc::c_int as usize]
        .name = b"read\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    fields[1 as libc::c_int as usize].type_0 = 3 as libc::c_int;
    fields[1 as libc::c_int as usize]
        .value = &mut *e.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut uint64_t
        as *mut libc::c_void;
    fields[2 as libc::c_int as usize]
        .name = b"write\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    fields[2 as libc::c_int as usize].type_0 = 3 as libc::c_int;
    fields[2 as libc::c_int as usize]
        .value = &mut *e.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut uint64_t
        as *mut libc::c_void;
    fields[3 as libc::c_int as usize]
        .name = b"status\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    fields[3 as libc::c_int as usize].type_0 = 3 as libc::c_int;
    fields[3 as libc::c_int as usize]
        .value = &mut *e.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut uint64_t
        as *mut libc::c_void;
    fields[4 as libc::c_int as usize]
        .name = b"timeout\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    fields[4 as libc::c_int as usize].type_0 = 3 as libc::c_int;
    fields[4 as libc::c_int as usize]
        .value = &mut *e.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut uint64_t
        as *mut libc::c_void;
    fields[5 as libc::c_int as usize].name = 0 as *mut libc::c_void as *mut libc::c_char;
    fields[5 as libc::c_int as usize].type_0 = 0 as libc::c_int;
    fields[5 as libc::c_int as usize].value = 0 as *mut libc::c_void;
    lua_createtable(L, 0 as libc::c_int, 0 as libc::c_int);
    set_fields(L, 2 as libc::c_int, fields.as_mut_ptr() as *const table_field);
    lua_setfield(L, 1 as libc::c_int, b"errors\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn script_push_stats(mut L: *mut lua_State, mut s: *mut stats) {
    let mut ptr: *mut *mut stats = 0 as *mut *mut stats;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = lua_newuserdata(L, ::std::mem::size_of::<*mut *mut stats>() as libc::c_ulong);
    ptr = tmp as *mut *mut stats;
    *ptr = s;
    lua_getfield(
        L,
        -(10000 as libc::c_int),
        b"wrk.stats\0" as *const u8 as *const libc::c_char,
    );
    lua_setmetatable(L, -(2 as libc::c_int));
}
pub unsafe extern "C" fn script_done(
    mut L: *mut lua_State,
    mut latency: *mut stats,
    mut requests: *mut stats,
) {
    lua_getfield(
        L,
        -(10002 as libc::c_int),
        b"done\0" as *const u8 as *const libc::c_char,
    );
    lua_pushvalue(L, 1 as libc::c_int);
    script_push_stats(L, latency);
    script_push_stats(L, requests);
    lua_call(L, 3 as libc::c_int, 0 as libc::c_int);
    lua_settop(L, -(2 as libc::c_int));
}
unsafe extern "C" fn verify_request(mut parser: *mut http_parser) -> libc::c_int {
    let mut count: *mut size_t = 0 as *mut size_t;
    count = (*parser).data as *mut size_t;
    *count = (*count).wrapping_add(1);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn script_verify_request(mut L: *mut lua_State) -> size_t {
    let mut settings: http_parser_settings = http_parser_settings {
        on_message_begin: None,
        on_url: None,
        on_status: None,
        on_header_field: None,
        on_header_value: None,
        on_headers_complete: None,
        on_body: None,
        on_message_complete: None,
        on_chunk_header: None,
        on_chunk_complete: None,
    };
    let mut parser: http_parser = http_parser {
        type_0_flags_state_header_state_index_lenient_http_headers: [0; 4],
        nread: 0,
        content_length: 0,
        http_major: 0,
        http_minor: 0,
        status_code_method_http_errno_upgrade: [0; 4],
        data: 0 as *mut libc::c_void,
    };
    let mut request: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut count: size_t = 0;
    let mut parsed: size_t = 0;
    let mut tmp: size_t = 0;
    let mut err: http_errno = HPE_OK;
    let mut desc: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut msg: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    let mut line: libc::c_int = 0;
    let mut column: libc::c_int = 0;
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    settings.on_message_begin = None;
    settings.on_url = None;
    settings.on_status = None;
    settings.on_header_field = None;
    settings.on_header_value = None;
    settings.on_headers_complete = None;
    settings.on_body = None;
    settings
        .on_message_complete = Some(
        verify_request as unsafe extern "C" fn(*mut http_parser) -> libc::c_int,
    );
    settings.on_chunk_header = None;
    settings.on_chunk_complete = None;
    request = 0 as *mut libc::c_void as *mut libc::c_char;
    count = 0 as libc::c_int as size_t;
    script_request(L, &mut request, &mut len);
    http_parser_init(&mut parser, HTTP_REQUEST);
    parser.data = &mut count as *mut size_t as *mut libc::c_void;
    tmp = http_parser_execute(
        &mut parser,
        &mut settings as *mut http_parser_settings as *const http_parser_settings,
        request as *const libc::c_char,
        len,
    );
    parsed = tmp;
    's_175: {
        if !(parsed != len) {
            if !(count == 0 as libc::c_ulong) {
                break 's_175;
            }
        }
        err = parser.http_errno() as http_errno;
        tmp___0 = http_errno_description(err);
        desc = tmp___0;
        if err as libc::c_uint != 0 as libc::c_uint {
            tmp___1 = desc;
        } else {
            tmp___1 = b"incomplete request\0" as *const u8 as *const libc::c_char;
        }
        msg = tmp___1;
        line = 1 as libc::c_int;
        column = 1 as libc::c_int;
        c = request;
        while (c as libc::c_ulong) < request.offset(parsed as isize) as libc::c_ulong {
            column += 1;
            if *c as libc::c_int == 10 as libc::c_int {
                column = 1 as libc::c_int;
                line += 1;
            }
            c = c.offset(1);
        }
        fprintf(
            stderr,
            b"%s at %d:%d\n\0" as *const u8 as *const libc::c_char,
            msg,
            line,
            column,
        );
        exit(1 as libc::c_int);
    }
    return count;
}
unsafe extern "C" fn checkaddr(mut L: *mut lua_State) -> *mut addrinfo {
    let mut addr: *mut addrinfo = 0 as *mut addrinfo;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    tmp = luaL_checkudata(
        L,
        -(1 as libc::c_int),
        b"wrk.addr\0" as *const u8 as *const libc::c_char,
    );
    addr = tmp as *mut addrinfo;
    if addr as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp___1 = 1 as libc::c_int;
    } else {
        tmp___0 = luaL_argerror(
            L,
            1 as libc::c_int,
            b"`addr' expected\0" as *const u8 as *const libc::c_char,
        );
        if tmp___0 != 0 {
            tmp___1 = 1 as libc::c_int;
        } else {
            tmp___1 = 0 as libc::c_int;
        }
    }
    return addr;
}
pub unsafe extern "C" fn script_addr_copy(
    mut src: *mut addrinfo,
    mut dst: *mut addrinfo,
) {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    *dst = *src;
    tmp = zmalloc((*src).ai_addrlen as size_t);
    (*dst).ai_addr = tmp as *mut sockaddr;
    memcpy(
        (*dst).ai_addr as *mut libc::c_void,
        (*src).ai_addr as *const libc::c_void,
        (*src).ai_addrlen as size_t,
    );
}
pub unsafe extern "C" fn script_addr_clone(
    mut L: *mut lua_State,
    mut addr: *mut addrinfo,
) -> *mut addrinfo {
    let mut udata: *mut addrinfo = 0 as *mut addrinfo;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = lua_newuserdata(L, ::std::mem::size_of::<addrinfo>() as libc::c_ulong);
    udata = tmp as *mut addrinfo;
    lua_getfield(
        L,
        -(10000 as libc::c_int),
        b"wrk.addr\0" as *const u8 as *const libc::c_char,
    );
    lua_setmetatable(L, -(2 as libc::c_int));
    script_addr_copy(addr, udata);
    return udata;
}
unsafe extern "C" fn script_addr_tostring(mut L: *mut lua_State) -> libc::c_int {
    let mut addr: *mut addrinfo = 0 as *mut addrinfo;
    let mut tmp: *mut addrinfo = 0 as *mut addrinfo;
    let mut host: [libc::c_char; 1025] = [0; 1025];
    let mut service: [libc::c_char; 32] = [0; 32];
    let mut flags: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut msg: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___2: libc::c_int = 0;
    tmp = checkaddr(L);
    addr = tmp;
    flags = 3 as libc::c_int;
    tmp___0 = getnameinfo(
        (*addr).ai_addr as *const sockaddr,
        (*addr).ai_addrlen,
        host.as_mut_ptr(),
        1025 as libc::c_int as socklen_t,
        service.as_mut_ptr(),
        32 as libc::c_int as socklen_t,
        flags,
    );
    rc = tmp___0;
    if rc != 0 as libc::c_int {
        tmp___1 = gai_strerror(rc);
        msg = tmp___1;
        tmp___2 = luaL_error(
            L,
            b"addr tostring failed %s\0" as *const u8 as *const libc::c_char,
            msg,
        );
        return tmp___2;
    }
    lua_pushfstring(
        L,
        b"%s:%s\0" as *const u8 as *const libc::c_char,
        host.as_mut_ptr(),
        service.as_mut_ptr(),
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn script_addr_gc(mut L: *mut lua_State) -> libc::c_int {
    let mut addr: *mut addrinfo = 0 as *mut addrinfo;
    let mut tmp: *mut addrinfo = 0 as *mut addrinfo;
    tmp = checkaddr(L);
    addr = tmp;
    zfree((*addr).ai_addr as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn checkstats(mut L: *mut lua_State) -> *mut stats {
    let mut s: *mut *mut stats = 0 as *mut *mut stats;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    tmp = luaL_checkudata(
        L,
        1 as libc::c_int,
        b"wrk.stats\0" as *const u8 as *const libc::c_char,
    );
    s = tmp as *mut *mut stats;
    if s as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp___1 = 1 as libc::c_int;
    } else {
        tmp___0 = luaL_argerror(
            L,
            1 as libc::c_int,
            b"`stats' expected\0" as *const u8 as *const libc::c_char,
        );
        if tmp___0 != 0 {
            tmp___1 = 1 as libc::c_int;
        } else {
            tmp___1 = 0 as libc::c_int;
        }
    }
    return *s;
}
unsafe extern "C" fn script_stats_percentile(mut L: *mut lua_State) -> libc::c_int {
    let mut s: *mut stats = 0 as *mut stats;
    let mut tmp: *mut stats = 0 as *mut stats;
    let mut p: lua_Number = 0.;
    let mut tmp___0: lua_Number = 0.;
    let mut tmp___1: uint64_t = 0;
    tmp = checkstats(L);
    s = tmp;
    tmp___0 = luaL_checknumber(L, 2 as libc::c_int);
    p = tmp___0;
    tmp___1 = stats_percentile(s, f128::f128::new(p));
    lua_pushnumber(L, tmp___1 as lua_Number);
    return 1 as libc::c_int;
}
unsafe extern "C" fn script_stats_call(mut L: *mut lua_State) -> libc::c_int {
    let mut s: *mut stats = 0 as *mut stats;
    let mut tmp: *mut stats = 0 as *mut stats;
    let mut index___0: uint64_t = 0;
    let mut tmp___0: lua_Number = 0.;
    let mut count: uint64_t = 0;
    let mut tmp___1: uint64_t = 0;
    tmp = checkstats(L);
    s = tmp;
    tmp___0 = lua_tonumber(L, 2 as libc::c_int);
    index___0 = tmp___0 as uint64_t;
    tmp___1 = stats_value_at(s, index___0.wrapping_sub(1 as libc::c_ulong), &mut count);
    lua_pushnumber(L, tmp___1 as lua_Number);
    lua_pushnumber(L, count as lua_Number);
    return 2 as libc::c_int;
}
unsafe extern "C" fn script_stats_index(mut L: *mut lua_State) -> libc::c_int {
    let mut s: *mut stats = 0 as *mut stats;
    let mut tmp: *mut stats = 0 as *mut stats;
    let mut method: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: f128::f128 = f128::f128::ZERO;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: f128::f128 = f128::f128::ZERO;
    let mut tmp___6: f128::f128 = f128::f128::ZERO;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    tmp = checkstats(L);
    s = tmp;
    tmp___0 = lua_tolstring(L, 2 as libc::c_int, 0 as *mut libc::c_void as *mut size_t);
    method = tmp___0;
    tmp___1 = strcmp(b"min\0" as *const u8 as *const libc::c_char, method);
    if tmp___1 == 0 {
        lua_pushnumber(L, (*s).min as lua_Number);
    }
    tmp___2 = strcmp(b"max\0" as *const u8 as *const libc::c_char, method);
    if tmp___2 == 0 {
        lua_pushnumber(L, (*s).max as lua_Number);
    }
    tmp___4 = strcmp(b"mean\0" as *const u8 as *const libc::c_char, method);
    if tmp___4 == 0 {
        tmp___3 = stats_mean(s);
        lua_pushnumber(L, tmp___3.to_f64().unwrap());
    }
    tmp___7 = strcmp(b"stdev\0" as *const u8 as *const libc::c_char, method);
    if tmp___7 == 0 {
        tmp___5 = stats_mean(s);
        tmp___6 = stats_stdev(s, tmp___5);
        lua_pushnumber(L, tmp___6.to_f64().unwrap());
    }
    tmp___8 = strcmp(b"percentile\0" as *const u8 as *const libc::c_char, method);
    if tmp___8 == 0 {
        lua_pushcclosure(
            L,
            Some(
                script_stats_percentile
                    as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
            ),
            0 as libc::c_int,
        );
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn script_stats_len(mut L: *mut lua_State) -> libc::c_int {
    let mut s: *mut stats = 0 as *mut stats;
    let mut tmp: *mut stats = 0 as *mut stats;
    let mut tmp___0: uint64_t = 0;
    tmp = checkstats(L);
    s = tmp;
    tmp___0 = stats_popcount(s);
    lua_pushinteger(L, tmp___0 as lua_Integer);
    return 1 as libc::c_int;
}
unsafe extern "C" fn checkthread(mut L: *mut lua_State) -> *mut thread {
    let mut t: *mut *mut thread = 0 as *mut *mut thread;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    tmp = luaL_checkudata(
        L,
        1 as libc::c_int,
        b"wrk.thread\0" as *const u8 as *const libc::c_char,
    );
    t = tmp as *mut *mut thread;
    if t as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp___1 = 1 as libc::c_int;
    } else {
        tmp___0 = luaL_argerror(
            L,
            1 as libc::c_int,
            b"`thread' expected\0" as *const u8 as *const libc::c_char,
        );
        if tmp___0 != 0 {
            tmp___1 = 1 as libc::c_int;
        } else {
            tmp___1 = 0 as libc::c_int;
        }
    }
    return *t;
}
unsafe extern "C" fn script_thread_get(mut L: *mut lua_State) -> libc::c_int {
    let mut t: *mut thread = 0 as *mut thread;
    let mut tmp: *mut thread = 0 as *mut thread;
    let mut key: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    tmp = checkthread(L);
    t = tmp;
    tmp___0 = lua_tolstring(
        L,
        -(1 as libc::c_int),
        0 as *mut libc::c_void as *mut size_t,
    );
    key = tmp___0;
    lua_getfield((*t).L, -(10002 as libc::c_int), key);
    script_copy_value((*t).L, L, -(1 as libc::c_int));
    lua_settop((*t).L, -(2 as libc::c_int));
    return 1 as libc::c_int;
}
unsafe extern "C" fn script_thread_set(mut L: *mut lua_State) -> libc::c_int {
    let mut t: *mut thread = 0 as *mut thread;
    let mut tmp: *mut thread = 0 as *mut thread;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    tmp = checkthread(L);
    t = tmp;
    tmp___0 = lua_tolstring(
        L,
        -(2 as libc::c_int),
        0 as *mut libc::c_void as *mut size_t,
    );
    name = tmp___0;
    script_copy_value(L, (*t).L, -(1 as libc::c_int));
    lua_setfield((*t).L, -(10002 as libc::c_int), name);
    return 0 as libc::c_int;
}
unsafe extern "C" fn script_thread_stop(mut L: *mut lua_State) -> libc::c_int {
    let mut t: *mut thread = 0 as *mut thread;
    let mut tmp: *mut thread = 0 as *mut thread;
    tmp = checkthread(L);
    t = tmp;
    aeStop((*t).loop_0);
    return 0 as libc::c_int;
}
unsafe extern "C" fn script_thread_index(mut L: *mut lua_State) -> libc::c_int {
    let mut t: *mut thread = 0 as *mut thread;
    let mut tmp: *mut thread = 0 as *mut thread;
    let mut key: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    tmp = checkthread(L);
    t = tmp;
    tmp___0 = lua_tolstring(L, 2 as libc::c_int, 0 as *mut libc::c_void as *mut size_t);
    key = tmp___0;
    tmp___1 = strcmp(b"get\0" as *const u8 as *const libc::c_char, key);
    if tmp___1 == 0 {
        lua_pushcclosure(
            L,
            Some(
                script_thread_get as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
            ),
            0 as libc::c_int,
        );
    }
    tmp___2 = strcmp(b"set\0" as *const u8 as *const libc::c_char, key);
    if tmp___2 == 0 {
        lua_pushcclosure(
            L,
            Some(
                script_thread_set as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
            ),
            0 as libc::c_int,
        );
    }
    tmp___3 = strcmp(b"stop\0" as *const u8 as *const libc::c_char, key);
    if tmp___3 == 0 {
        lua_pushcclosure(
            L,
            Some(
                script_thread_stop as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
            ),
            0 as libc::c_int,
        );
    }
    tmp___4 = strcmp(b"addr\0" as *const u8 as *const libc::c_char, key);
    if tmp___4 == 0 {
        script_addr_clone(L, (*t).addr);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn script_thread_newindex(mut L: *mut lua_State) -> libc::c_int {
    let mut t: *mut thread = 0 as *mut thread;
    let mut tmp: *mut thread = 0 as *mut thread;
    let mut key: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut addr: *mut addrinfo = 0 as *mut addrinfo;
    let mut tmp___1: *mut addrinfo = 0 as *mut addrinfo;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___5: libc::c_int = 0;
    tmp = checkthread(L);
    t = tmp;
    tmp___0 = lua_tolstring(
        L,
        -(2 as libc::c_int),
        0 as *mut libc::c_void as *mut size_t,
    );
    key = tmp___0;
    tmp___5 = strcmp(b"addr\0" as *const u8 as *const libc::c_char, key);
    if tmp___5 != 0 {
        tmp___3 = lua_type(L, -(1 as libc::c_int));
        tmp___4 = lua_typename(L, tmp___3);
        luaL_error(
            L,
            b"cannot set '%s' on thread\0" as *const u8 as *const libc::c_char,
            tmp___4,
        );
    } else {
        tmp___1 = checkaddr(L);
        addr = tmp___1;
        if !((*t).addr).is_null() {
            zfree((*(*t).addr).ai_addr as *mut libc::c_void);
        }
        tmp___2 = zrealloc(
            (*t).addr as *mut libc::c_void,
            ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
        );
        (*t).addr = tmp___2 as *mut addrinfo;
        script_addr_copy(addr, (*t).addr);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn script_wrk_lookup(mut L: *mut lua_State) -> libc::c_int {
    let mut addrs: *mut addrinfo = 0 as *mut addrinfo;
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
    let mut rc: libc::c_int = 0;
    let mut index___0: libc::c_int = 0;
    let mut host: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut service: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut msg: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    let mut addr: *mut addrinfo = 0 as *mut addrinfo;
    let mut tmp___2: libc::c_int = 0;
    hints.ai_flags = 0 as libc::c_int;
    hints.ai_family = 0 as libc::c_int;
    hints.ai_socktype = 1 as libc::c_int;
    hints.ai_protocol = 0 as libc::c_int;
    hints.ai_addrlen = 0 as libc::c_uint;
    hints.ai_addr = 0 as *mut sockaddr;
    hints.ai_canonname = 0 as *mut libc::c_char;
    hints.ai_next = 0 as *mut addrinfo;
    index___0 = 1 as libc::c_int;
    tmp = lua_tolstring(L, -(2 as libc::c_int), 0 as *mut libc::c_void as *mut size_t);
    host = tmp;
    tmp___0 = lua_tolstring(
        L,
        -(1 as libc::c_int),
        0 as *mut libc::c_void as *mut size_t,
    );
    service = tmp___0;
    rc = getaddrinfo(
        host,
        service,
        &mut hints as *mut addrinfo as *const addrinfo,
        &mut addrs as *mut *mut addrinfo,
    );
    if rc != 0 as libc::c_int {
        tmp___1 = gai_strerror(rc);
        msg = tmp___1;
        fprintf(
            stderr,
            b"unable to resolve %s:%s %s\n\0" as *const u8 as *const libc::c_char,
            host,
            service,
            msg,
        );
        exit(1 as libc::c_int);
    }
    lua_createtable(L, 0 as libc::c_int, 0 as libc::c_int);
    addr = addrs;
    while addr as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        script_addr_clone(L, addr);
        tmp___2 = index___0;
        index___0 += 1;
        lua_rawseti(L, -(2 as libc::c_int), tmp___2);
        addr = (*addr).ai_next;
    }
    freeaddrinfo(addrs);
    return 1 as libc::c_int;
}
unsafe extern "C" fn script_wrk_connect(mut L: *mut lua_State) -> libc::c_int {
    let mut addr: *mut addrinfo = 0 as *mut addrinfo;
    let mut tmp: *mut addrinfo = 0 as *mut addrinfo;
    let mut fd: libc::c_int = 0;
    let mut connected: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = checkaddr(L);
    addr = tmp;
    connected = 0 as libc::c_int;
    fd = socket((*addr).ai_family, (*addr).ai_socktype, (*addr).ai_protocol);
    if fd != -(1 as libc::c_int) {
        tmp___0 = connect(fd, (*addr).ai_addr as *const sockaddr, (*addr).ai_addrlen);
        connected = (tmp___0 == 0 as libc::c_int) as libc::c_int;
        close(fd);
    }
    lua_pushboolean(L, connected);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn script_copy_value(
    mut src: *mut lua_State,
    mut dst: *mut lua_State,
    mut index___0: libc::c_int,
) {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: lua_Number = 0.;
    let mut tmp___2: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: *const libc::c_char = 0 as *const libc::c_char;
    tmp = lua_type(src, index___0);
    match tmp {
        1 => {
            tmp___0 = lua_toboolean(src, index___0);
            lua_pushboolean(dst, tmp___0);
        }
        0 => {
            lua_pushnil(dst);
        }
        3 => {
            tmp___1 = lua_tonumber(src, index___0);
            lua_pushnumber(dst, tmp___1);
        }
        4 => {
            tmp___2 = lua_tolstring(
                src,
                index___0,
                0 as *mut libc::c_void as *mut size_t,
            );
            lua_pushstring(dst, tmp___2);
        }
        5 => {
            lua_createtable(dst, 0 as libc::c_int, 0 as libc::c_int);
            lua_pushnil(src);
            loop {
                tmp___3 = lua_next(src, index___0 - 1 as libc::c_int);
                if tmp___3 == 0 {
                    break;
                }
                script_copy_value(src, dst, -(2 as libc::c_int));
                script_copy_value(src, dst, -(1 as libc::c_int));
                lua_settable(dst, -(3 as libc::c_int));
                lua_settop(src, -(2 as libc::c_int));
            }
            lua_settop(src, -(2 as libc::c_int));
        }
        _ => {
            tmp___4 = lua_type(src, index___0);
            tmp___5 = lua_typename(src, tmp___4);
            luaL_error(
                src,
                b"cannot transfer '%s' to thread\0" as *const u8 as *const libc::c_char,
                tmp___5,
            );
        }
    };
}
pub unsafe extern "C" fn script_parse_url(
    mut url: *mut libc::c_char,
    mut parts: *mut http_parser_url,
) -> libc::c_int {
    let mut tmp: size_t = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = strlen(url as *const libc::c_char);
    tmp___0 = http_parser_parse_url(
        url as *const libc::c_char,
        tmp,
        0 as libc::c_int,
        parts,
    );
    if tmp___0 == 0 {
        if (*parts).field_set as libc::c_int & 1 as libc::c_int == 0 {
            return 0 as libc::c_int;
        }
        if (*parts).field_set as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int
            == 0
        {
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn push_url_part(
    mut L: *mut lua_State,
    mut url: *mut libc::c_char,
    mut parts: *mut http_parser_url,
    mut field: http_parser_url_fields,
) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut off: uint16_t = 0;
    let mut len: uint16_t = 0;
    if (*parts).field_set as libc::c_int & (1 as libc::c_int) << field as libc::c_uint
        != 0
    {
        tmp = 4 as libc::c_int;
    } else {
        tmp = 0 as libc::c_int;
    }
    type_0 = tmp;
    match type_0 {
        4 => {
            off = (*parts).field_data[field as usize].off;
            len = (*parts).field_data[field as usize].len;
            lua_pushlstring(
                L,
                url.offset(off as libc::c_int as isize) as *const libc::c_char,
                len as size_t,
            );
        }
        0 => {
            lua_pushnil(L);
        }
        _ => {}
    }
    return type_0;
}
unsafe extern "C" fn set_field(
    mut L: *mut lua_State,
    mut index___0: libc::c_int,
    mut field: *mut libc::c_char,
    mut type_0: libc::c_int,
) {
    lua_setfield(L, index___0, field as *const libc::c_char);
}
unsafe extern "C" fn set_fields(
    mut L: *mut lua_State,
    mut index___0: libc::c_int,
    mut fields: *const table_field,
) {
    let mut i: libc::c_int = 0;
    let mut f: table_field = table_field {
        name: 0 as *mut libc::c_char,
        type_0: 0,
        value: 0 as *mut libc::c_void,
    };
    let mut tmp: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !((*fields.offset(i as isize)).name).is_null() {
        f = *fields.offset(i as isize);
        if f.value as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            tmp = 0 as libc::c_int;
        } else {
            tmp = f.type_0;
        }
        match tmp {
            6 => {
                lua_pushcclosure(
                    L,
                    ::std::mem::transmute::<
                        *mut libc::c_void,
                        Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>,
                    >(f.value),
                    0 as libc::c_int,
                );
            }
            3 => {
                lua_pushinteger(L, *(f.value as *mut lua_Integer));
            }
            4 => {
                lua_pushstring(L, f.value as *const libc::c_char);
            }
            0 => {
                lua_pushnil(L);
            }
            _ => {}
        }
        lua_setfield(L, index___0, f.name as *const libc::c_char);
        i += 1;
    }
}
pub unsafe extern "C" fn buffer_append(
    mut b: *mut buffer,
    mut data: *const libc::c_char,
    mut len: size_t,
) {
    let mut used: size_t = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    used = ((*b).cursor).offset_from((*b).buffer) as libc::c_long as size_t;
    while used.wrapping_add(len).wrapping_add(1 as libc::c_ulong) >= (*b).length {
        (*b)
            .length = ((*b).length as libc::c_ulong).wrapping_add(1024 as libc::c_ulong)
            as size_t as size_t;
        tmp = realloc((*b).buffer as *mut libc::c_void, (*b).length);
        (*b).buffer = tmp as *mut libc::c_char;
        (*b).cursor = ((*b).buffer).offset(used as isize);
    }
    memcpy((*b).cursor as *mut libc::c_void, data as *const libc::c_void, len);
    (*b).cursor = ((*b).cursor).offset(len as isize);
}
pub unsafe extern "C" fn buffer_reset(mut b: *mut buffer) {
    (*b).cursor = (*b).buffer;
}
pub unsafe extern "C" fn buffer_pushlstring(
    mut L: *mut lua_State,
    mut start: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = strchr(start as *const libc::c_char, 0 as libc::c_int);
    end = tmp;
    lua_pushlstring(
        L,
        start as *const libc::c_char,
        end.offset_from(start) as libc::c_long as size_t,
    );
    return end.offset(1 as libc::c_int as isize);
}
pub static mut time_units_us: units = {
    let mut init = __anonstruct_units_255056401 {
        scale: 1000 as libc::c_int,
        base: b"us\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        units: [



        ],
    };
    init
};
pub static mut time_units_s: units = {
    let mut init = __anonstruct_units_255056401 {
        scale: 60 as libc::c_int,
        base: b"s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        units: [



        ],
    };
    init
};
pub static mut binary_units: units = {
    let mut init = __anonstruct_units_255056401 {
        scale: 1024 as libc::c_int,
        base: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        units: [






        ],
    };
    init
};
pub static mut metric_units: units = {
    let mut init = __anonstruct_units_255056401 {
        scale: 1000 as libc::c_int,
        base: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        units: [






        ],
    };
    init
};
unsafe extern "C" fn format_units(
    mut n: f128::f128,
    mut m: *mut units,
    mut p: libc::c_int,
) -> *mut libc::c_char {
    let mut amt: f128::f128 = f128::f128::ZERO;
    let mut scale: f128::f128 = f128::f128::ZERO;
    let mut unit: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    amt = n;
    unit = (*m).base;
    msg = 0 as *mut libc::c_void as *mut libc::c_char;
    scale = f128::f128::new((*m).scale as libc::c_double * 0.85f64);
    i = 0 as libc::c_int;
    while !(*((*m).units).as_mut_ptr().offset((i + 1 as libc::c_int) as isize)).is_null()
    {
        if !(amt >= scale) {
            break;
        }
        amt /= f128::f128::new((*m).scale);
        unit = *((*m).units).as_mut_ptr().offset(i as isize);
        i += 1;
    }
    aprintf(
        &mut msg as *mut *mut libc::c_char,
        b"%.*Lf%s\0" as *const u8 as *const libc::c_char,
        p,
        amt,
        unit,
    );
    return msg;
}
unsafe extern "C" fn scan_units(
    mut s: *mut libc::c_char,
    mut n: *mut uint64_t,
    mut m: *mut units,
) -> libc::c_int {
    let mut base: uint64_t = 0;
    let mut scale: uint64_t = 0;
    let mut unit: [libc::c_char; 3] = [0; 3];
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    scale = 1 as libc::c_int as uint64_t;
    unit[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    unit[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    unit[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    c = sscanf(
        s as *const libc::c_char,
        b"%lu%2s\0" as *const u8 as *const libc::c_char,
        &mut base as *mut uint64_t,
        unit.as_mut_ptr(),
    );
    if c < 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if c == 2 as libc::c_int {
        tmp___0 = strncasecmp(
            unit.as_mut_ptr() as *const libc::c_char,
            (*m).base as *const libc::c_char,
            3 as libc::c_int as size_t,
        );
        if tmp___0 != 0 {
            i = 0 as libc::c_int;
            while *((*m).units).as_mut_ptr().offset(i as isize) as libc::c_ulong
                != 0 as *mut libc::c_void as libc::c_ulong
            {
                scale = (scale as libc::c_ulong).wrapping_mul((*m).scale as uint64_t)
                    as uint64_t as uint64_t;
                tmp = strncasecmp(
                    unit.as_mut_ptr() as *const libc::c_char,
                    *((*m).units).as_mut_ptr().offset(i as isize) as *const libc::c_char,
                    3 as libc::c_int as size_t,
                );
                if tmp == 0 {
                    break;
                }
                i += 1;
            }
            if *((*m).units).as_mut_ptr().offset(i as isize) as libc::c_ulong
                == 0 as *mut libc::c_void as libc::c_ulong
            {
                return -(1 as libc::c_int);
            }
        }
    }
    *n = base.wrapping_mul(scale);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn format_binary(mut n: f128::f128) -> *mut libc::c_char {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = format_units(n, &mut binary_units, 2 as libc::c_int);
    return tmp;
}
pub unsafe extern "C" fn format_metric(mut n: f128::f128) -> *mut libc::c_char {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = format_units(n, &mut metric_units, 2 as libc::c_int);
    return tmp;
}
pub unsafe extern "C" fn format_time_us(mut n: f128::f128) -> *mut libc::c_char {
    let mut units___0: *mut units = 0 as *mut units;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    units___0 = &mut time_units_us;
    if n >= f128::f128::new(1000000.0f64) {
        n /= f128::f128::new(1000000.0f64);
        units___0 = &mut time_units_s;
    }
    tmp = format_units(n, units___0, 2 as libc::c_int);
    return tmp;
}
pub unsafe extern "C" fn format_time_s(mut n: f128::f128) -> *mut libc::c_char {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = format_units(n, &mut time_units_s, 0 as libc::c_int);
    return tmp;
}
pub unsafe extern "C" fn scan_metric(
    mut s: *mut libc::c_char,
    mut n: *mut uint64_t,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = scan_units(s, n, &mut metric_units);
    return tmp;
}
pub unsafe extern "C" fn scan_time(
    mut s: *mut libc::c_char,
    mut n: *mut uint64_t,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = scan_units(s, n, &mut time_units_s);
    return tmp;
}
unsafe extern "C" fn aeApiCreate(mut eventLoop: *mut aeEventLoop) -> libc::c_int {
    let mut state: *mut aeApiState = 0 as *mut aeApiState;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = zmalloc(::std::mem::size_of::<aeApiState>() as libc::c_ulong);
    state = tmp as *mut aeApiState;
    if state.is_null() {
        return -(1 as libc::c_int);
    }
    tmp___0 = zmalloc(
        (::std::mem::size_of::<epoll_event>() as libc::c_ulong)
            .wrapping_mul((*eventLoop).setsize as libc::c_ulong),
    );
    (*state).events = tmp___0 as *mut epoll_event;
    if ((*state).events).is_null() {
        zfree(state as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    (*state).epfd = epoll_create(1024 as libc::c_int);
    if (*state).epfd == -(1 as libc::c_int) {
        zfree((*state).events as *mut libc::c_void);
        zfree(state as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    (*eventLoop).apidata = state as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn aeApiResize(
    mut eventLoop: *mut aeEventLoop,
    mut setsize: libc::c_int,
) -> libc::c_int {
    let mut state: *mut aeApiState = 0 as *mut aeApiState;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    state = (*eventLoop).apidata as *mut aeApiState;
    tmp = zrealloc(
        (*state).events as *mut libc::c_void,
        (::std::mem::size_of::<epoll_event>() as libc::c_ulong)
            .wrapping_mul(setsize as libc::c_ulong),
    );
    (*state).events = tmp as *mut epoll_event;
    return 0 as libc::c_int;
}
unsafe extern "C" fn aeApiFree(mut eventLoop: *mut aeEventLoop) {
    let mut state: *mut aeApiState = 0 as *mut aeApiState;
    state = (*eventLoop).apidata as *mut aeApiState;
    close((*state).epfd);
    zfree((*state).events as *mut libc::c_void);
    zfree(state as *mut libc::c_void);
}
unsafe extern "C" fn aeApiAddEvent(
    mut eventLoop: *mut aeEventLoop,
    mut fd: libc::c_int,
    mut mask: libc::c_int,
) -> libc::c_int {
    let mut state: *mut aeApiState = 0 as *mut aeApiState;
    let mut ee: epoll_event = epoll_event {
        events: 0,
        data: epoll_data {
            ptr: 0 as *mut libc::c_void,
        },
    };
    let mut op: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    state = (*eventLoop).apidata as *mut aeApiState;
    ee.events = 0 as libc::c_int as uint32_t;
    ee.data.ptr = 0 as *mut libc::c_void;
    if (*((*eventLoop).events).offset(fd as isize)).mask == 0 as libc::c_int {
        tmp = 1 as libc::c_int;
    } else {
        tmp = 3 as libc::c_int;
    }
    op = tmp;
    ee.events = 0 as libc::c_int as uint32_t;
    mask |= (*((*eventLoop).events).offset(fd as isize)).mask;
    if mask & 1 as libc::c_int != 0 {
        ee.events |= 1 as libc::c_uint;
    }
    if mask & 2 as libc::c_int != 0 {
        ee.events |= 4 as libc::c_uint;
    }
    ee.data.fd = fd;
    tmp___0 = epoll_ctl((*state).epfd, op, fd, &mut ee);
    if tmp___0 == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn aeApiDelEvent(
    mut eventLoop: *mut aeEventLoop,
    mut fd: libc::c_int,
    mut delmask: libc::c_int,
) {
    let mut state: *mut aeApiState = 0 as *mut aeApiState;
    let mut ee: epoll_event = epoll_event {
        events: 0,
        data: epoll_data {
            ptr: 0 as *mut libc::c_void,
        },
    };
    let mut mask: libc::c_int = 0;
    state = (*eventLoop).apidata as *mut aeApiState;
    ee.events = 0 as libc::c_int as uint32_t;
    ee.data.ptr = 0 as *mut libc::c_void;
    mask = (*((*eventLoop).events).offset(fd as isize)).mask & !delmask;
    ee.events = 0 as libc::c_int as uint32_t;
    if mask & 1 as libc::c_int != 0 {
        ee.events |= 1 as libc::c_uint;
    }
    if mask & 2 as libc::c_int != 0 {
        ee.events |= 4 as libc::c_uint;
    }
    ee.data.fd = fd;
    if mask != 0 as libc::c_int {
        epoll_ctl((*state).epfd, 3 as libc::c_int, fd, &mut ee);
    } else {
        epoll_ctl((*state).epfd, 2 as libc::c_int, fd, &mut ee);
    };
}
unsafe extern "C" fn aeApiPoll(
    mut eventLoop: *mut aeEventLoop,
    mut tvp: *mut timeval,
) -> libc::c_int {
    let mut state: *mut aeApiState = 0 as *mut aeApiState;
    let mut retval: libc::c_int = 0;
    let mut numevents: libc::c_int = 0;
    let mut tmp: __time_t = 0;
    let mut j: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut e: *mut epoll_event = 0 as *mut epoll_event;
    state = (*eventLoop).apidata as *mut aeApiState;
    numevents = 0 as libc::c_int;
    if !tvp.is_null() {
        tmp = (*tvp).tv_sec * 1000 as libc::c_long
            + (*tvp).tv_usec / 1000 as libc::c_long;
    } else {
        tmp = -(1 as libc::c_int) as __time_t;
    }
    retval = epoll_wait(
        (*state).epfd,
        (*state).events,
        (*eventLoop).setsize,
        tmp as libc::c_int,
    );
    if retval > 0 as libc::c_int {
        numevents = retval;
        j = 0 as libc::c_int;
        while j < numevents {
            mask = 0 as libc::c_int;
            e = ((*state).events).offset(j as isize);
            if (*e).events & 1 as libc::c_uint != 0 {
                mask |= 1 as libc::c_int;
            }
            if (*e).events & 4 as libc::c_uint != 0 {
                mask |= 2 as libc::c_int;
            }
            if (*e).events & 8 as libc::c_uint != 0 {
                mask |= 2 as libc::c_int;
            }
            if (*e).events & 16 as libc::c_uint != 0 {
                mask |= 2 as libc::c_int;
            }
            (*((*eventLoop).fired).offset(j as isize)).fd = (*e).data.fd;
            (*((*eventLoop).fired).offset(j as isize)).mask = mask;
            j += 1;
        }
    }
    return numevents;
}
unsafe extern "C" fn aeApiName() -> *mut libc::c_char {
    return b"epoll\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
pub unsafe extern "C" fn aeCreateEventLoop(
    mut setsize: libc::c_int,
) -> *mut aeEventLoop {
    let mut eventLoop: *mut aeEventLoop = 0 as *mut aeEventLoop;
    let mut i: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: libc::c_int = 0;
    tmp = zmalloc(::std::mem::size_of::<aeEventLoop>() as libc::c_ulong);
    eventLoop = tmp as *mut aeEventLoop;
    if !(eventLoop as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
        tmp___0 = zmalloc(
            (::std::mem::size_of::<aeFileEvent>() as libc::c_ulong)
                .wrapping_mul(setsize as libc::c_ulong),
        );
        (*eventLoop).events = tmp___0 as *mut aeFileEvent;
        tmp___1 = zmalloc(
            (::std::mem::size_of::<aeFiredEvent>() as libc::c_ulong)
                .wrapping_mul(setsize as libc::c_ulong),
        );
        (*eventLoop).fired = tmp___1 as *mut aeFiredEvent;
        if !((*eventLoop).events as libc::c_ulong
            == 0 as *mut libc::c_void as libc::c_ulong)
        {
            if !((*eventLoop).fired as libc::c_ulong
                == 0 as *mut libc::c_void as libc::c_ulong)
            {
                (*eventLoop).setsize = setsize;
                (*eventLoop).lastTime = time(0 as *mut libc::c_void as *mut time_t);
                (*eventLoop).timeEventHead = 0 as *mut libc::c_void as *mut aeTimeEvent;
                (*eventLoop).timeEventNextId = 0 as libc::c_longlong;
                (*eventLoop).stop = 0 as libc::c_int;
                (*eventLoop).maxfd = -(1 as libc::c_int);
                (*eventLoop)
                    .beforesleep = ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<aeBeforeSleepProc>,
                >(0 as *mut libc::c_void);
                tmp___2 = aeApiCreate(eventLoop);
                if !(tmp___2 == -(1 as libc::c_int)) {
                    i = 0 as libc::c_int;
                    while i < setsize {
                        (*((*eventLoop).events).offset(i as isize))
                            .mask = 0 as libc::c_int;
                        i += 1;
                    }
                    return eventLoop;
                }
            }
        }
    }
    if !eventLoop.is_null() {
        zfree((*eventLoop).events as *mut libc::c_void);
        zfree((*eventLoop).fired as *mut libc::c_void);
        zfree(eventLoop as *mut libc::c_void);
    }
    return 0 as *mut libc::c_void as *mut aeEventLoop;
}
pub unsafe extern "C" fn aeGetSetSize(mut eventLoop: *mut aeEventLoop) -> libc::c_int {
    return (*eventLoop).setsize;
}
pub unsafe extern "C" fn aeResizeSetSize(
    mut eventLoop: *mut aeEventLoop,
    mut setsize: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    if setsize == (*eventLoop).setsize {
        return 0 as libc::c_int;
    }
    if (*eventLoop).maxfd >= setsize {
        return -(1 as libc::c_int);
    }
    tmp = aeApiResize(eventLoop, setsize);
    if tmp == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    tmp___0 = zrealloc(
        (*eventLoop).events as *mut libc::c_void,
        (::std::mem::size_of::<aeFileEvent>() as libc::c_ulong)
            .wrapping_mul(setsize as libc::c_ulong),
    );
    (*eventLoop).events = tmp___0 as *mut aeFileEvent;
    tmp___1 = zrealloc(
        (*eventLoop).fired as *mut libc::c_void,
        (::std::mem::size_of::<aeFiredEvent>() as libc::c_ulong)
            .wrapping_mul(setsize as libc::c_ulong),
    );
    (*eventLoop).fired = tmp___1 as *mut aeFiredEvent;
    (*eventLoop).setsize = setsize;
    i = (*eventLoop).maxfd + 1 as libc::c_int;
    while i < setsize {
        (*((*eventLoop).events).offset(i as isize)).mask = 0 as libc::c_int;
        i += 1;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn aeDeleteEventLoop(mut eventLoop: *mut aeEventLoop) {
    aeApiFree(eventLoop);
    zfree((*eventLoop).events as *mut libc::c_void);
    zfree((*eventLoop).fired as *mut libc::c_void);
    zfree(eventLoop as *mut libc::c_void);
}
pub unsafe extern "C" fn aeStop(mut eventLoop: *mut aeEventLoop) {
    (*eventLoop).stop = 1 as libc::c_int;
}
pub unsafe extern "C" fn aeCreateFileEvent(
    mut eventLoop: *mut aeEventLoop,
    mut fd: libc::c_int,
    mut mask: libc::c_int,
    mut proc_0: Option::<aeFileProc>,
    mut clientData: *mut libc::c_void,
) -> libc::c_int {
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut fe: *mut aeFileEvent = 0 as *mut aeFileEvent;
    let mut tmp___0: libc::c_int = 0;
    if fd >= (*eventLoop).setsize {
        tmp = __errno_location();
        *tmp = 34 as libc::c_int;
        return -(1 as libc::c_int);
    }
    fe = ((*eventLoop).events).offset(fd as isize);
    tmp___0 = aeApiAddEvent(eventLoop, fd, mask);
    if tmp___0 == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    (*fe).mask |= mask;
    if mask & 1 as libc::c_int != 0 {
        (*fe).rfileProc = proc_0;
    }
    if mask & 2 as libc::c_int != 0 {
        (*fe).wfileProc = proc_0;
    }
    (*fe).clientData = clientData;
    if fd > (*eventLoop).maxfd {
        (*eventLoop).maxfd = fd;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn aeDeleteFileEvent(
    mut eventLoop: *mut aeEventLoop,
    mut fd: libc::c_int,
    mut mask: libc::c_int,
) {
    let mut fe: *mut aeFileEvent = 0 as *mut aeFileEvent;
    let mut j: libc::c_int = 0;
    if fd >= (*eventLoop).setsize {
        return;
    }
    fe = ((*eventLoop).events).offset(fd as isize);
    if (*fe).mask == 0 as libc::c_int {
        return;
    }
    aeApiDelEvent(eventLoop, fd, mask);
    (*fe).mask &= !mask;
    if fd == (*eventLoop).maxfd {
        if (*fe).mask == 0 as libc::c_int {
            j = (*eventLoop).maxfd - 1 as libc::c_int;
            while j >= 0 as libc::c_int {
                if (*((*eventLoop).events).offset(j as isize)).mask != 0 as libc::c_int {
                    break;
                }
                j -= 1;
            }
            (*eventLoop).maxfd = j;
        }
    }
}
pub unsafe extern "C" fn aeGetFileEvents(
    mut eventLoop: *mut aeEventLoop,
    mut fd: libc::c_int,
) -> libc::c_int {
    let mut fe: *mut aeFileEvent = 0 as *mut aeFileEvent;
    if fd >= (*eventLoop).setsize {
        return 0 as libc::c_int;
    }
    fe = ((*eventLoop).events).offset(fd as isize);
    return (*fe).mask;
}
unsafe extern "C" fn aeGetTime(
    mut seconds: *mut libc::c_long,
    mut milliseconds: *mut libc::c_long,
) {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut tv as *mut timeval, 0 as *mut libc::c_void);
    *seconds = tv.tv_sec;
    *milliseconds = tv.tv_usec / 1000 as libc::c_long;
}
unsafe extern "C" fn aeAddMillisecondsToNow(
    mut milliseconds: libc::c_longlong,
    mut sec: *mut libc::c_long,
    mut ms: *mut libc::c_long,
) {
    let mut cur_sec: libc::c_long = 0;
    let mut cur_ms: libc::c_long = 0;
    let mut when_sec: libc::c_long = 0;
    let mut when_ms: libc::c_long = 0;
    aeGetTime(&mut cur_sec, &mut cur_ms);
    when_sec = (cur_sec as libc::c_longlong + milliseconds / 1000 as libc::c_longlong)
        as libc::c_long;
    when_ms = (cur_ms as libc::c_longlong + milliseconds % 1000 as libc::c_longlong)
        as libc::c_long;
    if when_ms >= 1000 as libc::c_long {
        when_sec += 1;
        when_ms -= 1000 as libc::c_long;
    }
    *sec = when_sec;
    *ms = when_ms;
}
pub unsafe extern "C" fn aeCreateTimeEvent(
    mut eventLoop: *mut aeEventLoop,
    mut milliseconds: libc::c_longlong,
    mut proc_0: Option::<aeTimeProc>,
    mut clientData: *mut libc::c_void,
    mut finalizerProc: Option::<aeEventFinalizerProc>,
) -> libc::c_longlong {
    let mut id: libc::c_longlong = 0;
    let mut tmp: libc::c_longlong = 0;
    let mut te: *mut aeTimeEvent = 0 as *mut aeTimeEvent;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = (*eventLoop).timeEventNextId;
    (*eventLoop).timeEventNextId += 1;
    id = tmp;
    tmp___0 = zmalloc(::std::mem::size_of::<aeTimeEvent>() as libc::c_ulong);
    te = tmp___0 as *mut aeTimeEvent;
    if te as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_longlong);
    }
    (*te).id = id;
    aeAddMillisecondsToNow(milliseconds, &mut (*te).when_sec, &mut (*te).when_ms);
    (*te).timeProc = proc_0;
    (*te).finalizerProc = finalizerProc;
    (*te).clientData = clientData;
    (*te).next = (*eventLoop).timeEventHead;
    (*eventLoop).timeEventHead = te;
    return id;
}
pub unsafe extern "C" fn aeDeleteTimeEvent(
    mut eventLoop: *mut aeEventLoop,
    mut id: libc::c_longlong,
) -> libc::c_int {
    let mut te: *mut aeTimeEvent = 0 as *mut aeTimeEvent;
    te = (*eventLoop).timeEventHead;
    while !te.is_null() {
        if (*te).id == id {
            (*te).id = -(1 as libc::c_longlong);
            return 0 as libc::c_int;
        }
        te = (*te).next;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn aeSearchNearestTimer(
    mut eventLoop: *mut aeEventLoop,
) -> *mut aeTimeEvent {
    let mut te: *mut aeTimeEvent = 0 as *mut aeTimeEvent;
    let mut nearest: *mut aeTimeEvent = 0 as *mut aeTimeEvent;
    te = (*eventLoop).timeEventHead;
    nearest = 0 as *mut libc::c_void as *mut aeTimeEvent;
    while !te.is_null() {
        if nearest.is_null() {
            nearest = te;
        } else if (*te).when_sec < (*nearest).when_sec {
            nearest = te;
        } else if (*te).when_sec == (*nearest).when_sec {
            if (*te).when_ms < (*nearest).when_ms {
                nearest = te;
            }
        }
        te = (*te).next;
    }
    return nearest;
}
unsafe extern "C" fn processTimeEvents(mut eventLoop: *mut aeEventLoop) -> libc::c_int {
    let mut processed: libc::c_int = 0;
    let mut te: *mut aeTimeEvent = 0 as *mut aeTimeEvent;
    let mut prev: *mut aeTimeEvent = 0 as *mut aeTimeEvent;
    let mut maxId: libc::c_longlong = 0;
    let mut now: time_t = 0;
    let mut tmp: time_t = 0;
    let mut now_sec: libc::c_long = 0;
    let mut now_ms: libc::c_long = 0;
    let mut id: libc::c_longlong = 0;
    let mut next: *mut aeTimeEvent = 0 as *mut aeTimeEvent;
    let mut retval: libc::c_int = 0;
    processed = 0 as libc::c_int;
    tmp = time(0 as *mut libc::c_void as *mut time_t);
    now = tmp;
    if now < (*eventLoop).lastTime {
        te = (*eventLoop).timeEventHead;
        while !te.is_null() {
            (*te).when_sec = 0 as libc::c_long;
            te = (*te).next;
        }
    }
    (*eventLoop).lastTime = now;
    prev = 0 as *mut libc::c_void as *mut aeTimeEvent;
    te = (*eventLoop).timeEventHead;
    maxId = (*eventLoop).timeEventNextId - 1 as libc::c_longlong;
    while !te.is_null() {
        if (*te).id == -(1 as libc::c_longlong) {
            next = (*te).next;
            if prev as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                (*eventLoop).timeEventHead = (*te).next;
            } else {
                (*prev).next = (*te).next;
            }
            if ((*te).finalizerProc).is_some() {
                (Some(((*te).finalizerProc).expect("non-null function pointer")))
                    .expect("non-null function pointer")(eventLoop, (*te).clientData);
            }
            zfree(te as *mut libc::c_void);
            te = next;
        } else if (*te).id > maxId {
            te = (*te).next;
        } else {
            aeGetTime(&mut now_sec, &mut now_ms);
            let mut current_block_34: u64;
            if now_sec > (*te).when_sec {
                current_block_34 = 5808507434933310164;
            } else if now_sec == (*te).when_sec {
                if now_ms >= (*te).when_ms {
                    current_block_34 = 5808507434933310164;
                } else {
                    current_block_34 = 15597372965620363352;
                }
            } else {
                current_block_34 = 15597372965620363352;
            }
            match current_block_34 {
                5808507434933310164 => {
                    id = (*te).id;
                    retval = (Some(((*te).timeProc).expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(eventLoop, id, (*te).clientData);
                    processed += 1;
                    if retval != -(1 as libc::c_int) {
                        aeAddMillisecondsToNow(
                            retval as libc::c_longlong,
                            &mut (*te).when_sec,
                            &mut (*te).when_ms,
                        );
                    } else {
                        (*te).id = -(1 as libc::c_longlong);
                    }
                }
                _ => {}
            }
            prev = te;
            te = (*te).next;
        }
    }
    return processed;
}
pub unsafe extern "C" fn aeProcessEvents(
    mut eventLoop: *mut aeEventLoop,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut processed: libc::c_int = 0;
    let mut numevents: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut shortest: *mut aeTimeEvent = 0 as *mut aeTimeEvent;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tvp: *mut timeval = 0 as *mut timeval;
    let mut now_sec: libc::c_long = 0;
    let mut now_ms: libc::c_long = 0;
    let mut ms: libc::c_longlong = 0;
    let mut fe: *mut aeFileEvent = 0 as *mut aeFileEvent;
    let mut mask: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    let mut rfired: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    processed = 0 as libc::c_int;
    if flags & 2 as libc::c_int == 0 {
        if flags & 1 as libc::c_int == 0 {
            return 0 as libc::c_int;
        }
    }
    let mut current_block_52: u64;
    if (*eventLoop).maxfd != -(1 as libc::c_int) {
        current_block_52 = 6508610651671576100;
    } else if flags & 2 as libc::c_int != 0 {
        if flags & 4 as libc::c_int == 0 {
            current_block_52 = 6508610651671576100;
        } else {
            current_block_52 = 1423531122933789233;
        }
    } else {
        current_block_52 = 1423531122933789233;
    }
    match current_block_52 {
        6508610651671576100 => {
            shortest = 0 as *mut libc::c_void as *mut aeTimeEvent;
            if flags & 2 as libc::c_int != 0 {
                if flags & 4 as libc::c_int == 0 {
                    shortest = aeSearchNearestTimer(eventLoop);
                }
            }
            if !shortest.is_null() {
                aeGetTime(&mut now_sec, &mut now_ms);
                tvp = &mut tv;
                ms = (((*shortest).when_sec - now_sec) * 1000 as libc::c_long
                    + (*shortest).when_ms - now_ms) as libc::c_longlong;
                if ms > 0 as libc::c_longlong {
                    (*tvp).tv_sec = (ms / 1000 as libc::c_longlong) as __time_t;
                    (*tvp)
                        .tv_usec = (ms % 1000 as libc::c_longlong
                        * 1000 as libc::c_longlong) as __suseconds_t;
                } else {
                    (*tvp).tv_sec = 0 as libc::c_int as __time_t;
                    (*tvp).tv_usec = 0 as libc::c_int as __suseconds_t;
                }
            } else if flags & 4 as libc::c_int != 0 {
                tv.tv_usec = 0 as libc::c_int as __suseconds_t;
                tv.tv_sec = tv.tv_usec;
                tvp = &mut tv;
            } else {
                tvp = 0 as *mut libc::c_void as *mut timeval;
            }
            numevents = aeApiPoll(eventLoop, tvp);
            j = 0 as libc::c_int;
            while j < numevents {
                fe = ((*eventLoop).events)
                    .offset((*((*eventLoop).fired).offset(j as isize)).fd as isize);
                mask = (*((*eventLoop).fired).offset(j as isize)).mask;
                fd = (*((*eventLoop).fired).offset(j as isize)).fd;
                rfired = 0 as libc::c_int;
                if (*fe).mask & mask & 1 as libc::c_int != 0 {
                    rfired = 1 as libc::c_int;
                    (Some(((*fe).rfileProc).expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(eventLoop, fd, (*fe).clientData, mask);
                }
                if (*fe).mask & mask & 2 as libc::c_int != 0 {
                    if rfired == 0 {
                        (Some(((*fe).wfileProc).expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(eventLoop, fd, (*fe).clientData, mask);
                    } else if ::std::mem::transmute::<
                            Option::<aeFileProc>,
                            libc::c_ulong,
                        >((*fe).wfileProc)
                            != ::std::mem::transmute::<
                                Option::<aeFileProc>,
                                libc::c_ulong,
                            >((*fe).rfileProc)
                        {
                        (Some(((*fe).wfileProc).expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(eventLoop, fd, (*fe).clientData, mask);
                    }
                }
                processed += 1;
                j += 1;
            }
        }
        _ => {}
    }
    if flags & 2 as libc::c_int != 0 {
        tmp = processTimeEvents(eventLoop);
        processed += tmp;
    }
    return processed;
}
pub unsafe extern "C" fn aeWait(
    mut fd: libc::c_int,
    mut mask: libc::c_int,
    mut milliseconds: libc::c_longlong,
) -> libc::c_int {
    let mut pfd: pollfd = pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    };
    let mut retmask: libc::c_int = 0;
    let mut retval: libc::c_int = 0;
    retmask = 0 as libc::c_int;
    memset(
        &mut pfd as *mut pollfd as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<pollfd>() as libc::c_ulong,
    );
    pfd.fd = fd;
    if mask & 1 as libc::c_int != 0 {
        pfd.events = (pfd.events as libc::c_int | 1 as libc::c_int) as libc::c_short;
    }
    if mask & 2 as libc::c_int != 0 {
        pfd.events = (pfd.events as libc::c_int | 4 as libc::c_int) as libc::c_short;
    }
    retval = poll(&mut pfd, 1 as libc::c_int as nfds_t, milliseconds as libc::c_int);
    if retval == 1 as libc::c_int {
        if pfd.revents as libc::c_int & 1 as libc::c_int != 0 {
            retmask |= 1 as libc::c_int;
        }
        if pfd.revents as libc::c_int & 4 as libc::c_int != 0 {
            retmask |= 2 as libc::c_int;
        }
        if pfd.revents as libc::c_int & 8 as libc::c_int != 0 {
            retmask |= 2 as libc::c_int;
        }
        if pfd.revents as libc::c_int & 16 as libc::c_int != 0 {
            retmask |= 2 as libc::c_int;
        }
        return retmask;
    } else {
        return retval
    };
}
pub unsafe extern "C" fn aeMain(mut eventLoop: *mut aeEventLoop) {
    (*eventLoop).stop = 0 as libc::c_int;
    while (*eventLoop).stop == 0 {
        if ::std::mem::transmute::<
            Option::<aeBeforeSleepProc>,
            libc::c_ulong,
        >((*eventLoop).beforesleep) != 0 as *mut libc::c_void as libc::c_ulong
        {
            (Some(((*eventLoop).beforesleep).expect("non-null function pointer")))
                .expect("non-null function pointer")(eventLoop);
        }
        aeProcessEvents(eventLoop, 3 as libc::c_int);
    }
}
pub unsafe extern "C" fn aeGetApiName() -> *mut libc::c_char {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = aeApiName();
    return tmp;
}
pub unsafe extern "C" fn aeSetBeforeSleepProc(
    mut eventLoop: *mut aeEventLoop,
    mut beforesleep: Option::<aeBeforeSleepProc>,
) {
    (*eventLoop).beforesleep = beforesleep;
}
pub unsafe extern "C" fn zlibc_free(mut ptr: *mut libc::c_void) {
    free(ptr);
}
static mut used_memory: size_t = 0 as libc::c_int as size_t;
pub static mut used_memory_mutex: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
unsafe extern "C" fn zmalloc_default_oom(mut size: size_t) {
    fprintf(
        stderr,
        b"zmalloc: Out of memory trying to allocate %zu bytes\n\0" as *const u8
            as *const libc::c_char,
        size,
    );
    fflush(stderr);
    abort();
}
static mut zmalloc_oom_handler: Option::<unsafe extern "C" fn(size_t) -> ()> = Some(
    zmalloc_default_oom as unsafe extern "C" fn(size_t) -> (),
);
pub unsafe extern "C" fn zmalloc(mut size: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _n: size_t = 0;
    tmp = malloc(size.wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong));
    ptr = tmp;
    if ptr.is_null() {
        (Some(zmalloc_oom_handler.expect("non-null function pointer")))
            .expect("non-null function pointer")(size);
    }
    *(ptr as *mut size_t) = size;
    _n = size.wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong);
    if _n
        & (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_ulong) != 0
    {
        _n = (_n as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                    .wrapping_sub(
                        _n
                            & (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_ulong),
                    ),
            ) as size_t as size_t;
    }
    pthread_mutex_lock(&mut used_memory_mutex);
    used_memory = (used_memory as libc::c_ulong)
        .wrapping_add(
            size.wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong),
        ) as size_t as size_t;
    pthread_mutex_unlock(&mut used_memory_mutex);
    return (ptr as *mut libc::c_char)
        .offset(::std::mem::size_of::<size_t>() as libc::c_ulong as isize)
        as *mut libc::c_void;
}
pub unsafe extern "C" fn zcalloc(mut size: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _n: size_t = 0;
    tmp = calloc(
        1 as libc::c_int as size_t,
        size.wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong),
    );
    ptr = tmp;
    if ptr.is_null() {
        (Some(zmalloc_oom_handler.expect("non-null function pointer")))
            .expect("non-null function pointer")(size);
    }
    *(ptr as *mut size_t) = size;
    _n = size.wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong);
    if _n
        & (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_ulong) != 0
    {
        _n = (_n as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                    .wrapping_sub(
                        _n
                            & (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_ulong),
                    ),
            ) as size_t as size_t;
    }
    pthread_mutex_lock(&mut used_memory_mutex);
    used_memory = (used_memory as libc::c_ulong)
        .wrapping_add(
            size.wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong),
        ) as size_t as size_t;
    pthread_mutex_unlock(&mut used_memory_mutex);
    return (ptr as *mut libc::c_char)
        .offset(::std::mem::size_of::<size_t>() as libc::c_ulong as isize)
        as *mut libc::c_void;
}
pub unsafe extern "C" fn zrealloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut realptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut oldsize: size_t = 0;
    let mut newptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _n: size_t = 0;
    let mut _n___0: size_t = 0;
    if ptr as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp = zmalloc(size);
        return tmp;
    }
    realptr = (ptr as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<size_t>() as libc::c_ulong as isize))
        as *mut libc::c_void;
    oldsize = *(realptr as *mut size_t);
    newptr = realloc(
        realptr,
        size.wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong),
    );
    if newptr.is_null() {
        (Some(zmalloc_oom_handler.expect("non-null function pointer")))
            .expect("non-null function pointer")(size);
    }
    *(newptr as *mut size_t) = size;
    _n = oldsize;
    if _n
        & (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_ulong) != 0
    {
        _n = (_n as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                    .wrapping_sub(
                        _n
                            & (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_ulong),
                    ),
            ) as size_t as size_t;
    }
    pthread_mutex_lock(&mut used_memory_mutex);
    used_memory = (used_memory as libc::c_ulong).wrapping_sub(oldsize) as size_t
        as size_t;
    pthread_mutex_unlock(&mut used_memory_mutex);
    _n___0 = size;
    if _n___0
        & (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_ulong) != 0
    {
        _n___0 = (_n___0 as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                    .wrapping_sub(
                        _n___0
                            & (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_ulong),
                    ),
            ) as size_t as size_t;
    }
    pthread_mutex_lock(&mut used_memory_mutex);
    used_memory = (used_memory as libc::c_ulong).wrapping_add(size) as size_t as size_t;
    pthread_mutex_unlock(&mut used_memory_mutex);
    return (newptr as *mut libc::c_char)
        .offset(::std::mem::size_of::<size_t>() as libc::c_ulong as isize)
        as *mut libc::c_void;
}
pub unsafe extern "C" fn zmalloc_size(mut ptr: *mut libc::c_void) -> size_t {
    let mut realptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut size: size_t = 0;
    realptr = (ptr as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<size_t>() as libc::c_ulong as isize))
        as *mut libc::c_void;
    size = *(realptr as *mut size_t);
    if size
        & (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_ulong) != 0
    {
        size = (size as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                    .wrapping_sub(
                        size
                            & (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_ulong),
                    ),
            ) as size_t as size_t;
    }
    return size.wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong);
}
pub unsafe extern "C" fn zfree(mut ptr: *mut libc::c_void) {
    let mut realptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut oldsize: size_t = 0;
    let mut _n: size_t = 0;
    if ptr as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    realptr = (ptr as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<size_t>() as libc::c_ulong as isize))
        as *mut libc::c_void;
    oldsize = *(realptr as *mut size_t);
    _n = oldsize.wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong);
    if _n
        & (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_ulong) != 0
    {
        _n = (_n as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                    .wrapping_sub(
                        _n
                            & (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_ulong),
                    ),
            ) as size_t as size_t;
    }
    pthread_mutex_lock(&mut used_memory_mutex);
    used_memory = (used_memory as libc::c_ulong)
        .wrapping_sub(
            oldsize.wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong),
        ) as size_t as size_t;
    pthread_mutex_unlock(&mut used_memory_mutex);
    free(realptr);
}
pub unsafe extern "C" fn zstrdup(mut s: *const libc::c_char) -> *mut libc::c_char {
    let mut l: size_t = 0;
    let mut tmp: size_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = strlen(s);
    l = tmp.wrapping_add(1 as libc::c_ulong);
    tmp___0 = zmalloc(l);
    p = tmp___0 as *mut libc::c_char;
    memcpy(p as *mut libc::c_void, s as *const libc::c_void, l);
    return p;
}
pub unsafe extern "C" fn zmalloc_used_memory() -> size_t {
    let mut um: size_t = 0;
    pthread_mutex_lock(&mut used_memory_mutex);
    um = used_memory;
    pthread_mutex_unlock(&mut used_memory_mutex);
    return um;
}
pub unsafe extern "C" fn zmalloc_set_oom_handler(
    mut oom_handler: Option::<unsafe extern "C" fn(size_t) -> ()>,
) {
    zmalloc_oom_handler = oom_handler;
}
pub unsafe extern "C" fn zmalloc_get_rss() -> size_t {
    let mut tmp: size_t = 0;
    tmp = zmalloc_used_memory();
    return tmp;
}
pub unsafe extern "C" fn zmalloc_get_fragmentation_ratio(
    mut rss: size_t,
) -> libc::c_float {
    let mut tmp: size_t = 0;
    tmp = zmalloc_used_memory();
    return rss as libc::c_float / tmp as libc::c_float;
}
pub unsafe extern "C" fn zmalloc_get_smap_bytes_by_field(
    mut field: *mut libc::c_char,
    mut pid: libc::c_long,
) -> size_t {
    return 0 as libc::c_int as size_t;
}
pub unsafe extern "C" fn zmalloc_get_private_dirty(mut pid: libc::c_long) -> size_t {
    let mut tmp: size_t = 0;
    tmp = zmalloc_get_smap_bytes_by_field(
        b"Private_Dirty:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pid,
    );
    return tmp;
}
pub unsafe extern "C" fn zmalloc_get_memory_size() -> size_t {
    return 0 as libc::c_long as size_t;
}
static mut method_strings: [*const libc::c_char; 33] = [
    b"DELETE\0" as *const u8 as *const libc::c_char,
    b"GET\0" as *const u8 as *const libc::c_char,
    b"HEAD\0" as *const u8 as *const libc::c_char,
    b"POST\0" as *const u8 as *const libc::c_char,
    b"PUT\0" as *const u8 as *const libc::c_char,
    b"CONNECT\0" as *const u8 as *const libc::c_char,
    b"OPTIONS\0" as *const u8 as *const libc::c_char,
    b"TRACE\0" as *const u8 as *const libc::c_char,
    b"COPY\0" as *const u8 as *const libc::c_char,
    b"LOCK\0" as *const u8 as *const libc::c_char,
    b"MKCOL\0" as *const u8 as *const libc::c_char,
    b"MOVE\0" as *const u8 as *const libc::c_char,
    b"PROPFIND\0" as *const u8 as *const libc::c_char,
    b"PROPPATCH\0" as *const u8 as *const libc::c_char,
    b"SEARCH\0" as *const u8 as *const libc::c_char,
    b"UNLOCK\0" as *const u8 as *const libc::c_char,
    b"BIND\0" as *const u8 as *const libc::c_char,
    b"REBIND\0" as *const u8 as *const libc::c_char,
    b"UNBIND\0" as *const u8 as *const libc::c_char,
    b"ACL\0" as *const u8 as *const libc::c_char,
    b"REPORT\0" as *const u8 as *const libc::c_char,
    b"MKACTIVITY\0" as *const u8 as *const libc::c_char,
    b"CHECKOUT\0" as *const u8 as *const libc::c_char,
    b"MERGE\0" as *const u8 as *const libc::c_char,
    b"M-SEARCH\0" as *const u8 as *const libc::c_char,
    b"NOTIFY\0" as *const u8 as *const libc::c_char,
    b"SUBSCRIBE\0" as *const u8 as *const libc::c_char,
    b"UNSUBSCRIBE\0" as *const u8 as *const libc::c_char,
    b"PATCH\0" as *const u8 as *const libc::c_char,
    b"PURGE\0" as *const u8 as *const libc::c_char,
    b"MKCALENDAR\0" as *const u8 as *const libc::c_char,
    b"LINK\0" as *const u8 as *const libc::c_char,
    b"UNLINK\0" as *const u8 as *const libc::c_char,
];
static mut tokens: [libc::c_char; 256] = [
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
    0 as libc::c_int as libc::c_char,
    '!' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    '$' as i32 as libc::c_char,
    '%' as i32 as libc::c_char,
    '&' as i32 as libc::c_char,
    '\'' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '*' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '-' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
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
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
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
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '^' as i32 as libc::c_char,
    '_' as i32 as libc::c_char,
    '`' as i32 as libc::c_char,
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
    0 as libc::c_int as libc::c_char,
    '|' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '~' as i32 as libc::c_char,
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
static mut unhex: [int8_t; 256] = [
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    0 as libc::c_int as int8_t,
    1 as libc::c_int as int8_t,
    2 as libc::c_int as int8_t,
    3 as libc::c_int as int8_t,
    4 as libc::c_int as int8_t,
    5 as libc::c_int as int8_t,
    6 as libc::c_int as int8_t,
    7 as libc::c_int as int8_t,
    8 as libc::c_int as int8_t,
    9 as libc::c_int as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    10 as libc::c_int as int8_t,
    11 as libc::c_int as int8_t,
    12 as libc::c_int as int8_t,
    13 as libc::c_int as int8_t,
    14 as libc::c_int as int8_t,
    15 as libc::c_int as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    10 as libc::c_int as int8_t,
    11 as libc::c_int as int8_t,
    12 as libc::c_int as int8_t,
    13 as libc::c_int as int8_t,
    14 as libc::c_int as int8_t,
    15 as libc::c_int as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
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
static mut normal_url_char: [uint8_t; 32] = [
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    246 as libc::c_int as uint8_t,
    255 as libc::c_int as uint8_t,
    255 as libc::c_int as uint8_t,
    127 as libc::c_int as uint8_t,
    255 as libc::c_int as uint8_t,
    255 as libc::c_int as uint8_t,
    255 as libc::c_int as uint8_t,
    255 as libc::c_int as uint8_t,
    255 as libc::c_int as uint8_t,
    255 as libc::c_int as uint8_t,
    255 as libc::c_int as uint8_t,
    127 as libc::c_int as uint8_t,
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
static mut http_strerror_tab: [__anonstruct_http_strerror_tab_527861670; 33] = [
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_OK\0" as *const u8 as *const libc::c_char,
            description: b"success\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_CB_message_begin\0" as *const u8 as *const libc::c_char,
            description: b"the on_message_begin callback failed\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_CB_url\0" as *const u8 as *const libc::c_char,
            description: b"the on_url callback failed\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_CB_header_field\0" as *const u8 as *const libc::c_char,
            description: b"the on_header_field callback failed\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_CB_header_value\0" as *const u8 as *const libc::c_char,
            description: b"the on_header_value callback failed\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_CB_headers_complete\0" as *const u8 as *const libc::c_char,
            description: b"the on_headers_complete callback failed\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_CB_body\0" as *const u8 as *const libc::c_char,
            description: b"the on_body callback failed\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_CB_message_complete\0" as *const u8 as *const libc::c_char,
            description: b"the on_message_complete callback failed\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_CB_status\0" as *const u8 as *const libc::c_char,
            description: b"the on_status callback failed\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_CB_chunk_header\0" as *const u8 as *const libc::c_char,
            description: b"the on_chunk_header callback failed\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_CB_chunk_complete\0" as *const u8 as *const libc::c_char,
            description: b"the on_chunk_complete callback failed\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_INVALID_EOF_STATE\0" as *const u8 as *const libc::c_char,
            description: b"stream ended at an unexpected time\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_HEADER_OVERFLOW\0" as *const u8 as *const libc::c_char,
            description: b"too many header bytes seen; overflow detected\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_CLOSED_CONNECTION\0" as *const u8 as *const libc::c_char,
            description: b"data received after completed connection: close message\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_INVALID_VERSION\0" as *const u8 as *const libc::c_char,
            description: b"invalid HTTP version\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_INVALID_STATUS\0" as *const u8 as *const libc::c_char,
            description: b"invalid HTTP status code\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_INVALID_METHOD\0" as *const u8 as *const libc::c_char,
            description: b"invalid HTTP method\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_INVALID_URL\0" as *const u8 as *const libc::c_char,
            description: b"invalid URL\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_INVALID_HOST\0" as *const u8 as *const libc::c_char,
            description: b"invalid host\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_INVALID_PORT\0" as *const u8 as *const libc::c_char,
            description: b"invalid port\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_INVALID_PATH\0" as *const u8 as *const libc::c_char,
            description: b"invalid path\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_INVALID_QUERY_STRING\0" as *const u8 as *const libc::c_char,
            description: b"invalid query string\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_INVALID_FRAGMENT\0" as *const u8 as *const libc::c_char,
            description: b"invalid fragment\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_LF_EXPECTED\0" as *const u8 as *const libc::c_char,
            description: b"LF character expected\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_INVALID_HEADER_TOKEN\0" as *const u8 as *const libc::c_char,
            description: b"invalid character in header\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_INVALID_CONTENT_LENGTH\0" as *const u8 as *const libc::c_char,
            description: b"invalid character in content-length header\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_UNEXPECTED_CONTENT_LENGTH\0" as *const u8 as *const libc::c_char,
            description: b"unexpected content-length header\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_INVALID_CHUNK_SIZE\0" as *const u8 as *const libc::c_char,
            description: b"invalid character in chunk size header\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_INVALID_CONSTANT\0" as *const u8 as *const libc::c_char,
            description: b"invalid constant string\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_INVALID_INTERNAL_STATE\0" as *const u8 as *const libc::c_char,
            description: b"encountered unexpected internal state\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_STRICT\0" as *const u8 as *const libc::c_char,
            description: b"strict mode assertion failed\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_PAUSED\0" as *const u8 as *const libc::c_char,
            description: b"parser is paused\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = __anonstruct_http_strerror_tab_527861670 {
            name: b"HPE_UNKNOWN\0" as *const u8 as *const libc::c_char,
            description: b"an unknown error occurred\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
];
unsafe extern "C" fn parse_url_char(mut s: state, ch: libc::c_char) -> state {
    if ch as libc::c_int == 32 as libc::c_int {
        return s_dead
    } else {
        if ch as libc::c_int == 13 as libc::c_int {
            return s_dead
        } else {
            if ch as libc::c_int == 10 as libc::c_int {
                return s_dead;
            }
        }
    }
    if ch as libc::c_int == 9 as libc::c_int {
        return s_dead
    } else {
        if ch as libc::c_int == 12 as libc::c_int {
            return s_dead;
        }
    }
    let mut current_block_137: u64;
    match s as libc::c_uint {
        20 => {
            if ch as libc::c_int == 47 as libc::c_int {
                return s_req_path
            } else {
                if ch as libc::c_int == 42 as libc::c_int {
                    return s_req_path;
                }
            }
            if (ch as libc::c_int | 32 as libc::c_int) as libc::c_uchar as libc::c_int
                >= 97 as libc::c_int
            {
                if (ch as libc::c_int | 32 as libc::c_int) as libc::c_uchar
                    as libc::c_int <= 122 as libc::c_int
                {
                    return s_req_schema;
                }
            }
            current_block_137 = 18002345992382212654;
        }
        21 => {
            if (ch as libc::c_int | 32 as libc::c_int) as libc::c_uchar as libc::c_int
                >= 97 as libc::c_int
            {
                if (ch as libc::c_int | 32 as libc::c_int) as libc::c_uchar
                    as libc::c_int <= 122 as libc::c_int
                {
                    return s;
                }
            }
            if ch as libc::c_int == 58 as libc::c_int {
                return s_req_schema_slash;
            }
            current_block_137 = 18002345992382212654;
        }
        22 => {
            if ch as libc::c_int == 47 as libc::c_int {
                return s_req_schema_slash_slash;
            }
            current_block_137 = 18002345992382212654;
        }
        23 => {
            if ch as libc::c_int == 47 as libc::c_int {
                return s_req_server_start;
            }
            current_block_137 = 18002345992382212654;
        }
        26 => {
            if ch as libc::c_int == 64 as libc::c_int {
                return s_dead;
            }
            current_block_137 = 8727624715163885381;
        }
        25 | 24 => {
            current_block_137 = 8727624715163885381;
        }
        27 => {
            if normal_url_char[(ch as libc::c_uchar as libc::c_uint >> 3 as libc::c_int)
                as usize] as libc::c_uint
                & ((1 as libc::c_int)
                    << (ch as libc::c_uchar as libc::c_uint & 7 as libc::c_uint))
                    as libc::c_uint != 0
            {
                return s;
            }
            match ch as libc::c_int {
                63 => return s_req_query_string_start,
                35 => return s_req_fragment_start,
                _ => {}
            }
            current_block_137 = 18002345992382212654;
        }
        29 | 28 => {
            if normal_url_char[(ch as libc::c_uchar as libc::c_uint >> 3 as libc::c_int)
                as usize] as libc::c_uint
                & ((1 as libc::c_int)
                    << (ch as libc::c_uchar as libc::c_uint & 7 as libc::c_uint))
                    as libc::c_uint != 0
            {
                return s_req_query_string;
            }
            match ch as libc::c_int {
                63 => return s_req_query_string,
                35 => return s_req_fragment_start,
                _ => {}
            }
            current_block_137 = 18002345992382212654;
        }
        30 => {
            if normal_url_char[(ch as libc::c_uchar as libc::c_uint >> 3 as libc::c_int)
                as usize] as libc::c_uint
                & ((1 as libc::c_int)
                    << (ch as libc::c_uchar as libc::c_uint & 7 as libc::c_uint))
                    as libc::c_uint != 0
            {
                return s_req_fragment;
            }
            match ch as libc::c_int {
                63 => return s_req_fragment,
                35 => return s,
                _ => {}
            }
            current_block_137 = 18002345992382212654;
        }
        31 => {
            if normal_url_char[(ch as libc::c_uchar as libc::c_uint >> 3 as libc::c_int)
                as usize] as libc::c_uint
                & ((1 as libc::c_int)
                    << (ch as libc::c_uchar as libc::c_uint & 7 as libc::c_uint))
                    as libc::c_uint != 0
            {
                return s;
            }
            match ch as libc::c_int {
                35 | 63 => return s,
                _ => {}
            }
            current_block_137 = 18002345992382212654;
        }
        _ => {
            current_block_137 = 18002345992382212654;
        }
    }
    match current_block_137 {
        8727624715163885381 => {
            if ch as libc::c_int == 47 as libc::c_int {
                return s_req_path;
            }
            if ch as libc::c_int == 63 as libc::c_int {
                return s_req_query_string_start;
            }
            if ch as libc::c_int == 64 as libc::c_int {
                return s_req_server_with_at;
            }
            if (ch as libc::c_int | 32 as libc::c_int) as libc::c_uchar as libc::c_int
                >= 97 as libc::c_int
            {
                if (ch as libc::c_int | 32 as libc::c_int) as libc::c_uchar
                    as libc::c_int <= 122 as libc::c_int
                {
                    return s_req_server;
                }
            }
            if ch as libc::c_int >= 48 as libc::c_int {
                if ch as libc::c_int <= 57 as libc::c_int {
                    return s_req_server;
                }
            }
            if ch as libc::c_int == 45 as libc::c_int {
                return s_req_server
            } else {
                if ch as libc::c_int == 95 as libc::c_int {
                    return s_req_server
                } else {
                    if ch as libc::c_int == 46 as libc::c_int {
                        return s_req_server
                    } else {
                        if ch as libc::c_int == 33 as libc::c_int {
                            return s_req_server
                        } else {
                            if ch as libc::c_int == 126 as libc::c_int {
                                return s_req_server
                            } else {
                                if ch as libc::c_int == 42 as libc::c_int {
                                    return s_req_server
                                } else {
                                    if ch as libc::c_int == 39 as libc::c_int {
                                        return s_req_server
                                    } else {
                                        if ch as libc::c_int == 40 as libc::c_int {
                                            return s_req_server
                                        } else {
                                            if ch as libc::c_int == 41 as libc::c_int {
                                                return s_req_server
                                            } else {
                                                if ch as libc::c_int == 37 as libc::c_int {
                                                    return s_req_server
                                                } else {
                                                    if ch as libc::c_int == 59 as libc::c_int {
                                                        return s_req_server
                                                    } else {
                                                        if ch as libc::c_int == 58 as libc::c_int {
                                                            return s_req_server
                                                        } else {
                                                            if ch as libc::c_int == 38 as libc::c_int {
                                                                return s_req_server
                                                            } else {
                                                                if ch as libc::c_int == 61 as libc::c_int {
                                                                    return s_req_server
                                                                } else {
                                                                    if ch as libc::c_int == 43 as libc::c_int {
                                                                        return s_req_server
                                                                    } else {
                                                                        if ch as libc::c_int == 36 as libc::c_int {
                                                                            return s_req_server
                                                                        } else {
                                                                            if ch as libc::c_int == 44 as libc::c_int {
                                                                                return s_req_server
                                                                            } else {
                                                                                if ch as libc::c_int == 91 as libc::c_int {
                                                                                    return s_req_server
                                                                                } else {
                                                                                    if ch as libc::c_int == 93 as libc::c_int {
                                                                                        return s_req_server;
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
                        }
                    }
                }
            }
        }
        _ => {}
    }
    return s_dead;
}
pub unsafe extern "C" fn http_parser_execute(
    mut parser: *mut http_parser,
    mut settings: *const http_parser_settings,
    mut data: *const libc::c_char,
    mut len: size_t,
) -> size_t {
    let mut current_block: u64;
    let mut c: libc::c_char = 0;
    let mut ch: libc::c_char = 0;
    let mut unhex_val: int8_t = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut header_field_mark: *const libc::c_char = 0 as *const libc::c_char;
    let mut header_value_mark: *const libc::c_char = 0 as *const libc::c_char;
    let mut url_mark: *const libc::c_char = 0 as *const libc::c_char;
    let mut body_mark: *const libc::c_char = 0 as *const libc::c_char;
    let mut status_mark: *const libc::c_char = 0 as *const libc::c_char;
    let mut p_state: state = 0 as state;
    let mut lenient: libc::c_uint = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_long = 0;
    let mut tmp___2: libc::c_long = 0;
    let mut tmp___3: libc::c_long = 0;
    let mut tmp___4: libc::c_long = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_long = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_long = 0;
    let mut tmp___10: libc::c_long = 0;
    let mut tmp___11: libc::c_long = 0;
    let mut tmp___12: libc::c_long = 0;
    let mut tmp___13: libc::c_int = 0;
    let mut tmp___14: libc::c_int = 0;
    let mut tmp___15: libc::c_long = 0;
    let mut tmp___16: libc::c_long = 0;
    let mut tmp___17: libc::c_long = 0;
    let mut tmp___18: libc::c_int = 0;
    let mut tmp___19: libc::c_long = 0;
    let mut tmp___20: libc::c_long = 0;
    let mut tmp___21: libc::c_int = 0;
    let mut tmp___22: libc::c_long = 0;
    let mut tmp___23: libc::c_long = 0;
    let mut tmp___24: libc::c_long = 0;
    let mut tmp___25: libc::c_int = 0;
    let mut tmp___26: libc::c_int = 0;
    let mut tmp___27: libc::c_long = 0;
    let mut tmp___28: libc::c_long = 0;
    let mut tmp___29: libc::c_long = 0;
    let mut tmp___30: libc::c_int = 0;
    let mut tmp___31: libc::c_int = 0;
    let mut tmp___32: libc::c_long = 0;
    let mut tmp___33: libc::c_long = 0;
    let mut tmp___34: libc::c_long = 0;
    let mut tmp___35: libc::c_int = 0;
    let mut tmp___36: libc::c_long = 0;
    let mut tmp___37: libc::c_int = 0;
    let mut tmp___38: libc::c_int = 0;
    let mut tmp___39: libc::c_long = 0;
    let mut tmp___40: libc::c_long = 0;
    let mut tmp___41: libc::c_long = 0;
    let mut matcher: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___42: libc::c_long = 0;
    let mut tmp___43: state = 0 as state;
    let mut tmp___44: libc::c_long = 0;
    let mut tmp___45: state = 0 as state;
    let mut tmp___46: libc::c_long = 0;
    let mut tmp___47: libc::c_int = 0;
    let mut tmp___48: libc::c_int = 0;
    let mut tmp___49: libc::c_long = 0;
    let mut tmp___50: libc::c_long = 0;
    let mut tmp___51: libc::c_long = 0;
    let mut tmp___52: libc::c_int = 0;
    let mut tmp___53: libc::c_int = 0;
    let mut tmp___54: libc::c_int = 0;
    let mut tmp___55: libc::c_long = 0;
    let mut tmp___56: libc::c_long = 0;
    let mut tmp___57: libc::c_long = 0;
    let mut tmp___58: state = 0 as state;
    let mut tmp___59: libc::c_long = 0;
    let mut tmp___60: libc::c_int = 0;
    let mut tmp___61: libc::c_long = 0;
    let mut tmp___62: libc::c_long = 0;
    let mut tmp___63: libc::c_int = 0;
    let mut tmp___64: libc::c_long = 0;
    let mut tmp___65: libc::c_long = 0;
    let mut tmp___66: libc::c_long = 0;
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___67: libc::c_long = 0;
    let mut tmp___68: libc::c_int = 0;
    let mut tmp___69: libc::c_int = 0;
    let mut tmp___70: libc::c_long = 0;
    let mut tmp___71: libc::c_long = 0;
    let mut tmp___72: libc::c_long = 0;
    let mut tmp___73: libc::c_int = 0;
    let mut tmp___74: libc::c_long = 0;
    let mut start___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut h_state: header_states = h_general;
    let mut tmp___75: libc::c_int = 0;
    let mut tmp___76: libc::c_int = 0;
    let mut tmp___77: libc::c_long = 0;
    let mut tmp___78: libc::c_long = 0;
    let mut tmp___79: libc::c_long = 0;
    let mut tmp___80: libc::c_long = 0;
    let mut tmp___81: libc::c_int = 0;
    let mut tmp___82: libc::c_int = 0;
    let mut tmp___83: libc::c_long = 0;
    let mut tmp___84: libc::c_long = 0;
    let mut tmp___85: libc::c_long = 0;
    let mut p_cr: *const libc::c_char = 0 as *const libc::c_char;
    let mut p_lf: *const libc::c_char = 0 as *const libc::c_char;
    let mut limit: size_t = 0;
    let mut tmp___86: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___87: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___88: libc::c_long = 0;
    let mut t: uint64_t = 0;
    let mut tmp___89: libc::c_int = 0;
    let mut tmp___90: libc::c_long = 0;
    let mut tmp___91: libc::c_long = 0;
    let mut tmp___92: libc::c_long = 0;
    let mut tmp___93: libc::c_long = 0;
    let mut tmp___94: libc::c_int = 0;
    let mut tmp___95: libc::c_int = 0;
    let mut tmp___96: libc::c_long = 0;
    let mut tmp___97: libc::c_long = 0;
    let mut tmp___98: libc::c_long = 0;
    let mut tmp___99: libc::c_int = 0;
    let mut tmp___100: libc::c_int = 0;
    let mut tmp___101: libc::c_long = 0;
    let mut tmp___102: libc::c_long = 0;
    let mut tmp___103: libc::c_long = 0;
    let mut tmp___104: libc::c_int = 0;
    let mut tmp___105: libc::c_int = 0;
    let mut hasBody: libc::c_int = 0;
    let mut tmp___106: libc::c_int = 0;
    let mut tmp___108: libc::c_int = 0;
    let mut tmp___109: libc::c_int = 0;
    let mut tmp___110: libc::c_int = 0;
    let mut tmp___111: libc::c_int = 0;
    let mut tmp___112: libc::c_int = 0;
    let mut tmp___113: libc::c_long = 0;
    let mut tmp___114: libc::c_long = 0;
    let mut tmp___115: libc::c_long = 0;
    let mut tmp___117: libc::c_int = 0;
    let mut tmp___118: libc::c_int = 0;
    let mut tmp___119: libc::c_int = 0;
    let mut tmp___120: libc::c_int = 0;
    let mut tmp___121: libc::c_int = 0;
    let mut tmp___122: libc::c_long = 0;
    let mut tmp___123: libc::c_long = 0;
    let mut tmp___124: libc::c_long = 0;
    let mut tmp___126: libc::c_int = 0;
    let mut tmp___127: libc::c_int = 0;
    let mut tmp___128: libc::c_int = 0;
    let mut tmp___129: libc::c_int = 0;
    let mut tmp___130: libc::c_int = 0;
    let mut tmp___131: libc::c_long = 0;
    let mut tmp___132: libc::c_long = 0;
    let mut tmp___133: libc::c_long = 0;
    let mut tmp___135: libc::c_int = 0;
    let mut tmp___136: libc::c_int = 0;
    let mut tmp___137: libc::c_int = 0;
    let mut tmp___138: libc::c_int = 0;
    let mut tmp___139: libc::c_int = 0;
    let mut tmp___140: libc::c_long = 0;
    let mut tmp___141: libc::c_long = 0;
    let mut tmp___142: libc::c_long = 0;
    let mut tmp___143: libc::c_int = 0;
    let mut to_read: uint64_t = 0;
    let mut tmp___144: uint64_t = 0;
    let mut tmp___145: libc::c_int = 0;
    let mut tmp___146: libc::c_int = 0;
    let mut tmp___147: libc::c_long = 0;
    let mut tmp___148: libc::c_long = 0;
    let mut tmp___149: libc::c_long = 0;
    let mut tmp___151: libc::c_int = 0;
    let mut tmp___152: libc::c_int = 0;
    let mut tmp___153: libc::c_int = 0;
    let mut tmp___154: libc::c_int = 0;
    let mut tmp___155: libc::c_int = 0;
    let mut tmp___156: libc::c_long = 0;
    let mut tmp___157: libc::c_long = 0;
    let mut tmp___158: libc::c_long = 0;
    let mut tmp___159: libc::c_long = 0;
    let mut t___0: uint64_t = 0;
    let mut tmp___160: libc::c_long = 0;
    let mut tmp___161: libc::c_int = 0;
    let mut tmp___162: libc::c_int = 0;
    let mut tmp___163: libc::c_long = 0;
    let mut tmp___164: libc::c_long = 0;
    let mut tmp___165: libc::c_long = 0;
    let mut to_read___0: uint64_t = 0;
    let mut tmp___166: uint64_t = 0;
    let mut tmp___167: libc::c_int = 0;
    let mut tmp___168: libc::c_int = 0;
    let mut tmp___169: libc::c_long = 0;
    let mut tmp___170: libc::c_long = 0;
    let mut tmp___171: libc::c_long = 0;
    let mut tmp___172: libc::c_int = 0;
    let mut tmp___173: libc::c_int = 0;
    let mut tmp___174: libc::c_long = 0;
    let mut tmp___175: libc::c_long = 0;
    let mut tmp___176: libc::c_long = 0;
    let mut tmp___177: libc::c_int = 0;
    let mut tmp___178: libc::c_int = 0;
    let mut tmp___179: libc::c_int = 0;
    let mut tmp___180: libc::c_int = 0;
    let mut tmp___181: libc::c_int = 0;
    let mut tmp___182: libc::c_int = 0;
    let mut tmp___183: libc::c_int = 0;
    let mut tmp___184: libc::c_long = 0;
    let mut tmp___185: libc::c_long = 0;
    let mut tmp___186: libc::c_long = 0;
    let mut tmp___187: libc::c_int = 0;
    let mut tmp___188: libc::c_int = 0;
    let mut tmp___189: libc::c_long = 0;
    let mut tmp___190: libc::c_long = 0;
    let mut tmp___191: libc::c_long = 0;
    let mut tmp___192: libc::c_int = 0;
    let mut tmp___193: libc::c_int = 0;
    let mut tmp___194: libc::c_long = 0;
    let mut tmp___195: libc::c_long = 0;
    let mut tmp___196: libc::c_long = 0;
    let mut tmp___197: libc::c_int = 0;
    let mut tmp___198: libc::c_int = 0;
    let mut tmp___199: libc::c_long = 0;
    let mut tmp___200: libc::c_long = 0;
    let mut tmp___201: libc::c_long = 0;
    let mut tmp___202: libc::c_int = 0;
    let mut tmp___203: libc::c_int = 0;
    let mut tmp___204: libc::c_long = 0;
    let mut tmp___205: libc::c_long = 0;
    let mut tmp___206: libc::c_long = 0;
    p = data;
    header_field_mark = 0 as *const libc::c_char;
    header_value_mark = 0 as *const libc::c_char;
    url_mark = 0 as *const libc::c_char;
    body_mark = 0 as *const libc::c_char;
    status_mark = 0 as *const libc::c_char;
    p_state = (*parser).state() as state;
    lenient = (*parser).lenient_http_headers();
    if (*parser).http_errno() as http_errno as libc::c_uint != 0 as libc::c_uint {
        return 0 as libc::c_int as size_t;
    }
    if len == 0 as libc::c_ulong {
        match p_state as libc::c_uint {
            61 => {
                if !((*parser).http_errno() as http_errno as libc::c_uint
                    == 0 as libc::c_uint)
                {
                    __assert_fail(
                        b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                            as *const libc::c_char,
                        b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                        658 as libc::c_uint,
                        b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                    );
                }
                tmp___3 = ((*settings).on_message_complete).is_some() as libc::c_int
                    as libc::c_long;
                if tmp___3 != 0 {
                    (*parser).set_state(p_state as libc::c_uint);
                    tmp = (Some(
                        ((*settings).on_message_complete)
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")(parser);
                    if 0 as libc::c_int != tmp {
                        tmp___0 = 1 as libc::c_int;
                    } else {
                        tmp___0 = 0 as libc::c_int;
                    }
                    tmp___1 = tmp___0 as libc::c_long;
                    if tmp___1 != 0 {
                        (*parser).set_http_errno(7 as libc::c_uint);
                    }
                    p_state = (*parser).state() as state;
                    tmp___2 = ((*parser).http_errno() as http_errno as libc::c_uint
                        != 0 as libc::c_uint) as libc::c_int as libc::c_long;
                    if tmp___2 != 0 {
                        return p.offset_from(data) as libc::c_long as size_t;
                    }
                }
                return 0 as libc::c_int as size_t;
            }
            18 | 4 | 2 | 1 => return 0 as libc::c_int as size_t,
            _ => {
                (*parser).set_http_errno(11 as libc::c_uint);
                return 1 as libc::c_int as size_t;
            }
        }
    }
    if p_state as libc::c_uint == 43 as libc::c_uint {
        header_field_mark = data;
    }
    if p_state as libc::c_uint == 48 as libc::c_uint {
        header_value_mark = data;
    }
    match p_state as libc::c_uint {
        31 | 30 | 29 | 28 | 26 | 25 | 24 | 23 | 22 | 21 | 27 => {
            url_mark = data;
        }
        16 => {
            status_mark = data;
        }
        _ => {}
    }
    p = data;
    's_697: loop {
        if !(p as libc::c_ulong != data.offset(len as isize) as libc::c_ulong) {
            current_block = 4512486547110630526;
            break;
        }
        ch = *p;
        if p_state as libc::c_uint <= 56 as libc::c_uint {
            (*parser).nread = ((*parser).nread).wrapping_add(1);
            tmp___4 = ((*parser).nread > 81920 as libc::c_uint) as libc::c_int
                as libc::c_long;
            if tmp___4 != 0 {
                (*parser).set_http_errno(12 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
        }
        '_reexecute: loop {
            match p_state as libc::c_uint {
                1 => {
                    if ch as libc::c_int == 13 as libc::c_int {
                        tmp___5 = 1 as libc::c_int;
                    } else if ch as libc::c_int == 10 as libc::c_int {
                        tmp___5 = 1 as libc::c_int;
                    } else {
                        tmp___5 = 0 as libc::c_int;
                    }
                    tmp___6 = tmp___5 as libc::c_long;
                    if tmp___6 != 0 {
                        current_block = 2823593449448810005;
                        break;
                    } else {
                        current_block = 6186816898867308296;
                        break;
                    }
                }
                2 => {
                    if ch as libc::c_int == 13 as libc::c_int {
                        current_block = 2823593449448810005;
                        break;
                    }
                    if ch as libc::c_int == 10 as libc::c_int {
                        current_block = 2823593449448810005;
                        break;
                    }
                    (*parser).set_flags(0 as libc::c_uint);
                    (*parser)
                        .content_length = 18446744073709551615 as libc::c_ulonglong
                        as uint64_t;
                    if ch as libc::c_int == 72 as libc::c_int {
                        p_state = s_res_or_resp_H;
                        if !((*parser).http_errno() as http_errno as libc::c_uint
                            == 0 as libc::c_uint)
                        {
                            __assert_fail(
                                b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                    as *const libc::c_char,
                                b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                                728 as libc::c_uint,
                                b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        tmp___11 = ((*settings).on_message_begin).is_some()
                            as libc::c_int as libc::c_long;
                        if tmp___11 != 0 {
                            (*parser).set_state(p_state as libc::c_uint);
                            tmp___7 = (Some(
                                ((*settings).on_message_begin)
                                    .expect("non-null function pointer"),
                            ))
                                .expect("non-null function pointer")(parser);
                            if 0 as libc::c_int != tmp___7 {
                                tmp___8 = 1 as libc::c_int;
                            } else {
                                tmp___8 = 0 as libc::c_int;
                            }
                            tmp___9 = tmp___8 as libc::c_long;
                            if tmp___9 != 0 {
                                (*parser).set_http_errno(1 as libc::c_uint);
                            }
                            p_state = (*parser).state() as state;
                            tmp___10 = ((*parser).http_errno() as http_errno
                                as libc::c_uint != 0 as libc::c_uint) as libc::c_int
                                as libc::c_long;
                            if tmp___10 != 0 {
                                return (p.offset_from(data) as libc::c_long
                                    + 1 as libc::c_long) as size_t;
                            }
                        }
                        current_block = 2823593449448810005;
                        break;
                    } else {
                        (*parser).set_type_0(0 as libc::c_uint);
                        p_state = s_start_req;
                    }
                }
                3 => {
                    if ch as libc::c_int == 84 as libc::c_int {
                        current_block = 7256998052328658819;
                        break;
                    } else {
                        current_block = 1131197912709891142;
                        break;
                    }
                }
                4 => {
                    (*parser).set_flags(0 as libc::c_uint);
                    (*parser)
                        .content_length = 18446744073709551615 as libc::c_ulonglong
                        as uint64_t;
                    match ch as libc::c_int {
                        72 => {
                            current_block = 1626147699861920882;
                            break;
                        }
                        10 | 13 => {
                            current_block = 1948584361000433526;
                            break;
                        }
                        _ => {
                            current_block = 671854326874165838;
                            break;
                        }
                    }
                }
                5 => {
                    if ch as libc::c_int != 84 as libc::c_int {
                        current_block = 5583171084533732733;
                        break;
                    } else {
                        current_block = 14723351692138084861;
                        break;
                    }
                }
                6 => {
                    if ch as libc::c_int != 84 as libc::c_int {
                        current_block = 7409542616634851420;
                        break;
                    } else {
                        current_block = 15275873808073048539;
                        break;
                    }
                }
                7 => {
                    if ch as libc::c_int != 80 as libc::c_int {
                        current_block = 12843036121559584357;
                        break;
                    } else {
                        current_block = 10029375464402185584;
                        break;
                    }
                }
                8 => {
                    if ch as libc::c_int != 47 as libc::c_int {
                        current_block = 15455430299222214173;
                        break;
                    } else {
                        current_block = 6936584767197543976;
                        break;
                    }
                }
                9 => {
                    if ch as libc::c_int >= 48 as libc::c_int {
                        if ch as libc::c_int <= 57 as libc::c_int {
                            tmp___18 = 0 as libc::c_int;
                        } else {
                            tmp___18 = 1 as libc::c_int;
                        }
                    } else {
                        tmp___18 = 1 as libc::c_int;
                    }
                    tmp___19 = tmp___18 as libc::c_long;
                    if tmp___19 != 0 {
                        current_block = 4954337906651494757;
                        break;
                    } else {
                        current_block = 6531417090144833949;
                        break;
                    }
                }
                10 => {
                    tmp___20 = (ch as libc::c_int != 46 as libc::c_int) as libc::c_int
                        as libc::c_long;
                    if tmp___20 != 0 {
                        current_block = 16749893710890171700;
                        break;
                    } else {
                        current_block = 9602112577253180622;
                        break;
                    }
                }
                11 => {
                    if ch as libc::c_int >= 48 as libc::c_int {
                        if ch as libc::c_int <= 57 as libc::c_int {
                            tmp___21 = 0 as libc::c_int;
                        } else {
                            tmp___21 = 1 as libc::c_int;
                        }
                    } else {
                        tmp___21 = 1 as libc::c_int;
                    }
                    tmp___22 = tmp___21 as libc::c_long;
                    if tmp___22 != 0 {
                        current_block = 15306500312484564201;
                        break;
                    } else {
                        current_block = 14123347050775008060;
                        break;
                    }
                }
                12 => {
                    tmp___23 = (ch as libc::c_int != 32 as libc::c_int) as libc::c_int
                        as libc::c_long;
                    if tmp___23 != 0 {
                        current_block = 17975683517549114783;
                        break;
                    } else {
                        current_block = 6837155546206671799;
                        break;
                    }
                }
                13 => {
                    if ch as libc::c_int >= 48 as libc::c_int {
                        current_block = 15775903806355281185;
                        break;
                    } else {
                        current_block = 1840006672846692558;
                        break;
                    }
                }
                14 => {
                    if ch as libc::c_int >= 48 as libc::c_int {
                        if ch as libc::c_int <= 57 as libc::c_int {
                            (*parser)
                                .set_status_code(
                                    (*parser).status_code() * 10 as libc::c_uint,
                                );
                            (*parser)
                                .set_status_code(
                                    (*parser).status_code()
                                        + (ch as libc::c_int - 48 as libc::c_int) as libc::c_uint,
                                );
                            tmp___24 = ((*parser).status_code() > 999 as libc::c_uint)
                                as libc::c_int as libc::c_long;
                            if tmp___24 != 0 {
                                current_block = 13878384739916089420;
                                break;
                            } else {
                                current_block = 2823593449448810005;
                                break;
                            }
                        }
                    }
                    match ch as libc::c_int {
                        32 => {
                            p_state = s_res_status_start;
                            current_block = 2823593449448810005;
                            break;
                        }
                        10 | 13 => {
                            p_state = s_res_status_start;
                        }
                        _ => {
                            (*parser).set_http_errno(15 as libc::c_uint);
                            current_block = 8656493661476283127;
                            break 's_697;
                        }
                    }
                }
                15 => {
                    if status_mark.is_null() {
                        status_mark = p;
                    }
                    p_state = s_res_status;
                    (*parser).set_index(0 as libc::c_uint);
                    if ch as libc::c_int == 13 as libc::c_int {
                        continue;
                    }
                    if !(ch as libc::c_int == 10 as libc::c_int) {
                        current_block = 2823593449448810005;
                        break;
                    }
                }
                16 => {
                    if ch as libc::c_int == 13 as libc::c_int {
                        current_block = 15140777596333492270;
                        break;
                    } else {
                        current_block = 16703904353010726814;
                        break;
                    }
                }
                17 => {
                    if ch as libc::c_int != 10 as libc::c_int {
                        current_block = 17936412886206549082;
                        break;
                    } else {
                        current_block = 1169396748315396699;
                        break;
                    }
                }
                18 => {
                    if ch as libc::c_int == 13 as libc::c_int {
                        current_block = 2823593449448810005;
                        break;
                    } else {
                        current_block = 12030841198858789628;
                        break;
                    }
                }
                19 => {
                    tmp___42 = (ch as libc::c_int == 0 as libc::c_int) as libc::c_int
                        as libc::c_long;
                    if tmp___42 != 0 {
                        current_block = 8889999340123292593;
                        break;
                    } else {
                        current_block = 16939702945023487982;
                        break;
                    }
                }
                20 => {
                    if ch as libc::c_int == 32 as libc::c_int {
                        current_block = 2823593449448810005;
                        break;
                    } else {
                        current_block = 2753962211632912778;
                        break;
                    }
                }
                24 | 23 | 22 | 21 => {
                    match ch as libc::c_int {
                        10 | 13 | 32 => {
                            current_block = 15710006729984671399;
                            break;
                        }
                        _ => {
                            current_block = 9536314573298548501;
                            break;
                        }
                    }
                }
                31 | 30 | 29 | 28 | 27 | 26 | 25 => {
                    match ch as libc::c_int {
                        32 => {
                            current_block = 1824917807432110207;
                            break;
                        }
                        10 | 13 => {
                            current_block = 3105948935974009916;
                            break;
                        }
                        _ => {
                            current_block = 10888153045549328516;
                            break;
                        }
                    }
                }
                32 => {
                    match ch as libc::c_int {
                        72 => {
                            current_block = 3239658638043768328;
                            break;
                        }
                        32 => {
                            current_block = 2823593449448810005;
                            break;
                        }
                        _ => {
                            current_block = 6162494029290681219;
                            break;
                        }
                    }
                }
                33 => {
                    if ch as libc::c_int != 84 as libc::c_int {
                        current_block = 17663900762289766202;
                        break;
                    } else {
                        current_block = 6408059329745668080;
                        break;
                    }
                }
                34 => {
                    if ch as libc::c_int != 84 as libc::c_int {
                        current_block = 12184831490747941883;
                        break;
                    } else {
                        current_block = 9448659001743590153;
                        break;
                    }
                }
                35 => {
                    if ch as libc::c_int != 80 as libc::c_int {
                        current_block = 9495722216958262053;
                        break;
                    } else {
                        current_block = 8924847599540368544;
                        break;
                    }
                }
                36 => {
                    if ch as libc::c_int != 47 as libc::c_int {
                        current_block = 4240322626966923948;
                        break;
                    } else {
                        current_block = 13052205784774183146;
                        break;
                    }
                }
                37 => {
                    if ch as libc::c_int >= 48 as libc::c_int {
                        if ch as libc::c_int <= 57 as libc::c_int {
                            tmp___60 = 0 as libc::c_int;
                        } else {
                            tmp___60 = 1 as libc::c_int;
                        }
                    } else {
                        tmp___60 = 1 as libc::c_int;
                    }
                    tmp___61 = tmp___60 as libc::c_long;
                    if tmp___61 != 0 {
                        current_block = 15340942182342897967;
                        break;
                    } else {
                        current_block = 18403398098674784951;
                        break;
                    }
                }
                38 => {
                    tmp___62 = (ch as libc::c_int != 46 as libc::c_int) as libc::c_int
                        as libc::c_long;
                    if tmp___62 != 0 {
                        current_block = 13198017010642886032;
                        break;
                    } else {
                        current_block = 15872201074975604606;
                        break;
                    }
                }
                39 => {
                    if ch as libc::c_int >= 48 as libc::c_int {
                        if ch as libc::c_int <= 57 as libc::c_int {
                            tmp___63 = 0 as libc::c_int;
                        } else {
                            tmp___63 = 1 as libc::c_int;
                        }
                    } else {
                        tmp___63 = 1 as libc::c_int;
                    }
                    tmp___64 = tmp___63 as libc::c_long;
                    if tmp___64 != 0 {
                        current_block = 18300168379702596888;
                        break;
                    } else {
                        current_block = 11298318405609849625;
                        break;
                    }
                }
                40 => {
                    if ch as libc::c_int == 13 as libc::c_int {
                        current_block = 7923635230025172457;
                        break;
                    } else {
                        current_block = 17569780573092599132;
                        break;
                    }
                }
                41 => {
                    tmp___65 = (ch as libc::c_int != 10 as libc::c_int) as libc::c_int
                        as libc::c_long;
                    if tmp___65 != 0 {
                        current_block = 12561890576527106478;
                        break;
                    } else {
                        current_block = 11721037647720733806;
                        break;
                    }
                }
                42 => {
                    if ch as libc::c_int == 13 as libc::c_int {
                        p_state = s_headers_almost_done;
                        current_block = 2823593449448810005;
                        break;
                    } else if ch as libc::c_int == 10 as libc::c_int {
                        p_state = s_headers_almost_done;
                    } else {
                        c = tokens[ch as libc::c_uchar as usize];
                        tmp___66 = (c == 0) as libc::c_int as libc::c_long;
                        if tmp___66 != 0 {
                            current_block = 9191143013813461585;
                            break;
                        } else {
                            current_block = 15287575108889763005;
                            break;
                        }
                    }
                }
                43 => {
                    start = p;
                    while p as libc::c_ulong
                        != data.offset(len as isize) as libc::c_ulong
                    {
                        ch = *p;
                        c = tokens[ch as libc::c_uchar as usize];
                        if c == 0 {
                            break;
                        }
                        match (*parser).header_state() as libc::c_int {
                            0 => {}
                            1 => {
                                (*parser).set_index((*parser).index() + 1);
                                if c as libc::c_int == 111 as libc::c_int {
                                    (*parser).set_header_state(2 as libc::c_uint);
                                } else {
                                    (*parser).set_header_state(0 as libc::c_uint);
                                }
                            }
                            2 => {
                                (*parser).set_index((*parser).index() + 1);
                                if c as libc::c_int == 110 as libc::c_int {
                                    (*parser).set_header_state(3 as libc::c_uint);
                                } else {
                                    (*parser).set_header_state(0 as libc::c_uint);
                                }
                            }
                            3 => {
                                (*parser).set_index((*parser).index() + 1);
                                match c as libc::c_int {
                                    110 => {
                                        (*parser).set_header_state(4 as libc::c_uint);
                                    }
                                    116 => {
                                        (*parser).set_header_state(6 as libc::c_uint);
                                    }
                                    _ => {
                                        (*parser).set_header_state(0 as libc::c_uint);
                                    }
                                }
                            }
                            4 => {
                                (*parser).set_index((*parser).index() + 1);
                                if (*parser).index() as libc::c_ulong
                                    > (::std::mem::size_of::<[libc::c_char; 11]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_ulong)
                                {
                                    (*parser).set_header_state(0 as libc::c_uint);
                                } else if c as libc::c_int
                                        != *(b"connection\0" as *const u8 as *const libc::c_char)
                                            .offset((*parser).index() as libc::c_int as isize)
                                            as libc::c_int
                                    {
                                    (*parser).set_header_state(0 as libc::c_uint);
                                } else if (*parser).index() as libc::c_ulong
                                        == (::std::mem::size_of::<[libc::c_char; 11]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_ulong)
                                    {
                                    (*parser).set_header_state(9 as libc::c_uint);
                                }
                            }
                            5 => {
                                (*parser).set_index((*parser).index() + 1);
                                if (*parser).index() as libc::c_ulong
                                    > (::std::mem::size_of::<[libc::c_char; 17]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_ulong)
                                {
                                    (*parser).set_header_state(0 as libc::c_uint);
                                } else if c as libc::c_int
                                        != *(b"proxy-connection\0" as *const u8
                                            as *const libc::c_char)
                                            .offset((*parser).index() as libc::c_int as isize)
                                            as libc::c_int
                                    {
                                    (*parser).set_header_state(0 as libc::c_uint);
                                } else if (*parser).index() as libc::c_ulong
                                        == (::std::mem::size_of::<[libc::c_char; 17]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_ulong)
                                    {
                                    (*parser).set_header_state(9 as libc::c_uint);
                                }
                            }
                            6 => {
                                (*parser).set_index((*parser).index() + 1);
                                if (*parser).index() as libc::c_ulong
                                    > (::std::mem::size_of::<[libc::c_char; 15]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_ulong)
                                {
                                    (*parser).set_header_state(0 as libc::c_uint);
                                } else if c as libc::c_int
                                        != *(b"content-length\0" as *const u8
                                            as *const libc::c_char)
                                            .offset((*parser).index() as libc::c_int as isize)
                                            as libc::c_int
                                    {
                                    (*parser).set_header_state(0 as libc::c_uint);
                                } else if (*parser).index() as libc::c_ulong
                                        == (::std::mem::size_of::<[libc::c_char; 15]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_ulong)
                                    {
                                    (*parser).set_header_state(10 as libc::c_uint);
                                }
                            }
                            7 => {
                                (*parser).set_index((*parser).index() + 1);
                                if (*parser).index() as libc::c_ulong
                                    > (::std::mem::size_of::<[libc::c_char; 18]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_ulong)
                                {
                                    (*parser).set_header_state(0 as libc::c_uint);
                                } else if c as libc::c_int
                                        != *(b"transfer-encoding\0" as *const u8
                                            as *const libc::c_char)
                                            .offset((*parser).index() as libc::c_int as isize)
                                            as libc::c_int
                                    {
                                    (*parser).set_header_state(0 as libc::c_uint);
                                } else if (*parser).index() as libc::c_ulong
                                        == (::std::mem::size_of::<[libc::c_char; 18]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_ulong)
                                    {
                                    (*parser).set_header_state(11 as libc::c_uint);
                                }
                            }
                            8 => {
                                (*parser).set_index((*parser).index() + 1);
                                if (*parser).index() as libc::c_ulong
                                    > (::std::mem::size_of::<[libc::c_char; 8]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_ulong)
                                {
                                    (*parser).set_header_state(0 as libc::c_uint);
                                } else if c as libc::c_int
                                        != *(b"upgrade\0" as *const u8 as *const libc::c_char)
                                            .offset((*parser).index() as libc::c_int as isize)
                                            as libc::c_int
                                    {
                                    (*parser).set_header_state(0 as libc::c_uint);
                                } else if (*parser).index() as libc::c_ulong
                                        == (::std::mem::size_of::<[libc::c_char; 8]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_ulong)
                                    {
                                    (*parser).set_header_state(12 as libc::c_uint);
                                }
                            }
                            12 | 11 | 10 | 9 => {
                                if ch as libc::c_int != 32 as libc::c_int {
                                    (*parser).set_header_state(0 as libc::c_uint);
                                }
                            }
                            _ => {
                                __assert_fail(
                                    b"0 && \"Unknown header_state\"\0" as *const u8
                                        as *const libc::c_char,
                                    b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                                    1335 as libc::c_uint,
                                    b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                                );
                            }
                        }
                        p = p.offset(1);
                    }
                    (*parser)
                        .nread = ((*parser).nread as libc::c_long
                        + p.offset_from(start) as libc::c_long) as uint32_t;
                    tmp___67 = ((*parser).nread > 81920 as libc::c_uint) as libc::c_int
                        as libc::c_long;
                    if tmp___67 != 0 {
                        current_block = 17329294740332298082;
                        break;
                    } else {
                        current_block = 16542685637124060003;
                        break;
                    }
                }
                44 => {
                    if ch as libc::c_int == 32 as libc::c_int {
                        current_block = 2823593449448810005;
                        break;
                    } else {
                        current_block = 16815518525555115870;
                        break;
                    }
                }
                47 => {
                    current_block = 11067245223857074510;
                    break;
                }
                48 => {
                    start___0 = p;
                    h_state = (*parser).header_state() as header_states;
                    loop {
                        if !(p as libc::c_ulong
                            != data.offset(len as isize) as libc::c_ulong)
                        {
                            current_block = 8432433212077272926;
                            break '_reexecute;
                        }
                        ch = *p;
                        if ch as libc::c_int == 13 as libc::c_int {
                            p_state = s_header_almost_done;
                            (*parser).set_header_state(h_state as libc::c_uint);
                            if !((*parser).http_errno() as http_errno as libc::c_uint
                                == 0 as libc::c_uint)
                            {
                                __assert_fail(
                                    b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                        as *const libc::c_char,
                                    b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                                    1445 as libc::c_uint,
                                    b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            if !header_value_mark.is_null() {
                                tmp___79 = ((*settings).on_header_value).is_some()
                                    as libc::c_int as libc::c_long;
                                if tmp___79 != 0 {
                                    (*parser).set_state(p_state as libc::c_uint);
                                    tmp___75 = (Some(
                                        ((*settings).on_header_value)
                                            .expect("non-null function pointer"),
                                    ))
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        parser,
                                        header_value_mark,
                                        p.offset_from(header_value_mark) as libc::c_long as size_t,
                                    );
                                    if 0 as libc::c_int != tmp___75 {
                                        tmp___76 = 1 as libc::c_int;
                                    } else {
                                        tmp___76 = 0 as libc::c_int;
                                    }
                                    tmp___77 = tmp___76 as libc::c_long;
                                    if tmp___77 != 0 {
                                        (*parser).set_http_errno(4 as libc::c_uint);
                                    }
                                    p_state = (*parser).state() as state;
                                    tmp___78 = ((*parser).http_errno() as http_errno
                                        as libc::c_uint != 0 as libc::c_uint) as libc::c_int
                                        as libc::c_long;
                                    if tmp___78 != 0 {
                                        return (p.offset_from(data) as libc::c_long
                                            + 1 as libc::c_long) as size_t;
                                    }
                                }
                                header_value_mark = 0 as *mut libc::c_void
                                    as *const libc::c_char;
                            }
                            current_block = 8432433212077272926;
                            break '_reexecute;
                        } else if ch as libc::c_int == 10 as libc::c_int {
                            p_state = s_header_almost_done;
                            (*parser)
                                .nread = ((*parser).nread as libc::c_long
                                + p.offset_from(start___0) as libc::c_long) as uint32_t;
                            tmp___80 = ((*parser).nread > 81920 as libc::c_uint)
                                as libc::c_int as libc::c_long;
                            if tmp___80 != 0 {
                                current_block = 8507819448703559594;
                                break;
                            } else {
                                current_block = 15021327521310868173;
                                break;
                            }
                        } else {
                            if lenient == 0 {
                                if !(ch as libc::c_int == 13 as libc::c_int) {
                                    if !(ch as libc::c_int == 10 as libc::c_int) {
                                        if !(ch as libc::c_int == 9 as libc::c_int) {
                                            if !(ch as libc::c_uchar as libc::c_int > 31 as libc::c_int)
                                            {
                                                current_block = 18304633536805257724;
                                                break '_reexecute;
                                            }
                                            if !(ch as libc::c_int != 127 as libc::c_int) {
                                                current_block = 18304633536805257724;
                                                break '_reexecute;
                                            }
                                        }
                                    }
                                }
                            }
                            c = (ch as libc::c_int | 32 as libc::c_int) as libc::c_uchar
                                as libc::c_char;
                            match h_state as libc::c_uint {
                                0 => {
                                    limit = data.offset(len as isize).offset_from(p)
                                        as libc::c_long as size_t;
                                    if limit < 81920 as libc::c_ulong {
                                        limit = limit;
                                    } else {
                                        limit = 81920 as libc::c_int as size_t;
                                    }
                                    tmp___86 = memchr(
                                        p as *const libc::c_void,
                                        '\r' as i32,
                                        limit,
                                    );
                                    p_cr = tmp___86 as *const libc::c_char;
                                    tmp___87 = memchr(
                                        p as *const libc::c_void,
                                        '\n' as i32,
                                        limit,
                                    );
                                    p_lf = tmp___87 as *const libc::c_char;
                                    if p_cr as libc::c_ulong
                                        != 0 as *mut libc::c_void as libc::c_ulong
                                    {
                                        if p_lf as libc::c_ulong
                                            != 0 as *mut libc::c_void as libc::c_ulong
                                        {
                                            if p_cr as libc::c_ulong >= p_lf as libc::c_ulong {
                                                p = p_lf;
                                            } else {
                                                p = p_cr;
                                            }
                                        } else {
                                            p = p_cr;
                                        }
                                    } else {
                                        tmp___88 = (p_lf as libc::c_ulong
                                            != 0 as *mut libc::c_void as libc::c_ulong) as libc::c_int
                                            as libc::c_long;
                                        if tmp___88 != 0 {
                                            p = p_lf;
                                        } else {
                                            p = data.offset(len as isize);
                                        }
                                    }
                                    p = p.offset(-1);
                                }
                                11 | 9 => {
                                    __assert_fail(
                                        b"0 && \"Shouldn't get here.\"\0" as *const u8
                                            as *const libc::c_char,
                                        b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                                        1492 as libc::c_uint,
                                        b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                10 => {
                                    if !(ch as libc::c_int == 32 as libc::c_int) {
                                        if ch as libc::c_int >= 48 as libc::c_int {
                                            if ch as libc::c_int <= 57 as libc::c_int {
                                                tmp___89 = 0 as libc::c_int;
                                            } else {
                                                tmp___89 = 1 as libc::c_int;
                                            }
                                        } else {
                                            tmp___89 = 1 as libc::c_int;
                                        }
                                        tmp___90 = tmp___89 as libc::c_long;
                                        if tmp___90 != 0 {
                                            (*parser).set_http_errno(25 as libc::c_uint);
                                            (*parser).set_header_state(h_state as libc::c_uint);
                                            current_block = 8656493661476283127;
                                            break 's_697;
                                        } else {
                                            t = (*parser).content_length;
                                            t = (t as libc::c_ulong).wrapping_mul(10 as libc::c_ulong)
                                                as uint64_t as uint64_t;
                                            t = (t as libc::c_ulong)
                                                .wrapping_add(
                                                    (ch as libc::c_int - 48 as libc::c_int) as uint64_t,
                                                ) as uint64_t as uint64_t;
                                            tmp___91 = ((1844674407370955160 as libc::c_ulonglong)
                                                < (*parser).content_length as libc::c_ulonglong)
                                                as libc::c_int as libc::c_long;
                                            if tmp___91 != 0 {
                                                (*parser).set_http_errno(25 as libc::c_uint);
                                                (*parser).set_header_state(h_state as libc::c_uint);
                                                current_block = 8656493661476283127;
                                                break 's_697;
                                            } else {
                                                (*parser).content_length = t;
                                            }
                                        }
                                    }
                                }
                                13 => {
                                    (*parser).set_index((*parser).index() + 1);
                                    if (*parser).index() as libc::c_ulong
                                        > (::std::mem::size_of::<[libc::c_char; 8]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_ulong)
                                    {
                                        h_state = h_general;
                                    } else if c as libc::c_int
                                            != *(b"chunked\0" as *const u8 as *const libc::c_char)
                                                .offset((*parser).index() as libc::c_int as isize)
                                                as libc::c_int
                                        {
                                        h_state = h_general;
                                    } else if (*parser).index() as libc::c_ulong
                                            == (::std::mem::size_of::<[libc::c_char; 8]>()
                                                as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_ulong)
                                        {
                                        h_state = h_transfer_encoding_chunked;
                                    }
                                }
                                14 => {
                                    if c as libc::c_int == 107 as libc::c_int {
                                        h_state = h_matching_connection_keep_alive;
                                    } else if c as libc::c_int == 99 as libc::c_int {
                                        h_state = h_matching_connection_close;
                                    } else if c as libc::c_int == 117 as libc::c_int {
                                        h_state = h_matching_connection_upgrade;
                                    } else if tokens[c as libc::c_uchar as usize] != 0 {
                                        h_state = h_matching_connection_token;
                                    } else if !(c as libc::c_int == 32 as libc::c_int) {
                                        if !(c as libc::c_int == 9 as libc::c_int) {
                                            h_state = h_general;
                                        }
                                    }
                                }
                                15 => {
                                    (*parser).set_index((*parser).index() + 1);
                                    if (*parser).index() as libc::c_ulong
                                        > (::std::mem::size_of::<[libc::c_char; 11]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_ulong)
                                    {
                                        h_state = h_matching_connection_token;
                                    } else if c as libc::c_int
                                            != *(b"keep-alive\0" as *const u8 as *const libc::c_char)
                                                .offset((*parser).index() as libc::c_int as isize)
                                                as libc::c_int
                                        {
                                        h_state = h_matching_connection_token;
                                    } else if (*parser).index() as libc::c_ulong
                                            == (::std::mem::size_of::<[libc::c_char; 11]>()
                                                as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_ulong)
                                        {
                                        h_state = h_connection_keep_alive;
                                    }
                                }
                                16 => {
                                    (*parser).set_index((*parser).index() + 1);
                                    if (*parser).index() as libc::c_ulong
                                        > (::std::mem::size_of::<[libc::c_char; 6]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_ulong)
                                    {
                                        h_state = h_matching_connection_token;
                                    } else if c as libc::c_int
                                            != *(b"close\0" as *const u8 as *const libc::c_char)
                                                .offset((*parser).index() as libc::c_int as isize)
                                                as libc::c_int
                                        {
                                        h_state = h_matching_connection_token;
                                    } else if (*parser).index() as libc::c_ulong
                                            == (::std::mem::size_of::<[libc::c_char; 6]>()
                                                as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_ulong)
                                        {
                                        h_state = h_connection_close;
                                    }
                                }
                                17 => {
                                    (*parser).set_index((*parser).index() + 1);
                                    if (*parser).index() as libc::c_ulong
                                        > (::std::mem::size_of::<[libc::c_char; 8]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_ulong)
                                    {
                                        h_state = h_matching_connection_token;
                                    } else if c as libc::c_int
                                            != *(b"upgrade\0" as *const u8 as *const libc::c_char)
                                                .offset((*parser).index() as libc::c_int as isize)
                                                as libc::c_int
                                        {
                                        h_state = h_matching_connection_token;
                                    } else if (*parser).index() as libc::c_ulong
                                            == (::std::mem::size_of::<[libc::c_char; 8]>()
                                                as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_ulong)
                                        {
                                        h_state = h_connection_upgrade;
                                    }
                                }
                                18 => {
                                    if ch as libc::c_int == 44 as libc::c_int {
                                        h_state = h_matching_connection_token_start;
                                        (*parser).set_index(0 as libc::c_uint);
                                    }
                                }
                                19 => {
                                    if ch as libc::c_int != 32 as libc::c_int {
                                        h_state = h_general;
                                    }
                                }
                                22 | 21 | 20 => {
                                    if ch as libc::c_int == 44 as libc::c_int {
                                        if h_state as libc::c_uint == 20 as libc::c_uint {
                                            (*parser).set_flags((*parser).flags() | 2 as libc::c_uint);
                                        } else if h_state as libc::c_uint == 21 as libc::c_uint {
                                            (*parser).set_flags((*parser).flags() | 4 as libc::c_uint);
                                        } else if h_state as libc::c_uint == 22 as libc::c_uint {
                                            (*parser).set_flags((*parser).flags() | 8 as libc::c_uint);
                                        }
                                        h_state = h_matching_connection_token_start;
                                        (*parser).set_index(0 as libc::c_uint);
                                    } else if ch as libc::c_int != 32 as libc::c_int {
                                        h_state = h_matching_connection_token;
                                    }
                                }
                                _ => {
                                    p_state = s_header_value;
                                    h_state = h_general;
                                }
                            }
                            p = p.offset(1);
                        }
                    }
                    match current_block {
                        8507819448703559594 => {
                            (*parser).set_http_errno(12 as libc::c_uint);
                            current_block = 8656493661476283127;
                            break 's_697;
                        }
                        _ => {
                            (*parser).set_header_state(h_state as libc::c_uint);
                            if !((*parser).http_errno() as http_errno as libc::c_uint
                                == 0 as libc::c_uint)
                            {
                                __assert_fail(
                                    b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                        as *const libc::c_char,
                                    b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                                    1453 as libc::c_uint,
                                    b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            if !header_value_mark.is_null() {
                                tmp___85 = ((*settings).on_header_value).is_some()
                                    as libc::c_int as libc::c_long;
                                if tmp___85 != 0 {
                                    (*parser).set_state(p_state as libc::c_uint);
                                    tmp___81 = (Some(
                                        ((*settings).on_header_value)
                                            .expect("non-null function pointer"),
                                    ))
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        parser,
                                        header_value_mark,
                                        p.offset_from(header_value_mark) as libc::c_long as size_t,
                                    );
                                    if 0 as libc::c_int != tmp___81 {
                                        tmp___82 = 1 as libc::c_int;
                                    } else {
                                        tmp___82 = 0 as libc::c_int;
                                    }
                                    tmp___83 = tmp___82 as libc::c_long;
                                    if tmp___83 != 0 {
                                        (*parser).set_http_errno(4 as libc::c_uint);
                                    }
                                    p_state = (*parser).state() as state;
                                    tmp___84 = ((*parser).http_errno() as http_errno
                                        as libc::c_uint != 0 as libc::c_uint) as libc::c_int
                                        as libc::c_long;
                                    if tmp___84 != 0 {
                                        return p.offset_from(data) as libc::c_long as size_t;
                                    }
                                }
                                header_value_mark = 0 as *mut libc::c_void
                                    as *const libc::c_char;
                            }
                        }
                    }
                }
                50 => {
                    tmp___93 = (ch as libc::c_int != 10 as libc::c_int) as libc::c_int
                        as libc::c_long;
                    if tmp___93 != 0 {
                        current_block = 12652763302434162366;
                        break;
                    } else {
                        current_block = 6702610100809040202;
                        break;
                    }
                }
                49 => {
                    if ch as libc::c_int == 32 as libc::c_int {
                        p_state = s_header_value_start;
                    } else if ch as libc::c_int == 9 as libc::c_int {
                        p_state = s_header_value_start;
                    } else {
                        match (*parser).header_state() as libc::c_int {
                            20 => {
                                (*parser).set_flags((*parser).flags() | 2 as libc::c_uint);
                            }
                            21 => {
                                (*parser).set_flags((*parser).flags() | 4 as libc::c_uint);
                            }
                            19 => {
                                (*parser).set_flags((*parser).flags() | 1 as libc::c_uint);
                            }
                            22 => {
                                (*parser).set_flags((*parser).flags() | 8 as libc::c_uint);
                            }
                            _ => {}
                        }
                        p_state = s_header_field_start;
                    }
                }
                45 => {
                    if ch as libc::c_int != 10 as libc::c_int {
                        current_block = 17335268068337147299;
                        break;
                    } else {
                        current_block = 10114390662193834916;
                        break;
                    }
                }
                46 => {
                    if ch as libc::c_int == 32 as libc::c_int {
                        p_state = s_header_value_discard_ws;
                        current_block = 2823593449448810005;
                        break;
                    } else if ch as libc::c_int == 9 as libc::c_int {
                        p_state = s_header_value_discard_ws;
                        current_block = 2823593449448810005;
                        break;
                    } else {
                        match (*parser).header_state() as libc::c_int {
                            20 => {
                                (*parser).set_flags((*parser).flags() | 2 as libc::c_uint);
                            }
                            21 => {
                                (*parser).set_flags((*parser).flags() | 4 as libc::c_uint);
                            }
                            22 => {
                                (*parser).set_flags((*parser).flags() | 8 as libc::c_uint);
                            }
                            19 => {
                                (*parser).set_flags((*parser).flags() | 1 as libc::c_uint);
                            }
                            _ => {}
                        }
                        if header_value_mark.is_null() {
                            header_value_mark = p;
                        }
                        p_state = s_header_field_start;
                        if !((*parser).http_errno() as http_errno as libc::c_uint
                            == 0 as libc::c_uint)
                        {
                            __assert_fail(
                                b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                    as *const libc::c_char,
                                b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                                1700 as libc::c_uint,
                                b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        if !header_value_mark.is_null() {
                            tmp___98 = ((*settings).on_header_value).is_some()
                                as libc::c_int as libc::c_long;
                            if tmp___98 != 0 {
                                (*parser).set_state(p_state as libc::c_uint);
                                tmp___94 = (Some(
                                    ((*settings).on_header_value)
                                        .expect("non-null function pointer"),
                                ))
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    parser,
                                    header_value_mark,
                                    p.offset_from(header_value_mark) as libc::c_long as size_t,
                                );
                                if 0 as libc::c_int != tmp___94 {
                                    tmp___95 = 1 as libc::c_int;
                                } else {
                                    tmp___95 = 0 as libc::c_int;
                                }
                                tmp___96 = tmp___95 as libc::c_long;
                                if tmp___96 != 0 {
                                    (*parser).set_http_errno(4 as libc::c_uint);
                                }
                                p_state = (*parser).state() as state;
                                tmp___97 = ((*parser).http_errno() as http_errno
                                    as libc::c_uint != 0 as libc::c_uint) as libc::c_int
                                    as libc::c_long;
                                if tmp___97 != 0 {
                                    return p.offset_from(data) as libc::c_long as size_t;
                                }
                            }
                            header_value_mark = 0 as *mut libc::c_void
                                as *const libc::c_char;
                        }
                    }
                }
                55 => {
                    if ch as libc::c_int != 10 as libc::c_int {
                        (*parser).set_http_errno(30 as libc::c_uint);
                        current_block = 8656493661476283127;
                        break 's_697;
                    } else if (*parser).flags() & 16 as libc::c_uint != 0 {
                        p_state = s_message_done;
                        if !((*parser).http_errno() as http_errno as libc::c_uint
                            == 0 as libc::c_uint)
                        {
                            __assert_fail(
                                b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                    as *const libc::c_char,
                                b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                                1712 as libc::c_uint,
                                b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        tmp___103 = ((*settings).on_chunk_complete).is_some()
                            as libc::c_int as libc::c_long;
                        if tmp___103 != 0 {
                            (*parser).set_state(p_state as libc::c_uint);
                            tmp___99 = (Some(
                                ((*settings).on_chunk_complete)
                                    .expect("non-null function pointer"),
                            ))
                                .expect("non-null function pointer")(parser);
                            if 0 as libc::c_int != tmp___99 {
                                tmp___100 = 1 as libc::c_int;
                            } else {
                                tmp___100 = 0 as libc::c_int;
                            }
                            tmp___101 = tmp___100 as libc::c_long;
                            if tmp___101 != 0 {
                                (*parser).set_http_errno(10 as libc::c_uint);
                            }
                            p_state = (*parser).state() as state;
                            tmp___102 = ((*parser).http_errno() as http_errno
                                as libc::c_uint != 0 as libc::c_uint) as libc::c_int
                                as libc::c_long;
                            if tmp___102 != 0 {
                                return p.offset_from(data) as libc::c_long as size_t;
                            }
                        }
                    } else {
                        if (*parser).flags() & 1 as libc::c_uint != 0 {
                            if (*parser).flags() & 128 as libc::c_uint != 0 {
                                (*parser).set_http_errno(26 as libc::c_uint);
                                current_block = 8656493661476283127;
                                break 's_697;
                            }
                        }
                        p_state = s_headers_done;
                        if (*parser).flags() & 32 as libc::c_uint != 0 {
                            if (*parser).flags() & 8 as libc::c_uint != 0 {
                                if (*parser).type_0() == 0 as libc::c_uint {
                                    tmp___104 = 1 as libc::c_int;
                                } else if (*parser).status_code() == 101 as libc::c_uint {
                                    tmp___104 = 1 as libc::c_int;
                                } else {
                                    tmp___104 = 0 as libc::c_int;
                                }
                                (*parser).set_upgrade(tmp___104 as libc::c_uint);
                            } else {
                                (*parser)
                                    .set_upgrade(
                                        ((*parser).method() == 5 as libc::c_uint) as libc::c_int
                                            as libc::c_uint,
                                    );
                            }
                        } else {
                            (*parser)
                                .set_upgrade(
                                    ((*parser).method() == 5 as libc::c_uint) as libc::c_int
                                        as libc::c_uint,
                                );
                        }
                        if ((*settings).on_headers_complete).is_some() {
                            tmp___105 = (Some(
                                ((*settings).on_headers_complete)
                                    .expect("non-null function pointer"),
                            ))
                                .expect("non-null function pointer")(parser);
                            let mut current_block_985: u64;
                            match tmp___105 {
                                0 => {
                                    current_block_985 = 8329004387349778464;
                                }
                                2 => {
                                    (*parser).set_upgrade(1 as libc::c_uint);
                                    current_block_985 = 11735567287435770986;
                                }
                                1 => {
                                    current_block_985 = 11735567287435770986;
                                }
                                _ => {
                                    (*parser).set_http_errno(5 as libc::c_uint);
                                    (*parser).set_state(p_state as libc::c_uint);
                                    return p.offset_from(data) as libc::c_long as size_t;
                                }
                            }
                            match current_block_985 {
                                11735567287435770986 => {
                                    (*parser).set_flags((*parser).flags() | 64 as libc::c_uint);
                                }
                                _ => {}
                            }
                        }
                        if (*parser).http_errno() as http_errno as libc::c_uint
                            != 0 as libc::c_uint
                        {
                            (*parser).set_state(p_state as libc::c_uint);
                            return p.offset_from(data) as libc::c_long as size_t;
                        }
                    }
                }
                56 => {
                    if ch as libc::c_int != 10 as libc::c_int {
                        current_block = 13843135405533155496;
                        break;
                    } else {
                        current_block = 10194173116391654254;
                        break;
                    }
                }
                60 => {
                    if (*parser).content_length
                        < data.offset(len as isize).offset_from(p) as libc::c_long
                            as uint64_t
                    {
                        tmp___144 = (*parser).content_length;
                    } else {
                        tmp___144 = data.offset(len as isize).offset_from(p)
                            as libc::c_long as uint64_t;
                    }
                    to_read = tmp___144;
                    if (*parser).content_length != 0 as libc::c_ulong {
                        if !((*parser).content_length as libc::c_ulonglong
                            != 18446744073709551615 as libc::c_ulonglong)
                        {
                            __assert_fail(
                                b"parser->content_length != 0 && parser->content_length != ULLONG_MAX\0"
                                    as *const u8 as *const libc::c_char,
                                b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                                1826 as libc::c_uint,
                                b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                            );
                        }
                    } else {
                        __assert_fail(
                            b"parser->content_length != 0 && parser->content_length != ULLONG_MAX\0"
                                as *const u8 as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1826 as libc::c_uint,
                            b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    if body_mark.is_null() {
                        body_mark = p;
                    }
                    (*parser)
                        .content_length = ((*parser).content_length as libc::c_ulong)
                        .wrapping_sub(to_read) as uint64_t as uint64_t;
                    p = p.offset(to_read.wrapping_sub(1 as libc::c_ulong) as isize);
                    if !((*parser).content_length == 0 as libc::c_ulong) {
                        current_block = 2823593449448810005;
                        break;
                    }
                    p_state = s_message_done;
                    if !((*parser).http_errno() as http_errno as libc::c_uint
                        == 0 as libc::c_uint)
                    {
                        __assert_fail(
                            b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1849 as libc::c_uint,
                            b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    if !body_mark.is_null() {
                        tmp___149 = ((*settings).on_body).is_some() as libc::c_int
                            as libc::c_long;
                        if tmp___149 != 0 {
                            (*parser).set_state(p_state as libc::c_uint);
                            tmp___145 = (Some(
                                ((*settings).on_body).expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                parser,
                                body_mark,
                                (p.offset_from(body_mark) as libc::c_long
                                    + 1 as libc::c_long) as size_t,
                            );
                            if 0 as libc::c_int != tmp___145 {
                                tmp___146 = 1 as libc::c_int;
                            } else {
                                tmp___146 = 0 as libc::c_int;
                            }
                            tmp___147 = tmp___146 as libc::c_long;
                            if tmp___147 != 0 {
                                (*parser).set_http_errno(6 as libc::c_uint);
                            }
                            p_state = (*parser).state() as state;
                            tmp___148 = ((*parser).http_errno() as http_errno
                                as libc::c_uint != 0 as libc::c_uint) as libc::c_int
                                as libc::c_long;
                            if tmp___148 != 0 {
                                return p.offset_from(data) as libc::c_long as size_t;
                            }
                        }
                        body_mark = 0 as *mut libc::c_void as *const libc::c_char;
                    }
                }
                61 => {
                    if body_mark.is_null() {
                        body_mark = p;
                    }
                    p = data.offset(len as isize).offset(-(1 as libc::c_int as isize));
                    current_block = 2823593449448810005;
                    break;
                }
                62 => {
                    tmp___153 = http_should_keep_alive(parser as *const http_parser);
                    if tmp___153 != 0 {
                        if (*parser).type_0() == 0 as libc::c_uint {
                            tmp___151 = 18 as libc::c_int;
                        } else {
                            tmp___151 = 4 as libc::c_int;
                        }
                        tmp___152 = tmp___151;
                    } else {
                        tmp___152 = 1 as libc::c_int;
                    }
                    p_state = tmp___152 as state;
                    if !((*parser).http_errno() as http_errno as libc::c_uint
                        == 0 as libc::c_uint)
                    {
                        __assert_fail(
                            b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1865 as libc::c_uint,
                            b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    tmp___158 = ((*settings).on_message_complete).is_some()
                        as libc::c_int as libc::c_long;
                    if tmp___158 != 0 {
                        (*parser).set_state(p_state as libc::c_uint);
                        tmp___154 = (Some(
                            ((*settings).on_message_complete)
                                .expect("non-null function pointer"),
                        ))
                            .expect("non-null function pointer")(parser);
                        if 0 as libc::c_int != tmp___154 {
                            tmp___155 = 1 as libc::c_int;
                        } else {
                            tmp___155 = 0 as libc::c_int;
                        }
                        tmp___156 = tmp___155 as libc::c_long;
                        if tmp___156 != 0 {
                            (*parser).set_http_errno(7 as libc::c_uint);
                        }
                        p_state = (*parser).state() as state;
                        tmp___157 = ((*parser).http_errno() as http_errno as libc::c_uint
                            != 0 as libc::c_uint) as libc::c_int as libc::c_long;
                        if tmp___157 != 0 {
                            return (p.offset_from(data) as libc::c_long
                                + 1 as libc::c_long) as size_t;
                        }
                    }
                    if (*parser).upgrade() != 0 {
                        (*parser).set_state(p_state as libc::c_uint);
                        return (p.offset_from(data) as libc::c_long + 1 as libc::c_long)
                            as size_t;
                    }
                    current_block = 2823593449448810005;
                    break;
                }
                51 => {
                    if !((*parser).nread == 1 as libc::c_uint) {
                        __assert_fail(
                            b"parser->nread == 1\0" as *const u8 as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1874 as libc::c_uint,
                            b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    if (*parser).flags() & 1 as libc::c_uint == 0 {
                        __assert_fail(
                            b"parser->flags & F_CHUNKED\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1875 as libc::c_uint,
                            b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    unhex_val = unhex[ch as libc::c_uchar as usize];
                    tmp___159 = (unhex_val as libc::c_int == -(1 as libc::c_int))
                        as libc::c_int as libc::c_long;
                    if tmp___159 != 0 {
                        current_block = 12520299732166378225;
                        break;
                    } else {
                        current_block = 16257149083642269134;
                        break;
                    }
                }
                52 => {
                    if (*parser).flags() & 1 as libc::c_uint == 0 {
                        __assert_fail(
                            b"parser->flags & F_CHUNKED\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1892 as libc::c_uint,
                            b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    if ch as libc::c_int == 13 as libc::c_int {
                        current_block = 10545230836686041720;
                        break;
                    } else {
                        current_block = 7658707337339137753;
                        break;
                    }
                }
                53 => {
                    if (*parser).flags() & 1 as libc::c_uint == 0 {
                        __assert_fail(
                            b"parser->flags & F_CHUNKED\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1927 as libc::c_uint,
                            b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    if ch as libc::c_int == 13 as libc::c_int {
                        current_block = 17464762320025024339;
                        break;
                    } else {
                        current_block = 2823593449448810005;
                        break;
                    }
                }
                54 => {
                    if (*parser).flags() & 1 as libc::c_uint == 0 {
                        __assert_fail(
                            b"parser->flags & F_CHUNKED\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1938 as libc::c_uint,
                            b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    if ch as libc::c_int != 10 as libc::c_int {
                        current_block = 1317541187880551388;
                        break;
                    } else {
                        current_block = 61686342394866974;
                        break;
                    }
                }
                57 => {
                    if (*parser).content_length
                        < data.offset(len as isize).offset_from(p) as libc::c_long
                            as uint64_t
                    {
                        tmp___166 = (*parser).content_length;
                    } else {
                        tmp___166 = data.offset(len as isize).offset_from(p)
                            as libc::c_long as uint64_t;
                    }
                    to_read___0 = tmp___166;
                    if (*parser).flags() & 1 as libc::c_uint == 0 {
                        __assert_fail(
                            b"parser->flags & F_CHUNKED\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1958 as libc::c_uint,
                            b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    if (*parser).content_length != 0 as libc::c_ulong {
                        if !((*parser).content_length as libc::c_ulonglong
                            != 18446744073709551615 as libc::c_ulonglong)
                        {
                            __assert_fail(
                                b"parser->content_length != 0 && parser->content_length != ULLONG_MAX\0"
                                    as *const u8 as *const libc::c_char,
                                b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                                1960 as libc::c_uint,
                                b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                            );
                        }
                    } else {
                        __assert_fail(
                            b"parser->content_length != 0 && parser->content_length != ULLONG_MAX\0"
                                as *const u8 as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1960 as libc::c_uint,
                            b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    if body_mark.is_null() {
                        body_mark = p;
                    }
                    (*parser)
                        .content_length = ((*parser).content_length as libc::c_ulong)
                        .wrapping_sub(to_read___0) as uint64_t as uint64_t;
                    p = p.offset(to_read___0.wrapping_sub(1 as libc::c_ulong) as isize);
                    if (*parser).content_length == 0 as libc::c_ulong {
                        p_state = s_chunk_data_almost_done;
                    }
                    current_block = 2823593449448810005;
                    break;
                }
                58 => {
                    if (*parser).flags() & 1 as libc::c_uint == 0 {
                        __assert_fail(
                            b"parser->flags & F_CHUNKED\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1977 as libc::c_uint,
                            b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    if !((*parser).content_length == 0 as libc::c_ulong) {
                        __assert_fail(
                            b"parser->content_length == 0\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1978 as libc::c_uint,
                            b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    if ch as libc::c_int != 13 as libc::c_int {
                        current_block = 17643269856826482833;
                        break;
                    } else {
                        current_block = 10831039602972093699;
                        break;
                    }
                }
                59 => {
                    if (*parser).flags() & 1 as libc::c_uint == 0 {
                        __assert_fail(
                            b"parser->flags & F_CHUNKED\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1985 as libc::c_uint,
                            b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    if ch as libc::c_int != 10 as libc::c_int {
                        current_block = 1547916710919041324;
                        break;
                    } else {
                        current_block = 10962111400457132340;
                        break;
                    }
                }
                _ => {
                    __assert_fail(
                        b"0 && \"unhandled state\"\0" as *const u8
                            as *const libc::c_char,
                        b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                        1993 as libc::c_uint,
                        b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
        match current_block {
            9536314573298548501 => {
                tmp___45 = parse_url_char(p_state, ch);
                p_state = tmp___45;
                tmp___46 = (p_state as libc::c_uint == 1 as libc::c_uint) as libc::c_int
                    as libc::c_long;
                if tmp___46 != 0 {
                    (*parser).set_http_errno(17 as libc::c_uint);
                    current_block = 8656493661476283127;
                    break;
                } else {
                    current_block = 2823593449448810005;
                }
            }
            2753962211632912778 => {
                if url_mark.is_null() {
                    url_mark = p;
                }
                if (*parser).method() == 5 as libc::c_uint {
                    p_state = s_req_server_start;
                }
                tmp___43 = parse_url_char(p_state, ch);
                p_state = tmp___43;
                tmp___44 = (p_state as libc::c_uint == 1 as libc::c_uint) as libc::c_int
                    as libc::c_long;
                if tmp___44 != 0 {
                    (*parser).set_http_errno(17 as libc::c_uint);
                    current_block = 8656493661476283127;
                    break;
                } else {
                    current_block = 2823593449448810005;
                }
            }
            16939702945023487982 => {
                matcher = method_strings[(*parser).method() as usize];
                if ch as libc::c_int == 32 as libc::c_int {
                    if *matcher.offset((*parser).index() as libc::c_int as isize)
                        as libc::c_int == 0 as libc::c_int
                    {
                        p_state = s_req_spaces_before_url;
                        current_block = 17633582168621927411;
                    } else {
                        current_block = 11285773165624774798;
                    }
                } else {
                    current_block = 11285773165624774798;
                }
                match current_block {
                    11285773165624774798 => {
                        if !(ch as libc::c_int
                            == *matcher.offset((*parser).index() as libc::c_int as isize)
                                as libc::c_int)
                        {
                            if ch as libc::c_int >= 65 as libc::c_int {
                                if ch as libc::c_int <= 90 as libc::c_int {
                                    current_block = 18165810986836730121;
                                } else {
                                    current_block = 16209767926218586426;
                                }
                            } else {
                                current_block = 16209767926218586426;
                            }
                            match current_block {
                                16209767926218586426 => {
                                    if !(ch as libc::c_int == 45 as libc::c_int) {
                                        (*parser).set_http_errno(16 as libc::c_uint);
                                        current_block = 8656493661476283127;
                                        break;
                                    }
                                }
                                _ => {}
                            }
                            match (((*parser).method() as libc::c_int)
                                << 16 as libc::c_int
                                | ((*parser).index() as libc::c_int) << 8 as libc::c_int)
                                as libc::c_uint | ch as libc::c_uint
                            {
                                196949 => {
                                    (*parser).set_method(4 as libc::c_uint);
                                }
                                196929 => {
                                    (*parser).set_method(28 as libc::c_uint);
                                }
                                196946 => {
                                    (*parser).set_method(12 as libc::c_uint);
                                }
                                262738 => {
                                    (*parser).set_method(29 as libc::c_uint);
                                }
                                328008 => {
                                    (*parser).set_method(22 as libc::c_uint);
                                }
                                328272 => {
                                    (*parser).set_method(8 as libc::c_uint);
                                }
                                655695 => {
                                    (*parser).set_method(11 as libc::c_uint);
                                }
                                655685 => {
                                    (*parser).set_method(23 as libc::c_uint);
                                }
                                655661 => {
                                    (*parser).set_method(24 as libc::c_uint);
                                }
                                655937 => {
                                    (*parser).set_method(21 as libc::c_uint);
                                }
                                656193 => {
                                    (*parser).set_method(30 as libc::c_uint);
                                }
                                1704261 => {
                                    (*parser).set_method(14 as libc::c_uint);
                                }
                                1311298 => {
                                    (*parser).set_method(17 as libc::c_uint);
                                }
                                787536 => {
                                    (*parser).set_method(13 as libc::c_uint);
                                }
                                590153 => {
                                    (*parser).set_method(31 as libc::c_uint);
                                }
                                983635 => {
                                    (*parser).set_method(27 as libc::c_uint);
                                }
                                983618 => {
                                    (*parser).set_method(18 as libc::c_uint);
                                }
                                983881 => {
                                    (*parser).set_method(32 as libc::c_uint);
                                }
                                _ => {
                                    (*parser).set_http_errno(16 as libc::c_uint);
                                    current_block = 8656493661476283127;
                                    break;
                                }
                            }
                        }
                    }
                    _ => {}
                }
                (*parser).set_index((*parser).index() + 1);
                current_block = 2823593449448810005;
            }
            10888153045549328516 => {
                tmp___58 = parse_url_char(p_state, ch);
                p_state = tmp___58;
                tmp___59 = (p_state as libc::c_uint == 1 as libc::c_uint) as libc::c_int
                    as libc::c_long;
                if tmp___59 != 0 {
                    (*parser).set_http_errno(17 as libc::c_uint);
                    current_block = 8656493661476283127;
                    break;
                } else {
                    current_block = 2823593449448810005;
                }
            }
            12030841198858789628 => {
                if ch as libc::c_int == 10 as libc::c_int {
                    current_block = 2823593449448810005;
                } else {
                    (*parser).set_flags(0 as libc::c_uint);
                    (*parser)
                        .content_length = 18446744073709551615 as libc::c_ulonglong
                        as uint64_t;
                    if (ch as libc::c_int | 32 as libc::c_int) as libc::c_uchar
                        as libc::c_int >= 97 as libc::c_int
                    {
                        if (ch as libc::c_int | 32 as libc::c_int) as libc::c_uchar
                            as libc::c_int <= 122 as libc::c_int
                        {
                            tmp___35 = 0 as libc::c_int;
                        } else {
                            tmp___35 = 1 as libc::c_int;
                        }
                    } else {
                        tmp___35 = 1 as libc::c_int;
                    }
                    tmp___36 = tmp___35 as libc::c_long;
                    if tmp___36 != 0 {
                        (*parser).set_http_errno(16 as libc::c_uint);
                        current_block = 8656493661476283127;
                        break;
                    } else {
                        (*parser).set_method(HTTP_DELETE as libc::c_uint);
                        (*parser).set_index(1 as libc::c_uint);
                        match ch as libc::c_int {
                            65 => {
                                (*parser).set_method(19 as libc::c_uint);
                            }
                            66 => {
                                (*parser).set_method(16 as libc::c_uint);
                            }
                            67 => {
                                (*parser).set_method(5 as libc::c_uint);
                            }
                            68 => {
                                (*parser).set_method(0 as libc::c_uint);
                            }
                            71 => {
                                (*parser).set_method(1 as libc::c_uint);
                            }
                            72 => {
                                (*parser).set_method(2 as libc::c_uint);
                            }
                            76 => {
                                (*parser).set_method(9 as libc::c_uint);
                            }
                            77 => {
                                (*parser).set_method(10 as libc::c_uint);
                            }
                            78 => {
                                (*parser).set_method(25 as libc::c_uint);
                            }
                            79 => {
                                (*parser).set_method(6 as libc::c_uint);
                            }
                            80 => {
                                (*parser).set_method(3 as libc::c_uint);
                            }
                            82 => {
                                (*parser).set_method(20 as libc::c_uint);
                            }
                            83 => {
                                (*parser).set_method(26 as libc::c_uint);
                            }
                            84 => {
                                (*parser).set_method(7 as libc::c_uint);
                            }
                            85 => {
                                (*parser).set_method(15 as libc::c_uint);
                            }
                            _ => {
                                (*parser).set_http_errno(16 as libc::c_uint);
                                current_block = 8656493661476283127;
                                break;
                            }
                        }
                        p_state = s_req_method;
                        if !((*parser).http_errno() as http_errno as libc::c_uint
                            == 0 as libc::c_uint)
                        {
                            __assert_fail(
                                b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                    as *const libc::c_char,
                                b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                                955 as libc::c_uint,
                                b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        tmp___41 = ((*settings).on_message_begin).is_some()
                            as libc::c_int as libc::c_long;
                        if tmp___41 != 0 {
                            (*parser).set_state(p_state as libc::c_uint);
                            tmp___37 = (Some(
                                ((*settings).on_message_begin)
                                    .expect("non-null function pointer"),
                            ))
                                .expect("non-null function pointer")(parser);
                            if 0 as libc::c_int != tmp___37 {
                                tmp___38 = 1 as libc::c_int;
                            } else {
                                tmp___38 = 0 as libc::c_int;
                            }
                            tmp___39 = tmp___38 as libc::c_long;
                            if tmp___39 != 0 {
                                (*parser).set_http_errno(1 as libc::c_uint);
                            }
                            p_state = (*parser).state() as state;
                            tmp___40 = ((*parser).http_errno() as http_errno
                                as libc::c_uint != 0 as libc::c_uint) as libc::c_int
                                as libc::c_long;
                            if tmp___40 != 0 {
                                return (p.offset_from(data) as libc::c_long
                                    + 1 as libc::c_long) as size_t;
                            }
                        }
                    }
                    current_block = 2823593449448810005;
                }
            }
            16703904353010726814 => {
                if ch as libc::c_int == 10 as libc::c_int {
                    p_state = s_header_field_start;
                    if !((*parser).http_errno() as http_errno as libc::c_uint
                        == 0 as libc::c_uint)
                    {
                        __assert_fail(
                            b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            906 as libc::c_uint,
                            b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    if !status_mark.is_null() {
                        tmp___34 = ((*settings).on_status).is_some() as libc::c_int
                            as libc::c_long;
                        if tmp___34 != 0 {
                            (*parser).set_state(p_state as libc::c_uint);
                            tmp___30 = (Some(
                                ((*settings).on_status).expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                parser,
                                status_mark,
                                p.offset_from(status_mark) as libc::c_long as size_t,
                            );
                            if 0 as libc::c_int != tmp___30 {
                                tmp___31 = 1 as libc::c_int;
                            } else {
                                tmp___31 = 0 as libc::c_int;
                            }
                            tmp___32 = tmp___31 as libc::c_long;
                            if tmp___32 != 0 {
                                (*parser).set_http_errno(8 as libc::c_uint);
                            }
                            p_state = (*parser).state() as state;
                            tmp___33 = ((*parser).http_errno() as http_errno
                                as libc::c_uint != 0 as libc::c_uint) as libc::c_int
                                as libc::c_long;
                            if tmp___33 != 0 {
                                return (p.offset_from(data) as libc::c_long
                                    + 1 as libc::c_long) as size_t;
                            }
                        }
                        status_mark = 0 as *mut libc::c_void as *const libc::c_char;
                    }
                    current_block = 2823593449448810005;
                } else {
                    current_block = 2823593449448810005;
                }
            }
            15140777596333492270 => {
                p_state = s_res_line_almost_done;
                if !((*parser).http_errno() as http_errno as libc::c_uint
                    == 0 as libc::c_uint)
                {
                    __assert_fail(
                        b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                            as *const libc::c_char,
                        b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                        900 as libc::c_uint,
                        b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                    );
                }
                if !status_mark.is_null() {
                    tmp___29 = ((*settings).on_status).is_some() as libc::c_int
                        as libc::c_long;
                    if tmp___29 != 0 {
                        (*parser).set_state(p_state as libc::c_uint);
                        tmp___25 = (Some(
                            ((*settings).on_status).expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            parser,
                            status_mark,
                            p.offset_from(status_mark) as libc::c_long as size_t,
                        );
                        if 0 as libc::c_int != tmp___25 {
                            tmp___26 = 1 as libc::c_int;
                        } else {
                            tmp___26 = 0 as libc::c_int;
                        }
                        tmp___27 = tmp___26 as libc::c_long;
                        if tmp___27 != 0 {
                            (*parser).set_http_errno(8 as libc::c_uint);
                        }
                        p_state = (*parser).state() as state;
                        tmp___28 = ((*parser).http_errno() as http_errno as libc::c_uint
                            != 0 as libc::c_uint) as libc::c_int as libc::c_long;
                        if tmp___28 != 0 {
                            return (p.offset_from(data) as libc::c_long
                                + 1 as libc::c_long) as size_t;
                        }
                    }
                    status_mark = 0 as *mut libc::c_void as *const libc::c_char;
                }
                current_block = 2823593449448810005;
            }
            3105948935974009916 => {
                (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                if ch as libc::c_int == 13 as libc::c_int {
                    tmp___52 = 41 as libc::c_int;
                } else {
                    tmp___52 = 42 as libc::c_int;
                }
                p_state = tmp___52 as state;
                if !((*parser).http_errno() as http_errno as libc::c_uint
                    == 0 as libc::c_uint)
                {
                    __assert_fail(
                        b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                            as *const libc::c_char,
                        b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                        1073 as libc::c_uint,
                        b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                    );
                }
                if !url_mark.is_null() {
                    tmp___57 = ((*settings).on_url).is_some() as libc::c_int
                        as libc::c_long;
                    if tmp___57 != 0 {
                        (*parser).set_state(p_state as libc::c_uint);
                        tmp___53 = (Some(
                            ((*settings).on_url).expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            parser,
                            url_mark,
                            p.offset_from(url_mark) as libc::c_long as size_t,
                        );
                        if 0 as libc::c_int != tmp___53 {
                            tmp___54 = 1 as libc::c_int;
                        } else {
                            tmp___54 = 0 as libc::c_int;
                        }
                        tmp___55 = tmp___54 as libc::c_long;
                        if tmp___55 != 0 {
                            (*parser).set_http_errno(2 as libc::c_uint);
                        }
                        p_state = (*parser).state() as state;
                        tmp___56 = ((*parser).http_errno() as http_errno as libc::c_uint
                            != 0 as libc::c_uint) as libc::c_int as libc::c_long;
                        if tmp___56 != 0 {
                            return (p.offset_from(data) as libc::c_long
                                + 1 as libc::c_long) as size_t;
                        }
                    }
                    url_mark = 0 as *mut libc::c_void as *const libc::c_char;
                }
                current_block = 2823593449448810005;
            }
            15775903806355281185 => {
                if !(ch as libc::c_int <= 57 as libc::c_int) {
                    current_block = 1840006672846692558;
                } else {
                    (*parser)
                        .set_status_code(
                            (ch as libc::c_int - 48 as libc::c_int) as libc::c_uint,
                        );
                    p_state = s_res_status_code;
                    current_block = 2823593449448810005;
                }
            }
            14123347050775008060 => {
                (*parser)
                    .http_minor = (ch as libc::c_int - 48 as libc::c_int)
                    as libc::c_ushort;
                p_state = s_res_http_end;
                current_block = 2823593449448810005;
            }
            6531417090144833949 => {
                (*parser)
                    .http_major = (ch as libc::c_int - 48 as libc::c_int)
                    as libc::c_ushort;
                p_state = s_res_http_dot;
                current_block = 2823593449448810005;
            }
            1131197912709891142 => {
                tmp___12 = (ch as libc::c_int != 69 as libc::c_int) as libc::c_int
                    as libc::c_long;
                if tmp___12 != 0 {
                    (*parser).set_http_errno(28 as libc::c_uint);
                    current_block = 8656493661476283127;
                    break;
                } else {
                    (*parser).set_type_0(0 as libc::c_uint);
                    (*parser).set_method(2 as libc::c_uint);
                    (*parser).set_index(2 as libc::c_uint);
                    p_state = s_req_method;
                }
                current_block = 2823593449448810005;
            }
            10194173116391654254 => {
                (*parser).nread = 0 as libc::c_int as uint32_t;
                if (*parser).flags() & 1 as libc::c_uint != 0 {
                    tmp___106 = 1 as libc::c_int;
                } else if (*parser).content_length > 0 as libc::c_ulong {
                    if (*parser).content_length as libc::c_ulonglong
                        != 18446744073709551615 as libc::c_ulonglong
                    {
                        tmp___106 = 1 as libc::c_int;
                    } else {
                        tmp___106 = 0 as libc::c_int;
                    }
                } else {
                    tmp___106 = 0 as libc::c_int;
                }
                hasBody = tmp___106;
                if (*parser).upgrade() != 0 {
                    if (*parser).method() == 5 as libc::c_uint {
                        current_block = 5046915721867642244;
                    } else if (*parser).flags() & 64 as libc::c_uint != 0 {
                        current_block = 5046915721867642244;
                    } else if hasBody == 0 {
                        current_block = 5046915721867642244;
                    } else {
                        current_block = 16630666138311799781;
                    }
                    match current_block {
                        16630666138311799781 => {}
                        _ => {
                            tmp___110 = http_should_keep_alive(
                                parser as *const http_parser,
                            );
                            if tmp___110 != 0 {
                                if (*parser).type_0() == 0 as libc::c_uint {
                                    tmp___108 = 18 as libc::c_int;
                                } else {
                                    tmp___108 = 4 as libc::c_int;
                                }
                                tmp___109 = tmp___108;
                            } else {
                                tmp___109 = 1 as libc::c_int;
                            }
                            p_state = tmp___109 as state;
                            if !((*parser).http_errno() as http_errno as libc::c_uint
                                == 0 as libc::c_uint)
                            {
                                __assert_fail(
                                    b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                        as *const libc::c_char,
                                    b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                                    1787 as libc::c_uint,
                                    b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            tmp___115 = ((*settings).on_message_complete).is_some()
                                as libc::c_int as libc::c_long;
                            if tmp___115 != 0 {
                                (*parser).set_state(p_state as libc::c_uint);
                                tmp___111 = (Some(
                                    ((*settings).on_message_complete)
                                        .expect("non-null function pointer"),
                                ))
                                    .expect("non-null function pointer")(parser);
                                if 0 as libc::c_int != tmp___111 {
                                    tmp___112 = 1 as libc::c_int;
                                } else {
                                    tmp___112 = 0 as libc::c_int;
                                }
                                tmp___113 = tmp___112 as libc::c_long;
                                if tmp___113 != 0 {
                                    (*parser).set_http_errno(7 as libc::c_uint);
                                }
                                p_state = (*parser).state() as state;
                                tmp___114 = ((*parser).http_errno() as http_errno
                                    as libc::c_uint != 0 as libc::c_uint) as libc::c_int
                                    as libc::c_long;
                                if tmp___114 != 0 {
                                    return (p.offset_from(data) as libc::c_long
                                        + 1 as libc::c_long) as size_t;
                                }
                            }
                            (*parser).set_state(p_state as libc::c_uint);
                            return (p.offset_from(data) as libc::c_long
                                + 1 as libc::c_long) as size_t;
                        }
                    }
                }
                if (*parser).flags() & 64 as libc::c_uint != 0 {
                    tmp___119 = http_should_keep_alive(parser as *const http_parser);
                    if tmp___119 != 0 {
                        if (*parser).type_0() == 0 as libc::c_uint {
                            tmp___117 = 18 as libc::c_int;
                        } else {
                            tmp___117 = 4 as libc::c_int;
                        }
                        tmp___118 = tmp___117;
                    } else {
                        tmp___118 = 1 as libc::c_int;
                    }
                    p_state = tmp___118 as state;
                    if !((*parser).http_errno() as http_errno as libc::c_uint
                        == 0 as libc::c_uint)
                    {
                        __assert_fail(
                            b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1793 as libc::c_uint,
                            b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    tmp___124 = ((*settings).on_message_complete).is_some()
                        as libc::c_int as libc::c_long;
                    if tmp___124 != 0 {
                        (*parser).set_state(p_state as libc::c_uint);
                        tmp___120 = (Some(
                            ((*settings).on_message_complete)
                                .expect("non-null function pointer"),
                        ))
                            .expect("non-null function pointer")(parser);
                        if 0 as libc::c_int != tmp___120 {
                            tmp___121 = 1 as libc::c_int;
                        } else {
                            tmp___121 = 0 as libc::c_int;
                        }
                        tmp___122 = tmp___121 as libc::c_long;
                        if tmp___122 != 0 {
                            (*parser).set_http_errno(7 as libc::c_uint);
                        }
                        p_state = (*parser).state() as state;
                        tmp___123 = ((*parser).http_errno() as http_errno as libc::c_uint
                            != 0 as libc::c_uint) as libc::c_int as libc::c_long;
                        if tmp___123 != 0 {
                            return (p.offset_from(data) as libc::c_long
                                + 1 as libc::c_long) as size_t;
                        }
                    }
                } else if (*parser).flags() & 1 as libc::c_uint != 0 {
                    p_state = s_chunk_size_start;
                } else if (*parser).content_length == 0 as libc::c_ulong {
                    tmp___128 = http_should_keep_alive(parser as *const http_parser);
                    if tmp___128 != 0 {
                        if (*parser).type_0() == 0 as libc::c_uint {
                            tmp___126 = 18 as libc::c_int;
                        } else {
                            tmp___126 = 4 as libc::c_int;
                        }
                        tmp___127 = tmp___126;
                    } else {
                        tmp___127 = 1 as libc::c_int;
                    }
                    p_state = tmp___127 as state;
                    if !((*parser).http_errno() as http_errno as libc::c_uint
                        == 0 as libc::c_uint)
                    {
                        __assert_fail(
                            b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1801 as libc::c_uint,
                            b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    tmp___133 = ((*settings).on_message_complete).is_some()
                        as libc::c_int as libc::c_long;
                    if tmp___133 != 0 {
                        (*parser).set_state(p_state as libc::c_uint);
                        tmp___129 = (Some(
                            ((*settings).on_message_complete)
                                .expect("non-null function pointer"),
                        ))
                            .expect("non-null function pointer")(parser);
                        if 0 as libc::c_int != tmp___129 {
                            tmp___130 = 1 as libc::c_int;
                        } else {
                            tmp___130 = 0 as libc::c_int;
                        }
                        tmp___131 = tmp___130 as libc::c_long;
                        if tmp___131 != 0 {
                            (*parser).set_http_errno(7 as libc::c_uint);
                        }
                        p_state = (*parser).state() as state;
                        tmp___132 = ((*parser).http_errno() as http_errno as libc::c_uint
                            != 0 as libc::c_uint) as libc::c_int as libc::c_long;
                        if tmp___132 != 0 {
                            return (p.offset_from(data) as libc::c_long
                                + 1 as libc::c_long) as size_t;
                        }
                    }
                } else if (*parser).content_length as libc::c_ulonglong
                        != 18446744073709551615 as libc::c_ulonglong
                    {
                    p_state = s_body_identity;
                } else {
                    tmp___143 = http_message_needs_eof(parser as *const http_parser);
                    if tmp___143 != 0 {
                        p_state = s_body_identity_eof;
                    } else {
                        tmp___137 = http_should_keep_alive(parser as *const http_parser);
                        if tmp___137 != 0 {
                            if (*parser).type_0() == 0 as libc::c_uint {
                                tmp___135 = 18 as libc::c_int;
                            } else {
                                tmp___135 = 4 as libc::c_int;
                            }
                            tmp___136 = tmp___135;
                        } else {
                            tmp___136 = 1 as libc::c_int;
                        }
                        p_state = tmp___136 as state;
                        if !((*parser).http_errno() as http_errno as libc::c_uint
                            == 0 as libc::c_uint)
                        {
                            __assert_fail(
                                b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                    as *const libc::c_char,
                                b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                                1809 as libc::c_uint,
                                b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        tmp___142 = ((*settings).on_message_complete).is_some()
                            as libc::c_int as libc::c_long;
                        if tmp___142 != 0 {
                            (*parser).set_state(p_state as libc::c_uint);
                            tmp___138 = (Some(
                                ((*settings).on_message_complete)
                                    .expect("non-null function pointer"),
                            ))
                                .expect("non-null function pointer")(parser);
                            if 0 as libc::c_int != tmp___138 {
                                tmp___139 = 1 as libc::c_int;
                            } else {
                                tmp___139 = 0 as libc::c_int;
                            }
                            tmp___140 = tmp___139 as libc::c_long;
                            if tmp___140 != 0 {
                                (*parser).set_http_errno(7 as libc::c_uint);
                            }
                            p_state = (*parser).state() as state;
                            tmp___141 = ((*parser).http_errno() as http_errno
                                as libc::c_uint != 0 as libc::c_uint) as libc::c_int
                                as libc::c_long;
                            if tmp___141 != 0 {
                                return (p.offset_from(data) as libc::c_long
                                    + 1 as libc::c_long) as size_t;
                            }
                        }
                    }
                }
                current_block = 2823593449448810005;
            }
            15287575108889763005 => {
                if header_field_mark.is_null() {
                    header_field_mark = p;
                }
                (*parser).set_index(0 as libc::c_uint);
                p_state = s_header_field;
                match c as libc::c_int {
                    99 => {
                        (*parser).set_header_state(1 as libc::c_uint);
                    }
                    112 => {
                        (*parser).set_header_state(5 as libc::c_uint);
                    }
                    116 => {
                        (*parser).set_header_state(7 as libc::c_uint);
                    }
                    117 => {
                        (*parser).set_header_state(8 as libc::c_uint);
                    }
                    _ => {
                        (*parser).set_header_state(0 as libc::c_uint);
                    }
                }
                current_block = 2823593449448810005;
            }
            16542685637124060003 => {
                if p as libc::c_ulong == data.offset(len as isize) as libc::c_ulong {
                    p = p.offset(-1);
                } else if ch as libc::c_int == 58 as libc::c_int {
                    p_state = s_header_value_discard_ws;
                    if !((*parser).http_errno() as http_errno as libc::c_uint
                        == 0 as libc::c_uint)
                    {
                        __assert_fail(
                            b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1349 as libc::c_uint,
                            b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    if !header_field_mark.is_null() {
                        tmp___72 = ((*settings).on_header_field).is_some() as libc::c_int
                            as libc::c_long;
                        if tmp___72 != 0 {
                            (*parser).set_state(p_state as libc::c_uint);
                            tmp___68 = (Some(
                                ((*settings).on_header_field)
                                    .expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                parser,
                                header_field_mark,
                                p.offset_from(header_field_mark) as libc::c_long as size_t,
                            );
                            if 0 as libc::c_int != tmp___68 {
                                tmp___69 = 1 as libc::c_int;
                            } else {
                                tmp___69 = 0 as libc::c_int;
                            }
                            tmp___70 = tmp___69 as libc::c_long;
                            if tmp___70 != 0 {
                                (*parser).set_http_errno(3 as libc::c_uint);
                            }
                            p_state = (*parser).state() as state;
                            tmp___71 = ((*parser).http_errno() as http_errno
                                as libc::c_uint != 0 as libc::c_uint) as libc::c_int
                                as libc::c_long;
                            if tmp___71 != 0 {
                                return (p.offset_from(data) as libc::c_long
                                    + 1 as libc::c_long) as size_t;
                            }
                        }
                        header_field_mark = 0 as *mut libc::c_void
                            as *const libc::c_char;
                    }
                } else {
                    (*parser).set_http_errno(24 as libc::c_uint);
                    current_block = 8656493661476283127;
                    break;
                }
                current_block = 2823593449448810005;
            }
            16815518525555115870 => {
                if ch as libc::c_int == 9 as libc::c_int {
                    current_block = 2823593449448810005;
                } else if ch as libc::c_int == 13 as libc::c_int {
                    p_state = s_header_value_discard_ws_almost_done;
                    current_block = 2823593449448810005;
                } else if ch as libc::c_int == 10 as libc::c_int {
                    p_state = s_header_value_discard_lws;
                    current_block = 2823593449448810005;
                } else {
                    current_block = 11067245223857074510;
                }
            }
            8432433212077272926 => {
                (*parser).set_header_state(h_state as libc::c_uint);
                (*parser)
                    .nread = ((*parser).nread as libc::c_long
                    + p.offset_from(start___0) as libc::c_long) as uint32_t;
                tmp___92 = ((*parser).nread > 81920 as libc::c_uint) as libc::c_int
                    as libc::c_long;
                if tmp___92 != 0 {
                    (*parser).set_http_errno(12 as libc::c_uint);
                    current_block = 8656493661476283127;
                    break;
                } else if p as libc::c_ulong
                        == data.offset(len as isize) as libc::c_ulong
                    {
                    p = p.offset(-1);
                }
                current_block = 2823593449448810005;
            }
            1824917807432110207 => {
                p_state = s_req_http_start;
                if !((*parser).http_errno() as http_errno as libc::c_uint
                    == 0 as libc::c_uint)
                {
                    __assert_fail(
                        b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                            as *const libc::c_char,
                        b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                        1064 as libc::c_uint,
                        b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                    );
                }
                if !url_mark.is_null() {
                    tmp___51 = ((*settings).on_url).is_some() as libc::c_int
                        as libc::c_long;
                    if tmp___51 != 0 {
                        (*parser).set_state(p_state as libc::c_uint);
                        tmp___47 = (Some(
                            ((*settings).on_url).expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            parser,
                            url_mark,
                            p.offset_from(url_mark) as libc::c_long as size_t,
                        );
                        if 0 as libc::c_int != tmp___47 {
                            tmp___48 = 1 as libc::c_int;
                        } else {
                            tmp___48 = 0 as libc::c_int;
                        }
                        tmp___49 = tmp___48 as libc::c_long;
                        if tmp___49 != 0 {
                            (*parser).set_http_errno(2 as libc::c_uint);
                        }
                        p_state = (*parser).state() as state;
                        tmp___50 = ((*parser).http_errno() as http_errno as libc::c_uint
                            != 0 as libc::c_uint) as libc::c_int as libc::c_long;
                        if tmp___50 != 0 {
                            return (p.offset_from(data) as libc::c_long
                                + 1 as libc::c_long) as size_t;
                        }
                    }
                    url_mark = 0 as *mut libc::c_void as *const libc::c_char;
                }
                current_block = 2823593449448810005;
            }
            17569780573092599132 => {
                if ch as libc::c_int == 10 as libc::c_int {
                    p_state = s_header_field_start;
                } else {
                    (*parser).set_http_errno(14 as libc::c_uint);
                    current_block = 8656493661476283127;
                    break;
                }
                current_block = 2823593449448810005;
            }
            11298318405609849625 => {
                (*parser)
                    .http_minor = (ch as libc::c_int - 48 as libc::c_int)
                    as libc::c_ushort;
                p_state = s_req_http_end;
                current_block = 2823593449448810005;
            }
            16257149083642269134 => {
                (*parser).content_length = unhex_val as uint64_t;
                p_state = s_chunk_size;
                current_block = 2823593449448810005;
            }
            7658707337339137753 => {
                unhex_val = unhex[ch as libc::c_uchar as usize];
                if unhex_val as libc::c_int == -(1 as libc::c_int) {
                    if ch as libc::c_int == 59 as libc::c_int {
                        p_state = s_chunk_parameters;
                    } else if ch as libc::c_int == 32 as libc::c_int {
                        p_state = s_chunk_parameters;
                    } else {
                        (*parser).set_http_errno(27 as libc::c_uint);
                        current_block = 8656493661476283127;
                        break;
                    }
                } else {
                    t___0 = (*parser).content_length;
                    t___0 = (t___0 as libc::c_ulong).wrapping_mul(16 as libc::c_ulong)
                        as uint64_t as uint64_t;
                    t___0 = (t___0 as libc::c_ulong).wrapping_add(unhex_val as uint64_t)
                        as uint64_t as uint64_t;
                    tmp___160 = ((1152921504606846974 as libc::c_ulonglong)
                        < (*parser).content_length as libc::c_ulonglong) as libc::c_int
                        as libc::c_long;
                    if tmp___160 != 0 {
                        (*parser).set_http_errno(25 as libc::c_uint);
                        current_block = 8656493661476283127;
                        break;
                    } else {
                        (*parser).content_length = t___0;
                    }
                }
                current_block = 2823593449448810005;
            }
            18403398098674784951 => {
                (*parser)
                    .http_major = (ch as libc::c_int - 48 as libc::c_int)
                    as libc::c_ushort;
                p_state = s_req_http_dot;
                current_block = 2823593449448810005;
            }
            61686342394866974 => {
                (*parser).nread = 0 as libc::c_int as uint32_t;
                if (*parser).content_length == 0 as libc::c_ulong {
                    (*parser).set_flags((*parser).flags() | 16 as libc::c_uint);
                    p_state = s_header_field_start;
                } else {
                    p_state = s_chunk_data;
                }
                if !((*parser).http_errno() as http_errno as libc::c_uint
                    == 0 as libc::c_uint)
                {
                    __assert_fail(
                        b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                            as *const libc::c_char,
                        b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                        1949 as libc::c_uint,
                        b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                    );
                }
                tmp___165 = ((*settings).on_chunk_header).is_some() as libc::c_int
                    as libc::c_long;
                if tmp___165 != 0 {
                    (*parser).set_state(p_state as libc::c_uint);
                    tmp___161 = (Some(
                        ((*settings).on_chunk_header).expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")(parser);
                    if 0 as libc::c_int != tmp___161 {
                        tmp___162 = 1 as libc::c_int;
                    } else {
                        tmp___162 = 0 as libc::c_int;
                    }
                    tmp___163 = tmp___162 as libc::c_long;
                    if tmp___163 != 0 {
                        (*parser).set_http_errno(9 as libc::c_uint);
                    }
                    p_state = (*parser).state() as state;
                    tmp___164 = ((*parser).http_errno() as http_errno as libc::c_uint
                        != 0 as libc::c_uint) as libc::c_int as libc::c_long;
                    if tmp___164 != 0 {
                        return (p.offset_from(data) as libc::c_long + 1 as libc::c_long)
                            as size_t;
                    }
                }
                current_block = 2823593449448810005;
            }
            10831039602972093699 => {
                p_state = s_chunk_data_done;
                if !((*parser).http_errno() as http_errno as libc::c_uint
                    == 0 as libc::c_uint)
                {
                    __assert_fail(
                        b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                            as *const libc::c_char,
                        b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                        1981 as libc::c_uint,
                        b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                    );
                }
                if !body_mark.is_null() {
                    tmp___171 = ((*settings).on_body).is_some() as libc::c_int
                        as libc::c_long;
                    if tmp___171 != 0 {
                        (*parser).set_state(p_state as libc::c_uint);
                        tmp___167 = (Some(
                            ((*settings).on_body).expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            parser,
                            body_mark,
                            p.offset_from(body_mark) as libc::c_long as size_t,
                        );
                        if 0 as libc::c_int != tmp___167 {
                            tmp___168 = 1 as libc::c_int;
                        } else {
                            tmp___168 = 0 as libc::c_int;
                        }
                        tmp___169 = tmp___168 as libc::c_long;
                        if tmp___169 != 0 {
                            (*parser).set_http_errno(6 as libc::c_uint);
                        }
                        p_state = (*parser).state() as state;
                        tmp___170 = ((*parser).http_errno() as http_errno as libc::c_uint
                            != 0 as libc::c_uint) as libc::c_int as libc::c_long;
                        if tmp___170 != 0 {
                            return (p.offset_from(data) as libc::c_long
                                + 1 as libc::c_long) as size_t;
                        }
                    }
                    body_mark = 0 as *mut libc::c_void as *const libc::c_char;
                }
                current_block = 2823593449448810005;
            }
            10962111400457132340 => {
                (*parser).nread = 0 as libc::c_int as uint32_t;
                p_state = s_chunk_size_start;
                if !((*parser).http_errno() as http_errno as libc::c_uint
                    == 0 as libc::c_uint)
                {
                    __assert_fail(
                        b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                            as *const libc::c_char,
                        b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                        1989 as libc::c_uint,
                        b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                    );
                }
                tmp___176 = ((*settings).on_chunk_complete).is_some() as libc::c_int
                    as libc::c_long;
                if tmp___176 != 0 {
                    (*parser).set_state(p_state as libc::c_uint);
                    tmp___172 = (Some(
                        ((*settings).on_chunk_complete)
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")(parser);
                    if 0 as libc::c_int != tmp___172 {
                        tmp___173 = 1 as libc::c_int;
                    } else {
                        tmp___173 = 0 as libc::c_int;
                    }
                    tmp___174 = tmp___173 as libc::c_long;
                    if tmp___174 != 0 {
                        (*parser).set_http_errno(10 as libc::c_uint);
                    }
                    p_state = (*parser).state() as state;
                    tmp___175 = ((*parser).http_errno() as http_errno as libc::c_uint
                        != 0 as libc::c_uint) as libc::c_int as libc::c_long;
                    if tmp___175 != 0 {
                        return (p.offset_from(data) as libc::c_long + 1 as libc::c_long)
                            as size_t;
                    }
                }
                current_block = 2823593449448810005;
            }
            12652763302434162366 => {
                (*parser).set_http_errno(23 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            12520299732166378225 => {
                (*parser).set_http_errno(27 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            10545230836686041720 => {
                p_state = s_chunk_size_almost_done;
                current_block = 2823593449448810005;
            }
            13878384739916089420 => {
                (*parser).set_http_errno(15 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            17643269856826482833 => {
                (*parser).set_http_errno(30 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            1547916710919041324 => {
                (*parser).set_http_errno(30 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            18304633536805257724 => {
                (*parser).set_http_errno(24 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            1317541187880551388 => {
                (*parser).set_http_errno(30 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            6186816898867308296 => {
                (*parser).set_http_errno(13 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            671854326874165838 => {
                (*parser).set_http_errno(28 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            14723351692138084861 => {
                p_state = s_res_HT;
                current_block = 2823593449448810005;
            }
            15275873808073048539 => {
                p_state = s_res_HTT;
                current_block = 2823593449448810005;
            }
            10029375464402185584 => {
                p_state = s_res_HTTP;
                current_block = 2823593449448810005;
            }
            6936584767197543976 => {
                p_state = s_res_http_major;
                current_block = 2823593449448810005;
            }
            9602112577253180622 => {
                p_state = s_res_http_minor;
                current_block = 2823593449448810005;
            }
            6837155546206671799 => {
                p_state = s_res_first_status_code;
                current_block = 2823593449448810005;
            }
            1169396748315396699 => {
                p_state = s_header_field_start;
                current_block = 2823593449448810005;
            }
            6162494029290681219 => {
                (*parser).set_http_errno(28 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            6408059329745668080 => {
                p_state = s_req_http_HT;
                current_block = 2823593449448810005;
            }
            9448659001743590153 => {
                p_state = s_req_http_HTT;
                current_block = 2823593449448810005;
            }
            8924847599540368544 => {
                p_state = s_req_http_HTTP;
                current_block = 2823593449448810005;
            }
            13052205784774183146 => {
                p_state = s_req_http_major;
                current_block = 2823593449448810005;
            }
            15872201074975604606 => {
                p_state = s_req_http_minor;
                current_block = 2823593449448810005;
            }
            11721037647720733806 => {
                p_state = s_header_field_start;
                current_block = 2823593449448810005;
            }
            6702610100809040202 => {
                p_state = s_header_value_lws;
                current_block = 2823593449448810005;
            }
            10114390662193834916 => {
                p_state = s_header_value_discard_lws;
                current_block = 2823593449448810005;
            }
            13843135405533155496 => {
                (*parser).set_http_errno(30 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            17335268068337147299 => {
                (*parser).set_http_errno(30 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            17464762320025024339 => {
                p_state = s_chunk_size_almost_done;
                current_block = 2823593449448810005;
            }
            7256998052328658819 => {
                (*parser).set_type_0(1 as libc::c_uint);
                p_state = s_res_HT;
                current_block = 2823593449448810005;
            }
            1626147699861920882 => {
                p_state = s_res_H;
                current_block = 1948584361000433526;
            }
            5583171084533732733 => {
                (*parser).set_http_errno(30 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            7409542616634851420 => {
                (*parser).set_http_errno(30 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            12843036121559584357 => {
                (*parser).set_http_errno(30 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            15455430299222214173 => {
                (*parser).set_http_errno(30 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            4954337906651494757 => {
                (*parser).set_http_errno(14 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            16749893710890171700 => {
                (*parser).set_http_errno(14 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            15306500312484564201 => {
                (*parser).set_http_errno(14 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            17975683517549114783 => {
                (*parser).set_http_errno(14 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            17936412886206549082 => {
                (*parser).set_http_errno(30 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            8889999340123292593 => {
                (*parser).set_http_errno(16 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            15710006729984671399 => {
                (*parser).set_http_errno(17 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            3239658638043768328 => {
                p_state = s_req_http_H;
                current_block = 2823593449448810005;
            }
            17663900762289766202 => {
                (*parser).set_http_errno(30 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            12184831490747941883 => {
                (*parser).set_http_errno(30 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            9495722216958262053 => {
                (*parser).set_http_errno(30 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            4240322626966923948 => {
                (*parser).set_http_errno(30 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            15340942182342897967 => {
                (*parser).set_http_errno(14 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            13198017010642886032 => {
                (*parser).set_http_errno(14 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            18300168379702596888 => {
                (*parser).set_http_errno(14 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            7923635230025172457 => {
                p_state = s_req_line_almost_done;
                current_block = 2823593449448810005;
            }
            12561890576527106478 => {
                (*parser).set_http_errno(23 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            17329294740332298082 => {
                (*parser).set_http_errno(12 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            9191143013813461585 => {
                (*parser).set_http_errno(24 as libc::c_uint);
                current_block = 8656493661476283127;
                break;
            }
            _ => {}
        }
        match current_block {
            1840006672846692558 => {
                if !(ch as libc::c_int == 32 as libc::c_int) {
                    (*parser).set_http_errno(15 as libc::c_uint);
                    current_block = 8656493661476283127;
                    break;
                }
            }
            11067245223857074510 => {
                if header_value_mark.is_null() {
                    header_value_mark = p;
                }
                p_state = s_header_value;
                (*parser).set_index(0 as libc::c_uint);
                c = (ch as libc::c_int | 32 as libc::c_int) as libc::c_uchar
                    as libc::c_char;
                match (*parser).header_state() as libc::c_int {
                    12 => {
                        current_block = 5086504867027940763;
                        match current_block {
                            15981105494017213667 => {
                                (*parser).set_header_state(0 as libc::c_uint);
                            }
                            12026824027391378951 => {
                                if ch as libc::c_int >= 48 as libc::c_int {
                                    if ch as libc::c_int <= 57 as libc::c_int {
                                        tmp___73 = 0 as libc::c_int;
                                    } else {
                                        tmp___73 = 1 as libc::c_int;
                                    }
                                } else {
                                    tmp___73 = 1 as libc::c_int;
                                }
                                tmp___74 = tmp___73 as libc::c_long;
                                if tmp___74 != 0 {
                                    (*parser).set_http_errno(25 as libc::c_uint);
                                    current_block = 8656493661476283127;
                                    break;
                                } else if (*parser).flags() & 128 as libc::c_uint != 0 {
                                    (*parser).set_http_errno(26 as libc::c_uint);
                                    current_block = 8656493661476283127;
                                    break;
                                } else {
                                    (*parser)
                                        .set_flags((*parser).flags() | 128 as libc::c_uint);
                                    (*parser)
                                        .content_length = (ch as libc::c_int - 48 as libc::c_int)
                                        as uint64_t;
                                }
                            }
                            9489126378800744729 => {
                                if 99 as libc::c_int == c as libc::c_int {
                                    (*parser).set_header_state(13 as libc::c_uint);
                                } else {
                                    (*parser).set_header_state(0 as libc::c_uint);
                                }
                            }
                            8558598843100317269 => {
                                if c as libc::c_int == 107 as libc::c_int {
                                    (*parser).set_header_state(15 as libc::c_uint);
                                } else if c as libc::c_int == 99 as libc::c_int {
                                    (*parser).set_header_state(16 as libc::c_uint);
                                } else if c as libc::c_int == 117 as libc::c_int {
                                    (*parser).set_header_state(17 as libc::c_uint);
                                } else {
                                    (*parser).set_header_state(18 as libc::c_uint);
                                }
                            }
                            _ => {
                                (*parser).set_flags((*parser).flags() | 32 as libc::c_uint);
                                (*parser).set_header_state(0 as libc::c_uint);
                            }
                        }
                    }
                    11 => {
                        current_block = 9489126378800744729;
                        match current_block {
                            15981105494017213667 => {
                                (*parser).set_header_state(0 as libc::c_uint);
                            }
                            12026824027391378951 => {
                                if ch as libc::c_int >= 48 as libc::c_int {
                                    if ch as libc::c_int <= 57 as libc::c_int {
                                        tmp___73 = 0 as libc::c_int;
                                    } else {
                                        tmp___73 = 1 as libc::c_int;
                                    }
                                } else {
                                    tmp___73 = 1 as libc::c_int;
                                }
                                tmp___74 = tmp___73 as libc::c_long;
                                if tmp___74 != 0 {
                                    (*parser).set_http_errno(25 as libc::c_uint);
                                    current_block = 8656493661476283127;
                                    break;
                                } else if (*parser).flags() & 128 as libc::c_uint != 0 {
                                    (*parser).set_http_errno(26 as libc::c_uint);
                                    current_block = 8656493661476283127;
                                    break;
                                } else {
                                    (*parser)
                                        .set_flags((*parser).flags() | 128 as libc::c_uint);
                                    (*parser)
                                        .content_length = (ch as libc::c_int - 48 as libc::c_int)
                                        as uint64_t;
                                }
                            }
                            9489126378800744729 => {
                                if 99 as libc::c_int == c as libc::c_int {
                                    (*parser).set_header_state(13 as libc::c_uint);
                                } else {
                                    (*parser).set_header_state(0 as libc::c_uint);
                                }
                            }
                            8558598843100317269 => {
                                if c as libc::c_int == 107 as libc::c_int {
                                    (*parser).set_header_state(15 as libc::c_uint);
                                } else if c as libc::c_int == 99 as libc::c_int {
                                    (*parser).set_header_state(16 as libc::c_uint);
                                } else if c as libc::c_int == 117 as libc::c_int {
                                    (*parser).set_header_state(17 as libc::c_uint);
                                } else {
                                    (*parser).set_header_state(18 as libc::c_uint);
                                }
                            }
                            _ => {
                                (*parser).set_flags((*parser).flags() | 32 as libc::c_uint);
                                (*parser).set_header_state(0 as libc::c_uint);
                            }
                        }
                    }
                    10 => {
                        current_block = 12026824027391378951;
                        match current_block {
                            15981105494017213667 => {
                                (*parser).set_header_state(0 as libc::c_uint);
                            }
                            12026824027391378951 => {
                                if ch as libc::c_int >= 48 as libc::c_int {
                                    if ch as libc::c_int <= 57 as libc::c_int {
                                        tmp___73 = 0 as libc::c_int;
                                    } else {
                                        tmp___73 = 1 as libc::c_int;
                                    }
                                } else {
                                    tmp___73 = 1 as libc::c_int;
                                }
                                tmp___74 = tmp___73 as libc::c_long;
                                if tmp___74 != 0 {
                                    (*parser).set_http_errno(25 as libc::c_uint);
                                    current_block = 8656493661476283127;
                                    break;
                                } else if (*parser).flags() & 128 as libc::c_uint != 0 {
                                    (*parser).set_http_errno(26 as libc::c_uint);
                                    current_block = 8656493661476283127;
                                    break;
                                } else {
                                    (*parser)
                                        .set_flags((*parser).flags() | 128 as libc::c_uint);
                                    (*parser)
                                        .content_length = (ch as libc::c_int - 48 as libc::c_int)
                                        as uint64_t;
                                }
                            }
                            9489126378800744729 => {
                                if 99 as libc::c_int == c as libc::c_int {
                                    (*parser).set_header_state(13 as libc::c_uint);
                                } else {
                                    (*parser).set_header_state(0 as libc::c_uint);
                                }
                            }
                            8558598843100317269 => {
                                if c as libc::c_int == 107 as libc::c_int {
                                    (*parser).set_header_state(15 as libc::c_uint);
                                } else if c as libc::c_int == 99 as libc::c_int {
                                    (*parser).set_header_state(16 as libc::c_uint);
                                } else if c as libc::c_int == 117 as libc::c_int {
                                    (*parser).set_header_state(17 as libc::c_uint);
                                } else {
                                    (*parser).set_header_state(18 as libc::c_uint);
                                }
                            }
                            _ => {
                                (*parser).set_flags((*parser).flags() | 32 as libc::c_uint);
                                (*parser).set_header_state(0 as libc::c_uint);
                            }
                        }
                    }
                    9 => {
                        current_block = 8558598843100317269;
                        match current_block {
                            15981105494017213667 => {
                                (*parser).set_header_state(0 as libc::c_uint);
                            }
                            12026824027391378951 => {
                                if ch as libc::c_int >= 48 as libc::c_int {
                                    if ch as libc::c_int <= 57 as libc::c_int {
                                        tmp___73 = 0 as libc::c_int;
                                    } else {
                                        tmp___73 = 1 as libc::c_int;
                                    }
                                } else {
                                    tmp___73 = 1 as libc::c_int;
                                }
                                tmp___74 = tmp___73 as libc::c_long;
                                if tmp___74 != 0 {
                                    (*parser).set_http_errno(25 as libc::c_uint);
                                    current_block = 8656493661476283127;
                                    break;
                                } else if (*parser).flags() & 128 as libc::c_uint != 0 {
                                    (*parser).set_http_errno(26 as libc::c_uint);
                                    current_block = 8656493661476283127;
                                    break;
                                } else {
                                    (*parser)
                                        .set_flags((*parser).flags() | 128 as libc::c_uint);
                                    (*parser)
                                        .content_length = (ch as libc::c_int - 48 as libc::c_int)
                                        as uint64_t;
                                }
                            }
                            9489126378800744729 => {
                                if 99 as libc::c_int == c as libc::c_int {
                                    (*parser).set_header_state(13 as libc::c_uint);
                                } else {
                                    (*parser).set_header_state(0 as libc::c_uint);
                                }
                            }
                            8558598843100317269 => {
                                if c as libc::c_int == 107 as libc::c_int {
                                    (*parser).set_header_state(15 as libc::c_uint);
                                } else if c as libc::c_int == 99 as libc::c_int {
                                    (*parser).set_header_state(16 as libc::c_uint);
                                } else if c as libc::c_int == 117 as libc::c_int {
                                    (*parser).set_header_state(17 as libc::c_uint);
                                } else {
                                    (*parser).set_header_state(18 as libc::c_uint);
                                }
                            }
                            _ => {
                                (*parser).set_flags((*parser).flags() | 32 as libc::c_uint);
                                (*parser).set_header_state(0 as libc::c_uint);
                            }
                        }
                    }
                    14 => {}
                    _ => {
                        current_block = 15981105494017213667;
                        match current_block {
                            15981105494017213667 => {
                                (*parser).set_header_state(0 as libc::c_uint);
                            }
                            12026824027391378951 => {
                                if ch as libc::c_int >= 48 as libc::c_int {
                                    if ch as libc::c_int <= 57 as libc::c_int {
                                        tmp___73 = 0 as libc::c_int;
                                    } else {
                                        tmp___73 = 1 as libc::c_int;
                                    }
                                } else {
                                    tmp___73 = 1 as libc::c_int;
                                }
                                tmp___74 = tmp___73 as libc::c_long;
                                if tmp___74 != 0 {
                                    (*parser).set_http_errno(25 as libc::c_uint);
                                    current_block = 8656493661476283127;
                                    break;
                                } else if (*parser).flags() & 128 as libc::c_uint != 0 {
                                    (*parser).set_http_errno(26 as libc::c_uint);
                                    current_block = 8656493661476283127;
                                    break;
                                } else {
                                    (*parser)
                                        .set_flags((*parser).flags() | 128 as libc::c_uint);
                                    (*parser)
                                        .content_length = (ch as libc::c_int - 48 as libc::c_int)
                                        as uint64_t;
                                }
                            }
                            9489126378800744729 => {
                                if 99 as libc::c_int == c as libc::c_int {
                                    (*parser).set_header_state(13 as libc::c_uint);
                                } else {
                                    (*parser).set_header_state(0 as libc::c_uint);
                                }
                            }
                            8558598843100317269 => {
                                if c as libc::c_int == 107 as libc::c_int {
                                    (*parser).set_header_state(15 as libc::c_uint);
                                } else if c as libc::c_int == 99 as libc::c_int {
                                    (*parser).set_header_state(16 as libc::c_uint);
                                } else if c as libc::c_int == 117 as libc::c_int {
                                    (*parser).set_header_state(17 as libc::c_uint);
                                } else {
                                    (*parser).set_header_state(18 as libc::c_uint);
                                }
                            }
                            _ => {
                                (*parser).set_flags((*parser).flags() | 32 as libc::c_uint);
                                (*parser).set_header_state(0 as libc::c_uint);
                            }
                        }
                    }
                }
            }
            1948584361000433526 => {
                if !((*parser).http_errno() as http_errno as libc::c_uint
                    == 0 as libc::c_uint)
                {
                    __assert_fail(
                        b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                            as *const libc::c_char,
                        b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                        774 as libc::c_uint,
                        b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                    );
                }
                tmp___17 = ((*settings).on_message_begin).is_some() as libc::c_int
                    as libc::c_long;
                if tmp___17 != 0 {
                    (*parser).set_state(p_state as libc::c_uint);
                    tmp___13 = (Some(
                        ((*settings).on_message_begin)
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")(parser);
                    if 0 as libc::c_int != tmp___13 {
                        tmp___14 = 1 as libc::c_int;
                    } else {
                        tmp___14 = 0 as libc::c_int;
                    }
                    tmp___15 = tmp___14 as libc::c_long;
                    if tmp___15 != 0 {
                        (*parser).set_http_errno(1 as libc::c_uint);
                    }
                    p_state = (*parser).state() as state;
                    tmp___16 = ((*parser).http_errno() as http_errno as libc::c_uint
                        != 0 as libc::c_uint) as libc::c_int as libc::c_long;
                    if tmp___16 != 0 {
                        return (p.offset_from(data) as libc::c_long + 1 as libc::c_long)
                            as size_t;
                    }
                }
            }
            _ => {}
        }
        p = p.offset(1);
    }
    match current_block {
        8656493661476283127 => {
            if (*parser).http_errno() as http_errno as libc::c_uint == 0 as libc::c_uint
            {
                (*parser).set_http_errno(32 as libc::c_uint);
            }
            (*parser).set_state(p_state as libc::c_uint);
            return p.offset_from(data) as libc::c_long as size_t;
        }
        _ => {
            if !header_field_mark.is_null() {
                tmp___177 = 1 as libc::c_int;
            } else {
                tmp___177 = 0 as libc::c_int;
            }
            if !header_value_mark.is_null() {
                tmp___178 = 1 as libc::c_int;
            } else {
                tmp___178 = 0 as libc::c_int;
            }
            if !url_mark.is_null() {
                tmp___179 = 1 as libc::c_int;
            } else {
                tmp___179 = 0 as libc::c_int;
            }
            if !body_mark.is_null() {
                tmp___180 = 1 as libc::c_int;
            } else {
                tmp___180 = 0 as libc::c_int;
            }
            if !status_mark.is_null() {
                tmp___181 = 1 as libc::c_int;
            } else {
                tmp___181 = 0 as libc::c_int;
            }
            if !(tmp___177 + tmp___178 + tmp___179 + tmp___180 + tmp___181
                <= 1 as libc::c_int)
            {
                __assert_fail(
                    b"((header_field_mark ? 1 : 0) + (header_value_mark ? 1 : 0) + (url_mark ? 1 : 0) + (body_mark ? 1 : 0) + (status_mark ? 1 : 0)) <= 1\0"
                        as *const u8 as *const libc::c_char,
                    b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                    2013 as libc::c_uint,
                    b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                );
            }
            if !((*parser).http_errno() as http_errno as libc::c_uint
                == 0 as libc::c_uint)
            {
                __assert_fail(
                    b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                        as *const libc::c_char,
                    b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                    2015 as libc::c_uint,
                    b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                );
            }
            if !header_field_mark.is_null() {
                tmp___186 = ((*settings).on_header_field).is_some() as libc::c_int
                    as libc::c_long;
                if tmp___186 != 0 {
                    (*parser).set_state(p_state as libc::c_uint);
                    tmp___182 = (Some(
                        ((*settings).on_header_field).expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        parser,
                        header_field_mark,
                        p.offset_from(header_field_mark) as libc::c_long as size_t,
                    );
                    if 0 as libc::c_int != tmp___182 {
                        tmp___183 = 1 as libc::c_int;
                    } else {
                        tmp___183 = 0 as libc::c_int;
                    }
                    tmp___184 = tmp___183 as libc::c_long;
                    if tmp___184 != 0 {
                        (*parser).set_http_errno(3 as libc::c_uint);
                    }
                    p_state = (*parser).state() as state;
                    tmp___185 = ((*parser).http_errno() as http_errno as libc::c_uint
                        != 0 as libc::c_uint) as libc::c_int as libc::c_long;
                    if tmp___185 != 0 {
                        return p.offset_from(data) as libc::c_long as size_t;
                    }
                }
                header_field_mark = 0 as *mut libc::c_void as *const libc::c_char;
            }
            if !((*parser).http_errno() as http_errno as libc::c_uint
                == 0 as libc::c_uint)
            {
                __assert_fail(
                    b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                        as *const libc::c_char,
                    b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                    2016 as libc::c_uint,
                    b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                );
            }
            if !header_value_mark.is_null() {
                tmp___191 = ((*settings).on_header_value).is_some() as libc::c_int
                    as libc::c_long;
                if tmp___191 != 0 {
                    (*parser).set_state(p_state as libc::c_uint);
                    tmp___187 = (Some(
                        ((*settings).on_header_value).expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        parser,
                        header_value_mark,
                        p.offset_from(header_value_mark) as libc::c_long as size_t,
                    );
                    if 0 as libc::c_int != tmp___187 {
                        tmp___188 = 1 as libc::c_int;
                    } else {
                        tmp___188 = 0 as libc::c_int;
                    }
                    tmp___189 = tmp___188 as libc::c_long;
                    if tmp___189 != 0 {
                        (*parser).set_http_errno(4 as libc::c_uint);
                    }
                    p_state = (*parser).state() as state;
                    tmp___190 = ((*parser).http_errno() as http_errno as libc::c_uint
                        != 0 as libc::c_uint) as libc::c_int as libc::c_long;
                    if tmp___190 != 0 {
                        return p.offset_from(data) as libc::c_long as size_t;
                    }
                }
                header_value_mark = 0 as *mut libc::c_void as *const libc::c_char;
            }
            if !((*parser).http_errno() as http_errno as libc::c_uint
                == 0 as libc::c_uint)
            {
                __assert_fail(
                    b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                        as *const libc::c_char,
                    b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                    2017 as libc::c_uint,
                    b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                );
            }
            if !url_mark.is_null() {
                tmp___196 = ((*settings).on_url).is_some() as libc::c_int
                    as libc::c_long;
                if tmp___196 != 0 {
                    (*parser).set_state(p_state as libc::c_uint);
                    tmp___192 = (Some(
                        ((*settings).on_url).expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        parser,
                        url_mark,
                        p.offset_from(url_mark) as libc::c_long as size_t,
                    );
                    if 0 as libc::c_int != tmp___192 {
                        tmp___193 = 1 as libc::c_int;
                    } else {
                        tmp___193 = 0 as libc::c_int;
                    }
                    tmp___194 = tmp___193 as libc::c_long;
                    if tmp___194 != 0 {
                        (*parser).set_http_errno(2 as libc::c_uint);
                    }
                    p_state = (*parser).state() as state;
                    tmp___195 = ((*parser).http_errno() as http_errno as libc::c_uint
                        != 0 as libc::c_uint) as libc::c_int as libc::c_long;
                    if tmp___195 != 0 {
                        return p.offset_from(data) as libc::c_long as size_t;
                    }
                }
                url_mark = 0 as *mut libc::c_void as *const libc::c_char;
            }
            if !((*parser).http_errno() as http_errno as libc::c_uint
                == 0 as libc::c_uint)
            {
                __assert_fail(
                    b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                        as *const libc::c_char,
                    b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                    2018 as libc::c_uint,
                    b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                );
            }
            if !body_mark.is_null() {
                tmp___201 = ((*settings).on_body).is_some() as libc::c_int
                    as libc::c_long;
                if tmp___201 != 0 {
                    (*parser).set_state(p_state as libc::c_uint);
                    tmp___197 = (Some(
                        ((*settings).on_body).expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        parser,
                        body_mark,
                        p.offset_from(body_mark) as libc::c_long as size_t,
                    );
                    if 0 as libc::c_int != tmp___197 {
                        tmp___198 = 1 as libc::c_int;
                    } else {
                        tmp___198 = 0 as libc::c_int;
                    }
                    tmp___199 = tmp___198 as libc::c_long;
                    if tmp___199 != 0 {
                        (*parser).set_http_errno(6 as libc::c_uint);
                    }
                    p_state = (*parser).state() as state;
                    tmp___200 = ((*parser).http_errno() as http_errno as libc::c_uint
                        != 0 as libc::c_uint) as libc::c_int as libc::c_long;
                    if tmp___200 != 0 {
                        return p.offset_from(data) as libc::c_long as size_t;
                    }
                }
                body_mark = 0 as *mut libc::c_void as *const libc::c_char;
            }
            if !((*parser).http_errno() as http_errno as libc::c_uint
                == 0 as libc::c_uint)
            {
                __assert_fail(
                    b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                        as *const libc::c_char,
                    b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                    2019 as libc::c_uint,
                    b"http_parser_execute\0" as *const u8 as *const libc::c_char,
                );
            }
            if !status_mark.is_null() {
                tmp___206 = ((*settings).on_status).is_some() as libc::c_int
                    as libc::c_long;
                if tmp___206 != 0 {
                    (*parser).set_state(p_state as libc::c_uint);
                    tmp___202 = (Some(
                        ((*settings).on_status).expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        parser,
                        status_mark,
                        p.offset_from(status_mark) as libc::c_long as size_t,
                    );
                    if 0 as libc::c_int != tmp___202 {
                        tmp___203 = 1 as libc::c_int;
                    } else {
                        tmp___203 = 0 as libc::c_int;
                    }
                    tmp___204 = tmp___203 as libc::c_long;
                    if tmp___204 != 0 {
                        (*parser).set_http_errno(8 as libc::c_uint);
                    }
                    p_state = (*parser).state() as state;
                    tmp___205 = ((*parser).http_errno() as http_errno as libc::c_uint
                        != 0 as libc::c_uint) as libc::c_int as libc::c_long;
                    if tmp___205 != 0 {
                        return p.offset_from(data) as libc::c_long as size_t;
                    }
                }
                status_mark = 0 as *mut libc::c_void as *const libc::c_char;
            }
            (*parser).set_state(p_state as libc::c_uint);
            return len;
        }
    };
}
pub unsafe extern "C" fn http_message_needs_eof(
    mut parser: *const http_parser,
) -> libc::c_int {
    if (*parser).type_0() == 0 as libc::c_uint {
        return 0 as libc::c_int;
    }
    if (*parser).status_code().wrapping_div(100 as libc::c_uint) == 1 as libc::c_uint {
        return 0 as libc::c_int
    } else {
        if (*parser).status_code() == 204 as libc::c_uint {
            return 0 as libc::c_int
        } else {
            if (*parser).status_code() == 304 as libc::c_uint {
                return 0 as libc::c_int
            } else {
                if (*parser).flags() & 64 as libc::c_uint != 0 {
                    return 0 as libc::c_int;
                }
            }
        }
    }
    if (*parser).flags() & 1 as libc::c_uint != 0 {
        return 0 as libc::c_int
    } else {
        if (*parser).content_length as libc::c_ulonglong
            != 18446744073709551615 as libc::c_ulonglong
        {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn http_should_keep_alive(
    mut parser: *const http_parser,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut current_block_6: u64;
    if (*parser).http_major as libc::c_int > 0 as libc::c_int {
        if (*parser).http_minor as libc::c_int > 0 as libc::c_int {
            if (*parser).flags() & 4 as libc::c_uint != 0 {
                return 0 as libc::c_int;
            }
            current_block_6 = 5720623009719927633;
        } else {
            current_block_6 = 2900286526258047768;
        }
    } else {
        current_block_6 = 2900286526258047768;
    }
    match current_block_6 {
        2900286526258047768 => {
            if (*parser).flags() & 2 as libc::c_uint == 0 {
                return 0 as libc::c_int;
            }
        }
        _ => {}
    }
    tmp = http_message_needs_eof(parser);
    if tmp != 0 {
        tmp___0 = 0 as libc::c_int;
    } else {
        tmp___0 = 1 as libc::c_int;
    }
    return tmp___0;
}
pub unsafe extern "C" fn http_method_str(mut m: http_method) -> *const libc::c_char {
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    if (m as libc::c_uint as libc::c_ulong)
        < (::std::mem::size_of::<[*const libc::c_char; 33]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        tmp = method_strings[m as usize];
    } else {
        tmp = b"<unknown>\0" as *const u8 as *const libc::c_char;
    }
    return tmp;
}
pub unsafe extern "C" fn http_parser_init(
    mut parser: *mut http_parser,
    mut t: http_parser_type,
) {
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp: libc::c_int = 0;
    data = (*parser).data;
    memset(
        parser as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<http_parser>() as libc::c_ulong,
    );
    (*parser).data = data;
    (*parser).set_type_0(t as libc::c_uint);
    if t as libc::c_uint == 0 as libc::c_uint {
        (*parser).set_state(18 as libc::c_uint);
    } else {
        if t as libc::c_uint == 1 as libc::c_uint {
            tmp = 4 as libc::c_int;
        } else {
            tmp = 2 as libc::c_int;
        }
        (*parser).set_state(tmp as libc::c_uint);
    }
    (*parser).set_http_errno(0 as libc::c_uint);
}
pub unsafe extern "C" fn http_parser_settings_init(
    mut settings: *mut http_parser_settings,
) {
    memset(
        settings as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<http_parser_settings>() as libc::c_ulong,
    );
}
pub unsafe extern "C" fn http_errno_name(mut err: http_errno) -> *const libc::c_char {
    if !((err as size_t)
        < (::std::mem::size_of::<[__anonstruct_http_strerror_tab_527861670; 33]>()
            as libc::c_ulong)
            .wrapping_div(
                ::std::mem::size_of::<__anonstruct_http_strerror_tab_527861670>()
                    as libc::c_ulong,
            ))
    {
        __assert_fail(
            b"((size_t) err) < ARRAY_SIZE(http_strerror_tab)\0" as *const u8
                as *const libc::c_char,
            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
            2101 as libc::c_uint,
            b"http_errno_name\0" as *const u8 as *const libc::c_char,
        );
    }
    return http_strerror_tab[err as usize].name;
}
pub unsafe extern "C" fn http_errno_description(
    mut err: http_errno,
) -> *const libc::c_char {
    if !((err as size_t)
        < (::std::mem::size_of::<[__anonstruct_http_strerror_tab_527861670; 33]>()
            as libc::c_ulong)
            .wrapping_div(
                ::std::mem::size_of::<__anonstruct_http_strerror_tab_527861670>()
                    as libc::c_ulong,
            ))
    {
        __assert_fail(
            b"((size_t) err) < ARRAY_SIZE(http_strerror_tab)\0" as *const u8
                as *const libc::c_char,
            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
            2107 as libc::c_uint,
            b"http_errno_description\0" as *const u8 as *const libc::c_char,
        );
    }
    return http_strerror_tab[err as usize].description;
}
unsafe extern "C" fn http_parse_host_char(
    mut s: http_host_state,
    ch: libc::c_char,
) -> http_host_state {
    let mut current_block_138: u64;
    match s as libc::c_uint {
        2 | 3 => {
            if ch as libc::c_int == 64 as libc::c_int {
                return s_http_host_start;
            }
            if (ch as libc::c_int | 32 as libc::c_int) as libc::c_uchar as libc::c_int
                >= 97 as libc::c_int
            {
                if (ch as libc::c_int | 32 as libc::c_int) as libc::c_uchar
                    as libc::c_int <= 122 as libc::c_int
                {
                    return s_http_userinfo;
                }
            }
            if ch as libc::c_int >= 48 as libc::c_int {
                if ch as libc::c_int <= 57 as libc::c_int {
                    return s_http_userinfo;
                }
            }
            if ch as libc::c_int == 45 as libc::c_int {
                return s_http_userinfo
            } else {
                if ch as libc::c_int == 95 as libc::c_int {
                    return s_http_userinfo
                } else {
                    if ch as libc::c_int == 46 as libc::c_int {
                        return s_http_userinfo
                    } else {
                        if ch as libc::c_int == 33 as libc::c_int {
                            return s_http_userinfo
                        } else {
                            if ch as libc::c_int == 126 as libc::c_int {
                                return s_http_userinfo
                            } else {
                                if ch as libc::c_int == 42 as libc::c_int {
                                    return s_http_userinfo
                                } else {
                                    if ch as libc::c_int == 39 as libc::c_int {
                                        return s_http_userinfo
                                    } else {
                                        if ch as libc::c_int == 40 as libc::c_int {
                                            return s_http_userinfo
                                        } else {
                                            if ch as libc::c_int == 41 as libc::c_int {
                                                return s_http_userinfo
                                            } else {
                                                if ch as libc::c_int == 37 as libc::c_int {
                                                    return s_http_userinfo
                                                } else {
                                                    if ch as libc::c_int == 59 as libc::c_int {
                                                        return s_http_userinfo
                                                    } else {
                                                        if ch as libc::c_int == 58 as libc::c_int {
                                                            return s_http_userinfo
                                                        } else {
                                                            if ch as libc::c_int == 38 as libc::c_int {
                                                                return s_http_userinfo
                                                            } else {
                                                                if ch as libc::c_int == 61 as libc::c_int {
                                                                    return s_http_userinfo
                                                                } else {
                                                                    if ch as libc::c_int == 43 as libc::c_int {
                                                                        return s_http_userinfo
                                                                    } else {
                                                                        if ch as libc::c_int == 36 as libc::c_int {
                                                                            return s_http_userinfo
                                                                        } else {
                                                                            if ch as libc::c_int == 44 as libc::c_int {
                                                                                return s_http_userinfo;
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
                }
            }
            current_block_138 = 7370318721998929769;
        }
        4 => {
            if ch as libc::c_int == 91 as libc::c_int {
                return s_http_host_v6_start;
            }
            if (ch as libc::c_int | 32 as libc::c_int) as libc::c_uchar as libc::c_int
                >= 97 as libc::c_int
            {
                if (ch as libc::c_int | 32 as libc::c_int) as libc::c_uchar
                    as libc::c_int <= 122 as libc::c_int
                {
                    return s_http_host;
                }
            }
            if ch as libc::c_int >= 48 as libc::c_int {
                if ch as libc::c_int <= 57 as libc::c_int {
                    return s_http_host;
                }
            }
            if ch as libc::c_int == 46 as libc::c_int {
                return s_http_host
            } else {
                if ch as libc::c_int == 45 as libc::c_int {
                    return s_http_host;
                }
            }
            current_block_138 = 7370318721998929769;
        }
        6 => {
            if (ch as libc::c_int | 32 as libc::c_int) as libc::c_uchar as libc::c_int
                >= 97 as libc::c_int
            {
                if (ch as libc::c_int | 32 as libc::c_int) as libc::c_uchar
                    as libc::c_int <= 122 as libc::c_int
                {
                    return s_http_host;
                }
            }
            if ch as libc::c_int >= 48 as libc::c_int {
                if ch as libc::c_int <= 57 as libc::c_int {
                    return s_http_host;
                }
            }
            if ch as libc::c_int == 46 as libc::c_int {
                return s_http_host
            } else {
                if ch as libc::c_int == 45 as libc::c_int {
                    return s_http_host;
                }
            }
            current_block_138 = 17152226948168769005;
        }
        8 => {
            current_block_138 = 17152226948168769005;
        }
        7 => {
            if ch as libc::c_int == 93 as libc::c_int {
                return s_http_host_v6_end;
            }
            current_block_138 = 11281570922350050651;
        }
        5 => {
            current_block_138 = 11281570922350050651;
        }
        10 => {
            if ch as libc::c_int == 93 as libc::c_int {
                return s_http_host_v6_end;
            }
            current_block_138 = 9364233119027335301;
        }
        9 => {
            current_block_138 = 9364233119027335301;
        }
        11 | 12 => {
            if ch as libc::c_int >= 48 as libc::c_int {
                if ch as libc::c_int <= 57 as libc::c_int {
                    return s_http_host_port;
                }
            }
            current_block_138 = 7370318721998929769;
        }
        _ => {
            current_block_138 = 7370318721998929769;
        }
    }
    match current_block_138 {
        11281570922350050651 => {
            if ch as libc::c_int >= 48 as libc::c_int {
                if ch as libc::c_int <= 57 as libc::c_int {
                    return s_http_host_v6;
                }
            }
            if (ch as libc::c_int | 32 as libc::c_int) as libc::c_uchar as libc::c_int
                >= 97 as libc::c_int
            {
                if (ch as libc::c_int | 32 as libc::c_int) as libc::c_uchar
                    as libc::c_int <= 102 as libc::c_int
                {
                    return s_http_host_v6;
                }
            }
            if ch as libc::c_int == 58 as libc::c_int {
                return s_http_host_v6
            } else {
                if ch as libc::c_int == 46 as libc::c_int {
                    return s_http_host_v6;
                }
            }
            if s as libc::c_uint == 7 as libc::c_uint {
                if ch as libc::c_int == 37 as libc::c_int {
                    return s_http_host_v6_zone_start;
                }
            }
        }
        17152226948168769005 => {
            if ch as libc::c_int == 58 as libc::c_int {
                return s_http_host_port_start;
            }
        }
        9364233119027335301 => {
            if (ch as libc::c_int | 32 as libc::c_int) as libc::c_uchar as libc::c_int
                >= 97 as libc::c_int
            {
                if (ch as libc::c_int | 32 as libc::c_int) as libc::c_uchar
                    as libc::c_int <= 122 as libc::c_int
                {
                    return s_http_host_v6_zone;
                }
            }
            if ch as libc::c_int >= 48 as libc::c_int {
                if ch as libc::c_int <= 57 as libc::c_int {
                    return s_http_host_v6_zone;
                }
            }
            if ch as libc::c_int == 37 as libc::c_int {
                return s_http_host_v6_zone
            } else {
                if ch as libc::c_int == 46 as libc::c_int {
                    return s_http_host_v6_zone
                } else {
                    if ch as libc::c_int == 45 as libc::c_int {
                        return s_http_host_v6_zone
                    } else {
                        if ch as libc::c_int == 95 as libc::c_int {
                            return s_http_host_v6_zone
                        } else {
                            if ch as libc::c_int == 126 as libc::c_int {
                                return s_http_host_v6_zone;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    return s_http_host_dead;
}
unsafe extern "C" fn http_parse_host(
    mut buf: *const libc::c_char,
    mut u: *mut http_parser_url,
    mut found_at: libc::c_int,
) -> libc::c_int {
    let mut s: http_host_state = 0 as http_host_state;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut buflen: size_t = 0;
    let mut new_s: http_host_state = 0 as http_host_state;
    let mut tmp: http_host_state = 0 as http_host_state;
    buflen = ((*u).field_data[1 as libc::c_int as usize].off as libc::c_int
        + (*u).field_data[1 as libc::c_int as usize].len as libc::c_int) as size_t;
    if (*u).field_set as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int == 0 {
        __assert_fail(
            b"u->field_set & (1 << UF_HOST)\0" as *const u8 as *const libc::c_char,
            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
            2200 as libc::c_uint,
            b"http_parse_host\0" as *const u8 as *const libc::c_char,
        );
    }
    (*u).field_data[1 as libc::c_int as usize].len = 0 as libc::c_int as uint16_t;
    if found_at != 0 {
        s = s_http_userinfo_start;
    } else {
        s = s_http_host_start;
    }
    p = buf
        .offset((*u).field_data[1 as libc::c_int as usize].off as libc::c_int as isize);
    while (p as libc::c_ulong) < buf.offset(buflen as isize) as libc::c_ulong {
        tmp = http_parse_host_char(s, *p);
        new_s = tmp;
        if new_s as libc::c_uint == 1 as libc::c_uint {
            return 1 as libc::c_int;
        }
        match new_s as libc::c_uint {
            6 => {
                if s as libc::c_uint != 6 as libc::c_uint {
                    (*u)
                        .field_data[1 as libc::c_int as usize]
                        .off = p.offset_from(buf) as libc::c_long as uint16_t;
                }
                (*u)
                    .field_data[1 as libc::c_int as usize]
                    .len = ((*u).field_data[1 as libc::c_int as usize].len as libc::c_int
                    + 1 as libc::c_int) as uint16_t;
            }
            7 => {
                if s as libc::c_uint != 7 as libc::c_uint {
                    (*u)
                        .field_data[1 as libc::c_int as usize]
                        .off = p.offset_from(buf) as libc::c_long as uint16_t;
                }
                (*u)
                    .field_data[1 as libc::c_int as usize]
                    .len = ((*u).field_data[1 as libc::c_int as usize].len as libc::c_int
                    + 1 as libc::c_int) as uint16_t;
            }
            10 | 9 => {
                (*u)
                    .field_data[1 as libc::c_int as usize]
                    .len = ((*u).field_data[1 as libc::c_int as usize].len as libc::c_int
                    + 1 as libc::c_int) as uint16_t;
            }
            12 => {
                if s as libc::c_uint != 12 as libc::c_uint {
                    (*u)
                        .field_data[2 as libc::c_int as usize]
                        .off = p.offset_from(buf) as libc::c_long as uint16_t;
                    (*u)
                        .field_data[2 as libc::c_int as usize]
                        .len = 0 as libc::c_int as uint16_t;
                    (*u)
                        .field_set = ((*u).field_set as libc::c_int
                        | (1 as libc::c_int) << 2 as libc::c_int) as uint16_t;
                }
                (*u)
                    .field_data[2 as libc::c_int as usize]
                    .len = ((*u).field_data[2 as libc::c_int as usize].len as libc::c_int
                    + 1 as libc::c_int) as uint16_t;
            }
            3 => {
                if s as libc::c_uint != 3 as libc::c_uint {
                    (*u)
                        .field_data[6 as libc::c_int as usize]
                        .off = p.offset_from(buf) as libc::c_long as uint16_t;
                    (*u)
                        .field_data[6 as libc::c_int as usize]
                        .len = 0 as libc::c_int as uint16_t;
                    (*u)
                        .field_set = ((*u).field_set as libc::c_int
                        | (1 as libc::c_int) << 6 as libc::c_int) as uint16_t;
                }
                (*u)
                    .field_data[6 as libc::c_int as usize]
                    .len = ((*u).field_data[6 as libc::c_int as usize].len as libc::c_int
                    + 1 as libc::c_int) as uint16_t;
            }
            _ => {}
        }
        s = new_s;
        p = p.offset(1);
    }
    match s as libc::c_uint {
        2 | 3 | 11 | 10 | 9 | 7 | 5 | 4 => return 1 as libc::c_int,
        _ => {}
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn http_parser_url_init(mut u: *mut http_parser_url) {
    memset(
        u as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<http_parser_url>() as libc::c_ulong,
    );
}
pub unsafe extern "C" fn http_parser_parse_url(
    mut buf: *const libc::c_char,
    mut buflen: size_t,
    mut is_connect: libc::c_int,
    mut u: *mut http_parser_url,
) -> libc::c_int {
    let mut current_block: u64;
    let mut s: state = 0 as state;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut uf: http_parser_url_fields = UF_SCHEMA;
    let mut old_uf: http_parser_url_fields = UF_SCHEMA;
    let mut found_at: libc::c_int = 0;
    let mut tmp: uint16_t = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut v: libc::c_ulong = 0;
    let mut tmp___1: libc::c_ulong = 0;
    found_at = 0 as libc::c_int;
    tmp = 0 as libc::c_int as uint16_t;
    (*u).field_set = tmp;
    (*u).port = tmp;
    if is_connect != 0 {
        s = s_req_server_start;
    } else {
        s = s_req_spaces_before_url;
    }
    old_uf = UF_MAX;
    p = buf;
    while (p as libc::c_ulong) < buf.offset(buflen as isize) as libc::c_ulong {
        s = parse_url_char(s, *p);
        match s as libc::c_uint {
            1 => return 1 as libc::c_int,
            30 | 28 | 24 | 23 | 22 => {
                current_block = 2706383102307530838;
            }
            21 => {
                uf = UF_SCHEMA;
                current_block = 9828876828309294594;
            }
            26 => {
                found_at = 1 as libc::c_int;
                current_block = 14276528561031184103;
            }
            25 => {
                current_block = 14276528561031184103;
            }
            27 => {
                uf = UF_PATH;
                current_block = 9828876828309294594;
            }
            29 => {
                uf = UF_QUERY;
                current_block = 9828876828309294594;
            }
            31 => {
                uf = UF_FRAGMENT;
                current_block = 9828876828309294594;
            }
            _ => {
                __assert_fail(
                    b"!\"Unexpected state\"\0" as *const u8 as *const libc::c_char,
                    b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                    2334 as libc::c_uint,
                    b"http_parser_parse_url\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        match current_block {
            14276528561031184103 => {
                uf = UF_HOST;
                current_block = 9828876828309294594;
            }
            _ => {}
        }
        match current_block {
            9828876828309294594 => {
                if uf as libc::c_uint == old_uf as libc::c_uint {
                    (*u)
                        .field_data[uf as usize]
                        .len = ((*u).field_data[uf as usize].len as libc::c_int
                        + 1 as libc::c_int) as uint16_t;
                } else {
                    (*u)
                        .field_data[uf as usize]
                        .off = p.offset_from(buf) as libc::c_long as uint16_t;
                    (*u).field_data[uf as usize].len = 1 as libc::c_int as uint16_t;
                    (*u)
                        .field_set = ((*u).field_set as libc::c_int
                        | (1 as libc::c_int) << uf as libc::c_uint) as uint16_t;
                    old_uf = uf;
                }
            }
            _ => {}
        }
        p = p.offset(1);
    }
    if (*u).field_set as libc::c_int & 1 as libc::c_int != 0 {
        if (*u).field_set as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int
            == 0 as libc::c_int
        {
            return 1 as libc::c_int;
        }
    }
    if (*u).field_set as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        tmp___0 = http_parse_host(buf, u, found_at);
        if tmp___0 != 0 as libc::c_int {
            return 1 as libc::c_int;
        }
    }
    if is_connect != 0 {
        if (*u).field_set as libc::c_int
            != (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
        {
            return 1 as libc::c_int;
        }
    }
    if (*u).field_set as libc::c_int & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        tmp___1 = strtoul(
            buf
                .offset(
                    (*u).field_data[2 as libc::c_int as usize].off as libc::c_int
                        as isize,
                ),
            0 as *mut libc::c_void as *mut *mut libc::c_char,
            10 as libc::c_int,
        );
        v = tmp___1;
        if v > 65535 as libc::c_ulong {
            return 1 as libc::c_int;
        }
        (*u).port = v as uint16_t;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn http_parser_pause(
    mut parser: *mut http_parser,
    mut paused: libc::c_int,
) {
    if !((*parser).http_errno() as http_errno as libc::c_uint == 0 as libc::c_uint) {
        if !((*parser).http_errno() as http_errno as libc::c_uint == 31 as libc::c_uint)
        {
            __assert_fail(
                b"0 && \"Attempting to pause parser in error state\"\0" as *const u8
                    as *const libc::c_char,
                b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                2394 as libc::c_uint,
                b"http_parser_pause\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if paused != 0 {
        (*parser).set_http_errno(31 as libc::c_uint);
    } else {
        (*parser).set_http_errno(0 as libc::c_uint);
    };
}
pub unsafe extern "C" fn http_body_is_final(
    mut parser: *const http_parser,
) -> libc::c_int {
    return ((*parser).state() == 62 as libc::c_uint) as libc::c_int;
}
pub unsafe extern "C" fn http_parser_version() -> libc::c_ulong {
    return 132865 as libc::c_ulong;
}
pub static mut luaJIT_BC_wrk: [libc::c_uchar; 1069] = [
    27 as libc::c_int as libc::c_uchar,
    76 as libc::c_int as libc::c_uchar,
    74 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    139 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    66 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    21 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    41 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    41 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    77 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    56 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    66 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    88 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    54 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    66 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    79 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    244 as libc::c_int as libc::c_uchar,
    127 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    61 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    75 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    97 as libc::c_int as libc::c_uchar,
    100 as libc::c_int as libc::c_uchar,
    100 as libc::c_int as libc::c_uchar,
    114 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    114 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    109 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    97 as libc::c_int as libc::c_uchar,
    98 as libc::c_int as libc::c_uchar,
    108 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    99 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    110 as libc::c_int as libc::c_uchar,
    110 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    99 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    108 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    107 as libc::c_int as libc::c_uchar,
    117 as libc::c_int as libc::c_uchar,
    112 as libc::c_int as libc::c_uchar,
    92 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    58 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    61 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    54 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    54 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    66 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    88 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    54 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    66 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    75 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    102 as libc::c_int as libc::c_uchar,
    117 as libc::c_int as libc::c_uchar,
    110 as libc::c_int as libc::c_uchar,
    99 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    105 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    110 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    117 as libc::c_int as libc::c_uchar,
    112 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    121 as libc::c_int as libc::c_uchar,
    112 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    97 as libc::c_int as libc::c_uchar,
    100 as libc::c_int as libc::c_uchar,
    100 as libc::c_int as libc::c_uchar,
    114 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    97 as libc::c_int as libc::c_uchar,
    100 as libc::c_int as libc::c_uchar,
    100 as libc::c_int as libc::c_uchar,
    114 as libc::c_int as libc::c_uchar,
    17 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    76 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    142 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    48 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    88 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    27 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    66 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    88 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    38 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    88 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    88 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    38 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    88 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    61 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    54 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    54 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    66 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    88 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    54 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    66 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    66 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    61 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    50 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    75 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    114 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    113 as libc::c_int as libc::c_uchar,
    117 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    102 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    114 as libc::c_int as libc::c_uchar,
    109 as libc::c_int as libc::c_uchar,
    97 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    102 as libc::c_int as libc::c_uchar,
    117 as libc::c_int as libc::c_uchar,
    110 as libc::c_int as libc::c_uchar,
    99 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    105 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    110 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    105 as libc::c_int as libc::c_uchar,
    110 as libc::c_int as libc::c_uchar,
    105 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    121 as libc::c_int as libc::c_uchar,
    112 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    93 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    91 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    58 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    102 as libc::c_int as libc::c_uchar,
    105 as libc::c_int as libc::c_uchar,
    110 as libc::c_int as libc::c_uchar,
    100 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    112 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    114 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    104 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    72 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    104 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    97 as libc::c_int as libc::c_uchar,
    100 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    114 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    134 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    20 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    68 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    88 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    88 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    88 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    88 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    52 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    88 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    61 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    88 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    54 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    66 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    61 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    54 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    66 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    62 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    54 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    66 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    72 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    21 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    22 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    54 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    17 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    66 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    60 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    70 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    82 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    245 as libc::c_int as libc::c_uchar,
    127 as libc::c_int as libc::c_uchar,
    21 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    22 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    60 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    21 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    22 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    88 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    60 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    54 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    68 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    99 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    110 as libc::c_int as libc::c_uchar,
    99 as libc::c_int as libc::c_uchar,
    97 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    97 as libc::c_int as libc::c_uchar,
    98 as libc::c_int as libc::c_uchar,
    108 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    58 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    112 as libc::c_int as libc::c_uchar,
    97 as libc::c_int as libc::c_uchar,
    105 as libc::c_int as libc::c_uchar,
    114 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    72 as libc::c_int as libc::c_uchar,
    84 as libc::c_int as libc::c_uchar,
    84 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
    47 as libc::c_int as libc::c_uchar,
    49 as libc::c_int as libc::c_uchar,
    46 as libc::c_int as libc::c_uchar,
    49 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    102 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    114 as libc::c_int as libc::c_uchar,
    109 as libc::c_int as libc::c_uchar,
    97 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    108 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    110 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    114 as libc::c_int as libc::c_uchar,
    105 as libc::c_int as libc::c_uchar,
    110 as libc::c_int as libc::c_uchar,
    103 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    67 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    110 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    110 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    76 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    110 as libc::c_int as libc::c_uchar,
    103 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    104 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    72 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    98 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    100 as libc::c_int as libc::c_uchar,
    121 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    104 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    97 as libc::c_int as libc::c_uchar,
    100 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    114 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    112 as libc::c_int as libc::c_uchar,
    97 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    104 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    109 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    104 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    100 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    145 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    53 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    52 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    61 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    61 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    61 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    61 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    61 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    50 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    76 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    102 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    114 as libc::c_int as libc::c_uchar,
    109 as libc::c_int as libc::c_uchar,
    97 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    105 as libc::c_int as libc::c_uchar,
    110 as libc::c_int as libc::c_uchar,
    105 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    117 as libc::c_int as libc::c_uchar,
    112 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    114 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    108 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    104 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    97 as libc::c_int as libc::c_uchar,
    100 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    114 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    104 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    108 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    99 as libc::c_int as libc::c_uchar,
    97 as libc::c_int as libc::c_uchar,
    108 as libc::c_int as libc::c_uchar,
    104 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    109 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    104 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    100 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    71 as libc::c_int as libc::c_uchar,
    69 as libc::c_int as libc::c_uchar,
    84 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    99 as libc::c_int as libc::c_uchar,
    104 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    109 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    104 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    112 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    112 as libc::c_int as libc::c_uchar,
    97 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    104 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    47 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
pub static mut VERSION: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
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
