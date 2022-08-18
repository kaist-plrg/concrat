use ::libc;
use libc::strcat;
use libc::strcpy;
use libc::strncpy;
use libc::memcpy;
use libc::memset;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type json_object;
    pub type pcap;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
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
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn __fgets_chk(
        __s: *mut libc::c_char,
        __size: size_t,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn __fgets_alias(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn __fgets_chk_warn(
        __s: *mut libc::c_char,
        __size: size_t,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn __fread_chk(
        __ptr: *mut libc::c_void,
        __ptrlen: size_t,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn __fread_alias(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn __fread_chk_warn(
        __ptr: *mut libc::c_void,
        __ptrlen: size_t,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn log_fatal(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> !;
    fn rijndaelKeySetupEnc(
        rk: *mut u32_0,
        cipherKey: *const u8_0,
        keyBits: libc::c_int,
    ) -> libc::c_int;
    fn rijndaelEncrypt(
        rk: *const u32_0,
        Nr: libc::c_int,
        pt: *const u8_0,
        ct: *mut u8_0,
    );
    fn random_bytes(dst: *mut libc::c_void, n: size_t) -> libc::c_int;
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_init(_: mpz_ptr);
    fn __gmpz_init_set_ui(_: mpz_ptr, _: libc::c_ulong);
    fn __gmpz_powm(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __recv_chk(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __buflen: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn __recv_alias(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn __recv_chk_warn(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __buflen: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn log_debug(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __printf_chk(
        __flag: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn u8_check(s: *const uint8_t, n: size_t) -> *const uint8_t;
    fn log_warn(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn xcalloc(count: size_t, size: size_t) -> *mut libc::c_void;
    static mut stderr: *mut FILE;
    fn __fprintf_chk(
        __stream: *mut FILE,
        __flag: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn __gethostname_chk(
        __buf: *mut libc::c_char,
        __buflen: size_t,
        __nreal: size_t,
    ) -> libc::c_int;
    fn __gethostname_alias(__buf: *mut libc::c_char, __buflen: size_t) -> libc::c_int;
    fn __gethostname_chk_warn(
        __buf: *mut libc::c_char,
        __buflen: size_t,
        __nreal: size_t,
    ) -> libc::c_int;
    fn log_error(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn getpid() -> __pid_t;
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
    fn pcap_lookupdev(_: *mut libc::c_char) -> *mut libc::c_char;
    fn if_nametoindex(__ifname: *const libc::c_char) -> libc::c_uint;
    fn if_indextoname(
        __ifindex: libc::c_uint,
        __ifname: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn blocklist_count_allowed() -> uint64_t;
    fn now() -> libc::c_double;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn lock_file(f: *mut FILE) -> libc::c_int;
    fn unlock_file(f: *mut FILE) -> libc::c_int;
    fn time_string(
        time_0: uint32_t,
        est: libc::c_int,
        buf: *mut libc::c_char,
        len: size_t,
    );
    fn number_string(n: uint32_t, buf: *mut libc::c_char, len: size_t);
    fn log_info(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn pbm_init() -> *mut *mut uint8_t;
    fn pbm_check(b: *mut *mut uint8_t, v: uint32_t) -> libc::c_int;
    fn pbm_set(b: *mut *mut uint8_t, v: uint32_t);
    fn sendto(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: *const sockaddr,
        __addr_len: socklen_t,
    ) -> ssize_t;
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
    fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: socklen_t,
    ) -> *const libc::c_char;
    fn perror(__s: *const libc::c_char);
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> libc::c_int;
    fn signal(
        __sig: libc::c_int,
        __handler: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> __sighandler_t;
    fn blocklist_lookup_index(index: uint64_t) -> uint32_t;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn gethostbyname(__name: *const libc::c_char) -> *mut hostent;
    fn dstrftime(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: libc::c_double,
    ) -> size_t;
    fn get_blocklisted_cidrs() -> *mut bl_cidr_node_t;
    fn get_allowlisted_cidrs() -> *mut bl_cidr_node_t;
    fn json_object_put(obj: *mut json_object) -> libc::c_int;
    fn json_object_to_json_string(obj: *mut json_object) -> *const libc::c_char;
    fn json_object_new_object() -> *mut json_object;
    fn json_object_object_add(
        obj: *mut json_object,
        key: *const libc::c_char,
        val: *mut json_object,
    ) -> libc::c_int;
    fn json_object_new_array() -> *mut json_object;
    fn json_object_array_add(
        obj: *mut json_object,
        val: *mut json_object,
    ) -> libc::c_int;
    fn json_object_new_int(i: int32_t) -> *mut json_object;
    fn json_object_new_int64(i: int64_t) -> *mut json_object;
    fn json_object_new_double(d: libc::c_double) -> *mut json_object;
    fn json_object_new_string(s: *const libc::c_char) -> *mut json_object;
    fn json_tokener_parse(str: *const libc::c_char) -> *mut json_object;
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn exit(_: libc::c_int) -> !;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn time(__timer: *mut time_t) -> time_t;
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
    fn blocklist_init(
        allowlist: *mut libc::c_char,
        blocklist: *mut libc::c_char,
        allowlist_entries: *mut *mut libc::c_char,
        allowlist_entries_len: size_t,
        blocklist_entries: *mut *mut libc::c_char,
        blocklist_entries_len: size_t,
        ignore_invalid_hosts: libc::c_int,
    ) -> libc::c_int;
    fn blocklist_count_not_allowed() -> uint64_t;
    fn log_init(
        stream: *mut FILE,
        level: LogLevel,
        syslog_enabled: libc::c_int,
        syslog_app: *const libc::c_char,
    ) -> libc::c_int;
    fn parse_max_hosts(max_targets: *mut libc::c_char) -> uint32_t;
    fn enforce_range(
        name: *const libc::c_char,
        v: libc::c_int,
        min: libc::c_int,
        max: libc::c_int,
    );
    fn split_string(
        in_0: *const libc::c_char,
        len: *mut libc::c_int,
        results: *mut *mut *const libc::c_char,
    );
    fn fprintw(f: *mut FILE, s: *const libc::c_char, w: size_t);
    fn parse_mac(out: *mut macaddr_t, in_0: *mut libc::c_char) -> libc::c_int;
    fn file_exists(name: *mut libc::c_char) -> libc::c_int;
    fn drop_privs() -> libc::c_int;
    fn set_cpu(core: uint32_t) -> libc::c_int;
    fn pbm_load_from_file(b: *mut *mut uint8_t, file_0: *mut libc::c_char) -> uint32_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn abort() -> !;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    static mut opterr: libc::c_int;
    static mut optopt: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn clearerr(__stream: *mut FILE);
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn blocklist_is_allowed(s_addr: uint32_t) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn xrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn random() -> libc::c_long;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strsep(
        __stringp: *mut *mut libc::c_char,
        __delim: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn check_and_log_file_error(file_0: *mut FILE, name: *const libc::c_char);
    fn json_object_to_json_string_ext(
        obj: *mut json_object,
        flags: libc::c_int,
    ) -> *const libc::c_char;
    fn json_object_new_boolean(b: json_bool) -> *mut json_object;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn pcap_open_live(
        _: *const libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_char,
    ) -> *mut pcap_t;
    fn pcap_close(_: *mut pcap_t);
    fn pcap_dispatch(
        _: *mut pcap_t,
        _: libc::c_int,
        _: Option::<
            unsafe extern "C" fn(*mut u_char, *const pcap_pkthdr, *const u_char) -> (),
        >,
        _: *mut u_char,
    ) -> libc::c_int;
    fn pcap_stats(_: *mut pcap_t, _: *mut pcap_stat) -> libc::c_int;
    fn pcap_setfilter(_: *mut pcap_t, _: *mut bpf_program) -> libc::c_int;
    fn pcap_setnonblock(
        _: *mut pcap_t,
        _: libc::c_int,
        _: *mut libc::c_char,
    ) -> libc::c_int;
    fn pcap_geterr(_: *mut pcap_t) -> *mut libc::c_char;
    fn pcap_compile(
        _: *mut pcap_t,
        _: *mut bpf_program,
        _: *const libc::c_char,
        _: libc::c_int,
        _: bpf_u_int32,
    ) -> libc::c_int;
    fn pcap_datalink(_: *mut pcap_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub type u8_0 = libc::c_uchar;
pub type u32_0 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aesrand {
    pub input: [uint32_t; 4],
    pub sched: [uint32_t; 44],
    pub output: [uint8_t; 16],
}
pub type aesrand_t = aesrand;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cyclic_group {
    pub prime: uint64_t,
    pub known_primroot: uint64_t,
    pub num_prime_factors: size_t,
    pub prime_factors: [uint64_t; 10],
}
pub type cyclic_group_t = cyclic_group;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cycle {
    pub group: *const cyclic_group_t,
    pub generator: uint64_t,
    pub order: uint64_t,
    pub offset: uint32_t,
}
pub type cycle_t = cycle;
pub type ssize_t = __ssize_t;
pub type mp_limb_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___mpz_struct_688629798 {
    pub _mp_alloc: libc::c_int,
    pub _mp_size: libc::c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub type __mpz_struct = __anonstruct___mpz_struct_688629798;
pub type mpz_t = [__mpz_struct; 1];
pub type mp_ptr = *mut mp_limb_t;
pub type mp_size_t = libc::c_long;
pub type mpz_srcptr = *const __mpz_struct;
pub type mpz_ptr = *mut __mpz_struct;
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
pub struct field_def {
    pub name: *const libc::c_char,
    pub type_0: *const libc::c_char,
    pub desc: *const libc::c_char,
}
pub type fielddef_t = field_def;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fielddef_set {
    pub fielddefs: [fielddef_t; 128],
    pub len: libc::c_int,
}
pub type fielddefset_t = fielddef_set;
#[derive(Copy, Clone)]
#[repr(C)]
pub union field_val {
    pub ptr: *mut libc::c_void,
    pub num: uint64_t,
}
pub type field_val_t = field_val;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field {
    pub name: *const libc::c_char,
    pub type_0: libc::c_int,
    pub free_: libc::c_int,
    pub len: size_t,
    pub value: field_val_t,
}
pub type field_t = field;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fieldset {
    pub len: libc::c_int,
    pub fields: [field_t; 128],
    pub fds: *mut fielddefset_t,
    pub inner_type: libc::c_int,
    pub type_0: libc::c_int,
    pub free_: libc::c_int,
}
pub type fieldset_t = fieldset;
pub type operation = libc::c_uint;
pub const GT_EQ: operation = 7;
pub const LT_EQ: operation = 6;
pub const OR: operation = 5;
pub const AND: operation = 4;
pub const NEQ: operation = 3;
pub const EQ: operation = 2;
pub const LT: operation = 1;
pub const GT: operation = 0;
pub type node_type = libc::c_uint;
pub const INT: node_type = 3;
pub const STRING: node_type = 2;
pub const FIELD: node_type = 1;
pub const OP: node_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field_id {
    pub index: libc::c_int,
    pub fieldname: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union node_value {
    pub field: field_id,
    pub string_literal: *mut libc::c_char,
    pub int_literal: uint64_t,
    pub op: operation,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_st {
    pub left_child: *mut node_st,
    pub right_child: *mut node_st,
    pub type_0: node_type,
    pub value: node_value,
}
pub type node_t = node_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct translation {
    pub len: libc::c_int,
    pub translation: [libc::c_int; 128],
}
pub type translation_t = translation;
pub type __uint16_t = libc::c_ushort;
pub type uint16_t = __uint16_t;
pub type port_h_t = uint16_t;
pub type macaddr_t = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output_filter {
    pub expression: *mut node_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct probe_module {
    pub name: *const libc::c_char,
    pub max_packet_length: size_t,
    pub pcap_filter: *const libc::c_char,
    pub pcap_snaplen: size_t,
    pub port_args: uint8_t,
    pub global_initialize: Option::<
        unsafe extern "C" fn(*mut state_conf) -> libc::c_int,
    >,
    pub thread_initialize: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut macaddr_t,
            *mut macaddr_t,
            port_n_t,
            *mut *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub make_packet: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut size_t,
            ipaddr_n_t,
            ipaddr_n_t,
            uint8_t,
            *mut uint32_t,
            libc::c_int,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub print_packet: Option::<unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> ()>,
    pub validate_packet: Option::<
        unsafe extern "C" fn(
            *const ip,
            uint32_t,
            *mut uint32_t,
            *mut uint32_t,
        ) -> libc::c_int,
    >,
    pub process_packet: Option::<
        unsafe extern "C" fn(
            *const u_char,
            uint32_t,
            *mut fieldset_t,
            *mut uint32_t,
            timespec,
        ) -> (),
    >,
    pub close: Option::<
        unsafe extern "C" fn(
            *mut state_conf,
            *mut state_send,
            *mut state_recv,
        ) -> libc::c_int,
    >,
    pub output_type: libc::c_int,
    pub fields: *mut fielddef_t,
    pub numfields: libc::c_int,
    pub helptext: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_recv {
    pub success_total: uint32_t,
    pub success_unique: uint32_t,
    pub app_success_total: uint32_t,
    pub app_success_unique: uint32_t,
    pub cooldown_total: uint32_t,
    pub cooldown_unique: uint32_t,
    pub failure_total: uint32_t,
    pub filter_success: uint64_t,
    pub ip_fragments: uint32_t,
    pub validation_passed: uint32_t,
    pub validation_failed: uint32_t,
    pub complete: libc::c_int,
    pub start: libc::c_double,
    pub finish: libc::c_double,
    pub pcap_recv: uint32_t,
    pub pcap_drop: uint32_t,
    pub pcap_ifdrop: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_send {
    pub start: libc::c_double,
    pub finish: libc::c_double,
    pub packets_sent: uint64_t,
    pub hosts_scanned: uint64_t,
    pub blocklisted: uint64_t,
    pub allowlisted: uint64_t,
    pub warmup: libc::c_int,
    pub complete: libc::c_int,
    pub first_scanned: uint32_t,
    pub max_targets: uint32_t,
    pub sendto_failures: uint32_t,
    pub max_index: uint32_t,
    pub list_of_ips_pbm: *mut *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_conf {
    pub log_level: libc::c_int,
    pub target_port: port_h_t,
    pub source_port_first: port_h_t,
    pub source_port_last: port_h_t,
    pub max_targets: uint32_t,
    pub max_runtime: uint32_t,
    pub max_results: uint32_t,
    pub iface: *mut libc::c_char,
    pub rate: libc::c_int,
    pub bandwidth: uint64_t,
    pub cooldown_secs: libc::c_int,
    pub senders: uint8_t,
    pub batch: uint8_t,
    pub pin_cores_len: uint32_t,
    pub pin_cores: *mut uint32_t,
    pub seed_provided: libc::c_int,
    pub seed: uint64_t,
    pub aes: *mut aesrand_t,
    pub generator: uint32_t,
    pub shard_num: uint16_t,
    pub total_shards: uint16_t,
    pub packet_streams: libc::c_int,
    pub probe_module: *mut probe_module,
    pub output_module_name: *mut libc::c_char,
    pub output_module: *mut output_module,
    pub probe_args: *mut libc::c_char,
    pub probe_ttl: uint8_t,
    pub output_args: *mut libc::c_char,
    pub gw_mac: [macaddr_t; 6],
    pub hw_mac: [macaddr_t; 6],
    pub gw_ip: uint32_t,
    pub gw_mac_set: libc::c_int,
    pub hw_mac_set: libc::c_int,
    pub source_ip_addresses: [in_addr_t; 256],
    pub number_source_ips: uint32_t,
    pub send_ip_pkts: libc::c_int,
    pub output_filename: *mut libc::c_char,
    pub blocklist_filename: *mut libc::c_char,
    pub allowlist_filename: *mut libc::c_char,
    pub list_of_ips_filename: *mut libc::c_char,
    pub list_of_ips_count: uint32_t,
    pub metadata_filename: *mut libc::c_char,
    pub metadata_file: *mut FILE,
    pub notes: *mut libc::c_char,
    pub custom_metadata_str: *mut libc::c_char,
    pub destination_cidrs: *mut *mut libc::c_char,
    pub destination_cidrs_len: libc::c_int,
    pub raw_output_fields: *const libc::c_char,
    pub output_fields: *mut *const libc::c_char,
    pub filter: output_filter,
    pub output_filter_str: *mut libc::c_char,
    pub fsconf: fieldset_conf,
    pub output_fields_len: libc::c_int,
    pub log_file: *mut libc::c_char,
    pub log_directory: *mut libc::c_char,
    pub status_updates_file: *mut libc::c_char,
    pub dryrun: libc::c_int,
    pub quiet: libc::c_int,
    pub ignore_invalid_hosts: libc::c_int,
    pub syslog: libc::c_int,
    pub recv_ready: libc::c_int,
    pub num_retries: libc::c_int,
    pub total_allowed: uint64_t,
    pub total_disallowed: uint64_t,
    pub max_sendto_failures: libc::c_int,
    pub min_hitrate: libc::c_float,
    pub data_link_size: libc::c_int,
    pub default_mode: libc::c_int,
    pub no_header_row: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fieldset_conf {
    pub defs: fielddefset_t,
    pub outdefs: fielddefset_t,
    pub translation: translation_t,
    pub success_index: libc::c_int,
    pub app_success_index: libc::c_int,
    pub classification_index: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output_module {
    pub name: *const libc::c_char,
    pub supports_dynamic_output: libc::c_int,
    pub update_interval: libc::c_uint,
    pub init: Option::<
        unsafe extern "C" fn(
            *mut state_conf,
            *mut *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub start: Option::<
        unsafe extern "C" fn(
            *mut state_conf,
            *mut state_send,
            *mut state_recv,
        ) -> libc::c_int,
    >,
    pub update: Option::<
        unsafe extern "C" fn(
            *mut state_conf,
            *mut state_send,
            *mut state_recv,
        ) -> libc::c_int,
    >,
    pub close: Option::<
        unsafe extern "C" fn(
            *mut state_conf,
            *mut state_send,
            *mut state_recv,
        ) -> libc::c_int,
    >,
    pub process_ip: Option::<unsafe extern "C" fn(*mut fieldset_t) -> libc::c_int>,
    pub helptext: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __syscall_slong_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type u_char = __u_char;
pub type __u_char = libc::c_uchar;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct ip {
    #[bitfield(name = "ip_hl", ty = "libc::c_uint", bits = "0..=3")]
    #[bitfield(name = "ip_v", ty = "libc::c_uint", bits = "4..=7")]
    pub ip_hl_ip_v: [u8; 1],
    pub ip_tos: uint8_t,
    pub ip_len: libc::c_ushort,
    pub ip_id: libc::c_ushort,
    pub ip_off: libc::c_ushort,
    pub ip_ttl: uint8_t,
    pub ip_p: uint8_t,
    pub ip_sum: libc::c_ushort,
    pub ip_src: in_addr,
    pub ip_dst: in_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type ipaddr_n_t = uint32_t;
pub type port_n_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yy_buffer_state {
    pub yy_input_file: *mut FILE,
    pub yy_ch_buf: *mut libc::c_char,
    pub yy_buf_pos: *mut libc::c_char,
    pub yy_buf_size: libc::c_int,
    pub yy_n_chars: libc::c_int,
    pub yy_is_our_buffer: libc::c_int,
    pub yy_is_interactive: libc::c_int,
    pub yy_at_bol: libc::c_int,
    pub yy_bs_lineno: libc::c_int,
    pub yy_bs_column: libc::c_int,
    pub yy_fill_buffer: libc::c_int,
    pub yy_buffer_status: libc::c_int,
}
pub type YY_BUFFER_STATE = *mut yy_buffer_state;
pub type __pid_t = libc::c_int;
pub type __caddr_t = *mut libc::c_char;
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
pub type __u8 = libc::c_uchar;
pub type __u16 = libc::c_ushort;
pub type __s32 = libc::c_int;
pub type __u32 = libc::c_uint;
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
pub struct nlmsghdr {
    pub nlmsg_len: __u32,
    pub nlmsg_type: __u16,
    pub nlmsg_flags: __u16,
    pub nlmsg_seq: __u32,
    pub nlmsg_pid: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ndmsg {
    pub ndm_family: __u8,
    pub ndm_pad1: __u8,
    pub ndm_pad2: __u16,
    pub ndm_ifindex: __s32,
    pub ndm_state: __u16,
    pub ndm_flags: __u8,
    pub ndm_type: __u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rtattr {
    pub rta_len: libc::c_ushort,
    pub rta_type: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rtmsg {
    pub rtm_family: libc::c_uchar,
    pub rtm_dst_len: libc::c_uchar,
    pub rtm_src_len: libc::c_uchar,
    pub rtm_tos: libc::c_uchar,
    pub rtm_table: libc::c_uchar,
    pub rtm_protocol: libc::c_uchar,
    pub rtm_scope: libc::c_uchar,
    pub rtm_type: libc::c_uchar,
    pub rtm_flags: libc::c_uint,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shard_state {
    pub packets_sent: uint64_t,
    pub hosts_scanned: uint32_t,
    pub max_hosts: uint32_t,
    pub max_packets: uint32_t,
    pub hosts_blocklisted: uint32_t,
    pub hosts_allowlisted: uint32_t,
    pub packets_failed: uint32_t,
    pub first_scanned: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shard_params {
    pub first: uint64_t,
    pub last: uint64_t,
    pub factor: uint64_t,
    pub modulus: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shard {
    pub state: shard_state,
    pub params: shard_params,
    pub current: uint64_t,
    pub iterations: uint64_t,
    pub thread_id: uint8_t,
    pub cb: Option::<unsafe extern "C" fn(uint8_t, *mut libc::c_void) -> ()>,
    pub arg: *mut libc::c_void,
}
pub type shard_t = shard;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iterator {
    pub cycle: cycle_t,
    pub num_threads: uint8_t,
    pub thread_shards: *mut shard_t,
    pub complete: *mut uint8_t,
    pub mutex: pthread_mutex_t,
    pub curr_threads: uint32_t,
}
pub type iterator_t = iterator;
pub type __suseconds_t = libc::c_long;
pub type time_t = __time_t;
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
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct internal_scan_status {
    pub last_now: libc::c_double,
    pub last_sent: uint64_t,
    pub last_tried_sent: uint64_t,
    pub last_send_failures: uint32_t,
    pub last_recv_net_success: uint32_t,
    pub last_recv_app_success: uint32_t,
    pub last_recv_total: uint32_t,
    pub last_pcap_drop: uint32_t,
    pub min_hitrate_start: libc::c_double,
}
pub type int_status_t = internal_scan_status;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct export_scan_status {
    pub total_sent: uint64_t,
    pub total_tried_sent: uint64_t,
    pub recv_success_unique: uint32_t,
    pub app_recv_success_unique: uint32_t,
    pub total_recv: uint64_t,
    pub complete: uint32_t,
    pub send_threads: uint32_t,
    pub percent_complete: libc::c_double,
    pub hitrate: libc::c_double,
    pub app_hitrate: libc::c_double,
    pub send_rate: libc::c_double,
    pub send_rate_str: [libc::c_char; 20],
    pub send_rate_avg: libc::c_double,
    pub send_rate_avg_str: [libc::c_char; 20],
    pub recv_rate: libc::c_double,
    pub recv_rate_str: [libc::c_char; 20],
    pub recv_avg: libc::c_double,
    pub recv_avg_str: [libc::c_char; 20],
    pub recv_total_rate: libc::c_double,
    pub recv_total_avg: libc::c_double,
    pub app_success_rate: libc::c_double,
    pub app_success_rate_str: [libc::c_char; 20],
    pub app_success_avg: libc::c_double,
    pub app_success_avg_str: [libc::c_char; 20],
    pub pcap_drop: uint32_t,
    pub pcap_ifdrop: uint32_t,
    pub pcap_drop_total: uint32_t,
    pub pcap_drop_total_str: [libc::c_char; 20],
    pub pcap_drop_last: libc::c_double,
    pub pcap_drop_last_str: [libc::c_char; 20],
    pub pcap_drop_avg: libc::c_double,
    pub pcap_drop_avg_str: [libc::c_char; 20],
    pub time_remaining: uint32_t,
    pub time_remaining_str: [libc::c_char; 20],
    pub time_past: uint32_t,
    pub time_past_str: [libc::c_char; 20],
    pub fail_total: uint32_t,
    pub fail_avg: libc::c_double,
    pub fail_last: libc::c_double,
    pub seconds_under_min_hitrate: libc::c_float,
}
pub type export_status_t = export_scan_status;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ether_header {
    pub ether_dhost: [uint8_t; 6],
    pub ether_shost: [uint8_t; 6],
    pub ether_type: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_sock_t_1072026035 {
    pub sock: libc::c_int,
}
pub type sock_t = __anonstruct_sock_t_1072026035;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_ll {
    pub sll_family: libc::c_ushort,
    pub sll_protocol: libc::c_ushort,
    pub sll_ifindex: libc::c_int,
    pub sll_hatype: libc::c_ushort,
    pub sll_pkttype: libc::c_uchar,
    pub sll_halen: libc::c_uchar,
    pub sll_addr: [libc::c_uchar; 8],
}
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type uint = libc::c_uint;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
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
pub struct bl_cidr_node {
    pub ip_address: uint32_t,
    pub prefix_len: libc::c_int,
    pub next: *mut bl_cidr_node,
}
pub type bl_cidr_node_t = bl_cidr_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
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
pub type LogLevel = libc::c_uint;
pub const ZNUM_LOGLEVELS: LogLevel = 6;
pub const ZLOG_TRACE: LogLevel = 5;
pub const ZLOG_DEBUG: LogLevel = 4;
pub const ZLOG_INFO: LogLevel = 3;
pub const ZLOG_WARN: LogLevel = 2;
pub const ZLOG_ERROR: LogLevel = 1;
pub const ZLOG_FATAL: LogLevel = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gengetopt_args_info {
    pub target_port_arg: libc::c_int,
    pub target_port_orig: *mut libc::c_char,
    pub target_port_help: *const libc::c_char,
    pub output_file_arg: *mut libc::c_char,
    pub output_file_orig: *mut libc::c_char,
    pub output_file_help: *const libc::c_char,
    pub blocklist_file_arg: *mut libc::c_char,
    pub blocklist_file_orig: *mut libc::c_char,
    pub blocklist_file_help: *const libc::c_char,
    pub allowlist_file_arg: *mut libc::c_char,
    pub allowlist_file_orig: *mut libc::c_char,
    pub allowlist_file_help: *const libc::c_char,
    pub list_of_ips_file_arg: *mut libc::c_char,
    pub list_of_ips_file_orig: *mut libc::c_char,
    pub list_of_ips_file_help: *const libc::c_char,
    pub rate_arg: libc::c_int,
    pub rate_orig: *mut libc::c_char,
    pub rate_help: *const libc::c_char,
    pub bandwidth_arg: *mut libc::c_char,
    pub bandwidth_orig: *mut libc::c_char,
    pub bandwidth_help: *const libc::c_char,
    pub batch_arg: libc::c_int,
    pub batch_orig: *mut libc::c_char,
    pub batch_help: *const libc::c_char,
    pub max_targets_arg: *mut libc::c_char,
    pub max_targets_orig: *mut libc::c_char,
    pub max_targets_help: *const libc::c_char,
    pub max_runtime_arg: libc::c_int,
    pub max_runtime_orig: *mut libc::c_char,
    pub max_runtime_help: *const libc::c_char,
    pub max_results_arg: libc::c_int,
    pub max_results_orig: *mut libc::c_char,
    pub max_results_help: *const libc::c_char,
    pub probes_arg: libc::c_int,
    pub probes_orig: *mut libc::c_char,
    pub probes_help: *const libc::c_char,
    pub cooldown_time_arg: libc::c_int,
    pub cooldown_time_orig: *mut libc::c_char,
    pub cooldown_time_help: *const libc::c_char,
    pub seed_arg: libc::c_long,
    pub seed_orig: *mut libc::c_char,
    pub seed_help: *const libc::c_char,
    pub retries_arg: libc::c_int,
    pub retries_orig: *mut libc::c_char,
    pub retries_help: *const libc::c_char,
    pub dryrun_help: *const libc::c_char,
    pub shards_arg: libc::c_int,
    pub shards_orig: *mut libc::c_char,
    pub shards_help: *const libc::c_char,
    pub shard_arg: libc::c_int,
    pub shard_orig: *mut libc::c_char,
    pub shard_help: *const libc::c_char,
    pub source_port_arg: *mut libc::c_char,
    pub source_port_orig: *mut libc::c_char,
    pub source_port_help: *const libc::c_char,
    pub source_ip_arg: *mut libc::c_char,
    pub source_ip_orig: *mut libc::c_char,
    pub source_ip_help: *const libc::c_char,
    pub gateway_mac_arg: *mut libc::c_char,
    pub gateway_mac_orig: *mut libc::c_char,
    pub gateway_mac_help: *const libc::c_char,
    pub source_mac_arg: *mut libc::c_char,
    pub source_mac_orig: *mut libc::c_char,
    pub source_mac_help: *const libc::c_char,
    pub interface_arg: *mut libc::c_char,
    pub interface_orig: *mut libc::c_char,
    pub interface_help: *const libc::c_char,
    pub iplayer_help: *const libc::c_char,
    pub probe_module_arg: *mut libc::c_char,
    pub probe_module_orig: *mut libc::c_char,
    pub probe_module_help: *const libc::c_char,
    pub probe_args_arg: *mut libc::c_char,
    pub probe_args_orig: *mut libc::c_char,
    pub probe_args_help: *const libc::c_char,
    pub probe_ttl_arg: libc::c_int,
    pub probe_ttl_orig: *mut libc::c_char,
    pub probe_ttl_help: *const libc::c_char,
    pub list_probe_modules_help: *const libc::c_char,
    pub output_fields_arg: *mut libc::c_char,
    pub output_fields_orig: *mut libc::c_char,
    pub output_fields_help: *const libc::c_char,
    pub output_module_arg: *mut libc::c_char,
    pub output_module_orig: *mut libc::c_char,
    pub output_module_help: *const libc::c_char,
    pub output_args_arg: *mut libc::c_char,
    pub output_args_orig: *mut libc::c_char,
    pub output_args_help: *const libc::c_char,
    pub output_filter_arg: *mut libc::c_char,
    pub output_filter_orig: *mut libc::c_char,
    pub output_filter_help: *const libc::c_char,
    pub list_output_modules_help: *const libc::c_char,
    pub list_output_fields_help: *const libc::c_char,
    pub no_header_row_help: *const libc::c_char,
    pub verbosity_arg: libc::c_int,
    pub verbosity_orig: *mut libc::c_char,
    pub verbosity_help: *const libc::c_char,
    pub log_file_arg: *mut libc::c_char,
    pub log_file_orig: *mut libc::c_char,
    pub log_file_help: *const libc::c_char,
    pub log_directory_arg: *mut libc::c_char,
    pub log_directory_orig: *mut libc::c_char,
    pub log_directory_help: *const libc::c_char,
    pub metadata_file_arg: *mut libc::c_char,
    pub metadata_file_orig: *mut libc::c_char,
    pub metadata_file_help: *const libc::c_char,
    pub status_updates_file_arg: *mut libc::c_char,
    pub status_updates_file_orig: *mut libc::c_char,
    pub status_updates_file_help: *const libc::c_char,
    pub quiet_help: *const libc::c_char,
    pub disable_syslog_help: *const libc::c_char,
    pub notes_arg: *mut libc::c_char,
    pub notes_orig: *mut libc::c_char,
    pub notes_help: *const libc::c_char,
    pub user_metadata_arg: *mut libc::c_char,
    pub user_metadata_orig: *mut libc::c_char,
    pub user_metadata_help: *const libc::c_char,
    pub config_arg: *mut libc::c_char,
    pub config_orig: *mut libc::c_char,
    pub config_help: *const libc::c_char,
    pub max_sendto_failures_arg: libc::c_int,
    pub max_sendto_failures_orig: *mut libc::c_char,
    pub max_sendto_failures_help: *const libc::c_char,
    pub min_hitrate_arg: libc::c_float,
    pub min_hitrate_orig: *mut libc::c_char,
    pub min_hitrate_help: *const libc::c_char,
    pub sender_threads_arg: libc::c_int,
    pub sender_threads_orig: *mut libc::c_char,
    pub sender_threads_help: *const libc::c_char,
    pub cores_arg: *mut libc::c_char,
    pub cores_orig: *mut libc::c_char,
    pub cores_help: *const libc::c_char,
    pub ignore_blocklist_errors_help: *const libc::c_char,
    pub help_help: *const libc::c_char,
    pub version_help: *const libc::c_char,
    pub target_port_given: libc::c_uint,
    pub output_file_given: libc::c_uint,
    pub blocklist_file_given: libc::c_uint,
    pub allowlist_file_given: libc::c_uint,
    pub list_of_ips_file_given: libc::c_uint,
    pub rate_given: libc::c_uint,
    pub bandwidth_given: libc::c_uint,
    pub batch_given: libc::c_uint,
    pub max_targets_given: libc::c_uint,
    pub max_runtime_given: libc::c_uint,
    pub max_results_given: libc::c_uint,
    pub probes_given: libc::c_uint,
    pub cooldown_time_given: libc::c_uint,
    pub seed_given: libc::c_uint,
    pub retries_given: libc::c_uint,
    pub dryrun_given: libc::c_uint,
    pub shards_given: libc::c_uint,
    pub shard_given: libc::c_uint,
    pub source_port_given: libc::c_uint,
    pub source_ip_given: libc::c_uint,
    pub gateway_mac_given: libc::c_uint,
    pub source_mac_given: libc::c_uint,
    pub interface_given: libc::c_uint,
    pub iplayer_given: libc::c_uint,
    pub probe_module_given: libc::c_uint,
    pub probe_args_given: libc::c_uint,
    pub probe_ttl_given: libc::c_uint,
    pub list_probe_modules_given: libc::c_uint,
    pub output_fields_given: libc::c_uint,
    pub output_module_given: libc::c_uint,
    pub output_args_given: libc::c_uint,
    pub output_filter_given: libc::c_uint,
    pub list_output_modules_given: libc::c_uint,
    pub list_output_fields_given: libc::c_uint,
    pub no_header_row_given: libc::c_uint,
    pub verbosity_given: libc::c_uint,
    pub log_file_given: libc::c_uint,
    pub log_directory_given: libc::c_uint,
    pub metadata_file_given: libc::c_uint,
    pub status_updates_file_given: libc::c_uint,
    pub quiet_given: libc::c_uint,
    pub disable_syslog_given: libc::c_uint,
    pub notes_given: libc::c_uint,
    pub user_metadata_given: libc::c_uint,
    pub config_given: libc::c_uint,
    pub max_sendto_failures_given: libc::c_uint,
    pub min_hitrate_given: libc::c_uint,
    pub sender_threads_given: libc::c_uint,
    pub cores_given: libc::c_uint,
    pub ignore_blocklist_errors_given: libc::c_uint,
    pub help_given: libc::c_uint,
    pub version_given: libc::c_uint,
    pub inputs: *mut *mut libc::c_char,
    pub inputs_num: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmdline_parser_params {
    pub override_0: libc::c_int,
    pub initialize: libc::c_int,
    pub check_required: libc::c_int,
    pub check_ambiguity: libc::c_int,
    pub print_errors: libc::c_int,
}
pub type output_module_t = output_module;
pub type probe_module_t = probe_module;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct send_arg {
    pub cpu: uint32_t,
    pub sock: sock_t,
    pub shard: *mut shard_t,
}
pub type send_arg_t = send_arg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct recv_arg {
    pub cpu: uint32_t,
}
pub type recv_arg_t = recv_arg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mon_start_arg {
    pub cpu: uint32_t,
    pub it: *mut iterator_t,
    pub recv_ready_mutex: *mut pthread_mutex_t,
}
pub type mon_start_arg_t = mon_start_arg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type __anonenum_cmdline_parser_arg_type_225089629 = libc::c_uint;
pub const ARG_LONGLONG: __anonenum_cmdline_parser_arg_type_225089629 = 4;
pub const ARG_FLOAT: __anonenum_cmdline_parser_arg_type_225089629 = 3;
pub const ARG_INT: __anonenum_cmdline_parser_arg_type_225089629 = 2;
pub const ARG_STRING: __anonenum_cmdline_parser_arg_type_225089629 = 1;
pub const ARG_NO: __anonenum_cmdline_parser_arg_type_225089629 = 0;
pub type cmdline_parser_arg_type = __anonenum_cmdline_parser_arg_type_225089629;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line_list {
    pub string_arg: *mut libc::c_char,
    pub next: *mut line_list,
}
pub type __int16_t = libc::c_short;
pub type int16_t = __int16_t;
pub type flex_uint8_t = uint8_t;
pub type flex_int16_t = int16_t;
pub type yy_size_t = size_t;
pub type YY_CHAR = flex_uint8_t;
pub type yy_state_type = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_YYSTYPE_714356573 {
    pub int_literal: libc::c_int,
    pub string_literal: *mut libc::c_char,
    pub expr: *mut node_st,
}
pub type YYSTYPE = __anonunion_YYSTYPE_714356573;
pub type YYINT = libc::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_YYSTACKDATA_590319461 {
    pub stacksize: libc::c_uint,
    pub s_base: *mut YYINT,
    pub s_mark: *mut YYINT,
    pub s_last: *mut YYINT,
    pub l_base: *mut YYSTYPE,
    pub l_mark: *mut YYSTYPE,
}
pub type YYSTACKDATA = __anonstruct_YYSTACKDATA_590319461;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icmp_ra_addr {
    pub ira_addr: uint32_t,
    pub ira_preference: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ih_idseq {
    pub icd_id: uint16_t,
    pub icd_seq: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ih_pmtu {
    pub ipm_void: uint16_t,
    pub ipm_nextmtu: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ih_rtradv {
    pub irt_num_addrs: uint8_t,
    pub irt_wpa: uint8_t,
    pub irt_lifetime: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_icmp_hun_562310239 {
    pub ih_pptr: libc::c_uchar,
    pub ih_gwaddr: in_addr,
    pub ih_idseq: ih_idseq,
    pub ih_void: uint32_t,
    pub ih_pmtu: ih_pmtu,
    pub ih_rtradv: ih_rtradv,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_id_ts_669259541 {
    pub its_otime: uint32_t,
    pub its_rtime: uint32_t,
    pub its_ttime: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_id_ip_706470303 {
    pub idi_ip: ip,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_icmp_dun_1026239311 {
    pub id_ts: __anonstruct_id_ts_669259541,
    pub id_ip: __anonstruct_id_ip_706470303,
    pub id_radv: icmp_ra_addr,
    pub id_mask: uint32_t,
    pub id_data: [uint8_t; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icmp {
    pub icmp_type: uint8_t,
    pub icmp_code: uint8_t,
    pub icmp_cksum: uint16_t,
    pub icmp_hun: __anonunion_icmp_hun_562310239,
    pub icmp_dun: __anonunion_icmp_dun_1026239311,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icmp_payload_for_rtt {
    pub sent_tv_sec: uint32_t,
    pub sent_tv_usec: uint32_t,
    pub dst: ipaddr_n_t,
}
pub type tcp_seq = uint32_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct __anonstruct____missing_field_name_112923541 {
    pub th_sport: uint16_t,
    pub th_dport: uint16_t,
    pub th_seq: tcp_seq,
    pub th_ack: tcp_seq,
    #[bitfield(name = "th_x2", ty = "uint8_t", bits = "0..=3")]
    #[bitfield(name = "th_off", ty = "uint8_t", bits = "4..=7")]
    pub th_x2_th_off: [u8; 1],
    pub th_flags: uint8_t,
    pub th_win: uint16_t,
    pub th_sum: uint16_t,
    pub th_urp: uint16_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct __anonstruct____missing_field_name_1013361302 {
    pub source: uint16_t,
    pub dest: uint16_t,
    pub seq: uint32_t,
    pub ack_seq: uint32_t,
    #[bitfield(name = "res1", ty = "uint16_t", bits = "0..=3")]
    #[bitfield(name = "doff", ty = "uint16_t", bits = "4..=7")]
    #[bitfield(name = "fin", ty = "uint16_t", bits = "8..=8")]
    #[bitfield(name = "syn", ty = "uint16_t", bits = "9..=9")]
    #[bitfield(name = "rst", ty = "uint16_t", bits = "10..=10")]
    #[bitfield(name = "psh", ty = "uint16_t", bits = "11..=11")]
    #[bitfield(name = "ack", ty = "uint16_t", bits = "12..=12")]
    #[bitfield(name = "urg", ty = "uint16_t", bits = "13..=13")]
    #[bitfield(name = "res2", ty = "uint16_t", bits = "14..=15")]
    pub res1_doff_fin_syn_rst_psh_ack_urg_res2: [u8; 2],
    pub window: uint16_t,
    pub check: uint16_t,
    pub urg_ptr: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_588722493 {
    pub __annonCompField6: __anonstruct____missing_field_name_112923541,
    pub __annonCompField7: __anonstruct____missing_field_name_1013361302,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tcphdr {
    pub __annonCompField8: __anonunion____missing_field_name_588722493,
}
pub type alias_unsigned_short = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct____missing_field_name_518125691 {
    pub uh_sport: uint16_t,
    pub uh_dport: uint16_t,
    pub uh_ulen: uint16_t,
    pub uh_sum: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct____missing_field_name_518125692 {
    pub source: uint16_t,
    pub dest: uint16_t,
    pub len: uint16_t,
    pub check: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_539173916 {
    pub __annonCompField3: __anonstruct____missing_field_name_518125691,
    pub __annonCompField4: __anonstruct____missing_field_name_518125692,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct udphdr {
    pub __annonCompField5: __anonunion____missing_field_name_539173916,
}
pub type udp_payload_field_type = libc::c_uint;
pub const UDP_RAND_ALPHANUM: udp_payload_field_type = 12;
pub const UDP_RAND_ALPHA: udp_payload_field_type = 11;
pub const UDP_RAND_DIGIT: udp_payload_field_type = 10;
pub const UDP_RAND_BYTE: udp_payload_field_type = 9;
pub const UDP_DPORT_A: udp_payload_field_type = 8;
pub const UDP_DPORT_N: udp_payload_field_type = 7;
pub const UDP_SPORT_A: udp_payload_field_type = 6;
pub const UDP_SPORT_N: udp_payload_field_type = 5;
pub const UDP_DADDR_A: udp_payload_field_type = 4;
pub const UDP_DADDR_N: udp_payload_field_type = 3;
pub const UDP_SADDR_A: udp_payload_field_type = 2;
pub const UDP_SADDR_N: udp_payload_field_type = 1;
pub const UDP_DATA: udp_payload_field_type = 0;
pub type udp_payload_field_type_t = udp_payload_field_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct udp_payload_field_type_def {
    pub name: *const libc::c_char,
    pub desc: *const libc::c_char,
    pub max_length: size_t,
    pub ftype: udp_payload_field_type_t,
}
pub type udp_payload_field_type_def_t = udp_payload_field_type_def;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct udp_payload_field {
    pub ftype: udp_payload_field_type,
    pub length: size_t,
    pub data: *mut libc::c_char,
}
pub type udp_payload_field_t = udp_payload_field;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct udp_payload_template {
    pub fcount: libc::c_uint,
    pub fields: *mut *mut udp_payload_field,
}
pub type udp_payload_template_t = udp_payload_template;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ntphdr {
    pub LI_VN_MODE: uint8_t,
    pub stratum: uint8_t,
    pub poll: uint8_t,
    pub precision: uint8_t,
    pub root_delay: uint32_t,
    pub root_dispersion: uint32_t,
    pub ref_ID: uint32_t,
    pub reference_timestamp: uint64_t,
    pub origin_timestamp: uint64_t,
    pub receive_timestamp: uint64_t,
    pub transmit_timestamp: uint64_t,
}
pub type __int8_t = libc::c_schar;
pub type int8_t = __int8_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C, packed)]
pub struct __anonstruct_dns_header_753342901 {
    pub id: uint16_t,
    #[bitfield(name = "rd", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "tc", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "aa", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "opcode", ty = "libc::c_uint", bits = "3..=6")]
    #[bitfield(name = "qr", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "rcode", ty = "libc::c_uint", bits = "8..=11")]
    #[bitfield(name = "cd", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "ad", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "z", ty = "libc::c_uint", bits = "14..=14")]
    #[bitfield(name = "ra", ty = "libc::c_uint", bits = "15..=15")]
    pub rd_tc_aa_opcode_qr_rcode_cd_ad_z_ra: [u8; 2],
    pub qdcount: uint16_t,
    pub ancount: uint16_t,
    pub nscount: uint16_t,
    pub arcount: uint16_t,
}
pub type dns_header = __anonstruct_dns_header_753342901;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct __anonstruct_dns_question_tail_753912357 {
    pub qtype: uint16_t,
    pub qclass: uint16_t,
}
pub type dns_question_tail = __anonstruct_dns_question_tail_753912357;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct __anonstruct_dns_answer_tail_215550957 {
    pub type_0: uint16_t,
    pub class: uint16_t,
    pub ttl: uint32_t,
    pub rdlength: uint16_t,
    pub rdata: [libc::c_char; 0],
}
pub type dns_answer_tail = __anonstruct_dns_answer_tail_215550957;
pub type __anonenum_dns_qtype_632227127 = libc::c_uint;
pub const DNS_QTYPE_ALL: __anonenum_dns_qtype_632227127 = 255;
pub const DNS_QTYPE_RRSIG: __anonenum_dns_qtype_632227127 = 46;
pub const DNS_QTYPE_AAAA: __anonenum_dns_qtype_632227127 = 28;
pub const DNS_QTYPE_TXT: __anonenum_dns_qtype_632227127 = 16;
pub const DNS_QTYPE_MX: __anonenum_dns_qtype_632227127 = 15;
pub const DNS_QTYPE_PTR: __anonenum_dns_qtype_632227127 = 12;
pub const DNS_QTYPE_SOA: __anonenum_dns_qtype_632227127 = 6;
pub const DNS_QTYPE_CNAME: __anonenum_dns_qtype_632227127 = 5;
pub const DNS_QTYPE_NS: __anonenum_dns_qtype_632227127 = 2;
pub const DNS_QTYPE_A: __anonenum_dns_qtype_632227127 = 1;
pub type dns_qtype = __anonenum_dns_qtype_632227127;
pub type bool_0 = uint8_t;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct bacnet_vlc {
    pub type_0: uint8_t,
    pub function: uint8_t,
    pub length: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct bacnet_npdu {
    pub version: uint8_t,
    pub control: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct bacnet_apdu {
    pub type_flags: uint8_t,
    pub max_segments_apdu: uint8_t,
    pub invoke_id: uint8_t,
    pub server_choice: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct bacnet_probe {
    pub vlc: bacnet_vlc,
    pub npdu: bacnet_npdu,
    pub apdu: bacnet_apdu,
}
pub type json_bool = libc::c_int;
pub type __u_short = libc::c_ushort;
pub type __u_int = libc::c_uint;
pub type __useconds_t = libc::c_uint;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type bpf_u_int32 = u_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bpf_insn {
    pub code: u_short,
    pub jt: u_char,
    pub jf: u_char,
    pub k: bpf_u_int32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bpf_program {
    pub bf_len: u_int,
    pub bf_insns: *mut bpf_insn,
}
pub type pcap_t = pcap;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pcap_pkthdr {
    pub ts: timeval,
    pub caplen: bpf_u_int32,
    pub len: bpf_u_int32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pcap_stat {
    pub ps_recv: u_int,
    pub ps_drop: u_int,
    pub ps_ifdrop: u_int,
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
unsafe extern "C" fn atoll(mut __nptr: *const libc::c_char) -> libc::c_longlong {
    let mut tmp: libc::c_longlong = 0;
    tmp = strtoll(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    return tmp;
}
#[inline(always)]
unsafe extern "C" fn fgets(
    mut __s: *mut libc::c_char,
    mut __n: libc::c_int,
    mut __stream: *mut FILE,
) -> *mut libc::c_char {
    let mut tmp: libc::c_ulong = 0;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: libc::c_ulong = 0;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: libc::c_ulong = 0;
    let mut tmp___4: libc::c_ulong = 0;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp___4 = (if 1 as libc::c_int & 2 == 0 { -1isize } else { 0isize }) as _;
    if tmp___4 != 0xffffffffffffffff as libc::c_ulong {
        tmp = (if 1 as libc::c_int & 2 == 0 { -1isize } else { 0isize }) as _;
        tmp___0 = __fgets_chk(__s, tmp, __n, __stream);
        return tmp___0;
    }
    tmp___5 = __fgets_alias(__s, __n, __stream);
    return tmp___5;
}
#[inline(always)]
unsafe extern "C" fn fread(
    mut __ptr: *mut libc::c_void,
    mut __size: size_t,
    mut __n: size_t,
    mut __stream: *mut FILE,
) -> libc::c_ulong {
    let mut tmp: libc::c_ulong = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: libc::c_ulong = 0;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: libc::c_ulong = 0;
    let mut tmp___4: libc::c_ulong = 0;
    let mut tmp___5: size_t = 0;
    tmp___4 = (if 0 as libc::c_int & 2 == 0 { -1isize } else { 0isize }) as _;
    if tmp___4 != 0xffffffffffffffff as libc::c_ulong {
        tmp = (if 0 as libc::c_int & 2 == 0 { -1isize } else { 0isize }) as _;
        tmp___0 = __fread_chk(__ptr, tmp, __size, __n, __stream);
        return tmp___0;
    }
    tmp___5 = __fread_alias(__ptr, __size, __n, __stream);
    return tmp___5;
}
unsafe extern "C" fn _aesrand_init(mut key: *mut uint8_t) -> *mut aesrand_t {
    let mut aes: *mut aesrand_t = 0 as *mut aesrand_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    tmp = xmalloc(::std::mem::size_of::<aesrand_t>() as libc::c_ulong);
    aes = tmp as *mut aesrand_t;
    memset(
        &mut (*aes).input as *mut [uint32_t; 4] as *mut libc::c_void,
        0 as libc::c_int,
        16 as libc::c_int as size_t as _,
    );
    tmp___0 = rijndaelKeySetupEnc(
        ((*aes).sched).as_mut_ptr(),
        key as *const u8_0,
        128 as libc::c_int,
    );
    if tmp___0 != 10 as libc::c_int {
        log_fatal(
            b"aesrand\0" as *const u8 as *const libc::c_char,
            b"could not initialize AES key\0" as *const u8 as *const libc::c_char,
        );
    }
    memset(
        ((*aes).output).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        16 as libc::c_int as size_t as _,
    );
    return aes;
}
pub unsafe extern "C" fn aesrand_init_from_seed(mut seed: uint64_t) -> *mut aesrand_t {
    let mut key: [uint8_t; 16] = [0; 16];
    let mut i: uint8_t = 0;
    let mut tmp: *mut aesrand_t = 0 as *mut aesrand_t;
    memset(
        key.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        16 as libc::c_int as size_t as _,
    );
    i = 0 as libc::c_int as uint8_t;
    while (i as libc::c_ulong) < ::std::mem::size_of::<uint64_t>() as libc::c_ulong {
        key[i
            as usize] = (seed >> 8 as libc::c_int * i as libc::c_int
            & 255 as libc::c_ulong) as uint8_t;
        i = (i as libc::c_int + 1 as libc::c_int) as uint8_t;
    }
    tmp = _aesrand_init(key.as_mut_ptr());
    return tmp;
}
pub unsafe extern "C" fn aesrand_init_from_random() -> *mut aesrand_t {
    let mut key: [uint8_t; 16] = [0; 16];
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut aesrand_t = 0 as *mut aesrand_t;
    tmp = random_bytes(
        key.as_mut_ptr() as *mut libc::c_void,
        16 as libc::c_int as size_t,
    );
    if tmp == 0 {
        log_fatal(
            b"aesrand\0" as *const u8 as *const libc::c_char,
            b"Couldn't get random bytes\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___0 = _aesrand_init(key.as_mut_ptr());
    return tmp___0;
}
pub unsafe extern "C" fn aesrand_getword(mut aes: *mut aesrand_t) -> uint64_t {
    let mut retval: uint64_t = 0;
    memcpy(
        ((*aes).input).as_mut_ptr() as *mut libc::c_void,
        ((*aes).output).as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[uint32_t; 4]>() as libc::c_ulong as _,
    );
    rijndaelEncrypt(
        ((*aes).sched).as_mut_ptr() as *const u32_0,
        10 as libc::c_int,
        ((*aes).input).as_mut_ptr() as *mut uint8_t as *const u8_0,
        ((*aes).output).as_mut_ptr() as *mut u8_0,
    );
    memcpy(
        &mut retval as *mut uint64_t as *mut libc::c_void,
        ((*aes).output).as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<uint64_t>() as libc::c_ulong as _,
    );
    return retval;
}
#[inline]
unsafe extern "C" fn __gmpz_get_ui(mut __gmp_z: mpz_srcptr) -> libc::c_ulong {
    let mut __gmp_p: mp_ptr = 0 as *mut mp_limb_t;
    let mut __gmp_n: mp_size_t = 0;
    let mut __gmp_l: mp_limb_t = 0;
    let mut tmp: mp_limb_t = 0;
    __gmp_p = (*__gmp_z)._mp_d;
    __gmp_n = (*__gmp_z)._mp_size as mp_size_t;
    __gmp_l = *__gmp_p.offset(0 as libc::c_int as isize);
    if __gmp_n != 0 as libc::c_long {
        tmp = __gmp_l;
    } else {
        tmp = 0 as libc::c_int as mp_limb_t;
    }
    return tmp;
}
#[inline(always)]
unsafe extern "C" fn recv(
    mut __fd: libc::c_int,
    mut __buf: *mut libc::c_void,
    mut __n: size_t,
    mut __flags: libc::c_int,
) -> ssize_t {
    let mut tmp: libc::c_ulong = 0;
    let mut tmp___0: ssize_t = 0;
    let mut tmp___1: libc::c_ulong = 0;
    let mut tmp___2: ssize_t = 0;
    let mut tmp___3: libc::c_ulong = 0;
    let mut tmp___4: libc::c_ulong = 0;
    let mut tmp___5: ssize_t = 0;
    tmp___4 = (if 0 as libc::c_int & 2 == 0 { -1isize } else { 0isize }) as libc::size_t as _;
    if tmp___4 != 0xffffffffffffffff as libc::c_ulong {
        tmp = (if 0 as libc::c_int & 2 == 0 { -1isize } else { 0isize }) as libc::size_t as _;
        tmp___0 = __recv_chk(__fd, __buf, __n, tmp, __flags);
        return tmp___0;
    }
    tmp___5 = __recv_alias(__fd, __buf, __n, __flags);
    return tmp___5;
}
static mut groups: [cyclic_group_t; 5] = [
    {
        let mut init = cyclic_group {
            prime: 257 as libc::c_int as uint64_t,
            known_primroot: 3 as libc::c_int as uint64_t,
            num_prime_factors: 1 as libc::c_int as size_t,
            prime_factors: [2 as libc::c_int as uint64_t, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        init
    },
    {
        let mut init = cyclic_group {
            prime: 65537 as libc::c_int as uint64_t,
            known_primroot: 3 as libc::c_int as uint64_t,
            num_prime_factors: 1 as libc::c_int as size_t,
            prime_factors: [2 as libc::c_int as uint64_t, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        init
    },
    {
        let mut init = cyclic_group {
            prime: 16777259 as libc::c_int as uint64_t,
            known_primroot: 2 as libc::c_int as uint64_t,
            num_prime_factors: 4 as libc::c_int as size_t,
            prime_factors: [
                2 as libc::c_int as uint64_t,
                23 as libc::c_int as uint64_t,
                103 as libc::c_int as uint64_t,
                3541 as libc::c_int as uint64_t,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        };
        init
    },
    {
        let mut init = cyclic_group {
            prime: 268435459 as libc::c_int as uint64_t,
            known_primroot: 2 as libc::c_int as uint64_t,
            num_prime_factors: 4 as libc::c_int as size_t,
            prime_factors: [
                2 as libc::c_int as uint64_t,
                3 as libc::c_int as uint64_t,
                19 as libc::c_int as uint64_t,
                87211 as libc::c_int as uint64_t,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        };
        init
    },
    {
        let mut init = cyclic_group {
            prime: 4294967311 as libc::c_long as uint64_t,
            known_primroot: 3 as libc::c_int as uint64_t,
            num_prime_factors: 5 as libc::c_int as size_t,
            prime_factors: [
                2 as libc::c_int as uint64_t,
                3 as libc::c_int as uint64_t,
                5 as libc::c_int as uint64_t,
                131 as libc::c_int as uint64_t,
                364289 as libc::c_int as uint64_t,
                0,
                0,
                0,
                0,
                0,
            ],
        };
        init
    },
];
unsafe extern "C" fn check_coprime(
    mut check: uint64_t,
    mut group: *const cyclic_group_t,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    if check == 0 as libc::c_ulong {
        return 0 as libc::c_int
    } else {
        if check == 1 as libc::c_ulong {
            return 0 as libc::c_int;
        }
    }
    i = 0 as libc::c_uint;
    while (i as size_t) < (*group).num_prime_factors {
        if (*group).prime_factors[i as usize] > check {
            if ((*group).prime_factors[i as usize]).wrapping_rem(check) == 0 {
                return 0 as libc::c_int;
            }
        }
        if (*group).prime_factors[i as usize] < check {
            if check.wrapping_rem((*group).prime_factors[i as usize]) == 0 {
                return 0 as libc::c_int;
            }
        }
        if (*group).prime_factors[i as usize] == check {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn find_primroot(
    mut group: *const cyclic_group_t,
    mut aes: *mut aesrand_t,
) -> uint32_t {
    let mut candidate: uint32_t = 0;
    let mut tmp: uint64_t = 0;
    let mut retv: uint64_t = 0;
    let mut max_root: uint64_t = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = aesrand_getword(aes);
    candidate = (tmp & 4294967295 as libc::c_ulong).wrapping_rem((*group).prime)
        as uint32_t;
    retv = 0 as libc::c_int as uint64_t;
    max_root = ((1 as libc::c_ulong) << 32 as libc::c_int)
        .wrapping_sub(14 as libc::c_ulong);
    loop {
        loop {
            tmp___0 = check_coprime(candidate as uint64_t, group);
            if !(tmp___0 != 1 as libc::c_int) {
                break;
            }
            candidate = candidate.wrapping_add(1);
            candidate = (candidate as libc::c_ulong).wrapping_rem((*group).prime)
                as uint32_t;
        }
        retv = isomorphism(candidate as uint64_t, group);
        if !(retv > max_root) {
            break;
        }
    }
    return retv as uint32_t;
}
pub unsafe extern "C" fn get_group(mut min_size: uint64_t) -> *const cyclic_group_t {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_uint;
    while (i as libc::c_ulong)
        < ::std::mem::size_of::<[cyclic_group_t; 5]>() as libc::c_ulong
    {
        if groups[i as usize].prime > min_size {
            return &mut *groups.as_mut_ptr().offset(i as isize) as *mut cyclic_group_t
                as *const cyclic_group_t;
        }
        i = i.wrapping_add(1);
    }
    __assert_fail(
        b"0\0" as *const u8 as *const libc::c_char,
        b"src/cyclic.c\0" as *const u8 as *const libc::c_char,
        142 as libc::c_uint,
        b"get_group\0" as *const u8 as *const libc::c_char,
    );
}
pub unsafe extern "C" fn make_cycle(
    mut group: *const cyclic_group_t,
    mut aes: *mut aesrand_t,
) -> cycle_t {
    let mut cycle: cycle_t = cycle_t {
        group: 0 as *const cyclic_group_t,
        generator: 0,
        order: 0,
        offset: 0,
    };
    let mut tmp: uint32_t = 0;
    let mut tmp___0: uint64_t = 0;
    cycle.group = group;
    tmp = find_primroot(group, aes);
    cycle.generator = tmp as uint64_t;
    tmp___0 = aesrand_getword(aes);
    cycle.offset = (tmp___0 & 4294967295 as libc::c_ulong) as uint32_t;
    cycle
        .offset = (cycle.offset as libc::c_ulong).wrapping_rem((*group).prime)
        as uint32_t;
    cycle.order = ((*group).prime).wrapping_sub(1 as libc::c_ulong);
    return cycle;
}
pub unsafe extern "C" fn isomorphism(
    mut additive_elt: uint64_t,
    mut mult_group: *const cyclic_group_t,
) -> uint64_t {
    let mut base: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut power: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut prime: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut primroot: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut retv: uint64_t = 0;
    let mut tmp___0: libc::c_ulong = 0;
    if !(additive_elt < (*mult_group).prime) {
        __assert_fail(
            b"additive_elt < mult_group->prime\0" as *const u8 as *const libc::c_char,
            b"src/cyclic.c\0" as *const u8 as *const libc::c_char,
            158 as libc::c_uint,
            b"isomorphism\0" as *const u8 as *const libc::c_char,
        );
    }
    __gmpz_init_set_ui(base.as_mut_ptr(), (*mult_group).known_primroot);
    __gmpz_init_set_ui(power.as_mut_ptr(), additive_elt);
    __gmpz_init_set_ui(prime.as_mut_ptr(), (*mult_group).prime);
    __gmpz_init(primroot.as_mut_ptr());
    __gmpz_powm(
        primroot.as_mut_ptr(),
        base.as_mut_ptr() as mpz_srcptr,
        power.as_mut_ptr() as mpz_srcptr,
        prime.as_mut_ptr() as mpz_srcptr,
    );
    tmp___0 = __gmpz_get_ui(primroot.as_mut_ptr() as mpz_srcptr);
    retv = tmp___0;
    log_debug(
        b"zmap\0" as *const u8 as *const libc::c_char,
        b"Isomorphism: %llu\0" as *const u8 as *const libc::c_char,
        retv,
    );
    __gmpz_clear(base.as_mut_ptr());
    __gmpz_clear(power.as_mut_ptr());
    __gmpz_clear(prime.as_mut_ptr());
    __gmpz_clear(primroot.as_mut_ptr());
    return retv;
}
unsafe extern "C" fn alloc_node() -> *mut node_t {
    let mut node: *mut node_t = 0 as *mut node_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = xmalloc(::std::mem::size_of::<node_t>() as libc::c_ulong);
    node = tmp as *mut node_t;
    return node;
}
unsafe extern "C" fn eval_gt_node(
    mut node: *mut node_t,
    mut fields___8: *mut fieldset_t,
) -> libc::c_int {
    let mut index___0: libc::c_int = 0;
    let mut expected: uint64_t = 0;
    let mut actual: uint64_t = 0;
    let mut tmp: uint64_t = 0;
    index___0 = (*(*node).left_child).value.field.index;
    expected = (*(*node).right_child).value.int_literal;
    tmp = fs_get_uint64_by_index(fields___8, index___0);
    actual = tmp;
    return (actual > expected) as libc::c_int;
}
unsafe extern "C" fn eval_lt_node(
    mut node: *mut node_t,
    mut fields___8: *mut fieldset_t,
) -> libc::c_int {
    let mut index___0: libc::c_int = 0;
    let mut expected: uint64_t = 0;
    let mut actual: uint64_t = 0;
    let mut tmp: uint64_t = 0;
    index___0 = (*(*node).left_child).value.field.index;
    expected = (*(*node).right_child).value.int_literal;
    tmp = fs_get_uint64_by_index(fields___8, index___0);
    actual = tmp;
    return (actual < expected) as libc::c_int;
}
unsafe extern "C" fn eval_eq_node(
    mut node: *mut node_t,
    mut fields___8: *mut fieldset_t,
) -> libc::c_int {
    let mut literal: *mut node_t = 0 as *mut node_t;
    let mut index___0: libc::c_int = 0;
    let mut expected: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut actual: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: uint64_t = 0;
    literal = (*node).right_child;
    index___0 = (*(*node).left_child).value.field.index;
    match (*literal).type_0 as libc::c_uint {
        2 => {
            expected = (*literal).value.string_literal;
            actual = fs_get_string_by_index(fields___8, index___0);
            tmp = strcmp(expected as *const libc::c_char, actual as *const libc::c_char);
            return (tmp == 0 as libc::c_int) as libc::c_int;
        }
        3 => {
            tmp___0 = fs_get_uint64_by_index(fields___8, index___0);
            return (tmp___0 == (*literal).value.int_literal) as libc::c_int;
        }
        _ => {
            __printf_chk(
                1 as libc::c_int,
                b"wat\n\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn eval_lt_eq_node(
    mut node: *mut node_t,
    mut fields___8: *mut fieldset_t,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = eval_gt_node(node, fields___8);
    if tmp != 0 {
        tmp___0 = 0 as libc::c_int;
    } else {
        tmp___0 = 1 as libc::c_int;
    }
    return tmp___0;
}
unsafe extern "C" fn eval_gt_eq_node(
    mut node: *mut node_t,
    mut fields___8: *mut fieldset_t,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = eval_lt_node(node, fields___8);
    if tmp != 0 {
        tmp___0 = 0 as libc::c_int;
    } else {
        tmp___0 = 1 as libc::c_int;
    }
    return tmp___0;
}
pub unsafe extern "C" fn make_op_node(mut op: operation) -> *mut node_t {
    let mut node: *mut node_t = 0 as *mut node_t;
    let mut tmp: *mut node_t = 0 as *mut node_t;
    tmp = alloc_node();
    node = tmp;
    (*node).type_0 = OP;
    (*node).value.op = op;
    return node;
}
pub unsafe extern "C" fn make_field_node(
    mut fieldname: *mut libc::c_char,
) -> *mut node_t {
    let mut node: *mut node_t = 0 as *mut node_t;
    let mut tmp: *mut node_t = 0 as *mut node_t;
    tmp = alloc_node();
    node = tmp;
    (*node).type_0 = FIELD;
    (*node).value.field.fieldname = fieldname;
    return node;
}
pub unsafe extern "C" fn make_string_node(
    mut literal: *mut libc::c_char,
) -> *mut node_t {
    let mut node: *mut node_t = 0 as *mut node_t;
    let mut tmp: *mut node_t = 0 as *mut node_t;
    tmp = alloc_node();
    node = tmp;
    (*node).type_0 = STRING;
    (*node).value.string_literal = literal;
    return node;
}
pub unsafe extern "C" fn make_int_node(mut literal: libc::c_int) -> *mut node_t {
    let mut node: *mut node_t = 0 as *mut node_t;
    let mut tmp: *mut node_t = 0 as *mut node_t;
    tmp = alloc_node();
    node = tmp;
    (*node).type_0 = INT;
    (*node).value.int_literal = literal as uint64_t;
    return node;
}
pub unsafe extern "C" fn evaluate_expression(
    mut root: *mut node_t,
    mut fields___8: *mut fieldset_t,
) -> libc::c_int {
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
    if root.is_null() {
        return 1 as libc::c_int;
    }
    match (*root).type_0 as libc::c_uint {
        3 | 2 | 1 => return 1 as libc::c_int,
        0 | _ => {}
    }
    match (*root).value.op as libc::c_uint {
        0 => {
            tmp = eval_gt_node(root, fields___8);
            return tmp;
        }
        1 => {
            tmp___0 = eval_lt_node(root, fields___8);
            return tmp___0;
        }
        2 => {
            tmp___1 = eval_eq_node(root, fields___8);
            return tmp___1;
        }
        3 => {
            tmp___2 = eval_eq_node(root, fields___8);
            if tmp___2 != 0 {
                tmp___3 = 0 as libc::c_int;
            } else {
                tmp___3 = 1 as libc::c_int;
            }
            return tmp___3;
        }
        6 => {
            tmp___4 = eval_lt_eq_node(root, fields___8);
            return tmp___4;
        }
        7 => {
            tmp___5 = eval_gt_eq_node(root, fields___8);
            return tmp___5;
        }
        4 => {
            tmp___6 = evaluate_expression((*root).left_child, fields___8);
            if tmp___6 != 0 {
                tmp___7 = evaluate_expression((*root).right_child, fields___8);
                if tmp___7 != 0 {
                    tmp___8 = 1 as libc::c_int;
                } else {
                    tmp___8 = 0 as libc::c_int;
                }
            } else {
                tmp___8 = 0 as libc::c_int;
            }
            return tmp___8;
        }
        5 => {
            tmp___9 = evaluate_expression((*root).left_child, fields___8);
            if tmp___9 != 0 {
                tmp___11 = 1 as libc::c_int;
            } else {
                tmp___10 = evaluate_expression((*root).right_child, fields___8);
                if tmp___10 != 0 {
                    tmp___11 = 1 as libc::c_int;
                } else {
                    tmp___11 = 0 as libc::c_int;
                }
            }
            return tmp___11;
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn print_expression(mut root: *mut node_t) {
    if root.is_null() {
        return;
    }
    __printf_chk(
        1 as libc::c_int,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"( \0" as *const u8 as *const libc::c_char,
    );
    print_expression((*root).left_child);
    match (*root).type_0 as libc::c_uint {
        0 => {
            __printf_chk(
                1 as libc::c_int,
                b" %i \0" as *const u8 as *const libc::c_char,
                (*root).value.op as libc::c_uint,
            );
        }
        1 => {
            __printf_chk(
                1 as libc::c_int,
                b" (%s\0" as *const u8 as *const libc::c_char,
                (*root).value.field.fieldname,
            );
        }
        2 => {
            __printf_chk(
                1 as libc::c_int,
                b"%s) \0" as *const u8 as *const libc::c_char,
                (*root).value.string_literal,
            );
        }
        3 => {
            __printf_chk(
                1 as libc::c_int,
                b" %llu) \0" as *const u8 as *const libc::c_char,
                (*root).value.int_literal as libc::c_ulonglong,
            );
        }
        _ => {}
    }
    print_expression((*root).right_child);
    __printf_chk(
        1 as libc::c_int,
        b"%s\0" as *const u8 as *const libc::c_char,
        b" )\0" as *const u8 as *const libc::c_char,
    );
}
pub unsafe extern "C" fn gen_fielddef_set(
    mut fds: *mut fielddefset_t,
    mut fs: *mut fielddef_t,
    mut len: libc::c_int,
) {
    let mut open: *mut fielddef_t = 0 as *mut fielddef_t;
    if (*fds).len + len > 128 as libc::c_int {
        log_fatal(
            b"fieldset\0" as *const u8 as *const libc::c_char,
            b"out of room in field def set\0" as *const u8 as *const libc::c_char,
        );
    }
    open = &mut *((*fds).fielddefs).as_mut_ptr().offset((*fds).len as isize)
        as *mut fielddef_t;
    memcpy(
        open as *mut libc::c_void,
        fs as *const libc::c_void,
        (len as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<fielddef_t>() as libc::c_ulong) as _,
    );
    (*fds).len += len;
}
pub unsafe extern "C" fn fs_new_fieldset(
    mut fds: *mut fielddefset_t,
) -> *mut fieldset_t {
    let mut f: *mut fieldset_t = 0 as *mut fieldset_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<fieldset_t>() as libc::c_ulong,
    );
    f = tmp as *mut fieldset_t;
    (*f).len = 0 as libc::c_int;
    (*f).type_0 = 5 as libc::c_int;
    (*f).fds = fds;
    return f;
}
pub unsafe extern "C" fn fs_new_repeated_field(
    mut type_0: libc::c_int,
    mut free_: libc::c_int,
) -> *mut fieldset_t {
    let mut f: *mut fieldset_t = 0 as *mut fieldset_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<fieldset_t>() as libc::c_ulong,
    );
    f = tmp as *mut fieldset_t;
    (*f).len = 0 as libc::c_int;
    (*f).type_0 = 6 as libc::c_int;
    (*f).inner_type = type_0;
    (*f).free_ = free_;
    return f;
}
pub unsafe extern "C" fn fs_new_repeated_uint64() -> *mut fieldset_t {
    let mut tmp: *mut fieldset_t = 0 as *mut fieldset_t;
    tmp = fs_new_repeated_field(2 as libc::c_int, 0 as libc::c_int);
    return tmp;
}
pub unsafe extern "C" fn fs_new_repeated_bool() -> *mut fieldset_t {
    let mut tmp: *mut fieldset_t = 0 as *mut fieldset_t;
    tmp = fs_new_repeated_field(7 as libc::c_int, 0 as libc::c_int);
    return tmp;
}
pub unsafe extern "C" fn fs_new_repeated_string(
    mut free_: libc::c_int,
) -> *mut fieldset_t {
    let mut tmp: *mut fieldset_t = 0 as *mut fieldset_t;
    tmp = fs_new_repeated_field(1 as libc::c_int, free_);
    return tmp;
}
pub unsafe extern "C" fn fs_new_repeated_binary(
    mut free_: libc::c_int,
) -> *mut fieldset_t {
    let mut tmp: *mut fieldset_t = 0 as *mut fieldset_t;
    tmp = fs_new_repeated_field(3 as libc::c_int, free_);
    return tmp;
}
pub unsafe extern "C" fn fs_new_repeated_fieldset() -> *mut fieldset_t {
    let mut tmp: *mut fieldset_t = 0 as *mut fieldset_t;
    tmp = fs_new_repeated_field(5 as libc::c_int, 0 as libc::c_int);
    return tmp;
}
#[inline]
unsafe extern "C" fn fs_add_word(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut type_0: libc::c_int,
    mut free_: libc::c_int,
    mut len: size_t,
    mut value: field_val_t,
) {
    let mut f: *mut field_t = 0 as *mut field_t;
    let mut tmp: libc::c_int = 0;
    if (*fs).len + 1 as libc::c_int >= 128 as libc::c_int {
        log_fatal(
            b"fieldset\0" as *const u8 as *const libc::c_char,
            b"out of room in fieldset\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*fs).type_0 == 6 as libc::c_int {
        if (*fs).inner_type != type_0 {
            log_fatal(
                b"fieldset\0" as *const u8 as *const libc::c_char,
                b"object added to repeated field does not match type of repeated field.\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    }
    f = &mut *((*fs).fields).as_mut_ptr().offset((*fs).len as isize) as *mut field_t;
    if !((*fs).fds).is_null() {
        tmp = strcmp((*(*fs).fds).fielddefs[(*fs).len as usize].name, name);
        if tmp != 0 {
            log_fatal(
                b"fieldset\0" as *const u8 as *const libc::c_char,
                b"added field (%s) is not next expected field (%s).\0" as *const u8
                    as *const libc::c_char,
                name,
                (*(*fs).fds).fielddefs[(*fs).len as usize].name,
            );
        }
    }
    (*fs).len += 1;
    (*f).type_0 = type_0;
    (*f).name = name;
    (*f).len = len;
    (*f).value = value;
    (*f).free_ = free_;
}
unsafe extern "C" fn fs_modify_word(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut type_0: libc::c_int,
    mut free_: libc::c_int,
    mut len: size_t,
    mut value: field_val_t,
) {
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*fs).len {
        tmp = strcmp((*fs).fields[i as usize].name, name);
        if tmp == 0 {
            if (*fs).fields[i as usize].free_ != 0 {
                free((*fs).fields[i as usize].value.ptr);
                (*fs).fields[i as usize].value.ptr = 0 as *mut libc::c_void;
            }
            (*fs).fields[i as usize].type_0 = type_0;
            (*fs).fields[i as usize].free_ = free_;
            (*fs).fields[i as usize].len = len;
            (*fs).fields[i as usize].value = value;
            return;
        }
        i += 1;
    }
    log_fatal(
        b"fs\0" as *const u8 as *const libc::c_char,
        b"attempting to modify non-existent field\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn sanitize_utf8(mut buf: *const libc::c_char) -> *mut libc::c_char {
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: uint32_t = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: *const uint8_t = 0 as *const uint8_t;
    let mut tmp___5: size_t = 0;
    let mut tmp___6: size_t = 0;
    let mut tmp___7: size_t = 0;
    let mut safe_buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___8: size_t = 0;
    let mut tmp___9: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut safe_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___10: size_t = 0;
    let mut j: uint32_t = 0;
    let mut tmp___11: size_t = 0;
    let mut tmp___12: *const uint8_t = 0 as *const uint8_t;
    let mut tmp___17: size_t = 0;
    let mut tmp___18: size_t = 0;
    let mut tmp___19: size_t = 0;
    let mut tmp___25: size_t = 0;
    let mut tmp___26: *const uint8_t = 0 as *const uint8_t;
    let mut tmp___30: size_t = 0;
    let mut tmp___36: size_t = 0;
    let mut tmp___37: size_t = 0;
    ptr = buf;
    i = 0 as libc::c_int as uint32_t;
    loop {
        tmp___6 = strlen(buf);
        if !((i as size_t) < tmp___6) {
            break;
        }
        tmp___7 = strlen(buf);
        if !((ptr as libc::c_ulong) < buf.offset(tmp___7 as isize) as libc::c_ulong) {
            break;
        }
        tmp = strlen(ptr);
        tmp___0 = u8_check(ptr as *mut uint8_t as *const uint8_t, tmp);
        ptr = tmp___0 as *mut libc::c_char as *const libc::c_char;
        if ptr as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            break;
        }
        if !(ptr as libc::c_ulong >= buf as libc::c_ulong) {
            __assert_fail(
                b"ptr >= buf\0" as *const u8 as *const libc::c_char,
                b"src/fieldset.c\0" as *const u8 as *const libc::c_char,
                140 as libc::c_uint,
                b"sanitize_utf8\0" as *const u8 as *const libc::c_char,
            );
        }
        tmp___5 = strlen(buf);
        if !((ptr as libc::c_ulong) < buf.offset(tmp___5 as isize) as libc::c_ulong) {
            __assert_fail(
                b"ptr < buf + strlen(buf)\0" as *const u8 as *const libc::c_char,
                b"src/fieldset.c\0" as *const u8 as *const libc::c_char,
                141 as libc::c_uint,
                b"sanitize_utf8\0" as *const u8 as *const libc::c_char,
            );
        }
        ptr = ptr.offset(1);
        i = i.wrapping_add(1);
    }
    tmp___8 = strlen(buf);
    tmp___9 = xmalloc(
        tmp___8
            .wrapping_add(i.wrapping_mul(2 as libc::c_uint) as size_t)
            .wrapping_add(1 as libc::c_ulong),
    );
    safe_buf = tmp___9 as *mut libc::c_char;
    safe_ptr = 0 as *mut libc::c_void as *mut libc::c_char;
    tmp___10 = strlen(buf);
    memcpy(safe_buf as *mut libc::c_void, buf as *const libc::c_void, tmp___10 as _);
    j = 0 as libc::c_int as uint32_t;
    while j < i {
        tmp___11 = strlen(safe_buf as *const libc::c_char);
        tmp___12 = u8_check(safe_buf as *mut uint8_t as *const uint8_t, tmp___11);
        safe_ptr = tmp___12 as *mut libc::c_char;
        if safe_ptr as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            log_warn(
                b"fieldset\0" as *const u8 as *const libc::c_char,
                b"UTF8 Sanitization issue. %u errors, fell through iter %u. Orig: %s new: %s\0"
                    as *const u8 as *const libc::c_char,
                i,
                j,
                buf,
                safe_buf,
            );
            i = j;
            break;
        } else {
            if !(safe_ptr as libc::c_ulong >= safe_buf as libc::c_ulong) {
                __assert_fail(
                    b"safe_ptr >= safe_buf\0" as *const u8 as *const libc::c_char,
                    b"src/fieldset.c\0" as *const u8 as *const libc::c_char,
                    171 as libc::c_uint,
                    b"sanitize_utf8\0" as *const u8 as *const libc::c_char,
                );
            }
            tmp___17 = strlen(safe_buf as *const libc::c_char);
            if !((safe_ptr as libc::c_ulong)
                < safe_buf.offset(tmp___17 as isize) as libc::c_ulong)
            {
                __assert_fail(
                    b"safe_ptr < safe_buf + strlen(safe_buf)\0" as *const u8
                        as *const libc::c_char,
                    b"src/fieldset.c\0" as *const u8 as *const libc::c_char,
                    172 as libc::c_uint,
                    b"sanitize_utf8\0" as *const u8 as *const libc::c_char,
                );
            }
            tmp___19 = strlen(safe_ptr as *const libc::c_char);
            if tmp___19 > 1 as libc::c_ulong {
                tmp___18 = strlen(
                    safe_ptr.offset(1 as libc::c_int as isize) as *const libc::c_char,
                );
                memcpy(
                    safe_ptr.offset(3 as libc::c_int as isize) as *mut libc::c_void,
                    safe_ptr.offset(1 as libc::c_int as isize) as *const libc::c_void,
                    tmp___18 as _,
                );
            }
            *safe_ptr
                .offset(
                    0 as libc::c_int as isize,
                ) = -(17 as libc::c_int) as libc::c_char;
            *safe_ptr
                .offset(
                    1 as libc::c_int as isize,
                ) = -(65 as libc::c_int) as libc::c_char;
            *safe_ptr
                .offset(
                    2 as libc::c_int as isize,
                ) = -(67 as libc::c_int) as libc::c_char;
            j = j.wrapping_add(1);
        }
    }
    tmp___25 = strlen(safe_buf as *const libc::c_char);
    tmp___26 = u8_check(safe_buf as *mut uint8_t as *const uint8_t, tmp___25);
    if !(tmp___26 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"u8_check((uint8_t *)safe_buf, strlen(safe_buf)) == NULL\0" as *const u8
                as *const libc::c_char,
            b"src/fieldset.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_uint,
            b"sanitize_utf8\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___30 = strlen(buf);
    if !(*safe_buf
        .offset(
            tmp___30.wrapping_add(i.wrapping_mul(2 as libc::c_uint) as size_t) as isize,
        ) as libc::c_int == 0 as libc::c_int)
    {
        __assert_fail(
            b"safe_buf[strlen(buf) + i * 2] == '\\0'\0" as *const u8
                as *const libc::c_char,
            b"src/fieldset.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_uint,
            b"sanitize_utf8\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___36 = strlen(safe_buf as *const libc::c_char);
    tmp___37 = strlen(buf);
    if !(tmp___36 == tmp___37.wrapping_add(i.wrapping_mul(2 as libc::c_uint) as size_t))
    {
        __assert_fail(
            b"strlen(safe_buf) == (strlen(buf) + i * 2)\0" as *const u8
                as *const libc::c_char,
            b"src/fieldset.c\0" as *const u8 as *const libc::c_char,
            191 as libc::c_uint,
            b"sanitize_utf8\0" as *const u8 as *const libc::c_char,
        );
    }
    return safe_buf;
}
pub unsafe extern "C" fn fs_add_null(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
) {
    let mut val: field_val_t = field_val {
        ptr: 0 as *mut libc::c_void,
    };
    val.ptr = 0 as *mut libc::c_void;
    fs_add_word(
        fs,
        name,
        4 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int as size_t,
        val,
    );
}
pub unsafe extern "C" fn fs_add_string(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut value: *mut libc::c_char,
    mut free_: libc::c_int,
) {
    let mut val: field_val_t = field_val {
        ptr: 0 as *mut libc::c_void,
    };
    let mut tmp: size_t = 0;
    val.ptr = value as *mut libc::c_void;
    tmp = strlen(value as *const libc::c_char);
    fs_add_word(fs, name, 1 as libc::c_int, free_, tmp, val);
}
pub unsafe extern "C" fn fs_add_unsafe_string(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut value: *mut libc::c_char,
    mut free_: libc::c_int,
) {
    let mut val: field_val_t = field_val {
        ptr: 0 as *mut libc::c_void,
    };
    let mut tmp: size_t = 0;
    let mut safe_value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val___0: field_val_t = field_val {
        ptr: 0 as *mut libc::c_void,
    };
    let mut tmp___1: size_t = 0;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: *const uint8_t = 0 as *const uint8_t;
    tmp___2 = strlen(value as *const libc::c_char);
    tmp___3 = u8_check(value as *mut uint8_t as *const uint8_t, tmp___2);
    if tmp___3 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        val.ptr = value as *mut libc::c_void;
        tmp = strlen(value as *const libc::c_char);
        fs_add_word(fs, name, 1 as libc::c_int, free_, tmp, val);
    } else {
        tmp___0 = sanitize_utf8(value as *const libc::c_char);
        safe_value = tmp___0;
        if free_ != 0 {
            free(value as *mut libc::c_void);
        }
        val___0.ptr = safe_value as *mut libc::c_void;
        tmp___1 = strlen(safe_value as *const libc::c_char);
        fs_add_word(fs, name, 1 as libc::c_int, 1 as libc::c_int, tmp___1, val___0);
    };
}
pub unsafe extern "C" fn fs_chkadd_string(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut value: *mut libc::c_char,
    mut free_: libc::c_int,
) {
    if !value.is_null() {
        fs_add_string(fs, name, value, free_);
    } else {
        fs_add_null(fs, name);
    };
}
pub unsafe extern "C" fn fs_chkadd_unsafe_string(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut value: *mut libc::c_char,
    mut free_: libc::c_int,
) {
    if !value.is_null() {
        fs_add_unsafe_string(fs, name, value, free_);
    } else {
        fs_add_null(fs, name);
    };
}
pub unsafe extern "C" fn fs_add_constchar(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut value: *const libc::c_char,
) {
    let mut val: field_val_t = field_val {
        ptr: 0 as *mut libc::c_void,
    };
    let mut tmp: size_t = 0;
    val.ptr = value as *mut libc::c_char as *mut libc::c_void;
    tmp = strlen(value);
    fs_add_word(fs, name, 1 as libc::c_int, 0 as libc::c_int, tmp, val);
}
pub unsafe extern "C" fn fs_add_uint64(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut value: uint64_t,
) {
    let mut val: field_val_t = field_val {
        ptr: 0 as *mut libc::c_void,
    };
    val.num = value;
    fs_add_word(
        fs,
        name,
        2 as libc::c_int,
        0 as libc::c_int,
        ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
        val,
    );
}
pub unsafe extern "C" fn fs_add_bool(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut value: libc::c_int,
) {
    let mut val: field_val_t = field_val {
        ptr: 0 as *mut libc::c_void,
    };
    val.num = value as uint64_t;
    fs_add_word(
        fs,
        name,
        7 as libc::c_int,
        0 as libc::c_int,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        val,
    );
}
pub unsafe extern "C" fn fs_add_binary(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut len: size_t,
    mut value: *mut libc::c_void,
    mut free_: libc::c_int,
) {
    let mut val: field_val_t = field_val {
        ptr: 0 as *mut libc::c_void,
    };
    val.ptr = value;
    fs_add_word(fs, name, 3 as libc::c_int, free_, len, val);
}
pub unsafe extern "C" fn fs_add_fieldset(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut child: *mut fieldset_t,
) {
    let mut val: field_val_t = field_val {
        ptr: 0 as *mut libc::c_void,
    };
    val.ptr = child as *mut libc::c_void;
    fs_add_word(
        fs,
        name,
        5 as libc::c_int,
        1 as libc::c_int,
        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
        val,
    );
}
pub unsafe extern "C" fn fs_add_repeated(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut child: *mut fieldset_t,
) {
    let mut val: field_val_t = field_val {
        ptr: 0 as *mut libc::c_void,
    };
    val.ptr = child as *mut libc::c_void;
    fs_add_word(
        fs,
        name,
        6 as libc::c_int,
        1 as libc::c_int,
        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
        val,
    );
}
pub unsafe extern "C" fn fs_modify_null(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
) {
    let mut val: field_val_t = field_val {
        ptr: 0 as *mut libc::c_void,
    };
    val.ptr = 0 as *mut libc::c_void;
    fs_modify_word(
        fs,
        name,
        4 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int as size_t,
        val,
    );
}
pub unsafe extern "C" fn fs_modify_string(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut value: *mut libc::c_char,
    mut free_: libc::c_int,
) {
    let mut val: field_val_t = field_val {
        ptr: 0 as *mut libc::c_void,
    };
    let mut tmp: size_t = 0;
    val.ptr = value as *mut libc::c_void;
    tmp = strlen(value as *const libc::c_char);
    fs_modify_word(fs, name, 1 as libc::c_int, free_, tmp, val);
}
pub unsafe extern "C" fn fs_modify_constchar(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut value: *const libc::c_char,
) {
    let mut val: field_val_t = field_val {
        ptr: 0 as *mut libc::c_void,
    };
    let mut tmp: size_t = 0;
    val.ptr = value as *mut libc::c_char as *mut libc::c_void;
    tmp = strlen(value);
    fs_modify_word(fs, name, 1 as libc::c_int, 0 as libc::c_int, tmp, val);
}
pub unsafe extern "C" fn fs_modify_uint64(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut value: uint64_t,
) {
    let mut val: field_val_t = field_val {
        ptr: 0 as *mut libc::c_void,
    };
    val.num = value;
    fs_modify_word(
        fs,
        name,
        2 as libc::c_int,
        0 as libc::c_int,
        ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
        val,
    );
}
pub unsafe extern "C" fn fs_modify_bool(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut value: libc::c_int,
) {
    let mut val: field_val_t = field_val {
        ptr: 0 as *mut libc::c_void,
    };
    val.num = value as uint64_t;
    fs_modify_word(
        fs,
        name,
        7 as libc::c_int,
        0 as libc::c_int,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        val,
    );
}
pub unsafe extern "C" fn fs_modify_binary(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut len: size_t,
    mut value: *mut libc::c_void,
    mut free_: libc::c_int,
) {
    let mut val: field_val_t = field_val {
        ptr: 0 as *mut libc::c_void,
    };
    val.ptr = value;
    fs_modify_word(fs, name, 3 as libc::c_int, free_, len, val);
}
pub unsafe extern "C" fn fs_get_uint64_by_index(
    mut fs: *mut fieldset_t,
    mut index___0: libc::c_int,
) -> uint64_t {
    return (*fs).fields[index___0 as usize].value.num;
}
pub unsafe extern "C" fn fs_get_string_by_index(
    mut fs: *mut fieldset_t,
    mut index___0: libc::c_int,
) -> *mut libc::c_char {
    return (*fs).fields[index___0 as usize].value.ptr as *mut libc::c_char;
}
pub unsafe extern "C" fn fds_get_index_by_name(
    mut fds: *mut fielddefset_t,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*fds).len {
        tmp = strcmp((*fds).fielddefs[i as usize].name, name);
        if tmp == 0 {
            return i;
        }
        i += 1;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn field_free(mut f: *mut field_t) {
    if (*f).type_0 == 5 as libc::c_int {
        fs_free((*f).value.ptr as *mut fieldset_t);
    } else if (*f).type_0 == 6 as libc::c_int {
        fs_free((*f).value.ptr as *mut fieldset_t);
    } else if (*f).free_ != 0 {
        free((*f).value.ptr);
    }
}
pub unsafe extern "C" fn fs_free(mut fs: *mut fieldset_t) {
    let mut i: libc::c_int = 0;
    let mut f: *mut field_t = 0 as *mut field_t;
    if fs.is_null() {
        return;
    }
    i = 0 as libc::c_int;
    while i < (*fs).len {
        f = &mut *((*fs).fields).as_mut_ptr().offset(i as isize) as *mut field_t;
        field_free(f);
        i += 1;
    }
    free(fs as *mut libc::c_void);
}
pub unsafe extern "C" fn fs_generate_fieldset_translation(
    mut t: *mut translation_t,
    mut avail: *mut fielddefset_t,
    mut req: *mut *const libc::c_char,
    mut reqlen: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    memset(
        t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<translation_t>() as libc::c_ulong as _,
    );
    if t.is_null() {
        log_fatal(
            b"fieldset\0" as *const u8 as *const libc::c_char,
            b"unable to allocate memory for translation\0" as *const u8
                as *const libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    while i < reqlen {
        tmp = fds_get_index_by_name(avail, *req.offset(i as isize));
        l = tmp;
        if l < 0 as libc::c_int {
            log_fatal(
                b"fieldset\0" as *const u8 as *const libc::c_char,
                b"specified field (%s) not available in selected probe module.\0"
                    as *const u8 as *const libc::c_char,
                *req.offset(i as isize),
            );
        }
        tmp___0 = (*t).len;
        (*t).len += 1;
        (*t).translation[tmp___0 as usize] = l;
        i += 1;
    }
}
pub unsafe extern "C" fn fs_generate_full_fieldset_translation(
    mut t: *mut translation_t,
    mut avail: *mut fielddefset_t,
) {
    let mut i: libc::c_int = 0;
    memset(
        t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<translation_t>() as libc::c_ulong as _,
    );
    if t.is_null() {
        log_fatal(
            b"fieldset\0" as *const u8 as *const libc::c_char,
            b"unable to allocate memory for translation\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*t).len = (*avail).len;
    i = 0 as libc::c_int;
    while i < (*avail).len {
        (*t).translation[i as usize] = i;
        i += 1;
    }
}
pub unsafe extern "C" fn translate_fieldset(
    mut fs: *mut fieldset_t,
    mut t: *mut translation_t,
) -> *mut fieldset_t {
    let mut retv: *mut fieldset_t = 0 as *mut fieldset_t;
    let mut tmp: *mut fieldset_t = 0 as *mut fieldset_t;
    let mut i: libc::c_int = 0;
    let mut o: libc::c_int = 0;
    tmp = fs_new_fieldset(0 as *mut libc::c_void as *mut fielddefset_t);
    retv = tmp;
    if retv.is_null() {
        log_fatal(
            b"fieldset\0" as *const u8 as *const libc::c_char,
            b"unable to allocate space for translated field set\0" as *const u8
                as *const libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    while i < (*t).len {
        o = (*t).translation[i as usize];
        memcpy(
            &mut *((*retv).fields).as_mut_ptr().offset(i as isize) as *mut field_t
                as *mut libc::c_void,
            &mut *((*fs).fields).as_mut_ptr().offset(o as isize) as *mut field_t
                as *const libc::c_void,
            ::std::mem::size_of::<field_t>() as libc::c_ulong as _,
        );
        i += 1;
    }
    (*retv).len = (*t).len;
    return retv;
}
#[inline(always)]
unsafe extern "C" fn gethostname(
    mut __buf: *mut libc::c_char,
    mut __buflen: size_t,
) -> libc::c_int {
    let mut tmp: libc::c_ulong = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_ulong = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_ulong = 0;
    let mut tmp___4: libc::c_ulong = 0;
    let mut tmp___5: libc::c_int = 0;
    tmp___4 = (if 1 as libc::c_int & 2 == 0 { -1isize } else { 0isize }) as libc::size_t as _;
    if tmp___4 != 0xffffffffffffffff as libc::c_ulong {
        tmp = (if 1 as libc::c_int & 2 == 0 { -1isize } else { 0isize }) as libc::size_t as _;
        tmp___0 = __gethostname_chk(__buf, __buflen, tmp);
        return tmp___0;
    }
    tmp___5 = __gethostname_alias(__buf, __buflen);
    return tmp___5;
}
pub static mut zfilter: *mut node_t = 0 as *const node_t as *mut node_t;
unsafe extern "C" fn validate_node(
    mut node: *mut node_t,
    mut fields___8: *mut fielddefset_t,
) -> libc::c_int {
    let mut index___0: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    found = 0 as libc::c_int;
    if (*node).type_0 as libc::c_uint == 0 as libc::c_uint {
        if (*node).value.op as libc::c_uint == 4 as libc::c_uint {
            return 1 as libc::c_int
        } else {
            if (*node).value.op as libc::c_uint == 5 as libc::c_uint {
                return 1 as libc::c_int;
            }
        }
        index___0 = 0 as libc::c_int;
        while index___0 < (*fields___8).len {
            if !((*fields___8).fielddefs[index___0 as usize].name).is_null() {
                tmp = strcmp(
                    (*fields___8).fielddefs[index___0 as usize].name,
                    (*(*node).left_child).value.field.fieldname as *const libc::c_char,
                );
                if tmp == 0 as libc::c_int {
                    (*(*node).left_child).value.field.index = index___0;
                    found = 1 as libc::c_int;
                    break;
                }
            }
            index___0 += 1;
        }
        if found == 0 {
            __fprintf_chk(
                stderr,
                1 as libc::c_int,
                b"Field '%s' does not exist\n\0" as *const u8 as *const libc::c_char,
                (*(*node).left_child).value.field.fieldname,
            );
            return 0 as libc::c_int;
        }
        match (*(*node).right_child).type_0 as libc::c_uint {
            2 => {
                tmp___0 = strcmp(
                    (*fields___8).fielddefs[index___0 as usize].type_0,
                    b"string\0" as *const u8 as *const libc::c_char,
                );
                if tmp___0 == 0 as libc::c_int {
                    return 1 as libc::c_int
                } else {
                    __fprintf_chk(
                        stderr,
                        1 as libc::c_int,
                        b"Field '%s' is not of type 'string'\n\0" as *const u8
                            as *const libc::c_char,
                        (*fields___8).fielddefs[index___0 as usize].name,
                    );
                    return 0 as libc::c_int;
                }
            }
            3 => {
                tmp___1 = strcmp(
                    (*fields___8).fielddefs[index___0 as usize].type_0,
                    b"int\0" as *const u8 as *const libc::c_char,
                );
                if tmp___1 == 0 as libc::c_int {
                    return 1 as libc::c_int
                } else {
                    tmp___2 = strcmp(
                        (*fields___8).fielddefs[index___0 as usize].type_0,
                        b"bool\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___2 == 0 as libc::c_int {
                        return 1 as libc::c_int
                    } else {
                        __fprintf_chk(
                            stderr,
                            1 as libc::c_int,
                            b"Field '%s' is not of type 'int'\n\0" as *const u8
                                as *const libc::c_char,
                            (*fields___8).fielddefs[index___0 as usize].name,
                        );
                        return 0 as libc::c_int;
                    }
                }
            }
            _ => return 0 as libc::c_int,
        }
    } else {
        return 1 as libc::c_int
    };
}
pub unsafe extern "C" fn parse_filter_string(
    mut filter: *mut libc::c_char,
) -> libc::c_int {
    let mut buffer_state: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    let mut tmp: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    let mut status: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = yy_scan_string(filter as *const libc::c_char);
    buffer_state = tmp;
    tmp___0 = yyparse();
    status = tmp___0;
    yy_delete_buffer(buffer_state);
    if status != 0 {
        log_error(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"Unable to parse filter string: '%s'\0" as *const u8 as *const libc::c_char,
            filter,
        );
        return 0 as libc::c_int;
    }
    zconf.filter.expression = zfilter;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn validate_filter(
    mut root: *mut node_t,
    mut fields___8: *mut fielddefset_t,
) -> libc::c_int {
    let mut valid: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    if root.is_null() {
        return 1 as libc::c_int;
    }
    valid = validate_node(root, fields___8);
    if valid == 0 {
        return 0 as libc::c_int;
    }
    tmp = validate_filter((*root).left_child, fields___8);
    if tmp != 0 {
        tmp___0 = validate_filter((*root).right_child, fields___8);
        if tmp___0 != 0 {
            tmp___1 = 1 as libc::c_int;
        } else {
            tmp___1 = 0 as libc::c_int;
        }
    } else {
        tmp___1 = 0 as libc::c_int;
    }
    return tmp___1;
}
pub unsafe extern "C" fn get_default_iface() -> *mut libc::c_char {
    let mut errbuf: [libc::c_char; 256] = [0; 256];
    let mut iface: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = pcap_lookupdev(errbuf.as_mut_ptr());
    iface = tmp;
    if iface as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        log_fatal(
            b"send\0" as *const u8 as *const libc::c_char,
            b"ZMap could not detect your default network interface. You likely do not privileges to open a raw packet socket. Are you running as root or with the CAP_NET_RAW capability? If you are, you may need to manually set interface using the \"-i\" flag.\0"
                as *const u8 as *const libc::c_char,
        );
    }
    return iface;
}
pub unsafe extern "C" fn read_nl_sock(
    mut sock: libc::c_int,
    mut buf: *mut libc::c_char,
    mut buf_len: libc::c_int,
) -> libc::c_int {
    let mut msg_len: libc::c_int = 0;
    let mut pbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut tmp: ssize_t = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nlhdr: *mut nlmsghdr = 0 as *mut nlmsghdr;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: libc::c_int = 0;
    msg_len = 0 as libc::c_int;
    pbuf = buf;
    loop {
        tmp = recv(
            sock,
            pbuf as *mut libc::c_void,
            (buf_len - msg_len) as size_t,
            0 as libc::c_int,
        );
        len = tmp as libc::c_int;
        if len <= 0 as libc::c_int {
            tmp___0 = __errno_location();
            tmp___1 = strerror(*tmp___0);
            log_debug(
                b"get-gw\0" as *const u8 as *const libc::c_char,
                b"recv failed: %s\0" as *const u8 as *const libc::c_char,
                tmp___1,
            );
            return -(1 as libc::c_int);
        }
        nlhdr = pbuf as *mut nlmsghdr;
        if len as libc::c_uint
            >= ::std::mem::size_of::<nlmsghdr>() as libc::c_ulong as libc::c_int
                as libc::c_uint
        {
            if (*nlhdr).nlmsg_len as libc::c_ulong
                >= ::std::mem::size_of::<nlmsghdr>() as libc::c_ulong
            {
                if (*nlhdr).nlmsg_len <= len as libc::c_uint {
                    tmp___4 = 1 as libc::c_int;
                } else {
                    tmp___4 = 0 as libc::c_int;
                }
            } else {
                tmp___4 = 0 as libc::c_int;
            }
        } else {
            tmp___4 = 0 as libc::c_int;
        }
        if tmp___4 == 0 as libc::c_int {
            tmp___2 = __errno_location();
            tmp___3 = strerror(*tmp___2);
            log_debug(
                b"get-gw\0" as *const u8 as *const libc::c_char,
                b"recv failed: %s\0" as *const u8 as *const libc::c_char,
                tmp___3,
            );
            return -(1 as libc::c_int);
        } else {
            if (*nlhdr).nlmsg_type as libc::c_int == 2 as libc::c_int {
                tmp___2 = __errno_location();
                tmp___3 = strerror(*tmp___2);
                log_debug(
                    b"get-gw\0" as *const u8 as *const libc::c_char,
                    b"recv failed: %s\0" as *const u8 as *const libc::c_char,
                    tmp___3,
                );
                return -(1 as libc::c_int);
            }
        }
        if (*nlhdr).nlmsg_type as libc::c_int == 3 as libc::c_int {
            break;
        }
        msg_len += len;
        pbuf = pbuf.offset(len as isize);
        if (*nlhdr).nlmsg_flags as libc::c_int & 2 as libc::c_int == 0 as libc::c_int {
            break;
        }
    }
    return msg_len;
}
pub unsafe extern "C" fn send_nl_req(
    mut msg_type: uint16_t,
    mut seq: uint32_t,
    mut payload: *mut libc::c_void,
    mut payload_len: uint32_t,
) -> libc::c_int {
    let mut sock: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nlmsg: *mut nlmsghdr = 0 as *mut nlmsghdr;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: __pid_t = 0;
    let mut tmp___4: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: ssize_t = 0;
    tmp = socket(16 as libc::c_int, 2 as libc::c_int, 0 as libc::c_int);
    sock = tmp;
    if sock < 0 as libc::c_int {
        tmp___0 = __errno_location();
        tmp___1 = strerror(*tmp___0);
        log_error(
            b"get-gw\0" as *const u8 as *const libc::c_char,
            b"unable to get socket: %s\0" as *const u8 as *const libc::c_char,
            tmp___1,
        );
        return -(1 as libc::c_int);
    }
    if (payload_len
        .wrapping_add(
            ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
                .wrapping_add(4 as libc::c_ulong)
                .wrapping_sub(1 as libc::c_ulong) & 4294967292 as libc::c_ulong)
                as libc::c_int as uint32_t,
        )
        .wrapping_add(4 as libc::c_uint)
        .wrapping_sub(1 as libc::c_uint) & 4294967292 as libc::c_uint) < payload_len
    {
        close(sock);
        return -(1 as libc::c_int);
    }
    tmp___2 = xmalloc(
        (payload_len
            .wrapping_add(
                ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
                    .wrapping_add(4 as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_ulong) & 4294967292 as libc::c_ulong)
                    as libc::c_int as uint32_t,
            )
            .wrapping_add(4 as libc::c_uint)
            .wrapping_sub(1 as libc::c_uint) & 4294967292 as libc::c_uint) as size_t,
    );
    nlmsg = tmp___2 as *mut nlmsghdr;
    memset(
        nlmsg as *mut libc::c_void,
        0 as libc::c_int,
        (payload_len
            .wrapping_add(
                ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
                    .wrapping_add(4 as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_ulong) & 4294967292 as libc::c_ulong)
                    as libc::c_int as uint32_t,
            )
            .wrapping_add(4 as libc::c_uint)
            .wrapping_sub(1 as libc::c_uint) & 4294967292 as libc::c_uint) as size_t as _,
    );
    memcpy(
        (nlmsg as *mut libc::c_char)
            .offset(
                ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
                    .wrapping_add(4 as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_ulong) & 4294967292 as libc::c_ulong)
                    as libc::c_int as isize,
            ) as *mut libc::c_void,
        payload as *const libc::c_void,
        payload_len as size_t as _,
    );
    (*nlmsg).nlmsg_type = msg_type;
    (*nlmsg)
        .nlmsg_len = payload_len
        .wrapping_add(
            ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
                .wrapping_add(4 as libc::c_ulong)
                .wrapping_sub(1 as libc::c_ulong) & 4294967292 as libc::c_ulong)
                as libc::c_int as uint32_t,
        );
    (*nlmsg).nlmsg_flags = 769 as libc::c_int as __u16;
    (*nlmsg).nlmsg_seq = seq;
    tmp___3 = getpid();
    (*nlmsg).nlmsg_pid = tmp___3 as __u32;
    tmp___6 = send(
        sock,
        nlmsg as *const libc::c_void,
        (*nlmsg).nlmsg_len as size_t,
        0 as libc::c_int,
    );
    if tmp___6 < 0 as libc::c_long {
        tmp___4 = __errno_location();
        tmp___5 = strerror(*tmp___4);
        log_error(
            b"get-gw\0" as *const u8 as *const libc::c_char,
            b"failure sending: %s\0" as *const u8 as *const libc::c_char,
            tmp___5,
        );
        return -(1 as libc::c_int);
    }
    free(nlmsg as *mut libc::c_void);
    return sock;
}
pub unsafe extern "C" fn get_hw_addr(
    mut gw_ip: *mut in_addr,
    mut iface: *mut libc::c_char,
    mut hw_mac: *mut libc::c_uchar,
) -> libc::c_int {
    let mut req: ndmsg = ndmsg {
        ndm_family: 0,
        ndm_pad1: 0,
        ndm_pad2: 0,
        ndm_ifindex: 0,
        ndm_state: 0,
        ndm_flags: 0,
        ndm_type: 0,
    };
    let mut tmp: libc::c_uint = 0;
    let mut sock: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut nl_len: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut nlhdr: *mut nlmsghdr = 0 as *mut nlmsghdr;
    let mut rt_attr: *mut rtattr = 0 as *mut rtattr;
    let mut rt_msg: *mut rtmsg = 0 as *mut rtmsg;
    let mut rt_len: libc::c_int = 0;
    let mut mac: [libc::c_uchar; 6] = [0; 6];
    let mut dst_ip: in_addr = in_addr { s_addr: 0 };
    let mut correct_ip: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    memset(
        &mut req as *mut ndmsg as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<ndmsg>() as libc::c_ulong as _,
    );
    if gw_ip.is_null() {
        return -(1 as libc::c_int)
    } else {
        if hw_mac.is_null() {
            return -(1 as libc::c_int);
        }
    }
    req.ndm_family = 2 as libc::c_int as __u8;
    tmp = if_nametoindex(iface as *const libc::c_char);
    req.ndm_ifindex = tmp as __s32;
    req.ndm_state = 2 as libc::c_int as __u16;
    req.ndm_type = 2 as libc::c_int as __u8;
    tmp___0 = send_nl_req(
        30 as libc::c_int as uint16_t,
        1 as libc::c_int as uint32_t,
        &mut req as *mut ndmsg as *mut libc::c_void,
        ::std::mem::size_of::<ndmsg>() as libc::c_ulong as uint32_t,
    );
    sock = tmp___0;
    tmp___1 = xmalloc(64000 as libc::c_int as size_t);
    buf = tmp___1 as *mut libc::c_char;
    tmp___2 = read_nl_sock(sock, buf, 64000 as libc::c_int);
    nl_len = tmp___2;
    if nl_len <= 0 as libc::c_int {
        free(buf as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    nlhdr = buf as *mut nlmsghdr;
    while nl_len >= ::std::mem::size_of::<nlmsghdr>() as libc::c_ulong as libc::c_int {
        if !((*nlhdr).nlmsg_len as libc::c_ulong
            >= ::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
        {
            break;
        }
        if !((*nlhdr).nlmsg_len <= nl_len as __u32) {
            break;
        }
        correct_ip = 0 as libc::c_int;
        rt_msg = (nlhdr as *mut libc::c_char)
            .offset(
                ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
                    .wrapping_add(4 as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_ulong) & 4294967292 as libc::c_ulong)
                    as libc::c_int as isize,
            ) as *mut libc::c_void as *mut rtmsg;
        if (*rt_msg).rtm_family as libc::c_int != 2 as libc::c_int {
            free(buf as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
        rt_attr = (rt_msg as *mut libc::c_char)
            .offset(
                ((::std::mem::size_of::<rtmsg>() as libc::c_ulong)
                    .wrapping_add(4 as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_ulong) & 4294967292 as libc::c_ulong)
                    as isize,
            ) as *mut rtattr;
        rt_len = ((*nlhdr).nlmsg_len as libc::c_ulong)
            .wrapping_sub(
                (::std::mem::size_of::<rtmsg>() as libc::c_ulong)
                    .wrapping_add(
                        ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
                            .wrapping_add(4 as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_ulong)
                            & 4294967292 as libc::c_ulong) as libc::c_int
                            as libc::c_ulong,
                    )
                    .wrapping_add(4 as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_ulong) & 4294967292 as libc::c_ulong,
            ) as libc::c_int;
        while rt_len >= ::std::mem::size_of::<rtattr>() as libc::c_ulong as libc::c_int {
            if !((*rt_attr).rta_len as libc::c_ulong
                >= ::std::mem::size_of::<rtattr>() as libc::c_ulong)
            {
                break;
            }
            if !((*rt_attr).rta_len as libc::c_int <= rt_len) {
                break;
            }
            match (*rt_attr).rta_type as libc::c_int {
                2 => {
                    if ((*rt_attr).rta_len as libc::c_int as libc::c_ulong)
                        .wrapping_sub(
                            (::std::mem::size_of::<rtattr>() as libc::c_ulong)
                                .wrapping_add(4 as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_ulong)
                                & 4294967292 as libc::c_ulong,
                        ) != 6 as libc::c_ulong
                    {
                        log_fatal(
                            b"get_gateway\0" as *const u8 as *const libc::c_char,
                            b"Unexpected hardware address length (%d). If you are using a VPN, supply the --iplayer flag (and provide an interface via -i)\0"
                                as *const u8 as *const libc::c_char,
                            ((*rt_attr).rta_len as libc::c_int as libc::c_ulong)
                                .wrapping_sub(
                                    (::std::mem::size_of::<rtattr>() as libc::c_ulong)
                                        .wrapping_add(4 as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_ulong)
                                        & 4294967292 as libc::c_ulong,
                                ),
                        );
                    }
                    memcpy(
                        mac.as_mut_ptr() as *mut libc::c_void,
                        (rt_attr as *mut libc::c_char)
                            .offset(
                                ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
                                    .wrapping_add(4 as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_ulong)
                                    & 4294967292 as libc::c_ulong) as isize,
                            ) as *mut libc::c_void as *const libc::c_void,
                        6 as libc::c_int as size_t as _,
                    );
                }
                1 => {
                    if ((*rt_attr).rta_len as libc::c_int as libc::c_ulong)
                        .wrapping_sub(
                            (::std::mem::size_of::<rtattr>() as libc::c_ulong)
                                .wrapping_add(4 as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_ulong)
                                & 4294967292 as libc::c_ulong,
                        ) != ::std::mem::size_of::<in_addr>() as libc::c_ulong
                    {
                        log_fatal(
                            b"get_gateway\0" as *const u8 as *const libc::c_char,
                            b"Unexpected IP address length (%d). If you are using a VPN, supply the --iplayer flag (and provide an interface via -i)\0"
                                as *const u8 as *const libc::c_char,
                            ((*rt_attr).rta_len as libc::c_int as libc::c_ulong)
                                .wrapping_sub(
                                    (::std::mem::size_of::<rtattr>() as libc::c_ulong)
                                        .wrapping_add(4 as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_ulong)
                                        & 4294967292 as libc::c_ulong,
                                ),
                        );
                    }
                    memcpy(
                        &mut dst_ip as *mut in_addr as *mut libc::c_void,
                        (rt_attr as *mut libc::c_char)
                            .offset(
                                ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
                                    .wrapping_add(4 as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_ulong)
                                    & 4294967292 as libc::c_ulong) as isize,
                            ) as *mut libc::c_void as *const libc::c_void,
                        ::std::mem::size_of::<in_addr>() as libc::c_ulong as _,
                    );
                    tmp___3 = memcmp(
                        &mut dst_ip as *mut in_addr as *const libc::c_void,
                        gw_ip as *const libc::c_void,
                        ::std::mem::size_of::<in_addr>() as libc::c_ulong,
                    );
                    if tmp___3 == 0 as libc::c_int {
                        correct_ip = 1 as libc::c_int;
                    }
                }
                _ => {}
            }
            rt_len = (rt_len as libc::c_uint)
                .wrapping_sub(
                    ((*rt_attr).rta_len as libc::c_uint)
                        .wrapping_add(4 as libc::c_uint)
                        .wrapping_sub(1 as libc::c_uint) & 4294967292 as libc::c_uint,
                ) as libc::c_int;
            rt_attr = (rt_attr as *mut libc::c_char)
                .offset(
                    (((*rt_attr).rta_len as libc::c_uint)
                        .wrapping_add(4 as libc::c_uint)
                        .wrapping_sub(1 as libc::c_uint) & 4294967292 as libc::c_uint)
                        as isize,
                ) as *mut rtattr;
        }
        if correct_ip != 0 {
            memcpy(
                hw_mac as *mut libc::c_void,
                mac.as_mut_ptr() as *const libc::c_void,
                6 as libc::c_int as size_t as _,
            );
            free(buf as *mut libc::c_void);
            return 0 as libc::c_int;
        }
        nl_len = (nl_len as libc::c_uint)
            .wrapping_sub(
                ((*nlhdr).nlmsg_len)
                    .wrapping_add(4 as libc::c_uint)
                    .wrapping_sub(1 as libc::c_uint) & 4294967292 as libc::c_uint,
            ) as libc::c_int;
        nlhdr = (nlhdr as *mut libc::c_char)
            .offset(
                (((*nlhdr).nlmsg_len)
                    .wrapping_add(4 as libc::c_uint)
                    .wrapping_sub(1 as libc::c_uint) & 4294967292 as libc::c_uint)
                    as isize,
            ) as *mut nlmsghdr;
    }
    free(buf as *mut libc::c_void);
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn _get_default_gw(
    mut gw: *mut in_addr,
    mut iface: *mut libc::c_char,
) -> libc::c_int {
    let mut req: rtmsg = rtmsg {
        rtm_family: 0,
        rtm_dst_len: 0,
        rtm_src_len: 0,
        rtm_tos: 0,
        rtm_table: 0,
        rtm_protocol: 0,
        rtm_scope: 0,
        rtm_type: 0,
        rtm_flags: 0,
    };
    let mut nl_len: libc::c_uint = 0;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut nlhdr: *mut nlmsghdr = 0 as *mut nlmsghdr;
    let mut sock: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut rt_attr: *mut rtattr = 0 as *mut rtattr;
    let mut rt_msg: *mut rtmsg = 0 as *mut rtmsg;
    let mut rt_len: libc::c_int = 0;
    let mut has_gw: libc::c_int = 0;
    if gw.is_null() {
        return -(1 as libc::c_int)
    } else {
        if iface.is_null() {
            return -(1 as libc::c_int);
        }
    }
    memset(
        &mut req as *mut rtmsg as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<rtmsg>() as libc::c_ulong as _,
    );
    tmp = send_nl_req(
        26 as libc::c_int as uint16_t,
        0 as libc::c_int as uint32_t,
        &mut req as *mut rtmsg as *mut libc::c_void,
        ::std::mem::size_of::<rtmsg>() as libc::c_ulong as uint32_t,
    );
    sock = tmp;
    tmp___0 = read_nl_sock(
        sock,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong as libc::c_int,
    );
    nl_len = tmp___0 as libc::c_uint;
    if nl_len <= 0 as libc::c_uint {
        return -(1 as libc::c_int);
    }
    nlhdr = buf.as_mut_ptr() as *mut nlmsghdr;
    while nl_len
        >= ::std::mem::size_of::<nlmsghdr>() as libc::c_ulong as libc::c_int
            as libc::c_uint
    {
        if !((*nlhdr).nlmsg_len as libc::c_ulong
            >= ::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
        {
            break;
        }
        if !((*nlhdr).nlmsg_len <= nl_len) {
            break;
        }
        has_gw = 0 as libc::c_int;
        rt_msg = (nlhdr as *mut libc::c_char)
            .offset(
                ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
                    .wrapping_add(4 as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_ulong) & 4294967292 as libc::c_ulong)
                    as libc::c_int as isize,
            ) as *mut libc::c_void as *mut rtmsg;
        if (*rt_msg).rtm_family as libc::c_int != 2 as libc::c_int {
            nl_len = nl_len
                .wrapping_sub(
                    ((*nlhdr).nlmsg_len)
                        .wrapping_add(4 as libc::c_uint)
                        .wrapping_sub(1 as libc::c_uint) & 4294967292 as libc::c_uint,
                );
            nlhdr = (nlhdr as *mut libc::c_char)
                .offset(
                    (((*nlhdr).nlmsg_len)
                        .wrapping_add(4 as libc::c_uint)
                        .wrapping_sub(1 as libc::c_uint) & 4294967292 as libc::c_uint)
                        as isize,
                ) as *mut nlmsghdr;
        } else if (*rt_msg).rtm_table as libc::c_int != 254 as libc::c_int {
            nl_len = nl_len
                .wrapping_sub(
                    ((*nlhdr).nlmsg_len)
                        .wrapping_add(4 as libc::c_uint)
                        .wrapping_sub(1 as libc::c_uint) & 4294967292 as libc::c_uint,
                );
            nlhdr = (nlhdr as *mut libc::c_char)
                .offset(
                    (((*nlhdr).nlmsg_len)
                        .wrapping_add(4 as libc::c_uint)
                        .wrapping_sub(1 as libc::c_uint) & 4294967292 as libc::c_uint)
                        as isize,
                ) as *mut nlmsghdr;
        } else {
            rt_attr = (rt_msg as *mut libc::c_char)
                .offset(
                    ((::std::mem::size_of::<rtmsg>() as libc::c_ulong)
                        .wrapping_add(4 as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_ulong) & 4294967292 as libc::c_ulong)
                        as isize,
                ) as *mut rtattr;
            rt_len = ((*nlhdr).nlmsg_len as libc::c_ulong)
                .wrapping_sub(
                    (::std::mem::size_of::<rtmsg>() as libc::c_ulong)
                        .wrapping_add(
                            ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
                                .wrapping_add(4 as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_ulong)
                                & 4294967292 as libc::c_ulong) as libc::c_int
                                as libc::c_ulong,
                        )
                        .wrapping_add(4 as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_ulong) & 4294967292 as libc::c_ulong,
                ) as libc::c_int;
            while rt_len
                >= ::std::mem::size_of::<rtattr>() as libc::c_ulong as libc::c_int
            {
                if !((*rt_attr).rta_len as libc::c_ulong
                    >= ::std::mem::size_of::<rtattr>() as libc::c_ulong)
                {
                    break;
                }
                if !((*rt_attr).rta_len as libc::c_int <= rt_len) {
                    break;
                }
                match (*rt_attr).rta_type as libc::c_int {
                    4 => {
                        if_indextoname(
                            *((rt_attr as *mut libc::c_char)
                                .offset(
                                    ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
                                        .wrapping_add(4 as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_ulong)
                                        & 4294967292 as libc::c_ulong) as isize,
                                ) as *mut libc::c_void as *mut libc::c_int) as libc::c_uint,
                            iface,
                        );
                    }
                    5 => {
                        (*gw)
                            .s_addr = *((rt_attr as *mut libc::c_char)
                            .offset(
                                ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
                                    .wrapping_add(4 as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_ulong)
                                    & 4294967292 as libc::c_ulong) as isize,
                            ) as *mut libc::c_void as *mut libc::c_uint);
                        has_gw = 1 as libc::c_int;
                    }
                    _ => {}
                }
                rt_len = (rt_len as libc::c_uint)
                    .wrapping_sub(
                        ((*rt_attr).rta_len as libc::c_uint)
                            .wrapping_add(4 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint) & 4294967292 as libc::c_uint,
                    ) as libc::c_int;
                rt_attr = (rt_attr as *mut libc::c_char)
                    .offset(
                        (((*rt_attr).rta_len as libc::c_uint)
                            .wrapping_add(4 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint)
                            & 4294967292 as libc::c_uint) as isize,
                    ) as *mut rtattr;
            }
            if has_gw != 0 {
                return 0 as libc::c_int;
            }
            nl_len = nl_len
                .wrapping_sub(
                    ((*nlhdr).nlmsg_len)
                        .wrapping_add(4 as libc::c_uint)
                        .wrapping_sub(1 as libc::c_uint) & 4294967292 as libc::c_uint,
                );
            nlhdr = (nlhdr as *mut libc::c_char)
                .offset(
                    (((*nlhdr).nlmsg_len)
                        .wrapping_add(4 as libc::c_uint)
                        .wrapping_sub(1 as libc::c_uint) & 4294967292 as libc::c_uint)
                        as isize,
                ) as *mut nlmsghdr;
        }
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn get_default_gw(
    mut gw: *mut in_addr,
    mut iface: *mut libc::c_char,
) -> libc::c_int {
    let mut _iface: [libc::c_char; 16] = [0; 16];
    let mut tmp: libc::c_int = 0;
    memset(
        _iface.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        16 as libc::c_int as size_t as _,
    );
    _get_default_gw(gw, _iface.as_mut_ptr());
    tmp = strcmp(
        iface as *const libc::c_char,
        _iface.as_mut_ptr() as *const libc::c_char,
    );
    if tmp != 0 {
        log_fatal(
            b"get-gateway\0" as *const u8 as *const libc::c_char,
            b"The specified network (\"%s\") does not match the interface associated with the default gateway (%s). You will need to manually specify the MAC address of your gateway using the \"--gateway-mac\" flag.\0"
                as *const u8 as *const libc::c_char,
            iface,
            _iface.as_mut_ptr(),
        );
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn get_iface_ip(
    mut iface: *mut libc::c_char,
    mut ip: *mut in_addr,
) -> libc::c_int {
    let mut sock: libc::c_int = 0;
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
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: libc::c_int = 0;
    memset(
        &mut ifr as *mut ifreq as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<ifreq>() as libc::c_ulong as _,
    );
    sock = socket(2 as libc::c_int, 2 as libc::c_int, 0 as libc::c_int);
    if sock < 0 as libc::c_int {
        tmp = __errno_location();
        tmp___0 = strerror(*tmp);
        log_fatal(
            b"get-iface-ip\0" as *const u8 as *const libc::c_char,
            b"failure opening socket: %s\0" as *const u8 as *const libc::c_char,
            tmp___0,
        );
    }
    ifr.ifr_ifru.ifru_addr.sa_family = 2 as libc::c_int as sa_family_t;
    strncpy(
        (ifr.ifr_ifrn.ifrn_name).as_mut_ptr(),
        iface as *const libc::c_char,
        15 as libc::c_int as size_t as _,
    );
    tmp___3 = ioctl(sock, 35093 as libc::c_ulong, &mut ifr as *mut ifreq);
    if tmp___3 < 0 as libc::c_int {
        close(sock);
        tmp___1 = __errno_location();
        tmp___2 = strerror(*tmp___1);
        log_fatal(
            b"get-iface-ip\0" as *const u8 as *const libc::c_char,
            b"Unable to automatically identify the correct source address for %s interface. ioctl failure: %s. If this is the unexpected interface, you can manually specify the correct interface with \"-i\" flag. If this is the correct interface, you likely need to manually specify the source IP address to use with the \"-S\" flag.\0"
                as *const u8 as *const libc::c_char,
            iface,
            tmp___2,
        );
    }
    (*ip)
        .s_addr = (*(&mut ifr.ifr_ifru.ifru_addr as *mut sockaddr as *mut sockaddr_in))
        .sin_addr
        .s_addr;
    close(sock);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn get_iface_hw_addr(
    mut iface: *mut libc::c_char,
    mut hw_mac: *mut libc::c_uchar,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut buffer: ifreq = ifreq {
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
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    s = socket(2 as libc::c_int, 2 as libc::c_int, 0 as libc::c_int);
    if s < 0 as libc::c_int {
        tmp = __errno_location();
        tmp___0 = strerror(*tmp);
        log_error(
            b"get_iface_hw_addr\0" as *const u8 as *const libc::c_char,
            b"Unable to open socket: %s\0" as *const u8 as *const libc::c_char,
            tmp___0,
        );
        return 1 as libc::c_int;
    }
    memset(
        &mut buffer as *mut ifreq as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<ifreq>() as libc::c_ulong as _,
    );
    strncpy(
        (buffer.ifr_ifrn.ifrn_name).as_mut_ptr(),
        iface as *const libc::c_char,
        16 as libc::c_int as size_t as _,
    );
    ioctl(s, 35111 as libc::c_ulong, &mut buffer as *mut ifreq);
    close(s);
    memcpy(
        hw_mac as *mut libc::c_void,
        (buffer.ifr_ifru.ifru_hwaddr.sa_data).as_mut_ptr() as *const libc::c_void,
        6 as libc::c_int as size_t as _,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn shard_complete(
    mut thread_id: uint8_t,
    mut arg: *mut libc::c_void,
) {
    let mut it: *mut iterator_t = 0 as *mut iterator_t;
    let mut s: *mut shard_t = 0 as *mut shard_t;
    let mut done: uint8_t = 0;
    let mut i: uint8_t = 0;
    let mut tmp___0: libc::c_int = 0;
    it = arg as *mut iterator_t;
    if !((thread_id as libc::c_int) < (*it).num_threads as libc::c_int) {
        __assert_fail(
            b"thread_id < it->num_threads\0" as *const u8 as *const libc::c_char,
            b"src/iterator.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_uint,
            b"shard_complete\0" as *const u8 as *const libc::c_char,
        );
    }
    pthread_mutex_lock(&mut (*it).mutex);
    *((*it).complete)
        .offset(thread_id as libc::c_int as isize) = 1 as libc::c_int as uint8_t;
    (*it).curr_threads = ((*it).curr_threads).wrapping_sub(1);
    s = ((*it).thread_shards).offset(thread_id as libc::c_int as isize);
    zsend
        .packets_sent = (zsend.packets_sent as libc::c_ulong)
        .wrapping_add((*s).state.packets_sent) as uint64_t as uint64_t;
    zsend
        .hosts_scanned = (zsend.hosts_scanned as libc::c_ulong)
        .wrapping_add((*s).state.hosts_scanned as uint64_t) as uint64_t as uint64_t;
    zsend
        .blocklisted = (zsend.blocklisted as libc::c_ulong)
        .wrapping_add((*s).state.hosts_blocklisted as uint64_t) as uint64_t as uint64_t;
    zsend
        .allowlisted = (zsend.allowlisted as libc::c_ulong)
        .wrapping_add((*s).state.hosts_allowlisted as uint64_t) as uint64_t as uint64_t;
    zsend
        .sendto_failures = (zsend.sendto_failures as libc::c_uint)
        .wrapping_add((*s).state.packets_failed) as uint32_t as uint32_t;
    done = 1 as libc::c_int as uint8_t;
    i = 0 as libc::c_int as uint8_t;
    while done != 0 {
        if !((i as libc::c_int) < (*it).num_threads as libc::c_int) {
            break;
        }
        if done != 0 {
            if *((*it).complete).offset(i as libc::c_int as isize) != 0 {
                tmp___0 = 1 as libc::c_int;
            } else {
                tmp___0 = 0 as libc::c_int;
            }
        } else {
            tmp___0 = 0 as libc::c_int;
        }
        done = tmp___0 as uint8_t;
        i = (i as libc::c_int + 1 as libc::c_int) as uint8_t;
    }
    if done != 0 {
        zsend.finish = now();
        zsend.complete = 1 as libc::c_int;
        zsend
            .first_scanned = (*((*it).thread_shards).offset(0 as libc::c_int as isize))
            .state
            .first_scanned;
    }
    pthread_mutex_unlock(&mut (*it).mutex);
}
pub unsafe extern "C" fn iterator_init(
    mut num_threads: uint8_t,
    mut shard: uint16_t,
    mut num_shards: uint16_t,
) -> *mut iterator_t {
    let mut num_addrs: uint64_t = 0;
    let mut tmp: uint64_t = 0;
    let mut group_min_size: uint64_t = 0;
    let mut it: *mut iterator_t = 0 as *mut iterator_t;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut group: *const cyclic_group_t = 0 as *const cyclic_group_t;
    let mut tmp___1: *const cyclic_group_t = 0 as *const cyclic_group_t;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: uint8_t = 0;
    tmp = blocklist_count_allowed();
    num_addrs = tmp;
    group_min_size = num_addrs;
    if !(zconf.list_of_ips_filename).is_null() {
        log_debug(
            b"send\0" as *const u8 as *const libc::c_char,
            b"forcing max group size for compatibility with -I\0" as *const u8
                as *const libc::c_char,
        );
        group_min_size = 4294967295 as libc::c_uint as uint64_t;
    }
    tmp___0 = xmalloc(::std::mem::size_of::<iterator>() as libc::c_ulong);
    it = tmp___0 as *mut iterator_t;
    tmp___1 = get_group(group_min_size);
    group = tmp___1;
    if num_addrs as libc::c_ulonglong
        > ((1 as libc::c_longlong) << 32 as libc::c_int) as libc::c_ulonglong
    {
        zsend.max_index = 4294967295 as libc::c_uint;
    } else {
        zsend.max_index = num_addrs as uint32_t;
    }
    log_debug(
        b"iterator\0" as *const u8 as *const libc::c_char,
        b"max index %u\0" as *const u8 as *const libc::c_char,
        zsend.max_index,
    );
    (*it).cycle = make_cycle(group, zconf.aes);
    (*it).num_threads = num_threads;
    (*it).curr_threads = num_threads as uint32_t;
    tmp___2 = xcalloc(
        num_threads as size_t,
        ::std::mem::size_of::<shard_t>() as libc::c_ulong,
    );
    (*it).thread_shards = tmp___2 as *mut shard_t;
    tmp___3 = xcalloc(
        (*it).num_threads as size_t,
        ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
    );
    (*it).complete = tmp___3 as *mut uint8_t;
    pthread_mutex_init(
        &mut (*it).mutex,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
    i = 0 as libc::c_int as uint8_t;
    while (i as libc::c_int) < num_threads as libc::c_int {
        shard_init(
            ((*it).thread_shards).offset(i as libc::c_int as isize),
            shard,
            num_shards,
            i,
            num_threads,
            zsend.max_targets,
            &mut (*it).cycle as *mut cycle_t as *const cycle_t,
            Some(
                shard_complete as unsafe extern "C" fn(uint8_t, *mut libc::c_void) -> (),
            ),
            it as *mut libc::c_void,
        );
        i = (i as libc::c_int + 1 as libc::c_int) as uint8_t;
    }
    zconf.generator = (*it).cycle.generator as uint32_t;
    return it;
}
pub unsafe extern "C" fn iterator_get_sent(mut it: *mut iterator_t) -> uint64_t {
    let mut sent: uint64_t = 0;
    let mut i: uint8_t = 0;
    sent = 0 as libc::c_int as uint64_t;
    i = 0 as libc::c_int as uint8_t;
    while (i as libc::c_int) < (*it).num_threads as libc::c_int {
        sent = (sent as libc::c_ulong)
            .wrapping_add(
                (*((*it).thread_shards).offset(i as libc::c_int as isize))
                    .state
                    .packets_sent,
            ) as uint64_t as uint64_t;
        i = (i as libc::c_int + 1 as libc::c_int) as uint8_t;
    }
    return sent;
}
pub unsafe extern "C" fn iterator_get_iterations(mut it: *mut iterator_t) -> uint64_t {
    let mut iterations: uint64_t = 0;
    let mut i: uint8_t = 0;
    iterations = 0 as libc::c_int as uint64_t;
    i = 0 as libc::c_int as uint8_t;
    while (i as libc::c_int) < (*it).num_threads as libc::c_int {
        iterations = (iterations as libc::c_ulong)
            .wrapping_add(
                (*((*it).thread_shards).offset(i as libc::c_int as isize)).iterations,
            ) as uint64_t as uint64_t;
        i = (i as libc::c_int + 1 as libc::c_int) as uint8_t;
    }
    return iterations;
}
pub unsafe extern "C" fn iterator_get_fail(mut it: *mut iterator_t) -> uint32_t {
    let mut fails: uint32_t = 0;
    let mut i: uint8_t = 0;
    fails = 0 as libc::c_int as uint32_t;
    i = 0 as libc::c_int as uint8_t;
    while (i as libc::c_int) < (*it).num_threads as libc::c_int {
        fails = (fails as libc::c_uint)
            .wrapping_add(
                (*((*it).thread_shards).offset(i as libc::c_int as isize))
                    .state
                    .packets_failed,
            ) as uint32_t as uint32_t;
        i = (i as libc::c_int + 1 as libc::c_int) as uint8_t;
    }
    return fails;
}
pub unsafe extern "C" fn get_shard(
    mut it: *mut iterator_t,
    mut thread_id: uint8_t,
) -> *mut shard_t {
    if !((thread_id as libc::c_int) < (*it).num_threads as libc::c_int) {
        __assert_fail(
            b"thread_id < it->num_threads\0" as *const u8 as *const libc::c_char,
            b"src/iterator.c\0" as *const u8 as *const libc::c_char,
            120 as libc::c_uint,
            b"get_shard\0" as *const u8 as *const libc::c_char,
        );
    }
    return ((*it).thread_shards).offset(thread_id as libc::c_int as isize);
}
pub unsafe extern "C" fn iterator_get_curr_send_threads(
    mut it: *mut iterator_t,
) -> uint32_t {
    if it.is_null() {
        __assert_fail(
            b"it\0" as *const u8 as *const libc::c_char,
            b"src/iterator.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_uint,
            b"iterator_get_curr_send_threads\0" as *const u8 as *const libc::c_char,
        );
    }
    return (*it).curr_threads;
}
static mut status_fd: *mut FILE = 0 as *const libc::c_void as *mut libc::c_void
    as *mut FILE;
unsafe extern "C" fn min_d(
    mut array: *mut libc::c_double,
    mut n: libc::c_int,
) -> libc::c_double {
    let mut value: libc::c_double = 0.;
    let mut tmp: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    tmp = ::std::f32::INFINITY;
    value = tmp as libc::c_double;
    i = 0 as libc::c_int;
    while i < n {
        if *array.offset(i as isize) < value {
            value = *array.offset(i as isize);
        }
        i += 1;
    }
    return value;
}
pub unsafe extern "C" fn compute_remaining_time(
    mut age: libc::c_double,
    mut packets_sent: uint64_t,
    mut iterations: uint64_t,
) -> libc::c_double {
    let mut remaining: [libc::c_double; 5] = [0.; 5];
    let mut tmp: libc::c_float = 0.;
    let mut tmp___0: libc::c_float = 0.;
    let mut tmp___1: libc::c_float = 0.;
    let mut tmp___2: libc::c_float = 0.;
    let mut tmp___3: libc::c_float = 0.;
    let mut done: libc::c_double = 0.;
    let mut done___0: libc::c_double = 0.;
    let mut done___1: libc::c_double = 0.;
    let mut done___2: libc::c_double = 0.;
    let mut tmp___4: libc::c_double = 0.;
    let mut tmp___5: libc::c_double = 0.;
    if zsend.complete == 0 {
        tmp = ::std::f32::INFINITY;
        tmp___0 = ::std::f32::INFINITY;
        tmp___1 = ::std::f32::INFINITY;
        tmp___2 = ::std::f32::INFINITY;
        tmp___3 = ::std::f32::INFINITY;
        remaining[0 as libc::c_int as usize] = tmp as libc::c_double;
        remaining[1 as libc::c_int as usize] = tmp___0 as libc::c_double;
        remaining[2 as libc::c_int as usize] = tmp___1 as libc::c_double;
        remaining[3 as libc::c_int as usize] = tmp___2 as libc::c_double;
        remaining[4 as libc::c_int as usize] = tmp___3 as libc::c_double;
        if !(zsend.list_of_ips_pbm).is_null() {
            done = iterations as libc::c_double
                / (4294967295 as libc::c_ulong)
                    .wrapping_div(zconf.total_shards as uint64_t) as libc::c_double;
            remaining[0 as libc::c_int
                as usize] = (1.0f64 - done) * (age / done)
                + zconf.cooldown_secs as libc::c_double;
        }
        if zsend.max_targets != 0 {
            done___0 = packets_sent as libc::c_double
                / (zsend.max_targets as uint64_t)
                    .wrapping_mul(zconf.packet_streams as uint64_t)
                    .wrapping_div(zconf.total_shards as uint64_t) as libc::c_double;
            remaining[1 as libc::c_int
                as usize] = (1.0f64 - done___0) * (age / done___0)
                + zconf.cooldown_secs as libc::c_double;
        }
        if zconf.max_runtime != 0 {
            remaining[2 as libc::c_int
                as usize] = zconf.max_runtime as libc::c_double - age
                + zconf.cooldown_secs as libc::c_double;
        }
        if zconf.max_results != 0 {
            done___1 = zrecv.filter_success as libc::c_double
                / zconf.max_results as libc::c_double;
            remaining[3 as libc::c_int
                as usize] = (1.0f64 - done___1) * (age / done___1);
        }
        if zsend.max_index != 0 {
            done___2 = packets_sent as libc::c_double
                / (zsend.max_index)
                    .wrapping_mul(zconf.packet_streams as uint32_t)
                    .wrapping_div(zconf.total_shards as uint32_t) as libc::c_double;
            remaining[4 as libc::c_int
                as usize] = (1.0f64 - done___2) * (age / done___2)
                + zconf.cooldown_secs as libc::c_double;
        }
        tmp___4 = min_d(
            remaining.as_mut_ptr(),
            (::std::mem::size_of::<[libc::c_double; 5]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                as libc::c_int,
        );
        return tmp___4;
    } else {
        tmp___5 = now();
        return zconf.cooldown_secs as libc::c_double - (tmp___5 - zsend.finish);
    };
}
unsafe extern "C" fn update_pcap_stats(mut recv_ready_mutex___0: *mut pthread_mutex_t) {
    pthread_mutex_lock(recv_ready_mutex___0);
    recv_update_stats();
    pthread_mutex_unlock(recv_ready_mutex___0);
}
unsafe extern "C" fn log_drop_warnings(mut exp___0: *mut export_status_t) {
    if (*exp___0).pcap_drop_last / (*exp___0).recv_rate > 0.05f64 {
        log_warn(
            b"monitor\0" as *const u8 as *const libc::c_char,
            b"Dropped %.0f packets in the last second, (%u total dropped (pcap: %u + iface: %u))\0"
                as *const u8 as *const libc::c_char,
            (*exp___0).pcap_drop_last,
            (*exp___0).pcap_drop_total,
            (*exp___0).pcap_drop,
            (*exp___0).pcap_ifdrop,
        );
    }
    if (*exp___0).fail_last / (*exp___0).send_rate > 0.01f64 {
        log_warn(
            b"monitor\0" as *const u8 as *const libc::c_char,
            b"Failed to send %.0f packets/sec (%u total failures)\0" as *const u8
                as *const libc::c_char,
            (*exp___0).fail_last,
            (*exp___0).fail_total,
        );
    }
}
unsafe extern "C" fn onscreen_appsuccess(mut exp___0: *mut export_status_t) {
    if (*exp___0).complete == 0 {
        __fprintf_chk(
            stderr,
            1 as libc::c_int,
            b"%5s %0.0f%%%s; sent: %lu %sp/s (%sp/s avg); recv: %u %sp/s (%sp/s avg); app success: %u %sp/s (%sp/s avg); drops: %sp/s (%sp/s avg); hitrate: %0.2f%% app hitrate: %0.2f%%\n\0"
                as *const u8 as *const libc::c_char,
            ((*exp___0).time_past_str).as_mut_ptr(),
            (*exp___0).percent_complete,
            ((*exp___0).time_remaining_str).as_mut_ptr(),
            (*exp___0).total_sent,
            ((*exp___0).send_rate_str).as_mut_ptr(),
            ((*exp___0).send_rate_avg_str).as_mut_ptr(),
            (*exp___0).recv_success_unique,
            ((*exp___0).recv_rate_str).as_mut_ptr(),
            ((*exp___0).recv_avg_str).as_mut_ptr(),
            (*exp___0).app_recv_success_unique,
            ((*exp___0).app_success_rate_str).as_mut_ptr(),
            ((*exp___0).app_success_avg_str).as_mut_ptr(),
            ((*exp___0).pcap_drop_last_str).as_mut_ptr(),
            ((*exp___0).pcap_drop_avg_str).as_mut_ptr(),
            (*exp___0).hitrate,
            (*exp___0).app_hitrate,
        );
    } else {
        __fprintf_chk(
            stderr,
            1 as libc::c_int,
            b"%5s %0.0f%%%s; sent: %lu done (%sp/s avg); recv: %u %sp/s (%sp/s avg); app success: %u %sp/s (%sp/s avg); drops: %sp/s (%sp/s avg); hitrate: %0.2f%% app hitrate: %0.2f%%\n\0"
                as *const u8 as *const libc::c_char,
            ((*exp___0).time_past_str).as_mut_ptr(),
            (*exp___0).percent_complete,
            ((*exp___0).time_remaining_str).as_mut_ptr(),
            (*exp___0).total_sent,
            ((*exp___0).send_rate_avg_str).as_mut_ptr(),
            (*exp___0).recv_success_unique,
            ((*exp___0).recv_rate_str).as_mut_ptr(),
            ((*exp___0).recv_avg_str).as_mut_ptr(),
            (*exp___0).app_recv_success_unique,
            ((*exp___0).app_success_rate_str).as_mut_ptr(),
            ((*exp___0).app_success_avg_str).as_mut_ptr(),
            ((*exp___0).pcap_drop_last_str).as_mut_ptr(),
            ((*exp___0).pcap_drop_avg_str).as_mut_ptr(),
            (*exp___0).hitrate,
            (*exp___0).app_hitrate,
        );
    };
}
unsafe extern "C" fn onscreen_generic(mut exp___0: *mut export_status_t) {
    if (*exp___0).complete == 0 {
        __fprintf_chk(
            stderr,
            1 as libc::c_int,
            b"%5s %0.0f%%%s; send: %lu %sp/s (%sp/s avg); recv: %u %sp/s (%sp/s avg); drops: %sp/s (%sp/s avg); hitrate: %0.2f%%\n\0"
                as *const u8 as *const libc::c_char,
            ((*exp___0).time_past_str).as_mut_ptr(),
            (*exp___0).percent_complete,
            ((*exp___0).time_remaining_str).as_mut_ptr(),
            (*exp___0).total_sent,
            ((*exp___0).send_rate_str).as_mut_ptr(),
            ((*exp___0).send_rate_avg_str).as_mut_ptr(),
            (*exp___0).recv_success_unique,
            ((*exp___0).recv_rate_str).as_mut_ptr(),
            ((*exp___0).recv_avg_str).as_mut_ptr(),
            ((*exp___0).pcap_drop_last_str).as_mut_ptr(),
            ((*exp___0).pcap_drop_avg_str).as_mut_ptr(),
            (*exp___0).hitrate,
        );
    } else {
        __fprintf_chk(
            stderr,
            1 as libc::c_int,
            b"%5s %0.0f%%%s; send: %lu done (%sp/s avg); recv: %u %sp/s (%sp/s avg); drops: %sp/s (%sp/s avg); hitrate: %0.2f%%\n\0"
                as *const u8 as *const libc::c_char,
            ((*exp___0).time_past_str).as_mut_ptr(),
            (*exp___0).percent_complete,
            ((*exp___0).time_remaining_str).as_mut_ptr(),
            (*exp___0).total_sent,
            ((*exp___0).send_rate_avg_str).as_mut_ptr(),
            (*exp___0).recv_success_unique,
            ((*exp___0).recv_rate_str).as_mut_ptr(),
            ((*exp___0).recv_avg_str).as_mut_ptr(),
            ((*exp___0).pcap_drop_last_str).as_mut_ptr(),
            ((*exp___0).pcap_drop_avg_str).as_mut_ptr(),
            (*exp___0).hitrate,
        );
    }
    fflush(stderr);
}
unsafe extern "C" fn init_status_update_file(mut path: *mut libc::c_char) -> *mut FILE {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut tmp: *mut FILE = 0 as *mut FILE;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = fopen(path as *const libc::c_char, b"w\0" as *const u8 as *const libc::c_char);
    f = tmp;
    if f.is_null() {
        tmp___0 = __errno_location();
        tmp___1 = strerror(*tmp___0);
        log_fatal(
            b"csv\0" as *const u8 as *const libc::c_char,
            b"could not open status updates file (%s): %s\0" as *const u8
                as *const libc::c_char,
            zconf.status_updates_file,
            tmp___1,
        );
    }
    log_debug(
        b"monitor\0" as *const u8 as *const libc::c_char,
        b"status updates CSV will be saved to %s\0" as *const u8 as *const libc::c_char,
        zconf.status_updates_file,
    );
    __fprintf_chk(
        f,
        1 as libc::c_int,
        b"real-time,time-elapsed,time-remaining,percent-complete,hit-rate,active-send-threads,sent-total,sent-last-one-sec,sent-avg-per-sec,recv-success-total,recv-success-last-one-sec,recv-success-avg-per-sec,recv-total,recv-total-last-one-sec,recv-total-avg-per-sec,pcap-drop-total,drop-last-one-sec,drop-avg-per-sec,sendto-fail-total,sendto-fail-last-one-sec,sendto-fail-avg-per-sec\n\0"
            as *const u8 as *const libc::c_char,
    );
    fflush(f);
    return f;
}
unsafe extern "C" fn update_status_updates_file(
    mut exp___0: *mut export_status_t,
    mut f: *mut FILE,
) {
    let mut now___0: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut timestamp: [libc::c_char; 256] = [0; 256];
    let mut sec: time_t = 0;
    let mut ptm: *mut tm = 0 as *mut tm;
    let mut tmp: *mut tm = 0 as *mut tm;
    gettimeofday(&mut now___0 as *mut timeval, 0 as *mut libc::c_void);
    sec = now___0.tv_sec;
    tmp = localtime(&mut sec as *mut time_t as *const time_t);
    ptm = tmp;
    strftime(
        timestamp.as_mut_ptr(),
        20 as libc::c_int as size_t,
        b"%Y-%m-%d %H:%M:%S\0" as *const u8 as *const libc::c_char,
        ptm as *const tm,
    );
    __fprintf_chk(
        f,
        1 as libc::c_int,
        b"%s,%u,%u,%f,%f,%u,%lu,%.0f,%.0f,%u,%.0f,%.0f,%lu,%.0f,%.0f,%u,%.0f,%.0f,%u,%.0f,%.0f\n\0"
            as *const u8 as *const libc::c_char,
        timestamp.as_mut_ptr(),
        (*exp___0).time_past,
        (*exp___0).time_remaining,
        (*exp___0).percent_complete,
        (*exp___0).hitrate,
        (*exp___0).send_threads,
        (*exp___0).total_sent,
        (*exp___0).send_rate,
        (*exp___0).send_rate_avg,
        (*exp___0).recv_success_unique,
        (*exp___0).recv_rate,
        (*exp___0).recv_avg,
        (*exp___0).total_recv,
        (*exp___0).recv_total_rate,
        (*exp___0).recv_total_avg,
        (*exp___0).pcap_drop_total,
        (*exp___0).pcap_drop_last,
        (*exp___0).pcap_drop_avg,
        (*exp___0).fail_total,
        (*exp___0).fail_last,
        (*exp___0).fail_avg,
    );
    fflush(f);
}
#[inline]
unsafe extern "C" fn check_min_hitrate(mut exp___0: *mut export_status_t) {
    if (*exp___0).seconds_under_min_hitrate >= 5 as libc::c_int as libc::c_float {
        log_fatal(
            b"monitor\0" as *const u8 as *const libc::c_char,
            b"hitrate below %.0f for %.0f seconds. aborting scan.\0" as *const u8
                as *const libc::c_char,
            zconf.min_hitrate as libc::c_double,
            (*exp___0).seconds_under_min_hitrate as libc::c_double,
        );
    }
}
#[inline]
unsafe extern "C" fn check_max_sendto_failures(mut exp___0: *mut export_status_t) {
    if zconf.max_sendto_failures >= 0 as libc::c_int {
        if (*exp___0).fail_total > zconf.max_sendto_failures as uint32_t {
            log_fatal(
                b"monitor\0" as *const u8 as *const libc::c_char,
                b"maximum number of sendto failures (%i) exceeded\0" as *const u8
                    as *const libc::c_char,
                zconf.max_sendto_failures,
            );
        }
    }
}
pub unsafe extern "C" fn monitor_init() {
    if !(zconf.status_updates_file).is_null() {
        status_fd = init_status_update_file(zconf.status_updates_file);
        if status_fd.is_null() {
            __assert_fail(
                b"status_fd\0" as *const u8 as *const libc::c_char,
                b"src/monitor.c\0" as *const u8 as *const libc::c_char,
                449 as libc::c_uint,
                b"monitor_init\0" as *const u8 as *const libc::c_char,
            );
        }
    }
}
pub unsafe extern "C" fn monitor_run(
    mut it: *mut iterator_t,
    mut lock: *mut pthread_mutex_t,
) {
    let mut internal_status: *mut int_status_t = 0 as *mut int_status_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut export_status: *mut export_status_t = 0 as *mut export_status_t;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = xmalloc(::std::mem::size_of::<int_status_t>() as libc::c_ulong);
    internal_status = tmp as *mut int_status_t;
    tmp___0 = xmalloc(::std::mem::size_of::<export_status_t>() as libc::c_ulong);
    export_status = tmp___0 as *mut export_status_t;
    loop {
        if zsend.complete != 0 {
            if zrecv.complete != 0 {
                break;
            }
        }
        update_pcap_stats(lock);
        log_drop_warnings(export_status);
        check_min_hitrate(export_status);
        check_max_sendto_failures(export_status);
        if zconf.quiet == 0 {
            lock_file(stderr);
            if zconf.fsconf.app_success_index >= 0 as libc::c_int {
                onscreen_appsuccess(export_status);
            } else {
                onscreen_generic(export_status);
            }
            unlock_file(stderr);
        }
        if !status_fd.is_null() {
            update_status_updates_file(export_status, status_fd);
        }
        sleep(1 as libc::c_uint);
    }
    if zconf.quiet == 0 {
        lock_file(stderr);
        fflush(stderr);
        unlock_file(stderr);
    }
    if !status_fd.is_null() {
        fflush(status_fd);
        fclose(status_fd);
    }
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
static mut fake_eth_hdr: [u_char; 65535] = [0; 65535];
static mut seen: *mut *mut uint8_t = 0 as *const libc::c_void as *mut libc::c_void
    as *mut *mut uint8_t;
pub unsafe extern "C" fn handle_packet(
    mut buflen: uint32_t,
    mut bytes: *const libc::c_uchar,
    ts: timespec,
) {
    let mut current_block: u64;
    let mut ip_hdr: *mut ip = 0 as *mut ip;
    let mut src_ip: uint32_t = 0;
    let mut validation: [uint32_t; 16] = [0; 16];
    let mut tmp: libc::c_ulong = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut is_repeat: libc::c_int = 0;
    let mut tmp___1: __uint32_t = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut fs: *mut fieldset_t = 0 as *mut fieldset_t;
    let mut tmp___3: *mut fieldset_t = 0 as *mut fieldset_t;
    let mut success_index: libc::c_int = 0;
    let mut is_success: libc::c_int = 0;
    let mut tmp___5: uint64_t = 0;
    let mut tmp___6: __uint32_t = 0;
    let mut is_app_success: libc::c_int = 0;
    let mut tmp___7: uint64_t = 0;
    let mut o: *mut fieldset_t = 0 as *mut fieldset_t;
    let mut tmp___8: libc::c_int = 0;
    if (::std::mem::size_of::<ip>() as libc::c_ulong)
        .wrapping_add(zconf.data_link_size as libc::c_ulong) > buflen as libc::c_ulong
    {
        return;
    }
    ip_hdr = bytes.offset(zconf.data_link_size as isize) as *mut ip;
    src_ip = (*ip_hdr).ip_src.s_addr;
    validate_gen(
        (*ip_hdr).ip_dst.s_addr,
        (*ip_hdr).ip_src.s_addr,
        validation.as_mut_ptr() as *mut uint8_t,
    );
    if zconf.send_ip_pkts != 0 {
        tmp = 0 as libc::c_ulong;
    } else {
        tmp = ::std::mem::size_of::<ether_header>() as libc::c_ulong;
    }
    tmp___0 = (Some(
        ((*zconf.probe_module).validate_packet).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(
        ip_hdr as *const ip,
        (buflen as libc::c_ulong).wrapping_sub(tmp) as uint32_t,
        &mut src_ip,
        validation.as_mut_ptr(),
    );
    if tmp___0 != 0 {
        zrecv.validation_passed = (zrecv.validation_passed).wrapping_add(1);
    } else {
        zrecv.validation_failed = (zrecv.validation_failed).wrapping_add(1);
        return;
    }
    tmp___1 = __bswap_32(src_ip);
    tmp___2 = pbm_check(seen, tmp___1);
    is_repeat = tmp___2;
    if (*ip_hdr).ip_off as libc::c_int & 8192 as libc::c_int != 0 {
        zrecv.ip_fragments = (zrecv.ip_fragments).wrapping_add(1);
    }
    tmp___3 = fs_new_fieldset(&mut zconf.fsconf.defs);
    fs = tmp___3;
    fs_add_ip_fields(fs, ip_hdr);
    if zconf.send_ip_pkts != 0 {
        if buflen as libc::c_ulong
            > ::std::mem::size_of::<[u_char; 65535]>() as libc::c_ulong
        {
            buflen = ::std::mem::size_of::<[u_char; 65535]>() as libc::c_ulong
                as uint32_t;
        }
        memcpy(
            &mut *fake_eth_hdr
                .as_mut_ptr()
                .offset(::std::mem::size_of::<ether_header>() as libc::c_ulong as isize)
                as *mut u_char as *mut libc::c_void,
            bytes.offset(zconf.data_link_size as isize) as *const libc::c_void,
            buflen as size_t as _,
        );
        bytes = fake_eth_hdr.as_mut_ptr() as *const libc::c_uchar;
    }
    (Some(((*zconf.probe_module).process_packet).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(bytes, buflen, fs, validation.as_mut_ptr(), ts);
    success_index = zconf.fsconf.success_index;
    if !(success_index < (*fs).len) {
        __assert_fail(
            b"success_index < fs->len\0" as *const u8 as *const libc::c_char,
            b"src/recv.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_uint,
            b"handle_packet\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___5 = fs_get_uint64_by_index(fs, success_index);
    is_success = tmp___5 as libc::c_int;
    if is_success != 0 {
        zrecv.success_total = (zrecv.success_total).wrapping_add(1);
        if is_repeat == 0 {
            zrecv.success_unique = (zrecv.success_unique).wrapping_add(1);
            tmp___6 = __bswap_32(src_ip);
            pbm_set(seen, tmp___6);
        }
        if zsend.complete != 0 {
            zrecv.cooldown_total = (zrecv.cooldown_total).wrapping_add(1);
            if is_repeat == 0 {
                zrecv.cooldown_unique = (zrecv.cooldown_unique).wrapping_add(1);
            }
        }
    } else {
        zrecv.failure_total = (zrecv.failure_total).wrapping_add(1);
    }
    if zconf.fsconf.app_success_index >= 0 as libc::c_int {
        tmp___7 = fs_get_uint64_by_index(fs, zconf.fsconf.app_success_index);
        is_app_success = tmp___7 as libc::c_int;
        if is_app_success != 0 {
            zrecv.app_success_total = (zrecv.app_success_total).wrapping_add(1);
            if is_repeat == 0 {
                zrecv.app_success_unique = (zrecv.app_success_unique).wrapping_add(1);
            }
        }
    }
    o = 0 as *mut libc::c_void as *mut fieldset_t;
    if is_success == 0 {
        if zconf.default_mode != 0 {
            current_block = 10480706514085580365;
        } else {
            current_block = 11441799814184323368;
        }
    } else {
        current_block = 11441799814184323368;
    }
    match current_block {
        11441799814184323368 => {
            if is_repeat != 0 {
                if zconf.default_mode != 0 {
                    current_block = 10480706514085580365;
                } else {
                    current_block = 4888910987971495881;
                }
            } else {
                current_block = 4888910987971495881;
            }
            match current_block {
                10480706514085580365 => {}
                _ => {
                    tmp___8 = evaluate_expression(zconf.filter.expression, fs);
                    if !(tmp___8 == 0) {
                        zrecv.filter_success = (zrecv.filter_success).wrapping_add(1);
                        o = translate_fieldset(fs, &mut zconf.fsconf.translation);
                        if !(zconf.output_module).is_null() {
                            if ((*zconf.output_module).process_ip).is_some() {
                                (Some(
                                    ((*zconf.output_module).process_ip)
                                        .expect("non-null function pointer"),
                                ))
                                    .expect("non-null function pointer")(o);
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    fs_free(fs);
    free(o as *mut libc::c_void);
    if !(zconf.output_module).is_null() {
        if ((*zconf.output_module).update).is_some() {
            if (zrecv.success_unique)
                .wrapping_rem((*zconf.output_module).update_interval) == 0
            {
                (Some(
                    ((*zconf.output_module).update).expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(&mut zconf, &mut zsend, &mut zrecv);
            }
        }
    }
}
pub unsafe extern "C" fn recv_run(
    mut recv_ready_mutex___0: *mut pthread_mutex_t,
) -> libc::c_int {
    let mut eth: *mut ether_header = 0 as *mut ether_header;
    let mut tmp: libc::c_double = 0.;
    log_debug(
        b"recv\0" as *const u8 as *const libc::c_char,
        b"capturing responses on %s\0" as *const u8 as *const libc::c_char,
        zconf.iface,
    );
    if zconf.dryrun == 0 {
    }
    if zconf.send_ip_pkts != 0 {
        eth = fake_eth_hdr.as_mut_ptr() as *mut ether_header;
        memset(
            fake_eth_hdr.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[u_char; 65535]>() as libc::c_ulong as _,
        );
        (*eth).ether_type = __bswap_16(2048 as libc::c_int as __uint16_t);
    }
    seen = pbm_init();
    if zconf.default_mode != 0 {
        log_info(
            b"recv\0" as *const u8 as *const libc::c_char,
            b"duplicate responses will be excluded from output\0" as *const u8
                as *const libc::c_char,
        );
        log_info(
            b"recv\0" as *const u8 as *const libc::c_char,
            b"unsuccessful responses will be excluded from output\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        log_info(
            b"recv\0" as *const u8 as *const libc::c_char,
            b"duplicate responses will be passed to the output module\0" as *const u8
                as *const libc::c_char,
        );
        log_info(
            b"recv\0" as *const u8 as *const libc::c_char,
            b"unsuccessful responses will be passed to the output module\0" as *const u8
                as *const libc::c_char,
        );
    }
    pthread_mutex_lock(recv_ready_mutex___0);
    zconf.recv_ready = 1 as libc::c_int;
    pthread_mutex_unlock(recv_ready_mutex___0);
    zrecv.start = now();
    if zconf.max_results == 0 as libc::c_uint {
        zconf.max_results = -(1 as libc::c_int) as uint32_t;
    }
    loop {
        if zconf.dryrun != 0 {
            sleep(1 as libc::c_uint);
        } else {
            recv_packets();
            if zconf.max_results != 0 {
                if zrecv.filter_success >= zconf.max_results as uint64_t {
                    break;
                }
            }
        }
        if !(zsend.complete != 0) {
            continue;
        }
        tmp = now();
        if tmp - zsend.finish > zconf.cooldown_secs as libc::c_double {
            break;
        }
    }
    zrecv.finish = now();
    recv_update_stats();
    if zconf.dryrun == 0 {
        pthread_mutex_lock(recv_ready_mutex___0);
        recv_cleanup();
        pthread_mutex_unlock(recv_ready_mutex___0);
    }
    zrecv.complete = 1 as libc::c_int;
    log_debug(
        b"recv\0" as *const u8 as *const libc::c_char,
        b"thread finished\0" as *const u8 as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
static mut sockaddr: sockaddr_ll = sockaddr_ll {
    sll_family: 0,
    sll_protocol: 0,
    sll_ifindex: 0,
    sll_hatype: 0,
    sll_pkttype: 0,
    sll_halen: 0,
    sll_addr: [0; 8],
};
unsafe extern "C" fn send_run_init(mut s: sock_t) -> libc::c_int {
    let mut sock: libc::c_int = 0;
    let mut if_idx: ifreq = ifreq {
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
    let mut tmp: size_t = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut ifindex: libc::c_int = 0;
    sock = s.sock;
    memset(
        &mut if_idx as *mut ifreq as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<ifreq>() as libc::c_ulong as _,
    );
    tmp = strlen(zconf.iface as *const libc::c_char);
    if tmp >= 16 as libc::c_ulong {
        log_error(
            b"send\0" as *const u8 as *const libc::c_char,
            b"device interface name (%s) too long\n\0" as *const u8
                as *const libc::c_char,
            zconf.iface,
        );
        return 1 as libc::c_int;
    }
    strncpy(
        (if_idx.ifr_ifrn.ifrn_name).as_mut_ptr(),
        zconf.iface as *const libc::c_char,
        15 as libc::c_int as size_t as _,
    );
    tmp___0 = ioctl(sock, 35123 as libc::c_ulong, &mut if_idx as *mut ifreq);
    if tmp___0 < 0 as libc::c_int {
        perror(b"SIOCGIFINDEX\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    ifindex = if_idx.ifr_ifru.ifru_ivalue;
    memset(
        &mut sockaddr as *mut sockaddr_ll as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sockaddr_ll>() as libc::c_ulong as _,
    );
    sockaddr.sll_ifindex = ifindex;
    sockaddr.sll_halen = 6 as libc::c_int as libc::c_uchar;
    if zconf.send_ip_pkts != 0 {
        sockaddr.sll_protocol = __bswap_16(2048 as libc::c_int as __uint16_t);
    }
    memcpy(
        (sockaddr.sll_addr).as_mut_ptr() as *mut libc::c_void,
        (zconf.gw_mac).as_mut_ptr() as *const libc::c_void,
        6 as libc::c_int as size_t as _,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn send_packet(
    mut sock: sock_t,
    mut buf: *mut libc::c_void,
    mut len: libc::c_int,
    mut idx: uint32_t,
) -> libc::c_int {
    let mut tmp: ssize_t = 0;
    tmp = sendto(
        sock.sock,
        buf as *const libc::c_void,
        len as size_t,
        0 as libc::c_int,
        &mut sockaddr as *mut sockaddr_ll as *mut sockaddr as *const sockaddr,
        ::std::mem::size_of::<sockaddr_ll>() as libc::c_ulong as socklen_t,
    );
    return tmp as libc::c_int;
}
static mut send_mutex: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
static mut num_src_ports: uint16_t = 0;
pub unsafe extern "C" fn sig_handler_increase_speed(mut signal___0: libc::c_int) {
    let mut old_rate: libc::c_int = 0;
    old_rate = zconf.rate;
    zconf
        .rate = (zconf.rate as libc::c_double + zconf.rate as libc::c_double * 0.05f64)
        as libc::c_int;
    log_info(
        b"send\0" as *const u8 as *const libc::c_char,
        b"send rate increased from %i to %i pps.\0" as *const u8 as *const libc::c_char,
        old_rate,
        zconf.rate,
    );
}
pub unsafe extern "C" fn sig_handler_decrease_speed(mut signal___0: libc::c_int) {
    let mut old_rate: libc::c_int = 0;
    old_rate = zconf.rate;
    zconf
        .rate = (zconf.rate as libc::c_double - zconf.rate as libc::c_double * 0.05f64)
        as libc::c_int;
    log_info(
        b"send\0" as *const u8 as *const libc::c_char,
        b"send rate decreased from %i to %i pps.\0" as *const u8 as *const libc::c_char,
        old_rate,
        zconf.rate,
    );
}
pub unsafe extern "C" fn send_init() -> *mut iterator_t {
    let mut it: *mut iterator_t = 0 as *mut iterator_t;
    let mut num_subshards: uint32_t = 0;
    let mut tmp: uint64_t = 0;
    let mut temp: in_addr = in_addr { s_addr: 0 };
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___4: libc::c_int = 0;
    let mut pkt_len: size_t = 0;
    let mut tmp___5: libc::c_int = 0;
    num_subshards = (zconf.senders as uint32_t)
        .wrapping_mul(zconf.total_shards as uint32_t);
    tmp = blocklist_count_allowed();
    if num_subshards as uint64_t > tmp {
        log_fatal(
            b"send\0" as *const u8 as *const libc::c_char,
            b"senders * shards > allowed probes\0" as *const u8 as *const libc::c_char,
        );
    }
    if zsend.max_targets != 0 {
        if num_subshards > zsend.max_targets {
            log_fatal(
                b"send\0" as *const u8 as *const libc::c_char,
                b"senders * shards > max targets\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    it = iterator_init(zconf.senders, zconf.shard_num, zconf.total_shards);
    temp.s_addr = zconf.source_ip_addresses[0 as libc::c_int as usize];
    tmp___0 = inet_ntoa(temp);
    log_debug(
        b"send\0" as *const u8 as *const libc::c_char,
        b"srcip_first: %s\0" as *const u8 as *const libc::c_char,
        tmp___0,
    );
    temp
        .s_addr = zconf
        .source_ip_addresses[(zconf.number_source_ips).wrapping_sub(1 as libc::c_uint)
        as usize];
    tmp___1 = inet_ntoa(temp);
    log_debug(
        b"send\0" as *const u8 as *const libc::c_char,
        b"srcip_last: %s\0" as *const u8 as *const libc::c_char,
        tmp___1,
    );
    num_src_ports = (zconf.source_port_last as libc::c_int
        - zconf.source_port_first as libc::c_int + 1 as libc::c_int) as uint16_t;
    if zconf.number_source_ips == 1 as libc::c_uint {
        tmp___2 = b"\0" as *const u8 as *const libc::c_char;
    } else {
        tmp___2 = b"es\0" as *const u8 as *const libc::c_char;
    }
    log_debug(
        b"send\0" as *const u8 as *const libc::c_char,
        b"will send from %u address%s on %hu source ports\0" as *const u8
            as *const libc::c_char,
        zconf.number_source_ips,
        tmp___2,
        num_src_ports as libc::c_int,
    );
    if (zconf.probe_module).is_null() {
        __assert_fail(
            b"zconf.probe_module\0" as *const u8 as *const libc::c_char,
            b"src/send.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_uint,
            b"send_init\0" as *const u8 as *const libc::c_char,
        );
    }
    if ((*zconf.probe_module).global_initialize).is_some() {
        tmp___4 = (Some(
            ((*zconf.probe_module).global_initialize).expect("non-null function pointer"),
        ))
            .expect("non-null function pointer")(&mut zconf);
        if tmp___4 != 0 {
            log_fatal(
                b"send\0" as *const u8 as *const libc::c_char,
                b"global initialization for probe module failed.\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    if zconf.bandwidth > 0 as libc::c_ulong {
        if zconf.rate > 0 as libc::c_int {
            log_fatal(
                b"send\0" as *const u8 as *const libc::c_char,
                b"must specify rate or bandwidth, or neither, not both.\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    if zconf.bandwidth > 0 as libc::c_ulong {
        pkt_len = (*zconf.probe_module).max_packet_length;
        pkt_len = (pkt_len as libc::c_ulong).wrapping_mul(8 as libc::c_ulong) as size_t
            as size_t;
        pkt_len = (pkt_len as libc::c_ulong).wrapping_add(192 as libc::c_ulong) as size_t
            as size_t;
        if pkt_len < 672 as libc::c_ulong {
            pkt_len = 672 as libc::c_int as size_t;
        }
        if (zconf.bandwidth).wrapping_div(pkt_len) > 4294967295 as libc::c_ulong {
            zconf.rate = 0 as libc::c_int;
        } else {
            zconf.rate = (zconf.bandwidth).wrapping_div(pkt_len) as libc::c_int;
            if zconf.rate == 0 as libc::c_int {
                log_warn(
                    b"send\0" as *const u8 as *const libc::c_char,
                    b"bandwidth %lu bit/s is slower than 1 pkt/s, setting rate to 1 pkt/s\0"
                        as *const u8 as *const libc::c_char,
                    zconf.bandwidth,
                );
                zconf.rate = 1 as libc::c_int;
            }
        }
        log_debug(
            b"send\0" as *const u8 as *const libc::c_char,
            b"using bandwidth %lu bits/s for %zu byte probe, rate set to %d pkt/s\0"
                as *const u8 as *const libc::c_char,
            zconf.bandwidth,
            pkt_len.wrapping_div(8 as libc::c_ulong),
            zconf.rate,
        );
    }
    if zconf.rate == -(1 as libc::c_int) {
        zconf.rate = 10000 as libc::c_int;
    }
    if zconf.rate < 0 as libc::c_int {
        log_fatal(
            b"send\0" as *const u8 as *const libc::c_char,
            b"rate impossibly slow\0" as *const u8 as *const libc::c_char,
        );
    }
    if zconf.rate > 0 as libc::c_int {
        if zconf.bandwidth <= 0 as libc::c_ulong {
            log_debug(
                b"send\0" as *const u8 as *const libc::c_char,
                b"rate set to %d pkt/s\0" as *const u8 as *const libc::c_char,
                zconf.rate,
            );
        }
    }
    if zconf.hw_mac_set == 0 {
        tmp___5 = get_iface_hw_addr(zconf.iface, (zconf.hw_mac).as_mut_ptr());
        if tmp___5 != 0 {
            log_fatal(
                b"send\0" as *const u8 as *const libc::c_char,
                b"ZMap could not retrieve the hardware (MAC) address for the interface \"%s\". You likely do not privileges to open a raw packet socket. Are you running as root or with the CAP_NET_RAW capability? If you are, you may need to manually set the source MAC address with the \"--source-mac\" flag.\0"
                    as *const u8 as *const libc::c_char,
                zconf.iface,
            );
        }
        log_debug(
            b"send\0" as *const u8 as *const libc::c_char,
            b"no source MAC provided. automatically detected %02x:%02x:%02x:%02x:%02x:%02x as hw interface for %s\0"
                as *const u8 as *const libc::c_char,
            zconf.hw_mac[0 as libc::c_int as usize] as libc::c_int,
            zconf.hw_mac[1 as libc::c_int as usize] as libc::c_int,
            zconf.hw_mac[2 as libc::c_int as usize] as libc::c_int,
            zconf.hw_mac[3 as libc::c_int as usize] as libc::c_int,
            zconf.hw_mac[4 as libc::c_int as usize] as libc::c_int,
            zconf.hw_mac[5 as libc::c_int as usize] as libc::c_int,
            zconf.iface,
        );
    }
    log_debug(
        b"send\0" as *const u8 as *const libc::c_char,
        b"source MAC address %02x:%02x:%02x:%02x:%02x:%02x\0" as *const u8
            as *const libc::c_char,
        zconf.hw_mac[0 as libc::c_int as usize] as libc::c_int,
        zconf.hw_mac[1 as libc::c_int as usize] as libc::c_int,
        zconf.hw_mac[2 as libc::c_int as usize] as libc::c_int,
        zconf.hw_mac[3 as libc::c_int as usize] as libc::c_int,
        zconf.hw_mac[4 as libc::c_int as usize] as libc::c_int,
        zconf.hw_mac[5 as libc::c_int as usize] as libc::c_int,
    );
    if zconf.dryrun != 0 {
        log_info(
            b"send\0" as *const u8 as *const libc::c_char,
            b"dryrun mode -- won't actually send packets\0" as *const u8
                as *const libc::c_char,
        );
    }
    validate_init();
    signal(
        10 as libc::c_int,
        Some(sig_handler_increase_speed as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    signal(
        12 as libc::c_int,
        Some(sig_handler_decrease_speed as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    zsend.start = now();
    return it;
}
#[inline]
unsafe extern "C" fn get_src_ip(
    mut dst: ipaddr_n_t,
    mut local_offset: libc::c_int,
) -> ipaddr_n_t {
    let mut tmp: __uint32_t = 0;
    if zconf.number_source_ips == 1 as libc::c_uint {
        return zconf.source_ip_addresses[0 as libc::c_int as usize];
    }
    tmp = __bswap_32(dst);
    return zconf
        .source_ip_addresses[tmp
        .wrapping_add(local_offset as __uint32_t)
        .wrapping_rem(zconf.number_source_ips) as usize];
}
unsafe extern "C" fn shard_roll_to_valid(mut s: *mut shard_t) -> uint32_t {
    let mut tmp: uint32_t = 0;
    if ((*s).current).wrapping_sub(1 as libc::c_ulong) < zsend.max_index as uint64_t {
        return (*s).current as uint32_t;
    }
    tmp = shard_get_next_ip(s);
    return tmp;
}
pub unsafe extern "C" fn shard_init(
    mut shard: *mut shard_t,
    mut shard_idx: uint16_t,
    mut num_shards: uint16_t,
    mut thread_idx: uint8_t,
    mut num_threads: uint8_t,
    mut max_total_targets: uint32_t,
    mut cycle: *const cycle_t,
    mut cb: Option::<unsafe extern "C" fn(uint8_t, *mut libc::c_void) -> ()>,
    mut arg: *mut libc::c_void,
) {
    let mut num_subshards: uint32_t = 0;
    let mut num_elts: uint64_t = 0;
    let mut sub_idx: uint32_t = 0;
    let mut exponent_begin: uint64_t = 0;
    let mut exponent_end: uint64_t = 0;
    let mut generator_m: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut exponent_begin_m: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut exponent_end_m: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut prime_m: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut start_m: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut stop_m: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut tmp___5: libc::c_ulong = 0;
    let mut tmp___6: libc::c_ulong = 0;
    let mut max_targets_this_shard: uint32_t = 0;
    if !(num_shards as libc::c_int > 0 as libc::c_int) {
        __assert_fail(
            b"num_shards > 0\0" as *const u8 as *const libc::c_char,
            b"src/shard.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_uint,
            b"shard_init\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(num_threads as libc::c_int > 0 as libc::c_int) {
        __assert_fail(
            b"num_threads > 0\0" as *const u8 as *const libc::c_char,
            b"src/shard.c\0" as *const u8 as *const libc::c_char,
            39 as libc::c_uint,
            b"shard_init\0" as *const u8 as *const libc::c_char,
        );
    }
    if !((shard_idx as libc::c_int) < num_shards as libc::c_int) {
        __assert_fail(
            b"shard_idx < num_shards\0" as *const u8 as *const libc::c_char,
            b"src/shard.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_uint,
            b"shard_init\0" as *const u8 as *const libc::c_char,
        );
    }
    if !((thread_idx as libc::c_int) < num_threads as libc::c_int) {
        __assert_fail(
            b"thread_idx < num_threads\0" as *const u8 as *const libc::c_char,
            b"src/shard.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_uint,
            b"shard_init\0" as *const u8 as *const libc::c_char,
        );
    }
    num_subshards = (num_shards as uint32_t).wrapping_mul(num_threads as uint32_t);
    num_elts = (*cycle).order;
    if !((num_subshards as uint64_t) < num_elts) {
        __assert_fail(
            b"num_subshards < num_elts\0" as *const u8 as *const libc::c_char,
            b"src/shard.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_uint,
            b"shard_init\0" as *const u8 as *const libc::c_char,
        );
    }
    if max_total_targets != 0 {
        if !(num_subshards <= max_total_targets) {
            __assert_fail(
                b"!max_total_targets || (num_subshards <= max_total_targets)\0"
                    as *const u8 as *const libc::c_char,
                b"src/shard.c\0" as *const u8 as *const libc::c_char,
                45 as libc::c_uint,
                b"shard_init\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    sub_idx = (shard_idx as libc::c_int * num_threads as libc::c_int
        + thread_idx as libc::c_int) as uint32_t;
    exponent_begin = num_elts
        .wrapping_div(num_subshards as uint64_t)
        .wrapping_mul(sub_idx as uint64_t);
    exponent_end = num_elts
        .wrapping_div(num_subshards as uint64_t)
        .wrapping_mul(
            sub_idx.wrapping_add(1 as libc::c_uint).wrapping_rem(num_subshards)
                as uint64_t,
        );
    exponent_begin = exponent_begin
        .wrapping_add((*cycle).offset as uint64_t)
        .wrapping_rem(num_elts);
    exponent_end = exponent_end
        .wrapping_add((*cycle).offset as uint64_t)
        .wrapping_rem(num_elts);
    __gmpz_init_set_ui(generator_m.as_mut_ptr(), (*cycle).generator);
    __gmpz_init_set_ui(exponent_begin_m.as_mut_ptr(), exponent_begin);
    __gmpz_init_set_ui(exponent_end_m.as_mut_ptr(), exponent_end);
    __gmpz_init_set_ui(prime_m.as_mut_ptr(), (*(*cycle).group).prime);
    __gmpz_init(start_m.as_mut_ptr());
    __gmpz_init(stop_m.as_mut_ptr());
    __gmpz_powm(
        start_m.as_mut_ptr(),
        generator_m.as_mut_ptr() as mpz_srcptr,
        exponent_begin_m.as_mut_ptr() as mpz_srcptr,
        prime_m.as_mut_ptr() as mpz_srcptr,
    );
    __gmpz_powm(
        stop_m.as_mut_ptr(),
        generator_m.as_mut_ptr() as mpz_srcptr,
        exponent_end_m.as_mut_ptr() as mpz_srcptr,
        prime_m.as_mut_ptr() as mpz_srcptr,
    );
    tmp___5 = __gmpz_get_ui(start_m.as_mut_ptr() as mpz_srcptr);
    (*shard).params.first = tmp___5;
    tmp___6 = __gmpz_get_ui(stop_m.as_mut_ptr() as mpz_srcptr);
    (*shard).params.last = tmp___6;
    (*shard).params.factor = (*cycle).generator;
    (*shard).params.modulus = (*(*cycle).group).prime;
    (*shard).current = (*shard).params.first;
    (*shard).thread_id = thread_idx;
    if max_total_targets > 0 as libc::c_uint {
        max_targets_this_shard = max_total_targets.wrapping_div(num_subshards);
        if sub_idx < max_total_targets.wrapping_rem(num_subshards) {
            max_targets_this_shard = max_targets_this_shard.wrapping_add(1);
        }
        (*shard).state.max_hosts = max_targets_this_shard;
    }
    (*shard).cb = cb;
    (*shard).arg = arg;
    shard_roll_to_valid(shard);
    __gmpz_clear(generator_m.as_mut_ptr());
    __gmpz_clear(exponent_begin_m.as_mut_ptr());
    __gmpz_clear(exponent_end_m.as_mut_ptr());
    __gmpz_clear(prime_m.as_mut_ptr());
    __gmpz_clear(start_m.as_mut_ptr());
    __gmpz_clear(stop_m.as_mut_ptr());
}
pub unsafe extern "C" fn shard_get_cur_ip(mut shard: *mut shard_t) -> uint32_t {
    let mut tmp: uint32_t = 0;
    tmp = blocklist_lookup_index(((*shard).current).wrapping_sub(1 as libc::c_ulong));
    return tmp;
}
#[inline]
unsafe extern "C" fn shard_get_next_elem(mut shard: *mut shard_t) -> uint32_t {
    loop {
        (*shard)
            .current = ((*shard).current as libc::c_ulong)
            .wrapping_mul((*shard).params.factor) as uint64_t as uint64_t;
        (*shard)
            .current = ((*shard).current as libc::c_ulong)
            .wrapping_rem((*shard).params.modulus) as uint64_t as uint64_t;
        if !((*shard).current as libc::c_ulonglong
            >= ((1 as libc::c_longlong) << 32 as libc::c_int) as libc::c_ulonglong)
        {
            break;
        }
    }
    return (*shard).current as uint32_t;
}
pub unsafe extern "C" fn shard_get_next_ip(mut shard: *mut shard_t) -> uint32_t {
    let mut candidate: uint32_t = 0;
    let mut tmp: uint32_t = 0;
    let mut tmp___0: uint32_t = 0;
    if (*shard).current == 0 as libc::c_ulong {
        return 0 as libc::c_int as uint32_t;
    }
    loop {
        tmp = shard_get_next_elem(shard);
        candidate = tmp;
        if candidate as uint64_t == (*shard).params.last {
            (*shard).current = 0 as libc::c_int as uint64_t;
            (*shard).iterations = ((*shard).iterations).wrapping_add(1);
            return 0 as libc::c_int as uint32_t;
        }
        if candidate.wrapping_sub(1 as libc::c_uint) < zsend.max_index {
            (*shard)
                .state
                .hosts_allowlisted = ((*shard).state.hosts_allowlisted).wrapping_add(1);
            (*shard).iterations = ((*shard).iterations).wrapping_add(1);
            tmp___0 = blocklist_lookup_index(
                candidate.wrapping_sub(1 as libc::c_uint) as uint64_t,
            );
            return tmp___0;
        }
        (*shard)
            .state
            .hosts_blocklisted = ((*shard).state.hosts_blocklisted).wrapping_add(1);
    };
}
pub unsafe extern "C" fn get_dryrun_socket() -> sock_t {
    let mut sock: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: sock_t = sock_t { sock: 0 };
    tmp = socket(2 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int);
    sock = tmp;
    if sock <= 0 as libc::c_int {
        tmp___0 = __errno_location();
        tmp___1 = strerror(*tmp___0);
        log_fatal(
            b"send\0" as *const u8 as *const libc::c_char,
            b"couldn't create socket. Error: %s\n\0" as *const u8 as *const libc::c_char,
            tmp___1,
        );
    }
    s.sock = sock;
    return s;
}
pub static mut zconf: state_conf = {
    let mut init = state_conf {
        log_level: 6 as libc::c_int,
        target_port: 0 as libc::c_int as port_h_t,
        source_port_first: 32768 as libc::c_int as port_h_t,
        source_port_last: 61000 as libc::c_int as port_h_t,
        max_targets: 4294967295 as libc::c_uint,
        max_runtime: 0 as libc::c_int as uint32_t,
        max_results: 0 as libc::c_int as uint32_t,
        iface: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
        rate: -(1 as libc::c_int),
        bandwidth: 0 as libc::c_int as uint64_t,
        cooldown_secs: 0 as libc::c_int,
        senders: 1 as libc::c_int as uint8_t,
        batch: 1 as libc::c_int as uint8_t,
        pin_cores_len: 0 as libc::c_uint,
        pin_cores: 0 as *const uint32_t as *mut uint32_t,
        seed_provided: 0 as libc::c_int,
        seed: 0 as libc::c_int as uint64_t,
        aes: 0 as *const aesrand_t as *mut aesrand_t,
        generator: 0 as libc::c_uint,
        shard_num: 0 as libc::c_int as libc::c_ushort,
        total_shards: 0 as libc::c_int as libc::c_ushort,
        packet_streams: 1 as libc::c_int,
        probe_module: 0 as *const libc::c_void as *mut libc::c_void as *mut probe_module,
        output_module_name: 0 as *const libc::c_char as *mut libc::c_char,
        output_module: 0 as *const libc::c_void as *mut libc::c_void
            as *mut output_module,
        probe_args: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
        probe_ttl: 255 as libc::c_int as uint8_t,
        output_args: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
        gw_mac: [0 as libc::c_int as macaddr_t, 0, 0, 0, 0, 0],
        hw_mac: [0 as libc::c_int as macaddr_t, 0, 0, 0, 0, 0],
        gw_ip: 0 as libc::c_int as uint32_t,
        gw_mac_set: 0 as libc::c_int,
        hw_mac_set: 0 as libc::c_int,
        source_ip_addresses: [
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
            0 as libc::c_uint,
        ],
        number_source_ips: 0 as libc::c_int as uint32_t,
        send_ip_pkts: 0 as libc::c_int,
        output_filename: 0 as *const libc::c_void as *mut libc::c_void
            as *mut libc::c_char,
        blocklist_filename: 0 as *const libc::c_void as *mut libc::c_void
            as *mut libc::c_char,
        allowlist_filename: 0 as *const libc::c_void as *mut libc::c_void
            as *mut libc::c_char,
        list_of_ips_filename: 0 as *const libc::c_void as *mut libc::c_void
            as *mut libc::c_char,
        list_of_ips_count: 0 as libc::c_int as uint32_t,
        metadata_filename: 0 as *const libc::c_void as *mut libc::c_void
            as *mut libc::c_char,
        metadata_file: 0 as *const libc::c_void as *mut libc::c_void as *mut FILE,
        notes: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
        custom_metadata_str: 0 as *const libc::c_void as *mut libc::c_void
            as *mut libc::c_char,
        destination_cidrs: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        destination_cidrs_len: 0 as libc::c_int,
        raw_output_fields: 0 as *const libc::c_void as *mut libc::c_void
            as *const libc::c_char,
        output_fields: 0 as *const libc::c_void as *mut libc::c_void
            as *mut *const libc::c_char,
        filter: {
            let mut init = output_filter {
                expression: 0 as *const node_t as *mut node_t,
            };
            init
        },
        output_filter_str: 0 as *const libc::c_void as *mut libc::c_void
            as *mut libc::c_char,
        fsconf: {
            let mut init = fieldset_conf {
                defs: {
                    let mut init = fielddef_set {
                        fielddefs: [
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                        ],
                        len: 0 as libc::c_int,
                    };
                    init
                },
                outdefs: {
                    let mut init = fielddef_set {
                        fielddefs: [
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                            {
                                let mut init = field_def {
                                    name: 0 as *const libc::c_char,
                                    type_0: 0 as *const libc::c_char,
                                    desc: 0 as *const libc::c_char,
                                };
                                init
                            },
                        ],
                        len: 0 as libc::c_int,
                    };
                    init
                },
                translation: {
                    let mut init = translation {
                        len: 0 as libc::c_int,
                        translation: [
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                        ],
                    };
                    init
                },
                success_index: 0 as libc::c_int,
                app_success_index: 0 as libc::c_int,
                classification_index: 0 as libc::c_int,
            };
            init
        },
        output_fields_len: 0 as libc::c_int,
        log_file: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
        log_directory: 0 as *const libc::c_void as *mut libc::c_void
            as *mut libc::c_char,
        status_updates_file: 0 as *const libc::c_void as *mut libc::c_void
            as *mut libc::c_char,
        dryrun: 0 as libc::c_int,
        quiet: 0 as libc::c_int,
        ignore_invalid_hosts: 0 as libc::c_int,
        syslog: 1 as libc::c_int,
        recv_ready: 0 as libc::c_int,
        num_retries: 0 as libc::c_int,
        total_allowed: 0 as libc::c_ulong,
        total_disallowed: 0 as libc::c_ulong,
        max_sendto_failures: -(1 as libc::c_int),
        min_hitrate: 0.0f64 as libc::c_float,
        data_link_size: 0 as libc::c_int,
        default_mode: 0 as libc::c_int,
        no_header_row: 0 as libc::c_int,
    };
    init
};
pub unsafe extern "C" fn init_empty_global_configuration(mut c: *mut state_conf) {
    memset(
        ((*c).source_ip_addresses).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[in_addr_t; 256]>() as libc::c_ulong as _,
    );
}
pub static mut zsend: state_send = {
    let mut init = state_send {
        start: 0.0f64,
        finish: 0.0f64,
        packets_sent: 0 as libc::c_int as uint64_t,
        hosts_scanned: 0 as libc::c_int as uint64_t,
        blocklisted: 0 as libc::c_int as uint64_t,
        allowlisted: 0 as libc::c_int as uint64_t,
        warmup: 1 as libc::c_int,
        complete: 0 as libc::c_int,
        first_scanned: 0 as libc::c_uint,
        max_targets: 0 as libc::c_int as uint32_t,
        sendto_failures: 0 as libc::c_int as uint32_t,
        max_index: 0 as libc::c_uint,
        list_of_ips_pbm: 0 as *const libc::c_void as *mut libc::c_void
            as *mut *mut uint8_t,
    };
    init
};
pub static mut zrecv: state_recv = {
    let mut init = state_recv {
        success_total: 0 as libc::c_int as uint32_t,
        success_unique: 0 as libc::c_int as uint32_t,
        app_success_total: 0 as libc::c_int as uint32_t,
        app_success_unique: 0 as libc::c_int as uint32_t,
        cooldown_total: 0 as libc::c_int as uint32_t,
        cooldown_unique: 0 as libc::c_int as uint32_t,
        failure_total: 0 as libc::c_int as uint32_t,
        filter_success: 0 as libc::c_int as uint64_t,
        ip_fragments: 0 as libc::c_int as uint32_t,
        validation_passed: 0 as libc::c_int as uint32_t,
        validation_failed: 0 as libc::c_int as uint32_t,
        complete: 0 as libc::c_int,
        start: 0.0f64,
        finish: 0.0f64,
        pcap_recv: 0 as libc::c_int as uint32_t,
        pcap_drop: 0 as libc::c_int as uint32_t,
        pcap_ifdrop: 0 as libc::c_int as uint32_t,
    };
    init
};
pub unsafe extern "C" fn string_to_ip_address(mut t: *mut libc::c_char) -> in_addr_t {
    let mut r: in_addr_t = 0;
    let mut tmp: in_addr_t = 0;
    tmp = inet_addr(t as *const libc::c_char);
    r = tmp;
    if r == 4294967295 as libc::c_uint {
        log_fatal(
            b"send\0" as *const u8 as *const libc::c_char,
            b"invalid ip address: `%s'\0" as *const u8 as *const libc::c_char,
            t,
        );
    }
    return r;
}
pub unsafe extern "C" fn add_to_array(mut to_add: *mut libc::c_char) {
    if zconf.number_source_ips >= 256 as libc::c_uint {
        log_fatal(
            b"parse\0" as *const u8 as *const libc::c_char,
            b"over 256 source IP addresses provided\0" as *const u8
                as *const libc::c_char,
        );
    }
    log_debug(
        b"SEND\0" as *const u8 as *const libc::c_char,
        b"ipaddress: %s\n\0" as *const u8 as *const libc::c_char,
        to_add,
    );
    zconf
        .source_ip_addresses[zconf.number_source_ips
        as usize] = string_to_ip_address(to_add);
    zconf.number_source_ips = (zconf.number_source_ips).wrapping_add(1);
}
pub unsafe extern "C" fn parse_source_ip_addresses(mut given_string: *mut libc::c_char) {
    let mut dash: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut comma: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: in_addr_t = 0;
    let mut tmp___1: in_addr_t = 0;
    let mut tmp___2: __uint32_t = 0;
    let mut end: in_addr_t = 0;
    let mut tmp___3: in_addr_t = 0;
    let mut tmp___4: __uint32_t = 0;
    let mut temp: in_addr = in_addr { s_addr: 0 };
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = strchr(given_string as *const libc::c_char, '-' as i32);
    dash = tmp;
    tmp___0 = strchr(given_string as *const libc::c_char, ',' as i32);
    comma = tmp___0;
    let mut current_block_38: u64;
    if !dash.is_null() {
        if !comma.is_null() {
            *comma = '\u{0}' as i32 as libc::c_char;
            parse_source_ip_addresses(given_string);
            parse_source_ip_addresses(comma.offset(1 as libc::c_int as isize));
            current_block_38 = 10692455896603418738;
        } else {
            current_block_38 = 5826895839038035959;
        }
    } else {
        current_block_38 = 5826895839038035959;
    }
    match current_block_38 {
        5826895839038035959 => {
            if !comma.is_null() {
                while !comma.is_null() {
                    *comma = '\u{0}' as i32 as libc::c_char;
                    add_to_array(given_string);
                    given_string = comma.offset(1 as libc::c_int as isize);
                    comma = strchr(given_string as *const libc::c_char, ',' as i32);
                    if comma.is_null() {
                        add_to_array(given_string);
                    }
                }
            } else if !dash.is_null() {
                *dash = '\u{0}' as i32 as libc::c_char;
                log_debug(
                    b"SEND\0" as *const u8 as *const libc::c_char,
                    b"address: %s\n\0" as *const u8 as *const libc::c_char,
                    given_string,
                );
                log_debug(
                    b"SEND\0" as *const u8 as *const libc::c_char,
                    b"address: %s\n\0" as *const u8 as *const libc::c_char,
                    dash.offset(1 as libc::c_int as isize),
                );
                tmp___1 = string_to_ip_address(given_string);
                tmp___2 = __bswap_32(tmp___1);
                start = tmp___2;
                tmp___3 = string_to_ip_address(dash.offset(1 as libc::c_int as isize));
                tmp___4 = __bswap_32(tmp___3);
                end = tmp___4.wrapping_add(1 as libc::c_uint);
                while start != end {
                    temp.s_addr = __bswap_32(start);
                    tmp___5 = inet_ntoa(temp);
                    tmp___6 = strdup(tmp___5 as *const libc::c_char);
                    add_to_array(tmp___6);
                    start = start.wrapping_add(1);
                }
            } else {
                add_to_array(given_string);
            }
        }
        _ => {}
    };
}
static mut inited: libc::c_int = 0 as libc::c_int;
static mut aes_sched: [uint32_t; 44] = [0; 44];
pub unsafe extern "C" fn validate_init() {
    let mut key: [uint8_t; 16] = [0; 16];
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = random_bytes(
        key.as_mut_ptr() as *mut libc::c_void,
        16 as libc::c_int as size_t,
    );
    if tmp == 0 {
        log_fatal(
            b"validate\0" as *const u8 as *const libc::c_char,
            b"couldn't get random bytes\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___0 = rijndaelKeySetupEnc(
        aes_sched.as_mut_ptr(),
        key.as_mut_ptr() as *const u8_0,
        128 as libc::c_int,
    );
    if tmp___0 != 10 as libc::c_int {
        log_fatal(
            b"validate\0" as *const u8 as *const libc::c_char,
            b"couldn't initialize AES key\0" as *const u8 as *const libc::c_char,
        );
    }
    inited = 1 as libc::c_int;
}
pub unsafe extern "C" fn validate_gen(
    src: uint32_t,
    dst: uint32_t,
    mut output: *mut uint8_t,
) {
    validate_gen_ex(
        src,
        dst,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        output,
    );
}
pub unsafe extern "C" fn validate_gen_ex(
    input0: uint32_t,
    input1: uint32_t,
    input2: uint32_t,
    input3: uint32_t,
    mut output: *mut uint8_t,
) {
    let mut aes_input: [uint32_t; 4] = [0; 4];
    if inited == 0 {
        __assert_fail(
            b"inited\0" as *const u8 as *const libc::c_char,
            b"src/validate.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_uint,
            b"validate_gen_ex\0" as *const u8 as *const libc::c_char,
        );
    }
    aes_input[0 as libc::c_int as usize] = input0;
    aes_input[1 as libc::c_int as usize] = input1;
    aes_input[2 as libc::c_int as usize] = input2;
    aes_input[3 as libc::c_int as usize] = input3;
    rijndaelEncrypt(
        aes_sched.as_mut_ptr() as *const u32_0,
        10 as libc::c_int,
        aes_input.as_mut_ptr() as *mut uint8_t as *const u8_0,
        output,
    );
}
pub static mut recv_ready_mutex: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
pub static mut default_help_text: *const libc::c_char = b"By default, ZMap prints out unique, successful IP addresses (e.g., SYN-ACK from a TCP SYN scan) in ASCII form (e.g., 192.168.1.5) to stdout or the specified output file. Internally this is handled by the \"csv\" output module and is equivalent to running zmap --output-module=csv --output-fields=saddr --output-filter=\"success = 1 && repeat = 0\" --no-header-row.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn start_send(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut s: *mut send_arg_t = 0 as *mut send_arg_t;
    s = arg as *mut send_arg_t;
    log_debug(
        b"zmap\0" as *const u8 as *const libc::c_char,
        b"Pinning a send thread to core %u\0" as *const u8 as *const libc::c_char,
        (*s).cpu,
    );
    set_cpu((*s).cpu);
    free(s as *mut libc::c_void);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn start_recv(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut r: *mut recv_arg_t = 0 as *mut recv_arg_t;
    r = arg as *mut recv_arg_t;
    log_debug(
        b"zmap\0" as *const u8 as *const libc::c_char,
        b"Pinning receive thread to core %u\0" as *const u8 as *const libc::c_char,
        (*r).cpu,
    );
    set_cpu((*r).cpu);
    recv_run(&mut recv_ready_mutex);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn start_mon(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut mon_arg: *mut mon_start_arg_t = 0 as *mut mon_start_arg_t;
    mon_arg = arg as *mut mon_start_arg_t;
    log_debug(
        b"zmap\0" as *const u8 as *const libc::c_char,
        b"Pinning monitor thread to core %u\0" as *const u8 as *const libc::c_char,
        (*mon_arg).cpu,
    );
    set_cpu((*mon_arg).cpu);
    monitor_run((*mon_arg).it, (*mon_arg).recv_ready_mutex);
    free(mon_arg as *mut libc::c_void);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn start_zmap() {
    let mut default_ip: in_addr = in_addr { s_addr: 0 };
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut gw_ip: in_addr = in_addr { s_addr: 0 };
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut it: *mut iterator_t = 0 as *mut iterator_t;
    let mut tmp___8: *mut iterator_t = 0 as *mut iterator_t;
    let mut cpu: uint32_t = 0;
    let mut tsend: *mut pthread_t = 0 as *mut pthread_t;
    let mut trecv: pthread_t = 0;
    let mut tmon: pthread_t = 0;
    let mut r: libc::c_int = 0;
    let mut recv_arg: *mut recv_arg_t = 0 as *mut recv_arg_t;
    let mut tmp___9: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___10: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: uint8_t = 0;
    let mut sock: sock_t = sock_t { sock: 0 };
    let mut arg: *mut send_arg_t = 0 as *mut send_arg_t;
    let mut tmp___11: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut r___0: libc::c_int = 0;
    let mut tmp___12: libc::c_int = 0;
    let mut mon_arg: *mut mon_start_arg_t = 0 as *mut mon_start_arg_t;
    let mut tmp___13: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut r___1: libc::c_int = 0;
    let mut tmp___14: libc::c_int = 0;
    let mut i___0: uint8_t = 0;
    let mut r___2: libc::c_int = 0;
    let mut tmp___15: libc::c_int = 0;
    if zconf.iface as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        zconf.iface = get_default_iface();
        if (zconf.iface).is_null() {
            __assert_fail(
                b"zconf.iface\0" as *const u8 as *const libc::c_char,
                b"src/zmap.c\0" as *const u8 as *const libc::c_char,
                119 as libc::c_uint,
                b"start_zmap\0" as *const u8 as *const libc::c_char,
            );
        }
        log_debug(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"no interface provided. will use default interface (%s).\0" as *const u8
                as *const libc::c_char,
            zconf.iface,
        );
    }
    if zconf.number_source_ips == 0 as libc::c_uint {
        tmp___0 = get_iface_ip(zconf.iface, &mut default_ip);
        if tmp___0 < 0 as libc::c_int {
            log_fatal(
                b"zmap\0" as *const u8 as *const libc::c_char,
                b"could not detect default IP address for %s. Try specifying a source address (-S).\0"
                    as *const u8 as *const libc::c_char,
                zconf.iface,
            );
        }
        zconf.source_ip_addresses[0 as libc::c_int as usize] = default_ip.s_addr;
        zconf.number_source_ips = (zconf.number_source_ips).wrapping_add(1);
        tmp___1 = inet_ntoa(default_ip);
        log_debug(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"no source IP address given. will use default address: %s.\0" as *const u8
                as *const libc::c_char,
            tmp___1,
        );
    }
    if zconf.gw_mac_set == 0 {
        memset(
            &mut gw_ip as *mut in_addr as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<in_addr>() as libc::c_ulong as _,
        );
        tmp___2 = get_default_gw(&mut gw_ip, zconf.iface);
        if tmp___2 < 0 as libc::c_int {
            log_fatal(
                b"zmap\0" as *const u8 as *const libc::c_char,
                b"could not detect default gateway address for %s. Try setting default gateway mac address (-G). If this is a newly launched machine, try completing an outgoing network connection (e.g. curl https://zmap.io), and trying again.\0"
                    as *const u8 as *const libc::c_char,
                zconf.iface,
            );
        }
        tmp___3 = inet_ntoa(gw_ip);
        log_debug(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"found gateway IP %s on %s\0" as *const u8 as *const libc::c_char,
            tmp___3,
            zconf.iface,
        );
        zconf.gw_ip = gw_ip.s_addr;
        memset(
            &mut zconf.gw_mac as *mut [macaddr_t; 6] as *mut libc::c_void,
            0 as libc::c_int,
            6 as libc::c_int as size_t as _,
        );
        tmp___5 = get_hw_addr(&mut gw_ip, zconf.iface, (zconf.gw_mac).as_mut_ptr());
        if tmp___5 != 0 {
            tmp___4 = inet_ntoa(gw_ip);
            log_fatal(
                b"zmap\0" as *const u8 as *const libc::c_char,
                b"could not detect GW MAC address for %s on %s. Try setting default gateway mac address (-G), or run \"arp <gateway_ip>\" in terminal. If this is a newly launched machine, try completing an outgoing network connection (e.g. curl https://zmap.io), and trying again.\0"
                    as *const u8 as *const libc::c_char,
                tmp___4,
                zconf.iface,
            );
        }
        zconf.gw_mac_set = 1 as libc::c_int;
    }
    log_debug(
        b"send\0" as *const u8 as *const libc::c_char,
        b"gateway MAC address %02x:%02x:%02x:%02x:%02x:%02x\0" as *const u8
            as *const libc::c_char,
        zconf.gw_mac[0 as libc::c_int as usize] as libc::c_int,
        zconf.gw_mac[1 as libc::c_int as usize] as libc::c_int,
        zconf.gw_mac[2 as libc::c_int as usize] as libc::c_int,
        zconf.gw_mac[3 as libc::c_int as usize] as libc::c_int,
        zconf.gw_mac[4 as libc::c_int as usize] as libc::c_int,
        zconf.gw_mac[5 as libc::c_int as usize] as libc::c_int,
    );
    if !(zconf.output_module).is_null() {
        if (b"no output module set\0" as *const u8 as *const libc::c_char).is_null() {
            __assert_fail(
                b"zconf.output_module && \"no output module set\"\0" as *const u8
                    as *const libc::c_char,
                b"src/zmap.c\0" as *const u8 as *const libc::c_char,
                170 as libc::c_uint,
                b"start_zmap\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        __assert_fail(
            b"zconf.output_module && \"no output module set\"\0" as *const u8
                as *const libc::c_char,
            b"src/zmap.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_uint,
            b"start_zmap\0" as *const u8 as *const libc::c_char,
        );
    }
    log_debug(
        b"zmap\0" as *const u8 as *const libc::c_char,
        b"output module: %s\0" as *const u8 as *const libc::c_char,
        (*zconf.output_module).name,
    );
    if !(zconf.output_module).is_null() {
        if ((*zconf.output_module).init).is_some() {
            tmp___7 = (Some(
                ((*zconf.output_module).init).expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(&mut zconf, zconf.output_fields, zconf.output_fields_len);
            if tmp___7 != 0 {
                log_fatal(
                    b"zmap\0" as *const u8 as *const libc::c_char,
                    b"output module did not initialize successfully.\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
    }
    tmp___8 = send_init();
    it = tmp___8;
    if it.is_null() {
        log_fatal(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"unable to initialize sending component\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(zconf.output_module).is_null() {
        if ((*zconf.output_module).start).is_some() {
            (Some(((*zconf.output_module).start).expect("non-null function pointer")))
                .expect("non-null function pointer")(&mut zconf, &mut zsend, &mut zrecv);
        }
    }
    cpu = 0 as libc::c_int as uint32_t;
    if zconf.dryrun == 0 {
        tmp___9 = xmalloc(::std::mem::size_of::<recv_arg_t>() as libc::c_ulong);
        recv_arg = tmp___9 as *mut recv_arg_t;
        (*recv_arg)
            .cpu = *(zconf.pin_cores)
            .offset(cpu.wrapping_rem(zconf.pin_cores_len) as isize);
        cpu = cpu.wrapping_add(1);
        r = pthread_create(
            &mut trecv as *mut pthread_t,
            0 as *mut libc::c_void as *const pthread_attr_t,
            Some(
                start_recv
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            recv_arg as *mut libc::c_void,
        );
        if r != 0 as libc::c_int {
            log_fatal(
                b"zmap\0" as *const u8 as *const libc::c_char,
                b"unable to create recv thread\0" as *const u8 as *const libc::c_char,
            );
        }
        loop {
            pthread_mutex_lock(&mut recv_ready_mutex);
            if zconf.recv_ready != 0 {
                pthread_mutex_unlock(&mut recv_ready_mutex);
                break;
            } else {
                pthread_mutex_unlock(&mut recv_ready_mutex);
            }
        }
    }
    tmp___10 = xmalloc(
        (zconf.senders as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<pthread_t>() as libc::c_ulong),
    );
    tsend = tmp___10 as *mut pthread_t;
    i = 0 as libc::c_int as uint8_t;
    while (i as libc::c_int) < zconf.senders as libc::c_int {
        if zconf.dryrun != 0 {
            sock = get_dryrun_socket();
        } else {
            sock = get_socket(i as uint32_t);
        }
        tmp___11 = xmalloc(::std::mem::size_of::<send_arg_t>() as libc::c_ulong);
        arg = tmp___11 as *mut send_arg_t;
        (*arg).sock = sock;
        (*arg).shard = get_shard(it, i);
        (*arg)
            .cpu = *(zconf.pin_cores)
            .offset(cpu.wrapping_rem(zconf.pin_cores_len) as isize);
        cpu = cpu.wrapping_add(1);
        tmp___12 = pthread_create(
            tsend.offset(i as libc::c_int as isize),
            0 as *mut libc::c_void as *const pthread_attr_t,
            Some(
                start_send
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            arg as *mut libc::c_void,
        );
        r___0 = tmp___12;
        if r___0 != 0 as libc::c_int {
            log_fatal(
                b"zmap\0" as *const u8 as *const libc::c_char,
                b"unable to create send thread\0" as *const u8 as *const libc::c_char,
            );
        }
        i = (i as libc::c_int + 1 as libc::c_int) as uint8_t;
    }
    log_debug(
        b"zmap\0" as *const u8 as *const libc::c_char,
        b"%d sender threads spawned\0" as *const u8 as *const libc::c_char,
        zconf.senders as libc::c_int,
    );
    if zconf.dryrun == 0 {
        monitor_init();
        tmp___13 = xmalloc(::std::mem::size_of::<mon_start_arg_t>() as libc::c_ulong);
        mon_arg = tmp___13 as *mut mon_start_arg_t;
        (*mon_arg).it = it;
        (*mon_arg).recv_ready_mutex = &mut recv_ready_mutex;
        (*mon_arg)
            .cpu = *(zconf.pin_cores)
            .offset(cpu.wrapping_rem(zconf.pin_cores_len) as isize);
        tmp___14 = pthread_create(
            &mut tmon as *mut pthread_t,
            0 as *mut libc::c_void as *const pthread_attr_t,
            Some(
                start_mon as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            mon_arg as *mut libc::c_void,
        );
        r___1 = tmp___14;
        if r___1 != 0 as libc::c_int {
            log_fatal(
                b"zmap\0" as *const u8 as *const libc::c_char,
                b"unable to create monitor thread\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    drop_privs();
    i___0 = 0 as libc::c_int as uint8_t;
    while (i___0 as libc::c_int) < zconf.senders as libc::c_int {
        tmp___15 = pthread_join(
            *tsend.offset(i___0 as libc::c_int as isize),
            0 as *mut libc::c_void as *mut *mut libc::c_void,
        );
        r___2 = tmp___15;
        if r___2 != 0 as libc::c_int {
            log_fatal(
                b"zmap\0" as *const u8 as *const libc::c_char,
                b"unable to join send thread\0" as *const u8 as *const libc::c_char,
            );
        }
        i___0 = (i___0 as libc::c_int + 1 as libc::c_int) as uint8_t;
    }
    log_debug(
        b"zmap\0" as *const u8 as *const libc::c_char,
        b"senders finished\0" as *const u8 as *const libc::c_char,
    );
    if zconf.dryrun == 0 {
        r = pthread_join(trecv, 0 as *mut libc::c_void as *mut *mut libc::c_void);
        if r != 0 as libc::c_int {
            log_fatal(
                b"zmap\0" as *const u8 as *const libc::c_char,
                b"unable to join recv thread\0" as *const u8 as *const libc::c_char,
            );
        }
        let mut current_block_135: u64;
        if zconf.quiet == 0 {
            current_block_135 = 3156627316214790248;
        } else if !(zconf.status_updates_file).is_null() {
            current_block_135 = 3156627316214790248;
        } else {
            current_block_135 = 10109057886293123569;
        }
        match current_block_135 {
            3156627316214790248 => {
                pthread_join(tmon, 0 as *mut libc::c_void as *mut *mut libc::c_void);
                if r != 0 as libc::c_int {
                    log_fatal(
                        b"zmap\0" as *const u8 as *const libc::c_char,
                        b"unable to join monitor thread\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
            _ => {}
        }
    }
    if !(zconf.metadata_filename).is_null() {
    }
    if !(zconf.output_module).is_null() {
        if ((*zconf.output_module).close).is_some() {
            (Some(((*zconf.output_module).close).expect("non-null function pointer")))
                .expect("non-null function pointer")(&mut zconf, &mut zsend, &mut zrecv);
        }
    }
    if !(zconf.probe_module).is_null() {
        if ((*zconf.probe_module).close).is_some() {
            (Some(((*zconf.probe_module).close).expect("non-null function pointer")))
                .expect("non-null function pointer")(&mut zconf, &mut zsend, &mut zrecv);
        }
    }
    log_info(
        b"zmap\0" as *const u8 as *const libc::c_char,
        b"completed\0" as *const u8 as *const libc::c_char,
    );
}
pub static mut gengetopt_args_info_purpose: *const libc::c_char = b"A fast Internet-wide scanner.\0"
    as *const u8 as *const libc::c_char;
pub static mut gengetopt_args_info_usage: *const libc::c_char = b"Usage: zmap [OPTION]... [SUBNETS]...\0"
    as *const u8 as *const libc::c_char;
pub static mut gengetopt_args_info_versiontext: *const libc::c_char = b"\0" as *const u8
    as *const libc::c_char;
pub static mut gengetopt_args_info_description: *const libc::c_char = b"\0" as *const u8
    as *const libc::c_char;
pub static mut gengetopt_args_info_help: [*const libc::c_char; 62] = [
    b"Basic Arguments:\0" as *const u8 as *const libc::c_char,
    b"  -p, --target-port=port        port number to scan (for TCP and UDP scans)\0"
        as *const u8 as *const libc::c_char,
    b"  -o, --output-file=name        Output file\0" as *const u8 as *const libc::c_char,
    b"  -b, --blocklist-file=path     File of subnets to exclude, in CIDR notation,\n                                  e.g. 192.168.0.0/16\0"
        as *const u8 as *const libc::c_char,
    b"  -w, --allowlist-file=path     File of subnets to constrain scan to, in CIDR\n                                  notation, e.g. 192.168.0.0/16\0"
        as *const u8 as *const libc::c_char,
    b"  -I, --list-of-ips-file=path   List of individual addresses to scan in random\n                                  order. Use --white-list file unless >1\n                                  million IPs\0"
        as *const u8 as *const libc::c_char,
    b"\nScan Options:\0" as *const u8 as *const libc::c_char,
    b"  -r, --rate=pps                Set send rate in packets/sec\0" as *const u8
        as *const libc::c_char,
    b"  -B, --bandwidth=bps           Set send rate in bits/second (supports suffixes\n                                  G, M and K)\0"
        as *const u8 as *const libc::c_char,
    b"      --batch=pps               Set the number of packets to send per iteration\0"
        as *const u8 as *const libc::c_char,
    b"  -n, --max-targets=n           Cap number of targets to probe (as a number or\n                                  a percentage of the address space)\0"
        as *const u8 as *const libc::c_char,
    b"  -t, --max-runtime=secs        Cap length of time for sending packets\0"
        as *const u8 as *const libc::c_char,
    b"  -N, --max-results=n           Cap number of results to return\0" as *const u8
        as *const libc::c_char,
    b"  -P, --probes=n                Number of probes to send to each IP\n                                  (default=`1')\0"
        as *const u8 as *const libc::c_char,
    b"  -c, --cooldown-time=secs      How long to continue receiving after sending\n                                  last probe  (default=`8')\0"
        as *const u8 as *const libc::c_char,
    b"  -e, --seed=n                  Seed used to select address permutation\0"
        as *const u8 as *const libc::c_char,
    b"      --retries=n               Max number of times to try to send packet if\n                                  send fails  (default=`10')\0"
        as *const u8 as *const libc::c_char,
    b"  -d, --dryrun                  Don't actually send packets\0" as *const u8
        as *const libc::c_char,
    b"\nScan Sharding:\0" as *const u8 as *const libc::c_char,
    b"      --shards=N                Set the total number of shards  (default=`1')\0"
        as *const u8 as *const libc::c_char,
    b"      --shard=n                 Set which shard this scan is (0 indexed)\n                                  (default=`0')\0"
        as *const u8 as *const libc::c_char,
    b"\nNetwork Options:\0" as *const u8 as *const libc::c_char,
    b"  -s, --source-port=port|range  Source port(s) for scan packets\0" as *const u8
        as *const libc::c_char,
    b"  -S, --source-ip=ip|range      Source address(es) for scan packets\0" as *const u8
        as *const libc::c_char,
    b"  -G, --gateway-mac=addr        Specify gateway MAC address\0" as *const u8
        as *const libc::c_char,
    b"      --source-mac=addr         Source MAC address\0" as *const u8
        as *const libc::c_char,
    b"  -i, --interface=name          Specify network interface to use\0" as *const u8
        as *const libc::c_char,
    b"  -X, --iplayer                 Sends IP packets instead of Ethernet (for VPNs)\0"
        as *const u8 as *const libc::c_char,
    b"\nProbe Modules:\0" as *const u8 as *const libc::c_char,
    b"  -M, --probe-module=name       Select probe module  (default=`tcp_synscan')\0"
        as *const u8 as *const libc::c_char,
    b"      --probe-args=args         Arguments to pass to probe module\0" as *const u8
        as *const libc::c_char,
    b"      --probe-ttl=n             Set TTL value for probe IP packets\n                                  (default=`255')\0"
        as *const u8 as *const libc::c_char,
    b"      --list-probe-modules      List available probe modules\0" as *const u8
        as *const libc::c_char,
    b"\nResults Output:\0" as *const u8 as *const libc::c_char,
    b"  -f, --output-fields=fields    Fields that should be output in result set\0"
        as *const u8 as *const libc::c_char,
    b"  -O, --output-module=name      Select output module\0" as *const u8
        as *const libc::c_char,
    b"      --output-args=args        Arguments to pass to output module\0" as *const u8
        as *const libc::c_char,
    b"      --output-filter=filter    Specify a filter over the response fields to\n                                  limit what responses get sent to the output\n                                  module\0"
        as *const u8 as *const libc::c_char,
    b"      --list-output-modules     List available output modules\0" as *const u8
        as *const libc::c_char,
    b"      --list-output-fields      List all fields that can be output by selected\n                                  probe module\0"
        as *const u8 as *const libc::c_char,
    b"      --no-header-row           Precludes outputting any header rows in data\n                                  (e.g., CSV headers)\0"
        as *const u8 as *const libc::c_char,
    b"\nLogging and Metadata:\0" as *const u8 as *const libc::c_char,
    b"  -v, --verbosity=n             Level of log detail (0-5)  (default=`3')\0"
        as *const u8 as *const libc::c_char,
    b"  -l, --log-file=name           Write log entries to file\0" as *const u8
        as *const libc::c_char,
    b"  -L, --log-directory=directory Write log entries to a timestamped file in this\n                                  directory\0"
        as *const u8 as *const libc::c_char,
    b"  -m, --metadata-file=name      Output file for scan metadata (JSON)\0"
        as *const u8 as *const libc::c_char,
    b"  -u, --status-updates-file=name\n                                Write scan progress updates to CSV file\0"
        as *const u8 as *const libc::c_char,
    b"  -q, --quiet                   Do not print status updates\0" as *const u8
        as *const libc::c_char,
    b"      --disable-syslog          Disables logging messages to syslog\0" as *const u8
        as *const libc::c_char,
    b"      --notes=notes             Inject user-specified notes into scan metadata\0"
        as *const u8 as *const libc::c_char,
    b"      --user-metadata=json      Inject user-specified JSON metadata into scan\n                                  metadata\0"
        as *const u8 as *const libc::c_char,
    b"\nAdditional Options:\0" as *const u8 as *const libc::c_char,
    b"  -C, --config=filename         Read a configuration file, which can specify\n                                  any of these options\n                                  (default=`/etc/zmap/zmap.conf')\0"
        as *const u8 as *const libc::c_char,
    b"      --max-sendto-failures=n   Maximum NIC sendto failures before scan is\n                                  aborted  (default=`-1')\0"
        as *const u8 as *const libc::c_char,
    b"      --min-hitrate=n           Minimum hitrate that scan can hit before scan\n                                  is aborted  (default=`0.0')\0"
        as *const u8 as *const libc::c_char,
    b"  -T, --sender-threads=n        Threads used to send packets  (default=`1')\0"
        as *const u8 as *const libc::c_char,
    b"      --cores=STRING            Comma-separated list of cores to pin to\0"
        as *const u8 as *const libc::c_char,
    b"      --ignore-blocklist-errors Ignore invalid entries in allowlist/blocklist\n                                  file.\0"
        as *const u8 as *const libc::c_char,
    b"  -h, --help                    Print help and exit\0" as *const u8
        as *const libc::c_char,
    b"  -V, --version                 Print version and exit\0" as *const u8
        as *const libc::c_char,
    b"\nExamples:\n    zmap -p 80 (scan full IPv4 address space for hosts on TCP/80)\n    zmap -N 5 -B 10M -p 80 (find 5 HTTP servers, scanning at 10 Mb/s)\n    zmap -p 80 10.0.0.0/8 192.168.0.0/16 (scan both subnets on TCP/80)\n    zmap -p 80 1.2.3.4 10.0.0.3 (scan 1.2.3.4, 10.0.0.3 on TCP/80)\0"
        as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut cmd_line_list: *mut line_list = 0 as *const line_list as *mut line_list;
static mut cmd_line_list_tmp: *mut line_list = 0 as *const line_list as *mut line_list;
unsafe extern "C" fn free_cmd_list() {
    if !cmd_line_list.is_null() {
        while !cmd_line_list.is_null() {
            cmd_line_list_tmp = cmd_line_list;
            cmd_line_list = (*cmd_line_list).next;
            free((*cmd_line_list_tmp).string_arg as *mut libc::c_void);
            free(cmd_line_list_tmp as *mut libc::c_void);
        }
    }
}
unsafe extern "C" fn clear_given(mut args_info: *mut gengetopt_args_info) {
    (*args_info).target_port_given = 0 as libc::c_uint;
    (*args_info).output_file_given = 0 as libc::c_uint;
    (*args_info).blocklist_file_given = 0 as libc::c_uint;
    (*args_info).allowlist_file_given = 0 as libc::c_uint;
    (*args_info).list_of_ips_file_given = 0 as libc::c_uint;
    (*args_info).rate_given = 0 as libc::c_uint;
    (*args_info).bandwidth_given = 0 as libc::c_uint;
    (*args_info).batch_given = 0 as libc::c_uint;
    (*args_info).max_targets_given = 0 as libc::c_uint;
    (*args_info).max_runtime_given = 0 as libc::c_uint;
    (*args_info).max_results_given = 0 as libc::c_uint;
    (*args_info).probes_given = 0 as libc::c_uint;
    (*args_info).cooldown_time_given = 0 as libc::c_uint;
    (*args_info).seed_given = 0 as libc::c_uint;
    (*args_info).retries_given = 0 as libc::c_uint;
    (*args_info).dryrun_given = 0 as libc::c_uint;
    (*args_info).shards_given = 0 as libc::c_uint;
    (*args_info).shard_given = 0 as libc::c_uint;
    (*args_info).source_port_given = 0 as libc::c_uint;
    (*args_info).source_ip_given = 0 as libc::c_uint;
    (*args_info).gateway_mac_given = 0 as libc::c_uint;
    (*args_info).source_mac_given = 0 as libc::c_uint;
    (*args_info).interface_given = 0 as libc::c_uint;
    (*args_info).iplayer_given = 0 as libc::c_uint;
    (*args_info).probe_module_given = 0 as libc::c_uint;
    (*args_info).probe_args_given = 0 as libc::c_uint;
    (*args_info).probe_ttl_given = 0 as libc::c_uint;
    (*args_info).list_probe_modules_given = 0 as libc::c_uint;
    (*args_info).output_fields_given = 0 as libc::c_uint;
    (*args_info).output_module_given = 0 as libc::c_uint;
    (*args_info).output_args_given = 0 as libc::c_uint;
    (*args_info).output_filter_given = 0 as libc::c_uint;
    (*args_info).list_output_modules_given = 0 as libc::c_uint;
    (*args_info).list_output_fields_given = 0 as libc::c_uint;
    (*args_info).no_header_row_given = 0 as libc::c_uint;
    (*args_info).verbosity_given = 0 as libc::c_uint;
    (*args_info).log_file_given = 0 as libc::c_uint;
    (*args_info).log_directory_given = 0 as libc::c_uint;
    (*args_info).metadata_file_given = 0 as libc::c_uint;
    (*args_info).status_updates_file_given = 0 as libc::c_uint;
    (*args_info).quiet_given = 0 as libc::c_uint;
    (*args_info).disable_syslog_given = 0 as libc::c_uint;
    (*args_info).notes_given = 0 as libc::c_uint;
    (*args_info).user_metadata_given = 0 as libc::c_uint;
    (*args_info).config_given = 0 as libc::c_uint;
    (*args_info).max_sendto_failures_given = 0 as libc::c_uint;
    (*args_info).min_hitrate_given = 0 as libc::c_uint;
    (*args_info).sender_threads_given = 0 as libc::c_uint;
    (*args_info).cores_given = 0 as libc::c_uint;
    (*args_info).ignore_blocklist_errors_given = 0 as libc::c_uint;
    (*args_info).help_given = 0 as libc::c_uint;
    (*args_info).version_given = 0 as libc::c_uint;
}
unsafe extern "C" fn clear_args(mut args_info: *mut gengetopt_args_info) {
    (*args_info).target_port_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).output_file_arg = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).output_file_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).blocklist_file_arg = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).blocklist_file_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).allowlist_file_arg = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).allowlist_file_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).list_of_ips_file_arg = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).list_of_ips_file_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).rate_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).bandwidth_arg = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).bandwidth_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).batch_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).max_targets_arg = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).max_targets_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).max_runtime_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).max_results_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).probes_arg = 1 as libc::c_int;
    (*args_info).probes_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).cooldown_time_arg = 8 as libc::c_int;
    (*args_info).cooldown_time_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).seed_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).retries_arg = 10 as libc::c_int;
    (*args_info).retries_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).shards_arg = 1 as libc::c_int;
    (*args_info).shards_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).shard_arg = 0 as libc::c_int;
    (*args_info).shard_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).source_port_arg = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).source_port_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).source_ip_arg = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).source_ip_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).gateway_mac_arg = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).gateway_mac_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).source_mac_arg = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).source_mac_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).interface_arg = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).interface_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info)
        .probe_module_arg = gengetopt_strdup(
        b"tcp_synscan\0" as *const u8 as *const libc::c_char,
    );
    (*args_info).probe_module_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).probe_args_arg = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).probe_args_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).probe_ttl_arg = 255 as libc::c_int;
    (*args_info).probe_ttl_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).output_fields_arg = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).output_fields_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).output_module_arg = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).output_module_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).output_args_arg = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).output_args_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).output_filter_arg = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).output_filter_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).verbosity_arg = 3 as libc::c_int;
    (*args_info).verbosity_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).log_file_arg = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).log_file_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).log_directory_arg = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).log_directory_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).metadata_file_arg = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).metadata_file_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).status_updates_file_arg = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).status_updates_file_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).notes_arg = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).notes_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).user_metadata_arg = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).user_metadata_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info)
        .config_arg = gengetopt_strdup(
        b"/etc/zmap/zmap.conf\0" as *const u8 as *const libc::c_char,
    );
    (*args_info).config_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).max_sendto_failures_arg = -(1 as libc::c_int);
    (*args_info).max_sendto_failures_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).min_hitrate_arg = 0.0f64 as libc::c_float;
    (*args_info).min_hitrate_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).sender_threads_arg = 1 as libc::c_int;
    (*args_info).sender_threads_orig = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).cores_arg = 0 as *mut libc::c_void as *mut libc::c_char;
    (*args_info).cores_orig = 0 as *mut libc::c_void as *mut libc::c_char;
}
unsafe extern "C" fn init_args_info(mut args_info: *mut gengetopt_args_info) {
    (*args_info).target_port_help = gengetopt_args_info_help[1 as libc::c_int as usize];
    (*args_info).output_file_help = gengetopt_args_info_help[2 as libc::c_int as usize];
    (*args_info)
        .blocklist_file_help = gengetopt_args_info_help[3 as libc::c_int as usize];
    (*args_info)
        .allowlist_file_help = gengetopt_args_info_help[4 as libc::c_int as usize];
    (*args_info)
        .list_of_ips_file_help = gengetopt_args_info_help[5 as libc::c_int as usize];
    (*args_info).rate_help = gengetopt_args_info_help[7 as libc::c_int as usize];
    (*args_info).bandwidth_help = gengetopt_args_info_help[8 as libc::c_int as usize];
    (*args_info).batch_help = gengetopt_args_info_help[9 as libc::c_int as usize];
    (*args_info).max_targets_help = gengetopt_args_info_help[10 as libc::c_int as usize];
    (*args_info).max_runtime_help = gengetopt_args_info_help[11 as libc::c_int as usize];
    (*args_info).max_results_help = gengetopt_args_info_help[12 as libc::c_int as usize];
    (*args_info).probes_help = gengetopt_args_info_help[13 as libc::c_int as usize];
    (*args_info)
        .cooldown_time_help = gengetopt_args_info_help[14 as libc::c_int as usize];
    (*args_info).seed_help = gengetopt_args_info_help[15 as libc::c_int as usize];
    (*args_info).retries_help = gengetopt_args_info_help[16 as libc::c_int as usize];
    (*args_info).dryrun_help = gengetopt_args_info_help[17 as libc::c_int as usize];
    (*args_info).shards_help = gengetopt_args_info_help[19 as libc::c_int as usize];
    (*args_info).shard_help = gengetopt_args_info_help[20 as libc::c_int as usize];
    (*args_info).source_port_help = gengetopt_args_info_help[22 as libc::c_int as usize];
    (*args_info).source_ip_help = gengetopt_args_info_help[23 as libc::c_int as usize];
    (*args_info).gateway_mac_help = gengetopt_args_info_help[24 as libc::c_int as usize];
    (*args_info).source_mac_help = gengetopt_args_info_help[25 as libc::c_int as usize];
    (*args_info).interface_help = gengetopt_args_info_help[26 as libc::c_int as usize];
    (*args_info).iplayer_help = gengetopt_args_info_help[27 as libc::c_int as usize];
    (*args_info)
        .probe_module_help = gengetopt_args_info_help[29 as libc::c_int as usize];
    (*args_info).probe_args_help = gengetopt_args_info_help[30 as libc::c_int as usize];
    (*args_info).probe_ttl_help = gengetopt_args_info_help[31 as libc::c_int as usize];
    (*args_info)
        .list_probe_modules_help = gengetopt_args_info_help[32 as libc::c_int as usize];
    (*args_info)
        .output_fields_help = gengetopt_args_info_help[34 as libc::c_int as usize];
    (*args_info)
        .output_module_help = gengetopt_args_info_help[35 as libc::c_int as usize];
    (*args_info).output_args_help = gengetopt_args_info_help[36 as libc::c_int as usize];
    (*args_info)
        .output_filter_help = gengetopt_args_info_help[37 as libc::c_int as usize];
    (*args_info)
        .list_output_modules_help = gengetopt_args_info_help[38 as libc::c_int as usize];
    (*args_info)
        .list_output_fields_help = gengetopt_args_info_help[39 as libc::c_int as usize];
    (*args_info)
        .no_header_row_help = gengetopt_args_info_help[40 as libc::c_int as usize];
    (*args_info).verbosity_help = gengetopt_args_info_help[42 as libc::c_int as usize];
    (*args_info).log_file_help = gengetopt_args_info_help[43 as libc::c_int as usize];
    (*args_info)
        .log_directory_help = gengetopt_args_info_help[44 as libc::c_int as usize];
    (*args_info)
        .metadata_file_help = gengetopt_args_info_help[45 as libc::c_int as usize];
    (*args_info)
        .status_updates_file_help = gengetopt_args_info_help[46 as libc::c_int as usize];
    (*args_info).quiet_help = gengetopt_args_info_help[47 as libc::c_int as usize];
    (*args_info)
        .disable_syslog_help = gengetopt_args_info_help[48 as libc::c_int as usize];
    (*args_info).notes_help = gengetopt_args_info_help[49 as libc::c_int as usize];
    (*args_info)
        .user_metadata_help = gengetopt_args_info_help[50 as libc::c_int as usize];
    (*args_info).config_help = gengetopt_args_info_help[52 as libc::c_int as usize];
    (*args_info)
        .max_sendto_failures_help = gengetopt_args_info_help[53 as libc::c_int as usize];
    (*args_info).min_hitrate_help = gengetopt_args_info_help[54 as libc::c_int as usize];
    (*args_info)
        .sender_threads_help = gengetopt_args_info_help[55 as libc::c_int as usize];
    (*args_info).cores_help = gengetopt_args_info_help[56 as libc::c_int as usize];
    (*args_info)
        .ignore_blocklist_errors_help = gengetopt_args_info_help[57 as libc::c_int
        as usize];
    (*args_info).help_help = gengetopt_args_info_help[58 as libc::c_int as usize];
    (*args_info).version_help = gengetopt_args_info_help[59 as libc::c_int as usize];
}
pub unsafe extern "C" fn cmdline_parser_print_version() {
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: size_t = 0;
    tmp___1 = strlen(b"zmap\0" as *const u8 as *const libc::c_char);
    if tmp___1 != 0 {
        tmp___0 = b"zmap\0" as *const u8 as *const libc::c_char;
    } else {
        tmp___0 = b"zmap\0" as *const u8 as *const libc::c_char;
    }
    __printf_chk(
        1 as libc::c_int,
        b"%s %s\n\0" as *const u8 as *const libc::c_char,
        tmp___0,
        b"DEVELOPMENT\0" as *const u8 as *const libc::c_char,
    );
    tmp___2 = strlen(gengetopt_args_info_versiontext);
    if tmp___2 > 0 as libc::c_ulong {
        __printf_chk(
            1 as libc::c_int,
            b"\n%s\n\0" as *const u8 as *const libc::c_char,
            gengetopt_args_info_versiontext,
        );
    }
}
unsafe extern "C" fn print_help_common() {
    let mut len_purpose: size_t = 0;
    let mut tmp: size_t = 0;
    let mut len_usage: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: size_t = 0;
    tmp = strlen(gengetopt_args_info_purpose);
    len_purpose = tmp;
    tmp___0 = strlen(gengetopt_args_info_usage);
    len_usage = tmp___0;
    if len_usage > 0 as libc::c_ulong {
        __printf_chk(
            1 as libc::c_int,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            gengetopt_args_info_usage,
        );
    }
    if len_purpose > 0 as libc::c_ulong {
        __printf_chk(
            1 as libc::c_int,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            gengetopt_args_info_purpose,
        );
    }
    if len_usage != 0 {
        __printf_chk(1 as libc::c_int, b"\n\0" as *const u8 as *const libc::c_char);
    } else if len_purpose != 0 {
        __printf_chk(1 as libc::c_int, b"\n\0" as *const u8 as *const libc::c_char);
    }
    tmp___1 = strlen(gengetopt_args_info_description);
    if tmp___1 > 0 as libc::c_ulong {
        __printf_chk(
            1 as libc::c_int,
            b"%s\n\n\0" as *const u8 as *const libc::c_char,
            gengetopt_args_info_description,
        );
    }
}
pub unsafe extern "C" fn cmdline_parser_print_help() {
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    i = 0 as libc::c_int;
    print_help_common();
    while !(gengetopt_args_info_help[i as usize]).is_null() {
        tmp = i;
        i += 1;
        __printf_chk(
            1 as libc::c_int,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            gengetopt_args_info_help[tmp as usize],
        );
    }
}
pub unsafe extern "C" fn cmdline_parser_init(mut args_info: *mut gengetopt_args_info) {
    clear_given(args_info);
    clear_args(args_info);
    init_args_info(args_info);
    (*args_info).inputs = 0 as *mut *mut libc::c_char;
    (*args_info).inputs_num = 0 as libc::c_uint;
}
pub unsafe extern "C" fn cmdline_parser_params_init(
    mut params: *mut cmdline_parser_params,
) {
    if !params.is_null() {
        (*params).override_0 = 0 as libc::c_int;
        (*params).initialize = 1 as libc::c_int;
        (*params).check_required = 1 as libc::c_int;
        (*params).check_ambiguity = 0 as libc::c_int;
        (*params).print_errors = 1 as libc::c_int;
    }
}
pub unsafe extern "C" fn cmdline_parser_params_create() -> *mut cmdline_parser_params {
    let mut params: *mut cmdline_parser_params = 0 as *mut cmdline_parser_params;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<cmdline_parser_params>() as libc::c_ulong);
    params = tmp as *mut cmdline_parser_params;
    cmdline_parser_params_init(params);
    return params;
}
unsafe extern "C" fn free_string_field(mut s: *mut *mut libc::c_char) {
    if !(*s).is_null() {
        free(*s as *mut libc::c_void);
        *s = 0 as *mut libc::c_char;
    }
}
unsafe extern "C" fn cmdline_parser_release(mut args_info: *mut gengetopt_args_info) {
    let mut i: libc::c_uint = 0;
    free_string_field(&mut (*args_info).target_port_orig);
    free_string_field(&mut (*args_info).output_file_arg);
    free_string_field(&mut (*args_info).output_file_orig);
    free_string_field(&mut (*args_info).blocklist_file_arg);
    free_string_field(&mut (*args_info).blocklist_file_orig);
    free_string_field(&mut (*args_info).allowlist_file_arg);
    free_string_field(&mut (*args_info).allowlist_file_orig);
    free_string_field(&mut (*args_info).list_of_ips_file_arg);
    free_string_field(&mut (*args_info).list_of_ips_file_orig);
    free_string_field(&mut (*args_info).rate_orig);
    free_string_field(&mut (*args_info).bandwidth_arg);
    free_string_field(&mut (*args_info).bandwidth_orig);
    free_string_field(&mut (*args_info).batch_orig);
    free_string_field(&mut (*args_info).max_targets_arg);
    free_string_field(&mut (*args_info).max_targets_orig);
    free_string_field(&mut (*args_info).max_runtime_orig);
    free_string_field(&mut (*args_info).max_results_orig);
    free_string_field(&mut (*args_info).probes_orig);
    free_string_field(&mut (*args_info).cooldown_time_orig);
    free_string_field(&mut (*args_info).seed_orig);
    free_string_field(&mut (*args_info).retries_orig);
    free_string_field(&mut (*args_info).shards_orig);
    free_string_field(&mut (*args_info).shard_orig);
    free_string_field(&mut (*args_info).source_port_arg);
    free_string_field(&mut (*args_info).source_port_orig);
    free_string_field(&mut (*args_info).source_ip_arg);
    free_string_field(&mut (*args_info).source_ip_orig);
    free_string_field(&mut (*args_info).gateway_mac_arg);
    free_string_field(&mut (*args_info).gateway_mac_orig);
    free_string_field(&mut (*args_info).source_mac_arg);
    free_string_field(&mut (*args_info).source_mac_orig);
    free_string_field(&mut (*args_info).interface_arg);
    free_string_field(&mut (*args_info).interface_orig);
    free_string_field(&mut (*args_info).probe_module_arg);
    free_string_field(&mut (*args_info).probe_module_orig);
    free_string_field(&mut (*args_info).probe_args_arg);
    free_string_field(&mut (*args_info).probe_args_orig);
    free_string_field(&mut (*args_info).probe_ttl_orig);
    free_string_field(&mut (*args_info).output_fields_arg);
    free_string_field(&mut (*args_info).output_fields_orig);
    free_string_field(&mut (*args_info).output_module_arg);
    free_string_field(&mut (*args_info).output_module_orig);
    free_string_field(&mut (*args_info).output_args_arg);
    free_string_field(&mut (*args_info).output_args_orig);
    free_string_field(&mut (*args_info).output_filter_arg);
    free_string_field(&mut (*args_info).output_filter_orig);
    free_string_field(&mut (*args_info).verbosity_orig);
    free_string_field(&mut (*args_info).log_file_arg);
    free_string_field(&mut (*args_info).log_file_orig);
    free_string_field(&mut (*args_info).log_directory_arg);
    free_string_field(&mut (*args_info).log_directory_orig);
    free_string_field(&mut (*args_info).metadata_file_arg);
    free_string_field(&mut (*args_info).metadata_file_orig);
    free_string_field(&mut (*args_info).status_updates_file_arg);
    free_string_field(&mut (*args_info).status_updates_file_orig);
    free_string_field(&mut (*args_info).notes_arg);
    free_string_field(&mut (*args_info).notes_orig);
    free_string_field(&mut (*args_info).user_metadata_arg);
    free_string_field(&mut (*args_info).user_metadata_orig);
    free_string_field(&mut (*args_info).config_arg);
    free_string_field(&mut (*args_info).config_orig);
    free_string_field(&mut (*args_info).max_sendto_failures_orig);
    free_string_field(&mut (*args_info).min_hitrate_orig);
    free_string_field(&mut (*args_info).sender_threads_orig);
    free_string_field(&mut (*args_info).cores_arg);
    free_string_field(&mut (*args_info).cores_orig);
    i = 0 as libc::c_uint;
    while i < (*args_info).inputs_num {
        free(*((*args_info).inputs).offset(i as isize) as *mut libc::c_void);
        i = i.wrapping_add(1);
    }
    if (*args_info).inputs_num != 0 {
        free((*args_info).inputs as *mut libc::c_void);
    }
    clear_given(args_info);
}
unsafe extern "C" fn write_into_file(
    mut outfile: *mut FILE,
    mut opt: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut values: *mut *const libc::c_char,
) {
    if !arg.is_null() {
        __fprintf_chk(
            outfile,
            1 as libc::c_int,
            b"%s=\"%s\"\n\0" as *const u8 as *const libc::c_char,
            opt,
            arg,
        );
    } else {
        __fprintf_chk(
            outfile,
            1 as libc::c_int,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            opt,
        );
    };
}
pub unsafe extern "C" fn cmdline_parser_dump(
    mut outfile: *mut FILE,
    mut args_info: *mut gengetopt_args_info,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    if outfile.is_null() {
        __fprintf_chk(
            stderr,
            1 as libc::c_int,
            b"%s: cannot dump options to stream\n\0" as *const u8 as *const libc::c_char,
            b"zmap\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    if (*args_info).target_port_given != 0 {
        write_into_file(
            outfile,
            b"target-port\0" as *const u8 as *const libc::c_char,
            (*args_info).target_port_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).output_file_given != 0 {
        write_into_file(
            outfile,
            b"output-file\0" as *const u8 as *const libc::c_char,
            (*args_info).output_file_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).blocklist_file_given != 0 {
        write_into_file(
            outfile,
            b"blocklist-file\0" as *const u8 as *const libc::c_char,
            (*args_info).blocklist_file_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).allowlist_file_given != 0 {
        write_into_file(
            outfile,
            b"allowlist-file\0" as *const u8 as *const libc::c_char,
            (*args_info).allowlist_file_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).list_of_ips_file_given != 0 {
        write_into_file(
            outfile,
            b"list-of-ips-file\0" as *const u8 as *const libc::c_char,
            (*args_info).list_of_ips_file_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).rate_given != 0 {
        write_into_file(
            outfile,
            b"rate\0" as *const u8 as *const libc::c_char,
            (*args_info).rate_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).bandwidth_given != 0 {
        write_into_file(
            outfile,
            b"bandwidth\0" as *const u8 as *const libc::c_char,
            (*args_info).bandwidth_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).batch_given != 0 {
        write_into_file(
            outfile,
            b"batch\0" as *const u8 as *const libc::c_char,
            (*args_info).batch_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).max_targets_given != 0 {
        write_into_file(
            outfile,
            b"max-targets\0" as *const u8 as *const libc::c_char,
            (*args_info).max_targets_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).max_runtime_given != 0 {
        write_into_file(
            outfile,
            b"max-runtime\0" as *const u8 as *const libc::c_char,
            (*args_info).max_runtime_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).max_results_given != 0 {
        write_into_file(
            outfile,
            b"max-results\0" as *const u8 as *const libc::c_char,
            (*args_info).max_results_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).probes_given != 0 {
        write_into_file(
            outfile,
            b"probes\0" as *const u8 as *const libc::c_char,
            (*args_info).probes_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).cooldown_time_given != 0 {
        write_into_file(
            outfile,
            b"cooldown-time\0" as *const u8 as *const libc::c_char,
            (*args_info).cooldown_time_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).seed_given != 0 {
        write_into_file(
            outfile,
            b"seed\0" as *const u8 as *const libc::c_char,
            (*args_info).seed_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).retries_given != 0 {
        write_into_file(
            outfile,
            b"retries\0" as *const u8 as *const libc::c_char,
            (*args_info).retries_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).dryrun_given != 0 {
        write_into_file(
            outfile,
            b"dryrun\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).shards_given != 0 {
        write_into_file(
            outfile,
            b"shards\0" as *const u8 as *const libc::c_char,
            (*args_info).shards_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).shard_given != 0 {
        write_into_file(
            outfile,
            b"shard\0" as *const u8 as *const libc::c_char,
            (*args_info).shard_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).source_port_given != 0 {
        write_into_file(
            outfile,
            b"source-port\0" as *const u8 as *const libc::c_char,
            (*args_info).source_port_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).source_ip_given != 0 {
        write_into_file(
            outfile,
            b"source-ip\0" as *const u8 as *const libc::c_char,
            (*args_info).source_ip_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).gateway_mac_given != 0 {
        write_into_file(
            outfile,
            b"gateway-mac\0" as *const u8 as *const libc::c_char,
            (*args_info).gateway_mac_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).source_mac_given != 0 {
        write_into_file(
            outfile,
            b"source-mac\0" as *const u8 as *const libc::c_char,
            (*args_info).source_mac_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).interface_given != 0 {
        write_into_file(
            outfile,
            b"interface\0" as *const u8 as *const libc::c_char,
            (*args_info).interface_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).iplayer_given != 0 {
        write_into_file(
            outfile,
            b"iplayer\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).probe_module_given != 0 {
        write_into_file(
            outfile,
            b"probe-module\0" as *const u8 as *const libc::c_char,
            (*args_info).probe_module_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).probe_args_given != 0 {
        write_into_file(
            outfile,
            b"probe-args\0" as *const u8 as *const libc::c_char,
            (*args_info).probe_args_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).probe_ttl_given != 0 {
        write_into_file(
            outfile,
            b"probe-ttl\0" as *const u8 as *const libc::c_char,
            (*args_info).probe_ttl_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).list_probe_modules_given != 0 {
        write_into_file(
            outfile,
            b"list-probe-modules\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).output_fields_given != 0 {
        write_into_file(
            outfile,
            b"output-fields\0" as *const u8 as *const libc::c_char,
            (*args_info).output_fields_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).output_module_given != 0 {
        write_into_file(
            outfile,
            b"output-module\0" as *const u8 as *const libc::c_char,
            (*args_info).output_module_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).output_args_given != 0 {
        write_into_file(
            outfile,
            b"output-args\0" as *const u8 as *const libc::c_char,
            (*args_info).output_args_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).output_filter_given != 0 {
        write_into_file(
            outfile,
            b"output-filter\0" as *const u8 as *const libc::c_char,
            (*args_info).output_filter_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).list_output_modules_given != 0 {
        write_into_file(
            outfile,
            b"list-output-modules\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).list_output_fields_given != 0 {
        write_into_file(
            outfile,
            b"list-output-fields\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).no_header_row_given != 0 {
        write_into_file(
            outfile,
            b"no-header-row\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).verbosity_given != 0 {
        write_into_file(
            outfile,
            b"verbosity\0" as *const u8 as *const libc::c_char,
            (*args_info).verbosity_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).log_file_given != 0 {
        write_into_file(
            outfile,
            b"log-file\0" as *const u8 as *const libc::c_char,
            (*args_info).log_file_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).log_directory_given != 0 {
        write_into_file(
            outfile,
            b"log-directory\0" as *const u8 as *const libc::c_char,
            (*args_info).log_directory_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).metadata_file_given != 0 {
        write_into_file(
            outfile,
            b"metadata-file\0" as *const u8 as *const libc::c_char,
            (*args_info).metadata_file_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).status_updates_file_given != 0 {
        write_into_file(
            outfile,
            b"status-updates-file\0" as *const u8 as *const libc::c_char,
            (*args_info).status_updates_file_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).quiet_given != 0 {
        write_into_file(
            outfile,
            b"quiet\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).disable_syslog_given != 0 {
        write_into_file(
            outfile,
            b"disable-syslog\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).notes_given != 0 {
        write_into_file(
            outfile,
            b"notes\0" as *const u8 as *const libc::c_char,
            (*args_info).notes_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).user_metadata_given != 0 {
        write_into_file(
            outfile,
            b"user-metadata\0" as *const u8 as *const libc::c_char,
            (*args_info).user_metadata_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).config_given != 0 {
        write_into_file(
            outfile,
            b"config\0" as *const u8 as *const libc::c_char,
            (*args_info).config_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).max_sendto_failures_given != 0 {
        write_into_file(
            outfile,
            b"max-sendto-failures\0" as *const u8 as *const libc::c_char,
            (*args_info).max_sendto_failures_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).min_hitrate_given != 0 {
        write_into_file(
            outfile,
            b"min-hitrate\0" as *const u8 as *const libc::c_char,
            (*args_info).min_hitrate_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).sender_threads_given != 0 {
        write_into_file(
            outfile,
            b"sender-threads\0" as *const u8 as *const libc::c_char,
            (*args_info).sender_threads_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).cores_given != 0 {
        write_into_file(
            outfile,
            b"cores\0" as *const u8 as *const libc::c_char,
            (*args_info).cores_orig as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).ignore_blocklist_errors_given != 0 {
        write_into_file(
            outfile,
            b"ignore-blocklist-errors\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).help_given != 0 {
        write_into_file(
            outfile,
            b"help\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    if (*args_info).version_given != 0 {
        write_into_file(
            outfile,
            b"version\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *mut *const libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    return i;
}
pub unsafe extern "C" fn cmdline_parser_file_save(
    mut filename: *const libc::c_char,
    mut args_info: *mut gengetopt_args_info,
) -> libc::c_int {
    let mut outfile: *mut FILE = 0 as *mut FILE;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    outfile = fopen(filename, b"w\0" as *const u8 as *const libc::c_char);
    if outfile.is_null() {
        __fprintf_chk(
            stderr,
            1 as libc::c_int,
            b"%s: cannot open file for writing: %s\n\0" as *const u8
                as *const libc::c_char,
            b"zmap\0" as *const u8 as *const libc::c_char,
            filename,
        );
        return 1 as libc::c_int;
    }
    i = cmdline_parser_dump(outfile, args_info);
    fclose(outfile);
    return i;
}
pub unsafe extern "C" fn cmdline_parser_free(mut args_info: *mut gengetopt_args_info) {
    cmdline_parser_release(args_info);
}
unsafe extern "C" fn gengetopt_strdup(mut s: *const libc::c_char) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: size_t = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    result = 0 as *mut libc::c_char;
    if s.is_null() {
        return result;
    }
    tmp = strlen(s);
    tmp___0 = malloc(tmp.wrapping_add(1 as libc::c_ulong));
    result = tmp___0 as *mut libc::c_char;
    if result as libc::c_ulong == 0 as *mut libc::c_char as libc::c_ulong {
        return 0 as *mut libc::c_char;
    }
    strcpy(result, s);
    return result;
}
pub unsafe extern "C" fn cmdline_parser(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut args_info: *mut gengetopt_args_info,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = cmdline_parser2(
        argc,
        argv,
        args_info,
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    );
    return tmp;
}
pub unsafe extern "C" fn cmdline_parser_ext(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut args_info: *mut gengetopt_args_info,
    mut params: *mut cmdline_parser_params,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    result = cmdline_parser_internal(
        argc,
        argv,
        args_info,
        params,
        0 as *const libc::c_char,
    );
    if result == 1 as libc::c_int {
        cmdline_parser_free(args_info);
        exit(1 as libc::c_int);
    }
    return result;
}
pub unsafe extern "C" fn cmdline_parser2(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut args_info: *mut gengetopt_args_info,
    mut override_0: libc::c_int,
    mut initialize: libc::c_int,
    mut check_required: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut params: cmdline_parser_params = cmdline_parser_params {
        override_0: 0,
        initialize: 0,
        check_required: 0,
        check_ambiguity: 0,
        print_errors: 0,
    };
    params.override_0 = override_0;
    params.initialize = initialize;
    params.check_required = check_required;
    params.check_ambiguity = 0 as libc::c_int;
    params.print_errors = 1 as libc::c_int;
    result = cmdline_parser_internal(
        argc,
        argv,
        args_info,
        &mut params,
        0 as *const libc::c_char,
    );
    if result == 1 as libc::c_int {
        cmdline_parser_free(args_info);
        exit(1 as libc::c_int);
    }
    return result;
}
pub unsafe extern "C" fn cmdline_parser_required(
    mut args_info: *mut gengetopt_args_info,
    mut prog_name: *const libc::c_char,
) -> libc::c_int {
    return 0 as libc::c_int;
}
static mut package_name: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
unsafe extern "C" fn update_arg(
    mut field: *mut libc::c_void,
    mut orig_field: *mut *mut libc::c_char,
    mut field_given: *mut libc::c_uint,
    mut prev_given: *mut libc::c_uint,
    mut value: *mut libc::c_char,
    mut possible_values: *mut *const libc::c_char,
    mut default_value: *const libc::c_char,
    mut arg_type: cmdline_parser_arg_type,
    mut check_ambiguity: libc::c_int,
    mut override_0: libc::c_int,
    mut no_free: libc::c_int,
    mut multiple_option: libc::c_int,
    mut long_opt: *const libc::c_char,
    mut short_opt: libc::c_char,
    mut additional_error: *const libc::c_char,
) -> libc::c_int {
    let mut stop_char: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: *const libc::c_char = 0 as *const libc::c_char;
    let mut found: libc::c_int = 0;
    let mut string_field: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: libc::c_long = 0;
    let mut tmp___2: libc::c_double = 0.;
    let mut tmp___3: libc::c_long = 0;
    stop_char = 0 as *mut libc::c_char;
    val = value as *const libc::c_char;
    stop_char = 0 as *mut libc::c_char;
    found = 0 as libc::c_int;
    if multiple_option == 0 {
        if !prev_given.is_null() {
            's_122: {
                let mut current_block_19: u64;
                if !(*prev_given != 0) {
                    if check_ambiguity != 0 {
                        if *field_given != 0 {
                            current_block_19 = 8583738727092093706;
                        } else {
                            current_block_19 = 11194104282611034094;
                        }
                    } else {
                        current_block_19 = 11194104282611034094;
                    }
                    match current_block_19 {
                        8583738727092093706 => {}
                        _ => {
                            break 's_122;
                        }
                    }
                }
                if short_opt as libc::c_int != 45 as libc::c_int {
                    if !additional_error.is_null() {
                        tmp = additional_error;
                    } else {
                        tmp = b"\0" as *const u8 as *const libc::c_char;
                    }
                    __fprintf_chk(
                        stderr,
                        1 as libc::c_int,
                        b"%s: `--%s' (`-%c') option given more than once%s\n\0"
                            as *const u8 as *const libc::c_char,
                        package_name,
                        long_opt,
                        short_opt as libc::c_int,
                        tmp,
                    );
                } else {
                    if !additional_error.is_null() {
                        tmp___0 = additional_error;
                    } else {
                        tmp___0 = b"\0" as *const u8 as *const libc::c_char;
                    }
                    __fprintf_chk(
                        stderr,
                        1 as libc::c_int,
                        b"%s: `--%s' option given more than once%s\n\0" as *const u8
                            as *const libc::c_char,
                        package_name,
                        long_opt,
                        tmp___0,
                    );
                }
                return 1 as libc::c_int;
            }
        }
    }
    if !field_given.is_null() {
        if *field_given != 0 {
            if override_0 == 0 {
                return 0 as libc::c_int;
            }
        }
    }
    if !prev_given.is_null() {
        *prev_given = (*prev_given).wrapping_add(1);
    }
    if !field_given.is_null() {
        *field_given = (*field_given).wrapping_add(1);
    }
    if !possible_values.is_null() {
        val = *possible_values.offset(found as isize);
    }
    match arg_type as libc::c_uint {
        2 => {
            if !val.is_null() {
                tmp___1 = strtol(
                    val,
                    &mut stop_char as *mut *mut libc::c_char,
                    0 as libc::c_int,
                );
                *(field as *mut libc::c_int) = tmp___1 as libc::c_int;
            }
        }
        3 => {
            if !val.is_null() {
                tmp___2 = strtod(val, &mut stop_char as *mut *mut libc::c_char);
                *(field as *mut libc::c_float) = tmp___2 as libc::c_float;
            }
        }
        4 => {
            if !val.is_null() {
                tmp___3 = strtol(
                    val,
                    &mut stop_char as *mut *mut libc::c_char,
                    0 as libc::c_int,
                );
                *(field as *mut libc::c_long) = tmp___3;
            }
        }
        1 => {
            if !val.is_null() {
                string_field = field as *mut *mut libc::c_char;
                if no_free == 0 {
                    if !(*string_field).is_null() {
                        free(*string_field as *mut libc::c_void);
                    }
                }
                *string_field = gengetopt_strdup(val);
            }
        }
        _ => {}
    }
    match arg_type as libc::c_uint {
        4 | 3 | 2 => {
            if !val.is_null() {
                if !stop_char.is_null() {
                    if !(*stop_char as libc::c_int == 0 as libc::c_int) {
                        __fprintf_chk(
                            stderr,
                            1 as libc::c_int,
                            b"%s: invalid numeric value: %s\n\0" as *const u8
                                as *const libc::c_char,
                            package_name,
                            val,
                        );
                        return 1 as libc::c_int;
                    }
                } else {
                    __fprintf_chk(
                        stderr,
                        1 as libc::c_int,
                        b"%s: invalid numeric value: %s\n\0" as *const u8
                            as *const libc::c_char,
                        package_name,
                        val,
                    );
                    return 1 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    match arg_type as libc::c_uint {
        0 => {}
        _ => {
            if !value.is_null() {
                if !orig_field.is_null() {
                    if no_free != 0 {
                        *orig_field = value;
                    } else {
                        if !(*orig_field).is_null() {
                            free(*orig_field as *mut libc::c_void);
                        }
                        *orig_field = gengetopt_strdup(value as *const libc::c_char);
                    }
                }
            }
        }
    }
    return 0 as libc::c_int;
}
static mut long_options: [option; 53] = [
    {
        let mut init = option {
            name: b"target-port\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"output-file\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'o' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"blocklist-file\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"allowlist-file\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'w' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"list-of-ips-file\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'I' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"rate\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"bandwidth\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'B' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"batch\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"max-targets\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"max-runtime\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"max-results\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'N' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"probes\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'P' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"cooldown-time\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"seed\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'e' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"retries\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"dryrun\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"shards\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"shard\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"source-port\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"source-ip\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'S' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"gateway-mac\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'G' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"source-mac\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"interface\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"iplayer\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'X' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"probe-module\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'M' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"probe-args\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"probe-ttl\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"list-probe-modules\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"output-fields\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'f' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"output-module\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'O' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"output-args\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"output-filter\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"list-output-modules\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"list-output-fields\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-header-row\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"verbosity\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"log-file\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"log-directory\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'L' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"metadata-file\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'm' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"status-updates-file\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'u' as i32,
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
            name: b"disable-syslog\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"notes\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"user-metadata\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"config\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'C' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"max-sendto-failures\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"min-hitrate\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"sender-threads\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'T' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"cores\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-blocklist-errors\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 0 as libc::c_int,
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
            name: 0 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn cmdline_parser_internal(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut args_info: *mut gengetopt_args_info,
    mut params: *mut cmdline_parser_params,
    mut additional_error: *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut c: libc::c_int = 0;
    let mut error_occurred: libc::c_int = 0;
    let mut local_args_info: gengetopt_args_info = gengetopt_args_info {
        target_port_arg: 0,
        target_port_orig: 0 as *mut libc::c_char,
        target_port_help: 0 as *const libc::c_char,
        output_file_arg: 0 as *mut libc::c_char,
        output_file_orig: 0 as *mut libc::c_char,
        output_file_help: 0 as *const libc::c_char,
        blocklist_file_arg: 0 as *mut libc::c_char,
        blocklist_file_orig: 0 as *mut libc::c_char,
        blocklist_file_help: 0 as *const libc::c_char,
        allowlist_file_arg: 0 as *mut libc::c_char,
        allowlist_file_orig: 0 as *mut libc::c_char,
        allowlist_file_help: 0 as *const libc::c_char,
        list_of_ips_file_arg: 0 as *mut libc::c_char,
        list_of_ips_file_orig: 0 as *mut libc::c_char,
        list_of_ips_file_help: 0 as *const libc::c_char,
        rate_arg: 0,
        rate_orig: 0 as *mut libc::c_char,
        rate_help: 0 as *const libc::c_char,
        bandwidth_arg: 0 as *mut libc::c_char,
        bandwidth_orig: 0 as *mut libc::c_char,
        bandwidth_help: 0 as *const libc::c_char,
        batch_arg: 0,
        batch_orig: 0 as *mut libc::c_char,
        batch_help: 0 as *const libc::c_char,
        max_targets_arg: 0 as *mut libc::c_char,
        max_targets_orig: 0 as *mut libc::c_char,
        max_targets_help: 0 as *const libc::c_char,
        max_runtime_arg: 0,
        max_runtime_orig: 0 as *mut libc::c_char,
        max_runtime_help: 0 as *const libc::c_char,
        max_results_arg: 0,
        max_results_orig: 0 as *mut libc::c_char,
        max_results_help: 0 as *const libc::c_char,
        probes_arg: 0,
        probes_orig: 0 as *mut libc::c_char,
        probes_help: 0 as *const libc::c_char,
        cooldown_time_arg: 0,
        cooldown_time_orig: 0 as *mut libc::c_char,
        cooldown_time_help: 0 as *const libc::c_char,
        seed_arg: 0,
        seed_orig: 0 as *mut libc::c_char,
        seed_help: 0 as *const libc::c_char,
        retries_arg: 0,
        retries_orig: 0 as *mut libc::c_char,
        retries_help: 0 as *const libc::c_char,
        dryrun_help: 0 as *const libc::c_char,
        shards_arg: 0,
        shards_orig: 0 as *mut libc::c_char,
        shards_help: 0 as *const libc::c_char,
        shard_arg: 0,
        shard_orig: 0 as *mut libc::c_char,
        shard_help: 0 as *const libc::c_char,
        source_port_arg: 0 as *mut libc::c_char,
        source_port_orig: 0 as *mut libc::c_char,
        source_port_help: 0 as *const libc::c_char,
        source_ip_arg: 0 as *mut libc::c_char,
        source_ip_orig: 0 as *mut libc::c_char,
        source_ip_help: 0 as *const libc::c_char,
        gateway_mac_arg: 0 as *mut libc::c_char,
        gateway_mac_orig: 0 as *mut libc::c_char,
        gateway_mac_help: 0 as *const libc::c_char,
        source_mac_arg: 0 as *mut libc::c_char,
        source_mac_orig: 0 as *mut libc::c_char,
        source_mac_help: 0 as *const libc::c_char,
        interface_arg: 0 as *mut libc::c_char,
        interface_orig: 0 as *mut libc::c_char,
        interface_help: 0 as *const libc::c_char,
        iplayer_help: 0 as *const libc::c_char,
        probe_module_arg: 0 as *mut libc::c_char,
        probe_module_orig: 0 as *mut libc::c_char,
        probe_module_help: 0 as *const libc::c_char,
        probe_args_arg: 0 as *mut libc::c_char,
        probe_args_orig: 0 as *mut libc::c_char,
        probe_args_help: 0 as *const libc::c_char,
        probe_ttl_arg: 0,
        probe_ttl_orig: 0 as *mut libc::c_char,
        probe_ttl_help: 0 as *const libc::c_char,
        list_probe_modules_help: 0 as *const libc::c_char,
        output_fields_arg: 0 as *mut libc::c_char,
        output_fields_orig: 0 as *mut libc::c_char,
        output_fields_help: 0 as *const libc::c_char,
        output_module_arg: 0 as *mut libc::c_char,
        output_module_orig: 0 as *mut libc::c_char,
        output_module_help: 0 as *const libc::c_char,
        output_args_arg: 0 as *mut libc::c_char,
        output_args_orig: 0 as *mut libc::c_char,
        output_args_help: 0 as *const libc::c_char,
        output_filter_arg: 0 as *mut libc::c_char,
        output_filter_orig: 0 as *mut libc::c_char,
        output_filter_help: 0 as *const libc::c_char,
        list_output_modules_help: 0 as *const libc::c_char,
        list_output_fields_help: 0 as *const libc::c_char,
        no_header_row_help: 0 as *const libc::c_char,
        verbosity_arg: 0,
        verbosity_orig: 0 as *mut libc::c_char,
        verbosity_help: 0 as *const libc::c_char,
        log_file_arg: 0 as *mut libc::c_char,
        log_file_orig: 0 as *mut libc::c_char,
        log_file_help: 0 as *const libc::c_char,
        log_directory_arg: 0 as *mut libc::c_char,
        log_directory_orig: 0 as *mut libc::c_char,
        log_directory_help: 0 as *const libc::c_char,
        metadata_file_arg: 0 as *mut libc::c_char,
        metadata_file_orig: 0 as *mut libc::c_char,
        metadata_file_help: 0 as *const libc::c_char,
        status_updates_file_arg: 0 as *mut libc::c_char,
        status_updates_file_orig: 0 as *mut libc::c_char,
        status_updates_file_help: 0 as *const libc::c_char,
        quiet_help: 0 as *const libc::c_char,
        disable_syslog_help: 0 as *const libc::c_char,
        notes_arg: 0 as *mut libc::c_char,
        notes_orig: 0 as *mut libc::c_char,
        notes_help: 0 as *const libc::c_char,
        user_metadata_arg: 0 as *mut libc::c_char,
        user_metadata_orig: 0 as *mut libc::c_char,
        user_metadata_help: 0 as *const libc::c_char,
        config_arg: 0 as *mut libc::c_char,
        config_orig: 0 as *mut libc::c_char,
        config_help: 0 as *const libc::c_char,
        max_sendto_failures_arg: 0,
        max_sendto_failures_orig: 0 as *mut libc::c_char,
        max_sendto_failures_help: 0 as *const libc::c_char,
        min_hitrate_arg: 0.,
        min_hitrate_orig: 0 as *mut libc::c_char,
        min_hitrate_help: 0 as *const libc::c_char,
        sender_threads_arg: 0,
        sender_threads_orig: 0 as *mut libc::c_char,
        sender_threads_help: 0 as *const libc::c_char,
        cores_arg: 0 as *mut libc::c_char,
        cores_orig: 0 as *mut libc::c_char,
        cores_help: 0 as *const libc::c_char,
        ignore_blocklist_errors_help: 0 as *const libc::c_char,
        help_help: 0 as *const libc::c_char,
        version_help: 0 as *const libc::c_char,
        target_port_given: 0,
        output_file_given: 0,
        blocklist_file_given: 0,
        allowlist_file_given: 0,
        list_of_ips_file_given: 0,
        rate_given: 0,
        bandwidth_given: 0,
        batch_given: 0,
        max_targets_given: 0,
        max_runtime_given: 0,
        max_results_given: 0,
        probes_given: 0,
        cooldown_time_given: 0,
        seed_given: 0,
        retries_given: 0,
        dryrun_given: 0,
        shards_given: 0,
        shard_given: 0,
        source_port_given: 0,
        source_ip_given: 0,
        gateway_mac_given: 0,
        source_mac_given: 0,
        interface_given: 0,
        iplayer_given: 0,
        probe_module_given: 0,
        probe_args_given: 0,
        probe_ttl_given: 0,
        list_probe_modules_given: 0,
        output_fields_given: 0,
        output_module_given: 0,
        output_args_given: 0,
        output_filter_given: 0,
        list_output_modules_given: 0,
        list_output_fields_given: 0,
        no_header_row_given: 0,
        verbosity_given: 0,
        log_file_given: 0,
        log_directory_given: 0,
        metadata_file_given: 0,
        status_updates_file_given: 0,
        quiet_given: 0,
        disable_syslog_given: 0,
        notes_given: 0,
        user_metadata_given: 0,
        config_given: 0,
        max_sendto_failures_given: 0,
        min_hitrate_given: 0,
        sender_threads_given: 0,
        cores_given: 0,
        ignore_blocklist_errors_given: 0,
        help_given: 0,
        version_given: 0,
        inputs: 0 as *mut *mut libc::c_char,
        inputs_num: 0,
    };
    let mut override_0: libc::c_int = 0;
    let mut initialize: libc::c_int = 0;
    let mut check_required: libc::c_int = 0;
    let mut check_ambiguity: libc::c_int = 0;
    let mut option_index: libc::c_int = 0;
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
    let mut tmp___28: libc::c_int = 0;
    let mut tmp___29: libc::c_int = 0;
    let mut tmp___30: libc::c_int = 0;
    let mut tmp___31: libc::c_int = 0;
    let mut tmp___32: libc::c_int = 0;
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
    let mut tmp___58: libc::c_int = 0;
    let mut tmp___59: libc::c_int = 0;
    let mut tmp___60: libc::c_int = 0;
    let mut tmp___61: libc::c_int = 0;
    let mut tmp___62: libc::c_int = 0;
    let mut tmp___63: libc::c_int = 0;
    let mut tmp___64: libc::c_int = 0;
    let mut tmp___65: libc::c_int = 0;
    let mut tmp___66: libc::c_int = 0;
    let mut tmp___67: libc::c_int = 0;
    let mut tmp___68: libc::c_int = 0;
    let mut tmp___69: libc::c_int = 0;
    let mut tmp___70: libc::c_int = 0;
    let mut tmp___71: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut found_prog_name: libc::c_int = 0;
    let mut tmp___72: libc::c_int = 0;
    let mut tmp___73: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___74: libc::c_int = 0;
    let mut tmp___75: libc::c_int = 0;
    error_occurred = 0 as libc::c_int;
    package_name = *argv.offset(0 as libc::c_int as isize);
    override_0 = (*params).override_0;
    initialize = (*params).initialize;
    check_required = (*params).check_required;
    check_ambiguity = (*params).check_ambiguity;
    if initialize != 0 {
        cmdline_parser_init(args_info);
    }
    cmdline_parser_init(&mut local_args_info);
    optarg = 0 as *mut libc::c_char;
    optind = 0 as libc::c_int;
    opterr = (*params).print_errors;
    optopt = '?' as i32;
    loop {
        option_index = 0 as libc::c_int;
        c = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            b"p:o:b:w:I:r:B:n:t:N:P:c:e:ds:S:G:i:XM:f:O:v:l:L:m:u:qC:T:hV\0" as *const u8
                as *const libc::c_char,
            long_options.as_mut_ptr() as *const option,
            &mut option_index,
        );
        if c == -(1 as libc::c_int) {
            current_block = 8700473759921513224;
            break;
        }
        match c {
            112 => {
                tmp = update_arg(
                    &mut (*args_info).target_port_arg as *mut libc::c_int
                        as *mut libc::c_void,
                    &mut (*args_info).target_port_orig,
                    &mut (*args_info).target_port_given,
                    &mut local_args_info.target_port_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_INT,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"target-port\0" as *const u8 as *const libc::c_char,
                    'p' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            111 => {
                tmp___0 = update_arg(
                    &mut (*args_info).output_file_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).output_file_orig,
                    &mut (*args_info).output_file_given,
                    &mut local_args_info.output_file_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"output-file\0" as *const u8 as *const libc::c_char,
                    'o' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___0 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            98 => {
                tmp___1 = update_arg(
                    &mut (*args_info).blocklist_file_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).blocklist_file_orig,
                    &mut (*args_info).blocklist_file_given,
                    &mut local_args_info.blocklist_file_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"blocklist-file\0" as *const u8 as *const libc::c_char,
                    'b' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___1 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            119 => {
                tmp___2 = update_arg(
                    &mut (*args_info).allowlist_file_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).allowlist_file_orig,
                    &mut (*args_info).allowlist_file_given,
                    &mut local_args_info.allowlist_file_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"allowlist-file\0" as *const u8 as *const libc::c_char,
                    'w' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___2 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            73 => {
                tmp___3 = update_arg(
                    &mut (*args_info).list_of_ips_file_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).list_of_ips_file_orig,
                    &mut (*args_info).list_of_ips_file_given,
                    &mut local_args_info.list_of_ips_file_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"list-of-ips-file\0" as *const u8 as *const libc::c_char,
                    'I' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___3 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            114 => {
                tmp___4 = update_arg(
                    &mut (*args_info).rate_arg as *mut libc::c_int as *mut libc::c_void,
                    &mut (*args_info).rate_orig,
                    &mut (*args_info).rate_given,
                    &mut local_args_info.rate_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_INT,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"rate\0" as *const u8 as *const libc::c_char,
                    'r' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___4 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            66 => {
                tmp___5 = update_arg(
                    &mut (*args_info).bandwidth_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).bandwidth_orig,
                    &mut (*args_info).bandwidth_given,
                    &mut local_args_info.bandwidth_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"bandwidth\0" as *const u8 as *const libc::c_char,
                    'B' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___5 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            110 => {
                tmp___6 = update_arg(
                    &mut (*args_info).max_targets_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).max_targets_orig,
                    &mut (*args_info).max_targets_given,
                    &mut local_args_info.max_targets_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"max-targets\0" as *const u8 as *const libc::c_char,
                    'n' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___6 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            116 => {
                tmp___7 = update_arg(
                    &mut (*args_info).max_runtime_arg as *mut libc::c_int
                        as *mut libc::c_void,
                    &mut (*args_info).max_runtime_orig,
                    &mut (*args_info).max_runtime_given,
                    &mut local_args_info.max_runtime_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_INT,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"max-runtime\0" as *const u8 as *const libc::c_char,
                    't' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___7 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            78 => {
                tmp___8 = update_arg(
                    &mut (*args_info).max_results_arg as *mut libc::c_int
                        as *mut libc::c_void,
                    &mut (*args_info).max_results_orig,
                    &mut (*args_info).max_results_given,
                    &mut local_args_info.max_results_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_INT,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"max-results\0" as *const u8 as *const libc::c_char,
                    'N' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___8 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            80 => {
                tmp___9 = update_arg(
                    &mut (*args_info).probes_arg as *mut libc::c_int
                        as *mut libc::c_void,
                    &mut (*args_info).probes_orig,
                    &mut (*args_info).probes_given,
                    &mut local_args_info.probes_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    b"1\0" as *const u8 as *const libc::c_char,
                    ARG_INT,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"probes\0" as *const u8 as *const libc::c_char,
                    'P' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___9 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            99 => {
                tmp___10 = update_arg(
                    &mut (*args_info).cooldown_time_arg as *mut libc::c_int
                        as *mut libc::c_void,
                    &mut (*args_info).cooldown_time_orig,
                    &mut (*args_info).cooldown_time_given,
                    &mut local_args_info.cooldown_time_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    b"8\0" as *const u8 as *const libc::c_char,
                    ARG_INT,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"cooldown-time\0" as *const u8 as *const libc::c_char,
                    'c' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___10 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            101 => {
                tmp___11 = update_arg(
                    &mut (*args_info).seed_arg as *mut libc::c_long as *mut libc::c_void,
                    &mut (*args_info).seed_orig,
                    &mut (*args_info).seed_given,
                    &mut local_args_info.seed_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_LONGLONG,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"seed\0" as *const u8 as *const libc::c_char,
                    'e' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___11 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            100 => {
                tmp___12 = update_arg(
                    0 as *mut libc::c_void,
                    0 as *mut *mut libc::c_char,
                    &mut (*args_info).dryrun_given,
                    &mut local_args_info.dryrun_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_NO,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"dryrun\0" as *const u8 as *const libc::c_char,
                    'd' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___12 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            115 => {
                tmp___13 = update_arg(
                    &mut (*args_info).source_port_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).source_port_orig,
                    &mut (*args_info).source_port_given,
                    &mut local_args_info.source_port_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"source-port\0" as *const u8 as *const libc::c_char,
                    's' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___13 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            83 => {
                tmp___14 = update_arg(
                    &mut (*args_info).source_ip_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).source_ip_orig,
                    &mut (*args_info).source_ip_given,
                    &mut local_args_info.source_ip_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"source-ip\0" as *const u8 as *const libc::c_char,
                    'S' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___14 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            71 => {
                tmp___15 = update_arg(
                    &mut (*args_info).gateway_mac_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).gateway_mac_orig,
                    &mut (*args_info).gateway_mac_given,
                    &mut local_args_info.gateway_mac_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"gateway-mac\0" as *const u8 as *const libc::c_char,
                    'G' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___15 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            105 => {
                tmp___16 = update_arg(
                    &mut (*args_info).interface_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).interface_orig,
                    &mut (*args_info).interface_given,
                    &mut local_args_info.interface_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"interface\0" as *const u8 as *const libc::c_char,
                    'i' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___16 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            88 => {
                tmp___17 = update_arg(
                    0 as *mut libc::c_void,
                    0 as *mut *mut libc::c_char,
                    &mut (*args_info).iplayer_given,
                    &mut local_args_info.iplayer_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_NO,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"iplayer\0" as *const u8 as *const libc::c_char,
                    'X' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___17 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            77 => {
                tmp___18 = update_arg(
                    &mut (*args_info).probe_module_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).probe_module_orig,
                    &mut (*args_info).probe_module_given,
                    &mut local_args_info.probe_module_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    b"tcp_synscan\0" as *const u8 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"probe-module\0" as *const u8 as *const libc::c_char,
                    'M' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___18 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            102 => {
                tmp___19 = update_arg(
                    &mut (*args_info).output_fields_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).output_fields_orig,
                    &mut (*args_info).output_fields_given,
                    &mut local_args_info.output_fields_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"output-fields\0" as *const u8 as *const libc::c_char,
                    'f' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___19 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            79 => {
                tmp___20 = update_arg(
                    &mut (*args_info).output_module_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).output_module_orig,
                    &mut (*args_info).output_module_given,
                    &mut local_args_info.output_module_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"output-module\0" as *const u8 as *const libc::c_char,
                    'O' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___20 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            118 => {
                tmp___21 = update_arg(
                    &mut (*args_info).verbosity_arg as *mut libc::c_int
                        as *mut libc::c_void,
                    &mut (*args_info).verbosity_orig,
                    &mut (*args_info).verbosity_given,
                    &mut local_args_info.verbosity_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    b"3\0" as *const u8 as *const libc::c_char,
                    ARG_INT,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"verbosity\0" as *const u8 as *const libc::c_char,
                    'v' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___21 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            108 => {
                tmp___22 = update_arg(
                    &mut (*args_info).log_file_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).log_file_orig,
                    &mut (*args_info).log_file_given,
                    &mut local_args_info.log_file_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"log-file\0" as *const u8 as *const libc::c_char,
                    'l' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___22 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            76 => {
                tmp___23 = update_arg(
                    &mut (*args_info).log_directory_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).log_directory_orig,
                    &mut (*args_info).log_directory_given,
                    &mut local_args_info.log_directory_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"log-directory\0" as *const u8 as *const libc::c_char,
                    'L' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___23 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            109 => {
                tmp___24 = update_arg(
                    &mut (*args_info).metadata_file_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).metadata_file_orig,
                    &mut (*args_info).metadata_file_given,
                    &mut local_args_info.metadata_file_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"metadata-file\0" as *const u8 as *const libc::c_char,
                    'm' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___24 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            117 => {
                tmp___25 = update_arg(
                    &mut (*args_info).status_updates_file_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).status_updates_file_orig,
                    &mut (*args_info).status_updates_file_given,
                    &mut local_args_info.status_updates_file_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"status-updates-file\0" as *const u8 as *const libc::c_char,
                    'u' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___25 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            113 => {
                tmp___26 = update_arg(
                    0 as *mut libc::c_void,
                    0 as *mut *mut libc::c_char,
                    &mut (*args_info).quiet_given,
                    &mut local_args_info.quiet_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_NO,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"quiet\0" as *const u8 as *const libc::c_char,
                    'q' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___26 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            67 => {
                tmp___27 = update_arg(
                    &mut (*args_info).config_arg as *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*args_info).config_orig,
                    &mut (*args_info).config_given,
                    &mut local_args_info.config_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    b"/etc/zmap/zmap.conf\0" as *const u8 as *const libc::c_char,
                    ARG_STRING,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"config\0" as *const u8 as *const libc::c_char,
                    'C' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___27 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            84 => {
                tmp___28 = update_arg(
                    &mut (*args_info).sender_threads_arg as *mut libc::c_int
                        as *mut libc::c_void,
                    &mut (*args_info).sender_threads_orig,
                    &mut (*args_info).sender_threads_given,
                    &mut local_args_info.sender_threads_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    b"1\0" as *const u8 as *const libc::c_char,
                    ARG_INT,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"sender-threads\0" as *const u8 as *const libc::c_char,
                    'T' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___28 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            104 => {
                tmp___29 = update_arg(
                    0 as *mut libc::c_void,
                    0 as *mut *mut libc::c_char,
                    &mut (*args_info).help_given,
                    &mut local_args_info.help_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_NO,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"help\0" as *const u8 as *const libc::c_char,
                    'h' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___29 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            86 => {
                tmp___30 = update_arg(
                    0 as *mut libc::c_void,
                    0 as *mut *mut libc::c_char,
                    &mut (*args_info).version_given,
                    &mut local_args_info.version_given,
                    optarg,
                    0 as *mut *const libc::c_char,
                    0 as *const libc::c_char,
                    ARG_NO,
                    check_ambiguity,
                    override_0,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"version\0" as *const u8 as *const libc::c_char,
                    'V' as i32 as libc::c_char,
                    additional_error,
                );
                if tmp___30 != 0 {
                    current_block = 7544259138455594556;
                    break;
                }
            }
            0 => {
                tmp___70 = strcmp(
                    long_options[option_index as usize].name,
                    b"batch\0" as *const u8 as *const libc::c_char,
                );
                if tmp___70 == 0 as libc::c_int {
                    tmp___31 = update_arg(
                        &mut (*args_info).batch_arg as *mut libc::c_int
                            as *mut libc::c_void,
                        &mut (*args_info).batch_orig,
                        &mut (*args_info).batch_given,
                        &mut local_args_info.batch_given,
                        optarg,
                        0 as *mut *const libc::c_char,
                        0 as *const libc::c_char,
                        ARG_INT,
                        check_ambiguity,
                        override_0,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        b"batch\0" as *const u8 as *const libc::c_char,
                        '-' as i32 as libc::c_char,
                        additional_error,
                    );
                    if tmp___31 != 0 {
                        current_block = 7544259138455594556;
                        break;
                    }
                } else {
                    tmp___69 = strcmp(
                        long_options[option_index as usize].name,
                        b"retries\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___69 == 0 as libc::c_int {
                        tmp___32 = update_arg(
                            &mut (*args_info).retries_arg as *mut libc::c_int
                                as *mut libc::c_void,
                            &mut (*args_info).retries_orig,
                            &mut (*args_info).retries_given,
                            &mut local_args_info.retries_given,
                            optarg,
                            0 as *mut *const libc::c_char,
                            b"10\0" as *const u8 as *const libc::c_char,
                            ARG_INT,
                            check_ambiguity,
                            override_0,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            b"retries\0" as *const u8 as *const libc::c_char,
                            '-' as i32 as libc::c_char,
                            additional_error,
                        );
                        if tmp___32 != 0 {
                            current_block = 7544259138455594556;
                            break;
                        }
                    } else {
                        tmp___68 = strcmp(
                            long_options[option_index as usize].name,
                            b"shards\0" as *const u8 as *const libc::c_char,
                        );
                        if tmp___68 == 0 as libc::c_int {
                            tmp___33 = update_arg(
                                &mut (*args_info).shards_arg as *mut libc::c_int
                                    as *mut libc::c_void,
                                &mut (*args_info).shards_orig,
                                &mut (*args_info).shards_given,
                                &mut local_args_info.shards_given,
                                optarg,
                                0 as *mut *const libc::c_char,
                                b"1\0" as *const u8 as *const libc::c_char,
                                ARG_INT,
                                check_ambiguity,
                                override_0,
                                0 as libc::c_int,
                                0 as libc::c_int,
                                b"shards\0" as *const u8 as *const libc::c_char,
                                '-' as i32 as libc::c_char,
                                additional_error,
                            );
                            if tmp___33 != 0 {
                                current_block = 7544259138455594556;
                                break;
                            }
                        } else {
                            tmp___67 = strcmp(
                                long_options[option_index as usize].name,
                                b"shard\0" as *const u8 as *const libc::c_char,
                            );
                            if tmp___67 == 0 as libc::c_int {
                                tmp___34 = update_arg(
                                    &mut (*args_info).shard_arg as *mut libc::c_int
                                        as *mut libc::c_void,
                                    &mut (*args_info).shard_orig,
                                    &mut (*args_info).shard_given,
                                    &mut local_args_info.shard_given,
                                    optarg,
                                    0 as *mut *const libc::c_char,
                                    b"0\0" as *const u8 as *const libc::c_char,
                                    ARG_INT,
                                    check_ambiguity,
                                    override_0,
                                    0 as libc::c_int,
                                    0 as libc::c_int,
                                    b"shard\0" as *const u8 as *const libc::c_char,
                                    '-' as i32 as libc::c_char,
                                    additional_error,
                                );
                                if tmp___34 != 0 {
                                    current_block = 7544259138455594556;
                                    break;
                                }
                            } else {
                                tmp___66 = strcmp(
                                    long_options[option_index as usize].name,
                                    b"source-mac\0" as *const u8 as *const libc::c_char,
                                );
                                if tmp___66 == 0 as libc::c_int {
                                    tmp___35 = update_arg(
                                        &mut (*args_info).source_mac_arg as *mut *mut libc::c_char
                                            as *mut libc::c_void,
                                        &mut (*args_info).source_mac_orig,
                                        &mut (*args_info).source_mac_given,
                                        &mut local_args_info.source_mac_given,
                                        optarg,
                                        0 as *mut *const libc::c_char,
                                        0 as *const libc::c_char,
                                        ARG_STRING,
                                        check_ambiguity,
                                        override_0,
                                        0 as libc::c_int,
                                        0 as libc::c_int,
                                        b"source-mac\0" as *const u8 as *const libc::c_char,
                                        '-' as i32 as libc::c_char,
                                        additional_error,
                                    );
                                    if tmp___35 != 0 {
                                        current_block = 7544259138455594556;
                                        break;
                                    }
                                } else {
                                    tmp___65 = strcmp(
                                        long_options[option_index as usize].name,
                                        b"probe-args\0" as *const u8 as *const libc::c_char,
                                    );
                                    if tmp___65 == 0 as libc::c_int {
                                        tmp___36 = update_arg(
                                            &mut (*args_info).probe_args_arg as *mut *mut libc::c_char
                                                as *mut libc::c_void,
                                            &mut (*args_info).probe_args_orig,
                                            &mut (*args_info).probe_args_given,
                                            &mut local_args_info.probe_args_given,
                                            optarg,
                                            0 as *mut *const libc::c_char,
                                            0 as *const libc::c_char,
                                            ARG_STRING,
                                            check_ambiguity,
                                            override_0,
                                            0 as libc::c_int,
                                            0 as libc::c_int,
                                            b"probe-args\0" as *const u8 as *const libc::c_char,
                                            '-' as i32 as libc::c_char,
                                            additional_error,
                                        );
                                        if tmp___36 != 0 {
                                            current_block = 7544259138455594556;
                                            break;
                                        }
                                    } else {
                                        tmp___64 = strcmp(
                                            long_options[option_index as usize].name,
                                            b"probe-ttl\0" as *const u8 as *const libc::c_char,
                                        );
                                        if tmp___64 == 0 as libc::c_int {
                                            tmp___37 = update_arg(
                                                &mut (*args_info).probe_ttl_arg as *mut libc::c_int
                                                    as *mut libc::c_void,
                                                &mut (*args_info).probe_ttl_orig,
                                                &mut (*args_info).probe_ttl_given,
                                                &mut local_args_info.probe_ttl_given,
                                                optarg,
                                                0 as *mut *const libc::c_char,
                                                b"255\0" as *const u8 as *const libc::c_char,
                                                ARG_INT,
                                                check_ambiguity,
                                                override_0,
                                                0 as libc::c_int,
                                                0 as libc::c_int,
                                                b"probe-ttl\0" as *const u8 as *const libc::c_char,
                                                '-' as i32 as libc::c_char,
                                                additional_error,
                                            );
                                            if tmp___37 != 0 {
                                                current_block = 7544259138455594556;
                                                break;
                                            }
                                        } else {
                                            tmp___63 = strcmp(
                                                long_options[option_index as usize].name,
                                                b"list-probe-modules\0" as *const u8 as *const libc::c_char,
                                            );
                                            if tmp___63 == 0 as libc::c_int {
                                                tmp___38 = update_arg(
                                                    0 as *mut libc::c_void,
                                                    0 as *mut *mut libc::c_char,
                                                    &mut (*args_info).list_probe_modules_given,
                                                    &mut local_args_info.list_probe_modules_given,
                                                    optarg,
                                                    0 as *mut *const libc::c_char,
                                                    0 as *const libc::c_char,
                                                    ARG_NO,
                                                    check_ambiguity,
                                                    override_0,
                                                    0 as libc::c_int,
                                                    0 as libc::c_int,
                                                    b"list-probe-modules\0" as *const u8 as *const libc::c_char,
                                                    '-' as i32 as libc::c_char,
                                                    additional_error,
                                                );
                                                if tmp___38 != 0 {
                                                    current_block = 7544259138455594556;
                                                    break;
                                                }
                                            } else {
                                                tmp___62 = strcmp(
                                                    long_options[option_index as usize].name,
                                                    b"output-args\0" as *const u8 as *const libc::c_char,
                                                );
                                                if tmp___62 == 0 as libc::c_int {
                                                    tmp___39 = update_arg(
                                                        &mut (*args_info).output_args_arg as *mut *mut libc::c_char
                                                            as *mut libc::c_void,
                                                        &mut (*args_info).output_args_orig,
                                                        &mut (*args_info).output_args_given,
                                                        &mut local_args_info.output_args_given,
                                                        optarg,
                                                        0 as *mut *const libc::c_char,
                                                        0 as *const libc::c_char,
                                                        ARG_STRING,
                                                        check_ambiguity,
                                                        override_0,
                                                        0 as libc::c_int,
                                                        0 as libc::c_int,
                                                        b"output-args\0" as *const u8 as *const libc::c_char,
                                                        '-' as i32 as libc::c_char,
                                                        additional_error,
                                                    );
                                                    if tmp___39 != 0 {
                                                        current_block = 7544259138455594556;
                                                        break;
                                                    }
                                                } else {
                                                    tmp___61 = strcmp(
                                                        long_options[option_index as usize].name,
                                                        b"output-filter\0" as *const u8 as *const libc::c_char,
                                                    );
                                                    if tmp___61 == 0 as libc::c_int {
                                                        tmp___40 = update_arg(
                                                            &mut (*args_info).output_filter_arg
                                                                as *mut *mut libc::c_char as *mut libc::c_void,
                                                            &mut (*args_info).output_filter_orig,
                                                            &mut (*args_info).output_filter_given,
                                                            &mut local_args_info.output_filter_given,
                                                            optarg,
                                                            0 as *mut *const libc::c_char,
                                                            0 as *const libc::c_char,
                                                            ARG_STRING,
                                                            check_ambiguity,
                                                            override_0,
                                                            0 as libc::c_int,
                                                            0 as libc::c_int,
                                                            b"output-filter\0" as *const u8 as *const libc::c_char,
                                                            '-' as i32 as libc::c_char,
                                                            additional_error,
                                                        );
                                                        if tmp___40 != 0 {
                                                            current_block = 7544259138455594556;
                                                            break;
                                                        }
                                                    } else {
                                                        tmp___60 = strcmp(
                                                            long_options[option_index as usize].name,
                                                            b"list-output-modules\0" as *const u8 as *const libc::c_char,
                                                        );
                                                        if tmp___60 == 0 as libc::c_int {
                                                            tmp___41 = update_arg(
                                                                0 as *mut libc::c_void,
                                                                0 as *mut *mut libc::c_char,
                                                                &mut (*args_info).list_output_modules_given,
                                                                &mut local_args_info.list_output_modules_given,
                                                                optarg,
                                                                0 as *mut *const libc::c_char,
                                                                0 as *const libc::c_char,
                                                                ARG_NO,
                                                                check_ambiguity,
                                                                override_0,
                                                                0 as libc::c_int,
                                                                0 as libc::c_int,
                                                                b"list-output-modules\0" as *const u8
                                                                    as *const libc::c_char,
                                                                '-' as i32 as libc::c_char,
                                                                additional_error,
                                                            );
                                                            if tmp___41 != 0 {
                                                                current_block = 7544259138455594556;
                                                                break;
                                                            }
                                                        } else {
                                                            tmp___59 = strcmp(
                                                                long_options[option_index as usize].name,
                                                                b"list-output-fields\0" as *const u8 as *const libc::c_char,
                                                            );
                                                            if tmp___59 == 0 as libc::c_int {
                                                                tmp___42 = update_arg(
                                                                    0 as *mut libc::c_void,
                                                                    0 as *mut *mut libc::c_char,
                                                                    &mut (*args_info).list_output_fields_given,
                                                                    &mut local_args_info.list_output_fields_given,
                                                                    optarg,
                                                                    0 as *mut *const libc::c_char,
                                                                    0 as *const libc::c_char,
                                                                    ARG_NO,
                                                                    check_ambiguity,
                                                                    override_0,
                                                                    0 as libc::c_int,
                                                                    0 as libc::c_int,
                                                                    b"list-output-fields\0" as *const u8 as *const libc::c_char,
                                                                    '-' as i32 as libc::c_char,
                                                                    additional_error,
                                                                );
                                                                if tmp___42 != 0 {
                                                                    current_block = 7544259138455594556;
                                                                    break;
                                                                }
                                                            } else {
                                                                tmp___58 = strcmp(
                                                                    long_options[option_index as usize].name,
                                                                    b"no-header-row\0" as *const u8 as *const libc::c_char,
                                                                );
                                                                if tmp___58 == 0 as libc::c_int {
                                                                    tmp___43 = update_arg(
                                                                        0 as *mut libc::c_void,
                                                                        0 as *mut *mut libc::c_char,
                                                                        &mut (*args_info).no_header_row_given,
                                                                        &mut local_args_info.no_header_row_given,
                                                                        optarg,
                                                                        0 as *mut *const libc::c_char,
                                                                        0 as *const libc::c_char,
                                                                        ARG_NO,
                                                                        check_ambiguity,
                                                                        override_0,
                                                                        0 as libc::c_int,
                                                                        0 as libc::c_int,
                                                                        b"no-header-row\0" as *const u8 as *const libc::c_char,
                                                                        '-' as i32 as libc::c_char,
                                                                        additional_error,
                                                                    );
                                                                    if tmp___43 != 0 {
                                                                        current_block = 7544259138455594556;
                                                                        break;
                                                                    }
                                                                } else {
                                                                    tmp___57 = strcmp(
                                                                        long_options[option_index as usize].name,
                                                                        b"disable-syslog\0" as *const u8 as *const libc::c_char,
                                                                    );
                                                                    if tmp___57 == 0 as libc::c_int {
                                                                        tmp___44 = update_arg(
                                                                            0 as *mut libc::c_void,
                                                                            0 as *mut *mut libc::c_char,
                                                                            &mut (*args_info).disable_syslog_given,
                                                                            &mut local_args_info.disable_syslog_given,
                                                                            optarg,
                                                                            0 as *mut *const libc::c_char,
                                                                            0 as *const libc::c_char,
                                                                            ARG_NO,
                                                                            check_ambiguity,
                                                                            override_0,
                                                                            0 as libc::c_int,
                                                                            0 as libc::c_int,
                                                                            b"disable-syslog\0" as *const u8 as *const libc::c_char,
                                                                            '-' as i32 as libc::c_char,
                                                                            additional_error,
                                                                        );
                                                                        if tmp___44 != 0 {
                                                                            current_block = 7544259138455594556;
                                                                            break;
                                                                        }
                                                                    } else {
                                                                        tmp___56 = strcmp(
                                                                            long_options[option_index as usize].name,
                                                                            b"notes\0" as *const u8 as *const libc::c_char,
                                                                        );
                                                                        if tmp___56 == 0 as libc::c_int {
                                                                            tmp___45 = update_arg(
                                                                                &mut (*args_info).notes_arg as *mut *mut libc::c_char
                                                                                    as *mut libc::c_void,
                                                                                &mut (*args_info).notes_orig,
                                                                                &mut (*args_info).notes_given,
                                                                                &mut local_args_info.notes_given,
                                                                                optarg,
                                                                                0 as *mut *const libc::c_char,
                                                                                0 as *const libc::c_char,
                                                                                ARG_STRING,
                                                                                check_ambiguity,
                                                                                override_0,
                                                                                0 as libc::c_int,
                                                                                0 as libc::c_int,
                                                                                b"notes\0" as *const u8 as *const libc::c_char,
                                                                                '-' as i32 as libc::c_char,
                                                                                additional_error,
                                                                            );
                                                                            if tmp___45 != 0 {
                                                                                current_block = 7544259138455594556;
                                                                                break;
                                                                            }
                                                                        } else {
                                                                            tmp___55 = strcmp(
                                                                                long_options[option_index as usize].name,
                                                                                b"user-metadata\0" as *const u8 as *const libc::c_char,
                                                                            );
                                                                            if tmp___55 == 0 as libc::c_int {
                                                                                tmp___46 = update_arg(
                                                                                    &mut (*args_info).user_metadata_arg
                                                                                        as *mut *mut libc::c_char as *mut libc::c_void,
                                                                                    &mut (*args_info).user_metadata_orig,
                                                                                    &mut (*args_info).user_metadata_given,
                                                                                    &mut local_args_info.user_metadata_given,
                                                                                    optarg,
                                                                                    0 as *mut *const libc::c_char,
                                                                                    0 as *const libc::c_char,
                                                                                    ARG_STRING,
                                                                                    check_ambiguity,
                                                                                    override_0,
                                                                                    0 as libc::c_int,
                                                                                    0 as libc::c_int,
                                                                                    b"user-metadata\0" as *const u8 as *const libc::c_char,
                                                                                    '-' as i32 as libc::c_char,
                                                                                    additional_error,
                                                                                );
                                                                                if tmp___46 != 0 {
                                                                                    current_block = 7544259138455594556;
                                                                                    break;
                                                                                }
                                                                            } else {
                                                                                tmp___54 = strcmp(
                                                                                    long_options[option_index as usize].name,
                                                                                    b"max-sendto-failures\0" as *const u8 as *const libc::c_char,
                                                                                );
                                                                                if tmp___54 == 0 as libc::c_int {
                                                                                    tmp___47 = update_arg(
                                                                                        &mut (*args_info).max_sendto_failures_arg
                                                                                            as *mut libc::c_int as *mut libc::c_void,
                                                                                        &mut (*args_info).max_sendto_failures_orig,
                                                                                        &mut (*args_info).max_sendto_failures_given,
                                                                                        &mut local_args_info.max_sendto_failures_given,
                                                                                        optarg,
                                                                                        0 as *mut *const libc::c_char,
                                                                                        b"-1\0" as *const u8 as *const libc::c_char,
                                                                                        ARG_INT,
                                                                                        check_ambiguity,
                                                                                        override_0,
                                                                                        0 as libc::c_int,
                                                                                        0 as libc::c_int,
                                                                                        b"max-sendto-failures\0" as *const u8
                                                                                            as *const libc::c_char,
                                                                                        '-' as i32 as libc::c_char,
                                                                                        additional_error,
                                                                                    );
                                                                                    if tmp___47 != 0 {
                                                                                        current_block = 7544259138455594556;
                                                                                        break;
                                                                                    }
                                                                                } else {
                                                                                    tmp___53 = strcmp(
                                                                                        long_options[option_index as usize].name,
                                                                                        b"min-hitrate\0" as *const u8 as *const libc::c_char,
                                                                                    );
                                                                                    if tmp___53 == 0 as libc::c_int {
                                                                                        tmp___48 = update_arg(
                                                                                            &mut (*args_info).min_hitrate_arg as *mut libc::c_float
                                                                                                as *mut libc::c_void,
                                                                                            &mut (*args_info).min_hitrate_orig,
                                                                                            &mut (*args_info).min_hitrate_given,
                                                                                            &mut local_args_info.min_hitrate_given,
                                                                                            optarg,
                                                                                            0 as *mut *const libc::c_char,
                                                                                            b"0.0\0" as *const u8 as *const libc::c_char,
                                                                                            ARG_FLOAT,
                                                                                            check_ambiguity,
                                                                                            override_0,
                                                                                            0 as libc::c_int,
                                                                                            0 as libc::c_int,
                                                                                            b"min-hitrate\0" as *const u8 as *const libc::c_char,
                                                                                            '-' as i32 as libc::c_char,
                                                                                            additional_error,
                                                                                        );
                                                                                        if tmp___48 != 0 {
                                                                                            current_block = 7544259138455594556;
                                                                                            break;
                                                                                        }
                                                                                    } else {
                                                                                        tmp___52 = strcmp(
                                                                                            long_options[option_index as usize].name,
                                                                                            b"cores\0" as *const u8 as *const libc::c_char,
                                                                                        );
                                                                                        if tmp___52 == 0 as libc::c_int {
                                                                                            tmp___49 = update_arg(
                                                                                                &mut (*args_info).cores_arg as *mut *mut libc::c_char
                                                                                                    as *mut libc::c_void,
                                                                                                &mut (*args_info).cores_orig,
                                                                                                &mut (*args_info).cores_given,
                                                                                                &mut local_args_info.cores_given,
                                                                                                optarg,
                                                                                                0 as *mut *const libc::c_char,
                                                                                                0 as *const libc::c_char,
                                                                                                ARG_STRING,
                                                                                                check_ambiguity,
                                                                                                override_0,
                                                                                                0 as libc::c_int,
                                                                                                0 as libc::c_int,
                                                                                                b"cores\0" as *const u8 as *const libc::c_char,
                                                                                                '-' as i32 as libc::c_char,
                                                                                                additional_error,
                                                                                            );
                                                                                            if tmp___49 != 0 {
                                                                                                current_block = 7544259138455594556;
                                                                                                break;
                                                                                            }
                                                                                        } else {
                                                                                            tmp___51 = strcmp(
                                                                                                long_options[option_index as usize].name,
                                                                                                b"ignore-blocklist-errors\0" as *const u8
                                                                                                    as *const libc::c_char,
                                                                                            );
                                                                                            if !(tmp___51 == 0 as libc::c_int) {
                                                                                                continue;
                                                                                            }
                                                                                            tmp___50 = update_arg(
                                                                                                0 as *mut libc::c_void,
                                                                                                0 as *mut *mut libc::c_char,
                                                                                                &mut (*args_info).ignore_blocklist_errors_given,
                                                                                                &mut local_args_info.ignore_blocklist_errors_given,
                                                                                                optarg,
                                                                                                0 as *mut *const libc::c_char,
                                                                                                0 as *const libc::c_char,
                                                                                                ARG_NO,
                                                                                                check_ambiguity,
                                                                                                override_0,
                                                                                                0 as libc::c_int,
                                                                                                0 as libc::c_int,
                                                                                                b"ignore-blocklist-errors\0" as *const u8
                                                                                                    as *const libc::c_char,
                                                                                                '-' as i32 as libc::c_char,
                                                                                                additional_error,
                                                                                            );
                                                                                            if tmp___50 != 0 {
                                                                                                current_block = 7544259138455594556;
                                                                                                break;
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
            63 => {
                current_block = 7544259138455594556;
                break;
            }
            _ => {
                if !additional_error.is_null() {
                    tmp___71 = additional_error;
                } else {
                    tmp___71 = b"\0" as *const u8 as *const libc::c_char;
                }
                __fprintf_chk(
                    stderr,
                    1 as libc::c_int,
                    b"%s: option unknown: %c%s\n\0" as *const u8 as *const libc::c_char,
                    b"zmap\0" as *const u8 as *const libc::c_char,
                    c,
                    tmp___71,
                );
                abort();
            }
        }
    }
    match current_block {
        7544259138455594556 => {
            cmdline_parser_release(&mut local_args_info);
            return 1 as libc::c_int;
        }
        _ => {
            cmdline_parser_release(&mut local_args_info);
            if error_occurred != 0 {
                return 1 as libc::c_int;
            }
            if optind < argc {
                i = 0 as libc::c_int;
                found_prog_name = 0 as libc::c_int;
                i = optind;
                while i < argc {
                    tmp___72 = i;
                    i += 1;
                    if !(*argv.offset(tmp___72 as isize) as libc::c_ulong
                        == *argv.offset(0 as libc::c_int as isize) as libc::c_ulong)
                    {
                        continue;
                    }
                    found_prog_name = 1 as libc::c_int;
                    break;
                }
                i = 0 as libc::c_int;
                (*args_info)
                    .inputs_num = (argc - optind - found_prog_name) as libc::c_uint;
                tmp___73 = malloc(
                    ((*args_info).inputs_num as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        ),
                );
                (*args_info).inputs = tmp___73 as *mut *mut libc::c_char;
                while optind < argc {
                    tmp___75 = optind;
                    optind += 1;
                    if *argv.offset(tmp___75 as isize) as libc::c_ulong
                        != *argv.offset(0 as libc::c_int as isize) as libc::c_ulong
                    {
                        tmp___74 = i;
                        i += 1;
                        let ref mut fresh0 = *((*args_info).inputs)
                            .offset(tmp___74 as isize);
                        *fresh0 = gengetopt_strdup(
                            *argv.offset((optind - 1 as libc::c_int) as isize)
                                as *const libc::c_char,
                        );
                    }
                }
            }
            return 0 as libc::c_int;
        }
    };
}
unsafe extern "C" fn _cmdline_parser_configfile(
    mut filename: *const libc::c_char,
    mut my_argc: *mut libc::c_int,
) -> libc::c_int {
    let mut file___1: *mut FILE = 0 as *mut FILE;
    let mut my_argv: [libc::c_char; 2052] = [0; 2052];
    let mut linebuf: [libc::c_char; 2048] = [0; 2048];
    let mut line_num: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    let mut equal: libc::c_int = 0;
    let mut fopt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut farg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str_index: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut next_token: size_t = 0;
    let mut delimiter: libc::c_char = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: size_t = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___6: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___7: *mut libc::c_char = 0 as *mut libc::c_char;
    line_num = 0 as libc::c_int;
    result = 0 as libc::c_int;
    file___1 = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    if file___1 as libc::c_ulong == 0 as *mut FILE as libc::c_ulong {
        __fprintf_chk(
            stderr,
            1 as libc::c_int,
            b"%s: Error opening configuration file '%s'\n\0" as *const u8
                as *const libc::c_char,
            b"zmap\0" as *const u8 as *const libc::c_char,
            filename,
        );
        return 1 as libc::c_int;
    }
    let mut current_block_86: u64;
    loop {
        tmp___7 = fgets(linebuf.as_mut_ptr(), 2048 as libc::c_int, file___1);
        if !(tmp___7 as libc::c_ulong != 0 as *mut libc::c_char as libc::c_ulong) {
            break;
        }
        line_num += 1;
        my_argv[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        len = strlen(linebuf.as_mut_ptr() as *const libc::c_char);
        if len > 2050 as libc::c_ulong {
            __fprintf_chk(
                stderr,
                1 as libc::c_int,
                b"%s:%s:%d: Line too long in configuration file\n\0" as *const u8
                    as *const libc::c_char,
                b"zmap\0" as *const u8 as *const libc::c_char,
                filename,
                line_num,
            );
            result = 1 as libc::c_int;
            break;
        } else {
            next_token = strspn(
                linebuf.as_mut_ptr() as *const libc::c_char,
                b" \t\r\n\0" as *const u8 as *const libc::c_char,
            );
            str_index = linebuf.as_mut_ptr().offset(next_token as isize);
            if *str_index.offset(0 as libc::c_int as isize) as libc::c_int
                == 0 as libc::c_int
            {
                continue;
            }
            if *str_index.offset(0 as libc::c_int as isize) as libc::c_int
                == 35 as libc::c_int
            {
                continue;
            }
            fopt = str_index;
            next_token = strcspn(
                fopt as *const libc::c_char,
                b" \t\r\n=\0" as *const u8 as *const libc::c_char,
            );
            if *fopt.offset(next_token as isize) as libc::c_int == 0 as libc::c_int {
                farg = 0 as *mut libc::c_char;
                equal = 0 as libc::c_int;
            } else {
                equal = (*fopt.offset(next_token as isize) as libc::c_int
                    == 61 as libc::c_int) as libc::c_int;
                tmp = next_token;
                next_token = next_token.wrapping_add(1);
                *fopt.offset(tmp as isize) = '\u{0}' as i32 as libc::c_char;
                tmp___0 = strspn(
                    fopt.offset(next_token as isize) as *const libc::c_char,
                    b" \t\r\n\0" as *const u8 as *const libc::c_char,
                );
                next_token = (next_token as libc::c_ulong).wrapping_add(tmp___0)
                    as size_t as size_t;
                if equal == 0 {
                    equal = (*fopt.offset(next_token as isize) as libc::c_int
                        == 61 as libc::c_int) as libc::c_int;
                    if equal != 0 {
                        next_token = next_token.wrapping_add(1);
                        tmp___1 = strspn(
                            fopt.offset(next_token as isize) as *const libc::c_char,
                            b" \t\r\n\0" as *const u8 as *const libc::c_char,
                        );
                        next_token = (next_token as libc::c_ulong).wrapping_add(tmp___1)
                            as size_t as size_t;
                    }
                }
                str_index = str_index.offset(next_token as isize);
                farg = str_index;
                if *farg.offset(0 as libc::c_int as isize) as libc::c_int
                    == 34 as libc::c_int
                {
                    current_block_86 = 1751906698440020857;
                } else if *farg.offset(0 as libc::c_int as isize) as libc::c_int
                        == 39 as libc::c_int
                    {
                    current_block_86 = 1751906698440020857;
                } else {
                    next_token = strcspn(
                        farg as *const libc::c_char,
                        b" \t\r\n#'\"\0" as *const u8 as *const libc::c_char,
                    );
                    str_index = str_index.offset(next_token as isize);
                    current_block_86 = 11763295167351361500;
                }
                match current_block_86 {
                    1751906698440020857 => {
                        farg = farg.offset(1);
                        str_index = strchr(
                            farg as *const libc::c_char,
                            *str_index.offset(0 as libc::c_int as isize) as libc::c_int,
                        );
                        if str_index.is_null() {
                            __fprintf_chk(
                                stderr,
                                1 as libc::c_int,
                                b"%s:%s:%d: unterminated string in configuration file\n\0"
                                    as *const u8 as *const libc::c_char,
                                b"zmap\0" as *const u8 as *const libc::c_char,
                                filename,
                                line_num,
                            );
                            result = 1 as libc::c_int;
                            break;
                        }
                    }
                    _ => {}
                }
                delimiter = *str_index;
                tmp___2 = str_index;
                str_index = str_index.offset(1);
                *tmp___2 = '\u{0}' as i32 as libc::c_char;
                if delimiter as libc::c_int != 0 as libc::c_int {
                    if delimiter as libc::c_int != 35 as libc::c_int {
                        tmp___3 = strspn(
                            str_index as *const libc::c_char,
                            b" \t\r\n\0" as *const u8 as *const libc::c_char,
                        );
                        str_index = str_index.offset(tmp___3 as isize);
                        if *str_index as libc::c_int != 0 as libc::c_int {
                            if *str_index as libc::c_int != 35 as libc::c_int {
                                __fprintf_chk(
                                    stderr,
                                    1 as libc::c_int,
                                    b"%s:%s:%d: malformed string in configuration file\n\0"
                                        as *const u8 as *const libc::c_char,
                                    b"zmap\0" as *const u8 as *const libc::c_char,
                                    filename,
                                    line_num,
                                );
                                result = 1 as libc::c_int;
                                break;
                            }
                        }
                    }
                }
            }
            tmp___4 = strcmp(
                fopt as *const libc::c_char,
                b"include\0" as *const u8 as *const libc::c_char,
            );
            if tmp___4 == 0 {
                if !farg.is_null() {
                    if *farg != 0 {
                        result = _cmdline_parser_configfile(
                            farg as *const libc::c_char,
                            my_argc,
                        );
                    } else {
                        __fprintf_chk(
                            stderr,
                            1 as libc::c_int,
                            b"%s:%s:%d: include requires a filename argument.\n\0"
                                as *const u8 as *const libc::c_char,
                            b"zmap\0" as *const u8 as *const libc::c_char,
                            filename,
                            line_num,
                        );
                    }
                } else {
                    __fprintf_chk(
                        stderr,
                        1 as libc::c_int,
                        b"%s:%s:%d: include requires a filename argument.\n\0"
                            as *const u8 as *const libc::c_char,
                        b"zmap\0" as *const u8 as *const libc::c_char,
                        filename,
                        line_num,
                    );
                }
            } else {
                len = strlen(fopt as *const libc::c_char);
                if len > 1 as libc::c_ulong {
                    tmp___5 = b"--\0" as *const u8 as *const libc::c_char;
                } else {
                    tmp___5 = b"-\0" as *const u8 as *const libc::c_char;
                }
                strcat(my_argv.as_mut_ptr(), tmp___5);
                strcat(my_argv.as_mut_ptr(), fopt as *const libc::c_char);
                if len > 1 as libc::c_ulong {
                    let mut current_block_72: u64;
                    if !farg.is_null() {
                        if *farg != 0 {
                            strcat(
                                my_argv.as_mut_ptr(),
                                b"=\0" as *const u8 as *const libc::c_char,
                            );
                            current_block_72 = 9705665520141849625;
                        } else {
                            current_block_72 = 14390628605638734663;
                        }
                    } else {
                        current_block_72 = 14390628605638734663;
                    }
                    match current_block_72 {
                        14390628605638734663 => {
                            if equal != 0 {
                                strcat(
                                    my_argv.as_mut_ptr(),
                                    b"=\0" as *const u8 as *const libc::c_char,
                                );
                            }
                        }
                        _ => {}
                    }
                }
                if !farg.is_null() {
                    if *farg != 0 {
                        strcat(my_argv.as_mut_ptr(), farg as *const libc::c_char);
                    }
                }
                *my_argc += 1;
                tmp___6 = malloc(::std::mem::size_of::<line_list>() as libc::c_ulong);
                cmd_line_list_tmp = tmp___6 as *mut line_list;
                (*cmd_line_list_tmp).next = cmd_line_list;
                cmd_line_list = cmd_line_list_tmp;
                (*cmd_line_list)
                    .string_arg = gengetopt_strdup(
                    my_argv.as_mut_ptr() as *const libc::c_char,
                );
            }
        }
    }
    if !file___1.is_null() {
        fclose(file___1);
    }
    return result;
}
pub unsafe extern "C" fn cmdline_parser_configfile(
    mut filename: *const libc::c_char,
    mut args_info: *mut gengetopt_args_info,
    mut override_0: libc::c_int,
    mut initialize: libc::c_int,
    mut check_required: libc::c_int,
) -> libc::c_int {
    let mut params: cmdline_parser_params = cmdline_parser_params {
        override_0: 0,
        initialize: 0,
        check_required: 0,
        check_ambiguity: 0,
        print_errors: 0,
    };
    let mut tmp: libc::c_int = 0;
    params.override_0 = override_0;
    params.initialize = initialize;
    params.check_required = check_required;
    params.check_ambiguity = 0 as libc::c_int;
    params.print_errors = 1 as libc::c_int;
    tmp = cmdline_parser_config_file(filename, args_info, &mut params);
    return tmp;
}
pub unsafe extern "C" fn cmdline_parser_config_file(
    mut filename: *const libc::c_char,
    mut args_info: *mut gengetopt_args_info,
    mut params: *mut cmdline_parser_params,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    let mut my_argc: libc::c_int = 0;
    let mut my_argv_arg: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut additional_error: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    my_argc = 1 as libc::c_int;
    tmp = malloc(::std::mem::size_of::<line_list>() as libc::c_ulong);
    cmd_line_list_tmp = tmp as *mut line_list;
    (*cmd_line_list_tmp).next = cmd_line_list;
    cmd_line_list = cmd_line_list_tmp;
    (*cmd_line_list)
        .string_arg = gengetopt_strdup(b"zmap\0" as *const u8 as *const libc::c_char);
    result = _cmdline_parser_configfile(filename, &mut my_argc);
    if result != 1 as libc::c_int {
        tmp___0 = malloc(
            ((my_argc + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        );
        my_argv_arg = tmp___0 as *mut *mut libc::c_char;
        cmd_line_list_tmp = cmd_line_list;
        i = my_argc - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            let ref mut fresh1 = *my_argv_arg.offset(i as isize);
            *fresh1 = (*cmd_line_list_tmp).string_arg;
            cmd_line_list_tmp = (*cmd_line_list_tmp).next;
            i -= 1;
        }
        let ref mut fresh2 = *my_argv_arg.offset(my_argc as isize);
        *fresh2 = 0 as *mut libc::c_char;
        tmp___1 = strlen(filename);
        tmp___2 = strlen(
            b" in configuration file \0" as *const u8 as *const libc::c_char,
        );
        tmp___3 = malloc(tmp___1.wrapping_add(tmp___2).wrapping_add(1 as libc::c_ulong));
        additional_error = tmp___3 as *mut libc::c_char;
        strcpy(
            additional_error,
            b" in configuration file \0" as *const u8 as *const libc::c_char,
        );
        strcat(additional_error, filename);
        result = cmdline_parser_internal(
            my_argc,
            my_argv_arg,
            args_info,
            params,
            additional_error as *const libc::c_char,
        );
        free(additional_error as *mut libc::c_void);
        free(my_argv_arg as *mut libc::c_void);
    }
    free_cmd_list();
    if result == 1 as libc::c_int {
        cmdline_parser_free(args_info);
        exit(1 as libc::c_int);
    }
    return result;
}
pub static mut yyleng: libc::c_int = 0;
static mut yy_buffer_stack_top: size_t = 0 as libc::c_int as size_t;
static mut yy_buffer_stack_max: size_t = 0 as libc::c_int as size_t;
static mut yy_buffer_stack: *mut YY_BUFFER_STATE = 0 as *const libc::c_void
    as *mut libc::c_void as *mut YY_BUFFER_STATE;
static mut yy_hold_char: libc::c_char = 0;
static mut yy_n_chars: libc::c_int = 0;
static mut yy_c_buf_p: *mut libc::c_char = 0 as *const libc::c_void as *mut libc::c_void
    as *mut libc::c_char;
static mut yy_init: libc::c_int = 0 as libc::c_int;
static mut yy_start: libc::c_int = 0 as libc::c_int;
static mut yy_did_buffer_switch_on_eof: libc::c_int = 0;
pub static mut yyin: *mut FILE = 0 as *const libc::c_void as *mut libc::c_void
    as *mut FILE;
pub static mut yyout: *mut FILE = 0 as *const libc::c_void as *mut libc::c_void
    as *mut FILE;
pub static mut yylineno: libc::c_int = 1 as libc::c_int;
pub static mut yytext: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut yy_accept: [flex_int16_t; 26] = [
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    13 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    9 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    7 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    8 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
];
static mut yy_ec: [YY_CHAR; 256] = [
    0 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    2 as libc::c_int as YY_CHAR,
    3 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    2 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    5 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    7 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    9 as libc::c_int as YY_CHAR,
    9 as libc::c_int as YY_CHAR,
    9 as libc::c_int as YY_CHAR,
    9 as libc::c_int as YY_CHAR,
    9 as libc::c_int as YY_CHAR,
    9 as libc::c_int as YY_CHAR,
    9 as libc::c_int as YY_CHAR,
    9 as libc::c_int as YY_CHAR,
    9 as libc::c_int as YY_CHAR,
    9 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    10 as libc::c_int as YY_CHAR,
    11 as libc::c_int as YY_CHAR,
    12 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    14 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
];
static mut yy_meta: [YY_CHAR; 15] = [
    0 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    2 as libc::c_int as YY_CHAR,
    2 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    2 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
];
static mut yy_base: [flex_int16_t; 27] = [
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    9 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    8 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    7 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    13 as libc::c_int as flex_int16_t,
];
static mut yy_def: [flex_int16_t; 27] = [
    0 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
];
static mut yy_nxt: [flex_int16_t; 41] = [
    0 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    7 as libc::c_int as flex_int16_t,
    8 as libc::c_int as flex_int16_t,
    9 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    13 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
];
static mut yy_chk: [flex_int16_t; 41] = [
    0 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    8 as libc::c_int as flex_int16_t,
    7 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
];
static mut yy_last_accepting_state: yy_state_type = 0;
static mut yy_last_accepting_cpos: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut yy_flex_debug: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn yylex() -> libc::c_int {
    let mut current_block: u64;
    let mut yy_current_state: yy_state_type = 0;
    let mut yy_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut yy_bp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut yy_act: libc::c_int = 0;
    let mut tmp: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    let mut yy_c: YY_CHAR = 0;
    let mut tmp___0: libc::c_longlong = 0;
    let mut yy_amount_of_matched_text: libc::c_int = 0;
    let mut yy_next_state: yy_state_type = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    if yy_init == 0 {
        yy_init = 1 as libc::c_int;
        if yy_start == 0 {
            yy_start = 1 as libc::c_int;
        }
        if yyin.is_null() {
            yyin = stdin;
        }
        if yyout.is_null() {
            yyout = stdout;
        }
        if !yy_buffer_stack.is_null() {
            tmp = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        } else {
            tmp = 0 as *mut libc::c_void as YY_BUFFER_STATE;
        }
        if tmp.is_null() {
            yyensure_buffer_stack();
            let ref mut fresh3 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
            *fresh3 = yy_create_buffer(yyin, 16384 as libc::c_int);
        }
        yy_load_buffer_state();
    }
    loop {
        yy_cp = yy_c_buf_p;
        *yy_cp = yy_hold_char;
        yy_bp = yy_cp;
        yy_current_state = yy_start;
        '_yy_match: loop {
            loop {
                yy_c = yy_ec[*yy_cp as YY_CHAR as usize];
                if yy_accept[yy_current_state as usize] != 0 {
                    yy_last_accepting_state = yy_current_state;
                    yy_last_accepting_cpos = yy_cp;
                }
                while yy_chk[(yy_base[yy_current_state as usize] as libc::c_int
                    + yy_c as libc::c_int) as usize] as libc::c_int != yy_current_state
                {
                    yy_current_state = yy_def[yy_current_state as usize] as libc::c_int;
                    if yy_current_state >= 26 as libc::c_int {
                        yy_c = yy_meta[yy_c as usize];
                    }
                }
                yy_current_state = yy_nxt[(yy_base[yy_current_state as usize]
                    as libc::c_int + yy_c as libc::c_int) as usize] as yy_state_type;
                yy_cp = yy_cp.offset(1);
                if !(yy_base[yy_current_state as usize] as libc::c_int
                    != 26 as libc::c_int)
                {
                    break;
                }
            }
            '_yy_find_action: loop {
                yy_act = yy_accept[yy_current_state as usize] as libc::c_int;
                if yy_act == 0 as libc::c_int {
                    yy_cp = yy_last_accepting_cpos;
                    yy_current_state = yy_last_accepting_state;
                    yy_act = yy_accept[yy_current_state as usize] as libc::c_int;
                }
                yytext = yy_bp;
                yyleng = yy_cp.offset_from(yy_bp) as libc::c_long as libc::c_int;
                yy_hold_char = *yy_cp;
                *yy_cp = '\u{0}' as i32 as libc::c_char;
                yy_c_buf_p = yy_cp;
                loop {
                    match yy_act {
                        0 => {
                            *yy_cp = yy_hold_char;
                            yy_cp = yy_last_accepting_cpos;
                            yy_current_state = yy_last_accepting_state;
                            continue '_yy_find_action;
                        }
                        1 => {
                            tmp___0 = atoll(yytext as *const libc::c_char);
                            yylval.int_literal = tmp___0 as uint64_t as libc::c_int;
                            return 259 as libc::c_int;
                        }
                        2 | 3 => {
                            break '_yy_match;
                        }
                        4 => return 261 as libc::c_int,
                        5 => return 262 as libc::c_int,
                        6 => return 263 as libc::c_int,
                        7 => return 257 as libc::c_int,
                        8 => return 258 as libc::c_int,
                        9 => return '=' as i32,
                        10 => return '>' as i32,
                        11 => return '<' as i32,
                        12 => return '(' as i32,
                        13 => return ')' as i32,
                        14 => {
                            yylval
                                .string_literal = strdup(yytext as *const libc::c_char);
                            return 260 as libc::c_int;
                        }
                        15 => {
                            fwrite(
                                yytext as *const libc::c_void,
                                yyleng as size_t,
                                1 as libc::c_int as size_t,
                                yyout,
                            );
                            break '_yy_match;
                        }
                        17 => return 0 as libc::c_int,
                        16 => {
                            yy_amount_of_matched_text = yy_cp.offset_from(yytext)
                                as libc::c_long as libc::c_int - 1 as libc::c_int;
                            *yy_cp = yy_hold_char;
                            if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                .yy_buffer_status == 0 as libc::c_int
                            {
                                yy_n_chars = (**yy_buffer_stack
                                    .offset(yy_buffer_stack_top as isize))
                                    .yy_n_chars;
                                let ref mut fresh4 = (**yy_buffer_stack
                                    .offset(yy_buffer_stack_top as isize))
                                    .yy_input_file;
                                *fresh4 = yyin;
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_buffer_status = 1 as libc::c_int;
                            }
                            if yy_c_buf_p as libc::c_ulong
                                <= ((**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_ch_buf)
                                    .offset(yy_n_chars as isize) as libc::c_ulong
                            {
                                yy_c_buf_p = yytext
                                    .offset(yy_amount_of_matched_text as isize);
                                yy_current_state = yy_get_previous_state();
                                yy_next_state = yy_try_NUL_trans(yy_current_state);
                                yy_bp = yytext.offset(0 as libc::c_int as isize);
                                if yy_next_state != 0 {
                                    current_block = 16231175055492490595;
                                    break;
                                } else {
                                    current_block = 6002151390280567665;
                                    break;
                                }
                            } else {
                                tmp___1 = yy_get_next_buffer();
                                match tmp___1 {
                                    1 => {
                                        yy_did_buffer_switch_on_eof = 0 as libc::c_int;
                                        tmp___2 = yywrap();
                                        if tmp___2 != 0 {
                                            yy_c_buf_p = yytext.offset(0 as libc::c_int as isize);
                                            yy_act = 16 as libc::c_int
                                                + (yy_start - 1 as libc::c_int) / 2 as libc::c_int
                                                + 1 as libc::c_int;
                                        } else {
                                            if yy_did_buffer_switch_on_eof == 0 {
                                                yyrestart(yyin);
                                            }
                                            break '_yy_match;
                                        }
                                    }
                                    0 => {
                                        yy_c_buf_p = yytext
                                            .offset(yy_amount_of_matched_text as isize);
                                        yy_current_state = yy_get_previous_state();
                                        yy_cp = yy_c_buf_p;
                                        yy_bp = yytext.offset(0 as libc::c_int as isize);
                                        break '_yy_find_action;
                                    }
                                    2 => {
                                        yy_c_buf_p = ((**yy_buffer_stack
                                            .offset(yy_buffer_stack_top as isize))
                                            .yy_ch_buf)
                                            .offset(yy_n_chars as isize);
                                        yy_current_state = yy_get_previous_state();
                                        yy_cp = yy_c_buf_p;
                                        yy_bp = yytext.offset(0 as libc::c_int as isize);
                                        continue '_yy_find_action;
                                    }
                                    _ => {
                                        break '_yy_match;
                                    }
                                }
                            }
                        }
                        _ => {
                            yy_fatal_error(
                                b"fatal flex scanner internal error--no action found\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                    }
                }
                match current_block {
                    6002151390280567665 => {
                        yy_cp = yy_c_buf_p;
                    }
                    _ => {
                        yy_c_buf_p = yy_c_buf_p.offset(1);
                        yy_cp = yy_c_buf_p;
                        yy_current_state = yy_next_state;
                        break;
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn yy_get_next_buffer() -> libc::c_int {
    let mut dest: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut source: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut number_to_move: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret_val: libc::c_int = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut num_to_read: libc::c_int = 0;
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    let mut yy_c_buf_p_offset: libc::c_int = 0;
    let mut new_size: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut c: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___5: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___6: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___7: size_t = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut new_size___0: libc::c_int = 0;
    let mut tmp___9: *mut libc::c_void = 0 as *mut libc::c_void;
    dest = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf;
    source = yytext;
    if yy_c_buf_p as libc::c_ulong
        > ((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
            .offset((yy_n_chars + 1 as libc::c_int) as isize) as libc::c_ulong
    {
        yy_fatal_error(
            b"fatal flex scanner internal error--end of buffer missed\0" as *const u8
                as *const libc::c_char,
        );
    }
    if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_fill_buffer
        == 0 as libc::c_int
    {
        if yy_c_buf_p.offset_from(yytext) as libc::c_long == 1 as libc::c_long {
            return 1 as libc::c_int
        } else {
            return 2 as libc::c_int
        }
    }
    number_to_move = (yy_c_buf_p.offset_from(yytext) as libc::c_long - 1 as libc::c_long)
        as libc::c_int;
    i = 0 as libc::c_int;
    while i < number_to_move {
        tmp = dest;
        dest = dest.offset(1);
        tmp___0 = source;
        source = source.offset(1);
        *tmp = *tmp___0;
        i += 1;
    }
    if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buffer_status
        == 2 as libc::c_int
    {
        yy_n_chars = 0 as libc::c_int;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    } else {
        num_to_read = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_buf_size - number_to_move - 1 as libc::c_int;
        while num_to_read <= 0 as libc::c_int {
            b = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
            yy_c_buf_p_offset = yy_c_buf_p.offset_from((*b).yy_ch_buf) as libc::c_long
                as libc::c_int;
            if (*b).yy_is_our_buffer != 0 {
                new_size = (*b).yy_buf_size * 2 as libc::c_int;
                if new_size <= 0 as libc::c_int {
                    (*b).yy_buf_size += (*b).yy_buf_size / 8 as libc::c_int;
                } else {
                    (*b).yy_buf_size *= 2 as libc::c_int;
                }
                tmp___1 = yyrealloc(
                    (*b).yy_ch_buf as *mut libc::c_void,
                    ((*b).yy_buf_size + 2 as libc::c_int) as yy_size_t,
                );
                (*b).yy_ch_buf = tmp___1 as *mut libc::c_char;
            } else {
                (*b).yy_ch_buf = 0 as *mut libc::c_void as *mut libc::c_char;
            }
            if ((*b).yy_ch_buf).is_null() {
                yy_fatal_error(
                    b"fatal error - scanner input buffer overflow\0" as *const u8
                        as *const libc::c_char,
                );
            }
            yy_c_buf_p = ((*b).yy_ch_buf).offset(yy_c_buf_p_offset as isize);
            num_to_read = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                .yy_buf_size - number_to_move - 1 as libc::c_int;
        }
        if num_to_read > 8192 as libc::c_int {
            num_to_read = 8192 as libc::c_int;
        }
        if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_is_interactive
            != 0
        {
            c = '*' as i32;
            n = 0 as libc::c_int;
            while n < num_to_read {
                c = getc(yyin);
                if !(c != -(1 as libc::c_int)) {
                    break;
                }
                if !(c != 10 as libc::c_int) {
                    break;
                }
                *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
                    .offset(number_to_move as isize)
                    .offset(n as isize) = c as libc::c_char;
                n += 1;
            }
            if c == 10 as libc::c_int {
                tmp___2 = n;
                n += 1;
                *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
                    .offset(number_to_move as isize)
                    .offset(tmp___2 as isize) = c as libc::c_char;
            }
            if c == -(1 as libc::c_int) {
                tmp___3 = ferror(yyin);
                if tmp___3 != 0 {
                    yy_fatal_error(
                        b"input in flex scanner failed\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
            yy_n_chars = n;
        } else {
            tmp___4 = __errno_location();
            *tmp___4 = 0 as libc::c_int;
            loop {
                tmp___7 = fread(
                    ((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
                        .offset(number_to_move as isize) as *mut libc::c_void,
                    1 as libc::c_int as size_t,
                    num_to_read as yy_size_t,
                    yyin,
                );
                yy_n_chars = tmp___7 as libc::c_int;
                if !(yy_n_chars == 0 as libc::c_int) {
                    break;
                }
                tmp___8 = ferror(yyin);
                if tmp___8 == 0 {
                    break;
                }
                tmp___5 = __errno_location();
                if *tmp___5 != 4 as libc::c_int {
                    yy_fatal_error(
                        b"input in flex scanner failed\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    tmp___6 = __errno_location();
                    *tmp___6 = 0 as libc::c_int;
                    clearerr(yyin);
                }
            }
        }
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    }
    if yy_n_chars == 0 as libc::c_int {
        if number_to_move == 0 as libc::c_int {
            ret_val = 1 as libc::c_int;
            yyrestart(yyin);
        } else {
            ret_val = 2 as libc::c_int;
            (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                .yy_buffer_status = 2 as libc::c_int;
        }
    } else {
        ret_val = 0 as libc::c_int;
    }
    if yy_n_chars + number_to_move
        > (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buf_size
    {
        new_size___0 = yy_n_chars + number_to_move + (yy_n_chars >> 1 as libc::c_int);
        tmp___9 = yyrealloc(
            (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf
                as *mut libc::c_void,
            new_size___0 as yy_size_t,
        );
        let ref mut fresh5 = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_ch_buf;
        *fresh5 = tmp___9 as *mut libc::c_char;
        if ((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf).is_null()
        {
            yy_fatal_error(
                b"out of dynamic memory in yy_get_next_buffer()\0" as *const u8
                    as *const libc::c_char,
            );
        }
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_buf_size = new_size___0 - 2 as libc::c_int;
    }
    yy_n_chars += number_to_move;
    *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
        .offset(yy_n_chars as isize) = 0 as libc::c_int as libc::c_char;
    *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
        .offset(
            (yy_n_chars + 1 as libc::c_int) as isize,
        ) = 0 as libc::c_int as libc::c_char;
    yytext = ((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
        .offset(0 as libc::c_int as isize);
    return ret_val;
}
unsafe extern "C" fn yy_get_previous_state() -> yy_state_type {
    let mut yy_current_state: yy_state_type = 0;
    let mut yy_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut yy_c: YY_CHAR = 0;
    let mut tmp: libc::c_int = 0;
    yy_current_state = yy_start;
    yy_cp = yytext.offset(0 as libc::c_int as isize);
    while (yy_cp as libc::c_ulong) < yy_c_buf_p as libc::c_ulong {
        if *yy_cp != 0 {
            tmp = yy_ec[*yy_cp as YY_CHAR as usize] as libc::c_int;
        } else {
            tmp = 1 as libc::c_int;
        }
        yy_c = tmp as YY_CHAR;
        if yy_accept[yy_current_state as usize] != 0 {
            yy_last_accepting_state = yy_current_state;
            yy_last_accepting_cpos = yy_cp;
        }
        while yy_chk[(yy_base[yy_current_state as usize] as libc::c_int
            + yy_c as libc::c_int) as usize] as libc::c_int != yy_current_state
        {
            yy_current_state = yy_def[yy_current_state as usize] as libc::c_int;
            if yy_current_state >= 26 as libc::c_int {
                yy_c = yy_meta[yy_c as usize];
            }
        }
        yy_current_state = yy_nxt[(yy_base[yy_current_state as usize] as libc::c_int
            + yy_c as libc::c_int) as usize] as yy_state_type;
        yy_cp = yy_cp.offset(1);
    }
    return yy_current_state;
}
unsafe extern "C" fn yy_try_NUL_trans(
    mut yy_current_state: yy_state_type,
) -> yy_state_type {
    let mut yy_is_jam: libc::c_int = 0;
    let mut yy_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut yy_c: YY_CHAR = 0;
    let mut tmp: libc::c_int = 0;
    yy_cp = yy_c_buf_p;
    yy_c = 1 as libc::c_int as YY_CHAR;
    if yy_accept[yy_current_state as usize] != 0 {
        yy_last_accepting_state = yy_current_state;
        yy_last_accepting_cpos = yy_cp;
    }
    while yy_chk[(yy_base[yy_current_state as usize] as libc::c_int
        + yy_c as libc::c_int) as usize] as libc::c_int != yy_current_state
    {
        yy_current_state = yy_def[yy_current_state as usize] as libc::c_int;
        if yy_current_state >= 26 as libc::c_int {
            yy_c = yy_meta[yy_c as usize];
        }
    }
    yy_current_state = yy_nxt[(yy_base[yy_current_state as usize] as libc::c_int
        + yy_c as libc::c_int) as usize] as yy_state_type;
    yy_is_jam = (yy_current_state == 25 as libc::c_int) as libc::c_int;
    if yy_is_jam != 0 {
        tmp = 0 as libc::c_int;
    } else {
        tmp = yy_current_state;
    }
    return tmp;
}
pub unsafe extern "C" fn yyrestart(mut input_file: *mut FILE) {
    let mut tmp: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    let mut tmp___0: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    if !yy_buffer_stack.is_null() {
        tmp = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    } else {
        tmp = 0 as *mut libc::c_void as YY_BUFFER_STATE;
    }
    if tmp.is_null() {
        yyensure_buffer_stack();
        let ref mut fresh6 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh6 = yy_create_buffer(yyin, 16384 as libc::c_int);
    }
    if !yy_buffer_stack.is_null() {
        tmp___0 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    } else {
        tmp___0 = 0 as *mut libc::c_void as YY_BUFFER_STATE;
    }
    yy_init_buffer(tmp___0, input_file);
    yy_load_buffer_state();
}
pub unsafe extern "C" fn yy_switch_to_buffer(mut new_buffer: YY_BUFFER_STATE) {
    let mut tmp: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    let mut tmp___0: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    yyensure_buffer_stack();
    if !yy_buffer_stack.is_null() {
        tmp = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    } else {
        tmp = 0 as *mut libc::c_void as YY_BUFFER_STATE;
    }
    if tmp as libc::c_ulong == new_buffer as libc::c_ulong {
        return;
    }
    if !yy_buffer_stack.is_null() {
        tmp___0 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    } else {
        tmp___0 = 0 as *mut libc::c_void as YY_BUFFER_STATE;
    }
    if !tmp___0.is_null() {
        *yy_c_buf_p = yy_hold_char;
        let ref mut fresh7 = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_buf_pos;
        *fresh7 = yy_c_buf_p;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    }
    let ref mut fresh8 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh8 = new_buffer;
    yy_load_buffer_state();
    yy_did_buffer_switch_on_eof = 1 as libc::c_int;
}
unsafe extern "C" fn yy_load_buffer_state() {
    yy_n_chars = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars;
    yy_c_buf_p = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buf_pos;
    yytext = yy_c_buf_p;
    yyin = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_input_file;
    yy_hold_char = *yy_c_buf_p;
}
pub unsafe extern "C" fn yy_create_buffer(
    mut file___1: *mut FILE,
    mut size: libc::c_int,
) -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = yyalloc(::std::mem::size_of::<yy_buffer_state>() as libc::c_ulong);
    b = tmp as YY_BUFFER_STATE;
    if b.is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_create_buffer()\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*b).yy_buf_size = size;
    tmp___0 = yyalloc(((*b).yy_buf_size + 2 as libc::c_int) as yy_size_t);
    (*b).yy_ch_buf = tmp___0 as *mut libc::c_char;
    if ((*b).yy_ch_buf).is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_create_buffer()\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*b).yy_is_our_buffer = 1 as libc::c_int;
    yy_init_buffer(b, file___1);
    return b;
}
pub unsafe extern "C" fn yy_delete_buffer(mut b: YY_BUFFER_STATE) {
    let mut tmp: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    if b.is_null() {
        return;
    }
    if !yy_buffer_stack.is_null() {
        tmp = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    } else {
        tmp = 0 as *mut libc::c_void as YY_BUFFER_STATE;
    }
    if b as libc::c_ulong == tmp as libc::c_ulong {
        let ref mut fresh9 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh9 = 0 as YY_BUFFER_STATE;
    }
    if (*b).yy_is_our_buffer != 0 {
        yyfree((*b).yy_ch_buf as *mut libc::c_void);
    }
    yyfree(b as *mut libc::c_void);
}
unsafe extern "C" fn yy_init_buffer(mut b: YY_BUFFER_STATE, mut file___1: *mut FILE) {
    let mut oerrno: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    tmp = __errno_location();
    oerrno = *tmp;
    yy_flush_buffer(b);
    (*b).yy_input_file = file___1;
    (*b).yy_fill_buffer = 1 as libc::c_int;
    if !yy_buffer_stack.is_null() {
        tmp___0 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    } else {
        tmp___0 = 0 as *mut libc::c_void as YY_BUFFER_STATE;
    }
    if b as libc::c_ulong != tmp___0 as libc::c_ulong {
        (*b).yy_bs_lineno = 1 as libc::c_int;
        (*b).yy_bs_column = 0 as libc::c_int;
    }
    if !file___1.is_null() {
        tmp___1 = fileno(file___1);
        tmp___2 = isatty(tmp___1);
        (*b).yy_is_interactive = (tmp___2 > 0 as libc::c_int) as libc::c_int;
    } else {
        (*b).yy_is_interactive = 0 as libc::c_int;
    }
    tmp___3 = __errno_location();
    *tmp___3 = oerrno;
}
pub unsafe extern "C" fn yy_flush_buffer(mut b: YY_BUFFER_STATE) {
    let mut tmp: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    if b.is_null() {
        return;
    }
    (*b).yy_n_chars = 0 as libc::c_int;
    *((*b).yy_ch_buf)
        .offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    *((*b).yy_ch_buf)
        .offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    (*b).yy_buf_pos = ((*b).yy_ch_buf).offset(0 as libc::c_int as isize);
    (*b).yy_at_bol = 1 as libc::c_int;
    (*b).yy_buffer_status = 0 as libc::c_int;
    if !yy_buffer_stack.is_null() {
        tmp = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    } else {
        tmp = 0 as *mut libc::c_void as YY_BUFFER_STATE;
    }
    if b as libc::c_ulong == tmp as libc::c_ulong {
        yy_load_buffer_state();
    }
}
pub unsafe extern "C" fn yypush_buffer_state(mut new_buffer: YY_BUFFER_STATE) {
    let mut tmp: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    let mut tmp___0: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    if new_buffer as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    yyensure_buffer_stack();
    if !yy_buffer_stack.is_null() {
        tmp = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    } else {
        tmp = 0 as *mut libc::c_void as YY_BUFFER_STATE;
    }
    if !tmp.is_null() {
        *yy_c_buf_p = yy_hold_char;
        let ref mut fresh10 = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_buf_pos;
        *fresh10 = yy_c_buf_p;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    }
    if !yy_buffer_stack.is_null() {
        tmp___0 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    } else {
        tmp___0 = 0 as *mut libc::c_void as YY_BUFFER_STATE;
    }
    if !tmp___0.is_null() {
        yy_buffer_stack_top = yy_buffer_stack_top.wrapping_add(1);
    }
    let ref mut fresh11 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh11 = new_buffer;
    yy_load_buffer_state();
    yy_did_buffer_switch_on_eof = 1 as libc::c_int;
}
pub unsafe extern "C" fn yypop_buffer_state() {
    let mut tmp: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    let mut tmp___0: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    let mut tmp___1: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    if !yy_buffer_stack.is_null() {
        tmp = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    } else {
        tmp = 0 as *mut libc::c_void as YY_BUFFER_STATE;
    }
    if tmp.is_null() {
        return;
    }
    if !yy_buffer_stack.is_null() {
        tmp___0 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    } else {
        tmp___0 = 0 as *mut libc::c_void as YY_BUFFER_STATE;
    }
    yy_delete_buffer(tmp___0);
    let ref mut fresh12 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh12 = 0 as *mut libc::c_void as YY_BUFFER_STATE;
    if yy_buffer_stack_top > 0 as libc::c_ulong {
        yy_buffer_stack_top = yy_buffer_stack_top.wrapping_sub(1);
    }
    if !yy_buffer_stack.is_null() {
        tmp___1 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    } else {
        tmp___1 = 0 as *mut libc::c_void as YY_BUFFER_STATE;
    }
    if !tmp___1.is_null() {
        yy_load_buffer_state();
        yy_did_buffer_switch_on_eof = 1 as libc::c_int;
    }
}
unsafe extern "C" fn yyensure_buffer_stack() {
    let mut num_to_alloc: yy_size_t = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut grow_size: yy_size_t = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    if yy_buffer_stack.is_null() {
        num_to_alloc = 1 as libc::c_int as yy_size_t;
        tmp = yyalloc(
            num_to_alloc
                .wrapping_mul(
                    ::std::mem::size_of::<*mut yy_buffer_state>() as libc::c_ulong,
                ),
        );
        yy_buffer_stack = tmp as *mut *mut yy_buffer_state;
        if yy_buffer_stack.is_null() {
            yy_fatal_error(
                b"out of dynamic memory in yyensure_buffer_stack()\0" as *const u8
                    as *const libc::c_char,
            );
        }
        memset(
            yy_buffer_stack as *mut libc::c_void,
            0 as libc::c_int,
            num_to_alloc
                .wrapping_mul(
                    ::std::mem::size_of::<*mut yy_buffer_state>() as libc::c_ulong,
                ) as _,
        );
        yy_buffer_stack_max = num_to_alloc;
        yy_buffer_stack_top = 0 as libc::c_int as size_t;
        return;
    }
    if yy_buffer_stack_top >= yy_buffer_stack_max.wrapping_sub(1 as libc::c_ulong) {
        grow_size = 8 as libc::c_int as yy_size_t;
        num_to_alloc = yy_buffer_stack_max.wrapping_add(grow_size);
        tmp___0 = yyrealloc(
            yy_buffer_stack as *mut libc::c_void,
            num_to_alloc
                .wrapping_mul(
                    ::std::mem::size_of::<*mut yy_buffer_state>() as libc::c_ulong,
                ),
        );
        yy_buffer_stack = tmp___0 as *mut *mut yy_buffer_state;
        if yy_buffer_stack.is_null() {
            yy_fatal_error(
                b"out of dynamic memory in yyensure_buffer_stack()\0" as *const u8
                    as *const libc::c_char,
            );
        }
        memset(
            yy_buffer_stack.offset(yy_buffer_stack_max as isize) as *mut libc::c_void,
            0 as libc::c_int,
            grow_size
                .wrapping_mul(
                    ::std::mem::size_of::<*mut yy_buffer_state>() as libc::c_ulong,
                ) as _,
        );
        yy_buffer_stack_max = num_to_alloc;
    }
}
pub unsafe extern "C" fn yy_scan_buffer(
    mut base: *mut libc::c_char,
    mut size: yy_size_t,
) -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    if size < 2 as libc::c_ulong {
        return 0 as *mut libc::c_void as YY_BUFFER_STATE
    } else {
        if *base.offset(size.wrapping_sub(2 as libc::c_ulong) as isize) as libc::c_int
            != 0 as libc::c_int
        {
            return 0 as *mut libc::c_void as YY_BUFFER_STATE
        } else {
            if *base.offset(size.wrapping_sub(1 as libc::c_ulong) as isize)
                as libc::c_int != 0 as libc::c_int
            {
                return 0 as *mut libc::c_void as YY_BUFFER_STATE;
            }
        }
    }
    tmp = yyalloc(::std::mem::size_of::<yy_buffer_state>() as libc::c_ulong);
    b = tmp as YY_BUFFER_STATE;
    if b.is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_scan_buffer()\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*b).yy_buf_size = size.wrapping_sub(2 as libc::c_ulong) as libc::c_int;
    tmp___0 = base;
    (*b).yy_ch_buf = tmp___0;
    (*b).yy_buf_pos = tmp___0;
    (*b).yy_is_our_buffer = 0 as libc::c_int;
    (*b).yy_input_file = 0 as *mut libc::c_void as *mut FILE;
    (*b).yy_n_chars = (*b).yy_buf_size;
    (*b).yy_is_interactive = 0 as libc::c_int;
    (*b).yy_at_bol = 1 as libc::c_int;
    (*b).yy_fill_buffer = 0 as libc::c_int;
    (*b).yy_buffer_status = 0 as libc::c_int;
    yy_switch_to_buffer(b);
    return b;
}
pub unsafe extern "C" fn yy_scan_string(
    mut yystr: *const libc::c_char,
) -> YY_BUFFER_STATE {
    let mut tmp: size_t = 0;
    let mut tmp___0: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    tmp = strlen(yystr);
    tmp___0 = yy_scan_bytes(yystr, tmp as libc::c_int);
    return tmp___0;
}
pub unsafe extern "C" fn yy_scan_bytes(
    mut yybytes: *const libc::c_char,
    mut _yybytes_len: libc::c_int,
) -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: yy_size_t = 0;
    let mut i: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_char = 0;
    n = (_yybytes_len + 2 as libc::c_int) as yy_size_t;
    tmp = yyalloc(n);
    buf = tmp as *mut libc::c_char;
    if buf.is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_scan_bytes()\0" as *const u8
                as *const libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    while i < _yybytes_len {
        *buf.offset(i as isize) = *yybytes.offset(i as isize);
        i += 1;
    }
    tmp___0 = 0 as libc::c_int as libc::c_char;
    *buf.offset((_yybytes_len + 1 as libc::c_int) as isize) = tmp___0;
    *buf.offset(_yybytes_len as isize) = tmp___0;
    b = yy_scan_buffer(buf, n);
    if b.is_null() {
        yy_fatal_error(
            b"bad buffer in yy_scan_bytes()\0" as *const u8 as *const libc::c_char,
        );
    }
    (*b).yy_is_our_buffer = 1 as libc::c_int;
    return b;
}
unsafe extern "C" fn yy_fatal_error(mut msg: *const libc::c_char) -> ! {
    __fprintf_chk(
        stderr,
        1 as libc::c_int,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        msg,
    );
    exit(2 as libc::c_int);
}
pub unsafe extern "C" fn yyget_lineno() -> libc::c_int {
    return yylineno;
}
pub unsafe extern "C" fn yyget_in() -> *mut FILE {
    return yyin;
}
pub unsafe extern "C" fn yyget_out() -> *mut FILE {
    return yyout;
}
pub unsafe extern "C" fn yyget_leng() -> libc::c_int {
    return yyleng;
}
pub unsafe extern "C" fn yyget_text() -> *mut libc::c_char {
    return yytext;
}
pub unsafe extern "C" fn yyset_lineno(mut _line_number: libc::c_int) {
    yylineno = _line_number;
}
pub unsafe extern "C" fn yyset_in(mut _in_str: *mut FILE) {
    yyin = _in_str;
}
pub unsafe extern "C" fn yyset_out(mut _out_str: *mut FILE) {
    yyout = _out_str;
}
pub unsafe extern "C" fn yyget_debug() -> libc::c_int {
    return yy_flex_debug;
}
pub unsafe extern "C" fn yyset_debug(mut _bdebug: libc::c_int) {
    yy_flex_debug = _bdebug;
}
unsafe extern "C" fn yy_init_globals() -> libc::c_int {
    yy_buffer_stack = 0 as *mut libc::c_void as *mut YY_BUFFER_STATE;
    yy_buffer_stack_top = 0 as libc::c_int as size_t;
    yy_buffer_stack_max = 0 as libc::c_int as size_t;
    yy_c_buf_p = 0 as *mut libc::c_void as *mut libc::c_char;
    yy_init = 0 as libc::c_int;
    yy_start = 0 as libc::c_int;
    yyin = 0 as *mut libc::c_void as *mut FILE;
    yyout = 0 as *mut libc::c_void as *mut FILE;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn yylex_destroy() -> libc::c_int {
    let mut tmp: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    let mut tmp___0: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    loop {
        if !yy_buffer_stack.is_null() {
            tmp___0 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        } else {
            tmp___0 = 0 as *mut libc::c_void as YY_BUFFER_STATE;
        }
        if tmp___0.is_null() {
            break;
        }
        if !yy_buffer_stack.is_null() {
            tmp = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        } else {
            tmp = 0 as *mut libc::c_void as YY_BUFFER_STATE;
        }
        yy_delete_buffer(tmp);
        let ref mut fresh13 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh13 = 0 as *mut libc::c_void as YY_BUFFER_STATE;
        yypop_buffer_state();
    }
    yyfree(yy_buffer_stack as *mut libc::c_void);
    yy_buffer_stack = 0 as *mut libc::c_void as *mut YY_BUFFER_STATE;
    yy_init_globals();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn yyalloc(mut size: yy_size_t) -> *mut libc::c_void {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(size);
    return tmp;
}
pub unsafe extern "C" fn yyrealloc(
    mut ptr: *mut libc::c_void,
    mut size: yy_size_t,
) -> *mut libc::c_void {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = realloc(ptr, size);
    return tmp;
}
pub unsafe extern "C" fn yyfree(mut ptr: *mut libc::c_void) {
    free(ptr as *mut libc::c_char as *mut libc::c_void);
}
pub unsafe extern "C" fn yyerror(mut str: *const libc::c_char) {
    __fprintf_chk(
        stderr,
        1 as libc::c_int,
        b"Parse error: %s\n\0" as *const u8 as *const libc::c_char,
        str,
    );
}
pub unsafe extern "C" fn yywrap() -> libc::c_int {
    return 1 as libc::c_int;
}
static mut yylhs: [YYINT; 16] = [
    -(1 as libc::c_int) as YYINT,
    0 as libc::c_int as YYINT,
    4 as libc::c_int as YYINT,
    4 as libc::c_int as YYINT,
    4 as libc::c_int as YYINT,
    4 as libc::c_int as YYINT,
    1 as libc::c_int as YYINT,
    1 as libc::c_int as YYINT,
    2 as libc::c_int as YYINT,
    2 as libc::c_int as YYINT,
    2 as libc::c_int as YYINT,
    2 as libc::c_int as YYINT,
    2 as libc::c_int as YYINT,
    2 as libc::c_int as YYINT,
    3 as libc::c_int as YYINT,
    3 as libc::c_int as YYINT,
];
static mut yylen: [YYINT; 16] = [
    2 as libc::c_int as YYINT,
    1 as libc::c_int as YYINT,
    3 as libc::c_int as YYINT,
    3 as libc::c_int as YYINT,
    3 as libc::c_int as YYINT,
    1 as libc::c_int as YYINT,
    1 as libc::c_int as YYINT,
    1 as libc::c_int as YYINT,
    3 as libc::c_int as YYINT,
    3 as libc::c_int as YYINT,
    3 as libc::c_int as YYINT,
    3 as libc::c_int as YYINT,
    3 as libc::c_int as YYINT,
    3 as libc::c_int as YYINT,
    3 as libc::c_int as YYINT,
    3 as libc::c_int as YYINT,
];
static mut yydefred: [YYINT; 28] = [
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    5 as libc::c_int as YYINT,
    6 as libc::c_int as YYINT,
    7 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    4 as libc::c_int as YYINT,
    11 as libc::c_int as YYINT,
    15 as libc::c_int as YYINT,
    12 as libc::c_int as YYINT,
    9 as libc::c_int as YYINT,
    10 as libc::c_int as YYINT,
    8 as libc::c_int as YYINT,
    14 as libc::c_int as YYINT,
    13 as libc::c_int as YYINT,
    3 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
];
static mut yydgoto: [YYINT; 5] = [
    3 as libc::c_int as YYINT,
    4 as libc::c_int as YYINT,
    5 as libc::c_int as YYINT,
    6 as libc::c_int as YYINT,
    7 as libc::c_int as YYINT,
];
static mut yysindex: [YYINT; 28] = [
    -(40 as libc::c_int) as YYINT,
    -(40 as libc::c_int) as YYINT,
    -(57 as libc::c_int) as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    -(250 as libc::c_int) as YYINT,
    -(39 as libc::c_int) as YYINT,
    -(249 as libc::c_int) as YYINT,
    -(245 as libc::c_int) as YYINT,
    -(244 as libc::c_int) as YYINT,
    -(243 as libc::c_int) as YYINT,
    -(247 as libc::c_int) as YYINT,
    -(242 as libc::c_int) as YYINT,
    -(40 as libc::c_int) as YYINT,
    -(40 as libc::c_int) as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    -(248 as libc::c_int) as YYINT,
];
static mut yyrindex: [YYINT; 28] = [
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    18 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    1 as libc::c_int as YYINT,
];
static mut yygindex: [YYINT; 5] = [
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    5 as libc::c_int as YYINT,
];
static mut yytable: [YYINT; 260] = [
    1 as libc::c_int as YYINT,
    2 as libc::c_int as YYINT,
    17 as libc::c_int as YYINT,
    12 as libc::c_int as YYINT,
    13 as libc::c_int as YYINT,
    11 as libc::c_int as YYINT,
    8 as libc::c_int as YYINT,
    15 as libc::c_int as YYINT,
    16 as libc::c_int as YYINT,
    15 as libc::c_int as YYINT,
    18 as libc::c_int as YYINT,
    19 as libc::c_int as YYINT,
    23 as libc::c_int as YYINT,
    24 as libc::c_int as YYINT,
    20 as libc::c_int as YYINT,
    21 as libc::c_int as YYINT,
    22 as libc::c_int as YYINT,
    25 as libc::c_int as YYINT,
    1 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    26 as libc::c_int as YYINT,
    27 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    2 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    9 as libc::c_int as YYINT,
    10 as libc::c_int as YYINT,
    14 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    15 as libc::c_int as YYINT,
    16 as libc::c_int as YYINT,
    2 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    2 as libc::c_int as YYINT,
];
static mut yycheck: [YYINT; 260] = [
    40 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    41 as libc::c_int as YYINT,
    60 as libc::c_int as YYINT,
    61 as libc::c_int as YYINT,
    62 as libc::c_int as YYINT,
    1 as libc::c_int as YYINT,
    257 as libc::c_int as YYINT,
    258 as libc::c_int as YYINT,
    257 as libc::c_int as YYINT,
    259 as libc::c_int as YYINT,
    260 as libc::c_int as YYINT,
    259 as libc::c_int as YYINT,
    260 as libc::c_int as YYINT,
    259 as libc::c_int as YYINT,
    259 as libc::c_int as YYINT,
    259 as libc::c_int as YYINT,
    259 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    -(1 as libc::c_int) as YYINT,
    15 as libc::c_int as YYINT,
    16 as libc::c_int as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    41 as libc::c_int as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    261 as libc::c_int as YYINT,
    262 as libc::c_int as YYINT,
    263 as libc::c_int as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    257 as libc::c_int as YYINT,
    258 as libc::c_int as YYINT,
    260 as libc::c_int as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    258 as libc::c_int as YYINT,
];
pub static mut yydebug: libc::c_int = 0;
pub static mut yynerrs: libc::c_int = 0;
pub static mut yyerrflag: libc::c_int = 0;
pub static mut yychar: libc::c_int = 0;
pub static mut yyval: YYSTYPE = __anonunion_YYSTYPE_714356573 {
    int_literal: 0,
};
pub static mut yylval: YYSTYPE = __anonunion_YYSTYPE_714356573 {
    int_literal: 0,
};
static mut yystack: YYSTACKDATA = YYSTACKDATA {
    stacksize: 0,
    s_base: 0 as *const YYINT as *mut YYINT,
    s_mark: 0 as *const YYINT as *mut YYINT,
    s_last: 0 as *const YYINT as *mut YYINT,
    l_base: 0 as *const YYSTYPE as *mut YYSTYPE,
    l_mark: 0 as *const YYSTYPE as *mut YYSTYPE,
};
unsafe extern "C" fn yygrowstack(mut data: *mut YYSTACKDATA) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut newsize: libc::c_uint = 0;
    let mut newss: *mut YYINT = 0 as *mut YYINT;
    let mut newvs: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    newsize = (*data).stacksize;
    if newsize == 0 as libc::c_uint {
        newsize = 200 as libc::c_uint;
    } else if newsize >= 10000 as libc::c_uint {
        return -(2 as libc::c_int)
    } else {
        newsize = newsize.wrapping_mul(2 as libc::c_uint);
        if newsize > 10000 as libc::c_uint {
            newsize = 10000 as libc::c_uint;
        }
    }
    i = ((*data).s_mark).offset_from((*data).s_base) as libc::c_long as libc::c_int;
    tmp = realloc(
        (*data).s_base as *mut libc::c_void,
        (newsize as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<YYINT>() as libc::c_ulong),
    );
    newss = tmp as *mut YYINT;
    if newss as libc::c_ulong == 0 as *mut YYINT as libc::c_ulong {
        return -(2 as libc::c_int);
    }
    (*data).s_base = newss;
    (*data).s_mark = newss.offset(i as isize);
    tmp___0 = realloc(
        (*data).l_base as *mut libc::c_void,
        (newsize as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<YYSTYPE>() as libc::c_ulong),
    );
    newvs = tmp___0 as *mut YYSTYPE;
    if newvs as libc::c_ulong == 0 as *mut YYSTYPE as libc::c_ulong {
        return -(2 as libc::c_int);
    }
    (*data).l_base = newvs;
    (*data).l_mark = newvs.offset(i as isize);
    (*data).stacksize = newsize;
    (*data)
        .s_last = ((*data).s_base)
        .offset(newsize as isize)
        .offset(-(1 as libc::c_int as isize));
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn yyparse() -> libc::c_int {
    let mut current_block: u64;
    let mut yym: libc::c_int = 0;
    let mut yyn: libc::c_int = 0;
    let mut yystate: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    yynerrs = 0 as libc::c_int;
    yyerrflag = 0 as libc::c_int;
    yychar = -(1 as libc::c_int);
    yystate = 0 as libc::c_int;
    if yystack.s_base as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp = yygrowstack(&mut yystack);
        if tmp == -(2 as libc::c_int) {
            current_block = 2636382364904497915;
        } else {
            current_block = 8515828400728868193;
        }
    } else {
        current_block = 8515828400728868193;
    }
    match current_block {
        8515828400728868193 => {
            yystack.s_mark = yystack.s_base;
            yystack.l_mark = yystack.l_base;
            yystate = 0 as libc::c_int;
            *yystack.s_mark = 0 as libc::c_int as YYINT;
            '_yyloop: loop {
                yyn = yydefred[yystate as usize] as libc::c_int;
                if !(yyn != 0 as libc::c_int) {
                    if yychar < 0 as libc::c_int {
                        yychar = yylex();
                        if yychar < 0 as libc::c_int {
                            yychar = 0 as libc::c_int;
                        }
                    }
                    yyn = yysindex[yystate as usize] as libc::c_int;
                    if yyn != 0 {
                        yyn += yychar;
                        if yyn >= 0 as libc::c_int {
                            if yyn <= 259 as libc::c_int {
                                if yycheck[yyn as usize] as libc::c_int == yychar {
                                    if yystack.s_mark as libc::c_ulong
                                        >= yystack.s_last as libc::c_ulong
                                    {
                                        tmp___0 = yygrowstack(&mut yystack);
                                        if tmp___0 == -(2 as libc::c_int) {
                                            current_block = 2636382364904497915;
                                            break;
                                        }
                                    }
                                    yystate = yytable[yyn as usize] as libc::c_int;
                                    yystack.s_mark = (yystack.s_mark).offset(1);
                                    *yystack.s_mark = yytable[yyn as usize];
                                    yystack.l_mark = (yystack.l_mark).offset(1);
                                    *yystack.l_mark = yylval;
                                    yychar = -(1 as libc::c_int);
                                    if yyerrflag > 0 as libc::c_int {
                                        yyerrflag -= 1;
                                    }
                                    continue;
                                }
                            }
                        }
                    }
                    yyn = yyrindex[yystate as usize] as libc::c_int;
                    if yyn != 0 {
                        yyn += yychar;
                        if yyn >= 0 as libc::c_int {
                            if yyn <= 259 as libc::c_int {
                                if yycheck[yyn as usize] as libc::c_int == yychar {
                                    yyn = yytable[yyn as usize] as libc::c_int;
                                    current_block = 17736937831720971013;
                                } else {
                                    current_block = 12199444798915819164;
                                }
                            } else {
                                current_block = 12199444798915819164;
                            }
                        } else {
                            current_block = 12199444798915819164;
                        }
                    } else {
                        current_block = 12199444798915819164;
                    }
                    match current_block {
                        17736937831720971013 => {}
                        _ => {
                            if !(yyerrflag != 0) {
                                yyerror(
                                    b"syntax error\0" as *const u8 as *const libc::c_char,
                                );
                                yynerrs += 1;
                            }
                            if yyerrflag < 3 as libc::c_int {
                                yyerrflag = 3 as libc::c_int;
                                loop {
                                    yyn = yysindex[*yystack.s_mark as usize] as libc::c_int;
                                    if yyn != 0 {
                                        yyn += 256 as libc::c_int;
                                        if yyn >= 0 as libc::c_int {
                                            if yyn <= 259 as libc::c_int {
                                                if yycheck[yyn as usize] as libc::c_int
                                                    == 256 as libc::c_int
                                                {
                                                    if yystack.s_mark as libc::c_ulong
                                                        >= yystack.s_last as libc::c_ulong
                                                    {
                                                        tmp___1 = yygrowstack(&mut yystack);
                                                        if tmp___1 == -(2 as libc::c_int) {
                                                            current_block = 2636382364904497915;
                                                            break '_yyloop;
                                                        }
                                                    }
                                                    yystate = yytable[yyn as usize] as libc::c_int;
                                                    yystack.s_mark = (yystack.s_mark).offset(1);
                                                    *yystack.s_mark = yytable[yyn as usize];
                                                    yystack.l_mark = (yystack.l_mark).offset(1);
                                                    *yystack.l_mark = yylval;
                                                    continue '_yyloop;
                                                }
                                            }
                                        }
                                    }
                                    if yystack.s_mark as libc::c_ulong
                                        <= yystack.s_base as libc::c_ulong
                                    {
                                        current_block = 13895078145312174667;
                                        break '_yyloop;
                                    }
                                    yystack.s_mark = (yystack.s_mark).offset(-1);
                                    yystack.l_mark = (yystack.l_mark).offset(-1);
                                }
                            } else {
                                if yychar == 0 as libc::c_int {
                                    current_block = 13895078145312174667;
                                    break;
                                }
                                yychar = -(1 as libc::c_int);
                                continue;
                            }
                        }
                    }
                }
                yym = yylen[yyn as usize] as libc::c_int;
                if yym != 0 {
                    yyval = *(yystack.l_mark).offset((1 as libc::c_int - yym) as isize);
                } else {
                    memset(
                        &mut yyval as *mut YYSTYPE as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<YYSTYPE>() as libc::c_ulong as _,
                    );
                }
                match yyn {
                    1 => {
                        zfilter = (*(yystack.l_mark).offset(0 as libc::c_int as isize))
                            .expr;
                    }
                    2 => {
                        yyval.expr = make_op_node(OR);
                        (*yyval.expr)
                            .left_child = (*(yystack.l_mark)
                            .offset(-(2 as libc::c_int) as isize))
                            .expr;
                        (*yyval.expr)
                            .right_child = (*(yystack.l_mark)
                            .offset(0 as libc::c_int as isize))
                            .expr;
                    }
                    3 => {
                        yyval.expr = make_op_node(AND);
                        (*yyval.expr)
                            .left_child = (*(yystack.l_mark)
                            .offset(-(2 as libc::c_int) as isize))
                            .expr;
                        (*yyval.expr)
                            .right_child = (*(yystack.l_mark)
                            .offset(0 as libc::c_int as isize))
                            .expr;
                    }
                    4 => {
                        yyval
                            .expr = (*(yystack.l_mark)
                            .offset(-(1 as libc::c_int) as isize))
                            .expr;
                    }
                    5 => {
                        yyval
                            .expr = (*(yystack.l_mark).offset(0 as libc::c_int as isize))
                            .expr;
                    }
                    6 => {
                        yyval
                            .expr = (*(yystack.l_mark).offset(0 as libc::c_int as isize))
                            .expr;
                    }
                    7 => {
                        yyval
                            .expr = (*(yystack.l_mark).offset(0 as libc::c_int as isize))
                            .expr;
                    }
                    8 => {
                        yyval.expr = make_op_node(EQ);
                        (*yyval.expr)
                            .left_child = make_field_node(
                            (*(yystack.l_mark).offset(-(2 as libc::c_int) as isize))
                                .string_literal,
                        );
                        (*yyval.expr)
                            .right_child = make_int_node(
                            (*(yystack.l_mark).offset(0 as libc::c_int as isize))
                                .int_literal,
                        );
                    }
                    9 => {
                        yyval.expr = make_op_node(GT);
                        (*yyval.expr)
                            .left_child = make_field_node(
                            (*(yystack.l_mark).offset(-(2 as libc::c_int) as isize))
                                .string_literal,
                        );
                        (*yyval.expr)
                            .right_child = make_int_node(
                            (*(yystack.l_mark).offset(0 as libc::c_int as isize))
                                .int_literal,
                        );
                    }
                    10 => {
                        yyval.expr = make_op_node(LT);
                        (*yyval.expr)
                            .left_child = make_field_node(
                            (*(yystack.l_mark).offset(-(2 as libc::c_int) as isize))
                                .string_literal,
                        );
                        (*yyval.expr)
                            .right_child = make_int_node(
                            (*(yystack.l_mark).offset(0 as libc::c_int as isize))
                                .int_literal,
                        );
                    }
                    11 => {
                        yyval.expr = make_op_node(NEQ);
                        (*yyval.expr)
                            .left_child = make_field_node(
                            (*(yystack.l_mark).offset(-(2 as libc::c_int) as isize))
                                .string_literal,
                        );
                        (*yyval.expr)
                            .right_child = make_int_node(
                            (*(yystack.l_mark).offset(0 as libc::c_int as isize))
                                .int_literal,
                        );
                    }
                    12 => {
                        yyval.expr = make_op_node(GT_EQ);
                        (*yyval.expr)
                            .left_child = make_field_node(
                            (*(yystack.l_mark).offset(-(2 as libc::c_int) as isize))
                                .string_literal,
                        );
                        (*yyval.expr)
                            .right_child = make_int_node(
                            (*(yystack.l_mark).offset(0 as libc::c_int as isize))
                                .int_literal,
                        );
                    }
                    13 => {
                        yyval.expr = make_op_node(LT_EQ);
                        (*yyval.expr)
                            .left_child = make_field_node(
                            (*(yystack.l_mark).offset(-(2 as libc::c_int) as isize))
                                .string_literal,
                        );
                        (*yyval.expr)
                            .right_child = make_int_node(
                            (*(yystack.l_mark).offset(0 as libc::c_int as isize))
                                .int_literal,
                        );
                    }
                    14 => {
                        yyval.expr = make_op_node(EQ);
                        (*yyval.expr)
                            .left_child = make_field_node(
                            (*(yystack.l_mark).offset(-(2 as libc::c_int) as isize))
                                .string_literal,
                        );
                        (*yyval.expr)
                            .right_child = make_string_node(
                            (*(yystack.l_mark).offset(0 as libc::c_int as isize))
                                .string_literal,
                        );
                    }
                    15 => {
                        yyval.expr = make_op_node(NEQ);
                        (*yyval.expr)
                            .left_child = make_field_node(
                            (*(yystack.l_mark).offset(-(2 as libc::c_int) as isize))
                                .string_literal,
                        );
                        (*yyval.expr)
                            .right_child = make_string_node(
                            (*(yystack.l_mark).offset(0 as libc::c_int as isize))
                                .string_literal,
                        );
                    }
                    _ => {}
                }
                yystack.s_mark = (yystack.s_mark).offset(-(yym as isize));
                yystate = *yystack.s_mark as libc::c_int;
                yystack.l_mark = (yystack.l_mark).offset(-(yym as isize));
                yym = yylhs[yyn as usize] as libc::c_int;
                if yystate == 0 as libc::c_int {
                    if yym == 0 as libc::c_int {
                        yystate = 3 as libc::c_int;
                        yystack.s_mark = (yystack.s_mark).offset(1);
                        *yystack.s_mark = 3 as libc::c_int as YYINT;
                        yystack.l_mark = (yystack.l_mark).offset(1);
                        *yystack.l_mark = yyval;
                        if yychar < 0 as libc::c_int {
                            yychar = yylex();
                            if yychar < 0 as libc::c_int {
                                yychar = 0 as libc::c_int;
                            }
                        }
                        if !(yychar == 0 as libc::c_int) {
                            continue;
                        }
                        return 0 as libc::c_int;
                    }
                }
                yyn = yygindex[yym as usize] as libc::c_int;
                if yyn != 0 {
                    yyn += yystate;
                    if yyn >= 0 as libc::c_int {
                        if yyn <= 259 as libc::c_int {
                            if yycheck[yyn as usize] as libc::c_int == yystate {
                                yystate = yytable[yyn as usize] as libc::c_int;
                            } else {
                                yystate = yydgoto[yym as usize] as libc::c_int;
                            }
                        } else {
                            yystate = yydgoto[yym as usize] as libc::c_int;
                        }
                    } else {
                        yystate = yydgoto[yym as usize] as libc::c_int;
                    }
                } else {
                    yystate = yydgoto[yym as usize] as libc::c_int;
                }
                if yystack.s_mark as libc::c_ulong >= yystack.s_last as libc::c_ulong {
                    tmp___2 = yygrowstack(&mut yystack);
                    if tmp___2 == -(2 as libc::c_int) {
                        current_block = 2636382364904497915;
                        break;
                    }
                }
                yystack.s_mark = (yystack.s_mark).offset(1);
                *yystack.s_mark = yystate as YYINT;
                yystack.l_mark = (yystack.l_mark).offset(1);
                *yystack.l_mark = yyval;
            }
        }
        _ => {}
    }
    match current_block {
        2636382364904497915 => {
            yyerror(b"yacc stack overflow\0" as *const u8 as *const libc::c_char);
        }
        _ => {}
    }
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn in_checksum(
    mut ip_pkt: *mut libc::c_ushort,
    mut len: libc::c_int,
) -> libc::c_ushort {
    let mut sum: libc::c_ulong = 0;
    let mut nwords: libc::c_int = 0;
    let mut tmp: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    sum = 0 as libc::c_ulong;
    nwords = len / 2 as libc::c_int;
    while nwords > 0 as libc::c_int {
        tmp = ip_pkt;
        ip_pkt = ip_pkt.offset(1);
        sum = sum.wrapping_add(*tmp as libc::c_ulong);
        nwords -= 1;
    }
    if len % 2 as libc::c_int == 1 as libc::c_int {
        sum = sum.wrapping_add(*(ip_pkt as *mut libc::c_uchar) as libc::c_ulong);
    }
    sum = (sum >> 16 as libc::c_int).wrapping_add(sum & 65535 as libc::c_ulong);
    return !sum as libc::c_ushort;
}
#[inline]
unsafe extern "C" fn zmap_ip_checksum(mut buf: *mut libc::c_ushort) -> libc::c_ushort {
    let mut tmp: libc::c_ushort = 0;
    tmp = in_checksum(buf, ::std::mem::size_of::<ip>() as libc::c_ulong as libc::c_int);
    return tmp;
}
#[inline]
unsafe extern "C" fn icmp_checksum(
    mut buf: *mut libc::c_ushort,
    mut buflen: size_t,
) -> libc::c_ushort {
    let mut tmp: libc::c_ushort = 0;
    tmp = in_checksum(buf, buflen as libc::c_int);
    return tmp;
}
#[inline]
unsafe extern "C" fn get_icmp_header(
    mut ip_hdr: *const ip,
    mut len: uint32_t,
) -> *mut icmp {
    if ((4 as libc::c_uint).wrapping_mul((*ip_hdr).ip_hl()) as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<icmp>() as libc::c_ulong)
        > len as libc::c_ulong
    {
        return 0 as *mut libc::c_void as *mut icmp;
    }
    return (ip_hdr as *mut libc::c_char)
        .offset((4 as libc::c_uint).wrapping_mul((*ip_hdr).ip_hl()) as isize)
        as *mut icmp;
}
pub static mut icmp_usage_error: *const libc::c_char = b"unknown ICMP probe specification (expected file:/path or text:STRING or hex:01020304)\0"
    as *const u8 as *const libc::c_char;
static mut icmp_payload_len: size_t = 0 as libc::c_int as size_t;
static mut icmp_payload_default_len: libc::c_ulong = 20 as libc::c_int as size_t;
static mut icmp_payload: *mut libc::c_char = 0 as *const libc::c_void
    as *mut libc::c_void as *mut libc::c_char;
pub unsafe extern "C" fn icmp_global_initialize(
    mut conf: *mut state_conf,
) -> libc::c_int {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: size_t = 0;
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut inp: *mut FILE = 0 as *mut FILE;
    let mut tmp___2: *mut FILE = 0 as *mut FILE;
    let mut tmp___3: libc::c_int = 0;
    let mut input_size: size_t = 0;
    let mut tmp___4: libc::c_long = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___7: size_t = 0;
    let mut tmp___8: size_t = 0;
    let mut tmp___9: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut n: libc::c_uint = 0;
    let mut i: size_t = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: libc::c_int = 0;
    let mut tmp___13: libc::c_int = 0;
    if !((*conf).probe_args).is_null() {
        tmp___0 = strlen((*conf).probe_args as *const libc::c_char);
        if !(tmp___0 > 0 as libc::c_ulong) {
            tmp = xmalloc(icmp_payload_default_len);
            icmp_payload = tmp as *mut libc::c_char;
            icmp_payload_len = icmp_payload_default_len;
            return 0 as libc::c_int;
        }
    } else {
        tmp = xmalloc(icmp_payload_default_len);
        icmp_payload = tmp as *mut libc::c_char;
        icmp_payload_len = icmp_payload_default_len;
        return 0 as libc::c_int;
    }
    tmp___1 = strchr((*conf).probe_args as *const libc::c_char, ':' as i32);
    c = tmp___1;
    if c.is_null() {
        log_error(b"icmp\0" as *const u8 as *const libc::c_char, icmp_usage_error);
        return 1 as libc::c_int;
    }
    c = c.offset(1);
    tmp___13 = strncmp(
        (*conf).probe_args as *const libc::c_char,
        b"text\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as size_t,
    );
    if tmp___13 == 0 as libc::c_int {
        icmp_payload = strdup(c as *const libc::c_char);
        icmp_payload_len = strlen(icmp_payload as *const libc::c_char);
    } else {
        tmp___12 = strncmp(
            (*conf).probe_args as *const libc::c_char,
            b"file\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as size_t,
        );
        if tmp___12 == 0 as libc::c_int {
            tmp___2 = fopen(
                c as *const libc::c_char,
                b"rb\0" as *const u8 as *const libc::c_char,
            );
            inp = tmp___2;
            if inp.is_null() {
                log_error(
                    b"icmp\0" as *const u8 as *const libc::c_char,
                    b"could not open ICMP data file '%s'\0" as *const u8
                        as *const libc::c_char,
                    c,
                );
                return 1 as libc::c_int;
            }
            tmp___3 = fseek(inp, 0 as libc::c_long, 2 as libc::c_int);
            if tmp___3 != 0 {
                log_error(
                    b"icmp\0" as *const u8 as *const libc::c_char,
                    b"unable to get size of ICMP data file '%s'\0" as *const u8
                        as *const libc::c_char,
                    c,
                );
                return 1 as libc::c_int;
            }
            tmp___4 = ftell(inp);
            input_size = tmp___4 as size_t;
            if input_size > 1458 as libc::c_ulong {
                log_error(
                    b"icmp\0" as *const u8 as *const libc::c_char,
                    b"input file larger than %d bytes and will not fit on the wire (%llu bytes provided)\0"
                        as *const u8 as *const libc::c_char,
                    1458 as libc::c_int,
                    input_size,
                );
                return 1 as libc::c_int;
            }
            tmp___5 = fseek(inp, 0 as libc::c_long, 0 as libc::c_int);
            if tmp___5 != 0 {
                log_error(
                    b"icmp\0" as *const u8 as *const libc::c_char,
                    b"unable to read ICMP data file '%s'\0" as *const u8
                        as *const libc::c_char,
                    c,
                );
                return 1 as libc::c_int;
            }
            tmp___6 = xmalloc(1458 as libc::c_int as size_t);
            icmp_payload = tmp___6 as *mut libc::c_char;
            icmp_payload_len = fread(
                icmp_payload as *mut libc::c_void,
                1 as libc::c_int as size_t,
                1458 as libc::c_int as size_t,
                inp,
            );
            fclose(inp);
        } else {
            tmp___11 = strncmp(
                (*conf).probe_args as *const libc::c_char,
                b"hex\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int as size_t,
            );
            if tmp___11 == 0 as libc::c_int {
                tmp___7 = strlen(c as *const libc::c_char);
                if tmp___7.wrapping_rem(2 as libc::c_ulong) != 0 as libc::c_ulong {
                    log_error(
                        b"icmp\0" as *const u8 as *const libc::c_char,
                        b"invalid hex input (length must be a multiple of 2)\0"
                            as *const u8 as *const libc::c_char,
                    );
                    return 1 as libc::c_int;
                }
                tmp___8 = strlen(c as *const libc::c_char);
                icmp_payload_len = tmp___8.wrapping_div(2 as libc::c_ulong);
                tmp___9 = xmalloc(icmp_payload_len);
                icmp_payload = tmp___9 as *mut libc::c_char;
                i = 0 as libc::c_int as size_t;
                while i < icmp_payload_len {
                    tmp___10 = sscanf(
                        c.offset(i.wrapping_mul(2 as libc::c_ulong) as isize)
                            as *const libc::c_char,
                        b"%2x\0" as *const u8 as *const libc::c_char,
                        &mut n as *mut libc::c_uint,
                    );
                    if tmp___10 != 1 as libc::c_int {
                        free(icmp_payload as *mut libc::c_void);
                        log_error(
                            b"icmp\0" as *const u8 as *const libc::c_char,
                            b"non-hex character: '%c'\0" as *const u8
                                as *const libc::c_char,
                            *c.offset(i.wrapping_mul(2 as libc::c_ulong) as isize)
                                as libc::c_int,
                        );
                        return 1 as libc::c_int;
                    }
                    *icmp_payload
                        .offset(i as isize) = (n & 255 as libc::c_uint) as libc::c_char;
                    i = i.wrapping_add(1);
                }
            } else {
                log_error(
                    b"icmp\0" as *const u8 as *const libc::c_char,
                    icmp_usage_error,
                );
                return 1 as libc::c_int;
            }
        }
    }
    if icmp_payload_len > 1458 as libc::c_ulong {
        log_error(
            b"icmp\0" as *const u8 as *const libc::c_char,
            b"reducing ICMP payload must be at most %d bytes to fit on the wire (%d were provided)\n\0"
                as *const u8 as *const libc::c_char,
            1458 as libc::c_int,
            icmp_payload_len,
        );
        return 1 as libc::c_int;
    }
    module_icmp_echo
        .max_packet_length = (::std::mem::size_of::<ether_header>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ip>() as libc::c_ulong)
        .wrapping_add(8 as libc::c_ulong)
        .wrapping_add(icmp_payload_len);
    if !(module_icmp_echo.max_packet_length <= 1500 as libc::c_ulong) {
        __assert_fail(
            b"module_icmp_echo.max_packet_length <= 1500\0" as *const u8
                as *const libc::c_char,
            b"src/probe_modules/module_icmp_echo.c\0" as *const u8
                as *const libc::c_char,
            112 as libc::c_uint,
            b"icmp_global_initialize\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn icmp_global_cleanup(
    mut zconf___0: *mut state_conf,
    mut zsend___0: *mut state_send,
    mut zrecv___0: *mut state_recv,
) -> libc::c_int {
    if !icmp_payload.is_null() {
        free(icmp_payload as *mut libc::c_void);
        icmp_payload = 0 as *mut libc::c_void as *mut libc::c_char;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn icmp_echo_init_perthread(
    mut buf: *mut libc::c_void,
    mut src: *mut macaddr_t,
    mut gw: *mut macaddr_t,
    mut dst_port: port_h_t,
    mut arg_ptr: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut eth_header: *mut ether_header = 0 as *mut ether_header;
    let mut ip_header: *mut ip = 0 as *mut ip;
    let mut len: uint16_t = 0;
    let mut tmp: __uint16_t = 0;
    let mut icmp_header: *mut icmp = 0 as *mut icmp;
    let mut payload: *mut libc::c_char = 0 as *mut libc::c_char;
    memset(buf, 0 as libc::c_int, 4096 as libc::c_int as size_t as _);
    eth_header = buf as *mut ether_header;
    make_eth_header(eth_header, src, gw);
    ip_header = eth_header.offset(1 as libc::c_int as isize) as *mut ip;
    tmp = __bswap_16(
        (::std::mem::size_of::<ip>() as libc::c_ulong)
            .wrapping_add(8 as libc::c_ulong)
            .wrapping_add(icmp_payload_len) as __uint16_t,
    );
    len = tmp;
    make_ip_header(ip_header, 1 as libc::c_int as uint8_t, len);
    icmp_header = ip_header.offset(1 as libc::c_int as isize) as *mut icmp;
    make_icmp_header(icmp_header);
    payload = (icmp_header as *mut libc::c_char).offset(8 as libc::c_int as isize);
    memcpy(
        payload as *mut libc::c_void,
        icmp_payload as *const libc::c_void,
        icmp_payload_len as _,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn icmp_echo_make_packet(
    mut buf: *mut libc::c_void,
    mut buf_len: *mut size_t,
    mut src_ip: ipaddr_n_t,
    mut dst_ip: ipaddr_n_t,
    mut ttl: uint8_t,
    mut validation: *mut uint32_t,
    mut probe_num: libc::c_int,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    let mut eth_header: *mut ether_header = 0 as *mut ether_header;
    let mut ip_header: *mut ip = 0 as *mut ip;
    let mut icmp_header: *mut icmp = 0 as *mut icmp;
    let mut icmp_idnum: uint16_t = 0;
    let mut icmp_seqnum: uint16_t = 0;
    let mut ip_len: size_t = 0;
    eth_header = buf as *mut ether_header;
    ip_header = eth_header.offset(1 as libc::c_int as isize) as *mut ip;
    icmp_header = ip_header.offset(1 as libc::c_int as isize) as *mut icmp;
    icmp_idnum = (*validation.offset(1 as libc::c_int as isize) & 65535 as libc::c_uint)
        as uint16_t;
    icmp_seqnum = (*validation.offset(2 as libc::c_int as isize) & 65535 as libc::c_uint)
        as uint16_t;
    (*ip_header).ip_src.s_addr = src_ip;
    (*ip_header).ip_dst.s_addr = dst_ip;
    (*ip_header).ip_ttl = ttl;
    (*icmp_header).icmp_hun.ih_idseq.icd_id = icmp_idnum;
    (*icmp_header).icmp_hun.ih_idseq.icd_seq = icmp_seqnum;
    (*icmp_header).icmp_cksum = 0 as libc::c_int as uint16_t;
    (*icmp_header)
        .icmp_cksum = icmp_checksum(
        icmp_header as *mut libc::c_ushort,
        (8 as libc::c_ulong).wrapping_add(icmp_payload_len),
    );
    ip_len = (::std::mem::size_of::<ip>() as libc::c_ulong)
        .wrapping_add(8 as libc::c_ulong)
        .wrapping_add(icmp_payload_len);
    (*ip_header).ip_len = __bswap_16(ip_len as __uint16_t);
    (*ip_header).ip_sum = 0 as libc::c_int as libc::c_ushort;
    (*ip_header).ip_sum = zmap_ip_checksum(ip_header as *mut libc::c_ushort);
    *buf_len = ip_len
        .wrapping_add(::std::mem::size_of::<ether_header>() as libc::c_ulong);
    return 0 as libc::c_int;
}
unsafe extern "C" fn icmp_echo_print_packet(
    mut fp: *mut FILE,
    mut packet: *mut libc::c_void,
) {
    let mut ethh: *mut ether_header = 0 as *mut ether_header;
    let mut iph: *mut ip = 0 as *mut ip;
    let mut icmp_header: *mut icmp = 0 as *mut icmp;
    let mut tmp: __uint16_t = 0;
    let mut tmp___0: __uint16_t = 0;
    let mut tmp___1: __uint16_t = 0;
    ethh = packet as *mut ether_header;
    iph = ethh.offset(1 as libc::c_int as isize) as *mut ip;
    icmp_header = iph.offset(1 as libc::c_int as isize) as *mut icmp;
    tmp = __bswap_16((*icmp_header).icmp_hun.ih_idseq.icd_seq);
    tmp___0 = __bswap_16((*icmp_header).icmp_hun.ih_idseq.icd_id);
    tmp___1 = __bswap_16((*icmp_header).icmp_cksum);
    __fprintf_chk(
        fp,
        1 as libc::c_int,
        b"icmp { type: %u | code: %u | checksum: %#04X | id: %u | seq: %u }\n\0"
            as *const u8 as *const libc::c_char,
        (*icmp_header).icmp_type as libc::c_int,
        (*icmp_header).icmp_code as libc::c_int,
        tmp___1 as libc::c_int,
        tmp___0 as libc::c_int,
        tmp as libc::c_int,
    );
    fprintf_ip_header(fp, iph);
    fprintf_eth_header(fp, ethh);
    __fprintf_chk(
        fp,
        1 as libc::c_int,
        b"------------------------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
}
unsafe extern "C" fn imcp_validate_id_seq(
    mut icmp_h: *mut icmp,
    mut validation: *mut uint32_t,
) -> libc::c_int {
    if (*icmp_h).icmp_hun.ih_idseq.icd_id as libc::c_uint
        != *validation.offset(1 as libc::c_int as isize) & 65535 as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if (*icmp_h).icmp_hun.ih_idseq.icd_seq as libc::c_uint
        != *validation.offset(2 as libc::c_int as isize) & 65535 as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn icmp_validate_packet(
    mut ip_hdr: *const ip,
    mut len: uint32_t,
    mut src_ip: *mut uint32_t,
    mut validation: *mut uint32_t,
) -> libc::c_int {
    let mut icmp_h: *mut icmp = 0 as *mut icmp;
    let mut tmp: *mut icmp = 0 as *mut icmp;
    let mut tmp___0: libc::c_int = 0;
    let mut ip_inner: *mut ip = 0 as *mut ip;
    let mut ip_inner_len: size_t = 0;
    let mut icmp_inner_valid: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut icmp_inner: *mut icmp = 0 as *mut icmp;
    let mut tmp___2: *mut icmp = 0 as *mut icmp;
    let mut tmp___3: libc::c_int = 0;
    if (*ip_hdr).ip_p as libc::c_int != 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    tmp = get_icmp_header(ip_hdr, len);
    icmp_h = tmp;
    if icmp_h.is_null() {
        return 0 as libc::c_int;
    }
    if (*icmp_h).icmp_type as libc::c_int == 0 as libc::c_int {
        tmp___0 = imcp_validate_id_seq(icmp_h, validation);
        return tmp___0;
    } else {
        tmp___1 = icmp_helper_validate(
            ip_hdr,
            len,
            ::std::mem::size_of::<icmp>() as libc::c_ulong,
            &mut ip_inner,
            &mut ip_inner_len,
        );
        icmp_inner_valid = tmp___1;
        if icmp_inner_valid == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        tmp___2 = get_icmp_header(ip_inner as *const ip, ip_inner_len as uint32_t);
        icmp_inner = tmp___2;
        if icmp_inner.is_null() {
            return 0 as libc::c_int;
        }
        validate_gen(
            (*ip_hdr).ip_dst.s_addr,
            (*ip_inner).ip_dst.s_addr,
            validation as *mut uint8_t,
        );
        tmp___3 = imcp_validate_id_seq(icmp_inner, validation);
        return tmp___3;
    };
}
unsafe extern "C" fn icmp_echo_process_packet(
    mut packet: *const u_char,
    mut len: uint32_t,
    mut fs: *mut fieldset_t,
    mut validation: *mut uint32_t,
    mut ts: timespec,
) {
    let mut ip_hdr: *mut ip = 0 as *mut ip;
    let mut icmp_hdr: *mut icmp = 0 as *mut icmp;
    let mut tmp: __uint16_t = 0;
    let mut tmp___0: __uint16_t = 0;
    let mut hdrlen: uint32_t = 0;
    let mut datalen: libc::c_int = 0;
    let mut data: *const uint8_t = 0 as *const uint8_t;
    ip_hdr = packet
        .offset(::std::mem::size_of::<ether_header>() as libc::c_ulong as isize)
        as *mut ip;
    icmp_hdr = (ip_hdr as *mut libc::c_char)
        .offset((4 as libc::c_uint).wrapping_mul((*ip_hdr).ip_hl()) as isize)
        as *mut icmp;
    fs_add_uint64(
        fs,
        b"type\0" as *const u8 as *const libc::c_char,
        (*icmp_hdr).icmp_type as uint64_t,
    );
    fs_add_uint64(
        fs,
        b"code\0" as *const u8 as *const libc::c_char,
        (*icmp_hdr).icmp_code as uint64_t,
    );
    tmp = __bswap_16((*icmp_hdr).icmp_hun.ih_idseq.icd_id);
    fs_add_uint64(fs, b"icmp_id\0" as *const u8 as *const libc::c_char, tmp as uint64_t);
    tmp___0 = __bswap_16((*icmp_hdr).icmp_hun.ih_idseq.icd_seq);
    fs_add_uint64(fs, b"seq\0" as *const u8 as *const libc::c_char, tmp___0 as uint64_t);
    hdrlen = (::std::mem::size_of::<ether_header>() as libc::c_ulong)
        .wrapping_add(
            (4 as libc::c_uint).wrapping_mul((*ip_hdr).ip_hl()) as libc::c_ulong,
        )
        .wrapping_add(4 as libc::c_ulong) as uint32_t;
    match (*icmp_hdr).icmp_type as libc::c_int {
        0 => {
            fs_add_string(
                fs,
                b"classification\0" as *const u8 as *const libc::c_char,
                b"echoreply\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
            );
            fs_add_uint64(
                fs,
                b"success\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as uint64_t,
            );
        }
        3 => {
            fs_add_string(
                fs,
                b"classification\0" as *const u8 as *const libc::c_char,
                b"unreach\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
            );
            fs_add_bool(
                fs,
                b"success\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        }
        4 => {
            fs_add_string(
                fs,
                b"classification\0" as *const u8 as *const libc::c_char,
                b"sourcequench\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as libc::c_int,
            );
            fs_add_bool(
                fs,
                b"success\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        }
        5 => {
            fs_add_string(
                fs,
                b"classification\0" as *const u8 as *const libc::c_char,
                b"redirect\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
            );
            fs_add_bool(
                fs,
                b"success\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        }
        11 => {
            fs_add_string(
                fs,
                b"classification\0" as *const u8 as *const libc::c_char,
                b"timxceed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
            );
            fs_add_bool(
                fs,
                b"success\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        }
        _ => {
            fs_add_string(
                fs,
                b"classification\0" as *const u8 as *const libc::c_char,
                b"other\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
            );
            fs_add_bool(
                fs,
                b"success\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        }
    }
    datalen = len.wrapping_sub(hdrlen) as libc::c_int;
    if datalen > 0 as libc::c_int {
        data = packet.offset(hdrlen as isize) as *mut uint8_t as *const uint8_t;
        fs_add_binary(
            fs,
            b"data\0" as *const u8 as *const libc::c_char,
            datalen as size_t,
            data as *mut libc::c_void,
            0 as libc::c_int,
        );
    } else {
        fs_add_null(fs, b"data\0" as *const u8 as *const libc::c_char);
    };
}
static mut fields: [fielddef_t; 7] = [
    {
        let mut init = field_def {
            name: b"type\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"icmp message type\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"code\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"icmp message sub type code\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_id\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"icmp id number\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"seq\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"icmp sequence number\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"classification\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"probe module classification\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"success\0" as *const u8 as *const libc::c_char,
            type_0: b"bool\0" as *const u8 as *const libc::c_char,
            desc: b"did probe module classify response as success\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"data\0" as *const u8 as *const libc::c_char,
            type_0: b"binary\0" as *const u8 as *const libc::c_char,
            desc: b"ICMP payload\0" as *const u8 as *const libc::c_char,
        };
        init
    },
];
pub static mut module_icmp_echo: probe_module_t = unsafe {
    {
        let mut init = probe_module {
            name: b"icmp_echoscan\0" as *const u8 as *const libc::c_char,
            max_packet_length: 48 as libc::c_int as size_t,
            pcap_filter: b"icmp and icmp[0]!=8\0" as *const u8 as *const libc::c_char,
            pcap_snaplen: 96 as libc::c_int as size_t,
            port_args: 0 as libc::c_int as uint8_t,
            global_initialize: Some(
                icmp_global_initialize
                    as unsafe extern "C" fn(*mut state_conf) -> libc::c_int,
            ),
            thread_initialize: Some(
                icmp_echo_init_perthread
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut macaddr_t,
                        *mut macaddr_t,
                        port_h_t,
                        *mut *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            make_packet: Some(
                icmp_echo_make_packet
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut size_t,
                        ipaddr_n_t,
                        ipaddr_n_t,
                        uint8_t,
                        *mut uint32_t,
                        libc::c_int,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            print_packet: Some(
                icmp_echo_print_packet
                    as unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> (),
            ),
            validate_packet: Some(
                icmp_validate_packet
                    as unsafe extern "C" fn(
                        *const ip,
                        uint32_t,
                        *mut uint32_t,
                        *mut uint32_t,
                    ) -> libc::c_int,
            ),
            process_packet: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const u_char,
                        uint32_t,
                        *mut fieldset_t,
                        *mut uint32_t,
                        timespec,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *const u_char,
                        uint32_t,
                        *mut fieldset_t,
                        *mut uint32_t,
                        timespec,
                    ) -> (),
                >,
            >(
                Some(
                    icmp_echo_process_packet
                        as unsafe extern "C" fn(
                            *const u_char,
                            uint32_t,
                            *mut fieldset_t,
                            *mut uint32_t,
                            timespec,
                        ) -> (),
                ),
            ),
            close: Some(
                icmp_global_cleanup
                    as unsafe extern "C" fn(
                        *mut state_conf,
                        *mut state_send,
                        *mut state_recv,
                    ) -> libc::c_int,
            ),
            output_type: 1 as libc::c_int,
            fields: fields.as_ptr() as *mut _,
            numfields: 7 as libc::c_int,
            helptext: b"Probe module that sends ICMP echo requests to hosts.\nPayload of ICMP packets will consist of zeroes unless you customize it with\n --probe-args=file:/path_to_payload_file\n --probe-args=text:SomeText\n --probe-args=hex:5061796c6f6164\0"
                as *const u8 as *const libc::c_char,
        };
        init
    }
};
unsafe extern "C" fn icmp_echo_init_perthread___0(
    mut buf: *mut libc::c_void,
    mut src: *mut macaddr_t,
    mut gw: *mut macaddr_t,
    mut dst_port: port_h_t,
    mut arg_ptr: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut eth_header: *mut ether_header = 0 as *mut ether_header;
    let mut ip_header: *mut ip = 0 as *mut ip;
    let mut len: uint16_t = 0;
    let mut tmp: __uint16_t = 0;
    let mut icmp_header: *mut icmp = 0 as *mut icmp;
    memset(buf, 0 as libc::c_int, 4096 as libc::c_int as size_t as _);
    eth_header = buf as *mut ether_header;
    make_eth_header(eth_header, src, gw);
    ip_header = eth_header.offset(1 as libc::c_int as isize) as *mut ip;
    tmp = __bswap_16(
        (::std::mem::size_of::<ip>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<icmp>() as libc::c_ulong)
            .wrapping_sub(8 as libc::c_ulong) as __uint16_t,
    );
    len = tmp;
    make_ip_header(ip_header, 1 as libc::c_int as uint8_t, len);
    icmp_header = ip_header.offset(1 as libc::c_int as isize) as *mut icmp;
    make_icmp_header(icmp_header);
    return 0 as libc::c_int;
}
unsafe extern "C" fn icmp_echo_make_packet___0(
    mut buf: *mut libc::c_void,
    mut buf_len: *mut size_t,
    mut src_ip: ipaddr_n_t,
    mut dst_ip: ipaddr_n_t,
    mut ttl: uint8_t,
    mut validation: *mut uint32_t,
    mut probe_num: libc::c_int,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    let mut eth_header: *mut ether_header = 0 as *mut ether_header;
    let mut ip_header: *mut ip = 0 as *mut ip;
    let mut icmp_header: *mut icmp = 0 as *mut icmp;
    let mut payload: *mut icmp_payload_for_rtt = 0 as *mut icmp_payload_for_rtt;
    let mut icmp_idnum: uint16_t = 0;
    let mut icmp_seqnum: uint16_t = 0;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut ip_len: size_t = 0;
    eth_header = buf as *mut ether_header;
    ip_header = eth_header.offset(1 as libc::c_int as isize) as *mut ip;
    icmp_header = ip_header.offset(1 as libc::c_int as isize) as *mut icmp;
    payload = (icmp_header as *mut libc::c_char).offset(8 as libc::c_int as isize)
        as *mut icmp_payload_for_rtt;
    icmp_idnum = (*validation.offset(1 as libc::c_int as isize) & 65535 as libc::c_uint)
        as uint16_t;
    icmp_seqnum = (*validation.offset(2 as libc::c_int as isize) & 65535 as libc::c_uint)
        as uint16_t;
    (*ip_header).ip_src.s_addr = src_ip;
    (*ip_header).ip_dst.s_addr = dst_ip;
    (*ip_header).ip_ttl = ttl;
    (*icmp_header).icmp_hun.ih_idseq.icd_id = icmp_idnum;
    (*icmp_header).icmp_hun.ih_idseq.icd_seq = icmp_seqnum;
    gettimeofday(&mut tv as *mut timeval, 0 as *mut libc::c_void);
    (*payload).sent_tv_sec = tv.tv_sec as uint32_t;
    (*payload).sent_tv_usec = tv.tv_usec as uint32_t;
    (*payload).dst = dst_ip;
    (*icmp_header).icmp_cksum = 0 as libc::c_int as uint16_t;
    (*icmp_header)
        .icmp_cksum = icmp_checksum(
        icmp_header as *mut libc::c_ushort,
        ::std::mem::size_of::<icmp>() as libc::c_ulong,
    );
    ip_len = (::std::mem::size_of::<ip>() as libc::c_ulong)
        .wrapping_add(8 as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<icmp_payload_for_rtt>() as libc::c_ulong);
    (*ip_header).ip_len = __bswap_16(ip_len as __uint16_t);
    (*ip_header).ip_sum = 0 as libc::c_int as libc::c_ushort;
    (*ip_header).ip_sum = zmap_ip_checksum(ip_header as *mut libc::c_ushort);
    *buf_len = ip_len
        .wrapping_add(::std::mem::size_of::<ether_header>() as libc::c_ulong);
    return 0 as libc::c_int;
}
unsafe extern "C" fn icmp_echo_print_packet___0(
    mut fp: *mut FILE,
    mut packet: *mut libc::c_void,
) {
    let mut ethh: *mut ether_header = 0 as *mut ether_header;
    let mut iph: *mut ip = 0 as *mut ip;
    let mut icmp_header: *mut icmp = 0 as *mut icmp;
    let mut tmp: __uint16_t = 0;
    let mut tmp___0: __uint16_t = 0;
    let mut tmp___1: __uint16_t = 0;
    ethh = packet as *mut ether_header;
    iph = ethh.offset(1 as libc::c_int as isize) as *mut ip;
    icmp_header = iph.offset(1 as libc::c_int as isize) as *mut icmp;
    tmp = __bswap_16((*icmp_header).icmp_hun.ih_idseq.icd_seq);
    tmp___0 = __bswap_16((*icmp_header).icmp_hun.ih_idseq.icd_id);
    tmp___1 = __bswap_16((*icmp_header).icmp_cksum);
    __fprintf_chk(
        fp,
        1 as libc::c_int,
        b"icmp { type: %u | code: %u | checksum: %#04X | id: %u | seq: %u }\n\0"
            as *const u8 as *const libc::c_char,
        (*icmp_header).icmp_type as libc::c_int,
        (*icmp_header).icmp_code as libc::c_int,
        tmp___1 as libc::c_int,
        tmp___0 as libc::c_int,
        tmp as libc::c_int,
    );
    fprintf_ip_header(fp, iph);
    fprintf_eth_header(fp, ethh);
    __fprintf_chk(
        fp,
        1 as libc::c_int,
        b"------------------------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
}
unsafe extern "C" fn icmp_validate_packet___0(
    mut ip_hdr: *const ip,
    mut len: uint32_t,
    mut src_ip: *mut uint32_t,
    mut validation: *mut uint32_t,
) -> libc::c_int {
    let mut icmp_h: *mut icmp = 0 as *mut icmp;
    let mut icmp_idnum: uint16_t = 0;
    let mut icmp_seqnum: uint16_t = 0;
    let mut ip_inner: *mut ip = 0 as *mut ip;
    let mut icmp_inner: *mut icmp = 0 as *mut icmp;
    if (*ip_hdr).ip_p as libc::c_int != 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (4 as libc::c_uint)
        .wrapping_mul((*ip_hdr).ip_hl())
        .wrapping_add(5 as libc::c_uint) > len
    {
        return 0 as libc::c_int;
    }
    icmp_h = (ip_hdr as *mut libc::c_char)
        .offset((4 as libc::c_uint).wrapping_mul((*ip_hdr).ip_hl()) as isize)
        as *mut icmp;
    icmp_idnum = (*icmp_h).icmp_hun.ih_idseq.icd_id;
    icmp_seqnum = (*icmp_h).icmp_hun.ih_idseq.icd_seq;
    let mut current_block_20: u64;
    if (*icmp_h).icmp_type as libc::c_int == 11 as libc::c_int {
        current_block_20 = 8185735187304638456;
    } else if (*icmp_h).icmp_type as libc::c_int == 3 as libc::c_int {
        current_block_20 = 8185735187304638456;
    } else {
        current_block_20 = 15904375183555213903;
    }
    match current_block_20 {
        8185735187304638456 => {
            if ((4 as libc::c_uint)
                .wrapping_mul((*ip_hdr).ip_hl())
                .wrapping_add(8 as libc::c_uint) as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<ip>() as libc::c_ulong)
                > len as libc::c_ulong
            {
                return 0 as libc::c_int;
            }
            ip_inner = (icmp_h as *mut libc::c_char).offset(8 as libc::c_int as isize)
                as *mut ip;
            if (4 as libc::c_uint)
                .wrapping_mul((*ip_hdr).ip_hl())
                .wrapping_add(8 as libc::c_uint)
                .wrapping_add((4 as libc::c_uint).wrapping_mul((*ip_inner).ip_hl()))
                .wrapping_add(8 as libc::c_uint) > len
            {
                return 0 as libc::c_int;
            }
            icmp_inner = (ip_inner as *mut libc::c_char)
                .offset((4 as libc::c_uint).wrapping_mul((*ip_hdr).ip_hl()) as isize)
                as *mut icmp;
            icmp_idnum = (*icmp_inner).icmp_hun.ih_idseq.icd_id;
            icmp_seqnum = (*icmp_inner).icmp_hun.ih_idseq.icd_seq;
            *src_ip = (*ip_inner).ip_dst.s_addr;
            validate_gen(
                (*ip_hdr).ip_dst.s_addr,
                (*ip_inner).ip_dst.s_addr,
                validation as *mut uint8_t,
            );
        }
        _ => {}
    }
    if icmp_idnum as libc::c_uint
        != *validation.offset(1 as libc::c_int as isize) & 65535 as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if icmp_seqnum as libc::c_uint
        != *validation.offset(2 as libc::c_int as isize) & 65535 as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn icmp_echo_process_packet___0(
    mut packet: *const u_char,
    mut len: uint32_t,
    mut fs: *mut fieldset_t,
    mut validation: *mut uint32_t,
    mut ts: timespec,
) {
    let mut ip_hdr: *mut ip = 0 as *mut ip;
    let mut icmp_hdr: *mut icmp = 0 as *mut icmp;
    let mut tmp: __uint16_t = 0;
    let mut tmp___0: __uint16_t = 0;
    let mut payload: *mut icmp_payload_for_rtt = 0 as *mut icmp_payload_for_rtt;
    ip_hdr = packet
        .offset(::std::mem::size_of::<ether_header>() as libc::c_ulong as isize)
        as *mut ip;
    icmp_hdr = (ip_hdr as *mut libc::c_char)
        .offset((4 as libc::c_uint).wrapping_mul((*ip_hdr).ip_hl()) as isize)
        as *mut icmp;
    fs_add_uint64(
        fs,
        b"type\0" as *const u8 as *const libc::c_char,
        (*icmp_hdr).icmp_type as uint64_t,
    );
    fs_add_uint64(
        fs,
        b"code\0" as *const u8 as *const libc::c_char,
        (*icmp_hdr).icmp_code as uint64_t,
    );
    tmp = __bswap_16((*icmp_hdr).icmp_hun.ih_idseq.icd_id);
    fs_add_uint64(fs, b"icmp_id\0" as *const u8 as *const libc::c_char, tmp as uint64_t);
    tmp___0 = __bswap_16((*icmp_hdr).icmp_hun.ih_idseq.icd_seq);
    fs_add_uint64(fs, b"seq\0" as *const u8 as *const libc::c_char, tmp___0 as uint64_t);
    payload = (icmp_hdr as *mut libc::c_char).offset(8 as libc::c_int as isize)
        as *mut icmp_payload_for_rtt;
    fs_add_uint64(
        fs,
        b"sent_timestamp_ts\0" as *const u8 as *const libc::c_char,
        (*payload).sent_tv_sec as uint64_t,
    );
    fs_add_uint64(
        fs,
        b"sent_timestamp_us\0" as *const u8 as *const libc::c_char,
        (*payload).sent_tv_usec as uint64_t,
    );
    fs_add_uint64(
        fs,
        b"dst_raw\0" as *const u8 as *const libc::c_char,
        (*payload).dst as uint64_t,
    );
    match (*icmp_hdr).icmp_type as libc::c_int {
        0 => {
            fs_add_string(
                fs,
                b"classification\0" as *const u8 as *const libc::c_char,
                b"echoreply\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
            );
            fs_add_uint64(
                fs,
                b"success\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as uint64_t,
            );
        }
        3 => {
            fs_add_string(
                fs,
                b"classification\0" as *const u8 as *const libc::c_char,
                b"unreach\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
            );
            fs_add_uint64(
                fs,
                b"success\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int as uint64_t,
            );
        }
        4 => {
            fs_add_string(
                fs,
                b"classification\0" as *const u8 as *const libc::c_char,
                b"sourcequench\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as libc::c_int,
            );
            fs_add_uint64(
                fs,
                b"success\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int as uint64_t,
            );
        }
        5 => {
            fs_add_string(
                fs,
                b"classification\0" as *const u8 as *const libc::c_char,
                b"redirect\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
            );
            fs_add_uint64(
                fs,
                b"success\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int as uint64_t,
            );
        }
        11 => {
            fs_add_string(
                fs,
                b"classification\0" as *const u8 as *const libc::c_char,
                b"timxceed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
            );
            fs_add_uint64(
                fs,
                b"success\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int as uint64_t,
            );
        }
        _ => {
            fs_add_string(
                fs,
                b"classification\0" as *const u8 as *const libc::c_char,
                b"other\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
            );
            fs_add_uint64(
                fs,
                b"success\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int as uint64_t,
            );
        }
    };
}
static mut fields___0: [fielddef_t; 9] = [
    {
        let mut init = field_def {
            name: b"type\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"icmp message type\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"code\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"icmp message sub type code\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_id\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"icmp id number\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"seq\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"icmp sequence number\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"sent_timestamp_ts\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"timestamp of sent probe in seconds since Epoch\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"sent_timestamp_us\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"microsecond part of sent timestamp\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dst-raw\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"raw destination IP address of sent probe\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"classification\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"probe module classification\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"success\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"did probe module classify response as success\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
];
pub static mut module_icmp_echo_time: probe_module_t = unsafe {
    {
        let mut init = probe_module {
            name: b"icmp_echo_time\0" as *const u8 as *const libc::c_char,
            max_packet_length: 62 as libc::c_int as size_t,
            pcap_filter: b"icmp and icmp[0]!=8\0" as *const u8 as *const libc::c_char,
            pcap_snaplen: 96 as libc::c_int as size_t,
            port_args: 0 as libc::c_int as uint8_t,
            global_initialize: None,
            thread_initialize: Some(
                icmp_echo_init_perthread___0
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut macaddr_t,
                        *mut macaddr_t,
                        port_h_t,
                        *mut *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            make_packet: Some(
                icmp_echo_make_packet___0
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut size_t,
                        ipaddr_n_t,
                        ipaddr_n_t,
                        uint8_t,
                        *mut uint32_t,
                        libc::c_int,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            print_packet: Some(
                icmp_echo_print_packet___0
                    as unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> (),
            ),
            validate_packet: Some(
                icmp_validate_packet___0
                    as unsafe extern "C" fn(
                        *const ip,
                        uint32_t,
                        *mut uint32_t,
                        *mut uint32_t,
                    ) -> libc::c_int,
            ),
            process_packet: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const u_char,
                        uint32_t,
                        *mut fieldset_t,
                        *mut uint32_t,
                        timespec,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *const u_char,
                        uint32_t,
                        *mut fieldset_t,
                        *mut uint32_t,
                        timespec,
                    ) -> (),
                >,
            >(
                Some(
                    icmp_echo_process_packet___0
                        as unsafe extern "C" fn(
                            *const u_char,
                            uint32_t,
                            *mut fieldset_t,
                            *mut uint32_t,
                            timespec,
                        ) -> (),
                ),
            ),
            close: ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<
                    unsafe extern "C" fn(
                        *mut state_conf,
                        *mut state_send,
                        *mut state_recv,
                    ) -> libc::c_int,
                >,
            >(0 as *const libc::c_void as *mut libc::c_void),
            output_type: 1 as libc::c_int,
            fields: fields___0.as_ptr() as *mut _,
            numfields: 9 as libc::c_int,
            helptext: 0 as *const libc::c_char,
        };
        init
    }
};
#[inline]
unsafe extern "C" fn tcp_checksum(
    mut len_tcp: libc::c_ushort,
    mut saddr: uint32_t,
    mut daddr: uint32_t,
    mut tcp_pkt: *mut tcphdr,
) -> uint16_t {
    let mut src_addr: *mut alias_unsigned_short = 0 as *mut alias_unsigned_short;
    let mut dest_addr: *mut alias_unsigned_short = 0 as *mut alias_unsigned_short;
    let mut prot_tcp: libc::c_uchar = 0;
    let mut sum: libc::c_ulong = 0;
    let mut nleft: libc::c_int = 0;
    let mut w: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    let mut tmp: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    let mut tmp___0: __uint16_t = 0;
    let mut tmp___1: __uint16_t = 0;
    let mut tmp___2: __uint16_t = 0;
    src_addr = &mut saddr as *mut uint32_t as *mut alias_unsigned_short;
    dest_addr = &mut daddr as *mut uint32_t as *mut alias_unsigned_short;
    prot_tcp = 6 as libc::c_int as libc::c_uchar;
    sum = 0 as libc::c_ulong;
    nleft = len_tcp as libc::c_int;
    w = tcp_pkt as *mut libc::c_ushort;
    while nleft > 1 as libc::c_int {
        tmp = w;
        w = w.offset(1);
        sum = sum.wrapping_add(*tmp as libc::c_ulong);
        nleft -= 2 as libc::c_int;
    }
    if nleft > 0 as libc::c_int {
        tmp___0 = __bswap_16(65280 as libc::c_int as __uint16_t);
        sum = sum
            .wrapping_add((*w as libc::c_int & tmp___0 as libc::c_int) as libc::c_ulong);
    }
    sum = sum.wrapping_add(*src_addr.offset(0 as libc::c_int as isize) as libc::c_ulong);
    sum = sum.wrapping_add(*src_addr.offset(1 as libc::c_int as isize) as libc::c_ulong);
    sum = sum
        .wrapping_add(*dest_addr.offset(0 as libc::c_int as isize) as libc::c_ulong);
    sum = sum
        .wrapping_add(*dest_addr.offset(1 as libc::c_int as isize) as libc::c_ulong);
    tmp___1 = __bswap_16(len_tcp);
    sum = sum.wrapping_add(tmp___1 as libc::c_ulong);
    tmp___2 = __bswap_16(prot_tcp as __uint16_t);
    sum = sum.wrapping_add(tmp___2 as libc::c_ulong);
    sum = (sum >> 16 as libc::c_int).wrapping_add(sum & 65535 as libc::c_ulong);
    sum = sum.wrapping_add(sum >> 16 as libc::c_int);
    return !sum as libc::c_ushort;
}
#[inline]
unsafe extern "C" fn check_dst_port(
    mut port: uint16_t,
    mut num_ports___6: libc::c_int,
    mut validation: *mut uint32_t,
) -> libc::c_int {
    let mut to_validate: int32_t = 0;
    let mut min: int32_t = 0;
    let mut max: int32_t = 0;
    if port as libc::c_int > zconf.source_port_last as libc::c_int {
        return 0 as libc::c_int
    } else {
        if (port as libc::c_int) < zconf.source_port_first as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    to_validate = port as libc::c_int - zconf.source_port_first as libc::c_int;
    min = (*validation.offset(1 as libc::c_int as isize))
        .wrapping_rem(num_ports___6 as libc::c_uint) as int32_t;
    max = (*validation.offset(1 as libc::c_int as isize))
        .wrapping_add(zconf.packet_streams as uint32_t)
        .wrapping_sub(1 as libc::c_uint)
        .wrapping_rem(num_ports___6 as libc::c_uint) as int32_t;
    return ((max - min) % num_ports___6 >= (to_validate - min) % num_ports___6)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn get_src_port(
    mut num_ports___6: libc::c_int,
    mut probe_num: libc::c_int,
    mut validation: *mut uint32_t,
) -> uint16_t {
    return (zconf.source_port_first as libc::c_uint)
        .wrapping_add(
            (*validation.offset(1 as libc::c_int as isize))
                .wrapping_add(probe_num as uint32_t)
                .wrapping_rem(num_ports___6 as libc::c_uint),
        ) as uint16_t;
}
#[inline]
unsafe extern "C" fn get_ip_header(
    mut packet: *const u_char,
    mut len: uint32_t,
) -> *mut ip {
    if (len as libc::c_ulong) < ::std::mem::size_of::<ether_header>() as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut ip;
    }
    return packet.offset(::std::mem::size_of::<ether_header>() as libc::c_ulong as isize)
        as *mut ip;
}
#[inline]
unsafe extern "C" fn get_tcp_header(
    mut ip_hdr: *const ip,
    mut len: uint32_t,
) -> *mut tcphdr {
    if ((4 as libc::c_uint).wrapping_mul((*ip_hdr).ip_hl()) as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<tcphdr>() as libc::c_ulong)
        > len as libc::c_ulong
    {
        return 0 as *mut libc::c_void as *mut tcphdr;
    }
    return (ip_hdr as *mut libc::c_char)
        .offset((4 as libc::c_uint).wrapping_mul((*ip_hdr).ip_hl()) as isize)
        as *mut tcphdr;
}
static mut num_ports: uint16_t = 0;
static mut target_port: port_h_t = 0;
unsafe extern "C" fn synscan_global_initialize(
    mut state: *mut state_conf,
) -> libc::c_int {
    num_ports = ((*state).source_port_last as libc::c_int
        - (*state).source_port_first as libc::c_int + 1 as libc::c_int) as uint16_t;
    target_port = (*state).target_port;
    return 0 as libc::c_int;
}
unsafe extern "C" fn synscan_init_perthread(
    mut buf: *mut libc::c_void,
    mut src: *mut macaddr_t,
    mut gw: *mut macaddr_t,
    mut dst_port: port_h_t,
    mut arg_ptr: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut eth_header: *mut ether_header = 0 as *mut ether_header;
    let mut ip_header: *mut ip = 0 as *mut ip;
    let mut len: uint16_t = 0;
    let mut tmp: __uint16_t = 0;
    let mut tcp_header: *mut tcphdr = 0 as *mut tcphdr;
    eth_header = buf as *mut ether_header;
    make_eth_header(eth_header, src, gw);
    ip_header = eth_header.offset(1 as libc::c_int as isize) as *mut ip;
    tmp = __bswap_16(
        (::std::mem::size_of::<ip>() as libc::c_ulong).wrapping_add(24 as libc::c_ulong)
            as __uint16_t,
    );
    len = tmp;
    make_ip_header(ip_header, 6 as libc::c_int as uint8_t, len);
    tcp_header = ip_header.offset(1 as libc::c_int as isize) as *mut tcphdr;
    make_tcp_header(tcp_header, dst_port, 2 as libc::c_int as uint16_t);
    set_mss_option(tcp_header);
    return 0 as libc::c_int;
}
unsafe extern "C" fn synscan_make_packet(
    mut buf: *mut libc::c_void,
    mut buf_len: *mut size_t,
    mut src_ip: ipaddr_n_t,
    mut dst_ip: ipaddr_n_t,
    mut ttl: uint8_t,
    mut validation: *mut uint32_t,
    mut probe_num: libc::c_int,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    let mut eth_header: *mut ether_header = 0 as *mut ether_header;
    let mut ip_header: *mut ip = 0 as *mut ip;
    let mut tcp_header: *mut tcphdr = 0 as *mut tcphdr;
    let mut tcp_seq___0: uint32_t = 0;
    let mut sport: port_h_t = 0;
    let mut tmp: uint16_t = 0;
    eth_header = buf as *mut ether_header;
    ip_header = eth_header.offset(1 as libc::c_int as isize) as *mut ip;
    tcp_header = ip_header.offset(1 as libc::c_int as isize) as *mut tcphdr;
    tcp_seq___0 = *validation.offset(0 as libc::c_int as isize);
    (*ip_header).ip_src.s_addr = src_ip;
    (*ip_header).ip_dst.s_addr = dst_ip;
    (*ip_header).ip_ttl = ttl;
    tmp = get_src_port(num_ports as libc::c_int, probe_num, validation);
    sport = tmp;
    (*tcp_header).__annonCompField8.__annonCompField6.th_sport = __bswap_16(sport);
    (*tcp_header).__annonCompField8.__annonCompField6.th_seq = tcp_seq___0;
    (*tcp_header)
        .__annonCompField8
        .__annonCompField6
        .th_sum = 0 as libc::c_int as uint16_t;
    (*tcp_header)
        .__annonCompField8
        .__annonCompField6
        .th_sum = tcp_checksum(
        24 as libc::c_int as libc::c_ushort,
        (*ip_header).ip_src.s_addr,
        (*ip_header).ip_dst.s_addr,
        tcp_header,
    );
    (*ip_header).ip_sum = 0 as libc::c_int as libc::c_ushort;
    (*ip_header).ip_sum = zmap_ip_checksum(ip_header as *mut libc::c_ushort);
    *buf_len = 58 as libc::c_int as size_t;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn synscan_print_packet(
    mut fp: *mut FILE,
    mut packet: *mut libc::c_void,
) {
    let mut ethh: *mut ether_header = 0 as *mut ether_header;
    let mut iph: *mut ip = 0 as *mut ip;
    let mut tcph: *mut tcphdr = 0 as *mut tcphdr;
    let mut tmp: __uint16_t = 0;
    let mut tmp___0: __uint32_t = 0;
    let mut tmp___1: __uint16_t = 0;
    let mut tmp___2: __uint16_t = 0;
    ethh = packet as *mut ether_header;
    iph = ethh.offset(1 as libc::c_int as isize) as *mut ip;
    tcph = iph.offset(1 as libc::c_int as isize) as *mut tcphdr;
    tmp = __bswap_16((*tcph).__annonCompField8.__annonCompField6.th_sum);
    tmp___0 = __bswap_32((*tcph).__annonCompField8.__annonCompField6.th_seq);
    tmp___1 = __bswap_16((*tcph).__annonCompField8.__annonCompField6.th_dport);
    tmp___2 = __bswap_16((*tcph).__annonCompField8.__annonCompField6.th_sport);
    __fprintf_chk(
        fp,
        1 as libc::c_int,
        b"tcp { source: %u | dest: %u | seq: %u | checksum: %#04X }\n\0" as *const u8
            as *const libc::c_char,
        tmp___2 as libc::c_int,
        tmp___1 as libc::c_int,
        tmp___0,
        tmp as libc::c_int,
    );
    fprintf_ip_header(fp, iph);
    fprintf_eth_header(fp, ethh);
    __fprintf_chk(
        fp,
        1 as libc::c_int,
        b"------------------------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
}
unsafe extern "C" fn synscan_validate_packet(
    mut ip_hdr: *const ip,
    mut len: uint32_t,
    mut src_ip: *mut uint32_t,
    mut validation: *mut uint32_t,
) -> libc::c_int {
    let mut tcp: *mut tcphdr = 0 as *mut tcphdr;
    let mut tmp: *mut tcphdr = 0 as *mut tcphdr;
    let mut sport: port_h_t = 0;
    let mut tmp___0: __uint16_t = 0;
    let mut dport: port_h_t = 0;
    let mut tmp___1: __uint16_t = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: __uint32_t = 0;
    let mut tmp___5: __uint32_t = 0;
    let mut tmp___6: __uint32_t = 0;
    let mut tmp___7: __uint32_t = 0;
    let mut tmp___8: __uint32_t = 0;
    let mut tmp___9: __uint32_t = 0;
    let mut ip_inner: *mut ip = 0 as *mut ip;
    let mut ip_inner_len: size_t = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut tcp___0: *mut tcphdr = 0 as *mut tcphdr;
    let mut tmp___11: *mut tcphdr = 0 as *mut tcphdr;
    let mut sport___0: port_h_t = 0;
    let mut tmp___12: __uint16_t = 0;
    let mut dport___0: port_h_t = 0;
    let mut tmp___13: __uint16_t = 0;
    let mut tmp___14: libc::c_int = 0;
    if (*ip_hdr).ip_p as libc::c_int == 6 as libc::c_int {
        tmp = get_tcp_header(ip_hdr, len);
        tcp = tmp;
        if tcp.is_null() {
            return 0 as libc::c_int;
        }
        tmp___0 = __bswap_16((*tcp).__annonCompField8.__annonCompField6.th_sport);
        sport = tmp___0;
        tmp___1 = __bswap_16((*tcp).__annonCompField8.__annonCompField6.th_dport);
        dport = tmp___1;
        if sport as libc::c_int != target_port as libc::c_int {
            return 0 as libc::c_int;
        }
        tmp___2 = check_dst_port(dport, num_ports as libc::c_int, validation);
        if tmp___2 == 0 {
            return 0 as libc::c_int;
        }
        tmp___3 = blocklist_is_allowed(*src_ip);
        if tmp___3 == 0 {
            return 0 as libc::c_int;
        }
        if (*tcp).__annonCompField8.__annonCompField6.th_flags as libc::c_int
            & 4 as libc::c_int != 0
        {
            tmp___4 = __bswap_32((*tcp).__annonCompField8.__annonCompField6.th_ack);
            tmp___5 = __bswap_32(*validation.offset(0 as libc::c_int as isize));
            if tmp___4 != tmp___5 {
                tmp___6 = __bswap_32((*tcp).__annonCompField8.__annonCompField6.th_ack);
                tmp___7 = __bswap_32(*validation.offset(0 as libc::c_int as isize));
                if tmp___6 != tmp___7.wrapping_add(1 as libc::c_uint) {
                    return 0 as libc::c_int;
                }
            }
        } else {
            tmp___8 = __bswap_32((*tcp).__annonCompField8.__annonCompField6.th_ack);
            tmp___9 = __bswap_32(*validation.offset(0 as libc::c_int as isize));
            if tmp___8 != tmp___9.wrapping_add(1 as libc::c_uint) {
                return 0 as libc::c_int;
            }
        }
    } else if (*ip_hdr).ip_p as libc::c_int == 1 as libc::c_int {
        tmp___10 = icmp_helper_validate(
            ip_hdr,
            len,
            ::std::mem::size_of::<tcphdr>() as libc::c_ulong,
            &mut ip_inner,
            &mut ip_inner_len,
        );
        if tmp___10 == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        tmp___11 = get_tcp_header(ip_inner as *const ip, ip_inner_len as uint32_t);
        tcp___0 = tmp___11;
        if tcp___0.is_null() {
            return 0 as libc::c_int;
        }
        tmp___12 = __bswap_16((*tcp___0).__annonCompField8.__annonCompField6.th_sport);
        sport___0 = tmp___12;
        tmp___13 = __bswap_16((*tcp___0).__annonCompField8.__annonCompField6.th_dport);
        dport___0 = tmp___13;
        if dport___0 as libc::c_int != target_port as libc::c_int {
            return 0 as libc::c_int;
        }
        validate_gen(
            (*ip_hdr).ip_dst.s_addr,
            (*ip_inner).ip_dst.s_addr,
            validation as *mut uint8_t,
        );
        tmp___14 = check_dst_port(sport___0, num_ports as libc::c_int, validation);
        if tmp___14 == 0 {
            return 0 as libc::c_int;
        }
    } else {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn synscan_process_packet(
    mut packet: *const u_char,
    mut len: uint32_t,
    mut fs: *mut fieldset_t,
    mut validation: *mut uint32_t,
    mut ts: timespec,
) {
    let mut ip_hdr: *mut ip = 0 as *mut ip;
    let mut tmp: *mut ip = 0 as *mut ip;
    let mut tcp: *mut tcphdr = 0 as *mut tcphdr;
    let mut tmp___1: *mut tcphdr = 0 as *mut tcphdr;
    let mut tmp___3: __uint16_t = 0;
    let mut tmp___4: __uint16_t = 0;
    let mut tmp___5: __uint32_t = 0;
    let mut tmp___6: __uint32_t = 0;
    let mut tmp___7: __uint16_t = 0;
    tmp = get_ip_header(packet, len);
    ip_hdr = tmp;
    if ip_hdr.is_null() {
        __assert_fail(
            b"ip_hdr\0" as *const u8 as *const libc::c_char,
            b"src/probe_modules/module_tcp_synscan.c\0" as *const u8
                as *const libc::c_char,
            175 as libc::c_uint,
            b"synscan_process_packet\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*ip_hdr).ip_p as libc::c_int == 6 as libc::c_int {
        tmp___1 = get_tcp_header(ip_hdr as *const ip, len);
        tcp = tmp___1;
        if tcp.is_null() {
            __assert_fail(
                b"tcp\0" as *const u8 as *const libc::c_char,
                b"src/probe_modules/module_tcp_synscan.c\0" as *const u8
                    as *const libc::c_char,
                178 as libc::c_uint,
                b"synscan_process_packet\0" as *const u8 as *const libc::c_char,
            );
        }
        tmp___3 = __bswap_16((*tcp).__annonCompField8.__annonCompField6.th_sport);
        fs_add_uint64(
            fs,
            b"sport\0" as *const u8 as *const libc::c_char,
            tmp___3 as uint64_t,
        );
        tmp___4 = __bswap_16((*tcp).__annonCompField8.__annonCompField6.th_dport);
        fs_add_uint64(
            fs,
            b"dport\0" as *const u8 as *const libc::c_char,
            tmp___4 as uint64_t,
        );
        tmp___5 = __bswap_32((*tcp).__annonCompField8.__annonCompField6.th_seq);
        fs_add_uint64(
            fs,
            b"seqnum\0" as *const u8 as *const libc::c_char,
            tmp___5 as uint64_t,
        );
        tmp___6 = __bswap_32((*tcp).__annonCompField8.__annonCompField6.th_ack);
        fs_add_uint64(
            fs,
            b"acknum\0" as *const u8 as *const libc::c_char,
            tmp___6 as uint64_t,
        );
        tmp___7 = __bswap_16((*tcp).__annonCompField8.__annonCompField6.th_win);
        fs_add_uint64(
            fs,
            b"window\0" as *const u8 as *const libc::c_char,
            tmp___7 as uint64_t,
        );
        if (*tcp).__annonCompField8.__annonCompField6.th_flags as libc::c_int
            & 4 as libc::c_int != 0
        {
            fs_add_constchar(
                fs,
                b"classification\0" as *const u8 as *const libc::c_char,
                b"rst\0" as *const u8 as *const libc::c_char,
            );
            fs_add_bool(
                fs,
                b"success\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        } else {
            fs_add_constchar(
                fs,
                b"classification\0" as *const u8 as *const libc::c_char,
                b"synack\0" as *const u8 as *const libc::c_char,
            );
            fs_add_bool(
                fs,
                b"success\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            );
        }
        fs_add_null_icmp(fs);
    } else if (*ip_hdr).ip_p as libc::c_int == 1 as libc::c_int {
        fs_add_null(fs, b"sport\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"dport\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"seqnum\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"acknum\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"window\0" as *const u8 as *const libc::c_char);
        fs_add_constchar(
            fs,
            b"classification\0" as *const u8 as *const libc::c_char,
            b"icmp\0" as *const u8 as *const libc::c_char,
        );
        fs_add_bool(
            fs,
            b"success\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        fs_populate_icmp_from_iphdr(ip_hdr, len as size_t, fs);
    }
}
static mut fields___1: [fielddef_t; 11] = [
    {
        let mut init = field_def {
            name: b"sport\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"TCP source port\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dport\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"TCP destination port\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"seqnum\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"TCP sequence number\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"acknum\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"TCP acknowledgement number\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"window\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"TCP window\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"classification\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"packet classification\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"success\0" as *const u8 as *const libc::c_char,
            type_0: b"bool\0" as *const u8 as *const libc::c_char,
            desc: b"is response considered success\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_responder\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"Source IP of ICMP_UNREACH messages\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_type\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"icmp message type\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_code\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"icmp message sub type code\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_unreach_str\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"for icmp_unreach responses, the string version of icmp_code (e.g. network-unreach)\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
];
pub static mut module_tcp_synscan: probe_module_t = probe_module_t {
    name: 0 as *const libc::c_char,
    max_packet_length: 0,
    pcap_filter: 0 as *const libc::c_char,
    pcap_snaplen: 0,
    port_args: 0,
    global_initialize: None,
    thread_initialize: None,
    make_packet: None,
    print_packet: None,
    validate_packet: None,
    process_packet: None,
    close: None,
    output_type: 0,
    fields: 0 as *const fielddef_t as *mut fielddef_t,
    numfields: 0,
    helptext: 0 as *const libc::c_char,
};
static mut num_ports___0: uint32_t = 0;
unsafe extern "C" fn synackscan_global_initialize(
    mut state: *mut state_conf,
) -> libc::c_int {
    num_ports___0 = ((*state).source_port_last as libc::c_int
        - (*state).source_port_first as libc::c_int + 1 as libc::c_int) as uint32_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn synackscan_init_perthread(
    mut buf: *mut libc::c_void,
    mut src: *mut macaddr_t,
    mut gw: *mut macaddr_t,
    mut dst_port: port_h_t,
    mut arg_ptr: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut eth_header: *mut ether_header = 0 as *mut ether_header;
    let mut ip_header: *mut ip = 0 as *mut ip;
    let mut len: uint16_t = 0;
    let mut tmp: __uint16_t = 0;
    let mut tcp_header: *mut tcphdr = 0 as *mut tcphdr;
    memset(buf, 0 as libc::c_int, 4096 as libc::c_int as size_t as _);
    eth_header = buf as *mut ether_header;
    make_eth_header(eth_header, src, gw);
    ip_header = eth_header.offset(1 as libc::c_int as isize) as *mut ip;
    tmp = __bswap_16(
        (::std::mem::size_of::<ip>() as libc::c_ulong).wrapping_add(24 as libc::c_ulong)
            as __uint16_t,
    );
    len = tmp;
    make_ip_header(ip_header, 6 as libc::c_int as uint8_t, len);
    tcp_header = ip_header.offset(1 as libc::c_int as isize) as *mut tcphdr;
    make_tcp_header(tcp_header, dst_port, 18 as libc::c_int as uint16_t);
    set_mss_option(tcp_header);
    return 0 as libc::c_int;
}
unsafe extern "C" fn synackscan_make_packet(
    mut buf: *mut libc::c_void,
    mut buf_len: *mut size_t,
    mut src_ip: ipaddr_n_t,
    mut dst_ip: ipaddr_n_t,
    mut ttl: uint8_t,
    mut validation: *mut uint32_t,
    mut probe_num: libc::c_int,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    let mut eth_header: *mut ether_header = 0 as *mut ether_header;
    let mut ip_header: *mut ip = 0 as *mut ip;
    let mut tcp_header: *mut tcphdr = 0 as *mut tcphdr;
    let mut tcp_seq___0: uint32_t = 0;
    let mut tcp_ack: uint32_t = 0;
    let mut tmp: uint16_t = 0;
    eth_header = buf as *mut ether_header;
    ip_header = eth_header.offset(1 as libc::c_int as isize) as *mut ip;
    tcp_header = ip_header.offset(1 as libc::c_int as isize) as *mut tcphdr;
    tcp_seq___0 = *validation.offset(0 as libc::c_int as isize);
    tcp_ack = *validation.offset(2 as libc::c_int as isize);
    (*ip_header).ip_src.s_addr = src_ip;
    (*ip_header).ip_dst.s_addr = dst_ip;
    (*ip_header).ip_ttl = ttl;
    tmp = get_src_port(num_ports___0 as libc::c_int, probe_num, validation);
    (*tcp_header).__annonCompField8.__annonCompField6.th_sport = __bswap_16(tmp);
    (*tcp_header).__annonCompField8.__annonCompField6.th_seq = tcp_seq___0;
    (*tcp_header).__annonCompField8.__annonCompField6.th_ack = tcp_ack;
    (*tcp_header)
        .__annonCompField8
        .__annonCompField6
        .th_sum = 0 as libc::c_int as uint16_t;
    (*tcp_header)
        .__annonCompField8
        .__annonCompField6
        .th_sum = tcp_checksum(
        24 as libc::c_int as libc::c_ushort,
        (*ip_header).ip_src.s_addr,
        (*ip_header).ip_dst.s_addr,
        tcp_header,
    );
    (*ip_header).ip_sum = 0 as libc::c_int as libc::c_ushort;
    (*ip_header).ip_sum = zmap_ip_checksum(ip_header as *mut libc::c_ushort);
    *buf_len = 58 as libc::c_int as size_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn synackscan_validate_packet(
    mut ip_hdr: *const ip,
    mut len: uint32_t,
    mut src_ip: *mut uint32_t,
    mut validation: *mut uint32_t,
) -> libc::c_int {
    let mut tcp: *mut tcphdr = 0 as *mut tcphdr;
    let mut tmp: *mut tcphdr = 0 as *mut tcphdr;
    let mut sport: uint16_t = 0;
    let mut tmp___0: __uint16_t = 0;
    let mut dport: uint16_t = 0;
    let mut tmp___1: __uint16_t = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: __uint32_t = 0;
    let mut tmp___5: __uint32_t = 0;
    let mut tmp___6: __uint32_t = 0;
    let mut tmp___7: __uint32_t = 0;
    let mut tmp___8: __uint32_t = 0;
    let mut tmp___9: __uint32_t = 0;
    let mut tmp___10: __uint32_t = 0;
    let mut tmp___11: __uint32_t = 0;
    let mut ip_inner: *mut ip = 0 as *mut ip;
    let mut ip_inner_len: size_t = 0;
    let mut tmp___12: libc::c_int = 0;
    let mut tcp___0: *mut tcphdr = 0 as *mut tcphdr;
    let mut tmp___13: *mut tcphdr = 0 as *mut tcphdr;
    let mut sport___0: uint16_t = 0;
    let mut tmp___14: __uint16_t = 0;
    let mut dport___0: uint16_t = 0;
    let mut tmp___15: __uint16_t = 0;
    let mut tmp___16: libc::c_int = 0;
    if (*ip_hdr).ip_p as libc::c_int == 6 as libc::c_int {
        tmp = get_tcp_header(ip_hdr, len);
        tcp = tmp;
        if tcp.is_null() {
            return 0 as libc::c_int;
        }
        tmp___0 = __bswap_16((*tcp).__annonCompField8.__annonCompField6.th_sport);
        sport = tmp___0;
        tmp___1 = __bswap_16((*tcp).__annonCompField8.__annonCompField6.th_dport);
        dport = tmp___1;
        if sport as libc::c_int != zconf.target_port as libc::c_int {
            return 0 as libc::c_int;
        }
        tmp___2 = check_dst_port(dport, num_ports___0 as libc::c_int, validation);
        if tmp___2 == 0 {
            return 0 as libc::c_int;
        }
        tmp___3 = blocklist_is_allowed(*src_ip);
        if tmp___3 == 0 {
            return 0 as libc::c_int;
        }
        if (*tcp).__annonCompField8.__annonCompField6.th_flags as libc::c_int
            & 4 as libc::c_int != 0
        {
            tmp___4 = __bswap_32((*tcp).__annonCompField8.__annonCompField6.th_ack);
            tmp___5 = __bswap_32(*validation.offset(0 as libc::c_int as isize));
            if tmp___4 != tmp___5.wrapping_add(1 as libc::c_uint) {
                tmp___6 = __bswap_32((*tcp).__annonCompField8.__annonCompField6.th_seq);
                tmp___7 = __bswap_32(*validation.offset(2 as libc::c_int as isize));
                if tmp___6 != tmp___7 {
                    tmp___8 = __bswap_32(
                        (*tcp).__annonCompField8.__annonCompField6.th_seq,
                    );
                    tmp___9 = __bswap_32(*validation.offset(2 as libc::c_int as isize));
                    if tmp___8 != tmp___9.wrapping_add(1 as libc::c_uint) {
                        return 0 as libc::c_int;
                    }
                }
            }
        } else {
            tmp___10 = __bswap_32((*tcp).__annonCompField8.__annonCompField6.th_ack);
            tmp___11 = __bswap_32(*validation.offset(0 as libc::c_int as isize));
            if tmp___10 != tmp___11.wrapping_add(1 as libc::c_uint) {
                return 0 as libc::c_int;
            }
        }
    } else if (*ip_hdr).ip_p as libc::c_int == 1 as libc::c_int {
        tmp___12 = icmp_helper_validate(
            ip_hdr,
            len,
            ::std::mem::size_of::<udphdr>() as libc::c_ulong,
            &mut ip_inner,
            &mut ip_inner_len,
        );
        if tmp___12 == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        tmp___13 = get_tcp_header(ip_inner as *const ip, ip_inner_len as uint32_t);
        tcp___0 = tmp___13;
        if tcp___0.is_null() {
            return 0 as libc::c_int;
        }
        tmp___14 = __bswap_16((*tcp___0).__annonCompField8.__annonCompField6.th_sport);
        sport___0 = tmp___14;
        tmp___15 = __bswap_16((*tcp___0).__annonCompField8.__annonCompField6.th_dport);
        dport___0 = tmp___15;
        if dport___0 as libc::c_int != zconf.target_port as libc::c_int {
            return 0 as libc::c_int;
        }
        validate_gen(
            (*ip_hdr).ip_dst.s_addr,
            (*ip_inner).ip_dst.s_addr,
            validation as *mut uint8_t,
        );
        tmp___16 = check_dst_port(sport___0, num_ports___0 as libc::c_int, validation);
        if tmp___16 == 0 {
            return 0 as libc::c_int;
        }
    } else {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn synackscan_process_packet(
    mut packet: *const u_char,
    mut len: uint32_t,
    mut fs: *mut fieldset_t,
    mut validation: *mut uint32_t,
    mut ts: timespec,
) {
    let mut ip_hdr: *mut ip = 0 as *mut ip;
    let mut tmp: *mut ip = 0 as *mut ip;
    let mut tcp: *mut tcphdr = 0 as *mut tcphdr;
    let mut tmp___0: *mut tcphdr = 0 as *mut tcphdr;
    let mut tmp___1: __uint16_t = 0;
    let mut tmp___2: __uint16_t = 0;
    let mut tmp___3: __uint32_t = 0;
    let mut tmp___4: __uint32_t = 0;
    let mut tmp___5: __uint16_t = 0;
    tmp = get_ip_header(packet, len);
    ip_hdr = tmp;
    if (*ip_hdr).ip_p as libc::c_int == 6 as libc::c_int {
        tmp___0 = get_tcp_header(ip_hdr as *const ip, len);
        tcp = tmp___0;
        tmp___1 = __bswap_16((*tcp).__annonCompField8.__annonCompField6.th_sport);
        fs_add_uint64(
            fs,
            b"sport\0" as *const u8 as *const libc::c_char,
            tmp___1 as uint64_t,
        );
        tmp___2 = __bswap_16((*tcp).__annonCompField8.__annonCompField6.th_dport);
        fs_add_uint64(
            fs,
            b"dport\0" as *const u8 as *const libc::c_char,
            tmp___2 as uint64_t,
        );
        tmp___3 = __bswap_32((*tcp).__annonCompField8.__annonCompField6.th_seq);
        fs_add_uint64(
            fs,
            b"seqnum\0" as *const u8 as *const libc::c_char,
            tmp___3 as uint64_t,
        );
        tmp___4 = __bswap_32((*tcp).__annonCompField8.__annonCompField6.th_ack);
        fs_add_uint64(
            fs,
            b"acknum\0" as *const u8 as *const libc::c_char,
            tmp___4 as uint64_t,
        );
        tmp___5 = __bswap_16((*tcp).__annonCompField8.__annonCompField6.th_win);
        fs_add_uint64(
            fs,
            b"window\0" as *const u8 as *const libc::c_char,
            tmp___5 as uint64_t,
        );
        if (*tcp).__annonCompField8.__annonCompField6.th_flags as libc::c_int
            & 4 as libc::c_int != 0
        {
            fs_add_constchar(
                fs,
                b"classification\0" as *const u8 as *const libc::c_char,
                b"rst\0" as *const u8 as *const libc::c_char,
            );
        } else {
            fs_add_constchar(
                fs,
                b"classification\0" as *const u8 as *const libc::c_char,
                b"synack\0" as *const u8 as *const libc::c_char,
            );
        }
        fs_add_bool(
            fs,
            b"success\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
        fs_add_null_icmp(fs);
    } else if (*ip_hdr).ip_p as libc::c_int == 1 as libc::c_int {
        fs_add_null(fs, b"sport\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"dport\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"seqnum\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"acknum\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"window\0" as *const u8 as *const libc::c_char);
        fs_add_constchar(
            fs,
            b"classification\0" as *const u8 as *const libc::c_char,
            b"icmp\0" as *const u8 as *const libc::c_char,
        );
        fs_add_bool(
            fs,
            b"success\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        fs_populate_icmp_from_iphdr(ip_hdr, len as size_t, fs);
    }
}
static mut fields___2: [fielddef_t; 11] = [
    {
        let mut init = field_def {
            name: b"sport\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"TCP source port\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dport\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"TCP destination port\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"seqnum\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"TCP sequence number\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"acknum\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"TCP acknowledgement number\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"window\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"TCP window\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"classification\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"packet classification\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"success\0" as *const u8 as *const libc::c_char,
            type_0: b"bool\0" as *const u8 as *const libc::c_char,
            desc: b"is response considered success\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_responder\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"Source IP of ICMP_UNREACH messages\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_type\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"icmp message type\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_code\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"icmp message sub type code\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_unreach_str\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"for icmp_unreach responses, the string version of icmp_code (e.g. network-unreach)\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
];
pub static mut module_tcp_synackscan: probe_module_t = probe_module_t {
    name: 0 as *const libc::c_char,
    max_packet_length: 0,
    pcap_filter: 0 as *const libc::c_char,
    pcap_snaplen: 0,
    port_args: 0,
    global_initialize: None,
    thread_initialize: None,
    make_packet: None,
    print_packet: None,
    validate_packet: None,
    process_packet: None,
    close: None,
    output_type: 0,
    fields: 0 as *const fielddef_t as *mut fielddef_t,
    numfields: 0,
    helptext: 0 as *const libc::c_char,
};
#[inline]
unsafe extern "C" fn get_udp_header(
    mut ip_hdr: *const ip,
    mut len: uint32_t,
) -> *mut udphdr {
    if ((4 as libc::c_uint).wrapping_mul((*ip_hdr).ip_hl()) as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong)
        > len as libc::c_ulong
    {
        return 0 as *mut libc::c_void as *mut udphdr;
    }
    return (ip_hdr as *mut libc::c_char)
        .offset((4 as libc::c_uint).wrapping_mul((*ip_hdr).ip_hl()) as isize)
        as *mut udphdr;
}
static mut udp_fixed_payload: *mut uint8_t = 0 as *const libc::c_void
    as *mut libc::c_void as *mut uint8_t;
static mut udp_fixed_payload_len: size_t = 0 as libc::c_int as size_t;
static mut udp_template: *mut udp_payload_template_t = 0 as *const libc::c_void
    as *mut libc::c_void as *mut udp_payload_template_t;
pub static mut udp_usage_error: *const libc::c_char = b"unknown UDP probe specification (expected file:/path or text:STRING or hex:01020304 or template:/path or template-fields)\0"
    as *const u8 as *const libc::c_char;
pub static mut charset_alphanum: *const libc::c_uchar = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789\0"
    as *const u8 as *const libc::c_char as *mut libc::c_uchar as *const libc::c_uchar;
pub static mut charset_alpha: *const libc::c_uchar = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ\0"
    as *const u8 as *const libc::c_char as *mut libc::c_uchar as *const libc::c_uchar;
pub static mut charset_digit: *const libc::c_uchar = b"0123456789\0" as *const u8
    as *const libc::c_char as *mut libc::c_uchar as *const libc::c_uchar;
pub static mut charset_all: [libc::c_uchar; 257] = [
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
    0 as libc::c_int as libc::c_uchar,
    0,
];
static mut num_ports___1: libc::c_int = 0;
static mut udp_num_template_field_types: uint32_t = 12 as libc::c_int as uint32_t;
static mut udp_payload_template_fields: [udp_payload_field_type_def_t; 12] = [
    {
        let mut init = udp_payload_field_type_def {
            name: b"SADDR_N\0" as *const u8 as *const libc::c_char,
            desc: b"Source IP address in network byte order\0" as *const u8
                as *const libc::c_char,
            max_length: 4 as libc::c_int as size_t,
            ftype: UDP_SADDR_N,
        };
        init
    },
    {
        let mut init = udp_payload_field_type_def {
            name: b"SADDR\0" as *const u8 as *const libc::c_char,
            desc: b"Source IP address in dotted-quad format\0" as *const u8
                as *const libc::c_char,
            max_length: 15 as libc::c_int as size_t,
            ftype: UDP_SADDR_A,
        };
        init
    },
    {
        let mut init = udp_payload_field_type_def {
            name: b"DADDR_N\0" as *const u8 as *const libc::c_char,
            desc: b"Destination IP address in network byte order\0" as *const u8
                as *const libc::c_char,
            max_length: 4 as libc::c_int as size_t,
            ftype: UDP_DADDR_N,
        };
        init
    },
    {
        let mut init = udp_payload_field_type_def {
            name: b"DADDR\0" as *const u8 as *const libc::c_char,
            desc: b"Destination IP address in dotted-quad format\0" as *const u8
                as *const libc::c_char,
            max_length: 15 as libc::c_int as size_t,
            ftype: UDP_DADDR_A,
        };
        init
    },
    {
        let mut init = udp_payload_field_type_def {
            name: b"SPORT_N\0" as *const u8 as *const libc::c_char,
            desc: b"UDP source port in netowrk byte order\0" as *const u8
                as *const libc::c_char,
            max_length: 2 as libc::c_int as size_t,
            ftype: UDP_SPORT_N,
        };
        init
    },
    {
        let mut init = udp_payload_field_type_def {
            name: b"SPORT\0" as *const u8 as *const libc::c_char,
            desc: b"UDP source port in ascii format\0" as *const u8
                as *const libc::c_char,
            max_length: 5 as libc::c_int as size_t,
            ftype: UDP_SPORT_A,
        };
        init
    },
    {
        let mut init = udp_payload_field_type_def {
            name: b"DPORT_N\0" as *const u8 as *const libc::c_char,
            desc: b"UDP destination port in network byte order\0" as *const u8
                as *const libc::c_char,
            max_length: 2 as libc::c_int as size_t,
            ftype: UDP_DPORT_N,
        };
        init
    },
    {
        let mut init = udp_payload_field_type_def {
            name: b"DPORT\0" as *const u8 as *const libc::c_char,
            desc: b"UDP destination port in ascii format\0" as *const u8
                as *const libc::c_char,
            max_length: 5 as libc::c_int as size_t,
            ftype: UDP_DPORT_A,
        };
        init
    },
    {
        let mut init = udp_payload_field_type_def {
            name: b"RAND_BYTE\0" as *const u8 as *const libc::c_char,
            desc: b"Random bytes from 0-255\0" as *const u8 as *const libc::c_char,
            max_length: 0 as libc::c_int as size_t,
            ftype: UDP_RAND_BYTE,
        };
        init
    },
    {
        let mut init = udp_payload_field_type_def {
            name: b"RAND_DIGIT\0" as *const u8 as *const libc::c_char,
            desc: b"Random digits from 0-9\0" as *const u8 as *const libc::c_char,
            max_length: 0 as libc::c_int as size_t,
            ftype: UDP_RAND_DIGIT,
        };
        init
    },
    {
        let mut init = udp_payload_field_type_def {
            name: b"RAND_ALPHA\0" as *const u8 as *const libc::c_char,
            desc: b"Random mixed-case letters (a-z)\0" as *const u8
                as *const libc::c_char,
            max_length: 0 as libc::c_int as size_t,
            ftype: UDP_RAND_ALPHA,
        };
        init
    },
    {
        let mut init = udp_payload_field_type_def {
            name: b"RAND_ALPHANUM\0" as *const u8 as *const libc::c_char,
            desc: b"Random mixed-case letters (a-z) and numbers\0" as *const u8
                as *const libc::c_char,
            max_length: 0 as libc::c_int as size_t,
            ftype: UDP_RAND_ALPHANUM,
        };
        init
    },
];
pub unsafe extern "C" fn udp_set_num_ports(mut x: libc::c_int) {
    num_ports___1 = x;
}
pub unsafe extern "C" fn udp_global_initialize(
    mut conf: *mut state_conf,
) -> libc::c_int {
    let mut udp_template_max_len: uint32_t = 0;
    let mut args: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: uint32_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut c: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut arg_name_len: size_t = 0;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut tmp___3: *mut FILE = 0 as *mut FILE;
    let mut in_0: [uint8_t; 1472] = [0; 1472];
    let mut f___0: *mut FILE = 0 as *mut FILE;
    let mut tmp___4: *mut FILE = 0 as *mut FILE;
    let mut in_len: size_t = 0;
    let mut tmp___5: size_t = 0;
    let mut tmp___6: size_t = 0;
    let mut tmp___7: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut n: libc::c_uint = 0;
    let mut i___0: size_t = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: libc::c_int = 0;
    let mut header_len: size_t = 0;
    udp_template_max_len = 0 as libc::c_int as uint32_t;
    num_ports___1 = (*conf).source_port_last as libc::c_int
        - (*conf).source_port_first as libc::c_int + 1 as libc::c_int;
    if ((*conf).probe_args).is_null() {
        log_error(
            b"udp\0" as *const u8 as *const libc::c_char,
            b"%s\0" as *const u8 as *const libc::c_char,
            b"--probe-args are required, run --probe-module=udp --help for a longer description of the arguments\0"
                as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    args = (*conf).probe_args as *const libc::c_char;
    tmp = strcmp(args, b"template-fields\0" as *const u8 as *const libc::c_char);
    if tmp == 0 as libc::c_int {
        lock_file(stderr);
        __fprintf_chk(
            stderr,
            1 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            b"List of allowed UDP template fields (name: description)\n\n\0" as *const u8
                as *const libc::c_char,
        );
        i = 0 as libc::c_int as uint32_t;
        while i < udp_num_template_field_types {
            __fprintf_chk(
                stderr,
                1 as libc::c_int,
                b"%s: %s\n\0" as *const u8 as *const libc::c_char,
                udp_payload_template_fields[i as usize].name,
                udp_payload_template_fields[i as usize].desc,
            );
            i = i.wrapping_add(1);
        }
        __fprintf_chk(
            stderr,
            1 as libc::c_int,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
        fflush(stderr);
        unlock_file(stderr);
        exit(0 as libc::c_int);
    }
    tmp___0 = strchr(args, ':' as i32);
    c = tmp___0 as *const libc::c_char;
    if c.is_null() {
        log_fatal(b"udp\0" as *const u8 as *const libc::c_char, udp_usage_error);
    }
    arg_name_len = c.offset_from(args) as libc::c_long as size_t;
    c = c.offset(1);
    tmp___12 = strncmp(
        args,
        b"text\0" as *const u8 as *const libc::c_char,
        arg_name_len,
    );
    if tmp___12 == 0 as libc::c_int {
        tmp___1 = strdup(c);
        udp_fixed_payload = tmp___1 as *mut uint8_t;
        udp_fixed_payload_len = strlen(c);
    } else {
        tmp___11 = strncmp(
            args,
            b"file\0" as *const u8 as *const libc::c_char,
            arg_name_len,
        );
        if tmp___11 == 0 as libc::c_int {
            tmp___2 = xmalloc(1472 as libc::c_int as size_t);
            udp_fixed_payload = tmp___2 as *mut uint8_t;
            tmp___3 = fopen(c, b"rb\0" as *const u8 as *const libc::c_char);
            f = tmp___3;
            if f.is_null() {
                log_fatal(
                    b"udp\0" as *const u8 as *const libc::c_char,
                    b"could not open UDP data file '%s'\n\0" as *const u8
                        as *const libc::c_char,
                    c,
                );
            }
            udp_fixed_payload_len = fread(
                udp_fixed_payload as *mut libc::c_void,
                1 as libc::c_int as size_t,
                1472 as libc::c_int as size_t,
                f,
            );
            fclose(f);
        } else {
            tmp___10 = strncmp(
                args,
                b"template\0" as *const u8 as *const libc::c_char,
                arg_name_len,
            );
            if tmp___10 == 0 as libc::c_int {
                tmp___4 = fopen(c, b"rb\0" as *const u8 as *const libc::c_char);
                f___0 = tmp___4;
                if f___0.is_null() {
                    log_fatal(
                        b"udp\0" as *const u8 as *const libc::c_char,
                        b"could not open UDP data file '%s'\n\0" as *const u8
                            as *const libc::c_char,
                        c,
                    );
                }
                tmp___5 = fread(
                    in_0.as_mut_ptr() as *mut libc::c_void,
                    1 as libc::c_int as size_t,
                    1472 as libc::c_int as size_t,
                    f___0,
                );
                in_len = tmp___5;
                fclose(f___0);
                udp_template = udp_template_load(
                    in_0.as_mut_ptr(),
                    in_len as uint32_t,
                    &mut udp_template_max_len,
                );
                module_udp
                    .make_packet = Some(
                    udp_make_templated_packet
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut size_t,
                            ipaddr_n_t,
                            ipaddr_n_t,
                            uint8_t,
                            *mut uint32_t,
                            libc::c_int,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                );
            } else {
                tmp___9 = strncmp(
                    args,
                    b"hex\0" as *const u8 as *const libc::c_char,
                    arg_name_len,
                );
                if tmp___9 == 0 as libc::c_int {
                    tmp___6 = strlen(c);
                    udp_fixed_payload_len = tmp___6.wrapping_div(2 as libc::c_ulong);
                    tmp___7 = xmalloc(udp_fixed_payload_len);
                    udp_fixed_payload = tmp___7 as *mut uint8_t;
                    i___0 = 0 as libc::c_int as size_t;
                    while i___0 < udp_fixed_payload_len {
                        tmp___8 = sscanf(
                            c.offset(i___0.wrapping_mul(2 as libc::c_ulong) as isize),
                            b"%2x\0" as *const u8 as *const libc::c_char,
                            &mut n as *mut libc::c_uint,
                        );
                        if tmp___8 != 1 as libc::c_int {
                            log_fatal(
                                b"udp\0" as *const u8 as *const libc::c_char,
                                b"non-hex character: '%c'\0" as *const u8
                                    as *const libc::c_char,
                                *c.offset(i___0.wrapping_mul(2 as libc::c_ulong) as isize)
                                    as libc::c_int,
                            );
                        }
                        *udp_fixed_payload
                            .offset(
                                i___0 as isize,
                            ) = (n & 255 as libc::c_uint) as uint8_t;
                        i___0 = i___0.wrapping_add(1);
                    }
                } else {
                    log_fatal(
                        b"udp\0" as *const u8 as *const libc::c_char,
                        udp_usage_error,
                    );
                }
            }
        }
    }
    if udp_fixed_payload_len > 1472 as libc::c_ulong {
        log_warn(
            b"udp\0" as *const u8 as *const libc::c_char,
            b"warning: reducing fixed UDP payload to %d bytes (from %d) to fit on the wire\n\0"
                as *const u8 as *const libc::c_char,
            1472 as libc::c_int,
            udp_fixed_payload_len,
        );
        udp_fixed_payload_len = 1472 as libc::c_int as size_t;
    }
    header_len = (::std::mem::size_of::<ether_header>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ip>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong);
    if udp_fixed_payload_len > 0 as libc::c_ulong {
        module_udp.max_packet_length = header_len.wrapping_add(udp_fixed_payload_len);
    } else if udp_template_max_len > 0 as libc::c_uint {
        module_udp
            .max_packet_length = header_len.wrapping_add(udp_template_max_len as size_t);
    }
    if module_udp.max_packet_length == 0 {
        __assert_fail(
            b"module_udp.max_packet_length\0" as *const u8 as *const libc::c_char,
            b"src/probe_modules/module_udp.c\0" as *const u8 as *const libc::c_char,
            224 as libc::c_uint,
            b"udp_global_initialize\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(module_udp.max_packet_length <= 4096 as libc::c_ulong) {
        __assert_fail(
            b"module_udp.max_packet_length <= MAX_PACKET_SIZE\0" as *const u8
                as *const libc::c_char,
            b"src/probe_modules/module_udp.c\0" as *const u8 as *const libc::c_char,
            225 as libc::c_uint,
            b"udp_global_initialize\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn udp_global_cleanup(
    mut zconf___0: *mut state_conf,
    mut zsend___0: *mut state_send,
    mut zrecv___0: *mut state_recv,
) -> libc::c_int {
    if !udp_fixed_payload.is_null() {
        free(udp_fixed_payload as *mut libc::c_void);
        udp_fixed_payload = 0 as *mut libc::c_void as *mut uint8_t;
    }
    if !udp_template.is_null() {
        udp_template_free(udp_template);
        udp_template = 0 as *mut libc::c_void as *mut udp_payload_template_t;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn udp_init_perthread(
    mut buf: *mut libc::c_void,
    mut src: *mut macaddr_t,
    mut gw: *mut macaddr_t,
    mut dst_port: port_h_t,
    mut arg_ptr: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut eth_header: *mut ether_header = 0 as *mut ether_header;
    let mut ip_header: *mut ip = 0 as *mut ip;
    let mut ip_len: uint16_t = 0;
    let mut tmp: __uint16_t = 0;
    let mut udp_header: *mut udphdr = 0 as *mut udphdr;
    let mut udp_len: uint16_t = 0;
    let mut payload: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut seed: uint32_t = 0;
    let mut tmp___0: uint64_t = 0;
    let mut aes: *mut aesrand_t = 0 as *mut aesrand_t;
    let mut tmp___1: *mut aesrand_t = 0 as *mut aesrand_t;
    memset(buf, 0 as libc::c_int, 4096 as libc::c_int as size_t as _);
    eth_header = buf as *mut ether_header;
    make_eth_header(eth_header, src, gw);
    ip_header = eth_header.offset(1 as libc::c_int as isize) as *mut ip;
    tmp = __bswap_16(
        (::std::mem::size_of::<ip>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong)
            .wrapping_add(udp_fixed_payload_len) as __uint16_t,
    );
    ip_len = tmp;
    make_ip_header(ip_header, 17 as libc::c_int as uint8_t, ip_len);
    udp_header = ip_header.offset(1 as libc::c_int as isize) as *mut udphdr;
    udp_len = (::std::mem::size_of::<udphdr>() as libc::c_ulong)
        .wrapping_add(udp_fixed_payload_len) as uint16_t;
    make_udp_header(udp_header, zconf.target_port, udp_len);
    if !udp_fixed_payload.is_null() {
        payload = udp_header.offset(1 as libc::c_int as isize) as *mut libc::c_void;
        memcpy(payload, udp_fixed_payload as *const libc::c_void, udp_fixed_payload_len as _);
    }
    tmp___0 = aesrand_getword(zconf.aes);
    seed = tmp___0 as uint32_t;
    tmp___1 = aesrand_init_from_seed(seed as uint64_t);
    aes = tmp___1;
    *arg_ptr = aes as *mut libc::c_void;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn udp_make_packet(
    mut buf: *mut libc::c_void,
    mut buf_len: *mut size_t,
    mut src_ip: ipaddr_n_t,
    mut dst_ip: ipaddr_n_t,
    mut ttl: uint8_t,
    mut validation: *mut uint32_t,
    mut probe_num: libc::c_int,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    let mut eth_header: *mut ether_header = 0 as *mut ether_header;
    let mut ip_header: *mut ip = 0 as *mut ip;
    let mut udp_header: *mut udphdr = 0 as *mut udphdr;
    let mut headers_len: size_t = 0;
    let mut tmp: uint16_t = 0;
    eth_header = buf as *mut ether_header;
    ip_header = eth_header.offset(1 as libc::c_int as isize) as *mut ip;
    udp_header = ip_header.offset(1 as libc::c_int as isize) as *mut udphdr;
    headers_len = (::std::mem::size_of::<ether_header>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ip>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong);
    (*ip_header).ip_src.s_addr = src_ip;
    (*ip_header).ip_dst.s_addr = dst_ip;
    (*ip_header).ip_ttl = ttl;
    tmp = get_src_port(num_ports___1, probe_num, validation);
    (*udp_header).__annonCompField5.__annonCompField3.uh_sport = __bswap_16(tmp);
    (*ip_header).ip_sum = 0 as libc::c_int as libc::c_ushort;
    (*ip_header).ip_sum = zmap_ip_checksum(ip_header as *mut libc::c_ushort);
    *buf_len = headers_len.wrapping_add(udp_fixed_payload_len);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn udp_make_templated_packet(
    mut buf: *mut libc::c_void,
    mut buf_len: *mut size_t,
    mut src_ip: ipaddr_n_t,
    mut dst_ip: ipaddr_n_t,
    mut ttl: uint8_t,
    mut validation: *mut uint32_t,
    mut probe_num: libc::c_int,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    let mut eth_header: *mut ether_header = 0 as *mut ether_header;
    let mut ip_header: *mut ip = 0 as *mut ip;
    let mut udp_header: *mut udphdr = 0 as *mut udphdr;
    let mut headers_len: size_t = 0;
    let mut tmp: uint16_t = 0;
    let mut payload: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut aes: *mut aesrand_t = 0 as *mut aesrand_t;
    let mut payload_len: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    eth_header = buf as *mut ether_header;
    ip_header = eth_header.offset(1 as libc::c_int as isize) as *mut ip;
    udp_header = ip_header.offset(1 as libc::c_int as isize) as *mut udphdr;
    headers_len = (::std::mem::size_of::<ether_header>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ip>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong);
    (*ip_header).ip_src.s_addr = src_ip;
    (*ip_header).ip_dst.s_addr = dst_ip;
    (*ip_header).ip_ttl = ttl;
    tmp = get_src_port(num_ports___1, probe_num, validation);
    (*udp_header).__annonCompField5.__annonCompField3.uh_sport = __bswap_16(tmp);
    payload = udp_header.offset(1 as libc::c_int as isize) as *mut libc::c_char;
    memset(
        payload as *mut libc::c_void,
        0 as libc::c_int,
        1472 as libc::c_int as size_t as _,
    );
    aes = arg as *mut aesrand_t;
    tmp___0 = 0;
    payload_len = tmp___0;
    if payload_len <= 0 as libc::c_int {
        log_fatal(
            b"udp\0" as *const u8 as *const libc::c_char,
            b"UDP payload template generated an empty payload\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*ip_header)
        .ip_len = __bswap_16(
        (::std::mem::size_of::<ip>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong)
            .wrapping_add(payload_len as libc::c_ulong) as __uint16_t,
    );
    (*udp_header)
        .__annonCompField5
        .__annonCompField3
        .uh_ulen = __bswap_16(
        (::std::mem::size_of::<udphdr>() as libc::c_ulong)
            .wrapping_add(payload_len as libc::c_ulong) as __uint16_t,
    );
    (*ip_header).ip_sum = 0 as libc::c_int as libc::c_ushort;
    (*ip_header).ip_sum = zmap_ip_checksum(ip_header as *mut libc::c_ushort);
    *buf_len = headers_len.wrapping_add(payload_len as size_t);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn udp_print_packet(
    mut fp: *mut FILE,
    mut packet: *mut libc::c_void,
) {
    let mut ethh: *mut ether_header = 0 as *mut ether_header;
    let mut iph: *mut ip = 0 as *mut ip;
    let mut udph: *mut udphdr = 0 as *mut udphdr;
    let mut tmp: __uint16_t = 0;
    let mut tmp___0: __uint16_t = 0;
    let mut tmp___1: __uint16_t = 0;
    ethh = packet as *mut ether_header;
    iph = ethh.offset(1 as libc::c_int as isize) as *mut ip;
    udph = iph.offset(1 as libc::c_int as isize) as *mut udphdr;
    tmp = __bswap_16((*udph).__annonCompField5.__annonCompField3.uh_sum);
    tmp___0 = __bswap_16((*udph).__annonCompField5.__annonCompField3.uh_dport);
    tmp___1 = __bswap_16((*udph).__annonCompField5.__annonCompField3.uh_sport);
    __fprintf_chk(
        fp,
        1 as libc::c_int,
        b"udp { source: %u | dest: %u | checksum: %#04X }\n\0" as *const u8
            as *const libc::c_char,
        tmp___1 as libc::c_int,
        tmp___0 as libc::c_int,
        tmp as libc::c_int,
    );
    fprintf_ip_header(fp, iph);
    fprintf_eth_header(fp, ethh);
    __fprintf_chk(
        fp,
        1 as libc::c_int,
        b"------------------------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
}
pub unsafe extern "C" fn udp_process_packet(
    mut packet: *const u_char,
    mut len: uint32_t,
    mut fs: *mut fieldset_t,
    mut validation: *mut uint32_t,
    mut ts: timespec,
) {
    let mut ip_hdr: *mut ip = 0 as *mut ip;
    let mut udp: *mut udphdr = 0 as *mut udphdr;
    let mut tmp: *mut udphdr = 0 as *mut udphdr;
    let mut tmp___0: __uint16_t = 0;
    let mut tmp___1: __uint16_t = 0;
    let mut tmp___2: __uint16_t = 0;
    let mut data_len: uint16_t = 0;
    let mut tmp___3: __uint16_t = 0;
    let mut overhead: uint32_t = 0;
    let mut max_rlen: uint32_t = 0;
    let mut max_ilen: uint32_t = 0;
    let mut tmp___4: __uint16_t = 0;
    ip_hdr = packet
        .offset(::std::mem::size_of::<ether_header>() as libc::c_ulong as isize)
        as *mut ip;
    if (*ip_hdr).ip_p as libc::c_int == 17 as libc::c_int {
        tmp = get_udp_header(ip_hdr as *const ip, len);
        udp = tmp;
        fs_add_constchar(
            fs,
            b"classification\0" as *const u8 as *const libc::c_char,
            b"udp\0" as *const u8 as *const libc::c_char,
        );
        fs_add_bool(
            fs,
            b"success\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
        tmp___0 = __bswap_16((*udp).__annonCompField5.__annonCompField3.uh_sport);
        fs_add_uint64(
            fs,
            b"sport\0" as *const u8 as *const libc::c_char,
            tmp___0 as uint64_t,
        );
        tmp___1 = __bswap_16((*udp).__annonCompField5.__annonCompField3.uh_dport);
        fs_add_uint64(
            fs,
            b"dport\0" as *const u8 as *const libc::c_char,
            tmp___1 as uint64_t,
        );
        tmp___2 = __bswap_16((*udp).__annonCompField5.__annonCompField3.uh_ulen);
        fs_add_uint64(
            fs,
            b"udp_pkt_size\0" as *const u8 as *const libc::c_char,
            tmp___2 as uint64_t,
        );
        tmp___3 = __bswap_16((*udp).__annonCompField5.__annonCompField3.uh_ulen);
        data_len = tmp___3;
        if data_len as libc::c_ulong > ::std::mem::size_of::<udphdr>() as libc::c_ulong {
            overhead = (::std::mem::size_of::<udphdr>() as libc::c_ulong)
                .wrapping_add(
                    (*ip_hdr).ip_hl().wrapping_mul(4 as libc::c_uint) as libc::c_ulong,
                ) as uint32_t;
            max_rlen = len.wrapping_sub(overhead);
            tmp___4 = __bswap_16((*ip_hdr).ip_len);
            max_ilen = (tmp___4 as uint32_t).wrapping_sub(overhead);
            if data_len as uint32_t > max_rlen {
                data_len = max_rlen as uint16_t;
            }
            if data_len as uint32_t > max_ilen {
                data_len = max_ilen as uint16_t;
            }
            fs_add_binary(
                fs,
                b"data\0" as *const u8 as *const libc::c_char,
                data_len as size_t,
                udp.offset(1 as libc::c_int as isize) as *mut libc::c_void,
                0 as libc::c_int,
            );
        } else {
            fs_add_null(fs, b"data\0" as *const u8 as *const libc::c_char);
        }
        fs_add_null_icmp(fs);
    } else if (*ip_hdr).ip_p as libc::c_int == 1 as libc::c_int {
        fs_add_constchar(
            fs,
            b"classification\0" as *const u8 as *const libc::c_char,
            b"icmp\0" as *const u8 as *const libc::c_char,
        );
        fs_add_bool(
            fs,
            b"success\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        fs_add_null(fs, b"sport\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"dport\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"udp_pkt_size\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"data\0" as *const u8 as *const libc::c_char);
        fs_populate_icmp_from_iphdr(ip_hdr, len as size_t, fs);
    } else {
        fs_add_constchar(
            fs,
            b"classification\0" as *const u8 as *const libc::c_char,
            b"other\0" as *const u8 as *const libc::c_char,
        );
        fs_add_bool(
            fs,
            b"success\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        fs_add_null(fs, b"sport\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"dport\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"udp_pkt_size\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"data\0" as *const u8 as *const libc::c_char);
        fs_add_null_icmp(fs);
    };
}
pub unsafe extern "C" fn udp_validate_packet(
    mut ip_hdr: *const ip,
    mut len: uint32_t,
    mut src_ip: *mut uint32_t,
    mut validation: *mut uint32_t,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = udp_do_validate_packet(
        ip_hdr,
        len,
        src_ip,
        validation,
        num_ports___1,
        -(1 as libc::c_int),
    );
    return tmp;
}
pub unsafe extern "C" fn udp_do_validate_packet(
    mut ip_hdr: *const ip,
    mut len: uint32_t,
    mut src_ip: *mut uint32_t,
    mut validation: *mut uint32_t,
    mut num_ports___6: libc::c_int,
    mut expected_port: libc::c_int,
) -> libc::c_int {
    let mut udp: *mut udphdr = 0 as *mut udphdr;
    let mut tmp: *mut udphdr = 0 as *mut udphdr;
    let mut dport: uint16_t = 0;
    let mut tmp___0: __uint16_t = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut ep: uint16_t = 0;
    let mut sport: uint16_t = 0;
    let mut tmp___3: __uint16_t = 0;
    let mut ip_inner: *mut ip = 0 as *mut ip;
    let mut ip_inner_len: size_t = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut udp___0: *mut udphdr = 0 as *mut udphdr;
    let mut tmp___5: *mut udphdr = 0 as *mut udphdr;
    let mut dport___0: uint16_t = 0;
    let mut tmp___6: __uint16_t = 0;
    let mut sport___0: uint16_t = 0;
    let mut tmp___7: __uint16_t = 0;
    let mut tmp___8: libc::c_int = 0;
    if (*ip_hdr).ip_p as libc::c_int == 17 as libc::c_int {
        tmp = get_udp_header(ip_hdr, len);
        udp = tmp;
        if udp.is_null() {
            return 0 as libc::c_int;
        }
        tmp___0 = __bswap_16((*udp).__annonCompField5.__annonCompField3.uh_dport);
        dport = tmp___0;
        tmp___1 = check_dst_port(dport, num_ports___6, validation);
        if tmp___1 == 0 {
            return 0 as libc::c_int;
        }
        tmp___2 = blocklist_is_allowed(*src_ip);
        if tmp___2 == 0 {
            return 0 as libc::c_int;
        }
        if expected_port != -(1 as libc::c_int) {
            ep = expected_port as uint16_t;
            tmp___3 = __bswap_16((*udp).__annonCompField5.__annonCompField3.uh_sport);
            sport = tmp___3;
            if sport as libc::c_int != ep as libc::c_int {
                return 0 as libc::c_int;
            }
        }
    } else if (*ip_hdr).ip_p as libc::c_int == 1 as libc::c_int {
        tmp___4 = icmp_helper_validate(
            ip_hdr,
            len,
            ::std::mem::size_of::<udphdr>() as libc::c_ulong,
            &mut ip_inner,
            &mut ip_inner_len,
        );
        tmp___5 = get_udp_header(ip_inner as *const ip, ip_inner_len as uint32_t);
        udp___0 = tmp___5;
        tmp___6 = __bswap_16((*udp___0).__annonCompField5.__annonCompField3.uh_dport);
        dport___0 = tmp___6;
        tmp___7 = __bswap_16((*udp___0).__annonCompField5.__annonCompField3.uh_sport);
        sport___0 = tmp___7;
        if dport___0 as libc::c_int != zconf.target_port as libc::c_int {
            return 0 as libc::c_int;
        }
        tmp___8 = check_dst_port(sport___0, num_ports___6, validation);
        if tmp___8 == 0 {
            return 0 as libc::c_int;
        }
    } else {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn udp_template_add_field(
    mut t: *mut udp_payload_template_t,
    mut ftype: udp_payload_field_type_t,
    mut length: libc::c_uint,
    mut data: *mut libc::c_char,
) {
    let mut c: *mut udp_payload_field_t = 0 as *mut udp_payload_field_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    (*t).fcount = ((*t).fcount).wrapping_add(1);
    tmp = xrealloc(
        (*t).fields as *mut libc::c_void,
        (::std::mem::size_of::<udp_payload_field_t>() as libc::c_ulong)
            .wrapping_mul((*t).fcount as libc::c_ulong),
    );
    (*t).fields = tmp as *mut *mut udp_payload_field;
    tmp___0 = xmalloc(::std::mem::size_of::<udp_payload_field_t>() as libc::c_ulong);
    let ref mut fresh14 = *((*t).fields)
        .offset(((*t).fcount).wrapping_sub(1 as libc::c_uint) as isize);
    *fresh14 = tmp___0 as *mut udp_payload_field;
    c = *((*t).fields).offset(((*t).fcount).wrapping_sub(1 as libc::c_uint) as isize);
    if c.is_null() {
        __assert_fail(
            b"c\0" as *const u8 as *const libc::c_char,
            b"src/probe_modules/module_udp.c\0" as *const u8 as *const libc::c_char,
            486 as libc::c_uint,
            b"udp_template_add_field\0" as *const u8 as *const libc::c_char,
        );
    }
    (*c).ftype = ftype;
    (*c).length = length as size_t;
    (*c).data = data;
}
pub unsafe extern "C" fn udp_template_free(mut t: *mut udp_payload_template_t) {
    let mut x: libc::c_uint = 0;
    x = 0 as libc::c_uint;
    while x < (*t).fcount {
        if !((**((*t).fields).offset(x as isize)).data).is_null() {
            free((**((*t).fields).offset(x as isize)).data as *mut libc::c_void);
            let ref mut fresh15 = (**((*t).fields).offset(x as isize)).data;
            *fresh15 = 0 as *mut libc::c_void as *mut libc::c_char;
        }
        free(*((*t).fields).offset(x as isize) as *mut libc::c_void);
        let ref mut fresh16 = *((*t).fields).offset(x as isize);
        *fresh16 = 0 as *mut libc::c_void as *mut udp_payload_field;
        x = x.wrapping_add(1);
    }
    free((*t).fields as *mut libc::c_void);
    (*t).fields = 0 as *mut libc::c_void as *mut *mut udp_payload_field;
    (*t).fcount = 0 as libc::c_uint;
    free(t as *mut libc::c_void);
}
pub unsafe extern "C" fn udp_random_bytes(
    mut dst: *mut libc::c_char,
    mut len: libc::c_int,
    mut charset: *const libc::c_uchar,
    mut charset_len: libc::c_int,
    mut aes: *mut aesrand_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: uint64_t = 0;
    i = 0 as libc::c_int;
    while i < len {
        tmp = dst;
        dst = dst.offset(1);
        tmp___0 = aesrand_getword(aes);
        *tmp = *charset
            .offset(
                (tmp___0 & 4294967295 as libc::c_ulong)
                    .wrapping_rem(charset_len as libc::c_ulong) as isize,
            ) as libc::c_char;
        i += 1;
    }
    return i;
}
static mut fcount: libc::c_ulong = 0;
pub unsafe extern "C" fn udp_template_field_lookup(
    mut vname: *const libc::c_char,
    mut c: *mut udp_payload_field_t,
) -> libc::c_int {
    let mut vname_len: size_t = 0;
    let mut tmp: size_t = 0;
    let mut type_name_len: size_t = 0;
    let mut param: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut olen: libc::c_long = 0;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut f: libc::c_uint = 0;
    let mut ftype: *const udp_payload_field_type_def_t = 0
        as *const udp_payload_field_type_def_t;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: size_t = 0;
    tmp = strlen(vname);
    vname_len = tmp;
    type_name_len = vname_len;
    tmp___0 = strstr(vname, b"=\0" as *const u8 as *const libc::c_char);
    param = tmp___0 as *const libc::c_char;
    if !param.is_null() {
        type_name_len = param.offset_from(vname) as libc::c_long as size_t;
        param = param.offset(1);
    }
    olen = 0 as libc::c_long;
    if !param.is_null() {
        if *param == 0 {
            log_fatal(
                b"udp\0" as *const u8 as *const libc::c_char,
                b"invalid template: field spec %s is invalid (missing length)\0"
                    as *const u8 as *const libc::c_char,
                vname,
            );
        }
    }
    if !param.is_null() {
        end = 0 as *mut libc::c_void as *mut libc::c_char;
        tmp___1 = __errno_location();
        *tmp___1 = 0 as libc::c_int;
        olen = strtol(param, &mut end as *mut *mut libc::c_char, 10 as libc::c_int);
        tmp___4 = __errno_location();
        if *tmp___4 != 0 {
            tmp___2 = __errno_location();
            tmp___3 = strerror(*tmp___2);
            log_fatal(
                b"udp\0" as *const u8 as *const libc::c_char,
                b"invalid template: unable to read length from %s: %s\0" as *const u8
                    as *const libc::c_char,
                vname,
                tmp___3,
            );
        }
        if end.is_null() {
            log_fatal(
                b"udp\0" as *const u8 as *const libc::c_char,
                b"invalid template: unable to read length from %s\0" as *const u8
                    as *const libc::c_char,
                vname,
            );
        } else {
            if end as libc::c_ulong != vname.offset(vname_len as isize) as libc::c_ulong
            {
                log_fatal(
                    b"udp\0" as *const u8 as *const libc::c_char,
                    b"invalid template: unable to read length from %s\0" as *const u8
                        as *const libc::c_char,
                    vname,
                );
            }
        }
        if olen < 0 as libc::c_long {
            log_fatal(
                b"udp\0" as *const u8 as *const libc::c_char,
                b"invalid template: field size %d is larger than the max (%d)\0"
                    as *const u8 as *const libc::c_char,
                olen,
                1472 as libc::c_int,
            );
        } else {
            if olen > 1472 as libc::c_long {
                log_fatal(
                    b"udp\0" as *const u8 as *const libc::c_char,
                    b"invalid template: field size %d is larger than the max (%d)\0"
                        as *const u8 as *const libc::c_char,
                    olen,
                    1472 as libc::c_int,
                );
            }
        }
    }
    f = 0 as libc::c_uint;
    while (f as libc::c_ulong) < fcount {
        ftype = &mut *udp_payload_template_fields.as_mut_ptr().offset(f as isize)
            as *mut udp_payload_field_type_def_t as *const udp_payload_field_type_def_t;
        tmp___5 = strncmp(vname, (*ftype).name, type_name_len);
        if tmp___5 == 0 as libc::c_int {
            tmp___6 = strlen((*ftype).name);
            if tmp___6 == type_name_len {
                (*c).ftype = (*ftype).ftype;
                if (*ftype).max_length != 0 {
                    (*c).length = (*ftype).max_length;
                } else {
                    (*c).length = olen as size_t;
                }
                (*c).data = 0 as *mut libc::c_void as *mut libc::c_char;
                return 1 as libc::c_int;
            }
        }
        f = f.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn udp_template_load(
    mut buf: *mut uint8_t,
    mut buf_len: uint32_t,
    mut max_pkt_len: *mut uint32_t,
) -> *mut udp_payload_template_t {
    let mut t: *mut udp_payload_template_t = 0 as *mut udp_payload_template_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _max_pkt_len: uint32_t = 0;
    let mut dollar: *mut uint8_t = 0 as *mut uint8_t;
    let mut lbrack: *mut uint8_t = 0 as *mut uint8_t;
    let mut s: *mut uint8_t = 0 as *mut uint8_t;
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tlen: libc::c_uint = 0;
    let mut c: udp_payload_field_t = udp_payload_field_t {
        ftype: UDP_DATA,
        length: 0,
        data: 0 as *mut libc::c_char,
    };
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = xmalloc(::std::mem::size_of::<udp_payload_template_t>() as libc::c_ulong);
    t = tmp as *mut udp_payload_template_t;
    _max_pkt_len = 0 as libc::c_int as uint32_t;
    dollar = 0 as *mut libc::c_void as *mut uint8_t;
    lbrack = 0 as *mut libc::c_void as *mut uint8_t;
    s = buf;
    p = buf;
    (*t).fcount = 0 as libc::c_uint;
    (*t).fields = 0 as *mut libc::c_void as *mut *mut udp_payload_field;
    while (p as libc::c_ulong) < buf.offset(buf_len as isize) as libc::c_ulong {
        match *p as libc::c_int {
            36 => {
                let mut current_block_13: u64;
                if !dollar.is_null() {
                    if lbrack.is_null() {
                        dollar = p;
                        current_block_13 = 12124785117276362961;
                    } else {
                        current_block_13 = 7043273740668273692;
                    }
                } else {
                    current_block_13 = 7043273740668273692;
                }
                match current_block_13 {
                    7043273740668273692 => {
                        if dollar.is_null() {
                            dollar = p;
                        }
                    }
                    _ => {}
                }
                p = p.offset(1);
                continue;
            }
            123 => {
                if !dollar.is_null() {
                    if lbrack.is_null() {
                        lbrack = p;
                    }
                }
                p = p.offset(1);
                continue;
            }
            125 => {
                if !dollar.is_null() {
                    if lbrack.is_null() {
                        p = p.offset(1);
                        continue;
                    } else {
                        tlen = dollar.offset_from(s) as libc::c_long as libc::c_uint;
                        if tlen > 0 as libc::c_uint {
                            tmp___1 = xmalloc(tlen as size_t);
                            tmp___0 = tmp___1 as *mut libc::c_char;
                            memcpy(
                                tmp___0 as *mut libc::c_void,
                                s as *const libc::c_void,
                                tlen as size_t as _,
                            );
                            udp_template_add_field(t, UDP_DATA, tlen, tmp___0);
                            _max_pkt_len = (_max_pkt_len as libc::c_uint)
                                .wrapping_add(tlen) as uint32_t as uint32_t;
                        }
                        tmp___2 = xcalloc(
                            1 as libc::c_int as size_t,
                            p.offset_from(lbrack) as libc::c_long as size_t,
                        );
                        tmp___0 = tmp___2 as *mut libc::c_char;
                        memcpy(
                            tmp___0 as *mut libc::c_void,
                            lbrack.offset(1 as libc::c_int as isize)
                                as *const libc::c_void,
                            (p.offset_from(lbrack) as libc::c_long - 1 as libc::c_long)
                                as size_t as _,
                        );
                        tmp___3 = udp_template_field_lookup(
                            tmp___0 as *const libc::c_char,
                            &mut c,
                        );
                        if tmp___3 != 0 {
                            udp_template_add_field(
                                t,
                                c.ftype,
                                c.length as libc::c_uint,
                                c.data,
                            );
                            _max_pkt_len = (_max_pkt_len as size_t)
                                .wrapping_add(c.length) as uint32_t;
                            s = p.offset(1 as libc::c_int as isize);
                        } else {
                            s = dollar;
                        }
                        free(tmp___0 as *mut libc::c_void);
                    }
                } else {
                    p = p.offset(1);
                    continue;
                }
            }
            _ => {
                if !dollar.is_null() {
                    if !lbrack.is_null() {
                        p = p.offset(1);
                        continue;
                    }
                }
            }
        }
        dollar = 0 as *mut libc::c_void as *mut uint8_t;
        lbrack = 0 as *mut libc::c_void as *mut uint8_t;
        p = p.offset(1);
    }
    if (s as libc::c_ulong) < p as libc::c_ulong {
        tlen = p.offset_from(s) as libc::c_long as libc::c_uint;
        tmp___4 = xmalloc(tlen as size_t);
        tmp___0 = tmp___4 as *mut libc::c_char;
        memcpy(tmp___0 as *mut libc::c_void, s as *const libc::c_void, tlen as size_t as _);
        udp_template_add_field(t, UDP_DATA, tlen, tmp___0);
        _max_pkt_len = (_max_pkt_len as libc::c_uint).wrapping_add(tlen) as uint32_t
            as uint32_t;
    }
    *max_pkt_len = _max_pkt_len;
    return t;
}
static mut fields___3: [fielddef_t; 10] = [
    {
        let mut init = field_def {
            name: b"classification\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"packet classification\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"success\0" as *const u8 as *const libc::c_char,
            type_0: b"bool\0" as *const u8 as *const libc::c_char,
            desc: b"is response considered success\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"sport\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"UDP source port\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dport\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"UDP destination port\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"udp_pkt_size\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"UDP packet length\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"data\0" as *const u8 as *const libc::c_char,
            type_0: b"binary\0" as *const u8 as *const libc::c_char,
            desc: b"UDP payload\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_responder\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"Source IP of ICMP_UNREACH messages\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_type\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"icmp message type\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_code\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"icmp message sub type code\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_unreach_str\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"for icmp_unreach responses, the string version of icmp_code (e.g. network-unreach)\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
];
pub static mut module_udp: probe_module_t = probe_module_t {
    name: 0 as *const libc::c_char,
    max_packet_length: 0,
    pcap_filter: 0 as *const libc::c_char,
    pcap_snaplen: 0,
    port_args: 0,
    global_initialize: None,
    thread_initialize: None,
    make_packet: None,
    print_packet: None,
    validate_packet: None,
    process_packet: None,
    close: None,
    output_type: 0,
    fields: 0 as *const fielddef_t as *mut fielddef_t,
    numfields: 0,
    helptext: 0 as *const libc::c_char,
};
#[inline]
unsafe extern "C" fn get_inner_ip_header(
    mut icmp: *const icmp,
    mut len: uint32_t,
) -> *mut ip {
    if (len as libc::c_ulong)
        < (8 as libc::c_ulong).wrapping_add(::std::mem::size_of::<ip>() as libc::c_ulong)
    {
        return 0 as *mut libc::c_void as *mut ip;
    }
    return (icmp as *mut libc::c_char).offset(8 as libc::c_int as isize) as *mut ip;
}
pub unsafe extern "C" fn print_macaddr(mut i: *mut ifreq) {
    __printf_chk(
        1 as libc::c_int,
        b"Device %s -> Ethernet %02x:%02x:%02x:%02x:%02x:%02x\n\0" as *const u8
            as *const libc::c_char,
        ((*i).ifr_ifrn.ifrn_name).as_mut_ptr(),
        *(&mut (*i).ifr_ifru.ifru_addr.sa_data as *mut [libc::c_char; 14]
            as *mut libc::c_uchar)
            .offset(0 as libc::c_int as isize) as libc::c_int,
        *(&mut (*i).ifr_ifru.ifru_addr.sa_data as *mut [libc::c_char; 14]
            as *mut libc::c_uchar)
            .offset(1 as libc::c_int as isize) as libc::c_int,
        *(&mut (*i).ifr_ifru.ifru_addr.sa_data as *mut [libc::c_char; 14]
            as *mut libc::c_uchar)
            .offset(2 as libc::c_int as isize) as libc::c_int,
        *(&mut (*i).ifr_ifru.ifru_addr.sa_data as *mut [libc::c_char; 14]
            as *mut libc::c_uchar)
            .offset(3 as libc::c_int as isize) as libc::c_int,
        *(&mut (*i).ifr_ifru.ifru_addr.sa_data as *mut [libc::c_char; 14]
            as *mut libc::c_uchar)
            .offset(4 as libc::c_int as isize) as libc::c_int,
        *(&mut (*i).ifr_ifru.ifru_addr.sa_data as *mut [libc::c_char; 14]
            as *mut libc::c_uchar)
            .offset(5 as libc::c_int as isize) as libc::c_int,
    );
}
pub unsafe extern "C" fn fprintf_ip_header(mut fp: *mut FILE, mut iph: *mut ip) {
    let mut s: *mut in_addr = 0 as *mut in_addr;
    let mut d: *mut in_addr = 0 as *mut in_addr;
    let mut srcip: [libc::c_char; 21] = [0; 21];
    let mut dstip: [libc::c_char; 21] = [0; 21];
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: __uint16_t = 0;
    s = &mut (*iph).ip_src;
    d = &mut (*iph).ip_dst;
    tmp = inet_ntoa(*s);
    strncpy(srcip.as_mut_ptr(), tmp as *const libc::c_char, 19 as libc::c_int as size_t as _);
    tmp___0 = inet_ntoa(*d);
    strncpy(
        dstip.as_mut_ptr(),
        tmp___0 as *const libc::c_char,
        19 as libc::c_int as size_t as _,
    );
    srcip[20 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    dstip[20 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    tmp___1 = __bswap_16((*iph).ip_sum);
    __fprintf_chk(
        fp,
        1 as libc::c_int,
        b"ip { saddr: %s | daddr: %s | checksum: %#04X }\n\0" as *const u8
            as *const libc::c_char,
        srcip.as_mut_ptr(),
        dstip.as_mut_ptr(),
        tmp___1 as libc::c_int,
    );
}
pub unsafe extern "C" fn fprintf_eth_header(
    mut fp: *mut FILE,
    mut ethh: *mut ether_header,
) {
    if zconf.send_ip_pkts == 0 {
        __fprintf_chk(
            fp,
            1 as libc::c_int,
            b"eth { shost: %02x:%02x:%02x:%02x:%02x:%02x | dhost: %02x:%02x:%02x:%02x:%02x:%02x }\n\0"
                as *const u8 as *const libc::c_char,
            (*ethh).ether_shost[0 as libc::c_int as usize] as libc::c_int,
            (*ethh).ether_shost[1 as libc::c_int as usize] as libc::c_int,
            (*ethh).ether_shost[2 as libc::c_int as usize] as libc::c_int,
            (*ethh).ether_shost[3 as libc::c_int as usize] as libc::c_int,
            (*ethh).ether_shost[4 as libc::c_int as usize] as libc::c_int,
            (*ethh).ether_shost[5 as libc::c_int as usize] as libc::c_int,
            (*ethh).ether_dhost[0 as libc::c_int as usize] as libc::c_int,
            (*ethh).ether_dhost[1 as libc::c_int as usize] as libc::c_int,
            (*ethh).ether_dhost[2 as libc::c_int as usize] as libc::c_int,
            (*ethh).ether_dhost[3 as libc::c_int as usize] as libc::c_int,
            (*ethh).ether_dhost[4 as libc::c_int as usize] as libc::c_int,
            (*ethh).ether_dhost[5 as libc::c_int as usize] as libc::c_int,
        );
    }
}
pub unsafe extern "C" fn make_eth_header(
    mut ethh: *mut ether_header,
    mut src: *mut macaddr_t,
    mut dst: *mut macaddr_t,
) {
    memcpy(
        ((*ethh).ether_shost).as_mut_ptr() as *mut libc::c_void,
        src as *const libc::c_void,
        6 as libc::c_int as size_t as _,
    );
    memcpy(
        ((*ethh).ether_dhost).as_mut_ptr() as *mut libc::c_void,
        dst as *const libc::c_void,
        6 as libc::c_int as size_t as _,
    );
    (*ethh).ether_type = __bswap_16(2048 as libc::c_int as __uint16_t);
}
pub unsafe extern "C" fn make_ip_header(
    mut iph: *mut ip,
    mut protocol: uint8_t,
    mut len: uint16_t,
) {
    (*iph).set_ip_hl(5 as libc::c_uint);
    (*iph).set_ip_v(4 as libc::c_uint);
    (*iph).ip_tos = 0 as libc::c_int as uint8_t;
    (*iph).ip_len = len;
    (*iph).ip_id = __bswap_16(54321 as libc::c_int as __uint16_t);
    (*iph).ip_off = 0 as libc::c_int as libc::c_ushort;
    (*iph).ip_ttl = 255 as libc::c_int as uint8_t;
    (*iph).ip_p = protocol;
    (*iph).ip_sum = 0 as libc::c_int as libc::c_ushort;
}
pub unsafe extern "C" fn make_icmp_header(mut buf: *mut icmp) {
    memset(
        buf as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<icmp>() as libc::c_ulong as _,
    );
    (*buf).icmp_type = 8 as libc::c_int as uint8_t;
    (*buf).icmp_code = 0 as libc::c_int as uint8_t;
    (*buf).icmp_hun.ih_idseq.icd_seq = 0 as libc::c_int as uint16_t;
}
pub unsafe extern "C" fn make_tcp_header(
    mut tcp_header: *mut tcphdr,
    mut dest_port: port_h_t,
    mut th_flags: uint16_t,
) {
    let mut tmp: libc::c_long = 0;
    tmp = random();
    (*tcp_header).__annonCompField8.__annonCompField6.th_seq = tmp as tcp_seq;
    (*tcp_header)
        .__annonCompField8
        .__annonCompField6
        .th_ack = 0 as libc::c_int as tcp_seq;
    ((*tcp_header).__annonCompField8.__annonCompField6)
        .set_th_x2(0 as libc::c_int as uint8_t);
    ((*tcp_header).__annonCompField8.__annonCompField6)
        .set_th_off(5 as libc::c_int as uint8_t);
    (*tcp_header)
        .__annonCompField8
        .__annonCompField6
        .th_flags = 0 as libc::c_int as uint8_t;
    (*tcp_header)
        .__annonCompField8
        .__annonCompField6
        .th_flags = ((*tcp_header).__annonCompField8.__annonCompField6.th_flags
        as libc::c_int | th_flags as libc::c_int) as uint8_t;
    (*tcp_header)
        .__annonCompField8
        .__annonCompField6
        .th_win = __bswap_16(65535 as libc::c_int as __uint16_t);
    (*tcp_header)
        .__annonCompField8
        .__annonCompField6
        .th_sum = 0 as libc::c_int as uint16_t;
    (*tcp_header)
        .__annonCompField8
        .__annonCompField6
        .th_urp = 0 as libc::c_int as uint16_t;
    (*tcp_header).__annonCompField8.__annonCompField6.th_dport = __bswap_16(dest_port);
}
pub unsafe extern "C" fn set_mss_option(mut tcp_header: *mut tcphdr) -> size_t {
    let mut header_size: size_t = 0;
    let mut base: *mut uint8_t = 0 as *mut uint8_t;
    let mut last_opt: *mut uint8_t = 0 as *mut uint8_t;
    header_size = (((*tcp_header).__annonCompField8.__annonCompField6).th_off()
        as libc::c_int * 4 as libc::c_int) as size_t;
    base = tcp_header as *mut uint8_t;
    last_opt = base.offset(header_size as isize);
    *last_opt.offset(0 as libc::c_int as isize) = 2 as libc::c_int as uint8_t;
    *last_opt.offset(1 as libc::c_int as isize) = 4 as libc::c_int as uint8_t;
    *last_opt.offset(2 as libc::c_int as isize) = 5 as libc::c_int as uint8_t;
    *last_opt.offset(3 as libc::c_int as isize) = 180 as libc::c_int as uint8_t;
    ((*tcp_header).__annonCompField8.__annonCompField6)
        .set_th_off(
            (((*tcp_header).__annonCompField8.__annonCompField6).th_off() as libc::c_int
                + 1 as libc::c_int) as uint8_t,
        );
    return (((*tcp_header).__annonCompField8.__annonCompField6).th_off() as libc::c_int
        * 4 as libc::c_int) as size_t;
}
pub unsafe extern "C" fn make_udp_header(
    mut udp_header: *mut udphdr,
    mut dest_port: port_h_t,
    mut len: uint16_t,
) {
    (*udp_header).__annonCompField5.__annonCompField3.uh_dport = __bswap_16(dest_port);
    (*udp_header).__annonCompField5.__annonCompField3.uh_ulen = __bswap_16(len);
    (*udp_header)
        .__annonCompField5
        .__annonCompField3
        .uh_sum = 0 as libc::c_int as uint16_t;
}
pub unsafe extern "C" fn icmp_helper_validate(
    mut ip_hdr: *const ip,
    mut len: uint32_t,
    mut min_l4_len: size_t,
    mut probe_pkt: *mut *mut ip,
    mut probe_len: *mut size_t,
) -> libc::c_int {
    let mut min_len: uint32_t = 0;
    let mut icmp: *mut icmp = 0 as *mut icmp;
    let mut ip_inner: *mut ip = 0 as *mut ip;
    let mut inner_packet_len: size_t = 0;
    let mut dest: uint32_t = 0;
    let mut tmp___0: libc::c_int = 0;
    if !((*ip_hdr).ip_p as libc::c_int == 1 as libc::c_int) {
        __assert_fail(
            b"ip_hdr->ip_p == IPPROTO_ICMP\0" as *const u8 as *const libc::c_char,
            b"src/probe_modules/packet.c\0" as *const u8 as *const libc::c_char,
            153 as libc::c_uint,
            b"icmp_helper_validate\0" as *const u8 as *const libc::c_char,
        );
    }
    min_len = ((4 as libc::c_uint)
        .wrapping_mul((*ip_hdr).ip_hl())
        .wrapping_add(8 as libc::c_uint) as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ip>() as libc::c_ulong)
        .wrapping_add(min_l4_len) as uint32_t;
    if len < min_len {
        return 0 as libc::c_int;
    }
    icmp = (ip_hdr as *mut libc::c_char)
        .offset((4 as libc::c_uint).wrapping_mul((*ip_hdr).ip_hl()) as isize)
        as *mut icmp;
    if !((*icmp).icmp_type as libc::c_int == 3 as libc::c_int) {
        if !((*icmp).icmp_type as libc::c_int == 4 as libc::c_int) {
            if !((*icmp).icmp_type as libc::c_int == 5 as libc::c_int) {
                if !((*icmp).icmp_type as libc::c_int == 11 as libc::c_int) {
                    return 0 as libc::c_int;
                }
            }
        }
    }
    ip_inner = (icmp as *mut libc::c_char).offset(8 as libc::c_int as isize) as *mut ip;
    inner_packet_len = len
        .wrapping_sub(
            (4 as libc::c_uint)
                .wrapping_mul((*ip_hdr).ip_hl())
                .wrapping_add(8 as libc::c_uint),
        ) as size_t;
    if inner_packet_len
        < ((4 as libc::c_uint).wrapping_mul((*ip_inner).ip_hl()) as size_t)
            .wrapping_add(min_l4_len)
    {
        return 0 as libc::c_int;
    }
    dest = (*ip_inner).ip_dst.s_addr;
    tmp___0 = blocklist_is_allowed(dest);
    if tmp___0 == 0 {
        return 0 as libc::c_int;
    }
    *probe_pkt = ip_inner;
    *probe_len = inner_packet_len;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fs_add_null_icmp(mut fs: *mut fieldset_t) {
    fs_add_null(fs, b"icmp_responder\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"icmp_type\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"icmp_code\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"icmp_unreach_str\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn fs_add_failure_no_port(mut fs: *mut fieldset_t) {
    fs_add_null(fs, b"icmp_responder\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"icmp_type\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"icmp_code\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"icmp_unreach_str\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn fs_populate_icmp_from_iphdr(
    mut ip: *mut ip,
    mut len: size_t,
    mut fs: *mut fieldset_t,
) {
    let mut icmp: *mut icmp = 0 as *mut icmp;
    let mut tmp___1: *mut icmp = 0 as *mut icmp;
    let mut ip_inner: *mut ip = 0 as *mut ip;
    let mut tmp___3: *mut ip = 0 as *mut ip;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    if !ip.is_null() {
        if (b"no ip header provide to fs_populate_icmp_from_iphdr\0" as *const u8
            as *const libc::c_char)
            .is_null()
        {
            __assert_fail(
                b"ip && \"no ip header provide to fs_populate_icmp_from_iphdr\"\0"
                    as *const u8 as *const libc::c_char,
                b"src/probe_modules/packet.c\0" as *const u8 as *const libc::c_char,
                215 as libc::c_uint,
                b"fs_populate_icmp_from_iphdr\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        __assert_fail(
            b"ip && \"no ip header provide to fs_populate_icmp_from_iphdr\"\0"
                as *const u8 as *const libc::c_char,
            b"src/probe_modules/packet.c\0" as *const u8 as *const libc::c_char,
            215 as libc::c_uint,
            b"fs_populate_icmp_from_iphdr\0" as *const u8 as *const libc::c_char,
        );
    }
    if !fs.is_null() {
        if (b"no fieldset provided to fs_populate_icmp_from_iphdr\0" as *const u8
            as *const libc::c_char)
            .is_null()
        {
            __assert_fail(
                b"fs && \"no fieldset provided to fs_populate_icmp_from_iphdr\"\0"
                    as *const u8 as *const libc::c_char,
                b"src/probe_modules/packet.c\0" as *const u8 as *const libc::c_char,
                216 as libc::c_uint,
                b"fs_populate_icmp_from_iphdr\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        __assert_fail(
            b"fs && \"no fieldset provided to fs_populate_icmp_from_iphdr\"\0"
                as *const u8 as *const libc::c_char,
            b"src/probe_modules/packet.c\0" as *const u8 as *const libc::c_char,
            216 as libc::c_uint,
            b"fs_populate_icmp_from_iphdr\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___1 = get_icmp_header(ip as *const ip, len as uint32_t);
    icmp = tmp___1;
    if icmp.is_null() {
        __assert_fail(
            b"icmp\0" as *const u8 as *const libc::c_char,
            b"src/probe_modules/packet.c\0" as *const u8 as *const libc::c_char,
            218 as libc::c_uint,
            b"fs_populate_icmp_from_iphdr\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___3 = get_inner_ip_header(icmp as *const icmp, len as uint32_t);
    ip_inner = tmp___3;
    tmp___4 = make_ip_str((*ip_inner).ip_dst.s_addr);
    fs_modify_string(
        fs,
        b"saddr\0" as *const u8 as *const libc::c_char,
        tmp___4,
        1 as libc::c_int,
    );
    tmp___5 = make_ip_str((*ip).ip_src.s_addr);
    fs_add_string(
        fs,
        b"icmp_responder\0" as *const u8 as *const libc::c_char,
        tmp___5,
        1 as libc::c_int,
    );
    fs_add_uint64(
        fs,
        b"icmp_type\0" as *const u8 as *const libc::c_char,
        (*icmp).icmp_type as uint64_t,
    );
    fs_add_uint64(
        fs,
        b"icmp_code\0" as *const u8 as *const libc::c_char,
        (*icmp).icmp_code as uint64_t,
    );
    if (*icmp).icmp_code as libc::c_int <= 15 as libc::c_int {
        fs_add_constchar(
            fs,
            b"icmp_unreach_str\0" as *const u8 as *const libc::c_char,
            icmp_unreach_strings[(*icmp).icmp_code as usize],
        );
    } else {
        fs_add_constchar(
            fs,
            b"icmp_unreach_str\0" as *const u8 as *const libc::c_char,
            b"unknown\0" as *const u8 as *const libc::c_char,
        );
    };
}
pub unsafe extern "C" fn make_ip_str(mut ip: uint32_t) -> *mut libc::c_char {
    let mut t: in_addr = in_addr { s_addr: 0 };
    let mut temp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut retv: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    t.s_addr = ip;
    tmp = inet_ntoa(t);
    temp = tmp as *const libc::c_char;
    tmp___0 = strlen(temp);
    tmp___1 = xmalloc(tmp___0.wrapping_add(1 as libc::c_ulong));
    retv = tmp___1 as *mut libc::c_char;
    strcpy(retv, temp);
    return retv;
}
pub static mut icmp_unreach_strings: [*const libc::c_char; 16] = [
    b"network unreachable\0" as *const u8 as *const libc::c_char,
    b"host unreachable\0" as *const u8 as *const libc::c_char,
    b"protocol unreachable\0" as *const u8 as *const libc::c_char,
    b"port unreachable\0" as *const u8 as *const libc::c_char,
    b"fragments required\0" as *const u8 as *const libc::c_char,
    b"source route failed\0" as *const u8 as *const libc::c_char,
    b"network unknown\0" as *const u8 as *const libc::c_char,
    b"host unknown\0" as *const u8 as *const libc::c_char,
    b"source host isolated\0" as *const u8 as *const libc::c_char,
    b"network admin. prohibited\0" as *const u8 as *const libc::c_char,
    b"host admin. prohibited\0" as *const u8 as *const libc::c_char,
    b"network unreachable TOS\0" as *const u8 as *const libc::c_char,
    b"host unreachable TOS\0" as *const u8 as *const libc::c_char,
    b"communication admin. prohibited\0" as *const u8 as *const libc::c_char,
    b"host presdence violation\0" as *const u8 as *const libc::c_char,
    b"precedence cutoff\0" as *const u8 as *const libc::c_char,
];
pub static mut probe_modules: [*mut probe_module_t; 9] = unsafe {
    [
        &module_tcp_synscan as *const probe_module_t as *mut probe_module_t,
        &module_tcp_synackscan as *const probe_module_t as *mut probe_module_t,
        &module_icmp_echo as *const probe_module_t as *mut probe_module_t,
        &module_icmp_echo_time as *const probe_module_t as *mut probe_module_t,
        &module_udp as *const probe_module_t as *mut probe_module_t,
        &module_ntp as *const probe_module_t as *mut probe_module_t,
        &module_upnp as *const probe_module_t as *mut probe_module_t,
        &module_dns as *const probe_module_t as *mut probe_module_t,
        &module_bacnet as *const probe_module_t as *mut probe_module_t,
    ]
};
pub unsafe extern "C" fn get_probe_module_by_name(
    mut name: *const libc::c_char,
) -> *mut probe_module_t {
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    len = (::std::mem::size_of::<[*mut probe_module_t; 9]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*mut probe_module_t>() as libc::c_ulong)
        as libc::c_int;
    i = 0 as libc::c_int;
    while i < len {
        tmp = strcmp((*probe_modules[i as usize]).name, name);
        if tmp == 0 {
            return probe_modules[i as usize];
        }
        i += 1;
    }
    return 0 as *mut libc::c_void as *mut probe_module_t;
}
pub unsafe extern "C" fn print_probe_modules() {
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    len = (::std::mem::size_of::<[*mut probe_module_t; 9]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*mut probe_module_t>() as libc::c_ulong)
        as libc::c_int;
    i = 0 as libc::c_int;
    while i < len {
        __printf_chk(
            1 as libc::c_int,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            (*probe_modules[i as usize]).name,
        );
        i += 1;
    }
}
pub unsafe extern "C" fn fs_add_ip_fields(mut fs: *mut fieldset_t, mut ip: *mut ip) {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: __uint16_t = 0;
    tmp = make_ip_str((*ip).ip_src.s_addr);
    fs_add_string(
        fs,
        b"saddr\0" as *const u8 as *const libc::c_char,
        tmp,
        1 as libc::c_int,
    );
    fs_add_uint64(
        fs,
        b"saddr_raw\0" as *const u8 as *const libc::c_char,
        (*ip).ip_src.s_addr as uint64_t,
    );
    tmp___0 = make_ip_str((*ip).ip_dst.s_addr);
    fs_add_string(
        fs,
        b"daddr\0" as *const u8 as *const libc::c_char,
        tmp___0,
        1 as libc::c_int,
    );
    fs_add_uint64(
        fs,
        b"daddr_raw\0" as *const u8 as *const libc::c_char,
        (*ip).ip_dst.s_addr as uint64_t,
    );
    tmp___1 = __bswap_16((*ip).ip_id);
    fs_add_uint64(
        fs,
        b"ipid\0" as *const u8 as *const libc::c_char,
        tmp___1 as uint64_t,
    );
    fs_add_uint64(
        fs,
        b"ttl\0" as *const u8 as *const libc::c_char,
        (*ip).ip_ttl as uint64_t,
    );
}
pub static mut ip_fields_len: libc::c_int = 6 as libc::c_int;
pub static mut ip_fields: [fielddef_t; 6] = [
    {
        let mut init = field_def {
            name: b"saddr\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"source IP address of response\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"saddr_raw\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"network order integer form of source IP address\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"daddr\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"destination IP address of response\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"daddr_raw\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"network order integer form of destination IP address\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"ipid\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"IP identification number of response\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"ttl\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"time-to-live of response packet\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
];
pub static mut sys_fields_len: libc::c_int = 5 as libc::c_int;
pub static mut sys_fields: [fielddef_t; 5] = [
    {
        let mut init = field_def {
            name: b"repeat\0" as *const u8 as *const libc::c_char,
            type_0: b"bool\0" as *const u8 as *const libc::c_char,
            desc: b"Is response a repeat response from host\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"cooldown\0" as *const u8 as *const libc::c_char,
            type_0: b"bool\0" as *const u8 as *const libc::c_char,
            desc: b"Was response received during the cooldown period\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"timestamp_str\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"timestamp of when response arrived in ISO8601 format.\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"timestamp_ts\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"timestamp of when response arrived in seconds since Epoch\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"timestamp_us\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"microsecond part of timestamp (e.g. microseconds since 'timestamp-ts')\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
];
static mut num_ports___2: libc::c_int = 0;
pub unsafe extern "C" fn ntp_global_initialize(
    mut conf: *mut state_conf,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    num_ports___2 = (*conf).source_port_last as libc::c_int
        - (*conf).source_port_first as libc::c_int + 1 as libc::c_int;
    tmp = udp_global_initialize(conf);
    return tmp;
}
pub unsafe extern "C" fn ntp_make_packet(
    mut buf: *mut libc::c_void,
    mut src_ip: ipaddr_n_t,
    mut dst_ip: ipaddr_n_t,
    mut ttl: uint8_t,
    mut validation: *mut uint32_t,
    mut probe_num: libc::c_int,
) -> libc::c_int {
    let mut eth_header: *mut ether_header = 0 as *mut ether_header;
    let mut ip_header: *mut ip = 0 as *mut ip;
    let mut udp_header: *mut udphdr = 0 as *mut udphdr;
    let mut ntp: *mut ntphdr = 0 as *mut ntphdr;
    let mut tmp: uint16_t = 0;
    eth_header = buf as *mut ether_header;
    ip_header = eth_header.offset(1 as libc::c_int as isize) as *mut ip;
    udp_header = ip_header.offset(1 as libc::c_int as isize) as *mut udphdr;
    ntp = udp_header.offset(1 as libc::c_int as isize) as *mut ntphdr;
    (*ip_header).ip_src.s_addr = src_ip;
    (*ip_header).ip_dst.s_addr = dst_ip;
    (*ip_header).ip_ttl = ttl;
    tmp = get_src_port(num_ports___2, probe_num, validation);
    (*udp_header).__annonCompField5.__annonCompField3.uh_sport = __bswap_16(tmp);
    (*ip_header).ip_sum = 0 as libc::c_int as libc::c_ushort;
    (*ip_header).ip_sum = zmap_ip_checksum(ip_header as *mut libc::c_ushort);
    (*ntp).LI_VN_MODE = 227 as libc::c_int as uint8_t;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ntp_validate_packet(
    mut ip_hdr: *const ip,
    mut len: uint32_t,
    mut src_ip: *mut uint32_t,
    mut validation: *mut uint32_t,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = udp_do_validate_packet(
        ip_hdr,
        len,
        src_ip,
        validation,
        num_ports___2,
        zconf.target_port as libc::c_int,
    );
    return tmp;
}
pub unsafe extern "C" fn ntp_process_packet(
    mut packet: *const u_char,
    mut len: uint32_t,
    mut fs: *mut fieldset_t,
    mut validation: *mut uint32_t,
    mut ts: timespec,
) {
    let mut ip_hdr: *mut ip = 0 as *mut ip;
    let mut temp64: uint64_t = 0;
    let mut temp8: uint8_t = 0;
    let mut temp32: uint32_t = 0;
    let mut udp: *mut udphdr = 0 as *mut udphdr;
    let mut ptr: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp: __uint16_t = 0;
    let mut tmp___0: __uint16_t = 0;
    let mut icmp: *mut icmp = 0 as *mut icmp;
    let mut ip_inner: *mut ip = 0 as *mut ip;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    ip_hdr = packet
        .offset(::std::mem::size_of::<ether_header>() as libc::c_ulong as isize)
        as *mut ip;
    if (*ip_hdr).ip_p as libc::c_int == 17 as libc::c_int {
        udp = (ip_hdr as *mut libc::c_char)
            .offset((*ip_hdr).ip_hl().wrapping_mul(4 as libc::c_uint) as isize)
            as *mut udphdr;
        ptr = udp.offset(1 as libc::c_int as isize) as *mut uint8_t;
        fs_add_string(
            fs,
            b"classification\0" as *const u8 as *const libc::c_char,
            b"ntp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int,
        );
        fs_add_bool(
            fs,
            b"success\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
        tmp = __bswap_16((*udp).__annonCompField5.__annonCompField3.uh_sport);
        fs_add_uint64(
            fs,
            b"sport\0" as *const u8 as *const libc::c_char,
            tmp as uint64_t,
        );
        tmp___0 = __bswap_16((*udp).__annonCompField5.__annonCompField3.uh_dport);
        fs_add_uint64(
            fs,
            b"dport\0" as *const u8 as *const libc::c_char,
            tmp___0 as uint64_t,
        );
        fs_add_null(fs, b"icmp_responder\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"icmp_type\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"icmp_code\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"icmp_unreach_str\0" as *const u8 as *const libc::c_char);
        if len > 90 as libc::c_uint {
            temp8 = *ptr;
            fs_add_uint64(
                fs,
                b"LI_VN_MODE\0" as *const u8 as *const libc::c_char,
                temp8 as uint64_t,
            );
            temp8 = *ptr.offset(1 as libc::c_int as isize);
            fs_add_uint64(
                fs,
                b"stratum\0" as *const u8 as *const libc::c_char,
                temp8 as uint64_t,
            );
            temp8 = *ptr.offset(2 as libc::c_int as isize);
            fs_add_uint64(
                fs,
                b"poll\0" as *const u8 as *const libc::c_char,
                temp8 as uint64_t,
            );
            temp8 = *ptr.offset(3 as libc::c_int as isize);
            fs_add_uint64(
                fs,
                b"precision\0" as *const u8 as *const libc::c_char,
                temp8 as uint64_t,
            );
            temp32 = *(ptr as *mut uint32_t).offset(4 as libc::c_int as isize);
            fs_add_uint64(
                fs,
                b"root_delay\0" as *const u8 as *const libc::c_char,
                temp32 as uint64_t,
            );
            temp32 = *(ptr as *mut uint32_t).offset(8 as libc::c_int as isize);
            fs_add_uint64(
                fs,
                b"root_dispersion\0" as *const u8 as *const libc::c_char,
                temp32 as uint64_t,
            );
            temp32 = *(ptr as *mut uint32_t).offset(12 as libc::c_int as isize);
            fs_add_uint64(
                fs,
                b"reference_clock_identifier\0" as *const u8 as *const libc::c_char,
                temp32 as uint64_t,
            );
            temp64 = *(ptr as *mut uint64_t).offset(16 as libc::c_int as isize);
            fs_add_uint64(
                fs,
                b"reference_timestamp\0" as *const u8 as *const libc::c_char,
                temp64,
            );
            temp64 = *(ptr as *mut uint64_t).offset(24 as libc::c_int as isize);
            fs_add_uint64(
                fs,
                b"originate_timestamp\0" as *const u8 as *const libc::c_char,
                temp64,
            );
            temp64 = *(ptr as *mut uint64_t).offset(32 as libc::c_int as isize);
            fs_add_uint64(
                fs,
                b"receive_timestamp\0" as *const u8 as *const libc::c_char,
                temp64,
            );
            temp64 = *(ptr as *mut uint64_t).offset(39 as libc::c_int as isize);
            fs_add_uint64(
                fs,
                b"transmit_timestamp\0" as *const u8 as *const libc::c_char,
                temp64,
            );
        } else {
            fs_add_null(fs, b"LI_VN_MODE\0" as *const u8 as *const libc::c_char);
            fs_add_null(fs, b"stratum\0" as *const u8 as *const libc::c_char);
            fs_add_null(fs, b"poll\0" as *const u8 as *const libc::c_char);
            fs_add_null(fs, b"precision\0" as *const u8 as *const libc::c_char);
            fs_add_null(fs, b"root_delay\0" as *const u8 as *const libc::c_char);
            fs_add_null(fs, b"root_dispersion\0" as *const u8 as *const libc::c_char);
            fs_add_null(
                fs,
                b"reference_clock_identifier\0" as *const u8 as *const libc::c_char,
            );
            fs_add_null(
                fs,
                b"reference_timestamp\0" as *const u8 as *const libc::c_char,
            );
            fs_add_null(
                fs,
                b"originate_timestamp\0" as *const u8 as *const libc::c_char,
            );
            fs_add_null(fs, b"receive_timestamp\0" as *const u8 as *const libc::c_char);
            fs_add_null(fs, b"transmit_timestamp\0" as *const u8 as *const libc::c_char);
        }
    } else if (*ip_hdr).ip_p as libc::c_int == 1 as libc::c_int {
        icmp = (ip_hdr as *mut libc::c_char)
            .offset((*ip_hdr).ip_hl().wrapping_mul(4 as libc::c_uint) as isize)
            as *mut icmp;
        ip_inner = (icmp as *mut libc::c_char).offset(8 as libc::c_int as isize)
            as *mut ip;
        tmp___1 = make_ip_str((*ip_inner).ip_dst.s_addr);
        fs_modify_string(
            fs,
            b"saddr\0" as *const u8 as *const libc::c_char,
            tmp___1,
            1 as libc::c_int,
        );
        fs_add_constchar(
            fs,
            b"classification\0" as *const u8 as *const libc::c_char,
            b"icmp\0" as *const u8 as *const libc::c_char,
        );
        fs_add_bool(
            fs,
            b"success\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        fs_add_null(fs, b"sport\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"dport\0" as *const u8 as *const libc::c_char);
        tmp___2 = make_ip_str((*ip_hdr).ip_src.s_addr);
        fs_add_string(
            fs,
            b"icmp_responder\0" as *const u8 as *const libc::c_char,
            tmp___2,
            1 as libc::c_int,
        );
        fs_add_uint64(
            fs,
            b"icmp_type\0" as *const u8 as *const libc::c_char,
            (*icmp).icmp_type as uint64_t,
        );
        fs_add_uint64(
            fs,
            b"icmp_code\0" as *const u8 as *const libc::c_char,
            (*icmp).icmp_code as uint64_t,
        );
        fs_add_null(fs, b"icmp_unreach_str\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"LI_VN_MODE\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"stratum\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"poll\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"precision\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"root_delay\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"root_dispersion\0" as *const u8 as *const libc::c_char);
        fs_add_null(
            fs,
            b"reference_clock_identifier\0" as *const u8 as *const libc::c_char,
        );
        fs_add_null(fs, b"reference_timestamp\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"originate_timestamp\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"receive_timestamp\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"transmit_timestamp\0" as *const u8 as *const libc::c_char);
    } else {
        fs_add_constchar(
            fs,
            b"classification\0" as *const u8 as *const libc::c_char,
            b"other\0" as *const u8 as *const libc::c_char,
        );
        fs_add_bool(
            fs,
            b"success\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        fs_add_null(fs, b"sport\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"dport\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"icmp_responder\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"icmp_type\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"icmp_code\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"icmp_unreach_str\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"LI_VN_MODE\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"stratum\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"poll\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"precision\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"root_delay\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"root_dispersion\0" as *const u8 as *const libc::c_char);
        fs_add_null(
            fs,
            b"reference_clock_identifier\0" as *const u8 as *const libc::c_char,
        );
        fs_add_null(fs, b"reference_timestamp\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"originate_timestamp\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"receive_timestamp\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"transmit_timestamp\0" as *const u8 as *const libc::c_char);
    };
}
pub unsafe extern "C" fn ntp_init_perthread(
    mut buf: *mut libc::c_void,
    mut src: *mut macaddr_t,
    mut gw: *mut macaddr_t,
    mut dst_port: port_h_t,
    mut arg: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut eth_header: *mut ether_header = 0 as *mut ether_header;
    let mut ip_header: *mut ip = 0 as *mut ip;
    let mut len: uint16_t = 0;
    let mut tmp: __uint16_t = 0;
    let mut udp_header: *mut udphdr = 0 as *mut udphdr;
    let mut ntp_header: *mut ntphdr = 0 as *mut ntphdr;
    let mut header_len: size_t = 0;
    let mut seed: uint32_t = 0;
    let mut tmp___0: uint64_t = 0;
    let mut aes: *mut aesrand_t = 0 as *mut aesrand_t;
    let mut tmp___1: *mut aesrand_t = 0 as *mut aesrand_t;
    memset(buf, 0 as libc::c_int, 4096 as libc::c_int as size_t as _);
    eth_header = buf as *mut ether_header;
    make_eth_header(eth_header, src, gw);
    ip_header = eth_header.offset(1 as libc::c_int as isize) as *mut ip;
    tmp = __bswap_16(
        (::std::mem::size_of::<ip>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<ntphdr>() as libc::c_ulong) as __uint16_t,
    );
    len = tmp;
    make_ip_header(ip_header, 17 as libc::c_int as uint8_t, len);
    udp_header = ip_header.offset(1 as libc::c_int as isize) as *mut udphdr;
    ntp_header = udp_header.offset(1 as libc::c_int as isize) as *mut ntphdr;
    (*ntp_header).LI_VN_MODE = 227 as libc::c_int as uint8_t;
    len = (::std::mem::size_of::<udphdr>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ntphdr>() as libc::c_ulong) as uint16_t;
    make_udp_header(udp_header, zconf.target_port, len);
    header_len = (::std::mem::size_of::<ether_header>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ip>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ntphdr>() as libc::c_ulong);
    module_ntp.max_packet_length = header_len;
    tmp___0 = aesrand_getword(zconf.aes);
    seed = tmp___0 as uint32_t;
    tmp___1 = aesrand_init_from_seed(seed as uint64_t);
    aes = tmp___1;
    *arg = aes as *mut libc::c_void;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ntp_print_packet(
    mut fp: *mut FILE,
    mut packet: *mut libc::c_void,
) {
    let mut ethh: *mut ether_header = 0 as *mut ether_header;
    let mut iph: *mut ip = 0 as *mut ip;
    let mut udph: *mut udphdr = 0 as *mut udphdr;
    let mut ntph: *mut ntphdr = 0 as *mut ntphdr;
    let mut tmp: __uint16_t = 0;
    let mut tmp___0: __uint16_t = 0;
    let mut tmp___1: __uint16_t = 0;
    ethh = packet as *mut ether_header;
    iph = ethh.offset(1 as libc::c_int as isize) as *mut ip;
    udph = iph.offset((4 as libc::c_uint).wrapping_mul((*iph).ip_hl()) as isize)
        as *mut udphdr;
    ntph = udph.offset(1 as libc::c_int as isize) as *mut ntphdr;
    __fprintf_chk(
        fp,
        1 as libc::c_int,
        b"ntp { LI_VN_MODE: %u | stratum: %u | poll: %u }\n\0" as *const u8
            as *const libc::c_char,
        (*ntph).LI_VN_MODE as libc::c_int,
        (*ntph).stratum as libc::c_int,
        (*ntph).poll as libc::c_int,
    );
    tmp = __bswap_16((*udph).__annonCompField5.__annonCompField3.uh_sum);
    tmp___0 = __bswap_16((*udph).__annonCompField5.__annonCompField3.uh_dport);
    tmp___1 = __bswap_16((*udph).__annonCompField5.__annonCompField3.uh_sport);
    __fprintf_chk(
        fp,
        1 as libc::c_int,
        b"udp { source: %u | dest: %u | checksum: %#04X }\n\0" as *const u8
            as *const libc::c_char,
        tmp___1 as libc::c_int,
        tmp___0 as libc::c_int,
        tmp as libc::c_int,
    );
    fprintf_ip_header(fp, iph);
    fprintf_eth_header(fp, ethh);
    __fprintf_chk(
        fp,
        1 as libc::c_int,
        b"------------------------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
}
static mut fields___4: [fielddef_t; 19] = [
    {
        let mut init = field_def {
            name: b"classification\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"packet classification\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"success\0" as *const u8 as *const libc::c_char,
            type_0: b"bool\0" as *const u8 as *const libc::c_char,
            desc: b"is  response considered success\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"sport\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"UDP source port\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dport\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"UDP destination port\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_responder\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"Source IP of ICMP_UNREACH messages\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_type\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"icmp message type\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_code\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"icmp message sub type code\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_unreach_str\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"for icmp_unreach responses, the string version of icmp_code (e.g. network-unreach)\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"LI_VN_MODE\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"leap indication, version number, mode\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"stratum\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"stratum\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"poll\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"poll\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"precision\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"precision\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"root_delay\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"root delay\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"root_dispersion\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"root dispersion\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"reference_clock_identifier\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"code identifying clock reference\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"reference_timestamp\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"local time at which local clock was last set or corrected\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"originate_timestamp\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"local time at which request deparated client for service\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"receive_timestamp\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"local time at which request arrvied at service host\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"transmit_timestamp\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"local time which reply departed service host for client\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
];
pub static mut module_ntp: probe_module_t = probe_module_t {
    name: 0 as *const libc::c_char,
    max_packet_length: 0,
    pcap_filter: 0 as *const libc::c_char,
    pcap_snaplen: 0,
    port_args: 0,
    global_initialize: None,
    thread_initialize: None,
    make_packet: None,
    print_packet: None,
    validate_packet: None,
    process_packet: None,
    close: None,
    output_type: 0,
    fields: 0 as *const fielddef_t as *mut fielddef_t,
    numfields: 0,
    helptext: 0 as *const libc::c_char,
};
static mut upnp_query: *const libc::c_char = b"M-SEARCH * HTTP/1.1\r\nHost:239.255.255.250:1900\r\nST:upnp:rootdevice\r\nMan:\"ssdp:discover\"\r\nMX:3\r\n\r\n\0"
    as *const u8 as *const libc::c_char;
static mut num_ports___3: libc::c_int = 0;
pub unsafe extern "C" fn upnp_global_initialize(
    mut state: *mut state_conf,
) -> libc::c_int {
    num_ports___3 = (*state).source_port_last as libc::c_int
        - (*state).source_port_first as libc::c_int + 1 as libc::c_int;
    udp_set_num_ports(num_ports___3);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn upnp_init_perthread(
    mut buf: *mut libc::c_void,
    mut src: *mut macaddr_t,
    mut gw: *mut macaddr_t,
    mut dst_port: port_h_t,
    mut arg_ptr: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut eth_header: *mut ether_header = 0 as *mut ether_header;
    let mut ip_header: *mut ip = 0 as *mut ip;
    let mut len: uint16_t = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: __uint16_t = 0;
    let mut udp_header: *mut udphdr = 0 as *mut udphdr;
    let mut tmp___1: size_t = 0;
    let mut payload: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: size_t = 0;
    let mut tmp___9: size_t = 0;
    memset(buf, 0 as libc::c_int, 4096 as libc::c_int as size_t as _);
    eth_header = buf as *mut ether_header;
    make_eth_header(eth_header, src, gw);
    ip_header = eth_header.offset(1 as libc::c_int as isize) as *mut ip;
    tmp = strlen(upnp_query);
    tmp___0 = __bswap_16(
        (::std::mem::size_of::<ip>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong)
            .wrapping_add(tmp) as __uint16_t,
    );
    len = tmp___0;
    make_ip_header(ip_header, 17 as libc::c_int as uint8_t, len);
    udp_header = ip_header.offset(1 as libc::c_int as isize) as *mut udphdr;
    tmp___1 = strlen(upnp_query);
    len = (::std::mem::size_of::<udphdr>() as libc::c_ulong).wrapping_add(tmp___1)
        as uint16_t;
    make_udp_header(udp_header, dst_port, len);
    payload = udp_header.offset(1 as libc::c_int as isize) as *mut libc::c_char;
    tmp___5 = strlen(upnp_query);
    if !((::std::mem::size_of::<ether_header>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ip>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong)
        .wrapping_add(tmp___5) <= 4096 as libc::c_ulong)
    {
        __assert_fail(
            b"sizeof(struct ether_header) + sizeof(struct ip) + sizeof(struct udphdr) + strlen(upnp_query) <= MAX_PACKET_SIZE\0"
                as *const u8 as *const libc::c_char,
            b"src/probe_modules/module_upnp.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_uint,
            b"upnp_init_perthread\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___9 = strlen(upnp_query);
    if !(4096 as libc::c_long
        - payload.offset_from(buf as *mut libc::c_char) as libc::c_long
        > tmp___9 as libc::c_int as libc::c_long)
    {
        __assert_fail(
            b"MAX_PACKET_SIZE - ((char *)payload - (char *)buf) > (int)strlen(upnp_query)\0"
                as *const u8 as *const libc::c_char,
            b"src/probe_modules/module_upnp.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_uint,
            b"upnp_init_perthread\0" as *const u8 as *const libc::c_char,
        );
    }
    strcpy(payload, upnp_query);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn upnp_validate_packet(
    mut ip_hdr: *const ip,
    mut len: uint32_t,
    mut src_ip: *mut uint32_t,
    mut validation: *mut uint32_t,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = udp_do_validate_packet(
        ip_hdr,
        len,
        src_ip,
        validation,
        num_ports___3,
        zconf.target_port as libc::c_int,
    );
    return tmp;
}
pub unsafe extern "C" fn upnp_process_packet(
    mut packet: *const u_char,
    mut len: uint32_t,
    mut fs: *mut fieldset_t,
    mut validation: *mut uint32_t,
    mut ts: timespec,
) {
    let mut ip_hdr: *mut ip = 0 as *mut ip;
    let mut udp: *mut udphdr = 0 as *mut udphdr;
    let mut payload: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut plen: uint16_t = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut is_first: libc::c_int = 0;
    let mut classification: *const libc::c_char = 0 as *const libc::c_char;
    let mut is_success: uint64_t = 0;
    let mut server: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut location: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut usn: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut st: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cachecontrol: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ext: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut xusragent: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut date: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut agent: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pch: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: size_t = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: libc::c_int = 0;
    let mut tmp___13: libc::c_int = 0;
    let mut tmp___14: libc::c_int = 0;
    let mut tmp___15: __uint16_t = 0;
    let mut tmp___16: __uint16_t = 0;
    let mut tmp___17: __uint16_t = 0;
    ip_hdr = packet
        .offset(::std::mem::size_of::<ether_header>() as libc::c_ulong as isize)
        as *mut ip;
    if (*ip_hdr).ip_p as libc::c_int == 17 as libc::c_int {
        udp = (ip_hdr as *mut libc::c_char)
            .offset((*ip_hdr).ip_hl().wrapping_mul(4 as libc::c_uint) as isize)
            as *mut udphdr;
        payload = udp.offset(1 as libc::c_int as isize) as *mut libc::c_char;
        plen = ((*udp).__annonCompField5.__annonCompField3.uh_ulen as libc::c_int
            - 8 as libc::c_int) as uint16_t;
        tmp = xmalloc((plen as libc::c_int + 1 as libc::c_int) as size_t);
        s = tmp as *mut libc::c_char;
        strncpy(s, payload as *const libc::c_char, plen as size_t as _);
        *s.offset(plen as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
        is_first = 1 as libc::c_int;
        classification = b"none\0" as *const u8 as *const libc::c_char;
        is_success = 0 as libc::c_int as uint64_t;
        server = 0 as *mut libc::c_void as *mut libc::c_char;
        location = 0 as *mut libc::c_void as *mut libc::c_char;
        usn = 0 as *mut libc::c_void as *mut libc::c_char;
        st = 0 as *mut libc::c_void as *mut libc::c_char;
        cachecontrol = 0 as *mut libc::c_void as *mut libc::c_char;
        ext = 0 as *mut libc::c_void as *mut libc::c_char;
        xusragent = 0 as *mut libc::c_void as *mut libc::c_char;
        date = 0 as *mut libc::c_void as *mut libc::c_char;
        agent = 0 as *mut libc::c_void as *mut libc::c_char;
        tmp___0 = strtok(s, b"\n\0" as *const u8 as *const libc::c_char);
        pch = tmp___0;
        while pch as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            tmp___2 = strlen(pch as *const libc::c_char);
            if *pch.offset(tmp___2.wrapping_sub(1 as libc::c_ulong) as isize)
                as libc::c_int == 13 as libc::c_int
            {
                tmp___1 = strlen(pch as *const libc::c_char);
                *pch
                    .offset(
                        tmp___1.wrapping_sub(1 as libc::c_ulong) as isize,
                    ) = '\u{0}' as i32 as libc::c_char;
            }
            tmp___3 = strlen(pch as *const libc::c_char);
            if tmp___3 == 0 as libc::c_ulong {
                pch = strtok(
                    0 as *mut libc::c_void as *mut libc::c_char,
                    b"\n\0" as *const u8 as *const libc::c_char,
                );
            } else if is_first != 0 {
                tmp___4 = strcmp(
                    pch as *const libc::c_char,
                    b"HTTP/1.1 200 OK\0" as *const u8 as *const libc::c_char,
                );
                if tmp___4 != 0 {
                    classification = b"no-http-header\0" as *const u8
                        as *const libc::c_char;
                    is_success = 0 as libc::c_int as uint64_t;
                    break;
                } else {
                    is_first = 0 as libc::c_int;
                    is_success = 1 as libc::c_int as uint64_t;
                    classification = b"upnp\0" as *const u8 as *const libc::c_char;
                    pch = strtok(
                        0 as *mut libc::c_void as *mut libc::c_char,
                        b"\n\0" as *const u8 as *const libc::c_char,
                    );
                }
            } else {
                value = pch;
                tmp___5 = strsep(
                    &mut value as *mut *mut libc::c_char,
                    b":\0" as *const u8 as *const libc::c_char,
                );
                key = tmp___5;
                if key.is_null() {
                    pch = strtok(
                        0 as *mut libc::c_void as *mut libc::c_char,
                        b"\n\0" as *const u8 as *const libc::c_char,
                    );
                } else if value.is_null() {
                    pch = strtok(
                        0 as *mut libc::c_void as *mut libc::c_char,
                        b"\n\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    if *value.offset(0 as libc::c_int as isize) as libc::c_int
                        == 32 as libc::c_int
                    {
                        value = value.offset(1);
                    }
                    tmp___14 = strcasecmp(
                        key as *const libc::c_char,
                        b"server\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___14 != 0 {
                        tmp___13 = strcasecmp(
                            key as *const libc::c_char,
                            b"location\0" as *const u8 as *const libc::c_char,
                        );
                        if tmp___13 != 0 {
                            tmp___12 = strcasecmp(
                                key as *const libc::c_char,
                                b"USN\0" as *const u8 as *const libc::c_char,
                            );
                            if tmp___12 != 0 {
                                tmp___11 = strcasecmp(
                                    key as *const libc::c_char,
                                    b"EXT\0" as *const u8 as *const libc::c_char,
                                );
                                if tmp___11 != 0 {
                                    tmp___10 = strcasecmp(
                                        key as *const libc::c_char,
                                        b"ST\0" as *const u8 as *const libc::c_char,
                                    );
                                    if tmp___10 != 0 {
                                        tmp___9 = strcasecmp(
                                            key as *const libc::c_char,
                                            b"Agent\0" as *const u8 as *const libc::c_char,
                                        );
                                        if tmp___9 != 0 {
                                            tmp___8 = strcasecmp(
                                                key as *const libc::c_char,
                                                b"X-User-Agent\0" as *const u8 as *const libc::c_char,
                                            );
                                            if tmp___8 != 0 {
                                                tmp___7 = strcasecmp(
                                                    key as *const libc::c_char,
                                                    b"date\0" as *const u8 as *const libc::c_char,
                                                );
                                                if tmp___7 != 0 {
                                                    tmp___6 = strcasecmp(
                                                        key as *const libc::c_char,
                                                        b"Cache-Control\0" as *const u8 as *const libc::c_char,
                                                    );
                                                    if tmp___6 == 0 {
                                                        cachecontrol = strdup(value as *const libc::c_char);
                                                    }
                                                } else {
                                                    date = strdup(value as *const libc::c_char);
                                                }
                                            } else {
                                                xusragent = strdup(value as *const libc::c_char);
                                            }
                                        } else {
                                            agent = strdup(value as *const libc::c_char);
                                        }
                                    } else {
                                        st = strdup(value as *const libc::c_char);
                                    }
                                } else {
                                    ext = strdup(value as *const libc::c_char);
                                }
                            } else {
                                usn = strdup(value as *const libc::c_char);
                            }
                        } else {
                            location = strdup(value as *const libc::c_char);
                        }
                    } else {
                        server = strdup(value as *const libc::c_char);
                    }
                    pch = strtok(
                        0 as *mut libc::c_void as *mut libc::c_char,
                        b"\n\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
        fs_add_string(
            fs,
            b"classification\0" as *const u8 as *const libc::c_char,
            classification as *mut libc::c_char,
            0 as libc::c_int,
        );
        fs_add_bool(
            fs,
            b"success\0" as *const u8 as *const libc::c_char,
            is_success as libc::c_int,
        );
        fs_chkadd_unsafe_string(
            fs,
            b"server\0" as *const u8 as *const libc::c_char,
            server,
            1 as libc::c_int,
        );
        fs_chkadd_unsafe_string(
            fs,
            b"location\0" as *const u8 as *const libc::c_char,
            location,
            1 as libc::c_int,
        );
        fs_chkadd_unsafe_string(
            fs,
            b"usn\0" as *const u8 as *const libc::c_char,
            usn,
            1 as libc::c_int,
        );
        fs_chkadd_unsafe_string(
            fs,
            b"st\0" as *const u8 as *const libc::c_char,
            st,
            1 as libc::c_int,
        );
        fs_chkadd_unsafe_string(
            fs,
            b"ext\0" as *const u8 as *const libc::c_char,
            ext,
            1 as libc::c_int,
        );
        fs_chkadd_unsafe_string(
            fs,
            b"cache_control\0" as *const u8 as *const libc::c_char,
            cachecontrol,
            1 as libc::c_int,
        );
        fs_chkadd_unsafe_string(
            fs,
            b"x_user_agent\0" as *const u8 as *const libc::c_char,
            xusragent,
            1 as libc::c_int,
        );
        fs_chkadd_unsafe_string(
            fs,
            b"agent\0" as *const u8 as *const libc::c_char,
            agent,
            1 as libc::c_int,
        );
        fs_chkadd_unsafe_string(
            fs,
            b"date\0" as *const u8 as *const libc::c_char,
            date,
            1 as libc::c_int,
        );
        tmp___15 = __bswap_16((*udp).__annonCompField5.__annonCompField3.uh_sport);
        fs_add_uint64(
            fs,
            b"sport\0" as *const u8 as *const libc::c_char,
            tmp___15 as uint64_t,
        );
        tmp___16 = __bswap_16((*udp).__annonCompField5.__annonCompField3.uh_dport);
        fs_add_uint64(
            fs,
            b"dport\0" as *const u8 as *const libc::c_char,
            tmp___16 as uint64_t,
        );
        fs_add_null(fs, b"icmp_responder\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"icmp_type\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"icmp_code\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"icmp_unreach_str\0" as *const u8 as *const libc::c_char);
        tmp___17 = __bswap_16((*udp).__annonCompField5.__annonCompField3.uh_ulen);
        fs_add_binary(
            fs,
            b"data\0" as *const u8 as *const libc::c_char,
            (tmp___17 as libc::c_ulong)
                .wrapping_sub(::std::mem::size_of::<udphdr>() as libc::c_ulong),
            udp.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            0 as libc::c_int,
        );
        free(s as *mut libc::c_void);
    } else if (*ip_hdr).ip_p as libc::c_int == 1 as libc::c_int {
        fs_add_constchar(
            fs,
            b"classification\0" as *const u8 as *const libc::c_char,
            b"icmp\0" as *const u8 as *const libc::c_char,
        );
        fs_add_uint64(
            fs,
            b"success\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as uint64_t,
        );
        fs_add_null(fs, b"server\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"location\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"usn\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"st\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"ext\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"cache_control\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"x_user_agent\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"agent\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"date\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"sport\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"dport\0" as *const u8 as *const libc::c_char);
        fs_populate_icmp_from_iphdr(ip_hdr, len as size_t, fs);
        fs_add_null(fs, b"data\0" as *const u8 as *const libc::c_char);
    } else {
        fs_add_constchar(
            fs,
            b"classification\0" as *const u8 as *const libc::c_char,
            b"other\0" as *const u8 as *const libc::c_char,
        );
        fs_add_bool(
            fs,
            b"success\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        fs_add_null(fs, b"server\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"location\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"usn\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"st\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"ext\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"cache_control\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"x_user_agent\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"agent\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"date\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"sport\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"dport\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"icmp_responder\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"icmp_type\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"icmp_code\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"icmp_unreach_str\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"data\0" as *const u8 as *const libc::c_char);
    };
}
static mut fields___5: [fielddef_t; 18] = [
    {
        let mut init = field_def {
            name: b"classification\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"packet classification\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"success\0" as *const u8 as *const libc::c_char,
            type_0: b"bool\0" as *const u8 as *const libc::c_char,
            desc: b"is response considered success\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"server\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"UPnP server\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"location\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"UPnP location\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"usn\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"UPnP usn\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"st\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"UPnP st\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"ext\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"UPnP ext\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"cache_control\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"UPnP cache-control\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"x_user_agent\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"UPnP x-user-agent\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"agent\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"UPnP agent\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"date\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"UPnP date\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"sport\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"UDP source port\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dport\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"UDP destination port\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_responder\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"Source IP of ICMP_UNREACH messages\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_type\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"icmp message type\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_code\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"icmp message sub type code\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_unreach_str\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"for icmp_unreach responses, the string version of icmp_code (e.g. network-unreach)\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"data\0" as *const u8 as *const libc::c_char,
            type_0: b"binary\0" as *const u8 as *const libc::c_char,
            desc: b"UDP payload\0" as *const u8 as *const libc::c_char,
        };
        init
    },
];
pub static mut module_upnp: probe_module_t = unsafe {
    {
        let mut init = probe_module {
            name: b"upnp\0" as *const u8 as *const libc::c_char,
            max_packet_length: 139 as libc::c_int as size_t,
            pcap_filter: b"udp || icmp\0" as *const u8 as *const libc::c_char,
            pcap_snaplen: 2048 as libc::c_int as size_t,
            port_args: 1 as libc::c_int as uint8_t,
            global_initialize: Some(
                upnp_global_initialize
                    as unsafe extern "C" fn(*mut state_conf) -> libc::c_int,
            ),
            thread_initialize: Some(
                upnp_init_perthread
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut macaddr_t,
                        *mut macaddr_t,
                        port_h_t,
                        *mut *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            make_packet: Some(
                udp_make_packet
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut size_t,
                        ipaddr_n_t,
                        ipaddr_n_t,
                        uint8_t,
                        *mut uint32_t,
                        libc::c_int,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            print_packet: Some(
                udp_print_packet
                    as unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> (),
            ),
            validate_packet: Some(
                upnp_validate_packet
                    as unsafe extern "C" fn(
                        *const ip,
                        uint32_t,
                        *mut uint32_t,
                        *mut uint32_t,
                    ) -> libc::c_int,
            ),
            process_packet: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const u_char,
                        uint32_t,
                        *mut fieldset_t,
                        *mut uint32_t,
                        timespec,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *const u_char,
                        uint32_t,
                        *mut fieldset_t,
                        *mut uint32_t,
                        timespec,
                    ) -> (),
                >,
            >(
                Some(
                    upnp_process_packet
                        as unsafe extern "C" fn(
                            *const u_char,
                            uint32_t,
                            *mut fieldset_t,
                            *mut uint32_t,
                            timespec,
                        ) -> (),
                ),
            ),
            close: ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<
                    unsafe extern "C" fn(
                        *mut state_conf,
                        *mut state_send,
                        *mut state_recv,
                    ) -> libc::c_int,
                >,
            >(0 as *const libc::c_void as *mut libc::c_void),
            output_type: 2 as libc::c_int,
            fields: fields___5.as_ptr() as *mut _,
            numfields: 18 as libc::c_int,
            helptext: b"Probe module that sends a TCP SYN packet to a specific port. Possible classifications are: synack and rst. A SYN-ACK packet is considered a success and a reset packet is considered a failed response.\0"
                as *const u8 as *const libc::c_char,
        };
        init
    }
};
static mut num_ports___4: libc::c_int = 0;
pub static mut default_domain: [libc::c_char; 15] = [
    'w' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
];
pub static mut default_qtype: libc::c_ushort = 1 as libc::c_int as uint16_t;
static mut dns_packets: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut dns_packet_lens: *mut uint16_t = 0 as *const uint16_t as *mut uint16_t;
static mut qname_lens: *mut uint16_t = 0 as *const uint16_t as *mut uint16_t;
static mut qnames: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut qtypes: *mut uint16_t = 0 as *const uint16_t as *mut uint16_t;
static mut num_questions: libc::c_int = 0 as libc::c_int;
pub static mut qtype_strs: [*const libc::c_char; 10] = [
    b"A\0" as *const u8 as *const libc::c_char,
    b"NS\0" as *const u8 as *const libc::c_char,
    b"CNAME\0" as *const u8 as *const libc::c_char,
    b"SOA\0" as *const u8 as *const libc::c_char,
    b"PTR\0" as *const u8 as *const libc::c_char,
    b"MX\0" as *const u8 as *const libc::c_char,
    b"TXT\0" as *const u8 as *const libc::c_char,
    b"AAAA\0" as *const u8 as *const libc::c_char,
    b"RRSIG\0" as *const u8 as *const libc::c_char,
    b"ALL\0" as *const u8 as *const libc::c_char,
];
pub static mut qtype_strs_len: libc::c_int = 10 as libc::c_int;
pub static mut qtype_strid_to_qtype: [dns_qtype; 10] = [
    DNS_QTYPE_A,
    DNS_QTYPE_NS,
    DNS_QTYPE_CNAME,
    DNS_QTYPE_SOA,
    DNS_QTYPE_PTR,
    DNS_QTYPE_MX,
    DNS_QTYPE_TXT,
    DNS_QTYPE_AAAA,
    DNS_QTYPE_RRSIG,
    DNS_QTYPE_ALL,
];
pub static mut qtype_qtype_to_strid: [int8_t; 256] = [
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
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
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
pub unsafe extern "C" fn setup_qtype_str_map() {
    qtype_qtype_to_strid[1 as libc::c_int as usize] = 0 as libc::c_int as int8_t;
    qtype_qtype_to_strid[2 as libc::c_int as usize] = 1 as libc::c_int as int8_t;
    qtype_qtype_to_strid[5 as libc::c_int as usize] = 2 as libc::c_int as int8_t;
    qtype_qtype_to_strid[6 as libc::c_int as usize] = 3 as libc::c_int as int8_t;
    qtype_qtype_to_strid[12 as libc::c_int as usize] = 4 as libc::c_int as int8_t;
    qtype_qtype_to_strid[15 as libc::c_int as usize] = 5 as libc::c_int as int8_t;
    qtype_qtype_to_strid[16 as libc::c_int as usize] = 6 as libc::c_int as int8_t;
    qtype_qtype_to_strid[28 as libc::c_int as usize] = 7 as libc::c_int as int8_t;
    qtype_qtype_to_strid[46 as libc::c_int as usize] = 8 as libc::c_int as int8_t;
    qtype_qtype_to_strid[255 as libc::c_int as usize] = 9 as libc::c_int as int8_t;
}
unsafe extern "C" fn qtype_str_to_code(mut str: *const libc::c_char) -> uint16_t {
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < qtype_strs_len {
        tmp = strcmp(qtype_strs[i as usize], str);
        if tmp == 0 as libc::c_int {
            return qtype_strid_to_qtype[i as usize] as uint16_t;
        }
        i += 1;
    }
    return 0 as libc::c_int as uint16_t;
}
unsafe extern "C" fn domain_to_qname(
    mut qname_handle: *mut *mut libc::c_char,
    mut domain: *const libc::c_char,
) -> uint16_t {
    let mut len: uint16_t = 0;
    let mut tmp: size_t = 0;
    let mut qname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    tmp = strlen(domain);
    len = tmp.wrapping_add(1 as libc::c_ulong).wrapping_add(1 as libc::c_ulong)
        as uint16_t;
    tmp___0 = xmalloc(len as size_t);
    qname = tmp___0 as *mut libc::c_char;
    *qname.offset(0 as libc::c_int as isize) = '.' as i32 as libc::c_char;
    strcpy(qname.offset(1 as libc::c_int as isize), domain);
    i = 0 as libc::c_int;
    while i < len as libc::c_int {
        if *qname.offset(i as isize) as libc::c_int == 46 as libc::c_int {
            j = i + 1 as libc::c_int;
            while j < len as libc::c_int - 1 as libc::c_int {
                if *qname.offset(j as isize) as libc::c_int == 46 as libc::c_int {
                    break;
                }
                j += 1;
            }
            *qname.offset(i as isize) = (j - i - 1 as libc::c_int) as libc::c_char;
        }
        i += 1;
    }
    *qname_handle = qname;
    if !(*(*qname_handle).offset((len as libc::c_int - 1 as libc::c_int) as isize)
        as libc::c_int == 0 as libc::c_int)
    {
        __assert_fail(
            b"(*qname_handle)[len - 1] == '\\0'\0" as *const u8 as *const libc::c_char,
            b"src/probe_modules/module_dns.c\0" as *const u8 as *const libc::c_char,
            152 as libc::c_uint,
            b"domain_to_qname\0" as *const u8 as *const libc::c_char,
        );
    }
    return len;
}
unsafe extern "C" fn build_global_dns_packets(
    mut domains: *mut *mut libc::c_char,
    mut num_domains: libc::c_int,
    mut max_len: *mut size_t,
) -> libc::c_int {
    let mut _max_len: size_t = 0;
    let mut i: libc::c_int = 0;
    let mut len: uint16_t = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut dns_header_p: *mut dns_header = 0 as *mut dns_header;
    let mut qname_p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tail_p: *mut dns_question_tail = 0 as *mut dns_question_tail;
    _max_len = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int;
    while i < num_domains {
        *qname_lens
            .offset(
                i as isize,
            ) = domain_to_qname(
            qnames.offset(i as isize),
            *domains.offset(i as isize) as *const libc::c_char,
        );
        if *domains.offset(i as isize) as libc::c_ulong
            != default_domain.as_ptr() as *mut libc::c_char as libc::c_ulong
        {
            free(*domains.offset(i as isize) as *mut libc::c_void);
        }
        len = (::std::mem::size_of::<dns_header>() as libc::c_ulong)
            .wrapping_add(*qname_lens.offset(i as isize) as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<dns_question_tail>() as libc::c_ulong)
            as uint16_t;
        *dns_packet_lens.offset(i as isize) = len;
        if len as size_t > _max_len {
            _max_len = len as size_t;
        }
        if *dns_packet_lens.offset(i as isize) as libc::c_int > 512 as libc::c_int {
            log_fatal(
                b"dns\0" as *const u8 as *const libc::c_char,
                b"DNS packet bigger (%d) than our limit (%d)\0" as *const u8
                    as *const libc::c_char,
                *dns_packet_lens.offset(i as isize) as libc::c_int,
                512 as libc::c_int,
            );
        }
        tmp = xmalloc(*dns_packet_lens.offset(i as isize) as size_t);
        let ref mut fresh17 = *dns_packets.offset(i as isize);
        *fresh17 = tmp as *mut libc::c_char;
        dns_header_p = *dns_packets.offset(i as isize) as *mut dns_header;
        qname_p = (*dns_packets.offset(i as isize))
            .offset(::std::mem::size_of::<dns_header>() as libc::c_ulong as isize);
        tail_p = (*dns_packets.offset(i as isize))
            .offset(::std::mem::size_of::<dns_header>() as libc::c_ulong as isize)
            .offset(*qname_lens.offset(i as isize) as libc::c_int as isize)
            as *mut dns_question_tail;
        (*dns_header_p).set_rd(1 as libc::c_uint);
        (*dns_header_p).qdcount = __bswap_16(1 as libc::c_int as __uint16_t);
        memcpy(
            qname_p as *mut libc::c_void,
            *qnames.offset(i as isize) as *const libc::c_void,
            *qname_lens.offset(i as isize) as size_t as _,
        );
        (*tail_p).qtype = __bswap_16(*qtypes.offset(i as isize));
        (*tail_p).qclass = __bswap_16(1 as libc::c_int as __uint16_t);
        i += 1;
    }
    *max_len = _max_len;
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_name_helper(
    mut data: *const libc::c_char,
    mut data_len: uint16_t,
    mut payload: *const libc::c_char,
    mut payload_len: uint16_t,
    mut name: *mut libc::c_char,
    mut name_len: uint16_t,
    mut recursion_level: uint16_t,
) -> uint16_t {
    let mut bytes_consumed: uint16_t = 0;
    let mut byte: uint8_t = 0;
    let mut offset: uint16_t = 0;
    let mut rec_bytes_consumed: uint16_t = 0;
    let mut tmp: uint16_t = 0;
    if data_len as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int as uint16_t
    } else {
        if name_len as libc::c_int == 0 as libc::c_int {
            return 0 as libc::c_int as uint16_t
        } else {
            if payload_len as libc::c_int == 0 as libc::c_int {
                return 0 as libc::c_int as uint16_t;
            }
        }
    }
    if recursion_level as libc::c_int > 10 as libc::c_int {
        return 0 as libc::c_int as uint16_t;
    }
    bytes_consumed = 0 as libc::c_int as uint16_t;
    while data_len as libc::c_int > 0 as libc::c_int {
        byte = *data.offset(0 as libc::c_int as isize) as uint8_t;
        if byte as libc::c_int >= 192 as libc::c_int {
            if (data_len as libc::c_int) < 2 as libc::c_int {
                return 0 as libc::c_int as uint16_t;
            }
            offset = ((byte as libc::c_int & 3 as libc::c_int) << 8 as libc::c_int
                | *data.offset(1 as libc::c_int as isize) as uint8_t as libc::c_int)
                as uint16_t;
            if offset as libc::c_int >= payload_len as libc::c_int {
                return 0 as libc::c_int as uint16_t;
            }
            let mut current_block_27: u64;
            if recursion_level as libc::c_int > 0 as libc::c_int {
                current_block_27 = 13513605098558570425;
            } else if bytes_consumed as libc::c_int > 0 as libc::c_int {
                current_block_27 = 13513605098558570425;
            } else {
                current_block_27 = 16203760046146113240;
            }
            match current_block_27 {
                13513605098558570425 => {
                    if (name_len as libc::c_int) < 1 as libc::c_int {
                        log_warn(
                            b"dns\0" as *const u8 as *const libc::c_char,
                            b"Exceeded static name field allocation.\0" as *const u8
                                as *const libc::c_char,
                        );
                        return 0 as libc::c_int as uint16_t;
                    }
                    *name.offset(0 as libc::c_int as isize) = '.' as i32 as libc::c_char;
                    name = name.offset(1);
                    name_len = (name_len as libc::c_int - 1 as libc::c_int) as uint16_t;
                }
                _ => {}
            }
            tmp = get_name_helper(
                payload.offset(offset as libc::c_int as isize),
                (payload_len as libc::c_int - offset as libc::c_int) as uint16_t,
                payload,
                payload_len,
                name,
                name_len,
                (recursion_level as libc::c_int + 1 as libc::c_int) as uint16_t,
            );
            rec_bytes_consumed = tmp;
            if rec_bytes_consumed as libc::c_int == 0 as libc::c_int {
                return 0 as libc::c_int as uint16_t
            } else {
                bytes_consumed = (bytes_consumed as libc::c_int + 2 as libc::c_int)
                    as uint16_t;
                return bytes_consumed;
            }
        } else {
            if byte as libc::c_int == 0 as libc::c_int {
                bytes_consumed = (bytes_consumed as libc::c_int + 1 as libc::c_int)
                    as uint16_t;
                return bytes_consumed;
            } else {
                data = data.offset(1);
                data_len = (data_len as libc::c_int - 1 as libc::c_int) as uint16_t;
                if byte as libc::c_int + 1 as libc::c_int > data_len as libc::c_int {
                    return 0 as libc::c_int as uint16_t;
                }
                if bytes_consumed as libc::c_int > 0 as libc::c_int {
                    if (name_len as libc::c_int) < 1 as libc::c_int {
                        log_warn(
                            b"dns\0" as *const u8 as *const libc::c_char,
                            b"Exceeded static name field allocation.\0" as *const u8
                                as *const libc::c_char,
                        );
                        return 0 as libc::c_int as uint16_t;
                    }
                    *name.offset(0 as libc::c_int as isize) = '.' as i32 as libc::c_char;
                    name = name.offset(1);
                    name_len = (name_len as libc::c_int - 1 as libc::c_int) as uint16_t;
                }
                bytes_consumed = (bytes_consumed as libc::c_int + 1 as libc::c_int)
                    as uint16_t;
                if byte as libc::c_int > name_len as libc::c_int {
                    log_warn(
                        b"dns\0" as *const u8 as *const libc::c_char,
                        b"Exceeded static name field allocation.\0" as *const u8
                            as *const libc::c_char,
                    );
                    return 0 as libc::c_int as uint16_t;
                }
                if !(data_len as libc::c_int > 0 as libc::c_int) {
                    __assert_fail(
                        b"data_len > 0\0" as *const u8 as *const libc::c_char,
                        b"src/probe_modules/module_dns.c\0" as *const u8
                            as *const libc::c_char,
                        335 as libc::c_uint,
                        b"get_name_helper\0" as *const u8 as *const libc::c_char,
                    );
                }
                memcpy(
                    name as *mut libc::c_void,
                    data as *const libc::c_void,
                    byte as size_t as _,
                );
                name = name.offset(byte as libc::c_int as isize);
                name_len = (name_len as libc::c_int - byte as libc::c_int) as uint16_t;
                data_len = (data_len as libc::c_int - byte as libc::c_int) as uint16_t;
                data = data.offset(byte as libc::c_int as isize);
                bytes_consumed = (bytes_consumed as libc::c_int + byte as libc::c_int)
                    as uint16_t;
                if !(data_len as libc::c_int > 0 as libc::c_int) {
                    __assert_fail(
                        b"data_len > 0\0" as *const u8 as *const libc::c_char,
                        b"src/probe_modules/module_dns.c\0" as *const u8
                            as *const libc::c_char,
                        343 as libc::c_uint,
                        b"get_name_helper\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
    }
    __assert_fail(
        b"0\0" as *const u8 as *const libc::c_char,
        b"src/probe_modules/module_dns.c\0" as *const u8 as *const libc::c_char,
        352 as libc::c_uint,
        b"get_name_helper\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn get_name(
    mut data: *const libc::c_char,
    mut data_len: uint16_t,
    mut payload: *const libc::c_char,
    mut payload_len: uint16_t,
    mut bytes_consumed: *mut uint16_t,
) -> *mut libc::c_char {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = xmalloc(512 as libc::c_int as size_t);
    name = tmp as *mut libc::c_char;
    *bytes_consumed = get_name_helper(
        data,
        data_len,
        payload,
        payload_len,
        name,
        511 as libc::c_int as uint16_t,
        0 as libc::c_int as uint16_t,
    );
    if *bytes_consumed as libc::c_int == 0 as libc::c_int {
        free(name as *mut libc::c_void);
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    if !(*name.offset(511 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int) {
        __assert_fail(
            b"name[MAX_NAME_LENGTH - 1] == '\\0'\0" as *const u8 as *const libc::c_char,
            b"src/probe_modules/module_dns.c\0" as *const u8 as *const libc::c_char,
            370 as libc::c_uint,
            b"get_name\0" as *const u8 as *const libc::c_char,
        );
    }
    return name;
}
unsafe extern "C" fn process_response_question(
    mut data: *mut *mut libc::c_char,
    mut data_len: *mut uint16_t,
    mut payload: *const libc::c_char,
    mut payload_len: uint16_t,
    mut list: *mut fieldset_t,
) -> bool_0 {
    let mut bytes_consumed: uint16_t = 0;
    let mut question_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tail: *mut dns_question_tail = 0 as *mut dns_question_tail;
    let mut qtype: uint16_t = 0;
    let mut tmp___1: __uint16_t = 0;
    let mut qclass: uint16_t = 0;
    let mut tmp___2: __uint16_t = 0;
    let mut qfs: *mut fieldset_t = 0 as *mut fieldset_t;
    let mut tmp___3: *mut fieldset_t = 0 as *mut fieldset_t;
    bytes_consumed = 0 as libc::c_int as uint16_t;
    tmp = get_name(
        *data as *const libc::c_char,
        *data_len,
        payload,
        payload_len,
        &mut bytes_consumed,
    );
    question_name = tmp;
    if question_name as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 1 as libc::c_int as bool_0;
    }
    if !(bytes_consumed as libc::c_int > 0 as libc::c_int) {
        __assert_fail(
            b"bytes_consumed > 0\0" as *const u8 as *const libc::c_char,
            b"src/probe_modules/module_dns.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_uint,
            b"process_response_question\0" as *const u8 as *const libc::c_char,
        );
    }
    if (bytes_consumed as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<dns_question_tail>() as libc::c_ulong)
        > *data_len as libc::c_ulong
    {
        free(question_name as *mut libc::c_void);
        return 1 as libc::c_int as bool_0;
    }
    tail = (*data).offset(bytes_consumed as libc::c_int as isize)
        as *mut dns_question_tail;
    tmp___1 = __bswap_16((*tail).qtype);
    qtype = tmp___1;
    tmp___2 = __bswap_16((*tail).qclass);
    qclass = tmp___2;
    tmp___3 = fs_new_fieldset(0 as *mut libc::c_void as *mut fielddefset_t);
    qfs = tmp___3;
    fs_add_unsafe_string(
        qfs,
        b"name\0" as *const u8 as *const libc::c_char,
        question_name,
        1 as libc::c_int,
    );
    fs_add_uint64(
        qfs,
        b"qtype\0" as *const u8 as *const libc::c_char,
        qtype as uint64_t,
    );
    if qtype as libc::c_int > 255 as libc::c_int {
        fs_add_string(
            qfs,
            b"qtype_str\0" as *const u8 as *const libc::c_char,
            b"BAD QTYPE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int,
        );
    } else if qtype_qtype_to_strid[qtype as usize] as libc::c_int == -(1 as libc::c_int)
        {
        fs_add_string(
            qfs,
            b"qtype_str\0" as *const u8 as *const libc::c_char,
            b"BAD QTYPE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int,
        );
    } else {
        fs_add_string(
            qfs,
            b"qtype_str\0" as *const u8 as *const libc::c_char,
            qtype_strs[qtype_qtype_to_strid[qtype as usize] as usize]
                as *mut libc::c_char,
            0 as libc::c_int,
        );
    }
    fs_add_uint64(
        qfs,
        b"qclass\0" as *const u8 as *const libc::c_char,
        qclass as uint64_t,
    );
    fs_add_fieldset(list, 0 as *mut libc::c_void as *const libc::c_char, qfs);
    *data = (*data)
        .offset(bytes_consumed as libc::c_int as isize)
        .offset(::std::mem::size_of::<dns_question_tail>() as libc::c_ulong as isize);
    *data_len = ((*data_len as libc::c_int - bytes_consumed as libc::c_int)
        as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<dns_question_tail>() as libc::c_ulong)
        as uint16_t;
    return 0 as libc::c_int as bool_0;
}
unsafe extern "C" fn dns_global_initialize(mut conf: *mut state_conf) -> libc::c_int {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut qtype_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut domains: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    let mut arg_strlen: libc::c_int = 0;
    let mut tmp___5: size_t = 0;
    let mut arg_pos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i___0: libc::c_int = 0;
    let mut probe_q_delimiter_p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut probe_arg_delimiter_p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___7: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___8: size_t = 0;
    let mut domain_len: libc::c_int = 0;
    let mut tmp___9: size_t = 0;
    let mut tmp___11: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___12: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut max_payload_len: size_t = 0;
    let mut ret: libc::c_int = 0;
    let mut tmp___13: libc::c_int = 0;
    num_questions = (*conf).packet_streams;
    if num_questions < 1 as libc::c_int {
        log_fatal(
            b"dns\0" as *const u8 as *const libc::c_char,
            b"Invalid number of probes for the DNS module: %i\0" as *const u8
                as *const libc::c_char,
            num_questions,
        );
    }
    tmp = xmalloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(num_questions as libc::c_ulong),
    );
    dns_packets = tmp as *mut *mut libc::c_char;
    tmp___0 = xmalloc(
        (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
            .wrapping_mul(num_questions as libc::c_ulong),
    );
    dns_packet_lens = tmp___0 as *mut uint16_t;
    tmp___1 = xmalloc(
        (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
            .wrapping_mul(num_questions as libc::c_ulong),
    );
    qname_lens = tmp___1 as *mut uint16_t;
    tmp___2 = xmalloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(num_questions as libc::c_ulong),
    );
    qnames = tmp___2 as *mut *mut libc::c_char;
    tmp___3 = xmalloc(
        (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
            .wrapping_mul(num_questions as libc::c_ulong),
    );
    qtypes = tmp___3 as *mut uint16_t;
    qtype_str = 0 as *mut libc::c_void as *mut libc::c_char;
    tmp___4 = xmalloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(num_questions as libc::c_ulong),
    );
    domains = tmp___4 as *mut *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < num_questions {
        let ref mut fresh18 = *domains.offset(i as isize);
        *fresh18 = default_domain.as_ptr() as *mut libc::c_char;
        *qtypes.offset(i as isize) = default_qtype;
        i += 1;
    }
    num_ports___4 = (*conf).source_port_last as libc::c_int
        - (*conf).source_port_first as libc::c_int + 1 as libc::c_int;
    setup_qtype_str_map();
    if !((*conf).probe_args).is_null() {
        tmp___5 = strlen((*conf).probe_args as *const libc::c_char);
        arg_strlen = tmp___5 as libc::c_int;
        arg_pos = (*conf).probe_args;
        i___0 = 0 as libc::c_int;
        while i___0 < num_questions {
            if arg_pos as libc::c_ulong
                >= ((*conf).probe_args).offset(arg_strlen as isize) as libc::c_ulong
            {
                log_fatal(
                    b"dns\0" as *const u8 as *const libc::c_char,
                    b"More probes than questions configured. Add additional questions.\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            tmp___6 = strchr(arg_pos as *const libc::c_char, ',' as i32);
            probe_q_delimiter_p = tmp___6;
            tmp___7 = strchr(arg_pos as *const libc::c_char, ';' as i32);
            probe_arg_delimiter_p = tmp___7;
            if probe_q_delimiter_p as libc::c_ulong
                == 0 as *mut libc::c_void as libc::c_ulong
            {
                log_fatal(
                    b"dns\0" as *const u8 as *const libc::c_char,
                    b"Invalid probe args. Format: \"A,google.com\" or \"A,google.com;A,example.com\"\0"
                        as *const u8 as *const libc::c_char,
                );
            } else {
                if probe_q_delimiter_p as libc::c_ulong == arg_pos as libc::c_ulong {
                    log_fatal(
                        b"dns\0" as *const u8 as *const libc::c_char,
                        b"Invalid probe args. Format: \"A,google.com\" or \"A,google.com;A,example.com\"\0"
                            as *const u8 as *const libc::c_char,
                    );
                } else {
                    tmp___8 = strlen(arg_pos as *const libc::c_char);
                    if arg_pos.offset(tmp___8 as isize) as libc::c_ulong
                        == probe_q_delimiter_p.offset(1 as libc::c_int as isize)
                            as libc::c_ulong
                    {
                        log_fatal(
                            b"dns\0" as *const u8 as *const libc::c_char,
                            b"Invalid probe args. Format: \"A,google.com\" or \"A,google.com;A,example.com\"\0"
                                as *const u8 as *const libc::c_char,
                        );
                    } else {
                        if probe_arg_delimiter_p as libc::c_ulong
                            == 0 as *mut libc::c_void as libc::c_ulong
                        {
                            if i___0 + 1 as libc::c_int != num_questions {
                                log_fatal(
                                    b"dns\0" as *const u8 as *const libc::c_char,
                                    b"Invalid probe args. Format: \"A,google.com\" or \"A,google.com;A,example.com\"\0"
                                        as *const u8 as *const libc::c_char,
                                );
                            }
                        }
                    }
                }
            }
            domain_len = 0 as libc::c_int;
            if !probe_arg_delimiter_p.is_null() {
                domain_len = (probe_arg_delimiter_p.offset_from(probe_q_delimiter_p)
                    as libc::c_long - 1 as libc::c_long) as libc::c_int;
            } else {
                tmp___9 = strlen(probe_q_delimiter_p as *const libc::c_char);
                domain_len = tmp___9 as libc::c_int;
            }
            if !(domain_len > 0 as libc::c_int) {
                __assert_fail(
                    b"domain_len > 0\0" as *const u8 as *const libc::c_char,
                    b"src/probe_modules/module_dns.c\0" as *const u8
                        as *const libc::c_char,
                    637 as libc::c_uint,
                    b"dns_global_initialize\0" as *const u8 as *const libc::c_char,
                );
            }
            tmp___11 = xmalloc((domain_len + 1 as libc::c_int) as size_t);
            let ref mut fresh19 = *domains.offset(i___0 as isize);
            *fresh19 = tmp___11 as *mut libc::c_char;
            strncpy(
                *domains.offset(i___0 as isize),
                probe_q_delimiter_p.offset(1 as libc::c_int as isize)
                    as *const libc::c_char,
                domain_len as size_t as _,
            );
            *(*domains.offset(i___0 as isize))
                .offset(domain_len as isize) = '\u{0}' as i32 as libc::c_char;
            tmp___12 = xmalloc(
                (probe_q_delimiter_p.offset_from(arg_pos) as libc::c_long
                    + 1 as libc::c_long) as size_t,
            );
            qtype_str = tmp___12 as *mut libc::c_char;
            strncpy(
                qtype_str,
                arg_pos as *const libc::c_char,
                probe_q_delimiter_p.offset_from(arg_pos) as libc::c_long as size_t as _,
            );
            *qtype_str
                .offset(
                    probe_q_delimiter_p.offset_from(arg_pos) as libc::c_long as isize,
                ) = '\u{0}' as i32 as libc::c_char;
            *qtypes
                .offset(
                    i___0 as isize,
                ) = qtype_str_to_code(qtype_str as *const libc::c_char);
            free(qtype_str as *mut libc::c_void);
            if *qtypes.offset(i___0 as isize) == 0 {
                log_fatal(
                    b"dns\0" as *const u8 as *const libc::c_char,
                    b"Incorrect qtype supplied. %s\0" as *const u8
                        as *const libc::c_char,
                    qtype_str,
                );
            }
            arg_pos = probe_q_delimiter_p
                .offset(domain_len as isize)
                .offset(2 as libc::c_int as isize);
            i___0 += 1;
        }
        if arg_pos as libc::c_ulong
            != ((*conf).probe_args)
                .offset(arg_strlen as isize)
                .offset(2 as libc::c_int as isize) as libc::c_ulong
        {
            log_fatal(
                b"dns\0" as *const u8 as *const libc::c_char,
                b"More args than probes passed. Add additional probes.\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    tmp___13 = build_global_dns_packets(domains, num_questions, &mut max_payload_len);
    ret = tmp___13;
    module_dns
        .max_packet_length = max_payload_len
        .wrapping_add(::std::mem::size_of::<ether_header>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ip>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong);
    return ret;
}
unsafe extern "C" fn dns_global_cleanup(
    mut zconf___0: *mut state_conf,
    mut zsend___0: *mut state_send,
    mut zrecv___0: *mut state_recv,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut i___0: libc::c_int = 0;
    if !dns_packets.is_null() {
        i = 0 as libc::c_int;
        while i < num_questions {
            if !(*dns_packets.offset(i as isize)).is_null() {
                free(*dns_packets.offset(i as isize) as *mut libc::c_void);
            }
            i += 1;
        }
        free(dns_packets as *mut libc::c_void);
    }
    dns_packets = 0 as *mut libc::c_void as *mut *mut libc::c_char;
    if !qnames.is_null() {
        i___0 = 0 as libc::c_int;
        while i___0 < num_questions {
            if !(*qnames.offset(i___0 as isize)).is_null() {
                free(*qnames.offset(i___0 as isize) as *mut libc::c_void);
            }
            i___0 += 1;
        }
        free(qnames as *mut libc::c_void);
    }
    qnames = 0 as *mut libc::c_void as *mut *mut libc::c_char;
    if !dns_packet_lens.is_null() {
        free(dns_packet_lens as *mut libc::c_void);
    }
    if !qname_lens.is_null() {
        free(qname_lens as *mut libc::c_void);
    }
    if !qtypes.is_null() {
        free(qtypes as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dns_init_perthread(
    mut buf: *mut libc::c_void,
    mut src: *mut macaddr_t,
    mut gw: *mut macaddr_t,
    mut dst_port: port_h_t,
    mut arg_ptr: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut eth_header: *mut ether_header = 0 as *mut ether_header;
    let mut ip_header: *mut ip = 0 as *mut ip;
    let mut len: uint16_t = 0;
    let mut tmp: __uint16_t = 0;
    let mut udp_header: *mut udphdr = 0 as *mut udphdr;
    let mut payload: *mut libc::c_char = 0 as *mut libc::c_char;
    memset(buf, 0 as libc::c_int, 4096 as libc::c_int as size_t as _);
    eth_header = buf as *mut ether_header;
    make_eth_header(eth_header, src, gw);
    ip_header = eth_header.offset(1 as libc::c_int as isize) as *mut ip;
    tmp = __bswap_16(
        (::std::mem::size_of::<ip>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong)
            .wrapping_add(
                *dns_packet_lens.offset(0 as libc::c_int as isize) as libc::c_ulong,
            ) as __uint16_t,
    );
    len = tmp;
    make_ip_header(ip_header, 17 as libc::c_int as uint8_t, len);
    udp_header = ip_header.offset(1 as libc::c_int as isize) as *mut udphdr;
    len = (::std::mem::size_of::<udphdr>() as libc::c_ulong)
        .wrapping_add(
            *dns_packet_lens.offset(0 as libc::c_int as isize) as libc::c_ulong,
        ) as uint16_t;
    make_udp_header(udp_header, zconf.target_port, len);
    payload = udp_header.offset(1 as libc::c_int as isize) as *mut libc::c_char;
    memcpy(
        payload as *mut libc::c_void,
        *dns_packets.offset(0 as libc::c_int as isize) as *const libc::c_void,
        *dns_packet_lens.offset(0 as libc::c_int as isize) as size_t as _,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dns_make_packet(
    mut buf: *mut libc::c_void,
    mut buf_len: *mut size_t,
    mut src_ip: ipaddr_n_t,
    mut dst_ip: ipaddr_n_t,
    mut ttl: uint8_t,
    mut validation: *mut uint32_t,
    mut probe_num: libc::c_int,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    let mut eth_header: *mut ether_header = 0 as *mut ether_header;
    let mut ip_header: *mut ip = 0 as *mut ip;
    let mut udp_header: *mut udphdr = 0 as *mut udphdr;
    let mut encoded_len: uint16_t = 0;
    let mut tmp: __uint16_t = 0;
    let mut tmp___0: __uint16_t = 0;
    let mut payload: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: uint16_t = 0;
    let mut dns_header_p: *mut dns_header = 0 as *mut dns_header;
    eth_header = buf as *mut ether_header;
    ip_header = eth_header.offset(1 as libc::c_int as isize) as *mut ip;
    udp_header = ip_header.offset(1 as libc::c_int as isize) as *mut udphdr;
    if num_questions > 1 as libc::c_int {
        tmp = __bswap_16(
            (::std::mem::size_of::<ip>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong)
                .wrapping_add(
                    *dns_packet_lens.offset(probe_num as isize) as libc::c_ulong,
                ) as __uint16_t,
        );
        encoded_len = tmp;
        make_ip_header(ip_header, 17 as libc::c_int as uint8_t, encoded_len);
        encoded_len = (::std::mem::size_of::<udphdr>() as libc::c_ulong)
            .wrapping_add(*dns_packet_lens.offset(probe_num as isize) as libc::c_ulong)
            as uint16_t;
        tmp___0 = __bswap_16((*udp_header).__annonCompField5.__annonCompField3.uh_dport);
        make_udp_header(udp_header, tmp___0, encoded_len);
        payload = udp_header.offset(1 as libc::c_int as isize) as *mut libc::c_char;
        *buf_len = (::std::mem::size_of::<ether_header>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<ip>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong)
            .wrapping_add(*dns_packet_lens.offset(probe_num as isize) as libc::c_ulong);
        if !(*buf_len <= 4096 as libc::c_ulong) {
            __assert_fail(
                b"*buf_len <= MAX_PACKET_SIZE\0" as *const u8 as *const libc::c_char,
                b"src/probe_modules/module_dns.c\0" as *const u8 as *const libc::c_char,
                766 as libc::c_uint,
                b"dns_make_packet\0" as *const u8 as *const libc::c_char,
            );
        }
        memcpy(
            payload as *mut libc::c_void,
            *dns_packets.offset(probe_num as isize) as *const libc::c_void,
            *dns_packet_lens.offset(probe_num as isize) as size_t as _,
        );
    }
    (*ip_header).ip_src.s_addr = src_ip;
    (*ip_header).ip_dst.s_addr = dst_ip;
    (*ip_header).ip_ttl = ttl;
    tmp___2 = get_src_port(num_ports___4, probe_num, validation);
    (*udp_header).__annonCompField5.__annonCompField3.uh_sport = __bswap_16(tmp___2);
    dns_header_p = udp_header.offset(1 as libc::c_int as isize) as *mut dns_header;
    (*dns_header_p)
        .id = (*validation.offset(2 as libc::c_int as isize) & 65535 as libc::c_uint)
        as uint16_t;
    (*ip_header).ip_sum = 0 as libc::c_int as libc::c_ushort;
    (*ip_header).ip_sum = zmap_ip_checksum(ip_header as *mut libc::c_ushort);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dns_print_packet(
    mut fp: *mut FILE,
    mut packet: *mut libc::c_void,
) {
    let mut ethh: *mut ether_header = 0 as *mut ether_header;
    let mut iph: *mut ip = 0 as *mut ip;
    let mut udph: *mut udphdr = 0 as *mut udphdr;
    let mut tmp: __uint16_t = 0;
    let mut tmp___0: __uint16_t = 0;
    let mut tmp___1: __uint16_t = 0;
    ethh = packet as *mut ether_header;
    iph = ethh.offset(1 as libc::c_int as isize) as *mut ip;
    udph = iph.offset(1 as libc::c_int as isize) as *mut udphdr;
    __fprintf_chk(
        fp,
        1 as libc::c_int,
        b"------------------------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    tmp = __bswap_16((*udph).__annonCompField5.__annonCompField3.uh_sum);
    tmp___0 = __bswap_16((*udph).__annonCompField5.__annonCompField3.uh_dport);
    tmp___1 = __bswap_16((*udph).__annonCompField5.__annonCompField3.uh_sport);
    __fprintf_chk(
        fp,
        1 as libc::c_int,
        b"dns { source: %u | dest: %u | checksum: %#04X }\n\0" as *const u8
            as *const libc::c_char,
        tmp___1 as libc::c_int,
        tmp___0 as libc::c_int,
        tmp as libc::c_int,
    );
    fprintf_ip_header(fp, iph);
    fprintf_eth_header(fp, ethh);
    __fprintf_chk(
        fp,
        1 as libc::c_int,
        b"------------------------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
}
pub unsafe extern "C" fn dns_validate_packet(
    mut ip_hdr: *const ip,
    mut len: uint32_t,
    mut src_ip: *mut uint32_t,
    mut validation: *mut uint32_t,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut udp: *mut udphdr = 0 as *mut udphdr;
    let mut tmp___0: *mut udphdr = 0 as *mut udphdr;
    let mut udp_len: uint16_t = 0;
    let mut tmp___1: __uint16_t = 0;
    let mut match_0: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    tmp = udp_do_validate_packet(
        ip_hdr,
        len,
        src_ip,
        validation,
        num_ports___4,
        zconf.target_port as libc::c_int,
    );
    if tmp == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*ip_hdr).ip_p as libc::c_int == 17 as libc::c_int {
        tmp___0 = get_udp_header(ip_hdr, len);
        udp = tmp___0;
        if udp.is_null() {
            return 0 as libc::c_int;
        }
        tmp___1 = __bswap_16((*udp).__annonCompField5.__annonCompField3.uh_ulen);
        udp_len = tmp___1;
        match_0 = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < num_questions {
            if udp_len as libc::c_int
                >= *dns_packet_lens.offset(i as isize) as libc::c_int
            {
                match_0 += 1;
            }
            i += 1;
        }
        if match_0 == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        if len < udp_len as uint32_t {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn dns_add_null_fs(mut fs: *mut fieldset_t) {
    let mut tmp: *mut fieldset_t = 0 as *mut fieldset_t;
    let mut tmp___0: *mut fieldset_t = 0 as *mut fieldset_t;
    let mut tmp___1: *mut fieldset_t = 0 as *mut fieldset_t;
    let mut tmp___2: *mut fieldset_t = 0 as *mut fieldset_t;
    fs_add_null(fs, b"dns_id\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"dns_rd\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"dns_tc\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"dns_aa\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"dns_opcode\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"dns_qr\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"dns_rcode\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"dns_cd\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"dns_ad\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"dns_z\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"dns_ra\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"dns_qdcount\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"dns_ancount\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"dns_nscount\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"dns_arcount\0" as *const u8 as *const libc::c_char);
    tmp = fs_new_repeated_fieldset();
    fs_add_repeated(fs, b"dns_questions\0" as *const u8 as *const libc::c_char, tmp);
    tmp___0 = fs_new_repeated_fieldset();
    fs_add_repeated(fs, b"dns_answers\0" as *const u8 as *const libc::c_char, tmp___0);
    tmp___1 = fs_new_repeated_fieldset();
    fs_add_repeated(
        fs,
        b"dns_authorities\0" as *const u8 as *const libc::c_char,
        tmp___1,
    );
    tmp___2 = fs_new_repeated_fieldset();
    fs_add_repeated(
        fs,
        b"dns_additionals\0" as *const u8 as *const libc::c_char,
        tmp___2,
    );
    fs_add_uint64(
        fs,
        b"dns_parse_err\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as uint64_t,
    );
    fs_add_uint64(
        fs,
        b"dns_unconsumed_bytes\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as uint64_t,
    );
}
pub unsafe extern "C" fn dns_process_packet(
    mut packet: *const u_char,
    mut len: uint32_t,
    mut fs: *mut fieldset_t,
    mut validation: *mut uint32_t,
    mut ts: timespec,
) {
    let mut ip_hdr: *mut ip = 0 as *mut ip;
    let mut udp_hdr: *mut udphdr = 0 as *mut udphdr;
    let mut tmp: *mut udphdr = 0 as *mut udphdr;
    let mut udp_len: uint16_t = 0;
    let mut tmp___1: __uint16_t = 0;
    let mut match_0: libc::c_int = 0;
    let mut is_valid: bool_0 = 0;
    let mut i: libc::c_int = 0;
    let mut qname_p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tail_p: *mut dns_question_tail = 0 as *mut dns_question_tail;
    let mut dns_header_p: *mut dns_header = 0 as *mut dns_header;
    let mut tmp___2: __uint16_t = 0;
    let mut tmp___3: __uint16_t = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut dns_hdr: *mut dns_header = 0 as *mut dns_header;
    let mut qr: uint16_t = 0;
    let mut rcode: uint16_t = 0;
    let mut tmp___6: __uint16_t = 0;
    let mut tmp___7: __uint16_t = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: __uint16_t = 0;
    let mut tmp___10: __uint16_t = 0;
    let mut tmp___11: __uint16_t = 0;
    let mut tmp___12: __uint16_t = 0;
    let mut tmp___13: __uint16_t = 0;
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut data_len: uint16_t = 0;
    let mut err: bool_0 = 0;
    let mut list: *mut fieldset_t = 0 as *mut fieldset_t;
    let mut tmp___14: *mut fieldset_t = 0 as *mut fieldset_t;
    let mut i___0: libc::c_int = 0;
    let mut tmp___15: __uint16_t = 0;
    let mut i___1: libc::c_int = 0;
    let mut tmp___16: __uint16_t = 0;
    let mut i___2: libc::c_int = 0;
    let mut tmp___17: __uint16_t = 0;
    let mut i___3: libc::c_int = 0;
    let mut tmp___18: __uint16_t = 0;
    ip_hdr = packet
        .offset(::std::mem::size_of::<ether_header>() as libc::c_ulong as isize)
        as *mut ip;
    if (*ip_hdr).ip_p as libc::c_int == 17 as libc::c_int {
        tmp = get_udp_header(ip_hdr as *const ip, len);
        udp_hdr = tmp;
        if udp_hdr.is_null() {
            __assert_fail(
                b"udp_hdr\0" as *const u8 as *const libc::c_char,
                b"src/probe_modules/module_dns.c\0" as *const u8 as *const libc::c_char,
                869 as libc::c_uint,
                b"dns_process_packet\0" as *const u8 as *const libc::c_char,
            );
        }
        tmp___1 = __bswap_16((*udp_hdr).__annonCompField5.__annonCompField3.uh_ulen);
        udp_len = tmp___1;
        match_0 = 0 as libc::c_int;
        is_valid = 0 as libc::c_int as bool_0;
        i = 0 as libc::c_int;
        while i < num_questions {
            if !((udp_len as libc::c_int)
                < *dns_packet_lens.offset(i as isize) as libc::c_int)
            {
                match_0 += 1;
                qname_p = 0 as *mut libc::c_void as *mut libc::c_char;
                tail_p = 0 as *mut libc::c_void as *mut dns_question_tail;
                dns_header_p = udp_hdr.offset(1 as libc::c_int as isize)
                    as *mut dns_header;
                if (*dns_header_p).id as libc::c_uint
                    == *validation.offset(2 as libc::c_int as isize)
                        & 65535 as libc::c_uint
                {
                    qname_p = (dns_header_p as *mut libc::c_char)
                        .offset(
                            ::std::mem::size_of::<dns_header>() as libc::c_ulong as isize,
                        );
                    tail_p = (*dns_packets.offset(i as isize))
                        .offset(
                            ::std::mem::size_of::<dns_header>() as libc::c_ulong as isize,
                        )
                        .offset(*qname_lens.offset(i as isize) as libc::c_int as isize)
                        as *mut dns_question_tail;
                    tmp___4 = strcmp(
                        *qnames.offset(i as isize) as *const libc::c_char,
                        qname_p as *const libc::c_char,
                    );
                    if tmp___4 == 0 as libc::c_int {
                        tmp___2 = __bswap_16(*qtypes.offset(i as isize));
                        if (*tail_p).qtype as libc::c_int == tmp___2 as libc::c_int {
                            tmp___3 = __bswap_16(1 as libc::c_int as __uint16_t);
                            if (*tail_p).qclass as libc::c_int == tmp___3 as libc::c_int
                            {
                                is_valid = 1 as libc::c_int as bool_0;
                                break;
                            }
                        }
                    }
                }
            }
            i += 1;
        }
        if !(match_0 > 0 as libc::c_int) {
            __assert_fail(
                b"match > 0\0" as *const u8 as *const libc::c_char,
                b"src/probe_modules/module_dns.c\0" as *const u8 as *const libc::c_char,
                903 as libc::c_uint,
                b"dns_process_packet\0" as *const u8 as *const libc::c_char,
            );
        }
        dns_hdr = udp_hdr.offset(1 as libc::c_int as isize) as *mut dns_header;
        qr = (*dns_hdr).qr() as uint16_t;
        rcode = (*dns_hdr).rcode() as uint16_t;
        tmp___6 = __bswap_16((*udp_hdr).__annonCompField5.__annonCompField3.uh_sport);
        fs_add_uint64(
            fs,
            b"sport\0" as *const u8 as *const libc::c_char,
            tmp___6 as uint64_t,
        );
        tmp___7 = __bswap_16((*udp_hdr).__annonCompField5.__annonCompField3.uh_dport);
        fs_add_uint64(
            fs,
            b"dport\0" as *const u8 as *const libc::c_char,
            tmp___7 as uint64_t,
        );
        fs_add_string(
            fs,
            b"classification\0" as *const u8 as *const libc::c_char,
            b"dns\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int,
        );
        fs_add_bool(
            fs,
            b"success\0" as *const u8 as *const libc::c_char,
            is_valid as libc::c_int,
        );
        if is_valid != 0 {
            if qr as libc::c_int == 1 as libc::c_int {
                if rcode as libc::c_int == 0 as libc::c_int {
                    tmp___8 = 1 as libc::c_int;
                } else {
                    tmp___8 = 0 as libc::c_int;
                }
            } else {
                tmp___8 = 0 as libc::c_int;
            }
        } else {
            tmp___8 = 0 as libc::c_int;
        }
        fs_add_bool(fs, b"app_success\0" as *const u8 as *const libc::c_char, tmp___8);
        fs_add_null_icmp(fs);
        fs_add_uint64(
            fs,
            b"udp_len\0" as *const u8 as *const libc::c_char,
            udp_len as uint64_t,
        );
        if is_valid == 0 {
            dns_add_null_fs(fs);
        } else {
            tmp___9 = __bswap_16((*dns_hdr).id);
            fs_add_uint64(
                fs,
                b"dns_id\0" as *const u8 as *const libc::c_char,
                tmp___9 as uint64_t,
            );
            fs_add_uint64(
                fs,
                b"dns_rd\0" as *const u8 as *const libc::c_char,
                (*dns_hdr).rd() as uint64_t,
            );
            fs_add_uint64(
                fs,
                b"dns_tc\0" as *const u8 as *const libc::c_char,
                (*dns_hdr).tc() as uint64_t,
            );
            fs_add_uint64(
                fs,
                b"dns_aa\0" as *const u8 as *const libc::c_char,
                (*dns_hdr).aa() as uint64_t,
            );
            fs_add_uint64(
                fs,
                b"dns_opcode\0" as *const u8 as *const libc::c_char,
                (*dns_hdr).opcode() as uint64_t,
            );
            fs_add_uint64(
                fs,
                b"dns_qr\0" as *const u8 as *const libc::c_char,
                qr as uint64_t,
            );
            fs_add_uint64(
                fs,
                b"dns_rcode\0" as *const u8 as *const libc::c_char,
                rcode as uint64_t,
            );
            fs_add_uint64(
                fs,
                b"dns_cd\0" as *const u8 as *const libc::c_char,
                (*dns_hdr).cd() as uint64_t,
            );
            fs_add_uint64(
                fs,
                b"dns_ad\0" as *const u8 as *const libc::c_char,
                (*dns_hdr).ad() as uint64_t,
            );
            fs_add_uint64(
                fs,
                b"dns_z\0" as *const u8 as *const libc::c_char,
                (*dns_hdr).z() as uint64_t,
            );
            fs_add_uint64(
                fs,
                b"dns_ra\0" as *const u8 as *const libc::c_char,
                (*dns_hdr).ra() as uint64_t,
            );
            tmp___10 = __bswap_16((*dns_hdr).qdcount);
            fs_add_uint64(
                fs,
                b"dns_qdcount\0" as *const u8 as *const libc::c_char,
                tmp___10 as uint64_t,
            );
            tmp___11 = __bswap_16((*dns_hdr).ancount);
            fs_add_uint64(
                fs,
                b"dns_ancount\0" as *const u8 as *const libc::c_char,
                tmp___11 as uint64_t,
            );
            tmp___12 = __bswap_16((*dns_hdr).nscount);
            fs_add_uint64(
                fs,
                b"dns_nscount\0" as *const u8 as *const libc::c_char,
                tmp___12 as uint64_t,
            );
            tmp___13 = __bswap_16((*dns_hdr).arcount);
            fs_add_uint64(
                fs,
                b"dns_arcount\0" as *const u8 as *const libc::c_char,
                tmp___13 as uint64_t,
            );
            data = (dns_hdr as *mut libc::c_char)
                .offset(::std::mem::size_of::<dns_header>() as libc::c_ulong as isize);
            data_len = (udp_len as libc::c_ulong)
                .wrapping_sub(::std::mem::size_of::<*mut udphdr>() as libc::c_ulong)
                .wrapping_sub(::std::mem::size_of::<dns_header>() as libc::c_ulong)
                as uint16_t;
            err = 0 as libc::c_int as bool_0;
            tmp___14 = fs_new_repeated_fieldset();
            list = tmp___14;
            i___0 = 0 as libc::c_int;
            loop {
                tmp___15 = __bswap_16((*dns_hdr).qdcount);
                if !(i___0 < tmp___15 as libc::c_int) {
                    break;
                }
                if err != 0 {
                    break;
                }
                err = process_response_question(
                    &mut data,
                    &mut data_len,
                    dns_hdr as *mut libc::c_char as *const libc::c_char,
                    udp_len,
                    list,
                );
                i___0 += 1;
            }
            fs_add_repeated(
                fs,
                b"dns_questions\0" as *const u8 as *const libc::c_char,
                list,
            );
            list = fs_new_repeated_fieldset();
            i___1 = 0 as libc::c_int;
            loop {
                tmp___16 = __bswap_16((*dns_hdr).ancount);
                if !(i___1 < tmp___16 as libc::c_int) {
                    break;
                }
                if err != 0 {
                    break;
                }
                err = 0;
                i___1 += 1;
            }
            fs_add_repeated(
                fs,
                b"dns_answers\0" as *const u8 as *const libc::c_char,
                list,
            );
            list = fs_new_repeated_fieldset();
            i___2 = 0 as libc::c_int;
            loop {
                tmp___17 = __bswap_16((*dns_hdr).nscount);
                if !(i___2 < tmp___17 as libc::c_int) {
                    break;
                }
                if err != 0 {
                    break;
                }
                err = 0;
                i___2 += 1;
            }
            fs_add_repeated(
                fs,
                b"dns_authorities\0" as *const u8 as *const libc::c_char,
                list,
            );
            list = fs_new_repeated_fieldset();
            i___3 = 0 as libc::c_int;
            loop {
                tmp___18 = __bswap_16((*dns_hdr).arcount);
                if !(i___3 < tmp___18 as libc::c_int) {
                    break;
                }
                if err != 0 {
                    break;
                }
                err = 0;
                i___3 += 1;
            }
            fs_add_repeated(
                fs,
                b"dns_additionals\0" as *const u8 as *const libc::c_char,
                list,
            );
            if data_len as libc::c_int != 0 as libc::c_int {
                err = 1 as libc::c_int as bool_0;
            }
            fs_add_uint64(
                fs,
                b"dns_parse_err\0" as *const u8 as *const libc::c_char,
                err as uint64_t,
            );
            fs_add_uint64(
                fs,
                b"dns_unconsumed_bytes\0" as *const u8 as *const libc::c_char,
                data_len as uint64_t,
            );
        }
        fs_add_binary(
            fs,
            b"raw_data\0" as *const u8 as *const libc::c_char,
            (udp_len as libc::c_ulong)
                .wrapping_sub(::std::mem::size_of::<udphdr>() as libc::c_ulong),
            udp_hdr.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            0 as libc::c_int,
        );
    } else if (*ip_hdr).ip_p as libc::c_int == 1 as libc::c_int {
        fs_add_null(fs, b"sport\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"dport\0" as *const u8 as *const libc::c_char);
        fs_add_constchar(
            fs,
            b"classification\0" as *const u8 as *const libc::c_char,
            b"icmp\0" as *const u8 as *const libc::c_char,
        );
        fs_add_bool(
            fs,
            b"success\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        fs_add_bool(
            fs,
            b"app_success\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        fs_populate_icmp_from_iphdr(ip_hdr, len as size_t, fs);
        fs_add_null(fs, b"udp_len\0" as *const u8 as *const libc::c_char);
        dns_add_null_fs(fs);
        fs_add_binary(
            fs,
            b"raw_data\0" as *const u8 as *const libc::c_char,
            len as size_t,
            packet as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        );
    } else {
        log_fatal(
            b"dns\0" as *const u8 as *const libc::c_char,
            b"Die. This can only happen if you change the pcap filter and don't update the process function.\0"
                as *const u8 as *const libc::c_char,
        );
    };
}
static mut fields___6: [fielddef_t; 32] = [
    {
        let mut init = field_def {
            name: b"sport\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"UDP source port\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dport\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"UDP destination port\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"classification\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"packet classification\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"success\0" as *const u8 as *const libc::c_char,
            type_0: b"bool\0" as *const u8 as *const libc::c_char,
            desc: b"is response considered success\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"app_success\0" as *const u8 as *const libc::c_char,
            type_0: b"bool\0" as *const u8 as *const libc::c_char,
            desc: b"Is the RA bit set with no error code?\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_responder\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"Source IP of ICMP_UNREACH messages\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_type\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"icmp message type\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_code\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"icmp message sub type code\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_unreach_str\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"for icmp_unreach responses, the string version of icmp_code (e.g. network-unreach)\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"udp_len\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"UDP packet lenght\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_id\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS transaction ID\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_rd\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS recursion desired\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_tc\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS packet truncated\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_aa\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS authoritative answer\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_opcode\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS opcode (query type)\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_qr\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS query(0) or response (1)\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_rcode\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS response code\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_cd\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS checking disabled\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_ad\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS authenticated data\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_z\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS reserved\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_ra\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS recursion available\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_qdcount\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS number questions\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_ancount\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS number answer RR's\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_nscount\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS number NS RR's in authority section\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_arcount\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS number additional RR's\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_questions\0" as *const u8 as *const libc::c_char,
            type_0: b"repeated\0" as *const u8 as *const libc::c_char,
            desc: b"DNS question list\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_answers\0" as *const u8 as *const libc::c_char,
            type_0: b"repeated\0" as *const u8 as *const libc::c_char,
            desc: b"DNS answer list\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_authorities\0" as *const u8 as *const libc::c_char,
            type_0: b"repeated\0" as *const u8 as *const libc::c_char,
            desc: b"DNS authority list\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_additionals\0" as *const u8 as *const libc::c_char,
            type_0: b"repeated\0" as *const u8 as *const libc::c_char,
            desc: b"DNS additional list\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_parse_err\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"Problem parsing the DNS response\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_unconsumed_bytes\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"Bytes left over when parsing the DNS response\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"raw_data\0" as *const u8 as *const libc::c_char,
            type_0: b"binary\0" as *const u8 as *const libc::c_char,
            desc: b"UDP payload\0" as *const u8 as *const libc::c_char,
        };
        init
    },
];
pub static mut module_dns: probe_module_t = probe_module_t {
    name: 0 as *const libc::c_char,
    max_packet_length: 0,
    pcap_filter: 0 as *const libc::c_char,
    pcap_snaplen: 0,
    port_args: 0,
    global_initialize: None,
    thread_initialize: None,
    make_packet: None,
    print_packet: None,
    validate_packet: None,
    process_packet: None,
    close: None,
    output_type: 0,
    fields: 0 as *const fielddef_t as *mut fielddef_t,
    numfields: 0,
    helptext: 0 as *const libc::c_char,
};
#[inline]
unsafe extern "C" fn get_udp_payload(
    mut udp: *const udphdr,
    mut len: uint32_t,
) -> *mut uint8_t {
    return udp.offset(1 as libc::c_int as isize) as *mut uint8_t;
}
static mut num_ports___5: libc::c_int = 0;
static mut bacnet_body: [uint8_t; 7] = [
    12 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    63 as libc::c_int as uint8_t,
    255 as libc::c_int as uint8_t,
    255 as libc::c_int as uint8_t,
    25 as libc::c_int as uint8_t,
    75 as libc::c_int as uint8_t,
];
#[inline]
unsafe extern "C" fn get_invoke_id(mut validation: *mut uint32_t) -> uint8_t {
    return (*validation.offset(1 as libc::c_int as isize) >> 24 as libc::c_int
        & 255 as libc::c_uint) as uint8_t;
}
pub unsafe extern "C" fn bacnet_init_perthread(
    mut buf: *mut libc::c_void,
    mut src: *mut macaddr_t,
    mut gw: *mut macaddr_t,
    mut dst_port: port_h_t,
    mut arg: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut eth_header: *mut ether_header = 0 as *mut ether_header;
    let mut ip_header: *mut ip = 0 as *mut ip;
    let mut udp_header: *mut udphdr = 0 as *mut udphdr;
    let mut bnp: *mut bacnet_probe = 0 as *mut bacnet_probe;
    let mut body: *mut uint8_t = 0 as *mut uint8_t;
    let mut ip_len: uint16_t = 0;
    let mut tmp___0: __uint16_t = 0;
    let mut udp_len: uint16_t = 0;
    let mut seed: uint32_t = 0;
    let mut tmp___1: uint64_t = 0;
    let mut aes: *mut aesrand_t = 0 as *mut aesrand_t;
    let mut tmp___2: *mut aesrand_t = 0 as *mut aesrand_t;
    memset(buf, 0 as libc::c_int, 4096 as libc::c_int as size_t as _);
    eth_header = buf as *mut ether_header;
    ip_header = eth_header.offset(1 as libc::c_int as isize) as *mut ip;
    udp_header = ip_header.offset(1 as libc::c_int as isize) as *mut udphdr;
    bnp = udp_header.offset(1 as libc::c_int as isize) as *mut bacnet_probe;
    body = bnp.offset(1 as libc::c_int as isize) as *mut uint8_t;
    make_eth_header(eth_header, src, gw);
    ip_len = (::std::mem::size_of::<ip>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong)
        .wrapping_add(17 as libc::c_ulong) as uint16_t;
    if !(ip_len as libc::c_int <= 4096 as libc::c_int) {
        __assert_fail(
            b"ip_len <= MAX_PACKET_SIZE\0" as *const u8 as *const libc::c_char,
            b"src/probe_modules/module_bacnet.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_uint,
            b"bacnet_init_perthread\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___0 = __bswap_16(ip_len);
    make_ip_header(ip_header, 17 as libc::c_int as uint8_t, tmp___0);
    udp_len = (::std::mem::size_of::<udphdr>() as libc::c_ulong)
        .wrapping_add(17 as libc::c_ulong) as uint16_t;
    make_udp_header(udp_header, zconf.target_port, udp_len);
    (*bnp).vlc.type_0 = 129 as libc::c_int as uint8_t;
    (*bnp).vlc.function = 10 as libc::c_int as uint8_t;
    (*bnp).vlc.length = __bswap_16(17 as libc::c_int as __uint16_t);
    (*bnp).npdu.version = 1 as libc::c_int as uint8_t;
    (*bnp).npdu.control = 4 as libc::c_int as uint8_t;
    (*bnp).apdu.type_flags = 0 as libc::c_int as uint8_t;
    (*bnp).apdu.max_segments_apdu = 5 as libc::c_int as uint8_t;
    (*bnp).apdu.server_choice = 12 as libc::c_int as uint8_t;
    memcpy(
        body as *mut libc::c_void,
        bacnet_body.as_mut_ptr() as *const libc::c_void,
        7 as libc::c_int as size_t as _,
    );
    tmp___1 = aesrand_getword(zconf.aes);
    seed = tmp___1 as uint32_t;
    tmp___2 = aesrand_init_from_seed(seed as uint64_t);
    aes = tmp___2;
    *arg = aes as *mut libc::c_void;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn bacnet_make_packet(
    mut buf: *mut libc::c_void,
    mut buf_len: *mut size_t,
    mut src_ip: ipaddr_n_t,
    mut dst_ip: ipaddr_n_t,
    mut ttl: uint8_t,
    mut validation: *mut uint32_t,
    mut probe_num: libc::c_int,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    let mut eth_header: *mut ether_header = 0 as *mut ether_header;
    let mut ip_header: *mut ip = 0 as *mut ip;
    let mut udp_header: *mut udphdr = 0 as *mut udphdr;
    let mut bnp: *mut bacnet_probe = 0 as *mut bacnet_probe;
    let mut tmp: uint16_t = 0;
    eth_header = buf as *mut ether_header;
    ip_header = eth_header.offset(1 as libc::c_int as isize) as *mut ip;
    udp_header = ip_header.offset(1 as libc::c_int as isize) as *mut udphdr;
    bnp = udp_header.offset(1 as libc::c_int as isize) as *mut bacnet_probe;
    (*ip_header).ip_src.s_addr = src_ip;
    (*ip_header).ip_dst.s_addr = dst_ip;
    (*ip_header).ip_ttl = ttl;
    (*ip_header).ip_sum = 0 as libc::c_int as libc::c_ushort;
    tmp = get_src_port(num_ports___5, probe_num, validation);
    (*udp_header).__annonCompField5.__annonCompField3.uh_sport = __bswap_16(tmp);
    (*bnp).apdu.invoke_id = get_invoke_id(validation);
    (*ip_header).ip_sum = zmap_ip_checksum(ip_header as *mut libc::c_ushort);
    *buf_len = (::std::mem::size_of::<ether_header>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ip>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong)
        .wrapping_add(17 as libc::c_ulong);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn bacnet_validate_packet(
    mut ip_hdr: *const ip,
    mut len: uint32_t,
    mut src_ip: *mut uint32_t,
    mut validation: *mut uint32_t,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut udp: *mut udphdr = 0 as *mut udphdr;
    let mut tmp___0: *mut udphdr = 0 as *mut udphdr;
    let mut min_len: size_t = 0;
    let mut vlc: *mut bacnet_vlc = 0 as *mut bacnet_vlc;
    let mut tmp___1: *mut uint8_t = 0 as *mut uint8_t;
    tmp = udp_do_validate_packet(
        ip_hdr,
        len,
        src_ip,
        validation,
        num_ports___5,
        zconf.target_port as libc::c_int,
    );
    if tmp == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*ip_hdr).ip_p as libc::c_int == 17 as libc::c_int {
        tmp___0 = get_udp_header(ip_hdr, len);
        udp = tmp___0;
        if udp.is_null() {
            return 0 as libc::c_int;
        }
        min_len = (::std::mem::size_of::<udphdr>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<bacnet_vlc>() as libc::c_ulong);
        if ((*udp).__annonCompField5.__annonCompField3.uh_ulen as size_t) < min_len {
            return 0 as libc::c_int;
        }
        tmp___1 = get_udp_payload(udp as *const udphdr, len);
        vlc = tmp___1 as *mut bacnet_vlc;
        if (*vlc).type_0 as libc::c_int != 129 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn bacnet_process_packet(
    mut packet: *const u_char,
    mut len: uint32_t,
    mut fs: *mut fieldset_t,
    mut validation: *mut uint32_t,
    mut ts: timespec,
) {
    let mut ip_hdr: *mut ip = 0 as *mut ip;
    let mut tmp: *mut ip = 0 as *mut ip;
    let mut udp: *mut udphdr = 0 as *mut udphdr;
    let mut tmp___1: *mut udphdr = 0 as *mut udphdr;
    let mut tmp___3: __uint16_t = 0;
    let mut tmp___4: __uint16_t = 0;
    let mut udp_offset: uint32_t = 0;
    let mut payload_offset: uint32_t = 0;
    let mut payload: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___6: *mut uint8_t = 0 as *mut uint8_t;
    let mut payload_len: uint32_t = 0;
    tmp = get_ip_header(packet, len);
    ip_hdr = tmp;
    if ip_hdr.is_null() {
        __assert_fail(
            b"ip_hdr\0" as *const u8 as *const libc::c_char,
            b"src/probe_modules/module_bacnet.c\0" as *const u8 as *const libc::c_char,
            134 as libc::c_uint,
            b"bacnet_process_packet\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*ip_hdr).ip_p as libc::c_int == 17 as libc::c_int {
        tmp___1 = get_udp_header(ip_hdr as *const ip, len);
        udp = tmp___1;
        if udp.is_null() {
            __assert_fail(
                b"udp\0" as *const u8 as *const libc::c_char,
                b"src/probe_modules/module_bacnet.c\0" as *const u8
                    as *const libc::c_char,
                137 as libc::c_uint,
                b"bacnet_process_packet\0" as *const u8 as *const libc::c_char,
            );
        }
        tmp___3 = __bswap_16((*udp).__annonCompField5.__annonCompField3.uh_sport);
        fs_add_uint64(
            fs,
            b"sport\0" as *const u8 as *const libc::c_char,
            tmp___3 as uint64_t,
        );
        tmp___4 = __bswap_16((*udp).__annonCompField5.__annonCompField3.uh_dport);
        fs_add_uint64(
            fs,
            b"dport\0" as *const u8 as *const libc::c_char,
            tmp___4 as uint64_t,
        );
        fs_add_constchar(
            fs,
            b"classification\0" as *const u8 as *const libc::c_char,
            b"bacnet\0" as *const u8 as *const libc::c_char,
        );
        fs_add_bool(
            fs,
            b"success\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
        fs_add_null_icmp(fs);
        udp_offset = (::std::mem::size_of::<ether_header>() as libc::c_ulong)
            .wrapping_add(
                (*ip_hdr).ip_hl().wrapping_mul(4 as libc::c_uint) as libc::c_ulong,
            ) as uint32_t;
        payload_offset = (udp_offset as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong) as uint32_t;
        if !(payload_offset < len) {
            __assert_fail(
                b"payload_offset < len\0" as *const u8 as *const libc::c_char,
                b"src/probe_modules/module_bacnet.c\0" as *const u8
                    as *const libc::c_char,
                145 as libc::c_uint,
                b"bacnet_process_packet\0" as *const u8 as *const libc::c_char,
            );
        }
        tmp___6 = get_udp_payload(udp as *const udphdr, len);
        payload = tmp___6;
        payload_len = len.wrapping_sub(payload_offset);
        fs_add_binary(
            fs,
            b"udp_payload\0" as *const u8 as *const libc::c_char,
            payload_len as size_t,
            payload as *mut libc::c_void,
            0 as libc::c_int,
        );
        fs_add_null_icmp(fs);
    } else if (*ip_hdr).ip_p as libc::c_int == 1 as libc::c_int {
        fs_add_null(fs, b"sport\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"dport\0" as *const u8 as *const libc::c_char);
        fs_add_constchar(
            fs,
            b"classification\0" as *const u8 as *const libc::c_char,
            b"icmp\0" as *const u8 as *const libc::c_char,
        );
        fs_add_bool(
            fs,
            b"success\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        fs_add_null(fs, b"udp_payload\0" as *const u8 as *const libc::c_char);
        fs_populate_icmp_from_iphdr(ip_hdr, len as size_t, fs);
    }
}
pub unsafe extern "C" fn bacnet_global_initialize(
    mut conf: *mut state_conf,
) -> libc::c_int {
    num_ports___5 = (*conf).source_port_last as libc::c_int
        - (*conf).source_port_first as libc::c_int + 1 as libc::c_int;
    return 0 as libc::c_int;
}
static mut fields___7: [fielddef_t; 9] = [
    {
        let mut init = field_def {
            name: b"sport\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"UDP source port\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dport\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"UDP destination port\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"classification\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"packet classification\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"success\0" as *const u8 as *const libc::c_char,
            type_0: b"bool\0" as *const u8 as *const libc::c_char,
            desc: b"is response considered success\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"udp_payload\0" as *const u8 as *const libc::c_char,
            type_0: b"binary\0" as *const u8 as *const libc::c_char,
            desc: b"UDP payload\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_responder\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"Source IP of ICMP_UNREACH messages\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_type\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"icmp message type\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_code\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"icmp message sub type code\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_unreach_str\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"for icmp_unreach responses, the string version of icmp_code (e.g. network-unreach)\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
];
pub static mut module_bacnet: probe_module_t = probe_module_t {
    name: 0 as *const libc::c_char,
    max_packet_length: 0,
    pcap_filter: 0 as *const libc::c_char,
    pcap_snaplen: 0,
    port_args: 0,
    global_initialize: None,
    thread_initialize: None,
    make_packet: None,
    print_packet: None,
    validate_packet: None,
    process_packet: None,
    close: None,
    output_type: 0,
    fields: 0 as *const fielddef_t as *mut fielddef_t,
    numfields: 0,
    helptext: 0 as *const libc::c_char,
};
static mut file: *mut FILE = 0 as *const libc::c_void as *mut libc::c_void as *mut FILE;
pub unsafe extern "C" fn csv_init(
    mut conf: *mut state_conf,
    mut fields___8: *mut *const libc::c_char,
    mut fieldlens: libc::c_int,
) -> libc::c_int {
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if conf.is_null() {
        __assert_fail(
            b"conf\0" as *const u8 as *const libc::c_char,
            b"src/output_modules/module_csv.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_uint,
            b"csv_init\0" as *const u8 as *const libc::c_char,
        );
    }
    if !((*conf).output_filename).is_null() {
        tmp___2 = strcmp(
            (*conf).output_filename as *const libc::c_char,
            b"-\0" as *const u8 as *const libc::c_char,
        );
        if tmp___2 != 0 {
            file = fopen(
                (*conf).output_filename as *const libc::c_char,
                b"w\0" as *const u8 as *const libc::c_char,
            );
            if file.is_null() {
                tmp___0 = __errno_location();
                tmp___1 = strerror(*tmp___0);
                log_fatal(
                    b"csv\0" as *const u8 as *const libc::c_char,
                    b"could not open CSV output file (%s): %s\0" as *const u8
                        as *const libc::c_char,
                    (*conf).output_filename,
                    tmp___1,
                );
            }
        } else {
            file = stdout;
        }
    } else {
        file = stdout;
        log_debug(
            b"csv\0" as *const u8 as *const libc::c_char,
            b"no output file selected, will use stdout\0" as *const u8
                as *const libc::c_char,
        );
    }
    if (*conf).no_header_row == 0 {
        log_debug(
            b"csv\0" as *const u8 as *const libc::c_char,
            b"more than one field, will add headers\0" as *const u8
                as *const libc::c_char,
        );
        i = 0 as libc::c_int;
        while i < fieldlens {
            if i != 0 {
                __fprintf_chk(
                    file,
                    1 as libc::c_int,
                    b",\0" as *const u8 as *const libc::c_char,
                );
            }
            __fprintf_chk(
                file,
                1 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                *fields___8.offset(i as isize),
            );
            i += 1;
        }
        __fprintf_chk(
            file,
            1 as libc::c_int,
            b"\n\0" as *const u8 as *const libc::c_char,
        );
    }
    check_and_log_file_error(file, b"csv\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn csv_close(
    mut c: *mut state_conf,
    mut s: *mut state_send,
    mut r: *mut state_recv,
) -> libc::c_int {
    if !file.is_null() {
        fflush(file);
        fclose(file);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn hex_encode___0(
    mut f: *mut FILE,
    mut readbuf: *mut libc::c_uchar,
    mut len: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < len {
        __fprintf_chk(
            f,
            1 as libc::c_int,
            b"%02x\0" as *const u8 as *const libc::c_char,
            *readbuf.offset(i as isize) as libc::c_int,
        );
        i = i.wrapping_add(1);
    }
    check_and_log_file_error(f, b"csv\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn csv_process(mut fs: *mut fieldset_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut f: *mut field_t = 0 as *mut field_t;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    if file.is_null() {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < (*fs).len {
        f = &mut *((*fs).fields).as_mut_ptr().offset(i as isize) as *mut field_t;
        if i != 0 {
            __fprintf_chk(
                file,
                1 as libc::c_int,
                b",\0" as *const u8 as *const libc::c_char,
            );
        }
        if (*f).type_0 == 1 as libc::c_int {
            tmp = strchr(
                (*f).value.ptr as *mut libc::c_char as *const libc::c_char,
                ',' as i32,
            );
            if !tmp.is_null() {
                __fprintf_chk(
                    file,
                    1 as libc::c_int,
                    b"\"%s\"\0" as *const u8 as *const libc::c_char,
                    (*f).value.ptr as *mut libc::c_char,
                );
            } else {
                __fprintf_chk(
                    file,
                    1 as libc::c_int,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    (*f).value.ptr as *mut libc::c_char,
                );
            }
        } else if (*f).type_0 == 2 as libc::c_int {
            __fprintf_chk(
                file,
                1 as libc::c_int,
                b"%lu\0" as *const u8 as *const libc::c_char,
                (*f).value.num,
            );
        } else if (*f).type_0 == 7 as libc::c_int {
            __fprintf_chk(
                file,
                1 as libc::c_int,
                b"%i\0" as *const u8 as *const libc::c_char,
                (*f).value.num as libc::c_int,
            );
        } else if (*f).type_0 == 3 as libc::c_int {
            hex_encode___0(file, (*f).value.ptr as *mut libc::c_uchar, (*f).len);
        } else if !((*f).type_0 == 4 as libc::c_int) {
            log_fatal(
                b"csv\0" as *const u8 as *const libc::c_char,
                b"received unknown output type\0" as *const u8 as *const libc::c_char,
            );
        }
        i += 1;
    }
    __fprintf_chk(file, 1 as libc::c_int, b"\n\0" as *const u8 as *const libc::c_char);
    fflush(file);
    check_and_log_file_error(file, b"csv\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub static mut module_csv_file: output_module_t = unsafe {
    {
        let mut init = output_module {
            name: b"csv\0" as *const u8 as *const libc::c_char,
            supports_dynamic_output: 0 as libc::c_int,
            update_interval: 0 as libc::c_uint,
            init: Some(
                csv_init
                    as unsafe extern "C" fn(
                        *mut state_conf,
                        *mut *const libc::c_char,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            start: ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<
                    unsafe extern "C" fn(
                        *mut state_conf,
                        *mut state_send,
                        *mut state_recv,
                    ) -> libc::c_int,
                >,
            >(0 as *const libc::c_void as *mut libc::c_void),
            update: ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<
                    unsafe extern "C" fn(
                        *mut state_conf,
                        *mut state_send,
                        *mut state_recv,
                    ) -> libc::c_int,
                >,
            >(0 as *const libc::c_void as *mut libc::c_void),
            close: Some(
                csv_close
                    as unsafe extern "C" fn(
                        *mut state_conf,
                        *mut state_send,
                        *mut state_recv,
                    ) -> libc::c_int,
            ),
            process_ip: Some(
                csv_process as unsafe extern "C" fn(*mut fieldset_t) -> libc::c_int,
            ),
            helptext: b"Outputs one or more output fields as a comma-delimited file. By default, the probe module does not filter out duplicates or limit to successful fields, but rather includes all received packets. Fields can be controlled by setting --output-fields. Filtering out failures and duplicate packets can be achieved by setting an --output-filter.\0"
                as *const u8 as *const libc::c_char,
        };
        init
    }
};
static mut file___0: *mut FILE = 0 as *const libc::c_void as *mut libc::c_void
    as *mut FILE;
pub unsafe extern "C" fn json_output_file_init(
    mut conf: *mut state_conf,
    mut fields___8: *mut *const libc::c_char,
    mut fieldlens: libc::c_int,
) -> libc::c_int {
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: libc::c_int = 0;
    if conf.is_null() {
        __assert_fail(
            b"conf\0" as *const u8 as *const libc::c_char,
            b"src/output_modules/module_json.c\0" as *const u8 as *const libc::c_char,
            39 as libc::c_uint,
            b"json_output_file_init\0" as *const u8 as *const libc::c_char,
        );
    }
    if ((*conf).output_filename).is_null() {
        file___0 = stdout;
    } else {
        tmp___2 = strcmp(
            (*conf).output_filename as *const libc::c_char,
            b"-\0" as *const u8 as *const libc::c_char,
        );
        if tmp___2 != 0 {
            file___0 = fopen(
                (*conf).output_filename as *const libc::c_char,
                b"w\0" as *const u8 as *const libc::c_char,
            );
            if file___0.is_null() {
                tmp___0 = __errno_location();
                tmp___1 = strerror(*tmp___0);
                log_fatal(
                    b"output-json\0" as *const u8 as *const libc::c_char,
                    b"could not open JSON output file (%s): %s\0" as *const u8
                        as *const libc::c_char,
                    (*conf).output_filename,
                    tmp___1,
                );
            }
        } else {
            file___0 = stdout;
        }
    }
    check_and_log_file_error(file___0, b"json\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn field_to_jsonobj(mut f: *mut field_t) -> *mut json_object {
    let mut tmp: *mut json_object = 0 as *mut json_object;
    let mut tmp___0: *mut json_object = 0 as *mut json_object;
    let mut tmp___1: *mut json_object = 0 as *mut json_object;
    let mut encoded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut json_object = 0 as *mut json_object;
    let mut tmp___3: *mut json_object = 0 as *mut json_object;
    let mut tmp___4: *mut json_object = 0 as *mut json_object;
    let mut tmp___5: *mut json_object = 0 as *mut json_object;
    if (*f).type_0 == 1 as libc::c_int {
        tmp = json_object_new_string(
            (*f).value.ptr as *mut libc::c_char as *const libc::c_char,
        );
        return tmp;
    } else if (*f).type_0 == 2 as libc::c_int {
        tmp___0 = json_object_new_int64((*f).value.num as int64_t);
        return tmp___0;
    } else if (*f).type_0 == 7 as libc::c_int {
        tmp___1 = json_object_new_boolean((*f).value.num as json_bool);
        return tmp___1;
    } else if (*f).type_0 == 3 as libc::c_int {
        tmp___2 = 0 as _;
        encoded = tmp___2;
        tmp___3 = json_object_new_string(encoded as *const libc::c_char);
        t = tmp___3;
        free(encoded as *mut libc::c_void);
        return t;
    } else if (*f).type_0 == 4 as libc::c_int {
        return 0 as *mut libc::c_void as *mut json_object
    } else if (*f).type_0 == 5 as libc::c_int {
        tmp___4 = fs_to_jsonobj((*f).value.ptr as *mut fieldset_t);
        return tmp___4;
    } else if (*f).type_0 == 6 as libc::c_int {
        tmp___5 = repeated_to_jsonobj((*f).value.ptr as *mut fieldset_t);
        return tmp___5;
    } else {
        log_fatal(
            b"json\0" as *const u8 as *const libc::c_char,
            b"received unknown output type: %i\0" as *const u8 as *const libc::c_char,
            (*f).type_0,
        );
    };
}
pub unsafe extern "C" fn repeated_to_jsonobj(
    mut fs: *mut fieldset_t,
) -> *mut json_object {
    let mut obj: *mut json_object = 0 as *mut json_object;
    let mut tmp: *mut json_object = 0 as *mut json_object;
    let mut i: libc::c_int = 0;
    let mut f: *mut field_t = 0 as *mut field_t;
    let mut tmp___0: *mut json_object = 0 as *mut json_object;
    tmp = json_object_new_array();
    obj = tmp;
    i = 0 as libc::c_int;
    while i < (*fs).len {
        f = &mut *((*fs).fields).as_mut_ptr().offset(i as isize) as *mut field_t;
        tmp___0 = field_to_jsonobj(f);
        json_object_array_add(obj, tmp___0);
        i += 1;
    }
    return obj;
}
pub unsafe extern "C" fn fs_to_jsonobj(mut fs: *mut fieldset_t) -> *mut json_object {
    let mut obj: *mut json_object = 0 as *mut json_object;
    let mut tmp: *mut json_object = 0 as *mut json_object;
    let mut i: libc::c_int = 0;
    let mut f: *mut field_t = 0 as *mut field_t;
    let mut tmp___0: *mut json_object = 0 as *mut json_object;
    tmp = json_object_new_object();
    obj = tmp;
    i = 0 as libc::c_int;
    while i < (*fs).len {
        f = &mut *((*fs).fields).as_mut_ptr().offset(i as isize) as *mut field_t;
        if (*f).type_0 != 4 as libc::c_int {
            tmp___0 = field_to_jsonobj(f);
            json_object_object_add(obj, (*f).name, tmp___0);
        }
        i += 1;
    }
    return obj;
}
pub unsafe extern "C" fn json_output_to_file(mut fs: *mut fieldset_t) -> libc::c_int {
    let mut record: *mut json_object = 0 as *mut json_object;
    let mut tmp: *mut json_object = 0 as *mut json_object;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    if file___0.is_null() {
        return 0 as libc::c_int;
    }
    tmp = fs_to_jsonobj(fs);
    record = tmp;
    tmp___0 = json_object_to_json_string_ext(record, 0 as libc::c_int);
    __fprintf_chk(
        file___0,
        1 as libc::c_int,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        tmp___0,
    );
    fflush(file___0);
    check_and_log_file_error(file___0, b"json\0" as *const u8 as *const libc::c_char);
    json_object_put(record);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn json_output_file_close(
    mut c: *mut state_conf,
    mut s: *mut state_send,
    mut r: *mut state_recv,
) -> libc::c_int {
    if !file___0.is_null() {
        fflush(file___0);
        fclose(file___0);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn print_json_fieldset(mut fs: *mut fieldset_t) -> libc::c_int {
    let mut record: *mut json_object = 0 as *mut json_object;
    let mut tmp: *mut json_object = 0 as *mut json_object;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    tmp = fs_to_jsonobj(fs);
    record = tmp;
    tmp___0 = json_object_to_json_string(record);
    __fprintf_chk(
        stdout,
        1 as libc::c_int,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        tmp___0,
    );
    json_object_put(record);
    return 0 as libc::c_int;
}
pub static mut module_json_file: output_module_t = unsafe {
    {
        let mut init = output_module {
            name: b"json\0" as *const u8 as *const libc::c_char,
            supports_dynamic_output: 1 as libc::c_int,
            update_interval: 0 as libc::c_uint,
            init: Some(
                json_output_file_init
                    as unsafe extern "C" fn(
                        *mut state_conf,
                        *mut *const libc::c_char,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            start: ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<
                    unsafe extern "C" fn(
                        *mut state_conf,
                        *mut state_send,
                        *mut state_recv,
                    ) -> libc::c_int,
                >,
            >(0 as *const libc::c_void as *mut libc::c_void),
            update: ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<
                    unsafe extern "C" fn(
                        *mut state_conf,
                        *mut state_send,
                        *mut state_recv,
                    ) -> libc::c_int,
                >,
            >(0 as *const libc::c_void as *mut libc::c_void),
            close: Some(
                json_output_file_close
                    as unsafe extern "C" fn(
                        *mut state_conf,
                        *mut state_send,
                        *mut state_recv,
                    ) -> libc::c_int,
            ),
            process_ip: Some(
                json_output_to_file
                    as unsafe extern "C" fn(*mut fieldset_t) -> libc::c_int,
            ),
            helptext: b"Outputs one or more output fileds as a json valid file. By default, the \nprobe module does not filter out duplicates or limit to successful fields, \nbut rather includes all received packets. Fields can be controlled by \nsetting --output-fields. Filtering out failures and duplicate pakcets can \nbe achieved by setting an --output-filter.\0"
                as *const u8 as *const libc::c_char,
        };
        init
    }
};
pub static mut output_modules: [*mut output_module_t; 2] = unsafe {
    [
        &module_csv_file as *const output_module_t as *mut output_module_t,
        &module_json_file as *const output_module_t as *mut output_module_t,
    ]
};
pub unsafe extern "C" fn get_output_module_by_name(
    mut name: *const libc::c_char,
) -> *mut output_module_t {
    let mut num_modules: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    num_modules = (::std::mem::size_of::<[*mut output_module_t; 2]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*mut output_module_t>() as libc::c_ulong)
        as libc::c_int;
    i = 0 as libc::c_int;
    while i < num_modules {
        tmp = strcmp((*output_modules[i as usize]).name, name);
        if tmp == 0 {
            return output_modules[i as usize];
        }
        i += 1;
    }
    return 0 as *mut libc::c_void as *mut output_module_t;
}
pub unsafe extern "C" fn print_output_modules() {
    let mut num_modules: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    num_modules = (::std::mem::size_of::<[*mut output_module_t; 2]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*mut output_module_t>() as libc::c_ulong)
        as libc::c_int;
    i = 0 as libc::c_int;
    while i < num_modules {
        __printf_chk(
            1 as libc::c_int,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            (*output_modules[i as usize]).name,
        );
        i += 1;
    }
}
pub unsafe extern "C" fn get_socket(mut id: uint32_t) -> sock_t {
    let mut sock: libc::c_int = 0;
    let mut tmp: __uint16_t = 0;
    let mut tmp___0: __uint16_t = 0;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: sock_t = sock_t { sock: 0 };
    if zconf.send_ip_pkts != 0 {
        tmp = __bswap_16(3 as libc::c_int as __uint16_t);
        sock = socket(17 as libc::c_int, 2 as libc::c_int, tmp as libc::c_int);
    } else {
        tmp___0 = __bswap_16(3 as libc::c_int as __uint16_t);
        sock = socket(17 as libc::c_int, 3 as libc::c_int, tmp___0 as libc::c_int);
    }
    if sock <= 0 as libc::c_int {
        tmp___1 = __errno_location();
        tmp___2 = strerror(*tmp___1);
        log_fatal(
            b"send\0" as *const u8 as *const libc::c_char,
            b"couldn't create socket. Are you root? Error: %s\n\0" as *const u8
                as *const libc::c_char,
            tmp___2,
        );
    }
    s.sock = sock;
    return s;
}
static mut pc: *mut pcap_t = 0 as *const libc::c_void as *mut libc::c_void
    as *mut pcap_t;
pub unsafe extern "C" fn packet_cb(
    mut user: *mut u_char,
    mut p: *const pcap_pkthdr,
    mut bytes: *const u_char,
) {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut buflen: uint32_t = 0;
    if p.is_null() {
        return;
    }
    if zrecv.filter_success >= zconf.max_results as uint64_t {
        return;
    }
    buflen = (*p).caplen;
    ts.tv_sec = (*p).ts.tv_sec;
    ts.tv_nsec = (*p).ts.tv_usec * 1000 as libc::c_long;
    handle_packet(buflen, bytes, ts);
}
pub unsafe extern "C" fn recv_packets() {
    let mut ret: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tmp = pcap_dispatch(
        pc,
        -(1 as libc::c_int),
        Some(
            packet_cb
                as unsafe extern "C" fn(
                    *mut u_char,
                    *const pcap_pkthdr,
                    *const u_char,
                ) -> (),
        ),
        0 as *mut libc::c_void as *mut u_char,
    );
    ret = tmp;
    if ret == -(1 as libc::c_int) {
        log_fatal(
            b"recv\0" as *const u8 as *const libc::c_char,
            b"pcap_dispatch error\0" as *const u8 as *const libc::c_char,
        );
    } else {
        if ret == 0 as libc::c_int {
            usleep(1000 as libc::c_int as __useconds_t);
        }
    };
}
pub unsafe extern "C" fn recv_cleanup() {
    pcap_close(pc);
    pc = 0 as *mut libc::c_void as *mut pcap_t;
}
pub unsafe extern "C" fn recv_update_stats() -> libc::c_int {
    let mut pcst: pcap_stat = pcap_stat {
        ps_recv: 0,
        ps_drop: 0,
        ps_ifdrop: 0,
    };
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    if pc.is_null() {
        return 1 as libc::c_int;
    }
    tmp___0 = pcap_stats(pc, &mut pcst);
    if tmp___0 != 0 {
        tmp = pcap_geterr(pc);
        log_error(
            b"recv\0" as *const u8 as *const libc::c_char,
            b"unable to retrieve pcap statistics: %s\0" as *const u8
                as *const libc::c_char,
            tmp,
        );
        return 1 as libc::c_int;
    } else {
        zrecv.pcap_recv = pcst.ps_recv;
        zrecv.pcap_drop = pcst.ps_drop;
        zrecv.pcap_ifdrop = pcst.ps_ifdrop;
    }
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
}
unsafe extern "C" fn run_static_initializers() {
    module_tcp_synscan = {
        let mut init = probe_module {
            name: b"tcp_synscan\0" as *const u8 as *const libc::c_char,
            max_packet_length: 58 as libc::c_int as size_t,
            pcap_filter: b"(tcp && tcp[13] & 4 != 0 || tcp[13] == 18) || icmp\0"
                as *const u8 as *const libc::c_char,
            pcap_snaplen: 96 as libc::c_int as size_t,
            port_args: 1 as libc::c_int as uint8_t,
            global_initialize: Some(
                synscan_global_initialize
                    as unsafe extern "C" fn(*mut state_conf) -> libc::c_int,
            ),
            thread_initialize: Some(
                synscan_init_perthread
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut macaddr_t,
                        *mut macaddr_t,
                        port_h_t,
                        *mut *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            make_packet: Some(
                synscan_make_packet
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut size_t,
                        ipaddr_n_t,
                        ipaddr_n_t,
                        uint8_t,
                        *mut uint32_t,
                        libc::c_int,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            print_packet: Some(
                synscan_print_packet
                    as unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> (),
            ),
            validate_packet: Some(
                synscan_validate_packet
                    as unsafe extern "C" fn(
                        *const ip,
                        uint32_t,
                        *mut uint32_t,
                        *mut uint32_t,
                    ) -> libc::c_int,
            ),
            process_packet: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const u_char,
                        uint32_t,
                        *mut fieldset_t,
                        *mut uint32_t,
                        timespec,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *const u_char,
                        uint32_t,
                        *mut fieldset_t,
                        *mut uint32_t,
                        timespec,
                    ) -> (),
                >,
            >(
                Some(
                    synscan_process_packet
                        as unsafe extern "C" fn(
                            *const u_char,
                            uint32_t,
                            *mut fieldset_t,
                            *mut uint32_t,
                            timespec,
                        ) -> (),
                ),
            ),
            close: ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<
                    unsafe extern "C" fn(
                        *mut state_conf,
                        *mut state_send,
                        *mut state_recv,
                    ) -> libc::c_int,
                >,
            >(0 as *mut libc::c_void),
            output_type: 1 as libc::c_int,
            fields: fields___1.as_mut_ptr(),
            numfields: (::std::mem::size_of::<[fielddef_t; 11]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<fielddef_t>() as libc::c_ulong)
                as libc::c_int,
            helptext: b"Probe module that sends a TCP SYN packet to a specific port. Possible classifications are: synack and rst. A SYN-ACK packet is considered a success and a reset packet is considered a failed response.\0"
                as *const u8 as *const libc::c_char,
        };
        init
    };
    module_tcp_synackscan = {
        let mut init = probe_module {
            name: b"tcp_synackscan\0" as *const u8 as *const libc::c_char,
            max_packet_length: 58 as libc::c_int as size_t,
            pcap_filter: b"(tcp && tcp[13] & 4 != 0 || tcp[13] == 18) || icmp\0"
                as *const u8 as *const libc::c_char,
            pcap_snaplen: 96 as libc::c_int as size_t,
            port_args: 1 as libc::c_int as uint8_t,
            global_initialize: Some(
                synackscan_global_initialize
                    as unsafe extern "C" fn(*mut state_conf) -> libc::c_int,
            ),
            thread_initialize: Some(
                synackscan_init_perthread
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut macaddr_t,
                        *mut macaddr_t,
                        port_h_t,
                        *mut *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            make_packet: Some(
                synackscan_make_packet
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut size_t,
                        ipaddr_n_t,
                        ipaddr_n_t,
                        uint8_t,
                        *mut uint32_t,
                        libc::c_int,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            print_packet: Some(
                synscan_print_packet
                    as unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> (),
            ),
            validate_packet: Some(
                synackscan_validate_packet
                    as unsafe extern "C" fn(
                        *const ip,
                        uint32_t,
                        *mut uint32_t,
                        *mut uint32_t,
                    ) -> libc::c_int,
            ),
            process_packet: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const u_char,
                        uint32_t,
                        *mut fieldset_t,
                        *mut uint32_t,
                        timespec,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *const u_char,
                        uint32_t,
                        *mut fieldset_t,
                        *mut uint32_t,
                        timespec,
                    ) -> (),
                >,
            >(
                Some(
                    synackscan_process_packet
                        as unsafe extern "C" fn(
                            *const u_char,
                            uint32_t,
                            *mut fieldset_t,
                            *mut uint32_t,
                            timespec,
                        ) -> (),
                ),
            ),
            close: ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<
                    unsafe extern "C" fn(
                        *mut state_conf,
                        *mut state_send,
                        *mut state_recv,
                    ) -> libc::c_int,
                >,
            >(0 as *mut libc::c_void),
            output_type: 1 as libc::c_int,
            fields: fields___2.as_mut_ptr(),
            numfields: (::std::mem::size_of::<[fielddef_t; 11]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<fielddef_t>() as libc::c_ulong)
                as libc::c_int,
            helptext: b"Probe module that sends a TCP SYNACK packet to a specific port. Possible classifications are: synack and rst. A SYN-ACK packet is considered a failure and a reset packet is considered a success.\0"
                as *const u8 as *const libc::c_char,
        };
        init
    };
    fcount = (::std::mem::size_of::<[udp_payload_field_type_def_t; 12]>()
        as libc::c_ulong)
        .wrapping_div(
            ::std::mem::size_of::<udp_payload_field_type_def_t>() as libc::c_ulong,
        );
    module_udp = {
        let mut init = probe_module {
            name: b"udp\0" as *const u8 as *const libc::c_char,
            max_packet_length: 0 as libc::c_int as size_t,
            pcap_filter: b"udp || icmp\0" as *const u8 as *const libc::c_char,
            pcap_snaplen: 1500 as libc::c_int as size_t,
            port_args: 1 as libc::c_int as uint8_t,
            global_initialize: Some(
                udp_global_initialize
                    as unsafe extern "C" fn(*mut state_conf) -> libc::c_int,
            ),
            thread_initialize: Some(
                udp_init_perthread
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut macaddr_t,
                        *mut macaddr_t,
                        port_h_t,
                        *mut *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            make_packet: Some(
                udp_make_packet
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut size_t,
                        ipaddr_n_t,
                        ipaddr_n_t,
                        uint8_t,
                        *mut uint32_t,
                        libc::c_int,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            print_packet: Some(
                udp_print_packet
                    as unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> (),
            ),
            validate_packet: Some(
                udp_validate_packet
                    as unsafe extern "C" fn(
                        *const ip,
                        uint32_t,
                        *mut uint32_t,
                        *mut uint32_t,
                    ) -> libc::c_int,
            ),
            process_packet: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const u_char,
                        uint32_t,
                        *mut fieldset_t,
                        *mut uint32_t,
                        timespec,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *const u_char,
                        uint32_t,
                        *mut fieldset_t,
                        *mut uint32_t,
                        timespec,
                    ) -> (),
                >,
            >(
                Some(
                    udp_process_packet
                        as unsafe extern "C" fn(
                            *const u_char,
                            uint32_t,
                            *mut fieldset_t,
                            *mut uint32_t,
                            timespec,
                        ) -> (),
                ),
            ),
            close: Some(
                udp_global_cleanup
                    as unsafe extern "C" fn(
                        *mut state_conf,
                        *mut state_send,
                        *mut state_recv,
                    ) -> libc::c_int,
            ),
            output_type: 0 as libc::c_int,
            fields: fields___3.as_mut_ptr(),
            numfields: (::std::mem::size_of::<[fielddef_t; 10]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<fielddef_t>() as libc::c_ulong)
                as libc::c_int,
            helptext: b"Probe module that sends UDP packets to hosts. Packets can optionally be templated based on destination host. Specify packet file with --probe-args=file:/path_to_packet_file and templates with template:/path_to_template_file.\0"
                as *const u8 as *const libc::c_char,
        };
        init
    };
    module_ntp = {
        let mut init = probe_module {
            name: b"ntp\0" as *const u8 as *const libc::c_char,
            max_packet_length: 0 as libc::c_int as size_t,
            pcap_filter: b"udp || icmp\0" as *const u8 as *const libc::c_char,
            pcap_snaplen: 1500 as libc::c_int as size_t,
            port_args: 1 as libc::c_int as uint8_t,
            global_initialize: Some(
                ntp_global_initialize
                    as unsafe extern "C" fn(*mut state_conf) -> libc::c_int,
            ),
            thread_initialize: Some(
                ntp_init_perthread
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut macaddr_t,
                        *mut macaddr_t,
                        port_h_t,
                        *mut *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            make_packet: Some(
                udp_make_packet
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut size_t,
                        ipaddr_n_t,
                        ipaddr_n_t,
                        uint8_t,
                        *mut uint32_t,
                        libc::c_int,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            print_packet: Some(
                ntp_print_packet
                    as unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> (),
            ),
            validate_packet: Some(
                ntp_validate_packet
                    as unsafe extern "C" fn(
                        *const ip,
                        uint32_t,
                        *mut uint32_t,
                        *mut uint32_t,
                    ) -> libc::c_int,
            ),
            process_packet: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const u_char,
                        uint32_t,
                        *mut fieldset_t,
                        *mut uint32_t,
                        timespec,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *const u_char,
                        uint32_t,
                        *mut fieldset_t,
                        *mut uint32_t,
                        timespec,
                    ) -> (),
                >,
            >(
                Some(
                    ntp_process_packet
                        as unsafe extern "C" fn(
                            *const u_char,
                            uint32_t,
                            *mut fieldset_t,
                            *mut uint32_t,
                            timespec,
                        ) -> (),
                ),
            ),
            close: Some(
                udp_global_cleanup
                    as unsafe extern "C" fn(
                        *mut state_conf,
                        *mut state_send,
                        *mut state_recv,
                    ) -> libc::c_int,
            ),
            output_type: 1 as libc::c_int,
            fields: fields___4.as_mut_ptr(),
            numfields: (::std::mem::size_of::<[fielddef_t; 19]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<fielddef_t>() as libc::c_ulong)
                as libc::c_int,
            helptext: 0 as *const libc::c_char,
        };
        init
    };
    module_dns = {
        let mut init = probe_module {
            name: b"dns\0" as *const u8 as *const libc::c_char,
            max_packet_length: 0 as libc::c_int as size_t,
            pcap_filter: b"udp || icmp\0" as *const u8 as *const libc::c_char,
            pcap_snaplen: 1500 as libc::c_int as size_t,
            port_args: 1 as libc::c_int as uint8_t,
            global_initialize: Some(
                dns_global_initialize
                    as unsafe extern "C" fn(*mut state_conf) -> libc::c_int,
            ),
            thread_initialize: Some(
                dns_init_perthread
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut macaddr_t,
                        *mut macaddr_t,
                        port_h_t,
                        *mut *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            make_packet: Some(
                dns_make_packet
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut size_t,
                        ipaddr_n_t,
                        ipaddr_n_t,
                        uint8_t,
                        *mut uint32_t,
                        libc::c_int,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            print_packet: Some(
                dns_print_packet
                    as unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> (),
            ),
            validate_packet: Some(
                dns_validate_packet
                    as unsafe extern "C" fn(
                        *const ip,
                        uint32_t,
                        *mut uint32_t,
                        *mut uint32_t,
                    ) -> libc::c_int,
            ),
            process_packet: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const u_char,
                        uint32_t,
                        *mut fieldset_t,
                        *mut uint32_t,
                        timespec,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *const u_char,
                        uint32_t,
                        *mut fieldset_t,
                        *mut uint32_t,
                        timespec,
                    ) -> (),
                >,
            >(
                Some(
                    dns_process_packet
                        as unsafe extern "C" fn(
                            *const u_char,
                            uint32_t,
                            *mut fieldset_t,
                            *mut uint32_t,
                            timespec,
                        ) -> (),
                ),
            ),
            close: Some(
                dns_global_cleanup
                    as unsafe extern "C" fn(
                        *mut state_conf,
                        *mut state_send,
                        *mut state_recv,
                    ) -> libc::c_int,
            ),
            output_type: 2 as libc::c_int,
            fields: fields___6.as_mut_ptr(),
            numfields: (::std::mem::size_of::<[fielddef_t; 32]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<fielddef_t>() as libc::c_ulong)
                as libc::c_int,
            helptext: b"This module sends out DNS queries and parses basic responses. By default, the module will perform an A record lookup for google.com. You can specify other queries using the --probe-args argument in the form: 'type,query', e.g. 'A,google.com'. The module supports sending the the following types: of queries: A, NS, CNAME, SOA, PTR, MX, TXT, AAAA, RRSIG, and ALL. The module will accept and attempt to parse all DNS responses. There is currently support for parsing out full data from A, NS, CNAME, MX, TXT, and AAAA. Any other types will be output in raw form.\0"
                as *const u8 as *const libc::c_char,
        };
        init
    };
    module_bacnet = {
        let mut init = probe_module {
            name: b"bacnet\0" as *const u8 as *const libc::c_char,
            max_packet_length: (::std::mem::size_of::<ether_header>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<ip>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong)
                .wrapping_add(17 as libc::c_ulong),
            pcap_filter: b"udp || icmp\0" as *const u8 as *const libc::c_char,
            pcap_snaplen: 1500 as libc::c_int as size_t,
            port_args: 1 as libc::c_int as uint8_t,
            global_initialize: Some(
                bacnet_global_initialize
                    as unsafe extern "C" fn(*mut state_conf) -> libc::c_int,
            ),
            thread_initialize: Some(
                bacnet_init_perthread
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut macaddr_t,
                        *mut macaddr_t,
                        port_h_t,
                        *mut *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            make_packet: Some(
                bacnet_make_packet
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut size_t,
                        ipaddr_n_t,
                        ipaddr_n_t,
                        uint8_t,
                        *mut uint32_t,
                        libc::c_int,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            print_packet: Some(
                udp_print_packet
                    as unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> (),
            ),
            validate_packet: Some(
                bacnet_validate_packet
                    as unsafe extern "C" fn(
                        *const ip,
                        uint32_t,
                        *mut uint32_t,
                        *mut uint32_t,
                    ) -> libc::c_int,
            ),
            process_packet: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const u_char,
                        uint32_t,
                        *mut fieldset_t,
                        *mut uint32_t,
                        timespec,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *const u_char,
                        uint32_t,
                        *mut fieldset_t,
                        *mut uint32_t,
                        timespec,
                    ) -> (),
                >,
            >(
                Some(
                    bacnet_process_packet
                        as unsafe extern "C" fn(
                            *const u_char,
                            uint32_t,
                            *mut fieldset_t,
                            *mut uint32_t,
                            timespec,
                        ) -> (),
                ),
            ),
            close: Some(
                udp_global_cleanup
                    as unsafe extern "C" fn(
                        *mut state_conf,
                        *mut state_send,
                        *mut state_recv,
                    ) -> libc::c_int,
            ),
            output_type: 1 as libc::c_int,
            fields: fields___7.as_mut_ptr(),
            numfields: (::std::mem::size_of::<[fielddef_t; 9]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<fielddef_t>() as libc::c_ulong)
                as libc::c_int,
            helptext: 0 as *const libc::c_char,
        };
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
