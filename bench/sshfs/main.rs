use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type fuse_pollhandle;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _GHashTable;
    pub type fuse_session;
    pub type fuse;
    fn fuse_opt_parse(
        args: *mut fuse_args,
        data: *mut libc::c_void,
        opts: *const fuse_opt,
        proc_0: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *const libc::c_char,
                libc::c_int,
                *mut fuse_args,
            ) -> libc::c_int,
        >,
    ) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn g_ptr_array_new() -> *mut GPtrArray;
    fn g_ptr_array_free(array: *mut GPtrArray, free_seg: gboolean) -> *mut gpointer;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_hash_table_new_full(
        hash_func: Option::<unsafe extern "C" fn(gconstpointer) -> guint>,
        key_equal_func: Option::<
            unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
        >,
        key_destroy_func: Option::<unsafe extern "C" fn(gpointer) -> ()>,
        value_destroy_func: Option::<unsafe extern "C" fn(gpointer) -> ()>,
    ) -> *mut GHashTable;
    fn g_hash_table_insert(
        hash_table: *mut GHashTable,
        key: gpointer,
        value: gpointer,
    ) -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_foreach_remove(
        hash_table: *mut GHashTable,
        func: Option::<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gboolean>,
        user_data: gpointer,
    ) -> guint;
    fn g_hash_table_size(hash_table: *mut GHashTable) -> guint;
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, _: ...) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn fuse_opt_add_arg(args: *mut fuse_args, arg: *const libc::c_char) -> libc::c_int;
    fn fuse_opt_insert_arg(
        args: *mut fuse_args,
        pos_0: libc::c_int,
        arg: *const libc::c_char,
    ) -> libc::c_int;
    fn fuse_opt_free_args(args: *mut fuse_args);
    fn fuse_daemonize(foreground: libc::c_int) -> libc::c_int;
    fn fuse_pkgversion() -> *const libc::c_char;
    fn fuse_set_signal_handlers(se: *mut fuse_session) -> libc::c_int;
    fn fuse_remove_signal_handlers(se: *mut fuse_session);
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn writev(__fd: libc::c_int, __iovec: *const iovec, __count: libc::c_int) -> ssize_t;
    fn fuse_lib_help(args: *mut fuse_args);
    fn fuse_new(
        args: *mut fuse_args,
        op: *const fuse_operations,
        op_size: size_t,
        private_data: *mut libc::c_void,
    ) -> *mut fuse;
    fn fuse_mount(f: *mut fuse, mountpoint: *const libc::c_char) -> libc::c_int;
    fn fuse_unmount(f: *mut fuse);
    fn fuse_destroy(f: *mut fuse);
    fn fuse_loop(f: *mut fuse) -> libc::c_int;
    fn fuse_loop_mt_31(f: *mut fuse, clone_fd: libc::c_int) -> libc::c_int;
    fn fuse_get_session(f: *mut fuse) -> *mut fuse_session;
    fn fuse_lowlevel_version();
    fn fuse_session_fd(se: *mut fuse_session) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn rand_r(__seed: *mut libc::c_uint) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn unsetenv(__name: *const libc::c_char) -> libc::c_int;
    fn realpath(
        __name: *const libc::c_char,
        __resolved: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn grantpt(__fd: libc::c_int) -> libc::c_int;
    fn unlockpt(__fd: libc::c_int) -> libc::c_int;
    fn ptsname(__fd: libc::c_int) -> *mut libc::c_char;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn getpid() -> __pid_t;
    fn setsid() -> __pid_t;
    fn getuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn fork() -> __pid_t;
    fn getpagesize() -> libc::c_int;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strsep(
        __stringp: *mut *mut libc::c_char,
        __delim: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn sem_init(
        __sem: *mut sem_t,
        __pshared: libc::c_int,
        __value: libc::c_uint,
    ) -> libc::c_int;
    fn sem_destroy(__sem: *mut sem_t) -> libc::c_int;
    fn sem_wait(__sem: *mut sem_t) -> libc::c_int;
    fn sem_post(__sem: *mut sem_t) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_detach(__th: pthread_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn socketpair(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
        __fds: *mut libc::c_int,
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
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn signal(
        __sig: libc::c_int,
        __handler: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> __sighandler_t;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn pthread_sigmask(
        __how: libc::c_int,
        __newmask: *const __sigset_t,
        __oldmask: *mut __sigset_t,
    ) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off64_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn mlock(__addr: *const libc::c_void, __len: size_t) -> libc::c_int;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_list_append(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_delete_link(list: *mut GList, link_: *mut GList) -> *mut GList;
    fn g_list_first(list: *mut GList) -> *mut GList;
    fn g_hash_table_new(
        hash_func: Option::<unsafe extern "C" fn(gconstpointer) -> guint>,
        key_equal_func: Option::<
            unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
        >,
    ) -> *mut GHashTable;
    fn g_hash_table_replace(
        hash_table: *mut GHashTable,
        key: gpointer,
        value: gpointer,
    ) -> gboolean;
    fn g_hash_table_lookup_extended(
        hash_table: *mut GHashTable,
        lookup_key: gconstpointer,
        orig_key: *mut gpointer,
        value: *mut gpointer,
    ) -> gboolean;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn getgrnam(__name: *const libc::c_char) -> *mut group;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_opt {
    pub templ: *const libc::c_char,
    pub offset: libc::c_ulong,
    pub value: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_args {
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub allocated: libc::c_int,
}
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __fsblkcnt64_t = libc::c_ulong;
pub type __fsfilcnt64_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type off_t = __off64_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct fuse_file_info {
    pub flags: libc::c_int,
    #[bitfield(name = "writepage", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "direct_io", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "keep_cache", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "flush", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "nonseekable", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "flock_release", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "cache_readdir", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "padding", ty = "libc::c_uint", bits = "7..=31")]
    #[bitfield(name = "padding2", ty = "libc::c_uint", bits = "32..=63")]
    pub writepage_direct_io_keep_cache_flush_nonseekable_flock_release_cache_readdir_padding_padding2: [u8; 8],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
    pub fh: uint64_t,
    pub lock_owner: uint64_t,
    pub poll_events: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_conn_info {
    pub proto_major: libc::c_uint,
    pub proto_minor: libc::c_uint,
    pub max_write: libc::c_uint,
    pub max_read: libc::c_uint,
    pub max_readahead: libc::c_uint,
    pub capable: libc::c_uint,
    pub want: libc::c_uint,
    pub max_background: libc::c_uint,
    pub congestion_threshold: libc::c_uint,
    pub time_gran: libc::c_uint,
    pub reserved: [libc::c_uint; 22],
}
pub type fuse_buf_flags = libc::c_uint;
pub const FUSE_BUF_FD_RETRY: fuse_buf_flags = 8;
pub const FUSE_BUF_FD_SEEK: fuse_buf_flags = 4;
pub const FUSE_BUF_IS_FD: fuse_buf_flags = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_buf {
    pub size: size_t,
    pub flags: fuse_buf_flags,
    pub mem: *mut libc::c_void,
    pub fd: libc::c_int,
    pub pos: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_bufvec {
    pub count: size_t,
    pub idx: size_t,
    pub off: size_t,
    pub buf: [fuse_buf; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flock {
    pub l_type: libc::c_short,
    pub l_whence: libc::c_short,
    pub l_start: __off64_t,
    pub l_len: __off64_t,
    pub l_pid: __pid_t,
}
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
pub struct statvfs {
    pub f_bsize: libc::c_ulong,
    pub f_frsize: libc::c_ulong,
    pub f_blocks: __fsblkcnt64_t,
    pub f_bfree: __fsblkcnt64_t,
    pub f_bavail: __fsblkcnt64_t,
    pub f_files: __fsfilcnt64_t,
    pub f_ffree: __fsfilcnt64_t,
    pub f_favail: __fsfilcnt64_t,
    pub f_fsid: libc::c_ulong,
    pub f_flag: libc::c_ulong,
    pub f_namemax: libc::c_ulong,
    pub __f_spare: [libc::c_int; 6],
}
pub type fuse_readdir_flags = libc::c_uint;
pub const FUSE_READDIR_PLUS: fuse_readdir_flags = 1;
pub type fuse_fill_dir_flags = libc::c_uint;
pub const FUSE_FILL_DIR_PLUS: fuse_fill_dir_flags = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_config {
    pub set_gid: libc::c_int,
    pub gid: libc::c_uint,
    pub set_uid: libc::c_int,
    pub uid: libc::c_uint,
    pub set_mode: libc::c_int,
    pub umask: libc::c_uint,
    pub entry_timeout: libc::c_double,
    pub negative_timeout: libc::c_double,
    pub attr_timeout: libc::c_double,
    pub intr: libc::c_int,
    pub intr_signal: libc::c_int,
    pub remember: libc::c_int,
    pub hard_remove: libc::c_int,
    pub use_ino: libc::c_int,
    pub readdir_ino: libc::c_int,
    pub direct_io: libc::c_int,
    pub kernel_cache: libc::c_int,
    pub auto_cache: libc::c_int,
    pub ac_attr_timeout_set: libc::c_int,
    pub ac_attr_timeout: libc::c_double,
    pub nullpath_ok: libc::c_int,
    pub show_help: libc::c_int,
    pub modules: *mut libc::c_char,
    pub debug: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_operations {
    pub getattr: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut stat,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub readlink: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut libc::c_char,
            size_t,
        ) -> libc::c_int,
    >,
    pub mknod: Option::<
        unsafe extern "C" fn(*const libc::c_char, mode_t, dev_t) -> libc::c_int,
    >,
    pub mkdir: Option::<
        unsafe extern "C" fn(*const libc::c_char, mode_t) -> libc::c_int,
    >,
    pub unlink: Option::<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int>,
    pub rmdir: Option::<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int>,
    pub symlink: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    >,
    pub rename: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
            libc::c_uint,
        ) -> libc::c_int,
    >,
    pub link: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    >,
    pub chmod: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            mode_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub chown: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            uid_t,
            gid_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub truncate: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            off_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub open: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut fuse_file_info) -> libc::c_int,
    >,
    pub read: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut libc::c_char,
            size_t,
            off_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub write: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
            size_t,
            off_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub statfs: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut statvfs) -> libc::c_int,
    >,
    pub flush: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut fuse_file_info) -> libc::c_int,
    >,
    pub release: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut fuse_file_info) -> libc::c_int,
    >,
    pub fsync: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            libc::c_int,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub setxattr: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
            *const libc::c_char,
            size_t,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub getxattr: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
            *mut libc::c_char,
            size_t,
        ) -> libc::c_int,
    >,
    pub listxattr: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut libc::c_char,
            size_t,
        ) -> libc::c_int,
    >,
    pub removexattr: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    >,
    pub opendir: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut fuse_file_info) -> libc::c_int,
    >,
    pub readdir: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    *const stat,
                    off_t,
                    fuse_fill_dir_flags,
                ) -> libc::c_int,
            >,
            off_t,
            *mut fuse_file_info,
            fuse_readdir_flags,
        ) -> libc::c_int,
    >,
    pub releasedir: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut fuse_file_info) -> libc::c_int,
    >,
    pub fsyncdir: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            libc::c_int,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub init: Option::<
        unsafe extern "C" fn(*mut fuse_conn_info, *mut fuse_config) -> *mut libc::c_void,
    >,
    pub destroy: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub access: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub create: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            mode_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub lock: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut fuse_file_info,
            libc::c_int,
            *mut flock,
        ) -> libc::c_int,
    >,
    pub utimens: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const timespec,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub bmap: Option::<
        unsafe extern "C" fn(*const libc::c_char, size_t, *mut uint64_t) -> libc::c_int,
    >,
    pub ioctl: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            libc::c_uint,
            *mut libc::c_void,
            *mut fuse_file_info,
            libc::c_uint,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub poll: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut fuse_file_info,
            *mut fuse_pollhandle,
            *mut libc::c_uint,
        ) -> libc::c_int,
    >,
    pub write_buf: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut fuse_bufvec,
            off_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub read_buf: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut *mut fuse_bufvec,
            size_t,
            off_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub flock: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut fuse_file_info,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub fallocate: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            libc::c_int,
            off_t,
            off_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub copy_file_range: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut fuse_file_info,
            off_t,
            *const libc::c_char,
            *mut fuse_file_info,
            off_t,
            size_t,
            libc::c_int,
        ) -> ssize_t,
    >,
    pub lseek: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            off_t,
            libc::c_int,
            *mut fuse_file_info,
        ) -> off_t,
    >,
}
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
pub type gsize = libc::c_ulong;
pub type gchar = libc::c_char;
pub type gint = libc::c_int;
pub type gboolean = gint;
pub type guint = libc::c_uint;
pub type gpointer = *mut libc::c_void;
pub type gconstpointer = *const libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
pub type GPtrArray = _GPtrArray;
pub type GHashTable = _GHashTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cache {
    pub on: libc::c_int,
    pub stat_timeout_secs: libc::c_uint,
    pub dir_timeout_secs: libc::c_uint,
    pub link_timeout_secs: libc::c_uint,
    pub max_size: libc::c_uint,
    pub clean_interval_secs: libc::c_uint,
    pub min_clean_interval_secs: libc::c_uint,
    pub next_oper: *mut fuse_operations,
    pub table: *mut GHashTable,
    pub lock: pthread_mutex_t,
    pub last_cleaned: time_t,
    pub write_ctr: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node {
    pub stat: stat,
    pub stat_valid: time_t,
    pub dir: *mut *mut libc::c_char,
    pub dir_valid: time_t,
    pub link: *mut libc::c_char,
    pub link_valid: time_t,
    pub valid: time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct readdir_handle {
    pub path: *const libc::c_char,
    pub buf: *mut libc::c_void,
    pub filler: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            *const stat,
            off_t,
            fuse_fill_dir_flags,
        ) -> libc::c_int,
    >,
    pub dir: *mut GPtrArray,
    pub wrctr: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_handle {
    pub is_open: libc::c_int,
    pub fs_fh: libc::c_ulong,
}
pub type __uint8_t = libc::c_uchar;
pub type __suseconds_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type uint8_t = __uint8_t;
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
pub struct __anonstruct___wseq32_112954846 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_505876810 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: __anonstruct___wseq32_112954846,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___g1_start32_897626227 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_897626226 {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: __anonstruct___g1_start32_897626227,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __annonCompField1: __anonunion____missing_field_name_505876810,
    pub __annonCompField2: __anonunion____missing_field_name_897626226,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_condattr_t_488594145 {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
pub type pthread_condattr_t = __anonunion_pthread_condattr_t_488594145;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
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
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
pub type socklen_t = __socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_sem_t_991265790 {
    pub __size: [libc::c_char; 32],
    pub __align: libc::c_long,
}
pub type sem_t = __anonunion_sem_t_991265790;
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
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub type gulong = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
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
pub struct conn {
    pub lock_write: pthread_mutex_t,
    pub processing_thread_started: libc::c_int,
    pub rfd: libc::c_int,
    pub wfd: libc::c_int,
    pub connver: libc::c_int,
    pub req_count: libc::c_int,
    pub dir_count: libc::c_int,
    pub file_count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub p: *mut uint8_t,
    pub len: size_t,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dir_handle {
    pub buf: buffer,
    pub conn: *mut conn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_head {
    pub prev: *mut list_head,
    pub next: *mut list_head,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct request {
    pub want_reply: libc::c_uint,
    pub ready: sem_t,
    pub reply_type: uint8_t,
    pub id: uint32_t,
    pub replied: libc::c_int,
    pub error: libc::c_int,
    pub reply: buffer,
    pub start: timeval,
    pub data: *mut libc::c_void,
    pub end_func: Option::<unsafe extern "C" fn(*mut request) -> ()>,
    pub len: size_t,
    pub list: list_head,
    pub conn: *mut conn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sshfs_io {
    pub num_reqs: libc::c_int,
    pub finished: pthread_cond_t,
    pub error: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct read_req {
    pub sio: *mut sshfs_io,
    pub list: list_head,
    pub data: buffer,
    pub size: size_t,
    pub res: ssize_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct read_chunk {
    pub offset: off_t,
    pub size: size_t,
    pub refs: libc::c_int,
    pub modifver: libc::c_long,
    pub reqs: list_head,
    pub sio: sshfs_io,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sshfs_file {
    pub handle: buffer,
    pub write_reqs: list_head,
    pub write_finished: pthread_cond_t,
    pub write_error: libc::c_int,
    pub readahead: *mut read_chunk,
    pub next_pos: off_t,
    pub is_seq: libc::c_int,
    pub conn: *mut conn,
    pub connver: libc::c_int,
    pub modifver: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conntab_entry {
    pub refcount: libc::c_uint,
    pub conn: *mut conn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sshfs {
    pub directport: *mut libc::c_char,
    pub ssh_command: *mut libc::c_char,
    pub sftp_server: *mut libc::c_char,
    pub ssh_args: fuse_args,
    pub workarounds: *mut libc::c_char,
    pub rename_workaround: libc::c_int,
    pub renamexdev_workaround: libc::c_int,
    pub truncate_workaround: libc::c_int,
    pub buflimit_workaround: libc::c_int,
    pub unrel_append: libc::c_int,
    pub fstat_workaround: libc::c_int,
    pub createmode_workaround: libc::c_int,
    pub transform_symlinks: libc::c_int,
    pub follow_symlinks: libc::c_int,
    pub no_check_root: libc::c_int,
    pub detect_uid: libc::c_int,
    pub idmap: libc::c_int,
    pub nomap: libc::c_int,
    pub disable_hardlink: libc::c_int,
    pub dir_cache: libc::c_int,
    pub show_version: libc::c_int,
    pub show_help: libc::c_int,
    pub singlethread: libc::c_int,
    pub mountpoint: *mut libc::c_char,
    pub uid_file: *mut libc::c_char,
    pub gid_file: *mut libc::c_char,
    pub uid_map: *mut GHashTable,
    pub gid_map: *mut GHashTable,
    pub r_uid_map: *mut GHashTable,
    pub r_gid_map: *mut GHashTable,
    pub max_read: libc::c_uint,
    pub max_write: libc::c_uint,
    pub ssh_ver: libc::c_uint,
    pub sync_write: libc::c_int,
    pub sync_read: libc::c_int,
    pub sync_readdir: libc::c_int,
    pub direct_io: libc::c_int,
    pub debug: libc::c_int,
    pub verbose: libc::c_int,
    pub foreground: libc::c_int,
    pub reconnect: libc::c_int,
    pub delay_connect: libc::c_int,
    pub passive: libc::c_int,
    pub host: *mut libc::c_char,
    pub base_path: *mut libc::c_char,
    pub reqtab: *mut GHashTable,
    pub conntab: *mut GHashTable,
    pub lock: pthread_mutex_t,
    pub randseed: libc::c_uint,
    pub max_conns: libc::c_int,
    pub conns: *mut conn,
    pub ptyfd: libc::c_int,
    pub ptypassivefd: libc::c_int,
    pub connvers: libc::c_int,
    pub server_version: libc::c_int,
    pub remote_uid: libc::c_uint,
    pub local_uid: libc::c_uint,
    pub remote_gid: libc::c_uint,
    pub local_gid: libc::c_uint,
    pub remote_uid_detected: libc::c_int,
    pub blksize: libc::c_uint,
    pub progname: *mut libc::c_char,
    pub modifver: libc::c_long,
    pub outstanding_len: libc::c_uint,
    pub max_outstanding_len: libc::c_uint,
    pub outstanding_cond: pthread_cond_t,
    pub password_stdin: libc::c_int,
    pub password: *mut libc::c_char,
    pub ext_posix_rename: libc::c_int,
    pub ext_statvfs: libc::c_int,
    pub ext_hardlink: libc::c_int,
    pub ext_fsync: libc::c_int,
    pub op: *mut fuse_operations,
    pub bytes_sent: uint64_t,
    pub bytes_received: uint64_t,
    pub num_sent: uint64_t,
    pub num_received: uint64_t,
    pub min_rtt: libc::c_uint,
    pub max_rtt: libc::c_uint,
    pub total_rtt: uint64_t,
    pub num_connect: libc::c_uint,
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = __fxstat(1 as libc::c_int, __fd, __statbuf);
    return tmp;
}
static mut cache: cache = cache {
    on: 0,
    stat_timeout_secs: 0,
    dir_timeout_secs: 0,
    link_timeout_secs: 0,
    max_size: 0,
    clean_interval_secs: 0,
    min_clean_interval_secs: 0,
    next_oper: 0 as *const fuse_operations as *mut fuse_operations,
    table: 0 as *const GHashTable as *mut GHashTable,
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
                __prev: 0 as *const __pthread_internal_list
                    as *mut __pthread_internal_list,
                __next: 0 as *const __pthread_internal_list
                    as *mut __pthread_internal_list,
            },
        },
    },
    last_cleaned: 0,
    write_ctr: 0,
};
unsafe extern "C" fn free_node(mut node_: gpointer) {
    let mut node: *mut node = 0 as *mut node;
    node = node_ as *mut node;
    g_strfreev((*node).dir);
    g_free(node as gpointer);
}
unsafe extern "C" fn cache_clean_entry(
    mut key_: *mut libc::c_void,
    mut node: *mut node,
    mut now: *mut time_t,
) -> libc::c_int {
    if *now > (*node).valid { return 1 as libc::c_int } else { return 0 as libc::c_int };
}
unsafe extern "C" fn cache_clean() {
    let mut now: time_t = 0;
    let mut tmp: time_t = 0;
    let mut tmp___0: guint = 0;
    tmp = time(0 as *mut libc::c_void as *mut time_t);
    now = tmp;
    if now > cache.last_cleaned + cache.min_clean_interval_secs as time_t {
        tmp___0 = g_hash_table_size(cache.table);
        if tmp___0 > cache.max_size {
            g_hash_table_foreach_remove(
                cache.table,
                ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut node,
                            *mut time_t,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gboolean,
                    >,
                >(
                    Some(
                        cache_clean_entry
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut node,
                                *mut time_t,
                            ) -> libc::c_int,
                    ),
                ),
                &mut now as *mut time_t as gpointer,
            );
            cache.last_cleaned = now;
        } else if now > cache.last_cleaned + cache.clean_interval_secs as time_t {
            g_hash_table_foreach_remove(
                cache.table,
                ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut node,
                            *mut time_t,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gboolean,
                    >,
                >(
                    Some(
                        cache_clean_entry
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut node,
                                *mut time_t,
                            ) -> libc::c_int,
                    ),
                ),
                &mut now as *mut time_t as gpointer,
            );
            cache.last_cleaned = now;
        }
    }
}
unsafe extern "C" fn cache_lookup(mut path: *const libc::c_char) -> *mut node {
    let mut tmp: gpointer = 0 as *mut libc::c_void;
    tmp = g_hash_table_lookup(cache.table, path as gconstpointer);
    return tmp as *mut node;
}
unsafe extern "C" fn cache_purge(mut path: *const libc::c_char) {
    g_hash_table_remove(cache.table, path as gconstpointer);
}
unsafe extern "C" fn cache_purge_parent(mut path: *const libc::c_char) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut parent: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut gchar = 0 as *mut gchar;
    tmp = strrchr(path, '/' as i32);
    s = tmp as *const libc::c_char;
    if !s.is_null() {
        if s as libc::c_ulong == path as libc::c_ulong {
            g_hash_table_remove(
                cache.table,
                b"/\0" as *const u8 as *const libc::c_char as gconstpointer,
            );
        } else {
            tmp___0 = g_strndup(path, s.offset_from(path) as libc::c_long as gsize);
            parent = tmp___0;
            cache_purge(parent as *const libc::c_char);
            g_free(parent as gpointer);
        }
    }
}
pub unsafe extern "C" fn cache_invalidate(mut path: *const libc::c_char) {
    pthread_mutex_lock(&mut cache.lock);
    cache_purge(path);
    pthread_mutex_unlock(&mut cache.lock);
}
unsafe extern "C" fn cache_invalidate_write(mut path: *const libc::c_char) {
    pthread_mutex_lock(&mut cache.lock);
    cache_purge(path);
    cache.write_ctr = (cache.write_ctr).wrapping_add(1);
    pthread_mutex_unlock(&mut cache.lock);
}
unsafe extern "C" fn cache_invalidate_dir(mut path: *const libc::c_char) {
    pthread_mutex_lock(&mut cache.lock);
    cache_purge(path);
    cache_purge_parent(path);
    pthread_mutex_unlock(&mut cache.lock);
}
unsafe extern "C" fn cache_del_children(
    mut key: *const libc::c_char,
    mut val_: *mut libc::c_void,
    mut path: *const libc::c_char,
) -> libc::c_int {
    let mut tmp: size_t = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = strlen(path);
    tmp___0 = strncmp(key, path, tmp);
    if tmp___0 == 0 as libc::c_int {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn cache_do_rename(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
) {
    pthread_mutex_lock(&mut cache.lock);
    g_hash_table_foreach_remove(
        cache.table,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_char,
                    *mut libc::c_void,
                    *const libc::c_char,
                ) -> libc::c_int,
            >,
            Option::<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gboolean>,
        >(
            Some(
                cache_del_children
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut libc::c_void,
                        *const libc::c_char,
                    ) -> libc::c_int,
            ),
        ),
        from as *mut libc::c_char as gpointer,
    );
    cache_purge(from);
    cache_purge(to);
    cache_purge_parent(from);
    cache_purge_parent(to);
    pthread_mutex_unlock(&mut cache.lock);
}
unsafe extern "C" fn cache_get(mut path: *const libc::c_char) -> *mut node {
    let mut node: *mut node = 0 as *mut node;
    let mut tmp: *mut node = 0 as *mut node;
    let mut pathcopy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut gchar = 0 as *mut gchar;
    let mut __n: gsize = 0;
    let mut __s: gsize = 0;
    let mut __p: gpointer = 0 as *mut libc::c_void;
    tmp = cache_lookup(path);
    node = tmp;
    if node as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___0 = g_strdup(path);
        pathcopy = tmp___0;
        __n = 1 as libc::c_int as gsize;
        __s = ::std::mem::size_of::<node>() as libc::c_ulong;
        if __s == 1 as libc::c_ulong {
            __p = g_malloc0(__n);
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        node = __p as *mut node;
        g_hash_table_insert(cache.table, pathcopy as gpointer, node as gpointer);
    }
    return node;
}
pub unsafe extern "C" fn cache_add_attr(
    mut path: *const libc::c_char,
    mut stbuf: *const stat,
    mut wrctr: uint64_t,
) {
    let mut node: *mut node = 0 as *mut node;
    let mut tmp: time_t = 0;
    pthread_mutex_lock(&mut cache.lock);
    if wrctr == cache.write_ctr {
        node = cache_get(path);
        (*node).stat = *stbuf;
        tmp = time(0 as *mut libc::c_void as *mut time_t);
        (*node).stat_valid = tmp + cache.stat_timeout_secs as time_t;
        if (*node).stat_valid > (*node).valid {
            (*node).valid = (*node).stat_valid;
        }
        cache_clean();
    }
    pthread_mutex_unlock(&mut cache.lock);
}
unsafe extern "C" fn cache_add_dir(
    mut path: *const libc::c_char,
    mut dir: *mut *mut libc::c_char,
) {
    let mut node: *mut node = 0 as *mut node;
    let mut tmp: time_t = 0;
    pthread_mutex_lock(&mut cache.lock);
    node = cache_get(path);
    g_strfreev((*node).dir);
    (*node).dir = dir;
    tmp = time(0 as *mut libc::c_void as *mut time_t);
    (*node).dir_valid = tmp + cache.dir_timeout_secs as time_t;
    if (*node).dir_valid > (*node).valid {
        (*node).valid = (*node).dir_valid;
    }
    cache_clean();
    pthread_mutex_unlock(&mut cache.lock);
}
unsafe extern "C" fn my_strnlen(
    mut s: *const libc::c_char,
    mut maxsize: size_t,
) -> size_t {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = s;
    while maxsize != 0 {
        if *p == 0 {
            break;
        }
        maxsize = maxsize.wrapping_sub(1);
        p = p.offset(1);
    }
    return p.offset_from(s) as libc::c_long as size_t;
}
unsafe extern "C" fn cache_add_link(
    mut path: *const libc::c_char,
    mut link: *const libc::c_char,
    mut size: size_t,
) {
    let mut node: *mut node = 0 as *mut node;
    let mut tmp: size_t = 0;
    let mut tmp___0: time_t = 0;
    pthread_mutex_lock(&mut cache.lock);
    node = cache_get(path);
    g_free((*node).link as gpointer);
    tmp = my_strnlen(link, size.wrapping_sub(1 as libc::c_ulong));
    (*node).link = g_strndup(link, tmp);
    tmp___0 = time(0 as *mut libc::c_void as *mut time_t);
    (*node).link_valid = tmp___0 + cache.link_timeout_secs as time_t;
    if (*node).link_valid > (*node).valid {
        (*node).valid = (*node).link_valid;
    }
    cache_clean();
    pthread_mutex_unlock(&mut cache.lock);
}
unsafe extern "C" fn cache_get_attr(
    mut path: *const libc::c_char,
    mut stbuf: *mut stat,
) -> libc::c_int {
    let mut node: *mut node = 0 as *mut node;
    let mut err: libc::c_int = 0;
    let mut now: time_t = 0;
    let mut tmp: time_t = 0;
    err = -(11 as libc::c_int);
    pthread_mutex_lock(&mut cache.lock);
    node = cache_lookup(path);
    if node as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp = time(0 as *mut libc::c_void as *mut time_t);
        now = tmp;
        if (*node).stat_valid - now >= 0 as libc::c_long {
            *stbuf = (*node).stat;
            err = 0 as libc::c_int;
        }
    }
    pthread_mutex_unlock(&mut cache.lock);
    return err;
}
pub unsafe extern "C" fn cache_get_write_ctr() -> uint64_t {
    let mut res: uint64_t = 0;
    pthread_mutex_lock(&mut cache.lock);
    res = cache.write_ctr;
    pthread_mutex_unlock(&mut cache.lock);
    return res;
}
unsafe extern "C" fn cache_init(
    mut conn: *mut fuse_conn_info,
    mut cfg: *mut fuse_config,
) -> *mut libc::c_void {
    let mut res: *mut libc::c_void = 0 as *mut libc::c_void;
    res = (Some(((*cache.next_oper).init).expect("non-null function pointer")))
        .expect("non-null function pointer")(conn, cfg);
    (*cfg).nullpath_ok = 0 as libc::c_int;
    return res;
}
unsafe extern "C" fn cache_getattr(
    mut path: *const libc::c_char,
    mut stbuf: *mut stat,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut wrctr: uint64_t = 0;
    let mut tmp___0: uint64_t = 0;
    tmp = cache_get_attr(path, stbuf);
    err = tmp;
    if err != 0 {
        tmp___0 = cache_get_write_ctr();
        wrctr = tmp___0;
        err = (Some(((*cache.next_oper).getattr).expect("non-null function pointer")))
            .expect("non-null function pointer")(path, stbuf, fi);
        if err == 0 {
            cache_add_attr(path, stbuf as *const stat, wrctr);
        }
    }
    return err;
}
unsafe extern "C" fn cache_readlink(
    mut path: *const libc::c_char,
    mut buf: *mut libc::c_char,
    mut size: size_t,
) -> libc::c_int {
    let mut node: *mut node = 0 as *mut node;
    let mut err: libc::c_int = 0;
    let mut now: time_t = 0;
    let mut tmp: time_t = 0;
    pthread_mutex_lock(&mut cache.lock);
    node = cache_lookup(path);
    if node as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp = time(0 as *mut libc::c_void as *mut time_t);
        now = tmp;
        if (*node).link_valid - now >= 0 as libc::c_long {
            strncpy(
                buf,
                (*node).link as *const libc::c_char,
                size.wrapping_sub(1 as libc::c_ulong),
            );
            *buf
                .offset(
                    size.wrapping_sub(1 as libc::c_ulong) as isize,
                ) = '\u{0}' as i32 as libc::c_char;
            pthread_mutex_unlock(&mut cache.lock);
            return 0 as libc::c_int;
        }
    }
    pthread_mutex_unlock(&mut cache.lock);
    err = (Some(((*cache.next_oper).readlink).expect("non-null function pointer")))
        .expect("non-null function pointer")(path, buf, size);
    if err == 0 {
        cache_add_link(path, buf as *const libc::c_char, size);
    }
    return err;
}
unsafe extern "C" fn cache_opendir(
    mut path: *const libc::c_char,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut cfi: *mut file_handle = 0 as *mut file_handle;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<file_handle>() as libc::c_ulong);
    cfi = tmp as *mut file_handle;
    if cfi as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(12 as libc::c_int);
    }
    (*cfi).is_open = 0 as libc::c_int;
    (*fi).fh = cfi as libc::c_ulong;
    return 0 as libc::c_int;
}
unsafe extern "C" fn cache_releasedir(
    mut path: *const libc::c_char,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut cfi: *mut file_handle = 0 as *mut file_handle;
    cfi = (*fi).fh as *mut file_handle;
    if (*cfi).is_open != 0 {
        (*fi).fh = (*cfi).fs_fh;
        err = (Some(((*cache.next_oper).releasedir).expect("non-null function pointer")))
            .expect("non-null function pointer")(path, fi);
    } else {
        err = 0 as libc::c_int;
    }
    free(cfi as *mut libc::c_void);
    return err;
}
unsafe extern "C" fn cache_dirfill(
    mut buf: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut stbuf: *const stat,
    mut off: off_t,
    mut flags: fuse_fill_dir_flags,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut ch: *mut readdir_handle = 0 as *mut readdir_handle;
    let mut tmp: *mut gchar = 0 as *mut gchar;
    let mut fullpath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut basepath: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    ch = buf as *mut readdir_handle;
    err = (Some(((*ch).filler).expect("non-null function pointer")))
        .expect("non-null function pointer")((*ch).buf, name, stbuf, off, flags);
    if err == 0 {
        tmp = g_strdup(name);
        g_ptr_array_add((*ch).dir, tmp as gpointer);
        if (*stbuf).st_mode & 61440 as libc::c_uint != 0 {
            if *((*ch).path).offset(1 as libc::c_int as isize) == 0 {
                tmp___0 = b"\0" as *const u8 as *const libc::c_char;
            } else {
                tmp___0 = (*ch).path;
            }
            basepath = tmp___0;
            fullpath = g_strdup_printf(
                b"%s/%s\0" as *const u8 as *const libc::c_char,
                basepath,
                name,
            );
            cache_add_attr(fullpath as *const libc::c_char, stbuf, (*ch).wrctr);
            g_free(fullpath as gpointer);
        }
    }
    return err;
}
unsafe extern "C" fn cache_readdir(
    mut path: *const libc::c_char,
    mut buf: *mut libc::c_void,
    mut filler: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            *const stat,
            off_t,
            fuse_fill_dir_flags,
        ) -> libc::c_int,
    >,
    mut offset: off_t,
    mut fi: *mut fuse_file_info,
    mut flags: fuse_readdir_flags,
) -> libc::c_int {
    let mut ch: readdir_handle = readdir_handle {
        path: 0 as *const libc::c_char,
        buf: 0 as *mut libc::c_void,
        filler: None,
        dir: 0 as *mut GPtrArray,
        wrctr: 0,
    };
    let mut cfi: *mut file_handle = 0 as *mut file_handle;
    let mut err: libc::c_int = 0;
    let mut dir: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut node: *mut node = 0 as *mut node;
    let mut now: time_t = 0;
    let mut tmp___0: time_t = 0;
    if !(offset == 0 as libc::c_long) {
        __assert_fail(
            b"offset == 0\0" as *const u8 as *const libc::c_char,
            b"../cache.c\0" as *const u8 as *const libc::c_char,
            367 as libc::c_uint,
            b"cache_readdir\0" as *const u8 as *const libc::c_char,
        );
    }
    pthread_mutex_lock(&mut cache.lock);
    node = cache_lookup(path);
    if node as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        if (*node).dir as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            tmp___0 = time(0 as *mut libc::c_void as *mut time_t);
            now = tmp___0;
            if (*node).dir_valid - now >= 0 as libc::c_long {
                dir = (*node).dir;
                while *dir as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                    (Some(filler.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(
                        buf,
                        *dir as *const libc::c_char,
                        0 as *mut libc::c_void as *const stat,
                        0 as libc::c_int as off_t,
                        0 as fuse_fill_dir_flags,
                    );
                    dir = dir.offset(1);
                }
                pthread_mutex_unlock(&mut cache.lock);
                return 0 as libc::c_int;
            }
        }
    }
    pthread_mutex_unlock(&mut cache.lock);
    cfi = (*fi).fh as *mut file_handle;
    if (*cfi).is_open != 0 {
        (*fi).fh = (*cfi).fs_fh;
    } else {
        if ((*cache.next_oper).opendir).is_some() {
            err = (Some(
                ((*cache.next_oper).opendir).expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")(path, fi);
            if err != 0 {
                return err;
            }
        }
        (*cfi).is_open = 1 as libc::c_int;
        (*cfi).fs_fh = (*fi).fh;
    }
    ch.path = path;
    ch.buf = buf;
    ch.filler = filler;
    ch.dir = g_ptr_array_new();
    ch.wrctr = cache_get_write_ctr();
    err = (Some(((*cache.next_oper).readdir).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        path,
        &mut ch as *mut readdir_handle as *mut libc::c_void,
        Some(
            cache_dirfill
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    *const stat,
                    off_t,
                    fuse_fill_dir_flags,
                ) -> libc::c_int,
        ),
        offset,
        fi,
        flags,
    );
    g_ptr_array_add(ch.dir, 0 as *mut libc::c_void);
    dir = (*ch.dir).pdata as *mut *mut libc::c_char;
    if err == 0 {
        cache_add_dir(path, dir);
    } else {
        g_strfreev(dir);
    }
    g_ptr_array_free(ch.dir, 0 as libc::c_int);
    return err;
}
unsafe extern "C" fn cache_mknod(
    mut path: *const libc::c_char,
    mut mode: mode_t,
    mut rdev: dev_t,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tmp = (Some(((*cache.next_oper).mknod).expect("non-null function pointer")))
        .expect("non-null function pointer")(path, mode, rdev);
    err = tmp;
    if err == 0 {
        cache_invalidate_dir(path);
    }
    return err;
}
unsafe extern "C" fn cache_mkdir(
    mut path: *const libc::c_char,
    mut mode: mode_t,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tmp = (Some(((*cache.next_oper).mkdir).expect("non-null function pointer")))
        .expect("non-null function pointer")(path, mode);
    err = tmp;
    if err == 0 {
        cache_invalidate_dir(path);
    }
    return err;
}
unsafe extern "C" fn cache_unlink(mut path: *const libc::c_char) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tmp = (Some(((*cache.next_oper).unlink).expect("non-null function pointer")))
        .expect("non-null function pointer")(path);
    err = tmp;
    if err == 0 {
        cache_invalidate_dir(path);
    }
    return err;
}
unsafe extern "C" fn cache_rmdir(mut path: *const libc::c_char) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tmp = (Some(((*cache.next_oper).rmdir).expect("non-null function pointer")))
        .expect("non-null function pointer")(path);
    err = tmp;
    if err == 0 {
        cache_invalidate_dir(path);
    }
    return err;
}
unsafe extern "C" fn cache_symlink(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tmp = (Some(((*cache.next_oper).symlink).expect("non-null function pointer")))
        .expect("non-null function pointer")(from, to);
    err = tmp;
    if err == 0 {
        cache_invalidate_dir(to);
    }
    return err;
}
unsafe extern "C" fn cache_rename(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
    mut flags: libc::c_uint,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tmp = (Some(((*cache.next_oper).rename).expect("non-null function pointer")))
        .expect("non-null function pointer")(from, to, flags);
    err = tmp;
    if err == 0 {
        cache_do_rename(from, to);
    }
    return err;
}
unsafe extern "C" fn cache_link(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tmp = (Some(((*cache.next_oper).link).expect("non-null function pointer")))
        .expect("non-null function pointer")(from, to);
    err = tmp;
    if err == 0 {
        cache_invalidate(from);
        cache_invalidate_dir(to);
    }
    return err;
}
unsafe extern "C" fn cache_chmod(
    mut path: *const libc::c_char,
    mut mode: mode_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tmp = (Some(((*cache.next_oper).chmod).expect("non-null function pointer")))
        .expect("non-null function pointer")(path, mode, fi);
    err = tmp;
    if err == 0 {
        cache_invalidate(path);
    }
    return err;
}
unsafe extern "C" fn cache_chown(
    mut path: *const libc::c_char,
    mut uid: uid_t,
    mut gid: gid_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tmp = (Some(((*cache.next_oper).chown).expect("non-null function pointer")))
        .expect("non-null function pointer")(path, uid, gid, fi);
    err = tmp;
    if err == 0 {
        cache_invalidate(path);
    }
    return err;
}
unsafe extern "C" fn cache_utimens(
    mut path: *const libc::c_char,
    mut tv: *const timespec,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tmp = (Some(((*cache.next_oper).utimens).expect("non-null function pointer")))
        .expect("non-null function pointer")(path, tv, fi);
    err = tmp;
    if err == 0 {
        cache_invalidate(path);
    }
    return err;
}
unsafe extern "C" fn cache_write(
    mut path: *const libc::c_char,
    mut buf: *const libc::c_char,
    mut size: size_t,
    mut offset: off_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tmp = (Some(((*cache.next_oper).write).expect("non-null function pointer")))
        .expect("non-null function pointer")(path, buf, size, offset, fi);
    res = tmp;
    if res >= 0 as libc::c_int {
        cache_invalidate_write(path);
    }
    return res;
}
unsafe extern "C" fn cache_create(
    mut path: *const libc::c_char,
    mut mode: mode_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tmp = (Some(((*cache.next_oper).create).expect("non-null function pointer")))
        .expect("non-null function pointer")(path, mode, fi);
    err = tmp;
    if err == 0 {
        cache_invalidate_dir(path);
    }
    return err;
}
unsafe extern "C" fn cache_truncate(
    mut path: *const libc::c_char,
    mut size: off_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    tmp = (Some(((*cache.next_oper).truncate).expect("non-null function pointer")))
        .expect("non-null function pointer")(path, size, fi);
    err = tmp;
    if err == 0 {
        cache_invalidate(path);
    }
    return err;
}
unsafe extern "C" fn cache_fill(
    mut oper: *mut fuse_operations,
    mut cache_oper___0: *mut fuse_operations,
) {
    (*cache_oper___0).access = (*oper).access;
    if ((*oper).chmod).is_some() {
        (*cache_oper___0)
            .chmod = Some(
            cache_chmod
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    mode_t,
                    *mut fuse_file_info,
                ) -> libc::c_int,
        );
    } else {
        (*cache_oper___0)
            .chmod = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_char,
                    mode_t,
                    *mut fuse_file_info,
                ) -> libc::c_int,
            >,
        >(0 as *mut libc::c_void);
    }
    if ((*oper).chown).is_some() {
        (*cache_oper___0)
            .chown = Some(
            cache_chown
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    uid_t,
                    gid_t,
                    *mut fuse_file_info,
                ) -> libc::c_int,
        );
    } else {
        (*cache_oper___0)
            .chown = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_char,
                    uid_t,
                    gid_t,
                    *mut fuse_file_info,
                ) -> libc::c_int,
            >,
        >(0 as *mut libc::c_void);
    }
    if ((*oper).create).is_some() {
        (*cache_oper___0)
            .create = Some(
            cache_create
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    mode_t,
                    *mut fuse_file_info,
                ) -> libc::c_int,
        );
    } else {
        (*cache_oper___0)
            .create = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_char,
                    mode_t,
                    *mut fuse_file_info,
                ) -> libc::c_int,
            >,
        >(0 as *mut libc::c_void);
    }
    (*cache_oper___0).flush = (*oper).flush;
    (*cache_oper___0).fsync = (*oper).fsync;
    if ((*oper).getattr).is_some() {
        (*cache_oper___0)
            .getattr = Some(
            cache_getattr
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *mut stat,
                    *mut fuse_file_info,
                ) -> libc::c_int,
        );
    } else {
        (*cache_oper___0)
            .getattr = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_char,
                    *mut stat,
                    *mut fuse_file_info,
                ) -> libc::c_int,
            >,
        >(0 as *mut libc::c_void);
    }
    (*cache_oper___0).getxattr = (*oper).getxattr;
    (*cache_oper___0)
        .init = Some(
        cache_init
            as unsafe extern "C" fn(
                *mut fuse_conn_info,
                *mut fuse_config,
            ) -> *mut libc::c_void,
    );
    if ((*oper).link).is_some() {
        (*cache_oper___0)
            .link = Some(
            cache_link
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        );
    } else {
        (*cache_oper___0)
            .link = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
            >,
        >(0 as *mut libc::c_void);
    }
    (*cache_oper___0).listxattr = (*oper).listxattr;
    if ((*oper).mkdir).is_some() {
        (*cache_oper___0)
            .mkdir = Some(
            cache_mkdir
                as unsafe extern "C" fn(*const libc::c_char, mode_t) -> libc::c_int,
        );
    } else {
        (*cache_oper___0)
            .mkdir = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(*const libc::c_char, mode_t) -> libc::c_int>,
        >(0 as *mut libc::c_void);
    }
    if ((*oper).mknod).is_some() {
        (*cache_oper___0)
            .mknod = Some(
            cache_mknod
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    mode_t,
                    dev_t,
                ) -> libc::c_int,
        );
    } else {
        (*cache_oper___0)
            .mknod = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(*const libc::c_char, mode_t, dev_t) -> libc::c_int,
            >,
        >(0 as *mut libc::c_void);
    }
    (*cache_oper___0).open = (*oper).open;
    (*cache_oper___0)
        .opendir = Some(
        cache_opendir
            as unsafe extern "C" fn(
                *const libc::c_char,
                *mut fuse_file_info,
            ) -> libc::c_int,
    );
    (*cache_oper___0).read = (*oper).read;
    if ((*oper).readdir).is_some() {
        (*cache_oper___0)
            .readdir = Some(
            cache_readdir
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *mut libc::c_void,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const libc::c_char,
                            *const stat,
                            off_t,
                            fuse_fill_dir_flags,
                        ) -> libc::c_int,
                    >,
                    off_t,
                    *mut fuse_file_info,
                    fuse_readdir_flags,
                ) -> libc::c_int,
        );
    } else {
        (*cache_oper___0)
            .readdir = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_char,
                    *mut libc::c_void,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const libc::c_char,
                            *const stat,
                            off_t,
                            fuse_fill_dir_flags,
                        ) -> libc::c_int,
                    >,
                    off_t,
                    *mut fuse_file_info,
                    fuse_readdir_flags,
                ) -> libc::c_int,
            >,
        >(0 as *mut libc::c_void);
    }
    if ((*oper).readlink).is_some() {
        (*cache_oper___0)
            .readlink = Some(
            cache_readlink
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *mut libc::c_char,
                    size_t,
                ) -> libc::c_int,
        );
    } else {
        (*cache_oper___0)
            .readlink = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_char,
                    *mut libc::c_char,
                    size_t,
                ) -> libc::c_int,
            >,
        >(0 as *mut libc::c_void);
    }
    (*cache_oper___0).release = (*oper).release;
    (*cache_oper___0)
        .releasedir = Some(
        cache_releasedir
            as unsafe extern "C" fn(
                *const libc::c_char,
                *mut fuse_file_info,
            ) -> libc::c_int,
    );
    (*cache_oper___0).removexattr = (*oper).removexattr;
    if ((*oper).rename).is_some() {
        (*cache_oper___0)
            .rename = Some(
            cache_rename
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                    libc::c_uint,
                ) -> libc::c_int,
        );
    } else {
        (*cache_oper___0)
            .rename = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                    libc::c_uint,
                ) -> libc::c_int,
            >,
        >(0 as *mut libc::c_void);
    }
    if ((*oper).rmdir).is_some() {
        (*cache_oper___0)
            .rmdir = Some(
            cache_rmdir as unsafe extern "C" fn(*const libc::c_char) -> libc::c_int,
        );
    } else {
        (*cache_oper___0)
            .rmdir = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int>,
        >(0 as *mut libc::c_void);
    }
    (*cache_oper___0).setxattr = (*oper).setxattr;
    (*cache_oper___0).statfs = (*oper).statfs;
    if ((*oper).symlink).is_some() {
        (*cache_oper___0)
            .symlink = Some(
            cache_symlink
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        );
    } else {
        (*cache_oper___0)
            .symlink = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
            >,
        >(0 as *mut libc::c_void);
    }
    if ((*oper).truncate).is_some() {
        (*cache_oper___0)
            .truncate = Some(
            cache_truncate
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    off_t,
                    *mut fuse_file_info,
                ) -> libc::c_int,
        );
    } else {
        (*cache_oper___0)
            .truncate = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_char,
                    off_t,
                    *mut fuse_file_info,
                ) -> libc::c_int,
            >,
        >(0 as *mut libc::c_void);
    }
    if ((*oper).unlink).is_some() {
        (*cache_oper___0)
            .unlink = Some(
            cache_unlink as unsafe extern "C" fn(*const libc::c_char) -> libc::c_int,
        );
    } else {
        (*cache_oper___0)
            .unlink = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int>,
        >(0 as *mut libc::c_void);
    }
    if ((*oper).utimens).is_some() {
        (*cache_oper___0)
            .utimens = Some(
            cache_utimens
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const timespec,
                    *mut fuse_file_info,
                ) -> libc::c_int,
        );
    } else {
        (*cache_oper___0)
            .utimens = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_char,
                    *const timespec,
                    *mut fuse_file_info,
                ) -> libc::c_int,
            >,
        >(0 as *mut libc::c_void);
    }
    if ((*oper).write).is_some() {
        (*cache_oper___0)
            .write = Some(
            cache_write
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                    size_t,
                    off_t,
                    *mut fuse_file_info,
                ) -> libc::c_int,
        );
    } else {
        (*cache_oper___0)
            .write = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                    size_t,
                    off_t,
                    *mut fuse_file_info,
                ) -> libc::c_int,
            >,
        >(0 as *mut libc::c_void);
    };
}
static mut cache_oper: fuse_operations = fuse_operations {
    getattr: None,
    readlink: None,
    mknod: None,
    mkdir: None,
    unlink: None,
    rmdir: None,
    symlink: None,
    rename: None,
    link: None,
    chmod: None,
    chown: None,
    truncate: None,
    open: None,
    read: None,
    write: None,
    statfs: None,
    flush: None,
    release: None,
    fsync: None,
    setxattr: None,
    getxattr: None,
    listxattr: None,
    removexattr: None,
    opendir: None,
    readdir: None,
    releasedir: None,
    fsyncdir: None,
    init: None,
    destroy: None,
    access: None,
    create: None,
    lock: None,
    utimens: None,
    bmap: None,
    ioctl: None,
    poll: None,
    write_buf: None,
    read_buf: None,
    flock: None,
    fallocate: None,
    copy_file_range: None,
    lseek: None,
};
pub unsafe extern "C" fn cache_wrap(
    mut oper: *mut fuse_operations,
) -> *mut fuse_operations {
    cache.next_oper = oper;
    cache_fill(oper, &mut cache_oper);
    pthread_mutex_init(
        &mut cache.lock,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
    cache
        .table = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(
            g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
        ),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        Some(free_node as unsafe extern "C" fn(gpointer) -> ()),
    );
    if cache.table as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"failed to create cache\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut libc::c_void as *mut fuse_operations;
    }
    return &mut cache_oper;
}
static mut cache_opts: [fuse_opt; 19] = [fuse_opt {
    templ: 0 as *const libc::c_char,
    offset: 0,
    value: 0,
}; 19];
pub unsafe extern "C" fn cache_parse_options(mut args: *mut fuse_args) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    cache.stat_timeout_secs = 20 as libc::c_uint;
    cache.dir_timeout_secs = 20 as libc::c_uint;
    cache.link_timeout_secs = 20 as libc::c_uint;
    cache.max_size = 10000 as libc::c_uint;
    cache.clean_interval_secs = 60 as libc::c_uint;
    cache.min_clean_interval_secs = 5 as libc::c_uint;
    tmp = fuse_opt_parse(
        args,
        &mut cache as *mut cache as *mut libc::c_void,
        cache_opts.as_ptr(),
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    libc::c_int,
                    *mut fuse_args,
                ) -> libc::c_int,
            >,
        >(0 as *mut libc::c_void),
    );
    return tmp;
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 4278190080 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 16711680 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 65280 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 255 as libc::c_uint) << 24 as libc::c_int;
}
static mut sshfs: sshfs = sshfs {
    directport: 0 as *const libc::c_char as *mut libc::c_char,
    ssh_command: 0 as *const libc::c_char as *mut libc::c_char,
    sftp_server: 0 as *const libc::c_char as *mut libc::c_char,
    ssh_args: fuse_args {
        argc: 0,
        argv: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        allocated: 0,
    },
    workarounds: 0 as *const libc::c_char as *mut libc::c_char,
    rename_workaround: 0,
    renamexdev_workaround: 0,
    truncate_workaround: 0,
    buflimit_workaround: 0,
    unrel_append: 0,
    fstat_workaround: 0,
    createmode_workaround: 0,
    transform_symlinks: 0,
    follow_symlinks: 0,
    no_check_root: 0,
    detect_uid: 0,
    idmap: 0,
    nomap: 0,
    disable_hardlink: 0,
    dir_cache: 0,
    show_version: 0,
    show_help: 0,
    singlethread: 0,
    mountpoint: 0 as *const libc::c_char as *mut libc::c_char,
    uid_file: 0 as *const libc::c_char as *mut libc::c_char,
    gid_file: 0 as *const libc::c_char as *mut libc::c_char,
    uid_map: 0 as *const GHashTable as *mut GHashTable,
    gid_map: 0 as *const GHashTable as *mut GHashTable,
    r_uid_map: 0 as *const GHashTable as *mut GHashTable,
    r_gid_map: 0 as *const GHashTable as *mut GHashTable,
    max_read: 0,
    max_write: 0,
    ssh_ver: 0,
    sync_write: 0,
    sync_read: 0,
    sync_readdir: 0,
    direct_io: 0,
    debug: 0,
    verbose: 0,
    foreground: 0,
    reconnect: 0,
    delay_connect: 0,
    passive: 0,
    host: 0 as *const libc::c_char as *mut libc::c_char,
    base_path: 0 as *const libc::c_char as *mut libc::c_char,
    reqtab: 0 as *const GHashTable as *mut GHashTable,
    conntab: 0 as *const GHashTable as *mut GHashTable,
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
                __prev: 0 as *const __pthread_internal_list
                    as *mut __pthread_internal_list,
                __next: 0 as *const __pthread_internal_list
                    as *mut __pthread_internal_list,
            },
        },
    },
    randseed: 0,
    max_conns: 0,
    conns: 0 as *const conn as *mut conn,
    ptyfd: 0,
    ptypassivefd: 0,
    connvers: 0,
    server_version: 0,
    remote_uid: 0,
    local_uid: 0,
    remote_gid: 0,
    local_gid: 0,
    remote_uid_detected: 0,
    blksize: 0,
    progname: 0 as *const libc::c_char as *mut libc::c_char,
    modifver: 0,
    outstanding_len: 0,
    max_outstanding_len: 0,
    outstanding_cond: __anonunion_pthread_cond_t_951761805 {
        __data: __pthread_cond_s {
            __annonCompField1: __anonunion____missing_field_name_505876810 {
                __wseq: 0,
            },
            __annonCompField2: __anonunion____missing_field_name_897626226 {
                __g1_start: 0,
            },
            __g_refs: [0; 2],
            __g_size: [0; 2],
            __g1_orig_size: 0,
            __wrefs: 0,
            __g_signals: [0; 2],
        },
    },
    password_stdin: 0,
    password: 0 as *const libc::c_char as *mut libc::c_char,
    ext_posix_rename: 0,
    ext_statvfs: 0,
    ext_hardlink: 0,
    ext_fsync: 0,
    op: 0 as *const fuse_operations as *mut fuse_operations,
    bytes_sent: 0,
    bytes_received: 0,
    num_sent: 0,
    num_received: 0,
    min_rtt: 0,
    max_rtt: 0,
    total_rtt: 0,
    num_connect: 0,
};
static mut ssh_opts: [*const libc::c_char; 62] = [
    b"AddressFamily\0" as *const u8 as *const libc::c_char,
    b"BatchMode\0" as *const u8 as *const libc::c_char,
    b"BindAddress\0" as *const u8 as *const libc::c_char,
    b"BindInterface\0" as *const u8 as *const libc::c_char,
    b"CertificateFile\0" as *const u8 as *const libc::c_char,
    b"ChallengeResponseAuthentication\0" as *const u8 as *const libc::c_char,
    b"CheckHostIP\0" as *const u8 as *const libc::c_char,
    b"Cipher\0" as *const u8 as *const libc::c_char,
    b"Ciphers\0" as *const u8 as *const libc::c_char,
    b"Compression\0" as *const u8 as *const libc::c_char,
    b"CompressionLevel\0" as *const u8 as *const libc::c_char,
    b"ConnectionAttempts\0" as *const u8 as *const libc::c_char,
    b"ConnectTimeout\0" as *const u8 as *const libc::c_char,
    b"ControlMaster\0" as *const u8 as *const libc::c_char,
    b"ControlPath\0" as *const u8 as *const libc::c_char,
    b"ControlPersist\0" as *const u8 as *const libc::c_char,
    b"FingerprintHash\0" as *const u8 as *const libc::c_char,
    b"GlobalKnownHostsFile\0" as *const u8 as *const libc::c_char,
    b"GSSAPIAuthentication\0" as *const u8 as *const libc::c_char,
    b"GSSAPIDelegateCredentials\0" as *const u8 as *const libc::c_char,
    b"HostbasedAuthentication\0" as *const u8 as *const libc::c_char,
    b"HostbasedKeyTypes\0" as *const u8 as *const libc::c_char,
    b"HostKeyAlgorithms\0" as *const u8 as *const libc::c_char,
    b"HostKeyAlias\0" as *const u8 as *const libc::c_char,
    b"HostName\0" as *const u8 as *const libc::c_char,
    b"IdentitiesOnly\0" as *const u8 as *const libc::c_char,
    b"IdentityFile\0" as *const u8 as *const libc::c_char,
    b"IdentityAgent\0" as *const u8 as *const libc::c_char,
    b"IPQoS\0" as *const u8 as *const libc::c_char,
    b"KbdInteractiveAuthentication\0" as *const u8 as *const libc::c_char,
    b"KbdInteractiveDevices\0" as *const u8 as *const libc::c_char,
    b"KexAlgorithms\0" as *const u8 as *const libc::c_char,
    b"LocalCommand\0" as *const u8 as *const libc::c_char,
    b"LogLevel\0" as *const u8 as *const libc::c_char,
    b"MACs\0" as *const u8 as *const libc::c_char,
    b"NoHostAuthenticationForLocalhost\0" as *const u8 as *const libc::c_char,
    b"NumberOfPasswordPrompts\0" as *const u8 as *const libc::c_char,
    b"PasswordAuthentication\0" as *const u8 as *const libc::c_char,
    b"PermitLocalCommand\0" as *const u8 as *const libc::c_char,
    b"PKCS11Provider\0" as *const u8 as *const libc::c_char,
    b"Port\0" as *const u8 as *const libc::c_char,
    b"PreferredAuthentications\0" as *const u8 as *const libc::c_char,
    b"ProxyCommand\0" as *const u8 as *const libc::c_char,
    b"ProxyJump\0" as *const u8 as *const libc::c_char,
    b"ProxyUseFdpass\0" as *const u8 as *const libc::c_char,
    b"PubkeyAcceptedKeyTypes\0" as *const u8 as *const libc::c_char,
    b"PubkeyAuthentication\0" as *const u8 as *const libc::c_char,
    b"RekeyLimit\0" as *const u8 as *const libc::c_char,
    b"RevokedHostKeys\0" as *const u8 as *const libc::c_char,
    b"RhostsRSAAuthentication\0" as *const u8 as *const libc::c_char,
    b"RSAAuthentication\0" as *const u8 as *const libc::c_char,
    b"ServerAliveCountMax\0" as *const u8 as *const libc::c_char,
    b"ServerAliveInterval\0" as *const u8 as *const libc::c_char,
    b"SmartcardDevice\0" as *const u8 as *const libc::c_char,
    b"StrictHostKeyChecking\0" as *const u8 as *const libc::c_char,
    b"TCPKeepAlive\0" as *const u8 as *const libc::c_char,
    b"UpdateHostKeys\0" as *const u8 as *const libc::c_char,
    b"UsePrivilegedPort\0" as *const u8 as *const libc::c_char,
    b"UserKnownHostsFile\0" as *const u8 as *const libc::c_char,
    b"VerifyHostKeyDNS\0" as *const u8 as *const libc::c_char,
    b"VisualHostKey\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_void as *mut libc::c_void as *const libc::c_char,
];
static mut sshfs_opts: [fuse_opt; 57] = [fuse_opt {
    templ: 0 as *const libc::c_char,
    offset: 0,
    value: 0,
}; 57];
static mut workaround_opts: [fuse_opt; 17] = [fuse_opt {
    templ: 0 as *const libc::c_char,
    offset: 0,
    value: 0,
}; 17];
unsafe extern "C" fn type_name(mut type_0: uint8_t) -> *const libc::c_char {
    match type_0 as libc::c_int {
        1 => return b"INIT\0" as *const u8 as *const libc::c_char,
        2 => return b"VERSION\0" as *const u8 as *const libc::c_char,
        3 => return b"OPEN\0" as *const u8 as *const libc::c_char,
        4 => return b"CLOSE\0" as *const u8 as *const libc::c_char,
        5 => return b"READ\0" as *const u8 as *const libc::c_char,
        6 => return b"WRITE\0" as *const u8 as *const libc::c_char,
        7 => return b"LSTAT\0" as *const u8 as *const libc::c_char,
        8 => return b"FSTAT\0" as *const u8 as *const libc::c_char,
        9 => return b"SETSTAT\0" as *const u8 as *const libc::c_char,
        10 => return b"FSETSTAT\0" as *const u8 as *const libc::c_char,
        11 => return b"OPENDIR\0" as *const u8 as *const libc::c_char,
        12 => return b"READDIR\0" as *const u8 as *const libc::c_char,
        13 => return b"REMOVE\0" as *const u8 as *const libc::c_char,
        14 => return b"MKDIR\0" as *const u8 as *const libc::c_char,
        15 => return b"RMDIR\0" as *const u8 as *const libc::c_char,
        16 => return b"REALPATH\0" as *const u8 as *const libc::c_char,
        17 => return b"STAT\0" as *const u8 as *const libc::c_char,
        18 => return b"RENAME\0" as *const u8 as *const libc::c_char,
        19 => return b"READLINK\0" as *const u8 as *const libc::c_char,
        20 => return b"SYMLINK\0" as *const u8 as *const libc::c_char,
        101 => return b"STATUS\0" as *const u8 as *const libc::c_char,
        102 => return b"HANDLE\0" as *const u8 as *const libc::c_char,
        103 => return b"DATA\0" as *const u8 as *const libc::c_char,
        104 => return b"NAME\0" as *const u8 as *const libc::c_char,
        105 => return b"ATTRS\0" as *const u8 as *const libc::c_char,
        200 => return b"EXTENDED\0" as *const u8 as *const libc::c_char,
        201 => return b"EXTENDED_REPLY\0" as *const u8 as *const libc::c_char,
        _ => return b"???\0" as *const u8 as *const libc::c_char,
    };
}
unsafe extern "C" fn list_init(mut head: *mut list_head) {
    (*head).next = head;
    (*head).prev = head;
}
unsafe extern "C" fn list_add(mut new: *mut list_head, mut head: *mut list_head) {
    let mut prev: *mut list_head = 0 as *mut list_head;
    let mut next: *mut list_head = 0 as *mut list_head;
    prev = head;
    next = (*head).next;
    (*next).prev = new;
    (*new).next = next;
    (*new).prev = prev;
    (*prev).next = new;
}
unsafe extern "C" fn list_del(mut entry: *mut list_head) {
    let mut prev: *mut list_head = 0 as *mut list_head;
    let mut next: *mut list_head = 0 as *mut list_head;
    prev = (*entry).prev;
    next = (*entry).next;
    (*next).prev = prev;
    (*prev).next = next;
}
unsafe extern "C" fn list_empty(mut head: *const list_head) -> libc::c_int {
    return ((*head).next as libc::c_ulong == head as libc::c_ulong) as libc::c_int;
}
#[inline]
unsafe extern "C" fn translate_id(
    mut id: *mut uint32_t,
    mut map: *mut GHashTable,
) -> libc::c_int {
    let mut id_p: gpointer = 0 as *mut libc::c_void;
    let mut tmp: gboolean = 0;
    tmp = g_hash_table_lookup_extended(
        map,
        *id as gulong as gpointer as gconstpointer,
        0 as *mut libc::c_void as *mut gpointer,
        &mut id_p,
    );
    if tmp != 0 {
        *id = id_p as gulong as guint;
        return 0 as libc::c_int;
    }
    match sshfs.nomap {
        1 => return -(1 as libc::c_int),
        0 => return 0 as libc::c_int,
        _ => {
            fprintf(stderr, b"internal error\n\0" as *const u8 as *const libc::c_char);
            abort();
        }
    };
}
#[inline]
unsafe extern "C" fn buf_init(mut buf: *mut buffer, mut size: size_t) {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    if size != 0 {
        tmp = malloc(size);
        (*buf).p = tmp as *mut uint8_t;
        if ((*buf).p).is_null() {
            fprintf(
                stderr,
                b"sshfs: memory allocation failed\n\0" as *const u8
                    as *const libc::c_char,
            );
            abort();
        }
    } else {
        (*buf).p = 0 as *mut libc::c_void as *mut uint8_t;
    }
    (*buf).len = 0 as libc::c_int as size_t;
    (*buf).size = size;
}
#[inline]
unsafe extern "C" fn buf_free(mut buf: *mut buffer) {
    free((*buf).p as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn buf_finish(mut buf: *mut buffer) {
    (*buf).len = (*buf).size;
}
#[inline]
unsafe extern "C" fn buf_clear(mut buf: *mut buffer) {
    buf_free(buf);
    buf_init(buf, 0 as libc::c_int as size_t);
}
unsafe extern "C" fn buf_resize(mut buf: *mut buffer, mut len: size_t) {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    (*buf)
        .size = ((*buf).len).wrapping_add(len).wrapping_add(63 as libc::c_ulong)
        & 0xffffffffffffffe0 as libc::c_ulong;
    tmp = realloc((*buf).p as *mut libc::c_void, (*buf).size);
    (*buf).p = tmp as *mut uint8_t;
    if ((*buf).p).is_null() {
        fprintf(
            stderr,
            b"sshfs: memory allocation failed\n\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
}
#[inline]
unsafe extern "C" fn buf_check_add(mut buf: *mut buffer, mut len: size_t) {
    if ((*buf).len).wrapping_add(len) > (*buf).size {
        buf_resize(buf, len);
    }
}
#[inline]
unsafe extern "C" fn buf_add_mem(
    mut buf: *mut buffer,
    mut data: *const libc::c_void,
    mut len: size_t,
) {
    buf_check_add(buf, len);
    memcpy(((*buf).p).offset((*buf).len as isize) as *mut libc::c_void, data, len);
    (*buf).len = ((*buf).len as libc::c_ulong).wrapping_add(len) as size_t as size_t;
}
#[inline]
unsafe extern "C" fn buf_add_buf(mut buf: *mut buffer, mut bufa: *const buffer) {
    buf_check_add(buf, (*bufa).len);
    memcpy(
        ((*buf).p).offset((*buf).len as isize) as *mut libc::c_void,
        (*bufa).p as *const libc::c_void,
        (*bufa).len,
    );
    (*buf)
        .len = ((*buf).len as libc::c_ulong).wrapping_add((*bufa).len) as size_t
        as size_t;
}
#[inline]
unsafe extern "C" fn buf_add_uint8(mut buf: *mut buffer, mut val: uint8_t) {
    buf_check_add(buf, 1 as libc::c_int as size_t);
    memcpy(
        ((*buf).p).offset((*buf).len as isize) as *mut libc::c_void,
        &mut val as *mut uint8_t as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    (*buf).len = ((*buf).len).wrapping_add(1);
}
#[inline]
unsafe extern "C" fn buf_add_uint32(mut buf: *mut buffer, mut val: uint32_t) {
    let mut nval: uint32_t = 0;
    let mut tmp: __uint32_t = 0;
    tmp = __bswap_32(val);
    nval = tmp;
    buf_check_add(buf, 4 as libc::c_int as size_t);
    memcpy(
        ((*buf).p).offset((*buf).len as isize) as *mut libc::c_void,
        &mut nval as *mut uint32_t as *const libc::c_void,
        4 as libc::c_int as size_t,
    );
    (*buf)
        .len = ((*buf).len as libc::c_ulong).wrapping_add(4 as libc::c_ulong) as size_t
        as size_t;
}
#[inline]
unsafe extern "C" fn buf_add_uint64(mut buf: *mut buffer, mut val: uint64_t) {
    buf_add_uint32(buf, (val >> 32 as libc::c_int) as uint32_t);
    buf_add_uint32(buf, (val & 4294967295 as libc::c_ulong) as uint32_t);
}
#[inline]
unsafe extern "C" fn buf_add_data(mut buf: *mut buffer, mut data: *const buffer) {
    buf_add_uint32(buf, (*data).len as uint32_t);
    buf_add_mem(buf, (*data).p as *const libc::c_void, (*data).len);
}
#[inline]
unsafe extern "C" fn buf_add_string(mut buf: *mut buffer, mut str: *const libc::c_char) {
    let mut data: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    data.p = str as *mut uint8_t;
    data.len = strlen(str);
    buf_add_data(buf, &mut data as *mut buffer as *const buffer);
}
#[inline]
unsafe extern "C" fn buf_add_path(mut buf: *mut buffer, mut path: *const libc::c_char) {
    let mut realpath___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: size_t = 0;
    if *(sshfs.base_path).offset(0 as libc::c_int as isize) != 0 {
        if *path.offset(1 as libc::c_int as isize) != 0 {
            tmp = strlen(sshfs.base_path as *const libc::c_char);
            if *(sshfs.base_path).offset(tmp.wrapping_sub(1 as libc::c_ulong) as isize)
                as libc::c_int != 47 as libc::c_int
            {
                realpath___0 = g_strdup_printf(
                    b"%s/%s\0" as *const u8 as *const libc::c_char,
                    sshfs.base_path,
                    path.offset(1 as libc::c_int as isize),
                );
            } else {
                realpath___0 = g_strdup_printf(
                    b"%s%s\0" as *const u8 as *const libc::c_char,
                    sshfs.base_path,
                    path.offset(1 as libc::c_int as isize),
                );
            }
        } else {
            realpath___0 = g_strdup(sshfs.base_path as *const gchar);
        }
    } else if *path.offset(1 as libc::c_int as isize) != 0 {
        realpath___0 = g_strdup(path.offset(1 as libc::c_int as isize));
    } else {
        realpath___0 = g_strdup(b".\0" as *const u8 as *const libc::c_char);
    }
    buf_add_string(buf, realpath___0 as *const libc::c_char);
    g_free(realpath___0 as gpointer);
}
unsafe extern "C" fn buf_check_get(
    mut buf: *mut buffer,
    mut len: size_t,
) -> libc::c_int {
    if ((*buf).len).wrapping_add(len) > (*buf).size {
        fprintf(stderr, b"buffer too short\n\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    } else {
        return 0 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn buf_get_mem(
    mut buf: *mut buffer,
    mut data: *mut libc::c_void,
    mut len: size_t,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = buf_check_get(buf, len);
    if tmp == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    memcpy(data, ((*buf).p).offset((*buf).len as isize) as *const libc::c_void, len);
    (*buf).len = ((*buf).len as libc::c_ulong).wrapping_add(len) as size_t as size_t;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn buf_get_uint8(
    mut buf: *mut buffer,
    mut val: *mut uint8_t,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = buf_get_mem(buf, val as *mut libc::c_void, 1 as libc::c_int as size_t);
    return tmp;
}
#[inline]
unsafe extern "C" fn buf_get_uint32(
    mut buf: *mut buffer,
    mut val: *mut uint32_t,
) -> libc::c_int {
    let mut nval: uint32_t = 0;
    let mut tmp: libc::c_int = 0;
    tmp = buf_get_mem(
        buf,
        &mut nval as *mut uint32_t as *mut libc::c_void,
        4 as libc::c_int as size_t,
    );
    if tmp == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    *val = __bswap_32(nval);
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn buf_get_uint64(
    mut buf: *mut buffer,
    mut val: *mut uint64_t,
) -> libc::c_int {
    let mut val1: uint32_t = 0;
    let mut val2: uint32_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = buf_get_uint32(buf, &mut val1);
    if tmp == -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    } else {
        tmp___0 = buf_get_uint32(buf, &mut val2);
        if tmp___0 == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    *val = ((val1 as uint64_t) << 32 as libc::c_int).wrapping_add(val2 as uint64_t);
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn buf_get_data(
    mut buf: *mut buffer,
    mut data: *mut buffer,
) -> libc::c_int {
    let mut len: uint32_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = buf_get_uint32(buf, &mut len);
    if tmp == -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    } else {
        if len as size_t > ((*buf).size).wrapping_sub((*buf).len) {
            return -(1 as libc::c_int);
        }
    }
    buf_init(data, len.wrapping_add(1 as libc::c_uint) as size_t);
    (*data).size = len as size_t;
    tmp___0 = buf_get_mem(buf, (*data).p as *mut libc::c_void, (*data).size);
    if tmp___0 == -(1 as libc::c_int) {
        buf_free(data);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn buf_get_string(
    mut buf: *mut buffer,
    mut str: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut data: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut tmp: libc::c_int = 0;
    tmp = buf_get_data(buf, &mut data);
    if tmp == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    *(data.p).offset(data.size as isize) = '\u{0}' as i32 as uint8_t;
    *str = data.p as *mut libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn buf_get_attrs(
    mut buf: *mut buffer,
    mut stbuf: *mut stat,
    mut flagsp: *mut libc::c_int,
) -> libc::c_int {
    let mut flags: uint32_t = 0;
    let mut size: uint64_t = 0;
    let mut uid: uint32_t = 0;
    let mut gid: uint32_t = 0;
    let mut atime: uint32_t = 0;
    let mut mtime: uint32_t = 0;
    let mut mode: uint32_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut extcount: uint32_t = 0;
    let mut i: libc::c_uint = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: __time_t = 0;
    size = 0 as libc::c_int as uint64_t;
    uid = 0 as libc::c_int as uint32_t;
    gid = 0 as libc::c_int as uint32_t;
    atime = 0 as libc::c_int as uint32_t;
    mtime = 0 as libc::c_int as uint32_t;
    mode = 33279 as libc::c_int as uint32_t;
    tmp = buf_get_uint32(buf, &mut flags);
    if tmp == -(1 as libc::c_int) {
        return -(5 as libc::c_int);
    }
    if !flagsp.is_null() {
        *flagsp = flags as libc::c_int;
    }
    if flags & 1 as libc::c_uint != 0 {
        tmp___0 = buf_get_uint64(buf, &mut size);
        if tmp___0 == -(1 as libc::c_int) {
            return -(5 as libc::c_int);
        }
    }
    if flags & 2 as libc::c_uint != 0 {
        tmp___1 = buf_get_uint32(buf, &mut uid);
        if tmp___1 == -(1 as libc::c_int) {
            return -(5 as libc::c_int)
        } else {
            tmp___2 = buf_get_uint32(buf, &mut gid);
            if tmp___2 == -(1 as libc::c_int) {
                return -(5 as libc::c_int);
            }
        }
    }
    if flags & 4 as libc::c_uint != 0 {
        tmp___3 = buf_get_uint32(buf, &mut mode);
        if tmp___3 == -(1 as libc::c_int) {
            return -(5 as libc::c_int);
        }
    }
    if flags & 8 as libc::c_uint != 0 {
        tmp___4 = buf_get_uint32(buf, &mut atime);
        if tmp___4 == -(1 as libc::c_int) {
            return -(5 as libc::c_int)
        } else {
            tmp___5 = buf_get_uint32(buf, &mut mtime);
            if tmp___5 == -(1 as libc::c_int) {
                return -(5 as libc::c_int);
            }
        }
    }
    if flags & 2147483648 as libc::c_uint != 0 {
        tmp___6 = buf_get_uint32(buf, &mut extcount);
        if tmp___6 == -(1 as libc::c_int) {
            return -(5 as libc::c_int);
        }
        i = 0 as libc::c_uint;
        while i < extcount {
            tmp___8 = buf_get_data(buf, &mut tmp___7);
            if tmp___8 == -(1 as libc::c_int) {
                return -(5 as libc::c_int);
            }
            buf_free(&mut tmp___7);
            tmp___9 = buf_get_data(buf, &mut tmp___7);
            if tmp___9 == -(1 as libc::c_int) {
                return -(5 as libc::c_int);
            }
            buf_free(&mut tmp___7);
            i = i.wrapping_add(1);
        }
    }
    if sshfs.remote_uid_detected != 0 {
        if uid == sshfs.remote_uid {
            uid = sshfs.local_uid;
        }
        if gid == sshfs.remote_gid {
            gid = sshfs.local_gid;
        }
    }
    if sshfs.idmap == 2 as libc::c_int {
        if !(sshfs.uid_map).is_null() {
            tmp___10 = translate_id(&mut uid, sshfs.uid_map);
            if tmp___10 == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
        }
    }
    if sshfs.idmap == 2 as libc::c_int {
        if !(sshfs.gid_map).is_null() {
            tmp___11 = translate_id(&mut gid, sshfs.gid_map);
            if tmp___11 == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
        }
    }
    memset(
        stbuf as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<stat>() as libc::c_ulong,
    );
    (*stbuf).st_mode = mode;
    (*stbuf).st_nlink = 1 as libc::c_int as __nlink_t;
    (*stbuf).st_size = size as __off_t;
    if sshfs.blksize != 0 {
        (*stbuf).st_blksize = sshfs.blksize as __blksize_t;
        (*stbuf)
            .st_blocks = ((size
            .wrapping_add(sshfs.blksize as uint64_t)
            .wrapping_sub(1 as libc::c_ulong) as libc::c_ulonglong
            & !(sshfs.blksize as libc::c_ulonglong).wrapping_sub(1 as libc::c_ulonglong))
            >> 9 as libc::c_int) as __blkcnt_t;
    }
    (*stbuf).st_uid = uid;
    (*stbuf).st_gid = gid;
    (*stbuf).st_atim.tv_sec = atime as __time_t;
    tmp___12 = mtime as __time_t;
    (*stbuf).st_mtim.tv_sec = tmp___12;
    (*stbuf).st_ctim.tv_sec = tmp___12;
    return 0 as libc::c_int;
}
unsafe extern "C" fn buf_get_statvfs(
    mut buf: *mut buffer,
    mut stbuf: *mut statvfs,
) -> libc::c_int {
    let mut bsize: uint64_t = 0;
    let mut frsize: uint64_t = 0;
    let mut blocks: uint64_t = 0;
    let mut bfree: uint64_t = 0;
    let mut bavail: uint64_t = 0;
    let mut files: uint64_t = 0;
    let mut ffree: uint64_t = 0;
    let mut favail: uint64_t = 0;
    let mut fsid: uint64_t = 0;
    let mut flag: uint64_t = 0;
    let mut namemax: uint64_t = 0;
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
    tmp = buf_get_uint64(buf, &mut bsize);
    if tmp == -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    } else {
        tmp___0 = buf_get_uint64(buf, &mut frsize);
        if tmp___0 == -(1 as libc::c_int) {
            return -(1 as libc::c_int)
        } else {
            tmp___1 = buf_get_uint64(buf, &mut blocks);
            if tmp___1 == -(1 as libc::c_int) {
                return -(1 as libc::c_int)
            } else {
                tmp___2 = buf_get_uint64(buf, &mut bfree);
                if tmp___2 == -(1 as libc::c_int) {
                    return -(1 as libc::c_int)
                } else {
                    tmp___3 = buf_get_uint64(buf, &mut bavail);
                    if tmp___3 == -(1 as libc::c_int) {
                        return -(1 as libc::c_int)
                    } else {
                        tmp___4 = buf_get_uint64(buf, &mut files);
                        if tmp___4 == -(1 as libc::c_int) {
                            return -(1 as libc::c_int)
                        } else {
                            tmp___5 = buf_get_uint64(buf, &mut ffree);
                            if tmp___5 == -(1 as libc::c_int) {
                                return -(1 as libc::c_int)
                            } else {
                                tmp___6 = buf_get_uint64(buf, &mut favail);
                                if tmp___6 == -(1 as libc::c_int) {
                                    return -(1 as libc::c_int)
                                } else {
                                    tmp___7 = buf_get_uint64(buf, &mut fsid);
                                    if tmp___7 == -(1 as libc::c_int) {
                                        return -(1 as libc::c_int)
                                    } else {
                                        tmp___8 = buf_get_uint64(buf, &mut flag);
                                        if tmp___8 == -(1 as libc::c_int) {
                                            return -(1 as libc::c_int)
                                        } else {
                                            tmp___9 = buf_get_uint64(buf, &mut namemax);
                                            if tmp___9 == -(1 as libc::c_int) {
                                                return -(1 as libc::c_int);
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
    memset(
        stbuf as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<statvfs>() as libc::c_ulong,
    );
    (*stbuf).f_bsize = bsize;
    (*stbuf).f_frsize = frsize;
    (*stbuf).f_blocks = blocks;
    (*stbuf).f_bfree = bfree;
    (*stbuf).f_bavail = bavail;
    (*stbuf).f_files = files;
    (*stbuf).f_ffree = ffree;
    (*stbuf).f_favail = favail;
    (*stbuf).f_namemax = namemax;
    return 0 as libc::c_int;
}
unsafe extern "C" fn buf_get_entries(
    mut buf: *mut buffer,
    mut dbuf: *mut libc::c_void,
    mut filler: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            *const stat,
            off_t,
            fuse_fill_dir_flags,
        ) -> libc::c_int,
    >,
) -> libc::c_int {
    let mut count: uint32_t = 0;
    let mut i: libc::c_uint = 0;
    let mut tmp: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut longname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stbuf: stat = stat {
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
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    tmp = buf_get_uint32(buf, &mut count);
    if tmp == -(1 as libc::c_int) {
        return -(5 as libc::c_int);
    }
    i = 0 as libc::c_uint;
    while i < count {
        err = -(1 as libc::c_int);
        tmp___0 = buf_get_string(buf, &mut name);
        if tmp___0 == -(1 as libc::c_int) {
            return -(5 as libc::c_int);
        }
        tmp___1 = buf_get_string(buf, &mut longname);
        if tmp___1 != -(1 as libc::c_int) {
            free(longname as *mut libc::c_void);
            err = buf_get_attrs(
                buf,
                &mut stbuf,
                0 as *mut libc::c_void as *mut libc::c_int,
            );
            if err == 0 {
                if sshfs.follow_symlinks != 0 {
                    if stbuf.st_mode & 61440 as libc::c_uint == 40960 as libc::c_uint {
                        stbuf.st_mode = 0 as libc::c_int as __mode_t;
                    }
                }
                (Some(filler.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    dbuf,
                    name as *const libc::c_char,
                    &mut stbuf as *mut stat as *const stat,
                    0 as libc::c_int as off_t,
                    0 as fuse_fill_dir_flags,
                );
            }
        }
        free(name as *mut libc::c_void);
        if err != 0 {
            return err;
        }
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ssh_add_arg(mut arg: *const libc::c_char) {
    let mut tmp: libc::c_int = 0;
    tmp = fuse_opt_add_arg(&mut sshfs.ssh_args, arg);
    if tmp == -(1 as libc::c_int) {
        _exit(1 as libc::c_int);
    }
}
unsafe extern "C" fn pty_expect_loop(mut conn: *mut conn) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut passwd_str: *const libc::c_char = 0 as *const libc::c_char;
    let mut timeout: libc::c_int = 0;
    let mut passwd_len: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut len: libc::c_int = 0;
    let mut c: libc::c_char = 0;
    let mut fds: [pollfd; 2] = [pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    }; 2];
    let mut tmp___0: ssize_t = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut size: size_t = 0;
    let mut tmp___3: libc::c_int = 0;
    passwd_str = b"assword:\0" as *const u8 as *const libc::c_char;
    timeout = 60000 as libc::c_int;
    tmp = strlen(passwd_str);
    passwd_len = tmp as libc::c_int;
    len = 0 as libc::c_int;
    loop {
        fds[0 as libc::c_int as usize].fd = (*conn).rfd;
        fds[0 as libc::c_int as usize].events = 1 as libc::c_int as libc::c_short;
        fds[1 as libc::c_int as usize].fd = sshfs.ptyfd;
        fds[1 as libc::c_int as usize].events = 1 as libc::c_int as libc::c_short;
        res = poll(fds.as_mut_ptr(), 2 as libc::c_int as nfds_t, timeout);
        if res == -(1 as libc::c_int) {
            perror(b"poll\0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int);
        }
        if res == 0 as libc::c_int {
            fprintf(
                stderr,
                b"Timeout waiting for prompt\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        if fds[0 as libc::c_int as usize].revents != 0 {
            break;
        }
        tmp___0 = read(
            sshfs.ptyfd,
            &mut c as *mut libc::c_char as *mut libc::c_void,
            1 as libc::c_int as size_t,
        );
        res = tmp___0 as libc::c_int;
        if res == -(1 as libc::c_int) {
            perror(b"read\0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int);
        }
        if res == 0 as libc::c_int {
            fprintf(
                stderr,
                b"EOF while waiting for prompt\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        buf[len as usize] = c;
        len += 1;
        if len == passwd_len {
            tmp___2 = memcmp(
                buf.as_mut_ptr() as *const libc::c_void,
                passwd_str as *const libc::c_void,
                passwd_len as size_t,
            );
            if tmp___2 == 0 as libc::c_int {
                tmp___1 = strlen(sshfs.password as *const libc::c_char);
                write(sshfs.ptyfd, sshfs.password as *const libc::c_void, tmp___1);
            }
            memmove(
                buf.as_mut_ptr() as *mut libc::c_void,
                buf.as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *const libc::c_void,
                (passwd_len - 1 as libc::c_int) as size_t,
            );
            len -= 1;
        }
    }
    if sshfs.reconnect == 0 {
        tmp___3 = getpagesize();
        size = tmp___3 as size_t;
        memset(sshfs.password as *mut libc::c_void, 0 as libc::c_int, size);
        munmap(sshfs.password as *mut libc::c_void, size);
        sshfs.password = 0 as *mut libc::c_void as *mut libc::c_char;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_conn(
    mut sf: *const sshfs_file,
    mut path: *const libc::c_char,
) -> *mut conn {
    let mut ce: *mut conntab_entry = 0 as *mut conntab_entry;
    let mut i: libc::c_int = 0;
    let mut tmp: gpointer = 0 as *mut libc::c_void;
    let mut conn: *mut conn = 0 as *mut conn;
    let mut best_index: libc::c_int = 0;
    let mut best_score: uint64_t = 0;
    let mut score: uint64_t = 0;
    let mut tmp___0: libc::c_int = 0;
    if sshfs.max_conns == 1 as libc::c_int {
        return (sshfs.conns).offset(0 as libc::c_int as isize);
    }
    if sf as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        return (*sf).conn;
    }
    if path as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        pthread_mutex_lock(&mut sshfs.lock);
        tmp = g_hash_table_lookup(sshfs.conntab, path as gconstpointer);
        ce = tmp as *mut conntab_entry;
        if ce as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            conn = (*ce).conn;
            pthread_mutex_unlock(&mut sshfs.lock);
            return conn;
        }
        pthread_mutex_unlock(&mut sshfs.lock);
    }
    best_index = 0 as libc::c_int;
    best_score = !(0 as libc::c_ulonglong) as uint64_t;
    i = 0 as libc::c_int;
    while i < sshfs.max_conns {
        if (*(sshfs.conns).offset(i as isize)).rfd >= 0 as libc::c_int {
            tmp___0 = 0 as libc::c_int;
        } else {
            tmp___0 = 1 as libc::c_int;
        }
        score = (((*(sshfs.conns).offset(i as isize)).req_count as uint64_t)
            << 43 as libc::c_int)
            .wrapping_add(
                ((*(sshfs.conns).offset(i as isize)).dir_count as uint64_t)
                    << 22 as libc::c_int,
            )
            .wrapping_add(
                ((*(sshfs.conns).offset(i as isize)).file_count as uint64_t)
                    << 1 as libc::c_int,
            )
            .wrapping_add(tmp___0 as uint64_t);
        if score < best_score {
            best_index = i;
            best_score = score;
        }
        i += 1;
    }
    return (sshfs.conns).offset(best_index as isize);
}
unsafe extern "C" fn pty_master(mut name: *mut *mut libc::c_char) -> libc::c_int {
    let mut mfd: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    mfd = open(b"/dev/ptmx\0" as *const u8 as *const libc::c_char, 258 as libc::c_int);
    if mfd == -(1 as libc::c_int) {
        perror(b"failed to open pty\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    tmp = grantpt(mfd);
    if tmp != 0 as libc::c_int {
        perror(b"grantpt\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    tmp___0 = unlockpt(mfd);
    if tmp___0 != 0 as libc::c_int {
        perror(b"unlockpt\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    *name = ptsname(mfd);
    return mfd;
}
unsafe extern "C" fn replace_arg(
    mut argp: *mut *mut libc::c_char,
    mut newarg: *const libc::c_char,
) {
    free(*argp as *mut libc::c_void);
    *argp = strdup(newarg);
    if *argp as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"sshfs: memory allocation failed\n\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
}
unsafe extern "C" fn start_ssh(mut conn: *mut conn) -> libc::c_int {
    let mut ptyname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sockpair: [libc::c_int; 2] = [0; 2];
    let mut pid: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut devnull: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: __pid_t = 0;
    let mut sfd: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    ptyname = 0 as *mut libc::c_void as *mut libc::c_char;
    if sshfs.password_stdin != 0 {
        sshfs.ptyfd = pty_master(&mut ptyname);
        if sshfs.ptyfd == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        sshfs.ptypassivefd = open(ptyname as *const libc::c_char, 258 as libc::c_int);
        if sshfs.ptypassivefd == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    tmp = socketpair(
        1 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        sockpair.as_mut_ptr(),
    );
    if tmp == -(1 as libc::c_int) {
        perror(b"failed to create socket pair\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    (*conn).rfd = sockpair[0 as libc::c_int as usize];
    (*conn).wfd = sockpair[0 as libc::c_int as usize];
    pid = fork();
    if pid == -(1 as libc::c_int) {
        perror(b"failed to fork\0" as *const u8 as *const libc::c_char);
        close(sockpair[1 as libc::c_int as usize]);
        return -(1 as libc::c_int);
    } else {
        if pid == 0 as libc::c_int {
            devnull = open(
                b"/dev/null\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            );
            tmp___0 = dup2(sockpair[1 as libc::c_int as usize], 0 as libc::c_int);
            if tmp___0 == -(1 as libc::c_int) {
                perror(
                    b"failed to redirect input/output\0" as *const u8
                        as *const libc::c_char,
                );
                _exit(1 as libc::c_int);
            } else {
                tmp___1 = dup2(sockpair[1 as libc::c_int as usize], 1 as libc::c_int);
                if tmp___1 == -(1 as libc::c_int) {
                    perror(
                        b"failed to redirect input/output\0" as *const u8
                            as *const libc::c_char,
                    );
                    _exit(1 as libc::c_int);
                }
            }
            if sshfs.verbose == 0 {
                if sshfs.foreground == 0 {
                    if devnull != -(1 as libc::c_int) {
                        dup2(devnull, 2 as libc::c_int);
                    }
                }
            }
            close(devnull);
            close(sockpair[0 as libc::c_int as usize]);
            close(sockpair[1 as libc::c_int as usize]);
            tmp___2 = fork();
            match tmp___2 {
                -1 => {
                    perror(b"failed to fork\0" as *const u8 as *const libc::c_char);
                    _exit(1 as libc::c_int);
                }
                0 => {}
                _ => {
                    _exit(0 as libc::c_int);
                }
            }
            chdir(b"/\0" as *const u8 as *const libc::c_char);
            unsetenv(b"OLDPWD\0" as *const u8 as *const libc::c_char);
            if sshfs.password_stdin != 0 {
                setsid();
                sfd = open(ptyname as *const libc::c_char, 2 as libc::c_int);
                if sfd == -(1 as libc::c_int) {
                    perror(ptyname as *const libc::c_char);
                    _exit(1 as libc::c_int);
                }
                close(sfd);
                close(sshfs.ptypassivefd);
                close(sshfs.ptyfd);
            }
            if sshfs.debug != 0 {
                fprintf(stderr, b"executing\0" as *const u8 as *const libc::c_char);
                i = 0 as libc::c_int;
                while i < sshfs.ssh_args.argc {
                    fprintf(
                        stderr,
                        b" <%s>\0" as *const u8 as *const libc::c_char,
                        *(sshfs.ssh_args.argv).offset(i as isize),
                    );
                    i += 1;
                }
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            }
            execvp(
                *(sshfs.ssh_args.argv).offset(0 as libc::c_int as isize)
                    as *const libc::c_char,
                sshfs.ssh_args.argv as *const *mut libc::c_char,
            );
            tmp___3 = __errno_location();
            tmp___4 = strerror(*tmp___3);
            fprintf(
                stderr,
                b"failed to execute '%s': %s\n\0" as *const u8 as *const libc::c_char,
                *(sshfs.ssh_args.argv).offset(0 as libc::c_int as isize),
                tmp___4,
            );
            _exit(1 as libc::c_int);
        }
    }
    waitpid(pid, 0 as *mut libc::c_void as *mut libc::c_int, 0 as libc::c_int);
    close(sockpair[1 as libc::c_int as usize]);
    return 0 as libc::c_int;
}
unsafe extern "C" fn connect_passive(mut conn: *mut conn) -> libc::c_int {
    (*conn).rfd = 0 as libc::c_int;
    (*conn).wfd = 1 as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn connect_to(
    mut conn: *mut conn,
    mut host: *mut libc::c_char,
    mut port: *mut libc::c_char,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut sock: libc::c_int = 0;
    let mut opt: libc::c_int = 0;
    let mut ai: *mut addrinfo = 0 as *mut addrinfo;
    let mut hint: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    memset(
        &mut hint as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hint.ai_family = 2 as libc::c_int;
    hint.ai_socktype = 1 as libc::c_int;
    err = getaddrinfo(
        host as *const libc::c_char,
        port as *const libc::c_char,
        &mut hint as *mut addrinfo as *const addrinfo,
        &mut ai as *mut *mut addrinfo,
    );
    if err != 0 {
        tmp = gai_strerror(err);
        fprintf(
            stderr,
            b"failed to resolve %s:%s: %s\n\0" as *const u8 as *const libc::c_char,
            host,
            port,
            tmp,
        );
        return -(1 as libc::c_int);
    }
    sock = socket((*ai).ai_family, (*ai).ai_socktype, (*ai).ai_protocol);
    if sock == -(1 as libc::c_int) {
        perror(b"failed to create socket\0" as *const u8 as *const libc::c_char);
        freeaddrinfo(ai);
        return -(1 as libc::c_int);
    }
    err = connect(sock, (*ai).ai_addr as *const sockaddr, (*ai).ai_addrlen);
    if err == -(1 as libc::c_int) {
        perror(b"failed to connect\0" as *const u8 as *const libc::c_char);
        freeaddrinfo(ai);
        close(sock);
        return -(1 as libc::c_int);
    }
    opt = 1 as libc::c_int;
    err = setsockopt(
        sock,
        6 as libc::c_int,
        1 as libc::c_int,
        &mut opt as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    if err == -(1 as libc::c_int) {
        perror(
            b"warning: failed to set TCP_NODELAY\0" as *const u8 as *const libc::c_char,
        );
    }
    freeaddrinfo(ai);
    (*conn).rfd = sock;
    (*conn).wfd = sock;
    return 0 as libc::c_int;
}
unsafe extern "C" fn do_write(
    mut conn: *mut conn,
    mut iov: *mut iovec,
    mut count: size_t,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut tmp: ssize_t = 0;
    while count != 0 {
        tmp = writev((*conn).wfd, iov as *const iovec, count as libc::c_int);
        res = tmp as libc::c_int;
        if res == -(1 as libc::c_int) {
            perror(b"write\0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int);
        } else {
            if res == 0 as libc::c_int {
                fprintf(stderr, b"zero write\n\0" as *const u8 as *const libc::c_char);
                return -(1 as libc::c_int);
            }
        }
        loop {
            if (res as libc::c_uint as size_t) < (*iov).iov_len {
                (*iov)
                    .iov_len = ((*iov).iov_len as libc::c_ulong)
                    .wrapping_sub(res as size_t) as size_t as size_t;
                (*iov).iov_base = ((*iov).iov_base).offset(res as isize);
                break;
            } else {
                res = (res as size_t).wrapping_sub((*iov).iov_len) as libc::c_int;
                count = count.wrapping_sub(1);
                iov = iov.offset(1);
                if count == 0 {
                    break;
                }
            }
        }
    }
    return 0 as libc::c_int;
}
static mut idctr: uint32_t = 0;
unsafe extern "C" fn sftp_get_id() -> uint32_t {
    let mut tmp: uint32_t = 0;
    tmp = idctr;
    idctr = idctr.wrapping_add(1);
    return tmp;
}
unsafe extern "C" fn buf_to_iov(mut buf: *const buffer, mut iov: *mut iovec) {
    (*iov).iov_base = (*buf).p as *mut libc::c_void;
    (*iov).iov_len = (*buf).len;
}
unsafe extern "C" fn iov_length(
    mut iov: *const iovec,
    mut nr_segs: libc::c_ulong,
) -> size_t {
    let mut seg: libc::c_ulong = 0;
    let mut ret: size_t = 0;
    ret = 0 as libc::c_int as size_t;
    seg = 0 as libc::c_ulong;
    while seg < nr_segs {
        ret = (ret as libc::c_ulong).wrapping_add((*iov.offset(seg as isize)).iov_len)
            as size_t as size_t;
        seg = seg.wrapping_add(1);
    }
    return ret;
}
unsafe extern "C" fn sftp_send_iov(
    mut conn: *mut conn,
    mut type_0: uint8_t,
    mut id: uint32_t,
    mut iov: *mut iovec,
    mut count: size_t,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut iovout: [iovec; 3] = [iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    }; 3];
    let mut i: libc::c_uint = 0;
    let mut nout: libc::c_uint = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: libc::c_uint = 0;
    let mut tmp___2: libc::c_uint = 0;
    nout = 0 as libc::c_uint;
    if !(count <= 2 as libc::c_ulong) {
        __assert_fail(
            b"count <= SFTP_MAX_IOV - 1\0" as *const u8 as *const libc::c_char,
            b"../sshfs.c\0" as *const u8 as *const libc::c_char,
            1344 as libc::c_uint,
            b"sftp_send_iov\0" as *const u8 as *const libc::c_char,
        );
    }
    buf_init(&mut buf, 9 as libc::c_int as size_t);
    tmp___0 = iov_length(iov as *const iovec, count);
    buf_add_uint32(&mut buf, tmp___0.wrapping_add(5 as libc::c_ulong) as uint32_t);
    buf_add_uint8(&mut buf, type_0);
    buf_add_uint32(&mut buf, id);
    tmp___1 = nout;
    nout = nout.wrapping_add(1);
    buf_to_iov(
        &mut buf as *mut buffer as *const buffer,
        &mut *iovout.as_mut_ptr().offset(tmp___1 as isize),
    );
    i = 0 as libc::c_uint;
    while (i as size_t) < count {
        tmp___2 = nout;
        nout = nout.wrapping_add(1);
        iovout[tmp___2 as usize] = *iov.offset(i as isize);
        i = i.wrapping_add(1);
    }
    pthread_mutex_lock(&mut (*conn).lock_write);
    res = do_write(conn, iovout.as_mut_ptr(), nout as size_t);
    pthread_mutex_unlock(&mut (*conn).lock_write);
    buf_free(&mut buf);
    return res;
}
unsafe extern "C" fn do_read(mut conn: *mut conn, mut buf: *mut buffer) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut size: size_t = 0;
    let mut tmp: ssize_t = 0;
    p = (*buf).p;
    size = (*buf).size;
    while size != 0 {
        tmp = read((*conn).rfd, p as *mut libc::c_void, size);
        res = tmp as libc::c_int;
        if res == -(1 as libc::c_int) {
            perror(b"read\0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int);
        } else {
            if res == 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"remote host has disconnected\n\0" as *const u8
                        as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
        }
        size = (size as libc::c_ulong).wrapping_sub(res as size_t) as size_t as size_t;
        p = p.offset(res as isize);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn sftp_read(
    mut conn: *mut conn,
    mut type_0: *mut uint8_t,
    mut buf: *mut buffer,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut buf2: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut len: uint32_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    buf_init(&mut buf2, 5 as libc::c_int as size_t);
    res = do_read(conn, &mut buf2);
    if res != -(1 as libc::c_int) {
        tmp = buf_get_uint32(&mut buf2, &mut len);
        if tmp == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        if len > ((1 as libc::c_int) << 17 as libc::c_int) as uint32_t {
            fprintf(
                stderr,
                b"reply len too large: %u\n\0" as *const u8 as *const libc::c_char,
                len,
            );
            return -(1 as libc::c_int);
        }
        tmp___0 = buf_get_uint8(&mut buf2, type_0);
        if tmp___0 == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        buf_init(buf, len.wrapping_sub(1 as libc::c_uint) as size_t);
        res = do_read(conn, buf);
    }
    buf_free(&mut buf2);
    return res;
}
unsafe extern "C" fn request_free(mut req: *mut request) {
    if ((*req).end_func).is_some() {
        (Some(((*req).end_func).expect("non-null function pointer")))
            .expect("non-null function pointer")(req);
    }
    (*(*req).conn).req_count -= 1;
    buf_free(&mut (*req).reply);
    sem_destroy(&mut (*req).ready);
    g_free(req as gpointer);
}
unsafe extern "C" fn chunk_free(mut chunk: *mut read_chunk) {
    let mut rreq: *mut read_req = 0 as *mut read_req;
    let mut __mptr: *const list_head = 0 as *const list_head;
    let mut tmp: libc::c_int = 0;
    loop {
        tmp = list_empty(&mut (*chunk).reqs as *mut list_head as *const list_head);
        if tmp != 0 {
            break;
        }
        __mptr = (*chunk).reqs.prev as *const list_head;
        rreq = (__mptr as *mut libc::c_char)
            .offset(
                -(&mut (*(0 as *mut read_req)).list as *mut list_head as libc::c_ulong
                    as isize),
            ) as *mut read_req;
        list_del(&mut (*rreq).list);
        buf_free(&mut (*rreq).data);
        g_free(rreq as gpointer);
    }
    g_free(chunk as gpointer);
}
unsafe extern "C" fn chunk_put(mut chunk: *mut read_chunk) {
    if !chunk.is_null() {
        (*chunk).refs -= 1;
        if (*chunk).refs == 0 {
            chunk_free(chunk);
        }
    }
}
unsafe extern "C" fn chunk_put_locked(mut chunk: *mut read_chunk) {
    pthread_mutex_lock(&mut sshfs.lock);
    chunk_put(chunk);
    pthread_mutex_unlock(&mut sshfs.lock);
}
unsafe extern "C" fn clean_req(
    mut key: *mut libc::c_void,
    mut req: *mut request,
    mut user_data: gpointer,
) -> libc::c_int {
    let mut conn: *mut conn = 0 as *mut conn;
    conn = user_data as *mut conn;
    if (*req).conn as libc::c_ulong != conn as libc::c_ulong {
        return 0 as libc::c_int;
    }
    (*req).error = -(5 as libc::c_int);
    if (*req).want_reply != 0 {
        sem_post(&mut (*req).ready);
    } else {
        request_free(req);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn process_one_request(mut conn: *mut conn) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut type_0: uint8_t = 0;
    let mut req: *mut request = 0 as *mut request;
    let mut id: uint32_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: gpointer = 0 as *mut libc::c_void;
    let mut was_over: libc::c_int = 0;
    let mut now: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut difftime___0: libc::c_uint = 0;
    let mut msgsize: libc::c_uint = 0;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    res = sftp_read(conn, &mut type_0, &mut buf);
    if res == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    tmp = buf_get_uint32(&mut buf, &mut id);
    if tmp == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    pthread_mutex_lock(&mut sshfs.lock);
    tmp___0 = g_hash_table_lookup(
        sshfs.reqtab,
        id as gulong as gpointer as gconstpointer,
    );
    req = tmp___0 as *mut request;
    if req as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"request %i not found\n\0" as *const u8 as *const libc::c_char,
            id,
        );
    } else {
        was_over = (sshfs.outstanding_len > sshfs.max_outstanding_len) as libc::c_int;
        sshfs
            .outstanding_len = (sshfs.outstanding_len as size_t).wrapping_sub((*req).len)
            as libc::c_uint;
        if was_over != 0 {
            if sshfs.outstanding_len <= sshfs.max_outstanding_len {
                pthread_cond_broadcast(&mut sshfs.outstanding_cond);
            }
        }
        g_hash_table_remove(sshfs.reqtab, id as gulong as gpointer as gconstpointer);
    }
    pthread_mutex_unlock(&mut sshfs.lock);
    if req as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        if sshfs.debug != 0 {
            msgsize = (buf.size).wrapping_add(5 as libc::c_ulong) as libc::c_uint;
            gettimeofday(&mut now as *mut timeval, 0 as *mut libc::c_void);
            difftime___0 = ((now.tv_sec - (*req).start.tv_sec) * 1000 as libc::c_long)
                as libc::c_uint;
            difftime___0 = (difftime___0 as __suseconds_t
                + (now.tv_usec - (*req).start.tv_usec) / 1000 as libc::c_long)
                as libc::c_uint;
            if sshfs.debug != 0 {
                tmp___1 = type_name(type_0);
                fprintf(
                    stderr,
                    b"  [%05i] %14s %8ubytes (%ims)\n\0" as *const u8
                        as *const libc::c_char,
                    id,
                    tmp___1,
                    msgsize,
                    difftime___0,
                );
            }
            if difftime___0 < sshfs.min_rtt {
                sshfs.min_rtt = difftime___0;
            } else if sshfs.num_received == 0 {
                sshfs.min_rtt = difftime___0;
            }
            if difftime___0 > sshfs.max_rtt {
                sshfs.max_rtt = difftime___0;
            }
            sshfs
                .total_rtt = (sshfs.total_rtt as libc::c_ulong)
                .wrapping_add(difftime___0 as uint64_t) as uint64_t as uint64_t;
            sshfs.num_received = (sshfs.num_received).wrapping_add(1);
            sshfs
                .bytes_received = (sshfs.bytes_received as libc::c_ulong)
                .wrapping_add(msgsize as uint64_t) as uint64_t as uint64_t;
        }
        (*req).reply = buf;
        (*req).reply_type = type_0;
        (*req).replied = 1 as libc::c_int;
        if (*req).want_reply != 0 {
            sem_post(&mut (*req).ready);
        } else {
            pthread_mutex_lock(&mut sshfs.lock);
            request_free(req);
            pthread_mutex_unlock(&mut sshfs.lock);
        }
    } else {
        buf_free(&mut buf);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn close_conn(mut conn: *mut conn) {
    close((*conn).rfd);
    if (*conn).rfd != (*conn).wfd {
        close((*conn).wfd);
    }
    (*conn).rfd = -(1 as libc::c_int);
    (*conn).wfd = -(1 as libc::c_int);
    if sshfs.ptyfd != -(1 as libc::c_int) {
        close(sshfs.ptyfd);
        sshfs.ptyfd = -(1 as libc::c_int);
    }
    if sshfs.ptypassivefd != -(1 as libc::c_int) {
        close(sshfs.ptypassivefd);
        sshfs.ptypassivefd = -(1 as libc::c_int);
    }
}
unsafe extern "C" fn process_requests(
    mut data_: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut conn: *mut conn = 0 as *mut conn;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: __pid_t = 0;
    conn = data_ as *mut conn;
    loop {
        tmp = process_one_request(conn);
        if tmp == -(1 as libc::c_int) {
            break;
        }
    }
    pthread_mutex_lock(&mut sshfs.lock);
    (*conn).processing_thread_started = 0 as libc::c_int;
    close_conn(conn);
    g_hash_table_foreach_remove(
        sshfs.reqtab,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut request,
                    gpointer,
                ) -> libc::c_int,
            >,
            Option::<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gboolean>,
        >(
            Some(
                clean_req
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut request,
                        gpointer,
                    ) -> libc::c_int,
            ),
        ),
        conn as gpointer,
    );
    sshfs.connvers += 1;
    (*conn).connver = sshfs.connvers;
    sshfs.outstanding_len = 0 as libc::c_uint;
    pthread_cond_broadcast(&mut sshfs.outstanding_cond);
    pthread_mutex_unlock(&mut sshfs.lock);
    if sshfs.reconnect == 0 {
        tmp___0 = getpid();
        kill(tmp___0, 15 as libc::c_int);
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn sftp_init_reply_ok(
    mut conn: *mut conn,
    mut buf: *mut buffer,
    mut version: *mut uint32_t,
) -> libc::c_int {
    let mut len: uint32_t = 0;
    let mut type_0: uint8_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut buf2: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut tmp___2: libc::c_int = 0;
    let mut ext: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut extdata: *mut libc::c_char = 0 as *mut libc::c_char;
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
    tmp = buf_get_uint32(buf, &mut len);
    if tmp == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if len < 5 as libc::c_uint {
        return 1 as libc::c_int
    } else {
        if len > ((1 as libc::c_int) << 17 as libc::c_int) as uint32_t {
            return 1 as libc::c_int;
        }
    }
    tmp___0 = buf_get_uint8(buf, &mut type_0);
    if tmp___0 == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if type_0 as libc::c_int != 2 as libc::c_int {
        return 1 as libc::c_int;
    }
    tmp___1 = buf_get_uint32(buf, version);
    if tmp___1 == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if sshfs.debug != 0 {
        fprintf(
            stderr,
            b"Server version: %u\n\0" as *const u8 as *const libc::c_char,
            *version,
        );
    }
    if len > 5 as libc::c_uint {
        buf_init(&mut buf2, len.wrapping_sub(5 as libc::c_uint) as size_t);
        tmp___2 = do_read(conn, &mut buf2);
        if tmp___2 == -(1 as libc::c_int) {
            buf_free(&mut buf2);
            return -(1 as libc::c_int);
        }
        loop {
            ext = 0 as *mut libc::c_void as *mut libc::c_char;
            extdata = 0 as *mut libc::c_void as *mut libc::c_char;
            tmp___3 = buf_get_string(&mut buf2, &mut ext);
            if tmp___3 == -(1 as libc::c_int) {
                buf_free(&mut buf2);
                free(ext as *mut libc::c_void);
                free(extdata as *mut libc::c_void);
                return -(1 as libc::c_int);
            } else {
                tmp___4 = buf_get_string(&mut buf2, &mut extdata);
                if tmp___4 == -(1 as libc::c_int) {
                    buf_free(&mut buf2);
                    free(ext as *mut libc::c_void);
                    free(extdata as *mut libc::c_void);
                    return -(1 as libc::c_int);
                }
            }
            if sshfs.debug != 0 {
                fprintf(
                    stderr,
                    b"Extension: %s <%s>\n\0" as *const u8 as *const libc::c_char,
                    ext,
                    extdata,
                );
            }
            tmp___5 = strcmp(
                ext as *const libc::c_char,
                b"posix-rename@openssh.com\0" as *const u8 as *const libc::c_char,
            );
            if tmp___5 == 0 as libc::c_int {
                tmp___6 = strcmp(
                    extdata as *const libc::c_char,
                    b"1\0" as *const u8 as *const libc::c_char,
                );
                if tmp___6 == 0 as libc::c_int {
                    sshfs.ext_posix_rename = 1 as libc::c_int;
                    sshfs.rename_workaround = 0 as libc::c_int;
                }
            }
            tmp___7 = strcmp(
                ext as *const libc::c_char,
                b"statvfs@openssh.com\0" as *const u8 as *const libc::c_char,
            );
            if tmp___7 == 0 as libc::c_int {
                tmp___8 = strcmp(
                    extdata as *const libc::c_char,
                    b"2\0" as *const u8 as *const libc::c_char,
                );
                if tmp___8 == 0 as libc::c_int {
                    sshfs.ext_statvfs = 1 as libc::c_int;
                }
            }
            tmp___9 = strcmp(
                ext as *const libc::c_char,
                b"hardlink@openssh.com\0" as *const u8 as *const libc::c_char,
            );
            if tmp___9 == 0 as libc::c_int {
                tmp___10 = strcmp(
                    extdata as *const libc::c_char,
                    b"1\0" as *const u8 as *const libc::c_char,
                );
                if tmp___10 == 0 as libc::c_int {
                    sshfs.ext_hardlink = 1 as libc::c_int;
                }
            }
            tmp___11 = strcmp(
                ext as *const libc::c_char,
                b"fsync@openssh.com\0" as *const u8 as *const libc::c_char,
            );
            if tmp___11 == 0 as libc::c_int {
                tmp___12 = strcmp(
                    extdata as *const libc::c_char,
                    b"1\0" as *const u8 as *const libc::c_char,
                );
                if tmp___12 == 0 as libc::c_int {
                    sshfs.ext_fsync = 1 as libc::c_int;
                }
            }
            free(ext as *mut libc::c_void);
            free(extdata as *mut libc::c_void);
            if !(buf2.len < buf2.size) {
                break;
            }
        }
        buf_free(&mut buf2);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn sftp_find_init_reply(
    mut conn: *mut conn,
    mut version: *mut uint32_t,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut buf2: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    buf_init(&mut buf, 9 as libc::c_int as size_t);
    res = do_read(conn, &mut buf);
    while res != -(1 as libc::c_int) {
        res = sftp_init_reply_ok(conn, &mut buf, version);
        if res <= 0 as libc::c_int {
            break;
        }
        if sshfs.debug != 0 {
            fprintf(
                stderr,
                b"%c\0" as *const u8 as *const libc::c_char,
                *buf.p as libc::c_int,
            );
        }
        memmove(
            buf.p as *mut libc::c_void,
            (buf.p).offset(1 as libc::c_int as isize) as *const libc::c_void,
            (buf.size).wrapping_sub(1 as libc::c_ulong),
        );
        buf.len = 0 as libc::c_int as size_t;
        buf2.p = (buf.p).offset(buf.size as isize).offset(-(1 as libc::c_int as isize));
        buf2.size = 1 as libc::c_int as size_t;
        res = do_read(conn, &mut buf2);
    }
    buf_free(&mut buf);
    return res;
}
unsafe extern "C" fn sftp_init(mut conn: *mut conn) -> libc::c_int {
    let mut current_block: u64;
    let mut res: libc::c_int = 0;
    let mut version: uint32_t = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    res = -(1 as libc::c_int);
    version = 0 as libc::c_int as uint32_t;
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    tmp = sftp_send_iov(
        conn,
        1 as libc::c_int as uint8_t,
        3 as libc::c_int as uint32_t,
        0 as *mut libc::c_void as *mut iovec,
        0 as libc::c_int as size_t,
    );
    if !(tmp == -(1 as libc::c_int)) {
        if sshfs.password_stdin != 0 {
            tmp___0 = pty_expect_loop(conn);
            if tmp___0 == -(1 as libc::c_int) {
                current_block = 7913488200737463304;
            } else {
                current_block = 1394248824506584008;
            }
        } else {
            current_block = 1394248824506584008;
        }
        match current_block {
            7913488200737463304 => {}
            _ => {
                tmp___1 = sftp_find_init_reply(conn, &mut version);
                if !(tmp___1 == -(1 as libc::c_int)) {
                    sshfs.server_version = version as libc::c_int;
                    if version > 3 as libc::c_uint {
                        fprintf(
                            stderr,
                            b"Warning: server uses version: %i, we support: %i\n\0"
                                as *const u8 as *const libc::c_char,
                            version,
                            3 as libc::c_int,
                        );
                    }
                    res = 0 as libc::c_int;
                }
            }
        }
    }
    buf_free(&mut buf);
    return res;
}
unsafe extern "C" fn sftp_error_to_errno(mut error: uint32_t) -> libc::c_int {
    match error {
        0 => return 0 as libc::c_int,
        2 => return 2 as libc::c_int,
        3 => return 13 as libc::c_int,
        4 => return 1 as libc::c_int,
        5 => return 74 as libc::c_int,
        6 => return 107 as libc::c_int,
        7 => return 103 as libc::c_int,
        8 => return 95 as libc::c_int,
        _ => return 5 as libc::c_int,
    };
}
unsafe extern "C" fn sftp_detect_uid(mut conn: *mut conn) {
    let mut current_block: u64;
    let mut flags: libc::c_int = 0;
    let mut id: uint32_t = 0;
    let mut tmp: uint32_t = 0;
    let mut replid: uint32_t = 0;
    let mut type_0: uint8_t = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut stbuf: stat = stat {
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
    let mut iov: [iovec; 1] = [iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    }; 1];
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut serr: uint32_t = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    tmp = sftp_get_id();
    id = tmp;
    buf_init(&mut buf, 5 as libc::c_int as size_t);
    buf_add_string(&mut buf, b".\0" as *const u8 as *const libc::c_char);
    buf_to_iov(
        &mut buf as *mut buffer as *const buffer,
        &mut *iov.as_mut_ptr().offset(0 as libc::c_int as isize),
    );
    tmp___0 = sftp_send_iov(
        conn,
        17 as libc::c_int as uint8_t,
        id,
        iov.as_mut_ptr(),
        1 as libc::c_int as size_t,
    );
    if !(tmp___0 == -(1 as libc::c_int)) {
        buf_clear(&mut buf);
        tmp___1 = sftp_read(conn, &mut type_0, &mut buf);
        if !(tmp___1 == -(1 as libc::c_int)) {
            if type_0 as libc::c_int != 105 as libc::c_int {
                if type_0 as libc::c_int != 101 as libc::c_int {
                    fprintf(
                        stderr,
                        b"protocol error\n\0" as *const u8 as *const libc::c_char,
                    );
                    current_block = 6492071313629511791;
                } else {
                    current_block = 7149356873433890176;
                }
            } else {
                current_block = 7149356873433890176;
            }
            match current_block {
                6492071313629511791 => {}
                _ => {
                    tmp___2 = buf_get_uint32(&mut buf, &mut replid);
                    if !(tmp___2 == -(1 as libc::c_int)) {
                        if replid != id {
                            fprintf(
                                stderr,
                                b"bad reply ID\n\0" as *const u8 as *const libc::c_char,
                            );
                        } else if type_0 as libc::c_int == 101 as libc::c_int {
                            tmp___3 = buf_get_uint32(&mut buf, &mut serr);
                            if !(tmp___3 == -(1 as libc::c_int)) {
                                fprintf(
                                    stderr,
                                    b"failed to stat home directory (%i)\n\0" as *const u8
                                        as *const libc::c_char,
                                    serr,
                                );
                            }
                        } else {
                            tmp___4 = buf_get_attrs(&mut buf, &mut stbuf, &mut flags);
                            if !(tmp___4 != 0 as libc::c_int) {
                                if !(flags & 2 as libc::c_int == 0) {
                                    sshfs.remote_uid = stbuf.st_uid;
                                    sshfs.local_uid = getuid();
                                    sshfs.remote_gid = stbuf.st_gid;
                                    sshfs.local_gid = getgid();
                                    sshfs.remote_uid_detected = 1 as libc::c_int;
                                    if sshfs.debug != 0 {
                                        fprintf(
                                            stderr,
                                            b"remote_uid = %i\n\0" as *const u8 as *const libc::c_char,
                                            sshfs.remote_uid,
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
    if sshfs.remote_uid_detected == 0 {
        fprintf(
            stderr,
            b"failed to detect remote user ID\n\0" as *const u8 as *const libc::c_char,
        );
    }
    buf_free(&mut buf);
}
unsafe extern "C" fn sftp_check_root(
    mut conn: *mut conn,
    mut base_path: *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut flags: libc::c_int = 0;
    let mut id: uint32_t = 0;
    let mut tmp: uint32_t = 0;
    let mut replid: uint32_t = 0;
    let mut type_0: uint8_t = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut stbuf: stat = stat {
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
    let mut iov: [iovec; 1] = [iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    }; 1];
    let mut err: libc::c_int = 0;
    let mut remote_dir: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut serr: uint32_t = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut err2: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    tmp = sftp_get_id();
    id = tmp;
    err = -(1 as libc::c_int);
    if *base_path.offset(0 as libc::c_int as isize) != 0 {
        tmp___0 = base_path;
    } else {
        tmp___0 = b".\0" as *const u8 as *const libc::c_char;
    }
    remote_dir = tmp___0;
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    buf_add_string(&mut buf, remote_dir);
    buf_to_iov(
        &mut buf as *mut buffer as *const buffer,
        &mut *iov.as_mut_ptr().offset(0 as libc::c_int as isize),
    );
    tmp___1 = sftp_send_iov(
        conn,
        7 as libc::c_int as uint8_t,
        id,
        iov.as_mut_ptr(),
        1 as libc::c_int as size_t,
    );
    if !(tmp___1 == -(1 as libc::c_int)) {
        buf_clear(&mut buf);
        tmp___2 = sftp_read(conn, &mut type_0, &mut buf);
        if !(tmp___2 == -(1 as libc::c_int)) {
            if type_0 as libc::c_int != 105 as libc::c_int {
                if type_0 as libc::c_int != 101 as libc::c_int {
                    fprintf(
                        stderr,
                        b"protocol error\n\0" as *const u8 as *const libc::c_char,
                    );
                    current_block = 6921805039964500870;
                } else {
                    current_block = 10043043949733653460;
                }
            } else {
                current_block = 10043043949733653460;
            }
            match current_block {
                6921805039964500870 => {}
                _ => {
                    tmp___3 = buf_get_uint32(&mut buf, &mut replid);
                    if !(tmp___3 == -(1 as libc::c_int)) {
                        if replid != id {
                            fprintf(
                                stderr,
                                b"bad reply ID\n\0" as *const u8 as *const libc::c_char,
                            );
                        } else if type_0 as libc::c_int == 101 as libc::c_int {
                            tmp___4 = buf_get_uint32(&mut buf, &mut serr);
                            if !(tmp___4 == -(1 as libc::c_int)) {
                                tmp___5 = sftp_error_to_errno(serr);
                                tmp___6 = strerror(tmp___5);
                                fprintf(
                                    stderr,
                                    b"%s:%s: %s\n\0" as *const u8 as *const libc::c_char,
                                    sshfs.host,
                                    remote_dir,
                                    tmp___6,
                                );
                            }
                        } else {
                            tmp___7 = buf_get_attrs(&mut buf, &mut stbuf, &mut flags);
                            err2 = tmp___7;
                            if err2 != 0 {
                                err = err2;
                            } else if !(flags & 4 as libc::c_int == 0) {
                                if !(stbuf.st_mode & 61440 as libc::c_uint
                                    == 16384 as libc::c_uint)
                                {
                                    fprintf(
                                        stderr,
                                        b"%s:%s: Not a directory\n\0" as *const u8
                                            as *const libc::c_char,
                                        sshfs.host,
                                        remote_dir,
                                    );
                                } else {
                                    err = 0 as libc::c_int;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn connect_remote(mut conn: *mut conn) -> libc::c_int {
    let mut err: libc::c_int = 0;
    if sshfs.passive != 0 {
        err = connect_passive(conn);
    } else if !(sshfs.directport).is_null() {
        err = connect_to(conn, sshfs.host, sshfs.directport);
    } else {
        err = start_ssh(conn);
    }
    if err == 0 {
        err = sftp_init(conn);
    }
    if err != 0 {
        close_conn(conn);
    } else {
        sshfs.num_connect = (sshfs.num_connect).wrapping_add(1);
    }
    return err;
}
unsafe extern "C" fn start_processing_thread(mut conn: *mut conn) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut thread_id: pthread_t = 0;
    let mut oldset: sigset_t = sigset_t { __val: [0; 16] };
    let mut newset: sigset_t = sigset_t { __val: [0; 16] };
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*conn).processing_thread_started != 0 {
        return 0 as libc::c_int;
    }
    if (*conn).rfd == -(1 as libc::c_int) {
        err = connect_remote(conn);
        if err != 0 {
            return -(5 as libc::c_int);
        }
    }
    if sshfs.detect_uid != 0 {
        sftp_detect_uid(conn);
        sshfs.detect_uid = 0 as libc::c_int;
    }
    sigemptyset(&mut newset);
    sigaddset(&mut newset, 15 as libc::c_int);
    sigaddset(&mut newset, 2 as libc::c_int);
    sigaddset(&mut newset, 1 as libc::c_int);
    sigaddset(&mut newset, 3 as libc::c_int);
    pthread_sigmask(
        0 as libc::c_int,
        &mut newset as *mut sigset_t as *const __sigset_t,
        &mut oldset as *mut sigset_t as *mut __sigset_t,
    );
    err = pthread_create(
        &mut thread_id as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(
            process_requests
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        conn as *mut libc::c_void,
    );
    if err != 0 {
        tmp = strerror(err);
        fprintf(
            stderr,
            b"failed to create thread: %s\n\0" as *const u8 as *const libc::c_char,
            tmp,
        );
        return -(5 as libc::c_int);
    }
    pthread_detach(thread_id);
    pthread_sigmask(
        2 as libc::c_int,
        &mut oldset as *mut sigset_t as *const __sigset_t,
        0 as *mut libc::c_void as *mut __sigset_t,
    );
    (*conn).processing_thread_started = 1 as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn sshfs_init(
    mut conn: *mut fuse_conn_info,
    mut cfg: *mut fuse_config,
) -> *mut libc::c_void {
    let mut tmp: libc::c_int = 0;
    if (*conn).capable & 1 as libc::c_uint != 0 {
        sshfs.sync_read = 1 as libc::c_int;
    }
    if sshfs.truncate_workaround != 0 {
        tmp = 0 as libc::c_int;
    } else if sshfs.fstat_workaround != 0 {
        tmp = 0 as libc::c_int;
    } else {
        tmp = 1 as libc::c_int;
    }
    (*cfg).nullpath_ok = tmp;
    if sshfs.max_conns > 1 as libc::c_int {
        (*cfg).nullpath_ok = 0 as libc::c_int;
    }
    (*conn).capable |= ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint;
    if sshfs.delay_connect == 0 {
        start_processing_thread((sshfs.conns).offset(0 as libc::c_int as isize));
    }
    (*conn).time_gran = 1000000000 as libc::c_uint;
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn sftp_request_wait(
    mut req: *mut request,
    mut type_0: uint8_t,
    mut expect_type: uint8_t,
    mut outbuf: *mut buffer,
) -> libc::c_int {
    let mut current_block: u64;
    let mut err: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut serr: uint32_t = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    if (*req).error != 0 {
        err = (*req).error;
    } else {
        loop {
            tmp = sem_wait(&mut (*req).ready);
            if tmp == 0 {
                break;
            }
        }
        if (*req).error != 0 {
            err = (*req).error;
        } else {
            err = -(5 as libc::c_int);
            if (*req).reply_type as libc::c_int != expect_type as libc::c_int {
                if (*req).reply_type as libc::c_int != 101 as libc::c_int {
                    fprintf(
                        stderr,
                        b"protocol error\n\0" as *const u8 as *const libc::c_char,
                    );
                    current_block = 8307366857425343120;
                } else {
                    current_block = 4166486009154926805;
                }
            } else {
                current_block = 4166486009154926805;
            }
            match current_block {
                8307366857425343120 => {}
                _ => {
                    if (*req).reply_type as libc::c_int == 101 as libc::c_int {
                        tmp___0 = buf_get_uint32(&mut (*req).reply, &mut serr);
                        if !(tmp___0 == -(1 as libc::c_int)) {
                            match serr {
                                0 => {
                                    if expect_type as libc::c_int == 101 as libc::c_int {
                                        err = 0 as libc::c_int;
                                    } else {
                                        err = -(5 as libc::c_int);
                                    }
                                }
                                1 => {
                                    if type_0 as libc::c_int == 5 as libc::c_int {
                                        err = 1 as libc::c_int;
                                    } else if type_0 as libc::c_int == 12 as libc::c_int {
                                        err = 1 as libc::c_int;
                                    } else {
                                        err = -(5 as libc::c_int);
                                    }
                                }
                                4 => {
                                    if type_0 as libc::c_int == 15 as libc::c_int {
                                        err = -(39 as libc::c_int);
                                    } else {
                                        err = -(1 as libc::c_int);
                                    }
                                }
                                _ => {
                                    tmp___1 = sftp_error_to_errno(serr);
                                    err = -tmp___1;
                                }
                            }
                        }
                    } else {
                        buf_init(
                            outbuf,
                            ((*req).reply.size).wrapping_sub((*req).reply.len),
                        );
                        buf_get_mem(
                            &mut (*req).reply,
                            (*outbuf).p as *mut libc::c_void,
                            (*outbuf).size,
                        );
                        err = 0 as libc::c_int;
                    }
                }
            }
        }
    }
    pthread_mutex_lock(&mut sshfs.lock);
    request_free(req);
    pthread_mutex_unlock(&mut sshfs.lock);
    return err;
}
unsafe extern "C" fn sftp_request_send(
    mut conn: *mut conn,
    mut type_0: uint8_t,
    mut iov: *mut iovec,
    mut count: size_t,
    mut begin_func: Option::<unsafe extern "C" fn(*mut request) -> ()>,
    mut end_func: Option::<unsafe extern "C" fn(*mut request) -> ()>,
    mut want_reply: libc::c_int,
    mut data: *mut libc::c_void,
    mut reqp: *mut *mut request,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut id: uint32_t = 0;
    let mut req: *mut request = 0 as *mut request;
    let mut __n: gsize = 0;
    let mut __s: gsize = 0;
    let mut __p: gpointer = 0 as *mut libc::c_void;
    let mut tmp: size_t = 0;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut rmed: gboolean = 0;
    let mut tmp___1: libc::c_int = 0;
    __n = 1 as libc::c_int as gsize;
    __s = ::std::mem::size_of::<request>() as libc::c_ulong;
    if __s == 1 as libc::c_ulong {
        __p = g_malloc0(__n);
    } else {
        __p = g_malloc0_n(__n, __s);
    }
    req = __p as *mut request;
    (*req).want_reply = want_reply as libc::c_uint;
    (*req).end_func = end_func;
    (*req).data = data;
    sem_init(&mut (*req).ready, 0 as libc::c_int, 0 as libc::c_uint);
    buf_init(&mut (*req).reply, 0 as libc::c_int as size_t);
    pthread_mutex_lock(&mut sshfs.lock);
    if begin_func.is_some() {
        (Some(begin_func.expect("non-null function pointer")))
            .expect("non-null function pointer")(req);
    }
    id = sftp_get_id();
    (*req).id = id;
    (*req).conn = conn;
    (*(*req).conn).req_count += 1;
    err = start_processing_thread(conn);
    if err != 0 {
        pthread_mutex_unlock(&mut sshfs.lock);
    } else {
        tmp = iov_length(iov as *const iovec, count);
        (*req).len = tmp.wrapping_add(9 as libc::c_ulong);
        sshfs
            .outstanding_len = (sshfs.outstanding_len as size_t).wrapping_add((*req).len)
            as libc::c_uint;
        while sshfs.outstanding_len > sshfs.max_outstanding_len {
            pthread_cond_wait(
                &mut sshfs.outstanding_cond as *mut pthread_cond_t,
                &mut sshfs.lock as *mut pthread_mutex_t,
            );
        }
        g_hash_table_insert(sshfs.reqtab, id as gulong as gpointer, req as gpointer);
        if sshfs.debug != 0 {
            gettimeofday(&mut (*req).start as *mut timeval, 0 as *mut libc::c_void);
            sshfs.num_sent = (sshfs.num_sent).wrapping_add(1);
            sshfs
                .bytes_sent = (sshfs.bytes_sent as libc::c_ulong)
                .wrapping_add((*req).len) as uint64_t as uint64_t;
        }
        if sshfs.debug != 0 {
            tmp___0 = type_name(type_0);
            fprintf(
                stderr,
                b"[%05i] %s\n\0" as *const u8 as *const libc::c_char,
                id,
                tmp___0,
            );
        }
        pthread_mutex_unlock(&mut sshfs.lock);
        err = -(5 as libc::c_int);
        tmp___1 = sftp_send_iov(conn, type_0, id, iov, count);
        if tmp___1 == -(1 as libc::c_int) {
            pthread_mutex_lock(&mut sshfs.lock);
            rmed = g_hash_table_remove(
                sshfs.reqtab,
                id as gulong as gpointer as gconstpointer,
            );
            pthread_mutex_unlock(&mut sshfs.lock);
            if rmed == 0 {
                if want_reply == 0 {
                    return err;
                }
            }
        } else {
            if want_reply != 0 {
                *reqp = req;
            }
            return 0 as libc::c_int;
        }
    }
    (*req).error = err;
    if want_reply == 0 {
        sftp_request_wait(
            req,
            type_0,
            0 as libc::c_int as uint8_t,
            0 as *mut libc::c_void as *mut buffer,
        );
    } else {
        *reqp = req;
    }
    return err;
}
unsafe extern "C" fn sftp_request_iov(
    mut conn: *mut conn,
    mut type_0: uint8_t,
    mut iov: *mut iovec,
    mut count: size_t,
    mut expect_type: uint8_t,
    mut outbuf: *mut buffer,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut req: *mut request = 0 as *mut request;
    let mut tmp: libc::c_int = 0;
    err = sftp_request_send(
        conn,
        type_0,
        iov,
        count,
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(*mut request) -> ()>,
        >(0 as *mut libc::c_void),
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(*mut request) -> ()>,
        >(0 as *mut libc::c_void),
        expect_type as libc::c_int,
        0 as *mut libc::c_void,
        &mut req,
    );
    if expect_type as libc::c_int == 0 as libc::c_int {
        return err;
    }
    tmp = sftp_request_wait(req, type_0, expect_type, outbuf);
    return tmp;
}
unsafe extern "C" fn sftp_request(
    mut conn: *mut conn,
    mut type_0: uint8_t,
    mut buf: *const buffer,
    mut expect_type: uint8_t,
    mut outbuf: *mut buffer,
) -> libc::c_int {
    let mut iov: iovec = iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    };
    let mut tmp: libc::c_int = 0;
    buf_to_iov(buf, &mut iov);
    tmp = sftp_request_iov(
        conn,
        type_0,
        &mut iov,
        1 as libc::c_int as size_t,
        expect_type,
        outbuf,
    );
    return tmp;
}
unsafe extern "C" fn sshfs_access(
    mut path: *const libc::c_char,
    mut mask: libc::c_int,
) -> libc::c_int {
    let mut stbuf: stat = stat {
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
    let mut err: libc::c_int = 0;
    err = 0 as libc::c_int;
    if mask & 1 as libc::c_int != 0 {
        err = (Some(((*sshfs.op).getattr).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(path, &mut stbuf, 0 as *mut libc::c_void as *mut fuse_file_info);
        if err == 0 {
            if stbuf.st_mode & 61440 as libc::c_uint == 32768 as libc::c_uint {
                if stbuf.st_mode
                    & (64 as libc::c_int | 64 as libc::c_int >> 3 as libc::c_int
                        | 64 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                        as libc::c_uint == 0
                {
                    err = -(13 as libc::c_int);
                }
            }
        }
    }
    return err;
}
unsafe extern "C" fn count_components(mut p: *const libc::c_char) -> libc::c_int {
    let mut ctr: libc::c_int = 0;
    while *p as libc::c_int == 47 as libc::c_int {
        p = p.offset(1);
    }
    ctr = 0 as libc::c_int;
    while *p != 0 {
        while *p != 0 {
            if !(*p as libc::c_int != 47 as libc::c_int) {
                break;
            }
            p = p.offset(1);
        }
        while *p as libc::c_int == 47 as libc::c_int {
            p = p.offset(1);
        }
        ctr += 1;
    }
    return ctr;
}
unsafe extern "C" fn strip_common(
    mut sp: *mut *const libc::c_char,
    mut tp: *mut *const libc::c_char,
) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut t: *const libc::c_char = 0 as *const libc::c_char;
    s = *sp;
    t = *tp;
    loop {
        while *s as libc::c_int == 47 as libc::c_int {
            s = s.offset(1);
        }
        while *t as libc::c_int == 47 as libc::c_int {
            t = t.offset(1);
        }
        *tp = t;
        *sp = s;
        while *s as libc::c_int == *t as libc::c_int {
            if !(*s != 0) {
                break;
            }
            if !(*s as libc::c_int != 47 as libc::c_int) {
                break;
            }
            s = s.offset(1);
            t = t.offset(1);
        }
        if *s as libc::c_int == *t as libc::c_int {
            if !(*s == 0) {
                continue;
            }
        }
        if *s == 0 {
            if *t as libc::c_int == 47 as libc::c_int {
                continue;
            }
        }
        if !(*s as libc::c_int == 47 as libc::c_int) {
            break;
        }
        if *t != 0 {
            break;
        }
    };
}
unsafe extern "C" fn transform_symlink(
    mut path: *const libc::c_char,
    mut linkp: *mut *mut libc::c_char,
) {
    let mut l: *const libc::c_char = 0 as *const libc::c_char;
    let mut b: *const libc::c_char = 0 as *const libc::c_char;
    let mut newlink: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dotdots: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    l = *linkp as *const libc::c_char;
    b = sshfs.base_path as *const libc::c_char;
    if *l.offset(0 as libc::c_int as isize) as libc::c_int != 47 as libc::c_int {
        return
    } else {
        if *b.offset(0 as libc::c_int as isize) as libc::c_int != 47 as libc::c_int {
            return;
        }
    }
    strip_common(&mut l, &mut b);
    if *b != 0 {
        return;
    }
    strip_common(&mut l, &mut path);
    dotdots = count_components(path);
    if dotdots == 0 {
        return;
    }
    dotdots -= 1;
    tmp = strlen(l);
    tmp___0 = malloc(
        ((dotdots * 3 as libc::c_int) as size_t)
            .wrapping_add(tmp)
            .wrapping_add(2 as libc::c_ulong),
    );
    newlink = tmp___0 as *mut libc::c_char;
    if newlink.is_null() {
        fprintf(
            stderr,
            b"sshfs: memory allocation failed\n\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    s = newlink;
    i = 0 as libc::c_int;
    while i < dotdots {
        strcpy(s, b"../\0" as *const u8 as *const libc::c_char);
        i += 1;
        s = s.offset(3 as libc::c_int as isize);
    }
    if *l.offset(0 as libc::c_int as isize) != 0 {
        strcpy(s, l);
    } else if dotdots == 0 {
        strcpy(s, b".\0" as *const u8 as *const libc::c_char);
    } else {
        *s.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    }
    free(*linkp as *mut libc::c_void);
    *linkp = newlink;
}
unsafe extern "C" fn sshfs_readlink(
    mut path: *const libc::c_char,
    mut linkbuf: *mut libc::c_char,
    mut size: size_t,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut name: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut tmp___0: *mut conn = 0 as *mut conn;
    let mut count: uint32_t = 0;
    let mut link___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    if !(size > 0 as libc::c_ulong) {
        __assert_fail(
            b"size > 0\0" as *const u8 as *const libc::c_char,
            b"../sshfs.c\0" as *const u8 as *const libc::c_char,
            2154 as libc::c_uint,
            b"sshfs_readlink\0" as *const u8 as *const libc::c_char,
        );
    }
    if sshfs.server_version < 3 as libc::c_int {
        return -(1 as libc::c_int);
    }
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    buf_add_path(&mut buf, path);
    tmp___0 = get_conn(
        0 as *mut libc::c_void as *const sshfs_file,
        0 as *mut libc::c_void as *const libc::c_char,
    );
    err = sftp_request(
        tmp___0,
        19 as libc::c_int as uint8_t,
        &mut buf as *mut buffer as *const buffer,
        104 as libc::c_int as uint8_t,
        &mut name,
    );
    if err == 0 {
        err = -(5 as libc::c_int);
        tmp___1 = buf_get_uint32(&mut name, &mut count);
        if tmp___1 != -(1 as libc::c_int) {
            if count == 1 as libc::c_uint {
                tmp___2 = buf_get_string(&mut name, &mut link___0);
                if tmp___2 != -(1 as libc::c_int) {
                    if sshfs.transform_symlinks != 0 {
                        transform_symlink(path, &mut link___0);
                    }
                    strncpy(
                        linkbuf,
                        link___0 as *const libc::c_char,
                        size.wrapping_sub(1 as libc::c_ulong),
                    );
                    *linkbuf
                        .offset(
                            size.wrapping_sub(1 as libc::c_ulong) as isize,
                        ) = '\u{0}' as i32 as libc::c_char;
                    free(link___0 as *mut libc::c_void);
                    err = 0 as libc::c_int;
                }
            }
        }
        buf_free(&mut name);
    }
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sftp_readdir_send(
    mut conn: *mut conn,
    mut req: *mut *mut request,
    mut handle: *mut buffer,
) -> libc::c_int {
    let mut iov: iovec = iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    };
    let mut tmp: libc::c_int = 0;
    buf_to_iov(handle as *const buffer, &mut iov);
    tmp = sftp_request_send(
        conn,
        12 as libc::c_int as uint8_t,
        &mut iov,
        1 as libc::c_int as size_t,
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(*mut request) -> ()>,
        >(0 as *mut libc::c_void),
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(*mut request) -> ()>,
        >(0 as *mut libc::c_void),
        104 as libc::c_int,
        0 as *mut libc::c_void,
        req,
    );
    return tmp;
}
unsafe extern "C" fn sshfs_req_pending(mut req: *mut request) -> libc::c_int {
    let mut tmp: gpointer = 0 as *mut libc::c_void;
    tmp = g_hash_table_lookup(
        sshfs.reqtab,
        (*req).id as gulong as gpointer as gconstpointer,
    );
    if !tmp.is_null() { return 1 as libc::c_int } else { return 0 as libc::c_int };
}
unsafe extern "C" fn sftp_readdir_async(
    mut conn: *mut conn,
    mut handle: *mut buffer,
    mut buf: *mut libc::c_void,
    mut offset: off_t,
    mut filler: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            *const stat,
            off_t,
            fuse_fill_dir_flags,
        ) -> libc::c_int,
    >,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut outstanding: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut list: *mut GList = 0 as *mut GList;
    let mut done: libc::c_int = 0;
    let mut req: *mut request = 0 as *mut request;
    let mut name: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut tmperr: libc::c_int = 0;
    let mut first: *mut GList = 0 as *mut GList;
    let mut want_reply: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    err = 0 as libc::c_int;
    outstanding = 0 as libc::c_int;
    max = 2 as libc::c_int;
    list = 0 as *mut libc::c_void as *mut GList;
    done = 0 as libc::c_int;
    if !(offset == 0 as libc::c_long) {
        __assert_fail(
            b"offset == 0\0" as *const u8 as *const libc::c_char,
            b"../sshfs.c\0" as *const u8 as *const libc::c_char,
            2210 as libc::c_uint,
            b"sftp_readdir_async\0" as *const u8 as *const libc::c_char,
        );
    }
    loop {
        if done != 0 {
            if outstanding == 0 {
                break;
            }
        }
        while done == 0 {
            if !(outstanding < max) {
                break;
            }
            tmperr = sftp_readdir_send(conn, &mut req, handle);
            if tmperr != 0 {
                if done == 0 {
                    err = tmperr;
                    done = 1 as libc::c_int;
                    break;
                }
            }
            list = g_list_append(list, req as gpointer);
            outstanding += 1;
        }
        if !(outstanding != 0) {
            continue;
        }
        first = g_list_first(list);
        req = (*first).data as *mut request;
        list = g_list_delete_link(list, first);
        outstanding -= 1;
        if done != 0 {
            pthread_mutex_lock(&mut sshfs.lock);
            tmp___0 = sshfs_req_pending(req);
            if tmp___0 != 0 {
                (*req).want_reply = 0 as libc::c_uint;
            }
            want_reply = (*req).want_reply as libc::c_int;
            pthread_mutex_unlock(&mut sshfs.lock);
            if want_reply == 0 {
                continue;
            }
        }
        tmperr = sftp_request_wait(
            req,
            12 as libc::c_int as uint8_t,
            104 as libc::c_int as uint8_t,
            &mut name,
        );
        if tmperr != 0 {
            if done == 0 {
                err = tmperr;
                if err == 1 as libc::c_int {
                    err = 0 as libc::c_int;
                }
                done = 1 as libc::c_int;
            }
        }
        if done == 0 {
            err = buf_get_entries(&mut name, buf, filler);
            buf_free(&mut name);
            if max < 32 as libc::c_int {
                max += 1;
            }
            if err != 0 {
                done = 1 as libc::c_int;
            }
        }
    }
    if !(list as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"list == NULL\0" as *const u8 as *const libc::c_char,
            b"../sshfs.c\0" as *const u8 as *const libc::c_char,
            2273 as libc::c_uint,
            b"sftp_readdir_async\0" as *const u8 as *const libc::c_char,
        );
    }
    return err;
}
unsafe extern "C" fn sftp_readdir_sync(
    mut conn: *mut conn,
    mut handle: *mut buffer,
    mut buf: *mut libc::c_void,
    mut offset: off_t,
    mut filler: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            *const stat,
            off_t,
            fuse_fill_dir_flags,
        ) -> libc::c_int,
    >,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut name: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    if !(offset == 0 as libc::c_long) {
        __assert_fail(
            b"offset == 0\0" as *const u8 as *const libc::c_char,
            b"../sshfs.c\0" as *const u8 as *const libc::c_char,
            2282 as libc::c_uint,
            b"sftp_readdir_sync\0" as *const u8 as *const libc::c_char,
        );
    }
    loop {
        err = sftp_request(
            conn,
            12 as libc::c_int as uint8_t,
            handle as *const buffer,
            104 as libc::c_int as uint8_t,
            &mut name,
        );
        if err == 0 {
            err = buf_get_entries(&mut name, buf, filler);
            buf_free(&mut name);
        }
        if err != 0 {
            break;
        }
    }
    if err == 1 as libc::c_int {
        err = 0 as libc::c_int;
    }
    return err;
}
unsafe extern "C" fn sshfs_opendir(
    mut path: *const libc::c_char,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut conn: *mut conn = 0 as *mut conn;
    let mut buf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut handle: *mut dir_handle = 0 as *mut dir_handle;
    let mut __n: gsize = 0;
    let mut __s: gsize = 0;
    let mut __p: gpointer = 0 as *mut libc::c_void;
    __n = 1 as libc::c_int as gsize;
    __s = ::std::mem::size_of::<dir_handle>() as libc::c_ulong;
    if __s == 1 as libc::c_ulong {
        __p = g_malloc0(__n);
    } else {
        __p = g_malloc0_n(__n, __s);
    }
    handle = __p as *mut dir_handle;
    if handle as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(12 as libc::c_int);
    }
    conn = get_conn(
        0 as *mut libc::c_void as *const sshfs_file,
        0 as *mut libc::c_void as *const libc::c_char,
    );
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    buf_add_path(&mut buf, path);
    err = sftp_request(
        conn,
        11 as libc::c_int as uint8_t,
        &mut buf as *mut buffer as *const buffer,
        102 as libc::c_int as uint8_t,
        &mut (*handle).buf,
    );
    if err == 0 {
        buf_finish(&mut (*handle).buf);
        pthread_mutex_lock(&mut sshfs.lock);
        (*handle).conn = conn;
        (*(*handle).conn).dir_count += 1;
        pthread_mutex_unlock(&mut sshfs.lock);
        (*fi).fh = handle as libc::c_ulong;
    } else {
        g_free(handle as gpointer);
    }
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sshfs_readdir(
    mut path: *const libc::c_char,
    mut dbuf: *mut libc::c_void,
    mut filler: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            *const stat,
            off_t,
            fuse_fill_dir_flags,
        ) -> libc::c_int,
    >,
    mut offset: off_t,
    mut fi: *mut fuse_file_info,
    mut flags: fuse_readdir_flags,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut handle: *mut dir_handle = 0 as *mut dir_handle;
    handle = (*fi).fh as *mut dir_handle;
    if sshfs.sync_readdir != 0 {
        err = sftp_readdir_sync(
            (*handle).conn,
            &mut (*handle).buf,
            dbuf,
            offset,
            filler,
        );
    } else {
        err = sftp_readdir_async(
            (*handle).conn,
            &mut (*handle).buf,
            dbuf,
            offset,
            filler,
        );
    }
    return err;
}
unsafe extern "C" fn sshfs_releasedir(
    mut path: *const libc::c_char,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut handle: *mut dir_handle = 0 as *mut dir_handle;
    handle = (*fi).fh as *mut dir_handle;
    err = sftp_request(
        (*handle).conn,
        4 as libc::c_int as uint8_t,
        &mut (*handle).buf as *mut buffer as *const buffer,
        0 as libc::c_int as uint8_t,
        0 as *mut libc::c_void as *mut buffer,
    );
    pthread_mutex_lock(&mut sshfs.lock);
    (*(*handle).conn).dir_count -= 1;
    pthread_mutex_unlock(&mut sshfs.lock);
    buf_free(&mut (*handle).buf);
    g_free(handle as gpointer);
    return err;
}
unsafe extern "C" fn sshfs_mkdir(
    mut path: *const libc::c_char,
    mut mode: mode_t,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut tmp: *mut conn = 0 as *mut conn;
    let mut tmp___0: libc::c_int = 0;
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    buf_add_path(&mut buf, path);
    buf_add_uint32(&mut buf, 4 as libc::c_int as uint32_t);
    buf_add_uint32(&mut buf, mode);
    tmp = get_conn(
        0 as *mut libc::c_void as *const sshfs_file,
        0 as *mut libc::c_void as *const libc::c_char,
    );
    err = sftp_request(
        tmp,
        14 as libc::c_int as uint8_t,
        &mut buf as *mut buffer as *const buffer,
        101 as libc::c_int as uint8_t,
        0 as *mut libc::c_void as *mut buffer,
    );
    buf_free(&mut buf);
    if err == -(1 as libc::c_int) {
        tmp___0 = (Some(((*sshfs.op).access).expect("non-null function pointer")))
            .expect("non-null function pointer")(path, 4 as libc::c_int);
        if tmp___0 == 0 as libc::c_int {
            return -(17 as libc::c_int);
        }
    }
    return err;
}
unsafe extern "C" fn sshfs_mknod(
    mut path: *const libc::c_char,
    mut mode: mode_t,
    mut rdev: dev_t,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut conn: *mut conn = 0 as *mut conn;
    let mut buf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut handle: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut err2: libc::c_int = 0;
    if mode & 61440 as libc::c_uint != 32768 as libc::c_uint {
        return -(1 as libc::c_int);
    }
    conn = get_conn(
        0 as *mut libc::c_void as *const sshfs_file,
        0 as *mut libc::c_void as *const libc::c_char,
    );
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    buf_add_path(&mut buf, path);
    buf_add_uint32(&mut buf, 42 as libc::c_int as uint32_t);
    buf_add_uint32(&mut buf, 4 as libc::c_int as uint32_t);
    buf_add_uint32(&mut buf, mode);
    err = sftp_request(
        conn,
        3 as libc::c_int as uint8_t,
        &mut buf as *mut buffer as *const buffer,
        102 as libc::c_int as uint8_t,
        &mut handle,
    );
    if err == 0 {
        buf_finish(&mut handle);
        err2 = sftp_request(
            conn,
            4 as libc::c_int as uint8_t,
            &mut handle as *mut buffer as *const buffer,
            101 as libc::c_int as uint8_t,
            0 as *mut libc::c_void as *mut buffer,
        );
        if err == 0 {
            err = err2;
        }
        buf_free(&mut handle);
    }
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sshfs_symlink(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut tmp: *mut conn = 0 as *mut conn;
    if sshfs.server_version < 3 as libc::c_int {
        return -(1 as libc::c_int);
    }
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    buf_add_string(&mut buf, from);
    buf_add_path(&mut buf, to);
    tmp = get_conn(
        0 as *mut libc::c_void as *const sshfs_file,
        0 as *mut libc::c_void as *const libc::c_char,
    );
    err = sftp_request(
        tmp,
        20 as libc::c_int as uint8_t,
        &mut buf as *mut buffer as *const buffer,
        101 as libc::c_int as uint8_t,
        0 as *mut libc::c_void as *mut buffer,
    );
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sshfs_unlink(mut path: *const libc::c_char) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut tmp: *mut conn = 0 as *mut conn;
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    buf_add_path(&mut buf, path);
    tmp = get_conn(
        0 as *mut libc::c_void as *const sshfs_file,
        0 as *mut libc::c_void as *const libc::c_char,
    );
    err = sftp_request(
        tmp,
        13 as libc::c_int as uint8_t,
        &mut buf as *mut buffer as *const buffer,
        101 as libc::c_int as uint8_t,
        0 as *mut libc::c_void as *mut buffer,
    );
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sshfs_rmdir(mut path: *const libc::c_char) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut tmp: *mut conn = 0 as *mut conn;
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    buf_add_path(&mut buf, path);
    tmp = get_conn(
        0 as *mut libc::c_void as *const sshfs_file,
        0 as *mut libc::c_void as *const libc::c_char,
    );
    err = sftp_request(
        tmp,
        15 as libc::c_int as uint8_t,
        &mut buf as *mut buffer as *const buffer,
        101 as libc::c_int as uint8_t,
        0 as *mut libc::c_void as *mut buffer,
    );
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sshfs_do_rename(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut tmp: *mut conn = 0 as *mut conn;
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    buf_add_path(&mut buf, from);
    buf_add_path(&mut buf, to);
    tmp = get_conn(
        0 as *mut libc::c_void as *const sshfs_file,
        0 as *mut libc::c_void as *const libc::c_char,
    );
    err = sftp_request(
        tmp,
        18 as libc::c_int as uint8_t,
        &mut buf as *mut buffer as *const buffer,
        101 as libc::c_int as uint8_t,
        0 as *mut libc::c_void as *mut buffer,
    );
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sshfs_ext_posix_rename(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut tmp: *mut conn = 0 as *mut conn;
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    buf_add_string(
        &mut buf,
        b"posix-rename@openssh.com\0" as *const u8 as *const libc::c_char,
    );
    buf_add_path(&mut buf, from);
    buf_add_path(&mut buf, to);
    tmp = get_conn(
        0 as *mut libc::c_void as *const sshfs_file,
        0 as *mut libc::c_void as *const libc::c_char,
    );
    err = sftp_request(
        tmp,
        200 as libc::c_int as uint8_t,
        &mut buf as *mut buffer as *const buffer,
        101 as libc::c_int as uint8_t,
        0 as *mut libc::c_void as *mut buffer,
    );
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn random_string(mut str: *mut libc::c_char, mut length: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < length {
        tmp = str;
        str = str.offset(1);
        tmp___0 = rand_r(&mut sshfs.randseed);
        *tmp = (48 as libc::c_int + tmp___0 % 10 as libc::c_int) as libc::c_char;
        i += 1;
    }
    *str = '\u{0}' as i32 as libc::c_char;
}
unsafe extern "C" fn sshfs_rename(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
    mut flags: libc::c_uint,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut ce: *mut conntab_entry = 0 as *mut conntab_entry;
    let mut tolen: size_t = 0;
    let mut tmp: size_t = 0;
    let mut tmperr: libc::c_int = 0;
    let mut totmp: [libc::c_char; 4096] = [0; 4096];
    let mut tmp___0: gpointer = 0 as *mut libc::c_void;
    let mut tmp___1: *mut gchar = 0 as *mut gchar;
    if flags != 0 as libc::c_uint {
        return -(22 as libc::c_int);
    }
    if sshfs.ext_posix_rename != 0 {
        err = sshfs_ext_posix_rename(from, to);
    } else {
        err = sshfs_do_rename(from, to);
    }
    if err == -(1 as libc::c_int) {
        if sshfs.rename_workaround != 0 {
            tmp = strlen(to);
            tolen = tmp;
            if tolen.wrapping_add(8 as libc::c_ulong) < 4096 as libc::c_ulong {
                strcpy(totmp.as_mut_ptr(), to);
                random_string(
                    totmp.as_mut_ptr().offset(tolen as isize),
                    8 as libc::c_int,
                );
                tmperr = sshfs_do_rename(to, totmp.as_mut_ptr() as *const libc::c_char);
                if tmperr == 0 {
                    err = sshfs_do_rename(from, to);
                    if err == 0 {
                        err = sshfs_unlink(totmp.as_mut_ptr() as *const libc::c_char);
                    } else {
                        sshfs_do_rename(totmp.as_mut_ptr() as *const libc::c_char, to);
                    }
                }
            }
        }
    }
    if err == -(1 as libc::c_int) {
        if sshfs.renamexdev_workaround != 0 {
            err = -(18 as libc::c_int);
        }
    }
    if err == 0 {
        if sshfs.max_conns > 1 as libc::c_int {
            pthread_mutex_lock(&mut sshfs.lock);
            tmp___0 = g_hash_table_lookup(sshfs.conntab, from as gconstpointer);
            ce = tmp___0 as *mut conntab_entry;
            if ce as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                tmp___1 = g_strdup(to);
                g_hash_table_replace(sshfs.conntab, tmp___1 as gpointer, ce as gpointer);
                g_hash_table_remove(sshfs.conntab, from as gconstpointer);
            }
            pthread_mutex_unlock(&mut sshfs.lock);
        }
    }
    return err;
}
unsafe extern "C" fn sshfs_link(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut tmp: *mut conn = 0 as *mut conn;
    err = -(38 as libc::c_int);
    if sshfs.ext_hardlink != 0 {
        if sshfs.disable_hardlink == 0 {
            buf_init(&mut buf, 0 as libc::c_int as size_t);
            buf_add_string(
                &mut buf,
                b"hardlink@openssh.com\0" as *const u8 as *const libc::c_char,
            );
            buf_add_path(&mut buf, from);
            buf_add_path(&mut buf, to);
            tmp = get_conn(
                0 as *mut libc::c_void as *const sshfs_file,
                0 as *mut libc::c_void as *const libc::c_char,
            );
            err = sftp_request(
                tmp,
                200 as libc::c_int as uint8_t,
                &mut buf as *mut buffer as *const buffer,
                101 as libc::c_int as uint8_t,
                0 as *mut libc::c_void as *mut buffer,
            );
            buf_free(&mut buf);
        }
    }
    return err;
}
#[inline]
unsafe extern "C" fn sshfs_file_is_conn(mut sf: *mut sshfs_file) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    pthread_mutex_lock(&mut sshfs.lock);
    ret = ((*sf).connver == (*(*sf).conn).connver) as libc::c_int;
    pthread_mutex_unlock(&mut sshfs.lock);
    return ret;
}
#[inline]
unsafe extern "C" fn get_sshfs_file(mut fi: *mut fuse_file_info) -> *mut sshfs_file {
    return (*fi).fh as *mut sshfs_file;
}
unsafe extern "C" fn sshfs_chmod(
    mut path: *const libc::c_char,
    mut mode: mode_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut sf: *mut sshfs_file = 0 as *mut sshfs_file;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut conn = 0 as *mut conn;
    sf = 0 as *mut libc::c_void as *mut sshfs_file;
    if fi as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        sf = get_sshfs_file(fi);
        tmp = sshfs_file_is_conn(sf);
        if tmp == 0 {
            return -(5 as libc::c_int);
        }
    }
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    if sf as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        buf_add_path(&mut buf, path);
    } else {
        buf_add_buf(&mut buf, &mut (*sf).handle as *mut buffer as *const buffer);
    }
    buf_add_uint32(&mut buf, 4 as libc::c_int as uint32_t);
    buf_add_uint32(&mut buf, mode);
    if sf as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___0 = 9 as libc::c_int;
    } else {
        tmp___0 = 10 as libc::c_int;
    }
    tmp___1 = get_conn(
        sf as *const sshfs_file,
        0 as *mut libc::c_void as *const libc::c_char,
    );
    err = sftp_request(
        tmp___1,
        tmp___0 as uint8_t,
        &mut buf as *mut buffer as *const buffer,
        101 as libc::c_int as uint8_t,
        0 as *mut libc::c_void as *mut buffer,
    );
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sshfs_chown(
    mut path: *const libc::c_char,
    mut uid: uid_t,
    mut gid: gid_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut sf: *mut sshfs_file = 0 as *mut sshfs_file;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: *mut conn = 0 as *mut conn;
    sf = 0 as *mut libc::c_void as *mut sshfs_file;
    if fi as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        sf = get_sshfs_file(fi);
        tmp = sshfs_file_is_conn(sf);
        if tmp == 0 {
            return -(5 as libc::c_int);
        }
    }
    if sshfs.remote_uid_detected != 0 {
        if uid == sshfs.local_uid {
            uid = sshfs.remote_uid;
        }
        if gid == sshfs.local_gid {
            gid = sshfs.remote_gid;
        }
    }
    if sshfs.idmap == 2 as libc::c_int {
        if !(sshfs.r_uid_map).is_null() {
            tmp___0 = translate_id(&mut uid, sshfs.r_uid_map);
            if tmp___0 == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
        }
    }
    if sshfs.idmap == 2 as libc::c_int {
        if !(sshfs.r_gid_map).is_null() {
            tmp___1 = translate_id(&mut gid, sshfs.r_gid_map);
            if tmp___1 == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
        }
    }
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    if sf as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        buf_add_path(&mut buf, path);
    } else {
        buf_add_buf(&mut buf, &mut (*sf).handle as *mut buffer as *const buffer);
    }
    buf_add_uint32(&mut buf, 2 as libc::c_int as uint32_t);
    buf_add_uint32(&mut buf, uid);
    buf_add_uint32(&mut buf, gid);
    if sf as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___2 = 9 as libc::c_int;
    } else {
        tmp___2 = 10 as libc::c_int;
    }
    tmp___3 = get_conn(
        sf as *const sshfs_file,
        0 as *mut libc::c_void as *const libc::c_char,
    );
    err = sftp_request(
        tmp___3,
        tmp___2 as uint8_t,
        &mut buf as *mut buffer as *const buffer,
        101 as libc::c_int as uint8_t,
        0 as *mut libc::c_void as *mut buffer,
    );
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sshfs_inc_modifver() {
    pthread_mutex_lock(&mut sshfs.lock);
    sshfs.modifver += 1;
    pthread_mutex_unlock(&mut sshfs.lock);
}
unsafe extern "C" fn sshfs_utimens(
    mut path: *const libc::c_char,
    mut tv: *const timespec,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut sf: *mut sshfs_file = 0 as *mut sshfs_file;
    let mut asec: time_t = 0;
    let mut msec: time_t = 0;
    let mut now: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut conn = 0 as *mut conn;
    sf = 0 as *mut libc::c_void as *mut sshfs_file;
    asec = (*tv.offset(0 as libc::c_int as isize)).tv_sec;
    msec = (*tv.offset(1 as libc::c_int as isize)).tv_sec;
    gettimeofday(&mut now as *mut timeval, 0 as *mut libc::c_void);
    if asec == 0 as libc::c_long {
        asec = now.tv_sec;
    }
    if msec == 0 as libc::c_long {
        msec = now.tv_sec;
    }
    if fi as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        sf = get_sshfs_file(fi);
        tmp = sshfs_file_is_conn(sf);
        if tmp == 0 {
            return -(5 as libc::c_int);
        }
    }
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    if sf as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        buf_add_path(&mut buf, path);
    } else {
        buf_add_buf(&mut buf, &mut (*sf).handle as *mut buffer as *const buffer);
    }
    buf_add_uint32(&mut buf, 8 as libc::c_int as uint32_t);
    buf_add_uint32(&mut buf, asec as uint32_t);
    buf_add_uint32(&mut buf, msec as uint32_t);
    if sf as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___0 = 9 as libc::c_int;
    } else {
        tmp___0 = 10 as libc::c_int;
    }
    tmp___1 = get_conn(sf as *const sshfs_file, path);
    err = sftp_request(
        tmp___1,
        tmp___0 as uint8_t,
        &mut buf as *mut buffer as *const buffer,
        101 as libc::c_int as uint8_t,
        0 as *mut libc::c_void as *mut buffer,
    );
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sshfs_open_common(
    mut path: *const libc::c_char,
    mut mode: mode_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut err2: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut outbuf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut stbuf: stat = stat {
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
    let mut sf: *mut sshfs_file = 0 as *mut sshfs_file;
    let mut open_req: *mut request = 0 as *mut request;
    let mut ce: *mut conntab_entry = 0 as *mut conntab_entry;
    let mut pflags: uint32_t = 0;
    let mut iov: iovec = iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    };
    let mut type_0: uint8_t = 0;
    let mut wrctr: uint64_t = 0;
    let mut __n: gsize = 0;
    let mut __s: gsize = 0;
    let mut __p: gpointer = 0 as *mut libc::c_void;
    let mut tmp: gpointer = 0 as *mut libc::c_void;
    let mut tmp___0: gpointer = 0 as *mut libc::c_void;
    let mut tmp___1: *mut gchar = 0 as *mut gchar;
    pflags = 0 as libc::c_int as uint32_t;
    wrctr = 0 as libc::c_int as uint64_t;
    if sshfs.dir_cache != 0 {
        wrctr = cache_get_write_ctr();
    }
    if sshfs.direct_io != 0 {
        (*fi).set_direct_io(1 as libc::c_uint);
    }
    if (*fi).flags & 3 as libc::c_int == 0 as libc::c_int {
        pflags = 1 as libc::c_int as uint32_t;
    } else if (*fi).flags & 3 as libc::c_int == 1 as libc::c_int {
        pflags = 2 as libc::c_int as uint32_t;
    } else if (*fi).flags & 3 as libc::c_int == 2 as libc::c_int {
        pflags = 3 as libc::c_int as uint32_t;
    } else {
        return -(22 as libc::c_int)
    }
    if (*fi).flags & 64 as libc::c_int != 0 {
        pflags |= 8 as libc::c_uint;
    }
    if (*fi).flags & 128 as libc::c_int != 0 {
        pflags |= 32 as libc::c_uint;
    }
    if (*fi).flags & 512 as libc::c_int != 0 {
        pflags |= 16 as libc::c_uint;
    }
    if (*fi).flags & 1024 as libc::c_int != 0 {
        pflags |= 4 as libc::c_uint;
    }
    __n = 1 as libc::c_int as gsize;
    __s = ::std::mem::size_of::<sshfs_file>() as libc::c_ulong;
    if __s == 1 as libc::c_ulong {
        __p = g_malloc0(__n);
    } else {
        __p = g_malloc0_n(__n, __s);
    }
    sf = __p as *mut sshfs_file;
    list_init(&mut (*sf).write_reqs);
    pthread_cond_init(
        &mut (*sf).write_finished as *mut pthread_cond_t,
        0 as *mut libc::c_void as *const pthread_condattr_t,
    );
    (*sf).is_seq = 0 as libc::c_int;
    (*sf).next_pos = 0 as libc::c_int as off_t;
    pthread_mutex_lock(&mut sshfs.lock);
    (*sf).modifver = sshfs.modifver as libc::c_int;
    if sshfs.max_conns > 1 as libc::c_int {
        tmp = g_hash_table_lookup(sshfs.conntab, path as gconstpointer);
        ce = tmp as *mut conntab_entry;
        if ce.is_null() {
            tmp___0 = g_malloc(::std::mem::size_of::<conntab_entry>() as libc::c_ulong);
            ce = tmp___0 as *mut conntab_entry;
            (*ce).refcount = 0 as libc::c_uint;
            (*ce)
                .conn = get_conn(
                0 as *mut libc::c_void as *const sshfs_file,
                0 as *mut libc::c_void as *const libc::c_char,
            );
            tmp___1 = g_strdup(path);
            g_hash_table_insert(sshfs.conntab, tmp___1 as gpointer, ce as gpointer);
        }
        (*sf).conn = (*ce).conn;
        (*ce).refcount = ((*ce).refcount).wrapping_add(1);
        (*(*sf).conn).file_count += 1;
        if !((*(*sf).conn).file_count > 0 as libc::c_int) {
            __assert_fail(
                b"sf->conn->file_count > 0\0" as *const u8 as *const libc::c_char,
                b"../sshfs.c\0" as *const u8 as *const libc::c_char,
                2763 as libc::c_uint,
                b"sshfs_open_common\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        (*sf).conn = (sshfs.conns).offset(0 as libc::c_int as isize);
        ce = 0 as *mut libc::c_void as *mut conntab_entry;
    }
    (*sf).connver = (*(*sf).conn).connver;
    pthread_mutex_unlock(&mut sshfs.lock);
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    buf_add_path(&mut buf, path);
    buf_add_uint32(&mut buf, pflags);
    buf_add_uint32(&mut buf, 4 as libc::c_int as uint32_t);
    buf_add_uint32(&mut buf, mode);
    buf_to_iov(&mut buf as *mut buffer as *const buffer, &mut iov);
    sftp_request_send(
        (*sf).conn,
        3 as libc::c_int as uint8_t,
        &mut iov,
        1 as libc::c_int as size_t,
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(*mut request) -> ()>,
        >(0 as *mut libc::c_void),
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(*mut request) -> ()>,
        >(0 as *mut libc::c_void),
        1 as libc::c_int,
        0 as *mut libc::c_void,
        &mut open_req,
    );
    buf_clear(&mut buf);
    buf_add_path(&mut buf, path);
    if sshfs.follow_symlinks != 0 {
        type_0 = 17 as libc::c_int as uint8_t;
    } else {
        type_0 = 7 as libc::c_int as uint8_t;
    }
    err2 = sftp_request(
        (*sf).conn,
        type_0,
        &mut buf as *mut buffer as *const buffer,
        105 as libc::c_int as uint8_t,
        &mut outbuf,
    );
    if err2 == 0 {
        err2 = buf_get_attrs(
            &mut outbuf,
            &mut stbuf,
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        buf_free(&mut outbuf);
    }
    err = sftp_request_wait(
        open_req,
        3 as libc::c_int as uint8_t,
        102 as libc::c_int as uint8_t,
        &mut (*sf).handle,
    );
    if err == 0 {
        if err2 != 0 {
            buf_finish(&mut (*sf).handle);
            sftp_request(
                (*sf).conn,
                4 as libc::c_int as uint8_t,
                &mut (*sf).handle as *mut buffer as *const buffer,
                0 as libc::c_int as uint8_t,
                0 as *mut libc::c_void as *mut buffer,
            );
            buf_free(&mut (*sf).handle);
            err = err2;
        }
    }
    if err == 0 {
        if sshfs.dir_cache != 0 {
            cache_add_attr(path, &mut stbuf as *mut stat as *const stat, wrctr);
        }
        buf_finish(&mut (*sf).handle);
        (*fi).fh = sf as libc::c_ulong;
    } else {
        if sshfs.dir_cache != 0 {
            cache_invalidate(path);
        }
        if sshfs.max_conns > 1 as libc::c_int {
            pthread_mutex_lock(&mut sshfs.lock);
            (*(*sf).conn).file_count -= 1;
            (*ce).refcount = ((*ce).refcount).wrapping_sub(1);
            if (*ce).refcount == 0 as libc::c_uint {
                g_hash_table_remove(sshfs.conntab, path as gconstpointer);
                g_free(ce as gpointer);
            }
            pthread_mutex_unlock(&mut sshfs.lock);
        }
        g_free(sf as gpointer);
    }
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sshfs_open(
    mut path: *const libc::c_char,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = sshfs_open_common(path, 0 as libc::c_int as mode_t, fi);
    return tmp;
}
unsafe extern "C" fn sshfs_flush(
    mut path: *const libc::c_char,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut sf: *mut sshfs_file = 0 as *mut sshfs_file;
    let mut tmp: *mut sshfs_file = 0 as *mut sshfs_file;
    let mut write_reqs: list_head = list_head {
        prev: 0 as *mut list_head,
        next: 0 as *mut list_head,
    };
    let mut curr_list: *mut list_head = 0 as *mut list_head;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    tmp = get_sshfs_file(fi);
    sf = tmp;
    tmp___0 = sshfs_file_is_conn(sf);
    if tmp___0 == 0 {
        return -(5 as libc::c_int);
    }
    if sshfs.sync_write != 0 {
        return 0 as libc::c_int;
    }
    pthread_mutex_lock(&mut sshfs.lock);
    tmp___2 = list_empty(&mut (*sf).write_reqs as *mut list_head as *const list_head);
    if tmp___2 == 0 {
        curr_list = (*sf).write_reqs.prev;
        list_del(&mut (*sf).write_reqs);
        list_init(&mut (*sf).write_reqs);
        list_add(&mut write_reqs, curr_list);
        loop {
            tmp___1 = list_empty(&mut write_reqs as *mut list_head as *const list_head);
            if tmp___1 != 0 {
                break;
            }
            pthread_cond_wait(
                &mut (*sf).write_finished as *mut pthread_cond_t,
                &mut sshfs.lock as *mut pthread_mutex_t,
            );
        }
    }
    err = (*sf).write_error;
    (*sf).write_error = 0 as libc::c_int;
    pthread_mutex_unlock(&mut sshfs.lock);
    return err;
}
unsafe extern "C" fn sshfs_fsync(
    mut path: *const libc::c_char,
    mut isdatasync: libc::c_int,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut sf: *mut sshfs_file = 0 as *mut sshfs_file;
    let mut tmp: *mut sshfs_file = 0 as *mut sshfs_file;
    err = sshfs_flush(path, fi);
    if err != 0 {
        return err;
    }
    if sshfs.ext_fsync == 0 {
        return err;
    }
    tmp = get_sshfs_file(fi);
    sf = tmp;
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    buf_add_string(&mut buf, b"fsync@openssh.com\0" as *const u8 as *const libc::c_char);
    buf_add_buf(&mut buf, &mut (*sf).handle as *mut buffer as *const buffer);
    err = sftp_request(
        (*sf).conn,
        200 as libc::c_int as uint8_t,
        &mut buf as *mut buffer as *const buffer,
        101 as libc::c_int as uint8_t,
        0 as *mut libc::c_void as *mut buffer,
    );
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sshfs_release(
    mut path: *const libc::c_char,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut sf: *mut sshfs_file = 0 as *mut sshfs_file;
    let mut tmp: *mut sshfs_file = 0 as *mut sshfs_file;
    let mut handle: *mut buffer = 0 as *mut buffer;
    let mut ce: *mut conntab_entry = 0 as *mut conntab_entry;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: gpointer = 0 as *mut libc::c_void;
    tmp = get_sshfs_file(fi);
    sf = tmp;
    handle = &mut (*sf).handle;
    tmp___0 = sshfs_file_is_conn(sf);
    if tmp___0 != 0 {
        sshfs_flush(path, fi);
        sftp_request(
            (*sf).conn,
            4 as libc::c_int as uint8_t,
            handle as *const buffer,
            0 as libc::c_int as uint8_t,
            0 as *mut libc::c_void as *mut buffer,
        );
    }
    buf_free(handle);
    chunk_put_locked((*sf).readahead);
    if sshfs.max_conns > 1 as libc::c_int {
        pthread_mutex_lock(&mut sshfs.lock);
        (*(*sf).conn).file_count -= 1;
        tmp___1 = g_hash_table_lookup(sshfs.conntab, path as gconstpointer);
        ce = tmp___1 as *mut conntab_entry;
        (*ce).refcount = ((*ce).refcount).wrapping_sub(1);
        if (*ce).refcount == 0 as libc::c_uint {
            g_hash_table_remove(sshfs.conntab, path as gconstpointer);
            g_free(ce as gpointer);
        }
        pthread_mutex_unlock(&mut sshfs.lock);
    }
    g_free(sf as gpointer);
    return 0 as libc::c_int;
}
unsafe extern "C" fn sshfs_read_end(mut req: *mut request) {
    let mut rreq: *mut read_req = 0 as *mut read_req;
    let mut serr: uint32_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut retsize: uint32_t = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    rreq = (*req).data as *mut read_req;
    if (*req).error != 0 {
        (*rreq).res = (*req).error as ssize_t;
    } else if (*req).replied != 0 {
        (*rreq).res = -(5 as libc::c_int) as ssize_t;
        if (*req).reply_type as libc::c_int == 101 as libc::c_int {
            tmp___0 = buf_get_uint32(&mut (*req).reply, &mut serr);
            if tmp___0 != -(1 as libc::c_int) {
                if serr == 1 as libc::c_uint {
                    (*rreq).res = 0 as libc::c_int as ssize_t;
                } else {
                    tmp = sftp_error_to_errno(serr);
                    (*rreq).res = -tmp as ssize_t;
                }
            }
        } else if (*req).reply_type as libc::c_int == 103 as libc::c_int {
            tmp___2 = buf_get_uint32(&mut (*req).reply, &mut retsize);
            if tmp___2 != -(1 as libc::c_int) {
                if retsize as size_t > (*rreq).size {
                    fprintf(
                        stderr,
                        b"long read\n\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    tmp___1 = buf_check_get(&mut (*req).reply, retsize as size_t);
                    if tmp___1 != -(1 as libc::c_int) {
                        (*rreq).res = retsize as ssize_t;
                        (*rreq).data = (*req).reply;
                        buf_init(&mut (*req).reply, 0 as libc::c_int as size_t);
                    }
                }
            }
        } else {
            fprintf(stderr, b"protocol error\n\0" as *const u8 as *const libc::c_char);
        }
    } else {
        (*rreq).res = -(5 as libc::c_int) as ssize_t;
    }
    (*(*rreq).sio).num_reqs -= 1;
    if (*(*rreq).sio).num_reqs == 0 {
        pthread_cond_broadcast(&mut (*(*rreq).sio).finished);
    }
}
unsafe extern "C" fn sshfs_read_begin(mut req: *mut request) {
    let mut rreq: *mut read_req = 0 as *mut read_req;
    rreq = (*req).data as *mut read_req;
    (*(*rreq).sio).num_reqs += 1;
}
unsafe extern "C" fn sshfs_send_read(
    mut sf: *mut sshfs_file,
    mut size: size_t,
    mut offset: off_t,
) -> *mut read_chunk {
    let mut chunk: *mut read_chunk = 0 as *mut read_chunk;
    let mut __n: gsize = 0;
    let mut __s: gsize = 0;
    let mut __p: gpointer = 0 as *mut libc::c_void;
    let mut handle: *mut buffer = 0 as *mut buffer;
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut iov: [iovec; 1] = [iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    }; 1];
    let mut rreq: *mut read_req = 0 as *mut read_req;
    let mut bsize: size_t = 0;
    let mut tmp: size_t = 0;
    let mut __n___0: gsize = 0;
    let mut __s___0: gsize = 0;
    let mut __p___0: gpointer = 0 as *mut libc::c_void;
    __n = 1 as libc::c_int as gsize;
    __s = ::std::mem::size_of::<read_chunk>() as libc::c_ulong;
    if __s == 1 as libc::c_ulong {
        __p = g_malloc0(__n);
    } else {
        __p = g_malloc0_n(__n, __s);
    }
    chunk = __p as *mut read_chunk;
    handle = &mut (*sf).handle;
    pthread_cond_init(
        &mut (*chunk).sio.finished as *mut pthread_cond_t,
        0 as *mut libc::c_void as *const pthread_condattr_t,
    );
    list_init(&mut (*chunk).reqs);
    (*chunk).size = size;
    (*chunk).offset = offset;
    (*chunk).refs = 1 as libc::c_int;
    while size != 0 {
        if size < sshfs.max_read as size_t {
            tmp = size;
        } else {
            tmp = sshfs.max_read as size_t;
        }
        bsize = tmp;
        __n___0 = 1 as libc::c_int as gsize;
        __s___0 = ::std::mem::size_of::<read_req>() as libc::c_ulong;
        if __s___0 == 1 as libc::c_ulong {
            __p___0 = g_malloc0(__n___0);
        } else {
            __p___0 = g_malloc0_n(__n___0, __s___0);
        }
        rreq = __p___0 as *mut read_req;
        (*rreq).sio = &mut (*chunk).sio;
        (*rreq).size = bsize;
        buf_init(&mut (*rreq).data, 0 as libc::c_int as size_t);
        list_add(&mut (*rreq).list, &mut (*chunk).reqs);
        buf_init(&mut buf, 0 as libc::c_int as size_t);
        buf_add_buf(&mut buf, handle as *const buffer);
        buf_add_uint64(&mut buf, offset as uint64_t);
        buf_add_uint32(&mut buf, bsize as uint32_t);
        buf_to_iov(
            &mut buf as *mut buffer as *const buffer,
            &mut *iov.as_mut_ptr().offset(0 as libc::c_int as isize),
        );
        err = sftp_request_send(
            (*sf).conn,
            5 as libc::c_int as uint8_t,
            iov.as_mut_ptr(),
            1 as libc::c_int as size_t,
            Some(sshfs_read_begin as unsafe extern "C" fn(*mut request) -> ()),
            Some(sshfs_read_end as unsafe extern "C" fn(*mut request) -> ()),
            0 as libc::c_int,
            rreq as *mut libc::c_void,
            0 as *mut libc::c_void as *mut *mut request,
        );
        buf_free(&mut buf);
        if err != 0 {
            break;
        }
        size = (size as libc::c_ulong).wrapping_sub(bsize) as size_t as size_t;
        offset = (offset as size_t).wrapping_add(bsize) as off_t;
    }
    return chunk;
}
unsafe extern "C" fn wait_chunk(
    mut chunk: *mut read_chunk,
    mut buf: *mut libc::c_char,
    mut size: size_t,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut rreq: *mut read_req = 0 as *mut read_req;
    let mut __mptr: *const list_head = 0 as *const list_head;
    let mut tmp: libc::c_int = 0;
    res = 0 as libc::c_int;
    pthread_mutex_lock(&mut sshfs.lock);
    while (*chunk).sio.num_reqs != 0 {
        pthread_cond_wait(
            &mut (*chunk).sio.finished as *mut pthread_cond_t,
            &mut sshfs.lock as *mut pthread_mutex_t,
        );
    }
    pthread_mutex_unlock(&mut sshfs.lock);
    if (*chunk).sio.error != 0 {
        if (*chunk).sio.error != 1 as libc::c_int {
            res = (*chunk).sio.error;
        }
    } else {
        loop {
            tmp = list_empty(&mut (*chunk).reqs as *mut list_head as *const list_head);
            if tmp != 0 {
                break;
            }
            if size == 0 {
                break;
            }
            __mptr = (*chunk).reqs.prev as *const list_head;
            rreq = (__mptr as *mut libc::c_char)
                .offset(
                    -(&mut (*(0 as *mut read_req)).list as *mut list_head
                        as libc::c_ulong as isize),
                ) as *mut read_req;
            if (*rreq).res < 0 as libc::c_long {
                (*chunk).sio.error = (*rreq).res as libc::c_int;
                break;
            } else if (*rreq).res == 0 as libc::c_long {
                (*chunk).sio.error = 1 as libc::c_int;
                break;
            } else if size < (*rreq).res as size_t {
                buf_get_mem(&mut (*rreq).data, buf as *mut libc::c_void, size);
                (*rreq).res = ((*rreq).res as size_t).wrapping_sub(size) as ssize_t;
                (*rreq)
                    .size = ((*rreq).size as libc::c_ulong).wrapping_sub(size) as size_t
                    as size_t;
                res = (res as size_t).wrapping_add(size) as libc::c_int;
                break;
            } else {
                buf_get_mem(
                    &mut (*rreq).data,
                    buf as *mut libc::c_void,
                    (*rreq).res as size_t,
                );
                res = (res as ssize_t + (*rreq).res) as libc::c_int;
                if ((*rreq).res as size_t) < (*rreq).size {
                    (*chunk).sio.error = 1 as libc::c_int;
                    break;
                } else {
                    buf = buf.offset((*rreq).res as isize);
                    size = (size as libc::c_ulong).wrapping_sub((*rreq).res as size_t)
                        as size_t as size_t;
                    list_del(&mut (*rreq).list);
                    buf_free(&mut (*rreq).data);
                    g_free(rreq as gpointer);
                }
            }
        }
        if res > 0 as libc::c_int {
            (*chunk).offset += res as off_t;
            (*chunk)
                .size = ((*chunk).size as libc::c_ulong).wrapping_sub(res as size_t)
                as size_t as size_t;
        }
    }
    chunk_put_locked(chunk);
    return res;
}
unsafe extern "C" fn sshfs_sync_read(
    mut sf: *mut sshfs_file,
    mut buf: *mut libc::c_char,
    mut size: size_t,
    mut offset: off_t,
) -> libc::c_int {
    let mut chunk: *mut read_chunk = 0 as *mut read_chunk;
    let mut tmp: libc::c_int = 0;
    chunk = sshfs_send_read(sf, size, offset);
    tmp = wait_chunk(chunk, buf, size);
    return tmp;
}
unsafe extern "C" fn submit_read(
    mut sf: *mut sshfs_file,
    mut size: size_t,
    mut offset: off_t,
    mut chunkp: *mut *mut read_chunk,
) {
    let mut chunk: *mut read_chunk = 0 as *mut read_chunk;
    chunk = sshfs_send_read(sf, size, offset);
    pthread_mutex_lock(&mut sshfs.lock);
    (*chunk).modifver = sshfs.modifver;
    chunk_put(*chunkp);
    *chunkp = chunk;
    (*chunk).refs += 1;
    pthread_mutex_unlock(&mut sshfs.lock);
}
unsafe extern "C" fn search_read_chunk(
    mut sf: *mut sshfs_file,
    mut offset: off_t,
) -> *mut read_chunk {
    let mut ch: *mut read_chunk = 0 as *mut read_chunk;
    ch = (*sf).readahead;
    if !ch.is_null() {
        if (*ch).offset == offset {
            if (*ch).modifver == sshfs.modifver {
                (*ch).refs += 1;
                return ch;
            } else {
                return 0 as *mut libc::c_void as *mut read_chunk
            }
        } else {
            return 0 as *mut libc::c_void as *mut read_chunk
        }
    } else {
        return 0 as *mut libc::c_void as *mut read_chunk
    };
}
unsafe extern "C" fn sshfs_async_read(
    mut sf: *mut sshfs_file,
    mut rbuf: *mut libc::c_char,
    mut size: size_t,
    mut offset: off_t,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut total: size_t = 0;
    let mut chunk: *mut read_chunk = 0 as *mut read_chunk;
    let mut chunk_prev: *mut read_chunk = 0 as *mut read_chunk;
    let mut origsize: size_t = 0;
    let mut curr_is_seq: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut prev_size: size_t = 0;
    res = 0 as libc::c_int;
    total = 0 as libc::c_int as size_t;
    chunk_prev = 0 as *mut libc::c_void as *mut read_chunk;
    origsize = size;
    pthread_mutex_lock(&mut sshfs.lock);
    curr_is_seq = (*sf).is_seq;
    if (*sf).next_pos == offset {
        if (*sf).modifver as libc::c_long == sshfs.modifver {
            tmp = 1 as libc::c_int;
        } else {
            tmp = 0 as libc::c_int;
        }
    } else {
        tmp = 0 as libc::c_int;
    }
    (*sf).is_seq = tmp;
    (*sf).next_pos = (offset as size_t).wrapping_add(size) as off_t;
    (*sf).modifver = sshfs.modifver as libc::c_int;
    chunk = search_read_chunk(sf, offset);
    pthread_mutex_unlock(&mut sshfs.lock);
    if !chunk.is_null() {
        if (*chunk).size < size {
            chunk_prev = chunk;
            size = (size as libc::c_ulong).wrapping_sub((*chunk).size) as size_t
                as size_t;
            offset = (offset as size_t).wrapping_add((*chunk).size) as off_t;
            chunk = 0 as *mut libc::c_void as *mut read_chunk;
        }
    }
    if chunk.is_null() {
        submit_read(sf, size, offset, &mut chunk);
    }
    if curr_is_seq != 0 {
        if !chunk.is_null() {
            if (*chunk).size <= size {
                submit_read(
                    sf,
                    origsize,
                    (offset as size_t).wrapping_add(size) as off_t,
                    &mut (*sf).readahead,
                );
            }
        }
    }
    if !chunk_prev.is_null() {
        prev_size = (*chunk_prev).size;
        res = wait_chunk(chunk_prev, rbuf, prev_size);
        if res < prev_size as libc::c_int {
            chunk_put_locked(chunk);
            return res;
        }
        rbuf = rbuf.offset(res as isize);
        total = (total as libc::c_ulong).wrapping_add(res as size_t) as size_t as size_t;
    }
    res = wait_chunk(chunk, rbuf, size);
    if res > 0 as libc::c_int {
        total = (total as libc::c_ulong).wrapping_add(res as size_t) as size_t as size_t;
    }
    if res < 0 as libc::c_int {
        return res;
    }
    return total as libc::c_int;
}
unsafe extern "C" fn sshfs_read(
    mut path: *const libc::c_char,
    mut rbuf: *mut libc::c_char,
    mut size: size_t,
    mut offset: off_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut sf: *mut sshfs_file = 0 as *mut sshfs_file;
    let mut tmp: *mut sshfs_file = 0 as *mut sshfs_file;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    tmp = get_sshfs_file(fi);
    sf = tmp;
    tmp___0 = sshfs_file_is_conn(sf);
    if tmp___0 == 0 {
        return -(5 as libc::c_int);
    }
    if sshfs.sync_read != 0 {
        tmp___1 = sshfs_sync_read(sf, rbuf, size, offset);
        return tmp___1;
    } else {
        tmp___2 = sshfs_async_read(sf, rbuf, size, offset);
        return tmp___2;
    };
}
unsafe extern "C" fn sshfs_write_begin(mut req: *mut request) {
    let mut sf: *mut sshfs_file = 0 as *mut sshfs_file;
    sf = (*req).data as *mut sshfs_file;
    list_add(&mut (*req).list, &mut (*sf).write_reqs);
}
unsafe extern "C" fn sshfs_write_end(mut req: *mut request) {
    let mut serr: uint32_t = 0;
    let mut sf: *mut sshfs_file = 0 as *mut sshfs_file;
    let mut tmp: libc::c_int = 0;
    sf = (*req).data as *mut sshfs_file;
    if (*req).error != 0 {
        (*sf).write_error = (*req).error;
    } else if (*req).replied != 0 {
        if (*req).reply_type as libc::c_int != 101 as libc::c_int {
            fprintf(stderr, b"protocol error\n\0" as *const u8 as *const libc::c_char);
        } else {
            tmp = buf_get_uint32(&mut (*req).reply, &mut serr);
            if tmp != -(1 as libc::c_int) {
                if serr != 0 as libc::c_uint {
                    (*sf).write_error = -(5 as libc::c_int);
                }
            }
        }
    }
    list_del(&mut (*req).list);
    pthread_cond_broadcast(&mut (*sf).write_finished);
}
unsafe extern "C" fn sshfs_async_write(
    mut sf: *mut sshfs_file,
    mut wbuf: *const libc::c_char,
    mut size: size_t,
    mut offset: off_t,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut handle: *mut buffer = 0 as *mut buffer;
    let mut buf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut iov: [iovec; 2] = [iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    }; 2];
    let mut bsize: size_t = 0;
    let mut tmp: size_t = 0;
    err = 0 as libc::c_int;
    handle = &mut (*sf).handle;
    while err == 0 {
        if size == 0 {
            break;
        }
        if size < sshfs.max_write as size_t {
            tmp = size;
        } else {
            tmp = sshfs.max_write as size_t;
        }
        bsize = tmp;
        buf_init(&mut buf, 0 as libc::c_int as size_t);
        buf_add_buf(&mut buf, handle as *const buffer);
        buf_add_uint64(&mut buf, offset as uint64_t);
        buf_add_uint32(&mut buf, bsize as uint32_t);
        buf_to_iov(
            &mut buf as *mut buffer as *const buffer,
            &mut *iov.as_mut_ptr().offset(0 as libc::c_int as isize),
        );
        iov[1 as libc::c_int as usize].iov_base = wbuf as *mut libc::c_void;
        iov[1 as libc::c_int as usize].iov_len = bsize;
        err = sftp_request_send(
            (*sf).conn,
            6 as libc::c_int as uint8_t,
            iov.as_mut_ptr(),
            2 as libc::c_int as size_t,
            Some(sshfs_write_begin as unsafe extern "C" fn(*mut request) -> ()),
            Some(sshfs_write_end as unsafe extern "C" fn(*mut request) -> ()),
            0 as libc::c_int,
            sf as *mut libc::c_void,
            0 as *mut libc::c_void as *mut *mut request,
        );
        buf_free(&mut buf);
        size = (size as libc::c_ulong).wrapping_sub(bsize) as size_t as size_t;
        wbuf = wbuf.offset(bsize as isize);
        offset = (offset as size_t).wrapping_add(bsize) as off_t;
    }
    return err;
}
unsafe extern "C" fn sshfs_sync_write_begin(mut req: *mut request) {
    let mut sio: *mut sshfs_io = 0 as *mut sshfs_io;
    sio = (*req).data as *mut sshfs_io;
    (*sio).num_reqs += 1;
}
unsafe extern "C" fn sshfs_sync_write_end(mut req: *mut request) {
    let mut serr: uint32_t = 0;
    let mut sio: *mut sshfs_io = 0 as *mut sshfs_io;
    let mut tmp: libc::c_int = 0;
    sio = (*req).data as *mut sshfs_io;
    if (*req).error != 0 {
        (*sio).error = (*req).error;
    } else if (*req).replied != 0 {
        if (*req).reply_type as libc::c_int != 101 as libc::c_int {
            fprintf(stderr, b"protocol error\n\0" as *const u8 as *const libc::c_char);
        } else {
            tmp = buf_get_uint32(&mut (*req).reply, &mut serr);
            if tmp != -(1 as libc::c_int) {
                if serr != 0 as libc::c_uint {
                    (*sio).error = -(5 as libc::c_int);
                }
            }
        }
    }
    (*sio).num_reqs -= 1;
    if (*sio).num_reqs == 0 {
        pthread_cond_broadcast(&mut (*sio).finished);
    }
}
unsafe extern "C" fn sshfs_sync_write(
    mut sf: *mut sshfs_file,
    mut wbuf: *const libc::c_char,
    mut size: size_t,
    mut offset: off_t,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut handle: *mut buffer = 0 as *mut buffer;
    let mut sio: sshfs_io = sshfs_io {
        num_reqs: 0,
        finished: __anonunion_pthread_cond_t_951761805 {
            __data: __pthread_cond_s {
                __annonCompField1: __anonunion____missing_field_name_505876810 {
                    __wseq: 0,
                },
                __annonCompField2: __anonunion____missing_field_name_897626226 {
                    __g1_start: 0,
                },
                __g_refs: [0; 2],
                __g_size: [0; 2],
                __g1_orig_size: 0,
                __wrefs: 0,
                __g_signals: [0; 2],
            },
        },
        error: 0,
    };
    let mut buf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut iov: [iovec; 2] = [iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    }; 2];
    let mut bsize: size_t = 0;
    let mut tmp: size_t = 0;
    err = 0 as libc::c_int;
    handle = &mut (*sf).handle;
    sio.num_reqs = 0 as libc::c_int;
    sio.finished.__data.__annonCompField1.__wseq = 0 as libc::c_ulonglong;
    sio.finished.__data.__annonCompField2.__g1_start = 0 as libc::c_ulonglong;
    sio.finished.__data.__g_refs[0 as libc::c_int as usize] = 0 as libc::c_uint;
    sio.finished.__data.__g_refs[1 as libc::c_int as usize] = 0 as libc::c_uint;
    sio.finished.__data.__g_size[0 as libc::c_int as usize] = 0 as libc::c_uint;
    sio.finished.__data.__g_size[1 as libc::c_int as usize] = 0 as libc::c_uint;
    sio.finished.__data.__g1_orig_size = 0 as libc::c_uint;
    sio.finished.__data.__wrefs = 0 as libc::c_uint;
    sio.finished.__data.__g_signals[0 as libc::c_int as usize] = 0 as libc::c_uint;
    sio.finished.__data.__g_signals[1 as libc::c_int as usize] = 0 as libc::c_uint;
    sio.error = 0 as libc::c_int;
    pthread_cond_init(
        &mut sio.finished as *mut pthread_cond_t,
        0 as *mut libc::c_void as *const pthread_condattr_t,
    );
    while err == 0 {
        if size == 0 {
            break;
        }
        if size < sshfs.max_write as size_t {
            tmp = size;
        } else {
            tmp = sshfs.max_write as size_t;
        }
        bsize = tmp;
        buf_init(&mut buf, 0 as libc::c_int as size_t);
        buf_add_buf(&mut buf, handle as *const buffer);
        buf_add_uint64(&mut buf, offset as uint64_t);
        buf_add_uint32(&mut buf, bsize as uint32_t);
        buf_to_iov(
            &mut buf as *mut buffer as *const buffer,
            &mut *iov.as_mut_ptr().offset(0 as libc::c_int as isize),
        );
        iov[1 as libc::c_int as usize].iov_base = wbuf as *mut libc::c_void;
        iov[1 as libc::c_int as usize].iov_len = bsize;
        err = sftp_request_send(
            (*sf).conn,
            6 as libc::c_int as uint8_t,
            iov.as_mut_ptr(),
            2 as libc::c_int as size_t,
            Some(sshfs_sync_write_begin as unsafe extern "C" fn(*mut request) -> ()),
            Some(sshfs_sync_write_end as unsafe extern "C" fn(*mut request) -> ()),
            0 as libc::c_int,
            &mut sio as *mut sshfs_io as *mut libc::c_void,
            0 as *mut libc::c_void as *mut *mut request,
        );
        buf_free(&mut buf);
        size = (size as libc::c_ulong).wrapping_sub(bsize) as size_t as size_t;
        wbuf = wbuf.offset(bsize as isize);
        offset = (offset as size_t).wrapping_add(bsize) as off_t;
    }
    pthread_mutex_lock(&mut sshfs.lock);
    while sio.num_reqs != 0 {
        pthread_cond_wait(
            &mut sio.finished as *mut pthread_cond_t,
            &mut sshfs.lock as *mut pthread_mutex_t,
        );
    }
    pthread_mutex_unlock(&mut sshfs.lock);
    if err == 0 {
        err = sio.error;
    }
    return err;
}
unsafe extern "C" fn sshfs_write(
    mut path: *const libc::c_char,
    mut wbuf: *const libc::c_char,
    mut size: size_t,
    mut offset: off_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut sf: *mut sshfs_file = 0 as *mut sshfs_file;
    let mut tmp: *mut sshfs_file = 0 as *mut sshfs_file;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    tmp = get_sshfs_file(fi);
    sf = tmp;
    tmp___0 = sshfs_file_is_conn(sf);
    if tmp___0 == 0 {
        return -(5 as libc::c_int);
    }
    sshfs_inc_modifver();
    if sshfs.sync_write == 0 {
        if (*sf).write_error == 0 {
            err = sshfs_async_write(sf, wbuf, size, offset);
        } else {
            err = sshfs_sync_write(sf, wbuf, size, offset);
        }
    } else {
        err = sshfs_sync_write(sf, wbuf, size, offset);
    }
    if err != 0 {
        tmp___1 = err;
    } else {
        tmp___1 = size as libc::c_int;
    }
    return tmp___1;
}
unsafe extern "C" fn sshfs_ext_statvfs(
    mut path: *const libc::c_char,
    mut stbuf: *mut statvfs,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut outbuf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut tmp: *mut conn = 0 as *mut conn;
    let mut tmp___0: libc::c_int = 0;
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    buf_add_string(
        &mut buf,
        b"statvfs@openssh.com\0" as *const u8 as *const libc::c_char,
    );
    buf_add_path(&mut buf, path);
    tmp = get_conn(
        0 as *mut libc::c_void as *const sshfs_file,
        0 as *mut libc::c_void as *const libc::c_char,
    );
    err = sftp_request(
        tmp,
        200 as libc::c_int as uint8_t,
        &mut buf as *mut buffer as *const buffer,
        201 as libc::c_int as uint8_t,
        &mut outbuf,
    );
    if err == 0 {
        tmp___0 = buf_get_statvfs(&mut outbuf, stbuf);
        if tmp___0 == -(1 as libc::c_int) {
            err = -(5 as libc::c_int);
        }
        buf_free(&mut outbuf);
    }
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sshfs_statfs(
    mut path: *const libc::c_char,
    mut buf: *mut statvfs,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: __fsblkcnt64_t = 0;
    let mut tmp___1: __fsblkcnt64_t = 0;
    let mut tmp___2: __fsfilcnt64_t = 0;
    if sshfs.ext_statvfs != 0 {
        tmp = sshfs_ext_statvfs(path, buf);
        return tmp;
    }
    (*buf).f_namemax = 255 as libc::c_ulong;
    (*buf).f_bsize = sshfs.blksize as libc::c_ulong;
    (*buf).f_frsize = (*buf).f_bsize;
    tmp___1 = (1073741824000 as libc::c_ulonglong)
        .wrapping_div((*buf).f_frsize as libc::c_ulonglong) as __fsblkcnt64_t;
    (*buf).f_bavail = tmp___1;
    tmp___0 = tmp___1;
    (*buf).f_bfree = tmp___0;
    (*buf).f_blocks = tmp___0;
    tmp___2 = 1000000000 as libc::c_int as __fsfilcnt64_t;
    (*buf).f_ffree = tmp___2;
    (*buf).f_files = tmp___2;
    return 0 as libc::c_int;
}
unsafe extern "C" fn sshfs_create(
    mut path: *const libc::c_char,
    mut mode: mode_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    if sshfs.createmode_workaround != 0 {
        mode = 0 as libc::c_int as mode_t;
    }
    tmp = sshfs_open_common(path, mode, fi);
    return tmp;
}
unsafe extern "C" fn sshfs_truncate(
    mut path: *const libc::c_char,
    mut size: off_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut sf: *mut sshfs_file = 0 as *mut sshfs_file;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut conn = 0 as *mut conn;
    sf = 0 as *mut libc::c_void as *mut sshfs_file;
    if fi as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        sf = get_sshfs_file(fi);
        tmp = sshfs_file_is_conn(sf);
        if tmp == 0 {
            return -(5 as libc::c_int);
        }
    }
    sshfs_inc_modifver();
    if sshfs.truncate_workaround != 0 {
        tmp___0 = sshfs_truncate_workaround(path, size, fi);
        return tmp___0;
    }
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    if sf as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        buf_add_buf(&mut buf, &mut (*sf).handle as *mut buffer as *const buffer);
    } else {
        buf_add_path(&mut buf, path);
    }
    buf_add_uint32(&mut buf, 1 as libc::c_int as uint32_t);
    buf_add_uint64(&mut buf, size as uint64_t);
    if sf as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___1 = 9 as libc::c_int;
    } else {
        tmp___1 = 10 as libc::c_int;
    }
    tmp___2 = get_conn(sf as *const sshfs_file, path);
    err = sftp_request(
        tmp___2,
        tmp___1 as uint8_t,
        &mut buf as *mut buffer as *const buffer,
        101 as libc::c_int as uint8_t,
        0 as *mut libc::c_void as *mut buffer,
    );
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sshfs_getattr(
    mut path: *const libc::c_char,
    mut stbuf: *mut stat,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut buf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut outbuf: buffer = buffer {
        p: 0 as *mut uint8_t,
        len: 0,
        size: 0,
    };
    let mut sf: *mut sshfs_file = 0 as *mut sshfs_file;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut conn = 0 as *mut conn;
    sf = 0 as *mut libc::c_void as *mut sshfs_file;
    if fi as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        if sshfs.fstat_workaround == 0 {
            sf = get_sshfs_file(fi);
            tmp = sshfs_file_is_conn(sf);
            if tmp == 0 {
                return -(5 as libc::c_int);
            }
        }
    }
    buf_init(&mut buf, 0 as libc::c_int as size_t);
    if sf as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        buf_add_path(&mut buf, path);
        if sshfs.follow_symlinks != 0 {
            tmp___0 = 17 as libc::c_int;
        } else {
            tmp___0 = 7 as libc::c_int;
        }
        tmp___1 = get_conn(sf as *const sshfs_file, path);
        err = sftp_request(
            tmp___1,
            tmp___0 as uint8_t,
            &mut buf as *mut buffer as *const buffer,
            105 as libc::c_int as uint8_t,
            &mut outbuf,
        );
    } else {
        buf_add_buf(&mut buf, &mut (*sf).handle as *mut buffer as *const buffer);
        err = sftp_request(
            (*sf).conn,
            8 as libc::c_int as uint8_t,
            &mut buf as *mut buffer as *const buffer,
            105 as libc::c_int as uint8_t,
            &mut outbuf,
        );
    }
    if err == 0 {
        err = buf_get_attrs(
            &mut outbuf,
            stbuf,
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        buf_free(&mut outbuf);
    }
    buf_free(&mut buf);
    return err;
}
unsafe extern "C" fn sshfs_truncate_zero(mut path: *const libc::c_char) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut fi: fuse_file_info = fuse_file_info {
        flags: 0,
        writepage_direct_io_keep_cache_flush_nonseekable_flock_release_cache_readdir_padding_padding2: [0; 8],
        c2rust_padding: [0; 4],
        fh: 0,
        lock_owner: 0,
        poll_events: 0,
    };
    fi.flags = 513 as libc::c_int;
    err = sshfs_open(path, &mut fi);
    if err == 0 {
        sshfs_release(path, &mut fi);
    }
    return err;
}
unsafe extern "C" fn calc_buf_size(mut size: off_t, mut offset: off_t) -> size_t {
    let mut tmp: off_t = 0;
    if (offset + sshfs.max_read as off_t) < size {
        tmp = sshfs.max_read as off_t;
    } else {
        tmp = size - offset;
    }
    return tmp as size_t;
}
unsafe extern "C" fn sshfs_truncate_shrink(
    mut path: *const libc::c_char,
    mut size: off_t,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut offset: off_t = 0;
    let mut fi: fuse_file_info = fuse_file_info {
        flags: 0,
        writepage_direct_io_keep_cache_flush_nonseekable_flock_release_cache_readdir_padding_padding2: [0; 8],
        c2rust_padding: [0; 4],
        fh: 0,
        lock_owner: 0,
        poll_events: 0,
    };
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut bufsize: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut bufsize___0: size_t = 0;
    let mut tmp___1: size_t = 0;
    tmp = calloc(size as size_t, 1 as libc::c_int as size_t);
    data = tmp as *mut libc::c_char;
    if data.is_null() {
        return -(12 as libc::c_int);
    }
    fi.flags = 0 as libc::c_int;
    res = sshfs_open(path, &mut fi);
    if !(res != 0) {
        offset = 0 as libc::c_int as off_t;
        while offset < size {
            tmp___0 = calc_buf_size(size, offset);
            bufsize = tmp___0;
            res = sshfs_read(
                path,
                data.offset(offset as isize),
                bufsize,
                offset,
                &mut fi,
            );
            if res <= 0 as libc::c_int {
                break;
            }
            offset += res as off_t;
        }
        sshfs_release(path, &mut fi);
        if !(res < 0 as libc::c_int) {
            fi.flags = 513 as libc::c_int;
            res = sshfs_open(path, &mut fi);
            if !(res != 0) {
                offset = 0 as libc::c_int as off_t;
                while offset < size {
                    tmp___1 = calc_buf_size(size, offset);
                    bufsize___0 = tmp___1;
                    res = sshfs_write(
                        path,
                        data.offset(offset as isize) as *const libc::c_char,
                        bufsize___0,
                        offset,
                        &mut fi,
                    );
                    if res < 0 as libc::c_int {
                        break;
                    }
                    offset += res as off_t;
                }
                if res >= 0 as libc::c_int {
                    res = sshfs_flush(path, &mut fi);
                }
                sshfs_release(path, &mut fi);
            }
        }
    }
    free(data as *mut libc::c_void);
    return res;
}
unsafe extern "C" fn sshfs_truncate_extend(
    mut path: *const libc::c_char,
    mut size: off_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut c: libc::c_char = 0;
    let mut tmpfi: fuse_file_info = fuse_file_info {
        flags: 0,
        writepage_direct_io_keep_cache_flush_nonseekable_flock_release_cache_readdir_padding_padding2: [0; 8],
        c2rust_padding: [0; 4],
        fh: 0,
        lock_owner: 0,
        poll_events: 0,
    };
    let mut openfi: *mut fuse_file_info = 0 as *mut fuse_file_info;
    c = 0 as libc::c_int as libc::c_char;
    openfi = fi;
    if fi.is_null() {
        openfi = &mut tmpfi;
        (*openfi).flags = 1 as libc::c_int;
        res = sshfs_open(path, openfi);
        if res != 0 {
            return res;
        }
    }
    res = sshfs_write(
        path,
        &mut c as *mut libc::c_char as *const libc::c_char,
        1 as libc::c_int as size_t,
        size - 1 as libc::c_long,
        openfi,
    );
    if res == 1 as libc::c_int {
        res = sshfs_flush(path, openfi);
    }
    if fi.is_null() {
        sshfs_release(path, openfi);
    }
    return res;
}
unsafe extern "C" fn sshfs_truncate_workaround(
    mut path: *const libc::c_char,
    mut size: off_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut stbuf: stat = stat {
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
    let mut err: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    if size == 0 as libc::c_long {
        tmp = sshfs_truncate_zero(path);
        return tmp;
    } else {
        err = sshfs_getattr(path, &mut stbuf, fi);
        if err != 0 {
            return err;
        }
        if stbuf.st_size == size {
            return 0 as libc::c_int
        } else if stbuf.st_size > size {
            tmp___0 = sshfs_truncate_shrink(path, size);
            return tmp___0;
        } else {
            tmp___1 = sshfs_truncate_extend(path, size, fi);
            return tmp___1;
        }
    };
}
unsafe extern "C" fn processing_init() -> libc::c_int {
    let mut i: libc::c_int = 0;
    signal(
        13 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(1 as libc::c_int as libc::intptr_t),
    );
    pthread_mutex_init(
        &mut sshfs.lock,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
    i = 0 as libc::c_int;
    while i < sshfs.max_conns {
        pthread_mutex_init(
            &mut (*(sshfs.conns).offset(i as isize)).lock_write,
            0 as *mut libc::c_void as *const pthread_mutexattr_t,
        );
        i += 1;
    }
    pthread_cond_init(
        &mut sshfs.outstanding_cond as *mut pthread_cond_t,
        0 as *mut libc::c_void as *const pthread_condattr_t,
    );
    sshfs
        .reqtab = g_hash_table_new(
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(gconstpointer) -> guint>,
        >(0 as *mut libc::c_void),
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>,
        >(0 as *mut libc::c_void),
    );
    if (sshfs.reqtab).is_null() {
        fprintf(
            stderr,
            b"failed to create hash table\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if sshfs.max_conns > 1 as libc::c_int {
        sshfs
            .conntab = g_hash_table_new_full(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(
                g_str_equal
                    as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
            ),
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<unsafe extern "C" fn(gpointer) -> ()>,
            >(0 as *mut libc::c_void),
        );
        if (sshfs.conntab).is_null() {
            fprintf(
                stderr,
                b"failed to create hash table\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
static mut sshfs_oper: fuse_operations = {
    let mut init = fuse_operations {
        getattr: Some(
            sshfs_getattr
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *mut stat,
                    *mut fuse_file_info,
                ) -> libc::c_int,
        ),
        readlink: Some(
            sshfs_readlink
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *mut libc::c_char,
                    size_t,
                ) -> libc::c_int,
        ),
        mknod: Some(
            sshfs_mknod
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    mode_t,
                    dev_t,
                ) -> libc::c_int,
        ),
        mkdir: Some(
            sshfs_mkdir
                as unsafe extern "C" fn(*const libc::c_char, mode_t) -> libc::c_int,
        ),
        unlink: Some(
            sshfs_unlink as unsafe extern "C" fn(*const libc::c_char) -> libc::c_int,
        ),
        rmdir: Some(
            sshfs_rmdir as unsafe extern "C" fn(*const libc::c_char) -> libc::c_int,
        ),
        symlink: Some(
            sshfs_symlink
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        rename: Some(
            sshfs_rename
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                    libc::c_uint,
                ) -> libc::c_int,
        ),
        link: Some(
            sshfs_link
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        chmod: Some(
            sshfs_chmod
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    mode_t,
                    *mut fuse_file_info,
                ) -> libc::c_int,
        ),
        chown: Some(
            sshfs_chown
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    uid_t,
                    gid_t,
                    *mut fuse_file_info,
                ) -> libc::c_int,
        ),
        truncate: Some(
            sshfs_truncate
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    off_t,
                    *mut fuse_file_info,
                ) -> libc::c_int,
        ),
        open: Some(
            sshfs_open
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *mut fuse_file_info,
                ) -> libc::c_int,
        ),
        read: Some(
            sshfs_read
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *mut libc::c_char,
                    size_t,
                    off_t,
                    *mut fuse_file_info,
                ) -> libc::c_int,
        ),
        write: Some(
            sshfs_write
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                    size_t,
                    off_t,
                    *mut fuse_file_info,
                ) -> libc::c_int,
        ),
        statfs: Some(
            sshfs_statfs
                as unsafe extern "C" fn(*const libc::c_char, *mut statvfs) -> libc::c_int,
        ),
        flush: Some(
            sshfs_flush
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *mut fuse_file_info,
                ) -> libc::c_int,
        ),
        release: Some(
            sshfs_release
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *mut fuse_file_info,
                ) -> libc::c_int,
        ),
        fsync: Some(
            sshfs_fsync
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                    *mut fuse_file_info,
                ) -> libc::c_int,
        ),
        setxattr: None,
        getxattr: None,
        listxattr: None,
        removexattr: None,
        opendir: Some(
            sshfs_opendir
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *mut fuse_file_info,
                ) -> libc::c_int,
        ),
        readdir: Some(
            sshfs_readdir
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *mut libc::c_void,
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const libc::c_char,
                            *const stat,
                            off_t,
                            fuse_fill_dir_flags,
                        ) -> libc::c_int,
                    >,
                    off_t,
                    *mut fuse_file_info,
                    fuse_readdir_flags,
                ) -> libc::c_int,
        ),
        releasedir: Some(
            sshfs_releasedir
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *mut fuse_file_info,
                ) -> libc::c_int,
        ),
        fsyncdir: None,
        init: Some(
            sshfs_init
                as unsafe extern "C" fn(
                    *mut fuse_conn_info,
                    *mut fuse_config,
                ) -> *mut libc::c_void,
        ),
        destroy: None,
        access: Some(
            sshfs_access
                as unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_int,
        ),
        create: Some(
            sshfs_create
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    mode_t,
                    *mut fuse_file_info,
                ) -> libc::c_int,
        ),
        lock: None,
        utimens: Some(
            sshfs_utimens
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const timespec,
                    *mut fuse_file_info,
                ) -> libc::c_int,
        ),
        bmap: None,
        ioctl: None,
        poll: None,
        write_buf: None,
        read_buf: None,
        flock: None,
        fallocate: None,
        copy_file_range: None,
        lseek: None,
    };
    init
};
unsafe extern "C" fn usage(mut progname: *const libc::c_char) {
    printf(
        b"usage: %s [user@]host:[dir] mountpoint [options]\n\n    -h   --help            print help\n    -V   --version         print version\n    -f                     foreground operation\n    -s                     disable multi-threaded operation\n    -p PORT                equivalent to '-o port=PORT'\n    -C                     equivalent to '-o compression=yes'\n    -F ssh_configfile      specifies alternative ssh configuration file\n    -1                     equivalent to '-o ssh_protocol=1'\n    -o opt,[opt...]        mount options\n    -o reconnect           reconnect to server\n    -o delay_connect       delay connection to server\n    -o sshfs_sync          synchronous writes\n    -o no_readahead        synchronous reads (no speculative readahead)\n    -o sync_readdir        synchronous readdir\n    -d, --debug            print some debugging information (implies -f)\n    -v, --verbose          print ssh replies and messages\n    -o dir_cache=BOOL      enable caching of directory contents (names,\n                           attributes, symlink targets) {yes,no} (default: yes)\n    -o dcache_max_size=N   sets the maximum size of the directory cache (default: 10000)\n    -o dcache_timeout=N    sets timeout for directory cache in seconds (default: 20)\n    -o dcache_{stat,link,dir}_timeout=N\n                           sets separate timeout for {attributes, symlinks, names}\n    -o dcache_clean_interval=N\n                           sets the interval for automatic cleaning of the\n                           cache (default: 60)\n    -o dcache_min_clean_interval=N\n                           sets the interval for forced cleaning of the\n                           cache if full (default: 5)\n    -o direct_io           enable direct i/o\n    -o workaround=LIST     colon separated list of workarounds\n             none             no workarounds enabled\n             [no]rename       fix renaming to existing file (default: off)\n             [no]renamexdev   fix moving across filesystems (default: off)\n             [no]truncate     fix truncate for old servers (default: off)\n             [no]buflimit     fix buffer fillup bug in server (default: off)\n             [no]fstat        always use stat() instead of fstat() (default: off)\n             [no]createmode   always pass mode 0 to create (default: off)\n    -o idmap=TYPE          user/group ID mapping (default: none)\n             none             no translation of the ID space\n             user             only translate UID/GID of connecting user\n             file             translate UIDs/GIDs contained in uidfile/gidfile\n    -o uidfile=FILE        file containing username:remote_uid mappings\n    -o gidfile=FILE        file containing groupname:remote_gid mappings\n    -o nomap=TYPE          with idmap=file, how to handle missing mappings\n             ignore           don't do any re-mapping\n             error            return an error (default)\n    -o ssh_command=CMD     execute CMD instead of 'ssh'\n    -o ssh_protocol=N      ssh protocol to use (default: 2)\n    -o sftp_server=SERV    path to sftp server or subsystem (default: sftp)\n    -o directport=PORT     directly connect to PORT bypassing ssh\n    -o passive             communicate over stdin and stdout bypassing network\n    -o disable_hardlink    link(2) will return with errno set to ENOSYS\n    -o transform_symlinks  transform absolute symlinks to relative\n    -o follow_symlinks     follow symlinks on the server\n    -o no_check_root       don't check for existence of 'dir' on server\n    -o password_stdin      read password from stdin (only for pam_mount!)\n    -o max_conns=N         open parallel SSH connections\n    -o SSHOPT=VAL          ssh options (see man ssh_config)\n\nFUSE Options:\n\0"
            as *const u8 as *const libc::c_char,
        progname,
    );
}
unsafe extern "C" fn is_ssh_opt(mut arg: *const libc::c_char) -> libc::c_int {
    let mut arglen: libc::c_uint = 0;
    let mut tmp: size_t = 0;
    let mut o: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut olen: libc::c_uint = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: libc::c_int = 0;
    if *arg.offset(0 as libc::c_int as isize) as libc::c_int != 45 as libc::c_int {
        tmp = strlen(arg);
        arglen = tmp as libc::c_uint;
        o = ssh_opts.as_mut_ptr();
        while !(*o).is_null() {
            tmp___0 = strlen(*o);
            olen = tmp___0 as libc::c_uint;
            if arglen > olen {
                if *arg.offset(olen as isize) as libc::c_int == 61 as libc::c_int {
                    tmp___1 = strncasecmp(arg, *o, olen as size_t);
                    if tmp___1 == 0 as libc::c_int {
                        return 1 as libc::c_int;
                    }
                }
            }
            o = o.offset(1);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn sshfs_opt_proc(
    mut data: *mut libc::c_void,
    mut arg: *const libc::c_char,
    mut key: libc::c_int,
    mut outargs: *mut fuse_args,
) -> libc::c_int {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    match key {
        -1 => {
            tmp___0 = is_ssh_opt(arg);
            if tmp___0 != 0 {
                tmp = g_strdup_printf(
                    b"-o%s\0" as *const u8 as *const libc::c_char,
                    arg,
                );
                ssh_add_arg(tmp as *const libc::c_char);
                g_free(tmp as gpointer);
                return 0 as libc::c_int;
            }
            return 1 as libc::c_int;
        }
        -2 => {
            if (sshfs.host).is_null() {
                tmp___5 = strchr(arg, ':' as i32);
                if !tmp___5.is_null() {
                    sshfs.host = strdup(arg);
                    return 0 as libc::c_int;
                }
            }
            if (sshfs.mountpoint).is_null() {
                tmp___1 = sscanf(
                    arg,
                    b"/dev/fd/%u%n\0" as *const u8 as *const libc::c_char,
                    &mut fd as *mut libc::c_int,
                    &mut len as *mut libc::c_int,
                );
                if tmp___1 == 1 as libc::c_int {
                    tmp___2 = strlen(arg);
                    if len as size_t == tmp___2 {
                        sshfs.mountpoint = strdup(arg);
                    } else {
                        sshfs
                            .mountpoint = realpath(
                            arg,
                            0 as *mut libc::c_void as *mut libc::c_char,
                        );
                    }
                } else {
                    sshfs
                        .mountpoint = realpath(
                        arg,
                        0 as *mut libc::c_void as *mut libc::c_char,
                    );
                }
                if (sshfs.mountpoint).is_null() {
                    tmp___3 = __errno_location();
                    tmp___4 = strerror(*tmp___3);
                    fprintf(
                        stderr,
                        b"sshfs: bad mount point `%s': %s\n\0" as *const u8
                            as *const libc::c_char,
                        arg,
                        tmp___4,
                    );
                    return -(1 as libc::c_int);
                }
                return 0 as libc::c_int;
            }
            fprintf(
                stderr,
                b"sshfs: invalid argument `%s'\n\0" as *const u8 as *const libc::c_char,
                arg,
            );
            return -(1 as libc::c_int);
        }
        0 => {
            tmp = g_strdup_printf(
                b"-oPort=%s\0" as *const u8 as *const libc::c_char,
                arg.offset(2 as libc::c_int as isize),
            );
            ssh_add_arg(tmp as *const libc::c_char);
            g_free(tmp as gpointer);
            return 0 as libc::c_int;
        }
        1 => {
            ssh_add_arg(b"-oCompression=yes\0" as *const u8 as *const libc::c_char);
            return 0 as libc::c_int;
        }
        2 => {
            tmp = g_strdup_printf(
                b"-F%s\0" as *const u8 as *const libc::c_char,
                arg.offset(2 as libc::c_int as isize),
            );
            ssh_add_arg(tmp as *const libc::c_char);
            g_free(tmp as gpointer);
            return 0 as libc::c_int;
        }
        _ => {
            fprintf(stderr, b"internal error\n\0" as *const u8 as *const libc::c_char);
            abort();
        }
    };
}
unsafe extern "C" fn workaround_opt_proc(
    mut data: *mut libc::c_void,
    mut arg: *const libc::c_char,
    mut key: libc::c_int,
    mut outargs: *mut fuse_args,
) -> libc::c_int {
    fprintf(
        stderr,
        b"unknown workaround: '%s'\n\0" as *const u8 as *const libc::c_char,
        arg,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn parse_workarounds() -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut argv0: [libc::c_char; 1] = [0; 1];
    let mut argv1: [libc::c_char; 3] = [0; 3];
    let mut argv: [*mut libc::c_char; 4] = [0 as *mut libc::c_char; 4];
    let mut args: fuse_args = fuse_args {
        argc: 0,
        argv: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        allocated: 0,
    };
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    argv0[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    argv1[0 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    argv1[1 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    argv1[2 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    argv[0 as libc::c_int as usize] = argv0.as_mut_ptr();
    argv[1 as libc::c_int as usize] = argv1.as_mut_ptr();
    argv[2 as libc::c_int as usize] = sshfs.workarounds;
    argv[3 as libc::c_int as usize] = 0 as *mut libc::c_void as *mut libc::c_char;
    args.argc = 3 as libc::c_int;
    args.argv = argv.as_mut_ptr();
    args.allocated = 0 as libc::c_int;
    s = sshfs.workarounds;
    if s.is_null() {
        return 0 as libc::c_int;
    }
    loop {
        s = strchr(s as *const libc::c_char, ':' as i32);
        if s.is_null() {
            break;
        }
        *s = ',' as i32 as libc::c_char;
    }
    res = fuse_opt_parse(
        &mut args,
        &mut sshfs as *mut sshfs as *mut libc::c_void,
        workaround_opts.as_mut_ptr() as *const fuse_opt,
        Some(
            workaround_opt_proc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    libc::c_int,
                    *mut fuse_args,
                ) -> libc::c_int,
        ),
    );
    fuse_opt_free_args(&mut args);
    return res;
}
unsafe extern "C" fn read_password() -> libc::c_int {
    let mut size: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut max_password: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut tmp___3: ssize_t = 0;
    tmp = getpagesize();
    size = tmp;
    if (1024 as libc::c_int) < size - 1 as libc::c_int {
        tmp___0 = 1024 as libc::c_int;
    } else {
        tmp___0 = size - 1 as libc::c_int;
    }
    max_password = tmp___0;
    tmp___1 = mmap(
        0 as *mut libc::c_void,
        size as size_t,
        3 as libc::c_int,
        8226 as libc::c_int,
        -(1 as libc::c_int),
        0 as libc::c_int as __off64_t,
    );
    sshfs.password = tmp___1 as *mut libc::c_char;
    if sshfs.password as libc::c_ulong
        == -(1 as libc::c_int) as *mut libc::c_void as libc::c_ulong
    {
        perror(
            b"Failed to allocate locked page for password\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    tmp___2 = mlock(sshfs.password as *const libc::c_void, size as size_t);
    if tmp___2 != 0 as libc::c_int {
        memset(sshfs.password as *mut libc::c_void, 0 as libc::c_int, size as size_t);
        munmap(sshfs.password as *mut libc::c_void, size as size_t);
        sshfs.password = 0 as *mut libc::c_void as *mut libc::c_char;
        perror(
            b"Failed to allocate locked page for password\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    n = 0 as libc::c_int;
    while n < max_password {
        tmp___3 = read(
            0 as libc::c_int,
            (sshfs.password).offset(n as isize) as *mut libc::c_void,
            1 as libc::c_int as size_t,
        );
        res = tmp___3 as libc::c_int;
        if res == -(1 as libc::c_int) {
            perror(b"Reading password\0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int);
        }
        if res == 0 as libc::c_int {
            *(sshfs.password).offset(n as isize) = '\n' as i32 as libc::c_char;
            break;
        } else {
            if *(sshfs.password).offset(n as isize) as libc::c_int == 10 as libc::c_int {
                break;
            }
            n += 1;
        }
    }
    if n == max_password {
        fprintf(stderr, b"Password too long\n\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    *(sshfs.password)
        .offset((n + 1 as libc::c_int) as isize) = '\u{0}' as i32 as libc::c_char;
    ssh_add_arg(b"-oNumberOfPasswordPrompts=1\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
static mut pos: *mut libc::c_char = 0 as *const libc::c_void as *mut libc::c_void
    as *mut libc::c_char;
unsafe extern "C" fn tokenize_on_space(mut str: *mut libc::c_char) -> *mut libc::c_char {
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    start = 0 as *mut libc::c_void as *mut libc::c_char;
    if !str.is_null() {
        pos = str;
    }
    if pos.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    while *pos as libc::c_int == 32 as libc::c_int {
        pos = pos.offset(1);
    }
    start = pos;
    while !pos.is_null() {
        if !(*pos as libc::c_int != 0 as libc::c_int) {
            break;
        }
        if *pos as libc::c_int == 32 as libc::c_int {
            if *pos.offset(-(1 as libc::c_int as isize)) as libc::c_int
                != 92 as libc::c_int
            {
                break;
            }
        }
        pos = pos.offset(1);
    }
    if *pos as libc::c_int == 0 as libc::c_int {
        pos = 0 as *mut libc::c_void as *mut libc::c_char;
    } else {
        *pos = '\u{0}' as i32 as libc::c_char;
        pos = pos.offset(1);
    }
    return start;
}
unsafe extern "C" fn set_ssh_command() {
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    token = 0 as *mut libc::c_void as *mut libc::c_char;
    i = 0 as libc::c_int;
    token = tokenize_on_space(sshfs.ssh_command);
    while token as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        if i == 0 as libc::c_int {
            replace_arg(
                (sshfs.ssh_args.argv).offset(0 as libc::c_int as isize),
                token as *const libc::c_char,
            );
        } else {
            tmp = fuse_opt_insert_arg(
                &mut sshfs.ssh_args,
                i,
                token as *const libc::c_char,
            );
            if tmp == -(1 as libc::c_int) {
                _exit(1 as libc::c_int);
            }
        }
        i += 1;
        token = tokenize_on_space(0 as *mut libc::c_void as *mut libc::c_char);
    }
}
unsafe extern "C" fn find_base_path() -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    s = sshfs.host;
    d = s;
    while *s != 0 {
        if !(*s as libc::c_int != 58 as libc::c_int) {
            break;
        }
        if *s as libc::c_int == 91 as libc::c_int {
            s = s.offset(1);
            while *s as libc::c_int != 93 as libc::c_int {
                if *s == 0 {
                    fprintf(
                        stderr,
                        b"missing ']' in hostname\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                tmp = d;
                d = d.offset(1);
                *tmp = *s;
                s = s.offset(1);
            }
        } else {
            tmp___0 = d;
            d = d.offset(1);
            *tmp___0 = *s;
        }
        s = s.offset(1);
    }
    tmp___1 = d;
    d = d.offset(1);
    *tmp___1 = '\u{0}' as i32 as libc::c_char;
    s = s.offset(1);
    return s;
}
unsafe extern "C" fn fsname_escape_commas(
    mut fsnameold: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut fsname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: size_t = 0;
    let mut tmp___0: gpointer = 0 as *mut libc::c_void;
    let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = strlen(fsnameold as *const libc::c_char);
    tmp___0 = g_malloc(
        tmp.wrapping_mul(2 as libc::c_ulong).wrapping_add(1 as libc::c_ulong),
    );
    fsname = tmp___0 as *mut libc::c_char;
    d = fsname;
    s = fsnameold;
    while *s != 0 {
        if *s as libc::c_int == 92 as libc::c_int {
            tmp___1 = d;
            d = d.offset(1);
            *tmp___1 = '\\' as i32 as libc::c_char;
        } else if *s as libc::c_int == 44 as libc::c_int {
            tmp___1 = d;
            d = d.offset(1);
            *tmp___1 = '\\' as i32 as libc::c_char;
        }
        tmp___2 = d;
        d = d.offset(1);
        *tmp___2 = *s;
        s = s.offset(1);
    }
    *d = '\u{0}' as i32 as libc::c_char;
    g_free(fsnameold as gpointer);
    return fsname;
}
unsafe extern "C" fn ssh_connect() -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    res = processing_init();
    if res == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if sshfs.delay_connect == 0 {
        tmp = connect_remote((sshfs.conns).offset(0 as libc::c_int as isize));
        if tmp == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        if sshfs.no_check_root == 0 {
            tmp___0 = sftp_check_root(
                (sshfs.conns).offset(0 as libc::c_int as isize),
                sshfs.base_path as *const libc::c_char,
            );
            if tmp___0 != 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_idmap_line(
    mut line: *mut libc::c_char,
    mut filename: *const libc::c_char,
    lineno: libc::c_uint,
    mut ret_id: *mut uint32_t,
    mut ret_name: *mut *mut libc::c_char,
    eof: libc::c_int,
) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tokens: [*mut libc::c_char; 3] = [0 as *mut libc::c_char; 3];
    let mut tok: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut name_tok: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut id_tok: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut remote_id: uint32_t = 0;
    let mut tmp___0: libc::c_ulong = 0;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    p = line;
    p = strrchr(line as *const libc::c_char, '\n' as i32);
    if !p.is_null() {
        *p = '\u{0}' as i32 as libc::c_char;
    } else if eof == 0 {
        fprintf(
            stderr,
            b"%s:%u: line too long\n\0" as *const u8 as *const libc::c_char,
            filename,
            lineno,
        );
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    loop {
        tok = strsep(
            &mut line as *mut *mut libc::c_char,
            b":\0" as *const u8 as *const libc::c_char,
        );
        if tok.is_null() {
            break;
        }
        if !(i < 3 as libc::c_int) {
            break;
        }
        tokens[i as usize] = tok;
        i += 1;
    }
    if i == 2 as libc::c_int {
        name_tok = tokens[0 as libc::c_int as usize];
        id_tok = tokens[1 as libc::c_int as usize];
    } else if i >= 3 as libc::c_int {
        name_tok = tokens[0 as libc::c_int as usize];
        id_tok = tokens[2 as libc::c_int as usize];
    } else {
        fprintf(
            stderr,
            b"%s:%u: unknown format\n\0" as *const u8 as *const libc::c_char,
            filename,
            lineno,
        );
        exit(1 as libc::c_int);
    }
    tmp = __errno_location();
    *tmp = 0 as libc::c_int;
    tmp___0 = strtoul(
        id_tok as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    remote_id = tmp___0 as uint32_t;
    tmp___3 = __errno_location();
    if *tmp___3 != 0 {
        tmp___1 = __errno_location();
        tmp___2 = strerror(*tmp___1);
        fprintf(
            stderr,
            b"Invalid id number on line %u of '%s': %s\n\0" as *const u8
                as *const libc::c_char,
            lineno,
            filename,
            tmp___2,
        );
        exit(1 as libc::c_int);
    }
    *ret_name = strdup(name_tok as *const libc::c_char);
    *ret_id = remote_id;
}
unsafe extern "C" fn read_id_map(
    mut file: *mut libc::c_char,
    mut map_fn: Option::<unsafe extern "C" fn(*mut libc::c_char) -> *mut uint32_t>,
    mut name_id: *const libc::c_char,
    mut idmap: *mut *mut GHashTable,
    mut r_idmap: *mut *mut GHashTable,
) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut line: [libc::c_char; 2048] = [0; 2048];
    let mut lineno: libc::c_uint = 0;
    let mut local_uid: uid_t = 0;
    let mut tmp: __uid_t = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
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
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut remote_id: uint32_t = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: libc::c_int = 0;
    let mut local_id: *mut uint32_t = 0 as *mut uint32_t;
    let mut tmp___7: *mut uint32_t = 0 as *mut uint32_t;
    let mut tmp___8: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___9: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___10: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___11: libc::c_int = 0;
    *idmap = g_hash_table_new(
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(gconstpointer) -> guint>,
        >(0 as *mut libc::c_void),
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>,
        >(0 as *mut libc::c_void),
    );
    *r_idmap = g_hash_table_new(
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(gconstpointer) -> guint>,
        >(0 as *mut libc::c_void),
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>,
        >(0 as *mut libc::c_void),
    );
    lineno = 0 as libc::c_uint;
    tmp = getuid();
    local_uid = tmp;
    fp = fopen(file as *const libc::c_char, b"r\0" as *const u8 as *const libc::c_char);
    if fp as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___0 = __errno_location();
        tmp___1 = strerror(*tmp___0);
        fprintf(
            stderr,
            b"failed to open '%s': %s\n\0" as *const u8 as *const libc::c_char,
            file,
            tmp___1,
        );
        exit(1 as libc::c_int);
    }
    tmp___4 = fileno(fp);
    tmp___5 = fstat(tmp___4, &mut st);
    if tmp___5 == -(1 as libc::c_int) {
        tmp___2 = __errno_location();
        tmp___3 = strerror(*tmp___2);
        fprintf(
            stderr,
            b"failed to stat '%s': %s\n\0" as *const u8 as *const libc::c_char,
            file,
            tmp___3,
        );
        exit(1 as libc::c_int);
    }
    if st.st_uid != local_uid {
        fprintf(
            stderr,
            b"'%s' is not owned by uid %lu\n\0" as *const u8 as *const libc::c_char,
            file,
            local_uid as libc::c_ulong,
        );
        exit(1 as libc::c_int);
    }
    if st.st_mode & (128 as libc::c_int >> 3 as libc::c_int) as libc::c_uint != 0 {
        fprintf(
            stderr,
            b"'%s' is writable by other users\n\0" as *const u8 as *const libc::c_char,
            file,
        );
        exit(1 as libc::c_int);
    } else {
        if st.st_mode
            & (128 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                as libc::c_uint != 0
        {
            fprintf(
                stderr,
                b"'%s' is writable by other users\n\0" as *const u8
                    as *const libc::c_char,
                file,
            );
            exit(1 as libc::c_int);
        }
    }
    loop {
        tmp___8 = fgets(line.as_mut_ptr(), 2048 as libc::c_int, fp);
        if !(tmp___8 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
            break;
        }
        lineno = lineno.wrapping_add(1);
        if line[0 as libc::c_int as usize] as libc::c_int == 10 as libc::c_int {
            continue;
        }
        if line[0 as libc::c_int as usize] as libc::c_int == 0 as libc::c_int {
            continue;
        }
        tmp___6 = feof(fp);
        parse_idmap_line(
            line.as_mut_ptr(),
            file as *const libc::c_char,
            lineno,
            &mut remote_id,
            &mut name,
            tmp___6,
        );
        tmp___7 = (Some(map_fn.expect("non-null function pointer")))
            .expect("non-null function pointer")(name);
        local_id = tmp___7;
        if local_id as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            if sshfs.debug != 0 {
                fprintf(
                    stderr,
                    b"%s(%u): no local %s\n\0" as *const u8 as *const libc::c_char,
                    name,
                    remote_id,
                    name_id,
                );
            }
            free(name as *mut libc::c_void);
        } else {
            if sshfs.debug != 0 {
                fprintf(
                    stderr,
                    b"%s: remote %s %u => local %s %u\n\0" as *const u8
                        as *const libc::c_char,
                    name,
                    name_id,
                    remote_id,
                    name_id,
                    *local_id,
                );
            }
            g_hash_table_insert(
                *idmap,
                remote_id as gulong as gpointer,
                *local_id as gulong as gpointer,
            );
            g_hash_table_insert(
                *r_idmap,
                *local_id as gulong as gpointer,
                remote_id as gulong as gpointer,
            );
            free(name as *mut libc::c_void);
            free(local_id as *mut libc::c_void);
        }
    }
    tmp___11 = fclose(fp);
    if tmp___11 == -(1 as libc::c_int) {
        tmp___9 = __errno_location();
        tmp___10 = strerror(*tmp___9);
        fprintf(
            stderr,
            b"failed to close '%s': %s\0" as *const u8 as *const libc::c_char,
            file,
            tmp___10,
        );
        exit(1 as libc::c_int);
    }
}
unsafe extern "C" fn username_to_uid(mut name: *mut libc::c_char) -> *mut uint32_t {
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pw: *mut passwd = 0 as *mut passwd;
    let mut tmp___0: *mut passwd = 0 as *mut passwd;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut uint32_t = 0 as *mut uint32_t;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = __errno_location();
    *tmp = 0 as libc::c_int;
    tmp___0 = getpwnam(name as *const libc::c_char);
    pw = tmp___0;
    if pw as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___1 = __errno_location();
        if *tmp___1 == 0 as libc::c_int {
            return 0 as *mut libc::c_void as *mut uint32_t;
        }
        tmp___2 = __errno_location();
        tmp___3 = strerror(*tmp___2);
        fprintf(
            stderr,
            b"Failed to look up user '%s': %s\n\0" as *const u8 as *const libc::c_char,
            name,
            tmp___3,
        );
        exit(1 as libc::c_int);
    }
    tmp___4 = malloc(::std::mem::size_of::<uint32_t>() as libc::c_ulong);
    r = tmp___4 as *mut uint32_t;
    if r as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"sshfs: memory allocation failed\n\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    *r = (*pw).pw_uid;
    return r;
}
unsafe extern "C" fn groupname_to_gid(mut name: *mut libc::c_char) -> *mut uint32_t {
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut gr: *mut group = 0 as *mut group;
    let mut tmp___0: *mut group = 0 as *mut group;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut uint32_t = 0 as *mut uint32_t;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = __errno_location();
    *tmp = 0 as libc::c_int;
    tmp___0 = getgrnam(name as *const libc::c_char);
    gr = tmp___0;
    if gr as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___1 = __errno_location();
        if *tmp___1 == 0 as libc::c_int {
            return 0 as *mut libc::c_void as *mut uint32_t;
        }
        tmp___2 = __errno_location();
        tmp___3 = strerror(*tmp___2);
        fprintf(
            stderr,
            b"Failed to look up group '%s': %s\n\0" as *const u8 as *const libc::c_char,
            name,
            tmp___3,
        );
        exit(1 as libc::c_int);
    }
    tmp___4 = malloc(::std::mem::size_of::<uint32_t>() as libc::c_ulong);
    r = tmp___4 as *mut uint32_t;
    if r as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"sshfs: memory allocation failed\n\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    *r = (*gr).gr_gid;
    return r;
}
#[inline]
unsafe extern "C" fn load_uid_map() {
    read_id_map(
        sshfs.uid_file,
        Some(
            username_to_uid as unsafe extern "C" fn(*mut libc::c_char) -> *mut uint32_t,
        ),
        b"uid\0" as *const u8 as *const libc::c_char,
        &mut sshfs.uid_map,
        &mut sshfs.r_uid_map,
    );
}
#[inline]
unsafe extern "C" fn load_gid_map() {
    read_id_map(
        sshfs.gid_file,
        Some(
            groupname_to_gid as unsafe extern "C" fn(*mut libc::c_char) -> *mut uint32_t,
        ),
        b"gid\0" as *const u8 as *const libc::c_char,
        &mut sshfs.gid_map,
        &mut sshfs.r_gid_map,
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut args: fuse_args = fuse_args {
        argc: 0,
        argv: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        allocated: 0,
    };
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fsname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sftp_server: *const libc::c_char = 0 as *const libc::c_char;
    let mut fuse: *mut fuse = 0 as *mut fuse;
    let mut se: *mut fuse_session = 0 as *mut fuse_session;
    let mut i: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: *const libc::c_char = 0 as *const libc::c_char;
    let mut __n: gsize = 0;
    let mut __s: gsize = 0;
    let mut __p: gpointer = 0 as *mut libc::c_void;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___7: time_t = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut avg_rtt: libc::c_uint = 0;
    args.argc = argc;
    args.argv = argv;
    args.allocated = 0 as libc::c_int;
    sshfs.blksize = 4096 as libc::c_uint;
    sshfs.max_read = 32768 as libc::c_uint;
    sshfs.max_write = 32768 as libc::c_uint;
    sshfs.rename_workaround = 0 as libc::c_int;
    sshfs.renamexdev_workaround = 0 as libc::c_int;
    sshfs.truncate_workaround = 0 as libc::c_int;
    sshfs.buflimit_workaround = 0 as libc::c_int;
    sshfs.createmode_workaround = 0 as libc::c_int;
    sshfs.ssh_ver = 2 as libc::c_uint;
    sshfs.progname = *argv.offset(0 as libc::c_int as isize);
    sshfs.max_conns = 1 as libc::c_int;
    sshfs.ptyfd = -(1 as libc::c_int);
    sshfs.dir_cache = 1 as libc::c_int;
    sshfs.show_help = 0 as libc::c_int;
    sshfs.show_version = 0 as libc::c_int;
    sshfs.singlethread = 0 as libc::c_int;
    sshfs.foreground = 0 as libc::c_int;
    sshfs.ptypassivefd = -(1 as libc::c_int);
    sshfs.delay_connect = 0 as libc::c_int;
    sshfs.passive = 0 as libc::c_int;
    sshfs.detect_uid = 0 as libc::c_int;
    tmp___1 = strcmp(
        b"none\0" as *const u8 as *const libc::c_char,
        b"none\0" as *const u8 as *const libc::c_char,
    );
    if tmp___1 == 0 as libc::c_int {
        sshfs.idmap = 0 as libc::c_int;
    } else {
        tmp___0 = strcmp(
            b"none\0" as *const u8 as *const libc::c_char,
            b"user\0" as *const u8 as *const libc::c_char,
        );
        if tmp___0 == 0 as libc::c_int {
            sshfs.idmap = 1 as libc::c_int;
        } else {
            fprintf(
                stderr,
                b"bad idmap default value built into sshfs; assuming none (bad logic in configure script?)\n\0"
                    as *const u8 as *const libc::c_char,
            );
            sshfs.idmap = 0 as libc::c_int;
        }
    }
    sshfs.nomap = 1 as libc::c_int;
    ssh_add_arg(b"ssh\0" as *const u8 as *const libc::c_char);
    ssh_add_arg(b"-x\0" as *const u8 as *const libc::c_char);
    ssh_add_arg(b"-a\0" as *const u8 as *const libc::c_char);
    ssh_add_arg(b"-oClearAllForwardings=yes\0" as *const u8 as *const libc::c_char);
    tmp___2 = fuse_opt_parse(
        &mut args,
        &mut sshfs as *mut sshfs as *mut libc::c_void,
        sshfs_opts.as_mut_ptr() as *const fuse_opt,
        Some(
            sshfs_opt_proc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    libc::c_int,
                    *mut fuse_args,
                ) -> libc::c_int,
        ),
    );
    if tmp___2 == -(1 as libc::c_int) {
        exit(1 as libc::c_int);
    } else {
        tmp___3 = parse_workarounds();
        if tmp___3 == -(1 as libc::c_int) {
            exit(1 as libc::c_int);
        }
    }
    if sshfs.show_version != 0 {
        printf(
            b"SSHFS version %s\n\0" as *const u8 as *const libc::c_char,
            b"3.7.3\0" as *const u8 as *const libc::c_char,
        );
        tmp___4 = fuse_pkgversion();
        printf(
            b"FUSE library version %s\n\0" as *const u8 as *const libc::c_char,
            tmp___4,
        );
        fuse_lowlevel_version();
        exit(0 as libc::c_int);
    }
    if sshfs.show_help != 0 {
        usage(*(args.argv).offset(0 as libc::c_int as isize) as *const libc::c_char);
        fuse_lib_help(&mut args);
        exit(0 as libc::c_int);
    } else {
        if (sshfs.host).is_null() {
            fprintf(stderr, b"missing host\n\0" as *const u8 as *const libc::c_char);
            fprintf(
                stderr,
                b"see `%s -h' for usage\n\0" as *const u8 as *const libc::c_char,
                *argv.offset(0 as libc::c_int as isize),
            );
            exit(1 as libc::c_int);
        } else {
            if (sshfs.mountpoint).is_null() {
                fprintf(
                    stderr,
                    b"error: no mountpoint specified\n\0" as *const u8
                        as *const libc::c_char,
                );
                fprintf(
                    stderr,
                    b"see `%s -h' for usage\n\0" as *const u8 as *const libc::c_char,
                    *argv.offset(0 as libc::c_int as isize),
                );
                exit(1 as libc::c_int);
            }
        }
    }
    if sshfs.idmap == 1 as libc::c_int {
        sshfs.detect_uid = 1 as libc::c_int;
    } else if sshfs.idmap == 2 as libc::c_int {
        sshfs.uid_map = 0 as *mut libc::c_void as *mut GHashTable;
        sshfs.gid_map = 0 as *mut libc::c_void as *mut GHashTable;
        sshfs.r_uid_map = 0 as *mut libc::c_void as *mut GHashTable;
        sshfs.r_gid_map = 0 as *mut libc::c_void as *mut GHashTable;
        if (sshfs.uid_file).is_null() {
            if (sshfs.gid_file).is_null() {
                fprintf(
                    stderr,
                    b"need a uidfile or gidfile with idmap=file\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
        }
        if !(sshfs.uid_file).is_null() {
            load_uid_map();
        }
        if !(sshfs.gid_file).is_null() {
            load_gid_map();
        }
    }
    free(sshfs.uid_file as *mut libc::c_void);
    free(sshfs.gid_file as *mut libc::c_void);
    if sshfs.debug != 0 {
        fprintf(
            stderr,
            b"SSHFS version %s\n\0" as *const u8 as *const libc::c_char,
            b"3.7.3\0" as *const u8 as *const libc::c_char,
        );
    }
    if sshfs.passive != 0 {
        sshfs.foreground = 1 as libc::c_int;
    }
    if sshfs.passive != 0 {
        if sshfs.password_stdin != 0 {
            fprintf(
                stderr,
                b"the password_stdin and passive options cannot both be specified\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    if sshfs.password_stdin != 0 {
        res = read_password();
        if res == -(1 as libc::c_int) {
            exit(1 as libc::c_int);
        }
    }
    if sshfs.debug != 0 {
        sshfs.foreground = 1 as libc::c_int;
    }
    if sshfs.buflimit_workaround != 0 {
        sshfs.max_outstanding_len = 8388608 as libc::c_uint;
    } else {
        sshfs.max_outstanding_len = !(0 as libc::c_int) as libc::c_uint;
    }
    if sshfs.max_conns > 1 as libc::c_int {
        if sshfs.buflimit_workaround != 0 {
            fprintf(
                stderr,
                b"buflimit workaround is not supported with parallel connections\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        if sshfs.password_stdin != 0 {
            fprintf(
                stderr,
                b"password_stdin option cannot be specified with parallel connections\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        if sshfs.passive != 0 {
            fprintf(
                stderr,
                b"passive option cannot be specified with parallel connections\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    } else if sshfs.max_conns <= 0 as libc::c_int {
        fprintf(
            stderr,
            b"value of max_conns option must be at least 1\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    __n = sshfs.max_conns as gsize;
    __s = ::std::mem::size_of::<conn>() as libc::c_ulong;
    if __s == 1 as libc::c_ulong {
        __p = g_malloc0(__n);
    } else {
        __p = g_malloc0_n(__n, __s);
    }
    sshfs.conns = __p as *mut conn;
    i = 0 as libc::c_int;
    while i < sshfs.max_conns {
        (*(sshfs.conns).offset(i as isize)).rfd = -(1 as libc::c_int);
        (*(sshfs.conns).offset(i as isize)).wfd = -(1 as libc::c_int);
        i += 1;
    }
    fsname = g_strdup(sshfs.host as *const gchar);
    tmp___5 = find_base_path();
    sshfs.base_path = g_strdup(tmp___5 as *const gchar);
    if !(sshfs.ssh_command).is_null() {
        set_ssh_command();
    }
    tmp = g_strdup_printf(b"-%i\0" as *const u8 as *const libc::c_char, sshfs.ssh_ver);
    ssh_add_arg(tmp as *const libc::c_char);
    g_free(tmp as gpointer);
    ssh_add_arg(sshfs.host as *const libc::c_char);
    if !(sshfs.sftp_server).is_null() {
        sftp_server = sshfs.sftp_server as *const libc::c_char;
    } else if sshfs.ssh_ver == 1 as libc::c_uint {
        sftp_server = b"/usr/lib/sftp-server\0" as *const u8 as *const libc::c_char;
    } else {
        sftp_server = b"sftp\0" as *const u8 as *const libc::c_char;
    }
    if sshfs.ssh_ver != 1 as libc::c_uint {
        tmp___6 = strchr(sftp_server, '/' as i32);
        if tmp___6 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            ssh_add_arg(b"-s\0" as *const u8 as *const libc::c_char);
        }
    }
    ssh_add_arg(sftp_server);
    free(sshfs.sftp_server as *mut libc::c_void);
    res = cache_parse_options(&mut args);
    if res == -(1 as libc::c_int) {
        exit(1 as libc::c_int);
    }
    tmp___7 = time(0 as *mut time_t);
    sshfs.randseed = tmp___7 as libc::c_uint;
    if sshfs.max_read > 65536 as libc::c_uint {
        sshfs.max_read = 65536 as libc::c_uint;
    }
    if sshfs.max_write > 65536 as libc::c_uint {
        sshfs.max_write = 65536 as libc::c_uint;
    }
    fsname = fsname_escape_commas(fsname);
    tmp = g_strdup_printf(
        b"-osubtype=sshfs,fsname=%s\0" as *const u8 as *const libc::c_char,
        fsname,
    );
    fuse_opt_insert_arg(&mut args, 1 as libc::c_int, tmp as *const libc::c_char);
    g_free(tmp as gpointer);
    g_free(fsname as gpointer);
    if sshfs.dir_cache != 0 {
        sshfs.op = cache_wrap(&mut sshfs_oper);
    } else {
        sshfs.op = &mut sshfs_oper;
    }
    fuse = fuse_new(
        &mut args,
        sshfs.op as *const fuse_operations,
        ::std::mem::size_of::<fuse_operations>() as libc::c_ulong,
        0 as *mut libc::c_void,
    );
    if fuse as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        exit(1 as libc::c_int);
    }
    se = fuse_get_session(fuse);
    res = fuse_set_signal_handlers(se);
    if res != 0 as libc::c_int {
        fuse_destroy(fuse);
        exit(1 as libc::c_int);
    }
    res = fuse_mount(fuse, sshfs.mountpoint as *const libc::c_char);
    if res != 0 as libc::c_int {
        fuse_destroy(fuse);
        exit(1 as libc::c_int);
    }
    tmp___8 = fuse_session_fd(se);
    res = fcntl(tmp___8, 2 as libc::c_int, 1 as libc::c_int);
    if res == -(1 as libc::c_int) {
        perror(
            b"WARNING: failed to set FD_CLOEXEC on fuse device\0" as *const u8
                as *const libc::c_char,
        );
    }
    res = ssh_connect();
    if res == -(1 as libc::c_int) {
        fuse_unmount(fuse);
        fuse_destroy(fuse);
        exit(1 as libc::c_int);
    }
    res = fuse_daemonize(sshfs.foreground);
    if res == -(1 as libc::c_int) {
        fuse_unmount(fuse);
        fuse_destroy(fuse);
        exit(1 as libc::c_int);
    }
    if sshfs.singlethread != 0 {
        res = fuse_loop(fuse);
    } else {
        res = fuse_loop_mt_31(fuse, 0 as libc::c_int);
    }
    if res != 0 as libc::c_int {
        res = 1 as libc::c_int;
    } else {
        res = 0 as libc::c_int;
    }
    fuse_remove_signal_handlers(se);
    fuse_unmount(fuse);
    fuse_destroy(fuse);
    if sshfs.debug != 0 {
        avg_rtt = 0 as libc::c_uint;
        if sshfs.num_sent != 0 {
            avg_rtt = (sshfs.total_rtt).wrapping_div(sshfs.num_sent) as libc::c_uint;
        }
        if sshfs.debug != 0 {
            fprintf(
                stderr,
                b"\nsent:               %llu messages, %llu bytes\nreceived:           %llu messages, %llu bytes\nrtt min/max/avg:    %ums/%ums/%ums\nnum connect:        %u\n\0"
                    as *const u8 as *const libc::c_char,
                sshfs.num_sent as libc::c_ulonglong,
                sshfs.bytes_sent as libc::c_ulonglong,
                sshfs.num_received as libc::c_ulonglong,
                sshfs.bytes_received as libc::c_ulonglong,
                sshfs.min_rtt,
                sshfs.max_rtt,
                avg_rtt,
                sshfs.num_connect,
            );
        }
    }
    fuse_opt_free_args(&mut args);
    fuse_opt_free_args(&mut sshfs.ssh_args);
    free(sshfs.directport as *mut libc::c_void);
    return res;
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
unsafe extern "C" fn run_static_initializers() {
    cache_opts = [
        {
            let mut init = fuse_opt {
                templ: b"dcache_timeout=%u\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut cache)).stat_timeout_secs as *mut libc::c_uint
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"dcache_timeout=%u\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut cache)).dir_timeout_secs as *mut libc::c_uint
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"dcache_timeout=%u\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut cache)).link_timeout_secs as *mut libc::c_uint
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"dcache_stat_timeout=%u\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut cache)).stat_timeout_secs as *mut libc::c_uint
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"dcache_dir_timeout=%u\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut cache)).dir_timeout_secs as *mut libc::c_uint
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"dcache_link_timeout=%u\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut cache)).link_timeout_secs as *mut libc::c_uint
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"dcache_max_size=%u\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut cache)).max_size as *mut libc::c_uint
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"dcache_clean_interval=%u\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut cache)).clean_interval_secs
                    as *mut libc::c_uint as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"dcache_min_clean_interval=%u\0" as *const u8
                    as *const libc::c_char,
                offset: &mut (*(0 as *mut cache)).min_clean_interval_secs
                    as *mut libc::c_uint as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"cache_timeout=%u\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut cache)).stat_timeout_secs as *mut libc::c_uint
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"cache_timeout=%u\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut cache)).dir_timeout_secs as *mut libc::c_uint
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"cache_timeout=%u\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut cache)).link_timeout_secs as *mut libc::c_uint
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"cache_stat_timeout=%u\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut cache)).stat_timeout_secs as *mut libc::c_uint
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"cache_dir_timeout=%u\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut cache)).dir_timeout_secs as *mut libc::c_uint
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"cache_link_timeout=%u\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut cache)).link_timeout_secs as *mut libc::c_uint
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"cache_max_size=%u\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut cache)).max_size as *mut libc::c_uint
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"cache_clean_interval=%u\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut cache)).clean_interval_secs
                    as *mut libc::c_uint as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"cache_min_clean_interval=%u\0" as *const u8
                    as *const libc::c_char,
                offset: &mut (*(0 as *mut cache)).min_clean_interval_secs
                    as *mut libc::c_uint as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: 0 as *mut libc::c_void as *const libc::c_char,
                offset: 0 as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
    ];
    sshfs_opts = [
        {
            let mut init = fuse_opt {
                templ: b"directport=%s\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).directport as *mut *mut libc::c_char
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"ssh_command=%s\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).ssh_command as *mut *mut libc::c_char
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"sftp_server=%s\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).sftp_server as *mut *mut libc::c_char
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"max_read=%u\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).max_read as *mut libc::c_uint
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"max_write=%u\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).max_write as *mut libc::c_uint
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"ssh_protocol=%u\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).ssh_ver as *mut libc::c_uint
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"-1\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).ssh_ver as *mut libc::c_uint
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"workaround=%s\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).workarounds as *mut *mut libc::c_char
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"idmap=none\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).idmap as *mut libc::c_int
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"idmap=user\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).idmap as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"idmap=file\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).idmap as *mut libc::c_int
                    as libc::c_ulong,
                value: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"uidfile=%s\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).uid_file as *mut *mut libc::c_char
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"gidfile=%s\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).gid_file as *mut *mut libc::c_char
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"nomap=ignore\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).nomap as *mut libc::c_int
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"nomap=error\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).nomap as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"sshfs_sync\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).sync_write as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"no_readahead\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).sync_read as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"sync_readdir\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).sync_readdir as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"sshfs_debug\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).debug as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"sshfs_verbose\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).verbose as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"reconnect\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).reconnect as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"transform_symlinks\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).transform_symlinks as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"follow_symlinks\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).follow_symlinks as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"no_check_root\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).no_check_root as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"password_stdin\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).password_stdin as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"delay_connect\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).delay_connect as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"slave\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).passive as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"passive\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).passive as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"disable_hardlink\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).disable_hardlink as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"dir_cache=yes\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).dir_cache as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"dir_cache=no\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).dir_cache as *mut libc::c_int
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"direct_io\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).direct_io as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"max_conns=%u\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).max_conns as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"-h\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).show_help as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"--help\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).show_help as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"-V\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).show_version as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"--version\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).show_version as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"-d\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).debug as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"debug\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).debug as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"-v\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).verbose as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"verbose\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).verbose as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"-f\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).foreground as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"-s\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).singlethread as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"-p \0" as *const u8 as *const libc::c_char,
                offset: 4294967295 as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"-C\0" as *const u8 as *const libc::c_char,
                offset: 4294967295 as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"-F \0" as *const u8 as *const libc::c_char,
                offset: 4294967295 as libc::c_ulong,
                value: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"cache=yes\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).dir_cache as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"cache=no\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).dir_cache as *mut libc::c_int
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"writeback_cache=no\0" as *const u8 as *const libc::c_char,
                offset: 4294967295 as libc::c_ulong,
                value: -(4 as libc::c_int),
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"unreliable_append\0" as *const u8 as *const libc::c_char,
                offset: 4294967295 as libc::c_ulong,
                value: -(4 as libc::c_int),
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"auto\0" as *const u8 as *const libc::c_char,
                offset: 4294967295 as libc::c_ulong,
                value: -(4 as libc::c_int),
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"noauto\0" as *const u8 as *const libc::c_char,
                offset: 4294967295 as libc::c_ulong,
                value: -(4 as libc::c_int),
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"user\0" as *const u8 as *const libc::c_char,
                offset: 4294967295 as libc::c_ulong,
                value: -(4 as libc::c_int),
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"nouser\0" as *const u8 as *const libc::c_char,
                offset: 4294967295 as libc::c_ulong,
                value: -(4 as libc::c_int),
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"users\0" as *const u8 as *const libc::c_char,
                offset: 4294967295 as libc::c_ulong,
                value: -(4 as libc::c_int),
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"_netdev\0" as *const u8 as *const libc::c_char,
                offset: 4294967295 as libc::c_ulong,
                value: -(4 as libc::c_int),
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: 0 as *mut libc::c_void as *const libc::c_char,
                offset: 0 as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
    ];
    workaround_opts = [
        {
            let mut init = fuse_opt {
                templ: b"none\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).rename_workaround as *mut libc::c_int
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"none\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).truncate_workaround as *mut libc::c_int
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"none\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).buflimit_workaround as *mut libc::c_int
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"none\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).fstat_workaround as *mut libc::c_int
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"rename\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).rename_workaround as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"norename\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).rename_workaround as *mut libc::c_int
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"renamexdev\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).renamexdev_workaround
                    as *mut libc::c_int as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"norenamexdev\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).renamexdev_workaround
                    as *mut libc::c_int as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"truncate\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).truncate_workaround as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"notruncate\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).truncate_workaround as *mut libc::c_int
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"buflimit\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).buflimit_workaround as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"nobuflimit\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).buflimit_workaround as *mut libc::c_int
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"fstat\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).fstat_workaround as *mut libc::c_int
                    as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"nofstat\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).fstat_workaround as *mut libc::c_int
                    as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"createmode\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).createmode_workaround
                    as *mut libc::c_int as libc::c_ulong,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"nocreatemode\0" as *const u8 as *const libc::c_char,
                offset: &mut (*(0 as *mut sshfs)).createmode_workaround
                    as *mut libc::c_int as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: 0 as *mut libc::c_void as *const libc::c_char,
                offset: 0 as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
