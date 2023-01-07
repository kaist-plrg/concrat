use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ck_ht_map;
    pub type rd_kafka_s;
    pub type MHD_Daemon;
    pub type MHD_Connection;
    pub type MHD_Response;
    pub type rd_kafka_topic_s;
    pub type rd_kafka_conf_s;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut optarg: *mut libc::c_char;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn clock_nanosleep(
        __clock_id: clockid_t,
        __flags: libc::c_int,
        __req: *const timespec,
        __rem: *mut timespec,
    ) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn json_unpack_ex(
        root: *mut json_t,
        error: *mut json_error_t,
        flags: size_t,
        fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn log(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: Option::<
            unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        >,
    );
    fn floor(_: libc::c_double) -> libc::c_double;
    fn free(__ptr: *mut libc::c_void);
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn ck_ht_next(
        _: *mut ck_ht_t,
        _: *mut ck_ht_iterator_t,
        entry: *mut *mut ck_ht_entry_t,
    ) -> bool;
    fn ck_ht_hash(
        _: *mut ck_ht_hash_t,
        _: *mut ck_ht_t,
        _: *const libc::c_void,
        _: uint16_t,
    );
    fn ck_ht_init(
        _: *mut ck_ht_t,
        _: libc::c_uint,
        _: Option::<ck_ht_hash_cb_t>,
        _: *mut ck_malloc,
        _: uint64_t,
        _: uint64_t,
    ) -> bool;
    fn ck_ht_put_spmc(_: *mut ck_ht_t, _: ck_ht_hash_t, _: *mut ck_ht_entry_t) -> bool;
    fn ck_ht_get_spmc(_: *mut ck_ht_t, _: ck_ht_hash_t, _: *mut ck_ht_entry_t) -> bool;
    fn ck_ht_count(_: *mut ck_ht_t) -> uint64_t;
    fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: socklen_t,
    ) -> *const libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn getpid() -> __pid_t;
    fn json_array() -> *mut json_t;
    fn json_delete(json: *mut json_t);
    fn json_array_append_new(array: *mut json_t, value: *mut json_t) -> libc::c_int;
    fn json_pack(fmt: *const libc::c_char, _: ...) -> *mut json_t;
    fn json_dumps(json: *const json_t, flags: size_t) -> *mut libc::c_char;
    fn MHD_start_daemon(
        flags: libc::c_uint,
        port: uint16_t,
        apc: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *const sockaddr,
                socklen_t,
            ) -> libc::c_int,
        >,
        apc_cls: *mut libc::c_void,
        dh: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *mut MHD_Connection,
                *const libc::c_char,
                *const libc::c_char,
                *const libc::c_char,
                *const libc::c_char,
                *mut size_t,
                *mut *mut libc::c_void,
            ) -> libc::c_int,
        >,
        dh_cls: *mut libc::c_void,
        _: ...
    ) -> *mut MHD_Daemon;
    fn MHD_queue_response(
        connection: *mut MHD_Connection,
        status_code: libc::c_uint,
        response: *mut MHD_Response,
    ) -> libc::c_int;
    fn MHD_create_response_from_buffer(
        size: size_t,
        buffer: *mut libc::c_void,
        mode: MHD_ResponseMemoryMode,
    ) -> *mut MHD_Response;
    fn MHD_destroy_response(response: *mut MHD_Response);
    fn MHD_add_response_header(
        response: *mut MHD_Response,
        header: *const libc::c_char,
        content: *const libc::c_char,
    ) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn closelog();
    fn openlog(
        __ident: *const libc::c_char,
        __option: libc::c_int,
        __facility: libc::c_int,
    );
    fn vsyslog(__pri: libc::c_int, __fmt: *const libc::c_char, __ap: ::std::ffi::VaList);
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn pthread_spin_init(
        __lock: *mut pthread_spinlock_t,
        __pshared: libc::c_int,
    ) -> libc::c_int;
    fn pthread_spin_lock(__lock: *mut pthread_spinlock_t) -> libc::c_int;
    fn pthread_spin_unlock(__lock: *mut pthread_spinlock_t) -> libc::c_int;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t) -> libc::c_int;
    fn recvfrom(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> ssize_t;
    fn recvmmsg(
        __fd: libc::c_int,
        __vmessages: *mut mmsghdr,
        __vlen: libc::c_uint,
        __flags: libc::c_int,
        __tmo: *mut timespec,
    ) -> libc::c_int;
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
    fn pthread_cancel(__th: pthread_t) -> libc::c_int;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
    fn signal(
        __sig: libc::c_int,
        __handler: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> __sighandler_t;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn signalfd(
        __fd: libc::c_int,
        __mask: *const sigset_t,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn timerfd_create(__clock_id: __clockid_t, __flags: libc::c_int) -> libc::c_int;
    fn timerfd_settime(
        __ufd: libc::c_int,
        __flags: libc::c_int,
        __utmr: *const itimerspec,
        __otmr: *mut itimerspec,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn json_object_get(object: *const json_t, key: *const libc::c_char) -> *mut json_t;
    fn json_array_size(array: *const json_t) -> size_t;
    fn json_array_get(array: *const json_t, index: size_t) -> *mut json_t;
    fn json_string_value(string: *const json_t) -> *const libc::c_char;
    fn json_load_file(
        path: *const libc::c_char,
        flags: size_t,
        error: *mut json_error_t,
    ) -> *mut json_t;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    static mut environ: *mut *mut libc::c_char;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
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
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn json_object() -> *mut json_t;
    fn json_string(value: *const libc::c_char) -> *mut json_t;
    fn json_integer(value: json_int_t) -> *mut json_t;
    fn json_real(value: libc::c_double) -> *mut json_t;
    fn json_object_set_new_nocheck(
        object: *mut json_t,
        key: *const libc::c_char,
        value: *mut json_t,
    ) -> libc::c_int;
    fn json_object_clear(object: *mut json_t) -> libc::c_int;
    fn json_object_iter(object: *mut json_t) -> *mut libc::c_void;
    fn json_object_key_to_iter(key: *const libc::c_char) -> *mut libc::c_void;
    fn json_object_iter_next(
        object: *mut json_t,
        iter: *mut libc::c_void,
    ) -> *mut libc::c_void;
    fn json_object_iter_key(iter: *mut libc::c_void) -> *const libc::c_char;
    fn json_object_iter_value(iter: *mut libc::c_void) -> *mut json_t;
    fn rd_kafka_err2str(err: rd_kafka_resp_err_t) -> *const libc::c_char;
    fn rd_kafka_err2name(err: rd_kafka_resp_err_t) -> *const libc::c_char;
    fn rd_kafka_fatal_error(
        rk: *mut rd_kafka_t,
        errstr_0: *mut libc::c_char,
        errstr_size: size_t,
    ) -> rd_kafka_resp_err_t;
    fn rd_kafka_conf_new() -> *mut rd_kafka_conf_t;
    fn rd_kafka_conf_destroy(conf: *mut rd_kafka_conf_t);
    fn rd_kafka_conf_set(
        conf: *mut rd_kafka_conf_t,
        name: *const libc::c_char,
        value: *const libc::c_char,
        errstr_0: *mut libc::c_char,
        errstr_size: size_t,
    ) -> rd_kafka_conf_res_t;
    fn rd_kafka_conf_set_dr_msg_cb(
        conf: *mut rd_kafka_conf_t,
        dr_msg_cb_0: Option::<
            unsafe extern "C" fn(
                *mut rd_kafka_t,
                *const rd_kafka_message_t,
                *mut libc::c_void,
            ) -> (),
        >,
    );
    fn rd_kafka_conf_set_error_cb(
        conf: *mut rd_kafka_conf_t,
        error_cb_0: Option::<
            unsafe extern "C" fn(
                *mut rd_kafka_t,
                libc::c_int,
                *const libc::c_char,
                *mut libc::c_void,
            ) -> (),
        >,
    );
    fn rd_kafka_conf_set_opaque(conf: *mut rd_kafka_conf_t, opaque: *mut libc::c_void);
    fn rd_kafka_new(
        type_0: rd_kafka_type_t,
        conf: *mut rd_kafka_conf_t,
        errstr_0: *mut libc::c_char,
        errstr_size: size_t,
    ) -> *mut rd_kafka_t;
    fn rd_kafka_destroy(rk: *mut rd_kafka_t);
    fn rd_kafka_poll(rk: *mut rd_kafka_t, timeout_ms: libc::c_int) -> libc::c_int;
    fn rd_kafka_producev(rk: *mut rd_kafka_t, _: ...) -> rd_kafka_resp_err_t;
    fn rd_kafka_flush(
        rk: *mut rd_kafka_t,
        timeout_ms: libc::c_int,
    ) -> rd_kafka_resp_err_t;
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
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
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
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_mutex_t_335460617 {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type pthread_mutex_t = __anonunion_pthread_mutex_t_335460617;
pub type pthread_spinlock_t = libc::c_int;
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
pub struct msghdr {
    pub msg_name: *mut libc::c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: size_t,
    pub msg_control: *mut libc::c_void,
    pub msg_controllen: size_t,
    pub msg_flags: libc::c_int,
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
pub type value_t = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_server {
    pub name: *const libc::c_char,
    pub dump_path: *const libc::c_char,
    pub config_name: *const libc::c_char,
    pub running: libc::c_int,
    pub active_backends: libc::c_int,
    pub active_samplers: libc::c_int,
    pub set_proctitle: bool,
    pub fd_signal: libc::c_int,
    pub fd_expire: libc::c_int,
    pub fd_update: libc::c_int,
    pub slab: brubeck_slab,
    pub metrics: *mut brubeck_hashtable_t,
    pub tags: *mut brubeck_tags_t,
    pub at_capacity: libc::c_int,
    pub samplers: [*mut brubeck_sampler; 8],
    pub backends: [*mut brubeck_backend; 8],
    pub config: *mut json_t,
    pub internal_stats: brubeck_internal_stats,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_internal_stats {
    pub sample_freq: libc::c_int,
    pub live: __anonstruct_live_669259541,
    pub sample: __anonstruct_live_669259541,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_live_669259541 {
    pub metrics: uint32_t,
    pub errors: uint32_t,
    pub unique_keys: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_t {
    pub type_0: json_type,
    pub refcount: size_t,
}
pub type json_type = __anonenum_json_type_587412934;
pub type __anonenum_json_type_587412934 = libc::c_uint;
pub const JSON_NULL: __anonenum_json_type_587412934 = 7;
pub const JSON_FALSE: __anonenum_json_type_587412934 = 6;
pub const JSON_TRUE: __anonenum_json_type_587412934 = 5;
pub const JSON_REAL: __anonenum_json_type_587412934 = 4;
pub const JSON_INTEGER: __anonenum_json_type_587412934 = 3;
pub const JSON_STRING: __anonenum_json_type_587412934 = 2;
pub const JSON_ARRAY: __anonenum_json_type_587412934 = 1;
pub const JSON_OBJECT: __anonenum_json_type_587412934 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_backend {
    pub type_0: brubeck_backend_t,
    pub server: *mut brubeck_server,
    pub sample_freq: libc::c_int,
    pub shard_n: libc::c_int,
    pub connect: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub is_connected: Option::<unsafe extern "C" fn(*mut libc::c_void) -> bool>,
    pub sample: Option::<
        unsafe extern "C" fn(
            *const brubeck_metric,
            *const libc::c_char,
            value_t,
            *mut libc::c_void,
        ) -> (),
    >,
    pub flush: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub tick_time: uint32_t,
    pub thread: pthread_t,
    pub queue: *mut brubeck_metric,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_metric {
    pub next: *mut brubeck_metric,
    pub tags: *const brubeck_tag_set,
    pub lock: pthread_spinlock_t,
    pub key_len: uint16_t,
    pub type_0: uint8_t,
    pub private_state: uint8_t,
    pub as_0: __anonunion_as_920192508,
    pub key: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_as_920192508 {
    pub gauge: __anonstruct_gauge_908274918,
    pub meter: __anonstruct_gauge_908274918,
    pub counter: __anonstruct_counter_908274919,
    pub histogram: brubeck_histo,
    pub other: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_histo {
    pub values: *mut value_t,
    pub count: uint32_t,
    pub alloc: uint16_t,
    pub size: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_counter_908274919 {
    pub value: value_t,
    pub previous: value_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_gauge_908274918 {
    pub value: value_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_tag_set {
    pub index: uint32_t,
    pub tag_len: uint16_t,
    pub num_tags: uint16_t,
    pub tags: [brubeck_tag; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_tag {
    pub key: *const libc::c_char,
    pub value: *const libc::c_char,
}
pub type brubeck_backend_t = libc::c_uint;
pub const BRUBECK_BACKEND_KAFKA: brubeck_backend_t = 1;
pub const BRUBECK_BACKEND_CARBON: brubeck_backend_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_sampler {
    pub type_0: brubeck_sampler_t,
    pub server: *mut brubeck_server,
    pub in_sock: libc::c_int,
    pub addr: sockaddr_in,
    pub inflow: size_t,
    pub current_flow: size_t,
    pub shutdown: Option::<unsafe extern "C" fn(*mut brubeck_sampler) -> ()>,
}
pub type brubeck_sampler_t = libc::c_uint;
pub const BRUBECK_SAMPLER_STATSD: brubeck_sampler_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_tags_t {
    pub num_tag_sets: uint32_t,
    pub table: ck_ht_t,
    pub write_mutex: pthread_mutex_t,
}
pub type ck_ht_t = ck_ht;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ck_ht {
    pub m: *mut ck_malloc,
    pub map: *mut ck_ht_map,
    pub mode: libc::c_uint,
    pub seed: uint64_t,
    pub h: Option::<ck_ht_hash_cb_t>,
}
pub type ck_ht_hash_cb_t = unsafe extern "C" fn(
    *mut ck_ht_hash_t,
    *const libc::c_void,
    size_t,
    uint64_t,
) -> ();
pub type uint64_t = __uint64_t;
pub type __uint64_t = libc::c_ulong;
pub type ck_ht_hash_t = ck_ht_hash;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ck_ht_hash {
    pub value: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ck_malloc {
    pub malloc: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub realloc: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            size_t,
            size_t,
            bool,
        ) -> *mut libc::c_void,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void, size_t, bool) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_hashtable_t {
    pub table: ck_ht_t,
    pub write_mutex: pthread_mutex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_slab {
    pub current: *mut brubeck_slab_node,
    pub total_alloc: size_t,
    pub lock: pthread_mutex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_slab_node {
    pub next: *mut brubeck_slab_node,
    pub alloc: size_t,
    pub heap: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type __clockid_t = libc::c_int;
pub type clockid_t = __clockid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_error_t {
    pub line: libc::c_int,
    pub column: libc::c_int,
    pub position: libc::c_int,
    pub source: [libc::c_char; 80],
    pub text: [libc::c_char; 160],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pickler {
    pub ptr: *mut libc::c_char,
    pub pos: uint16_t,
    pub pt: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_carbon {
    pub backend: brubeck_backend,
    pub out_sock: libc::c_int,
    pub out_sockaddr: sockaddr_in,
    pub pickler: pickler,
    pub bytes_sent: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct multibloom {
    pub bits: libc::c_int,
    pub bytes: libc::c_int,
    pub hashes: libc::c_int,
    pub filters: [*mut libc::c_uchar; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_histo_sample {
    pub sum: value_t,
    pub min: value_t,
    pub max: value_t,
    pub mean: value_t,
    pub median: value_t,
    pub count: value_t,
    pub percentile: [value_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_mutexattr_t_488594144 {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
pub type pthread_mutexattr_t = __anonunion_pthread_mutexattr_t_488594144;
pub type uintptr_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ck_ht_entry {
    pub key: uintptr_t,
    pub value: uintptr_t,
    pub key_length: uint64_t,
    pub hash: uint64_t,
}
pub type ck_ht_entry_t = ck_ht_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ck_ht_iterator {
    pub current: *mut ck_ht_entry,
    pub offset: uint64_t,
}
pub type ck_ht_iterator_t = ck_ht_iterator;
pub type __pid_t = libc::c_int;
pub type json_int_t = libc::c_longlong;
pub type rd_kafka_t = rd_kafka_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_kafka_document {
    pub json: *mut json_t,
    pub is_dirty: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_kafka {
    pub backend: brubeck_backend,
    pub rk: *mut rd_kafka_t,
    pub connected: bool,
    pub topic: *const libc::c_char,
    pub tag_subdocument: *const libc::c_char,
    pub bytes_sent: size_t,
    pub documents: *mut *mut brubeck_kafka_document,
}
pub type MHD_ResponseMemoryMode = libc::c_uint;
pub const MHD_RESPMEM_MUST_COPY: MHD_ResponseMemoryMode = 2;
pub const MHD_RESPMEM_MUST_FREE: MHD_ResponseMemoryMode = 1;
pub const MHD_RESPMEM_PERSISTENT: MHD_ResponseMemoryMode = 0;
pub type va_list___0 = __gnuc_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_metric__proto {
    pub record: Option::<
        unsafe extern "C" fn(*mut brubeck_metric, value_t, value_t, uint8_t) -> (),
    >,
    pub sample: Option::<
        unsafe extern "C" fn(
            *mut brubeck_metric,
            Option::<
                unsafe extern "C" fn(
                    *const brubeck_metric,
                    *const libc::c_char,
                    value_t,
                    *mut libc::c_void,
                ) -> (),
            >,
            *mut libc::c_void,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mmsghdr {
    pub msg_hdr: msghdr,
    pub msg_len: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_statsd_msg {
    pub key: *mut libc::c_char,
    pub key_len: uint16_t,
    pub type_0: uint16_t,
    pub value: value_t,
    pub sample_freq: value_t,
    pub modifiers: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_statsd {
    pub sampler: brubeck_sampler,
    pub workers: *mut pthread_t,
    pub worker_count: libc::c_uint,
    pub mmsg_count: libc::c_uint,
    pub scale_timers_by: libc::c_double,
}
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub type __int32_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___sigset_t_991265788 {
    pub __val: [libc::c_ulong; 16],
}
pub type __sigset_t = __anonstruct___sigset_t_991265788;
pub type sigset_t = __sigset_t;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct signalfd_siginfo {
    pub ssi_signo: uint32_t,
    pub ssi_errno: int32_t,
    pub ssi_code: int32_t,
    pub ssi_pid: uint32_t,
    pub ssi_uid: uint32_t,
    pub ssi_fd: int32_t,
    pub ssi_tid: uint32_t,
    pub ssi_band: uint32_t,
    pub ssi_overrun: uint32_t,
    pub ssi_trapno: uint32_t,
    pub ssi_status: int32_t,
    pub ssi_int: int32_t,
    pub ssi_ptr: uint64_t,
    pub ssi_utime: uint64_t,
    pub ssi_stime: uint64_t,
    pub ssi_addr: uint64_t,
    pub ssi_addr_lsb: uint16_t,
    pub __pad2: uint16_t,
    pub ssi_syscall: int32_t,
    pub ssi_call_addr: uint64_t,
    pub ssi_arch: uint32_t,
    pub __pad: [uint8_t; 28],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
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
pub union __anonunion_x_375160861 {
    pub L: libc::c_int,
    pub F: libc::c_float,
}
pub type __int64_t = libc::c_long;
pub type int64_t = __int64_t;
pub type rd_kafka_type_t = libc::c_uint;
pub const RD_KAFKA_CONSUMER: rd_kafka_type_t = 1;
pub const RD_KAFKA_PRODUCER: rd_kafka_type_t = 0;
pub type rd_kafka_topic_t = rd_kafka_topic_s;
pub type rd_kafka_conf_t = rd_kafka_conf_s;
pub type __anonenum_rd_kafka_resp_err_t_686298611 = libc::c_int;
pub const RD_KAFKA_RESP_ERR_END_ALL: __anonenum_rd_kafka_resp_err_t_686298611 = 82;
pub const RD_KAFKA_RESP_ERR_GROUP_MAX_SIZE_REACHED: __anonenum_rd_kafka_resp_err_t_686298611 = 81;
pub const RD_KAFKA_RESP_ERR_PREFERRED_LEADER_NOT_AVAILABLE: __anonenum_rd_kafka_resp_err_t_686298611 = 80;
pub const RD_KAFKA_RESP_ERR_MEMBER_ID_REQUIRED: __anonenum_rd_kafka_resp_err_t_686298611 = 79;
pub const RD_KAFKA_RESP_ERR_OFFSET_NOT_AVAILABLE: __anonenum_rd_kafka_resp_err_t_686298611 = 78;
pub const RD_KAFKA_RESP_ERR_STALE_BROKER_EPOCH: __anonenum_rd_kafka_resp_err_t_686298611 = 77;
pub const RD_KAFKA_RESP_ERR_UNSUPPORTED_COMPRESSION_TYPE: __anonenum_rd_kafka_resp_err_t_686298611 = 76;
pub const RD_KAFKA_RESP_ERR_UNKNOWN_LEADER_EPOCH: __anonenum_rd_kafka_resp_err_t_686298611 = 75;
pub const RD_KAFKA_RESP_ERR_FENCED_LEADER_EPOCH: __anonenum_rd_kafka_resp_err_t_686298611 = 74;
pub const RD_KAFKA_RESP_ERR_TOPIC_DELETION_DISABLED: __anonenum_rd_kafka_resp_err_t_686298611 = 73;
pub const RD_KAFKA_RESP_ERR_LISTENER_NOT_FOUND: __anonenum_rd_kafka_resp_err_t_686298611 = 72;
pub const RD_KAFKA_RESP_ERR_INVALID_FETCH_SESSION_EPOCH: __anonenum_rd_kafka_resp_err_t_686298611 = 71;
pub const RD_KAFKA_RESP_ERR_FETCH_SESSION_ID_NOT_FOUND: __anonenum_rd_kafka_resp_err_t_686298611 = 70;
pub const RD_KAFKA_RESP_ERR_GROUP_ID_NOT_FOUND: __anonenum_rd_kafka_resp_err_t_686298611 = 69;
pub const RD_KAFKA_RESP_ERR_NON_EMPTY_GROUP: __anonenum_rd_kafka_resp_err_t_686298611 = 68;
pub const RD_KAFKA_RESP_ERR_INVALID_PRINCIPAL_TYPE: __anonenum_rd_kafka_resp_err_t_686298611 = 67;
pub const RD_KAFKA_RESP_ERR_DELEGATION_TOKEN_EXPIRED: __anonenum_rd_kafka_resp_err_t_686298611 = 66;
pub const RD_KAFKA_RESP_ERR_DELEGATION_TOKEN_AUTHORIZATION_FAILED: __anonenum_rd_kafka_resp_err_t_686298611 = 65;
pub const RD_KAFKA_RESP_ERR_DELEGATION_TOKEN_REQUEST_NOT_ALLOWED: __anonenum_rd_kafka_resp_err_t_686298611 = 64;
pub const RD_KAFKA_RESP_ERR_DELEGATION_TOKEN_OWNER_MISMATCH: __anonenum_rd_kafka_resp_err_t_686298611 = 63;
pub const RD_KAFKA_RESP_ERR_DELEGATION_TOKEN_NOT_FOUND: __anonenum_rd_kafka_resp_err_t_686298611 = 62;
pub const RD_KAFKA_RESP_ERR_DELEGATION_TOKEN_AUTH_DISABLED: __anonenum_rd_kafka_resp_err_t_686298611 = 61;
pub const RD_KAFKA_RESP_ERR_REASSIGNMENT_IN_PROGRESS: __anonenum_rd_kafka_resp_err_t_686298611 = 60;
pub const RD_KAFKA_RESP_ERR_UNKNOWN_PRODUCER_ID: __anonenum_rd_kafka_resp_err_t_686298611 = 59;
pub const RD_KAFKA_RESP_ERR_SASL_AUTHENTICATION_FAILED: __anonenum_rd_kafka_resp_err_t_686298611 = 58;
pub const RD_KAFKA_RESP_ERR_LOG_DIR_NOT_FOUND: __anonenum_rd_kafka_resp_err_t_686298611 = 57;
pub const RD_KAFKA_RESP_ERR_KAFKA_STORAGE_ERROR: __anonenum_rd_kafka_resp_err_t_686298611 = 56;
pub const RD_KAFKA_RESP_ERR_OPERATION_NOT_ATTEMPTED: __anonenum_rd_kafka_resp_err_t_686298611 = 55;
pub const RD_KAFKA_RESP_ERR_SECURITY_DISABLED: __anonenum_rd_kafka_resp_err_t_686298611 = 54;
pub const RD_KAFKA_RESP_ERR_TRANSACTIONAL_ID_AUTHORIZATION_FAILED: __anonenum_rd_kafka_resp_err_t_686298611 = 53;
pub const RD_KAFKA_RESP_ERR_TRANSACTION_COORDINATOR_FENCED: __anonenum_rd_kafka_resp_err_t_686298611 = 52;
pub const RD_KAFKA_RESP_ERR_CONCURRENT_TRANSACTIONS: __anonenum_rd_kafka_resp_err_t_686298611 = 51;
pub const RD_KAFKA_RESP_ERR_INVALID_TRANSACTION_TIMEOUT: __anonenum_rd_kafka_resp_err_t_686298611 = 50;
pub const RD_KAFKA_RESP_ERR_INVALID_PRODUCER_ID_MAPPING: __anonenum_rd_kafka_resp_err_t_686298611 = 49;
pub const RD_KAFKA_RESP_ERR_INVALID_TXN_STATE: __anonenum_rd_kafka_resp_err_t_686298611 = 48;
pub const RD_KAFKA_RESP_ERR_INVALID_PRODUCER_EPOCH: __anonenum_rd_kafka_resp_err_t_686298611 = 47;
pub const RD_KAFKA_RESP_ERR_DUPLICATE_SEQUENCE_NUMBER: __anonenum_rd_kafka_resp_err_t_686298611 = 46;
pub const RD_KAFKA_RESP_ERR_OUT_OF_ORDER_SEQUENCE_NUMBER: __anonenum_rd_kafka_resp_err_t_686298611 = 45;
pub const RD_KAFKA_RESP_ERR_POLICY_VIOLATION: __anonenum_rd_kafka_resp_err_t_686298611 = 44;
pub const RD_KAFKA_RESP_ERR_UNSUPPORTED_FOR_MESSAGE_FORMAT: __anonenum_rd_kafka_resp_err_t_686298611 = 43;
pub const RD_KAFKA_RESP_ERR_INVALID_REQUEST: __anonenum_rd_kafka_resp_err_t_686298611 = 42;
pub const RD_KAFKA_RESP_ERR_NOT_CONTROLLER: __anonenum_rd_kafka_resp_err_t_686298611 = 41;
pub const RD_KAFKA_RESP_ERR_INVALID_CONFIG: __anonenum_rd_kafka_resp_err_t_686298611 = 40;
pub const RD_KAFKA_RESP_ERR_INVALID_REPLICA_ASSIGNMENT: __anonenum_rd_kafka_resp_err_t_686298611 = 39;
pub const RD_KAFKA_RESP_ERR_INVALID_REPLICATION_FACTOR: __anonenum_rd_kafka_resp_err_t_686298611 = 38;
pub const RD_KAFKA_RESP_ERR_INVALID_PARTITIONS: __anonenum_rd_kafka_resp_err_t_686298611 = 37;
pub const RD_KAFKA_RESP_ERR_TOPIC_ALREADY_EXISTS: __anonenum_rd_kafka_resp_err_t_686298611 = 36;
pub const RD_KAFKA_RESP_ERR_UNSUPPORTED_VERSION: __anonenum_rd_kafka_resp_err_t_686298611 = 35;
pub const RD_KAFKA_RESP_ERR_ILLEGAL_SASL_STATE: __anonenum_rd_kafka_resp_err_t_686298611 = 34;
pub const RD_KAFKA_RESP_ERR_UNSUPPORTED_SASL_MECHANISM: __anonenum_rd_kafka_resp_err_t_686298611 = 33;
pub const RD_KAFKA_RESP_ERR_INVALID_TIMESTAMP: __anonenum_rd_kafka_resp_err_t_686298611 = 32;
pub const RD_KAFKA_RESP_ERR_CLUSTER_AUTHORIZATION_FAILED: __anonenum_rd_kafka_resp_err_t_686298611 = 31;
pub const RD_KAFKA_RESP_ERR_GROUP_AUTHORIZATION_FAILED: __anonenum_rd_kafka_resp_err_t_686298611 = 30;
pub const RD_KAFKA_RESP_ERR_TOPIC_AUTHORIZATION_FAILED: __anonenum_rd_kafka_resp_err_t_686298611 = 29;
pub const RD_KAFKA_RESP_ERR_INVALID_COMMIT_OFFSET_SIZE: __anonenum_rd_kafka_resp_err_t_686298611 = 28;
pub const RD_KAFKA_RESP_ERR_REBALANCE_IN_PROGRESS: __anonenum_rd_kafka_resp_err_t_686298611 = 27;
pub const RD_KAFKA_RESP_ERR_INVALID_SESSION_TIMEOUT: __anonenum_rd_kafka_resp_err_t_686298611 = 26;
pub const RD_KAFKA_RESP_ERR_UNKNOWN_MEMBER_ID: __anonenum_rd_kafka_resp_err_t_686298611 = 25;
pub const RD_KAFKA_RESP_ERR_INVALID_GROUP_ID: __anonenum_rd_kafka_resp_err_t_686298611 = 24;
pub const RD_KAFKA_RESP_ERR_INCONSISTENT_GROUP_PROTOCOL: __anonenum_rd_kafka_resp_err_t_686298611 = 23;
pub const RD_KAFKA_RESP_ERR_ILLEGAL_GENERATION: __anonenum_rd_kafka_resp_err_t_686298611 = 22;
pub const RD_KAFKA_RESP_ERR_INVALID_REQUIRED_ACKS: __anonenum_rd_kafka_resp_err_t_686298611 = 21;
pub const RD_KAFKA_RESP_ERR_NOT_ENOUGH_REPLICAS_AFTER_APPEND: __anonenum_rd_kafka_resp_err_t_686298611 = 20;
pub const RD_KAFKA_RESP_ERR_NOT_ENOUGH_REPLICAS: __anonenum_rd_kafka_resp_err_t_686298611 = 19;
pub const RD_KAFKA_RESP_ERR_RECORD_LIST_TOO_LARGE: __anonenum_rd_kafka_resp_err_t_686298611 = 18;
pub const RD_KAFKA_RESP_ERR_TOPIC_EXCEPTION: __anonenum_rd_kafka_resp_err_t_686298611 = 17;
pub const RD_KAFKA_RESP_ERR_NOT_COORDINATOR_FOR_GROUP: __anonenum_rd_kafka_resp_err_t_686298611 = 16;
pub const RD_KAFKA_RESP_ERR_GROUP_COORDINATOR_NOT_AVAILABLE: __anonenum_rd_kafka_resp_err_t_686298611 = 15;
pub const RD_KAFKA_RESP_ERR_GROUP_LOAD_IN_PROGRESS: __anonenum_rd_kafka_resp_err_t_686298611 = 14;
pub const RD_KAFKA_RESP_ERR_NETWORK_EXCEPTION: __anonenum_rd_kafka_resp_err_t_686298611 = 13;
pub const RD_KAFKA_RESP_ERR_OFFSET_METADATA_TOO_LARGE: __anonenum_rd_kafka_resp_err_t_686298611 = 12;
pub const RD_KAFKA_RESP_ERR_STALE_CTRL_EPOCH: __anonenum_rd_kafka_resp_err_t_686298611 = 11;
pub const RD_KAFKA_RESP_ERR_MSG_SIZE_TOO_LARGE: __anonenum_rd_kafka_resp_err_t_686298611 = 10;
pub const RD_KAFKA_RESP_ERR_REPLICA_NOT_AVAILABLE: __anonenum_rd_kafka_resp_err_t_686298611 = 9;
pub const RD_KAFKA_RESP_ERR_BROKER_NOT_AVAILABLE: __anonenum_rd_kafka_resp_err_t_686298611 = 8;
pub const RD_KAFKA_RESP_ERR_REQUEST_TIMED_OUT: __anonenum_rd_kafka_resp_err_t_686298611 = 7;
pub const RD_KAFKA_RESP_ERR_NOT_LEADER_FOR_PARTITION: __anonenum_rd_kafka_resp_err_t_686298611 = 6;
pub const RD_KAFKA_RESP_ERR_LEADER_NOT_AVAILABLE: __anonenum_rd_kafka_resp_err_t_686298611 = 5;
pub const RD_KAFKA_RESP_ERR_INVALID_MSG_SIZE: __anonenum_rd_kafka_resp_err_t_686298611 = 4;
pub const RD_KAFKA_RESP_ERR_UNKNOWN_TOPIC_OR_PART: __anonenum_rd_kafka_resp_err_t_686298611 = 3;
pub const RD_KAFKA_RESP_ERR_INVALID_MSG: __anonenum_rd_kafka_resp_err_t_686298611 = 2;
pub const RD_KAFKA_RESP_ERR_OFFSET_OUT_OF_RANGE: __anonenum_rd_kafka_resp_err_t_686298611 = 1;
pub const RD_KAFKA_RESP_ERR_NO_ERROR: __anonenum_rd_kafka_resp_err_t_686298611 = 0;
pub const RD_KAFKA_RESP_ERR_UNKNOWN: __anonenum_rd_kafka_resp_err_t_686298611 = -1;
pub const RD_KAFKA_RESP_ERR__END: __anonenum_rd_kafka_resp_err_t_686298611 = -100;
pub const RD_KAFKA_RESP_ERR__MAX_POLL_EXCEEDED: __anonenum_rd_kafka_resp_err_t_686298611 = -147;
pub const RD_KAFKA_RESP_ERR__GAPLESS_GUARANTEE: __anonenum_rd_kafka_resp_err_t_686298611 = -148;
pub const RD_KAFKA_RESP_ERR__INCONSISTENT: __anonenum_rd_kafka_resp_err_t_686298611 = -149;
pub const RD_KAFKA_RESP_ERR__FATAL: __anonenum_rd_kafka_resp_err_t_686298611 = -150;
pub const RD_KAFKA_RESP_ERR__PURGE_INFLIGHT: __anonenum_rd_kafka_resp_err_t_686298611 = -151;
pub const RD_KAFKA_RESP_ERR__PURGE_QUEUE: __anonenum_rd_kafka_resp_err_t_686298611 = -152;
pub const RD_KAFKA_RESP_ERR__RETRY: __anonenum_rd_kafka_resp_err_t_686298611 = -153;
pub const RD_KAFKA_RESP_ERR__INVALID_TYPE: __anonenum_rd_kafka_resp_err_t_686298611 = -154;
pub const RD_KAFKA_RESP_ERR__UNDERFLOW: __anonenum_rd_kafka_resp_err_t_686298611 = -155;
pub const RD_KAFKA_RESP_ERR__NOENT: __anonenum_rd_kafka_resp_err_t_686298611 = -156;
pub const RD_KAFKA_RESP_ERR__READ_ONLY: __anonenum_rd_kafka_resp_err_t_686298611 = -157;
pub const RD_KAFKA_RESP_ERR__PARTIAL: __anonenum_rd_kafka_resp_err_t_686298611 = -158;
pub const RD_KAFKA_RESP_ERR__VALUE_DESERIALIZATION: __anonenum_rd_kafka_resp_err_t_686298611 = -159;
pub const RD_KAFKA_RESP_ERR__KEY_DESERIALIZATION: __anonenum_rd_kafka_resp_err_t_686298611 = -160;
pub const RD_KAFKA_RESP_ERR__VALUE_SERIALIZATION: __anonenum_rd_kafka_resp_err_t_686298611 = -161;
pub const RD_KAFKA_RESP_ERR__KEY_SERIALIZATION: __anonenum_rd_kafka_resp_err_t_686298611 = -162;
pub const RD_KAFKA_RESP_ERR__INTR: __anonenum_rd_kafka_resp_err_t_686298611 = -163;
pub const RD_KAFKA_RESP_ERR__WAIT_CACHE: __anonenum_rd_kafka_resp_err_t_686298611 = -164;
pub const RD_KAFKA_RESP_ERR__UNSUPPORTED_FEATURE: __anonenum_rd_kafka_resp_err_t_686298611 = -165;
pub const RD_KAFKA_RESP_ERR__TIMED_OUT_QUEUE: __anonenum_rd_kafka_resp_err_t_686298611 = -166;
pub const RD_KAFKA_RESP_ERR__OUTDATED: __anonenum_rd_kafka_resp_err_t_686298611 = -167;
pub const RD_KAFKA_RESP_ERR__NO_OFFSET: __anonenum_rd_kafka_resp_err_t_686298611 = -168;
pub const RD_KAFKA_RESP_ERR__AUTHENTICATION: __anonenum_rd_kafka_resp_err_t_686298611 = -169;
pub const RD_KAFKA_RESP_ERR__NOT_IMPLEMENTED: __anonenum_rd_kafka_resp_err_t_686298611 = -170;
pub const RD_KAFKA_RESP_ERR__UNKNOWN_PROTOCOL: __anonenum_rd_kafka_resp_err_t_686298611 = -171;
pub const RD_KAFKA_RESP_ERR__STATE: __anonenum_rd_kafka_resp_err_t_686298611 = -172;
pub const RD_KAFKA_RESP_ERR__CONFLICT: __anonenum_rd_kafka_resp_err_t_686298611 = -173;
pub const RD_KAFKA_RESP_ERR__REVOKE_PARTITIONS: __anonenum_rd_kafka_resp_err_t_686298611 = -174;
pub const RD_KAFKA_RESP_ERR__ASSIGN_PARTITIONS: __anonenum_rd_kafka_resp_err_t_686298611 = -175;
pub const RD_KAFKA_RESP_ERR__EXISTING_SUBSCRIPTION: __anonenum_rd_kafka_resp_err_t_686298611 = -176;
pub const RD_KAFKA_RESP_ERR__PREV_IN_PROGRESS: __anonenum_rd_kafka_resp_err_t_686298611 = -177;
pub const RD_KAFKA_RESP_ERR__IN_PROGRESS: __anonenum_rd_kafka_resp_err_t_686298611 = -178;
pub const RD_KAFKA_RESP_ERR__UNKNOWN_GROUP: __anonenum_rd_kafka_resp_err_t_686298611 = -179;
pub const RD_KAFKA_RESP_ERR__WAIT_COORD: __anonenum_rd_kafka_resp_err_t_686298611 = -180;
pub const RD_KAFKA_RESP_ERR__SSL: __anonenum_rd_kafka_resp_err_t_686298611 = -181;
pub const RD_KAFKA_RESP_ERR__NODE_UPDATE: __anonenum_rd_kafka_resp_err_t_686298611 = -182;
pub const RD_KAFKA_RESP_ERR__ISR_INSUFF: __anonenum_rd_kafka_resp_err_t_686298611 = -183;
pub const RD_KAFKA_RESP_ERR__QUEUE_FULL: __anonenum_rd_kafka_resp_err_t_686298611 = -184;
pub const RD_KAFKA_RESP_ERR__TIMED_OUT: __anonenum_rd_kafka_resp_err_t_686298611 = -185;
pub const RD_KAFKA_RESP_ERR__INVALID_ARG: __anonenum_rd_kafka_resp_err_t_686298611 = -186;
pub const RD_KAFKA_RESP_ERR__ALL_BROKERS_DOWN: __anonenum_rd_kafka_resp_err_t_686298611 = -187;
pub const RD_KAFKA_RESP_ERR__UNKNOWN_TOPIC: __anonenum_rd_kafka_resp_err_t_686298611 = -188;
pub const RD_KAFKA_RESP_ERR__FS: __anonenum_rd_kafka_resp_err_t_686298611 = -189;
pub const RD_KAFKA_RESP_ERR__UNKNOWN_PARTITION: __anonenum_rd_kafka_resp_err_t_686298611 = -190;
pub const RD_KAFKA_RESP_ERR__PARTITION_EOF: __anonenum_rd_kafka_resp_err_t_686298611 = -191;
pub const RD_KAFKA_RESP_ERR__MSG_TIMED_OUT: __anonenum_rd_kafka_resp_err_t_686298611 = -192;
pub const RD_KAFKA_RESP_ERR__RESOLVE: __anonenum_rd_kafka_resp_err_t_686298611 = -193;
pub const RD_KAFKA_RESP_ERR__CRIT_SYS_RESOURCE: __anonenum_rd_kafka_resp_err_t_686298611 = -194;
pub const RD_KAFKA_RESP_ERR__TRANSPORT: __anonenum_rd_kafka_resp_err_t_686298611 = -195;
pub const RD_KAFKA_RESP_ERR__FAIL: __anonenum_rd_kafka_resp_err_t_686298611 = -196;
pub const RD_KAFKA_RESP_ERR__DESTROY: __anonenum_rd_kafka_resp_err_t_686298611 = -197;
pub const RD_KAFKA_RESP_ERR__BAD_COMPRESSION: __anonenum_rd_kafka_resp_err_t_686298611 = -198;
pub const RD_KAFKA_RESP_ERR__BAD_MSG: __anonenum_rd_kafka_resp_err_t_686298611 = -199;
pub const RD_KAFKA_RESP_ERR__BEGIN: __anonenum_rd_kafka_resp_err_t_686298611 = -200;
pub type rd_kafka_resp_err_t = __anonenum_rd_kafka_resp_err_t_686298611;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rd_kafka_message_s {
    pub err: rd_kafka_resp_err_t,
    pub rkt: *mut rd_kafka_topic_t,
    pub partition: int32_t,
    pub payload: *mut libc::c_void,
    pub len: size_t,
    pub key: *mut libc::c_void,
    pub key_len: size_t,
    pub offset: int64_t,
    pub _private: *mut libc::c_void,
}
pub type rd_kafka_message_t = rd_kafka_message_s;
pub type __anonenum_rd_kafka_conf_res_t_437023812 = libc::c_int;
pub const RD_KAFKA_CONF_OK: __anonenum_rd_kafka_conf_res_t_437023812 = 0;
pub const RD_KAFKA_CONF_INVALID: __anonenum_rd_kafka_conf_res_t_437023812 = -1;
pub const RD_KAFKA_CONF_UNKNOWN: __anonenum_rd_kafka_conf_res_t_437023812 = -2;
pub type rd_kafka_conf_res_t = __anonenum_rd_kafka_conf_res_t_437023812;
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
static mut longopts: [option; 5] = [
    {
        let mut init = option {
            name: b"log\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"config\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'c' as i32,
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
            name: b"no-setproctitle\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'n' as i32,
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
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut server: brubeck_server = brubeck_server {
        name: 0 as *const libc::c_char,
        dump_path: 0 as *const libc::c_char,
        config_name: 0 as *const libc::c_char,
        running: 0,
        active_backends: 0,
        active_samplers: 0,
        set_proctitle: false,
        fd_signal: 0,
        fd_expire: 0,
        fd_update: 0,
        slab: brubeck_slab {
            current: 0 as *mut brubeck_slab_node,
            total_alloc: 0,
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
        },
        metrics: 0 as *mut brubeck_hashtable_t,
        tags: 0 as *mut brubeck_tags_t,
        at_capacity: 0,
        samplers: [0 as *mut brubeck_sampler; 8],
        backends: [0 as *mut brubeck_backend; 8],
        config: 0 as *mut json_t,
        internal_stats: brubeck_internal_stats {
            sample_freq: 0,
            live: __anonstruct_live_669259541 {
                metrics: 0,
                errors: 0,
                unique_keys: 0,
            },
            sample: __anonstruct_live_669259541 {
                metrics: 0,
                errors: 0,
                unique_keys: 0,
            },
        },
    };
    let mut config_file: *const libc::c_char = 0 as *const libc::c_char;
    let mut log_file: *const libc::c_char = 0 as *const libc::c_char;
    let mut opt: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    memset(
        &mut server as *mut brubeck_server as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<brubeck_server>() as libc::c_ulong,
    );
    server.set_proctitle = 1 as libc::c_int != 0;
    config_file = b"config.default.json\0" as *const u8 as *const libc::c_char;
    log_file = 0 as *mut libc::c_void as *const libc::c_char;
    loop {
        opt = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            b":l:c:vn\0" as *const u8 as *const libc::c_char,
            longopts.as_mut_ptr() as *const option,
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        if !(opt != -(1 as libc::c_int)) {
            break;
        }
        match opt {
            108 => {
                log_file = optarg as *const libc::c_char;
            }
            99 => {
                config_file = optarg as *const libc::c_char;
            }
            118 => {
                puts(b"brubeck f306c25\0" as *const u8 as *const libc::c_char);
                return 0 as libc::c_int;
            }
            110 => {
                server.set_proctitle = 0 as libc::c_int != 0;
            }
            _ => {
                printf(
                    b"Usage: %s [--log LOG_FILE] [--config CONFIG_FILE] [--version] [--no-setproctitle] \n\0"
                        as *const u8 as *const libc::c_char,
                    *argv.offset(0 as libc::c_int as isize),
                );
                return 1 as libc::c_int;
            }
        }
    }
    if server.set_proctitle {
        initproctitle(argc, argv);
    }
    gh_log_open(log_file);
    brubeck_server_init(&mut server, config_file);
    tmp = brubeck_server_run(&mut server);
    return tmp;
}
#[inline]
unsafe extern "C" fn brubeck_metric_get_state(
    mut metric: *const brubeck_metric,
) -> uint8_t {
    let mut tmp: uint8_t = 0;
    tmp = ::std::intrinsics::atomic_load_seqcst(
        &(*metric).private_state as *const uint8_t,
    );
    return tmp;
}
#[inline]
unsafe extern "C" fn brubeck_metric_set_state_if_equal(
    mut metric: *mut brubeck_metric,
    mut expected: uint8_t,
    state: uint8_t,
) -> bool {
    let mut tmp: bool = false;
    let fresh0 = ::std::intrinsics::atomic_cxchg(
        &mut (*metric).private_state,
        *&mut expected,
        state as libc::c_int as uint8_t,
    );
    *&mut expected = fresh0.0;
    tmp = fresh0.1;
    return tmp;
}
pub unsafe extern "C" fn brubeck_backend_register_metric(
    mut self_0: *mut brubeck_backend,
    mut metric: *mut brubeck_metric,
) {
    let mut next: *mut brubeck_metric = 0 as *mut brubeck_metric;
    let mut tmp: bool = false;
    loop {
        next = (*self_0).queue;
        (*metric).next = next;
        tmp = (::std::intrinsics::atomic_cxchg(&mut (*self_0).queue, next, metric)).1;
        if tmp {
            break;
        }
    };
}
unsafe extern "C" fn backend__thread(mut _ptr: *mut libc::c_void) -> *mut libc::c_void {
    let mut self_0: *mut brubeck_backend = 0 as *mut brubeck_backend;
    let mut now: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut then: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut mt: *mut brubeck_metric = 0 as *mut brubeck_metric;
    let mut state: uint8_t = 0;
    let mut tmp: uint8_t = 0;
    let mut tmp___0: libc::c_int = 0;
    self_0 = _ptr as *mut brubeck_backend;
    loop {
        clock_gettime(1 as libc::c_int, &mut then);
        then.tv_sec += (*self_0).sample_freq as __time_t;
        tmp___0 = (Some(((*self_0).connect).expect("non-null function pointer")))
            .expect("non-null function pointer")(self_0 as *mut libc::c_void);
        if tmp___0 == 0 {
            clock_gettime(0 as libc::c_int, &mut now);
            (*self_0).tick_time = now.tv_sec as uint32_t;
            mt = (*self_0).queue;
            while !mt.is_null() {
                tmp = brubeck_metric_get_state(mt as *const brubeck_metric);
                state = tmp;
                if state as libc::c_int == 2 as libc::c_int {
                    brubeck_metric_sample(
                        mt,
                        (*self_0).sample,
                        self_0 as *mut libc::c_void,
                    );
                    brubeck_metric_set_state_if_equal(
                        mt,
                        state,
                        1 as libc::c_int as uint8_t,
                    );
                } else if state as libc::c_int == 1 as libc::c_int {
                    brubeck_metric_sample(
                        mt,
                        (*self_0).sample,
                        self_0 as *mut libc::c_void,
                    );
                    brubeck_metric_set_state_if_equal(
                        mt,
                        state,
                        0 as libc::c_int as uint8_t,
                    );
                }
                mt = (*mt).next;
            }
            if ((*self_0).flush).is_some() {
                (Some(((*self_0).flush).expect("non-null function pointer")))
                    .expect("non-null function pointer")(self_0 as *mut libc::c_void);
            }
        }
        clock_nanosleep(
            1 as libc::c_int,
            1 as libc::c_int,
            &mut then as *mut timespec as *const timespec,
            0 as *mut libc::c_void as *mut timespec,
        );
    };
}
pub unsafe extern "C" fn brubeck_backend_run_threaded(mut self_0: *mut brubeck_backend) {
    let mut tmp: libc::c_int = 0;
    tmp = pthread_create(
        &mut (*self_0).thread as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(
            backend__thread
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        self_0 as *mut libc::c_void,
    );
    if tmp != 0 as libc::c_int {
        fprintf(
            stderr,
            b"[FATAL]: failed to start backend thread\n\0" as *const u8
                as *const libc::c_char,
        );
        gh_log_die();
    }
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 4278190080 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 16711680 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 65280 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 255 as libc::c_uint) << 24 as libc::c_int;
}
#[inline]
unsafe extern "C" fn xcalloc(mut n: size_t, mut size: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_long = 0;
    tmp = calloc(n, size);
    ptr = tmp;
    tmp___0 = (ptr as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong)
        as libc::c_int as libc::c_long;
    if tmp___0 != 0 {
        fprintf(stderr, b"[FATAL]: oom\n\0" as *const u8 as *const libc::c_char);
        gh_log_die();
    }
    return ptr;
}
#[inline]
unsafe extern "C" fn xwrite(
    mut fd: libc::c_int,
    mut buf: *const libc::c_void,
    mut len: size_t,
) -> ssize_t {
    let mut nr: ssize_t = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    loop {
        nr = write(fd, buf, len);
        if nr < 0 as libc::c_long {
            tmp = __errno_location();
            if *tmp == 11 as libc::c_int {
                continue;
            }
            tmp___0 = __errno_location();
            if *tmp___0 == 4 as libc::c_int {
                continue;
            }
        }
        return nr;
    };
}
#[inline]
unsafe extern "C" fn write_in_full(
    mut fd: libc::c_int,
    mut buf: *const libc::c_void,
    mut count: size_t,
) -> ssize_t {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut total: ssize_t = 0;
    let mut written: ssize_t = 0;
    let mut tmp: ssize_t = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    p = buf as *const libc::c_char;
    total = 0 as libc::c_int as ssize_t;
    while count > 0 as libc::c_ulong {
        tmp = xwrite(fd, p as *const libc::c_void, count);
        written = tmp;
        if written < 0 as libc::c_long {
            return -(1 as libc::c_int) as ssize_t;
        }
        if written == 0 {
            tmp___0 = __errno_location();
            *tmp___0 = 28 as libc::c_int;
            return -(1 as libc::c_int) as ssize_t;
        }
        count = (count as libc::c_ulong).wrapping_sub(written as size_t) as size_t
            as size_t;
        p = p.offset(written as isize);
        total += written;
    }
    return total;
}
unsafe extern "C" fn carbon_is_connected(mut backend: *mut libc::c_void) -> bool {
    let mut self_0: *mut brubeck_carbon = 0 as *mut brubeck_carbon;
    self_0 = backend as *mut brubeck_carbon;
    return (*self_0).out_sock >= 0 as libc::c_int;
}
unsafe extern "C" fn carbon_connect(mut backend: *mut libc::c_void) -> libc::c_int {
    let mut self_0: *mut brubeck_carbon = 0 as *mut brubeck_carbon;
    let mut tmp: bool = false;
    let mut rc: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___5: *const libc::c_char = 0 as *const libc::c_char;
    self_0 = backend as *mut brubeck_carbon;
    tmp = carbon_is_connected(self_0 as *mut libc::c_void);
    if tmp {
        return 0 as libc::c_int;
    }
    (*self_0).out_sock = socket(2 as libc::c_int, 1 as libc::c_int, 6 as libc::c_int);
    if (*self_0).out_sock >= 0 as libc::c_int {
        tmp___0 = connect(
            (*self_0).out_sock,
            &mut (*self_0).out_sockaddr as *mut sockaddr_in as *mut sockaddr
                as *const sockaddr,
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
        );
        rc = tmp___0;
        if rc == 0 as libc::c_int {
            tmp___1 = gh_log_instance();
            gh_log_write(
                b"instance=%s backend=carbon event=connected\n\0" as *const u8
                    as *const libc::c_char,
                tmp___1,
            );
            sock_enlarge_out((*self_0).out_sock);
            return 0 as libc::c_int;
        }
        close((*self_0).out_sock);
        (*self_0).out_sock = -(1 as libc::c_int);
    }
    tmp___2 = __errno_location();
    tmp___3 = strerror(*tmp___2);
    tmp___4 = __errno_location();
    tmp___5 = gh_log_instance();
    gh_log_write(
        b"instance=%s backend=carbon event=failed_to_connect errno=%d msg=\"%s\"\n\0"
            as *const u8 as *const libc::c_char,
        tmp___5,
        *tmp___4,
        tmp___3,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn carbon_disconnect(mut self_0: *mut brubeck_carbon) {
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *const libc::c_char = 0 as *const libc::c_char;
    tmp = __errno_location();
    tmp___0 = strerror(*tmp);
    tmp___1 = __errno_location();
    tmp___2 = gh_log_instance();
    gh_log_write(
        b"instance=%s backend=carbon event=disconnected errno=%d msg=\"%s\"\n\0"
            as *const u8 as *const libc::c_char,
        tmp___2,
        *tmp___1,
        tmp___0,
    );
    close((*self_0).out_sock);
    (*self_0).out_sock = -(1 as libc::c_int);
}
unsafe extern "C" fn plaintext_each(
    mut metric: *const brubeck_metric,
    mut key: *const libc::c_char,
    mut value: value_t,
    mut backend: *mut libc::c_void,
) {
    let mut carbon: *mut brubeck_carbon = 0 as *mut brubeck_carbon;
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key_len: size_t = 0;
    let mut tmp: size_t = 0;
    let mut wr: ssize_t = 0;
    let mut tmp___0: bool = false;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    carbon = backend as *mut brubeck_carbon;
    ptr = buffer.as_mut_ptr();
    tmp = strlen(key);
    key_len = tmp;
    tmp___0 = carbon_is_connected(carbon as *mut libc::c_void);
    if !tmp___0 {
        return;
    }
    tmp___1 = strchr(key, ' ' as i32);
    if tmp___1 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    memcpy(ptr as *mut libc::c_void, key as *const libc::c_void, key_len);
    ptr = ptr.offset(key_len as isize);
    tmp___2 = ptr;
    ptr = ptr.offset(1);
    *tmp___2 = ' ' as i32 as libc::c_char;
    tmp___3 = brubeck_ftoa(ptr, value as libc::c_float);
    ptr = ptr.offset(tmp___3 as isize);
    tmp___4 = ptr;
    ptr = ptr.offset(1);
    *tmp___4 = ' ' as i32 as libc::c_char;
    tmp___5 = brubeck_itoa(ptr, (*carbon).backend.tick_time as uint64_t);
    ptr = ptr.offset(tmp___5 as isize);
    tmp___6 = ptr;
    ptr = ptr.offset(1);
    *tmp___6 = '\n' as i32 as libc::c_char;
    wr = write_in_full(
        (*carbon).out_sock,
        buffer.as_mut_ptr() as *const libc::c_void,
        ptr.offset_from(buffer.as_mut_ptr()) as libc::c_long as size_t,
    );
    if wr < 0 as libc::c_long {
        carbon_disconnect(carbon);
        return;
    }
    (*carbon)
        .bytes_sent = ((*carbon).bytes_sent as libc::c_ulong).wrapping_add(wr as size_t)
        as size_t as size_t;
}
#[inline]
unsafe extern "C" fn pickle1_int32(
    mut ptr: *mut libc::c_char,
    mut _src: *mut libc::c_void,
) -> size_t {
    *ptr = 'J' as i32 as libc::c_char;
    memcpy(
        ptr.offset(1 as libc::c_int as isize) as *mut libc::c_void,
        _src as *const libc::c_void,
        4 as libc::c_int as size_t,
    );
    return 5 as libc::c_int as size_t;
}
#[inline]
unsafe extern "C" fn pickle1_double(
    mut ptr: *mut libc::c_char,
    mut _src: *mut libc::c_void,
) -> size_t {
    let mut source: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    source = _src as *mut uint8_t;
    tmp = ptr;
    ptr = ptr.offset(1);
    *tmp = 'G' as i32 as libc::c_char;
    *ptr
        .offset(
            0 as libc::c_int as isize,
        ) = *source.offset(7 as libc::c_int as isize) as libc::c_char;
    *ptr
        .offset(
            1 as libc::c_int as isize,
        ) = *source.offset(6 as libc::c_int as isize) as libc::c_char;
    *ptr
        .offset(
            2 as libc::c_int as isize,
        ) = *source.offset(5 as libc::c_int as isize) as libc::c_char;
    *ptr
        .offset(
            3 as libc::c_int as isize,
        ) = *source.offset(4 as libc::c_int as isize) as libc::c_char;
    *ptr
        .offset(
            4 as libc::c_int as isize,
        ) = *source.offset(3 as libc::c_int as isize) as libc::c_char;
    *ptr
        .offset(
            5 as libc::c_int as isize,
        ) = *source.offset(2 as libc::c_int as isize) as libc::c_char;
    *ptr
        .offset(
            6 as libc::c_int as isize,
        ) = *source.offset(1 as libc::c_int as isize) as libc::c_char;
    *ptr
        .offset(
            7 as libc::c_int as isize,
        ) = *source.offset(0 as libc::c_int as isize) as libc::c_char;
    return 9 as libc::c_int as size_t;
}
unsafe extern "C" fn pickle1_push(
    mut buf: *mut pickler,
    mut key: *const libc::c_char,
    mut key_len: uint8_t,
    mut timestamp: uint32_t,
    mut value: value_t,
) {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: uint16_t = 0;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: size_t = 0;
    let mut tmp___7: size_t = 0;
    let mut tmp___8: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___9: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___10: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___11: uint16_t = 0;
    let mut tmp___12: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___13: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___14: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___15: uint16_t = 0;
    ptr = ((*buf).ptr).offset((*buf).pos as libc::c_int as isize);
    tmp = ptr;
    ptr = ptr.offset(1);
    *tmp = '(' as i32 as libc::c_char;
    tmp___0 = ptr;
    ptr = ptr.offset(1);
    *tmp___0 = 'U' as i32 as libc::c_char;
    tmp___1 = ptr;
    ptr = ptr.offset(1);
    *tmp___1 = key_len as libc::c_char;
    memcpy(ptr as *mut libc::c_void, key as *const libc::c_void, key_len as size_t);
    ptr = ptr.offset(key_len as libc::c_int as isize);
    tmp___2 = ptr;
    ptr = ptr.offset(1);
    *tmp___2 = 'q' as i32 as libc::c_char;
    tmp___3 = ptr;
    ptr = ptr.offset(1);
    tmp___4 = (*buf).pt;
    (*buf).pt = ((*buf).pt as libc::c_int + 1 as libc::c_int) as uint16_t;
    *tmp___3 = tmp___4 as libc::c_char;
    tmp___5 = ptr;
    ptr = ptr.offset(1);
    *tmp___5 = '(' as i32 as libc::c_char;
    tmp___6 = pickle1_int32(ptr, &mut timestamp as *mut uint32_t as *mut libc::c_void);
    ptr = ptr.offset(tmp___6 as isize);
    tmp___7 = pickle1_double(ptr, &mut value as *mut value_t as *mut libc::c_void);
    ptr = ptr.offset(tmp___7 as isize);
    tmp___8 = ptr;
    ptr = ptr.offset(1);
    *tmp___8 = 't' as i32 as libc::c_char;
    tmp___9 = ptr;
    ptr = ptr.offset(1);
    *tmp___9 = 'q' as i32 as libc::c_char;
    tmp___10 = ptr;
    ptr = ptr.offset(1);
    tmp___11 = (*buf).pt;
    (*buf).pt = ((*buf).pt as libc::c_int + 1 as libc::c_int) as uint16_t;
    *tmp___10 = tmp___11 as libc::c_char;
    tmp___12 = ptr;
    ptr = ptr.offset(1);
    *tmp___12 = 't' as i32 as libc::c_char;
    tmp___13 = ptr;
    ptr = ptr.offset(1);
    *tmp___13 = 'q' as i32 as libc::c_char;
    tmp___14 = ptr;
    ptr = ptr.offset(1);
    tmp___15 = (*buf).pt;
    (*buf).pt = ((*buf).pt as libc::c_int + 1 as libc::c_int) as uint16_t;
    *tmp___14 = tmp___15 as libc::c_char;
    (*buf).pos = ptr.offset_from((*buf).ptr) as libc::c_long as uint16_t;
}
static mut lead: [uint8_t; 4] = [
    ']' as i32 as uint8_t,
    'q' as i32 as uint8_t,
    0 as libc::c_int as uint8_t,
    '(' as i32 as uint8_t,
];
#[inline]
unsafe extern "C" fn pickle1_init(mut buf: *mut pickler) {
    memcpy(
        ((*buf).ptr).offset(4 as libc::c_int as isize) as *mut libc::c_void,
        lead.as_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong,
    );
    (*buf)
        .pos = (4 as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong)
        as uint16_t;
    (*buf).pt = 1 as libc::c_int as uint16_t;
}
static mut trail: [uint8_t; 2] = ['e' as i32 as uint8_t, '.' as i32 as uint8_t];
unsafe extern "C" fn pickle1_flush(mut backend: *mut libc::c_void) {
    let mut carbon: *mut brubeck_carbon = 0 as *mut brubeck_carbon;
    let mut buf: *mut pickler = 0 as *mut pickler;
    let mut buf_lead: *mut uint32_t = 0 as *mut uint32_t;
    let mut wr: ssize_t = 0;
    let mut tmp: bool = false;
    carbon = backend as *mut brubeck_carbon;
    buf = &mut (*carbon).pickler;
    if (*buf).pt as libc::c_int == 1 as libc::c_int {
        return
    } else {
        tmp = carbon_is_connected(carbon as *mut libc::c_void);
        if !tmp {
            return;
        }
    }
    memcpy(
        ((*buf).ptr).offset((*buf).pos as libc::c_int as isize) as *mut libc::c_void,
        trail.as_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[uint8_t; 2]>() as libc::c_ulong,
    );
    (*buf)
        .pos = ((*buf).pos as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<[uint8_t; 2]>() as libc::c_ulong)
        as uint16_t;
    buf_lead = (*buf).ptr as *mut uint32_t;
    *buf_lead = __bswap_32(((*buf).pos as uint32_t).wrapping_sub(4 as libc::c_uint));
    wr = write_in_full(
        (*carbon).out_sock,
        (*buf).ptr as *const libc::c_void,
        (*buf).pos as size_t,
    );
    pickle1_init(&mut (*carbon).pickler);
    if wr < 0 as libc::c_long {
        carbon_disconnect(carbon);
        return;
    }
    (*carbon)
        .bytes_sent = ((*carbon).bytes_sent as libc::c_ulong).wrapping_add(wr as size_t)
        as size_t as size_t;
}
unsafe extern "C" fn pickle1_each(
    mut metric: *const brubeck_metric,
    mut key: *const libc::c_char,
    mut value: value_t,
    mut backend: *mut libc::c_void,
) {
    let mut carbon: *mut brubeck_carbon = 0 as *mut brubeck_carbon;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key_len: uint8_t = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: bool = false;
    carbon = backend as *mut brubeck_carbon;
    tmp = strchr(key, ' ' as i32);
    if tmp as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    tmp___0 = strlen(key);
    key_len = tmp___0 as uint8_t;
    if (*carbon).pickler.pos as libc::c_int
        + (32 as libc::c_int + key_len as libc::c_int) >= 4096 as libc::c_int
    {
        pickle1_flush(carbon as *mut libc::c_void);
    }
    tmp___1 = carbon_is_connected(carbon as *mut libc::c_void);
    if !tmp___1 {
        return;
    }
    pickle1_push(
        &mut (*carbon).pickler,
        key,
        key_len,
        (*carbon).backend.tick_time,
        value,
    );
}
pub unsafe extern "C" fn brubeck_carbon_new(
    mut server: *mut brubeck_server,
    mut settings: *mut json_t,
    mut shard_n: libc::c_int,
) -> *mut brubeck_backend {
    let mut carbon: *mut brubeck_carbon = 0 as *mut brubeck_carbon;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut address: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut port: libc::c_int = 0;
    let mut frequency: libc::c_int = 0;
    let mut pickle: libc::c_int = 0;
    let mut _error_j: json_error_t = json_error_t {
        line: 0,
        column: 0,
        position: 0,
        source: [0; 80],
        text: [0; 160],
    };
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *const libc::c_char = 0 as *const libc::c_char;
    tmp = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<brubeck_carbon>() as libc::c_ulong,
    );
    carbon = tmp as *mut brubeck_carbon;
    pickle = 0 as libc::c_int;
    tmp___0 = json_unpack_ex(
        settings,
        &mut _error_j as *mut json_error_t,
        0 as libc::c_int as size_t,
        b"{s:s, s:i, s?:b, s:i}\0" as *const u8 as *const libc::c_char,
        b"address\0" as *const u8 as *const libc::c_char,
        &mut address as *mut *mut libc::c_char,
        b"port\0" as *const u8 as *const libc::c_char,
        &mut port as *mut libc::c_int,
        b"pickle\0" as *const u8 as *const libc::c_char,
        &mut pickle as *mut libc::c_int,
        b"frequency\0" as *const u8 as *const libc::c_char,
        &mut frequency as *mut libc::c_int,
    );
    if tmp___0 < 0 as libc::c_int {
        fprintf(
            stderr,
            b"[FATAL]: config error: %s\n\0" as *const u8 as *const libc::c_char,
            (_error_j.text).as_mut_ptr(),
        );
        gh_log_die();
    }
    (*carbon).backend.type_0 = BRUBECK_BACKEND_CARBON;
    (*carbon).backend.shard_n = shard_n;
    (*carbon)
        .backend
        .connect = Some(
        carbon_connect as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
    );
    (*carbon)
        .backend
        .is_connected = Some(
        carbon_is_connected as unsafe extern "C" fn(*mut libc::c_void) -> bool,
    );
    if pickle != 0 {
        (*carbon)
            .backend
            .sample = Some(
            pickle1_each
                as unsafe extern "C" fn(
                    *const brubeck_metric,
                    *const libc::c_char,
                    value_t,
                    *mut libc::c_void,
                ) -> (),
        );
        (*carbon)
            .backend
            .flush = Some(
            pickle1_flush as unsafe extern "C" fn(*mut libc::c_void) -> (),
        );
        tmp___1 = malloc(4096 as libc::c_int as size_t);
        (*carbon).pickler.ptr = tmp___1 as *mut libc::c_char;
        pickle1_init(&mut (*carbon).pickler);
    } else {
        (*carbon)
            .backend
            .sample = Some(
            plaintext_each
                as unsafe extern "C" fn(
                    *const brubeck_metric,
                    *const libc::c_char,
                    value_t,
                    *mut libc::c_void,
                ) -> (),
        );
        (*carbon)
            .backend
            .flush = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(0 as *mut libc::c_void);
    }
    (*carbon).backend.sample_freq = frequency;
    (*carbon).backend.server = server;
    (*carbon).out_sock = -(1 as libc::c_int);
    url_to_inaddr2(&mut (*carbon).out_sockaddr, address as *const libc::c_char, port);
    brubeck_backend_run_threaded(carbon as *mut brubeck_backend);
    tmp___2 = gh_log_instance();
    gh_log_write(
        b"instance=%s backend=carbon event=started\n\0" as *const u8
            as *const libc::c_char,
        tmp___2,
    );
    return carbon as *mut brubeck_backend;
}
#[inline]
unsafe extern "C" fn xmalloc(mut size: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_long = 0;
    tmp = malloc(size);
    ptr = tmp;
    tmp___0 = (ptr as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong)
        as libc::c_int as libc::c_long;
    if tmp___0 != 0 {
        fprintf(stderr, b"[FATAL]: oom\n\0" as *const u8 as *const libc::c_char);
        gh_log_die();
    }
    return ptr;
}
pub unsafe extern "C" fn multibloom_check(
    mut bloom: *mut multibloom,
    mut f: libc::c_int,
    mut a: uint32_t,
    mut b: uint32_t,
) -> libc::c_int {
    let mut filter: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut hits: libc::c_int = 0;
    let mut x: uint32_t = 0;
    let mut i: uint32_t = 0;
    let mut byte: uint32_t = 0;
    let mut mask: uint32_t = 0;
    let mut c: libc::c_uchar = 0;
    filter = *((*bloom).filters).as_mut_ptr().offset(f as isize);
    hits = 0 as libc::c_int;
    i = 0 as libc::c_int as uint32_t;
    while i < (*bloom).hashes as uint32_t {
        x = a
            .wrapping_add(i.wrapping_mul(b))
            .wrapping_rem((*bloom).bits as libc::c_uint);
        byte = x >> 3 as libc::c_int;
        c = *filter.offset(byte as isize);
        mask = ((1 as libc::c_int) << x.wrapping_rem(8 as libc::c_uint)) as uint32_t;
        if c as libc::c_uint & mask != 0 {
            hits += 1;
        } else {
            *filter.offset(byte as isize) = (c as libc::c_uint | mask) as libc::c_uchar;
        }
        i = i.wrapping_add(1);
    }
    return (hits == (*bloom).hashes) as libc::c_int;
}
pub unsafe extern "C" fn multibloom_reset(
    mut bloom: *mut multibloom,
    mut f: libc::c_int,
) {
    memset(
        *((*bloom).filters).as_mut_ptr().offset(f as isize) as *mut libc::c_void,
        0 as libc::c_int,
        (*bloom).bytes as size_t,
    );
}
pub unsafe extern "C" fn multibloom_new(
    mut filters: libc::c_int,
    mut entries: libc::c_int,
    mut error: libc::c_double,
) -> *mut multibloom {
    let mut bpe: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut bloom: *mut multibloom = 0 as *mut multibloom;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: libc::c_double = 0.;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: *const libc::c_char = 0 as *const libc::c_char;
    tmp = log(error);
    bpe = -(tmp / 0.480453013918201f64);
    tmp___0 = xmalloc(
        (::std::mem::size_of::<multibloom>() as libc::c_ulong)
            .wrapping_add(
                (filters as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                    ),
            ),
    );
    bloom = tmp___0 as *mut multibloom;
    (*bloom).bits = (entries as libc::c_double * bpe) as libc::c_int;
    (*bloom).bytes = (*bloom).bits / 8 as libc::c_int;
    if (*bloom).bits % 8 as libc::c_int != 0 {
        (*bloom).bytes += 1;
    }
    tmp___1 = ceil(0.693147180559945f64 * bpe);
    (*bloom).hashes = tmp___1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < filters {
        tmp___2 = xcalloc(1 as libc::c_int as size_t, (*bloom).bytes as size_t);
        let ref mut fresh1 = *((*bloom).filters).as_mut_ptr().offset(i as isize);
        *fresh1 = tmp___2 as *mut libc::c_uchar;
        i += 1;
    }
    tmp___3 = gh_log_instance();
    gh_log_write(
        b"instance=%s event=bloom_init entries=%d error=%f bits=%d bpe=%f bytes=%d hash_funcs=%d\n\0"
            as *const u8 as *const libc::c_char,
        tmp___3,
        entries,
        error,
        (*bloom).bits,
        bpe,
        (*bloom).bytes,
        (*bloom).hashes,
    );
    return bloom;
}
unsafe extern "C" fn read32(mut p: *const libc::c_char) -> uint32_t {
    let mut result: uint32_t = 0;
    memcpy(
        &mut result as *mut uint32_t as *mut libc::c_void,
        p as *const libc::c_void,
        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
    );
    return result;
}
static mut c1: libc::c_uint = 3432918353 as libc::c_uint;
static mut c2: libc::c_uint = 461845907 as libc::c_int as uint32_t;
unsafe extern "C" fn fmix(mut h: uint32_t) -> uint32_t {
    h ^= h >> 16 as libc::c_int;
    h = (h as libc::c_uint).wrapping_mul(2246822507 as libc::c_uint) as uint32_t
        as uint32_t;
    h ^= h >> 13 as libc::c_int;
    h = (h as libc::c_uint).wrapping_mul(3266489909 as libc::c_uint) as uint32_t
        as uint32_t;
    h ^= h >> 16 as libc::c_int;
    return h;
}
unsafe extern "C" fn ror32(mut val: uint32_t, mut shift: libc::c_int) -> uint32_t {
    let mut tmp: uint32_t = 0;
    if shift == 0 as libc::c_int {
        tmp = val;
    } else {
        tmp = val >> shift | val << 32 as libc::c_int - shift;
    }
    return tmp;
}
unsafe extern "C" fn mur(mut a: uint32_t, mut h: uint32_t) -> uint32_t {
    a = (a as libc::c_uint).wrapping_mul(c1) as uint32_t as uint32_t;
    a = ror32(a, 17 as libc::c_int);
    a = (a as libc::c_uint).wrapping_mul(c2) as uint32_t as uint32_t;
    h ^= a;
    h = ror32(h, 19 as libc::c_int);
    return h.wrapping_mul(5 as libc::c_uint).wrapping_add(3864292196 as libc::c_uint);
}
unsafe extern "C" fn Hash32Len13to24(
    mut s: *const libc::c_char,
    mut len: size_t,
) -> uint32_t {
    let mut a: uint32_t = 0;
    let mut tmp: uint32_t = 0;
    let mut b: uint32_t = 0;
    let mut tmp___0: uint32_t = 0;
    let mut c: uint32_t = 0;
    let mut tmp___1: uint32_t = 0;
    let mut d: uint32_t = 0;
    let mut tmp___2: uint32_t = 0;
    let mut e: uint32_t = 0;
    let mut tmp___3: uint32_t = 0;
    let mut f: uint32_t = 0;
    let mut tmp___4: uint32_t = 0;
    let mut h: uint32_t = 0;
    let mut tmp___5: uint32_t = 0;
    let mut tmp___6: uint32_t = 0;
    let mut tmp___7: uint32_t = 0;
    let mut tmp___8: uint32_t = 0;
    let mut tmp___9: uint32_t = 0;
    let mut tmp___10: uint32_t = 0;
    let mut tmp___11: uint32_t = 0;
    tmp = read32(
        s.offset(-(4 as libc::c_int as isize)).offset((len >> 1 as libc::c_int) as isize),
    );
    a = tmp;
    tmp___0 = read32(s.offset(4 as libc::c_int as isize));
    b = tmp___0;
    tmp___1 = read32(s.offset(len as isize).offset(-(8 as libc::c_int as isize)));
    c = tmp___1;
    tmp___2 = read32(s.offset((len >> 1 as libc::c_int) as isize));
    d = tmp___2;
    tmp___3 = read32(s);
    e = tmp___3;
    tmp___4 = read32(s.offset(len as isize).offset(-(4 as libc::c_int as isize)));
    f = tmp___4;
    h = len as uint32_t;
    tmp___5 = mur(a, h);
    tmp___6 = mur(b, tmp___5);
    tmp___7 = mur(c, tmp___6);
    tmp___8 = mur(d, tmp___7);
    tmp___9 = mur(e, tmp___8);
    tmp___10 = mur(f, tmp___9);
    tmp___11 = fmix(tmp___10);
    return tmp___11;
}
unsafe extern "C" fn Hash32Len0to4(
    mut s: *const libc::c_char,
    mut len: size_t,
) -> uint32_t {
    let mut b: uint32_t = 0;
    let mut c: uint32_t = 0;
    let mut i: libc::c_int = 0;
    let mut tmp: uint32_t = 0;
    let mut tmp___0: uint32_t = 0;
    let mut tmp___1: uint32_t = 0;
    b = 0 as libc::c_int as uint32_t;
    c = 9 as libc::c_int as uint32_t;
    i = 0 as libc::c_int;
    while (i as size_t) < len {
        b = b.wrapping_mul(c1).wrapping_add(*s.offset(i as isize) as uint32_t);
        c ^= b;
        i += 1;
    }
    tmp = mur(len as uint32_t, c);
    tmp___0 = mur(b, tmp);
    tmp___1 = fmix(tmp___0);
    return tmp___1;
}
unsafe extern "C" fn Hash32Len5to12(
    mut s: *const libc::c_char,
    mut len: size_t,
) -> uint32_t {
    let mut a: uint32_t = 0;
    let mut b: uint32_t = 0;
    let mut c: uint32_t = 0;
    let mut d: uint32_t = 0;
    let mut tmp: uint32_t = 0;
    let mut tmp___0: uint32_t = 0;
    let mut tmp___1: uint32_t = 0;
    let mut tmp___2: uint32_t = 0;
    let mut tmp___3: uint32_t = 0;
    let mut tmp___4: uint32_t = 0;
    let mut tmp___5: uint32_t = 0;
    a = len as uint32_t;
    b = len.wrapping_mul(5 as libc::c_ulong) as uint32_t;
    c = 9 as libc::c_int as uint32_t;
    d = b;
    tmp = read32(s);
    a = (a as libc::c_uint).wrapping_add(tmp) as uint32_t as uint32_t;
    tmp___0 = read32(s.offset(len as isize).offset(-(4 as libc::c_int as isize)));
    b = (b as libc::c_uint).wrapping_add(tmp___0) as uint32_t as uint32_t;
    tmp___1 = read32(s.offset((len >> 1 as libc::c_int & 4 as libc::c_ulong) as isize));
    c = (c as libc::c_uint).wrapping_add(tmp___1) as uint32_t as uint32_t;
    tmp___2 = mur(a, d);
    tmp___3 = mur(b, tmp___2);
    tmp___4 = mur(c, tmp___3);
    tmp___5 = fmix(tmp___4);
    return tmp___5;
}
pub unsafe extern "C" fn CityHash32(
    mut s: *const libc::c_char,
    mut len: size_t,
) -> uint32_t {
    let mut iters: size_t = 0;
    let mut a0: uint32_t = 0;
    let mut a1: uint32_t = 0;
    let mut a2: uint32_t = 0;
    let mut a3: uint32_t = 0;
    let mut a4: uint32_t = 0;
    let mut h: uint32_t = 0;
    let mut g: uint32_t = 0;
    let mut f: uint32_t = 0;
    let mut tmp: uint32_t = 0;
    let mut tmp___0: uint32_t = 0;
    let mut tmp___1: uint32_t = 0;
    let mut tmp___2: uint32_t = 0;
    let mut tmp___3: uint32_t = 0;
    let mut tmp___4: uint32_t = 0;
    let mut tmp___5: uint32_t = 0;
    let mut tmp___6: uint32_t = 0;
    let mut tmp___7: uint32_t = 0;
    let mut tmp___8: uint32_t = 0;
    let mut tmp___9: uint32_t = 0;
    let mut tmp___10: uint32_t = 0;
    let mut tmp___11: uint32_t = 0;
    let mut tmp___12: uint32_t = 0;
    let mut tmp___13: uint32_t = 0;
    let mut a0___0: uint32_t = 0;
    let mut tmp___14: uint32_t = 0;
    let mut tmp___15: uint32_t = 0;
    let mut a1___0: uint32_t = 0;
    let mut tmp___16: uint32_t = 0;
    let mut a2___0: uint32_t = 0;
    let mut tmp___17: uint32_t = 0;
    let mut tmp___18: uint32_t = 0;
    let mut a3___0: uint32_t = 0;
    let mut tmp___19: uint32_t = 0;
    let mut tmp___20: uint32_t = 0;
    let mut a4___0: uint32_t = 0;
    let mut tmp___21: uint32_t = 0;
    let mut tmp___22: __uint32_t = 0;
    let mut aux: uint32_t = 0;
    let mut aux___0: uint32_t = 0;
    let mut tmp___23: uint32_t = 0;
    let mut tmp___24: uint32_t = 0;
    let mut tmp___25: uint32_t = 0;
    let mut tmp___26: uint32_t = 0;
    let mut tmp___27: uint32_t = 0;
    let mut tmp___28: uint32_t = 0;
    if len <= 24 as libc::c_ulong {
        if len <= 12 as libc::c_ulong {
            if len <= 4 as libc::c_ulong {
                tmp = Hash32Len0to4(s, len);
                tmp___1 = tmp;
            } else {
                tmp___0 = Hash32Len5to12(s, len);
                tmp___1 = tmp___0;
            }
            tmp___3 = tmp___1;
        } else {
            tmp___2 = Hash32Len13to24(s, len);
            tmp___3 = tmp___2;
        }
        return tmp___3;
    }
    h = len as uint32_t;
    g = (c1 as size_t).wrapping_mul(len) as uint32_t;
    f = g;
    tmp___4 = read32(s.offset(len as isize).offset(-(4 as libc::c_int as isize)));
    tmp___5 = ror32(tmp___4.wrapping_mul(c1), 17 as libc::c_int);
    a0 = tmp___5.wrapping_mul(c2);
    tmp___6 = read32(s.offset(len as isize).offset(-(8 as libc::c_int as isize)));
    tmp___7 = ror32(tmp___6.wrapping_mul(c1), 17 as libc::c_int);
    a1 = tmp___7.wrapping_mul(c2);
    tmp___8 = read32(s.offset(len as isize).offset(-(16 as libc::c_int as isize)));
    tmp___9 = ror32(tmp___8.wrapping_mul(c1), 17 as libc::c_int);
    a2 = tmp___9.wrapping_mul(c2);
    tmp___10 = read32(s.offset(len as isize).offset(-(12 as libc::c_int as isize)));
    tmp___11 = ror32(tmp___10.wrapping_mul(c1), 17 as libc::c_int);
    a3 = tmp___11.wrapping_mul(c2);
    tmp___12 = read32(s.offset(len as isize).offset(-(20 as libc::c_int as isize)));
    tmp___13 = ror32(tmp___12.wrapping_mul(c1), 17 as libc::c_int);
    a4 = tmp___13.wrapping_mul(c2);
    h ^= a0;
    h = ror32(h, 19 as libc::c_int);
    h = h.wrapping_mul(5 as libc::c_uint).wrapping_add(3864292196 as libc::c_uint);
    h ^= a2;
    h = ror32(h, 19 as libc::c_int);
    h = h.wrapping_mul(5 as libc::c_uint).wrapping_add(3864292196 as libc::c_uint);
    g ^= a1;
    g = ror32(g, 19 as libc::c_int);
    g = g.wrapping_mul(5 as libc::c_uint).wrapping_add(3864292196 as libc::c_uint);
    g ^= a3;
    g = ror32(g, 19 as libc::c_int);
    g = g.wrapping_mul(5 as libc::c_uint).wrapping_add(3864292196 as libc::c_uint);
    f = (f as libc::c_uint).wrapping_add(a4) as uint32_t as uint32_t;
    f = ror32(f, 19 as libc::c_int);
    f = f.wrapping_mul(5 as libc::c_uint).wrapping_add(3864292196 as libc::c_uint);
    iters = len.wrapping_sub(1 as libc::c_ulong).wrapping_div(20 as libc::c_ulong);
    loop {
        tmp___14 = read32(s);
        tmp___15 = ror32(tmp___14.wrapping_mul(c1), 17 as libc::c_int);
        a0___0 = tmp___15.wrapping_mul(c2);
        tmp___16 = read32(s.offset(4 as libc::c_int as isize));
        a1___0 = tmp___16;
        tmp___17 = read32(s.offset(8 as libc::c_int as isize));
        tmp___18 = ror32(tmp___17.wrapping_mul(c1), 17 as libc::c_int);
        a2___0 = tmp___18.wrapping_mul(c2);
        tmp___19 = read32(s.offset(12 as libc::c_int as isize));
        tmp___20 = ror32(tmp___19.wrapping_mul(c1), 17 as libc::c_int);
        a3___0 = tmp___20.wrapping_mul(c2);
        tmp___21 = read32(s.offset(16 as libc::c_int as isize));
        a4___0 = tmp___21;
        h ^= a0___0;
        h = ror32(h, 18 as libc::c_int);
        h = h.wrapping_mul(5 as libc::c_uint).wrapping_add(3864292196 as libc::c_uint);
        f = (f as libc::c_uint).wrapping_add(a1___0) as uint32_t as uint32_t;
        f = ror32(f, 19 as libc::c_int);
        f = (f as libc::c_uint).wrapping_mul(c1) as uint32_t as uint32_t;
        g = (g as libc::c_uint).wrapping_add(a2___0) as uint32_t as uint32_t;
        g = ror32(g, 18 as libc::c_int);
        g = g.wrapping_mul(5 as libc::c_uint).wrapping_add(3864292196 as libc::c_uint);
        h ^= a3___0.wrapping_add(a1___0);
        h = ror32(h, 19 as libc::c_int);
        h = h.wrapping_mul(5 as libc::c_uint).wrapping_add(3864292196 as libc::c_uint);
        g ^= a4___0;
        tmp___22 = __bswap_32(g);
        g = tmp___22.wrapping_mul(5 as libc::c_uint);
        h = (h as libc::c_uint).wrapping_add(a4___0.wrapping_mul(5 as libc::c_uint))
            as uint32_t as uint32_t;
        h = __bswap_32(h);
        f = (f as libc::c_uint).wrapping_add(a0___0) as uint32_t as uint32_t;
        aux = f;
        f = h;
        h = aux;
        aux___0 = f;
        f = g;
        g = aux___0;
        s = s.offset(20 as libc::c_int as isize);
        iters = iters.wrapping_sub(1);
        if !(iters != 0 as libc::c_ulong) {
            break;
        }
    }
    tmp___23 = ror32(g, 11 as libc::c_int);
    g = tmp___23.wrapping_mul(c1);
    tmp___24 = ror32(g, 17 as libc::c_int);
    g = tmp___24.wrapping_mul(c1);
    tmp___25 = ror32(f, 11 as libc::c_int);
    f = tmp___25.wrapping_mul(c1);
    tmp___26 = ror32(f, 17 as libc::c_int);
    f = tmp___26.wrapping_mul(c1);
    h = ror32(h.wrapping_add(g), 19 as libc::c_int);
    h = h.wrapping_mul(5 as libc::c_uint).wrapping_add(3864292196 as libc::c_uint);
    tmp___27 = ror32(h, 17 as libc::c_int);
    h = tmp___27.wrapping_mul(c1);
    h = ror32(h.wrapping_add(f), 19 as libc::c_int);
    h = h.wrapping_mul(5 as libc::c_uint).wrapping_add(3864292196 as libc::c_uint);
    tmp___28 = ror32(h, 17 as libc::c_int);
    h = tmp___28.wrapping_mul(c1);
    return h;
}
#[inline]
unsafe extern "C" fn xrealloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut new_ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_long = 0;
    tmp = realloc(ptr, size);
    new_ptr = tmp;
    tmp___0 = (new_ptr as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong)
        as libc::c_int as libc::c_long;
    if tmp___0 != 0 {
        fprintf(stderr, b"[FATAL]: oom\n\0" as *const u8 as *const libc::c_char);
        gh_log_die();
    }
    return new_ptr;
}
pub unsafe extern "C" fn brubeck_histo_push(
    mut histo: *mut brubeck_histo,
    mut value: value_t,
    mut sample_freq: value_t,
) {
    let mut new_size: size_t = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: uint16_t = 0;
    (*histo).count = ((*histo).count as value_t + sample_freq) as uint32_t;
    if (*histo).size as libc::c_int == (*histo).alloc as libc::c_int {
        if (*histo).size as libc::c_int == 65535 as libc::c_int {
            return;
        }
        new_size = ((*histo).alloc as libc::c_int * 2 as libc::c_int) as size_t;
        if new_size > 65535 as libc::c_ulong {
            new_size = 65535 as libc::c_int as size_t;
        }
        if new_size < 16 as libc::c_ulong {
            new_size = 16 as libc::c_int as size_t;
        }
        if new_size != (*histo).alloc as size_t {
            (*histo).alloc = new_size as uint16_t;
            tmp = xrealloc(
                (*histo).values as *mut libc::c_void,
                ((*histo).alloc as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<value_t>() as libc::c_ulong),
            );
            (*histo).values = tmp as *mut value_t;
        }
    }
    tmp___0 = (*histo).size;
    (*histo).size = ((*histo).size as libc::c_int + 1 as libc::c_int) as uint16_t;
    *((*histo).values).offset(tmp___0 as libc::c_int as isize) = value;
}
#[inline]
unsafe extern "C" fn histo_percentile(
    mut histo: *mut brubeck_histo,
    mut rank: libc::c_float,
) -> value_t {
    let mut irank: size_t = 0;
    let mut tmp: libc::c_double = 0.;
    tmp = floor((rank * (*histo).size as libc::c_float + 0.5f32) as libc::c_double);
    irank = tmp as size_t;
    return *((*histo).values).offset(irank.wrapping_sub(1 as libc::c_ulong) as isize);
}
#[inline]
unsafe extern "C" fn histo_sum(mut histo: *mut brubeck_histo) -> value_t {
    let mut i: size_t = 0;
    let mut sum: value_t = 0.;
    sum = 0.0f64;
    i = 0 as libc::c_int as size_t;
    while i < (*histo).size as size_t {
        sum += *((*histo).values).offset(i as isize);
        i = i.wrapping_add(1);
    }
    return sum;
}
unsafe extern "C" fn value_cmp(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut va: value_t = 0.;
    let mut vb: value_t = 0.;
    va = *(a as *const value_t);
    vb = *(b as *const value_t);
    if va < vb {
        return -(1 as libc::c_int);
    }
    if va > vb {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn histo_sort(mut histo: *mut brubeck_histo) {
    qsort(
        (*histo).values as *mut libc::c_void,
        (*histo).size as size_t,
        ::std::mem::size_of::<value_t>() as libc::c_ulong,
        Some(
            value_cmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
}
pub unsafe extern "C" fn brubeck_histo_sample(
    mut sample: *mut brubeck_histo_sample,
    mut histo: *mut brubeck_histo,
) {
    if (*histo).size as libc::c_int == 0 as libc::c_int {
        memset(
            sample as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<brubeck_histo_sample>() as libc::c_ulong,
        );
        return;
    }
    histo_sort(histo);
    (*sample).sum = histo_sum(histo);
    (*sample).min = *((*histo).values).offset(0 as libc::c_int as isize);
    (*sample)
        .max = *((*histo).values)
        .offset(((*histo).size as libc::c_int - 1 as libc::c_int) as isize);
    (*sample).mean = (*sample).sum / (*histo).size as value_t;
    (*sample).median = histo_percentile(histo, 0.5f32);
    (*sample).count = (*histo).count as value_t;
    (*sample).percentile[0 as libc::c_int as usize] = histo_percentile(histo, 0.75f32);
    (*sample).percentile[1 as libc::c_int as usize] = histo_percentile(histo, 0.95f32);
    (*sample).percentile[2 as libc::c_int as usize] = histo_percentile(histo, 0.98f32);
    (*sample).percentile[3 as libc::c_int as usize] = histo_percentile(histo, 0.99f32);
    (*sample).percentile[4 as libc::c_int as usize] = histo_percentile(histo, 0.999f32);
    (*histo).size = 0 as libc::c_int as uint16_t;
    (*histo).count = 0 as libc::c_int as uint32_t;
}
#[inline]
unsafe extern "C" fn ck_ht_entry_key_set(
    mut entry: *mut ck_ht_entry_t,
    mut key: *const libc::c_void,
    mut key_length: uint16_t,
) {
    (*entry).key = key as uintptr_t;
    (*entry).key_length = key_length as uint64_t;
}
#[inline]
unsafe extern "C" fn ck_ht_entry_value(
    mut entry: *mut ck_ht_entry_t,
) -> *mut libc::c_void {
    return (*entry).value as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn ck_ht_entry_set(
    mut entry: *mut ck_ht_entry,
    mut h: ck_ht_hash_t,
    mut key: *const libc::c_void,
    mut key_length: uint16_t,
    mut value: *const libc::c_void,
) {
    (*entry).key = key as uintptr_t;
    (*entry).value = value as uintptr_t;
    (*entry).key_length = key_length as uint64_t;
    (*entry).hash = h.value;
}
unsafe extern "C" fn ht_malloc(mut r: size_t) -> *mut libc::c_void {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = xmalloc(r);
    return tmp;
}
unsafe extern "C" fn ht_free(mut p: *mut libc::c_void, mut b: size_t, mut r: bool) {
    free(p);
}
static mut ALLOCATOR: ck_malloc = {
    let mut init = ck_malloc {
        malloc: Some(ht_malloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        realloc: None,
        free: Some(
            ht_free as unsafe extern "C" fn(*mut libc::c_void, size_t, bool) -> (),
        ),
    };
    init
};
pub unsafe extern "C" fn brubeck_hashtable_new(
    size: uint64_t,
) -> *mut brubeck_hashtable_t {
    let mut ht: *mut brubeck_hashtable_t = 0 as *mut brubeck_hashtable_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: bool = false;
    tmp = xmalloc(::std::mem::size_of::<brubeck_hashtable_t>() as libc::c_ulong);
    ht = tmp as *mut brubeck_hashtable_t;
    pthread_mutex_init(
        &mut (*ht).write_mutex,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
    tmp___0 = ck_ht_init(
        &mut (*ht).table,
        2 as libc::c_uint,
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<ck_ht_hash_cb_t>,
        >(0 as *mut libc::c_void),
        &mut ALLOCATOR,
        size,
        3735928559 as libc::c_uint as uint64_t,
    );
    if !tmp___0 {
        free(ht as *mut libc::c_void);
        return 0 as *mut libc::c_void as *mut brubeck_hashtable_t;
    }
    return ht;
}
pub unsafe extern "C" fn brubeck_hashtable_free(mut ht: *mut brubeck_hashtable_t) {}
pub unsafe extern "C" fn brubeck_hashtable_find(
    mut ht: *mut brubeck_hashtable_t,
    mut key: *const libc::c_char,
    mut key_len: uint16_t,
) -> *mut brubeck_metric {
    let mut h: ck_ht_hash_t = ck_ht_hash_t { value: 0 };
    let mut entry: ck_ht_entry_t = ck_ht_entry_t {
        key: 0,
        value: 0,
        key_length: 0,
        hash: 0,
    };
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: bool = false;
    ck_ht_hash(&mut h, &mut (*ht).table, key as *const libc::c_void, key_len);
    ck_ht_entry_key_set(&mut entry, key as *const libc::c_void, key_len);
    tmp___0 = ck_ht_get_spmc(&mut (*ht).table, h, &mut entry);
    if tmp___0 {
        tmp = ck_ht_entry_value(&mut entry);
        return tmp as *mut brubeck_metric;
    }
    return 0 as *mut libc::c_void as *mut brubeck_metric;
}
pub unsafe extern "C" fn brubeck_hashtable_insert(
    mut ht: *mut brubeck_hashtable_t,
    mut key: *const libc::c_char,
    mut key_len: uint16_t,
    mut val: *mut brubeck_metric,
) -> bool {
    let mut h: ck_ht_hash_t = ck_ht_hash_t { value: 0 };
    let mut entry: ck_ht_entry_t = ck_ht_entry_t {
        key: 0,
        value: 0,
        key_length: 0,
        hash: 0,
    };
    let mut result: bool = false;
    ck_ht_hash(&mut h, &mut (*ht).table, key as *const libc::c_void, key_len);
    ck_ht_entry_set(
        &mut entry,
        h,
        key as *const libc::c_void,
        key_len,
        val as *const libc::c_void,
    );
    pthread_mutex_lock(&mut (*ht).write_mutex);
    result = ck_ht_put_spmc(&mut (*ht).table, h, &mut entry);
    pthread_mutex_unlock(&mut (*ht).write_mutex);
    return result;
}
pub unsafe extern "C" fn brubeck_hashtable_size(
    mut ht: *mut brubeck_hashtable_t,
) -> size_t {
    let mut len: size_t = 0;
    pthread_mutex_lock(&mut (*ht).write_mutex);
    len = ck_ht_count(&mut (*ht).table);
    pthread_mutex_unlock(&mut (*ht).write_mutex);
    return len;
}
pub unsafe extern "C" fn brubeck_hashtable_foreach(
    mut ht: *mut brubeck_hashtable_t,
    mut callback: Option::<
        unsafe extern "C" fn(*mut brubeck_metric, *mut libc::c_void) -> (),
    >,
    mut payload: *mut libc::c_void,
) {
    let mut iterator: ck_ht_iterator_t = ck_ht_iterator_t {
        current: 0 as *mut ck_ht_entry,
        offset: 0,
    };
    let mut entry: *mut ck_ht_entry_t = 0 as *mut ck_ht_entry_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: bool = false;
    iterator.current = 0 as *mut libc::c_void as *mut ck_ht_entry;
    iterator.offset = 0 as libc::c_int as uint64_t;
    pthread_mutex_lock(&mut (*ht).write_mutex);
    loop {
        tmp___0 = ck_ht_next(&mut (*ht).table, &mut iterator, &mut entry);
        if !tmp___0 {
            break;
        }
        tmp = ck_ht_entry_value(entry);
        (Some(callback.expect("non-null function pointer")))
            .expect("non-null function pointer")(tmp as *mut brubeck_metric, payload);
    }
    pthread_mutex_unlock(&mut (*ht).write_mutex);
}
pub unsafe extern "C" fn brubeck_hashtable_to_a(
    mut ht: *mut brubeck_hashtable_t,
    mut length: *mut size_t,
) -> *mut *mut brubeck_metric {
    let mut iterator: ck_ht_iterator_t = ck_ht_iterator_t {
        current: 0 as *mut ck_ht_entry,
        offset: 0,
    };
    let mut entry: *mut ck_ht_entry_t = 0 as *mut ck_ht_entry_t;
    let mut array: *mut *mut brubeck_metric = 0 as *mut *mut brubeck_metric;
    let mut i: size_t = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: bool = false;
    iterator.current = 0 as *mut libc::c_void as *mut ck_ht_entry;
    iterator.offset = 0 as libc::c_int as uint64_t;
    i = 0 as libc::c_int as size_t;
    pthread_mutex_lock(&mut (*ht).write_mutex);
    *length = ck_ht_count(&mut (*ht).table);
    tmp = xmalloc(
        (*length)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
    );
    array = tmp as *mut *mut brubeck_metric;
    loop {
        tmp___2 = ck_ht_next(&mut (*ht).table, &mut iterator, &mut entry);
        if !tmp___2 {
            break;
        }
        tmp___0 = i;
        i = i.wrapping_add(1);
        tmp___1 = ck_ht_entry_value(entry);
        let ref mut fresh2 = *array.offset(tmp___0 as isize);
        *fresh2 = tmp___1 as *mut brubeck_metric;
    }
    pthread_mutex_unlock(&mut (*ht).write_mutex);
    return array;
}
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 255 as libc::c_int
        | (__bsx as libc::c_int & 255 as libc::c_int) << 8 as libc::c_int) as __uint16_t;
}
#[inline]
unsafe extern "C" fn json_decref(mut json: *mut json_t) {
    let mut tmp: size_t = 0;
    if !json.is_null() {
        if (*json).refcount != 0xffffffffffffffff as libc::c_ulong {
            let fresh3 = &mut (*json).refcount as *mut size_t;
            let fresh4 = 1 as libc::c_int as size_t;
            tmp = ::std::intrinsics::atomic_xsub_release(fresh3, fresh4) - fresh4;
            if tmp == 0 as libc::c_int as size_t {
                json_delete(json);
            }
        }
    }
}
#[inline]
unsafe extern "C" fn brubeck_metric_set_state(
    mut metric: *mut brubeck_metric,
    state: uint8_t,
) {
    ::std::intrinsics::atomic_store(
        &mut (*metric).private_state,
        state as libc::c_int as uint8_t,
    );
}
#[inline]
unsafe extern "C" fn starts_with(
    mut str: *const libc::c_char,
    mut prefix: *const libc::c_char,
) -> libc::c_int {
    loop {
        if *prefix == 0 {
            return 1 as libc::c_int
        } else {
            if *str as libc::c_int != *prefix as libc::c_int {
                return 0 as libc::c_int;
            }
        }
        str = str.offset(1);
        prefix = prefix.offset(1);
    };
}
unsafe extern "C" fn flow_stats(mut server: *mut brubeck_server) -> *mut MHD_Response {
    return 0 as *mut libc::c_void as *mut MHD_Response;
}
unsafe extern "C" fn safe_lookup_metric(
    mut server: *mut brubeck_server,
    mut key: *const libc::c_char,
) -> *mut brubeck_metric {
    let mut tmp: size_t = 0;
    let mut tmp___0: *mut brubeck_metric = 0 as *mut brubeck_metric;
    tmp = strlen(key);
    tmp___0 = brubeck_hashtable_find((*server).metrics, key, tmp as uint16_t);
    return tmp___0;
}
unsafe extern "C" fn expire_metric(
    mut server: *mut brubeck_server,
    mut url: *const libc::c_char,
) -> *mut MHD_Response {
    let mut metric: *mut brubeck_metric = 0 as *mut brubeck_metric;
    let mut tmp: size_t = 0;
    let mut tmp___0: *mut brubeck_metric = 0 as *mut brubeck_metric;
    let mut tmp___1: *mut MHD_Response = 0 as *mut MHD_Response;
    tmp = strlen(b"/expire/\0" as *const u8 as *const libc::c_char);
    tmp___0 = safe_lookup_metric(server, url.offset(tmp as isize));
    metric = tmp___0;
    if !metric.is_null() {
        brubeck_metric_set_state(metric, 0 as libc::c_int as uint8_t);
        tmp___1 = MHD_create_response_from_buffer(
            0 as libc::c_int as size_t,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
            MHD_RESPMEM_PERSISTENT,
        );
        return tmp___1;
    }
    return 0 as *mut libc::c_void as *mut MHD_Response;
}
static mut metric_types: [*const libc::c_char; 6] = [
    b"gauge\0" as *const u8 as *const libc::c_char,
    b"meter\0" as *const u8 as *const libc::c_char,
    b"counter\0" as *const u8 as *const libc::c_char,
    b"histogram\0" as *const u8 as *const libc::c_char,
    b"timer\0" as *const u8 as *const libc::c_char,
    b"internal\0" as *const u8 as *const libc::c_char,
];
static mut expire_status: [*const libc::c_char; 3] = [
    b"disabled\0" as *const u8 as *const libc::c_char,
    b"inactive\0" as *const u8 as *const libc::c_char,
    b"active\0" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn send_metric(
    mut server: *mut brubeck_server,
    mut url: *const libc::c_char,
) -> *mut MHD_Response {
    let mut metric: *mut brubeck_metric = 0 as *mut brubeck_metric;
    let mut tmp: size_t = 0;
    let mut tmp___0: *mut brubeck_metric = 0 as *mut brubeck_metric;
    let mut mj: *mut json_t = 0 as *mut json_t;
    let mut tmp___1: uint8_t = 0;
    let mut tmp___2: *mut json_t = 0 as *mut json_t;
    let mut jsonr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: size_t = 0;
    let mut tmp___5: *mut MHD_Response = 0 as *mut MHD_Response;
    tmp = strlen(b"/metric/\0" as *const u8 as *const libc::c_char);
    tmp___0 = safe_lookup_metric(server, url.offset(tmp as isize));
    metric = tmp___0;
    if !metric.is_null() {
        tmp___1 = brubeck_metric_get_state(metric as *const brubeck_metric);
        tmp___2 = json_pack(
            b"{s:s, s:s, s:i, s:s}\0" as *const u8 as *const libc::c_char,
            b"key\0" as *const u8 as *const libc::c_char,
            ((*metric).key).as_mut_ptr(),
            b"type\0" as *const u8 as *const libc::c_char,
            metric_types[(*metric).type_0 as usize],
            b"shard\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"expire\0" as *const u8 as *const libc::c_char,
            expire_status[tmp___1 as usize],
        );
        mj = tmp___2;
        tmp___3 = json_dumps(mj as *const json_t, 260 as libc::c_int as size_t);
        jsonr = tmp___3;
        json_decref(mj);
        tmp___4 = strlen(jsonr as *const libc::c_char);
        tmp___5 = MHD_create_response_from_buffer(
            tmp___4,
            jsonr as *mut libc::c_void,
            MHD_RESPMEM_MUST_FREE,
        );
        return tmp___5;
    }
    return 0 as *mut libc::c_void as *mut MHD_Response;
}
unsafe extern "C" fn send_stats(mut brubeck: *mut brubeck_server) -> *mut MHD_Response {
    let mut jsonr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stats: *mut json_t = 0 as *mut json_t;
    let mut backends: *mut json_t = 0 as *mut json_t;
    let mut samplers: *mut json_t = 0 as *mut json_t;
    let mut i: libc::c_int = 0;
    let mut backend: *mut brubeck_backend = 0 as *mut brubeck_backend;
    let mut carbon: *mut brubeck_carbon = 0 as *mut brubeck_carbon;
    let mut address: *mut sockaddr_in = 0 as *mut sockaddr_in;
    let mut addr: [libc::c_char; 16] = [0; 16];
    let mut tmp: __uint16_t = 0;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: *mut json_t = 0 as *mut json_t;
    let mut kafka: *mut brubeck_kafka = 0 as *mut brubeck_kafka;
    let mut tmp___2: *mut json_t = 0 as *mut json_t;
    let mut sampler: *mut brubeck_sampler = 0 as *mut brubeck_sampler;
    let mut address___0: *mut sockaddr_in = 0 as *mut sockaddr_in;
    let mut addr___0: [libc::c_char; 16] = [0; 16];
    let mut sampler_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___3: __uint16_t = 0;
    let mut tmp___4: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___5: *mut json_t = 0 as *mut json_t;
    let mut tmp___6: size_t = 0;
    let mut tmp___7: *mut MHD_Response = 0 as *mut MHD_Response;
    backends = json_array();
    i = 0 as libc::c_int;
    while i < (*brubeck).active_backends {
        backend = (*brubeck).backends[i as usize];
        if (*backend).type_0 as libc::c_uint == 0 as libc::c_uint {
            carbon = backend as *mut brubeck_carbon;
            address = &mut (*carbon).out_sockaddr;
            tmp = __bswap_16((*address).sin_port);
            tmp___0 = inet_ntop(
                2 as libc::c_int,
                &mut (*address).sin_addr.s_addr as *mut in_addr_t as *const libc::c_void,
                addr.as_mut_ptr(),
                16 as libc::c_int as socklen_t,
            );
            tmp___1 = json_pack(
                b"{s:s, s:i, s:b, s:s, s:i, s:I}\0" as *const u8 as *const libc::c_char,
                b"type\0" as *const u8 as *const libc::c_char,
                b"carbon\0" as *const u8 as *const libc::c_char,
                b"sample_freq\0" as *const u8 as *const libc::c_char,
                (*carbon).backend.sample_freq,
                b"connected\0" as *const u8 as *const libc::c_char,
                ((*carbon).out_sock >= 0 as libc::c_int) as libc::c_int,
                b"address\0" as *const u8 as *const libc::c_char,
                tmp___0,
                b"port\0" as *const u8 as *const libc::c_char,
                tmp as libc::c_int,
                b"bytes_sent\0" as *const u8 as *const libc::c_char,
                (*carbon).bytes_sent as json_int_t,
            );
            json_array_append_new(backends, tmp___1);
        } else if (*backend).type_0 as libc::c_uint == 1 as libc::c_uint {
            kafka = backend as *mut brubeck_kafka;
            tmp___2 = json_pack(
                b"{s:s, s:i, s:b, s:I}\0" as *const u8 as *const libc::c_char,
                b"type\0" as *const u8 as *const libc::c_char,
                b"kafka\0" as *const u8 as *const libc::c_char,
                b"sample_freq\0" as *const u8 as *const libc::c_char,
                (*kafka).backend.sample_freq,
                b"connected\0" as *const u8 as *const libc::c_char,
                (*kafka).connected as libc::c_int,
                b"bytes_sent\0" as *const u8 as *const libc::c_char,
                (*kafka).bytes_sent as json_int_t,
            );
            json_array_append_new(backends, tmp___2);
        }
        i += 1;
    }
    samplers = json_array();
    i = 0 as libc::c_int;
    while i < (*brubeck).active_samplers {
        sampler = (*brubeck).samplers[i as usize];
        address___0 = &mut (*sampler).addr;
        sampler_name = 0 as *mut libc::c_void as *const libc::c_char;
        match (*sampler).type_0 as libc::c_uint {
            0 => {
                sampler_name = b"statsd\0" as *const u8 as *const libc::c_char;
            }
            _ => {}
        }
        tmp___3 = __bswap_16((*address___0).sin_port);
        tmp___4 = inet_ntop(
            2 as libc::c_int,
            &mut (*address___0).sin_addr.s_addr as *mut in_addr_t as *const libc::c_void,
            addr___0.as_mut_ptr(),
            16 as libc::c_int as socklen_t,
        );
        tmp___5 = json_pack(
            b"{s:s, s:f, s:s, s:i}\0" as *const u8 as *const libc::c_char,
            b"type\0" as *const u8 as *const libc::c_char,
            sampler_name,
            b"sample_freq\0" as *const u8 as *const libc::c_char,
            (*sampler).current_flow as libc::c_double,
            b"address\0" as *const u8 as *const libc::c_char,
            tmp___4,
            b"port\0" as *const u8 as *const libc::c_char,
            tmp___3 as libc::c_int,
        );
        json_array_append_new(samplers, tmp___5);
        i += 1;
    }
    stats = json_pack(
        b"{s:s, s:i, s:i, s:i, s:o, s:o}\0" as *const u8 as *const libc::c_char,
        b"version\0" as *const u8 as *const libc::c_char,
        b"brubeck f306c25\0" as *const u8 as *const libc::c_char,
        b"metrics\0" as *const u8 as *const libc::c_char,
        (*brubeck).internal_stats.sample.metrics,
        b"errors\0" as *const u8 as *const libc::c_char,
        (*brubeck).internal_stats.sample.errors,
        b"unique_keys\0" as *const u8 as *const libc::c_char,
        (*brubeck).internal_stats.sample.unique_keys,
        b"backends\0" as *const u8 as *const libc::c_char,
        backends,
        b"samplers\0" as *const u8 as *const libc::c_char,
        samplers,
    );
    jsonr = json_dumps(stats as *const json_t, 260 as libc::c_int as size_t);
    json_decref(stats);
    tmp___6 = strlen(jsonr as *const libc::c_char);
    tmp___7 = MHD_create_response_from_buffer(
        tmp___6,
        jsonr as *mut libc::c_void,
        MHD_RESPMEM_MUST_FREE,
    );
    return tmp___7;
}
unsafe extern "C" fn send_ping(mut brubeck: *mut brubeck_server) -> *mut MHD_Response {
    let mut frequency: value_t = 0.;
    let mut status: *const libc::c_char = 0 as *const libc::c_char;
    let mut jsonr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stats: *mut json_t = 0 as *mut json_t;
    let mut i: libc::c_int = 0;
    let mut backend: *mut brubeck_backend = 0 as *mut brubeck_backend;
    let mut tmp: bool = false;
    let mut tmp___0: __pid_t = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: *mut MHD_Response = 0 as *mut MHD_Response;
    frequency = (*brubeck).internal_stats.sample_freq as libc::c_double;
    status = b"OK\0" as *const u8 as *const libc::c_char;
    i = 0 as libc::c_int;
    while i < (*brubeck).active_backends {
        backend = (*brubeck).backends[i as usize];
        tmp = (Some(((*backend).is_connected).expect("non-null function pointer")))
            .expect("non-null function pointer")(backend as *mut libc::c_void);
        if !tmp {
            status = b"ERROR (backend disconnected)\0" as *const u8
                as *const libc::c_char;
            break;
        } else {
            i += 1;
        }
    }
    tmp___0 = getpid();
    stats = json_pack(
        b"{s:s, s:i, s:s, s:f, s:f, s:i}\0" as *const u8 as *const libc::c_char,
        b"version\0" as *const u8 as *const libc::c_char,
        b"brubeck f306c25\0" as *const u8 as *const libc::c_char,
        b"pid\0" as *const u8 as *const libc::c_char,
        tmp___0,
        b"status\0" as *const u8 as *const libc::c_char,
        status,
        b"metrics_per_second\0" as *const u8 as *const libc::c_char,
        (*brubeck).internal_stats.sample.metrics as value_t / frequency,
        b"errors_per_second\0" as *const u8 as *const libc::c_char,
        (*brubeck).internal_stats.sample.errors as value_t / frequency,
        b"unique_keys\0" as *const u8 as *const libc::c_char,
        (*brubeck).internal_stats.sample.unique_keys,
    );
    jsonr = json_dumps(stats as *const json_t, 260 as libc::c_int as size_t);
    json_decref(stats);
    tmp___1 = strlen(jsonr as *const libc::c_char);
    tmp___2 = MHD_create_response_from_buffer(
        tmp___1,
        jsonr as *mut libc::c_void,
        MHD_RESPMEM_MUST_FREE,
    );
    return tmp___2;
}
static mut NOT_FOUND: *const libc::c_char = b"404 not found\0" as *const u8
    as *const libc::c_char;
unsafe extern "C" fn handle_request(
    mut cls: *mut libc::c_void,
    mut connection: *mut MHD_Connection,
    mut url: *const libc::c_char,
    mut method: *const libc::c_char,
    mut version: *const libc::c_char,
    mut upload_data: *const libc::c_char,
    mut upload_data_size: *mut size_t,
    mut con_cls: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut response: *mut MHD_Response = 0 as *mut MHD_Response;
    let mut brubeck: *mut brubeck_server = 0 as *mut brubeck_server;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: size_t = 0;
    response = 0 as *mut libc::c_void as *mut MHD_Response;
    brubeck = cls as *mut brubeck_server;
    tmp___6 = strcmp(method, b"GET\0" as *const u8 as *const libc::c_char);
    if tmp___6 != 0 {
        tmp___5 = strcmp(method, b"POST\0" as *const u8 as *const libc::c_char);
        if tmp___5 == 0 {
            tmp___4 = starts_with(
                url,
                b"/expire/\0" as *const u8 as *const libc::c_char,
            );
            if tmp___4 != 0 {
                response = expire_metric(brubeck, url);
            }
        }
    } else {
        tmp___2 = strcmp(url, b"/_ping\0" as *const u8 as *const libc::c_char);
        if tmp___2 != 0 {
            tmp___3 = strcmp(url, b"/ping\0" as *const u8 as *const libc::c_char);
            if tmp___3 != 0 {
                tmp___1 = strcmp(url, b"/stats\0" as *const u8 as *const libc::c_char);
                if tmp___1 != 0 {
                    tmp___0 = strcmp(
                        url,
                        b"/flow_stats\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___0 != 0 {
                        tmp = starts_with(
                            url,
                            b"/metric/\0" as *const u8 as *const libc::c_char,
                        );
                        if tmp != 0 {
                            response = send_metric(brubeck, url);
                        }
                    } else {
                        response = flow_stats(brubeck);
                    }
                } else {
                    response = send_stats(brubeck);
                }
            } else {
                response = send_ping(brubeck);
            }
        } else {
            response = send_ping(brubeck);
        }
    }
    if response.is_null() {
        tmp___7 = strlen(NOT_FOUND);
        response = MHD_create_response_from_buffer(
            tmp___7,
            NOT_FOUND as *mut libc::c_void,
            MHD_RESPMEM_PERSISTENT,
        );
        MHD_add_response_header(
            response,
            b"Connection\0" as *const u8 as *const libc::c_char,
            b"close\0" as *const u8 as *const libc::c_char,
        );
        ret = MHD_queue_response(connection, 404 as libc::c_uint, response);
    } else {
        MHD_add_response_header(
            response,
            b"Connection\0" as *const u8 as *const libc::c_char,
            b"close\0" as *const u8 as *const libc::c_char,
        );
        MHD_add_response_header(
            response,
            b"Content-Type\0" as *const u8 as *const libc::c_char,
            b"application/json\0" as *const u8 as *const libc::c_char,
        );
        ret = MHD_queue_response(connection, 200 as libc::c_uint, response);
    }
    MHD_destroy_response(response);
    return ret;
}
pub unsafe extern "C" fn brubeck_http_endpoint_init(
    mut server: *mut brubeck_server,
    mut listen___0: *const libc::c_char,
) {
    let mut daemon___0: *mut MHD_Daemon = 0 as *mut MHD_Daemon;
    let mut port: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    tmp = strrchr(listen___0, ':' as i32);
    port = tmp as *const libc::c_char;
    if !port.is_null() {
        port = port.offset(1);
    } else {
        port = listen___0;
    }
    tmp___0 = atoi(port);
    daemon___0 = MHD_start_daemon(
        8 as libc::c_uint,
        tmp___0 as uint16_t,
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const sockaddr,
                    socklen_t,
                ) -> libc::c_int,
            >,
        >(0 as *mut libc::c_void),
        0 as *mut libc::c_void,
        Some(
            handle_request
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut MHD_Connection,
                    *const libc::c_char,
                    *const libc::c_char,
                    *const libc::c_char,
                    *const libc::c_char,
                    *mut size_t,
                    *mut *mut libc::c_void,
                ) -> libc::c_int,
        ),
        server as *mut libc::c_void,
        3 as libc::c_int,
        10 as libc::c_uint,
        0 as libc::c_int,
    );
    if daemon___0.is_null() {
        fprintf(
            stderr,
            b"[FATAL]: failed to start HTTP endpoint\n\0" as *const u8
                as *const libc::c_char,
        );
        gh_log_die();
    }
    tmp___1 = gh_log_instance();
    gh_log_write(
        b"instance=%s event=http_server listen=%s\n\0" as *const u8
            as *const libc::c_char,
        tmp___1,
        port,
    );
}
pub unsafe extern "C" fn brubeck_internal__sample(
    mut metric: *mut brubeck_metric,
    mut sample: Option::<
        unsafe extern "C" fn(
            *const brubeck_metric,
            *const libc::c_char,
            value_t,
            *mut libc::c_void,
        ) -> (),
    >,
    mut opaque: *mut libc::c_void,
) {
    let mut stats: *mut brubeck_internal_stats = 0 as *mut brubeck_internal_stats;
    let mut value: uint32_t = 0;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: size_t = 0;
    let mut tmp___0: *mut _ = 0 as *mut _;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: size_t = 0;
    stats = (*metric).as_0.other as *mut brubeck_internal_stats;
    tmp = strlen(b".unique_keys\0" as *const u8 as *const libc::c_char);
    let mut fresh5 = ::std::vec::from_elem(
        0,
        ((*metric).key_len as size_t).wrapping_add(tmp).wrapping_add(1 as libc::c_ulong)
            as usize,
    );
    tmp___0 = fresh5.as_mut_ptr();
    key = tmp___0 as *mut libc::c_char;
    memcpy(
        key as *mut libc::c_void,
        ((*metric).key).as_mut_ptr() as *const libc::c_void,
        (*metric).key_len as size_t,
    );
    tmp___1 = strlen(b".metrics\0" as *const u8 as *const libc::c_char);
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".metrics\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        tmp___1.wrapping_add(1 as libc::c_ulong),
    );
    value = ::std::intrinsics::atomic_xchg_acquire(
        &mut (*stats).live.metrics as *mut uint32_t,
        0 as libc::c_int as uint32_t,
    );
    (*stats).sample.metrics = value;
    (Some(sample.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        metric as *const brubeck_metric,
        key as *const libc::c_char,
        value as value_t,
        opaque,
    );
    tmp___2 = strlen(b".errors\0" as *const u8 as *const libc::c_char);
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".errors\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        tmp___2.wrapping_add(1 as libc::c_ulong),
    );
    value = ::std::intrinsics::atomic_xchg_acquire(
        &mut (*stats).live.errors as *mut uint32_t,
        0 as libc::c_int as uint32_t,
    );
    (*stats).sample.errors = value;
    (Some(sample.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        metric as *const brubeck_metric,
        key as *const libc::c_char,
        value as value_t,
        opaque,
    );
    tmp___3 = strlen(b".unique_keys\0" as *const u8 as *const libc::c_char);
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".unique_keys\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        tmp___3.wrapping_add(1 as libc::c_ulong),
    );
    let fresh6 = &mut (*stats).live.unique_keys as *mut uint32_t;
    let fresh7 = 0 as libc::c_int as uint32_t;
    value = ::std::intrinsics::atomic_xadd_seqcst(fresh6, fresh7) + fresh7;
    (*stats).sample.unique_keys = value;
    (Some(sample.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        metric as *const brubeck_metric,
        key as *const libc::c_char,
        value as value_t,
        opaque,
    );
    brubeck_metric_set_state(metric, 2 as libc::c_int as uint8_t);
}
pub unsafe extern "C" fn brubeck_internal__init(mut server: *mut brubeck_server) {
    let mut internal: *mut brubeck_metric = 0 as *mut brubeck_metric;
    let mut backend: *mut brubeck_backend = 0 as *mut brubeck_backend;
    let mut tmp: size_t = 0;
    tmp = strlen((*server).name);
    internal = brubeck_metric_new(
        server,
        (*server).name,
        tmp,
        5 as libc::c_int as uint8_t,
    );
    if internal as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"[FATAL]: Failed to initialize internal stats sampler\n\0" as *const u8
                as *const libc::c_char,
        );
        gh_log_die();
    }
    (*internal)
        .as_0
        .other = &mut (*server).internal_stats as *mut brubeck_internal_stats
        as *mut libc::c_void;
    backend = brubeck_metric_shard(server, internal);
    (*server).internal_stats.sample_freq = (*backend).sample_freq;
}
static mut gh_log_path: *const libc::c_char = 0 as *const libc::c_void
    as *mut libc::c_void as *const libc::c_char;
static mut gh_log_file: *mut FILE = 0 as *const FILE as *mut FILE;
static mut gh_syslog_enabled: libc::c_int = 0;
pub unsafe extern "C" fn gh_log_open(mut path: *const libc::c_char) {
    let mut new_log: *mut FILE = 0 as *mut FILE;
    let mut tmp: libc::c_int = 0;
    if path as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        gh_syslog_enabled = 0 as libc::c_int;
        gh_log_file = 0 as *mut libc::c_void as *mut FILE;
        return;
    }
    tmp = strcmp(path, b"syslog\0" as *const u8 as *const libc::c_char);
    if tmp == 0 {
        openlog(
            0 as *mut libc::c_void as *const libc::c_char,
            1 as libc::c_int,
            (23 as libc::c_int) << 3 as libc::c_int,
        );
        gh_syslog_enabled = 1 as libc::c_int;
        return;
    }
    new_log = fopen(path, b"a\0" as *const u8 as *const libc::c_char);
    if new_log as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"Failed to open log file at '%s'\n\0" as *const u8 as *const libc::c_char,
            path,
        );
        return;
    }
    if !gh_log_file.is_null() {
        fclose(gh_log_file);
    }
    if gh_syslog_enabled != 0 {
        closelog();
        gh_syslog_enabled = 0 as libc::c_int;
    }
    gh_log_file = new_log;
    gh_log_path = path;
}
pub unsafe extern "C" fn gh_log_reopen() {
    if gh_log_path as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        gh_log_open(gh_log_path);
    }
}
pub unsafe extern "C" fn gh_log_write(mut message: *const libc::c_char, mut args: ...) {
    let mut vl: ::std::ffi::VaListImpl;
    vl = args.clone();
    if gh_syslog_enabled != 0 {
        vsyslog(6 as libc::c_int, message, vl.as_va_list());
    } else if !gh_log_file.is_null() {
        vfprintf(gh_log_file, message, vl.as_va_list());
        fflush(gh_log_file);
    } else {
        vfprintf(stderr, message, vl.as_va_list());
        fflush(stderr);
    };
}
pub unsafe extern "C" fn gh_log_die() -> ! {
    exit(1 as libc::c_int);
}
static mut _app_instance: *const libc::c_char = 0 as *const libc::c_void
    as *mut libc::c_void as *const libc::c_char;
pub unsafe extern "C" fn gh_log_instance() -> *const libc::c_char {
    return _app_instance;
}
pub unsafe extern "C" fn gh_log_set_instance(mut instance: *const libc::c_char) {
    _app_instance = instance;
}
#[inline]
unsafe extern "C" fn new_metric(
    mut server: *mut brubeck_server,
    mut key: *const libc::c_char,
    mut key_len: size_t,
    mut type_0: uint8_t,
) -> *mut brubeck_metric {
    let mut metric: *mut brubeck_metric = 0 as *mut brubeck_metric;
    let mut tags: *const brubeck_tag_set = 0 as *const brubeck_tag_set;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tags = 0 as *mut libc::c_void as *const brubeck_tag_set;
    if !((*server).tags).is_null() {
        tags = brubeck_get_tag_set((*server).tags, key, key_len as uint16_t);
        if !tags.is_null() {
            key_len = (key_len as libc::c_ulong).wrapping_sub((*tags).tag_len as size_t)
                as size_t as size_t;
        }
    }
    tmp = brubeck_slab_alloc(
        &mut (*server).slab,
        (::std::mem::size_of::<brubeck_metric>() as libc::c_ulong)
            .wrapping_add(key_len)
            .wrapping_add(1 as libc::c_ulong),
    );
    metric = tmp as *mut brubeck_metric;
    memset(
        metric as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<brubeck_metric>() as libc::c_ulong,
    );
    (*metric).tags = tags;
    memcpy(
        ((*metric).key).as_mut_ptr() as *mut libc::c_void,
        key as *const libc::c_void,
        key_len,
    );
    *((*metric).key)
        .as_mut_ptr()
        .offset(key_len as isize) = '\u{0}' as i32 as libc::c_char;
    (*metric).key_len = key_len as uint16_t;
    brubeck_metric_set_state(metric, 2 as libc::c_int as uint8_t);
    (*metric).type_0 = type_0;
    pthread_spin_init(&mut (*metric).lock, 0 as libc::c_int);
    return metric;
}
unsafe extern "C" fn gauge__record(
    mut metric: *mut brubeck_metric,
    mut value: value_t,
    mut sample_freq: value_t,
    mut modifiers: uint8_t,
) {
    pthread_spin_lock(&mut (*metric).lock);
    if modifiers as libc::c_int & 1 as libc::c_int != 0 {
        (*metric).as_0.gauge.value += value;
    } else {
        (*metric).as_0.gauge.value = value;
    }
    pthread_spin_unlock(&mut (*metric).lock);
}
unsafe extern "C" fn gauge__sample(
    mut metric: *mut brubeck_metric,
    mut sample: Option::<
        unsafe extern "C" fn(
            *const brubeck_metric,
            *const libc::c_char,
            value_t,
            *mut libc::c_void,
        ) -> (),
    >,
    mut opaque: *mut libc::c_void,
) {
    let mut value: value_t = 0.;
    pthread_spin_lock(&mut (*metric).lock);
    value = (*metric).as_0.gauge.value;
    pthread_spin_unlock(&mut (*metric).lock);
    (Some(sample.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        metric as *const brubeck_metric,
        ((*metric).key).as_mut_ptr() as *const libc::c_char,
        value,
        opaque,
    );
}
unsafe extern "C" fn meter__record(
    mut metric: *mut brubeck_metric,
    mut value: value_t,
    mut sample_freq: value_t,
    mut modifiers: uint8_t,
) {
    value *= sample_freq;
    pthread_spin_lock(&mut (*metric).lock);
    (*metric).as_0.meter.value += value;
    pthread_spin_unlock(&mut (*metric).lock);
}
unsafe extern "C" fn meter__sample(
    mut metric: *mut brubeck_metric,
    mut sample: Option::<
        unsafe extern "C" fn(
            *const brubeck_metric,
            *const libc::c_char,
            value_t,
            *mut libc::c_void,
        ) -> (),
    >,
    mut opaque: *mut libc::c_void,
) {
    let mut value: value_t = 0.;
    pthread_spin_lock(&mut (*metric).lock);
    value = (*metric).as_0.meter.value;
    (*metric).as_0.meter.value = 0.0f64;
    pthread_spin_unlock(&mut (*metric).lock);
    (Some(sample.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        metric as *const brubeck_metric,
        ((*metric).key).as_mut_ptr() as *const libc::c_char,
        value,
        opaque,
    );
}
unsafe extern "C" fn counter__record(
    mut metric: *mut brubeck_metric,
    mut value: value_t,
    mut sample_freq: value_t,
    mut modifiers: uint8_t,
) {
    let mut diff: value_t = 0.;
    let mut tmp: value_t = 0.;
    value *= sample_freq;
    pthread_spin_lock(&mut (*metric).lock);
    if (*metric).as_0.counter.previous > 0.0f64 {
        if value >= (*metric).as_0.counter.previous {
            tmp = value - (*metric).as_0.counter.previous;
        } else {
            tmp = value;
        }
        diff = tmp;
        (*metric).as_0.counter.value += diff;
    }
    (*metric).as_0.counter.previous = value;
    pthread_spin_unlock(&mut (*metric).lock);
}
unsafe extern "C" fn counter__sample(
    mut metric: *mut brubeck_metric,
    mut sample: Option::<
        unsafe extern "C" fn(
            *const brubeck_metric,
            *const libc::c_char,
            value_t,
            *mut libc::c_void,
        ) -> (),
    >,
    mut opaque: *mut libc::c_void,
) {
    let mut value: value_t = 0.;
    pthread_spin_lock(&mut (*metric).lock);
    value = (*metric).as_0.counter.value;
    (*metric).as_0.counter.value = 0.0f64;
    pthread_spin_unlock(&mut (*metric).lock);
    (Some(sample.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        metric as *const brubeck_metric,
        ((*metric).key).as_mut_ptr() as *const libc::c_char,
        value,
        opaque,
    );
}
unsafe extern "C" fn histogram__record(
    mut metric: *mut brubeck_metric,
    mut value: value_t,
    mut sample_freq: value_t,
    mut modifiers: uint8_t,
) {
    pthread_spin_lock(&mut (*metric).lock);
    brubeck_histo_push(&mut (*metric).as_0.histogram, value, sample_freq);
    pthread_spin_unlock(&mut (*metric).lock);
}
unsafe extern "C" fn histogram__sample(
    mut metric: *mut brubeck_metric,
    mut sample: Option::<
        unsafe extern "C" fn(
            *const brubeck_metric,
            *const libc::c_char,
            value_t,
            *mut libc::c_void,
        ) -> (),
    >,
    mut opaque: *mut libc::c_void,
) {
    let mut hsample: brubeck_histo_sample = brubeck_histo_sample {
        sum: 0.,
        min: 0.,
        max: 0.,
        mean: 0.,
        median: 0.,
        count: 0.,
        percentile: [0.; 5],
    };
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: size_t = 0;
    let mut tmp___0: *mut _ = 0 as *mut _;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: size_t = 0;
    let mut backend: *mut brubeck_backend = 0 as *mut brubeck_backend;
    let mut tmp___3: size_t = 0;
    let mut tmp___4: size_t = 0;
    let mut tmp___5: size_t = 0;
    let mut tmp___6: size_t = 0;
    let mut tmp___7: size_t = 0;
    let mut tmp___8: size_t = 0;
    let mut tmp___9: size_t = 0;
    let mut tmp___10: size_t = 0;
    let mut tmp___11: size_t = 0;
    let mut tmp___12: size_t = 0;
    pthread_spin_lock(&mut (*metric).lock);
    brubeck_histo_sample(&mut hsample, &mut (*metric).as_0.histogram);
    pthread_spin_unlock(&mut (*metric).lock);
    tmp = strlen(b".percentile.999\0" as *const u8 as *const libc::c_char);
    let mut fresh8 = ::std::vec::from_elem(
        0,
        ((*metric).key_len as size_t).wrapping_add(tmp).wrapping_add(1 as libc::c_ulong)
            as usize,
    );
    tmp___0 = fresh8.as_mut_ptr();
    key = tmp___0 as *mut libc::c_char;
    memcpy(
        key as *mut libc::c_void,
        ((*metric).key).as_mut_ptr() as *const libc::c_void,
        (*metric).key_len as size_t,
    );
    tmp___1 = strlen(b".count\0" as *const u8 as *const libc::c_char);
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".count\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        tmp___1.wrapping_add(1 as libc::c_ulong),
    );
    (Some(sample.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        metric as *const brubeck_metric,
        key as *const libc::c_char,
        hsample.count,
        opaque,
    );
    tmp___2 = strlen(b".count_ps\0" as *const u8 as *const libc::c_char);
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".count_ps\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        tmp___2.wrapping_add(1 as libc::c_ulong),
    );
    backend = opaque as *mut brubeck_backend;
    (Some(sample.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        metric as *const brubeck_metric,
        key as *const libc::c_char,
        hsample.count / (*backend).sample_freq as libc::c_double,
        opaque,
    );
    if hsample.count == 0.0f64 {
        return;
    }
    tmp___3 = strlen(b".min\0" as *const u8 as *const libc::c_char);
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".min\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        tmp___3.wrapping_add(1 as libc::c_ulong),
    );
    (Some(sample.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        metric as *const brubeck_metric,
        key as *const libc::c_char,
        hsample.min,
        opaque,
    );
    tmp___4 = strlen(b".max\0" as *const u8 as *const libc::c_char);
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".max\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        tmp___4.wrapping_add(1 as libc::c_ulong),
    );
    (Some(sample.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        metric as *const brubeck_metric,
        key as *const libc::c_char,
        hsample.max,
        opaque,
    );
    tmp___5 = strlen(b".sum\0" as *const u8 as *const libc::c_char);
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".sum\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        tmp___5.wrapping_add(1 as libc::c_ulong),
    );
    (Some(sample.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        metric as *const brubeck_metric,
        key as *const libc::c_char,
        hsample.sum,
        opaque,
    );
    tmp___6 = strlen(b".mean\0" as *const u8 as *const libc::c_char);
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".mean\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        tmp___6.wrapping_add(1 as libc::c_ulong),
    );
    (Some(sample.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        metric as *const brubeck_metric,
        key as *const libc::c_char,
        hsample.mean,
        opaque,
    );
    tmp___7 = strlen(b".median\0" as *const u8 as *const libc::c_char);
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".median\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        tmp___7.wrapping_add(1 as libc::c_ulong),
    );
    (Some(sample.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        metric as *const brubeck_metric,
        key as *const libc::c_char,
        hsample.median,
        opaque,
    );
    tmp___8 = strlen(b".percentile.75\0" as *const u8 as *const libc::c_char);
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".percentile.75\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        tmp___8.wrapping_add(1 as libc::c_ulong),
    );
    (Some(sample.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        metric as *const brubeck_metric,
        key as *const libc::c_char,
        hsample.percentile[0 as libc::c_int as usize],
        opaque,
    );
    tmp___9 = strlen(b".percentile.95\0" as *const u8 as *const libc::c_char);
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".percentile.95\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        tmp___9.wrapping_add(1 as libc::c_ulong),
    );
    (Some(sample.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        metric as *const brubeck_metric,
        key as *const libc::c_char,
        hsample.percentile[1 as libc::c_int as usize],
        opaque,
    );
    tmp___10 = strlen(b".percentile.98\0" as *const u8 as *const libc::c_char);
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".percentile.98\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        tmp___10.wrapping_add(1 as libc::c_ulong),
    );
    (Some(sample.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        metric as *const brubeck_metric,
        key as *const libc::c_char,
        hsample.percentile[2 as libc::c_int as usize],
        opaque,
    );
    tmp___11 = strlen(b".percentile.99\0" as *const u8 as *const libc::c_char);
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".percentile.99\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        tmp___11.wrapping_add(1 as libc::c_ulong),
    );
    (Some(sample.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        metric as *const brubeck_metric,
        key as *const libc::c_char,
        hsample.percentile[3 as libc::c_int as usize],
        opaque,
    );
    tmp___12 = strlen(b".percentile.999\0" as *const u8 as *const libc::c_char);
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".percentile.999\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        tmp___12.wrapping_add(1 as libc::c_ulong),
    );
    (Some(sample.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        metric as *const brubeck_metric,
        key as *const libc::c_char,
        hsample.percentile[4 as libc::c_int as usize],
        opaque,
    );
}
static mut _prototypes: [brubeck_metric__proto; 6] = unsafe {
    [
        {
            let mut init = brubeck_metric__proto {
                record: Some(
                    gauge__record
                        as unsafe extern "C" fn(
                            *mut brubeck_metric,
                            value_t,
                            value_t,
                            uint8_t,
                        ) -> (),
                ),
                sample: Some(
                    gauge__sample
                        as unsafe extern "C" fn(
                            *mut brubeck_metric,
                            Option::<
                                unsafe extern "C" fn(
                                    *const brubeck_metric,
                                    *const libc::c_char,
                                    value_t,
                                    *mut libc::c_void,
                                ) -> (),
                            >,
                            *mut libc::c_void,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = brubeck_metric__proto {
                record: Some(
                    meter__record
                        as unsafe extern "C" fn(
                            *mut brubeck_metric,
                            value_t,
                            value_t,
                            uint8_t,
                        ) -> (),
                ),
                sample: Some(
                    meter__sample
                        as unsafe extern "C" fn(
                            *mut brubeck_metric,
                            Option::<
                                unsafe extern "C" fn(
                                    *const brubeck_metric,
                                    *const libc::c_char,
                                    value_t,
                                    *mut libc::c_void,
                                ) -> (),
                            >,
                            *mut libc::c_void,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = brubeck_metric__proto {
                record: Some(
                    counter__record
                        as unsafe extern "C" fn(
                            *mut brubeck_metric,
                            value_t,
                            value_t,
                            uint8_t,
                        ) -> (),
                ),
                sample: Some(
                    counter__sample
                        as unsafe extern "C" fn(
                            *mut brubeck_metric,
                            Option::<
                                unsafe extern "C" fn(
                                    *const brubeck_metric,
                                    *const libc::c_char,
                                    value_t,
                                    *mut libc::c_void,
                                ) -> (),
                            >,
                            *mut libc::c_void,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = brubeck_metric__proto {
                record: Some(
                    histogram__record
                        as unsafe extern "C" fn(
                            *mut brubeck_metric,
                            value_t,
                            value_t,
                            uint8_t,
                        ) -> (),
                ),
                sample: Some(
                    histogram__sample
                        as unsafe extern "C" fn(
                            *mut brubeck_metric,
                            Option::<
                                unsafe extern "C" fn(
                                    *const brubeck_metric,
                                    *const libc::c_char,
                                    value_t,
                                    *mut libc::c_void,
                                ) -> (),
                            >,
                            *mut libc::c_void,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = brubeck_metric__proto {
                record: Some(
                    histogram__record
                        as unsafe extern "C" fn(
                            *mut brubeck_metric,
                            value_t,
                            value_t,
                            uint8_t,
                        ) -> (),
                ),
                sample: Some(
                    histogram__sample
                        as unsafe extern "C" fn(
                            *mut brubeck_metric,
                            Option::<
                                unsafe extern "C" fn(
                                    *const brubeck_metric,
                                    *const libc::c_char,
                                    value_t,
                                    *mut libc::c_void,
                                ) -> (),
                            >,
                            *mut libc::c_void,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = brubeck_metric__proto {
                record: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<
                        unsafe extern "C" fn(
                            *mut brubeck_metric,
                            value_t,
                            value_t,
                            uint8_t,
                        ) -> (),
                    >,
                >(0 as *const libc::c_void as *mut libc::c_void),
                sample: Some(
                    brubeck_internal__sample
                        as unsafe extern "C" fn(
                            *mut brubeck_metric,
                            Option::<
                                unsafe extern "C" fn(
                                    *const brubeck_metric,
                                    *const libc::c_char,
                                    value_t,
                                    *mut libc::c_void,
                                ) -> (),
                            >,
                            *mut libc::c_void,
                        ) -> (),
                ),
            };
            init
        },
    ]
};
pub unsafe extern "C" fn brubeck_metric_sample(
    mut metric: *mut brubeck_metric,
    mut cb: Option::<
        unsafe extern "C" fn(
            *const brubeck_metric,
            *const libc::c_char,
            value_t,
            *mut libc::c_void,
        ) -> (),
    >,
    mut backend: *mut libc::c_void,
) {
    (Some(
        ((*_prototypes.as_mut_ptr().offset((*metric).type_0 as isize)).sample)
            .expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(metric, cb, backend);
}
pub unsafe extern "C" fn brubeck_metric_record(
    mut metric: *mut brubeck_metric,
    mut value: value_t,
    mut sample_freq: value_t,
    mut modifiers: uint8_t,
) {
    brubeck_metric_set_state(metric, 2 as libc::c_int as uint8_t);
    (Some(
        ((*_prototypes.as_mut_ptr().offset((*metric).type_0 as isize)).record)
            .expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(metric, value, sample_freq, modifiers);
}
pub unsafe extern "C" fn brubeck_metric_shard(
    mut server: *mut brubeck_server,
    mut metric: *mut brubeck_metric,
) -> *mut brubeck_backend {
    let mut shard: libc::c_int = 0;
    let mut tmp: uint32_t = 0;
    shard = 0 as libc::c_int;
    if (*server).active_backends > 1 as libc::c_int {
        tmp = CityHash32(
            ((*metric).key).as_mut_ptr() as *const libc::c_char,
            (*metric).key_len as size_t,
        );
        shard = tmp.wrapping_rem((*server).active_backends as libc::c_uint)
            as libc::c_int;
    }
    return (*server).backends[shard as usize];
}
pub unsafe extern "C" fn brubeck_metric_new(
    mut server: *mut brubeck_server,
    mut key: *const libc::c_char,
    mut key_len: size_t,
    mut type_0: uint8_t,
) -> *mut brubeck_metric {
    let mut metric: *mut brubeck_metric = 0 as *mut brubeck_metric;
    let mut key_for_ht: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut brubeck_metric = 0 as *mut brubeck_metric;
    let mut tmp___1: bool = false;
    let mut tmp___2: *mut brubeck_backend = 0 as *mut brubeck_backend;
    tmp = strndup(key, key_len);
    key_for_ht = tmp;
    metric = new_metric(server, key, key_len, type_0);
    if metric.is_null() {
        return 0 as *mut libc::c_void as *mut brubeck_metric;
    }
    tmp___1 = brubeck_hashtable_insert(
        (*server).metrics,
        key_for_ht as *const libc::c_char,
        key_len as uint16_t,
        metric,
    );
    if !tmp___1 {
        free(key_for_ht as *mut libc::c_void);
        tmp___0 = brubeck_hashtable_find((*server).metrics, key, key_len as uint16_t);
        return tmp___0;
    }
    tmp___2 = brubeck_metric_shard(server, metric);
    brubeck_backend_register_metric(tmp___2, metric);
    let fresh9 = &mut (*server).internal_stats.live.unique_keys;
    let fresh10 = 1 as libc::c_int as uint32_t;
    ::std::intrinsics::atomic_xadd_seqcst(fresh9, fresh10) + fresh10;
    return metric;
}
pub unsafe extern "C" fn brubeck_metric_find(
    mut server: *mut brubeck_server,
    mut key: *const libc::c_char,
    mut key_len: size_t,
    mut type_0: uint8_t,
) -> *mut brubeck_metric {
    let mut metric: *mut brubeck_metric = 0 as *mut brubeck_metric;
    let mut tmp: *mut brubeck_metric = 0 as *mut brubeck_metric;
    let mut tmp___0: libc::c_long = 0;
    metric = brubeck_hashtable_find((*server).metrics, key, key_len as uint16_t);
    tmp___0 = (metric as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong)
        as libc::c_int as libc::c_long;
    if tmp___0 != 0 {
        if (*server).at_capacity != 0 {
            return 0 as *mut libc::c_void as *mut brubeck_metric;
        }
        tmp = brubeck_metric_new(server, key, key_len, type_0);
        return tmp;
    }
    return metric;
}
#[inline]
unsafe extern "C" fn brubeck_sampler_name(
    mut sampler: *mut brubeck_sampler,
) -> *const libc::c_char {
    match (*sampler).type_0 as libc::c_uint {
        0 => return b"statsd\0" as *const u8 as *const libc::c_char,
        _ => return 0 as *mut libc::c_void as *const libc::c_char,
    };
}
pub unsafe extern "C" fn brubeck_sampler_init_inet(
    mut sampler: *mut brubeck_sampler,
    mut server: *mut brubeck_server,
    mut url: *const libc::c_char,
    mut port: libc::c_int,
) {
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    (*sampler).server = server;
    url_to_inaddr2(&mut (*sampler).addr, url, port);
    tmp = brubeck_sampler_name(sampler);
    tmp___0 = gh_log_instance();
    gh_log_write(
        b"instance=%s sampler=%s event=load_udp addr=0.0.0.0:%d\n\0" as *const u8
            as *const libc::c_char,
        tmp___0,
        tmp,
        port,
    );
}
pub unsafe extern "C" fn brubeck_sampler_socket(
    mut sampler: *mut brubeck_sampler,
    mut multisock: libc::c_int,
) -> libc::c_int {
    let mut sock: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = socket(2 as libc::c_int, 2 as libc::c_int, 17 as libc::c_int);
    sock = tmp;
    sock_enlarge_in(sock);
    sock_setreuse(sock, 1 as libc::c_int);
    if multisock != 0 {
        sock_setreuse_port(sock, 1 as libc::c_int);
    }
    tmp___0 = bind(
        sock,
        &mut (*sampler).addr as *mut sockaddr_in as *mut sockaddr as *const sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    );
    if tmp___0 < 0 as libc::c_int {
        fprintf(
            stderr,
            b"[FATAL]: failed to bind socket\n\0" as *const u8 as *const libc::c_char,
        );
        gh_log_die();
    }
    return sock;
}
unsafe extern "C" fn statsd_run_recvmmsg(
    mut statsd: *mut brubeck_statsd,
    mut sock: libc::c_int,
) {
    let mut SIM_PACKETS: libc::c_uint = 0;
    let mut server: *mut brubeck_server = 0 as *mut brubeck_server;
    let mut i: libc::c_uint = 0;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut res: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___6: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    SIM_PACKETS = (*statsd).mmsg_count;
    server = (*statsd).sampler.server;
    let vla = SIM_PACKETS as usize;
    let mut iovecs: Vec::<iovec> = ::std::vec::from_elem(
        iovec {
            iov_base: 0 as *mut libc::c_void,
            iov_len: 0,
        },
        vla,
    );
    let vla_0 = SIM_PACKETS as usize;
    let mut msgs: Vec::<mmsghdr> = ::std::vec::from_elem(
        mmsghdr {
            msg_hdr: msghdr {
                msg_name: 0 as *mut libc::c_void,
                msg_namelen: 0,
                msg_iov: 0 as *mut iovec,
                msg_iovlen: 0,
                msg_control: 0 as *mut libc::c_void,
                msg_controllen: 0,
                msg_flags: 0,
            },
            msg_len: 0,
        },
        vla_0,
    );
    memset(
        msgs.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (vla_0 * ::std::mem::size_of::<mmsghdr>()) as libc::c_ulong,
    );
    i = 0 as libc::c_uint;
    while i < SIM_PACKETS {
        let ref mut fresh11 = (*iovecs.as_mut_ptr().offset(i as isize)).iov_base;
        *fresh11 = xmalloc(8192 as libc::c_int as size_t);
        (*iovecs.as_mut_ptr().offset(i as isize))
            .iov_len = 8191 as libc::c_int as size_t;
        let ref mut fresh12 = (*msgs.as_mut_ptr().offset(i as isize)).msg_hdr.msg_iov;
        *fresh12 = &mut *iovecs.as_mut_ptr().offset(i as isize) as *mut iovec;
        (*msgs.as_mut_ptr().offset(i as isize))
            .msg_hdr
            .msg_iovlen = 1 as libc::c_int as size_t;
        i = i.wrapping_add(1);
    }
    tmp = gh_log_instance();
    gh_log_write(
        b"instance=%s sampler=statsd event=worker_online syscall=recvmmsg socket=%d\n\0"
            as *const u8 as *const libc::c_char,
        tmp,
        sock,
    );
    loop {
        tmp___0 = recvmmsg(
            sock,
            msgs.as_mut_ptr(),
            SIM_PACKETS,
            65536 as libc::c_int,
            0 as *mut libc::c_void as *mut timespec,
        );
        res = tmp___0;
        if res < 0 as libc::c_int {
            tmp___1 = __errno_location();
            if *tmp___1 == 11 as libc::c_int {
                continue;
            }
            tmp___2 = __errno_location();
            if *tmp___2 == 4 as libc::c_int {
                continue;
            }
            tmp___3 = __errno_location();
            tmp___4 = strerror(*tmp___3);
            tmp___5 = __errno_location();
            tmp___6 = gh_log_instance();
            gh_log_write(
                b"instance=%s sampler=statsd event=failed_read errno=%d msg=\"%s\"\n\0"
                    as *const u8 as *const libc::c_char,
                tmp___6,
                *tmp___5,
                tmp___4,
            );
            let fresh13 = &mut (*server).internal_stats.live.errors;
            let fresh14 = 1 as libc::c_int as uint32_t;
            ::std::intrinsics::atomic_xadd_seqcst(fresh13, fresh14) + fresh14;
        } else {
            let fresh15 = &mut (*statsd).sampler.inflow;
            let fresh16 = res as size_t;
            ::std::intrinsics::atomic_xadd_seqcst(fresh15, fresh16) + fresh16;
            i = 0 as libc::c_uint;
            while i < res as libc::c_uint {
                buf = (*(*msgs.as_mut_ptr().offset(i as isize)).msg_hdr.msg_iov).iov_base
                    as *mut libc::c_char;
                end = buf
                    .offset((*msgs.as_mut_ptr().offset(i as isize)).msg_len as isize);
                brubeck_statsd_packet_parse(server, buf, end, (*statsd).scale_timers_by);
                i = i.wrapping_add(1);
            }
        }
    };
}
unsafe extern "C" fn statsd_run_recvmsg(
    mut statsd: *mut brubeck_statsd,
    mut sock: libc::c_int,
) {
    let mut server: *mut brubeck_server = 0 as *mut brubeck_server;
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut reporter: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut reporter_len: socklen_t = 0;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut res: libc::c_int = 0;
    let mut tmp___1: ssize_t = 0;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___4: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___7: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___8: *const libc::c_char = 0 as *const libc::c_char;
    server = (*statsd).sampler.server;
    tmp = xmalloc(8192 as libc::c_int as size_t);
    buffer = tmp as *mut libc::c_char;
    reporter_len = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    memset(
        &mut reporter as *mut sockaddr_in as *mut libc::c_void,
        0 as libc::c_int,
        reporter_len as size_t,
    );
    tmp___0 = gh_log_instance();
    gh_log_write(
        b"instance=%s sampler=statsd event=worker_online syscall=recvmsg socket=%d\n\0"
            as *const u8 as *const libc::c_char,
        tmp___0,
        sock,
    );
    loop {
        tmp___1 = recvfrom(
            sock,
            buffer as *mut libc::c_void,
            8191 as libc::c_int as size_t,
            0 as libc::c_int,
            &mut reporter as *mut sockaddr_in as *mut sockaddr,
            &mut reporter_len as *mut socklen_t,
        );
        res = tmp___1 as libc::c_int;
        if res < 0 as libc::c_int {
            tmp___2 = __errno_location();
            if *tmp___2 == 11 as libc::c_int {
                continue;
            }
            tmp___3 = __errno_location();
            if *tmp___3 == 4 as libc::c_int {
                continue;
            }
            tmp___4 = __errno_location();
            tmp___5 = strerror(*tmp___4);
            tmp___6 = __errno_location();
            tmp___7 = inet_ntoa(reporter.sin_addr);
            tmp___8 = gh_log_instance();
            gh_log_write(
                b"instance=%s sampler=statsd event=failed_read from=%s errno=%d msg=\"%s\"\n\0"
                    as *const u8 as *const libc::c_char,
                tmp___8,
                tmp___7,
                *tmp___6,
                tmp___5,
            );
            let fresh17 = &mut (*server).internal_stats.live.errors;
            let fresh18 = 1 as libc::c_int as uint32_t;
            ::std::intrinsics::atomic_xadd_seqcst(fresh17, fresh18) + fresh18;
        } else {
            let fresh19 = &mut (*statsd).sampler.inflow;
            let fresh20 = 1 as libc::c_int as size_t;
            ::std::intrinsics::atomic_xadd_seqcst(fresh19, fresh20) + fresh20;
            brubeck_statsd_packet_parse(
                server,
                buffer,
                buffer.offset(res as isize),
                (*statsd).scale_timers_by,
            );
        }
    };
}
#[inline]
unsafe extern "C" fn parse_float(
    mut buffer: *mut libc::c_char,
    mut result: *mut value_t,
    mut mods: *mut uint8_t,
) -> *mut libc::c_char {
    let mut negative: libc::c_int = 0;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: value_t = 0.;
    let mut f: libc::c_double = 0.;
    let mut n: libc::c_int = 0;
    let mut tmp: libc::c_double = 0.;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_long = 0;
    negative = 0 as libc::c_int;
    start = buffer;
    value = 0.0f64;
    if *buffer as libc::c_int == 45 as libc::c_int {
        buffer = buffer.offset(1);
        negative = 1 as libc::c_int;
        *mods = (*mods as libc::c_int | 1 as libc::c_int) as uint8_t;
    } else if *buffer as libc::c_int == 43 as libc::c_int {
        buffer = buffer.offset(1);
        *mods = (*mods as libc::c_int | 1 as libc::c_int) as uint8_t;
    }
    while *buffer as libc::c_int >= 48 as libc::c_int {
        if !(*buffer as libc::c_int <= 57 as libc::c_int) {
            break;
        }
        value = value * 10.0f64
            + (*buffer as libc::c_int - 48 as libc::c_int) as value_t;
        buffer = buffer.offset(1);
    }
    if *buffer as libc::c_int == 46 as libc::c_int {
        f = 0.0f64;
        n = 0 as libc::c_int;
        buffer = buffer.offset(1);
        while *buffer as libc::c_int >= 48 as libc::c_int {
            if !(*buffer as libc::c_int <= 57 as libc::c_int) {
                break;
            }
            f = f * 10.0f64
                + (*buffer as libc::c_int - 48 as libc::c_int) as libc::c_double;
            buffer = buffer.offset(1);
            n += 1;
        }
        tmp = pow(10.0f64, n as libc::c_double);
        value += f / tmp;
    }
    if negative != 0 {
        value = -value;
    }
    if *buffer as libc::c_int == 101 as libc::c_int {
        tmp___0 = 1 as libc::c_int;
    } else if *buffer as libc::c_int == 69 as libc::c_int {
        tmp___0 = 1 as libc::c_int;
    } else {
        tmp___0 = 0 as libc::c_int;
    }
    tmp___1 = tmp___0 as libc::c_long;
    if tmp___1 != 0 {
        value = strtod(
            start as *const libc::c_char,
            &mut buffer as *mut *mut libc::c_char,
        );
    }
    *result = value;
    return buffer;
}
pub unsafe extern "C" fn brubeck_statsd_msg_parse(
    mut msg: *mut brubeck_statsd_msg,
    mut buffer: *mut libc::c_char,
    mut end: *mut libc::c_char,
    scale_timers_by: libc::c_double,
) -> libc::c_int {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sample_rate: libc::c_double = 0.;
    let mut dummy: uint8_t = 0;
    *end = '\u{0}' as i32 as libc::c_char;
    (*msg).key = buffer;
    (*msg).key_len = 0 as libc::c_int as uint16_t;
    while *buffer as libc::c_int != 58 as libc::c_int {
        if !(*buffer as libc::c_int != 0 as libc::c_int) {
            break;
        }
        buffer = buffer.offset(1);
    }
    if *buffer as libc::c_int == 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*msg).key_len = buffer.offset_from((*msg).key) as libc::c_long as uint16_t;
    tmp = buffer;
    buffer = buffer.offset(1);
    *tmp = '\u{0}' as i32 as libc::c_char;
    if *((*msg).key).offset(((*msg).key_len as libc::c_int - 1 as libc::c_int) as isize)
        as libc::c_int == 46 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    (*msg).modifiers = 0 as libc::c_int as uint8_t;
    buffer = parse_float(buffer, &mut (*msg).value, &mut (*msg).modifiers);
    if *buffer as libc::c_int != 124 as libc::c_int {
        return -(1 as libc::c_int);
    }
    buffer = buffer.offset(1);
    let mut current_block_28: u64;
    match *buffer as libc::c_int {
        103 => {
            (*msg).type_0 = 0 as libc::c_int as uint16_t;
            current_block_28 = 7205609094909031804;
        }
        99 => {
            (*msg).type_0 = 1 as libc::c_int as uint16_t;
            current_block_28 = 7205609094909031804;
        }
        67 => {
            (*msg).type_0 = 2 as libc::c_int as uint16_t;
            current_block_28 = 7205609094909031804;
        }
        104 => {
            (*msg).type_0 = 3 as libc::c_int as uint16_t;
            current_block_28 = 7205609094909031804;
        }
        109 => {
            buffer = buffer.offset(1);
            if *buffer as libc::c_int == 115 as libc::c_int {
                (*msg).type_0 = 4 as libc::c_int as uint16_t;
                (*msg).value *= scale_timers_by;
                current_block_28 = 7205609094909031804;
            } else {
                current_block_28 = 3161266064288215281;
            }
        }
        _ => {
            current_block_28 = 3161266064288215281;
        }
    }
    match current_block_28 {
        7205609094909031804 => {}
        _ => return -(1 as libc::c_int),
    }
    buffer = buffer.offset(1);
    if *buffer.offset(0 as libc::c_int as isize) as libc::c_int == 124 as libc::c_int {
        if *buffer.offset(1 as libc::c_int as isize) as libc::c_int == 64 as libc::c_int
        {
            buffer = parse_float(
                buffer.offset(2 as libc::c_int as isize),
                &mut sample_rate,
                &mut dummy,
            );
            if sample_rate <= 0.0f64 {
                return -(1 as libc::c_int)
            } else {
                if sample_rate > 1.0f64 {
                    return -(1 as libc::c_int);
                }
            }
            (*msg).sample_freq = 1.0f64 / sample_rate;
        } else {
            (*msg).sample_freq = 1.0f64;
        }
    } else {
        (*msg).sample_freq = 1.0f64;
    }
    if *buffer.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int
    } else {
        if *buffer.offset(0 as libc::c_int as isize) as libc::c_int == 10 as libc::c_int
        {
            if *buffer.offset(1 as libc::c_int as isize) as libc::c_int
                == 0 as libc::c_int
            {
                return 0 as libc::c_int;
            }
        }
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn brubeck_statsd_packet_parse(
    mut server: *mut brubeck_server,
    mut buffer: *mut libc::c_char,
    mut end: *mut libc::c_char,
    scale_timers_by: libc::c_double,
) {
    let mut msg: brubeck_statsd_msg = brubeck_statsd_msg {
        key: 0 as *mut libc::c_char,
        key_len: 0,
        type_0: 0,
        value: 0.,
        sample_freq: 0.,
        modifiers: 0,
    };
    let mut metric: *mut brubeck_metric = 0 as *mut brubeck_metric;
    let mut stat_end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    while (buffer as libc::c_ulong) < end as libc::c_ulong {
        tmp = memchr(
            buffer as *const libc::c_void,
            '\n' as i32,
            end.offset_from(buffer) as libc::c_long as size_t,
        );
        stat_end = tmp as *mut libc::c_char;
        if stat_end.is_null() {
            stat_end = end;
        }
        tmp___1 = brubeck_statsd_msg_parse(&mut msg, buffer, stat_end, scale_timers_by);
        if tmp___1 < 0 as libc::c_int {
            let fresh21 = &mut (*server).internal_stats.live.errors;
            let fresh22 = 1 as libc::c_int as uint32_t;
            ::std::intrinsics::atomic_xadd_seqcst(fresh21, fresh22) + fresh22;
            tmp___0 = gh_log_instance();
            gh_log_write(
                b"instance=%s sampler=statsd event=packet_drop\n\0" as *const u8
                    as *const libc::c_char,
                tmp___0,
            );
        } else {
            let fresh23 = &mut (*server).internal_stats.live.metrics;
            let fresh24 = 1 as libc::c_int as uint32_t;
            ::std::intrinsics::atomic_xadd_seqcst(fresh23, fresh24) + fresh24;
            metric = brubeck_metric_find(
                server,
                msg.key as *const libc::c_char,
                msg.key_len as size_t,
                msg.type_0 as uint8_t,
            );
            if metric as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                brubeck_metric_record(metric, msg.value, msg.sample_freq, msg.modifiers);
            }
        }
        buffer = stat_end.offset(1 as libc::c_int as isize);
    }
}
unsafe extern "C" fn statsd__thread(mut _in: *mut libc::c_void) -> *mut libc::c_void {
    let mut statsd: *mut brubeck_statsd = 0 as *mut brubeck_statsd;
    let mut sock: libc::c_int = 0;
    statsd = _in as *mut brubeck_statsd;
    sock = (*statsd).sampler.in_sock;
    if sock < 0 as libc::c_int {
        sock = brubeck_sampler_socket(&mut (*statsd).sampler, 1 as libc::c_int);
    }
    if (*statsd).mmsg_count > 1 as libc::c_uint {
        statsd_run_recvmmsg(statsd, sock);
        return 0 as *mut libc::c_void;
    }
    statsd_run_recvmsg(statsd, sock);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn run_worker_threads(mut statsd: *mut brubeck_statsd) {
    let mut i: libc::c_uint = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    tmp = xmalloc(
        ((*statsd).worker_count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<pthread_t>() as libc::c_ulong),
    );
    (*statsd).workers = tmp as *mut pthread_t;
    i = 0 as libc::c_uint;
    while i < (*statsd).worker_count {
        tmp___0 = pthread_create(
            ((*statsd).workers).offset(i as isize),
            0 as *mut libc::c_void as *const pthread_attr_t,
            Some(
                statsd__thread
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            statsd as *mut libc::c_void,
        );
        if tmp___0 != 0 as libc::c_int {
            fprintf(
                stderr,
                b"[FATAL]: failed to start sampler thread\n\0" as *const u8
                    as *const libc::c_char,
            );
            gh_log_die();
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn shutdown_sampler(mut sampler: *mut brubeck_sampler) {
    let mut statsd: *mut brubeck_statsd = 0 as *mut brubeck_statsd;
    let mut i: size_t = 0;
    statsd = sampler as *mut brubeck_statsd;
    i = 0 as libc::c_int as size_t;
    while i < (*statsd).worker_count as size_t {
        pthread_cancel(*((*statsd).workers).offset(i as isize));
        i = i.wrapping_add(1);
    }
}
pub unsafe extern "C" fn brubeck_statsd_new(
    mut server: *mut brubeck_server,
    mut settings: *mut json_t,
) -> *mut brubeck_sampler {
    let mut std: *mut brubeck_statsd = 0 as *mut brubeck_statsd;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut address: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut port: libc::c_int = 0;
    let mut multisock: libc::c_int = 0;
    let mut _error_j: json_error_t = json_error_t {
        line: 0,
        column: 0,
        position: 0,
        source: [0; 80],
        text: [0; 160],
    };
    let mut tmp___0: libc::c_int = 0;
    tmp = xmalloc(::std::mem::size_of::<brubeck_statsd>() as libc::c_ulong);
    std = tmp as *mut brubeck_statsd;
    multisock = 0 as libc::c_int;
    (*std).sampler.type_0 = BRUBECK_SAMPLER_STATSD;
    (*std)
        .sampler
        .shutdown = Some(
        shutdown_sampler as unsafe extern "C" fn(*mut brubeck_sampler) -> (),
    );
    (*std).sampler.in_sock = -(1 as libc::c_int);
    (*std).worker_count = 4 as libc::c_uint;
    (*std).mmsg_count = 1 as libc::c_uint;
    (*std).scale_timers_by = 1.0f64;
    tmp___0 = json_unpack_ex(
        settings,
        &mut _error_j as *mut json_error_t,
        0 as libc::c_int as size_t,
        b"{s:s, s:i, s?:i, s?:i, s?:b, s?:F}\0" as *const u8 as *const libc::c_char,
        b"address\0" as *const u8 as *const libc::c_char,
        &mut address as *mut *mut libc::c_char,
        b"port\0" as *const u8 as *const libc::c_char,
        &mut port as *mut libc::c_int,
        b"workers\0" as *const u8 as *const libc::c_char,
        &mut (*std).worker_count as *mut libc::c_uint,
        b"multimsg\0" as *const u8 as *const libc::c_char,
        &mut (*std).mmsg_count as *mut libc::c_uint,
        b"multisock\0" as *const u8 as *const libc::c_char,
        &mut multisock as *mut libc::c_int,
        b"scale_timers_by\0" as *const u8 as *const libc::c_char,
        &mut (*std).scale_timers_by as *mut libc::c_double,
    );
    if tmp___0 < 0 as libc::c_int {
        fprintf(
            stderr,
            b"[FATAL]: config error: %s\n\0" as *const u8 as *const libc::c_char,
            (_error_j.text).as_mut_ptr(),
        );
        gh_log_die();
    }
    brubeck_sampler_init_inet(
        &mut (*std).sampler,
        server,
        address as *const libc::c_char,
        port,
    );
    if multisock == 0 {
        (*std)
            .sampler
            .in_sock = brubeck_sampler_socket(&mut (*std).sampler, 0 as libc::c_int);
    }
    run_worker_threads(std);
    return &mut (*std).sampler;
}
unsafe extern "C" fn update_flows(mut server: *mut brubeck_server) {
    let mut i: libc::c_int = 0;
    let mut sampler: *mut brubeck_sampler = 0 as *mut brubeck_sampler;
    i = 0 as libc::c_int;
    while i < (*server).active_samplers {
        sampler = (*server).samplers[i as usize];
        (*sampler).current_flow = (*sampler).inflow;
        (*sampler).inflow = 0 as libc::c_int as size_t;
        i += 1;
    }
}
static mut size_suffix: [*const libc::c_char; 7] = [
    b"b\0" as *const u8 as *const libc::c_char,
    b"kb\0" as *const u8 as *const libc::c_char,
    b"mb\0" as *const u8 as *const libc::c_char,
    b"gb\0" as *const u8 as *const libc::c_char,
    b"tb\0" as *const u8 as *const libc::c_char,
    b"pb\0" as *const u8 as *const libc::c_char,
    b"eb\0" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn update_proctitle(mut server: *mut brubeck_server) {
    let mut buf: [libc::c_char; 2048] = [0; 2048];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut bytes_sent: libc::c_double = 0.;
    let mut connected: bool = false;
    let mut backend: *mut brubeck_backend = 0 as *mut brubeck_backend;
    let mut carbon: *mut brubeck_carbon = 0 as *mut brubeck_carbon;
    let mut tmp___0: libc::c_int = 0;
    let mut kafka: *mut brubeck_kafka = 0 as *mut brubeck_kafka;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___3: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut sampler: *mut brubeck_sampler = 0 as *mut brubeck_sampler;
    let mut tmp___6: __uint16_t = 0;
    let mut tmp___7: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_int = 0;
    if (*server).set_proctitle {
        pos = 0 as libc::c_int;
        tmp = snprintf(
            buf.as_mut_ptr().offset(pos as isize),
            (::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
                .wrapping_sub(pos as libc::c_ulong),
            b"[%s] [ \xE2\x86\x91\0" as *const u8 as *const libc::c_char,
            (*server).config_name,
        );
        pos += tmp;
        bytes_sent = 0.0f64;
        connected = 0 as libc::c_int != 0;
        i = 0 as libc::c_int;
        while i < (*server).active_backends {
            backend = (*server).backends[i as usize];
            if (*backend).type_0 as libc::c_uint == 0 as libc::c_uint {
                carbon = backend as *mut brubeck_carbon;
                bytes_sent += (*carbon).bytes_sent as libc::c_double;
                if connected {
                    tmp___0 = 1 as libc::c_int;
                } else if (*carbon).out_sock >= 0 as libc::c_int {
                    tmp___0 = 1 as libc::c_int;
                } else {
                    tmp___0 = 0 as libc::c_int;
                }
                connected = tmp___0 != 0;
            } else if (*backend).type_0 as libc::c_uint == 1 as libc::c_uint {
                kafka = backend as *mut brubeck_kafka;
                bytes_sent += (*kafka).bytes_sent as libc::c_double;
                if connected {
                    tmp___1 = 1 as libc::c_int;
                } else if (*kafka).connected {
                    tmp___1 = 1 as libc::c_int;
                } else {
                    tmp___1 = 0 as libc::c_int;
                }
                connected = tmp___1 != 0;
            }
            i += 1;
        }
        j = 0 as libc::c_int;
        while j < 7 as libc::c_int {
            if !(bytes_sent >= 1024.0f64) {
                break;
            }
            bytes_sent /= 1024.0f64;
            j += 1;
        }
        if connected {
            tmp___2 = b"\0" as *const u8 as *const libc::c_char;
        } else {
            tmp___2 = b" (dc)\0" as *const u8 as *const libc::c_char;
        }
        if i > 0 as libc::c_int {
            tmp___3 = b",\0" as *const u8 as *const libc::c_char;
        } else {
            tmp___3 = b"\0" as *const u8 as *const libc::c_char;
        }
        tmp___4 = snprintf(
            buf.as_mut_ptr().offset(pos as isize),
            (::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
                .wrapping_sub(pos as libc::c_ulong),
            b"%s #%d %.1f%s%s\0" as *const u8 as *const libc::c_char,
            tmp___3,
            i + 1 as libc::c_int,
            bytes_sent,
            size_suffix[j as usize],
            tmp___2,
        );
        pos += tmp___4;
        tmp___5 = snprintf(
            buf.as_mut_ptr().offset(pos as isize),
            (::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
                .wrapping_sub(pos as libc::c_ulong),
            b" ] [ \xE2\x86\x93\0" as *const u8 as *const libc::c_char,
        );
        pos += tmp___5;
        i = 0 as libc::c_int;
        while i < (*server).active_samplers {
            sampler = (*server).samplers[i as usize];
            tmp___6 = __bswap_16((*sampler).addr.sin_port);
            if i > 0 as libc::c_int {
                tmp___7 = b",\0" as *const u8 as *const libc::c_char;
            } else {
                tmp___7 = b"\0" as *const u8 as *const libc::c_char;
            }
            tmp___8 = snprintf(
                buf.as_mut_ptr().offset(pos as isize),
                (::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
                    .wrapping_sub(pos as libc::c_ulong),
                b"%s :%d %d/s\0" as *const u8 as *const libc::c_char,
                tmp___7,
                tmp___6 as libc::c_int,
                (*sampler).current_flow as libc::c_int,
            );
            pos += tmp___8;
            i += 1;
        }
        tmp___9 = snprintf(
            buf.as_mut_ptr().offset(pos as isize),
            (::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
                .wrapping_sub(pos as libc::c_ulong),
            b" ]\0" as *const u8 as *const libc::c_char,
        );
        pos += tmp___9;
        setproctitle(
            b"brubeck\0" as *const u8 as *const libc::c_char,
            buf.as_mut_ptr() as *const libc::c_char,
        );
    }
}
static mut METRIC_NAMES: [*const libc::c_char; 6] = [
    b"g\0" as *const u8 as *const libc::c_char,
    b"c\0" as *const u8 as *const libc::c_char,
    b"C\0" as *const u8 as *const libc::c_char,
    b"h\0" as *const u8 as *const libc::c_char,
    b"ms\0" as *const u8 as *const libc::c_char,
    b"internal\0" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn dump_metric(
    mut mt: *mut brubeck_metric,
    mut out_file: *mut libc::c_void,
) {
    fprintf(
        out_file as *mut FILE,
        b"%s|%s\n\0" as *const u8 as *const libc::c_char,
        ((*mt).key).as_mut_ptr(),
        METRIC_NAMES[(*mt).type_0 as usize],
    );
}
unsafe extern "C" fn dump_all_metrics(mut server: *mut brubeck_server) {
    let mut dump: *mut FILE = 0 as *mut FILE;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: *const libc::c_char = 0 as *const libc::c_char;
    dump = 0 as *mut libc::c_void as *mut FILE;
    tmp = gh_log_instance();
    gh_log_write(
        b"instance=%s event=dump_metrics\n\0" as *const u8 as *const libc::c_char,
        tmp,
    );
    if !((*server).dump_path).is_null() {
        dump = fopen((*server).dump_path, b"w+\0" as *const u8 as *const libc::c_char);
    }
    if dump.is_null() {
        tmp___0 = __errno_location();
        tmp___1 = strerror(*tmp___0);
        tmp___2 = __errno_location();
        tmp___3 = gh_log_instance();
        gh_log_write(
            b"instance=%s event=dump_failed errno=%d msg=\"%s\"\n\0" as *const u8
                as *const libc::c_char,
            tmp___3,
            *tmp___2,
            tmp___1,
        );
        return;
    }
    brubeck_hashtable_foreach(
        (*server).metrics,
        Some(
            dump_metric
                as unsafe extern "C" fn(*mut brubeck_metric, *mut libc::c_void) -> (),
        ),
        dump as *mut libc::c_void,
    );
    fclose(dump);
}
unsafe extern "C" fn load_backends(
    mut server: *mut brubeck_server,
    mut backends: *mut json_t,
) {
    let mut idx: size_t = 0;
    let mut b: *mut json_t = 0 as *mut json_t;
    let mut type_0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *mut json_t = 0 as *mut json_t;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut backend: *mut brubeck_backend = 0 as *mut brubeck_backend;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: size_t = 0;
    idx = 0 as libc::c_int as size_t;
    loop {
        tmp___6 = json_array_size(backends as *const json_t);
        if !(idx < tmp___6) {
            break;
        }
        b = json_array_get(backends as *const json_t, idx);
        if b.is_null() {
            break;
        }
        tmp = json_object_get(
            b as *const json_t,
            b"type\0" as *const u8 as *const libc::c_char,
        );
        tmp___0 = json_string_value(tmp as *const json_t);
        type_0 = tmp___0;
        backend = 0 as *mut libc::c_void as *mut brubeck_backend;
        let mut current_block_27: u64;
        if !type_0.is_null() {
            tmp___5 = strcmp(type_0, b"carbon\0" as *const u8 as *const libc::c_char);
            if tmp___5 != 0 {
                current_block_27 = 7757499455844833761;
            } else {
                backend = brubeck_carbon_new(server, b, (*server).active_backends);
                tmp___1 = (*server).active_backends;
                (*server).active_backends += 1;
                (*server).backends[tmp___1 as usize] = backend;
                current_block_27 = 15897653523371991391;
            }
        } else {
            current_block_27 = 7757499455844833761;
        }
        match current_block_27 {
            7757499455844833761 => {
                if !type_0.is_null() {
                    tmp___4 = strcmp(
                        type_0,
                        b"kafka\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___4 != 0 {
                        tmp___3 = gh_log_instance();
                        gh_log_write(
                            b"instance=%s backend=%s event=invalid_backend\n\0"
                                as *const u8 as *const libc::c_char,
                            tmp___3,
                            type_0,
                        );
                    } else {
                        backend = brubeck_kafka_new(
                            server,
                            b,
                            (*server).active_backends,
                        );
                        tmp___2 = (*server).active_backends;
                        (*server).active_backends += 1;
                        (*server).backends[tmp___2 as usize] = backend;
                    }
                } else {
                    tmp___3 = gh_log_instance();
                    gh_log_write(
                        b"instance=%s backend=%s event=invalid_backend\n\0" as *const u8
                            as *const libc::c_char,
                        tmp___3,
                        type_0,
                    );
                }
            }
            _ => {}
        }
        idx = idx.wrapping_add(1);
    }
    if (*server).active_backends == 0 as libc::c_int {
        fprintf(
            stderr,
            b"[FATAL]: no backends were loaded\n\0" as *const u8 as *const libc::c_char,
        );
        gh_log_die();
    }
}
unsafe extern "C" fn load_samplers(
    mut server: *mut brubeck_server,
    mut samplers: *mut json_t,
) {
    let mut idx: size_t = 0;
    let mut s: *mut json_t = 0 as *mut json_t;
    let mut type_0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *mut json_t = 0 as *mut json_t;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: size_t = 0;
    idx = 0 as libc::c_int as size_t;
    loop {
        tmp___4 = json_array_size(samplers as *const json_t);
        if !(idx < tmp___4) {
            break;
        }
        s = json_array_get(samplers as *const json_t, idx);
        if s.is_null() {
            break;
        }
        tmp = json_object_get(
            s as *const json_t,
            b"type\0" as *const u8 as *const libc::c_char,
        );
        tmp___0 = json_string_value(tmp as *const json_t);
        type_0 = tmp___0;
        if !type_0.is_null() {
            tmp___3 = strcmp(type_0, b"statsd\0" as *const u8 as *const libc::c_char);
            if tmp___3 != 0 {
                tmp___2 = gh_log_instance();
                gh_log_write(
                    b"instance=%s sampler=%s event=invalid_sampler\n\0" as *const u8
                        as *const libc::c_char,
                    tmp___2,
                    type_0,
                );
            } else {
                tmp___1 = (*server).active_samplers;
                (*server).active_samplers += 1;
                (*server).samplers[tmp___1 as usize] = brubeck_statsd_new(server, s);
            }
        } else {
            tmp___2 = gh_log_instance();
            gh_log_write(
                b"instance=%s sampler=%s event=invalid_sampler\n\0" as *const u8
                    as *const libc::c_char,
                tmp___2,
                type_0,
            );
        }
        idx = idx.wrapping_add(1);
    };
}
unsafe extern "C" fn load_timerfd(mut interval: libc::c_int) -> libc::c_int {
    let mut timer: itimerspec = itimerspec {
        it_interval: timespec { tv_sec: 0, tv_nsec: 0 },
        it_value: timespec { tv_sec: 0, tv_nsec: 0 },
    };
    let mut timerfd: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = timerfd_create(1 as libc::c_int, 0 as libc::c_int);
    timerfd = tmp;
    if timerfd < 0 as libc::c_int {
        fprintf(
            stderr,
            b"[FATAL]: failed to create timer\n\0" as *const u8 as *const libc::c_char,
        );
        gh_log_die();
    }
    memset(
        &mut timer as *mut itimerspec as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<itimerspec>() as libc::c_ulong,
    );
    timer.it_value.tv_sec = interval as __time_t;
    timer.it_value.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
    timer.it_interval.tv_sec = interval as __time_t;
    timer.it_interval.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
    tmp___0 = timerfd_settime(
        timerfd,
        0 as libc::c_int,
        &mut timer as *mut itimerspec as *const itimerspec,
        0 as *mut libc::c_void as *mut itimerspec,
    );
    if tmp___0 < 0 as libc::c_int {
        fprintf(
            stderr,
            b"[FATAL]: failed to set system timer\n\0" as *const u8
                as *const libc::c_char,
        );
        gh_log_die();
    }
    return timerfd;
}
unsafe extern "C" fn load_signalfd() -> libc::c_int {
    let mut mask: sigset_t = sigset_t { __val: [0; 16] };
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    sigemptyset(&mut mask);
    sigaddset(&mut mask, 2 as libc::c_int);
    sigaddset(&mut mask, 15 as libc::c_int);
    sigaddset(&mut mask, 1 as libc::c_int);
    sigaddset(&mut mask, 12 as libc::c_int);
    tmp = sigprocmask(
        0 as libc::c_int,
        &mut mask as *mut sigset_t as *const sigset_t,
        0 as *mut libc::c_void as *mut sigset_t,
    );
    if tmp == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"[FATAL]: failed to sigprocmask the needed signals\n\0" as *const u8
                as *const libc::c_char,
        );
        gh_log_die();
    }
    tmp___0 = signalfd(
        -(1 as libc::c_int),
        &mut mask as *mut sigset_t as *const sigset_t,
        0 as libc::c_int,
    );
    return tmp___0;
}
unsafe extern "C" fn get_config_name(
    mut full_path: *const libc::c_char,
) -> *mut libc::c_char {
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut config_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ext: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = strrchr(full_path, '/' as i32);
    filename = tmp as *const libc::c_char;
    if !filename.is_null() {
        tmp___0 = filename.offset(1 as libc::c_int as isize);
    } else {
        tmp___0 = full_path;
    }
    tmp___1 = strdup(tmp___0);
    config_name = tmp___1;
    tmp___2 = strrchr(config_name as *const libc::c_char, '.' as i32);
    ext = tmp___2;
    if !ext.is_null() {
        *ext = '\u{0}' as i32 as libc::c_char;
    }
    return config_name;
}
unsafe extern "C" fn load_config(
    mut server: *mut brubeck_server,
    mut path: *const libc::c_char,
) {
    let mut error: json_error_t = json_error_t {
        line: 0,
        column: 0,
        position: 0,
        source: [0; 80],
        text: [0; 160],
    };
    let mut capacity: libc::c_int = 0;
    let mut backends: *mut json_t = 0 as *mut json_t;
    let mut samplers: *mut json_t = 0 as *mut json_t;
    let mut http: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tag_capacity: libc::c_int = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut _error_j: json_error_t = json_error_t {
        line: 0,
        column: 0,
        position: 0,
        source: [0; 80],
        text: [0; 160],
    };
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    http = 0 as *mut libc::c_void as *mut libc::c_char;
    tag_capacity = 0 as libc::c_int;
    (*server).name = b"brubeck\0" as *const u8 as *const libc::c_char;
    tmp = get_config_name(path);
    (*server).config_name = tmp as *const libc::c_char;
    (*server).dump_path = 0 as *mut libc::c_void as *const libc::c_char;
    (*server).config = json_load_file(path, 0 as libc::c_int as size_t, &mut error);
    if ((*server).config).is_null() {
        fprintf(
            stderr,
            b"[FATAL]: failed to load config file, %s (%s:%d:%d)\n\0" as *const u8
                as *const libc::c_char,
            (error.text).as_mut_ptr(),
            (error.source).as_mut_ptr(),
            error.line,
            error.column,
        );
        gh_log_die();
    }
    tmp___0 = json_unpack_ex(
        (*server).config,
        &mut _error_j as *mut json_error_t,
        0 as libc::c_int as size_t,
        b"{s?:s, s:s, s:i, s?:i, s:o, s:o, s?:s}\0" as *const u8 as *const libc::c_char,
        b"server_name\0" as *const u8 as *const libc::c_char,
        &mut (*server).name as *mut *const libc::c_char,
        b"dumpfile\0" as *const u8 as *const libc::c_char,
        &mut (*server).dump_path as *mut *const libc::c_char,
        b"capacity\0" as *const u8 as *const libc::c_char,
        &mut capacity as *mut libc::c_int,
        b"tag_capacity\0" as *const u8 as *const libc::c_char,
        &mut tag_capacity as *mut libc::c_int,
        b"backends\0" as *const u8 as *const libc::c_char,
        &mut backends as *mut *mut json_t,
        b"samplers\0" as *const u8 as *const libc::c_char,
        &mut samplers as *mut *mut json_t,
        b"http\0" as *const u8 as *const libc::c_char,
        &mut http as *mut *mut libc::c_char,
    );
    if tmp___0 < 0 as libc::c_int {
        fprintf(
            stderr,
            b"[FATAL]: config error: %s\n\0" as *const u8 as *const libc::c_char,
            (_error_j.text).as_mut_ptr(),
        );
        gh_log_die();
    }
    gh_log_set_instance((*server).name);
    (*server)
        .metrics = brubeck_hashtable_new(((1 as libc::c_int) << capacity) as uint64_t);
    if ((*server).metrics).is_null() {
        fprintf(
            stderr,
            b"[FATAL]: failed to initialize hash table (size: %lu)\n\0" as *const u8
                as *const libc::c_char,
            (1 as libc::c_ulong) << capacity,
        );
        gh_log_die();
    }
    if tag_capacity != 0 {
        (*server)
            .tags = brubeck_tags_create(
            ((1 as libc::c_int) << tag_capacity) as uint64_t,
        );
        if ((*server).tags).is_null() {
            fprintf(
                stderr,
                b"[FATAL]: failed to initialize tags (size: %lu)\n\0" as *const u8
                    as *const libc::c_char,
                (1 as libc::c_ulong) << tag_capacity,
            );
            gh_log_die();
        }
        tmp___1 = gh_log_instance();
        gh_log_write(
            b"instance=%s event=tagging_initialized\n\0" as *const u8
                as *const libc::c_char,
            tmp___1,
        );
    }
    load_backends(server, backends);
    load_samplers(server, samplers);
    if !http.is_null() {
        brubeck_http_endpoint_init(server, http as *const libc::c_char);
    }
}
pub unsafe extern "C" fn brubeck_server_init(
    mut server: *mut brubeck_server,
    mut config: *const libc::c_char,
) {
    signal(
        13 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(1 as libc::c_int as libc::intptr_t),
    );
    (*server).fd_signal = load_signalfd();
    (*server).fd_update = load_timerfd(1 as libc::c_int);
    brubeck_slab_init(&mut (*server).slab);
    load_config(server, config);
    brubeck_internal__init(server);
}
unsafe extern "C" fn timer_elapsed(mut fd: *mut pollfd) -> libc::c_int {
    let mut timer: uint64_t = 0;
    let mut s: libc::c_int = 0;
    let mut tmp: ssize_t = 0;
    if (*fd).revents as libc::c_int & 1 as libc::c_int != 0 {
        tmp = read(
            (*fd).fd,
            &mut timer as *mut uint64_t as *mut libc::c_void,
            ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
        );
        s = tmp as libc::c_int;
        return (s as libc::c_ulong == ::std::mem::size_of::<uint64_t>() as libc::c_ulong)
            as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn signal_triggered(mut fd: *mut pollfd) -> libc::c_int {
    let mut fdsi: signalfd_siginfo = signalfd_siginfo {
        ssi_signo: 0,
        ssi_errno: 0,
        ssi_code: 0,
        ssi_pid: 0,
        ssi_uid: 0,
        ssi_fd: 0,
        ssi_tid: 0,
        ssi_band: 0,
        ssi_overrun: 0,
        ssi_trapno: 0,
        ssi_status: 0,
        ssi_int: 0,
        ssi_ptr: 0,
        ssi_utime: 0,
        ssi_stime: 0,
        ssi_addr: 0,
        ssi_addr_lsb: 0,
        __pad2: 0,
        ssi_syscall: 0,
        ssi_call_addr: 0,
        ssi_arch: 0,
        __pad: [0; 28],
    };
    let mut s: libc::c_int = 0;
    let mut tmp: ssize_t = 0;
    if (*fd).revents as libc::c_int & 1 as libc::c_int != 0 {
        tmp = read(
            (*fd).fd,
            &mut fdsi as *mut signalfd_siginfo as *mut libc::c_void,
            ::std::mem::size_of::<signalfd_siginfo>() as libc::c_ulong,
        );
        s = tmp as libc::c_int;
        if s as libc::c_ulong
            == ::std::mem::size_of::<signalfd_siginfo>() as libc::c_ulong
        {
            return fdsi.ssi_signo as libc::c_int;
        }
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn brubeck_server_run(
    mut server: *mut brubeck_server,
) -> libc::c_int {
    let mut fds: [pollfd; 2] = [pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    }; 2];
    let mut nfd: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___3: libc::c_int = 0;
    let mut sampler: *mut brubeck_sampler = 0 as *mut brubeck_sampler;
    let mut tmp___4: *const libc::c_char = 0 as *const libc::c_char;
    nfd = 2 as libc::c_int;
    memset(
        fds.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[pollfd; 2]>() as libc::c_ulong,
    );
    fds[0 as libc::c_int as usize].fd = (*server).fd_signal;
    fds[0 as libc::c_int as usize].events = 1 as libc::c_int as libc::c_short;
    fds[1 as libc::c_int as usize].fd = (*server).fd_update;
    fds[1 as libc::c_int as usize].events = 1 as libc::c_int as libc::c_short;
    (*server).running = 1 as libc::c_int;
    tmp = gh_log_instance();
    gh_log_write(
        b"instance=%s event=listening\n\0" as *const u8 as *const libc::c_char,
        tmp,
    );
    while (*server).running != 0 {
        tmp___0 = poll(fds.as_mut_ptr(), nfd as nfds_t, -(1 as libc::c_int));
        if tmp___0 < 0 as libc::c_int {
            continue;
        }
        tmp___1 = signal_triggered(
            &mut *fds.as_mut_ptr().offset(0 as libc::c_int as isize),
        );
        match tmp___1 {
            1 => {
                gh_log_reopen();
                tmp___2 = gh_log_instance();
                gh_log_write(
                    b"instance=%s event=reload_log\n\0" as *const u8
                        as *const libc::c_char,
                    tmp___2,
                );
            }
            12 => {
                dump_all_metrics(server);
            }
            15 | 2 => {
                (*server).running = 0 as libc::c_int;
            }
            _ => {}
        }
        tmp___3 = timer_elapsed(
            &mut *fds.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
        if tmp___3 != 0 {
            update_flows(server);
            update_proctitle(server);
        }
    }
    i = 0 as libc::c_int as size_t;
    while i < (*server).active_backends as size_t {
        pthread_cancel((*(*server).backends[i as usize]).thread);
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as size_t;
    while i < (*server).active_samplers as size_t {
        sampler = (*server).samplers[i as usize];
        if ((*sampler).shutdown).is_some() {
            (Some(((*sampler).shutdown).expect("non-null function pointer")))
                .expect("non-null function pointer")(sampler);
        }
        i = i.wrapping_add(1);
    }
    tmp___4 = gh_log_instance();
    gh_log_write(
        b"instance=%s event=shutdown\n\0" as *const u8 as *const libc::c_char,
        tmp___4,
    );
    return 0 as libc::c_int;
}
static mut argv0: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut argv_lth: libc::c_int = 0;
pub unsafe extern "C" fn initproctitle(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut envp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: size_t = 0;
    envp = environ;
    i = 0 as libc::c_int;
    while *envp.offset(i as isize) as libc::c_ulong
        != 0 as *mut libc::c_void as libc::c_ulong
    {
        i += 1;
    }
    tmp = malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul((i + 1 as libc::c_int) as libc::c_ulong),
    );
    environ = tmp as *mut *mut libc::c_char;
    if environ as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    i = 0 as libc::c_int;
    while *envp.offset(i as isize) as libc::c_ulong
        != 0 as *mut libc::c_void as libc::c_ulong
    {
        tmp___0 = strdup(*envp.offset(i as isize) as *const libc::c_char);
        let ref mut fresh25 = *environ.offset(i as isize);
        *fresh25 = tmp___0;
        if tmp___0 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            return;
        }
        i += 1;
    }
    let ref mut fresh26 = *environ.offset(i as isize);
    *fresh26 = 0 as *mut libc::c_void as *mut libc::c_char;
    argv0 = argv;
    if i > 0 as libc::c_int {
        tmp___1 = strlen(
            *envp.offset((i - 1 as libc::c_int) as isize) as *const libc::c_char,
        );
        argv_lth = (*envp.offset((i - 1 as libc::c_int) as isize))
            .offset(tmp___1 as isize)
            .offset_from(*argv0.offset(0 as libc::c_int as isize)) as libc::c_long
            as libc::c_int;
    } else {
        tmp___2 = strlen(
            *argv0.offset((argc - 1 as libc::c_int) as isize) as *const libc::c_char,
        );
        argv_lth = (*argv0.offset((argc - 1 as libc::c_int) as isize))
            .offset(tmp___2 as isize)
            .offset_from(*argv0.offset(0 as libc::c_int as isize)) as libc::c_long
            as libc::c_int;
    };
}
pub unsafe extern "C" fn getproctitle(
    mut procbuffer: *mut *mut libc::c_char,
) -> libc::c_int {
    if argv0.is_null() {
        return -(1 as libc::c_int);
    }
    memset(
        *argv0.offset(0 as libc::c_int as isize) as *mut libc::c_void,
        '\u{0}' as i32,
        argv_lth as size_t,
    );
    let ref mut fresh27 = *argv0.offset(1 as libc::c_int as isize);
    *fresh27 = 0 as *mut libc::c_void as *mut libc::c_char;
    *procbuffer = *argv0.offset(0 as libc::c_int as isize);
    return argv_lth;
}
pub unsafe extern "C" fn setproctitle(
    mut prog: *const libc::c_char,
    mut txt: *const libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 2048] = [0; 2048];
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: size_t = 0;
    if argv0.is_null() {
        return;
    }
    tmp = strlen(prog);
    tmp___0 = strlen(txt);
    if tmp.wrapping_add(tmp___0).wrapping_add(5 as libc::c_ulong) > 2048 as libc::c_ulong
    {
        return;
    }
    sprintf(
        buf.as_mut_ptr(),
        b"%s -- %s\0" as *const u8 as *const libc::c_char,
        prog,
        txt,
    );
    tmp___1 = strlen(buf.as_mut_ptr() as *const libc::c_char);
    i = tmp___1 as libc::c_int;
    if i > argv_lth - 2 as libc::c_int {
        i = argv_lth - 2 as libc::c_int;
        buf[i as usize] = '\u{0}' as i32 as libc::c_char;
    }
    memset(
        *argv0.offset(0 as libc::c_int as isize) as *mut libc::c_void,
        '\u{0}' as i32,
        argv_lth as size_t,
    );
    strcpy(
        *argv0.offset(0 as libc::c_int as isize),
        buf.as_mut_ptr() as *const libc::c_char,
    );
    let ref mut fresh28 = *argv0.offset(1 as libc::c_int as isize);
    *fresh28 = 0 as *mut libc::c_void as *mut libc::c_char;
}
unsafe extern "C" fn push_node(mut slab: *mut brubeck_slab) -> *mut brubeck_slab_node {
    let mut node: *mut brubeck_slab_node = 0 as *mut brubeck_slab_node;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = xmalloc(4096 as libc::c_int as size_t);
    node = tmp as *mut brubeck_slab_node;
    (*node).alloc = 0 as libc::c_int as size_t;
    (*node).next = (*slab).current;
    (*slab).current = node;
    return node;
}
pub unsafe extern "C" fn brubeck_slab_alloc(
    mut slab: *mut brubeck_slab,
    mut need: size_t,
) -> *mut libc::c_void {
    let mut node: *mut brubeck_slab_node = 0 as *mut brubeck_slab_node;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    need = need.wrapping_add(32 as libc::c_ulong).wrapping_sub(1 as libc::c_ulong)
        & 0xffffffffffffffe0 as libc::c_ulong;
    pthread_mutex_lock(&mut (*slab).lock);
    node = (*slab).current;
    if ((*node).alloc).wrapping_add(need) > 4064 as libc::c_ulong {
        node = push_node(slab);
    }
    (*slab)
        .total_alloc = ((*slab).total_alloc as libc::c_ulong).wrapping_add(need)
        as size_t as size_t;
    ptr = ((*node).heap).as_mut_ptr().offset((*node).alloc as isize)
        as *mut libc::c_void;
    (*node)
        .alloc = ((*node).alloc as libc::c_ulong).wrapping_add(need) as size_t as size_t;
    pthread_mutex_unlock(&mut (*slab).lock);
    return ptr;
}
pub unsafe extern "C" fn brubeck_slab_init(mut slab: *mut brubeck_slab) {
    push_node(slab);
    pthread_mutex_init(
        &mut (*slab).lock,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
}
static mut char_assoc: libc::c_char = '=' as i32 as libc::c_char;
static mut str_assoc: *const libc::c_char = b"=\0" as *const u8 as *const libc::c_char;
static mut str_delim: *const libc::c_char = b",\0" as *const u8 as *const libc::c_char;
unsafe extern "C" fn tags_malloc(mut r: size_t) -> *mut libc::c_void {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = xmalloc(r);
    return tmp;
}
unsafe extern "C" fn tags_free(mut p: *mut libc::c_void, mut b: size_t, mut r: bool) {
    free(p);
}
static mut ALLOCATOR___0: ck_malloc = {
    let mut init = ck_malloc {
        malloc: Some(tags_malloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        realloc: None,
        free: Some(
            tags_free as unsafe extern "C" fn(*mut libc::c_void, size_t, bool) -> (),
        ),
    };
    init
};
pub unsafe extern "C" fn brubeck_tags_create(size: uint64_t) -> *mut brubeck_tags_t {
    let mut tags: *mut brubeck_tags_t = 0 as *mut brubeck_tags_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: bool = false;
    tmp = xmalloc(::std::mem::size_of::<brubeck_tags_t>() as libc::c_ulong);
    tags = tmp as *mut brubeck_tags_t;
    memset(
        tags as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<brubeck_tags_t>() as libc::c_ulong,
    );
    pthread_mutex_init(
        &mut (*tags).write_mutex,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
    tmp___0 = ck_ht_init(
        &mut (*tags).table,
        2 as libc::c_uint,
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<ck_ht_hash_cb_t>,
        >(0 as *mut libc::c_void),
        &mut ALLOCATOR___0,
        size,
        195948557 as libc::c_int as uint64_t,
    );
    if !tmp___0 {
        free(tags as *mut libc::c_void);
        return 0 as *mut libc::c_void as *mut brubeck_tags_t;
    }
    return tags;
}
pub unsafe extern "C" fn brubeck_tags_insert(
    mut tags: *mut brubeck_tags_t,
    mut key: *const libc::c_char,
    mut key_len: uint16_t,
    mut val: *mut brubeck_tag_set,
) -> bool {
    let mut h: ck_ht_hash_t = ck_ht_hash_t { value: 0 };
    let mut entry: ck_ht_entry_t = ck_ht_entry_t {
        key: 0,
        value: 0,
        key_length: 0,
        hash: 0,
    };
    let mut result: bool = false;
    ck_ht_hash(&mut h, &mut (*tags).table, key as *const libc::c_void, key_len);
    ck_ht_entry_set(
        &mut entry,
        h,
        key as *const libc::c_void,
        key_len,
        val as *const libc::c_void,
    );
    pthread_mutex_lock(&mut (*tags).write_mutex);
    result = ck_ht_put_spmc(&mut (*tags).table, h, &mut entry);
    if result {
        (*tags).num_tag_sets = ((*tags).num_tag_sets).wrapping_add(1);
        (*val).index = (*tags).num_tag_sets;
    }
    pthread_mutex_unlock(&mut (*tags).write_mutex);
    return result;
}
pub unsafe extern "C" fn brubeck_tags_find(
    mut tags: *mut brubeck_tags_t,
    mut key: *const libc::c_char,
    mut key_len: uint16_t,
) -> *mut brubeck_tag_set {
    let mut h: ck_ht_hash_t = ck_ht_hash_t { value: 0 };
    let mut entry: ck_ht_entry_t = ck_ht_entry_t {
        key: 0,
        value: 0,
        key_length: 0,
        hash: 0,
    };
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: bool = false;
    ck_ht_hash(&mut h, &mut (*tags).table, key as *const libc::c_void, key_len);
    ck_ht_entry_key_set(&mut entry, key as *const libc::c_void, key_len);
    tmp___0 = ck_ht_get_spmc(&mut (*tags).table, h, &mut entry);
    if tmp___0 {
        tmp = ck_ht_entry_value(&mut entry);
        return tmp as *mut brubeck_tag_set;
    }
    return 0 as *mut libc::c_void as *mut brubeck_tag_set;
}
pub unsafe extern "C" fn brubeck_tag_offset(mut str: *const libc::c_char) -> uint16_t {
    let mut offset: uint16_t = 0;
    offset = 0 as libc::c_int as uint16_t;
    offset = 0 as libc::c_int as uint16_t;
    while *str != 0 {
        if !(*str as libc::c_int != 44 as libc::c_int) {
            break;
        }
        if !(*str as libc::c_int != 35 as libc::c_int) {
            break;
        }
        str = str.offset(1);
        offset = (offset as libc::c_int + 1 as libc::c_int) as uint16_t;
    }
    return offset;
}
pub unsafe extern "C" fn count_char_in_str(
    mut str: *const libc::c_char,
    mut c: libc::c_char,
) -> uint16_t {
    let mut count: uint16_t = 0;
    count = 0 as libc::c_int as uint16_t;
    while *str.offset(count as libc::c_int as isize) != 0 {
        if *str.offset(count as libc::c_int as isize) as libc::c_int == c as libc::c_int
        {
            count = (count as libc::c_int + 1 as libc::c_int) as uint16_t;
        } else {
            c = (c as libc::c_int + 1 as libc::c_int) as libc::c_char;
        }
    }
    return count;
}
pub unsafe extern "C" fn parse_tag(
    mut kv_str: *mut libc::c_char,
    mut tag: *mut brubeck_tag,
) -> bool {
    let mut state: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    key = strtok_r(kv_str, str_assoc, &mut state as *mut *mut libc::c_char);
    if key as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as libc::c_int != 0;
    }
    value = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        str_assoc,
        &mut state as *mut *mut libc::c_char,
    );
    if value as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as libc::c_int != 0;
    }
    (*tag).key = key as *const libc::c_char;
    (*tag).value = value as *const libc::c_char;
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn brubeck_parse_tags(
    mut tag_str: *mut libc::c_char,
    mut tag_str_len: uint16_t,
) -> *mut brubeck_tag_set {
    let mut state: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut num_possible_tags: uint16_t = 0;
    let mut tmp: uint16_t = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut alloc_size: size_t = 0;
    let mut tag_set: *mut brubeck_tag_set = 0 as *mut brubeck_tag_set;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: bool = false;
    if !tag_str.is_null() {
        tmp = count_char_in_str(tag_str as *const libc::c_char, char_assoc);
        tmp___0 = tmp as libc::c_int;
    } else {
        tmp___0 = 0 as libc::c_int;
    }
    num_possible_tags = tmp___0 as uint16_t;
    alloc_size = (::std::mem::size_of::<brubeck_tag_set>() as libc::c_ulong)
        .wrapping_add(
            (num_possible_tags as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut brubeck_tag>() as libc::c_ulong),
        );
    tmp___1 = malloc(alloc_size);
    tag_set = tmp___1 as *mut brubeck_tag_set;
    memset(tag_set as *mut libc::c_void, 0 as libc::c_int, alloc_size);
    if !tag_str.is_null() {
        (*tag_set).tag_len = tag_str_len;
        tag_str = strtok_r(tag_str, str_delim, &mut state as *mut *mut libc::c_char);
        while !tag_str.is_null() {
            tmp___2 = parse_tag(
                tag_str,
                &mut *((*tag_set).tags).as_mut_ptr().offset((*tag_set).num_tags as isize),
            );
            if tmp___2 {
                (*tag_set)
                    .num_tags = ((*tag_set).num_tags as libc::c_int + 1 as libc::c_int)
                    as uint16_t;
            }
            tag_str = strtok_r(
                0 as *mut libc::c_void as *mut libc::c_char,
                str_delim,
                &mut state as *mut *mut libc::c_char,
            );
        }
    }
    return tag_set;
}
pub unsafe extern "C" fn brubeck_get_tag_set_of_tag_str(
    mut tags: *mut brubeck_tags_t,
    mut tag_str: *const libc::c_char,
    mut tag_str_len: uint16_t,
) -> *const brubeck_tag_set {
    let mut tag_set: *mut brubeck_tag_set = 0 as *mut brubeck_tag_set;
    let mut tag_str_for_parse: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tag_str_for_key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: bool = false;
    tag_set = brubeck_tags_find(tags, tag_str, tag_str_len);
    if tag_set as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp = strdup(tag_str);
        tag_str_for_parse = tmp;
        tmp___0 = strdup(tag_str);
        tag_str_for_key = tmp___0;
        tag_set = brubeck_parse_tags(tag_str_for_parse, tag_str_len);
        tmp___1 = brubeck_tags_insert(
            tags,
            tag_str_for_key as *const libc::c_char,
            tag_str_len,
            tag_set,
        );
        if !tmp___1 {
            free(tag_set as *mut libc::c_void);
            free(tag_str_for_parse as *mut libc::c_void);
            free(tag_str_for_key as *mut libc::c_void);
            tag_set = brubeck_tags_find(tags, tag_str, tag_str_len);
        }
    }
    return tag_set as *const brubeck_tag_set;
}
pub unsafe extern "C" fn brubeck_get_tag_set(
    mut tags: *mut brubeck_tags_t,
    mut key_str: *const libc::c_char,
    mut key_len: uint16_t,
) -> *const brubeck_tag_set {
    let mut tag_offset: uint16_t = 0;
    let mut tmp: uint16_t = 0;
    let mut tmp___0: *const brubeck_tag_set = 0 as *const brubeck_tag_set;
    tmp = brubeck_tag_offset(key_str);
    tag_offset = tmp;
    tmp___0 = brubeck_get_tag_set_of_tag_str(
        tags,
        key_str.offset(tag_offset as libc::c_int as isize),
        (key_len as libc::c_int - tag_offset as libc::c_int) as uint16_t,
    );
    return tmp___0;
}
pub unsafe extern "C" fn find_substr(
    mut s: *const libc::c_char,
    mut find: *const libc::c_char,
    mut slen: size_t,
) -> *mut libc::c_char {
    let mut c: libc::c_char = 0;
    let mut sc: libc::c_char = 0;
    let mut len: size_t = 0;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *const libc::c_char = 0 as *const libc::c_char;
    tmp___2 = find;
    find = find.offset(1);
    c = *tmp___2;
    if c as libc::c_int != 0 as libc::c_int {
        len = strlen(find);
        loop {
            loop {
                tmp = s;
                s = s.offset(1);
                sc = *tmp;
                if sc as libc::c_int == 0 as libc::c_int {
                    return 0 as *mut libc::c_void as *mut libc::c_char
                } else {
                    tmp___0 = slen;
                    slen = slen.wrapping_sub(1);
                    if tmp___0 < 1 as libc::c_ulong {
                        return 0 as *mut libc::c_void as *mut libc::c_char;
                    }
                }
                if !(sc as libc::c_int != c as libc::c_int) {
                    break;
                }
            }
            if len > slen {
                return 0 as *mut libc::c_void as *mut libc::c_char;
            }
            tmp___1 = strncmp(s, find, len);
            if !(tmp___1 != 0 as libc::c_int) {
                break;
            }
        }
        s = s.offset(-1);
    }
    return s as *mut libc::c_char;
}
pub unsafe extern "C" fn sock_setnonblock(mut fd: libc::c_int) {
    let mut flags: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    flags = fcntl(fd, 3 as libc::c_int);
    flags |= 2048 as libc::c_int;
    tmp = fcntl(fd, 4 as libc::c_int, flags);
    if tmp < 0 as libc::c_int {
        fprintf(
            stderr,
            b"[FATAL]: Failed to set O_NONBLOCK\n\0" as *const u8 as *const libc::c_char,
        );
        gh_log_die();
    }
}
pub unsafe extern "C" fn sock_setreuse(mut fd: libc::c_int, mut reuse: libc::c_int) {
    let mut tmp: libc::c_int = 0;
    tmp = setsockopt(
        fd,
        1 as libc::c_int,
        2 as libc::c_int,
        &mut reuse as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    if tmp == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"[FATAL]: Failed to set SO_REUSEADDR\n\0" as *const u8
                as *const libc::c_char,
        );
        gh_log_die();
    }
}
pub unsafe extern "C" fn sock_enlarge_in(mut fd: libc::c_int) {
    let mut bs: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    bs = 33554431 as libc::c_int;
    tmp = setsockopt(
        fd,
        1 as libc::c_int,
        8 as libc::c_int,
        &mut bs as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    if tmp == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"[FATAL]: Failed to set SO_RCVBUF\n\0" as *const u8 as *const libc::c_char,
        );
        gh_log_die();
    }
}
pub unsafe extern "C" fn sock_enlarge_out(mut fd: libc::c_int) {
    let mut bs: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    bs = 33554431 as libc::c_int;
    tmp = setsockopt(
        fd,
        1 as libc::c_int,
        7 as libc::c_int,
        &mut bs as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    if tmp == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"[FATAL]: Failed to set SO_SNDBUF\n\0" as *const u8 as *const libc::c_char,
        );
        gh_log_die();
    }
}
pub unsafe extern "C" fn sock_setreuse_port(
    mut fd: libc::c_int,
    mut reuse: libc::c_int,
) {
    let mut tmp: libc::c_int = 0;
    tmp = setsockopt(
        fd,
        1 as libc::c_int,
        15 as libc::c_int,
        &mut reuse as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    if tmp == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"[FATAL]: failed to set SO_REUSEPORT\n\0" as *const u8
                as *const libc::c_char,
        );
        gh_log_die();
    }
}
pub unsafe extern "C" fn url_to_inaddr2(
    mut addr: *mut sockaddr_in,
    mut url: *const libc::c_char,
    mut port: libc::c_int,
) {
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
    let mut tmp: libc::c_int = 0;
    memset(
        addr as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
    );
    if !url.is_null() {
        memset(
            &mut hints as *mut addrinfo as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
        );
        hints.ai_family = 2 as libc::c_int;
        tmp = getaddrinfo(
            url,
            0 as *mut libc::c_void as *const libc::c_char,
            &mut hints as *mut addrinfo as *const addrinfo,
            &mut result as *mut *mut addrinfo,
        );
        if tmp != 0 as libc::c_int {
            fprintf(
                stderr,
                b"[FATAL]: failed to resolve address '%s'\n\0" as *const u8
                    as *const libc::c_char,
                url,
            );
            gh_log_die();
        }
        rp = result;
        while !rp.is_null() {
            if (*result).ai_family == 2 as libc::c_int {
                if (*result).ai_addrlen as libc::c_ulong
                    == ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
                {
                    break;
                }
            }
            rp = (*rp).ai_next;
        }
        if rp.is_null() {
            fprintf(
                stderr,
                b"[FATAL]: address format not supported\n\0" as *const u8
                    as *const libc::c_char,
            );
            gh_log_die();
        }
        memcpy(
            addr as *mut libc::c_void,
            (*rp).ai_addr as *const libc::c_void,
            (*rp).ai_addrlen as size_t,
        );
        (*addr).sin_port = __bswap_16(port as __uint16_t);
        freeaddrinfo(result);
    } else {
        (*addr).sin_family = 2 as libc::c_int as sa_family_t;
        (*addr).sin_port = __bswap_16(port as __uint16_t);
        (*addr).sin_addr.s_addr = __bswap_32(0 as libc::c_int as in_addr_t);
    };
}
pub unsafe extern "C" fn brubeck_itoa(
    mut ptr: *mut libc::c_char,
    mut number: uint64_t,
) -> libc::c_int {
    let mut origin: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: libc::c_int = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: libc::c_char = 0;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    origin = ptr;
    loop {
        tmp = ptr;
        ptr = ptr.offset(1);
        *tmp = (48 as libc::c_ulong)
            .wrapping_add(number.wrapping_rem(10 as libc::c_ulong)) as libc::c_char;
        number = (number as libc::c_ulong).wrapping_div(10 as libc::c_ulong) as uint64_t
            as uint64_t;
        if number == 0 {
            break;
        }
    }
    size = ptr.offset_from(origin) as libc::c_long as libc::c_int;
    ptr = ptr.offset(-1);
    while (origin as libc::c_ulong) < ptr as libc::c_ulong {
        t = *ptr;
        tmp___0 = ptr;
        ptr = ptr.offset(-1);
        *tmp___0 = *origin;
        tmp___1 = origin;
        origin = origin.offset(1);
        *tmp___1 = t;
    }
    return size;
}
pub unsafe extern "C" fn brubeck_ftoa(
    mut outbuf: *mut libc::c_char,
    mut f: libc::c_float,
) -> libc::c_int {
    let mut mantissa: uint64_t = 0;
    let mut int_part: uint64_t = 0;
    let mut frac_part: uint64_t = 0;
    let mut safe_shift: libc::c_int = 0;
    let mut safe_mask: uint64_t = 0;
    let mut exp2___0: libc::c_short = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x: __anonunion_x_375160861 = __anonunion_x_375160861 { L: 0 };
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    x.F = f;
    p = outbuf;
    exp2___0 = ((x.L >> 23 as libc::c_int) as libc::c_uchar as libc::c_int
        - 127 as libc::c_int) as libc::c_short;
    mantissa = (x.L & 16777215 as libc::c_int | 8388608 as libc::c_int) as uint64_t;
    frac_part = 0 as libc::c_int as uint64_t;
    int_part = 0 as libc::c_int as uint64_t;
    if x.L < 0 as libc::c_int {
        tmp = p;
        p = p.offset(1);
        *tmp = '-' as i32 as libc::c_char;
    }
    if (exp2___0 as libc::c_int) < -(36 as libc::c_int) {
        tmp___0 = p;
        p = p.offset(1);
        *tmp___0 = '0' as i32 as libc::c_char;
    } else {
        safe_shift = -(exp2___0 as libc::c_int + 1 as libc::c_int);
        safe_mask = (18446744073709551615 as libc::c_ulonglong
            >> 40 as libc::c_int - safe_shift) as uint64_t;
        if exp2___0 as libc::c_int >= 64 as libc::c_int {
            int_part = 18446744073709551615 as libc::c_ulonglong as uint64_t;
        } else if exp2___0 as libc::c_int >= 23 as libc::c_int {
            int_part = mantissa << exp2___0 as libc::c_int - 23 as libc::c_int;
        } else if exp2___0 as libc::c_int >= 0 as libc::c_int {
            int_part = mantissa >> 23 as libc::c_int - exp2___0 as libc::c_int;
            frac_part = mantissa & safe_mask;
        } else {
            frac_part = mantissa & 16777215 as libc::c_ulong;
        }
        if int_part == 0 as libc::c_ulong {
            tmp___1 = p;
            p = p.offset(1);
            *tmp___1 = '0' as i32 as libc::c_char;
        } else {
            tmp___2 = brubeck_itoa(p, int_part);
            p = p.offset(tmp___2 as isize);
        }
        if frac_part != 0 as libc::c_ulong {
            tmp___3 = p;
            p = p.offset(1);
            *tmp___3 = '.' as i32 as libc::c_char;
            m = 0 as libc::c_int;
            while m < 4 as libc::c_int {
                frac_part = (frac_part << 3 as libc::c_int)
                    .wrapping_add(frac_part << 1 as libc::c_int);
                tmp___4 = p;
                p = p.offset(1);
                *tmp___4 = (frac_part >> 24 as libc::c_int + safe_shift)
                    .wrapping_add(48 as libc::c_ulong) as libc::c_char;
                frac_part &= safe_mask;
                m += 1;
            }
            while *p.offset(-(1 as libc::c_int) as isize) as libc::c_int
                == 48 as libc::c_int
            {
                p = p.offset(-1);
            }
            if *p.offset(-(1 as libc::c_int) as isize) as libc::c_int
                == 46 as libc::c_int
            {
                p = p.offset(-1);
            }
        }
    }
    *p = 0 as libc::c_int as libc::c_char;
    return p.offset_from(outbuf) as libc::c_long as libc::c_int;
}
#[inline]
unsafe extern "C" fn json_incref(mut json: *mut json_t) -> *mut json_t {
    if !json.is_null() {
        if (*json).refcount != 0xffffffffffffffff as libc::c_ulong {
            let fresh29 = &mut (*json).refcount;
            let fresh30 = 1 as libc::c_int as size_t;
            ::std::intrinsics::atomic_xadd_acquire(fresh29, fresh30) + fresh30;
        }
    }
    return json;
}
#[inline]
unsafe extern "C" fn json_object_set_nocheck(
    mut object: *mut json_t,
    mut key: *const libc::c_char,
    mut value: *mut json_t,
) -> libc::c_int {
    let mut tmp: *mut json_t = 0 as *mut json_t;
    let mut tmp___0: libc::c_int = 0;
    tmp = json_incref(value);
    tmp___0 = json_object_set_new_nocheck(object, key, tmp);
    return tmp___0;
}
static mut errstr: [libc::c_char; 512] = [0; 512];
unsafe extern "C" fn kafka_shutdown(mut backend: *mut libc::c_void) -> libc::c_int {
    let mut self_0: *mut brubeck_kafka = 0 as *mut brubeck_kafka;
    let mut err: rd_kafka_resp_err_t = RD_KAFKA_RESP_ERR_NO_ERROR;
    let mut i: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___2: *const libc::c_char = 0 as *const libc::c_char;
    self_0 = backend as *mut brubeck_kafka;
    (*self_0).connected = 0 as libc::c_int != 0;
    i = 0 as libc::c_int;
    loop {
        if !((*self_0).documents).is_null() {
            tmp = *((*self_0).documents as *mut size_t)
                .offset(-(2 as libc::c_int) as isize);
        } else {
            tmp = 0 as libc::c_int as size_t;
        }
        if !((i as size_t) < tmp) {
            break;
        }
        if *((*self_0).documents).offset(i as isize) as libc::c_ulong
            != 0 as *mut libc::c_void as libc::c_ulong
        {
            json_decref((**((*self_0).documents).offset(i as isize)).json);
        }
        i += 1;
    }
    err = rd_kafka_fatal_error(
        (*self_0).rk,
        errstr.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
    );
    if err as u64 != 0 {
        tmp___0 = rd_kafka_err2name(err);
        tmp___1 = gh_log_instance();
        gh_log_write(
            b"instance=%s backend=kafka event=fatal_error reason=%s msg=\"%s\"\n\0"
                as *const u8 as *const libc::c_char,
            tmp___1,
            tmp___0,
            errstr.as_mut_ptr(),
        );
    }
    tmp___2 = gh_log_instance();
    gh_log_write(
        b"instance=%s backend=kafka event=flushing-outstanding-messages\n\0" as *const u8
            as *const libc::c_char,
        tmp___2,
    );
    rd_kafka_flush((*self_0).rk, 10000 as libc::c_int);
    rd_kafka_destroy((*self_0).rk);
    return err as libc::c_int;
}
unsafe extern "C" fn dr_msg_cb(
    mut rk: *mut rd_kafka_t,
    mut rkmessage: *const rd_kafka_message_t,
    mut opaque: *mut libc::c_void,
) {
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    if (*rkmessage).err as u64 != 0 {
        tmp = rd_kafka_err2name((*rkmessage).err);
        tmp___0 = gh_log_instance();
        gh_log_write(
            b"instance=%s backend=kafka event=delivery_error msg=\"%s\"\n\0" as *const u8
                as *const libc::c_char,
            tmp___0,
            tmp,
        );
    }
}
unsafe extern "C" fn error_cb(
    mut rk: *mut rd_kafka_t,
    mut err: libc::c_int,
    mut reason: *const libc::c_char,
    mut opaque: *mut libc::c_void,
) {
    let mut self_0: *mut brubeck_kafka = 0 as *mut brubeck_kafka;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    self_0 = opaque as *mut brubeck_kafka;
    tmp = rd_kafka_err2name(err as rd_kafka_resp_err_t);
    tmp___0 = gh_log_instance();
    gh_log_write(
        b"instance=%s backend=kafka event=producer_error reason=%s msg=\"%s\"\n\0"
            as *const u8 as *const libc::c_char,
        tmp___0,
        tmp,
        reason,
    );
    if err != -(150 as libc::c_int) {
        return;
    }
    tmp___1 = kafka_shutdown(self_0 as *mut libc::c_void);
    if tmp___1 != 0 {
        exit(1 as libc::c_int);
    }
}
unsafe extern "C" fn kafka_is_connected(mut backend: *mut libc::c_void) -> bool {
    let mut self_0: *mut brubeck_kafka = 0 as *mut brubeck_kafka;
    self_0 = backend as *mut brubeck_kafka;
    return (*self_0).connected;
}
unsafe extern "C" fn kafka_connect(mut backend: *mut libc::c_void) -> libc::c_int {
    let mut self_0: *mut brubeck_kafka = 0 as *mut brubeck_kafka;
    self_0 = backend as *mut brubeck_kafka;
    if (*self_0).connected {
        return 0 as libc::c_int
    } else {
        return -(1 as libc::c_int)
    };
}
unsafe extern "C" fn each_metric(
    mut metric: *const brubeck_metric,
    mut key: *const libc::c_char,
    mut value: value_t,
    mut backend: *mut libc::c_void,
) {
    let mut self_0: *mut brubeck_kafka = 0 as *mut brubeck_kafka;
    let mut tag_index: uint32_t = 0;
    let mut doc: *mut brubeck_kafka_document = 0 as *mut brubeck_kafka_document;
    let mut tmp___0: *mut brubeck_kafka_document = 0 as *mut brubeck_kafka_document;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut __cap: size_t = 0;
    let mut tmp___3: size_t = 0;
    let mut __size: size_t = 0;
    let mut __p: *mut size_t = 0 as *mut size_t;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut __prev_size: size_t = 0;
    let mut tmp___5: size_t = 0;
    let mut __p1: *mut size_t = 0 as *mut size_t;
    let mut __p2: *mut size_t = 0 as *mut size_t;
    let mut tmp___6: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tag_destination: *mut json_t = 0 as *mut json_t;
    let mut i: uint16_t = 0;
    let mut tmp___7: *mut json_t = 0 as *mut json_t;
    let mut tmp___8: *mut json_t = 0 as *mut json_t;
    self_0 = backend as *mut brubeck_kafka;
    tag_index = 0 as libc::c_int as uint32_t;
    if (*metric).tags as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tag_index = (*(*metric).tags).index;
    }
    if !((*self_0).documents).is_null() {
        tmp___1 = *((*self_0).documents as *mut size_t)
            .offset(-(2 as libc::c_int) as isize);
    } else {
        tmp___1 = 0 as libc::c_int as size_t;
    }
    if tag_index as size_t >= tmp___1 {
        tmp___0 = 0 as *mut libc::c_void as *mut brubeck_kafka_document;
    } else {
        tmp___0 = *((*self_0).documents).offset(tag_index as isize);
    }
    doc = tmp___0;
    if doc as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___2 = malloc(
            ::std::mem::size_of::<brubeck_kafka_document>() as libc::c_ulong,
        );
        doc = tmp___2 as *mut brubeck_kafka_document;
        (*doc).is_dirty = 0 as libc::c_int != 0;
        (*doc).json = json_object();
        if !((*self_0).documents).is_null() {
            tmp___3 = *((*self_0).documents as *mut size_t)
                .offset(-(1 as libc::c_int) as isize);
        } else {
            tmp___3 = 0 as libc::c_int as size_t;
        }
        __cap = tmp___3;
        if __cap <= tag_index as size_t {
            if __cap == 0 {
                __cap = 1 as libc::c_int as size_t;
            }
            while __cap <= tag_index as size_t {
                __cap <<= 1 as libc::c_int;
            }
            __size = __cap
                .wrapping_mul(
                    ::std::mem::size_of::<*mut brubeck_kafka_document>() as libc::c_ulong,
                )
                .wrapping_add(
                    (::std::mem::size_of::<size_t>() as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_ulong),
                );
            if ((*self_0).documents).is_null() {
                tmp___4 = malloc(__size);
                __p = tmp___4 as *mut size_t;
                memset(__p as *mut libc::c_void, 0 as libc::c_int, __size);
                (*self_0)
                    .documents = __p.offset(2 as libc::c_int as isize)
                    as *mut libc::c_void as *mut *mut brubeck_kafka_document;
                if !((*self_0).documents).is_null() {
                    *((*self_0).documents as *mut size_t)
                        .offset(-(1 as libc::c_int) as isize) = __cap;
                }
                if !((*self_0).documents).is_null() {
                    *((*self_0).documents as *mut size_t)
                        .offset(
                            -(2 as libc::c_int) as isize,
                        ) = 0 as libc::c_int as size_t;
                }
            } else {
                if !((*self_0).documents).is_null() {
                    tmp___5 = *((*self_0).documents as *mut size_t)
                        .offset(-(1 as libc::c_int) as isize);
                } else {
                    tmp___5 = 0 as libc::c_int as size_t;
                }
                __prev_size = tmp___5
                    .wrapping_mul(
                        ::std::mem::size_of::<*mut brubeck_kafka_document>()
                            as libc::c_ulong,
                    )
                    .wrapping_add(
                        (::std::mem::size_of::<size_t>() as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong),
                    );
                __p1 = ((*self_0).documents as *mut size_t)
                    .offset(-(2 as libc::c_int) as isize);
                tmp___6 = realloc(__p1 as *mut libc::c_void, __size);
                __p2 = tmp___6 as *mut size_t;
                memset(
                    (__p2 as *mut libc::c_char).offset(__prev_size as isize)
                        as *mut libc::c_void,
                    0 as libc::c_int,
                    __size.wrapping_sub(__prev_size),
                );
                (*self_0)
                    .documents = __p2.offset(2 as libc::c_int as isize)
                    as *mut libc::c_void as *mut *mut brubeck_kafka_document;
                if !((*self_0).documents).is_null() {
                    *((*self_0).documents as *mut size_t)
                        .offset(-(1 as libc::c_int) as isize) = __cap;
                }
            }
        }
        let ref mut fresh31 = *((*self_0).documents).offset(tag_index as isize);
        *fresh31 = doc;
        if !((*self_0).documents).is_null() {
            *((*self_0).documents as *mut size_t)
                .offset(
                    -(2 as libc::c_int) as isize,
                ) = tag_index.wrapping_add(1 as libc::c_uint) as size_t;
        }
    }
    if (*doc).is_dirty as libc::c_int == 0 as libc::c_int {
        if (*metric).tags as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            if (*(*metric).tags).num_tags as libc::c_int > 0 as libc::c_int {
                tag_destination = (*doc).json;
                if (*self_0).tag_subdocument as libc::c_ulong
                    != 0 as *mut libc::c_void as libc::c_ulong
                {
                    tag_destination = json_object();
                    json_object_set_new_nocheck(
                        (*doc).json,
                        (*self_0).tag_subdocument,
                        tag_destination,
                    );
                }
                i = 0 as libc::c_int as uint16_t;
                while (i as libc::c_int) < (*(*metric).tags).num_tags as libc::c_int {
                    tmp___7 = json_string(
                        (*((*(*metric).tags).tags).as_ptr().offset(i as isize)).value,
                    );
                    json_object_set_new_nocheck(
                        tag_destination,
                        (*((*(*metric).tags).tags).as_ptr().offset(i as isize)).key,
                        tmp___7,
                    );
                    i = (i as libc::c_int + 1 as libc::c_int) as uint16_t;
                }
            }
        }
    }
    tmp___8 = json_real(value);
    json_object_set_new_nocheck((*doc).json, key, tmp___8);
    (*doc).is_dirty = 1 as libc::c_int != 0;
}
unsafe extern "C" fn kafka_flush(mut backend: *mut libc::c_void) {
    let mut self_0: *mut brubeck_kafka = 0 as *mut brubeck_kafka;
    let mut err: rd_kafka_resp_err_t = RD_KAFKA_RESP_ERR_NO_ERROR;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut epoch_ms: int64_t = 0;
    let mut json_epoch_ms: *mut json_t = 0 as *mut json_t;
    let mut i: libc::c_int = 0;
    let mut doc: *mut brubeck_kafka_document = 0 as *mut brubeck_kafka_document;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: size_t = 0;
    self_0 = backend as *mut brubeck_kafka;
    epoch_ms = (*self_0).backend.tick_time as int64_t;
    epoch_ms *= 1000 as libc::c_long;
    json_epoch_ms = json_integer(epoch_ms as json_int_t);
    i = 0 as libc::c_int;
    loop {
        if !((*self_0).documents).is_null() {
            tmp___1 = *((*self_0).documents as *mut size_t)
                .offset(-(2 as libc::c_int) as isize);
        } else {
            tmp___1 = 0 as libc::c_int as size_t;
        }
        if !((i as size_t) < tmp___1) {
            break;
        }
        doc = *((*self_0).documents).offset(i as isize);
        if doc as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            if (*doc).is_dirty {
                json_object_set_nocheck(
                    (*doc).json,
                    b"@timestamp\0" as *const u8 as *const libc::c_char,
                    json_epoch_ms,
                );
                buf = json_dumps(
                    (*doc).json as *const json_t,
                    32 as libc::c_int as size_t,
                );
                len = strlen(buf as *const libc::c_char);
                err = rd_kafka_producev(
                    (*self_0).rk,
                    1 as libc::c_int,
                    (*self_0).topic,
                    7 as libc::c_int,
                    1 as libc::c_int,
                    4 as libc::c_int,
                    buf as *mut libc::c_void,
                    len,
                    6 as libc::c_int,
                    0 as *mut libc::c_void,
                    0 as libc::c_int,
                );
                if err as u64 != 0 {
                    tmp = rd_kafka_err2str(err);
                    tmp___0 = gh_log_instance();
                    gh_log_write(
                        b"instance=%s backend=kafka event=failed_to_enqueue msg=\"%s\"\n\0"
                            as *const u8 as *const libc::c_char,
                        tmp___0,
                        tmp,
                    );
                    free(buf as *mut libc::c_void);
                } else {
                    (*self_0)
                        .bytes_sent = ((*self_0).bytes_sent as libc::c_ulong)
                        .wrapping_add(len) as size_t as size_t;
                }
                json_object_clear((*doc).json);
                (*doc).is_dirty = 0 as libc::c_int != 0;
                rd_kafka_poll((*self_0).rk, 0 as libc::c_int);
            }
        }
        i += 1;
    }
    json_decref(json_epoch_ms);
}
unsafe extern "C" fn build_rdkafka_config(
    mut json: *mut json_t,
) -> *mut rd_kafka_conf_t {
    let mut conf: *mut rd_kafka_conf_t = 0 as *mut rd_kafka_conf_t;
    let mut retval: libc::c_int = 0;
    let mut key: *const libc::c_char = 0 as *const libc::c_char;
    let mut value: *mut json_t = 0 as *mut json_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___3: rd_kafka_conf_res_t = RD_KAFKA_CONF_OK;
    let mut tmp___4: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___5: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___6: *mut libc::c_void = 0 as *mut libc::c_void;
    conf = rd_kafka_conf_new();
    tmp = json_object_iter(json);
    key = json_object_iter_key(tmp);
    while !key.is_null() {
        tmp___6 = json_object_key_to_iter(key);
        value = json_object_iter_value(tmp___6);
        if value.is_null() {
            break;
        }
        tmp___2 = json_string_value(value as *const json_t);
        tmp___3 = rd_kafka_conf_set(
            conf,
            key,
            tmp___2,
            errstr.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
        );
        retval = tmp___3 as libc::c_int;
        if retval != 0 as libc::c_int {
            tmp___4 = rd_kafka_err2name(retval as rd_kafka_resp_err_t);
            tmp___5 = gh_log_instance();
            gh_log_write(
                b"instance=%s backend=kafka event=conf_error key=%s code=%s msg=\"%s\"\n\0"
                    as *const u8 as *const libc::c_char,
                tmp___5,
                key,
                tmp___4,
                errstr.as_mut_ptr(),
            );
            rd_kafka_conf_destroy(conf);
            exit(1 as libc::c_int);
        }
        tmp___0 = json_object_key_to_iter(key);
        tmp___1 = json_object_iter_next(json, tmp___0);
        key = json_object_iter_key(tmp___1);
    }
    return conf;
}
pub unsafe extern "C" fn brubeck_kafka_new(
    mut server: *mut brubeck_server,
    mut settings: *mut json_t,
    mut shard_n: libc::c_int,
) -> *mut brubeck_backend {
    let mut self_0: *mut brubeck_kafka = 0 as *mut brubeck_kafka;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut frequency: libc::c_int = 0;
    let mut rdkafka_config: *mut json_t = 0 as *mut json_t;
    let mut conf: *mut rd_kafka_conf_t = 0 as *mut rd_kafka_conf_t;
    let mut _error_j: json_error_t = json_error_t {
        line: 0,
        column: 0,
        position: 0,
        source: [0; 80],
        text: [0; 160],
    };
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___2: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___3: *const libc::c_char = 0 as *const libc::c_char;
    tmp = xmalloc(::std::mem::size_of::<brubeck_kafka>() as libc::c_ulong);
    self_0 = tmp as *mut brubeck_kafka;
    memset(
        self_0 as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<brubeck_kafka>() as libc::c_ulong,
    );
    frequency = 0 as libc::c_int;
    tmp___0 = json_unpack_ex(
        settings,
        &mut _error_j as *mut json_error_t,
        0 as libc::c_int as size_t,
        b"{s:s, s:i, s:o, s?:s}\0" as *const u8 as *const libc::c_char,
        b"topic\0" as *const u8 as *const libc::c_char,
        &mut (*self_0).topic as *mut *const libc::c_char,
        b"frequency\0" as *const u8 as *const libc::c_char,
        &mut frequency as *mut libc::c_int,
        b"rdkafka_config\0" as *const u8 as *const libc::c_char,
        &mut rdkafka_config as *mut *mut json_t,
        b"tag_subdocument\0" as *const u8 as *const libc::c_char,
        &mut (*self_0).tag_subdocument as *mut *const libc::c_char,
    );
    if tmp___0 < 0 as libc::c_int {
        fprintf(
            stderr,
            b"[FATAL]: config error: %s\n\0" as *const u8 as *const libc::c_char,
            (_error_j.text).as_mut_ptr(),
        );
        gh_log_die();
    }
    conf = build_rdkafka_config(rdkafka_config);
    (*self_0).connected = 1 as libc::c_int != 0;
    (*self_0).backend.type_0 = BRUBECK_BACKEND_KAFKA;
    (*self_0)
        .backend
        .connect = Some(
        kafka_connect as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
    );
    (*self_0)
        .backend
        .is_connected = Some(
        kafka_is_connected as unsafe extern "C" fn(*mut libc::c_void) -> bool,
    );
    (*self_0)
        .backend
        .sample = Some(
        each_metric
            as unsafe extern "C" fn(
                *const brubeck_metric,
                *const libc::c_char,
                value_t,
                *mut libc::c_void,
            ) -> (),
    );
    (*self_0)
        .backend
        .flush = Some(kafka_flush as unsafe extern "C" fn(*mut libc::c_void) -> ());
    (*self_0).backend.sample_freq = frequency;
    (*self_0).backend.server = server;
    rd_kafka_conf_set_dr_msg_cb(
        conf,
        Some(
            dr_msg_cb
                as unsafe extern "C" fn(
                    *mut rd_kafka_t,
                    *const rd_kafka_message_t,
                    *mut libc::c_void,
                ) -> (),
        ),
    );
    rd_kafka_conf_set_error_cb(
        conf,
        Some(
            error_cb
                as unsafe extern "C" fn(
                    *mut rd_kafka_t,
                    libc::c_int,
                    *const libc::c_char,
                    *mut libc::c_void,
                ) -> (),
        ),
    );
    rd_kafka_conf_set_opaque(conf, self_0 as *mut libc::c_void);
    (*self_0)
        .rk = rd_kafka_new(
        RD_KAFKA_PRODUCER,
        conf,
        errstr.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
    );
    if ((*self_0).rk).is_null() {
        tmp___1 = gh_log_instance();
        gh_log_write(
            b"instance=%s backend=kafka event=producer_creation_failed error=\"%s\"\n\0"
                as *const u8 as *const libc::c_char,
            tmp___1,
            errstr.as_mut_ptr(),
        );
        rd_kafka_conf_destroy(conf);
        exit(1 as libc::c_int);
    }
    tmp___2 = gh_log_instance();
    gh_log_write(
        b"instance=%s backend=kafka event=ready\n\0" as *const u8 as *const libc::c_char,
        tmp___2,
    );
    brubeck_backend_run_threaded(self_0 as *mut brubeck_backend);
    tmp___3 = gh_log_instance();
    gh_log_write(
        b"instance=%s backend=kafka event=started\n\0" as *const u8
            as *const libc::c_char,
        tmp___3,
    );
    return self_0 as *mut brubeck_backend;
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
