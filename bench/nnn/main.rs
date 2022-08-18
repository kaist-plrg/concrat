use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type __dirstream;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;
    pub type screen;
    pub type ldat;
    fn inotify_init1(__flags: libc::c_int) -> libc::c_int;
    fn inotify_add_watch(
        __fd: libc::c_int,
        __name: *const libc::c_char,
        __mask: uint32_t,
    ) -> libc::c_int;
    fn inotify_rm_watch(__fd: libc::c_int, __wd: libc::c_int) -> libc::c_int;
    fn getrlimit(__resource: __rlimit_resource_t, __rlimits: *mut rlimit) -> libc::c_int;
    fn setrlimit(
        __resource: __rlimit_resource_t,
        __rlimits: *const rlimit,
    ) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn fstatat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __buf: *mut stat,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn mkfifo(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn statvfs(__file: *const libc::c_char, __buf: *mut statvfs) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn dirfd(__dirp: *mut DIR) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn posix_fadvise(
        __fd: libc::c_int,
        __offset: off_t,
        __len: off_t,
        __advise: libc::c_int,
    ) -> libc::c_int;
    fn fts_close(_: *mut FTS) -> libc::c_int;
    fn fts_open(
        _: *const *mut libc::c_char,
        _: libc::c_int,
        _: Option::<
            unsafe extern "C" fn(*mut *const FTSENT, *mut *const FTSENT) -> libc::c_int,
        >,
    ) -> *mut FTS;
    fn fts_read(_: *mut FTS) -> *mut FTSENT;
    fn time(__timer: *mut time_t) -> time_t;
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn renameat(
        __oldfd: libc::c_int,
        __old: *const libc::c_char,
        __newfd: libc::c_int,
        __new: *const libc::c_char,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn dprintf(__fd: libc::c_int, __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn ctermid(__s: *mut libc::c_char) -> *mut libc::c_char;
    fn regcomp(
        __preg: *mut regex_t,
        __pattern: *const libc::c_char,
        __cflags: libc::c_int,
    ) -> libc::c_int;
    fn regexec(
        __preg: *const regex_t,
        __String: *const libc::c_char,
        __nmatch: size_t,
        __pmatch: *mut regmatch_t,
        __eflags: libc::c_int,
    ) -> libc::c_int;
    fn regfree(__preg: *mut regex_t);
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn strtoll(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_longlong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
    fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
    fn mkdtemp(__template: *mut libc::c_char) -> *mut libc::c_char;
    fn realpath(
        __name: *const libc::c_char,
        __resolved: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: Option::<
            unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        >,
    );
    fn mbstowcs(__pwcs: *mut wchar_t, __s: *const libc::c_char, __n: size_t) -> size_t;
    fn wcstombs(__s: *mut libc::c_char, __pwcs: *const wchar_t, __n: size_t) -> size_t;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memccpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn rawmemchr(__s: *const libc::c_void, __c: libc::c_int) -> *mut libc::c_void;
    fn memrchr(
        __s: *const libc::c_void,
        __c: libc::c_int,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcasestr(
        __haystack: *const libc::c_char,
        __needle: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn memmem(
        __haystack: *const libc::c_void,
        __haystacklen: size_t,
        __needle: *const libc::c_void,
        __needlelen: size_t,
    ) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn ffs(__i: libc::c_int) -> libc::c_int;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn getpid() -> __pid_t;
    fn getppid() -> __pid_t;
    fn setsid() -> __pid_t;
    fn fork() -> __pid_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn link(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;
    fn symlink(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;
    fn readlink(
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: size_t,
    ) -> ssize_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn rmdir(__path: *const libc::c_char) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn wcswidth(__s: *const wchar_t, __n: size_t) -> libc::c_int;
    fn cbreak() -> libc::c_int;
    fn curs_set(_: libc::c_int) -> libc::c_int;
    fn endwin() -> libc::c_int;
    fn initscr() -> *mut WINDOW;
    fn init_pair(_: libc::c_short, _: libc::c_short, _: libc::c_short) -> libc::c_int;
    fn keyname(_: libc::c_int) -> *const libc::c_char;
    fn keypad(_: *mut WINDOW, _: bool) -> libc::c_int;
    fn newterm(_: *const libc::c_char, _: *mut FILE, _: *mut FILE) -> *mut SCREEN;
    fn noecho() -> libc::c_int;
    fn nonl() -> libc::c_int;
    fn printw(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn start_color() -> libc::c_int;
    fn waddch(_: *mut WINDOW, _: chtype) -> libc::c_int;
    fn waddnstr(_: *mut WINDOW, _: *const libc::c_char, _: libc::c_int) -> libc::c_int;
    fn wattr_on(_: *mut WINDOW, _: attr_t, _: *mut libc::c_void) -> libc::c_int;
    fn wattr_off(_: *mut WINDOW, _: attr_t, _: *mut libc::c_void) -> libc::c_int;
    fn wclrtoeol(_: *mut WINDOW) -> libc::c_int;
    fn werase(_: *mut WINDOW) -> libc::c_int;
    fn wmove(_: *mut WINDOW, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn wrefresh(_: *mut WINDOW) -> libc::c_int;
    fn wtimeout(_: *mut WINDOW, _: libc::c_int);
    fn set_escdelay(_: libc::c_int) -> libc::c_int;
    fn use_default_colors() -> libc::c_int;
    static mut stdscr: *mut WINDOW;
    static mut COLORS: libc::c_int;
    static mut COLS: libc::c_int;
    static mut LINES: libc::c_int;
    fn unget_wch(_: wchar_t) -> libc::c_int;
    fn waddnwstr(_: *mut WINDOW, _: *const wchar_t, _: libc::c_int) -> libc::c_int;
    fn wget_wch(_: *mut WINDOW, _: *mut wint_t) -> libc::c_int;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
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
pub type __rlim_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __fsblkcnt_t = libc::c_ulong;
pub type __fsfilcnt_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type int8_t = __int8_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inotify_event {
    pub wd: libc::c_int,
    pub mask: uint32_t,
    pub cookie: uint32_t,
    pub len: uint32_t,
    pub name: [libc::c_char; 0],
}
pub type __rlimit_resource = libc::c_uint;
pub const __RLIM_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __RLIMIT_NICE: __rlimit_resource = 13;
pub const __RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __RLIMIT_NPROC: __rlimit_resource = 6;
pub const RLIMIT_AS: __rlimit_resource = 9;
pub const __RLIMIT_OFILE: __rlimit_resource = 7;
pub const RLIMIT_NOFILE: __rlimit_resource = 7;
pub const __RLIMIT_RSS: __rlimit_resource = 5;
pub const RLIMIT_CORE: __rlimit_resource = 4;
pub const RLIMIT_STACK: __rlimit_resource = 3;
pub const RLIMIT_DATA: __rlimit_resource = 2;
pub const RLIMIT_FSIZE: __rlimit_resource = 1;
pub const RLIMIT_CPU: __rlimit_resource = 0;
pub type rlim_t = __rlim_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
pub type __rlimit_resource_t = __rlimit_resource;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type time_t = __time_t;
pub type dev_t = __dev_t;
pub type ino_t = __ino_t;
pub type mode_t = __mode_t;
pub type nlink_t = __nlink_t;
pub type off_t = __off_t;
pub type blkcnt_t = __blkcnt_t;
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
    pub f_blocks: __fsblkcnt_t,
    pub f_bfree: __fsblkcnt_t,
    pub f_bavail: __fsblkcnt_t,
    pub f_files: __fsfilcnt_t,
    pub f_ffree: __fsfilcnt_t,
    pub f_favail: __fsfilcnt_t,
    pub f_fsid: libc::c_ulong,
    pub f_flag: libc::c_ulong,
    pub f_namemax: libc::c_ulong,
    pub __f_spare: [libc::c_int; 6],
}
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___sigset_t_991265788 {
    pub __val: [libc::c_ulong; 16],
}
pub type __sigset_t = __anonstruct___sigset_t_991265788;
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
pub union __anonunion__bounds_432680557 {
    pub _addr_bnd: __anonstruct__addr_bnd_5259977,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct__sigfault_40845203 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: __anonunion__bounds_432680557,
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
pub union __anonunion__sifields_85651287 {
    pub _pad: [libc::c_int; 28],
    pub _kill: __anonstruct__kill_244518854,
    pub _timer: __anonstruct__timer_490064738,
    pub _rt: __anonstruct__rt_619254530,
    pub _sigchld: __anonstruct__sigchld_284671705,
    pub _sigfault: __anonstruct__sigfault_40845203,
    pub _sigpoll: __anonstruct__sigpoll_386613454,
    pub _sigsys: __anonstruct__sigsys_44812255,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_siginfo_t_27519928 {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: __anonunion__sifields_85651287,
}
pub type siginfo_t = __anonstruct_siginfo_t_27519928;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
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
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
pub type ssize_t = __ssize_t;
pub type useconds_t = __useconds_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ftsent {
    pub fts_cycle: *mut _ftsent,
    pub fts_parent: *mut _ftsent,
    pub fts_link: *mut _ftsent,
    pub fts_number: libc::c_long,
    pub fts_pointer: *mut libc::c_void,
    pub fts_accpath: *mut libc::c_char,
    pub fts_path: *mut libc::c_char,
    pub fts_errno: libc::c_int,
    pub fts_symfd: libc::c_int,
    pub fts_pathlen: libc::c_ushort,
    pub fts_namelen: libc::c_ushort,
    pub fts_ino: ino_t,
    pub fts_dev: dev_t,
    pub fts_nlink: nlink_t,
    pub fts_level: libc::c_short,
    pub fts_info: libc::c_ushort,
    pub fts_flags: libc::c_ushort,
    pub fts_instr: libc::c_ushort,
    pub fts_statp: *mut stat,
    pub fts_name: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_FTS_272296835 {
    pub fts_cur: *mut _ftsent,
    pub fts_child: *mut _ftsent,
    pub fts_array: *mut *mut _ftsent,
    pub fts_dev: dev_t,
    pub fts_path: *mut libc::c_char,
    pub fts_rfd: libc::c_int,
    pub fts_pathlen: libc::c_int,
    pub fts_nitems: libc::c_int,
    pub fts_compar: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
    pub fts_options: libc::c_int,
}
pub type FTS = __anonstruct_FTS_272296835;
pub type FTSENT = _ftsent;
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
pub type __re_long_size_t = libc::c_ulong;
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut libc::c_char,
    pub translate: *mut libc::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_regmatch_t_1035675074 {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
pub type regmatch_t = __anonstruct_regmatch_t_1035675074;
pub type wchar_t = libc::c_int;
pub type chtype = libc::c_uint;
pub type SCREEN = screen;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _win_st {
    pub _cury: libc::c_short,
    pub _curx: libc::c_short,
    pub _maxy: libc::c_short,
    pub _maxx: libc::c_short,
    pub _begy: libc::c_short,
    pub _begx: libc::c_short,
    pub _flags: libc::c_short,
    pub _attrs: attr_t,
    pub _bkgd: chtype,
    pub _notimeout: bool,
    pub _clear: bool,
    pub _leaveok: bool,
    pub _scroll: bool,
    pub _idlok: bool,
    pub _idcok: bool,
    pub _immed: bool,
    pub _sync: bool,
    pub _use_keypad: bool,
    pub _delay: libc::c_int,
    pub _line: *mut ldat,
    pub _regtop: libc::c_short,
    pub _regbottom: libc::c_short,
    pub _parx: libc::c_int,
    pub _pary: libc::c_int,
    pub _parent: *mut WINDOW,
    pub _pad: pdat,
    pub _yoffset: libc::c_short,
    pub _bkgrnd: cchar_t,
    pub _color: libc::c_int,
}
pub type cchar_t = __anonstruct_cchar_t_667532103;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_cchar_t_667532103 {
    pub attr: attr_t,
    pub chars: [wchar_t; 5],
    pub ext_color: libc::c_int,
}
pub type attr_t = chtype;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdat {
    pub _pad_y: libc::c_short,
    pub _pad_x: libc::c_short,
    pub _pad_top: libc::c_short,
    pub _pad_left: libc::c_short,
    pub _pad_bottom: libc::c_short,
    pub _pad_right: libc::c_short,
}
pub type WINDOW = _win_st;
pub type wint_t = libc::c_uint;
pub type action = libc::c_uint;
pub const SEL_QUITERR: action = 65;
pub const SEL_QUIT: action = 64;
pub const SEL_QUITCD: action = 63;
pub const SEL_QUITCTX: action = 62;
pub const SEL_TIMETYPE: action = 61;
pub const SEL_EXPORT: action = 60;
pub const SEL_SESSIONS: action = 59;
pub const SEL_LOCK: action = 58;
pub const SEL_PROMPT: action = 57;
pub const SEL_LAUNCH: action = 56;
pub const SEL_SHELL: action = 55;
pub const SEL_PLUGIN: action = 54;
pub const SEL_EDIT: action = 53;
pub const SEL_AUTONEXT: action = 52;
pub const SEL_HELP: action = 51;
pub const SEL_UMOUNT: action = 50;
pub const SEL_RENAMEMUL: action = 49;
pub const SEL_RENAME: action = 48;
pub const SEL_NEW: action = 47;
pub const SEL_OPENWITH: action = 46;
pub const SEL_RM: action = 45;
pub const SEL_CPMVAS: action = 44;
pub const SEL_MV: action = 43;
pub const SEL_CP: action = 42;
pub const SEL_SELEDIT: action = 41;
pub const SEL_SELINV: action = 40;
pub const SEL_SELALL: action = 39;
pub const SEL_SELMUL: action = 38;
pub const SEL_SEL: action = 37;
pub const SEL_REDRAW: action = 36;
pub const SEL_SORT: action = 35;
pub const SEL_ARCHIVE: action = 34;
pub const SEL_CHMODX: action = 33;
pub const SEL_STATS: action = 32;
pub const SEL_DETAIL: action = 31;
pub const SEL_HIDDEN: action = 30;
pub const SEL_MFLTR: action = 29;
pub const SEL_FLTR: action = 28;
pub const SEL_BMARK: action = 27;
pub const SEL_MARK: action = 26;
pub const SEL_CTX4: action = 25;
pub const SEL_CTX3: action = 24;
pub const SEL_CTX2: action = 23;
pub const SEL_CTX1: action = 22;
pub const SEL_CYCLER: action = 21;
pub const SEL_CYCLE: action = 20;
pub const SEL_REMOTE: action = 19;
pub const SEL_BMOPEN: action = 18;
pub const SEL_CDROOT: action = 17;
pub const SEL_CDLAST: action = 16;
pub const SEL_CDBEGIN: action = 15;
pub const SEL_CDHOME: action = 14;
pub const SEL_JUMP: action = 13;
pub const SEL_FIRST: action = 12;
pub const SEL_END: action = 11;
pub const SEL_HOME: action = 10;
pub const SEL_CTRL_U: action = 9;
pub const SEL_CTRL_D: action = 8;
pub const SEL_PGUP: action = 7;
pub const SEL_PGDN: action = 6;
pub const SEL_PREV: action = 5;
pub const SEL_NEXT: action = 4;
pub const SEL_NAV_IN: action = 3;
pub const SEL_OPEN: action = 2;
pub const SEL_BACK: action = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key {
    pub sym: wint_t,
    pub act: action,
}
pub type uint_t = libc::c_uint;
pub type uchar_t = libc::c_uchar;
pub type ushort_t = libc::c_ushort;
pub type ullong_t = libc::c_ulonglong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct __anonstruct____missing_field_name_824606445 {
    #[bitfield(name = "blocks", ty = "ullong_t", bits = "0..=39")]
    #[bitfield(name = "nlen", ty = "ullong_t", bits = "40..=55")]
    #[bitfield(name = "flags", ty = "ullong_t", bits = "56..=63")]
    pub blocks_nlen_flags: [u8; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct entry {
    pub name: *mut libc::c_char,
    pub sec: time_t,
    pub nsec: uint_t,
    pub mode: mode_t,
    pub size: off_t,
    pub __annonCompField18: __anonstruct____missing_field_name_824606445,
}
pub type pEntry = *mut entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_selmark_653462436 {
    pub startpos: *mut libc::c_char,
    pub len: size_t,
}
pub type selmark = __anonstruct_selmark_653462436;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_kv_1012109243 {
    pub key: libc::c_int,
    pub off: libc::c_int,
}
pub type kv = __anonstruct_kv_1012109243;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_fltrexp_t_723168813 {
    pub regex: *const regex_t,
    pub str_0: *const libc::c_char,
}
pub type fltrexp_t = __anonstruct_fltrexp_t_723168813;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct __anonstruct_settings_893539002 {
    #[bitfield(name = "filtermode", ty = "uint_t", bits = "0..=0")]
    #[bitfield(name = "timeorder", ty = "uint_t", bits = "1..=1")]
    #[bitfield(name = "sizeorder", ty = "uint_t", bits = "2..=2")]
    #[bitfield(name = "apparentsz", ty = "uint_t", bits = "3..=3")]
    #[bitfield(name = "blkorder", ty = "uint_t", bits = "4..=4")]
    #[bitfield(name = "extnorder", ty = "uint_t", bits = "5..=5")]
    #[bitfield(name = "showhidden", ty = "uint_t", bits = "6..=6")]
    #[bitfield(name = "reserved0", ty = "uint_t", bits = "7..=7")]
    #[bitfield(name = "showdetail", ty = "uint_t", bits = "8..=8")]
    #[bitfield(name = "ctxactive", ty = "uint_t", bits = "9..=9")]
    #[bitfield(name = "reverse", ty = "uint_t", bits = "10..=10")]
    #[bitfield(name = "version", ty = "uint_t", bits = "11..=11")]
    #[bitfield(name = "reserved1", ty = "uint_t", bits = "12..=12")]
    #[bitfield(name = "curctx", ty = "uint_t", bits = "13..=15")]
    #[bitfield(name = "prefersel", ty = "uint_t", bits = "16..=16")]
    #[bitfield(name = "fileinfo", ty = "uint_t", bits = "17..=17")]
    #[bitfield(name = "nonavopen", ty = "uint_t", bits = "18..=18")]
    #[bitfield(name = "autoenter", ty = "uint_t", bits = "19..=19")]
    #[bitfield(name = "reserved2", ty = "uint_t", bits = "20..=20")]
    #[bitfield(name = "useeditor", ty = "uint_t", bits = "21..=21")]
    #[bitfield(name = "reserved3", ty = "uint_t", bits = "22..=24")]
    #[bitfield(name = "regex", ty = "uint_t", bits = "25..=25")]
    #[bitfield(name = "x11", ty = "uint_t", bits = "26..=26")]
    #[bitfield(name = "timetype", ty = "uint_t", bits = "27..=28")]
    #[bitfield(name = "cliopener", ty = "uint_t", bits = "29..=29")]
    #[bitfield(name = "waitedit", ty = "uint_t", bits = "30..=30")]
    #[bitfield(name = "rollover", ty = "uint_t", bits = "31..=31")]
    pub filtermode_timeorder_sizeorder_apparentsz_blkorder_extnorder_showhidden_reserved0_showdetail_ctxactive_reverse_version_reserved1_curctx_prefersel_fileinfo_nonavopen_autoenter_reserved2_useeditor_reserved3_regex_x11_timetype_cliopener_waitedit_rollover: [u8; 4],
}
pub type settings = __anonstruct_settings_893539002;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct __anonstruct_runstate_1070726198 {
    #[bitfield(name = "autofifo", ty = "uint_t", bits = "0..=0")]
    #[bitfield(name = "autonext", ty = "uint_t", bits = "1..=1")]
    #[bitfield(name = "dircolor", ty = "uint_t", bits = "2..=2")]
    #[bitfield(name = "dirctx", ty = "uint_t", bits = "3..=3")]
    #[bitfield(name = "duinit", ty = "uint_t", bits = "4..=4")]
    #[bitfield(name = "fifomode", ty = "uint_t", bits = "5..=5")]
    #[bitfield(name = "forcequit", ty = "uint_t", bits = "6..=6")]
    #[bitfield(name = "initfile", ty = "uint_t", bits = "7..=7")]
    #[bitfield(name = "interrupt", ty = "uint_t", bits = "8..=8")]
    #[bitfield(name = "move_0", ty = "uint_t", bits = "9..=9")]
    #[bitfield(name = "oldcolor", ty = "uint_t", bits = "10..=10")]
    #[bitfield(name = "picked", ty = "uint_t", bits = "11..=11")]
    #[bitfield(name = "picker", ty = "uint_t", bits = "12..=12")]
    #[bitfield(name = "pluginit", ty = "uint_t", bits = "13..=13")]
    #[bitfield(name = "prstssn", ty = "uint_t", bits = "14..=14")]
    #[bitfield(name = "rangesel", ty = "uint_t", bits = "15..=15")]
    #[bitfield(name = "runctx", ty = "uint_t", bits = "16..=18")]
    #[bitfield(name = "runplugin", ty = "uint_t", bits = "19..=19")]
    #[bitfield(name = "selbm", ty = "uint_t", bits = "20..=20")]
    #[bitfield(name = "selmode", ty = "uint_t", bits = "21..=21")]
    #[bitfield(name = "stayonsel", ty = "uint_t", bits = "22..=22")]
    #[bitfield(name = "trash", ty = "uint_t", bits = "23..=24")]
    #[bitfield(name = "uidgid", ty = "uint_t", bits = "25..=25")]
    #[bitfield(name = "usebsdtar", ty = "uint_t", bits = "26..=26")]
    #[bitfield(name = "reserved", ty = "uint_t", bits = "27..=31")]
    pub autofifo_autonext_dircolor_dirctx_duinit_fifomode_forcequit_initfile_interrupt_move_0_oldcolor_picked_picker_pluginit_prstssn_rangesel_runctx_runplugin_selbm_selmode_stayonsel_trash_uidgid_usebsdtar_reserved: [u8; 4],
}
pub type runstate = __anonstruct_runstate_1070726198;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_context_430508109 {
    pub c_path: [libc::c_char; 4096],
    pub c_last: [libc::c_char; 4096],
    pub c_name: [libc::c_char; 256],
    pub c_fltr: [libc::c_char; 48],
    pub c_cfg: settings,
    pub color: uint_t,
}
pub type context = __anonstruct_context_430508109;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_thread_data_232432842 {
    pub path: [libc::c_char; 4096],
    pub entnum: libc::c_int,
    pub core: ushort_t,
    pub mntpoint: bool,
}
pub type thread_data = __anonstruct_thread_data_232432842;
static mut bindings: [key; 84] = [
    {
        let mut init = key {
            sym: 260 as libc::c_int as wint_t,
            act: SEL_BACK,
        };
        init
    },
    {
        let mut init = key {
            sym: 'h' as i32 as wint_t,
            act: SEL_BACK,
        };
        init
    },
    {
        let mut init = key {
            sym: 343 as libc::c_int as wint_t,
            act: SEL_OPEN,
        };
        init
    },
    {
        let mut init = key {
            sym: '\r' as i32 as wint_t,
            act: SEL_OPEN,
        };
        init
    },
    {
        let mut init = key {
            sym: 261 as libc::c_int as wint_t,
            act: SEL_NAV_IN,
        };
        init
    },
    {
        let mut init = key {
            sym: 'l' as i32 as wint_t,
            act: SEL_NAV_IN,
        };
        init
    },
    {
        let mut init = key {
            sym: 'j' as i32 as wint_t,
            act: SEL_NEXT,
        };
        init
    },
    {
        let mut init = key {
            sym: 258 as libc::c_int as wint_t,
            act: SEL_NEXT,
        };
        init
    },
    {
        let mut init = key {
            sym: 'k' as i32 as wint_t,
            act: SEL_PREV,
        };
        init
    },
    {
        let mut init = key {
            sym: 259 as libc::c_int as wint_t,
            act: SEL_PREV,
        };
        init
    },
    {
        let mut init = key {
            sym: 338 as libc::c_int as wint_t,
            act: SEL_PGDN,
        };
        init
    },
    {
        let mut init = key {
            sym: 339 as libc::c_int as wint_t,
            act: SEL_PGUP,
        };
        init
    },
    {
        let mut init = key {
            sym: 4 as libc::c_int as wint_t,
            act: SEL_CTRL_D,
        };
        init
    },
    {
        let mut init = key {
            sym: 21 as libc::c_int as wint_t,
            act: SEL_CTRL_U,
        };
        init
    },
    {
        let mut init = key {
            sym: 262 as libc::c_int as wint_t,
            act: SEL_HOME,
        };
        init
    },
    {
        let mut init = key {
            sym: 'g' as i32 as wint_t,
            act: SEL_HOME,
        };
        init
    },
    {
        let mut init = key {
            sym: 1 as libc::c_int as wint_t,
            act: SEL_HOME,
        };
        init
    },
    {
        let mut init = key {
            sym: 360 as libc::c_int as wint_t,
            act: SEL_END,
        };
        init
    },
    {
        let mut init = key {
            sym: 'G' as i32 as wint_t,
            act: SEL_END,
        };
        init
    },
    {
        let mut init = key {
            sym: 5 as libc::c_int as wint_t,
            act: SEL_END,
        };
        init
    },
    {
        let mut init = key {
            sym: '\'' as i32 as wint_t,
            act: SEL_FIRST,
        };
        init
    },
    {
        let mut init = key {
            sym: 'J' as i32 as wint_t,
            act: SEL_JUMP,
        };
        init
    },
    {
        let mut init = key {
            sym: '~' as i32 as wint_t,
            act: SEL_CDHOME,
        };
        init
    },
    {
        let mut init = key {
            sym: '@' as i32 as wint_t,
            act: SEL_CDBEGIN,
        };
        init
    },
    {
        let mut init = key {
            sym: '-' as i32 as wint_t,
            act: SEL_CDLAST,
        };
        init
    },
    {
        let mut init = key {
            sym: '`' as i32 as wint_t,
            act: SEL_CDROOT,
        };
        init
    },
    {
        let mut init = key {
            sym: 'b' as i32 as wint_t,
            act: SEL_BMOPEN,
        };
        init
    },
    {
        let mut init = key {
            sym: 31 as libc::c_int as wint_t,
            act: SEL_BMOPEN,
        };
        init
    },
    {
        let mut init = key {
            sym: 'c' as i32 as wint_t,
            act: SEL_REMOTE,
        };
        init
    },
    {
        let mut init = key {
            sym: '\t' as i32 as wint_t,
            act: SEL_CYCLE,
        };
        init
    },
    {
        let mut init = key {
            sym: 353 as libc::c_int as wint_t,
            act: SEL_CYCLER,
        };
        init
    },
    {
        let mut init = key {
            sym: '1' as i32 as wint_t,
            act: SEL_CTX1,
        };
        init
    },
    {
        let mut init = key {
            sym: '2' as i32 as wint_t,
            act: SEL_CTX2,
        };
        init
    },
    {
        let mut init = key {
            sym: '3' as i32 as wint_t,
            act: SEL_CTX3,
        };
        init
    },
    {
        let mut init = key {
            sym: '4' as i32 as wint_t,
            act: SEL_CTX4,
        };
        init
    },
    {
        let mut init = key {
            sym: ',' as i32 as wint_t,
            act: SEL_MARK,
        };
        init
    },
    {
        let mut init = key {
            sym: 'B' as i32 as wint_t,
            act: SEL_BMARK,
        };
        init
    },
    {
        let mut init = key {
            sym: '/' as i32 as wint_t,
            act: SEL_FLTR,
        };
        init
    },
    {
        let mut init = key {
            sym: 14 as libc::c_int as wint_t,
            act: SEL_MFLTR,
        };
        init
    },
    {
        let mut init = key {
            sym: '.' as i32 as wint_t,
            act: SEL_HIDDEN,
        };
        init
    },
    {
        let mut init = key {
            sym: 'd' as i32 as wint_t,
            act: SEL_DETAIL,
        };
        init
    },
    {
        let mut init = key {
            sym: 'f' as i32 as wint_t,
            act: SEL_STATS,
        };
        init
    },
    {
        let mut init = key {
            sym: 6 as libc::c_int as wint_t,
            act: SEL_STATS,
        };
        init
    },
    {
        let mut init = key {
            sym: '*' as i32 as wint_t,
            act: SEL_CHMODX,
        };
        init
    },
    {
        let mut init = key {
            sym: 'z' as i32 as wint_t,
            act: SEL_ARCHIVE,
        };
        init
    },
    {
        let mut init = key {
            sym: 't' as i32 as wint_t,
            act: SEL_SORT,
        };
        init
    },
    {
        let mut init = key {
            sym: 20 as libc::c_int as wint_t,
            act: SEL_SORT,
        };
        init
    },
    {
        let mut init = key {
            sym: 12 as libc::c_int as wint_t,
            act: SEL_REDRAW,
        };
        init
    },
    {
        let mut init = key {
            sym: ' ' as i32 as wint_t,
            act: SEL_SEL,
        };
        init
    },
    {
        let mut init = key {
            sym: '+' as i32 as wint_t,
            act: SEL_SEL,
        };
        init
    },
    {
        let mut init = key {
            sym: 'm' as i32 as wint_t,
            act: SEL_SELMUL,
        };
        init
    },
    {
        let mut init = key {
            sym: 'a' as i32 as wint_t,
            act: SEL_SELALL,
        };
        init
    },
    {
        let mut init = key {
            sym: 'A' as i32 as wint_t,
            act: SEL_SELINV,
        };
        init
    },
    {
        let mut init = key {
            sym: 'E' as i32 as wint_t,
            act: SEL_SELEDIT,
        };
        init
    },
    {
        let mut init = key {
            sym: 'p' as i32 as wint_t,
            act: SEL_CP,
        };
        init
    },
    {
        let mut init = key {
            sym: 16 as libc::c_int as wint_t,
            act: SEL_CP,
        };
        init
    },
    {
        let mut init = key {
            sym: 'v' as i32 as wint_t,
            act: SEL_MV,
        };
        init
    },
    {
        let mut init = key {
            sym: 22 as libc::c_int as wint_t,
            act: SEL_MV,
        };
        init
    },
    {
        let mut init = key {
            sym: 'w' as i32 as wint_t,
            act: SEL_CPMVAS,
        };
        init
    },
    {
        let mut init = key {
            sym: 23 as libc::c_int as wint_t,
            act: SEL_CPMVAS,
        };
        init
    },
    {
        let mut init = key {
            sym: 'x' as i32 as wint_t,
            act: SEL_RM,
        };
        init
    },
    {
        let mut init = key {
            sym: 24 as libc::c_int as wint_t,
            act: SEL_RM,
        };
        init
    },
    {
        let mut init = key {
            sym: 'o' as i32 as wint_t,
            act: SEL_OPENWITH,
        };
        init
    },
    {
        let mut init = key {
            sym: 15 as libc::c_int as wint_t,
            act: SEL_OPENWITH,
        };
        init
    },
    {
        let mut init = key {
            sym: 'n' as i32 as wint_t,
            act: SEL_NEW,
        };
        init
    },
    {
        let mut init = key {
            sym: 18 as libc::c_int as wint_t,
            act: SEL_RENAME,
        };
        init
    },
    {
        let mut init = key {
            sym: 'r' as i32 as wint_t,
            act: SEL_RENAMEMUL,
        };
        init
    },
    {
        let mut init = key {
            sym: 'u' as i32 as wint_t,
            act: SEL_UMOUNT,
        };
        init
    },
    {
        let mut init = key {
            sym: '?' as i32 as wint_t,
            act: SEL_HELP,
        };
        init
    },
    {
        let mut init = key {
            sym: 10 as libc::c_int as wint_t,
            act: SEL_AUTONEXT,
        };
        init
    },
    {
        let mut init = key {
            sym: 'e' as i32 as wint_t,
            act: SEL_EDIT,
        };
        init
    },
    {
        let mut init = key {
            sym: ';' as i32 as wint_t,
            act: SEL_PLUGIN,
        };
        init
    },
    {
        let mut init = key {
            sym: '!' as i32 as wint_t,
            act: SEL_SHELL,
        };
        init
    },
    {
        let mut init = key {
            sym: 29 as libc::c_int as wint_t,
            act: SEL_SHELL,
        };
        init
    },
    {
        let mut init = key {
            sym: '=' as i32 as wint_t,
            act: SEL_LAUNCH,
        };
        init
    },
    {
        let mut init = key {
            sym: ']' as i32 as wint_t,
            act: SEL_PROMPT,
        };
        init
    },
    {
        let mut init = key {
            sym: '0' as i32 as wint_t,
            act: SEL_LOCK,
        };
        init
    },
    {
        let mut init = key {
            sym: 's' as i32 as wint_t,
            act: SEL_SESSIONS,
        };
        init
    },
    {
        let mut init = key {
            sym: '>' as i32 as wint_t,
            act: SEL_EXPORT,
        };
        init
    },
    {
        let mut init = key {
            sym: 'T' as i32 as wint_t,
            act: SEL_TIMETYPE,
        };
        init
    },
    {
        let mut init = key {
            sym: 'q' as i32 as wint_t,
            act: SEL_QUITCTX,
        };
        init
    },
    {
        let mut init = key {
            sym: 7 as libc::c_int as wint_t,
            act: SEL_QUITCD,
        };
        init
    },
    {
        let mut init = key {
            sym: 17 as libc::c_int as wint_t,
            act: SEL_QUIT,
        };
        init
    },
    {
        let mut init = key {
            sym: 'Q' as i32 as wint_t,
            act: SEL_QUITERR,
        };
        init
    },
];
static mut cfg: settings = settings {
    filtermode_timeorder_sizeorder_apparentsz_blkorder_extnorder_showhidden_reserved0_showdetail_ctxactive_reverse_version_reserved1_curctx_prefersel_fileinfo_nonavopen_autoenter_reserved2_useeditor_reserved3_regex_x11_timetype_cliopener_waitedit_rollover: [0; 4],
};
static mut g_ctx: [context; 4] = [context {
    c_path: [0; 4096],
    c_last: [0; 4096],
    c_name: [0; 256],
    c_fltr: [0; 48],
    c_cfg: settings {
        filtermode_timeorder_sizeorder_apparentsz_blkorder_extnorder_showhidden_reserved0_showdetail_ctxactive_reverse_version_reserved1_curctx_prefersel_fileinfo_nonavopen_autoenter_reserved2_useeditor_reserved3_regex_x11_timetype_cliopener_waitedit_rollover: [0; 4],
    },
    color: 0,
}; 4];
static mut ndents: libc::c_int = 0;
static mut cur: libc::c_int = 0;
static mut last: libc::c_int = 0;
static mut curscroll: libc::c_int = 0;
static mut last_curscroll: libc::c_int = 0;
static mut total_dents: libc::c_int = 64 as libc::c_int;
static mut scroll_lines: libc::c_int = 1 as libc::c_int;
static mut nselected: libc::c_int = 0;
static mut gtimesecs: time_t = 0;
static mut idletimeout: uint_t = 0;
static mut selbufpos: uint_t = 0;
static mut selbuflen: uint_t = 0;
static mut xlines: ushort_t = 0;
static mut xcols: ushort_t = 0;
static mut idle: ushort_t = 0;
static mut maxbm: uchar_t = 0;
static mut maxplug: uchar_t = 0;
static mut maxorder: uchar_t = 0;
static mut cfgsort: [uchar_t; 5] = [0; 5];
static mut bmstr: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut pluginstr: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut orderstr: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut opener: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut editor: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut enveditor: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut pager: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut shell: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut home: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut initpath: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut cfgpath: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut selpath: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut listpath: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut listroot: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut plgpath: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut pnamebuf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut pselbuf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut findselpos: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut mark: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut lastcmd: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut ihashbmp: *mut ullong_t = 0 as *const ullong_t as *mut ullong_t;
static mut pdents: *mut entry = 0 as *const entry as *mut entry;
static mut dir_blocks: blkcnt_t = 0;
static mut bookmark: *mut kv = 0 as *const kv as *mut kv;
static mut plug: *mut kv = 0 as *const kv as *mut kv;
static mut order: *mut kv = 0 as *const kv as *mut kv;
static mut tmpfplen: uchar_t = 0;
static mut homelen: uchar_t = 0;
static mut blk_shift: uchar_t = 9 as libc::c_int as uchar_t;
static mut archive_re: regex_t = regex_t {
    buffer: 0 as *const re_dfa_t as *mut re_dfa_t,
    allocated: 0,
    used: 0,
    syntax: 0,
    fastmap: 0 as *const libc::c_char as *mut libc::c_char,
    translate: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    re_nsub: 0,
    can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [0; 1],
    c2rust_padding: [0; 7],
};
static mut threadbmp: libc::c_int = -(1 as libc::c_int);
static mut active_threads: libc::c_int = 0;
static mut running_mutex: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
static mut hardlink_mutex: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
static mut core_files: *mut ullong_t = 0 as *const ullong_t as *mut ullong_t;
static mut core_blocks: *mut blkcnt_t = 0 as *const blkcnt_t as *mut blkcnt_t;
static mut num_files: ullong_t = 0;
static mut core_data: *mut thread_data = 0 as *const thread_data as *mut thread_data;
static mut oldsighup: sigaction = sigaction {
    __sigaction_handler: __anonunion___sigaction_handler_363639592 {
        sa_handler: None,
    },
    sa_mask: __sigset_t { __val: [0; 16] },
    sa_flags: 0,
    sa_restorer: None,
};
static mut oldsigtstp: sigaction = sigaction {
    __sigaction_handler: __anonunion___sigaction_handler_363639592 {
        sa_handler: None,
    },
    sa_mask: __sigset_t { __val: [0; 16] },
    sa_flags: 0,
    sa_restorer: None,
};
static mut oldsigwinch: sigaction = sigaction {
    __sigaction_handler: __anonunion___sigaction_handler_363639592 {
        sa_handler: None,
    },
    sa_mask: __sigset_t { __val: [0; 16] },
    sa_flags: 0,
    sa_restorer: None,
};
static mut g_buf: [libc::c_char; 4608] = [0; 4608];
static mut g_sel: [libc::c_char; 4096] = [0; 4096];
static mut g_tmpfpath: [libc::c_char; 64] = [0; 64];
static mut g_pipepath: [libc::c_char; 64] = [0; 64];
static mut g_state: runstate = runstate {
    autofifo_autonext_dircolor_dirctx_duinit_fifomode_forcequit_initfile_interrupt_move_0_oldcolor_picked_picker_pluginit_prstssn_rangesel_runctx_runplugin_selbm_selmode_stayonsel_trash_uidgid_usebsdtar_reserved: [0; 4],
};
static mut utils: [*mut libc::c_char; 21] = [
    b"xdg-open\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"atool\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"bsdtar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"unzip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"vlock\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"launch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sh -c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"bash\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sshfs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"rclone\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"vi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"less\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sh\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"fzf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b".ntfy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b".cbcp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b".nmv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"trash-put\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gio trash\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"rm -rf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
static mut messages: [*const libc::c_char; 45] = [
    b"\0" as *const u8 as *const libc::c_char,
    b"0 entries\0" as *const u8 as *const libc::c_char,
    b"/.nnnXXXXXX\0" as *const u8 as *const libc::c_char,
    b"0 selected\0" as *const u8 as *const libc::c_char,
    b"cancelled\0" as *const u8 as *const libc::c_char,
    b"failed!\0" as *const u8 as *const libc::c_char,
    b"session name: \0" as *const u8 as *const libc::c_char,
    b"'c'p/'m'v as?\0" as *const u8 as *const libc::c_char,
    b"'c'urrent/'s'el?\0" as *const u8 as *const libc::c_char,
    b"%s %s? [Esc cancels]\0" as *const u8 as *const libc::c_char,
    b"size limit exceeded\0" as *const u8 as *const libc::c_char,
    b"'f'ile/'d'ir/'s'ym/'h'ard?\0" as *const u8 as *const libc::c_char,
    b"'c'li/'g'ui?\0" as *const u8 as *const libc::c_char,
    b"overwrite?\0" as *const u8 as *const libc::c_char,
    b"'s'ave/'l'oad/'r'estore?\0" as *const u8 as *const libc::c_char,
    b"Quit all contexts?\0" as *const u8 as *const libc::c_char,
    b"remote name (- for hovered): \0" as *const u8 as *const libc::c_char,
    b"archive [path/]name: \0" as *const u8 as *const libc::c_char,
    b"open with: \0" as *const u8 as *const libc::c_char,
    b"[path/]name: \0" as *const u8 as *const libc::c_char,
    b"link prefix [@ for none]: \0" as *const u8 as *const libc::c_char,
    b"copy [path/]name: \0" as *const u8 as *const libc::c_char,
    b"\n'Enter' to continue\0" as *const u8 as *const libc::c_char,
    b"open failed\0" as *const u8 as *const libc::c_char,
    b"dir inaccessible\0" as *const u8 as *const libc::c_char,
    b"empty! edit/open with\0" as *const u8 as *const libc::c_char,
    b"?\0" as *const u8 as *const libc::c_char,
    b"not set\0" as *const u8 as *const libc::c_char,
    b"entry exists\0" as *const u8 as *const libc::c_char,
    b"too few cols!\0" as *const u8 as *const libc::c_char,
    b"'s'shfs/'r'clone?\0" as *const u8 as *const libc::c_char,
    b"refresh if slow\0" as *const u8 as *const libc::c_char,
    b"app: \0" as *const u8 as *const libc::c_char,
    b"'o'pen/e'x'tract/'l's/'m'nt?\0" as *const u8 as *const libc::c_char,
    b"keys:\0" as *const u8 as *const libc::c_char,
    b"invalid regex\0" as *const u8 as *const libc::c_char,
    b"'a'u/'d'u/'e'xt/'r'ev/'s'z/'t'm/'v'er/'c'lr/'^T'?\0" as *const u8
        as *const libc::c_char,
    b"unmount failed! try lazy?\0" as *const u8 as *const libc::c_char,
    b"first file (')/char?\0" as *const u8 as *const libc::c_char,
    b"remove tmp file?\0" as *const u8 as *const libc::c_char,
    b"invalid key\0" as *const u8 as *const libc::c_char,
    b"unchanged\0" as *const u8 as *const libc::c_char,
    b"dir changed, range sel off\0" as *const u8 as *const libc::c_char,
    b"name: \0" as *const u8 as *const libc::c_char,
    b"file limit exceeded\0" as *const u8 as *const libc::c_char,
];
static mut env_cfg: [*const libc::c_char; 14] = [
    b"NNN_OPTS\0" as *const u8 as *const libc::c_char,
    b"NNN_BMS\0" as *const u8 as *const libc::c_char,
    b"NNN_PLUG\0" as *const u8 as *const libc::c_char,
    b"NNN_OPENER\0" as *const u8 as *const libc::c_char,
    b"NNN_COLORS\0" as *const u8 as *const libc::c_char,
    b"NNN_FCOLORS\0" as *const u8 as *const libc::c_char,
    b"NNNLVL\0" as *const u8 as *const libc::c_char,
    b"NNN_PIPE\0" as *const u8 as *const libc::c_char,
    b"NNN_MCLICK\0" as *const u8 as *const libc::c_char,
    b"NNN_SEL\0" as *const u8 as *const libc::c_char,
    b"NNN_ARCHIVE\0" as *const u8 as *const libc::c_char,
    b"NNN_ORDER\0" as *const u8 as *const libc::c_char,
    b"NNN_HELP\0" as *const u8 as *const libc::c_char,
    b"NNN_TRASH\0" as *const u8 as *const libc::c_char,
];
static mut envs: [*const libc::c_char; 5] = [
    b"SHELL\0" as *const u8 as *const libc::c_char,
    b"VISUAL\0" as *const u8 as *const libc::c_char,
    b"EDITOR\0" as *const u8 as *const libc::c_char,
    b"PAGER\0" as *const u8 as *const libc::c_char,
    b"nnn\0" as *const u8 as *const libc::c_char,
];
static mut cp: [libc::c_char; 10] = [
    'c' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
];
static mut mv: [libc::c_char; 8] = [
    'm' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
];
static mut archive_cmd: [*const libc::c_char; 4] = [
    b"atool -a\0" as *const u8 as *const libc::c_char,
    b"bsdtar -acvf\0" as *const u8 as *const libc::c_char,
    b"zip -r\0" as *const u8 as *const libc::c_char,
    b"tar -acvf\0" as *const u8 as *const libc::c_char,
];
static mut toks: [*const libc::c_char; 4] = [
    b"bookmarks\0" as *const u8 as *const libc::c_char,
    b"sessions\0" as *const u8 as *const libc::c_char,
    b"mounts\0" as *const u8 as *const libc::c_char,
    b"plugins\0" as *const u8 as *const libc::c_char,
];
static mut patterns: [*const libc::c_char; 5] = [
    b"sed -i 's|^\\(\\(.*/\\)\\(.*\\)$\\)|#\\1\\n\\3|' %s\0" as *const u8
        as *const libc::c_char,
    b"sed 's|^\\([^#/][^/]\\?.*\\)$|%s/\\1|;s|^#\\(/.*\\)$|\\1|' %s | tr '\\n' '\\0' | xargs -0 -n2 sh -c '%s \"$0\" \"$@\" < /dev/tty'\0"
        as *const u8 as *const libc::c_char,
    b"\\.(bz|bz2|gz|tar|taz|tbz|tbz2|tgz|z|zip)$\0" as *const u8 as *const libc::c_char,
    b"sed -i 's|^%s\\(.*\\)$|%s\\1|' %s\0" as *const u8 as *const libc::c_char,
    b"sed -ze 's|^%s/||' '%s' | xargs -0 %s %s\0" as *const u8 as *const libc::c_char,
];
static mut gcolors: [libc::c_char; 25] = [
    'c' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
];
static mut fcolors: [uint_t; 17] = [
    0 as libc::c_int as uint_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
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
static mut inotify_fd: libc::c_int = 0;
static mut inotify_wd: libc::c_int = -(1 as libc::c_int);
static mut INOTIFY_MASK: uint_t = 4034 as libc::c_int as uint_t;
unsafe extern "C" fn sigint_handler(mut sig: libc::c_int) {
    g_state.set_interrupt(1 as libc::c_int as uint_t);
}
unsafe extern "C" fn clean_exit_sighandler(mut sig: libc::c_int) {
    endwin();
    exit(0 as libc::c_int);
}
static mut dst: [libc::c_char; 32] = [
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
];
static mut digits: [libc::c_char; 201] = [
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
];
unsafe extern "C" fn xitoa(mut val: uint_t) -> *mut libc::c_char {
    let mut next: uint_t = 0;
    let mut quo: uint_t = 0;
    let mut i: uint_t = 0;
    next = 30 as libc::c_int as uint_t;
    while val >= 100 as libc::c_uint {
        quo = val.wrapping_div(100 as libc::c_uint);
        i = val
            .wrapping_sub(quo.wrapping_mul(100 as libc::c_uint))
            .wrapping_mul(2 as libc::c_uint);
        val = quo;
        dst[next as usize] = digits[i.wrapping_add(1 as libc::c_uint) as usize];
        next = next.wrapping_sub(1);
        dst[next as usize] = digits[i as usize];
        next = next.wrapping_sub(1);
    }
    if val < 10 as libc::c_uint {
        dst[next as usize] = (48 as libc::c_uint).wrapping_add(val) as libc::c_char;
    } else {
        i = val.wrapping_mul(2 as libc::c_uint);
        dst[next as usize] = digits[i.wrapping_add(1 as libc::c_uint) as usize];
        next = next.wrapping_sub(1);
        dst[next as usize] = digits[i as usize];
    }
    return &mut *dst.as_mut_ptr().offset(next as isize) as *mut libc::c_char;
}
unsafe extern "C" fn xchartohex(mut c: uchar_t) -> uchar_t {
    if (c as libc::c_uint).wrapping_sub(48 as libc::c_uint) <= 9 as libc::c_uint {
        return (c as libc::c_int - 48 as libc::c_int) as uchar_t;
    }
    if c as libc::c_int >= 97 as libc::c_int {
        if c as libc::c_int <= 102 as libc::c_int {
            return (c as libc::c_int - 97 as libc::c_int + 10 as libc::c_int) as uchar_t;
        }
    }
    if c as libc::c_int >= 65 as libc::c_int {
        if c as libc::c_int <= 70 as libc::c_int {
            return (c as libc::c_int - 65 as libc::c_int + 10 as libc::c_int) as uchar_t;
        }
    }
    return c;
}
unsafe extern "C" fn test_set_bit(mut nr: uint_t) -> bool {
    let mut m: *mut ullong_t = 0 as *mut ullong_t;
    nr &= 16777215 as libc::c_uint;
    pthread_mutex_lock(&mut hardlink_mutex);
    m = ihashbmp.offset((nr >> 6 as libc::c_int) as isize);
    if *m & ((1 as libc::c_int) << (nr & 63 as libc::c_uint)) as libc::c_ulonglong != 0 {
        pthread_mutex_unlock(&mut hardlink_mutex);
        return 0 as libc::c_int != 0;
    }
    *m |= ((1 as libc::c_int) << (nr & 63 as libc::c_uint)) as libc::c_ulonglong;
    pthread_mutex_unlock(&mut hardlink_mutex);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn max_openfds() {
    let mut rl: rlimit = rlimit { rlim_cur: 0, rlim_max: 0 };
    let mut tmp: libc::c_int = 0;
    tmp = getrlimit(RLIMIT_NOFILE, &mut rl);
    if tmp == 0 {
        if rl.rlim_cur < rl.rlim_max {
            rl.rlim_cur = rl.rlim_max;
            setrlimit(RLIMIT_NOFILE, &mut rl as *mut rlimit as *const rlimit);
        }
    }
}
unsafe extern "C" fn xrealloc(
    mut pcur: *mut libc::c_void,
    mut len: size_t,
) -> *mut libc::c_void {
    let mut pmem: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = realloc(pcur, len);
    pmem = tmp;
    if pmem.is_null() {
        free(pcur);
    }
    return pmem;
}
unsafe extern "C" fn xstrsncpy(
    mut dst___0: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut n: size_t,
) -> size_t {
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = memccpy(
        dst___0 as *mut libc::c_void,
        src as *const libc::c_void,
        '\u{0}' as i32,
        n,
    );
    end = tmp as *mut libc::c_char;
    if end.is_null() {
        *dst___0
            .offset(
                n.wrapping_sub(1 as libc::c_ulong) as isize,
            ) = '\u{0}' as i32 as libc::c_char;
        end = dst___0.offset(n as isize);
    }
    return end.offset_from(dst___0) as libc::c_long as size_t;
}
#[inline]
unsafe extern "C" fn xstrlen(mut s: *const libc::c_char) -> size_t {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = rawmemchr(s as *const libc::c_void, '\u{0}' as i32);
    return (tmp as *mut libc::c_char).offset_from(s as *mut libc::c_char) as libc::c_long
        as size_t;
}
unsafe extern "C" fn xstrdup(mut s: *const libc::c_char) -> *mut libc::c_char {
    let mut len: size_t = 0;
    let mut tmp: size_t = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = xstrlen(s);
    len = tmp.wrapping_add(1 as libc::c_ulong);
    tmp___0 = malloc(len);
    ptr = tmp___0 as *mut libc::c_char;
    if !ptr.is_null() {
        xstrsncpy(ptr, s, len);
    }
    return ptr;
}
unsafe extern "C" fn is_suffix(
    mut str: *const libc::c_char,
    mut suffix: *const libc::c_char,
) -> bool {
    let mut lenstr: size_t = 0;
    let mut tmp: size_t = 0;
    let mut lensuffix: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    if str.is_null() {
        return 0 as libc::c_int != 0
    } else {
        if suffix.is_null() {
            return 0 as libc::c_int != 0;
        }
    }
    tmp = xstrlen(str);
    lenstr = tmp;
    tmp___0 = xstrlen(suffix);
    lensuffix = tmp___0;
    if lensuffix > lenstr {
        return 0 as libc::c_int != 0;
    }
    if *str.offset(lenstr.wrapping_sub(lensuffix) as isize) as libc::c_int
        != *suffix as libc::c_int
    {
        tmp___2 = -(1 as libc::c_int);
    } else {
        tmp___1 = strcmp(str.offset(lenstr.wrapping_sub(lensuffix) as isize), suffix);
        tmp___2 = tmp___1;
    }
    return tmp___2 == 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn is_prefix(
    mut str: *const libc::c_char,
    mut prefix: *const libc::c_char,
    mut len: size_t,
) -> bool {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = strncmp(str, prefix, len);
    if tmp != 0 {
        tmp___0 = 0 as libc::c_int;
    } else {
        tmp___0 = 1 as libc::c_int;
    }
    return tmp___0 != 0;
}
unsafe extern "C" fn xmemrchr(
    mut s: *mut uchar_t,
    mut ch: uchar_t,
    mut n: size_t,
) -> *mut libc::c_void {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = memrchr(s as *const libc::c_void, ch as libc::c_int, n);
    return tmp;
}
unsafe extern "C" fn xdirname(mut path: *mut libc::c_char) -> *mut libc::c_char {
    let mut base: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: size_t = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = xstrlen(path as *const libc::c_char);
    tmp___0 = xmemrchr(path as *mut uchar_t, '/' as i32 as uchar_t, tmp);
    base = tmp___0 as *mut libc::c_char;
    if base as libc::c_ulong == path as libc::c_ulong {
        *path.offset(1 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    } else {
        *base = '\u{0}' as i32 as libc::c_char;
    }
    return path;
}
unsafe extern "C" fn xbasename(mut path: *mut libc::c_char) -> *mut libc::c_char {
    let mut base: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: size_t = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = xstrlen(path as *const libc::c_char);
    tmp___0 = xmemrchr(path as *mut uchar_t, '/' as i32 as uchar_t, tmp);
    base = tmp___0 as *mut libc::c_char;
    if !base.is_null() {
        tmp___1 = base.offset(1 as libc::c_int as isize);
    } else {
        tmp___1 = path;
    }
    return tmp___1;
}
#[inline]
unsafe extern "C" fn xextension(
    mut fname: *const libc::c_char,
    mut len: size_t,
) -> *mut libc::c_char {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = xmemrchr(fname as *mut uchar_t, '.' as i32 as uchar_t, len);
    return tmp as *mut libc::c_char;
}
#[inline]
unsafe extern "C" fn getutil(mut util: *mut libc::c_char) -> bool {
    let mut tmp: libc::c_int = 0;
    tmp = spawn(
        b"which\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        util,
        0 as *mut libc::c_void as *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
        12 as libc::c_int as ushort_t,
    );
    return tmp == 0 as libc::c_int;
}
unsafe extern "C" fn mkpath(
    mut dir: *const libc::c_char,
    mut name: *const libc::c_char,
    mut out: *mut libc::c_char,
) -> size_t {
    let mut len: size_t = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    len = 0 as libc::c_int as size_t;
    if *name.offset(0 as libc::c_int as isize) as libc::c_int != 47 as libc::c_int {
        if *dir.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
            if *dir.offset(0 as libc::c_int as isize) as libc::c_int == 47 as libc::c_int
            {
                len = 1 as libc::c_int as size_t;
            } else {
                tmp = xstrsncpy(out, dir, 4096 as libc::c_int as size_t);
                len = tmp;
            }
        } else {
            tmp = xstrsncpy(out, dir, 4096 as libc::c_int as size_t);
            len = tmp;
        }
        *out
            .offset(
                len.wrapping_sub(1 as libc::c_ulong) as isize,
            ) = '/' as i32 as libc::c_char;
    }
    tmp___0 = xstrsncpy(
        out.offset(len as isize),
        name,
        (4096 as libc::c_ulong).wrapping_sub(len),
    );
    return tmp___0.wrapping_add(len);
}
unsafe extern "C" fn common_prefix(
    mut path: *const libc::c_char,
    mut prefix: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut x: *const libc::c_char = 0 as *const libc::c_char;
    let mut y: *const libc::c_char = 0 as *const libc::c_char;
    let mut sep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    x = path;
    y = prefix as *const libc::c_char;
    if path.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char
    } else {
        if *path == 0 {
            return 0 as *mut libc::c_void as *mut libc::c_char
        } else {
            if prefix.is_null() {
                return 0 as *mut libc::c_void as *mut libc::c_char;
            }
        }
    }
    if *prefix == 0 {
        xstrsncpy(prefix, path, 4096 as libc::c_int as size_t);
        return prefix;
    }
    while *x != 0 {
        if !(*y != 0) {
            break;
        }
        if !(*x as libc::c_int == *y as libc::c_int) {
            break;
        }
        x = x.offset(1);
        y = y.offset(1);
    }
    if *x == 0 {
        if *y == 0 {
            return prefix;
        }
    }
    if *x == 0 {
        if *y as libc::c_int == 47 as libc::c_int {
            xstrsncpy(prefix, path, y.offset_from(path) as libc::c_long as size_t);
            return prefix;
        }
    }
    if *y == 0 {
        if *x as libc::c_int == 47 as libc::c_int {
            return prefix;
        }
    }
    *prefix
        .offset(
            y.offset_from(prefix as *const libc::c_char) as libc::c_long as isize,
        ) = '\u{0}' as i32 as libc::c_char;
    tmp = xmemrchr(
        prefix as *mut uchar_t,
        '/' as i32 as uchar_t,
        y.offset_from(prefix as *const libc::c_char) as libc::c_long as size_t,
    );
    sep = tmp as *mut libc::c_char;
    if sep as libc::c_ulong != prefix as libc::c_ulong {
        *sep = '\u{0}' as i32 as libc::c_char;
    } else {
        *prefix.offset(1 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    }
    return prefix;
}
unsafe extern "C" fn abspath(
    mut path: *const libc::c_char,
    mut cwd: *const libc::c_char,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dst_size: size_t = 0;
    let mut src_size: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut cwd_size: size_t = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: size_t = 0;
    let mut len: size_t = 0;
    let mut src: *const libc::c_char = 0 as *const libc::c_char;
    let mut dst___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut resolved_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___5: size_t = 0;
    let mut next: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___6: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___7: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___8: *mut libc::c_char = 0 as *mut libc::c_char;
    if path.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    if *path.offset(0 as libc::c_int as isize) as libc::c_int == 126 as libc::c_int {
        cwd = home as *const libc::c_char;
    } else if *path.offset(0 as libc::c_int as isize) as libc::c_int != 47 as libc::c_int
        {
        if cwd.is_null() {
            tmp = getcwd(
                0 as *mut libc::c_void as *mut libc::c_char,
                0 as libc::c_int as size_t,
            );
            cwd = tmp as *const libc::c_char;
        }
    }
    dst_size = 0 as libc::c_int as size_t;
    tmp___0 = xstrlen(path);
    src_size = tmp___0;
    if !cwd.is_null() {
        tmp___1 = xstrlen(cwd);
        tmp___2 = tmp___1;
    } else {
        tmp___2 = 0 as libc::c_int as size_t;
    }
    cwd_size = tmp___2;
    len = src_size;
    if !buf.is_null() {
        tmp___4 = buf as *mut libc::c_void;
    } else {
        tmp___3 = malloc(
            src_size.wrapping_add(cwd_size).wrapping_add(2 as libc::c_ulong),
        );
        tmp___4 = tmp___3;
    }
    resolved_path = tmp___4 as *mut libc::c_char;
    if resolved_path.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    if *path.offset(0 as libc::c_int as isize) as libc::c_int != 47 as libc::c_int {
        if cwd.is_null() {
            if buf.is_null() {
                free(resolved_path as *mut libc::c_void);
            }
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
        tmp___5 = xstrsncpy(
            resolved_path,
            cwd,
            cwd_size.wrapping_add(1 as libc::c_ulong),
        );
        dst_size = tmp___5.wrapping_sub(1 as libc::c_ulong);
    } else {
        *resolved_path
            .offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    }
    src = path;
    dst___0 = resolved_path.offset(dst_size as isize);
    next = 0 as *mut libc::c_void as *const libc::c_char;
    while next as libc::c_ulong != path.offset(src_size as isize) as libc::c_ulong {
        tmp___6 = memchr(src as *const libc::c_void, '/' as i32, len);
        next = tmp___6 as *const libc::c_char;
        if next.is_null() {
            next = path.offset(src_size as isize);
        }
        if next.offset_from(src) as libc::c_long == 2 as libc::c_long {
            if *src.offset(0 as libc::c_int as isize) as libc::c_int == 46 as libc::c_int
            {
                if *src.offset(1 as libc::c_int as isize) as libc::c_int
                    == 46 as libc::c_int
                {
                    if dst___0.offset_from(resolved_path) as libc::c_long != 0 {
                        tmp___7 = xmemrchr(
                            resolved_path as *mut uchar_t,
                            '/' as i32 as uchar_t,
                            dst___0.offset_from(resolved_path) as libc::c_long as size_t,
                        );
                        dst___0 = tmp___7 as *mut libc::c_char;
                        *dst___0 = '\u{0}' as i32 as libc::c_char;
                    }
                    current_block = 307447392441238883;
                } else {
                    current_block = 15106417378165705176;
                }
            } else {
                current_block = 15106417378165705176;
            }
        } else {
            current_block = 15106417378165705176;
        }
        match current_block {
            15106417378165705176 => {
                let mut current_block_65: u64;
                if next.offset_from(src) as libc::c_long == 1 as libc::c_long {
                    if !(*src.offset(0 as libc::c_int as isize) as libc::c_int
                        == 46 as libc::c_int)
                    {
                        current_block_65 = 2461986818656919617;
                    } else {
                        current_block_65 = 3580086814630675314;
                    }
                } else {
                    current_block_65 = 2461986818656919617;
                }
                match current_block_65 {
                    2461986818656919617 => {
                        if next.offset_from(src) as libc::c_long != 0 {
                            tmp___8 = dst___0;
                            dst___0 = dst___0.offset(1);
                            *tmp___8 = '/' as i32 as libc::c_char;
                            xstrsncpy(
                                dst___0,
                                src,
                                (next.offset_from(src) as libc::c_long + 1 as libc::c_long)
                                    as size_t,
                            );
                            dst___0 = dst___0
                                .offset(next.offset_from(src) as libc::c_long as isize);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        src = next.offset(1 as libc::c_int as isize);
        len = src_size.wrapping_sub(src.offset_from(path) as libc::c_long as size_t);
    }
    if *resolved_path as libc::c_int == 0 as libc::c_int {
        *resolved_path.offset(0 as libc::c_int as isize) = '/' as i32 as libc::c_char;
        *resolved_path
            .offset(1 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    }
    return resolved_path;
}
unsafe extern "C" fn set_tilde_in_path(mut path: *mut libc::c_char) -> bool {
    let mut tmp: bool = false;
    tmp = is_prefix(
        path as *const libc::c_char,
        home as *const libc::c_char,
        homelen as size_t,
    );
    if tmp {
        *home
            .offset(
                homelen as libc::c_int as isize,
            ) = *path.offset((homelen as libc::c_int - 1 as libc::c_int) as isize);
        *path
            .offset(
                (homelen as libc::c_int - 1 as libc::c_int) as isize,
            ) = '~' as i32 as libc::c_char;
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn reset_tilde_in_path(mut path: *mut libc::c_char) {
    *path
        .offset(
            (homelen as libc::c_int - 1 as libc::c_int) as isize,
        ) = *home.offset(homelen as libc::c_int as isize);
    *home.offset(homelen as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
}
unsafe extern "C" fn convert_tilde(
    mut path: *const libc::c_char,
    mut buf: *mut libc::c_char,
) {
    let mut len: ssize_t = 0;
    let mut tmp: size_t = 0;
    let mut loclen: ssize_t = 0;
    let mut tmp___0: size_t = 0;
    if *path.offset(0 as libc::c_int as isize) as libc::c_int == 126 as libc::c_int {
        tmp = xstrlen(home as *const libc::c_char);
        len = tmp as ssize_t;
        tmp___0 = xstrlen(path);
        loclen = tmp___0 as ssize_t;
        xstrsncpy(buf, home as *const libc::c_char, (len + 1 as libc::c_long) as size_t);
        xstrsncpy(
            buf.offset(len as isize),
            path.offset(1 as libc::c_int as isize),
            loclen as size_t,
        );
    }
}
unsafe extern "C" fn create_tmp_file() -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    xstrsncpy(
        g_tmpfpath
            .as_mut_ptr()
            .offset(tmpfplen as libc::c_int as isize)
            .offset(-(1 as libc::c_int as isize)),
        messages[2 as libc::c_int as usize],
        (64 as libc::c_int - tmpfplen as libc::c_int) as size_t,
    );
    tmp = mkstemp(g_tmpfpath.as_mut_ptr());
    fd = tmp;
    return fd;
}
unsafe extern "C" fn msg(mut message: *const libc::c_char) {
    dprintf(2 as libc::c_int, b"%s\n\0" as *const u8 as *const libc::c_char, message);
}
unsafe extern "C" fn handle_key_resize() {
    endwin();
    wrefresh(stdscr);
}
unsafe extern "C" fn clearoldprompt() {
    wmove(stdscr, xlines as libc::c_int - 2 as libc::c_int, 0 as libc::c_int);
    wclrtoeol(stdscr);
    wmove(stdscr, xlines as libc::c_int - 1 as libc::c_int, 0 as libc::c_int);
    wclrtoeol(stdscr);
    handle_key_resize();
}
#[inline]
unsafe extern "C" fn printmsg_nc(mut msg___0: *const libc::c_char) {
    wmove(stdscr, xlines as libc::c_int - 1 as libc::c_int, 0 as libc::c_int);
    waddnstr(stdscr, msg___0, -(1 as libc::c_int));
    wclrtoeol(stdscr);
}
unsafe extern "C" fn printmsg(mut msg___0: *const libc::c_char) {
    wattr_on(
        stdscr,
        cfg.curctx().wrapping_add(1 as libc::c_uint) << 8 as libc::c_int
            & ((1 as libc::c_uint) << 8 as libc::c_int).wrapping_sub(1 as libc::c_uint)
                << 8 as libc::c_int,
        0 as *mut libc::c_void,
    );
    printmsg_nc(msg___0);
    wattr_off(
        stdscr,
        cfg.curctx().wrapping_add(1 as libc::c_uint) << 8 as libc::c_int
            & ((1 as libc::c_uint) << 8 as libc::c_int).wrapping_sub(1 as libc::c_uint)
                << 8 as libc::c_int,
        0 as *mut libc::c_void,
    );
}
unsafe extern "C" fn printwait(
    mut msg___0: *const libc::c_char,
    mut presel: *mut libc::c_int,
) {
    printmsg(msg___0);
    if !presel.is_null() {
        *presel = '$' as i32;
        if ndents != 0 {
            xstrsncpy(
                (g_ctx[cfg.curctx() as usize].c_name).as_mut_ptr(),
                (*pdents.offset(cur as isize)).name as *const libc::c_char,
                256 as libc::c_int as size_t,
            );
        }
    }
}
unsafe extern "C" fn printerr(mut linenum: libc::c_int) {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    endwin();
    tmp = xitoa(linenum as uint_t);
    perror(tmp as *const libc::c_char);
    if g_state.picker() == 0 {
        if !selpath.is_null() {
            unlink(selpath as *const libc::c_char);
        }
    }
    free(pselbuf as *mut libc::c_void);
    exit(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn xconfirm(mut c: libc::c_int) -> bool {
    let mut tmp: libc::c_int = 0;
    if c == 121 as libc::c_int {
        tmp = 1 as libc::c_int;
    } else if c == 89 as libc::c_int {
        tmp = 1 as libc::c_int;
    } else {
        tmp = 0 as libc::c_int;
    }
    return tmp != 0;
}
unsafe extern "C" fn get_input(mut prompt: *const libc::c_char) -> libc::c_int {
    let mut ch: [wint_t; 1] = [0; 1];
    if !prompt.is_null() {
        printmsg(prompt);
    }
    wtimeout(stdscr, -(1 as libc::c_int));
    wget_wch(stdscr, ch.as_mut_ptr());
    while ch[0 as libc::c_int as usize] == 410 as libc::c_uint {
        if !prompt.is_null() {
            clearoldprompt();
            xlines = LINES as ushort_t;
            printmsg(prompt);
        }
        wget_wch(stdscr, ch.as_mut_ptr());
    }
    wtimeout(stdscr, 1000 as libc::c_int);
    return ch[0 as libc::c_int as usize] as libc::c_int;
}
unsafe extern "C" fn isselfileempty() -> bool {
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
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = stat(selpath as *const libc::c_char, &mut sb as *mut stat);
    if tmp == -(1 as libc::c_int) {
        tmp___0 = 1 as libc::c_int;
    } else if sb.st_size == 0 {
        tmp___0 = 1 as libc::c_int;
    } else {
        tmp___0 = 0 as libc::c_int;
    }
    return tmp___0 != 0;
}
unsafe extern "C" fn get_cur_or_sel() -> libc::c_int {
    let mut sel: bool = false;
    let mut tmp: bool = false;
    let mut tmp___0: libc::c_int = 0;
    let mut choice: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    if selbufpos != 0 {
        tmp___0 = 1 as libc::c_int;
    } else {
        tmp = isselfileempty();
        if tmp {
            tmp___0 = 0 as libc::c_int;
        } else {
            tmp___0 = 1 as libc::c_int;
        }
    }
    sel = tmp___0 != 0;
    if sel {
        if ndents != 0 {
            if cfg.prefersel() != 0 {
                if selbufpos != 0 {
                    return 's' as i32;
                }
            }
            tmp___1 = get_input(messages[8 as libc::c_int as usize]);
            choice = tmp___1;
            if choice == 99 as libc::c_int {
                tmp___2 = choice;
            } else if choice == 115 as libc::c_int {
                tmp___2 = choice;
            } else {
                tmp___2 = 0 as libc::c_int;
            }
            return tmp___2;
        }
    }
    if sel {
        return 's' as i32;
    }
    if ndents != 0 {
        return 'c' as i32;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn xdelay(mut delay: useconds_t) {
    wrefresh(stdscr);
    usleep(delay);
}
unsafe extern "C" fn confirm_force(mut selection: bool) -> libc::c_char {
    let mut str: [libc::c_char; 64] = [0; 64];
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    if selection {
        tmp = b"selection\0" as *const u8 as *const libc::c_char;
    } else {
        tmp = b"hovered\0" as *const u8 as *const libc::c_char;
    }
    if g_state.trash() != 0 {
        tmp___0 = (utils[19 as libc::c_int as usize]).offset(4 as libc::c_int as isize);
    } else {
        tmp___0 = utils[20 as libc::c_int as usize];
    }
    snprintf(
        str.as_mut_ptr(),
        64 as libc::c_int as size_t,
        messages[9 as libc::c_int as usize],
        tmp___0,
        tmp,
    );
    tmp___1 = get_input(str.as_mut_ptr() as *const libc::c_char);
    r = tmp___1;
    if r == 27 as libc::c_int {
        return '\u{0}' as i32 as libc::c_char;
    }
    if r == 121 as libc::c_int {
        return 'f' as i32 as libc::c_char
    } else {
        if r == 89 as libc::c_int {
            return 'f' as i32 as libc::c_char;
        }
    }
    if g_state.trash() != 0 {
        tmp___2 = '\u{0}' as i32;
    } else {
        tmp___2 = 'i' as i32;
    }
    return tmp___2 as libc::c_char;
}
unsafe extern "C" fn writesel(mut buf: *const libc::c_char, buflen: size_t) {
    let mut fd: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: ssize_t = 0;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    if selpath.is_null() {
        return;
    }
    tmp = open(selpath as *const libc::c_char, 577 as libc::c_int, 438 as libc::c_int);
    fd = tmp;
    if fd != -(1 as libc::c_int) {
        tmp___2 = write(fd, buf as *const libc::c_void, buflen);
        if tmp___2 != buflen as ssize_t {
            tmp___0 = __errno_location();
            tmp___1 = strerror(*tmp___0);
            printwait(
                tmp___1 as *const libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_int,
            );
        }
        close(fd);
    } else {
        tmp___3 = __errno_location();
        tmp___4 = strerror(*tmp___3);
        printwait(
            tmp___4 as *const libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_int,
        );
    };
}
unsafe extern "C" fn appendfpath(mut path: *const libc::c_char, len: size_t) {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: size_t = 0;
    let mut current_block_5: u64;
    if selbufpos >= selbuflen {
        current_block_5 = 4733795185702969795;
    } else if len.wrapping_add(3 as libc::c_ulong)
            > selbuflen.wrapping_sub(selbufpos) as size_t
        {
        current_block_5 = 4733795185702969795;
    } else {
        current_block_5 = 14523784380283086299;
    }
    match current_block_5 {
        4733795185702969795 => {
            selbuflen = (selbuflen as libc::c_uint).wrapping_add(4096 as libc::c_uint)
                as uint_t as uint_t;
            tmp = xrealloc(pselbuf as *mut libc::c_void, selbuflen as size_t);
            pselbuf = tmp as *mut libc::c_char;
            if pselbuf.is_null() {
                printerr(1465 as libc::c_int);
            }
        }
        _ => {}
    }
    tmp___0 = xstrsncpy(pselbuf.offset(selbufpos as isize), path, len);
    selbufpos = (selbufpos as size_t).wrapping_add(tmp___0) as uint_t;
}
unsafe extern "C" fn selbufrealloc(alloclen: size_t) {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    if (selbufpos as size_t).wrapping_add(alloclen) > selbuflen as size_t {
        selbuflen = (selbufpos as size_t)
            .wrapping_add(alloclen)
            .wrapping_add(4096 as libc::c_ulong)
            .wrapping_sub(1 as libc::c_ulong)
            .wrapping_div(4096 as libc::c_ulong)
            .wrapping_mul(4096 as libc::c_ulong) as uint_t;
        tmp = xrealloc(pselbuf as *mut libc::c_void, selbuflen as size_t);
        pselbuf = tmp as *mut libc::c_char;
        if pselbuf.is_null() {
            printerr(1477 as libc::c_int);
        }
    }
}
unsafe extern "C" fn seltofile(mut fd: libc::c_int, mut pcount: *mut uint_t) -> size_t {
    let mut lastpos: uint_t = 0;
    let mut count: uint_t = 0;
    let mut pbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pos: size_t = 0;
    let mut len: ssize_t = 0;
    let mut prefixlen: ssize_t = 0;
    let mut initlen: ssize_t = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: ssize_t = 0;
    let mut tmp___3: ssize_t = 0;
    let mut tmp___4: ssize_t = 0;
    let mut tmp___5: bool = false;
    let mut tmp___6: ssize_t = 0;
    count = 0 as libc::c_int as uint_t;
    pbuf = pselbuf;
    pos = 0 as libc::c_int as size_t;
    prefixlen = 0 as libc::c_int as ssize_t;
    initlen = 0 as libc::c_int as ssize_t;
    if !pcount.is_null() {
        *pcount = 0 as libc::c_int as uint_t;
    }
    if selbufpos == 0 {
        return 0 as libc::c_int as size_t;
    }
    lastpos = selbufpos.wrapping_sub(1 as libc::c_uint);
    if !listpath.is_null() {
        tmp = xstrlen(listroot as *const libc::c_char);
        prefixlen = tmp as ssize_t;
        tmp___0 = xstrlen(listpath as *const libc::c_char);
        initlen = tmp___0 as ssize_t;
    }
    while pos <= lastpos as size_t {
        tmp___1 = xstrlen(pbuf as *const libc::c_char);
        len = tmp___1 as ssize_t;
        let mut current_block_33: u64;
        if listpath.is_null() {
            current_block_33 = 5862380932769338654;
        } else {
            tmp___5 = is_prefix(
                pbuf as *const libc::c_char,
                listpath as *const libc::c_char,
                initlen as size_t,
            );
            if tmp___5 {
                tmp___3 = write(
                    fd,
                    listroot as *const libc::c_void,
                    prefixlen as size_t,
                );
                if tmp___3 != prefixlen {
                    return pos;
                }
                tmp___4 = write(
                    fd,
                    pbuf.offset(initlen as isize) as *const libc::c_void,
                    (len - initlen) as size_t,
                );
                if tmp___4 != len - initlen {
                    return pos;
                }
                current_block_33 = 8704759739624374314;
            } else {
                current_block_33 = 5862380932769338654;
            }
        }
        match current_block_33 {
            5862380932769338654 => {
                tmp___2 = write(fd, pbuf as *const libc::c_void, len as size_t);
                if tmp___2 != len {
                    return pos;
                }
            }
            _ => {}
        }
        pos = (pos as libc::c_ulong).wrapping_add(len as size_t) as size_t as size_t;
        if pos <= lastpos as size_t {
            tmp___6 = write(
                fd,
                b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
            if tmp___6 != 1 as libc::c_long {
                return pos;
            }
            pbuf = pbuf.offset((len + 1 as libc::c_long) as isize);
        }
        pos = pos.wrapping_add(1);
        count = count.wrapping_add(1);
    }
    if !pcount.is_null() {
        *pcount = count;
    }
    return pos;
}
unsafe extern "C" fn listselfile() -> bool {
    let mut tmp: bool = false;
    tmp = isselfileempty();
    if tmp {
        return 0 as libc::c_int != 0;
    }
    snprintf(
        g_buf.as_mut_ptr(),
        (4096 as libc::c_int + ((256 as libc::c_int) << 1 as libc::c_int)) as size_t,
        b"tr '\\0' '\\n' < %s\0" as *const u8 as *const libc::c_char,
        selpath,
    );
    spawn(
        utils[7 as libc::c_int as usize],
        g_buf.as_mut_ptr(),
        0 as *mut libc::c_void as *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
        25 as libc::c_int as ushort_t,
    );
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn resetselind() {
    let mut r: libc::c_int = 0;
    r = 0 as libc::c_int;
    while r < ndents {
        if ((*pdents.offset(r as isize)).__annonCompField18).flags()
            & 16 as libc::c_ulonglong != 0
        {
            let ref mut fresh0 = (*pdents.offset(r as isize)).__annonCompField18;
            (*fresh0)
                .set_flags(
                    (*fresh0).flags()
                        & 0xffffffffffffffef as libc::c_ulonglong as ullong_t,
                );
        }
        r += 1;
    }
}
unsafe extern "C" fn startselection() {
    if g_state.selmode() == 0 {
        g_state.set_selmode(1 as libc::c_int as uint_t);
        nselected = 0 as libc::c_int;
        if selbufpos != 0 {
            resetselind();
            writesel(
                0 as *mut libc::c_void as *const libc::c_char,
                0 as libc::c_int as size_t,
            );
            selbufpos = 0 as libc::c_int as uint_t;
        }
    }
}
unsafe extern "C" fn clearselection() {
    nselected = 0 as libc::c_int;
    selbufpos = 0 as libc::c_int as uint_t;
    g_state.set_selmode(0 as libc::c_int as uint_t);
    writesel(0 as *mut libc::c_void as *const libc::c_char, 0 as libc::c_int as size_t);
}
unsafe extern "C" fn findinsel(
    mut startpos: *mut libc::c_char,
    mut len: libc::c_int,
) -> *mut libc::c_char {
    let mut found___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buflen: size_t = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    if selbufpos == 0 {
        return 0 as *mut libc::c_char;
    }
    if startpos.is_null() {
        startpos = pselbuf;
    }
    found___0 = startpos;
    buflen = (selbufpos as libc::c_long - startpos.offset_from(pselbuf) as libc::c_long)
        as size_t;
    loop {
        tmp = memmem(
            found___0 as *const libc::c_void,
            buflen
                .wrapping_sub(found___0.offset_from(startpos) as libc::c_long as size_t),
            g_sel.as_mut_ptr() as *const libc::c_void,
            len as size_t,
        );
        found___0 = tmp as *mut libc::c_char;
        if found___0.is_null() {
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
        if found___0 as libc::c_ulong == startpos as libc::c_ulong {
            return found___0
        } else {
            if *found___0.offset(-(1 as libc::c_int as isize)) as libc::c_int
                == 0 as libc::c_int
            {
                return found___0;
            }
        }
        found___0 = found___0.offset(len as isize);
        if found___0 as libc::c_ulong
            >= startpos.offset(buflen as isize) as libc::c_ulong
        {
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
    };
}
unsafe extern "C" fn markcmp(
    mut va: *const libc::c_void,
    mut vb: *const libc::c_void,
) -> libc::c_int {
    let mut ma: *const selmark = 0 as *const selmark;
    let mut mb: *const selmark = 0 as *const selmark;
    ma = va as *mut selmark as *const selmark;
    mb = vb as *mut selmark as *const selmark;
    return ((*ma).startpos).offset_from((*mb).startpos) as libc::c_long as libc::c_int;
}
#[inline]
unsafe extern "C" fn findmarkentry(mut len: size_t, mut dentp: *mut entry) {
    let mut tmp: size_t = 0;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    if ((*dentp).__annonCompField18).flags() & 32 as libc::c_ulonglong == 0 {
        tmp = xstrsncpy(
            g_sel.as_mut_ptr().offset(len as isize),
            (*dentp).name as *const libc::c_char,
            ((*dentp).__annonCompField18).nlen() as size_t,
        );
        tmp___0 = findinsel(findselpos, len.wrapping_add(tmp) as libc::c_int);
        if !tmp___0.is_null() {
            ((*dentp).__annonCompField18)
                .set_flags(
                    ((*dentp).__annonCompField18).flags()
                        | 16 as libc::c_ulonglong as ullong_t,
                );
        }
        ((*dentp).__annonCompField18)
            .set_flags(
                ((*dentp).__annonCompField18).flags()
                    | 32 as libc::c_ulonglong as ullong_t,
            );
    }
}
unsafe extern "C" fn invertselbuf(pathlen: libc::c_int) {
    let mut len: size_t = 0;
    let mut endpos: size_t = 0;
    let mut shrinklen: size_t = 0;
    let mut alloclen: size_t = 0;
    let mut pbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut found___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut nmarked: libc::c_int = 0;
    let mut prev: libc::c_int = 0;
    let mut dentp: *mut entry = 0 as *mut entry;
    let mut scan: bool = false;
    let mut marked: *mut selmark = 0 as *mut selmark;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: size_t = 0;
    shrinklen = 0 as libc::c_int as size_t;
    alloclen = 0 as libc::c_int as size_t;
    pbuf = g_sel.as_mut_ptr().offset(pathlen as isize);
    nmarked = 0 as libc::c_int;
    prev = 0 as libc::c_int;
    scan = 0 as libc::c_int != 0;
    tmp = malloc(
        (nselected as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<selmark>() as libc::c_ulong),
    );
    marked = tmp as *mut selmark;
    if marked.is_null() {
        tmp___0 = __errno_location();
        tmp___1 = strerror(*tmp___0);
        printwait(
            tmp___1 as *const libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        return;
    }
    i = 0 as libc::c_int;
    while i < ndents {
        dentp = pdents.offset(i as isize);
        if ((*dentp).__annonCompField18).flags() & 32 as libc::c_ulonglong != 0 {
            if ((*dentp).__annonCompField18).flags() & 16 as libc::c_ulonglong != 0 {
                ((*dentp).__annonCompField18)
                    .set_flags(
                        ((*dentp).__annonCompField18).flags()
                            ^ 16 as libc::c_ulonglong as ullong_t,
                    );
                scan = 1 as libc::c_int != 0;
            } else {
                ((*dentp).__annonCompField18)
                    .set_flags(
                        ((*dentp).__annonCompField18).flags()
                            | 16 as libc::c_ulonglong as ullong_t,
                    );
                alloclen = (alloclen as ullong_t)
                    .wrapping_add(
                        (pathlen as ullong_t)
                            .wrapping_add(((*dentp).__annonCompField18).nlen()),
                    ) as size_t;
            }
        } else {
            ((*dentp).__annonCompField18)
                .set_flags(
                    ((*dentp).__annonCompField18).flags()
                        | 32 as libc::c_ulonglong as ullong_t,
                );
            scan = 1 as libc::c_int != 0;
        }
        if scan {
            tmp___2 = xstrsncpy(
                pbuf,
                (*dentp).name as *const libc::c_char,
                255 as libc::c_int as size_t,
            );
            len = (pathlen as size_t).wrapping_add(tmp___2);
            found___0 = findinsel(findselpos, len as libc::c_int);
            if !found___0.is_null() {
                if findselpos as libc::c_ulong == found___0 as libc::c_ulong {
                    findselpos = findselpos.offset(len as isize);
                }
                if nmarked != 0 {
                    if found___0 as libc::c_ulong
                        == ((*marked.offset((nmarked - 1 as libc::c_int) as isize))
                            .startpos)
                            .offset(
                                (*marked.offset((nmarked - 1 as libc::c_int) as isize)).len
                                    as isize,
                            ) as libc::c_ulong
                    {
                        let ref mut fresh1 = (*marked
                            .offset((nmarked - 1 as libc::c_int) as isize))
                            .len;
                        *fresh1 = (*fresh1 as libc::c_ulong).wrapping_add(len) as size_t
                            as size_t;
                    } else {
                        let ref mut fresh2 = (*marked.offset(nmarked as isize)).startpos;
                        *fresh2 = found___0;
                        (*marked.offset(nmarked as isize)).len = len;
                        nmarked += 1;
                    }
                } else {
                    let ref mut fresh3 = (*marked.offset(nmarked as isize)).startpos;
                    *fresh3 = found___0;
                    (*marked.offset(nmarked as isize)).len = len;
                    nmarked += 1;
                }
                nselected -= 1;
                shrinklen = (shrinklen as libc::c_ulong).wrapping_add(len) as size_t
                    as size_t;
            } else {
                ((*dentp).__annonCompField18)
                    .set_flags(
                        ((*dentp).__annonCompField18).flags()
                            | 16 as libc::c_ulonglong as ullong_t,
                    );
                alloclen = (alloclen as ullong_t)
                    .wrapping_add(
                        (pathlen as ullong_t)
                            .wrapping_add(((*dentp).__annonCompField18).nlen()),
                    ) as size_t;
            }
            scan = 0 as libc::c_int != 0;
        }
        i += 1;
    }
    qsort(
        marked as *mut libc::c_void,
        nmarked as size_t,
        ::std::mem::size_of::<selmark>() as libc::c_ulong,
        Some(
            markcmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 1 as libc::c_int;
    while i < nmarked {
        if (*marked.offset(i as isize)).startpos as libc::c_ulong
            == ((*marked.offset(prev as isize)).startpos)
                .offset((*marked.offset(prev as isize)).len as isize) as libc::c_ulong
        {
            let ref mut fresh4 = (*marked.offset(prev as isize)).len;
            *fresh4 = (*fresh4 as libc::c_ulong)
                .wrapping_add((*marked.offset(i as isize)).len) as size_t as size_t;
        } else {
            prev += 1;
            let ref mut fresh5 = (*marked.offset(prev as isize)).startpos;
            *fresh5 = (*marked.offset(i as isize)).startpos;
            (*marked.offset(prev as isize)).len = (*marked.offset(i as isize)).len;
        }
        i += 1;
    }
    if nmarked != 0 {
        nmarked = prev + 1 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < nmarked {
        found___0 = (*marked.offset(i as isize)).startpos;
        if i + 1 as libc::c_int == nmarked {
            endpos = selbufpos as size_t;
        } else {
            endpos = ((*marked.offset((i + 1 as libc::c_int) as isize)).startpos)
                .offset_from(pselbuf) as libc::c_long as size_t;
        }
        len = (*marked.offset(i as isize)).len;
        memmove(
            found___0 as *mut libc::c_void,
            found___0.offset(len as isize) as *const libc::c_void,
            endpos
                .wrapping_sub(
                    found___0.offset(len as isize).offset_from(pselbuf) as libc::c_long
                        as size_t,
                ),
        );
        i += 1;
    }
    free(marked as *mut libc::c_void);
    selbufpos = (selbufpos as size_t).wrapping_sub(shrinklen) as uint_t;
    selbufrealloc(alloclen);
    i = 0 as libc::c_int;
    while i < ndents {
        if ((*pdents.offset(i as isize)).__annonCompField18).flags()
            & 16 as libc::c_ulonglong != 0
        {
            tmp___3 = xstrsncpy(
                pbuf,
                (*pdents.offset(i as isize)).name as *const libc::c_char,
                255 as libc::c_int as size_t,
            );
            len = (pathlen as size_t).wrapping_add(tmp___3);
            appendfpath(g_sel.as_mut_ptr() as *const libc::c_char, len);
            nselected += 1;
        }
        i += 1;
    }
    if nselected != 0 {
        writesel(
            pselbuf as *const libc::c_char,
            selbufpos.wrapping_sub(1 as libc::c_uint) as size_t,
        );
    } else {
        clearselection();
    };
}
unsafe extern "C" fn addtoselbuf(
    pathlen: libc::c_int,
    mut startid: libc::c_int,
    mut endid: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut len: size_t = 0;
    let mut alloclen: size_t = 0;
    let mut dentp: *mut entry = 0 as *mut entry;
    let mut found___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    alloclen = 0 as libc::c_int as size_t;
    pbuf = g_sel.as_mut_ptr().offset(pathlen as isize);
    i = startid;
    while i <= endid {
        dentp = pdents.offset(i as isize);
        if !findselpos.is_null() {
            tmp = xstrsncpy(
                pbuf,
                (*dentp).name as *const libc::c_char,
                255 as libc::c_int as size_t,
            );
            len = (pathlen as size_t).wrapping_add(tmp);
            found___0 = findinsel(findselpos, len as libc::c_int);
            if !found___0.is_null() {
                ((*dentp).__annonCompField18)
                    .set_flags(
                        ((*dentp).__annonCompField18).flags()
                            | 48 as libc::c_ulonglong as ullong_t,
                    );
                if found___0 as libc::c_ulong == findselpos as libc::c_ulong {
                    findselpos = findselpos.offset(len as isize);
                    if findselpos as libc::c_ulong
                        == pselbuf.offset(selbufpos as isize) as libc::c_ulong
                    {
                        findselpos = 0 as *mut libc::c_void as *mut libc::c_char;
                    }
                }
            } else {
                alloclen = (alloclen as ullong_t)
                    .wrapping_add(
                        (pathlen as ullong_t)
                            .wrapping_add(((*dentp).__annonCompField18).nlen()),
                    ) as size_t;
            }
        } else {
            alloclen = (alloclen as ullong_t)
                .wrapping_add(
                    (pathlen as ullong_t)
                        .wrapping_add(((*dentp).__annonCompField18).nlen()),
                ) as size_t;
        }
        i += 1;
    }
    selbufrealloc(alloclen);
    i = startid;
    while i <= endid {
        if ((*pdents.offset(i as isize)).__annonCompField18).flags()
            & 16 as libc::c_ulonglong == 0
        {
            tmp___0 = xstrsncpy(
                pbuf,
                (*pdents.offset(i as isize)).name as *const libc::c_char,
                255 as libc::c_int as size_t,
            );
            len = (pathlen as size_t).wrapping_add(tmp___0);
            appendfpath(g_sel.as_mut_ptr() as *const libc::c_char, len);
            nselected += 1;
            let ref mut fresh6 = (*pdents.offset(i as isize)).__annonCompField18;
            (*fresh6).set_flags((*fresh6).flags() | 48 as libc::c_ulonglong as ullong_t);
        }
        i += 1;
    }
    writesel(
        pselbuf as *const libc::c_char,
        selbufpos.wrapping_sub(1 as libc::c_uint) as size_t,
    );
}
unsafe extern "C" fn rmfromselbuf(mut len: size_t) {
    let mut found___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = findinsel(findselpos, len as libc::c_int);
    found___0 = tmp;
    if found___0.is_null() {
        return;
    }
    memmove(
        found___0 as *mut libc::c_void,
        found___0.offset(len as isize) as *const libc::c_void,
        (selbufpos as libc::c_long
            - found___0.offset(len as isize).offset_from(pselbuf) as libc::c_long)
            as size_t,
    );
    selbufpos = (selbufpos as size_t).wrapping_sub(len) as uint_t;
    if nselected != 0 {
        writesel(
            pselbuf as *const libc::c_char,
            selbufpos.wrapping_sub(1 as libc::c_uint) as size_t,
        );
    } else {
        clearselection();
    };
}
unsafe extern "C" fn scanselforpath(
    mut path: *const libc::c_char,
    mut getsize: bool,
) -> libc::c_int {
    let mut off: size_t = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    if *path.offset(1 as libc::c_int as isize) == 0 {
        g_sel[0 as libc::c_int as usize] = '/' as i32 as libc::c_char;
        findselpos = pselbuf;
        return 1 as libc::c_int;
    }
    tmp = xstrsncpy(g_sel.as_mut_ptr(), path, 4096 as libc::c_int as size_t);
    off = tmp;
    g_sel[off.wrapping_sub(1 as libc::c_ulong) as usize] = '/' as i32 as libc::c_char;
    findselpos = findinsel(
        0 as *mut libc::c_void as *mut libc::c_char,
        off as libc::c_int,
    );
    if getsize {
        return off as libc::c_int;
    }
    if !findselpos.is_null() {
        tmp___0 = off;
    } else {
        tmp___0 = 0 as libc::c_int as size_t;
    }
    return tmp___0 as libc::c_int;
}
unsafe extern "C" fn endselection(mut endselmode: bool) {
    let mut fd: libc::c_int = 0;
    let mut count: ssize_t = 0;
    let mut buf: [libc::c_char; 4232] = [0; 4232];
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___8: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___13: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___14: libc::c_int = 0;
    let mut tmp___15: libc::c_int = 0;
    if endselmode {
        if g_state.selmode() != 0 {
            g_state.set_selmode(0 as libc::c_int as uint_t);
        }
    }
    if listpath.is_null() {
        return
    } else {
        if selbufpos == 0 {
            return;
        }
    }
    fd = create_tmp_file();
    if fd == -(1 as libc::c_int) {
        return;
    }
    seltofile(fd, 0 as *mut libc::c_void as *mut uint_t);
    tmp___1 = close(fd);
    if tmp___1 != 0 {
        tmp = __errno_location();
        tmp___0 = strerror(*tmp);
        printwait(
            tmp___0 as *const libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        return;
    }
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4232]>() as libc::c_ulong,
        patterns[3 as libc::c_int as usize],
        listpath,
        listroot,
        g_tmpfpath.as_mut_ptr(),
    );
    spawn(
        utils[7 as libc::c_int as usize],
        buf.as_mut_ptr(),
        0 as *mut libc::c_void as *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
        9 as libc::c_int as ushort_t,
    );
    fd = open(g_tmpfpath.as_mut_ptr() as *const libc::c_char, 0 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        tmp___2 = __errno_location();
        tmp___3 = strerror(*tmp___2);
        printwait(
            tmp___3 as *const libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        tmp___6 = unlink(g_tmpfpath.as_mut_ptr() as *const libc::c_char);
        if tmp___6 != 0 {
            tmp___4 = __errno_location();
            tmp___5 = strerror(*tmp___4);
            printwait(
                tmp___5 as *const libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_int,
            );
        }
        return;
    }
    count = read(fd, pselbuf as *mut libc::c_void, selbuflen as size_t);
    if count < 0 as libc::c_long {
        tmp___7 = __errno_location();
        tmp___8 = strerror(*tmp___7);
        printwait(
            tmp___8 as *const libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        tmp___9 = close(fd);
        if tmp___9 != 0 {
            tmp___11 = 1 as libc::c_int;
        } else {
            tmp___10 = unlink(g_tmpfpath.as_mut_ptr() as *const libc::c_char);
            if tmp___10 != 0 {
                tmp___11 = 1 as libc::c_int;
            } else {
                tmp___11 = 0 as libc::c_int;
            }
        }
        return;
    }
    tmp___14 = close(fd);
    if tmp___14 != 0 {
        tmp___12 = __errno_location();
        tmp___13 = strerror(*tmp___12);
        printwait(
            tmp___13 as *const libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        return;
    } else {
        tmp___15 = unlink(g_tmpfpath.as_mut_ptr() as *const libc::c_char);
        if tmp___15 != 0 {
            tmp___12 = __errno_location();
            tmp___13 = strerror(*tmp___12);
            printwait(
                tmp___13 as *const libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_int,
            );
            return;
        }
    }
    selbufpos = count as uint_t;
    count -= 1;
    *pselbuf.offset(count as isize) = '\u{0}' as i32 as libc::c_char;
    count -= 1;
    while count > 0 as libc::c_long {
        if *pselbuf.offset(count as isize) as libc::c_int == 10 as libc::c_int {
            if *pselbuf.offset((count + 1 as libc::c_long) as isize) as libc::c_int
                == 47 as libc::c_int
            {
                *pselbuf.offset(count as isize) = '\u{0}' as i32 as libc::c_char;
            }
        }
        count -= 1;
    }
    writesel(
        pselbuf as *const libc::c_char,
        selbufpos.wrapping_sub(1 as libc::c_uint) as size_t,
    );
}
unsafe extern "C" fn editselection() -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    let mut lines: libc::c_int = 0;
    let mut count: ssize_t = 0;
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
    let mut mtime: time_t = 0;
    let mut tmp: bool = false;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___10: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: libc::c_int = 0;
    ret = -(1 as libc::c_int);
    lines = 0 as libc::c_int;
    if selbufpos == 0 {
        tmp = listselfile();
        return tmp as libc::c_int;
    }
    fd = create_tmp_file();
    if fd == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    seltofile(fd, 0 as *mut libc::c_void as *mut uint_t);
    tmp___0 = close(fd);
    if tmp___0 != 0 {
        return -(1 as libc::c_int);
    }
    tmp___1 = stat(g_tmpfpath.as_mut_ptr() as *const libc::c_char, &mut sb as *mut stat);
    if tmp___1 != 0 {
        unlink(g_tmpfpath.as_mut_ptr() as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    mtime = sb.st_mtim.tv_sec;
    if cfg.waitedit() != 0 {
        tmp___2 = enveditor;
    } else {
        tmp___2 = editor;
    }
    spawn(
        tmp___2,
        g_tmpfpath.as_mut_ptr(),
        0 as *mut libc::c_void as *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
        9 as libc::c_int as ushort_t,
    );
    fd = open(g_tmpfpath.as_mut_ptr() as *const libc::c_char, 0 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        unlink(g_tmpfpath.as_mut_ptr() as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    fstat(fd, &mut sb);
    if mtime == sb.st_mtim.tv_sec {
        unlink(g_tmpfpath.as_mut_ptr() as *const libc::c_char);
        return 1 as libc::c_int;
    }
    if sb.st_size > selbufpos as __off_t {
        unlink(g_tmpfpath.as_mut_ptr() as *const libc::c_char);
    } else {
        count = read(fd, pselbuf as *mut libc::c_void, selbuflen as size_t);
        if count < 0 as libc::c_long {
            tmp___3 = __errno_location();
            tmp___4 = strerror(*tmp___3);
            printwait(
                tmp___4 as *const libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_int,
            );
            tmp___7 = close(fd);
            if tmp___7 != 0 {
                tmp___5 = __errno_location();
                tmp___6 = strerror(*tmp___5);
                printwait(
                    tmp___6 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_int,
                );
            } else {
                tmp___8 = unlink(g_tmpfpath.as_mut_ptr() as *const libc::c_char);
                if tmp___8 != 0 {
                    tmp___5 = __errno_location();
                    tmp___6 = strerror(*tmp___5);
                    printwait(
                        tmp___6 as *const libc::c_char,
                        0 as *mut libc::c_void as *mut libc::c_int,
                    );
                }
            }
        } else {
            tmp___11 = close(fd);
            if tmp___11 != 0 {
                tmp___9 = __errno_location();
                tmp___10 = strerror(*tmp___9);
                printwait(
                    tmp___10 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_int,
                );
            } else {
                tmp___12 = unlink(g_tmpfpath.as_mut_ptr() as *const libc::c_char);
                if tmp___12 != 0 {
                    tmp___9 = __errno_location();
                    tmp___10 = strerror(*tmp___9);
                    printwait(
                        tmp___10 as *const libc::c_char,
                        0 as *mut libc::c_void as *mut libc::c_int,
                    );
                } else if count == 0 {
                    ret = 1 as libc::c_int;
                } else {
                    resetselind();
                    selbufpos = count as uint_t;
                    count -= 1;
                    *pselbuf.offset(count as isize) = '\u{0}' as i32 as libc::c_char;
                    count -= 1;
                    while count > 0 as libc::c_long {
                        if *pselbuf.offset(count as isize) as libc::c_int
                            == 10 as libc::c_int
                        {
                            if *pselbuf.offset((count + 1 as libc::c_long) as isize)
                                as libc::c_int == 47 as libc::c_int
                            {
                                lines += 1;
                                *pselbuf
                                    .offset(count as isize) = '\u{0}' as i32 as libc::c_char;
                            }
                        }
                        count -= 1;
                    }
                    lines += 1;
                    if !(lines > nselected) {
                        nselected = lines;
                        writesel(
                            pselbuf as *const libc::c_char,
                            selbufpos.wrapping_sub(1 as libc::c_uint) as size_t,
                        );
                        return 1 as libc::c_int;
                    }
                }
            }
        }
    }
    resetselind();
    clearselection();
    return ret;
}
unsafe extern "C" fn selsafe() -> bool {
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: libc::c_int = 0;
    if selpath.is_null() {
        printmsg(messages[23 as libc::c_int as usize]);
        return 0 as libc::c_int != 0;
    }
    tmp___2 = access(selpath as *const libc::c_char, 6 as libc::c_int);
    if tmp___2 == -(1 as libc::c_int) {
        tmp___1 = __errno_location();
        if *tmp___1 == 2 as libc::c_int {
            printmsg(messages[3 as libc::c_int as usize]);
        } else {
            tmp = __errno_location();
            tmp___0 = strerror(*tmp);
            printwait(
                tmp___0 as *const libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn export_file_list() {
    let mut pdent: *mut entry = 0 as *mut entry;
    let mut fd: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut tmp___0: ssize_t = 0;
    let mut tmp___1: ssize_t = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: bool = false;
    if ndents == 0 {
        return;
    }
    pdent = pdents;
    tmp = create_tmp_file();
    fd = tmp;
    if fd == -(1 as libc::c_int) {
        return;
    }
    r = 0 as libc::c_int;
    while r < ndents {
        tmp___0 = write(
            fd,
            (*pdent).name as *const libc::c_void,
            ((*pdent).__annonCompField18).nlen().wrapping_sub(1 as libc::c_ulonglong)
                as size_t,
        );
        if tmp___0 as ullong_t
            != ((*pdent).__annonCompField18).nlen().wrapping_sub(1 as libc::c_ulonglong)
        {
            break;
        }
        if r != ndents - 1 as libc::c_int {
            tmp___1 = write(
                fd,
                b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
            if tmp___1 != 1 as libc::c_long {
                break;
            }
        }
        pdent = pdent.offset(1);
        r += 1;
    }
    close(fd);
    spawn(
        editor,
        g_tmpfpath.as_mut_ptr(),
        0 as *mut libc::c_void as *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
        9 as libc::c_int as ushort_t,
    );
    tmp___2 = get_input(messages[39 as libc::c_int as usize]);
    tmp___3 = xconfirm(tmp___2);
    if tmp___3 {
        unlink(g_tmpfpath.as_mut_ptr() as *const libc::c_char);
    }
}
unsafe extern "C" fn init_fcolors() -> bool {
    let mut f_colors: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut id: uchar_t = 0;
    let mut tmp___0: uchar_t = 0;
    let mut tmp___1: uchar_t = 0;
    tmp = getenv(env_cfg[5 as libc::c_int as usize]);
    f_colors = tmp;
    if f_colors.is_null() {
        f_colors = gcolors.as_mut_ptr();
    } else if *f_colors == 0 {
        f_colors = gcolors.as_mut_ptr();
    }
    id = 5 as libc::c_int as uchar_t;
    while *f_colors != 0 {
        if !(id as libc::c_int <= 16 as libc::c_int) {
            break;
        }
        tmp___0 = xchartohex(*f_colors as uchar_t);
        fcolors[id as usize] = ((tmp___0 as libc::c_int) << 4 as libc::c_int) as uint_t;
        f_colors = f_colors.offset(1);
        if *f_colors != 0 {
            tmp___1 = xchartohex(*f_colors as uchar_t);
            fcolors[id
                as usize] = (fcolors[id as usize] as libc::c_uint)
                .wrapping_add(tmp___1 as uint_t) as uint_t as uint_t;
            if fcolors[id as usize] != 0 {
                init_pair(
                    id as libc::c_short,
                    fcolors[id as usize] as libc::c_short,
                    -(1 as libc::c_int) as libc::c_short,
                );
            }
        } else {
            return 0 as libc::c_int != 0
        }
        f_colors = f_colors.offset(1);
        id = (id as libc::c_int + 1 as libc::c_int) as uchar_t;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn initcurses(mut oldmask: *mut libc::c_void) -> bool {
    let mut tmp: *mut SCREEN = 0 as *mut SCREEN;
    let mut tmp___0: *mut WINDOW = 0 as *mut WINDOW;
    let mut colors: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pcode: *mut uint_t = 0 as *mut uint_t;
    let mut ext: bool = false;
    let mut tmp___2: bool = false;
    let mut sep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: uchar_t = 0;
    let mut tmp___4: uchar_t = 0;
    let mut tmp___5: uchar_t = 0;
    let mut tmp___6: uint_t = 0;
    let mut tmp___7: *mut libc::c_char = 0 as *mut libc::c_char;
    if g_state.picker() != 0 {
        tmp = newterm(0 as *mut libc::c_void as *const libc::c_char, stderr, stdin);
        if tmp.is_null() {
            msg(b"newterm!\0" as *const u8 as *const libc::c_char);
            return 0 as libc::c_int != 0;
        }
    } else {
        tmp___0 = initscr();
        if tmp___0.is_null() {
            msg(b"initscr!\0" as *const u8 as *const libc::c_char);
            return 0 as libc::c_int != 0;
        }
    }
    cbreak();
    noecho();
    nonl();
    keypad(stdscr, 1 as libc::c_int != 0);
    curs_set(0 as libc::c_int);
    tmp___1 = getenv(env_cfg[4 as libc::c_int as usize]);
    colors = tmp___1;
    let mut current_block_90: u64;
    if !colors.is_null() {
        current_block_90 = 2113136921683473179;
    } else {
        tmp___7 = getenv(b"NO_COLOR\0" as *const u8 as *const libc::c_char);
        if tmp___7.is_null() {
            current_block_90 = 2113136921683473179;
        } else {
            current_block_90 = 16108440464692313034;
        }
    }
    match current_block_90 {
        2113136921683473179 => {
            ext = 0 as libc::c_int != 0;
            start_color();
            use_default_colors();
            if COLORS >= 256 as libc::c_int {
                if g_state.oldcolor() == 0 {
                    tmp___2 = init_fcolors();
                    if !tmp___2 {
                        endwin();
                        msg(env_cfg[5 as libc::c_int as usize]);
                        return 0 as libc::c_int != 0;
                    }
                }
            } else {
                g_state.set_oldcolor(1 as libc::c_int as uint_t);
            }
            if !colors.is_null() {
                if *colors as libc::c_int == 35 as libc::c_int {
                    tmp___3 = strchr(colors as *const libc::c_char, ';' as i32);
                    sep = tmp___3;
                    let mut current_block_47: u64;
                    if g_state.oldcolor() == 0 {
                        if COLORS >= 256 as libc::c_int {
                            colors = colors.offset(1);
                            ext = 1 as libc::c_int != 0;
                            if !sep.is_null() {
                                *sep = '\u{0}' as i32 as libc::c_char;
                            }
                            current_block_47 = 1345366029464561491;
                        } else {
                            current_block_47 = 4698769682622422388;
                        }
                    } else {
                        current_block_47 = 4698769682622422388;
                    }
                    match current_block_47 {
                        4698769682622422388 => {
                            colors = sep;
                            if !colors.is_null() {
                                colors = colors.offset(1);
                            }
                        }
                        _ => {}
                    }
                }
            }
            i = 0 as libc::c_int as uchar_t;
            while (i as libc::c_int) < 4 as libc::c_int {
                pcode = &mut (*g_ctx.as_mut_ptr().offset(i as isize)).color;
                if !colors.is_null() {
                    if *colors != 0 {
                        if ext {
                            tmp___4 = xchartohex(*colors as uchar_t);
                            *pcode = ((tmp___4 as libc::c_int) << 4 as libc::c_int)
                                as uint_t;
                            colors = colors.offset(1);
                            if *colors != 0 {
                                tmp___5 = xchartohex(*colors as uchar_t);
                                tmp___6 = (*pcode).wrapping_add(tmp___5 as uint_t);
                                *pcode = tmp___6;
                                fcolors[(i as libc::c_int + 1 as libc::c_int)
                                    as usize] = tmp___6;
                            } else {
                                endwin();
                                msg(env_cfg[4 as libc::c_int as usize]);
                                return 0 as libc::c_int != 0;
                            }
                        } else if (*colors as libc::c_int) < 48 as libc::c_int {
                            *pcode = 4 as libc::c_int as uint_t;
                        } else if *colors as libc::c_int > 55 as libc::c_int {
                            *pcode = 4 as libc::c_int as uint_t;
                        } else {
                            *pcode = (*colors as libc::c_int - 48 as libc::c_int)
                                as uint_t;
                        }
                        colors = colors.offset(1);
                    } else {
                        *pcode = 4 as libc::c_int as uint_t;
                    }
                } else {
                    *pcode = 4 as libc::c_int as uint_t;
                }
                init_pair(
                    (i as libc::c_int + 1 as libc::c_int) as libc::c_short,
                    *pcode as libc::c_short,
                    -(1 as libc::c_int) as libc::c_short,
                );
                i = (i as libc::c_int + 1 as libc::c_int) as uchar_t;
            }
        }
        _ => {}
    }
    wtimeout(stdscr, 1000 as libc::c_int);
    set_escdelay(25 as libc::c_int);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parseargs(
    mut cmd___0: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut pindex: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut count: libc::c_int = 0;
    let mut len: size_t = 0;
    let mut tmp: size_t = 0;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: libc::c_int = 0;
    count = 0 as libc::c_int;
    tmp = xstrlen(cmd___0 as *const libc::c_char);
    len = tmp.wrapping_add(1 as libc::c_ulong);
    tmp___0 = malloc(len);
    line = tmp___0 as *mut libc::c_char;
    if line.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    xstrsncpy(line, cmd___0 as *const libc::c_char, len);
    tmp___1 = count;
    count += 1;
    let ref mut fresh7 = *argv.offset(tmp___1 as isize);
    *fresh7 = line;
    cmd___0 = line;
    let mut current_block_20: u64;
    while *line != 0 {
        if *line as libc::c_int == 32 as libc::c_int {
            current_block_20 = 5680342740982639791;
        } else if *line as libc::c_int == 9 as libc::c_int {
            current_block_20 = 5680342740982639791;
        } else {
            current_block_20 = 18317007320854588510;
        }
        match current_block_20 {
            5680342740982639791 => {
                tmp___2 = line;
                line = line.offset(1);
                *tmp___2 = '\u{0}' as i32 as libc::c_char;
                if *line == 0 {
                    break;
                }
                tmp___3 = count;
                count += 1;
                let ref mut fresh8 = *argv.offset(tmp___3 as isize);
                *fresh8 = line;
                if count == 10 as libc::c_int {
                    count = -(1 as libc::c_int);
                    break;
                }
            }
            _ => {}
        }
        line = line.offset(1);
    }
    if count == -(1 as libc::c_int) {
        free(cmd___0 as *mut libc::c_void);
        cmd___0 = 0 as *mut libc::c_void as *mut libc::c_char;
    } else if count > 6 as libc::c_int {
        free(cmd___0 as *mut libc::c_void);
        cmd___0 = 0 as *mut libc::c_void as *mut libc::c_char;
    }
    *pindex = count;
    return cmd___0;
}
unsafe extern "C" fn enable_signals() {
    let mut dfl_act: sigaction = sigaction {
        __sigaction_handler: __anonunion___sigaction_handler_363639592 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    dfl_act.__sigaction_handler.sa_handler = None;
    dfl_act.sa_mask.__val[0 as libc::c_int as usize] = 0 as libc::c_ulong;
    dfl_act.sa_mask.__val[1 as libc::c_int as usize] = 0 as libc::c_ulong;
    dfl_act.sa_mask.__val[2 as libc::c_int as usize] = 0 as libc::c_ulong;
    dfl_act.sa_mask.__val[3 as libc::c_int as usize] = 0 as libc::c_ulong;
    dfl_act.sa_mask.__val[4 as libc::c_int as usize] = 0 as libc::c_ulong;
    dfl_act.sa_mask.__val[5 as libc::c_int as usize] = 0 as libc::c_ulong;
    dfl_act.sa_mask.__val[6 as libc::c_int as usize] = 0 as libc::c_ulong;
    dfl_act.sa_mask.__val[7 as libc::c_int as usize] = 0 as libc::c_ulong;
    dfl_act.sa_mask.__val[8 as libc::c_int as usize] = 0 as libc::c_ulong;
    dfl_act.sa_mask.__val[9 as libc::c_int as usize] = 0 as libc::c_ulong;
    dfl_act.sa_mask.__val[10 as libc::c_int as usize] = 0 as libc::c_ulong;
    dfl_act.sa_mask.__val[11 as libc::c_int as usize] = 0 as libc::c_ulong;
    dfl_act.sa_mask.__val[12 as libc::c_int as usize] = 0 as libc::c_ulong;
    dfl_act.sa_mask.__val[13 as libc::c_int as usize] = 0 as libc::c_ulong;
    dfl_act.sa_mask.__val[14 as libc::c_int as usize] = 0 as libc::c_ulong;
    dfl_act.sa_mask.__val[15 as libc::c_int as usize] = 0 as libc::c_ulong;
    dfl_act.sa_flags = 0 as libc::c_int;
    dfl_act.sa_restorer = None;
    sigaction(
        1 as libc::c_int,
        &mut dfl_act as *mut sigaction as *const sigaction,
        0 as *mut libc::c_void as *mut sigaction,
    );
    sigaction(
        2 as libc::c_int,
        &mut dfl_act as *mut sigaction as *const sigaction,
        0 as *mut libc::c_void as *mut sigaction,
    );
    sigaction(
        3 as libc::c_int,
        &mut dfl_act as *mut sigaction as *const sigaction,
        0 as *mut libc::c_void as *mut sigaction,
    );
    sigaction(
        20 as libc::c_int,
        &mut dfl_act as *mut sigaction as *const sigaction,
        0 as *mut libc::c_void as *mut sigaction,
    );
    sigaction(
        28 as libc::c_int,
        &mut dfl_act as *mut sigaction as *const sigaction,
        0 as *mut libc::c_void as *mut sigaction,
    );
}
unsafe extern "C" fn xfork(mut flag: uchar_t) -> pid_t {
    let mut p: pid_t = 0;
    let mut tmp: __pid_t = 0;
    let mut __constr_expr_0: sigaction = sigaction {
        __sigaction_handler: __anonunion___sigaction_handler_363639592 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut __constr_expr_1: sigaction = sigaction {
        __sigaction_handler: __anonunion___sigaction_handler_363639592 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut __constr_expr_2: sigaction = sigaction {
        __sigaction_handler: __anonunion___sigaction_handler_363639592 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    tmp = fork();
    p = tmp;
    if p > 0 as libc::c_int {
        __constr_expr_0
            .__sigaction_handler
            .sa_handler = ::std::mem::transmute::<
            libc::intptr_t,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(1 as libc::c_int as libc::intptr_t);
        __constr_expr_0.sa_mask.__val[0 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_0.sa_mask.__val[1 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_0.sa_mask.__val[2 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_0.sa_mask.__val[3 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_0.sa_mask.__val[4 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_0.sa_mask.__val[5 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_0.sa_mask.__val[6 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_0.sa_mask.__val[7 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_0.sa_mask.__val[8 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_0.sa_mask.__val[9 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_0.sa_mask.__val[10 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_0.sa_mask.__val[11 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_0.sa_mask.__val[12 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_0.sa_mask.__val[13 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_0.sa_mask.__val[14 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_0.sa_mask.__val[15 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_0.sa_flags = 0 as libc::c_int;
        __constr_expr_0.sa_restorer = None;
        sigaction(
            1 as libc::c_int,
            &mut __constr_expr_0 as *mut sigaction as *const sigaction,
            &mut oldsighup as *mut sigaction,
        );
        __constr_expr_1.__sigaction_handler.sa_handler = None;
        __constr_expr_1.sa_mask.__val[0 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_1.sa_mask.__val[1 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_1.sa_mask.__val[2 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_1.sa_mask.__val[3 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_1.sa_mask.__val[4 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_1.sa_mask.__val[5 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_1.sa_mask.__val[6 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_1.sa_mask.__val[7 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_1.sa_mask.__val[8 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_1.sa_mask.__val[9 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_1.sa_mask.__val[10 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_1.sa_mask.__val[11 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_1.sa_mask.__val[12 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_1.sa_mask.__val[13 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_1.sa_mask.__val[14 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_1.sa_mask.__val[15 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_1.sa_flags = 0 as libc::c_int;
        __constr_expr_1.sa_restorer = None;
        sigaction(
            20 as libc::c_int,
            &mut __constr_expr_1 as *mut sigaction as *const sigaction,
            &mut oldsigtstp as *mut sigaction,
        );
        __constr_expr_2
            .__sigaction_handler
            .sa_handler = ::std::mem::transmute::<
            libc::intptr_t,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(1 as libc::c_int as libc::intptr_t);
        __constr_expr_2.sa_mask.__val[0 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_2.sa_mask.__val[1 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_2.sa_mask.__val[2 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_2.sa_mask.__val[3 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_2.sa_mask.__val[4 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_2.sa_mask.__val[5 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_2.sa_mask.__val[6 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_2.sa_mask.__val[7 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_2.sa_mask.__val[8 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_2.sa_mask.__val[9 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_2.sa_mask.__val[10 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_2.sa_mask.__val[11 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_2.sa_mask.__val[12 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_2.sa_mask.__val[13 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_2.sa_mask.__val[14 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_2.sa_mask.__val[15 as libc::c_int as usize] = 0 as libc::c_ulong;
        __constr_expr_2.sa_flags = 0 as libc::c_int;
        __constr_expr_2.sa_restorer = None;
        sigaction(
            28 as libc::c_int,
            &mut __constr_expr_2 as *mut sigaction as *const sigaction,
            &mut oldsigwinch as *mut sigaction,
        );
    } else if p == 0 as libc::c_int {
        if flag as libc::c_int & 2 as libc::c_int != 0 {
            p = fork();
            if p > 0 as libc::c_int {
                _exit(0 as libc::c_int);
            } else {
                if p == 0 as libc::c_int {
                    enable_signals();
                    setsid();
                    return p;
                }
            }
            perror(b"fork\0" as *const u8 as *const libc::c_char);
            _exit(1 as libc::c_int);
        }
        enable_signals();
    }
    if flag as libc::c_int & 2 as libc::c_int != 0 {
        waitpid(p, 0 as *mut libc::c_void as *mut libc::c_int, 0 as libc::c_int);
    }
    if p == -(1 as libc::c_int) {
        perror(b"fork\0" as *const u8 as *const libc::c_char);
    }
    return p;
}
unsafe extern "C" fn join(mut p: pid_t, mut flag: uchar_t) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut tmp: __pid_t = 0;
    status = 65535 as libc::c_int;
    if flag as libc::c_int & 2 as libc::c_int == 0 {
        loop {
            tmp = waitpid(p, &mut status, 0 as libc::c_int);
            if !(tmp == -(1 as libc::c_int)) {
                break;
            }
        }
        if status & 127 as libc::c_int == 0 as libc::c_int {
            status = (status & 65280 as libc::c_int) >> 8 as libc::c_int;
        }
    }
    sigaction(
        1 as libc::c_int,
        &mut oldsighup as *mut sigaction as *const sigaction,
        0 as *mut libc::c_void as *mut sigaction,
    );
    sigaction(
        20 as libc::c_int,
        &mut oldsigtstp as *mut sigaction as *const sigaction,
        0 as *mut libc::c_void as *mut sigaction,
    );
    sigaction(
        28 as libc::c_int,
        &mut oldsigwinch as *mut sigaction as *const sigaction,
        0 as *mut libc::c_void as *mut sigaction,
    );
    return status;
}
unsafe extern "C" fn spawn(
    mut file: *mut libc::c_char,
    mut arg1: *mut libc::c_char,
    mut arg2: *mut libc::c_char,
    mut arg3: *mut libc::c_char,
    mut flag: ushort_t,
) -> libc::c_int {
    let mut pid: pid_t = 0;
    let mut status: libc::c_int = 0;
    let mut retstatus: libc::c_int = 0;
    let mut argv: [*mut libc::c_char; 10] = [0 as *mut libc::c_char; 10];
    let mut tmp: libc::c_uint = 0;
    let mut cmd___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut fd___0: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: size_t = 0;
    let mut tmp___6: ssize_t = 0;
    let mut tmp___7: ssize_t = 0;
    status = 0 as libc::c_int;
    retstatus = 65535 as libc::c_int;
    argv[0 as libc::c_int as usize] = 0 as *mut libc::c_char;
    tmp = 1 as libc::c_uint;
    while !(tmp >= 10 as libc::c_uint) {
        argv[tmp as usize] = 0 as *mut libc::c_char;
        tmp = tmp.wrapping_add(1);
    }
    cmd___0 = 0 as *mut libc::c_void as *mut libc::c_char;
    if file.is_null() {
        return retstatus
    } else {
        if *file == 0 {
            return retstatus;
        }
    }
    if arg1.is_null() {
        if !arg2.is_null() {
            arg1 = arg2;
            if !arg3.is_null() {
                arg2 = arg3;
                arg3 = 0 as *mut libc::c_void as *mut libc::c_char;
            } else {
                arg2 = 0 as *mut libc::c_void as *mut libc::c_char;
            }
        }
    }
    if flag as libc::c_int & 1 as libc::c_int != 0 {
        cmd___0 = parseargs(file, argv.as_mut_ptr(), &mut status);
        if cmd___0.is_null() {
            return -(1 as libc::c_int);
        }
    } else {
        tmp___0 = status;
        status += 1;
        argv[tmp___0 as usize] = file;
    }
    argv[status as usize] = arg1;
    status += 1;
    argv[status as usize] = arg2;
    status += 1;
    argv[status as usize] = arg3;
    if flag as libc::c_int & 8 as libc::c_int != 0 {
        endwin();
    }
    pid = xfork(flag as uchar_t);
    if pid == 0 as libc::c_int {
        if flag as libc::c_int & 4 as libc::c_int != 0 {
            tmp___1 = open(
                b"/dev/null\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
                128 as libc::c_int,
            );
            fd = tmp___1;
            if flag as libc::c_int & 64 as libc::c_int != 0 {
                dup2(fd, 0 as libc::c_int);
            }
            dup2(fd, 1 as libc::c_int);
            dup2(fd, 2 as libc::c_int);
            close(fd);
        } else if flag as libc::c_int & 256 as libc::c_int != 0 {
            tmp___4 = isatty(1 as libc::c_int);
            if tmp___4 == 0 {
                tmp___2 = ctermid(0 as *mut libc::c_void as *mut libc::c_char);
                tmp___3 = open(
                    tmp___2 as *const libc::c_char,
                    1 as libc::c_int,
                    128 as libc::c_int,
                );
                fd___0 = tmp___3;
                dup2(fd___0, 1 as libc::c_int);
                close(fd___0);
            }
        }
        execvp(
            argv[0 as libc::c_int as usize] as *const libc::c_char,
            argv.as_mut_ptr() as *const *mut libc::c_char,
        );
        _exit(0 as libc::c_int);
    } else {
        retstatus = join(pid, flag as uchar_t);
        let mut current_block_73: u64;
        if flag as libc::c_int & 16 as libc::c_int != 0 {
            current_block_73 = 444392418175548731;
        } else if flag as libc::c_int & 32 as libc::c_int != 0 {
            if retstatus != 0 {
                current_block_73 = 444392418175548731;
            } else {
                current_block_73 = 4216521074440650966;
            }
        } else {
            current_block_73 = 4216521074440650966;
        }
        match current_block_73 {
            444392418175548731 => {
                tmp___5 = xstrlen(messages[22 as libc::c_int as usize]);
                tmp___6 = write(
                    1 as libc::c_int,
                    messages[22 as libc::c_int as usize] as *const libc::c_void,
                    tmp___5,
                );
                status = tmp___6 as libc::c_int;
                loop {
                    tmp___7 = read(
                        0 as libc::c_int,
                        &mut status as *mut libc::c_int as *mut libc::c_void,
                        1 as libc::c_int as size_t,
                    );
                    if !(tmp___7 > 0 as libc::c_long) {
                        break;
                    }
                    if !(status != 10 as libc::c_int) {
                        break;
                    }
                }
            }
            _ => {}
        }
        if flag as libc::c_int & 8 as libc::c_int != 0 {
            wrefresh(stdscr);
        }
        free(cmd___0 as *mut libc::c_void);
    }
    return retstatus;
}
unsafe extern "C" fn xgetenv(
    name: *const libc::c_char,
    mut fallback: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = getenv(name);
    value = tmp;
    if !value.is_null() {
        if *value.offset(0 as libc::c_int as isize) != 0 {
            tmp___0 = value;
        } else {
            tmp___0 = fallback;
        }
    } else {
        tmp___0 = fallback;
    }
    return tmp___0;
}
#[inline]
unsafe extern "C" fn xgetenv_val(mut name: *const libc::c_char) -> uint_t {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    tmp = getenv(name);
    str = tmp;
    if !str.is_null() {
        if *str.offset(0 as libc::c_int as isize) != 0 {
            tmp___0 = atoi(str as *const libc::c_char);
            return tmp___0 as uint_t;
        }
    }
    return 0 as libc::c_int as uint_t;
}
unsafe extern "C" fn xdiraccess(mut path: *const libc::c_char) -> bool {
    let mut dirp: *mut DIR = 0 as *mut DIR;
    let mut tmp: *mut DIR = 0 as *mut DIR;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = opendir(path);
    dirp = tmp;
    if dirp.is_null() {
        tmp___0 = __errno_location();
        tmp___1 = strerror(*tmp___0);
        printwait(
            tmp___1 as *const libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        return 0 as libc::c_int != 0;
    }
    closedir(dirp);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn plugscript(
    mut plugin: *const libc::c_char,
    mut flags: uchar_t,
) -> bool {
    let mut tmp: libc::c_int = 0;
    mkpath(plgpath as *const libc::c_char, plugin, g_buf.as_mut_ptr());
    tmp = access(g_buf.as_mut_ptr() as *const libc::c_char, 1 as libc::c_int);
    if tmp == 0 {
        spawn(
            g_buf.as_mut_ptr(),
            0 as *mut libc::c_void as *mut libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_char,
            flags as ushort_t,
        );
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn opstr(mut buf: *mut libc::c_char, mut op: *mut libc::c_char) {
    snprintf(
        buf,
        (4096 as libc::c_int + ((256 as libc::c_int) << 1 as libc::c_int)) as size_t,
        b"xargs -0 sh -c '%s \"$0\" \"$@\" . < /dev/tty' < %s\0" as *const u8
            as *const libc::c_char,
        op,
        selpath,
    );
}
unsafe extern "C" fn rmmulstr(mut buf: *mut libc::c_char) -> bool {
    let mut r: libc::c_char = 0;
    let mut tmp: libc::c_char = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = confirm_force(1 as libc::c_int != 0);
    r = tmp;
    if r == 0 {
        return 0 as libc::c_int != 0;
    }
    if g_state.trash() == 0 {
        snprintf(
            buf,
            (4096 as libc::c_int + ((256 as libc::c_int) << 1 as libc::c_int)) as size_t,
            b"xargs -0 sh -c 'rm -%cr \"$0\" \"$@\" < /dev/tty' < %s\0" as *const u8
                as *const libc::c_char,
            r as libc::c_int,
            selpath,
        );
    } else {
        if g_state.trash() == 1 as libc::c_uint {
            tmp___0 = 18 as libc::c_int;
        } else {
            tmp___0 = 19 as libc::c_int;
        }
        snprintf(
            buf,
            (4096 as libc::c_int + ((256 as libc::c_int) << 1 as libc::c_int)) as size_t,
            b"xargs -0 %s < %s\0" as *const u8 as *const libc::c_char,
            utils[tmp___0 as usize],
            selpath,
        );
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn xrm(fpath: *mut libc::c_char) -> bool {
    let mut r: libc::c_char = 0;
    let mut tmp: libc::c_char = 0;
    let mut rm_opts: [libc::c_char; 4] = [0; 4];
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    tmp = confirm_force(0 as libc::c_int != 0);
    r = tmp;
    if r == 0 {
        return 0 as libc::c_int != 0;
    }
    if g_state.trash() == 0 {
        rm_opts[0 as libc::c_int as usize] = '-' as i32 as libc::c_char;
        rm_opts[1 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
        rm_opts[2 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
        rm_opts[3 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        rm_opts[1 as libc::c_int as usize] = r;
        spawn(
            b"rm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            rm_opts.as_mut_ptr(),
            fpath,
            0 as *mut libc::c_void as *mut libc::c_char,
            40 as libc::c_int as ushort_t,
        );
    } else {
        if g_state.trash() == 1 as libc::c_uint {
            tmp___0 = 18 as libc::c_int;
        } else {
            tmp___0 = 19 as libc::c_int;
        }
        spawn(
            utils[tmp___0 as usize],
            fpath,
            0 as *mut libc::c_void as *mut libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_char,
            9 as libc::c_int as ushort_t,
        );
    }
    tmp___1 = access(fpath as *const libc::c_char, 0 as libc::c_int);
    return tmp___1 == -(1 as libc::c_int);
}
unsafe extern "C" fn xrmfromsel(
    mut path: *mut libc::c_char,
    mut fpath: *mut libc::c_char,
) {
    let mut tmp: size_t = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut current_block_7: u64;
    if ((*pdents.offset(cur as isize)).__annonCompField18).flags()
        & 1 as libc::c_ulonglong != 0
    {
        tmp___0 = scanselforpath(fpath as *const libc::c_char, 0 as libc::c_int != 0);
        if tmp___0 != 0 {
            clearselection();
            current_block_7 = 3640593987805443782;
        } else {
            current_block_7 = 12072953051249647693;
        }
    } else {
        current_block_7 = 12072953051249647693;
    }
    match current_block_7 {
        12072953051249647693 => {
            if ((*pdents.offset(cur as isize)).__annonCompField18).flags()
                & 16 as libc::c_ulonglong != 0
            {
                nselected -= 1;
                tmp = mkpath(
                    path as *const libc::c_char,
                    (*pdents.offset(cur as isize)).name as *const libc::c_char,
                    g_sel.as_mut_ptr(),
                );
                rmfromselbuf(tmp);
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn lines_in_file(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_char,
    mut buflen: size_t,
) -> uint_t {
    let mut len: ssize_t = 0;
    let mut count: uint_t = 0;
    let mut tmp: uint_t = 0;
    count = 0 as libc::c_int as uint_t;
    loop {
        len = read(fd, buf as *mut libc::c_void, buflen);
        if !(len > 0 as libc::c_long) {
            break;
        }
        while len != 0 {
            len -= 1;
            count = (count as libc::c_uint)
                .wrapping_add(
                    (*buf.offset(len as isize) as libc::c_int == 10 as libc::c_int)
                        as libc::c_int as uint_t,
                ) as uint_t as uint_t;
        }
    }
    if len < 0 as libc::c_long {
        tmp = 0 as libc::c_int as uint_t;
    } else {
        tmp = count;
    }
    return tmp;
}
unsafe extern "C" fn cpmv_rename(
    mut choice: libc::c_int,
    mut path: *const libc::c_char,
) -> bool {
    let mut current_block: u64;
    let mut fd: libc::c_int = 0;
    let mut count: uint_t = 0;
    let mut lines: uint_t = 0;
    let mut ret: bool = false;
    let mut cmd___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 8210] = [0; 8210];
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    count = 0 as libc::c_int as uint_t;
    lines = 0 as libc::c_int as uint_t;
    ret = 0 as libc::c_int != 0;
    if choice == 99 as libc::c_int {
        tmp = cp.as_mut_ptr();
    } else {
        tmp = mv.as_mut_ptr();
    }
    cmd___0 = tmp;
    fd = create_tmp_file();
    if fd == -(1 as libc::c_int) {
        return ret;
    }
    if selbufpos == 0 {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8210]>() as libc::c_ulong,
            b"tr '\\0' '\\n' < %s > %s\0" as *const u8 as *const libc::c_char,
            selpath,
            g_tmpfpath.as_mut_ptr(),
        );
        spawn(
            utils[7 as libc::c_int as usize],
            buf.as_mut_ptr(),
            0 as *mut libc::c_void as *mut libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_char,
            9 as libc::c_int as ushort_t,
        );
        count = lines_in_file(
            fd,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8210]>() as libc::c_ulong,
        );
        if count == 0 {
            current_block = 2993558768300439429;
        } else {
            current_block = 6057473163062296781;
        }
    } else {
        seltofile(fd, &mut count);
        current_block = 6057473163062296781;
    }
    match current_block {
        6057473163062296781 => {
            close(fd);
            snprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8210]>() as libc::c_ulong,
                patterns[0 as libc::c_int as usize],
                g_tmpfpath.as_mut_ptr(),
            );
            spawn(
                utils[7 as libc::c_int as usize],
                buf.as_mut_ptr(),
                0 as *mut libc::c_void as *mut libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_char,
                9 as libc::c_int as ushort_t,
            );
            if cfg.waitedit() != 0 {
                tmp___0 = enveditor;
            } else {
                tmp___0 = editor;
            }
            spawn(
                tmp___0,
                g_tmpfpath.as_mut_ptr(),
                0 as *mut libc::c_void as *mut libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_char,
                9 as libc::c_int as ushort_t,
            );
            fd = open(g_tmpfpath.as_mut_ptr() as *const libc::c_char, 0 as libc::c_int);
            if !(fd == -(1 as libc::c_int)) {
                lines = lines_in_file(
                    fd,
                    buf.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 8210]>() as libc::c_ulong,
                );
                if !(lines == 0) {
                    if !((2 as libc::c_uint).wrapping_mul(count) != lines) {
                        snprintf(
                            buf.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 8210]>()
                                as libc::c_ulong,
                            patterns[1 as libc::c_int as usize],
                            path,
                            g_tmpfpath.as_mut_ptr(),
                            cmd___0,
                        );
                        tmp___1 = spawn(
                            utils[7 as libc::c_int as usize],
                            buf.as_mut_ptr(),
                            0 as *mut libc::c_void as *mut libc::c_char,
                            0 as *mut libc::c_void as *mut libc::c_char,
                            41 as libc::c_int as ushort_t,
                        );
                        if tmp___1 == 0 {
                            ret = 1 as libc::c_int != 0;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if fd >= 0 as libc::c_int {
        close(fd);
    }
    return ret;
}
unsafe extern "C" fn cpmvrm_selection(
    mut sel: action,
    mut path: *mut libc::c_char,
) -> bool {
    let mut r: libc::c_int = 0;
    let mut tmp: bool = false;
    let mut tmp___0: bool = false;
    let mut tmp___1: bool = false;
    let mut tmp___2: bool = false;
    let mut tmp___3: libc::c_int = 0;
    tmp = isselfileempty();
    if tmp {
        if nselected != 0 {
            clearselection();
        }
        printmsg(messages[3 as libc::c_int as usize]);
        return 0 as libc::c_int != 0;
    }
    tmp___0 = selsafe();
    if !tmp___0 {
        return 0 as libc::c_int != 0;
    }
    match sel as libc::c_uint {
        42 => {
            opstr(g_buf.as_mut_ptr(), cp.as_mut_ptr());
        }
        43 => {
            opstr(g_buf.as_mut_ptr(), mv.as_mut_ptr());
        }
        44 => {
            r = get_input(messages[7 as libc::c_int as usize]);
            if r != 99 as libc::c_int {
                if r != 109 as libc::c_int {
                    printmsg(messages[40 as libc::c_int as usize]);
                    return 0 as libc::c_int != 0;
                }
            }
            tmp___1 = cpmv_rename(r, path as *const libc::c_char);
            if !tmp___1 {
                printmsg(messages[5 as libc::c_int as usize]);
                return 0 as libc::c_int != 0;
            }
        }
        _ => {
            tmp___2 = rmmulstr(g_buf.as_mut_ptr());
            if !tmp___2 {
                printmsg(messages[4 as libc::c_int as usize]);
                return 0 as libc::c_int != 0;
            }
        }
    }
    if sel as libc::c_uint != 44 as libc::c_uint {
        tmp___3 = spawn(
            utils[7 as libc::c_int as usize],
            g_buf.as_mut_ptr(),
            0 as *mut libc::c_void as *mut libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_char,
            41 as libc::c_int as ushort_t,
        );
        if tmp___3 != 0 {
            printmsg(messages[5 as libc::c_int as usize]);
            return 0 as libc::c_int != 0;
        }
    }
    clearselection();
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn get_archive_cmd(
    mut cmd___0: *mut libc::c_char,
    mut archive: *const libc::c_char,
) {
    let mut i: uchar_t = 0;
    let mut tmp: bool = false;
    let mut tmp___0: bool = false;
    let mut tmp___1: bool = false;
    i = 3 as libc::c_int as uchar_t;
    let mut current_block_12: u64;
    if g_state.usebsdtar() == 0 {
        tmp___1 = getutil(utils[1 as libc::c_int as usize]);
        if tmp___1 {
            i = 0 as libc::c_int as uchar_t;
            current_block_12 = 7976072742316086414;
        } else {
            current_block_12 = 4762133537728076040;
        }
    } else {
        current_block_12 = 4762133537728076040;
    }
    match current_block_12 {
        4762133537728076040 => {
            tmp___0 = getutil(utils[2 as libc::c_int as usize]);
            if tmp___0 {
                i = 1 as libc::c_int as uchar_t;
            } else {
                tmp = is_suffix(archive, b".zip\0" as *const u8 as *const libc::c_char);
                if tmp {
                    i = 2 as libc::c_int as uchar_t;
                }
            }
        }
        _ => {}
    }
    xstrsncpy(cmd___0, archive_cmd[i as usize], 16 as libc::c_int as size_t);
}
unsafe extern "C" fn archive_selection(
    mut cmd___0: *const libc::c_char,
    mut archive: *const libc::c_char,
    mut curpath: *const libc::c_char,
) {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: size_t = 0;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___5: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = xstrlen(patterns[4 as libc::c_int as usize]);
    tmp___0 = xstrlen(cmd___0);
    tmp___1 = xstrlen(archive);
    tmp___2 = xstrlen(curpath);
    tmp___3 = xstrlen(selpath as *const libc::c_char);
    tmp___4 = malloc(
        tmp
            .wrapping_add(tmp___0)
            .wrapping_add(tmp___1)
            .wrapping_add(tmp___2)
            .wrapping_add(tmp___3)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    buf = tmp___4 as *mut libc::c_char;
    if buf.is_null() {
        tmp___5 = __errno_location();
        tmp___6 = strerror(*tmp___5);
        printwait(
            tmp___6 as *const libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        return;
    }
    snprintf(
        buf,
        (4096 as libc::c_int + ((256 as libc::c_int) << 1 as libc::c_int)) as size_t,
        patterns[4 as libc::c_int as usize],
        curpath,
        selpath,
        cmd___0,
        archive,
    );
    spawn(
        utils[7 as libc::c_int as usize],
        buf,
        0 as *mut libc::c_void as *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
        25 as libc::c_int as ushort_t,
    );
    free(buf as *mut libc::c_void);
}
unsafe extern "C" fn write_lastdir(
    mut curpath: *const libc::c_char,
    mut outfile: *const libc::c_char,
) {
    let mut tmp: size_t = 0;
    let mut fd: libc::c_int = 0;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___2: libc::c_int = 0;
    if outfile.is_null() {
        tmp = xstrlen(cfgpath as *const libc::c_char);
        xstrsncpy(
            cfgpath.offset(tmp as isize),
            b"/.lastd\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as size_t,
        );
    } else {
        convert_tilde(outfile, g_buf.as_mut_ptr());
    }
    if !outfile.is_null() {
        if *outfile.offset(0 as libc::c_int as isize) as libc::c_int
            == 126 as libc::c_int
        {
            tmp___0 = g_buf.as_mut_ptr() as *const libc::c_char;
        } else {
            tmp___0 = outfile;
        }
        tmp___1 = tmp___0;
    } else {
        tmp___1 = cfgpath as *const libc::c_char;
    }
    tmp___2 = open(tmp___1, 577 as libc::c_int, 438 as libc::c_int);
    fd = tmp___2;
    if fd != -(1 as libc::c_int) {
        dprintf(fd, b"cd \"%s\"\0" as *const u8 as *const libc::c_char, curpath);
        close(fd);
    }
}
unsafe extern "C" fn xstricmp(
    s1: *const libc::c_char,
    s2: *const libc::c_char,
) -> libc::c_int {
    let mut p1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v1: libc::c_longlong = 0;
    let mut tmp: libc::c_longlong = 0;
    let mut v2: libc::c_longlong = 0;
    let mut tmp___0: libc::c_longlong = 0;
    let mut tmp___1: libc::c_int = 0;
    tmp = strtoll(s1, &mut p1 as *mut *mut libc::c_char, 10 as libc::c_int);
    v1 = tmp;
    tmp___0 = strtoll(s2, &mut p2 as *mut *mut libc::c_char, 10 as libc::c_int);
    v2 = tmp___0;
    let mut current_block_19: u64;
    if s1 as libc::c_ulong != p1 as libc::c_ulong {
        current_block_19 = 18108685279695006177;
    } else if s2 as libc::c_ulong != p2 as libc::c_ulong {
        current_block_19 = 18108685279695006177;
    } else {
        current_block_19 = 13797916685926291137;
    }
    match current_block_19 {
        18108685279695006177 => {
            if s1 as libc::c_ulong != p1 as libc::c_ulong {
                if s2 as libc::c_ulong != p2 as libc::c_ulong {
                    if v2 > v1 {
                        return -(1 as libc::c_int);
                    }
                    if v1 > v2 {
                        return 1 as libc::c_int;
                    }
                }
            }
            if s1 as libc::c_ulong == p1 as libc::c_ulong {
                return 1 as libc::c_int;
            }
            if s2 as libc::c_ulong == p2 as libc::c_ulong {
                return -(1 as libc::c_int);
            }
        }
        _ => {}
    }
    tmp___1 = strcasecmp(s1, s2);
    return tmp___1;
}
static mut next_state: [uint8_t; 12] = [
    0 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
];
static mut result_type: [int8_t; 36] = [
    2 as libc::c_int as int8_t,
    2 as libc::c_int as int8_t,
    2 as libc::c_int as int8_t,
    2 as libc::c_int as int8_t,
    3 as libc::c_int as int8_t,
    2 as libc::c_int as int8_t,
    2 as libc::c_int as int8_t,
    2 as libc::c_int as int8_t,
    2 as libc::c_int as int8_t,
    2 as libc::c_int as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    1 as libc::c_int as int8_t,
    3 as libc::c_int as int8_t,
    3 as libc::c_int as int8_t,
    1 as libc::c_int as int8_t,
    3 as libc::c_int as int8_t,
    3 as libc::c_int as int8_t,
    2 as libc::c_int as int8_t,
    2 as libc::c_int as int8_t,
    2 as libc::c_int as int8_t,
    2 as libc::c_int as int8_t,
    2 as libc::c_int as int8_t,
    2 as libc::c_int as int8_t,
    2 as libc::c_int as int8_t,
    2 as libc::c_int as int8_t,
    2 as libc::c_int as int8_t,
    2 as libc::c_int as int8_t,
    1 as libc::c_int as int8_t,
    1 as libc::c_int as int8_t,
    -(1 as libc::c_int) as int8_t,
    2 as libc::c_int as int8_t,
    2 as libc::c_int as int8_t,
    -(1 as libc::c_int) as int8_t,
    2 as libc::c_int as int8_t,
    2 as libc::c_int as int8_t,
];
unsafe extern "C" fn xstrverscasecmp(
    s1: *const libc::c_char,
    s2: *const libc::c_char,
) -> libc::c_int {
    let mut p1: *const uchar_t = 0 as *const uchar_t;
    let mut p2: *const uchar_t = 0 as *const uchar_t;
    let mut state: libc::c_int = 0;
    let mut diff: libc::c_int = 0;
    let mut c1: uchar_t = 0;
    let mut c2: uchar_t = 0;
    let mut tmp: *const uchar_t = 0 as *const uchar_t;
    let mut tmp___0: *const uchar_t = 0 as *const uchar_t;
    let mut tmp___1: libc::c_int = 0;
    p1 = s1 as *const uchar_t;
    p2 = s2 as *const uchar_t;
    if p1 as libc::c_ulong == p2 as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if *p1 as libc::c_int >= 97 as libc::c_int {
        if *p1 as libc::c_int <= 122 as libc::c_int {
            c1 = (*p1 as libc::c_int - 97 as libc::c_int + 65 as libc::c_int) as uchar_t;
        } else {
            c1 = *p1;
        }
    } else {
        c1 = *p1;
    }
    p1 = p1.offset(1);
    if *p2 as libc::c_int >= 97 as libc::c_int {
        if *p2 as libc::c_int <= 122 as libc::c_int {
            c2 = (*p2 as libc::c_int - 97 as libc::c_int + 65 as libc::c_int) as uchar_t;
        } else {
            c2 = *p2;
        }
    } else {
        c2 = *p2;
    }
    p2 = p2.offset(1);
    state = (c1 as libc::c_int == 48 as libc::c_int) as libc::c_int
        + (((c1 as libc::c_uint).wrapping_sub(48 as libc::c_uint) <= 9 as libc::c_uint)
            as libc::c_int != 0 as libc::c_int) as libc::c_int;
    loop {
        diff = c1 as libc::c_int - c2 as libc::c_int;
        if !(diff == 0 as libc::c_int) {
            break;
        }
        if c1 as libc::c_int == 0 as libc::c_int {
            return diff;
        }
        state = next_state[state as usize] as libc::c_int;
        if *p1 as libc::c_int >= 97 as libc::c_int {
            if *p1 as libc::c_int <= 122 as libc::c_int {
                c1 = (*p1 as libc::c_int - 97 as libc::c_int + 65 as libc::c_int)
                    as uchar_t;
            } else {
                c1 = *p1;
            }
        } else {
            c1 = *p1;
        }
        p1 = p1.offset(1);
        if *p2 as libc::c_int >= 97 as libc::c_int {
            if *p2 as libc::c_int <= 122 as libc::c_int {
                c2 = (*p2 as libc::c_int - 97 as libc::c_int + 65 as libc::c_int)
                    as uchar_t;
            } else {
                c2 = *p2;
            }
        } else {
            c2 = *p2;
        }
        p2 = p2.offset(1);
        state
            += (c1 as libc::c_int == 48 as libc::c_int) as libc::c_int
                + (((c1 as libc::c_uint).wrapping_sub(48 as libc::c_uint)
                    <= 9 as libc::c_uint) as libc::c_int != 0 as libc::c_int)
                    as libc::c_int;
    }
    state = result_type[(state * 3 as libc::c_int
        + ((c2 as libc::c_int == 48 as libc::c_int) as libc::c_int
            + (((c2 as libc::c_uint).wrapping_sub(48 as libc::c_uint)
                <= 9 as libc::c_uint) as libc::c_int != 0 as libc::c_int)
                as libc::c_int)) as usize] as libc::c_int;
    match state {
        2 => return diff,
        3 => {
            loop {
                tmp___0 = p1;
                p1 = p1.offset(1);
                if !((*tmp___0 as libc::c_uint).wrapping_sub(48 as libc::c_uint)
                    <= 9 as libc::c_uint)
                {
                    break;
                }
                tmp = p2;
                p2 = p2.offset(1);
                if !((*tmp as libc::c_uint).wrapping_sub(48 as libc::c_uint)
                    <= 9 as libc::c_uint)
                {
                    return 1 as libc::c_int;
                }
            }
            if (*p2 as libc::c_uint).wrapping_sub(48 as libc::c_uint)
                <= 9 as libc::c_uint
            {
                tmp___1 = -(1 as libc::c_int);
            } else {
                tmp___1 = diff;
            }
            return tmp___1;
        }
        _ => return state,
    };
}
static mut namecmpfn: Option::<
    unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
> = Some(
    xstricmp
        as unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
);
static mut fnstrstr: Option::<
    unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> *mut libc::c_char,
> = Some(
    strcasestr
        as unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
        ) -> *mut libc::c_char,
);
static mut regflags: libc::c_int = (1 as libc::c_int) << 3 as libc::c_int
    | 1 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int;
unsafe extern "C" fn setfilter(
    mut regex: *mut regex_t,
    mut filter___0: *const libc::c_char,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = regcomp(regex, filter___0, regflags);
    return tmp;
}
unsafe extern "C" fn visible_re(
    mut fltrexp: *const fltrexp_t,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = regexec(
        (*fltrexp).regex,
        fname,
        0 as libc::c_int as size_t,
        0 as *mut libc::c_void as *mut regmatch_t,
        0 as libc::c_int,
    );
    return (tmp == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn visible_str(
    mut fltrexp: *const fltrexp_t,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = (Some(fnstrstr.expect("non-null function pointer")))
        .expect("non-null function pointer")(fname, (*fltrexp).str_0);
    return (tmp as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong)
        as libc::c_int;
}
static mut filterfn: Option::<
    unsafe extern "C" fn(*const fltrexp_t, *const libc::c_char) -> libc::c_int,
> = Some(
    visible_str
        as unsafe extern "C" fn(*const fltrexp_t, *const libc::c_char) -> libc::c_int,
);
unsafe extern "C" fn clearfilter() {
    let mut fltr: *mut libc::c_char = 0 as *mut libc::c_char;
    fltr = (g_ctx[cfg.curctx() as usize].c_fltr).as_mut_ptr();
    if *fltr.offset(1 as libc::c_int as isize) != 0 {
        *fltr
            .offset(
                47 as libc::c_int as isize,
            ) = *fltr.offset(1 as libc::c_int as isize);
        *fltr.offset(1 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    }
}
unsafe extern "C" fn entrycmp(
    mut va: *const libc::c_void,
    mut vb: *const libc::c_void,
) -> libc::c_int {
    let mut pa: *const entry = 0 as *const entry;
    let mut pb: *const entry = 0 as *const entry;
    let mut extna: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut extnb: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    pa = va as pEntry as *const entry;
    pb = vb as pEntry as *const entry;
    if ((*pb).__annonCompField18).flags() & 1 as libc::c_ulonglong
        != ((*pa).__annonCompField18).flags() & 1 as libc::c_ulonglong
    {
        if ((*pb).__annonCompField18).flags() & 1 as libc::c_ulonglong != 0 {
            return 1 as libc::c_int;
        }
        return -(1 as libc::c_int);
    }
    if cfg.timeorder() != 0 {
        if (*pb).sec > (*pa).sec {
            return 1 as libc::c_int;
        }
        if (*pb).sec < (*pa).sec {
            return -(1 as libc::c_int);
        }
        if (*pb).nsec > (*pa).nsec {
            return 1 as libc::c_int;
        }
        if (*pb).nsec < (*pa).nsec {
            return -(1 as libc::c_int);
        }
    } else if cfg.sizeorder() != 0 {
        if (*pb).size > (*pa).size {
            return 1 as libc::c_int;
        }
        if (*pb).size < (*pa).size {
            return -(1 as libc::c_int);
        }
    } else if cfg.blkorder() != 0 {
        if ((*pb).__annonCompField18).blocks() > ((*pa).__annonCompField18).blocks() {
            return 1 as libc::c_int;
        }
        if ((*pb).__annonCompField18).blocks() < ((*pa).__annonCompField18).blocks() {
            return -(1 as libc::c_int);
        }
    } else if cfg.extnorder() != 0 {
        if ((*pb).__annonCompField18).flags() & 1 as libc::c_ulonglong == 0 {
            tmp = xextension(
                (*pa).name as *const libc::c_char,
                ((*pa).__annonCompField18).nlen().wrapping_sub(1 as libc::c_ulonglong)
                    as size_t,
            );
            extna = tmp;
            tmp___0 = xextension(
                (*pb).name as *const libc::c_char,
                ((*pb).__annonCompField18).nlen().wrapping_sub(1 as libc::c_ulonglong)
                    as size_t,
            );
            extnb = tmp___0;
            let mut current_block_49: u64;
            if !extna.is_null() {
                current_block_49 = 4678235918457740128;
            } else if !extnb.is_null() {
                current_block_49 = 4678235918457740128;
            } else {
                current_block_49 = 2480299350034459858;
            }
            match current_block_49 {
                4678235918457740128 => {
                    if extna.is_null() {
                        return -(1 as libc::c_int);
                    }
                    if extnb.is_null() {
                        return 1 as libc::c_int;
                    }
                    tmp___1 = strcasecmp(
                        extna as *const libc::c_char,
                        extnb as *const libc::c_char,
                    );
                    ret = tmp___1;
                    if ret != 0 {
                        return ret;
                    }
                }
                _ => {}
            }
        }
    }
    tmp___2 = (Some(namecmpfn.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((*pa).name as *const libc::c_char, (*pb).name as *const libc::c_char);
    return tmp___2;
}
unsafe extern "C" fn reventrycmp(
    mut va: *const libc::c_void,
    mut vb: *const libc::c_void,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    if ((*(vb as pEntry)).__annonCompField18).flags() & 1 as libc::c_ulonglong
        != ((*(va as pEntry)).__annonCompField18).flags() & 1 as libc::c_ulonglong
    {
        if ((*(vb as pEntry)).__annonCompField18).flags() & 1 as libc::c_ulonglong != 0 {
            return 1 as libc::c_int;
        }
        return -(1 as libc::c_int);
    }
    tmp = entrycmp(va, vb);
    return -tmp;
}
static mut entrycmpfn: Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
> = Some(
    entrycmp
        as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
);
unsafe extern "C" fn handle_alt_key(mut wch: *mut wint_t) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    wtimeout(stdscr, 0 as libc::c_int);
    tmp = wget_wch(stdscr, wch);
    r = tmp;
    if r == -(1 as libc::c_int) {
        *wch = 27 as libc::c_int as wint_t;
    }
    wtimeout(stdscr, -(1 as libc::c_int));
    return r;
}
#[inline]
unsafe extern "C" fn handle_event() -> libc::c_int {
    let mut tmp: bool = false;
    if nselected != 0 {
        tmp = isselfileempty();
        if tmp {
            clearselection();
        }
    }
    return 12 as libc::c_int;
}
unsafe extern "C" fn nextsel(mut presel: libc::c_int) -> libc::c_int {
    let mut current_block: u64;
    let mut c: wint_t = 0;
    let mut i: libc::c_int = 0;
    let mut escaped: bool = false;
    let mut event: *mut inotify_event = 0 as *mut inotify_event;
    let mut inotify_buf: [libc::c_char; 512] = [0; 512];
    let mut tmp: libc::c_uint = 0;
    let mut tmp___0: ssize_t = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    c = presel as wint_t;
    i = 0 as libc::c_int;
    escaped = 0 as libc::c_int != 0;
    if c == 0 as libc::c_uint {
        current_block = 18027489711510726807;
    } else if c == 36 as libc::c_uint {
        current_block = 18027489711510726807;
    } else {
        current_block = 3222590281903869779;
    }
    match current_block {
        18027489711510726807 => {
            loop {
                i = wget_wch(stdscr, &mut c);
                if c == 410 as libc::c_uint {
                    handle_key_resize();
                }
                if !(c == 27 as libc::c_uint) {
                    break;
                }
                wtimeout(stdscr, 0 as libc::c_int);
                i = wget_wch(stdscr, &mut c);
                if i != -(1 as libc::c_int) {
                    if c == 27 as libc::c_uint {
                        c = 12 as libc::c_int as wint_t;
                    } else {
                        unget_wch(c as wchar_t);
                        c = ';' as i32 as wint_t;
                    }
                    wtimeout(stdscr, 1000 as libc::c_int);
                    break;
                } else if escaped {
                    wtimeout(stdscr, 1000 as libc::c_int);
                    c = 17 as libc::c_int as wint_t;
                    break;
                } else {
                    escaped = 1 as libc::c_int != 0;
                    wtimeout(stdscr, 1000 as libc::c_int);
                }
            }
            let mut current_block_36: u64;
            if i == -(1 as libc::c_int) {
                if presel == 36 as libc::c_int {
                    if cfg.filtermode() != 0 {
                        c = '/' as i32 as wint_t;
                    } else if g_ctx[cfg.curctx() as usize]
                            .c_fltr[1 as libc::c_int as usize] != 0
                        {
                        c = '/' as i32 as wint_t;
                    } else {
                        c = 12 as libc::c_int as wint_t;
                    }
                    current_block_36 = 7427571413727699167;
                } else {
                    current_block_36 = 10122836698091527856;
                }
            } else {
                current_block_36 = 10122836698091527856;
            }
            match current_block_36 {
                10122836698091527856 => {
                    if c == 47 as libc::c_uint {
                        clearfilter();
                    } else if c == 12 as libc::c_uint {
                        clearfilter();
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
    if i == -(1 as libc::c_int) {
        idle = (idle as libc::c_int + 1 as libc::c_int) as ushort_t;
        if cfg.blkorder() == 0 {
            if inotify_wd >= 0 as libc::c_int {
                if idle as libc::c_int & 1 as libc::c_int != 0 {
                    inotify_buf[0 as libc::c_int
                        as usize] = 0 as libc::c_int as libc::c_char;
                    tmp = 1 as libc::c_uint;
                    while !(tmp >= 512 as libc::c_uint) {
                        inotify_buf[tmp as usize] = 0 as libc::c_int as libc::c_char;
                        tmp = tmp.wrapping_add(1);
                    }
                    tmp___0 = read(
                        inotify_fd,
                        inotify_buf.as_mut_ptr() as *mut libc::c_void,
                        (::std::mem::size_of::<inotify_event>() as libc::c_ulong)
                            .wrapping_mul(32 as libc::c_ulong),
                    );
                    i = tmp___0 as libc::c_int;
                    if i > 0 as libc::c_int {
                        ptr = inotify_buf.as_mut_ptr();
                        while (ptr.offset((*(ptr as *mut inotify_event)).len as isize)
                            as libc::c_ulong)
                            < inotify_buf.as_mut_ptr().offset(i as isize)
                                as libc::c_ulong
                        {
                            event = ptr as *mut inotify_event;
                            if (*event).wd == 0 {
                                break;
                            }
                            if (*event).mask & INOTIFY_MASK != 0 {
                                tmp___1 = handle_event();
                                c = tmp___1 as wint_t;
                                break;
                            } else {
                                ptr = ptr
                                    .offset(
                                        (::std::mem::size_of::<inotify_event>() as libc::c_ulong)
                                            .wrapping_add((*event).len as libc::c_ulong) as isize,
                                    );
                            }
                        }
                    }
                }
            }
        }
    } else {
        idle = 0 as libc::c_int as ushort_t;
    }
    i = 0 as libc::c_int;
    while i
        < (::std::mem::size_of::<[key; 84]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<key>() as libc::c_ulong) as libc::c_int
    {
        if c == bindings[i as usize].sym {
            return bindings[i as usize].act as libc::c_int;
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn getorderstr(mut sort: *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    i = 0 as libc::c_int;
    if cfg.showhidden() != 0 {
        tmp = i;
        i += 1;
        *sort.offset(tmp as isize) = 'H' as i32 as libc::c_char;
    }
    if cfg.timeorder() != 0 {
        tmp___0 = i;
        i += 1;
        if cfg.timetype() == 2 as libc::c_uint {
            *sort.offset(tmp___0 as isize) = 'M' as i32 as libc::c_char;
        } else {
            if cfg.timetype() == 0 as libc::c_uint {
                tmp___1 = 'A' as i32;
            } else {
                tmp___1 = 'C' as i32;
            }
            *sort.offset(tmp___0 as isize) = tmp___1 as libc::c_char;
        }
    } else if cfg.sizeorder() != 0 {
        tmp___2 = i;
        i += 1;
        *sort.offset(tmp___2 as isize) = 'S' as i32 as libc::c_char;
    } else if cfg.extnorder() != 0 {
        tmp___3 = i;
        i += 1;
        *sort.offset(tmp___3 as isize) = 'E' as i32 as libc::c_char;
    }
    if ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        >,
        libc::c_ulong,
    >(entrycmpfn)
        == ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                reventrycmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        )
    {
        tmp___4 = i;
        i += 1;
        *sort.offset(tmp___4 as isize) = 'R' as i32 as libc::c_char;
    }
    if ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
        >,
        libc::c_ulong,
    >(namecmpfn)
        == ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                xstrverscasecmp
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *const libc::c_char,
                    ) -> libc::c_int,
            ),
        )
    {
        tmp___5 = i;
        i += 1;
        *sort.offset(tmp___5 as isize) = 'V' as i32 as libc::c_char;
    }
    if i != 0 {
        *sort.offset(i as isize) = ' ' as i32 as libc::c_char;
    }
    return i;
}
unsafe extern "C" fn showfilterinfo() {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut info: [libc::c_char; 48] = [0; 48];
    let mut tmp: libc::c_uint = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___2: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___3: bool = false;
    let mut tmp___4: size_t = 0;
    let mut tmp___5: libc::c_int = 0;
    i = 0 as libc::c_int;
    info[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    info[1 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    info[2 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    info[3 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    info[4 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    info[5 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    tmp = 6 as libc::c_uint;
    while !(tmp >= 48 as libc::c_uint) {
        info[tmp as usize] = 0 as libc::c_int as libc::c_char;
        tmp = tmp.wrapping_add(1);
    }
    i = getorderstr(info.as_mut_ptr());
    if cfg.fileinfo() != 0 {
        if ndents != 0 {
            tmp___3 = get_output(
                b"file\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"-b\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*pdents.offset(cur as isize)).name,
                -(1 as libc::c_int),
                0 as libc::c_int != 0,
                0 as libc::c_int != 0,
            );
            if tmp___3 {
                tmp___0 = wmove(
                    stdscr,
                    xlines as libc::c_int - 2 as libc::c_int,
                    2 as libc::c_int,
                );
                if !(tmp___0 == -(1 as libc::c_int)) {
                    waddnstr(
                        stdscr,
                        g_buf.as_mut_ptr() as *const libc::c_char,
                        -(1 as libc::c_int),
                    );
                }
                current_block = 15897653523371991391;
            } else {
                current_block = 18377454917952818918;
            }
        } else {
            current_block = 18377454917952818918;
        }
    } else {
        current_block = 18377454917952818918;
    }
    match current_block {
        18377454917952818918 => {
            if ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const libc::c_char,
                        *const libc::c_char,
                    ) -> *mut libc::c_char,
                >,
                libc::c_ulong,
            >(fnstrstr)
                == ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                        ) -> *mut libc::c_char,
                    >,
                    libc::c_ulong,
                >(
                    Some(
                        strcasestr
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                )
            {
                tmp___1 = b"ic\0" as *const u8 as *const libc::c_char;
            } else {
                tmp___1 = b"noic\0" as *const u8 as *const libc::c_char;
            }
            if cfg.regex() != 0 {
                tmp___2 = b"reg\0" as *const u8 as *const libc::c_char;
            } else {
                tmp___2 = b"str\0" as *const u8 as *const libc::c_char;
            }
            snprintf(
                info.as_mut_ptr().offset(i as isize),
                (48 as libc::c_int - i - 1 as libc::c_int) as size_t,
                b"  %s [/], %4s [:]\0" as *const u8 as *const libc::c_char,
                tmp___2,
                tmp___1,
            );
        }
        _ => {}
    }
    tmp___4 = xstrlen(info.as_mut_ptr() as *const libc::c_char);
    tmp___5 = wmove(
        stdscr,
        xlines as libc::c_int - 2 as libc::c_int,
        (xcols as size_t).wrapping_sub(tmp___4) as libc::c_int,
    );
    if !(tmp___5 == -(1 as libc::c_int)) {
        waddnstr(stdscr, info.as_mut_ptr() as *const libc::c_char, -(1 as libc::c_int));
    }
}
unsafe extern "C" fn showfilter(mut str: *mut libc::c_char) {
    wattr_on(
        stdscr,
        cfg.curctx().wrapping_add(1 as libc::c_uint) << 8 as libc::c_int
            & ((1 as libc::c_uint) << 8 as libc::c_int).wrapping_sub(1 as libc::c_uint)
                << 8 as libc::c_int,
        0 as *mut libc::c_void,
    );
    showfilterinfo();
    printmsg(str as *const libc::c_char);
}
#[inline]
unsafe extern "C" fn swap_ent(mut id1: libc::c_int, mut id2: libc::c_int) {
    let mut _dent: entry = entry {
        name: 0 as *mut libc::c_char,
        sec: 0,
        nsec: 0,
        mode: 0,
        size: 0,
        __annonCompField18: __anonstruct____missing_field_name_824606445 {
            blocks_nlen_flags: [0; 8],
        },
    };
    let mut pdent1: *mut entry = 0 as *mut entry;
    let mut pdent2: *mut entry = 0 as *mut entry;
    pdent1 = pdents.offset(id1 as isize);
    pdent2 = pdents.offset(id2 as isize);
    _dent = *pdent1;
    *pdent1 = *pdent2;
    *pdent2 = _dent;
}
unsafe extern "C" fn fill(
    mut fltr: *const libc::c_char,
    mut re: *mut regex_t,
) -> libc::c_int {
    let mut fltrexp: fltrexp_t = fltrexp_t {
        regex: 0 as *const regex_t,
        str_0: 0 as *const libc::c_char,
    };
    let mut count: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    fltrexp.regex = re as *const regex_t;
    fltrexp.str_0 = fltr;
    count = 0 as libc::c_int;
    while count < ndents {
        tmp = (Some(filterfn.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            &mut fltrexp as *mut fltrexp_t as *const fltrexp_t,
            (*pdents.offset(count as isize)).name as *const libc::c_char,
        );
        if tmp == 0 as libc::c_int {
            ndents -= 1;
            if count != ndents {
                swap_ent(count, ndents);
                count -= 1;
            }
        }
        count += 1;
    }
    return ndents;
}
unsafe extern "C" fn matches(mut fltr: *const libc::c_char) -> libc::c_int {
    let mut re: regex_t = regex_t {
        buffer: 0 as *const re_dfa_t as *mut re_dfa_t,
        allocated: 0,
        used: 0,
        syntax: 0,
        fastmap: 0 as *const libc::c_char as *mut libc::c_char,
        translate: 0 as *const libc::c_uchar as *mut libc::c_uchar,
        re_nsub: 0,
        can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut tmp: libc::c_int = 0;
    if cfg.regex() != 0 {
        tmp = setfilter(&mut re, fltr);
        if tmp != 0 {
            return -(1 as libc::c_int);
        }
    }
    ndents = fill(fltr, &mut re);
    if cfg.regex() != 0 {
        regfree(&mut re);
    }
    qsort(
        pdents as *mut libc::c_void,
        ndents as size_t,
        ::std::mem::size_of::<entry>() as libc::c_ulong,
        entrycmpfn,
    );
    return ndents;
}
unsafe extern "C" fn dentfind(
    mut fname: *const libc::c_char,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        if *fname as libc::c_int != *(*pdents.offset(i as isize)).name as libc::c_int {
            tmp___0 = -(1 as libc::c_int);
        } else {
            tmp = strcmp(
                fname,
                (*pdents.offset(i as isize)).name as *const libc::c_char,
            );
            tmp___0 = tmp;
        }
        if tmp___0 == 0 as libc::c_int {
            return i;
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn filterentries(
    mut path: *mut libc::c_char,
    mut lastname: *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut wln: *mut wchar_t = 0 as *mut wchar_t;
    let mut tmp = 0 as *mut _;
    let mut ln: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ch: [wint_t; 1] = [0; 1];
    let mut r: libc::c_int = 0;
    let mut total: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut pln: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: wchar_t = 0;
    let mut tmp___4: wchar_t = 0;
    let mut tmp___5: wchar_t = 0;
    let mut tmp___6: size_t = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: *const libc::c_char = 0 as *const libc::c_char;
    let mut fresh9 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<wchar_t>() as libc::c_ulong)
            .wrapping_mul(48 as libc::c_ulong) as usize,
    );
    tmp = fresh9.as_mut_ptr();
    wln = tmp as *mut wchar_t;
    ln = (g_ctx[cfg.curctx() as usize].c_fltr).as_mut_ptr();
    total = ndents;
    pln = (g_ctx[cfg.curctx() as usize].c_fltr)
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize);
    if ndents != 0 {
        if *ln.offset(0 as libc::c_int as isize) as libc::c_int == 47 as libc::c_int {
            current_block = 1166636033579579057;
        } else if *ln.offset(0 as libc::c_int as isize) as libc::c_int
                == 92 as libc::c_int
            {
            current_block = 1166636033579579057;
        } else {
            current_block = 12052177935736200261;
        }
        match current_block {
            12052177935736200261 => {}
            _ => {
                if *pln != 0 {
                    tmp___1 = matches(pln as *const libc::c_char);
                    if tmp___1 != -(1 as libc::c_int) {
                        tmp___0 = dentfind(lastname as *const libc::c_char, ndents);
                        move_cursor(tmp___0, 0 as libc::c_int);
                        redraw(path);
                    }
                    if cfg.filtermode() == 0 {
                        statusbar(path);
                        return 0 as libc::c_int;
                    }
                    tmp___2 = mbstowcs(
                        wln,
                        ln as *const libc::c_char,
                        48 as libc::c_int as size_t,
                    );
                    len = tmp___2 as libc::c_int;
                    current_block = 17184638872671510253;
                } else {
                    current_block = 12052177935736200261;
                }
            }
        }
    } else {
        current_block = 12052177935736200261;
    }
    match current_block {
        12052177935736200261 => {
            if cfg.regex() != 0 {
                tmp___3 = '\\' as i32;
            } else {
                tmp___3 = '/' as i32;
            }
            *wln.offset(0 as libc::c_int as isize) = tmp___3;
            *ln.offset(0 as libc::c_int as isize) = tmp___3 as libc::c_char;
            tmp___4 = '\u{0}' as i32;
            *wln.offset(1 as libc::c_int as isize) = tmp___4;
            *ln.offset(1 as libc::c_int as isize) = tmp___4 as libc::c_char;
            len = 1 as libc::c_int;
        }
        _ => {}
    }
    wtimeout(stdscr, -(1 as libc::c_int));
    curs_set(1 as libc::c_int);
    showfilter(ln);
    loop {
        r = wget_wch(stdscr, ch.as_mut_ptr());
        if !(r != -(1 as libc::c_int)) {
            break;
        }
        match ch[0 as libc::c_int as usize] {
            410 | 0 => {
                clearoldprompt();
                redraw(path);
                showfilter(ln);
                continue;
            }
            127 | 8 | 263 | 330 => {
                if len != 1 as libc::c_int {
                    len -= 1;
                    *wln.offset(len as isize) = '\u{0}' as i32;
                    wcstombs(ln, wln as *const wchar_t, 48 as libc::c_int as size_t);
                    ndents = total;
                } else {
                    ch[0 as libc::c_int as usize] = '/' as i32 as wint_t;
                    break;
                }
            }
            12 => {}
            27 => {
                tmp___8 = handle_alt_key(ch.as_mut_ptr());
                if tmp___8 != -(1 as libc::c_int) {
                    if ch[0 as libc::c_int as usize] == 27 as libc::c_uint {
                        ch[0 as libc::c_int as usize] = 'q' as i32 as wint_t;
                    } else {
                        unget_wch(ch[0 as libc::c_int as usize] as wchar_t);
                        ch[0 as libc::c_int as usize] = ';' as i32 as wint_t;
                    }
                }
                break;
            }
            _ => {
                if r != 0 as libc::c_int {
                    break;
                }
                if ch[0 as libc::c_int as usize] < 128 as libc::c_uint {
                    tmp___9 = keyname(ch[0 as libc::c_int as usize] as libc::c_int);
                    if *tmp___9.offset(0 as libc::c_int as isize) as libc::c_int
                        == 94 as libc::c_int
                    {
                        if ch[0 as libc::c_int as usize] != 94 as libc::c_uint {
                            break;
                        }
                    }
                }
                if len == 1 as libc::c_int {
                    if ch[0 as libc::c_int as usize] == 63 as libc::c_uint {
                        break;
                    }
                    if cfg.filtermode() != 0 {
                        match ch[0 as libc::c_int as usize] {
                            126 | 96 | 93 | 64 | 62 | 61 | 59 | 46 | 45 | 44 | 43
                            | 39 => {
                                break;
                            }
                            _ => {}
                        }
                    }
                    if ch[0 as libc::c_int as usize] == 58 as libc::c_uint {
                        if ::std::mem::transmute::<
                            Option::<
                                unsafe extern "C" fn(
                                    *const libc::c_char,
                                    *const libc::c_char,
                                ) -> *mut libc::c_char,
                            >,
                            libc::c_ulong,
                        >(fnstrstr)
                            == ::std::mem::transmute::<
                                Option::<
                                    unsafe extern "C" fn(
                                        *const libc::c_char,
                                        *const libc::c_char,
                                    ) -> *mut libc::c_char,
                                >,
                                libc::c_ulong,
                            >(
                                Some(
                                    strcasestr
                                        as unsafe extern "C" fn(
                                            *const libc::c_char,
                                            *const libc::c_char,
                                        ) -> *mut libc::c_char,
                                ),
                            )
                        {
                            fnstrstr = ::std::mem::transmute::<
                                Option::<
                                    unsafe extern "C" fn(
                                        *const libc::c_char,
                                        *const libc::c_char,
                                    ) -> *mut libc::c_char,
                                >,
                                Option::<
                                    unsafe extern "C" fn(
                                        *const libc::c_char,
                                        *const libc::c_char,
                                    ) -> *mut libc::c_char,
                                >,
                            >(
                                Some(
                                    strstr
                                        as unsafe extern "C" fn(
                                            *const libc::c_char,
                                            *const libc::c_char,
                                        ) -> *mut libc::c_char,
                                ),
                            );
                        } else {
                            fnstrstr = Some(
                                strcasestr
                                    as unsafe extern "C" fn(
                                        *const libc::c_char,
                                        *const libc::c_char,
                                    ) -> *mut libc::c_char,
                            );
                        }
                        regflags ^= (1 as libc::c_int) << 1 as libc::c_int;
                        showfilter(ln);
                        continue;
                    } else if ch[0 as libc::c_int as usize] == 47 as libc::c_uint {
                        if *ln.offset(0 as libc::c_int as isize) as libc::c_int
                            == 47 as libc::c_int
                        {
                            *ln
                                .offset(
                                    0 as libc::c_int as isize,
                                ) = '\\' as i32 as libc::c_char;
                        } else {
                            *ln
                                .offset(
                                    0 as libc::c_int as isize,
                                ) = '/' as i32 as libc::c_char;
                        }
                        *wln
                            .offset(
                                0 as libc::c_int as isize,
                            ) = *ln.offset(0 as libc::c_int as isize) as uchar_t
                            as wchar_t;
                        cfg.set_regex(cfg.regex() ^ 1 as libc::c_uint as uint_t);
                        if cfg.regex() != 0 {
                            filterfn = Some(
                                visible_re
                                    as unsafe extern "C" fn(
                                        *const fltrexp_t,
                                        *const libc::c_char,
                                    ) -> libc::c_int,
                            );
                        } else {
                            filterfn = Some(
                                visible_str
                                    as unsafe extern "C" fn(
                                        *const fltrexp_t,
                                        *const libc::c_char,
                                    ) -> libc::c_int,
                            );
                        }
                        showfilter(ln);
                        continue;
                    } else {
                        cur = 0 as libc::c_int;
                    }
                } else if len == 47 as libc::c_int {
                    continue;
                }
                *wln.offset(len as isize) = ch[0 as libc::c_int as usize] as wchar_t;
                len += 1;
                *wln.offset(len as isize) = '\u{0}' as i32;
                wcstombs(ln, wln as *const wchar_t, 48 as libc::c_int as size_t);
                tmp___10 = matches(pln as *const libc::c_char);
                if tmp___10 == -(1 as libc::c_int) {
                    showfilter(ln);
                    continue;
                } else {
                    if ndents == 1 as libc::c_int {
                        if cfg.autoenter() != 0 {
                            if ((*pdents.offset(0 as libc::c_int as isize))
                                .__annonCompField18)
                                .flags() & 1 as libc::c_ulonglong != 0
                            {
                                ch[0 as libc::c_int
                                    as usize] = 343 as libc::c_int as wint_t;
                                cur = 0 as libc::c_int;
                                break;
                            }
                        }
                    }
                    redraw(path);
                    showfilter(ln);
                    continue;
                }
            }
        }
        if ch[0 as libc::c_int as usize] == 12 as libc::c_uint {
            if *wln.offset(1 as libc::c_int as isize) != 0 {
                *ln
                    .offset(
                        47 as libc::c_int as isize,
                    ) = *ln.offset(1 as libc::c_int as isize);
                tmp___5 = '\u{0}' as i32;
                *wln.offset(1 as libc::c_int as isize) = tmp___5;
                *ln.offset(1 as libc::c_int as isize) = tmp___5 as libc::c_char;
                len = 1 as libc::c_int;
                ndents = total;
            } else {
                if !(*ln.offset(47 as libc::c_int as isize) != 0) {
                    break;
                }
                *ln
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *ln.offset(47 as libc::c_int as isize);
                *ln.offset(47 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
                tmp___6 = mbstowcs(
                    wln,
                    ln as *const libc::c_char,
                    48 as libc::c_int as size_t,
                );
                len = tmp___6 as libc::c_int;
            }
        }
        cur = 0 as libc::c_int;
        tmp___7 = matches(pln as *const libc::c_char);
        if tmp___7 != -(1 as libc::c_int) {
            redraw(path);
        }
        showfilter(ln);
    }
    if *ln.offset(1 as libc::c_int as isize) != 0 {
        *ln.offset(47 as libc::c_int as isize) = *ln.offset(1 as libc::c_int as isize);
    }
    if ndents != 0 {
        tmp___11 = (*pdents.offset(cur as isize)).name as *const libc::c_char;
    } else {
        tmp___11 = b"\0\0" as *const u8 as *const libc::c_char;
    }
    xstrsncpy(lastname, tmp___11, 256 as libc::c_int as size_t);
    curs_set(0 as libc::c_int);
    wtimeout(stdscr, 1000 as libc::c_int);
    return ch[0 as libc::c_int as usize] as libc::c_int;
}
unsafe extern "C" fn xreadline(
    mut prefill: *const libc::c_char,
    mut prompt: *const libc::c_char,
) -> *mut libc::c_char {
    let mut len: size_t = 0;
    let mut pos: size_t = 0;
    let mut x: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut WCHAR_T_WIDTH: libc::c_int = 0;
    let mut ch: [wint_t; 1] = [0; 1];
    let mut buf: *mut wchar_t = 0 as *mut wchar_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    WCHAR_T_WIDTH = ::std::mem::size_of::<wchar_t>() as libc::c_ulong as libc::c_int;
    tmp = malloc(
        (::std::mem::size_of::<wchar_t>() as libc::c_ulong)
            .wrapping_mul(256 as libc::c_ulong),
    );
    buf = tmp as *mut wchar_t;
    if buf.is_null() {
        printerr(3442 as libc::c_int);
    }
    wtimeout(stdscr, -(1 as libc::c_int));
    printmsg(prompt);
    if !prefill.is_null() {
        pos = mbstowcs(buf, prefill, 256 as libc::c_int as size_t);
        len = pos;
    } else {
        len = -(1 as libc::c_int) as size_t;
    }
    if len == 0xffffffffffffffff as libc::c_ulong {
        *buf.offset(0 as libc::c_int as isize) = '\u{0}' as i32;
        pos = 0 as libc::c_int as size_t;
        len = pos;
    }
    if 0 as *const libc::c_void as libc::c_ulong
        != stdscr as *const libc::c_void as libc::c_ulong
    {
        x = (*stdscr)._curx as libc::c_int;
    } else {
        x = -(1 as libc::c_int);
    }
    curs_set(1 as libc::c_int);
    loop {
        *buf.offset(len as isize) = ' ' as i32;
        wattr_on(
            stdscr,
            cfg.curctx().wrapping_add(1 as libc::c_uint) << 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint) << 8 as libc::c_int,
            0 as *mut libc::c_void,
        );
        if pos > (xcols as libc::c_int - x) as size_t {
            tmp___0 = wmove(stdscr, xlines as libc::c_int - 1 as libc::c_int, x);
            if !(tmp___0 == -(1 as libc::c_int)) {
                waddnwstr(
                    stdscr,
                    buf
                        .offset(
                            pos
                                .wrapping_sub((xcols as libc::c_int - x) as size_t)
                                .wrapping_add(1 as libc::c_ulong) as isize,
                        ) as *const wchar_t,
                    xcols as libc::c_int - x,
                );
            }
            wmove(
                stdscr,
                xlines as libc::c_int - 1 as libc::c_int,
                xcols as libc::c_int - 1 as libc::c_int,
            );
        } else {
            tmp___1 = wmove(stdscr, xlines as libc::c_int - 1 as libc::c_int, x);
            if !(tmp___1 == -(1 as libc::c_int)) {
                waddnwstr(
                    stdscr,
                    buf as *const wchar_t,
                    len.wrapping_add(1 as libc::c_ulong) as libc::c_int,
                );
            }
            tmp___2 = wcswidth(buf as *const wchar_t, pos);
            wmove(stdscr, xlines as libc::c_int - 1 as libc::c_int, x + tmp___2);
        }
        wattr_off(
            stdscr,
            cfg.curctx().wrapping_add(1 as libc::c_uint) << 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint) << 8 as libc::c_int,
            0 as *mut libc::c_void,
        );
        r = wget_wch(stdscr, ch.as_mut_ptr());
        if r == -(1 as libc::c_int) {
            continue;
        }
        if r == 0 as libc::c_int {
            match ch[0 as libc::c_int as usize] {
                13 | 10 | 343 => {
                    break;
                }
                4 => {
                    if pos < len {
                        pos = pos.wrapping_add(1);
                    } else {
                        if pos != 0 {
                            continue;
                        }
                        if len != 0 {
                            continue;
                        }
                        len = 0 as libc::c_int as size_t;
                        break;
                    }
                }
                8 | 127 => {}
                9 => {
                    if len == 0 {
                        if pos == 0 {
                            if ndents != 0 {
                                pos = mbstowcs(
                                    buf,
                                    (*pdents.offset(cur as isize)).name as *const libc::c_char,
                                    256 as libc::c_int as size_t,
                                );
                                len = pos;
                            }
                        }
                    }
                    continue;
                }
                6 => {
                    if pos < len {
                        pos = pos.wrapping_add(1);
                    }
                    continue;
                }
                2 => {
                    if pos > 0 as libc::c_ulong {
                        pos = pos.wrapping_sub(1);
                    }
                    continue;
                }
                23 => {
                    printmsg(prompt);
                    while !(pos == 0 as libc::c_ulong) {
                        memmove(
                            buf.offset(pos as isize).offset(-(1 as libc::c_int as isize))
                                as *mut libc::c_void,
                            buf.offset(pos as isize) as *const libc::c_void,
                            len.wrapping_sub(pos).wrapping_mul(WCHAR_T_WIDTH as size_t),
                        );
                        pos = pos.wrapping_sub(1);
                        len = len.wrapping_sub(1);
                        if !(*buf.offset(pos.wrapping_sub(1 as libc::c_ulong) as isize)
                            != 32 as libc::c_int)
                        {
                            break;
                        }
                        if !(*buf.offset(pos.wrapping_sub(1 as libc::c_ulong) as isize)
                            != 47 as libc::c_int)
                        {
                            break;
                        }
                    }
                    continue;
                }
                11 => {
                    printmsg(prompt);
                    len = pos;
                    continue;
                }
                12 => {
                    printmsg(prompt);
                    pos = 0 as libc::c_int as size_t;
                    len = pos;
                    continue;
                }
                1 => {
                    pos = 0 as libc::c_int as size_t;
                    continue;
                }
                5 => {
                    pos = len;
                    continue;
                }
                21 => {
                    printmsg(prompt);
                    memmove(
                        buf as *mut libc::c_void,
                        buf.offset(pos as isize) as *const libc::c_void,
                        len.wrapping_sub(pos).wrapping_mul(WCHAR_T_WIDTH as size_t),
                    );
                    len = (len as libc::c_ulong).wrapping_sub(pos) as size_t as size_t;
                    pos = 0 as libc::c_int as size_t;
                    continue;
                }
                27 => {
                    tmp___3 = handle_alt_key(ch.as_mut_ptr());
                    if tmp___3 != -(1 as libc::c_int) {
                        continue;
                    }
                    len = 0 as libc::c_int as size_t;
                    break;
                }
                _ => {
                    if ch[0 as libc::c_int as usize] < 128 as libc::c_uint {
                        tmp___4 = keyname(ch[0 as libc::c_int as usize] as libc::c_int);
                        if *tmp___4.offset(0 as libc::c_int as isize) as libc::c_int
                            == 94 as libc::c_int
                        {
                            continue;
                        }
                    }
                    if !(pos < 255 as libc::c_ulong) {
                        continue;
                    }
                    memmove(
                        buf.offset(pos as isize).offset(1 as libc::c_int as isize)
                            as *mut libc::c_void,
                        buf.offset(pos as isize) as *const libc::c_void,
                        len.wrapping_sub(pos).wrapping_mul(WCHAR_T_WIDTH as size_t),
                    );
                    *buf.offset(pos as isize) = ch[0 as libc::c_int as usize] as wchar_t;
                    len = len.wrapping_add(1);
                    pos = pos.wrapping_add(1);
                    continue;
                }
            }
            if pos > 0 as libc::c_ulong {
                memmove(
                    buf.offset(pos as isize).offset(-(1 as libc::c_int as isize))
                        as *mut libc::c_void,
                    buf.offset(pos as isize) as *const libc::c_void,
                    len.wrapping_sub(pos).wrapping_mul(WCHAR_T_WIDTH as size_t),
                );
                len = len.wrapping_sub(1);
                pos = pos.wrapping_sub(1);
            }
        } else {
            match ch[0 as libc::c_int as usize] {
                410 => {
                    clearoldprompt();
                    xlines = LINES as ushort_t;
                    printmsg(prompt);
                }
                260 => {
                    if pos > 0 as libc::c_ulong {
                        pos = pos.wrapping_sub(1);
                    }
                }
                261 => {
                    if pos < len {
                        pos = pos.wrapping_add(1);
                    }
                }
                263 => {
                    if pos > 0 as libc::c_ulong {
                        memmove(
                            buf.offset(pos as isize).offset(-(1 as libc::c_int as isize))
                                as *mut libc::c_void,
                            buf.offset(pos as isize) as *const libc::c_void,
                            len.wrapping_sub(pos).wrapping_mul(WCHAR_T_WIDTH as size_t),
                        );
                        len = len.wrapping_sub(1);
                        pos = pos.wrapping_sub(1);
                    }
                }
                330 => {
                    if pos < len {
                        memmove(
                            buf.offset(pos as isize) as *mut libc::c_void,
                            buf.offset(pos as isize).offset(1 as libc::c_int as isize)
                                as *const libc::c_void,
                            len
                                .wrapping_sub(pos)
                                .wrapping_sub(1 as libc::c_ulong)
                                .wrapping_mul(WCHAR_T_WIDTH as size_t),
                        );
                        len = len.wrapping_sub(1);
                    }
                }
                360 => {
                    pos = len;
                }
                262 => {
                    pos = 0 as libc::c_int as size_t;
                }
                258 | 259 => {
                    if !prompt.is_null() {
                        if !lastcmd.is_null() {
                            if *prompt as libc::c_int
                                != *(b">>> \0" as *const u8 as *const libc::c_char)
                                    as libc::c_int
                            {
                                tmp___6 = -(1 as libc::c_int);
                            } else {
                                tmp___5 = strcmp(
                                    prompt,
                                    b">>> \0" as *const u8 as *const libc::c_char,
                                );
                                tmp___6 = tmp___5;
                            }
                            if tmp___6 == 0 as libc::c_int {
                                printmsg(prompt);
                                pos = mbstowcs(
                                    buf,
                                    lastcmd as *const libc::c_char,
                                    256 as libc::c_int as size_t,
                                );
                                len = pos;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    curs_set(0 as libc::c_int);
    wtimeout(stdscr, 1000 as libc::c_int);
    printmsg(b"\0" as *const u8 as *const libc::c_char);
    *buf.offset(len as isize) = '\u{0}' as i32;
    pos = wcstombs(
        g_buf.as_mut_ptr(),
        buf as *const wchar_t,
        255 as libc::c_int as size_t,
    );
    if pos >= 255 as libc::c_ulong {
        g_buf[255 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    }
    free(buf as *mut libc::c_void);
    return g_buf.as_mut_ptr();
}
unsafe extern "C" fn xlink(
    mut prefix: *mut libc::c_char,
    mut path: *mut libc::c_char,
    mut curfname: *mut libc::c_char,
    mut buf: *mut libc::c_char,
    mut presel: *mut libc::c_int,
    mut type_0: libc::c_int,
) -> libc::c_int {
    let mut count: libc::c_int = 0;
    let mut choice: libc::c_int = 0;
    let mut psel: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pos: size_t = 0;
    let mut len: size_t = 0;
    let mut r: size_t = 0;
    let mut link_fn: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    > = None;
    let mut lnpath: [libc::c_char; 4096] = [0; 4096];
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: libc::c_int = 0;
    count = 0 as libc::c_int;
    psel = pselbuf;
    pos = 0 as libc::c_int as size_t;
    link_fn = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
        >,
    >(0 as *mut libc::c_void);
    choice = get_cur_or_sel();
    if choice == 0 {
        return -(1 as libc::c_int);
    }
    if type_0 == 115 as libc::c_int {
        link_fn = Some(
            symlink
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        );
    } else {
        link_fn = Some(
            link
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        );
    }
    's_152: {
        if !(choice == 99 as libc::c_int) {
            if !(nselected == 1 as libc::c_int) {
                break 's_152;
            }
        }
        mkpath(
            path as *const libc::c_char,
            prefix as *const libc::c_char,
            lnpath.as_mut_ptr(),
        );
        if choice == 99 as libc::c_int {
            tmp = curfname;
        } else {
            tmp = pselbuf;
        }
        mkpath(path as *const libc::c_char, tmp as *const libc::c_char, buf);
        tmp___0 = (Some(link_fn.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(buf as *const libc::c_char, lnpath.as_mut_ptr() as *const libc::c_char);
        if tmp___0 == 0 {
            if choice == 115 as libc::c_int {
                clearselection();
            }
            return 1 as libc::c_int;
        }
        tmp___1 = __errno_location();
        tmp___2 = strerror(*tmp___1);
        printwait(tmp___2 as *const libc::c_char, presel);
        return -(1 as libc::c_int);
    }
    while pos < selbufpos as size_t {
        len = xstrlen(psel as *const libc::c_char);
        fname = xbasename(psel);
        r = xstrsncpy(buf, prefix as *const libc::c_char, 256 as libc::c_int as size_t);
        xstrsncpy(
            buf.offset(r as isize).offset(-(1 as libc::c_int as isize)),
            fname as *const libc::c_char,
            (255 as libc::c_ulong).wrapping_sub(r),
        );
        mkpath(
            path as *const libc::c_char,
            buf as *const libc::c_char,
            lnpath.as_mut_ptr(),
        );
        tmp___3 = (Some(link_fn.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(psel as *const libc::c_char, lnpath.as_mut_ptr() as *const libc::c_char);
        if tmp___3 == 0 {
            count += 1;
        }
        pos = (pos as libc::c_ulong).wrapping_add(len.wrapping_add(1 as libc::c_ulong))
            as size_t as size_t;
        psel = psel.offset(len.wrapping_add(1 as libc::c_ulong) as isize);
    }
    clearselection();
    return count;
}
unsafe extern "C" fn parsekvpair(
    mut arr: *mut *mut kv,
    mut envcpy: *mut *mut libc::c_char,
    id: uchar_t,
    mut items: *mut uchar_t,
) -> bool {
    let mut new: bool = false;
    let mut INCR: uchar_t = 0;
    let mut i: uint_t = 0;
    let mut kvarr: *mut kv = 0 as *mut kv;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    new = 1 as libc::c_int != 0;
    INCR = 8 as libc::c_int as uchar_t;
    i = 0 as libc::c_int as uint_t;
    kvarr = 0 as *mut libc::c_void as *mut kv;
    tmp = getenv(env_cfg[id as usize]);
    ptr = tmp;
    if ptr.is_null() {
        return 1 as libc::c_int != 0
    } else {
        if *ptr == 0 {
            return 1 as libc::c_int != 0;
        }
    }
    *envcpy = xstrdup(ptr as *const libc::c_char);
    if (*envcpy).is_null() {
        tmp___0 = xitoa(3716 as libc::c_int as uint_t);
        perror(tmp___0 as *const libc::c_char);
        return 0 as libc::c_int != 0;
    }
    ptr = *envcpy;
    while *ptr != 0 {
        if !(i < 100 as libc::c_uint) {
            break;
        }
        if new {
            if i & (INCR as libc::c_int - 1 as libc::c_int) as libc::c_uint == 0 {
                tmp___1 = xrealloc(
                    kvarr as *mut libc::c_void,
                    (::std::mem::size_of::<kv>() as libc::c_ulong)
                        .wrapping_mul(i.wrapping_add(INCR as uint_t) as libc::c_ulong),
                );
                kvarr = tmp___1 as *mut kv;
                *arr = kvarr;
                if kvarr.is_null() {
                    tmp___2 = xitoa(3728 as libc::c_int as uint_t);
                    perror(tmp___2 as *const libc::c_char);
                    return 0 as libc::c_int != 0;
                }
                memset(
                    kvarr.offset(i as isize) as *mut libc::c_void,
                    0 as libc::c_int,
                    (::std::mem::size_of::<kv>() as libc::c_ulong)
                        .wrapping_mul(INCR as libc::c_ulong),
                );
            }
            (*kvarr.offset(i as isize)).key = *ptr as uchar_t as libc::c_int;
            ptr = ptr.offset(1);
            if *ptr as libc::c_int != 58 as libc::c_int {
                return 0 as libc::c_int != 0
            } else {
                ptr = ptr.offset(1);
                if *ptr as libc::c_int == 0 as libc::c_int {
                    return 0 as libc::c_int != 0
                } else {
                    if *ptr as libc::c_int == 59 as libc::c_int {
                        return 0 as libc::c_int != 0;
                    }
                }
            }
            (*kvarr.offset(i as isize))
                .off = ptr.offset_from(*envcpy) as libc::c_long as libc::c_int;
            i = i.wrapping_add(1);
            new = 0 as libc::c_int != 0;
        }
        if *ptr as libc::c_int == 59 as libc::c_int {
            *ptr = '\u{0}' as i32 as libc::c_char;
            new = 1 as libc::c_int != 0;
        }
        ptr = ptr.offset(1);
    }
    *items = i as uchar_t;
    return i != 0 as libc::c_uint;
}
unsafe extern "C" fn get_kv_val(
    mut kvarr: *mut kv,
    mut buf: *mut libc::c_char,
    mut key: libc::c_int,
    mut max: uchar_t,
    mut id: uchar_t,
) -> *mut libc::c_char {
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: libc::c_int = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    if kvarr.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    r = 0 as libc::c_int;
    while r < max as libc::c_int {
        if (*kvarr.offset(r as isize)).key == 0 {
            break;
        }
        if (*kvarr.offset(r as isize)).key == key {
            if id as libc::c_int == 2 as libc::c_int {
                return pluginstr.offset((*kvarr.offset(r as isize)).off as isize);
            }
            val = bmstr.offset((*kvarr.offset(r as isize)).off as isize);
            convert_tilde(val as *const libc::c_char, g_buf.as_mut_ptr());
            if *val.offset(0 as libc::c_int as isize) as libc::c_int
                == 126 as libc::c_int
            {
                tmp = g_buf.as_mut_ptr();
            } else {
                tmp = val;
            }
            tmp___0 = abspath(
                tmp as *const libc::c_char,
                0 as *mut libc::c_void as *const libc::c_char,
                buf,
            );
            return tmp___0;
        }
        r += 1;
    }
    return 0 as *mut libc::c_void as *mut libc::c_char;
}
unsafe extern "C" fn get_kv_key(
    mut kvarr: *mut kv,
    mut val: *mut libc::c_char,
    mut max: uchar_t,
    mut id: uchar_t,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    if kvarr.is_null() {
        return -(1 as libc::c_int);
    }
    if id as libc::c_int != 11 as libc::c_int {
        return -(1 as libc::c_int);
    }
    r = 0 as libc::c_int;
    while r < max as libc::c_int {
        if (*kvarr.offset(r as isize)).key == 0 {
            break;
        }
        if *orderstr.offset((*kvarr.offset(r as isize)).off as isize) as libc::c_int
            != *val as libc::c_int
        {
            tmp___0 = -(1 as libc::c_int);
        } else {
            tmp = strcmp(
                orderstr.offset((*kvarr.offset(r as isize)).off as isize)
                    as *const libc::c_char,
                val as *const libc::c_char,
            );
            tmp___0 = tmp;
        }
        if tmp___0 == 0 as libc::c_int {
            return (*kvarr.offset(r as isize)).key;
        }
        r += 1;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn resetdircolor(mut flags: libc::c_int) {
    if g_state.dircolor() != 0 {
        if flags & 1 as libc::c_int == 0 {
            wattr_off(
                stdscr,
                cfg.curctx().wrapping_add(1 as libc::c_uint) << 8 as libc::c_int
                    & ((1 as libc::c_uint) << 8 as libc::c_int)
                        .wrapping_sub(1 as libc::c_uint) << 8 as libc::c_int
                    | (1 as libc::c_uint) << 21 as libc::c_int,
                0 as *mut libc::c_void,
            );
            g_state.set_dircolor(0 as libc::c_int as uint_t);
        }
    }
}
unsafe extern "C" fn unescape(
    mut str: *const libc::c_char,
    mut maxcols: uint_t,
) -> *mut libc::c_char {
    let mut wbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    wbuf = g_buf.as_mut_ptr();
    buf = wbuf;
    xstrsncpy(wbuf, str, maxcols as size_t);
    while *buf != 0 {
        if *buf as libc::c_int <= 31 as libc::c_int {
            *buf = '?' as i32 as libc::c_char;
        } else if *buf as libc::c_int == 127 as libc::c_int {
            *buf = '?' as i32 as libc::c_char;
        }
        buf = buf.offset(1);
    }
    return wbuf;
}
unsafe extern "C" fn get_size(
    mut size: off_t,
    mut pval: *mut off_t,
    mut comp: libc::c_int,
) -> off_t {
    let mut rem: off_t = 0;
    let mut quo: off_t = 0;
    rem = *pval;
    quo = rem / 10 as libc::c_long;
    if rem - quo * 10 as libc::c_long >= 5 as libc::c_long {
        rem = quo + 1 as libc::c_long;
        if rem == comp as off_t {
            size += 1;
            rem = 0 as libc::c_int as off_t;
        }
    } else {
        rem = quo;
    }
    *pval = rem;
    return size;
}
static mut size_buf: [libc::c_char; 12] = [0; 12];
unsafe extern "C" fn coolsize(mut size: off_t) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut U: *const libc::c_char = 0 as *const libc::c_char;
    let mut rem: off_t = 0;
    let mut ret: size_t = 0;
    let mut i: libc::c_int = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut frac: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut toprint: size_t = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut len: size_t = 0;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: libc::c_char = 0;
    let mut tmp___4: libc::c_char = 0;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: *const libc::c_char = 0 as *const libc::c_char;
    U = b"BKMGTPEZY\0" as *const u8 as *const libc::c_char;
    rem = 0 as libc::c_int as off_t;
    i = 0 as libc::c_int;
    while size >= 1024 as libc::c_long {
        rem = size & 1023 as libc::c_long;
        size >>= 10 as libc::c_int;
        i += 1;
    }
    if i == 1 as libc::c_int {
        rem = rem * 1000 as libc::c_long >> 10 as libc::c_int;
        rem /= 10 as libc::c_long;
        size = get_size(size, &mut rem, 10 as libc::c_int);
    } else if i == 2 as libc::c_int {
        rem = rem * 1000 as libc::c_long >> 10 as libc::c_int;
        size = get_size(size, &mut rem, 100 as libc::c_int);
    } else if i > 2 as libc::c_int {
        rem = rem * 10000 as libc::c_long >> 10 as libc::c_int;
        size = get_size(size, &mut rem, 1000 as libc::c_int);
    }
    if i > 0 as libc::c_int {
        if i < 6 as libc::c_int {
            if rem != 0 {
                tmp = xitoa(size as uint_t);
                ret = xstrsncpy(
                    size_buf.as_mut_ptr(),
                    tmp as *const libc::c_char,
                    12 as libc::c_int as size_t,
                );
                size_buf[ret.wrapping_sub(1 as libc::c_ulong)
                    as usize] = '.' as i32 as libc::c_char;
                tmp___0 = xitoa(rem as uint_t);
                frac = tmp___0;
                if i > 3 as libc::c_int {
                    tmp___1 = 3 as libc::c_int;
                } else {
                    tmp___1 = i;
                }
                toprint = tmp___1 as size_t;
                tmp___2 = xstrlen(frac as *const libc::c_char);
                len = tmp___2;
                if len < toprint {
                    tmp___4 = '0' as i32 as libc::c_char;
                    size_buf[ret.wrapping_add(2 as libc::c_ulong) as usize] = tmp___4;
                    tmp___3 = tmp___4;
                    size_buf[ret.wrapping_add(1 as libc::c_ulong) as usize] = tmp___3;
                    size_buf[ret as usize] = tmp___3;
                    xstrsncpy(
                        size_buf
                            .as_mut_ptr()
                            .offset(ret as isize)
                            .offset(toprint.wrapping_sub(len) as isize),
                        frac as *const libc::c_char,
                        len.wrapping_add(1 as libc::c_ulong),
                    );
                } else {
                    xstrsncpy(
                        size_buf.as_mut_ptr().offset(ret as isize),
                        frac as *const libc::c_char,
                        toprint.wrapping_add(1 as libc::c_ulong),
                    );
                }
                ret = (ret as libc::c_ulong).wrapping_add(toprint) as size_t as size_t;
                current_block = 5141539773904409130;
            } else {
                current_block = 10701073571703952864;
            }
        } else {
            current_block = 10701073571703952864;
        }
    } else {
        current_block = 10701073571703952864;
    }
    match current_block {
        10701073571703952864 => {
            if size != 0 {
                tmp___5 = xitoa(size as uint_t);
                tmp___6 = tmp___5 as *const libc::c_char;
            } else {
                tmp___6 = b"0\0" as *const u8 as *const libc::c_char;
            }
            ret = xstrsncpy(size_buf.as_mut_ptr(), tmp___6, 12 as libc::c_int as size_t);
            ret = ret.wrapping_sub(1);
        }
        _ => {}
    }
    size_buf[ret as usize] = *U.offset(i as isize);
    size_buf[ret.wrapping_add(1 as libc::c_ulong)
        as usize] = '\u{0}' as i32 as libc::c_char;
    return size_buf.as_mut_ptr();
}
static mut rwx: [*const libc::c_char; 8] = [
    b"---\0" as *const u8 as *const libc::c_char,
    b"--x\0" as *const u8 as *const libc::c_char,
    b"-w-\0" as *const u8 as *const libc::c_char,
    b"-wx\0" as *const u8 as *const libc::c_char,
    b"r--\0" as *const u8 as *const libc::c_char,
    b"r-x\0" as *const u8 as *const libc::c_char,
    b"rw-\0" as *const u8 as *const libc::c_char,
    b"rwx\0" as *const u8 as *const libc::c_char,
];
static mut bits: [libc::c_char; 11] = [
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
];
unsafe extern "C" fn get_lsperms(mut mode: mode_t) -> *mut libc::c_char {
    match mode & 61440 as libc::c_uint {
        32768 => {
            bits[0 as libc::c_int as usize] = '-' as i32 as libc::c_char;
        }
        16384 => {
            bits[0 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
        }
        40960 => {
            bits[0 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
        }
        49152 => {
            bits[0 as libc::c_int as usize] = 's' as i32 as libc::c_char;
        }
        4096 => {
            bits[0 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
        }
        24576 => {
            bits[0 as libc::c_int as usize] = 'b' as i32 as libc::c_char;
        }
        8192 => {
            bits[0 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
        }
        _ => {
            bits[0 as libc::c_int as usize] = '?' as i32 as libc::c_char;
        }
    }
    xstrsncpy(
        &mut *bits.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut libc::c_char,
        rwx[(mode >> 6 as libc::c_int & 7 as libc::c_uint) as usize],
        4 as libc::c_int as size_t,
    );
    xstrsncpy(
        &mut *bits.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut libc::c_char,
        rwx[(mode >> 3 as libc::c_int & 7 as libc::c_uint) as usize],
        4 as libc::c_int as size_t,
    );
    xstrsncpy(
        &mut *bits.as_mut_ptr().offset(7 as libc::c_int as isize) as *mut libc::c_char,
        rwx[(mode & 7 as libc::c_uint) as usize],
        4 as libc::c_int as size_t,
    );
    if mode & 2048 as libc::c_uint != 0 {
        if mode & 64 as libc::c_uint != 0 {
            bits[3 as libc::c_int as usize] = 's' as i32 as libc::c_char;
        } else {
            bits[3 as libc::c_int as usize] = 'S' as i32 as libc::c_char;
        }
    }
    if mode & 1024 as libc::c_uint != 0 {
        if mode & 8 as libc::c_uint != 0 {
            bits[6 as libc::c_int as usize] = 's' as i32 as libc::c_char;
        } else {
            bits[6 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
        }
    }
    if mode & 512 as libc::c_uint != 0 {
        if mode & 1 as libc::c_uint != 0 {
            bits[9 as libc::c_int as usize] = 't' as i32 as libc::c_char;
        } else {
            bits[9 as libc::c_int as usize] = 'T' as i32 as libc::c_char;
        }
    }
    return bits.as_mut_ptr();
}
unsafe extern "C" fn print_time(mut timep: *const time_t, flags: uchar_t) {
    let mut t: tm = tm {
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
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    if flags as libc::c_int & 64 as libc::c_int != 0 {
        wattr_on(
            stdscr,
            (1 as libc::c_uint) << 18 as libc::c_int,
            0 as *mut libc::c_void,
        );
    }
    localtime_r(timep, &mut t as *mut tm);
    tmp = xitoa((t.tm_year + 1900 as libc::c_int) as uint_t);
    printw(
        b"%s-%02d-%02d %02d:%02d\0" as *const u8 as *const libc::c_char,
        tmp,
        t.tm_mon + 1 as libc::c_int,
        t.tm_mday,
        t.tm_hour,
        t.tm_min,
    );
    if flags as libc::c_int & 64 as libc::c_int != 0 {
        wattr_off(
            stdscr,
            (1 as libc::c_uint) << 18 as libc::c_int,
            0 as *mut libc::c_void,
        );
    }
}
unsafe extern "C" fn get_detail_ind(mode: mode_t) -> libc::c_char {
    match mode & 61440 as libc::c_uint {
        32768 | 16384 => return ' ' as i32 as libc::c_char,
        40960 => return '@' as i32 as libc::c_char,
        49152 => return '=' as i32 as libc::c_char,
        4096 => return '|' as i32 as libc::c_char,
        24576 => return 'b' as i32 as libc::c_char,
        8192 => return 'c' as i32 as libc::c_char,
        _ => {}
    }
    return '?' as i32 as libc::c_char;
}
unsafe extern "C" fn get_color_pair_name_ind(
    mut ent: *const entry,
    mut pind: *mut libc::c_char,
    mut pattr: *mut libc::c_int,
) -> uchar_t {
    let mut tmp: uint_t = 0;
    let mut tmp___0: chtype = 0;
    let mut tmp___1: libc::c_int = 0;
    match (*ent).mode & 61440 as libc::c_uint {
        32768 => {
            if (*ent).size == 0 {
                if (*ent).mode & 64 as libc::c_uint != 0 {
                    *pind = '*' as i32 as libc::c_char;
                }
                return 16 as libc::c_int as uchar_t;
            }
            if ((*ent).__annonCompField18).flags() & 2 as libc::c_ulonglong != 0 {
                if (*ent).mode & 64 as libc::c_uint != 0 {
                    *pind = '*' as i32 as libc::c_char;
                }
                return 10 as libc::c_int as uchar_t;
            }
            if (*ent).mode & 64 as libc::c_uint != 0 {
                *pind = '*' as i32 as libc::c_char;
                return 8 as libc::c_int as uchar_t;
            }
            return 9 as libc::c_int as uchar_t;
        }
        16384 => {
            *pind = '/' as i32 as libc::c_char;
            if g_state.oldcolor() != 0 {
                return 7 as libc::c_int as uchar_t;
            }
            *pattr = (*pattr as libc::c_uint | (1 as libc::c_uint) << 21 as libc::c_int)
                as libc::c_int;
            if g_state.dirctx() != 0 {
                tmp = cfg.curctx().wrapping_add(1 as libc::c_uint);
            } else {
                tmp = 7 as libc::c_int as uint_t;
            }
            return tmp as uchar_t;
        }
        40960 => {
            if ((*ent).__annonCompField18).flags() & 1 as libc::c_ulonglong != 0 {
                *pind = '/' as i32 as libc::c_char;
                if g_state.oldcolor() != 0 {
                    tmp___0 = (1 as libc::c_uint) << 20 as libc::c_int;
                } else {
                    tmp___0 = (1 as libc::c_uint) << 21 as libc::c_int;
                }
                *pattr = (*pattr as libc::c_uint | tmp___0) as libc::c_int;
            } else {
                *pind = '@' as i32 as libc::c_char;
                if g_state.oldcolor() != 0 {
                    *pattr = (*pattr as libc::c_uint
                        | (1 as libc::c_uint) << 20 as libc::c_int) as libc::c_int;
                }
            }
            's_208: {
                if !(g_state.oldcolor() == 0) {
                    if !(cfg.showdetail() != 0) {
                        break 's_208;
                    }
                }
                if ((*ent).__annonCompField18).flags() & 4 as libc::c_ulonglong != 0 {
                    tmp___1 = 13 as libc::c_int;
                } else {
                    tmp___1 = 11 as libc::c_int;
                }
                return tmp___1 as uchar_t;
            }
            return 0 as libc::c_int as uchar_t;
        }
        49152 => {
            *pind = '=' as i32 as libc::c_char;
            return 15 as libc::c_int as uchar_t;
        }
        4096 => {
            *pind = '|' as i32 as libc::c_char;
            return 14 as libc::c_int as uchar_t;
        }
        24576 => return 5 as libc::c_int as uchar_t,
        8192 => return 6 as libc::c_int as uchar_t,
        _ => {}
    }
    *pind = '?' as i32 as libc::c_char;
    return 16 as libc::c_int as uchar_t;
}
unsafe extern "C" fn printent(
    mut ent: *const entry,
    mut namecols: uint_t,
    mut sel: bool,
) {
    let mut ind: libc::c_char = 0;
    let mut attrs: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut perms: [libc::c_char; 6] = [0; 6];
    let mut tmp: libc::c_uint = 0;
    let mut tmp___0: blkcnt_t = 0;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: libc::c_char = 0;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut color_pair: uchar_t = 0;
    let mut tmp___4: uchar_t = 0;
    let mut tmp___5: libc::c_uint = 0;
    let mut tmp___6: ullong_t = 0;
    let mut tmp___7: *mut libc::c_char = 0 as *mut libc::c_char;
    ind = '\u{0}' as i32 as libc::c_char;
    if cfg.showdetail() != 0 {
        type_0 = ((*ent).mode & 61440 as libc::c_uint) as libc::c_int;
        perms[0 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
        perms[1 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
        perms[2 as libc::c_int
            as usize] = (48 as libc::c_uint)
            .wrapping_add((*ent).mode >> 6 as libc::c_int & 7 as libc::c_uint)
            as libc::c_char;
        perms[3 as libc::c_int
            as usize] = (48 as libc::c_uint)
            .wrapping_add((*ent).mode >> 3 as libc::c_int & 7 as libc::c_uint)
            as libc::c_char;
        perms[4 as libc::c_int
            as usize] = (48 as libc::c_uint)
            .wrapping_add((*ent).mode & 7 as libc::c_uint) as libc::c_char;
        perms[5 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        waddch(stdscr, ' ' as i32 as chtype);
        if g_state.oldcolor() != 0 {
            resetdircolor(((*ent).__annonCompField18).flags() as libc::c_int);
            attrs = ((1 as libc::c_uint) << 20 as libc::c_int) as libc::c_int;
        } else {
            if fcolors[12 as libc::c_int as usize] != 0 {
                tmp = (12 as libc::c_uint) << 8 as libc::c_int
                    & ((1 as libc::c_uint) << 8 as libc::c_int)
                        .wrapping_sub(1 as libc::c_uint) << 8 as libc::c_int;
            } else {
                tmp = 0 as libc::c_uint;
            }
            attrs = tmp as libc::c_int;
        }
        if attrs != 0 {
            wattr_on(stdscr, attrs as attr_t, 0 as *mut libc::c_void);
        }
        print_time(&(*ent).sec, ((*ent).__annonCompField18).flags() as uchar_t);
        let mut current_block_34: u64;
        if type_0 == 32768 as libc::c_int {
            current_block_34 = 13607148488916905390;
        } else if type_0 == 16384 as libc::c_int {
            current_block_34 = 13607148488916905390;
        } else {
            tmp___2 = get_detail_ind((*ent).mode);
            type_0 = tmp___2 as uchar_t as libc::c_int;
            tmp___3 = &mut type_0 as *mut libc::c_int as *mut libc::c_char;
            current_block_34 = 9853141518545631134;
        }
        match current_block_34 {
            13607148488916905390 => {
                if cfg.blkorder() != 0 {
                    tmp___0 = (((*ent).__annonCompField18).blocks() as blkcnt_t)
                        << blk_shift as libc::c_int;
                } else {
                    tmp___0 = (*ent).size;
                }
                tmp___1 = coolsize(tmp___0);
                tmp___3 = tmp___1;
            }
            _ => {}
        }
        printw(
            b"%s%9s \0" as *const u8 as *const libc::c_char,
            perms.as_mut_ptr(),
            tmp___3,
        );
        if attrs != 0 {
            wattr_off(stdscr, attrs as attr_t, 0 as *mut libc::c_void);
        }
    }
    attrs = 0 as libc::c_int;
    tmp___4 = get_color_pair_name_ind(ent, &mut ind, &mut attrs);
    color_pair = tmp___4;
    if ((*ent).__annonCompField18).flags() & 16 as libc::c_ulonglong != 0 {
        tmp___5 = 43 as libc::c_uint | (1 as libc::c_uint) << 18 as libc::c_int
            | (1 as libc::c_uint) << 21 as libc::c_int;
    } else {
        tmp___5 = ' ' as i32 as libc::c_uint;
    }
    waddch(stdscr, tmp___5);
    if g_state.oldcolor() != 0 {
        resetdircolor(((*ent).__annonCompField18).flags() as libc::c_int);
    } else {
        if ((*ent).__annonCompField18).flags() & 8 as libc::c_ulonglong != 0 {
            color_pair = 12 as libc::c_int as uchar_t;
        }
        if color_pair != 0 {
            if fcolors[color_pair as usize] != 0 {
                attrs = (attrs as libc::c_uint
                    | (color_pair as chtype) << 8 as libc::c_int
                        & ((1 as libc::c_uint) << 8 as libc::c_int)
                            .wrapping_sub(1 as libc::c_uint) << 8 as libc::c_int)
                    as libc::c_int;
            }
        }
    }
    if sel {
        attrs = (attrs as libc::c_uint | (1 as libc::c_uint) << 18 as libc::c_int)
            as libc::c_int;
    }
    if attrs != 0 {
        wattr_on(stdscr, attrs as attr_t, 0 as *mut libc::c_void);
    }
    if ind == 0 {
        namecols = namecols.wrapping_add(1);
    }
    if (namecols as ullong_t) < ((*ent).__annonCompField18).nlen() {
        tmp___6 = namecols as ullong_t;
    } else {
        tmp___6 = ((*ent).__annonCompField18).nlen();
    }
    tmp___7 = unescape(
        (*ent).name as *const libc::c_char,
        tmp___6.wrapping_add(1 as libc::c_ulonglong) as uint_t,
    );
    waddnstr(stdscr, tmp___7 as *const libc::c_char, -(1 as libc::c_int));
    if attrs != 0 {
        wattr_off(stdscr, attrs as attr_t, 0 as *mut libc::c_void);
    }
    if ind != 0 {
        waddch(stdscr, ind as chtype);
    }
}
unsafe extern "C" fn savecurctx(
    mut path: *mut libc::c_char,
    mut curname: *mut libc::c_char,
    mut nextctx: libc::c_int,
) {
    let mut tmpcfg: settings = settings {
        filtermode_timeorder_sizeorder_apparentsz_blkorder_extnorder_showhidden_reserved0_showdetail_ctxactive_reverse_version_reserved1_curctx_prefersel_fileinfo_nonavopen_autoenter_reserved2_useeditor_reserved3_regex_x11_timetype_cliopener_waitedit_rollover: [0; 4],
    };
    let mut ctxr: *mut context = 0 as *mut context;
    let mut tmp: libc::c_char = 0;
    let mut tmp___0: libc::c_char = 0;
    let mut tmp___1: libc::c_char = 0;
    tmpcfg = cfg;
    ctxr = &mut *g_ctx.as_mut_ptr().offset(nextctx as isize) as *mut context;
    if !curname.is_null() {
        xstrsncpy(
            (g_ctx[tmpcfg.curctx() as usize].c_name).as_mut_ptr(),
            curname as *const libc::c_char,
            256 as libc::c_int as size_t,
        );
    } else {
        g_ctx[tmpcfg.curctx() as usize]
            .c_name[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    }
    g_ctx[tmpcfg.curctx() as usize].c_cfg = tmpcfg;
    if ((*ctxr).c_cfg).ctxactive() != 0 {
        tmpcfg = (*ctxr).c_cfg;
        if !order.is_null() {
            cfgsort[4 as libc::c_int as usize] = cfgsort[nextctx as usize];
            cfgsort[nextctx as usize] = '0' as i32 as uchar_t;
        }
    } else {
        ((*ctxr).c_cfg).set_ctxactive(1 as libc::c_int as uint_t);
        xstrsncpy(
            ((*ctxr).c_path).as_mut_ptr(),
            path as *const libc::c_char,
            4096 as libc::c_int as size_t,
        );
        tmp___1 = '\u{0}' as i32 as libc::c_char;
        (*ctxr).c_fltr[1 as libc::c_int as usize] = tmp___1;
        tmp___0 = tmp___1;
        (*ctxr).c_fltr[0 as libc::c_int as usize] = tmp___0;
        tmp = tmp___0;
        (*ctxr).c_name[0 as libc::c_int as usize] = tmp;
        (*ctxr).c_last[0 as libc::c_int as usize] = tmp;
        (*ctxr).c_cfg = tmpcfg;
        if cfgsort[cfg.curctx() as usize] as libc::c_int == 122 as libc::c_int {
            cfgsort[nextctx as usize] = 'z' as i32 as uchar_t;
        }
    }
    tmpcfg.set_curctx(nextctx as uint_t);
    cfg = tmpcfg;
}
unsafe extern "C" fn get_free_ctx() -> uchar_t {
    let mut r: uchar_t = 0;
    r = cfg.curctx() as uchar_t;
    loop {
        r = (r as libc::c_int + 1 as libc::c_int & -(5 as libc::c_int)) as uchar_t;
        if !((g_ctx[r as usize].c_cfg).ctxactive() != 0) {
            break;
        }
        if !(r as uint_t != cfg.curctx()) {
            break;
        }
    }
    return r;
}
unsafe extern "C" fn set_smart_ctx(
    mut ctx: libc::c_int,
    mut nextpath: *mut libc::c_char,
    mut path: *mut *mut libc::c_char,
    mut file: *mut libc::c_char,
    mut lastname: *mut *mut libc::c_char,
    mut lastdir: *mut *mut libc::c_char,
) {
    let mut tmp: uchar_t = 0;
    if ctx == 43 as libc::c_int {
        tmp = get_free_ctx();
        ctx = tmp as libc::c_int + 1 as libc::c_int;
    }
    if ctx == 0 as libc::c_int {
        xstrsncpy(*lastdir, *path as *const libc::c_char, 4096 as libc::c_int as size_t);
        xstrsncpy(*path, nextpath as *const libc::c_char, 4096 as libc::c_int as size_t);
    } else if ctx as uint_t == cfg.curctx().wrapping_add(1 as libc::c_uint) {
        xstrsncpy(*lastdir, *path as *const libc::c_char, 4096 as libc::c_int as size_t);
        xstrsncpy(*path, nextpath as *const libc::c_char, 4096 as libc::c_int as size_t);
    } else {
        ctx -= 1;
        (g_ctx[ctx as usize].c_cfg).set_ctxactive(0 as libc::c_int as uint_t);
        savecurctx(nextpath, file, ctx);
        *path = (g_ctx[ctx as usize].c_path).as_mut_ptr();
        *lastdir = (g_ctx[ctx as usize].c_last).as_mut_ptr();
        *lastname = (g_ctx[ctx as usize].c_name).as_mut_ptr();
    };
}
unsafe extern "C" fn get_output(
    mut file: *mut libc::c_char,
    mut arg1: *mut libc::c_char,
    mut arg2: *mut libc::c_char,
    mut fdout: libc::c_int,
    mut multi: bool,
    mut page: bool,
) -> bool {
    let mut pid: pid_t = 0;
    let mut pipefd: [libc::c_int; 2] = [0; 2];
    let mut index___0: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut ret: bool = false;
    let mut tmpfile___0: bool = false;
    let mut tmp: libc::c_int = 0;
    let mut argv: [*mut libc::c_char; 10] = [0 as *mut libc::c_char; 10];
    let mut tmp___0: libc::c_uint = 0;
    let mut cmd___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fd: libc::c_int = 0;
    let mut len: ssize_t = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: ssize_t = 0;
    index___0 = 0 as libc::c_int;
    ret = 0 as libc::c_int != 0;
    if fdout == -(1 as libc::c_int) {
        if page {
            tmp = 1 as libc::c_int;
        } else {
            tmp = 0 as libc::c_int;
        }
    } else {
        tmp = 0 as libc::c_int;
    }
    tmpfile___0 = tmp != 0;
    argv[0 as libc::c_int as usize] = 0 as *mut libc::c_char;
    tmp___0 = 1 as libc::c_uint;
    while !(tmp___0 >= 10 as libc::c_uint) {
        argv[tmp___0 as usize] = 0 as *mut libc::c_char;
        tmp___0 = tmp___0.wrapping_add(1);
    }
    cmd___0 = 0 as *mut libc::c_void as *mut libc::c_char;
    fd = -(1 as libc::c_int);
    if tmpfile___0 {
        fdout = create_tmp_file();
        if fdout == -(1 as libc::c_int) {
            return 0 as libc::c_int != 0;
        }
    }
    if multi {
        cmd___0 = parseargs(file, argv.as_mut_ptr(), &mut index___0);
        if cmd___0.is_null() {
            return 0 as libc::c_int != 0;
        }
    } else {
        tmp___1 = index___0;
        index___0 += 1;
        argv[tmp___1 as usize] = file;
    }
    argv[index___0 as usize] = arg1;
    index___0 += 1;
    argv[index___0 as usize] = arg2;
    tmp___2 = pipe(pipefd.as_mut_ptr());
    if tmp___2 == -(1 as libc::c_int) {
        free(cmd___0 as *mut libc::c_void);
        printerr(4395 as libc::c_int);
    }
    index___0 = 0 as libc::c_int;
    while index___0 < 2 as libc::c_int {
        flags = fcntl(pipefd[index___0 as usize], 3 as libc::c_int, 0 as libc::c_int);
        flags |= 2048 as libc::c_int;
        fcntl(pipefd[index___0 as usize], 4 as libc::c_int, flags);
        index___0 += 1;
    }
    pid = fork();
    if pid == 0 as libc::c_int {
        close(pipefd[0 as libc::c_int as usize]);
        dup2(pipefd[1 as libc::c_int as usize], 1 as libc::c_int);
        dup2(pipefd[1 as libc::c_int as usize], 2 as libc::c_int);
        close(pipefd[1 as libc::c_int as usize]);
        execvp(
            argv[0 as libc::c_int as usize] as *const libc::c_char,
            argv.as_mut_ptr() as *const *mut libc::c_char,
        );
        _exit(0 as libc::c_int);
    }
    waitpid(pid, 0 as *mut libc::c_void as *mut libc::c_int, 0 as libc::c_int);
    close(pipefd[1 as libc::c_int as usize]);
    free(cmd___0 as *mut libc::c_void);
    loop {
        len = read(
            pipefd[0 as libc::c_int as usize],
            g_buf.as_mut_ptr() as *mut libc::c_void,
            (4096 as libc::c_int + ((256 as libc::c_int) << 1 as libc::c_int)
                - 1 as libc::c_int) as size_t,
        );
        if !(len > 0 as libc::c_long) {
            break;
        }
        ret = 1 as libc::c_int != 0;
        if fdout == -(1 as libc::c_int) {
            break;
        }
        tmp___3 = write(fdout, g_buf.as_mut_ptr() as *const libc::c_void, len as size_t);
        if tmp___3 != len {
            break;
        }
    }
    close(pipefd[0 as libc::c_int as usize]);
    if !page {
        return ret;
    }
    if tmpfile___0 {
        close(fdout);
        close(fd);
    }
    spawn(
        pager,
        g_tmpfpath.as_mut_ptr(),
        0 as *mut libc::c_void as *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
        265 as libc::c_int as ushort_t,
    );
    if tmpfile___0 {
        unlink(g_tmpfpath.as_mut_ptr() as *const libc::c_char);
    }
    return 1 as libc::c_int != 0;
}
static mut cmds: [*mut libc::c_char; 3] = [
    b"file -biL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"file -b\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"stat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
unsafe extern "C" fn show_stats(mut fpath: *mut libc::c_char) -> bool {
    let mut r: size_t = 0;
    let mut fd: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    r = (::std::mem::size_of::<[*mut libc::c_char; 3]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong);
    tmp = create_tmp_file();
    fd = tmp;
    if fd == -(1 as libc::c_int) {
        return 0 as libc::c_int != 0;
    }
    while r != 0 {
        r = r.wrapping_sub(1);
        get_output(
            cmds[r as usize],
            fpath,
            0 as *mut libc::c_void as *mut libc::c_char,
            fd,
            1 as libc::c_int != 0,
            0 as libc::c_int != 0,
        );
    }
    close(fd);
    spawn(
        pager,
        g_tmpfpath.as_mut_ptr(),
        0 as *mut libc::c_void as *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
        265 as libc::c_int as ushort_t,
    );
    unlink(g_tmpfpath.as_mut_ptr() as *const libc::c_char);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn xchmod(mut fpath: *const libc::c_char, mut mode: mode_t) -> bool {
    let mut tmp: libc::c_int = 0;
    if 64 as libc::c_uint & mode != 0 {
        mode &= 4294967222 as libc::c_uint;
    } else {
        mode |= 73 as libc::c_uint;
    }
    tmp = chmod(fpath, mode);
    return tmp == 0 as libc::c_int;
}
unsafe extern "C" fn get_fs_info(
    mut path: *const libc::c_char,
    mut type_0: uchar_t,
) -> size_t {
    let mut svb: statvfs = statvfs {
        f_bsize: 0,
        f_frsize: 0,
        f_blocks: 0,
        f_bfree: 0,
        f_bavail: 0,
        f_files: 0,
        f_ffree: 0,
        f_favail: 0,
        f_fsid: 0,
        f_flag: 0,
        f_namemax: 0,
        __f_spare: [0; 6],
    };
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    tmp = statvfs(path, &mut svb as *mut statvfs);
    if tmp == -(1 as libc::c_int) {
        return 0 as libc::c_int as size_t;
    }
    if type_0 as libc::c_int == 0 as libc::c_int {
        tmp___0 = ffs((svb.f_frsize >> 1 as libc::c_int) as libc::c_int);
        return svb.f_bavail << tmp___0;
    }
    if type_0 as libc::c_int == 1 as libc::c_int {
        tmp___1 = ffs((svb.f_frsize >> 1 as libc::c_int) as libc::c_int);
        return (svb.f_blocks).wrapping_sub(svb.f_bfree) << tmp___1;
    }
    tmp___2 = ffs((svb.f_frsize >> 1 as libc::c_int) as libc::c_int);
    return svb.f_blocks << tmp___2;
}
unsafe extern "C" fn xmktree(mut path: *mut libc::c_char, mut dir: bool) -> bool {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut slash: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut fd: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: *mut libc::c_int = 0 as *mut libc::c_int;
    p = path;
    slash = path;
    if p.is_null() {
        return 0 as libc::c_int != 0
    } else {
        if *p == 0 {
            return 0 as libc::c_int != 0;
        }
    }
    p = p.offset(1);
    while *p as libc::c_int != 0 as libc::c_int {
        if *p as libc::c_int == 47 as libc::c_int {
            slash = p;
            *p = '\u{0}' as i32 as libc::c_char;
            tmp = mkdir(path as *const libc::c_char, 511 as libc::c_int as __mode_t);
            if tmp == -(1 as libc::c_int) {
                tmp___0 = __errno_location();
                if *tmp___0 != 17 as libc::c_int {
                    *slash = '/' as i32 as libc::c_char;
                    return 0 as libc::c_int != 0;
                }
            }
            *slash = '/' as i32 as libc::c_char;
            p = p.offset(1);
        } else {
            p = p.offset(1);
        }
    }
    if dir {
        tmp___1 = mkdir(path as *const libc::c_char, 511 as libc::c_int as __mode_t);
        if tmp___1 == -(1 as libc::c_int) {
            tmp___2 = __errno_location();
            if *tmp___2 != 17 as libc::c_int {
                return 0 as libc::c_int != 0;
            }
        }
    } else {
        tmp___3 = open(
            path as *const libc::c_char,
            64 as libc::c_int,
            438 as libc::c_int,
        );
        fd = tmp___3;
        if fd == -(1 as libc::c_int) {
            tmp___4 = __errno_location();
            if *tmp___4 != 17 as libc::c_int {
                return 0 as libc::c_int != 0;
            }
        }
        close(fd);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn handle_archive(
    mut fpath: *mut libc::c_char,
    mut op: libc::c_char,
) -> bool {
    let mut arg: [libc::c_char; 5] = [0; 5];
    let mut util: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outdir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x_to: bool = false;
    let mut is_atool: bool = false;
    let mut tmp: bool = false;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: bool = false;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: bool = false;
    let mut tmp___8: bool = false;
    let mut tmp___9: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___10: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___11: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___12: libc::c_int = 0;
    arg[0 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    arg[1 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    arg[2 as libc::c_int as usize] = 'v' as i32 as libc::c_char;
    arg[3 as libc::c_int as usize] = 'f' as i32 as libc::c_char;
    arg[4 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    outdir = 0 as *mut libc::c_void as *mut libc::c_char;
    x_to = 0 as libc::c_int != 0;
    if g_state.usebsdtar() == 0 {
        tmp = getutil(utils[1 as libc::c_int as usize]);
        if tmp {
            tmp___0 = 1 as libc::c_int;
        } else {
            tmp___0 = 0 as libc::c_int;
        }
    } else {
        tmp___0 = 0 as libc::c_int;
    }
    is_atool = tmp___0 != 0;
    if op as libc::c_int == 120 as libc::c_int {
        if is_atool {
            tmp___2 = b".\0" as *const u8 as *const libc::c_char;
        } else {
            tmp___1 = xbasename(fpath);
            tmp___2 = tmp___1 as *const libc::c_char;
        }
        outdir = xreadline(tmp___2, messages[19 as libc::c_int as usize]);
        if outdir.is_null() {
            printwait(
                messages[4 as libc::c_int as usize],
                0 as *mut libc::c_void as *mut libc::c_int,
            );
            return 0 as libc::c_int != 0;
        } else {
            if *outdir == 0 {
                printwait(
                    messages[4 as libc::c_int as usize],
                    0 as *mut libc::c_void as *mut libc::c_int,
                );
                return 0 as libc::c_int != 0;
            }
        }
        let mut current_block_49: u64;
        if *outdir as libc::c_int == 46 as libc::c_int {
            if !(*outdir.offset(1 as libc::c_int as isize) as libc::c_int
                == 0 as libc::c_int)
            {
                current_block_49 = 9059532038564164568;
            } else {
                current_block_49 = 1434579379687443766;
            }
        } else {
            current_block_49 = 9059532038564164568;
        }
        match current_block_49 {
            9059532038564164568 => {
                tmp___5 = xmktree(outdir, 1 as libc::c_int != 0);
                if tmp___5 {
                    tmp___6 = chdir(outdir as *const libc::c_char);
                    if tmp___6 == -(1 as libc::c_int) {
                        tmp___3 = __errno_location();
                        tmp___4 = strerror(*tmp___3);
                        printwait(
                            tmp___4 as *const libc::c_char,
                            0 as *mut libc::c_void as *mut libc::c_int,
                        );
                        return 0 as libc::c_int != 0;
                    }
                } else {
                    tmp___3 = __errno_location();
                    tmp___4 = strerror(*tmp___3);
                    printwait(
                        tmp___4 as *const libc::c_char,
                        0 as *mut libc::c_void as *mut libc::c_int,
                    );
                    return 0 as libc::c_int != 0;
                }
                outdir = getcwd(
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as libc::c_int as size_t,
                );
                x_to = 1 as libc::c_int != 0;
            }
            _ => {}
        }
    }
    if is_atool {
        util = utils[1 as libc::c_int as usize];
        arg[1 as libc::c_int as usize] = op;
        arg[2 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    } else {
        tmp___8 = getutil(utils[2 as libc::c_int as usize]);
        if tmp___8 {
            util = utils[2 as libc::c_int as usize];
            if op as libc::c_int == 120 as libc::c_int {
                arg[1 as libc::c_int as usize] = op;
            }
        } else {
            tmp___7 = is_suffix(
                fpath as *const libc::c_char,
                b".zip\0" as *const u8 as *const libc::c_char,
            );
            if tmp___7 {
                util = utils[3 as libc::c_int as usize];
                if op as libc::c_int == 108 as libc::c_int {
                    arg[1 as libc::c_int as usize] = 'v' as i32 as libc::c_char;
                } else {
                    arg[1 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
                }
                arg[2 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
            } else {
                util = utils[4 as libc::c_int as usize];
                if op as libc::c_int == 120 as libc::c_int {
                    arg[1 as libc::c_int as usize] = op;
                }
            }
        }
    }
    if op as libc::c_int == 120 as libc::c_int {
        spawn(
            util,
            arg.as_mut_ptr(),
            fpath,
            0 as *mut libc::c_void as *mut libc::c_char,
            9 as libc::c_int as ushort_t,
        );
    } else {
        get_output(
            util,
            arg.as_mut_ptr(),
            fpath,
            -(1 as libc::c_int),
            1 as libc::c_int != 0,
            1 as libc::c_int != 0,
        );
    }
    if x_to {
        tmp___11 = xdirname(fpath);
        tmp___12 = chdir(tmp___11 as *const libc::c_char);
        if tmp___12 == -(1 as libc::c_int) {
            tmp___9 = __errno_location();
            tmp___10 = strerror(*tmp___9);
            printwait(
                tmp___10 as *const libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_int,
            );
            free(outdir as *mut libc::c_void);
            return 0 as libc::c_int != 0;
        }
        xstrsncpy(fpath, outdir as *const libc::c_char, 4096 as libc::c_int as size_t);
        free(outdir as *mut libc::c_void);
    } else if op as libc::c_int == 120 as libc::c_int {
        *fpath.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn visit_parent(
    mut path: *mut libc::c_char,
    mut newpath: *mut libc::c_char,
    mut presel: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    if *path.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
        if *path.offset(0 as libc::c_int as isize) as libc::c_int == 47 as libc::c_int {
            if cfg.filtermode() != 0 {
                if !presel.is_null() {
                    *presel = '/' as i32;
                }
            }
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
    }
    if !newpath.is_null() {
        xstrsncpy(newpath, path as *const libc::c_char, 4096 as libc::c_int as size_t);
    } else {
        newpath = path;
    }
    dir = xdirname(newpath);
    tmp___1 = chdir(dir as *const libc::c_char);
    if tmp___1 == -(1 as libc::c_int) {
        tmp = __errno_location();
        tmp___0 = strerror(*tmp);
        printwait(tmp___0 as *const libc::c_char, presel);
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    return dir;
}
unsafe extern "C" fn valid_parent(
    mut path: *mut libc::c_char,
    mut lastname: *mut libc::c_char,
) {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = xbasename(path);
    xstrsncpy(lastname, tmp as *const libc::c_char, 256 as libc::c_int as size_t);
    loop {
        if *path.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
            if *path.offset(0 as libc::c_int as isize) as libc::c_int
                == 47 as libc::c_int
            {
                break;
            }
        }
        tmp___0 = visit_parent(
            path,
            0 as *mut libc::c_void as *mut libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        if !tmp___0.is_null() {
            break;
        }
    }
    tmp___1 = __errno_location();
    tmp___2 = strerror(*tmp___1);
    printwait(
        tmp___2 as *const libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_int,
    );
    xdelay(350000 as libc::c_int as useconds_t);
}
unsafe extern "C" fn archive_mount(mut newpath: *mut libc::c_char) -> bool {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmd___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut mntpath: [libc::c_char; 4096] = [0; 4096];
    let mut tmp: bool = false;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: bool = false;
    let mut tmp___3: libc::c_int = 0;
    str = b"install archivemount\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    cmd___0 = str.offset(8 as libc::c_int as isize);
    name = (*pdents.offset(cur as isize)).name;
    len = ((*pdents.offset(cur as isize)).__annonCompField18).nlen() as size_t;
    tmp = getutil(cmd___0);
    if !tmp {
        printmsg(str as *const libc::c_char);
        return 0 as libc::c_int != 0;
    }
    dir = xstrdup(name as *const libc::c_char);
    if dir.is_null() {
        printmsg(messages[5 as libc::c_int as usize]);
        return 0 as libc::c_int != 0;
    }
    while len > 1 as libc::c_ulong {
        len = len.wrapping_sub(1);
        if !(*dir.offset(len as isize) as libc::c_int == 46 as libc::c_int) {
            continue;
        }
        *dir.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
        break;
    }
    mkpath(
        cfgpath as *const libc::c_char,
        toks[2 as libc::c_int as usize],
        mntpath.as_mut_ptr(),
    );
    mkpath(
        mntpath.as_mut_ptr() as *const libc::c_char,
        dir as *const libc::c_char,
        newpath,
    );
    free(dir as *mut libc::c_void);
    tmp___2 = xmktree(newpath, 1 as libc::c_int != 0);
    if !tmp___2 {
        tmp___0 = __errno_location();
        tmp___1 = strerror(*tmp___0);
        printwait(
            tmp___1 as *const libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        return 0 as libc::c_int != 0;
    }
    tmp___3 = spawn(
        cmd___0,
        name,
        newpath,
        0 as *mut libc::c_void as *mut libc::c_char,
        8 as libc::c_int as ushort_t,
    );
    if tmp___3 != 0 {
        printmsg(messages[5 as libc::c_int as usize]);
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn remote_mount(mut newpath: *mut libc::c_char) -> bool {
    let mut flag: uchar_t = 0;
    let mut opt: libc::c_int = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut env: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: bool = false;
    let mut tmp___0: bool = false;
    let mut s: bool = false;
    let mut tmp___1: bool = false;
    let mut mntpath: [libc::c_char; 4096] = [0; 4096];
    let mut div___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: bool = false;
    let mut len: size_t = 0;
    let mut tmp___6: size_t = 0;
    let mut tmp___7: libc::c_int = 0;
    flag = 9 as libc::c_int as uchar_t;
    tmp___0 = getutil(utils[10 as libc::c_int as usize]);
    r = tmp___0;
    tmp___1 = getutil(utils[9 as libc::c_int as usize]);
    s = tmp___1;
    if !r {
        if !s {
            printmsg(b"install sshfs/rclone\0" as *const u8 as *const libc::c_char);
            return 0 as libc::c_int != 0;
        }
    }
    let mut current_block_17: u64;
    if r {
        if s {
            opt = get_input(messages[30 as libc::c_int as usize]);
            current_block_17 = 14401909646449704462;
        } else {
            current_block_17 = 6165757582300958552;
        }
    } else {
        current_block_17 = 6165757582300958552;
    }
    match current_block_17 {
        6165757582300958552 => {
            if !s {
                opt = 'r' as i32;
            } else {
                opt = 's' as i32;
            }
        }
        _ => {}
    }
    if opt == 115 as libc::c_int {
        env = xgetenv(
            b"NNN_SSHFS\0" as *const u8 as *const libc::c_char,
            utils[9 as libc::c_int as usize],
        );
    } else if opt == 114 as libc::c_int {
        flag = (flag as libc::c_int | 6 as libc::c_int) as uchar_t;
        env = xgetenv(
            b"NNN_RCLONE\0" as *const u8 as *const libc::c_char,
            b"rclone mount\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else {
        printmsg(messages[40 as libc::c_int as usize]);
        return 0 as libc::c_int != 0;
    }
    tmp = xreadline(
        0 as *mut libc::c_void as *const libc::c_char,
        b"host[:dir] > \0" as *const u8 as *const libc::c_char,
    );
    if *tmp.offset(0 as libc::c_int as isize) == 0 {
        printmsg(messages[4 as libc::c_int as usize]);
        return 0 as libc::c_int != 0;
    }
    tmp___2 = strchr(tmp as *const libc::c_char, ':' as i32);
    div___0 = tmp___2;
    if !div___0.is_null() {
        *div___0 = '\u{0}' as i32 as libc::c_char;
    }
    mkpath(
        cfgpath as *const libc::c_char,
        toks[2 as libc::c_int as usize],
        mntpath.as_mut_ptr(),
    );
    mkpath(
        mntpath.as_mut_ptr() as *const libc::c_char,
        tmp as *const libc::c_char,
        newpath,
    );
    tmp___5 = xmktree(newpath, 1 as libc::c_int != 0);
    if !tmp___5 {
        tmp___3 = __errno_location();
        tmp___4 = strerror(*tmp___3);
        printwait(
            tmp___4 as *const libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        return 0 as libc::c_int != 0;
    }
    if div___0.is_null() {
        tmp___6 = xstrlen(tmp as *const libc::c_char);
        len = tmp___6;
        *tmp.offset(len as isize) = ':' as i32 as libc::c_char;
        *tmp
            .offset(
                len.wrapping_add(1 as libc::c_ulong) as isize,
            ) = '\u{0}' as i32 as libc::c_char;
    } else {
        *div___0 = ':' as i32 as libc::c_char;
    }
    if opt == 115 as libc::c_int {
        tmp___7 = spawn(
            env,
            tmp,
            newpath,
            0 as *mut libc::c_void as *mut libc::c_char,
            flag as ushort_t,
        );
        if tmp___7 != 0 {
            printmsg(messages[5 as libc::c_int as usize]);
            return 0 as libc::c_int != 0;
        }
    } else {
        spawn(
            env,
            tmp,
            newpath,
            0 as *mut libc::c_void as *mut libc::c_char,
            flag as ushort_t,
        );
        printmsg(messages[31 as libc::c_int as usize]);
        xdelay(((350000 as libc::c_int) << 2 as libc::c_int) as useconds_t);
    }
    return 1 as libc::c_int != 0;
}
static mut cmd: [libc::c_char; 12] = [
    'f' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
];
static mut found: bool = 0 as libc::c_int != 0;
unsafe extern "C" fn unmount(
    mut name: *mut libc::c_char,
    mut newpath: *mut libc::c_char,
    mut presel: *mut libc::c_int,
    mut currentpath: *mut libc::c_char,
) -> bool {
    let mut current_block: u64;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
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
    let mut psb: stat = stat {
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
    let mut child: bool = false;
    let mut parent: bool = false;
    let mut hovered: bool = false;
    let mut mntpath: [libc::c_char; 4096] = [0; 4096];
    let mut tmp___0: bool = false;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: bool = false;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: bool = false;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___11: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___12: libc::c_int = 0;
    tmp = name;
    child = 0 as libc::c_int != 0;
    parent = 0 as libc::c_int != 0;
    hovered = 0 as libc::c_int != 0;
    if !found {
        tmp___0 = getutil(cmd.as_mut_ptr());
        if !tmp___0 {
            cmd[10 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
            found = 1 as libc::c_int != 0;
        }
    }
    mkpath(
        cfgpath as *const libc::c_char,
        toks[2 as libc::c_int as usize],
        mntpath.as_mut_ptr(),
    );
    if !tmp.is_null() {
        tmp___4 = strcmp(
            mntpath.as_mut_ptr() as *const libc::c_char,
            currentpath as *const libc::c_char,
        );
        if tmp___4 == 0 as libc::c_int {
            mkpath(
                mntpath.as_mut_ptr() as *const libc::c_char,
                tmp as *const libc::c_char,
                newpath,
            );
            tmp___1 = lstat(newpath as *const libc::c_char, &mut sb as *mut stat);
            child = tmp___1 != -(1 as libc::c_int);
            tmp___2 = xdirname(newpath);
            tmp___3 = lstat(tmp___2 as *const libc::c_char, &mut psb as *mut stat);
            parent = tmp___3 != -(1 as libc::c_int);
            if !child {
                if !parent {
                    *presel = '$' as i32;
                    return 0 as libc::c_int != 0;
                }
            }
        }
    }
    if tmp.is_null() {
        current_block = 4575321678384368962;
    } else if !child {
        current_block = 4575321678384368962;
    } else if !(sb.st_mode & 61440 as libc::c_uint == 16384 as libc::c_uint) {
        current_block = 4575321678384368962;
    } else if child {
        if parent {
            if sb.st_dev == psb.st_dev {
                current_block = 4575321678384368962;
            } else {
                current_block = 10399321362245223758;
            }
        } else {
            current_block = 10399321362245223758;
        }
    } else {
        current_block = 10399321362245223758;
    }
    match current_block {
        4575321678384368962 => {
            tmp = xreadline(
                0 as *mut libc::c_void as *const libc::c_char,
                messages[16 as libc::c_int as usize],
            );
            if *tmp.offset(0 as libc::c_int as isize) == 0 {
                return 0 as libc::c_int != 0;
            }
            if !name.is_null() {
                if *tmp.offset(0 as libc::c_int as isize) as libc::c_int
                    == 45 as libc::c_int
                {
                    if *tmp.offset(1 as libc::c_int as isize) as libc::c_int
                        == 0 as libc::c_int
                    {
                        mkpath(
                            currentpath as *const libc::c_char,
                            name as *const libc::c_char,
                            newpath,
                        );
                        hovered = 1 as libc::c_int != 0;
                    }
                }
            }
        }
        _ => {}
    }
    if !hovered {
        mkpath(
            mntpath.as_mut_ptr() as *const libc::c_char,
            tmp as *const libc::c_char,
            newpath,
        );
    }
    tmp___5 = xdiraccess(newpath as *const libc::c_char);
    if !tmp___5 {
        *presel = '$' as i32;
        return 0 as libc::c_int != 0;
    }
    tmp___9 = spawn(
        cmd.as_mut_ptr(),
        b"-qu\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        newpath,
        0 as *mut libc::c_void as *mut libc::c_char,
        8 as libc::c_int as ushort_t,
    );
    if tmp___9 != 0 {
        tmp___6 = get_input(messages[37 as libc::c_int as usize]);
        tmp___7 = xconfirm(tmp___6);
        if !tmp___7 {
            return 0 as libc::c_int != 0;
        }
        tmp___8 = spawn(
            cmd.as_mut_ptr(),
            b"-quz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            newpath,
            0 as *mut libc::c_void as *mut libc::c_char,
            8 as libc::c_int as ushort_t,
        );
        if tmp___8 != 0 {
            printwait(messages[5 as libc::c_int as usize], presel);
            return 0 as libc::c_int != 0;
        }
    }
    tmp___12 = rmdir(newpath as *const libc::c_char);
    if tmp___12 == -(1 as libc::c_int) {
        tmp___10 = __errno_location();
        tmp___11 = strerror(*tmp___10);
        printwait(tmp___11 as *const libc::c_char, presel);
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn lock_terminal() {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = xgetenv(
        b"NNN_LOCKER\0" as *const u8 as *const libc::c_char,
        utils[5 as libc::c_int as usize],
    );
    spawn(
        tmp,
        0 as *mut libc::c_void as *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
        9 as libc::c_int as ushort_t,
    );
}
unsafe extern "C" fn printkv(
    mut kvarr: *mut kv,
    mut fd: libc::c_int,
    mut max: uchar_t,
    mut id: uchar_t,
) {
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: uchar_t = 0;
    if id as libc::c_int == 1 as libc::c_int {
        tmp = bmstr;
    } else {
        tmp = pluginstr;
    }
    val = tmp;
    i = 0 as libc::c_int as uchar_t;
    while (i as libc::c_int) < max as libc::c_int {
        if (*kvarr.offset(i as libc::c_int as isize)).key == 0 {
            break;
        }
        dprintf(
            fd,
            b" %c: %s\n\0" as *const u8 as *const libc::c_char,
            (*kvarr.offset(i as libc::c_int as isize)).key as libc::c_char
                as libc::c_int,
            val.offset((*kvarr.offset(i as libc::c_int as isize)).off as isize),
        );
        i = (i as libc::c_int + 1 as libc::c_int) as uchar_t;
    }
}
unsafe extern "C" fn printkeys(
    mut kvarr: *mut kv,
    mut buf: *mut libc::c_char,
    mut max: uchar_t,
) {
    let mut i: uchar_t = 0;
    i = 0 as libc::c_int as uchar_t;
    while (i as libc::c_int) < max as libc::c_int {
        if (*kvarr.offset(i as libc::c_int as isize)).key == 0 {
            break;
        }
        *buf
            .offset(
                ((i as libc::c_int) << 1 as libc::c_int) as isize,
            ) = ' ' as i32 as libc::c_char;
        *buf
            .offset(
                (((i as libc::c_int) << 1 as libc::c_int) + 1 as libc::c_int) as isize,
            ) = (*kvarr.offset(i as libc::c_int as isize)).key as libc::c_char;
        i = (i as libc::c_int + 1 as libc::c_int) as uchar_t;
    }
    *buf
        .offset(
            ((i as libc::c_int) << 1 as libc::c_int) as isize,
        ) = '\u{0}' as i32 as libc::c_char;
}
unsafe extern "C" fn handle_bookmark(
    mut bmark: *const libc::c_char,
    mut newpath: *mut libc::c_char,
) -> size_t {
    let mut fd: libc::c_int = 0;
    let mut r: size_t = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    fd = '\r' as i32;
    let mut current_block_13: u64;
    if maxbm != 0 {
        current_block_13 = 8357604818694948764;
    } else if !bmark.is_null() {
        current_block_13 = 8357604818694948764;
    } else {
        current_block_13 = 4166486009154926805;
    }
    match current_block_13 {
        8357604818694948764 => {
            r = xstrsncpy(
                g_buf.as_mut_ptr(),
                messages[34 as libc::c_int as usize],
                (4096 as libc::c_int + ((256 as libc::c_int) << 1 as libc::c_int))
                    as size_t,
            );
            if !bmark.is_null() {
                r = r.wrapping_sub(1);
                g_buf[r as usize] = ' ' as i32 as libc::c_char;
                r = r.wrapping_add(1);
                g_buf[r as usize] = ',' as i32 as libc::c_char;
                r = r.wrapping_add(1);
                g_buf[r as usize] = '\u{0}' as i32 as libc::c_char;
                r = r.wrapping_add(1);
            }
            printkeys(
                bookmark,
                g_buf
                    .as_mut_ptr()
                    .offset(r as isize)
                    .offset(-(1 as libc::c_int as isize)),
                maxbm,
            );
            printmsg(g_buf.as_mut_ptr() as *const libc::c_char);
            fd = get_input(0 as *mut libc::c_void as *const libc::c_char);
        }
        _ => {}
    }
    r = 0 as libc::c_int as size_t;
    if fd == 44 as libc::c_int {
        if !bmark.is_null() {
            xstrsncpy(newpath, bmark, 4096 as libc::c_int as size_t);
        } else {
            r = 27 as libc::c_int as size_t;
        }
    } else if fd == 13 as libc::c_int {
        mkpath(cfgpath as *const libc::c_char, toks[0 as libc::c_int as usize], newpath);
        g_state.set_selbm(1 as libc::c_int as uint_t);
    } else {
        tmp = get_kv_val(bookmark, newpath, fd, maxbm, 1 as libc::c_int as uchar_t);
        if tmp.is_null() {
            r = 40 as libc::c_int as size_t;
        }
    }
    if r == 0 {
        tmp___0 = chdir(newpath as *const libc::c_char);
        if tmp___0 == -(1 as libc::c_int) {
            r = 24 as libc::c_int as size_t;
            if g_state.selbm() != 0 {
                g_state.set_selbm(0 as libc::c_int as uint_t);
            }
        }
    }
    return r;
}
unsafe extern "C" fn add_bookmark(
    mut path: *mut libc::c_char,
    mut newpath: *mut libc::c_char,
    mut presel: *mut libc::c_int,
) {
    let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: size_t = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: libc::c_int = 0;
    tmp = xbasename(path);
    dir = tmp;
    if *dir.offset(0 as libc::c_int as isize) != 0 {
        tmp___0 = dir;
    } else {
        tmp___0 = 0 as *mut libc::c_void as *mut libc::c_char;
    }
    dir = xreadline(
        tmp___0 as *const libc::c_char,
        messages[43 as libc::c_int as usize],
    );
    if !dir.is_null() {
        if *dir != 0 {
            tmp___1 = mkpath(
                cfgpath as *const libc::c_char,
                toks[0 as libc::c_int as usize],
                newpath,
            );
            r = tmp___1;
            *newpath
                .offset(
                    r.wrapping_sub(1 as libc::c_ulong) as isize,
                ) = '/' as i32 as libc::c_char;
            xstrsncpy(
                newpath.offset(r as isize),
                dir as *const libc::c_char,
                (4096 as libc::c_ulong).wrapping_sub(r),
            );
            tmp___6 = symlink(
                path as *const libc::c_char,
                newpath as *const libc::c_char,
            );
            if tmp___6 == -(1 as libc::c_int) {
                tmp___3 = __errno_location();
                tmp___4 = strerror(*tmp___3);
                tmp___5 = tmp___4;
            } else {
                tmp___5 = newpath;
            }
            printwait(tmp___5 as *const libc::c_char, presel);
        } else {
            printwait(messages[4 as libc::c_int as usize], presel);
        }
    } else {
        printwait(messages[4 as libc::c_int as usize], presel);
    };
}
unsafe extern "C" fn show_help(mut path: *const libc::c_char) {
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut helpstr: [libc::c_char; 1184] = [0; 1184];
    let mut fd: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut prog: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: uchar_t = 0;
    let mut i: uchar_t = 0;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: size_t = 0;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: size_t = 0;
    let mut tmp___7: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i___0: uchar_t = 0;
    let mut tmp___8: *mut libc::c_char = 0 as *mut libc::c_char;
    helpstr[0 as libc::c_int as usize] = '0' as i32 as libc::c_char;
    helpstr[1 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[2 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[3 as libc::c_int as usize] = 'N' as i32 as libc::c_char;
    helpstr[4 as libc::c_int as usize] = 'A' as i32 as libc::c_char;
    helpstr[5 as libc::c_int as usize] = 'V' as i32 as libc::c_char;
    helpstr[6 as libc::c_int as usize] = 'I' as i32 as libc::c_char;
    helpstr[7 as libc::c_int as usize] = 'G' as i32 as libc::c_char;
    helpstr[8 as libc::c_int as usize] = 'A' as i32 as libc::c_char;
    helpstr[9 as libc::c_int as usize] = 'T' as i32 as libc::c_char;
    helpstr[10 as libc::c_int as usize] = 'I' as i32 as libc::c_char;
    helpstr[11 as libc::c_int as usize] = 'O' as i32 as libc::c_char;
    helpstr[12 as libc::c_int as usize] = 'N' as i32 as libc::c_char;
    helpstr[13 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[14 as libc::c_int as usize] = '9' as i32 as libc::c_char;
    helpstr[15 as libc::c_int as usize] = 'U' as i32 as libc::c_char;
    helpstr[16 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    helpstr[17 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[18 as libc::c_int as usize] = 'k' as i32 as libc::c_char;
    helpstr[19 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[20 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[21 as libc::c_int as usize] = 'U' as i32 as libc::c_char;
    helpstr[22 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    helpstr[23 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[24 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[25 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[26 as libc::c_int as usize] = '6' as i32 as libc::c_char;
    helpstr[27 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[28 as libc::c_int as usize] = 'P' as i32 as libc::c_char;
    helpstr[29 as libc::c_int as usize] = 'g' as i32 as libc::c_char;
    helpstr[30 as libc::c_int as usize] = 'U' as i32 as libc::c_char;
    helpstr[31 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    helpstr[32 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[33 as libc::c_int as usize] = '^' as i32 as libc::c_char;
    helpstr[34 as libc::c_int as usize] = 'U' as i32 as libc::c_char;
    helpstr[35 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[36 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[37 as libc::c_int as usize] = 'P' as i32 as libc::c_char;
    helpstr[38 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[39 as libc::c_int as usize] = 'g' as i32 as libc::c_char;
    helpstr[40 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[41 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[42 as libc::c_int as usize] = 'u' as i32 as libc::c_char;
    helpstr[43 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    helpstr[44 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[45 as libc::c_int as usize] = '9' as i32 as libc::c_char;
    helpstr[46 as libc::c_int as usize] = 'D' as i32 as libc::c_char;
    helpstr[47 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[48 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[49 as libc::c_int as usize] = 'j' as i32 as libc::c_char;
    helpstr[50 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[51 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[52 as libc::c_int as usize] = 'D' as i32 as libc::c_char;
    helpstr[53 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[54 as libc::c_int as usize] = 'w' as i32 as libc::c_char;
    helpstr[55 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[56 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[57 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[58 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[59 as libc::c_int as usize] = '4' as i32 as libc::c_char;
    helpstr[60 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[61 as libc::c_int as usize] = 'P' as i32 as libc::c_char;
    helpstr[62 as libc::c_int as usize] = 'g' as i32 as libc::c_char;
    helpstr[63 as libc::c_int as usize] = 'D' as i32 as libc::c_char;
    helpstr[64 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[65 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[66 as libc::c_int as usize] = '^' as i32 as libc::c_char;
    helpstr[67 as libc::c_int as usize] = 'D' as i32 as libc::c_char;
    helpstr[68 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[69 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[70 as libc::c_int as usize] = 'P' as i32 as libc::c_char;
    helpstr[71 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[72 as libc::c_int as usize] = 'g' as i32 as libc::c_char;
    helpstr[73 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[74 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[75 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
    helpstr[76 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[77 as libc::c_int as usize] = 'w' as i32 as libc::c_char;
    helpstr[78 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[79 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[80 as libc::c_int as usize] = '9' as i32 as libc::c_char;
    helpstr[81 as libc::c_int as usize] = 'L' as i32 as libc::c_char;
    helpstr[82 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[83 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[84 as libc::c_int as usize] = 'h' as i32 as libc::c_char;
    helpstr[85 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[86 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[87 as libc::c_int as usize] = 'P' as i32 as libc::c_char;
    helpstr[88 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[89 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[90 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[91 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[92 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[93 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[94 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[95 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[96 as libc::c_int as usize] = '2' as i32 as libc::c_char;
    helpstr[97 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[98 as libc::c_int as usize] = '~' as i32 as libc::c_char;
    helpstr[99 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[100 as libc::c_int as usize] = '`' as i32 as libc::c_char;
    helpstr[101 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[102 as libc::c_int as usize] = '@' as i32 as libc::c_char;
    helpstr[103 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[104 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[105 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[106 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[107 as libc::c_int as usize] = '~' as i32 as libc::c_char;
    helpstr[108 as libc::c_int as usize] = ',' as i32 as libc::c_char;
    helpstr[109 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[110 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    helpstr[111 as libc::c_int as usize] = ',' as i32 as libc::c_char;
    helpstr[112 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[113 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    helpstr[114 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[115 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[116 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[117 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[118 as libc::c_int as usize] = ',' as i32 as libc::c_char;
    helpstr[119 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[120 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    helpstr[121 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[122 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[123 as libc::c_int as usize] = 'v' as i32 as libc::c_char;
    helpstr[124 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[125 as libc::c_int as usize] = '5' as i32 as libc::c_char;
    helpstr[126 as libc::c_int as usize] = 'R' as i32 as libc::c_char;
    helpstr[127 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[128 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[129 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[130 as libc::c_int as usize] = 'R' as i32 as libc::c_char;
    helpstr[131 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[132 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[133 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[134 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[135 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[136 as libc::c_int as usize] = 'O' as i32 as libc::c_char;
    helpstr[137 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    helpstr[138 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[139 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[140 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[141 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[142 as libc::c_int as usize] = '2' as i32 as libc::c_char;
    helpstr[143 as libc::c_int as usize] = '0' as i32 as libc::c_char;
    helpstr[144 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[145 as libc::c_int as usize] = '\'' as i32 as libc::c_char;
    helpstr[146 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[147 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[148 as libc::c_int as usize] = 'F' as i32 as libc::c_char;
    helpstr[149 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    helpstr[150 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[151 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    helpstr[152 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[153 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[154 as libc::c_int as usize] = 'f' as i32 as libc::c_char;
    helpstr[155 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    helpstr[156 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[157 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[158 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    helpstr[159 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    helpstr[160 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[161 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[162 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[163 as libc::c_int as usize] = 'h' as i32 as libc::c_char;
    helpstr[164 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[165 as libc::c_int as usize] = '9' as i32 as libc::c_char;
    helpstr[166 as libc::c_int as usize] = 'g' as i32 as libc::c_char;
    helpstr[167 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[168 as libc::c_int as usize] = '^' as i32 as libc::c_char;
    helpstr[169 as libc::c_int as usize] = 'A' as i32 as libc::c_char;
    helpstr[170 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[171 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[172 as libc::c_int as usize] = 'T' as i32 as libc::c_char;
    helpstr[173 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[174 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    helpstr[175 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[176 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[177 as libc::c_int as usize] = '2' as i32 as libc::c_char;
    helpstr[178 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[179 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[180 as libc::c_int as usize] = 'J' as i32 as libc::c_char;
    helpstr[181 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[182 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[183 as libc::c_int as usize] = 'J' as i32 as libc::c_char;
    helpstr[184 as libc::c_int as usize] = 'u' as i32 as libc::c_char;
    helpstr[185 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    helpstr[186 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    helpstr[187 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[188 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[189 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[190 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[191 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[192 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[193 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[194 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[195 as libc::c_int as usize] = 'y' as i32 as libc::c_char;
    helpstr[196 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    helpstr[197 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[198 as libc::c_int as usize] = 'f' as i32 as libc::c_char;
    helpstr[199 as libc::c_int as usize] = 'f' as i32 as libc::c_char;
    helpstr[200 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    helpstr[201 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[202 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[203 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[204 as libc::c_int as usize] = '9' as i32 as libc::c_char;
    helpstr[205 as libc::c_int as usize] = 'G' as i32 as libc::c_char;
    helpstr[206 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[207 as libc::c_int as usize] = '^' as i32 as libc::c_char;
    helpstr[208 as libc::c_int as usize] = 'E' as i32 as libc::c_char;
    helpstr[209 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[210 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[211 as libc::c_int as usize] = 'E' as i32 as libc::c_char;
    helpstr[212 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[213 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
    helpstr[214 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[215 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[216 as libc::c_int as usize] = '2' as i32 as libc::c_char;
    helpstr[217 as libc::c_int as usize] = '0' as i32 as libc::c_char;
    helpstr[218 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[219 as libc::c_int as usize] = '^' as i32 as libc::c_char;
    helpstr[220 as libc::c_int as usize] = 'J' as i32 as libc::c_char;
    helpstr[221 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[222 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[223 as libc::c_int as usize] = 'T' as i32 as libc::c_char;
    helpstr[224 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[225 as libc::c_int as usize] = 'g' as i32 as libc::c_char;
    helpstr[226 as libc::c_int as usize] = 'g' as i32 as libc::c_char;
    helpstr[227 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[228 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[229 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[230 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[231 as libc::c_int as usize] = 'u' as i32 as libc::c_char;
    helpstr[232 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[233 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[234 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[235 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[236 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
    helpstr[237 as libc::c_int as usize] = 'v' as i32 as libc::c_char;
    helpstr[238 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[239 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[240 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[241 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[242 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[243 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[244 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[245 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[246 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[247 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    helpstr[248 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[249 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[250 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[251 as libc::c_int as usize] = '8' as i32 as libc::c_char;
    helpstr[252 as libc::c_int as usize] = 'B' as i32 as libc::c_char;
    helpstr[253 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[254 as libc::c_int as usize] = '(' as i32 as libc::c_char;
    helpstr[255 as libc::c_int as usize] = ',' as i32 as libc::c_char;
    helpstr[256 as libc::c_int as usize] = ')' as i32 as libc::c_char;
    helpstr[257 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[258 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[259 as libc::c_int as usize] = 'B' as i32 as libc::c_char;
    helpstr[260 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[261 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[262 as libc::c_int as usize] = 'k' as i32 as libc::c_char;
    helpstr[263 as libc::c_int as usize] = '(' as i32 as libc::c_char;
    helpstr[264 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    helpstr[265 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[266 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[267 as libc::c_int as usize] = 'k' as i32 as libc::c_char;
    helpstr[268 as libc::c_int as usize] = ')' as i32 as libc::c_char;
    helpstr[269 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[270 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[271 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[272 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[273 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[274 as libc::c_int as usize] = 'b' as i32 as libc::c_char;
    helpstr[275 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[276 as libc::c_int as usize] = '^' as i32 as libc::c_char;
    helpstr[277 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    helpstr[278 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[279 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[280 as libc::c_int as usize] = 'S' as i32 as libc::c_char;
    helpstr[281 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[282 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[283 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[284 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[285 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[286 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[287 as libc::c_int as usize] = 'b' as i32 as libc::c_char;
    helpstr[288 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[289 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[290 as libc::c_int as usize] = 'k' as i32 as libc::c_char;
    helpstr[291 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    helpstr[292 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[293 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[294 as libc::c_int as usize] = 'k' as i32 as libc::c_char;
    helpstr[295 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[296 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[297 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[298 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[299 as libc::c_int as usize] = '4' as i32 as libc::c_char;
    helpstr[300 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[301 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[302 as libc::c_int as usize] = 'C' as i32 as libc::c_char;
    helpstr[303 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[304 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[305 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[306 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[307 as libc::c_int as usize] = 'x' as i32 as libc::c_char;
    helpstr[308 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[309 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[310 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[311 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[312 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[313 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[314 as libc::c_int as usize] = '(' as i32 as libc::c_char;
    helpstr[315 as libc::c_int as usize] = 'S' as i32 as libc::c_char;
    helpstr[316 as libc::c_int as usize] = 'h' as i32 as libc::c_char;
    helpstr[317 as libc::c_int as usize] = ')' as i32 as libc::c_char;
    helpstr[318 as libc::c_int as usize] = 'T' as i32 as libc::c_char;
    helpstr[319 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[320 as libc::c_int as usize] = 'b' as i32 as libc::c_char;
    helpstr[321 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[322 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[323 as libc::c_int as usize] = 'C' as i32 as libc::c_char;
    helpstr[324 as libc::c_int as usize] = 'y' as i32 as libc::c_char;
    helpstr[325 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[326 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[327 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[328 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    helpstr[329 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[330 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[331 as libc::c_int as usize] = 'w' as i32 as libc::c_char;
    helpstr[332 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[333 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[334 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[335 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[336 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[337 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[338 as libc::c_int as usize] = 'x' as i32 as libc::c_char;
    helpstr[339 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[340 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[341 as libc::c_int as usize] = '6' as i32 as libc::c_char;
    helpstr[342 as libc::c_int as usize] = '2' as i32 as libc::c_char;
    helpstr[343 as libc::c_int as usize] = 'E' as i32 as libc::c_char;
    helpstr[344 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    helpstr[345 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[346 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[347 as libc::c_int as usize] = '^' as i32 as libc::c_char;
    helpstr[348 as libc::c_int as usize] = 'Q' as i32 as libc::c_char;
    helpstr[349 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[350 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[351 as libc::c_int as usize] = 'Q' as i32 as libc::c_char;
    helpstr[352 as libc::c_int as usize] = 'u' as i32 as libc::c_char;
    helpstr[353 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    helpstr[354 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[355 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[356 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[357 as libc::c_int as usize] = '2' as i32 as libc::c_char;
    helpstr[358 as libc::c_int as usize] = '0' as i32 as libc::c_char;
    helpstr[359 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[360 as libc::c_int as usize] = 'q' as i32 as libc::c_char;
    helpstr[361 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[362 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[363 as libc::c_int as usize] = 'Q' as i32 as libc::c_char;
    helpstr[364 as libc::c_int as usize] = 'u' as i32 as libc::c_char;
    helpstr[365 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    helpstr[366 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[367 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[368 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[369 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[370 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[371 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[372 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[373 as libc::c_int as usize] = 'x' as i32 as libc::c_char;
    helpstr[374 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[375 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[376 as libc::c_int as usize] = 'b' as i32 as libc::c_char;
    helpstr[377 as libc::c_int as usize] = '^' as i32 as libc::c_char;
    helpstr[378 as libc::c_int as usize] = 'G' as i32 as libc::c_char;
    helpstr[379 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[380 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[381 as libc::c_int as usize] = 'Q' as i32 as libc::c_char;
    helpstr[382 as libc::c_int as usize] = 'u' as i32 as libc::c_char;
    helpstr[383 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    helpstr[384 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[385 as libc::c_int as usize] = 'C' as i32 as libc::c_char;
    helpstr[386 as libc::c_int as usize] = 'D' as i32 as libc::c_char;
    helpstr[387 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[388 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[389 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[390 as libc::c_int as usize] = '8' as i32 as libc::c_char;
    helpstr[391 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[392 as libc::c_int as usize] = 'Q' as i32 as libc::c_char;
    helpstr[393 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[394 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[395 as libc::c_int as usize] = 'P' as i32 as libc::c_char;
    helpstr[396 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    helpstr[397 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[398 as libc::c_int as usize] = 'k' as i32 as libc::c_char;
    helpstr[399 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    helpstr[400 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[401 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[402 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[403 as libc::c_int as usize] = ',' as i32 as libc::c_char;
    helpstr[404 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[405 as libc::c_int as usize] = 'q' as i32 as libc::c_char;
    helpstr[406 as libc::c_int as usize] = 'u' as i32 as libc::c_char;
    helpstr[407 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    helpstr[408 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[409 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[410 as libc::c_int as usize] = '0' as i32 as libc::c_char;
    helpstr[411 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[412 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[413 as libc::c_int as usize] = 'F' as i32 as libc::c_char;
    helpstr[414 as libc::c_int as usize] = 'I' as i32 as libc::c_char;
    helpstr[415 as libc::c_int as usize] = 'L' as i32 as libc::c_char;
    helpstr[416 as libc::c_int as usize] = 'T' as i32 as libc::c_char;
    helpstr[417 as libc::c_int as usize] = 'E' as i32 as libc::c_char;
    helpstr[418 as libc::c_int as usize] = 'R' as i32 as libc::c_char;
    helpstr[419 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[420 as libc::c_int as usize] = '&' as i32 as libc::c_char;
    helpstr[421 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[422 as libc::c_int as usize] = 'P' as i32 as libc::c_char;
    helpstr[423 as libc::c_int as usize] = 'R' as i32 as libc::c_char;
    helpstr[424 as libc::c_int as usize] = 'O' as i32 as libc::c_char;
    helpstr[425 as libc::c_int as usize] = 'M' as i32 as libc::c_char;
    helpstr[426 as libc::c_int as usize] = 'P' as i32 as libc::c_char;
    helpstr[427 as libc::c_int as usize] = 'T' as i32 as libc::c_char;
    helpstr[428 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[429 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[430 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    helpstr[431 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[432 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[433 as libc::c_int as usize] = 'F' as i32 as libc::c_char;
    helpstr[434 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    helpstr[435 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[436 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[437 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[438 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[439 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[440 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[441 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[442 as libc::c_int as usize] = '7' as i32 as libc::c_char;
    helpstr[443 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[444 as libc::c_int as usize] = '^' as i32 as libc::c_char;
    helpstr[445 as libc::c_int as usize] = 'N' as i32 as libc::c_char;
    helpstr[446 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[447 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[448 as libc::c_int as usize] = 'T' as i32 as libc::c_char;
    helpstr[449 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[450 as libc::c_int as usize] = 'g' as i32 as libc::c_char;
    helpstr[451 as libc::c_int as usize] = 'g' as i32 as libc::c_char;
    helpstr[452 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[453 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[454 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[455 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[456 as libc::c_int as usize] = 'y' as i32 as libc::c_char;
    helpstr[457 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    helpstr[458 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[459 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[460 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[461 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[462 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[463 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[464 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[465 as libc::c_int as usize] = 'v' as i32 as libc::c_char;
    helpstr[466 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[467 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[468 as libc::c_int as usize] = 'E' as i32 as libc::c_char;
    helpstr[469 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    helpstr[470 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[471 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[472 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[473 as libc::c_int as usize] = 'E' as i32 as libc::c_char;
    helpstr[474 as libc::c_int as usize] = 'x' as i32 as libc::c_char;
    helpstr[475 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    helpstr[476 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[477 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[478 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    helpstr[479 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[480 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[481 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    helpstr[482 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    helpstr[483 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[484 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[485 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[486 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[487 as libc::c_int as usize] = '2' as i32 as libc::c_char;
    helpstr[488 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[489 as libc::c_int as usize] = '^' as i32 as libc::c_char;
    helpstr[490 as libc::c_int as usize] = 'L' as i32 as libc::c_char;
    helpstr[491 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[492 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[493 as libc::c_int as usize] = 'T' as i32 as libc::c_char;
    helpstr[494 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[495 as libc::c_int as usize] = 'g' as i32 as libc::c_char;
    helpstr[496 as libc::c_int as usize] = 'g' as i32 as libc::c_char;
    helpstr[497 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[498 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[499 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[500 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[501 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[502 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    helpstr[503 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[504 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[505 as libc::c_int as usize] = 'f' as i32 as libc::c_char;
    helpstr[506 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    helpstr[507 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[508 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[509 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[510 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[511 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[512 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[513 as libc::c_int as usize] = '.' as i32 as libc::c_char;
    helpstr[514 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[515 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[516 as libc::c_int as usize] = 'T' as i32 as libc::c_char;
    helpstr[517 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[518 as libc::c_int as usize] = 'g' as i32 as libc::c_char;
    helpstr[519 as libc::c_int as usize] = 'g' as i32 as libc::c_char;
    helpstr[520 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[521 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[522 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[523 as libc::c_int as usize] = 'h' as i32 as libc::c_char;
    helpstr[524 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    helpstr[525 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
    helpstr[526 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
    helpstr[527 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[528 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[529 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[530 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[531 as libc::c_int as usize] = '5' as i32 as libc::c_char;
    helpstr[532 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[533 as libc::c_int as usize] = 'A' as i32 as libc::c_char;
    helpstr[534 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[535 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[536 as libc::c_int as usize] = '+' as i32 as libc::c_char;
    helpstr[537 as libc::c_int as usize] = 'E' as i32 as libc::c_char;
    helpstr[538 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    helpstr[539 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[540 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[541 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[542 as libc::c_int as usize] = 'U' as i32 as libc::c_char;
    helpstr[543 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[544 as libc::c_int as usize] = 'f' as i32 as libc::c_char;
    helpstr[545 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    helpstr[546 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[547 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[548 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[549 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[550 as libc::c_int as usize] = ',' as i32 as libc::c_char;
    helpstr[551 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[552 as libc::c_int as usize] = 'q' as i32 as libc::c_char;
    helpstr[553 as libc::c_int as usize] = 'u' as i32 as libc::c_char;
    helpstr[554 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    helpstr[555 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[556 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[557 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[558 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[559 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[560 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[561 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[562 as libc::c_int as usize] = 'x' as i32 as libc::c_char;
    helpstr[563 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[564 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[565 as libc::c_int as usize] = '0' as i32 as libc::c_char;
    helpstr[566 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[567 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[568 as libc::c_int as usize] = 'F' as i32 as libc::c_char;
    helpstr[569 as libc::c_int as usize] = 'I' as i32 as libc::c_char;
    helpstr[570 as libc::c_int as usize] = 'L' as i32 as libc::c_char;
    helpstr[571 as libc::c_int as usize] = 'E' as i32 as libc::c_char;
    helpstr[572 as libc::c_int as usize] = 'S' as i32 as libc::c_char;
    helpstr[573 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[574 as libc::c_int as usize] = '9' as i32 as libc::c_char;
    helpstr[575 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[576 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[577 as libc::c_int as usize] = '^' as i32 as libc::c_char;
    helpstr[578 as libc::c_int as usize] = 'O' as i32 as libc::c_char;
    helpstr[579 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[580 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[581 as libc::c_int as usize] = 'O' as i32 as libc::c_char;
    helpstr[582 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    helpstr[583 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[584 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[585 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[586 as libc::c_int as usize] = 'w' as i32 as libc::c_char;
    helpstr[587 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    helpstr[588 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[589 as libc::c_int as usize] = 'h' as i32 as libc::c_char;
    helpstr[590 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[591 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[592 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[593 as libc::c_int as usize] = '5' as i32 as libc::c_char;
    helpstr[594 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[595 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[596 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[597 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[598 as libc::c_int as usize] = 'C' as i32 as libc::c_char;
    helpstr[599 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[600 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[601 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[602 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[603 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[604 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[605 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[606 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[607 as libc::c_int as usize] = 'w' as i32 as libc::c_char;
    helpstr[608 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    helpstr[609 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[610 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    helpstr[611 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[612 as libc::c_int as usize] = 'k' as i32 as libc::c_char;
    helpstr[613 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[614 as libc::c_int as usize] = '9' as i32 as libc::c_char;
    helpstr[615 as libc::c_int as usize] = 'f' as i32 as libc::c_char;
    helpstr[616 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[617 as libc::c_int as usize] = '^' as i32 as libc::c_char;
    helpstr[618 as libc::c_int as usize] = 'F' as i32 as libc::c_char;
    helpstr[619 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[620 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[621 as libc::c_int as usize] = 'F' as i32 as libc::c_char;
    helpstr[622 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    helpstr[623 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[624 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[625 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[626 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    helpstr[627 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[628 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[629 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[630 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    helpstr[631 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[632 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[633 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[634 as libc::c_int as usize] = '4' as i32 as libc::c_char;
    helpstr[635 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[636 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
    helpstr[637 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[638 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[639 as libc::c_int as usize] = 'D' as i32 as libc::c_char;
    helpstr[640 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[641 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[642 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[643 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    helpstr[644 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[645 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[646 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    helpstr[647 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[648 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
    helpstr[649 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[650 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[651 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[652 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[653 as libc::c_int as usize] = 'g' as i32 as libc::c_char;
    helpstr[654 as libc::c_int as usize] = 'g' as i32 as libc::c_char;
    helpstr[655 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[656 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[657 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[658 as libc::c_int as usize] = 'b' as i32 as libc::c_char;
    helpstr[659 as libc::c_int as usize] = '^' as i32 as libc::c_char;
    helpstr[660 as libc::c_int as usize] = 'R' as i32 as libc::c_char;
    helpstr[661 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[662 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[663 as libc::c_int as usize] = 'R' as i32 as libc::c_char;
    helpstr[664 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[665 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[666 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[667 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    helpstr[668 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[669 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    helpstr[670 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
    helpstr[671 as libc::c_int as usize] = 'u' as i32 as libc::c_char;
    helpstr[672 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    helpstr[673 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[674 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[675 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[676 as libc::c_int as usize] = '4' as i32 as libc::c_char;
    helpstr[677 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[678 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[679 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[680 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[681 as libc::c_int as usize] = 'B' as i32 as libc::c_char;
    helpstr[682 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[683 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[684 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[685 as libc::c_int as usize] = 'h' as i32 as libc::c_char;
    helpstr[686 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[687 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[688 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[689 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[690 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[691 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    helpstr[692 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[693 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[694 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[695 as libc::c_int as usize] = 'z' as i32 as libc::c_char;
    helpstr[696 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[697 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[698 as libc::c_int as usize] = 'A' as i32 as libc::c_char;
    helpstr[699 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[700 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[701 as libc::c_int as usize] = 'h' as i32 as libc::c_char;
    helpstr[702 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    helpstr[703 as libc::c_int as usize] = 'v' as i32 as libc::c_char;
    helpstr[704 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[705 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[706 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[707 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[708 as libc::c_int as usize] = '7' as i32 as libc::c_char;
    helpstr[709 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[710 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[711 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[712 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[713 as libc::c_int as usize] = 'E' as i32 as libc::c_char;
    helpstr[714 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
    helpstr[715 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    helpstr[716 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[717 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[718 as libc::c_int as usize] = 'f' as i32 as libc::c_char;
    helpstr[719 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    helpstr[720 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[721 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[722 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[723 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[724 as libc::c_int as usize] = '*' as i32 as libc::c_char;
    helpstr[725 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[726 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[727 as libc::c_int as usize] = 'T' as i32 as libc::c_char;
    helpstr[728 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[729 as libc::c_int as usize] = 'g' as i32 as libc::c_char;
    helpstr[730 as libc::c_int as usize] = 'g' as i32 as libc::c_char;
    helpstr[731 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[732 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[733 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[734 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[735 as libc::c_int as usize] = 'x' as i32 as libc::c_char;
    helpstr[736 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[737 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[738 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[739 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[740 as libc::c_int as usize] = '4' as i32 as libc::c_char;
    helpstr[741 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[742 as libc::c_int as usize] = '>' as i32 as libc::c_char;
    helpstr[743 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[744 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[745 as libc::c_int as usize] = 'E' as i32 as libc::c_char;
    helpstr[746 as libc::c_int as usize] = 'x' as i32 as libc::c_char;
    helpstr[747 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    helpstr[748 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[749 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[750 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[751 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[752 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[753 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    helpstr[754 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    helpstr[755 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[756 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[757 as libc::c_int as usize] = '6' as i32 as libc::c_char;
    helpstr[758 as libc::c_int as usize] = 'S' as i32 as libc::c_char;
    helpstr[759 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    helpstr[760 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[761 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[762 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[763 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[764 as libc::c_int as usize] = '+' as i32 as libc::c_char;
    helpstr[765 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[766 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[767 as libc::c_int as usize] = '(' as i32 as libc::c_char;
    helpstr[768 as libc::c_int as usize] = 'U' as i32 as libc::c_char;
    helpstr[769 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[770 as libc::c_int as usize] = ')' as i32 as libc::c_char;
    helpstr[771 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    helpstr[772 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[773 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[774 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[775 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[776 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[777 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[778 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[779 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[780 as libc::c_int as usize] = '2' as i32 as libc::c_char;
    helpstr[781 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[782 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    helpstr[783 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[784 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    helpstr[785 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[786 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[787 as libc::c_int as usize] = 'S' as i32 as libc::c_char;
    helpstr[788 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[789 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[790 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[791 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[792 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[793 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[794 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[795 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[796 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[797 as libc::c_int as usize] = 'g' as i32 as libc::c_char;
    helpstr[798 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[799 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    helpstr[800 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[801 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[802 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[803 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[804 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[805 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[806 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[807 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[808 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[809 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[810 as libc::c_int as usize] = 'S' as i32 as libc::c_char;
    helpstr[811 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[812 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[813 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[814 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[815 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[816 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[817 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[818 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[819 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[820 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[821 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[822 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[823 as libc::c_int as usize] = '4' as i32 as libc::c_char;
    helpstr[824 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[825 as libc::c_int as usize] = 'A' as i32 as libc::c_char;
    helpstr[826 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[827 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[828 as libc::c_int as usize] = 'I' as i32 as libc::c_char;
    helpstr[829 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[830 as libc::c_int as usize] = 'v' as i32 as libc::c_char;
    helpstr[831 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[832 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[833 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[834 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[835 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    helpstr[836 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[837 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[838 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[839 as libc::c_int as usize] = '9' as i32 as libc::c_char;
    helpstr[840 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    helpstr[841 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[842 as libc::c_int as usize] = '^' as i32 as libc::c_char;
    helpstr[843 as libc::c_int as usize] = 'P' as i32 as libc::c_char;
    helpstr[844 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[845 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[846 as libc::c_int as usize] = 'C' as i32 as libc::c_char;
    helpstr[847 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[848 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    helpstr[849 as libc::c_int as usize] = 'y' as i32 as libc::c_char;
    helpstr[850 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[851 as libc::c_int as usize] = 'h' as i32 as libc::c_char;
    helpstr[852 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[853 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[854 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[855 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[856 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[857 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[858 as libc::c_int as usize] = '2' as i32 as libc::c_char;
    helpstr[859 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[860 as libc::c_int as usize] = 'w' as i32 as libc::c_char;
    helpstr[861 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[862 as libc::c_int as usize] = '^' as i32 as libc::c_char;
    helpstr[863 as libc::c_int as usize] = 'W' as i32 as libc::c_char;
    helpstr[864 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[865 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[866 as libc::c_int as usize] = 'C' as i32 as libc::c_char;
    helpstr[867 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    helpstr[868 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    helpstr[869 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    helpstr[870 as libc::c_int as usize] = 'v' as i32 as libc::c_char;
    helpstr[871 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[872 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    helpstr[873 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[874 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[875 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[876 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[877 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    helpstr[878 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[879 as libc::c_int as usize] = '9' as i32 as libc::c_char;
    helpstr[880 as libc::c_int as usize] = 'v' as i32 as libc::c_char;
    helpstr[881 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[882 as libc::c_int as usize] = '^' as i32 as libc::c_char;
    helpstr[883 as libc::c_int as usize] = 'V' as i32 as libc::c_char;
    helpstr[884 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[885 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[886 as libc::c_int as usize] = 'M' as i32 as libc::c_char;
    helpstr[887 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[888 as libc::c_int as usize] = 'v' as i32 as libc::c_char;
    helpstr[889 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[890 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[891 as libc::c_int as usize] = 'h' as i32 as libc::c_char;
    helpstr[892 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[893 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[894 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[895 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[896 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[897 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[898 as libc::c_int as usize] = '5' as i32 as libc::c_char;
    helpstr[899 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[900 as libc::c_int as usize] = 'E' as i32 as libc::c_char;
    helpstr[901 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[902 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[903 as libc::c_int as usize] = 'E' as i32 as libc::c_char;
    helpstr[904 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
    helpstr[905 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    helpstr[906 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[907 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[908 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    helpstr[909 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[910 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[911 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[912 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[913 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    helpstr[914 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    helpstr[915 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[916 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[917 as libc::c_int as usize] = '9' as i32 as libc::c_char;
    helpstr[918 as libc::c_int as usize] = 'x' as i32 as libc::c_char;
    helpstr[919 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[920 as libc::c_int as usize] = '^' as i32 as libc::c_char;
    helpstr[921 as libc::c_int as usize] = 'X' as i32 as libc::c_char;
    helpstr[922 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[923 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[924 as libc::c_int as usize] = 'D' as i32 as libc::c_char;
    helpstr[925 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[926 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[927 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[928 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[929 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[930 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[931 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[932 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[933 as libc::c_int as usize] = '6' as i32 as libc::c_char;
    helpstr[934 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[935 as libc::c_int as usize] = 'E' as i32 as libc::c_char;
    helpstr[936 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    helpstr[937 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[938 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[939 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[940 as libc::c_int as usize] = 'S' as i32 as libc::c_char;
    helpstr[941 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[942 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[943 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
    helpstr[944 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[945 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[946 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[947 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[948 as libc::c_int as usize] = 'F' as i32 as libc::c_char;
    helpstr[949 as libc::c_int as usize] = 'I' as i32 as libc::c_char;
    helpstr[950 as libc::c_int as usize] = 'F' as i32 as libc::c_char;
    helpstr[951 as libc::c_int as usize] = 'O' as i32 as libc::c_char;
    helpstr[952 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[953 as libc::c_int as usize] = '0' as i32 as libc::c_char;
    helpstr[954 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[955 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[956 as libc::c_int as usize] = 'M' as i32 as libc::c_char;
    helpstr[957 as libc::c_int as usize] = 'I' as i32 as libc::c_char;
    helpstr[958 as libc::c_int as usize] = 'S' as i32 as libc::c_char;
    helpstr[959 as libc::c_int as usize] = 'C' as i32 as libc::c_char;
    helpstr[960 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[961 as libc::c_int as usize] = '8' as i32 as libc::c_char;
    helpstr[962 as libc::c_int as usize] = 'A' as i32 as libc::c_char;
    helpstr[963 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[964 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[965 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[966 as libc::c_int as usize] = ';' as i32 as libc::c_char;
    helpstr[967 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[968 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[969 as libc::c_int as usize] = 'S' as i32 as libc::c_char;
    helpstr[970 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[971 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[972 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[973 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[974 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[975 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[976 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    helpstr[977 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[978 as libc::c_int as usize] = 'u' as i32 as libc::c_char;
    helpstr[979 as libc::c_int as usize] = 'g' as i32 as libc::c_char;
    helpstr[980 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    helpstr[981 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[982 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[983 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[984 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[985 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[986 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[987 as libc::c_int as usize] = '=' as i32 as libc::c_char;
    helpstr[988 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[989 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[990 as libc::c_int as usize] = 'L' as i32 as libc::c_char;
    helpstr[991 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[992 as libc::c_int as usize] = 'u' as i32 as libc::c_char;
    helpstr[993 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[994 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[995 as libc::c_int as usize] = 'h' as i32 as libc::c_char;
    helpstr[996 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[997 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[998 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    helpstr[999 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    helpstr[1000 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[1001 as libc::c_int as usize] = '9' as i32 as libc::c_char;
    helpstr[1002 as libc::c_int as usize] = '!' as i32 as libc::c_char;
    helpstr[1003 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1004 as libc::c_int as usize] = '^' as i32 as libc::c_char;
    helpstr[1005 as libc::c_int as usize] = ']' as i32 as libc::c_char;
    helpstr[1006 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1007 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1008 as libc::c_int as usize] = 'S' as i32 as libc::c_char;
    helpstr[1009 as libc::c_int as usize] = 'h' as i32 as libc::c_char;
    helpstr[1010 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[1011 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[1012 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[1013 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[1014 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[1015 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[1016 as libc::c_int as usize] = '9' as i32 as libc::c_char;
    helpstr[1017 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[1018 as libc::c_int as usize] = ']' as i32 as libc::c_char;
    helpstr[1019 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1020 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1021 as libc::c_int as usize] = 'C' as i32 as libc::c_char;
    helpstr[1022 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    helpstr[1023 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
    helpstr[1024 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1025 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    helpstr[1026 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[1027 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[1028 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    helpstr[1029 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    helpstr[1030 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[1031 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[1032 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[1033 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[1034 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1035 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1036 as libc::c_int as usize] = 'C' as i32 as libc::c_char;
    helpstr[1037 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[1038 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[1039 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[1040 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[1041 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[1042 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[1043 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1044 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[1045 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[1046 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    helpstr[1047 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[1048 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[1049 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[1050 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[1051 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[1052 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[1053 as libc::c_int as usize] = '0' as i32 as libc::c_char;
    helpstr[1054 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[1055 as libc::c_int as usize] = 'u' as i32 as libc::c_char;
    helpstr[1056 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1057 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1058 as libc::c_int as usize] = 'U' as i32 as libc::c_char;
    helpstr[1059 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[1060 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    helpstr[1061 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[1062 as libc::c_int as usize] = 'u' as i32 as libc::c_char;
    helpstr[1063 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[1064 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[1065 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1066 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[1067 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[1068 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    helpstr[1069 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[1070 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[1071 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[1072 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    helpstr[1073 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[1074 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[1075 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[1076 as libc::c_int as usize] = 'h' as i32 as libc::c_char;
    helpstr[1077 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    helpstr[1078 as libc::c_int as usize] = 'v' as i32 as libc::c_char;
    helpstr[1079 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[1080 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[1081 as libc::c_int as usize] = '9' as i32 as libc::c_char;
    helpstr[1082 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[1083 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1084 as libc::c_int as usize] = '^' as i32 as libc::c_char;
    helpstr[1085 as libc::c_int as usize] = 'T' as i32 as libc::c_char;
    helpstr[1086 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1087 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1088 as libc::c_int as usize] = 'S' as i32 as libc::c_char;
    helpstr[1089 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[1090 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[1091 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[1092 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1093 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[1094 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[1095 as libc::c_int as usize] = 'g' as i32 as libc::c_char;
    helpstr[1096 as libc::c_int as usize] = 'g' as i32 as libc::c_char;
    helpstr[1097 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[1098 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[1099 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    helpstr[1100 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[1101 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[1102 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[1103 as libc::c_int as usize] = '2' as i32 as libc::c_char;
    helpstr[1104 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[1105 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    helpstr[1106 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1107 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1108 as libc::c_int as usize] = 'M' as i32 as libc::c_char;
    helpstr[1109 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[1110 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[1111 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[1112 as libc::c_int as usize] = 'g' as i32 as libc::c_char;
    helpstr[1113 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[1114 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1115 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    helpstr[1116 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[1117 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    helpstr[1118 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    helpstr[1119 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    helpstr[1120 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[1121 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[1122 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[1123 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[1124 as libc::c_int as usize] = 'T' as i32 as libc::c_char;
    helpstr[1125 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1126 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1127 as libc::c_int as usize] = 'S' as i32 as libc::c_char;
    helpstr[1128 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[1129 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[1130 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1131 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[1132 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    helpstr[1133 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    helpstr[1134 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[1135 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1136 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    helpstr[1137 as libc::c_int as usize] = 'y' as i32 as libc::c_char;
    helpstr[1138 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    helpstr[1139 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[1140 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[1141 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[1142 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[1143 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[1144 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[1145 as libc::c_int as usize] = '0' as i32 as libc::c_char;
    helpstr[1146 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1147 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1148 as libc::c_int as usize] = 'L' as i32 as libc::c_char;
    helpstr[1149 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[1150 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[1151 as libc::c_int as usize] = 'k' as i32 as libc::c_char;
    helpstr[1152 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[1153 as libc::c_int as usize] = 'b' as i32 as libc::c_char;
    helpstr[1154 as libc::c_int as usize] = '^' as i32 as libc::c_char;
    helpstr[1155 as libc::c_int as usize] = 'L' as i32 as libc::c_char;
    helpstr[1156 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1157 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1158 as libc::c_int as usize] = 'R' as i32 as libc::c_char;
    helpstr[1159 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[1160 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
    helpstr[1161 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    helpstr[1162 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    helpstr[1163 as libc::c_int as usize] = 'w' as i32 as libc::c_char;
    helpstr[1164 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    helpstr[1165 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    helpstr[1166 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    helpstr[1167 as libc::c_int as usize] = '8' as i32 as libc::c_char;
    helpstr[1168 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[1169 as libc::c_int as usize] = '?' as i32 as libc::c_char;
    helpstr[1170 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1171 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1172 as libc::c_int as usize] = 'H' as i32 as libc::c_char;
    helpstr[1173 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    helpstr[1174 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    helpstr[1175 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    helpstr[1176 as libc::c_int as usize] = ',' as i32 as libc::c_char;
    helpstr[1177 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    helpstr[1178 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    helpstr[1179 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    helpstr[1180 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    helpstr[1181 as libc::c_int as usize] = 'f' as i32 as libc::c_char;
    helpstr[1182 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
    helpstr[1183 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    tmp = create_tmp_file();
    fd = tmp;
    if fd == -(1 as libc::c_int) {
        return;
    }
    dprintf(
        fd,
        b"  |V\\_\n  /. \\\\\n (;^; ||\n   /___3\n  (___n))\n\0" as *const u8
            as *const libc::c_char,
    );
    tmp___0 = xgetenv(
        env_cfg[12 as libc::c_int as usize],
        0 as *mut libc::c_void as *mut libc::c_char,
    );
    prog = tmp___0;
    if !prog.is_null() {
        get_output(
            prog,
            0 as *mut libc::c_void as *mut libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_char,
            fd,
            1 as libc::c_int != 0,
            0 as libc::c_int != 0,
        );
    }
    end = helpstr.as_mut_ptr() as *const libc::c_char;
    start = end;
    while *end != 0 {
        if *end as libc::c_int == 10 as libc::c_int {
            tmp___1 = xchartohex(*start as uchar_t);
            snprintf(
                g_buf.as_mut_ptr(),
                (4096 as libc::c_int + ((256 as libc::c_int) << 1 as libc::c_int))
                    as size_t,
                b"%*c%.*s\0" as *const u8 as *const libc::c_char,
                tmp___1 as libc::c_int,
                ' ' as i32,
                end.offset_from(start) as libc::c_long as libc::c_int,
                start.offset(1 as libc::c_int as isize),
            );
            dprintf(fd, g_buf.as_mut_ptr() as *const libc::c_char, ' ' as i32);
            start = end.offset(1 as libc::c_int as isize);
        }
        end = end.offset(1);
    }
    dprintf(fd, b"\nLOCATIONS\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as uchar_t;
    while (i as libc::c_int) < 4 as libc::c_int {
        if (g_ctx[i as usize].c_cfg).ctxactive() != 0 {
            dprintf(
                fd,
                b" %u: %s\n\0" as *const u8 as *const libc::c_char,
                i as libc::c_int + 1 as libc::c_int,
                (g_ctx[i as usize].c_path).as_mut_ptr(),
            );
        }
        i = (i as libc::c_int + 1 as libc::c_int) as uchar_t;
    }
    tmp___2 = get_fs_info(path, 0 as libc::c_int as uchar_t);
    tmp___3 = coolsize(tmp___2 as off_t);
    dprintf(fd, b"\nVOLUME: avail:%s \0" as *const u8 as *const libc::c_char, tmp___3);
    tmp___4 = get_fs_info(path, 1 as libc::c_int as uchar_t);
    tmp___5 = coolsize(tmp___4 as off_t);
    dprintf(fd, b"used:%s \0" as *const u8 as *const libc::c_char, tmp___5);
    tmp___6 = get_fs_info(path, 2 as libc::c_int as uchar_t);
    tmp___7 = coolsize(tmp___6 as off_t);
    dprintf(fd, b"size:%s\n\n\0" as *const u8 as *const libc::c_char, tmp___7);
    if !bookmark.is_null() {
        dprintf(fd, b"BOOKMARKS\n\0" as *const u8 as *const libc::c_char);
        printkv(bookmark, fd, maxbm, 1 as libc::c_int as uchar_t);
        dprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    }
    if !plug.is_null() {
        dprintf(fd, b"PLUGIN KEYS\n\0" as *const u8 as *const libc::c_char);
        printkv(plug, fd, maxplug, 2 as libc::c_int as uchar_t);
        dprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    }
    i___0 = 3 as libc::c_int as uchar_t;
    while i___0 as libc::c_int <= 13 as libc::c_int {
        tmp___8 = getenv(env_cfg[i___0 as usize]);
        start = tmp___8 as *const libc::c_char;
        if !start.is_null() {
            dprintf(
                fd,
                b"%s: %s\n\0" as *const u8 as *const libc::c_char,
                env_cfg[i___0 as usize],
                start,
            );
        }
        i___0 = (i___0 as libc::c_int + 1 as libc::c_int) as uchar_t;
    }
    if !selpath.is_null() {
        dprintf(
            fd,
            b"SELECTION FILE: %s\n\0" as *const u8 as *const libc::c_char,
            selpath,
        );
    }
    dprintf(
        fd,
        b"\nv%s\n%s\n\0" as *const u8 as *const libc::c_char,
        b"4.6\0" as *const u8 as *const libc::c_char,
        b"BSD 2-Clause\nhttps://github.com/jarun/nnn\0" as *const u8
            as *const libc::c_char,
    );
    close(fd);
    spawn(
        pager,
        g_tmpfpath.as_mut_ptr(),
        0 as *mut libc::c_void as *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
        265 as libc::c_int as ushort_t,
    );
    unlink(g_tmpfpath.as_mut_ptr() as *const libc::c_char);
}
unsafe extern "C" fn setexports() {
    let mut dvar: [libc::c_char; 3] = [0; 3];
    let mut fvar: [libc::c_char; 3] = [0; 3];
    let mut i: uchar_t = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    dvar[0 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
    dvar[1 as libc::c_int as usize] = '0' as i32 as libc::c_char;
    dvar[2 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    fvar[0 as libc::c_int as usize] = 'f' as i32 as libc::c_char;
    fvar[1 as libc::c_int as usize] = '0' as i32 as libc::c_char;
    fvar[2 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    if ndents != 0 {
        setenv(
            envs[4 as libc::c_int as usize],
            (*pdents.offset(cur as isize)).name as *const libc::c_char,
            1 as libc::c_int,
        );
        xstrsncpy(
            (g_ctx[cfg.curctx() as usize].c_name).as_mut_ptr(),
            (*pdents.offset(cur as isize)).name as *const libc::c_char,
            256 as libc::c_int as size_t,
        );
    } else if g_ctx[cfg.curctx() as usize].c_name[0 as libc::c_int as usize] != 0 {
        g_ctx[cfg.curctx() as usize]
            .c_name[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    }
    i = 0 as libc::c_int as uchar_t;
    while (i as libc::c_int) < 4 as libc::c_int {
        if (g_ctx[i as usize].c_cfg).ctxactive() != 0 {
            fvar[1 as libc::c_int
                as usize] = (49 as libc::c_int + i as libc::c_int) as libc::c_char;
            dvar[1 as libc::c_int as usize] = fvar[1 as libc::c_int as usize];
            setenv(
                dvar.as_mut_ptr() as *const libc::c_char,
                (g_ctx[i as usize].c_path).as_mut_ptr() as *const libc::c_char,
                1 as libc::c_int,
            );
            if g_ctx[i as usize].c_name[0 as libc::c_int as usize] != 0 {
                mkpath(
                    (g_ctx[i as usize].c_path).as_mut_ptr() as *const libc::c_char,
                    (g_ctx[i as usize].c_name).as_mut_ptr() as *const libc::c_char,
                    g_buf.as_mut_ptr(),
                );
                setenv(
                    fvar.as_mut_ptr() as *const libc::c_char,
                    g_buf.as_mut_ptr() as *const libc::c_char,
                    1 as libc::c_int,
                );
            }
        }
        i = (i as libc::c_int + 1 as libc::c_int) as uchar_t;
    }
    tmp = xitoa(cfg.showhidden());
    setenv(
        b"NNN_INCLUDE_HIDDEN\0" as *const u8 as *const libc::c_char,
        tmp as *const libc::c_char,
        1 as libc::c_int,
    );
}
unsafe extern "C" fn run_cmd_as_plugin(
    mut file: *const libc::c_char,
    mut runfile: *mut libc::c_char,
    mut flags: uchar_t,
) {
    let mut len: size_t = 0;
    let mut tmp: bool = false;
    xstrsncpy(g_buf.as_mut_ptr(), file, 4096 as libc::c_int as size_t);
    len = xstrlen(g_buf.as_mut_ptr() as *const libc::c_char);
    if len > 1 as libc::c_ulong {
        if g_buf[len.wrapping_sub(1 as libc::c_ulong) as usize] as libc::c_int
            == 42 as libc::c_int
        {
            flags = (flags as libc::c_int & -(17 as libc::c_int)) as uchar_t;
            g_buf[len.wrapping_sub(1 as libc::c_ulong)
                as usize] = '\u{0}' as i32 as libc::c_char;
            len = len.wrapping_sub(1);
        }
    }
    let mut current_block_21: u64;
    if flags as libc::c_int & 128 as libc::c_int != 0 {
        current_block_21 = 15812749227941975616;
    } else if flags as libc::c_int & 4 as libc::c_int != 0 {
        current_block_21 = 15812749227941975616;
    } else {
        spawn(
            utils[7 as libc::c_int as usize],
            g_buf.as_mut_ptr(),
            0 as *mut libc::c_void as *mut libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_char,
            flags as ushort_t,
        );
        current_block_21 = 12147880666119273379;
    }
    match current_block_21 {
        15812749227941975616 => {
            tmp = is_suffix(
                g_buf.as_mut_ptr() as *const libc::c_char,
                b" $nnn\0" as *const u8 as *const libc::c_char,
            );
            if tmp {
                g_buf[len.wrapping_sub(5 as libc::c_ulong)
                    as usize] = '\u{0}' as i32 as libc::c_char;
            } else {
                runfile = 0 as *mut libc::c_void as *mut libc::c_char;
            }
            if flags as libc::c_int & 128 as libc::c_int != 0 {
                get_output(
                    g_buf.as_mut_ptr(),
                    runfile,
                    0 as *mut libc::c_void as *mut libc::c_char,
                    -(1 as libc::c_int),
                    1 as libc::c_int != 0,
                    1 as libc::c_int != 0,
                );
            } else {
                spawn(
                    g_buf.as_mut_ptr(),
                    runfile,
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                    flags as ushort_t,
                );
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn plctrl_init() -> bool {
    let mut len: size_t = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: __pid_t = 0;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    g_tmpfpath[(tmpfplen as libc::c_int - 1 as libc::c_int)
        as usize] = '\u{0}' as i32 as libc::c_char;
    len = xstrsncpy(
        g_pipepath.as_mut_ptr(),
        g_tmpfpath.as_mut_ptr() as *const libc::c_char,
        64 as libc::c_int as size_t,
    );
    g_pipepath[len.wrapping_sub(1 as libc::c_ulong)
        as usize] = '/' as i32 as libc::c_char;
    tmp = xstrsncpy(
        g_pipepath.as_mut_ptr().offset(len as isize),
        b"nnn-pipe.\0" as *const u8 as *const libc::c_char,
        (64 as libc::c_ulong).wrapping_sub(len),
    );
    len = tmp.wrapping_add(len);
    tmp___0 = getpid();
    tmp___1 = xitoa(tmp___0 as uint_t);
    xstrsncpy(
        g_pipepath
            .as_mut_ptr()
            .offset(len as isize)
            .offset(-(1 as libc::c_int as isize)),
        tmp___1 as *const libc::c_char,
        (64 as libc::c_ulong).wrapping_sub(len),
    );
    setenv(
        env_cfg[7 as libc::c_int as usize],
        g_pipepath.as_mut_ptr() as *const libc::c_char,
        1 as libc::c_int,
    );
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn rmlistpath() {
    if !listpath.is_null() {
        spawn(
            utils[20 as libc::c_int as usize],
            listpath,
            0 as *mut libc::c_void as *mut libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_char,
            5 as libc::c_int as ushort_t,
        );
        if listpath as libc::c_ulong != initpath as libc::c_ulong {
            free(listpath as *mut libc::c_void);
        }
        listpath = 0 as *mut libc::c_void as *mut libc::c_char;
    }
}
unsafe extern "C" fn read_nointr(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_void,
    mut count: size_t,
) -> ssize_t {
    let mut len: ssize_t = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    loop {
        len = read(fd, buf, count);
        if !(len == -(1 as libc::c_long)) {
            break;
        }
        tmp = __errno_location();
        if !(*tmp == 4 as libc::c_int) {
            break;
        }
    }
    return len;
}
unsafe extern "C" fn readpipe(
    mut fd: libc::c_int,
    mut ctxnum: *mut libc::c_char,
    mut path: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    let mut ctx: libc::c_char = 0;
    let mut nextpath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: ssize_t = 0;
    let mut tmp___0: ssize_t = 0;
    let mut tmp___1: uchar_t = 0;
    let mut tmp___2: ssize_t = 0;
    let mut op: libc::c_char = 0;
    let mut len: ssize_t = 0;
    let mut tmp___3: ssize_t = 0;
    let mut tmp___4: size_t = 0;
    nextpath = 0 as *mut libc::c_void as *mut libc::c_char;
    tmp = read_nointr(
        fd,
        g_buf.as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int as size_t,
    );
    if tmp != 1 as libc::c_long {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    if g_buf[0 as libc::c_int as usize] as libc::c_int == 45 as libc::c_int {
        clearselection();
        tmp___0 = read_nointr(
            fd,
            g_buf.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as size_t,
        );
        if tmp___0 != 1 as libc::c_long {
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
    }
    if g_buf[0 as libc::c_int as usize] as libc::c_int == 43 as libc::c_int {
        tmp___1 = get_free_ctx();
        ctx = (tmp___1 as libc::c_int + 1 as libc::c_int) as libc::c_char;
    } else if (g_buf[0 as libc::c_int as usize] as libc::c_int) < 48 as libc::c_int {
        return 0 as *mut libc::c_void as *mut libc::c_char
    } else {
        ctx = (g_buf[0 as libc::c_int as usize] as libc::c_int - 48 as libc::c_int)
            as libc::c_char;
        if ctx as libc::c_int > 4 as libc::c_int {
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
    }
    tmp___2 = read_nointr(
        fd,
        g_buf.as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int as size_t,
    );
    if tmp___2 != 1 as libc::c_long {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    op = g_buf[0 as libc::c_int as usize];
    if op as libc::c_int == 99 as libc::c_int {
        tmp___3 = read_nointr(
            fd,
            g_buf.as_mut_ptr() as *mut libc::c_void,
            4096 as libc::c_int as size_t,
        );
        len = tmp___3;
        if len <= 0 as libc::c_long {
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
        g_buf[len as usize] = '\u{0}' as i32 as libc::c_char;
        if g_buf[0 as libc::c_int as usize] as libc::c_int == 47 as libc::c_int {
            nextpath = g_buf.as_mut_ptr();
            tmp___4 = xstrlen(g_buf.as_mut_ptr() as *const libc::c_char);
            len = tmp___4 as ssize_t;
            loop {
                len -= 1;
                if !(len != 0) {
                    break;
                }
                if !(g_buf[len as usize] as libc::c_int == 47 as libc::c_int) {
                    break;
                }
                g_buf[len as usize] = '\u{0}' as i32 as libc::c_char;
            }
        }
    } else if op as libc::c_int == 108 as libc::c_int {
        rmlistpath();
        nextpath = load_input(fd, *path as *const libc::c_char);
    } else if op as libc::c_int == 112 as libc::c_int {
        free(selpath as *mut libc::c_void);
        selpath = 0 as *mut libc::c_void as *mut libc::c_char;
        clearselection();
        g_state.set_picker(0 as libc::c_int as uint_t);
        g_state.set_picked(1 as libc::c_int as uint_t);
    }
    *ctxnum = ctx;
    return nextpath;
}
unsafe extern "C" fn run_plugin(
    mut path: *mut *mut libc::c_char,
    mut file: *const libc::c_char,
    mut runfile: *mut libc::c_char,
    mut lastname: *mut *mut libc::c_char,
    mut lastdir: *mut *mut libc::c_char,
) -> bool {
    let mut p: pid_t = 0;
    let mut ctx: libc::c_char = 0;
    let mut flags: uchar_t = 0;
    let mut cmd_as_plugin: bool = false;
    let mut nextpath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut wfd: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut sel: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut std: [libc::c_char; 2] = [0; 2];
    let mut rfd: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    ctx = 0 as libc::c_int as libc::c_char;
    flags = 0 as libc::c_int as uchar_t;
    cmd_as_plugin = 0 as libc::c_int != 0;
    if g_state.pluginit() == 0 {
        plctrl_init();
        g_state.set_pluginit(1 as libc::c_int as uint_t);
    }
    setexports();
    if *file as libc::c_int == 33 as libc::c_int {
        flags = 17 as libc::c_int as uchar_t;
        file = file.offset(1);
        if *file as libc::c_int == 124 as libc::c_int {
            flags = (flags as libc::c_int | 128 as libc::c_int) as uchar_t;
            file = file.offset(1);
        } else if *file as libc::c_int == 38 as libc::c_int {
            flags = 6 as libc::c_int as uchar_t;
            file = file.offset(1);
        }
        if *file == 0 {
            return 0 as libc::c_int != 0;
        }
        if flags as libc::c_int & 4 as libc::c_int != 0 {
            run_cmd_as_plugin(file, runfile, flags);
            return 1 as libc::c_int != 0;
        } else {
            if flags as libc::c_int & 128 as libc::c_int != 0 {
                run_cmd_as_plugin(file, runfile, flags);
                return 1 as libc::c_int != 0;
            }
        }
        cmd_as_plugin = 1 as libc::c_int != 0;
    }
    tmp = mkfifo(
        g_pipepath.as_mut_ptr() as *const libc::c_char,
        384 as libc::c_int as __mode_t,
    );
    if tmp != 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    endwin();
    p = fork();
    if p == 0 {
        tmp___0 = open(
            g_pipepath.as_mut_ptr() as *const libc::c_char,
            524289 as libc::c_int,
        );
        wfd = tmp___0;
        if wfd == -(1 as libc::c_int) {
            _exit(1 as libc::c_int);
        }
        if !cmd_as_plugin {
            sel = 0 as *mut libc::c_void as *mut libc::c_char;
            std[0 as libc::c_int as usize] = '-' as i32 as libc::c_char;
            std[1 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
            mkpath(plgpath as *const libc::c_char, file, g_buf.as_mut_ptr());
            if g_state.picker() != 0 {
                if !selpath.is_null() {
                    sel = selpath;
                } else {
                    sel = std.as_mut_ptr();
                }
            }
            if !runfile.is_null() {
                if *runfile.offset(0 as libc::c_int as isize) != 0 {
                    xstrsncpy(
                        *lastname,
                        runfile as *const libc::c_char,
                        255 as libc::c_int as size_t,
                    );
                    spawn(
                        g_buf.as_mut_ptr(),
                        *lastname,
                        *path,
                        sel,
                        0 as libc::c_int as ushort_t,
                    );
                } else {
                    spawn(
                        g_buf.as_mut_ptr(),
                        0 as *mut libc::c_void as *mut libc::c_char,
                        *path,
                        sel,
                        0 as libc::c_int as ushort_t,
                    );
                }
            } else {
                spawn(
                    g_buf.as_mut_ptr(),
                    0 as *mut libc::c_void as *mut libc::c_char,
                    *path,
                    sel,
                    0 as libc::c_int as ushort_t,
                );
            }
        } else {
            run_cmd_as_plugin(file, runfile, flags);
        }
        close(wfd);
        _exit(0 as libc::c_int);
    }
    loop {
        rfd = open(g_pipepath.as_mut_ptr() as *const libc::c_char, 0 as libc::c_int);
        if !(rfd == -(1 as libc::c_int)) {
            break;
        }
        tmp___1 = __errno_location();
        if !(*tmp___1 == 4 as libc::c_int) {
            break;
        }
    }
    nextpath = readpipe(rfd, &mut ctx, path);
    if !nextpath.is_null() {
        set_smart_ctx(ctx as libc::c_int, nextpath, path, runfile, lastname, lastdir);
    }
    close(rfd);
    waitpid(p, 0 as *mut libc::c_void as *mut libc::c_int, 0 as libc::c_int);
    wrefresh(stdscr);
    unlink(g_pipepath.as_mut_ptr() as *const libc::c_char);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn launch_app(mut newpath: *mut libc::c_char) -> bool {
    let mut r: libc::c_int = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: bool = false;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *const libc::c_char = 0 as *const libc::c_char;
    r = 8 as libc::c_int;
    tmp = newpath;
    mkpath(
        plgpath as *const libc::c_char,
        utils[6 as libc::c_int as usize] as *const libc::c_char,
        newpath,
    );
    tmp___0 = getutil(utils[14 as libc::c_int as usize]);
    if tmp___0 {
        tmp___1 = access(newpath as *const libc::c_char, 1 as libc::c_int);
        if tmp___1 < 0 as libc::c_int {
            tmp = xreadline(
                0 as *mut libc::c_void as *const libc::c_char,
                messages[32 as libc::c_int as usize],
            );
            r = 7 as libc::c_int;
        }
    } else {
        tmp = xreadline(
            0 as *mut libc::c_void as *const libc::c_char,
            messages[32 as libc::c_int as usize],
        );
        r = 7 as libc::c_int;
    }
    if !tmp.is_null() {
        if *tmp != 0 {
            if r == 8 as libc::c_int {
                tmp___2 = b"0\0" as *const u8 as *const libc::c_char;
            } else {
                tmp___2 = 0 as *mut libc::c_void as *const libc::c_char;
            }
            spawn(
                tmp,
                tmp___2 as *mut libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_char,
                r as ushort_t,
            );
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn prompt_run() -> bool {
    let mut ret: bool = false;
    let mut cmdline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cnt_j: libc::c_int = 0;
    let mut cnt_J: libc::c_int = 0;
    let mut cmd_ret: libc::c_int = 0;
    let mut len: size_t = 0;
    let mut xargs_j: *const libc::c_char = 0 as *const libc::c_char;
    let mut xargs_J: *const libc::c_char = 0 as *const libc::c_char;
    let mut cmd___0: [libc::c_char; 4640] = [0; 4640];
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    ret = 0 as libc::c_int != 0;
    xargs_j = b"xargs -0 -I{} %s < %s\0" as *const u8 as *const libc::c_char;
    xargs_J = b"xargs -0 %s < %s\0" as *const u8 as *const libc::c_char;
    loop {
        cmdline = xreadline(
            0 as *mut libc::c_void as *const libc::c_char,
            b">>> \0" as *const u8 as *const libc::c_char,
        );
        if cmdline.is_null() {
            break;
        }
        if *cmdline.offset(0 as libc::c_int as isize) == 0 {
            break;
        }
        free(lastcmd as *mut libc::c_void);
        lastcmd = xstrdup(cmdline as *const libc::c_char);
        ret = 1 as libc::c_int != 0;
        len = xstrlen(cmdline as *const libc::c_char);
        cnt_j = 0 as libc::c_int;
        next = cmdline;
        loop {
            next = strstr(
                next as *const libc::c_char,
                b"%j\0" as *const u8 as *const libc::c_char,
            );
            if next.is_null() {
                break;
            }
            cnt_j += 1;
            *next.offset(0 as libc::c_int as isize) = '{' as i32 as libc::c_char;
            *next.offset(1 as libc::c_int as isize) = '}' as i32 as libc::c_char;
            next = next.offset(1);
        }
        cnt_J = 0 as libc::c_int;
        next = cmdline;
        loop {
            next = strstr(
                next as *const libc::c_char,
                b"%J\0" as *const u8 as *const libc::c_char,
            );
            if next.is_null() {
                break;
            }
            cnt_J += 1;
            if next as libc::c_ulong
                == cmdline.offset(len as isize).offset(-(2 as libc::c_int as isize))
                    as libc::c_ulong
            {
                *cmdline
                    .offset(
                        len.wrapping_sub(2 as libc::c_ulong) as isize,
                    ) = '\u{0}' as i32 as libc::c_char;
            }
            next = next.offset(1);
        }
        if cnt_j != 0 {
            if cnt_J != 0 {
                break;
            }
        }
        if cnt_j != 0 {
            snprintf(
                cmd___0.as_mut_ptr(),
                (4096 as libc::c_int + ((256 as libc::c_int) << 1 as libc::c_int)
                    + 32 as libc::c_int) as size_t,
                xargs_j,
                cmdline,
                selpath,
            );
        } else if cnt_J != 0 {
            snprintf(
                cmd___0.as_mut_ptr(),
                (4096 as libc::c_int + ((256 as libc::c_int) << 1 as libc::c_int)
                    + 32 as libc::c_int) as size_t,
                xargs_J,
                cmdline,
                selpath,
            );
        }
        if cnt_j != 0 {
            tmp = cmd___0.as_mut_ptr();
        } else if cnt_J != 0 {
            tmp = cmd___0.as_mut_ptr();
        } else {
            tmp = cmdline;
        }
        cmd_ret = spawn(
            shell,
            b"-c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            tmp,
            0 as *mut libc::c_void as *mut libc::c_char,
            25 as libc::c_int as ushort_t,
        );
        let mut current_block_42: u64;
        if cnt_j != 0 {
            current_block_42 = 16630112972684685292;
        } else if cnt_J != 0 {
            current_block_42 = 16630112972684685292;
        } else {
            current_block_42 = 13619784596304402172;
        }
        match current_block_42 {
            16630112972684685292 => {
                if cmd_ret == 0 as libc::c_int {
                    clearselection();
                }
            }
            _ => {}
        }
    }
    return ret;
}
unsafe extern "C" fn handle_cmd(
    mut sel: action,
    mut newpath: *mut libc::c_char,
) -> bool {
    let mut tmp: bool = false;
    let mut tmp___0: bool = false;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    endselection(0 as libc::c_int != 0);
    if sel as libc::c_uint == 56 as libc::c_uint {
        tmp = launch_app(newpath);
        return tmp;
    }
    setexports();
    if sel as libc::c_uint == 57 as libc::c_uint {
        tmp___0 = prompt_run();
        return tmp___0;
    }
    tmp___2 = getenv(env_cfg[6 as libc::c_int as usize]);
    tmp___1 = tmp___2;
    if !tmp___1.is_null() {
        tmp___3 = atoi(tmp___1 as *const libc::c_char);
        tmp___4 = tmp___3;
    } else {
        tmp___4 = 0 as libc::c_int;
    }
    r = tmp___4;
    tmp___5 = xitoa((r + 1 as libc::c_int) as uint_t);
    setenv(
        env_cfg[6 as libc::c_int as usize],
        tmp___5 as *const libc::c_char,
        1 as libc::c_int,
    );
    spawn(
        shell,
        0 as *mut libc::c_void as *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
        9 as libc::c_int as ushort_t,
    );
    tmp___6 = xitoa(r as uint_t);
    setenv(
        env_cfg[6 as libc::c_int as usize],
        tmp___6 as *const libc::c_char,
        1 as libc::c_int,
    );
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn dentfree() {
    free(pnamebuf as *mut libc::c_void);
    free(pdents as *mut libc::c_void);
    free(mark as *mut libc::c_void);
    free(core_blocks as *mut libc::c_void);
    free(core_data as *mut libc::c_void);
    free(core_files as *mut libc::c_void);
}
unsafe extern "C" fn du_thread(mut p_data: *mut libc::c_void) -> *mut libc::c_void {
    let mut pdata: *mut thread_data = 0 as *mut thread_data;
    let mut path: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
    let mut tfiles: ullong_t = 0;
    let mut tblocks: blkcnt_t = 0;
    let mut sb: *mut stat = 0 as *mut stat;
    let mut tree: *mut FTS = 0 as *mut FTS;
    let mut tmp: *mut FTS = 0 as *mut FTS;
    let mut node: *mut FTSENT = 0 as *mut FTSENT;
    let mut tmp___0: bool = false;
    let mut tmp___1: bool = false;
    pdata = p_data as *mut thread_data;
    path[0 as libc::c_int as usize] = ((*pdata).path).as_mut_ptr();
    path[1 as libc::c_int as usize] = 0 as *mut libc::c_void as *mut libc::c_char;
    tfiles = 0 as libc::c_int as ullong_t;
    tblocks = 0 as libc::c_int as blkcnt_t;
    tmp = fts_open(
        path.as_mut_ptr() as *const *mut libc::c_char,
        84 as libc::c_int,
        None,
    );
    tree = tmp;
    loop {
        node = fts_read(tree);
        if node.is_null() {
            break;
        }
        if (*node).fts_info as libc::c_int & 1 as libc::c_int != 0 {
            if g_state.interrupt() != 0 {
                break;
            }
        } else {
            sb = (*node).fts_statp;
            if cfg.apparentsz() != 0 {
                if (*sb).st_size != 0 {
                    let mut current_block_16: u64;
                    if (*node).fts_info as libc::c_int & 8 as libc::c_int != 0 {
                        if (*sb).st_nlink <= 1 as libc::c_ulong {
                            tblocks += (*sb).st_size;
                            current_block_16 = 7056779235015430508;
                        } else {
                            tmp___0 = test_set_bit((*sb).st_ino as uint_t);
                            if tmp___0 {
                                tblocks += (*sb).st_size;
                                current_block_16 = 7056779235015430508;
                            } else {
                                current_block_16 = 16410502208528105634;
                            }
                        }
                    } else {
                        current_block_16 = 16410502208528105634;
                    }
                    match current_block_16 {
                        16410502208528105634 => {
                            if (*node).fts_info as libc::c_int & 6 as libc::c_int != 0 {
                                tblocks += (*sb).st_size;
                            }
                        }
                        _ => {}
                    }
                }
            } else if (*sb).st_blocks != 0 {
                let mut current_block_28: u64;
                if (*node).fts_info as libc::c_int & 8 as libc::c_int != 0 {
                    if (*sb).st_nlink <= 1 as libc::c_ulong {
                        tblocks += (*sb).st_blocks;
                        current_block_28 = 6450597802325118133;
                    } else {
                        tmp___1 = test_set_bit((*sb).st_ino as uint_t);
                        if tmp___1 {
                            tblocks += (*sb).st_blocks;
                            current_block_28 = 6450597802325118133;
                        } else {
                            current_block_28 = 7884692776483956936;
                        }
                    }
                } else {
                    current_block_28 = 7884692776483956936;
                }
                match current_block_28 {
                    7884692776483956936 => {
                        if (*node).fts_info as libc::c_int & 6 as libc::c_int != 0 {
                            tblocks += (*sb).st_blocks;
                        }
                    }
                    _ => {}
                }
            }
            tfiles = tfiles.wrapping_add(1);
        }
    }
    fts_close(tree);
    if (*pdata).entnum >= 0 as libc::c_int {
        let ref mut fresh10 = (*pdents.offset((*pdata).entnum as isize))
            .__annonCompField18;
        (*fresh10).set_blocks(tblocks as ullong_t);
    }
    if !(*pdata).mntpoint {
        let ref mut fresh11 = *core_blocks.offset((*pdata).core as libc::c_int as isize);
        *fresh11 += tblocks;
        let ref mut fresh12 = *core_files.offset((*pdata).core as libc::c_int as isize);
        *fresh12 = (*fresh12 as libc::c_ulonglong).wrapping_add(tfiles) as ullong_t
            as ullong_t;
    } else {
        let ref mut fresh13 = *core_files.offset((*pdata).core as libc::c_int as isize);
        *fresh13 = (*fresh13).wrapping_add(1);
    }
    pthread_mutex_lock(&mut running_mutex);
    threadbmp |= (1 as libc::c_int) << (*pdata).core as libc::c_int;
    active_threads = ::std::ptr::read_volatile::<
        libc::c_int,
    >(&active_threads as *const libc::c_int) - 1 as libc::c_int;
    pthread_mutex_unlock(&mut running_mutex);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn dirwalk(
    mut path: *mut libc::c_char,
    mut entnum: libc::c_int,
    mut mountpoint: bool,
) {
    let mut core: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tid: pthread_t = 0;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    while active_threads == 4 as libc::c_int {}
    if g_state.interrupt() != 0 {
        return;
    }
    pthread_mutex_lock(&mut running_mutex);
    tmp = ffs(threadbmp);
    core = tmp - 1 as libc::c_int;
    threadbmp &= !((1 as libc::c_int) << core);
    active_threads = ::std::ptr::read_volatile::<
        libc::c_int,
    >(&active_threads as *const libc::c_int) + 1 as libc::c_int;
    pthread_mutex_unlock(&mut running_mutex);
    xstrsncpy(
        ((*core_data.offset(core as isize)).path).as_mut_ptr(),
        path as *const libc::c_char,
        4096 as libc::c_int as size_t,
    );
    (*core_data.offset(core as isize)).entnum = entnum;
    (*core_data.offset(core as isize)).core = core as ushort_t;
    (*core_data.offset(core as isize)).mntpoint = mountpoint;
    tid = 0 as libc::c_int as pthread_t;
    pthread_create(
        &mut tid as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(du_thread as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        core_data.offset(core as isize) as *mut libc::c_void,
    );
    wmove(stdscr, xlines as libc::c_int - 1 as libc::c_int, 0 as libc::c_int);
    tmp___0 = xbasename(path);
    waddnstr(stdscr, tmp___0 as *const libc::c_char, -(1 as libc::c_int));
    waddnstr(
        stdscr,
        b" [^C aborts]\n\0" as *const u8 as *const libc::c_char,
        -(1 as libc::c_int),
    );
    wrefresh(stdscr);
}
unsafe extern "C" fn prep_threads() -> bool {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    if g_state.duinit() == 0 {
        threadbmp >>= 28 as libc::c_int;
        if core_blocks.is_null() {
            tmp = calloc(
                4 as libc::c_int as size_t,
                ::std::mem::size_of::<blkcnt_t>() as libc::c_ulong,
            );
            core_blocks = tmp as *mut blkcnt_t;
        }
        if core_data.is_null() {
            tmp___0 = calloc(
                4 as libc::c_int as size_t,
                ::std::mem::size_of::<thread_data>() as libc::c_ulong,
            );
            core_data = tmp___0 as *mut thread_data;
        }
        if core_files.is_null() {
            tmp___1 = calloc(
                4 as libc::c_int as size_t,
                ::std::mem::size_of::<ullong_t>() as libc::c_ulong,
            );
            core_files = tmp___1 as *mut ullong_t;
        }
        if core_blocks.is_null() {
            tmp___2 = __errno_location();
            tmp___3 = strerror(*tmp___2);
            printwait(
                tmp___3 as *const libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_int,
            );
            return 0 as libc::c_int != 0;
        } else {
            if core_data.is_null() {
                tmp___2 = __errno_location();
                tmp___3 = strerror(*tmp___2);
                printwait(
                    tmp___3 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_int,
                );
                return 0 as libc::c_int != 0;
            } else {
                if core_files.is_null() {
                    tmp___2 = __errno_location();
                    tmp___3 = strerror(*tmp___2);
                    printwait(
                        tmp___3 as *const libc::c_char,
                        0 as *mut libc::c_void as *mut libc::c_int,
                    );
                    return 0 as libc::c_int != 0;
                }
            }
        }
        max_openfds();
        g_state.set_duinit(1 as libc::c_int as uint_t);
    } else {
        memset(
            core_blocks as *mut libc::c_void,
            0 as libc::c_int,
            (4 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<blkcnt_t>() as libc::c_ulong),
        );
        memset(
            core_data as *mut libc::c_void,
            0 as libc::c_int,
            (4 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<thread_data>() as libc::c_ulong),
        );
        memset(
            core_files as *mut libc::c_void,
            0 as libc::c_int,
            (4 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<ullong_t>() as libc::c_ulong),
        );
    }
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn selforparent(mut path: *const libc::c_char) -> bool {
    let mut tmp: libc::c_int = 0;
    if *path.offset(0 as libc::c_int as isize) as libc::c_int == 46 as libc::c_int {
        if *path.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
            tmp = 1 as libc::c_int;
        } else if *path.offset(1 as libc::c_int as isize) as libc::c_int
                == 46 as libc::c_int
            {
            if *path.offset(2 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
            {
                tmp = 1 as libc::c_int;
            } else {
                tmp = 0 as libc::c_int;
            }
        } else {
            tmp = 0 as libc::c_int;
        }
    } else {
        tmp = 0 as libc::c_int;
    }
    return tmp != 0;
}
unsafe extern "C" fn dentfill(
    mut path: *mut libc::c_char,
    mut ppdents: *mut *mut entry,
) -> libc::c_int {
    let mut current_block: u64;
    let mut entflags: uchar_t = 0;
    let mut flags: libc::c_int = 0;
    let mut dp: *mut dirent = 0 as *mut dirent;
    let mut namep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pnb: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dentp: *mut entry = 0 as *mut entry;
    let mut off: size_t = 0;
    let mut namebuflen: size_t = 0;
    let mut sb_path: stat = stat {
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
    let mut dirp: *mut DIR = 0 as *mut DIR;
    let mut tmp: *mut DIR = 0 as *mut DIR;
    let mut fd: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: bool = false;
    let mut tmp___4: bool = false;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: __off_t = 0;
    let mut tmp___7: bool = false;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___11: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut count: libc::c_int = 0;
    let mut tmp___12: size_t = 0;
    let mut tmp___13: libc::c_int = 0;
    let mut tmp___14: bool = false;
    let mut i: libc::c_int = 0;
    let mut tmp___15: libc::c_int = 0;
    entflags = 0 as libc::c_int as uchar_t;
    flags = 0 as libc::c_int;
    off = 0 as libc::c_int as size_t;
    namebuflen = 2048 as libc::c_int as size_t;
    tmp = opendir(path as *const libc::c_char);
    dirp = tmp;
    ndents = 0 as libc::c_int;
    gtimesecs = time(0 as *mut libc::c_void as *mut time_t);
    if dirp.is_null() {
        return 0 as libc::c_int;
    }
    tmp___0 = dirfd(dirp);
    fd = tmp___0;
    if cfg.blkorder() != 0 {
        num_files = 0 as libc::c_int as ullong_t;
        dir_blocks = 0 as libc::c_int as blkcnt_t;
        buf = g_buf.as_mut_ptr();
        tmp___1 = fstatat(
            fd,
            path as *const libc::c_char,
            &mut sb_path as *mut stat,
            0 as libc::c_int,
        );
        if tmp___1 == -(1 as libc::c_int) {
            current_block = 13397314465273517025;
        } else {
            if ihashbmp.is_null() {
                tmp___2 = calloc(
                    1 as libc::c_int as size_t,
                    ((16777215 as libc::c_int >> 6 as libc::c_int) << 3 as libc::c_int)
                        as size_t,
                );
                ihashbmp = tmp___2 as *mut ullong_t;
                if ihashbmp.is_null() {
                    current_block = 13397314465273517025;
                } else {
                    current_block = 11459959175219260272;
                }
            } else {
                memset(
                    ihashbmp as *mut libc::c_void,
                    0 as libc::c_int,
                    ((16777215 as libc::c_int >> 6 as libc::c_int) << 3 as libc::c_int)
                        as size_t,
                );
                current_block = 11459959175219260272;
            }
            match current_block {
                13397314465273517025 => {}
                _ => {
                    tmp___3 = prep_threads();
                    if !tmp___3 {
                        current_block = 13397314465273517025;
                    } else {
                        wattr_on(
                            stdscr,
                            cfg.curctx().wrapping_add(1 as libc::c_uint)
                                << 8 as libc::c_int
                                & ((1 as libc::c_uint) << 8 as libc::c_int)
                                    .wrapping_sub(1 as libc::c_uint) << 8 as libc::c_int,
                            0 as *mut libc::c_void,
                        );
                        current_block = 18153031941552419006;
                    }
                }
            }
        }
    } else {
        current_block = 18153031941552419006;
    }
    match current_block {
        18153031941552419006 => {
            posix_fadvise(
                fd,
                0 as libc::c_int as off_t,
                0 as libc::c_int as off_t,
                2 as libc::c_int,
            );
            dp = readdir(dirp);
            if !dp.is_null() {
                if cfg.blkorder() != 0 {
                    flags = 256 as libc::c_int;
                } else if (*dp).d_type as libc::c_int == 0 as libc::c_int {
                    flags = 256 as libc::c_int;
                }
                loop {
                    namep = ((*dp).d_name).as_mut_ptr();
                    tmp___4 = selforparent(namep as *const libc::c_char);
                    if !tmp___4 {
                        if cfg.showhidden() == 0 {
                            if *namep.offset(0 as libc::c_int as isize) as libc::c_int
                                == 46 as libc::c_int
                            {
                                if cfg.blkorder() == 0 {
                                    current_block = 9688129337859931739;
                                } else {
                                    tmp___5 = fstatat(
                                        fd,
                                        namep as *const libc::c_char,
                                        &mut sb as *mut stat,
                                        256 as libc::c_int,
                                    );
                                    if tmp___5 == -(1 as libc::c_int) {
                                        current_block = 9688129337859931739;
                                    } else {
                                        if sb.st_mode & 61440 as libc::c_uint
                                            == 16384 as libc::c_uint
                                        {
                                            if sb_path.st_dev == sb.st_dev {
                                                mkpath(
                                                    path as *const libc::c_char,
                                                    namep as *const libc::c_char,
                                                    buf,
                                                );
                                                dirwalk(buf, -(1 as libc::c_int), 0 as libc::c_int != 0);
                                                if g_state.interrupt() != 0 {
                                                    break;
                                                }
                                            }
                                        } else {
                                            let mut current_block_42: u64;
                                            if sb.st_nlink <= 1 as libc::c_ulong {
                                                current_block_42 = 12527975023709851102;
                                            } else {
                                                tmp___7 = test_set_bit(sb.st_ino as uint_t);
                                                if tmp___7 {
                                                    current_block_42 = 12527975023709851102;
                                                } else {
                                                    current_block_42 = 13125627826496529465;
                                                }
                                            }
                                            match current_block_42 {
                                                12527975023709851102 => {
                                                    if cfg.apparentsz() != 0 {
                                                        tmp___6 = sb.st_size;
                                                    } else {
                                                        tmp___6 = sb.st_blocks;
                                                    }
                                                    dir_blocks += tmp___6;
                                                }
                                                _ => {}
                                            }
                                            num_files = num_files.wrapping_add(1);
                                        }
                                        current_block = 9688129337859931739;
                                    }
                                }
                            } else {
                                current_block = 5028470053297453708;
                            }
                        } else {
                            current_block = 5028470053297453708;
                        }
                        match current_block {
                            9688129337859931739 => {}
                            _ => {
                                tmp___9 = fstatat(
                                    fd,
                                    namep as *const libc::c_char,
                                    &mut sb as *mut stat,
                                    flags,
                                );
                                if tmp___9 == -(1 as libc::c_int) {
                                    if flags != 0 {
                                        entflags = 8 as libc::c_int as uchar_t;
                                        memset(
                                            &mut sb as *mut stat as *mut libc::c_void,
                                            0 as libc::c_int,
                                            ::std::mem::size_of::<stat>() as libc::c_ulong,
                                        );
                                    } else {
                                        tmp___8 = fstatat(
                                            fd,
                                            namep as *const libc::c_char,
                                            &mut sb as *mut stat,
                                            256 as libc::c_int,
                                        );
                                        if tmp___8 == -(1 as libc::c_int) {
                                            entflags = 8 as libc::c_int as uchar_t;
                                            memset(
                                                &mut sb as *mut stat as *mut libc::c_void,
                                                0 as libc::c_int,
                                                ::std::mem::size_of::<stat>() as libc::c_ulong,
                                            );
                                        } else {
                                            entflags = 4 as libc::c_int as uchar_t;
                                        }
                                    }
                                }
                                if ndents == total_dents {
                                    if cfg.blkorder() != 0 {
                                        while active_threads != 0 {}
                                    }
                                    total_dents += 64 as libc::c_int;
                                    tmp___10 = xrealloc(
                                        *ppdents as *mut libc::c_void,
                                        (total_dents as libc::c_ulong)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<entry>() as libc::c_ulong,
                                            ),
                                    );
                                    *ppdents = tmp___10 as *mut entry;
                                    if (*ppdents).is_null() {
                                        free(pnamebuf as *mut libc::c_void);
                                        closedir(dirp);
                                        printerr(5673 as libc::c_int);
                                    }
                                }
                                if namebuflen.wrapping_sub(off) < 256 as libc::c_ulong {
                                    namebuflen = (namebuflen as libc::c_ulong)
                                        .wrapping_add(2048 as libc::c_ulong) as size_t as size_t;
                                    pnb = pnamebuf;
                                    tmp___11 = xrealloc(
                                        pnamebuf as *mut libc::c_void,
                                        namebuflen,
                                    );
                                    pnamebuf = tmp___11 as *mut libc::c_char;
                                    if pnamebuf.is_null() {
                                        free(*ppdents as *mut libc::c_void);
                                        closedir(dirp);
                                        printerr(5687 as libc::c_int);
                                    }
                                    if pnb as libc::c_ulong != pnamebuf as libc::c_ulong {
                                        dentp = *ppdents;
                                        (*dentp).name = pnamebuf;
                                        count = 1 as libc::c_int;
                                        while count < ndents {
                                            let ref mut fresh14 = (*dentp
                                                .offset(1 as libc::c_int as isize))
                                                .name;
                                            *fresh14 = ((*dentp).name as size_t as ullong_t)
                                                .wrapping_add(((*dentp).__annonCompField18).nlen())
                                                as *mut libc::c_char;
                                            dentp = dentp.offset(1);
                                            count += 1;
                                        }
                                    }
                                }
                                dentp = (*ppdents).offset(ndents as isize);
                                (*dentp)
                                    .name = (pnamebuf as size_t).wrapping_add(off)
                                    as *mut libc::c_char;
                                tmp___12 = xstrsncpy(
                                    (*dentp).name,
                                    namep as *const libc::c_char,
                                    256 as libc::c_int as size_t,
                                );
                                ((*dentp).__annonCompField18)
                                    .set_nlen(tmp___12 as ullong_t);
                                off = (off as ullong_t)
                                    .wrapping_add(((*dentp).__annonCompField18).nlen())
                                    as size_t;
                                if cfg.timetype() == 2 as libc::c_uint {
                                    (*dentp).sec = sb.st_mtim.tv_sec;
                                    (*dentp).nsec = sb.st_mtim.tv_nsec as uint_t;
                                } else if cfg.timetype() == 0 as libc::c_uint {
                                    (*dentp).sec = sb.st_atim.tv_sec;
                                    (*dentp).nsec = sb.st_atim.tv_nsec as uint_t;
                                } else {
                                    (*dentp).sec = sb.st_ctim.tv_sec;
                                    (*dentp).nsec = sb.st_ctim.tv_nsec as uint_t;
                                }
                                if gtimesecs - sb.st_mtim.tv_sec <= 300 as libc::c_long {
                                    entflags = (entflags as libc::c_int | 64 as libc::c_int)
                                        as uchar_t;
                                } else if gtimesecs - sb.st_ctim.tv_sec
                                        <= 300 as libc::c_long
                                    {
                                    entflags = (entflags as libc::c_int | 64 as libc::c_int)
                                        as uchar_t;
                                }
                                if flags == 0 {
                                    if (*dp).d_type as libc::c_int == 10 as libc::c_int {
                                        (*dentp)
                                            .mode = sb.st_mode & 4294905855 as libc::c_uint
                                            | 40960 as libc::c_uint;
                                        if !listpath.is_null() {
                                            (*dentp).size = sb.st_size;
                                        } else {
                                            (*dentp).size = 0 as libc::c_int as off_t;
                                        }
                                    } else {
                                        (*dentp).mode = sb.st_mode;
                                        (*dentp).size = sb.st_size;
                                    }
                                } else {
                                    (*dentp).mode = sb.st_mode;
                                    (*dentp).size = sb.st_size;
                                }
                                if sb.st_mode & 61440 as libc::c_uint
                                    == 16384 as libc::c_uint
                                {
                                    ((*dentp).__annonCompField18)
                                        .set_flags(0 as libc::c_int as ullong_t);
                                } else {
                                    if sb.st_nlink > 1 as libc::c_ulong {
                                        tmp___13 = 2 as libc::c_int;
                                    } else {
                                        tmp___13 = 0 as libc::c_int;
                                    }
                                    ((*dentp).__annonCompField18)
                                        .set_flags(tmp___13 as ullong_t);
                                }
                                if entflags != 0 {
                                    ((*dentp).__annonCompField18)
                                        .set_flags(
                                            ((*dentp).__annonCompField18).flags()
                                                | entflags as libc::c_ulonglong as ullong_t,
                                        );
                                    entflags = 0 as libc::c_int as uchar_t;
                                }
                                if cfg.blkorder() != 0 {
                                    if sb.st_mode & 61440 as libc::c_uint
                                        == 16384 as libc::c_uint
                                    {
                                        mkpath(
                                            path as *const libc::c_char,
                                            namep as *const libc::c_char,
                                            buf,
                                        );
                                        dirwalk(buf, ndents, sb_path.st_dev != sb.st_dev);
                                        if g_state.interrupt() != 0 {
                                            break;
                                        }
                                    } else {
                                        if cfg.apparentsz() != 0 {
                                            ((*dentp).__annonCompField18)
                                                .set_blocks(sb.st_size as ullong_t);
                                        } else {
                                            ((*dentp).__annonCompField18)
                                                .set_blocks(sb.st_blocks as ullong_t);
                                        }
                                        if sb.st_nlink <= 1 as libc::c_ulong {
                                            dir_blocks = (dir_blocks as ullong_t)
                                                .wrapping_add(((*dentp).__annonCompField18).blocks())
                                                as blkcnt_t;
                                        } else {
                                            tmp___14 = test_set_bit(sb.st_ino as uint_t);
                                            if tmp___14 {
                                                dir_blocks = (dir_blocks as ullong_t)
                                                    .wrapping_add(((*dentp).__annonCompField18).blocks())
                                                    as blkcnt_t;
                                            }
                                        }
                                        num_files = num_files.wrapping_add(1);
                                    }
                                }
                                if flags != 0 {
                                    if sb.st_mode & 61440 as libc::c_uint
                                        == 40960 as libc::c_uint
                                    {
                                        sb.st_mode = 0 as libc::c_int as __mode_t;
                                        fstatat(
                                            fd,
                                            namep as *const libc::c_char,
                                            &mut sb as *mut stat,
                                            0 as libc::c_int,
                                        );
                                    }
                                    if sb.st_mode & 61440 as libc::c_uint
                                        == 16384 as libc::c_uint
                                    {
                                        ((*dentp).__annonCompField18)
                                            .set_flags(
                                                ((*dentp).__annonCompField18).flags()
                                                    | 1 as libc::c_ulonglong as ullong_t,
                                            );
                                    }
                                } else if (*dp).d_type as libc::c_int == 4 as libc::c_int {
                                    ((*dentp).__annonCompField18)
                                        .set_flags(
                                            ((*dentp).__annonCompField18).flags()
                                                | 1 as libc::c_ulonglong as ullong_t,
                                        );
                                } else {
                                    let mut current_block_177: u64;
                                    if (*dp).d_type as libc::c_int == 10 as libc::c_int {
                                        current_block_177 = 10185719803480765928;
                                    } else if (*dp).d_type as libc::c_int == 0 as libc::c_int {
                                        current_block_177 = 10185719803480765928;
                                    } else {
                                        current_block_177 = 15908231092227701503;
                                    }
                                    match current_block_177 {
                                        10185719803480765928 => {
                                            if sb.st_mode & 61440 as libc::c_uint
                                                == 16384 as libc::c_uint
                                            {
                                                ((*dentp).__annonCompField18)
                                                    .set_flags(
                                                        ((*dentp).__annonCompField18).flags()
                                                            | 1 as libc::c_ulonglong as ullong_t,
                                                    );
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                ndents += 1;
                            }
                        }
                    }
                    dp = readdir(dirp);
                    if dp.is_null() {
                        break;
                    }
                }
            }
        }
        _ => {}
    }
    if g_state.duinit() != 0 {
        if cfg.blkorder() != 0 {
            while active_threads != 0 {}
            wattr_off(
                stdscr,
                cfg.curctx().wrapping_add(1 as libc::c_uint) << 8 as libc::c_int
                    & ((1 as libc::c_uint) << 8 as libc::c_int)
                        .wrapping_sub(1 as libc::c_uint) << 8 as libc::c_int,
                0 as *mut libc::c_void,
            );
            i = 0 as libc::c_int;
            while i < 4 as libc::c_int {
                num_files = (num_files as libc::c_ulonglong)
                    .wrapping_add(*core_files.offset(i as isize)) as ullong_t
                    as ullong_t;
                dir_blocks += *core_blocks.offset(i as isize);
                i += 1;
            }
        }
    }
    tmp___15 = closedir(dirp);
    if tmp___15 == -(1 as libc::c_int) {
        printerr(5811 as libc::c_int);
    }
    return ndents;
}
unsafe extern "C" fn populate(
    mut path: *mut libc::c_char,
    mut lastname: *mut libc::c_char,
) {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    ndents = dentfill(path, &mut pdents);
    if ndents == 0 {
        return;
    }
    if *lastname != 0 {
        tmp = dentfind(lastname as *const libc::c_char, ndents);
        tmp___0 = tmp;
    } else {
        tmp___0 = 0 as libc::c_int;
    }
    move_cursor(tmp___0, 0 as libc::c_int);
    last_curscroll = -(1 as libc::c_int);
}
unsafe extern "C" fn move_cursor(
    mut target: libc::c_int,
    mut ignore_scrolloff: libc::c_int,
) {
    let mut onscreen: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut delta: libc::c_int = 0;
    let mut scrolloff: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    onscreen = xlines as libc::c_int - 4 as libc::c_int;
    if (ndents - 1 as libc::c_int) < target {
        tmp___1 = ndents - 1 as libc::c_int;
    } else {
        tmp___1 = target;
    }
    if 0 as libc::c_int > tmp___1 {
        target = 0 as libc::c_int;
    } else {
        if (ndents - 1 as libc::c_int) < target {
            tmp___0 = ndents - 1 as libc::c_int;
        } else {
            tmp___0 = target;
        }
        target = tmp___0;
    }
    last_curscroll = curscroll;
    last = cur;
    cur = target;
    if ignore_scrolloff == 0 {
        delta = target - last;
        if (3 as libc::c_int) < onscreen >> 1 as libc::c_int {
            tmp___2 = 3 as libc::c_int;
        } else {
            tmp___2 = onscreen >> 1 as libc::c_int;
        }
        scrolloff = tmp___2;
        let mut current_block_32: u64;
        if cur < curscroll + scrolloff {
            if delta < 0 as libc::c_int {
                curscroll += delta;
                current_block_32 = 17500079516916021833;
            } else {
                current_block_32 = 14314907983631908374;
            }
        } else {
            current_block_32 = 14314907983631908374;
        }
        match current_block_32 {
            14314907983631908374 => {
                if cur > curscroll + onscreen - scrolloff - 1 as libc::c_int {
                    if delta > 0 as libc::c_int {
                        curscroll += delta;
                    }
                }
            }
            _ => {}
        }
    }
    if cur < ndents - onscreen {
        tmp___5 = cur;
    } else {
        tmp___5 = ndents - onscreen;
    }
    if curscroll < tmp___5 {
        curscroll = curscroll;
    } else {
        if cur < ndents - onscreen {
            tmp___4 = cur;
        } else {
            tmp___4 = ndents - onscreen;
        }
        curscroll = tmp___4;
    }
    if cur - (onscreen - 1 as libc::c_int) > 0 as libc::c_int {
        tmp___8 = cur - (onscreen - 1 as libc::c_int);
    } else {
        tmp___8 = 0 as libc::c_int;
    }
    if curscroll > tmp___8 {
        curscroll = curscroll;
    } else {
        if cur - (onscreen - 1 as libc::c_int) > 0 as libc::c_int {
            tmp___7 = cur - (onscreen - 1 as libc::c_int);
        } else {
            tmp___7 = 0 as libc::c_int;
        }
        curscroll = tmp___7;
    };
}
unsafe extern "C" fn handle_screen_move(mut sel: action) {
    let mut onscreen: libc::c_int = 0;
    let mut input: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut index___0: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut current_block_84: u64;
    match sel as libc::c_uint {
        4 => {
            if cfg.rollover() != 0 {
                move_cursor((cur + 1 as libc::c_int) % ndents, 0 as libc::c_int);
            } else if cur != ndents - 1 as libc::c_int {
                move_cursor((cur + 1 as libc::c_int) % ndents, 0 as libc::c_int);
            }
        }
        5 => {
            if cfg.rollover() != 0 {
                move_cursor(
                    (cur + ndents - 1 as libc::c_int) % ndents,
                    0 as libc::c_int,
                );
            } else if cur != 0 {
                move_cursor(
                    (cur + ndents - 1 as libc::c_int) % ndents,
                    0 as libc::c_int,
                );
            }
        }
        6 => {
            onscreen = xlines as libc::c_int - 4 as libc::c_int;
            move_cursor(curscroll + (onscreen - 1 as libc::c_int), 1 as libc::c_int);
            curscroll += onscreen - 1 as libc::c_int;
        }
        8 => {
            onscreen = xlines as libc::c_int - 4 as libc::c_int;
            move_cursor(curscroll + (onscreen - 1 as libc::c_int), 1 as libc::c_int);
            curscroll += onscreen >> 1 as libc::c_int;
        }
        7 => {
            onscreen = xlines as libc::c_int - 4 as libc::c_int;
            move_cursor(curscroll, 1 as libc::c_int);
            curscroll -= onscreen - 1 as libc::c_int;
        }
        9 => {
            onscreen = xlines as libc::c_int - 4 as libc::c_int;
            move_cursor(curscroll, 1 as libc::c_int);
            curscroll -= onscreen >> 1 as libc::c_int;
        }
        13 => {
            tmp = xreadline(
                0 as *mut libc::c_void as *const libc::c_char,
                b"jump (+n/-n/n): \0" as *const u8 as *const libc::c_char,
            );
            input = tmp;
            if !input.is_null() {
                if !(*input == 0) {
                    if *input.offset(0 as libc::c_int as isize) as libc::c_int
                        == 45 as libc::c_int
                    {
                        tmp___0 = atoi(
                            input.offset(1 as libc::c_int as isize)
                                as *const libc::c_char,
                        );
                        cur -= tmp___0;
                        if cur < 0 as libc::c_int {
                            cur = 0 as libc::c_int;
                        }
                        current_block_84 = 1924505913685386279;
                    } else if *input.offset(0 as libc::c_int as isize) as libc::c_int
                            == 43 as libc::c_int
                        {
                        tmp___1 = atoi(
                            input.offset(1 as libc::c_int as isize)
                                as *const libc::c_char,
                        );
                        cur += tmp___1;
                        if cur >= ndents {
                            cur = ndents - 1 as libc::c_int;
                        }
                        current_block_84 = 1924505913685386279;
                    } else {
                        tmp___2 = atoi(input as *const libc::c_char);
                        index___0 = tmp___2;
                        if index___0 < 1 as libc::c_int {
                            current_block_84 = 5023038348526654800;
                        } else if index___0 > ndents {
                            current_block_84 = 5023038348526654800;
                        } else {
                            cur = index___0 - 1 as libc::c_int;
                            current_block_84 = 1924505913685386279;
                        }
                    }
                    match current_block_84 {
                        5023038348526654800 => {}
                        _ => {
                            onscreen = xlines as libc::c_int - 4 as libc::c_int;
                            move_cursor(cur, 1 as libc::c_int);
                            curscroll -= onscreen >> 1 as libc::c_int;
                        }
                    }
                }
            }
        }
        10 => {
            move_cursor(0 as libc::c_int, 1 as libc::c_int);
        }
        11 => {
            move_cursor(ndents - 1 as libc::c_int, 1 as libc::c_int);
        }
        _ => {
            tmp___3 = get_input(messages[38 as libc::c_int as usize]);
            c = tmp___3;
            if !(c == 0) {
                if c >= 97 as libc::c_int {
                    if c <= 122 as libc::c_int {
                        c = c - 97 as libc::c_int + 65 as libc::c_int;
                    } else {
                        c = c;
                    }
                } else {
                    c = c;
                }
                if *(*pdents.offset(cur as isize)).name as libc::c_int
                    >= 97 as libc::c_int
                {
                    if *(*pdents.offset(cur as isize)).name as libc::c_int
                        <= 122 as libc::c_int
                    {
                        tmp___6 = *(*pdents.offset(cur as isize)).name as libc::c_int
                            - 97 as libc::c_int + 65 as libc::c_int;
                    } else {
                        tmp___6 = *(*pdents.offset(cur as isize)).name as libc::c_int;
                    }
                } else {
                    tmp___6 = *(*pdents.offset(cur as isize)).name as libc::c_int;
                }
                if c == tmp___6 {
                    tmp___5 = cur + 1 as libc::c_int;
                } else {
                    tmp___5 = 0 as libc::c_int;
                }
                r = tmp___5;
                while r < ndents {
                    if c == 39 as libc::c_int {
                        if ((*pdents.offset(r as isize)).__annonCompField18).flags()
                            & 1 as libc::c_ulonglong == 0
                        {
                            move_cursor(r % ndents, 0 as libc::c_int);
                            break;
                        }
                    }
                    if *(*pdents.offset(r as isize)).name as libc::c_int
                        >= 97 as libc::c_int
                    {
                        if *(*pdents.offset(r as isize)).name as libc::c_int
                            <= 122 as libc::c_int
                        {
                            tmp___7 = *(*pdents.offset(r as isize)).name as libc::c_int
                                - 97 as libc::c_int + 65 as libc::c_int;
                        } else {
                            tmp___7 = *(*pdents.offset(r as isize)).name as libc::c_int;
                        }
                    } else {
                        tmp___7 = *(*pdents.offset(r as isize)).name as libc::c_int;
                    }
                    if c == tmp___7 {
                        move_cursor(r % ndents, 0 as libc::c_int);
                        break;
                    } else {
                        r += 1;
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn handle_openwith(
    mut path: *const libc::c_char,
    mut name: *const libc::c_char,
    mut newpath: *mut libc::c_char,
    mut tmp: *mut libc::c_char,
) {
    let mut r: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    tmp___0 = get_input(messages[12 as libc::c_int as usize]);
    r = tmp___0;
    if r == 99 as libc::c_int {
        r = 9 as libc::c_int;
    } else {
        if r == 103 as libc::c_int {
            tmp___1 = 7 as libc::c_int;
        } else {
            tmp___1 = 0 as libc::c_int;
        }
        r = tmp___1;
    }
    if r != 0 {
        mkpath(path, name, newpath);
        spawn(
            tmp,
            newpath,
            0 as *mut libc::c_void as *mut libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_char,
            r as ushort_t,
        );
    }
}
unsafe extern "C" fn copynextname(mut lastname: *mut libc::c_char) {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    if cur != 0 {
        if cur != ndents - 1 as libc::c_int {
            tmp = 1 as libc::c_int;
        } else {
            tmp = -(1 as libc::c_int);
        }
        cur += tmp;
        if ndents != 0 {
            tmp___0 = (*pdents.offset(cur as isize)).name as *const libc::c_char;
        } else {
            tmp___0 = b"\0\0" as *const u8 as *const libc::c_char;
        }
        xstrsncpy(lastname, tmp___0, 256 as libc::c_int as size_t);
    } else {
        *lastname.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    };
}
unsafe extern "C" fn handle_context_switch(mut sel: action) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = -(1 as libc::c_int);
    match sel as libc::c_uint {
        21 | 20 => {
            r = cfg.curctx() as libc::c_int;
            if sel as libc::c_uint == 20 as libc::c_uint {
                loop {
                    r = r + 1 as libc::c_int & -(5 as libc::c_int);
                    if (g_ctx[r as usize].c_cfg).ctxactive() != 0 {
                        break;
                    }
                }
            } else {
                loop {
                    r = r + 1 as libc::c_int & -(5 as libc::c_int);
                    if !((g_ctx[r as usize].c_cfg).ctxactive() != 0) {
                        break;
                    }
                    if !(r as uint_t != cfg.curctx()) {
                        break;
                    }
                }
                if r as uint_t == cfg.curctx() {
                    loop {
                        r = r + 3 as libc::c_int & 3 as libc::c_int;
                        if (g_ctx[r as usize].c_cfg).ctxactive() != 0 {
                            break;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if sel as libc::c_uint >= 22 as libc::c_uint {
        r = (sel as libc::c_uint).wrapping_sub(22 as libc::c_uint) as libc::c_int;
    }
    if cfg.curctx() == r as uint_t {
        if sel as libc::c_uint == 20 as libc::c_uint {
            if r == 3 as libc::c_int {
                r = 0 as libc::c_int;
            } else {
                r += 1;
            }
        } else if sel as libc::c_uint == 21 as libc::c_uint {
            if r == 0 as libc::c_int {
                r = 3 as libc::c_int;
            } else {
                r -= 1;
            }
        } else {
            return -(1 as libc::c_int)
        }
    }
    return r;
}
unsafe extern "C" fn set_sort_flags(mut r: libc::c_int) -> libc::c_int {
    let mut session: bool = false;
    let mut reverse: bool = false;
    let mut tmp: libc::c_int = 0;
    session = r == 0 as libc::c_int;
    reverse = 0 as libc::c_int != 0;
    if r >= 65 as libc::c_int {
        if r <= 90 as libc::c_int {
            if r != 82 as libc::c_int {
                if r != 67 as libc::c_int {
                    reverse = 1 as libc::c_int != 0;
                    if r >= 65 as libc::c_int {
                        if r <= 90 as libc::c_int {
                            r = r - 65 as libc::c_int + 97 as libc::c_int;
                        } else {
                            r = r;
                        }
                    } else {
                        r = r;
                    }
                }
            }
        }
    }
    if session {
        if cfg.apparentsz() != 0 {
            cfg.set_apparentsz(0 as libc::c_int as uint_t);
            r = 'a' as i32;
        } else if cfg.blkorder() != 0 {
            cfg.set_blkorder(0 as libc::c_int as uint_t);
            r = 'd' as i32;
        }
        if cfg.version() != 0 {
            namecmpfn = Some(
                xstrverscasecmp
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *const libc::c_char,
                    ) -> libc::c_int,
            );
        }
        if cfg.reverse() != 0 {
            entrycmpfn = Some(
                reventrycmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            );
        }
    } else if r == 20 as libc::c_int {
        if cfg.timeorder() != 0 {
            r = 's' as i32;
        } else if cfg.sizeorder() != 0 {
            r = 'c' as i32;
        } else {
            r = 't' as i32;
        }
    }
    let mut current_block_119: u64;
    match r {
        97 => {
            cfg.set_apparentsz(cfg.apparentsz() ^ 1 as libc::c_uint as uint_t);
            if cfg.apparentsz() != 0 {
                cfg.set_blkorder(1 as libc::c_int as uint_t);
                blk_shift = 0 as libc::c_int as uchar_t;
            } else {
                cfg.set_blkorder(0 as libc::c_int as uint_t);
            }
            current_block_119 = 12820311850980724778;
        }
        100 => {
            current_block_119 = 12820311850980724778;
        }
        99 => {
            cfg.set_timeorder(0 as libc::c_int as uint_t);
            cfg.set_sizeorder(0 as libc::c_int as uint_t);
            cfg.set_apparentsz(0 as libc::c_int as uint_t);
            cfg.set_blkorder(0 as libc::c_int as uint_t);
            cfg.set_extnorder(0 as libc::c_int as uint_t);
            cfg.set_reverse(0 as libc::c_int as uint_t);
            cfg.set_version(0 as libc::c_int as uint_t);
            entrycmpfn = Some(
                entrycmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            );
            namecmpfn = Some(
                xstricmp
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *const libc::c_char,
                    ) -> libc::c_int,
            );
            current_block_119 = 14141370668937312244;
        }
        101 => {
            cfg.set_extnorder(cfg.extnorder() ^ 1 as libc::c_uint as uint_t);
            cfg.set_sizeorder(0 as libc::c_int as uint_t);
            cfg.set_timeorder(0 as libc::c_int as uint_t);
            cfg.set_apparentsz(0 as libc::c_int as uint_t);
            cfg.set_blkorder(0 as libc::c_int as uint_t);
            cfg.set_reverse(0 as libc::c_int as uint_t);
            entrycmpfn = Some(
                entrycmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            );
            current_block_119 = 14141370668937312244;
        }
        114 => {
            cfg.set_reverse(cfg.reverse() ^ 1 as libc::c_uint as uint_t);
            if cfg.reverse() != 0 {
                entrycmpfn = Some(
                    reventrycmp
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                );
            } else {
                entrycmpfn = Some(
                    entrycmp
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                );
            }
            current_block_119 = 14141370668937312244;
        }
        115 => {
            cfg.set_sizeorder(cfg.sizeorder() ^ 1 as libc::c_uint as uint_t);
            cfg.set_timeorder(0 as libc::c_int as uint_t);
            cfg.set_apparentsz(0 as libc::c_int as uint_t);
            cfg.set_blkorder(0 as libc::c_int as uint_t);
            cfg.set_extnorder(0 as libc::c_int as uint_t);
            cfg.set_reverse(0 as libc::c_int as uint_t);
            entrycmpfn = Some(
                entrycmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            );
            current_block_119 = 14141370668937312244;
        }
        116 => {
            cfg.set_timeorder(cfg.timeorder() ^ 1 as libc::c_uint as uint_t);
            cfg.set_sizeorder(0 as libc::c_int as uint_t);
            cfg.set_apparentsz(0 as libc::c_int as uint_t);
            cfg.set_blkorder(0 as libc::c_int as uint_t);
            cfg.set_extnorder(0 as libc::c_int as uint_t);
            cfg.set_reverse(0 as libc::c_int as uint_t);
            entrycmpfn = Some(
                entrycmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            );
            current_block_119 = 14141370668937312244;
        }
        118 => {
            cfg.set_version(cfg.version() ^ 1 as libc::c_uint as uint_t);
            if cfg.version() != 0 {
                namecmpfn = Some(
                    xstrverscasecmp
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                        ) -> libc::c_int,
                );
            } else {
                namecmpfn = Some(
                    xstricmp
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                        ) -> libc::c_int,
                );
            }
            cfg.set_timeorder(0 as libc::c_int as uint_t);
            cfg.set_sizeorder(0 as libc::c_int as uint_t);
            cfg.set_apparentsz(0 as libc::c_int as uint_t);
            cfg.set_blkorder(0 as libc::c_int as uint_t);
            cfg.set_extnorder(0 as libc::c_int as uint_t);
            current_block_119 = 14141370668937312244;
        }
        _ => return 0 as libc::c_int,
    }
    match current_block_119 {
        12820311850980724778 => {
            if r == 100 as libc::c_int {
                if cfg.apparentsz() == 0 {
                    cfg.set_blkorder(cfg.blkorder() ^ 1 as libc::c_uint as uint_t);
                }
                cfg.set_apparentsz(0 as libc::c_int as uint_t);
                tmp = ffs(512 as libc::c_int);
                blk_shift = (tmp - 1 as libc::c_int) as uchar_t;
            }
            if cfg.blkorder() != 0 {
                cfg.set_showdetail(1 as libc::c_int as uint_t);
            }
            cfg.set_timeorder(0 as libc::c_int as uint_t);
            cfg.set_sizeorder(0 as libc::c_int as uint_t);
            cfg.set_extnorder(0 as libc::c_int as uint_t);
            if !session {
                cfg.set_reverse(0 as libc::c_int as uint_t);
                entrycmpfn = Some(
                    entrycmp
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                );
            }
            endselection(1 as libc::c_int != 0);
        }
        _ => {}
    }
    if reverse {
        cfg.set_reverse(1 as libc::c_int as uint_t);
        entrycmpfn = Some(
            reventrycmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        );
    }
    cfgsort[cfg.curctx() as usize] = r as uchar_t;
    return r;
}
unsafe extern "C" fn set_time_type(mut presel: *mut libc::c_int) -> bool {
    let mut current_block: u64;
    let mut ret: bool = false;
    let mut buf: [libc::c_char; 32] = [0; 32];
    let mut tmp: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    ret = 0 as libc::c_int != 0;
    buf[0 as libc::c_int as usize] = '\'' as i32 as libc::c_char;
    buf[1 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    buf[2 as libc::c_int as usize] = '\'' as i32 as libc::c_char;
    buf[3 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    buf[4 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    buf[5 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    buf[6 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    buf[7 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    buf[8 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    buf[9 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    buf[10 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    buf[11 as libc::c_int as usize] = '\'' as i32 as libc::c_char;
    buf[12 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    buf[13 as libc::c_int as usize] = '\'' as i32 as libc::c_char;
    buf[14 as libc::c_int as usize] = 'h' as i32 as libc::c_char;
    buf[15 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    buf[16 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    buf[17 as libc::c_int as usize] = 'g' as i32 as libc::c_char;
    buf[18 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    buf[19 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    buf[20 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    buf[21 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    buf[22 as libc::c_int as usize] = '\'' as i32 as libc::c_char;
    buf[23 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    buf[24 as libc::c_int as usize] = '\'' as i32 as libc::c_char;
    buf[25 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    buf[26 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
    buf[27 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    buf[28 as libc::c_int as usize] = '[' as i32 as libc::c_char;
    buf[29 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    buf[30 as libc::c_int as usize] = ']' as i32 as libc::c_char;
    buf[31 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    if cfg.timetype() == 2 as libc::c_uint {
        buf[(::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
            .wrapping_sub(3 as libc::c_ulong) as usize] = 'm' as i32 as libc::c_char;
    } else {
        if cfg.timetype() == 0 as libc::c_uint {
            tmp = 'a' as i32;
        } else {
            tmp = 'c' as i32;
        }
        buf[(::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
            .wrapping_sub(3 as libc::c_ulong) as usize] = tmp as libc::c_char;
    }
    tmp___0 = get_input(buf.as_mut_ptr() as *const libc::c_char);
    r = tmp___0;
    if r == 97 as libc::c_int {
        current_block = 5482623550091817796;
    } else if r == 99 as libc::c_int {
        current_block = 5482623550091817796;
    } else if r == 109 as libc::c_int {
        current_block = 5482623550091817796;
    } else {
        r = 40 as libc::c_int;
        current_block = 12829669402821218572;
    }
    match current_block {
        5482623550091817796 => {
            if r == 109 as libc::c_int {
                r = 2 as libc::c_int;
            } else {
                if r == 97 as libc::c_int {
                    tmp___1 = 0 as libc::c_int;
                } else {
                    tmp___1 = 1 as libc::c_int;
                }
                r = tmp___1;
            }
            if cfg.timetype() != r as uint_t {
                cfg.set_timetype(r as uint_t);
                if cfg.filtermode() != 0 {
                    *presel = '/' as i32;
                } else if g_ctx[cfg.curctx() as usize].c_fltr[1 as libc::c_int as usize]
                        != 0
                    {
                    *presel = '/' as i32;
                }
                ret = 1 as libc::c_int != 0;
            } else {
                r = 41 as libc::c_int;
            }
        }
        _ => {}
    }
    if !ret {
        printwait(messages[r as usize], presel);
    }
    return ret;
}
unsafe extern "C" fn statusbar(mut path: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pent: pEntry = 0 as *mut entry;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: bool = false;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 24] = [0; 24];
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: size_t = 0;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: libc::c_int = 0;
    let mut sort: [libc::c_char; 6] = [0; 6];
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___9: ssize_t = 0;
    let mut tmp___10: off_t = 0;
    let mut tmp___11: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut y: libc::c_int = 0;
    let mut tmp___12: *mut libc::c_char = 0 as *mut libc::c_char;
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
    let mut tmp___13: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___14: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___15: libc::c_int = 0;
    i = 0 as libc::c_int;
    len = 0 as libc::c_int;
    pent = pdents.offset(cur as isize);
    if ndents == 0 {
        printmsg(b"0/0\0" as *const u8 as *const libc::c_char);
        return;
    }
    if (*pent).mode & 61440 as libc::c_uint == 32768 as libc::c_uint {
        i = ((*pent).__annonCompField18).nlen().wrapping_sub(1 as libc::c_ulonglong)
            as libc::c_int;
        ptr = xextension((*pent).name as *const libc::c_char, i as size_t);
        if !ptr.is_null() {
            len = (i as libc::c_long - ptr.offset_from((*pent).name) as libc::c_long)
                as libc::c_int;
        }
        if ptr.is_null() {
            ptr = b"\x08\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else if len > 5 as libc::c_int {
            ptr = b"\x08\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else if len < 2 as libc::c_int {
            ptr = b"\x08\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    } else {
        ptr = b"\x08\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    wattr_on(
        stdscr,
        cfg.curctx().wrapping_add(1 as libc::c_uint) << 8 as libc::c_int
            & ((1 as libc::c_uint) << 8 as libc::c_int).wrapping_sub(1 as libc::c_uint)
                << 8 as libc::c_int,
        0 as *mut libc::c_void,
    );
    if cfg.fileinfo() != 0 {
        tmp___0 = get_output(
            b"file\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"-b\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*pdents.offset(cur as isize)).name,
            -(1 as libc::c_int),
            0 as libc::c_int != 0,
            0 as libc::c_int != 0,
        );
        if tmp___0 {
            tmp = wmove(
                stdscr,
                xlines as libc::c_int - 2 as libc::c_int,
                2 as libc::c_int,
            );
            if !(tmp == -(1 as libc::c_int)) {
                waddnstr(
                    stdscr,
                    g_buf.as_mut_ptr() as *const libc::c_char,
                    -(1 as libc::c_int),
                );
            }
        }
    }
    wmove(stdscr, xlines as libc::c_int - 1 as libc::c_int, 0 as libc::c_int);
    tmp___1 = xitoa(ndents as uint_t);
    printw(
        b"%d/%s \0" as *const u8 as *const libc::c_char,
        cur + 1 as libc::c_int,
        tmp___1,
    );
    let mut current_block_52: u64;
    if g_state.selmode() != 0 {
        current_block_52 = 1497488620103345003;
    } else if nselected != 0 {
        current_block_52 = 1497488620103345003;
    } else {
        current_block_52 = 13321564401369230990;
    }
    match current_block_52 {
        1497488620103345003 => {
            wattr_on(
                stdscr,
                (1 as libc::c_uint) << 18 as libc::c_int,
                0 as *mut libc::c_void,
            );
            waddch(stdscr, ' ' as i32 as chtype);
            if g_state.rangesel() != 0 {
                waddch(stdscr, '*' as i32 as chtype);
            } else if g_state.selmode() != 0 {
                waddch(stdscr, '+' as i32 as chtype);
            }
            if nselected != 0 {
                tmp___2 = xitoa(nselected as uint_t);
                waddnstr(stdscr, tmp___2 as *const libc::c_char, -(1 as libc::c_int));
            }
            waddch(stdscr, ' ' as i32 as chtype);
            wattr_off(
                stdscr,
                (1 as libc::c_uint) << 18 as libc::c_int,
                0 as *mut libc::c_void,
            );
            waddch(stdscr, ' ' as i32 as chtype);
        }
        _ => {}
    }
    if cfg.blkorder() != 0 {
        tmp___3 = coolsize(dir_blocks << blk_shift as libc::c_int);
        xstrsncpy(
            buf.as_mut_ptr(),
            tmp___3 as *const libc::c_char,
            12 as libc::c_int as size_t,
        );
        tmp___4 = get_fs_info(path as *const libc::c_char, 0 as libc::c_int as uchar_t);
        tmp___5 = coolsize(tmp___4 as off_t);
        if cfg.apparentsz() != 0 {
            tmp___6 = 'a' as i32;
        } else {
            tmp___6 = 'd' as i32;
        }
        printw(
            b"%cu:%s avail:%s files:%llu %lluB %s\n\0" as *const u8
                as *const libc::c_char,
            tmp___6,
            buf.as_mut_ptr(),
            tmp___5,
            num_files,
            ((*pent).__annonCompField18).blocks() << blk_shift as libc::c_int,
            ptr,
        );
    } else {
        sort[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        sort[1 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        sort[2 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        sort[3 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        sort[4 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        sort[5 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        tmp___7 = getorderstr(sort.as_mut_ptr());
        if tmp___7 != 0 {
            waddnstr(
                stdscr,
                sort.as_mut_ptr() as *const libc::c_char,
                -(1 as libc::c_int),
            );
        }
        print_time(
            &mut (*pent).sec as *mut time_t as *const time_t,
            ((*pent).__annonCompField18).flags() as uchar_t,
        );
        waddch(stdscr, ' ' as i32 as chtype);
        tmp___8 = get_lsperms((*pent).mode);
        waddnstr(stdscr, tmp___8 as *const libc::c_char, -(1 as libc::c_int));
        waddch(stdscr, ' ' as i32 as chtype);
        if (*pent).mode & 61440 as libc::c_uint == 40960 as libc::c_uint {
            if cfg.fileinfo() == 0 {
                tmp___9 = readlink(
                    (*pent).name as *const libc::c_char,
                    g_buf.as_mut_ptr(),
                    4096 as libc::c_int as size_t,
                );
                i = tmp___9 as libc::c_int;
                if i >= 0 as libc::c_int {
                    tmp___10 = i as off_t;
                } else {
                    tmp___10 = (*pent).size;
                }
                tmp___11 = coolsize(tmp___10);
                waddnstr(stdscr, tmp___11 as *const libc::c_char, -(1 as libc::c_int));
                if i > 1 as libc::c_int {
                    waddnstr(
                        stdscr,
                        b" ->\0" as *const u8 as *const libc::c_char,
                        -(1 as libc::c_int),
                    );
                    if 0 as *const libc::c_void as libc::c_ulong
                        != stdscr as *const libc::c_void as libc::c_ulong
                    {
                        len = (*stdscr)._cury as libc::c_int;
                    } else {
                        len = -(1 as libc::c_int);
                    }
                    if 0 as *const libc::c_void as libc::c_ulong
                        != stdscr as *const libc::c_void as libc::c_ulong
                    {
                        y = (*stdscr)._curx as libc::c_int;
                    } else {
                        y = -(1 as libc::c_int);
                    }
                    if i < xcols as libc::c_int - y {
                        i = i;
                    } else {
                        i = xcols as libc::c_int - y;
                    }
                    g_buf[i as usize] = '\u{0}' as i32 as libc::c_char;
                    waddnstr(
                        stdscr,
                        g_buf.as_mut_ptr() as *const libc::c_char,
                        -(1 as libc::c_int),
                    );
                }
            }
        } else {
            tmp___12 = coolsize((*pent).size);
            waddnstr(stdscr, tmp___12 as *const libc::c_char, -(1 as libc::c_int));
            waddch(stdscr, ' ' as i32 as chtype);
            waddnstr(stdscr, ptr as *const libc::c_char, -(1 as libc::c_int));
            if ((*pent).__annonCompField18).flags() & 2 as libc::c_ulonglong != 0 {
                tmp___15 = stat(
                    (*pent).name as *const libc::c_char,
                    &mut sb as *mut stat,
                );
                if tmp___15 != -(1 as libc::c_int) {
                    waddch(stdscr, ' ' as i32 as chtype);
                    tmp___13 = xitoa(sb.st_nlink as libc::c_int as uint_t);
                    waddnstr(
                        stdscr,
                        tmp___13 as *const libc::c_char,
                        -(1 as libc::c_int),
                    );
                    waddch(stdscr, '-' as i32 as chtype);
                    tmp___14 = xitoa(sb.st_ino as libc::c_int as uint_t);
                    waddnstr(
                        stdscr,
                        tmp___14 as *const libc::c_char,
                        -(1 as libc::c_int),
                    );
                }
            }
        }
        wclrtoeol(stdscr);
    }
    wattr_off(
        stdscr,
        cfg.curctx().wrapping_add(1 as libc::c_uint) << 8 as libc::c_int
            & ((1 as libc::c_uint) << 8 as libc::c_int).wrapping_sub(1 as libc::c_uint)
                << 8 as libc::c_int,
        0 as *mut libc::c_void,
    );
    wmove(stdscr, cur + 2 as libc::c_int - curscroll, 0 as libc::c_int);
}
#[inline]
unsafe extern "C" fn markhovered() {
    if cfg.showdetail() != 0 {
        if ndents != 0 {
            wmove(stdscr, cur + 2 as libc::c_int - curscroll, 0 as libc::c_int);
            waddch(
                stdscr,
                62 as libc::c_uint | (1 as libc::c_uint) << 21 as libc::c_int,
            );
        }
    }
}
unsafe extern "C" fn adjust_cols(mut n: libc::c_int) -> libc::c_int {
    if cfg.showdetail() != 0 {
        if n < 36 as libc::c_int {
            cfg.set_showdetail(cfg.showdetail() ^ 1 as libc::c_uint as uint_t);
        } else {
            n -= 32 as libc::c_int;
        }
    }
    return n - 2 as libc::c_int;
}
unsafe extern "C" fn draw_line(mut ncols: libc::c_int) {
    let mut dir: bool = false;
    dir = 0 as libc::c_int != 0;
    ncols = adjust_cols(ncols);
    if g_state.oldcolor() != 0 {
        if ((*pdents.offset(last as isize)).__annonCompField18).flags()
            & 1 as libc::c_ulonglong != 0
        {
            wattr_on(
                stdscr,
                cfg.curctx().wrapping_add(1 as libc::c_uint) << 8 as libc::c_int
                    & ((1 as libc::c_uint) << 8 as libc::c_int)
                        .wrapping_sub(1 as libc::c_uint) << 8 as libc::c_int
                    | (1 as libc::c_uint) << 21 as libc::c_int,
                0 as *mut libc::c_void,
            );
            dir = 1 as libc::c_int != 0;
        }
    }
    wmove(stdscr, 2 as libc::c_int + last - curscroll, 0 as libc::c_int);
    printent(
        pdents.offset(last as isize) as *const entry,
        ncols as uint_t,
        0 as libc::c_int != 0,
    );
    let mut current_block_18: u64;
    if g_state.oldcolor() != 0 {
        if ((*pdents.offset(cur as isize)).__annonCompField18).flags()
            & 1 as libc::c_ulonglong != 0
        {
            if !dir {
                wattr_on(
                    stdscr,
                    cfg.curctx().wrapping_add(1 as libc::c_uint) << 8 as libc::c_int
                        & ((1 as libc::c_uint) << 8 as libc::c_int)
                            .wrapping_sub(1 as libc::c_uint) << 8 as libc::c_int
                        | (1 as libc::c_uint) << 21 as libc::c_int,
                    0 as *mut libc::c_void,
                );
                dir = 1 as libc::c_int != 0;
            }
            current_block_18 = 8457315219000651999;
        } else {
            current_block_18 = 15564511451906810276;
        }
    } else {
        current_block_18 = 15564511451906810276;
    }
    match current_block_18 {
        15564511451906810276 => {
            if dir {
                wattr_off(
                    stdscr,
                    cfg.curctx().wrapping_add(1 as libc::c_uint) << 8 as libc::c_int
                        & ((1 as libc::c_uint) << 8 as libc::c_int)
                            .wrapping_sub(1 as libc::c_uint) << 8 as libc::c_int
                        | (1 as libc::c_uint) << 21 as libc::c_int,
                    0 as *mut libc::c_void,
                );
                dir = 0 as libc::c_int != 0;
            }
        }
        _ => {}
    }
    wmove(stdscr, 2 as libc::c_int + cur - curscroll, 0 as libc::c_int);
    printent(
        pdents.offset(cur as isize) as *const entry,
        ncols as uint_t,
        1 as libc::c_int != 0,
    );
    if dir {
        wattr_off(
            stdscr,
            cfg.curctx().wrapping_add(1 as libc::c_uint) << 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint) << 8 as libc::c_int
                | (1 as libc::c_uint) << 21 as libc::c_int,
            0 as *mut libc::c_void,
        );
    }
    markhovered();
}
unsafe extern "C" fn redraw(mut path: *mut libc::c_char) {
    let mut ncols: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut onscreen: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tmp___0: chtype = 0;
    let mut in_home: bool = false;
    let mut tmp___1: bool = false;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: size_t = 0;
    let mut base: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut len: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    if 0 as *const libc::c_void as libc::c_ulong
        != stdscr as *const libc::c_void as libc::c_ulong
    {
        xlines = ((*stdscr)._maxy as libc::c_int + 1 as libc::c_int) as ushort_t;
    } else {
        xlines = -(1 as libc::c_int) as ushort_t;
    }
    if 0 as *const libc::c_void as libc::c_ulong
        != stdscr as *const libc::c_void as libc::c_ulong
    {
        xcols = ((*stdscr)._maxx as libc::c_int + 1 as libc::c_int) as ushort_t;
    } else {
        xcols = -(1 as libc::c_int) as ushort_t;
    }
    if xcols as libc::c_int <= 4096 as libc::c_int {
        tmp = xcols as libc::c_int;
    } else {
        tmp = 4096 as libc::c_int;
    }
    ncols = tmp;
    onscreen = xlines as libc::c_int - 4 as libc::c_int;
    j = 1 as libc::c_int;
    if g_state.move_0() != 0 {
        g_state.set_move_0(0 as libc::c_int as uint_t);
        if ndents != 0 {
            if last_curscroll == curscroll {
                draw_line(ncols);
                return;
            }
        }
    }
    werase(stdscr);
    move_cursor(cur, 1 as libc::c_int);
    if ncols <= 8 as libc::c_int {
        printmsg(messages[29 as libc::c_int as usize]);
        return;
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if (g_ctx[i as usize].c_cfg).ctxactive() == 0 {
            waddch(stdscr, (i + 49 as libc::c_int) as chtype);
        } else {
            if cfg.curctx() != i as uint_t {
                tmp___0 = (1 as libc::c_uint) << 17 as libc::c_int;
            } else {
                tmp___0 = (1 as libc::c_uint) << 18 as libc::c_int;
            }
            waddch(
                stdscr,
                (i + 49 as libc::c_int) as libc::c_uint
                    | (((i + 1 as libc::c_int) as chtype) << 8 as libc::c_int
                        & ((1 as libc::c_uint) << 8 as libc::c_int)
                            .wrapping_sub(1 as libc::c_uint) << 8 as libc::c_int
                        | (1 as libc::c_uint) << 21 as libc::c_int | tmp___0),
            );
        }
        waddch(stdscr, ' ' as i32 as chtype);
        i += 1;
    }
    wattr_on(
        stdscr,
        (1 as libc::c_uint) << 17 as libc::c_int
            | cfg.curctx().wrapping_add(1 as libc::c_uint) << 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint) << 8 as libc::c_int,
        0 as *mut libc::c_void,
    );
    tmp___1 = set_tilde_in_path(path);
    in_home = tmp___1;
    if in_home {
        tmp___2 = path.offset((homelen as libc::c_int - 1 as libc::c_int) as isize);
    } else {
        tmp___2 = path;
    }
    ptr = tmp___2;
    tmp___3 = xstrlen(ptr as *const libc::c_char);
    i = tmp___3 as libc::c_int;
    if i + 8 as libc::c_int <= ncols {
        waddnstr(stdscr, ptr as *const libc::c_char, ncols - 8 as libc::c_int);
    } else {
        tmp___4 = xmemrchr(ptr as *mut uchar_t, '/' as i32 as uchar_t, i as size_t);
        base = tmp___4 as *mut libc::c_char;
        if in_home {
            waddch(stdscr, *ptr as chtype);
            ptr = ptr.offset(1);
            i = 1 as libc::c_int;
        } else {
            i = 0 as libc::c_int;
        }
        if !ptr.is_null() {
            if base as libc::c_ulong != ptr as libc::c_ulong {
                while (ptr as libc::c_ulong) < base as libc::c_ulong {
                    if *ptr as libc::c_int == 47 as libc::c_int {
                        i += 2 as libc::c_int;
                        if ncols < i + 8 as libc::c_int {
                            base = 0 as *mut libc::c_void as *mut libc::c_char;
                            break;
                        } else {
                            waddch(stdscr, *ptr as chtype);
                            ptr = ptr.offset(1);
                            waddch(stdscr, *ptr as chtype);
                        }
                    }
                    ptr = ptr.offset(1);
                }
            }
        }
        if !base.is_null() {
            waddnstr(
                stdscr,
                base as *const libc::c_char,
                ncols - (8 as libc::c_int + i),
            );
        }
    }
    if in_home {
        reset_tilde_in_path(path);
    }
    wattr_off(
        stdscr,
        (1 as libc::c_uint) << 17 as libc::c_int
            | cfg.curctx().wrapping_add(1 as libc::c_uint) << 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint) << 8 as libc::c_int,
        0 as *mut libc::c_void,
    );
    if curscroll > 0 as libc::c_int {
        wmove(stdscr, 1 as libc::c_int, 0 as libc::c_int);
        waddch(stdscr, '^' as i32 as chtype);
    }
    if g_state.oldcolor() != 0 {
        wattr_on(
            stdscr,
            cfg.curctx().wrapping_add(1 as libc::c_uint) << 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint) << 8 as libc::c_int
                | (1 as libc::c_uint) << 21 as libc::c_int,
            0 as *mut libc::c_void,
        );
        g_state.set_dircolor(1 as libc::c_int as uint_t);
    }
    if onscreen + curscroll < ndents {
        onscreen += curscroll;
    } else {
        onscreen = ndents;
    }
    ncols = adjust_cols(ncols);
    tmp___5 = scanselforpath(path as *const libc::c_char, 0 as libc::c_int != 0);
    len = tmp___5;
    i = curscroll;
    while i < onscreen {
        j += 1;
        wmove(stdscr, j, 0 as libc::c_int);
        if len != 0 {
            findmarkentry(len as size_t, pdents.offset(i as isize));
        }
        printent(pdents.offset(i as isize) as *const entry, ncols as uint_t, i == cur);
        i += 1;
    }
    if g_state.dircolor() != 0 {
        wattr_off(
            stdscr,
            cfg.curctx().wrapping_add(1 as libc::c_uint) << 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint) << 8 as libc::c_int
                | (1 as libc::c_uint) << 21 as libc::c_int,
            0 as *mut libc::c_void,
        );
        g_state.set_dircolor(0 as libc::c_int as uint_t);
    }
    if onscreen < ndents {
        wmove(stdscr, xlines as libc::c_int - 2 as libc::c_int, 0 as libc::c_int);
        waddch(stdscr, 'v' as i32 as chtype);
    }
    markhovered();
}
unsafe extern "C" fn cdprep(
    mut lastdir: *mut libc::c_char,
    mut lastname: *mut libc::c_char,
    mut path: *mut libc::c_char,
    mut newpath: *mut libc::c_char,
) -> bool {
    if !lastname.is_null() {
        *lastname.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    }
    xstrsncpy(lastdir, path as *const libc::c_char, 4096 as libc::c_int as size_t);
    xstrsncpy(path, newpath as *const libc::c_char, 4096 as libc::c_int as size_t);
    clearfilter();
    return cfg.filtermode() != 0;
}
unsafe extern "C" fn browse(
    mut ipath: *mut libc::c_char,
    mut session: *const libc::c_char,
    mut pkey: libc::c_int,
) -> bool {
    let mut current_block: u64;
    let mut newpath: [libc::c_char; 4096] = [0; 4096];
    let mut runfile: [libc::c_char; 256] = [0; 256];
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lastdir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lastname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pent: pEntry = 0 as *mut entry;
    let mut sel: action = 0 as action;
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
    let mut r: libc::c_int = 0;
    let mut presel: libc::c_int = 0;
    let mut selstartid: libc::c_int = 0;
    let mut selendid: libc::c_int = 0;
    let mut opener_flags: uchar_t = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut watch: bool = false;
    let mut cd: bool = false;
    let mut inode: ino_t = 0;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: __pid_t = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___11: bool = false;
    let mut tmp___12: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___13: size_t = 0;
    let mut tmp___14: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___15: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___16: libc::c_int = 0;
    let mut tmp___17: bool = false;
    let mut tmp___18: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___19: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___20: libc::c_int = 0;
    let mut tmp___21: libc::c_int = 0;
    let mut tmp___22: bool = false;
    let mut tmp___23: libc::c_int = 0;
    let mut tmp___24: libc::c_int = 0;
    let mut tmp___25: size_t = 0;
    let mut tmp___26: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___27: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___28: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___29: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___30: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___31: libc::c_int = 0;
    let mut tmp___32: bool = false;
    let mut tmp___33: size_t = 0;
    let mut tmp___34: bool = false;
    let mut tmp___35: bool = false;
    let mut tmp___36: bool = false;
    let mut tmp___37: bool = false;
    let mut tmp___38: bool = false;
    let mut tmp___39: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___40: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___41: libc::c_int = 0;
    let mut tmp___42: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___43: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___44: libc::c_int = 0;
    let mut tmp___45: libc::c_int = 0;
    let mut tmp___46: size_t = 0;
    let mut tmp___47: libc::c_int = 0;
    let mut tmp___48: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___49: bool = false;
    let mut tmp___50: bool = false;
    let mut tmp___51: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___52: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___53: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___54: libc::c_int = 0;
    let mut tmp___55: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___56: libc::c_int = 0;
    let mut tmp___57: libc::c_int = 0;
    let mut tmp___60: libc::c_int = 0;
    let mut tmp___61: libc::c_int = 0;
    let mut tmp___62: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___63: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___64: bool = false;
    let mut tmp___65: libc::c_int = 0;
    let mut tmp___66: bool = false;
    let mut refresh___0: bool = false;
    let mut tmp___67: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___68: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___69: bool = false;
    let mut tmp___70: bool = false;
    let mut tmp___71: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___72: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___73: size_t = 0;
    let mut tmp___74: size_t = 0;
    let mut tmp___75: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___76: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___77: libc::c_int = 0;
    let mut tmp___80: libc::c_int = 0;
    let mut tmp___81: libc::c_int = 0;
    let mut tmp___82: bool = false;
    let mut tmp___83: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___84: bool = false;
    let mut tmp___85: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___86: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___87: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut tmp___88: bool = false;
    let mut tmp___89: libc::c_int = 0;
    let mut tmp___90: libc::c_int = 0;
    let mut tmp___91: libc::c_int = 0;
    let mut tmp___92: bool = false;
    let mut tmp___93: libc::c_int = 0;
    let mut tmp___94: libc::c_int = 0;
    let mut tmp___95: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___96: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___97: libc::c_int = 0;
    let mut tmp___98: libc::c_int = 0;
    let mut tmp___99: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___100: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___101: libc::c_int = 0;
    let mut tmp___102: bool = false;
    let mut tmp___103: libc::c_int = 0;
    let mut tmp___104: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___105: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___106: libc::c_int = 0;
    let mut tmp___107: libc::c_int = 0;
    let mut tmp___108: bool = false;
    let mut tmp___109: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___110: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___111: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___112: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___113: bool = false;
    let mut tmp___114: size_t = 0;
    let mut tmp___115: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___116: bool = false;
    let mut tmp___117: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___118: libc::c_int = 0;
    let mut tmp___119: bool = false;
    let mut tmp___120: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___121: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___122: bool = false;
    let mut tmp___123: bool = false;
    let mut ctx: libc::c_int = 0;
    let mut tmp___124: bool = false;
    let mut tmp___125: *const libc::c_char = 0 as *const libc::c_char;
    r = -(1 as libc::c_int);
    selstartid = 0 as libc::c_int;
    selendid = 0 as libc::c_int;
    if cfg.cliopener() != 0 {
        tmp___0 = 9 as libc::c_int;
    } else {
        tmp___0 = 70 as libc::c_int;
    }
    opener_flags = tmp___0 as uchar_t;
    watch = 0 as libc::c_int != 0;
    cd = 1 as libc::c_int != 0;
    inode = 0 as libc::c_int as ino_t;
    atexit(Some(dentfree as unsafe extern "C" fn() -> ()));
    if 0 as *const libc::c_void as libc::c_ulong
        != stdscr as *const libc::c_void as libc::c_ulong
    {
        xlines = ((*stdscr)._maxy as libc::c_int + 1 as libc::c_int) as ushort_t;
    } else {
        xlines = -(1 as libc::c_int) as ushort_t;
    }
    if 0 as *const libc::c_void as libc::c_ulong
        != stdscr as *const libc::c_void as libc::c_ulong
    {
        xcols = ((*stdscr)._maxx as libc::c_int + 1 as libc::c_int) as ushort_t;
    } else {
        xcols = -(1 as libc::c_int) as ushort_t;
    }
    g_ctx[0 as libc::c_int as usize]
        .c_last[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    lastdir = (g_ctx[0 as libc::c_int as usize].c_last).as_mut_ptr();
    if g_state.initfile() != 0 {
        tmp___1 = xbasename(ipath);
        xstrsncpy(
            (g_ctx[0 as libc::c_int as usize].c_name).as_mut_ptr(),
            tmp___1 as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
        xdirname(ipath);
    } else {
        g_ctx[0 as libc::c_int as usize]
            .c_name[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    }
    lastname = (g_ctx[0 as libc::c_int as usize].c_name).as_mut_ptr();
    xstrsncpy(
        (g_ctx[0 as libc::c_int as usize].c_path).as_mut_ptr(),
        ipath as *const libc::c_char,
        4096 as libc::c_int as size_t,
    );
    if g_state.initfile() != 0 {
        free(initpath as *mut libc::c_void);
        ipath = getcwd(
            0 as *mut libc::c_void as *mut libc::c_char,
            0 as libc::c_int as size_t,
        );
        initpath = ipath;
    }
    path = (g_ctx[0 as libc::c_int as usize].c_path).as_mut_ptr();
    g_ctx[0 as libc::c_int as usize]
        .c_fltr[1 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    g_ctx[0 as libc::c_int as usize]
        .c_fltr[0 as libc::c_int
        as usize] = g_ctx[0 as libc::c_int as usize].c_fltr[1 as libc::c_int as usize];
    g_ctx[0 as libc::c_int as usize].c_cfg = cfg;
    runfile[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    newpath[0 as libc::c_int as usize] = runfile[0 as libc::c_int as usize];
    if pkey != 0 {
        presel = ';' as i32;
    } else {
        if cfg.filtermode() != 0 {
            tmp___2 = '/' as i32;
        } else if !session.is_null() {
            let mut current_block_55: u64;
            if g_ctx[cfg.curctx() as usize].c_fltr[0 as libc::c_int as usize]
                as libc::c_int == 47 as libc::c_int
            {
                current_block_55 = 9373269695858442403;
            } else if g_ctx[cfg.curctx() as usize].c_fltr[0 as libc::c_int as usize]
                    as libc::c_int == 92 as libc::c_int
                {
                current_block_55 = 9373269695858442403;
            } else {
                tmp___2 = 0 as libc::c_int;
                current_block_55 = 2705889988320590074;
            }
            match current_block_55 {
                9373269695858442403 => {
                    if g_ctx[cfg.curctx() as usize].c_fltr[1 as libc::c_int as usize]
                        != 0
                    {
                        tmp___2 = '/' as i32;
                    } else {
                        tmp___2 = 0 as libc::c_int;
                    }
                }
                _ => {}
            }
        } else {
            tmp___2 = 0 as libc::c_int;
        }
        presel = tmp___2;
    }
    tmp___3 = xrealloc(
        pdents as *mut libc::c_void,
        (total_dents as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<entry>() as libc::c_ulong),
    );
    pdents = tmp___3 as *mut entry;
    if pdents.is_null() {
        printerr(6626 as libc::c_int);
    }
    tmp___4 = xrealloc(pnamebuf as *mut libc::c_void, 2048 as libc::c_int as size_t);
    pnamebuf = tmp___4 as *mut libc::c_char;
    if pnamebuf.is_null() {
        printerr(6631 as libc::c_int);
    }
    if presel == 47 as libc::c_int {
        handle_key_resize();
    }
    '_begin: loop {
        tmp___5 = chdir(path as *const libc::c_char);
        if tmp___5 == -(1 as libc::c_int) {
            valid_parent(path, lastname);
            if cfg.filtermode() != 0 {
                presel = '/' as i32;
            } else {
                watch = 1 as libc::c_int != 0;
            }
        }
        let mut current_block_91: u64;
        if presel == 47 as libc::c_int {
            current_block_91 = 4977201492416208240;
        } else if watch {
            current_block_91 = 4977201492416208240;
        } else {
            current_block_91 = 3921975509081277429;
        }
        match current_block_91 {
            4977201492416208240 => {
                if inotify_wd >= 0 as libc::c_int {
                    inotify_rm_watch(inotify_fd, inotify_wd);
                    inotify_wd = -(1 as libc::c_int);
                    watch = 0 as libc::c_int != 0;
                }
            }
            _ => {}
        }
        if !order.is_null() {
            if cd {
                if cfgsort[cfg.curctx() as usize] as libc::c_int != 48 as libc::c_int {
                    if cfgsort[cfg.curctx() as usize] as libc::c_int
                        == 122 as libc::c_int
                    {
                        set_sort_flags('c' as i32);
                    }
                    let mut current_block_100: u64;
                    if cfgsort[cfg.curctx() as usize] == 0 {
                        current_block_100 = 1121699884069356717;
                    } else if cfgsort[cfg.curctx() as usize] as libc::c_int
                            == 99 as libc::c_int
                        {
                        current_block_100 = 1121699884069356717;
                    } else {
                        current_block_100 = 11227437541145425351;
                    }
                    match current_block_100 {
                        1121699884069356717 => {
                            r = get_kv_key(
                                order,
                                path,
                                maxorder,
                                11 as libc::c_int as uchar_t,
                            );
                            if r > 0 as libc::c_int {
                                set_sort_flags(r);
                                cfgsort[cfg.curctx() as usize] = 'z' as i32 as uchar_t;
                            }
                        }
                        _ => {}
                    }
                } else {
                    cfgsort[cfg.curctx() as usize] = cfgsort[4 as libc::c_int as usize];
                }
            }
        }
        cd = 1 as libc::c_int != 0;
        populate(path, lastname);
        if g_state.interrupt() != 0 {
            cfg.set_blkorder(0 as libc::c_int as uint_t);
            cfg.set_apparentsz(cfg.blkorder());
            g_state.set_interrupt(cfg.apparentsz());
            blk_shift = 9 as libc::c_int as uchar_t;
            presel = 12 as libc::c_int;
        }
        if presel != 47 as libc::c_int {
            if inotify_wd == -(1 as libc::c_int) {
                inotify_wd = inotify_add_watch(
                    inotify_fd,
                    path as *const libc::c_char,
                    INOTIFY_MASK,
                );
            }
        }
        's_802: loop {
            if presel != 47 as libc::c_int {
                redraw(path);
                statusbar(path);
            } else if g_ctx[cfg.curctx() as usize].c_fltr[1 as libc::c_int as usize] == 0
                {
                redraw(path);
                statusbar(path);
            }
            loop {
                tmp___6 = getppid();
                if tmp___6 == 1 as libc::c_int {
                    _exit(1 as libc::c_int);
                }
                tmp___7 = chdir(path as *const libc::c_char);
                if tmp___7 == -(1 as libc::c_int) {
                    continue '_begin;
                }
                tmp___8 = isatty(0 as libc::c_int);
                if tmp___8 == 0 {
                    if g_state.picker() == 0 {
                        return 1 as libc::c_int != 0;
                    }
                }
                tmp___9 = nextsel(presel);
                sel = tmp___9 as action;
                if presel != 0 {
                    presel = 0 as libc::c_int;
                }
                match sel as libc::c_uint {
                    1 => {
                        dir = visit_parent(path, newpath.as_mut_ptr(), &mut presel);
                        if dir.is_null() {
                            continue;
                        }
                        tmp___10 = xbasename(path);
                        xstrsncpy(
                            lastname,
                            tmp___10 as *const libc::c_char,
                            256 as libc::c_int as size_t,
                        );
                        tmp___11 = cdprep(
                            lastdir,
                            0 as *mut libc::c_void as *mut libc::c_char,
                            path,
                            dir,
                        );
                        if tmp___11 {
                            presel = '/' as i32;
                        } else {
                            watch = 1 as libc::c_int != 0;
                        }
                        continue '_begin;
                    }
                    2 | 3 => {
                        if ndents == 0 {
                            cd = 0 as libc::c_int != 0;
                            g_state.set_runplugin(0 as libc::c_int as uint_t);
                            g_state.set_selbm(g_state.runplugin());
                            continue '_begin;
                        } else {
                            pent = pdents.offset(cur as isize);
                            if g_state.selbm() == 0 {
                                mkpath(
                                    path as *const libc::c_char,
                                    (*pent).name as *const libc::c_char,
                                    newpath.as_mut_ptr(),
                                );
                            } else if (*pent).mode & 61440 as libc::c_uint
                                    == 40960 as libc::c_uint
                                {
                                tmp___12 = realpath(
                                    (*pent).name as *const libc::c_char,
                                    newpath.as_mut_ptr(),
                                );
                                if !tmp___12.is_null() {
                                    tmp___13 = xstrsncpy(
                                        path,
                                        lastdir as *const libc::c_char,
                                        4096 as libc::c_int as size_t,
                                    );
                                    if tmp___13 == 0 {
                                        mkpath(
                                            path as *const libc::c_char,
                                            (*pent).name as *const libc::c_char,
                                            newpath.as_mut_ptr(),
                                        );
                                    }
                                } else {
                                    mkpath(
                                        path as *const libc::c_char,
                                        (*pent).name as *const libc::c_char,
                                        newpath.as_mut_ptr(),
                                    );
                                }
                            } else {
                                mkpath(
                                    path as *const libc::c_char,
                                    (*pent).name as *const libc::c_char,
                                    newpath.as_mut_ptr(),
                                );
                            }
                            g_state.set_selbm(0 as libc::c_int as uint_t);
                            if ((*pent).__annonCompField18).flags()
                                & 1 as libc::c_ulonglong != 0
                            {
                                tmp___16 = chdir(
                                    newpath.as_mut_ptr() as *const libc::c_char,
                                );
                                if tmp___16 == -(1 as libc::c_int) {
                                    tmp___14 = __errno_location();
                                    tmp___15 = strerror(*tmp___14);
                                    printwait(tmp___15 as *const libc::c_char, &mut presel);
                                    continue;
                                } else {
                                    tmp___17 = cdprep(
                                        lastdir,
                                        lastname,
                                        path,
                                        newpath.as_mut_ptr(),
                                    );
                                    if tmp___17 {
                                        presel = '/' as i32;
                                    } else {
                                        watch = 1 as libc::c_int != 0;
                                    }
                                    continue '_begin;
                                }
                            } else {
                                tmp___20 = stat(
                                    newpath.as_mut_ptr() as *const libc::c_char,
                                    &mut sb as *mut stat,
                                );
                                if tmp___20 == -(1 as libc::c_int) {
                                    tmp___18 = __errno_location();
                                    tmp___19 = strerror(*tmp___18);
                                    printwait(tmp___19 as *const libc::c_char, &mut presel);
                                    continue;
                                } else if !(sb.st_mode & 61440 as libc::c_uint
                                        == 32768 as libc::c_uint)
                                    {
                                    printwait(
                                        messages[26 as libc::c_int as usize],
                                        &mut presel,
                                    );
                                    continue;
                                } else {
                                    if g_state.runplugin() != 0 {
                                        g_state.set_runplugin(0 as libc::c_int as uint_t);
                                        if g_state.runctx() as libc::c_int
                                            == cfg.curctx() as libc::c_int
                                        {
                                            tmp___24 = strcmp(
                                                path as *const libc::c_char,
                                                plgpath as *const libc::c_char,
                                            );
                                            if tmp___24 == 0 {
                                                endselection(0 as libc::c_int != 0);
                                                xstrsncpy(
                                                    path,
                                                    lastdir as *const libc::c_char,
                                                    4096 as libc::c_int as size_t,
                                                );
                                                clearfilter();
                                                tmp___21 = chdir(path as *const libc::c_char);
                                                if tmp___21 == -(1 as libc::c_int) {
                                                    tmp___23 = 1 as libc::c_int;
                                                } else {
                                                    tmp___22 = run_plugin(
                                                        &mut path,
                                                        (*pent).name as *const libc::c_char,
                                                        runfile.as_mut_ptr(),
                                                        &mut lastname,
                                                        &mut lastdir,
                                                    );
                                                    if tmp___22 {
                                                        tmp___23 = 0 as libc::c_int;
                                                    } else {
                                                        tmp___23 = 1 as libc::c_int;
                                                    }
                                                }
                                                if g_state.picked() != 0 {
                                                    return 0 as libc::c_int != 0;
                                                }
                                                if runfile[0 as libc::c_int as usize] != 0 {
                                                    xstrsncpy(
                                                        lastname,
                                                        runfile.as_mut_ptr() as *const libc::c_char,
                                                        256 as libc::c_int as size_t,
                                                    );
                                                    runfile[0 as libc::c_int
                                                        as usize] = '\u{0}' as i32 as libc::c_char;
                                                }
                                                if cfg.filtermode() != 0 {
                                                    presel = '/' as i32;
                                                } else {
                                                    watch = 1 as libc::c_int != 0;
                                                }
                                                continue '_begin;
                                            }
                                        }
                                    }
                                    if g_state.picker() != 0 {
                                        if sel as libc::c_uint == 2 as libc::c_uint {
                                            if nselected == 0 as libc::c_int {
                                                tmp___25 = mkpath(
                                                    path as *const libc::c_char,
                                                    (*pent).name as *const libc::c_char,
                                                    newpath.as_mut_ptr(),
                                                );
                                                appendfpath(
                                                    newpath.as_mut_ptr() as *const libc::c_char,
                                                    tmp___25,
                                                );
                                            }
                                            return 0 as libc::c_int != 0;
                                        }
                                    }
                                    if sel as libc::c_uint == 3 as libc::c_uint {
                                        if !listpath.is_null() {
                                            if (*pent).mode & 61440 as libc::c_uint
                                                == 40960 as libc::c_uint
                                            {
                                                tmp___33 = xstrlen(listpath as *const libc::c_char);
                                                tmp___34 = is_prefix(
                                                    path as *const libc::c_char,
                                                    listpath as *const libc::c_char,
                                                    tmp___33,
                                                );
                                                if tmp___34 {
                                                    tmp___28 = realpath(
                                                        (*pent).name as *const libc::c_char,
                                                        newpath.as_mut_ptr(),
                                                    );
                                                    if tmp___28.is_null() {
                                                        tmp___26 = __errno_location();
                                                        tmp___27 = strerror(*tmp___26);
                                                        printwait(tmp___27 as *const libc::c_char, &mut presel);
                                                        continue;
                                                    } else {
                                                        xdirname(newpath.as_mut_ptr());
                                                        tmp___31 = chdir(
                                                            newpath.as_mut_ptr() as *const libc::c_char,
                                                        );
                                                        if tmp___31 == -(1 as libc::c_int) {
                                                            tmp___29 = __errno_location();
                                                            tmp___30 = strerror(*tmp___29);
                                                            printwait(tmp___30 as *const libc::c_char, &mut presel);
                                                            continue;
                                                        } else {
                                                            tmp___32 = cdprep(
                                                                lastdir,
                                                                0 as *mut libc::c_void as *mut libc::c_char,
                                                                path,
                                                                newpath.as_mut_ptr(),
                                                            );
                                                            if tmp___32 {
                                                                presel = '/' as i32;
                                                            } else {
                                                                watch = 1 as libc::c_int != 0;
                                                            }
                                                            xstrsncpy(
                                                                lastname,
                                                                (*pent).name as *const libc::c_char,
                                                                256 as libc::c_int as size_t,
                                                            );
                                                            continue '_begin;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        if cfg.nonavopen() != 0 {
                                            continue;
                                        }
                                    }
                                    if sb.st_size == 0 {
                                        printwait(
                                            messages[25 as libc::c_int as usize],
                                            &mut presel,
                                        );
                                        continue;
                                    } else {
                                        if cfg.useeditor() != 0 {
                                            tmp___35 = get_output(
                                                b"file\0" as *const u8 as *const libc::c_char
                                                    as *mut libc::c_char,
                                                b"-biL\0" as *const u8 as *const libc::c_char
                                                    as *mut libc::c_char,
                                                newpath.as_mut_ptr(),
                                                -(1 as libc::c_int),
                                                0 as libc::c_int != 0,
                                                0 as libc::c_int != 0,
                                            );
                                            if tmp___35 {
                                                tmp___36 = is_prefix(
                                                    g_buf.as_mut_ptr() as *const libc::c_char,
                                                    b"text/\0" as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int as size_t,
                                                );
                                                if tmp___36 {
                                                    spawn(
                                                        editor,
                                                        newpath.as_mut_ptr(),
                                                        0 as *mut libc::c_void as *mut libc::c_char,
                                                        0 as *mut libc::c_void as *mut libc::c_char,
                                                        9 as libc::c_int as ushort_t,
                                                    );
                                                    if cfg.filtermode() != 0 {
                                                        presel = '/' as i32;
                                                        clearfilter();
                                                    }
                                                    continue 's_802;
                                                }
                                            }
                                        }
                                        tmp = xextension(
                                            (*pent).name as *const libc::c_char,
                                            ((*pent).__annonCompField18)
                                                .nlen()
                                                .wrapping_sub(1 as libc::c_ulonglong) as size_t,
                                        );
                                        if tmp.is_null() {
                                            current_block = 11271090240167667812;
                                            break;
                                        }
                                        tmp___41 = regexec(
                                            &mut archive_re as *mut regex_t as *const regex_t,
                                            tmp as *const libc::c_char,
                                            0 as libc::c_int as size_t,
                                            0 as *mut libc::c_void as *mut regmatch_t,
                                            0 as libc::c_int,
                                        );
                                        if !(tmp___41 == 0) {
                                            current_block = 11271090240167667812;
                                            break;
                                        }
                                        r = get_input(messages[33 as libc::c_int as usize]);
                                        if r == 108 as libc::c_int {
                                            current_block = 11041553272636791077;
                                        } else if r == 120 as libc::c_int {
                                            current_block = 11041553272636791077;
                                        } else {
                                            current_block = 8401395776411289795;
                                        }
                                        match current_block {
                                            11041553272636791077 => {
                                                mkpath(
                                                    path as *const libc::c_char,
                                                    (*pent).name as *const libc::c_char,
                                                    newpath.as_mut_ptr(),
                                                );
                                                tmp___37 = handle_archive(
                                                    newpath.as_mut_ptr(),
                                                    r as libc::c_char,
                                                );
                                                if !tmp___37 {
                                                    presel = '$' as i32;
                                                    continue;
                                                } else if r == 108 as libc::c_int {
                                                    statusbar(path);
                                                    continue;
                                                }
                                            }
                                            _ => {}
                                        }
                                        if r == 109 as libc::c_int {
                                            tmp___38 = archive_mount(newpath.as_mut_ptr());
                                            if !tmp___38 {
                                                presel = '$' as i32;
                                                continue;
                                            }
                                        }
                                        if r == 120 as libc::c_int {
                                            current_block = 6727606101734016984;
                                            break 's_802;
                                        }
                                        if r == 109 as libc::c_int {
                                            current_block = 6727606101734016984;
                                            break 's_802;
                                        }
                                        if !(r != 111 as libc::c_int) {
                                            current_block = 11271090240167667812;
                                            break;
                                        }
                                        printwait(
                                            messages[40 as libc::c_int as usize],
                                            &mut presel,
                                        );
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    13 | 12 | 11 | 10 | 9 | 7 | 8 | 6 | 5 | 4 => {
                        if ndents != 0 {
                            g_state.set_move_0(1 as libc::c_int as uint_t);
                            handle_screen_move(sel);
                        }
                        continue 's_802;
                    }
                    17 | 16 | 15 | 14 => {
                        if sel as libc::c_uint == 14 as libc::c_uint {
                            dir = home;
                        } else {
                            if sel as libc::c_uint == 15 as libc::c_uint {
                                tmp___43 = ipath as *const libc::c_char;
                            } else {
                                if sel as libc::c_uint == 16 as libc::c_uint {
                                    tmp___42 = lastdir as *const libc::c_char;
                                } else {
                                    tmp___42 = b"/\0" as *const u8 as *const libc::c_char;
                                }
                                tmp___43 = tmp___42;
                            }
                            dir = tmp___43 as *mut libc::c_char;
                        }
                        if dir.is_null() {
                            printwait(messages[27 as libc::c_int as usize], &mut presel);
                            continue;
                        } else if *dir == 0 {
                            printwait(messages[27 as libc::c_int as usize], &mut presel);
                            continue;
                        } else {
                            g_state.set_selbm(0 as libc::c_int as uint_t);
                            tmp___44 = strcmp(
                                path as *const libc::c_char,
                                dir as *const libc::c_char,
                            );
                            if tmp___44 == 0 as libc::c_int {
                                if dir as libc::c_ulong == ipath as libc::c_ulong {
                                    if cfg.filtermode() != 0 {
                                        presel = '/' as i32;
                                    }
                                    continue;
                                } else {
                                    dir = lastdir;
                                }
                            }
                            tmp___45 = chdir(dir as *const libc::c_char);
                            if tmp___45 == -(1 as libc::c_int) {
                                presel = '$' as i32;
                                continue;
                            } else {
                                xstrsncpy(
                                    newpath.as_mut_ptr(),
                                    dir as *const libc::c_char,
                                    4096 as libc::c_int as size_t,
                                );
                            }
                        }
                        current_block = 13183274545681798691;
                    }
                    18 => {
                        current_block = 13183274545681798691;
                    }
                    19 => {
                        if !(sel as libc::c_uint == 19 as libc::c_uint) {
                            current_block = 14904012976982599327;
                            break 's_802;
                        }
                        tmp___50 = remote_mount(newpath.as_mut_ptr());
                        if tmp___50 {
                            current_block = 14904012976982599327;
                            break 's_802;
                        }
                        presel = '$' as i32;
                        continue;
                    }
                    25 | 24 | 23 | 22 | 21 | 20 => {
                        r = handle_context_switch(sel);
                        if r < 0 as libc::c_int {
                            continue 's_802;
                        }
                        if ndents != 0 {
                            tmp___52 = (*pdents.offset(cur as isize)).name;
                        } else {
                            tmp___52 = 0 as *mut libc::c_void as *mut libc::c_char;
                        }
                        savecurctx(path, tmp___52, r);
                        path = (g_ctx[r as usize].c_path).as_mut_ptr();
                        lastdir = (g_ctx[r as usize].c_last).as_mut_ptr();
                        lastname = (g_ctx[r as usize].c_name).as_mut_ptr();
                        tmp = (g_ctx[r as usize].c_fltr).as_mut_ptr();
                        if cfg.filtermode() != 0 {
                            presel = '/' as i32;
                        } else {
                            let mut current_block_372: u64;
                            if *tmp.offset(0 as libc::c_int as isize) as libc::c_int
                                == 47 as libc::c_int
                            {
                                current_block_372 = 4050371153694240832;
                            } else if *tmp.offset(0 as libc::c_int as isize)
                                    as libc::c_int == 92 as libc::c_int
                                {
                                current_block_372 = 4050371153694240832;
                            } else {
                                watch = 1 as libc::c_int != 0;
                                current_block_372 = 5996944888495321466;
                            }
                            match current_block_372 {
                                4050371153694240832 => {
                                    if *tmp.offset(1 as libc::c_int as isize) != 0 {
                                        presel = '/' as i32;
                                    } else {
                                        watch = 1 as libc::c_int != 0;
                                    }
                                }
                                _ => {}
                            }
                        }
                        continue '_begin;
                    }
                    26 => {
                        free(mark as *mut libc::c_void);
                        mark = xstrdup(path as *const libc::c_char);
                        printwait(mark as *const libc::c_char, &mut presel);
                        continue;
                    }
                    27 => {
                        add_bookmark(path, newpath.as_mut_ptr(), &mut presel);
                        continue;
                    }
                    28 => {
                        if inotify_wd >= 0 as libc::c_int {
                            inotify_rm_watch(inotify_fd, inotify_wd);
                            inotify_wd = -(1 as libc::c_int);
                        }
                        presel = filterentries(path, lastname);
                        if presel == 27 as libc::c_int {
                            presel = 0 as libc::c_int;
                            continue 's_802;
                        } else {
                            if !(presel == 47 as libc::c_int) {
                                continue;
                            }
                            cd = 0 as libc::c_int != 0;
                            continue '_begin;
                        }
                    }
                    35 | 31 | 30 | 29 => {
                        match sel as libc::c_uint {
                            29 => {
                                cfg
                                    .set_filtermode(
                                        cfg.filtermode() ^ 1 as libc::c_uint as uint_t,
                                    );
                                if cfg.filtermode() != 0 {
                                    presel = '/' as i32;
                                    clearfilter();
                                    continue;
                                } else {
                                    watch = 1 as libc::c_int != 0;
                                    current_block = 3382726224265272059;
                                    break 's_802;
                                }
                            }
                            30 => {
                                current_block = 3382726224265272059;
                                break 's_802;
                            }
                            31 => {
                                cfg
                                    .set_showdetail(
                                        cfg.showdetail() ^ 1 as libc::c_uint as uint_t,
                                    );
                                cfg.set_blkorder(0 as libc::c_int as uint_t);
                                continue 's_802;
                            }
                            _ => {
                                tmp___54 = get_input(messages[36 as libc::c_int as usize]);
                                r = set_sort_flags(tmp___54);
                                if r == 0 {
                                    printwait(
                                        messages[40 as libc::c_int as usize],
                                        &mut presel,
                                    );
                                    continue;
                                } else {
                                    if cfg.filtermode() != 0 {
                                        presel = '/' as i32;
                                    } else if g_ctx[cfg.curctx() as usize]
                                            .c_fltr[1 as libc::c_int as usize] != 0
                                        {
                                        presel = '/' as i32;
                                    }
                                    if !(ndents != 0) {
                                        continue 's_802;
                                    }
                                    if ndents != 0 {
                                        tmp___55 = (*pdents.offset(cur as isize)).name
                                            as *const libc::c_char;
                                    } else {
                                        tmp___55 = b"\0\0" as *const u8 as *const libc::c_char;
                                    }
                                    xstrsncpy(lastname, tmp___55, 256 as libc::c_int as size_t);
                                    if r == 100 as libc::c_int {
                                        current_block = 9181201145437202872;
                                        break;
                                    } else {
                                        current_block = 1993610015827684295;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    33 | 32 => {
                        if !(ndents != 0) {
                            continue 's_802;
                        }
                        if !listpath.is_null() {
                            if *path as libc::c_int != *listpath as libc::c_int {
                                tmp___61 = -(1 as libc::c_int);
                            } else {
                                tmp___60 = strcmp(
                                    path as *const libc::c_char,
                                    listpath as *const libc::c_char,
                                );
                                tmp___61 = tmp___60;
                            }
                            if tmp___61 == 0 as libc::c_int {
                                tmp = listroot;
                            } else {
                                tmp = path;
                            }
                        } else {
                            tmp = path;
                        }
                        mkpath(
                            tmp as *const libc::c_char,
                            (*pdents.offset(cur as isize)).name as *const libc::c_char,
                            newpath.as_mut_ptr(),
                        );
                        if sel as libc::c_uint == 32 as libc::c_uint {
                            tmp___64 = show_stats(newpath.as_mut_ptr());
                            if !tmp___64 {
                                tmp___62 = __errno_location();
                                tmp___63 = strerror(*tmp___62);
                                printwait(tmp___63 as *const libc::c_char, &mut presel);
                                continue;
                            }
                        }
                        tmp___65 = lstat(
                            newpath.as_mut_ptr() as *const libc::c_char,
                            &mut sb as *mut stat,
                        );
                        if tmp___65 == -(1 as libc::c_int) {
                            tmp___62 = __errno_location();
                            tmp___63 = strerror(*tmp___62);
                            printwait(tmp___63 as *const libc::c_char, &mut presel);
                            continue;
                        } else {
                            if !(sel as libc::c_uint == 33 as libc::c_uint) {
                                current_block = 16708736497050535312;
                                break;
                            }
                            tmp___66 = xchmod(
                                newpath.as_mut_ptr() as *const libc::c_char,
                                sb.st_mode,
                            );
                            if tmp___66 {
                                current_block = 16708736497050535312;
                                break;
                            }
                            tmp___62 = __errno_location();
                            tmp___63 = strerror(*tmp___62);
                            printwait(tmp___63 as *const libc::c_char, &mut presel);
                            continue;
                        }
                    }
                    58 | 53 | 52 | 51 | 49 | 36 => {
                        refresh___0 = 0 as libc::c_int != 0;
                        if ndents != 0 {
                            mkpath(
                                path as *const libc::c_char,
                                (*pdents.offset(cur as isize)).name as *const libc::c_char,
                                newpath.as_mut_ptr(),
                            );
                        } else if sel as libc::c_uint == 53 as libc::c_uint {
                            continue;
                        }
                        match sel as libc::c_uint {
                            36 => {
                                refresh___0 = 1 as libc::c_int != 0;
                                current_block = 15326269721003185778;
                            }
                            49 => {
                                endselection(1 as libc::c_int != 0);
                                tmp___67 = xitoa(cfg.showhidden());
                                setenv(
                                    b"NNN_INCLUDE_HIDDEN\0" as *const u8 as *const libc::c_char,
                                    tmp___67 as *const libc::c_char,
                                    1 as libc::c_int,
                                );
                                if !listpath.is_null() {
                                    tmp___68 = listroot as *const libc::c_char;
                                } else {
                                    tmp___68 = b"\0" as *const u8 as *const libc::c_char;
                                }
                                setenv(
                                    b"NNN_LIST\0" as *const u8 as *const libc::c_char,
                                    tmp___68,
                                    1 as libc::c_int,
                                );
                                tmp___69 = getutil(utils[8 as libc::c_int as usize]);
                                if tmp___69 {
                                    tmp___70 = plugscript(
                                        utils[17 as libc::c_int as usize] as *const libc::c_char,
                                        9 as libc::c_int as uchar_t,
                                    );
                                    if !tmp___70 {
                                        printwait(messages[5 as libc::c_int as usize], &mut presel);
                                        continue;
                                    } else {
                                        clearselection();
                                        refresh___0 = 1 as libc::c_int != 0;
                                    }
                                } else {
                                    printwait(messages[5 as libc::c_int as usize], &mut presel);
                                    continue;
                                }
                                current_block = 15326269721003185778;
                            }
                            51 => {
                                show_help(path as *const libc::c_char);
                                current_block = 12623038205816195448;
                            }
                            52 => {
                                current_block = 12623038205816195448;
                            }
                            53 => {
                                if g_state.picker() == 0 {
                                    if g_state.fifomode() == 0 {
                                        spawn(
                                            editor,
                                            newpath.as_mut_ptr(),
                                            0 as *mut libc::c_void as *mut libc::c_char,
                                            0 as *mut libc::c_void as *mut libc::c_char,
                                            9 as libc::c_int as ushort_t,
                                        );
                                    }
                                }
                                continue 's_802;
                            }
                            _ => {
                                lock_terminal();
                                current_block = 15326269721003185778;
                            }
                        }
                        match current_block {
                            12623038205816195448 => {
                                if sel as libc::c_uint == 52 as libc::c_uint {
                                    g_state
                                        .set_autonext(
                                            g_state.autonext() ^ 1 as libc::c_uint as uint_t,
                                        );
                                }
                                if cfg.filtermode() != 0 {
                                    presel = '/' as i32;
                                }
                                if ndents != 0 {
                                    tmp___71 = (*pdents.offset(cur as isize)).name
                                        as *const libc::c_char;
                                } else {
                                    tmp___71 = b"\0\0" as *const u8 as *const libc::c_char;
                                }
                                xstrsncpy(lastname, tmp___71, 256 as libc::c_int as size_t);
                                continue;
                            }
                            _ => {
                                if !(cfg.filtermode() != 0) {
                                    if !(g_ctx[cfg.curctx() as usize]
                                        .c_fltr[1 as libc::c_int as usize] != 0)
                                    {
                                        current_block = 14559976248321351685;
                                        break 's_802;
                                    }
                                }
                                if refresh___0 {
                                    current_block = 14559976248321351685;
                                    break 's_802;
                                }
                                presel = '/' as i32;
                                continue;
                            }
                        }
                    }
                    37 => {
                        if ndents == 0 {
                            continue;
                        }
                        startselection();
                        if g_state.rangesel() != 0 {
                            g_state.set_rangesel(0 as libc::c_int as uint_t);
                        }
                        let ref mut fresh16 = (*pdents.offset(cur as isize))
                            .__annonCompField18;
                        (*fresh16)
                            .set_flags(
                                (*fresh16).flags() ^ 16 as libc::c_ulonglong as ullong_t,
                            );
                        if ((*pdents.offset(cur as isize)).__annonCompField18).flags()
                            & 16 as libc::c_ulonglong != 0
                        {
                            nselected += 1;
                            tmp___73 = mkpath(
                                path as *const libc::c_char,
                                (*pdents.offset(cur as isize)).name as *const libc::c_char,
                                newpath.as_mut_ptr(),
                            );
                            appendfpath(
                                newpath.as_mut_ptr() as *const libc::c_char,
                                tmp___73,
                            );
                            writesel(
                                pselbuf as *const libc::c_char,
                                selbufpos.wrapping_sub(1 as libc::c_uint) as size_t,
                            );
                        } else {
                            nselected -= 1;
                            tmp___74 = mkpath(
                                path as *const libc::c_char,
                                (*pdents.offset(cur as isize)).name as *const libc::c_char,
                                g_sel.as_mut_ptr(),
                            );
                            rmfromselbuf(tmp___74);
                        }
                        if g_state.stayonsel() == 0 {
                            if cur != ndents - 1 as libc::c_int {
                                move_cursor(
                                    (cur + 1 as libc::c_int) % ndents,
                                    0 as libc::c_int,
                                );
                            }
                        }
                        continue 's_802;
                    }
                    38 => {
                        if ndents == 0 {
                            continue;
                        }
                        startselection();
                        g_state
                            .set_rangesel(
                                g_state.rangesel() ^ 1 as libc::c_uint as uint_t,
                            );
                        tmp___77 = stat(
                            path as *const libc::c_char,
                            &mut sb as *mut stat,
                        );
                        if tmp___77 == -(1 as libc::c_int) {
                            tmp___75 = __errno_location();
                            tmp___76 = strerror(*tmp___75);
                            printwait(tmp___76 as *const libc::c_char, &mut presel);
                            continue;
                        } else if g_state.rangesel() != 0 {
                            inode = sb.st_ino;
                            selstartid = cur;
                            continue 's_802;
                        } else if inode != sb.st_ino {
                            printwait(messages[42 as libc::c_int as usize], &mut presel);
                            continue;
                        } else {
                            if cur < selstartid {
                                selendid = selstartid;
                                selstartid = cur;
                            } else {
                                selendid = cur;
                            }
                            if selstartid == selendid {
                                resetselind();
                                clearselection();
                                continue 's_802;
                            }
                        }
                        current_block = 1709191387750011928;
                    }
                    40 | 39 => {
                        current_block = 1709191387750011928;
                    }
                    41 => {
                        r = editselection();
                        if r <= 0 as libc::c_int {
                            if r == 0 {
                                r = 3 as libc::c_int;
                            } else {
                                r = 5 as libc::c_int;
                            }
                            printwait(messages[r as usize], &mut presel);
                        } else if cfg.filtermode() != 0 {
                            presel = '/' as i32;
                        } else {
                            statusbar(path);
                        }
                        continue;
                    }
                    45 | 44 | 43 | 42 => {
                        if sel as libc::c_uint == 45 as libc::c_uint {
                            r = get_cur_or_sel();
                            if r == 0 {
                                statusbar(path);
                                continue;
                            } else if r == 99 as libc::c_int {
                                if !listpath.is_null() {
                                    if *path as libc::c_int != *listpath as libc::c_int {
                                        tmp___81 = -(1 as libc::c_int);
                                    } else {
                                        tmp___80 = strcmp(
                                            path as *const libc::c_char,
                                            listpath as *const libc::c_char,
                                        );
                                        tmp___81 = tmp___80;
                                    }
                                    if tmp___81 == 0 as libc::c_int {
                                        tmp = listroot;
                                    } else {
                                        tmp = path;
                                    }
                                } else {
                                    tmp = path;
                                }
                                mkpath(
                                    tmp as *const libc::c_char,
                                    (*pdents.offset(cur as isize)).name as *const libc::c_char,
                                    newpath.as_mut_ptr(),
                                );
                                tmp___82 = xrm(newpath.as_mut_ptr());
                                if !tmp___82 {
                                    continue 's_802;
                                }
                                xrmfromsel(tmp, newpath.as_mut_ptr());
                                copynextname(lastname);
                                if cfg.filtermode() != 0 {
                                    presel = '/' as i32;
                                } else if g_ctx[cfg.curctx() as usize]
                                        .c_fltr[1 as libc::c_int as usize] != 0
                                    {
                                    presel = '/' as i32;
                                }
                                cd = 0 as libc::c_int != 0;
                                continue '_begin;
                            }
                        }
                        if nselected == 1 as libc::c_int {
                            if sel as libc::c_uint == 42 as libc::c_uint {
                                tmp___83 = xbasename(pselbuf);
                                mkpath(
                                    path as *const libc::c_char,
                                    tmp___83 as *const libc::c_char,
                                    newpath.as_mut_ptr(),
                                );
                            } else if sel as libc::c_uint == 43 as libc::c_uint {
                                tmp___83 = xbasename(pselbuf);
                                mkpath(
                                    path as *const libc::c_char,
                                    tmp___83 as *const libc::c_char,
                                    newpath.as_mut_ptr(),
                                );
                            } else {
                                newpath[0 as libc::c_int
                                    as usize] = '\u{0}' as i32 as libc::c_char;
                            }
                        } else {
                            newpath[0 as libc::c_int
                                as usize] = '\u{0}' as i32 as libc::c_char;
                        }
                        endselection(1 as libc::c_int != 0);
                        tmp___84 = cpmvrm_selection(sel, path);
                        if !tmp___84 {
                            presel = '$' as i32;
                            continue;
                        } else {
                            if cfg.filtermode() != 0 {
                                presel = '/' as i32;
                            }
                            clearfilter();
                            let mut current_block_643: u64;
                            if newpath[0 as libc::c_int as usize] != 0 {
                                tmp___87 = access(
                                    newpath.as_mut_ptr() as *const libc::c_char,
                                    0 as libc::c_int,
                                );
                                if tmp___87 != 0 {
                                    current_block_643 = 5839495510726360737;
                                } else {
                                    tmp___85 = xbasename(newpath.as_mut_ptr());
                                    xstrsncpy(
                                        lastname,
                                        tmp___85 as *const libc::c_char,
                                        256 as libc::c_int as size_t,
                                    );
                                    current_block_643 = 15944773282265231861;
                                }
                            } else {
                                current_block_643 = 5839495510726360737;
                            }
                            match current_block_643 {
                                5839495510726360737 => {
                                    if ndents != 0 {
                                        tmp___86 = (*pdents.offset(cur as isize)).name
                                            as *const libc::c_char;
                                    } else {
                                        tmp___86 = b"\0\0" as *const u8 as *const libc::c_char;
                                    }
                                    xstrsncpy(lastname, tmp___86, 256 as libc::c_int as size_t);
                                }
                                _ => {}
                            }
                            cd = 0 as libc::c_int != 0;
                            continue '_begin;
                        }
                    }
                    48 | 47 | 46 | 34 => {
                        ret = 'n' as i32;
                        if ndents == 0 {
                            if sel as libc::c_uint == 46 as libc::c_uint {
                                continue 's_802;
                            }
                            if sel as libc::c_uint == 48 as libc::c_uint {
                                continue 's_802;
                            }
                        }
                        if sel as libc::c_uint != 46 as libc::c_uint {
                            endselection(1 as libc::c_int != 0);
                        }
                        match sel as libc::c_uint {
                            34 => {
                                r = get_cur_or_sel();
                                if r == 0 {
                                    statusbar(path);
                                    continue;
                                } else {
                                    if r == 115 as libc::c_int {
                                        tmp___88 = selsafe();
                                        if !tmp___88 {
                                            presel = '$' as i32;
                                            continue;
                                        } else {
                                            tmp = 0 as *mut libc::c_void as *mut libc::c_char;
                                        }
                                    } else {
                                        tmp = (*pdents.offset(cur as isize)).name;
                                    }
                                    tmp = xreadline(
                                        tmp as *const libc::c_char,
                                        messages[17 as libc::c_int as usize],
                                    );
                                }
                            }
                            46 => {
                                tmp = xreadline(
                                    0 as *mut libc::c_void as *const libc::c_char,
                                    messages[18 as libc::c_int as usize],
                                );
                            }
                            47 => {
                                r = get_input(messages[11 as libc::c_int as usize]);
                                if r == 102 as libc::c_int {
                                    tmp = xreadline(
                                        0 as *mut libc::c_void as *const libc::c_char,
                                        messages[19 as libc::c_int as usize],
                                    );
                                } else if r == 100 as libc::c_int {
                                    tmp = xreadline(
                                        0 as *mut libc::c_void as *const libc::c_char,
                                        messages[19 as libc::c_int as usize],
                                    );
                                } else {
                                    let mut current_block_671: u64;
                                    if r == 115 as libc::c_int {
                                        current_block_671 = 8223502108133464676;
                                    } else if r == 104 as libc::c_int {
                                        current_block_671 = 8223502108133464676;
                                    } else {
                                        tmp = 0 as *mut libc::c_void as *mut libc::c_char;
                                        current_block_671 = 9709721630822525588;
                                    }
                                    match current_block_671 {
                                        8223502108133464676 => {
                                            if nselected <= 1 as libc::c_int {
                                                tmp___89 = 19 as libc::c_int;
                                            } else {
                                                tmp___89 = 20 as libc::c_int;
                                            }
                                            tmp = xreadline(
                                                0 as *mut libc::c_void as *const libc::c_char,
                                                messages[tmp___89 as usize],
                                            );
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            _ => {
                                tmp = xreadline(
                                    (*pdents.offset(cur as isize)).name as *const libc::c_char,
                                    b"\0" as *const u8 as *const libc::c_char,
                                );
                            }
                        }
                        if tmp.is_null() {
                            continue 's_802;
                        }
                        if *tmp == 0 {
                            continue 's_802;
                        }
                        match sel as libc::c_uint {
                            34 => {
                                if r == 99 as libc::c_int {
                                    tmp___90 = strcmp(
                                        tmp as *const libc::c_char,
                                        (*pdents.offset(cur as isize)).name as *const libc::c_char,
                                    );
                                    if tmp___90 == 0 as libc::c_int {
                                        continue;
                                    }
                                }
                                mkpath(
                                    path as *const libc::c_char,
                                    tmp as *const libc::c_char,
                                    newpath.as_mut_ptr(),
                                );
                                tmp___93 = access(
                                    newpath.as_mut_ptr() as *const libc::c_char,
                                    0 as libc::c_int,
                                );
                                if !(tmp___93 == 0 as libc::c_int) {
                                    current_block = 6344850934232263781;
                                    break;
                                }
                                tmp___91 = get_input(messages[13 as libc::c_int as usize]);
                                tmp___92 = xconfirm(tmp___91);
                                if tmp___92 {
                                    current_block = 6344850934232263781;
                                    break;
                                }
                                statusbar(path);
                                continue;
                            }
                            46 => {
                                handle_openwith(
                                    path as *const libc::c_char,
                                    (*pdents.offset(cur as isize)).name as *const libc::c_char,
                                    newpath.as_mut_ptr(),
                                    tmp,
                                );
                                if cfg.filtermode() != 0 {
                                    presel = '/' as i32;
                                } else {
                                    statusbar(path);
                                }
                                if ndents != 0 {
                                    tmp___95 = (*pdents.offset(cur as isize)).name
                                        as *const libc::c_char;
                                } else {
                                    tmp___95 = b"\0\0" as *const u8 as *const libc::c_char;
                                }
                                xstrsncpy(lastname, tmp___95, 256 as libc::c_int as size_t);
                                continue;
                            }
                            48 => {
                                tmp___98 = strcmp(
                                    tmp as *const libc::c_char,
                                    (*pdents.offset(cur as isize)).name as *const libc::c_char,
                                );
                                if tmp___98 == 0 as libc::c_int {
                                    tmp = xreadline(
                                        (*pdents.offset(cur as isize)).name as *const libc::c_char,
                                        messages[21 as libc::c_int as usize],
                                    );
                                    if tmp.is_null() {
                                        current_block = 11012552078951829645;
                                    } else if *tmp.offset(0 as libc::c_int as isize) == 0 {
                                        current_block = 11012552078951829645;
                                    } else {
                                        tmp___97 = strcmp(
                                            tmp as *const libc::c_char,
                                            (*pdents.offset(cur as isize)).name as *const libc::c_char,
                                        );
                                        if tmp___97 == 0 {
                                            current_block = 11012552078951829645;
                                        } else {
                                            ret = 'd' as i32;
                                            current_block = 6772277538283851598;
                                        }
                                    }
                                    match current_block {
                                        6772277538283851598 => {}
                                        _ => {
                                            if cfg.filtermode() != 0 {
                                                presel = '/' as i32;
                                            } else {
                                                statusbar(path);
                                            }
                                            if ndents != 0 {
                                                tmp___96 = (*pdents.offset(cur as isize)).name
                                                    as *const libc::c_char;
                                            } else {
                                                tmp___96 = b"\0\0" as *const u8 as *const libc::c_char;
                                            }
                                            xstrsncpy(lastname, tmp___96, 256 as libc::c_int as size_t);
                                            continue;
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                        fd = open(path as *const libc::c_char, 65536 as libc::c_int);
                        if fd == -(1 as libc::c_int) {
                            tmp___99 = __errno_location();
                            tmp___100 = strerror(*tmp___99);
                            printwait(tmp___100 as *const libc::c_char, &mut presel);
                            continue;
                        } else {
                            tmp___103 = fstatat(
                                fd,
                                tmp as *const libc::c_char,
                                &mut sb as *mut stat,
                                256 as libc::c_int,
                            );
                            if tmp___103 == 0 as libc::c_int {
                                if sel as libc::c_uint == 48 as libc::c_uint {
                                    tmp___101 = get_input(messages[13 as libc::c_int as usize]);
                                    tmp___102 = xconfirm(tmp___101);
                                    if !tmp___102 {
                                        close(fd);
                                        continue 's_802;
                                    }
                                } else {
                                    close(fd);
                                    printwait(
                                        messages[28 as libc::c_int as usize],
                                        &mut presel,
                                    );
                                    continue;
                                }
                            }
                            if sel as libc::c_uint == 48 as libc::c_uint {
                                if ret == 100 as libc::c_int {
                                    spawn(
                                        b"cp -rp\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                        (*pdents.offset(cur as isize)).name,
                                        tmp,
                                        0 as *mut libc::c_void as *mut libc::c_char,
                                        13 as libc::c_int as ushort_t,
                                    );
                                    current_block = 3046210090062227051;
                                    break 's_802;
                                } else {
                                    tmp___106 = renameat(
                                        fd,
                                        (*pdents.offset(cur as isize)).name as *const libc::c_char,
                                        fd,
                                        tmp as *const libc::c_char,
                                    );
                                    if !(tmp___106 != 0 as libc::c_int) {
                                        current_block = 3046210090062227051;
                                        break 's_802;
                                    }
                                    close(fd);
                                    tmp___104 = __errno_location();
                                    tmp___105 = strerror(*tmp___104);
                                    printwait(tmp___105 as *const libc::c_char, &mut presel);
                                    continue;
                                }
                            } else {
                                close(fd);
                                presel = 0 as libc::c_int;
                                let mut current_block_763: u64;
                                if r == 102 as libc::c_int {
                                    current_block_763 = 5732436915694508537;
                                } else if r == 100 as libc::c_int {
                                    current_block_763 = 5732436915694508537;
                                } else {
                                    let mut current_block_762: u64;
                                    if r == 115 as libc::c_int {
                                        current_block_762 = 265207927618644247;
                                    } else if r == 104 as libc::c_int {
                                        current_block_762 = 265207927618644247;
                                    } else {
                                        current_block_762 = 8026555004257951386;
                                    }
                                    match current_block_762 {
                                        265207927618644247 => {
                                            if nselected > 1 as libc::c_int {
                                                if *tmp.offset(0 as libc::c_int as isize) as libc::c_int
                                                    == 64 as libc::c_int
                                                {
                                                    if *tmp.offset(1 as libc::c_int as isize) as libc::c_int
                                                        == 0 as libc::c_int
                                                    {
                                                        *tmp
                                                            .offset(
                                                                0 as libc::c_int as isize,
                                                            ) = '\u{0}' as i32 as libc::c_char;
                                                    }
                                                }
                                            }
                                            if ndents != 0 {
                                                tmp___109 = (*pdents.offset(cur as isize)).name;
                                            } else {
                                                tmp___109 = 0 as *mut libc::c_void as *mut libc::c_char;
                                            }
                                            ret = xlink(
                                                tmp,
                                                path,
                                                tmp___109,
                                                newpath.as_mut_ptr(),
                                                &mut presel,
                                                r,
                                            );
                                        }
                                        _ => {}
                                    }
                                    current_block_763 = 8570820179795813527;
                                }
                                match current_block_763 {
                                    5732436915694508537 => {
                                        mkpath(
                                            path as *const libc::c_char,
                                            tmp as *const libc::c_char,
                                            newpath.as_mut_ptr(),
                                        );
                                        if r == 102 as libc::c_int {
                                            tmp___107 = 0 as libc::c_int;
                                        } else {
                                            tmp___107 = 1 as libc::c_int;
                                        }
                                        tmp___108 = xmktree(newpath.as_mut_ptr(), tmp___107 != 0);
                                        ret = tmp___108 as libc::c_int;
                                    }
                                    _ => {}
                                }
                                if ret == 0 {
                                    printwait(messages[5 as libc::c_int as usize], &mut presel);
                                }
                                if ret <= 0 as libc::c_int {
                                    continue;
                                }
                                if r == 102 as libc::c_int {
                                    xstrsncpy(
                                        lastname,
                                        tmp as *const libc::c_char,
                                        256 as libc::c_int as size_t,
                                    );
                                } else if r == 100 as libc::c_int {
                                    xstrsncpy(
                                        lastname,
                                        tmp as *const libc::c_char,
                                        256 as libc::c_int as size_t,
                                    );
                                } else if ndents != 0 {
                                    if cfg.filtermode() != 0 {
                                        presel = '/' as i32;
                                    }
                                    if ndents != 0 {
                                        tmp___110 = (*pdents.offset(cur as isize)).name
                                            as *const libc::c_char;
                                    } else {
                                        tmp___110 = b"\0\0" as *const u8 as *const libc::c_char;
                                    }
                                    xstrsncpy(
                                        lastname,
                                        tmp___110,
                                        256 as libc::c_int as size_t,
                                    );
                                }
                                clearfilter();
                                current_block = 5617391961331132692;
                                break 's_802;
                            }
                        }
                    }
                    54 => {
                        tmp___113 = xdiraccess(plgpath as *const libc::c_char);
                        if !tmp___113 {
                            tmp___111 = __errno_location();
                            tmp___112 = strerror(*tmp___111);
                            printwait(tmp___112 as *const libc::c_char, &mut presel);
                            continue;
                        } else {
                            if pkey == 0 {
                                tmp___114 = xstrsncpy(
                                    g_buf.as_mut_ptr(),
                                    messages[34 as libc::c_int as usize],
                                    (4096 as libc::c_int
                                        + ((256 as libc::c_int) << 1 as libc::c_int)) as size_t,
                                );
                                r = tmp___114 as libc::c_int;
                                printkeys(
                                    plug,
                                    g_buf
                                        .as_mut_ptr()
                                        .offset(r as isize)
                                        .offset(-(1 as libc::c_int as isize)),
                                    maxplug,
                                );
                                printmsg(g_buf.as_mut_ptr() as *const libc::c_char);
                                r = get_input(
                                    0 as *mut libc::c_void as *const libc::c_char,
                                );
                            } else {
                                r = pkey;
                                pkey = '\u{0}' as i32;
                            }
                            if r != 13 as libc::c_int {
                                endselection(0 as libc::c_int != 0);
                                tmp = get_kv_val(
                                    plug,
                                    0 as *mut libc::c_void as *mut libc::c_char,
                                    r,
                                    maxplug,
                                    2 as libc::c_int as uchar_t,
                                );
                                if tmp.is_null() {
                                    printwait(
                                        messages[40 as libc::c_int as usize],
                                        &mut presel,
                                    );
                                    continue;
                                } else {
                                    if *tmp.offset(0 as libc::c_int as isize) as libc::c_int
                                        == 45 as libc::c_int
                                    {
                                        if *tmp.offset(1 as libc::c_int as isize) != 0 {
                                            tmp = tmp.offset(1);
                                            r = 0 as libc::c_int;
                                        } else {
                                            r = 1 as libc::c_int;
                                        }
                                    } else {
                                        r = 1 as libc::c_int;
                                    }
                                    if ndents != 0 {
                                        tmp___115 = (*pdents.offset(cur as isize)).name;
                                    } else {
                                        tmp___115 = 0 as *mut libc::c_void as *mut libc::c_char;
                                    }
                                    tmp___116 = run_plugin(
                                        &mut path,
                                        tmp as *const libc::c_char,
                                        tmp___115,
                                        &mut lastname,
                                        &mut lastdir,
                                    );
                                    if !tmp___116 {
                                        printwait(messages[5 as libc::c_int as usize], &mut presel);
                                        continue;
                                    } else {
                                        if g_state.picked() != 0 {
                                            return 0 as libc::c_int != 0;
                                        }
                                        if ndents != 0 {
                                            tmp___117 = (*pdents.offset(cur as isize)).name
                                                as *const libc::c_char;
                                        } else {
                                            tmp___117 = b"\0\0" as *const u8 as *const libc::c_char;
                                        }
                                        xstrsncpy(
                                            lastname,
                                            tmp___117,
                                            256 as libc::c_int as size_t,
                                        );
                                        if !(r == 0) {
                                            current_block = 5922202355411293373;
                                            break 's_802;
                                        }
                                        if cfg.filtermode() != 0 {
                                            presel = '/' as i32;
                                        } else {
                                            statusbar(path);
                                        }
                                        continue;
                                    }
                                }
                            } else {
                                g_state
                                    .set_runplugin(
                                        g_state.runplugin() ^ 1 as libc::c_uint as uint_t,
                                    );
                                if g_state.runplugin() == 0 {
                                    current_block = 18207192640746394217;
                                    break 's_802;
                                } else {
                                    current_block = 3733147086002097443;
                                    break 's_802;
                                }
                            }
                        }
                    }
                    57 | 56 | 55 => {
                        tmp___119 = handle_cmd(sel, newpath.as_mut_ptr());
                        r = tmp___119 as libc::c_int;
                        if cfg.filtermode() != 0 {
                            presel = '/' as i32;
                        }
                        if ndents != 0 {
                            tmp___120 = (*pdents.offset(cur as isize)).name
                                as *const libc::c_char;
                        } else {
                            tmp___120 = b"\0\0" as *const u8 as *const libc::c_char;
                        }
                        xstrsncpy(lastname, tmp___120, 256 as libc::c_int as size_t);
                        if r == 0 {
                            continue;
                        }
                        cd = 0 as libc::c_int != 0;
                        continue '_begin;
                    }
                    50 => {
                        presel = 0 as libc::c_int;
                        if ndents != 0 {
                            tmp___121 = (*pdents.offset(cur as isize)).name;
                        } else {
                            tmp___121 = 0 as *mut libc::c_void as *mut libc::c_char;
                        }
                        tmp___122 = unmount(
                            tmp___121,
                            newpath.as_mut_ptr(),
                            &mut presel,
                            path,
                        );
                        if !tmp___122 {
                            if presel == 0 as libc::c_int {
                                statusbar(path);
                            }
                            continue;
                        } else {
                            copynextname(lastname);
                            cd = 0 as libc::c_int != 0;
                            continue '_begin;
                        }
                    }
                    60 => {
                        export_file_list();
                        if cfg.filtermode() != 0 {
                            presel = '/' as i32;
                        } else {
                            statusbar(path);
                        }
                        continue;
                    }
                    61 => {
                        tmp___123 = set_time_type(&mut presel);
                        if !tmp___123 {
                            continue;
                        }
                        cd = 0 as libc::c_int != 0;
                        continue '_begin;
                    }
                    65 | 64 | 63 | 62 => {
                        if sel as libc::c_uint == 62 as libc::c_uint {
                            current_block = 3545793444454788863;
                            break;
                        } else {
                            current_block = 10882616296157082316;
                            break;
                        }
                    }
                    _ => {
                        if xlines as libc::c_int != LINES {
                            continue 's_802;
                        }
                        if xcols as libc::c_int != COLS {
                            continue 's_802;
                        }
                        if idletimeout != 0 {
                            if idle as uint_t == idletimeout {
                                lock_terminal();
                                idle = 0 as libc::c_int as ushort_t;
                            }
                        }
                        if ndents != 0 {
                            tmp___125 = (*pdents.offset(cur as isize)).name
                                as *const libc::c_char;
                        } else {
                            tmp___125 = b"\0\0" as *const u8 as *const libc::c_char;
                        }
                        xstrsncpy(lastname, tmp___125, 256 as libc::c_int as size_t);
                        continue;
                    }
                }
                match current_block {
                    13183274545681798691 => {
                        if !(sel as libc::c_uint == 18 as libc::c_uint) {
                            current_block = 15502218481637409867;
                            break 's_802;
                        }
                        tmp___46 = handle_bookmark(
                            mark as *const libc::c_char,
                            newpath.as_mut_ptr(),
                        );
                        r = tmp___46 as libc::c_int;
                        if r != 0 {
                            printwait(messages[r as usize], &mut presel);
                        } else {
                            if g_state.selbm() == 1 as libc::c_uint {
                                presel = '/' as i32;
                            }
                            tmp___47 = strcmp(
                                path as *const libc::c_char,
                                newpath.as_mut_ptr() as *const libc::c_char,
                            );
                            if tmp___47 == 0 as libc::c_int {
                                continue 's_802;
                            } else {
                                current_block = 15502218481637409867;
                                break 's_802;
                            }
                        }
                    }
                    _ => {
                        if !(sel as libc::c_uint == 39 as libc::c_uint) {
                            if !(sel as libc::c_uint == 40 as libc::c_uint) {
                                current_block = 14756287629061532915;
                                break;
                            }
                        }
                        if ndents == 0 {
                            continue;
                        }
                        startselection();
                        if g_state.rangesel() != 0 {
                            g_state.set_rangesel(0 as libc::c_int as uint_t);
                        }
                        selstartid = 0 as libc::c_int;
                        selendid = ndents - 1 as libc::c_int;
                        current_block = 14756287629061532915;
                        break;
                    }
                }
            }
            match current_block {
                9181201145437202872 => {
                    presel = 0 as libc::c_int;
                    continue '_begin;
                }
                1993610015827684295 => {
                    if r == 97 as libc::c_int {
                        presel = 0 as libc::c_int;
                        continue '_begin;
                    } else {
                        qsort(
                            pdents as *mut libc::c_void,
                            ndents as size_t,
                            ::std::mem::size_of::<entry>() as libc::c_ulong,
                            entrycmpfn,
                        );
                        if ndents != 0 {
                            tmp___56 = dentfind(lastname as *const libc::c_char, ndents);
                            tmp___57 = tmp___56;
                        } else {
                            tmp___57 = 0 as libc::c_int;
                        }
                        move_cursor(tmp___57, 0 as libc::c_int);
                    }
                }
                14756287629061532915 => {
                    if nselected > 1000 as libc::c_int {
                        printmsg(b"processing...\0" as *const u8 as *const libc::c_char);
                        wrefresh(stdscr);
                    } else if nselected != 0 {
                        if ndents > 1000 as libc::c_int {
                            printmsg(
                                b"processing...\0" as *const u8 as *const libc::c_char,
                            );
                            wrefresh(stdscr);
                        }
                    }
                    r = scanselforpath(
                        path as *const libc::c_char,
                        1 as libc::c_int != 0,
                    );
                    if sel as libc::c_uint == 40 as libc::c_uint {
                        if !findselpos.is_null() {
                            invertselbuf(r);
                        } else {
                            addtoselbuf(r, selstartid, selendid);
                        }
                    } else {
                        addtoselbuf(r, selstartid, selendid);
                    }
                }
                6344850934232263781 => {
                    get_archive_cmd(newpath.as_mut_ptr(), tmp as *const libc::c_char);
                    if r == 115 as libc::c_int {
                        archive_selection(
                            newpath.as_mut_ptr() as *const libc::c_char,
                            tmp as *const libc::c_char,
                            path as *const libc::c_char,
                        );
                    } else {
                        spawn(
                            newpath.as_mut_ptr(),
                            tmp,
                            (*pdents.offset(cur as isize)).name,
                            0 as *mut libc::c_void as *mut libc::c_char,
                            25 as libc::c_int as ushort_t,
                        );
                    }
                    mkpath(
                        path as *const libc::c_char,
                        tmp as *const libc::c_char,
                        newpath.as_mut_ptr(),
                    );
                    tmp___94 = access(
                        newpath.as_mut_ptr() as *const libc::c_char,
                        0 as libc::c_int,
                    );
                    if !(tmp___94 == 0 as libc::c_int) {
                        continue;
                    }
                    xstrsncpy(
                        lastname,
                        tmp as *const libc::c_char,
                        256 as libc::c_int as size_t,
                    );
                    clearfilter();
                    clearselection();
                    cd = 0 as libc::c_int != 0;
                    continue '_begin;
                }
                11271090240167667812 => {
                    spawn(
                        opener,
                        newpath.as_mut_ptr(),
                        0 as *mut libc::c_void as *mut libc::c_char,
                        0 as *mut libc::c_void as *mut libc::c_char,
                        opener_flags as ushort_t,
                    );
                    if g_state.autonext() != 0 {
                        if cur != ndents - 1 as libc::c_int {
                            move_cursor(
                                (cur + 1 as libc::c_int) % ndents,
                                0 as libc::c_int,
                            );
                        }
                    }
                    if cfg.filtermode() != 0 {
                        presel = '/' as i32;
                        clearfilter();
                    }
                }
                10882616296157082316 => {
                    if !(g_state.forcequit() == 0) {
                        break '_begin;
                    }
                    r = 0 as libc::c_int;
                    while r < 4 as libc::c_int {
                        if r as uint_t != cfg.curctx() {
                            if (g_ctx[r as usize].c_cfg).ctxactive() != 0 {
                                r = get_input(messages[15 as libc::c_int as usize]);
                                break;
                            }
                        }
                        r += 1;
                    }
                    if r == 4 as libc::c_int {
                        break '_begin;
                    }
                    tmp___124 = xconfirm(r);
                    if tmp___124 {
                        break '_begin;
                    }
                }
                16708736497050535312 => {
                    if sel as libc::c_uint == 33 as libc::c_uint {
                        let ref mut fresh15 = (*pdents.offset(cur as isize)).mode;
                        *fresh15 ^= 73 as libc::c_uint;
                    }
                }
                _ => {
                    ctx = cfg.curctx() as libc::c_int;
                    r = ctx - 1 as libc::c_int & 3 as libc::c_int;
                    while r != ctx {
                        if (g_ctx[r as usize].c_cfg).ctxactive() != 0 {
                            break;
                        }
                        r = r - 1 as libc::c_int & 3 as libc::c_int;
                    }
                    if !(r != ctx) {
                        break '_begin;
                    }
                    (g_ctx[ctx as usize].c_cfg)
                        .set_ctxactive(0 as libc::c_int as uint_t);
                    path = (g_ctx[r as usize].c_path).as_mut_ptr();
                    lastdir = (g_ctx[r as usize].c_last).as_mut_ptr();
                    lastname = (g_ctx[r as usize].c_name).as_mut_ptr();
                    cfg = g_ctx[r as usize].c_cfg;
                    cfg.set_curctx(r as uint_t);
                    if cfg.filtermode() != 0 {
                        presel = '/' as i32;
                    } else {
                        watch = 1 as libc::c_int != 0;
                    }
                    continue '_begin;
                }
            }
        }
        match current_block {
            15502218481637409867 => {
                if !listpath.is_null() {
                    if sel as libc::c_uint == 16 as libc::c_uint {
                        tmp___48 = 0 as *mut libc::c_void as *mut libc::c_char;
                    } else {
                        tmp___48 = lastname;
                    }
                } else {
                    tmp___48 = lastname;
                }
                tmp___49 = cdprep(lastdir, tmp___48, path, newpath.as_mut_ptr());
                if tmp___49 {
                    presel = '/' as i32;
                } else {
                    watch = 1 as libc::c_int != 0;
                }
                continue;
            }
            18207192640746394217 => {
                tmp___118 = strcmp(
                    path as *const libc::c_char,
                    plgpath as *const libc::c_char,
                );
                if tmp___118 == 0 as libc::c_int {
                    xstrsncpy(
                        path,
                        lastdir as *const libc::c_char,
                        4096 as libc::c_int as size_t,
                    );
                    xstrsncpy(
                        lastname,
                        runfile.as_mut_ptr() as *const libc::c_char,
                        256 as libc::c_int as size_t,
                    );
                    runfile[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
                    if cfg.filtermode() != 0 {
                        presel = '/' as i32;
                    } else {
                        watch = 1 as libc::c_int != 0;
                    }
                    continue;
                } else {
                    g_state.set_runplugin(1 as libc::c_int as uint_t);
                }
                current_block = 3733147086002097443;
            }
            6727606101734016984 => {
                if newpath[0 as libc::c_int as usize] != 0 {
                    if ndents != 0 {
                        tmp___39 = (*pdents.offset(cur as isize)).name;
                    } else {
                        tmp___39 = 0 as *mut libc::c_void as *mut libc::c_char;
                    }
                    set_smart_ctx(
                        '+' as i32,
                        newpath.as_mut_ptr(),
                        &mut path,
                        tmp___39,
                        &mut lastname,
                        &mut lastdir,
                    );
                } else {
                    if ndents != 0 {
                        tmp___40 = (*pdents.offset(cur as isize)).name
                            as *const libc::c_char;
                    } else {
                        tmp___40 = b"\0\0" as *const u8 as *const libc::c_char;
                    }
                    xstrsncpy(lastname, tmp___40, 256 as libc::c_int as size_t);
                }
                clearfilter();
                continue;
            }
            3382726224265272059 => {
                if sel as libc::c_uint == 30 as libc::c_uint {
                    cfg.set_showhidden(cfg.showhidden() ^ 1 as libc::c_uint as uint_t);
                    if cfg.filtermode() != 0 {
                        presel = '/' as i32;
                    }
                    clearfilter();
                }
                if ndents != 0 {
                    tmp___53 = (*pdents.offset(cur as isize)).name
                        as *const libc::c_char;
                } else {
                    tmp___53 = b"\0\0" as *const u8 as *const libc::c_char;
                }
                xstrsncpy(lastname, tmp___53, 256 as libc::c_int as size_t);
                cd = 0 as libc::c_int != 0;
                continue;
            }
            3046210090062227051 => {
                close(fd);
                xstrsncpy(
                    lastname,
                    tmp as *const libc::c_char,
                    256 as libc::c_int as size_t,
                );
                current_block = 5617391961331132692;
            }
            14559976248321351685 => {
                if ndents != 0 {
                    tmp___72 = (*pdents.offset(cur as isize)).name
                        as *const libc::c_char;
                } else {
                    tmp___72 = b"\0\0" as *const u8 as *const libc::c_char;
                }
                xstrsncpy(lastname, tmp___72, 256 as libc::c_int as size_t);
                cd = 0 as libc::c_int != 0;
                continue;
            }
            14904012976982599327 => {
                if ndents != 0 {
                    tmp___51 = (*pdents.offset(cur as isize)).name;
                } else {
                    tmp___51 = 0 as *mut libc::c_void as *mut libc::c_char;
                }
                set_smart_ctx(
                    '+' as i32,
                    newpath.as_mut_ptr(),
                    &mut path,
                    tmp___51,
                    &mut lastname,
                    &mut lastdir,
                );
                clearfilter();
                continue;
            }
            _ => {}
        }
        match current_block {
            3733147086002097443 => {
                xstrsncpy(
                    lastdir,
                    path as *const libc::c_char,
                    4096 as libc::c_int as size_t,
                );
                xstrsncpy(
                    path,
                    plgpath as *const libc::c_char,
                    4096 as libc::c_int as size_t,
                );
                if ndents != 0 {
                    xstrsncpy(
                        runfile.as_mut_ptr(),
                        (*pdents.offset(cur as isize)).name as *const libc::c_char,
                        255 as libc::c_int as size_t,
                    );
                }
                g_state.set_runctx(cfg.curctx());
                *lastname
                    .offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
            }
            5617391961331132692 => {
                cd = 0 as libc::c_int != 0;
                continue;
            }
            _ => {}
        }
        if cfg.filtermode() != 0 {
            presel = '/' as i32;
        } else {
            watch = 1 as libc::c_int != 0;
        }
        clearfilter();
        if g_state.runplugin() == 1 as libc::c_uint {
            presel = '/' as i32;
        }
    }
    tmp = getenv(b"NNN_TMPFILE\0" as *const u8 as *const libc::c_char);
    let mut current_block_920: u64;
    if sel as libc::c_uint == 63 as libc::c_uint {
        current_block_920 = 6342536648414161247;
    } else if !tmp.is_null() {
        current_block_920 = 6342536648414161247;
    } else {
        current_block_920 = 7813901650037356473;
    }
    match current_block_920 {
        6342536648414161247 => {
            write_lastdir(path as *const libc::c_char, tmp as *const libc::c_char);
            if sel as libc::c_uint == 63 as libc::c_uint {
                if g_state.picker() != 0 {
                    selbufpos = 0 as libc::c_int as uint_t;
                }
            }
        }
        _ => {}
    }
    if sel as libc::c_uint != 65 as libc::c_uint {
        return 0 as libc::c_int != 0;
    }
    if selbufpos != 0 {
        if g_state.picker() == 0 {
            g_state.set_picker(1 as libc::c_int as uint_t);
            free(selpath as *mut libc::c_void);
            selpath = 0 as *mut libc::c_void as *mut libc::c_char;
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn make_tmp_tree(
    mut paths: *mut *mut libc::c_char,
    mut entries: ssize_t,
    mut prefix: *const libc::c_char,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut err: libc::c_int = 0;
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
    let mut slash: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: ssize_t = 0;
    let mut tmp___0: size_t = 0;
    let mut tmpdir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: ssize_t = 0;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___4: size_t = 0;
    let mut tmp___5: size_t = 0;
    let mut tmp___6: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___7: libc::c_int = 0;
    tmp___0 = xstrlen(prefix);
    len = tmp___0 as ssize_t;
    tmp___1 = malloc(4096 as libc::c_int as size_t);
    tmpdir = tmp___1 as *mut libc::c_char;
    if tmpdir.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    tmp = tmpdir
        .offset(tmpfplen as libc::c_int as isize)
        .offset(-(1 as libc::c_int as isize));
    xstrsncpy(
        tmpdir,
        g_tmpfpath.as_mut_ptr() as *const libc::c_char,
        tmpfplen as size_t,
    );
    xstrsncpy(
        tmp,
        b"/nnnXXXXXX\0" as *const u8 as *const libc::c_char,
        11 as libc::c_int as size_t,
    );
    tmp = tmp.offset(10 as libc::c_int as isize);
    if *prefix.offset(1 as libc::c_int as isize) == 0 {
        if *prefix.offset(0 as libc::c_int as isize) as libc::c_int == 47 as libc::c_int
        {
            len = 0 as libc::c_int as ssize_t;
        }
    }
    tmp___2 = mkdtemp(tmpdir);
    if tmp___2.is_null() {
        free(tmpdir as *mut libc::c_void);
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    listpath = tmpdir;
    i = 0 as libc::c_int as ssize_t;
    while i < entries {
        if !(*paths.offset(i as isize)).is_null() {
            err = stat(
                *paths.offset(i as isize) as *const libc::c_char,
                &mut sb as *mut stat,
            );
            if err != 0 {
                tmp___3 = __errno_location();
                if *tmp___3 == 2 as libc::c_int {
                    current_block = 3745403955677880229;
                } else {
                    current_block = 15925075030174552612;
                }
            } else {
                current_block = 15925075030174552612;
            }
            match current_block {
                3745403955677880229 => {}
                _ => {
                    tmp___4 = xstrlen(*paths.offset(i as isize) as *const libc::c_char);
                    xstrsncpy(
                        tmp,
                        (*paths.offset(i as isize)).offset(len as isize)
                            as *const libc::c_char,
                        tmp___4
                            .wrapping_sub(len as size_t)
                            .wrapping_add(1 as libc::c_ulong),
                    );
                    tmp___5 = xstrlen(*paths.offset(i as isize) as *const libc::c_char);
                    tmp___6 = xmemrchr(
                        tmp as *mut uchar_t,
                        '/' as i32 as uchar_t,
                        tmp___5.wrapping_sub(len as size_t),
                    );
                    slash = tmp___6 as *mut libc::c_char;
                    if !slash.is_null() {
                        *slash = '\u{0}' as i32 as libc::c_char;
                    }
                    tmp___7 = access(tmpdir as *const libc::c_char, 0 as libc::c_int);
                    if tmp___7 != 0 {
                        xmktree(tmpdir, 1 as libc::c_int != 0);
                    }
                    if !slash.is_null() {
                        *slash = '/' as i32 as libc::c_char;
                    }
                    symlink(
                        *paths.offset(i as isize) as *const libc::c_char,
                        tmpdir as *const libc::c_char,
                    );
                }
            }
        }
        i += 1;
    }
    *tmp = '\u{0}' as i32 as libc::c_char;
    return tmpdir;
}
unsafe extern "C" fn load_input(
    mut fd: libc::c_int,
    mut path: *const libc::c_char,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut i: ssize_t = 0;
    let mut chunk_count: ssize_t = 0;
    let mut chunk: ssize_t = 0;
    let mut entries: ssize_t = 0;
    let mut input: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmpdir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cwd: [libc::c_char; 4096] = [0; 4096];
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut offsets: [size_t; 16384] = [0; 16384];
    let mut paths: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut input_read: ssize_t = 0;
    let mut total_read: ssize_t = 0;
    let mut off: ssize_t = 0;
    let mut msgnum: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: ssize_t = 0;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut buf: [libc::c_char; 512] = [0; 512];
    let mut tmp___4: ssize_t = 0;
    let mut tmp___5: ssize_t = 0;
    let mut tmp___6: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___7: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___8: bool = false;
    let mut tmp___9: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___10: *mut libc::c_char = 0 as *mut libc::c_char;
    chunk_count = 1 as libc::c_int as ssize_t;
    chunk = 524288 as libc::c_int as ssize_t;
    entries = 0 as libc::c_int as ssize_t;
    tmp = malloc(
        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(chunk as libc::c_ulong),
    );
    input = tmp as *mut libc::c_char;
    tmpdir = 0 as *mut libc::c_void as *mut libc::c_char;
    paths = 0 as *mut libc::c_void as *mut *mut libc::c_char;
    total_read = 0 as libc::c_int as ssize_t;
    off = 0 as libc::c_int as ssize_t;
    msgnum = 0 as libc::c_int;
    if input.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    if path.is_null() {
        tmp___0 = getcwd(cwd.as_mut_ptr(), 4096 as libc::c_int as size_t);
        if tmp___0.is_null() {
            free(input as *mut libc::c_void);
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
    } else {
        xstrsncpy(cwd.as_mut_ptr(), path, 4096 as libc::c_int as size_t);
    }
    loop {
        if !(chunk_count < 512 as libc::c_long) {
            current_block = 8347882395825654554;
            break;
        }
        if msgnum != 0 {
            current_block = 8347882395825654554;
            break;
        }
        input_read = read(
            fd,
            input.offset(total_read as isize) as *mut libc::c_void,
            chunk as size_t,
        );
        if input_read < 0 as libc::c_long {
            current_block = 3778618757952947976;
            break;
        }
        if input_read == 0 as libc::c_long {
            current_block = 8347882395825654554;
            break;
        }
        total_read += input_read;
        chunk_count += 1;
        while off < total_read {
            tmp___1 = memchr(
                input.offset(off as isize) as *const libc::c_void,
                '\u{0}' as i32,
                (total_read - off) as size_t,
            );
            next = tmp___1 as *mut libc::c_char;
            if next as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                break;
            }
            next = next.offset(1);
            if next.offset_from(input) as libc::c_long == off + 1 as libc::c_long {
                off = next.offset_from(input) as libc::c_long;
            } else if entries == ((1 as libc::c_int) << 14 as libc::c_int) as ssize_t {
                msgnum = 44 as libc::c_int;
                break;
            } else {
                tmp___2 = entries;
                entries += 1;
                offsets[tmp___2 as usize] = off as size_t;
                off = next.offset_from(input) as libc::c_long;
            }
        }
        if chunk_count == 512 as libc::c_long {
            msgnum = 10 as libc::c_int;
            current_block = 8347882395825654554;
            break;
        } else {
            if chunk_count == (total_read - input_read) / chunk {
                continue;
            }
            chunk_count = total_read / chunk;
            if total_read % chunk != 0 {
                chunk_count += 1;
            }
            tmp___3 = xrealloc(
                input as *mut libc::c_void,
                ((chunk_count + 1 as libc::c_long) * chunk) as size_t,
            );
            input = tmp___3 as *mut libc::c_char;
            if input.is_null() {
                return 0 as *mut libc::c_void as *mut libc::c_char;
            }
        }
    }
    match current_block {
        8347882395825654554 => {
            if msgnum != 0 {
                loop {
                    tmp___4 = read(
                        fd,
                        buf.as_mut_ptr() as *mut libc::c_void,
                        512 as libc::c_int as size_t,
                    );
                    if !(tmp___4 > 0 as libc::c_long) {
                        break;
                    }
                }
            }
            if off != total_read {
                if entries == ((1 as libc::c_int) << 14 as libc::c_int) as ssize_t {
                    msgnum = 44 as libc::c_int;
                } else {
                    tmp___5 = entries;
                    entries += 1;
                    offsets[tmp___5 as usize] = off as size_t;
                }
            }
            if entries == 0 {
                msgnum = 1 as libc::c_int;
            } else {
                *input.offset(total_read as isize) = '\u{0}' as i32 as libc::c_char;
                tmp___6 = malloc(
                    (entries as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        ),
                );
                paths = tmp___6 as *mut *mut libc::c_char;
                if !paths.is_null() {
                    i = 0 as libc::c_int as ssize_t;
                    while i < entries {
                        let ref mut fresh17 = *paths.offset(i as isize);
                        *fresh17 = input.offset(offsets[i as usize] as isize);
                        i += 1;
                    }
                    tmp___7 = malloc(
                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                            .wrapping_mul(4096 as libc::c_ulong),
                    );
                    listroot = tmp___7 as *mut libc::c_char;
                    if !listroot.is_null() {
                        *listroot
                            .offset(
                                0 as libc::c_int as isize,
                            ) = '\u{0}' as i32 as libc::c_char;
                        i = 0 as libc::c_int as ssize_t;
                        loop {
                            if !(i < entries) {
                                current_block = 18425699056680496821;
                                break;
                            }
                            if *(*paths.offset(i as isize))
                                .offset(0 as libc::c_int as isize) as libc::c_int
                                == 10 as libc::c_int
                            {
                                let ref mut fresh18 = *paths.offset(i as isize);
                                *fresh18 = 0 as *mut libc::c_void as *mut libc::c_char;
                            } else {
                                tmp___8 = selforparent(
                                    *paths.offset(i as isize) as *const libc::c_char,
                                );
                                if tmp___8 {
                                    let ref mut fresh19 = *paths.offset(i as isize);
                                    *fresh19 = 0 as *mut libc::c_void as *mut libc::c_char;
                                } else {
                                    let ref mut fresh20 = *paths.offset(i as isize);
                                    *fresh20 = abspath(
                                        *paths.offset(i as isize) as *const libc::c_char,
                                        cwd.as_mut_ptr() as *const libc::c_char,
                                        0 as *mut libc::c_void as *mut libc::c_char,
                                    );
                                    if (*paths.offset(i as isize)).is_null() {
                                        entries = i;
                                        current_block = 14349302347827478602;
                                        break;
                                    } else {
                                        xstrsncpy(
                                            g_buf.as_mut_ptr(),
                                            *paths.offset(i as isize) as *const libc::c_char,
                                            4096 as libc::c_int as size_t,
                                        );
                                        tmp___9 = xdirname(g_buf.as_mut_ptr());
                                        tmp___10 = common_prefix(
                                            tmp___9 as *const libc::c_char,
                                            listroot,
                                        );
                                        if tmp___10.is_null() {
                                            entries = i + 1 as libc::c_long;
                                            current_block = 14349302347827478602;
                                            break;
                                        }
                                    }
                                }
                            }
                            i += 1;
                        }
                        match current_block {
                            18425699056680496821 => {
                                if *listroot.offset(0 as libc::c_int as isize) != 0 {
                                    tmpdir = make_tmp_tree(
                                        paths,
                                        entries,
                                        listroot as *const libc::c_char,
                                    );
                                }
                            }
                            _ => {}
                        }
                        i = entries - 1 as libc::c_long;
                        while i >= 0 as libc::c_long {
                            free(*paths.offset(i as isize) as *mut libc::c_void);
                            i -= 1;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if msgnum != 0 {
        if !home.is_null() {
            printmsg(messages[msgnum as usize]);
            xdelay(((350000 as libc::c_int) << 2 as libc::c_int) as useconds_t);
        } else {
            msg(messages[msgnum as usize]);
            usleep(((350000 as libc::c_int) << 2 as libc::c_int) as __useconds_t);
        }
    }
    free(input as *mut libc::c_void);
    free(paths as *mut libc::c_void);
    return tmpdir;
}
unsafe extern "C" fn check_key_collision() {
    let mut key: libc::c_int = 0;
    let mut bitmap: [bool; 511] = [false; 511];
    let mut tmp: libc::c_uint = 0;
    let mut i: ullong_t = 0;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    bitmap[0 as libc::c_int as usize] = 0 as libc::c_int != 0;
    tmp = 1 as libc::c_uint;
    while !(tmp >= 511 as libc::c_uint) {
        bitmap[tmp as usize] = 0 as libc::c_int != 0;
        tmp = tmp.wrapping_add(1);
    }
    i = 0 as libc::c_int as ullong_t;
    while i
        < (::std::mem::size_of::<[key; 84]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<key>() as libc::c_ulong) as ullong_t
    {
        key = bindings[i as usize].sym as libc::c_int;
        if bitmap[key as usize] {
            tmp___0 = keyname(key);
            dprintf(
                2 as libc::c_int,
                b"key collision! [%s]\n\0" as *const u8 as *const libc::c_char,
                tmp___0,
            );
        } else {
            bitmap[key as usize] = 1 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn usage() {
    dprintf(
        1 as libc::c_int,
        b"%s: nnn [OPTIONS] [PATH]\n\nThe unorthodox terminal file manager.\n\npositional args:\n  PATH   start dir/file [default: .]\n\noptional args:\n -A      no dir auto-enter during filter\n -b key  open bookmark key (trumps -s/S)\n -B      use bsdtar for archives\n -c      cli-only NNN_OPENER (trumps -e)\n -C      8-color scheme\n -d      detail mode\n -D      dirs in context color\n -e      text in $VISUAL/$EDITOR/vi\n -E      internal edits in EDITOR\n -g      regex filters\n -H      show hidden files\n -i      show current file info\n -J      no auto-advance on selection\n -K      detect key collision\n -l val  set scroll lines\n -n      type-to-nav mode\n -o      open files only on Enter\n -p file selection file [-:stdout]\n -P key  run plugin key\n -Q      no quit confirmation\n -r      use advcpmv patched cp, mv\n -R      no rollover at edges\n -t secs timeout to lock\n -T key  sort order [a/d/e/r/s/t/v]\n -u      use selection (no prompt)\n -V      show version\n -h      show help\n\nv%s\n%s\n\0"
            as *const u8 as *const libc::c_char,
        b"usage\0" as *const u8 as *const libc::c_char,
        b"4.6\0" as *const u8 as *const libc::c_char,
        b"BSD 2-Clause\nhttps://github.com/jarun/nnn\0" as *const u8
            as *const libc::c_char,
    );
}
unsafe extern "C" fn setup_config() -> bool {
    let mut r: size_t = 0;
    let mut len: size_t = 0;
    let mut xdgcfg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut xdg: bool = false;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: bool = false;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: size_t = 0;
    let mut tmp___4: size_t = 0;
    let mut tmp___5: size_t = 0;
    let mut tmp___6: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___7: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___8: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___9: size_t = 0;
    let mut tmp___10: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: bool = false;
    let mut env_sel: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___13: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___14: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___15: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___16: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = getenv(b"XDG_CONFIG_HOME\0" as *const u8 as *const libc::c_char);
    xdgcfg = tmp;
    xdg = 0 as libc::c_int != 0;
    if !xdgcfg.is_null() {
        if *xdgcfg.offset(0 as libc::c_int as isize) != 0 {
            if *xdgcfg.offset(0 as libc::c_int as isize) as libc::c_int
                == 126 as libc::c_int
            {
                r = xstrsncpy(
                    g_buf.as_mut_ptr(),
                    home as *const libc::c_char,
                    4096 as libc::c_int as size_t,
                );
                xstrsncpy(
                    g_buf
                        .as_mut_ptr()
                        .offset(r as isize)
                        .offset(-(1 as libc::c_int as isize)),
                    xdgcfg.offset(1 as libc::c_int as isize) as *const libc::c_char,
                    4096 as libc::c_int as size_t,
                );
                xdgcfg = g_buf.as_mut_ptr();
            }
            tmp___1 = xdiraccess(xdgcfg as *const libc::c_char);
            if !tmp___1 {
                tmp___0 = xitoa(8196 as libc::c_int as uint_t);
                perror(tmp___0 as *const libc::c_char);
                return 0 as libc::c_int != 0;
            }
            tmp___2 = xstrlen(xdgcfg as *const libc::c_char);
            tmp___3 = xstrlen(b"/nnn/bookmarks\0" as *const u8 as *const libc::c_char);
            len = tmp___2.wrapping_add(tmp___3).wrapping_add(1 as libc::c_ulong);
            xdg = 1 as libc::c_int != 0;
        }
    }
    if !xdg {
        tmp___4 = xstrlen(home as *const libc::c_char);
        tmp___5 = xstrlen(
            b"/.config/nnn/bookmarks\0" as *const u8 as *const libc::c_char,
        );
        len = tmp___4.wrapping_add(tmp___5).wrapping_add(1 as libc::c_ulong);
    }
    tmp___6 = malloc(len);
    cfgpath = tmp___6 as *mut libc::c_char;
    tmp___7 = malloc(len);
    plgpath = tmp___7 as *mut libc::c_char;
    if cfgpath.is_null() {
        tmp___8 = xitoa(8210 as libc::c_int as uint_t);
        perror(tmp___8 as *const libc::c_char);
        return 0 as libc::c_int != 0;
    } else {
        if plgpath.is_null() {
            tmp___8 = xitoa(8210 as libc::c_int as uint_t);
            perror(tmp___8 as *const libc::c_char);
            return 0 as libc::c_int != 0;
        }
    }
    if xdg {
        xstrsncpy(cfgpath, xdgcfg as *const libc::c_char, len);
        tmp___9 = xstrlen(b"/nnn/bookmarks\0" as *const u8 as *const libc::c_char);
        r = len.wrapping_sub(tmp___9);
    } else {
        r = xstrsncpy(cfgpath, home as *const libc::c_char, len);
        xstrsncpy(
            cfgpath.offset(r as isize).offset(-(1 as libc::c_int as isize)),
            b"/.config\0" as *const u8 as *const libc::c_char,
            len.wrapping_sub(r),
        );
        r = (r as libc::c_ulong).wrapping_add(8 as libc::c_ulong) as size_t as size_t;
    }
    xstrsncpy(
        cfgpath.offset(r as isize).offset(-(1 as libc::c_int as isize)),
        b"/nnn\0" as *const u8 as *const libc::c_char,
        len.wrapping_sub(r),
    );
    r = 0 as libc::c_int as size_t;
    while r
        < (::std::mem::size_of::<[*const libc::c_char; 4]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        mkpath(cfgpath as *const libc::c_char, toks[r as usize], plgpath);
        tmp___11 = access(plgpath as *const libc::c_char, 0 as libc::c_int);
        if tmp___11 != 0 {
            tmp___12 = xmktree(plgpath, 1 as libc::c_int != 0);
            if !tmp___12 {
                tmp___10 = xitoa(8236 as libc::c_int as uint_t);
                perror(tmp___10 as *const libc::c_char);
                return 0 as libc::c_int != 0;
            }
        }
        r = r.wrapping_add(1);
    }
    if g_state.picker() == 0 {
        tmp___13 = xgetenv(
            env_cfg[9 as libc::c_int as usize],
            0 as *mut libc::c_void as *mut libc::c_char,
        );
        env_sel = tmp___13;
        if !env_sel.is_null() {
            tmp___14 = xstrdup(env_sel as *const libc::c_char);
            selpath = tmp___14;
        } else {
            tmp___15 = malloc(len.wrapping_add(3 as libc::c_ulong));
            selpath = tmp___15 as *mut libc::c_char;
        }
        if selpath.is_null() {
            tmp___16 = xitoa(8249 as libc::c_int as uint_t);
            perror(tmp___16 as *const libc::c_char);
            return 0 as libc::c_int != 0;
        }
        if env_sel.is_null() {
            r = xstrsncpy(
                selpath,
                cfgpath as *const libc::c_char,
                len.wrapping_add(3 as libc::c_ulong),
            );
            xstrsncpy(
                selpath.offset(r as isize).offset(-(1 as libc::c_int as isize)),
                b"/.selection\0" as *const u8 as *const libc::c_char,
                12 as libc::c_int as size_t,
            );
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn set_tmp_path() -> bool {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: bool = false;
    let mut tmp___4: size_t = 0;
    tmp = b"/tmp\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    tmp___3 = xdiraccess(tmp as *const libc::c_char);
    if tmp___3 {
        tmp___2 = tmp;
    } else {
        tmp___1 = getenv(b"TMPDIR\0" as *const u8 as *const libc::c_char);
        tmp___2 = tmp___1;
    }
    path = tmp___2;
    if path.is_null() {
        msg(b"set TMPDIR\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int != 0;
    }
    tmp___4 = xstrsncpy(
        g_tmpfpath.as_mut_ptr(),
        path as *const libc::c_char,
        64 as libc::c_int as size_t,
    );
    tmpfplen = tmp___4 as uchar_t;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn cleanup() {
    free(selpath as *mut libc::c_void);
    free(plgpath as *mut libc::c_void);
    free(cfgpath as *mut libc::c_void);
    free(initpath as *mut libc::c_void);
    free(bmstr as *mut libc::c_void);
    free(pluginstr as *mut libc::c_void);
    free(listroot as *mut libc::c_void);
    free(ihashbmp as *mut libc::c_void);
    free(bookmark as *mut libc::c_void);
    free(plug as *mut libc::c_void);
    free(lastcmd as *mut libc::c_void);
    if g_state.pluginit() != 0 {
        unlink(g_pipepath.as_mut_ptr() as *const libc::c_char);
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut session: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fd: libc::c_int = 0;
    let mut opt: libc::c_int = 0;
    let mut sort: libc::c_int = 0;
    let mut pkey: libc::c_int = 0;
    let mut env_opts: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut env_opts_id: libc::c_int = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: bool = false;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: size_t = 0;
    let mut tmp___10: bool = false;
    let mut tmp___11: bool = false;
    let mut tmp___12: bool = false;
    let mut tmp___13: bool = false;
    let mut startpath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___14: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___15: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___16: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___17: size_t = 0;
    let mut tmp___18: bool = false;
    let mut tmp___19: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___20: *mut libc::c_char = 0 as *mut libc::c_char;
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
    let mut tmp___21: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___22: libc::c_int = 0;
    let mut tmp___23: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___24: libc::c_int = 0;
    let mut tmp___25: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___26: uint_t = 0;
    let mut act: sigaction = sigaction {
        __sigaction_handler: __anonunion___sigaction_handler_363639592 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut tmp___27: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___28: libc::c_int = 0;
    let mut tmp___29: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___30: libc::c_int = 0;
    let mut tmp___31: libc::c_int = 0;
    let mut tmp___32: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___33: libc::c_int = 0;
    let mut tmp___34: bool = false;
    let mut tmp___35: bool = false;
    let mut tmp___36: libc::c_int = 0;
    let mut tmp___37: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___38: size_t = 0;
    arg = 0 as *mut libc::c_void as *mut libc::c_char;
    session = 0 as *mut libc::c_void as *mut libc::c_char;
    sort = 0 as libc::c_int;
    pkey = '\u{0}' as i32;
    tmp = xgetenv(
        env_cfg[0 as libc::c_int as usize],
        0 as *mut libc::c_void as *mut libc::c_char,
    );
    env_opts = tmp as *const libc::c_char;
    if !env_opts.is_null() {
        tmp___0 = xstrlen(env_opts);
        tmp___1 = tmp___0 as libc::c_int;
    } else {
        tmp___1 = -(1 as libc::c_int);
    }
    env_opts_id = tmp___1;
    loop {
        if env_opts_id > 0 as libc::c_int {
            env_opts_id -= 1;
            opt = *env_opts.offset(env_opts_id as isize) as libc::c_int;
        } else {
            tmp___4 = getopt(
                argc,
                argv as *const *mut libc::c_char,
                b"aAb:BcCdDeEfF:gHiJKl:nop:P:QrRs:St:T:uUVxh\0" as *const u8
                    as *const libc::c_char,
            );
            opt = tmp___4;
        }
        if !(opt != -(1 as libc::c_int)) {
            break;
        }
        match opt {
            65 => {
                cfg.set_autoenter(0 as libc::c_int as uint_t);
            }
            98 => {
                if env_opts_id < 0 as libc::c_int {
                    arg = optarg;
                }
            }
            66 => {
                g_state.set_usebsdtar(1 as libc::c_int as uint_t);
            }
            99 => {
                cfg.set_cliopener(1 as libc::c_int as uint_t);
            }
            67 => {
                g_state.set_oldcolor(1 as libc::c_int as uint_t);
            }
            100 => {
                cfg.set_showdetail(1 as libc::c_int as uint_t);
            }
            68 => {
                g_state.set_dirctx(1 as libc::c_int as uint_t);
            }
            101 => {
                cfg.set_useeditor(1 as libc::c_int as uint_t);
            }
            69 => {
                cfg.set_waitedit(1 as libc::c_int as uint_t);
            }
            102 => {}
            103 => {
                cfg.set_regex(1 as libc::c_int as uint_t);
                filterfn = Some(
                    visible_re
                        as unsafe extern "C" fn(
                            *const fltrexp_t,
                            *const libc::c_char,
                        ) -> libc::c_int,
                );
            }
            72 => {
                cfg.set_showhidden(1 as libc::c_int as uint_t);
            }
            105 => {
                cfg.set_fileinfo(1 as libc::c_int as uint_t);
            }
            74 => {
                g_state.set_stayonsel(1 as libc::c_int as uint_t);
            }
            75 => {
                check_key_collision();
                return 0 as libc::c_int;
            }
            108 => {
                if env_opts_id < 0 as libc::c_int {
                    scroll_lines = atoi(optarg as *const libc::c_char);
                }
            }
            110 => {
                cfg.set_filtermode(1 as libc::c_int as uint_t);
            }
            111 => {
                cfg.set_nonavopen(1 as libc::c_int as uint_t);
            }
            112 => {
                if !(env_opts_id >= 0 as libc::c_int) {
                    g_state.set_picker(1 as libc::c_int as uint_t);
                    let mut current_block_52: u64;
                    if *optarg.offset(0 as libc::c_int as isize) as libc::c_int
                        == 45 as libc::c_int
                    {
                        if !(*optarg.offset(1 as libc::c_int as isize) as libc::c_int
                            == 0 as libc::c_int)
                        {
                            current_block_52 = 3936204310756686921;
                        } else {
                            current_block_52 = 13723035087248630346;
                        }
                    } else {
                        current_block_52 = 3936204310756686921;
                    }
                    match current_block_52 {
                        3936204310756686921 => {
                            fd = open(
                                optarg as *const libc::c_char,
                                65 as libc::c_int,
                                384 as libc::c_int,
                            );
                            if fd == -(1 as libc::c_int) {
                                tmp___2 = xitoa(8416 as libc::c_int as uint_t);
                                perror(tmp___2 as *const libc::c_char);
                                return 1 as libc::c_int;
                            }
                            close(fd);
                            selpath = abspath(
                                optarg as *const libc::c_char,
                                0 as *mut libc::c_void as *const libc::c_char,
                                0 as *mut libc::c_void as *mut libc::c_char,
                            );
                            unlink(selpath as *const libc::c_char);
                        }
                        _ => {}
                    }
                }
            }
            80 => {
                if env_opts_id < 0 as libc::c_int {
                    if *optarg.offset(1 as libc::c_int as isize) == 0 {
                        pkey = *optarg.offset(0 as libc::c_int as isize) as uchar_t
                            as libc::c_int;
                    }
                }
            }
            81 => {
                g_state.set_forcequit(1 as libc::c_int as uint_t);
            }
            114 => {
                mv[5 as libc::c_int as usize] = 'g' as i32 as libc::c_char;
                mv[2 as libc::c_int as usize] = mv[5 as libc::c_int as usize];
                cp[5 as libc::c_int as usize] = mv[2 as libc::c_int as usize];
                cp[2 as libc::c_int as usize] = cp[5 as libc::c_int as usize];
                mv[4 as libc::c_int as usize] = '-' as i32 as libc::c_char;
                cp[4 as libc::c_int as usize] = mv[4 as libc::c_int as usize];
            }
            82 => {
                cfg.set_rollover(0 as libc::c_int as uint_t);
            }
            116 => {
                if env_opts_id < 0 as libc::c_int {
                    tmp___3 = atoi(optarg as *const libc::c_char);
                    idletimeout = tmp___3 as uint_t;
                }
            }
            84 => {
                if env_opts_id < 0 as libc::c_int {
                    sort = *optarg.offset(0 as libc::c_int as isize) as uchar_t
                        as libc::c_int;
                }
            }
            117 => {
                cfg.set_prefersel(1 as libc::c_int as uint_t);
            }
            85 => {
                g_state.set_uidgid(1 as libc::c_int as uint_t);
            }
            86 => {
                dprintf(
                    1 as libc::c_int,
                    b"%s\n\0" as *const u8 as *const libc::c_char,
                    b"4.6\0" as *const u8 as *const libc::c_char,
                );
                return 0 as libc::c_int;
            }
            120 => {
                cfg.set_x11(1 as libc::c_int as uint_t);
            }
            104 => {
                usage();
                return 0 as libc::c_int;
            }
            _ => {
                usage();
                return 1 as libc::c_int;
            }
        }
        if env_opts_id == 0 as libc::c_int {
            env_opts_id = -(1 as libc::c_int);
        }
    }
    tmp___5 = set_tmp_path();
    if !tmp___5 {
        return 1 as libc::c_int;
    }
    atexit(Some(cleanup as unsafe extern "C" fn() -> ()));
    tmp___8 = isatty(0 as libc::c_int);
    if tmp___8 == 0 {
        initpath = load_input(
            0 as libc::c_int,
            0 as *mut libc::c_void as *const libc::c_char,
        );
        if initpath.is_null() {
            return 1 as libc::c_int;
        }
        tmp___7 = isatty(1 as libc::c_int);
        if tmp___7 != 0 {
            dup2(1 as libc::c_int, 0 as libc::c_int);
        } else {
            tmp___6 = ctermid(0 as *mut libc::c_void as *mut libc::c_char);
            fd = open(
                tmp___6 as *const libc::c_char,
                0 as libc::c_int,
                256 as libc::c_int,
            );
            dup2(fd, 0 as libc::c_int);
            close(fd);
        }
        if !session.is_null() {
            session = 0 as *mut libc::c_void as *mut libc::c_char;
        }
    }
    home = getenv(b"HOME\0" as *const u8 as *const libc::c_char);
    if home.is_null() {
        msg(b"set HOME\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    tmp___9 = xstrlen(home as *const libc::c_char);
    homelen = tmp___9 as uchar_t;
    tmp___10 = setup_config();
    if !tmp___10 {
        return 1 as libc::c_int;
    }
    opener = xgetenv(
        env_cfg[3 as libc::c_int as usize],
        utils[0 as libc::c_int as usize],
    );
    tmp___11 = parsekvpair(
        &mut bookmark,
        &mut bmstr,
        1 as libc::c_int as uchar_t,
        &mut maxbm,
    );
    if !tmp___11 {
        msg(env_cfg[1 as libc::c_int as usize]);
        return 1 as libc::c_int;
    }
    tmp___12 = parsekvpair(
        &mut plug,
        &mut pluginstr,
        2 as libc::c_int as uchar_t,
        &mut maxplug,
    );
    if !tmp___12 {
        msg(env_cfg[2 as libc::c_int as usize]);
        return 1 as libc::c_int;
    }
    tmp___13 = parsekvpair(
        &mut order,
        &mut orderstr,
        11 as libc::c_int as uchar_t,
        &mut maxorder,
    );
    if !tmp___13 {
        msg(env_cfg[11 as libc::c_int as usize]);
        return 1 as libc::c_int;
    }
    if initpath.is_null() {
        if !arg.is_null() {
            if *arg.offset(1 as libc::c_int as isize) == 0 {
                initpath = get_kv_val(
                    bookmark,
                    0 as *mut libc::c_void as *mut libc::c_char,
                    *arg as libc::c_int,
                    maxbm,
                    1 as libc::c_int as uchar_t,
                );
            }
            if initpath.is_null() {
                msg(messages[40 as libc::c_int as usize]);
                return 1 as libc::c_int;
            }
            if !session.is_null() {
                session = 0 as *mut libc::c_void as *mut libc::c_char;
            }
        } else if argc == optind {
            tmp___14 = getenv(b"PWD\0" as *const u8 as *const libc::c_char);
            startpath = tmp___14;
            if !startpath.is_null() {
                tmp___15 = xstrdup(startpath as *const libc::c_char);
                initpath = tmp___15;
            } else {
                tmp___16 = getcwd(
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as libc::c_int as size_t,
                );
                initpath = tmp___16;
            }
            if initpath.is_null() {
                initpath = b"/\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
        } else {
            arg = *argv.offset(optind as isize);
            tmp___17 = xstrlen(arg as *const libc::c_char);
            if tmp___17 > 7 as libc::c_ulong {
                tmp___18 = is_prefix(
                    arg as *const libc::c_char,
                    b"file://\0" as *const u8 as *const libc::c_char,
                    7 as libc::c_int as size_t,
                );
                if tmp___18 {
                    arg = arg.offset(7 as libc::c_int as isize);
                }
            }
            initpath = abspath(
                arg as *const libc::c_char,
                0 as *mut libc::c_void as *const libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_char,
            );
            if initpath.is_null() {
                tmp___19 = xitoa(8573 as libc::c_int as uint_t);
                perror(tmp___19 as *const libc::c_char);
                return 1 as libc::c_int;
            }
            tmp___20 = xbasename(initpath);
            if *tmp___20 as libc::c_int == 46 as libc::c_int {
                cfg.set_showhidden(1 as libc::c_int as uint_t);
            }
            tmp___22 = stat(initpath as *const libc::c_char, &mut sb as *mut stat);
            if tmp___22 == -(1 as libc::c_int) {
                tmp___21 = xitoa(8589 as libc::c_int as uint_t);
                perror(tmp___21 as *const libc::c_char);
                return 1 as libc::c_int;
            }
            if !(sb.st_mode & 61440 as libc::c_uint == 16384 as libc::c_uint) {
                g_state.set_initfile(1 as libc::c_int as uint_t);
            }
            if !session.is_null() {
                session = 0 as *mut libc::c_void as *mut libc::c_char;
            }
        }
    }
    enveditor = getenv(env_cfg[10 as libc::c_int as usize]);
    if !enveditor.is_null() {
        tmp___23 = enveditor as *const libc::c_char;
    } else {
        tmp___23 = patterns[2 as libc::c_int as usize];
    }
    tmp___24 = setfilter(&mut archive_re, tmp___23);
    if tmp___24 != 0 {
        msg(messages[35 as libc::c_int as usize]);
        return 1 as libc::c_int;
    }
    if cfg.cliopener() != 0 {
        cfg.set_useeditor(0 as libc::c_int as uint_t);
    }
    enveditor = xgetenv(
        envs[2 as libc::c_int as usize],
        utils[11 as libc::c_int as usize],
    );
    editor = xgetenv(envs[1 as libc::c_int as usize], enveditor);
    pager = xgetenv(envs[3 as libc::c_int as usize], utils[12 as libc::c_int as usize]);
    shell = xgetenv(envs[0 as libc::c_int as usize], utils[13 as libc::c_int as usize]);
    inotify_fd = inotify_init1(526336 as libc::c_int);
    if inotify_fd < 0 as libc::c_int {
        tmp___25 = xitoa(8659 as libc::c_int as uint_t);
        perror(tmp___25 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    tmp___26 = xgetenv_val(env_cfg[13 as libc::c_int as usize]);
    opt = tmp___26 as libc::c_int;
    if opt != 0 {
        if opt <= 2 as libc::c_int {
            g_state.set_trash(opt as uint_t);
        }
    }
    act
        .__sigaction_handler
        .sa_handler = Some(sigint_handler as unsafe extern "C" fn(libc::c_int) -> ());
    act.sa_mask.__val[0 as libc::c_int as usize] = 0 as libc::c_ulong;
    act.sa_mask.__val[1 as libc::c_int as usize] = 0 as libc::c_ulong;
    act.sa_mask.__val[2 as libc::c_int as usize] = 0 as libc::c_ulong;
    act.sa_mask.__val[3 as libc::c_int as usize] = 0 as libc::c_ulong;
    act.sa_mask.__val[4 as libc::c_int as usize] = 0 as libc::c_ulong;
    act.sa_mask.__val[5 as libc::c_int as usize] = 0 as libc::c_ulong;
    act.sa_mask.__val[6 as libc::c_int as usize] = 0 as libc::c_ulong;
    act.sa_mask.__val[7 as libc::c_int as usize] = 0 as libc::c_ulong;
    act.sa_mask.__val[8 as libc::c_int as usize] = 0 as libc::c_ulong;
    act.sa_mask.__val[9 as libc::c_int as usize] = 0 as libc::c_ulong;
    act.sa_mask.__val[10 as libc::c_int as usize] = 0 as libc::c_ulong;
    act.sa_mask.__val[11 as libc::c_int as usize] = 0 as libc::c_ulong;
    act.sa_mask.__val[12 as libc::c_int as usize] = 0 as libc::c_ulong;
    act.sa_mask.__val[13 as libc::c_int as usize] = 0 as libc::c_ulong;
    act.sa_mask.__val[14 as libc::c_int as usize] = 0 as libc::c_ulong;
    act.sa_mask.__val[15 as libc::c_int as usize] = 0 as libc::c_ulong;
    act.sa_flags = 0 as libc::c_int;
    act.sa_restorer = None;
    tmp___28 = sigaction(
        2 as libc::c_int,
        &mut act as *mut sigaction as *const sigaction,
        0 as *mut libc::c_void as *mut sigaction,
    );
    if tmp___28 < 0 as libc::c_int {
        tmp___27 = xitoa(8685 as libc::c_int as uint_t);
        perror(tmp___27 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    act
        .__sigaction_handler
        .sa_handler = Some(
        clean_exit_sighandler as unsafe extern "C" fn(libc::c_int) -> (),
    );
    tmp___30 = sigaction(
        15 as libc::c_int,
        &mut act as *mut sigaction as *const sigaction,
        0 as *mut libc::c_void as *mut sigaction,
    );
    if tmp___30 < 0 as libc::c_int {
        tmp___29 = xitoa(8692 as libc::c_int as uint_t);
        perror(tmp___29 as *const libc::c_char);
        return 1 as libc::c_int;
    } else {
        tmp___31 = sigaction(
            1 as libc::c_int,
            &mut act as *mut sigaction as *const sigaction,
            0 as *mut libc::c_void as *mut sigaction,
        );
        if tmp___31 < 0 as libc::c_int {
            tmp___29 = xitoa(8692 as libc::c_int as uint_t);
            perror(tmp___29 as *const libc::c_char);
            return 1 as libc::c_int;
        }
    }
    act
        .__sigaction_handler
        .sa_handler = ::std::mem::transmute::<
        libc::intptr_t,
        Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    >(1 as libc::c_int as libc::intptr_t);
    tmp___33 = sigaction(
        3 as libc::c_int,
        &mut act as *mut sigaction as *const sigaction,
        0 as *mut libc::c_void as *mut sigaction,
    );
    if tmp___33 < 0 as libc::c_int {
        tmp___32 = xitoa(8699 as libc::c_int as uint_t);
        perror(tmp___32 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    tmp___34 = initcurses(0 as *mut libc::c_void);
    if !tmp___34 {
        return 1 as libc::c_int;
    }
    if sort != 0 {
        set_sort_flags(sort);
    }
    tmp___35 = browse(initpath, session as *const libc::c_char, pkey);
    opt = tmp___35 as libc::c_int;
    endwin();
    if g_state.picker() != 0 {
        if selbufpos != 0 {
            if !selpath.is_null() {
                tmp___36 = open(
                    selpath as *const libc::c_char,
                    577 as libc::c_int,
                    384 as libc::c_int,
                );
                fd = tmp___36;
            } else {
                fd = 1 as libc::c_int;
            }
            if fd == -(1 as libc::c_int) {
                tmp___37 = xitoa(8773 as libc::c_int as uint_t);
                perror(tmp___37 as *const libc::c_char);
            } else {
                tmp___38 = seltofile(fd, 0 as *mut libc::c_void as *mut uint_t);
                if tmp___38 != selbufpos as size_t {
                    tmp___37 = xitoa(8773 as libc::c_int as uint_t);
                    perror(tmp___37 as *const libc::c_char);
                }
            }
            if fd > 1 as libc::c_int {
                close(fd);
            }
        }
    } else if !selpath.is_null() {
        unlink(selpath as *const libc::c_char);
    }
    rmlistpath();
    regfree(&mut archive_re);
    free(pselbuf as *mut libc::c_void);
    if inotify_wd >= 0 as libc::c_int {
        inotify_rm_watch(inotify_fd, inotify_wd);
    }
    close(inotify_fd);
    return opt;
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
    cfg = {
        let mut init = __anonstruct_settings_893539002 {
            filtermode_timeorder_sizeorder_apparentsz_blkorder_extnorder_showhidden_reserved0_showdetail_ctxactive_reverse_version_reserved1_curctx_prefersel_fileinfo_nonavopen_autoenter_reserved2_useeditor_reserved3_regex_x11_timetype_cliopener_waitedit_rollover: [0; 4],
        };
        init.set_filtermode(0 as libc::c_uint);
        init.set_timeorder(0 as libc::c_uint);
        init.set_sizeorder(0 as libc::c_uint);
        init.set_apparentsz(0 as libc::c_uint);
        init.set_blkorder(0 as libc::c_uint);
        init.set_extnorder(0 as libc::c_uint);
        init.set_showhidden(0 as libc::c_uint);
        init.set_reserved0(0 as libc::c_uint);
        init.set_showdetail(0 as libc::c_uint);
        init.set_ctxactive(1 as libc::c_int as uint_t);
        init.set_reverse(0 as libc::c_uint);
        init.set_version(0 as libc::c_uint);
        init.set_reserved1(0 as libc::c_uint);
        init.set_curctx(0 as libc::c_uint);
        init.set_prefersel(0 as libc::c_uint);
        init.set_fileinfo(0 as libc::c_uint);
        init.set_nonavopen(0 as libc::c_uint);
        init.set_autoenter(1 as libc::c_int as uint_t);
        init.set_reserved2(0 as libc::c_uint);
        init.set_useeditor(0 as libc::c_uint);
        init.set_reserved3(0 as libc::c_uint);
        init.set_regex(0 as libc::c_uint);
        init.set_x11(0 as libc::c_uint);
        init.set_timetype(2 as libc::c_int as uint_t);
        init.set_cliopener(0 as libc::c_uint);
        init.set_waitedit(0 as libc::c_uint);
        init.set_rollover(1 as libc::c_int as uint_t);
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
