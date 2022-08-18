use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type pg_conn;
    pub type pg_result;
    pub type pg_cancel;
    static mut stderr: *mut FILE;
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
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn pg_snprintf(
        str: *mut libc::c_char,
        count: size_t,
        fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn pg_sprintf(
        str: *mut libc::c_char,
        fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn pg_fprintf(stream: *mut FILE, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn pg_printf(fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn __read_chk(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __nbytes: size_t,
        __buflen: size_t,
    ) -> ssize_t;
    fn __read_alias(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __nbytes: size_t,
    ) -> ssize_t;
    fn __read_chk_warn(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __nbytes: size_t,
        __buflen: size_t,
    ) -> ssize_t;
    fn PQstatus(conn: *const PGconn) -> ConnStatusType;
    fn PQparameterStatus(
        conn: *const PGconn,
        paramName: *const libc::c_char,
    ) -> *const libc::c_char;
    fn PQserverVersion(conn: *const PGconn) -> libc::c_int;
    fn PQerrorMessage(conn: *const PGconn) -> *mut libc::c_char;
    fn PQsocket(conn: *const PGconn) -> libc::c_int;
    fn PQsendQuery(conn: *mut PGconn, query: *const libc::c_char) -> libc::c_int;
    fn PQgetResult(conn: *mut PGconn) -> *mut PGresult;
    fn PQisBusy(conn: *mut PGconn) -> libc::c_int;
    fn PQconsumeInput(conn: *mut PGconn) -> libc::c_int;
    fn PQsetnonblocking(conn: *mut PGconn, arg: libc::c_int) -> libc::c_int;
    fn PQresultStatus(res: *const PGresult) -> ExecStatusType;
    fn PQresultErrorField(
        res: *const PGresult,
        fieldcode: libc::c_int,
    ) -> *mut libc::c_char;
    fn PQntuples(res: *const PGresult) -> libc::c_int;
    fn PQgetvalue(
        res: *const PGresult,
        tup_num: libc::c_int,
        field_num: libc::c_int,
    ) -> *mut libc::c_char;
    fn PQgetisnull(
        res: *const PGresult,
        tup_num: libc::c_int,
        field_num: libc::c_int,
    ) -> libc::c_int;
    fn PQclear(res: *mut PGresult);
    fn initPQExpBuffer(str: PQExpBuffer);
    fn termPQExpBuffer(str: PQExpBuffer);
    fn resetPQExpBuffer(str: PQExpBuffer);
    fn printfPQExpBuffer(str: PQExpBuffer, fmt: *const libc::c_char, _: ...);
    fn appendPQExpBuffer(str: PQExpBuffer, fmt: *const libc::c_char, _: ...);
    fn appendPQExpBufferStr(str: PQExpBuffer, data: *const libc::c_char);
    fn appendPQExpBufferChar(str: PQExpBuffer, ch: libc::c_char);
    fn time(__timer: *mut time_t) -> time_t;
    fn __poll_alias(
        __fds: *mut pollfd,
        __nfds: nfds_t,
        __timeout: libc::c_int,
    ) -> libc::c_int;
    fn __poll_chk(
        __fds: *mut pollfd,
        __nfds: nfds_t,
        __timeout: libc::c_int,
        __fdslen: libc::c_ulong,
    ) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn __fdelt_chk(__d: libc::c_long) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn abort() -> !;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn putenv(__string: *mut libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn get_progname(argv0: *const libc::c_char) -> *const libc::c_char;
    fn get_parent_directory(path: *mut libc::c_char);
    fn set_pglocale_pgservice(argv0: *const libc::c_char, app: *const libc::c_char);
    fn pg_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn pg_strncasecmp(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
    fn pg_vsnprintf(
        str: *mut libc::c_char,
        count: size_t,
        fmt: *const libc::c_char,
        args: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn pg_strerror(errnum: libc::c_int) -> *mut libc::c_char;
    fn simple_prompt(
        prompt: *const libc::c_char,
        destination: *mut libc::c_char,
        destlen: size_t,
        echo: bool,
    );
    fn strlcpy(dst: *mut libc::c_char, src: *const libc::c_char, siz: size_t) -> size_t;
    fn pqsignal(
        signo: libc::c_int,
        func: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> pqsigfunc;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn mktime(__tp: *mut tm) -> time_t;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn PQconnectdb(conninfo: *const libc::c_char) -> *mut PGconn;
    fn PQfinish(conn: *mut PGconn);
    fn PQgetCancel(conn: *mut PGconn) -> *mut PGcancel;
    fn PQfreeCancel(cancel: *mut PGcancel);
    fn PQcancel(
        cancel: *mut PGcancel,
        errbuf: *mut libc::c_char,
        errbufsize: libc::c_int,
    ) -> libc::c_int;
    fn PQtransactionStatus(conn: *const PGconn) -> PGTransactionStatusType;
    fn PQconnectionNeedsPassword(conn: *const PGconn) -> libc::c_int;
    fn PQexec(conn: *mut PGconn, query: *const libc::c_char) -> *mut PGresult;
    fn PQexecParams(
        conn: *mut PGconn,
        command_0: *const libc::c_char,
        nParams: libc::c_int,
        paramTypes: *const Oid,
        paramValues: *const *const libc::c_char,
        paramLengths: *const libc::c_int,
        paramFormats: *const libc::c_int,
        resultFormat: libc::c_int,
    ) -> *mut PGresult;
    fn PQsendQueryParams(
        conn: *mut PGconn,
        command_0: *const libc::c_char,
        nParams: libc::c_int,
        paramTypes: *const Oid,
        paramValues: *const *const libc::c_char,
        paramLengths: *const libc::c_int,
        paramFormats: *const libc::c_int,
        resultFormat: libc::c_int,
    ) -> libc::c_int;
    fn enlargePQExpBuffer(str: PQExpBuffer, needed: size_t) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn geteuid() -> __uid_t;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn PQpass(conn: *const PGconn) -> *mut libc::c_char;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
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
pub type Oid = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type __gnuc_va_list = __builtin_va_list;
pub type __int32_t = libc::c_int;
pub type __gid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __anonenum_ConnStatusType_301395160 = libc::c_uint;
pub const CONNECTION_CHECK_STANDBY: __anonenum_ConnStatusType_301395160 = 13;
pub const CONNECTION_CHECK_TARGET: __anonenum_ConnStatusType_301395160 = 12;
pub const CONNECTION_GSS_STARTUP: __anonenum_ConnStatusType_301395160 = 11;
pub const CONNECTION_CONSUME: __anonenum_ConnStatusType_301395160 = 10;
pub const CONNECTION_CHECK_WRITABLE: __anonenum_ConnStatusType_301395160 = 9;
pub const CONNECTION_NEEDED: __anonenum_ConnStatusType_301395160 = 8;
pub const CONNECTION_SSL_STARTUP: __anonenum_ConnStatusType_301395160 = 7;
pub const CONNECTION_SETENV: __anonenum_ConnStatusType_301395160 = 6;
pub const CONNECTION_AUTH_OK: __anonenum_ConnStatusType_301395160 = 5;
pub const CONNECTION_AWAITING_RESPONSE: __anonenum_ConnStatusType_301395160 = 4;
pub const CONNECTION_MADE: __anonenum_ConnStatusType_301395160 = 3;
pub const CONNECTION_STARTED: __anonenum_ConnStatusType_301395160 = 2;
pub const CONNECTION_BAD: __anonenum_ConnStatusType_301395160 = 1;
pub const CONNECTION_OK: __anonenum_ConnStatusType_301395160 = 0;
pub type ConnStatusType = __anonenum_ConnStatusType_301395160;
pub type __anonenum_ExecStatusType_61969095 = libc::c_uint;
pub const PGRES_PIPELINE_ABORTED: __anonenum_ExecStatusType_61969095 = 11;
pub const PGRES_PIPELINE_SYNC: __anonenum_ExecStatusType_61969095 = 10;
pub const PGRES_SINGLE_TUPLE: __anonenum_ExecStatusType_61969095 = 9;
pub const PGRES_COPY_BOTH: __anonenum_ExecStatusType_61969095 = 8;
pub const PGRES_FATAL_ERROR: __anonenum_ExecStatusType_61969095 = 7;
pub const PGRES_NONFATAL_ERROR: __anonenum_ExecStatusType_61969095 = 6;
pub const PGRES_BAD_RESPONSE: __anonenum_ExecStatusType_61969095 = 5;
pub const PGRES_COPY_IN: __anonenum_ExecStatusType_61969095 = 4;
pub const PGRES_COPY_OUT: __anonenum_ExecStatusType_61969095 = 3;
pub const PGRES_TUPLES_OK: __anonenum_ExecStatusType_61969095 = 2;
pub const PGRES_COMMAND_OK: __anonenum_ExecStatusType_61969095 = 1;
pub const PGRES_EMPTY_QUERY: __anonenum_ExecStatusType_61969095 = 0;
pub type ExecStatusType = __anonenum_ExecStatusType_61969095;
pub type PGconn = pg_conn;
pub type PGresult = pg_result;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PQExpBufferData {
    pub data: *mut libc::c_char,
    pub len: size_t,
    pub maxlen: size_t,
}
pub type PQExpBuffer = *mut PQExpBufferData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SimpleStringListCell {
    pub next: *mut SimpleStringListCell,
    pub val: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SimpleStringList {
    pub head: *mut SimpleStringListCell,
    pub tail: *mut SimpleStringListCell,
}
pub type pgut_optsrc = libc::c_uint;
pub const SOURCE_CONST: pgut_optsrc = 4;
pub const SOURCE_CMDLINE: pgut_optsrc = 3;
pub const SOURCE_FILE: pgut_optsrc = 2;
pub const SOURCE_ENV: pgut_optsrc = 1;
pub const SOURCE_DEFAULT: pgut_optsrc = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pgut_option {
    pub type_0: libc::c_char,
    pub sname: libc::c_char,
    pub lname: *const libc::c_char,
    pub var: *mut libc::c_void,
    pub allowed: pgut_optsrc,
    pub source: pgut_optsrc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct worker_conns {
    pub max_num_workers: libc::c_int,
    pub num_workers: libc::c_int,
    pub conns: *mut *mut PGconn,
}
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub type __anonenum_index_status_t_747877378 = libc::c_uint;
pub const FINISHED: __anonenum_index_status_t_747877378 = 2;
pub const INPROGRESS: __anonenum_index_status_t_747877378 = 1;
pub const UNPROCESSED: __anonenum_index_status_t_747877378 = 0;
pub type index_status_t = __anonenum_index_status_t_747877378;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct repack_index {
    pub target_oid: Oid,
    pub create_index: *const libc::c_char,
    pub status: index_status_t,
    pub worker_idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct repack_table {
    pub target_name: *const libc::c_char,
    pub target_oid: Oid,
    pub target_toast: Oid,
    pub target_tidx: Oid,
    pub pkid: Oid,
    pub ckid: Oid,
    pub create_pktype: *const libc::c_char,
    pub create_log: *const libc::c_char,
    pub create_trigger: *const libc::c_char,
    pub enable_trigger: *const libc::c_char,
    pub create_table: *const libc::c_char,
    pub copy_data: *const libc::c_char,
    pub alter_col_storage: *const libc::c_char,
    pub drop_columns: *const libc::c_char,
    pub delete_log: *const libc::c_char,
    pub lock_table: *const libc::c_char,
    pub sql_peek: *const libc::c_char,
    pub sql_insert: *const libc::c_char,
    pub sql_delete: *const libc::c_char,
    pub sql_update: *const libc::c_char,
    pub sql_pop: *const libc::c_char,
    pub n_indexes: libc::c_int,
    pub indexes: *mut repack_index,
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type va_list___0 = __gnuc_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
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
pub union __anonunion_pthread_mutex_t_335460617 {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type pthread_mutex_t = __anonunion_pthread_mutex_t_335460617;
pub type int32 = libc::c_int;
pub type uint32 = libc::c_uint;
pub type int64 = libc::c_long;
pub type uint64 = libc::c_ulong;
pub type pqsigfunc = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
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
pub type __anonenum_PGTransactionStatusType_947083977 = libc::c_uint;
pub const PQTRANS_UNKNOWN: __anonenum_PGTransactionStatusType_947083977 = 4;
pub const PQTRANS_INERROR: __anonenum_PGTransactionStatusType_947083977 = 3;
pub const PQTRANS_INTRANS: __anonenum_PGTransactionStatusType_947083977 = 2;
pub const PQTRANS_ACTIVE: __anonenum_PGTransactionStatusType_947083977 = 1;
pub const PQTRANS_IDLE: __anonenum_PGTransactionStatusType_947083977 = 0;
pub type PGTransactionStatusType = __anonenum_PGTransactionStatusType_947083977;
pub type PGcancel = pg_cancel;
pub type YesNo = libc::c_uint;
pub const YES: YesNo = 2;
pub const NO: YesNo = 1;
pub const DEFAULT: YesNo = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pgutConn {
    pub conn: *mut PGconn,
    pub cancel: *mut PGcancel,
    pub next: *mut pgutConn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pgutErrorData {
    pub elevel: libc::c_int,
    pub save_errno: libc::c_int,
    pub code: libc::c_int,
    pub msg: PQExpBufferData,
    pub detail: PQExpBufferData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pgut_atexit_item {
    pub callback: Option::<unsafe extern "C" fn(bool, *mut libc::c_void) -> ()>,
    pub userdata: *mut libc::c_void,
    pub next: *mut pgut_atexit_item,
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
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub static mut PROGRAM_URL: *const libc::c_char = b"https://reorg.github.io/pg_repack/\0"
    as *const u8 as *const libc::c_char;
pub static mut PROGRAM_ISSUES: *const libc::c_char = b"https://github.com/reorg/pg_repack/issues\0"
    as *const u8 as *const libc::c_char;
pub static mut PROGRAM_VERSION: *const libc::c_char = b"1.4.7\0" as *const u8
    as *const libc::c_char;
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
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    let mut tmp: *mut *const __int32_t = 0 as *mut *const __int32_t;
    let mut tmp___0: __int32_t = 0;
    if __c >= -(128 as libc::c_int) {
        if __c < 256 as libc::c_int {
            tmp = __ctype_tolower_loc();
            tmp___0 = *(*tmp).offset(__c as isize);
        } else {
            tmp___0 = __c;
        }
    } else {
        tmp___0 = __c;
    }
    return tmp___0;
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    let mut tmp: *mut *const __int32_t = 0 as *mut *const __int32_t;
    let mut tmp___0: __int32_t = 0;
    if __c >= -(128 as libc::c_int) {
        if __c < 256 as libc::c_int {
            tmp = __ctype_toupper_loc();
            tmp___0 = *(*tmp).offset(__c as isize);
        } else {
            tmp___0 = __c;
        }
    } else {
        tmp___0 = __c;
    }
    return tmp___0;
}
#[inline(always)]
unsafe extern "C" fn read(
    mut __fd: libc::c_int,
    mut __buf: *mut libc::c_void,
    mut __nbytes: size_t,
) -> ssize_t {
    let mut tmp: libc::c_ulong = 0;
    let mut tmp___0: ssize_t = 0;
    let mut tmp___1: libc::c_ulong = 0;
    let mut tmp___2: ssize_t = 0;
    let mut tmp___3: libc::c_ulong = 0;
    let mut tmp___4: libc::c_ulong = 0;
    let mut tmp___5: ssize_t = 0;
    tmp___4 = (if 0 as libc::c_int & 2 == 0 { -1isize } else { 0isize }) as _;
    if tmp___4 != 0xffffffffffffffff as libc::c_ulong {
        tmp = (if 0 as libc::c_int & 2 == 0 { -1isize } else { 0isize }) as _;
        tmp___0 = __read_chk(__fd, __buf, __nbytes, tmp);
        return tmp___0;
    }
    tmp___5 = __read_alias(__fd, __buf, __nbytes);
    return tmp___5;
}
#[inline(always)]
unsafe extern "C" fn poll(
    mut __fds: *mut pollfd,
    mut __nfds: nfds_t,
    mut __timeout: libc::c_int,
) -> libc::c_int {
    let mut tmp: libc::c_ulong = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___4: libc::c_ulong = 0;
    let mut tmp___5: libc::c_int = 0;
    tmp___4 = (if 1 as libc::c_int & 2 == 0 { -1isize } else { 0isize }) as _;
    if tmp___4 as libc::c_ulonglong != 18446744073709551615 as libc::c_ulonglong {
        tmp = (if 1 as libc::c_int & 2 == 0 { -1isize } else { 0isize }) as _;
        tmp___0 = __poll_chk(__fds, __nfds, __timeout, tmp);
        return tmp___0;
    }
    tmp___5 = __poll_alias(__fds, __nfds, __timeout);
    return tmp___5;
}
unsafe extern "C" fn sqlstate_equals(
    mut res: *mut PGresult,
    mut state: *const libc::c_char,
) -> bool {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    tmp = PQresultErrorField(res as *const PGresult, 'C' as i32);
    tmp___0 = strcmp(tmp as *const libc::c_char, state);
    return tmp___0 == 0 as libc::c_int;
}
static mut analyze: bool = 1 as libc::c_int != 0;
static mut alldb: bool = 0 as libc::c_int != 0;
static mut noorder: bool = 0 as libc::c_int != 0;
static mut parent_table_list: SimpleStringList = {
    let mut init = SimpleStringList {
        head: 0 as *const libc::c_void as *mut libc::c_void as *mut SimpleStringListCell,
        tail: 0 as *const libc::c_void as *mut libc::c_void as *mut SimpleStringListCell,
    };
    init
};
static mut table_list: SimpleStringList = {
    let mut init = SimpleStringList {
        head: 0 as *const libc::c_void as *mut libc::c_void as *mut SimpleStringListCell,
        tail: 0 as *const libc::c_void as *mut libc::c_void as *mut SimpleStringListCell,
    };
    init
};
static mut schema_list: SimpleStringList = {
    let mut init = SimpleStringList {
        head: 0 as *const libc::c_void as *mut libc::c_void as *mut SimpleStringListCell,
        tail: 0 as *const libc::c_void as *mut libc::c_void as *mut SimpleStringListCell,
    };
    init
};
static mut orderby: *mut libc::c_char = 0 as *const libc::c_void as *mut libc::c_void
    as *mut libc::c_char;
static mut tablespace: *mut libc::c_char = 0 as *const libc::c_void as *mut libc::c_void
    as *mut libc::c_char;
static mut moveidx: bool = 0 as libc::c_int != 0;
static mut r_index: SimpleStringList = {
    let mut init = SimpleStringList {
        head: 0 as *const libc::c_void as *mut libc::c_void as *mut SimpleStringListCell,
        tail: 0 as *const libc::c_void as *mut libc::c_void as *mut SimpleStringListCell,
    };
    init
};
static mut only_indexes: bool = 0 as libc::c_int != 0;
static mut wait_timeout: libc::c_int = 60 as libc::c_int;
static mut jobs: libc::c_int = 0 as libc::c_int;
static mut dryrun: bool = 0 as libc::c_int != 0;
static mut temp_obj_num: libc::c_uint = 0 as libc::c_uint;
static mut no_kill_backend: bool = 0 as libc::c_int != 0;
static mut no_superuser_check: bool = 0 as libc::c_int != 0;
static mut exclude_extension_list: SimpleStringList = {
    let mut init = SimpleStringList {
        head: 0 as *const libc::c_void as *mut libc::c_void as *mut SimpleStringListCell,
        tail: 0 as *const libc::c_void as *mut libc::c_void as *mut SimpleStringListCell,
    };
    init
};
unsafe extern "C" fn utoa(
    mut value: libc::c_uint,
    mut buffer: *mut libc::c_char,
) -> *mut libc::c_char {
    pg_sprintf(buffer, b"%u\0" as *const u8 as *const libc::c_char, value);
    return buffer;
}
static mut options: [pgut_option; 18] = unsafe {
    [
        {
            let mut init = pgut_option {
                type_0: 'b' as i32 as libc::c_char,
                sname: 'a' as i32 as libc::c_char,
                lname: b"all\0" as *const u8 as *const libc::c_char,
                var: &alldb as *const bool as *mut bool as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'l' as i32 as libc::c_char,
                sname: 't' as i32 as libc::c_char,
                lname: b"table\0" as *const u8 as *const libc::c_char,
                var: &table_list as *const SimpleStringList as *mut SimpleStringList
                    as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'l' as i32 as libc::c_char,
                sname: 'I' as i32 as libc::c_char,
                lname: b"parent-table\0" as *const u8 as *const libc::c_char,
                var: &parent_table_list as *const SimpleStringList
                    as *mut SimpleStringList as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'l' as i32 as libc::c_char,
                sname: 'c' as i32 as libc::c_char,
                lname: b"schema\0" as *const u8 as *const libc::c_char,
                var: &schema_list as *const SimpleStringList as *mut SimpleStringList
                    as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'b' as i32 as libc::c_char,
                sname: 'n' as i32 as libc::c_char,
                lname: b"no-order\0" as *const u8 as *const libc::c_char,
                var: &noorder as *const bool as *mut bool as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'b' as i32 as libc::c_char,
                sname: 'N' as i32 as libc::c_char,
                lname: b"dry-run\0" as *const u8 as *const libc::c_char,
                var: &dryrun as *const bool as *mut bool as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 's' as i32 as libc::c_char,
                sname: 'o' as i32 as libc::c_char,
                lname: b"order-by\0" as *const u8 as *const libc::c_char,
                var: &orderby as *const *mut libc::c_char as *mut *mut libc::c_char
                    as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 's' as i32 as libc::c_char,
                sname: 's' as i32 as libc::c_char,
                lname: b"tablespace\0" as *const u8 as *const libc::c_char,
                var: &tablespace as *const *mut libc::c_char as *mut *mut libc::c_char
                    as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'b' as i32 as libc::c_char,
                sname: 'S' as i32 as libc::c_char,
                lname: b"moveidx\0" as *const u8 as *const libc::c_char,
                var: &moveidx as *const bool as *mut bool as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'l' as i32 as libc::c_char,
                sname: 'i' as i32 as libc::c_char,
                lname: b"index\0" as *const u8 as *const libc::c_char,
                var: &r_index as *const SimpleStringList as *mut SimpleStringList
                    as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'b' as i32 as libc::c_char,
                sname: 'x' as i32 as libc::c_char,
                lname: b"only-indexes\0" as *const u8 as *const libc::c_char,
                var: &only_indexes as *const bool as *mut bool as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'i' as i32 as libc::c_char,
                sname: 'T' as i32 as libc::c_char,
                lname: b"wait-timeout\0" as *const u8 as *const libc::c_char,
                var: &wait_timeout as *const libc::c_int as *mut libc::c_int
                    as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'B' as i32 as libc::c_char,
                sname: 'Z' as i32 as libc::c_char,
                lname: b"no-analyze\0" as *const u8 as *const libc::c_char,
                var: &analyze as *const bool as *mut bool as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'i' as i32 as libc::c_char,
                sname: 'j' as i32 as libc::c_char,
                lname: b"jobs\0" as *const u8 as *const libc::c_char,
                var: &jobs as *const libc::c_int as *mut libc::c_int
                    as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'b' as i32 as libc::c_char,
                sname: 'D' as i32 as libc::c_char,
                lname: b"no-kill-backend\0" as *const u8 as *const libc::c_char,
                var: &no_kill_backend as *const bool as *mut bool as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'b' as i32 as libc::c_char,
                sname: 'k' as i32 as libc::c_char,
                lname: b"no-superuser-check\0" as *const u8 as *const libc::c_char,
                var: &no_superuser_check as *const bool as *mut bool
                    as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'l' as i32 as libc::c_char,
                sname: 'C' as i32 as libc::c_char,
                lname: b"exclude-extension\0" as *const u8 as *const libc::c_char,
                var: &exclude_extension_list as *const SimpleStringList
                    as *mut SimpleStringList as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 0 as libc::c_int as libc::c_char,
                sname: 0 as libc::c_int as libc::c_char,
                lname: 0 as *const libc::c_char,
                var: 0 as *const libc::c_void as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
    ]
};
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut errbuf: [libc::c_char; 256] = [0; 256];
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: bool = false;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: bool = false;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: bool = false;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: bool = false;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: libc::c_int = 0;
    let mut tmp___13: bool = false;
    let mut tmp___14: libc::c_int = 0;
    let mut tmp___15: libc::c_int = 0;
    let mut tmp___16: bool = false;
    let mut tmp___17: libc::c_int = 0;
    let mut tmp___18: libc::c_int = 0;
    let mut tmp___19: bool = false;
    let mut tmp___20: libc::c_int = 0;
    let mut tmp___21: libc::c_int = 0;
    let mut tmp___22: bool = false;
    let mut tmp___23: libc::c_int = 0;
    let mut tmp___24: libc::c_int = 0;
    let mut tmp___25: bool = false;
    let mut tmp___26: libc::c_int = 0;
    let mut tmp___27: libc::c_int = 0;
    let mut tmp___28: bool = false;
    let mut tmp___29: libc::c_int = 0;
    let mut tmp___30: libc::c_int = 0;
    let mut tmp___31: bool = false;
    let mut tmp___32: libc::c_int = 0;
    let mut tmp___33: libc::c_int = 0;
    let mut tmp___34: bool = false;
    let mut tmp___35: libc::c_int = 0;
    let mut tmp___36: libc::c_int = 0;
    let mut tmp___37: bool = false;
    let mut tmp___38: bool = false;
    let mut tmp___39: libc::c_int = 0;
    let mut tmp___40: libc::c_int = 0;
    let mut tmp___41: bool = false;
    let mut tmp___42: libc::c_int = 0;
    let mut tmp___43: libc::c_int = 0;
    let mut tmp___44: bool = false;
    let mut tmp___45: libc::c_int = 0;
    let mut tmp___46: libc::c_int = 0;
    let mut tmp___47: bool = false;
    let mut tmp___48: libc::c_int = 0;
    let mut tmp___49: libc::c_int = 0;
    let mut tmp___50: bool = false;
    let mut tmp___51: libc::c_int = 0;
    let mut tmp___52: libc::c_int = 0;
    let mut tmp___53: bool = false;
    let mut tmp___54: libc::c_int = 0;
    let mut tmp___55: libc::c_int = 0;
    let mut tmp___56: bool = false;
    let mut tmp___57: bool = false;
    i = pgut_getopt(argc, argv, options.as_mut_ptr());
    if i == argc - 1 as libc::c_int {
        dbname = *argv.offset(i as isize);
    } else if i < argc {
        tmp___1 = pgut_errstart(20 as libc::c_int);
        if tmp___1 {
            tmp = errmsg(b"too many arguments\0" as *const u8 as *const libc::c_char);
            tmp___0 = errcode(22 as libc::c_int);
            pgut_errfinish(tmp___0, tmp);
        }
    }
    check_tablespace();
    if dryrun {
        elog(
            17 as libc::c_int,
            b"Dry run enabled, not executing repack\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(r_index.head).is_null() {
        current_block = 6366187563082267978;
    } else if only_indexes {
        current_block = 6366187563082267978;
    } else {
        if !(schema_list.head).is_null() {
            let mut current_block_117: u64;
            if !(table_list.head).is_null() {
                current_block_117 = 9883361772205930751;
            } else if !(parent_table_list.head).is_null() {
                current_block_117 = 9883361772205930751;
            } else {
                current_block_117 = 16937825661756021828;
            }
            match current_block_117 {
                9883361772205930751 => {
                    tmp___41 = pgut_errstart(20 as libc::c_int);
                    if tmp___41 {
                        tmp___39 = errmsg(
                            b"cannot repack specific table(s) in schema, use schema.table notation instead\0"
                                as *const u8 as *const libc::c_char,
                        );
                        tmp___40 = errcode(22 as libc::c_int);
                        pgut_errfinish(tmp___40, tmp___39);
                    }
                }
                _ => {}
            }
        }
        if !(exclude_extension_list.head).is_null() {
            if !(table_list.head).is_null() {
                tmp___44 = pgut_errstart(20 as libc::c_int);
                if tmp___44 {
                    tmp___42 = errmsg(
                        b"cannot specify --table (-t) and --exclude-extension (-C)\0"
                            as *const u8 as *const libc::c_char,
                    );
                    tmp___43 = errcode(22 as libc::c_int);
                    pgut_errfinish(tmp___43, tmp___42);
                }
            }
        }
        if !(exclude_extension_list.head).is_null() {
            if !(parent_table_list.head).is_null() {
                tmp___47 = pgut_errstart(20 as libc::c_int);
                if tmp___47 {
                    tmp___45 = errmsg(
                        b"cannot specify --parent-table (-I) and --exclude-extension (-C)\0"
                            as *const u8 as *const libc::c_char,
                    );
                    tmp___46 = errcode(22 as libc::c_int);
                    pgut_errfinish(tmp___46, tmp___45);
                }
            }
        }
        if noorder {
            orderby = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        if alldb {
            let mut current_block_149: u64;
            if !(table_list.head).is_null() {
                current_block_149 = 16869621181989210477;
            } else if !(parent_table_list.head).is_null() {
                current_block_149 = 16869621181989210477;
            } else {
                current_block_149 = 9216188846964669005;
            }
            match current_block_149 {
                16869621181989210477 => {
                    tmp___50 = pgut_errstart(20 as libc::c_int);
                    if tmp___50 {
                        tmp___48 = errmsg(
                            b"cannot repack specific table(s) in all databases\0"
                                as *const u8 as *const libc::c_char,
                        );
                        tmp___49 = errcode(22 as libc::c_int);
                        pgut_errfinish(tmp___49, tmp___48);
                    }
                }
                _ => {}
            }
            if !(schema_list.head).is_null() {
                tmp___53 = pgut_errstart(20 as libc::c_int);
                if tmp___53 {
                    tmp___51 = errmsg(
                        b"cannot repack specific schema(s) in all databases\0"
                            as *const u8 as *const libc::c_char,
                    );
                    tmp___52 = errcode(22 as libc::c_int);
                    pgut_errfinish(tmp___52, tmp___51);
                }
            }
            repack_all_databases(orderby as *const libc::c_char);
        } else {
            tmp___57 = repack_one_database(
                orderby as *const libc::c_char,
                errbuf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            );
            if !tmp___57 {
                tmp___56 = pgut_errstart(20 as libc::c_int);
                if tmp___56 {
                    tmp___54 = errmsg(
                        b"%s failed with error: %s\0" as *const u8
                            as *const libc::c_char,
                        PROGRAM_NAME,
                        errbuf.as_mut_ptr(),
                    );
                    tmp___55 = errcode(20 as libc::c_int);
                    pgut_errfinish(tmp___55, tmp___54);
                }
            }
        }
        current_block = 16512738885216853798;
    }
    match current_block {
        6366187563082267978 => {
            if !(r_index.head).is_null() {
                if !(table_list.head).is_null() {
                    tmp___4 = pgut_errstart(20 as libc::c_int);
                    if tmp___4 {
                        tmp___2 = errmsg(
                            b"cannot specify --index (-i) and --table (-t)\0"
                                as *const u8 as *const libc::c_char,
                        );
                        tmp___3 = errcode(22 as libc::c_int);
                        pgut_errfinish(tmp___3, tmp___2);
                    }
                }
            }
            if !(r_index.head).is_null() {
                if !(parent_table_list.head).is_null() {
                    tmp___7 = pgut_errstart(20 as libc::c_int);
                    if tmp___7 {
                        tmp___5 = errmsg(
                            b"cannot specify --index (-i) and --parent-table (-I)\0"
                                as *const u8 as *const libc::c_char,
                        );
                        tmp___6 = errcode(22 as libc::c_int);
                        pgut_errfinish(tmp___6, tmp___5);
                    }
                    current_block = 16512738885216853798;
                } else {
                    current_block = 5324276660319579342;
                }
            } else {
                current_block = 5324276660319579342;
            }
            match current_block {
                16512738885216853798 => {}
                _ => {
                    if !(r_index.head).is_null() {
                        if only_indexes {
                            tmp___10 = pgut_errstart(20 as libc::c_int);
                            if tmp___10 {
                                tmp___8 = errmsg(
                                    b"cannot specify --index (-i) and --only-indexes (-x)\0"
                                        as *const u8 as *const libc::c_char,
                                );
                                tmp___9 = errcode(22 as libc::c_int);
                                pgut_errfinish(tmp___9, tmp___8);
                            }
                            current_block = 16512738885216853798;
                        } else {
                            current_block = 17722525762118303510;
                        }
                    } else {
                        current_block = 17722525762118303510;
                    }
                    match current_block {
                        16512738885216853798 => {}
                        _ => {
                            if !(r_index.head).is_null() {
                                if !(exclude_extension_list.head).is_null() {
                                    tmp___13 = pgut_errstart(20 as libc::c_int);
                                    if tmp___13 {
                                        tmp___11 = errmsg(
                                            b"cannot specify --index (-i) and --exclude-extension (-C)\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                        tmp___12 = errcode(22 as libc::c_int);
                                        pgut_errfinish(tmp___12, tmp___11);
                                    }
                                    current_block = 16512738885216853798;
                                } else {
                                    current_block = 2402526077591545746;
                                }
                            } else {
                                current_block = 2402526077591545746;
                            }
                            match current_block {
                                16512738885216853798 => {}
                                _ => {
                                    if only_indexes {
                                        if !(table_list.head).is_null() {
                                            current_block = 3727274548616262782;
                                        } else if !(parent_table_list.head).is_null() {
                                            current_block = 3727274548616262782;
                                        } else {
                                            tmp___16 = pgut_errstart(20 as libc::c_int);
                                            if tmp___16 {
                                                tmp___14 = errmsg(
                                                    b"cannot repack all indexes of database, specify the table(s)via --table (-t) or --parent-table (-I)\0"
                                                        as *const u8 as *const libc::c_char,
                                                );
                                                tmp___15 = errcode(22 as libc::c_int);
                                                pgut_errfinish(tmp___15, tmp___14);
                                            }
                                            current_block = 16512738885216853798;
                                        }
                                    } else {
                                        current_block = 3727274548616262782;
                                    }
                                    match current_block {
                                        16512738885216853798 => {}
                                        _ => {
                                            let mut current_block_110: u64;
                                            if only_indexes {
                                                if !(exclude_extension_list.head).is_null() {
                                                    tmp___19 = pgut_errstart(20 as libc::c_int);
                                                    if tmp___19 {
                                                        tmp___17 = errmsg(
                                                            b"cannot specify --only-indexes (-x) and --exclude-extension (-C)\0"
                                                                as *const u8 as *const libc::c_char,
                                                        );
                                                        tmp___18 = errcode(22 as libc::c_int);
                                                        pgut_errfinish(tmp___18, tmp___17);
                                                    }
                                                    current_block_110 = 228501038991332163;
                                                } else {
                                                    current_block_110 = 6102065738294282542;
                                                }
                                            } else {
                                                current_block_110 = 6102065738294282542;
                                            }
                                            match current_block_110 {
                                                6102065738294282542 => {
                                                    if alldb {
                                                        tmp___22 = pgut_errstart(20 as libc::c_int);
                                                        if tmp___22 {
                                                            tmp___20 = errmsg(
                                                                b"cannot repack specific index(es) in all databases\0"
                                                                    as *const u8 as *const libc::c_char,
                                                            );
                                                            tmp___21 = errcode(22 as libc::c_int);
                                                            pgut_errfinish(tmp___21, tmp___20);
                                                        }
                                                    } else {
                                                        if !orderby.is_null() {
                                                            tmp___25 = pgut_errstart(19 as libc::c_int);
                                                            if tmp___25 {
                                                                tmp___23 = errmsg(
                                                                    b"option -o (--order-by) has no effect while repacking indexes\0"
                                                                        as *const u8 as *const libc::c_char,
                                                                );
                                                                tmp___24 = errcode(22 as libc::c_int);
                                                                pgut_errfinish(tmp___24, tmp___23);
                                                            }
                                                        } else if noorder {
                                                            tmp___28 = pgut_errstart(19 as libc::c_int);
                                                            if tmp___28 {
                                                                tmp___26 = errmsg(
                                                                    b"option -n (--no-order) has no effect while repacking indexes\0"
                                                                        as *const u8 as *const libc::c_char,
                                                                );
                                                                tmp___27 = errcode(22 as libc::c_int);
                                                                pgut_errfinish(tmp___27, tmp___26);
                                                            }
                                                        } else if !analyze {
                                                            tmp___31 = pgut_errstart(19 as libc::c_int);
                                                            if tmp___31 {
                                                                tmp___29 = errmsg(
                                                                    b"ANALYZE is not performed after repacking indexes, -z (--no-analyze) has no effect\0"
                                                                        as *const u8 as *const libc::c_char,
                                                                );
                                                                tmp___30 = errcode(22 as libc::c_int);
                                                                pgut_errfinish(tmp___30, tmp___29);
                                                            }
                                                        } else if jobs != 0 {
                                                            tmp___34 = pgut_errstart(19 as libc::c_int);
                                                            if tmp___34 {
                                                                tmp___32 = errmsg(
                                                                    b"option -j (--jobs) has no effect, repacking indexes does not use parallel jobs\0"
                                                                        as *const u8 as *const libc::c_char,
                                                                );
                                                                tmp___33 = errcode(22 as libc::c_int);
                                                                pgut_errfinish(tmp___33, tmp___32);
                                                            }
                                                        }
                                                        tmp___38 = repack_all_indexes(
                                                            errbuf.as_mut_ptr(),
                                                            ::std::mem::size_of::<[libc::c_char; 256]>()
                                                                as libc::c_ulong,
                                                        );
                                                        if !tmp___38 {
                                                            tmp___37 = pgut_errstart(20 as libc::c_int);
                                                            if tmp___37 {
                                                                tmp___35 = errmsg(
                                                                    b"%s\0" as *const u8 as *const libc::c_char,
                                                                    errbuf.as_mut_ptr(),
                                                                );
                                                                tmp___36 = errcode(20 as libc::c_int);
                                                                pgut_errfinish(tmp___36, tmp___35);
                                                            }
                                                        }
                                                    }
                                                }
                                                _ => {}
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
    return 0 as libc::c_int;
}
unsafe extern "C" fn is_superuser() -> bool {
    let mut val: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: libc::c_int = 0;
    if no_superuser_check {
        return 1 as libc::c_int != 0;
    }
    if connection.is_null() {
        return 0 as libc::c_int != 0;
    }
    val = PQparameterStatus(
        connection as *const PGconn,
        b"is_superuser\0" as *const u8 as *const libc::c_char,
    );
    if !val.is_null() {
        tmp = strcmp(val, b"on\0" as *const u8 as *const libc::c_char);
        if tmp == 0 as libc::c_int {
            return 1 as libc::c_int != 0;
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn check_tablespace() {
    let mut res: *mut PGresult = 0 as *mut PGresult;
    let mut params: [*const libc::c_char; 1] = [0 as *const libc::c_char; 1];
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: bool = false;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: bool = false;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: bool = false;
    let mut tmp___10: ExecStatusType = PGRES_EMPTY_QUERY;
    res = 0 as *mut libc::c_void as *mut PGresult;
    if tablespace as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        if moveidx {
            tmp___1 = pgut_errstart(20 as libc::c_int);
            if tmp___1 {
                tmp = errmsg(
                    b"cannot specify --moveidx (-S) without --tablespace (-s)\0"
                        as *const u8 as *const libc::c_char,
                );
                tmp___0 = errcode(22 as libc::c_int);
                pgut_errfinish(tmp___0, tmp);
            }
        }
        return;
    }
    reconnect(20 as libc::c_int);
    params[0 as libc::c_int as usize] = tablespace as *const libc::c_char;
    res = execute_elevel(
        b"select spcname from pg_tablespace where spcname = $1\0" as *const u8
            as *const libc::c_char,
        1 as libc::c_int,
        params.as_mut_ptr(),
        13 as libc::c_int,
    );
    tmp___10 = PQresultStatus(res as *const PGresult);
    if tmp___10 as libc::c_uint == 2 as libc::c_uint {
        tmp___5 = PQntuples(res as *const PGresult);
        if tmp___5 == 0 as libc::c_int {
            tmp___4 = pgut_errstart(20 as libc::c_int);
            if tmp___4 {
                tmp___2 = errmsg(
                    b"the tablespace \"%s\" doesn't exist\0" as *const u8
                        as *const libc::c_char,
                    tablespace,
                );
                tmp___3 = errcode(22 as libc::c_int);
                pgut_errfinish(tmp___3, tmp___2);
            }
        }
    } else {
        tmp___9 = pgut_errstart(20 as libc::c_int);
        if tmp___9 {
            tmp___6 = PQerrorMessage(connection as *const PGconn);
            tmp___7 = errmsg(
                b"error checking the namespace: %s\0" as *const u8
                    as *const libc::c_char,
                tmp___6,
            );
            tmp___8 = errcode(22 as libc::c_int);
            pgut_errfinish(tmp___8, tmp___7);
        }
    }
    PQclear(res);
    res = 0 as *mut libc::c_void as *mut PGresult;
}
unsafe extern "C" fn preliminary_checks(
    mut errbuf: *mut libc::c_char,
    mut errsize: size_t,
) -> bool {
    let mut ret: bool = false;
    let mut res: *mut PGresult = 0 as *mut PGresult;
    let mut tmp: bool = false;
    let mut libver: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: [libc::c_char; 64] = [0; 64];
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: bool = false;
    let mut tmp___6: bool = false;
    let mut tmp___7: ExecStatusType = PGRES_EMPTY_QUERY;
    ret = 0 as libc::c_int != 0;
    res = 0 as *mut libc::c_void as *mut PGresult;
    tmp = is_superuser();
    if !tmp {
        if !errbuf.is_null() {
            pg_snprintf(
                errbuf,
                errsize,
                b"You must be a superuser to use %s\0" as *const u8
                    as *const libc::c_char,
                PROGRAM_NAME,
            );
        }
    } else {
        res = execute_elevel(
            b"select repack.version(), repack.version_sql()\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
            0 as *mut libc::c_void as *mut *const libc::c_char,
            13 as libc::c_int,
        );
        tmp___7 = PQresultStatus(res as *const PGresult);
        if tmp___7 as libc::c_uint == 2 as libc::c_uint {
            pg_snprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                b"%s %s\0" as *const u8 as *const libc::c_char,
                PROGRAM_NAME,
                PROGRAM_VERSION,
            );
            tmp___0 = getstr(res, 0 as libc::c_int, 0 as libc::c_int);
            libver = tmp___0 as *const libc::c_char;
            tmp___1 = strcmp(buf.as_mut_ptr() as *const libc::c_char, libver);
            if 0 as libc::c_int != tmp___1 {
                if !errbuf.is_null() {
                    pg_snprintf(
                        errbuf,
                        errsize,
                        b"program '%s' does not match database library '%s'\0"
                            as *const u8 as *const libc::c_char,
                        buf.as_mut_ptr(),
                        libver,
                    );
                }
            } else {
                tmp___2 = getstr(res, 0 as libc::c_int, 1 as libc::c_int);
                libver = tmp___2 as *const libc::c_char;
                tmp___3 = strcmp(buf.as_mut_ptr() as *const libc::c_char, libver);
                if 0 as libc::c_int != tmp___3 {
                    if !errbuf.is_null() {
                        pg_snprintf(
                            errbuf,
                            errsize,
                            b"extension '%s' required, found '%s'; please drop and re-create the extension\0"
                                as *const u8 as *const libc::c_char,
                            buf.as_mut_ptr(),
                            libver,
                        );
                    }
                } else {
                    PQclear(res);
                    res = 0 as *mut libc::c_void as *mut PGresult;
                    command(
                        b"SET statement_timeout = 0\0" as *const u8
                            as *const libc::c_char,
                        0 as libc::c_int,
                        0 as *mut libc::c_void as *mut *const libc::c_char,
                    );
                    command(
                        b"SET search_path = pg_catalog, pg_temp, public\0" as *const u8
                            as *const libc::c_char,
                        0 as libc::c_int,
                        0 as *mut libc::c_void as *mut *const libc::c_char,
                    );
                    command(
                        b"SET client_min_messages = warning\0" as *const u8
                            as *const libc::c_char,
                        0 as libc::c_int,
                        0 as *mut libc::c_void as *mut *const libc::c_char,
                    );
                    ret = 1 as libc::c_int != 0;
                }
            }
        } else {
            tmp___5 = sqlstate_equals(
                res,
                b"3F000\0" as *const u8 as *const libc::c_char,
            );
            let mut current_block_29: u64;
            if tmp___5 {
                current_block_29 = 11039786497382000233;
            } else {
                tmp___6 = sqlstate_equals(
                    res,
                    b"42883\0" as *const u8 as *const libc::c_char,
                );
                if tmp___6 {
                    current_block_29 = 11039786497382000233;
                } else {
                    if !errbuf.is_null() {
                        tmp___4 = PQerrorMessage(connection as *const PGconn);
                        pg_snprintf(
                            errbuf,
                            errsize,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            tmp___4,
                        );
                    }
                    current_block_29 = 17184638872671510253;
                }
            }
            match current_block_29 {
                11039786497382000233 => {
                    if !errbuf.is_null() {
                        pg_snprintf(
                            errbuf,
                            errsize,
                            b"%s %s is not installed in the database\0" as *const u8
                                as *const libc::c_char,
                            PROGRAM_NAME,
                            PROGRAM_VERSION,
                        );
                    }
                }
                _ => {}
            }
        }
    }
    PQclear(res);
    res = 0 as *mut libc::c_void as *mut PGresult;
    return ret;
}
unsafe extern "C" fn is_requested_relation_exists(
    mut errbuf: *mut libc::c_char,
    mut errsize: size_t,
) -> bool {
    let mut ret: bool = false;
    let mut res: *mut PGresult = 0 as *mut PGresult;
    let mut params: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut iparam: libc::c_int = 0;
    let mut sql: PQExpBufferData = PQExpBufferData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
    };
    let mut num_relations: libc::c_int = 0;
    let mut cell: *mut SimpleStringListCell = 0 as *mut SimpleStringListCell;
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut rel_names: PQExpBufferData = PQExpBufferData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
    };
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___7: ExecStatusType = PGRES_EMPTY_QUERY;
    ret = 0 as libc::c_int != 0;
    res = 0 as *mut libc::c_void as *mut PGresult;
    params = 0 as *mut libc::c_void as *mut *const libc::c_char;
    iparam = 0 as libc::c_int;
    tmp = simple_string_list_size(parent_table_list);
    tmp___0 = simple_string_list_size(table_list);
    num_relations = tmp.wrapping_add(tmp___0) as libc::c_int;
    if num_relations == 0 as libc::c_int {
        return 1 as libc::c_int != 0;
    }
    tmp___1 = PQserverVersion(connection as *const PGconn);
    if tmp___1 < 90600 as libc::c_int {
        return 1 as libc::c_int != 0;
    }
    tmp___2 = pgut_malloc(
        (num_relations as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    );
    params = tmp___2 as *mut *const libc::c_char;
    initPQExpBuffer(&mut sql);
    appendPQExpBufferStr(
        &mut sql,
        b"SELECT r FROM (VALUES \0" as *const u8 as *const libc::c_char,
    );
    cell = table_list.head;
    while !cell.is_null() {
        appendPQExpBuffer(
            &mut sql as *mut PQExpBufferData,
            b"($%d)\0" as *const u8 as *const libc::c_char,
            iparam + 1 as libc::c_int,
        );
        tmp___3 = iparam;
        iparam += 1;
        let ref mut fresh0 = *params.offset(tmp___3 as isize);
        *fresh0 = ((*cell).val).as_mut_ptr() as *const libc::c_char;
        if iparam < num_relations {
            appendPQExpBufferChar(&mut sql, ',' as i32 as libc::c_char);
        }
        cell = (*cell).next;
    }
    cell = parent_table_list.head;
    while !cell.is_null() {
        appendPQExpBuffer(
            &mut sql as *mut PQExpBufferData,
            b"($%d)\0" as *const u8 as *const libc::c_char,
            iparam + 1 as libc::c_int,
        );
        tmp___4 = iparam;
        iparam += 1;
        let ref mut fresh1 = *params.offset(tmp___4 as isize);
        *fresh1 = ((*cell).val).as_mut_ptr() as *const libc::c_char;
        if iparam < num_relations {
            appendPQExpBufferChar(&mut sql, ',' as i32 as libc::c_char);
        }
        cell = (*cell).next;
    }
    appendPQExpBufferStr(
        &mut sql,
        b") AS given_t(r) WHERE NOT EXISTS(  SELECT FROM repack.tables WHERE relid=to_regclass(given_t.r) )\0"
            as *const u8 as *const libc::c_char,
    );
    if iparam != num_relations {
        if !errbuf.is_null() {
            pg_snprintf(
                errbuf,
                errsize,
                b"internal error: bad parameters count: %i instead of %i\0" as *const u8
                    as *const libc::c_char,
                iparam,
                num_relations,
            );
        }
    } else {
        res = execute_elevel(
            sql.data as *const libc::c_char,
            iparam,
            params,
            13 as libc::c_int,
        );
        tmp___7 = PQresultStatus(res as *const PGresult);
        if tmp___7 as libc::c_uint == 2 as libc::c_uint {
            num = PQntuples(res as *const PGresult);
            if num != 0 as libc::c_int {
                initPQExpBuffer(&mut rel_names);
                i = 0 as libc::c_int;
                while i < num {
                    tmp___5 = getstr(res, i, 0 as libc::c_int);
                    appendPQExpBuffer(
                        &mut rel_names as *mut PQExpBufferData,
                        b"\"%s\"\0" as *const u8 as *const libc::c_char,
                        tmp___5,
                    );
                    if i + 1 as libc::c_int != num {
                        appendPQExpBufferStr(
                            &mut rel_names,
                            b", \0" as *const u8 as *const libc::c_char,
                        );
                    }
                    i += 1;
                }
                if !errbuf.is_null() {
                    if num > 1 as libc::c_int {
                        pg_snprintf(
                            errbuf,
                            errsize,
                            b"relations do not exist: %s\0" as *const u8
                                as *const libc::c_char,
                            rel_names.data,
                        );
                    } else {
                        pg_snprintf(
                            errbuf,
                            errsize,
                            b"ERROR:  relation %s does not exist\0" as *const u8
                                as *const libc::c_char,
                            rel_names.data,
                        );
                    }
                }
                termPQExpBuffer(&mut rel_names);
            } else {
                ret = 1 as libc::c_int != 0;
            }
        } else if !errbuf.is_null() {
            tmp___6 = PQerrorMessage(connection as *const PGconn);
            pg_snprintf(
                errbuf,
                errsize,
                b"%s\0" as *const u8 as *const libc::c_char,
                tmp___6,
            );
        }
        PQclear(res);
        res = 0 as *mut libc::c_void as *mut PGresult;
    }
    PQclear(res);
    res = 0 as *mut libc::c_void as *mut PGresult;
    termPQExpBuffer(&mut sql);
    free(params as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn repack_all_databases(mut orderby___0: *const libc::c_char) {
    let mut result: *mut PGresult = 0 as *mut PGresult;
    let mut i: libc::c_int = 0;
    let mut tmp: bool = false;
    let mut ret: bool = false;
    let mut errbuf: [libc::c_char; 256] = [0; 256];
    let mut tmp___0: libc::c_int = 0;
    dbname = b"postgres\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    reconnect(20 as libc::c_int);
    tmp = is_superuser();
    if !tmp {
        elog(
            20 as libc::c_int,
            b"You must be a superuser to use %s\0" as *const u8 as *const libc::c_char,
            PROGRAM_NAME,
        );
    }
    result = execute(
        b"SELECT datname FROM pg_database WHERE datallowconn ORDER BY 1;\0" as *const u8
            as *const libc::c_char,
        0 as libc::c_int,
        0 as *mut libc::c_void as *mut *const libc::c_char,
    );
    disconnect();
    i = 0 as libc::c_int;
    loop {
        tmp___0 = PQntuples(result as *const PGresult);
        if !(i < tmp___0) {
            break;
        }
        dbname = PQgetvalue(result as *const PGresult, i, 0 as libc::c_int);
        elog(
            17 as libc::c_int,
            b"repacking database \"%s\"\0" as *const u8 as *const libc::c_char,
            dbname,
        );
        if !dryrun {
            ret = repack_one_database(
                orderby___0,
                errbuf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            );
            if !ret {
                elog(
                    17 as libc::c_int,
                    b"database \"%s\" skipped: %s\0" as *const u8 as *const libc::c_char,
                    dbname,
                    errbuf.as_mut_ptr(),
                );
            }
        }
        i += 1;
    }
    PQclear(result);
    result = 0 as *mut libc::c_void as *mut PGresult;
}
unsafe extern "C" fn getstr(
    mut res: *mut PGresult,
    mut row: libc::c_int,
    mut col: libc::c_int,
) -> *mut libc::c_char {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    tmp___0 = PQgetisnull(res as *const PGresult, row, col);
    if tmp___0 != 0 {
        return 0 as *mut libc::c_void as *mut libc::c_char
    } else {
        tmp = PQgetvalue(res as *const PGresult, row, col);
        return tmp;
    };
}
unsafe extern "C" fn getoid(
    mut res: *mut PGresult,
    mut row: libc::c_int,
    mut col: libc::c_int,
) -> Oid {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_ulong = 0;
    let mut tmp___1: libc::c_int = 0;
    tmp___1 = PQgetisnull(res as *const PGresult, row, col);
    if tmp___1 != 0 {
        return 0 as libc::c_int as Oid
    } else {
        tmp = PQgetvalue(res as *const PGresult, row, col);
        tmp___0 = strtoul(
            tmp as *const libc::c_char,
            0 as *mut libc::c_void as *mut *mut libc::c_char,
            10 as libc::c_int,
        );
        return tmp___0 as Oid;
    };
}
unsafe extern "C" fn repack_one_database(
    mut orderby___0: *const libc::c_char,
    mut errbuf: *mut libc::c_char,
    mut errsize: size_t,
) -> bool {
    let mut ret: bool = false;
    let mut res: *mut PGresult = 0 as *mut PGresult;
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut sql: PQExpBufferData = PQExpBufferData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
    };
    let mut cell: *mut SimpleStringListCell = 0 as *mut SimpleStringListCell;
    let mut params: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut iparam: libc::c_int = 0;
    let mut num_parent_tables: size_t = 0;
    let mut num_tables: size_t = 0;
    let mut num_schemas: size_t = 0;
    let mut num_params: size_t = 0;
    let mut num_excluded_extensions: size_t = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: bool = false;
    let mut tmp___1: bool = false;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___8: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___9: ExecStatusType = PGRES_EMPTY_QUERY;
    let mut table: repack_table = repack_table {
        target_name: 0 as *const libc::c_char,
        target_oid: 0,
        target_toast: 0,
        target_tidx: 0,
        pkid: 0,
        ckid: 0,
        create_pktype: 0 as *const libc::c_char,
        create_log: 0 as *const libc::c_char,
        create_trigger: 0 as *const libc::c_char,
        enable_trigger: 0 as *const libc::c_char,
        create_table: 0 as *const libc::c_char,
        copy_data: 0 as *const libc::c_char,
        alter_col_storage: 0 as *const libc::c_char,
        drop_columns: 0 as *const libc::c_char,
        delete_log: 0 as *const libc::c_char,
        lock_table: 0 as *const libc::c_char,
        sql_peek: 0 as *const libc::c_char,
        sql_insert: 0 as *const libc::c_char,
        sql_delete: 0 as *const libc::c_char,
        sql_update: 0 as *const libc::c_char,
        sql_pop: 0 as *const libc::c_char,
        n_indexes: 0,
        indexes: 0 as *mut repack_index,
    };
    let mut copy_sql: PQExpBufferData = PQExpBufferData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
    };
    let mut create_table_1: *const libc::c_char = 0 as *const libc::c_char;
    let mut create_table_2: *const libc::c_char = 0 as *const libc::c_char;
    let mut tablespace___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut ckey: *const libc::c_char = 0 as *const libc::c_char;
    let mut c: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___12: libc::c_int = 0;
    let mut tmp___13: libc::c_int = 0;
    let mut tmp___14: libc::c_int = 0;
    let mut tmp___15: libc::c_int = 0;
    let mut tmp___16: libc::c_int = 0;
    let mut tmp___17: libc::c_int = 0;
    let mut tmp___18: libc::c_int = 0;
    let mut tmp___19: bool = false;
    let mut tmp___20: libc::c_int = 0;
    let mut tmp___21: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___22: libc::c_int = 0;
    let mut tmp___23: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___24: libc::c_int = 0;
    let mut tmp___25: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___26: libc::c_int = 0;
    let mut tmp___27: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___28: libc::c_int = 0;
    let mut tmp___29: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___30: libc::c_int = 0;
    let mut tmp___31: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___32: libc::c_int = 0;
    let mut tmp___33: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___34: libc::c_int = 0;
    let mut tmp___35: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___36: libc::c_int = 0;
    let mut tmp___37: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___38: libc::c_int = 0;
    let mut tmp___39: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___40: libc::c_int = 0;
    let mut tmp___41: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___42: libc::c_int = 0;
    let mut tmp___43: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___44: libc::c_int = 0;
    let mut tmp___45: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___46: libc::c_int = 0;
    let mut tmp___47: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___48: libc::c_int = 0;
    let mut tmp___49: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___50: libc::c_int = 0;
    let mut tmp___51: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___52: libc::c_int = 0;
    let mut tmp___53: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___54: libc::c_int = 0;
    let mut tmp___55: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___56: libc::c_int = 0;
    let mut tmp___57: *mut libc::c_char = 0 as *mut libc::c_char;
    ret = 0 as libc::c_int != 0;
    res = 0 as *mut libc::c_void as *mut PGresult;
    params = 0 as *mut libc::c_void as *mut *const libc::c_char;
    iparam = 0 as libc::c_int;
    num_parent_tables = simple_string_list_size(parent_table_list);
    num_tables = simple_string_list_size(table_list);
    num_schemas = simple_string_list_size(schema_list);
    num_excluded_extensions = simple_string_list_size(exclude_extension_list);
    num_params = num_excluded_extensions
        .wrapping_add(num_parent_tables)
        .wrapping_add(num_tables)
        .wrapping_add(num_schemas)
        .wrapping_add(1 as libc::c_ulong);
    tmp = pgut_malloc(
        num_params
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    );
    params = tmp as *mut *const libc::c_char;
    initPQExpBuffer(&mut sql);
    reconnect(20 as libc::c_int);
    if jobs > 1 as libc::c_int {
        setup_workers(jobs);
    }
    tmp___0 = preliminary_checks(errbuf, errsize);
    if tmp___0 {
        tmp___1 = is_requested_relation_exists(errbuf, errsize);
        if tmp___1 {
            appendPQExpBufferStr(
                &mut sql,
                b"SELECT t.*, coalesce(v.tablespace, t.tablespace_orig) as tablespace_dest FROM repack.tables t,  (VALUES (quote_ident($1::text))) as v (tablespace) WHERE \0"
                    as *const u8 as *const libc::c_char,
            );
            tmp___2 = iparam;
            iparam += 1;
            let ref mut fresh2 = *params.offset(tmp___2 as isize);
            *fresh2 = tablespace as *const libc::c_char;
            let mut current_block_73: u64;
            if num_tables != 0 {
                current_block_73 = 292809133874490552;
            } else if num_parent_tables != 0 {
                current_block_73 = 292809133874490552;
            } else {
                if num_schemas != 0 {
                    appendPQExpBufferStr(
                        &mut sql,
                        b"schemaname IN (\0" as *const u8 as *const libc::c_char,
                    );
                    cell = schema_list.head;
                    while !cell.is_null() {
                        appendPQExpBuffer(
                            &mut sql as *mut PQExpBufferData,
                            b"$%d\0" as *const u8 as *const libc::c_char,
                            iparam + 1 as libc::c_int,
                        );
                        tmp___5 = iparam;
                        iparam += 1;
                        let ref mut fresh5 = *params.offset(tmp___5 as isize);
                        *fresh5 = ((*cell).val).as_mut_ptr() as *const libc::c_char;
                        if !((*cell).next).is_null() {
                            appendPQExpBufferStr(
                                &mut sql,
                                b", \0" as *const u8 as *const libc::c_char,
                            );
                        }
                        cell = (*cell).next;
                    }
                    appendPQExpBufferStr(
                        &mut sql,
                        b")\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    appendPQExpBufferStr(
                        &mut sql,
                        b"pkid IS NOT NULL\0" as *const u8 as *const libc::c_char,
                    );
                }
                current_block_73 = 14541395414537699361;
            }
            match current_block_73 {
                292809133874490552 => {
                    if num_tables != 0 {
                        appendPQExpBufferStr(
                            &mut sql,
                            b"(\0" as *const u8 as *const libc::c_char,
                        );
                        cell = table_list.head;
                        while !cell.is_null() {
                            appendPQExpBuffer(
                                &mut sql as *mut PQExpBufferData,
                                b"relid = $%d::regclass\0" as *const u8
                                    as *const libc::c_char,
                                iparam + 1 as libc::c_int,
                            );
                            tmp___3 = iparam;
                            iparam += 1;
                            let ref mut fresh3 = *params.offset(tmp___3 as isize);
                            *fresh3 = ((*cell).val).as_mut_ptr() as *const libc::c_char;
                            if !((*cell).next).is_null() {
                                appendPQExpBufferStr(
                                    &mut sql,
                                    b" OR \0" as *const u8 as *const libc::c_char,
                                );
                            }
                            cell = (*cell).next;
                        }
                        appendPQExpBufferStr(
                            &mut sql,
                            b")\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    if num_tables != 0 {
                        if num_parent_tables != 0 {
                            appendPQExpBufferStr(
                                &mut sql,
                                b" OR \0" as *const u8 as *const libc::c_char,
                            );
                        }
                    }
                    if num_parent_tables != 0 {
                        appendPQExpBufferStr(
                            &mut sql,
                            b"(\0" as *const u8 as *const libc::c_char,
                        );
                        cell = parent_table_list.head;
                        while !cell.is_null() {
                            appendPQExpBuffer(
                                &mut sql as *mut PQExpBufferData,
                                b"relid = ANY(repack.get_table_and_inheritors($%d::regclass))\0"
                                    as *const u8 as *const libc::c_char,
                                iparam + 1 as libc::c_int,
                            );
                            tmp___4 = iparam;
                            iparam += 1;
                            let ref mut fresh4 = *params.offset(tmp___4 as isize);
                            *fresh4 = ((*cell).val).as_mut_ptr() as *const libc::c_char;
                            if !((*cell).next).is_null() {
                                appendPQExpBufferStr(
                                    &mut sql,
                                    b" OR \0" as *const u8 as *const libc::c_char,
                                );
                            }
                            cell = (*cell).next;
                        }
                        appendPQExpBufferStr(
                            &mut sql,
                            b")\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
                _ => {}
            }
            if !(exclude_extension_list.head).is_null() {
                appendPQExpBufferStr(
                    &mut sql,
                    b" AND t.relid NOT IN  (SELECT d.objid::regclass   FROM pg_depend d JOIN pg_extension e   ON d.refobjid = e.oid   WHERE d.classid = 'pg_class'::regclass AND (\0"
                        as *const u8 as *const libc::c_char,
                );
                cell = exclude_extension_list.head;
                while !cell.is_null() {
                    appendPQExpBuffer(
                        &mut sql as *mut PQExpBufferData,
                        b"e.extname = $%d\0" as *const u8 as *const libc::c_char,
                        iparam + 1 as libc::c_int,
                    );
                    tmp___6 = iparam;
                    iparam += 1;
                    let ref mut fresh6 = *params.offset(tmp___6 as isize);
                    *fresh6 = ((*cell).val).as_mut_ptr() as *const libc::c_char;
                    if !((*cell).next).is_null() {
                        tmp___7 = b" OR \0" as *const u8 as *const libc::c_char;
                    } else {
                        tmp___7 = b")\0" as *const u8 as *const libc::c_char;
                    }
                    appendPQExpBufferStr(&mut sql, tmp___7);
                    cell = (*cell).next;
                }
                appendPQExpBufferStr(
                    &mut sql,
                    b")\0" as *const u8 as *const libc::c_char,
                );
            }
            appendPQExpBufferStr(
                &mut sql,
                b" ORDER BY t.relname, t.schemaname\0" as *const u8
                    as *const libc::c_char,
            );
            if iparam as size_t != num_params {
                if !errbuf.is_null() {
                    pg_snprintf(
                        errbuf,
                        errsize,
                        b"internal error: bad parameters count: %i instead of %zi\0"
                            as *const u8 as *const libc::c_char,
                        iparam,
                        num_params,
                    );
                }
            } else {
                res = execute_elevel(
                    sql.data as *const libc::c_char,
                    num_params as libc::c_int,
                    params,
                    13 as libc::c_int,
                );
                tmp___9 = PQresultStatus(res as *const PGresult);
                if tmp___9 as libc::c_uint != 2 as libc::c_uint {
                    if !errbuf.is_null() {
                        tmp___8 = PQerrorMessage(connection as *const PGconn);
                        pg_snprintf(
                            errbuf,
                            errsize,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            tmp___8,
                        );
                    }
                } else {
                    num = PQntuples(res as *const PGresult);
                    i = 0 as libc::c_int;
                    while i < num {
                        c = 0 as libc::c_int;
                        tmp___10 = c;
                        c += 1;
                        tmp___11 = getstr(res, i, tmp___10);
                        table.target_name = tmp___11 as *const libc::c_char;
                        tmp___12 = c;
                        c += 1;
                        table.target_oid = getoid(res, i, tmp___12);
                        tmp___13 = c;
                        c += 1;
                        table.target_toast = getoid(res, i, tmp___13);
                        tmp___14 = c;
                        c += 1;
                        table.target_tidx = getoid(res, i, tmp___14);
                        c += 1;
                        tmp___15 = c;
                        c += 1;
                        table.pkid = getoid(res, i, tmp___15);
                        tmp___16 = c;
                        c += 1;
                        table.ckid = getoid(res, i, tmp___16);
                        if table.pkid == 0 as libc::c_uint {
                            tmp___19 = pgut_errstart(19 as libc::c_int);
                            if tmp___19 {
                                tmp___17 = errmsg(
                                    b"relation \"%s\" must have a primary key or not-null unique keys\0"
                                        as *const u8 as *const libc::c_char,
                                    table.target_name,
                                );
                                tmp___18 = errcode(-(2 as libc::c_int));
                                pgut_errfinish(tmp___18, tmp___17);
                            }
                        } else {
                            tmp___20 = c;
                            c += 1;
                            tmp___21 = getstr(res, i, tmp___20);
                            table.create_pktype = tmp___21 as *const libc::c_char;
                            tmp___22 = c;
                            c += 1;
                            tmp___23 = getstr(res, i, tmp___22);
                            table.create_log = tmp___23 as *const libc::c_char;
                            tmp___24 = c;
                            c += 1;
                            tmp___25 = getstr(res, i, tmp___24);
                            table.create_trigger = tmp___25 as *const libc::c_char;
                            tmp___26 = c;
                            c += 1;
                            tmp___27 = getstr(res, i, tmp___26);
                            table.enable_trigger = tmp___27 as *const libc::c_char;
                            tmp___28 = c;
                            c += 1;
                            tmp___29 = getstr(res, i, tmp___28);
                            create_table_1 = tmp___29 as *const libc::c_char;
                            tmp___30 = c;
                            c += 1;
                            tmp___31 = getstr(res, i, tmp___30);
                            tablespace___0 = tmp___31 as *const libc::c_char;
                            tmp___32 = c;
                            c += 1;
                            tmp___33 = getstr(res, i, tmp___32);
                            create_table_2 = tmp___33 as *const libc::c_char;
                            tmp___34 = c;
                            c += 1;
                            tmp___35 = getstr(res, i, tmp___34);
                            table.copy_data = tmp___35 as *const libc::c_char;
                            tmp___36 = c;
                            c += 1;
                            tmp___37 = getstr(res, i, tmp___36);
                            table.alter_col_storage = tmp___37 as *const libc::c_char;
                            tmp___38 = c;
                            c += 1;
                            tmp___39 = getstr(res, i, tmp___38);
                            table.drop_columns = tmp___39 as *const libc::c_char;
                            tmp___40 = c;
                            c += 1;
                            tmp___41 = getstr(res, i, tmp___40);
                            table.delete_log = tmp___41 as *const libc::c_char;
                            tmp___42 = c;
                            c += 1;
                            tmp___43 = getstr(res, i, tmp___42);
                            table.lock_table = tmp___43 as *const libc::c_char;
                            tmp___44 = c;
                            c += 1;
                            tmp___45 = getstr(res, i, tmp___44);
                            ckey = tmp___45 as *const libc::c_char;
                            tmp___46 = c;
                            c += 1;
                            tmp___47 = getstr(res, i, tmp___46);
                            table.sql_peek = tmp___47 as *const libc::c_char;
                            tmp___48 = c;
                            c += 1;
                            tmp___49 = getstr(res, i, tmp___48);
                            table.sql_insert = tmp___49 as *const libc::c_char;
                            tmp___50 = c;
                            c += 1;
                            tmp___51 = getstr(res, i, tmp___50);
                            table.sql_delete = tmp___51 as *const libc::c_char;
                            tmp___52 = c;
                            c += 1;
                            tmp___53 = getstr(res, i, tmp___52);
                            table.sql_update = tmp___53 as *const libc::c_char;
                            tmp___54 = c;
                            c += 1;
                            tmp___55 = getstr(res, i, tmp___54);
                            table.sql_pop = tmp___55 as *const libc::c_char;
                            tmp___56 = c;
                            c += 1;
                            tmp___57 = getstr(res, i, tmp___56);
                            tablespace___0 = tmp___57 as *const libc::c_char;
                            resetPQExpBuffer(&mut sql);
                            appendPQExpBufferStr(&mut sql, create_table_1);
                            appendPQExpBufferStr(&mut sql, tablespace___0);
                            appendPQExpBufferStr(&mut sql, create_table_2);
                            appendPQExpBufferStr(
                                &mut sql,
                                b" WITH NO DATA\0" as *const u8 as *const libc::c_char,
                            );
                            table.create_table = sql.data as *const libc::c_char;
                            initPQExpBuffer(&mut copy_sql);
                            appendPQExpBufferStr(&mut copy_sql, table.copy_data);
                            if orderby___0.is_null() {
                                if ckey as libc::c_ulong
                                    != 0 as *mut libc::c_void as libc::c_ulong
                                {
                                    appendPQExpBufferStr(
                                        &mut copy_sql,
                                        b" ORDER BY \0" as *const u8 as *const libc::c_char,
                                    );
                                    appendPQExpBufferStr(&mut copy_sql, ckey);
                                }
                            } else if *orderby___0.offset(0 as libc::c_int as isize) != 0
                                {
                                appendPQExpBufferStr(
                                    &mut copy_sql,
                                    b" ORDER BY \0" as *const u8 as *const libc::c_char,
                                );
                                appendPQExpBufferStr(&mut copy_sql, orderby___0);
                            }
                            table.copy_data = copy_sql.data as *const libc::c_char;
                            repack_one_table(&mut table, orderby___0);
                        }
                        i += 1;
                    }
                    ret = 1 as libc::c_int != 0;
                }
            }
        }
    }
    PQclear(res);
    res = 0 as *mut libc::c_void as *mut PGresult;
    disconnect();
    termPQExpBuffer(&mut sql);
    free(params as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn apply_log(
    mut conn: *mut PGconn,
    mut table: *const repack_table,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut res: *mut PGresult = 0 as *mut PGresult;
    let mut params: [*const libc::c_char; 6] = [0 as *const libc::c_char; 6];
    let mut buffer: [libc::c_char; 12] = [0; 12];
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    params[0 as libc::c_int as usize] = (*table).sql_peek;
    params[1 as libc::c_int as usize] = (*table).sql_insert;
    params[2 as libc::c_int as usize] = (*table).sql_delete;
    params[3 as libc::c_int as usize] = (*table).sql_update;
    params[4 as libc::c_int as usize] = (*table).sql_pop;
    tmp = utoa(count as libc::c_uint, buffer.as_mut_ptr());
    params[5 as libc::c_int as usize] = tmp as *const libc::c_char;
    res = pgut_execute(
        conn,
        b"SELECT repack.repack_apply($1, $2, $3, $4, $5, $6)\0" as *const u8
            as *const libc::c_char,
        6 as libc::c_int,
        params.as_mut_ptr(),
    );
    tmp___0 = PQgetvalue(res as *const PGresult, 0 as libc::c_int, 0 as libc::c_int);
    result = atoi(tmp___0 as *const libc::c_char);
    PQclear(res);
    res = 0 as *mut libc::c_void as *mut PGresult;
    return result;
}
unsafe extern "C" fn rebuild_indexes(mut table: *const repack_table) -> bool {
    let mut current_block: u64;
    let mut res: *mut PGresult = 0 as *mut PGresult;
    let mut num_indexes: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut num_active_workers: libc::c_int = 0;
    let mut num_workers: libc::c_int = 0;
    let mut index_jobs: *mut repack_index = 0 as *mut repack_index;
    let mut have_error: bool = false;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    let mut freed_worker: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut input_fds: *mut pollfd = 0 as *mut pollfd;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___7: ExecStatusType = PGRES_EMPTY_QUERY;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___10: libc::c_int = 0;
    res = 0 as *mut libc::c_void as *mut PGresult;
    have_error = 0 as libc::c_int != 0;
    elog(
        13 as libc::c_int,
        b"---- create indexes ----\0" as *const u8 as *const libc::c_char,
    );
    num_indexes = (*table).n_indexes;
    if num_indexes > workers.num_workers {
        num_workers = workers.num_workers;
    } else {
        num_workers = num_indexes;
    }
    num_active_workers = num_workers;
    elog(
        13 as libc::c_int,
        b"Have %d indexes and num_workers=%d\0" as *const u8 as *const libc::c_char,
        num_indexes,
        num_workers,
    );
    index_jobs = (*table).indexes;
    i = 0 as libc::c_int;
    loop {
        if !(i < num_indexes) {
            current_block = 15897653523371991391;
            break;
        }
        elog(
            13 as libc::c_int,
            b"set up index_jobs [%d]\0" as *const u8 as *const libc::c_char,
            i,
        );
        elog(
            13 as libc::c_int,
            b"target_oid   : %u\0" as *const u8 as *const libc::c_char,
            (*index_jobs.offset(i as isize)).target_oid,
        );
        elog(
            13 as libc::c_int,
            b"create_index : %s\0" as *const u8 as *const libc::c_char,
            (*index_jobs.offset(i as isize)).create_index,
        );
        if num_workers <= 1 as libc::c_int {
            command(
                (*index_jobs.offset(i as isize)).create_index,
                0 as libc::c_int,
                0 as *mut libc::c_void as *mut *const libc::c_char,
            );
            (*index_jobs.offset(i as isize)).status = FINISHED;
        } else if i < num_workers {
            (*index_jobs.offset(i as isize)).status = INPROGRESS;
            (*index_jobs.offset(i as isize)).worker_idx = i;
            elog(
                15 as libc::c_int,
                b"Initial worker %d to build index: %s\0" as *const u8
                    as *const libc::c_char,
                i,
                (*index_jobs.offset(i as isize)).create_index,
            );
            tmp___0 = PQsendQuery(
                *(workers.conns).offset(i as isize),
                (*index_jobs.offset(i as isize)).create_index,
            );
            if tmp___0 == 0 {
                tmp = PQerrorMessage(
                    *(workers.conns).offset(i as isize) as *const PGconn,
                );
                elog(
                    19 as libc::c_int,
                    b"Error sending async query: %s\n%s\0" as *const u8
                        as *const libc::c_char,
                    (*index_jobs.offset(i as isize)).create_index,
                    tmp,
                );
                have_error = 1 as libc::c_int != 0;
                current_block = 4934180340219882360;
                break;
            }
        }
        i += 1;
    }
    match current_block {
        15897653523371991391 => {
            if num_workers > 1 as libc::c_int {
                freed_worker = -(1 as libc::c_int);
                tmp___1 = pgut_malloc(
                    (::std::mem::size_of::<pollfd>() as libc::c_ulong)
                        .wrapping_mul(num_workers as libc::c_ulong),
                );
                input_fds = tmp___1 as *mut pollfd;
                i = 0 as libc::c_int;
                while i < num_workers {
                    (*input_fds.offset(i as isize))
                        .fd = PQsocket(
                        *(workers.conns).offset(i as isize) as *const PGconn,
                    );
                    (*input_fds.offset(i as isize))
                        .events = 9 as libc::c_int as libc::c_short;
                    (*input_fds.offset(i as isize))
                        .revents = 0 as libc::c_int as libc::c_short;
                    i += 1;
                }
                's_197: while num_active_workers > 0 as libc::c_int {
                    elog(
                        13 as libc::c_int,
                        b"polling %d active workers\0" as *const u8
                            as *const libc::c_char,
                        num_active_workers,
                    );
                    ret = poll(input_fds, num_workers as nfds_t, 3000 as libc::c_int);
                    if ret < 0 as libc::c_int {
                        tmp___3 = __errno_location();
                        if *tmp___3 != 4 as libc::c_int {
                            tmp___2 = __errno_location();
                            elog(
                                20 as libc::c_int,
                                b"poll() failed: %d, %d\0" as *const u8
                                    as *const libc::c_char,
                                ret,
                                *tmp___2,
                            );
                        }
                    }
                    elog(
                        13 as libc::c_int,
                        b"Poll returned: %d\0" as *const u8 as *const libc::c_char,
                        ret,
                    );
                    i = 0 as libc::c_int;
                    while i < num_indexes {
                        if (*index_jobs.offset(i as isize)).status as libc::c_uint
                            == 1 as libc::c_uint
                        {
                            tmp___5 = PQconsumeInput(
                                *(workers.conns)
                                    .offset(
                                        (*index_jobs.offset(i as isize)).worker_idx as isize,
                                    ),
                            );
                            if tmp___5 != 1 as libc::c_int {
                                tmp___4 = PQerrorMessage(
                                    *(workers.conns)
                                        .offset(
                                            (*index_jobs.offset(i as isize)).worker_idx as isize,
                                        ) as *const PGconn,
                                );
                                elog(
                                    19 as libc::c_int,
                                    b"Error fetching async query status: %s\0" as *const u8
                                        as *const libc::c_char,
                                    tmp___4,
                                );
                                have_error = 1 as libc::c_int != 0;
                                break 's_197;
                            } else {
                                tmp___8 = PQisBusy(
                                    *(workers.conns)
                                        .offset(
                                            (*index_jobs.offset(i as isize)).worker_idx as isize,
                                        ),
                                );
                                if tmp___8 == 0 {
                                    elog(
                                        15 as libc::c_int,
                                        b"Command finished in worker %d: %s\0" as *const u8
                                            as *const libc::c_char,
                                        (*index_jobs.offset(i as isize)).worker_idx,
                                        (*index_jobs.offset(i as isize)).create_index,
                                    );
                                    loop {
                                        res = PQgetResult(
                                            *(workers.conns)
                                                .offset(
                                                    (*index_jobs.offset(i as isize)).worker_idx as isize,
                                                ),
                                        );
                                        if res.is_null() {
                                            break;
                                        }
                                        tmp___7 = PQresultStatus(res as *const PGresult);
                                        if tmp___7 as libc::c_uint != 1 as libc::c_uint {
                                            tmp___6 = PQerrorMessage(
                                                *(workers.conns)
                                                    .offset(
                                                        (*index_jobs.offset(i as isize)).worker_idx as isize,
                                                    ) as *const PGconn,
                                            );
                                            elog(
                                                19 as libc::c_int,
                                                b"Error with create index: %s\0" as *const u8
                                                    as *const libc::c_char,
                                                tmp___6,
                                            );
                                            have_error = 1 as libc::c_int != 0;
                                            break 's_197;
                                        } else {
                                            PQclear(res);
                                            res = 0 as *mut libc::c_void as *mut PGresult;
                                        }
                                    }
                                    freed_worker = (*index_jobs.offset(i as isize)).worker_idx;
                                    (*index_jobs.offset(i as isize)).status = FINISHED;
                                    num_active_workers -= 1;
                                    break;
                                }
                            }
                        }
                        i += 1;
                    }
                    if !(freed_worker > -(1 as libc::c_int)) {
                        continue;
                    }
                    i = 0 as libc::c_int;
                    while i < num_indexes {
                        if (*index_jobs.offset(i as isize)).status as libc::c_uint
                            == 0 as libc::c_uint
                        {
                            (*index_jobs.offset(i as isize)).status = INPROGRESS;
                            (*index_jobs.offset(i as isize)).worker_idx = freed_worker;
                            elog(
                                15 as libc::c_int,
                                b"Assigning worker %d to build index #%d: %s\0" as *const u8
                                    as *const libc::c_char,
                                freed_worker,
                                i,
                                (*index_jobs.offset(i as isize)).create_index,
                            );
                            tmp___10 = PQsendQuery(
                                *(workers.conns).offset(freed_worker as isize),
                                (*index_jobs.offset(i as isize)).create_index,
                            );
                            if tmp___10 == 0 {
                                tmp___9 = PQerrorMessage(
                                    *(workers.conns).offset(freed_worker as isize)
                                        as *const PGconn,
                                );
                                elog(
                                    19 as libc::c_int,
                                    b"Error sending async query: %s\n%s\0" as *const u8
                                        as *const libc::c_char,
                                    (*index_jobs.offset(i as isize)).create_index,
                                    tmp___9,
                                );
                                have_error = 1 as libc::c_int != 0;
                                break 's_197;
                            } else {
                                num_active_workers += 1;
                                break;
                            }
                        } else {
                            i += 1;
                        }
                    }
                    freed_worker = -(1 as libc::c_int);
                }
            }
        }
        _ => {}
    }
    PQclear(res);
    res = 0 as *mut libc::c_void as *mut PGresult;
    return !have_error;
}
unsafe extern "C" fn repack_one_table(
    mut table: *mut repack_table,
    mut orderby___0: *const libc::c_char,
) {
    let mut current_block: u64;
    let mut res: *mut PGresult = 0 as *mut PGresult;
    let mut params: [*const libc::c_char; 3] = [0 as *const libc::c_char; 3];
    let mut num: libc::c_int = 0;
    let mut vxid: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buffer: [libc::c_char; 12] = [0; 12];
    let mut sql: PQExpBufferData = PQExpBufferData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
    };
    let mut ret: bool = false;
    let mut indexres: *mut PGresult = 0 as *mut PGresult;
    let mut indexparams: [*const libc::c_char; 2] = [0 as *const libc::c_char; 2];
    let mut indexbuffer: [libc::c_char; 12] = [0; 12];
    let mut j: libc::c_int = 0;
    let mut appname: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut table_init: bool = false;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: bool = false;
    let mut tmp___4: bool = false;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut indexdef: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___9: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: libc::c_int = 0;
    let mut tmp___13: bool = false;
    let mut tmp___14: libc::c_int = 0;
    let mut tmp___15: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___16: libc::c_int = 0;
    let mut tmp___17: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___18: libc::c_int = 0;
    let mut tmp___19: bool = false;
    let mut tmp___20: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___21: ExecStatusType = PGRES_EMPTY_QUERY;
    let mut tmp___22: libc::c_int = 0;
    let mut tmp___25: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___26: libc::c_int = 0;
    let mut tmp___27: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___28: libc::c_int = 0;
    let mut tmp___29: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___30: bool = false;
    let mut tmp___31: bool = false;
    let mut tmp___32: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___33: libc::c_int = 0;
    let mut tmp___34: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___35: bool = false;
    let mut tmp___36: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___37: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___38: bool = false;
    let mut tmp___39: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___40: *mut libc::c_char = 0 as *mut libc::c_char;
    res = 0 as *mut libc::c_void as *mut PGresult;
    vxid = 0 as *mut libc::c_void as *mut libc::c_char;
    ret = 0 as libc::c_int != 0;
    indexres = 0 as *mut libc::c_void as *mut PGresult;
    tmp = getenv(b"PGAPPNAME\0" as *const u8 as *const libc::c_char);
    appname = tmp as *const libc::c_char;
    table_init = 0 as libc::c_int != 0;
    initPQExpBuffer(&mut sql);
    elog(
        17 as libc::c_int,
        b"repacking table \"%s\"\0" as *const u8 as *const libc::c_char,
        (*table).target_name,
    );
    elog(
        13 as libc::c_int,
        b"---- repack_one_table ----\0" as *const u8 as *const libc::c_char,
    );
    elog(
        13 as libc::c_int,
        b"target_name       : %s\0" as *const u8 as *const libc::c_char,
        (*table).target_name,
    );
    elog(
        13 as libc::c_int,
        b"target_oid        : %u\0" as *const u8 as *const libc::c_char,
        (*table).target_oid,
    );
    elog(
        13 as libc::c_int,
        b"target_toast      : %u\0" as *const u8 as *const libc::c_char,
        (*table).target_toast,
    );
    elog(
        13 as libc::c_int,
        b"target_tidx       : %u\0" as *const u8 as *const libc::c_char,
        (*table).target_tidx,
    );
    elog(
        13 as libc::c_int,
        b"pkid              : %u\0" as *const u8 as *const libc::c_char,
        (*table).pkid,
    );
    elog(
        13 as libc::c_int,
        b"ckid              : %u\0" as *const u8 as *const libc::c_char,
        (*table).ckid,
    );
    elog(
        13 as libc::c_int,
        b"create_pktype     : %s\0" as *const u8 as *const libc::c_char,
        (*table).create_pktype,
    );
    elog(
        13 as libc::c_int,
        b"create_log        : %s\0" as *const u8 as *const libc::c_char,
        (*table).create_log,
    );
    elog(
        13 as libc::c_int,
        b"create_trigger    : %s\0" as *const u8 as *const libc::c_char,
        (*table).create_trigger,
    );
    elog(
        13 as libc::c_int,
        b"enable_trigger    : %s\0" as *const u8 as *const libc::c_char,
        (*table).enable_trigger,
    );
    elog(
        13 as libc::c_int,
        b"create_table      : %s\0" as *const u8 as *const libc::c_char,
        (*table).create_table,
    );
    elog(
        13 as libc::c_int,
        b"copy_data         : %s\0" as *const u8 as *const libc::c_char,
        (*table).copy_data,
    );
    if !((*table).alter_col_storage).is_null() {
        tmp___0 = (*table).alter_col_storage;
    } else {
        tmp___0 = b"(skipped)\0" as *const u8 as *const libc::c_char;
    }
    elog(
        13 as libc::c_int,
        b"alter_col_storage : %s\0" as *const u8 as *const libc::c_char,
        tmp___0,
    );
    if !((*table).drop_columns).is_null() {
        tmp___1 = (*table).drop_columns;
    } else {
        tmp___1 = b"(skipped)\0" as *const u8 as *const libc::c_char;
    }
    elog(
        13 as libc::c_int,
        b"drop_columns      : %s\0" as *const u8 as *const libc::c_char,
        tmp___1,
    );
    elog(
        13 as libc::c_int,
        b"delete_log        : %s\0" as *const u8 as *const libc::c_char,
        (*table).delete_log,
    );
    elog(
        13 as libc::c_int,
        b"lock_table        : %s\0" as *const u8 as *const libc::c_char,
        (*table).lock_table,
    );
    elog(
        13 as libc::c_int,
        b"sql_peek          : %s\0" as *const u8 as *const libc::c_char,
        (*table).sql_peek,
    );
    elog(
        13 as libc::c_int,
        b"sql_insert        : %s\0" as *const u8 as *const libc::c_char,
        (*table).sql_insert,
    );
    elog(
        13 as libc::c_int,
        b"sql_delete        : %s\0" as *const u8 as *const libc::c_char,
        (*table).sql_delete,
    );
    elog(
        13 as libc::c_int,
        b"sql_update        : %s\0" as *const u8 as *const libc::c_char,
        (*table).sql_update,
    );
    elog(
        13 as libc::c_int,
        b"sql_pop           : %s\0" as *const u8 as *const libc::c_char,
        (*table).sql_pop,
    );
    if dryrun {
        return;
    }
    pgut_atexit_push(
        Some(
            repack_cleanup_callback
                as unsafe extern "C" fn(bool, *mut libc::c_void) -> (),
        ),
        &mut (*table).target_oid as *mut Oid as *mut libc::c_void,
    );
    elog(13 as libc::c_int, b"---- setup ----\0" as *const u8 as *const libc::c_char);
    tmp___2 = utoa((*table).target_oid, buffer.as_mut_ptr());
    params[0 as libc::c_int as usize] = tmp___2 as *const libc::c_char;
    tmp___3 = advisory_lock(connection, buffer.as_mut_ptr() as *const libc::c_char);
    if tmp___3 {
        tmp___4 = lock_exclusive(
            connection,
            buffer.as_mut_ptr() as *const libc::c_char,
            (*table).lock_table,
            1 as libc::c_int != 0,
        );
        if !tmp___4 {
            if no_kill_backend {
                elog(
                    17 as libc::c_int,
                    b"Skipping repack %s due to timeout\0" as *const u8
                        as *const libc::c_char,
                    (*table).target_name,
                );
            } else {
                elog(
                    19 as libc::c_int,
                    b"lock_exclusive() failed for %s\0" as *const u8
                        as *const libc::c_char,
                    (*table).target_name,
                );
            }
        } else {
            tmp___5 = utoa((*table).target_oid, indexbuffer.as_mut_ptr());
            indexparams[0 as libc::c_int as usize] = tmp___5 as *const libc::c_char;
            if moveidx {
                indexparams[1 as libc::c_int
                    as usize] = tablespace as *const libc::c_char;
            } else {
                indexparams[1 as libc::c_int
                    as usize] = 0 as *mut libc::c_void as *const libc::c_char;
            }
            indexres = execute(
                b"SELECT pg_get_indexdef(indexrelid) FROM pg_index WHERE indrelid = $1 AND NOT indisvalid\0"
                    as *const u8 as *const libc::c_char,
                1 as libc::c_int,
                indexparams.as_mut_ptr(),
            );
            j = 0 as libc::c_int;
            loop {
                tmp___7 = PQntuples(indexres as *const PGresult);
                if !(j < tmp___7) {
                    break;
                }
                tmp___6 = getstr(indexres, j, 0 as libc::c_int);
                indexdef = tmp___6 as *const libc::c_char;
                elog(
                    19 as libc::c_int,
                    b"skipping invalid index: %s\0" as *const u8 as *const libc::c_char,
                    indexdef,
                );
                j += 1;
            }
            indexres = execute(
                b"SELECT indexrelid, repack.repack_indexdef(indexrelid, indrelid, $2, FALSE)  FROM pg_index WHERE indrelid = $1 AND indisvalid\0"
                    as *const u8 as *const libc::c_char,
                2 as libc::c_int,
                indexparams.as_mut_ptr(),
            );
            (*table).n_indexes = PQntuples(indexres as *const PGresult);
            tmp___8 = pgut_malloc(
                ((*table).n_indexes as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<repack_index>() as libc::c_ulong),
            );
            (*table).indexes = tmp___8 as *mut repack_index;
            j = 0 as libc::c_int;
            while j < (*table).n_indexes {
                (*((*table).indexes).offset(j as isize))
                    .target_oid = getoid(indexres, j, 0 as libc::c_int);
                tmp___9 = getstr(indexres, j, 1 as libc::c_int);
                let ref mut fresh7 = (*((*table).indexes).offset(j as isize))
                    .create_index;
                *fresh7 = tmp___9 as *const libc::c_char;
                (*((*table).indexes).offset(j as isize)).status = UNPROCESSED;
                (*((*table).indexes).offset(j as isize))
                    .worker_idx = -(1 as libc::c_int);
                j += 1;
            }
            j = 0 as libc::c_int;
            while j < (*table).n_indexes {
                elog(
                    13 as libc::c_int,
                    b"index[%d].target_oid      : %u\0" as *const u8
                        as *const libc::c_char,
                    j,
                    (*((*table).indexes).offset(j as isize)).target_oid,
                );
                elog(
                    13 as libc::c_int,
                    b"index[%d].create_index    : %s\0" as *const u8
                        as *const libc::c_char,
                    j,
                    (*((*table).indexes).offset(j as isize)).create_index,
                );
                j += 1;
            }
            res = execute(
                b"SELECT repack.conflicted_triggers($1)\0" as *const u8
                    as *const libc::c_char,
                1 as libc::c_int,
                params.as_mut_ptr(),
            );
            tmp___14 = PQntuples(res as *const PGresult);
            if tmp___14 > 0 as libc::c_int {
                tmp___13 = pgut_errstart(19 as libc::c_int);
                if tmp___13 {
                    tmp___10 = errdetail(
                        b"The trigger was probably installed during a previous attempt to run pg_repack on the table which was interrupted and for some reason failed to clean up the temporary objects.  Please drop the trigger or drop and recreate the pg_repack extension altogether to remove all the temporary objects left over.\0"
                            as *const u8 as *const libc::c_char,
                    );
                    tmp___11 = errmsg(
                        b"the table \"%s\" already has a trigger called \"%s\"\0"
                            as *const u8 as *const libc::c_char,
                        (*table).target_name,
                        b"repack_trigger\0" as *const u8 as *const libc::c_char,
                    );
                    tmp___12 = errcode(-(2 as libc::c_int));
                    pgut_errfinish(tmp___12, tmp___11, tmp___10);
                }
            } else {
                PQclear(res);
                res = 0 as *mut libc::c_void as *mut PGresult;
                command(
                    (*table).create_pktype,
                    0 as libc::c_int,
                    0 as *mut libc::c_void as *mut *const libc::c_char,
                );
                temp_obj_num = temp_obj_num.wrapping_add(1);
                command(
                    (*table).create_log,
                    0 as libc::c_int,
                    0 as *mut libc::c_void as *mut *const libc::c_char,
                );
                temp_obj_num = temp_obj_num.wrapping_add(1);
                command(
                    (*table).create_trigger,
                    0 as libc::c_int,
                    0 as *mut libc::c_void as *mut *const libc::c_char,
                );
                temp_obj_num = temp_obj_num.wrapping_add(1);
                command(
                    (*table).enable_trigger,
                    0 as libc::c_int,
                    0 as *mut libc::c_void as *mut *const libc::c_char,
                );
                printfPQExpBuffer(
                    &mut sql as *mut PQExpBufferData,
                    b"SELECT repack.disable_autovacuum('repack.log_%u')\0" as *const u8
                        as *const libc::c_char,
                    (*table).target_oid,
                );
                command(
                    sql.data as *const libc::c_char,
                    0 as libc::c_int,
                    0 as *mut libc::c_void as *mut *const libc::c_char,
                );
                pgut_command(
                    conn2,
                    b"BEGIN ISOLATION LEVEL READ COMMITTED\0" as *const u8
                        as *const libc::c_char,
                    0 as libc::c_int,
                    0 as *mut libc::c_void as *mut *const libc::c_char,
                );
                res = pgut_execute(
                    conn2,
                    b"SELECT pg_backend_pid()\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                    0 as *mut libc::c_void as *mut *const libc::c_char,
                );
                buffer[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
                tmp___15 = PQgetvalue(
                    res as *const PGresult,
                    0 as libc::c_int,
                    0 as libc::c_int,
                );
                libc::strncat(
                    buffer.as_mut_ptr(),
                    tmp___15 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_ulong) as _,
                );
                PQclear(res);
                res = 0 as *mut libc::c_void as *mut PGresult;
                printfPQExpBuffer(
                    &mut sql as *mut PQExpBufferData,
                    b"LOCK TABLE %s IN ACCESS SHARE MODE\0" as *const u8
                        as *const libc::c_char,
                    (*table).target_name,
                );
                elog(
                    13 as libc::c_int,
                    b"LOCK TABLE %s IN ACCESS SHARE MODE\0" as *const u8
                        as *const libc::c_char,
                    (*table).target_name,
                );
                tmp___16 = PQsetnonblocking(conn2, 1 as libc::c_int);
                if tmp___16 != 0 {
                    elog(
                        19 as libc::c_int,
                        b"Unable to set conn2 nonblocking.\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    tmp___18 = PQsendQuery(conn2, sql.data as *const libc::c_char);
                    if tmp___18 == 0 {
                        tmp___17 = PQerrorMessage(conn2 as *const PGconn);
                        elog(
                            19 as libc::c_int,
                            b"Error sending async query: %s\n%s\0" as *const u8
                                as *const libc::c_char,
                            sql.data,
                            tmp___17,
                        );
                    } else {
                        tmp___19 = kill_ddl(
                            connection,
                            (*table).target_oid,
                            1 as libc::c_int != 0,
                        );
                        if !tmp___19 {
                            if no_kill_backend {
                                elog(
                                    17 as libc::c_int,
                                    b"Skipping repack %s due to timeout.\0" as *const u8
                                        as *const libc::c_char,
                                    (*table).target_name,
                                );
                            } else {
                                elog(
                                    19 as libc::c_int,
                                    b"kill_ddl() failed.\0" as *const u8 as *const libc::c_char,
                                );
                            }
                        } else {
                            command(
                                b"COMMIT\0" as *const u8 as *const libc::c_char,
                                0 as libc::c_int,
                                0 as *mut libc::c_void as *mut *const libc::c_char,
                            );
                            table_init = 1 as libc::c_int != 0;
                            loop {
                                res = PQgetResult(conn2);
                                if res.is_null() {
                                    current_block = 9437385368411212698;
                                    break;
                                }
                                elog(
                                    13 as libc::c_int,
                                    b"Waiting on ACCESS SHARE lock...\0" as *const u8
                                        as *const libc::c_char,
                                );
                                tmp___21 = PQresultStatus(res as *const PGresult);
                                if tmp___21 as libc::c_uint != 1 as libc::c_uint {
                                    tmp___20 = PQerrorMessage(conn2 as *const PGconn);
                                    elog(
                                        19 as libc::c_int,
                                        b"Error with LOCK TABLE: %s\0" as *const u8
                                            as *const libc::c_char,
                                        tmp___20,
                                    );
                                    current_block = 7937605043431339171;
                                    break;
                                } else {
                                    PQclear(res);
                                    res = 0 as *mut libc::c_void as *mut PGresult;
                                }
                            }
                            match current_block {
                                7937605043431339171 => {}
                                _ => {
                                    tmp___22 = PQsetnonblocking(conn2, 0 as libc::c_int);
                                    if tmp___22 != 0 {
                                        elog(
                                            19 as libc::c_int,
                                            b"Unable to set conn2 blocking.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                    } else {
                                        elog(
                                            13 as libc::c_int,
                                            b"---- copy tuples ----\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        command(
                                            b"BEGIN ISOLATION LEVEL SERIALIZABLE\0" as *const u8
                                                as *const libc::c_char,
                                            0 as libc::c_int,
                                            0 as *mut libc::c_void as *mut *const libc::c_char,
                                        );
                                        command(
                                            b"SELECT set_config('work_mem', current_setting('maintenance_work_mem'), true)\0"
                                                as *const u8 as *const libc::c_char,
                                            0 as libc::c_int,
                                            0 as *mut libc::c_void as *mut *const libc::c_char,
                                        );
                                        if !orderby___0.is_null() {
                                            if *orderby___0.offset(0 as libc::c_int as isize) == 0 {
                                                command(
                                                    b"SET LOCAL synchronize_seqscans = off\0" as *const u8
                                                        as *const libc::c_char,
                                                    0 as libc::c_int,
                                                    0 as *mut libc::c_void as *mut *const libc::c_char,
                                                );
                                            }
                                        }
                                        params[0 as libc::c_int
                                            as usize] = buffer.as_mut_ptr() as *const libc::c_char;
                                        params[1 as libc::c_int as usize] = PROGRAM_NAME;
                                        tmp___28 = PQserverVersion(connection as *const PGconn);
                                        if tmp___28 >= 90200 as libc::c_int {
                                            tmp___27 = b"SELECT coalesce(array_agg(l.virtualtransaction), '{}')   FROM pg_locks AS l   LEFT JOIN pg_stat_activity AS a     ON l.pid = a.pid   LEFT JOIN pg_database AS d     ON a.datid = d.oid   WHERE l.locktype = 'virtualxid'   AND l.pid NOT IN (pg_backend_pid(), $1)   AND (l.virtualxid, l.virtualtransaction) <> ('1/1', '-1/0')   AND (a.application_name IS NULL OR a.application_name <> $2)  AND a.query !~* E'^\\\\s*vacuum\\\\s+'   AND a.query !~ E'^autovacuum: '   AND ((d.datname IS NULL OR d.datname = current_database()) OR l.database = 0)\0"
                                                as *const u8 as *const libc::c_char;
                                        } else {
                                            tmp___26 = PQserverVersion(connection as *const PGconn);
                                            if tmp___26 >= 90000 as libc::c_int {
                                                tmp___25 = b"SELECT coalesce(array_agg(l.virtualtransaction), '{}')   FROM pg_locks AS l   LEFT JOIN pg_stat_activity AS a     ON l.pid = a.procpid   LEFT JOIN pg_database AS d     ON a.datid = d.oid   WHERE l.locktype = 'virtualxid'   AND l.pid NOT IN (pg_backend_pid(), $1)   AND (l.virtualxid, l.virtualtransaction) <> ('1/1', '-1/0')   AND (a.application_name IS NULL OR a.application_name <> $2)  AND a.current_query !~* E'^\\\\s*vacuum\\\\s+'   AND a.current_query !~ E'^autovacuum: '   AND ((d.datname IS NULL OR d.datname = current_database()) OR l.database = 0)\0"
                                                    as *const u8 as *const libc::c_char;
                                            } else {
                                                tmp___25 = b"SELECT coalesce(array_agg(l.virtualtransaction), '{}')   FROM pg_locks AS l  LEFT JOIN pg_stat_activity AS a     ON l.pid = a.procpid   LEFT JOIN pg_database AS d     ON a.datid = d.oid  WHERE l.locktype = 'virtualxid' AND l.pid NOT IN (pg_backend_pid(), $1) AND (l.virtualxid, l.virtualtransaction) <> ('1/1', '-1/0')  AND a.current_query !~* E'^\\\\s*vacuum\\\\s+'  AND a.current_query !~ E'^autovacuum: '  AND ((d.datname IS NULL OR d.datname = current_database()) OR l.database = 0) AND ($2::text IS NOT NULL)\0"
                                                    as *const u8 as *const libc::c_char;
                                            }
                                            tmp___27 = tmp___25;
                                        }
                                        res = execute(
                                            tmp___27,
                                            2 as libc::c_int,
                                            params.as_mut_ptr(),
                                        );
                                        tmp___29 = PQgetvalue(
                                            res as *const PGresult,
                                            0 as libc::c_int,
                                            0 as libc::c_int,
                                        );
                                        vxid = pgut_strdup(tmp___29 as *const libc::c_char);
                                        PQclear(res);
                                        res = 0 as *mut libc::c_void as *mut PGresult;
                                        command(
                                            (*table).delete_log,
                                            0 as libc::c_int,
                                            0 as *mut libc::c_void as *mut *const libc::c_char,
                                        );
                                        tmp___30 = lock_access_share(
                                            connection,
                                            (*table).target_oid,
                                            (*table).target_name,
                                        );
                                        if tmp___30 {
                                            command(
                                                (*table).create_table,
                                                0 as libc::c_int,
                                                0 as *mut libc::c_void as *mut *const libc::c_char,
                                            );
                                            if !((*table).alter_col_storage).is_null() {
                                                command(
                                                    (*table).alter_col_storage,
                                                    0 as libc::c_int,
                                                    0 as *mut libc::c_void as *mut *const libc::c_char,
                                                );
                                            }
                                            command(
                                                (*table).copy_data,
                                                0 as libc::c_int,
                                                0 as *mut libc::c_void as *mut *const libc::c_char,
                                            );
                                            temp_obj_num = temp_obj_num.wrapping_add(1);
                                            printfPQExpBuffer(
                                                &mut sql as *mut PQExpBufferData,
                                                b"SELECT repack.disable_autovacuum('repack.table_%u')\0"
                                                    as *const u8 as *const libc::c_char,
                                                (*table).target_oid,
                                            );
                                            if !((*table).drop_columns).is_null() {
                                                command(
                                                    (*table).drop_columns,
                                                    0 as libc::c_int,
                                                    0 as *mut libc::c_void as *mut *const libc::c_char,
                                                );
                                            }
                                            command(
                                                sql.data as *const libc::c_char,
                                                0 as libc::c_int,
                                                0 as *mut libc::c_void as *mut *const libc::c_char,
                                            );
                                            command(
                                                b"COMMIT\0" as *const u8 as *const libc::c_char,
                                                0 as libc::c_int,
                                                0 as *mut libc::c_void as *mut *const libc::c_char,
                                            );
                                            tmp___31 = rebuild_indexes(table as *const repack_table);
                                            if tmp___31 {
                                                PQclear(indexres);
                                                indexres = 0 as *mut libc::c_void as *mut PGresult;
                                                PQclear(res);
                                                res = 0 as *mut libc::c_void as *mut PGresult;
                                                loop {
                                                    num = apply_log(
                                                        connection,
                                                        table as *const repack_table,
                                                        1000 as libc::c_int,
                                                    );
                                                    if num > 20 as libc::c_int {
                                                        continue;
                                                    }
                                                    params[0 as libc::c_int
                                                        as usize] = vxid as *const libc::c_char;
                                                    res = execute(
                                                        b"SELECT pid FROM pg_locks WHERE locktype = 'virtualxid' AND pid <> pg_backend_pid() AND virtualtransaction = ANY($1)\0"
                                                            as *const u8 as *const libc::c_char,
                                                        1 as libc::c_int,
                                                        params.as_mut_ptr(),
                                                    );
                                                    num = PQntuples(res as *const PGresult);
                                                    if num > 0 as libc::c_int {
                                                        if appname.is_null() {
                                                            tmp___32 = PQgetvalue(
                                                                res as *const PGresult,
                                                                0 as libc::c_int,
                                                                0 as libc::c_int,
                                                            );
                                                            elog(
                                                                18 as libc::c_int,
                                                                b"Waiting for %d transactions to finish. First PID: %s\0"
                                                                    as *const u8 as *const libc::c_char,
                                                                num,
                                                                tmp___32,
                                                            );
                                                        } else {
                                                            tmp___33 = strcmp(
                                                                appname,
                                                                b"pg_regress\0" as *const u8 as *const libc::c_char,
                                                            );
                                                            if tmp___33 != 0 as libc::c_int {
                                                                tmp___32 = PQgetvalue(
                                                                    res as *const PGresult,
                                                                    0 as libc::c_int,
                                                                    0 as libc::c_int,
                                                                );
                                                                elog(
                                                                    18 as libc::c_int,
                                                                    b"Waiting for %d transactions to finish. First PID: %s\0"
                                                                        as *const u8 as *const libc::c_char,
                                                                    num,
                                                                    tmp___32,
                                                                );
                                                            }
                                                        }
                                                        PQclear(res);
                                                        res = 0 as *mut libc::c_void as *mut PGresult;
                                                        sleep(1 as libc::c_uint);
                                                    } else {
                                                        PQclear(res);
                                                        res = 0 as *mut libc::c_void as *mut PGresult;
                                                        break;
                                                    }
                                                }
                                                elog(
                                                    13 as libc::c_int,
                                                    b"---- swap ----\0" as *const u8 as *const libc::c_char,
                                                );
                                                tmp___34 = utoa((*table).target_oid, buffer.as_mut_ptr());
                                                tmp___35 = lock_exclusive(
                                                    conn2,
                                                    tmp___34 as *const libc::c_char,
                                                    (*table).lock_table,
                                                    0 as libc::c_int != 0,
                                                );
                                                if !tmp___35 {
                                                    elog(
                                                        19 as libc::c_int,
                                                        b"lock_exclusive() failed in conn2 for %s\0" as *const u8
                                                            as *const libc::c_char,
                                                        (*table).target_name,
                                                    );
                                                } else {
                                                    apply_log(
                                                        conn2,
                                                        table as *const repack_table,
                                                        0 as libc::c_int,
                                                    );
                                                    tmp___36 = utoa((*table).target_oid, buffer.as_mut_ptr());
                                                    params[0 as libc::c_int
                                                        as usize] = tmp___36 as *const libc::c_char;
                                                    pgut_command(
                                                        conn2,
                                                        b"SELECT repack.repack_swap($1)\0" as *const u8
                                                            as *const libc::c_char,
                                                        1 as libc::c_int,
                                                        params.as_mut_ptr(),
                                                    );
                                                    pgut_command(
                                                        conn2,
                                                        b"COMMIT\0" as *const u8 as *const libc::c_char,
                                                        0 as libc::c_int,
                                                        0 as *mut libc::c_void as *mut *const libc::c_char,
                                                    );
                                                    elog(
                                                        13 as libc::c_int,
                                                        b"---- drop ----\0" as *const u8 as *const libc::c_char,
                                                    );
                                                    command(
                                                        b"BEGIN ISOLATION LEVEL READ COMMITTED\0" as *const u8
                                                            as *const libc::c_char,
                                                        0 as libc::c_int,
                                                        0 as *mut libc::c_void as *mut *const libc::c_char,
                                                    );
                                                    tmp___37 = utoa((*table).target_oid, buffer.as_mut_ptr());
                                                    tmp___38 = lock_exclusive(
                                                        connection,
                                                        tmp___37 as *const libc::c_char,
                                                        (*table).lock_table,
                                                        0 as libc::c_int != 0,
                                                    );
                                                    if !tmp___38 {
                                                        elog(
                                                            19 as libc::c_int,
                                                            b"lock_exclusive() failed in connection for %s\0"
                                                                as *const u8 as *const libc::c_char,
                                                            (*table).target_name,
                                                        );
                                                    } else {
                                                        tmp___39 = utoa(temp_obj_num, indexbuffer.as_mut_ptr());
                                                        params[1 as libc::c_int
                                                            as usize] = tmp___39 as *const libc::c_char;
                                                        command(
                                                            b"SELECT repack.repack_drop($1, $2)\0" as *const u8
                                                                as *const libc::c_char,
                                                            2 as libc::c_int,
                                                            params.as_mut_ptr(),
                                                        );
                                                        command(
                                                            b"COMMIT\0" as *const u8 as *const libc::c_char,
                                                            0 as libc::c_int,
                                                            0 as *mut libc::c_void as *mut *const libc::c_char,
                                                        );
                                                        temp_obj_num = 0 as libc::c_uint;
                                                        if analyze {
                                                            elog(
                                                                13 as libc::c_int,
                                                                b"---- analyze ----\0" as *const u8 as *const libc::c_char,
                                                            );
                                                            command(
                                                                b"BEGIN ISOLATION LEVEL READ COMMITTED\0" as *const u8
                                                                    as *const libc::c_char,
                                                                0 as libc::c_int,
                                                                0 as *mut libc::c_void as *mut *const libc::c_char,
                                                            );
                                                            printfPQExpBuffer(
                                                                &mut sql as *mut PQExpBufferData,
                                                                b"ANALYZE %s\0" as *const u8 as *const libc::c_char,
                                                                (*table).target_name,
                                                            );
                                                            command(
                                                                sql.data as *const libc::c_char,
                                                                0 as libc::c_int,
                                                                0 as *mut libc::c_void as *mut *const libc::c_char,
                                                            );
                                                            command(
                                                                b"COMMIT\0" as *const u8 as *const libc::c_char,
                                                                0 as libc::c_int,
                                                                0 as *mut libc::c_void as *mut *const libc::c_char,
                                                            );
                                                        }
                                                        params[0 as libc::c_int
                                                            as usize] = b"16185446\0" as *const u8
                                                            as *const libc::c_char;
                                                        tmp___40 = utoa((*table).target_oid, buffer.as_mut_ptr());
                                                        params[1 as libc::c_int
                                                            as usize] = tmp___40 as *const libc::c_char;
                                                        res = pgut_execute(
                                                            connection,
                                                            b"SELECT pg_advisory_unlock($1, CAST(-2147483648 + $2::bigint AS integer))\0"
                                                                as *const u8 as *const libc::c_char,
                                                            2 as libc::c_int,
                                                            params.as_mut_ptr(),
                                                        );
                                                        ret = 1 as libc::c_int != 0;
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
    PQclear(res);
    res = 0 as *mut libc::c_void as *mut PGresult;
    termPQExpBuffer(&mut sql);
    if !vxid.is_null() {
        free(vxid as *mut libc::c_void);
    }
    pgut_rollback(connection);
    pgut_rollback(conn2);
    if !ret {
        if table_init {
            repack_cleanup(0 as libc::c_int != 0, table as *const repack_table);
        }
    }
}
unsafe extern "C" fn kill_ddl(
    mut conn: *mut PGconn,
    mut relid: Oid,
    mut terminate: bool,
) -> bool {
    let mut current_block: u64;
    let mut ret: bool = false;
    let mut res: *mut PGresult = 0 as *mut PGresult;
    let mut sql: PQExpBufferData = PQExpBufferData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
    };
    let mut n_tuples: libc::c_int = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: ExecStatusType = PGRES_EMPTY_QUERY;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: ExecStatusType = PGRES_EMPTY_QUERY;
    ret = 1 as libc::c_int != 0;
    initPQExpBuffer(&mut sql);
    printfPQExpBuffer(
        &mut sql as *mut PQExpBufferData,
        b"SELECT pid FROM pg_locks WHERE locktype = 'relation' AND granted = false AND relation = %u AND mode = 'AccessExclusiveLock' AND pid <> pg_backend_pid()\0"
            as *const u8 as *const libc::c_char,
        relid,
    );
    res = pgut_execute(
        conn,
        sql.data as *const libc::c_char,
        0 as libc::c_int,
        0 as *mut libc::c_void as *mut *const libc::c_char,
    );
    n_tuples = PQntuples(res as *const PGresult);
    if n_tuples != 0 as libc::c_int {
        if no_kill_backend {
            elog(
                19 as libc::c_int,
                b"%d unsafe queries remain but do not cancel them and skip to repack it\0"
                    as *const u8 as *const libc::c_char,
                n_tuples,
            );
            ret = 0 as libc::c_int != 0;
        } else {
            resetPQExpBuffer(&mut sql);
            printfPQExpBuffer(
                &mut sql as *mut PQExpBufferData,
                b"SELECT pg_cancel_backend(pid) FROM pg_locks WHERE locktype = 'relation' AND granted = false AND relation = %u AND mode = 'AccessExclusiveLock' AND pid <> pg_backend_pid()\0"
                    as *const u8 as *const libc::c_char,
                relid,
            );
            res = pgut_execute(
                conn,
                sql.data as *const libc::c_char,
                0 as libc::c_int,
                0 as *mut libc::c_void as *mut *const libc::c_char,
            );
            tmp___7 = PQresultStatus(res as *const PGresult);
            if tmp___7 as libc::c_uint != 2 as libc::c_uint {
                tmp = PQerrorMessage(conn as *const PGconn);
                elog(
                    19 as libc::c_int,
                    b"Error canceling unsafe queries: %s\0" as *const u8
                        as *const libc::c_char,
                    tmp,
                );
                ret = 0 as libc::c_int != 0;
            } else {
                tmp___5 = PQntuples(res as *const PGresult);
                if tmp___5 > 0 as libc::c_int {
                    if terminate {
                        tmp___6 = PQserverVersion(conn as *const PGconn);
                        if tmp___6 >= 80400 as libc::c_int {
                            tmp___0 = PQntuples(res as *const PGresult);
                            elog(
                                19 as libc::c_int,
                                b"Canceled %d unsafe queries. Terminating any remaining PIDs.\0"
                                    as *const u8 as *const libc::c_char,
                                tmp___0,
                            );
                            PQclear(res);
                            res = 0 as *mut libc::c_void as *mut PGresult;
                            printfPQExpBuffer(
                                &mut sql as *mut PQExpBufferData,
                                b"SELECT pg_terminate_backend(pid) FROM pg_locks WHERE locktype = 'relation' AND granted = false AND relation = %u AND mode = 'AccessExclusiveLock' AND pid <> pg_backend_pid()\0"
                                    as *const u8 as *const libc::c_char,
                                relid,
                            );
                            res = pgut_execute(
                                conn,
                                sql.data as *const libc::c_char,
                                0 as libc::c_int,
                                0 as *mut libc::c_void as *mut *const libc::c_char,
                            );
                            tmp___2 = PQresultStatus(res as *const PGresult);
                            if tmp___2 as libc::c_uint != 2 as libc::c_uint {
                                tmp___1 = PQerrorMessage(conn as *const PGconn);
                                elog(
                                    19 as libc::c_int,
                                    b"Error killing unsafe queries: %s\0" as *const u8
                                        as *const libc::c_char,
                                    tmp___1,
                                );
                                ret = 0 as libc::c_int != 0;
                            }
                            current_block = 2480299350034459858;
                        } else {
                            current_block = 8698692796900969279;
                        }
                    } else {
                        current_block = 8698692796900969279;
                    }
                } else {
                    current_block = 8698692796900969279;
                }
                match current_block {
                    2480299350034459858 => {}
                    _ => {
                        tmp___4 = PQntuples(res as *const PGresult);
                        if tmp___4 > 0 as libc::c_int {
                            tmp___3 = PQntuples(res as *const PGresult);
                            elog(
                                18 as libc::c_int,
                                b"Canceled %d unsafe queries\0" as *const u8
                                    as *const libc::c_char,
                                tmp___3,
                            );
                        }
                    }
                }
            }
        }
    } else {
        elog(
            13 as libc::c_int,
            b"No competing DDL to cancel.\0" as *const u8 as *const libc::c_char,
        );
    }
    PQclear(res);
    res = 0 as *mut libc::c_void as *mut PGresult;
    termPQExpBuffer(&mut sql);
    return ret;
}
unsafe extern "C" fn lock_access_share(
    mut conn: *mut PGconn,
    mut relid: Oid,
    mut target_name: *const libc::c_char,
) -> bool {
    let mut sql: PQExpBufferData = PQExpBufferData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
    };
    let mut start: time_t = 0;
    let mut tmp: time_t = 0;
    let mut i: libc::c_int = 0;
    let mut ret: bool = false;
    let mut duration: time_t = 0;
    let mut res: *mut PGresult = 0 as *mut PGresult;
    let mut wait_msec: libc::c_int = 0;
    let mut tmp___0: time_t = 0;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: bool = false;
    let mut tmp___3: ExecStatusType = PGRES_EMPTY_QUERY;
    tmp = time(0 as *mut libc::c_void as *mut time_t);
    start = tmp;
    ret = 1 as libc::c_int != 0;
    initPQExpBuffer(&mut sql);
    i = 1 as libc::c_int;
    loop {
        tmp___0 = time(0 as *mut libc::c_void as *mut time_t);
        duration = tmp___0 - start;
        if duration > (wait_timeout * 2 as libc::c_int) as time_t {
            ret = kill_ddl(conn, relid, 1 as libc::c_int != 0);
        } else {
            ret = kill_ddl(conn, relid, 0 as libc::c_int != 0);
        }
        if !ret {
            break;
        }
        if (1000 as libc::c_int) < i * 100 as libc::c_int {
            wait_msec = 1000 as libc::c_int;
        } else {
            wait_msec = i * 100 as libc::c_int;
        }
        printfPQExpBuffer(
            &mut sql as *mut PQExpBufferData,
            b"SET LOCAL statement_timeout = %d\0" as *const u8 as *const libc::c_char,
            wait_msec,
        );
        pgut_command(
            conn,
            sql.data as *const libc::c_char,
            0 as libc::c_int,
            0 as *mut libc::c_void as *mut *const libc::c_char,
        );
        printfPQExpBuffer(
            &mut sql as *mut PQExpBufferData,
            b"LOCK TABLE %s IN ACCESS SHARE MODE\0" as *const u8 as *const libc::c_char,
            target_name,
        );
        res = pgut_execute_elevel(
            conn,
            sql.data as *const libc::c_char,
            0 as libc::c_int,
            0 as *mut libc::c_void as *mut *const libc::c_char,
            13 as libc::c_int,
        );
        tmp___3 = PQresultStatus(res as *const PGresult);
        if tmp___3 as libc::c_uint == 1 as libc::c_uint {
            PQclear(res);
            res = 0 as *mut libc::c_void as *mut PGresult;
            break;
        } else {
            tmp___2 = sqlstate_equals(
                res,
                b"57014\0" as *const u8 as *const libc::c_char,
            );
            if tmp___2 {
                PQclear(res);
                res = 0 as *mut libc::c_void as *mut PGresult;
                pgut_rollback(conn);
                i += 1;
            } else {
                tmp___1 = PQerrorMessage(connection as *const PGconn);
                elog(
                    19 as libc::c_int,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    tmp___1,
                );
                PQclear(res);
                res = 0 as *mut libc::c_void as *mut PGresult;
                ret = 0 as libc::c_int != 0;
                break;
            }
        }
    }
    termPQExpBuffer(&mut sql);
    pgut_command(
        conn,
        b"RESET statement_timeout\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        0 as *mut libc::c_void as *mut *const libc::c_char,
    );
    return ret;
}
unsafe extern "C" fn advisory_lock(
    mut conn: *mut PGconn,
    mut relid: *const libc::c_char,
) -> bool {
    let mut res: *mut PGresult = 0 as *mut PGresult;
    let mut ret: bool = false;
    let mut params: [*const libc::c_char; 2] = [0 as *const libc::c_char; 2];
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: ExecStatusType = PGRES_EMPTY_QUERY;
    res = 0 as *mut libc::c_void as *mut PGresult;
    ret = 0 as libc::c_int != 0;
    params[0 as libc::c_int
        as usize] = b"16185446\0" as *const u8 as *const libc::c_char;
    params[1 as libc::c_int as usize] = relid;
    res = pgut_execute(
        conn,
        b"SELECT pg_try_advisory_lock($1, CAST(-2147483648 + $2::bigint AS integer))\0"
            as *const u8 as *const libc::c_char,
        2 as libc::c_int,
        params.as_mut_ptr(),
    );
    tmp___2 = PQresultStatus(res as *const PGresult);
    if tmp___2 as libc::c_uint != 2 as libc::c_uint {
        tmp = PQerrorMessage(connection as *const PGconn);
        elog(20 as libc::c_int, b"%s\0" as *const u8 as *const libc::c_char, tmp);
    } else {
        tmp___0 = getstr(res, 0 as libc::c_int, 0 as libc::c_int);
        tmp___1 = strcmp(
            tmp___0 as *const libc::c_char,
            b"t\0" as *const u8 as *const libc::c_char,
        );
        if tmp___1 != 0 as libc::c_int {
            elog(
                20 as libc::c_int,
                b"Another pg_repack command may be running on the table. Please try again later.\0"
                    as *const u8 as *const libc::c_char,
            );
        } else {
            ret = 1 as libc::c_int != 0;
        }
    }
    PQclear(res);
    res = 0 as *mut libc::c_void as *mut PGresult;
    return ret;
}
unsafe extern "C" fn lock_exclusive(
    mut conn: *mut PGconn,
    mut relid: *const libc::c_char,
    mut lock_query: *const libc::c_char,
    mut start_xact: bool,
) -> bool {
    let mut start: time_t = 0;
    let mut tmp: time_t = 0;
    let mut i: libc::c_int = 0;
    let mut ret: bool = false;
    let mut duration: time_t = 0;
    let mut sql: [libc::c_char; 1024] = [0; 1024];
    let mut res: *mut PGresult = 0 as *mut PGresult;
    let mut wait_msec: libc::c_int = 0;
    let mut tmp___0: time_t = 0;
    let mut cancel_query: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: bool = false;
    let mut tmp___4: ExecStatusType = PGRES_EMPTY_QUERY;
    tmp = time(0 as *mut libc::c_void as *mut time_t);
    start = tmp;
    ret = 1 as libc::c_int != 0;
    i = 1 as libc::c_int;
    loop {
        if start_xact {
            pgut_command(
                conn,
                b"BEGIN ISOLATION LEVEL READ COMMITTED\0" as *const u8
                    as *const libc::c_char,
                0 as libc::c_int,
                0 as *mut libc::c_void as *mut *const libc::c_char,
            );
        } else {
            pgut_command(
                conn,
                b"SAVEPOINT repack_sp1\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                0 as *mut libc::c_void as *mut *const libc::c_char,
            );
        }
        tmp___0 = time(0 as *mut libc::c_void as *mut time_t);
        duration = tmp___0 - start;
        if duration > wait_timeout as time_t {
            if no_kill_backend {
                elog(
                    19 as libc::c_int,
                    b"timed out, do not cancel conflicting backends\0" as *const u8
                        as *const libc::c_char,
                );
                ret = 0 as libc::c_int != 0;
                if start_xact {
                    pgut_rollback(conn);
                } else {
                    pgut_command(
                        conn,
                        b"ROLLBACK TO SAVEPOINT repack_sp1\0" as *const u8
                            as *const libc::c_char,
                        0 as libc::c_int,
                        0 as *mut libc::c_void as *mut *const libc::c_char,
                    );
                }
                break;
            } else {
                tmp___1 = PQserverVersion(conn as *const PGconn);
                if tmp___1 >= 80400 as libc::c_int {
                    if duration > (wait_timeout * 2 as libc::c_int) as time_t {
                        elog(
                            19 as libc::c_int,
                            b"terminating conflicted backends\0" as *const u8
                                as *const libc::c_char,
                        );
                        cancel_query = b"SELECT pg_terminate_backend(pid) FROM pg_locks WHERE locktype = 'relation'   AND relation = $1 AND pid <> pg_backend_pid()\0"
                            as *const u8 as *const libc::c_char;
                    } else {
                        elog(
                            19 as libc::c_int,
                            b"canceling conflicted backends\0" as *const u8
                                as *const libc::c_char,
                        );
                        cancel_query = b"SELECT pg_cancel_backend(pid) FROM pg_locks WHERE locktype = 'relation'   AND relation = $1 AND pid <> pg_backend_pid()\0"
                            as *const u8 as *const libc::c_char;
                    }
                } else {
                    elog(
                        19 as libc::c_int,
                        b"canceling conflicted backends\0" as *const u8
                            as *const libc::c_char,
                    );
                    cancel_query = b"SELECT pg_cancel_backend(pid) FROM pg_locks WHERE locktype = 'relation'   AND relation = $1 AND pid <> pg_backend_pid()\0"
                        as *const u8 as *const libc::c_char;
                }
                pgut_command(conn, cancel_query, 1 as libc::c_int, &mut relid);
            }
        }
        if (1000 as libc::c_int) < i * 100 as libc::c_int {
            wait_msec = 1000 as libc::c_int;
        } else {
            wait_msec = i * 100 as libc::c_int;
        }
        pg_snprintf(
            sql.as_mut_ptr(),
            (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            b"SET LOCAL statement_timeout = %d\0" as *const u8 as *const libc::c_char,
            wait_msec,
        );
        pgut_command(
            conn,
            sql.as_mut_ptr() as *const libc::c_char,
            0 as libc::c_int,
            0 as *mut libc::c_void as *mut *const libc::c_char,
        );
        res = pgut_execute_elevel(
            conn,
            lock_query,
            0 as libc::c_int,
            0 as *mut libc::c_void as *mut *const libc::c_char,
            13 as libc::c_int,
        );
        tmp___4 = PQresultStatus(res as *const PGresult);
        if tmp___4 as libc::c_uint == 1 as libc::c_uint {
            PQclear(res);
            res = 0 as *mut libc::c_void as *mut PGresult;
            break;
        } else {
            tmp___3 = sqlstate_equals(
                res,
                b"57014\0" as *const u8 as *const libc::c_char,
            );
            if tmp___3 {
                PQclear(res);
                res = 0 as *mut libc::c_void as *mut PGresult;
                if start_xact {
                    pgut_rollback(conn);
                } else {
                    pgut_command(
                        conn,
                        b"ROLLBACK TO SAVEPOINT repack_sp1\0" as *const u8
                            as *const libc::c_char,
                        0 as libc::c_int,
                        0 as *mut libc::c_void as *mut *const libc::c_char,
                    );
                }
                i += 1;
            } else {
                tmp___2 = PQerrorMessage(connection as *const PGconn);
                pg_printf(b"%s\0" as *const u8 as *const libc::c_char, tmp___2);
                PQclear(res);
                res = 0 as *mut libc::c_void as *mut PGresult;
                ret = 0 as libc::c_int != 0;
                break;
            }
        }
    }
    pgut_command(
        conn,
        b"RESET statement_timeout\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        0 as *mut libc::c_void as *mut *const libc::c_char,
    );
    return ret;
}
unsafe extern "C" fn repack_cleanup_callback(
    mut fatal: bool,
    mut userdata: *mut libc::c_void,
) {
    let mut target_table: Oid = 0;
    let mut params: [*const libc::c_char; 2] = [0 as *const libc::c_char; 2];
    let mut buffer: [libc::c_char; 12] = [0; 12];
    let mut num_buff: [libc::c_char; 12] = [0; 12];
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    target_table = *(userdata as *mut Oid);
    if fatal {
        tmp = utoa(target_table, buffer.as_mut_ptr());
        params[0 as libc::c_int as usize] = tmp as *const libc::c_char;
        tmp___0 = utoa(temp_obj_num, num_buff.as_mut_ptr());
        params[1 as libc::c_int as usize] = tmp___0 as *const libc::c_char;
        reconnect(20 as libc::c_int);
        command(
            b"SELECT repack.repack_drop($1, $2)\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int,
            params.as_mut_ptr(),
        );
        temp_obj_num = 0 as libc::c_uint;
    }
}
unsafe extern "C" fn repack_cleanup(mut fatal: bool, mut table: *const repack_table) {
    let mut buffer: [libc::c_char; 12] = [0; 12];
    let mut num_buff: [libc::c_char; 12] = [0; 12];
    let mut params: [*const libc::c_char; 2] = [0 as *const libc::c_char; 2];
    let mut tmp: ConnStatusType = CONNECTION_OK;
    let mut tmp___0: ConnStatusType = CONNECTION_OK;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    if fatal {
        pg_fprintf(
            stderr,
            b"!!!FATAL ERROR!!! Please refer to the manual.\n\n\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        tmp = PQstatus(connection as *const PGconn);
        if tmp as libc::c_uint != 0 as libc::c_uint {
            reconnect(20 as libc::c_int);
        } else {
            tmp___0 = PQstatus(conn2 as *const PGconn);
            if tmp___0 as libc::c_uint != 0 as libc::c_uint {
                reconnect(20 as libc::c_int);
            }
        }
        tmp___1 = utoa((*table).target_oid, buffer.as_mut_ptr());
        params[0 as libc::c_int as usize] = tmp___1 as *const libc::c_char;
        tmp___2 = utoa(temp_obj_num, num_buff.as_mut_ptr());
        params[1 as libc::c_int as usize] = tmp___2 as *const libc::c_char;
        command(
            b"SELECT repack.repack_drop($1, $2)\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int,
            params.as_mut_ptr(),
        );
        temp_obj_num = 0 as libc::c_uint;
    };
}
unsafe extern "C" fn repack_table_indexes(mut index_details: *mut PGresult) -> bool {
    let mut ret: bool = false;
    let mut res: *mut PGresult = 0 as *mut PGresult;
    let mut res2: *mut PGresult = 0 as *mut PGresult;
    let mut sql: PQExpBufferData = PQExpBufferData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
    };
    let mut sql_drop: PQExpBufferData = PQExpBufferData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
    };
    let mut buffer: [[libc::c_char; 12]; 2] = [[0; 12]; 2];
    let mut create_idx: *const libc::c_char = 0 as *const libc::c_char;
    let mut schema_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut table_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut params: [*const libc::c_char; 3] = [0 as *const libc::c_char; 3];
    let mut table: Oid = 0;
    let mut index___0: Oid = 0;
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut num_repacked: libc::c_int = 0;
    let mut repacked_indexes: *mut bool = 0 as *mut bool;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: bool = false;
    let mut tmp___5: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: bool = false;
    let mut tmp___9: bool = false;
    let mut isvalid: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___10: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut idx_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___11: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___12: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___13: ExecStatusType = PGRES_EMPTY_QUERY;
    let mut tmp___14: libc::c_int = 0;
    let mut tmp___15: libc::c_int = 0;
    let mut tmp___16: libc::c_int = 0;
    let mut tmp___17: bool = false;
    let mut tmp___18: libc::c_int = 0;
    let mut tmp___19: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___20: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___21: libc::c_int = 0;
    let mut tmp___22: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___23: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___24: libc::c_int = 0;
    let mut tmp___25: libc::c_int = 0;
    let mut tmp___26: bool = false;
    let mut tmp___27: ExecStatusType = PGRES_EMPTY_QUERY;
    let mut tmp___28: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___29: bool = false;
    let mut tmp___30: *mut libc::c_char = 0 as *mut libc::c_char;
    ret = 0 as libc::c_int != 0;
    res = 0 as *mut libc::c_void as *mut PGresult;
    res2 = 0 as *mut libc::c_void as *mut PGresult;
    num_repacked = 0 as libc::c_int;
    initPQExpBuffer(&mut sql);
    num = PQntuples(index_details as *const PGresult);
    table = getoid(index_details, 0 as libc::c_int, 3 as libc::c_int);
    tmp = utoa(table, (buffer[1 as libc::c_int as usize]).as_mut_ptr());
    params[1 as libc::c_int as usize] = tmp as *const libc::c_char;
    params[2 as libc::c_int as usize] = tablespace as *const libc::c_char;
    tmp___0 = getstr(index_details, 0 as libc::c_int, 5 as libc::c_int);
    schema_name = tmp___0 as *const libc::c_char;
    tmp___1 = getstr(index_details, 0 as libc::c_int, 4 as libc::c_int);
    table_name = tmp___1 as *const libc::c_char;
    tmp___5 = calloc(num as size_t, ::std::mem::size_of::<bool>() as libc::c_ulong);
    repacked_indexes = tmp___5 as *mut bool;
    if repacked_indexes.is_null() {
        tmp___4 = pgut_errstart(20 as libc::c_int);
        if tmp___4 {
            tmp___2 = errmsg(
                b"Unable to calloc repacked_indexes\0" as *const u8
                    as *const libc::c_char,
            );
            tmp___3 = errcode(12 as libc::c_int);
            pgut_errfinish(tmp___3, tmp___2);
        }
    }
    tmp___9 = advisory_lock(connection, params[1 as libc::c_int as usize]);
    if !tmp___9 {
        tmp___8 = pgut_errstart(20 as libc::c_int);
        if tmp___8 {
            tmp___6 = errmsg(
                b"Unable to obtain advisory lock on \"%s\"\0" as *const u8
                    as *const libc::c_char,
                table_name,
            );
            tmp___7 = errcode(22 as libc::c_int);
            pgut_errfinish(tmp___7, tmp___6);
        }
    }
    i = 0 as libc::c_int;
    while i < num {
        tmp___10 = getstr(index_details, i, 2 as libc::c_int);
        isvalid = tmp___10;
        tmp___11 = getstr(index_details, i, 0 as libc::c_int);
        idx_name = tmp___11;
        if *isvalid.offset(0 as libc::c_int as isize) as libc::c_int
            == 116 as libc::c_int
        {
            index___0 = getoid(index_details, i, 1 as libc::c_int);
            resetPQExpBuffer(&mut sql);
            appendPQExpBuffer(
                &mut sql as *mut PQExpBufferData,
                b"SELECT pgc.relname, nsp.nspname FROM pg_class pgc INNER JOIN pg_namespace nsp ON nsp.oid = pgc.relnamespace WHERE pgc.relname = 'index_%u' AND nsp.nspname = $1\0"
                    as *const u8 as *const libc::c_char,
                index___0,
            );
            params[0 as libc::c_int as usize] = schema_name;
            elog(
                17 as libc::c_int,
                b"repacking index \"%s\"\0" as *const u8 as *const libc::c_char,
                idx_name,
            );
            res = execute(
                sql.data as *const libc::c_char,
                1 as libc::c_int,
                params.as_mut_ptr(),
            );
            tmp___13 = PQresultStatus(res as *const PGresult);
            if tmp___13 as libc::c_uint != 2 as libc::c_uint {
                tmp___12 = PQerrorMessage(connection as *const PGconn);
                elog(
                    19 as libc::c_int,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    tmp___12,
                );
            } else {
                tmp___18 = PQntuples(res as *const PGresult);
                if tmp___18 > 0 as libc::c_int {
                    tmp___17 = pgut_errstart(19 as libc::c_int);
                    if tmp___17 {
                        tmp___14 = errdetail(
                            b"An invalid index may have been left behind by a previous pg_repack on the table which was interrupted. Please use DROP INDEX \"%s\".\"index_%u\" to remove this index and try again.\0"
                                as *const u8 as *const libc::c_char,
                            schema_name,
                            index___0,
                        );
                        tmp___15 = errmsg(
                            b"Cannot create index \"%s\".\"index_%u\", already exists\0"
                                as *const u8 as *const libc::c_char,
                            schema_name,
                            index___0,
                        );
                        tmp___16 = errcode(-(2 as libc::c_int));
                        pgut_errfinish(tmp___16, tmp___15, tmp___14);
                    }
                } else if !dryrun {
                    tmp___19 = utoa(
                        index___0,
                        (buffer[0 as libc::c_int as usize]).as_mut_ptr(),
                    );
                    params[0 as libc::c_int as usize] = tmp___19 as *const libc::c_char;
                    res = execute(
                        b"SELECT repack.repack_indexdef($1, $2, $3, true)\0" as *const u8
                            as *const libc::c_char,
                        3 as libc::c_int,
                        params.as_mut_ptr(),
                    );
                    tmp___21 = PQntuples(res as *const PGresult);
                    if tmp___21 < 1 as libc::c_int {
                        tmp___20 = getstr(index_details, i, 0 as libc::c_int);
                        elog(
                            19 as libc::c_int,
                            b"unable to generate SQL to CREATE work index for %s\0"
                                as *const u8 as *const libc::c_char,
                            tmp___20,
                        );
                    } else {
                        tmp___22 = getstr(res, 0 as libc::c_int, 0 as libc::c_int);
                        create_idx = tmp___22 as *const libc::c_char;
                        res2 = execute_elevel(
                            create_idx,
                            0 as libc::c_int,
                            0 as *mut libc::c_void as *mut *const libc::c_char,
                            13 as libc::c_int,
                        );
                        tmp___27 = PQresultStatus(res2 as *const PGresult);
                        if tmp___27 as libc::c_uint != 1 as libc::c_uint {
                            tmp___26 = pgut_errstart(19 as libc::c_int);
                            if tmp___26 {
                                tmp___23 = PQerrorMessage(connection as *const PGconn);
                                tmp___24 = errmsg(
                                    b"Error creating index \"%s\".\"index_%u\": %s\0"
                                        as *const u8 as *const libc::c_char,
                                    schema_name,
                                    index___0,
                                    tmp___23,
                                );
                                tmp___25 = errcode(-(2 as libc::c_int));
                                pgut_errfinish(tmp___25, tmp___24);
                            }
                        } else {
                            *repacked_indexes.offset(i as isize) = 1 as libc::c_int != 0;
                            num_repacked += 1;
                        }
                        PQclear(res);
                        res = 0 as *mut libc::c_void as *mut PGresult;
                        PQclear(res2);
                        res2 = 0 as *mut libc::c_void as *mut PGresult;
                    }
                }
            }
        } else {
            tmp___28 = getstr(index_details, i, 0 as libc::c_int);
            elog(
                19 as libc::c_int,
                b"skipping invalid index: %s.%s\0" as *const u8 as *const libc::c_char,
                schema_name,
                tmp___28,
            );
        }
        i += 1;
    }
    if dryrun {
        ret = 1 as libc::c_int != 0;
    } else {
        if num_repacked == 0 {
            elog(
                19 as libc::c_int,
                b"Skipping index swapping for \"%s\", since no new indexes built\0"
                    as *const u8 as *const libc::c_char,
                table_name,
            );
        } else {
            resetPQExpBuffer(&mut sql);
            appendPQExpBuffer(
                &mut sql as *mut PQExpBufferData,
                b"LOCK TABLE %s IN ACCESS EXCLUSIVE MODE\0" as *const u8
                    as *const libc::c_char,
                table_name,
            );
            tmp___29 = lock_exclusive(
                connection,
                params[1 as libc::c_int as usize],
                sql.data as *const libc::c_char,
                1 as libc::c_int != 0,
            );
            if !tmp___29 {
                elog(
                    19 as libc::c_int,
                    b"lock_exclusive() failed in connection for %s\0" as *const u8
                        as *const libc::c_char,
                    table_name,
                );
            } else {
                i = 0 as libc::c_int;
                while i < num {
                    index___0 = getoid(index_details, i, 1 as libc::c_int);
                    if *repacked_indexes.offset(i as isize) {
                        tmp___30 = utoa(
                            index___0,
                            (buffer[0 as libc::c_int as usize]).as_mut_ptr(),
                        );
                        params[0 as libc::c_int
                            as usize] = tmp___30 as *const libc::c_char;
                        pgut_command(
                            connection,
                            b"SELECT repack.repack_index_swap($1)\0" as *const u8
                                as *const libc::c_char,
                            1 as libc::c_int,
                            params.as_mut_ptr(),
                        );
                    } else {
                        elog(
                            17 as libc::c_int,
                            b"Skipping index swap for index_%u\0" as *const u8
                                as *const libc::c_char,
                            index___0,
                        );
                    }
                    i += 1;
                }
                pgut_command(
                    connection,
                    b"COMMIT\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                    0 as *mut libc::c_void as *mut *const libc::c_char,
                );
                ret = 1 as libc::c_int != 0;
            }
        }
        resetPQExpBuffer(&mut sql);
        initPQExpBuffer(&mut sql_drop);
        appendPQExpBufferStr(
            &mut sql,
            b"DROP INDEX CONCURRENTLY \0" as *const u8 as *const libc::c_char,
        );
        appendPQExpBuffer(
            &mut sql as *mut PQExpBufferData,
            b"\"%s\".\0" as *const u8 as *const libc::c_char,
            schema_name,
        );
        i = 0 as libc::c_int;
        while i < num {
            index___0 = getoid(index_details, i, 1 as libc::c_int);
            if *repacked_indexes.offset(i as isize) {
                initPQExpBuffer(&mut sql_drop);
                appendPQExpBuffer(
                    &mut sql_drop as *mut PQExpBufferData,
                    b"%s\"index_%u\"\0" as *const u8 as *const libc::c_char,
                    sql.data,
                    index___0,
                );
                command(
                    sql_drop.data as *const libc::c_char,
                    0 as libc::c_int,
                    0 as *mut libc::c_void as *mut *const libc::c_char,
                );
            } else {
                elog(
                    17 as libc::c_int,
                    b"Skipping drop of index_%u\0" as *const u8 as *const libc::c_char,
                    index___0,
                );
            }
            i += 1;
        }
        termPQExpBuffer(&mut sql_drop);
        termPQExpBuffer(&mut sql);
    }
    PQclear(res);
    res = 0 as *mut libc::c_void as *mut PGresult;
    free(repacked_indexes as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn repack_all_indexes(
    mut errbuf: *mut libc::c_char,
    mut errsize: size_t,
) -> bool {
    let mut current_block: u64;
    let mut ret: bool = false;
    let mut res: *mut PGresult = 0 as *mut PGresult;
    let mut sql: PQExpBufferData = PQExpBufferData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
    };
    let mut cell: *mut SimpleStringListCell = 0 as *mut SimpleStringListCell;
    let mut params: [*const libc::c_char; 1] = [0 as *const libc::c_char; 1];
    let mut tmp___0: bool = false;
    let mut tmp___1: bool = false;
    let mut nchildren: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: ExecStatusType = PGRES_EMPTY_QUERY;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: ExecStatusType = PGRES_EMPTY_QUERY;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: bool = false;
    ret = 0 as libc::c_int != 0;
    res = 0 as *mut libc::c_void as *mut PGresult;
    cell = 0 as *mut libc::c_void as *mut SimpleStringListCell;
    initPQExpBuffer(&mut sql);
    reconnect(20 as libc::c_int);
    if (r_index.head).is_null() {
        if (table_list.head).is_null() {
            if (parent_table_list.head).is_null() {
                __assert_fail(
                    b"r_index.head || table_list.head || parent_table_list.head\0"
                        as *const u8 as *const libc::c_char,
                    b"pg_repack.c\0" as *const u8 as *const libc::c_char,
                    2225 as libc::c_uint,
                    b"repack_all_indexes\0" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    tmp___0 = preliminary_checks(errbuf, errsize);
    if tmp___0 {
        tmp___1 = is_requested_relation_exists(errbuf, errsize);
        if tmp___1 {
            if !(r_index.head).is_null() {
                appendPQExpBufferStr(
                    &mut sql,
                    b"SELECT repack.oid2text(i.oid), idx.indexrelid, idx.indisvalid, idx.indrelid, repack.oid2text(idx.indrelid), n.nspname FROM pg_index idx JOIN pg_class i ON i.oid = idx.indexrelid JOIN pg_namespace n ON n.oid = i.relnamespace WHERE idx.indexrelid = $1::regclass ORDER BY indisvalid DESC, i.relname, n.nspname\0"
                        as *const u8 as *const libc::c_char,
                );
                cell = r_index.head;
            } else {
                if !(table_list.head).is_null() {
                    current_block = 265029363770428529;
                } else if !(parent_table_list.head).is_null() {
                    current_block = 265029363770428529;
                } else {
                    current_block = 1356832168064818221;
                }
                match current_block {
                    1356832168064818221 => {}
                    _ => {
                        appendPQExpBufferStr(
                            &mut sql,
                            b"SELECT repack.oid2text(i.oid), idx.indexrelid, idx.indisvalid, idx.indrelid, $1::text, n.nspname FROM pg_index idx JOIN pg_class i ON i.oid = idx.indexrelid JOIN pg_namespace n ON n.oid = i.relnamespace WHERE idx.indrelid = $1::regclass ORDER BY indisvalid DESC, i.relname, n.nspname\0"
                                as *const u8 as *const libc::c_char,
                        );
                        cell = parent_table_list.head;
                        while !cell.is_null() {
                            params[0 as libc::c_int
                                as usize] = ((*cell).val).as_mut_ptr()
                                as *const libc::c_char;
                            res = execute_elevel(
                                b"SELECT quote_ident(n.nspname) || '.' || quote_ident(c.relname) FROM pg_class c JOIN pg_namespace n on n.oid = c.relnamespace WHERE c.oid = ANY (repack.get_table_and_inheritors($1::regclass)) ORDER BY n.nspname, c.relname\0"
                                    as *const u8 as *const libc::c_char,
                                1 as libc::c_int,
                                params.as_mut_ptr(),
                                13 as libc::c_int,
                            );
                            tmp___3 = PQresultStatus(res as *const PGresult);
                            if tmp___3 as libc::c_uint != 2 as libc::c_uint {
                                tmp___2 = PQerrorMessage(connection as *const PGconn);
                                elog(
                                    19 as libc::c_int,
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    tmp___2,
                                );
                            } else {
                                nchildren = PQntuples(res as *const PGresult);
                                if nchildren == 0 as libc::c_int {
                                    elog(
                                        19 as libc::c_int,
                                        b"relation \"%s\" does not exist\0" as *const u8
                                            as *const libc::c_char,
                                        ((*cell).val).as_mut_ptr(),
                                    );
                                } else {
                                    i = 0 as libc::c_int;
                                    while i < nchildren {
                                        tmp___4 = getstr(res, i, 0 as libc::c_int);
                                        simple_string_list_append(
                                            &mut table_list,
                                            tmp___4 as *const libc::c_char,
                                        );
                                        i += 1;
                                    }
                                }
                            }
                            cell = (*cell).next;
                        }
                        PQclear(res);
                        res = 0 as *mut libc::c_void as *mut PGresult;
                        cell = table_list.head;
                    }
                }
            }
            while !cell.is_null() {
                params[0 as libc::c_int
                    as usize] = ((*cell).val).as_mut_ptr() as *const libc::c_char;
                res = execute_elevel(
                    sql.data as *const libc::c_char,
                    1 as libc::c_int,
                    params.as_mut_ptr(),
                    13 as libc::c_int,
                );
                tmp___6 = PQresultStatus(res as *const PGresult);
                if tmp___6 as libc::c_uint != 2 as libc::c_uint {
                    tmp___5 = PQerrorMessage(connection as *const PGconn);
                    elog(
                        19 as libc::c_int,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        tmp___5,
                    );
                } else {
                    tmp___7 = PQntuples(res as *const PGresult);
                    if tmp___7 == 0 as libc::c_int {
                        if !(table_list.head).is_null() {
                            elog(
                                19 as libc::c_int,
                                b"\"%s\" does not have any indexes\0" as *const u8
                                    as *const libc::c_char,
                                ((*cell).val).as_mut_ptr(),
                            );
                        } else if !(r_index.head).is_null() {
                            elog(
                                19 as libc::c_int,
                                b"\"%s\" is not a valid index\0" as *const u8
                                    as *const libc::c_char,
                                ((*cell).val).as_mut_ptr(),
                            );
                        }
                    } else {
                        if !(table_list.head).is_null() {
                            elog(
                                17 as libc::c_int,
                                b"repacking indexes of \"%s\"\0" as *const u8
                                    as *const libc::c_char,
                                ((*cell).val).as_mut_ptr(),
                            );
                        }
                        tmp___8 = repack_table_indexes(res);
                        if !tmp___8 {
                            elog(
                                19 as libc::c_int,
                                b"repack failed for \"%s\"\0" as *const u8
                                    as *const libc::c_char,
                                ((*cell).val).as_mut_ptr(),
                            );
                        }
                        PQclear(res);
                        res = 0 as *mut libc::c_void as *mut PGresult;
                    }
                }
                cell = (*cell).next;
            }
            ret = 1 as libc::c_int != 0;
        }
    }
    disconnect();
    termPQExpBuffer(&mut sql);
    return ret;
}
pub unsafe extern "C" fn pgut_help(mut details: bool) {
    pg_printf(
        b"%s re-organizes a PostgreSQL database.\n\n\0" as *const u8
            as *const libc::c_char,
        PROGRAM_NAME,
    );
    pg_printf(b"Usage:\n\0" as *const u8 as *const libc::c_char);
    pg_printf(
        b"  %s [OPTION]... [DBNAME]\n\0" as *const u8 as *const libc::c_char,
        PROGRAM_NAME,
    );
    if !details {
        return;
    }
    pg_printf(b"Options:\n\0" as *const u8 as *const libc::c_char);
    pg_printf(
        b"  -a, --all                 repack all databases\n\0" as *const u8
            as *const libc::c_char,
    );
    pg_printf(
        b"  -t, --table=TABLE         repack specific table only\n\0" as *const u8
            as *const libc::c_char,
    );
    pg_printf(
        b"  -I, --parent-table=TABLE  repack specific parent table and its inheritors\n\0"
            as *const u8 as *const libc::c_char,
    );
    pg_printf(
        b"  -c, --schema=SCHEMA       repack tables in specific schema only\n\0"
            as *const u8 as *const libc::c_char,
    );
    pg_printf(
        b"  -s, --tablespace=TBLSPC   move repacked tables to a new tablespace\n\0"
            as *const u8 as *const libc::c_char,
    );
    pg_printf(
        b"  -S, --moveidx             move repacked indexes to TBLSPC too\n\0"
            as *const u8 as *const libc::c_char,
    );
    pg_printf(
        b"  -o, --order-by=COLUMNS    order by columns instead of cluster keys\n\0"
            as *const u8 as *const libc::c_char,
    );
    pg_printf(
        b"  -n, --no-order            do vacuum full instead of cluster\n\0" as *const u8
            as *const libc::c_char,
    );
    pg_printf(
        b"  -N, --dry-run             print what would have been repacked\n\0"
            as *const u8 as *const libc::c_char,
    );
    pg_printf(
        b"  -j, --jobs=NUM            Use this many parallel jobs for each table\n\0"
            as *const u8 as *const libc::c_char,
    );
    pg_printf(
        b"  -i, --index=INDEX         move only the specified index\n\0" as *const u8
            as *const libc::c_char,
    );
    pg_printf(
        b"  -x, --only-indexes        move only indexes of the specified table\n\0"
            as *const u8 as *const libc::c_char,
    );
    pg_printf(
        b"  -T, --wait-timeout=SECS   timeout to cancel other backends on conflict\n\0"
            as *const u8 as *const libc::c_char,
    );
    pg_printf(
        b"  -D, --no-kill-backend     don't kill other backends when timed out\n\0"
            as *const u8 as *const libc::c_char,
    );
    pg_printf(
        b"  -Z, --no-analyze          don't analyze at end\n\0" as *const u8
            as *const libc::c_char,
    );
    pg_printf(
        b"  -k, --no-superuser-check  skip superuser checks in client\n\0" as *const u8
            as *const libc::c_char,
    );
    pg_printf(
        b"  -C, --exclude-extension   don't repack tables which belong to specific extension\n\0"
            as *const u8 as *const libc::c_char,
    );
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
static mut pgut_conn_mutex: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
pub static mut PROGRAM_NAME: *const libc::c_char = 0 as *const libc::c_void
    as *mut libc::c_void as *const libc::c_char;
pub static mut interrupted: bool = 0 as libc::c_int != 0;
static mut in_cleanup: bool = 0 as libc::c_int != 0;
pub static mut pgut_log_level: libc::c_int = 17 as libc::c_int;
pub static mut pgut_abort_level: libc::c_int = 20 as libc::c_int;
pub static mut pgut_echo: bool = 0 as libc::c_int != 0;
static mut pgut_connections: *mut pgutConn = 0 as *const pgutConn as *mut pgutConn;
pub unsafe extern "C" fn pgut_init(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    if PROGRAM_NAME as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        PROGRAM_NAME = get_progname(
            *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        );
        set_pglocale_pgservice(
            *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
            b"pgscripts\0" as *const u8 as *const libc::c_char,
        );
        tmp = getenv(b"PGAPPNAME\0" as *const u8 as *const libc::c_char);
        if tmp as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            pgut_putenv(
                b"PGAPPNAME\0" as *const u8 as *const libc::c_char,
                PROGRAM_NAME,
            );
        }
        init_cancel_handler();
        atexit(Some(on_cleanup as unsafe extern "C" fn() -> ()));
    }
}
pub unsafe extern "C" fn pgut_putenv(
    mut key: *const libc::c_char,
    mut value: *const libc::c_char,
) {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    pg_snprintf(
        buf.as_mut_ptr(),
        (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
        b"%s=%s\0" as *const u8 as *const libc::c_char,
        key,
        value,
    );
    tmp = pgut_strdup(buf.as_mut_ptr() as *const libc::c_char);
    putenv(tmp);
}
pub unsafe extern "C" fn parse_bool(
    mut value: *const libc::c_char,
    mut result: *mut bool,
) -> bool {
    let mut tmp: size_t = 0;
    let mut tmp___0: bool = false;
    tmp = strlen(value);
    tmp___0 = parse_bool_with_len(value, tmp, result);
    return tmp___0;
}
pub unsafe extern "C" fn parse_bool_with_len(
    mut value: *const libc::c_char,
    mut len: size_t,
    mut result: *mut bool,
) -> bool {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: size_t = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: size_t = 0;
    let mut tmp___6: libc::c_int = 0;
    match *value as libc::c_int {
        84 | 116 => {
            tmp = pg_strncasecmp(
                value,
                b"true\0" as *const u8 as *const libc::c_char,
                len,
            );
            if tmp == 0 as libc::c_int {
                if !result.is_null() {
                    *result = 1 as libc::c_int != 0;
                }
                return 1 as libc::c_int != 0;
            }
        }
        70 | 102 => {
            tmp___0 = pg_strncasecmp(
                value,
                b"false\0" as *const u8 as *const libc::c_char,
                len,
            );
            if tmp___0 == 0 as libc::c_int {
                if !result.is_null() {
                    *result = 0 as libc::c_int != 0;
                }
                return 1 as libc::c_int != 0;
            }
        }
        89 | 121 => {
            tmp___1 = pg_strncasecmp(
                value,
                b"yes\0" as *const u8 as *const libc::c_char,
                len,
            );
            if tmp___1 == 0 as libc::c_int {
                if !result.is_null() {
                    *result = 1 as libc::c_int != 0;
                }
                return 1 as libc::c_int != 0;
            }
        }
        78 | 110 => {
            tmp___2 = pg_strncasecmp(
                value,
                b"no\0" as *const u8 as *const libc::c_char,
                len,
            );
            if tmp___2 == 0 as libc::c_int {
                if !result.is_null() {
                    *result = 0 as libc::c_int != 0;
                }
                return 1 as libc::c_int != 0;
            }
        }
        79 | 111 => {
            if len > 2 as libc::c_ulong {
                tmp___5 = len;
            } else {
                tmp___5 = 2 as libc::c_int as size_t;
            }
            tmp___6 = pg_strncasecmp(
                value,
                b"on\0" as *const u8 as *const libc::c_char,
                tmp___5,
            );
            if tmp___6 == 0 as libc::c_int {
                if !result.is_null() {
                    *result = 1 as libc::c_int != 0;
                }
                return 1 as libc::c_int != 0;
            } else {
                if len > 2 as libc::c_ulong {
                    tmp___3 = len;
                } else {
                    tmp___3 = 2 as libc::c_int as size_t;
                }
                tmp___4 = pg_strncasecmp(
                    value,
                    b"off\0" as *const u8 as *const libc::c_char,
                    tmp___3,
                );
                if tmp___4 == 0 as libc::c_int {
                    if !result.is_null() {
                        *result = 0 as libc::c_int != 0;
                    }
                    return 1 as libc::c_int != 0;
                }
            }
        }
        49 => {
            if len == 1 as libc::c_ulong {
                if !result.is_null() {
                    *result = 1 as libc::c_int != 0;
                }
                return 1 as libc::c_int != 0;
            }
        }
        48 => {
            if len == 1 as libc::c_ulong {
                if !result.is_null() {
                    *result = 0 as libc::c_int != 0;
                }
                return 1 as libc::c_int != 0;
            }
        }
        _ => {}
    }
    if !result.is_null() {
        *result = 0 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn parse_int32(
    mut value: *const libc::c_char,
    mut result: *mut int32,
) -> bool {
    let mut val: int64 = 0;
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    tmp = strcmp(value, b"INFINITE\0" as *const u8 as *const libc::c_char);
    if tmp == 0 as libc::c_int {
        *result = 2147483647 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    tmp___0 = __errno_location();
    *tmp___0 = 0 as libc::c_int;
    val = strtol(value, &mut endptr as *mut *mut libc::c_char, 0 as libc::c_int);
    if endptr as libc::c_ulong == value as libc::c_ulong {
        return 0 as libc::c_int != 0
    } else {
        if *endptr != 0 {
            return 0 as libc::c_int != 0;
        }
    }
    tmp___1 = __errno_location();
    if *tmp___1 == 34 as libc::c_int {
        return 0 as libc::c_int != 0
    } else {
        if val != val as int32 as int64 {
            return 0 as libc::c_int != 0;
        }
    }
    *result = val as int32;
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn parse_uint32(
    mut value: *const libc::c_char,
    mut result: *mut uint32,
) -> bool {
    let mut val: uint64 = 0;
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    tmp = strcmp(value, b"INFINITE\0" as *const u8 as *const libc::c_char);
    if tmp == 0 as libc::c_int {
        *result = 4294967295 as libc::c_uint;
        return 1 as libc::c_int != 0;
    }
    tmp___0 = __errno_location();
    *tmp___0 = 0 as libc::c_int;
    val = strtoul(value, &mut endptr as *mut *mut libc::c_char, 0 as libc::c_int);
    if endptr as libc::c_ulong == value as libc::c_ulong {
        return 0 as libc::c_int != 0
    } else {
        if *endptr != 0 {
            return 0 as libc::c_int != 0;
        }
    }
    tmp___1 = __errno_location();
    if *tmp___1 == 34 as libc::c_int {
        return 0 as libc::c_int != 0
    } else {
        if val != val as uint32 as uint64 {
            return 0 as libc::c_int != 0;
        }
    }
    *result = val as uint32;
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn parse_int64(
    mut value: *const libc::c_char,
    mut result: *mut int64,
) -> bool {
    let mut val: int64 = 0;
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    tmp = strcmp(value, b"INFINITE\0" as *const u8 as *const libc::c_char);
    if tmp == 0 as libc::c_int {
        *result = 9223372036854775807 as libc::c_longlong as int64;
        return 1 as libc::c_int != 0;
    }
    tmp___0 = __errno_location();
    *tmp___0 = 0 as libc::c_int;
    val = strtol(value, &mut endptr as *mut *mut libc::c_char, 0 as libc::c_int);
    if endptr as libc::c_ulong == value as libc::c_ulong {
        return 0 as libc::c_int != 0
    } else {
        if *endptr != 0 {
            return 0 as libc::c_int != 0;
        }
    }
    tmp___1 = __errno_location();
    if *tmp___1 == 34 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    *result = val;
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn parse_uint64(
    mut value: *const libc::c_char,
    mut result: *mut uint64,
) -> bool {
    let mut val: uint64 = 0;
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    tmp = strcmp(value, b"INFINITE\0" as *const u8 as *const libc::c_char);
    if tmp == 0 as libc::c_int {
        *result = 18446744073709551615 as libc::c_ulonglong as uint64;
        return 1 as libc::c_int != 0;
    }
    tmp___0 = __errno_location();
    *tmp___0 = 0 as libc::c_int;
    val = strtoul(value, &mut endptr as *mut *mut libc::c_char, 0 as libc::c_int);
    if endptr as libc::c_ulong == value as libc::c_ulong {
        return 0 as libc::c_int != 0
    } else {
        if *endptr != 0 {
            return 0 as libc::c_int != 0;
        }
    }
    tmp___1 = __errno_location();
    if *tmp___1 == 34 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    *result = val;
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn parse_time(
    mut value: *const libc::c_char,
    mut time___0: *mut time_t,
) -> bool {
    let mut len: size_t = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut junk: [libc::c_char; 2] = [0; 2];
    let mut tmp___0: size_t = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: size_t = 0;
    let mut tmp___4: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    tmp___0 = strlen(value);
    tmp___1 = pgut_malloc(tmp___0.wrapping_add(1 as libc::c_ulong));
    tmp = tmp___1 as *mut libc::c_char;
    len = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int;
    while *value.offset(i as isize) != 0 {
        tmp___2 = len;
        len = len.wrapping_add(1);
        tmp___4 = __ctype_b_loc();
        if *(*tmp___4)
            .offset(*value.offset(i as isize) as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & 8 as libc::c_int != 0
        {
            *tmp.offset(tmp___2 as isize) = *value.offset(i as isize);
        } else {
            *tmp.offset(tmp___2 as isize) = ' ' as i32 as libc::c_char;
        }
        i += 1;
    }
    *tmp.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    libc::memset(
        &mut tm as *mut tm as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<tm>() as _,
    );
    tm.tm_year = 0 as libc::c_int;
    tm.tm_mon = 0 as libc::c_int;
    tm.tm_mday = 1 as libc::c_int;
    tm.tm_hour = 0 as libc::c_int;
    tm.tm_min = 0 as libc::c_int;
    tm.tm_sec = 0 as libc::c_int;
    i = sscanf(
        tmp as *const libc::c_char,
        b"%04d %02d %02d %02d %02d %02d%1s\0" as *const u8 as *const libc::c_char,
        &mut tm.tm_year as *mut libc::c_int,
        &mut tm.tm_mon as *mut libc::c_int,
        &mut tm.tm_mday as *mut libc::c_int,
        &mut tm.tm_hour as *mut libc::c_int,
        &mut tm.tm_min as *mut libc::c_int,
        &mut tm.tm_sec as *mut libc::c_int,
        junk.as_mut_ptr(),
    );
    free(tmp as *mut libc::c_void);
    if i < 1 as libc::c_int {
        return 0 as libc::c_int != 0
    } else {
        if (6 as libc::c_int) < i {
            return 0 as libc::c_int != 0;
        }
    }
    if tm.tm_year < 100 as libc::c_int {
        tm.tm_year += 100 as libc::c_int;
    } else if tm.tm_year >= 1900 as libc::c_int {
        tm.tm_year -= 1900 as libc::c_int;
    }
    if i > 1 as libc::c_int {
        tm.tm_mon -= 1;
    }
    tm.tm_isdst = -(1 as libc::c_int);
    *time___0 = mktime(&mut tm);
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn simple_string_list_append(
    mut list: *mut SimpleStringList,
    mut val: *const libc::c_char,
) {
    let mut cell: *mut SimpleStringListCell = 0 as *mut SimpleStringListCell;
    let mut tmp: size_t = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = strlen(val);
    tmp___0 = pgut_malloc(
        (::std::mem::size_of::<SimpleStringListCell>() as libc::c_ulong)
            .wrapping_add(tmp),
    );
    cell = tmp___0 as *mut SimpleStringListCell;
    (*cell).next = 0 as *mut libc::c_void as *mut SimpleStringListCell;
    libc::strcpy(((*cell).val).as_mut_ptr(), val);
    if !((*list).tail).is_null() {
        (*(*list).tail).next = cell;
    } else {
        (*list).head = cell;
    }
    (*list).tail = cell;
}
pub unsafe extern "C" fn simple_string_list_member(
    mut list: *mut SimpleStringList,
    mut val: *const libc::c_char,
) -> bool {
    let mut cell: *mut SimpleStringListCell = 0 as *mut SimpleStringListCell;
    let mut tmp: libc::c_int = 0;
    cell = (*list).head;
    while !cell.is_null() {
        tmp = strcmp(((*cell).val).as_mut_ptr() as *const libc::c_char, val);
        if tmp == 0 as libc::c_int {
            return 1 as libc::c_int != 0;
        }
        cell = (*cell).next;
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn simple_string_list_size(mut list: SimpleStringList) -> size_t {
    let mut i: size_t = 0;
    let mut cell: *mut SimpleStringListCell = 0 as *mut SimpleStringListCell;
    i = 0 as libc::c_int as size_t;
    cell = list.head;
    while !cell.is_null() {
        cell = (*cell).next;
        i = i.wrapping_add(1);
    }
    return i;
}
static mut passwdbuf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut have_passwd: bool = 0 as libc::c_int != 0;
unsafe extern "C" fn prompt_for_password() -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: bool = false;
    tmp = pgut_malloc(100 as libc::c_int as size_t);
    buf = tmp as *mut libc::c_char;
    if have_passwd {
        libc::memcpy(
            buf as *mut libc::c_void,
            passwdbuf as *const libc::c_void,
            (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(100 as libc::c_ulong) as _,
        );
    } else {
        if buf as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            simple_prompt(
                b"Password: \0" as *const u8 as *const libc::c_char,
                buf,
                100 as libc::c_int as size_t,
                0 as libc::c_int != 0,
            );
        }
        have_passwd = 1 as libc::c_int != 0;
        tmp___0 = pgut_malloc(100 as libc::c_int as size_t);
        passwdbuf = tmp___0 as *mut libc::c_char;
        libc::memcpy(
            passwdbuf as *mut libc::c_void,
            buf as *const libc::c_void,
            (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(100 as libc::c_ulong) as _,
        );
    }
    if buf as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___3 = pgut_errstart(21 as libc::c_int);
        if tmp___3 {
            tmp___1 = errmsg(
                b"could not allocate memory (%lu bytes): \0" as *const u8
                    as *const libc::c_char,
                100 as libc::c_int as uint64,
            );
            tmp___2 = errcode_errno();
            pgut_errfinish(tmp___2, tmp___1);
        }
    }
    return buf;
}
pub unsafe extern "C" fn pgut_connect(
    mut info: *const libc::c_char,
    mut prompt: YesNo,
    mut elevel: libc::c_int,
) -> *mut PGconn {
    let mut passwd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut add_pass: PQExpBufferData = PQExpBufferData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
    };
    let mut conn: *mut PGconn = 0 as *mut PGconn;
    let mut c: *mut pgutConn = 0 as *mut pgutConn;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: ConnStatusType = CONNECTION_OK;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: bool = false;
    if prompt as libc::c_uint == 2 as libc::c_uint {
        passwd = prompt_for_password();
        initPQExpBuffer(&mut add_pass);
        appendPQExpBufferStr(&mut add_pass, info);
        appendPQExpBuffer(
            &mut add_pass as *mut PQExpBufferData,
            b" password=%s \0" as *const u8 as *const libc::c_char,
            passwd,
        );
    } else {
        passwd = 0 as *mut libc::c_void as *mut libc::c_char;
        add_pass.data = 0 as *mut libc::c_void as *mut libc::c_char;
    }
    loop {
        's_248: {
            CHECK_FOR_INTERRUPTS();
            if passwd.is_null() {
                conn = PQconnectdb(info);
            } else {
                conn = PQconnectdb(add_pass.data as *const libc::c_char);
            }
            tmp___0 = PQstatus(conn as *const PGconn);
            if tmp___0 as libc::c_uint == 0 as libc::c_uint {
                tmp = pgut_malloc(::std::mem::size_of::<pgutConn>() as libc::c_ulong);
                c = tmp as *mut pgutConn;
                (*c).conn = conn;
                (*c).cancel = 0 as *mut libc::c_void as *mut PGcancel;
                pthread_mutex_lock(&mut pgut_conn_mutex);
                (*c).next = pgut_connections;
                pgut_connections = c;
                pthread_mutex_unlock(&mut pgut_conn_mutex);
                if add_pass.data as libc::c_ulong
                    != 0 as *mut libc::c_void as libc::c_ulong
                {
                    termPQExpBuffer(&mut add_pass);
                }
                free(passwd as *mut libc::c_void);
                pgut_command(
                    conn,
                    b"SET search_path TO pg_catalog, pg_temp, public\0" as *const u8
                        as *const libc::c_char,
                    0 as libc::c_int,
                    0 as *mut libc::c_void as *mut *const libc::c_char,
                );
                return conn;
            }
            if !conn.is_null() {
                tmp___1 = PQconnectionNeedsPassword(conn as *const PGconn);
                if tmp___1 != 0 {
                    if prompt as libc::c_uint != 1 as libc::c_uint {
                        PQfinish(conn);
                        free(passwd as *mut libc::c_void);
                        passwd = prompt_for_password();
                        if add_pass.data as libc::c_ulong
                            != 0 as *mut libc::c_void as libc::c_ulong
                        {
                            resetPQExpBuffer(&mut add_pass);
                        } else {
                            initPQExpBuffer(&mut add_pass);
                        }
                        appendPQExpBufferStr(&mut add_pass, info);
                        appendPQExpBuffer(
                            &mut add_pass as *mut PQExpBufferData,
                            b" password=%s \0" as *const u8 as *const libc::c_char,
                            passwd,
                        );
                        break 's_248;
                    }
                }
            }
            if add_pass.data as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong
            {
                termPQExpBuffer(&mut add_pass);
            }
            free(passwd as *mut libc::c_void);
            tmp___5 = pgut_errstart(elevel);
            if tmp___5 {
                tmp___2 = PQerrorMessage(conn as *const PGconn);
                tmp___3 = errmsg(
                    b"could not connect to database: %s\0" as *const u8
                        as *const libc::c_char,
                    tmp___2,
                );
                tmp___4 = errcode(-(1 as libc::c_int));
                pgut_errfinish(tmp___4, tmp___3);
            }
            PQfinish(conn);
            return 0 as *mut libc::c_void as *mut PGconn;
        }
    };
}
pub unsafe extern "C" fn pgut_disconnect(mut conn: *mut PGconn) {
    let mut c: *mut pgutConn = 0 as *mut pgutConn;
    let mut prev: *mut *mut pgutConn = 0 as *mut *mut pgutConn;
    if !conn.is_null() {
        pthread_mutex_lock(&mut pgut_conn_mutex);
        prev = &mut pgut_connections;
        c = pgut_connections;
        while !c.is_null() {
            if (*c).conn as libc::c_ulong == conn as libc::c_ulong {
                *prev = (*c).next;
                break;
            } else {
                prev = &mut (*c).next;
                c = (*c).next;
            }
        }
        pthread_mutex_unlock(&mut pgut_conn_mutex);
        PQfinish(conn);
    }
}
pub unsafe extern "C" fn pgut_disconnect_all() {
    pthread_mutex_lock(&mut pgut_conn_mutex);
    while !pgut_connections.is_null() {
        PQfinish((*pgut_connections).conn);
        pgut_connections = (*pgut_connections).next;
    }
    pthread_mutex_unlock(&mut pgut_conn_mutex);
}
unsafe extern "C" fn echo_query(
    mut query: *const libc::c_char,
    mut nParams: libc::c_int,
    mut params: *mut *const libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    tmp = strchr(query, '\n' as i32);
    if !tmp.is_null() {
        elog(
            15 as libc::c_int,
            b"(query)\n%s\0" as *const u8 as *const libc::c_char,
            query,
        );
    } else {
        elog(
            15 as libc::c_int,
            b"(query) %s\0" as *const u8 as *const libc::c_char,
            query,
        );
    }
    i = 0 as libc::c_int;
    while i < nParams {
        if !(*params.offset(i as isize)).is_null() {
            tmp___0 = *params.offset(i as isize);
        } else {
            tmp___0 = b"(null)\0" as *const u8 as *const libc::c_char;
        }
        elog(
            15 as libc::c_int,
            b"\t(param:%d) = %s\0" as *const u8 as *const libc::c_char,
            i,
            tmp___0,
        );
        i += 1;
    }
}
pub unsafe extern "C" fn pgut_execute(
    mut conn: *mut PGconn,
    mut query: *const libc::c_char,
    mut nParams: libc::c_int,
    mut params: *mut *const libc::c_char,
) -> *mut PGresult {
    let mut tmp: *mut PGresult = 0 as *mut PGresult;
    tmp = pgut_execute_elevel(conn, query, nParams, params, 20 as libc::c_int);
    return tmp;
}
pub unsafe extern "C" fn pgut_execute_elevel(
    mut conn: *mut PGconn,
    mut query: *const libc::c_char,
    mut nParams: libc::c_int,
    mut params: *mut *const libc::c_char,
    mut elevel: libc::c_int,
) -> *mut PGresult {
    let mut res: *mut PGresult = 0 as *mut PGresult;
    let mut c: *mut pgutConn = 0 as *mut pgutConn;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: bool = false;
    let mut tmp___2: ExecStatusType = PGRES_EMPTY_QUERY;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: bool = false;
    CHECK_FOR_INTERRUPTS();
    if pgut_echo {
        echo_query(query, nParams, params);
    }
    if conn as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___1 = pgut_errstart(elevel);
        if tmp___1 {
            tmp = errmsg(b"not connected\0" as *const u8 as *const libc::c_char);
            tmp___0 = errcode(-(2 as libc::c_int));
            pgut_errfinish(tmp___0, tmp);
        }
        return 0 as *mut libc::c_void as *mut PGresult;
    }
    pthread_mutex_lock(&mut pgut_conn_mutex);
    c = pgut_connections;
    while !c.is_null() {
        if (*c).conn as libc::c_ulong == conn as libc::c_ulong {
            break;
        }
        c = (*c).next;
    }
    pthread_mutex_unlock(&mut pgut_conn_mutex);
    if !c.is_null() {
        on_before_exec(c);
    }
    if nParams == 0 as libc::c_int {
        res = PQexec(conn, query);
    } else {
        res = PQexecParams(
            conn,
            query,
            nParams,
            0 as *mut libc::c_void as *const Oid,
            params as *const *const libc::c_char,
            0 as *mut libc::c_void as *const libc::c_int,
            0 as *mut libc::c_void as *const libc::c_int,
            0 as libc::c_int,
        );
    }
    if !c.is_null() {
        on_after_exec(c);
    }
    tmp___2 = PQresultStatus(res as *const PGresult);
    match tmp___2 as libc::c_uint {
        4 | 1 | 2 => {}
        _ => {
            tmp___7 = pgut_errstart(elevel);
            if tmp___7 {
                tmp___3 = errdetail(
                    b"query was: %s\0" as *const u8 as *const libc::c_char,
                    query,
                );
                tmp___4 = PQerrorMessage(conn as *const PGconn);
                tmp___5 = errmsg(
                    b"query failed: %s\0" as *const u8 as *const libc::c_char,
                    tmp___4,
                );
                tmp___6 = errcode(-(2 as libc::c_int));
                pgut_errfinish(tmp___6, tmp___5, tmp___3);
            }
        }
    }
    return res;
}
pub unsafe extern "C" fn pgut_command(
    mut conn: *mut PGconn,
    mut query: *const libc::c_char,
    mut nParams: libc::c_int,
    mut params: *mut *const libc::c_char,
) -> ExecStatusType {
    let mut res: *mut PGresult = 0 as *mut PGresult;
    let mut code: ExecStatusType = PGRES_EMPTY_QUERY;
    res = pgut_execute(conn, query, nParams, params);
    code = PQresultStatus(res as *const PGresult);
    PQclear(res);
    return code;
}
pub unsafe extern "C" fn pgut_commit(mut conn: *mut PGconn) -> bool {
    let mut tmp: ExecStatusType = PGRES_EMPTY_QUERY;
    let mut tmp___0: PGTransactionStatusType = PQTRANS_IDLE;
    if !conn.is_null() {
        tmp___0 = PQtransactionStatus(conn as *const PGconn);
        if tmp___0 as libc::c_uint != 0 as libc::c_uint {
            tmp = pgut_command(
                conn,
                b"COMMIT\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                0 as *mut libc::c_void as *mut *const libc::c_char,
            );
            return tmp as libc::c_uint == 1 as libc::c_uint;
        }
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn pgut_rollback(mut conn: *mut PGconn) {
    let mut tmp: PGTransactionStatusType = PQTRANS_IDLE;
    if !conn.is_null() {
        tmp = PQtransactionStatus(conn as *const PGconn);
        if tmp as libc::c_uint != 0 as libc::c_uint {
            pgut_command(
                conn,
                b"ROLLBACK\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                0 as *mut libc::c_void as *mut *const libc::c_char,
            );
        }
    }
}
pub unsafe extern "C" fn pgut_send(
    mut conn: *mut PGconn,
    mut query: *const libc::c_char,
    mut nParams: libc::c_int,
    mut params: *mut *const libc::c_char,
) -> bool {
    let mut res: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: bool = false;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: bool = false;
    CHECK_FOR_INTERRUPTS();
    if pgut_echo {
        echo_query(query, nParams, params);
    }
    if conn as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___1 = pgut_errstart(20 as libc::c_int);
        if tmp___1 {
            tmp = errmsg(b"not connected\0" as *const u8 as *const libc::c_char);
            tmp___0 = errcode(-(2 as libc::c_int));
            pgut_errfinish(tmp___0, tmp);
        }
        return 0 as libc::c_int != 0;
    }
    if nParams == 0 as libc::c_int {
        res = PQsendQuery(conn, query);
    } else {
        res = PQsendQueryParams(
            conn,
            query,
            nParams,
            0 as *mut libc::c_void as *const Oid,
            params as *const *const libc::c_char,
            0 as *mut libc::c_void as *const libc::c_int,
            0 as *mut libc::c_void as *const libc::c_int,
            0 as libc::c_int,
        );
    }
    if res != 1 as libc::c_int {
        tmp___6 = pgut_errstart(20 as libc::c_int);
        if tmp___6 {
            tmp___2 = errdetail(
                b"query was: %s\0" as *const u8 as *const libc::c_char,
                query,
            );
            tmp___3 = PQerrorMessage(conn as *const PGconn);
            tmp___4 = errmsg(
                b"query failed: %s\0" as *const u8 as *const libc::c_char,
                tmp___3,
            );
            tmp___5 = errcode(-(2 as libc::c_int));
            pgut_errfinish(tmp___5, tmp___4, tmp___2);
        }
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn pgut_wait(
    mut num: libc::c_int,
    mut connections: *mut *mut PGconn,
    mut timeout: *mut timeval,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut mask: fd_set = fd_set { fds_bits: [0; 16] };
    let mut maxsock: libc::c_int = 0;
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let mut sock: libc::c_int = 0;
    let mut __d: libc::c_long = 0;
    let mut tmp___1: libc::c_long = 0;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: libc::c_int = 0;
    let mut __d___0: libc::c_long = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___7: libc::c_long = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: *mut libc::c_int = 0 as *mut libc::c_int;
    while !interrupted {
        let fresh8 = &mut __d0;
        let fresh9;
        let fresh10 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
        let fresh11 = &mut __d1;
        let fresh12;
        let fresh13 = &mut *(mask.fds_bits)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh8,
            fresh10) => fresh9, inlateout("di")
            c2rust_asm_casts::AsmCast::cast_in(fresh11, fresh13) => fresh12,
            inlateout("ax") 0 as libc::c_int => _, options(preserves_flags, att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh8, fresh10, fresh9);
        c2rust_asm_casts::AsmCast::cast_out(fresh11, fresh13, fresh12);
        maxsock = -(1 as libc::c_int);
        i = 0 as libc::c_int;
        while i < num {
            if !(*connections.offset(i as isize) as libc::c_ulong
                == 0 as *mut libc::c_void as libc::c_ulong)
            {
                sock = PQsocket(*connections.offset(i as isize) as *const PGconn);
                if sock >= 0 as libc::c_int {
                    __d = sock as libc::c_long;
                    tmp___1 = __fdelt_chk(__d);
                    mask.fds_bits[tmp___1 as usize]
                        |= ((1 as libc::c_ulong)
                            << sock
                                % (8 as libc::c_int
                                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                        as libc::c_int)) as __fd_mask;
                    if maxsock < sock {
                        maxsock = sock;
                    }
                }
            }
            i += 1;
        }
        if maxsock == -(1 as libc::c_int) {
            tmp___2 = __errno_location();
            *tmp___2 = 2 as libc::c_int;
            return -(1 as libc::c_int);
        }
        i = wait_for_sockets(maxsock + 1 as libc::c_int, &mut mask, timeout);
        if i == 0 as libc::c_int {
            break;
        }
        i = 0 as libc::c_int;
        while i < num {
            if !(*connections.offset(i as isize)).is_null() {
                tmp___4 = PQsocket(*connections.offset(i as isize) as *const PGconn);
                __d___0 = tmp___4 as libc::c_long;
                tmp___7 = __fdelt_chk(__d___0);
                tmp___8 = PQsocket(*connections.offset(i as isize) as *const PGconn);
                if mask.fds_bits[tmp___7 as usize]
                    & ((1 as libc::c_ulong)
                        << tmp___8
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask != 0 as libc::c_long
                {
                    PQconsumeInput(*connections.offset(i as isize));
                    tmp___3 = PQisBusy(*connections.offset(i as isize));
                    if !(tmp___3 != 0) {
                        return i;
                    }
                }
            }
            i += 1;
        }
    }
    tmp___9 = __errno_location();
    *tmp___9 = 4 as libc::c_int;
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn CHECK_FOR_INTERRUPTS() {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: bool = false;
    if interrupted {
        if !in_cleanup {
            tmp___1 = pgut_errstart(21 as libc::c_int);
            if tmp___1 {
                tmp = errmsg(b"interrupted\0" as *const u8 as *const libc::c_char);
                tmp___0 = errcode(4 as libc::c_int);
                pgut_errfinish(tmp___0, tmp);
            }
        }
    }
}
static mut edata: pgutErrorData = pgutErrorData {
    elevel: 0,
    save_errno: 0,
    code: 0,
    msg: PQExpBufferData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
    },
    detail: PQExpBufferData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
    },
};
unsafe extern "C" fn getErrorData() -> *mut pgutErrorData {
    return &mut edata;
}
unsafe extern "C" fn pgut_errinit(mut elevel: libc::c_int) -> *mut pgutErrorData {
    let mut save_errno: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut edata___0: *mut pgutErrorData = 0 as *mut pgutErrorData;
    let mut tmp___0: *mut pgutErrorData = 0 as *mut pgutErrorData;
    tmp = __errno_location();
    save_errno = *tmp;
    tmp___0 = getErrorData();
    edata___0 = tmp___0;
    (*edata___0).elevel = elevel;
    (*edata___0).save_errno = save_errno;
    if elevel >= 20 as libc::c_int {
        (*edata___0).code = 1 as libc::c_int;
    } else {
        (*edata___0).code = 0 as libc::c_int;
    }
    if !((*edata___0).msg.data).is_null() {
        resetPQExpBuffer(&mut (*edata___0).msg);
    } else {
        initPQExpBuffer(&mut (*edata___0).msg);
    }
    if !((*edata___0).detail.data).is_null() {
        resetPQExpBuffer(&mut (*edata___0).detail);
    } else {
        initPQExpBuffer(&mut (*edata___0).detail);
    }
    return edata___0;
}
unsafe extern "C" fn trimStringBuffer(mut str: PQExpBuffer) {
    let mut tmp: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    while (*str).len > 0 as libc::c_ulong {
        tmp = __ctype_b_loc();
        if *(*tmp)
            .offset(
                *((*str).data)
                    .offset(((*str).len).wrapping_sub(1 as libc::c_ulong) as isize)
                    as libc::c_uchar as libc::c_int as isize,
            ) as libc::c_int & 8192 as libc::c_int == 0
        {
            break;
        }
        (*str).len = ((*str).len).wrapping_sub(1);
        *((*str).data).offset((*str).len as isize) = '\u{0}' as i32 as libc::c_char;
    }
}
pub unsafe extern "C" fn elog(
    mut elevel: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::std::ffi::VaListImpl;
    let mut ok: bool = false;
    let mut len: size_t = 0;
    let mut edata___0: *mut pgutErrorData = 0 as *mut pgutErrorData;
    let mut tmp: bool = false;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    if elevel < pgut_abort_level {
        tmp = log_required(elevel, pgut_log_level);
        if !tmp {
            return;
        }
    }
    edata___0 = pgut_errinit(elevel);
    loop {
        args_0 = args.clone();
        ok = pgut_appendStringInfoVA(&mut (*edata___0).msg, fmt, args_0.as_va_list());
        if ok {
            break;
        }
    }
    len = strlen(fmt);
    if len > 2 as libc::c_ulong {
        tmp___1 = strcmp(
            fmt.offset(len as isize).offset(-(2 as libc::c_int as isize)),
            b": \0" as *const u8 as *const libc::c_char,
        );
        if tmp___1 == 0 as libc::c_int {
            tmp___0 = pg_strerror((*edata___0).save_errno);
            appendPQExpBufferStr(&mut (*edata___0).msg, tmp___0 as *const libc::c_char);
        }
    }
    trimStringBuffer(&mut (*edata___0).msg);
    pgut_errfinish(1 as libc::c_int);
}
pub unsafe extern "C" fn pgut_errstart(mut elevel: libc::c_int) -> bool {
    let mut tmp: bool = false;
    if elevel < pgut_abort_level {
        tmp = log_required(elevel, pgut_log_level);
        if !tmp {
            return 0 as libc::c_int != 0;
        }
    }
    pgut_errinit(elevel);
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn pgut_errfinish(mut dummy: libc::c_int, mut args: ...) {
    let mut edata___0: *mut pgutErrorData = 0 as *mut pgutErrorData;
    let mut tmp: *mut pgutErrorData = 0 as *mut pgutErrorData;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: bool = false;
    tmp = getErrorData();
    edata___0 = tmp;
    tmp___1 = log_required((*edata___0).elevel, pgut_log_level);
    if tmp___1 {
        if !((*edata___0).msg.data).is_null() {
            tmp___0 = (*edata___0).msg.data as *const libc::c_char;
        } else {
            tmp___0 = b"unknown\0" as *const u8 as *const libc::c_char;
        }
        pgut_error(
            (*edata___0).elevel,
            (*edata___0).code,
            tmp___0,
            (*edata___0).detail.data as *const libc::c_char,
        );
    }
    if pgut_abort_level <= (*edata___0).elevel {
        if (*edata___0).elevel <= 22 as libc::c_int {
            in_cleanup = 1 as libc::c_int != 0;
            exit_or_abort((*edata___0).code, (*edata___0).elevel);
        }
    }
}
pub unsafe extern "C" fn pgut_error(
    mut elevel: libc::c_int,
    mut code: libc::c_int,
    mut msg: *const libc::c_char,
    mut detail: *const libc::c_char,
) {
    let mut tag: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    tmp = format_elevel(elevel);
    tag = tmp;
    if !detail.is_null() {
        if *detail.offset(0 as libc::c_int as isize) != 0 {
            pg_fprintf(
                stderr,
                b"%s: %s\nDETAIL: %s\n\0" as *const u8 as *const libc::c_char,
                tag,
                msg,
                detail,
            );
        } else {
            pg_fprintf(
                stderr,
                b"%s: %s\n\0" as *const u8 as *const libc::c_char,
                tag,
                msg,
            );
        }
    } else {
        pg_fprintf(stderr, b"%s: %s\n\0" as *const u8 as *const libc::c_char, tag, msg);
    }
    fflush(stderr);
}
pub unsafe extern "C" fn log_required(
    mut elevel: libc::c_int,
    mut log_min_level: libc::c_int,
) -> bool {
    let mut current_block_13: u64;
    if elevel == 15 as libc::c_int {
        current_block_13 = 12910196309224966777;
    } else if elevel == 16 as libc::c_int {
        current_block_13 = 12910196309224966777;
    } else {
        if log_min_level == 15 as libc::c_int {
            if elevel >= 21 as libc::c_int {
                return 1 as libc::c_int != 0;
            }
        } else if elevel >= log_min_level {
            return 1 as libc::c_int != 0
        }
        current_block_13 = 10599921512955367680;
    }
    match current_block_13 {
        12910196309224966777 => {
            if log_min_level == 15 as libc::c_int {
                return 1 as libc::c_int != 0
            } else {
                if log_min_level <= 20 as libc::c_int {
                    return 1 as libc::c_int != 0;
                }
            }
        }
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn format_elevel(mut elevel: libc::c_int) -> *const libc::c_char {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: bool = false;
    match elevel {
        14 | 13 | 12 | 11 | 10 => return b"DEBUG\0" as *const u8 as *const libc::c_char,
        15 => return b"LOG\0" as *const u8 as *const libc::c_char,
        17 => return b"INFO\0" as *const u8 as *const libc::c_char,
        18 => return b"NOTICE\0" as *const u8 as *const libc::c_char,
        19 => return b"WARNING\0" as *const u8 as *const libc::c_char,
        20 | 16 => return b"ERROR\0" as *const u8 as *const libc::c_char,
        21 => return b"FATAL\0" as *const u8 as *const libc::c_char,
        22 => return b"PANIC\0" as *const u8 as *const libc::c_char,
        _ => {
            tmp___1 = pgut_errstart(20 as libc::c_int);
            if tmp___1 {
                tmp = errmsg(
                    b"invalid elevel: %d\0" as *const u8 as *const libc::c_char,
                    elevel,
                );
                tmp___0 = errcode(22 as libc::c_int);
                pgut_errfinish(tmp___0, tmp);
            }
            return b"\0" as *const u8 as *const libc::c_char;
        }
    };
}
pub unsafe extern "C" fn parse_elevel(mut value: *const libc::c_char) -> libc::c_int {
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
    let mut tmp___9: bool = false;
    tmp___6 = pg_strcasecmp(value, b"DEBUG\0" as *const u8 as *const libc::c_char);
    if tmp___6 == 0 as libc::c_int {
        return 13 as libc::c_int
    } else {
        tmp___5 = pg_strcasecmp(value, b"INFO\0" as *const u8 as *const libc::c_char);
        if tmp___5 == 0 as libc::c_int {
            return 17 as libc::c_int
        } else {
            tmp___4 = pg_strcasecmp(
                value,
                b"NOTICE\0" as *const u8 as *const libc::c_char,
            );
            if tmp___4 == 0 as libc::c_int {
                return 18 as libc::c_int
            } else {
                tmp___3 = pg_strcasecmp(
                    value,
                    b"LOG\0" as *const u8 as *const libc::c_char,
                );
                if tmp___3 == 0 as libc::c_int {
                    return 15 as libc::c_int
                } else {
                    tmp___2 = pg_strcasecmp(
                        value,
                        b"WARNING\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___2 == 0 as libc::c_int {
                        return 19 as libc::c_int
                    } else {
                        tmp___1 = pg_strcasecmp(
                            value,
                            b"ERROR\0" as *const u8 as *const libc::c_char,
                        );
                        if tmp___1 == 0 as libc::c_int {
                            return 20 as libc::c_int
                        } else {
                            tmp___0 = pg_strcasecmp(
                                value,
                                b"FATAL\0" as *const u8 as *const libc::c_char,
                            );
                            if tmp___0 == 0 as libc::c_int {
                                return 21 as libc::c_int
                            } else {
                                tmp = pg_strcasecmp(
                                    value,
                                    b"PANIC\0" as *const u8 as *const libc::c_char,
                                );
                                if tmp == 0 as libc::c_int {
                                    return 22 as libc::c_int;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    tmp___9 = pgut_errstart(20 as libc::c_int);
    if tmp___9 {
        tmp___7 = errmsg(
            b"invalid elevel: %s\0" as *const u8 as *const libc::c_char,
            value,
        );
        tmp___8 = errcode(22 as libc::c_int);
        pgut_errfinish(tmp___8, tmp___7);
    }
    return 20 as libc::c_int;
}
pub unsafe extern "C" fn errcode(mut sqlerrcode: libc::c_int) -> libc::c_int {
    let mut edata___0: *mut pgutErrorData = 0 as *mut pgutErrorData;
    let mut tmp: *mut pgutErrorData = 0 as *mut pgutErrorData;
    tmp = getErrorData();
    edata___0 = tmp;
    (*edata___0).code = sqlerrcode;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn errcode_errno() -> libc::c_int {
    let mut edata___0: *mut pgutErrorData = 0 as *mut pgutErrorData;
    let mut tmp: *mut pgutErrorData = 0 as *mut pgutErrorData;
    tmp = getErrorData();
    edata___0 = tmp;
    (*edata___0).code = (*edata___0).save_errno;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn errmsg(
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut edata___0: *mut pgutErrorData = 0 as *mut pgutErrorData;
    let mut tmp: *mut pgutErrorData = 0 as *mut pgutErrorData;
    let mut args_0: ::std::ffi::VaListImpl;
    let mut len: size_t = 0;
    let mut ok: bool = false;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    tmp = getErrorData();
    edata___0 = tmp;
    loop {
        args_0 = args.clone();
        ok = pgut_appendStringInfoVA(&mut (*edata___0).msg, fmt, args_0.as_va_list());
        if ok {
            break;
        }
    }
    len = strlen(fmt);
    if len > 2 as libc::c_ulong {
        tmp___1 = strcmp(
            fmt.offset(len as isize).offset(-(2 as libc::c_int as isize)),
            b": \0" as *const u8 as *const libc::c_char,
        );
        if tmp___1 == 0 as libc::c_int {
            tmp___0 = pg_strerror((*edata___0).save_errno);
            appendPQExpBufferStr(&mut (*edata___0).msg, tmp___0 as *const libc::c_char);
        }
    }
    trimStringBuffer(&mut (*edata___0).msg);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn errdetail(
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut edata___0: *mut pgutErrorData = 0 as *mut pgutErrorData;
    let mut tmp: *mut pgutErrorData = 0 as *mut pgutErrorData;
    let mut args_0: ::std::ffi::VaListImpl;
    let mut ok: bool = false;
    tmp = getErrorData();
    edata___0 = tmp;
    loop {
        args_0 = args.clone();
        ok = pgut_appendStringInfoVA(&mut (*edata___0).detail, fmt, args_0.as_va_list());
        if ok {
            break;
        }
    }
    trimStringBuffer(&mut (*edata___0).detail);
    return 0 as libc::c_int;
}
unsafe extern "C" fn on_before_exec(mut conn: *mut pgutConn) {
    let mut old: *mut PGcancel = 0 as *mut PGcancel;
    if in_cleanup {
        return;
    }
    old = (*conn).cancel;
    (*conn).cancel = 0 as *mut libc::c_void as *mut PGcancel;
    if old as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        PQfreeCancel(old);
    }
    (*conn).cancel = PQgetCancel((*conn).conn);
}
unsafe extern "C" fn on_after_exec(mut conn: *mut pgutConn) {
    let mut old: *mut PGcancel = 0 as *mut PGcancel;
    if in_cleanup {
        return;
    }
    old = (*conn).cancel;
    (*conn).cancel = 0 as *mut libc::c_void as *mut PGcancel;
    if old as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        PQfreeCancel(old);
    }
}
unsafe extern "C" fn on_interrupt() {
    let mut c: *mut pgutConn = 0 as *mut pgutConn;
    let mut save_errno: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    tmp = __errno_location();
    save_errno = *tmp;
    interrupted = 1 as libc::c_int != 0;
    if in_cleanup {
        return;
    }
    pthread_mutex_lock(&mut pgut_conn_mutex);
    c = pgut_connections;
    while !c.is_null() {
        if (*c).cancel as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            tmp___0 = PQcancel(
                (*c).cancel,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                    as libc::c_int,
            );
            if tmp___0 != 0 {
                elog(
                    19 as libc::c_int,
                    b"Cancel request sent\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        c = (*c).next;
    }
    pthread_mutex_unlock(&mut pgut_conn_mutex);
    tmp___1 = __errno_location();
    *tmp___1 = save_errno;
}
static mut pgut_atexit_stack: *mut pgut_atexit_item = 0 as *const libc::c_void
    as *mut libc::c_void as *mut pgut_atexit_item;
pub unsafe extern "C" fn pgut_atexit_push(
    mut callback: Option::<unsafe extern "C" fn(bool, *mut libc::c_void) -> ()>,
    mut userdata: *mut libc::c_void,
) {
    let mut item: *mut pgut_atexit_item = 0 as *mut pgut_atexit_item;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = pgut_malloc(::std::mem::size_of::<pgut_atexit_item>() as libc::c_ulong);
    item = tmp as *mut pgut_atexit_item;
    (*item).callback = callback;
    (*item).userdata = userdata;
    (*item).next = pgut_atexit_stack;
    pgut_atexit_stack = item;
}
pub unsafe extern "C" fn pgut_atexit_pop(
    mut callback: Option::<unsafe extern "C" fn(bool, *mut libc::c_void) -> ()>,
    mut userdata: *mut libc::c_void,
) {
    let mut item: *mut pgut_atexit_item = 0 as *mut pgut_atexit_item;
    let mut prev: *mut *mut pgut_atexit_item = 0 as *mut *mut pgut_atexit_item;
    item = pgut_atexit_stack;
    prev = &mut pgut_atexit_stack;
    while !item.is_null() {
        if ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(bool, *mut libc::c_void) -> ()>,
            libc::c_ulong,
        >((*item).callback)
            == ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(bool, *mut libc::c_void) -> ()>,
                libc::c_ulong,
            >(callback)
        {
            if (*item).userdata as libc::c_ulong == userdata as libc::c_ulong {
                *prev = (*item).next;
                free(item as *mut libc::c_void);
                break;
            }
        }
        prev = &mut (*item).next;
        item = (*item).next;
    }
}
unsafe extern "C" fn call_atexit_callbacks(mut fatal: bool) {
    let mut item: *mut pgut_atexit_item = 0 as *mut pgut_atexit_item;
    item = pgut_atexit_stack;
    while !item.is_null() {
        (Some(((*item).callback).expect("non-null function pointer")))
            .expect("non-null function pointer")(fatal, (*item).userdata);
        item = (*item).next;
    }
}
unsafe extern "C" fn on_cleanup() {
    in_cleanup = 1 as libc::c_int != 0;
    interrupted = 0 as libc::c_int != 0;
    call_atexit_callbacks(0 as libc::c_int != 0);
    pgut_disconnect_all();
}
unsafe extern "C" fn exit_or_abort(mut exitcode: libc::c_int, mut elevel: libc::c_int) {
    if in_cleanup {
        if 21 as libc::c_int > elevel {
            call_atexit_callbacks(1 as libc::c_int != 0);
            exit(exitcode);
        }
    }
    if elevel >= 21 as libc::c_int {
        if elevel <= 22 as libc::c_int {
            call_atexit_callbacks(1 as libc::c_int != 0);
            abort();
        } else {
            exit(exitcode);
        }
    } else {
        exit(exitcode);
    };
}
pub unsafe extern "C" fn pgut_appendStringInfoVA(
    mut str: PQExpBuffer,
    mut fmt: *const libc::c_char,
    mut args: ::std::ffi::VaList,
) -> bool {
    let mut avail: size_t = 0;
    let mut nprinted: libc::c_int = 0;
    avail = ((*str).maxlen).wrapping_sub((*str).len).wrapping_sub(1 as libc::c_ulong);
    nprinted = pg_vsnprintf(
        ((*str).data).offset((*str).len as isize),
        avail,
        fmt,
        args.as_va_list(),
    );
    if nprinted >= 0 as libc::c_int {
        if nprinted < avail as libc::c_int - 1 as libc::c_int {
            (*str)
                .len = ((*str).len as libc::c_ulong).wrapping_add(nprinted as size_t)
                as size_t as size_t;
            return 1 as libc::c_int != 0;
        }
    }
    enlargePQExpBuffer(str, (*str).maxlen);
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn appendStringInfoFile(
    mut str: PQExpBuffer,
    mut fp: *mut FILE,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: *mut libc::c_int = 0 as *mut libc::c_int;
    loop {
        if ((*str).maxlen).wrapping_sub((*str).len) < 2 as libc::c_ulong {
            tmp___1 = enlargePQExpBuffer(str, 1024 as libc::c_int as size_t);
            if tmp___1 == 0 as libc::c_int {
                tmp = __errno_location();
                tmp___0 = 12 as libc::c_int;
                *tmp = tmp___0;
                return tmp___0;
            }
        }
        tmp___2 = fread(
            ((*str).data).offset((*str).len as isize) as *mut libc::c_void,
            1 as libc::c_int as size_t,
            ((*str).maxlen).wrapping_sub((*str).len).wrapping_sub(1 as libc::c_ulong),
            fp,
        );
        rc = tmp___2 as libc::c_int;
        if rc == 0 as libc::c_int {
            break;
        }
        if rc > 0 as libc::c_int {
            (*str)
                .len = ((*str).len as libc::c_ulong).wrapping_add(rc as size_t) as size_t
                as size_t;
            *((*str).data).offset((*str).len as isize) = '\u{0}' as i32 as libc::c_char;
        } else {
            tmp___4 = ferror(fp);
            if tmp___4 != 0 {
                tmp___5 = __errno_location();
                if *tmp___5 != 4 as libc::c_int {
                    tmp___3 = __errno_location();
                    return *tmp___3;
                }
            }
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn appendStringInfoFd(
    mut str: PQExpBuffer,
    mut fd: libc::c_int,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: ssize_t = 0;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___4: *mut libc::c_int = 0 as *mut libc::c_int;
    loop {
        if ((*str).maxlen).wrapping_sub((*str).len) < 2 as libc::c_ulong {
            tmp___1 = enlargePQExpBuffer(str, 1024 as libc::c_int as size_t);
            if tmp___1 == 0 as libc::c_int {
                tmp = __errno_location();
                tmp___0 = 12 as libc::c_int;
                *tmp = tmp___0;
                return tmp___0;
            }
        }
        tmp___2 = read(
            fd,
            ((*str).data).offset((*str).len as isize) as *mut libc::c_void,
            ((*str).maxlen).wrapping_sub((*str).len).wrapping_sub(1 as libc::c_ulong),
        );
        rc = tmp___2 as libc::c_int;
        if rc == 0 as libc::c_int {
            break;
        }
        if rc > 0 as libc::c_int {
            (*str)
                .len = ((*str).len as libc::c_ulong).wrapping_add(rc as size_t) as size_t
                as size_t;
            *((*str).data).offset((*str).len as isize) = '\u{0}' as i32 as libc::c_char;
        } else {
            tmp___4 = __errno_location();
            if *tmp___4 != 4 as libc::c_int {
                tmp___3 = __errno_location();
                return *tmp___3;
            }
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn pgut_malloc(mut size: size_t) -> *mut libc::c_void {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: bool = false;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp___2 = malloc(size);
    ret = tmp___2 as *mut libc::c_char;
    if ret as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___1 = pgut_errstart(21 as libc::c_int);
        if tmp___1 {
            tmp = errmsg(
                b"could not allocate memory (%lu bytes): \0" as *const u8
                    as *const libc::c_char,
                size,
            );
            tmp___0 = errcode_errno();
            pgut_errfinish(tmp___0, tmp);
        }
    }
    return ret as *mut libc::c_void;
}
pub unsafe extern "C" fn pgut_realloc(
    mut p: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: bool = false;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp___2 = realloc(p, size);
    ret = tmp___2 as *mut libc::c_char;
    if ret as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___1 = pgut_errstart(21 as libc::c_int);
        if tmp___1 {
            tmp = errmsg(
                b"could not re-allocate memory (%lu bytes): \0" as *const u8
                    as *const libc::c_char,
                size,
            );
            tmp___0 = errcode_errno();
            pgut_errfinish(tmp___0, tmp);
        }
    }
    return ret as *mut libc::c_void;
}
pub unsafe extern "C" fn pgut_strdup(mut str: *const libc::c_char) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: bool = false;
    if str as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    ret = strdup(str);
    if ret as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___1 = pgut_errstart(21 as libc::c_int);
        if tmp___1 {
            tmp = errmsg(
                b"could not duplicate string \"%s\": \0" as *const u8
                    as *const libc::c_char,
                str,
            );
            tmp___0 = errcode_errno();
            pgut_errfinish(tmp___0, tmp);
        }
    }
    return ret;
}
pub unsafe extern "C" fn strdup_with_len(
    mut str: *const libc::c_char,
    mut len: size_t,
) -> *mut libc::c_char {
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    if str as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    tmp = pgut_malloc(len.wrapping_add(1 as libc::c_ulong));
    r = tmp as *mut libc::c_char;
    libc::memcpy(r as *mut libc::c_void, str as *const libc::c_void, len as _);
    *r.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    return r;
}
pub unsafe extern "C" fn strdup_trim(mut str: *const libc::c_char) -> *mut libc::c_char {
    let mut len: size_t = 0;
    let mut tmp: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut tmp___0: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    if str as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    loop {
        tmp = __ctype_b_loc();
        if *(*tmp)
            .offset(
                *str.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                    as isize,
            ) as libc::c_int & 8192 as libc::c_int == 0
        {
            break;
        }
        str = str.offset(1);
    }
    len = strlen(str);
    while len > 0 as libc::c_ulong {
        tmp___0 = __ctype_b_loc();
        if *(*tmp___0)
            .offset(
                *str.offset(len.wrapping_sub(1 as libc::c_ulong) as isize)
                    as libc::c_uchar as libc::c_int as isize,
            ) as libc::c_int & 8192 as libc::c_int == 0
        {
            break;
        }
        len = len.wrapping_sub(1);
    }
    tmp___1 = strdup_with_len(str, len);
    return tmp___1;
}
pub unsafe extern "C" fn pgut_fopen(
    mut path: *const libc::c_char,
    mut omode: *const libc::c_char,
) -> *mut FILE {
    let mut current_block: u64;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut missing_ok: bool = false;
    let mut mode: [libc::c_char; 16] = [0; 16];
    let mut dir: [libc::c_char; 1024] = [0; 1024];
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: bool = false;
    missing_ok = 0 as libc::c_int != 0;
    strlcpy(
        mode.as_mut_ptr(),
        omode,
        (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    if mode[0 as libc::c_int as usize] as libc::c_int == 82 as libc::c_int {
        mode[0 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
        missing_ok = 1 as libc::c_int != 0;
    }
    loop {
        fp = fopen(path, mode.as_mut_ptr() as *const libc::c_char);
        if !(fp as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
            current_block = 17478428563724192186;
            break;
        }
        tmp = __errno_location();
        if !(*tmp == 2 as libc::c_int) {
            current_block = 12124785117276362961;
            break;
        }
        if missing_ok {
            return 0 as *mut libc::c_void as *mut FILE;
        }
        if mode[0 as libc::c_int as usize] as libc::c_int == 119 as libc::c_int {
            strlcpy(dir.as_mut_ptr(), path, 1024 as libc::c_int as size_t);
            get_parent_directory(dir.as_mut_ptr());
            pgut_mkdir(dir.as_mut_ptr() as *const libc::c_char);
        } else {
            if !(mode[0 as libc::c_int as usize] as libc::c_int == 97 as libc::c_int) {
                current_block = 12124785117276362961;
                break;
            }
            strlcpy(dir.as_mut_ptr(), path, 1024 as libc::c_int as size_t);
            get_parent_directory(dir.as_mut_ptr());
            pgut_mkdir(dir.as_mut_ptr() as *const libc::c_char);
        }
    }
    match current_block {
        12124785117276362961 => {
            tmp___2 = pgut_errstart(20 as libc::c_int);
            if tmp___2 {
                tmp___0 = errmsg(
                    b"could not open file \"%s\": \0" as *const u8
                        as *const libc::c_char,
                    path,
                );
                tmp___1 = errcode_errno();
                pgut_errfinish(tmp___1, tmp___0);
            }
        }
        _ => {}
    }
    return fp;
}
pub unsafe extern "C" fn pgut_mkdir(mut dirpath: *const libc::c_char) -> bool {
    let mut sb: stat = stat {
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
    let mut first: libc::c_int = 0;
    let mut last: libc::c_int = 0;
    let mut retval: libc::c_int = 0;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: bool = false;
    path = pgut_strdup(dirpath);
    p = path;
    retval = 0 as libc::c_int;
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == 47 as libc::c_int {
        p = p.offset(1);
    }
    first = 1 as libc::c_int;
    last = 0 as libc::c_int;
    let mut current_block_34: u64;
    's_57: while last == 0 {
        if *p.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
            last = 1 as libc::c_int;
            current_block_34 = 17407779659766490442;
        } else if *p.offset(0 as libc::c_int as isize) as libc::c_int
                != 47 as libc::c_int
            {
            current_block_34 = 4840823220790654551;
        } else {
            current_block_34 = 17407779659766490442;
        }
        match current_block_34 {
            17407779659766490442 => {
                *p = '\u{0}' as i32 as libc::c_char;
                if last == 0 {
                    if *p.offset(1 as libc::c_int as isize) as libc::c_int
                        == 0 as libc::c_int
                    {
                        last = 1 as libc::c_int;
                    }
                }
                if first != 0 {
                    first = 0 as libc::c_int;
                }
                loop {
                    tmp___3 = stat(path as *const libc::c_char, &mut sb as *mut stat);
                    if tmp___3 == 0 as libc::c_int {
                        if !(sb.st_mode & 61440 as libc::c_uint == 16384 as libc::c_uint)
                        {
                            current_block_34 = 9828876828309294594;
                            break;
                        } else {
                            current_block_34 = 14832935472441733737;
                            break;
                        }
                    } else {
                        tmp___2 = mkdir(
                            path as *const libc::c_char,
                            448 as libc::c_int as __mode_t,
                        );
                        if !(tmp___2 < 0 as libc::c_int) {
                            current_block_34 = 14832935472441733737;
                            break;
                        }
                        tmp___1 = __errno_location();
                        if *tmp___1 == 17 as libc::c_int {
                            continue;
                        }
                        retval = 1 as libc::c_int;
                        break 's_57;
                    }
                }
                match current_block_34 {
                    14832935472441733737 => {
                        if last == 0 {
                            *p = '/' as i32 as libc::c_char;
                        }
                    }
                    _ => {
                        if last != 0 {
                            tmp = __errno_location();
                            *tmp = 17 as libc::c_int;
                        } else {
                            tmp___0 = __errno_location();
                            *tmp___0 = 20 as libc::c_int;
                        }
                        retval = 1 as libc::c_int;
                        break;
                    }
                }
            }
            _ => {}
        }
        p = p.offset(1);
    }
    free(path as *mut libc::c_void);
    if retval == 0 as libc::c_int {
        tmp___6 = pgut_errstart(20 as libc::c_int);
        if tmp___6 {
            tmp___4 = errmsg(
                b"could not create directory \"%s\": \0" as *const u8
                    as *const libc::c_char,
                dirpath,
            );
            tmp___5 = errcode_errno();
            pgut_errfinish(tmp___5, tmp___4);
        }
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn wait_for_socket(
    mut sock: libc::c_int,
    mut timeout: *mut timeval,
) -> libc::c_int {
    let mut fds: fd_set = fd_set { fds_bits: [0; 16] };
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let mut __d: libc::c_long = 0;
    let mut tmp___1: libc::c_long = 0;
    let mut tmp___2: libc::c_int = 0;
    let fresh14 = &mut __d0;
    let fresh15;
    let fresh16 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh17 = &mut __d1;
    let fresh18;
    let fresh19 = &mut *(fds.fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh14,
        fresh16) => fresh15, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh17,
        fresh19) => fresh18, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh14, fresh16, fresh15);
    c2rust_asm_casts::AsmCast::cast_out(fresh17, fresh19, fresh18);
    __d = sock as libc::c_long;
    tmp___1 = __fdelt_chk(__d);
    fds.fds_bits[tmp___1 as usize]
        |= ((1 as libc::c_ulong)
            << sock
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask;
    tmp___2 = wait_for_sockets(sock + 1 as libc::c_int, &mut fds, timeout);
    return tmp___2;
}
pub unsafe extern "C" fn wait_for_sockets(
    mut nfds: libc::c_int,
    mut fds: *mut fd_set,
    mut timeout: *mut timeval,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: bool = false;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    loop {
        i = select(
            nfds,
            fds,
            0 as *mut libc::c_void as *mut fd_set,
            0 as *mut libc::c_void as *mut fd_set,
            timeout,
        );
        if i < 0 as libc::c_int {
            CHECK_FOR_INTERRUPTS();
            tmp___2 = __errno_location();
            if *tmp___2 != 4 as libc::c_int {
                tmp___1 = pgut_errstart(20 as libc::c_int);
                if tmp___1 {
                    tmp = errmsg(
                        b"select failed: \0" as *const u8 as *const libc::c_char,
                    );
                    tmp___0 = errcode_errno();
                    pgut_errfinish(tmp___0, tmp);
                }
                return -(1 as libc::c_int);
            }
        } else {
            return i
        }
    };
}
unsafe extern "C" fn handle_sigint(mut postgres_signal_arg: libc::c_int) {
    on_interrupt();
}
unsafe extern "C" fn init_cancel_handler() {
    pqsignal(
        2 as libc::c_int,
        Some(handle_sigint as unsafe extern "C" fn(libc::c_int) -> ()),
    );
}
pub static mut dbname: *mut libc::c_char = 0 as *const libc::c_void as *mut libc::c_void
    as *mut libc::c_char;
pub static mut host: *mut libc::c_char = 0 as *const libc::c_void as *mut libc::c_void
    as *mut libc::c_char;
pub static mut port: *mut libc::c_char = 0 as *const libc::c_void as *mut libc::c_void
    as *mut libc::c_char;
pub static mut username: *mut libc::c_char = 0 as *const libc::c_void
    as *mut libc::c_void as *mut libc::c_char;
pub static mut password: *mut libc::c_char = 0 as *const libc::c_void
    as *mut libc::c_void as *mut libc::c_char;
pub static mut prompt_password: YesNo = DEFAULT;
pub static mut connection: *mut PGconn = 0 as *const libc::c_void as *mut libc::c_void
    as *mut PGconn;
pub static mut conn2: *mut PGconn = 0 as *const libc::c_void as *mut libc::c_void
    as *mut PGconn;
pub static mut workers: worker_conns = {
    let mut init = worker_conns {
        max_num_workers: 0 as libc::c_int,
        num_workers: 0 as libc::c_int,
        conns: 0 as *const libc::c_void as *mut libc::c_void as *mut *mut PGconn,
    };
    init
};
pub unsafe extern "C" fn setup_workers(mut num_workers: libc::c_int) {
    let mut buf: PQExpBufferData = PQExpBufferData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
    };
    let mut i: libc::c_int = 0;
    let mut conn: *mut PGconn = 0 as *mut PGconn;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: ConnStatusType = CONNECTION_OK;
    let mut tmp___2: libc::c_int = 0;
    elog(
        13 as libc::c_int,
        b"In setup_workers(), target num_workers = %d\0" as *const u8
            as *const libc::c_char,
        num_workers,
    );
    if num_workers > 1 as libc::c_int {
        if num_workers > workers.num_workers {
            initPQExpBuffer(&mut buf);
            if !dbname.is_null() {
                if *dbname.offset(0 as libc::c_int as isize) != 0 {
                    appendPQExpBuffer(
                        &mut buf as *mut PQExpBufferData,
                        b"dbname=%s \0" as *const u8 as *const libc::c_char,
                        dbname,
                    );
                }
            }
            if !host.is_null() {
                if *host.offset(0 as libc::c_int as isize) != 0 {
                    appendPQExpBuffer(
                        &mut buf as *mut PQExpBufferData,
                        b"host=%s \0" as *const u8 as *const libc::c_char,
                        host,
                    );
                }
            }
            if !port.is_null() {
                if *port.offset(0 as libc::c_int as isize) != 0 {
                    appendPQExpBuffer(
                        &mut buf as *mut PQExpBufferData,
                        b"port=%s \0" as *const u8 as *const libc::c_char,
                        port,
                    );
                }
            }
            if !username.is_null() {
                if *username.offset(0 as libc::c_int as isize) != 0 {
                    appendPQExpBuffer(
                        &mut buf as *mut PQExpBufferData,
                        b"user=%s \0" as *const u8 as *const libc::c_char,
                        username,
                    );
                }
            }
            if !password.is_null() {
                if *password.offset(0 as libc::c_int as isize) != 0 {
                    appendPQExpBuffer(
                        &mut buf as *mut PQExpBufferData,
                        b"password=%s \0" as *const u8 as *const libc::c_char,
                        password,
                    );
                }
            }
            if workers.conns as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong
            {
                elog(
                    18 as libc::c_int,
                    b"Setting up workers.conns\0" as *const u8 as *const libc::c_char,
                );
                tmp = pgut_malloc(
                    (::std::mem::size_of::<*mut PGconn>() as libc::c_ulong)
                        .wrapping_mul(num_workers as libc::c_ulong),
                );
                workers.conns = tmp as *mut *mut PGconn;
            } else {
                elog(
                    20 as libc::c_int,
                    b"TODO: Implement pool resizing.\0" as *const u8
                        as *const libc::c_char,
                );
            }
            i = 0 as libc::c_int;
            while i < num_workers {
                elog(
                    13 as libc::c_int,
                    b"Setting up worker conn %d\0" as *const u8 as *const libc::c_char,
                    i,
                );
                conn = PQconnectdb(buf.data as *const libc::c_char);
                tmp___1 = PQstatus(conn as *const PGconn);
                if tmp___1 as libc::c_uint == 0 as libc::c_uint {
                    let ref mut fresh20 = *(workers.conns).offset(i as isize);
                    *fresh20 = conn;
                    pgut_command(
                        conn,
                        b"SET search_path TO pg_catalog, pg_temp, public\0" as *const u8
                            as *const libc::c_char,
                        0 as libc::c_int,
                        0 as *mut libc::c_void as *mut *const libc::c_char,
                    );
                    tmp___2 = PQsetnonblocking(
                        *(workers.conns).offset(i as isize),
                        1 as libc::c_int,
                    );
                    if tmp___2 != 0 {
                        elog(
                            20 as libc::c_int,
                            b"Unable to set worker connection %d non-blocking.\0"
                                as *const u8 as *const libc::c_char,
                            i,
                        );
                    }
                    i += 1;
                } else {
                    tmp___0 = PQerrorMessage(conn as *const PGconn);
                    elog(
                        19 as libc::c_int,
                        b"Unable to set up worker conn #%d: %s\0" as *const u8
                            as *const libc::c_char,
                        i,
                        tmp___0,
                    );
                    break;
                }
            }
            workers.num_workers = i;
            termPQExpBuffer(&mut buf);
        }
    }
}
pub unsafe extern "C" fn disconnect_workers() {
    let mut i: libc::c_int = 0;
    if workers.num_workers == 0 {
        elog(
            13 as libc::c_int,
            b"No workers to disconnect.\0" as *const u8 as *const libc::c_char,
        );
    } else {
        i = 0 as libc::c_int;
        while i < workers.num_workers {
            if !(*(workers.conns).offset(i as isize)).is_null() {
                elog(
                    13 as libc::c_int,
                    b"Disconnecting worker %d.\0" as *const u8 as *const libc::c_char,
                    i,
                );
                PQfinish(*(workers.conns).offset(i as isize));
                let ref mut fresh21 = *(workers.conns).offset(i as isize);
                *fresh21 = 0 as *mut libc::c_void as *mut PGconn;
            } else {
                elog(
                    18 as libc::c_int,
                    b"Worker %d already disconnected?\0" as *const u8
                        as *const libc::c_char,
                    i,
                );
            }
            i += 1;
        }
        workers.num_workers = 0 as libc::c_int;
        free(workers.conns as *mut libc::c_void);
        workers.conns = 0 as *mut libc::c_void as *mut *mut PGconn;
    };
}
pub unsafe extern "C" fn reconnect(mut elevel: libc::c_int) {
    let mut buf: PQExpBufferData = PQExpBufferData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
    };
    let mut new_password: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: libc::c_int = 0;
    disconnect();
    initPQExpBuffer(&mut buf);
    if !dbname.is_null() {
        if *dbname.offset(0 as libc::c_int as isize) != 0 {
            appendPQExpBuffer(
                &mut buf as *mut PQExpBufferData,
                b"dbname=%s \0" as *const u8 as *const libc::c_char,
                dbname,
            );
        }
    }
    if !host.is_null() {
        if *host.offset(0 as libc::c_int as isize) != 0 {
            appendPQExpBuffer(
                &mut buf as *mut PQExpBufferData,
                b"host=%s \0" as *const u8 as *const libc::c_char,
                host,
            );
        }
    }
    if !port.is_null() {
        if *port.offset(0 as libc::c_int as isize) != 0 {
            appendPQExpBuffer(
                &mut buf as *mut PQExpBufferData,
                b"port=%s \0" as *const u8 as *const libc::c_char,
                port,
            );
        }
    }
    if !username.is_null() {
        if *username.offset(0 as libc::c_int as isize) != 0 {
            appendPQExpBuffer(
                &mut buf as *mut PQExpBufferData,
                b"user=%s \0" as *const u8 as *const libc::c_char,
                username,
            );
        }
    }
    if !password.is_null() {
        if *password.offset(0 as libc::c_int as isize) != 0 {
            appendPQExpBuffer(
                &mut buf as *mut PQExpBufferData,
                b"password=%s \0" as *const u8 as *const libc::c_char,
                password,
            );
        }
    }
    connection = pgut_connect(buf.data as *const libc::c_char, prompt_password, elevel);
    conn2 = pgut_connect(buf.data as *const libc::c_char, prompt_password, elevel);
    if !connection.is_null() {
        new_password = PQpass(connection as *const PGconn);
        if !new_password.is_null() {
            if *new_password.offset(0 as libc::c_int as isize) != 0 {
                if password as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                    free(password as *mut libc::c_void);
                    password = pgut_strdup(new_password as *const libc::c_char);
                } else {
                    tmp = strcmp(
                        new_password as *const libc::c_char,
                        password as *const libc::c_char,
                    );
                    if tmp != 0 as libc::c_int {
                        free(password as *mut libc::c_void);
                        password = pgut_strdup(new_password as *const libc::c_char);
                    }
                }
            }
        }
    }
    termPQExpBuffer(&mut buf);
}
pub unsafe extern "C" fn disconnect() {
    if !connection.is_null() {
        pgut_disconnect(connection);
        connection = 0 as *mut libc::c_void as *mut PGconn;
    }
    if !conn2.is_null() {
        pgut_disconnect(conn2);
        conn2 = 0 as *mut libc::c_void as *mut PGconn;
    }
    disconnect_workers();
}
unsafe extern "C" fn option_from_env(mut options___0: *mut pgut_option) {
    let mut i: size_t = 0;
    let mut opt: *mut pgut_option = 0 as *mut pgut_option;
    let mut name: [libc::c_char; 256] = [0; 256];
    let mut j: size_t = 0;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut value: *const libc::c_char = 0 as *const libc::c_char;
    let mut __res: libc::c_int = 0;
    let mut tmp___0: *mut *const __int32_t = 0 as *mut *const __int32_t;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int as size_t;
    while !options___0.is_null() {
        if (*options___0.offset(i as isize)).type_0 == 0 {
            break;
        }
        opt = options___0.offset(i as isize);
        if !((*opt).source as libc::c_uint > 1 as libc::c_uint) {
            if !((*opt).allowed as libc::c_uint == 0 as libc::c_uint) {
                if !((*opt).allowed as libc::c_uint > 1 as libc::c_uint) {
                    s = (*opt).lname;
                    j = 0 as libc::c_int as size_t;
                    while *s != 0 {
                        if !(j
                            < (::std::mem::size_of::<[libc::c_char; 256]>()
                                as libc::c_ulong)
                                .wrapping_div(
                                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                )
                                .wrapping_sub(1 as libc::c_ulong))
                        {
                            break;
                        }
                        tmp___1 = strchr(
                            b"-_ \0" as *const u8 as *const libc::c_char,
                            *s as libc::c_int,
                        );
                        if !tmp___1.is_null() {
                            name[j as usize] = '_' as i32 as libc::c_char;
                        } else {
                            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                > 1 as libc::c_ulong
                            {
                                __res = toupper(*s as libc::c_int);
                            } else {
                                tmp___0 = __ctype_toupper_loc();
                                __res = *(*tmp___0).offset(*s as libc::c_int as isize);
                            }
                            name[j as usize] = __res as libc::c_char;
                        }
                        s = s.offset(1);
                        j = j.wrapping_add(1);
                    }
                    name[j as usize] = '\u{0}' as i32 as libc::c_char;
                    tmp___2 = getenv(name.as_mut_ptr() as *const libc::c_char);
                    value = tmp___2 as *const libc::c_char;
                    if value as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong
                    {
                        pgut_setopt(opt, value, SOURCE_ENV);
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
}
pub unsafe extern "C" fn pgut_keyeq(
    mut lhs: *const libc::c_char,
    mut rhs: *const libc::c_char,
) -> bool {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut __res: libc::c_int = 0;
    let mut tmp___1: *mut *const __int32_t = 0 as *mut *const __int32_t;
    let mut __res___0: libc::c_int = 0;
    let mut tmp___3: *mut *const __int32_t = 0 as *mut *const __int32_t;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: libc::c_int = 0;
    while *lhs != 0 {
        if *rhs == 0 {
            break;
        }
        tmp___4 = strchr(
            b"-_ \0" as *const u8 as *const libc::c_char,
            *lhs as libc::c_int,
        );
        if !tmp___4.is_null() {
            tmp = strchr(
                b"-_ \0" as *const u8 as *const libc::c_char,
                *rhs as libc::c_int,
            );
            if tmp.is_null() {
                return 0 as libc::c_int != 0;
            }
        } else {
            if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                > 1 as libc::c_ulong
            {
                __res = tolower(*lhs as libc::c_uchar as libc::c_int);
            } else {
                tmp___1 = __ctype_tolower_loc();
                __res = *(*tmp___1)
                    .offset(*lhs as libc::c_uchar as libc::c_int as isize);
            }
            if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                > 1 as libc::c_ulong
            {
                __res___0 = tolower(*rhs as libc::c_uchar as libc::c_int);
            } else {
                tmp___3 = __ctype_tolower_loc();
                __res___0 = *(*tmp___3)
                    .offset(*rhs as libc::c_uchar as libc::c_int as isize);
            }
            if __res != __res___0 {
                return 0 as libc::c_int != 0;
            }
        }
        lhs = lhs.offset(1);
        rhs = rhs.offset(1);
    }
    if *lhs as libc::c_int == 0 as libc::c_int {
        if *rhs as libc::c_int == 0 as libc::c_int {
            tmp___5 = 1 as libc::c_int;
        } else {
            tmp___5 = 0 as libc::c_int;
        }
    } else {
        tmp___5 = 0 as libc::c_int;
    }
    return tmp___5 != 0;
}
pub unsafe extern "C" fn pgut_setopt(
    mut opt: *mut pgut_option,
    mut optarg___0: *const libc::c_char,
    mut src: pgut_optsrc,
) {
    let mut current_block: u64;
    let mut message: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: bool = false;
    let mut tmp___0: bool = false;
    let mut tmp___1: bool = false;
    let mut tmp___2: bool = false;
    let mut tmp___3: bool = false;
    let mut tmp___4: bool = false;
    let mut value: bool = false;
    let mut tmp___5: bool = false;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: bool = false;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: bool = false;
    let mut tmp___12: libc::c_int = 0;
    let mut tmp___13: libc::c_int = 0;
    let mut tmp___14: bool = false;
    let mut tmp___15: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    if opt as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        pg_fprintf(
            stderr,
            b"Try \"%s --help\" for more information.\n\0" as *const u8
                as *const libc::c_char,
            PROGRAM_NAME,
        );
        exit(22 as libc::c_int);
    }
    if (*opt).source as libc::c_uint > src as libc::c_uint {
        return
    } else {
        if src as libc::c_uint >= 3 as libc::c_uint {
            if (*opt).source as libc::c_uint >= src as libc::c_uint {
                if (*opt).type_0 as libc::c_int != 108 as libc::c_int {
                    message = b"specified only once\0" as *const u8
                        as *const libc::c_char;
                    current_block = 479107131381816815;
                } else {
                    current_block = 2273208954694866844;
                }
            } else {
                current_block = 2273208954694866844;
            }
        } else {
            current_block = 2273208954694866844;
        }
        match current_block {
            2273208954694866844 => {
                (*opt).source = src;
                match (*opt).type_0 as libc::c_int {
                    66 | 98 => {
                        if optarg___0 as libc::c_ulong
                            == 0 as *mut libc::c_void as libc::c_ulong
                        {
                            *((*opt).var
                                as *mut bool) = (*opt).type_0 as libc::c_int
                                == 98 as libc::c_int;
                            return;
                        } else {
                            tmp = parse_bool(optarg___0, (*opt).var as *mut bool);
                            if tmp {
                                return;
                            }
                        }
                        message = b"a boolean\0" as *const u8 as *const libc::c_char;
                    }
                    102 => {
                        (Some(
                            (::std::mem::transmute::<
                                *mut libc::c_void,
                                Option::<
                                    unsafe extern "C" fn(
                                        *mut pgut_option,
                                        *const libc::c_char,
                                    ) -> (),
                                >,
                            >((*opt).var))
                                .expect("non-null function pointer"),
                        ))
                            .expect("non-null function pointer")(opt, optarg___0);
                        return;
                    }
                    105 => {
                        tmp___0 = parse_int32(optarg___0, (*opt).var as *mut int32);
                        if tmp___0 {
                            return;
                        }
                        message = b"a 32bit signed integer\0" as *const u8
                            as *const libc::c_char;
                    }
                    108 => {
                        message = b"a List\0" as *const u8 as *const libc::c_char;
                        simple_string_list_append(
                            (*opt).var as *mut SimpleStringList,
                            optarg___0,
                        );
                        return;
                    }
                    117 => {
                        tmp___1 = parse_uint32(optarg___0, (*opt).var as *mut uint32);
                        if tmp___1 {
                            return;
                        }
                        message = b"a 32bit unsigned integer\0" as *const u8
                            as *const libc::c_char;
                    }
                    73 => {
                        tmp___2 = parse_int64(optarg___0, (*opt).var as *mut int64);
                        if tmp___2 {
                            return;
                        }
                        message = b"a 64bit signed integer\0" as *const u8
                            as *const libc::c_char;
                    }
                    85 => {
                        tmp___3 = parse_uint64(optarg___0, (*opt).var as *mut uint64);
                        if tmp___3 {
                            return;
                        }
                        message = b"a 64bit unsigned integer\0" as *const u8
                            as *const libc::c_char;
                    }
                    115 => {
                        if (*opt).source as libc::c_uint != 0 as libc::c_uint {
                            free(
                                *((*opt).var as *mut *mut libc::c_char) as *mut libc::c_void,
                            );
                        }
                        let ref mut fresh22 = *((*opt).var as *mut *mut libc::c_char);
                        *fresh22 = pgut_strdup(optarg___0);
                        return;
                    }
                    116 => {
                        tmp___4 = parse_time(optarg___0, (*opt).var as *mut time_t);
                        if tmp___4 {
                            return;
                        }
                        message = b"a time\0" as *const u8 as *const libc::c_char;
                    }
                    89 | 121 => {
                        if optarg___0 as libc::c_ulong
                            == 0 as *mut libc::c_void as libc::c_ulong
                        {
                            if (*opt).type_0 as libc::c_int == 121 as libc::c_int {
                                *((*opt).var as *mut YesNo) = YES;
                            } else {
                                *((*opt).var as *mut YesNo) = NO;
                            }
                            return;
                        } else {
                            tmp___5 = parse_bool(optarg___0, &mut value);
                            if tmp___5 {
                                if value {
                                    *((*opt).var as *mut YesNo) = YES;
                                } else {
                                    *((*opt).var as *mut YesNo) = NO;
                                }
                                return;
                            }
                        }
                        message = b"a boolean\0" as *const u8 as *const libc::c_char;
                    }
                    _ => {
                        tmp___8 = pgut_errstart(20 as libc::c_int);
                        if tmp___8 {
                            tmp___6 = errmsg(
                                b"invalid option type: %c\0" as *const u8
                                    as *const libc::c_char,
                                (*opt).type_0 as libc::c_int,
                            );
                            tmp___7 = errcode(22 as libc::c_int);
                            pgut_errfinish(tmp___7, tmp___6);
                        }
                        return;
                    }
                }
            }
            _ => {}
        }
        tmp___15 = __ctype_b_loc();
        if *(*tmp___15).offset((*opt).sname as libc::c_int as isize) as libc::c_int
            & 16384 as libc::c_int != 0
        {
            tmp___11 = pgut_errstart(20 as libc::c_int);
            if tmp___11 {
                tmp___9 = errmsg(
                    b"option -%c, --%s should be %s: '%s'\0" as *const u8
                        as *const libc::c_char,
                    (*opt).sname as libc::c_int,
                    (*opt).lname,
                    message,
                    optarg___0,
                );
                tmp___10 = errcode(22 as libc::c_int);
                pgut_errfinish(tmp___10, tmp___9);
            }
        } else {
            tmp___14 = pgut_errstart(20 as libc::c_int);
            if tmp___14 {
                tmp___12 = errmsg(
                    b"option --%s should be %s: '%s'\0" as *const u8
                        as *const libc::c_char,
                    (*opt).lname,
                    message,
                    optarg___0,
                );
                tmp___13 = errcode(22 as libc::c_int);
                pgut_errfinish(tmp___13, tmp___12);
            }
        }
        return;
    };
}
pub unsafe extern "C" fn pgut_readopt(
    mut path: *const libc::c_char,
    mut options___0: *mut pgut_option,
    mut elevel: libc::c_int,
) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut key: [libc::c_char; 1024] = [0; 1024];
    let mut value: [libc::c_char; 1024] = [0; 1024];
    let mut i: size_t = 0;
    let mut tmp: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut opt: *mut pgut_option = 0 as *mut pgut_option;
    let mut tmp___0: bool = false;
    let mut tmp___1: bool = false;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    if options___0.is_null() {
        return;
    }
    fp = pgut_fopen(path, b"Rt\0" as *const u8 as *const libc::c_char);
    if fp as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    loop {
        tmp___2 = fgets(
            buf.as_mut_ptr(),
            (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                as libc::c_int,
            fp,
        );
        if tmp___2.is_null() {
            break;
        }
        i = strlen(buf.as_mut_ptr() as *const libc::c_char);
        while i > 0 as libc::c_ulong {
            tmp = __ctype_b_loc();
            if *(*tmp)
                .offset(
                    buf[i.wrapping_sub(1 as libc::c_ulong) as usize] as libc::c_uchar
                        as libc::c_int as isize,
                ) as libc::c_int & 8192 as libc::c_int == 0
            {
                break;
            }
            buf[i.wrapping_sub(1 as libc::c_ulong)
                as usize] = '\u{0}' as i32 as libc::c_char;
            i = i.wrapping_sub(1);
        }
        tmp___1 = parse_pair(
            buf.as_mut_ptr() as *const libc::c_char,
            key.as_mut_ptr(),
            value.as_mut_ptr(),
        );
        if tmp___1 {
            i = 0 as libc::c_int as size_t;
            while (*options___0.offset(i as isize)).type_0 != 0 {
                opt = options___0.offset(i as isize);
                tmp___0 = pgut_keyeq(
                    key.as_mut_ptr() as *const libc::c_char,
                    (*opt).lname,
                );
                if tmp___0 {
                    if (*opt).allowed as libc::c_uint == 0 as libc::c_uint {
                        elog(
                            elevel,
                            b"option %s cannot specified in file\0" as *const u8
                                as *const libc::c_char,
                            (*opt).lname,
                        );
                    } else if (*opt).allowed as libc::c_uint > 2 as libc::c_uint {
                        elog(
                            elevel,
                            b"option %s cannot specified in file\0" as *const u8
                                as *const libc::c_char,
                            (*opt).lname,
                        );
                    } else if (*opt).source as libc::c_uint <= 2 as libc::c_uint {
                        pgut_setopt(
                            opt,
                            value.as_mut_ptr() as *const libc::c_char,
                            SOURCE_FILE,
                        );
                    }
                    break;
                } else {
                    i = i.wrapping_add(1);
                }
            }
            if (*options___0.offset(i as isize)).type_0 == 0 {
                elog(
                    elevel,
                    b"invalid option \"%s\"\0" as *const u8 as *const libc::c_char,
                    key.as_mut_ptr(),
                );
            }
        }
    }
    fclose(fp);
}
unsafe extern "C" fn skip_space(
    mut str: *const libc::c_char,
    mut line: *const libc::c_char,
) -> *const libc::c_char {
    let mut tmp: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    loop {
        tmp = __ctype_b_loc();
        if *(*tmp).offset(*str as libc::c_uchar as libc::c_int as isize) as libc::c_int
            & 8192 as libc::c_int == 0
        {
            break;
        }
        str = str.offset(1);
    }
    return str;
}
unsafe extern "C" fn get_next_token(
    mut src: *const libc::c_char,
    mut dst: *mut libc::c_char,
    mut line: *const libc::c_char,
) -> *const libc::c_char {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: libc::c_int = 0;
    let mut octVal: libc::c_long = 0;
    s = skip_space(src, line);
    if s as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *const libc::c_char;
    }
    if *s as libc::c_int == 39 as libc::c_int {
        s = s.offset(1);
        i = 0 as libc::c_int as size_t;
        j = 0 as libc::c_int as size_t;
        while *s.offset(i as isize) as libc::c_int != 0 as libc::c_int {
            if *s.offset(i as isize) as libc::c_int == 92 as libc::c_int {
                i = i.wrapping_add(1);
                match *s.offset(i as isize) as libc::c_int {
                    98 => {
                        *dst.offset(j as isize) = '\u{8}' as i32 as libc::c_char;
                    }
                    102 => {
                        *dst.offset(j as isize) = '\u{c}' as i32 as libc::c_char;
                    }
                    110 => {
                        *dst.offset(j as isize) = '\n' as i32 as libc::c_char;
                    }
                    114 => {
                        *dst.offset(j as isize) = '\r' as i32 as libc::c_char;
                    }
                    116 => {
                        *dst.offset(j as isize) = '\t' as i32 as libc::c_char;
                    }
                    55 | 54 | 53 | 52 | 51 | 50 | 49 | 48 => {
                        octVal = 0 as libc::c_long;
                        k = 0 as libc::c_int;
                        while *s.offset(i.wrapping_add(k as size_t) as isize)
                            as libc::c_int >= 48 as libc::c_int
                        {
                            if !(*s.offset(i.wrapping_add(k as size_t) as isize)
                                as libc::c_int <= 55 as libc::c_int)
                            {
                                break;
                            }
                            if !(k < 3 as libc::c_int) {
                                break;
                            }
                            octVal = (octVal << 3 as libc::c_int)
                                + (*s.offset(i.wrapping_add(k as size_t) as isize)
                                    as libc::c_int - 48 as libc::c_int) as libc::c_long;
                            k += 1;
                        }
                        i = (i as libc::c_ulong)
                            .wrapping_add((k - 1 as libc::c_int) as size_t) as size_t
                            as size_t;
                        *dst.offset(j as isize) = octVal as libc::c_char;
                    }
                    _ => {
                        *dst.offset(j as isize) = *s.offset(i as isize);
                    }
                }
            } else if *s.offset(i as isize) as libc::c_int == 39 as libc::c_int {
                i = i.wrapping_add(1);
                if !(*s.offset(i as isize) as libc::c_int == 39 as libc::c_int) {
                    break;
                }
                *dst.offset(j as isize) = *s.offset(i as isize);
            } else {
                *dst.offset(j as isize) = *s.offset(i as isize);
            }
            j = j.wrapping_add(1);
            i = i.wrapping_add(1);
        }
    } else {
        j = strcspn(s, b"# \n\r\t\x0B\0" as *const u8 as *const libc::c_char);
        i = j;
        libc::memcpy(dst as *mut libc::c_void, s as *const libc::c_void, j as _);
    }
    *dst.offset(j as isize) = '\u{0}' as i32 as libc::c_char;
    return s.offset(i as isize);
}
unsafe extern "C" fn parse_pair(
    mut buffer: *const libc::c_char,
    mut key: *mut libc::c_char,
    mut value: *mut libc::c_char,
) -> bool {
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: libc::c_char = 0;
    let mut tmp___0: size_t = 0;
    tmp = '\u{0}' as i32 as libc::c_char;
    *value.offset(0 as libc::c_int as isize) = tmp;
    *key.offset(0 as libc::c_int as isize) = tmp;
    start = buffer;
    start = skip_space(start, buffer);
    if start as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as libc::c_int != 0;
    }
    tmp___0 = strcspn(start, b"=# \n\r\t\x0B\0" as *const u8 as *const libc::c_char);
    end = start.offset(tmp___0 as isize);
    if end.offset_from(start) as libc::c_long <= 0 as libc::c_long {
        if *start as libc::c_int == 61 as libc::c_int {
            elog(
                19 as libc::c_int,
                b"syntax error in \"%s\"\0" as *const u8 as *const libc::c_char,
                buffer,
            );
        }
        return 0 as libc::c_int != 0;
    }
    libc::strncpy(key, start, end.offset_from(start) as _);
    *key
        .offset(
            end.offset_from(start) as libc::c_long as isize,
        ) = '\u{0}' as i32 as libc::c_char;
    start = skip_space(end, buffer);
    if start as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as libc::c_int != 0;
    }
    if *start as libc::c_int != 61 as libc::c_int {
        elog(
            19 as libc::c_int,
            b"syntax error in \"%s\"\0" as *const u8 as *const libc::c_char,
            buffer,
        );
        return 0 as libc::c_int != 0;
    }
    start = start.offset(1);
    end = get_next_token(start, value, buffer);
    if end as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as libc::c_int != 0;
    }
    start = skip_space(end, buffer);
    if start as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as libc::c_int != 0;
    }
    if *start as libc::c_int != 0 as libc::c_int {
        if *start as libc::c_int != 35 as libc::c_int {
            elog(
                19 as libc::c_int,
                b"syntax error in \"%s\"\0" as *const u8 as *const libc::c_char,
                buffer,
            );
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn execute(
    mut query: *const libc::c_char,
    mut nParams: libc::c_int,
    mut params: *mut *const libc::c_char,
) -> *mut PGresult {
    let mut tmp: *mut PGresult = 0 as *mut PGresult;
    tmp = pgut_execute(connection, query, nParams, params);
    return tmp;
}
pub unsafe extern "C" fn execute_elevel(
    mut query: *const libc::c_char,
    mut nParams: libc::c_int,
    mut params: *mut *const libc::c_char,
    mut elevel: libc::c_int,
) -> *mut PGresult {
    let mut tmp: *mut PGresult = 0 as *mut PGresult;
    tmp = pgut_execute_elevel(connection, query, nParams, params, elevel);
    return tmp;
}
pub unsafe extern "C" fn command(
    mut query: *const libc::c_char,
    mut nParams: libc::c_int,
    mut params: *mut *const libc::c_char,
) -> ExecStatusType {
    let mut tmp: ExecStatusType = PGRES_EMPTY_QUERY;
    tmp = pgut_command(connection, query, nParams, params);
    return tmp;
}
unsafe extern "C" fn set_elevel(
    mut opt: *mut pgut_option,
    mut arg: *const libc::c_char,
) {
    pgut_log_level = parse_elevel(arg);
}
static mut default_options: [pgut_option; 9] = unsafe {
    [
        {
            let mut init = pgut_option {
                type_0: 'b' as i32 as libc::c_char,
                sname: 'e' as i32 as libc::c_char,
                lname: b"echo\0" as *const u8 as *const libc::c_char,
                var: &pgut_echo as *const bool as *mut bool as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'f' as i32 as libc::c_char,
                sname: 'E' as i32 as libc::c_char,
                lname: b"elevel\0" as *const u8 as *const libc::c_char,
                var: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(*mut pgut_option, *const libc::c_char) -> (),
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        set_elevel
                            as unsafe extern "C" fn(
                                *mut pgut_option,
                                *const libc::c_char,
                            ) -> (),
                    ),
                ),
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 's' as i32 as libc::c_char,
                sname: 'd' as i32 as libc::c_char,
                lname: b"dbname\0" as *const u8 as *const libc::c_char,
                var: &dbname as *const *mut libc::c_char as *mut *mut libc::c_char
                    as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 's' as i32 as libc::c_char,
                sname: 'h' as i32 as libc::c_char,
                lname: b"host\0" as *const u8 as *const libc::c_char,
                var: &host as *const *mut libc::c_char as *mut *mut libc::c_char
                    as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 's' as i32 as libc::c_char,
                sname: 'p' as i32 as libc::c_char,
                lname: b"port\0" as *const u8 as *const libc::c_char,
                var: &port as *const *mut libc::c_char as *mut *mut libc::c_char
                    as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 's' as i32 as libc::c_char,
                sname: 'U' as i32 as libc::c_char,
                lname: b"username\0" as *const u8 as *const libc::c_char,
                var: &username as *const *mut libc::c_char as *mut *mut libc::c_char
                    as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'Y' as i32 as libc::c_char,
                sname: 'w' as i32 as libc::c_char,
                lname: b"no-password\0" as *const u8 as *const libc::c_char,
                var: &prompt_password as *const YesNo as *mut YesNo as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 'y' as i32 as libc::c_char,
                sname: 'W' as i32 as libc::c_char,
                lname: b"password\0" as *const u8 as *const libc::c_char,
                var: &prompt_password as *const YesNo as *mut YesNo as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
        {
            let mut init = pgut_option {
                type_0: 0 as libc::c_int as libc::c_char,
                sname: 0 as libc::c_int as libc::c_char,
                lname: 0 as *const libc::c_char,
                var: 0 as *const libc::c_void as *mut libc::c_void,
                allowed: SOURCE_DEFAULT,
                source: SOURCE_DEFAULT,
            };
            init
        },
    ]
};
unsafe extern "C" fn option_length(mut opts: *const pgut_option) -> size_t {
    let mut len: size_t = 0;
    len = 0 as libc::c_int as size_t;
    while !opts.is_null() {
        if (*opts.offset(len as isize)).type_0 == 0 {
            break;
        }
        len = len.wrapping_add(1);
    }
    return len;
}
unsafe extern "C" fn option_find(
    mut c: libc::c_int,
    mut opts1: *mut pgut_option,
    mut opts2: *mut pgut_option,
) -> *mut pgut_option {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while !opts1.is_null() {
        if (*opts1.offset(i as isize)).type_0 == 0 {
            break;
        }
        if (*opts1.offset(i as isize)).sname as libc::c_int == c {
            return opts1.offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as size_t;
    while !opts2.is_null() {
        if (*opts2.offset(i as isize)).type_0 == 0 {
            break;
        }
        if (*opts2.offset(i as isize)).sname as libc::c_int == c {
            return opts2.offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    return 0 as *mut libc::c_void as *mut pgut_option;
}
unsafe extern "C" fn get_username() -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pw: *mut passwd = 0 as *mut passwd;
    let mut tmp: __uid_t = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: bool = false;
    tmp = geteuid();
    pw = getpwuid(tmp);
    if !pw.is_null() {
        ret = (*pw).pw_name;
    } else {
        ret = 0 as *mut libc::c_void as *mut libc::c_char;
    }
    if ret as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___2 = pgut_errstart(20 as libc::c_int);
        if tmp___2 {
            tmp___0 = errmsg(
                b"could not get current user name: \0" as *const u8
                    as *const libc::c_char,
            );
            tmp___1 = errcode_errno();
            pgut_errfinish(tmp___1, tmp___0);
        }
    }
    return ret;
}
unsafe extern "C" fn option_has_arg(mut type_0: libc::c_char) -> libc::c_int {
    match type_0 as libc::c_int {
        89 | 121 | 66 | 98 => return 0 as libc::c_int,
        _ => return 1 as libc::c_int,
    };
}
unsafe extern "C" fn option_copy(
    mut dst: *mut option,
    mut opts: *const pgut_option,
    mut len: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < len {
        let ref mut fresh23 = (*dst.offset(i as isize)).name;
        *fresh23 = (*opts.offset(i as isize)).lname;
        (*dst.offset(i as isize))
            .has_arg = option_has_arg((*opts.offset(i as isize)).type_0);
        let ref mut fresh24 = (*dst.offset(i as isize)).flag;
        *fresh24 = 0 as *mut libc::c_void as *mut libc::c_int;
        (*dst.offset(i as isize)).val = (*opts.offset(i as isize)).sname as libc::c_int;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn option_merge(
    mut opts1: *const pgut_option,
    mut opts2: *const pgut_option,
) -> *mut option {
    let mut result: *mut option = 0 as *mut option;
    let mut len1: size_t = 0;
    let mut tmp: size_t = 0;
    let mut len2: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut n: size_t = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = option_length(opts1);
    len1 = tmp;
    tmp___0 = option_length(opts2);
    len2 = tmp___0;
    n = len1.wrapping_add(len2);
    tmp___1 = pgut_malloc(
        (::std::mem::size_of::<option>() as libc::c_ulong)
            .wrapping_mul(n.wrapping_add(1 as libc::c_ulong)),
    );
    result = tmp___1 as *mut option;
    option_copy(result, opts1, len1);
    option_copy(result.offset(len1 as isize), opts2, len2);
    libc::memset(
        result.offset(n as isize) as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<pgut_option>() as _,
    );
    return result;
}
unsafe extern "C" fn longopts_to_optstring(
    mut opts: *const option,
) -> *mut libc::c_char {
    let mut len: size_t = 0;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    len = 0 as libc::c_int as size_t;
    while !((*opts.offset(len as isize)).name).is_null() {
        len = len.wrapping_add(1);
    }
    tmp = pgut_malloc(
        len.wrapping_mul(2 as libc::c_ulong).wrapping_add(1 as libc::c_ulong),
    );
    result = tmp as *mut libc::c_char;
    s = result;
    len = 0 as libc::c_int as size_t;
    while !((*opts.offset(len as isize)).name).is_null() {
        tmp___0 = __ctype_b_loc();
        if !(*(*tmp___0).offset((*opts.offset(len as isize)).val as isize) as libc::c_int
            & 16384 as libc::c_int == 0)
        {
            tmp___1 = s;
            s = s.offset(1);
            *tmp___1 = (*opts.offset(len as isize)).val as libc::c_char;
            if (*opts.offset(len as isize)).has_arg != 0 as libc::c_int {
                tmp___2 = s;
                s = s.offset(1);
                *tmp___2 = ':' as i32 as libc::c_char;
            }
        }
        len = len.wrapping_add(1);
    }
    *s = '\u{0}' as i32 as libc::c_char;
    return result;
}
pub unsafe extern "C" fn pgut_getopt(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut options___0: *mut pgut_option,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut optindex: libc::c_int = 0;
    let mut optstring: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut longopts: *mut option = 0 as *mut option;
    let mut opt: *mut pgut_option = 0 as *mut pgut_option;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    optindex = 0 as libc::c_int;
    pgut_init(argc, argv);
    if argc > 1 as libc::c_int {
        tmp = strcmp(
            *argv.offset(1 as libc::c_int as isize) as *const libc::c_char,
            b"--help\0" as *const u8 as *const libc::c_char,
        );
        if tmp == 0 as libc::c_int {
            help(1 as libc::c_int != 0);
            exit(0 as libc::c_int);
        } else {
            tmp___0 = strcmp(
                *argv.offset(1 as libc::c_int as isize) as *const libc::c_char,
                b"-?\0" as *const u8 as *const libc::c_char,
            );
            if tmp___0 == 0 as libc::c_int {
                help(1 as libc::c_int != 0);
                exit(0 as libc::c_int);
            }
        }
        tmp___1 = strcmp(
            *argv.offset(1 as libc::c_int as isize) as *const libc::c_char,
            b"--version\0" as *const u8 as *const libc::c_char,
        );
        if tmp___1 == 0 as libc::c_int {
            pg_printf(
                b"%s %s\n\0" as *const u8 as *const libc::c_char,
                PROGRAM_NAME,
                PROGRAM_VERSION,
            );
            exit(0 as libc::c_int);
        } else {
            tmp___2 = strcmp(
                *argv.offset(1 as libc::c_int as isize) as *const libc::c_char,
                b"-V\0" as *const u8 as *const libc::c_char,
            );
            if tmp___2 == 0 as libc::c_int {
                pg_printf(
                    b"%s %s\n\0" as *const u8 as *const libc::c_char,
                    PROGRAM_NAME,
                    PROGRAM_VERSION,
                );
                exit(0 as libc::c_int);
            }
        }
        tmp___3 = strcmp(
            *argv.offset(1 as libc::c_int as isize) as *const libc::c_char,
            b"--configuration\0" as *const u8 as *const libc::c_char,
        );
        if tmp___3 == 0 as libc::c_int {
            pg_printf(
                b"%s\n\0" as *const u8 as *const libc::c_char,
                b"PostgreSQL 13.7 (Ubuntu 13.7-1.pgdg20.04+1) on x86_64-pc-linux-gnu, compiled by gcc (Ubuntu 9.4.0-1ubuntu1~20.04.1) 9.4.0, 64-bit\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(0 as libc::c_int);
        }
    }
    longopts = option_merge(
        default_options.as_mut_ptr() as *const pgut_option,
        options___0 as *const pgut_option,
    );
    optstring = longopts_to_optstring(longopts as *const option);
    loop {
        c = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            optstring as *const libc::c_char,
            longopts as *const option,
            &mut optindex,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        opt = option_find(c, default_options.as_mut_ptr(), options___0);
        pgut_setopt(opt, optarg as *const libc::c_char, SOURCE_CMDLINE);
    }
    option_from_env(options___0);
    if !dbname.is_null() {
        tmp___4 = 1 as libc::c_int;
    } else {
        dbname = getenv(b"PGDATABASE\0" as *const u8 as *const libc::c_char);
        if !dbname.is_null() {
            tmp___4 = 1 as libc::c_int;
        } else {
            dbname = getenv(b"PGUSER\0" as *const u8 as *const libc::c_char);
            if !dbname.is_null() {
                tmp___4 = 1 as libc::c_int;
            } else {
                dbname = get_username();
                if !dbname.is_null() {
                    tmp___4 = 1 as libc::c_int;
                } else {
                    tmp___4 = 0 as libc::c_int;
                }
            }
        }
    }
    return optind;
}
pub unsafe extern "C" fn help(mut details: bool) {
    pgut_help(details);
    if details {
        pg_printf(b"\nConnection options:\n\0" as *const u8 as *const libc::c_char);
        pg_printf(
            b"  -d, --dbname=DBNAME       database to connect\n\0" as *const u8
                as *const libc::c_char,
        );
        pg_printf(
            b"  -h, --host=HOSTNAME       database server host or socket directory\n\0"
                as *const u8 as *const libc::c_char,
        );
        pg_printf(
            b"  -p, --port=PORT           database server port\n\0" as *const u8
                as *const libc::c_char,
        );
        pg_printf(
            b"  -U, --username=USERNAME   user name to connect as\n\0" as *const u8
                as *const libc::c_char,
        );
        pg_printf(
            b"  -w, --no-password         never prompt for password\n\0" as *const u8
                as *const libc::c_char,
        );
        pg_printf(
            b"  -W, --password            force password prompt\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    pg_printf(b"\nGeneric options:\n\0" as *const u8 as *const libc::c_char);
    if details {
        pg_printf(
            b"  -e, --echo                echo queries\n\0" as *const u8
                as *const libc::c_char,
        );
        pg_printf(
            b"  -E, --elevel=LEVEL        set output message level\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    pg_printf(
        b"  --help                    show this help, then exit\n\0" as *const u8
            as *const libc::c_char,
    );
    pg_printf(
        b"  --version                 output version information, then exit\n\0"
            as *const u8 as *const libc::c_char,
    );
    if details {
        let mut current_block_23: u64;
        if !PROGRAM_URL.is_null() {
            current_block_23 = 1334427664935841474;
        } else if !PROGRAM_ISSUES.is_null() {
            current_block_23 = 1334427664935841474;
        } else {
            current_block_23 = 11584701595673473500;
        }
        match current_block_23 {
            1334427664935841474 => {
                pg_printf(b"\n\0" as *const u8 as *const libc::c_char);
                if !PROGRAM_URL.is_null() {
                    pg_printf(
                        b"Read the website for details: <%s>.\n\0" as *const u8
                            as *const libc::c_char,
                        PROGRAM_URL,
                    );
                }
                if !PROGRAM_ISSUES.is_null() {
                    pg_printf(
                        b"Report bugs to <%s>.\n\0" as *const u8 as *const libc::c_char,
                        PROGRAM_ISSUES,
                    );
                }
            }
            _ => {}
        }
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
