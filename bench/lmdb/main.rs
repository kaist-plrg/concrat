use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigwait(__set: *const sigset_t, __sig: *mut libc::c_int) -> libc::c_int;
    fn pthread_sigmask(
        __how: libc::c_int,
        __newmask: *const __sigset_t,
        __oldmask: *mut __sigset_t,
    ) -> libc::c_int;
    fn writev(__fd: libc::c_int, __iovec: *const iovec, __count: libc::c_int) -> ssize_t;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn msync(
        __addr: *mut libc::c_void,
        __len: size_t,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn madvise(
        __addr: *mut libc::c_void,
        __len: size_t,
        __advice: libc::c_int,
    ) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn posix_memalign(
        __memptr: *mut *mut libc::c_void,
        __alignment: size_t,
        __size: size_t,
    ) -> libc::c_int;
    fn abort() -> !;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pread(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __nbytes: size_t,
        __offset: __off_t,
    ) -> ssize_t;
    fn pwrite(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __offset: __off_t,
    ) -> ssize_t;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn getpid() -> __pid_t;
    fn fsync(__fd: libc::c_int) -> libc::c_int;
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
    fn fdatasync(__fildes: libc::c_int) -> libc::c_int;
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
    fn pthread_self() -> pthread_t;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_consistent(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutexattr_init(__attr: *mut pthread_mutexattr_t) -> libc::c_int;
    fn pthread_mutexattr_destroy(__attr: *mut pthread_mutexattr_t) -> libc::c_int;
    fn pthread_mutexattr_setpshared(
        __attr: *mut pthread_mutexattr_t,
        __pshared: libc::c_int,
    ) -> libc::c_int;
    fn pthread_mutexattr_setrobust(
        __attr: *mut pthread_mutexattr_t,
        __robustness: libc::c_int,
    ) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn pthread_key_create(
        __key: *mut pthread_key_t,
        __destr_function: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> libc::c_int;
    fn pthread_key_delete(__key: pthread_key_t) -> libc::c_int;
    fn pthread_getspecific(__key: pthread_key_t) -> *mut libc::c_void;
    fn pthread_setspecific(
        __key: pthread_key_t,
        __pointer: *const libc::c_void,
    ) -> libc::c_int;
    fn uname(__name: *mut utsname) -> libc::c_int;
    fn fstatfs(__fildes: libc::c_int, __buf: *mut statfs) -> libc::c_int;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn time(__timer: *mut time_t) -> time_t;
}
pub type __uint16_t = libc::c_ushort;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___fsid_t_109580352 {
    pub __val: [libc::c_int; 2],
}
pub type __fsid_t = __anonstruct___fsid_t_109580352;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __fsblkcnt_t = libc::c_ulong;
pub type __fsfilcnt_t = libc::c_ulong;
pub type __fsword_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type mode_t = __mode_t;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___sigset_t_991265788 {
    pub __val: [libc::c_ulong; 16],
}
pub type __sigset_t = __anonstruct___sigset_t_991265788;
pub type sigset_t = __sigset_t;
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
pub struct __anonstruct___wseq32_112954846 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_578844408 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: __anonstruct___wseq32_112954846,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___g1_start32_175635502 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_175635501 {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: __anonstruct___g1_start32_175635502,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __annonCompField1: __anonunion____missing_field_name_578844408,
    pub __annonCompField2: __anonunion____missing_field_name_175635501,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
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
pub union __anonunion_pthread_condattr_t_488594145 {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
pub type pthread_condattr_t = __anonunion_pthread_condattr_t_488594145;
pub type pthread_key_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
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
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flock {
    pub l_type: libc::c_short,
    pub l_whence: libc::c_short,
    pub l_start: __off_t,
    pub l_len: __off_t,
    pub l_pid: __pid_t,
}
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub type mdb_mode_t = mode_t;
pub type mdb_size_t = size_t;
pub type mdb_filehandle_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_env {
    pub me_fd: libc::c_int,
    pub me_lfd: libc::c_int,
    pub me_mfd: libc::c_int,
    pub me_flags: uint32_t,
    pub me_psize: libc::c_uint,
    pub me_os_psize: libc::c_uint,
    pub me_maxreaders: libc::c_uint,
    pub me_close_readers: libc::c_int,
    pub me_numdbs: MDB_dbi,
    pub me_maxdbs: MDB_dbi,
    pub me_pid: pid_t,
    pub me_path: *mut libc::c_char,
    pub me_map: *mut libc::c_char,
    pub me_txns: *mut MDB_txninfo,
    pub me_metas: [*mut MDB_meta; 2],
    pub me_pbuf: *mut libc::c_void,
    pub me_txn: *mut MDB_txn,
    pub me_txn0: *mut MDB_txn,
    pub me_mapsize: mdb_size_t,
    pub me_size: off_t,
    pub me_maxpg: pgno_t,
    pub me_dbxs: *mut MDB_dbx,
    pub me_dbflags: *mut uint16_t,
    pub me_dbiseqs: *mut libc::c_uint,
    pub me_txkey: pthread_key_t,
    pub me_pgoldest: txnid_t,
    pub me_pgstate: MDB_pgstate,
    pub me_dpages: *mut MDB_page,
    pub me_free_pgs: MDB_IDL,
    pub me_dirty_list: MDB_ID2L,
    pub me_maxfree_1pg: libc::c_int,
    pub me_nodemax: libc::c_uint,
    pub me_live_reader: libc::c_int,
    pub me_userctx: *mut libc::c_void,
    pub me_assert_func: Option::<MDB_assert_func>,
}
pub type MDB_assert_func = unsafe extern "C" fn(*mut MDB_env, *const libc::c_char) -> ();
pub type MDB_ID2L = *mut MDB_ID2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_ID2 {
    pub mid: MDB_ID,
    pub mptr: *mut libc::c_void,
}
pub type MDB_ID = mdb_size_t;
pub type MDB_IDL = *mut MDB_ID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_page {
    pub mp_p: __anonunion_mp_p_409824043,
    pub mp_pad: uint16_t,
    pub mp_flags: uint16_t,
    pub mp_pb: __anonunion_mp_pb_280193743,
    pub mp_ptrs: [indx_t; 1],
}
pub type indx_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_mp_pb_280193743 {
    pub pb: __anonstruct_pb_438838223,
    pub pb_pages: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_pb_438838223 {
    pub pb_lower: indx_t,
    pub pb_upper: indx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_mp_p_409824043 {
    pub p_pgno: pgno_t,
    pub p_next: *mut MDB_page,
}
pub type pgno_t = MDB_ID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_pgstate {
    pub mf_pghead: *mut pgno_t,
    pub mf_pglast: txnid_t,
}
pub type txnid_t = MDB_ID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_dbx {
    pub md_name: MDB_val,
    pub md_cmp: Option::<MDB_cmp_func>,
    pub md_dcmp: Option::<MDB_cmp_func>,
    pub md_rel: Option::<MDB_rel_func>,
    pub md_relctx: *mut libc::c_void,
}
pub type MDB_rel_func = unsafe extern "C" fn(
    *mut MDB_val,
    *mut libc::c_void,
    *mut libc::c_void,
    *mut libc::c_void,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_val {
    pub mv_size: size_t,
    pub mv_data: *mut libc::c_void,
}
pub type MDB_cmp_func = unsafe extern "C" fn(
    *const MDB_val,
    *const MDB_val,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_txn {
    pub mt_parent: *mut MDB_txn,
    pub mt_child: *mut MDB_txn,
    pub mt_next_pgno: pgno_t,
    pub mt_txnid: txnid_t,
    pub mt_env: *mut MDB_env,
    pub mt_free_pgs: MDB_IDL,
    pub mt_loose_pgs: *mut MDB_page,
    pub mt_loose_count: libc::c_int,
    pub mt_spill_pgs: MDB_IDL,
    pub mt_u: __anonunion_mt_u_704457406,
    pub mt_dbxs: *mut MDB_dbx,
    pub mt_dbs: *mut MDB_db,
    pub mt_dbiseqs: *mut libc::c_uint,
    pub mt_cursors: *mut *mut MDB_cursor,
    pub mt_dbflags: *mut libc::c_uchar,
    pub mt_numdbs: MDB_dbi,
    pub mt_flags: libc::c_uint,
    pub mt_dirty_room: libc::c_uint,
}
pub type MDB_dbi = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_cursor {
    pub mc_next: *mut MDB_cursor,
    pub mc_backup: *mut MDB_cursor,
    pub mc_xcursor: *mut MDB_xcursor,
    pub mc_txn: *mut MDB_txn,
    pub mc_dbi: MDB_dbi,
    pub mc_db: *mut MDB_db,
    pub mc_dbx: *mut MDB_dbx,
    pub mc_dbflag: *mut libc::c_uchar,
    pub mc_snum: libc::c_ushort,
    pub mc_top: libc::c_ushort,
    pub mc_flags: libc::c_uint,
    pub mc_pg: [*mut MDB_page; 32],
    pub mc_ki: [indx_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_db {
    pub md_pad: uint32_t,
    pub md_flags: uint16_t,
    pub md_depth: uint16_t,
    pub md_branch_pages: pgno_t,
    pub md_leaf_pages: pgno_t,
    pub md_overflow_pages: pgno_t,
    pub md_entries: mdb_size_t,
    pub md_root: pgno_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_xcursor {
    pub mx_cursor: MDB_cursor,
    pub mx_db: MDB_db,
    pub mx_dbx: MDB_dbx,
    pub mx_dbflag: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_mt_u_704457406 {
    pub dirty_list: MDB_ID2L,
    pub reader: *mut MDB_reader,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_reader {
    pub mru: __anonunion_mru_29383123,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_mru_29383123 {
    pub mrx: MDB_rxbody,
    pub pad: [libc::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_rxbody {
    pub mrb_txnid: txnid_t,
    pub mrb_pid: pid_t,
    pub mrb_tid: pthread_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_meta {
    pub mm_magic: uint32_t,
    pub mm_version: uint32_t,
    pub mm_address: *mut libc::c_void,
    pub mm_mapsize: mdb_size_t,
    pub mm_dbs: [MDB_db; 2],
    pub mm_last_pg: pgno_t,
    pub mm_txnid: txnid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_txninfo {
    pub mt1: __anonunion_mt1_1001079246,
    pub mt2: __anonunion_mt2_574372486,
    pub mti_readers: [MDB_reader; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_mt2_574372486 {
    pub mt2_wmutex: mdb_mutex_t,
    pub pad: [libc::c_char; 64],
}
pub type mdb_mutex_t = [pthread_mutex_t; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_mt1_1001079246 {
    pub mtb: MDB_txbody,
    pub pad: [libc::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_txbody {
    pub mtb_magic: uint32_t,
    pub mtb_format: uint32_t,
    pub mtb_txnid: txnid_t,
    pub mtb_numreaders: libc::c_uint,
    pub mtb_rmutex: mdb_mutex_t,
}
pub type MDB_cursor_op = libc::c_uint;
pub const MDB_PREV_MULTIPLE: MDB_cursor_op = 18;
pub const MDB_SET_RANGE: MDB_cursor_op = 17;
pub const MDB_SET_KEY: MDB_cursor_op = 16;
pub const MDB_SET: MDB_cursor_op = 15;
pub const MDB_PREV_NODUP: MDB_cursor_op = 14;
pub const MDB_PREV_DUP: MDB_cursor_op = 13;
pub const MDB_PREV: MDB_cursor_op = 12;
pub const MDB_NEXT_NODUP: MDB_cursor_op = 11;
pub const MDB_NEXT_MULTIPLE: MDB_cursor_op = 10;
pub const MDB_NEXT_DUP: MDB_cursor_op = 9;
pub const MDB_NEXT: MDB_cursor_op = 8;
pub const MDB_LAST_DUP: MDB_cursor_op = 7;
pub const MDB_LAST: MDB_cursor_op = 6;
pub const MDB_GET_MULTIPLE: MDB_cursor_op = 5;
pub const MDB_GET_CURRENT: MDB_cursor_op = 4;
pub const MDB_GET_BOTH_RANGE: MDB_cursor_op = 3;
pub const MDB_GET_BOTH: MDB_cursor_op = 2;
pub const MDB_FIRST_DUP: MDB_cursor_op = 1;
pub const MDB_FIRST: MDB_cursor_op = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_stat {
    pub ms_psize: libc::c_uint,
    pub ms_depth: libc::c_uint,
    pub ms_branch_pages: mdb_size_t,
    pub ms_leaf_pages: mdb_size_t,
    pub ms_overflow_pages: mdb_size_t,
    pub ms_entries: mdb_size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_envinfo {
    pub me_mapaddr: *mut libc::c_void,
    pub me_mapsize: mdb_size_t,
    pub me_last_pgno: mdb_size_t,
    pub me_last_txnid: mdb_size_t,
    pub me_maxreaders: libc::c_uint,
    pub me_numreaders: libc::c_uint,
}
pub type MDB_msg_func = unsafe extern "C" fn(
    *const libc::c_char,
    *mut libc::c_void,
) -> libc::c_int;
pub type mdb_mutexref_t = *mut pthread_mutex_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_node {
    pub mn_lo: libc::c_ushort,
    pub mn_hi: libc::c_ushort,
    pub mn_flags: libc::c_ushort,
    pub mn_ksize: libc::c_ushort,
    pub mn_data: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_mb_metabuf_1013834518 {
    pub mm_pad: [libc::c_char; 16],
    pub mm_meta: MDB_meta,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union MDB_metabuf {
    pub mb_page: MDB_page,
    pub mb_metabuf: __anonstruct_mb_metabuf_1013834518,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_ntxn {
    pub mnt_txn: MDB_txn,
    pub mnt_pgstate: MDB_pgstate,
}
pub type Pidlock_op = libc::c_uint;
pub const Pidcheck: Pidlock_op = 5;
pub const Pidset: Pidlock_op = 6;
pub type mdb_nchar_t = libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_name {
    pub mn_len: libc::c_int,
    pub mn_alloced: libc::c_int,
    pub mn_val: *mut mdb_nchar_t,
}
pub type mdb_fopen_type = libc::c_uint;
pub const MDB_O_LOCKS: mdb_fopen_type = 524358;
pub const MDB_O_MASK: mdb_fopen_type = 528579;
pub const MDB_O_COPY: mdb_fopen_type = 524481;
pub const MDB_O_META: mdb_fopen_type = 528385;
pub const MDB_O_RDWR: mdb_fopen_type = 66;
pub const MDB_O_RDONLY: mdb_fopen_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [libc::c_char; 65],
    pub nodename: [libc::c_char; 65],
    pub release: [libc::c_char; 65],
    pub version: [libc::c_char; 65],
    pub machine: [libc::c_char; 65],
    pub domainname: [libc::c_char; 65],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statfs {
    pub f_type: __fsword_t,
    pub f_bsize: __fsword_t,
    pub f_blocks: __fsblkcnt_t,
    pub f_bfree: __fsblkcnt_t,
    pub f_bavail: __fsblkcnt_t,
    pub f_files: __fsfilcnt_t,
    pub f_ffree: __fsfilcnt_t,
    pub f_fsid: __fsid_t,
    pub f_namelen: __fsword_t,
    pub f_frsize: __fsword_t,
    pub f_flags: __fsword_t,
    pub f_spare: [__fsword_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mdb_copy {
    pub mc_env: *mut MDB_env,
    pub mc_txn: *mut MDB_txn,
    pub mc_mutex: pthread_mutex_t,
    pub mc_cond: pthread_cond_t,
    pub mc_wbuf: [*mut libc::c_char; 2],
    pub mc_over: [*mut libc::c_char; 2],
    pub mc_wlen: [libc::c_int; 2],
    pub mc_olen: [libc::c_int; 2],
    pub mc_next_pgno: pgno_t,
    pub mc_fd: libc::c_int,
    pub mc_toggle: libc::c_int,
    pub mc_new: libc::c_int,
    pub mc_error: libc::c_int,
}
pub type time_t = __time_t;
pub unsafe extern "C" fn mdb_version(
    mut major: *mut libc::c_int,
    mut minor: *mut libc::c_int,
    mut patch: *mut libc::c_int,
) -> *mut libc::c_char {
    if !major.is_null() {
        *major = 0 as libc::c_int;
    }
    if !minor.is_null() {
        *minor = 9 as libc::c_int;
    }
    if !patch.is_null() {
        *patch = 70 as libc::c_int;
    }
    return b"LMDB 0.9.70: (December 19, 2015)\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
}
static mut mdb_errstr: [*mut libc::c_char; 21] = [
    b"MDB_KEYEXIST: Key/data pair already exists\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"MDB_NOTFOUND: No matching key/data pair found\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_PAGE_NOTFOUND: Requested page not found\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"MDB_CORRUPTED: Located page was wrong type\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"MDB_PANIC: Update of meta page failed or environment had fatal error\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MDB_VERSION_MISMATCH: Database environment version mismatch\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_INVALID: File is not an LMDB file\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"MDB_MAP_FULL: Environment mapsize limit reached\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_DBS_FULL: Environment maxdbs limit reached\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_READERS_FULL: Environment maxreaders limit reached\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_TLS_FULL: Thread-local storage keys full - too many environments open\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MDB_TXN_FULL: Transaction has too many dirty pages - transaction too big\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MDB_CURSOR_FULL: Internal error - cursor stack limit reached\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_PAGE_FULL: Internal error - page has no more space\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_MAP_RESIZED: Database contents grew beyond environment mapsize\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_INCOMPATIBLE: Operation and DB incompatible, or DB flags changed\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MDB_BAD_RSLOT: Invalid reuse of reader locktable slot\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_BAD_TXN: Transaction must abort, has a child, or is invalid\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_BAD_VALSIZE: Unsupported size of key/DB name/data, or wrong DUPFIXED size\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MDB_BAD_DBI: The specified DBI handle was closed/changed unexpectedly\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MDB_PROBLEM: Unexpected problem - txn should abort\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
];
pub unsafe extern "C" fn mdb_strerror(mut err: libc::c_int) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    if err == 0 {
        return b"Successful return: 0\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    if err >= -(30799 as libc::c_int) {
        if err <= -(30779 as libc::c_int) {
            i = err - -(30799 as libc::c_int);
            return mdb_errstr[i as usize];
        }
    }
    tmp = strerror(err);
    return tmp;
}
unsafe extern "C" fn mdb_assert_fail(
    mut env: *mut MDB_env,
    mut expr_txt: *const libc::c_char,
    mut func: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
) {
    let mut buf: [libc::c_char; 400] = [0; 400];
    sprintf(
        buf.as_mut_ptr(),
        b"%.100s:%d: Assertion '%.200s' failed in %.40s()\0" as *const u8
            as *const libc::c_char,
        file,
        line,
        expr_txt,
        func,
    );
    if ((*env).me_assert_func).is_some() {
        (Some(((*env).me_assert_func).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(env, buf.as_mut_ptr() as *const libc::c_char);
    }
    fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, buf.as_mut_ptr());
    abort();
}
pub unsafe extern "C" fn mdb_cmp(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut a: *const MDB_val,
    mut b: *const MDB_val,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = (Some(
        ((*((*txn).mt_dbxs).offset(dbi as isize)).md_cmp)
            .expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(a, b);
    return tmp;
}
pub unsafe extern "C" fn mdb_dcmp(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut a: *const MDB_val,
    mut b: *const MDB_val,
) -> libc::c_int {
    let mut dcmp: Option::<MDB_cmp_func> = None;
    let mut tmp: libc::c_int = 0;
    dcmp = (*((*txn).mt_dbxs).offset(dbi as isize)).md_dcmp;
    if ::std::mem::transmute::<Option::<MDB_cmp_func>, libc::c_ulong>(dcmp)
        == ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*const MDB_val, *const MDB_val) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                mdb_cmp_int
                    as unsafe extern "C" fn(
                        *const MDB_val,
                        *const MDB_val,
                    ) -> libc::c_int,
            ),
        )
    {
        if (*a).mv_size == ::std::mem::size_of::<mdb_size_t>() as libc::c_ulong {
            dcmp = Some(
                mdb_cmp_long
                    as unsafe extern "C" fn(
                        *const MDB_val,
                        *const MDB_val,
                    ) -> libc::c_int,
            );
        }
    }
    tmp = (Some(dcmp.expect("non-null function pointer")))
        .expect("non-null function pointer")(a, b);
    return tmp;
}
unsafe extern "C" fn mdb_page_malloc(
    mut txn: *mut MDB_txn,
    mut num: libc::c_uint,
) -> *mut MDB_page {
    let mut env: *mut MDB_env = 0 as *mut MDB_env;
    let mut ret: *mut MDB_page = 0 as *mut MDB_page;
    let mut psize: size_t = 0;
    let mut sz: size_t = 0;
    let mut off: size_t = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    env = (*txn).mt_env;
    ret = (*env).me_dpages;
    psize = (*env).me_psize as size_t;
    sz = psize;
    if num == 1 as libc::c_uint {
        if !ret.is_null() {
            (*env).me_dpages = (*ret).mp_p.p_next;
            return ret;
        }
        off = &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
            as libc::c_uint as size_t;
        psize = (psize as libc::c_ulong).wrapping_sub(off) as size_t as size_t;
    } else {
        sz = (sz as libc::c_ulong).wrapping_mul(num as size_t) as size_t as size_t;
        off = sz.wrapping_sub(psize);
    }
    tmp = malloc(sz);
    ret = tmp as *mut MDB_page;
    if ret as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        if (*env).me_flags & 16777216 as libc::c_uint == 0 {
            memset(
                (ret as *mut libc::c_char).offset(off as isize) as *mut libc::c_void,
                0 as libc::c_int,
                psize,
            );
            (*ret).mp_pad = 0 as libc::c_int as uint16_t;
        }
    } else {
        (*txn).mt_flags |= 2 as libc::c_uint;
    }
    return ret;
}
unsafe extern "C" fn mdb_page_free(mut env: *mut MDB_env, mut mp: *mut MDB_page) {
    (*mp).mp_p.p_next = (*env).me_dpages;
    (*env).me_dpages = mp;
}
unsafe extern "C" fn mdb_dpage_free(mut env: *mut MDB_env, mut dp: *mut MDB_page) {
    if !((*dp).mp_flags as libc::c_int & 4 as libc::c_int == 4 as libc::c_int) {
        mdb_page_free(env, dp);
    } else if (*dp).mp_pb.pb_pages == 1 as libc::c_uint {
        mdb_page_free(env, dp);
    } else {
        free(dp as *mut libc::c_void);
    };
}
unsafe extern "C" fn mdb_dlist_free(mut txn: *mut MDB_txn) {
    let mut env: *mut MDB_env = 0 as *mut MDB_env;
    let mut dl: MDB_ID2L = 0 as *mut MDB_ID2;
    let mut i: libc::c_uint = 0;
    let mut n: libc::c_uint = 0;
    env = (*txn).mt_env;
    dl = (*txn).mt_u.dirty_list;
    n = (*dl.offset(0 as libc::c_int as isize)).mid as libc::c_uint;
    i = 1 as libc::c_uint;
    while i <= n {
        mdb_dpage_free(env, (*dl.offset(i as isize)).mptr as *mut MDB_page);
        i = i.wrapping_add(1);
    }
    (*dl.offset(0 as libc::c_int as isize)).mid = 0 as libc::c_int as MDB_ID;
}
unsafe extern "C" fn mdb_page_loose(
    mut mc: *mut MDB_cursor,
    mut mp: *mut MDB_page,
) -> libc::c_int {
    let mut loose: libc::c_int = 0;
    let mut pgno: pgno_t = 0;
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut dl: *mut MDB_ID2 = 0 as *mut MDB_ID2;
    let mut x: libc::c_uint = 0;
    let mut tmp: libc::c_uint = 0;
    let mut rc: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    loose = 0 as libc::c_int;
    pgno = (*mp).mp_p.p_pgno;
    txn = (*mc).mc_txn;
    if (*mp).mp_flags as libc::c_int & 16 as libc::c_int != 0 {
        if (*mc).mc_dbi != 0 as libc::c_uint {
            if !((*txn).mt_parent).is_null() {
                dl = (*txn).mt_u.dirty_list;
                if (*dl.offset(0 as libc::c_int as isize)).mid != 0 {
                    tmp = mdb_mid2l_search(dl, pgno);
                    x = tmp;
                    if x as MDB_ID <= (*dl.offset(0 as libc::c_int as isize)).mid {
                        if (*dl.offset(x as isize)).mid == pgno {
                            if mp as libc::c_ulong
                                != (*dl.offset(x as isize)).mptr as libc::c_ulong
                            {
                                (*mc).mc_flags &= 4294967292 as libc::c_uint;
                                (*txn).mt_flags |= 2 as libc::c_uint;
                                return -(30779 as libc::c_int);
                            }
                            loose = 1 as libc::c_int;
                        }
                    }
                }
            } else {
                loose = 1 as libc::c_int;
            }
        }
    }
    if loose != 0 {
        let ref mut fresh0 = *(mp.offset(2 as libc::c_int as isize)
            as *mut *mut MDB_page);
        *fresh0 = (*txn).mt_loose_pgs;
        (*txn).mt_loose_pgs = mp;
        (*txn).mt_loose_count += 1;
        (*mp)
            .mp_flags = ((*mp).mp_flags as libc::c_int | 16384 as libc::c_int)
            as uint16_t;
    } else {
        tmp___0 = mdb_midl_append(&mut (*txn).mt_free_pgs, pgno);
        rc = tmp___0;
        if rc != 0 {
            return rc;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_pages_xkeep(
    mut mc: *mut MDB_cursor,
    mut pflags: libc::c_uint,
    mut all: libc::c_int,
) -> libc::c_int {
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut m3: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut m0: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut mx: *mut MDB_xcursor = 0 as *mut MDB_xcursor;
    let mut dp: *mut MDB_page = 0 as *mut MDB_page;
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut leaf: *mut MDB_node = 0 as *mut MDB_node;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut rc: libc::c_int = 0;
    let mut level: libc::c_int = 0;
    let mut pgno: pgno_t = 0;
    txn = (*mc).mc_txn;
    m0 = mc;
    rc = 0 as libc::c_int;
    i = (*txn).mt_numdbs;
    's_41: loop {
        if (*mc).mc_flags & 1 as libc::c_uint != 0 {
            m3 = mc;
            loop {
                mp = 0 as *mut libc::c_void as *mut MDB_page;
                j = 0 as libc::c_uint;
                while j < (*m3).mc_snum as libc::c_uint {
                    mp = (*m3).mc_pg[j as usize];
                    if ((*mp).mp_flags as libc::c_int & 49232 as libc::c_int)
                        as libc::c_uint == pflags
                    {
                        (*mp)
                            .mp_flags = ((*mp).mp_flags as libc::c_int
                            ^ 32768 as libc::c_int) as uint16_t;
                    }
                    j = j.wrapping_add(1);
                }
                mx = (*m3).mc_xcursor;
                if mx.is_null() {
                    break;
                }
                if (*mx).mx_cursor.mc_flags & 1 as libc::c_uint == 0 {
                    break;
                }
                if mp.is_null() {
                    break;
                }
                if (*mp).mp_flags as libc::c_int & 2 as libc::c_int == 0 {
                    break;
                }
                leaf = (mp as *mut libc::c_char)
                    .offset(
                        *((*mp).mp_ptrs)
                            .as_mut_ptr()
                            .offset(
                                (*m3).mc_ki[j.wrapping_sub(1 as libc::c_uint) as usize]
                                    as isize,
                            ) as libc::c_int as isize,
                    )
                    .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                if (*leaf).mn_flags as libc::c_int & 2 as libc::c_int == 0 {
                    break;
                }
                m3 = &mut (*mx).mx_cursor;
            }
        }
        mc = (*mc).mc_next;
        loop {
            if !mc.is_null() {
                if !(mc as libc::c_ulong == m0 as libc::c_ulong) {
                    break;
                }
            }
            if i == 0 as libc::c_uint {
                break 's_41;
            }
            i = i.wrapping_sub(1);
            mc = *((*txn).mt_cursors).offset(i as isize);
        }
    }
    if all != 0 {
        i = 0 as libc::c_uint;
        while i < (*txn).mt_numdbs {
            if *((*txn).mt_dbflags).offset(i as isize) as libc::c_int & 1 as libc::c_int
                != 0
            {
                pgno = (*((*txn).mt_dbs).offset(i as isize)).md_root;
                if !(pgno as libc::c_ulonglong
                    == 18446744073709551615 as libc::c_ulonglong)
                {
                    rc = mdb_page_get(m0, pgno, &mut dp, &mut level);
                    if rc != 0 as libc::c_int {
                        break;
                    }
                    if ((*dp).mp_flags as libc::c_int & 49232 as libc::c_int)
                        as libc::c_uint == pflags
                    {
                        if level <= 1 as libc::c_int {
                            (*dp)
                                .mp_flags = ((*dp).mp_flags as libc::c_int
                                ^ 32768 as libc::c_int) as uint16_t;
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
        }
    }
    return rc;
}
unsafe extern "C" fn mdb_page_spill(
    mut m0: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
) -> libc::c_int {
    let mut current_block: u64;
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut dp: *mut MDB_page = 0 as *mut MDB_page;
    let mut dl: MDB_ID2L = 0 as *mut MDB_ID2;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut need: libc::c_uint = 0;
    let mut rc: libc::c_int = 0;
    let mut sl: MDB_IDL = 0 as *mut MDB_ID;
    let mut num: libc::c_uint = 0;
    let mut pn: MDB_ID = 0;
    let mut tx2: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut tmp: libc::c_int = 0;
    txn = (*m0).mc_txn;
    dl = (*txn).mt_u.dirty_list;
    if (*m0).mc_flags & 4 as libc::c_uint != 0 {
        return 0 as libc::c_int;
    }
    i = (*(*m0).mc_db).md_depth as libc::c_uint;
    if (*m0).mc_dbi >= 2 as libc::c_uint {
        i = i
            .wrapping_add(
                (*((*txn).mt_dbs).offset(1 as libc::c_int as isize)).md_depth
                    as libc::c_uint,
            );
    }
    if !key.is_null() {
        i = (i as libc::c_ulong)
            .wrapping_add(
                (&mut (*(0 as *mut MDB_node)).mn_data as *mut [libc::c_char; 1]
                    as libc::c_ulong)
                    .wrapping_add((*key).mv_size)
                    .wrapping_add((*data).mv_size)
                    .wrapping_add((*(*txn).mt_env).me_psize as libc::c_ulong)
                    .wrapping_div((*(*txn).mt_env).me_psize as libc::c_ulong),
            ) as libc::c_uint;
    }
    i = i.wrapping_add(i);
    need = i;
    if (*txn).mt_dirty_room > i {
        return 0 as libc::c_int;
    }
    if ((*txn).mt_spill_pgs).is_null() {
        (*txn)
            .mt_spill_pgs = mdb_midl_alloc(
            ((1 as libc::c_int) << 17 as libc::c_int) - 1 as libc::c_int,
        );
        if ((*txn).mt_spill_pgs).is_null() {
            return 12 as libc::c_int;
        }
    } else {
        sl = (*txn).mt_spill_pgs;
        num = *sl.offset(0 as libc::c_int as isize) as libc::c_uint;
        j = 0 as libc::c_uint;
        i = 1 as libc::c_uint;
        while i <= num {
            if *sl.offset(i as isize) & 1 as libc::c_ulong == 0 {
                j = j.wrapping_add(1);
                *sl.offset(j as isize) = *sl.offset(i as isize);
            }
            i = i.wrapping_add(1);
        }
        *sl.offset(0 as libc::c_int as isize) = j as MDB_ID;
    }
    rc = mdb_pages_xkeep(m0, 16 as libc::c_uint, 1 as libc::c_int);
    if !(rc != 0 as libc::c_int) {
        if need
            < ((((1 as libc::c_int) << 17 as libc::c_int) - 1 as libc::c_int)
                / 8 as libc::c_int) as libc::c_uint
        {
            need = ((((1 as libc::c_int) << 17 as libc::c_int) - 1 as libc::c_int)
                / 8 as libc::c_int) as libc::c_uint;
        }
        i = (*dl.offset(0 as libc::c_int as isize)).mid as libc::c_uint;
        loop {
            if !(i != 0) {
                current_block = 9241535491006583629;
                break;
            }
            if need == 0 {
                current_block = 9241535491006583629;
                break;
            }
            pn = (*dl.offset(i as isize)).mid << 1 as libc::c_int;
            dp = (*dl.offset(i as isize)).mptr as *mut MDB_page;
            if !((*dp).mp_flags as libc::c_int & 49152 as libc::c_int != 0) {
                if !((*txn).mt_parent).is_null() {
                    tx2 = (*txn).mt_parent;
                    while !tx2.is_null() {
                        if !((*tx2).mt_spill_pgs).is_null() {
                            j = mdb_midl_search((*tx2).mt_spill_pgs, pn);
                            if j as MDB_ID
                                <= *((*tx2).mt_spill_pgs).offset(0 as libc::c_int as isize)
                            {
                                if *((*tx2).mt_spill_pgs).offset(j as isize) == pn {
                                    (*dp)
                                        .mp_flags = ((*dp).mp_flags as libc::c_int
                                        | 32768 as libc::c_int) as uint16_t;
                                    break;
                                }
                            }
                        }
                        tx2 = (*tx2).mt_parent;
                    }
                    if !tx2.is_null() {
                        current_block = 10457092304298400490;
                    } else {
                        current_block = 1724319918354933278;
                    }
                } else {
                    current_block = 1724319918354933278;
                }
                match current_block {
                    10457092304298400490 => {}
                    _ => {
                        rc = mdb_midl_append(&mut (*txn).mt_spill_pgs, pn);
                        if rc != 0 {
                            current_block = 10203720320027772226;
                            break;
                        }
                        need = need.wrapping_sub(1);
                    }
                }
            }
            i = i.wrapping_sub(1);
        }
        match current_block {
            10203720320027772226 => {}
            _ => {
                mdb_midl_sort((*txn).mt_spill_pgs);
                rc = mdb_page_flush(txn, i as libc::c_int);
                if !(rc != 0 as libc::c_int) {
                    rc = mdb_pages_xkeep(m0, 32784 as libc::c_uint, i as libc::c_int);
                }
            }
        }
    }
    if rc != 0 {
        tmp = 2 as libc::c_int;
    } else {
        tmp = 8 as libc::c_int;
    }
    (*txn).mt_flags |= tmp as libc::c_uint;
    return rc;
}
unsafe extern "C" fn mdb_find_oldest(mut txn: *mut MDB_txn) -> txnid_t {
    let mut i: libc::c_int = 0;
    let mut mr: txnid_t = 0;
    let mut oldest: txnid_t = 0;
    let mut r: *mut MDB_reader = 0 as *mut MDB_reader;
    oldest = ((*txn).mt_txnid).wrapping_sub(1 as libc::c_ulong);
    if !((*(*txn).mt_env).me_txns).is_null() {
        r = ((*(*(*txn).mt_env).me_txns).mti_readers).as_mut_ptr();
        i = (*(*(*txn).mt_env).me_txns).mt1.mtb.mtb_numreaders as libc::c_int;
        loop {
            i -= 1;
            if !(i >= 0 as libc::c_int) {
                break;
            }
            if (*r.offset(i as isize)).mru.mrx.mrb_pid != 0 {
                mr = (*r.offset(i as isize)).mru.mrx.mrb_txnid;
                if oldest > mr {
                    oldest = mr;
                }
            }
        }
    }
    return oldest;
}
unsafe extern "C" fn mdb_page_dirty(mut txn: *mut MDB_txn, mut mp: *mut MDB_page) {
    let mut mid: MDB_ID2 = MDB_ID2 {
        mid: 0,
        mptr: 0 as *mut libc::c_void,
    };
    let mut rc: libc::c_int = 0;
    let mut insert: Option::<
        unsafe extern "C" fn(MDB_ID2L, *mut MDB_ID2) -> libc::c_int,
    > = None;
    if (*txn).mt_flags & 524288 as libc::c_uint != 0 {
        insert = Some(
            mdb_mid2l_append
                as unsafe extern "C" fn(MDB_ID2L, *mut MDB_ID2) -> libc::c_int,
        );
    } else {
        insert = Some(
            mdb_mid2l_insert
                as unsafe extern "C" fn(MDB_ID2L, *mut MDB_ID2) -> libc::c_int,
        );
    }
    mid.mid = (*mp).mp_p.p_pgno;
    mid.mptr = mp as *mut libc::c_void;
    rc = (Some(insert.expect("non-null function pointer")))
        .expect("non-null function pointer")((*txn).mt_u.dirty_list, &mut mid);
    if !(rc == 0 as libc::c_int) {
        mdb_assert_fail(
            (*txn).mt_env,
            b"rc == 0\0" as *const u8 as *const libc::c_char,
            b"mdb_page_dirty\0" as *const u8 as *const libc::c_char,
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            2412 as libc::c_int,
        );
    }
    (*txn).mt_dirty_room = ((*txn).mt_dirty_room).wrapping_sub(1);
}
unsafe extern "C" fn mdb_page_alloc(
    mut mc: *mut MDB_cursor,
    mut num: libc::c_int,
    mut mp: *mut *mut MDB_page,
) -> libc::c_int {
    let mut current_block: u64;
    let mut rc: libc::c_int = 0;
    let mut retry: libc::c_int = 0;
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut env: *mut MDB_env = 0 as *mut MDB_env;
    let mut pgno: pgno_t = 0;
    let mut mop: *mut pgno_t = 0 as *mut pgno_t;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut mop_len: libc::c_uint = 0;
    let mut tmp: pgno_t = 0;
    let mut n2: libc::c_uint = 0;
    let mut np: *mut MDB_page = 0 as *mut MDB_page;
    let mut oldest: txnid_t = 0;
    let mut last: txnid_t = 0;
    let mut op: MDB_cursor_op = MDB_FIRST;
    let mut m2: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut found_old: libc::c_int = 0;
    let mut key: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut data: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut leaf: *mut MDB_node = 0 as *mut MDB_node;
    let mut idl: *mut pgno_t = 0 as *mut pgno_t;
    let mut tmp___0: *mut pgno_t = 0 as *mut pgno_t;
    retry = num * 60 as libc::c_int;
    txn = (*mc).mc_txn;
    env = (*txn).mt_env;
    mop = (*env).me_pgstate.mf_pghead;
    if !mop.is_null() {
        tmp = *mop.offset(0 as libc::c_int as isize);
    } else {
        tmp = 0 as libc::c_int as pgno_t;
    }
    mop_len = tmp as libc::c_uint;
    n2 = (num - 1 as libc::c_int) as libc::c_uint;
    oldest = 0 as libc::c_int as txnid_t;
    found_old = 0 as libc::c_int;
    if num == 1 as libc::c_int {
        if !((*txn).mt_loose_pgs).is_null() {
            np = (*txn).mt_loose_pgs;
            (*txn)
                .mt_loose_pgs = *(np.offset(2 as libc::c_int as isize)
                as *mut *mut MDB_page);
            (*txn).mt_loose_count -= 1;
            *mp = np;
            return 0 as libc::c_int;
        }
    }
    *mp = 0 as *mut libc::c_void as *mut MDB_page;
    if (*txn).mt_dirty_room == 0 as libc::c_uint {
        rc = -(30788 as libc::c_int);
    } else {
        op = MDB_FIRST;
        's_140: loop {
            if mop_len > n2 {
                i = mop_len;
                loop {
                    pgno = *mop.offset(i as isize);
                    if *mop.offset(i.wrapping_sub(n2) as isize)
                        == pgno.wrapping_add(n2 as pgno_t)
                    {
                        current_block = 91249680650791448;
                        break 's_140;
                    }
                    i = i.wrapping_sub(1);
                    if !(i > n2) {
                        break;
                    }
                }
                retry -= 1;
                if retry < 0 as libc::c_int {
                    current_block = 479107131381816815;
                    break;
                }
            }
            if op as libc::c_uint == 0 as libc::c_uint {
                last = (*env).me_pgstate.mf_pglast;
                oldest = (*env).me_pgoldest;
                mdb_cursor_init(
                    &mut m2,
                    txn,
                    0 as libc::c_int as MDB_dbi,
                    0 as *mut libc::c_void as *mut MDB_xcursor,
                );
                if last != 0 {
                    op = MDB_SET_RANGE;
                    key.mv_data = &mut last as *mut txnid_t as *mut libc::c_void;
                    key.mv_size = ::std::mem::size_of::<txnid_t>() as libc::c_ulong;
                }
            }
            last = last.wrapping_add(1);
            if oldest <= last {
                if found_old == 0 {
                    oldest = mdb_find_oldest(txn);
                    (*env).me_pgoldest = oldest;
                    found_old = 1 as libc::c_int;
                }
                if oldest <= last {
                    current_block = 479107131381816815;
                    break;
                }
            }
            rc = mdb_cursor_get(
                &mut m2,
                &mut key,
                0 as *mut libc::c_void as *mut MDB_val,
                op,
            );
            if rc != 0 {
                if rc == -(30798 as libc::c_int) {
                    current_block = 479107131381816815;
                    break;
                } else {
                    current_block = 3455911788394277335;
                    break;
                }
            } else {
                last = *(key.mv_data as *mut txnid_t);
                if oldest <= last {
                    if found_old == 0 {
                        oldest = mdb_find_oldest(txn);
                        (*env).me_pgoldest = oldest;
                        found_old = 1 as libc::c_int;
                    }
                    if oldest <= last {
                        current_block = 479107131381816815;
                        break;
                    }
                }
                np = m2.mc_pg[m2.mc_top as usize];
                leaf = (np as *mut libc::c_char)
                    .offset(
                        *((*np).mp_ptrs)
                            .as_mut_ptr()
                            .offset(m2.mc_ki[m2.mc_top as usize] as isize) as libc::c_int
                            as isize,
                    )
                    .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                rc = mdb_node_read(&mut m2, leaf, &mut data);
                if rc != 0 as libc::c_int {
                    current_block = 3455911788394277335;
                    break;
                }
                idl = data.mv_data as *mut MDB_ID;
                i = *idl.offset(0 as libc::c_int as isize) as libc::c_uint;
                if mop.is_null() {
                    mop = mdb_midl_alloc(i as libc::c_int);
                    tmp___0 = mop;
                    (*env).me_pgstate.mf_pghead = tmp___0;
                    if tmp___0.is_null() {
                        rc = 12 as libc::c_int;
                        current_block = 3455911788394277335;
                        break;
                    }
                } else {
                    rc = mdb_midl_need(&mut (*env).me_pgstate.mf_pghead, i);
                    if rc != 0 as libc::c_int {
                        current_block = 3455911788394277335;
                        break;
                    }
                    mop = (*env).me_pgstate.mf_pghead;
                }
                (*env).me_pgstate.mf_pglast = last;
                mdb_midl_xmerge(mop, idl);
                mop_len = *mop.offset(0 as libc::c_int as isize) as libc::c_uint;
                op = MDB_NEXT;
            }
        }
        match current_block {
            3455911788394277335 => {}
            _ => {
                match current_block {
                    479107131381816815 => {
                        i = 0 as libc::c_uint;
                        pgno = (*txn).mt_next_pgno;
                        if pgno.wrapping_add(num as pgno_t) >= (*env).me_maxpg {
                            rc = -(30792 as libc::c_int);
                            current_block = 3455911788394277335;
                        } else {
                            current_block = 91249680650791448;
                        }
                    }
                    _ => {}
                }
                match current_block {
                    3455911788394277335 => {}
                    _ => {
                        if (*env).me_flags & 524288 as libc::c_uint != 0 {
                            np = ((*env).me_map)
                                .offset(
                                    ((*env).me_psize as pgno_t).wrapping_mul(pgno) as isize,
                                ) as *mut MDB_page;
                            current_block = 15864857819291709765;
                        } else {
                            np = mdb_page_malloc(txn, num as libc::c_uint);
                            if np.is_null() {
                                rc = 12 as libc::c_int;
                                current_block = 3455911788394277335;
                            } else {
                                current_block = 15864857819291709765;
                            }
                        }
                        match current_block {
                            3455911788394277335 => {}
                            _ => {
                                if i != 0 {
                                    mop_len = mop_len.wrapping_sub(num as libc::c_uint);
                                    *mop.offset(0 as libc::c_int as isize) = mop_len as pgno_t;
                                    j = i.wrapping_sub(num as libc::c_uint);
                                    while j < mop_len {
                                        j = j.wrapping_add(1);
                                        i = i.wrapping_add(1);
                                        *mop.offset(j as isize) = *mop.offset(i as isize);
                                    }
                                } else {
                                    (*txn).mt_next_pgno = pgno.wrapping_add(num as pgno_t);
                                }
                                (*np).mp_p.p_pgno = pgno;
                                mdb_page_dirty(txn, np);
                                *mp = np;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                }
            }
        }
    }
    (*txn).mt_flags |= 2 as libc::c_uint;
    return rc;
}
unsafe extern "C" fn mdb_page_copy(
    mut dst: *mut MDB_page,
    mut src: *mut MDB_page,
    mut psize: libc::c_uint,
) {
    let mut upper: indx_t = 0;
    let mut lower: indx_t = 0;
    let mut unused: indx_t = 0;
    upper = (*src).mp_pb.pb.pb_upper;
    lower = (*src).mp_pb.pb.pb_lower;
    unused = (upper as libc::c_int - lower as libc::c_int) as indx_t;
    unused = (unused as libc::c_int & -(8 as libc::c_int)) as indx_t;
    if unused != 0 {
        if !((*src).mp_flags as libc::c_int & 32 as libc::c_int == 32 as libc::c_int) {
            upper = (upper as libc::c_uint & 4294967288 as libc::c_uint) as indx_t;
            memcpy(
                dst as *mut libc::c_void,
                src as *const libc::c_void,
                ((lower as libc::c_uint).wrapping_add(7 as libc::c_uint)
                    & 4294967288 as libc::c_uint) as size_t,
            );
            memcpy(
                (dst as *mut libc::c_char).offset(upper as libc::c_int as isize)
                    as *mut pgno_t as *mut libc::c_void,
                (src as *mut libc::c_char).offset(upper as libc::c_int as isize)
                    as *mut pgno_t as *const libc::c_void,
                psize.wrapping_sub(upper as libc::c_uint) as size_t,
            );
        } else {
            memcpy(
                dst as *mut libc::c_void,
                src as *const libc::c_void,
                psize.wrapping_sub(unused as libc::c_uint) as size_t,
            );
        }
    } else {
        memcpy(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            psize.wrapping_sub(unused as libc::c_uint) as size_t,
        );
    };
}
unsafe extern "C" fn mdb_page_unspill(
    mut txn: *mut MDB_txn,
    mut mp: *mut MDB_page,
    mut ret: *mut *mut MDB_page,
) -> libc::c_int {
    let mut env: *mut MDB_env = 0 as *mut MDB_env;
    let mut tx2: *const MDB_txn = 0 as *const MDB_txn;
    let mut x: libc::c_uint = 0;
    let mut pgno: pgno_t = 0;
    let mut pn: pgno_t = 0;
    let mut np: *mut MDB_page = 0 as *mut MDB_page;
    let mut num: libc::c_int = 0;
    env = (*txn).mt_env;
    pgno = (*mp).mp_p.p_pgno;
    pn = pgno << 1 as libc::c_int;
    tx2 = txn as *const MDB_txn;
    while !tx2.is_null() {
        if !((*tx2).mt_spill_pgs).is_null() {
            x = mdb_midl_search((*tx2).mt_spill_pgs, pn);
            if x as MDB_ID <= *((*tx2).mt_spill_pgs).offset(0 as libc::c_int as isize) {
                if *((*tx2).mt_spill_pgs).offset(x as isize) == pn {
                    if (*txn).mt_dirty_room == 0 as libc::c_uint {
                        return -(30788 as libc::c_int);
                    }
                    if (*mp).mp_flags as libc::c_int & 4 as libc::c_int
                        == 4 as libc::c_int
                    {
                        num = (*mp).mp_pb.pb_pages as libc::c_int;
                    } else {
                        num = 1 as libc::c_int;
                    }
                    if (*env).me_flags & 524288 as libc::c_uint != 0 {
                        np = mp;
                    } else {
                        np = mdb_page_malloc(txn, num as libc::c_uint);
                        if np.is_null() {
                            return 12 as libc::c_int;
                        }
                        if num > 1 as libc::c_int {
                            memcpy(
                                np as *mut libc::c_void,
                                mp as *const libc::c_void,
                                (num as libc::c_uint).wrapping_mul((*env).me_psize)
                                    as size_t,
                            );
                        } else {
                            mdb_page_copy(np, mp, (*env).me_psize);
                        }
                    }
                    if tx2 as libc::c_ulong == txn as libc::c_ulong {
                        if x as MDB_ID
                            == *((*txn).mt_spill_pgs).offset(0 as libc::c_int as isize)
                        {
                            let ref mut fresh1 = *((*txn).mt_spill_pgs)
                                .offset(0 as libc::c_int as isize);
                            *fresh1 = (*fresh1).wrapping_sub(1);
                        } else {
                            let ref mut fresh2 = *((*txn).mt_spill_pgs)
                                .offset(x as isize);
                            *fresh2 |= 1 as libc::c_ulong;
                        }
                    }
                    mdb_page_dirty(txn, np);
                    (*np)
                        .mp_flags = ((*np).mp_flags as libc::c_int | 16 as libc::c_int)
                        as uint16_t;
                    *ret = np;
                    break;
                }
            }
        }
        tx2 = (*tx2).mt_parent as *const MDB_txn;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_page_touch(mut mc: *mut MDB_cursor) -> libc::c_int {
    let mut current_block: u64;
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut np: *mut MDB_page = 0 as *mut MDB_page;
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut m2: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut m3: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut pgno: pgno_t = 0;
    let mut rc: libc::c_int = 0;
    let mut xidl: *mut MDB_ID = 0 as *mut MDB_ID;
    let mut xlen: MDB_ID = 0;
    let mut parent: *mut MDB_page = 0 as *mut MDB_page;
    let mut node: *mut MDB_node = 0 as *mut MDB_node;
    let mut mid: MDB_ID2 = MDB_ID2 {
        mid: 0,
        mptr: 0 as *mut libc::c_void,
    };
    let mut dl: *mut MDB_ID2 = 0 as *mut MDB_ID2;
    let mut x: libc::c_uint = 0;
    let mut tmp: libc::c_uint = 0;
    let mut xr_pg: *mut MDB_page = 0 as *mut MDB_page;
    let mut xr_node: *mut MDB_node = 0 as *mut MDB_node;
    mp = (*mc).mc_pg[(*mc).mc_top as usize];
    txn = (*mc).mc_txn;
    if !((*mp).mp_flags as libc::c_int & 16 as libc::c_int == 16 as libc::c_int) {
        if (*txn).mt_flags & 8 as libc::c_uint != 0 {
            np = 0 as *mut libc::c_void as *mut MDB_page;
            rc = mdb_page_unspill(txn, mp, &mut np);
            if rc != 0 {
                current_block = 10159888946499633837;
            } else if !np.is_null() {
                current_block = 3939203112607292616;
            } else {
                current_block = 12599329904712511516;
            }
        } else {
            current_block = 12599329904712511516;
        }
        match current_block {
            3939203112607292616 => {}
            _ => {
                match current_block {
                    12599329904712511516 => {
                        rc = mdb_midl_need(&mut (*txn).mt_free_pgs, 1 as libc::c_uint);
                        if rc != 0 {
                            current_block = 10159888946499633837;
                        } else {
                            rc = mdb_page_alloc(mc, 1 as libc::c_int, &mut np);
                            if rc != 0 {
                                current_block = 10159888946499633837;
                            } else {
                                pgno = (*np).mp_p.p_pgno;
                                if !((*mp).mp_p.p_pgno != pgno) {
                                    mdb_assert_fail(
                                        (*(*mc).mc_txn).mt_env,
                                        b"mp->mp_pgno != pgno\0" as *const u8
                                            as *const libc::c_char,
                                        b"mdb_page_touch\0" as *const u8 as *const libc::c_char,
                                        b"mdb.c\0" as *const u8 as *const libc::c_char,
                                        2741 as libc::c_int,
                                    );
                                }
                                xidl = (*txn).mt_free_pgs;
                                let ref mut fresh3 = *xidl
                                    .offset(0 as libc::c_int as isize);
                                *fresh3 = (*fresh3).wrapping_add(1);
                                xlen = *xidl.offset(0 as libc::c_int as isize);
                                *xidl.offset(xlen as isize) = (*mp).mp_p.p_pgno;
                                if (*mc).mc_top != 0 {
                                    parent = (*mc)
                                        .mc_pg[((*mc).mc_top as libc::c_int - 1 as libc::c_int)
                                        as usize];
                                    node = (parent as *mut libc::c_char)
                                        .offset(
                                            *((*parent).mp_ptrs)
                                                .as_mut_ptr()
                                                .offset(
                                                    (*mc)
                                                        .mc_ki[((*mc).mc_top as libc::c_int - 1 as libc::c_int)
                                                        as usize] as isize,
                                                ) as libc::c_int as isize,
                                        )
                                        .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                                    (*node)
                                        .mn_lo = (pgno & 65535 as libc::c_ulong) as libc::c_ushort;
                                    (*node)
                                        .mn_hi = (pgno >> 16 as libc::c_int) as libc::c_ushort;
                                    (*node)
                                        .mn_flags = (pgno >> 32 as libc::c_int) as libc::c_ushort;
                                } else {
                                    (*(*mc).mc_db).md_root = pgno;
                                }
                                current_block = 2706659501864706830;
                            }
                        }
                    }
                    _ => {}
                }
                match current_block {
                    2706659501864706830 => {}
                    _ => {
                        (*txn).mt_flags |= 2 as libc::c_uint;
                        return rc;
                    }
                }
            }
        }
    } else {
        if !((*txn).mt_parent).is_null() {
            if !((*mp).mp_flags as libc::c_int & 64 as libc::c_int == 64 as libc::c_int)
            {
                dl = (*txn).mt_u.dirty_list;
                pgno = (*mp).mp_p.p_pgno;
                if (*dl.offset(0 as libc::c_int as isize)).mid != 0 {
                    tmp = mdb_mid2l_search(dl, pgno);
                    x = tmp;
                    if x as MDB_ID <= (*dl.offset(0 as libc::c_int as isize)).mid {
                        if (*dl.offset(x as isize)).mid == pgno {
                            if mp as libc::c_ulong
                                != (*dl.offset(x as isize)).mptr as libc::c_ulong
                            {
                                (*mc).mc_flags &= 4294967292 as libc::c_uint;
                                (*txn).mt_flags |= 2 as libc::c_uint;
                                return -(30779 as libc::c_int);
                            }
                            return 0 as libc::c_int;
                        }
                    }
                }
                if !((*dl.offset(0 as libc::c_int as isize)).mid
                    < (((1 as libc::c_int) << 17 as libc::c_int) - 1 as libc::c_int)
                        as MDB_ID)
                {
                    mdb_assert_fail(
                        (*(*mc).mc_txn).mt_env,
                        b"dl[0].mid < MDB_IDL_UM_MAX\0" as *const u8
                            as *const libc::c_char,
                        b"mdb_page_touch\0" as *const u8 as *const libc::c_char,
                        b"mdb.c\0" as *const u8 as *const libc::c_char,
                        2768 as libc::c_int,
                    );
                }
                np = mdb_page_malloc(txn, 1 as libc::c_uint);
                if np.is_null() {
                    return 12 as libc::c_int;
                }
                mid.mid = pgno;
                mid.mptr = np as *mut libc::c_void;
                rc = mdb_mid2l_insert(dl, &mut mid);
                if !(rc == 0 as libc::c_int) {
                    mdb_assert_fail(
                        (*(*mc).mc_txn).mt_env,
                        b"rc == 0\0" as *const u8 as *const libc::c_char,
                        b"mdb_page_touch\0" as *const u8 as *const libc::c_char,
                        b"mdb.c\0" as *const u8 as *const libc::c_char,
                        2776 as libc::c_int,
                    );
                }
            } else {
                return 0 as libc::c_int
            }
        } else {
            return 0 as libc::c_int
        }
        current_block = 2706659501864706830;
    }
    match current_block {
        2706659501864706830 => {
            mdb_page_copy(np, mp, (*(*txn).mt_env).me_psize);
            (*np).mp_p.p_pgno = pgno;
            (*np)
                .mp_flags = ((*np).mp_flags as libc::c_int | 16 as libc::c_int)
                as uint16_t;
        }
        _ => {}
    }
    (*mc).mc_pg[(*mc).mc_top as usize] = np;
    m2 = *((*txn).mt_cursors).offset((*mc).mc_dbi as isize);
    if (*mc).mc_flags & 4 as libc::c_uint != 0 {
        while !m2.is_null() {
            m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
            if !(((*m3).mc_snum as libc::c_int) < (*mc).mc_snum as libc::c_int) {
                if (*m3).mc_pg[(*mc).mc_top as usize] as libc::c_ulong
                    == mp as libc::c_ulong
                {
                    (*m3).mc_pg[(*mc).mc_top as usize] = np;
                }
            }
            m2 = (*m2).mc_next;
        }
    } else {
        while !m2.is_null() {
            if !(((*m2).mc_snum as libc::c_int) < (*mc).mc_snum as libc::c_int) {
                if !(m2 as libc::c_ulong == mc as libc::c_ulong) {
                    if (*m2).mc_pg[(*mc).mc_top as usize] as libc::c_ulong
                        == mp as libc::c_ulong
                    {
                        (*m2).mc_pg[(*mc).mc_top as usize] = np;
                        if (*np).mp_flags as libc::c_int & 2 as libc::c_int
                            == 2 as libc::c_int
                        {
                            xr_pg = np;
                            if !((*m2).mc_xcursor).is_null() {
                                if (*(*m2).mc_xcursor).mx_cursor.mc_flags
                                    & 1 as libc::c_uint != 0
                                {
                                    if !((*m2).mc_ki[(*mc).mc_top as usize] as libc::c_uint
                                        >= ((*xr_pg).mp_pb.pb.pb_lower as libc::c_uint)
                                            .wrapping_sub(
                                                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                                    as libc::c_ulong as libc::c_uint,
                                            ) >> 1 as libc::c_int)
                                    {
                                        xr_node = (xr_pg as *mut libc::c_char)
                                            .offset(
                                                *((*xr_pg).mp_ptrs)
                                                    .as_mut_ptr()
                                                    .offset((*m2).mc_ki[(*mc).mc_top as usize] as isize)
                                                    as libc::c_int as isize,
                                            )
                                            .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                                        if (*xr_node).mn_flags as libc::c_int & 6 as libc::c_int
                                            == 4 as libc::c_int
                                        {
                                            (*(*m2).mc_xcursor)
                                                .mx_cursor
                                                .mc_pg[0 as libc::c_int
                                                as usize] = ((*xr_node).mn_data)
                                                .as_mut_ptr()
                                                .offset((*xr_node).mn_ksize as libc::c_int as isize)
                                                as *mut libc::c_void as *mut MDB_page;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            m2 = (*m2).mc_next;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_env_sync0(
    mut env: *mut MDB_env,
    mut force: libc::c_int,
    mut numpgs: pgno_t,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___5: libc::c_int = 0;
    rc = 0 as libc::c_int;
    if (*env).me_flags & 131072 as libc::c_uint != 0 {
        return 13 as libc::c_int;
    }
    let mut current_block_33: u64;
    if force != 0 {
        current_block_33 = 14302937838250073426;
    } else if (*env).me_flags & 65536 as libc::c_uint == 0 {
        current_block_33 = 14302937838250073426;
    } else {
        current_block_33 = 11913429853522160501;
    }
    match current_block_33 {
        14302937838250073426 => {
            if (*env).me_flags & 524288 as libc::c_uint != 0 {
                if (*env).me_flags & 1048576 as libc::c_uint != 0 {
                    if force == 0 {
                        tmp = 1 as libc::c_int;
                    } else {
                        tmp = 4 as libc::c_int;
                    }
                } else {
                    tmp = 4 as libc::c_int;
                }
                flags = tmp;
                tmp___1 = msync(
                    (*env).me_map as *mut libc::c_void,
                    ((*env).me_psize as pgno_t).wrapping_mul(numpgs),
                    flags,
                );
                if tmp___1 != 0 {
                    tmp___0 = __errno_location();
                    rc = *tmp___0;
                }
            } else if (*env).me_flags & 134217728 as libc::c_uint != 0 {
                tmp___3 = fsync((*env).me_fd);
                if tmp___3 != 0 {
                    tmp___2 = __errno_location();
                    rc = *tmp___2;
                }
            } else {
                tmp___5 = fdatasync((*env).me_fd);
                if tmp___5 != 0 {
                    tmp___4 = __errno_location();
                    rc = *tmp___4;
                }
            }
        }
        _ => {}
    }
    return rc;
}
pub unsafe extern "C" fn mdb_env_sync(
    mut env: *mut MDB_env,
    mut force: libc::c_int,
) -> libc::c_int {
    let mut m: *mut MDB_meta = 0 as *mut MDB_meta;
    let mut tmp: *mut MDB_meta = 0 as *mut MDB_meta;
    let mut tmp___0: libc::c_int = 0;
    tmp = mdb_env_pick_meta(env as *const MDB_env);
    m = tmp;
    tmp___0 = mdb_env_sync0(
        env,
        force,
        ((*m).mm_last_pg).wrapping_add(1 as libc::c_ulong),
    );
    return tmp___0;
}
unsafe extern "C" fn mdb_cursor_shadow(
    mut src: *mut MDB_txn,
    mut dst: *mut MDB_txn,
) -> libc::c_int {
    let mut mc: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut bk: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut mx: *mut MDB_xcursor = 0 as *mut MDB_xcursor;
    let mut size: size_t = 0;
    let mut i: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    i = (*src).mt_numdbs as libc::c_int;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        mc = *((*src).mt_cursors).offset(i as isize);
        if mc as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            size = ::std::mem::size_of::<MDB_cursor>() as libc::c_ulong;
            if !((*mc).mc_xcursor).is_null() {
                size = (size as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<MDB_xcursor>() as libc::c_ulong)
                    as size_t as size_t;
            }
            while !mc.is_null() {
                tmp = malloc(size);
                bk = tmp as *mut MDB_cursor;
                if bk.is_null() {
                    return 12 as libc::c_int;
                }
                *bk = *mc;
                (*mc).mc_backup = bk;
                (*mc).mc_db = ((*dst).mt_dbs).offset(i as isize);
                (*mc).mc_txn = dst;
                (*mc).mc_dbflag = ((*dst).mt_dbflags).offset(i as isize);
                mx = (*mc).mc_xcursor;
                if mx as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                    *(bk.offset(1 as libc::c_int as isize) as *mut MDB_xcursor) = *mx;
                    (*mx).mx_cursor.mc_txn = dst;
                }
                (*mc).mc_next = *((*dst).mt_cursors).offset(i as isize);
                let ref mut fresh4 = *((*dst).mt_cursors).offset(i as isize);
                *fresh4 = mc;
                mc = (*bk).mc_next;
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_cursors_close(mut txn: *mut MDB_txn, mut merge: libc::c_uint) {
    let mut cursors: *mut *mut MDB_cursor = 0 as *mut *mut MDB_cursor;
    let mut mc: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut next: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut bk: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut mx: *mut MDB_xcursor = 0 as *mut MDB_xcursor;
    let mut i: libc::c_int = 0;
    cursors = (*txn).mt_cursors;
    i = (*txn).mt_numdbs as libc::c_int;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        mc = *cursors.offset(i as isize);
        while !mc.is_null() {
            next = (*mc).mc_next;
            bk = (*mc).mc_backup;
            if bk as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                if merge != 0 {
                    (*mc).mc_next = (*bk).mc_next;
                    (*mc).mc_backup = (*bk).mc_backup;
                    (*mc).mc_txn = (*bk).mc_txn;
                    (*mc).mc_db = (*bk).mc_db;
                    (*mc).mc_dbflag = (*bk).mc_dbflag;
                    mx = (*mc).mc_xcursor;
                    if mx as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                        (*mx).mx_cursor.mc_txn = (*bk).mc_txn;
                    }
                } else {
                    *mc = *bk;
                    mx = (*mc).mc_xcursor;
                    if mx as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                        *mx = *(bk.offset(1 as libc::c_int as isize)
                            as *mut MDB_xcursor);
                    }
                }
                mc = bk;
            }
            free(mc as *mut libc::c_void);
            mc = next;
        }
        let ref mut fresh5 = *cursors.offset(i as isize);
        *fresh5 = 0 as *mut libc::c_void as *mut MDB_cursor;
    };
}
unsafe extern "C" fn mdb_reader_pid(
    mut env: *mut MDB_env,
    mut op: Pidlock_op,
    mut pid: pid_t,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut lock_info: flock = flock {
        l_type: 0,
        l_whence: 0,
        l_start: 0,
        l_len: 0,
        l_pid: 0,
    };
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    loop {
        's_85: {
            memset(
                &mut lock_info as *mut flock as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<flock>() as libc::c_ulong,
            );
            lock_info.l_type = 1 as libc::c_int as libc::c_short;
            lock_info.l_whence = 0 as libc::c_int as libc::c_short;
            lock_info.l_start = pid as __off_t;
            lock_info.l_len = 1 as libc::c_int as __off_t;
            rc = fcntl((*env).me_lfd, op as libc::c_int, &mut lock_info as *mut flock);
            if rc == 0 as libc::c_int {
                if op as libc::c_uint == 5 as libc::c_uint {
                    if lock_info.l_type as libc::c_int != 2 as libc::c_int {
                        rc = -(1 as libc::c_int);
                    }
                }
            } else {
                tmp = __errno_location();
                rc = *tmp;
                if rc == 4 as libc::c_int {
                    break 's_85;
                }
            }
            return rc;
        }
    };
}
unsafe extern "C" fn mdb_txn_renew0(mut txn: *mut MDB_txn) -> libc::c_int {
    let mut env: *mut MDB_env = 0 as *mut MDB_env;
    let mut ti: *mut MDB_txninfo = 0 as *mut MDB_txninfo;
    let mut meta: *mut MDB_meta = 0 as *mut MDB_meta;
    let mut i: libc::c_uint = 0;
    let mut nr: libc::c_uint = 0;
    let mut flags: libc::c_uint = 0;
    let mut x: uint16_t = 0;
    let mut rc: libc::c_int = 0;
    let mut new_notls: libc::c_int = 0;
    let mut r: *mut MDB_reader = 0 as *mut MDB_reader;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut pid: pid_t = 0;
    let mut tid: pthread_t = 0;
    let mut tmp___1: pthread_t = 0;
    let mut rmutex: mdb_mutexref_t = 0 as *mut pthread_mutex_t;
    env = (*txn).mt_env;
    ti = (*env).me_txns;
    flags = (*txn).mt_flags;
    new_notls = 0 as libc::c_int;
    flags &= 131072 as libc::c_uint;
    if flags != 0 as libc::c_uint {
        if ti.is_null() {
            meta = mdb_env_pick_meta(env as *const MDB_env);
            (*txn).mt_txnid = (*meta).mm_txnid;
            (*txn).mt_u.reader = 0 as *mut libc::c_void as *mut MDB_reader;
        } else {
            if (*env).me_flags & 2097152 as libc::c_uint != 0 {
                tmp___0 = (*txn).mt_u.reader as *mut libc::c_void;
            } else {
                tmp = pthread_getspecific((*env).me_txkey);
                tmp___0 = tmp;
            }
            r = tmp___0 as *mut MDB_reader;
            if !r.is_null() {
                if (*r).mru.mrx.mrb_pid != (*env).me_pid {
                    return -(30783 as libc::c_int)
                } else {
                    if (*r).mru.mrx.mrb_txnid != 0xffffffffffffffff as libc::c_ulong {
                        return -(30783 as libc::c_int);
                    }
                }
            } else {
                pid = (*env).me_pid;
                tmp___1 = pthread_self();
                tid = tmp___1;
                rmutex = ((*(*env).me_txns).mt1.mtb.mtb_rmutex).as_mut_ptr();
                if (*env).me_live_reader == 0 {
                    rc = mdb_reader_pid(env, Pidset, pid);
                    if rc != 0 {
                        return rc;
                    }
                    (*env).me_live_reader = 1 as libc::c_int;
                }
                rc = pthread_mutex_lock(rmutex);
                if rc != 0 {
                    rc = mdb_mutex_failed(env, rmutex, rc);
                    if rc != 0 {
                        return rc;
                    }
                }
                nr = (*ti).mt1.mtb.mtb_numreaders;
                i = 0 as libc::c_uint;
                while i < nr {
                    if (*((*ti).mti_readers).as_mut_ptr().offset(i as isize))
                        .mru
                        .mrx
                        .mrb_pid == 0 as libc::c_int
                    {
                        break;
                    }
                    i = i.wrapping_add(1);
                }
                if i == (*env).me_maxreaders {
                    pthread_mutex_unlock(rmutex);
                    return -(30790 as libc::c_int);
                }
                r = &mut *((*ti).mti_readers).as_mut_ptr().offset(i as isize)
                    as *mut MDB_reader;
                (*r).mru.mrx.mrb_pid = 0 as libc::c_int;
                (*r).mru.mrx.mrb_txnid = -(1 as libc::c_int) as txnid_t;
                (*r).mru.mrx.mrb_tid = tid;
                if i == nr {
                    nr = nr.wrapping_add(1);
                    (*ti).mt1.mtb.mtb_numreaders = nr;
                }
                (*env).me_close_readers = nr as libc::c_int;
                (*r).mru.mrx.mrb_pid = pid;
                pthread_mutex_unlock(rmutex);
                new_notls = ((*env).me_flags & 2097152 as libc::c_uint) as libc::c_int;
                if new_notls == 0 {
                    rc = pthread_setspecific((*env).me_txkey, r as *const libc::c_void);
                    if rc != 0 {
                        (*r).mru.mrx.mrb_pid = 0 as libc::c_int;
                        return rc;
                    }
                }
            }
            loop {
                (*r).mru.mrx.mrb_txnid = (*ti).mt1.mtb.mtb_txnid;
                if !((*r).mru.mrx.mrb_txnid != (*ti).mt1.mtb.mtb_txnid) {
                    break;
                }
            }
            (*txn).mt_txnid = (*r).mru.mrx.mrb_txnid;
            (*txn).mt_u.reader = r;
            meta = (*env).me_metas[((*txn).mt_txnid & 1 as libc::c_ulong) as usize];
        }
    } else {
        if !ti.is_null() {
            rc = pthread_mutex_lock(((*(*env).me_txns).mt2.mt2_wmutex).as_mut_ptr());
            if rc != 0 {
                rc = mdb_mutex_failed(
                    env,
                    ((*(*env).me_txns).mt2.mt2_wmutex).as_mut_ptr(),
                    rc,
                );
                if rc != 0 {
                    return rc;
                }
            }
            (*txn).mt_txnid = (*ti).mt1.mtb.mtb_txnid;
            meta = (*env).me_metas[((*txn).mt_txnid & 1 as libc::c_ulong) as usize];
        } else {
            meta = mdb_env_pick_meta(env as *const MDB_env);
            (*txn).mt_txnid = (*meta).mm_txnid;
        }
        (*txn).mt_txnid = ((*txn).mt_txnid).wrapping_add(1);
        (*txn).mt_child = 0 as *mut libc::c_void as *mut MDB_txn;
        (*txn).mt_loose_pgs = 0 as *mut libc::c_void as *mut MDB_page;
        (*txn).mt_loose_count = 0 as libc::c_int;
        (*txn)
            .mt_dirty_room = (((1 as libc::c_int) << 17 as libc::c_int)
            - 1 as libc::c_int) as libc::c_uint;
        (*txn).mt_u.dirty_list = (*env).me_dirty_list;
        (*((*txn).mt_u.dirty_list).offset(0 as libc::c_int as isize))
            .mid = 0 as libc::c_int as MDB_ID;
        (*txn).mt_free_pgs = (*env).me_free_pgs;
        *((*txn).mt_free_pgs)
            .offset(0 as libc::c_int as isize) = 0 as libc::c_int as MDB_ID;
        (*txn).mt_spill_pgs = 0 as *mut libc::c_void as MDB_IDL;
        (*env).me_txn = txn;
        memcpy(
            (*txn).mt_dbiseqs as *mut libc::c_void,
            (*env).me_dbiseqs as *const libc::c_void,
            ((*env).me_maxdbs as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
        );
    }
    memcpy(
        (*txn).mt_dbs as *mut libc::c_void,
        ((*meta).mm_dbs).as_mut_ptr() as *const libc::c_void,
        (2 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<MDB_db>() as libc::c_ulong),
    );
    (*txn).mt_next_pgno = ((*meta).mm_last_pg).wrapping_add(1 as libc::c_ulong);
    (*txn).mt_flags = flags;
    (*txn).mt_numdbs = (*env).me_numdbs;
    i = 2 as libc::c_uint;
    while i < (*txn).mt_numdbs {
        x = *((*env).me_dbflags).offset(i as isize);
        (*((*txn).mt_dbs).offset(i as isize))
            .md_flags = (x as libc::c_int & 32767 as libc::c_int) as uint16_t;
        if x as libc::c_int & 32768 as libc::c_int != 0 {
            *((*txn).mt_dbflags).offset(i as isize) = 26 as libc::c_int as libc::c_uchar;
        } else {
            *((*txn).mt_dbflags).offset(i as isize) = 0 as libc::c_int as libc::c_uchar;
        }
        i = i.wrapping_add(1);
    }
    *((*txn).mt_dbflags)
        .offset(1 as libc::c_int as isize) = 24 as libc::c_int as libc::c_uchar;
    *((*txn).mt_dbflags)
        .offset(0 as libc::c_int as isize) = 8 as libc::c_int as libc::c_uchar;
    if (*env).me_flags & 2147483648 as libc::c_uint != 0 {
        rc = -(30795 as libc::c_int);
    } else if (*env).me_maxpg < (*txn).mt_next_pgno {
        rc = -(30785 as libc::c_int);
    } else {
        return 0 as libc::c_int
    }
    mdb_txn_end(txn, (new_notls | 5 as libc::c_int) as libc::c_uint);
    return rc;
}
pub unsafe extern "C" fn mdb_txn_renew(mut txn: *mut MDB_txn) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    if txn.is_null() {
        return 22 as libc::c_int
    } else {
        if !((*txn).mt_flags & 131073 as libc::c_uint == 131073 as libc::c_uint) {
            return 22 as libc::c_int;
        }
    }
    rc = mdb_txn_renew0(txn);
    return rc;
}
pub unsafe extern "C" fn mdb_txn_begin(
    mut env: *mut MDB_env,
    mut parent: *mut MDB_txn,
    mut flags: libc::c_uint,
    mut ret: *mut *mut MDB_txn,
) -> libc::c_int {
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut ntxn: *mut MDB_ntxn = 0 as *mut MDB_ntxn;
    let mut rc: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut tsize: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_uint = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: MDB_IDL = 0 as *mut MDB_ID;
    let mut current_block_94: u64;
    flags &= 458752 as libc::c_uint;
    flags |= (*env).me_flags & 524288 as libc::c_uint;
    if (*env).me_flags & 131072 as libc::c_uint & !flags != 0 {
        return 13 as libc::c_int;
    }
    if !parent.is_null() {
        flags |= (*parent).mt_flags;
        if flags & 655379 as libc::c_uint != 0 {
            if (*parent).mt_flags & 131072 as libc::c_uint != 0 {
                tmp = 22 as libc::c_int;
            } else {
                tmp = -(30782 as libc::c_int);
            }
            return tmp;
        }
        size = ((*env).me_maxdbs as libc::c_ulong)
            .wrapping_mul(
                (::std::mem::size_of::<MDB_db>() as libc::c_ulong)
                    .wrapping_add(
                        ::std::mem::size_of::<*mut MDB_cursor>() as libc::c_ulong,
                    )
                    .wrapping_add(1 as libc::c_ulong),
            ) as libc::c_int;
        tsize = ::std::mem::size_of::<MDB_ntxn>() as libc::c_ulong as libc::c_int;
        size += tsize;
        current_block_94 = 11042950489265723346;
    } else if flags & 131072 as libc::c_uint != 0 {
        size = ((*env).me_maxdbs as libc::c_ulong)
            .wrapping_mul(
                (::std::mem::size_of::<MDB_db>() as libc::c_ulong)
                    .wrapping_add(1 as libc::c_ulong),
            ) as libc::c_int;
        tsize = ::std::mem::size_of::<MDB_txn>() as libc::c_ulong as libc::c_int;
        size += tsize;
        current_block_94 = 11042950489265723346;
    } else {
        txn = (*env).me_txn0;
        current_block_94 = 14441142177199468829;
    }
    match current_block_94 {
        11042950489265723346 => {
            tmp___0 = calloc(1 as libc::c_int as size_t, size as size_t);
            txn = tmp___0 as *mut MDB_txn;
            if txn as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                return 12 as libc::c_int;
            }
            (*txn).mt_dbxs = (*env).me_dbxs;
            (*txn)
                .mt_dbs = (txn as *mut libc::c_char).offset(tsize as isize)
                as *mut MDB_db;
            (*txn)
                .mt_dbflags = (txn as *mut libc::c_uchar)
                .offset(size as isize)
                .offset(-((*env).me_maxdbs as isize));
            (*txn).mt_flags = flags;
            (*txn).mt_env = env;
            if !parent.is_null() {
                (*txn)
                    .mt_cursors = ((*txn).mt_dbs).offset((*env).me_maxdbs as isize)
                    as *mut *mut MDB_cursor;
                (*txn).mt_dbiseqs = (*parent).mt_dbiseqs;
                tmp___1 = malloc(
                    (::std::mem::size_of::<MDB_ID2>() as libc::c_ulong)
                        .wrapping_mul(
                            ((1 as libc::c_int) << 17 as libc::c_int) as libc::c_ulong,
                        ),
                );
                (*txn).mt_u.dirty_list = tmp___1 as MDB_ID2L;
                if ((*txn).mt_u.dirty_list).is_null() {
                    free((*txn).mt_u.dirty_list as *mut libc::c_void);
                    free(txn as *mut libc::c_void);
                    return 12 as libc::c_int;
                } else {
                    tmp___2 = mdb_midl_alloc(
                        ((1 as libc::c_int) << 17 as libc::c_int) - 1 as libc::c_int,
                    );
                    (*txn).mt_free_pgs = tmp___2;
                    if tmp___2.is_null() {
                        free((*txn).mt_u.dirty_list as *mut libc::c_void);
                        free(txn as *mut libc::c_void);
                        return 12 as libc::c_int;
                    }
                }
                (*txn).mt_txnid = (*parent).mt_txnid;
                (*txn).mt_dirty_room = (*parent).mt_dirty_room;
                (*((*txn).mt_u.dirty_list).offset(0 as libc::c_int as isize))
                    .mid = 0 as libc::c_int as MDB_ID;
                (*txn).mt_spill_pgs = 0 as *mut libc::c_void as MDB_IDL;
                (*txn).mt_next_pgno = (*parent).mt_next_pgno;
                (*parent).mt_flags |= 16 as libc::c_uint;
                (*parent).mt_child = txn;
                (*txn).mt_parent = parent;
                (*txn).mt_numdbs = (*parent).mt_numdbs;
                memcpy(
                    (*txn).mt_dbs as *mut libc::c_void,
                    (*parent).mt_dbs as *const libc::c_void,
                    ((*txn).mt_numdbs as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<MDB_db>() as libc::c_ulong),
                );
                i = 0 as libc::c_uint;
                while i < (*txn).mt_numdbs {
                    *((*txn).mt_dbflags)
                        .offset(
                            i as isize,
                        ) = (*((*parent).mt_dbflags).offset(i as isize) as libc::c_int
                        & -(5 as libc::c_int)) as libc::c_uchar;
                    i = i.wrapping_add(1);
                }
                rc = 0 as libc::c_int;
                ntxn = txn as *mut MDB_ntxn;
                (*ntxn).mnt_pgstate = (*env).me_pgstate;
                if !((*env).me_pgstate.mf_pghead).is_null() {
                    size = (*((*env).me_pgstate.mf_pghead)
                        .offset(0 as libc::c_int as isize))
                        .wrapping_add(1 as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<MDB_ID>() as libc::c_ulong)
                        as libc::c_int;
                    (*env)
                        .me_pgstate
                        .mf_pghead = mdb_midl_alloc(
                        *((*env).me_pgstate.mf_pghead).offset(0 as libc::c_int as isize)
                            as libc::c_int,
                    );
                    if !((*env).me_pgstate.mf_pghead).is_null() {
                        memcpy(
                            (*env).me_pgstate.mf_pghead as *mut libc::c_void,
                            (*ntxn).mnt_pgstate.mf_pghead as *const libc::c_void,
                            size as size_t,
                        );
                    } else {
                        rc = 12 as libc::c_int;
                    }
                }
                if rc == 0 {
                    rc = mdb_cursor_shadow(parent, txn);
                }
                if rc != 0 {
                    mdb_txn_end(txn, 6 as libc::c_uint);
                }
                current_block_94 = 15970011996474399071;
            } else {
                (*txn).mt_dbiseqs = (*env).me_dbiseqs;
                current_block_94 = 14441142177199468829;
            }
        }
        _ => {}
    }
    match current_block_94 {
        14441142177199468829 => {
            rc = mdb_txn_renew0(txn);
        }
        _ => {}
    }
    if rc != 0 {
        if txn as libc::c_ulong != (*env).me_txn0 as libc::c_ulong {
            free(txn as *mut libc::c_void);
        }
    } else {
        (*txn).mt_flags |= flags;
        *ret = txn;
    }
    return rc;
}
pub unsafe extern "C" fn mdb_txn_env(mut txn: *mut MDB_txn) -> *mut MDB_env {
    if txn.is_null() {
        return 0 as *mut libc::c_void as *mut MDB_env;
    }
    return (*txn).mt_env;
}
pub unsafe extern "C" fn mdb_txn_id(mut txn: *mut MDB_txn) -> mdb_size_t {
    if txn.is_null() {
        return 0 as libc::c_int as mdb_size_t;
    }
    return (*txn).mt_txnid;
}
unsafe extern "C" fn mdb_dbis_update(mut txn: *mut MDB_txn, mut keep: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut n: MDB_dbi = 0;
    let mut env: *mut MDB_env = 0 as *mut MDB_env;
    let mut tdbflags: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    n = (*txn).mt_numdbs;
    env = (*txn).mt_env;
    tdbflags = (*txn).mt_dbflags;
    i = n as libc::c_int;
    loop {
        i -= 1;
        if !(i >= 2 as libc::c_int) {
            break;
        }
        if *tdbflags.offset(i as isize) as libc::c_int & 4 as libc::c_int != 0 {
            if keep != 0 {
                *((*env).me_dbflags)
                    .offset(
                        i as isize,
                    ) = ((*((*txn).mt_dbs).offset(i as isize)).md_flags as libc::c_int
                    | 32768 as libc::c_int) as uint16_t;
            } else {
                ptr = (*((*env).me_dbxs).offset(i as isize)).md_name.mv_data
                    as *mut libc::c_char;
                if !ptr.is_null() {
                    let ref mut fresh6 = (*((*env).me_dbxs).offset(i as isize))
                        .md_name
                        .mv_data;
                    *fresh6 = 0 as *mut libc::c_void;
                    (*((*env).me_dbxs).offset(i as isize))
                        .md_name
                        .mv_size = 0 as libc::c_int as size_t;
                    *((*env).me_dbflags)
                        .offset(i as isize) = 0 as libc::c_int as uint16_t;
                    let ref mut fresh7 = *((*env).me_dbiseqs).offset(i as isize);
                    *fresh7 = (*fresh7).wrapping_add(1);
                    free(ptr as *mut libc::c_void);
                }
            }
        }
    }
    if keep != 0 {
        if (*env).me_numdbs < n {
            (*env).me_numdbs = n;
        }
    }
}
unsafe extern "C" fn mdb_txn_end(mut txn: *mut MDB_txn, mut mode: libc::c_uint) {
    let mut env: *mut MDB_env = 0 as *mut MDB_env;
    let mut pghead: *mut pgno_t = 0 as *mut pgno_t;
    env = (*txn).mt_env;
    mdb_dbis_update(txn, (mode & 16 as libc::c_uint) as libc::c_int);
    if (*txn).mt_flags & 131072 as libc::c_uint == 131072 as libc::c_uint {
        if !((*txn).mt_u.reader).is_null() {
            (*(*txn).mt_u.reader).mru.mrx.mrb_txnid = -(1 as libc::c_int) as txnid_t;
            if (*env).me_flags & 2097152 as libc::c_uint == 0 {
                (*txn).mt_u.reader = 0 as *mut libc::c_void as *mut MDB_reader;
            } else if mode & 2097152 as libc::c_uint != 0 {
                (*(*txn).mt_u.reader).mru.mrx.mrb_pid = 0 as libc::c_int;
                (*txn).mt_u.reader = 0 as *mut libc::c_void as *mut MDB_reader;
            }
        }
        (*txn).mt_numdbs = 0 as libc::c_int as MDB_dbi;
        (*txn).mt_flags |= 1 as libc::c_uint;
    } else if !((*txn).mt_flags & 1 as libc::c_uint == 1 as libc::c_uint) {
        pghead = (*env).me_pgstate.mf_pghead;
        if mode & 16 as libc::c_uint == 0 {
            mdb_cursors_close(txn, 0 as libc::c_uint);
        }
        if (*env).me_flags & 524288 as libc::c_uint == 0 {
            mdb_dlist_free(txn);
        }
        (*txn).mt_numdbs = 0 as libc::c_int as MDB_dbi;
        (*txn).mt_flags = 1 as libc::c_uint;
        if ((*txn).mt_parent).is_null() {
            mdb_midl_shrink(&mut (*txn).mt_free_pgs);
            (*env).me_free_pgs = (*txn).mt_free_pgs;
            (*env).me_pgstate.mf_pghead = 0 as *mut libc::c_void as *mut pgno_t;
            (*env).me_pgstate.mf_pglast = 0 as libc::c_int as txnid_t;
            (*env).me_txn = 0 as *mut libc::c_void as *mut MDB_txn;
            mode = 0 as libc::c_uint;
            if !((*env).me_txns).is_null() {
                pthread_mutex_unlock(((*(*env).me_txns).mt2.mt2_wmutex).as_mut_ptr());
            }
        } else {
            (*(*txn).mt_parent).mt_child = 0 as *mut libc::c_void as *mut MDB_txn;
            (*(*txn).mt_parent).mt_flags &= 4294967279 as libc::c_uint;
            (*env).me_pgstate = (*(txn as *mut MDB_ntxn)).mnt_pgstate;
            mdb_midl_free((*txn).mt_free_pgs);
            free((*txn).mt_u.dirty_list as *mut libc::c_void);
        }
        mdb_midl_free((*txn).mt_spill_pgs);
        mdb_midl_free(pghead);
    }
    if mode & 32 as libc::c_uint != 0 {
        free(txn as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn mdb_txn_reset(mut txn: *mut MDB_txn) {
    if txn as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    if (*txn).mt_flags & 131072 as libc::c_uint == 0 {
        return;
    }
    mdb_txn_end(txn, 3 as libc::c_uint);
}
pub unsafe extern "C" fn mdb_txn_abort(mut txn: *mut MDB_txn) {
    if txn as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    if !((*txn).mt_child).is_null() {
        mdb_txn_abort((*txn).mt_child);
    }
    mdb_txn_end(txn, 2097186 as libc::c_uint);
}
unsafe extern "C" fn mdb_freelist_save(mut txn: *mut MDB_txn) -> libc::c_int {
    let mut mc: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut env: *mut MDB_env = 0 as *mut MDB_env;
    let mut rc: libc::c_int = 0;
    let mut maxfree_1pg: libc::c_int = 0;
    let mut more: libc::c_int = 0;
    let mut pglast: txnid_t = 0;
    let mut head_id: txnid_t = 0;
    let mut freecnt: pgno_t = 0;
    let mut free_pgs: *mut pgno_t = 0 as *mut pgno_t;
    let mut mop: *mut pgno_t = 0 as *mut pgno_t;
    let mut head_room: ssize_t = 0;
    let mut total_room: ssize_t = 0;
    let mut mop_len: ssize_t = 0;
    let mut clean_limit: ssize_t = 0;
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut dl: *mut MDB_ID2 = 0 as *mut MDB_ID2;
    let mut x: libc::c_uint = 0;
    let mut xidl: *mut MDB_ID = 0 as *mut MDB_ID;
    let mut xlen: MDB_ID = 0;
    let mut y: libc::c_uint = 0;
    let mut tmp: libc::c_uint = 0;
    let mut tmp___0: libc::c_uint = 0;
    let mut key: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut data: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut pgs: *mut pgno_t = 0 as *mut pgno_t;
    let mut j: ssize_t = 0;
    let mut tmp___1: pgno_t = 0;
    let mut mp___0: *mut MDB_page = 0 as *mut MDB_page;
    let mut count: libc::c_uint = 0;
    let mut loose: MDB_IDL = 0 as *mut MDB_ID;
    let mut key___0: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut data___0: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut id: txnid_t = 0;
    let mut len: ssize_t = 0;
    let mut save: MDB_ID = 0;
    env = (*txn).mt_env;
    maxfree_1pg = (*env).me_maxfree_1pg;
    more = 1 as libc::c_int;
    pglast = 0 as libc::c_int as txnid_t;
    head_id = 0 as libc::c_int as txnid_t;
    freecnt = 0 as libc::c_int as pgno_t;
    head_room = 0 as libc::c_int as ssize_t;
    total_room = 0 as libc::c_int as ssize_t;
    mdb_cursor_init(
        &mut mc,
        txn,
        0 as libc::c_int as MDB_dbi,
        0 as *mut libc::c_void as *mut MDB_xcursor,
    );
    if !((*env).me_pgstate.mf_pghead).is_null() {
        rc = mdb_page_search(
            &mut mc,
            0 as *mut libc::c_void as *mut MDB_val,
            5 as libc::c_int,
        );
        if rc != 0 {
            if rc != -(30798 as libc::c_int) {
                return rc;
            }
        }
    }
    if ((*env).me_pgstate.mf_pghead).is_null() {
        if !((*txn).mt_loose_pgs).is_null() {
            mp = (*txn).mt_loose_pgs;
            dl = (*txn).mt_u.dirty_list;
            rc = mdb_midl_need(
                &mut (*txn).mt_free_pgs,
                (*txn).mt_loose_count as libc::c_uint,
            );
            if rc != 0 as libc::c_int {
                return rc;
            }
            while !mp.is_null() {
                xidl = (*txn).mt_free_pgs;
                let ref mut fresh8 = *xidl.offset(0 as libc::c_int as isize);
                *fresh8 = (*fresh8).wrapping_add(1);
                xlen = *xidl.offset(0 as libc::c_int as isize);
                *xidl.offset(xlen as isize) = (*mp).mp_p.p_pgno;
                if (*txn).mt_flags & 524288 as libc::c_uint != 0 {
                    x = 1 as libc::c_uint;
                    while x as MDB_ID <= (*dl.offset(0 as libc::c_int as isize)).mid {
                        if (*dl.offset(x as isize)).mid == (*mp).mp_p.p_pgno {
                            break;
                        }
                        x = x.wrapping_add(1);
                    }
                    if !(x as MDB_ID <= (*dl.offset(0 as libc::c_int as isize)).mid) {
                        mdb_assert_fail(
                            (*txn).mt_env,
                            b"x <= dl[0].mid\0" as *const u8 as *const libc::c_char,
                            b"mdb_freelist_save\0" as *const u8 as *const libc::c_char,
                            b"mdb.c\0" as *const u8 as *const libc::c_char,
                            3476 as libc::c_int,
                        );
                    }
                } else {
                    x = mdb_mid2l_search(dl, (*mp).mp_p.p_pgno);
                    if !((*dl.offset(x as isize)).mid == (*mp).mp_p.p_pgno) {
                        mdb_assert_fail(
                            (*txn).mt_env,
                            b"dl[x].mid == mp->mp_pgno\0" as *const u8
                                as *const libc::c_char,
                            b"mdb_freelist_save\0" as *const u8 as *const libc::c_char,
                            b"mdb.c\0" as *const u8 as *const libc::c_char,
                            3479 as libc::c_int,
                        );
                    }
                    mdb_dpage_free(env, mp);
                }
                let ref mut fresh9 = (*dl.offset(x as isize)).mptr;
                *fresh9 = 0 as *mut libc::c_void;
                mp = *(mp.offset(2 as libc::c_int as isize) as *mut *mut MDB_page);
            }
            y = 1 as libc::c_uint;
            while !((*dl.offset(y as isize)).mptr).is_null() {
                if !(y as MDB_ID <= (*dl.offset(0 as libc::c_int as isize)).mid) {
                    break;
                }
                y = y.wrapping_add(1);
            }
            if y as MDB_ID <= (*dl.offset(0 as libc::c_int as isize)).mid {
                x = y;
                y = y.wrapping_add(1);
                loop {
                    while ((*dl.offset(y as isize)).mptr).is_null() {
                        if !(y as MDB_ID <= (*dl.offset(0 as libc::c_int as isize)).mid)
                        {
                            break;
                        }
                        y = y.wrapping_add(1);
                    }
                    if y as MDB_ID > (*dl.offset(0 as libc::c_int as isize)).mid {
                        break;
                    }
                    tmp = x;
                    x = x.wrapping_add(1);
                    tmp___0 = y;
                    y = y.wrapping_add(1);
                    *dl.offset(tmp as isize) = *dl.offset(tmp___0 as isize);
                }
                (*dl.offset(0 as libc::c_int as isize))
                    .mid = x.wrapping_sub(1 as libc::c_uint) as MDB_ID;
            } else {
                (*dl.offset(0 as libc::c_int as isize)).mid = 0 as libc::c_int as MDB_ID;
            }
            (*txn).mt_loose_pgs = 0 as *mut libc::c_void as *mut MDB_page;
            (*txn).mt_loose_count = 0 as libc::c_int;
        }
    }
    if (*env).me_flags & 17301504 as libc::c_uint != 0 {
        clean_limit = 9223372036854775807 as libc::c_long;
    } else {
        clean_limit = maxfree_1pg as ssize_t;
    }
    loop {
        while pglast < (*env).me_pgstate.mf_pglast {
            rc = mdb_cursor_first(
                &mut mc,
                &mut key,
                0 as *mut libc::c_void as *mut MDB_val,
            );
            if rc != 0 {
                return rc;
            }
            head_id = *(key.mv_data as *mut txnid_t);
            pglast = head_id;
            head_room = 0 as libc::c_int as ssize_t;
            total_room = head_room;
            if !(pglast <= (*env).me_pgstate.mf_pglast) {
                mdb_assert_fail(
                    (*txn).mt_env,
                    b"pglast <= env->me_pglast\0" as *const u8 as *const libc::c_char,
                    b"mdb_freelist_save\0" as *const u8 as *const libc::c_char,
                    b"mdb.c\0" as *const u8 as *const libc::c_char,
                    3523 as libc::c_int,
                );
            }
            rc = mdb_cursor_del(&mut mc, 0 as libc::c_uint);
            if rc != 0 {
                return rc;
            }
        }
        if freecnt < *((*txn).mt_free_pgs).offset(0 as libc::c_int as isize) {
            if freecnt == 0 {
                rc = mdb_page_search(
                    &mut mc,
                    0 as *mut libc::c_void as *mut MDB_val,
                    9 as libc::c_int,
                );
                if rc != 0 {
                    if rc != -(30798 as libc::c_int) {
                        return rc;
                    }
                }
            }
            free_pgs = (*txn).mt_free_pgs;
            key.mv_size = ::std::mem::size_of::<txnid_t>() as libc::c_ulong;
            key.mv_data = &mut (*txn).mt_txnid as *mut txnid_t as *mut libc::c_void;
            loop {
                freecnt = *free_pgs.offset(0 as libc::c_int as isize);
                data
                    .mv_size = (*free_pgs.offset(0 as libc::c_int as isize))
                    .wrapping_add(1 as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<MDB_ID>() as libc::c_ulong);
                rc = mdb_cursor_put(&mut mc, &mut key, &mut data, 65536 as libc::c_uint);
                if rc != 0 {
                    return rc;
                }
                free_pgs = (*txn).mt_free_pgs;
                if !(freecnt < *free_pgs.offset(0 as libc::c_int as isize)) {
                    break;
                }
            }
            mdb_midl_sort(free_pgs);
            memcpy(data.mv_data, free_pgs as *const libc::c_void, data.mv_size);
        } else {
            mop = (*env).me_pgstate.mf_pghead;
            if !mop.is_null() {
                tmp___1 = *mop.offset(0 as libc::c_int as isize);
            } else {
                tmp___1 = 0 as libc::c_int as pgno_t;
            }
            mop_len = tmp___1.wrapping_add((*txn).mt_loose_count as pgno_t) as ssize_t;
            if total_room >= mop_len {
                if total_room == mop_len {
                    break;
                }
                more -= 1;
                if more < 0 as libc::c_int {
                    break;
                }
            } else if head_room >= maxfree_1pg as ssize_t {
                if head_id > 1 as libc::c_ulong {
                    head_id = head_id.wrapping_sub(1);
                    head_room = 0 as libc::c_int as ssize_t;
                }
            }
            total_room -= head_room;
            head_room = mop_len - total_room;
            let mut current_block_134: u64;
            if head_room > maxfree_1pg as ssize_t {
                if head_id > 1 as libc::c_ulong {
                    head_room = (head_room as txnid_t).wrapping_div(head_id) as ssize_t;
                    head_room
                        += maxfree_1pg as libc::c_long
                            - head_room
                                % (maxfree_1pg + 1 as libc::c_int) as libc::c_long;
                    current_block_134 = 18221534353613080499;
                } else {
                    current_block_134 = 13949408038079131816;
                }
            } else {
                current_block_134 = 13949408038079131816;
            }
            match current_block_134 {
                13949408038079131816 => {
                    if head_room < 0 as libc::c_long {
                        head_room = 0 as libc::c_int as ssize_t;
                    }
                }
                _ => {}
            }
            key.mv_size = ::std::mem::size_of::<txnid_t>() as libc::c_ulong;
            key.mv_data = &mut head_id as *mut txnid_t as *mut libc::c_void;
            data
                .mv_size = ((head_room + 1 as libc::c_long) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<pgno_t>() as libc::c_ulong);
            rc = mdb_cursor_put(&mut mc, &mut key, &mut data, 65536 as libc::c_uint);
            if rc != 0 {
                return rc;
            }
            pgs = data.mv_data as *mut pgno_t;
            if head_room > clean_limit {
                j = head_room;
            } else {
                j = 0 as libc::c_int as ssize_t;
            }
            loop {
                *pgs.offset(j as isize) = 0 as libc::c_int as pgno_t;
                j -= 1;
                if !(j >= 0 as libc::c_long) {
                    break;
                }
            }
            total_room += head_room;
        }
    }
    if !((*txn).mt_loose_pgs).is_null() {
        mp___0 = (*txn).mt_loose_pgs;
        count = (*txn).mt_loose_count as libc::c_uint;
        rc = mdb_midl_need(
            &mut (*env).me_pgstate.mf_pghead,
            (2 as libc::c_uint).wrapping_mul(count).wrapping_add(1 as libc::c_uint),
        );
        if rc != 0 as libc::c_int {
            return rc;
        }
        mop = (*env).me_pgstate.mf_pghead;
        loose = mop
            .offset(*mop.offset(-(1 as libc::c_int) as isize) as isize)
            .offset(-(count as isize));
        count = 0 as libc::c_uint;
        while !mp___0.is_null() {
            count = count.wrapping_add(1);
            *loose.offset(count as isize) = (*mp___0).mp_p.p_pgno;
            mp___0 = *(mp___0.offset(2 as libc::c_int as isize) as *mut *mut MDB_page);
        }
        *loose.offset(0 as libc::c_int as isize) = count as MDB_ID;
        mdb_midl_sort(loose);
        mdb_midl_xmerge(mop, loose);
        (*txn).mt_loose_pgs = 0 as *mut libc::c_void as *mut MDB_page;
        (*txn).mt_loose_count = 0 as libc::c_int;
        mop_len = *mop.offset(0 as libc::c_int as isize) as ssize_t;
    }
    rc = 0 as libc::c_int;
    if mop_len != 0 {
        mop = mop.offset(mop_len as isize);
        rc = mdb_cursor_first(&mut mc, &mut key___0, &mut data___0);
        while rc == 0 {
            id = *(key___0.mv_data as *mut txnid_t);
            len = (data___0.mv_size)
                .wrapping_div(::std::mem::size_of::<MDB_ID>() as libc::c_ulong)
                as ssize_t - 1 as libc::c_long;
            if len >= 0 as libc::c_long {
                if !(id <= (*env).me_pgstate.mf_pglast) {
                    mdb_assert_fail(
                        (*txn).mt_env,
                        b"len >= 0 && id <= env->me_pglast\0" as *const u8
                            as *const libc::c_char,
                        b"mdb_freelist_save\0" as *const u8 as *const libc::c_char,
                        b"mdb.c\0" as *const u8 as *const libc::c_char,
                        3639 as libc::c_int,
                    );
                }
            } else {
                mdb_assert_fail(
                    (*txn).mt_env,
                    b"len >= 0 && id <= env->me_pglast\0" as *const u8
                        as *const libc::c_char,
                    b"mdb_freelist_save\0" as *const u8 as *const libc::c_char,
                    b"mdb.c\0" as *const u8 as *const libc::c_char,
                    3639 as libc::c_int,
                );
            }
            key___0.mv_data = &mut id as *mut txnid_t as *mut libc::c_void;
            if len > mop_len {
                len = mop_len;
                data___0
                    .mv_size = ((len + 1 as libc::c_long) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<MDB_ID>() as libc::c_ulong);
            }
            mop = mop.offset(-(len as isize));
            data___0.mv_data = mop as *mut libc::c_void;
            save = *mop.offset(0 as libc::c_int as isize);
            *mop.offset(0 as libc::c_int as isize) = len as pgno_t;
            rc = mdb_cursor_put(
                &mut mc,
                &mut key___0,
                &mut data___0,
                64 as libc::c_uint,
            );
            *mop.offset(0 as libc::c_int as isize) = save;
            if rc != 0 {
                break;
            }
            mop_len -= len;
            if mop_len == 0 {
                break;
            }
            rc = mdb_cursor_next(&mut mc, &mut key___0, &mut data___0, MDB_NEXT);
        }
    }
    return rc;
}
unsafe extern "C" fn mdb_page_flush(
    mut txn: *mut MDB_txn,
    mut keep: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut env: *mut MDB_env = 0 as *mut MDB_env;
    let mut dl: MDB_ID2L = 0 as *mut MDB_ID2;
    let mut psize: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    let mut pagecount: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut size: size_t = 0;
    let mut pos: off_t = 0;
    let mut pgno: pgno_t = 0;
    let mut dp: *mut MDB_page = 0 as *mut MDB_page;
    let mut iov: [iovec; 64] = [iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    }; 64];
    let mut fd: libc::c_int = 0;
    let mut wsize: ssize_t = 0;
    let mut wres: ssize_t = 0;
    let mut wpos: off_t = 0;
    let mut next_pos: off_t = 0;
    let mut n: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: __off_t = 0;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    env = (*txn).mt_env;
    dl = (*txn).mt_u.dirty_list;
    psize = (*env).me_psize;
    pagecount = (*dl.offset(0 as libc::c_int as isize)).mid as libc::c_int;
    size = 0 as libc::c_int as size_t;
    pos = 0 as libc::c_int as off_t;
    pgno = 0 as libc::c_int as pgno_t;
    dp = 0 as *mut libc::c_void as *mut MDB_page;
    fd = (*env).me_fd;
    wsize = 0 as libc::c_int as ssize_t;
    wpos = 0 as libc::c_int as off_t;
    next_pos = 1 as libc::c_int as off_t;
    n = 0 as libc::c_int;
    i = keep;
    j = i as libc::c_uint;
    if (*env).me_flags & 524288 as libc::c_uint != 0 {
        loop {
            i += 1;
            if !(i <= pagecount) {
                break;
            }
            dp = (*dl.offset(i as isize)).mptr as *mut MDB_page;
            if (*dp).mp_flags as libc::c_int & 49152 as libc::c_int != 0 {
                (*dp)
                    .mp_flags = ((*dp).mp_flags as libc::c_int & -(32769 as libc::c_int))
                    as uint16_t;
                j = j.wrapping_add(1);
                *dl.offset(j as isize) = *dl.offset(i as isize);
            } else {
                (*dp)
                    .mp_flags = ((*dp).mp_flags as libc::c_int & -(17 as libc::c_int))
                    as uint16_t;
            }
        }
    } else {
        loop {
            i += 1;
            if i <= pagecount {
                dp = (*dl.offset(i as isize)).mptr as *mut MDB_page;
                if (*dp).mp_flags as libc::c_int & 49152 as libc::c_int != 0 {
                    (*dp)
                        .mp_flags = ((*dp).mp_flags as libc::c_int
                        & -(32769 as libc::c_int)) as uint16_t;
                    (*dl.offset(i as isize)).mid = 0 as libc::c_int as MDB_ID;
                    continue;
                } else {
                    pgno = (*dl.offset(i as isize)).mid;
                    (*dp)
                        .mp_flags = ((*dp).mp_flags as libc::c_int
                        & -(17 as libc::c_int)) as uint16_t;
                    pos = pgno.wrapping_mul(psize as pgno_t) as off_t;
                    size = psize as size_t;
                    if (*dp).mp_flags as libc::c_int & 4 as libc::c_int
                        == 4 as libc::c_int
                    {
                        size = (size as libc::c_ulong)
                            .wrapping_mul((*dp).mp_pb.pb_pages as size_t) as size_t
                            as size_t;
                    }
                }
            }
            if pos != next_pos {
                current_block = 1982260669945688287;
            } else if n == 64 as libc::c_int {
                current_block = 1982260669945688287;
            } else if (wsize as size_t).wrapping_add(size)
                    > (1073741824 as libc::c_uint
                        >> (::std::mem::size_of::<ssize_t>() as libc::c_ulong
                            == 4 as libc::c_ulong) as libc::c_int) as size_t
                {
                current_block = 1982260669945688287;
            } else {
                current_block = 307447392441238883;
            }
            match current_block {
                1982260669945688287 => {
                    if n != 0 {
                        's_300: {
                            loop {
                                if n == 1 as libc::c_int {
                                    wres = pwrite(
                                        fd,
                                        iov[0 as libc::c_int as usize].iov_base
                                            as *const libc::c_void,
                                        wsize as size_t,
                                        wpos,
                                    );
                                } else {
                                    's_255: {
                                        loop {
                                            tmp___0 = lseek(fd, wpos, 0 as libc::c_int);
                                            if tmp___0 == -(1 as libc::c_long) {
                                                tmp = __errno_location();
                                                rc = *tmp;
                                                if rc == 4 as libc::c_int {
                                                    continue;
                                                }
                                                return rc;
                                            } else {
                                                wres = writev(fd, iov.as_mut_ptr() as *const iovec, n);
                                                break 's_255;
                                            }
                                        }
                                    }
                                }
                                if wres != wsize {
                                    if wres < 0 as libc::c_long {
                                        tmp___1 = __errno_location();
                                        rc = *tmp___1;
                                        if !(rc == 4 as libc::c_int) {
                                            break;
                                        }
                                    } else {
                                        rc = 5 as libc::c_int;
                                        break;
                                    }
                                } else {
                                    n = 0 as libc::c_int;
                                    break 's_300;
                                }
                            }
                            return rc;
                        }
                    }
                    if i > pagecount {
                        break;
                    }
                    wpos = pos;
                    wsize = 0 as libc::c_int as ssize_t;
                }
                _ => {}
            }
            iov[n as usize].iov_len = size;
            iov[n as usize].iov_base = dp as *mut libc::c_char as *mut libc::c_void;
            next_pos = (pos as size_t).wrapping_add(size) as off_t;
            wsize = (wsize as size_t).wrapping_add(size) as ssize_t;
            n += 1;
        }
        if (*env).me_flags & 524288 as libc::c_uint == 0 {
            i = keep;
            loop {
                i += 1;
                if !(i <= pagecount) {
                    break;
                }
                dp = (*dl.offset(i as isize)).mptr as *mut MDB_page;
                if (*dl.offset(i as isize)).mid == 0 {
                    j = j.wrapping_add(1);
                    *dl.offset(j as isize) = *dl.offset(i as isize);
                    (*dl.offset(j as isize)).mid = (*dp).mp_p.p_pgno;
                } else {
                    mdb_dpage_free(env, dp);
                }
            }
        }
    }
    i -= 1;
    (*txn)
        .mt_dirty_room = ((*txn).mt_dirty_room)
        .wrapping_add((i as libc::c_uint).wrapping_sub(j));
    (*dl.offset(0 as libc::c_int as isize)).mid = j as MDB_ID;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_txn_commit(mut txn: *mut MDB_txn) -> libc::c_int {
    let mut current_block: u64;
    let mut rc: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    let mut end_mode: libc::c_uint = 0;
    let mut env: *mut MDB_env = 0 as *mut MDB_env;
    let mut parent: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut lp: *mut *mut MDB_page = 0 as *mut *mut MDB_page;
    let mut dst: MDB_ID2L = 0 as *mut MDB_ID2;
    let mut src: MDB_ID2L = 0 as *mut MDB_ID2;
    let mut pspill: MDB_IDL = 0 as *mut MDB_ID;
    let mut x: libc::c_uint = 0;
    let mut y: libc::c_uint = 0;
    let mut len: libc::c_uint = 0;
    let mut ps_len: libc::c_uint = 0;
    let mut pn: MDB_ID = 0;
    let mut pn___0: MDB_ID = 0;
    let mut tmp: libc::c_uint = 0;
    let mut yp: pgno_t = 0;
    let mut tmp___0: libc::c_uint = 0;
    let mut tmp___1: libc::c_uint = 0;
    let mut yp___0: pgno_t = 0;
    let mut tmp___2: libc::c_uint = 0;
    let mut tmp___3: libc::c_uint = 0;
    let mut tmp___4: libc::c_uint = 0;
    let mut mc: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut i___0: MDB_dbi = 0;
    let mut data: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut excl: libc::c_int = 0;
    if txn as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 22 as libc::c_int;
    }
    end_mode = 2097201 as libc::c_uint;
    if !((*txn).mt_child).is_null() {
        rc = mdb_txn_commit((*txn).mt_child);
        if rc != 0 {
            current_block = 9131838931983798056;
        } else {
            current_block = 5143058163439228106;
        }
    } else {
        current_block = 5143058163439228106;
    }
    match current_block {
        5143058163439228106 => {
            env = (*txn).mt_env;
            if (*txn).mt_flags & 131072 as libc::c_uint == 131072 as libc::c_uint {
                current_block = 6306164742312600464;
            } else if (*txn).mt_flags & 3 as libc::c_uint != 0 {
                if !((*txn).mt_parent).is_null() {
                    (*(*txn).mt_parent).mt_flags |= 2 as libc::c_uint;
                }
                rc = -(30782 as libc::c_int);
                current_block = 9131838931983798056;
            } else if !((*txn).mt_parent).is_null() {
                parent = (*txn).mt_parent;
                rc = mdb_midl_append_list(
                    &mut (*parent).mt_free_pgs,
                    (*txn).mt_free_pgs,
                );
                if rc != 0 {
                    current_block = 9131838931983798056;
                } else {
                    mdb_midl_free((*txn).mt_free_pgs);
                    (*parent).mt_next_pgno = (*txn).mt_next_pgno;
                    (*parent).mt_flags = (*txn).mt_flags;
                    mdb_cursors_close(txn, 1 as libc::c_uint);
                    memcpy(
                        (*parent).mt_dbs as *mut libc::c_void,
                        (*txn).mt_dbs as *const libc::c_void,
                        ((*txn).mt_numdbs as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<MDB_db>() as libc::c_ulong,
                            ),
                    );
                    (*parent).mt_numdbs = (*txn).mt_numdbs;
                    *((*parent).mt_dbflags)
                        .offset(
                            0 as libc::c_int as isize,
                        ) = *((*txn).mt_dbflags).offset(0 as libc::c_int as isize);
                    *((*parent).mt_dbflags)
                        .offset(
                            1 as libc::c_int as isize,
                        ) = *((*txn).mt_dbflags).offset(1 as libc::c_int as isize);
                    i = 2 as libc::c_uint;
                    while i < (*txn).mt_numdbs {
                        x = (*((*parent).mt_dbflags).offset(i as isize) as libc::c_int
                            & 4 as libc::c_int) as libc::c_uint;
                        *((*parent).mt_dbflags)
                            .offset(
                                i as isize,
                            ) = (*((*txn).mt_dbflags).offset(i as isize) as libc::c_uint
                            | x) as libc::c_uchar;
                        i = i.wrapping_add(1);
                    }
                    dst = (*parent).mt_u.dirty_list;
                    src = (*txn).mt_u.dirty_list;
                    pspill = (*parent).mt_spill_pgs;
                    if !pspill.is_null() {
                        ps_len = *pspill.offset(0 as libc::c_int as isize)
                            as libc::c_uint;
                        if ps_len != 0 {
                            y = ps_len;
                            x = y;
                            *pspill
                                .offset(
                                    0 as libc::c_int as isize,
                                ) = -(1 as libc::c_int) as pgno_t;
                            i = 0 as libc::c_uint;
                            len = (*src.offset(0 as libc::c_int as isize)).mid
                                as libc::c_uint;
                            loop {
                                i = i.wrapping_add(1);
                                if !(i <= len) {
                                    break;
                                }
                                pn = (*src.offset(i as isize)).mid << 1 as libc::c_int;
                                while pn > *pspill.offset(x as isize) {
                                    x = x.wrapping_sub(1);
                                }
                                if pn == *pspill.offset(x as isize) {
                                    *pspill.offset(x as isize) = 1 as libc::c_int as MDB_ID;
                                    x = x.wrapping_sub(1);
                                    y = x;
                                }
                            }
                            x = y;
                            loop {
                                x = x.wrapping_add(1);
                                if !(x <= ps_len) {
                                    break;
                                }
                                if *pspill.offset(x as isize) & 1 as libc::c_ulong == 0 {
                                    y = y.wrapping_add(1);
                                    *pspill.offset(y as isize) = *pspill.offset(x as isize);
                                }
                            }
                            *pspill.offset(0 as libc::c_int as isize) = y as MDB_ID;
                        }
                    }
                    if !((*txn).mt_spill_pgs).is_null() {
                        if *((*txn).mt_spill_pgs).offset(0 as libc::c_int as isize) != 0
                        {
                            i = 1 as libc::c_uint;
                            while i as MDB_ID
                                <= *((*txn).mt_spill_pgs).offset(0 as libc::c_int as isize)
                            {
                                pn___0 = *((*txn).mt_spill_pgs).offset(i as isize);
                                if !(pn___0 & 1 as libc::c_ulong != 0) {
                                    pn___0 >>= 1 as libc::c_int;
                                    y = mdb_mid2l_search(dst, pn___0);
                                    if y as MDB_ID
                                        <= (*dst.offset(0 as libc::c_int as isize)).mid
                                    {
                                        if (*dst.offset(y as isize)).mid == pn___0 {
                                            free((*dst.offset(y as isize)).mptr);
                                            while (y as MDB_ID)
                                                < (*dst.offset(0 as libc::c_int as isize)).mid
                                            {
                                                *dst
                                                    .offset(
                                                        y as isize,
                                                    ) = *dst.offset(y.wrapping_add(1 as libc::c_uint) as isize);
                                                y = y.wrapping_add(1);
                                            }
                                            let ref mut fresh10 = (*dst
                                                .offset(0 as libc::c_int as isize))
                                                .mid;
                                            *fresh10 = (*fresh10).wrapping_sub(1);
                                        }
                                    }
                                }
                                i = i.wrapping_add(1);
                            }
                        }
                    }
                    x = (*dst.offset(0 as libc::c_int as isize)).mid as libc::c_uint;
                    (*dst.offset(0 as libc::c_int as isize))
                        .mid = 0 as libc::c_int as MDB_ID;
                    if !((*parent).mt_parent).is_null() {
                        len = (x as MDB_ID)
                            .wrapping_add((*src.offset(0 as libc::c_int as isize)).mid)
                            as libc::c_uint;
                        tmp = mdb_mid2l_search(
                            src,
                            ((*dst.offset(x as isize)).mid)
                                .wrapping_add(1 as libc::c_ulong),
                        );
                        y = tmp.wrapping_sub(1 as libc::c_uint);
                        i = x;
                        while y != 0 {
                            if i == 0 {
                                break;
                            }
                            yp = (*src.offset(y as isize)).mid;
                            while yp < (*dst.offset(i as isize)).mid {
                                i = i.wrapping_sub(1);
                            }
                            if yp == (*dst.offset(i as isize)).mid {
                                i = i.wrapping_sub(1);
                                len = len.wrapping_sub(1);
                            }
                            y = y.wrapping_sub(1);
                        }
                    } else {
                        len = ((((1 as libc::c_int) << 17 as libc::c_int)
                            - 1 as libc::c_int) as libc::c_uint)
                            .wrapping_sub((*txn).mt_dirty_room);
                    }
                    y = (*src.offset(0 as libc::c_int as isize)).mid as libc::c_uint;
                    i = len;
                    while y != 0 {
                        yp___0 = (*src.offset(y as isize)).mid;
                        while yp___0 < (*dst.offset(x as isize)).mid {
                            tmp___2 = i;
                            i = i.wrapping_sub(1);
                            tmp___3 = x;
                            x = x.wrapping_sub(1);
                            *dst
                                .offset(tmp___2 as isize) = *dst.offset(tmp___3 as isize);
                        }
                        if yp___0 == (*dst.offset(x as isize)).mid {
                            tmp___4 = x;
                            x = x.wrapping_sub(1);
                            free((*dst.offset(tmp___4 as isize)).mptr);
                        }
                        tmp___0 = i;
                        i = i.wrapping_sub(1);
                        tmp___1 = y;
                        y = y.wrapping_sub(1);
                        *dst.offset(tmp___0 as isize) = *src.offset(tmp___1 as isize);
                    }
                    if !(i == x) {
                        mdb_assert_fail(
                            (*txn).mt_env,
                            b"i == x\0" as *const u8 as *const libc::c_char,
                            b"mdb_txn_commit\0" as *const u8 as *const libc::c_char,
                            b"mdb.c\0" as *const u8 as *const libc::c_char,
                            4028 as libc::c_int,
                        );
                    }
                    (*dst.offset(0 as libc::c_int as isize)).mid = len as MDB_ID;
                    free((*txn).mt_u.dirty_list as *mut libc::c_void);
                    (*parent).mt_dirty_room = (*txn).mt_dirty_room;
                    if !((*txn).mt_spill_pgs).is_null() {
                        if !((*parent).mt_spill_pgs).is_null() {
                            rc = mdb_midl_append_list(
                                &mut (*parent).mt_spill_pgs,
                                (*txn).mt_spill_pgs,
                            );
                            if rc != 0 {
                                (*parent).mt_flags |= 2 as libc::c_uint;
                            }
                            mdb_midl_free((*txn).mt_spill_pgs);
                            mdb_midl_sort((*parent).mt_spill_pgs);
                        } else {
                            (*parent).mt_spill_pgs = (*txn).mt_spill_pgs;
                        }
                    }
                    lp = &mut (*parent).mt_loose_pgs;
                    while !(*lp).is_null() {
                        lp = (*lp).offset(2 as libc::c_int as isize)
                            as *mut *mut MDB_page;
                    }
                    *lp = (*txn).mt_loose_pgs;
                    (*parent).mt_loose_count += (*txn).mt_loose_count;
                    (*parent).mt_child = 0 as *mut libc::c_void as *mut MDB_txn;
                    mdb_midl_free((*(txn as *mut MDB_ntxn)).mnt_pgstate.mf_pghead);
                    free(txn as *mut libc::c_void);
                    return rc;
                }
            } else if txn as libc::c_ulong != (*env).me_txn as libc::c_ulong {
                rc = 22 as libc::c_int;
                current_block = 9131838931983798056;
            } else {
                mdb_cursors_close(txn, 0 as libc::c_uint);
                if (*((*txn).mt_u.dirty_list).offset(0 as libc::c_int as isize)).mid == 0
                {
                    if (*txn).mt_flags & 12 as libc::c_uint == 0 {
                        current_block = 6306164742312600464;
                    } else {
                        current_block = 939350892795860671;
                    }
                } else {
                    current_block = 939350892795860671;
                }
                match current_block {
                    6306164742312600464 => {}
                    _ => {
                        if (*txn).mt_numdbs > 2 as libc::c_uint {
                            data
                                .mv_size = ::std::mem::size_of::<MDB_db>() as libc::c_ulong;
                            mdb_cursor_init(
                                &mut mc,
                                txn,
                                1 as libc::c_int as MDB_dbi,
                                0 as *mut libc::c_void as *mut MDB_xcursor,
                            );
                            i___0 = 2 as libc::c_int as MDB_dbi;
                            loop {
                                if !(i___0 < (*txn).mt_numdbs) {
                                    current_block = 8444733628337052024;
                                    break;
                                }
                                if *((*txn).mt_dbflags).offset(i___0 as isize)
                                    as libc::c_int & 1 as libc::c_int != 0
                                {
                                    if *((*txn).mt_dbiseqs).offset(i___0 as isize)
                                        != *((*(*txn).mt_env).me_dbiseqs).offset(i___0 as isize)
                                    {
                                        rc = -(30780 as libc::c_int);
                                        current_block = 9131838931983798056;
                                        break;
                                    } else {
                                        data
                                            .mv_data = ((*txn).mt_dbs).offset(i___0 as isize)
                                            as *mut libc::c_void;
                                        rc = mdb_cursor_put(
                                            &mut mc,
                                            &mut (*((*txn).mt_dbxs).offset(i___0 as isize)).md_name,
                                            &mut data,
                                            2 as libc::c_uint,
                                        );
                                        if rc != 0 {
                                            current_block = 9131838931983798056;
                                            break;
                                        }
                                    }
                                }
                                i___0 = i___0.wrapping_add(1);
                            }
                        } else {
                            current_block = 8444733628337052024;
                        }
                        match current_block {
                            9131838931983798056 => {}
                            _ => {
                                rc = mdb_freelist_save(txn);
                                if rc != 0 {
                                    current_block = 9131838931983798056;
                                } else {
                                    mdb_midl_free((*env).me_pgstate.mf_pghead);
                                    (*env)
                                        .me_pgstate
                                        .mf_pghead = 0 as *mut libc::c_void as *mut pgno_t;
                                    mdb_midl_shrink(&mut (*txn).mt_free_pgs);
                                    rc = mdb_page_flush(txn, 0 as libc::c_int);
                                    if rc != 0 {
                                        current_block = 9131838931983798056;
                                    } else {
                                        if !((*txn).mt_flags & 65536 as libc::c_uint
                                            == 65536 as libc::c_uint)
                                        {
                                            rc = mdb_env_sync0(
                                                env,
                                                0 as libc::c_int,
                                                (*txn).mt_next_pgno,
                                            );
                                            if rc != 0 {
                                                current_block = 9131838931983798056;
                                            } else {
                                                current_block = 10253503901371725554;
                                            }
                                        } else {
                                            current_block = 10253503901371725554;
                                        }
                                        match current_block {
                                            9131838931983798056 => {}
                                            _ => {
                                                rc = mdb_env_write_meta(txn);
                                                if rc != 0 {
                                                    current_block = 9131838931983798056;
                                                } else {
                                                    end_mode = 16 as libc::c_uint;
                                                    if (*env).me_flags & 33554432 as libc::c_uint != 0 {
                                                        if (*env).me_flags & 4194304 as libc::c_uint == 0 {
                                                            rc = mdb_env_share_locks(env, &mut excl);
                                                            if rc != 0 {
                                                                current_block = 9131838931983798056;
                                                            } else {
                                                                current_block = 13161952823003036500;
                                                            }
                                                        } else {
                                                            current_block = 13161952823003036500;
                                                        }
                                                        match current_block {
                                                            9131838931983798056 => {}
                                                            _ => {
                                                                (*env).me_flags ^= 33554432 as libc::c_uint;
                                                                current_block = 6306164742312600464;
                                                            }
                                                        }
                                                    } else {
                                                        current_block = 6306164742312600464;
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
            match current_block {
                9131838931983798056 => {}
                _ => {
                    mdb_txn_end(txn, end_mode);
                    return 0 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    mdb_txn_abort(txn);
    return rc;
}
unsafe extern "C" fn mdb_env_read_header(
    mut env: *mut MDB_env,
    mut prev: libc::c_int,
    mut meta: *mut MDB_meta,
) -> libc::c_int {
    let mut pbuf: MDB_metabuf = MDB_metabuf {
        mb_page: MDB_page {
            mp_p: __anonunion_mp_p_409824043 {
                p_pgno: 0,
            },
            mp_pad: 0,
            mp_flags: 0,
            mp_pb: __anonunion_mp_pb_280193743 {
                pb: __anonstruct_pb_438838223 {
                    pb_lower: 0,
                    pb_upper: 0,
                },
            },
            mp_ptrs: [0; 1],
        },
    };
    let mut p: *mut MDB_page = 0 as *mut MDB_page;
    let mut m: *mut MDB_meta = 0 as *mut MDB_meta;
    let mut i: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut off: libc::c_int = 0;
    let mut tmp: ssize_t = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: libc::c_int = 0;
    off = 0 as libc::c_int;
    i = off;
    while i < 2 as libc::c_int {
        tmp = pread(
            (*env).me_fd,
            &mut pbuf as *mut MDB_metabuf as *mut libc::c_void,
            152 as libc::c_int as size_t,
            off as __off_t,
        );
        rc = tmp as libc::c_int;
        if rc != 152 as libc::c_int {
            if rc == 0 as libc::c_int {
                if off == 0 as libc::c_int {
                    return 2 as libc::c_int;
                }
            }
            if rc < 0 as libc::c_int {
                tmp___0 = __errno_location();
                rc = *tmp___0;
            } else {
                rc = -(30793 as libc::c_int);
            }
            return rc;
        }
        p = &mut pbuf as *mut MDB_metabuf as *mut MDB_page;
        if !((*p).mp_flags as libc::c_int & 8 as libc::c_int == 8 as libc::c_int) {
            return -(30793 as libc::c_int);
        }
        m = (p as *mut libc::c_char)
            .offset(
                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                    as libc::c_uint as isize,
            ) as *mut libc::c_void as *mut MDB_meta;
        if (*m).mm_magic != 3203383518 as libc::c_uint {
            return -(30793 as libc::c_int);
        }
        if (*m).mm_version != 1 as libc::c_uint {
            return -(30794 as libc::c_int);
        }
        if off == 0 as libc::c_int {
            *meta = *m;
        } else {
            if prev != 0 {
                tmp___1 = ((*m).mm_txnid < (*meta).mm_txnid) as libc::c_int;
            } else {
                tmp___1 = ((*m).mm_txnid > (*meta).mm_txnid) as libc::c_int;
            }
            if tmp___1 != 0 {
                *meta = *m;
            }
        }
        i += 1;
        off = (off as uint32_t)
            .wrapping_add((*meta).mm_dbs[0 as libc::c_int as usize].md_pad)
            as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_env_init_meta0(mut env: *mut MDB_env, mut meta: *mut MDB_meta) {
    (*meta).mm_magic = 3203383518 as libc::c_uint;
    (*meta).mm_version = 1 as libc::c_int as uint32_t;
    (*meta).mm_mapsize = (*env).me_mapsize;
    (*meta).mm_dbs[0 as libc::c_int as usize].md_pad = (*env).me_psize;
    (*meta).mm_last_pg = 1 as libc::c_int as pgno_t;
    (*meta)
        .mm_dbs[0 as libc::c_int as usize]
        .md_flags = ((*env).me_flags & 65535 as libc::c_uint) as uint16_t;
    (*meta)
        .mm_dbs[0 as libc::c_int as usize]
        .md_flags = ((*meta).mm_dbs[0 as libc::c_int as usize].md_flags as libc::c_int
        | 8 as libc::c_int) as uint16_t;
    (*meta).mm_dbs[0 as libc::c_int as usize].md_root = !(0 as libc::c_int as pgno_t);
    (*meta).mm_dbs[1 as libc::c_int as usize].md_root = !(0 as libc::c_int as pgno_t);
}
unsafe extern "C" fn mdb_env_init_meta(
    mut env: *mut MDB_env,
    mut meta: *mut MDB_meta,
) -> libc::c_int {
    let mut p: *mut MDB_page = 0 as *mut MDB_page;
    let mut q: *mut MDB_page = 0 as *mut MDB_page;
    let mut rc: libc::c_int = 0;
    let mut psize: libc::c_uint = 0;
    let mut len: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: ssize_t = 0;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    psize = (*env).me_psize;
    tmp = calloc(2 as libc::c_int as size_t, psize as size_t);
    p = tmp as *mut MDB_page;
    if p.is_null() {
        return 12 as libc::c_int;
    }
    (*p).mp_p.p_pgno = 0 as libc::c_int as pgno_t;
    (*p).mp_flags = 8 as libc::c_int as uint16_t;
    *((p as *mut libc::c_char)
        .offset(
            &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                as libc::c_uint as isize,
        ) as *mut libc::c_void as *mut MDB_meta) = *meta;
    q = (p as *mut libc::c_char).offset(psize as isize) as *mut MDB_page;
    (*q).mp_p.p_pgno = 1 as libc::c_int as pgno_t;
    (*q).mp_flags = 8 as libc::c_int as uint16_t;
    *((q as *mut libc::c_char)
        .offset(
            &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                as libc::c_uint as isize,
        ) as *mut libc::c_void as *mut MDB_meta) = *meta;
    loop {
        tmp___0 = pwrite(
            (*env).me_fd,
            p as *const libc::c_void,
            psize.wrapping_mul(2 as libc::c_uint) as size_t,
            0 as libc::c_int as __off_t,
        );
        len = tmp___0 as libc::c_int;
        if len == -(1 as libc::c_int) {
            tmp___1 = __errno_location();
            if *tmp___1 == 4 as libc::c_int {
                continue;
            }
        }
        rc = (len >= 0 as libc::c_int) as libc::c_int;
        break;
    }
    if rc == 0 {
        tmp___2 = __errno_location();
        rc = *tmp___2;
    } else if len as libc::c_uint == psize.wrapping_mul(2 as libc::c_uint) {
        rc = 0 as libc::c_int;
    } else {
        rc = 28 as libc::c_int;
    }
    free(p as *mut libc::c_void);
    return rc;
}
unsafe extern "C" fn mdb_env_write_meta(mut txn: *mut MDB_txn) -> libc::c_int {
    let mut env: *mut MDB_env = 0 as *mut MDB_env;
    let mut meta: MDB_meta = MDB_meta {
        mm_magic: 0,
        mm_version: 0,
        mm_address: 0 as *mut libc::c_void,
        mm_mapsize: 0,
        mm_dbs: [MDB_db {
            md_pad: 0,
            md_flags: 0,
            md_depth: 0,
            md_branch_pages: 0,
            md_leaf_pages: 0,
            md_overflow_pages: 0,
            md_entries: 0,
            md_root: 0,
        }; 2],
        mm_last_pg: 0,
        mm_txnid: 0,
    };
    let mut metab: MDB_meta = MDB_meta {
        mm_magic: 0,
        mm_version: 0,
        mm_address: 0 as *mut libc::c_void,
        mm_mapsize: 0,
        mm_dbs: [MDB_db {
            md_pad: 0,
            md_flags: 0,
            md_depth: 0,
            md_branch_pages: 0,
            md_leaf_pages: 0,
            md_overflow_pages: 0,
            md_entries: 0,
            md_root: 0,
        }; 2],
        mm_last_pg: 0,
        mm_txnid: 0,
    };
    let mut mp: *mut MDB_meta = 0 as *mut MDB_meta;
    let mut flags: libc::c_uint = 0;
    let mut mapsize: mdb_size_t = 0;
    let mut off: off_t = 0;
    let mut rc: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut toggle: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mfd: libc::c_int = 0;
    let mut r2: libc::c_int = 0;
    let mut meta_size: libc::c_uint = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: ssize_t = 0;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: ssize_t = 0;
    let mut current_block_59: u64;
    toggle = ((*txn).mt_txnid & 1 as libc::c_ulong) as libc::c_int;
    env = (*txn).mt_env;
    flags = (*txn).mt_flags | (*env).me_flags;
    mp = (*env).me_metas[toggle as usize];
    mapsize = (*(*env).me_metas[(toggle ^ 1 as libc::c_int) as usize]).mm_mapsize;
    if mapsize < (*env).me_mapsize {
        mapsize = (*env).me_mapsize;
    }
    if flags & 524288 as libc::c_uint != 0 {
        (*mp).mm_mapsize = mapsize;
        (*mp)
            .mm_dbs[0 as libc::c_int
            as usize] = *((*txn).mt_dbs).offset(0 as libc::c_int as isize);
        (*mp)
            .mm_dbs[1 as libc::c_int
            as usize] = *((*txn).mt_dbs).offset(1 as libc::c_int as isize);
        (*mp).mm_last_pg = ((*txn).mt_next_pgno).wrapping_sub(1 as libc::c_ulong);
        (*mp).mm_txnid = (*txn).mt_txnid;
        if flags & 327680 as libc::c_uint == 0 {
            meta_size = (*env).me_psize;
            if (*env).me_flags & 1048576 as libc::c_uint != 0 {
                rc = 1 as libc::c_int;
            } else {
                rc = 4 as libc::c_int;
            }
            ptr = (mp as *mut libc::c_char)
                .offset(
                    -(&mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                        as libc::c_ulong as libc::c_uint as isize),
                );
            r2 = (ptr.offset_from((*env).me_map) as libc::c_long
                & ((*env).me_os_psize).wrapping_sub(1 as libc::c_uint) as libc::c_long)
                as libc::c_int;
            ptr = ptr.offset(-(r2 as isize));
            meta_size = meta_size.wrapping_add(r2 as libc::c_uint);
            tmp___0 = msync(ptr as *mut libc::c_void, meta_size as size_t, rc);
            if tmp___0 != 0 {
                tmp = __errno_location();
                rc = *tmp;
                current_block_59 = 4899541690612778199;
            } else {
                current_block_59 = 15846100102687479166;
            }
        } else {
            current_block_59 = 15846100102687479166;
        }
    } else {
        metab.mm_txnid = (*mp).mm_txnid;
        metab.mm_last_pg = (*mp).mm_last_pg;
        meta.mm_mapsize = mapsize;
        meta
            .mm_dbs[0 as libc::c_int
            as usize] = *((*txn).mt_dbs).offset(0 as libc::c_int as isize);
        meta
            .mm_dbs[1 as libc::c_int
            as usize] = *((*txn).mt_dbs).offset(1 as libc::c_int as isize);
        meta.mm_last_pg = ((*txn).mt_next_pgno).wrapping_sub(1 as libc::c_ulong);
        meta.mm_txnid = (*txn).mt_txnid;
        off = &mut (*(0 as *mut MDB_meta)).mm_mapsize as *mut mdb_size_t as libc::c_ulong
            as off_t;
        ptr = (&mut meta as *mut MDB_meta as *mut libc::c_char).offset(off as isize);
        len = (::std::mem::size_of::<MDB_meta>() as libc::c_ulong)
            .wrapping_sub(off as libc::c_ulong) as libc::c_int;
        off += (mp as *mut libc::c_char).offset_from((*env).me_map) as libc::c_long;
        if flags & 327680 as libc::c_uint != 0 {
            mfd = (*env).me_fd;
        } else {
            mfd = (*env).me_mfd;
        }
        loop {
            tmp___1 = pwrite(mfd, ptr as *const libc::c_void, len as size_t, off);
            rc = tmp___1 as libc::c_int;
            if !(rc != len) {
                current_block_59 = 15846100102687479166;
                break;
            }
            if rc < 0 as libc::c_int {
                tmp___2 = __errno_location();
                rc = *tmp___2;
            } else {
                rc = 5 as libc::c_int;
            }
            if rc == 4 as libc::c_int {
                continue;
            }
            meta.mm_last_pg = metab.mm_last_pg;
            meta.mm_txnid = metab.mm_txnid;
            tmp___3 = pwrite(
                (*env).me_fd,
                ptr as *const libc::c_void,
                len as size_t,
                off,
            );
            r2 = tmp___3 as libc::c_int;
            current_block_59 = 4899541690612778199;
            break;
        }
    }
    match current_block_59 {
        15846100102687479166 => {
            if !((*env).me_txns).is_null() {
                (*(*env).me_txns).mt1.mtb.mtb_txnid = (*txn).mt_txnid;
            }
            return 0 as libc::c_int;
        }
        _ => {
            (*env).me_flags |= 2147483648 as libc::c_uint;
            return rc;
        }
    };
}
unsafe extern "C" fn mdb_env_pick_meta(mut env: *const MDB_env) -> *mut MDB_meta {
    let mut metas: *const *mut MDB_meta = 0 as *const *mut MDB_meta;
    metas = ((*env).me_metas).as_ptr();
    return *metas
        .offset(
            (((**metas.offset(0 as libc::c_int as isize)).mm_txnid
                < (**metas.offset(1 as libc::c_int as isize)).mm_txnid) as libc::c_int
                ^ ((*env).me_flags & 33554432 as libc::c_uint != 0 as libc::c_uint)
                    as libc::c_int) as isize,
        );
}
pub unsafe extern "C" fn mdb_env_create(mut env: *mut *mut MDB_env) -> libc::c_int {
    let mut e: *mut MDB_env = 0 as *mut MDB_env;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: MDB_dbi = 0;
    let mut tmp___1: libc::c_long = 0;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<MDB_env>() as libc::c_ulong,
    );
    e = tmp as *mut MDB_env;
    if e.is_null() {
        return 12 as libc::c_int;
    }
    (*e).me_maxreaders = 126 as libc::c_uint;
    tmp___0 = 2 as libc::c_int as MDB_dbi;
    (*e).me_numdbs = tmp___0;
    (*e).me_maxdbs = tmp___0;
    (*e).me_fd = -(1 as libc::c_int);
    (*e).me_lfd = -(1 as libc::c_int);
    (*e).me_mfd = -(1 as libc::c_int);
    (*e).me_pid = getpid();
    tmp___1 = sysconf(30 as libc::c_int);
    (*e).me_os_psize = tmp___1 as libc::c_uint;
    *env = e;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_env_map(
    mut env: *mut MDB_env,
    mut addr: *mut libc::c_void,
) -> libc::c_int {
    let mut p: *mut MDB_page = 0 as *mut MDB_page;
    let mut flags: libc::c_uint = 0;
    let mut mmap_flags: libc::c_int = 0;
    let mut prot: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    flags = (*env).me_flags;
    mmap_flags = 1 as libc::c_int;
    prot = 1 as libc::c_int;
    if flags & 524288 as libc::c_uint != 0 {
        prot |= 2 as libc::c_int;
        tmp___0 = ftruncate((*env).me_fd, (*env).me_mapsize as __off_t);
        if tmp___0 < 0 as libc::c_int {
            tmp = __errno_location();
            return *tmp;
        }
    }
    tmp___1 = mmap(
        addr,
        (*env).me_mapsize,
        prot,
        mmap_flags,
        (*env).me_fd,
        0 as libc::c_int as __off_t,
    );
    (*env).me_map = tmp___1 as *mut libc::c_char;
    if (*env).me_map as libc::c_ulong
        == -(1 as libc::c_int) as *mut libc::c_void as libc::c_ulong
    {
        (*env).me_map = 0 as *mut libc::c_void as *mut libc::c_char;
        tmp___2 = __errno_location();
        return *tmp___2;
    }
    if flags & 8388608 as libc::c_uint != 0 {
        madvise((*env).me_map as *mut libc::c_void, (*env).me_mapsize, 1 as libc::c_int);
    }
    if !addr.is_null() {
        if (*env).me_map as libc::c_ulong != addr as libc::c_ulong {
            return 16 as libc::c_int;
        }
    }
    p = (*env).me_map as *mut MDB_page;
    (*env)
        .me_metas[0 as libc::c_int
        as usize] = (p as *mut libc::c_char)
        .offset(
            &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                as libc::c_uint as isize,
        ) as *mut libc::c_void as *mut MDB_meta;
    (*env)
        .me_metas[1 as libc::c_int
        as usize] = ((*env).me_metas[0 as libc::c_int as usize] as *mut libc::c_char)
        .offset((*env).me_psize as isize) as *mut MDB_meta;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_env_set_mapsize(
    mut env: *mut MDB_env,
    mut size: mdb_size_t,
) -> libc::c_int {
    let mut meta: *mut MDB_meta = 0 as *mut MDB_meta;
    let mut old: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut rc: libc::c_int = 0;
    let mut minsize: mdb_size_t = 0;
    if !((*env).me_map).is_null() {
        if !((*env).me_txn).is_null() {
            return 22 as libc::c_int;
        }
        meta = mdb_env_pick_meta(env as *const MDB_env);
        if size == 0 {
            size = (*meta).mm_mapsize;
        }
        minsize = ((*meta).mm_last_pg)
            .wrapping_add(1 as libc::c_ulong)
            .wrapping_mul((*env).me_psize as pgno_t);
        if size < minsize {
            size = minsize;
        }
        munmap((*env).me_map as *mut libc::c_void, (*env).me_mapsize);
        (*env).me_mapsize = size;
        if (*env).me_flags & 1 as libc::c_uint != 0 {
            old = (*env).me_map as *mut libc::c_void;
        } else {
            old = 0 as *mut libc::c_void;
        }
        rc = mdb_env_map(env, old);
        if rc != 0 {
            return rc;
        }
    }
    (*env).me_mapsize = size;
    if (*env).me_psize != 0 {
        (*env)
            .me_maxpg = ((*env).me_mapsize).wrapping_div((*env).me_psize as mdb_size_t);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_env_set_maxdbs(
    mut env: *mut MDB_env,
    mut dbs: MDB_dbi,
) -> libc::c_int {
    if !((*env).me_map).is_null() {
        return 22 as libc::c_int;
    }
    (*env).me_maxdbs = dbs.wrapping_add(2 as libc::c_uint);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_env_set_maxreaders(
    mut env: *mut MDB_env,
    mut readers: libc::c_uint,
) -> libc::c_int {
    if !((*env).me_map).is_null() {
        return 22 as libc::c_int
    } else {
        if readers < 1 as libc::c_uint {
            return 22 as libc::c_int;
        }
    }
    (*env).me_maxreaders = readers;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_env_get_maxreaders(
    mut env: *mut MDB_env,
    mut readers: *mut libc::c_uint,
) -> libc::c_int {
    if env.is_null() {
        return 22 as libc::c_int
    } else {
        if readers.is_null() {
            return 22 as libc::c_int;
        }
    }
    *readers = (*env).me_maxreaders;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_fsize(
    mut fd: libc::c_int,
    mut size: *mut mdb_size_t,
) -> libc::c_int {
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
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: libc::c_int = 0;
    tmp___0 = fstat(fd, &mut st);
    if tmp___0 != 0 {
        tmp = __errno_location();
        return *tmp;
    }
    *size = st.st_size as mdb_size_t;
    return 0 as libc::c_int;
}
static mut mdb_suffixes: [[*const mdb_nchar_t; 2]; 2] = [
    [
        b"/data.mdb\0" as *const u8 as *const libc::c_char as *const mdb_nchar_t,
        b"\0" as *const u8 as *const libc::c_char as *const mdb_nchar_t,
    ],
    [
        b"/lock.mdb\0" as *const u8 as *const libc::c_char as *const mdb_nchar_t,
        b"-lock\0" as *const u8 as *const libc::c_char as *const mdb_nchar_t,
    ],
];
unsafe extern "C" fn mdb_fname_init(
    mut path: *const libc::c_char,
    mut envflags: libc::c_uint,
    mut fname: *mut MDB_name,
) -> libc::c_int {
    let mut no_suffix: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: *mut mdb_nchar_t = 0 as *mut mdb_nchar_t;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    no_suffix = (envflags & 4210688 as libc::c_uint == 4210688 as libc::c_uint)
        as libc::c_int;
    (*fname).mn_alloced = 0 as libc::c_int;
    tmp = strlen(path);
    (*fname).mn_len = tmp as libc::c_int;
    if no_suffix != 0 {
        (*fname).mn_val = path as *mut libc::c_char;
    } else {
        tmp___1 = malloc(
            ((*fname).mn_len + 9 as libc::c_int + 1 as libc::c_int) as size_t,
        );
        tmp___0 = tmp___1 as *mut mdb_nchar_t;
        (*fname).mn_val = tmp___0;
        if tmp___0 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            (*fname).mn_alloced = 1 as libc::c_int;
            strcpy((*fname).mn_val as *mut libc::c_char, path);
        } else {
            return 12 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_fopen(
    mut env: *const MDB_env,
    mut fname: *mut MDB_name,
    mut which: mdb_fopen_type,
    mut mode: mdb_mode_t,
    mut res: *mut libc::c_int,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: libc::c_int = 0;
    rc = 0 as libc::c_int;
    if (*fname).mn_alloced != 0 {
        strcpy(
            ((*fname).mn_val).offset((*fname).mn_len as isize) as *mut libc::c_char,
            mdb_suffixes[(which as libc::c_uint == 524358 as libc::c_uint) as libc::c_int
                as usize][((*env).me_flags & 16384 as libc::c_uint
                == 16384 as libc::c_uint) as libc::c_int as usize] as *const libc::c_char,
        );
    }
    fd = open(
        (*fname).mn_val as *const libc::c_char,
        (which as libc::c_uint & 528579 as libc::c_uint) as libc::c_int,
        mode,
    );
    if fd == -(1 as libc::c_int) {
        tmp = __errno_location();
        rc = *tmp;
    } else {
        if which as libc::c_uint != 0 as libc::c_uint {
            if which as libc::c_uint != 66 as libc::c_uint {
                tmp___0 = 1 as libc::c_int;
            } else {
                tmp___0 = 0 as libc::c_int;
            }
        } else {
            tmp___0 = 0 as libc::c_int;
        }
        if which as libc::c_uint == 524481 as libc::c_uint {
            if (*env).me_psize >= (*env).me_os_psize {
                flags = fcntl(fd, 3 as libc::c_int);
                if flags != -(1 as libc::c_int) {
                    fcntl(fd, 4 as libc::c_int, flags | 16384 as libc::c_int);
                }
            }
        }
    }
    *res = fd;
    return rc;
}
unsafe extern "C" fn mdb_env_open2(
    mut env: *mut MDB_env,
    mut prev: libc::c_int,
) -> libc::c_int {
    let mut flags: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    let mut newenv: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut meta: MDB_meta = MDB_meta {
        mm_magic: 0,
        mm_version: 0,
        mm_address: 0 as *mut libc::c_void,
        mm_mapsize: 0,
        mm_dbs: [MDB_db {
            md_pad: 0,
            md_flags: 0,
            md_depth: 0,
            md_branch_pages: 0,
            md_leaf_pages: 0,
            md_overflow_pages: 0,
            md_entries: 0,
            md_root: 0,
        }; 2],
        mm_last_pg: 0,
        mm_txnid: 0,
    };
    let mut st: statfs = statfs {
        f_type: 0,
        f_bsize: 0,
        f_blocks: 0,
        f_bfree: 0,
        f_bavail: 0,
        f_files: 0,
        f_ffree: 0,
        f_fsid: __fsid_t { __val: [0; 2] },
        f_namelen: 0,
        f_frsize: 0,
        f_flags: 0,
        f_spare: [0; 4],
    };
    let mut uts: utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        domainname: [0; 65],
    };
    let mut i___0: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut minsize: mdb_size_t = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    flags = (*env).me_flags;
    newenv = 0 as libc::c_int;
    fstatfs((*env).me_fd, &mut st);
    let mut current_block_11: u64;
    if st.f_type == 61267 as libc::c_long {
        uname(&mut uts);
        if (uts.release[0 as libc::c_int as usize] as libc::c_int) < 51 as libc::c_int {
            tmp___0 = strncmp(
                (uts.release).as_mut_ptr() as *const libc::c_char,
                b"2.6.32.\0" as *const u8 as *const libc::c_char,
                7 as libc::c_int as size_t,
            );
            if tmp___0 != 0 {
                tmp = strncmp(
                    (uts.release).as_mut_ptr() as *const libc::c_char,
                    b"2.6.34.\0" as *const u8 as *const libc::c_char,
                    7 as libc::c_int as size_t,
                );
                if tmp == 0 {
                    i___0 = atoi(
                        (uts.release).as_mut_ptr().offset(7 as libc::c_int as isize)
                            as *const libc::c_char,
                    );
                    if i___0 >= 15 as libc::c_int {
                        current_block_11 = 11459959175219260272;
                    } else {
                        current_block_11 = 4488286894823169796;
                    }
                } else {
                    current_block_11 = 4488286894823169796;
                }
            } else {
                i___0 = atoi(
                    (uts.release).as_mut_ptr().offset(7 as libc::c_int as isize)
                        as *const libc::c_char,
                );
                if i___0 >= 60 as libc::c_int {
                    current_block_11 = 11459959175219260272;
                } else {
                    current_block_11 = 4488286894823169796;
                }
            }
        } else if uts.release[0 as libc::c_int as usize] as libc::c_int
                == 51 as libc::c_int
            {
            i___0 = atoi(
                (uts.release).as_mut_ptr().offset(2 as libc::c_int as isize)
                    as *const libc::c_char,
            );
            if i___0 > 5 as libc::c_int {
                current_block_11 = 11459959175219260272;
            } else if i___0 == 5 as libc::c_int {
                i___0 = atoi(
                    (uts.release).as_mut_ptr().offset(4 as libc::c_int as isize)
                        as *const libc::c_char,
                );
                if i___0 >= 4 as libc::c_int {
                    current_block_11 = 11459959175219260272;
                } else {
                    current_block_11 = 4488286894823169796;
                }
            } else if i___0 == 2 as libc::c_int {
                i___0 = atoi(
                    (uts.release).as_mut_ptr().offset(4 as libc::c_int as isize)
                        as *const libc::c_char,
                );
                if i___0 >= 30 as libc::c_int {
                    current_block_11 = 11459959175219260272;
                } else {
                    current_block_11 = 4488286894823169796;
                }
            } else {
                current_block_11 = 4488286894823169796;
            }
        } else {
            current_block_11 = 11459959175219260272;
        }
        match current_block_11 {
            11459959175219260272 => {}
            _ => {
                (*env).me_flags |= 134217728 as libc::c_uint;
            }
        }
    }
    i = mdb_env_read_header(env, prev, &mut meta);
    if i != 0 as libc::c_int {
        if i != 2 as libc::c_int {
            return i;
        }
        newenv = 1 as libc::c_int;
        (*env).me_psize = (*env).me_os_psize;
        if (*env).me_psize > 32768 as libc::c_uint {
            (*env).me_psize = 32768 as libc::c_uint;
        }
        memset(
            &mut meta as *mut MDB_meta as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<MDB_meta>() as libc::c_ulong,
        );
        mdb_env_init_meta0(env, &mut meta);
        meta.mm_mapsize = 1048576 as libc::c_int as mdb_size_t;
    } else {
        (*env).me_psize = meta.mm_dbs[0 as libc::c_int as usize].md_pad;
    }
    if (*env).me_mapsize == 0 {
        (*env).me_mapsize = meta.mm_mapsize;
    }
    minsize = (meta.mm_last_pg)
        .wrapping_add(1 as libc::c_ulong)
        .wrapping_mul(meta.mm_dbs[0 as libc::c_int as usize].md_pad as pgno_t);
    if (*env).me_mapsize < minsize {
        (*env).me_mapsize = minsize;
    }
    meta.mm_mapsize = (*env).me_mapsize;
    if newenv != 0 {
        if flags & 1 as libc::c_uint == 0 {
            rc = mdb_env_init_meta(env, &mut meta);
            if rc != 0 {
                return rc;
            }
            newenv = 0 as libc::c_int;
        }
    }
    if flags & 1 as libc::c_uint != 0 {
        tmp___1 = meta.mm_address;
    } else {
        tmp___1 = 0 as *mut libc::c_void;
    }
    rc = mdb_env_map(env, tmp___1);
    if rc != 0 {
        return rc;
    }
    if newenv != 0 {
        if flags & 1 as libc::c_uint != 0 {
            meta.mm_address = (*env).me_map as *mut libc::c_void;
        }
        i = mdb_env_init_meta(env, &mut meta);
        if i != 0 as libc::c_int {
            return i;
        }
    }
    (*env)
        .me_maxfree_1pg = (((*env).me_psize)
        .wrapping_sub(
            &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                as libc::c_uint,
        ) as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<pgno_t>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_ulong) as libc::c_int;
    (*env)
        .me_nodemax = ((((*env).me_psize)
        .wrapping_sub(
            &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                as libc::c_uint,
        )
        .wrapping_div(2 as libc::c_uint) & 4294967294 as libc::c_uint) as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<indx_t>() as libc::c_ulong) as libc::c_uint;
    (*env).me_maxpg = ((*env).me_mapsize).wrapping_div((*env).me_psize as mdb_size_t);
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_env_reader_dest(mut ptr: *mut libc::c_void) {
    let mut reader: *mut MDB_reader = 0 as *mut MDB_reader;
    let mut tmp: __pid_t = 0;
    reader = ptr as *mut MDB_reader;
    tmp = getpid();
    if (*reader).mru.mrx.mrb_pid == tmp {
        (*reader).mru.mrx.mrb_pid = 0 as libc::c_int;
    }
}
unsafe extern "C" fn mdb_env_share_locks(
    mut env: *mut MDB_env,
    mut excl: *mut libc::c_int,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut meta: *mut MDB_meta = 0 as *mut MDB_meta;
    let mut tmp: *mut MDB_meta = 0 as *mut MDB_meta;
    let mut lock_info: flock = flock {
        l_type: 0,
        l_whence: 0,
        l_start: 0,
        l_len: 0,
        l_pid: 0,
    };
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    rc = 0 as libc::c_int;
    tmp = mdb_env_pick_meta(env as *const MDB_env);
    meta = tmp;
    (*(*env).me_txns).mt1.mtb.mtb_txnid = (*meta).mm_txnid;
    memset(
        &mut lock_info as *mut flock as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<flock>() as libc::c_ulong,
    );
    lock_info.l_type = 0 as libc::c_int as libc::c_short;
    lock_info.l_whence = 0 as libc::c_int as libc::c_short;
    lock_info.l_start = 0 as libc::c_int as __off_t;
    lock_info.l_len = 1 as libc::c_int as __off_t;
    loop {
        rc = fcntl((*env).me_lfd, 6 as libc::c_int, &mut lock_info as *mut flock);
        if !(rc != 0) {
            break;
        }
        tmp___0 = __errno_location();
        rc = *tmp___0;
        if !(rc == 4 as libc::c_int) {
            break;
        }
    }
    if rc != 0 {
        *excl = -(1 as libc::c_int);
    } else {
        *excl = 0 as libc::c_int;
    }
    return rc;
}
unsafe extern "C" fn mdb_env_excl_lock(
    mut env: *mut MDB_env,
    mut excl: *mut libc::c_int,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut lock_info: flock = flock {
        l_type: 0,
        l_whence: 0,
        l_start: 0,
        l_len: 0,
        l_pid: 0,
    };
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    rc = 0 as libc::c_int;
    memset(
        &mut lock_info as *mut flock as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<flock>() as libc::c_ulong,
    );
    lock_info.l_type = 1 as libc::c_int as libc::c_short;
    lock_info.l_whence = 0 as libc::c_int as libc::c_short;
    lock_info.l_start = 0 as libc::c_int as __off_t;
    lock_info.l_len = 1 as libc::c_int as __off_t;
    loop {
        rc = fcntl((*env).me_lfd, 6 as libc::c_int, &mut lock_info as *mut flock);
        if !(rc != 0) {
            break;
        }
        tmp = __errno_location();
        rc = *tmp;
        if !(rc == 4 as libc::c_int) {
            break;
        }
    }
    if rc == 0 {
        *excl = 1 as libc::c_int;
    } else {
        lock_info.l_type = 0 as libc::c_int as libc::c_short;
        loop {
            rc = fcntl((*env).me_lfd, 7 as libc::c_int, &mut lock_info as *mut flock);
            if !(rc != 0) {
                break;
            }
            tmp___0 = __errno_location();
            rc = *tmp___0;
            if !(rc == 4 as libc::c_int) {
                break;
            }
        }
        if rc == 0 as libc::c_int {
            *excl = 0 as libc::c_int;
        }
    }
    return rc;
}
unsafe extern "C" fn mdb_env_setup_locks(
    mut env: *mut MDB_env,
    mut fname: *mut MDB_name,
    mut mode: libc::c_int,
    mut excl: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut rc: libc::c_int = 0;
    let mut size: off_t = 0;
    let mut rsize: off_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut m: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut mattr: pthread_mutexattr_t = __anonunion_pthread_mutexattr_t_488594144 {
        __size: [0; 4],
    };
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    rc = mdb_fopen(
        env as *const MDB_env,
        fname,
        MDB_O_LOCKS,
        mode as mdb_mode_t,
        &mut (*env).me_lfd,
    );
    if rc != 0 {
        if rc == 30 as libc::c_int {
            if (*env).me_flags & 131072 as libc::c_uint != 0 {
                return 0 as libc::c_int;
            }
        }
    } else {
        if (*env).me_flags & 2097152 as libc::c_uint == 0 {
            rc = pthread_key_create(
                &mut (*env).me_txkey,
                Some(
                    mdb_env_reader_dest as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            );
            if rc != 0 {
                current_block = 11577588345108706200;
            } else {
                (*env).me_flags |= 268435456 as libc::c_uint;
                current_block = 12800627514080957624;
            }
        } else {
            current_block = 12800627514080957624;
        }
        match current_block {
            11577588345108706200 => {}
            _ => {
                rc = mdb_env_excl_lock(env, excl);
                if !(rc != 0) {
                    size = lseek(
                        (*env).me_lfd,
                        0 as libc::c_int as __off_t,
                        2 as libc::c_int,
                    );
                    if size == -(1 as libc::c_long) {
                        current_block = 4405547841843181101;
                    } else {
                        rsize = (((*env).me_maxreaders).wrapping_sub(1 as libc::c_uint)
                            as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<MDB_reader>() as libc::c_ulong,
                            )
                            .wrapping_add(
                                ::std::mem::size_of::<MDB_txninfo>() as libc::c_ulong,
                            ) as off_t;
                        if size < rsize {
                            if *excl > 0 as libc::c_int {
                                tmp = ftruncate((*env).me_lfd, rsize);
                                if tmp != 0 as libc::c_int {
                                    current_block = 4405547841843181101;
                                } else {
                                    current_block = 3437258052017859086;
                                }
                            } else {
                                rsize = size;
                                size = (rsize as libc::c_ulong)
                                    .wrapping_sub(
                                        ::std::mem::size_of::<MDB_txninfo>() as libc::c_ulong,
                                    ) as off_t;
                                (*env)
                                    .me_maxreaders = (size as libc::c_ulong)
                                    .wrapping_div(
                                        ::std::mem::size_of::<MDB_reader>() as libc::c_ulong,
                                    )
                                    .wrapping_add(1 as libc::c_ulong) as libc::c_uint;
                                current_block = 3437258052017859086;
                            }
                        } else {
                            rsize = size;
                            size = (rsize as libc::c_ulong)
                                .wrapping_sub(
                                    ::std::mem::size_of::<MDB_txninfo>() as libc::c_ulong,
                                ) as off_t;
                            (*env)
                                .me_maxreaders = (size as libc::c_ulong)
                                .wrapping_div(
                                    ::std::mem::size_of::<MDB_reader>() as libc::c_ulong,
                                )
                                .wrapping_add(1 as libc::c_ulong) as libc::c_uint;
                            current_block = 3437258052017859086;
                        }
                        match current_block {
                            4405547841843181101 => {}
                            _ => {
                                tmp___0 = mmap(
                                    0 as *mut libc::c_void,
                                    rsize as size_t,
                                    3 as libc::c_int,
                                    1 as libc::c_int,
                                    (*env).me_lfd,
                                    0 as libc::c_int as __off_t,
                                );
                                m = tmp___0;
                                if m as libc::c_ulong
                                    == -(1 as libc::c_int) as *mut libc::c_void as libc::c_ulong
                                {
                                    current_block = 4405547841843181101;
                                } else {
                                    (*env).me_txns = m as *mut MDB_txninfo;
                                    if *excl > 0 as libc::c_int {
                                        memset(
                                            ((*(*env).me_txns).mt1.mtb.mtb_rmutex).as_mut_ptr()
                                                as *mut libc::c_void,
                                            0 as libc::c_int,
                                            ::std::mem::size_of::<pthread_mutex_t>() as libc::c_ulong,
                                        );
                                        memset(
                                            ((*(*env).me_txns).mt2.mt2_wmutex).as_mut_ptr()
                                                as *mut libc::c_void,
                                            0 as libc::c_int,
                                            ::std::mem::size_of::<pthread_mutex_t>() as libc::c_ulong,
                                        );
                                        rc = pthread_mutexattr_init(&mut mattr);
                                        if rc != 0 as libc::c_int {
                                            current_block = 11577588345108706200;
                                        } else {
                                            rc = pthread_mutexattr_setpshared(
                                                &mut mattr,
                                                1 as libc::c_int,
                                            );
                                            if rc == 0 {
                                                rc = pthread_mutexattr_setrobust(
                                                    &mut mattr,
                                                    1 as libc::c_int,
                                                );
                                            }
                                            if rc == 0 {
                                                rc = pthread_mutex_init(
                                                    ((*(*env).me_txns).mt1.mtb.mtb_rmutex).as_mut_ptr(),
                                                    &mut mattr as *mut pthread_mutexattr_t
                                                        as *const pthread_mutexattr_t,
                                                );
                                            }
                                            if rc == 0 {
                                                rc = pthread_mutex_init(
                                                    ((*(*env).me_txns).mt2.mt2_wmutex).as_mut_ptr(),
                                                    &mut mattr as *mut pthread_mutexattr_t
                                                        as *const pthread_mutexattr_t,
                                                );
                                            }
                                            pthread_mutexattr_destroy(&mut mattr);
                                            if rc != 0 {
                                                current_block = 11577588345108706200;
                                            } else {
                                                (*(*env).me_txns)
                                                    .mt1
                                                    .mtb
                                                    .mtb_magic = 3203383518 as libc::c_uint;
                                                (*(*env).me_txns)
                                                    .mt1
                                                    .mtb
                                                    .mtb_format = (2 as libc::c_uint)
                                                    .wrapping_rem((1 as libc::c_uint) << 12 as libc::c_int)
                                                    .wrapping_add(
                                                        (180982 as libc::c_uint)
                                                            .wrapping_mul((1 as libc::c_uint) << 12 as libc::c_int),
                                                    );
                                                (*(*env).me_txns)
                                                    .mt1
                                                    .mtb
                                                    .mtb_txnid = 0 as libc::c_int as txnid_t;
                                                (*(*env).me_txns)
                                                    .mt1
                                                    .mtb
                                                    .mtb_numreaders = 0 as libc::c_int as libc::c_uint;
                                                current_block = 9859671972921157070;
                                            }
                                        }
                                    } else if (*(*env).me_txns).mt1.mtb.mtb_magic
                                            != 3203383518 as libc::c_uint
                                        {
                                        rc = -(30793 as libc::c_int);
                                        current_block = 11577588345108706200;
                                    } else if (*(*env).me_txns).mt1.mtb.mtb_format
                                            != (2 as libc::c_uint)
                                                .wrapping_rem((1 as libc::c_uint) << 12 as libc::c_int)
                                                .wrapping_add(
                                                    (180982 as libc::c_uint)
                                                        .wrapping_mul((1 as libc::c_uint) << 12 as libc::c_int),
                                                )
                                        {
                                        rc = -(30794 as libc::c_int);
                                        current_block = 11577588345108706200;
                                    } else {
                                        tmp___1 = __errno_location();
                                        rc = *tmp___1;
                                        if rc != 0 {
                                            if rc != 13 as libc::c_int {
                                                if rc != 11 as libc::c_int {
                                                    current_block = 11577588345108706200;
                                                } else {
                                                    current_block = 9859671972921157070;
                                                }
                                            } else {
                                                current_block = 9859671972921157070;
                                            }
                                        } else {
                                            current_block = 9859671972921157070;
                                        }
                                    }
                                    match current_block {
                                        11577588345108706200 => {}
                                        _ => return 0 as libc::c_int,
                                    }
                                }
                            }
                        }
                    }
                    match current_block {
                        11577588345108706200 => {}
                        _ => {
                            tmp___2 = __errno_location();
                            rc = *tmp___2;
                        }
                    }
                }
            }
        }
    }
    return rc;
}
pub unsafe extern "C" fn mdb_env_open(
    mut env: *mut MDB_env,
    mut path: *const libc::c_char,
    mut flags: libc::c_uint,
    mut mode: mdb_mode_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut rc: libc::c_int = 0;
    let mut excl: libc::c_int = 0;
    let mut fname: MDB_name = MDB_name {
        mn_len: 0,
        mn_alloced: 0,
        mn_val: 0 as *mut mdb_nchar_t,
    };
    let mut tmp: MDB_IDL = 0 as *mut MDB_ID;
    let mut tmp___0: MDB_ID2L = 0 as *mut MDB_ID2;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___5: libc::c_int = 0;
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut tsize: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut tmp___6: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___7: *mut libc::c_void = 0 as *mut libc::c_void;
    excl = -(1 as libc::c_int);
    if (*env).me_fd != -(1 as libc::c_int) {
        return 22 as libc::c_int
    } else {
        if flags & 4227907582 as libc::c_uint != 0 {
            return 22 as libc::c_int;
        }
    }
    flags |= (*env).me_flags;
    rc = mdb_fname_init(path, flags, &mut fname);
    if rc != 0 {
        return rc;
    }
    flags |= 536870912 as libc::c_uint;
    if flags & 131072 as libc::c_uint != 0 {
        flags &= 4294443007 as libc::c_uint;
    } else {
        tmp = mdb_midl_alloc(
            ((1 as libc::c_int) << 17 as libc::c_int) - 1 as libc::c_int,
        );
        (*env).me_free_pgs = tmp;
        if !tmp.is_null() {
            tmp___1 = calloc(
                ((1 as libc::c_int) << 17 as libc::c_int) as size_t,
                ::std::mem::size_of::<MDB_ID2>() as libc::c_ulong,
            );
            tmp___0 = tmp___1 as MDB_ID2L;
            (*env).me_dirty_list = tmp___0;
            if tmp___0.is_null() {
                rc = 12 as libc::c_int;
            }
        } else {
            rc = 12 as libc::c_int;
        }
    }
    (*env).me_flags = flags;
    if !(rc != 0) {
        (*env).me_path = strdup(path);
        tmp___2 = calloc(
            (*env).me_maxdbs as size_t,
            ::std::mem::size_of::<MDB_dbx>() as libc::c_ulong,
        );
        (*env).me_dbxs = tmp___2 as *mut MDB_dbx;
        tmp___3 = calloc(
            (*env).me_maxdbs as size_t,
            ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
        );
        (*env).me_dbflags = tmp___3 as *mut uint16_t;
        tmp___4 = calloc(
            (*env).me_maxdbs as size_t,
            ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
        );
        (*env).me_dbiseqs = tmp___4 as *mut libc::c_uint;
        if !((*env).me_dbxs).is_null() {
            if !((*env).me_path).is_null() {
                if !((*env).me_dbflags).is_null() {
                    if ((*env).me_dbiseqs).is_null() {
                        rc = 12 as libc::c_int;
                    } else {
                        let ref mut fresh11 = (*((*env).me_dbxs)
                            .offset(0 as libc::c_int as isize))
                            .md_cmp;
                        *fresh11 = Some(
                            mdb_cmp_long
                                as unsafe extern "C" fn(
                                    *const MDB_val,
                                    *const MDB_val,
                                ) -> libc::c_int,
                        );
                        if flags & 4325376 as libc::c_uint == 0 {
                            rc = mdb_env_setup_locks(
                                env,
                                &mut fname,
                                mode as libc::c_int,
                                &mut excl,
                            );
                            if rc != 0 {
                                current_block = 2240650245217424877;
                            } else if flags & 33554432 as libc::c_uint != 0 {
                                if excl == 0 {
                                    rc = 11 as libc::c_int;
                                    current_block = 2240650245217424877;
                                } else {
                                    current_block = 11793792312832361944;
                                }
                            } else {
                                current_block = 11793792312832361944;
                            }
                        } else {
                            current_block = 11793792312832361944;
                        }
                        match current_block {
                            2240650245217424877 => {}
                            _ => {
                                if flags & 131072 as libc::c_uint != 0 {
                                    tmp___5 = 0 as libc::c_int;
                                } else {
                                    tmp___5 = 66 as libc::c_int;
                                }
                                rc = mdb_fopen(
                                    env as *const MDB_env,
                                    &mut fname,
                                    tmp___5 as mdb_fopen_type,
                                    mode,
                                    &mut (*env).me_fd,
                                );
                                if !(rc != 0) {
                                    if flags & 4325376 as libc::c_uint == 131072 as libc::c_uint
                                    {
                                        rc = mdb_env_setup_locks(
                                            env,
                                            &mut fname,
                                            mode as libc::c_int,
                                            &mut excl,
                                        );
                                        if rc != 0 {
                                            current_block = 2240650245217424877;
                                        } else {
                                            current_block = 13303144130133872306;
                                        }
                                    } else {
                                        current_block = 13303144130133872306;
                                    }
                                    match current_block {
                                        2240650245217424877 => {}
                                        _ => {
                                            rc = mdb_env_open2(
                                                env,
                                                (flags & 33554432 as libc::c_uint) as libc::c_int,
                                            );
                                            if rc == 0 as libc::c_int {
                                                if flags & 655360 as libc::c_uint == 0 {
                                                    rc = mdb_fopen(
                                                        env as *const MDB_env,
                                                        &mut fname,
                                                        MDB_O_META,
                                                        mode,
                                                        &mut (*env).me_mfd,
                                                    );
                                                    if rc != 0 {
                                                        current_block = 2240650245217424877;
                                                    } else {
                                                        current_block = 1134115459065347084;
                                                    }
                                                } else {
                                                    current_block = 1134115459065347084;
                                                }
                                                match current_block {
                                                    2240650245217424877 => {}
                                                    _ => {
                                                        if excl > 0 as libc::c_int {
                                                            if flags & 33554432 as libc::c_uint == 0 {
                                                                rc = mdb_env_share_locks(env, &mut excl);
                                                                if rc != 0 {
                                                                    current_block = 2240650245217424877;
                                                                } else {
                                                                    current_block = 2606304779496145856;
                                                                }
                                                            } else {
                                                                current_block = 2606304779496145856;
                                                            }
                                                        } else {
                                                            current_block = 2606304779496145856;
                                                        }
                                                        match current_block {
                                                            2240650245217424877 => {}
                                                            _ => {
                                                                if flags & 131072 as libc::c_uint == 0 {
                                                                    tsize = ::std::mem::size_of::<MDB_txn>() as libc::c_ulong
                                                                        as libc::c_int;
                                                                    size = (tsize as libc::c_ulong)
                                                                        .wrapping_add(
                                                                            ((*env).me_maxdbs as libc::c_ulong)
                                                                                .wrapping_mul(
                                                                                    (::std::mem::size_of::<MDB_db>() as libc::c_ulong)
                                                                                        .wrapping_add(
                                                                                            ::std::mem::size_of::<*mut MDB_cursor>() as libc::c_ulong,
                                                                                        )
                                                                                        .wrapping_add(
                                                                                            ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                                                                                        )
                                                                                        .wrapping_add(1 as libc::c_ulong),
                                                                                ),
                                                                        ) as libc::c_int;
                                                                    tmp___6 = calloc(
                                                                        1 as libc::c_int as size_t,
                                                                        (*env).me_psize as size_t,
                                                                    );
                                                                    (*env).me_pbuf = tmp___6;
                                                                    if !tmp___6.is_null() {
                                                                        tmp___7 = calloc(
                                                                            1 as libc::c_int as size_t,
                                                                            size as size_t,
                                                                        );
                                                                        txn = tmp___7 as *mut MDB_txn;
                                                                        if !txn.is_null() {
                                                                            (*txn)
                                                                                .mt_dbs = (txn as *mut libc::c_char).offset(tsize as isize)
                                                                                as *mut MDB_db;
                                                                            (*txn)
                                                                                .mt_cursors = ((*txn).mt_dbs)
                                                                                .offset((*env).me_maxdbs as isize) as *mut *mut MDB_cursor;
                                                                            (*txn)
                                                                                .mt_dbiseqs = ((*txn).mt_cursors)
                                                                                .offset((*env).me_maxdbs as isize) as *mut libc::c_uint;
                                                                            (*txn)
                                                                                .mt_dbflags = ((*txn).mt_dbiseqs)
                                                                                .offset((*env).me_maxdbs as isize) as *mut libc::c_uchar;
                                                                            (*txn).mt_env = env;
                                                                            (*txn).mt_dbxs = (*env).me_dbxs;
                                                                            (*txn).mt_flags = 1 as libc::c_uint;
                                                                            (*env).me_txn0 = txn;
                                                                        } else {
                                                                            rc = 12 as libc::c_int;
                                                                        }
                                                                    } else {
                                                                        rc = 12 as libc::c_int;
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
                } else {
                    rc = 12 as libc::c_int;
                }
            } else {
                rc = 12 as libc::c_int;
            }
        } else {
            rc = 12 as libc::c_int;
        }
    }
    if rc != 0 {
        mdb_env_close0(env, excl);
    }
    if fname.mn_alloced != 0 {
        free(fname.mn_val as *mut libc::c_void);
    }
    return rc;
}
unsafe extern "C" fn mdb_env_close0(mut env: *mut MDB_env, mut excl: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut pid: pid_t = 0;
    let mut tmp: __pid_t = 0;
    if (*env).me_flags & 536870912 as libc::c_uint == 0 {
        return;
    }
    if !((*env).me_dbxs).is_null() {
        i = (*env).me_maxdbs as libc::c_int;
        loop {
            i -= 1;
            if !(i >= 2 as libc::c_int) {
                break;
            }
            free((*((*env).me_dbxs).offset(i as isize)).md_name.mv_data);
        }
        free((*env).me_dbxs as *mut libc::c_void);
    }
    free((*env).me_pbuf);
    free((*env).me_dbiseqs as *mut libc::c_void);
    free((*env).me_dbflags as *mut libc::c_void);
    free((*env).me_path as *mut libc::c_void);
    free((*env).me_dirty_list as *mut libc::c_void);
    free((*env).me_txn0 as *mut libc::c_void);
    mdb_midl_free((*env).me_free_pgs);
    if (*env).me_flags & 268435456 as libc::c_uint != 0 {
        pthread_key_delete((*env).me_txkey);
    }
    if !((*env).me_map).is_null() {
        munmap((*env).me_map as *mut libc::c_void, (*env).me_mapsize);
    }
    if (*env).me_mfd != -(1 as libc::c_int) {
        close((*env).me_mfd);
    }
    if (*env).me_fd != -(1 as libc::c_int) {
        close((*env).me_fd);
    }
    if !((*env).me_txns).is_null() {
        tmp = getpid();
        pid = tmp;
        i = (*env).me_close_readers;
        loop {
            i -= 1;
            if !(i >= 0 as libc::c_int) {
                break;
            }
            if (*((*(*env).me_txns).mti_readers).as_mut_ptr().offset(i as isize))
                .mru
                .mrx
                .mrb_pid == pid
            {
                (*((*(*env).me_txns).mti_readers).as_mut_ptr().offset(i as isize))
                    .mru
                    .mrx
                    .mrb_pid = 0 as libc::c_int;
            }
        }
        if excl == 0 as libc::c_int {
            mdb_env_excl_lock(env, &mut excl);
        }
        if excl > 0 as libc::c_int {
            pthread_mutex_destroy(((*(*env).me_txns).mt1.mtb.mtb_rmutex).as_mut_ptr());
            pthread_mutex_destroy(((*(*env).me_txns).mt2.mt2_wmutex).as_mut_ptr());
        }
        munmap(
            (*env).me_txns as *mut libc::c_void,
            (((*env).me_maxreaders).wrapping_sub(1 as libc::c_uint) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<MDB_reader>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<MDB_txninfo>() as libc::c_ulong),
        );
    }
    if (*env).me_lfd != -(1 as libc::c_int) {
        close((*env).me_lfd);
    }
    (*env).me_flags &= 3489660927 as libc::c_uint;
}
pub unsafe extern "C" fn mdb_env_close(mut env: *mut MDB_env) {
    let mut dp: *mut MDB_page = 0 as *mut MDB_page;
    if env as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    loop {
        dp = (*env).me_dpages;
        if !(dp as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
            break;
        }
        (*env).me_dpages = (*dp).mp_p.p_next;
        free(dp as *mut libc::c_void);
    }
    mdb_env_close0(env, 0 as libc::c_int);
    free(env as *mut libc::c_void);
}
unsafe extern "C" fn mdb_cmp_long(
    mut a: *const MDB_val,
    mut b: *const MDB_val,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    if *((*a).mv_data as *mut mdb_size_t) < *((*b).mv_data as *mut mdb_size_t) {
        tmp = -(1 as libc::c_int);
    } else {
        tmp = (*((*a).mv_data as *mut mdb_size_t) > *((*b).mv_data as *mut mdb_size_t))
            as libc::c_int;
    }
    return tmp;
}
unsafe extern "C" fn mdb_cmp_int(
    mut a: *const MDB_val,
    mut b: *const MDB_val,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    if *((*a).mv_data as *mut libc::c_uint) < *((*b).mv_data as *mut libc::c_uint) {
        tmp = -(1 as libc::c_int);
    } else {
        tmp = (*((*a).mv_data as *mut libc::c_uint)
            > *((*b).mv_data as *mut libc::c_uint)) as libc::c_int;
    }
    return tmp;
}
unsafe extern "C" fn mdb_cmp_cint(
    mut a: *const MDB_val,
    mut b: *const MDB_val,
) -> libc::c_int {
    let mut u: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    let mut c: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    let mut x: libc::c_int = 0;
    u = ((*a).mv_data as *mut libc::c_char).offset((*a).mv_size as isize)
        as *mut libc::c_ushort;
    c = ((*b).mv_data as *mut libc::c_char).offset((*a).mv_size as isize)
        as *mut libc::c_ushort;
    loop {
        u = u.offset(-1);
        c = c.offset(-1);
        x = *u as libc::c_int - *c as libc::c_int;
        if !(x == 0) {
            break;
        }
        if !(u as libc::c_ulong > (*a).mv_data as *mut libc::c_ushort as libc::c_ulong) {
            break;
        }
    }
    return x;
}
unsafe extern "C" fn mdb_cmp_memn(
    mut a: *const MDB_val,
    mut b: *const MDB_val,
) -> libc::c_int {
    let mut diff: libc::c_int = 0;
    let mut len_diff: ssize_t = 0;
    let mut len: libc::c_uint = 0;
    let mut tmp: ssize_t = 0;
    let mut tmp___0: ssize_t = 0;
    len = (*a).mv_size as libc::c_uint;
    len_diff = (*a).mv_size as ssize_t - (*b).mv_size as ssize_t;
    if len_diff > 0 as libc::c_long {
        len = (*b).mv_size as libc::c_uint;
        len_diff = 1 as libc::c_int as ssize_t;
    }
    diff = memcmp(
        (*a).mv_data as *const libc::c_void,
        (*b).mv_data as *const libc::c_void,
        len as size_t,
    );
    if diff != 0 {
        tmp___0 = diff as ssize_t;
    } else {
        if len_diff < 0 as libc::c_long {
            tmp = -(1 as libc::c_int) as ssize_t;
        } else {
            tmp = len_diff;
        }
        tmp___0 = tmp;
    }
    return tmp___0 as libc::c_int;
}
unsafe extern "C" fn mdb_cmp_memnr(
    mut a: *const MDB_val,
    mut b: *const MDB_val,
) -> libc::c_int {
    let mut p1: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut p2: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut p1_lim: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut len_diff: ssize_t = 0;
    let mut diff: libc::c_int = 0;
    let mut tmp: ssize_t = 0;
    p1_lim = (*a).mv_data as *const libc::c_uchar;
    p1 = ((*a).mv_data as *const libc::c_uchar).offset((*a).mv_size as isize);
    p2 = ((*b).mv_data as *const libc::c_uchar).offset((*b).mv_size as isize);
    len_diff = (*a).mv_size as ssize_t - (*b).mv_size as ssize_t;
    if len_diff > 0 as libc::c_long {
        p1_lim = p1_lim.offset(len_diff as isize);
        len_diff = 1 as libc::c_int as ssize_t;
    }
    while p1 as libc::c_ulong > p1_lim as libc::c_ulong {
        p1 = p1.offset(-1);
        p2 = p2.offset(-1);
        diff = *p1 as libc::c_int - *p2 as libc::c_int;
        if diff != 0 {
            return diff;
        }
    }
    if len_diff < 0 as libc::c_long {
        tmp = -(1 as libc::c_int) as ssize_t;
    } else {
        tmp = len_diff;
    }
    return tmp as libc::c_int;
}
unsafe extern "C" fn mdb_node_search(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut exactp: *mut libc::c_int,
) -> *mut MDB_node {
    let mut i: libc::c_uint = 0;
    let mut nkeys: libc::c_uint = 0;
    let mut low: libc::c_int = 0;
    let mut high: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut node: *mut MDB_node = 0 as *mut MDB_node;
    let mut nodekey: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut cmp: Option::<MDB_cmp_func> = None;
    let mut tmp: libc::c_int = 0;
    i = 0 as libc::c_uint;
    rc = 0 as libc::c_int;
    mp = (*mc).mc_pg[(*mc).mc_top as usize];
    node = 0 as *mut libc::c_void as *mut MDB_node;
    nkeys = ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
        .wrapping_sub(
            &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                as libc::c_uint,
        ) >> 1 as libc::c_int;
    if (*mp).mp_flags as libc::c_int & 2 as libc::c_int == 2 as libc::c_int {
        low = 0 as libc::c_int;
    } else {
        low = 1 as libc::c_int;
    }
    high = nkeys.wrapping_sub(1 as libc::c_uint) as libc::c_int;
    cmp = (*(*mc).mc_dbx).md_cmp;
    if ::std::mem::transmute::<Option::<MDB_cmp_func>, libc::c_ulong>(cmp)
        == ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*const MDB_val, *const MDB_val) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                mdb_cmp_cint
                    as unsafe extern "C" fn(
                        *const MDB_val,
                        *const MDB_val,
                    ) -> libc::c_int,
            ),
        )
    {
        if (*mp).mp_flags as libc::c_int & 1 as libc::c_int == 1 as libc::c_int {
            if (*((mp as *mut libc::c_char)
                .offset(
                    *((*mp).mp_ptrs).as_mut_ptr().offset(1 as libc::c_int as isize)
                        as libc::c_int as isize,
                )
                .offset(0 as libc::c_uint as isize) as *mut MDB_node))
                .mn_ksize as libc::c_ulong
                == ::std::mem::size_of::<mdb_size_t>() as libc::c_ulong
            {
                cmp = Some(
                    mdb_cmp_long
                        as unsafe extern "C" fn(
                            *const MDB_val,
                            *const MDB_val,
                        ) -> libc::c_int,
                );
            } else {
                cmp = Some(
                    mdb_cmp_int
                        as unsafe extern "C" fn(
                            *const MDB_val,
                            *const MDB_val,
                        ) -> libc::c_int,
                );
            }
        }
    }
    if (*mp).mp_flags as libc::c_int & 32 as libc::c_int == 32 as libc::c_int {
        nodekey.mv_size = (*(*mc).mc_db).md_pad as size_t;
        node = (mp as *mut libc::c_char)
            .offset(
                *((*mp).mp_ptrs).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as libc::c_int as isize,
            )
            .offset(0 as libc::c_uint as isize) as *mut MDB_node;
        while low <= high {
            i = (low + high >> 1 as libc::c_int) as libc::c_uint;
            nodekey
                .mv_data = (mp as *mut libc::c_char)
                .offset(
                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                        as libc::c_ulong as libc::c_uint as isize,
                )
                .offset((i as size_t).wrapping_mul(nodekey.mv_size) as isize)
                as *mut libc::c_void;
            rc = (Some(cmp.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(key as *const MDB_val, &mut nodekey as *mut MDB_val as *const MDB_val);
            if rc == 0 as libc::c_int {
                break;
            }
            if rc > 0 as libc::c_int {
                low = i.wrapping_add(1 as libc::c_uint) as libc::c_int;
            } else {
                high = i.wrapping_sub(1 as libc::c_uint) as libc::c_int;
            }
        }
    } else {
        while low <= high {
            i = (low + high >> 1 as libc::c_int) as libc::c_uint;
            node = (mp as *mut libc::c_char)
                .offset(
                    *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize) as libc::c_int
                        as isize,
                )
                .offset(0 as libc::c_uint as isize) as *mut MDB_node;
            nodekey.mv_size = (*node).mn_ksize as size_t;
            nodekey.mv_data = ((*node).mn_data).as_mut_ptr() as *mut libc::c_void;
            rc = (Some(cmp.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(key as *const MDB_val, &mut nodekey as *mut MDB_val as *const MDB_val);
            if rc == 0 as libc::c_int {
                break;
            }
            if rc > 0 as libc::c_int {
                low = i.wrapping_add(1 as libc::c_uint) as libc::c_int;
            } else {
                high = i.wrapping_sub(1 as libc::c_uint) as libc::c_int;
            }
        }
    }
    if rc > 0 as libc::c_int {
        i = i.wrapping_add(1);
        if !((*mp).mp_flags as libc::c_int & 32 as libc::c_int == 32 as libc::c_int) {
            node = (mp as *mut libc::c_char)
                .offset(
                    *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize) as libc::c_int
                        as isize,
                )
                .offset(0 as libc::c_uint as isize) as *mut MDB_node;
        }
    }
    if !exactp.is_null() {
        if rc == 0 as libc::c_int {
            if nkeys > 0 as libc::c_uint {
                tmp = 1 as libc::c_int;
            } else {
                tmp = 0 as libc::c_int;
            }
        } else {
            tmp = 0 as libc::c_int;
        }
        *exactp = tmp;
    }
    (*mc).mc_ki[(*mc).mc_top as usize] = i as indx_t;
    if i >= nkeys {
        return 0 as *mut libc::c_void as *mut MDB_node;
    }
    return node;
}
unsafe extern "C" fn mdb_cursor_pop(mut mc: *mut MDB_cursor) {
    if (*mc).mc_snum != 0 {
        (*mc)
            .mc_snum = ((*mc).mc_snum as libc::c_int - 1 as libc::c_int)
            as libc::c_ushort;
        if (*mc).mc_snum != 0 {
            (*mc)
                .mc_top = ((*mc).mc_top as libc::c_int - 1 as libc::c_int)
                as libc::c_ushort;
        } else {
            (*mc).mc_flags &= 4294967294 as libc::c_uint;
        }
    }
}
unsafe extern "C" fn mdb_cursor_push(
    mut mc: *mut MDB_cursor,
    mut mp: *mut MDB_page,
) -> libc::c_int {
    let mut tmp: libc::c_ushort = 0;
    if (*mc).mc_snum as libc::c_int >= 32 as libc::c_int {
        (*(*mc).mc_txn).mt_flags |= 2 as libc::c_uint;
        return -(30787 as libc::c_int);
    }
    tmp = (*mc).mc_snum;
    (*mc).mc_snum = ((*mc).mc_snum as libc::c_int + 1 as libc::c_int) as libc::c_ushort;
    (*mc).mc_top = tmp;
    (*mc).mc_pg[(*mc).mc_top as usize] = mp;
    (*mc).mc_ki[(*mc).mc_top as usize] = 0 as libc::c_int as indx_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_page_get(
    mut mc: *mut MDB_cursor,
    mut pgno: pgno_t,
    mut ret: *mut *mut MDB_page,
    mut lvl: *mut libc::c_int,
) -> libc::c_int {
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut p: *mut MDB_page = 0 as *mut MDB_page;
    let mut level: libc::c_int = 0;
    let mut tx2: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut dl: MDB_ID2L = 0 as *mut MDB_ID2;
    let mut x: libc::c_uint = 0;
    let mut pn: MDB_ID = 0;
    let mut x___0: libc::c_uint = 0;
    let mut tmp: libc::c_uint = 0;
    let mut env: *mut MDB_env = 0 as *mut MDB_env;
    let mut current_block_23: u64;
    txn = (*mc).mc_txn;
    p = 0 as *mut libc::c_void as *mut MDB_page;
    if (*mc).mc_flags & 655360 as libc::c_uint == 0 {
        tx2 = txn;
        level = 1 as libc::c_int;
        loop {
            dl = (*tx2).mt_u.dirty_list;
            if !((*tx2).mt_spill_pgs).is_null() {
                pn = pgno << 1 as libc::c_int;
                x = mdb_midl_search((*tx2).mt_spill_pgs, pn);
                if x as MDB_ID
                    <= *((*tx2).mt_spill_pgs).offset(0 as libc::c_int as isize)
                {
                    if *((*tx2).mt_spill_pgs).offset(x as isize) == pn {
                        current_block_23 = 16262438568338155876;
                        break;
                    }
                }
            }
            if (*dl.offset(0 as libc::c_int as isize)).mid != 0 {
                tmp = mdb_mid2l_search(dl, pgno);
                x___0 = tmp;
                if x___0 as MDB_ID <= (*dl.offset(0 as libc::c_int as isize)).mid {
                    if (*dl.offset(x___0 as isize)).mid == pgno {
                        p = (*dl.offset(x___0 as isize)).mptr as *mut MDB_page;
                        current_block_23 = 7786757759290214883;
                        break;
                    }
                }
            }
            level += 1;
            tx2 = (*tx2).mt_parent;
            if !(tx2 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
                current_block_23 = 11194104282611034094;
                break;
            }
        }
    } else {
        current_block_23 = 11194104282611034094;
    }
    match current_block_23 {
        11194104282611034094 => {
            if pgno >= (*txn).mt_next_pgno {
                (*txn).mt_flags |= 2 as libc::c_uint;
                return -(30797 as libc::c_int);
            }
            level = 0 as libc::c_int;
            current_block_23 = 16262438568338155876;
        }
        _ => {}
    }
    match current_block_23 {
        16262438568338155876 => {
            env = (*txn).mt_env;
            p = ((*env).me_map)
                .offset(((*env).me_psize as pgno_t).wrapping_mul(pgno) as isize)
                as *mut MDB_page;
        }
        _ => {}
    }
    *ret = p;
    if !lvl.is_null() {
        *lvl = level;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_page_search_root(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut rc: libc::c_int = 0;
    let mut node: *mut MDB_node = 0 as *mut MDB_node;
    let mut i: indx_t = 0;
    let mut tmp: libc::c_ushort = 0;
    let mut exact: libc::c_int = 0;
    mp = (*mc).mc_pg[(*mc).mc_top as usize];
    while (*mp).mp_flags as libc::c_int & 1 as libc::c_int == 1 as libc::c_int {
        let mut current_block_44: u64;
        if (*mc).mc_dbi != 0 {
            if !(((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                .wrapping_sub(
                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                        as libc::c_ulong as libc::c_uint,
                ) >> 1 as libc::c_int > 1 as libc::c_uint)
            {
                mdb_assert_fail(
                    (*(*mc).mc_txn).mt_env,
                    b"!mc->mc_dbi || NUMKEYS(mp) > 1\0" as *const u8
                        as *const libc::c_char,
                    b"mdb_page_search_root\0" as *const u8 as *const libc::c_char,
                    b"mdb.c\0" as *const u8 as *const libc::c_char,
                    6482 as libc::c_int,
                );
            }
        }
        if flags & 12 as libc::c_int != 0 {
            i = 0 as libc::c_int as indx_t;
            if flags & 8 as libc::c_int != 0 {
                i = (((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                    .wrapping_sub(
                        &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                            as libc::c_ulong as libc::c_uint,
                    ) >> 1 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint) as indx_t;
                if (*mc).mc_flags & 1 as libc::c_uint != 0 {
                    if (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_int
                        == i as libc::c_int
                    {
                        tmp = (*mc).mc_snum;
                        (*mc)
                            .mc_snum = ((*mc).mc_snum as libc::c_int + 1 as libc::c_int)
                            as libc::c_ushort;
                        (*mc).mc_top = tmp;
                        mp = (*mc).mc_pg[(*mc).mc_top as usize];
                        current_block_44 = 7980409359141560661;
                    } else {
                        current_block_44 = 15925075030174552612;
                    }
                } else {
                    current_block_44 = 15925075030174552612;
                }
            } else {
                current_block_44 = 15925075030174552612;
            }
        } else {
            node = mdb_node_search(mc, key, &mut exact);
            if node as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                i = (((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                    .wrapping_sub(
                        &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                            as libc::c_ulong as libc::c_uint,
                    ) >> 1 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint) as indx_t;
            } else {
                i = (*mc).mc_ki[(*mc).mc_top as usize];
                if exact == 0 {
                    if !(i as libc::c_int > 0 as libc::c_int) {
                        mdb_assert_fail(
                            (*(*mc).mc_txn).mt_env,
                            b"i > 0\0" as *const u8 as *const libc::c_char,
                            b"mdb_page_search_root\0" as *const u8
                                as *const libc::c_char,
                            b"mdb.c\0" as *const u8 as *const libc::c_char,
                            6506 as libc::c_int,
                        );
                    }
                    i = (i as libc::c_int - 1 as libc::c_int) as indx_t;
                }
            }
            current_block_44 = 15925075030174552612;
        }
        match current_block_44 {
            15925075030174552612 => {
                if !((i as libc::c_uint)
                    < ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                        .wrapping_sub(
                            &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                as libc::c_ulong as libc::c_uint,
                        ) >> 1 as libc::c_int)
                {
                    mdb_assert_fail(
                        (*(*mc).mc_txn).mt_env,
                        b"i < NUMKEYS(mp)\0" as *const u8 as *const libc::c_char,
                        b"mdb_page_search_root\0" as *const u8 as *const libc::c_char,
                        b"mdb.c\0" as *const u8 as *const libc::c_char,
                        6513 as libc::c_int,
                    );
                }
                node = (mp as *mut libc::c_char)
                    .offset(
                        *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize) as libc::c_int
                            as isize,
                    )
                    .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                rc = mdb_page_get(
                    mc,
                    (*node).mn_lo as libc::c_ulong
                        | ((*node).mn_hi as pgno_t) << 16 as libc::c_int
                        | ((*node).mn_flags as pgno_t) << 32 as libc::c_int,
                    &mut mp,
                    0 as *mut libc::c_void as *mut libc::c_int,
                );
                if rc != 0 as libc::c_int {
                    return rc;
                }
                (*mc).mc_ki[(*mc).mc_top as usize] = i;
                rc = mdb_cursor_push(mc, mp);
                if rc != 0 {
                    return rc;
                }
            }
            _ => {}
        }
        if flags & 1 as libc::c_int != 0 {
            rc = mdb_page_touch(mc);
            if rc != 0 as libc::c_int {
                return rc;
            }
            mp = (*mc).mc_pg[(*mc).mc_top as usize];
        }
    }
    if !((*mp).mp_flags as libc::c_int & 2 as libc::c_int == 2 as libc::c_int) {
        (*(*mc).mc_txn).mt_flags |= 2 as libc::c_uint;
        return -(30796 as libc::c_int);
    }
    (*mc).mc_flags |= 1 as libc::c_uint;
    (*mc).mc_flags &= 4294967293 as libc::c_uint;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_page_search_lowest(mut mc: *mut MDB_cursor) -> libc::c_int {
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut node: *mut MDB_node = 0 as *mut MDB_node;
    let mut rc: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    mp = (*mc).mc_pg[(*mc).mc_top as usize];
    node = (mp as *mut libc::c_char)
        .offset(
            *((*mp).mp_ptrs).as_mut_ptr().offset(0 as libc::c_int as isize)
                as libc::c_int as isize,
        )
        .offset(0 as libc::c_uint as isize) as *mut MDB_node;
    rc = mdb_page_get(
        mc,
        (*node).mn_lo as libc::c_ulong | ((*node).mn_hi as pgno_t) << 16 as libc::c_int
            | ((*node).mn_flags as pgno_t) << 32 as libc::c_int,
        &mut mp,
        0 as *mut libc::c_void as *mut libc::c_int,
    );
    if rc != 0 as libc::c_int {
        return rc;
    }
    (*mc).mc_ki[(*mc).mc_top as usize] = 0 as libc::c_int as indx_t;
    rc = mdb_cursor_push(mc, mp);
    if rc != 0 {
        return rc;
    }
    tmp = mdb_page_search_root(
        mc,
        0 as *mut libc::c_void as *mut MDB_val,
        4 as libc::c_int,
    );
    return tmp;
}
unsafe extern "C" fn mdb_page_search(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut root: pgno_t = 0;
    let mut mc2: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut data: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut exact: libc::c_int = 0;
    let mut flags___0: uint16_t = 0;
    let mut leaf: *mut MDB_node = 0 as *mut MDB_node;
    let mut tmp: *mut MDB_node = 0 as *mut MDB_node;
    let mut tmp___0: libc::c_int = 0;
    if (*(*mc).mc_txn).mt_flags & 19 as libc::c_uint != 0 {
        return -(30782 as libc::c_int)
    } else {
        if *(*mc).mc_dbflag as libc::c_int & 2 as libc::c_int != 0 {
            if *((*(*mc).mc_txn).mt_dbiseqs).offset((*mc).mc_dbi as isize)
                != *((*(*(*mc).mc_txn).mt_env).me_dbiseqs).offset((*mc).mc_dbi as isize)
            {
                return -(30780 as libc::c_int);
            }
            mdb_cursor_init(
                &mut mc2,
                (*mc).mc_txn,
                1 as libc::c_int as MDB_dbi,
                0 as *mut libc::c_void as *mut MDB_xcursor,
            );
            rc = mdb_page_search(
                &mut mc2,
                &mut (*(*mc).mc_dbx).md_name,
                0 as libc::c_int,
            );
            if rc != 0 {
                return rc;
            }
            exact = 0 as libc::c_int;
            tmp = mdb_node_search(&mut mc2, &mut (*(*mc).mc_dbx).md_name, &mut exact);
            leaf = tmp;
            if exact == 0 {
                return -(30798 as libc::c_int);
            }
            if (*leaf).mn_flags as libc::c_int & 6 as libc::c_int != 2 as libc::c_int {
                return -(30784 as libc::c_int);
            }
            rc = mdb_node_read(&mut mc2, leaf, &mut data);
            if rc != 0 {
                return rc;
            }
            memcpy(
                &mut flags___0 as *mut uint16_t as *mut libc::c_void,
                (data.mv_data as *mut libc::c_char)
                    .offset(
                        &mut (*(0 as *mut MDB_db)).md_flags as *mut uint16_t
                            as libc::c_ulong as isize,
                    ) as *const libc::c_void,
                ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
            );
            if (*(*mc).mc_db).md_flags as libc::c_int & 32767 as libc::c_int
                != flags___0 as libc::c_int
            {
                return -(30784 as libc::c_int);
            }
            memcpy(
                (*mc).mc_db as *mut libc::c_void,
                data.mv_data as *const libc::c_void,
                ::std::mem::size_of::<MDB_db>() as libc::c_ulong,
            );
            *(*mc)
                .mc_dbflag = (*(*mc).mc_dbflag as libc::c_int & -(3 as libc::c_int))
                as libc::c_uchar;
        }
        root = (*(*mc).mc_db).md_root;
        if root as libc::c_ulonglong == 18446744073709551615 as libc::c_ulonglong {
            return -(30798 as libc::c_int);
        }
    }
    if !(root > 1 as libc::c_ulong) {
        mdb_assert_fail(
            (*(*mc).mc_txn).mt_env,
            b"root > 1\0" as *const u8 as *const libc::c_char,
            b"mdb_page_search\0" as *const u8 as *const libc::c_char,
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            6633 as libc::c_int,
        );
    }
    let mut current_block_43: u64;
    if ((*mc).mc_pg[0 as libc::c_int as usize]).is_null() {
        current_block_43 = 1984454686076872458;
    } else if (*(*mc).mc_pg[0 as libc::c_int as usize]).mp_p.p_pgno != root {
        current_block_43 = 1984454686076872458;
    } else {
        current_block_43 = 6450636197030046351;
    }
    match current_block_43 {
        1984454686076872458 => {
            rc = mdb_page_get(
                mc,
                root,
                &mut *((*mc).mc_pg).as_mut_ptr().offset(0 as libc::c_int as isize),
                0 as *mut libc::c_void as *mut libc::c_int,
            );
            if rc != 0 as libc::c_int {
                return rc;
            }
        }
        _ => {}
    }
    (*mc).mc_snum = 1 as libc::c_int as libc::c_ushort;
    (*mc).mc_top = 0 as libc::c_int as libc::c_ushort;
    if flags & 1 as libc::c_int != 0 {
        rc = mdb_page_touch(mc);
        if rc != 0 {
            return rc;
        }
    }
    if flags & 2 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    tmp___0 = mdb_page_search_root(mc, key, flags);
    return tmp___0;
}
unsafe extern "C" fn mdb_ovpage_free(
    mut mc: *mut MDB_cursor,
    mut mp: *mut MDB_page,
) -> libc::c_int {
    let mut current_block: u64;
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut pg: pgno_t = 0;
    let mut x: libc::c_uint = 0;
    let mut ovpages: libc::c_uint = 0;
    let mut env: *mut MDB_env = 0 as *mut MDB_env;
    let mut sl: MDB_IDL = 0 as *mut MDB_ID;
    let mut pn: MDB_ID = 0;
    let mut rc: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut mop: *mut pgno_t = 0 as *mut pgno_t;
    let mut dl: *mut MDB_ID2 = 0 as *mut MDB_ID2;
    let mut ix: MDB_ID2 = MDB_ID2 {
        mid: 0,
        mptr: 0 as *mut libc::c_void,
    };
    let mut iy: MDB_ID2 = MDB_ID2 {
        mid: 0,
        mptr: 0 as *mut libc::c_void,
    };
    let mut tmp: MDB_ID = 0;
    let mut tmp___0: libc::c_uint = 0;
    let mut tmp___1: libc::c_uint = 0;
    let mut tmp___2: pgno_t = 0;
    txn = (*mc).mc_txn;
    pg = (*mp).mp_p.p_pgno;
    x = 0 as libc::c_uint;
    ovpages = (*mp).mp_pb.pb_pages;
    env = (*txn).mt_env;
    sl = (*txn).mt_spill_pgs;
    pn = pg << 1 as libc::c_int;
    if !((*env).me_pgstate.mf_pghead).is_null() {
        if ((*txn).mt_parent).is_null() {
            if (*mp).mp_flags as libc::c_int & 16 as libc::c_int != 0 {
                current_block = 4608081875973427183;
            } else if !sl.is_null() {
                x = mdb_midl_search(sl, pn);
                if x as MDB_ID <= *sl.offset(0 as libc::c_int as isize) {
                    if *sl.offset(x as isize) == pn {
                        current_block = 4608081875973427183;
                    } else {
                        current_block = 854259047636918131;
                    }
                } else {
                    current_block = 854259047636918131;
                }
            } else {
                current_block = 854259047636918131;
            }
            match current_block {
                854259047636918131 => {}
                _ => {
                    rc = mdb_midl_need(&mut (*env).me_pgstate.mf_pghead, ovpages);
                    if rc != 0 {
                        return rc;
                    }
                    if (*mp).mp_flags as libc::c_int & 16 as libc::c_int == 0 {
                        if x as MDB_ID == *sl.offset(0 as libc::c_int as isize) {
                            let ref mut fresh12 = *sl.offset(0 as libc::c_int as isize);
                            *fresh12 = (*fresh12).wrapping_sub(1);
                        } else {
                            let ref mut fresh13 = *sl.offset(x as isize);
                            *fresh13 |= 1 as libc::c_ulong;
                        }
                    } else {
                        dl = (*txn).mt_u.dirty_list;
                        tmp = (*dl.offset(0 as libc::c_int as isize)).mid;
                        let ref mut fresh14 = (*dl.offset(0 as libc::c_int as isize))
                            .mid;
                        *fresh14 = (*fresh14).wrapping_sub(1);
                        x = tmp as libc::c_uint;
                        ix = *dl.offset(x as isize);
                        while ix.mptr as libc::c_ulong != mp as libc::c_ulong {
                            if x > 1 as libc::c_uint {
                                x = x.wrapping_sub(1);
                                iy = *dl.offset(x as isize);
                                *dl.offset(x as isize) = ix;
                            } else {
                                if !(x > 1 as libc::c_uint) {
                                    mdb_assert_fail(
                                        (*(*mc).mc_txn).mt_env,
                                        b"x > 1\0" as *const u8 as *const libc::c_char,
                                        b"mdb_ovpage_free\0" as *const u8 as *const libc::c_char,
                                        b"mdb.c\0" as *const u8 as *const libc::c_char,
                                        6715 as libc::c_int,
                                    );
                                }
                                let ref mut fresh15 = (*dl
                                    .offset(0 as libc::c_int as isize))
                                    .mid;
                                *fresh15 = (*fresh15).wrapping_add(1);
                                j = (*dl.offset(0 as libc::c_int as isize)).mid
                                    as libc::c_uint;
                                *dl.offset(j as isize) = ix;
                                (*txn).mt_flags |= 2 as libc::c_uint;
                                return -(30779 as libc::c_int);
                            }
                            ix = iy;
                        }
                        (*txn).mt_dirty_room = ((*txn).mt_dirty_room).wrapping_add(1);
                        if (*env).me_flags & 524288 as libc::c_uint == 0 {
                            mdb_dpage_free(env, mp);
                        }
                    }
                    mop = (*env).me_pgstate.mf_pghead;
                    j = (*mop.offset(0 as libc::c_int as isize))
                        .wrapping_add(ovpages as pgno_t) as libc::c_uint;
                    i = *mop.offset(0 as libc::c_int as isize) as libc::c_uint;
                    while i != 0 {
                        if !(*mop.offset(i as isize) < pg) {
                            break;
                        }
                        tmp___0 = j;
                        j = j.wrapping_sub(1);
                        *mop.offset(tmp___0 as isize) = *mop.offset(i as isize);
                        i = i.wrapping_sub(1);
                    }
                    while j > i {
                        tmp___1 = j;
                        j = j.wrapping_sub(1);
                        tmp___2 = pg;
                        pg = pg.wrapping_add(1);
                        *mop.offset(tmp___1 as isize) = tmp___2;
                    }
                    let ref mut fresh16 = *mop.offset(0 as libc::c_int as isize);
                    *fresh16 = (*fresh16 as libc::c_ulong)
                        .wrapping_add(ovpages as pgno_t) as pgno_t as pgno_t;
                    current_block = 18038362259723567392;
                }
            }
        } else {
            current_block = 854259047636918131;
        }
    } else {
        current_block = 854259047636918131;
    }
    match current_block {
        854259047636918131 => {
            rc = mdb_midl_append_range(&mut (*txn).mt_free_pgs, pg, ovpages);
            if rc != 0 {
                return rc;
            }
        }
        _ => {}
    }
    (*(*mc).mc_db)
        .md_overflow_pages = ((*(*mc).mc_db).md_overflow_pages as libc::c_ulong)
        .wrapping_sub(ovpages as pgno_t) as pgno_t as pgno_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_node_read(
    mut mc: *mut MDB_cursor,
    mut leaf: *mut MDB_node,
    mut data: *mut MDB_val,
) -> libc::c_int {
    let mut omp: *mut MDB_page = 0 as *mut MDB_page;
    let mut pgno: pgno_t = 0;
    let mut rc: libc::c_int = 0;
    if !((*leaf).mn_flags as libc::c_int & 1 as libc::c_int == 1 as libc::c_int) {
        (*data)
            .mv_size = ((*leaf).mn_lo as libc::c_uint
            | ((*leaf).mn_hi as libc::c_uint) << 16 as libc::c_int) as size_t;
        (*data)
            .mv_data = ((*leaf).mn_data)
            .as_mut_ptr()
            .offset((*leaf).mn_ksize as libc::c_int as isize) as *mut libc::c_void;
        return 0 as libc::c_int;
    }
    (*data)
        .mv_size = ((*leaf).mn_lo as libc::c_uint
        | ((*leaf).mn_hi as libc::c_uint) << 16 as libc::c_int) as size_t;
    memcpy(
        &mut pgno as *mut pgno_t as *mut libc::c_void,
        ((*leaf).mn_data).as_mut_ptr().offset((*leaf).mn_ksize as libc::c_int as isize)
            as *mut libc::c_void as *const libc::c_void,
        ::std::mem::size_of::<pgno_t>() as libc::c_ulong,
    );
    rc = mdb_page_get(mc, pgno, &mut omp, 0 as *mut libc::c_void as *mut libc::c_int);
    if rc != 0 as libc::c_int {
        return rc;
    }
    (*data)
        .mv_data = (omp as *mut libc::c_char)
        .offset(
            &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                as libc::c_uint as isize,
        ) as *mut libc::c_void;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_get(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
) -> libc::c_int {
    let mut mc: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut mx: MDB_xcursor = MDB_xcursor {
        mx_cursor: MDB_cursor {
            mc_next: 0 as *mut MDB_cursor,
            mc_backup: 0 as *mut MDB_cursor,
            mc_xcursor: 0 as *mut MDB_xcursor,
            mc_txn: 0 as *mut MDB_txn,
            mc_dbi: 0,
            mc_db: 0 as *mut MDB_db,
            mc_dbx: 0 as *mut MDB_dbx,
            mc_dbflag: 0 as *mut libc::c_uchar,
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [0 as *mut MDB_page; 32],
            mc_ki: [0; 32],
        },
        mx_db: MDB_db {
            md_pad: 0,
            md_flags: 0,
            md_depth: 0,
            md_branch_pages: 0,
            md_leaf_pages: 0,
            md_overflow_pages: 0,
            md_entries: 0,
            md_root: 0,
        },
        mx_dbx: MDB_dbx {
            md_name: MDB_val {
                mv_size: 0,
                mv_data: 0 as *mut libc::c_void,
            },
            md_cmp: None,
            md_dcmp: None,
            md_rel: None,
            md_relctx: 0 as *mut libc::c_void,
        },
        mx_dbflag: 0,
    };
    let mut exact: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    exact = 0 as libc::c_int;
    if key.is_null() {
        return 22 as libc::c_int
    } else {
        if data.is_null() {
            return 22 as libc::c_int
        } else {
            if !txn.is_null() {
                if dbi < (*txn).mt_numdbs {
                    if *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int
                        & 16 as libc::c_int == 0
                    {
                        return 22 as libc::c_int;
                    }
                } else {
                    return 22 as libc::c_int
                }
            } else {
                return 22 as libc::c_int
            }
        }
    }
    if (*txn).mt_flags & 19 as libc::c_uint != 0 {
        return -(30782 as libc::c_int);
    }
    mdb_cursor_init(&mut mc, txn, dbi, &mut mx);
    rc = mdb_cursor_set(&mut mc, key, data, MDB_SET, &mut exact);
    return rc;
}
unsafe extern "C" fn mdb_cursor_sibling(
    mut mc: *mut MDB_cursor,
    mut move_right: libc::c_int,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut indx: *mut MDB_node = 0 as *mut MDB_node;
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut tmp: libc::c_int = 0;
    if ((*mc).mc_snum as libc::c_int) < 2 as libc::c_int {
        return -(30798 as libc::c_int);
    }
    mdb_cursor_pop(mc);
    if move_right != 0 {
        tmp = (((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint)
            .wrapping_add(1 as libc::c_uint)
            >= ((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_pb.pb.pb_lower as libc::c_uint)
                .wrapping_sub(
                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                        as libc::c_ulong as libc::c_uint,
                ) >> 1 as libc::c_int) as libc::c_int;
    } else {
        tmp = ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_int == 0 as libc::c_int)
            as libc::c_int;
    }
    if tmp != 0 {
        rc = mdb_cursor_sibling(mc, move_right);
        if rc != 0 as libc::c_int {
            (*mc)
                .mc_top = ((*mc).mc_top as libc::c_int + 1 as libc::c_int)
                as libc::c_ushort;
            (*mc)
                .mc_snum = ((*mc).mc_snum as libc::c_int + 1 as libc::c_int)
                as libc::c_ushort;
            return rc;
        }
    } else if move_right != 0 {
        (*mc)
            .mc_ki[(*mc).mc_top
            as usize] = ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_int
            + 1 as libc::c_int) as indx_t;
    } else {
        (*mc)
            .mc_ki[(*mc).mc_top
            as usize] = ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_int
            - 1 as libc::c_int) as indx_t;
    }
    if !((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_flags as libc::c_int & 1 as libc::c_int
        == 1 as libc::c_int)
    {
        mdb_assert_fail(
            (*(*mc).mc_txn).mt_env,
            b"IS_BRANCH(mc->mc_pg[mc->mc_top])\0" as *const u8 as *const libc::c_char,
            b"mdb_cursor_sibling\0" as *const u8 as *const libc::c_char,
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            6857 as libc::c_int,
        );
    }
    indx = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
        .offset(
            *((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_ptrs)
                .as_mut_ptr()
                .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize) as libc::c_int
                as isize,
        )
        .offset(0 as libc::c_uint as isize) as *mut MDB_node;
    rc = mdb_page_get(
        mc,
        (*indx).mn_lo as libc::c_ulong | ((*indx).mn_hi as pgno_t) << 16 as libc::c_int
            | ((*indx).mn_flags as pgno_t) << 32 as libc::c_int,
        &mut mp,
        0 as *mut libc::c_void as *mut libc::c_int,
    );
    if rc != 0 as libc::c_int {
        (*mc).mc_flags &= 4294967292 as libc::c_uint;
        return rc;
    }
    mdb_cursor_push(mc, mp);
    if move_right == 0 {
        (*mc)
            .mc_ki[(*mc).mc_top
            as usize] = (((*mp).mp_pb.pb.pb_lower as libc::c_uint)
            .wrapping_sub(
                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                    as libc::c_uint,
            ) >> 1 as libc::c_int)
            .wrapping_sub(1 as libc::c_uint) as indx_t;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_cursor_next(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut op: MDB_cursor_op,
) -> libc::c_int {
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut leaf: *mut MDB_node = 0 as *mut MDB_node;
    let mut rc: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    if (*mc).mc_flags & 8 as libc::c_uint != 0 {
        if op as libc::c_uint == 9 as libc::c_uint {
            return -(30798 as libc::c_int);
        }
    }
    if (*mc).mc_flags & 1 as libc::c_uint == 0 {
        tmp = mdb_cursor_first(mc, key, data);
        return tmp;
    }
    mp = (*mc).mc_pg[(*mc).mc_top as usize];
    if (*mc).mc_flags & 2 as libc::c_uint != 0 {
        if (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint
            >= (((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                .wrapping_sub(
                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                        as libc::c_ulong as libc::c_uint,
                ) >> 1 as libc::c_int)
                .wrapping_sub(1 as libc::c_uint)
        {
            return -(30798 as libc::c_int);
        }
        (*mc).mc_flags ^= 2 as libc::c_uint;
    }
    if (*(*mc).mc_db).md_flags as libc::c_int & 4 as libc::c_int != 0 {
        leaf = (mp as *mut libc::c_char)
            .offset(
                *((*mp).mp_ptrs)
                    .as_mut_ptr()
                    .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize) as libc::c_int
                    as isize,
            )
            .offset(0 as libc::c_uint as isize) as *mut MDB_node;
        if (*leaf).mn_flags as libc::c_int & 4 as libc::c_int == 4 as libc::c_int {
            let mut current_block_25: u64;
            if op as libc::c_uint == 8 as libc::c_uint {
                current_block_25 = 6476829792258022247;
            } else if op as libc::c_uint == 9 as libc::c_uint {
                current_block_25 = 6476829792258022247;
            } else {
                current_block_25 = 14763689060501151050;
            }
            match current_block_25 {
                6476829792258022247 => {
                    rc = mdb_cursor_next(
                        &mut (*(*mc).mc_xcursor).mx_cursor,
                        data,
                        0 as *mut libc::c_void as *mut MDB_val,
                        MDB_NEXT,
                    );
                    's_132: {
                        if !(op as libc::c_uint != 8 as libc::c_uint) {
                            if !(rc != -(30798 as libc::c_int)) {
                                break 's_132;
                            }
                        }
                        if rc == 0 as libc::c_int {
                            if key as libc::c_ulong
                                != 0 as *mut libc::c_void as libc::c_ulong
                            {
                                (*key).mv_size = (*leaf).mn_ksize as size_t;
                                (*key)
                                    .mv_data = ((*leaf).mn_data).as_mut_ptr()
                                    as *mut libc::c_void;
                            }
                        }
                        return rc;
                    }
                }
                _ => {}
            }
        } else {
            (*(*mc).mc_xcursor).mx_cursor.mc_flags &= 4294967292 as libc::c_uint;
            if op as libc::c_uint == 9 as libc::c_uint {
                return -(30798 as libc::c_int);
            }
        }
    }
    if (*mc).mc_flags & 8 as libc::c_uint != 0 {
        (*mc).mc_flags ^= 8 as libc::c_uint;
    } else if ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint)
            .wrapping_add(1 as libc::c_uint)
            >= ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                .wrapping_sub(
                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                        as libc::c_ulong as libc::c_uint,
                ) >> 1 as libc::c_int
        {
        rc = mdb_cursor_sibling(mc, 1 as libc::c_int);
        if rc != 0 as libc::c_int {
            (*mc).mc_flags |= 2 as libc::c_uint;
            return rc;
        }
        mp = (*mc).mc_pg[(*mc).mc_top as usize];
    } else {
        (*mc)
            .mc_ki[(*mc).mc_top
            as usize] = ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_int
            + 1 as libc::c_int) as indx_t;
    }
    if (*mp).mp_flags as libc::c_int & 32 as libc::c_int == 32 as libc::c_int {
        (*key).mv_size = (*(*mc).mc_db).md_pad as size_t;
        (*key)
            .mv_data = (mp as *mut libc::c_char)
            .offset(
                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                    as libc::c_uint as isize,
            )
            .offset(
                ((*mc).mc_ki[(*mc).mc_top as usize] as size_t)
                    .wrapping_mul((*key).mv_size) as isize,
            ) as *mut libc::c_void;
        return 0 as libc::c_int;
    }
    if !((*mp).mp_flags as libc::c_int & 2 as libc::c_int == 2 as libc::c_int) {
        mdb_assert_fail(
            (*(*mc).mc_txn).mt_env,
            b"IS_LEAF(mp)\0" as *const u8 as *const libc::c_char,
            b"mdb_cursor_next\0" as *const u8 as *const libc::c_char,
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            6946 as libc::c_int,
        );
    }
    leaf = (mp as *mut libc::c_char)
        .offset(
            *((*mp).mp_ptrs)
                .as_mut_ptr()
                .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize) as libc::c_int
                as isize,
        )
        .offset(0 as libc::c_uint as isize) as *mut MDB_node;
    if (*leaf).mn_flags as libc::c_int & 4 as libc::c_int == 4 as libc::c_int {
        mdb_xcursor_init1(mc, leaf);
        rc = mdb_cursor_first(
            &mut (*(*mc).mc_xcursor).mx_cursor,
            data,
            0 as *mut libc::c_void as *mut MDB_val,
        );
        if rc != 0 as libc::c_int {
            return rc;
        }
    } else if !data.is_null() {
        rc = mdb_node_read(mc, leaf, data);
        if rc != 0 as libc::c_int {
            return rc;
        }
    }
    if key as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        (*key).mv_size = (*leaf).mn_ksize as size_t;
        (*key).mv_data = ((*leaf).mn_data).as_mut_ptr() as *mut libc::c_void;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_cursor_prev(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut op: MDB_cursor_op,
) -> libc::c_int {
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut leaf: *mut MDB_node = 0 as *mut MDB_node;
    let mut rc: libc::c_int = 0;
    if (*mc).mc_flags & 1 as libc::c_uint == 0 {
        rc = mdb_cursor_last(mc, key, data);
        if rc != 0 {
            return rc;
        }
        (*mc)
            .mc_ki[(*mc).mc_top
            as usize] = ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_int
            + 1 as libc::c_int) as indx_t;
    }
    mp = (*mc).mc_pg[(*mc).mc_top as usize];
    if (*(*mc).mc_db).md_flags as libc::c_int & 4 as libc::c_int != 0 {
        if ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint)
            < ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                .wrapping_sub(
                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                        as libc::c_ulong as libc::c_uint,
                ) >> 1 as libc::c_int
        {
            leaf = (mp as *mut libc::c_char)
                .offset(
                    *((*mp).mp_ptrs)
                        .as_mut_ptr()
                        .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                        as libc::c_int as isize,
                )
                .offset(0 as libc::c_uint as isize) as *mut MDB_node;
            if (*leaf).mn_flags as libc::c_int & 4 as libc::c_int == 4 as libc::c_int {
                let mut current_block_18: u64;
                if op as libc::c_uint == 12 as libc::c_uint {
                    current_block_18 = 9721623463541018151;
                } else if op as libc::c_uint == 13 as libc::c_uint {
                    current_block_18 = 9721623463541018151;
                } else {
                    current_block_18 = 18317007320854588510;
                }
                match current_block_18 {
                    9721623463541018151 => {
                        rc = mdb_cursor_prev(
                            &mut (*(*mc).mc_xcursor).mx_cursor,
                            data,
                            0 as *mut libc::c_void as *mut MDB_val,
                            MDB_PREV,
                        );
                        's_107: {
                            if !(op as libc::c_uint != 12 as libc::c_uint) {
                                if !(rc != -(30798 as libc::c_int)) {
                                    break 's_107;
                                }
                            }
                            if rc == 0 as libc::c_int {
                                if key as libc::c_ulong
                                    != 0 as *mut libc::c_void as libc::c_ulong
                                {
                                    (*key).mv_size = (*leaf).mn_ksize as size_t;
                                    (*key)
                                        .mv_data = ((*leaf).mn_data).as_mut_ptr()
                                        as *mut libc::c_void;
                                }
                                (*mc).mc_flags &= 4294967293 as libc::c_uint;
                            }
                            return rc;
                        }
                    }
                    _ => {}
                }
            } else {
                (*(*mc).mc_xcursor).mx_cursor.mc_flags &= 4294967292 as libc::c_uint;
                if op as libc::c_uint == 13 as libc::c_uint {
                    return -(30798 as libc::c_int);
                }
            }
        }
    }
    (*mc).mc_flags &= 4294967285 as libc::c_uint;
    if (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_int == 0 as libc::c_int {
        rc = mdb_cursor_sibling(mc, 0 as libc::c_int);
        if rc != 0 as libc::c_int {
            return rc;
        }
        mp = (*mc).mc_pg[(*mc).mc_top as usize];
        (*mc)
            .mc_ki[(*mc).mc_top
            as usize] = (((*mp).mp_pb.pb.pb_lower as libc::c_uint)
            .wrapping_sub(
                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                    as libc::c_uint,
            ) >> 1 as libc::c_int)
            .wrapping_sub(1 as libc::c_uint) as indx_t;
    } else {
        (*mc)
            .mc_ki[(*mc).mc_top
            as usize] = ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_int
            - 1 as libc::c_int) as indx_t;
    }
    if !((*mp).mp_flags as libc::c_int & 2 as libc::c_int == 2 as libc::c_int) {
        return -(30796 as libc::c_int);
    }
    if (*mp).mp_flags as libc::c_int & 32 as libc::c_int == 32 as libc::c_int {
        (*key).mv_size = (*(*mc).mc_db).md_pad as size_t;
        (*key)
            .mv_data = (mp as *mut libc::c_char)
            .offset(
                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                    as libc::c_uint as isize,
            )
            .offset(
                ((*mc).mc_ki[(*mc).mc_top as usize] as size_t)
                    .wrapping_mul((*key).mv_size) as isize,
            ) as *mut libc::c_void;
        return 0 as libc::c_int;
    }
    leaf = (mp as *mut libc::c_char)
        .offset(
            *((*mp).mp_ptrs)
                .as_mut_ptr()
                .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize) as libc::c_int
                as isize,
        )
        .offset(0 as libc::c_uint as isize) as *mut MDB_node;
    if (*leaf).mn_flags as libc::c_int & 4 as libc::c_int == 4 as libc::c_int {
        mdb_xcursor_init1(mc, leaf);
        rc = mdb_cursor_last(
            &mut (*(*mc).mc_xcursor).mx_cursor,
            data,
            0 as *mut libc::c_void as *mut MDB_val,
        );
        if rc != 0 as libc::c_int {
            return rc;
        }
    } else if !data.is_null() {
        rc = mdb_node_read(mc, leaf, data);
        if rc != 0 as libc::c_int {
            return rc;
        }
    }
    if key as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        (*key).mv_size = (*leaf).mn_ksize as size_t;
        (*key).mv_data = ((*leaf).mn_data).as_mut_ptr() as *mut libc::c_void;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_cursor_set(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut op: MDB_cursor_op,
    mut exactp: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut rc: libc::c_int = 0;
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut leaf: *mut MDB_node = 0 as *mut MDB_node;
    let mut nodekey: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut i: libc::c_uint = 0;
    let mut nkeys: libc::c_uint = 0;
    let mut ex2: libc::c_int = 0;
    let mut ex2p: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut olddata: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut dcmp: Option::<MDB_cmp_func> = None;
    leaf = 0 as *mut libc::c_void as *mut MDB_node;
    if (*key).mv_size == 0 as libc::c_ulong {
        return -(30781 as libc::c_int);
    }
    if !((*mc).mc_xcursor).is_null() {
        (*(*mc).mc_xcursor).mx_cursor.mc_flags &= 4294967292 as libc::c_uint;
    }
    if (*mc).mc_flags & 1 as libc::c_uint != 0 {
        mp = (*mc).mc_pg[(*mc).mc_top as usize];
        if ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
            .wrapping_sub(
                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                    as libc::c_uint,
            ) >> 1 as libc::c_int == 0
        {
            (*mc).mc_ki[(*mc).mc_top as usize] = 0 as libc::c_int as indx_t;
            return -(30798 as libc::c_int);
        }
        if (*mp).mp_flags as libc::c_int & 32 as libc::c_int != 0 {
            nodekey.mv_size = (*(*mc).mc_db).md_pad as size_t;
            nodekey
                .mv_data = (mp as *mut libc::c_char)
                .offset(
                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                        as libc::c_ulong as libc::c_uint as isize,
                )
                .offset(0 as libc::c_ulong as isize) as *mut libc::c_void;
        } else {
            leaf = (mp as *mut libc::c_char)
                .offset(
                    *((*mp).mp_ptrs).as_mut_ptr().offset(0 as libc::c_int as isize)
                        as libc::c_int as isize,
                )
                .offset(0 as libc::c_uint as isize) as *mut MDB_node;
            nodekey.mv_size = (*leaf).mn_ksize as size_t;
            nodekey.mv_data = ((*leaf).mn_data).as_mut_ptr() as *mut libc::c_void;
        }
        rc = (Some(((*(*mc).mc_dbx).md_cmp).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(key as *const MDB_val, &mut nodekey as *mut MDB_val as *const MDB_val);
        if rc == 0 as libc::c_int {
            (*mc).mc_ki[(*mc).mc_top as usize] = 0 as libc::c_int as indx_t;
            if !exactp.is_null() {
                *exactp = 1 as libc::c_int;
            }
            current_block = 809883266258122424;
        } else {
            if rc > 0 as libc::c_int {
                nkeys = ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                    .wrapping_sub(
                        &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                            as libc::c_ulong as libc::c_uint,
                    ) >> 1 as libc::c_int;
                if nkeys > 1 as libc::c_uint {
                    if (*mp).mp_flags as libc::c_int & 32 as libc::c_int != 0 {
                        nodekey
                            .mv_data = (mp as *mut libc::c_char)
                            .offset(
                                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                    as libc::c_ulong as libc::c_uint as isize,
                            )
                            .offset(
                                (nkeys.wrapping_sub(1 as libc::c_uint) as size_t)
                                    .wrapping_mul(nodekey.mv_size) as isize,
                            ) as *mut libc::c_void;
                    } else {
                        leaf = (mp as *mut libc::c_char)
                            .offset(
                                *((*mp).mp_ptrs)
                                    .as_mut_ptr()
                                    .offset(nkeys.wrapping_sub(1 as libc::c_uint) as isize)
                                    as libc::c_int as isize,
                            )
                            .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                        nodekey.mv_size = (*leaf).mn_ksize as size_t;
                        nodekey
                            .mv_data = ((*leaf).mn_data).as_mut_ptr()
                            as *mut libc::c_void;
                    }
                    rc = (Some(
                        ((*(*mc).mc_dbx).md_cmp).expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        key as *const MDB_val,
                        &mut nodekey as *mut MDB_val as *const MDB_val,
                    );
                    if rc == 0 as libc::c_int {
                        (*mc)
                            .mc_ki[(*mc).mc_top
                            as usize] = nkeys.wrapping_sub(1 as libc::c_uint) as indx_t;
                        if !exactp.is_null() {
                            *exactp = 1 as libc::c_int;
                        }
                        current_block = 809883266258122424;
                    } else if rc < 0 as libc::c_int {
                        if ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint)
                            < ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                                .wrapping_sub(
                                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                        as libc::c_ulong as libc::c_uint,
                                ) >> 1 as libc::c_int
                        {
                            if (*mp).mp_flags as libc::c_int & 32 as libc::c_int != 0 {
                                nodekey
                                    .mv_data = (mp as *mut libc::c_char)
                                    .offset(
                                        &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                            as libc::c_ulong as libc::c_uint as isize,
                                    )
                                    .offset(
                                        ((*mc).mc_ki[(*mc).mc_top as usize] as size_t)
                                            .wrapping_mul(nodekey.mv_size) as isize,
                                    ) as *mut libc::c_void;
                            } else {
                                leaf = (mp as *mut libc::c_char)
                                    .offset(
                                        *((*mp).mp_ptrs)
                                            .as_mut_ptr()
                                            .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                                            as libc::c_int as isize,
                                    )
                                    .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                                nodekey.mv_size = (*leaf).mn_ksize as size_t;
                                nodekey
                                    .mv_data = ((*leaf).mn_data).as_mut_ptr()
                                    as *mut libc::c_void;
                            }
                            rc = (Some(
                                ((*(*mc).mc_dbx).md_cmp).expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                key as *const MDB_val,
                                &mut nodekey as *mut MDB_val as *const MDB_val,
                            );
                            if rc == 0 as libc::c_int {
                                if !exactp.is_null() {
                                    *exactp = 1 as libc::c_int;
                                }
                                current_block = 809883266258122424;
                            } else {
                                current_block = 14775119014532381840;
                            }
                        } else {
                            current_block = 14775119014532381840;
                        }
                        match current_block {
                            809883266258122424 => {}
                            _ => {
                                rc = 0 as libc::c_int;
                                (*mc).mc_flags &= 4294967293 as libc::c_uint;
                                current_block = 16888758775459114693;
                            }
                        }
                    } else {
                        current_block = 1423531122933789233;
                    }
                } else {
                    current_block = 1423531122933789233;
                }
                match current_block {
                    809883266258122424 => {}
                    16888758775459114693 => {}
                    _ => {
                        i = 0 as libc::c_uint;
                        while i < (*mc).mc_top as libc::c_uint {
                            if ((*mc).mc_ki[i as usize] as libc::c_uint)
                                < (((*(*mc).mc_pg[i as usize]).mp_pb.pb.pb_lower
                                    as libc::c_uint)
                                    .wrapping_sub(
                                        &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                            as libc::c_ulong as libc::c_uint,
                                    ) >> 1 as libc::c_int)
                                    .wrapping_sub(1 as libc::c_uint)
                            {
                                break;
                            }
                            i = i.wrapping_add(1);
                        }
                        if i == (*mc).mc_top as libc::c_uint {
                            (*mc).mc_ki[(*mc).mc_top as usize] = nkeys as indx_t;
                            return -(30798 as libc::c_int);
                        }
                        current_block = 5159818223158340697;
                    }
                }
            } else {
                current_block = 5159818223158340697;
            }
            match current_block {
                809883266258122424 => {}
                16888758775459114693 => {}
                _ => {
                    if (*mc).mc_top == 0 {
                        (*mc).mc_ki[(*mc).mc_top as usize] = 0 as libc::c_int as indx_t;
                        if op as libc::c_uint == 17 as libc::c_uint {
                            if exactp.is_null() {
                                rc = 0 as libc::c_int;
                            } else {
                                return -(30798 as libc::c_int)
                            }
                        } else {
                            return -(30798 as libc::c_int)
                        }
                        current_block = 809883266258122424;
                    } else {
                        current_block = 16779030619667747692;
                    }
                }
            }
        }
    } else {
        (*mc).mc_pg[0 as libc::c_int as usize] = 0 as *mut MDB_page;
        current_block = 16779030619667747692;
    }
    match current_block {
        16779030619667747692 => {
            rc = mdb_page_search(mc, key, 0 as libc::c_int);
            if rc != 0 as libc::c_int {
                return rc;
            }
            mp = (*mc).mc_pg[(*mc).mc_top as usize];
            if !((*mp).mp_flags as libc::c_int & 2 as libc::c_int == 2 as libc::c_int) {
                mdb_assert_fail(
                    (*(*mc).mc_txn).mt_env,
                    b"IS_LEAF(mp)\0" as *const u8 as *const libc::c_char,
                    b"mdb_cursor_set\0" as *const u8 as *const libc::c_char,
                    b"mdb.c\0" as *const u8 as *const libc::c_char,
                    7165 as libc::c_int,
                );
            }
            current_block = 16888758775459114693;
        }
        _ => {}
    }
    match current_block {
        16888758775459114693 => {
            leaf = mdb_node_search(mc, key, exactp);
            if exactp as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                if *exactp == 0 {
                    return -(30798 as libc::c_int);
                }
            }
            if leaf as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                rc = mdb_cursor_sibling(mc, 1 as libc::c_int);
                if rc != 0 as libc::c_int {
                    (*mc).mc_flags |= 2 as libc::c_uint;
                    return rc;
                }
                mp = (*mc).mc_pg[(*mc).mc_top as usize];
                if !((*mp).mp_flags as libc::c_int & 2 as libc::c_int
                    == 2 as libc::c_int)
                {
                    mdb_assert_fail(
                        (*(*mc).mc_txn).mt_env,
                        b"IS_LEAF(mp)\0" as *const u8 as *const libc::c_char,
                        b"mdb_cursor_set\0" as *const u8 as *const libc::c_char,
                        b"mdb.c\0" as *const u8 as *const libc::c_char,
                        7181 as libc::c_int,
                    );
                }
                leaf = (mp as *mut libc::c_char)
                    .offset(
                        *((*mp).mp_ptrs).as_mut_ptr().offset(0 as libc::c_int as isize)
                            as libc::c_int as isize,
                    )
                    .offset(0 as libc::c_uint as isize) as *mut MDB_node;
            }
        }
        _ => {}
    }
    (*mc).mc_flags |= 1 as libc::c_uint;
    (*mc).mc_flags &= 4294967293 as libc::c_uint;
    if (*mp).mp_flags as libc::c_int & 32 as libc::c_int == 32 as libc::c_int {
        if op as libc::c_uint == 17 as libc::c_uint {
            (*key).mv_size = (*(*mc).mc_db).md_pad as size_t;
            (*key)
                .mv_data = (mp as *mut libc::c_char)
                .offset(
                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                        as libc::c_ulong as libc::c_uint as isize,
                )
                .offset(
                    ((*mc).mc_ki[(*mc).mc_top as usize] as size_t)
                        .wrapping_mul((*key).mv_size) as isize,
                ) as *mut libc::c_void;
        } else if op as libc::c_uint == 16 as libc::c_uint {
            (*key).mv_size = (*(*mc).mc_db).md_pad as size_t;
            (*key)
                .mv_data = (mp as *mut libc::c_char)
                .offset(
                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                        as libc::c_ulong as libc::c_uint as isize,
                )
                .offset(
                    ((*mc).mc_ki[(*mc).mc_top as usize] as size_t)
                        .wrapping_mul((*key).mv_size) as isize,
                ) as *mut libc::c_void;
        }
        return 0 as libc::c_int;
    }
    if (*leaf).mn_flags as libc::c_int & 4 as libc::c_int == 4 as libc::c_int {
        mdb_xcursor_init1(mc, leaf);
        if op as libc::c_uint == 15 as libc::c_uint {
            rc = mdb_cursor_first(
                &mut (*(*mc).mc_xcursor).mx_cursor,
                data,
                0 as *mut libc::c_void as *mut MDB_val,
            );
        } else if op as libc::c_uint == 16 as libc::c_uint {
            rc = mdb_cursor_first(
                &mut (*(*mc).mc_xcursor).mx_cursor,
                data,
                0 as *mut libc::c_void as *mut MDB_val,
            );
        } else if op as libc::c_uint == 17 as libc::c_uint {
            rc = mdb_cursor_first(
                &mut (*(*mc).mc_xcursor).mx_cursor,
                data,
                0 as *mut libc::c_void as *mut MDB_val,
            );
        } else {
            if op as libc::c_uint == 2 as libc::c_uint {
                ex2p = &mut ex2;
                ex2 = 0 as libc::c_int;
            } else {
                ex2p = 0 as *mut libc::c_void as *mut libc::c_int;
            }
            rc = mdb_cursor_set(
                &mut (*(*mc).mc_xcursor).mx_cursor,
                data,
                0 as *mut libc::c_void as *mut MDB_val,
                MDB_SET_RANGE,
                ex2p,
            );
            if rc != 0 as libc::c_int {
                return rc;
            }
        }
    } else if !data.is_null() {
        let mut current_block_155: u64;
        if op as libc::c_uint == 2 as libc::c_uint {
            current_block_155 = 2560320692558747116;
        } else if op as libc::c_uint == 3 as libc::c_uint {
            current_block_155 = 2560320692558747116;
        } else {
            if !((*mc).mc_xcursor).is_null() {
                (*(*mc).mc_xcursor).mx_cursor.mc_flags &= 4294967292 as libc::c_uint;
            }
            rc = mdb_node_read(mc, leaf, data);
            if rc != 0 as libc::c_int {
                return rc;
            }
            current_block_155 = 15321816652064063775;
        }
        match current_block_155 {
            2560320692558747116 => {
                rc = mdb_node_read(mc, leaf, &mut olddata);
                if rc != 0 as libc::c_int {
                    return rc;
                }
                dcmp = (*(*mc).mc_dbx).md_dcmp;
                if ::std::mem::transmute::<Option::<MDB_cmp_func>, libc::c_ulong>(dcmp)
                    == ::std::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(
                                *const MDB_val,
                                *const MDB_val,
                            ) -> libc::c_int,
                        >,
                        libc::c_ulong,
                    >(
                        Some(
                            mdb_cmp_int
                                as unsafe extern "C" fn(
                                    *const MDB_val,
                                    *const MDB_val,
                                ) -> libc::c_int,
                        ),
                    )
                {
                    if olddata.mv_size
                        == ::std::mem::size_of::<mdb_size_t>() as libc::c_ulong
                    {
                        dcmp = Some(
                            mdb_cmp_long
                                as unsafe extern "C" fn(
                                    *const MDB_val,
                                    *const MDB_val,
                                ) -> libc::c_int,
                        );
                    }
                }
                rc = (Some(dcmp.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    data as *const MDB_val,
                    &mut olddata as *mut MDB_val as *const MDB_val,
                );
                if rc != 0 {
                    if op as libc::c_uint == 2 as libc::c_uint {
                        return -(30798 as libc::c_int)
                    } else {
                        if rc > 0 as libc::c_int {
                            return -(30798 as libc::c_int);
                        }
                    }
                    rc = 0 as libc::c_int;
                }
                *data = olddata;
            }
            _ => {}
        }
    }
    let mut current_block_163: u64;
    if op as libc::c_uint == 17 as libc::c_uint {
        current_block_163 = 12314582351725639221;
    } else if op as libc::c_uint == 16 as libc::c_uint {
        current_block_163 = 12314582351725639221;
    } else {
        current_block_163 = 7627602990488000394;
    }
    match current_block_163 {
        12314582351725639221 => {
            if key as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                (*key).mv_size = (*leaf).mn_ksize as size_t;
                (*key).mv_data = ((*leaf).mn_data).as_mut_ptr() as *mut libc::c_void;
            }
        }
        _ => {}
    }
    return rc;
}
unsafe extern "C" fn mdb_cursor_first(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut leaf: *mut MDB_node = 0 as *mut MDB_node;
    if !((*mc).mc_xcursor).is_null() {
        (*(*mc).mc_xcursor).mx_cursor.mc_flags &= 4294967292 as libc::c_uint;
    }
    let mut current_block_6: u64;
    if (*mc).mc_flags & 1 as libc::c_uint == 0 {
        current_block_6 = 16270957535625076812;
    } else if (*mc).mc_top != 0 {
        current_block_6 = 16270957535625076812;
    } else {
        current_block_6 = 5720623009719927633;
    }
    match current_block_6 {
        16270957535625076812 => {
            rc = mdb_page_search(
                mc,
                0 as *mut libc::c_void as *mut MDB_val,
                4 as libc::c_int,
            );
            if rc != 0 as libc::c_int {
                return rc;
            }
        }
        _ => {}
    }
    if !((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_flags as libc::c_int & 2 as libc::c_int
        == 2 as libc::c_int)
    {
        mdb_assert_fail(
            (*(*mc).mc_txn).mt_env,
            b"IS_LEAF(mc->mc_pg[mc->mc_top])\0" as *const u8 as *const libc::c_char,
            b"mdb_cursor_first\0" as *const u8 as *const libc::c_char,
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            7263 as libc::c_int,
        );
    }
    leaf = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
        .offset(
            *((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_ptrs)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as libc::c_int as isize,
        )
        .offset(0 as libc::c_uint as isize) as *mut MDB_node;
    (*mc).mc_flags |= 1 as libc::c_uint;
    (*mc).mc_flags &= 4294967293 as libc::c_uint;
    (*mc).mc_ki[(*mc).mc_top as usize] = 0 as libc::c_int as indx_t;
    if (*(*mc).mc_pg[(*mc).mc_top as usize]).mp_flags as libc::c_int & 32 as libc::c_int
        == 32 as libc::c_int
    {
        if !key.is_null() {
            (*key).mv_size = (*(*mc).mc_db).md_pad as size_t;
            (*key)
                .mv_data = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
                .offset(
                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                        as libc::c_ulong as libc::c_uint as isize,
                )
                .offset(0 as libc::c_ulong as isize) as *mut libc::c_void;
        }
        return 0 as libc::c_int;
    }
    if (*leaf).mn_flags as libc::c_int & 4 as libc::c_int == 4 as libc::c_int {
        mdb_xcursor_init1(mc, leaf);
        rc = mdb_cursor_first(
            &mut (*(*mc).mc_xcursor).mx_cursor,
            data,
            0 as *mut libc::c_void as *mut MDB_val,
        );
        if rc != 0 {
            return rc;
        }
    } else if !data.is_null() {
        rc = mdb_node_read(mc, leaf, data);
        if rc != 0 as libc::c_int {
            return rc;
        }
    }
    if key as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        (*key).mv_size = (*leaf).mn_ksize as size_t;
        (*key).mv_data = ((*leaf).mn_data).as_mut_ptr() as *mut libc::c_void;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_cursor_last(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut leaf: *mut MDB_node = 0 as *mut MDB_node;
    if !((*mc).mc_xcursor).is_null() {
        (*(*mc).mc_xcursor).mx_cursor.mc_flags &= 4294967292 as libc::c_uint;
    }
    let mut current_block_6: u64;
    if (*mc).mc_flags & 1 as libc::c_uint == 0 {
        current_block_6 = 15828230041459502670;
    } else if (*mc).mc_top != 0 {
        current_block_6 = 15828230041459502670;
    } else {
        current_block_6 = 5720623009719927633;
    }
    match current_block_6 {
        15828230041459502670 => {
            rc = mdb_page_search(
                mc,
                0 as *mut libc::c_void as *mut MDB_val,
                8 as libc::c_int,
            );
            if rc != 0 as libc::c_int {
                return rc;
            }
        }
        _ => {}
    }
    if !((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_flags as libc::c_int & 2 as libc::c_int
        == 2 as libc::c_int)
    {
        mdb_assert_fail(
            (*(*mc).mc_txn).mt_env,
            b"IS_LEAF(mc->mc_pg[mc->mc_top])\0" as *const u8 as *const libc::c_char,
            b"mdb_cursor_last\0" as *const u8 as *const libc::c_char,
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            7310 as libc::c_int,
        );
    }
    (*mc)
        .mc_ki[(*mc).mc_top
        as usize] = (((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_pb.pb.pb_lower
        as libc::c_uint)
        .wrapping_sub(
            &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                as libc::c_uint,
        ) >> 1 as libc::c_int)
        .wrapping_sub(1 as libc::c_uint) as indx_t;
    (*mc).mc_flags |= 3 as libc::c_uint;
    leaf = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
        .offset(
            *((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_ptrs)
                .as_mut_ptr()
                .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize) as libc::c_int
                as isize,
        )
        .offset(0 as libc::c_uint as isize) as *mut MDB_node;
    if (*(*mc).mc_pg[(*mc).mc_top as usize]).mp_flags as libc::c_int & 32 as libc::c_int
        == 32 as libc::c_int
    {
        if !key.is_null() {
            (*key).mv_size = (*(*mc).mc_db).md_pad as size_t;
            (*key)
                .mv_data = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
                .offset(
                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                        as libc::c_ulong as libc::c_uint as isize,
                )
                .offset(
                    ((*mc).mc_ki[(*mc).mc_top as usize] as size_t)
                        .wrapping_mul((*key).mv_size) as isize,
                ) as *mut libc::c_void;
        }
        return 0 as libc::c_int;
    }
    if (*leaf).mn_flags as libc::c_int & 4 as libc::c_int == 4 as libc::c_int {
        mdb_xcursor_init1(mc, leaf);
        rc = mdb_cursor_last(
            &mut (*(*mc).mc_xcursor).mx_cursor,
            data,
            0 as *mut libc::c_void as *mut MDB_val,
        );
        if rc != 0 {
            return rc;
        }
    } else if !data.is_null() {
        rc = mdb_node_read(mc, leaf, data);
        if rc != 0 as libc::c_int {
            return rc;
        }
    }
    if key as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        (*key).mv_size = (*leaf).mn_ksize as size_t;
        (*key).mv_data = ((*leaf).mn_data).as_mut_ptr() as *mut libc::c_void;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_cursor_get(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut op: MDB_cursor_op,
) -> libc::c_int {
    let mut current_block: u64;
    let mut rc: libc::c_int = 0;
    let mut exact: libc::c_int = 0;
    let mut mfunc: Option::<
        unsafe extern "C" fn(*mut MDB_cursor, *mut MDB_val, *mut MDB_val) -> libc::c_int,
    > = None;
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut nkeys: libc::c_int = 0;
    let mut leaf: *mut MDB_node = 0 as *mut MDB_node;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut mx: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut mx___0: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut leaf___0: *mut MDB_node = 0 as *mut MDB_node;
    exact = 0 as libc::c_int;
    if mc as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 22 as libc::c_int;
    }
    if (*(*mc).mc_txn).mt_flags & 19 as libc::c_uint != 0 {
        return -(30782 as libc::c_int);
    }
    match op as libc::c_uint {
        4 => {
            if (*mc).mc_flags & 1 as libc::c_uint == 0 {
                rc = 22 as libc::c_int;
            } else {
                mp = (*mc).mc_pg[(*mc).mc_top as usize];
                nkeys = (((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                    .wrapping_sub(
                        &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                            as libc::c_ulong as libc::c_uint,
                    ) >> 1 as libc::c_int) as libc::c_int;
                if nkeys == 0 {
                    (*mc).mc_ki[(*mc).mc_top as usize] = nkeys as indx_t;
                    rc = -(30798 as libc::c_int);
                } else if (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_int >= nkeys {
                    (*mc).mc_ki[(*mc).mc_top as usize] = nkeys as indx_t;
                    rc = -(30798 as libc::c_int);
                } else {
                    rc = 0 as libc::c_int;
                    if (*mp).mp_flags as libc::c_int & 32 as libc::c_int
                        == 32 as libc::c_int
                    {
                        (*key).mv_size = (*(*mc).mc_db).md_pad as size_t;
                        (*key)
                            .mv_data = (mp as *mut libc::c_char)
                            .offset(
                                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                    as libc::c_ulong as libc::c_uint as isize,
                            )
                            .offset(
                                ((*mc).mc_ki[(*mc).mc_top as usize] as size_t)
                                    .wrapping_mul((*key).mv_size) as isize,
                            ) as *mut libc::c_void;
                    } else {
                        leaf = (mp as *mut libc::c_char)
                            .offset(
                                *((*mp).mp_ptrs)
                                    .as_mut_ptr()
                                    .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                                    as libc::c_int as isize,
                            )
                            .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                        if key as libc::c_ulong
                            != 0 as *mut libc::c_void as libc::c_ulong
                        {
                            (*key).mv_size = (*leaf).mn_ksize as size_t;
                            (*key)
                                .mv_data = ((*leaf).mn_data).as_mut_ptr()
                                as *mut libc::c_void;
                        }
                        if !data.is_null() {
                            if (*leaf).mn_flags as libc::c_int & 4 as libc::c_int
                                == 4 as libc::c_int
                            {
                                rc = mdb_cursor_get(
                                    &mut (*(*mc).mc_xcursor).mx_cursor,
                                    data,
                                    0 as *mut libc::c_void as *mut MDB_val,
                                    MDB_GET_CURRENT,
                                );
                            } else {
                                rc = mdb_node_read(mc, leaf, data);
                            }
                        }
                    }
                }
            }
            current_block = 10041771570435381152;
        }
        3 | 2 => {
            if data as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                rc = 22 as libc::c_int;
                current_block = 10041771570435381152;
            } else if (*mc).mc_xcursor as libc::c_ulong
                    == 0 as *mut libc::c_void as libc::c_ulong
                {
                rc = -(30784 as libc::c_int);
                current_block = 10041771570435381152;
            } else {
                current_block = 11404259345410850769;
            }
        }
        17 | 16 | 15 => {
            current_block = 11404259345410850769;
        }
        5 => {
            if data as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                rc = 22 as libc::c_int;
                current_block = 10041771570435381152;
            } else if (*mc).mc_flags & 1 as libc::c_uint == 0 {
                rc = 22 as libc::c_int;
                current_block = 10041771570435381152;
            } else if (*(*mc).mc_db).md_flags as libc::c_int & 16 as libc::c_int == 0 {
                rc = -(30784 as libc::c_int);
                current_block = 10041771570435381152;
            } else {
                rc = 0 as libc::c_int;
                if (*(*mc).mc_xcursor).mx_cursor.mc_flags & 1 as libc::c_uint == 0 {
                    current_block = 10041771570435381152;
                } else if (*(*mc).mc_xcursor).mx_cursor.mc_flags & 2 as libc::c_uint != 0
                    {
                    current_block = 10041771570435381152;
                } else {
                    current_block = 1416643152622400344;
                }
            }
        }
        10 => {
            if data as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                rc = 22 as libc::c_int;
                current_block = 10041771570435381152;
            } else if (*(*mc).mc_db).md_flags as libc::c_int & 16 as libc::c_int == 0 {
                rc = -(30784 as libc::c_int);
                current_block = 10041771570435381152;
            } else {
                rc = mdb_cursor_next(mc, key, data, MDB_NEXT_DUP);
                if rc == 0 as libc::c_int {
                    if (*(*mc).mc_xcursor).mx_cursor.mc_flags & 1 as libc::c_uint != 0 {
                        current_block = 1416643152622400344;
                    } else {
                        rc = -(30798 as libc::c_int);
                        current_block = 10041771570435381152;
                    }
                } else {
                    current_block = 10041771570435381152;
                }
            }
        }
        18 => {
            if data as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                rc = 22 as libc::c_int;
                current_block = 10041771570435381152;
            } else if (*(*mc).mc_db).md_flags as libc::c_int & 16 as libc::c_int == 0 {
                rc = -(30784 as libc::c_int);
                current_block = 10041771570435381152;
            } else {
                if (*mc).mc_flags & 1 as libc::c_uint == 0 {
                    rc = mdb_cursor_last(mc, key, data);
                } else {
                    rc = 0 as libc::c_int;
                }
                if rc == 0 as libc::c_int {
                    mx___0 = &mut (*(*mc).mc_xcursor).mx_cursor;
                    if (*mx___0).mc_flags & 1 as libc::c_uint != 0 {
                        rc = mdb_cursor_sibling(mx___0, 0 as libc::c_int);
                        if rc == 0 as libc::c_int {
                            current_block = 1416643152622400344;
                        } else {
                            current_block = 10041771570435381152;
                        }
                    } else {
                        rc = -(30798 as libc::c_int);
                        current_block = 10041771570435381152;
                    }
                } else {
                    current_block = 10041771570435381152;
                }
            }
        }
        11 | 9 | 8 => {
            rc = mdb_cursor_next(mc, key, data, op);
            current_block = 10041771570435381152;
        }
        14 | 13 | 12 => {
            rc = mdb_cursor_prev(mc, key, data, op);
            current_block = 10041771570435381152;
        }
        0 => {
            rc = mdb_cursor_first(mc, key, data);
            current_block = 10041771570435381152;
        }
        1 => {
            mfunc = Some(
                mdb_cursor_first
                    as unsafe extern "C" fn(
                        *mut MDB_cursor,
                        *mut MDB_val,
                        *mut MDB_val,
                    ) -> libc::c_int,
            );
            current_block = 11789378541793160248;
        }
        6 => {
            rc = mdb_cursor_last(mc, key, data);
            current_block = 10041771570435381152;
        }
        7 => {
            mfunc = Some(
                mdb_cursor_last
                    as unsafe extern "C" fn(
                        *mut MDB_cursor,
                        *mut MDB_val,
                        *mut MDB_val,
                    ) -> libc::c_int,
            );
            current_block = 11789378541793160248;
        }
        _ => {
            rc = 22 as libc::c_int;
            current_block = 10041771570435381152;
        }
    }
    match current_block {
        11789378541793160248 => {
            if data as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                rc = 22 as libc::c_int;
            } else if (*mc).mc_flags & 1 as libc::c_uint == 0 {
                rc = 22 as libc::c_int;
            } else if (*mc).mc_xcursor as libc::c_ulong
                    == 0 as *mut libc::c_void as libc::c_ulong
                {
                rc = -(30784 as libc::c_int);
            } else if (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint
                    >= ((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_pb.pb.pb_lower
                        as libc::c_uint)
                        .wrapping_sub(
                            &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                as libc::c_ulong as libc::c_uint,
                        ) >> 1 as libc::c_int
                {
                (*mc)
                    .mc_ki[(*mc).mc_top
                    as usize] = (((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_pb.pb.pb_lower
                    as libc::c_uint)
                    .wrapping_sub(
                        &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                            as libc::c_ulong as libc::c_uint,
                    ) >> 1 as libc::c_int) as indx_t;
                rc = -(30798 as libc::c_int);
            } else {
                (*mc).mc_flags &= 4294967293 as libc::c_uint;
                leaf___0 = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
                    .offset(
                        *((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_ptrs)
                            .as_mut_ptr()
                            .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                            as libc::c_int as isize,
                    )
                    .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                if !((*leaf___0).mn_flags as libc::c_int & 4 as libc::c_int
                    == 4 as libc::c_int)
                {
                    if key as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                        (*key).mv_size = (*leaf___0).mn_ksize as size_t;
                        (*key)
                            .mv_data = ((*leaf___0).mn_data).as_mut_ptr()
                            as *mut libc::c_void;
                    }
                    rc = mdb_node_read(mc, leaf___0, data);
                } else if (*(*mc).mc_xcursor).mx_cursor.mc_flags & 1 as libc::c_uint == 0
                    {
                    rc = 22 as libc::c_int;
                } else {
                    rc = (Some(mfunc.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(
                        &mut (*(*mc).mc_xcursor).mx_cursor,
                        data,
                        0 as *mut libc::c_void as *mut MDB_val,
                    );
                }
            }
        }
        1416643152622400344 => {
            mx = &mut (*(*mc).mc_xcursor).mx_cursor;
            (*data)
                .mv_size = (((*(*mx).mc_pg[(*mx).mc_top as usize]).mp_pb.pb.pb_lower
                as libc::c_uint)
                .wrapping_sub(
                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                        as libc::c_ulong as libc::c_uint,
                ) >> 1 as libc::c_int)
                .wrapping_mul((*(*mx).mc_db).md_pad) as size_t;
            (*data)
                .mv_data = ((*mx).mc_pg[(*mx).mc_top as usize] as *mut libc::c_char)
                .offset(
                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                        as libc::c_ulong as libc::c_uint as isize,
                ) as *mut libc::c_void;
            (*mx)
                .mc_ki[(*mx).mc_top
                as usize] = (((*(*mx).mc_pg[(*mx).mc_top as usize]).mp_pb.pb.pb_lower
                as libc::c_uint)
                .wrapping_sub(
                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                        as libc::c_ulong as libc::c_uint,
                ) >> 1 as libc::c_int)
                .wrapping_sub(1 as libc::c_uint) as indx_t;
        }
        11404259345410850769 => {
            if key as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                rc = 22 as libc::c_int;
            } else {
                if op as libc::c_uint == 17 as libc::c_uint {
                    tmp = 0 as *mut libc::c_void as *mut libc::c_int;
                } else {
                    tmp = &mut exact;
                }
                rc = mdb_cursor_set(mc, key, data, op, tmp);
            }
        }
        _ => {}
    }
    if (*mc).mc_flags & 8 as libc::c_uint != 0 {
        (*mc).mc_flags ^= 8 as libc::c_uint;
    }
    return rc;
}
unsafe extern "C" fn mdb_cursor_touch(mut mc: *mut MDB_cursor) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut mc2: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut mcx: MDB_xcursor = MDB_xcursor {
        mx_cursor: MDB_cursor {
            mc_next: 0 as *mut MDB_cursor,
            mc_backup: 0 as *mut MDB_cursor,
            mc_xcursor: 0 as *mut MDB_xcursor,
            mc_txn: 0 as *mut MDB_txn,
            mc_dbi: 0,
            mc_db: 0 as *mut MDB_db,
            mc_dbx: 0 as *mut MDB_dbx,
            mc_dbflag: 0 as *mut libc::c_uchar,
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [0 as *mut MDB_page; 32],
            mc_ki: [0; 32],
        },
        mx_db: MDB_db {
            md_pad: 0,
            md_flags: 0,
            md_depth: 0,
            md_branch_pages: 0,
            md_leaf_pages: 0,
            md_overflow_pages: 0,
            md_entries: 0,
            md_root: 0,
        },
        mx_dbx: MDB_dbx {
            md_name: MDB_val {
                mv_size: 0,
                mv_data: 0 as *mut libc::c_void,
            },
            md_cmp: None,
            md_dcmp: None,
            md_rel: None,
            md_relctx: 0 as *mut libc::c_void,
        },
        mx_dbflag: 0,
    };
    rc = 0 as libc::c_int;
    if (*mc).mc_dbi >= 2 as libc::c_uint {
        if *(*mc).mc_dbflag as libc::c_int & 33 as libc::c_int == 0 {
            if *((*(*mc).mc_txn).mt_dbiseqs).offset((*mc).mc_dbi as isize)
                != *((*(*(*mc).mc_txn).mt_env).me_dbiseqs).offset((*mc).mc_dbi as isize)
            {
                return -(30780 as libc::c_int);
            }
            mdb_cursor_init(
                &mut mc2,
                (*mc).mc_txn,
                1 as libc::c_int as MDB_dbi,
                &mut mcx,
            );
            rc = mdb_page_search(
                &mut mc2,
                &mut (*(*mc).mc_dbx).md_name,
                1 as libc::c_int,
            );
            if rc != 0 {
                return rc;
            }
            *(*mc)
                .mc_dbflag = (*(*mc).mc_dbflag as libc::c_int | 1 as libc::c_int)
                as libc::c_uchar;
        }
    }
    (*mc).mc_top = 0 as libc::c_int as libc::c_ushort;
    if (*mc).mc_snum != 0 {
        loop {
            rc = mdb_page_touch(mc);
            if !(rc == 0) {
                break;
            }
            (*mc)
                .mc_top = ((*mc).mc_top as libc::c_int + 1 as libc::c_int)
                as libc::c_ushort;
            if !(((*mc).mc_top as libc::c_int) < (*mc).mc_snum as libc::c_int) {
                break;
            }
        }
        (*mc)
            .mc_top = ((*mc).mc_snum as libc::c_int - 1 as libc::c_int)
            as libc::c_ushort;
    }
    return rc;
}
pub unsafe extern "C" fn mdb_cursor_put(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut flags: libc::c_uint,
) -> libc::c_int {
    let mut current_block: u64;
    let mut env: *mut MDB_env = 0 as *mut MDB_env;
    let mut leaf: *mut MDB_node = 0 as *mut MDB_node;
    let mut fp: *mut MDB_page = 0 as *mut MDB_page;
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut sub_root: *mut MDB_page = 0 as *mut MDB_page;
    let mut fp_flags: uint16_t = 0;
    let mut xdata: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut rdata: *mut MDB_val = 0 as *mut MDB_val;
    let mut dkey: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut olddata: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut dummy: MDB_db = MDB_db {
        md_pad: 0,
        md_flags: 0,
        md_depth: 0,
        md_branch_pages: 0,
        md_leaf_pages: 0,
        md_overflow_pages: 0,
        md_entries: 0,
        md_root: 0,
    };
    let mut do_sub: libc::c_int = 0;
    let mut insert_key: libc::c_int = 0;
    let mut insert_data: libc::c_int = 0;
    let mut mcount: libc::c_uint = 0;
    let mut dcount: libc::c_uint = 0;
    let mut nospill: libc::c_uint = 0;
    let mut nsize: size_t = 0;
    let mut rc: libc::c_int = 0;
    let mut rc2: libc::c_int = 0;
    let mut nflags: libc::c_uint = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_ulong = 0;
    let mut exact: libc::c_int = 0;
    let mut d2: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut k2: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut np: *mut MDB_page = 0 as *mut MDB_page;
    let mut tmp___1: indx_t = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ksize: libc::c_uint = 0;
    let mut dtop: libc::c_ushort = 0;
    let mut i: libc::c_uint = 0;
    let mut offset: libc::c_uint = 0;
    let mut dcmp: Option::<MDB_cmp_func> = None;
    let mut tmp___2: libc::c_int = 0;
    let mut omp: *mut MDB_page = 0 as *mut MDB_page;
    let mut pg: pgno_t = 0;
    let mut level: libc::c_int = 0;
    let mut ovpages: libc::c_int = 0;
    let mut dpages: libc::c_int = 0;
    let mut sz: size_t = 0;
    let mut off: size_t = 0;
    let mut np___0: *mut MDB_page = 0 as *mut MDB_page;
    let mut tmp___3: *mut MDB_page = 0 as *mut MDB_page;
    let mut id2: MDB_ID2 = MDB_ID2 {
        mid: 0,
        mptr: 0 as *mut libc::c_void,
    };
    let mut tmp___4: size_t = 0;
    let mut m2: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut m3: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut dbi: MDB_dbi = 0;
    let mut i___0: libc::c_uint = 0;
    let mut mp___0: *mut MDB_page = 0 as *mut MDB_page;
    let mut xr_pg: *mut MDB_page = 0 as *mut MDB_page;
    let mut xr_node: *mut MDB_node = 0 as *mut MDB_node;
    let mut xflags: libc::c_int = 0;
    let mut new_dupdata: libc::c_int = 0;
    let mut ecount: mdb_size_t = 0;
    let mut m2___0: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut mx: *mut MDB_xcursor = 0 as *mut MDB_xcursor;
    let mut i___1: libc::c_uint = 0;
    let mut mp___1: *mut MDB_page = 0 as *mut MDB_page;
    let mut xr_pg___0: *mut MDB_page = 0 as *mut MDB_page;
    let mut xr_node___0: *mut MDB_node = 0 as *mut MDB_node;
    let mut db: *mut libc::c_void = 0 as *mut libc::c_void;
    leaf = 0 as *mut libc::c_void as *mut MDB_node;
    sub_root = 0 as *mut libc::c_void as *mut MDB_page;
    do_sub = 0 as libc::c_int;
    mcount = 0 as libc::c_uint;
    dcount = 0 as libc::c_uint;
    if mc as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 22 as libc::c_int
    } else {
        if key as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            return 22 as libc::c_int;
        }
    }
    env = (*(*mc).mc_txn).mt_env;
    if flags & 524288 as libc::c_uint != 0 {
        dcount = (*data.offset(1 as libc::c_int as isize)).mv_size as libc::c_uint;
        (*data.offset(1 as libc::c_int as isize)).mv_size = 0 as libc::c_int as size_t;
        if !((*(*mc).mc_db).md_flags as libc::c_int & 16 as libc::c_int
            == 16 as libc::c_int)
        {
            return -(30784 as libc::c_int);
        }
    }
    nospill = flags & 32768 as libc::c_uint;
    flags &= 4294934527 as libc::c_uint;
    if (*(*mc).mc_txn).mt_flags & 131091 as libc::c_uint != 0 {
        if (*(*mc).mc_txn).mt_flags & 131072 as libc::c_uint != 0 {
            tmp = 13 as libc::c_int;
        } else {
            tmp = -(30782 as libc::c_int);
        }
        return tmp;
    }
    if ((*key).mv_size).wrapping_sub(1 as libc::c_ulong) >= 511 as libc::c_ulong {
        return -(30781 as libc::c_int);
    }
    if (*(*mc).mc_db).md_flags as libc::c_int & 4 as libc::c_int != 0 {
        tmp___0 = 511 as libc::c_ulong;
    } else {
        tmp___0 = 4294967295 as libc::c_ulong;
    }
    if (*data).mv_size > tmp___0 {
        return -(30781 as libc::c_int);
    }
    dkey.mv_size = 0 as libc::c_int as size_t;
    if flags & 64 as libc::c_uint != 0 {
        if (*mc).mc_flags & 1 as libc::c_uint == 0 {
            return 22 as libc::c_int;
        }
        rc = 0 as libc::c_int;
    } else if (*(*mc).mc_db).md_root as libc::c_ulonglong
            == 18446744073709551615 as libc::c_ulonglong
        {
        (*mc).mc_snum = 0 as libc::c_int as libc::c_ushort;
        (*mc).mc_top = 0 as libc::c_int as libc::c_ushort;
        (*mc).mc_flags &= 4294967294 as libc::c_uint;
        rc = -(30769 as libc::c_int);
    } else {
        exact = 0 as libc::c_int;
        if flags & 131072 as libc::c_uint != 0 {
            rc = mdb_cursor_last(mc, &mut k2, &mut d2);
            if rc == 0 as libc::c_int {
                rc = (Some(((*(*mc).mc_dbx).md_cmp).expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(key as *const MDB_val, &mut k2 as *mut MDB_val as *const MDB_val);
                if rc > 0 as libc::c_int {
                    rc = -(30798 as libc::c_int);
                    (*mc)
                        .mc_ki[(*mc).mc_top
                        as usize] = ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_int
                        + 1 as libc::c_int) as indx_t;
                } else {
                    rc = -(30799 as libc::c_int);
                }
            }
        } else {
            rc = mdb_cursor_set(mc, key, &mut d2, MDB_SET, &mut exact);
        }
        if flags & 16 as libc::c_uint != 0 {
            if rc == 0 as libc::c_int {
                *data = d2;
                return -(30799 as libc::c_int);
            }
        }
        if rc != 0 {
            if rc != -(30798 as libc::c_int) {
                return rc;
            }
        }
    }
    if (*mc).mc_flags & 8 as libc::c_uint != 0 {
        (*mc).mc_flags ^= 8 as libc::c_uint;
    }
    if nospill == 0 {
        if flags & 524288 as libc::c_uint != 0 {
            rdata = &mut xdata;
            xdata.mv_size = ((*data).mv_size).wrapping_mul(dcount as size_t);
        } else {
            rdata = data;
        }
        rc2 = mdb_page_spill(mc, key, rdata);
        if rc2 != 0 {
            return rc2;
        }
    }
    if rc == -(30769 as libc::c_int) {
        rc2 = mdb_page_new(mc, 2 as libc::c_int as uint32_t, 1 as libc::c_int, &mut np);
        if rc2 != 0 {
            return rc2;
        }
        mdb_cursor_push(mc, np);
        (*(*mc).mc_db).md_root = (*np).mp_p.p_pgno;
        (*(*mc).mc_db)
            .md_depth = ((*(*mc).mc_db).md_depth as libc::c_int + 1 as libc::c_int)
            as uint16_t;
        *(*mc)
            .mc_dbflag = (*(*mc).mc_dbflag as libc::c_int | 1 as libc::c_int)
            as libc::c_uchar;
        if (*(*mc).mc_db).md_flags as libc::c_int & 20 as libc::c_int
            == 16 as libc::c_int
        {
            (*np)
                .mp_flags = ((*np).mp_flags as libc::c_int | 32 as libc::c_int)
                as uint16_t;
        }
        (*mc).mc_flags |= 1 as libc::c_uint;
    } else {
        rc2 = mdb_cursor_touch(mc);
        if rc2 != 0 {
            return rc2;
        }
    }
    insert_data = rc;
    insert_key = insert_data;
    if insert_key != 0 {
        if (*(*mc).mc_db).md_flags as libc::c_int & 4 as libc::c_int != 0 {
            if (&mut (*(0 as *mut MDB_node)).mn_data as *mut [libc::c_char; 1]
                as libc::c_ulong)
                .wrapping_add((*key).mv_size)
                .wrapping_add((*data).mv_size) > (*env).me_nodemax as libc::c_ulong
            {
                fp_flags = 18 as libc::c_int as uint16_t;
                fp = (*env).me_pbuf as *mut MDB_page;
                (*fp).mp_pad = (*data).mv_size as uint16_t;
                tmp___1 = &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                    as libc::c_ulong as libc::c_uint as indx_t;
                (*fp).mp_pb.pb.pb_upper = tmp___1;
                (*fp).mp_pb.pb.pb_lower = tmp___1;
                olddata
                    .mv_size = &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                    as libc::c_ulong as libc::c_uint as size_t;
                current_block = 7534526937055664506;
            } else {
                current_block = 17061923640837754246;
            }
        } else {
            current_block = 17061923640837754246;
        }
    } else if (*(*mc).mc_pg[(*mc).mc_top as usize]).mp_flags as libc::c_int
            & 32 as libc::c_int == 32 as libc::c_int
        {
        ksize = (*(*mc).mc_db).md_pad;
        if (*key).mv_size != ksize as size_t {
            return -(30781 as libc::c_int);
        }
        ptr = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
            .offset(
                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                    as libc::c_uint as isize,
            )
            .offset(
                ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint).wrapping_mul(ksize)
                    as isize,
            );
        memcpy(
            ptr as *mut libc::c_void,
            (*key).mv_data as *const libc::c_void,
            ksize as size_t,
        );
        current_block = 7324368273598849437;
    } else {
        current_block = 9768578057025712957;
    }
    loop {
        match current_block {
            17061923640837754246 => {
                rdata = data;
                current_block = 11950936639099900136;
            }
            9768578057025712957 => {
                leaf = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
                    .offset(
                        *((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_ptrs)
                            .as_mut_ptr()
                            .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                            as libc::c_int as isize,
                    )
                    .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                olddata
                    .mv_size = ((*leaf).mn_lo as libc::c_uint
                    | ((*leaf).mn_hi as libc::c_uint) << 16 as libc::c_int) as size_t;
                olddata
                    .mv_data = ((*leaf).mn_data)
                    .as_mut_ptr()
                    .offset((*leaf).mn_ksize as libc::c_int as isize)
                    as *mut libc::c_void;
                if (*(*mc).mc_db).md_flags as libc::c_int & 4 as libc::c_int
                    == 4 as libc::c_int
                {
                    offset = 0 as libc::c_uint;
                    xdata.mv_data = (*env).me_pbuf;
                    fp = xdata.mv_data as *mut MDB_page;
                    mp = fp;
                    (*mp)
                        .mp_p
                        .p_pgno = (*(*mc).mc_pg[(*mc).mc_top as usize]).mp_p.p_pgno;
                    if !((*leaf).mn_flags as libc::c_int & 4 as libc::c_int
                        == 4 as libc::c_int)
                    {
                        if flags == 64 as libc::c_uint {
                            current_block = 13429473295153001803;
                        } else {
                            dcmp = (*(*mc).mc_dbx).md_dcmp;
                            if ::std::mem::transmute::<
                                Option::<MDB_cmp_func>,
                                libc::c_ulong,
                            >(dcmp)
                                == ::std::mem::transmute::<
                                    Option::<
                                        unsafe extern "C" fn(
                                            *const MDB_val,
                                            *const MDB_val,
                                        ) -> libc::c_int,
                                    >,
                                    libc::c_ulong,
                                >(
                                    Some(
                                        mdb_cmp_int
                                            as unsafe extern "C" fn(
                                                *const MDB_val,
                                                *const MDB_val,
                                            ) -> libc::c_int,
                                    ),
                                )
                            {
                                if olddata.mv_size
                                    == ::std::mem::size_of::<mdb_size_t>() as libc::c_ulong
                                {
                                    dcmp = Some(
                                        mdb_cmp_long
                                            as unsafe extern "C" fn(
                                                *const MDB_val,
                                                *const MDB_val,
                                            ) -> libc::c_int,
                                    );
                                }
                            }
                            tmp___2 = (Some(dcmp.expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(
                                data as *const MDB_val,
                                &mut olddata as *mut MDB_val as *const MDB_val,
                            );
                            if tmp___2 == 0 {
                                if flags & 262176 as libc::c_uint != 0 {
                                    return -(30799 as libc::c_int);
                                }
                                current_block = 13429473295153001803;
                            } else {
                                dkey.mv_size = olddata.mv_size;
                                dkey
                                    .mv_data = memcpy(
                                    fp.offset(1 as libc::c_int as isize) as *mut libc::c_void,
                                    olddata.mv_data as *const libc::c_void,
                                    olddata.mv_size,
                                );
                                (*fp).mp_flags = 82 as libc::c_int as uint16_t;
                                (*fp)
                                    .mp_pb
                                    .pb
                                    .pb_lower = &mut (*(0 as *mut MDB_page)).mp_ptrs
                                    as *mut [indx_t; 1] as libc::c_ulong as libc::c_uint
                                    as indx_t;
                                xdata
                                    .mv_size = (&mut (*(0 as *mut MDB_page)).mp_ptrs
                                    as *mut [indx_t; 1] as libc::c_ulong as libc::c_uint
                                    as size_t)
                                    .wrapping_add(dkey.mv_size)
                                    .wrapping_add((*data).mv_size);
                                if (*(*mc).mc_db).md_flags as libc::c_int
                                    & 16 as libc::c_int != 0
                                {
                                    (*fp)
                                        .mp_flags = ((*fp).mp_flags as libc::c_int
                                        | 32 as libc::c_int) as uint16_t;
                                    (*fp).mp_pad = (*data).mv_size as uint16_t;
                                    xdata
                                        .mv_size = (xdata.mv_size as libc::c_ulong)
                                        .wrapping_add(
                                            (2 as libc::c_ulong).wrapping_mul((*data).mv_size),
                                        ) as size_t as size_t;
                                } else {
                                    xdata
                                        .mv_size = (xdata.mv_size as libc::c_ulong)
                                        .wrapping_add(
                                            (2 as libc::c_ulong)
                                                .wrapping_mul(
                                                    (::std::mem::size_of::<indx_t>() as libc::c_ulong)
                                                        .wrapping_add(
                                                            &mut (*(0 as *mut MDB_node)).mn_data
                                                                as *mut [libc::c_char; 1] as libc::c_ulong,
                                                        ),
                                                )
                                                .wrapping_add(dkey.mv_size & 1 as libc::c_ulong)
                                                .wrapping_add((*data).mv_size & 1 as libc::c_ulong),
                                        ) as size_t as size_t;
                                }
                                (*fp).mp_pb.pb.pb_upper = xdata.mv_size as indx_t;
                                olddata.mv_size = xdata.mv_size;
                                current_block = 12045739402850935335;
                            }
                        }
                    } else if (*leaf).mn_flags as libc::c_int & 2 as libc::c_int != 0 {
                        flags |= 6 as libc::c_uint;
                        current_block = 7606014630461122334;
                    } else {
                        fp = olddata.mv_data as *mut MDB_page;
                        match flags {
                            64 => {
                                current_block = 16145095014185817216;
                            }
                            _ => {
                                if (*(*mc).mc_db).md_flags as libc::c_int
                                    & 16 as libc::c_int == 0
                                {
                                    offset = ((&mut (*(0 as *mut MDB_node)).mn_data
                                        as *mut [libc::c_char; 1] as libc::c_ulong)
                                        .wrapping_add(
                                            ::std::mem::size_of::<indx_t>() as libc::c_ulong,
                                        )
                                        .wrapping_add((*data).mv_size)
                                        .wrapping_add(1 as libc::c_ulong) as libc::c_ulonglong
                                        & 18446744073709551614 as libc::c_ulonglong)
                                        as libc::c_uint;
                                    current_block = 6692429041597365501;
                                } else {
                                    offset = (*fp).mp_pad as libc::c_uint;
                                    if (((*fp).mp_pb.pb.pb_upper as libc::c_int
                                        - (*fp).mp_pb.pb.pb_lower as libc::c_int) as indx_t
                                        as libc::c_uint) < offset
                                    {
                                        offset = offset.wrapping_mul(4 as libc::c_uint);
                                        current_block = 6692429041597365501;
                                    } else {
                                        current_block = 16145095014185817216;
                                    }
                                }
                                match current_block {
                                    16145095014185817216 => {}
                                    _ => {
                                        xdata
                                            .mv_size = (olddata.mv_size).wrapping_add(offset as size_t);
                                        current_block = 12045739402850935335;
                                    }
                                }
                            }
                        }
                        match current_block {
                            12045739402850935335 => {}
                            _ => {
                                (*fp)
                                    .mp_flags = ((*fp).mp_flags as libc::c_int
                                    | 16 as libc::c_int) as uint16_t;
                                (*fp).mp_p.p_pgno = (*mp).mp_p.p_pgno;
                                (*(*mc).mc_xcursor)
                                    .mx_cursor
                                    .mc_pg[0 as libc::c_int as usize] = fp;
                                flags |= 4 as libc::c_uint;
                                current_block = 7606014630461122334;
                            }
                        }
                    }
                    match current_block {
                        7606014630461122334 => {}
                        13429473295153001803 => {}
                        _ => {
                            fp_flags = (*fp).mp_flags;
                            if (&mut (*(0 as *mut MDB_node)).mn_data
                                as *mut [libc::c_char; 1] as libc::c_ulong)
                                .wrapping_add((*leaf).mn_ksize as libc::c_ulong)
                                .wrapping_add(xdata.mv_size)
                                > (*env).me_nodemax as libc::c_ulong
                            {
                                fp_flags = (fp_flags as libc::c_int & -(65 as libc::c_int))
                                    as uint16_t;
                                current_block = 7534526937055664506;
                                continue;
                            } else {
                                current_block = 4491581808830814586;
                            }
                        }
                    }
                } else {
                    current_block = 13429473295153001803;
                }
                match current_block {
                    4491581808830814586 => {}
                    7606014630461122334 => {}
                    _ => {
                        if ((*leaf).mn_flags as libc::c_uint ^ flags) & 2 as libc::c_uint
                            != 0
                        {
                            return -(30784 as libc::c_int);
                        }
                        if (*leaf).mn_flags as libc::c_int & 1 as libc::c_int
                            == 1 as libc::c_int
                        {
                            dpages = ((&mut (*(0 as *mut MDB_page)).mp_ptrs
                                as *mut [indx_t; 1] as libc::c_ulong as libc::c_uint)
                                .wrapping_sub(1 as libc::c_uint) as size_t)
                                .wrapping_add((*data).mv_size)
                                .wrapping_div((*env).me_psize as size_t)
                                .wrapping_add(1 as libc::c_ulong) as libc::c_int;
                            memcpy(
                                &mut pg as *mut pgno_t as *mut libc::c_void,
                                olddata.mv_data as *const libc::c_void,
                                ::std::mem::size_of::<pgno_t>() as libc::c_ulong,
                            );
                            rc2 = mdb_page_get(mc, pg, &mut omp, &mut level);
                            if rc2 != 0 as libc::c_int {
                                return rc2;
                            }
                            ovpages = (*omp).mp_pb.pb_pages as libc::c_int;
                            if ovpages >= dpages {
                                if (*omp).mp_flags as libc::c_int & 16 as libc::c_int == 0 {
                                    let mut current_block_255: u64;
                                    if level != 0 {
                                        current_block_255 = 295236766281327930;
                                    } else if (*env).me_flags & 524288 as libc::c_uint != 0 {
                                        current_block_255 = 295236766281327930;
                                    } else {
                                        current_block_255 = 17792673440321285907;
                                    }
                                    match current_block_255 {
                                        295236766281327930 => {
                                            rc = mdb_page_unspill((*mc).mc_txn, omp, &mut omp);
                                            if rc != 0 {
                                                return rc;
                                            }
                                            level = 0 as libc::c_int;
                                        }
                                        _ => {}
                                    }
                                }
                                if (*omp).mp_flags as libc::c_int & 16 as libc::c_int != 0 {
                                    if level > 1 as libc::c_int {
                                        sz = ((*env).me_psize as size_t)
                                            .wrapping_mul(ovpages as size_t);
                                        tmp___3 = mdb_page_malloc(
                                            (*mc).mc_txn,
                                            ovpages as libc::c_uint,
                                        );
                                        np___0 = tmp___3;
                                        if np___0.is_null() {
                                            return 12 as libc::c_int;
                                        }
                                        id2.mid = pg;
                                        id2.mptr = np___0 as *mut libc::c_void;
                                        rc2 = mdb_mid2l_insert(
                                            (*(*mc).mc_txn).mt_u.dirty_list,
                                            &mut id2,
                                        );
                                        if !(rc2 == 0 as libc::c_int) {
                                            mdb_assert_fail(
                                                (*(*mc).mc_txn).mt_env,
                                                b"rc2 == 0\0" as *const u8 as *const libc::c_char,
                                                b"mdb_cursor_put\0" as *const u8 as *const libc::c_char,
                                                b"mdb.c\0" as *const u8 as *const libc::c_char,
                                                7913 as libc::c_int,
                                            );
                                        }
                                        if flags & 65536 as libc::c_uint == 0 {
                                            off = (&mut (*(0 as *mut MDB_page)).mp_ptrs
                                                as *mut [indx_t; 1] as libc::c_ulong as libc::c_uint
                                                as size_t)
                                                .wrapping_add((*data).mv_size)
                                                & (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                                    .wrapping_neg();
                                            memcpy(
                                                (np___0 as *mut libc::c_char).offset(off as isize)
                                                    as *mut size_t as *mut libc::c_void,
                                                (omp as *mut libc::c_char).offset(off as isize)
                                                    as *mut size_t as *const libc::c_void,
                                                sz.wrapping_sub(off),
                                            );
                                            sz = &mut (*(0 as *mut MDB_page)).mp_ptrs
                                                as *mut [indx_t; 1] as libc::c_ulong as libc::c_uint
                                                as size_t;
                                        }
                                        memcpy(
                                            np___0 as *mut libc::c_void,
                                            omp as *const libc::c_void,
                                            sz,
                                        );
                                        omp = np___0;
                                    }
                                    (*leaf)
                                        .mn_lo = ((*data).mv_size & 65535 as libc::c_ulong)
                                        as libc::c_ushort;
                                    (*leaf)
                                        .mn_hi = ((*data).mv_size >> 16 as libc::c_int)
                                        as libc::c_ushort;
                                    if flags & 65536 as libc::c_uint == 65536 as libc::c_uint {
                                        (*data)
                                            .mv_data = (omp as *mut libc::c_char)
                                            .offset(
                                                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                                    as libc::c_ulong as libc::c_uint as isize,
                                            ) as *mut libc::c_void;
                                    } else {
                                        memcpy(
                                            (omp as *mut libc::c_char)
                                                .offset(
                                                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                                        as libc::c_ulong as libc::c_uint as isize,
                                                ) as *mut libc::c_void,
                                            (*data).mv_data as *const libc::c_void,
                                            (*data).mv_size,
                                        );
                                    }
                                    return 0 as libc::c_int;
                                }
                            }
                            rc2 = mdb_ovpage_free(mc, omp);
                            if rc2 != 0 as libc::c_int {
                                return rc2;
                            }
                        } else if (*data).mv_size == olddata.mv_size {
                            if flags & 65536 as libc::c_uint == 65536 as libc::c_uint {
                                (*data).mv_data = olddata.mv_data;
                                current_block = 5323028055506826224;
                                break;
                            } else if (*mc).mc_flags & 4 as libc::c_uint == 0 {
                                memcpy(
                                    olddata.mv_data,
                                    (*data).mv_data as *const libc::c_void,
                                    (*data).mv_size,
                                );
                                current_block = 5323028055506826224;
                                break;
                            } else {
                                memcpy(
                                    ((*leaf).mn_data).as_mut_ptr() as *mut libc::c_void,
                                    (*key).mv_data as *const libc::c_void,
                                    (*key).mv_size,
                                );
                                current_block = 7324368273598849437;
                                continue;
                            }
                        }
                        mdb_node_del(mc, 0 as libc::c_int);
                        current_block = 17061923640837754246;
                        continue;
                    }
                }
            }
            7324368273598849437 => {
                if (*mc).mc_top != 0 {
                    if (*mc).mc_ki[(*mc).mc_top as usize] == 0 {
                        dtop = 1 as libc::c_int as libc::c_ushort;
                        (*mc)
                            .mc_top = ((*mc).mc_top as libc::c_int - 1 as libc::c_int)
                            as libc::c_ushort;
                        while (*mc).mc_top != 0 {
                            if (*mc).mc_ki[(*mc).mc_top as usize] != 0 {
                                break;
                            }
                            (*mc)
                                .mc_top = ((*mc).mc_top as libc::c_int - 1 as libc::c_int)
                                as libc::c_ushort;
                            dtop = (dtop as libc::c_int + 1 as libc::c_int)
                                as libc::c_ushort;
                        }
                        if (*mc).mc_ki[(*mc).mc_top as usize] != 0 {
                            rc2 = mdb_update_key(mc, key);
                        } else {
                            rc2 = 0 as libc::c_int;
                        }
                        (*mc)
                            .mc_top = ((*mc).mc_top as libc::c_int + dtop as libc::c_int)
                            as libc::c_ushort;
                        if rc2 != 0 {
                            return rc2;
                        }
                    }
                }
                return 0 as libc::c_int;
            }
            _ => {
                if (*(*mc).mc_db).md_flags as libc::c_int & 16 as libc::c_int != 0 {
                    fp_flags = (fp_flags as libc::c_int | 32 as libc::c_int) as uint16_t;
                    dummy.md_pad = (*fp).mp_pad as uint32_t;
                    dummy.md_flags = 16 as libc::c_int as uint16_t;
                    if (*(*mc).mc_db).md_flags as libc::c_int & 32 as libc::c_int != 0 {
                        dummy
                            .md_flags = (dummy.md_flags as libc::c_int
                            | 8 as libc::c_int) as uint16_t;
                    }
                } else {
                    dummy.md_pad = 0 as libc::c_int as uint32_t;
                    dummy.md_flags = 0 as libc::c_int as uint16_t;
                }
                dummy.md_depth = 1 as libc::c_int as uint16_t;
                dummy.md_branch_pages = 0 as libc::c_int as pgno_t;
                dummy.md_leaf_pages = 1 as libc::c_int as pgno_t;
                dummy.md_overflow_pages = 0 as libc::c_int as pgno_t;
                dummy
                    .md_entries = (((*fp).mp_pb.pb.pb_lower as libc::c_uint)
                    .wrapping_sub(
                        &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                            as libc::c_ulong as libc::c_uint,
                    ) >> 1 as libc::c_int) as mdb_size_t;
                xdata.mv_size = ::std::mem::size_of::<MDB_db>() as libc::c_ulong;
                xdata.mv_data = &mut dummy as *mut MDB_db as *mut libc::c_void;
                rc = mdb_page_alloc(mc, 1 as libc::c_int, &mut mp);
                if rc != 0 {
                    return rc;
                }
                offset = ((*env).me_psize as size_t).wrapping_sub(olddata.mv_size)
                    as libc::c_uint;
                flags |= 6 as libc::c_uint;
                dummy.md_root = (*mp).mp_p.p_pgno;
                sub_root = mp;
                current_block = 4491581808830814586;
            }
        }
        match current_block {
            4491581808830814586 => {
                if mp as libc::c_ulong != fp as libc::c_ulong {
                    (*mp)
                        .mp_flags = (fp_flags as libc::c_int | 16 as libc::c_int)
                        as uint16_t;
                    (*mp).mp_pad = (*fp).mp_pad;
                    (*mp).mp_pb.pb.pb_lower = (*fp).mp_pb.pb.pb_lower;
                    (*mp)
                        .mp_pb
                        .pb
                        .pb_upper = ((*fp).mp_pb.pb.pb_upper as libc::c_uint)
                        .wrapping_add(offset) as indx_t;
                    if fp_flags as libc::c_int & 32 as libc::c_int != 0 {
                        memcpy(
                            (mp as *mut libc::c_char)
                                .offset(
                                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                        as libc::c_ulong as libc::c_uint as isize,
                                ) as *mut libc::c_void,
                            (fp as *mut libc::c_char)
                                .offset(
                                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                        as libc::c_ulong as libc::c_uint as isize,
                                ) as *mut libc::c_void as *const libc::c_void,
                            (((*fp).mp_pb.pb.pb_lower as libc::c_uint)
                                .wrapping_sub(
                                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                        as libc::c_ulong as libc::c_uint,
                                ) >> 1 as libc::c_int)
                                .wrapping_mul((*fp).mp_pad as libc::c_uint) as size_t,
                        );
                    } else {
                        memcpy(
                            (mp as *mut libc::c_char)
                                .offset((*mp).mp_pb.pb.pb_upper as libc::c_int as isize)
                                .offset(0 as libc::c_uint as isize) as *mut libc::c_void,
                            (fp as *mut libc::c_char)
                                .offset((*fp).mp_pb.pb.pb_upper as libc::c_int as isize)
                                .offset(0 as libc::c_uint as isize) as *const libc::c_void,
                            (olddata.mv_size)
                                .wrapping_sub((*fp).mp_pb.pb.pb_upper as size_t),
                        );
                        memcpy(
                            &mut (*mp).mp_ptrs as *mut [indx_t; 1] as *mut libc::c_char
                                as *mut libc::c_void,
                            &mut (*fp).mp_ptrs as *mut [indx_t; 1] as *mut libc::c_char
                                as *const libc::c_void,
                            ((((*fp).mp_pb.pb.pb_lower as libc::c_uint)
                                .wrapping_sub(
                                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                        as libc::c_ulong as libc::c_uint,
                                ) >> 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<indx_t>() as libc::c_ulong,
                                ),
                        );
                        i = 0 as libc::c_uint;
                        while i
                            < ((*fp).mp_pb.pb.pb_lower as libc::c_uint)
                                .wrapping_sub(
                                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                        as libc::c_ulong as libc::c_uint,
                                ) >> 1 as libc::c_int
                        {
                            *((*mp).mp_ptrs)
                                .as_mut_ptr()
                                .offset(
                                    i as isize,
                                ) = (*((*mp).mp_ptrs).as_mut_ptr().offset(i as isize)
                                as libc::c_uint)
                                .wrapping_add(offset) as indx_t;
                            i = i.wrapping_add(1);
                        }
                    }
                }
                rdata = &mut xdata;
                flags |= 4 as libc::c_uint;
                do_sub = 1 as libc::c_int;
                if insert_key == 0 {
                    mdb_node_del(mc, 0 as libc::c_int);
                }
                current_block = 11950936639099900136;
            }
            _ => {}
        }
        match current_block {
            11950936639099900136 => {
                nflags = flags & 196614 as libc::c_uint;
                if (*(*mc).mc_pg[(*mc).mc_top as usize]).mp_flags as libc::c_int
                    & 32 as libc::c_int == 32 as libc::c_int
                {
                    nsize = (*key).mv_size;
                } else {
                    tmp___4 = mdb_leaf_size(env, key, rdata);
                    nsize = tmp___4;
                }
                if (((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_pb.pb.pb_upper
                    as libc::c_int
                    - (*(*mc).mc_pg[(*mc).mc_top as usize]).mp_pb.pb.pb_lower
                        as libc::c_int) as indx_t as size_t) < nsize
                {
                    if flags & 6 as libc::c_uint == 4 as libc::c_uint {
                        nflags &= 4294836223 as libc::c_uint;
                    }
                    if insert_key == 0 {
                        nflags |= 262144 as libc::c_uint;
                    }
                    rc = mdb_page_split(
                        mc,
                        key,
                        rdata,
                        !(0 as libc::c_int as pgno_t),
                        nflags,
                    );
                } else {
                    rc = mdb_node_add(
                        mc,
                        (*mc).mc_ki[(*mc).mc_top as usize],
                        key,
                        rdata,
                        0 as libc::c_int as pgno_t,
                        nflags,
                    );
                    if rc == 0 as libc::c_int {
                        dbi = (*mc).mc_dbi;
                        i___0 = (*mc).mc_top as libc::c_uint;
                        mp___0 = (*mc).mc_pg[i___0 as usize];
                        m2 = *((*(*mc).mc_txn).mt_cursors).offset(dbi as isize);
                        while !m2.is_null() {
                            if (*mc).mc_flags & 4 as libc::c_uint != 0 {
                                m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
                            } else {
                                m3 = m2;
                            }
                            if !(m3 as libc::c_ulong == mc as libc::c_ulong) {
                                if !(((*m3).mc_snum as libc::c_int)
                                    < (*mc).mc_snum as libc::c_int)
                                {
                                    if !((*m3).mc_pg[i___0 as usize] as libc::c_ulong
                                        != mp___0 as libc::c_ulong)
                                    {
                                        if (*m3).mc_ki[i___0 as usize] as libc::c_int
                                            >= (*mc).mc_ki[i___0 as usize] as libc::c_int
                                        {
                                            if insert_key != 0 {
                                                (*m3)
                                                    .mc_ki[i___0
                                                    as usize] = ((*m3).mc_ki[i___0 as usize] as libc::c_int
                                                    + 1 as libc::c_int) as indx_t;
                                            }
                                        }
                                        xr_pg = mp___0;
                                        if !((*m3).mc_xcursor).is_null() {
                                            if (*(*m3).mc_xcursor).mx_cursor.mc_flags
                                                & 1 as libc::c_uint != 0
                                            {
                                                if !((*m3).mc_ki[i___0 as usize] as libc::c_uint
                                                    >= ((*xr_pg).mp_pb.pb.pb_lower as libc::c_uint)
                                                        .wrapping_sub(
                                                            &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                                                as libc::c_ulong as libc::c_uint,
                                                        ) >> 1 as libc::c_int)
                                                {
                                                    xr_node = (xr_pg as *mut libc::c_char)
                                                        .offset(
                                                            *((*xr_pg).mp_ptrs)
                                                                .as_mut_ptr()
                                                                .offset((*m3).mc_ki[i___0 as usize] as isize) as libc::c_int
                                                                as isize,
                                                        )
                                                        .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                                                    if (*xr_node).mn_flags as libc::c_int & 6 as libc::c_int
                                                        == 4 as libc::c_int
                                                    {
                                                        (*(*m3).mc_xcursor)
                                                            .mx_cursor
                                                            .mc_pg[0 as libc::c_int
                                                            as usize] = ((*xr_node).mn_data)
                                                            .as_mut_ptr()
                                                            .offset((*xr_node).mn_ksize as libc::c_int as isize)
                                                            as *mut libc::c_void as *mut MDB_page;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            m2 = (*m2).mc_next;
                        }
                    }
                }
                if !(rc == 0 as libc::c_int) {
                    current_block = 2365107296380899847;
                    break;
                }
                if do_sub != 0 {
                    current_block = 7606014630461122334;
                } else {
                    current_block = 7792909578691485565;
                }
            }
            _ => {}
        }
        match current_block {
            7606014630461122334 => {
                xdata.mv_size = 0 as libc::c_int as size_t;
                xdata
                    .mv_data = b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_void;
                leaf = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
                    .offset(
                        *((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_ptrs)
                            .as_mut_ptr()
                            .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                            as libc::c_int as isize,
                    )
                    .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                if flags & 262208 as libc::c_uint == 64 as libc::c_uint {
                    xflags = 32832 as libc::c_int;
                } else {
                    mdb_xcursor_init1(mc, leaf);
                    if flags & 32 as libc::c_uint != 0 {
                        xflags = 32784 as libc::c_int;
                    } else {
                        xflags = 32768 as libc::c_int;
                    }
                }
                if !sub_root.is_null() {
                    (*(*mc).mc_xcursor)
                        .mx_cursor
                        .mc_pg[0 as libc::c_int as usize] = sub_root;
                }
                new_dupdata = dkey.mv_size as libc::c_int;
                if dkey.mv_size != 0 {
                    rc = mdb_cursor_put(
                        &mut (*(*mc).mc_xcursor).mx_cursor,
                        &mut dkey,
                        &mut xdata,
                        xflags as libc::c_uint,
                    );
                    if rc != 0 {
                        current_block = 12739769678470784980;
                        break;
                    }
                    dkey.mv_size = 0 as libc::c_int as size_t;
                }
                if (*leaf).mn_flags as libc::c_int & 2 as libc::c_int == 0 {
                    current_block = 572908253397452749;
                } else if !sub_root.is_null() {
                    current_block = 572908253397452749;
                } else {
                    current_block = 2115636134784596892;
                }
                match current_block {
                    572908253397452749 => {
                        mx = (*mc).mc_xcursor;
                        i___1 = (*mc).mc_top as libc::c_uint;
                        mp___1 = (*mc).mc_pg[i___1 as usize];
                        m2___0 = *((*(*mc).mc_txn).mt_cursors)
                            .offset((*mc).mc_dbi as isize);
                        while !m2___0.is_null() {
                            if !(m2___0 as libc::c_ulong == mc as libc::c_ulong) {
                                if !(((*m2___0).mc_snum as libc::c_int)
                                    < (*mc).mc_snum as libc::c_int)
                                {
                                    if !((*m2___0).mc_flags & 1 as libc::c_uint == 0) {
                                        if (*m2___0).mc_pg[i___1 as usize] as libc::c_ulong
                                            == mp___1 as libc::c_ulong
                                        {
                                            if (*m2___0).mc_ki[i___1 as usize] as libc::c_int
                                                == (*mc).mc_ki[i___1 as usize] as libc::c_int
                                            {
                                                mdb_xcursor_init2(m2___0, mx, new_dupdata);
                                            } else if insert_key == 0 {
                                                xr_pg___0 = mp___1;
                                                if !((*m2___0).mc_xcursor).is_null() {
                                                    if (*(*m2___0).mc_xcursor).mx_cursor.mc_flags
                                                        & 1 as libc::c_uint != 0
                                                    {
                                                        if !((*m2___0).mc_ki[i___1 as usize] as libc::c_uint
                                                            >= ((*xr_pg___0).mp_pb.pb.pb_lower as libc::c_uint)
                                                                .wrapping_sub(
                                                                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                                                        as libc::c_ulong as libc::c_uint,
                                                                ) >> 1 as libc::c_int)
                                                        {
                                                            xr_node___0 = (xr_pg___0 as *mut libc::c_char)
                                                                .offset(
                                                                    *((*xr_pg___0).mp_ptrs)
                                                                        .as_mut_ptr()
                                                                        .offset((*m2___0).mc_ki[i___1 as usize] as isize)
                                                                        as libc::c_int as isize,
                                                                )
                                                                .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                                                            if (*xr_node___0).mn_flags as libc::c_int & 6 as libc::c_int
                                                                == 4 as libc::c_int
                                                            {
                                                                (*(*m2___0).mc_xcursor)
                                                                    .mx_cursor
                                                                    .mc_pg[0 as libc::c_int
                                                                    as usize] = ((*xr_node___0).mn_data)
                                                                    .as_mut_ptr()
                                                                    .offset((*xr_node___0).mn_ksize as libc::c_int as isize)
                                                                    as *mut libc::c_void as *mut MDB_page;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            m2___0 = (*m2___0).mc_next;
                        }
                    }
                    _ => {}
                }
                ecount = (*(*mc).mc_xcursor).mx_db.md_entries;
                if flags & 262144 as libc::c_uint != 0 {
                    xflags |= 131072 as libc::c_int;
                }
                rc = mdb_cursor_put(
                    &mut (*(*mc).mc_xcursor).mx_cursor,
                    data,
                    &mut xdata,
                    xflags as libc::c_uint,
                );
                if flags & 2 as libc::c_uint != 0 {
                    db = ((*leaf).mn_data)
                        .as_mut_ptr()
                        .offset((*leaf).mn_ksize as libc::c_int as isize)
                        as *mut libc::c_void;
                    memcpy(
                        db,
                        &mut (*(*mc).mc_xcursor).mx_db as *mut MDB_db
                            as *const libc::c_void,
                        ::std::mem::size_of::<MDB_db>() as libc::c_ulong,
                    );
                }
                insert_data = ((*(*mc).mc_xcursor).mx_db.md_entries).wrapping_sub(ecount)
                    as libc::c_int;
            }
            _ => {}
        }
        if insert_data != 0 {
            (*(*mc).mc_db).md_entries = ((*(*mc).mc_db).md_entries).wrapping_add(1);
        }
        if insert_key != 0 {
            if rc != 0 {
                current_block = 12739769678470784980;
                break;
            }
            (*mc).mc_flags |= 1 as libc::c_uint;
        }
        if !(flags & 524288 as libc::c_uint != 0) {
            current_block = 7840998185605118132;
            break;
        }
        if !(rc == 0) {
            current_block = 7840998185605118132;
            break;
        }
        mcount = mcount.wrapping_add(1);
        (*data.offset(1 as libc::c_int as isize)).mv_size = mcount as size_t;
        if !(mcount < dcount) {
            current_block = 7840998185605118132;
            break;
        }
        let ref mut fresh17 = (*data.offset(0 as libc::c_int as isize)).mv_data;
        *fresh17 = ((*data.offset(0 as libc::c_int as isize)).mv_data
            as *mut libc::c_char)
            .offset((*data.offset(0 as libc::c_int as isize)).mv_size as isize)
            as *mut libc::c_void;
        insert_data = 0 as libc::c_int;
        insert_key = insert_data;
        current_block = 9768578057025712957;
    }
    match current_block {
        7840998185605118132 => return rc,
        12739769678470784980 => {
            if rc == -(30799 as libc::c_int) {
                rc = -(30779 as libc::c_int);
            }
        }
        5323028055506826224 => return 0 as libc::c_int,
        _ => {}
    }
    (*(*mc).mc_txn).mt_flags |= 2 as libc::c_uint;
    return rc;
}
pub unsafe extern "C" fn mdb_cursor_del(
    mut mc: *mut MDB_cursor,
    mut flags: libc::c_uint,
) -> libc::c_int {
    let mut current_block: u64;
    let mut leaf: *mut MDB_node = 0 as *mut MDB_node;
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut rc: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut db: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut m2: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut xr_pg: *mut MDB_page = 0 as *mut MDB_page;
    let mut xr_node: *mut MDB_node = 0 as *mut MDB_node;
    let mut omp: *mut MDB_page = 0 as *mut MDB_page;
    let mut pg: pgno_t = 0;
    let mut tmp___0: libc::c_int = 0;
    if (*(*mc).mc_txn).mt_flags & 131091 as libc::c_uint != 0 {
        if (*(*mc).mc_txn).mt_flags & 131072 as libc::c_uint != 0 {
            tmp = 13 as libc::c_int;
        } else {
            tmp = -(30782 as libc::c_int);
        }
        return tmp;
    }
    if (*mc).mc_flags & 1 as libc::c_uint == 0 {
        return 22 as libc::c_int;
    }
    if (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint
        >= ((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_pb.pb.pb_lower as libc::c_uint)
            .wrapping_sub(
                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                    as libc::c_uint,
            ) >> 1 as libc::c_int
    {
        return -(30798 as libc::c_int);
    }
    if flags & 32768 as libc::c_uint == 0 {
        rc = mdb_page_spill(
            mc,
            0 as *mut libc::c_void as *mut MDB_val,
            0 as *mut libc::c_void as *mut MDB_val,
        );
        if rc != 0 {
            return rc;
        }
    }
    rc = mdb_cursor_touch(mc);
    if rc != 0 {
        return rc;
    }
    mp = (*mc).mc_pg[(*mc).mc_top as usize];
    if !((*mp).mp_flags as libc::c_int & 2 as libc::c_int == 2 as libc::c_int) {
        return -(30796 as libc::c_int);
    }
    if !((*mp).mp_flags as libc::c_int & 32 as libc::c_int == 32 as libc::c_int) {
        leaf = (mp as *mut libc::c_char)
            .offset(
                *((*mp).mp_ptrs)
                    .as_mut_ptr()
                    .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize) as libc::c_int
                    as isize,
            )
            .offset(0 as libc::c_uint as isize) as *mut MDB_node;
        if (*leaf).mn_flags as libc::c_int & 4 as libc::c_int == 4 as libc::c_int {
            if flags & 32 as libc::c_uint != 0 {
                (*(*mc).mc_db)
                    .md_entries = ((*(*mc).mc_db).md_entries as libc::c_ulong)
                    .wrapping_sub(
                        ((*(*mc).mc_xcursor).mx_db.md_entries)
                            .wrapping_sub(1 as libc::c_ulong),
                    ) as mdb_size_t as mdb_size_t;
                (*(*mc).mc_xcursor).mx_cursor.mc_flags &= 4294967294 as libc::c_uint;
            } else {
                if !((*leaf).mn_flags as libc::c_int & 2 as libc::c_int
                    == 2 as libc::c_int)
                {
                    (*(*mc).mc_xcursor)
                        .mx_cursor
                        .mc_pg[0 as libc::c_int
                        as usize] = ((*leaf).mn_data)
                        .as_mut_ptr()
                        .offset((*leaf).mn_ksize as libc::c_int as isize)
                        as *mut libc::c_void as *mut MDB_page;
                }
                rc = mdb_cursor_del(
                    &mut (*(*mc).mc_xcursor).mx_cursor,
                    32768 as libc::c_uint,
                );
                if rc != 0 {
                    return rc;
                }
                if (*(*mc).mc_xcursor).mx_db.md_entries != 0 {
                    if (*leaf).mn_flags as libc::c_int & 2 as libc::c_int != 0 {
                        db = ((*leaf).mn_data)
                            .as_mut_ptr()
                            .offset((*leaf).mn_ksize as libc::c_int as isize)
                            as *mut libc::c_void;
                        memcpy(
                            db,
                            &mut (*(*mc).mc_xcursor).mx_db as *mut MDB_db
                                as *const libc::c_void,
                            ::std::mem::size_of::<MDB_db>() as libc::c_ulong,
                        );
                    } else {
                        mdb_node_shrink(mp, (*mc).mc_ki[(*mc).mc_top as usize]);
                        leaf = (mp as *mut libc::c_char)
                            .offset(
                                *((*mp).mp_ptrs)
                                    .as_mut_ptr()
                                    .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                                    as libc::c_int as isize,
                            )
                            .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                        (*(*mc).mc_xcursor)
                            .mx_cursor
                            .mc_pg[0 as libc::c_int
                            as usize] = ((*leaf).mn_data)
                            .as_mut_ptr()
                            .offset((*leaf).mn_ksize as libc::c_int as isize)
                            as *mut libc::c_void as *mut MDB_page;
                        m2 = *((*(*mc).mc_txn).mt_cursors).offset((*mc).mc_dbi as isize);
                        while !m2.is_null() {
                            if !(m2 as libc::c_ulong == mc as libc::c_ulong) {
                                if !(((*m2).mc_snum as libc::c_int)
                                    < (*mc).mc_snum as libc::c_int)
                                {
                                    if !((*m2).mc_flags & 1 as libc::c_uint == 0) {
                                        if (*m2).mc_pg[(*mc).mc_top as usize] as libc::c_ulong
                                            == mp as libc::c_ulong
                                        {
                                            xr_pg = mp;
                                            if !((*m2).mc_xcursor).is_null() {
                                                if (*(*m2).mc_xcursor).mx_cursor.mc_flags
                                                    & 1 as libc::c_uint != 0
                                                {
                                                    if !((*m2).mc_ki[(*mc).mc_top as usize] as libc::c_uint
                                                        >= ((*xr_pg).mp_pb.pb.pb_lower as libc::c_uint)
                                                            .wrapping_sub(
                                                                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                                                    as libc::c_ulong as libc::c_uint,
                                                            ) >> 1 as libc::c_int)
                                                    {
                                                        xr_node = (xr_pg as *mut libc::c_char)
                                                            .offset(
                                                                *((*xr_pg).mp_ptrs)
                                                                    .as_mut_ptr()
                                                                    .offset((*m2).mc_ki[(*mc).mc_top as usize] as isize)
                                                                    as libc::c_int as isize,
                                                            )
                                                            .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                                                        if (*xr_node).mn_flags as libc::c_int & 6 as libc::c_int
                                                            == 4 as libc::c_int
                                                        {
                                                            (*(*m2).mc_xcursor)
                                                                .mx_cursor
                                                                .mc_pg[0 as libc::c_int
                                                                as usize] = ((*xr_node).mn_data)
                                                                .as_mut_ptr()
                                                                .offset((*xr_node).mn_ksize as libc::c_int as isize)
                                                                as *mut libc::c_void as *mut MDB_page;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            m2 = (*m2).mc_next;
                        }
                    }
                    (*(*mc).mc_db)
                        .md_entries = ((*(*mc).mc_db).md_entries).wrapping_sub(1);
                    return rc;
                } else {
                    (*(*mc).mc_xcursor).mx_cursor.mc_flags &= 4294967294 as libc::c_uint;
                }
            }
            if (*leaf).mn_flags as libc::c_int & 2 as libc::c_int != 0 {
                rc = mdb_drop0(&mut (*(*mc).mc_xcursor).mx_cursor, 0 as libc::c_int);
                if rc != 0 {
                    current_block = 18015419125629982070;
                } else {
                    current_block = 5684854171168229155;
                }
            } else {
                current_block = 5684854171168229155;
            }
        } else if ((*leaf).mn_flags as libc::c_uint ^ flags) & 2 as libc::c_uint != 0 {
            rc = -(30784 as libc::c_int);
            current_block = 18015419125629982070;
        } else {
            current_block = 5684854171168229155;
        }
        match current_block {
            5684854171168229155 => {
                if (*leaf).mn_flags as libc::c_int & 1 as libc::c_int == 1 as libc::c_int
                {
                    memcpy(
                        &mut pg as *mut pgno_t as *mut libc::c_void,
                        ((*leaf).mn_data)
                            .as_mut_ptr()
                            .offset((*leaf).mn_ksize as libc::c_int as isize)
                            as *mut libc::c_void as *const libc::c_void,
                        ::std::mem::size_of::<pgno_t>() as libc::c_ulong,
                    );
                    rc = mdb_page_get(
                        mc,
                        pg,
                        &mut omp,
                        0 as *mut libc::c_void as *mut libc::c_int,
                    );
                    if rc != 0 {
                        current_block = 18015419125629982070;
                    } else {
                        rc = mdb_ovpage_free(mc, omp);
                        if rc != 0 {
                            current_block = 18015419125629982070;
                        } else {
                            current_block = 16642386089830349773;
                        }
                    }
                } else {
                    current_block = 16642386089830349773;
                }
            }
            _ => {}
        }
        match current_block {
            16642386089830349773 => {}
            _ => {
                (*(*mc).mc_txn).mt_flags |= 2 as libc::c_uint;
                return rc;
            }
        }
    }
    tmp___0 = mdb_cursor_del0(mc);
    return tmp___0;
}
unsafe extern "C" fn mdb_page_new(
    mut mc: *mut MDB_cursor,
    mut flags: uint32_t,
    mut num: libc::c_int,
    mut mp: *mut *mut MDB_page,
) -> libc::c_int {
    let mut np: *mut MDB_page = 0 as *mut MDB_page;
    let mut rc: libc::c_int = 0;
    rc = mdb_page_alloc(mc, num, &mut np);
    if rc != 0 {
        return rc;
    }
    (*np).mp_flags = (flags | 16 as libc::c_uint) as uint16_t;
    (*np)
        .mp_pb
        .pb
        .pb_lower = &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
        as libc::c_ulong as libc::c_uint as indx_t;
    (*np).mp_pb.pb.pb_upper = (*(*(*mc).mc_txn).mt_env).me_psize as indx_t;
    if (*np).mp_flags as libc::c_int & 1 as libc::c_int == 1 as libc::c_int {
        (*(*mc).mc_db)
            .md_branch_pages = ((*(*mc).mc_db).md_branch_pages).wrapping_add(1);
    } else if (*np).mp_flags as libc::c_int & 2 as libc::c_int == 2 as libc::c_int {
        (*(*mc).mc_db).md_leaf_pages = ((*(*mc).mc_db).md_leaf_pages).wrapping_add(1);
    } else if (*np).mp_flags as libc::c_int & 4 as libc::c_int == 4 as libc::c_int {
        (*(*mc).mc_db)
            .md_overflow_pages = ((*(*mc).mc_db).md_overflow_pages as libc::c_ulong)
            .wrapping_add(num as pgno_t) as pgno_t as pgno_t;
        (*np).mp_pb.pb_pages = num as uint32_t;
    }
    *mp = np;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_leaf_size(
    mut env: *mut MDB_env,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
) -> size_t {
    let mut sz: size_t = 0;
    sz = (&mut (*(0 as *mut MDB_node)).mn_data as *mut [libc::c_char; 1]
        as libc::c_ulong)
        .wrapping_add((*key).mv_size)
        .wrapping_add((*data).mv_size);
    if sz > (*env).me_nodemax as size_t {
        sz = (sz as libc::c_ulong)
            .wrapping_sub(
                ((*data).mv_size)
                    .wrapping_sub(::std::mem::size_of::<pgno_t>() as libc::c_ulong),
            ) as size_t as size_t;
    }
    return (sz
        .wrapping_add(::std::mem::size_of::<indx_t>() as libc::c_ulong)
        .wrapping_add(1 as libc::c_ulong) as libc::c_ulonglong
        & 18446744073709551614 as libc::c_ulonglong) as size_t;
}
unsafe extern "C" fn mdb_branch_size(
    mut env: *mut MDB_env,
    mut key: *mut MDB_val,
) -> size_t {
    let mut sz: size_t = 0;
    let mut tmp: size_t = 0;
    if key as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp = 0 as libc::c_int as size_t;
    } else {
        tmp = (*key).mv_size;
    }
    sz = (&mut (*(0 as *mut MDB_node)).mn_data as *mut [libc::c_char; 1]
        as libc::c_ulong)
        .wrapping_add(tmp);
    return sz.wrapping_add(::std::mem::size_of::<indx_t>() as libc::c_ulong);
}
unsafe extern "C" fn mdb_node_add(
    mut mc: *mut MDB_cursor,
    mut indx: indx_t,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut pgno: pgno_t,
    mut flags: libc::c_uint,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_uint = 0;
    let mut node_size: size_t = 0;
    let mut room: ssize_t = 0;
    let mut ofs: indx_t = 0;
    let mut node: *mut MDB_node = 0 as *mut MDB_node;
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut ofp: *mut MDB_page = 0 as *mut MDB_page;
    let mut ndata: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ksize: libc::c_int = 0;
    let mut dif: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ovpages: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    node_size = &mut (*(0 as *mut MDB_node)).mn_data as *mut [libc::c_char; 1]
        as libc::c_ulong;
    mp = (*mc).mc_pg[(*mc).mc_top as usize];
    ofp = 0 as *mut libc::c_void as *mut MDB_page;
    if !((*mp).mp_pb.pb.pb_upper as libc::c_int
        >= (*mp).mp_pb.pb.pb_lower as libc::c_int)
    {
        mdb_assert_fail(
            (*(*mc).mc_txn).mt_env,
            b"mp->mp_upper >= mp->mp_lower\0" as *const u8 as *const libc::c_char,
            b"mdb_node_add\0" as *const u8 as *const libc::c_char,
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            8306 as libc::c_int,
        );
    }
    if (*mp).mp_flags as libc::c_int & 32 as libc::c_int == 32 as libc::c_int {
        ksize = (*(*mc).mc_db).md_pad as libc::c_int;
        ptr = (mp as *mut libc::c_char)
            .offset(
                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                    as libc::c_uint as isize,
            )
            .offset((indx as libc::c_int * ksize) as isize);
        dif = (((*mp).mp_pb.pb.pb_lower as libc::c_uint)
            .wrapping_sub(
                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                    as libc::c_uint,
            ) >> 1 as libc::c_int)
            .wrapping_sub(indx as libc::c_uint) as libc::c_int;
        if dif > 0 as libc::c_int {
            memmove(
                ptr.offset(ksize as isize) as *mut libc::c_void,
                ptr as *const libc::c_void,
                (dif * ksize) as size_t,
            );
        }
        memcpy(
            ptr as *mut libc::c_void,
            (*key).mv_data as *const libc::c_void,
            ksize as size_t,
        );
        (*mp)
            .mp_pb
            .pb
            .pb_lower = ((*mp).mp_pb.pb.pb_lower as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<indx_t>() as libc::c_ulong) as indx_t;
        (*mp)
            .mp_pb
            .pb
            .pb_upper = ((*mp).mp_pb.pb.pb_upper as libc::c_ulong)
            .wrapping_sub(
                (ksize as libc::c_ulong)
                    .wrapping_sub(::std::mem::size_of::<indx_t>() as libc::c_ulong),
            ) as indx_t;
        return 0 as libc::c_int;
    }
    room = ((*mp).mp_pb.pb.pb_upper as libc::c_int
        - (*mp).mp_pb.pb.pb_lower as libc::c_int) as indx_t as ssize_t
        - ::std::mem::size_of::<indx_t>() as libc::c_ulong as ssize_t;
    if key as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        node_size = (node_size as libc::c_ulong).wrapping_add((*key).mv_size) as size_t
            as size_t;
    }
    if (*mp).mp_flags as libc::c_int & 2 as libc::c_int == 2 as libc::c_int {
        if !key.is_null() {
            if data.is_null() {
                mdb_assert_fail(
                    (*(*mc).mc_txn).mt_env,
                    b"key && data\0" as *const u8 as *const libc::c_char,
                    b"mdb_node_add\0" as *const u8 as *const libc::c_char,
                    b"mdb.c\0" as *const u8 as *const libc::c_char,
                    8334 as libc::c_int,
                );
            }
        } else {
            mdb_assert_fail(
                (*(*mc).mc_txn).mt_env,
                b"key && data\0" as *const u8 as *const libc::c_char,
                b"mdb_node_add\0" as *const u8 as *const libc::c_char,
                b"mdb.c\0" as *const u8 as *const libc::c_char,
                8334 as libc::c_int,
            );
        }
        if flags & 1 as libc::c_uint == 1 as libc::c_uint {
            node_size = (node_size as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<pgno_t>() as libc::c_ulong) as size_t
                as size_t;
            current_block = 7746103178988627676;
        } else if node_size.wrapping_add((*data).mv_size)
                > (*(*(*mc).mc_txn).mt_env).me_nodemax as size_t
            {
            ovpages = ((&mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                as libc::c_ulong as libc::c_uint)
                .wrapping_sub(1 as libc::c_uint) as size_t)
                .wrapping_add((*data).mv_size)
                .wrapping_div((*(*(*mc).mc_txn).mt_env).me_psize as size_t)
                .wrapping_add(1 as libc::c_ulong) as libc::c_int;
            node_size = (node_size
                .wrapping_add(::std::mem::size_of::<pgno_t>() as libc::c_ulong)
                .wrapping_add(1 as libc::c_ulong) as libc::c_ulonglong
                & 18446744073709551614 as libc::c_ulonglong) as size_t;
            if node_size as ssize_t > room {
                current_block = 6524449426826752846;
            } else {
                rc = mdb_page_new(mc, 4 as libc::c_int as uint32_t, ovpages, &mut ofp);
                if rc != 0 {
                    return rc;
                }
                flags |= 1 as libc::c_uint;
                current_block = 4238926510240165287;
            }
        } else {
            node_size = (node_size as libc::c_ulong).wrapping_add((*data).mv_size)
                as size_t as size_t;
            current_block = 7746103178988627676;
        }
    } else {
        current_block = 7746103178988627676;
    }
    match current_block {
        7746103178988627676 => {
            node_size = (node_size.wrapping_add(1 as libc::c_ulong) as libc::c_ulonglong
                & 18446744073709551614 as libc::c_ulonglong) as size_t;
            if node_size as ssize_t > room {
                current_block = 6524449426826752846;
            } else {
                current_block = 4238926510240165287;
            }
        }
        _ => {}
    }
    match current_block {
        6524449426826752846 => {
            (*(*mc).mc_txn).mt_flags |= 2 as libc::c_uint;
            return -(30786 as libc::c_int);
        }
        _ => {
            i = ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                .wrapping_sub(
                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                        as libc::c_ulong as libc::c_uint,
                ) >> 1 as libc::c_int;
            while i > indx as libc::c_uint {
                *((*mp).mp_ptrs)
                    .as_mut_ptr()
                    .offset(
                        i as isize,
                    ) = *((*mp).mp_ptrs)
                    .as_mut_ptr()
                    .offset(i.wrapping_sub(1 as libc::c_uint) as isize);
                i = i.wrapping_sub(1);
            }
            ofs = ((*mp).mp_pb.pb.pb_upper as size_t).wrapping_sub(node_size) as indx_t;
            if !(ofs as libc::c_ulong
                >= ((*mp).mp_pb.pb.pb_lower as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<indx_t>() as libc::c_ulong))
            {
                mdb_assert_fail(
                    (*(*mc).mc_txn).mt_env,
                    b"ofs >= mp->mp_lower + sizeof(indx_t)\0" as *const u8
                        as *const libc::c_char,
                    b"mdb_node_add\0" as *const u8 as *const libc::c_char,
                    b"mdb.c\0" as *const u8 as *const libc::c_char,
                    8367 as libc::c_int,
                );
            }
            *((*mp).mp_ptrs).as_mut_ptr().offset(indx as isize) = ofs;
            (*mp).mp_pb.pb.pb_upper = ofs;
            (*mp)
                .mp_pb
                .pb
                .pb_lower = ((*mp).mp_pb.pb.pb_lower as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<indx_t>() as libc::c_ulong)
                as indx_t;
            node = (mp as *mut libc::c_char)
                .offset(
                    *((*mp).mp_ptrs).as_mut_ptr().offset(indx as isize) as libc::c_int
                        as isize,
                )
                .offset(0 as libc::c_uint as isize) as *mut MDB_node;
            if key as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                (*node).mn_ksize = 0 as libc::c_int as libc::c_ushort;
            } else {
                (*node).mn_ksize = (*key).mv_size as libc::c_ushort;
            }
            (*node).mn_flags = flags as libc::c_ushort;
            if (*mp).mp_flags as libc::c_int & 2 as libc::c_int == 2 as libc::c_int {
                (*node)
                    .mn_lo = ((*data).mv_size & 65535 as libc::c_ulong)
                    as libc::c_ushort;
                (*node).mn_hi = ((*data).mv_size >> 16 as libc::c_int) as libc::c_ushort;
            } else {
                (*node).mn_lo = (pgno & 65535 as libc::c_ulong) as libc::c_ushort;
                (*node).mn_hi = (pgno >> 16 as libc::c_int) as libc::c_ushort;
                (*node).mn_flags = (pgno >> 32 as libc::c_int) as libc::c_ushort;
            }
            if !key.is_null() {
                memcpy(
                    ((*node).mn_data).as_mut_ptr() as *mut libc::c_void,
                    (*key).mv_data as *const libc::c_void,
                    (*key).mv_size,
                );
            }
            if (*mp).mp_flags as libc::c_int & 2 as libc::c_int == 2 as libc::c_int {
                ndata = ((*node).mn_data)
                    .as_mut_ptr()
                    .offset((*node).mn_ksize as libc::c_int as isize)
                    as *mut libc::c_void;
                if ofp as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                    if flags & 1 as libc::c_uint == 1 as libc::c_uint {
                        memcpy(
                            ndata,
                            (*data).mv_data as *const libc::c_void,
                            ::std::mem::size_of::<pgno_t>() as libc::c_ulong,
                        );
                    } else if flags & 65536 as libc::c_uint == 65536 as libc::c_uint {
                        (*data).mv_data = ndata;
                    } else {
                        memcpy(
                            ndata,
                            (*data).mv_data as *const libc::c_void,
                            (*data).mv_size,
                        );
                    }
                } else {
                    memcpy(
                        ndata,
                        &mut (*ofp).mp_p.p_pgno as *mut pgno_t as *const libc::c_void,
                        ::std::mem::size_of::<pgno_t>() as libc::c_ulong,
                    );
                    ndata = (ofp as *mut libc::c_char)
                        .offset(
                            &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                as libc::c_ulong as libc::c_uint as isize,
                        ) as *mut libc::c_void;
                    if flags & 65536 as libc::c_uint == 65536 as libc::c_uint {
                        (*data).mv_data = ndata;
                    } else {
                        memcpy(
                            ndata,
                            (*data).mv_data as *const libc::c_void,
                            (*data).mv_size,
                        );
                    }
                }
            }
            return 0 as libc::c_int;
        }
    };
}
unsafe extern "C" fn mdb_node_del(mut mc: *mut MDB_cursor, mut ksize: libc::c_int) {
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut indx: indx_t = 0;
    let mut sz: libc::c_uint = 0;
    let mut i: indx_t = 0;
    let mut j: indx_t = 0;
    let mut numkeys: indx_t = 0;
    let mut ptr: indx_t = 0;
    let mut node: *mut MDB_node = 0 as *mut MDB_node;
    let mut base: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x: libc::c_int = 0;
    mp = (*mc).mc_pg[(*mc).mc_top as usize];
    indx = (*mc).mc_ki[(*mc).mc_top as usize];
    numkeys = (((*mp).mp_pb.pb.pb_lower as libc::c_uint)
        .wrapping_sub(
            &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                as libc::c_uint,
        ) >> 1 as libc::c_int) as indx_t;
    if !((indx as libc::c_int) < numkeys as libc::c_int) {
        mdb_assert_fail(
            (*(*mc).mc_txn).mt_env,
            b"indx < numkeys\0" as *const u8 as *const libc::c_char,
            b"mdb_node_del\0" as *const u8 as *const libc::c_char,
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            8432 as libc::c_int,
        );
    }
    if (*mp).mp_flags as libc::c_int & 32 as libc::c_int == 32 as libc::c_int {
        x = numkeys as libc::c_int - 1 as libc::c_int - indx as libc::c_int;
        base = (mp as *mut libc::c_char)
            .offset(
                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                    as libc::c_uint as isize,
            )
            .offset((indx as libc::c_int * ksize) as isize);
        if x != 0 {
            memmove(
                base as *mut libc::c_void,
                base.offset(ksize as isize) as *const libc::c_void,
                (x * ksize) as size_t,
            );
        }
        (*mp)
            .mp_pb
            .pb
            .pb_lower = ((*mp).mp_pb.pb.pb_lower as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<indx_t>() as libc::c_ulong) as indx_t;
        (*mp)
            .mp_pb
            .pb
            .pb_upper = ((*mp).mp_pb.pb.pb_upper as libc::c_ulong)
            .wrapping_add(
                (ksize as libc::c_ulong)
                    .wrapping_sub(::std::mem::size_of::<indx_t>() as libc::c_ulong),
            ) as indx_t;
        return;
    }
    node = (mp as *mut libc::c_char)
        .offset(
            *((*mp).mp_ptrs).as_mut_ptr().offset(indx as isize) as libc::c_int as isize,
        )
        .offset(0 as libc::c_uint as isize) as *mut MDB_node;
    sz = (&mut (*(0 as *mut MDB_node)).mn_data as *mut [libc::c_char; 1]
        as libc::c_ulong)
        .wrapping_add((*node).mn_ksize as libc::c_ulong) as libc::c_uint;
    if (*mp).mp_flags as libc::c_int & 2 as libc::c_int == 2 as libc::c_int {
        if (*node).mn_flags as libc::c_int & 1 as libc::c_int == 1 as libc::c_int {
            sz = (sz as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<pgno_t>() as libc::c_ulong)
                as libc::c_uint;
        } else {
            sz = sz
                .wrapping_add(
                    (*node).mn_lo as libc::c_uint
                        | ((*node).mn_hi as libc::c_uint) << 16 as libc::c_int,
                );
        }
    }
    sz = sz.wrapping_add(1 as libc::c_uint) & 4294967294 as libc::c_uint;
    ptr = *((*mp).mp_ptrs).as_mut_ptr().offset(indx as isize);
    j = 0 as libc::c_int as indx_t;
    i = j;
    while (i as libc::c_int) < numkeys as libc::c_int {
        if i as libc::c_int != indx as libc::c_int {
            *((*mp).mp_ptrs)
                .as_mut_ptr()
                .offset(j as isize) = *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize);
            if (*((*mp).mp_ptrs).as_mut_ptr().offset(i as isize) as libc::c_int)
                < ptr as libc::c_int
            {
                *((*mp).mp_ptrs)
                    .as_mut_ptr()
                    .offset(
                        j as isize,
                    ) = (*((*mp).mp_ptrs).as_mut_ptr().offset(j as isize)
                    as libc::c_uint)
                    .wrapping_add(sz) as indx_t;
            }
            j = (j as libc::c_int + 1 as libc::c_int) as indx_t;
        }
        i = (i as libc::c_int + 1 as libc::c_int) as indx_t;
    }
    base = (mp as *mut libc::c_char)
        .offset((*mp).mp_pb.pb.pb_upper as libc::c_int as isize)
        .offset(0 as libc::c_uint as isize);
    memmove(
        base.offset(sz as isize) as *mut libc::c_void,
        base as *const libc::c_void,
        (ptr as libc::c_int - (*mp).mp_pb.pb.pb_upper as libc::c_int) as size_t,
    );
    (*mp)
        .mp_pb
        .pb
        .pb_lower = ((*mp).mp_pb.pb.pb_lower as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<indx_t>() as libc::c_ulong) as indx_t;
    (*mp)
        .mp_pb
        .pb
        .pb_upper = ((*mp).mp_pb.pb.pb_upper as libc::c_uint).wrapping_add(sz) as indx_t;
}
unsafe extern "C" fn mdb_node_shrink(mut mp: *mut MDB_page, mut indx: indx_t) {
    let mut node: *mut MDB_node = 0 as *mut MDB_node;
    let mut sp: *mut MDB_page = 0 as *mut MDB_page;
    let mut xp: *mut MDB_page = 0 as *mut MDB_page;
    let mut base: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut delta: indx_t = 0;
    let mut nsize: indx_t = 0;
    let mut len: indx_t = 0;
    let mut ptr: indx_t = 0;
    let mut i: libc::c_int = 0;
    node = (mp as *mut libc::c_char)
        .offset(
            *((*mp).mp_ptrs).as_mut_ptr().offset(indx as isize) as libc::c_int as isize,
        )
        .offset(0 as libc::c_uint as isize) as *mut MDB_node;
    sp = ((*node).mn_data).as_mut_ptr().offset((*node).mn_ksize as libc::c_int as isize)
        as *mut libc::c_void as *mut MDB_page;
    delta = ((*sp).mp_pb.pb.pb_upper as libc::c_int
        - (*sp).mp_pb.pb.pb_lower as libc::c_int) as indx_t;
    nsize = ((*node).mn_lo as libc::c_uint
        | ((*node).mn_hi as libc::c_uint) << 16 as libc::c_int)
        .wrapping_sub(delta as libc::c_uint) as indx_t;
    if (*sp).mp_flags as libc::c_int & 32 as libc::c_int == 32 as libc::c_int {
        len = nsize;
        if nsize as libc::c_int & 1 as libc::c_int != 0 {
            return;
        }
    } else {
        xp = (sp as *mut libc::c_char).offset(delta as libc::c_int as isize)
            as *mut MDB_page;
        i = (((*sp).mp_pb.pb.pb_lower as libc::c_uint)
            .wrapping_sub(
                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                    as libc::c_uint,
            ) >> 1 as libc::c_int) as libc::c_int;
        loop {
            i -= 1;
            if !(i >= 0 as libc::c_int) {
                break;
            }
            *((*xp).mp_ptrs)
                .as_mut_ptr()
                .offset(
                    i as isize,
                ) = (*((*sp).mp_ptrs).as_mut_ptr().offset(i as isize) as libc::c_int
                - delta as libc::c_int) as indx_t;
        }
        len = &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
            as libc::c_uint as indx_t;
    }
    (*sp).mp_pb.pb.pb_upper = (*sp).mp_pb.pb.pb_lower;
    (*sp).mp_p.p_pgno = (*mp).mp_p.p_pgno;
    (*node).mn_lo = (nsize as libc::c_int & 65535 as libc::c_int) as libc::c_ushort;
    (*node).mn_hi = (nsize as libc::c_int >> 16 as libc::c_int) as libc::c_ushort;
    base = (mp as *mut libc::c_char)
        .offset((*mp).mp_pb.pb.pb_upper as libc::c_int as isize)
        .offset(0 as libc::c_uint as isize);
    memmove(
        base.offset(delta as libc::c_int as isize) as *mut libc::c_void,
        base as *const libc::c_void,
        (sp as *mut libc::c_char).offset(len as libc::c_int as isize).offset_from(base)
            as libc::c_long as size_t,
    );
    ptr = *((*mp).mp_ptrs).as_mut_ptr().offset(indx as isize);
    i = (((*mp).mp_pb.pb.pb_lower as libc::c_uint)
        .wrapping_sub(
            &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                as libc::c_uint,
        ) >> 1 as libc::c_int) as libc::c_int;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        if *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize) as libc::c_int
            <= ptr as libc::c_int
        {
            *((*mp).mp_ptrs)
                .as_mut_ptr()
                .offset(
                    i as isize,
                ) = (*((*mp).mp_ptrs).as_mut_ptr().offset(i as isize) as libc::c_int
                + delta as libc::c_int) as indx_t;
        }
    }
    (*mp)
        .mp_pb
        .pb
        .pb_upper = ((*mp).mp_pb.pb.pb_upper as libc::c_int + delta as libc::c_int)
        as indx_t;
}
unsafe extern "C" fn mdb_xcursor_init0(mut mc: *mut MDB_cursor) {
    let mut mx: *mut MDB_xcursor = 0 as *mut MDB_xcursor;
    mx = (*mc).mc_xcursor;
    (*mx).mx_cursor.mc_xcursor = 0 as *mut libc::c_void as *mut MDB_xcursor;
    (*mx).mx_cursor.mc_txn = (*mc).mc_txn;
    (*mx).mx_cursor.mc_db = &mut (*mx).mx_db;
    (*mx).mx_cursor.mc_dbx = &mut (*mx).mx_dbx;
    (*mx).mx_cursor.mc_dbi = (*mc).mc_dbi;
    (*mx).mx_cursor.mc_dbflag = &mut (*mx).mx_dbflag;
    (*mx).mx_cursor.mc_snum = 0 as libc::c_int as libc::c_ushort;
    (*mx).mx_cursor.mc_top = 0 as libc::c_int as libc::c_ushort;
    (*mx)
        .mx_cursor
        .mc_flags = 4 as libc::c_uint | (*mc).mc_flags & 655360 as libc::c_uint;
    (*mx).mx_dbx.md_name.mv_size = 0 as libc::c_int as size_t;
    (*mx).mx_dbx.md_name.mv_data = 0 as *mut libc::c_void;
    (*mx).mx_dbx.md_cmp = (*(*mc).mc_dbx).md_dcmp;
    (*mx)
        .mx_dbx
        .md_dcmp = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<MDB_cmp_func>,
    >(0 as *mut libc::c_void);
    (*mx).mx_dbx.md_rel = (*(*mc).mc_dbx).md_rel;
}
unsafe extern "C" fn mdb_xcursor_init1(
    mut mc: *mut MDB_cursor,
    mut node: *mut MDB_node,
) {
    let mut mx: *mut MDB_xcursor = 0 as *mut MDB_xcursor;
    let mut fp: *mut MDB_page = 0 as *mut MDB_page;
    mx = (*mc).mc_xcursor;
    (*mx).mx_cursor.mc_flags &= 655364 as libc::c_uint;
    if (*node).mn_flags as libc::c_int & 2 as libc::c_int != 0 {
        memcpy(
            &mut (*mx).mx_db as *mut MDB_db as *mut libc::c_void,
            ((*node).mn_data)
                .as_mut_ptr()
                .offset((*node).mn_ksize as libc::c_int as isize) as *mut libc::c_void
                as *const libc::c_void,
            ::std::mem::size_of::<MDB_db>() as libc::c_ulong,
        );
        (*mx).mx_cursor.mc_pg[0 as libc::c_int as usize] = 0 as *mut MDB_page;
        (*mx).mx_cursor.mc_snum = 0 as libc::c_int as libc::c_ushort;
        (*mx).mx_cursor.mc_top = 0 as libc::c_int as libc::c_ushort;
    } else {
        fp = ((*node).mn_data)
            .as_mut_ptr()
            .offset((*node).mn_ksize as libc::c_int as isize) as *mut libc::c_void
            as *mut MDB_page;
        (*mx).mx_db.md_pad = 0 as libc::c_int as uint32_t;
        (*mx).mx_db.md_flags = 0 as libc::c_int as uint16_t;
        (*mx).mx_db.md_depth = 1 as libc::c_int as uint16_t;
        (*mx).mx_db.md_branch_pages = 0 as libc::c_int as pgno_t;
        (*mx).mx_db.md_leaf_pages = 1 as libc::c_int as pgno_t;
        (*mx).mx_db.md_overflow_pages = 0 as libc::c_int as pgno_t;
        (*mx)
            .mx_db
            .md_entries = (((*fp).mp_pb.pb.pb_lower as libc::c_uint)
            .wrapping_sub(
                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                    as libc::c_uint,
            ) >> 1 as libc::c_int) as mdb_size_t;
        (*mx).mx_db.md_root = (*fp).mp_p.p_pgno;
        (*mx).mx_cursor.mc_snum = 1 as libc::c_int as libc::c_ushort;
        (*mx).mx_cursor.mc_top = 0 as libc::c_int as libc::c_ushort;
        (*mx).mx_cursor.mc_flags |= 1 as libc::c_uint;
        (*mx).mx_cursor.mc_pg[0 as libc::c_int as usize] = fp;
        (*mx).mx_cursor.mc_ki[0 as libc::c_int as usize] = 0 as libc::c_int as indx_t;
        if (*(*mc).mc_db).md_flags as libc::c_int & 16 as libc::c_int != 0 {
            (*mx).mx_db.md_flags = 16 as libc::c_int as uint16_t;
            (*mx).mx_db.md_pad = (*fp).mp_pad as uint32_t;
            if (*(*mc).mc_db).md_flags as libc::c_int & 32 as libc::c_int != 0 {
                (*mx)
                    .mx_db
                    .md_flags = ((*mx).mx_db.md_flags as libc::c_int | 8 as libc::c_int)
                    as uint16_t;
            }
        }
    }
    (*mx).mx_dbflag = 56 as libc::c_int as libc::c_uchar;
    if ::std::mem::transmute::<
        Option::<MDB_cmp_func>,
        libc::c_ulong,
    >((*mx).mx_dbx.md_cmp)
        == ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*const MDB_val, *const MDB_val) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                mdb_cmp_int
                    as unsafe extern "C" fn(
                        *const MDB_val,
                        *const MDB_val,
                    ) -> libc::c_int,
            ),
        )
    {
        if (*mx).mx_db.md_pad as libc::c_ulong
            == ::std::mem::size_of::<mdb_size_t>() as libc::c_ulong
        {
            (*mx)
                .mx_dbx
                .md_cmp = Some(
                mdb_cmp_long
                    as unsafe extern "C" fn(
                        *const MDB_val,
                        *const MDB_val,
                    ) -> libc::c_int,
            );
        }
    }
}
unsafe extern "C" fn mdb_xcursor_init2(
    mut mc: *mut MDB_cursor,
    mut src_mx: *mut MDB_xcursor,
    mut new_dupdata: libc::c_int,
) {
    let mut mx: *mut MDB_xcursor = 0 as *mut MDB_xcursor;
    mx = (*mc).mc_xcursor;
    if new_dupdata != 0 {
        (*mx).mx_cursor.mc_snum = 1 as libc::c_int as libc::c_ushort;
        (*mx).mx_cursor.mc_top = 0 as libc::c_int as libc::c_ushort;
        (*mx).mx_cursor.mc_flags |= 1 as libc::c_uint;
        (*mx).mx_cursor.mc_ki[0 as libc::c_int as usize] = 0 as libc::c_int as indx_t;
        (*mx).mx_dbflag = 56 as libc::c_int as libc::c_uchar;
        (*mx).mx_dbx.md_cmp = (*src_mx).mx_dbx.md_cmp;
    } else if (*mx).mx_cursor.mc_flags & 1 as libc::c_uint == 0 {
        return
    }
    (*mx).mx_db = (*src_mx).mx_db;
    (*mx)
        .mx_cursor
        .mc_pg[0 as libc::c_int
        as usize] = (*src_mx).mx_cursor.mc_pg[0 as libc::c_int as usize];
}
unsafe extern "C" fn mdb_cursor_init(
    mut mc: *mut MDB_cursor,
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut mx: *mut MDB_xcursor,
) {
    (*mc).mc_next = 0 as *mut libc::c_void as *mut MDB_cursor;
    (*mc).mc_backup = 0 as *mut libc::c_void as *mut MDB_cursor;
    (*mc).mc_dbi = dbi;
    (*mc).mc_txn = txn;
    (*mc).mc_db = ((*txn).mt_dbs).offset(dbi as isize);
    (*mc).mc_dbx = ((*txn).mt_dbxs).offset(dbi as isize);
    (*mc).mc_dbflag = ((*txn).mt_dbflags).offset(dbi as isize);
    (*mc).mc_snum = 0 as libc::c_int as libc::c_ushort;
    (*mc).mc_top = 0 as libc::c_int as libc::c_ushort;
    (*mc).mc_pg[0 as libc::c_int as usize] = 0 as *mut MDB_page;
    (*mc).mc_ki[0 as libc::c_int as usize] = 0 as libc::c_int as indx_t;
    (*mc).mc_flags = (*txn).mt_flags & 655360 as libc::c_uint;
    if (*((*txn).mt_dbs).offset(dbi as isize)).md_flags as libc::c_int & 4 as libc::c_int
        != 0
    {
        if !(mx as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
            mdb_assert_fail(
                (*txn).mt_env,
                b"mx != NULL\0" as *const u8 as *const libc::c_char,
                b"mdb_cursor_init\0" as *const u8 as *const libc::c_char,
                b"mdb.c\0" as *const u8 as *const libc::c_char,
                8643 as libc::c_int,
            );
        }
        (*mc).mc_xcursor = mx;
        mdb_xcursor_init0(mc);
    } else {
        (*mc).mc_xcursor = 0 as *mut libc::c_void as *mut MDB_xcursor;
    }
    if *(*mc).mc_dbflag as libc::c_int & 2 as libc::c_int != 0 {
        mdb_page_search(mc, 0 as *mut libc::c_void as *mut MDB_val, 2 as libc::c_int);
    }
}
pub unsafe extern "C" fn mdb_cursor_open(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut ret: *mut *mut MDB_cursor,
) -> libc::c_int {
    let mut mc: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut size: size_t = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    size = ::std::mem::size_of::<MDB_cursor>() as libc::c_ulong;
    if ret.is_null() {
        return 22 as libc::c_int
    } else {
        if !txn.is_null() {
            if dbi < (*txn).mt_numdbs {
                if *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int
                    & 8 as libc::c_int == 0
                {
                    return 22 as libc::c_int;
                }
            } else {
                return 22 as libc::c_int
            }
        } else {
            return 22 as libc::c_int
        }
    }
    if (*txn).mt_flags & 19 as libc::c_uint != 0 {
        return -(30782 as libc::c_int);
    }
    if dbi == 0 as libc::c_uint {
        if !((*txn).mt_flags & 131072 as libc::c_uint == 131072 as libc::c_uint) {
            return 22 as libc::c_int;
        }
    }
    if (*((*txn).mt_dbs).offset(dbi as isize)).md_flags as libc::c_int & 4 as libc::c_int
        != 0
    {
        size = (size as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<MDB_xcursor>() as libc::c_ulong)
            as size_t as size_t;
    }
    tmp = malloc(size);
    mc = tmp as *mut MDB_cursor;
    if mc as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        mdb_cursor_init(
            mc,
            txn,
            dbi,
            mc.offset(1 as libc::c_int as isize) as *mut MDB_xcursor,
        );
        if !((*txn).mt_cursors).is_null() {
            (*mc).mc_next = *((*txn).mt_cursors).offset(dbi as isize);
            let ref mut fresh18 = *((*txn).mt_cursors).offset(dbi as isize);
            *fresh18 = mc;
            (*mc).mc_flags |= 64 as libc::c_uint;
        }
    } else {
        return 12 as libc::c_int
    }
    *ret = mc;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_cursor_renew(
    mut txn: *mut MDB_txn,
    mut mc: *mut MDB_cursor,
) -> libc::c_int {
    if mc.is_null() {
        return 22 as libc::c_int
    } else {
        if !txn.is_null() {
            if (*mc).mc_dbi < (*txn).mt_numdbs {
                if *((*txn).mt_dbflags).offset((*mc).mc_dbi as isize) as libc::c_int
                    & 8 as libc::c_int == 0
                {
                    return 22 as libc::c_int;
                }
            } else {
                return 22 as libc::c_int
            }
        } else {
            return 22 as libc::c_int
        }
    }
    if (*mc).mc_flags & 64 as libc::c_uint != 0 {
        return 22 as libc::c_int
    } else {
        if !((*txn).mt_cursors).is_null() {
            return 22 as libc::c_int;
        }
    }
    if (*txn).mt_flags & 19 as libc::c_uint != 0 {
        return -(30782 as libc::c_int);
    }
    mdb_cursor_init(mc, txn, (*mc).mc_dbi, (*mc).mc_xcursor);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_cursor_count(
    mut mc: *mut MDB_cursor,
    mut countp: *mut mdb_size_t,
) -> libc::c_int {
    let mut leaf: *mut MDB_node = 0 as *mut MDB_node;
    if mc as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 22 as libc::c_int
    } else {
        if countp as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            return 22 as libc::c_int;
        }
    }
    if (*mc).mc_xcursor as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(30784 as libc::c_int);
    }
    if (*(*mc).mc_txn).mt_flags & 19 as libc::c_uint != 0 {
        return -(30782 as libc::c_int);
    }
    if (*mc).mc_flags & 1 as libc::c_uint == 0 {
        return 22 as libc::c_int;
    }
    if (*mc).mc_snum == 0 {
        return -(30798 as libc::c_int);
    }
    if (*mc).mc_flags & 2 as libc::c_uint != 0 {
        if (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint
            >= ((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_pb.pb.pb_lower as libc::c_uint)
                .wrapping_sub(
                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                        as libc::c_ulong as libc::c_uint,
                ) >> 1 as libc::c_int
        {
            return -(30798 as libc::c_int);
        }
        (*mc).mc_flags ^= 2 as libc::c_uint;
    }
    leaf = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
        .offset(
            *((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_ptrs)
                .as_mut_ptr()
                .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize) as libc::c_int
                as isize,
        )
        .offset(0 as libc::c_uint as isize) as *mut MDB_node;
    if !((*leaf).mn_flags as libc::c_int & 4 as libc::c_int == 4 as libc::c_int) {
        *countp = 1 as libc::c_int as mdb_size_t;
    } else {
        if (*(*mc).mc_xcursor).mx_cursor.mc_flags & 1 as libc::c_uint == 0 {
            return 22 as libc::c_int;
        }
        *countp = (*(*mc).mc_xcursor).mx_db.md_entries;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_cursor_close(mut mc: *mut MDB_cursor) {
    let mut prev: *mut *mut MDB_cursor = 0 as *mut *mut MDB_cursor;
    if !mc.is_null() {
        if ((*mc).mc_backup).is_null() {
            if (*mc).mc_flags & 64 as libc::c_uint != 0 {
                if !((*(*mc).mc_txn).mt_cursors).is_null() {
                    prev = ((*(*mc).mc_txn).mt_cursors).offset((*mc).mc_dbi as isize);
                    while !(*prev).is_null() {
                        if !(*prev as libc::c_ulong != mc as libc::c_ulong) {
                            break;
                        }
                        prev = &mut (**prev).mc_next;
                    }
                    if *prev as libc::c_ulong == mc as libc::c_ulong {
                        *prev = (*mc).mc_next;
                    }
                }
            }
            free(mc as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn mdb_cursor_txn(mut mc: *mut MDB_cursor) -> *mut MDB_txn {
    if mc.is_null() {
        return 0 as *mut libc::c_void as *mut MDB_txn;
    }
    return (*mc).mc_txn;
}
pub unsafe extern "C" fn mdb_cursor_dbi(mut mc: *mut MDB_cursor) -> MDB_dbi {
    return (*mc).mc_dbi;
}
unsafe extern "C" fn mdb_update_key(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
) -> libc::c_int {
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut node: *mut MDB_node = 0 as *mut MDB_node;
    let mut base: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut delta: libc::c_int = 0;
    let mut ksize: libc::c_int = 0;
    let mut oksize: libc::c_int = 0;
    let mut ptr: indx_t = 0;
    let mut i: indx_t = 0;
    let mut numkeys: indx_t = 0;
    let mut indx: indx_t = 0;
    let mut pgno: pgno_t = 0;
    let mut tmp: libc::c_int = 0;
    indx = (*mc).mc_ki[(*mc).mc_top as usize];
    mp = (*mc).mc_pg[(*mc).mc_top as usize];
    node = (mp as *mut libc::c_char)
        .offset(
            *((*mp).mp_ptrs).as_mut_ptr().offset(indx as isize) as libc::c_int as isize,
        )
        .offset(0 as libc::c_uint as isize) as *mut MDB_node;
    ptr = *((*mp).mp_ptrs).as_mut_ptr().offset(indx as isize);
    ksize = (((*key).mv_size).wrapping_add(1 as libc::c_ulong) as libc::c_ulonglong
        & 18446744073709551614 as libc::c_ulonglong) as libc::c_int;
    oksize = (((*node).mn_ksize as libc::c_uint).wrapping_add(1 as libc::c_uint)
        & 4294967294 as libc::c_uint) as libc::c_int;
    delta = ksize - oksize;
    if delta != 0 {
        if delta > 0 as libc::c_int {
            if (((*mp).mp_pb.pb.pb_upper as libc::c_int
                - (*mp).mp_pb.pb.pb_lower as libc::c_int) as indx_t as libc::c_int)
                < delta
            {
                pgno = (*node).mn_lo as libc::c_ulong
                    | ((*node).mn_hi as pgno_t) << 16 as libc::c_int
                    | ((*node).mn_flags as pgno_t) << 32 as libc::c_int;
                mdb_node_del(mc, 0 as libc::c_int);
                tmp = mdb_page_split(
                    mc,
                    key,
                    0 as *mut libc::c_void as *mut MDB_val,
                    pgno,
                    262144 as libc::c_uint,
                );
                return tmp;
            }
        }
        numkeys = (((*mp).mp_pb.pb.pb_lower as libc::c_uint)
            .wrapping_sub(
                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                    as libc::c_uint,
            ) >> 1 as libc::c_int) as indx_t;
        i = 0 as libc::c_int as indx_t;
        while (i as libc::c_int) < numkeys as libc::c_int {
            if *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize) as libc::c_int
                <= ptr as libc::c_int
            {
                *((*mp).mp_ptrs)
                    .as_mut_ptr()
                    .offset(
                        i as isize,
                    ) = (*((*mp).mp_ptrs).as_mut_ptr().offset(i as isize) as libc::c_int
                    - delta) as indx_t;
            }
            i = (i as libc::c_int + 1 as libc::c_int) as indx_t;
        }
        base = (mp as *mut libc::c_char)
            .offset((*mp).mp_pb.pb.pb_upper as libc::c_int as isize)
            .offset(0 as libc::c_uint as isize);
        len = ((ptr as libc::c_int - (*mp).mp_pb.pb.pb_upper as libc::c_int)
            as libc::c_ulong)
            .wrapping_add(
                &mut (*(0 as *mut MDB_node)).mn_data as *mut [libc::c_char; 1]
                    as libc::c_ulong,
            );
        memmove(
            base.offset(-(delta as isize)) as *mut libc::c_void,
            base as *const libc::c_void,
            len,
        );
        (*mp)
            .mp_pb
            .pb
            .pb_upper = ((*mp).mp_pb.pb.pb_upper as libc::c_int - delta) as indx_t;
        node = (mp as *mut libc::c_char)
            .offset(
                *((*mp).mp_ptrs).as_mut_ptr().offset(indx as isize) as libc::c_int
                    as isize,
            )
            .offset(0 as libc::c_uint as isize) as *mut MDB_node;
    }
    if (*node).mn_ksize as size_t != (*key).mv_size {
        (*node).mn_ksize = (*key).mv_size as libc::c_ushort;
    }
    if (*key).mv_size != 0 {
        memcpy(
            ((*node).mn_data).as_mut_ptr() as *mut libc::c_void,
            (*key).mv_data as *const libc::c_void,
            (*key).mv_size,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_node_move(
    mut csrc: *mut MDB_cursor,
    mut cdst: *mut MDB_cursor,
    mut fromleft: libc::c_int,
) -> libc::c_int {
    let mut srcnode: *mut MDB_node = 0 as *mut MDB_node;
    let mut key: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut data: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut srcpg: pgno_t = 0;
    let mut mn: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut rc: libc::c_int = 0;
    let mut flags: libc::c_ushort = 0;
    let mut snum: libc::c_uint = 0;
    let mut s2: *mut MDB_node = 0 as *mut MDB_node;
    let mut tmp: libc::c_uint = 0;
    let mut snum___0: libc::c_uint = 0;
    let mut s2___0: *mut MDB_node = 0 as *mut MDB_node;
    let mut bkey: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut tmp___0: libc::c_uint = 0;
    let mut m2: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut m3: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut dbi: MDB_dbi = 0;
    let mut mpd: *mut MDB_page = 0 as *mut MDB_page;
    let mut mps: *mut MDB_page = 0 as *mut MDB_page;
    let mut xr_pg: *mut MDB_page = 0 as *mut MDB_page;
    let mut xr_node: *mut MDB_node = 0 as *mut MDB_node;
    let mut xr_pg___0: *mut MDB_page = 0 as *mut MDB_page;
    let mut xr_node___0: *mut MDB_node = 0 as *mut MDB_node;
    let mut dummy: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut tracked: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut tp: *mut *mut MDB_cursor = 0 as *mut *mut MDB_cursor;
    let mut nullkey: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut ix: indx_t = 0;
    let mut dummy___0: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut tracked___0: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut tp___0: *mut *mut MDB_cursor = 0 as *mut *mut MDB_cursor;
    let mut nullkey___0: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut ix___0: indx_t = 0;
    rc = mdb_page_touch(csrc);
    if rc != 0 {
        return rc
    } else {
        rc = mdb_page_touch(cdst);
        if rc != 0 {
            return rc;
        }
    }
    if (*(*csrc).mc_pg[(*csrc).mc_top as usize]).mp_flags as libc::c_int
        & 32 as libc::c_int == 32 as libc::c_int
    {
        key.mv_size = (*(*csrc).mc_db).md_pad as size_t;
        key
            .mv_data = ((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut libc::c_char)
            .offset(
                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                    as libc::c_uint as isize,
            )
            .offset(
                ((*csrc).mc_ki[(*csrc).mc_top as usize] as size_t)
                    .wrapping_mul(key.mv_size) as isize,
            ) as *mut libc::c_void;
        data.mv_size = 0 as libc::c_int as size_t;
        data.mv_data = 0 as *mut libc::c_void;
        srcpg = 0 as libc::c_int as pgno_t;
        flags = 0 as libc::c_int as libc::c_ushort;
    } else {
        srcnode = ((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut libc::c_char)
            .offset(
                *((*(*csrc).mc_pg[(*csrc).mc_top as usize]).mp_ptrs)
                    .as_mut_ptr()
                    .offset((*csrc).mc_ki[(*csrc).mc_top as usize] as isize)
                    as libc::c_int as isize,
            )
            .offset(0 as libc::c_uint as isize) as *mut MDB_node;
        if srcnode as size_t & 1 as libc::c_ulong != 0 {
            mdb_assert_fail(
                (*(*csrc).mc_txn).mt_env,
                b"!((size_t)srcnode & 1)\0" as *const u8 as *const libc::c_char,
                b"mdb_node_move\0" as *const u8 as *const libc::c_char,
                b"mdb.c\0" as *const u8 as *const libc::c_char,
                8899 as libc::c_int,
            );
        }
        srcpg = (*srcnode).mn_lo as libc::c_ulong
            | ((*srcnode).mn_hi as pgno_t) << 16 as libc::c_int
            | ((*srcnode).mn_flags as pgno_t) << 32 as libc::c_int;
        flags = (*srcnode).mn_flags;
        if (*csrc).mc_ki[(*csrc).mc_top as usize] as libc::c_int == 0 as libc::c_int {
            if (*(*csrc).mc_pg[(*csrc).mc_top as usize]).mp_flags as libc::c_int
                & 1 as libc::c_int == 1 as libc::c_int
            {
                snum = (*csrc).mc_snum as libc::c_uint;
                rc = mdb_page_search_lowest(csrc);
                if rc != 0 {
                    return rc;
                }
                if (*(*csrc).mc_pg[(*csrc).mc_top as usize]).mp_flags as libc::c_int
                    & 32 as libc::c_int == 32 as libc::c_int
                {
                    key.mv_size = (*(*csrc).mc_db).md_pad as size_t;
                    key
                        .mv_data = ((*csrc).mc_pg[(*csrc).mc_top as usize]
                        as *mut libc::c_char)
                        .offset(
                            &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                as libc::c_ulong as libc::c_uint as isize,
                        )
                        .offset(0 as libc::c_ulong as isize) as *mut libc::c_void;
                } else {
                    s2 = ((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut libc::c_char)
                        .offset(
                            *((*(*csrc).mc_pg[(*csrc).mc_top as usize]).mp_ptrs)
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize) as libc::c_int as isize,
                        )
                        .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                    key.mv_size = (*s2).mn_ksize as size_t;
                    key.mv_data = ((*s2).mn_data).as_mut_ptr() as *mut libc::c_void;
                }
                tmp = snum;
                snum = snum.wrapping_sub(1);
                (*csrc).mc_snum = tmp as libc::c_ushort;
                (*csrc).mc_top = snum as libc::c_ushort;
            } else {
                key.mv_size = (*srcnode).mn_ksize as size_t;
                key.mv_data = ((*srcnode).mn_data).as_mut_ptr() as *mut libc::c_void;
            }
        } else {
            key.mv_size = (*srcnode).mn_ksize as size_t;
            key.mv_data = ((*srcnode).mn_data).as_mut_ptr() as *mut libc::c_void;
        }
        data
            .mv_size = ((*srcnode).mn_lo as libc::c_uint
            | ((*srcnode).mn_hi as libc::c_uint) << 16 as libc::c_int) as size_t;
        data
            .mv_data = ((*srcnode).mn_data)
            .as_mut_ptr()
            .offset((*srcnode).mn_ksize as libc::c_int as isize) as *mut libc::c_void;
    }
    mn.mc_xcursor = 0 as *mut libc::c_void as *mut MDB_xcursor;
    if (*(*cdst).mc_pg[(*cdst).mc_top as usize]).mp_flags as libc::c_int
        & 1 as libc::c_int == 1 as libc::c_int
    {
        if (*cdst).mc_ki[(*cdst).mc_top as usize] as libc::c_int == 0 as libc::c_int {
            snum___0 = (*cdst).mc_snum as libc::c_uint;
            mdb_cursor_copy(cdst as *const MDB_cursor, &mut mn);
            rc = mdb_page_search_lowest(&mut mn);
            if rc != 0 {
                return rc;
            }
            if (*mn.mc_pg[mn.mc_top as usize]).mp_flags as libc::c_int
                & 32 as libc::c_int == 32 as libc::c_int
            {
                bkey.mv_size = (*mn.mc_db).md_pad as size_t;
                bkey
                    .mv_data = (mn.mc_pg[mn.mc_top as usize] as *mut libc::c_char)
                    .offset(
                        &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                            as libc::c_ulong as libc::c_uint as isize,
                    )
                    .offset(0 as libc::c_ulong as isize) as *mut libc::c_void;
            } else {
                s2___0 = (mn.mc_pg[mn.mc_top as usize] as *mut libc::c_char)
                    .offset(
                        *((*mn.mc_pg[mn.mc_top as usize]).mp_ptrs)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as libc::c_int as isize,
                    )
                    .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                bkey.mv_size = (*s2___0).mn_ksize as size_t;
                bkey.mv_data = ((*s2___0).mn_data).as_mut_ptr() as *mut libc::c_void;
            }
            tmp___0 = snum___0;
            snum___0 = snum___0.wrapping_sub(1);
            mn.mc_snum = tmp___0 as libc::c_ushort;
            mn.mc_top = snum___0 as libc::c_ushort;
            mn.mc_ki[snum___0 as usize] = 0 as libc::c_int as indx_t;
            rc = mdb_update_key(&mut mn, &mut bkey);
            if rc != 0 {
                return rc;
            }
        }
    }
    rc = mdb_node_add(
        cdst,
        (*cdst).mc_ki[(*cdst).mc_top as usize],
        &mut key,
        &mut data,
        srcpg,
        flags as libc::c_uint,
    );
    if rc != 0 as libc::c_int {
        return rc;
    }
    mdb_node_del(csrc, key.mv_size as libc::c_int);
    dbi = (*csrc).mc_dbi;
    mps = (*csrc).mc_pg[(*csrc).mc_top as usize];
    if fromleft != 0 {
        mpd = (*cdst).mc_pg[(*csrc).mc_top as usize];
        m2 = *((*(*csrc).mc_txn).mt_cursors).offset(dbi as isize);
        while !m2.is_null() {
            if (*csrc).mc_flags & 4 as libc::c_uint != 0 {
                m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
            } else {
                m3 = m2;
            }
            if !((*m3).mc_flags & 1 as libc::c_uint == 0) {
                if !(((*m3).mc_top as libc::c_int) < (*csrc).mc_top as libc::c_int) {
                    if m3 as libc::c_ulong != cdst as libc::c_ulong {
                        if (*m3).mc_pg[(*csrc).mc_top as usize] as libc::c_ulong
                            == mpd as libc::c_ulong
                        {
                            if (*m3).mc_ki[(*csrc).mc_top as usize] as libc::c_int
                                >= (*cdst).mc_ki[(*csrc).mc_top as usize] as libc::c_int
                            {
                                (*m3)
                                    .mc_ki[(*csrc).mc_top
                                    as usize] = ((*m3).mc_ki[(*csrc).mc_top as usize]
                                    as libc::c_int + 1 as libc::c_int) as indx_t;
                            }
                        }
                    }
                    if m3 as libc::c_ulong != csrc as libc::c_ulong {
                        if (*m3).mc_pg[(*csrc).mc_top as usize] as libc::c_ulong
                            == mps as libc::c_ulong
                        {
                            if (*m3).mc_ki[(*csrc).mc_top as usize] as libc::c_int
                                == (*csrc).mc_ki[(*csrc).mc_top as usize] as libc::c_int
                            {
                                (*m3)
                                    .mc_pg[(*csrc).mc_top
                                    as usize] = (*cdst).mc_pg[(*cdst).mc_top as usize];
                                (*m3)
                                    .mc_ki[(*csrc).mc_top
                                    as usize] = (*cdst).mc_ki[(*cdst).mc_top as usize];
                                (*m3)
                                    .mc_ki[((*csrc).mc_top as libc::c_int - 1 as libc::c_int)
                                    as usize] = ((*m3)
                                    .mc_ki[((*csrc).mc_top as libc::c_int - 1 as libc::c_int)
                                    as usize] as libc::c_int + 1 as libc::c_int) as indx_t;
                            }
                        }
                    }
                    if (*mps).mp_flags as libc::c_int & 2 as libc::c_int
                        == 2 as libc::c_int
                    {
                        xr_pg = (*m3).mc_pg[(*csrc).mc_top as usize];
                        if !((*m3).mc_xcursor).is_null() {
                            if (*(*m3).mc_xcursor).mx_cursor.mc_flags & 1 as libc::c_uint
                                != 0
                            {
                                if !((*m3).mc_ki[(*csrc).mc_top as usize] as libc::c_uint
                                    >= ((*xr_pg).mp_pb.pb.pb_lower as libc::c_uint)
                                        .wrapping_sub(
                                            &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                                as libc::c_ulong as libc::c_uint,
                                        ) >> 1 as libc::c_int)
                                {
                                    xr_node = (xr_pg as *mut libc::c_char)
                                        .offset(
                                            *((*xr_pg).mp_ptrs)
                                                .as_mut_ptr()
                                                .offset((*m3).mc_ki[(*csrc).mc_top as usize] as isize)
                                                as libc::c_int as isize,
                                        )
                                        .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                                    if (*xr_node).mn_flags as libc::c_int & 6 as libc::c_int
                                        == 4 as libc::c_int
                                    {
                                        (*(*m3).mc_xcursor)
                                            .mx_cursor
                                            .mc_pg[0 as libc::c_int
                                            as usize] = ((*xr_node).mn_data)
                                            .as_mut_ptr()
                                            .offset((*xr_node).mn_ksize as libc::c_int as isize)
                                            as *mut libc::c_void as *mut MDB_page;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            m2 = (*m2).mc_next;
        }
    } else {
        m2 = *((*(*csrc).mc_txn).mt_cursors).offset(dbi as isize);
        while !m2.is_null() {
            if (*csrc).mc_flags & 4 as libc::c_uint != 0 {
                m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
            } else {
                m3 = m2;
            }
            if !(m3 as libc::c_ulong == csrc as libc::c_ulong) {
                if !((*m3).mc_flags & 1 as libc::c_uint == 0) {
                    if !(((*m3).mc_top as libc::c_int) < (*csrc).mc_top as libc::c_int) {
                        if (*m3).mc_pg[(*csrc).mc_top as usize] as libc::c_ulong
                            == mps as libc::c_ulong
                        {
                            if (*m3).mc_ki[(*csrc).mc_top as usize] == 0 {
                                (*m3)
                                    .mc_pg[(*csrc).mc_top
                                    as usize] = (*cdst).mc_pg[(*cdst).mc_top as usize];
                                (*m3)
                                    .mc_ki[(*csrc).mc_top
                                    as usize] = (*cdst).mc_ki[(*cdst).mc_top as usize];
                                (*m3)
                                    .mc_ki[((*csrc).mc_top as libc::c_int - 1 as libc::c_int)
                                    as usize] = ((*m3)
                                    .mc_ki[((*csrc).mc_top as libc::c_int - 1 as libc::c_int)
                                    as usize] as libc::c_int - 1 as libc::c_int) as indx_t;
                            } else {
                                (*m3)
                                    .mc_ki[(*csrc).mc_top
                                    as usize] = ((*m3).mc_ki[(*csrc).mc_top as usize]
                                    as libc::c_int - 1 as libc::c_int) as indx_t;
                            }
                            if (*mps).mp_flags as libc::c_int & 2 as libc::c_int
                                == 2 as libc::c_int
                            {
                                xr_pg___0 = (*m3).mc_pg[(*csrc).mc_top as usize];
                                if !((*m3).mc_xcursor).is_null() {
                                    if (*(*m3).mc_xcursor).mx_cursor.mc_flags
                                        & 1 as libc::c_uint != 0
                                    {
                                        if !((*m3).mc_ki[(*csrc).mc_top as usize] as libc::c_uint
                                            >= ((*xr_pg___0).mp_pb.pb.pb_lower as libc::c_uint)
                                                .wrapping_sub(
                                                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                                        as libc::c_ulong as libc::c_uint,
                                                ) >> 1 as libc::c_int)
                                        {
                                            xr_node___0 = (xr_pg___0 as *mut libc::c_char)
                                                .offset(
                                                    *((*xr_pg___0).mp_ptrs)
                                                        .as_mut_ptr()
                                                        .offset((*m3).mc_ki[(*csrc).mc_top as usize] as isize)
                                                        as libc::c_int as isize,
                                                )
                                                .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                                            if (*xr_node___0).mn_flags as libc::c_int & 6 as libc::c_int
                                                == 4 as libc::c_int
                                            {
                                                (*(*m3).mc_xcursor)
                                                    .mx_cursor
                                                    .mc_pg[0 as libc::c_int
                                                    as usize] = ((*xr_node___0).mn_data)
                                                    .as_mut_ptr()
                                                    .offset((*xr_node___0).mn_ksize as libc::c_int as isize)
                                                    as *mut libc::c_void as *mut MDB_page;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            m2 = (*m2).mc_next;
        }
    }
    if (*csrc).mc_ki[(*csrc).mc_top as usize] as libc::c_int == 0 as libc::c_int {
        if (*csrc).mc_ki[((*csrc).mc_top as libc::c_int - 1 as libc::c_int) as usize]
            as libc::c_int != 0 as libc::c_int
        {
            if (*(*csrc).mc_pg[(*csrc).mc_top as usize]).mp_flags as libc::c_int
                & 32 as libc::c_int == 32 as libc::c_int
            {
                key
                    .mv_data = ((*csrc).mc_pg[(*csrc).mc_top as usize]
                    as *mut libc::c_char)
                    .offset(
                        &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                            as libc::c_ulong as libc::c_uint as isize,
                    )
                    .offset(0 as libc::c_ulong as isize) as *mut libc::c_void;
            } else {
                srcnode = ((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut libc::c_char)
                    .offset(
                        *((*(*csrc).mc_pg[(*csrc).mc_top as usize]).mp_ptrs)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as libc::c_int as isize,
                    )
                    .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                key.mv_size = (*srcnode).mn_ksize as size_t;
                key.mv_data = ((*srcnode).mn_data).as_mut_ptr() as *mut libc::c_void;
            }
            mdb_cursor_copy(csrc as *const MDB_cursor, &mut mn);
            mn
                .mc_snum = (mn.mc_snum as libc::c_int - 1 as libc::c_int)
                as libc::c_ushort;
            mn.mc_top = (mn.mc_top as libc::c_int - 1 as libc::c_int) as libc::c_ushort;
            tp = ((*mn.mc_txn).mt_cursors).offset(mn.mc_dbi as isize);
            if mn.mc_flags & 4 as libc::c_uint != 0 {
                dummy.mc_flags = 1 as libc::c_uint;
                dummy.mc_xcursor = &mut mn as *mut MDB_cursor as *mut MDB_xcursor;
                tracked = &mut dummy;
            } else {
                tracked = &mut mn;
            }
            (*tracked).mc_next = *tp;
            *tp = tracked;
            rc = mdb_update_key(&mut mn, &mut key);
            *tp = (*tracked).mc_next;
            if rc != 0 {
                return rc;
            }
        }
        if (*(*csrc).mc_pg[(*csrc).mc_top as usize]).mp_flags as libc::c_int
            & 1 as libc::c_int == 1 as libc::c_int
        {
            ix = (*csrc).mc_ki[(*csrc).mc_top as usize];
            nullkey.mv_size = 0 as libc::c_int as size_t;
            (*csrc).mc_ki[(*csrc).mc_top as usize] = 0 as libc::c_int as indx_t;
            rc = mdb_update_key(csrc, &mut nullkey);
            (*csrc).mc_ki[(*csrc).mc_top as usize] = ix;
            if !(rc == 0 as libc::c_int) {
                mdb_assert_fail(
                    (*(*csrc).mc_txn).mt_env,
                    b"rc == MDB_SUCCESS\0" as *const u8 as *const libc::c_char,
                    b"mdb_node_move\0" as *const u8 as *const libc::c_char,
                    b"mdb.c\0" as *const u8 as *const libc::c_char,
                    9056 as libc::c_int,
                );
            }
        }
    }
    if (*cdst).mc_ki[(*cdst).mc_top as usize] as libc::c_int == 0 as libc::c_int {
        if (*cdst).mc_ki[((*cdst).mc_top as libc::c_int - 1 as libc::c_int) as usize]
            as libc::c_int != 0 as libc::c_int
        {
            if (*(*csrc).mc_pg[(*csrc).mc_top as usize]).mp_flags as libc::c_int
                & 32 as libc::c_int == 32 as libc::c_int
            {
                key
                    .mv_data = ((*cdst).mc_pg[(*cdst).mc_top as usize]
                    as *mut libc::c_char)
                    .offset(
                        &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                            as libc::c_ulong as libc::c_uint as isize,
                    )
                    .offset(0 as libc::c_ulong as isize) as *mut libc::c_void;
            } else {
                srcnode = ((*cdst).mc_pg[(*cdst).mc_top as usize] as *mut libc::c_char)
                    .offset(
                        *((*(*cdst).mc_pg[(*cdst).mc_top as usize]).mp_ptrs)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as libc::c_int as isize,
                    )
                    .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                key.mv_size = (*srcnode).mn_ksize as size_t;
                key.mv_data = ((*srcnode).mn_data).as_mut_ptr() as *mut libc::c_void;
            }
            mdb_cursor_copy(cdst as *const MDB_cursor, &mut mn);
            mn
                .mc_snum = (mn.mc_snum as libc::c_int - 1 as libc::c_int)
                as libc::c_ushort;
            mn.mc_top = (mn.mc_top as libc::c_int - 1 as libc::c_int) as libc::c_ushort;
            tp___0 = ((*mn.mc_txn).mt_cursors).offset(mn.mc_dbi as isize);
            if mn.mc_flags & 4 as libc::c_uint != 0 {
                dummy___0.mc_flags = 1 as libc::c_uint;
                dummy___0.mc_xcursor = &mut mn as *mut MDB_cursor as *mut MDB_xcursor;
                tracked___0 = &mut dummy___0;
            } else {
                tracked___0 = &mut mn;
            }
            (*tracked___0).mc_next = *tp___0;
            *tp___0 = tracked___0;
            rc = mdb_update_key(&mut mn, &mut key);
            *tp___0 = (*tracked___0).mc_next;
            if rc != 0 {
                return rc;
            }
        }
        if (*(*cdst).mc_pg[(*cdst).mc_top as usize]).mp_flags as libc::c_int
            & 1 as libc::c_int == 1 as libc::c_int
        {
            ix___0 = (*cdst).mc_ki[(*cdst).mc_top as usize];
            nullkey___0.mv_size = 0 as libc::c_int as size_t;
            (*cdst).mc_ki[(*cdst).mc_top as usize] = 0 as libc::c_int as indx_t;
            rc = mdb_update_key(cdst, &mut nullkey___0);
            (*cdst).mc_ki[(*cdst).mc_top as usize] = ix___0;
            if !(rc == 0 as libc::c_int) {
                mdb_assert_fail(
                    (*(*cdst).mc_txn).mt_env,
                    b"rc == MDB_SUCCESS\0" as *const u8 as *const libc::c_char,
                    b"mdb_node_move\0" as *const u8 as *const libc::c_char,
                    b"mdb.c\0" as *const u8 as *const libc::c_char,
                    9087 as libc::c_int,
                );
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_page_merge(
    mut csrc: *mut MDB_cursor,
    mut cdst: *mut MDB_cursor,
) -> libc::c_int {
    let mut psrc: *mut MDB_page = 0 as *mut MDB_page;
    let mut pdst: *mut MDB_page = 0 as *mut MDB_page;
    let mut srcnode: *mut MDB_node = 0 as *mut MDB_node;
    let mut key: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut data: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut nkeys: libc::c_uint = 0;
    let mut rc: libc::c_int = 0;
    let mut i: indx_t = 0;
    let mut j: indx_t = 0;
    let mut mn: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut s2: *mut MDB_node = 0 as *mut MDB_node;
    let mut m2: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut m3: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut dbi: MDB_dbi = 0;
    let mut top: libc::c_uint = 0;
    let mut xr_pg: *mut MDB_page = 0 as *mut MDB_page;
    let mut xr_node: *mut MDB_node = 0 as *mut MDB_node;
    let mut snum: libc::c_uint = 0;
    let mut depth: uint16_t = 0;
    psrc = (*csrc).mc_pg[(*csrc).mc_top as usize];
    pdst = (*cdst).mc_pg[(*cdst).mc_top as usize];
    if !((*csrc).mc_snum as libc::c_int > 1 as libc::c_int) {
        mdb_assert_fail(
            (*(*csrc).mc_txn).mt_env,
            b"csrc->mc_snum > 1\0" as *const u8 as *const libc::c_char,
            b"mdb_page_merge\0" as *const u8 as *const libc::c_char,
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            9117 as libc::c_int,
        );
    }
    if !((*cdst).mc_snum as libc::c_int > 1 as libc::c_int) {
        mdb_assert_fail(
            (*(*csrc).mc_txn).mt_env,
            b"cdst->mc_snum > 1\0" as *const u8 as *const libc::c_char,
            b"mdb_page_merge\0" as *const u8 as *const libc::c_char,
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            9118 as libc::c_int,
        );
    }
    rc = mdb_page_touch(cdst);
    if rc != 0 {
        return rc;
    }
    pdst = (*cdst).mc_pg[(*cdst).mc_top as usize];
    nkeys = ((*pdst).mp_pb.pb.pb_lower as libc::c_uint)
        .wrapping_sub(
            &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                as libc::c_uint,
        ) >> 1 as libc::c_int;
    j = nkeys as indx_t;
    if (*psrc).mp_flags as libc::c_int & 32 as libc::c_int == 32 as libc::c_int {
        key.mv_size = (*(*csrc).mc_db).md_pad as size_t;
        key
            .mv_data = (psrc as *mut libc::c_char)
            .offset(
                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                    as libc::c_uint as isize,
            ) as *mut libc::c_void;
        i = 0 as libc::c_int as indx_t;
        while (i as libc::c_uint)
            < ((*psrc).mp_pb.pb.pb_lower as libc::c_uint)
                .wrapping_sub(
                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                        as libc::c_ulong as libc::c_uint,
                ) >> 1 as libc::c_int
        {
            rc = mdb_node_add(
                cdst,
                j,
                &mut key,
                0 as *mut libc::c_void as *mut MDB_val,
                0 as libc::c_int as pgno_t,
                0 as libc::c_uint,
            );
            if rc != 0 as libc::c_int {
                return rc;
            }
            key
                .mv_data = (key.mv_data as *mut libc::c_char)
                .offset(key.mv_size as isize) as *mut libc::c_void;
            i = (i as libc::c_int + 1 as libc::c_int) as indx_t;
            j = (j as libc::c_int + 1 as libc::c_int) as indx_t;
        }
    } else {
        i = 0 as libc::c_int as indx_t;
        while (i as libc::c_uint)
            < ((*psrc).mp_pb.pb.pb_lower as libc::c_uint)
                .wrapping_sub(
                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                        as libc::c_ulong as libc::c_uint,
                ) >> 1 as libc::c_int
        {
            srcnode = (psrc as *mut libc::c_char)
                .offset(
                    *((*psrc).mp_ptrs).as_mut_ptr().offset(i as isize) as libc::c_int
                        as isize,
                )
                .offset(0 as libc::c_uint as isize) as *mut MDB_node;
            if i as libc::c_int == 0 as libc::c_int {
                if (*psrc).mp_flags as libc::c_int & 1 as libc::c_int == 1 as libc::c_int
                {
                    mdb_cursor_copy(csrc as *const MDB_cursor, &mut mn);
                    mn.mc_xcursor = 0 as *mut libc::c_void as *mut MDB_xcursor;
                    rc = mdb_page_search_lowest(&mut mn);
                    if rc != 0 {
                        return rc;
                    }
                    if (*mn.mc_pg[mn.mc_top as usize]).mp_flags as libc::c_int
                        & 32 as libc::c_int == 32 as libc::c_int
                    {
                        key.mv_size = (*mn.mc_db).md_pad as size_t;
                        key
                            .mv_data = (mn.mc_pg[mn.mc_top as usize]
                            as *mut libc::c_char)
                            .offset(
                                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                    as libc::c_ulong as libc::c_uint as isize,
                            )
                            .offset(0 as libc::c_ulong as isize) as *mut libc::c_void;
                    } else {
                        s2 = (mn.mc_pg[mn.mc_top as usize] as *mut libc::c_char)
                            .offset(
                                *((*mn.mc_pg[mn.mc_top as usize]).mp_ptrs)
                                    .as_mut_ptr()
                                    .offset(0 as libc::c_int as isize) as libc::c_int as isize,
                            )
                            .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                        key.mv_size = (*s2).mn_ksize as size_t;
                        key.mv_data = ((*s2).mn_data).as_mut_ptr() as *mut libc::c_void;
                    }
                } else {
                    key.mv_size = (*srcnode).mn_ksize as size_t;
                    key.mv_data = ((*srcnode).mn_data).as_mut_ptr() as *mut libc::c_void;
                }
            } else {
                key.mv_size = (*srcnode).mn_ksize as size_t;
                key.mv_data = ((*srcnode).mn_data).as_mut_ptr() as *mut libc::c_void;
            }
            data
                .mv_size = ((*srcnode).mn_lo as libc::c_uint
                | ((*srcnode).mn_hi as libc::c_uint) << 16 as libc::c_int) as size_t;
            data
                .mv_data = ((*srcnode).mn_data)
                .as_mut_ptr()
                .offset((*srcnode).mn_ksize as libc::c_int as isize)
                as *mut libc::c_void;
            rc = mdb_node_add(
                cdst,
                j,
                &mut key,
                &mut data,
                (*srcnode).mn_lo as libc::c_ulong
                    | ((*srcnode).mn_hi as pgno_t) << 16 as libc::c_int
                    | ((*srcnode).mn_flags as pgno_t) << 32 as libc::c_int,
                (*srcnode).mn_flags as libc::c_uint,
            );
            if rc != 0 as libc::c_int {
                return rc;
            }
            i = (i as libc::c_int + 1 as libc::c_int) as indx_t;
            j = (j as libc::c_int + 1 as libc::c_int) as indx_t;
        }
    }
    (*csrc)
        .mc_top = ((*csrc).mc_top as libc::c_int - 1 as libc::c_int) as libc::c_ushort;
    mdb_node_del(csrc, 0 as libc::c_int);
    if (*csrc).mc_ki[(*csrc).mc_top as usize] as libc::c_int == 0 as libc::c_int {
        key.mv_size = 0 as libc::c_int as size_t;
        rc = mdb_update_key(csrc, &mut key);
        if rc != 0 {
            (*csrc)
                .mc_top = ((*csrc).mc_top as libc::c_int + 1 as libc::c_int)
                as libc::c_ushort;
            return rc;
        }
    }
    (*csrc)
        .mc_top = ((*csrc).mc_top as libc::c_int + 1 as libc::c_int) as libc::c_ushort;
    psrc = (*csrc).mc_pg[(*csrc).mc_top as usize];
    rc = mdb_page_loose(csrc, psrc);
    if rc != 0 {
        return rc;
    }
    if (*psrc).mp_flags as libc::c_int & 2 as libc::c_int == 2 as libc::c_int {
        (*(*csrc).mc_db)
            .md_leaf_pages = ((*(*csrc).mc_db).md_leaf_pages).wrapping_sub(1);
    } else {
        (*(*csrc).mc_db)
            .md_branch_pages = ((*(*csrc).mc_db).md_branch_pages).wrapping_sub(1);
    }
    dbi = (*csrc).mc_dbi;
    top = (*csrc).mc_top as libc::c_uint;
    m2 = *((*(*csrc).mc_txn).mt_cursors).offset(dbi as isize);
    while !m2.is_null() {
        if (*csrc).mc_flags & 4 as libc::c_uint != 0 {
            m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
        } else {
            m3 = m2;
        }
        if !(m3 as libc::c_ulong == csrc as libc::c_ulong) {
            if !(((*m3).mc_snum as libc::c_int) < (*csrc).mc_snum as libc::c_int) {
                if (*m3).mc_pg[top as usize] as libc::c_ulong == psrc as libc::c_ulong {
                    (*m3).mc_pg[top as usize] = pdst;
                    (*m3)
                        .mc_ki[top
                        as usize] = ((*m3).mc_ki[top as usize] as libc::c_uint)
                        .wrapping_add(nkeys) as indx_t;
                    (*m3)
                        .mc_ki[top.wrapping_sub(1 as libc::c_uint)
                        as usize] = (*cdst)
                        .mc_ki[top.wrapping_sub(1 as libc::c_uint) as usize];
                } else if (*m3).mc_pg[top.wrapping_sub(1 as libc::c_uint) as usize]
                        as libc::c_ulong
                        == (*csrc).mc_pg[top.wrapping_sub(1 as libc::c_uint) as usize]
                            as libc::c_ulong
                    {
                    if (*m3).mc_ki[top.wrapping_sub(1 as libc::c_uint) as usize]
                        as libc::c_int
                        > (*csrc).mc_ki[top.wrapping_sub(1 as libc::c_uint) as usize]
                            as libc::c_int
                    {
                        (*m3)
                            .mc_ki[top.wrapping_sub(1 as libc::c_uint)
                            as usize] = ((*m3)
                            .mc_ki[top.wrapping_sub(1 as libc::c_uint) as usize]
                            as libc::c_int - 1 as libc::c_int) as indx_t;
                    }
                }
                if (*psrc).mp_flags as libc::c_int & 2 as libc::c_int == 2 as libc::c_int
                {
                    xr_pg = (*m3).mc_pg[top as usize];
                    if !((*m3).mc_xcursor).is_null() {
                        if (*(*m3).mc_xcursor).mx_cursor.mc_flags & 1 as libc::c_uint
                            != 0
                        {
                            if !((*m3).mc_ki[top as usize] as libc::c_uint
                                >= ((*xr_pg).mp_pb.pb.pb_lower as libc::c_uint)
                                    .wrapping_sub(
                                        &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                            as libc::c_ulong as libc::c_uint,
                                    ) >> 1 as libc::c_int)
                            {
                                xr_node = (xr_pg as *mut libc::c_char)
                                    .offset(
                                        *((*xr_pg).mp_ptrs)
                                            .as_mut_ptr()
                                            .offset((*m3).mc_ki[top as usize] as isize) as libc::c_int
                                            as isize,
                                    )
                                    .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                                if (*xr_node).mn_flags as libc::c_int & 6 as libc::c_int
                                    == 4 as libc::c_int
                                {
                                    (*(*m3).mc_xcursor)
                                        .mx_cursor
                                        .mc_pg[0 as libc::c_int
                                        as usize] = ((*xr_node).mn_data)
                                        .as_mut_ptr()
                                        .offset((*xr_node).mn_ksize as libc::c_int as isize)
                                        as *mut libc::c_void as *mut MDB_page;
                                }
                            }
                        }
                    }
                }
            }
        }
        m2 = (*m2).mc_next;
    }
    snum = (*cdst).mc_snum as libc::c_uint;
    depth = (*(*cdst).mc_db).md_depth;
    mdb_cursor_pop(cdst);
    rc = mdb_rebalance(cdst);
    if depth as libc::c_int != (*(*cdst).mc_db).md_depth as libc::c_int {
        snum = snum
            .wrapping_add(
                ((*(*cdst).mc_db).md_depth as libc::c_int - depth as libc::c_int)
                    as libc::c_uint,
            );
    }
    (*cdst).mc_snum = snum as libc::c_ushort;
    (*cdst).mc_top = snum.wrapping_sub(1 as libc::c_uint) as libc::c_ushort;
    return rc;
}
unsafe extern "C" fn mdb_cursor_copy(
    mut csrc: *const MDB_cursor,
    mut cdst: *mut MDB_cursor,
) {
    let mut i: libc::c_uint = 0;
    (*cdst).mc_txn = (*csrc).mc_txn;
    (*cdst).mc_dbi = (*csrc).mc_dbi;
    (*cdst).mc_db = (*csrc).mc_db;
    (*cdst).mc_dbx = (*csrc).mc_dbx;
    (*cdst).mc_snum = (*csrc).mc_snum;
    (*cdst).mc_top = (*csrc).mc_top;
    (*cdst).mc_flags = (*csrc).mc_flags;
    i = 0 as libc::c_uint;
    while i < (*csrc).mc_snum as libc::c_uint {
        (*cdst).mc_pg[i as usize] = (*csrc).mc_pg[i as usize];
        (*cdst).mc_ki[i as usize] = (*csrc).mc_ki[i as usize];
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn mdb_rebalance(mut mc: *mut MDB_cursor) -> libc::c_int {
    let mut node: *mut MDB_node = 0 as *mut MDB_node;
    let mut rc: libc::c_int = 0;
    let mut fromleft: libc::c_int = 0;
    let mut ptop: libc::c_uint = 0;
    let mut minkeys: libc::c_uint = 0;
    let mut thresh: libc::c_uint = 0;
    let mut mn: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut oldki: indx_t = 0;
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut m2: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut m3: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut dbi: MDB_dbi = 0;
    let mut i: libc::c_int = 0;
    let mut m2___0: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut m3___0: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut dbi___0: MDB_dbi = 0;
    let mut dummy: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut tracked: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut tp: *mut *mut MDB_cursor = 0 as *mut *mut MDB_cursor;
    if (*(*mc).mc_pg[(*mc).mc_top as usize]).mp_flags as libc::c_int & 1 as libc::c_int
        == 1 as libc::c_int
    {
        minkeys = 2 as libc::c_uint;
        thresh = 1 as libc::c_uint;
    } else {
        minkeys = 1 as libc::c_uint;
        thresh = 250 as libc::c_uint;
    }
    if 1000 as libc::c_long
        * ((*(*(*mc).mc_txn).mt_env).me_psize)
            .wrapping_sub(
                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                    as libc::c_uint,
            )
            .wrapping_sub(
                ((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_pb.pb.pb_upper as libc::c_int
                    - (*(*mc).mc_pg[(*mc).mc_top as usize]).mp_pb.pb.pb_lower
                        as libc::c_int) as indx_t as libc::c_uint,
            ) as libc::c_long
        / ((*(*(*mc).mc_txn).mt_env).me_psize)
            .wrapping_sub(
                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                    as libc::c_uint,
            ) as libc::c_long >= thresh as libc::c_long
    {
        if ((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_pb.pb.pb_lower as libc::c_uint)
            .wrapping_sub(
                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                    as libc::c_uint,
            ) >> 1 as libc::c_int >= minkeys
        {
            return 0 as libc::c_int;
        }
    }
    if ((*mc).mc_snum as libc::c_int) < 2 as libc::c_int {
        mp = (*mc).mc_pg[0 as libc::c_int as usize];
        if (*mp).mp_flags as libc::c_int & 64 as libc::c_int == 64 as libc::c_int {
            return 0 as libc::c_int;
        }
        if ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
            .wrapping_sub(
                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                    as libc::c_uint,
            ) >> 1 as libc::c_int == 0 as libc::c_uint
        {
            (*(*mc).mc_db).md_root = !(0 as libc::c_int as pgno_t);
            (*(*mc).mc_db).md_depth = 0 as libc::c_int as uint16_t;
            (*(*mc).mc_db).md_leaf_pages = 0 as libc::c_int as pgno_t;
            rc = mdb_midl_append(&mut (*(*mc).mc_txn).mt_free_pgs, (*mp).mp_p.p_pgno);
            if rc != 0 {
                return rc;
            }
            (*mc).mc_snum = 0 as libc::c_int as libc::c_ushort;
            (*mc).mc_top = 0 as libc::c_int as libc::c_ushort;
            (*mc).mc_flags &= 4294967294 as libc::c_uint;
            dbi = (*mc).mc_dbi;
            m2 = *((*(*mc).mc_txn).mt_cursors).offset(dbi as isize);
            while !m2.is_null() {
                if (*mc).mc_flags & 4 as libc::c_uint != 0 {
                    m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
                } else {
                    m3 = m2;
                }
                if !((*m3).mc_flags & 1 as libc::c_uint == 0) {
                    if !(((*m3).mc_snum as libc::c_int) < (*mc).mc_snum as libc::c_int) {
                        if (*m3).mc_pg[0 as libc::c_int as usize] as libc::c_ulong
                            == mp as libc::c_ulong
                        {
                            (*m3).mc_snum = 0 as libc::c_int as libc::c_ushort;
                            (*m3).mc_top = 0 as libc::c_int as libc::c_ushort;
                            (*m3).mc_flags &= 4294967294 as libc::c_uint;
                        }
                    }
                }
                m2 = (*m2).mc_next;
            }
        } else if (*mp).mp_flags as libc::c_int & 1 as libc::c_int == 1 as libc::c_int {
            if ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                .wrapping_sub(
                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                        as libc::c_ulong as libc::c_uint,
                ) >> 1 as libc::c_int == 1 as libc::c_uint
            {
                rc = mdb_midl_append(
                    &mut (*(*mc).mc_txn).mt_free_pgs,
                    (*mp).mp_p.p_pgno,
                );
                if rc != 0 {
                    return rc;
                }
                (*(*mc).mc_db)
                    .md_root = (*((mp as *mut libc::c_char)
                    .offset(
                        *((*mp).mp_ptrs).as_mut_ptr().offset(0 as libc::c_int as isize)
                            as libc::c_int as isize,
                    )
                    .offset(0 as libc::c_uint as isize) as *mut MDB_node))
                    .mn_lo as libc::c_ulong
                    | ((*((mp as *mut libc::c_char)
                        .offset(
                            *((*mp).mp_ptrs)
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize) as libc::c_int as isize,
                        )
                        .offset(0 as libc::c_uint as isize) as *mut MDB_node))
                        .mn_hi as pgno_t) << 16 as libc::c_int
                    | ((*((mp as *mut libc::c_char)
                        .offset(
                            *((*mp).mp_ptrs)
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize) as libc::c_int as isize,
                        )
                        .offset(0 as libc::c_uint as isize) as *mut MDB_node))
                        .mn_flags as pgno_t) << 32 as libc::c_int;
                rc = mdb_page_get(
                    mc,
                    (*(*mc).mc_db).md_root,
                    &mut *((*mc).mc_pg).as_mut_ptr().offset(0 as libc::c_int as isize),
                    0 as *mut libc::c_void as *mut libc::c_int,
                );
                if rc != 0 {
                    return rc;
                }
                (*(*mc).mc_db)
                    .md_depth = ((*(*mc).mc_db).md_depth as libc::c_int
                    - 1 as libc::c_int) as uint16_t;
                (*(*mc).mc_db)
                    .md_branch_pages = ((*(*mc).mc_db).md_branch_pages).wrapping_sub(1);
                (*mc)
                    .mc_ki[0 as libc::c_int
                    as usize] = (*mc).mc_ki[1 as libc::c_int as usize];
                i = 1 as libc::c_int;
                while i < (*(*mc).mc_db).md_depth as libc::c_int {
                    (*mc)
                        .mc_pg[i
                        as usize] = (*mc).mc_pg[(i + 1 as libc::c_int) as usize];
                    (*mc)
                        .mc_ki[i
                        as usize] = (*mc).mc_ki[(i + 1 as libc::c_int) as usize];
                    i += 1;
                }
                dbi___0 = (*mc).mc_dbi;
                m2___0 = *((*(*mc).mc_txn).mt_cursors).offset(dbi___0 as isize);
                while !m2___0.is_null() {
                    if (*mc).mc_flags & 4 as libc::c_uint != 0 {
                        m3___0 = &mut (*(*m2___0).mc_xcursor).mx_cursor;
                    } else {
                        m3___0 = m2___0;
                    }
                    if !(m3___0 as libc::c_ulong == mc as libc::c_ulong) {
                        if !((*m3___0).mc_flags & 1 as libc::c_uint == 0) {
                            if (*m3___0).mc_pg[0 as libc::c_int as usize]
                                as libc::c_ulong == mp as libc::c_ulong
                            {
                                i = 0 as libc::c_int;
                                while i < (*(*mc).mc_db).md_depth as libc::c_int {
                                    (*m3___0)
                                        .mc_pg[i
                                        as usize] = (*m3___0)
                                        .mc_pg[(i + 1 as libc::c_int) as usize];
                                    (*m3___0)
                                        .mc_ki[i
                                        as usize] = (*m3___0)
                                        .mc_ki[(i + 1 as libc::c_int) as usize];
                                    i += 1;
                                }
                                (*m3___0)
                                    .mc_snum = ((*m3___0).mc_snum as libc::c_int
                                    - 1 as libc::c_int) as libc::c_ushort;
                                (*m3___0)
                                    .mc_top = ((*m3___0).mc_top as libc::c_int
                                    - 1 as libc::c_int) as libc::c_ushort;
                            }
                        }
                    }
                    m2___0 = (*m2___0).mc_next;
                }
            }
        }
        return 0 as libc::c_int;
    } else {
        ptop = ((*mc).mc_top as libc::c_int - 1 as libc::c_int) as libc::c_uint;
        if !(((*(*mc).mc_pg[ptop as usize]).mp_pb.pb.pb_lower as libc::c_uint)
            .wrapping_sub(
                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                    as libc::c_uint,
            ) >> 1 as libc::c_int > 1 as libc::c_uint)
        {
            mdb_assert_fail(
                (*(*mc).mc_txn).mt_env,
                b"NUMKEYS(mc->mc_pg[ptop]) > 1\0" as *const u8 as *const libc::c_char,
                b"mdb_rebalance\0" as *const u8 as *const libc::c_char,
                b"mdb.c\0" as *const u8 as *const libc::c_char,
                9382 as libc::c_int,
            );
        }
        mdb_cursor_copy(mc as *const MDB_cursor, &mut mn);
        mn.mc_xcursor = 0 as *mut libc::c_void as *mut MDB_xcursor;
        oldki = (*mc).mc_ki[(*mc).mc_top as usize];
        if (*mc).mc_ki[ptop as usize] as libc::c_int == 0 as libc::c_int {
            mn
                .mc_ki[ptop
                as usize] = (mn.mc_ki[ptop as usize] as libc::c_int + 1 as libc::c_int)
                as indx_t;
            node = ((*mc).mc_pg[ptop as usize] as *mut libc::c_char)
                .offset(
                    *((*(*mc).mc_pg[ptop as usize]).mp_ptrs)
                        .as_mut_ptr()
                        .offset(mn.mc_ki[ptop as usize] as isize) as libc::c_int as isize,
                )
                .offset(0 as libc::c_uint as isize) as *mut MDB_node;
            rc = mdb_page_get(
                mc,
                (*node).mn_lo as libc::c_ulong
                    | ((*node).mn_hi as pgno_t) << 16 as libc::c_int
                    | ((*node).mn_flags as pgno_t) << 32 as libc::c_int,
                &mut *(mn.mc_pg).as_mut_ptr().offset(mn.mc_top as isize),
                0 as *mut libc::c_void as *mut libc::c_int,
            );
            if rc != 0 {
                return rc;
            }
            mn.mc_ki[mn.mc_top as usize] = 0 as libc::c_int as indx_t;
            (*mc)
                .mc_ki[(*mc).mc_top
                as usize] = (((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_pb.pb.pb_lower
                as libc::c_uint)
                .wrapping_sub(
                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                        as libc::c_ulong as libc::c_uint,
                ) >> 1 as libc::c_int) as indx_t;
            fromleft = 0 as libc::c_int;
        } else {
            mn
                .mc_ki[ptop
                as usize] = (mn.mc_ki[ptop as usize] as libc::c_int - 1 as libc::c_int)
                as indx_t;
            node = ((*mc).mc_pg[ptop as usize] as *mut libc::c_char)
                .offset(
                    *((*(*mc).mc_pg[ptop as usize]).mp_ptrs)
                        .as_mut_ptr()
                        .offset(mn.mc_ki[ptop as usize] as isize) as libc::c_int as isize,
                )
                .offset(0 as libc::c_uint as isize) as *mut MDB_node;
            rc = mdb_page_get(
                mc,
                (*node).mn_lo as libc::c_ulong
                    | ((*node).mn_hi as pgno_t) << 16 as libc::c_int
                    | ((*node).mn_flags as pgno_t) << 32 as libc::c_int,
                &mut *(mn.mc_pg).as_mut_ptr().offset(mn.mc_top as isize),
                0 as *mut libc::c_void as *mut libc::c_int,
            );
            if rc != 0 {
                return rc;
            }
            mn
                .mc_ki[mn.mc_top
                as usize] = (((*mn.mc_pg[mn.mc_top as usize]).mp_pb.pb.pb_lower
                as libc::c_uint)
                .wrapping_sub(
                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                        as libc::c_ulong as libc::c_uint,
                ) >> 1 as libc::c_int)
                .wrapping_sub(1 as libc::c_uint) as indx_t;
            (*mc).mc_ki[(*mc).mc_top as usize] = 0 as libc::c_int as indx_t;
            fromleft = 1 as libc::c_int;
        }
        let mut current_block_129: u64;
        if 1000 as libc::c_long
            * ((*(*(*mc).mc_txn).mt_env).me_psize)
                .wrapping_sub(
                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                        as libc::c_ulong as libc::c_uint,
                )
                .wrapping_sub(
                    ((*mn.mc_pg[mn.mc_top as usize]).mp_pb.pb.pb_upper as libc::c_int
                        - (*mn.mc_pg[mn.mc_top as usize]).mp_pb.pb.pb_lower
                            as libc::c_int) as indx_t as libc::c_uint,
                ) as libc::c_long
            / ((*(*(*mc).mc_txn).mt_env).me_psize)
                .wrapping_sub(
                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                        as libc::c_ulong as libc::c_uint,
                ) as libc::c_long >= thresh as libc::c_long
        {
            if ((*mn.mc_pg[mn.mc_top as usize]).mp_pb.pb.pb_lower as libc::c_uint)
                .wrapping_sub(
                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                        as libc::c_ulong as libc::c_uint,
                ) >> 1 as libc::c_int > minkeys
            {
                rc = mdb_node_move(&mut mn, mc, fromleft);
                if fromleft != 0 {
                    oldki = (oldki as libc::c_int + 1 as libc::c_int) as indx_t;
                }
                current_block_129 = 13740693533991687037;
            } else {
                current_block_129 = 2444660997150821759;
            }
        } else {
            current_block_129 = 2444660997150821759;
        }
        match current_block_129 {
            2444660997150821759 => {
                if fromleft == 0 {
                    rc = mdb_page_merge(&mut mn, mc);
                } else {
                    oldki = (oldki as libc::c_uint)
                        .wrapping_add(
                            ((*mn.mc_pg[mn.mc_top as usize]).mp_pb.pb.pb_lower
                                as libc::c_uint)
                                .wrapping_sub(
                                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                        as libc::c_ulong as libc::c_uint,
                                ) >> 1 as libc::c_int,
                        ) as indx_t;
                    mn
                        .mc_ki[mn.mc_top
                        as usize] = (mn.mc_ki[mn.mc_top as usize] as libc::c_int
                        + ((*mc).mc_ki[mn.mc_top as usize] as libc::c_int
                            + 1 as libc::c_int)) as indx_t;
                    tp = ((*mn.mc_txn).mt_cursors).offset(mn.mc_dbi as isize);
                    if mn.mc_flags & 4 as libc::c_uint != 0 {
                        dummy.mc_flags = 1 as libc::c_uint;
                        dummy
                            .mc_xcursor = &mut mn as *mut MDB_cursor as *mut MDB_xcursor;
                        tracked = &mut dummy;
                    } else {
                        tracked = &mut mn;
                    }
                    (*tracked).mc_next = *tp;
                    *tp = tracked;
                    rc = mdb_page_merge(mc, &mut mn);
                    *tp = (*tracked).mc_next;
                    mdb_cursor_copy(&mut mn as *mut MDB_cursor as *const MDB_cursor, mc);
                }
                (*mc).mc_flags &= 4294967293 as libc::c_uint;
            }
            _ => {}
        }
        (*mc).mc_ki[(*mc).mc_top as usize] = oldki;
        return rc;
    };
}
unsafe extern "C" fn mdb_cursor_del0(mut mc: *mut MDB_cursor) -> libc::c_int {
    let mut current_block: u64;
    let mut rc: libc::c_int = 0;
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut ki: indx_t = 0;
    let mut nkeys: libc::c_uint = 0;
    let mut m2: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut m3: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut dbi: MDB_dbi = 0;
    let mut xr_pg: *mut MDB_page = 0 as *mut MDB_page;
    let mut xr_node: *mut MDB_node = 0 as *mut MDB_node;
    let mut node: *mut MDB_node = 0 as *mut MDB_node;
    dbi = (*mc).mc_dbi;
    ki = (*mc).mc_ki[(*mc).mc_top as usize];
    mp = (*mc).mc_pg[(*mc).mc_top as usize];
    mdb_node_del(mc, (*(*mc).mc_db).md_pad as libc::c_int);
    (*(*mc).mc_db).md_entries = ((*(*mc).mc_db).md_entries).wrapping_sub(1);
    m2 = *((*(*mc).mc_txn).mt_cursors).offset(dbi as isize);
    while !m2.is_null() {
        if (*mc).mc_flags & 4 as libc::c_uint != 0 {
            m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
        } else {
            m3 = m2;
        }
        if !((*m2).mc_flags & (*m3).mc_flags & 1 as libc::c_uint == 0) {
            if !(m3 as libc::c_ulong == mc as libc::c_ulong) {
                if !(((*m3).mc_snum as libc::c_int) < (*mc).mc_snum as libc::c_int) {
                    if (*m3).mc_pg[(*mc).mc_top as usize] as libc::c_ulong
                        == mp as libc::c_ulong
                    {
                        if (*m3).mc_ki[(*mc).mc_top as usize] as libc::c_int
                            == ki as libc::c_int
                        {
                            (*m3).mc_flags |= 8 as libc::c_uint;
                            if (*(*mc).mc_db).md_flags as libc::c_int & 4 as libc::c_int
                                != 0
                            {
                                (*(*m3).mc_xcursor).mx_cursor.mc_flags
                                    &= 4294967292 as libc::c_uint;
                            }
                        } else {
                            if (*m3).mc_ki[(*mc).mc_top as usize] as libc::c_int
                                > ki as libc::c_int
                            {
                                (*m3)
                                    .mc_ki[(*mc).mc_top
                                    as usize] = ((*m3).mc_ki[(*mc).mc_top as usize]
                                    as libc::c_int - 1 as libc::c_int) as indx_t;
                            }
                            xr_pg = mp;
                            if !((*m3).mc_xcursor).is_null() {
                                if (*(*m3).mc_xcursor).mx_cursor.mc_flags
                                    & 1 as libc::c_uint != 0
                                {
                                    if !((*m3).mc_ki[(*mc).mc_top as usize] as libc::c_uint
                                        >= ((*xr_pg).mp_pb.pb.pb_lower as libc::c_uint)
                                            .wrapping_sub(
                                                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                                    as libc::c_ulong as libc::c_uint,
                                            ) >> 1 as libc::c_int)
                                    {
                                        xr_node = (xr_pg as *mut libc::c_char)
                                            .offset(
                                                *((*xr_pg).mp_ptrs)
                                                    .as_mut_ptr()
                                                    .offset((*m3).mc_ki[(*mc).mc_top as usize] as isize)
                                                    as libc::c_int as isize,
                                            )
                                            .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                                        if (*xr_node).mn_flags as libc::c_int & 6 as libc::c_int
                                            == 4 as libc::c_int
                                        {
                                            (*(*m3).mc_xcursor)
                                                .mx_cursor
                                                .mc_pg[0 as libc::c_int
                                                as usize] = ((*xr_node).mn_data)
                                                .as_mut_ptr()
                                                .offset((*xr_node).mn_ksize as libc::c_int as isize)
                                                as *mut libc::c_void as *mut MDB_page;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        m2 = (*m2).mc_next;
    }
    rc = mdb_rebalance(mc);
    if !(rc != 0) {
        if (*mc).mc_snum == 0 {
            (*mc).mc_flags |= 2 as libc::c_uint;
            return rc;
        }
        mp = (*mc).mc_pg[(*mc).mc_top as usize];
        nkeys = ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
            .wrapping_sub(
                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                    as libc::c_uint,
            ) >> 1 as libc::c_int;
        m2 = *((*(*mc).mc_txn).mt_cursors).offset(dbi as isize);
        loop {
            if !(rc == 0) {
                current_block = 726525485109251713;
                break;
            }
            if m2.is_null() {
                current_block = 726525485109251713;
                break;
            }
            if (*mc).mc_flags & 4 as libc::c_uint != 0 {
                m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
            } else {
                m3 = m2;
            }
            if !((*m2).mc_flags & (*m3).mc_flags & 1 as libc::c_uint == 0) {
                if !(((*m3).mc_snum as libc::c_int) < (*mc).mc_snum as libc::c_int) {
                    if (*m3).mc_pg[(*mc).mc_top as usize] as libc::c_ulong
                        == mp as libc::c_ulong
                    {
                        if (*m3).mc_ki[(*mc).mc_top as usize] as libc::c_int
                            >= (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_int
                        {
                            if (*m3).mc_ki[(*mc).mc_top as usize] as libc::c_uint
                                >= nkeys
                            {
                                rc = mdb_cursor_sibling(m3, 1 as libc::c_int);
                                if rc == -(30798 as libc::c_int) {
                                    (*m3).mc_flags |= 2 as libc::c_uint;
                                    rc = 0 as libc::c_int;
                                    current_block = 2056236492220029646;
                                } else {
                                    if rc != 0 {
                                        current_block = 2490902864818026603;
                                        break;
                                    }
                                    current_block = 15594603006322722090;
                                }
                            } else {
                                current_block = 15594603006322722090;
                            }
                            match current_block {
                                2056236492220029646 => {}
                                _ => {
                                    if !((*m3).mc_xcursor).is_null() {
                                        if (*m3).mc_flags & 2 as libc::c_uint == 0 {
                                            node = ((*m3).mc_pg[(*m3).mc_top as usize]
                                                as *mut libc::c_char)
                                                .offset(
                                                    *((*(*m3).mc_pg[(*m3).mc_top as usize]).mp_ptrs)
                                                        .as_mut_ptr()
                                                        .offset((*m3).mc_ki[(*m3).mc_top as usize] as isize)
                                                        as libc::c_int as isize,
                                                )
                                                .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                                            if (*node).mn_flags as libc::c_int & 4 as libc::c_int != 0 {
                                                if (*(*m3).mc_xcursor).mx_cursor.mc_flags
                                                    & 1 as libc::c_uint != 0
                                                {
                                                    if (*node).mn_flags as libc::c_int & 2 as libc::c_int == 0 {
                                                        (*(*m3).mc_xcursor)
                                                            .mx_cursor
                                                            .mc_pg[0 as libc::c_int
                                                            as usize] = ((*node).mn_data)
                                                            .as_mut_ptr()
                                                            .offset((*node).mn_ksize as libc::c_int as isize)
                                                            as *mut libc::c_void as *mut MDB_page;
                                                    }
                                                } else {
                                                    mdb_xcursor_init1(m3, node);
                                                    rc = mdb_cursor_first(
                                                        &mut (*(*m3).mc_xcursor).mx_cursor,
                                                        0 as *mut libc::c_void as *mut MDB_val,
                                                        0 as *mut libc::c_void as *mut MDB_val,
                                                    );
                                                    if rc != 0 {
                                                        current_block = 2490902864818026603;
                                                        break;
                                                    }
                                                }
                                            }
                                            (*(*m3).mc_xcursor).mx_cursor.mc_flags |= 8 as libc::c_uint;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            m2 = (*m2).mc_next;
        }
        match current_block {
            2490902864818026603 => {}
            _ => {
                (*mc).mc_flags |= 8 as libc::c_uint;
            }
        }
    }
    if rc != 0 {
        (*(*mc).mc_txn).mt_flags |= 2 as libc::c_uint;
    }
    return rc;
}
pub unsafe extern "C" fn mdb_del(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    if key.is_null() {
        return 22 as libc::c_int
    } else {
        if !txn.is_null() {
            if dbi < (*txn).mt_numdbs {
                if *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int
                    & 16 as libc::c_int == 0
                {
                    return 22 as libc::c_int;
                }
            } else {
                return 22 as libc::c_int
            }
        } else {
            return 22 as libc::c_int
        }
    }
    if (*txn).mt_flags & 131091 as libc::c_uint != 0 {
        if (*txn).mt_flags & 131072 as libc::c_uint != 0 {
            tmp = 13 as libc::c_int;
        } else {
            tmp = -(30782 as libc::c_int);
        }
        return tmp;
    }
    if !((*((*txn).mt_dbs).offset(dbi as isize)).md_flags as libc::c_int
        & 4 as libc::c_int == 4 as libc::c_int)
    {
        data = 0 as *mut libc::c_void as *mut MDB_val;
    }
    tmp___0 = mdb_del0(txn, dbi, key, data, 0 as libc::c_uint);
    return tmp___0;
}
unsafe extern "C" fn mdb_del0(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut flags: libc::c_uint,
) -> libc::c_int {
    let mut mc: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut mx: MDB_xcursor = MDB_xcursor {
        mx_cursor: MDB_cursor {
            mc_next: 0 as *mut MDB_cursor,
            mc_backup: 0 as *mut MDB_cursor,
            mc_xcursor: 0 as *mut MDB_xcursor,
            mc_txn: 0 as *mut MDB_txn,
            mc_dbi: 0,
            mc_db: 0 as *mut MDB_db,
            mc_dbx: 0 as *mut MDB_dbx,
            mc_dbflag: 0 as *mut libc::c_uchar,
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [0 as *mut MDB_page; 32],
            mc_ki: [0; 32],
        },
        mx_db: MDB_db {
            md_pad: 0,
            md_flags: 0,
            md_depth: 0,
            md_branch_pages: 0,
            md_leaf_pages: 0,
            md_overflow_pages: 0,
            md_entries: 0,
            md_root: 0,
        },
        mx_dbx: MDB_dbx {
            md_name: MDB_val {
                mv_size: 0,
                mv_data: 0 as *mut libc::c_void,
            },
            md_cmp: None,
            md_dcmp: None,
            md_rel: None,
            md_relctx: 0 as *mut libc::c_void,
        },
        mx_dbflag: 0,
    };
    let mut op: MDB_cursor_op = MDB_FIRST;
    let mut rdata: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut xdata: *mut MDB_val = 0 as *mut MDB_val;
    let mut rc: libc::c_int = 0;
    let mut exact: libc::c_int = 0;
    exact = 0 as libc::c_int;
    mdb_cursor_init(&mut mc, txn, dbi, &mut mx);
    if !data.is_null() {
        op = MDB_GET_BOTH;
        rdata = *data;
        xdata = &mut rdata;
    } else {
        op = MDB_SET;
        xdata = 0 as *mut libc::c_void as *mut MDB_val;
        flags |= 32 as libc::c_uint;
    }
    rc = mdb_cursor_set(&mut mc, key, xdata, op, &mut exact);
    if rc == 0 as libc::c_int {
        mc.mc_next = *((*txn).mt_cursors).offset(dbi as isize);
        let ref mut fresh19 = *((*txn).mt_cursors).offset(dbi as isize);
        *fresh19 = &mut mc;
        rc = mdb_cursor_del(&mut mc, flags);
        let ref mut fresh20 = *((*txn).mt_cursors).offset(dbi as isize);
        *fresh20 = mc.mc_next;
    }
    return rc;
}
unsafe extern "C" fn mdb_page_split(
    mut mc: *mut MDB_cursor,
    mut newkey: *mut MDB_val,
    mut newdata: *mut MDB_val,
    mut newpgno: pgno_t,
    mut nflags: libc::c_uint,
) -> libc::c_int {
    let mut current_block: u64;
    let mut flags: libc::c_uint = 0;
    let mut rc: libc::c_int = 0;
    let mut new_root: libc::c_int = 0;
    let mut did_split: libc::c_int = 0;
    let mut newindx: indx_t = 0;
    let mut pgno: pgno_t = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut split_indx: libc::c_int = 0;
    let mut nkeys: libc::c_int = 0;
    let mut pmax: libc::c_int = 0;
    let mut env: *mut MDB_env = 0 as *mut MDB_env;
    let mut node: *mut MDB_node = 0 as *mut MDB_node;
    let mut sepkey: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut rkey: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut xdata: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut rdata: *mut MDB_val = 0 as *mut MDB_val;
    let mut copy: *mut MDB_page = 0 as *mut MDB_page;
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut rp: *mut MDB_page = 0 as *mut MDB_page;
    let mut pp: *mut MDB_page = 0 as *mut MDB_page;
    let mut ptop: libc::c_int = 0;
    let mut mn: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut tmp: uint16_t = 0;
    let mut split: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ins: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x: libc::c_int = 0;
    let mut lsize: libc::c_uint = 0;
    let mut rsize: libc::c_uint = 0;
    let mut ksize: libc::c_uint = 0;
    let mut psize: libc::c_int = 0;
    let mut nsize: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut snum: libc::c_int = 0;
    let mut dummy: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut tracked: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut tp: *mut *mut MDB_cursor = 0 as *mut *mut MDB_cursor;
    let mut tmp___4: size_t = 0;
    let mut m2: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut m3: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut dbi: MDB_dbi = 0;
    let mut k___0: libc::c_int = 0;
    let mut xr_pg: *mut MDB_page = 0 as *mut MDB_page;
    let mut xr_node: *mut MDB_node = 0 as *mut MDB_node;
    rc = 0 as libc::c_int;
    new_root = 0 as libc::c_int;
    did_split = 0 as libc::c_int;
    pgno = 0 as libc::c_int as pgno_t;
    env = (*(*mc).mc_txn).mt_env;
    rdata = &mut xdata;
    copy = 0 as *mut libc::c_void as *mut MDB_page;
    mp = (*mc).mc_pg[(*mc).mc_top as usize];
    newindx = (*mc).mc_ki[(*mc).mc_top as usize];
    nkeys = (((*mp).mp_pb.pb.pb_lower as libc::c_uint)
        .wrapping_sub(
            &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                as libc::c_uint,
        ) >> 1 as libc::c_int) as libc::c_int;
    rc = mdb_page_new(mc, (*mp).mp_flags as uint32_t, 1 as libc::c_int, &mut rp);
    if rc != 0 {
        return rc;
    }
    (*rp).mp_pad = (*mp).mp_pad;
    if ((*mc).mc_top as libc::c_int) < 1 as libc::c_int {
        rc = mdb_page_new(mc, 1 as libc::c_int as uint32_t, 1 as libc::c_int, &mut pp);
        if rc != 0 {
            current_block = 7830855388708405077;
        } else {
            i = (*mc).mc_snum as libc::c_int;
            while i > 0 as libc::c_int {
                (*mc).mc_pg[i as usize] = (*mc).mc_pg[(i - 1 as libc::c_int) as usize];
                (*mc).mc_ki[i as usize] = (*mc).mc_ki[(i - 1 as libc::c_int) as usize];
                i -= 1;
            }
            (*mc).mc_pg[0 as libc::c_int as usize] = pp;
            (*mc).mc_ki[0 as libc::c_int as usize] = 0 as libc::c_int as indx_t;
            (*(*mc).mc_db).md_root = (*pp).mp_p.p_pgno;
            tmp = (*(*mc).mc_db).md_depth;
            (*(*mc).mc_db)
                .md_depth = ((*(*mc).mc_db).md_depth as libc::c_int + 1 as libc::c_int)
                as uint16_t;
            new_root = tmp as libc::c_int;
            rc = mdb_node_add(
                mc,
                0 as libc::c_int as indx_t,
                0 as *mut libc::c_void as *mut MDB_val,
                0 as *mut libc::c_void as *mut MDB_val,
                (*mp).mp_p.p_pgno,
                0 as libc::c_uint,
            );
            if rc != 0 as libc::c_int {
                (*mc)
                    .mc_pg[0 as libc::c_int
                    as usize] = (*mc).mc_pg[1 as libc::c_int as usize];
                (*mc)
                    .mc_ki[0 as libc::c_int
                    as usize] = (*mc).mc_ki[1 as libc::c_int as usize];
                (*(*mc).mc_db).md_root = (*mp).mp_p.p_pgno;
                (*(*mc).mc_db)
                    .md_depth = ((*(*mc).mc_db).md_depth as libc::c_int
                    - 1 as libc::c_int) as uint16_t;
                current_block = 7830855388708405077;
            } else {
                (*mc)
                    .mc_snum = ((*mc).mc_snum as libc::c_int + 1 as libc::c_int)
                    as libc::c_ushort;
                (*mc)
                    .mc_top = ((*mc).mc_top as libc::c_int + 1 as libc::c_int)
                    as libc::c_ushort;
                ptop = 0 as libc::c_int;
                current_block = 10150597327160359210;
            }
        }
    } else {
        ptop = (*mc).mc_top as libc::c_int - 1 as libc::c_int;
        current_block = 10150597327160359210;
    }
    match current_block {
        10150597327160359210 => {
            mdb_cursor_copy(mc as *const MDB_cursor, &mut mn);
            mn.mc_xcursor = 0 as *mut libc::c_void as *mut MDB_xcursor;
            mn.mc_pg[mn.mc_top as usize] = rp;
            mn
                .mc_ki[ptop
                as usize] = ((*mc).mc_ki[ptop as usize] as libc::c_int
                + 1 as libc::c_int) as indx_t;
            if nflags & 131072 as libc::c_uint != 0 {
                mn.mc_ki[mn.mc_top as usize] = 0 as libc::c_int as indx_t;
                sepkey = *newkey;
                split_indx = newindx as libc::c_int;
                nkeys = 0 as libc::c_int;
                current_block = 15614898248724990345;
            } else {
                split_indx = (nkeys + 1 as libc::c_int) / 2 as libc::c_int;
                if (*rp).mp_flags as libc::c_int & 32 as libc::c_int == 32 as libc::c_int
                {
                    x = (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_int - split_indx;
                    ksize = (*(*mc).mc_db).md_pad;
                    split = (mp as *mut libc::c_char)
                        .offset(
                            &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                as libc::c_ulong as libc::c_uint as isize,
                        )
                        .offset(
                            (split_indx as libc::c_uint).wrapping_mul(ksize) as isize,
                        );
                    rsize = ((nkeys - split_indx) as libc::c_uint).wrapping_mul(ksize);
                    lsize = ((nkeys - split_indx) as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<indx_t>() as libc::c_ulong)
                        as libc::c_uint;
                    (*mp)
                        .mp_pb
                        .pb
                        .pb_lower = ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                        .wrapping_sub(lsize) as indx_t;
                    (*rp)
                        .mp_pb
                        .pb
                        .pb_lower = ((*rp).mp_pb.pb.pb_lower as libc::c_uint)
                        .wrapping_add(lsize) as indx_t;
                    (*mp)
                        .mp_pb
                        .pb
                        .pb_upper = ((*mp).mp_pb.pb.pb_upper as libc::c_uint)
                        .wrapping_add(rsize.wrapping_sub(lsize)) as indx_t;
                    (*rp)
                        .mp_pb
                        .pb
                        .pb_upper = ((*rp).mp_pb.pb.pb_upper as libc::c_uint)
                        .wrapping_sub(rsize.wrapping_sub(lsize)) as indx_t;
                    sepkey.mv_size = ksize as size_t;
                    if newindx as libc::c_int == split_indx {
                        sepkey.mv_data = (*newkey).mv_data;
                    } else {
                        sepkey.mv_data = split as *mut libc::c_void;
                    }
                    if x < 0 as libc::c_int {
                        ins = (mp as *mut libc::c_char)
                            .offset(
                                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                    as libc::c_ulong as libc::c_uint as isize,
                            )
                            .offset(
                                ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint)
                                    .wrapping_mul(ksize) as isize,
                            );
                        memcpy(
                            ((*rp).mp_ptrs).as_mut_ptr() as *mut libc::c_void,
                            split as *const libc::c_void,
                            rsize as size_t,
                        );
                        sepkey
                            .mv_data = ((*rp).mp_ptrs).as_mut_ptr() as *mut libc::c_void;
                        memmove(
                            ins.offset(ksize as isize) as *mut libc::c_void,
                            ins as *const libc::c_void,
                            ((split_indx
                                - (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_int)
                                as libc::c_uint)
                                .wrapping_mul(ksize) as size_t,
                        );
                        memcpy(
                            ins as *mut libc::c_void,
                            (*newkey).mv_data as *const libc::c_void,
                            ksize as size_t,
                        );
                        (*mp)
                            .mp_pb
                            .pb
                            .pb_lower = ((*mp).mp_pb.pb.pb_lower as libc::c_ulong)
                            .wrapping_add(
                                ::std::mem::size_of::<indx_t>() as libc::c_ulong,
                            ) as indx_t;
                        (*mp)
                            .mp_pb
                            .pb
                            .pb_upper = ((*mp).mp_pb.pb.pb_upper as libc::c_ulong)
                            .wrapping_sub(
                                (ksize as libc::c_ulong)
                                    .wrapping_sub(
                                        ::std::mem::size_of::<indx_t>() as libc::c_ulong,
                                    ),
                            ) as indx_t;
                    } else {
                        if x != 0 {
                            memcpy(
                                ((*rp).mp_ptrs).as_mut_ptr() as *mut libc::c_void,
                                split as *const libc::c_void,
                                (x as libc::c_uint).wrapping_mul(ksize) as size_t,
                            );
                        }
                        ins = (rp as *mut libc::c_char)
                            .offset(
                                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                    as libc::c_ulong as libc::c_uint as isize,
                            )
                            .offset((x as libc::c_uint).wrapping_mul(ksize) as isize);
                        memcpy(
                            ins as *mut libc::c_void,
                            (*newkey).mv_data as *const libc::c_void,
                            ksize as size_t,
                        );
                        memcpy(
                            ins.offset(ksize as isize) as *mut libc::c_void,
                            split
                                .offset((x as libc::c_uint).wrapping_mul(ksize) as isize)
                                as *const libc::c_void,
                            rsize.wrapping_sub((x as libc::c_uint).wrapping_mul(ksize))
                                as size_t,
                        );
                        (*rp)
                            .mp_pb
                            .pb
                            .pb_lower = ((*rp).mp_pb.pb.pb_lower as libc::c_ulong)
                            .wrapping_add(
                                ::std::mem::size_of::<indx_t>() as libc::c_ulong,
                            ) as indx_t;
                        (*rp)
                            .mp_pb
                            .pb
                            .pb_upper = ((*rp).mp_pb.pb.pb_upper as libc::c_ulong)
                            .wrapping_sub(
                                (ksize as libc::c_ulong)
                                    .wrapping_sub(
                                        ::std::mem::size_of::<indx_t>() as libc::c_ulong,
                                    ),
                            ) as indx_t;
                        (*mc).mc_ki[(*mc).mc_top as usize] = x as indx_t;
                    }
                    current_block = 15614898248724990345;
                } else {
                    pmax = ((*env).me_psize)
                        .wrapping_sub(
                            &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                as libc::c_ulong as libc::c_uint,
                        ) as libc::c_int;
                    if (*mp).mp_flags as libc::c_int & 2 as libc::c_int
                        == 2 as libc::c_int
                    {
                        tmp___0 = mdb_leaf_size(env, newkey, newdata);
                        nsize = tmp___0 as libc::c_int;
                    } else {
                        tmp___1 = mdb_branch_size(env, newkey);
                        nsize = tmp___1 as libc::c_int;
                    }
                    nsize = ((nsize as libc::c_uint).wrapping_add(1 as libc::c_uint)
                        & 4294967294 as libc::c_uint) as libc::c_int;
                    copy = mdb_page_malloc((*mc).mc_txn, 1 as libc::c_uint);
                    if copy as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                        rc = 12 as libc::c_int;
                        current_block = 7830855388708405077;
                    } else {
                        (*copy).mp_p.p_pgno = (*mp).mp_p.p_pgno;
                        (*copy).mp_flags = (*mp).mp_flags;
                        (*copy)
                            .mp_pb
                            .pb
                            .pb_lower = &mut (*(0 as *mut MDB_page)).mp_ptrs
                            as *mut [indx_t; 1] as libc::c_ulong as libc::c_uint
                            as indx_t;
                        (*copy).mp_pb.pb.pb_upper = (*env).me_psize as indx_t;
                        i = 0 as libc::c_int;
                        j = 0 as libc::c_int;
                        while i < nkeys {
                            if i == newindx as libc::c_int {
                                tmp___2 = j;
                                j += 1;
                                *((*copy).mp_ptrs)
                                    .as_mut_ptr()
                                    .offset(tmp___2 as isize) = 0 as libc::c_int as indx_t;
                            }
                            tmp___3 = j;
                            j += 1;
                            *((*copy).mp_ptrs)
                                .as_mut_ptr()
                                .offset(
                                    tmp___3 as isize,
                                ) = *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize);
                            i += 1;
                        }
                        if nkeys < 32 as libc::c_int {
                            current_block = 2918026538399382141;
                        } else if nsize > pmax / 16 as libc::c_int {
                            current_block = 2918026538399382141;
                        } else if newindx as libc::c_int >= nkeys {
                            current_block = 2918026538399382141;
                        } else {
                            current_block = 13425230902034816933;
                        }
                        match current_block {
                            2918026538399382141 => {
                                psize = 0 as libc::c_int;
                                let mut current_block_122: u64;
                                if newindx as libc::c_int <= split_indx {
                                    current_block_122 = 5136574189917220510;
                                } else if newindx as libc::c_int >= nkeys {
                                    current_block_122 = 5136574189917220510;
                                } else {
                                    i = nkeys;
                                    j = -(1 as libc::c_int);
                                    k = split_indx - 1 as libc::c_int;
                                    current_block_122 = 16778110326724371720;
                                }
                                match current_block_122 {
                                    5136574189917220510 => {
                                        i = 0 as libc::c_int;
                                        j = 1 as libc::c_int;
                                        if newindx as libc::c_int >= nkeys {
                                            k = nkeys;
                                        } else {
                                            k = split_indx + 1 as libc::c_int
                                                + ((*mp).mp_flags as libc::c_int & 2 as libc::c_int
                                                    == 2 as libc::c_int) as libc::c_int;
                                        }
                                    }
                                    _ => {}
                                }
                                while i != k {
                                    if i == newindx as libc::c_int {
                                        psize += nsize;
                                        node = 0 as *mut libc::c_void as *mut MDB_node;
                                    } else {
                                        node = (mp as *mut libc::c_char)
                                            .offset(
                                                *((*copy).mp_ptrs).as_mut_ptr().offset(i as isize)
                                                    as libc::c_int as isize,
                                            )
                                            .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                                        psize = (psize as libc::c_ulong)
                                            .wrapping_add(
                                                (&mut (*(0 as *mut MDB_node)).mn_data
                                                    as *mut [libc::c_char; 1] as libc::c_ulong)
                                                    .wrapping_add((*node).mn_ksize as libc::c_ulong)
                                                    .wrapping_add(
                                                        ::std::mem::size_of::<indx_t>() as libc::c_ulong,
                                                    ),
                                            ) as libc::c_int;
                                        if (*mp).mp_flags as libc::c_int & 2 as libc::c_int
                                            == 2 as libc::c_int
                                        {
                                            if (*node).mn_flags as libc::c_int & 1 as libc::c_int
                                                == 1 as libc::c_int
                                            {
                                                psize = (psize as libc::c_ulong)
                                                    .wrapping_add(
                                                        ::std::mem::size_of::<pgno_t>() as libc::c_ulong,
                                                    ) as libc::c_int;
                                            } else {
                                                psize = (psize as libc::c_uint)
                                                    .wrapping_add(
                                                        (*node).mn_lo as libc::c_uint
                                                            | ((*node).mn_hi as libc::c_uint) << 16 as libc::c_int,
                                                    ) as libc::c_int;
                                            }
                                        }
                                        psize = ((psize as libc::c_uint)
                                            .wrapping_add(1 as libc::c_uint)
                                            & 4294967294 as libc::c_uint) as libc::c_int;
                                    }
                                    if psize > pmax {
                                        split_indx = i + (j < 0 as libc::c_int) as libc::c_int;
                                        break;
                                    } else if i == k - j {
                                        split_indx = i + (j < 0 as libc::c_int) as libc::c_int;
                                        break;
                                    } else {
                                        i += j;
                                    }
                                }
                            }
                            _ => {}
                        }
                        if split_indx == newindx as libc::c_int {
                            sepkey.mv_size = (*newkey).mv_size;
                            sepkey.mv_data = (*newkey).mv_data;
                        } else {
                            node = (mp as *mut libc::c_char)
                                .offset(
                                    *((*copy).mp_ptrs).as_mut_ptr().offset(split_indx as isize)
                                        as libc::c_int as isize,
                                )
                                .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                            sepkey.mv_size = (*node).mn_ksize as size_t;
                            sepkey
                                .mv_data = ((*node).mn_data).as_mut_ptr()
                                as *mut libc::c_void;
                        }
                        current_block = 15614898248724990345;
                    }
                }
            }
            match current_block {
                7830855388708405077 => {}
                _ => {
                    tmp___4 = mdb_branch_size(env, &mut sepkey);
                    if (((*mn.mc_pg[ptop as usize]).mp_pb.pb.pb_upper as libc::c_int
                        - (*mn.mc_pg[ptop as usize]).mp_pb.pb.pb_lower as libc::c_int)
                        as indx_t as size_t) < tmp___4
                    {
                        snum = (*mc).mc_snum as libc::c_int;
                        mn
                            .mc_snum = (mn.mc_snum as libc::c_int - 1 as libc::c_int)
                            as libc::c_ushort;
                        mn
                            .mc_top = (mn.mc_top as libc::c_int - 1 as libc::c_int)
                            as libc::c_ushort;
                        did_split = 1 as libc::c_int;
                        tp = ((*mn.mc_txn).mt_cursors).offset(mn.mc_dbi as isize);
                        if mn.mc_flags & 4 as libc::c_uint != 0 {
                            dummy.mc_flags = 1 as libc::c_uint;
                            dummy
                                .mc_xcursor = &mut mn as *mut MDB_cursor
                                as *mut MDB_xcursor;
                            tracked = &mut dummy;
                        } else {
                            tracked = &mut mn;
                        }
                        (*tracked).mc_next = *tp;
                        *tp = tracked;
                        rc = mdb_page_split(
                            &mut mn,
                            &mut sepkey,
                            0 as *mut libc::c_void as *mut MDB_val,
                            (*rp).mp_p.p_pgno,
                            0 as libc::c_uint,
                        );
                        *tp = (*tracked).mc_next;
                        if rc != 0 {
                            current_block = 7830855388708405077;
                        } else {
                            if (*mc).mc_snum as libc::c_int > snum {
                                ptop += 1;
                            }
                            if mn.mc_pg[ptop as usize] as libc::c_ulong
                                != (*mc).mc_pg[ptop as usize] as libc::c_ulong
                            {
                                if (*mc).mc_ki[ptop as usize] as libc::c_uint
                                    >= ((*(*mc).mc_pg[ptop as usize]).mp_pb.pb.pb_lower
                                        as libc::c_uint)
                                        .wrapping_sub(
                                            &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                                as libc::c_ulong as libc::c_uint,
                                        ) >> 1 as libc::c_int
                                {
                                    i = 0 as libc::c_int;
                                    while i < ptop {
                                        (*mc).mc_pg[i as usize] = mn.mc_pg[i as usize];
                                        (*mc).mc_ki[i as usize] = mn.mc_ki[i as usize];
                                        i += 1;
                                    }
                                    (*mc).mc_pg[ptop as usize] = mn.mc_pg[ptop as usize];
                                    if mn.mc_ki[ptop as usize] != 0 {
                                        (*mc)
                                            .mc_ki[ptop
                                            as usize] = (mn.mc_ki[ptop as usize] as libc::c_int
                                            - 1 as libc::c_int) as indx_t;
                                    } else {
                                        (*mc).mc_ki[ptop as usize] = mn.mc_ki[ptop as usize];
                                        rc = mdb_cursor_sibling(mc, 0 as libc::c_int);
                                    }
                                }
                            }
                            current_block = 10996290961880923853;
                        }
                    } else {
                        mn
                            .mc_top = (mn.mc_top as libc::c_int - 1 as libc::c_int)
                            as libc::c_ushort;
                        rc = mdb_node_add(
                            &mut mn,
                            mn.mc_ki[ptop as usize],
                            &mut sepkey,
                            0 as *mut libc::c_void as *mut MDB_val,
                            (*rp).mp_p.p_pgno,
                            0 as libc::c_uint,
                        );
                        mn
                            .mc_top = (mn.mc_top as libc::c_int + 1 as libc::c_int)
                            as libc::c_ushort;
                        current_block = 10996290961880923853;
                    }
                    match current_block {
                        7830855388708405077 => {}
                        _ => {
                            if rc != 0 as libc::c_int {
                                if rc == -(30798 as libc::c_int) {
                                    rc = -(30779 as libc::c_int);
                                }
                            } else {
                                if nflags & 131072 as libc::c_uint != 0 {
                                    (*mc).mc_pg[(*mc).mc_top as usize] = rp;
                                    (*mc)
                                        .mc_ki[(*mc).mc_top as usize] = 0 as libc::c_int as indx_t;
                                    rc = mdb_node_add(
                                        mc,
                                        0 as libc::c_int as indx_t,
                                        newkey,
                                        newdata,
                                        newpgno,
                                        nflags,
                                    );
                                    if rc != 0 {
                                        current_block = 7830855388708405077;
                                    } else {
                                        i = 0 as libc::c_int;
                                        while i < (*mc).mc_top as libc::c_int {
                                            (*mc).mc_ki[i as usize] = mn.mc_ki[i as usize];
                                            i += 1;
                                        }
                                        current_block = 3677748771821733454;
                                    }
                                } else if !((*mp).mp_flags as libc::c_int
                                        & 32 as libc::c_int == 32 as libc::c_int)
                                    {
                                    (*mc).mc_pg[(*mc).mc_top as usize] = rp;
                                    i = split_indx;
                                    j = 0 as libc::c_int;
                                    loop {
                                        if i == newindx as libc::c_int {
                                            rkey.mv_data = (*newkey).mv_data;
                                            rkey.mv_size = (*newkey).mv_size;
                                            if (*mp).mp_flags as libc::c_int & 2 as libc::c_int
                                                == 2 as libc::c_int
                                            {
                                                rdata = newdata;
                                            } else {
                                                pgno = newpgno;
                                            }
                                            flags = nflags;
                                            (*mc).mc_ki[(*mc).mc_top as usize] = j as indx_t;
                                        } else {
                                            node = (mp as *mut libc::c_char)
                                                .offset(
                                                    *((*copy).mp_ptrs).as_mut_ptr().offset(i as isize)
                                                        as libc::c_int as isize,
                                                )
                                                .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                                            rkey
                                                .mv_data = ((*node).mn_data).as_mut_ptr()
                                                as *mut libc::c_void;
                                            rkey.mv_size = (*node).mn_ksize as size_t;
                                            if (*mp).mp_flags as libc::c_int & 2 as libc::c_int
                                                == 2 as libc::c_int
                                            {
                                                xdata
                                                    .mv_data = ((*node).mn_data)
                                                    .as_mut_ptr()
                                                    .offset((*node).mn_ksize as libc::c_int as isize)
                                                    as *mut libc::c_void;
                                                xdata
                                                    .mv_size = ((*node).mn_lo as libc::c_uint
                                                    | ((*node).mn_hi as libc::c_uint) << 16 as libc::c_int)
                                                    as size_t;
                                                rdata = &mut xdata;
                                            } else {
                                                pgno = (*node).mn_lo as libc::c_ulong
                                                    | ((*node).mn_hi as pgno_t) << 16 as libc::c_int
                                                    | ((*node).mn_flags as pgno_t) << 32 as libc::c_int;
                                            }
                                            flags = (*node).mn_flags as libc::c_uint;
                                        }
                                        if !((*mp).mp_flags as libc::c_int & 2 as libc::c_int
                                            == 2 as libc::c_int)
                                        {
                                            if j == 0 as libc::c_int {
                                                rkey.mv_size = 0 as libc::c_int as size_t;
                                            }
                                        }
                                        rc = mdb_node_add(
                                            mc,
                                            j as indx_t,
                                            &mut rkey,
                                            rdata,
                                            pgno,
                                            flags,
                                        );
                                        if rc != 0 {
                                            current_block = 7830855388708405077;
                                            break;
                                        }
                                        if i == nkeys {
                                            i = 0 as libc::c_int;
                                            j = 0 as libc::c_int;
                                            (*mc).mc_pg[(*mc).mc_top as usize] = copy;
                                        } else {
                                            i += 1;
                                            j += 1;
                                        }
                                        if !(i != split_indx) {
                                            current_block = 10528013381497917728;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        7830855388708405077 => {}
                                        _ => {
                                            nkeys = (((*copy).mp_pb.pb.pb_lower as libc::c_uint)
                                                .wrapping_sub(
                                                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                                        as libc::c_ulong as libc::c_uint,
                                                ) >> 1 as libc::c_int) as libc::c_int;
                                            i = 0 as libc::c_int;
                                            while i < nkeys {
                                                *((*mp).mp_ptrs)
                                                    .as_mut_ptr()
                                                    .offset(
                                                        i as isize,
                                                    ) = *((*copy).mp_ptrs).as_mut_ptr().offset(i as isize);
                                                i += 1;
                                            }
                                            (*mp).mp_pb.pb.pb_lower = (*copy).mp_pb.pb.pb_lower;
                                            (*mp).mp_pb.pb.pb_upper = (*copy).mp_pb.pb.pb_upper;
                                            memcpy(
                                                (mp as *mut libc::c_char)
                                                    .offset(
                                                        *((*mp).mp_ptrs)
                                                            .as_mut_ptr()
                                                            .offset((nkeys - 1 as libc::c_int) as isize) as libc::c_int
                                                            as isize,
                                                    )
                                                    .offset(0 as libc::c_uint as isize) as *mut MDB_node
                                                    as *mut libc::c_void,
                                                (copy as *mut libc::c_char)
                                                    .offset(
                                                        *((*copy).mp_ptrs)
                                                            .as_mut_ptr()
                                                            .offset((nkeys - 1 as libc::c_int) as isize) as libc::c_int
                                                            as isize,
                                                    )
                                                    .offset(0 as libc::c_uint as isize) as *mut MDB_node
                                                    as *const libc::c_void,
                                                ((*env).me_psize)
                                                    .wrapping_sub((*copy).mp_pb.pb.pb_upper as libc::c_uint)
                                                    as size_t,
                                            );
                                            if (newindx as libc::c_int) < split_indx {
                                                (*mc).mc_pg[(*mc).mc_top as usize] = mp;
                                            } else {
                                                (*mc).mc_pg[(*mc).mc_top as usize] = rp;
                                                (*mc)
                                                    .mc_ki[ptop
                                                    as usize] = ((*mc).mc_ki[ptop as usize] as libc::c_int
                                                    + 1 as libc::c_int) as indx_t;
                                                if mn.mc_pg[ptop as usize] as libc::c_ulong
                                                    != (*mc).mc_pg[ptop as usize] as libc::c_ulong
                                                {
                                                    if (*mc).mc_ki[ptop as usize] as libc::c_uint
                                                        >= ((*(*mc).mc_pg[ptop as usize]).mp_pb.pb.pb_lower
                                                            as libc::c_uint)
                                                            .wrapping_sub(
                                                                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                                                    as libc::c_ulong as libc::c_uint,
                                                            ) >> 1 as libc::c_int
                                                    {
                                                        i = 0 as libc::c_int;
                                                        while i <= ptop {
                                                            (*mc).mc_pg[i as usize] = mn.mc_pg[i as usize];
                                                            (*mc).mc_ki[i as usize] = mn.mc_ki[i as usize];
                                                            i += 1;
                                                        }
                                                    }
                                                }
                                            }
                                            if nflags & 65536 as libc::c_uint != 0 {
                                                node = ((*mc).mc_pg[(*mc).mc_top as usize]
                                                    as *mut libc::c_char)
                                                    .offset(
                                                        *((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_ptrs)
                                                            .as_mut_ptr()
                                                            .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                                                            as libc::c_int as isize,
                                                    )
                                                    .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                                                if (*node).mn_flags as libc::c_int & 1 as libc::c_int == 0 {
                                                    (*newdata)
                                                        .mv_data = ((*node).mn_data)
                                                        .as_mut_ptr()
                                                        .offset((*node).mn_ksize as libc::c_int as isize)
                                                        as *mut libc::c_void;
                                                }
                                            }
                                            current_block = 3677748771821733454;
                                        }
                                    }
                                } else {
                                    if newindx as libc::c_int >= split_indx {
                                        (*mc).mc_pg[(*mc).mc_top as usize] = rp;
                                        (*mc)
                                            .mc_ki[ptop
                                            as usize] = ((*mc).mc_ki[ptop as usize] as libc::c_int
                                            + 1 as libc::c_int) as indx_t;
                                        if mn.mc_pg[ptop as usize] as libc::c_ulong
                                            != (*mc).mc_pg[ptop as usize] as libc::c_ulong
                                        {
                                            if (*mc).mc_ki[ptop as usize] as libc::c_uint
                                                >= ((*(*mc).mc_pg[ptop as usize]).mp_pb.pb.pb_lower
                                                    as libc::c_uint)
                                                    .wrapping_sub(
                                                        &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                                            as libc::c_ulong as libc::c_uint,
                                                    ) >> 1 as libc::c_int
                                            {
                                                i = 0 as libc::c_int;
                                                while i <= ptop {
                                                    (*mc).mc_pg[i as usize] = mn.mc_pg[i as usize];
                                                    (*mc).mc_ki[i as usize] = mn.mc_ki[i as usize];
                                                    i += 1;
                                                }
                                            }
                                        }
                                    }
                                    current_block = 3677748771821733454;
                                }
                                match current_block {
                                    7830855388708405077 => {}
                                    _ => {
                                        dbi = (*mc).mc_dbi;
                                        nkeys = (((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                                            .wrapping_sub(
                                                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                                    as libc::c_ulong as libc::c_uint,
                                            ) >> 1 as libc::c_int) as libc::c_int;
                                        m2 = *((*(*mc).mc_txn).mt_cursors).offset(dbi as isize);
                                        while !m2.is_null() {
                                            if (*mc).mc_flags & 4 as libc::c_uint != 0 {
                                                m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
                                            } else {
                                                m3 = m2;
                                            }
                                            if !(m3 as libc::c_ulong == mc as libc::c_ulong) {
                                                if !((*m2).mc_flags & (*m3).mc_flags & 1 as libc::c_uint
                                                    == 0)
                                                {
                                                    if new_root != 0 {
                                                        if (*m3).mc_pg[0 as libc::c_int as usize] as libc::c_ulong
                                                            != mp as libc::c_ulong
                                                        {
                                                            current_block = 15914853135630552058;
                                                        } else {
                                                            k___0 = new_root;
                                                            while k___0 >= 0 as libc::c_int {
                                                                (*m3)
                                                                    .mc_ki[(k___0 + 1 as libc::c_int)
                                                                    as usize] = (*m3).mc_ki[k___0 as usize];
                                                                (*m3)
                                                                    .mc_pg[(k___0 + 1 as libc::c_int)
                                                                    as usize] = (*m3).mc_pg[k___0 as usize];
                                                                k___0 -= 1;
                                                            }
                                                            if (*m3).mc_ki[0 as libc::c_int as usize] as libc::c_int
                                                                >= nkeys
                                                            {
                                                                (*m3)
                                                                    .mc_ki[0 as libc::c_int
                                                                    as usize] = 1 as libc::c_int as indx_t;
                                                            } else {
                                                                (*m3)
                                                                    .mc_ki[0 as libc::c_int
                                                                    as usize] = 0 as libc::c_int as indx_t;
                                                            }
                                                            (*m3)
                                                                .mc_pg[0 as libc::c_int
                                                                as usize] = (*mc).mc_pg[0 as libc::c_int as usize];
                                                            (*m3)
                                                                .mc_snum = ((*m3).mc_snum as libc::c_int + 1 as libc::c_int)
                                                                as libc::c_ushort;
                                                            (*m3)
                                                                .mc_top = ((*m3).mc_top as libc::c_int + 1 as libc::c_int)
                                                                as libc::c_ushort;
                                                            current_block = 15517153985884127030;
                                                        }
                                                    } else {
                                                        current_block = 15517153985884127030;
                                                    }
                                                    match current_block {
                                                        15914853135630552058 => {}
                                                        _ => {
                                                            let mut current_block_335: u64;
                                                            if (*m3).mc_top as libc::c_int
                                                                >= (*mc).mc_top as libc::c_int
                                                            {
                                                                if (*m3).mc_pg[(*mc).mc_top as usize] as libc::c_ulong
                                                                    == mp as libc::c_ulong
                                                                {
                                                                    if (*m3).mc_ki[(*mc).mc_top as usize] as libc::c_int
                                                                        >= newindx as libc::c_int
                                                                    {
                                                                        if nflags & 262144 as libc::c_uint == 0 {
                                                                            (*m3)
                                                                                .mc_ki[(*mc).mc_top
                                                                                as usize] = ((*m3).mc_ki[(*mc).mc_top as usize]
                                                                                as libc::c_int + 1 as libc::c_int) as indx_t;
                                                                        }
                                                                    }
                                                                    if (*m3).mc_ki[(*mc).mc_top as usize] as libc::c_int
                                                                        >= nkeys
                                                                    {
                                                                        (*m3).mc_pg[(*mc).mc_top as usize] = rp;
                                                                        (*m3)
                                                                            .mc_ki[(*mc).mc_top
                                                                            as usize] = ((*m3).mc_ki[(*mc).mc_top as usize]
                                                                            as libc::c_int - nkeys) as indx_t;
                                                                        i = 0 as libc::c_int;
                                                                        while i < (*mc).mc_top as libc::c_int {
                                                                            (*m3).mc_ki[i as usize] = mn.mc_ki[i as usize];
                                                                            (*m3).mc_pg[i as usize] = mn.mc_pg[i as usize];
                                                                            i += 1;
                                                                        }
                                                                    }
                                                                    current_block_335 = 13894589077314508493;
                                                                } else {
                                                                    current_block_335 = 11209382128811362333;
                                                                }
                                                            } else {
                                                                current_block_335 = 11209382128811362333;
                                                            }
                                                            match current_block_335 {
                                                                11209382128811362333 => {
                                                                    if did_split == 0 {
                                                                        if (*m3).mc_top as libc::c_int >= ptop {
                                                                            if (*m3).mc_pg[ptop as usize] as libc::c_ulong
                                                                                == (*mc).mc_pg[ptop as usize] as libc::c_ulong
                                                                            {
                                                                                if (*m3).mc_ki[ptop as usize] as libc::c_int
                                                                                    >= (*mc).mc_ki[ptop as usize] as libc::c_int
                                                                                {
                                                                                    (*m3)
                                                                                        .mc_ki[ptop
                                                                                        as usize] = ((*m3).mc_ki[ptop as usize] as libc::c_int
                                                                                        + 1 as libc::c_int) as indx_t;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                _ => {}
                                                            }
                                                            if (*mp).mp_flags as libc::c_int & 2 as libc::c_int
                                                                == 2 as libc::c_int
                                                            {
                                                                xr_pg = (*m3).mc_pg[(*mc).mc_top as usize];
                                                                if !((*m3).mc_xcursor).is_null() {
                                                                    if (*(*m3).mc_xcursor).mx_cursor.mc_flags
                                                                        & 1 as libc::c_uint != 0
                                                                    {
                                                                        if !((*m3).mc_ki[(*mc).mc_top as usize] as libc::c_uint
                                                                            >= ((*xr_pg).mp_pb.pb.pb_lower as libc::c_uint)
                                                                                .wrapping_sub(
                                                                                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                                                                        as libc::c_ulong as libc::c_uint,
                                                                                ) >> 1 as libc::c_int)
                                                                        {
                                                                            xr_node = (xr_pg as *mut libc::c_char)
                                                                                .offset(
                                                                                    *((*xr_pg).mp_ptrs)
                                                                                        .as_mut_ptr()
                                                                                        .offset((*m3).mc_ki[(*mc).mc_top as usize] as isize)
                                                                                        as libc::c_int as isize,
                                                                                )
                                                                                .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                                                                            if (*xr_node).mn_flags as libc::c_int & 6 as libc::c_int
                                                                                == 4 as libc::c_int
                                                                            {
                                                                                (*(*m3).mc_xcursor)
                                                                                    .mx_cursor
                                                                                    .mc_pg[0 as libc::c_int
                                                                                    as usize] = ((*xr_node).mn_data)
                                                                                    .as_mut_ptr()
                                                                                    .offset((*xr_node).mn_ksize as libc::c_int as isize)
                                                                                    as *mut libc::c_void as *mut MDB_page;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            m2 = (*m2).mc_next;
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
    if !copy.is_null() {
        mdb_page_free(env, copy);
    }
    if rc != 0 {
        (*(*mc).mc_txn).mt_flags |= 2 as libc::c_uint;
    }
    return rc;
}
pub unsafe extern "C" fn mdb_put(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut flags: libc::c_uint,
) -> libc::c_int {
    let mut mc: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut mx: MDB_xcursor = MDB_xcursor {
        mx_cursor: MDB_cursor {
            mc_next: 0 as *mut MDB_cursor,
            mc_backup: 0 as *mut MDB_cursor,
            mc_xcursor: 0 as *mut MDB_xcursor,
            mc_txn: 0 as *mut MDB_txn,
            mc_dbi: 0,
            mc_db: 0 as *mut MDB_db,
            mc_dbx: 0 as *mut MDB_dbx,
            mc_dbflag: 0 as *mut libc::c_uchar,
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [0 as *mut MDB_page; 32],
            mc_ki: [0; 32],
        },
        mx_db: MDB_db {
            md_pad: 0,
            md_flags: 0,
            md_depth: 0,
            md_branch_pages: 0,
            md_leaf_pages: 0,
            md_overflow_pages: 0,
            md_entries: 0,
            md_root: 0,
        },
        mx_dbx: MDB_dbx {
            md_name: MDB_val {
                mv_size: 0,
                mv_data: 0 as *mut libc::c_void,
            },
            md_cmp: None,
            md_dcmp: None,
            md_rel: None,
            md_relctx: 0 as *mut libc::c_void,
        },
        mx_dbflag: 0,
    };
    let mut rc: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    if key.is_null() {
        return 22 as libc::c_int
    } else {
        if data.is_null() {
            return 22 as libc::c_int
        } else {
            if !txn.is_null() {
                if dbi < (*txn).mt_numdbs {
                    if *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int
                        & 16 as libc::c_int == 0
                    {
                        return 22 as libc::c_int;
                    }
                } else {
                    return 22 as libc::c_int
                }
            } else {
                return 22 as libc::c_int
            }
        }
    }
    if flags & 4294508495 as libc::c_uint != 0 {
        return 22 as libc::c_int;
    }
    if (*txn).mt_flags & 131091 as libc::c_uint != 0 {
        if (*txn).mt_flags & 131072 as libc::c_uint != 0 {
            tmp = 13 as libc::c_int;
        } else {
            tmp = -(30782 as libc::c_int);
        }
        return tmp;
    }
    mdb_cursor_init(&mut mc, txn, dbi, &mut mx);
    mc.mc_next = *((*txn).mt_cursors).offset(dbi as isize);
    let ref mut fresh21 = *((*txn).mt_cursors).offset(dbi as isize);
    *fresh21 = &mut mc;
    rc = mdb_cursor_put(&mut mc, key, data, flags);
    let ref mut fresh22 = *((*txn).mt_cursors).offset(dbi as isize);
    *fresh22 = mc.mc_next;
    return rc;
}
unsafe extern "C" fn mdb_env_copythr(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut my: *mut mdb_copy = 0 as *mut mdb_copy;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut toggle: libc::c_int = 0;
    let mut wsize: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut set: sigset_t = sigset_t { __val: [0; 16] };
    let mut tmp: ssize_t = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: libc::c_int = 0;
    my = arg as *mut mdb_copy;
    toggle = 0 as libc::c_int;
    sigemptyset(&mut set);
    sigaddset(&mut set, 13 as libc::c_int);
    rc = pthread_sigmask(
        0 as libc::c_int,
        &mut set as *mut sigset_t as *const __sigset_t,
        0 as *mut libc::c_void as *mut __sigset_t,
    );
    if rc != 0 as libc::c_int {
        (*my).mc_error = rc;
    }
    pthread_mutex_lock(&mut (*my).mc_mutex);
    loop {
        while (*my).mc_new == 0 {
            pthread_cond_wait(
                &mut (*my).mc_cond as *mut pthread_cond_t,
                &mut (*my).mc_mutex as *mut pthread_mutex_t,
            );
        }
        if (*my).mc_new == 16 as libc::c_int {
            break;
        }
        wsize = (*my).mc_wlen[toggle as usize];
        ptr = (*my).mc_wbuf[toggle as usize];
        loop {
            rc = 0 as libc::c_int;
            while wsize > 0 as libc::c_int {
                if (*my).mc_error != 0 {
                    break;
                }
                tmp = write((*my).mc_fd, ptr as *const libc::c_void, wsize as size_t);
                len = tmp as libc::c_int;
                rc = (len >= 0 as libc::c_int) as libc::c_int;
                if rc == 0 {
                    tmp___0 = __errno_location();
                    rc = *tmp___0;
                    if rc == 32 as libc::c_int {
                        sigwait(
                            &mut set as *mut sigset_t as *const sigset_t,
                            &mut tmp___1 as *mut libc::c_int,
                        );
                    }
                    break;
                } else if len > 0 as libc::c_int {
                    rc = 0 as libc::c_int;
                    ptr = ptr.offset(len as isize);
                    wsize -= len;
                } else {
                    rc = 5 as libc::c_int;
                    break;
                }
            }
            if rc != 0 {
                (*my).mc_error = rc;
            }
            if !((*my).mc_olen[toggle as usize] != 0) {
                break;
            }
            wsize = (*my).mc_olen[toggle as usize];
            ptr = (*my).mc_over[toggle as usize];
            (*my).mc_olen[toggle as usize] = 0 as libc::c_int;
        }
        (*my).mc_wlen[toggle as usize] = 0 as libc::c_int;
        toggle ^= 1 as libc::c_int;
        (*my).mc_new -= 1;
        pthread_cond_signal(&mut (*my).mc_cond);
    }
    pthread_mutex_unlock(&mut (*my).mc_mutex);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn mdb_env_cthr_toggle(
    mut my: *mut mdb_copy,
    mut adjust: libc::c_int,
) -> libc::c_int {
    pthread_mutex_lock(&mut (*my).mc_mutex);
    (*my).mc_new += adjust;
    pthread_cond_signal(&mut (*my).mc_cond);
    while (*my).mc_new & 2 as libc::c_int != 0 {
        pthread_cond_wait(
            &mut (*my).mc_cond as *mut pthread_cond_t,
            &mut (*my).mc_mutex as *mut pthread_mutex_t,
        );
    }
    pthread_mutex_unlock(&mut (*my).mc_mutex);
    (*my).mc_toggle ^= adjust & 1 as libc::c_int;
    (*my).mc_wlen[(*my).mc_toggle as usize] = 0 as libc::c_int;
    return (*my).mc_error;
}
unsafe extern "C" fn mdb_env_cwalk(
    mut my: *mut mdb_copy,
    mut pg: *mut pgno_t,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut mc: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut ni: *mut MDB_node = 0 as *mut MDB_node;
    let mut mo: *mut MDB_page = 0 as *mut MDB_page;
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut leaf: *mut MDB_page = 0 as *mut MDB_page;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rc: libc::c_int = 0;
    let mut toggle: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut n: libc::c_uint = 0;
    let mut omp: *mut MDB_page = 0 as *mut MDB_page;
    let mut pg___0: pgno_t = 0;
    let mut db: MDB_db = MDB_db {
        md_pad: 0,
        md_flags: 0,
        md_depth: 0,
        md_branch_pages: 0,
        md_leaf_pages: 0,
        md_overflow_pages: 0,
        md_entries: 0,
        md_root: 0,
    };
    let mut pg___1: pgno_t = 0;
    let mut tmp___0: pgno_t = 0;
    mc.mc_next = 0 as *mut MDB_cursor;
    mc.mc_backup = 0 as *mut MDB_cursor;
    mc.mc_xcursor = 0 as *mut MDB_xcursor;
    mc.mc_txn = 0 as *mut MDB_txn;
    mc.mc_dbi = 0 as libc::c_uint;
    mc.mc_db = 0 as *mut MDB_db;
    mc.mc_dbx = 0 as *mut MDB_dbx;
    mc.mc_dbflag = 0 as *mut libc::c_uchar;
    mc.mc_snum = 0 as libc::c_int as libc::c_ushort;
    mc.mc_top = 0 as libc::c_int as libc::c_ushort;
    mc.mc_flags = 0 as libc::c_uint;
    mc.mc_pg[0 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[1 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[2 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[3 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[4 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[5 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[6 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[7 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[8 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[9 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[10 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[11 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[12 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[13 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[14 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[15 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[16 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[17 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[18 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[19 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[20 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[21 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[22 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[23 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[24 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[25 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[26 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[27 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[28 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[29 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[30 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_pg[31 as libc::c_int as usize] = 0 as *mut MDB_page;
    mc.mc_ki[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[3 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[4 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[5 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[6 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[7 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[9 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[10 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[11 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[12 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[13 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[14 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[15 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[16 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[17 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[18 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[19 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[20 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[21 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[22 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[23 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[24 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[25 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[26 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[27 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[28 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[29 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[30 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    mc.mc_ki[31 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    if *pg as libc::c_ulonglong == 18446744073709551615 as libc::c_ulonglong {
        return 0 as libc::c_int;
    }
    mc.mc_snum = 1 as libc::c_int as libc::c_ushort;
    mc.mc_txn = (*my).mc_txn;
    mc.mc_flags = (*(*my).mc_txn).mt_flags & 655360 as libc::c_uint;
    rc = mdb_page_get(
        &mut mc,
        *pg,
        &mut *(mc.mc_pg).as_mut_ptr().offset(0 as libc::c_int as isize),
        0 as *mut libc::c_void as *mut libc::c_int,
    );
    if rc != 0 {
        return rc;
    }
    rc = mdb_page_search_root(
        &mut mc,
        0 as *mut libc::c_void as *mut MDB_val,
        4 as libc::c_int,
    );
    if rc != 0 {
        return rc;
    }
    tmp = malloc(
        ((*(*my).mc_env).me_psize).wrapping_mul(mc.mc_snum as libc::c_uint) as size_t,
    );
    ptr = tmp as *mut libc::c_char;
    buf = ptr;
    if buf as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 12 as libc::c_int;
    }
    i = 0 as libc::c_uint;
    while i < mc.mc_top as libc::c_uint {
        mdb_page_copy(
            ptr as *mut MDB_page,
            mc.mc_pg[i as usize],
            (*(*my).mc_env).me_psize,
        );
        mc.mc_pg[i as usize] = ptr as *mut MDB_page;
        ptr = ptr.offset((*(*my).mc_env).me_psize as isize);
        i = i.wrapping_add(1);
    }
    leaf = ptr as *mut MDB_page;
    toggle = (*my).mc_toggle;
    's_356: while mc.mc_snum as libc::c_int > 0 as libc::c_int {
        mp = mc.mc_pg[mc.mc_top as usize];
        n = ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
            .wrapping_sub(
                &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1] as libc::c_ulong
                    as libc::c_uint,
            ) >> 1 as libc::c_int;
        if (*mp).mp_flags as libc::c_int & 2 as libc::c_int == 2 as libc::c_int {
            if !((*mp).mp_flags as libc::c_int & 32 as libc::c_int == 32 as libc::c_int)
            {
                if flags & 4 as libc::c_int == 0 {
                    i = 0 as libc::c_uint;
                    while i < n {
                        ni = (mp as *mut libc::c_char)
                            .offset(
                                *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize)
                                    as libc::c_int as isize,
                            )
                            .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                        if (*ni).mn_flags as libc::c_int & 1 as libc::c_int != 0 {
                            if mp as libc::c_ulong != leaf as libc::c_ulong {
                                mc.mc_pg[mc.mc_top as usize] = leaf;
                                mdb_page_copy(leaf, mp, (*(*my).mc_env).me_psize);
                                mp = leaf;
                                ni = (mp as *mut libc::c_char)
                                    .offset(
                                        *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize)
                                            as libc::c_int as isize,
                                    )
                                    .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                            }
                            memcpy(
                                &mut pg___0 as *mut pgno_t as *mut libc::c_void,
                                ((*ni).mn_data)
                                    .as_mut_ptr()
                                    .offset((*ni).mn_ksize as libc::c_int as isize)
                                    as *mut libc::c_void as *const libc::c_void,
                                ::std::mem::size_of::<pgno_t>() as libc::c_ulong,
                            );
                            memcpy(
                                ((*ni).mn_data)
                                    .as_mut_ptr()
                                    .offset((*ni).mn_ksize as libc::c_int as isize)
                                    as *mut libc::c_void,
                                &mut (*my).mc_next_pgno as *mut pgno_t
                                    as *const libc::c_void,
                                ::std::mem::size_of::<pgno_t>() as libc::c_ulong,
                            );
                            rc = mdb_page_get(
                                &mut mc,
                                pg___0,
                                &mut omp,
                                0 as *mut libc::c_void as *mut libc::c_int,
                            );
                            if rc != 0 {
                                break 's_356;
                            }
                            if (*my).mc_wlen[toggle as usize] >= 1048576 as libc::c_int {
                                rc = mdb_env_cthr_toggle(my, 1 as libc::c_int);
                                if rc != 0 {
                                    break 's_356;
                                }
                                toggle = (*my).mc_toggle;
                            }
                            mo = ((*my).mc_wbuf[toggle as usize])
                                .offset((*my).mc_wlen[toggle as usize] as isize)
                                as *mut MDB_page;
                            memcpy(
                                mo as *mut libc::c_void,
                                omp as *const libc::c_void,
                                (*(*my).mc_env).me_psize as size_t,
                            );
                            (*mo).mp_p.p_pgno = (*my).mc_next_pgno;
                            (*my)
                                .mc_next_pgno = ((*my).mc_next_pgno as libc::c_ulong)
                                .wrapping_add((*omp).mp_pb.pb_pages as pgno_t) as pgno_t
                                as pgno_t;
                            (*my)
                                .mc_wlen[toggle
                                as usize] = ((*my).mc_wlen[toggle as usize] as libc::c_uint)
                                .wrapping_add((*(*my).mc_env).me_psize) as libc::c_int;
                            if (*omp).mp_pb.pb_pages > 1 as libc::c_uint {
                                (*my)
                                    .mc_olen[toggle
                                    as usize] = ((*(*my).mc_env).me_psize)
                                    .wrapping_mul(
                                        ((*omp).mp_pb.pb_pages).wrapping_sub(1 as libc::c_uint),
                                    ) as libc::c_int;
                                (*my)
                                    .mc_over[toggle
                                    as usize] = (omp as *mut libc::c_char)
                                    .offset((*(*my).mc_env).me_psize as isize);
                                rc = mdb_env_cthr_toggle(my, 1 as libc::c_int);
                                if rc != 0 {
                                    break 's_356;
                                }
                                toggle = (*my).mc_toggle;
                            }
                        } else if (*ni).mn_flags as libc::c_int & 2 as libc::c_int != 0 {
                            if mp as libc::c_ulong != leaf as libc::c_ulong {
                                mc.mc_pg[mc.mc_top as usize] = leaf;
                                mdb_page_copy(leaf, mp, (*(*my).mc_env).me_psize);
                                mp = leaf;
                                ni = (mp as *mut libc::c_char)
                                    .offset(
                                        *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize)
                                            as libc::c_int as isize,
                                    )
                                    .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                            }
                            memcpy(
                                &mut db as *mut MDB_db as *mut libc::c_void,
                                ((*ni).mn_data)
                                    .as_mut_ptr()
                                    .offset((*ni).mn_ksize as libc::c_int as isize)
                                    as *mut libc::c_void as *const libc::c_void,
                                ::std::mem::size_of::<MDB_db>() as libc::c_ulong,
                            );
                            (*my).mc_toggle = toggle;
                            rc = mdb_env_cwalk(
                                my,
                                &mut db.md_root,
                                (*ni).mn_flags as libc::c_int & 4 as libc::c_int,
                            );
                            if rc != 0 {
                                break 's_356;
                            }
                            toggle = (*my).mc_toggle;
                            memcpy(
                                ((*ni).mn_data)
                                    .as_mut_ptr()
                                    .offset((*ni).mn_ksize as libc::c_int as isize)
                                    as *mut libc::c_void,
                                &mut db as *mut MDB_db as *const libc::c_void,
                                ::std::mem::size_of::<MDB_db>() as libc::c_ulong,
                            );
                        }
                        i = i.wrapping_add(1);
                    }
                }
            }
        } else {
            mc
                .mc_ki[mc.mc_top
                as usize] = (mc.mc_ki[mc.mc_top as usize] as libc::c_int
                + 1 as libc::c_int) as indx_t;
            if (mc.mc_ki[mc.mc_top as usize] as libc::c_uint) < n {
                loop {
                    ni = (mp as *mut libc::c_char)
                        .offset(
                            *((*mp).mp_ptrs)
                                .as_mut_ptr()
                                .offset(mc.mc_ki[mc.mc_top as usize] as isize)
                                as libc::c_int as isize,
                        )
                        .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                    pg___1 = (*ni).mn_lo as libc::c_ulong
                        | ((*ni).mn_hi as pgno_t) << 16 as libc::c_int
                        | ((*ni).mn_flags as pgno_t) << 32 as libc::c_int;
                    rc = mdb_page_get(
                        &mut mc,
                        pg___1,
                        &mut mp,
                        0 as *mut libc::c_void as *mut libc::c_int,
                    );
                    if rc != 0 {
                        break 's_356;
                    }
                    mc
                        .mc_top = (mc.mc_top as libc::c_int + 1 as libc::c_int)
                        as libc::c_ushort;
                    mc
                        .mc_snum = (mc.mc_snum as libc::c_int + 1 as libc::c_int)
                        as libc::c_ushort;
                    mc.mc_ki[mc.mc_top as usize] = 0 as libc::c_int as indx_t;
                    if !((*mp).mp_flags as libc::c_int & 1 as libc::c_int
                        == 1 as libc::c_int)
                    {
                        break;
                    }
                    mdb_page_copy(
                        mc.mc_pg[mc.mc_top as usize],
                        mp,
                        (*(*my).mc_env).me_psize,
                    );
                }
                mc.mc_pg[mc.mc_top as usize] = mp;
                continue;
            }
        }
        if (*my).mc_wlen[toggle as usize] >= 1048576 as libc::c_int {
            rc = mdb_env_cthr_toggle(my, 1 as libc::c_int);
            if rc != 0 {
                break;
            }
            toggle = (*my).mc_toggle;
        }
        mo = ((*my).mc_wbuf[toggle as usize])
            .offset((*my).mc_wlen[toggle as usize] as isize) as *mut MDB_page;
        mdb_page_copy(mo, mp, (*(*my).mc_env).me_psize);
        tmp___0 = (*my).mc_next_pgno;
        (*my).mc_next_pgno = ((*my).mc_next_pgno).wrapping_add(1);
        (*mo).mp_p.p_pgno = tmp___0;
        (*my)
            .mc_wlen[toggle
            as usize] = ((*my).mc_wlen[toggle as usize] as libc::c_uint)
            .wrapping_add((*(*my).mc_env).me_psize) as libc::c_int;
        if mc.mc_top != 0 {
            ni = (mc.mc_pg[(mc.mc_top as libc::c_int - 1 as libc::c_int) as usize]
                as *mut libc::c_char)
                .offset(
                    *((*mc.mc_pg[(mc.mc_top as libc::c_int - 1 as libc::c_int) as usize])
                        .mp_ptrs)
                        .as_mut_ptr()
                        .offset(
                            mc
                                .mc_ki[(mc.mc_top as libc::c_int - 1 as libc::c_int)
                                as usize] as isize,
                        ) as libc::c_int as isize,
                )
                .offset(0 as libc::c_uint as isize) as *mut MDB_node;
            (*ni).mn_lo = ((*mo).mp_p.p_pgno & 65535 as libc::c_ulong) as libc::c_ushort;
            (*ni).mn_hi = ((*mo).mp_p.p_pgno >> 16 as libc::c_int) as libc::c_ushort;
            (*ni).mn_flags = ((*mo).mp_p.p_pgno >> 32 as libc::c_int) as libc::c_ushort;
            mdb_cursor_pop(&mut mc);
        } else {
            *pg = (*mo).mp_p.p_pgno;
            break;
        }
    }
    free(buf as *mut libc::c_void);
    return rc;
}
unsafe extern "C" fn mdb_env_copyfd1(
    mut env: *mut MDB_env,
    mut fd: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut mm: *mut MDB_meta = 0 as *mut MDB_meta;
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut my: mdb_copy = mdb_copy {
        mc_env: 0 as *mut MDB_env,
        mc_txn: 0 as *mut MDB_txn,
        mc_mutex: __anonunion_pthread_mutex_t_335460617 {
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
        mc_cond: __anonunion_pthread_cond_t_951761805 {
            __data: __pthread_cond_s {
                __annonCompField1: __anonunion____missing_field_name_578844408 {
                    __wseq: 0,
                },
                __annonCompField2: __anonunion____missing_field_name_175635501 {
                    __g1_start: 0,
                },
                __g_refs: [0; 2],
                __g_size: [0; 2],
                __g1_orig_size: 0,
                __wrefs: 0,
                __g_signals: [0; 2],
            },
        },
        mc_wbuf: [0 as *mut libc::c_char; 2],
        mc_over: [0 as *mut libc::c_char; 2],
        mc_wlen: [0; 2],
        mc_olen: [0; 2],
        mc_next_pgno: 0,
        mc_fd: 0,
        mc_toggle: 0,
        mc_new: 0,
        mc_error: 0,
    };
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut thr: pthread_t = 0;
    let mut root: pgno_t = 0;
    let mut new_root: pgno_t = 0;
    let mut rc: libc::c_int = 0;
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut freecount: MDB_ID = 0;
    let mut mc: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut key: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut data: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut tmp: libc::c_int = 0;
    my.mc_env = 0 as *mut MDB_env;
    my.mc_txn = 0 as *mut MDB_txn;
    my.mc_mutex.__data.__lock = 0 as libc::c_int;
    my.mc_mutex.__data.__count = 0 as libc::c_uint;
    my.mc_mutex.__data.__owner = 0 as libc::c_int;
    my.mc_mutex.__data.__nusers = 0 as libc::c_uint;
    my.mc_mutex.__data.__kind = 0 as libc::c_int;
    my.mc_mutex.__data.__spins = 0 as libc::c_int as libc::c_short;
    my.mc_mutex.__data.__elision = 0 as libc::c_int as libc::c_short;
    my.mc_mutex.__data.__list.__prev = 0 as *mut __pthread_internal_list;
    my.mc_mutex.__data.__list.__next = 0 as *mut __pthread_internal_list;
    my.mc_cond.__data.__annonCompField1.__wseq = 0 as libc::c_ulonglong;
    my.mc_cond.__data.__annonCompField2.__g1_start = 0 as libc::c_ulonglong;
    my.mc_cond.__data.__g_refs[0 as libc::c_int as usize] = 0 as libc::c_uint;
    my.mc_cond.__data.__g_refs[1 as libc::c_int as usize] = 0 as libc::c_uint;
    my.mc_cond.__data.__g_size[0 as libc::c_int as usize] = 0 as libc::c_uint;
    my.mc_cond.__data.__g_size[1 as libc::c_int as usize] = 0 as libc::c_uint;
    my.mc_cond.__data.__g1_orig_size = 0 as libc::c_uint;
    my.mc_cond.__data.__wrefs = 0 as libc::c_uint;
    my.mc_cond.__data.__g_signals[0 as libc::c_int as usize] = 0 as libc::c_uint;
    my.mc_cond.__data.__g_signals[1 as libc::c_int as usize] = 0 as libc::c_uint;
    my.mc_wbuf[0 as libc::c_int as usize] = 0 as *mut libc::c_char;
    my.mc_wbuf[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
    my.mc_over[0 as libc::c_int as usize] = 0 as *mut libc::c_char;
    my.mc_over[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
    my.mc_wlen[0 as libc::c_int as usize] = 0 as libc::c_int;
    my.mc_wlen[1 as libc::c_int as usize] = 0 as libc::c_int;
    my.mc_olen[0 as libc::c_int as usize] = 0 as libc::c_int;
    my.mc_olen[1 as libc::c_int as usize] = 0 as libc::c_int;
    my.mc_next_pgno = 0 as libc::c_ulong;
    my.mc_fd = 0 as libc::c_int;
    my.mc_toggle = 0 as libc::c_int;
    my.mc_new = 0 as libc::c_int;
    my.mc_error = 0 as libc::c_int;
    txn = 0 as *mut libc::c_void as *mut MDB_txn;
    rc = 0 as libc::c_int;
    rc = pthread_mutex_init(
        &mut my.mc_mutex,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
    if rc != 0 as libc::c_int {
        return rc;
    }
    rc = pthread_cond_init(
        &mut my.mc_cond as *mut pthread_cond_t,
        0 as *mut libc::c_void as *const pthread_condattr_t,
    );
    if !(rc != 0 as libc::c_int) {
        rc = posix_memalign(
            &mut p,
            (*env).me_os_psize as size_t,
            2097152 as libc::c_int as size_t,
        );
        if !(rc != 0 as libc::c_int) {
            my.mc_wbuf[0 as libc::c_int as usize] = p as *mut libc::c_char;
            memset(
                my.mc_wbuf[0 as libc::c_int as usize] as *mut libc::c_void,
                0 as libc::c_int,
                2097152 as libc::c_int as size_t,
            );
            my
                .mc_wbuf[1 as libc::c_int
                as usize] = (my.mc_wbuf[0 as libc::c_int as usize])
                .offset(1048576 as libc::c_int as isize);
            my.mc_next_pgno = 2 as libc::c_int as pgno_t;
            my.mc_env = env;
            my.mc_fd = fd;
            rc = pthread_create(
                &mut thr as *mut pthread_t,
                0 as *mut libc::c_void as *const pthread_attr_t,
                Some(
                    mdb_env_copythr
                        as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                ),
                &mut my as *mut mdb_copy as *mut libc::c_void,
            );
            if !(rc != 0) {
                rc = mdb_txn_begin(
                    env,
                    0 as *mut libc::c_void as *mut MDB_txn,
                    131072 as libc::c_uint,
                    &mut txn,
                );
                if !(rc != 0) {
                    mp = my.mc_wbuf[0 as libc::c_int as usize] as *mut MDB_page;
                    memset(
                        mp as *mut libc::c_void,
                        0 as libc::c_int,
                        (2 as libc::c_uint).wrapping_mul((*env).me_psize) as size_t,
                    );
                    (*mp).mp_p.p_pgno = 0 as libc::c_int as pgno_t;
                    (*mp).mp_flags = 8 as libc::c_int as uint16_t;
                    mm = (mp as *mut libc::c_char)
                        .offset(
                            &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                as libc::c_ulong as libc::c_uint as isize,
                        ) as *mut libc::c_void as *mut MDB_meta;
                    mdb_env_init_meta0(env, mm);
                    (*mm)
                        .mm_address = (*(*env).me_metas[0 as libc::c_int as usize])
                        .mm_address;
                    mp = (my.mc_wbuf[0 as libc::c_int as usize])
                        .offset((*env).me_psize as isize) as *mut MDB_page;
                    (*mp).mp_p.p_pgno = 1 as libc::c_int as pgno_t;
                    (*mp).mp_flags = 8 as libc::c_int as uint16_t;
                    *((mp as *mut libc::c_char)
                        .offset(
                            &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                as libc::c_ulong as libc::c_uint as isize,
                        ) as *mut libc::c_void as *mut MDB_meta) = *mm;
                    mm = (mp as *mut libc::c_char)
                        .offset(
                            &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                                as libc::c_ulong as libc::c_uint as isize,
                        ) as *mut libc::c_void as *mut MDB_meta;
                    new_root = (*((*txn).mt_dbs).offset(1 as libc::c_int as isize))
                        .md_root;
                    root = new_root;
                    if root as libc::c_ulonglong
                        != 18446744073709551615 as libc::c_ulonglong
                    {
                        freecount = 0 as libc::c_int as MDB_ID;
                        mdb_cursor_init(
                            &mut mc,
                            txn,
                            0 as libc::c_int as MDB_dbi,
                            0 as *mut libc::c_void as *mut MDB_xcursor,
                        );
                        loop {
                            rc = mdb_cursor_get(&mut mc, &mut key, &mut data, MDB_NEXT);
                            if !(rc == 0 as libc::c_int) {
                                break;
                            }
                            freecount = (freecount as libc::c_ulong)
                                .wrapping_add(*(data.mv_data as *mut MDB_ID)) as MDB_ID
                                as MDB_ID;
                        }
                        if rc != -(30798 as libc::c_int) {
                            current_block = 17664116004594544052;
                        } else {
                            freecount = (freecount as libc::c_ulong)
                                .wrapping_add(
                                    ((*((*txn).mt_dbs).offset(0 as libc::c_int as isize))
                                        .md_branch_pages)
                                        .wrapping_add(
                                            (*((*txn).mt_dbs).offset(0 as libc::c_int as isize))
                                                .md_leaf_pages,
                                        )
                                        .wrapping_add(
                                            (*((*txn).mt_dbs).offset(0 as libc::c_int as isize))
                                                .md_overflow_pages,
                                        ),
                                ) as MDB_ID as MDB_ID;
                            new_root = ((*txn).mt_next_pgno)
                                .wrapping_sub(1 as libc::c_ulong)
                                .wrapping_sub(freecount);
                            (*mm).mm_last_pg = new_root;
                            (*mm)
                                .mm_dbs[1 as libc::c_int
                                as usize] = *((*txn).mt_dbs)
                                .offset(1 as libc::c_int as isize);
                            (*mm).mm_dbs[1 as libc::c_int as usize].md_root = new_root;
                            current_block = 12930649117290160518;
                        }
                    } else {
                        (*mm)
                            .mm_dbs[1 as libc::c_int as usize]
                            .md_flags = (*((*txn).mt_dbs)
                            .offset(1 as libc::c_int as isize))
                            .md_flags;
                        current_block = 12930649117290160518;
                    }
                    match current_block {
                        17664116004594544052 => {}
                        _ => {
                            if root as libc::c_ulonglong
                                != 18446744073709551615 as libc::c_ulonglong
                            {
                                (*mm).mm_txnid = 1 as libc::c_int as txnid_t;
                            } else if (*mm).mm_dbs[1 as libc::c_int as usize].md_flags
                                    != 0
                                {
                                (*mm).mm_txnid = 1 as libc::c_int as txnid_t;
                            }
                            my
                                .mc_wlen[0 as libc::c_int
                                as usize] = ((*env).me_psize)
                                .wrapping_mul(2 as libc::c_uint) as libc::c_int;
                            my.mc_txn = txn;
                            rc = mdb_env_cwalk(&mut my, &mut root, 0 as libc::c_int);
                            if rc == 0 as libc::c_int {
                                if root != new_root {
                                    rc = -(30784 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                if rc != 0 {
                    my.mc_error = rc;
                }
                mdb_env_cthr_toggle(&mut my, 17 as libc::c_int);
                rc = pthread_join(thr, 0 as *mut libc::c_void as *mut *mut libc::c_void);
                mdb_txn_abort(txn);
            }
        }
        free(my.mc_wbuf[0 as libc::c_int as usize] as *mut libc::c_void);
        pthread_cond_destroy(&mut my.mc_cond);
    }
    pthread_mutex_destroy(&mut my.mc_mutex);
    if rc != 0 {
        tmp = rc;
    } else {
        tmp = my.mc_error;
    }
    return tmp;
}
unsafe extern "C" fn mdb_env_copyfd0(
    mut env: *mut MDB_env,
    mut fd: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut wmutex: mdb_mutexref_t = 0 as *mut pthread_mutex_t;
    let mut rc: libc::c_int = 0;
    let mut wsize: mdb_size_t = 0;
    let mut w3: mdb_size_t = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: ssize_t = 0;
    let mut w2: size_t = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut fsize: mdb_size_t = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    txn = 0 as *mut libc::c_void as *mut MDB_txn;
    wmutex = 0 as *mut libc::c_void as mdb_mutexref_t;
    rc = mdb_txn_begin(
        env,
        0 as *mut libc::c_void as *mut MDB_txn,
        131072 as libc::c_uint,
        &mut txn,
    );
    if rc != 0 {
        return rc;
    }
    if !((*env).me_txns).is_null() {
        mdb_txn_end(txn, 4 as libc::c_uint);
        wmutex = ((*(*env).me_txns).mt2.mt2_wmutex).as_mut_ptr();
        rc = pthread_mutex_lock(wmutex);
        if rc != 0 {
            rc = mdb_mutex_failed(env, wmutex, rc);
            if rc != 0 {
                current_block = 14411349311596161428;
            } else {
                current_block = 3512920355445576850;
            }
        } else {
            current_block = 3512920355445576850;
        }
        match current_block {
            14411349311596161428 => {}
            _ => {
                rc = mdb_txn_renew0(txn);
                if rc != 0 {
                    pthread_mutex_unlock(wmutex);
                    current_block = 14411349311596161428;
                } else {
                    current_block = 15652330335145281839;
                }
            }
        }
    } else {
        current_block = 15652330335145281839;
    }
    match current_block {
        15652330335145281839 => {
            wsize = ((*env).me_psize).wrapping_mul(2 as libc::c_uint) as mdb_size_t;
            ptr = (*env).me_map;
            w2 = wsize;
            while w2 > 0 as libc::c_ulong {
                len = write(fd, ptr as *const libc::c_void, w2);
                rc = (len >= 0 as libc::c_long) as libc::c_int;
                if rc == 0 {
                    tmp = __errno_location();
                    rc = *tmp;
                    break;
                } else if len > 0 as libc::c_long {
                    rc = 0 as libc::c_int;
                    ptr = ptr.offset(len as isize);
                    w2 = (w2 as libc::c_ulong).wrapping_sub(len as size_t) as size_t
                        as size_t;
                } else {
                    rc = 5 as libc::c_int;
                    break;
                }
            }
            if !wmutex.is_null() {
                pthread_mutex_unlock(wmutex);
            }
            if !(rc != 0) {
                w3 = ((*txn).mt_next_pgno).wrapping_mul((*env).me_psize as pgno_t);
                fsize = 0 as libc::c_int as mdb_size_t;
                rc = mdb_fsize((*env).me_fd, &mut fsize);
                if !(rc != 0) {
                    if w3 > fsize {
                        w3 = fsize;
                    }
                    wsize = w3.wrapping_sub(wsize);
                    while wsize > 0 as libc::c_ulong {
                        if wsize
                            > (1073741824 as libc::c_uint
                                >> (::std::mem::size_of::<ssize_t>() as libc::c_ulong
                                    == 4 as libc::c_ulong) as libc::c_int) as mdb_size_t
                        {
                            w2 = (1073741824 as libc::c_uint
                                >> (::std::mem::size_of::<ssize_t>() as libc::c_ulong
                                    == 4 as libc::c_ulong) as libc::c_int) as size_t;
                        } else {
                            w2 = wsize;
                        }
                        len = write(fd, ptr as *const libc::c_void, w2);
                        rc = (len >= 0 as libc::c_long) as libc::c_int;
                        if rc == 0 {
                            tmp___0 = __errno_location();
                            rc = *tmp___0;
                            break;
                        } else if len > 0 as libc::c_long {
                            rc = 0 as libc::c_int;
                            ptr = ptr.offset(len as isize);
                            wsize = (wsize as libc::c_ulong)
                                .wrapping_sub(len as mdb_size_t) as mdb_size_t
                                as mdb_size_t;
                        } else {
                            rc = 5 as libc::c_int;
                            break;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    mdb_txn_abort(txn);
    return rc;
}
pub unsafe extern "C" fn mdb_env_copyfd2(
    mut env: *mut MDB_env,
    mut fd: libc::c_int,
    mut flags: libc::c_uint,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    if flags & 1 as libc::c_uint != 0 {
        tmp = mdb_env_copyfd1(env, fd);
        return tmp;
    } else {
        tmp___0 = mdb_env_copyfd0(env, fd);
        return tmp___0;
    };
}
pub unsafe extern "C" fn mdb_env_copyfd(
    mut env: *mut MDB_env,
    mut fd: libc::c_int,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = mdb_env_copyfd2(env, fd, 0 as libc::c_uint);
    return tmp;
}
pub unsafe extern "C" fn mdb_env_copy2(
    mut env: *mut MDB_env,
    mut path: *const libc::c_char,
    mut flags: libc::c_uint,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut fname: MDB_name = MDB_name {
        mn_len: 0,
        mn_alloced: 0,
        mn_val: 0 as *mut mdb_nchar_t,
    };
    let mut newfd: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: libc::c_int = 0;
    newfd = -(1 as libc::c_int);
    rc = mdb_fname_init(path, (*env).me_flags | 4194304 as libc::c_uint, &mut fname);
    if rc == 0 as libc::c_int {
        rc = mdb_fopen(
            env as *const MDB_env,
            &mut fname,
            MDB_O_COPY,
            438 as libc::c_int as mdb_mode_t,
            &mut newfd,
        );
        if fname.mn_alloced != 0 {
            free(fname.mn_val as *mut libc::c_void);
        }
    }
    if rc == 0 as libc::c_int {
        rc = mdb_env_copyfd2(env, newfd, flags);
        tmp___0 = close(newfd);
        if tmp___0 < 0 as libc::c_int {
            if rc == 0 as libc::c_int {
                tmp = __errno_location();
                rc = *tmp;
            }
        }
    }
    return rc;
}
pub unsafe extern "C" fn mdb_env_copy(
    mut env: *mut MDB_env,
    mut path: *const libc::c_char,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = mdb_env_copy2(env, path, 0 as libc::c_uint);
    return tmp;
}
pub unsafe extern "C" fn mdb_env_set_flags(
    mut env: *mut MDB_env,
    mut flag: libc::c_uint,
    mut onoff: libc::c_int,
) -> libc::c_int {
    if flag & 4276813823 as libc::c_uint != 0 {
        return 22 as libc::c_int;
    }
    if onoff != 0 {
        (*env).me_flags |= flag;
    } else {
        (*env).me_flags &= !flag;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_env_get_flags(
    mut env: *mut MDB_env,
    mut arg: *mut libc::c_uint,
) -> libc::c_int {
    if env.is_null() {
        return 22 as libc::c_int
    } else {
        if arg.is_null() {
            return 22 as libc::c_int;
        }
    }
    *arg = (*env).me_flags & 67059713 as libc::c_uint;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_env_set_userctx(
    mut env: *mut MDB_env,
    mut ctx: *mut libc::c_void,
) -> libc::c_int {
    if env.is_null() {
        return 22 as libc::c_int;
    }
    (*env).me_userctx = ctx;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_env_get_userctx(
    mut env: *mut MDB_env,
) -> *mut libc::c_void {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    if !env.is_null() {
        tmp = (*env).me_userctx;
    } else {
        tmp = 0 as *mut libc::c_void;
    }
    return tmp;
}
pub unsafe extern "C" fn mdb_env_set_assert(
    mut env: *mut MDB_env,
    mut func: Option::<MDB_assert_func>,
) -> libc::c_int {
    if env.is_null() {
        return 22 as libc::c_int;
    }
    (*env).me_assert_func = func;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_env_get_path(
    mut env: *mut MDB_env,
    mut arg: *mut *const libc::c_char,
) -> libc::c_int {
    if env.is_null() {
        return 22 as libc::c_int
    } else {
        if arg.is_null() {
            return 22 as libc::c_int;
        }
    }
    *arg = (*env).me_path as *const libc::c_char;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_env_get_fd(
    mut env: *mut MDB_env,
    mut arg: *mut mdb_filehandle_t,
) -> libc::c_int {
    if env.is_null() {
        return 22 as libc::c_int
    } else {
        if arg.is_null() {
            return 22 as libc::c_int;
        }
    }
    *arg = (*env).me_fd;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_stat0(
    mut env: *mut MDB_env,
    mut db: *mut MDB_db,
    mut arg: *mut MDB_stat,
) -> libc::c_int {
    (*arg).ms_psize = (*env).me_psize;
    (*arg).ms_depth = (*db).md_depth as libc::c_uint;
    (*arg).ms_branch_pages = (*db).md_branch_pages;
    (*arg).ms_leaf_pages = (*db).md_leaf_pages;
    (*arg).ms_overflow_pages = (*db).md_overflow_pages;
    (*arg).ms_entries = (*db).md_entries;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_env_stat(
    mut env: *mut MDB_env,
    mut arg: *mut MDB_stat,
) -> libc::c_int {
    let mut meta: *mut MDB_meta = 0 as *mut MDB_meta;
    let mut tmp: libc::c_int = 0;
    if env as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 22 as libc::c_int
    } else {
        if arg as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            return 22 as libc::c_int;
        }
    }
    meta = mdb_env_pick_meta(env as *const MDB_env);
    tmp = mdb_stat0(
        env,
        &mut *((*meta).mm_dbs).as_mut_ptr().offset(1 as libc::c_int as isize),
        arg,
    );
    return tmp;
}
pub unsafe extern "C" fn mdb_env_info(
    mut env: *mut MDB_env,
    mut arg: *mut MDB_envinfo,
) -> libc::c_int {
    let mut meta: *mut MDB_meta = 0 as *mut MDB_meta;
    if env as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 22 as libc::c_int
    } else {
        if arg as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            return 22 as libc::c_int;
        }
    }
    meta = mdb_env_pick_meta(env as *const MDB_env);
    (*arg).me_mapaddr = (*meta).mm_address;
    (*arg).me_last_pgno = (*meta).mm_last_pg;
    (*arg).me_last_txnid = (*meta).mm_txnid;
    (*arg).me_mapsize = (*env).me_mapsize;
    (*arg).me_maxreaders = (*env).me_maxreaders;
    if !((*env).me_txns).is_null() {
        (*arg).me_numreaders = (*(*env).me_txns).mt1.mtb.mtb_numreaders;
    } else {
        (*arg).me_numreaders = 0 as libc::c_uint;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_default_cmp(mut txn: *mut MDB_txn, mut dbi: MDB_dbi) {
    let mut f: uint16_t = 0;
    let mut tmp: Option::<
        unsafe extern "C" fn(*const MDB_val, *const MDB_val) -> libc::c_int,
    > = None;
    let mut tmp___0: Option::<
        unsafe extern "C" fn(*const MDB_val, *const MDB_val) -> libc::c_int,
    > = None;
    let mut tmp___1: Option::<
        unsafe extern "C" fn(*const MDB_val, *const MDB_val) -> libc::c_int,
    > = None;
    let mut tmp___2: Option::<
        unsafe extern "C" fn(*const MDB_val, *const MDB_val) -> libc::c_int,
    > = None;
    f = (*((*txn).mt_dbs).offset(dbi as isize)).md_flags;
    if f as libc::c_int & 2 as libc::c_int != 0 {
        let ref mut fresh23 = (*((*txn).mt_dbxs).offset(dbi as isize)).md_cmp;
        *fresh23 = Some(
            mdb_cmp_memnr
                as unsafe extern "C" fn(*const MDB_val, *const MDB_val) -> libc::c_int,
        );
    } else {
        if f as libc::c_int & 8 as libc::c_int != 0 {
            tmp = Some(
                mdb_cmp_cint
                    as unsafe extern "C" fn(
                        *const MDB_val,
                        *const MDB_val,
                    ) -> libc::c_int,
            );
        } else {
            tmp = Some(
                mdb_cmp_memn
                    as unsafe extern "C" fn(
                        *const MDB_val,
                        *const MDB_val,
                    ) -> libc::c_int,
            );
        }
        let ref mut fresh24 = (*((*txn).mt_dbxs).offset(dbi as isize)).md_cmp;
        *fresh24 = tmp;
    }
    if f as libc::c_int & 4 as libc::c_int == 0 {
        let ref mut fresh25 = (*((*txn).mt_dbxs).offset(dbi as isize)).md_dcmp;
        *fresh25 = None;
    } else {
        if f as libc::c_int & 32 as libc::c_int != 0 {
            if f as libc::c_int & 16 as libc::c_int != 0 {
                tmp___0 = Some(
                    mdb_cmp_int
                        as unsafe extern "C" fn(
                            *const MDB_val,
                            *const MDB_val,
                        ) -> libc::c_int,
                );
            } else {
                tmp___0 = Some(
                    mdb_cmp_cint
                        as unsafe extern "C" fn(
                            *const MDB_val,
                            *const MDB_val,
                        ) -> libc::c_int,
                );
            }
            tmp___2 = tmp___0;
        } else {
            if f as libc::c_int & 64 as libc::c_int != 0 {
                tmp___1 = Some(
                    mdb_cmp_memnr
                        as unsafe extern "C" fn(
                            *const MDB_val,
                            *const MDB_val,
                        ) -> libc::c_int,
                );
            } else {
                tmp___1 = Some(
                    mdb_cmp_memn
                        as unsafe extern "C" fn(
                            *const MDB_val,
                            *const MDB_val,
                        ) -> libc::c_int,
                );
            }
            tmp___2 = tmp___1;
        }
        let ref mut fresh26 = (*((*txn).mt_dbxs).offset(dbi as isize)).md_dcmp;
        *fresh26 = tmp___2;
    };
}
pub unsafe extern "C" fn mdb_dbi_open(
    mut txn: *mut MDB_txn,
    mut name: *const libc::c_char,
    mut flags: libc::c_uint,
    mut dbi: *mut MDB_dbi,
) -> libc::c_int {
    let mut key: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut data: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut i: MDB_dbi = 0;
    let mut mc: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut dummy: MDB_db = MDB_db {
        md_pad: 0,
        md_flags: 0,
        md_depth: 0,
        md_branch_pages: 0,
        md_leaf_pages: 0,
        md_overflow_pages: 0,
        md_entries: 0,
        md_root: 0,
    };
    let mut rc: libc::c_int = 0;
    let mut dbflag: libc::c_int = 0;
    let mut exact: libc::c_int = 0;
    let mut unused: libc::c_uint = 0;
    let mut seq: libc::c_uint = 0;
    let mut namedup: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut f2: uint16_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut node: *mut MDB_node = 0 as *mut MDB_node;
    let mut dummy___0: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut tracked: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut tp: *mut *mut MDB_cursor = 0 as *mut *mut MDB_cursor;
    let mut slot: libc::c_uint = 0;
    let mut tmp___1: libc::c_uint = 0;
    unused = 0 as libc::c_uint;
    if flags & 4294705025 as libc::c_uint != 0 {
        return 22 as libc::c_int;
    }
    if (*txn).mt_flags & 19 as libc::c_uint != 0 {
        return -(30782 as libc::c_int);
    }
    if name.is_null() {
        *dbi = 1 as libc::c_int as MDB_dbi;
        if flags & 32767 as libc::c_uint != 0 {
            f2 = (flags & 32767 as libc::c_uint) as uint16_t;
            if (*((*txn).mt_dbs).offset(1 as libc::c_int as isize)).md_flags
                as libc::c_int | f2 as libc::c_int
                != (*((*txn).mt_dbs).offset(1 as libc::c_int as isize)).md_flags
                    as libc::c_int
            {
                (*((*txn).mt_dbs).offset(1 as libc::c_int as isize))
                    .md_flags = ((*((*txn).mt_dbs).offset(1 as libc::c_int as isize))
                    .md_flags as libc::c_int | f2 as libc::c_int) as uint16_t;
                (*txn).mt_flags |= 4 as libc::c_uint;
            }
        }
        mdb_default_cmp(txn, 1 as libc::c_int as MDB_dbi);
        return 0 as libc::c_int;
    }
    if ::std::mem::transmute::<
        Option::<MDB_cmp_func>,
        libc::c_ulong,
    >((*((*txn).mt_dbxs).offset(1 as libc::c_int as isize)).md_cmp)
        == 0 as *mut libc::c_void as libc::c_ulong
    {
        mdb_default_cmp(txn, 1 as libc::c_int as MDB_dbi);
    }
    len = strlen(name);
    i = 2 as libc::c_int as MDB_dbi;
    while i < (*txn).mt_numdbs {
        if (*((*txn).mt_dbxs).offset(i as isize)).md_name.mv_size == 0 {
            if unused == 0 {
                unused = i;
            }
        } else if len == (*((*txn).mt_dbxs).offset(i as isize)).md_name.mv_size {
            tmp = strncmp(
                name,
                (*((*txn).mt_dbxs).offset(i as isize)).md_name.mv_data
                    as *const libc::c_char,
                len,
            );
            if tmp == 0 {
                *dbi = i;
                return 0 as libc::c_int;
            }
        }
        i = i.wrapping_add(1);
    }
    if unused == 0 {
        if (*txn).mt_numdbs >= (*(*txn).mt_env).me_maxdbs {
            return -(30791 as libc::c_int);
        }
    }
    if (*((*txn).mt_dbs).offset(1 as libc::c_int as isize)).md_flags as libc::c_int
        & 12 as libc::c_int != 0
    {
        if flags & 262144 as libc::c_uint != 0 {
            tmp___0 = -(30784 as libc::c_int);
        } else {
            tmp___0 = -(30798 as libc::c_int);
        }
        return tmp___0;
    }
    dbflag = 28 as libc::c_int;
    exact = 0 as libc::c_int;
    key.mv_size = len;
    key.mv_data = name as *mut libc::c_void;
    mdb_cursor_init(
        &mut mc,
        txn,
        1 as libc::c_int as MDB_dbi,
        0 as *mut libc::c_void as *mut MDB_xcursor,
    );
    rc = mdb_cursor_set(&mut mc, &mut key, &mut data, MDB_SET, &mut exact);
    if rc == 0 as libc::c_int {
        node = (mc.mc_pg[mc.mc_top as usize] as *mut libc::c_char)
            .offset(
                *((*mc.mc_pg[mc.mc_top as usize]).mp_ptrs)
                    .as_mut_ptr()
                    .offset(mc.mc_ki[mc.mc_top as usize] as isize) as libc::c_int
                    as isize,
            )
            .offset(0 as libc::c_uint as isize) as *mut MDB_node;
        if (*node).mn_flags as libc::c_int & 6 as libc::c_int != 2 as libc::c_int {
            return -(30784 as libc::c_int);
        }
    } else {
        if rc != -(30798 as libc::c_int) {
            return rc
        } else {
            if flags & 262144 as libc::c_uint == 0 {
                return rc;
            }
        }
        if (*txn).mt_flags & 131072 as libc::c_uint == 131072 as libc::c_uint {
            return 13 as libc::c_int;
        }
    }
    namedup = strdup(name);
    if namedup as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 12 as libc::c_int;
    }
    if rc != 0 {
        data.mv_size = ::std::mem::size_of::<MDB_db>() as libc::c_ulong;
        data.mv_data = &mut dummy as *mut MDB_db as *mut libc::c_void;
        memset(
            &mut dummy as *mut MDB_db as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<MDB_db>() as libc::c_ulong,
        );
        dummy.md_root = !(0 as libc::c_int as pgno_t);
        dummy.md_flags = (flags & 32767 as libc::c_uint) as uint16_t;
        tp = ((*mc.mc_txn).mt_cursors).offset(mc.mc_dbi as isize);
        if mc.mc_flags & 4 as libc::c_uint != 0 {
            dummy___0.mc_flags = 1 as libc::c_uint;
            dummy___0.mc_xcursor = &mut mc as *mut MDB_cursor as *mut MDB_xcursor;
            tracked = &mut dummy___0;
        } else {
            tracked = &mut mc;
        }
        (*tracked).mc_next = *tp;
        *tp = tracked;
        rc = mdb_cursor_put(&mut mc, &mut key, &mut data, 2 as libc::c_uint);
        *tp = (*tracked).mc_next;
        dbflag |= 1 as libc::c_int;
    }
    if rc != 0 {
        free(namedup as *mut libc::c_void);
    } else {
        if unused != 0 {
            tmp___1 = unused;
        } else {
            tmp___1 = (*txn).mt_numdbs;
        }
        slot = tmp___1;
        let ref mut fresh27 = (*((*txn).mt_dbxs).offset(slot as isize)).md_name.mv_data;
        *fresh27 = namedup as *mut libc::c_void;
        (*((*txn).mt_dbxs).offset(slot as isize)).md_name.mv_size = len;
        let ref mut fresh28 = (*((*txn).mt_dbxs).offset(slot as isize)).md_rel;
        *fresh28 = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<MDB_rel_func>,
        >(0 as *mut libc::c_void);
        *((*txn).mt_dbflags).offset(slot as isize) = dbflag as libc::c_uchar;
        let ref mut fresh29 = *((*(*txn).mt_env).me_dbiseqs).offset(slot as isize);
        *fresh29 = (*fresh29).wrapping_add(1);
        seq = *((*(*txn).mt_env).me_dbiseqs).offset(slot as isize);
        *((*txn).mt_dbiseqs).offset(slot as isize) = seq;
        memcpy(
            ((*txn).mt_dbs).offset(slot as isize) as *mut libc::c_void,
            data.mv_data as *const libc::c_void,
            ::std::mem::size_of::<MDB_db>() as libc::c_ulong,
        );
        *dbi = slot;
        mdb_default_cmp(txn, slot);
        if unused == 0 {
            (*txn).mt_numdbs = ((*txn).mt_numdbs).wrapping_add(1);
        }
    }
    return rc;
}
pub unsafe extern "C" fn mdb_stat(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut arg: *mut MDB_stat,
) -> libc::c_int {
    let mut mc: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut mx: MDB_xcursor = MDB_xcursor {
        mx_cursor: MDB_cursor {
            mc_next: 0 as *mut MDB_cursor,
            mc_backup: 0 as *mut MDB_cursor,
            mc_xcursor: 0 as *mut MDB_xcursor,
            mc_txn: 0 as *mut MDB_txn,
            mc_dbi: 0,
            mc_db: 0 as *mut MDB_db,
            mc_dbx: 0 as *mut MDB_dbx,
            mc_dbflag: 0 as *mut libc::c_uchar,
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [0 as *mut MDB_page; 32],
            mc_ki: [0; 32],
        },
        mx_db: MDB_db {
            md_pad: 0,
            md_flags: 0,
            md_depth: 0,
            md_branch_pages: 0,
            md_leaf_pages: 0,
            md_overflow_pages: 0,
            md_entries: 0,
            md_root: 0,
        },
        mx_dbx: MDB_dbx {
            md_name: MDB_val {
                mv_size: 0,
                mv_data: 0 as *mut libc::c_void,
            },
            md_cmp: None,
            md_dcmp: None,
            md_rel: None,
            md_relctx: 0 as *mut libc::c_void,
        },
        mx_dbflag: 0,
    };
    let mut tmp: libc::c_int = 0;
    if arg.is_null() {
        return 22 as libc::c_int
    } else {
        if !txn.is_null() {
            if dbi < (*txn).mt_numdbs {
                if *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int
                    & 8 as libc::c_int == 0
                {
                    return 22 as libc::c_int;
                }
            } else {
                return 22 as libc::c_int
            }
        } else {
            return 22 as libc::c_int
        }
    }
    if (*txn).mt_flags & 19 as libc::c_uint != 0 {
        return -(30782 as libc::c_int);
    }
    if *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int & 2 as libc::c_int != 0 {
        mdb_cursor_init(&mut mc, txn, dbi, &mut mx);
    }
    tmp = mdb_stat0((*txn).mt_env, ((*txn).mt_dbs).offset(dbi as isize), arg);
    return tmp;
}
pub unsafe extern "C" fn mdb_dbi_close(mut env: *mut MDB_env, mut dbi: MDB_dbi) {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if dbi < 2 as libc::c_uint {
        return
    } else {
        if dbi >= (*env).me_maxdbs {
            return;
        }
    }
    ptr = (*((*env).me_dbxs).offset(dbi as isize)).md_name.mv_data as *mut libc::c_char;
    if !ptr.is_null() {
        let ref mut fresh30 = (*((*env).me_dbxs).offset(dbi as isize)).md_name.mv_data;
        *fresh30 = 0 as *mut libc::c_void;
        (*((*env).me_dbxs).offset(dbi as isize))
            .md_name
            .mv_size = 0 as libc::c_int as size_t;
        *((*env).me_dbflags).offset(dbi as isize) = 0 as libc::c_int as uint16_t;
        let ref mut fresh31 = *((*env).me_dbiseqs).offset(dbi as isize);
        *fresh31 = (*fresh31).wrapping_add(1);
        free(ptr as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn mdb_dbi_flags(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut flags: *mut libc::c_uint,
) -> libc::c_int {
    if !txn.is_null() {
        if dbi < (*txn).mt_numdbs {
            if *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int
                & 16 as libc::c_int == 0
            {
                return 22 as libc::c_int;
            }
        } else {
            return 22 as libc::c_int
        }
    } else {
        return 22 as libc::c_int
    }
    *flags = ((*((*txn).mt_dbs).offset(dbi as isize)).md_flags as libc::c_int
        & 32767 as libc::c_int) as libc::c_uint;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_drop0(
    mut mc: *mut MDB_cursor,
    mut subs: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut rc: libc::c_int = 0;
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut ni: *mut MDB_node = 0 as *mut MDB_node;
    let mut mx: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut i: libc::c_uint = 0;
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut n: libc::c_uint = 0;
    let mut omp: *mut MDB_page = 0 as *mut MDB_page;
    let mut pg: pgno_t = 0;
    let mut pg___0: pgno_t = 0;
    let mut xidl: *mut MDB_ID = 0 as *mut MDB_ID;
    let mut xlen: MDB_ID = 0;
    rc = mdb_page_search(mc, 0 as *mut libc::c_void as *mut MDB_val, 4 as libc::c_int);
    if rc == 0 as libc::c_int {
        txn = (*mc).mc_txn;
        if (*mc).mc_flags & 4 as libc::c_uint != 0 {
            mdb_cursor_pop(mc);
        } else if subs == 0 {
            if (*(*mc).mc_db).md_overflow_pages == 0 {
                mdb_cursor_pop(mc);
            }
        }
        mdb_cursor_copy(mc as *const MDB_cursor, &mut mx);
        's_73: loop {
            if !((*mc).mc_snum as libc::c_int > 0 as libc::c_int) {
                current_block = 1134115459065347084;
                break;
            }
            mp = (*mc).mc_pg[(*mc).mc_top as usize];
            n = ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                .wrapping_sub(
                    &mut (*(0 as *mut MDB_page)).mp_ptrs as *mut [indx_t; 1]
                        as libc::c_ulong as libc::c_uint,
                ) >> 1 as libc::c_int;
            if (*mp).mp_flags as libc::c_int & 2 as libc::c_int == 2 as libc::c_int {
                i = 0 as libc::c_uint;
                while i < n {
                    ni = (mp as *mut libc::c_char)
                        .offset(
                            *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize)
                                as libc::c_int as isize,
                        )
                        .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                    if (*ni).mn_flags as libc::c_int & 1 as libc::c_int != 0 {
                        memcpy(
                            &mut pg as *mut pgno_t as *mut libc::c_void,
                            ((*ni).mn_data)
                                .as_mut_ptr()
                                .offset((*ni).mn_ksize as libc::c_int as isize)
                                as *mut libc::c_void as *const libc::c_void,
                            ::std::mem::size_of::<pgno_t>() as libc::c_ulong,
                        );
                        rc = mdb_page_get(
                            mc,
                            pg,
                            &mut omp,
                            0 as *mut libc::c_void as *mut libc::c_int,
                        );
                        if rc != 0 as libc::c_int {
                            current_block = 3961749645939082954;
                            break 's_73;
                        }
                        if !((*omp).mp_flags as libc::c_int & 4 as libc::c_int
                            == 4 as libc::c_int)
                        {
                            mdb_assert_fail(
                                (*(*mc).mc_txn).mt_env,
                                b"IS_OVERFLOW(omp)\0" as *const u8 as *const libc::c_char,
                                b"mdb_drop0\0" as *const u8 as *const libc::c_char,
                                b"mdb.c\0" as *const u8 as *const libc::c_char,
                                10975 as libc::c_int,
                            );
                        }
                        rc = mdb_midl_append_range(
                            &mut (*txn).mt_free_pgs,
                            pg,
                            (*omp).mp_pb.pb_pages,
                        );
                        if rc != 0 {
                            current_block = 3961749645939082954;
                            break 's_73;
                        }
                        (*(*mc).mc_db)
                            .md_overflow_pages = ((*(*mc).mc_db).md_overflow_pages
                            as libc::c_ulong)
                            .wrapping_sub((*omp).mp_pb.pb_pages as pgno_t) as pgno_t
                            as pgno_t;
                        if (*(*mc).mc_db).md_overflow_pages == 0 {
                            if subs == 0 {
                                break;
                            }
                        }
                    } else if subs != 0 {
                        if (*ni).mn_flags as libc::c_int & 2 as libc::c_int != 0 {
                            mdb_xcursor_init1(mc, ni);
                            rc = mdb_drop0(
                                &mut (*(*mc).mc_xcursor).mx_cursor,
                                0 as libc::c_int,
                            );
                            if rc != 0 {
                                current_block = 3961749645939082954;
                                break 's_73;
                            }
                        }
                    }
                    i = i.wrapping_add(1);
                }
                if subs == 0 {
                    if (*(*mc).mc_db).md_overflow_pages == 0 {
                        current_block = 16721787309824851445;
                    } else {
                        current_block = 5892776923941496671;
                    }
                } else {
                    current_block = 5892776923941496671;
                }
            } else {
                rc = mdb_midl_need(&mut (*txn).mt_free_pgs, n);
                if rc != 0 as libc::c_int {
                    current_block = 3961749645939082954;
                    break;
                }
                i = 0 as libc::c_uint;
                while i < n {
                    ni = (mp as *mut libc::c_char)
                        .offset(
                            *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize)
                                as libc::c_int as isize,
                        )
                        .offset(0 as libc::c_uint as isize) as *mut MDB_node;
                    pg___0 = (*ni).mn_lo as libc::c_ulong
                        | ((*ni).mn_hi as pgno_t) << 16 as libc::c_int
                        | ((*ni).mn_flags as pgno_t) << 32 as libc::c_int;
                    xidl = (*txn).mt_free_pgs;
                    let ref mut fresh32 = *xidl.offset(0 as libc::c_int as isize);
                    *fresh32 = (*fresh32).wrapping_add(1);
                    xlen = *xidl.offset(0 as libc::c_int as isize);
                    *xidl.offset(xlen as isize) = pg___0;
                    i = i.wrapping_add(1);
                }
                current_block = 5892776923941496671;
            }
            match current_block {
                5892776923941496671 => {
                    if (*mc).mc_top == 0 {
                        current_block = 1134115459065347084;
                        break;
                    }
                    (*mc).mc_ki[(*mc).mc_top as usize] = i as indx_t;
                    rc = mdb_cursor_sibling(mc, 1 as libc::c_int);
                    if !(rc != 0) {
                        continue;
                    }
                    if rc != -(30798 as libc::c_int) {
                        current_block = 3961749645939082954;
                        break;
                    }
                }
                _ => {}
            }
            mdb_cursor_pop(mc);
            (*mc).mc_ki[0 as libc::c_int as usize] = 0 as libc::c_int as indx_t;
            i = 1 as libc::c_uint;
            while i < (*mc).mc_snum as libc::c_uint {
                (*mc).mc_ki[i as usize] = 0 as libc::c_int as indx_t;
                (*mc).mc_pg[i as usize] = mx.mc_pg[i as usize];
                i = i.wrapping_add(1);
            }
        }
        match current_block {
            1134115459065347084 => {
                rc = mdb_midl_append(&mut (*txn).mt_free_pgs, (*(*mc).mc_db).md_root);
            }
            _ => {}
        }
        if rc != 0 {
            (*txn).mt_flags |= 2 as libc::c_uint;
        }
    } else if rc == -(30798 as libc::c_int) {
        rc = 0 as libc::c_int;
    }
    (*mc).mc_flags &= 4294967294 as libc::c_uint;
    return rc;
}
pub unsafe extern "C" fn mdb_drop(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut del: libc::c_int,
) -> libc::c_int {
    let mut mc: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut m2: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut rc: libc::c_int = 0;
    if del as libc::c_uint > 1 as libc::c_uint {
        return 22 as libc::c_int
    } else {
        if !txn.is_null() {
            if dbi < (*txn).mt_numdbs {
                if *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int
                    & 16 as libc::c_int == 0
                {
                    return 22 as libc::c_int;
                }
            } else {
                return 22 as libc::c_int
            }
        } else {
            return 22 as libc::c_int
        }
    }
    if (*txn).mt_flags & 131072 as libc::c_uint == 131072 as libc::c_uint {
        return 13 as libc::c_int;
    }
    if *((*txn).mt_dbiseqs).offset(dbi as isize)
        != *((*(*txn).mt_env).me_dbiseqs).offset(dbi as isize)
    {
        return -(30780 as libc::c_int);
    }
    rc = mdb_cursor_open(txn, dbi, &mut mc);
    if rc != 0 {
        return rc;
    }
    rc = mdb_drop0(mc, (*(*mc).mc_db).md_flags as libc::c_int & 4 as libc::c_int);
    m2 = *((*txn).mt_cursors).offset(dbi as isize);
    while !m2.is_null() {
        (*m2).mc_flags &= 4294967292 as libc::c_uint;
        m2 = (*m2).mc_next;
    }
    if !(rc != 0) {
        let mut current_block_45: u64;
        if del != 0 {
            if dbi >= 2 as libc::c_uint {
                rc = mdb_del0(
                    txn,
                    1 as libc::c_int as MDB_dbi,
                    &mut (*(*mc).mc_dbx).md_name,
                    0 as *mut libc::c_void as *mut MDB_val,
                    2 as libc::c_uint,
                );
                if rc == 0 {
                    *((*txn).mt_dbflags)
                        .offset(dbi as isize) = 2 as libc::c_int as libc::c_uchar;
                    mdb_dbi_close((*txn).mt_env, dbi);
                } else {
                    (*txn).mt_flags |= 2 as libc::c_uint;
                }
                current_block_45 = 3123434771885419771;
            } else {
                current_block_45 = 16488481916362631568;
            }
        } else {
            current_block_45 = 16488481916362631568;
        }
        match current_block_45 {
            16488481916362631568 => {
                *((*txn).mt_dbflags)
                    .offset(
                        dbi as isize,
                    ) = (*((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int
                    | 1 as libc::c_int) as libc::c_uchar;
                (*((*txn).mt_dbs).offset(dbi as isize))
                    .md_depth = 0 as libc::c_int as uint16_t;
                (*((*txn).mt_dbs).offset(dbi as isize))
                    .md_branch_pages = 0 as libc::c_int as pgno_t;
                (*((*txn).mt_dbs).offset(dbi as isize))
                    .md_leaf_pages = 0 as libc::c_int as pgno_t;
                (*((*txn).mt_dbs).offset(dbi as isize))
                    .md_overflow_pages = 0 as libc::c_int as pgno_t;
                (*((*txn).mt_dbs).offset(dbi as isize))
                    .md_entries = 0 as libc::c_int as mdb_size_t;
                (*((*txn).mt_dbs).offset(dbi as isize))
                    .md_root = !(0 as libc::c_int as pgno_t);
                (*txn).mt_flags |= 4 as libc::c_uint;
            }
            _ => {}
        }
    }
    mdb_cursor_close(mc);
    return rc;
}
pub unsafe extern "C" fn mdb_set_compare(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut cmp: Option::<MDB_cmp_func>,
) -> libc::c_int {
    if !txn.is_null() {
        if dbi < (*txn).mt_numdbs {
            if *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int
                & 16 as libc::c_int == 0
            {
                return 22 as libc::c_int;
            }
        } else {
            return 22 as libc::c_int
        }
    } else {
        return 22 as libc::c_int
    }
    let ref mut fresh33 = (*((*txn).mt_dbxs).offset(dbi as isize)).md_cmp;
    *fresh33 = cmp;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_set_dupsort(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut cmp: Option::<MDB_cmp_func>,
) -> libc::c_int {
    if !txn.is_null() {
        if dbi < (*txn).mt_numdbs {
            if *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int
                & 16 as libc::c_int == 0
            {
                return 22 as libc::c_int;
            }
        } else {
            return 22 as libc::c_int
        }
    } else {
        return 22 as libc::c_int
    }
    let ref mut fresh34 = (*((*txn).mt_dbxs).offset(dbi as isize)).md_dcmp;
    *fresh34 = cmp;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_set_relfunc(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut rel: Option::<MDB_rel_func>,
) -> libc::c_int {
    if !txn.is_null() {
        if dbi < (*txn).mt_numdbs {
            if *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int
                & 16 as libc::c_int == 0
            {
                return 22 as libc::c_int;
            }
        } else {
            return 22 as libc::c_int
        }
    } else {
        return 22 as libc::c_int
    }
    let ref mut fresh35 = (*((*txn).mt_dbxs).offset(dbi as isize)).md_rel;
    *fresh35 = rel;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_set_relctx(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut ctx: *mut libc::c_void,
) -> libc::c_int {
    if !txn.is_null() {
        if dbi < (*txn).mt_numdbs {
            if *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int
                & 16 as libc::c_int == 0
            {
                return 22 as libc::c_int;
            }
        } else {
            return 22 as libc::c_int
        }
    } else {
        return 22 as libc::c_int
    }
    let ref mut fresh36 = (*((*txn).mt_dbxs).offset(dbi as isize)).md_relctx;
    *fresh36 = ctx;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_env_get_maxkeysize(mut env: *mut MDB_env) -> libc::c_int {
    return 511 as libc::c_int;
}
pub unsafe extern "C" fn mdb_reader_list(
    mut env: *mut MDB_env,
    mut func: Option::<MDB_msg_func>,
    mut ctx: *mut libc::c_void,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut rdrs: libc::c_uint = 0;
    let mut mr: *mut MDB_reader = 0 as *mut MDB_reader;
    let mut buf: [libc::c_char; 64] = [0; 64];
    let mut rc: libc::c_int = 0;
    let mut first: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut txnid: txnid_t = 0;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    rc = 0 as libc::c_int;
    first = 1 as libc::c_int;
    if env.is_null() {
        return -(1 as libc::c_int)
    } else {
        if func.is_none() {
            return -(1 as libc::c_int);
        }
    }
    if ((*env).me_txns).is_null() {
        tmp = (Some(func.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(b"(no reader locks)\n\0" as *const u8 as *const libc::c_char, ctx);
        return tmp;
    }
    rdrs = (*(*env).me_txns).mt1.mtb.mtb_numreaders;
    mr = ((*(*env).me_txns).mti_readers).as_mut_ptr();
    i = 0 as libc::c_uint;
    while i < rdrs {
        if (*mr.offset(i as isize)).mru.mrx.mrb_pid != 0 {
            txnid = (*mr.offset(i as isize)).mru.mrx.mrb_txnid;
            if txnid == 0xffffffffffffffff as libc::c_ulong {
                tmp___0 = b"%10d %zx -\n\0" as *const u8 as *const libc::c_char;
            } else {
                tmp___0 = b"%10d %zx %zu\n\0" as *const u8 as *const libc::c_char;
            }
            sprintf(
                buf.as_mut_ptr(),
                tmp___0,
                (*mr.offset(i as isize)).mru.mrx.mrb_pid,
                (*mr.offset(i as isize)).mru.mrx.mrb_tid,
                txnid,
            );
            if first != 0 {
                first = 0 as libc::c_int;
                rc = (Some(func.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"    pid     thread     txnid\n\0" as *const u8
                        as *const libc::c_char,
                    ctx,
                );
                if rc < 0 as libc::c_int {
                    break;
                }
            }
            rc = (Some(func.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(buf.as_mut_ptr() as *const libc::c_char, ctx);
            if rc < 0 as libc::c_int {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    if first != 0 {
        rc = (Some(func.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(b"(no active readers)\n\0" as *const u8 as *const libc::c_char, ctx);
    }
    return rc;
}
unsafe extern "C" fn mdb_pid_insert(mut ids: *mut pid_t, mut pid: pid_t) -> libc::c_int {
    let mut base: libc::c_uint = 0;
    let mut cursor: libc::c_uint = 0;
    let mut val: libc::c_int = 0;
    let mut n: libc::c_uint = 0;
    let mut pivot: libc::c_uint = 0;
    base = 0 as libc::c_uint;
    cursor = 1 as libc::c_uint;
    val = 0 as libc::c_int;
    n = *ids.offset(0 as libc::c_int as isize) as libc::c_uint;
    while (0 as libc::c_uint) < n {
        pivot = n >> 1 as libc::c_int;
        cursor = base.wrapping_add(pivot).wrapping_add(1 as libc::c_uint);
        val = pid - *ids.offset(cursor as isize);
        if val < 0 as libc::c_int {
            n = pivot;
        } else if val > 0 as libc::c_int {
            base = cursor;
            n = n.wrapping_sub(pivot.wrapping_add(1 as libc::c_uint));
        } else {
            return -(1 as libc::c_int)
        }
    }
    if val > 0 as libc::c_int {
        cursor = cursor.wrapping_add(1);
    }
    let ref mut fresh37 = *ids.offset(0 as libc::c_int as isize);
    *fresh37 += 1;
    n = *ids.offset(0 as libc::c_int as isize) as libc::c_uint;
    while n > cursor {
        *ids
            .offset(
                n as isize,
            ) = *ids.offset(n.wrapping_sub(1 as libc::c_uint) as isize);
        n = n.wrapping_sub(1);
    }
    *ids.offset(n as isize) = pid;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_reader_check(
    mut env: *mut MDB_env,
    mut dead: *mut libc::c_int,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    if env.is_null() {
        return 22 as libc::c_int;
    }
    if !dead.is_null() {
        *dead = 0 as libc::c_int;
    }
    if !((*env).me_txns).is_null() {
        tmp = mdb_reader_check0(env, 0 as libc::c_int, dead);
        tmp___0 = tmp;
    } else {
        tmp___0 = 0 as libc::c_int;
    }
    return tmp___0;
}
unsafe extern "C" fn mdb_reader_check0(
    mut env: *mut MDB_env,
    mut rlocked: libc::c_int,
    mut dead: *mut libc::c_int,
) -> libc::c_int {
    let mut rmutex: mdb_mutexref_t = 0 as *mut pthread_mutex_t;
    let mut tmp: *mut pthread_mutex_t = 0 as *mut pthread_mutex_t;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut rdrs: libc::c_uint = 0;
    let mut mr: *mut MDB_reader = 0 as *mut MDB_reader;
    let mut pids: *mut pid_t = 0 as *mut pid_t;
    let mut pid: pid_t = 0;
    let mut rc: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    if rlocked != 0 {
        tmp = 0 as *mut libc::c_void as *mut pthread_mutex_t;
    } else {
        tmp = ((*(*env).me_txns).mt1.mtb.mtb_rmutex).as_mut_ptr();
    }
    rmutex = tmp;
    rc = 0 as libc::c_int;
    count = 0 as libc::c_int;
    rdrs = (*(*env).me_txns).mt1.mtb.mtb_numreaders;
    tmp___0 = malloc(
        (rdrs.wrapping_add(1 as libc::c_uint) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<pid_t>() as libc::c_ulong),
    );
    pids = tmp___0 as *mut pid_t;
    if pids.is_null() {
        return 12 as libc::c_int;
    }
    *pids.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    mr = ((*(*env).me_txns).mti_readers).as_mut_ptr();
    i = 0 as libc::c_uint;
    while i < rdrs {
        pid = (*mr.offset(i as isize)).mru.mrx.mrb_pid;
        if pid != 0 {
            if pid != (*env).me_pid {
                tmp___3 = mdb_pid_insert(pids, pid);
                if tmp___3 == 0 as libc::c_int {
                    tmp___2 = mdb_reader_pid(env, Pidcheck, pid);
                    if tmp___2 == 0 {
                        j = i;
                        if !rmutex.is_null() {
                            rc = pthread_mutex_lock(rmutex);
                            if rc != 0 as libc::c_int {
                                rc = mdb_mutex_failed(env, rmutex, rc);
                                if rc != 0 {
                                    break;
                                }
                                rdrs = 0 as libc::c_uint;
                            } else {
                                tmp___1 = mdb_reader_pid(env, Pidcheck, pid);
                                if tmp___1 != 0 {
                                    j = rdrs;
                                }
                            }
                        }
                        while j < rdrs {
                            if (*mr.offset(j as isize)).mru.mrx.mrb_pid == pid {
                                (*mr.offset(j as isize)).mru.mrx.mrb_pid = 0 as libc::c_int;
                                count += 1;
                            }
                            j = j.wrapping_add(1);
                        }
                        if !rmutex.is_null() {
                            pthread_mutex_unlock(rmutex);
                        }
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
    free(pids as *mut libc::c_void);
    if !dead.is_null() {
        *dead = count;
    }
    return rc;
}
unsafe extern "C" fn mdb_mutex_failed(
    mut env: *mut MDB_env,
    mut mutex: mdb_mutexref_t,
    mut rc: libc::c_int,
) -> libc::c_int {
    let mut rlocked: libc::c_int = 0;
    let mut rc2: libc::c_int = 0;
    let mut meta: *mut MDB_meta = 0 as *mut MDB_meta;
    if rc == 130 as libc::c_int {
        rc = 0 as libc::c_int;
        rlocked = (mutex as libc::c_ulong
            == ((*(*env).me_txns).mt1.mtb.mtb_rmutex).as_mut_ptr() as libc::c_ulong)
            as libc::c_int;
        if rlocked == 0 {
            meta = mdb_env_pick_meta(env as *const MDB_env);
            (*(*env).me_txns).mt1.mtb.mtb_txnid = (*meta).mm_txnid;
            if !((*env).me_txn).is_null() {
                (*env).me_flags |= 2147483648 as libc::c_uint;
                (*env).me_txn = 0 as *mut libc::c_void as *mut MDB_txn;
                rc = -(30795 as libc::c_int);
            }
        }
        rc2 = mdb_reader_check0(
            env,
            rlocked,
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        if rc2 == 0 as libc::c_int {
            rc2 = pthread_mutex_consistent(mutex);
        }
        if rc != 0 {
            pthread_mutex_unlock(mutex);
        } else {
            rc = rc2;
            if rc != 0 {
                pthread_mutex_unlock(mutex);
            }
        }
    }
    return rc;
}
pub unsafe extern "C" fn mdb_midl_search(
    mut ids: MDB_IDL,
    mut id: MDB_ID,
) -> libc::c_uint {
    let mut base: libc::c_uint = 0;
    let mut cursor: libc::c_uint = 0;
    let mut val: libc::c_int = 0;
    let mut n: libc::c_uint = 0;
    let mut pivot: libc::c_uint = 0;
    base = 0 as libc::c_uint;
    cursor = 1 as libc::c_uint;
    val = 0 as libc::c_int;
    n = *ids.offset(0 as libc::c_int as isize) as libc::c_uint;
    while (0 as libc::c_uint) < n {
        pivot = n >> 1 as libc::c_int;
        cursor = base.wrapping_add(pivot).wrapping_add(1 as libc::c_uint);
        if *ids.offset(cursor as isize) < id {
            val = -(1 as libc::c_int);
        } else {
            val = (*ids.offset(cursor as isize) > id) as libc::c_int;
        }
        if val < 0 as libc::c_int {
            n = pivot;
        } else if val > 0 as libc::c_int {
            base = cursor;
            n = n.wrapping_sub(pivot.wrapping_add(1 as libc::c_uint));
        } else {
            return cursor
        }
    }
    if val > 0 as libc::c_int {
        cursor = cursor.wrapping_add(1);
    }
    return cursor;
}
pub unsafe extern "C" fn mdb_midl_alloc(mut num: libc::c_int) -> MDB_IDL {
    let mut ids: MDB_IDL = 0 as *mut MDB_ID;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: MDB_IDL = 0 as *mut MDB_ID;
    tmp = malloc(
        ((num + 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<MDB_ID>() as libc::c_ulong),
    );
    ids = tmp as MDB_IDL;
    if !ids.is_null() {
        tmp___0 = ids;
        ids = ids.offset(1);
        *tmp___0 = num as MDB_ID;
        *ids = 0 as libc::c_int as MDB_ID;
    }
    return ids;
}
pub unsafe extern "C" fn mdb_midl_free(mut ids: MDB_IDL) {
    if !ids.is_null() {
        free(ids.offset(-(1 as libc::c_int as isize)) as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn mdb_midl_shrink(mut idp: *mut MDB_IDL) {
    let mut ids: MDB_IDL = 0 as *mut MDB_ID;
    let mut tmp: MDB_IDL = 0 as *mut MDB_ID;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    ids = *idp;
    ids = ids.offset(-1);
    if *ids > (((1 as libc::c_int) << 17 as libc::c_int) - 1 as libc::c_int) as MDB_ID {
        tmp___0 = realloc(
            ids as *mut libc::c_void,
            ((((1 as libc::c_int) << 17 as libc::c_int) - 1 as libc::c_int
                + 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<MDB_ID>() as libc::c_ulong),
        );
        ids = tmp___0 as MDB_IDL;
        if !ids.is_null() {
            tmp = ids;
            ids = ids.offset(1);
            *tmp = (((1 as libc::c_int) << 17 as libc::c_int) - 1 as libc::c_int)
                as MDB_ID;
            *idp = ids;
        }
    }
}
unsafe extern "C" fn mdb_midl_grow(
    mut idp: *mut MDB_IDL,
    mut num: libc::c_int,
) -> libc::c_int {
    let mut idn: MDB_IDL = 0 as *mut MDB_ID;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: MDB_IDL = 0 as *mut MDB_ID;
    idn = (*idp).offset(-(1 as libc::c_int as isize));
    tmp = realloc(
        idn as *mut libc::c_void,
        (*idn)
            .wrapping_add(num as MDB_ID)
            .wrapping_add(2 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<MDB_ID>() as libc::c_ulong),
    );
    idn = tmp as MDB_IDL;
    if idn.is_null() {
        return 12 as libc::c_int;
    }
    tmp___0 = idn;
    idn = idn.offset(1);
    *tmp___0 = (*tmp___0 as libc::c_ulong).wrapping_add(num as MDB_ID) as MDB_ID
        as MDB_ID;
    *idp = idn;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_midl_need(
    mut idp: *mut MDB_IDL,
    mut num: libc::c_uint,
) -> libc::c_int {
    let mut ids: MDB_IDL = 0 as *mut MDB_ID;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: MDB_IDL = 0 as *mut MDB_ID;
    ids = *idp;
    num = (num as MDB_ID).wrapping_add(*ids.offset(0 as libc::c_int as isize))
        as libc::c_uint;
    if num as MDB_ID > *ids.offset(-(1 as libc::c_int) as isize) {
        num = num
            .wrapping_add(num.wrapping_div(4 as libc::c_uint))
            .wrapping_add(258 as libc::c_uint) & 4294967040 as libc::c_uint;
        tmp = realloc(
            ids.offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
            (num as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<MDB_ID>() as libc::c_ulong),
        );
        ids = tmp as MDB_IDL;
        if ids.is_null() {
            return 12 as libc::c_int;
        }
        tmp___0 = ids;
        ids = ids.offset(1);
        *tmp___0 = num.wrapping_sub(2 as libc::c_uint) as MDB_ID;
        *idp = ids;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_midl_append(
    mut idp: *mut MDB_IDL,
    mut id: MDB_ID,
) -> libc::c_int {
    let mut ids: MDB_IDL = 0 as *mut MDB_ID;
    let mut tmp: libc::c_int = 0;
    ids = *idp;
    if *ids.offset(0 as libc::c_int as isize)
        >= *ids.offset(-(1 as libc::c_int) as isize)
    {
        tmp = mdb_midl_grow(
            idp,
            ((1 as libc::c_int) << 17 as libc::c_int) - 1 as libc::c_int,
        );
        if tmp != 0 {
            return 12 as libc::c_int;
        }
        ids = *idp;
    }
    let ref mut fresh38 = *ids.offset(0 as libc::c_int as isize);
    *fresh38 = (*fresh38).wrapping_add(1);
    *ids.offset(*ids.offset(0 as libc::c_int as isize) as isize) = id;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_midl_append_list(
    mut idp: *mut MDB_IDL,
    mut app: MDB_IDL,
) -> libc::c_int {
    let mut ids: MDB_IDL = 0 as *mut MDB_ID;
    let mut tmp: libc::c_int = 0;
    ids = *idp;
    if (*ids.offset(0 as libc::c_int as isize))
        .wrapping_add(*app.offset(0 as libc::c_int as isize))
        >= *ids.offset(-(1 as libc::c_int) as isize)
    {
        tmp = mdb_midl_grow(idp, *app.offset(0 as libc::c_int as isize) as libc::c_int);
        if tmp != 0 {
            return 12 as libc::c_int;
        }
        ids = *idp;
    }
    memcpy(
        ids
            .offset(
                (*ids.offset(0 as libc::c_int as isize)).wrapping_add(1 as libc::c_ulong)
                    as isize,
            ) as *mut libc::c_void,
        app.offset(1 as libc::c_int as isize) as *const libc::c_void,
        (*app.offset(0 as libc::c_int as isize))
            .wrapping_mul(::std::mem::size_of::<MDB_ID>() as libc::c_ulong),
    );
    let ref mut fresh39 = *ids.offset(0 as libc::c_int as isize);
    *fresh39 = (*fresh39 as libc::c_ulong)
        .wrapping_add(*app.offset(0 as libc::c_int as isize)) as MDB_ID as MDB_ID;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_midl_append_range(
    mut idp: *mut MDB_IDL,
    mut id: MDB_ID,
    mut n: libc::c_uint,
) -> libc::c_int {
    let mut ids: *mut MDB_ID = 0 as *mut MDB_ID;
    let mut len: MDB_ID = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_uint = 0;
    let mut tmp___1: MDB_ID = 0;
    ids = *idp;
    len = *ids.offset(0 as libc::c_int as isize);
    if len.wrapping_add(n as MDB_ID) > *ids.offset(-(1 as libc::c_int) as isize) {
        tmp = mdb_midl_grow(
            idp,
            (n
                | (((1 as libc::c_int) << 17 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint) as libc::c_int,
        );
        if tmp != 0 {
            return 12 as libc::c_int;
        }
        ids = *idp;
    }
    *ids.offset(0 as libc::c_int as isize) = len.wrapping_add(n as MDB_ID);
    ids = ids.offset(len as isize);
    while n != 0 {
        tmp___0 = n;
        n = n.wrapping_sub(1);
        tmp___1 = id;
        id = id.wrapping_add(1);
        *ids.offset(tmp___0 as isize) = tmp___1;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_midl_xmerge(mut idl: MDB_IDL, mut merge: MDB_IDL) {
    let mut old_id: MDB_ID = 0;
    let mut merge_id: MDB_ID = 0;
    let mut i: MDB_ID = 0;
    let mut j: MDB_ID = 0;
    let mut k: MDB_ID = 0;
    let mut total: MDB_ID = 0;
    let mut tmp: MDB_ID = 0;
    let mut tmp___0: MDB_ID = 0;
    let mut tmp___1: MDB_ID = 0;
    i = *merge.offset(0 as libc::c_int as isize);
    j = *idl.offset(0 as libc::c_int as isize);
    k = i.wrapping_add(j);
    total = k;
    *idl.offset(0 as libc::c_int as isize) = -(1 as libc::c_int) as MDB_ID;
    old_id = *idl.offset(j as isize);
    while i != 0 {
        tmp = i;
        i = i.wrapping_sub(1);
        merge_id = *merge.offset(tmp as isize);
        while old_id < merge_id {
            tmp___0 = k;
            k = k.wrapping_sub(1);
            *idl.offset(tmp___0 as isize) = old_id;
            j = j.wrapping_sub(1);
            old_id = *idl.offset(j as isize);
        }
        tmp___1 = k;
        k = k.wrapping_sub(1);
        *idl.offset(tmp___1 as isize) = merge_id;
    }
    *idl.offset(0 as libc::c_int as isize) = total;
}
pub unsafe extern "C" fn mdb_midl_sort(mut ids: MDB_IDL) {
    let mut istack: [libc::c_int; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut ir: libc::c_int = 0;
    let mut jstack: libc::c_int = 0;
    let mut a: MDB_ID = 0;
    let mut itmp: MDB_ID = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    ir = *ids.offset(0 as libc::c_int as isize) as libc::c_int;
    l = 1 as libc::c_int;
    jstack = 0 as libc::c_int;
    loop {
        if ir - l < 8 as libc::c_int {
            j = l + 1 as libc::c_int;
            while j <= ir {
                a = *ids.offset(j as isize);
                i = j - 1 as libc::c_int;
                while i >= 1 as libc::c_int {
                    if *ids.offset(i as isize) >= a {
                        break;
                    }
                    *ids
                        .offset(
                            (i + 1 as libc::c_int) as isize,
                        ) = *ids.offset(i as isize);
                    i -= 1;
                }
                *ids.offset((i + 1 as libc::c_int) as isize) = a;
                j += 1;
            }
            if jstack == 0 as libc::c_int {
                break;
            }
            tmp = jstack;
            jstack -= 1;
            ir = istack[tmp as usize];
            tmp___0 = jstack;
            jstack -= 1;
            l = istack[tmp___0 as usize];
        } else {
            k = l + ir >> 1 as libc::c_int;
            itmp = *ids.offset(k as isize);
            *ids.offset(k as isize) = *ids.offset((l + 1 as libc::c_int) as isize);
            *ids.offset((l + 1 as libc::c_int) as isize) = itmp;
            if *ids.offset(l as isize) < *ids.offset(ir as isize) {
                itmp = *ids.offset(l as isize);
                *ids.offset(l as isize) = *ids.offset(ir as isize);
                *ids.offset(ir as isize) = itmp;
            }
            if *ids.offset((l + 1 as libc::c_int) as isize) < *ids.offset(ir as isize) {
                itmp = *ids.offset((l + 1 as libc::c_int) as isize);
                *ids.offset((l + 1 as libc::c_int) as isize) = *ids.offset(ir as isize);
                *ids.offset(ir as isize) = itmp;
            }
            if *ids.offset(l as isize) < *ids.offset((l + 1 as libc::c_int) as isize) {
                itmp = *ids.offset(l as isize);
                *ids.offset(l as isize) = *ids.offset((l + 1 as libc::c_int) as isize);
                *ids.offset((l + 1 as libc::c_int) as isize) = itmp;
            }
            i = l + 1 as libc::c_int;
            j = ir;
            a = *ids.offset((l + 1 as libc::c_int) as isize);
            loop {
                loop {
                    i += 1;
                    if !(*ids.offset(i as isize) > a) {
                        break;
                    }
                }
                loop {
                    j -= 1;
                    if !(*ids.offset(j as isize) < a) {
                        break;
                    }
                }
                if j < i {
                    break;
                }
                itmp = *ids.offset(i as isize);
                *ids.offset(i as isize) = *ids.offset(j as isize);
                *ids.offset(j as isize) = itmp;
            }
            *ids.offset((l + 1 as libc::c_int) as isize) = *ids.offset(j as isize);
            *ids.offset(j as isize) = a;
            jstack += 2 as libc::c_int;
            if ir - i + 1 as libc::c_int >= j - l {
                istack[jstack as usize] = ir;
                istack[(jstack - 1 as libc::c_int) as usize] = i;
                ir = j - 1 as libc::c_int;
            } else {
                istack[jstack as usize] = j - 1 as libc::c_int;
                istack[(jstack - 1 as libc::c_int) as usize] = l;
                l = i;
            }
        }
    };
}
pub unsafe extern "C" fn mdb_mid2l_search(
    mut ids: MDB_ID2L,
    mut id: MDB_ID,
) -> libc::c_uint {
    let mut base: libc::c_uint = 0;
    let mut cursor: libc::c_uint = 0;
    let mut val: libc::c_int = 0;
    let mut n: libc::c_uint = 0;
    let mut pivot: libc::c_uint = 0;
    base = 0 as libc::c_uint;
    cursor = 1 as libc::c_uint;
    val = 0 as libc::c_int;
    n = (*ids.offset(0 as libc::c_int as isize)).mid as libc::c_uint;
    while (0 as libc::c_uint) < n {
        pivot = n >> 1 as libc::c_int;
        cursor = base.wrapping_add(pivot).wrapping_add(1 as libc::c_uint);
        if id < (*ids.offset(cursor as isize)).mid {
            val = -(1 as libc::c_int);
        } else {
            val = (id > (*ids.offset(cursor as isize)).mid) as libc::c_int;
        }
        if val < 0 as libc::c_int {
            n = pivot;
        } else if val > 0 as libc::c_int {
            base = cursor;
            n = n.wrapping_sub(pivot.wrapping_add(1 as libc::c_uint));
        } else {
            return cursor
        }
    }
    if val > 0 as libc::c_int {
        cursor = cursor.wrapping_add(1);
    }
    return cursor;
}
pub unsafe extern "C" fn mdb_mid2l_insert(
    mut ids: MDB_ID2L,
    mut id: *mut MDB_ID2,
) -> libc::c_int {
    let mut x: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    x = mdb_mid2l_search(ids, (*id).mid);
    if x < 1 as libc::c_uint {
        return -(2 as libc::c_int);
    }
    if x as MDB_ID <= (*ids.offset(0 as libc::c_int as isize)).mid {
        if (*ids.offset(x as isize)).mid == (*id).mid {
            return -(1 as libc::c_int);
        }
    }
    if (*ids.offset(0 as libc::c_int as isize)).mid
        >= (((1 as libc::c_int) << 17 as libc::c_int) - 1 as libc::c_int) as MDB_ID
    {
        return -(2 as libc::c_int)
    } else {
        let ref mut fresh40 = (*ids.offset(0 as libc::c_int as isize)).mid;
        *fresh40 = (*fresh40).wrapping_add(1);
        i = (*ids.offset(0 as libc::c_int as isize)).mid as libc::c_uint;
        while i > x {
            *ids
                .offset(
                    i as isize,
                ) = *ids.offset(i.wrapping_sub(1 as libc::c_uint) as isize);
            i = i.wrapping_sub(1);
        }
        *ids.offset(x as isize) = *id;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_mid2l_append(
    mut ids: MDB_ID2L,
    mut id: *mut MDB_ID2,
) -> libc::c_int {
    if (*ids.offset(0 as libc::c_int as isize)).mid
        >= (((1 as libc::c_int) << 17 as libc::c_int) - 1 as libc::c_int) as MDB_ID
    {
        return -(2 as libc::c_int);
    }
    let ref mut fresh41 = (*ids.offset(0 as libc::c_int as isize)).mid;
    *fresh41 = (*fresh41).wrapping_add(1);
    *ids.offset((*ids.offset(0 as libc::c_int as isize)).mid as isize) = *id;
    return 0 as libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut env: *mut MDB_env = 0 as *mut MDB_env;
    let mut dbi: MDB_dbi = 0;
    let mut key: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut data: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut mst: MDB_stat = MDB_stat {
        ms_psize: 0,
        ms_depth: 0,
        ms_branch_pages: 0,
        ms_leaf_pages: 0,
        ms_overflow_pages: 0,
        ms_entries: 0,
    };
    let mut cursor: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut cur2: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut op: MDB_cursor_op = MDB_FIRST;
    let mut count: libc::c_int = 0;
    let mut values: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut sval: [libc::c_char; 32] = [0; 32];
    let mut tmp: libc::c_uint = 0;
    let mut tmp___0: time_t = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___7: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___8: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___9: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___10: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___11: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___12: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___13: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___14: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___15: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___16: libc::c_int = 0;
    let mut tmp___17: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___18: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___19: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___20: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___21: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___22: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___23: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___24: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___25: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___26: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___27: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___28: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___29: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___30: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___31: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___32: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___33: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___34: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___35: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___36: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    j = 0 as libc::c_int;
    sval[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    tmp = 1 as libc::c_uint;
    while !(tmp >= 32 as libc::c_uint) {
        sval[tmp as usize] = 0 as libc::c_int as libc::c_char;
        tmp = tmp.wrapping_add(1);
    }
    tmp___0 = time(0 as *mut libc::c_void as *mut time_t);
    srand(tmp___0 as libc::c_uint);
    tmp___1 = rand();
    count = tmp___1 % 384 as libc::c_int + 64 as libc::c_int;
    tmp___2 = malloc(
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    values = tmp___2 as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < count {
        tmp___3 = rand();
        *values.offset(i as isize) = tmp___3 % 1024 as libc::c_int;
        i += 1;
    }
    rc = mdb_env_create(&mut env);
    if !(rc == 0 as libc::c_int) {
        tmp___4 = mdb_strerror(rc);
        fprintf(
            stderr,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int,
            b"mdb_env_create(&env)\0" as *const u8 as *const libc::c_char,
            tmp___4,
        );
        abort();
    }
    rc = mdb_env_set_maxreaders(env, 1 as libc::c_uint);
    if !(rc == 0 as libc::c_int) {
        tmp___5 = mdb_strerror(rc);
        fprintf(
            stderr,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            b"mdb_env_set_maxreaders(env, 1)\0" as *const u8 as *const libc::c_char,
            tmp___5,
        );
        abort();
    }
    rc = mdb_env_set_mapsize(env, 10485760 as libc::c_int as mdb_size_t);
    if !(rc == 0 as libc::c_int) {
        tmp___6 = mdb_strerror(rc);
        fprintf(
            stderr,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int,
            b"mdb_env_set_mapsize(env, 10485760)\0" as *const u8 as *const libc::c_char,
            tmp___6,
        );
        abort();
    }
    rc = mdb_env_open(
        env,
        b"./testdb\0" as *const u8 as *const libc::c_char,
        1 as libc::c_uint,
        436 as libc::c_int as mdb_mode_t,
    );
    if !(rc == 0 as libc::c_int) {
        tmp___7 = mdb_strerror(rc);
        fprintf(
            stderr,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest.c\0" as *const u8 as *const libc::c_char,
            50 as libc::c_int,
            b"mdb_env_open(env, \"./testdb\", MDB_FIXEDMAP , 0664)\0" as *const u8
                as *const libc::c_char,
            tmp___7,
        );
        abort();
    }
    rc = mdb_txn_begin(
        env,
        0 as *mut libc::c_void as *mut MDB_txn,
        0 as libc::c_uint,
        &mut txn,
    );
    if !(rc == 0 as libc::c_int) {
        tmp___8 = mdb_strerror(rc);
        fprintf(
            stderr,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int,
            b"mdb_txn_begin(env, NULL, 0, &txn)\0" as *const u8 as *const libc::c_char,
            tmp___8,
        );
        abort();
    }
    rc = mdb_dbi_open(
        txn,
        0 as *mut libc::c_void as *const libc::c_char,
        0 as libc::c_uint,
        &mut dbi,
    );
    if !(rc == 0 as libc::c_int) {
        tmp___9 = mdb_strerror(rc);
        fprintf(
            stderr,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            b"mdb_dbi_open(txn, NULL, 0, &dbi)\0" as *const u8 as *const libc::c_char,
            tmp___9,
        );
        abort();
    }
    key.mv_size = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong;
    key.mv_data = sval.as_mut_ptr() as *mut libc::c_void;
    printf(b"Adding %d values\n\0" as *const u8 as *const libc::c_char, count);
    i = 0 as libc::c_int;
    while i < count {
        sprintf(
            sval.as_mut_ptr(),
            b"%03x %d foo bar\0" as *const u8 as *const libc::c_char,
            *values.offset(i as isize),
            *values.offset(i as isize),
        );
        data.mv_size = ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong;
        data.mv_data = sval.as_mut_ptr() as *mut libc::c_void;
        rc = mdb_put(txn, dbi, &mut key, &mut data, 16 as libc::c_uint);
        if rc == -(30799 as libc::c_int) {
            j += 1;
            data.mv_size = ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong;
            data.mv_data = sval.as_mut_ptr() as *mut libc::c_void;
        } else if rc != 0 {
            tmp___10 = mdb_strerror(rc);
            fprintf(
                stderr,
                b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
                b"mtest.c\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int,
                b"mdb_put(txn, dbi, &key, &data, MDB_NOOVERWRITE)\0" as *const u8
                    as *const libc::c_char,
                tmp___10,
            );
            abort();
        }
        i += 1;
    }
    if j != 0 {
        printf(b"%d duplicates skipped\n\0" as *const u8 as *const libc::c_char, j);
    }
    rc = mdb_txn_commit(txn);
    if !(rc == 0 as libc::c_int) {
        tmp___11 = mdb_strerror(rc);
        fprintf(
            stderr,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int,
            b"mdb_txn_commit(txn)\0" as *const u8 as *const libc::c_char,
            tmp___11,
        );
        abort();
    }
    rc = mdb_env_stat(env, &mut mst);
    if !(rc == 0 as libc::c_int) {
        tmp___12 = mdb_strerror(rc);
        fprintf(
            stderr,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int,
            b"mdb_env_stat(env, &mst)\0" as *const u8 as *const libc::c_char,
            tmp___12,
        );
        abort();
    }
    rc = mdb_txn_begin(
        env,
        0 as *mut libc::c_void as *mut MDB_txn,
        131072 as libc::c_uint,
        &mut txn,
    );
    if !(rc == 0 as libc::c_int) {
        tmp___13 = mdb_strerror(rc);
        fprintf(
            stderr,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int,
            b"mdb_txn_begin(env, NULL, MDB_RDONLY, &txn)\0" as *const u8
                as *const libc::c_char,
            tmp___13,
        );
        abort();
    }
    rc = mdb_cursor_open(txn, dbi, &mut cursor);
    if !(rc == 0 as libc::c_int) {
        tmp___14 = mdb_strerror(rc);
        fprintf(
            stderr,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int,
            b"mdb_cursor_open(txn, dbi, &cursor)\0" as *const u8 as *const libc::c_char,
            tmp___14,
        );
        abort();
    }
    loop {
        rc = mdb_cursor_get(cursor, &mut key, &mut data, MDB_NEXT);
        if !(rc == 0 as libc::c_int) {
            break;
        }
        printf(
            b"key: %p %.*s, data: %p %.*s\n\0" as *const u8 as *const libc::c_char,
            key.mv_data,
            key.mv_size as libc::c_int,
            key.mv_data as *mut libc::c_char,
            data.mv_data,
            data.mv_size as libc::c_int,
            data.mv_data as *mut libc::c_char,
        );
    }
    if !(rc == -(30798 as libc::c_int)) {
        tmp___15 = mdb_strerror(rc);
        fprintf(
            stderr,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int,
            b"mdb_cursor_get\0" as *const u8 as *const libc::c_char,
            tmp___15,
        );
        abort();
    }
    mdb_cursor_close(cursor);
    mdb_txn_abort(txn);
    j = 0 as libc::c_int;
    key.mv_data = sval.as_mut_ptr() as *mut libc::c_void;
    i = count - 1 as libc::c_int;
    while i > -(1 as libc::c_int) {
        j += 1;
        txn = 0 as *mut libc::c_void as *mut MDB_txn;
        rc = mdb_txn_begin(
            env,
            0 as *mut libc::c_void as *mut MDB_txn,
            0 as libc::c_uint,
            &mut txn,
        );
        if !(rc == 0 as libc::c_int) {
            tmp___17 = mdb_strerror(rc);
            fprintf(
                stderr,
                b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
                b"mtest.c\0" as *const u8 as *const libc::c_char,
                90 as libc::c_int,
                b"mdb_txn_begin(env, NULL, 0, &txn)\0" as *const u8
                    as *const libc::c_char,
                tmp___17,
            );
            abort();
        }
        sprintf(
            sval.as_mut_ptr(),
            b"%03x \0" as *const u8 as *const libc::c_char,
            *values.offset(i as isize),
        );
        rc = mdb_del(txn, dbi, &mut key, 0 as *mut libc::c_void as *mut MDB_val);
        if rc == -(30798 as libc::c_int) {
            j -= 1;
            mdb_txn_abort(txn);
        } else {
            if rc != 0 {
                tmp___19 = mdb_strerror(rc);
                fprintf(
                    stderr,
                    b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
                    b"mtest.c\0" as *const u8 as *const libc::c_char,
                    92 as libc::c_int,
                    b"mdb_del(txn, dbi, &key, NULL)\0" as *const u8
                        as *const libc::c_char,
                    tmp___19,
                );
                abort();
            }
            rc = mdb_txn_commit(txn);
            if !(rc == 0 as libc::c_int) {
                tmp___18 = mdb_strerror(rc);
                fprintf(
                    stderr,
                    b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
                    b"mtest.c\0" as *const u8 as *const libc::c_char,
                    96 as libc::c_int,
                    b"mdb_txn_commit(txn)\0" as *const u8 as *const libc::c_char,
                    tmp___18,
                );
                abort();
            }
        }
        tmp___16 = rand();
        i -= tmp___16 % 5 as libc::c_int;
    }
    free(values as *mut libc::c_void);
    printf(b"Deleted %d values\n\0" as *const u8 as *const libc::c_char, j);
    rc = mdb_env_stat(env, &mut mst);
    if !(rc == 0 as libc::c_int) {
        tmp___20 = mdb_strerror(rc);
        fprintf(
            stderr,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest.c\0" as *const u8 as *const libc::c_char,
            102 as libc::c_int,
            b"mdb_env_stat(env, &mst)\0" as *const u8 as *const libc::c_char,
            tmp___20,
        );
        abort();
    }
    rc = mdb_txn_begin(
        env,
        0 as *mut libc::c_void as *mut MDB_txn,
        131072 as libc::c_uint,
        &mut txn,
    );
    if !(rc == 0 as libc::c_int) {
        tmp___21 = mdb_strerror(rc);
        fprintf(
            stderr,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int,
            b"mdb_txn_begin(env, NULL, MDB_RDONLY, &txn)\0" as *const u8
                as *const libc::c_char,
            tmp___21,
        );
        abort();
    }
    rc = mdb_cursor_open(txn, dbi, &mut cursor);
    if !(rc == 0 as libc::c_int) {
        tmp___22 = mdb_strerror(rc);
        fprintf(
            stderr,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int,
            b"mdb_cursor_open(txn, dbi, &cursor)\0" as *const u8 as *const libc::c_char,
            tmp___22,
        );
        abort();
    }
    printf(b"Cursor next\n\0" as *const u8 as *const libc::c_char);
    loop {
        rc = mdb_cursor_get(cursor, &mut key, &mut data, MDB_NEXT);
        if !(rc == 0 as libc::c_int) {
            break;
        }
        printf(
            b"key: %.*s, data: %.*s\n\0" as *const u8 as *const libc::c_char,
            key.mv_size as libc::c_int,
            key.mv_data as *mut libc::c_char,
            data.mv_size as libc::c_int,
            data.mv_data as *mut libc::c_char,
        );
    }
    if !(rc == -(30798 as libc::c_int)) {
        tmp___23 = mdb_strerror(rc);
        fprintf(
            stderr,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            b"mdb_cursor_get\0" as *const u8 as *const libc::c_char,
            tmp___23,
        );
        abort();
    }
    printf(b"Cursor last\n\0" as *const u8 as *const libc::c_char);
    rc = mdb_cursor_get(cursor, &mut key, &mut data, MDB_LAST);
    if !(rc == 0 as libc::c_int) {
        tmp___24 = mdb_strerror(rc);
        fprintf(
            stderr,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest.c\0" as *const u8 as *const libc::c_char,
            113 as libc::c_int,
            b"mdb_cursor_get(cursor, &key, &data, MDB_LAST)\0" as *const u8
                as *const libc::c_char,
            tmp___24,
        );
        abort();
    }
    printf(
        b"key: %.*s, data: %.*s\n\0" as *const u8 as *const libc::c_char,
        key.mv_size as libc::c_int,
        key.mv_data as *mut libc::c_char,
        data.mv_size as libc::c_int,
        data.mv_data as *mut libc::c_char,
    );
    printf(b"Cursor prev\n\0" as *const u8 as *const libc::c_char);
    loop {
        rc = mdb_cursor_get(cursor, &mut key, &mut data, MDB_PREV);
        if !(rc == 0 as libc::c_int) {
            break;
        }
        printf(
            b"key: %.*s, data: %.*s\n\0" as *const u8 as *const libc::c_char,
            key.mv_size as libc::c_int,
            key.mv_data as *mut libc::c_char,
            data.mv_size as libc::c_int,
            data.mv_data as *mut libc::c_char,
        );
    }
    if !(rc == -(30798 as libc::c_int)) {
        tmp___25 = mdb_strerror(rc);
        fprintf(
            stderr,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest.c\0" as *const u8 as *const libc::c_char,
            123 as libc::c_int,
            b"mdb_cursor_get\0" as *const u8 as *const libc::c_char,
            tmp___25,
        );
        abort();
    }
    printf(b"Cursor last/prev\n\0" as *const u8 as *const libc::c_char);
    rc = mdb_cursor_get(cursor, &mut key, &mut data, MDB_LAST);
    if !(rc == 0 as libc::c_int) {
        tmp___26 = mdb_strerror(rc);
        fprintf(
            stderr,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest.c\0" as *const u8 as *const libc::c_char,
            125 as libc::c_int,
            b"mdb_cursor_get(cursor, &key, &data, MDB_LAST)\0" as *const u8
                as *const libc::c_char,
            tmp___26,
        );
        abort();
    }
    printf(
        b"key: %.*s, data: %.*s\n\0" as *const u8 as *const libc::c_char,
        key.mv_size as libc::c_int,
        key.mv_data as *mut libc::c_char,
        data.mv_size as libc::c_int,
        data.mv_data as *mut libc::c_char,
    );
    rc = mdb_cursor_get(cursor, &mut key, &mut data, MDB_PREV);
    if !(rc == 0 as libc::c_int) {
        tmp___27 = mdb_strerror(rc);
        fprintf(
            stderr,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest.c\0" as *const u8 as *const libc::c_char,
            129 as libc::c_int,
            b"mdb_cursor_get(cursor, &key, &data, MDB_PREV)\0" as *const u8
                as *const libc::c_char,
            tmp___27,
        );
        abort();
    }
    printf(
        b"key: %.*s, data: %.*s\n\0" as *const u8 as *const libc::c_char,
        key.mv_size as libc::c_int,
        key.mv_data as *mut libc::c_char,
        data.mv_size as libc::c_int,
        data.mv_data as *mut libc::c_char,
    );
    mdb_cursor_close(cursor);
    mdb_txn_abort(txn);
    printf(b"Deleting with cursor\n\0" as *const u8 as *const libc::c_char);
    rc = mdb_txn_begin(
        env,
        0 as *mut libc::c_void as *mut MDB_txn,
        0 as libc::c_uint,
        &mut txn,
    );
    if !(rc == 0 as libc::c_int) {
        tmp___28 = mdb_strerror(rc);
        fprintf(
            stderr,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest.c\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int,
            b"mdb_txn_begin(env, NULL, 0, &txn)\0" as *const u8 as *const libc::c_char,
            tmp___28,
        );
        abort();
    }
    rc = mdb_cursor_open(txn, dbi, &mut cur2);
    if !(rc == 0 as libc::c_int) {
        tmp___29 = mdb_strerror(rc);
        fprintf(
            stderr,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int,
            b"mdb_cursor_open(txn, dbi, &cur2)\0" as *const u8 as *const libc::c_char,
            tmp___29,
        );
        abort();
    }
    i = 0 as libc::c_int;
    while i < 50 as libc::c_int {
        rc = mdb_cursor_get(cur2, &mut key, &mut data, MDB_NEXT);
        if rc == -(30798 as libc::c_int) {
            break;
        }
        if rc != 0 {
            tmp___30 = mdb_strerror(rc);
            fprintf(
                stderr,
                b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
                b"mtest.c\0" as *const u8 as *const libc::c_char,
                141 as libc::c_int,
                b"mdb_cursor_get(cur2, &key, &data, MDB_NEXT)\0" as *const u8
                    as *const libc::c_char,
                tmp___30,
            );
            abort();
        }
        printf(
            b"key: %p %.*s, data: %p %.*s\n\0" as *const u8 as *const libc::c_char,
            key.mv_data,
            key.mv_size as libc::c_int,
            key.mv_data as *mut libc::c_char,
            data.mv_data,
            data.mv_size as libc::c_int,
            data.mv_data as *mut libc::c_char,
        );
        rc = mdb_del(txn, dbi, &mut key, 0 as *mut libc::c_void as *mut MDB_val);
        if !(rc == 0 as libc::c_int) {
            tmp___31 = mdb_strerror(rc);
            fprintf(
                stderr,
                b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
                b"mtest.c\0" as *const u8 as *const libc::c_char,
                146 as libc::c_int,
                b"mdb_del(txn, dbi, &key, NULL)\0" as *const u8 as *const libc::c_char,
                tmp___31,
            );
            abort();
        }
        i += 1;
    }
    printf(b"Restarting cursor in txn\n\0" as *const u8 as *const libc::c_char);
    op = MDB_FIRST;
    i = 0 as libc::c_int;
    while i <= 32 as libc::c_int {
        rc = mdb_cursor_get(cur2, &mut key, &mut data, op);
        if rc == -(30798 as libc::c_int) {
            break;
        }
        if rc != 0 {
            tmp___32 = mdb_strerror(rc);
            fprintf(
                stderr,
                b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
                b"mtest.c\0" as *const u8 as *const libc::c_char,
                151 as libc::c_int,
                b"mdb_cursor_get(cur2, &key, &data, op)\0" as *const u8
                    as *const libc::c_char,
                tmp___32,
            );
            abort();
        }
        printf(
            b"key: %p %.*s, data: %p %.*s\n\0" as *const u8 as *const libc::c_char,
            key.mv_data,
            key.mv_size as libc::c_int,
            key.mv_data as *mut libc::c_char,
            data.mv_data,
            data.mv_size as libc::c_int,
            data.mv_data as *mut libc::c_char,
        );
        op = MDB_NEXT;
        i += 1;
    }
    mdb_cursor_close(cur2);
    rc = mdb_txn_commit(txn);
    if !(rc == 0 as libc::c_int) {
        tmp___33 = mdb_strerror(rc);
        fprintf(
            stderr,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest.c\0" as *const u8 as *const libc::c_char,
            158 as libc::c_int,
            b"mdb_txn_commit(txn)\0" as *const u8 as *const libc::c_char,
            tmp___33,
        );
        abort();
    }
    printf(b"Restarting cursor outside txn\n\0" as *const u8 as *const libc::c_char);
    rc = mdb_txn_begin(
        env,
        0 as *mut libc::c_void as *mut MDB_txn,
        0 as libc::c_uint,
        &mut txn,
    );
    if !(rc == 0 as libc::c_int) {
        tmp___34 = mdb_strerror(rc);
        fprintf(
            stderr,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int,
            b"mdb_txn_begin(env, NULL, 0, &txn)\0" as *const u8 as *const libc::c_char,
            tmp___34,
        );
        abort();
    }
    rc = mdb_cursor_open(txn, dbi, &mut cursor);
    if !(rc == 0 as libc::c_int) {
        tmp___35 = mdb_strerror(rc);
        fprintf(
            stderr,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest.c\0" as *const u8 as *const libc::c_char,
            162 as libc::c_int,
            b"mdb_cursor_open(txn, dbi, &cursor)\0" as *const u8 as *const libc::c_char,
            tmp___35,
        );
        abort();
    }
    op = MDB_FIRST;
    i = 0 as libc::c_int;
    while i <= 32 as libc::c_int {
        rc = mdb_cursor_get(cursor, &mut key, &mut data, op);
        if rc == -(30798 as libc::c_int) {
            break;
        }
        if rc != 0 {
            tmp___36 = mdb_strerror(rc);
            fprintf(
                stderr,
                b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
                b"mtest.c\0" as *const u8 as *const libc::c_char,
                164 as libc::c_int,
                b"mdb_cursor_get(cursor, &key, &data, op)\0" as *const u8
                    as *const libc::c_char,
                tmp___36,
            );
            abort();
        }
        printf(
            b"key: %p %.*s, data: %p %.*s\n\0" as *const u8 as *const libc::c_char,
            key.mv_data,
            key.mv_size as libc::c_int,
            key.mv_data as *mut libc::c_char,
            data.mv_data,
            data.mv_size as libc::c_int,
            data.mv_data as *mut libc::c_char,
        );
        op = MDB_NEXT;
        i += 1;
    }
    mdb_cursor_close(cursor);
    mdb_txn_abort(txn);
    mdb_dbi_close(env, dbi);
    mdb_env_close(env);
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
