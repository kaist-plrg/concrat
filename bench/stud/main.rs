use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type bignum_ctx;
    pub type bn_blinding_st;
    pub type engine_st;
    pub type ASN1_VALUE_st;
    pub type evp_pkey_ctx_st;
    pub type ec_key_st;
    pub type evp_pkey_asn1_method_st;
    pub type X509_POLICY_CACHE_st;
    pub type x509_crl_method_st;
    pub type stack_st_GENERAL_NAMES;
    pub type X509_POLICY_TREE_st;
    pub type ssl3_buf_freelist_st;
    pub type cert_st;
    pub type sess_cert_st;
    pub type ssl3_enc_method;
    pub type stack_st_OCSP_RESPID;
    pub type _pqueue;
    pub type ev_loop;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
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
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn wait(__stat_loc: *mut libc::c_int) -> __pid_t;
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
    fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: socklen_t,
    ) -> *const libc::c_char;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pause() -> libc::c_int;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn getppid() -> __pid_t;
    fn setsid() -> __pid_t;
    fn setuid(__uid: __uid_t) -> libc::c_int;
    fn setgid(__gid: __gid_t) -> libc::c_int;
    fn fork() -> __pid_t;
    fn chroot(__path: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn openlog(
        __ident: *const libc::c_char,
        __option: libc::c_int,
        __facility: libc::c_int,
    );
    fn syslog(__pri: libc::c_int, __fmt: *const libc::c_char, _: ...);
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn sched_setaffinity(
        __pid: __pid_t,
        __cpusetsize: size_t,
        __cpuset: *const cpu_set_t,
    ) -> libc::c_int;
    fn sk_num(_: *const _STACK) -> libc::c_int;
    fn sk_value(_: *const _STACK, _: libc::c_int) -> *mut libc::c_void;
    fn sk_pop_free(
        st: *mut _STACK,
        func: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    );
    fn SSLeay() -> libc::c_ulong;
    fn BIO_test_flags(b: *const BIO, flags: libc::c_int) -> libc::c_int;
    fn BIO_s_file() -> *mut BIO_METHOD;
    fn BIO_new_file(
        filename: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> *mut BIO;
    fn BIO_new(type_0: *mut BIO_METHOD) -> *mut BIO;
    fn BIO_free(a: *mut BIO) -> libc::c_int;
    fn BIO_read(b: *mut BIO, data: *mut libc::c_void, len: libc::c_int) -> libc::c_int;
    fn BIO_ctrl(
        bp: *mut BIO,
        cmd: libc::c_int,
        larg: libc::c_long,
        parg: *mut libc::c_void,
    ) -> libc::c_long;
    fn ASN1_STRING_to_UTF8(
        out: *mut *mut libc::c_uchar,
        in_0: *mut ASN1_STRING,
    ) -> libc::c_int;
    fn EVP_sha1() -> *const EVP_MD;
    fn EC_KEY_new_by_curve_name(nid: libc::c_int) -> *mut EC_KEY;
    fn EC_KEY_free(key: *mut EC_KEY);
    fn RSA_free(r: *mut RSA);
    fn i2d_RSAPrivateKey(a: *const RSA, out: *mut *mut libc::c_uchar) -> libc::c_int;
    fn DH_free(dh: *mut DH);
    fn DH_size(dh: *const DH) -> libc::c_int;
    fn SHA1(
        d: *const libc::c_uchar,
        n: size_t,
        md: *mut libc::c_uchar,
    ) -> *mut libc::c_uchar;
    fn X509_get_subject_name(a: *mut X509) -> *mut X509_NAME;
    fn X509_NAME_get_index_by_NID(
        name: *mut X509_NAME,
        nid: libc::c_int,
        lastpos: libc::c_int,
    ) -> libc::c_int;
    fn X509_NAME_get_entry(
        name: *mut X509_NAME,
        loc: libc::c_int,
    ) -> *mut X509_NAME_ENTRY;
    fn X509_get_ext_d2i(
        x: *mut X509,
        nid: libc::c_int,
        crit: *mut libc::c_int,
        idx: *mut libc::c_int,
    ) -> *mut libc::c_void;
    fn PEM_read_bio_X509_AUX(
        bp: *mut BIO,
        x: *mut *mut X509,
        cb: Option::<pem_password_cb>,
        u: *mut libc::c_void,
    ) -> *mut X509;
    fn PEM_read_bio_RSAPrivateKey(
        bp: *mut BIO,
        x: *mut *mut RSA,
        cb: Option::<pem_password_cb>,
        u: *mut libc::c_void,
    ) -> *mut RSA;
    fn PEM_read_bio_DHparams(
        bp: *mut BIO,
        x: *mut *mut DH,
        cb: Option::<pem_password_cb>,
        u: *mut libc::c_void,
    ) -> *mut DH;
    fn HMAC(
        evp_md: *const EVP_MD,
        key: *const libc::c_void,
        key_len: libc::c_int,
        d: *const libc::c_uchar,
        n: size_t,
        md: *mut libc::c_uchar,
        md_len: *mut libc::c_uint,
    ) -> *mut libc::c_uchar;
    fn SSL_CTX_set_info_callback(
        ctx: *mut SSL_CTX,
        cb: Option::<unsafe extern "C" fn(*const SSL, libc::c_int, libc::c_int) -> ()>,
    );
    fn SSL_get_servername(s: *const SSL, type_0: libc::c_int) -> *const libc::c_char;
    fn SSL_CTX_set_cipher_list(_: *mut SSL_CTX, str: *const libc::c_char) -> libc::c_int;
    fn SSL_CTX_new(meth: *const SSL_METHOD) -> *mut SSL_CTX;
    fn SSL_CTX_get_timeout(ctx: *const SSL_CTX) -> libc::c_long;
    fn SSL_set_fd(s: *mut SSL, fd: libc::c_int) -> libc::c_int;
    fn SSL_get_rbio(s: *const SSL) -> *mut BIO;
    fn SSL_CTX_use_certificate_chain_file(
        ctx: *mut SSL_CTX,
        file: *const libc::c_char,
    ) -> libc::c_int;
    fn SSL_load_error_strings();
    fn SSL_SESSION_free(ses: *mut SSL_SESSION);
    fn SSL_set_session(to: *mut SSL, session: *mut SSL_SESSION) -> libc::c_int;
    fn SSL_CTX_use_RSAPrivateKey(ctx: *mut SSL_CTX, rsa: *mut RSA) -> libc::c_int;
    fn SSL_new(ctx: *mut SSL_CTX) -> *mut SSL;
    fn SSL_free(ssl_0: *mut SSL);
    fn SSL_read(
        ssl_0: *mut SSL,
        buf: *mut libc::c_void,
        num: libc::c_int,
    ) -> libc::c_int;
    fn SSL_write(
        ssl_0: *mut SSL,
        buf: *const libc::c_void,
        num: libc::c_int,
    ) -> libc::c_int;
    fn SSL_ctrl(
        ssl_0: *mut SSL,
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
    fn SSL_CTX_callback_ctrl(
        _: *mut SSL_CTX,
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn() -> ()>,
    ) -> libc::c_long;
    fn SSL_get_error(s: *const SSL, ret_code: libc::c_int) -> libc::c_int;
    fn SSLv23_server_method() -> *const SSL_METHOD;
    fn SSLv23_client_method() -> *const SSL_METHOD;
    fn TLSv1_server_method() -> *const SSL_METHOD;
    fn TLSv1_client_method() -> *const SSL_METHOD;
    fn SSL_do_handshake(s: *mut SSL) -> libc::c_int;
    fn SSL_set_connect_state(s: *mut SSL);
    fn SSL_set_accept_state(s: *mut SSL);
    fn SSL_library_init() -> libc::c_int;
    fn SSL_set_shutdown(ssl_0: *mut SSL, mode: libc::c_int);
    fn SSL_get1_session(ssl_0: *mut SSL) -> *mut SSL_SESSION;
    fn SSL_set_SSL_CTX(ssl_0: *mut SSL, ctx: *mut SSL_CTX) -> *mut SSL_CTX;
    fn SSL_set_ex_data(
        ssl_0: *mut SSL,
        idx: libc::c_int,
        data: *mut libc::c_void,
    ) -> libc::c_int;
    fn SSL_get_ex_data(ssl_0: *const SSL, idx: libc::c_int) -> *mut libc::c_void;
    fn GENERAL_NAME_free(a: *mut GENERAL_NAME);
    fn ERR_print_errors_fp(fp: *mut FILE);
    fn ENGINE_by_id(id: *const libc::c_char) -> *mut ENGINE;
    fn ENGINE_load_builtin_engines();
    fn ENGINE_register_all_complete() -> libc::c_int;
    fn ENGINE_free(e: *mut ENGINE) -> libc::c_int;
    fn ENGINE_get_id(e: *const ENGINE) -> *const libc::c_char;
    fn ENGINE_init(e: *mut ENGINE) -> libc::c_int;
    fn ENGINE_finish(e: *mut ENGINE) -> libc::c_int;
    fn ENGINE_set_default(e: *mut ENGINE, flags: libc::c_uint) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn ev_default_loop(flags: libc::c_uint) -> *mut ev_loop;
    fn ev_now(loop_1: *mut ev_loop) -> ev_tstamp;
    fn ev_run(loop_1: *mut ev_loop, flags: libc::c_int) -> libc::c_int;
    fn ev_io_start(loop_1: *mut ev_loop, w: *mut ev_io);
    fn ev_io_stop(loop_1: *mut ev_loop, w: *mut ev_io);
    fn ev_timer_start(loop_1: *mut ev_loop, w: *mut ev_timer);
    fn ev_timer_stop(loop_1: *mut ev_loop, w: *mut ev_timer);
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncat(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn geteuid() -> __uid_t;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn __xpg_basename(__path: *mut libc::c_char) -> *mut libc::c_char;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn getgrgid(__gid: __gid_t) -> *mut group;
    fn getgrnam(__name: *const libc::c_char) -> *mut group;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutexattr_init(__attr: *mut pthread_mutexattr_t) -> libc::c_int;
    fn pthread_mutexattr_setpshared(
        __attr: *mut pthread_mutexattr_t,
        __pshared: libc::c_int,
    ) -> libc::c_int;
    fn eb_delete(node: *mut eb_node);
    fn ebmb_lookup(
        root: *mut eb_root,
        x: *const libc::c_void,
        len: libc::c_uint,
    ) -> *mut ebmb_node;
    fn ebmb_insert(
        root: *mut eb_root,
        new: *mut ebmb_node,
        len: libc::c_uint,
    ) -> *mut ebmb_node;
    fn SSL_CTX_sess_set_new_cb(
        ctx: *mut SSL_CTX,
        new_session_cb: Option::<
            unsafe extern "C" fn(*mut ssl_st, *mut SSL_SESSION) -> libc::c_int,
        >,
    );
    fn SSL_CTX_sess_set_remove_cb(
        ctx: *mut SSL_CTX,
        remove_session_cb: Option::<
            unsafe extern "C" fn(*mut ssl_ctx_st, *mut SSL_SESSION) -> (),
        >,
    );
    fn SSL_CTX_sess_set_get_cb(
        ctx: *mut SSL_CTX,
        get_session_cb: Option::<
            unsafe extern "C" fn(
                *mut ssl_st,
                *mut libc::c_uchar,
                libc::c_int,
                *mut libc::c_int,
            ) -> *mut SSL_SESSION,
        >,
    );
    fn SSL_SESSION_get_time(s: *const SSL_SESSION) -> libc::c_long;
    fn SSL_SESSION_set_time(s: *mut SSL_SESSION, t: libc::c_long) -> libc::c_long;
    fn i2d_SSL_SESSION(
        in_0: *mut SSL_SESSION,
        pp: *mut *mut libc::c_uchar,
    ) -> libc::c_int;
    fn d2i_SSL_SESSION(
        a: *mut *mut SSL_SESSION,
        pp: *mut *const libc::c_uchar,
        length: libc::c_long,
    ) -> *mut SSL_SESSION;
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
pub type __uint8_t = libc::c_uchar;
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
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __caddr_t = *mut libc::c_char;
pub type __socklen_t = libc::c_uint;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
pub type int32_t = __int32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub type uint32_t = __uint32_t;
pub type in_port_t = uint16_t;
pub type uint16_t = __uint16_t;
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
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip_mreqn {
    pub imr_multiaddr: in_addr,
    pub imr_address: in_addr,
    pub imr_ifindex: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipv6_mreq {
    pub ipv6mr_multiaddr: in6_addr,
    pub ipv6mr_interface: libc::c_uint,
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
pub union __anonunion__bounds_979030728 {
    pub _addr_bnd: __anonstruct__addr_bnd_5259977,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct__sigfault_33223220 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: __anonunion__bounds_979030728,
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
pub union __anonunion__sifields_1013095824 {
    pub _pad: [libc::c_int; 28],
    pub _kill: __anonstruct__kill_244518854,
    pub _timer: __anonstruct__timer_490064738,
    pub _rt: __anonstruct__rt_619254530,
    pub _sigchld: __anonstruct__sigchld_284671705,
    pub _sigfault: __anonstruct__sigfault_33223220,
    pub _sigpoll: __anonstruct__sigpoll_386613454,
    pub _sigsys: __anonstruct__sigsys_44812255,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_siginfo_t_781435142 {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: __anonunion__sifields_1013095824,
}
pub type siginfo_t = __anonstruct_siginfo_t_781435142;
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
pub type __cpu_mask = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_cpu_set_t_826868708 {
    pub __bits: [__cpu_mask; 16],
}
pub type cpu_set_t = __anonstruct_cpu_set_t_826868708;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st {
    pub num: libc::c_int,
    pub data: *mut *mut libc::c_char,
    pub sorted: libc::c_int,
    pub num_alloc: libc::c_int,
    pub comp: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
}
pub type _STACK = stack_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct asn1_string_st {
    pub length: libc::c_int,
    pub type_0: libc::c_int,
    pub data: *mut libc::c_uchar,
    pub flags: libc::c_long,
}
pub type ASN1_INTEGER = asn1_string_st;
pub type ASN1_ENUMERATED = asn1_string_st;
pub type ASN1_BIT_STRING = asn1_string_st;
pub type ASN1_OCTET_STRING = asn1_string_st;
pub type ASN1_PRINTABLESTRING = asn1_string_st;
pub type ASN1_T61STRING = asn1_string_st;
pub type ASN1_IA5STRING = asn1_string_st;
pub type ASN1_GENERALSTRING = asn1_string_st;
pub type ASN1_UNIVERSALSTRING = asn1_string_st;
pub type ASN1_BMPSTRING = asn1_string_st;
pub type ASN1_UTCTIME = asn1_string_st;
pub type ASN1_TIME = asn1_string_st;
pub type ASN1_GENERALIZEDTIME = asn1_string_st;
pub type ASN1_VISIBLESTRING = asn1_string_st;
pub type ASN1_UTF8STRING = asn1_string_st;
pub type ASN1_STRING = asn1_string_st;
pub type ASN1_BOOLEAN = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bignum_st {
    pub d: *mut libc::c_ulong,
    pub top: libc::c_int,
    pub dmax: libc::c_int,
    pub neg: libc::c_int,
    pub flags: libc::c_int,
}
pub type BIGNUM = bignum_st;
pub type BN_CTX = bignum_ctx;
pub type BN_BLINDING = bn_blinding_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bn_mont_ctx_st {
    pub ri: libc::c_int,
    pub RR: BIGNUM,
    pub N: BIGNUM,
    pub Ni: BIGNUM,
    pub n0: [libc::c_ulong; 2],
    pub flags: libc::c_int,
}
pub type BN_MONT_CTX = bn_mont_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bn_gencb_st {
    pub ver: libc::c_uint,
    pub arg: *mut libc::c_void,
    pub cb: __anonunion_cb_888073939,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_cb_888073939 {
    pub cb_1: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
    >,
    pub cb_2: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, *mut BN_GENCB) -> libc::c_int,
    >,
}
pub type BN_GENCB = bn_gencb_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buf_mem_st {
    pub length: size_t,
    pub data: *mut libc::c_char,
    pub max: size_t,
}
pub type BUF_MEM = buf_mem_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct evp_cipher_st {
    pub nid: libc::c_int,
    pub block_size: libc::c_int,
    pub key_len: libc::c_int,
    pub iv_len: libc::c_int,
    pub flags: libc::c_ulong,
    pub init: Option::<
        unsafe extern "C" fn(
            *mut EVP_CIPHER_CTX,
            *const libc::c_uchar,
            *const libc::c_uchar,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub do_cipher: Option::<
        unsafe extern "C" fn(
            *mut EVP_CIPHER_CTX,
            *mut libc::c_uchar,
            *const libc::c_uchar,
            size_t,
        ) -> libc::c_int,
    >,
    pub cleanup: Option::<unsafe extern "C" fn(*mut EVP_CIPHER_CTX) -> libc::c_int>,
    pub ctx_size: libc::c_int,
    pub set_asn1_parameters: Option::<
        unsafe extern "C" fn(*mut EVP_CIPHER_CTX, *mut ASN1_TYPE) -> libc::c_int,
    >,
    pub get_asn1_parameters: Option::<
        unsafe extern "C" fn(*mut EVP_CIPHER_CTX, *mut ASN1_TYPE) -> libc::c_int,
    >,
    pub ctrl: Option::<
        unsafe extern "C" fn(
            *mut EVP_CIPHER_CTX,
            libc::c_int,
            libc::c_int,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub app_data: *mut libc::c_void,
}
pub type EVP_CIPHER_CTX = evp_cipher_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct evp_cipher_ctx_st {
    pub cipher: *const EVP_CIPHER,
    pub engine: *mut ENGINE,
    pub encrypt: libc::c_int,
    pub buf_len: libc::c_int,
    pub oiv: [libc::c_uchar; 16],
    pub iv: [libc::c_uchar; 16],
    pub buf: [libc::c_uchar; 32],
    pub num: libc::c_int,
    pub app_data: *mut libc::c_void,
    pub key_len: libc::c_int,
    pub flags: libc::c_ulong,
    pub cipher_data: *mut libc::c_void,
    pub final_used: libc::c_int,
    pub block_mask: libc::c_int,
    pub final_0: [libc::c_uchar; 32],
}
pub type ENGINE = engine_st;
pub type EVP_CIPHER = evp_cipher_st;
pub type ASN1_TYPE = asn1_type_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct asn1_type_st {
    pub type_0: libc::c_int,
    pub value: __anonunion_value_401497255,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_value_401497255 {
    pub ptr: *mut libc::c_char,
    pub boolean: ASN1_BOOLEAN,
    pub asn1_string: *mut ASN1_STRING,
    pub object: *mut ASN1_OBJECT,
    pub integer: *mut ASN1_INTEGER,
    pub enumerated: *mut ASN1_ENUMERATED,
    pub bit_string: *mut ASN1_BIT_STRING,
    pub octet_string: *mut ASN1_OCTET_STRING,
    pub printablestring: *mut ASN1_PRINTABLESTRING,
    pub t61string: *mut ASN1_T61STRING,
    pub ia5string: *mut ASN1_IA5STRING,
    pub generalstring: *mut ASN1_GENERALSTRING,
    pub bmpstring: *mut ASN1_BMPSTRING,
    pub universalstring: *mut ASN1_UNIVERSALSTRING,
    pub utctime: *mut ASN1_UTCTIME,
    pub generalizedtime: *mut ASN1_GENERALIZEDTIME,
    pub visiblestring: *mut ASN1_VISIBLESTRING,
    pub utf8string: *mut ASN1_UTF8STRING,
    pub set: *mut ASN1_STRING,
    pub sequence: *mut ASN1_STRING,
    pub asn1_value: *mut ASN1_VALUE,
}
pub type ASN1_VALUE = ASN1_VALUE_st;
pub type ASN1_OBJECT = asn1_object_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct asn1_object_st {
    pub sn: *const libc::c_char,
    pub ln: *const libc::c_char,
    pub nid: libc::c_int,
    pub length: libc::c_int,
    pub data: *const libc::c_uchar,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct env_md_st {
    pub type_0: libc::c_int,
    pub pkey_type: libc::c_int,
    pub md_size: libc::c_int,
    pub flags: libc::c_ulong,
    pub init: Option::<unsafe extern "C" fn(*mut EVP_MD_CTX) -> libc::c_int>,
    pub update: Option::<
        unsafe extern "C" fn(*mut EVP_MD_CTX, *const libc::c_void, size_t) -> libc::c_int,
    >,
    pub final_0: Option::<
        unsafe extern "C" fn(*mut EVP_MD_CTX, *mut libc::c_uchar) -> libc::c_int,
    >,
    pub copy: Option::<
        unsafe extern "C" fn(*mut EVP_MD_CTX, *const EVP_MD_CTX) -> libc::c_int,
    >,
    pub cleanup: Option::<unsafe extern "C" fn(*mut EVP_MD_CTX) -> libc::c_int>,
    pub sign: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            libc::c_uint,
            *mut libc::c_uchar,
            *mut libc::c_uint,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub verify: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            libc::c_uint,
            *const libc::c_uchar,
            libc::c_uint,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub required_pkey_type: [libc::c_int; 5],
    pub block_size: libc::c_int,
    pub ctx_size: libc::c_int,
    pub md_ctrl: Option::<
        unsafe extern "C" fn(
            *mut EVP_MD_CTX,
            libc::c_int,
            libc::c_int,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
}
pub type EVP_MD_CTX = env_md_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct env_md_ctx_st {
    pub digest: *const EVP_MD,
    pub engine: *mut ENGINE,
    pub flags: libc::c_ulong,
    pub md_data: *mut libc::c_void,
    pub pctx: *mut EVP_PKEY_CTX,
    pub update: Option::<
        unsafe extern "C" fn(*mut EVP_MD_CTX, *const libc::c_void, size_t) -> libc::c_int,
    >,
}
pub type EVP_PKEY_CTX = evp_pkey_ctx_st;
pub type EVP_MD = env_md_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct evp_pkey_st {
    pub type_0: libc::c_int,
    pub save_type: libc::c_int,
    pub references: libc::c_int,
    pub ameth: *const EVP_PKEY_ASN1_METHOD,
    pub engine: *mut ENGINE,
    pub pkey: __anonunion_pkey_1024245030,
    pub save_parameters: libc::c_int,
    pub attributes: *mut stack_st_X509_ATTRIBUTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_X509_ATTRIBUTE {
    pub stack: _STACK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pkey_1024245030 {
    pub ptr: *mut libc::c_char,
    pub rsa: *mut rsa_st,
    pub dsa: *mut dsa_st,
    pub dh: *mut dh_st,
    pub ec: *mut ec_key_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dh_st {
    pub pad: libc::c_int,
    pub version: libc::c_int,
    pub p: *mut BIGNUM,
    pub g: *mut BIGNUM,
    pub length: libc::c_long,
    pub pub_key: *mut BIGNUM,
    pub priv_key: *mut BIGNUM,
    pub flags: libc::c_int,
    pub method_mont_p: *mut BN_MONT_CTX,
    pub q: *mut BIGNUM,
    pub j: *mut BIGNUM,
    pub seed: *mut libc::c_uchar,
    pub seedlen: libc::c_int,
    pub counter: *mut BIGNUM,
    pub references: libc::c_int,
    pub ex_data: CRYPTO_EX_DATA,
    pub meth: *const DH_METHOD,
    pub engine: *mut ENGINE,
}
pub type DH_METHOD = dh_method;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dh_method {
    pub name: *const libc::c_char,
    pub generate_key: Option::<unsafe extern "C" fn(*mut DH) -> libc::c_int>,
    pub compute_key: Option::<
        unsafe extern "C" fn(*mut libc::c_uchar, *const BIGNUM, *mut DH) -> libc::c_int,
    >,
    pub bn_mod_exp: Option::<
        unsafe extern "C" fn(
            *const DH,
            *mut BIGNUM,
            *const BIGNUM,
            *const BIGNUM,
            *const BIGNUM,
            *mut BN_CTX,
            *mut BN_MONT_CTX,
        ) -> libc::c_int,
    >,
    pub init: Option::<unsafe extern "C" fn(*mut DH) -> libc::c_int>,
    pub finish: Option::<unsafe extern "C" fn(*mut DH) -> libc::c_int>,
    pub flags: libc::c_int,
    pub app_data: *mut libc::c_char,
    pub generate_params: Option::<
        unsafe extern "C" fn(
            *mut DH,
            libc::c_int,
            libc::c_int,
            *mut BN_GENCB,
        ) -> libc::c_int,
    >,
}
pub type DH = dh_st;
pub type CRYPTO_EX_DATA = crypto_ex_data_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crypto_ex_data_st {
    pub sk: *mut stack_st_void,
    pub dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_void {
    pub stack: _STACK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsa_st {
    pub pad: libc::c_int,
    pub version: libc::c_long,
    pub write_params: libc::c_int,
    pub p: *mut BIGNUM,
    pub q: *mut BIGNUM,
    pub g: *mut BIGNUM,
    pub pub_key: *mut BIGNUM,
    pub priv_key: *mut BIGNUM,
    pub kinv: *mut BIGNUM,
    pub r: *mut BIGNUM,
    pub flags: libc::c_int,
    pub method_mont_p: *mut BN_MONT_CTX,
    pub references: libc::c_int,
    pub ex_data: CRYPTO_EX_DATA,
    pub meth: *const DSA_METHOD,
    pub engine: *mut ENGINE,
}
pub type DSA_METHOD = dsa_method;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsa_method {
    pub name: *const libc::c_char,
    pub dsa_do_sign: Option::<
        unsafe extern "C" fn(*const libc::c_uchar, libc::c_int, *mut DSA) -> *mut DSA_SIG,
    >,
    pub dsa_sign_setup: Option::<
        unsafe extern "C" fn(
            *mut DSA,
            *mut BN_CTX,
            *mut *mut BIGNUM,
            *mut *mut BIGNUM,
        ) -> libc::c_int,
    >,
    pub dsa_do_verify: Option::<
        unsafe extern "C" fn(
            *const libc::c_uchar,
            libc::c_int,
            *mut DSA_SIG,
            *mut DSA,
        ) -> libc::c_int,
    >,
    pub dsa_mod_exp: Option::<
        unsafe extern "C" fn(
            *mut DSA,
            *mut BIGNUM,
            *mut BIGNUM,
            *mut BIGNUM,
            *mut BIGNUM,
            *mut BIGNUM,
            *mut BIGNUM,
            *mut BN_CTX,
            *mut BN_MONT_CTX,
        ) -> libc::c_int,
    >,
    pub bn_mod_exp: Option::<
        unsafe extern "C" fn(
            *mut DSA,
            *mut BIGNUM,
            *mut BIGNUM,
            *const BIGNUM,
            *const BIGNUM,
            *mut BN_CTX,
            *mut BN_MONT_CTX,
        ) -> libc::c_int,
    >,
    pub init: Option::<unsafe extern "C" fn(*mut DSA) -> libc::c_int>,
    pub finish: Option::<unsafe extern "C" fn(*mut DSA) -> libc::c_int>,
    pub flags: libc::c_int,
    pub app_data: *mut libc::c_char,
    pub dsa_paramgen: Option::<
        unsafe extern "C" fn(
            *mut DSA,
            libc::c_int,
            *const libc::c_uchar,
            libc::c_int,
            *mut libc::c_int,
            *mut libc::c_ulong,
            *mut BN_GENCB,
        ) -> libc::c_int,
    >,
    pub dsa_keygen: Option::<unsafe extern "C" fn(*mut DSA) -> libc::c_int>,
}
pub type DSA = dsa_st;
pub type DSA_SIG = DSA_SIG_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DSA_SIG_st {
    pub r: *mut BIGNUM,
    pub s: *mut BIGNUM,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsa_st {
    pub pad: libc::c_int,
    pub version: libc::c_long,
    pub meth: *const RSA_METHOD,
    pub engine: *mut ENGINE,
    pub n: *mut BIGNUM,
    pub e: *mut BIGNUM,
    pub d: *mut BIGNUM,
    pub p: *mut BIGNUM,
    pub q: *mut BIGNUM,
    pub dmp1: *mut BIGNUM,
    pub dmq1: *mut BIGNUM,
    pub iqmp: *mut BIGNUM,
    pub ex_data: CRYPTO_EX_DATA,
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub _method_mod_n: *mut BN_MONT_CTX,
    pub _method_mod_p: *mut BN_MONT_CTX,
    pub _method_mod_q: *mut BN_MONT_CTX,
    pub bignum_data: *mut libc::c_char,
    pub blinding: *mut BN_BLINDING,
    pub mt_blinding: *mut BN_BLINDING,
}
pub type RSA_METHOD = rsa_meth_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsa_meth_st {
    pub name: *const libc::c_char,
    pub rsa_pub_enc: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            *mut libc::c_uchar,
            *mut RSA,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub rsa_pub_dec: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            *mut libc::c_uchar,
            *mut RSA,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub rsa_priv_enc: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            *mut libc::c_uchar,
            *mut RSA,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub rsa_priv_dec: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            *mut libc::c_uchar,
            *mut RSA,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub rsa_mod_exp: Option::<
        unsafe extern "C" fn(
            *mut BIGNUM,
            *const BIGNUM,
            *mut RSA,
            *mut BN_CTX,
        ) -> libc::c_int,
    >,
    pub bn_mod_exp: Option::<
        unsafe extern "C" fn(
            *mut BIGNUM,
            *const BIGNUM,
            *const BIGNUM,
            *const BIGNUM,
            *mut BN_CTX,
            *mut BN_MONT_CTX,
        ) -> libc::c_int,
    >,
    pub init: Option::<unsafe extern "C" fn(*mut RSA) -> libc::c_int>,
    pub finish: Option::<unsafe extern "C" fn(*mut RSA) -> libc::c_int>,
    pub flags: libc::c_int,
    pub app_data: *mut libc::c_char,
    pub rsa_sign: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            libc::c_uint,
            *mut libc::c_uchar,
            *mut libc::c_uint,
            *const RSA,
        ) -> libc::c_int,
    >,
    pub rsa_verify: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            libc::c_uint,
            *const libc::c_uchar,
            libc::c_uint,
            *const RSA,
        ) -> libc::c_int,
    >,
    pub rsa_keygen: Option::<
        unsafe extern "C" fn(
            *mut RSA,
            libc::c_int,
            *mut BIGNUM,
            *mut BN_GENCB,
        ) -> libc::c_int,
    >,
}
pub type RSA = rsa_st;
pub type EVP_PKEY_ASN1_METHOD = evp_pkey_asn1_method_st;
pub type EVP_PKEY = evp_pkey_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x509_st {
    pub cert_info: *mut X509_CINF,
    pub sig_alg: *mut X509_ALGOR,
    pub signature: *mut ASN1_BIT_STRING,
    pub valid: libc::c_int,
    pub references: libc::c_int,
    pub name: *mut libc::c_char,
    pub ex_data: CRYPTO_EX_DATA,
    pub ex_pathlen: libc::c_long,
    pub ex_pcpathlen: libc::c_long,
    pub ex_flags: libc::c_ulong,
    pub ex_kusage: libc::c_ulong,
    pub ex_xkusage: libc::c_ulong,
    pub ex_nscert: libc::c_ulong,
    pub skid: *mut ASN1_OCTET_STRING,
    pub akid: *mut AUTHORITY_KEYID,
    pub policy_cache: *mut X509_POLICY_CACHE,
    pub crldp: *mut stack_st_DIST_POINT,
    pub altname: *mut stack_st_GENERAL_NAME,
    pub nc: *mut NAME_CONSTRAINTS,
    pub sha1_hash: [libc::c_uchar; 20],
    pub aux: *mut X509_CERT_AUX,
}
pub type X509_CERT_AUX = x509_cert_aux_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x509_cert_aux_st {
    pub trust: *mut stack_st_ASN1_OBJECT,
    pub reject: *mut stack_st_ASN1_OBJECT,
    pub alias: *mut ASN1_UTF8STRING,
    pub keyid: *mut ASN1_OCTET_STRING,
    pub other: *mut stack_st_X509_ALGOR,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_X509_ALGOR {
    pub stack: _STACK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_ASN1_OBJECT {
    pub stack: _STACK,
}
pub type NAME_CONSTRAINTS = NAME_CONSTRAINTS_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NAME_CONSTRAINTS_st {
    pub permittedSubtrees: *mut stack_st_GENERAL_SUBTREE,
    pub excludedSubtrees: *mut stack_st_GENERAL_SUBTREE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_GENERAL_SUBTREE {
    pub stack: _STACK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_GENERAL_NAME {
    pub stack: _STACK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_DIST_POINT {
    pub stack: _STACK,
}
pub type X509_POLICY_CACHE = X509_POLICY_CACHE_st;
pub type AUTHORITY_KEYID = AUTHORITY_KEYID_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AUTHORITY_KEYID_st {
    pub keyid: *mut ASN1_OCTET_STRING,
    pub issuer: *mut GENERAL_NAMES,
    pub serial: *mut ASN1_INTEGER,
}
pub type GENERAL_NAMES = stack_st_GENERAL_NAME;
pub type X509_ALGOR = X509_algor_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct X509_algor_st {
    pub algorithm: *mut ASN1_OBJECT,
    pub parameter: *mut ASN1_TYPE,
}
pub type X509_CINF = x509_cinf_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x509_cinf_st {
    pub version: *mut ASN1_INTEGER,
    pub serialNumber: *mut ASN1_INTEGER,
    pub signature: *mut X509_ALGOR,
    pub issuer: *mut X509_NAME,
    pub validity: *mut X509_VAL,
    pub subject: *mut X509_NAME,
    pub key: *mut X509_PUBKEY,
    pub issuerUID: *mut ASN1_BIT_STRING,
    pub subjectUID: *mut ASN1_BIT_STRING,
    pub extensions: *mut stack_st_X509_EXTENSION,
    pub enc: ASN1_ENCODING,
}
pub type ASN1_ENCODING = ASN1_ENCODING_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ASN1_ENCODING_st {
    pub enc: *mut libc::c_uchar,
    pub len: libc::c_long,
    pub modified: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_X509_EXTENSION {
    pub stack: _STACK,
}
pub type X509_PUBKEY = X509_pubkey_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct X509_pubkey_st {
    pub algor: *mut X509_ALGOR,
    pub public_key: *mut ASN1_BIT_STRING,
    pub pkey: *mut EVP_PKEY,
}
pub type X509_NAME = X509_name_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct X509_name_st {
    pub entries: *mut stack_st_X509_NAME_ENTRY,
    pub modified: libc::c_int,
    pub bytes: *mut BUF_MEM,
    pub canon_enc: *mut libc::c_uchar,
    pub canon_enclen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_X509_NAME_ENTRY {
    pub stack: _STACK,
}
pub type X509_VAL = X509_val_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct X509_val_st {
    pub notBefore: *mut ASN1_TIME,
    pub notAfter: *mut ASN1_TIME,
}
pub type X509 = x509_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct X509_crl_st {
    pub crl: *mut X509_CRL_INFO,
    pub sig_alg: *mut X509_ALGOR,
    pub signature: *mut ASN1_BIT_STRING,
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub akid: *mut AUTHORITY_KEYID,
    pub idp: *mut ISSUING_DIST_POINT,
    pub idp_flags: libc::c_int,
    pub idp_reasons: libc::c_int,
    pub crl_number: *mut ASN1_INTEGER,
    pub base_crl_number: *mut ASN1_INTEGER,
    pub sha1_hash: [libc::c_uchar; 20],
    pub issuers: *mut stack_st_GENERAL_NAMES,
    pub meth: *const X509_CRL_METHOD,
    pub meth_data: *mut libc::c_void,
}
pub type X509_CRL_METHOD = x509_crl_method_st;
pub type ISSUING_DIST_POINT = ISSUING_DIST_POINT_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ISSUING_DIST_POINT_st {
    pub distpoint: *mut DIST_POINT_NAME,
    pub onlyuser: libc::c_int,
    pub onlyCA: libc::c_int,
    pub onlysomereasons: *mut ASN1_BIT_STRING,
    pub indirectCRL: libc::c_int,
    pub onlyattr: libc::c_int,
}
pub type DIST_POINT_NAME = DIST_POINT_NAME_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DIST_POINT_NAME_st {
    pub type_0: libc::c_int,
    pub name: __anonunion_name_997224671,
    pub dpname: *mut X509_NAME,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_name_997224671 {
    pub fullname: *mut GENERAL_NAMES,
    pub relativename: *mut stack_st_X509_NAME_ENTRY,
}
pub type X509_CRL_INFO = X509_crl_info_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct X509_crl_info_st {
    pub version: *mut ASN1_INTEGER,
    pub sig_alg: *mut X509_ALGOR,
    pub issuer: *mut X509_NAME,
    pub lastUpdate: *mut ASN1_TIME,
    pub nextUpdate: *mut ASN1_TIME,
    pub revoked: *mut stack_st_X509_REVOKED,
    pub extensions: *mut stack_st_X509_EXTENSION,
    pub enc: ASN1_ENCODING,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_X509_REVOKED {
    pub stack: _STACK,
}
pub type X509_CRL = X509_crl_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x509_store_st {
    pub cache: libc::c_int,
    pub objs: *mut stack_st_X509_OBJECT,
    pub get_cert_methods: *mut stack_st_X509_LOOKUP,
    pub param: *mut X509_VERIFY_PARAM,
    pub verify: Option::<unsafe extern "C" fn(*mut X509_STORE_CTX) -> libc::c_int>,
    pub verify_cb: Option::<
        unsafe extern "C" fn(libc::c_int, *mut X509_STORE_CTX) -> libc::c_int,
    >,
    pub get_issuer: Option::<
        unsafe extern "C" fn(
            *mut *mut X509,
            *mut X509_STORE_CTX,
            *mut X509,
        ) -> libc::c_int,
    >,
    pub check_issued: Option::<
        unsafe extern "C" fn(*mut X509_STORE_CTX, *mut X509, *mut X509) -> libc::c_int,
    >,
    pub check_revocation: Option::<
        unsafe extern "C" fn(*mut X509_STORE_CTX) -> libc::c_int,
    >,
    pub get_crl: Option::<
        unsafe extern "C" fn(
            *mut X509_STORE_CTX,
            *mut *mut X509_CRL,
            *mut X509,
        ) -> libc::c_int,
    >,
    pub check_crl: Option::<
        unsafe extern "C" fn(*mut X509_STORE_CTX, *mut X509_CRL) -> libc::c_int,
    >,
    pub cert_crl: Option::<
        unsafe extern "C" fn(
            *mut X509_STORE_CTX,
            *mut X509_CRL,
            *mut X509,
        ) -> libc::c_int,
    >,
    pub lookup_certs: Option::<
        unsafe extern "C" fn(*mut X509_STORE_CTX, *mut X509_NAME) -> *mut stack_st_X509,
    >,
    pub lookup_crls: Option::<
        unsafe extern "C" fn(
            *mut X509_STORE_CTX,
            *mut X509_NAME,
        ) -> *mut stack_st_X509_CRL,
    >,
    pub cleanup: Option::<unsafe extern "C" fn(*mut X509_STORE_CTX) -> libc::c_int>,
    pub ex_data: CRYPTO_EX_DATA,
    pub references: libc::c_int,
}
pub type X509_STORE_CTX = x509_store_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x509_store_ctx_st {
    pub ctx: *mut X509_STORE,
    pub current_method: libc::c_int,
    pub cert: *mut X509,
    pub untrusted: *mut stack_st_X509,
    pub crls: *mut stack_st_X509_CRL,
    pub param: *mut X509_VERIFY_PARAM,
    pub other_ctx: *mut libc::c_void,
    pub verify: Option::<unsafe extern "C" fn(*mut X509_STORE_CTX) -> libc::c_int>,
    pub verify_cb: Option::<
        unsafe extern "C" fn(libc::c_int, *mut X509_STORE_CTX) -> libc::c_int,
    >,
    pub get_issuer: Option::<
        unsafe extern "C" fn(
            *mut *mut X509,
            *mut X509_STORE_CTX,
            *mut X509,
        ) -> libc::c_int,
    >,
    pub check_issued: Option::<
        unsafe extern "C" fn(*mut X509_STORE_CTX, *mut X509, *mut X509) -> libc::c_int,
    >,
    pub check_revocation: Option::<
        unsafe extern "C" fn(*mut X509_STORE_CTX) -> libc::c_int,
    >,
    pub get_crl: Option::<
        unsafe extern "C" fn(
            *mut X509_STORE_CTX,
            *mut *mut X509_CRL,
            *mut X509,
        ) -> libc::c_int,
    >,
    pub check_crl: Option::<
        unsafe extern "C" fn(*mut X509_STORE_CTX, *mut X509_CRL) -> libc::c_int,
    >,
    pub cert_crl: Option::<
        unsafe extern "C" fn(
            *mut X509_STORE_CTX,
            *mut X509_CRL,
            *mut X509,
        ) -> libc::c_int,
    >,
    pub check_policy: Option::<unsafe extern "C" fn(*mut X509_STORE_CTX) -> libc::c_int>,
    pub lookup_certs: Option::<
        unsafe extern "C" fn(*mut X509_STORE_CTX, *mut X509_NAME) -> *mut stack_st_X509,
    >,
    pub lookup_crls: Option::<
        unsafe extern "C" fn(
            *mut X509_STORE_CTX,
            *mut X509_NAME,
        ) -> *mut stack_st_X509_CRL,
    >,
    pub cleanup: Option::<unsafe extern "C" fn(*mut X509_STORE_CTX) -> libc::c_int>,
    pub valid: libc::c_int,
    pub last_untrusted: libc::c_int,
    pub chain: *mut stack_st_X509,
    pub tree: *mut X509_POLICY_TREE,
    pub explicit_policy: libc::c_int,
    pub error_depth: libc::c_int,
    pub error: libc::c_int,
    pub current_cert: *mut X509,
    pub current_issuer: *mut X509,
    pub current_crl: *mut X509_CRL,
    pub current_crl_score: libc::c_int,
    pub current_reasons: libc::c_uint,
    pub parent: *mut X509_STORE_CTX,
    pub ex_data: CRYPTO_EX_DATA,
}
pub type X509_POLICY_TREE = X509_POLICY_TREE_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_X509 {
    pub stack: _STACK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_X509_CRL {
    pub stack: _STACK,
}
pub type X509_VERIFY_PARAM = X509_VERIFY_PARAM_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct X509_VERIFY_PARAM_st {
    pub name: *mut libc::c_char,
    pub check_time: time_t,
    pub inh_flags: libc::c_ulong,
    pub flags: libc::c_ulong,
    pub purpose: libc::c_int,
    pub trust: libc::c_int,
    pub depth: libc::c_int,
    pub policies: *mut stack_st_ASN1_OBJECT,
}
pub type X509_STORE = x509_store_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_X509_LOOKUP {
    pub stack: _STACK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_X509_OBJECT {
    pub stack: _STACK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ssl_st {
    pub version: libc::c_int,
    pub type_0: libc::c_int,
    pub method: *const SSL_METHOD,
    pub rbio: *mut BIO,
    pub wbio: *mut BIO,
    pub bbio: *mut BIO,
    pub rwstate: libc::c_int,
    pub in_handshake: libc::c_int,
    pub handshake_func: Option::<unsafe extern "C" fn(*mut SSL) -> libc::c_int>,
    pub server: libc::c_int,
    pub new_session: libc::c_int,
    pub quiet_shutdown: libc::c_int,
    pub shutdown: libc::c_int,
    pub state: libc::c_int,
    pub rstate: libc::c_int,
    pub init_buf: *mut BUF_MEM,
    pub init_msg: *mut libc::c_void,
    pub init_num: libc::c_int,
    pub init_off: libc::c_int,
    pub packet: *mut libc::c_uchar,
    pub packet_length: libc::c_uint,
    pub s2: *mut ssl2_state_st,
    pub s3: *mut ssl3_state_st,
    pub d1: *mut dtls1_state_st,
    pub read_ahead: libc::c_int,
    pub msg_callback: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *const libc::c_void,
            size_t,
            *mut SSL,
            *mut libc::c_void,
        ) -> (),
    >,
    pub msg_callback_arg: *mut libc::c_void,
    pub hit: libc::c_int,
    pub param: *mut X509_VERIFY_PARAM,
    pub cipher_list: *mut stack_st_SSL_CIPHER,
    pub cipher_list_by_id: *mut stack_st_SSL_CIPHER,
    pub mac_flags: libc::c_int,
    pub enc_read_ctx: *mut EVP_CIPHER_CTX,
    pub read_hash: *mut EVP_MD_CTX,
    pub expand: *mut COMP_CTX,
    pub enc_write_ctx: *mut EVP_CIPHER_CTX,
    pub write_hash: *mut EVP_MD_CTX,
    pub compress: *mut COMP_CTX,
    pub cert: *mut cert_st,
    pub sid_ctx_length: libc::c_uint,
    pub sid_ctx: [libc::c_uchar; 32],
    pub session: *mut SSL_SESSION,
    pub generate_session_id: Option::<
        unsafe extern "C" fn(
            *const SSL,
            *mut libc::c_uchar,
            *mut libc::c_uint,
        ) -> libc::c_int,
    >,
    pub verify_mode: libc::c_int,
    pub verify_callback: Option::<
        unsafe extern "C" fn(libc::c_int, *mut X509_STORE_CTX) -> libc::c_int,
    >,
    pub info_callback: Option::<
        unsafe extern "C" fn(*const SSL, libc::c_int, libc::c_int) -> (),
    >,
    pub error: libc::c_int,
    pub error_code: libc::c_int,
    pub psk_client_callback: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            *const libc::c_char,
            *mut libc::c_char,
            libc::c_uint,
            *mut libc::c_uchar,
            libc::c_uint,
        ) -> libc::c_uint,
    >,
    pub psk_server_callback: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            *const libc::c_char,
            *mut libc::c_uchar,
            libc::c_uint,
        ) -> libc::c_uint,
    >,
    pub ctx: *mut SSL_CTX,
    pub debug: libc::c_int,
    pub verify_result: libc::c_long,
    pub ex_data: CRYPTO_EX_DATA,
    pub client_CA: *mut stack_st_X509_NAME,
    pub references: libc::c_int,
    pub options: libc::c_ulong,
    pub mode: libc::c_ulong,
    pub max_cert_list: libc::c_long,
    pub first_packet: libc::c_int,
    pub client_version: libc::c_int,
    pub max_send_fragment: libc::c_uint,
    pub tlsext_debug_cb: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            libc::c_int,
            libc::c_int,
            *mut libc::c_uchar,
            libc::c_int,
            *mut libc::c_void,
        ) -> (),
    >,
    pub tlsext_debug_arg: *mut libc::c_void,
    pub tlsext_hostname: *mut libc::c_char,
    pub servername_done: libc::c_int,
    pub tlsext_status_type: libc::c_int,
    pub tlsext_status_expected: libc::c_int,
    pub tlsext_ocsp_ids: *mut stack_st_OCSP_RESPID,
    pub tlsext_ocsp_exts: *mut X509_EXTENSIONS,
    pub tlsext_ocsp_resp: *mut libc::c_uchar,
    pub tlsext_ocsp_resplen: libc::c_int,
    pub tlsext_ticket_expected: libc::c_int,
    pub tlsext_ecpointformatlist_length: size_t,
    pub tlsext_ecpointformatlist: *mut libc::c_uchar,
    pub tlsext_ellipticcurvelist_length: size_t,
    pub tlsext_ellipticcurvelist: *mut libc::c_uchar,
    pub tlsext_opaque_prf_input: *mut libc::c_void,
    pub tlsext_opaque_prf_input_len: size_t,
    pub tlsext_session_ticket: *mut TLS_SESSION_TICKET_EXT,
    pub tls_session_ticket_ext_cb: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            *const libc::c_uchar,
            libc::c_int,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub tls_session_ticket_ext_cb_arg: *mut libc::c_void,
    pub tls_session_secret_cb: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            *mut libc::c_void,
            *mut libc::c_int,
            *mut stack_st_SSL_CIPHER,
            *mut *mut SSL_CIPHER,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub tls_session_secret_cb_arg: *mut libc::c_void,
    pub initial_ctx: *mut SSL_CTX,
    pub next_proto_negotiated: *mut libc::c_uchar,
    pub next_proto_negotiated_len: libc::c_uchar,
    pub srtp_profiles: *mut stack_st_SRTP_PROTECTION_PROFILE,
    pub srtp_profile: *mut SRTP_PROTECTION_PROFILE,
    pub tlsext_heartbeat: libc::c_uint,
    pub tlsext_hb_pending: libc::c_uint,
    pub tlsext_hb_seq: libc::c_uint,
    pub renegotiate: libc::c_int,
    pub srp_ctx: SRP_CTX,
}
pub type SRP_CTX = srp_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct srp_ctx_st {
    pub SRP_cb_arg: *mut libc::c_void,
    pub TLS_ext_srp_username_callback: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            *mut libc::c_int,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub SRP_verify_param_callback: Option::<
        unsafe extern "C" fn(*mut SSL, *mut libc::c_void) -> libc::c_int,
    >,
    pub SRP_give_srp_client_pwd_callback: Option::<
        unsafe extern "C" fn(*mut SSL, *mut libc::c_void) -> *mut libc::c_char,
    >,
    pub login: *mut libc::c_char,
    pub N: *mut BIGNUM,
    pub g: *mut BIGNUM,
    pub s: *mut BIGNUM,
    pub B: *mut BIGNUM,
    pub A: *mut BIGNUM,
    pub a: *mut BIGNUM,
    pub b: *mut BIGNUM,
    pub v: *mut BIGNUM,
    pub info: *mut libc::c_char,
    pub strength: libc::c_int,
    pub srp_Mask: libc::c_ulong,
}
pub type SSL = ssl_st;
pub type SRTP_PROTECTION_PROFILE = srtp_protection_profile_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct srtp_protection_profile_st {
    pub name: *const libc::c_char,
    pub id: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_SRTP_PROTECTION_PROFILE {
    pub stack: _STACK,
}
pub type SSL_CTX = ssl_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ssl_ctx_st {
    pub method: *const SSL_METHOD,
    pub cipher_list: *mut stack_st_SSL_CIPHER,
    pub cipher_list_by_id: *mut stack_st_SSL_CIPHER,
    pub cert_store: *mut x509_store_st,
    pub sessions: *mut lhash_st_SSL_SESSION,
    pub session_cache_size: libc::c_ulong,
    pub session_cache_head: *mut ssl_session_st,
    pub session_cache_tail: *mut ssl_session_st,
    pub session_cache_mode: libc::c_int,
    pub session_timeout: libc::c_long,
    pub new_session_cb: Option::<
        unsafe extern "C" fn(*mut ssl_st, *mut SSL_SESSION) -> libc::c_int,
    >,
    pub remove_session_cb: Option::<
        unsafe extern "C" fn(*mut ssl_ctx_st, *mut SSL_SESSION) -> (),
    >,
    pub get_session_cb: Option::<
        unsafe extern "C" fn(
            *mut ssl_st,
            *mut libc::c_uchar,
            libc::c_int,
            *mut libc::c_int,
        ) -> *mut SSL_SESSION,
    >,
    pub stats: __anonstruct_stats_359712565,
    pub references: libc::c_int,
    pub app_verify_callback: Option::<
        unsafe extern "C" fn(*mut X509_STORE_CTX, *mut libc::c_void) -> libc::c_int,
    >,
    pub app_verify_arg: *mut libc::c_void,
    pub default_passwd_callback: Option::<pem_password_cb>,
    pub default_passwd_callback_userdata: *mut libc::c_void,
    pub client_cert_cb: Option::<
        unsafe extern "C" fn(*mut SSL, *mut *mut X509, *mut *mut EVP_PKEY) -> libc::c_int,
    >,
    pub app_gen_cookie_cb: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            *mut libc::c_uchar,
            *mut libc::c_uint,
        ) -> libc::c_int,
    >,
    pub app_verify_cookie_cb: Option::<
        unsafe extern "C" fn(*mut SSL, *mut libc::c_uchar, libc::c_uint) -> libc::c_int,
    >,
    pub ex_data: CRYPTO_EX_DATA,
    pub rsa_md5: *const EVP_MD,
    pub md5: *const EVP_MD,
    pub sha1: *const EVP_MD,
    pub extra_certs: *mut stack_st_X509,
    pub comp_methods: *mut stack_st_SSL_COMP,
    pub info_callback: Option::<
        unsafe extern "C" fn(*const SSL, libc::c_int, libc::c_int) -> (),
    >,
    pub client_CA: *mut stack_st_X509_NAME,
    pub options: libc::c_ulong,
    pub mode: libc::c_ulong,
    pub max_cert_list: libc::c_long,
    pub cert: *mut cert_st,
    pub read_ahead: libc::c_int,
    pub msg_callback: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *const libc::c_void,
            size_t,
            *mut SSL,
            *mut libc::c_void,
        ) -> (),
    >,
    pub msg_callback_arg: *mut libc::c_void,
    pub verify_mode: libc::c_int,
    pub sid_ctx_length: libc::c_uint,
    pub sid_ctx: [libc::c_uchar; 32],
    pub default_verify_callback: Option::<
        unsafe extern "C" fn(libc::c_int, *mut X509_STORE_CTX) -> libc::c_int,
    >,
    pub generate_session_id: Option::<
        unsafe extern "C" fn(
            *const SSL,
            *mut libc::c_uchar,
            *mut libc::c_uint,
        ) -> libc::c_int,
    >,
    pub param: *mut X509_VERIFY_PARAM,
    pub quiet_shutdown: libc::c_int,
    pub max_send_fragment: libc::c_uint,
    pub client_cert_engine: *mut ENGINE,
    pub tlsext_servername_callback: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            *mut libc::c_int,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub tlsext_servername_arg: *mut libc::c_void,
    pub tlsext_tick_key_name: [libc::c_uchar; 16],
    pub tlsext_tick_hmac_key: [libc::c_uchar; 16],
    pub tlsext_tick_aes_key: [libc::c_uchar; 16],
    pub tlsext_ticket_key_cb: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            *mut libc::c_uchar,
            *mut libc::c_uchar,
            *mut EVP_CIPHER_CTX,
            *mut HMAC_CTX,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub tlsext_status_cb: Option::<
        unsafe extern "C" fn(*mut SSL, *mut libc::c_void) -> libc::c_int,
    >,
    pub tlsext_status_arg: *mut libc::c_void,
    pub tlsext_opaque_prf_input_callback: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            *mut libc::c_void,
            size_t,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub tlsext_opaque_prf_input_callback_arg: *mut libc::c_void,
    pub psk_identity_hint: *mut libc::c_char,
    pub psk_client_callback: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            *const libc::c_char,
            *mut libc::c_char,
            libc::c_uint,
            *mut libc::c_uchar,
            libc::c_uint,
        ) -> libc::c_uint,
    >,
    pub psk_server_callback: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            *const libc::c_char,
            *mut libc::c_uchar,
            libc::c_uint,
        ) -> libc::c_uint,
    >,
    pub freelist_max_len: libc::c_uint,
    pub wbuf_freelist: *mut ssl3_buf_freelist_st,
    pub rbuf_freelist: *mut ssl3_buf_freelist_st,
    pub srp_ctx: SRP_CTX,
    pub next_protos_advertised_cb: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            *mut *const libc::c_uchar,
            *mut libc::c_uint,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub next_protos_advertised_cb_arg: *mut libc::c_void,
    pub next_proto_select_cb: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            *mut *mut libc::c_uchar,
            *mut libc::c_uchar,
            *const libc::c_uchar,
            libc::c_uint,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub next_proto_select_cb_arg: *mut libc::c_void,
    pub srtp_profiles: *mut stack_st_SRTP_PROTECTION_PROFILE,
}
pub type HMAC_CTX = hmac_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hmac_ctx_st {
    pub md: *const EVP_MD,
    pub md_ctx: EVP_MD_CTX,
    pub i_ctx: EVP_MD_CTX,
    pub o_ctx: EVP_MD_CTX,
    pub key_length: libc::c_uint,
    pub key: [libc::c_uchar; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_X509_NAME {
    pub stack: _STACK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_SSL_COMP {
    pub stack: _STACK,
}
pub type pem_password_cb = unsafe extern "C" fn(
    *mut libc::c_char,
    libc::c_int,
    libc::c_int,
    *mut libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_stats_359712565 {
    pub sess_connect: libc::c_int,
    pub sess_connect_renegotiate: libc::c_int,
    pub sess_connect_good: libc::c_int,
    pub sess_accept: libc::c_int,
    pub sess_accept_renegotiate: libc::c_int,
    pub sess_accept_good: libc::c_int,
    pub sess_miss: libc::c_int,
    pub sess_timeout: libc::c_int,
    pub sess_cache_full: libc::c_int,
    pub sess_hit: libc::c_int,
    pub sess_cb_hit: libc::c_int,
}
pub type SSL_SESSION = ssl_session_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ssl_session_st {
    pub ssl_version: libc::c_int,
    pub key_arg_length: libc::c_uint,
    pub key_arg: [libc::c_uchar; 8],
    pub master_key_length: libc::c_int,
    pub master_key: [libc::c_uchar; 48],
    pub session_id_length: libc::c_uint,
    pub session_id: [libc::c_uchar; 32],
    pub sid_ctx_length: libc::c_uint,
    pub sid_ctx: [libc::c_uchar; 32],
    pub psk_identity_hint: *mut libc::c_char,
    pub psk_identity: *mut libc::c_char,
    pub not_resumable: libc::c_int,
    pub sess_cert: *mut sess_cert_st,
    pub peer: *mut X509,
    pub verify_result: libc::c_long,
    pub references: libc::c_int,
    pub timeout: libc::c_long,
    pub time: libc::c_long,
    pub compress_meth: libc::c_uint,
    pub cipher: *const SSL_CIPHER,
    pub cipher_id: libc::c_ulong,
    pub ciphers: *mut stack_st_SSL_CIPHER,
    pub ex_data: CRYPTO_EX_DATA,
    pub prev: *mut ssl_session_st,
    pub next: *mut ssl_session_st,
    pub tlsext_hostname: *mut libc::c_char,
    pub tlsext_ecpointformatlist_length: size_t,
    pub tlsext_ecpointformatlist: *mut libc::c_uchar,
    pub tlsext_ellipticcurvelist_length: size_t,
    pub tlsext_ellipticcurvelist: *mut libc::c_uchar,
    pub tlsext_tick: *mut libc::c_uchar,
    pub tlsext_ticklen: size_t,
    pub tlsext_tick_lifetime_hint: libc::c_long,
    pub srp_username: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_SSL_CIPHER {
    pub stack: _STACK,
}
pub type SSL_CIPHER = ssl_cipher_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ssl_cipher_st {
    pub valid: libc::c_int,
    pub name: *const libc::c_char,
    pub id: libc::c_ulong,
    pub algorithm_mkey: libc::c_ulong,
    pub algorithm_auth: libc::c_ulong,
    pub algorithm_enc: libc::c_ulong,
    pub algorithm_mac: libc::c_ulong,
    pub algorithm_ssl: libc::c_ulong,
    pub algo_strength: libc::c_ulong,
    pub algorithm2: libc::c_ulong,
    pub strength_bits: libc::c_int,
    pub alg_bits: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lhash_st_SSL_SESSION {
    pub dummy: libc::c_int,
}
pub type SSL_METHOD = ssl_method_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ssl_method_st {
    pub version: libc::c_int,
    pub ssl_new: Option::<unsafe extern "C" fn(*mut SSL) -> libc::c_int>,
    pub ssl_clear: Option::<unsafe extern "C" fn(*mut SSL) -> ()>,
    pub ssl_free: Option::<unsafe extern "C" fn(*mut SSL) -> ()>,
    pub ssl_accept: Option::<unsafe extern "C" fn(*mut SSL) -> libc::c_int>,
    pub ssl_connect: Option::<unsafe extern "C" fn(*mut SSL) -> libc::c_int>,
    pub ssl_read: Option::<
        unsafe extern "C" fn(*mut SSL, *mut libc::c_void, libc::c_int) -> libc::c_int,
    >,
    pub ssl_peek: Option::<
        unsafe extern "C" fn(*mut SSL, *mut libc::c_void, libc::c_int) -> libc::c_int,
    >,
    pub ssl_write: Option::<
        unsafe extern "C" fn(*mut SSL, *const libc::c_void, libc::c_int) -> libc::c_int,
    >,
    pub ssl_shutdown: Option::<unsafe extern "C" fn(*mut SSL) -> libc::c_int>,
    pub ssl_renegotiate: Option::<unsafe extern "C" fn(*mut SSL) -> libc::c_int>,
    pub ssl_renegotiate_check: Option::<unsafe extern "C" fn(*mut SSL) -> libc::c_int>,
    pub ssl_get_message: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_long,
            *mut libc::c_int,
        ) -> libc::c_long,
    >,
    pub ssl_read_bytes: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            libc::c_int,
            *mut libc::c_uchar,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub ssl_write_bytes: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            libc::c_int,
            *const libc::c_void,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub ssl_dispatch_alert: Option::<unsafe extern "C" fn(*mut SSL) -> libc::c_int>,
    pub ssl_ctrl: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            libc::c_int,
            libc::c_long,
            *mut libc::c_void,
        ) -> libc::c_long,
    >,
    pub ssl_ctx_ctrl: Option::<
        unsafe extern "C" fn(
            *mut SSL_CTX,
            libc::c_int,
            libc::c_long,
            *mut libc::c_void,
        ) -> libc::c_long,
    >,
    pub get_cipher_by_char: Option::<
        unsafe extern "C" fn(*const libc::c_uchar) -> *const SSL_CIPHER,
    >,
    pub put_cipher_by_char: Option::<
        unsafe extern "C" fn(*const SSL_CIPHER, *mut libc::c_uchar) -> libc::c_int,
    >,
    pub ssl_pending: Option::<unsafe extern "C" fn(*const SSL) -> libc::c_int>,
    pub num_ciphers: Option::<unsafe extern "C" fn() -> libc::c_int>,
    pub get_cipher: Option::<unsafe extern "C" fn(libc::c_uint) -> *const SSL_CIPHER>,
    pub get_ssl_method: Option::<
        unsafe extern "C" fn(libc::c_int) -> *const ssl_method_st,
    >,
    pub get_timeout: Option::<unsafe extern "C" fn() -> libc::c_long>,
    pub ssl3_enc: *mut ssl3_enc_method,
    pub ssl_version: Option::<unsafe extern "C" fn() -> libc::c_int>,
    pub ssl_callback_ctrl: Option::<
        unsafe extern "C" fn(
            *mut SSL,
            libc::c_int,
            Option::<unsafe extern "C" fn() -> ()>,
        ) -> libc::c_long,
    >,
    pub ssl_ctx_callback_ctrl: Option::<
        unsafe extern "C" fn(
            *mut SSL_CTX,
            libc::c_int,
            Option::<unsafe extern "C" fn() -> ()>,
        ) -> libc::c_long,
    >,
}
pub type TLS_SESSION_TICKET_EXT = tls_session_ticket_ext_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tls_session_ticket_ext_st {
    pub length: libc::c_ushort,
    pub data: *mut libc::c_void,
}
pub type X509_EXTENSIONS = stack_st_X509_EXTENSION;
pub type COMP_CTX = comp_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct comp_ctx_st {
    pub meth: *mut COMP_METHOD,
    pub compress_in: libc::c_ulong,
    pub compress_out: libc::c_ulong,
    pub expand_in: libc::c_ulong,
    pub expand_out: libc::c_ulong,
    pub ex_data: CRYPTO_EX_DATA,
}
pub type COMP_METHOD = comp_method_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct comp_method_st {
    pub type_0: libc::c_int,
    pub name: *const libc::c_char,
    pub init: Option::<unsafe extern "C" fn(*mut COMP_CTX) -> libc::c_int>,
    pub finish: Option::<unsafe extern "C" fn(*mut COMP_CTX) -> ()>,
    pub compress: Option::<
        unsafe extern "C" fn(
            *mut COMP_CTX,
            *mut libc::c_uchar,
            libc::c_uint,
            *mut libc::c_uchar,
            libc::c_uint,
        ) -> libc::c_int,
    >,
    pub expand: Option::<
        unsafe extern "C" fn(
            *mut COMP_CTX,
            *mut libc::c_uchar,
            libc::c_uint,
            *mut libc::c_uchar,
            libc::c_uint,
        ) -> libc::c_int,
    >,
    pub ctrl: Option::<unsafe extern "C" fn() -> libc::c_long>,
    pub callback_ctrl: Option::<unsafe extern "C" fn() -> libc::c_long>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dtls1_state_st {
    pub send_cookie: libc::c_uint,
    pub cookie: [libc::c_uchar; 256],
    pub rcvd_cookie: [libc::c_uchar; 256],
    pub cookie_len: libc::c_uint,
    pub r_epoch: libc::c_ushort,
    pub w_epoch: libc::c_ushort,
    pub bitmap: DTLS1_BITMAP,
    pub next_bitmap: DTLS1_BITMAP,
    pub handshake_write_seq: libc::c_ushort,
    pub next_handshake_write_seq: libc::c_ushort,
    pub handshake_read_seq: libc::c_ushort,
    pub last_write_sequence: [libc::c_uchar; 8],
    pub unprocessed_rcds: record_pqueue,
    pub processed_rcds: record_pqueue,
    pub buffered_messages: pqueue,
    pub sent_messages: pqueue,
    pub buffered_app_data: record_pqueue,
    pub listen: libc::c_uint,
    pub link_mtu: libc::c_uint,
    pub mtu: libc::c_uint,
    pub w_msg_hdr: hm_header_st,
    pub r_msg_hdr: hm_header_st,
    pub timeout: dtls1_timeout_st,
    pub next_timeout: timeval,
    pub timeout_duration: libc::c_ushort,
    pub alert_fragment: [libc::c_uchar; 2],
    pub alert_fragment_len: libc::c_uint,
    pub handshake_fragment: [libc::c_uchar; 12],
    pub handshake_fragment_len: libc::c_uint,
    pub retransmitting: libc::c_uint,
    pub change_cipher_spec_ok: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dtls1_timeout_st {
    pub read_timeouts: libc::c_uint,
    pub write_timeouts: libc::c_uint,
    pub num_alerts: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hm_header_st {
    pub type_0: libc::c_uchar,
    pub msg_len: libc::c_ulong,
    pub seq: libc::c_ushort,
    pub frag_off: libc::c_ulong,
    pub frag_len: libc::c_ulong,
    pub is_ccs: libc::c_uint,
    pub saved_retransmit_state: dtls1_retransmit_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dtls1_retransmit_state {
    pub enc_write_ctx: *mut EVP_CIPHER_CTX,
    pub write_hash: *mut EVP_MD_CTX,
    pub compress: *mut COMP_CTX,
    pub session: *mut SSL_SESSION,
    pub epoch: libc::c_ushort,
}
pub type record_pqueue = record_pqueue_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct record_pqueue_st {
    pub epoch: libc::c_ushort,
    pub q: pqueue,
}
pub type pqueue = *mut _pqueue;
pub type DTLS1_BITMAP = dtls1_bitmap_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dtls1_bitmap_st {
    pub map: libc::c_ulong,
    pub max_seq_num: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ssl3_state_st {
    pub flags: libc::c_long,
    pub delay_buf_pop_ret: libc::c_int,
    pub read_sequence: [libc::c_uchar; 8],
    pub read_mac_secret_size: libc::c_int,
    pub read_mac_secret: [libc::c_uchar; 64],
    pub write_sequence: [libc::c_uchar; 8],
    pub write_mac_secret_size: libc::c_int,
    pub write_mac_secret: [libc::c_uchar; 64],
    pub server_random: [libc::c_uchar; 32],
    pub client_random: [libc::c_uchar; 32],
    pub need_empty_fragments: libc::c_int,
    pub empty_fragment_done: libc::c_int,
    pub init_extra: libc::c_int,
    pub rbuf: SSL3_BUFFER,
    pub wbuf: SSL3_BUFFER,
    pub rrec: SSL3_RECORD,
    pub wrec: SSL3_RECORD,
    pub alert_fragment: [libc::c_uchar; 2],
    pub alert_fragment_len: libc::c_uint,
    pub handshake_fragment: [libc::c_uchar; 4],
    pub handshake_fragment_len: libc::c_uint,
    pub wnum: libc::c_uint,
    pub wpend_tot: libc::c_int,
    pub wpend_type: libc::c_int,
    pub wpend_ret: libc::c_int,
    pub wpend_buf: *const libc::c_uchar,
    pub handshake_buffer: *mut BIO,
    pub handshake_dgst: *mut *mut EVP_MD_CTX,
    pub change_cipher_spec: libc::c_int,
    pub warn_alert: libc::c_int,
    pub fatal_alert: libc::c_int,
    pub alert_dispatch: libc::c_int,
    pub send_alert: [libc::c_uchar; 2],
    pub renegotiate: libc::c_int,
    pub total_renegotiations: libc::c_int,
    pub num_renegotiations: libc::c_int,
    pub in_read_app_data: libc::c_int,
    pub client_opaque_prf_input: *mut libc::c_void,
    pub client_opaque_prf_input_len: size_t,
    pub server_opaque_prf_input: *mut libc::c_void,
    pub server_opaque_prf_input_len: size_t,
    pub tmp: __anonstruct_tmp_985610861,
    pub previous_client_finished: [libc::c_uchar; 64],
    pub previous_client_finished_len: libc::c_uchar,
    pub previous_server_finished: [libc::c_uchar; 64],
    pub previous_server_finished_len: libc::c_uchar,
    pub send_connection_binding: libc::c_int,
    pub next_proto_neg_seen: libc::c_int,
    pub is_probably_safari: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_tmp_985610861 {
    pub cert_verify_md: [libc::c_uchar; 128],
    pub finish_md: [libc::c_uchar; 128],
    pub finish_md_len: libc::c_int,
    pub peer_finish_md: [libc::c_uchar; 128],
    pub peer_finish_md_len: libc::c_int,
    pub message_size: libc::c_ulong,
    pub message_type: libc::c_int,
    pub new_cipher: *const SSL_CIPHER,
    pub dh: *mut DH,
    pub ecdh: *mut EC_KEY,
    pub next_state: libc::c_int,
    pub reuse_message: libc::c_int,
    pub cert_req: libc::c_int,
    pub ctype_num: libc::c_int,
    pub ctype: [libc::c_char; 9],
    pub ca_names: *mut stack_st_X509_NAME,
    pub use_rsa_tmp: libc::c_int,
    pub key_block_length: libc::c_int,
    pub key_block: *mut libc::c_uchar,
    pub new_sym_enc: *const EVP_CIPHER,
    pub new_hash: *const EVP_MD,
    pub new_mac_pkey_type: libc::c_int,
    pub new_mac_secret_size: libc::c_int,
    pub new_compression: *const SSL_COMP,
    pub cert_request: libc::c_int,
}
pub type SSL_COMP = ssl_comp_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ssl_comp_st {
    pub id: libc::c_int,
    pub name: *const libc::c_char,
    pub method: *mut COMP_METHOD,
}
pub type EC_KEY = ec_key_st;
pub type BIO = bio_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bio_st {
    pub method: *mut BIO_METHOD,
    pub callback: Option::<
        unsafe extern "C" fn(
            *mut bio_st,
            libc::c_int,
            *const libc::c_char,
            libc::c_int,
            libc::c_long,
            libc::c_long,
        ) -> libc::c_long,
    >,
    pub cb_arg: *mut libc::c_char,
    pub init: libc::c_int,
    pub shutdown: libc::c_int,
    pub flags: libc::c_int,
    pub retry_reason: libc::c_int,
    pub num: libc::c_int,
    pub ptr: *mut libc::c_void,
    pub next_bio: *mut bio_st,
    pub prev_bio: *mut bio_st,
    pub references: libc::c_int,
    pub num_read: libc::c_ulong,
    pub num_write: libc::c_ulong,
    pub ex_data: CRYPTO_EX_DATA,
}
pub type BIO_METHOD = bio_method_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bio_method_st {
    pub type_0: libc::c_int,
    pub name: *const libc::c_char,
    pub bwrite: Option::<
        unsafe extern "C" fn(*mut BIO, *const libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub bread: Option::<
        unsafe extern "C" fn(*mut BIO, *mut libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub bputs: Option::<
        unsafe extern "C" fn(*mut BIO, *const libc::c_char) -> libc::c_int,
    >,
    pub bgets: Option::<
        unsafe extern "C" fn(*mut BIO, *mut libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub ctrl: Option::<
        unsafe extern "C" fn(
            *mut BIO,
            libc::c_int,
            libc::c_long,
            *mut libc::c_void,
        ) -> libc::c_long,
    >,
    pub create: Option::<unsafe extern "C" fn(*mut BIO) -> libc::c_int>,
    pub destroy: Option::<unsafe extern "C" fn(*mut BIO) -> libc::c_int>,
    pub callback_ctrl: Option::<
        unsafe extern "C" fn(
            *mut BIO,
            libc::c_int,
            Option::<bio_info_cb>,
        ) -> libc::c_long,
    >,
}
pub type bio_info_cb = unsafe extern "C" fn(
    *mut bio_st,
    libc::c_int,
    *const libc::c_char,
    libc::c_int,
    libc::c_long,
    libc::c_long,
) -> ();
pub type SSL3_RECORD = ssl3_record_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ssl3_record_st {
    pub type_0: libc::c_int,
    pub length: libc::c_uint,
    pub off: libc::c_uint,
    pub data: *mut libc::c_uchar,
    pub input: *mut libc::c_uchar,
    pub comp: *mut libc::c_uchar,
    pub epoch: libc::c_ulong,
    pub seq_num: [libc::c_uchar; 8],
}
pub type SSL3_BUFFER = ssl3_buffer_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ssl3_buffer_st {
    pub buf: *mut libc::c_uchar,
    pub len: size_t,
    pub offset: libc::c_int,
    pub left: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ssl2_state_st {
    pub three_byte_header: libc::c_int,
    pub clear_text: libc::c_int,
    pub escape: libc::c_int,
    pub ssl2_rollback: libc::c_int,
    pub wnum: libc::c_uint,
    pub wpend_tot: libc::c_int,
    pub wpend_buf: *const libc::c_uchar,
    pub wpend_off: libc::c_int,
    pub wpend_len: libc::c_int,
    pub wpend_ret: libc::c_int,
    pub rbuf_left: libc::c_int,
    pub rbuf_offs: libc::c_int,
    pub rbuf: *mut libc::c_uchar,
    pub wbuf: *mut libc::c_uchar,
    pub write_ptr: *mut libc::c_uchar,
    pub padding: libc::c_uint,
    pub rlength: libc::c_uint,
    pub ract_data_length: libc::c_int,
    pub wlength: libc::c_uint,
    pub wact_data_length: libc::c_int,
    pub ract_data: *mut libc::c_uchar,
    pub wact_data: *mut libc::c_uchar,
    pub mac_data: *mut libc::c_uchar,
    pub read_key: *mut libc::c_uchar,
    pub write_key: *mut libc::c_uchar,
    pub challenge_length: libc::c_uint,
    pub challenge: [libc::c_uchar; 32],
    pub conn_id_length: libc::c_uint,
    pub conn_id: [libc::c_uchar; 16],
    pub key_material_length: libc::c_uint,
    pub key_material: [libc::c_uchar; 48],
    pub read_sequence: libc::c_ulong,
    pub write_sequence: libc::c_ulong,
    pub tmp: __anonstruct_tmp_263383761,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_tmp_263383761 {
    pub conn_id_length: libc::c_uint,
    pub cert_type: libc::c_uint,
    pub cert_length: libc::c_uint,
    pub csl: libc::c_uint,
    pub clear: libc::c_uint,
    pub enc: libc::c_uint,
    pub ccl: [libc::c_uchar; 32],
    pub cipher_spec_length: libc::c_uint,
    pub session_id_length: libc::c_uint,
    pub clen: libc::c_uint,
    pub rlen: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct X509_name_entry_st {
    pub object: *mut ASN1_OBJECT,
    pub value: *mut ASN1_STRING,
    pub set: libc::c_int,
    pub size: libc::c_int,
}
pub type X509_NAME_ENTRY = X509_name_entry_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otherName_st {
    pub type_id: *mut ASN1_OBJECT,
    pub value: *mut ASN1_TYPE,
}
pub type OTHERNAME = otherName_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EDIPartyName_st {
    pub nameAssigner: *mut ASN1_STRING,
    pub partyName: *mut ASN1_STRING,
}
pub type EDIPARTYNAME = EDIPartyName_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_d_825737363 {
    pub ptr: *mut libc::c_char,
    pub otherName: *mut OTHERNAME,
    pub rfc822Name: *mut ASN1_IA5STRING,
    pub dNSName: *mut ASN1_IA5STRING,
    pub x400Address: *mut ASN1_TYPE,
    pub directoryName: *mut X509_NAME,
    pub ediPartyName: *mut EDIPARTYNAME,
    pub uniformResourceIdentifier: *mut ASN1_IA5STRING,
    pub iPAddress: *mut ASN1_OCTET_STRING,
    pub registeredID: *mut ASN1_OBJECT,
    pub ip: *mut ASN1_OCTET_STRING,
    pub dirn: *mut X509_NAME,
    pub ia5: *mut ASN1_IA5STRING,
    pub rid: *mut ASN1_OBJECT,
    pub other: *mut ASN1_TYPE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GENERAL_NAME_st {
    pub type_0: libc::c_int,
    pub d: __anonunion_d_825737363,
}
pub type GENERAL_NAME = GENERAL_NAME_st;
pub type ev_tstamp = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_watcher {
    pub active: libc::c_int,
    pub pending: libc::c_int,
    pub priority: libc::c_int,
    pub data: *mut libc::c_void,
    pub cb: Option::<
        unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_watcher_list {
    pub active: libc::c_int,
    pub pending: libc::c_int,
    pub priority: libc::c_int,
    pub data: *mut libc::c_void,
    pub cb: Option::<
        unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher_list, libc::c_int) -> (),
    >,
    pub next: *mut ev_watcher_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_watcher_time {
    pub active: libc::c_int,
    pub pending: libc::c_int,
    pub priority: libc::c_int,
    pub data: *mut libc::c_void,
    pub cb: Option::<
        unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher_time, libc::c_int) -> (),
    >,
    pub at: ev_tstamp,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_io {
    pub active: libc::c_int,
    pub pending: libc::c_int,
    pub priority: libc::c_int,
    pub data: *mut libc::c_void,
    pub cb: Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
    pub next: *mut ev_watcher_list,
    pub fd: libc::c_int,
    pub events: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_timer {
    pub active: libc::c_int,
    pub pending: libc::c_int,
    pub priority: libc::c_int,
    pub data: *mut libc::c_void,
    pub cb: Option::<
        unsafe extern "C" fn(*mut ev_loop, *mut ev_timer, libc::c_int) -> (),
    >,
    pub at: ev_tstamp,
    pub repeat: ev_tstamp,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bufent {
    pub data: [libc::c_char; 32768],
    pub ptr: *mut libc::c_char,
    pub left: size_t,
    pub next: *mut bufent,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ringbuffer {
    pub slots: [bufent; 3],
    pub head: *mut bufent,
    pub tail: *mut bufent,
    pub used: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shcupd_peer_opt {
    pub ip: *mut libc::c_char,
    pub port: *mut libc::c_char,
}
pub type __anonenum_ENC_TYPE_1013557608 = libc::c_uint;
pub const ENC_SSL: __anonenum_ENC_TYPE_1013557608 = 1;
pub const ENC_TLS: __anonenum_ENC_TYPE_1013557608 = 0;
pub type ENC_TYPE = __anonenum_ENC_TYPE_1013557608;
pub type __anonenum_PROXY_MODE_13968060 = libc::c_uint;
pub const SSL_CLIENT: __anonenum_PROXY_MODE_13968060 = 1;
pub const SSL_SERVER: __anonenum_PROXY_MODE_13968060 = 0;
pub type PROXY_MODE = __anonenum_PROXY_MODE_13968060;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cert_files {
    pub CERT_FILE: *mut libc::c_char,
    pub NEXT: *mut cert_files,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __stud_config {
    pub ETYPE: ENC_TYPE,
    pub PMODE: PROXY_MODE,
    pub WRITE_IP_OCTET: libc::c_int,
    pub WRITE_PROXY_LINE: libc::c_int,
    pub PROXY_PROXY_LINE: libc::c_int,
    pub CHROOT: *mut libc::c_char,
    pub UID: uid_t,
    pub GID: gid_t,
    pub FRONT_IP: *mut libc::c_char,
    pub FRONT_PORT: *mut libc::c_char,
    pub BACK_IP: *mut libc::c_char,
    pub BACK_PORT: *mut libc::c_char,
    pub NCORES: libc::c_long,
    pub CERT_FILES: *mut cert_files,
    pub CIPHER_SUITE: *mut libc::c_char,
    pub ENGINE: *mut libc::c_char,
    pub BACKLOG: libc::c_int,
    pub SHARED_CACHE: libc::c_int,
    pub SHCUPD_IP: *mut libc::c_char,
    pub SHCUPD_PORT: *mut libc::c_char,
    pub SHCUPD_PEERS: [shcupd_peer_opt; 16],
    pub SHCUPD_MCASTIF: *mut libc::c_char,
    pub SHCUPD_MCASTTTL: *mut libc::c_char,
    pub QUIET: libc::c_int,
    pub SYSLOG: libc::c_int,
    pub SYSLOG_FACILITY: libc::c_int,
    pub TCP_KEEPALIVE_TIME: libc::c_int,
    pub DAEMONIZE: libc::c_int,
    pub PREFER_SERVER_CIPHERS: libc::c_int,
}
pub type stud_config = __stud_config;
pub type _SHUTDOWN_REQUESTOR = libc::c_uint;
pub const SHUTDOWN_SSL: _SHUTDOWN_REQUESTOR = 2;
pub const SHUTDOWN_CLEAR: _SHUTDOWN_REQUESTOR = 1;
pub const SHUTDOWN_HARD: _SHUTDOWN_REQUESTOR = 0;
pub type SHUTDOWN_REQUESTOR = _SHUTDOWN_REQUESTOR;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ctx_list {
    pub servername: *mut libc::c_char,
    pub ctx: *mut SSL_CTX,
    pub next: *mut ctx_list,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct proxystate {
    pub ring_ssl2clear: ringbuffer,
    pub ring_clear2ssl: ringbuffer,
    pub ev_r_ssl: ev_io,
    pub ev_w_ssl: ev_io,
    pub ev_r_handshake: ev_io,
    pub ev_w_handshake: ev_io,
    pub ev_w_connect: ev_io,
    pub ev_r_clear: ev_io,
    pub ev_w_clear: ev_io,
    pub ev_proxy: ev_io,
    pub fd_up: libc::c_int,
    pub fd_down: libc::c_int,
    #[bitfield(name = "want_shutdown", ty = "libc::c_int", bits = "0..=0")]
    #[bitfield(name = "handshaked", ty = "libc::c_int", bits = "1..=1")]
    #[bitfield(name = "clear_connected", ty = "libc::c_int", bits = "2..=2")]
    #[bitfield(name = "renegotiation", ty = "libc::c_int", bits = "3..=3")]
    pub want_shutdown_handshaked_clear_connected_renegotiation: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub ssl: *mut SSL,
    pub remote_ip: sockaddr_storage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
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
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
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
pub type eb_troot_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eb_root {
    pub b: [*mut libc::c_void; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eb_node {
    pub branches: eb_root,
    pub node_p: *mut libc::c_void,
    pub leaf_p: *mut libc::c_void,
    pub bit: libc::c_short,
    pub pfx: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ebmb_node {
    pub node: eb_node,
    pub key: [libc::c_uchar; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shared_session {
    pub key: ebmb_node,
    pub key_data: [libc::c_uchar; 32],
    pub c_date: libc::c_long,
    pub data_len: libc::c_int,
    pub data: [libc::c_uchar; 512],
    pub p: *mut shared_session,
    pub n: *mut shared_session,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shared_context {
    pub mutex: pthread_mutex_t,
    pub active: shared_session,
    pub free: shared_session,
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
unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> libc::c_long {
    let mut tmp: libc::c_long = 0;
    tmp = strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
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
#[inline]
unsafe extern "C" fn ev_loop(mut loop___0: *mut ev_loop, mut flags: libc::c_int) {
    ev_run(loop___0, flags);
}
static mut loop_0: *mut ev_loop = 0 as *const ev_loop as *mut ev_loop;
static mut backaddr: *mut addrinfo = 0 as *const addrinfo as *mut addrinfo;
static mut master_pid: pid_t = 0;
static mut listener: ev_io = ev_io {
    active: 0,
    pending: 0,
    priority: 0,
    data: 0 as *const libc::c_void as *mut libc::c_void,
    cb: None,
    next: 0 as *const ev_watcher_list as *mut ev_watcher_list,
    fd: 0,
    events: 0,
};
static mut listener_socket: libc::c_int = 0;
static mut child_num: libc::c_int = 0;
static mut child_pids: *mut pid_t = 0 as *const pid_t as *mut pid_t;
static mut default_ctx: *mut SSL_CTX = 0 as *const SSL_CTX as *mut SSL_CTX;
static mut client_session: *mut SSL_SESSION = 0 as *const SSL_SESSION
    as *mut SSL_SESSION;
static mut shcupd_listener: ev_io = ev_io {
    active: 0,
    pending: 0,
    priority: 0,
    data: 0 as *const libc::c_void as *mut libc::c_void,
    cb: None,
    next: 0 as *const ev_watcher_list as *mut ev_watcher_list,
    fd: 0,
    events: 0,
};
static mut shcupd_socket: libc::c_int = 0;
pub static mut shcupd_peers: [*mut addrinfo; 16] = [0 as *const addrinfo
    as *mut addrinfo; 16];
static mut shared_secret: [libc::c_uchar; 20] = [0; 20];
pub static mut openssl_version: libc::c_long = 0;
pub static mut create_workers: libc::c_int = 0;
pub static mut CONFIG: *mut stud_config = 0 as *const stud_config as *mut stud_config;
static mut tcp_proxy_line: [libc::c_char; 128] = [
    '\u{0}' as i32 as libc::c_char,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
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
static mut sni_ctxs: *mut ctx_list = 0 as *const ctx_list as *mut ctx_list;
unsafe extern "C" fn setnonblocking(mut fd: libc::c_int) {
    let mut flag: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    flag = 1 as libc::c_int;
    tmp = ioctl(fd, 21537 as libc::c_ulong, &mut flag as *mut libc::c_int);
    if !(tmp == 0 as libc::c_int) {
        __assert_fail(
            b"ioctl(fd, FIONBIO, &flag) == 0\0" as *const u8 as *const libc::c_char,
            b"stud.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_uint,
            b"setnonblocking\0" as *const u8 as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn settcpkeepalive(mut fd: libc::c_int) {
    let mut optval: libc::c_int = 0;
    let mut optlen: socklen_t = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___7: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___8: libc::c_int = 0;
    optval = 1 as libc::c_int;
    optlen = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
    tmp___3 = setsockopt(
        fd,
        1 as libc::c_int,
        9 as libc::c_int,
        &mut optval as *mut libc::c_int as *const libc::c_void,
        optlen,
    );
    if tmp___3 < 0 as libc::c_int {
        tmp = __errno_location();
        tmp___0 = strerror(*tmp);
        fprintf(
            stderr,
            b"Error activating SO_KEEPALIVE on client socket: %s\0" as *const u8
                as *const libc::c_char,
            tmp___0,
        );
        if (*CONFIG).SYSLOG != 0 {
            tmp___1 = __errno_location();
            tmp___2 = strerror(*tmp___1);
            syslog(
                3 as libc::c_int,
                b"Error activating SO_KEEPALIVE on client socket: %s\0" as *const u8
                    as *const libc::c_char,
                tmp___2,
            );
        }
    }
    optval = (*CONFIG).TCP_KEEPALIVE_TIME;
    optlen = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
    tmp___8 = setsockopt(
        fd,
        6 as libc::c_int,
        4 as libc::c_int,
        &mut optval as *mut libc::c_int as *const libc::c_void,
        optlen,
    );
    if tmp___8 < 0 as libc::c_int {
        tmp___4 = __errno_location();
        tmp___5 = strerror(*tmp___4);
        fprintf(
            stderr,
            b"Error setting TCP_KEEPIDLE on client socket: %s\0" as *const u8
                as *const libc::c_char,
            tmp___5,
        );
        if (*CONFIG).SYSLOG != 0 {
            tmp___6 = __errno_location();
            tmp___7 = strerror(*tmp___6);
            syslog(
                3 as libc::c_int,
                b"Error setting TCP_KEEPIDLE on client socket: %s\0" as *const u8
                    as *const libc::c_char,
                tmp___7,
            );
        }
    }
}
unsafe extern "C" fn fail(mut s: *const libc::c_char) {
    perror(s);
    exit(1 as libc::c_int);
}
pub unsafe extern "C" fn die(mut fmt: *mut libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    vfprintf(stderr, fmt as *const libc::c_char, args_0.as_va_list());
    exit(1 as libc::c_int);
}
unsafe extern "C" fn init_dh(
    mut ctx: *mut SSL_CTX,
    mut cert: *const libc::c_char,
) -> libc::c_int {
    let mut dh: *mut DH = 0 as *mut DH;
    let mut bio: *mut BIO = 0 as *mut BIO;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut ecdh: *mut EC_KEY = 0 as *mut EC_KEY;
    if cert.is_null() {
        __assert_fail(
            b"cert\0" as *const u8 as *const libc::c_char,
            b"stud.c\0" as *const u8 as *const libc::c_char,
            234 as libc::c_uint,
            b"init_dh\0" as *const u8 as *const libc::c_char,
        );
    }
    bio = BIO_new_file(cert, b"r\0" as *const u8 as *const libc::c_char);
    if bio.is_null() {
        ERR_print_errors_fp(stderr);
        return -(1 as libc::c_int);
    }
    dh = PEM_read_bio_DHparams(
        bio,
        0 as *mut libc::c_void as *mut *mut DH,
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<pem_password_cb>,
        >(0 as *mut libc::c_void),
        0 as *mut libc::c_void,
    );
    BIO_free(bio);
    if dh.is_null() {
        fprintf(
            stderr,
            b"{core} Note: no DH parameters found in %s\n\0" as *const u8
                as *const libc::c_char,
            cert,
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"{core} Note: no DH parameters found in %s\n\0" as *const u8
                    as *const libc::c_char,
                cert,
            );
        }
        return -(1 as libc::c_int);
    }
    if (*CONFIG).QUIET == 0 {
        fprintf(
            stdout,
            b"{core} Using DH parameters from %s\n\0" as *const u8
                as *const libc::c_char,
            cert,
        );
    }
    if (*CONFIG).SYSLOG != 0 {
        syslog(
            6 as libc::c_int,
            b"{core} Using DH parameters from %s\n\0" as *const u8
                as *const libc::c_char,
            cert,
        );
    }
    SSL_CTX_ctrl(
        ctx,
        3 as libc::c_int,
        0 as libc::c_long,
        dh as *mut libc::c_char as *mut libc::c_void,
    );
    if (*CONFIG).QUIET == 0 {
        tmp = DH_size(dh as *const DH);
        fprintf(
            stdout,
            b"{core} DH initialized with %d bit key\n\0" as *const u8
                as *const libc::c_char,
            8 as libc::c_int * tmp,
        );
    }
    if (*CONFIG).SYSLOG != 0 {
        tmp___0 = DH_size(dh as *const DH);
        syslog(
            6 as libc::c_int,
            b"{core} DH initialized with %d bit key\n\0" as *const u8
                as *const libc::c_char,
            8 as libc::c_int * tmp___0,
        );
    }
    DH_free(dh);
    ecdh = 0 as *mut libc::c_void as *mut EC_KEY;
    ecdh = EC_KEY_new_by_curve_name(415 as libc::c_int);
    SSL_CTX_ctrl(
        ctx,
        4 as libc::c_int,
        0 as libc::c_long,
        ecdh as *mut libc::c_char as *mut libc::c_void,
    );
    EC_KEY_free(ecdh);
    if (*CONFIG).QUIET == 0 {
        fprintf(
            stdout,
            b"{core} ECDH Initialized with NIST P-256\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if (*CONFIG).SYSLOG != 0 {
        syslog(
            6 as libc::c_int,
            b"{core} ECDH Initialized with NIST P-256\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn info_callback(
    mut ssl___0: *const SSL,
    mut where_0: libc::c_int,
    mut ret: libc::c_int,
) {
    let mut ps: *mut proxystate = 0 as *mut proxystate;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    if where_0 & 16 as libc::c_int != 0 {
        tmp = SSL_get_ex_data(ssl___0, 0 as libc::c_int);
        ps = tmp as *mut proxystate;
        if (*ps).handshaked() != 0 {
            (*ps).set_renegotiation(1 as libc::c_int);
            if (*CONFIG).QUIET == 0 {
                fprintf(
                    stdout,
                    b"{core} SSL renegotiation asked by client\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if (*CONFIG).SYSLOG != 0 {
                syslog(
                    6 as libc::c_int,
                    b"{core} SSL renegotiation asked by client\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
    }
}
unsafe extern "C" fn handle_shcupd(
    mut loop___0: *mut ev_loop,
    mut w: *mut ev_io,
    mut revents: libc::c_int,
) {
    let mut msg: [libc::c_uchar; 612] = [0; 612];
    let mut hash: [libc::c_uchar; 64] = [0; 64];
    let mut r: ssize_t = 0;
    let mut hash_len: libc::c_uint = 0;
    let mut encdate: uint32_t = 0;
    let mut now: libc::c_long = 0;
    let mut tmp: ev_tstamp = 0.;
    let mut tmp___0: *const EVP_MD = 0 as *const EVP_MD;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: __uint32_t = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_long = 0;
    tmp = ev_now(loop___0);
    now = tmp as time_t;
    loop {
        r = recv(
            (*w).fd,
            msg.as_mut_ptr() as *mut libc::c_void,
            ::std::mem::size_of::<[libc::c_uchar; 612]>() as libc::c_ulong,
            0 as libc::c_int,
        );
        if !(r > 0 as libc::c_long) {
            break;
        }
        if r
            < (1 as libc::c_ulong)
                .wrapping_add(
                    ::std::mem::size_of::<[libc::c_uchar; 20]>() as libc::c_ulong,
                ) as libc::c_int as ssize_t
        {
            continue;
        }
        r = (r as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<[libc::c_uchar; 20]>() as libc::c_ulong)
            as ssize_t;
        tmp___0 = EVP_sha1();
        HMAC(
            tmp___0,
            shared_secret.as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_uchar; 20]>() as libc::c_ulong as libc::c_int,
            msg.as_mut_ptr() as *const libc::c_uchar,
            r as size_t,
            hash.as_mut_ptr(),
            &mut hash_len,
        );
        if hash_len as libc::c_ulong
            != ::std::mem::size_of::<[libc::c_uchar; 20]>() as libc::c_ulong
        {
            continue;
        }
        tmp___1 = memcmp(
            msg.as_mut_ptr().offset(r as isize) as *const libc::c_void,
            hash.as_mut_ptr() as *const libc::c_void,
            hash_len as size_t,
        );
        if tmp___1 != 0 {
            continue;
        }
        if r
            < (1 as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                as libc::c_int as ssize_t
        {
            continue;
        }
        r = (r as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<uint32_t>() as libc::c_ulong) as ssize_t;
        encdate = *(&mut *msg.as_mut_ptr().offset(r as isize) as *mut libc::c_uchar
            as *mut uint32_t);
        tmp___2 = __bswap_32(encdate);
        tmp___3 = abs(
            (now as int32_t as __uint32_t).wrapping_sub(tmp___2) as libc::c_int,
        );
        tmp___4 = SSL_CTX_get_timeout(default_ctx as *const SSL_CTX);
        if !((tmp___3 as libc::c_long) < tmp___4) {
            continue;
        }
        shctx_sess_add(msg.as_mut_ptr() as *const libc::c_uchar, r as libc::c_uint, now);
    };
}
pub unsafe extern "C" fn shcupd_session_new(
    mut msg: *mut libc::c_uchar,
    mut len: libc::c_uint,
    mut cdate: libc::c_long,
) {
    let mut hash_len: libc::c_uint = 0;
    let mut pai: *mut *mut addrinfo = 0 as *mut *mut addrinfo;
    let mut ncdate: uint32_t = 0;
    let mut tmp: *const EVP_MD = 0 as *const EVP_MD;
    pai = shcupd_peers.as_mut_ptr();
    ncdate = __bswap_32(cdate as uint32_t);
    memcpy(
        msg.offset(len as isize) as *mut libc::c_void,
        &mut ncdate as *mut uint32_t as *const libc::c_void,
        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
    );
    len = (len as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
        as libc::c_uint;
    tmp = EVP_sha1();
    HMAC(
        tmp,
        shared_secret.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; 20]>() as libc::c_ulong as libc::c_int,
        msg as *const libc::c_uchar,
        len as size_t,
        msg.offset(len as isize),
        &mut hash_len,
    );
    len = len.wrapping_add(hash_len);
    while !(*pai).is_null() {
        sendto(
            shcupd_socket,
            msg as *const libc::c_void,
            len as size_t,
            0 as libc::c_int,
            (**pai).ai_addr as *const sockaddr,
            (**pai).ai_addrlen,
        );
        pai = pai.offset(1);
    }
}
unsafe extern "C" fn compute_secret(
    mut rsa: *mut RSA,
    mut secret: *mut libc::c_uchar,
) -> libc::c_int {
    let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut length: libc::c_uint = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = i2d_RSAPrivateKey(
        rsa as *const RSA,
        0 as *mut libc::c_void as *mut *mut libc::c_uchar,
    );
    length = tmp as libc::c_uint;
    if length <= 0 as libc::c_uint {
        return -(1 as libc::c_int);
    }
    tmp___0 = malloc(
        (length as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
    );
    buf = tmp___0 as *mut libc::c_uchar;
    p = buf;
    if buf.is_null() {
        return -(1 as libc::c_int);
    }
    i2d_RSAPrivateKey(rsa as *const RSA, &mut p);
    SHA1(buf as *const libc::c_uchar, length as size_t, secret);
    free(buf as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn create_shcupd_socket() -> libc::c_int {
    let mut ai: *mut addrinfo = 0 as *mut addrinfo;
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
    let mut pai: *mut *mut addrinfo = 0 as *mut *mut addrinfo;
    let mut gai_err: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    let mut s: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut mreqn: ip_mreqn = ip_mreqn {
        imr_multiaddr: in_addr { s_addr: 0 },
        imr_address: in_addr { s_addr: 0 },
        imr_ifindex: 0,
    };
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
    let mut tmp___3: size_t = 0;
    let mut tmp___4: size_t = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___7: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut tmp___8: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut loop___0: libc::c_uchar = 0;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: libc::c_int = 0;
    let mut ttl: libc::c_uchar = 0;
    let mut tmp___12: libc::c_int = 0;
    let mut tmp___13: libc::c_int = 0;
    let mut mreq: ipv6_mreq = ipv6_mreq {
        ipv6mr_multiaddr: in6_addr {
            __in6_u: __anonunion___in6_u_979734923 {
                __u6_addr8: [0; 16],
            },
        },
        ipv6mr_interface: 0,
    };
    let mut ifr___0: ifreq = ifreq {
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
    let mut tmp___14: size_t = 0;
    let mut tmp___15: size_t = 0;
    let mut tmp___16: libc::c_int = 0;
    let mut tmp___17: libc::c_int = 0;
    let mut tmp___18: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut tmp___19: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut loop___1: libc::c_uint = 0;
    let mut tmp___20: libc::c_int = 0;
    let mut tmp___21: libc::c_int = 0;
    let mut tmp___22: libc::c_int = 0;
    let mut hops: libc::c_int = 0;
    let mut tmp___23: libc::c_int = 0;
    let mut tmp___24: libc::c_int = 0;
    pai = shcupd_peers.as_mut_ptr();
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_family = 0 as libc::c_int;
    hints.ai_socktype = 2 as libc::c_int;
    hints.ai_flags = 33 as libc::c_int;
    tmp = getaddrinfo(
        (*CONFIG).SHCUPD_IP as *const libc::c_char,
        (*CONFIG).SHCUPD_PORT as *const libc::c_char,
        &mut hints as *mut addrinfo as *const addrinfo,
        &mut ai as *mut *mut addrinfo,
    );
    gai_err = tmp;
    if gai_err != 0 as libc::c_int {
        tmp___0 = gai_strerror(gai_err);
        fprintf(
            stderr,
            b"{getaddrinfo}: [%s]\n\0" as *const u8 as *const libc::c_char,
            tmp___0,
        );
        if (*CONFIG).SYSLOG != 0 {
            tmp___1 = gai_strerror(gai_err);
            syslog(
                3 as libc::c_int,
                b"{getaddrinfo}: [%s]\n\0" as *const u8 as *const libc::c_char,
                tmp___1,
            );
        }
        exit(1 as libc::c_int);
    }
    while !(*pai).is_null() {
        if (**pai).ai_family != (*ai).ai_family {
            fprintf(
                stderr,
                b"Share host and peers inet family differs\n\0" as *const u8
                    as *const libc::c_char,
            );
            if (*CONFIG).SYSLOG != 0 {
                syslog(
                    3 as libc::c_int,
                    b"Share host and peers inet family differs\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            exit(1 as libc::c_int);
        }
        pai = pai.offset(1);
    }
    tmp___2 = socket((*ai).ai_family, 2 as libc::c_int, 17 as libc::c_int);
    s = tmp___2;
    if s == -(1 as libc::c_int) {
        fail(b"{socket: shared cache updates}\0" as *const u8 as *const libc::c_char);
    }
    t = 1 as libc::c_int;
    setsockopt(
        s,
        1 as libc::c_int,
        2 as libc::c_int,
        &mut t as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    setsockopt(
        s,
        1 as libc::c_int,
        15 as libc::c_int,
        &mut t as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    setnonblocking(s);
    if (*(*ai).ai_addr).sa_family as libc::c_int == 2 as libc::c_int {
        memset(
            &mut mreqn as *mut ip_mreqn as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<ip_mreqn>() as libc::c_ulong,
        );
        mreqn
            .imr_multiaddr
            .s_addr = (*((*ai).ai_addr as *mut sockaddr_in)).sin_addr.s_addr;
        if !((*CONFIG).SHCUPD_MCASTIF).is_null() {
            tmp___7 = __ctype_b_loc();
            if *(*tmp___7).offset(*(*CONFIG).SHCUPD_MCASTIF as libc::c_int as isize)
                as libc::c_int & 1024 as libc::c_int != 0
            {
                memset(
                    &mut ifr as *mut ifreq as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<ifreq>() as libc::c_ulong,
                );
                tmp___3 = strlen((*CONFIG).SHCUPD_MCASTIF as *const libc::c_char);
                if tmp___3 > 16 as libc::c_ulong {
                    fprintf(
                        stderr,
                        b"Error iface name is too long [%s]\n\0" as *const u8
                            as *const libc::c_char,
                        (*CONFIG).SHCUPD_MCASTIF,
                    );
                    if (*CONFIG).SYSLOG != 0 {
                        syslog(
                            3 as libc::c_int,
                            b"Error iface name is too long [%s]\n\0" as *const u8
                                as *const libc::c_char,
                            (*CONFIG).SHCUPD_MCASTIF,
                        );
                    }
                    exit(1 as libc::c_int);
                }
                tmp___4 = strlen((*CONFIG).SHCUPD_MCASTIF as *const libc::c_char);
                memcpy(
                    (ifr.ifr_ifrn.ifrn_name).as_mut_ptr() as *mut libc::c_void,
                    (*CONFIG).SHCUPD_MCASTIF as *const libc::c_void,
                    tmp___4,
                );
                tmp___5 = ioctl(s, 35123 as libc::c_ulong, &mut ifr as *mut ifreq);
                if tmp___5 != 0 {
                    fail(b"{ioctl: SIOCGIFINDEX}\0" as *const u8 as *const libc::c_char);
                }
                mreqn.imr_ifindex = ifr.ifr_ifru.ifru_ivalue;
            } else {
                tmp___6 = strchr(
                    (*CONFIG).SHCUPD_MCASTIF as *const libc::c_char,
                    '.' as i32,
                );
                if !tmp___6.is_null() {
                    mreqn
                        .imr_address
                        .s_addr = inet_addr(
                        (*CONFIG).SHCUPD_MCASTIF as *const libc::c_char,
                    );
                } else {
                    mreqn
                        .imr_ifindex = atoi(
                        (*CONFIG).SHCUPD_MCASTIF as *const libc::c_char,
                    );
                }
            }
        }
        tmp___10 = setsockopt(
            s,
            0 as libc::c_int,
            35 as libc::c_int,
            &mut mreqn as *mut ip_mreqn as *const libc::c_void,
            ::std::mem::size_of::<ip_mreqn>() as libc::c_ulong as socklen_t,
        );
        if tmp___10 < 0 as libc::c_int {
            tmp___8 = __errno_location();
            if *tmp___8 != 22 as libc::c_int {
                fail(
                    b"{setsockopt: IP_ADD_MEMBERSIP}\0" as *const u8
                        as *const libc::c_char,
                );
            }
        } else {
            loop___0 = 0 as libc::c_int as libc::c_uchar;
            tmp___9 = setsockopt(
                s,
                0 as libc::c_int,
                34 as libc::c_int,
                &mut loop___0 as *mut libc::c_uchar as *const libc::c_void,
                ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong as socklen_t,
            );
            if tmp___9 < 0 as libc::c_int {
                fail(
                    b"{setsockopt: IP_MULTICAST_LOOP}\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        if !((*CONFIG).SHCUPD_MCASTIF).is_null() {
            tmp___11 = setsockopt(
                s,
                0 as libc::c_int,
                32 as libc::c_int,
                &mut mreqn as *mut ip_mreqn as *const libc::c_void,
                ::std::mem::size_of::<ip_mreqn>() as libc::c_ulong as socklen_t,
            );
            if tmp___11 < 0 as libc::c_int {
                fail(
                    b"{setsockopt: IP_MULTICAST_IF}\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        if !((*CONFIG).SHCUPD_MCASTTTL).is_null() {
            tmp___12 = atoi((*CONFIG).SHCUPD_MCASTTTL as *const libc::c_char);
            ttl = tmp___12 as libc::c_uchar;
            tmp___13 = setsockopt(
                s,
                0 as libc::c_int,
                33 as libc::c_int,
                &mut ttl as *mut libc::c_uchar as *const libc::c_void,
                ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong as socklen_t,
            );
            if tmp___13 < 0 as libc::c_int {
                fail(
                    b"{setsockopt: IP_MULTICAST_TTL}\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
    } else if (*(*ai).ai_addr).sa_family as libc::c_int == 10 as libc::c_int {
        memset(
            &mut mreq as *mut ipv6_mreq as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<ipv6_mreq>() as libc::c_ulong,
        );
        memcpy(
            &mut mreq.ipv6mr_multiaddr as *mut in6_addr as *mut libc::c_void,
            &mut (*((*ai).ai_addr as *mut sockaddr_in6)).sin6_addr as *mut in6_addr
                as *const libc::c_void,
            ::std::mem::size_of::<in6_addr>() as libc::c_ulong,
        );
        if !((*CONFIG).SHCUPD_MCASTIF).is_null() {
            tmp___18 = __ctype_b_loc();
            if *(*tmp___18).offset(*(*CONFIG).SHCUPD_MCASTIF as libc::c_int as isize)
                as libc::c_int & 1024 as libc::c_int != 0
            {
                memset(
                    &mut ifr___0 as *mut ifreq as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<ifreq>() as libc::c_ulong,
                );
                tmp___14 = strlen((*CONFIG).SHCUPD_MCASTIF as *const libc::c_char);
                if tmp___14 > 16 as libc::c_ulong {
                    fprintf(
                        stderr,
                        b"Error iface name is too long [%s]\n\0" as *const u8
                            as *const libc::c_char,
                        (*CONFIG).SHCUPD_MCASTIF,
                    );
                    if (*CONFIG).SYSLOG != 0 {
                        syslog(
                            3 as libc::c_int,
                            b"Error iface name is too long [%s]\n\0" as *const u8
                                as *const libc::c_char,
                            (*CONFIG).SHCUPD_MCASTIF,
                        );
                    }
                    exit(1 as libc::c_int);
                }
                tmp___15 = strlen((*CONFIG).SHCUPD_MCASTIF as *const libc::c_char);
                memcpy(
                    (ifr___0.ifr_ifrn.ifrn_name).as_mut_ptr() as *mut libc::c_void,
                    (*CONFIG).SHCUPD_MCASTIF as *const libc::c_void,
                    tmp___15,
                );
                tmp___16 = ioctl(s, 35123 as libc::c_ulong, &mut ifr___0 as *mut ifreq);
                if tmp___16 != 0 {
                    fail(b"{ioctl: SIOCGIFINDEX}\0" as *const u8 as *const libc::c_char);
                }
                mreq.ipv6mr_interface = ifr___0.ifr_ifru.ifru_ivalue as libc::c_uint;
            } else {
                tmp___17 = atoi((*CONFIG).SHCUPD_MCASTIF as *const libc::c_char);
                mreq.ipv6mr_interface = tmp___17 as libc::c_uint;
            }
        }
        tmp___21 = setsockopt(
            s,
            41 as libc::c_int,
            20 as libc::c_int,
            &mut mreq as *mut ipv6_mreq as *const libc::c_void,
            ::std::mem::size_of::<ipv6_mreq>() as libc::c_ulong as socklen_t,
        );
        if tmp___21 < 0 as libc::c_int {
            tmp___19 = __errno_location();
            if *tmp___19 != 22 as libc::c_int {
                fail(
                    b"{setsockopt: IPV6_ADD_MEMBERSIP}\0" as *const u8
                        as *const libc::c_char,
                );
            }
        } else {
            loop___1 = 0 as libc::c_uint;
            tmp___20 = setsockopt(
                s,
                41 as libc::c_int,
                19 as libc::c_int,
                &mut loop___1 as *mut libc::c_uint as *const libc::c_void,
                ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong as socklen_t,
            );
            if tmp___20 < 0 as libc::c_int {
                fail(
                    b"{setsockopt: IPV6_MULTICAST_LOOP}\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        tmp___22 = setsockopt(
            s,
            41 as libc::c_int,
            17 as libc::c_int,
            &mut mreq.ipv6mr_interface as *mut libc::c_uint as *const libc::c_void,
            ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong as socklen_t,
        );
        if tmp___22 < 0 as libc::c_int {
            fail(
                b"{setsockopt: IPV6_MULTICAST_IF}\0" as *const u8 as *const libc::c_char,
            );
        }
        if !((*CONFIG).SHCUPD_MCASTTTL).is_null() {
            hops = atoi((*CONFIG).SHCUPD_MCASTTTL as *const libc::c_char);
            tmp___23 = setsockopt(
                s,
                41 as libc::c_int,
                18 as libc::c_int,
                &mut hops as *mut libc::c_int as *const libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
            );
            if tmp___23 < 0 as libc::c_int {
                fail(
                    b"{setsockopt: IPV6_MULTICAST_HOPS}\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
    }
    tmp___24 = bind(s, (*ai).ai_addr as *const sockaddr, (*ai).ai_addrlen);
    if tmp___24 != 0 {
        fail(b"{bind-socket}\0" as *const u8 as *const libc::c_char);
    }
    freeaddrinfo(ai);
    return s;
}
pub unsafe extern "C" fn load_rsa_privatekey(
    mut ctx: *mut SSL_CTX,
    mut file: *const libc::c_char,
) -> *mut RSA {
    let mut bio: *mut BIO = 0 as *mut BIO;
    let mut rsa: *mut RSA = 0 as *mut RSA;
    bio = BIO_new_file(file, b"r\0" as *const u8 as *const libc::c_char);
    if bio.is_null() {
        ERR_print_errors_fp(stderr);
        return 0 as *mut libc::c_void as *mut RSA;
    }
    rsa = PEM_read_bio_RSAPrivateKey(
        bio,
        0 as *mut libc::c_void as *mut *mut RSA,
        (*ctx).default_passwd_callback,
        (*ctx).default_passwd_callback_userdata,
    );
    BIO_free(bio);
    return rsa;
}
pub unsafe extern "C" fn sni_switch_ctx(
    mut ssl___0: *mut SSL,
    mut al: *mut libc::c_int,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut servername: *const libc::c_char = 0 as *const libc::c_char;
    let mut cl: *const ctx_list = 0 as *const ctx_list;
    let mut tmp: libc::c_int = 0;
    servername = SSL_get_servername(ssl___0 as *const SSL, 0 as libc::c_int);
    if servername.is_null() {
        return 3 as libc::c_int;
    }
    cl = sni_ctxs as *const ctx_list;
    while cl as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp = strcasecmp(servername, (*cl).servername as *const libc::c_char);
        if tmp == 0 as libc::c_int {
            SSL_set_SSL_CTX(ssl___0, (*cl).ctx);
            return 3 as libc::c_int;
        }
        cl = (*cl).next as *const ctx_list;
    }
    return 3 as libc::c_int;
}
pub unsafe extern "C" fn make_ctx(mut pemfile: *const libc::c_char) -> *mut SSL_CTX {
    let mut ctx: *mut SSL_CTX = 0 as *mut SSL_CTX;
    let mut rsa: *mut RSA = 0 as *mut RSA;
    let mut ssloptions: libc::c_long = 0;
    let mut tmp: *const SSL_METHOD = 0 as *const SSL_METHOD;
    let mut tmp___0: *const SSL_METHOD = 0 as *const SSL_METHOD;
    let mut tmp___1: *const SSL_METHOD = 0 as *const SSL_METHOD;
    let mut tmp___2: *const SSL_METHOD = 0 as *const SSL_METHOD;
    let mut tmp___3: *const SSL_METHOD = 0 as *const SSL_METHOD;
    let mut tmp___4: *const SSL_METHOD = 0 as *const SSL_METHOD;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_long = 0;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    ssloptions = 2164329471 as libc::c_long;
    ssloptions |= 131072 as libc::c_long;
    if (*CONFIG).ETYPE as libc::c_uint == 0 as libc::c_uint {
        if (*CONFIG).PMODE as libc::c_uint == 1 as libc::c_uint {
            tmp = TLSv1_client_method();
            tmp___1 = tmp;
        } else {
            tmp___0 = TLSv1_server_method();
            tmp___1 = tmp___0;
        }
        ctx = SSL_CTX_new(tmp___1);
    } else if (*CONFIG).ETYPE as libc::c_uint == 1 as libc::c_uint {
        if (*CONFIG).PMODE as libc::c_uint == 1 as libc::c_uint {
            tmp___2 = SSLv23_client_method();
            tmp___4 = tmp___2;
        } else {
            tmp___3 = SSLv23_server_method();
            tmp___4 = tmp___3;
        }
        ctx = SSL_CTX_new(tmp___4);
    } else {
        if !((*CONFIG).ETYPE as libc::c_uint == 0 as libc::c_uint) {
            if !((*CONFIG).ETYPE as libc::c_uint == 1 as libc::c_uint) {
                __assert_fail(
                    b"CONFIG->ETYPE == ENC_TLS || CONFIG->ETYPE == ENC_SSL\0"
                        as *const u8 as *const libc::c_char,
                    b"stud.c\0" as *const u8 as *const libc::c_char,
                    607 as libc::c_uint,
                    b"make_ctx\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        return 0 as *mut libc::c_void as *mut SSL_CTX;
    }
    SSL_CTX_ctrl(ctx, 32 as libc::c_int, ssloptions, 0 as *mut libc::c_void);
    SSL_CTX_set_info_callback(
        ctx,
        Some(
            info_callback
                as unsafe extern "C" fn(*const SSL, libc::c_int, libc::c_int) -> (),
        ),
    );
    if !((*CONFIG).CIPHER_SUITE).is_null() {
        tmp___5 = SSL_CTX_set_cipher_list(
            ctx,
            (*CONFIG).CIPHER_SUITE as *const libc::c_char,
        );
        if tmp___5 != 1 as libc::c_int {
            ERR_print_errors_fp(stderr);
        }
    }
    if (*CONFIG).PREFER_SERVER_CIPHERS != 0 {
        SSL_CTX_ctrl(
            ctx,
            32 as libc::c_int,
            4194304 as libc::c_long,
            0 as *mut libc::c_void,
        );
    }
    if (*CONFIG).PMODE as libc::c_uint == 1 as libc::c_uint {
        return ctx;
    }
    tmp___6 = SSL_CTX_use_certificate_chain_file(ctx, pemfile);
    if tmp___6 <= 0 as libc::c_int {
        ERR_print_errors_fp(stderr);
        exit(1 as libc::c_int);
    }
    rsa = load_rsa_privatekey(ctx, pemfile);
    if rsa.is_null() {
        fprintf(
            stderr,
            b"Error loading rsa private key\n\0" as *const u8 as *const libc::c_char,
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"Error loading rsa private key\n\0" as *const u8 as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    }
    tmp___7 = SSL_CTX_use_RSAPrivateKey(ctx, rsa);
    if tmp___7 <= 0 as libc::c_int {
        ERR_print_errors_fp(stderr);
        exit(1 as libc::c_int);
    }
    init_dh(ctx, pemfile);
    tmp___8 = SSL_CTX_callback_ctrl(
        ctx,
        53 as libc::c_int,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut SSL,
                    *mut libc::c_int,
                    *mut libc::c_void,
                ) -> libc::c_int,
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                sni_switch_ctx
                    as unsafe extern "C" fn(
                        *mut SSL,
                        *mut libc::c_int,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ),
    );
    if tmp___8 == 0 {
        fprintf(
            stderr,
            b"Error setting up SNI support\n\0" as *const u8 as *const libc::c_char,
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"Error setting up SNI support\n\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if (*CONFIG).SHARED_CACHE != 0 {
        tmp___9 = shared_context_init(ctx, (*CONFIG).SHARED_CACHE);
        if tmp___9 < 0 as libc::c_int {
            fprintf(
                stderr,
                b"Unable to alloc memory for shared cache.\n\0" as *const u8
                    as *const libc::c_char,
            );
            if (*CONFIG).SYSLOG != 0 {
                syslog(
                    3 as libc::c_int,
                    b"Unable to alloc memory for shared cache.\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            exit(1 as libc::c_int);
        }
        if !((*CONFIG).SHCUPD_PORT).is_null() {
            tmp___10 = compute_secret(rsa, shared_secret.as_mut_ptr());
            if tmp___10 < 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"Unable to compute shared secret.\n\0" as *const u8
                        as *const libc::c_char,
                );
                if (*CONFIG).SYSLOG != 0 {
                    syslog(
                        3 as libc::c_int,
                        b"Unable to compute shared secret.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                exit(1 as libc::c_int);
            }
            SSL_CTX_ctrl(
                ctx,
                32 as libc::c_int,
                16384 as libc::c_long,
                0 as *mut libc::c_void,
            );
            if !(shcupd_peers[0 as libc::c_int as usize]).is_null() {
                shsess_set_new_cbk(
                    Some(
                        shcupd_session_new
                            as unsafe extern "C" fn(
                                *mut libc::c_uchar,
                                libc::c_uint,
                                libc::c_long,
                            ) -> (),
                    ),
                );
            }
        }
    }
    RSA_free(rsa);
    return ctx;
}
pub unsafe extern "C" fn init_openssl() -> *mut SSL_CTX {
    let mut cf: *mut cert_files = 0 as *mut cert_files;
    let mut i: libc::c_int = 0;
    let mut ctx: *mut SSL_CTX = 0 as *mut SSL_CTX;
    let mut x509: *mut X509 = 0 as *mut X509;
    let mut f: *mut BIO = 0 as *mut BIO;
    let mut names: *mut stack_st_GENERAL_NAME = 0 as *mut stack_st_GENERAL_NAME;
    let mut name: *mut GENERAL_NAME = 0 as *mut GENERAL_NAME;
    let mut tmp: *mut BIO_METHOD = 0 as *mut BIO_METHOD;
    let mut tmp___0: libc::c_long = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut cl: *mut ctx_list = 0 as *mut ctx_list;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut x509_name: *mut X509_NAME = 0 as *mut X509_NAME;
    let mut tmp___6: *mut X509_NAME = 0 as *mut X509_NAME;
    let mut x509_entry: *mut X509_NAME_ENTRY = 0 as *mut X509_NAME_ENTRY;
    let mut tmp___7: *mut X509_NAME_ENTRY = 0 as *mut X509_NAME_ENTRY;
    let mut cl___0: *mut ctx_list = 0 as *mut ctx_list;
    let mut tmp___8: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut e: *mut ENGINE = 0 as *mut ENGINE;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___12: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___13: libc::c_int = 0;
    SSL_library_init();
    SSL_load_error_strings();
    if !((*CONFIG).CERT_FILES as libc::c_ulong
        != 0 as *mut libc::c_void as libc::c_ulong)
    {
        __assert_fail(
            b"CONFIG->CERT_FILES != NULL\0" as *const u8 as *const libc::c_char,
            b"stud.c\0" as *const u8 as *const libc::c_char,
            688 as libc::c_uint,
            b"init_openssl\0" as *const u8 as *const libc::c_char,
        );
    }
    default_ctx = make_ctx((*(*CONFIG).CERT_FILES).CERT_FILE as *const libc::c_char);
    names = 0 as *mut libc::c_void as *mut stack_st_GENERAL_NAME;
    cf = (*(*CONFIG).CERT_FILES).NEXT;
    while cf as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        ctx = make_ctx((*cf).CERT_FILE as *const libc::c_char);
        tmp = BIO_s_file();
        f = BIO_new(tmp);
        tmp___0 = BIO_ctrl(
            f,
            108 as libc::c_int,
            3 as libc::c_long,
            (*cf).CERT_FILE as *mut libc::c_void,
        );
        if tmp___0 == 0 {
            fprintf(
                stderr,
                b"Could not read cert '%s'\n\0" as *const u8 as *const libc::c_char,
                (*cf).CERT_FILE,
            );
            if (*CONFIG).SYSLOG != 0 {
                syslog(
                    3 as libc::c_int,
                    b"Could not read cert '%s'\n\0" as *const u8 as *const libc::c_char,
                    (*cf).CERT_FILE,
                );
            }
        }
        x509 = PEM_read_bio_X509_AUX(
            f,
            0 as *mut libc::c_void as *mut *mut X509,
            ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<pem_password_cb>,
            >(0 as *mut libc::c_void),
            0 as *mut libc::c_void,
        );
        BIO_free(f);
        tmp___1 = X509_get_ext_d2i(
            x509,
            85 as libc::c_int,
            0 as *mut libc::c_void as *mut libc::c_int,
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        names = tmp___1 as *mut stack_st_GENERAL_NAME;
        i = 0 as libc::c_int;
        loop {
            tmp___4 = sk_num(names as *mut _STACK as *const _STACK);
            if !(i < tmp___4) {
                break;
            }
            tmp___2 = sk_value(names as *mut _STACK as *const _STACK, i);
            name = tmp___2 as *mut GENERAL_NAME;
            if (*name).type_0 == 2 as libc::c_int {
                tmp___3 = calloc(
                    1 as libc::c_int as size_t,
                    ::std::mem::size_of::<ctx_list>() as libc::c_ulong,
                );
                cl = tmp___3 as *mut ctx_list;
                ASN1_STRING_to_UTF8(
                    &mut (*cl).servername as *mut *mut libc::c_char
                        as *mut *mut libc::c_uchar,
                    (*name).d.dNSName,
                );
                (*cl).ctx = ctx;
                (*cl).next = sni_ctxs;
                sni_ctxs = cl;
            }
            i += 1;
        }
        tmp___5 = sk_num(names as *mut _STACK as *const _STACK);
        if tmp___5 > 0 as libc::c_int {
            sk_pop_free(
                names as *mut _STACK,
                ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut GENERAL_NAME) -> ()>,
                    Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
                >(
                    Some(
                        GENERAL_NAME_free
                            as unsafe extern "C" fn(*mut GENERAL_NAME) -> (),
                    ),
                ),
            );
        } else {
            if names as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                sk_pop_free(
                    names as *mut _STACK,
                    ::std::mem::transmute::<
                        Option::<unsafe extern "C" fn(*mut GENERAL_NAME) -> ()>,
                        Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
                    >(
                        Some(
                            GENERAL_NAME_free
                                as unsafe extern "C" fn(*mut GENERAL_NAME) -> (),
                        ),
                    ),
                );
            }
            tmp___6 = X509_get_subject_name(x509);
            x509_name = tmp___6;
            i = X509_NAME_get_index_by_NID(
                x509_name,
                13 as libc::c_int,
                -(1 as libc::c_int),
            );
            if i < 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"Could not find Subject Alternative Names or a CN on cert %s\n\0"
                        as *const u8 as *const libc::c_char,
                    (*cf).CERT_FILE,
                );
                if (*CONFIG).SYSLOG != 0 {
                    syslog(
                        3 as libc::c_int,
                        b"Could not find Subject Alternative Names or a CN on cert %s\n\0"
                            as *const u8 as *const libc::c_char,
                        (*cf).CERT_FILE,
                    );
                }
            }
            tmp___7 = X509_NAME_get_entry(x509_name, i);
            x509_entry = tmp___7;
            tmp___8 = calloc(
                1 as libc::c_int as size_t,
                ::std::mem::size_of::<ctx_list>() as libc::c_ulong,
            );
            cl___0 = tmp___8 as *mut ctx_list;
            ASN1_STRING_to_UTF8(
                &mut (*cl___0).servername as *mut *mut libc::c_char
                    as *mut *mut libc::c_uchar,
                (*x509_entry).value,
            );
            (*cl___0).ctx = ctx;
            (*cl___0).next = sni_ctxs;
            sni_ctxs = cl___0;
        }
        cf = (*cf).NEXT;
    }
    if !((*CONFIG).ENGINE).is_null() {
        e = 0 as *mut libc::c_void as *mut ENGINE;
        ENGINE_load_builtin_engines();
        tmp___13 = strcmp(
            (*CONFIG).ENGINE as *const libc::c_char,
            b"auto\0" as *const u8 as *const libc::c_char,
        );
        if tmp___13 != 0 {
            e = ENGINE_by_id((*CONFIG).ENGINE as *const libc::c_char);
            if e as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                ERR_print_errors_fp(stderr);
                exit(1 as libc::c_int);
            } else {
                tmp___9 = ENGINE_init(e);
                if tmp___9 != 0 {
                    tmp___10 = ENGINE_set_default(e, 65535 as libc::c_uint);
                    if tmp___10 == 0 {
                        ERR_print_errors_fp(stderr);
                        exit(1 as libc::c_int);
                    }
                } else {
                    ERR_print_errors_fp(stderr);
                    exit(1 as libc::c_int);
                }
            }
            if (*CONFIG).QUIET == 0 {
                tmp___11 = ENGINE_get_id(e as *const ENGINE);
                fprintf(
                    stdout,
                    b"{core} will use OpenSSL engine %s.\n\0" as *const u8
                        as *const libc::c_char,
                    tmp___11,
                );
            }
            if (*CONFIG).SYSLOG != 0 {
                tmp___12 = ENGINE_get_id(e as *const ENGINE);
                syslog(
                    6 as libc::c_int,
                    b"{core} will use OpenSSL engine %s.\n\0" as *const u8
                        as *const libc::c_char,
                    tmp___12,
                );
            }
            ENGINE_finish(e);
            ENGINE_free(e);
        } else {
            ENGINE_register_all_complete();
        }
    }
    return 0 as *mut SSL_CTX;
}
unsafe extern "C" fn prepare_proxy_line(mut ai_addr: *mut sockaddr) {
    let mut tcp6_address_string: [libc::c_char; 46] = [0; 46];
    let mut addr: *mut sockaddr_in = 0 as *mut sockaddr_in;
    let mut res: size_t = 0;
    let mut tmp: __uint16_t = 0;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    let mut addr___0: *mut sockaddr_in6 = 0 as *mut sockaddr_in6;
    let mut res___0: size_t = 0;
    let mut tmp___2: __uint16_t = 0;
    let mut tmp___3: libc::c_int = 0;
    tcp_proxy_line[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    if (*ai_addr).sa_family as libc::c_int == 2 as libc::c_int {
        addr = ai_addr as *mut sockaddr_in;
        tmp = __bswap_16((*addr).sin_port);
        tmp___0 = inet_ntoa((*addr).sin_addr);
        tmp___1 = snprintf(
            tcp_proxy_line.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"PROXY %%s %%s %s %%hu %hu\r\n\0" as *const u8 as *const libc::c_char,
            tmp___0,
            tmp as libc::c_int,
        );
        res = tmp___1 as size_t;
        if !(res < ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong) {
            __assert_fail(
                b"res < sizeof(tcp_proxy_line)\0" as *const u8 as *const libc::c_char,
                b"stud.c\0" as *const u8 as *const libc::c_char,
                788 as libc::c_uint,
                b"prepare_proxy_line\0" as *const u8 as *const libc::c_char,
            );
        }
    } else if (*ai_addr).sa_family as libc::c_int == 10 as libc::c_int {
        addr___0 = ai_addr as *mut sockaddr_in6;
        inet_ntop(
            10 as libc::c_int,
            &mut (*addr___0).sin6_addr as *mut in6_addr as *const libc::c_void,
            tcp6_address_string.as_mut_ptr(),
            46 as libc::c_int as socklen_t,
        );
        tmp___2 = __bswap_16((*addr___0).sin6_port);
        tmp___3 = snprintf(
            tcp_proxy_line.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"PROXY %%s %%s %s %%hu %hu\r\n\0" as *const u8 as *const libc::c_char,
            tcp6_address_string.as_mut_ptr(),
            tmp___2 as libc::c_int,
        );
        res___0 = tmp___3 as size_t;
        if !(res___0 < ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong) {
            __assert_fail(
                b"res < sizeof(tcp_proxy_line)\0" as *const u8 as *const libc::c_char,
                b"stud.c\0" as *const u8 as *const libc::c_char,
                798 as libc::c_uint,
                b"prepare_proxy_line\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        fprintf(
            stderr,
            b"The --write-proxy mode is not implemented for this address family.\n\0"
                as *const u8 as *const libc::c_char,
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"The --write-proxy mode is not implemented for this address family.\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    };
}
unsafe extern "C" fn create_main_socket() -> libc::c_int {
    let mut ai: *mut addrinfo = 0 as *mut addrinfo;
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
    let mut gai_err: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    let mut s: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut timeout: libc::c_int = 0;
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_family = 0 as libc::c_int;
    hints.ai_socktype = 1 as libc::c_int;
    hints.ai_flags = 33 as libc::c_int;
    tmp = getaddrinfo(
        (*CONFIG).FRONT_IP as *const libc::c_char,
        (*CONFIG).FRONT_PORT as *const libc::c_char,
        &mut hints as *mut addrinfo as *const addrinfo,
        &mut ai as *mut *mut addrinfo,
    );
    gai_err = tmp;
    if gai_err != 0 as libc::c_int {
        tmp___0 = gai_strerror(gai_err);
        fprintf(
            stderr,
            b"{getaddrinfo}: [%s]\n\0" as *const u8 as *const libc::c_char,
            tmp___0,
        );
        if (*CONFIG).SYSLOG != 0 {
            tmp___1 = gai_strerror(gai_err);
            syslog(
                3 as libc::c_int,
                b"{getaddrinfo}: [%s]\n\0" as *const u8 as *const libc::c_char,
                tmp___1,
            );
        }
        exit(1 as libc::c_int);
    }
    tmp___2 = socket((*ai).ai_family, 1 as libc::c_int, 6 as libc::c_int);
    s = tmp___2;
    if s == -(1 as libc::c_int) {
        fail(b"{socket: main}\0" as *const u8 as *const libc::c_char);
    }
    t = 1 as libc::c_int;
    setsockopt(
        s,
        1 as libc::c_int,
        2 as libc::c_int,
        &mut t as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    setsockopt(
        s,
        1 as libc::c_int,
        15 as libc::c_int,
        &mut t as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    setnonblocking(s);
    tmp___3 = bind(s, (*ai).ai_addr as *const sockaddr, (*ai).ai_addrlen);
    if tmp___3 != 0 {
        fail(b"{bind-socket}\0" as *const u8 as *const libc::c_char);
    }
    timeout = 1 as libc::c_int;
    setsockopt(
        s,
        6 as libc::c_int,
        9 as libc::c_int,
        &mut timeout as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    prepare_proxy_line((*ai).ai_addr);
    freeaddrinfo(ai);
    listen(s, (*CONFIG).BACKLOG);
    return s;
}
unsafe extern "C" fn create_back_socket() -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = socket((*backaddr).ai_family, 1 as libc::c_int, 6 as libc::c_int);
    s = tmp;
    if s == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    flag = 1 as libc::c_int;
    tmp___0 = setsockopt(
        s,
        6 as libc::c_int,
        1 as libc::c_int,
        &mut flag as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    ret = tmp___0;
    if ret == -(1 as libc::c_int) {
        perror(
            b"Couldn't setsockopt to backend (TCP_NODELAY)\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    setnonblocking(s);
    return s;
}
unsafe extern "C" fn safe_enable_io(mut ps: *mut proxystate, mut w: *mut ev_io) {
    if (*ps).want_shutdown() == 0 {
        ev_io_start(loop_0, w);
    }
}
unsafe extern "C" fn shutdown_proxy(
    mut ps: *mut proxystate,
    mut req: SHUTDOWN_REQUESTOR,
) {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut current_block_24: u64;
    if (*ps).want_shutdown() != 0 {
        current_block_24 = 13362358186534565311;
    } else if req as libc::c_uint == 0 as libc::c_uint {
        current_block_24 = 13362358186534565311;
    } else {
        (*ps).set_want_shutdown(1 as libc::c_int);
        let mut current_block_22: u64;
        if req as libc::c_uint == 1 as libc::c_uint {
            tmp___0 = ringbuffer_is_empty(&mut (*ps).ring_clear2ssl);
            if tmp___0 != 0 {
                shutdown_proxy(ps, SHUTDOWN_HARD);
                current_block_22 = 18317007320854588510;
            } else {
                current_block_22 = 9975064640536201261;
            }
        } else {
            current_block_22 = 9975064640536201261;
        }
        match current_block_22 {
            9975064640536201261 => {
                if req as libc::c_uint == 2 as libc::c_uint {
                    tmp = ringbuffer_is_empty(&mut (*ps).ring_ssl2clear);
                    if tmp != 0 {
                        shutdown_proxy(ps, SHUTDOWN_HARD);
                    }
                }
            }
            _ => {}
        }
        current_block_24 = 11194104282611034094;
    }
    match current_block_24 {
        13362358186534565311 => {
            ev_io_stop(loop_0, &mut (*ps).ev_w_ssl);
            ev_io_stop(loop_0, &mut (*ps).ev_r_ssl);
            ev_io_stop(loop_0, &mut (*ps).ev_w_handshake);
            ev_io_stop(loop_0, &mut (*ps).ev_r_handshake);
            ev_io_stop(loop_0, &mut (*ps).ev_w_connect);
            ev_io_stop(loop_0, &mut (*ps).ev_w_clear);
            ev_io_stop(loop_0, &mut (*ps).ev_r_clear);
            ev_io_stop(loop_0, &mut (*ps).ev_proxy);
            close((*ps).fd_up);
            close((*ps).fd_down);
            SSL_set_shutdown((*ps).ssl, 1 as libc::c_int);
            SSL_free((*ps).ssl);
            free(ps as *mut libc::c_void);
        }
        _ => {}
    };
}
unsafe extern "C" fn handle_socket_errno(
    mut ps: *mut proxystate,
    mut backend: libc::c_int,
) {
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___3: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___4: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___5: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___6: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___7: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___8: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___9: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___10: *mut libc::c_int = 0 as *mut libc::c_int;
    tmp = __errno_location();
    if *tmp == 11 as libc::c_int {
        return
    } else {
        tmp___0 = __errno_location();
        if *tmp___0 == 11 as libc::c_int {
            return
        } else {
            tmp___1 = __errno_location();
            if *tmp___1 == 4 as libc::c_int {
                return;
            }
        }
    }
    tmp___10 = __errno_location();
    if *tmp___10 == 104 as libc::c_int {
        if backend != 0 {
            tmp___2 = b"backend\0" as *const u8 as *const libc::c_char;
        } else {
            tmp___2 = b"client\0" as *const u8 as *const libc::c_char;
        }
        fprintf(
            stderr,
            b"{%s} Connection reset by peer\n\0" as *const u8 as *const libc::c_char,
            tmp___2,
        );
        if (*CONFIG).SYSLOG != 0 {
            if backend != 0 {
                tmp___3 = b"backend\0" as *const u8 as *const libc::c_char;
            } else {
                tmp___3 = b"client\0" as *const u8 as *const libc::c_char;
            }
            syslog(
                3 as libc::c_int,
                b"{%s} Connection reset by peer\n\0" as *const u8 as *const libc::c_char,
                tmp___3,
            );
        }
    } else {
        tmp___9 = __errno_location();
        if *tmp___9 == 110 as libc::c_int {
            if backend != 0 {
                tmp___4 = b"backend\0" as *const u8 as *const libc::c_char;
            } else {
                tmp___4 = b"client\0" as *const u8 as *const libc::c_char;
            }
            fprintf(
                stderr,
                b"{%s} Connection to backend timed out\n\0" as *const u8
                    as *const libc::c_char,
                tmp___4,
            );
            if (*CONFIG).SYSLOG != 0 {
                if backend != 0 {
                    tmp___5 = b"backend\0" as *const u8 as *const libc::c_char;
                } else {
                    tmp___5 = b"client\0" as *const u8 as *const libc::c_char;
                }
                syslog(
                    3 as libc::c_int,
                    b"{%s} Connection to backend timed out\n\0" as *const u8
                        as *const libc::c_char,
                    tmp___5,
                );
            }
        } else {
            tmp___8 = __errno_location();
            if *tmp___8 == 32 as libc::c_int {
                if backend != 0 {
                    tmp___6 = b"backend\0" as *const u8 as *const libc::c_char;
                } else {
                    tmp___6 = b"client\0" as *const u8 as *const libc::c_char;
                }
                fprintf(
                    stderr,
                    b"{%s} Broken pipe to backend (EPIPE)\n\0" as *const u8
                        as *const libc::c_char,
                    tmp___6,
                );
                if (*CONFIG).SYSLOG != 0 {
                    if backend != 0 {
                        tmp___7 = b"backend\0" as *const u8 as *const libc::c_char;
                    } else {
                        tmp___7 = b"client\0" as *const u8 as *const libc::c_char;
                    }
                    syslog(
                        3 as libc::c_int,
                        b"{%s} Broken pipe to backend (EPIPE)\n\0" as *const u8
                            as *const libc::c_char,
                        tmp___7,
                    );
                }
            } else {
                perror(b"{backend} [errno]\0" as *const u8 as *const libc::c_char);
            }
        }
    }
    shutdown_proxy(ps, SHUTDOWN_CLEAR);
}
unsafe extern "C" fn start_connect(mut ps: *mut proxystate) {
    let mut t: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    t = 1 as libc::c_int;
    t = connect(
        (*ps).fd_down,
        (*backaddr).ai_addr as *const sockaddr,
        (*backaddr).ai_addrlen,
    );
    if t == 0 as libc::c_int {
        ev_io_start(loop_0, &mut (*ps).ev_w_connect);
        return;
    } else {
        tmp = __errno_location();
        if *tmp == 115 as libc::c_int {
            ev_io_start(loop_0, &mut (*ps).ev_w_connect);
            return;
        } else {
            tmp___0 = __errno_location();
            if *tmp___0 == 4 as libc::c_int {
                ev_io_start(loop_0, &mut (*ps).ev_w_connect);
                return;
            }
        }
    }
    perror(b"{backend-connect}\0" as *const u8 as *const libc::c_char);
    shutdown_proxy(ps, SHUTDOWN_HARD);
}
unsafe extern "C" fn clear_read(
    mut loop___0: *mut ev_loop,
    mut w: *mut ev_io,
    mut revents: libc::c_int,
) {
    let mut t: libc::c_int = 0;
    let mut ps: *mut proxystate = 0 as *mut proxystate;
    let mut fd: libc::c_int = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: ssize_t = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___3: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___4: libc::c_int = 0;
    ps = (*w).data as *mut proxystate;
    if (*ps).want_shutdown() != 0 {
        ev_io_stop(loop___0, &mut (*ps).ev_r_clear);
        return;
    }
    fd = (*w).fd;
    tmp = ringbuffer_write_ptr(&mut (*ps).ring_clear2ssl);
    buf = tmp;
    tmp___0 = recv(
        fd,
        buf as *mut libc::c_void,
        32768 as libc::c_int as size_t,
        0 as libc::c_int,
    );
    t = tmp___0 as libc::c_int;
    if t > 0 as libc::c_int {
        ringbuffer_write_append(&mut (*ps).ring_clear2ssl, t);
        tmp___1 = ringbuffer_is_full(&mut (*ps).ring_clear2ssl);
        if tmp___1 != 0 {
            ev_io_stop(loop___0, &mut (*ps).ev_r_clear);
        }
        if (*ps).handshaked() != 0 {
            safe_enable_io(ps, &mut (*ps).ev_w_ssl);
        }
    } else if t == 0 as libc::c_int {
        if (*CONFIG).QUIET == 0 {
            if fd == (*ps).fd_down {
                tmp___2 = b"backend\0" as *const u8 as *const libc::c_char;
            } else {
                tmp___2 = b"client\0" as *const u8 as *const libc::c_char;
            }
            fprintf(
                stdout,
                b"{%s} Connection closed\n\0" as *const u8 as *const libc::c_char,
                tmp___2,
            );
        }
        if (*CONFIG).SYSLOG != 0 {
            if fd == (*ps).fd_down {
                tmp___3 = b"backend\0" as *const u8 as *const libc::c_char;
            } else {
                tmp___3 = b"client\0" as *const u8 as *const libc::c_char;
            }
            syslog(
                6 as libc::c_int,
                b"{%s} Connection closed\n\0" as *const u8 as *const libc::c_char,
                tmp___3,
            );
        }
        shutdown_proxy(ps, SHUTDOWN_CLEAR);
    } else {
        if !(t == -(1 as libc::c_int)) {
            __assert_fail(
                b"t == -1\0" as *const u8 as *const libc::c_char,
                b"stud.c\0" as *const u8 as *const libc::c_char,
                960 as libc::c_uint,
                b"clear_read\0" as *const u8 as *const libc::c_char,
            );
        }
        if fd == (*ps).fd_down {
            tmp___4 = 1 as libc::c_int;
        } else {
            tmp___4 = 0 as libc::c_int;
        }
        handle_socket_errno(ps, tmp___4);
    };
}
unsafe extern "C" fn clear_write(
    mut loop___0: *mut ev_loop,
    mut w: *mut ev_io,
    mut revents: libc::c_int,
) {
    let mut t: libc::c_int = 0;
    let mut ps: *mut proxystate = 0 as *mut proxystate;
    let mut fd: libc::c_int = 0;
    let mut sz: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: ssize_t = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    ps = (*w).data as *mut proxystate;
    fd = (*w).fd;
    tmp = ringbuffer_is_empty(&mut (*ps).ring_ssl2clear);
    if tmp != 0 {
        __assert_fail(
            b"!ringbuffer_is_empty(&ps->ring_ssl2clear)\0" as *const u8
                as *const libc::c_char,
            b"stud.c\0" as *const u8 as *const libc::c_char,
            973 as libc::c_uint,
            b"clear_write\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___0 = ringbuffer_read_next(&mut (*ps).ring_ssl2clear, &mut sz);
    next = tmp___0;
    tmp___1 = send(fd, next as *const libc::c_void, sz as size_t, 16384 as libc::c_int);
    t = tmp___1 as libc::c_int;
    if t > 0 as libc::c_int {
        if t == sz {
            ringbuffer_read_pop(&mut (*ps).ring_ssl2clear);
            if (*ps).handshaked() != 0 {
                safe_enable_io(ps, &mut (*ps).ev_r_ssl);
            }
            tmp___2 = ringbuffer_is_empty(&mut (*ps).ring_ssl2clear);
            if tmp___2 != 0 {
                if (*ps).want_shutdown() != 0 {
                    shutdown_proxy(ps, SHUTDOWN_HARD);
                    return;
                }
                ev_io_stop(loop___0, &mut (*ps).ev_w_clear);
            }
        } else {
            ringbuffer_read_skip(&mut (*ps).ring_ssl2clear, t);
        }
    } else {
        if !(t == -(1 as libc::c_int)) {
            __assert_fail(
                b"t == -1\0" as *const u8 as *const libc::c_char,
                b"stud.c\0" as *const u8 as *const libc::c_char,
                996 as libc::c_uint,
                b"clear_write\0" as *const u8 as *const libc::c_char,
            );
        }
        if fd == (*ps).fd_down {
            tmp___3 = 1 as libc::c_int;
        } else {
            tmp___3 = 0 as libc::c_int;
        }
        handle_socket_errno(ps, tmp___3);
    };
}
unsafe extern "C" fn handle_connect(
    mut loop___0: *mut ev_loop,
    mut w: *mut ev_io,
    mut revents: libc::c_int,
) {
    let mut current_block: u64;
    let mut t: libc::c_int = 0;
    let mut ps: *mut proxystate = 0 as *mut proxystate;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___4: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___5: *mut libc::c_int = 0 as *mut libc::c_int;
    ps = (*w).data as *mut proxystate;
    t = connect(
        (*ps).fd_down,
        (*backaddr).ai_addr as *const sockaddr,
        (*backaddr).ai_addrlen,
    );
    if t == 0 {
        current_block = 4979637367829006752;
    } else {
        tmp___4 = __errno_location();
        if *tmp___4 == 106 as libc::c_int {
            current_block = 4979637367829006752;
        } else {
            tmp___5 = __errno_location();
            if *tmp___5 != 0 {
                tmp___1 = __errno_location();
                if !(*tmp___1 == 115 as libc::c_int) {
                    tmp___2 = __errno_location();
                    if !(*tmp___2 == 4 as libc::c_int) {
                        tmp___3 = __errno_location();
                        if !(*tmp___3 == 114 as libc::c_int) {
                            perror(
                                b"{backend-connect}\0" as *const u8 as *const libc::c_char,
                            );
                            shutdown_proxy(ps, SHUTDOWN_HARD);
                        }
                    }
                }
                current_block = 652864300344834934;
            } else {
                current_block = 4979637367829006752;
            }
        }
    }
    match current_block {
        4979637367829006752 => {
            ev_io_stop(loop___0, &mut (*ps).ev_w_connect);
            if (*ps).clear_connected() == 0 {
                (*ps).set_clear_connected(1 as libc::c_int);
                tmp = ringbuffer_is_full(&mut (*ps).ring_clear2ssl);
                if tmp == 0 {
                    safe_enable_io(ps, &mut (*ps).ev_r_clear);
                }
                tmp___0 = ringbuffer_is_empty(&mut (*ps).ring_ssl2clear);
                if tmp___0 == 0 {
                    ev_io_start(loop___0, &mut (*ps).ev_w_clear);
                }
            } else {
                start_handshake(ps, 3 as libc::c_int);
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn start_handshake(mut ps: *mut proxystate, mut err: libc::c_int) {
    ev_io_stop(loop_0, &mut (*ps).ev_r_ssl);
    ev_io_stop(loop_0, &mut (*ps).ev_w_ssl);
    (*ps).set_handshaked(0 as libc::c_int);
    if err == 2 as libc::c_int {
        ev_io_start(loop_0, &mut (*ps).ev_r_handshake);
    } else if err == 3 as libc::c_int {
        ev_io_start(loop_0, &mut (*ps).ev_w_handshake);
    }
}
unsafe extern "C" fn end_handshake(mut ps: *mut proxystate) {
    let mut tcp6_address_string: [libc::c_char; 46] = [0; 46];
    let mut written: size_t = 0;
    let mut ring_pnt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut addr: *mut sockaddr_in = 0 as *mut sockaddr_in;
    let mut tmp___0: __uint16_t = 0;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: libc::c_int = 0;
    let mut addr___0: *mut sockaddr_in6 = 0 as *mut sockaddr_in6;
    let mut tmp___3: __uint16_t = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut ring_pnt___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___7: libc::c_long = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_int = 0;
    written = 0 as libc::c_int as size_t;
    ev_io_stop(loop_0, &mut (*ps).ev_r_handshake);
    ev_io_stop(loop_0, &mut (*ps).ev_w_handshake);
    if !((*(*ps).ssl).s3).is_null() {
        (*(*(*ps).ssl).s3).flags |= 1 as libc::c_long;
    }
    (*ps).set_handshaked(1 as libc::c_int);
    if (*ps).clear_connected() == 0 {
        if (*CONFIG).WRITE_PROXY_LINE != 0 {
            tmp = ringbuffer_write_ptr(&mut (*ps).ring_ssl2clear);
            ring_pnt = tmp;
            if !((*ps).remote_ip.ss_family as libc::c_int == 2 as libc::c_int) {
                if !((*ps).remote_ip.ss_family as libc::c_int == 10 as libc::c_int) {
                    __assert_fail(
                        b"ps->remote_ip.ss_family == AF_INET || ps->remote_ip.ss_family == AF_INET6\0"
                            as *const u8 as *const libc::c_char,
                        b"stud.c\0" as *const u8 as *const libc::c_char,
                        1072 as libc::c_uint,
                        b"end_handshake\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            if (*ps).remote_ip.ss_family as libc::c_int == 2 as libc::c_int {
                addr = &mut (*ps).remote_ip as *mut sockaddr_storage as *mut sockaddr_in;
                tmp___0 = __bswap_16((*addr).sin_port);
                tmp___1 = inet_ntoa((*addr).sin_addr);
                tmp___2 = snprintf(
                    ring_pnt,
                    32768 as libc::c_int as size_t,
                    tcp_proxy_line.as_mut_ptr() as *const libc::c_char,
                    b"TCP4\0" as *const u8 as *const libc::c_char,
                    tmp___1,
                    tmp___0 as libc::c_int,
                );
                written = tmp___2 as size_t;
            } else if (*ps).remote_ip.ss_family as libc::c_int == 10 as libc::c_int {
                addr___0 = &mut (*ps).remote_ip as *mut sockaddr_storage
                    as *mut sockaddr_in6;
                inet_ntop(
                    10 as libc::c_int,
                    &mut (*addr___0).sin6_addr as *mut in6_addr as *const libc::c_void,
                    tcp6_address_string.as_mut_ptr(),
                    46 as libc::c_int as socklen_t,
                );
                tmp___3 = __bswap_16((*addr___0).sin6_port);
                tmp___4 = snprintf(
                    ring_pnt,
                    32768 as libc::c_int as size_t,
                    tcp_proxy_line.as_mut_ptr() as *const libc::c_char,
                    b"TCP6\0" as *const u8 as *const libc::c_char,
                    tcp6_address_string.as_mut_ptr(),
                    tmp___3 as libc::c_int,
                );
                written = tmp___4 as size_t;
            }
            ringbuffer_write_append(&mut (*ps).ring_ssl2clear, written as libc::c_int);
        } else if (*CONFIG).WRITE_IP_OCTET != 0 {
            tmp___5 = ringbuffer_write_ptr(&mut (*ps).ring_ssl2clear);
            ring_pnt___0 = tmp___5;
            if !((*ps).remote_ip.ss_family as libc::c_int == 2 as libc::c_int) {
                if !((*ps).remote_ip.ss_family as libc::c_int == 10 as libc::c_int) {
                    __assert_fail(
                        b"ps->remote_ip.ss_family == AF_INET || ps->remote_ip.ss_family == AF_INET6\0"
                            as *const u8 as *const libc::c_char,
                        b"stud.c\0" as *const u8 as *const libc::c_char,
                        1097 as libc::c_uint,
                        b"end_handshake\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            tmp___6 = ring_pnt___0;
            ring_pnt___0 = ring_pnt___0.offset(1);
            *tmp___6 = (*ps).remote_ip.ss_family as libc::c_uchar as libc::c_char;
            if (*ps).remote_ip.ss_family as libc::c_int == 10 as libc::c_int {
                memcpy(
                    ring_pnt___0 as *mut libc::c_void,
                    &mut (*(&mut (*ps).remote_ip as *mut sockaddr_storage
                        as *mut sockaddr_in6))
                        .sin6_addr
                        .__in6_u
                        .__u6_addr8 as *mut [uint8_t; 16] as *const libc::c_void,
                    16 as libc::c_uint as size_t,
                );
                ringbuffer_write_append(&mut (*ps).ring_ssl2clear, 17 as libc::c_int);
            } else {
                memcpy(
                    ring_pnt___0 as *mut libc::c_void,
                    &mut (*(&mut (*ps).remote_ip as *mut sockaddr_storage
                        as *mut sockaddr_in))
                        .sin_addr
                        .s_addr as *mut in_addr_t as *const libc::c_void,
                    4 as libc::c_uint as size_t,
                );
                ringbuffer_write_append(&mut (*ps).ring_ssl2clear, 5 as libc::c_int);
            }
        }
        start_connect(ps);
    } else {
        tmp___7 = SSL_ctrl(
            (*ps).ssl,
            8 as libc::c_int,
            0 as libc::c_long,
            0 as *mut libc::c_void,
        );
        if tmp___7 == 0 {
            if !client_session.is_null() {
                SSL_SESSION_free(client_session);
            }
            client_session = SSL_get1_session((*ps).ssl);
        }
    }
    tmp___8 = ringbuffer_is_full(&mut (*ps).ring_ssl2clear);
    if tmp___8 == 0 {
        safe_enable_io(ps, &mut (*ps).ev_r_ssl);
    }
    tmp___9 = ringbuffer_is_empty(&mut (*ps).ring_clear2ssl);
    if tmp___9 == 0 {
        ev_io_start(loop_0, &mut (*ps).ev_w_ssl);
    }
}
unsafe extern "C" fn client_proxy_proxy(
    mut loop___0: *mut ev_loop,
    mut w: *mut ev_io,
    mut revents: libc::c_int,
) {
    let mut t: libc::c_int = 0;
    let mut proxy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ps: *mut proxystate = 0 as *mut proxystate;
    let mut b: *mut BIO = 0 as *mut BIO;
    let mut tmp: *mut BIO = 0 as *mut BIO;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    let mut ring: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: libc::c_int = 0;
    proxy = tcp_proxy_line.as_mut_ptr();
    end = tcp_proxy_line
        .as_mut_ptr()
        .offset(::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as isize);
    ps = (*w).data as *mut proxystate;
    tmp = SSL_get_rbio((*ps).ssl as *const SSL);
    b = tmp;
    while proxy as libc::c_ulong != end as libc::c_ulong {
        t = BIO_read(b, proxy as *mut libc::c_void, 1 as libc::c_int);
        if !(t == 1 as libc::c_int) {
            break;
        }
        tmp___0 = proxy;
        proxy = proxy.offset(1);
        if *tmp___0 as libc::c_int == 10 as libc::c_int {
            break;
        }
    }
    if proxy as libc::c_ulong == end as libc::c_ulong {
        if (*CONFIG).QUIET == 0 {
            fprintf(
                stdout,
                b"{client} Unexpectedly long PROXY line. Perhaps a malformed request?\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                6 as libc::c_int,
                b"{client} Unexpectedly long PROXY line. Perhaps a malformed request?\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        shutdown_proxy(ps, SHUTDOWN_SSL);
    } else if t == 1 as libc::c_int {
        tmp___1 = ringbuffer_is_full(&mut (*ps).ring_ssl2clear);
        if tmp___1 != 0 {
            if (*CONFIG).QUIET == 0 {
                fprintf(
                    stdout,
                    b"{client} Error writing PROXY line\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if (*CONFIG).SYSLOG != 0 {
                syslog(
                    6 as libc::c_int,
                    b"{client} Error writing PROXY line\0" as *const u8
                        as *const libc::c_char,
                );
            }
            shutdown_proxy(ps, SHUTDOWN_SSL);
            return;
        }
        tmp___2 = ringbuffer_write_ptr(&mut (*ps).ring_ssl2clear);
        ring = tmp___2;
        memcpy(
            ring as *mut libc::c_void,
            tcp_proxy_line.as_mut_ptr() as *const libc::c_void,
            proxy.offset_from(tcp_proxy_line.as_mut_ptr()) as libc::c_long as size_t,
        );
        ringbuffer_write_append(
            &mut (*ps).ring_ssl2clear,
            proxy.offset_from(tcp_proxy_line.as_mut_ptr()) as libc::c_long as libc::c_int,
        );
        if *proxy.offset(-(1 as libc::c_int as isize)) as libc::c_int
            == 10 as libc::c_int
        {
            ev_io_stop(loop___0, &mut (*ps).ev_proxy);
            start_handshake(ps, 2 as libc::c_int);
        }
    } else {
        tmp___3 = BIO_test_flags(b as *const BIO, 8 as libc::c_int);
        if tmp___3 == 0 {
            if (*CONFIG).QUIET == 0 {
                fprintf(
                    stdout,
                    b"{client} Unexpected error reading PROXY line\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if (*CONFIG).SYSLOG != 0 {
                syslog(
                    6 as libc::c_int,
                    b"{client} Unexpected error reading PROXY line\0" as *const u8
                        as *const libc::c_char,
                );
            }
            shutdown_proxy(ps, SHUTDOWN_SSL);
        }
    };
}
unsafe extern "C" fn client_handshake(
    mut loop___0: *mut ev_loop,
    mut w: *mut ev_io,
    mut revents: libc::c_int,
) {
    let mut t: libc::c_int = 0;
    let mut ps: *mut proxystate = 0 as *mut proxystate;
    let mut err: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___2: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___3: *const libc::c_char = 0 as *const libc::c_char;
    ps = (*w).data as *mut proxystate;
    t = SSL_do_handshake((*ps).ssl);
    if t == 1 as libc::c_int {
        end_handshake(ps);
    } else {
        tmp = SSL_get_error((*ps).ssl as *const SSL, t);
        err = tmp;
        if err == 2 as libc::c_int {
            ev_io_stop(loop___0, &mut (*ps).ev_w_handshake);
            ev_io_start(loop___0, &mut (*ps).ev_r_handshake);
        } else if err == 3 as libc::c_int {
            ev_io_stop(loop___0, &mut (*ps).ev_r_handshake);
            ev_io_start(loop___0, &mut (*ps).ev_w_handshake);
        } else if err == 6 as libc::c_int {
            if (*CONFIG).QUIET == 0 {
                if (*w).fd == (*ps).fd_up {
                    tmp___0 = b"client\0" as *const u8 as *const libc::c_char;
                } else {
                    tmp___0 = b"backend\0" as *const u8 as *const libc::c_char;
                }
                fprintf(
                    stdout,
                    b"{%s} Connection closed (in handshake)\n\0" as *const u8
                        as *const libc::c_char,
                    tmp___0,
                );
            }
            if (*CONFIG).SYSLOG != 0 {
                if (*w).fd == (*ps).fd_up {
                    tmp___1 = b"client\0" as *const u8 as *const libc::c_char;
                } else {
                    tmp___1 = b"backend\0" as *const u8 as *const libc::c_char;
                }
                syslog(
                    6 as libc::c_int,
                    b"{%s} Connection closed (in handshake)\n\0" as *const u8
                        as *const libc::c_char,
                    tmp___1,
                );
            }
            shutdown_proxy(ps, SHUTDOWN_SSL);
        } else {
            if (*CONFIG).QUIET == 0 {
                if (*w).fd == (*ps).fd_up {
                    tmp___2 = b"client\0" as *const u8 as *const libc::c_char;
                } else {
                    tmp___2 = b"backend\0" as *const u8 as *const libc::c_char;
                }
                fprintf(
                    stdout,
                    b"{%s} Unexpected SSL error (in handshake): %d\n\0" as *const u8
                        as *const libc::c_char,
                    tmp___2,
                    err,
                );
            }
            if (*CONFIG).SYSLOG != 0 {
                if (*w).fd == (*ps).fd_up {
                    tmp___3 = b"client\0" as *const u8 as *const libc::c_char;
                } else {
                    tmp___3 = b"backend\0" as *const u8 as *const libc::c_char;
                }
                syslog(
                    6 as libc::c_int,
                    b"{%s} Unexpected SSL error (in handshake): %d\n\0" as *const u8
                        as *const libc::c_char,
                    tmp___3,
                    err,
                );
            }
            shutdown_proxy(ps, SHUTDOWN_SSL);
        }
    };
}
unsafe extern "C" fn handle_fatal_ssl_error(
    mut ps: *mut proxystate,
    mut err: libc::c_int,
    mut backend: libc::c_int,
) {
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___2: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___3: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___4: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___5: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___6: *const libc::c_char = 0 as *const libc::c_char;
    if err == 6 as libc::c_int {
        if backend != 0 {
            tmp = b"backend\0" as *const u8 as *const libc::c_char;
        } else {
            tmp = b"client\0" as *const u8 as *const libc::c_char;
        }
        fprintf(
            stderr,
            b"{%s} Connection closed (in data)\n\0" as *const u8 as *const libc::c_char,
            tmp,
        );
        if (*CONFIG).SYSLOG != 0 {
            if backend != 0 {
                tmp___0 = b"backend\0" as *const u8 as *const libc::c_char;
            } else {
                tmp___0 = b"client\0" as *const u8 as *const libc::c_char;
            }
            syslog(
                3 as libc::c_int,
                b"{%s} Connection closed (in data)\n\0" as *const u8
                    as *const libc::c_char,
                tmp___0,
            );
        }
    } else if err == 5 as libc::c_int {
        tmp___4 = __errno_location();
        if *tmp___4 == 0 as libc::c_int {
            if backend != 0 {
                tmp___1 = b"backend\0" as *const u8 as *const libc::c_char;
            } else {
                tmp___1 = b"client\0" as *const u8 as *const libc::c_char;
            }
            fprintf(
                stderr,
                b"{%s} Connection closed (in data)\n\0" as *const u8
                    as *const libc::c_char,
                tmp___1,
            );
            if (*CONFIG).SYSLOG != 0 {
                if backend != 0 {
                    tmp___2 = b"backend\0" as *const u8 as *const libc::c_char;
                } else {
                    tmp___2 = b"client\0" as *const u8 as *const libc::c_char;
                }
                syslog(
                    3 as libc::c_int,
                    b"{%s} Connection closed (in data)\n\0" as *const u8
                        as *const libc::c_char,
                    tmp___2,
                );
            }
        } else {
            if backend != 0 {
                tmp___3 = b"{backend} [errno] \0" as *const u8 as *const libc::c_char;
            } else {
                tmp___3 = b"{client} [errno] \0" as *const u8 as *const libc::c_char;
            }
            perror(tmp___3);
        }
    } else {
        if backend != 0 {
            tmp___5 = b"backend\0" as *const u8 as *const libc::c_char;
        } else {
            tmp___5 = b"client\0" as *const u8 as *const libc::c_char;
        }
        fprintf(
            stderr,
            b"{%s} Unexpected SSL_read error: %d\n\0" as *const u8
                as *const libc::c_char,
            tmp___5,
            err,
        );
        if (*CONFIG).SYSLOG != 0 {
            if backend != 0 {
                tmp___6 = b"backend\0" as *const u8 as *const libc::c_char;
            } else {
                tmp___6 = b"client\0" as *const u8 as *const libc::c_char;
            }
            syslog(
                3 as libc::c_int,
                b"{%s} Unexpected SSL_read error: %d\n\0" as *const u8
                    as *const libc::c_char,
                tmp___6,
                err,
            );
        }
    }
    shutdown_proxy(ps, SHUTDOWN_SSL);
}
unsafe extern "C" fn ssl_read(
    mut loop___0: *mut ev_loop,
    mut w: *mut ev_io,
    mut revents: libc::c_int,
) {
    let mut t: libc::c_int = 0;
    let mut ps: *mut proxystate = 0 as *mut proxystate;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    ps = (*w).data as *mut proxystate;
    if (*ps).want_shutdown() != 0 {
        ev_io_stop(loop___0, &mut (*ps).ev_r_ssl);
        return;
    }
    tmp = ringbuffer_write_ptr(&mut (*ps).ring_ssl2clear);
    buf = tmp;
    t = SSL_read((*ps).ssl, buf as *mut libc::c_void, 32768 as libc::c_int);
    if (*ps).renegotiation() != 0 {
        shutdown_proxy(ps, SHUTDOWN_SSL);
        return;
    }
    if t > 0 as libc::c_int {
        ringbuffer_write_append(&mut (*ps).ring_ssl2clear, t);
        tmp___0 = ringbuffer_is_full(&mut (*ps).ring_ssl2clear);
        if tmp___0 != 0 {
            ev_io_stop(loop___0, &mut (*ps).ev_r_ssl);
        }
        if (*ps).clear_connected() != 0 {
            safe_enable_io(ps, &mut (*ps).ev_w_clear);
        }
    } else {
        tmp___1 = SSL_get_error((*ps).ssl as *const SSL, t);
        err = tmp___1;
        if err == 3 as libc::c_int {
            start_handshake(ps, err);
        } else if !(err == 2 as libc::c_int) {
            if (*w).fd == (*ps).fd_up {
                tmp___2 = 0 as libc::c_int;
            } else {
                tmp___2 = 1 as libc::c_int;
            }
            handle_fatal_ssl_error(ps, err, tmp___2);
        }
    };
}
unsafe extern "C" fn ssl_write(
    mut loop___0: *mut ev_loop,
    mut w: *mut ev_io,
    mut revents: libc::c_int,
) {
    let mut t: libc::c_int = 0;
    let mut sz: libc::c_int = 0;
    let mut ps: *mut proxystate = 0 as *mut proxystate;
    let mut tmp: libc::c_int = 0;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    ps = (*w).data as *mut proxystate;
    tmp = ringbuffer_is_empty(&mut (*ps).ring_clear2ssl);
    if tmp != 0 {
        __assert_fail(
            b"!ringbuffer_is_empty(&ps->ring_clear2ssl)\0" as *const u8
                as *const libc::c_char,
            b"stud.c\0" as *const u8 as *const libc::c_char,
            1264 as libc::c_uint,
            b"ssl_write\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___0 = ringbuffer_read_next(&mut (*ps).ring_clear2ssl, &mut sz);
    next = tmp___0;
    t = SSL_write((*ps).ssl, next as *const libc::c_void, sz);
    if t > 0 as libc::c_int {
        if t == sz {
            ringbuffer_read_pop(&mut (*ps).ring_clear2ssl);
            if (*ps).clear_connected() != 0 {
                safe_enable_io(ps, &mut (*ps).ev_r_clear);
            }
            tmp___1 = ringbuffer_is_empty(&mut (*ps).ring_clear2ssl);
            if tmp___1 != 0 {
                if (*ps).want_shutdown() != 0 {
                    shutdown_proxy(ps, SHUTDOWN_HARD);
                    return;
                }
                ev_io_stop(loop___0, &mut (*ps).ev_w_ssl);
            }
        } else {
            ringbuffer_read_skip(&mut (*ps).ring_clear2ssl, t);
        }
    } else {
        tmp___2 = SSL_get_error((*ps).ssl as *const SSL, t);
        err = tmp___2;
        if err == 2 as libc::c_int {
            start_handshake(ps, err);
        } else if !(err == 3 as libc::c_int) {
            if (*w).fd == (*ps).fd_up {
                tmp___3 = 0 as libc::c_int;
            } else {
                tmp___3 = 1 as libc::c_int;
            }
            handle_fatal_ssl_error(ps, err, tmp___3);
        }
    };
}
unsafe extern "C" fn handle_accept(
    mut loop___0: *mut ev_loop,
    mut w: *mut ev_io,
    mut revents: libc::c_int,
) {
    let mut addr: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut sl: socklen_t = 0;
    let mut client___0: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut flag: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut back: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut ctx: *mut SSL_CTX = 0 as *mut SSL_CTX;
    let mut ssl___0: *mut SSL = 0 as *mut SSL;
    let mut tmp___6: *mut SSL = 0 as *mut SSL;
    let mut mode: libc::c_long = 0;
    let mut ps: *mut proxystate = 0 as *mut proxystate;
    let mut tmp___7: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: libc::c_int = 0;
    let mut tmp___13: libc::c_int = 0;
    let mut tmp___14: libc::c_int = 0;
    let mut tmp___15: libc::c_int = 0;
    sl = ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong as socklen_t;
    tmp = accept(
        (*w).fd,
        &mut addr as *mut sockaddr_storage as *mut sockaddr,
        &mut sl as *mut socklen_t,
    );
    client___0 = tmp;
    if client___0 == -(1 as libc::c_int) {
        tmp___0 = __errno_location();
        match *tmp___0 {
            24 => {
                fprintf(
                    stderr,
                    b"{client} accept() failed; too many open files for this process\n\0"
                        as *const u8 as *const libc::c_char,
                );
                if (*CONFIG).SYSLOG != 0 {
                    syslog(
                        3 as libc::c_int,
                        b"{client} accept() failed; too many open files for this process\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
            }
            23 => {
                fprintf(
                    stderr,
                    b"{client} accept() failed; too many open files for this system\n\0"
                        as *const u8 as *const libc::c_char,
                );
                if (*CONFIG).SYSLOG != 0 {
                    syslog(
                        3 as libc::c_int,
                        b"{client} accept() failed; too many open files for this system\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
            }
            _ => {
                tmp___1 = __errno_location();
                if !(*tmp___1 == 4 as libc::c_int) {
                    tmp___2 = __errno_location();
                    if !(*tmp___2 == 11 as libc::c_int) {
                        tmp___3 = __errno_location();
                        if !(*tmp___3 == 11 as libc::c_int) {
                            __assert_fail(
                                b"errno == EINTR || errno == EWOULDBLOCK || errno == EAGAIN\0"
                                    as *const u8 as *const libc::c_char,
                                b"stud.c\0" as *const u8 as *const libc::c_char,
                                1315 as libc::c_uint,
                                b"handle_accept\0" as *const u8 as *const libc::c_char,
                            );
                        }
                    }
                }
            }
        }
        return;
    }
    flag = 1 as libc::c_int;
    tmp___4 = setsockopt(
        client___0,
        6 as libc::c_int,
        1 as libc::c_int,
        &mut flag as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    ret = tmp___4;
    if ret == -(1 as libc::c_int) {
        perror(
            b"Couldn't setsockopt on client (TCP_NODELAY)\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    setnonblocking(client___0);
    settcpkeepalive(client___0);
    tmp___5 = create_back_socket();
    back = tmp___5;
    if back == -(1 as libc::c_int) {
        close(client___0);
        perror(b"{backend-socket}\0" as *const u8 as *const libc::c_char);
        return;
    }
    ctx = (*w).data as *mut SSL_CTX;
    tmp___6 = SSL_new(ctx);
    ssl___0 = tmp___6;
    mode = 1 as libc::c_long;
    mode |= 16 as libc::c_long;
    SSL_ctrl(ssl___0, 33 as libc::c_int, mode, 0 as *mut libc::c_void);
    SSL_set_accept_state(ssl___0);
    SSL_set_fd(ssl___0, client___0);
    tmp___7 = malloc(::std::mem::size_of::<proxystate>() as libc::c_ulong);
    ps = tmp___7 as *mut proxystate;
    (*ps).fd_up = client___0;
    (*ps).fd_down = back;
    (*ps).ssl = ssl___0;
    (*ps).set_want_shutdown(0 as libc::c_int);
    (*ps).set_clear_connected(0 as libc::c_int);
    (*ps).set_handshaked(0 as libc::c_int);
    (*ps).set_renegotiation(0 as libc::c_int);
    (*ps).remote_ip = addr;
    ringbuffer_init(&mut (*ps).ring_clear2ssl);
    ringbuffer_init(&mut (*ps).ring_ssl2clear);
    tmp___8 = 0 as libc::c_int;
    (*(&mut (*ps).ev_r_ssl as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .pending = tmp___8;
    (*(&mut (*ps).ev_r_ssl as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = tmp___8;
    (*(&mut (*ps).ev_r_ssl as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_r_ssl
        .cb = Some(
        ssl_read as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_r_ssl as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_r_ssl.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_r_ssl.fd = client___0;
    (*ps).ev_r_ssl.events = 129 as libc::c_int;
    tmp___9 = 0 as libc::c_int;
    (*(&mut (*ps).ev_w_ssl as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .pending = tmp___9;
    (*(&mut (*ps).ev_w_ssl as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = tmp___9;
    (*(&mut (*ps).ev_w_ssl as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_w_ssl
        .cb = Some(
        ssl_write as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_w_ssl as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_w_ssl.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_w_ssl.fd = client___0;
    (*ps).ev_w_ssl.events = 130 as libc::c_int;
    tmp___10 = 0 as libc::c_int;
    (*(&mut (*ps).ev_r_handshake as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .pending = tmp___10;
    (*(&mut (*ps).ev_r_handshake as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = tmp___10;
    (*(&mut (*ps).ev_r_handshake as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_r_handshake
        .cb = Some(
        client_handshake
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_r_handshake as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_r_handshake.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_r_handshake.fd = client___0;
    (*ps).ev_r_handshake.events = 129 as libc::c_int;
    tmp___11 = 0 as libc::c_int;
    (*(&mut (*ps).ev_w_handshake as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .pending = tmp___11;
    (*(&mut (*ps).ev_w_handshake as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = tmp___11;
    (*(&mut (*ps).ev_w_handshake as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_w_handshake
        .cb = Some(
        client_handshake
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_w_handshake as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_w_handshake.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_w_handshake.fd = client___0;
    (*ps).ev_w_handshake.events = 130 as libc::c_int;
    tmp___12 = 0 as libc::c_int;
    (*(&mut (*ps).ev_proxy as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .pending = tmp___12;
    (*(&mut (*ps).ev_proxy as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = tmp___12;
    (*(&mut (*ps).ev_proxy as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_proxy
        .cb = Some(
        client_proxy_proxy
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_proxy as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_proxy.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_proxy.fd = client___0;
    (*ps).ev_proxy.events = 129 as libc::c_int;
    tmp___13 = 0 as libc::c_int;
    (*(&mut (*ps).ev_w_connect as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .pending = tmp___13;
    (*(&mut (*ps).ev_w_connect as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = tmp___13;
    (*(&mut (*ps).ev_w_connect as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_w_connect
        .cb = Some(
        handle_connect
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_w_connect as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_w_connect.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_w_connect.fd = back;
    (*ps).ev_w_connect.events = 130 as libc::c_int;
    tmp___14 = 0 as libc::c_int;
    (*(&mut (*ps).ev_w_clear as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .pending = tmp___14;
    (*(&mut (*ps).ev_w_clear as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = tmp___14;
    (*(&mut (*ps).ev_w_clear as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_w_clear
        .cb = Some(
        clear_write as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_w_clear as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_w_clear.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_w_clear.fd = back;
    (*ps).ev_w_clear.events = 130 as libc::c_int;
    tmp___15 = 0 as libc::c_int;
    (*(&mut (*ps).ev_r_clear as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .pending = tmp___15;
    (*(&mut (*ps).ev_r_clear as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = tmp___15;
    (*(&mut (*ps).ev_r_clear as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_r_clear
        .cb = Some(
        clear_read as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_r_clear as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_r_clear.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_r_clear.fd = back;
    (*ps).ev_r_clear.events = 129 as libc::c_int;
    (*ps).ev_r_ssl.data = ps as *mut libc::c_void;
    (*ps).ev_w_ssl.data = ps as *mut libc::c_void;
    (*ps).ev_r_clear.data = ps as *mut libc::c_void;
    (*ps).ev_w_clear.data = ps as *mut libc::c_void;
    (*ps).ev_proxy.data = ps as *mut libc::c_void;
    (*ps).ev_w_connect.data = ps as *mut libc::c_void;
    (*ps).ev_r_handshake.data = ps as *mut libc::c_void;
    (*ps).ev_w_handshake.data = ps as *mut libc::c_void;
    SSL_set_ex_data(
        ssl___0,
        0 as libc::c_int,
        ps as *mut libc::c_char as *mut libc::c_void,
    );
    if (*CONFIG).PROXY_PROXY_LINE != 0 {
        ev_io_start(loop___0, &mut (*ps).ev_proxy);
    } else {
        start_handshake(ps, 2 as libc::c_int);
    };
}
unsafe extern "C" fn check_ppid(
    mut loop___0: *mut ev_loop,
    mut w: *mut ev_timer,
    mut revents: libc::c_int,
) {
    let mut ppid: pid_t = 0;
    let mut tmp: __pid_t = 0;
    tmp = getppid();
    ppid = tmp;
    if ppid != master_pid {
        fprintf(
            stderr,
            b"{core} Process %d detected parent death, closing listener socket.\n\0"
                as *const u8 as *const libc::c_char,
            child_num,
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"{core} Process %d detected parent death, closing listener socket.\n\0"
                    as *const u8 as *const libc::c_char,
                child_num,
            );
        }
        ev_timer_stop(loop___0, w);
        ev_io_stop(loop___0, &mut listener);
        close(listener_socket);
    }
}
unsafe extern "C" fn handle_clear_accept(
    mut loop___0: *mut ev_loop,
    mut w: *mut ev_io,
    mut revents: libc::c_int,
) {
    let mut addr: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut sl: socklen_t = 0;
    let mut client___0: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut flag: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut back: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut ctx: *mut SSL_CTX = 0 as *mut SSL_CTX;
    let mut ssl___0: *mut SSL = 0 as *mut SSL;
    let mut tmp___6: *mut SSL = 0 as *mut SSL;
    let mut mode: libc::c_long = 0;
    let mut ps: *mut proxystate = 0 as *mut proxystate;
    let mut tmp___7: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: libc::c_int = 0;
    let mut tmp___13: libc::c_int = 0;
    let mut tmp___14: libc::c_int = 0;
    sl = ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong as socklen_t;
    tmp = accept(
        (*w).fd,
        &mut addr as *mut sockaddr_storage as *mut sockaddr,
        &mut sl as *mut socklen_t,
    );
    client___0 = tmp;
    if client___0 == -(1 as libc::c_int) {
        tmp___0 = __errno_location();
        match *tmp___0 {
            24 => {
                fprintf(
                    stderr,
                    b"{client} accept() failed; too many open files for this process\n\0"
                        as *const u8 as *const libc::c_char,
                );
                if (*CONFIG).SYSLOG != 0 {
                    syslog(
                        3 as libc::c_int,
                        b"{client} accept() failed; too many open files for this process\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
            }
            23 => {
                fprintf(
                    stderr,
                    b"{client} accept() failed; too many open files for this system\n\0"
                        as *const u8 as *const libc::c_char,
                );
                if (*CONFIG).SYSLOG != 0 {
                    syslog(
                        3 as libc::c_int,
                        b"{client} accept() failed; too many open files for this system\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
            }
            _ => {
                tmp___1 = __errno_location();
                if !(*tmp___1 == 4 as libc::c_int) {
                    tmp___2 = __errno_location();
                    if !(*tmp___2 == 11 as libc::c_int) {
                        tmp___3 = __errno_location();
                        if !(*tmp___3 == 11 as libc::c_int) {
                            __assert_fail(
                                b"errno == EINTR || errno == EWOULDBLOCK || errno == EAGAIN\0"
                                    as *const u8 as *const libc::c_char,
                                b"stud.c\0" as *const u8 as *const libc::c_char,
                                1432 as libc::c_uint,
                                b"handle_clear_accept\0" as *const u8 as *const libc::c_char,
                            );
                        }
                    }
                }
            }
        }
        return;
    }
    flag = 1 as libc::c_int;
    tmp___4 = setsockopt(
        client___0,
        6 as libc::c_int,
        1 as libc::c_int,
        &mut flag as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    ret = tmp___4;
    if ret == -(1 as libc::c_int) {
        perror(
            b"Couldn't setsockopt on client (TCP_NODELAY)\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    setnonblocking(client___0);
    settcpkeepalive(client___0);
    tmp___5 = create_back_socket();
    back = tmp___5;
    if back == -(1 as libc::c_int) {
        close(client___0);
        perror(b"{backend-socket}\0" as *const u8 as *const libc::c_char);
        return;
    }
    ctx = (*w).data as *mut SSL_CTX;
    tmp___6 = SSL_new(ctx);
    ssl___0 = tmp___6;
    mode = 1 as libc::c_long;
    mode |= 16 as libc::c_long;
    SSL_ctrl(ssl___0, 33 as libc::c_int, mode, 0 as *mut libc::c_void);
    SSL_set_connect_state(ssl___0);
    SSL_set_fd(ssl___0, back);
    if !client_session.is_null() {
        SSL_set_session(ssl___0, client_session);
    }
    tmp___7 = malloc(::std::mem::size_of::<proxystate>() as libc::c_ulong);
    ps = tmp___7 as *mut proxystate;
    (*ps).fd_up = client___0;
    (*ps).fd_down = back;
    (*ps).ssl = ssl___0;
    (*ps).set_want_shutdown(0 as libc::c_int);
    (*ps).set_clear_connected(1 as libc::c_int);
    (*ps).set_handshaked(0 as libc::c_int);
    (*ps).set_renegotiation(0 as libc::c_int);
    (*ps).remote_ip = addr;
    ringbuffer_init(&mut (*ps).ring_clear2ssl);
    ringbuffer_init(&mut (*ps).ring_ssl2clear);
    tmp___8 = 0 as libc::c_int;
    (*(&mut (*ps).ev_r_clear as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .pending = tmp___8;
    (*(&mut (*ps).ev_r_clear as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = tmp___8;
    (*(&mut (*ps).ev_r_clear as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_r_clear
        .cb = Some(
        clear_read as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_r_clear as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_r_clear.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_r_clear.fd = client___0;
    (*ps).ev_r_clear.events = 129 as libc::c_int;
    tmp___9 = 0 as libc::c_int;
    (*(&mut (*ps).ev_w_clear as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .pending = tmp___9;
    (*(&mut (*ps).ev_w_clear as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = tmp___9;
    (*(&mut (*ps).ev_w_clear as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_w_clear
        .cb = Some(
        clear_write as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_w_clear as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_w_clear.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_w_clear.fd = client___0;
    (*ps).ev_w_clear.events = 130 as libc::c_int;
    tmp___10 = 0 as libc::c_int;
    (*(&mut (*ps).ev_w_connect as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .pending = tmp___10;
    (*(&mut (*ps).ev_w_connect as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = tmp___10;
    (*(&mut (*ps).ev_w_connect as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_w_connect
        .cb = Some(
        handle_connect
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_w_connect as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_w_connect.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_w_connect.fd = back;
    (*ps).ev_w_connect.events = 130 as libc::c_int;
    tmp___11 = 0 as libc::c_int;
    (*(&mut (*ps).ev_r_handshake as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .pending = tmp___11;
    (*(&mut (*ps).ev_r_handshake as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = tmp___11;
    (*(&mut (*ps).ev_r_handshake as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_r_handshake
        .cb = Some(
        client_handshake
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_r_handshake as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_r_handshake.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_r_handshake.fd = back;
    (*ps).ev_r_handshake.events = 129 as libc::c_int;
    tmp___12 = 0 as libc::c_int;
    (*(&mut (*ps).ev_w_handshake as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .pending = tmp___12;
    (*(&mut (*ps).ev_w_handshake as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = tmp___12;
    (*(&mut (*ps).ev_w_handshake as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_w_handshake
        .cb = Some(
        client_handshake
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_w_handshake as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_w_handshake.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_w_handshake.fd = back;
    (*ps).ev_w_handshake.events = 130 as libc::c_int;
    tmp___13 = 0 as libc::c_int;
    (*(&mut (*ps).ev_w_ssl as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .pending = tmp___13;
    (*(&mut (*ps).ev_w_ssl as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = tmp___13;
    (*(&mut (*ps).ev_w_ssl as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_w_ssl
        .cb = Some(
        ssl_write as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_w_ssl as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_w_ssl.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_w_ssl.fd = back;
    (*ps).ev_w_ssl.events = 130 as libc::c_int;
    tmp___14 = 0 as libc::c_int;
    (*(&mut (*ps).ev_r_ssl as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .pending = tmp___14;
    (*(&mut (*ps).ev_r_ssl as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = tmp___14;
    (*(&mut (*ps).ev_r_ssl as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    (*ps)
        .ev_r_ssl
        .cb = Some(
        ssl_read as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut (*ps).ev_r_ssl as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut (*ps).ev_r_ssl.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    (*ps).ev_r_ssl.fd = back;
    (*ps).ev_r_ssl.events = 129 as libc::c_int;
    (*ps).ev_r_ssl.data = ps as *mut libc::c_void;
    (*ps).ev_w_ssl.data = ps as *mut libc::c_void;
    (*ps).ev_r_clear.data = ps as *mut libc::c_void;
    (*ps).ev_w_clear.data = ps as *mut libc::c_void;
    (*ps).ev_w_connect.data = ps as *mut libc::c_void;
    (*ps).ev_r_handshake.data = ps as *mut libc::c_void;
    (*ps).ev_w_handshake.data = ps as *mut libc::c_void;
    SSL_set_ex_data(
        ssl___0,
        0 as libc::c_int,
        ps as *mut libc::c_char as *mut libc::c_void,
    );
    ev_io_start(loop___0, &mut (*ps).ev_r_clear);
    start_connect(ps);
}
unsafe extern "C" fn handle_connections() {
    let mut cpus: cpu_set_t = cpu_set_t { __bits: [0; 16] };
    let mut __cpu: size_t = 0;
    let mut res: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut timer_ppid_check: ev_timer = ev_timer {
        active: 0,
        pending: 0,
        priority: 0,
        data: 0 as *mut libc::c_void,
        cb: None,
        at: 0.,
        repeat: 0.,
    };
    let mut tmp___0: libc::c_int = 0;
    if (*CONFIG).QUIET == 0 {
        fprintf(
            stdout,
            b"{core} Process %d online\n\0" as *const u8 as *const libc::c_char,
            child_num,
        );
    }
    if (*CONFIG).SYSLOG != 0 {
        syslog(
            6 as libc::c_int,
            b"{core} Process %d online\n\0" as *const u8 as *const libc::c_char,
            child_num,
        );
    }
    create_workers = 0 as libc::c_int;
    libc::memset(
        &mut cpus as *mut cpu_set_t as *mut libc::c_void,
        '\u{0}' as i32,
        ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong as libc::c_int
            as libc::c_ulong as libc::size_t,
    );
    __cpu = child_num as size_t;
    if __cpu.wrapping_div(8 as libc::c_ulong)
        < ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong
    {
        cpus
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
    tmp = sched_setaffinity(
        0 as libc::c_int,
        ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong,
        &mut cpus as *mut cpu_set_t as *const cpu_set_t,
    );
    res = tmp;
    if res == 0 {
        if (*CONFIG).QUIET == 0 {
            fprintf(
                stdout,
                b"{core} Successfully attached to CPU #%d\n\0" as *const u8
                    as *const libc::c_char,
                child_num,
            );
        }
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                6 as libc::c_int,
                b"{core} Successfully attached to CPU #%d\n\0" as *const u8
                    as *const libc::c_char,
                child_num,
            );
        }
    } else {
        fprintf(
            stderr,
            b"{core-warning} Unable to attach to CPU #%d; do you have that many cores?\n\0"
                as *const u8 as *const libc::c_char,
            child_num,
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"{core-warning} Unable to attach to CPU #%d; do you have that many cores?\n\0"
                    as *const u8 as *const libc::c_char,
                child_num,
            );
        }
    }
    loop_0 = ev_default_loop(0 as libc::c_uint);
    tmp___0 = 0 as libc::c_int;
    (*(&mut timer_ppid_check as *mut ev_timer as *mut libc::c_void as *mut ev_watcher))
        .pending = tmp___0;
    (*(&mut timer_ppid_check as *mut ev_timer as *mut libc::c_void as *mut ev_watcher))
        .active = tmp___0;
    (*(&mut timer_ppid_check as *mut ev_timer as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    timer_ppid_check
        .cb = Some(
        check_ppid
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_timer, libc::c_int) -> (),
    );
    memmove(
        &mut (*(&mut timer_ppid_check as *mut ev_timer as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut timer_ppid_check.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_timer, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_timer, libc::c_int) -> (),
            >,
        >() as libc::c_ulong,
    );
    (*(&mut timer_ppid_check as *mut ev_timer as *mut ev_watcher_time)).at = 1.0f64;
    timer_ppid_check.repeat = 1.0f64;
    ev_timer_start(loop_0, &mut timer_ppid_check);
    (*(&mut listener as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .pending = 0 as libc::c_int;
    (*(&mut listener as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .active = (*(&mut listener as *mut ev_io as *mut libc::c_void
        as *mut ev_watcher))
        .pending;
    (*(&mut listener as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
        .priority = 0 as libc::c_int;
    if (*CONFIG).PMODE as libc::c_uint == 1 as libc::c_uint {
        listener
            .cb = Some(
            handle_clear_accept
                as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
        );
    } else {
        listener
            .cb = Some(
            handle_accept
                as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
        );
    }
    memmove(
        &mut (*(&mut listener as *mut ev_io as *mut ev_watcher)).cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, libc::c_int) -> (),
            > as *mut libc::c_void,
        &mut listener.cb
            as *mut Option::<
                unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
            > as *const libc::c_void,
        ::std::mem::size_of::<
            Option::<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> ()>,
        >() as libc::c_ulong,
    );
    listener.fd = listener_socket;
    listener.events = 129 as libc::c_int;
    listener.data = default_ctx as *mut libc::c_void;
    ev_io_start(loop_0, &mut listener);
    ev_loop(loop_0, 0 as libc::c_int);
    fprintf(
        stderr,
        b"{core} Child %d exiting.\n\0" as *const u8 as *const libc::c_char,
        child_num,
    );
    if (*CONFIG).SYSLOG != 0 {
        syslog(
            3 as libc::c_int,
            b"{core} Child %d exiting.\n\0" as *const u8 as *const libc::c_char,
            child_num,
        );
    }
    exit(1 as libc::c_int);
}
pub unsafe extern "C" fn change_root() {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = chroot((*CONFIG).CHROOT as *const libc::c_char);
    if tmp == -(1 as libc::c_int) {
        fail(b"chroot\0" as *const u8 as *const libc::c_char);
    }
    tmp___0 = chdir(b"/\0" as *const u8 as *const libc::c_char);
    if tmp___0 != 0 {
        fail(b"chdir\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn drop_privileges() {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = setgid((*CONFIG).GID);
    if tmp != 0 {
        fail(b"setgid failed\0" as *const u8 as *const libc::c_char);
    }
    tmp___0 = setuid((*CONFIG).UID);
    if tmp___0 != 0 {
        fail(b"setuid failed\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn init_globals() {
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
    let mut gai_err: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    let mut spo: *mut shcupd_peer_opt = 0 as *mut shcupd_peer_opt;
    let mut pai: *mut *mut addrinfo = 0 as *mut *mut addrinfo;
    let mut gai_err___0: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___5: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___6: *mut libc::c_void = 0 as *mut libc::c_void;
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_family = 0 as libc::c_int;
    hints.ai_socktype = 1 as libc::c_int;
    hints.ai_flags = 0 as libc::c_int;
    tmp = getaddrinfo(
        (*CONFIG).BACK_IP as *const libc::c_char,
        (*CONFIG).BACK_PORT as *const libc::c_char,
        &mut hints as *mut addrinfo as *const addrinfo,
        &mut backaddr as *mut *mut addrinfo,
    );
    gai_err = tmp;
    if gai_err != 0 as libc::c_int {
        tmp___0 = gai_strerror(gai_err);
        fprintf(
            stderr,
            b"{getaddrinfo}: [%s]\0" as *const u8 as *const libc::c_char,
            tmp___0,
        );
        if (*CONFIG).SYSLOG != 0 {
            tmp___1 = gai_strerror(gai_err);
            syslog(
                3 as libc::c_int,
                b"{getaddrinfo}: [%s]\0" as *const u8 as *const libc::c_char,
                tmp___1,
            );
        }
        exit(1 as libc::c_int);
    }
    if (*CONFIG).SHARED_CACHE != 0 {
        spo = ((*CONFIG).SHCUPD_PEERS).as_mut_ptr();
        pai = shcupd_peers.as_mut_ptr();
        while !((*spo).ip).is_null() {
            memset(
                &mut hints as *mut addrinfo as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
            );
            hints.ai_family = 0 as libc::c_int;
            hints.ai_socktype = 2 as libc::c_int;
            hints.ai_flags = 0 as libc::c_int;
            if !((*spo).port).is_null() {
                tmp___2 = (*spo).port;
            } else {
                tmp___2 = (*CONFIG).SHCUPD_PORT;
            }
            tmp___3 = getaddrinfo(
                (*spo).ip as *const libc::c_char,
                tmp___2 as *const libc::c_char,
                &mut hints as *mut addrinfo as *const addrinfo,
                pai,
            );
            gai_err___0 = tmp___3;
            if gai_err___0 != 0 as libc::c_int {
                tmp___4 = gai_strerror(gai_err___0);
                fprintf(
                    stderr,
                    b"{getaddrinfo}: [%s]\0" as *const u8 as *const libc::c_char,
                    tmp___4,
                );
                if (*CONFIG).SYSLOG != 0 {
                    tmp___5 = gai_strerror(gai_err___0);
                    syslog(
                        3 as libc::c_int,
                        b"{getaddrinfo}: [%s]\0" as *const u8 as *const libc::c_char,
                        tmp___5,
                    );
                }
                exit(1 as libc::c_int);
            }
            spo = spo.offset(1);
            pai = pai.offset(1);
        }
    }
    tmp___6 = calloc(
        (*CONFIG).NCORES as size_t,
        ::std::mem::size_of::<pid_t>() as libc::c_ulong,
    );
    child_pids = tmp___6 as *mut pid_t;
    if child_pids as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fail(b"calloc\0" as *const u8 as *const libc::c_char);
    }
    if (*CONFIG).SYSLOG != 0 {
        openlog(
            b"stud\0" as *const u8 as *const libc::c_char,
            11 as libc::c_int,
            (*CONFIG).SYSLOG_FACILITY,
        );
    }
}
pub unsafe extern "C" fn start_children(
    mut start_index: libc::c_int,
    mut count: libc::c_int,
) {
    let mut pid: libc::c_int = 0;
    let mut tmp: __pid_t = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    if create_workers == 0 {
        return;
    }
    child_num = start_index;
    while child_num < start_index + count {
        tmp = fork();
        pid = tmp;
        if pid == -(1 as libc::c_int) {
            tmp___0 = __errno_location();
            tmp___1 = strerror(*tmp___0);
            fprintf(
                stderr,
                b"{core} fork() failed: %s; Goodbye cruel world!\n\0" as *const u8
                    as *const libc::c_char,
                tmp___1,
            );
            if (*CONFIG).SYSLOG != 0 {
                tmp___2 = __errno_location();
                tmp___3 = strerror(*tmp___2);
                syslog(
                    3 as libc::c_int,
                    b"{core} fork() failed: %s; Goodbye cruel world!\n\0" as *const u8
                        as *const libc::c_char,
                    tmp___3,
                );
            }
            exit(1 as libc::c_int);
        } else {
            if pid == 0 as libc::c_int {
                handle_connections();
                exit(0 as libc::c_int);
            } else {
                *child_pids.offset(child_num as isize) = pid;
            }
        }
        child_num += 1;
    }
}
pub unsafe extern "C" fn replace_child_with_pid(mut pid: pid_t) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_long) < (*CONFIG).NCORES {
        if *child_pids.offset(i as isize) == pid {
            start_children(i, 1 as libc::c_int);
            return;
        }
        i += 1;
    }
    fprintf(
        stderr,
        b"Cannot find index for child pid %d\0" as *const u8 as *const libc::c_char,
        pid,
    );
    if (*CONFIG).SYSLOG != 0 {
        syslog(
            3 as libc::c_int,
            b"Cannot find index for child pid %d\0" as *const u8 as *const libc::c_char,
            pid,
        );
    }
}
unsafe extern "C" fn do_wait(mut signo: libc::c_int) {
    let mut status: libc::c_int = 0;
    let mut pid: libc::c_int = 0;
    let mut tmp: __pid_t = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    tmp = wait(&mut status);
    pid = tmp;
    if pid == -(1 as libc::c_int) {
        tmp___1 = __errno_location();
        if *tmp___1 == 10 as libc::c_int {
            fprintf(
                stderr,
                b"{core} All children have exited! Restarting...\n\0" as *const u8
                    as *const libc::c_char,
            );
            if (*CONFIG).SYSLOG != 0 {
                syslog(
                    3 as libc::c_int,
                    b"{core} All children have exited! Restarting...\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            start_children(0 as libc::c_int, (*CONFIG).NCORES as libc::c_int);
        } else {
            tmp___0 = __errno_location();
            if *tmp___0 == 4 as libc::c_int {
                fprintf(
                    stderr,
                    b"{core} Interrupted wait\n\0" as *const u8 as *const libc::c_char,
                );
                if (*CONFIG).SYSLOG != 0 {
                    syslog(
                        3 as libc::c_int,
                        b"{core} Interrupted wait\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            } else {
                fail(b"wait\0" as *const u8 as *const libc::c_char);
            }
        }
    } else if status & 127 as libc::c_int == 0 as libc::c_int {
        fprintf(
            stderr,
            b"{core} Child %d exited with status %d. Replacing...\n\0" as *const u8
                as *const libc::c_char,
            pid,
            (status & 65280 as libc::c_int) >> 8 as libc::c_int,
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"{core} Child %d exited with status %d. Replacing...\n\0" as *const u8
                    as *const libc::c_char,
                pid,
                (status & 65280 as libc::c_int) >> 8 as libc::c_int,
            );
        }
        replace_child_with_pid(pid);
    } else if ((status & 127 as libc::c_int) + 1 as libc::c_int) as libc::c_schar
            as libc::c_int >> 1 as libc::c_int > 0 as libc::c_int
        {
        fprintf(
            stderr,
            b"{core} Child %d was terminated by signal %d. Replacing...\n\0" as *const u8
                as *const libc::c_char,
            pid,
            status & 127 as libc::c_int,
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"{core} Child %d was terminated by signal %d. Replacing...\n\0"
                    as *const u8 as *const libc::c_char,
                pid,
                status & 127 as libc::c_int,
            );
        }
        replace_child_with_pid(pid);
    }
}
unsafe extern "C" fn sigh_terminate(mut signo: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: __pid_t = 0;
    create_workers = 0 as libc::c_int;
    tmp___4 = getpid();
    if tmp___4 == master_pid {
        if (*CONFIG).QUIET == 0 {
            fprintf(
                stdout,
                b"{core} Received signal %d, shutting down.\n\0" as *const u8
                    as *const libc::c_char,
                signo,
            );
        }
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                6 as libc::c_int,
                b"{core} Received signal %d, shutting down.\n\0" as *const u8
                    as *const libc::c_char,
                signo,
            );
        }
        i = 0 as libc::c_int;
        while (i as libc::c_long) < (*CONFIG).NCORES {
            if *child_pids.offset(i as isize) > 1 as libc::c_int {
                tmp___3 = kill(*child_pids.offset(i as isize), 15 as libc::c_int);
                if tmp___3 != 0 as libc::c_int {
                    tmp = __errno_location();
                    tmp___0 = strerror(*tmp);
                    fprintf(
                        stderr,
                        b"{core} Unable to send SIGTERM to worker pid %d: %s\n\0"
                            as *const u8 as *const libc::c_char,
                        *child_pids.offset(i as isize),
                        tmp___0,
                    );
                    if (*CONFIG).SYSLOG != 0 {
                        tmp___1 = __errno_location();
                        tmp___2 = strerror(*tmp___1);
                        syslog(
                            3 as libc::c_int,
                            b"{core} Unable to send SIGTERM to worker pid %d: %s\n\0"
                                as *const u8 as *const libc::c_char,
                            *child_pids.offset(i as isize),
                            tmp___2,
                        );
                    }
                }
            }
            i += 1;
        }
    }
    exit(0 as libc::c_int);
}
pub unsafe extern "C" fn init_signals() {
    let mut act: sigaction = sigaction {
        __sigaction_handler: __anonunion___sigaction_handler_363639592 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___7: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___8: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___9: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___10: libc::c_int = 0;
    sigemptyset(&mut act.sa_mask);
    act.sa_flags = 0 as libc::c_int;
    act
        .__sigaction_handler
        .sa_handler = ::std::mem::transmute::<
        libc::intptr_t,
        Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    >(1 as libc::c_int as libc::intptr_t);
    tmp = sigaction(
        13 as libc::c_int,
        &mut act as *mut sigaction as *const sigaction,
        0 as *mut libc::c_void as *mut sigaction,
    );
    if tmp < 0 as libc::c_int {
        fail(b"sigaction - sigpipe\0" as *const u8 as *const libc::c_char);
    }
    act.sa_flags = 1 as libc::c_int;
    act
        .__sigaction_handler
        .sa_handler = Some(do_wait as unsafe extern "C" fn(libc::c_int) -> ());
    tmp___0 = sigaction(
        17 as libc::c_int,
        &mut act as *mut sigaction as *const sigaction,
        0 as *mut libc::c_void as *mut sigaction,
    );
    if tmp___0 < 0 as libc::c_int {
        fail(b"sigaction - sigchld\0" as *const u8 as *const libc::c_char);
    }
    act.sa_flags = 0 as libc::c_int;
    act
        .__sigaction_handler
        .sa_handler = Some(sigh_terminate as unsafe extern "C" fn(libc::c_int) -> ());
    tmp___5 = sigaction(
        2 as libc::c_int,
        &mut act as *mut sigaction as *const sigaction,
        0 as *mut libc::c_void as *mut sigaction,
    );
    if tmp___5 < 0 as libc::c_int {
        tmp___1 = __errno_location();
        tmp___2 = strerror(*tmp___1);
        fprintf(
            stderr,
            b"Unable to register SIGINT signal handler: %s\n\0" as *const u8
                as *const libc::c_char,
            tmp___2,
        );
        if (*CONFIG).SYSLOG != 0 {
            tmp___3 = __errno_location();
            tmp___4 = strerror(*tmp___3);
            syslog(
                3 as libc::c_int,
                b"Unable to register SIGINT signal handler: %s\n\0" as *const u8
                    as *const libc::c_char,
                tmp___4,
            );
        }
        exit(1 as libc::c_int);
    }
    tmp___10 = sigaction(
        15 as libc::c_int,
        &mut act as *mut sigaction as *const sigaction,
        0 as *mut libc::c_void as *mut sigaction,
    );
    if tmp___10 < 0 as libc::c_int {
        tmp___6 = __errno_location();
        tmp___7 = strerror(*tmp___6);
        fprintf(
            stderr,
            b"Unable to register SIGTERM signal handler: %s\n\0" as *const u8
                as *const libc::c_char,
            tmp___7,
        );
        if (*CONFIG).SYSLOG != 0 {
            tmp___8 = __errno_location();
            tmp___9 = strerror(*tmp___8);
            syslog(
                3 as libc::c_int,
                b"Unable to register SIGTERM signal handler: %s\n\0" as *const u8
                    as *const libc::c_char,
                tmp___9,
            );
        }
        exit(1 as libc::c_int);
    }
}
pub unsafe extern "C" fn daemonize() {
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: libc::c_int = 0;
    let mut pid: pid_t = 0;
    let mut tmp___4: __pid_t = 0;
    let mut tmp___5: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___7: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___8: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___9: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___10: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___11: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___12: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___13: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___14: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___15: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___16: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___17: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___18: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___19: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___20: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: pid_t = 0;
    let mut tmp___21: __pid_t = 0;
    let mut tmp___22: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___23: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___24: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___25: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___26: __pid_t = 0;
    let mut tmp___27: __pid_t = 0;
    tmp___3 = chdir(b"/\0" as *const u8 as *const libc::c_char);
    if tmp___3 != 0 as libc::c_int {
        tmp = __errno_location();
        tmp___0 = strerror(*tmp);
        fprintf(
            stderr,
            b"Unable change directory to /: %s\n\0" as *const u8 as *const libc::c_char,
            tmp___0,
        );
        if (*CONFIG).SYSLOG != 0 {
            tmp___1 = __errno_location();
            tmp___2 = strerror(*tmp___1);
            syslog(
                3 as libc::c_int,
                b"Unable change directory to /: %s\n\0" as *const u8
                    as *const libc::c_char,
                tmp___2,
            );
        }
        exit(1 as libc::c_int);
    }
    tmp___4 = fork();
    pid = tmp___4;
    if pid < 0 as libc::c_int {
        tmp___5 = __errno_location();
        tmp___6 = strerror(*tmp___5);
        fprintf(
            stderr,
            b"Unable to daemonize: fork failed: %s\n\0" as *const u8
                as *const libc::c_char,
            tmp___6,
        );
        if (*CONFIG).SYSLOG != 0 {
            tmp___7 = __errno_location();
            tmp___8 = strerror(*tmp___7);
            syslog(
                3 as libc::c_int,
                b"Unable to daemonize: fork failed: %s\n\0" as *const u8
                    as *const libc::c_char,
                tmp___8,
            );
        }
        exit(1 as libc::c_int);
    }
    if pid != 0 as libc::c_int {
        printf(
            b"{core} Daemonized as pid %d.\n\0" as *const u8 as *const libc::c_char,
            pid,
        );
        exit(0 as libc::c_int);
    }
    fclose(stdin);
    fclose(stdout);
    fclose(stderr);
    stdin = fopen(
        b"/dev/null\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if stdin as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___9 = __errno_location();
        tmp___10 = strerror(*tmp___9);
        fprintf(
            stderr,
            b"Unable to reopen stdin to %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"/dev/null\0" as *const u8 as *const libc::c_char,
            tmp___10,
        );
        if (*CONFIG).SYSLOG != 0 {
            tmp___11 = __errno_location();
            tmp___12 = strerror(*tmp___11);
            syslog(
                3 as libc::c_int,
                b"Unable to reopen stdin to %s: %s\n\0" as *const u8
                    as *const libc::c_char,
                b"/dev/null\0" as *const u8 as *const libc::c_char,
                tmp___12,
            );
        }
        exit(1 as libc::c_int);
    }
    stdout = fopen(
        b"/dev/null\0" as *const u8 as *const libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char,
    );
    if stdout as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___13 = __errno_location();
        tmp___14 = strerror(*tmp___13);
        fprintf(
            stderr,
            b"Unable to reopen stdout to %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"/dev/null\0" as *const u8 as *const libc::c_char,
            tmp___14,
        );
        if (*CONFIG).SYSLOG != 0 {
            tmp___15 = __errno_location();
            tmp___16 = strerror(*tmp___15);
            syslog(
                3 as libc::c_int,
                b"Unable to reopen stdout to %s: %s\n\0" as *const u8
                    as *const libc::c_char,
                b"/dev/null\0" as *const u8 as *const libc::c_char,
                tmp___16,
            );
        }
        exit(1 as libc::c_int);
    }
    stderr = fopen(
        b"/dev/null\0" as *const u8 as *const libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char,
    );
    if stderr as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___17 = __errno_location();
        tmp___18 = strerror(*tmp___17);
        fprintf(
            stderr,
            b"Unable to reopen stderr to %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"/dev/null\0" as *const u8 as *const libc::c_char,
            tmp___18,
        );
        if (*CONFIG).SYSLOG != 0 {
            tmp___19 = __errno_location();
            tmp___20 = strerror(*tmp___19);
            syslog(
                3 as libc::c_int,
                b"Unable to reopen stderr to %s: %s\n\0" as *const u8
                    as *const libc::c_char,
                b"/dev/null\0" as *const u8 as *const libc::c_char,
                tmp___20,
            );
        }
        exit(1 as libc::c_int);
    }
    tmp___21 = setsid();
    s = tmp___21;
    if s < 0 as libc::c_int {
        tmp___22 = __errno_location();
        tmp___23 = strerror(*tmp___22);
        fprintf(
            stderr,
            b"Unable to create new session, setsid(2) failed: %s :: %d\n\0" as *const u8
                as *const libc::c_char,
            tmp___23,
            s,
        );
        if (*CONFIG).SYSLOG != 0 {
            tmp___24 = __errno_location();
            tmp___25 = strerror(*tmp___24);
            syslog(
                3 as libc::c_int,
                b"Unable to create new session, setsid(2) failed: %s :: %d\n\0"
                    as *const u8 as *const libc::c_char,
                tmp___25,
                s,
            );
        }
        exit(1 as libc::c_int);
    }
    if (*CONFIG).QUIET == 0 {
        tmp___26 = getpid();
        fprintf(
            stdout,
            b"Successfully daemonized as pid %d.\n\0" as *const u8
                as *const libc::c_char,
            tmp___26,
        );
    }
    if (*CONFIG).SYSLOG != 0 {
        tmp___27 = getpid();
        syslog(
            6 as libc::c_int,
            b"Successfully daemonized as pid %d.\n\0" as *const u8
                as *const libc::c_char,
            tmp___27,
        );
    }
}
pub unsafe extern "C" fn openssl_check_version() {
    let mut tmp: libc::c_ulong = 0;
    tmp = SSLeay();
    openssl_version = tmp as libc::c_long;
    if (openssl_version ^ 268439903 as libc::c_long) & -(4081 as libc::c_long) != 0 {
        fprintf(
            stderr,
            b"WARNING: {core} OpenSSL version mismatch; stud was compiled with %lx, now using %lx.\n\0"
                as *const u8 as *const libc::c_char,
            268439903 as libc::c_ulong,
            openssl_version as libc::c_ulong,
        );
        if (*CONFIG).SYSLOG != 0 {
            syslog(
                3 as libc::c_int,
                b"WARNING: {core} OpenSSL version mismatch; stud was compiled with %lx, now using %lx.\n\0"
                    as *const u8 as *const libc::c_char,
                268439903 as libc::c_ulong,
                openssl_version as libc::c_ulong,
            );
        }
    }
    if (*CONFIG).QUIET == 0 {
        fprintf(
            stdout,
            b"{core} Using OpenSSL version %lx.\n\0" as *const u8 as *const libc::c_char,
            openssl_version as libc::c_ulong,
        );
    }
    if (*CONFIG).SYSLOG != 0 {
        syslog(
            6 as libc::c_int,
            b"{core} Using OpenSSL version %lx.\n\0" as *const u8 as *const libc::c_char,
            openssl_version as libc::c_ulong,
        );
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    CONFIG = config_new();
    config_parse_cli(argc, argv, CONFIG);
    create_workers = 1 as libc::c_int;
    openssl_check_version();
    init_signals();
    init_globals();
    listener_socket = create_main_socket();
    if !((*CONFIG).SHCUPD_PORT).is_null() {
        shcupd_socket = create_shcupd_socket();
    }
    init_openssl();
    if !((*CONFIG).CHROOT).is_null() {
        if *((*CONFIG).CHROOT).offset(0 as libc::c_int as isize) != 0 {
            change_root();
        }
    }
    if (*CONFIG).UID != 0 {
        drop_privileges();
    } else if (*CONFIG).GID != 0 {
        drop_privileges();
    }
    if (*CONFIG).DAEMONIZE != 0 {
        (*CONFIG).QUIET = 1 as libc::c_int;
        (*CONFIG).SYSLOG = 1 as libc::c_int;
        daemonize();
    }
    master_pid = getpid();
    start_children(0 as libc::c_int, (*CONFIG).NCORES as libc::c_int);
    if !((*CONFIG).SHCUPD_PORT).is_null() {
        loop_0 = ev_default_loop(0 as libc::c_uint);
        (*(&mut shcupd_listener as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
            .pending = 0 as libc::c_int;
        (*(&mut shcupd_listener as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
            .active = (*(&mut shcupd_listener as *mut ev_io as *mut libc::c_void
            as *mut ev_watcher))
            .pending;
        (*(&mut shcupd_listener as *mut ev_io as *mut libc::c_void as *mut ev_watcher))
            .priority = 0 as libc::c_int;
        shcupd_listener
            .cb = Some(
            handle_shcupd
                as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
        );
        memmove(
            &mut (*(&mut shcupd_listener as *mut ev_io as *mut ev_watcher)).cb
                as *mut Option::<
                    unsafe extern "C" fn(
                        *mut ev_loop,
                        *mut ev_watcher,
                        libc::c_int,
                    ) -> (),
                > as *mut libc::c_void,
            &mut shcupd_listener.cb
                as *mut Option::<
                    unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
                > as *const libc::c_void,
            ::std::mem::size_of::<
                Option::<
                    unsafe extern "C" fn(*mut ev_loop, *mut ev_io, libc::c_int) -> (),
                >,
            >() as libc::c_ulong,
        );
        shcupd_listener.fd = shcupd_socket;
        shcupd_listener.events = 129 as libc::c_int;
        ev_io_start(loop_0, &mut shcupd_listener);
        ev_loop(loop_0, 0 as libc::c_int);
    }
    loop {
        pause();
    };
}
pub unsafe extern "C" fn ringbuffer_init(mut rb: *mut ringbuffer) {
    let mut x: libc::c_int = 0;
    (*rb)
        .head = &mut *((*rb).slots).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut bufent;
    (*rb)
        .tail = &mut *((*rb).slots).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut bufent;
    (*rb).used = 0 as libc::c_int as size_t;
    x = 0 as libc::c_int;
    while x < 3 as libc::c_int {
        (*rb)
            .slots[x as usize]
            .next = &mut *((*rb).slots)
            .as_mut_ptr()
            .offset(((x + 1 as libc::c_int) % 3 as libc::c_int) as isize) as *mut bufent;
        x += 1;
    }
}
pub unsafe extern "C" fn ringbuffer_read_next(
    mut rb: *mut ringbuffer,
    mut length: *mut libc::c_int,
) -> *mut libc::c_char {
    if (*rb).used == 0 {
        __assert_fail(
            b"rb->used\0" as *const u8 as *const libc::c_char,
            b"ringbuffer.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_uint,
            b"ringbuffer_read_next\0" as *const u8 as *const libc::c_char,
        );
    }
    *length = (*(*rb).head).left as libc::c_int;
    return (*(*rb).head).ptr;
}
pub unsafe extern "C" fn ringbuffer_read_skip(
    mut rb: *mut ringbuffer,
    mut length: libc::c_int,
) {
    if (*rb).used == 0 {
        __assert_fail(
            b"rb->used\0" as *const u8 as *const libc::c_char,
            b"ringbuffer.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_uint,
            b"ringbuffer_read_skip\0" as *const u8 as *const libc::c_char,
        );
    }
    (*(*rb).head).ptr = ((*(*rb).head).ptr).offset(length as isize);
    (*(*rb).head)
        .left = ((*(*rb).head).left as libc::c_ulong).wrapping_sub(length as size_t)
        as size_t as size_t;
}
pub unsafe extern "C" fn ringbuffer_read_pop(mut rb: *mut ringbuffer) {
    if (*rb).used == 0 {
        __assert_fail(
            b"rb->used\0" as *const u8 as *const libc::c_char,
            b"ringbuffer.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_uint,
            b"ringbuffer_read_pop\0" as *const u8 as *const libc::c_char,
        );
    }
    (*rb).head = (*(*rb).head).next;
    (*rb).used = ((*rb).used).wrapping_sub(1);
}
pub unsafe extern "C" fn ringbuffer_write_ptr(
    mut rb: *mut ringbuffer,
) -> *mut libc::c_char {
    if !((*rb).used < 3 as libc::c_ulong) {
        __assert_fail(
            b"rb->used < RING_SLOTS\0" as *const u8 as *const libc::c_char,
            b"ringbuffer.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_uint,
            b"ringbuffer_write_ptr\0" as *const u8 as *const libc::c_char,
        );
    }
    return ((*(*rb).tail).data).as_mut_ptr();
}
pub unsafe extern "C" fn ringbuffer_write_append(
    mut rb: *mut ringbuffer,
    mut length: libc::c_int,
) {
    if !((*rb).used < 3 as libc::c_ulong) {
        __assert_fail(
            b"rb->used < RING_SLOTS\0" as *const u8 as *const libc::c_char,
            b"ringbuffer.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_uint,
            b"ringbuffer_write_append\0" as *const u8 as *const libc::c_char,
        );
    }
    (*rb).used = ((*rb).used).wrapping_add(1);
    (*(*rb).tail).ptr = ((*(*rb).tail).data).as_mut_ptr();
    (*(*rb).tail).left = length as size_t;
    (*rb).tail = (*(*rb).tail).next;
}
pub unsafe extern "C" fn ringbuffer_size(mut rb: *mut ringbuffer) -> libc::c_int {
    return (*rb).used as libc::c_int;
}
pub unsafe extern "C" fn ringbuffer_capacity(mut rb: *mut ringbuffer) -> libc::c_int {
    return 3 as libc::c_int;
}
pub unsafe extern "C" fn ringbuffer_is_empty(mut rb: *mut ringbuffer) -> libc::c_int {
    return ((*rb).used == 0 as libc::c_ulong) as libc::c_int;
}
pub unsafe extern "C" fn ringbuffer_is_full(mut rb: *mut ringbuffer) -> libc::c_int {
    return ((*rb).used == 3 as libc::c_ulong) as libc::c_int;
}
static mut var_buf: [libc::c_char; 1024] = [0; 1024];
static mut val_buf: [libc::c_char; 1024] = [0; 1024];
static mut error_buf: [libc::c_char; 1024] = [0; 1024];
static mut tmp_buf: [libc::c_char; 150] = [0; 150];
unsafe extern "C" fn config_error_set(mut fmt: *mut libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    memset(
        error_buf.as_mut_ptr() as *mut libc::c_void,
        '\u{0}' as i32,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    args_0 = args.clone();
    vsnprintf(
        error_buf.as_mut_ptr(),
        (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_ulong),
        fmt as *const libc::c_char,
        args_0.as_va_list(),
    );
}
pub unsafe extern "C" fn config_error_get() -> *mut libc::c_char {
    return error_buf.as_mut_ptr();
}
pub unsafe extern "C" fn config_die(mut fmt: *mut libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    vfprintf(stderr, fmt as *const libc::c_char, args_0.as_va_list());
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    exit(1 as libc::c_int);
}
pub unsafe extern "C" fn config_new() -> *mut stud_config {
    let mut r: *mut stud_config = 0 as *mut stud_config;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    r = 0 as *mut libc::c_void as *mut stud_config;
    tmp = malloc(::std::mem::size_of::<stud_config>() as libc::c_ulong);
    r = tmp as *mut stud_config;
    if r as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___0 = __errno_location();
        tmp___1 = strerror(*tmp___0);
        config_error_set(
            b"Unable to allocate memory for configuration structure: %s\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            tmp___1,
        );
        return 0 as *mut libc::c_void as *mut stud_config;
    }
    (*r).ETYPE = ENC_TLS;
    (*r).PMODE = SSL_SERVER;
    (*r).WRITE_IP_OCTET = 0 as libc::c_int;
    (*r).WRITE_PROXY_LINE = 0 as libc::c_int;
    (*r).PROXY_PROXY_LINE = 0 as libc::c_int;
    (*r).CHROOT = 0 as *mut libc::c_void as *mut libc::c_char;
    (*r).UID = 0 as libc::c_int as uid_t;
    (*r).GID = 0 as libc::c_int as gid_t;
    (*r).FRONT_IP = 0 as *mut libc::c_void as *mut libc::c_char;
    (*r).FRONT_PORT = strdup(b"8443\0" as *const u8 as *const libc::c_char);
    (*r).BACK_IP = strdup(b"127.0.0.1\0" as *const u8 as *const libc::c_char);
    (*r).BACK_PORT = strdup(b"8000\0" as *const u8 as *const libc::c_char);
    (*r).NCORES = 1 as libc::c_long;
    (*r).CERT_FILES = 0 as *mut libc::c_void as *mut cert_files;
    (*r).CIPHER_SUITE = 0 as *mut libc::c_void as *mut libc::c_char;
    (*r).ENGINE = 0 as *mut libc::c_void as *mut libc::c_char;
    (*r).BACKLOG = 100 as libc::c_int;
    (*r).SHARED_CACHE = 0 as libc::c_int;
    (*r).SHCUPD_IP = 0 as *mut libc::c_void as *mut libc::c_char;
    (*r).SHCUPD_PORT = 0 as *mut libc::c_void as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < 15 as libc::c_int {
        memset(
            &mut *((*r).SHCUPD_PEERS).as_mut_ptr().offset(i as isize)
                as *mut shcupd_peer_opt as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<shcupd_peer_opt>() as libc::c_ulong,
        );
        i += 1;
    }
    (*r).SHCUPD_MCASTIF = 0 as *mut libc::c_void as *mut libc::c_char;
    (*r).SHCUPD_MCASTTTL = 0 as *mut libc::c_void as *mut libc::c_char;
    (*r).QUIET = 0 as libc::c_int;
    (*r).SYSLOG = 0 as libc::c_int;
    (*r).SYSLOG_FACILITY = (3 as libc::c_int) << 3 as libc::c_int;
    (*r).TCP_KEEPALIVE_TIME = 3600 as libc::c_int;
    (*r).DAEMONIZE = 0 as libc::c_int;
    (*r).PREFER_SERVER_CIPHERS = 0 as libc::c_int;
    return r;
}
pub unsafe extern "C" fn config_destroy(mut cfg: *mut stud_config) {
    let mut curr: *mut cert_files = 0 as *mut cert_files;
    let mut next: *mut cert_files = 0 as *mut cert_files;
    let mut i: libc::c_int = 0;
    if cfg as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    if (*cfg).CHROOT as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        free((*cfg).CHROOT as *mut libc::c_void);
    }
    if (*cfg).FRONT_IP as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        free((*cfg).FRONT_IP as *mut libc::c_void);
    }
    if (*cfg).FRONT_PORT as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        free((*cfg).FRONT_PORT as *mut libc::c_void);
    }
    if (*cfg).BACK_IP as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        free((*cfg).BACK_IP as *mut libc::c_void);
    }
    if (*cfg).BACK_PORT as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        free((*cfg).BACK_PORT as *mut libc::c_void);
    }
    if (*cfg).CERT_FILES as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        curr = (*cfg).CERT_FILES;
        while (*cfg).CERT_FILES as libc::c_ulong
            != 0 as *mut libc::c_void as libc::c_ulong
        {
            next = (*curr).NEXT;
            free(curr as *mut libc::c_void);
            curr = next;
        }
    }
    if (*cfg).CIPHER_SUITE as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        free((*cfg).CIPHER_SUITE as *mut libc::c_void);
    }
    if (*cfg).ENGINE as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        free((*cfg).ENGINE as *mut libc::c_void);
    }
    if (*cfg).SHCUPD_IP as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        free((*cfg).SHCUPD_IP as *mut libc::c_void);
    }
    if (*cfg).SHCUPD_PORT as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        free((*cfg).SHCUPD_PORT as *mut libc::c_void);
    }
    i = 0 as libc::c_int;
    while i < 15 as libc::c_int {
        if (*cfg).SHCUPD_PEERS[i as usize].ip as libc::c_ulong
            != 0 as *mut libc::c_void as libc::c_ulong
        {
            free((*cfg).SHCUPD_PEERS[i as usize].ip as *mut libc::c_void);
        }
        if (*cfg).SHCUPD_PEERS[i as usize].port as libc::c_ulong
            != 0 as *mut libc::c_void as libc::c_ulong
        {
            free((*cfg).SHCUPD_PEERS[i as usize].port as *mut libc::c_void);
        }
        i += 1;
    }
    if (*cfg).SHCUPD_MCASTIF as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong
    {
        free((*cfg).SHCUPD_MCASTIF as *mut libc::c_void);
    }
    if (*cfg).SHCUPD_MCASTTTL as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong
    {
        free((*cfg).SHCUPD_MCASTTTL as *mut libc::c_void);
    }
    free(cfg as *mut libc::c_void);
}
pub unsafe extern "C" fn config_get_param(
    mut str: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut tmp___2: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut tmp___3: size_t = 0;
    if str as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    tmp = strlen(str as *const libc::c_char);
    if tmp < 1 as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut libc::c_char
    } else {
        if *str.offset(0 as libc::c_int as isize) as libc::c_int == 10 as libc::c_int {
            return 0 as *mut libc::c_void as *mut libc::c_char
        } else {
            tmp___0 = strcmp(
                str as *const libc::c_char,
                b"\r\n\0" as *const u8 as *const libc::c_char,
            );
            if tmp___0 == 0 as libc::c_int {
                return 0 as *mut libc::c_void as *mut libc::c_char;
            }
        }
    }
    ptr = str;
    if *str.offset(0 as libc::c_int as isize) as libc::c_int == 35 as libc::c_int {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    while ptr as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp___1 = __ctype_b_loc();
        if *(*tmp___1).offset(*ptr as libc::c_int as isize) as libc::c_int
            & 1024 as libc::c_int != 0
        {
            break;
        }
        ptr = ptr.offset(1);
    }
    memset(
        var_buf.as_mut_ptr() as *mut libc::c_void,
        '\u{0}' as i32,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while ptr as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp___2 = __ctype_b_loc();
        if *(*tmp___2).offset(*ptr as libc::c_int as isize) as libc::c_int
            & 8 as libc::c_int == 0
        {
            if !(*ptr as libc::c_int == 45 as libc::c_int) {
                break;
            }
        }
        var_buf[i as usize] = *ptr;
        i += 1;
        ptr = ptr.offset(1);
    }
    tmp___3 = strlen(var_buf.as_mut_ptr() as *const libc::c_char);
    if tmp___3 < 1 as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    return var_buf.as_mut_ptr();
}
pub unsafe extern "C" fn config_get_value(
    mut str: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut tmp___3: size_t = 0;
    i = 0 as libc::c_int;
    if str as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    tmp = strlen(str as *const libc::c_char);
    if tmp < 1 as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    ptr = str;
    while ptr as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        if !(*ptr as libc::c_int != 61 as libc::c_int) {
            break;
        }
        ptr = ptr.offset(1);
    }
    ptr = ptr.offset(1);
    while ptr as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp___0 = __ctype_b_loc();
        if *(*tmp___0).offset(*ptr as libc::c_int as isize) as libc::c_int
            & 32768 as libc::c_int != 0
        {
            break;
        }
        ptr = ptr.offset(1);
    }
    if ptr as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    memset(
        val_buf.as_mut_ptr() as *mut libc::c_void,
        '\u{0}' as i32,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    while ptr as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp___2 = __ctype_b_loc();
        if *(*tmp___2).offset(*ptr as libc::c_int as isize) as libc::c_int
            & 32768 as libc::c_int == 0
        {
            break;
        }
        tmp___1 = i;
        i += 1;
        val_buf[tmp___1 as usize] = *ptr;
        ptr = ptr.offset(1);
    }
    tmp___3 = strlen(val_buf.as_mut_ptr() as *const libc::c_char);
    if tmp___3 < 1 as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    return val_buf.as_mut_ptr();
}
pub unsafe extern "C" fn str_rtrim(mut str: *mut libc::c_char) -> *mut libc::c_char {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    tmp = strlen(str as *const libc::c_char);
    len = tmp as libc::c_int;
    ptr = str.offset(len as isize).offset(-(1 as libc::c_int as isize));
    while ptr as libc::c_ulong >= str as libc::c_ulong {
        tmp___0 = __ctype_b_loc();
        if *(*tmp___0).offset(*ptr as libc::c_int as isize) as libc::c_int
            & 8192 as libc::c_int == 0
        {
            if !(*ptr as libc::c_int == 34 as libc::c_int) {
                if !(*ptr as libc::c_int == 39 as libc::c_int) {
                    break;
                }
            }
        }
        ptr = ptr.offset(-1);
    }
    *ptr.offset(1 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    return str;
}
pub unsafe extern "C" fn str_ltrim(mut str: *mut libc::c_char) -> *mut libc::c_char {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut tmp: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut tmp___0: size_t = 0;
    ptr = str;
    while *ptr != 0 {
        tmp = __ctype_b_loc();
        if *(*tmp).offset(*ptr as libc::c_int as isize) as libc::c_int
            & 8192 as libc::c_int == 0
        {
            if !(*ptr as libc::c_int == 34 as libc::c_int) {
                if !(*ptr as libc::c_int == 39 as libc::c_int) {
                    break;
                }
            }
        }
        ptr = ptr.offset(1);
    }
    tmp___0 = strlen(ptr as *const libc::c_char);
    len = tmp___0 as libc::c_int;
    memmove(
        str as *mut libc::c_void,
        ptr as *const libc::c_void,
        (len + 1 as libc::c_int) as size_t,
    );
    return str;
}
pub unsafe extern "C" fn str_trim(mut str: *mut libc::c_char) -> *mut libc::c_char {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    ptr = str_rtrim(str);
    str = str_ltrim(ptr);
    return str;
}
pub unsafe extern "C" fn config_assign_str(
    mut dst: *mut *mut libc::c_char,
    mut v: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: size_t = 0;
    if *dst as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        if v as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            tmp = strlen(v as *const libc::c_char);
            if tmp > 0 as libc::c_ulong {
                *dst = strdup(v as *const libc::c_char);
            }
        }
    } else if v as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp___2 = strlen(v as *const libc::c_char);
        if tmp___2 > 0 as libc::c_ulong {
            tmp___0 = strlen(v as *const libc::c_char);
            memset(
                *dst as *mut libc::c_void,
                '\u{0}' as i32,
                tmp___0.wrapping_add(1 as libc::c_ulong),
            );
            tmp___1 = strlen(v as *const libc::c_char);
            memcpy(*dst as *mut libc::c_void, v as *const libc::c_void, tmp___1);
        } else {
            free(*dst as *mut libc::c_void);
        }
    } else {
        free(*dst as *mut libc::c_void);
    }
    return *dst;
}
pub unsafe extern "C" fn config_param_val_bool(
    mut val: *mut libc::c_char,
    mut res: *mut libc::c_int,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    if val as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as libc::c_int;
    }
    tmp = strcasecmp(
        val as *const libc::c_char,
        b"on\0" as *const u8 as *const libc::c_char,
    );
    if tmp == 0 as libc::c_int {
        *res = 1 as libc::c_int;
    } else {
        tmp___0 = strcasecmp(
            val as *const libc::c_char,
            b"yes\0" as *const u8 as *const libc::c_char,
        );
        if tmp___0 == 0 as libc::c_int {
            *res = 1 as libc::c_int;
        } else {
            tmp___1 = strcasecmp(
                val as *const libc::c_char,
                b"y\0" as *const u8 as *const libc::c_char,
            );
            if tmp___1 == 0 as libc::c_int {
                *res = 1 as libc::c_int;
            } else {
                tmp___2 = strcasecmp(
                    val as *const libc::c_char,
                    b"true\0" as *const u8 as *const libc::c_char,
                );
                if tmp___2 == 0 as libc::c_int {
                    *res = 1 as libc::c_int;
                } else {
                    tmp___3 = strcasecmp(
                        val as *const libc::c_char,
                        b"t\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___3 == 0 as libc::c_int {
                        *res = 1 as libc::c_int;
                    } else {
                        tmp___4 = strcasecmp(
                            val as *const libc::c_char,
                            b"1\0" as *const u8 as *const libc::c_char,
                        );
                        if tmp___4 == 0 as libc::c_int {
                            *res = 1 as libc::c_int;
                        }
                    }
                }
            }
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn config_param_val_str(
    mut val: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = strdup(val as *const libc::c_char);
    return tmp;
}
pub unsafe extern "C" fn config_param_host_port_wildcard(
    mut str: *mut libc::c_char,
    mut addr: *mut *mut libc::c_char,
    mut port: *mut *mut libc::c_char,
    mut wildcard_okay: libc::c_int,
) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut port_buf: [libc::c_char; 6] = [0; 6];
    let mut addr_buf: [libc::c_char; 150] = [0; 150];
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut addr_len: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    if str as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp = strlen(str as *const libc::c_char);
        tmp___0 = tmp;
    } else {
        tmp___0 = 0 as libc::c_int as size_t;
    }
    len = tmp___0 as libc::c_int;
    if str as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        config_error_set(
            b"Invalid/unset host/port string.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 0 as libc::c_int;
    } else {
        if len == 0 {
            config_error_set(
                b"Invalid/unset host/port string.\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return 0 as libc::c_int;
        }
    }
    memset(
        port_buf.as_mut_ptr() as *mut libc::c_void,
        '\u{0}' as i32,
        ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
    );
    memset(
        addr_buf.as_mut_ptr() as *mut libc::c_void,
        '\u{0}' as i32,
        ::std::mem::size_of::<[libc::c_char; 150]>() as libc::c_ulong,
    );
    if *str as libc::c_int == 91 as libc::c_int {
        ptr = str.offset(1 as libc::c_int as isize);
        tmp___1 = strrchr(ptr as *const libc::c_char, ']' as i32);
        x = tmp___1;
        if x as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            config_error_set(
                b"Invalid address '%s'.\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                str,
            );
            return 0 as libc::c_int;
        }
        memcpy(
            addr_buf.as_mut_ptr() as *mut libc::c_void,
            ptr as *const libc::c_void,
            x.offset_from(ptr) as libc::c_long as size_t,
        );
        x = x.offset(2 as libc::c_int as isize);
        memcpy(
            port_buf.as_mut_ptr() as *mut libc::c_void,
            x as *const libc::c_void,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_ulong),
        );
    } else {
        tmp___2 = strrchr(str as *const libc::c_char, ',' as i32);
        x___0 = tmp___2;
        if x___0 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            config_error_set(
                b"Invalid address string '%s'\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                str,
            );
            return 0 as libc::c_int;
        }
        addr_len = x___0.offset_from(str) as libc::c_long as libc::c_int;
        memcpy(
            addr_buf.as_mut_ptr() as *mut libc::c_void,
            str as *const libc::c_void,
            addr_len as size_t,
        );
        x___0 = x___0.offset(1);
        memcpy(
            port_buf.as_mut_ptr() as *mut libc::c_void,
            x___0 as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
        );
    }
    tmp___3 = atoi(port_buf.as_mut_ptr() as *const libc::c_char);
    p = tmp___3;
    if p < 1 as libc::c_int {
        config_error_set(
            b"Invalid port number '%s'\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            port_buf.as_mut_ptr(),
        );
        return 0 as libc::c_int;
    } else {
        if p > 65536 as libc::c_int {
            config_error_set(
                b"Invalid port number '%s'\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                port_buf.as_mut_ptr(),
            );
            return 0 as libc::c_int;
        }
    }
    tmp___4 = strcmp(
        addr_buf.as_mut_ptr() as *const libc::c_char,
        b"*\0" as *const u8 as *const libc::c_char,
    );
    if tmp___4 == 0 as libc::c_int {
        if wildcard_okay != 0 {
            free(*addr as *mut libc::c_void);
        } else {
            config_error_set(
                b"Invalid address: wildcards are not allowed.\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            return 0 as libc::c_int;
        }
    } else {
        *addr = strdup(addr_buf.as_mut_ptr() as *const libc::c_char);
    }
    *port = strdup(port_buf.as_mut_ptr() as *const libc::c_char);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn config_param_host_port(
    mut str: *mut libc::c_char,
    mut addr: *mut *mut libc::c_char,
    mut port: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = config_param_host_port_wildcard(str, addr, port, 0 as libc::c_int);
    return tmp;
}
pub unsafe extern "C" fn config_param_val_int(
    mut str: *mut libc::c_char,
    mut dst: *mut libc::c_int,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    if str as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp = atoi(str as *const libc::c_char);
        *dst = tmp;
    } else {
        *dst = 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn config_param_val_int_pos(
    mut str: *mut libc::c_char,
    mut dst: *mut libc::c_int,
) -> libc::c_int {
    let mut num: libc::c_int = 0;
    num = 0 as libc::c_int;
    if str as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        num = atoi(str as *const libc::c_char);
    }
    if num < 1 as libc::c_int {
        config_error_set(
            b"Not a positive number.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 0 as libc::c_int;
    }
    *dst = num;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn config_param_val_intl(
    mut str: *mut libc::c_char,
    mut dst: *mut libc::c_long,
) -> libc::c_int {
    let mut tmp: libc::c_long = 0;
    if str as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp = atol(str as *const libc::c_char);
        *dst = tmp;
    } else {
        *dst = 0 as libc::c_long;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn config_param_val_intl_pos(
    mut str: *mut libc::c_char,
    mut dst: *mut libc::c_long,
) -> libc::c_int {
    let mut num: libc::c_long = 0;
    num = 0 as libc::c_long;
    if str as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        num = atol(str as *const libc::c_char);
    }
    if num < 1 as libc::c_long {
        config_error_set(
            b"Not a positive number.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 0 as libc::c_int;
    }
    *dst = num;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn config_param_shcupd_mcastif(
    mut str: *mut libc::c_char,
    mut iface: *mut *mut libc::c_char,
    mut ttl: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut buf: [libc::c_char; 150] = [0; 150];
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: size_t = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    tmp = strlen(str as *const libc::c_char);
    if tmp >= ::std::mem::size_of::<[libc::c_char; 150]>() as libc::c_ulong {
        config_error_set(
            b"Invalid option for IFACE[,TTL]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 0 as libc::c_int;
    }
    sp = strchr(str as *const libc::c_char, ',' as i32);
    if sp.is_null() {
        tmp___0 = strcmp(
            str as *const libc::c_char,
            b"*\0" as *const u8 as *const libc::c_char,
        );
        if tmp___0 != 0 {
            *iface = str;
        } else {
            *iface = 0 as *mut libc::c_void as *mut libc::c_char;
        }
        *ttl = 0 as *mut libc::c_void as *mut libc::c_char;
        return 1 as libc::c_int;
    } else {
        tmp___1 = strncmp(
            str as *const libc::c_char,
            b"*\0" as *const u8 as *const libc::c_char,
            sp.offset_from(str) as libc::c_long as size_t,
        );
        if tmp___1 != 0 {
            *sp = 0 as libc::c_int as libc::c_char;
            *iface = str;
        } else {
            *iface = 0 as *mut libc::c_void as *mut libc::c_char;
        }
    }
    *ttl = sp.offset(1 as libc::c_int as isize);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn config_param_shcupd_peer(
    mut str: *mut libc::c_char,
    mut cfg: *mut stud_config,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut addr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut port: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: libc::c_int = 0;
    if cfg as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        config_error_set(
            b"Configuration pointer is NULL.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 0 as libc::c_int;
    }
    r = 1 as libc::c_int;
    offset = 0 as libc::c_int;
    i = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 15 as libc::c_int {
        if (*cfg).SHCUPD_PEERS[i as usize].ip as libc::c_ulong
            == 0 as *mut libc::c_void as libc::c_ulong
        {
            if (*cfg).SHCUPD_PEERS[i as usize].port as libc::c_ulong
                == 0 as *mut libc::c_void as libc::c_ulong
            {
                offset = i;
                break;
            }
        }
        i += 1;
    }
    if i >= 15 as libc::c_int {
        config_error_set(
            b"Reached maximum number of shared cache update peers (%d).\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            15 as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    tmp = malloc(150 as libc::c_int as size_t);
    addr = tmp as *mut libc::c_char;
    if addr as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___0 = __errno_location();
        tmp___1 = strerror(*tmp___0);
        config_error_set(
            b"Unable to allocate memory for new shared cache update peer address: %s\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            tmp___1,
        );
        r = 0 as libc::c_int;
    } else {
        memset(addr as *mut libc::c_void, '\u{0}' as i32, 150 as libc::c_int as size_t);
        tmp___2 = malloc(6 as libc::c_int as size_t);
        port = tmp___2 as *mut libc::c_char;
        if port as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            tmp___3 = __errno_location();
            tmp___4 = strerror(*tmp___3);
            config_error_set(
                b"Unable to allocate memory for new shared cache update peer port: %s\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                tmp___4,
            );
            r = 0 as libc::c_int;
        } else {
            memset(
                port as *mut libc::c_void,
                '\u{0}' as i32,
                6 as libc::c_int as size_t,
            );
            tmp___5 = config_param_host_port(str, &mut addr, &mut port);
            if tmp___5 == 0 {
                r = 0 as libc::c_int;
            }
        }
    }
    if r == 0 {
        if addr as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            free(addr as *mut libc::c_void);
        }
        if port as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            free(port as *mut libc::c_void);
        }
    } else {
        (*cfg).SHCUPD_PEERS[offset as usize].ip = addr;
        (*cfg).SHCUPD_PEERS[offset as usize].port = port;
    }
    return r;
}
pub unsafe extern "C" fn config_param_validate(
    mut k: *mut libc::c_char,
    mut v: *mut libc::c_char,
    mut cfg: *mut stud_config,
    mut file: *mut libc::c_char,
    mut line: libc::c_int,
) {
    let mut r: libc::c_int = 0;
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
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: size_t = 0;
    let mut passwd: *mut passwd = 0 as *mut passwd;
    let mut tmp___8: size_t = 0;
    let mut grp: *mut group = 0 as *mut group;
    let mut tmp___9: size_t = 0;
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
    let mut tmp___27: libc::c_int = 0;
    let mut tmp___28: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___29: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cert: *mut cert_files = 0 as *mut cert_files;
    let mut tmp___30: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___31: libc::c_int = 0;
    let mut tmp___32: size_t = 0;
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
    let mut tmp___50: libc::c_int = 0;
    let mut tmp___51: libc::c_int = 0;
    let mut tmp___52: libc::c_int = 0;
    let mut tmp___53: libc::c_int = 0;
    let mut tmp___54: libc::c_int = 0;
    let mut tmp___55: libc::c_int = 0;
    let mut tmp___56: libc::c_int = 0;
    let mut tmp___57: libc::c_int = 0;
    let mut tmp___58: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___59: *mut libc::c_char = 0 as *mut libc::c_char;
    r = 1 as libc::c_int;
    tmp___57 = strcmp(
        k as *const libc::c_char,
        b"tls\0" as *const u8 as *const libc::c_char,
    );
    if tmp___57 == 0 as libc::c_int {
        (*cfg).ETYPE = ENC_TLS;
    } else {
        tmp___56 = strcmp(
            k as *const libc::c_char,
            b"ssl\0" as *const u8 as *const libc::c_char,
        );
        if tmp___56 == 0 as libc::c_int {
            (*cfg).ETYPE = ENC_SSL;
        } else {
            tmp___55 = strcmp(
                k as *const libc::c_char,
                b"ciphers\0" as *const u8 as *const libc::c_char,
            );
            if tmp___55 == 0 as libc::c_int {
                if v as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                    tmp = strlen(v as *const libc::c_char);
                    if tmp > 0 as libc::c_ulong {
                        config_assign_str(&mut (*cfg).CIPHER_SUITE, v);
                    }
                }
            } else {
                tmp___54 = strcmp(
                    k as *const libc::c_char,
                    b"ssl-engine\0" as *const u8 as *const libc::c_char,
                );
                if tmp___54 == 0 as libc::c_int {
                    if v as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                        tmp___0 = strlen(v as *const libc::c_char);
                        if tmp___0 > 0 as libc::c_ulong {
                            config_assign_str(&mut (*cfg).ENGINE, v);
                        }
                    }
                } else {
                    tmp___53 = strcmp(
                        k as *const libc::c_char,
                        b"prefer-server-ciphers\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___53 == 0 as libc::c_int {
                        r = config_param_val_bool(v, &mut (*cfg).PREFER_SERVER_CIPHERS);
                    } else {
                        tmp___52 = strcmp(
                            k as *const libc::c_char,
                            b"frontend\0" as *const u8 as *const libc::c_char,
                        );
                        if tmp___52 == 0 as libc::c_int {
                            r = config_param_host_port_wildcard(
                                v,
                                &mut (*cfg).FRONT_IP,
                                &mut (*cfg).FRONT_PORT,
                                1 as libc::c_int,
                            );
                        } else {
                            tmp___51 = strcmp(
                                k as *const libc::c_char,
                                b"backend\0" as *const u8 as *const libc::c_char,
                            );
                            if tmp___51 == 0 as libc::c_int {
                                r = config_param_host_port(
                                    v,
                                    &mut (*cfg).BACK_IP,
                                    &mut (*cfg).BACK_PORT,
                                );
                            } else {
                                tmp___50 = strcmp(
                                    k as *const libc::c_char,
                                    b"workers\0" as *const u8 as *const libc::c_char,
                                );
                                if tmp___50 == 0 as libc::c_int {
                                    r = config_param_val_intl_pos(v, &mut (*cfg).NCORES);
                                } else {
                                    tmp___49 = strcmp(
                                        k as *const libc::c_char,
                                        b"backlog\0" as *const u8 as *const libc::c_char,
                                    );
                                    if tmp___49 == 0 as libc::c_int {
                                        r = config_param_val_int(v, &mut (*cfg).BACKLOG);
                                        if r != 0 {
                                            if (*cfg).BACKLOG < -(1 as libc::c_int) {
                                                (*cfg).BACKLOG = -(1 as libc::c_int);
                                            }
                                        }
                                    } else {
                                        tmp___48 = strcmp(
                                            k as *const libc::c_char,
                                            b"keepalive\0" as *const u8 as *const libc::c_char,
                                        );
                                        if tmp___48 == 0 as libc::c_int {
                                            r = config_param_val_int_pos(
                                                v,
                                                &mut (*cfg).TCP_KEEPALIVE_TIME,
                                            );
                                        } else {
                                            tmp___47 = strcmp(
                                                k as *const libc::c_char,
                                                b"shared-cache\0" as *const u8 as *const libc::c_char,
                                            );
                                            if tmp___47 == 0 as libc::c_int {
                                                r = config_param_val_int(v, &mut (*cfg).SHARED_CACHE);
                                            } else {
                                                tmp___46 = strcmp(
                                                    k as *const libc::c_char,
                                                    b"shared-cache-listen\0" as *const u8 as *const libc::c_char,
                                                );
                                                if tmp___46 == 0 as libc::c_int {
                                                    if v as libc::c_ulong
                                                        != 0 as *mut libc::c_void as libc::c_ulong
                                                    {
                                                        tmp___1 = strlen(v as *const libc::c_char);
                                                        if tmp___1 > 0 as libc::c_ulong {
                                                            r = config_param_host_port_wildcard(
                                                                v,
                                                                &mut (*cfg).SHCUPD_IP,
                                                                &mut (*cfg).SHCUPD_PORT,
                                                                1 as libc::c_int,
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    tmp___45 = strcmp(
                                                        k as *const libc::c_char,
                                                        b"shared-cache-peer\0" as *const u8 as *const libc::c_char,
                                                    );
                                                    if tmp___45 == 0 as libc::c_int {
                                                        r = config_param_shcupd_peer(v, cfg);
                                                    } else {
                                                        tmp___44 = strcmp(
                                                            k as *const libc::c_char,
                                                            b"shared-cache-if\0" as *const u8 as *const libc::c_char,
                                                        );
                                                        if tmp___44 == 0 as libc::c_int {
                                                            r = config_param_shcupd_mcastif(
                                                                v,
                                                                &mut (*cfg).SHCUPD_MCASTIF,
                                                                &mut (*cfg).SHCUPD_MCASTTTL,
                                                            );
                                                        } else {
                                                            tmp___43 = strcmp(
                                                                k as *const libc::c_char,
                                                                b"chroot\0" as *const u8 as *const libc::c_char,
                                                            );
                                                            if tmp___43 == 0 as libc::c_int {
                                                                if v as libc::c_ulong
                                                                    != 0 as *mut libc::c_void as libc::c_ulong
                                                                {
                                                                    tmp___7 = strlen(v as *const libc::c_char);
                                                                    if tmp___7 > 0 as libc::c_ulong {
                                                                        tmp___6 = stat(
                                                                            v as *const libc::c_char,
                                                                            &mut st as *mut stat,
                                                                        );
                                                                        if tmp___6 != 0 as libc::c_int {
                                                                            tmp___2 = __errno_location();
                                                                            tmp___3 = strerror(*tmp___2);
                                                                            config_error_set(
                                                                                b"Unable to stat directory '%s': %s'.\0" as *const u8
                                                                                    as *const libc::c_char as *mut libc::c_char,
                                                                                v,
                                                                                tmp___3,
                                                                            );
                                                                            r = 0 as libc::c_int;
                                                                        } else if !(st.st_mode & 61440 as libc::c_uint
                                                                                == 16384 as libc::c_uint)
                                                                            {
                                                                            tmp___4 = __errno_location();
                                                                            tmp___5 = strerror(*tmp___4);
                                                                            config_error_set(
                                                                                b"Bad chroot directory '%s': Not a directory.\0"
                                                                                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                                                                                v,
                                                                                tmp___5,
                                                                            );
                                                                            r = 0 as libc::c_int;
                                                                        } else {
                                                                            config_assign_str(&mut (*cfg).CHROOT, v);
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                tmp___42 = strcmp(
                                                                    k as *const libc::c_char,
                                                                    b"user\0" as *const u8 as *const libc::c_char,
                                                                );
                                                                if tmp___42 == 0 as libc::c_int {
                                                                    if v as libc::c_ulong
                                                                        != 0 as *mut libc::c_void as libc::c_ulong
                                                                    {
                                                                        tmp___8 = strlen(v as *const libc::c_char);
                                                                        if tmp___8 > 0 as libc::c_ulong {
                                                                            passwd = getpwnam(v as *const libc::c_char);
                                                                            if passwd.is_null() {
                                                                                config_error_set(
                                                                                    b"Invalid user '%s'.\0" as *const u8 as *const libc::c_char
                                                                                        as *mut libc::c_char,
                                                                                    v,
                                                                                );
                                                                                r = 0 as libc::c_int;
                                                                            } else {
                                                                                (*cfg).UID = (*passwd).pw_uid;
                                                                                (*cfg).GID = (*passwd).pw_gid;
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    tmp___41 = strcmp(
                                                                        k as *const libc::c_char,
                                                                        b"group\0" as *const u8 as *const libc::c_char,
                                                                    );
                                                                    if tmp___41 == 0 as libc::c_int {
                                                                        if v as libc::c_ulong
                                                                            != 0 as *mut libc::c_void as libc::c_ulong
                                                                        {
                                                                            tmp___9 = strlen(v as *const libc::c_char);
                                                                            if tmp___9 > 0 as libc::c_ulong {
                                                                                grp = getgrnam(v as *const libc::c_char);
                                                                                if grp.is_null() {
                                                                                    config_error_set(
                                                                                        b"Invalid group '%s'.\0" as *const u8 as *const libc::c_char
                                                                                            as *mut libc::c_char,
                                                                                        v,
                                                                                    );
                                                                                    r = 0 as libc::c_int;
                                                                                } else {
                                                                                    (*cfg).GID = (*grp).gr_gid;
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        tmp___40 = strcmp(
                                                                            k as *const libc::c_char,
                                                                            b"quiet\0" as *const u8 as *const libc::c_char,
                                                                        );
                                                                        if tmp___40 == 0 as libc::c_int {
                                                                            r = config_param_val_bool(v, &mut (*cfg).QUIET);
                                                                        } else {
                                                                            tmp___39 = strcmp(
                                                                                k as *const libc::c_char,
                                                                                b"syslog\0" as *const u8 as *const libc::c_char,
                                                                            );
                                                                            if tmp___39 == 0 as libc::c_int {
                                                                                r = config_param_val_bool(v, &mut (*cfg).SYSLOG);
                                                                            } else {
                                                                                tmp___38 = strcmp(
                                                                                    k as *const libc::c_char,
                                                                                    b"syslog-facility\0" as *const u8 as *const libc::c_char,
                                                                                );
                                                                                if tmp___38 == 0 as libc::c_int {
                                                                                    r = 1 as libc::c_int;
                                                                                    tmp___26 = strcmp(
                                                                                        v as *const libc::c_char,
                                                                                        b"auth\0" as *const u8 as *const libc::c_char,
                                                                                    );
                                                                                    if tmp___26 != 0 {
                                                                                        tmp___27 = strcmp(
                                                                                            v as *const libc::c_char,
                                                                                            b"authpriv\0" as *const u8 as *const libc::c_char,
                                                                                        );
                                                                                        if tmp___27 != 0 {
                                                                                            tmp___25 = strcmp(
                                                                                                v as *const libc::c_char,
                                                                                                b"cron\0" as *const u8 as *const libc::c_char,
                                                                                            );
                                                                                            if tmp___25 != 0 {
                                                                                                tmp___24 = strcmp(
                                                                                                    v as *const libc::c_char,
                                                                                                    b"daemon\0" as *const u8 as *const libc::c_char,
                                                                                                );
                                                                                                if tmp___24 != 0 {
                                                                                                    tmp___23 = strcmp(
                                                                                                        v as *const libc::c_char,
                                                                                                        b"ftp\0" as *const u8 as *const libc::c_char,
                                                                                                    );
                                                                                                    if tmp___23 != 0 {
                                                                                                        tmp___22 = strcmp(
                                                                                                            v as *const libc::c_char,
                                                                                                            b"local0\0" as *const u8 as *const libc::c_char,
                                                                                                        );
                                                                                                        if tmp___22 != 0 {
                                                                                                            tmp___21 = strcmp(
                                                                                                                v as *const libc::c_char,
                                                                                                                b"local1\0" as *const u8 as *const libc::c_char,
                                                                                                            );
                                                                                                            if tmp___21 != 0 {
                                                                                                                tmp___20 = strcmp(
                                                                                                                    v as *const libc::c_char,
                                                                                                                    b"local2\0" as *const u8 as *const libc::c_char,
                                                                                                                );
                                                                                                                if tmp___20 != 0 {
                                                                                                                    tmp___19 = strcmp(
                                                                                                                        v as *const libc::c_char,
                                                                                                                        b"local3\0" as *const u8 as *const libc::c_char,
                                                                                                                    );
                                                                                                                    if tmp___19 != 0 {
                                                                                                                        tmp___18 = strcmp(
                                                                                                                            v as *const libc::c_char,
                                                                                                                            b"local4\0" as *const u8 as *const libc::c_char,
                                                                                                                        );
                                                                                                                        if tmp___18 != 0 {
                                                                                                                            tmp___17 = strcmp(
                                                                                                                                v as *const libc::c_char,
                                                                                                                                b"local5\0" as *const u8 as *const libc::c_char,
                                                                                                                            );
                                                                                                                            if tmp___17 != 0 {
                                                                                                                                tmp___16 = strcmp(
                                                                                                                                    v as *const libc::c_char,
                                                                                                                                    b"local6\0" as *const u8 as *const libc::c_char,
                                                                                                                                );
                                                                                                                                if tmp___16 != 0 {
                                                                                                                                    tmp___15 = strcmp(
                                                                                                                                        v as *const libc::c_char,
                                                                                                                                        b"local7\0" as *const u8 as *const libc::c_char,
                                                                                                                                    );
                                                                                                                                    if tmp___15 != 0 {
                                                                                                                                        tmp___14 = strcmp(
                                                                                                                                            v as *const libc::c_char,
                                                                                                                                            b"lpr\0" as *const u8 as *const libc::c_char,
                                                                                                                                        );
                                                                                                                                        if tmp___14 != 0 {
                                                                                                                                            tmp___13 = strcmp(
                                                                                                                                                v as *const libc::c_char,
                                                                                                                                                b"mail\0" as *const u8 as *const libc::c_char,
                                                                                                                                            );
                                                                                                                                            if tmp___13 != 0 {
                                                                                                                                                tmp___12 = strcmp(
                                                                                                                                                    v as *const libc::c_char,
                                                                                                                                                    b"news\0" as *const u8 as *const libc::c_char,
                                                                                                                                                );
                                                                                                                                                if tmp___12 != 0 {
                                                                                                                                                    tmp___11 = strcmp(
                                                                                                                                                        v as *const libc::c_char,
                                                                                                                                                        b"user\0" as *const u8 as *const libc::c_char,
                                                                                                                                                    );
                                                                                                                                                    if tmp___11 != 0 {
                                                                                                                                                        tmp___10 = strcmp(
                                                                                                                                                            v as *const libc::c_char,
                                                                                                                                                            b"uucp\0" as *const u8 as *const libc::c_char,
                                                                                                                                                        );
                                                                                                                                                        if tmp___10 != 0 {
                                                                                                                                                            config_error_set(
                                                                                                                                                                b"Invalid facility '%s'.\0" as *const u8
                                                                                                                                                                    as *const libc::c_char as *mut libc::c_char,
                                                                                                                                                                v,
                                                                                                                                                            );
                                                                                                                                                            r = 0 as libc::c_int;
                                                                                                                                                        } else {
                                                                                                                                                            (*cfg)
                                                                                                                                                                .SYSLOG_FACILITY = (8 as libc::c_int) << 3 as libc::c_int;
                                                                                                                                                        }
                                                                                                                                                    } else {
                                                                                                                                                        (*cfg)
                                                                                                                                                            .SYSLOG_FACILITY = (1 as libc::c_int) << 3 as libc::c_int;
                                                                                                                                                    }
                                                                                                                                                } else {
                                                                                                                                                    (*cfg)
                                                                                                                                                        .SYSLOG_FACILITY = (7 as libc::c_int) << 3 as libc::c_int;
                                                                                                                                                }
                                                                                                                                            } else {
                                                                                                                                                (*cfg)
                                                                                                                                                    .SYSLOG_FACILITY = (2 as libc::c_int) << 3 as libc::c_int;
                                                                                                                                            }
                                                                                                                                        } else {
                                                                                                                                            (*cfg)
                                                                                                                                                .SYSLOG_FACILITY = (6 as libc::c_int) << 3 as libc::c_int;
                                                                                                                                        }
                                                                                                                                    } else {
                                                                                                                                        (*cfg)
                                                                                                                                            .SYSLOG_FACILITY = (23 as libc::c_int) << 3 as libc::c_int;
                                                                                                                                    }
                                                                                                                                } else {
                                                                                                                                    (*cfg)
                                                                                                                                        .SYSLOG_FACILITY = (22 as libc::c_int) << 3 as libc::c_int;
                                                                                                                                }
                                                                                                                            } else {
                                                                                                                                (*cfg)
                                                                                                                                    .SYSLOG_FACILITY = (21 as libc::c_int) << 3 as libc::c_int;
                                                                                                                            }
                                                                                                                        } else {
                                                                                                                            (*cfg)
                                                                                                                                .SYSLOG_FACILITY = (20 as libc::c_int) << 3 as libc::c_int;
                                                                                                                        }
                                                                                                                    } else {
                                                                                                                        (*cfg)
                                                                                                                            .SYSLOG_FACILITY = (19 as libc::c_int) << 3 as libc::c_int;
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    (*cfg)
                                                                                                                        .SYSLOG_FACILITY = (18 as libc::c_int) << 3 as libc::c_int;
                                                                                                                }
                                                                                                            } else {
                                                                                                                (*cfg)
                                                                                                                    .SYSLOG_FACILITY = (17 as libc::c_int) << 3 as libc::c_int;
                                                                                                            }
                                                                                                        } else {
                                                                                                            (*cfg)
                                                                                                                .SYSLOG_FACILITY = (16 as libc::c_int) << 3 as libc::c_int;
                                                                                                        }
                                                                                                    } else {
                                                                                                        (*cfg)
                                                                                                            .SYSLOG_FACILITY = (11 as libc::c_int) << 3 as libc::c_int;
                                                                                                    }
                                                                                                } else {
                                                                                                    (*cfg)
                                                                                                        .SYSLOG_FACILITY = (3 as libc::c_int) << 3 as libc::c_int;
                                                                                                }
                                                                                            } else {
                                                                                                (*cfg)
                                                                                                    .SYSLOG_FACILITY = (9 as libc::c_int) << 3 as libc::c_int;
                                                                                            }
                                                                                        } else {
                                                                                            (*cfg)
                                                                                                .SYSLOG_FACILITY = (10 as libc::c_int) << 3 as libc::c_int;
                                                                                        }
                                                                                    } else {
                                                                                        (*cfg)
                                                                                            .SYSLOG_FACILITY = (10 as libc::c_int) << 3 as libc::c_int;
                                                                                    }
                                                                                } else {
                                                                                    tmp___37 = strcmp(
                                                                                        k as *const libc::c_char,
                                                                                        b"daemon\0" as *const u8 as *const libc::c_char,
                                                                                    );
                                                                                    if tmp___37 == 0 as libc::c_int {
                                                                                        r = config_param_val_bool(v, &mut (*cfg).DAEMONIZE);
                                                                                    } else {
                                                                                        tmp___36 = strcmp(
                                                                                            k as *const libc::c_char,
                                                                                            b"write-ip\0" as *const u8 as *const libc::c_char,
                                                                                        );
                                                                                        if tmp___36 == 0 as libc::c_int {
                                                                                            r = config_param_val_bool(v, &mut (*cfg).WRITE_IP_OCTET);
                                                                                        } else {
                                                                                            tmp___35 = strcmp(
                                                                                                k as *const libc::c_char,
                                                                                                b"write-proxy\0" as *const u8 as *const libc::c_char,
                                                                                            );
                                                                                            if tmp___35 == 0 as libc::c_int {
                                                                                                r = config_param_val_bool(v, &mut (*cfg).WRITE_PROXY_LINE);
                                                                                            } else {
                                                                                                tmp___34 = strcmp(
                                                                                                    k as *const libc::c_char,
                                                                                                    b"proxy-proxy\0" as *const u8 as *const libc::c_char,
                                                                                                );
                                                                                                if tmp___34 == 0 as libc::c_int {
                                                                                                    r = config_param_val_bool(v, &mut (*cfg).PROXY_PROXY_LINE);
                                                                                                } else {
                                                                                                    tmp___33 = strcmp(
                                                                                                        k as *const libc::c_char,
                                                                                                        b"pem-file\0" as *const u8 as *const libc::c_char,
                                                                                                    );
                                                                                                    if tmp___33 == 0 as libc::c_int {
                                                                                                        if v as libc::c_ulong
                                                                                                            != 0 as *mut libc::c_void as libc::c_ulong
                                                                                                        {
                                                                                                            tmp___32 = strlen(v as *const libc::c_char);
                                                                                                            if tmp___32 > 0 as libc::c_ulong {
                                                                                                                tmp___31 = stat(
                                                                                                                    v as *const libc::c_char,
                                                                                                                    &mut st as *mut stat,
                                                                                                                );
                                                                                                                if tmp___31 != 0 as libc::c_int {
                                                                                                                    tmp___28 = __errno_location();
                                                                                                                    tmp___29 = strerror(*tmp___28);
                                                                                                                    config_error_set(
                                                                                                                        b"Unable to stat x509 certificate PEM file '%s': \0"
                                                                                                                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                                                                                                                        v,
                                                                                                                        tmp___29,
                                                                                                                    );
                                                                                                                    r = 0 as libc::c_int;
                                                                                                                } else if !(st.st_mode & 61440 as libc::c_uint
                                                                                                                        == 32768 as libc::c_uint)
                                                                                                                    {
                                                                                                                    config_error_set(
                                                                                                                        b"Invalid x509 certificate PEM file '%s': Not a file.\0"
                                                                                                                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                                                                                                                        v,
                                                                                                                    );
                                                                                                                    r = 0 as libc::c_int;
                                                                                                                } else {
                                                                                                                    tmp___30 = calloc(
                                                                                                                        1 as libc::c_int as size_t,
                                                                                                                        ::std::mem::size_of::<cert_files>() as libc::c_ulong,
                                                                                                                    );
                                                                                                                    cert = tmp___30 as *mut cert_files;
                                                                                                                    config_assign_str(&mut (*cert).CERT_FILE, v);
                                                                                                                    (*cert).NEXT = (*cfg).CERT_FILES;
                                                                                                                    (*cfg).CERT_FILES = cert;
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    } else {
                                                                                                        fprintf(
                                                                                                            stderr,
                                                                                                            b"Ignoring unknown configuration key '%s' in configuration file '%s', line %d\n\0"
                                                                                                                as *const u8 as *const libc::c_char,
                                                                                                            k,
                                                                                                            file,
                                                                                                            line,
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
    if r == 0 {
        if file as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            tmp___58 = config_error_get();
            config_die(
                b"Error in configuration file '%s', line %d: %s\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                file,
                line,
                tmp___58,
            );
        } else {
            tmp___59 = config_error_get();
            config_die(
                b"Invalid parameter '%s': %s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                k,
                tmp___59,
            );
        }
    }
}
pub unsafe extern "C" fn config_file_parse(
    mut file: *mut libc::c_char,
    mut cfg: *mut stud_config,
) -> libc::c_int {
    let mut line: [libc::c_char; 1024] = [0; 1024];
    let mut fd: *mut FILE = 0 as *mut FILE;
    let mut tmp: size_t = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    if cfg as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        config_die(
            b"Undefined stud options; THIS IS A BUG!\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    fd = 0 as *mut libc::c_void as *mut FILE;
    if file as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fd = stdin;
    } else {
        tmp = strlen(file as *const libc::c_char);
        if tmp < 1 as libc::c_ulong {
            fd = stdin;
        } else {
            tmp___0 = strcmp(
                file as *const libc::c_char,
                b"-\0" as *const u8 as *const libc::c_char,
            );
            if tmp___0 == 0 as libc::c_int {
                fd = stdin;
            } else {
                fd = fopen(
                    file as *const libc::c_char,
                    b"r\0" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    if fd as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___1 = __errno_location();
        tmp___2 = strerror(*tmp___1);
        config_die(
            b"Unable to open configuration file '%s': %s\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            file,
            tmp___2,
        );
    }
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        memset(
            line.as_mut_ptr() as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
        tmp___3 = fgets(
            line.as_mut_ptr(),
            (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_ulong) as libc::c_int,
            fd,
        );
        if tmp___3 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            break;
        }
        i += 1;
        key = config_get_param(line.as_mut_ptr());
        if key as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            continue;
        }
        val = config_get_value(line.as_mut_ptr());
        if val as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            continue;
        }
        str_trim(val);
        config_param_validate(key, val, cfg, file, i);
    }
    fclose(fd);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn config_disp_str(
    mut str: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    if str as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp = b"\0" as *const u8 as *const libc::c_char;
    } else {
        tmp = str as *const libc::c_char;
    }
    return tmp as *mut libc::c_char;
}
pub unsafe extern "C" fn config_disp_bool(mut v: libc::c_int) -> *mut libc::c_char {
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    if v > 0 as libc::c_int {
        tmp = b"on\0" as *const u8 as *const libc::c_char;
    } else {
        tmp = b"off\0" as *const u8 as *const libc::c_char;
    }
    return tmp as *mut libc::c_char;
}
pub unsafe extern "C" fn config_disp_uid(mut uid: uid_t) -> *mut libc::c_char {
    let mut tmp: __uid_t = 0;
    let mut pw: *mut passwd = 0 as *mut passwd;
    let mut tmp___0: *mut passwd = 0 as *mut passwd;
    let mut tmp___1: size_t = 0;
    memset(
        tmp_buf.as_mut_ptr() as *mut libc::c_void,
        '\u{0}' as i32,
        ::std::mem::size_of::<[libc::c_char; 150]>() as libc::c_ulong,
    );
    if uid == 0 as libc::c_uint {
        tmp = geteuid();
        if tmp != 0 as libc::c_uint {
            return tmp_buf.as_mut_ptr();
        }
    }
    tmp___0 = getpwuid(uid);
    pw = tmp___0;
    if !pw.is_null() {
        tmp___1 = strlen((*pw).pw_name as *const libc::c_char);
        memcpy(
            tmp_buf.as_mut_ptr() as *mut libc::c_void,
            (*pw).pw_name as *const libc::c_void,
            tmp___1,
        );
    }
    return tmp_buf.as_mut_ptr();
}
pub unsafe extern "C" fn config_disp_gid(mut gid: gid_t) -> *mut libc::c_char {
    let mut tmp: __uid_t = 0;
    let mut gr: *mut group = 0 as *mut group;
    let mut tmp___0: *mut group = 0 as *mut group;
    let mut tmp___1: size_t = 0;
    memset(
        tmp_buf.as_mut_ptr() as *mut libc::c_void,
        '\u{0}' as i32,
        ::std::mem::size_of::<[libc::c_char; 150]>() as libc::c_ulong,
    );
    if gid == 0 as libc::c_uint {
        tmp = geteuid();
        if tmp != 0 as libc::c_uint {
            return tmp_buf.as_mut_ptr();
        }
    }
    tmp___0 = getgrgid(gid);
    gr = tmp___0;
    if !gr.is_null() {
        tmp___1 = strlen((*gr).gr_name as *const libc::c_char);
        memcpy(
            tmp_buf.as_mut_ptr() as *mut libc::c_void,
            (*gr).gr_name as *const libc::c_void,
            tmp___1,
        );
    }
    return tmp_buf.as_mut_ptr();
}
pub unsafe extern "C" fn config_disp_hostport(
    mut host: *mut libc::c_char,
    mut port: *mut libc::c_char,
) -> *mut libc::c_char {
    memset(
        tmp_buf.as_mut_ptr() as *mut libc::c_void,
        '\u{0}' as i32,
        ::std::mem::size_of::<[libc::c_char; 150]>() as libc::c_ulong,
    );
    if host as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        if port as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            return b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    }
    strcat(tmp_buf.as_mut_ptr(), b"[\0" as *const u8 as *const libc::c_char);
    if host as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        strcat(tmp_buf.as_mut_ptr(), b"*\0" as *const u8 as *const libc::c_char);
    } else {
        strncat(
            tmp_buf.as_mut_ptr(),
            host as *const libc::c_char,
            40 as libc::c_int as size_t,
        );
    }
    strcat(tmp_buf.as_mut_ptr(), b"]:\0" as *const u8 as *const libc::c_char);
    strncat(
        tmp_buf.as_mut_ptr(),
        port as *const libc::c_char,
        5 as libc::c_int as size_t,
    );
    return tmp_buf.as_mut_ptr();
}
pub unsafe extern "C" fn config_disp_log_facility(
    mut facility: libc::c_int,
) -> *const libc::c_char {
    match facility {
        80 => return b"authpriv\0" as *const u8 as *const libc::c_char,
        72 => return b"cron\0" as *const u8 as *const libc::c_char,
        24 => return b"daemon\0" as *const u8 as *const libc::c_char,
        88 => return b"ftp\0" as *const u8 as *const libc::c_char,
        128 => return b"local0\0" as *const u8 as *const libc::c_char,
        136 => return b"local1\0" as *const u8 as *const libc::c_char,
        144 => return b"local2\0" as *const u8 as *const libc::c_char,
        152 => return b"local3\0" as *const u8 as *const libc::c_char,
        160 => return b"local4\0" as *const u8 as *const libc::c_char,
        168 => return b"local5\0" as *const u8 as *const libc::c_char,
        176 => return b"local6\0" as *const u8 as *const libc::c_char,
        184 => return b"local7\0" as *const u8 as *const libc::c_char,
        48 => return b"lpr\0" as *const u8 as *const libc::c_char,
        16 => return b"mail\0" as *const u8 as *const libc::c_char,
        56 => return b"news\0" as *const u8 as *const libc::c_char,
        8 => return b"user\0" as *const u8 as *const libc::c_char,
        64 => return b"uucp\0" as *const u8 as *const libc::c_char,
        _ => return b"UNKNOWN\0" as *const u8 as *const libc::c_char,
    };
}
pub unsafe extern "C" fn config_print_usage_fd(
    mut prog: *mut libc::c_char,
    mut cfg: *mut stud_config,
    mut out: *mut FILE,
) {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___7: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___8: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___9: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___10: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___11: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___12: *mut libc::c_char = 0 as *mut libc::c_char;
    if out as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        out = stderr;
    }
    tmp = __xpg_basename(prog);
    fprintf(
        out,
        b"Usage: %s [OPTIONS] PEM\n\n\0" as *const u8 as *const libc::c_char,
        tmp,
    );
    fprintf(
        out,
        b"This is stud, The Scalable TLS Unwrapping Daemon.\n\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(out, b"CONFIGURATION:\n\0" as *const u8 as *const libc::c_char);
    fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        out,
        b"        --config=FILE      Load configuration from specified file.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        out,
        b"        --default-config   Prints default configuration to stdout.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(out, b"ENCRYPTION METHODS:\n\0" as *const u8 as *const libc::c_char);
    fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        out,
        b"      --tls                   TLSv1 (default)\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        out,
        b"      --ssl                   SSLv3 (implies no TLSv1)\n\0" as *const u8
            as *const libc::c_char,
    );
    tmp___0 = config_disp_str((*cfg).CIPHER_SUITE);
    fprintf(
        out,
        b"  -c  --ciphers=SUITE         Sets allowed ciphers (Default: \"%s\")\n\0"
            as *const u8 as *const libc::c_char,
        tmp___0,
    );
    tmp___1 = config_disp_str((*cfg).ENGINE);
    fprintf(
        out,
        b"  -e  --ssl-engine=NAME       Sets OpenSSL engine (Default: \"%s\")\n\0"
            as *const u8 as *const libc::c_char,
        tmp___1,
    );
    fprintf(
        out,
        b"  -O  --prefer-server-ciphers Prefer server list order\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(out, b"SOCKET:\n\0" as *const u8 as *const libc::c_char);
    fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        out,
        b"  --client                    Enable client proxy mode\n\0" as *const u8
            as *const libc::c_char,
    );
    tmp___2 = config_disp_hostport((*cfg).BACK_IP, (*cfg).BACK_PORT);
    fprintf(
        out,
        b"  -b  --backend=HOST,PORT     Backend [connect] (default is \"%s\")\n\0"
            as *const u8 as *const libc::c_char,
        tmp___2,
    );
    tmp___3 = config_disp_hostport((*cfg).FRONT_IP, (*cfg).FRONT_PORT);
    fprintf(
        out,
        b"  -f  --frontend=HOST,PORT    Frontend [bind] (default is \"%s\")\n\0"
            as *const u8 as *const libc::c_char,
        tmp___3,
    );
    fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        out,
        b"  -U  --shared-cache-listen=HOST,PORT\n\0" as *const u8 as *const libc::c_char,
    );
    tmp___4 = config_disp_hostport((*cfg).SHCUPD_IP, (*cfg).SHCUPD_PORT);
    fprintf(
        out,
        b"                              Accept cache updates on UDP (Default: \"%s\")\n\0"
            as *const u8 as *const libc::c_char,
        tmp___4,
    );
    fprintf(
        out,
        b"                              NOTE: This option requires enabled SSL session cache.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        out,
        b"  -P  --shared-cache-peer=HOST,PORT\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        out,
        b"                              Send cache updates to specified peer\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        out,
        b"                              NOTE: This option can be specified multiple times.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        out,
        b"  -M  --shared-cache-if=IFACE[,TTL]\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        out,
        b"                              Force iface and ttl to receive and send multicast updates\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(out, b"PERFORMANCE:\n\0" as *const u8 as *const libc::c_char);
    fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        out,
        b"  -n  --workers=NUM          Number of worker processes (Default: %ld)\n\0"
            as *const u8 as *const libc::c_char,
        (*cfg).NCORES,
    );
    fprintf(
        out,
        b"  -B  --backlog=NUM          Set listen backlog size (Default: %d)\n\0"
            as *const u8 as *const libc::c_char,
        (*cfg).BACKLOG,
    );
    fprintf(
        out,
        b"  -k  --keepalive=SECS       TCP keepalive on client socket (Default: %d)\n\0"
            as *const u8 as *const libc::c_char,
        (*cfg).TCP_KEEPALIVE_TIME,
    );
    fprintf(
        out,
        b"  -C  --session-cache=NUM    Enable and set SSL session cache to specified number\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        out,
        b"                             of sessions (Default: %d)\n\0" as *const u8
            as *const libc::c_char,
        (*cfg).SHARED_CACHE,
    );
    fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(out, b"SECURITY:\n\0" as *const u8 as *const libc::c_char);
    fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    tmp___5 = config_disp_str((*cfg).CHROOT);
    fprintf(
        out,
        b"  -r  --chroot=DIR           Sets chroot directory (Default: \"%s\")\n\0"
            as *const u8 as *const libc::c_char,
        tmp___5,
    );
    tmp___6 = config_disp_uid((*cfg).UID);
    fprintf(
        out,
        b"  -u  --user=USER            Set uid/gid after binding the socket (Default: \"%s\")\n\0"
            as *const u8 as *const libc::c_char,
        tmp___6,
    );
    tmp___7 = config_disp_gid((*cfg).GID);
    fprintf(
        out,
        b"  -g  --group=GROUP          Set gid after binding the socket (Default: \"%s\")\n\0"
            as *const u8 as *const libc::c_char,
        tmp___7,
    );
    fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(out, b"LOGGING:\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        out,
        b"  -q  --quiet                Be quiet; emit only error messages\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        out,
        b"  -s  --syslog               Send log message to syslog in addition to stderr/stdout\n\0"
            as *const u8 as *const libc::c_char,
    );
    tmp___8 = config_disp_log_facility((*cfg).SYSLOG_FACILITY);
    fprintf(
        out,
        b"  --syslog-facility=FACILITY Syslog facility to use (Default: \"%s\")\n\0"
            as *const u8 as *const libc::c_char,
        tmp___8,
    );
    fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(out, b"OTHER OPTIONS:\n\0" as *const u8 as *const libc::c_char);
    tmp___9 = config_disp_bool((*cfg).DAEMONIZE);
    fprintf(
        out,
        b"      --daemon               Fork into background and become a daemon (Default: %s)\n\0"
            as *const u8 as *const libc::c_char,
        tmp___9,
    );
    fprintf(
        out,
        b"      --write-ip             Write 1 octet with the IP family followed by the IP\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        out,
        b"                             address in 4 (IPv4) or 16 (IPv6) octets little-endian\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        out,
        b"                             to backend before the actual data\n\0"
            as *const u8 as *const libc::c_char,
    );
    tmp___10 = config_disp_bool((*cfg).WRITE_IP_OCTET);
    fprintf(
        out,
        b"                             (Default: %s)\n\0" as *const u8
            as *const libc::c_char,
        tmp___10,
    );
    fprintf(
        out,
        b"      --write-proxy          Write HaProxy's PROXY (IPv4 or IPv6) protocol line\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        out,
        b"                             before actual data\n\0" as *const u8
            as *const libc::c_char,
    );
    tmp___11 = config_disp_bool((*cfg).WRITE_PROXY_LINE);
    fprintf(
        out,
        b"                             (Default: %s)\n\0" as *const u8
            as *const libc::c_char,
        tmp___11,
    );
    fprintf(
        out,
        b"      --proxy-proxy          Proxy HaProxy's PROXY (IPv4 or IPv6) protocol line\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        out,
        b"                             before actual data\n\0" as *const u8
            as *const libc::c_char,
    );
    tmp___12 = config_disp_bool((*cfg).PROXY_PROXY_LINE);
    fprintf(
        out,
        b"                             (Default: %s)\n\0" as *const u8
            as *const libc::c_char,
        tmp___12,
    );
    fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        out,
        b"  -t  --test                 Test configuration and exit\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        out,
        b"  -V  --version              Print program version and exit\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        out,
        b"  -h  --help                 This help message\n\0" as *const u8
            as *const libc::c_char,
    );
}
pub unsafe extern "C" fn config_print_default(
    mut fd: *mut FILE,
    mut cfg: *mut stud_config,
) {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___7: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___8: size_t = 0;
    let mut tmp___9: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___10: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___11: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___12: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___13: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___14: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___15: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___16: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___17: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___18: *mut libc::c_char = 0 as *mut libc::c_char;
    if fd as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# stud(8), The Scalable TLS Unwrapping Daemon's configuration\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# NOTE: all config file parameters can be overriden\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(fd, b"#       from command line!\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# Listening address. REQUIRED.\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: string\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# syntax: [HOST]:PORT\n\0" as *const u8 as *const libc::c_char);
    tmp = config_disp_hostport((*cfg).FRONT_IP, (*cfg).FRONT_PORT);
    fprintf(
        fd,
        b"%s = \"%s\"\n\0" as *const u8 as *const libc::c_char,
        b"frontend\0" as *const u8 as *const libc::c_char,
        tmp,
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# Upstream server address. REQUIRED.\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: string\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# syntax: [HOST]:PORT.\n\0" as *const u8 as *const libc::c_char);
    tmp___0 = config_disp_hostport((*cfg).BACK_IP, (*cfg).BACK_PORT);
    fprintf(
        fd,
        b"%s = \"%s\"\n\0" as *const u8 as *const libc::c_char,
        b"backend\0" as *const u8 as *const libc::c_char,
        tmp___0,
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# SSL x509 certificate file. REQUIRED.\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        fd,
        b"# List multiple certs to use SNI. Certs are used in the order they\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        fd,
        b"# are listed; the last cert listed will be used if none of the others match\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: string\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"%s = \"%s\"\n\0" as *const u8 as *const libc::c_char,
        b"pem-file\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# SSL protocol.\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# tls = on\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# ssl = off\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# List of allowed SSL ciphers.\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# Run openssl ciphers for list of available ciphers.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(fd, b"# type: string\n\0" as *const u8 as *const libc::c_char);
    tmp___1 = config_disp_str((*cfg).CIPHER_SUITE);
    fprintf(
        fd,
        b"%s = \"%s\"\n\0" as *const u8 as *const libc::c_char,
        b"ciphers\0" as *const u8 as *const libc::c_char,
        tmp___1,
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# Enforce server cipher list order\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: boolean\n\0" as *const u8 as *const libc::c_char);
    tmp___2 = config_disp_bool((*cfg).PREFER_SERVER_CIPHERS);
    fprintf(
        fd,
        b"%s = %s\n\0" as *const u8 as *const libc::c_char,
        b"prefer-server-ciphers\0" as *const u8 as *const libc::c_char,
        tmp___2,
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# Use specified SSL engine\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: string\n\0" as *const u8 as *const libc::c_char);
    tmp___3 = config_disp_str((*cfg).ENGINE);
    fprintf(
        fd,
        b"%s = \"%s\"\n\0" as *const u8 as *const libc::c_char,
        b"ssl-engine\0" as *const u8 as *const libc::c_char,
        tmp___3,
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# Number of worker processes\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: integer\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"%s = %d\n\0" as *const u8 as *const libc::c_char,
        b"workers\0" as *const u8 as *const libc::c_char,
        (*cfg).NCORES as libc::c_int,
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# Listen backlog size\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: integer\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"%s = %d\n\0" as *const u8 as *const libc::c_char,
        b"backlog\0" as *const u8 as *const libc::c_char,
        (*cfg).BACKLOG,
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# TCP socket keepalive interval in seconds\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: integer\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"%s = %d\n\0" as *const u8 as *const libc::c_char,
        b"keepalive\0" as *const u8 as *const libc::c_char,
        (*cfg).TCP_KEEPALIVE_TIME,
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# SSL session cache size\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: integer\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"%s = %d\n\0" as *const u8 as *const libc::c_char,
        b"shared-cache\0" as *const u8 as *const libc::c_char,
        (*cfg).SHARED_CACHE,
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# Accept shared SSL cache updates on specified listener.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: string\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# syntax: [HOST]:PORT\n\0" as *const u8 as *const libc::c_char);
    tmp___4 = config_disp_hostport((*cfg).SHCUPD_IP, (*cfg).SHCUPD_PORT);
    fprintf(
        fd,
        b"%s = \"%s\"\n\0" as *const u8 as *const libc::c_char,
        b"shared-cache-listen\0" as *const u8 as *const libc::c_char,
        tmp___4,
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# Shared cache peer address.\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# Multiple stud processes on multiple hosts (host limit: %d)\n\0" as *const u8
            as *const libc::c_char,
        15 as libc::c_int,
    );
    fprintf(
        fd,
        b"# can share SSL session cache by sending updates to peers.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# NOTE: This parameter can be specified multiple times in order\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        fd,
        b"#       to specify multiple peers.\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: string\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# syntax: [HOST]:PORT\n\0" as *const u8 as *const libc::c_char);
    tmp___5 = config_disp_hostport(
        0 as *mut libc::c_void as *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
    );
    fprintf(
        fd,
        b"# %s = \"%s\"\n\0" as *const u8 as *const libc::c_char,
        b"shared-cache-peer\0" as *const u8 as *const libc::c_char,
        tmp___5,
    );
    i = 0 as libc::c_int;
    while i < 15 as libc::c_int {
        if (*cfg).SHCUPD_PEERS[i as usize].ip as libc::c_ulong
            == 0 as *mut libc::c_void as libc::c_ulong
        {
            if (*cfg).SHCUPD_PEERS[i as usize].port as libc::c_ulong
                == 0 as *mut libc::c_void as libc::c_ulong
            {
                break;
            }
        }
        tmp___6 = config_disp_hostport(
            (*cfg).SHCUPD_PEERS[i as usize].ip,
            (*cfg).SHCUPD_PEERS[i as usize].port,
        );
        fprintf(
            fd,
            b"%s = \"%s\"\n\0" as *const u8 as *const libc::c_char,
            b"shared-cache-peer\0" as *const u8 as *const libc::c_char,
            tmp___6,
        );
        i += 1;
    }
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# Shared cache interface name and optional TTL\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: string\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# syntax: iface[,TTL]\n\0" as *const u8 as *const libc::c_char);
    tmp___7 = config_disp_str((*cfg).SHCUPD_MCASTIF);
    fprintf(
        fd,
        b"# %s = \"%s\0" as *const u8 as *const libc::c_char,
        b"shared-cache-if\0" as *const u8 as *const libc::c_char,
        tmp___7,
    );
    if (*cfg).SHCUPD_MCASTTTL as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong
    {
        tmp___8 = strlen((*cfg).SHCUPD_MCASTTTL as *const libc::c_char);
        if tmp___8 > 0 as libc::c_ulong {
            fprintf(
                fd,
                b",%s\0" as *const u8 as *const libc::c_char,
                (*cfg).SHCUPD_MCASTTTL,
            );
        }
    }
    fprintf(fd, b"\"\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# Chroot directory\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: string\n\0" as *const u8 as *const libc::c_char);
    tmp___9 = config_disp_str((*cfg).CHROOT);
    fprintf(
        fd,
        b"%s = \"%s\"\n\0" as *const u8 as *const libc::c_char,
        b"chroot\0" as *const u8 as *const libc::c_char,
        tmp___9,
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# Set uid after binding a socket\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: string\n\0" as *const u8 as *const libc::c_char);
    tmp___10 = config_disp_uid((*cfg).UID);
    fprintf(
        fd,
        b"%s = \"%s\"\n\0" as *const u8 as *const libc::c_char,
        b"user\0" as *const u8 as *const libc::c_char,
        tmp___10,
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# Set gid after binding a socket\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: string\n\0" as *const u8 as *const libc::c_char);
    tmp___11 = config_disp_gid((*cfg).GID);
    fprintf(
        fd,
        b"%s = \"%s\"\n\0" as *const u8 as *const libc::c_char,
        b"group\0" as *const u8 as *const libc::c_char,
        tmp___11,
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# Quiet execution, report only error messages\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: boolean\n\0" as *const u8 as *const libc::c_char);
    tmp___12 = config_disp_bool((*cfg).QUIET);
    fprintf(
        fd,
        b"%s = %s\n\0" as *const u8 as *const libc::c_char,
        b"quiet\0" as *const u8 as *const libc::c_char,
        tmp___12,
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# Use syslog for logging\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: boolean\n\0" as *const u8 as *const libc::c_char);
    tmp___13 = config_disp_bool((*cfg).SYSLOG);
    fprintf(
        fd,
        b"%s = %s\n\0" as *const u8 as *const libc::c_char,
        b"syslog\0" as *const u8 as *const libc::c_char,
        tmp___13,
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# Syslog facility to use\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: string\n\0" as *const u8 as *const libc::c_char);
    tmp___14 = config_disp_log_facility((*cfg).SYSLOG_FACILITY);
    fprintf(
        fd,
        b"%s = \"%s\"\n\0" as *const u8 as *const libc::c_char,
        b"syslog-facility\0" as *const u8 as *const libc::c_char,
        tmp___14,
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# Run as daemon\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: boolean\n\0" as *const u8 as *const libc::c_char);
    tmp___15 = config_disp_bool((*cfg).DAEMONIZE);
    fprintf(
        fd,
        b"%s = %s\n\0" as *const u8 as *const libc::c_char,
        b"daemon\0" as *const u8 as *const libc::c_char,
        tmp___15,
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# Report client address by writing IP before sending data\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# NOTE: This option is mutually exclusive with option %s and %s.\n\0"
            as *const u8 as *const libc::c_char,
        b"write-proxy\0" as *const u8 as *const libc::c_char,
        b"proxy-proxy\0" as *const u8 as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: boolean\n\0" as *const u8 as *const libc::c_char);
    tmp___16 = config_disp_bool((*cfg).WRITE_IP_OCTET);
    fprintf(
        fd,
        b"%s = %s\n\0" as *const u8 as *const libc::c_char,
        b"write-ip\0" as *const u8 as *const libc::c_char,
        tmp___16,
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# Report client address using SENDPROXY protocol, see\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        fd,
        b"# http://haproxy.1wt.eu/download/1.5/doc/proxy-protocol.txt\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(fd, b"# for details.\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# NOTE: This option is mutually exclusive with option %s and %s.\n\0"
            as *const u8 as *const libc::c_char,
        b"write-ip\0" as *const u8 as *const libc::c_char,
        b"proxy-proxy\0" as *const u8 as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: boolean\n\0" as *const u8 as *const libc::c_char);
    tmp___17 = config_disp_bool((*cfg).WRITE_PROXY_LINE);
    fprintf(
        fd,
        b"%s = %s\n\0" as *const u8 as *const libc::c_char,
        b"write-proxy\0" as *const u8 as *const libc::c_char,
        tmp___17,
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# Proxy an existing SENDPROXY protocol header through this request.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# NOTE: This option is mutually exclusive with option %s and %s.\n\0"
            as *const u8 as *const libc::c_char,
        b"write-ip\0" as *const u8 as *const libc::c_char,
        b"write-proxy\0" as *const u8 as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: boolean\n\0" as *const u8 as *const libc::c_char);
    tmp___18 = config_disp_bool((*cfg).PROXY_PROXY_LINE);
    fprintf(
        fd,
        b"%s = %s\n\0" as *const u8 as *const libc::c_char,
        b"proxy-proxy\0" as *const u8 as *const libc::c_char,
        tmp___18,
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# EOF\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn config_print_usage(
    mut prog: *mut libc::c_char,
    mut cfg: *mut stud_config,
) {
    config_print_usage_fd(prog, cfg, stdout);
}
static mut tls: libc::c_int = 0 as libc::c_int;
static mut ssl: libc::c_int = 0 as libc::c_int;
static mut client: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn config_parse_cli(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut cfg: *mut stud_config,
) {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut test_only: libc::c_int = 0;
    let mut prog: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut long_options: [option; 30] = [option {
        name: 0 as *const libc::c_char,
        has_arg: 0,
        flag: 0 as *mut libc::c_int,
        val: 0,
    }; 30];
    let mut option_index: libc::c_int = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut SSL_CTX = 0 as *mut SSL_CTX;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    test_only = 0 as libc::c_int;
    long_options[0 as libc::c_int as usize]
        .name = b"config\0" as *const u8 as *const libc::c_char;
    long_options[0 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    long_options[0 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[0 as libc::c_int as usize].val = 10000 as libc::c_int;
    long_options[1 as libc::c_int as usize]
        .name = b"default-config\0" as *const u8 as *const libc::c_char;
    long_options[1 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    long_options[1 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[1 as libc::c_int as usize].val = 10001 as libc::c_int;
    long_options[2 as libc::c_int as usize]
        .name = b"tls\0" as *const u8 as *const libc::c_char;
    long_options[2 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    long_options[2 as libc::c_int as usize].flag = &mut tls;
    long_options[2 as libc::c_int as usize].val = 1 as libc::c_int;
    long_options[3 as libc::c_int as usize]
        .name = b"ssl\0" as *const u8 as *const libc::c_char;
    long_options[3 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    long_options[3 as libc::c_int as usize].flag = &mut ssl;
    long_options[3 as libc::c_int as usize].val = 1 as libc::c_int;
    long_options[4 as libc::c_int as usize]
        .name = b"client\0" as *const u8 as *const libc::c_char;
    long_options[4 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    long_options[4 as libc::c_int as usize].flag = &mut client;
    long_options[4 as libc::c_int as usize].val = 1 as libc::c_int;
    long_options[5 as libc::c_int as usize]
        .name = b"ciphers\0" as *const u8 as *const libc::c_char;
    long_options[5 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    long_options[5 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[5 as libc::c_int as usize].val = 'c' as i32;
    long_options[6 as libc::c_int as usize]
        .name = b"prefer-server-ciphers\0" as *const u8 as *const libc::c_char;
    long_options[6 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    long_options[6 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[6 as libc::c_int as usize].val = 'O' as i32;
    long_options[7 as libc::c_int as usize]
        .name = b"backend\0" as *const u8 as *const libc::c_char;
    long_options[7 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    long_options[7 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[7 as libc::c_int as usize].val = 'b' as i32;
    long_options[8 as libc::c_int as usize]
        .name = b"frontend\0" as *const u8 as *const libc::c_char;
    long_options[8 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    long_options[8 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[8 as libc::c_int as usize].val = 'f' as i32;
    long_options[9 as libc::c_int as usize]
        .name = b"workers\0" as *const u8 as *const libc::c_char;
    long_options[9 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    long_options[9 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[9 as libc::c_int as usize].val = 'n' as i32;
    long_options[10 as libc::c_int as usize]
        .name = b"backlog\0" as *const u8 as *const libc::c_char;
    long_options[10 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    long_options[10 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[10 as libc::c_int as usize].val = 'B' as i32;
    long_options[11 as libc::c_int as usize]
        .name = b"shared-cache\0" as *const u8 as *const libc::c_char;
    long_options[11 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    long_options[11 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[11 as libc::c_int as usize].val = 'C' as i32;
    long_options[12 as libc::c_int as usize]
        .name = b"shared-cache-listen\0" as *const u8 as *const libc::c_char;
    long_options[12 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    long_options[12 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[12 as libc::c_int as usize].val = 'U' as i32;
    long_options[13 as libc::c_int as usize]
        .name = b"shared-cache-peer\0" as *const u8 as *const libc::c_char;
    long_options[13 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    long_options[13 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[13 as libc::c_int as usize].val = 'P' as i32;
    long_options[14 as libc::c_int as usize]
        .name = b"shared-cache-if\0" as *const u8 as *const libc::c_char;
    long_options[14 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    long_options[14 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[14 as libc::c_int as usize].val = 'M' as i32;
    long_options[15 as libc::c_int as usize]
        .name = b"keepalive\0" as *const u8 as *const libc::c_char;
    long_options[15 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    long_options[15 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[15 as libc::c_int as usize].val = 'k' as i32;
    long_options[16 as libc::c_int as usize]
        .name = b"chroot\0" as *const u8 as *const libc::c_char;
    long_options[16 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    long_options[16 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[16 as libc::c_int as usize].val = 'r' as i32;
    long_options[17 as libc::c_int as usize]
        .name = b"user\0" as *const u8 as *const libc::c_char;
    long_options[17 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    long_options[17 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[17 as libc::c_int as usize].val = 'u' as i32;
    long_options[18 as libc::c_int as usize]
        .name = b"group\0" as *const u8 as *const libc::c_char;
    long_options[18 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    long_options[18 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[18 as libc::c_int as usize].val = 'g' as i32;
    long_options[19 as libc::c_int as usize]
        .name = b"quiet\0" as *const u8 as *const libc::c_char;
    long_options[19 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    long_options[19 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[19 as libc::c_int as usize].val = 'q' as i32;
    long_options[20 as libc::c_int as usize]
        .name = b"syslog\0" as *const u8 as *const libc::c_char;
    long_options[20 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    long_options[20 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[20 as libc::c_int as usize].val = 's' as i32;
    long_options[21 as libc::c_int as usize]
        .name = b"syslog-facility\0" as *const u8 as *const libc::c_char;
    long_options[21 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    long_options[21 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[21 as libc::c_int as usize].val = 11015 as libc::c_int;
    long_options[22 as libc::c_int as usize]
        .name = b"daemon\0" as *const u8 as *const libc::c_char;
    long_options[22 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    long_options[22 as libc::c_int as usize].flag = &mut (*cfg).DAEMONIZE;
    long_options[22 as libc::c_int as usize].val = 1 as libc::c_int;
    long_options[23 as libc::c_int as usize]
        .name = b"write-ip\0" as *const u8 as *const libc::c_char;
    long_options[23 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    long_options[23 as libc::c_int as usize].flag = &mut (*cfg).WRITE_IP_OCTET;
    long_options[23 as libc::c_int as usize].val = 1 as libc::c_int;
    long_options[24 as libc::c_int as usize]
        .name = b"write-proxy\0" as *const u8 as *const libc::c_char;
    long_options[24 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    long_options[24 as libc::c_int as usize].flag = &mut (*cfg).WRITE_PROXY_LINE;
    long_options[24 as libc::c_int as usize].val = 1 as libc::c_int;
    long_options[25 as libc::c_int as usize]
        .name = b"proxy-proxy\0" as *const u8 as *const libc::c_char;
    long_options[25 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    long_options[25 as libc::c_int as usize].flag = &mut (*cfg).PROXY_PROXY_LINE;
    long_options[25 as libc::c_int as usize].val = 1 as libc::c_int;
    long_options[26 as libc::c_int as usize]
        .name = b"test\0" as *const u8 as *const libc::c_char;
    long_options[26 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    long_options[26 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[26 as libc::c_int as usize].val = 't' as i32;
    long_options[27 as libc::c_int as usize]
        .name = b"version\0" as *const u8 as *const libc::c_char;
    long_options[27 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    long_options[27 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[27 as libc::c_int as usize].val = 'V' as i32;
    long_options[28 as libc::c_int as usize]
        .name = b"help\0" as *const u8 as *const libc::c_char;
    long_options[28 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    long_options[28 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[28 as libc::c_int as usize].val = 'h' as i32;
    long_options[29 as libc::c_int as usize].name = 0 as *const libc::c_char;
    long_options[29 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    long_options[29 as libc::c_int as usize].flag = 0 as *mut libc::c_int;
    long_options[29 as libc::c_int as usize].val = 0 as libc::c_int;
    loop {
        option_index = 0 as libc::c_int;
        c = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            b"c:e:Ob:f:n:B:C:U:P:M:k:r:u:g:qstVh\0" as *const u8 as *const libc::c_char,
            long_options.as_mut_ptr() as *const option,
            &mut option_index,
        );
        if c == -(1 as libc::c_int) {
            break;
        }
        match c {
            0 => {}
            10000 => {
                tmp___0 = config_file_parse(optarg, cfg);
                if tmp___0 == 0 {
                    tmp = config_error_get();
                    config_die(
                        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        tmp,
                    );
                }
            }
            10001 => {
                config_print_default(stdout, cfg);
                exit(0 as libc::c_int);
            }
            11015 => {
                config_param_validate(
                    b"syslog-facility\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    optarg,
                    cfg,
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            99 => {
                config_param_validate(
                    b"ciphers\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    optarg,
                    cfg,
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            101 => {
                config_param_validate(
                    b"ssl-engine\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    optarg,
                    cfg,
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            79 => {
                config_param_validate(
                    b"prefer-server-ciphers\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"on\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    cfg,
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            98 => {
                config_param_validate(
                    b"backend\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    optarg,
                    cfg,
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            102 => {
                config_param_validate(
                    b"frontend\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    optarg,
                    cfg,
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            110 => {
                config_param_validate(
                    b"workers\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    optarg,
                    cfg,
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            66 => {
                config_param_validate(
                    b"backlog\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    optarg,
                    cfg,
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            67 => {
                config_param_validate(
                    b"shared-cache\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    optarg,
                    cfg,
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            85 => {
                config_param_validate(
                    b"shared-cache-listen\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    optarg,
                    cfg,
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            80 => {
                config_param_validate(
                    b"shared-cache-peer\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    optarg,
                    cfg,
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            77 => {
                config_param_validate(
                    b"shared-cache-if\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    optarg,
                    cfg,
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            107 => {
                config_param_validate(
                    b"keepalive\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    optarg,
                    cfg,
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            114 => {
                config_param_validate(
                    b"chroot\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    optarg,
                    cfg,
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            117 => {
                config_param_validate(
                    b"user\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    optarg,
                    cfg,
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            103 => {
                config_param_validate(
                    b"group\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    optarg,
                    cfg,
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            113 => {
                config_param_validate(
                    b"quiet\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    b"on\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    cfg,
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            115 => {
                config_param_validate(
                    b"syslog\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    b"on\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    cfg,
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            116 => {
                test_only = 1 as libc::c_int;
            }
            86 => {
                tmp___1 = __xpg_basename(*argv.offset(0 as libc::c_int as isize));
                printf(
                    b"%s %s\n\0" as *const u8 as *const libc::c_char,
                    tmp___1,
                    b"0.3-dev\0" as *const u8 as *const libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            104 => {
                config_print_usage(*argv.offset(0 as libc::c_int as isize), cfg);
                exit(0 as libc::c_int);
            }
            _ => {
                tmp___2 = __xpg_basename(*argv.offset(0 as libc::c_int as isize));
                config_die(
                    b"Invalid command line parameters. Run %s --help for instructions.\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                    tmp___2,
                );
            }
        }
    }
    prog = *argv.offset(0 as libc::c_int as isize);
    let mut current_block_166: u64;
    if tls != 0 {
        if ssl != 0 {
            config_die(
                b"Options --tls and --ssl are mutually exclusive.\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            current_block_166 = 15460309861373144675;
        } else {
            current_block_166 = 490108661212568629;
        }
    } else {
        current_block_166 = 490108661212568629;
    }
    match current_block_166 {
        490108661212568629 => {
            if ssl != 0 {
                (*cfg).ETYPE = ENC_SSL;
            } else if tls != 0 {
                (*cfg).ETYPE = ENC_TLS;
            }
        }
        _ => {}
    }
    if client != 0 {
        (*cfg).PMODE = SSL_CLIENT;
    }
    if (*cfg).WRITE_IP_OCTET != 0 {
        if (*cfg).WRITE_PROXY_LINE != 0 {
            config_die(
                b"Options --write-ip and --write-proxy are mutually exclusive.\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
    }
    if (*cfg).WRITE_PROXY_LINE != 0 {
        if (*cfg).PROXY_PROXY_LINE != 0 {
            config_die(
                b"Options --write-proxy and --proxy-proxy are mutually exclusive.\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
    }
    if (*cfg).WRITE_IP_OCTET != 0 {
        if (*cfg).PROXY_PROXY_LINE != 0 {
            config_die(
                b"Options --write-ip and --proxy-proxy are mutually exclusive.\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
    }
    if (*cfg).DAEMONIZE != 0 {
        (*cfg).SYSLOG = 1 as libc::c_int;
        (*cfg).QUIET = 1 as libc::c_int;
    }
    if (*cfg).SHCUPD_IP as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        if (*cfg).SHARED_CACHE == 0 {
            config_die(
                b"Shared cache update listener is defined, but shared cache is disabled.\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
    }
    argc -= optind;
    argv = argv.offset(optind as isize);
    i = 0 as libc::c_int;
    while i < argc {
        config_param_validate(
            b"pem-file\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            *argv.offset(i as isize),
            cfg,
            0 as *mut libc::c_void as *mut libc::c_char,
            0 as libc::c_int,
        );
        i += 1;
    }
    if (*cfg).PMODE as libc::c_uint == 0 as libc::c_uint {
        if (*cfg).CERT_FILES as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong
        {
            config_die(
                b"No x509 certificate PEM file specified!\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
    }
    if test_only != 0 {
        fprintf(
            stderr,
            b"Trying to initialize SSL contexts with your certificates\0" as *const u8
                as *const libc::c_char,
        );
        tmp___3 = init_openssl();
        if tmp___3.is_null() {
            config_die(
                b"Error initializing OpenSSL.\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        tmp___4 = __xpg_basename(prog);
        printf(
            b"%s configuration looks ok.\n\0" as *const u8 as *const libc::c_char,
            tmp___4,
        );
        exit(0 as libc::c_int);
    }
}
#[inline(always)]
unsafe extern "C" fn ebmb_delete(mut ebmb: *mut ebmb_node) {
    eb_delete(&mut (*ebmb).node);
}
static mut shctx: *mut shared_context = 0 as *const libc::c_void as *mut libc::c_void
    as *mut shared_context;
static mut shared_session_new_cbk: Option::<
    unsafe extern "C" fn(*mut libc::c_uchar, libc::c_uint, libc::c_long) -> (),
> = None;
pub unsafe extern "C" fn shctx_new_cb(
    mut ssl___0: *mut SSL,
    mut sess: *mut SSL_SESSION,
) -> libc::c_int {
    let mut shsess: *mut shared_session = 0 as *mut shared_session;
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut data_len: libc::c_uint = 0;
    let mut encsess: [libc::c_uchar; 612] = [0; 612];
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut ebmb_node = 0 as *mut ebmb_node;
    let mut tmp___1: libc::c_long = 0;
    tmp = i2d_SSL_SESSION(sess, 0 as *mut libc::c_void as *mut *mut libc::c_uchar);
    data_len = tmp as libc::c_uint;
    if data_len > 512 as libc::c_uint {
        return 1 as libc::c_int;
    }
    data = encsess.as_mut_ptr().offset(32 as libc::c_int as isize);
    p = data;
    i2d_SSL_SESSION(sess, &mut p);
    pthread_mutex_lock(&mut (*shctx).mutex);
    if (*shctx).free.p as libc::c_ulong == (*shctx).free.n as libc::c_ulong {
        shsess = (*shctx).active.p;
    } else {
        shsess = (*shctx).free.p;
    }
    ebmb_delete(&mut (*shsess).key);
    memcpy(
        ((*shsess).key_data).as_mut_ptr() as *mut libc::c_void,
        ((*sess).session_id).as_mut_ptr() as *const libc::c_void,
        (*sess).session_id_length as size_t,
    );
    if (*sess).session_id_length < 32 as libc::c_uint {
        memset(
            ((*shsess).key_data).as_mut_ptr().offset((*sess).session_id_length as isize)
                as *mut libc::c_void,
            0 as libc::c_int,
            (32 as libc::c_uint).wrapping_sub((*sess).session_id_length) as size_t,
        );
    }
    tmp___0 = ebmb_insert(
        &mut (*shctx).active.key.node.branches,
        &mut (*shsess).key,
        32 as libc::c_uint,
    );
    shsess = tmp___0 as *mut shared_session;
    (*shsess).data_len = data_len as libc::c_int;
    memcpy(
        ((*shsess).data).as_mut_ptr() as *mut libc::c_void,
        data as *const libc::c_void,
        data_len as size_t,
    );
    (*shsess).c_date = SSL_SESSION_get_time(sess as *const SSL_SESSION);
    (*(*shsess).n).p = (*shsess).p;
    (*(*shsess).p).n = (*shsess).n;
    (*shsess).p = &mut (*shctx).active;
    (*shsess).n = (*shctx).active.n;
    (*(*shctx).active.n).p = shsess;
    (*shctx).active.n = shsess;
    pthread_mutex_unlock(&mut (*shctx).mutex);
    if shared_session_new_cbk.is_some() {
        memcpy(
            encsess.as_mut_ptr() as *mut libc::c_void,
            ((*sess).session_id).as_mut_ptr() as *const libc::c_void,
            (*sess).session_id_length as size_t,
        );
        if (*sess).session_id_length < 32 as libc::c_uint {
            memset(
                encsess.as_mut_ptr().offset((*sess).session_id_length as isize)
                    as *mut libc::c_void,
                0 as libc::c_int,
                (32 as libc::c_uint).wrapping_sub((*sess).session_id_length) as size_t,
            );
        }
        tmp___1 = SSL_SESSION_get_time(sess as *const SSL_SESSION);
        (Some(shared_session_new_cbk.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            encsess.as_mut_ptr(),
            (32 as libc::c_uint).wrapping_add(data_len),
            tmp___1,
        );
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn shctx_get_cb(
    mut ssl___0: *mut SSL,
    mut key: *mut libc::c_uchar,
    mut key_len: libc::c_int,
    mut do_copy: *mut libc::c_int,
) -> *mut SSL_SESSION {
    let mut shsess: *mut shared_session = 0 as *mut shared_session;
    let mut data: [libc::c_uchar; 512] = [0; 512];
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmpkey: [libc::c_uchar; 32] = [0; 32];
    let mut data_len: libc::c_uint = 0;
    let mut cdate: libc::c_long = 0;
    let mut sess: *mut SSL_SESSION = 0 as *mut SSL_SESSION;
    let mut tmp: *mut ebmb_node = 0 as *mut ebmb_node;
    *do_copy = 0 as libc::c_int;
    if key_len < 32 as libc::c_int {
        memcpy(
            tmpkey.as_mut_ptr() as *mut libc::c_void,
            key as *const libc::c_void,
            key_len as size_t,
        );
        memset(
            tmpkey.as_mut_ptr().offset(key_len as isize) as *mut libc::c_void,
            0 as libc::c_int,
            (32 as libc::c_int - key_len) as size_t,
        );
        key = tmpkey.as_mut_ptr();
    }
    pthread_mutex_lock(&mut (*shctx).mutex);
    tmp = ebmb_lookup(
        &mut (*shctx).active.key.node.branches,
        key as *const libc::c_void,
        32 as libc::c_uint,
    );
    shsess = tmp as *mut shared_session;
    if shsess.is_null() {
        pthread_mutex_unlock(&mut (*shctx).mutex);
        return 0 as *mut libc::c_void as *mut SSL_SESSION;
    }
    cdate = (*shsess).c_date;
    data_len = (*shsess).data_len as libc::c_uint;
    memcpy(
        data.as_mut_ptr() as *mut libc::c_void,
        ((*shsess).data).as_mut_ptr() as *const libc::c_void,
        (*shsess).data_len as size_t,
    );
    (*(*shsess).n).p = (*shsess).p;
    (*(*shsess).p).n = (*shsess).n;
    (*shsess).p = &mut (*shctx).active;
    (*shsess).n = (*shctx).active.n;
    (*(*shctx).active.n).p = shsess;
    (*shctx).active.n = shsess;
    pthread_mutex_unlock(&mut (*shctx).mutex);
    p = data.as_mut_ptr();
    sess = d2i_SSL_SESSION(
        0 as *mut libc::c_void as *mut *mut SSL_SESSION,
        &mut p as *mut *mut libc::c_uchar as *mut *const libc::c_uchar,
        data_len as libc::c_long,
    );
    if !sess.is_null() {
        SSL_SESSION_set_time(sess, cdate);
    }
    return sess;
}
pub unsafe extern "C" fn shctx_remove_cb(
    mut ctx: *mut SSL_CTX,
    mut sess: *mut SSL_SESSION,
) {
    let mut shsess: *mut shared_session = 0 as *mut shared_session;
    let mut tmpkey: [libc::c_uchar; 32] = [0; 32];
    let mut key: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp: *mut ebmb_node = 0 as *mut ebmb_node;
    key = ((*sess).session_id).as_mut_ptr();
    if (*sess).session_id_length < 32 as libc::c_uint {
        memcpy(
            tmpkey.as_mut_ptr() as *mut libc::c_void,
            ((*sess).session_id).as_mut_ptr() as *const libc::c_void,
            (*sess).session_id_length as size_t,
        );
        memset(
            tmpkey.as_mut_ptr().offset((*sess).session_id_length as isize)
                as *mut libc::c_void,
            0 as libc::c_int,
            (32 as libc::c_uint).wrapping_sub((*sess).session_id_length) as size_t,
        );
        key = tmpkey.as_mut_ptr();
    }
    pthread_mutex_lock(&mut (*shctx).mutex);
    tmp = ebmb_lookup(
        &mut (*shctx).active.key.node.branches,
        key as *const libc::c_void,
        32 as libc::c_uint,
    );
    shsess = tmp as *mut shared_session;
    if !shsess.is_null() {
        (*(*shsess).n).p = (*shsess).p;
        (*(*shsess).p).n = (*shsess).n;
        (*shsess).p = &mut (*shctx).free;
        (*shsess).n = (*shctx).free.n;
        (*(*shctx).free.n).p = shsess;
        (*shctx).free.n = shsess;
    }
    pthread_mutex_unlock(&mut (*shctx).mutex);
}
pub unsafe extern "C" fn shctx_sess_add(
    mut encsess: *const libc::c_uchar,
    mut len: libc::c_uint,
    mut cdate: libc::c_long,
) {
    let mut shsess: *mut shared_session = 0 as *mut shared_session;
    let mut tmp: *mut ebmb_node = 0 as *mut ebmb_node;
    if len <= 32 as libc::c_uint {
        return
    } else {
        if len > 544 as libc::c_uint {
            return;
        }
    }
    pthread_mutex_lock(&mut (*shctx).mutex);
    if (*shctx).free.p as libc::c_ulong == (*shctx).free.n as libc::c_ulong {
        shsess = (*shctx).active.p;
    } else {
        shsess = (*shctx).free.p;
    }
    ebmb_delete(&mut (*shsess).key);
    memcpy(
        ((*shsess).key_data).as_mut_ptr() as *mut libc::c_void,
        encsess as *const libc::c_void,
        32 as libc::c_int as size_t,
    );
    tmp = ebmb_insert(
        &mut (*shctx).active.key.node.branches,
        &mut (*shsess).key,
        32 as libc::c_uint,
    );
    shsess = tmp as *mut shared_session;
    if cdate != 0 {
        (*shsess).c_date = cdate;
    }
    (*shsess).data_len = len.wrapping_sub(32 as libc::c_uint) as libc::c_int;
    memcpy(
        ((*shsess).data).as_mut_ptr() as *mut libc::c_void,
        encsess.offset(32 as libc::c_int as isize) as *const libc::c_void,
        (*shsess).data_len as size_t,
    );
    (*(*shsess).n).p = (*shsess).p;
    (*(*shsess).p).n = (*shsess).n;
    (*shsess).p = &mut (*shctx).active;
    (*shsess).n = (*shctx).active.n;
    (*(*shctx).active.n).p = shsess;
    (*shctx).active.n = shsess;
    pthread_mutex_unlock(&mut (*shctx).mutex);
}
pub unsafe extern "C" fn shsess_set_new_cbk(
    mut func: Option::<
        unsafe extern "C" fn(*mut libc::c_uchar, libc::c_uint, libc::c_long) -> (),
    >,
) {
    shared_session_new_cbk = func;
}
pub unsafe extern "C" fn shared_context_init(
    mut ctx: *mut SSL_CTX,
    mut size: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut attr: pthread_mutexattr_t = __anonunion_pthread_mutexattr_t_488594144 {
        __size: [0; 4],
    };
    let mut prev: *mut shared_session = 0 as *mut shared_session;
    let mut cur: *mut shared_session = 0 as *mut shared_session;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut shared_session = 0 as *mut shared_session;
    ret = 0 as libc::c_int;
    if shctx.is_null() {
        tmp = mmap(
            0 as *mut libc::c_void,
            (::std::mem::size_of::<shared_context>() as libc::c_ulong)
                .wrapping_add(
                    (size as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<shared_session>() as libc::c_ulong,
                        ),
                ),
            3 as libc::c_int,
            33 as libc::c_int,
            -(1 as libc::c_int),
            0 as libc::c_int as __off_t,
        );
        shctx = tmp as *mut shared_context;
        if shctx.is_null() {
            return -(1 as libc::c_int)
        } else {
            if shctx as libc::c_ulong
                == -(1 as libc::c_int) as *mut libc::c_void as libc::c_ulong
            {
                return -(1 as libc::c_int);
            }
        }
        pthread_mutexattr_init(&mut attr);
        pthread_mutexattr_setpshared(&mut attr, 1 as libc::c_int);
        pthread_mutex_init(
            &mut (*shctx).mutex,
            &mut attr as *mut pthread_mutexattr_t as *const pthread_mutexattr_t,
        );
        memset(
            &mut (*shctx).active.key as *mut ebmb_node as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<ebmb_node>() as libc::c_ulong,
        );
        memset(
            &mut (*shctx).free.key as *mut ebmb_node as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<ebmb_node>() as libc::c_ulong,
        );
        (*shctx)
            .active
            .key
            .node
            .branches
            .b[1 as libc::c_int as usize] = 1 as libc::c_int as *mut libc::c_void;
        cur = &mut (*shctx).active;
        tmp___0 = cur;
        (*cur).p = tmp___0;
        (*cur).n = tmp___0;
        cur = &mut (*shctx).free;
        i = 0 as libc::c_int;
        while i < size {
            prev = cur;
            cur = (prev as *mut libc::c_char)
                .offset(
                    ::std::mem::size_of::<shared_session>() as libc::c_ulong as isize,
                ) as *mut shared_session;
            (*prev).n = cur;
            (*cur).p = prev;
            i += 1;
        }
        (*cur).n = &mut (*shctx).free;
        (*shctx).free.p = cur;
        ret = size;
    }
    SSL_CTX_ctrl(
        ctx,
        42 as libc::c_int,
        (size >> 3 as libc::c_int | 1023 as libc::c_int) as libc::c_long,
        0 as *mut libc::c_void,
    );
    SSL_CTX_sess_set_new_cb(
        ctx,
        Some(
            shctx_new_cb
                as unsafe extern "C" fn(*mut SSL, *mut SSL_SESSION) -> libc::c_int,
        ),
    );
    SSL_CTX_sess_set_get_cb(
        ctx,
        Some(
            shctx_get_cb
                as unsafe extern "C" fn(
                    *mut SSL,
                    *mut libc::c_uchar,
                    libc::c_int,
                    *mut libc::c_int,
                ) -> *mut SSL_SESSION,
        ),
    );
    SSL_CTX_sess_set_remove_cb(
        ctx,
        Some(
            shctx_remove_cb as unsafe extern "C" fn(*mut SSL_CTX, *mut SSL_SESSION) -> (),
        ),
    );
    return ret;
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
